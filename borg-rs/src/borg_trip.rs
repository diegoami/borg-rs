//! The trip policy (borg-rs#60): how deep a trip into the dungeon goes and
//! when it comes back to town.
//!
//! The munchkin and lunal modes were removed: there is one play mode. What
//! remains to decide is one question, asked whenever the Borg considers
//! leaving its level: **continue** on this level, **descend**, **ascend**
//! one level, or **return** to town - with the reasons. This module is that
//! decision and nothing else:
//!
//! * [`TripInputs`] - the player-known state the decision reads, in the
//!   game's own units (levels, gold, pack slots per distinct stack, stacks);
//! * [`TripParams`] - the five `borg_trip_*` settings;
//! * [`decide`] - a pure function from the two to a [`Decision`]: the
//!   action, the route a return or a move would take, the planned depth,
//!   and every [`Reason`] that fired, in priority order;
//! * [`plan_trip`] - the planned trip (depth target, levels, Recall scrolls)
//!   the town plan prices its purchases for.
//!
//! **Deciding is not moving.** Walking to a staircase, reading Word of
//! Recall, or searching for a way out is `borg_leave_level`'s and the flow
//! code's, unchanged. A decision whose route is [`Route::Search`] says "go
//! back, but no way back is known yet"; the executor keeps exploring until a
//! staircase is seen, and the decision is asked again. **Survival comes
//! first and is not here**: `borg_caution` (fleeing, escapes, recall under
//! threat) runs before the Borg ever considers leaving a level, and nothing
//! in this policy can delay it.
//!
//! **Off by default.** `borg_trip_policy = 0` keeps today's behaviour,
//! byte for byte (the C/Rust parity of the default config); the policy is
//! then consulted only to write `BORG_TRIP_RECORD`, which observes and
//! never acts. `borg_trip_policy = 1` lets the decision act. No formula,
//! weight or threshold here is accepted yet: the values are the provisional
//! proposal of docs/TRIP-POLICY.md, to be fitted on runs and confirmed on
//! others.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

use crate::borg_headers::borg_h::{
    BORG_TRIP_DEPTH_MARGIN, BORG_TRIP_PACK_SLACK, BORG_TRIP_POLICY, BORG_TRIP_RETURN_GOLD,
    BORG_TRIP_UNKNOWN_STACKS,
};
use crate::borg_headers::borg_trait_h::{
    BI_CDEPTH, BI_CLASS, BI_CLEVEL, BI_GOLD, BI_MAXCLEVEL, BI_MAXDEPTH, BI_RECALL,
};
use crate::state::BorgState;

/// The shallowest depth from which a return reads Word of Recall
/// (`borg_leave_level` recalls only from depth 5 down).
pub const RECALL_MIN_DEPTH: i32 = 5;
/// The deepest level.
pub const MAX_DEPTH: i32 = 127;

/// The five `borg_trip_*` settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TripParams {
    /// `borg_trip_policy`: 0 = today's behaviour (the policy only records),
    /// 1 = the policy acts.
    pub policy: i32,
    /// `borg_trip_depth_margin`, levels: the planned depth stays this many
    /// levels above the deepest level the Borg is prepared for.
    pub depth_margin: i32,
    /// `borg_trip_pack_slack`, pack slots: return when this many slots or
    /// fewer are free, counted per distinct stack (the town plan's count).
    pub pack_slack: i32,
    /// `borg_trip_unknown_stacks`, stacks: return when this many carried
    /// equipment or device stacks are not fully known (0 = never for this reason).
    pub unknown_stacks: i32,
    /// `borg_trip_return_gold`, gold per character level: return when the
    /// gold carried plus the estimated sale value reaches this times the
    /// character level (0 = never for this reason).
    pub return_gold: i32,
}

impl TripParams {
    pub fn of(cfg: &[i32]) -> Self {
        let get = |i: u32| cfg.get(i as usize).copied().unwrap_or(0);
        TripParams {
            policy: get(BORG_TRIP_POLICY),
            depth_margin: get(BORG_TRIP_DEPTH_MARGIN),
            pack_slack: get(BORG_TRIP_PACK_SLACK),
            unknown_stacks: get(BORG_TRIP_UNKNOWN_STACKS),
            return_gold: get(BORG_TRIP_RETURN_GOLD),
        }
    }
}

/// Validate the five settings (`borg_init` refuses to start otherwise).
pub fn config_error(cfg: &[i32]) -> Option<&'static str> {
    let p = TripParams::of(cfg);
    if !matches!(p.policy, 0 | 1) {
        return Some("borg_trip_policy must be 0 (off) or 1 (on)");
    }
    if !(0..=MAX_DEPTH).contains(&p.depth_margin) {
        return Some("borg_trip_depth_margin must be 0..127 levels");
    }
    if !(0..=23).contains(&p.pack_slack) {
        return Some("borg_trip_pack_slack must be 0..23 pack slots");
    }
    if p.unknown_stacks < 0 {
        return Some("borg_trip_unknown_stacks must be 0 (off) or more stacks");
    }
    if p.return_gold < 0 {
        return Some("borg_trip_return_gold must be 0 (off) or more gold per level");
    }
    None
}

/// Is the policy acting (`borg_trip_policy = 1`)?
pub fn acting(st: &BorgState) -> bool {
    !st.core.cfg.is_empty() && st.cfg(BORG_TRIP_POLICY) == 1
}

/// What the decision reads: what the player knows, nothing hidden.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TripInputs {
    pub clevel: i32,
    pub max_clevel: i32,
    /// `BI_CLASS` and `BI_RACE`: recorded, not yet read by [`decide`]. The
    /// class enters through `prepared_depth` (the preparation tests are by
    /// class); a direct class or race term needs evidence first.
    pub class: i32,
    pub race: i32,
    /// `BI_CDEPTH`, `BI_MAXDEPTH`.
    pub depth: i32,
    pub max_depth: i32,
    /// The deepest level `borg_prepared` accepts, scanning down from level
    /// 1: what the Borg's kit and level are ready for.
    pub prepared_depth: i32,
    /// `borg_prepared(depth)` accepts the current level.
    pub prepared_here: bool,
    /// `borg_must_return_to_town` gives a reason: a crucial supply is out.
    pub restock: bool,
    /// `BI_GOLD`.
    pub gold: i64,
    /// The estimated sale value of the pack, in gold: the Borg's own value
    /// of every carried stack (`borg_item.value`, what it believes the item
    /// is worth from what it knows) times the quantity. An estimate; the
    /// shops pay less and the town plan decides what is sold.
    pub sale_value: i64,
    /// `borg_count_sell` as `borg_leave_level` computed it (-1 = not known).
    pub sellable: i32,
    /// The pack slots a stack can occupy, and those occupied, one per
    /// distinct stack (`borg_town_budget::Facts`, `borg_town_stock`).
    pub pack_capacity: i32,
    pub pack_used: i32,
    /// Carried stacks the player does not fully know: an unaware flavour, or
    /// runes not yet learnt (`borg_item_note_needs_id`).
    pub unknown_stacks: i32,
    /// Unknown equipment/device stacks: the knowledge-return backlog.
    /// Routine consumables still occupy pack slots, but do not independently
    /// justify a trip solely because their flavour is unknown.
    pub unknown_gear_stacks: i32,
    /// `BI_RECALL`: Word of Recall scrolls and equivalents.
    pub recall: i32,
    /// Staircases seen on this level.
    pub up_stairs_known: bool,
    pub down_stairs_known: bool,
}

impl TripInputs {
    pub fn pack_free(&self) -> i32 {
        self.pack_capacity - self.pack_used
    }
}

/// What to do about the level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Stay: keep exploring this level.
    Continue,
    /// Go one level deeper.
    Descend,
    /// Go one level up, not to town.
    Ascend,
    /// Go back to town.
    Return,
}

impl Action {
    pub fn name(self) -> &'static str {
        match self {
            Action::Continue => "continue",
            Action::Descend => "descend",
            Action::Ascend => "ascend",
            Action::Return => "return",
        }
    }
}

/// How the action would be carried out, as far as the decision can see.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// Nothing to execute.
    None,
    /// Read Word of Recall (a return from `RECALL_MIN_DEPTH` or deeper).
    Recall,
    /// A known staircase.
    Stairs,
    /// No route is known yet: keep exploring until a staircase is seen.
    Search,
}

impl Route {
    pub fn name(self) -> &'static str {
        match self {
            Route::None => "none",
            Route::Recall => "recall",
            Route::Stairs => "stairs",
            Route::Search => "search",
        }
    }
}

/// Why. Each reason carries the input and the threshold it was held to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    /// A crucial supply is out (`borg_must_return_to_town`).
    Restock,
    /// Not prepared for the current level.
    Unprepared { depth: i32, prepared_depth: i32 },
    /// The pack is (nearly) full: `free <= slack`.
    PackFull { free: i32, slack: i32 },
    /// Too much is unknown: `stacks >= threshold`.
    Unknown { stacks: i32, threshold: i32 },
    /// The purse is worth a visit: `gold + sale >= threshold`.
    Purse { worth: i64, threshold: i64 },
    /// Above the planned depth.
    BelowPlan { depth: i32, target: i32 },
    /// At the planned depth.
    AtPlan { depth: i32, target: i32 },
    /// Deeper than the planned depth.
    BeyondPlan { depth: i32, target: i32 },
}

impl Reason {
    /// The reason as one log word with its numbers.
    pub fn text(&self) -> String {
        match *self {
            Reason::Restock => "restock".into(),
            Reason::Unprepared { depth, prepared_depth } => {
                format!("unprepared(depth {depth} > prepared {prepared_depth})")
            }
            Reason::PackFull { free, slack } => format!("pack(free {free} <= {slack})"),
            Reason::Unknown { stacks, threshold } => format!("unknown-gear({stacks} >= {threshold})"),
            Reason::Purse { worth, threshold } => format!("purse({worth} >= {threshold})"),
            Reason::BelowPlan { depth, target } => format!("below-plan({depth} < {target})"),
            Reason::AtPlan { depth, target } => format!("at-plan({depth} = {target})"),
            Reason::BeyondPlan { depth, target } => format!("beyond-plan({depth} > {target})"),
        }
    }
}

/// The decision, its route, the planned depth, and all the reasons that
/// fired (the first one decided; the rest are recorded so that feedback can
/// see everything that pointed the same way).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub action: Action,
    pub route: Route,
    pub target: i32,
    pub reasons: Vec<Reason>,
}

impl Decision {
    /// `return via recall: pack(free 1 <= 2) unknown-gear(5 >= 4)`
    pub fn summary(&self) -> String {
        let reasons: Vec<String> = self.reasons.iter().map(Reason::text).collect();
        format!(
            "{} via {} (plan {}): {}",
            self.action.name(),
            self.route.name(),
            self.target,
            reasons.join(" ")
        )
    }
}

/// The planned depth: the deepest prepared level less the margin, never
/// above level 1, never past the deepest level.
pub fn planned_depth(i: &TripInputs, p: &TripParams) -> i32 {
    (i.prepared_depth - p.depth_margin).clamp(1, MAX_DEPTH)
}

/// The route a move in this direction takes from here.
fn route(i: &TripInputs, action: Action) -> Route {
    match action {
        Action::Continue => Route::None,
        Action::Return if i.depth >= RECALL_MIN_DEPTH && i.recall >= 1 => Route::Recall,
        Action::Return | Action::Ascend if i.up_stairs_known => Route::Stairs,
        Action::Descend if i.down_stairs_known => Route::Stairs,
        _ => Route::Search,
    }
}

/// The decision. The priorities, highest first:
///
/// 1. **restock** - a crucial supply is out: return;
/// 2. **unprepared** - the level is beyond what the Borg is ready for:
///    ascend;
/// 3. **the value of going back** - the pack is nearly full, too much is
///    unknown, or the purse is worth a visit: return, with every one of
///    these that fired;
/// 4. **the planned depth** - descend above it, continue at it, ascend
///    below it.
///
/// Survival is not on the list: `borg_caution` runs first and is not
/// delayed by anything here. In town (`depth == 0`) there is nothing to
/// decide: continue.
pub fn decide(i: &TripInputs, p: &TripParams) -> Decision {
    let target = planned_depth(i, p);
    let done = |action: Action, reasons: Vec<Reason>| Decision {
        action,
        route: route(i, action),
        target,
        reasons,
    };
    if i.depth == 0 {
        return done(Action::Continue, Vec::new());
    }

    // 1. a crucial supply is out
    let mut back = Vec::new();
    if i.restock {
        back.push(Reason::Restock);
    }

    // 3. the value of going back (collected now; 2 outranks it below)
    if i.pack_free() <= p.pack_slack {
        back.push(Reason::PackFull { free: i.pack_free(), slack: p.pack_slack });
    }
    if p.unknown_stacks > 0 && i.unknown_gear_stacks >= p.unknown_stacks {
        back.push(Reason::Unknown { stacks: i.unknown_gear_stacks, threshold: p.unknown_stacks });
    }
    if p.return_gold > 0 {
        let worth = i.gold + i.sale_value;
        let threshold = i64::from(p.return_gold) * i64::from(i.clevel.max(1));
        if worth >= threshold {
            back.push(Reason::Purse { worth, threshold });
        }
    }

    // 2. not ready for this level: one level up, unless a return is due
    // for a supply anyway (the return goes up too, and further)
    if !i.prepared_here && !i.restock {
        let mut reasons =
            vec![Reason::Unprepared { depth: i.depth, prepared_depth: i.prepared_depth }];
        reasons.extend(back);
        return done(Action::Ascend, reasons);
    }
    if !back.is_empty() {
        return done(Action::Return, back);
    }

    // 4. the planned depth
    if i.depth < target {
        done(Action::Descend, vec![Reason::BelowPlan { depth: i.depth, target }])
    } else if i.depth == target {
        done(Action::Continue, vec![Reason::AtPlan { depth: i.depth, target }])
    } else {
        done(Action::Ascend, vec![Reason::BeyondPlan { depth: i.depth, target }])
    }
}

/// The trip the town plan should supply (borg-rs#60 with #36): where it
/// aims, how many levels it crosses from the recall depth, and how many
/// Word of Recall scrolls it consumes.
///
/// The town plan's `Trip` (borg_town_usefulness) is the intended reader;
/// until it reads this (docs/TRIP-POLICY.md, "One trip model") the item
/// carries the self-removing marker the town modules use.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlannedTrip {
    pub depth_target: i32,
    /// Levels from where the trip starts in the dungeon (the recall depth,
    /// `BI_MAXDEPTH`, or level 1 on foot) to the target, at least 1.
    pub levels: i32,
    /// Word of Recall reads the trip uses: one down from town when the
    /// recall depth is at least `RECALL_MIN_DEPTH`, one back up when the
    /// target is.
    pub recall_scrolls: i32,
    /// The scrolls to carry out of town: the reads plus one spare when there
    /// is any read. For the usual round trip that is 3, which is what the
    /// current code holds the Borg to before it recalls down (`leave_town`);
    /// below that, `borg_prepared` wants 1 from level 5 ("1 recall"). (The
    /// "restock recall" rule of `borg_restock` is commented out, in C too.)
    pub recall_carry: i32,
}

/// The planned trip from the inputs (in town or in the dungeon).
#[cfg_attr(not(test), expect(dead_code))]
pub fn plan_trip(i: &TripInputs, p: &TripParams) -> PlannedTrip {
    let depth_target = planned_depth(i, p);
    let start = if i.max_depth >= RECALL_MIN_DEPTH { i.max_depth.min(depth_target) } else { 1 };
    let down = i32::from(i.max_depth >= RECALL_MIN_DEPTH);
    let up = i32::from(depth_target >= RECALL_MIN_DEPTH);
    let reads = down + up;
    PlannedTrip {
        depth_target,
        levels: (depth_target - start).max(1),
        recall_scrolls: reads,
        recall_carry: if reads > 0 { reads + 1 } else { 0 },
    }
}

/// `borg_prepared(depth)` without its side effects: it sets
/// `ready_morgoth` and, deep down, re-notices the Home; both are put back.
fn prepared_probe(st: &mut BorgState, depth: i32) -> bool {
    let ready = st.borg.ready_morgoth;
    let home = st.home.clone();
    let ok = crate::borg_prepared::borg_prepared(st, depth).is_none();
    st.borg.ready_morgoth = ready;
    st.home = home;
    ok
}

/// `borg_must_return_to_town` without its side effects (as above).
fn restock_probe(st: &mut BorgState) -> bool {
    let ready = st.borg.ready_morgoth;
    let home = st.home.clone();
    let due = crate::borg_prepared::borg_must_return_to_town(st).is_some();
    st.borg.ready_morgoth = ready;
    st.home = home;
    due
}

/// Equipment, ammunition and devices can conceal reusable powers. Count
/// their unknown stacks as backlog; routine consumables and books do not
/// receive an independent knowledge-return vote. Uses only visible tval,
/// never the hidden kind level, true value or powers.
fn knowledge_backlog_type(tval: u8) -> bool {
    use crate::headers::list_tvals_h::{TV_SHOT, TV_ROD};
    (TV_SHOT..=TV_ROD).contains(&u32::from(tval))
}

/// The pack slots in use, one per distinct stack, as the town plan counts
/// them, and the stacks that are not fully known.
fn pack_counts(st: &BorgState) -> (i32, i32, i32, i64) {
    let facts = crate::borg_town_budget::Facts::of(st);
    let n = usize::try_from(facts.pack_capacity).unwrap_or(0).min(st.items.items.len());
    let pack = &st.items.items[..n];
    let used = crate::borg_town_stock::pack_slots_used_of(pack);
    let mut unknown = 0;
    let mut unknown_gear = 0;
    let mut sale = 0i64;
    for item in pack.iter().filter(|it| it.iqty != 0) {
        if !item.aware || crate::borg_item_id::borg_item_note_needs_id(st, item) {
            unknown += 1;
            if knowledge_backlog_type(item.tval) {
                unknown_gear += 1;
            }
        }
        if item.value > 0 {
            sale += i64::from(item.value) * i64::from(item.iqty);
        }
    }
    (used, unknown, unknown_gear, sale)
}

/// Read the inputs from the state. `sellable` is what the caller already
/// counted (`borg_count_sell` is not called again here). Leaves the state
/// as it found it.
pub fn observe(st: &mut BorgState, sellable: i32) -> TripInputs {
    let depth = st.borg.tr(BI_CDEPTH);
    let mut prepared_depth = 1;
    while prepared_depth < MAX_DEPTH && prepared_probe(st, prepared_depth + 1) {
        prepared_depth += 1;
    }
    let prepared_here = depth <= 1 || prepared_probe(st, depth);
    let restock = restock_probe(st);
    let (pack_used, unknown_stacks, unknown_gear_stacks, sale_value) = pack_counts(st);
    let facts = crate::borg_town_budget::Facts::of(st);
    TripInputs {
        clevel: st.borg.tr(BI_CLEVEL),
        max_clevel: st.borg.tr(BI_MAXCLEVEL),
        class: st.borg.tr(BI_CLASS),
        race: crate::engine::race_ridx() as i32,
        depth,
        max_depth: st.borg.tr(BI_MAXDEPTH),
        prepared_depth,
        prepared_here,
        restock,
        gold: i64::from(st.borg.tr(BI_GOLD)),
        sale_value,
        sellable,
        pack_capacity: facts.pack_capacity,
        pack_used,
        unknown_stacks,
        unknown_gear_stacks,
        recall: st.borg.tr(BI_RECALL),
        up_stairs_known: st.stairs.less.num > 0,
        down_stairs_known: st.stairs.more.num > 0,
    }
}

/// The record file, `BORG_TRIP_RECORD=<file>`, once per process.
fn record_file() -> Option<&'static Mutex<File>> {
    static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    FILE.get_or_init(|| {
        let path = std::env::var("BORG_TRIP_RECORD").ok().filter(|p| !p.is_empty())?;
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(f) => Some(Mutex::new(f)),
            Err(err) => {
                eprintln!("borg: BORG_TRIP_RECORD {path}: {err}");
                None
            }
        }
    })
    .as_ref()
}

/// The last decision noted. Records also include input-only changes via
/// `LAST_INPUTS`; gameplay notes remain limited to decision/level changes.
static LAST: Mutex<Option<(i32, i32, String)>> = Mutex::new(None);
/// Keep inventory/input changes visible even when the recommendation stays
/// the same. Otherwise a suppressed knowledge trigger disappears from the
/// evidence we need to assess it. This does not add gameplay notes.
static LAST_INPUTS: Mutex<Option<(i32, TripInputs)>> = Mutex::new(None);

/// One record line: the version, the clock, the effective parameters, the
/// inputs and the decision. Integers and fixed words only.
pub fn record_line(turn: i32, game_turn: i32, i: &TripInputs, p: &TripParams, d: &Decision) -> String {
    let reasons: Vec<String> = d.reasons.iter().map(|r| format!("\"{}\"", r.text())).collect();
    format!(
        "{{\"v\":2,\"turn\":{turn},\"game_turn\":{game_turn},\"params\":{{\"policy\":{},\"depth_margin\":{},\"pack_slack\":{},\
         \"unknown_stacks\":{},\"return_gold\":{}}},\"inputs\":{{\"clevel\":{},\"max_clevel\":{},\
         \"class\":{},\"race\":{},\"depth\":{},\"max_depth\":{},\"prepared_depth\":{},\
         \"prepared_here\":{},\"restock\":{},\"gold\":{},\"sale_value\":{},\"sellable\":{},\
         \"pack_capacity\":{},\"pack_used\":{},\"unknown_stacks\":{},\"unknown_gear_stacks\":{},\"recall\":{},\
         \"up_stairs_known\":{},\"down_stairs_known\":{}}},\"decision\":{{\"action\":\"{}\",\
         \"route\":\"{}\",\"target\":{},\"reasons\":[{}]}}}}",
        p.policy, p.depth_margin, p.pack_slack, p.unknown_stacks, p.return_gold,
        i.clevel, i.max_clevel, i.class, i.race, i.depth, i.max_depth, i.prepared_depth,
        i.prepared_here, i.restock, i.gold, i.sale_value, i.sellable, i.pack_capacity,
        i.pack_used, i.unknown_stacks, i.unknown_gear_stacks, i.recall, i.up_stairs_known, i.down_stairs_known,
        d.action.name(), d.route.name(), d.target, reasons.join(","),
    )
}

/// Ask the policy (from `borg_leave_level`, in the dungeon). Returns the
/// decision when the policy acts; records it when `BORG_TRIP_RECORD` is
/// set. With the policy off and no record, it reads nothing at all, so the
/// default play is today's play.
pub fn consult(st: &mut BorgState, sellable: i32) -> Option<Decision> {
    let act = acting(st);
    let file = record_file();
    if !act && file.is_none() {
        return None;
    }
    let params = TripParams::of(&st.core.cfg);
    let inputs = observe(st, sellable);
    let decision = decide(&inputs, &params);

    let summary = decision.summary();
    let key = (inputs.depth, st.borg.time.level, summary.clone());
    let changed = {
        let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
        let changed = last.as_ref() != Some(&key);
        *last = Some(key);
        changed
    };
    if let Some(f) = file {
        let inputs_changed = {
            let key = (st.borg.time.level, inputs);
            let mut last = LAST_INPUTS.lock().unwrap_or_else(|e| e.into_inner());
            let changed = last.as_ref() != Some(&key);
            *last = Some(key);
            changed
        };
        if changed || inputs_changed {
            let line = record_line(st.borg.time.now, crate::engine::game_turn(), &inputs, &params, &decision);
            let mut f = f.lock().unwrap_or_else(|e| e.into_inner());
            let _ = writeln!(f, "{line}");
        }
    }
    if changed && act {
        crate::borg_io::borg_note(st, &format!("# Trip: {summary}"));
    }
    act.then_some(decision)
}

/// The legacy "needing to sell" hold: below character level 25 and 25,000
/// gold, with 13 or more sellable items, the Borg does not go down and
/// heads up. Four copies of this condition were spread over
/// `borg_think_dungeon` and `borg_flow_stairs`; this is the one copy, in
/// the C evaluation order (so `borg_count_sell` runs exactly when it did).
/// With the policy acting, its pack and purse reasons replace the hold.
pub fn legacy_sale_hold(st: &mut BorgState) -> bool {
    legacy_sale_gate(acting(st), st.borg.tr(BI_CDEPTH), st.borg.tr(BI_CLEVEL), st.borg.tr(BI_GOLD))
        && crate::borg_store_sell::borg_count_sell(st) >= 13
}

/// The part of [`legacy_sale_hold`] before `borg_count_sell`: the policy
/// off, in the dungeon, below level 25 and 25,000 gold.
fn legacy_sale_gate(acting: bool, depth: i32, clevel: i32, gold: i32) -> bool {
    !acting && depth != 0 && clevel < 25 && gold < 25000
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A level-10 character on level 5, prepared down to 8, with room in the
    /// pack, nothing unknown, a Recall scroll and both staircases seen.
    fn base() -> TripInputs {
        TripInputs {
            clevel: 10,
            max_clevel: 10,
            class: 0,
            race: 0,
            depth: 5,
            max_depth: 6,
            prepared_depth: 8,
            prepared_here: true,
            restock: false,
            gold: 300,
            sale_value: 100,
            sellable: 2,
            pack_capacity: 23,
            pack_used: 10,
            unknown_stacks: 0,
            unknown_gear_stacks: 0,
            recall: 1,
            up_stairs_known: true,
            down_stairs_known: true,
        }
    }

    fn params() -> TripParams {
        let cfg: Vec<i32> = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
        TripParams::of(&cfg)
    }

    #[test]
    fn the_defaults_are_off_and_valid() {
        let cfg: Vec<i32> = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
        assert_eq!(config_error(&cfg), None);
        let p = TripParams::of(&cfg);
        assert_eq!(p, TripParams { policy: 0, depth_margin: 0, pack_slack: 2, unknown_stacks: 4, return_gold: 0 });
        let mut st = BorgState::default();
        st.core.cfg = cfg;
        assert!(!acting(&st));
    }

    #[test]
    fn out_of_range_settings_are_rejected() {
        let ok: Vec<i32> = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
        for (index, bad) in [
            (BORG_TRIP_POLICY, 2),
            (BORG_TRIP_DEPTH_MARGIN, -1),
            (BORG_TRIP_PACK_SLACK, 24),
            (BORG_TRIP_UNKNOWN_STACKS, -1),
            (BORG_TRIP_RETURN_GOLD, -5),
        ] {
            let mut c = ok.clone();
            c[index as usize] = bad;
            assert!(config_error(&c).is_some(), "{index} = {bad}");
        }
    }

    /// Level and class reach the depth through preparation: the planned
    /// depth is the prepared depth less the margin.
    #[test]
    fn the_planned_depth_follows_preparation_and_margin() {
        let mut p = params();
        let i = base();
        assert_eq!(decide(&i, &p).action, Action::Descend);
        assert_eq!(decide(&i, &p).reasons, vec![Reason::BelowPlan { depth: 5, target: 8 }]);
        p.depth_margin = 3;
        let d = decide(&i, &p);
        assert_eq!((d.action, d.target), (Action::Continue, 5));
        p.depth_margin = 4;
        assert_eq!(decide(&i, &p).action, Action::Ascend);
        // a stronger character (same class, deeper preparation) plans deeper
        let strong = TripInputs { clevel: 25, prepared_depth: 20, ..base() };
        assert_eq!(decide(&strong, &params()).target, 20);
        // the margin never plans above level 1
        p.depth_margin = 100;
        assert_eq!(planned_depth(&i, &p), 1);
    }

    /// The owner's case: plenty of unknown items, return earlier.
    #[test]
    fn returning_with_unidentified_items_is_justified() {
        let p = params();
        let i = TripInputs { unknown_stacks: 5, unknown_gear_stacks: 5, ..base() };
        let d = decide(&i, &p);
        assert_eq!(d.action, Action::Return);
        assert_eq!(d.route, Route::Recall);
        assert_eq!(d.reasons, vec![Reason::Unknown { stacks: 5, threshold: 4 }]);
        // below the threshold, the trip goes on
        let i = TripInputs { unknown_stacks: 3, unknown_gear_stacks: 3, ..base() };
        assert_eq!(decide(&i, &p).action, Action::Descend);
        // 0 switches the reason off
        let off = TripParams { unknown_stacks: 0, ..p };
        let i = TripInputs { unknown_stacks: 20, unknown_gear_stacks: 20, ..base() };
        assert_eq!(decide(&i, &off).action, Action::Descend);
    }

    #[test]
    fn routine_unknown_finds_do_not_end_a_trip_with_room_left() {
        let p = params();
        let i = TripInputs { clevel: 1, depth: 1, prepared_depth: 1,
            unknown_stacks: 8, unknown_gear_stacks: 0, ..base() };
        assert_eq!(decide(&i, &p).action, Action::Continue);
        // They still fill the pack: the pack-pressure reason is unchanged.
        let full = TripInputs { pack_used: 21, ..i };
        assert_eq!(decide(&full, &p).reasons,
            vec![Reason::PackFull { free: 2, slack: 2 }]);
        // A mixed pack counts only its equipment/device backlog.
        let mixed = TripInputs { unknown_gear_stacks: 4, ..i };
        assert_eq!(decide(&mixed, &p).reasons,
            vec![Reason::Unknown { stacks: 4, threshold: 4 }]);
    }

    #[test]
    fn knowledge_backlog_uses_visible_item_categories() {
        use crate::headers::list_tvals_h::*;
        for tval in [TV_SHOT, TV_ARROW, TV_BOLT, TV_BOW, TV_SWORD,
            TV_SOFT_ARMOR, TV_LIGHT, TV_AMULET, TV_RING, TV_STAFF, TV_WAND, TV_ROD] {
            assert!(knowledge_backlog_type(tval as u8), "tval {tval}");
        }
        for tval in [TV_NULL, TV_CHEST, TV_SCROLL, TV_POTION, TV_FLASK,
            TV_FOOD, TV_MUSHROOM, TV_MAGIC_BOOK, TV_GOLD] {
            assert!(!knowledge_backlog_type(tval as u8), "tval {tval}");
        }
    }

    /// Pack pressure is counted per distinct stack: a pack of 21 stacks
    /// has two free slots, which is the slack.
    #[test]
    fn pack_pressure_returns_before_the_pack_is_full() {
        let p = params();
        let i = TripInputs { pack_used: 20, ..base() };
        assert_eq!(decide(&i, &p).action, Action::Descend);
        let i = TripInputs { pack_used: 21, ..base() };
        let d = decide(&i, &p);
        assert_eq!(d.action, Action::Return);
        assert_eq!(d.reasons, vec![Reason::PackFull { free: 2, slack: 2 }]);
    }

    /// Gold and what it could buy: the purse reason scales with the level.
    #[test]
    fn the_purse_returns_when_worth_a_visit() {
        let p = TripParams { return_gold: 40, ..params() };
        // 300 + 100 = 400 >= 40 * 10
        let d = decide(&base(), &p);
        assert_eq!(d.action, Action::Return);
        assert_eq!(d.reasons, vec![Reason::Purse { worth: 400, threshold: 400 }]);
        // the same purse is not worth it at level 20
        let i = TripInputs { clevel: 20, ..base() };
        assert_eq!(decide(&i, &p).action, Action::Descend);
        // a sale estimate counts as well as the gold
        let i = TripInputs { clevel: 20, sale_value: 600, ..base() };
        assert_eq!(decide(&i, &p).action, Action::Return);
    }

    /// Every reason that fired is kept, the first one decides.
    #[test]
    fn every_reason_is_recorded() {
        let p = TripParams { return_gold: 1, ..params() };
        let i = TripInputs { restock: true, pack_used: 23, unknown_stacks: 6, unknown_gear_stacks: 6, ..base() };
        let d = decide(&i, &p);
        assert_eq!(d.action, Action::Return);
        assert_eq!(d.reasons.len(), 4);
        assert_eq!(d.reasons[0], Reason::Restock);
        assert!(d.summary().starts_with("return via recall (plan 8): restock pack(free 0 <= 2)"));
    }

    /// Unprepared for the level: one level up; a supply return outranks it.
    #[test]
    fn unprepared_ascends_and_restock_outranks_it() {
        let p = params();
        let i = TripInputs { prepared_here: false, prepared_depth: 4, unknown_stacks: 9, unknown_gear_stacks: 9, ..base() };
        let d = decide(&i, &p);
        assert_eq!(d.action, Action::Ascend);
        assert_eq!(d.reasons[0], Reason::Unprepared { depth: 5, prepared_depth: 4 });
        assert_eq!(d.reasons.len(), 2, "the return reasons are kept for the record");
        let i = TripInputs { restock: true, ..i };
        assert_eq!(decide(&i, &p).action, Action::Return);
    }

    /// The route is part of the decision, and an unavailable route is not
    /// a different decision: the Borg still wants to go back.
    #[test]
    fn unavailable_routes_are_reported_not_hidden() {
        let p = params();
        let full = TripInputs { pack_used: 23, ..base() };
        assert_eq!(decide(&full, &p).route, Route::Recall);
        let no_recall = TripInputs { recall: 0, ..full };
        assert_eq!(decide(&no_recall, &p).route, Route::Stairs);
        let lost = TripInputs { up_stairs_known: false, ..no_recall };
        let d = decide(&lost, &p);
        assert_eq!((d.action, d.route), (Action::Return, Route::Search));
        // shallower than RECALL_MIN_DEPTH the stairs are the route
        let shallow = TripInputs { depth: 3, ..full };
        assert_eq!(decide(&shallow, &p).route, Route::Stairs);
        // no known down staircase: search for one
        let i = TripInputs { down_stairs_known: false, ..base() };
        assert_eq!(decide(&i, &p).route, Route::Search);
    }

    #[test]
    fn town_decides_nothing() {
        let i = TripInputs { depth: 0, pack_used: 23, restock: true, ..base() };
        let d = decide(&i, &params());
        assert_eq!((d.action, d.route), (Action::Continue, Route::None));
        assert!(d.reasons.is_empty());
    }

    /// The planned trip the town plan reads: target, levels, Recall.
    #[test]
    fn the_planned_trip_counts_recall_scrolls() {
        let p = params();
        // recall depth 6, target 8: one scroll down, one back
        assert_eq!(
            plan_trip(&base(), &p),
            PlannedTrip { depth_target: 8, levels: 2, recall_scrolls: 2, recall_carry: 3 }
        );
        // a new character walks: no scroll down, and none back from level 3
        let new = TripInputs { depth: 0, max_depth: 0, prepared_depth: 3, ..base() };
        assert_eq!(
            plan_trip(&new, &p),
            PlannedTrip { depth_target: 3, levels: 2, recall_scrolls: 0, recall_carry: 0 }
        );
        // walking down to level 6: no scroll down, one back
        let mid = TripInputs { max_depth: 3, prepared_depth: 6, ..base() };
        assert_eq!((plan_trip(&mid, &p).recall_scrolls, plan_trip(&mid, &p).recall_carry), (1, 2));
        // a recall depth deeper than the plan starts at the plan
        let deep = TripInputs { max_depth: 30, prepared_depth: 20, ..base() };
        assert_eq!(
            plan_trip(&deep, &p),
            PlannedTrip { depth_target: 20, levels: 1, recall_scrolls: 2, recall_carry: 3 }
        );
    }

    /// The record is one line of integers, booleans and fixed words.
    #[test]
    fn the_record_line_is_stable() {
        let p = params();
        let i = TripInputs { unknown_stacks: 5, unknown_gear_stacks: 5, ..base() };
        let d = decide(&i, &p);
        let line = record_line(1234, 5678, &i, &p, &d);
        assert!(line.starts_with("{\"v\":2,\"turn\":1234,\"game_turn\":5678,\"params\":{\"policy\":0,"));
        assert!(line.contains("\"unknown_stacks\":5,"));
        assert!(line.contains("\"unknown_gear_stacks\":5,"));
        assert!(line.ends_with(
            "\"decision\":{\"action\":\"return\",\"route\":\"recall\",\"target\":8,\
             \"reasons\":[\"unknown-gear(5 >= 4)\"]}}"
        ));
        assert!(!line.contains('\n'));
    }

    /// With the policy off, the legacy sale hold is the C condition; the
    /// sale count is only reached past the depth, level and gold tests.
    #[test]
    fn the_legacy_gate_is_the_c_condition_and_off_with_the_policy() {
        assert!(legacy_sale_gate(false, 3, 10, 100));
        assert!(!legacy_sale_gate(false, 0, 10, 100), "town");
        assert!(!legacy_sale_gate(false, 3, 25, 100), "level 25");
        assert!(!legacy_sale_gate(false, 3, 10, 25000), "25,000 gold");
        assert!(!legacy_sale_gate(true, 3, 10, 100), "the policy replaces the hold");
    }
}
