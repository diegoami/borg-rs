//! Phase 4 of the town inventory redesign (borg-rs#36), slice 1: the plan
//! record (P13, integer version 4) and its replay.
//!
//! The shape is the one accepted on 2026-11-13 in
//! `docs/TOWN-INVENTORY-PHASE4.md`, Part B, with the dated correction
//! recorded there: version 2's fields unchanged and in order, then the
//! plan's. One record per plan step - one solve, and the action taken from
//! it.
//!
//! **Replay** is the record's contract: from a v4 line alone, rebuild the
//! candidates, the limits, the floors and the values, re-run the step, and
//! get the same plan, the same solve and the same runner-up. The facts a
//! record cannot recompute without the engine travel in it: the try-on
//! values (`tryon`) and, per ware, the facts the engine's item tables and
//! the stacking predicate decide (`wares`). Everything else - the deficits,
//! the weights, the caution, the groups, the solve - is recomputed by the
//! same code that produced it: [`step`] is the one core both sides call.
//!
//! The floor pass is not replayed: its result (`floors`) is the step's
//! input, and the pass has its own suite (phase 2).
//!
//! Nothing in production calls this module yet (slice 2 will, in shadow
//! mode); every public item carries the self-removing `expect(dead_code)`
//! marker of phases 1-3.

use std::collections::BTreeMap;

use crate::borg_headers::borg_h::BORG_TOWN_PLAN;
use crate::borg_headers::borg_trait_h::BI_MAX;
use crate::borg_town_budget::{Budget, FloorPurchase};
use crate::borg_town_solve::{groups, solve, Group, GroupKey, Limits, Solution};
use crate::borg_town_stock::{Candidate, Stock};
use crate::borg_town_usefulness::{usefulness, Need, Trip, Weights};
use crate::state::BorgState;

/// The plan record's schema version (v2: the decision record, v3: the
/// handover's state record).
pub const PLAN_SCHEMA: i64 = 4;

/// The twelve town settings, `borg_town_plan` (index 45) through
/// `borg_town_food_trip` (56), contiguous in the settings table.
pub const PARAMS: usize = 12;

/// The action codes: v2's, extended for the plan.
#[cfg_attr(not(test), expect(dead_code))]
pub mod action {
    pub const VISIT: i64 = 0;
    pub const BUY: i64 = 1;
    pub const TAKE_HOME: i64 = 2;
    pub const SELL: i64 = 3;
    pub const SELL_HOME: i64 = 4;
    pub const STORE: i64 = 5;
    pub const DROP: i64 = 6;
    pub const LEAVE: i64 = 7;
}

/// The `solve` flags, as bits.
pub mod flag {
    pub const OPTIMAL: i64 = 1;
    pub const GOLD_EXACT: i64 = 2;
    pub const SLOTS_MET: i64 = 4;
    pub const WEIGHT_MET: i64 = 8;
    pub const WEIGHT_REPAIRED: i64 = 16;
}

/// The engine-decided facts of one ware (the `wares` field).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WareFact {
    pub shop: u8,
    pub slot: u8,
    /// The object kind (`k_idx`); v2's tuple carries the tval.
    pub kind: u32,
    /// The stack class (phase 2).
    pub class: u16,
    /// 0 if a copy joins a carried stack, else 1.
    pub slots_cost: i32,
    /// The weight of one copy.
    pub weight: i32,
    /// The need it serves, as an index into [`Need::ALL`]; -1 for none.
    pub need: i8,
    /// What one copy closes of that need, in the need's own units.
    pub reduction: i64,
    /// The floors it discharges, a bitmask in [`Need::ALL`] order.
    pub serves: u8,
}

/// A wearable's try-on (the `tryon` field): the equipment slot and the
/// power gained by wearing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryOn {
    pub shop: u8,
    pub slot: u8,
    pub eq_slot: u8,
    pub value: i64,
}

/// One chosen option of the plan (the `plan` field), or the runner-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanEntry {
    /// 0 a stack class, 1 an equipment slot.
    pub key_kind: u8,
    pub key: i64,
    pub option: usize,
    pub copies: i32,
    pub cost: i64,
    pub value: i64,
    pub slots: i32,
}

/// The v4 fields (after v2's).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanRecord {
    /// 0 shadow (computed, not executed), 1 live.
    pub mode: u8,
    pub visit: u64,
    pub step: u64,
    /// The game's turn (v2's `turn` is the Borg's clock).
    pub gturn: i64,
    /// `borg.power`: the power need reads it, and it is no trait.
    pub power: i64,
    /// The twelve town settings, in index order.
    pub params: [i64; PARAMS],
    /// `[gold, pack_slots, home_slots, weight]` after the floors.
    pub limits: [i64; 4],
    /// `[pack_capacity, home_capacity]`.
    pub caps: [i64; 2],
    /// `(shop, slot, needs)` of each floor purchase, `needs` a bitmask.
    pub floors: Vec<(u8, u8, u8)>,
    /// The floors the visit could not meet, a bitmask.
    pub unmet: u8,
    pub wares: Vec<WareFact>,
    pub tryon: Vec<TryOn>,
    /// The chosen set, ranked.
    pub plan: Vec<PlanEntry>,
    /// `[value, bound, lambda, flags]`.
    pub solve: [i64; 4],
    /// The plan's action, written as `paction`: v2's `action` keeps its
    /// meaning (the Borg's goal), and one object cannot hold two keys of
    /// one name.
    pub action: i64,
    /// `[shop, slot_or_item, copies, price]`.
    pub act: [i64; 4],
    /// The need the action served (0..7), -1 for none.
    pub for_need: i64,
    /// 1 when the action was a floor purchase.
    pub floor: bool,
    /// The runner-up: the best option the solve did not take.
    pub displaced: Option<PlanEntry>,
    /// 0 none, 1 the 2000-turn give-up, 2 the anti-bounce give-up, 3 the
    /// solver refused.
    pub guard: u8,
}

// --- writing -------------------------------------------------------------

fn num(out: &mut String, v: impl std::fmt::Display) {
    out.push_str(&v.to_string());
}

fn arr<T>(out: &mut String, items: &[T], mut each: impl FnMut(&mut String, &T)) {
    out.push('[');
    for (i, x) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        each(out, x);
    }
    out.push(']');
}

fn ints(out: &mut String, xs: &[i64]) {
    arr(out, xs, |o, &x| num(o, x));
}

fn entry(e: &PlanEntry) -> [i64; 7] {
    [
        i64::from(e.key_kind),
        e.key,
        e.option as i64,
        i64::from(e.copies),
        e.cost,
        e.value,
        i64::from(e.slots),
    ]
}

impl PlanRecord {
    /// Append the v4 fields, each with its leading comma, in the fixed
    /// order: the caller writes v2's fields (with `"v":4`) before them and
    /// the closing brace after.
    pub fn push_fields(&self, out: &mut String) {
        let field = |out: &mut String, name: &str| {
            out.push_str(",\"");
            out.push_str(name);
            out.push_str("\":");
        };
        field(out, "mode");
        num(out, self.mode);
        field(out, "visit");
        num(out, self.visit);
        field(out, "step");
        num(out, self.step);
        field(out, "gturn");
        num(out, self.gturn);
        field(out, "power");
        num(out, self.power);
        field(out, "params");
        ints(out, &self.params);
        field(out, "limits");
        ints(out, &self.limits);
        field(out, "caps");
        ints(out, &self.caps);
        field(out, "floors");
        arr(out, &self.floors, |o, &(s, sl, n)| ints(o, &[i64::from(s), i64::from(sl), i64::from(n)]));
        field(out, "unmet");
        num(out, self.unmet);
        field(out, "wares");
        arr(out, &self.wares, |o, w| {
            ints(
                o,
                &[
                    i64::from(w.shop),
                    i64::from(w.slot),
                    i64::from(w.kind),
                    i64::from(w.class),
                    i64::from(w.slots_cost),
                    i64::from(w.weight),
                    i64::from(w.need),
                    w.reduction,
                    i64::from(w.serves),
                ],
            )
        });
        field(out, "tryon");
        arr(out, &self.tryon, |o, t| {
            ints(o, &[i64::from(t.shop), i64::from(t.slot), i64::from(t.eq_slot), t.value])
        });
        field(out, "plan");
        arr(out, &self.plan, |o, e| ints(o, &entry(e)));
        field(out, "solve");
        ints(out, &self.solve);
        field(out, "paction");
        num(out, self.action);
        field(out, "act");
        ints(out, &self.act);
        field(out, "for");
        num(out, self.for_need);
        field(out, "floor");
        num(out, i64::from(self.floor));
        field(out, "displaced");
        match &self.displaced {
            Some(e) => ints(out, &entry(e)),
            None => ints(out, &[-1]),
        }
        field(out, "guard");
        num(out, self.guard);
    }
}

// --- reading -------------------------------------------------------------

/// The records' JSON: integers, arrays and objects only.
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Int(i64),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    /// Parse one record line.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn parse(text: &str) -> Result<Json, String> {
        let b = text.as_bytes();
        let mut at = 0;
        let v = parse_value(b, &mut at)?;
        skip_ws(b, &mut at);
        if at != b.len() {
            return Err(format!("trailing input at {at}"));
        }
        Ok(v)
    }

    fn get(&self, key: &str) -> Result<&Json, String> {
        match self {
            Json::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v).ok_or(format!("no field {key}")),
            _ => Err(format!("not an object looking for {key}")),
        }
    }

    fn int(&self) -> Result<i64, String> {
        match self {
            Json::Int(i) => Ok(*i),
            _ => Err("not an integer".into()),
        }
    }

    fn arr(&self) -> Result<&[Json], String> {
        match self {
            Json::Arr(a) => Ok(a),
            _ => Err("not an array".into()),
        }
    }

    fn ints(&self) -> Result<Vec<i64>, String> {
        self.arr()?.iter().map(Json::int).collect()
    }

    /// The object's keys, in order.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn keys(&self) -> Vec<&str> {
        match self {
            Json::Obj(kv) => kv.iter().map(|(k, _)| k.as_str()).collect(),
            _ => Vec::new(),
        }
    }
}

fn skip_ws(b: &[u8], at: &mut usize) {
    while *at < b.len() && b[*at].is_ascii_whitespace() {
        *at += 1;
    }
}

fn parse_value(b: &[u8], at: &mut usize) -> Result<Json, String> {
    skip_ws(b, at);
    match b.get(*at) {
        Some(b'[') => {
            *at += 1;
            let mut v = Vec::new();
            skip_ws(b, at);
            if b.get(*at) == Some(&b']') {
                *at += 1;
                return Ok(Json::Arr(v));
            }
            loop {
                v.push(parse_value(b, at)?);
                skip_ws(b, at);
                match b.get(*at) {
                    Some(b',') => *at += 1,
                    Some(b']') => {
                        *at += 1;
                        return Ok(Json::Arr(v));
                    }
                    _ => return Err(format!("bad array at {at}")),
                }
            }
        }
        Some(b'{') => {
            *at += 1;
            let mut kv = Vec::new();
            skip_ws(b, at);
            if b.get(*at) == Some(&b'}') {
                *at += 1;
                return Ok(Json::Obj(kv));
            }
            loop {
                skip_ws(b, at);
                if b.get(*at) != Some(&b'"') {
                    return Err(format!("bad key at {at}"));
                }
                let start = *at + 1;
                let end = start + b[start..].iter().position(|&c| c == b'"').ok_or("unterminated key")?;
                let key = String::from_utf8_lossy(&b[start..end]).into_owned();
                *at = end + 1;
                skip_ws(b, at);
                if b.get(*at) != Some(&b':') {
                    return Err(format!("no colon at {at}"));
                }
                *at += 1;
                kv.push((key, parse_value(b, at)?));
                skip_ws(b, at);
                match b.get(*at) {
                    Some(b',') => *at += 1,
                    Some(b'}') => {
                        *at += 1;
                        return Ok(Json::Obj(kv));
                    }
                    _ => return Err(format!("bad object at {at}")),
                }
            }
        }
        Some(c) if *c == b'-' || c.is_ascii_digit() => {
            let start = *at;
            *at += 1;
            while *at < b.len() && b[*at].is_ascii_digit() {
                *at += 1;
            }
            std::str::from_utf8(&b[start..*at])
                .ok()
                .and_then(|s| s.parse().ok())
                .map(Json::Int)
                .ok_or(format!("bad integer at {start}"))
        }
        _ => Err(format!("unexpected input at {at}")),
    }
}

fn entry_of(xs: &[i64]) -> Result<PlanEntry, String> {
    if xs.len() != 7 {
        return Err("a plan entry has 7 fields".into());
    }
    Ok(PlanEntry {
        key_kind: xs[0] as u8,
        key: xs[1],
        option: xs[2] as usize,
        copies: xs[3] as i32,
        cost: xs[4],
        value: xs[5],
        slots: xs[6] as i32,
    })
}

fn fixed<const N: usize>(xs: Vec<i64>, what: &str) -> Result<[i64; N], String> {
    xs.try_into().map_err(|_| format!("{what} has the wrong length"))
}

impl PlanRecord {
    /// Read the v4 fields of a parsed record.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn from_json(j: &Json) -> Result<PlanRecord, String> {
        if j.get("v")?.int()? != PLAN_SCHEMA {
            return Err("not a version 4 record".into());
        }
        let small = |k: &str| -> Result<i64, String> { j.get(k)?.int() };
        let rows = |k: &str| -> Result<Vec<Vec<i64>>, String> { j.get(k)?.arr()?.iter().map(Json::ints).collect() };
        let displaced = j.get("displaced")?.ints()?;
        Ok(PlanRecord {
            mode: small("mode")? as u8,
            visit: small("visit")? as u64,
            step: small("step")? as u64,
            gturn: small("gturn")?,
            power: small("power")?,
            params: fixed(j.get("params")?.ints()?, "params")?,
            limits: fixed(j.get("limits")?.ints()?, "limits")?,
            caps: fixed(j.get("caps")?.ints()?, "caps")?,
            floors: rows("floors")?
                .into_iter()
                .map(|r| match r[..] {
                    [s, sl, n] => Ok((s as u8, sl as u8, n as u8)),
                    _ => Err("a floor has 3 fields".to_string()),
                })
                .collect::<Result<_, _>>()?,
            unmet: small("unmet")? as u8,
            wares: rows("wares")?
                .into_iter()
                .map(|r| match r[..] {
                    [shop, slot, kind, class, slots_cost, weight, need, reduction, serves] => Ok(WareFact {
                        shop: shop as u8,
                        slot: slot as u8,
                        kind: kind as u32,
                        class: class as u16,
                        slots_cost: slots_cost as i32,
                        weight: weight as i32,
                        need: need as i8,
                        reduction,
                        serves: serves as u8,
                    }),
                    _ => Err("a ware has 9 fields".to_string()),
                })
                .collect::<Result<_, _>>()?,
            tryon: rows("tryon")?
                .into_iter()
                .map(|r| match r[..] {
                    [shop, slot, eq_slot, value] => {
                        Ok(TryOn { shop: shop as u8, slot: slot as u8, eq_slot: eq_slot as u8, value })
                    }
                    _ => Err("a try-on has 4 fields".to_string()),
                })
                .collect::<Result<_, _>>()?,
            plan: rows("plan")?.iter().map(|r| entry_of(r)).collect::<Result<_, _>>()?,
            solve: fixed(j.get("solve")?.ints()?, "solve")?,
            action: small("paction")?,
            act: fixed(j.get("act")?.ints()?, "act")?,
            for_need: small("for")?,
            floor: small("floor")? != 0,
            displaced: if displaced == [-1] { None } else { Some(entry_of(&displaced)?) },
            guard: small("guard")? as u8,
        })
    }
}

// --- the step: the one core the producer and the replay share ------------

/// What a step reads.
pub struct StepInput<'a> {
    /// The candidates, in stock order.
    pub candidates: Vec<Candidate>,
    pub facts: &'a [WareFact],
    pub tryon: &'a [TryOn],
    pub limits: Limits,
    pub floors: Vec<FloorPurchase>,
    pub params: [i64; PARAMS],
    /// The whole trait vector.
    pub traits: &'a [i32],
    pub power: i64,
    /// The carried items (pack and equipment) as `(tval, quantity)`: the
    /// light need counts the light sources among them.
    pub carried: &'a [(u8, i32)],
}

/// What a step produces.
#[derive(Clone, Debug)]
pub struct StepOutput {
    pub groups: Vec<Group>,
    pub solution: Solution,
    pub plan: Vec<PlanEntry>,
    pub solve: [i64; 4],
    pub displaced: Option<PlanEntry>,
}

/// A state the usefulness function can read: the traits, the power, and
/// the settings with the twelve town settings from `params`.
fn state_of(traits: &[i32], power: i64, params: &[i64; PARAMS], carried: &[(u8, i32)]) -> BorgState {
    let mut st = BorgState::default();
    st.items.items = carried
        .iter()
        .map(|&(tval, qty)| {
            let mut it = crate::borg_item::blank_item();
            it.tval = tval;
            it.iqty = qty.clamp(0, 255) as u8;
            it
        })
        .collect();
    st.core.cfg = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
    for (i, &p) in params.iter().enumerate() {
        st.core.cfg[BORG_TOWN_PLAN as usize + i] = p as i32;
    }
    let mut t = vec![0; BI_MAX as usize];
    t[..traits.len().min(BI_MAX as usize)].copy_from_slice(&traits[..traits.len().min(BI_MAX as usize)]);
    st.borg.trait_ = t;
    st.borg.power = power as i32;
    st
}

fn plan_entry(g: &Group, option: usize) -> PlanEntry {
    let c = &g.options[option];
    let (key_kind, key) = match g.key {
        GroupKey::Class(k) => (0, i64::from(k)),
        GroupKey::Slot(s) => (1, i64::from(s)),
    };
    PlanEntry {
        key_kind,
        key,
        option,
        copies: c.picks.iter().map(|&(_, k)| k).sum(),
        cost: c.cost,
        value: c.value,
        slots: c.slots,
    }
}

/// The runner-up: of the options that would raise a group above what the
/// solve chose, the one adding the most value; ties to less gold, then the
/// earlier group, then the lower option.
fn runner_up(groups: &[Group], s: &Solution) -> Option<PlanEntry> {
    let mut best: Option<(usize, usize, (i64, i64))> = None;
    for (g, &o) in s.chosen.iter().enumerate() {
        let cur = &groups[g].options[o];
        for (j, alt) in groups[g].options.iter().enumerate() {
            let gain = alt.value - cur.value;
            if gain <= 0 {
                continue;
            }
            let key = (gain, -(alt.cost - cur.cost));
            if best.is_none_or(|(_, _, k)| key > k) {
                best = Some((g, j, key));
            }
        }
    }
    best.map(|(g, j, _)| plan_entry(&groups[g], j))
}

/// One plan step: the groups, the solve, the ranked plan and the runner-up.
pub fn step(input: &StepInput) -> StepOutput {
    let st = state_of(input.traits, input.power, &input.params, input.carried);
    let trip = Trip::of(&st);
    let weights = Weights::of(&st.core.cfg);
    let facts: BTreeMap<(u8, u8), &WareFact> = input.facts.iter().map(|f| ((f.shop, f.slot), f)).collect();
    let tryon: BTreeMap<(u8, u8), &TryOn> = input.tryon.iter().map(|t| ((t.shop, t.slot), t)).collect();
    let stock = Stock { candidates: input.candidates.clone(), ..Stock::default() };
    let budget = Budget {
        gold: input.limits.gold,
        pack_slots: input.limits.pack_slots,
        home_slots: 0,
        weight: input.limits.weight,
        floors: input.floors.clone(),
        unmet: Vec::new(),
    };
    let caution = weights.caution;
    let groups = groups(
        &stock,
        &budget,
        caution,
        |c| tryon.get(&(c.shop, c.slot)).map(|t| (t.eq_slot, t.value)),
        |c, n| match facts.get(&(c.shop, c.slot)) {
            Some(f) if f.need >= 0 => {
                usefulness(Need::ALL[f.need as usize], f.reduction, i64::from(n), &st, &trip, &weights)
            }
            _ => 0,
        },
    );
    let solution = solve(&groups, &input.limits);
    let plan = solution.ranked(&groups).into_iter().map(|(g, o)| plan_entry(&groups[g], o)).collect();
    let flags = [
        (solution.optimal, flag::OPTIMAL),
        (solution.gold_exact, flag::GOLD_EXACT),
        (solution.slots_met, flag::SLOTS_MET),
        (solution.weight_met, flag::WEIGHT_MET),
        (solution.weight_repaired, flag::WEIGHT_REPAIRED),
    ]
    .iter()
    .filter(|(on, _)| *on)
    .fold(0, |m, (_, b)| m | b);
    let solve_row = [solution.value, solution.bound, solution.lambda, flags];
    let displaced = runner_up(&groups, &solution);
    StepOutput { groups, solution, plan, solve: solve_row, displaced }
}

/// The step's action: the floors first (B3, Part A), then the first ranked
/// option of the plan, else leave. Returns `(paction, act, for, floor)`,
/// `act` = `[shop, slot, copies, price]`; a purchase takes its cheapest
/// ware first, as the cost table does.
pub fn first_action(
    candidates: &[Candidate],
    floors: &[FloorPurchase],
    out: &StepOutput,
    facts: &[WareFact],
) -> (i64, [i64; 4], i64, bool) {
    let need_of = |c: &Candidate| {
        facts.iter().find(|f| f.shop == c.shop && f.slot == c.slot).map_or(-1, |f| i64::from(f.need))
    };
    if let Some(f) = floors.first() {
        let c = &candidates[f.candidate];
        let need = f.needs.first().and_then(|n| Need::ALL.iter().position(|x| x == n)).map_or(-1, |i| i as i64);
        return (action::BUY, [i64::from(c.shop), i64::from(c.slot), 1, i64::from(c.price)], need, true);
    }
    if let Some(&(g, o)) = out.solution.ranked(&out.groups).first() {
        if let Some(&(i, copies)) = out.groups[g].options[o].picks.first() {
            let c = &candidates[i];
            return (
                action::BUY,
                [i64::from(c.shop), i64::from(c.slot), i64::from(copies), i64::from(c.price)],
                need_of(c),
                false,
            );
        }
    }
    (action::LEAVE, [-1, -1, -1, -1], -1, false)
}

// --- replay --------------------------------------------------------------

/// Replay a v4 line: rebuild the step's inputs from the line alone and run
/// it. The caller compares the output with the record's `plan`, `solve`
/// and `displaced` ([`replays`] does).
#[cfg_attr(not(test), expect(dead_code))]
pub fn replay(line: &str) -> Result<(PlanRecord, StepOutput), String> {
    let j = Json::parse(line)?;
    let rec = PlanRecord::from_json(&j)?;

    // v2's shops: [shop, [[slot, tval, sval, quantity, price, flags], ...]].
    let mut shop_rows: BTreeMap<(u8, u8), Vec<i64>> = BTreeMap::new();
    for s in j.get("shops")?.arr()? {
        let s = s.arr()?;
        let shop = s.first().ok_or("an empty shop")?.int()? as u8;
        for w in s.get(1).ok_or("a shop without wares")?.arr()? {
            let w = w.ints()?;
            shop_rows.insert((shop, *w.first().ok_or("an empty ware")? as u8), w);
        }
    }
    // The candidates, in the producer's (shop, slot) order.
    let mut facts = rec.wares.clone();
    facts.sort_by_key(|f| (f.shop, f.slot));
    let candidates: Vec<Candidate> = facts
        .iter()
        .map(|f| {
            let w = shop_rows.get(&(f.shop, f.slot)).ok_or(format!("ware {}:{} not in shops", f.shop, f.slot))?;
            match w[..] {
                [_, tval, sval, qty, price, _] => Ok(Candidate {
                    shop: f.shop,
                    slot: f.slot,
                    kind: f.kind,
                    tval: tval as u8,
                    sval: sval as u8,
                    price: price as i32,
                    qty: qty as i32,
                    weight: f.weight,
                    slots_cost: f.slots_cost,
                    stack_count: 0,
                    class: f.class,
                }),
                _ => Err("a v2 ware has 6 fields".to_string()),
            }
        })
        .collect::<Result<_, String>>()?;
    let index: BTreeMap<(u8, u8), usize> =
        candidates.iter().enumerate().map(|(i, c)| ((c.shop, c.slot), i)).collect();
    let floors = rec
        .floors
        .iter()
        .map(|&(s, sl, n)| {
            let candidate = *index.get(&(s, sl)).ok_or("a floor ware not in wares")?;
            let needs = Need::ALL.iter().enumerate().filter(|(b, _)| n & (1 << b) != 0).map(|(_, &x)| x).collect();
            Ok(FloorPurchase { candidate, needs })
        })
        .collect::<Result<_, String>>()?;

    // v2's traits, sparse.
    let mut traits = vec![0i32; BI_MAX as usize];
    for t in j.get("traits")?.arr()? {
        if let [i, v] = t.ints()?[..] {
            if let Some(slot) = traits.get_mut(i as usize) {
                *slot = v as i32;
            }
        }
    }

    // v2's pack and equipment: [slot, tval, sval, quantity, value, flags].
    let mut carried = Vec::new();
    for key in ["pack", "equip"] {
        for it in j.get(key)?.arr()? {
            if let [_, tval, _, qty, ..] = it.ints()?[..] {
                carried.push((tval as u8, qty as i32));
            }
        }
    }

    let limits = Limits { gold: rec.limits[0], pack_slots: rec.limits[1] as i32, weight: rec.limits[3] };
    let out = step(&StepInput {
        candidates,
        facts: &rec.wares,
        tryon: &rec.tryon,
        limits,
        floors,
        params: rec.params,
        traits: &traits,
        power: rec.power,
        carried: &carried,
    });
    Ok((rec, out))
}

/// Does a v4 line replay to its own plan, solve and runner-up?
#[cfg_attr(not(test), expect(dead_code))]
pub fn replays(line: &str) -> Result<bool, String> {
    let (rec, out) = replay(line)?;
    Ok(rec.plan == out.plan && rec.solve == out.solve && rec.displaced == out.displaced)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borg_headers::borg_trait_h::{BI_CARRY, BI_FOOD, BI_LIGHT, BI_MAXDEPTH};
    use crate::borg_town_budget::Facts;
    use crate::borg_town_stock::tests::{build, item, town};

    // Test kinds (the helpers' tval is the kind).
    const TORCH: u32 = 1;
    const RATION: u32 = 2;
    const CLW: u32 = 3;
    const PHASE: u32 = 5;
    const ARMOUR: u32 = 7;
    const ARMOUR_2: u32 = 8;

    /// The need and reduction a test kind serves, and the floors it
    /// discharges - the role the engine's tables play in production.
    fn role(kind: u32) -> (i8, i64, u8) {
        match kind {
            TORCH => (0, 1, 1 << 0),
            RATION => (1, 1, 1 << 1),
            CLW => (2, 1, 1 << 2),
            PHASE => (4, 1, 0),
            _ => (-1, 0, 0),
        }
    }

    fn params() -> [i64; PARAMS] {
        let d: Vec<i64> = crate::borg_init::BORG_SETTINGS.iter().map(|s| i64::from(s.default_value)).collect();
        let mut p = [0; PARAMS];
        p.copy_from_slice(&d[BORG_TOWN_PLAN as usize..BORG_TOWN_PLAN as usize + PARAMS]);
        p
    }

    /// The v2 part of a record, as the dump writes it: only the fields a
    /// replay reads are filled with meaning.
    fn v2_prefix(stock: &Stock, traits: &[i32], gold: i64) -> String {
        let mut out = String::from("{\"v\":4,\"seq\":0,\"turn\":0,\"depth\":0,\"level\":1,\"chp\":10,\"mhp\":10,");
        out.push_str(&format!("\"gold\":{gold},\"race\":0,\"class\":0,\"traits\":["));
        let tr: Vec<String> =
            traits.iter().enumerate().filter(|(_, &v)| v != 0).map(|(i, v)| format!("[{i},{v}]")).collect();
        out.push_str(&tr.join(","));
        out.push_str("],\"action\":0,\"shop\":-1,\"ware\":-1,\"item\":-1,\"price\":-1,");
        // the carried light: one torch worn, of the two wanted
        out.push_str("\"pack\":[],\"equip\":[[29,19,0,1,0,1]],\"home\":[],\"shops\":[");
        let mut by_shop: BTreeMap<u8, Vec<String>> = BTreeMap::new();
        for c in &stock.candidates {
            by_shop
                .entry(c.shop)
                .or_default()
                .push(format!("[{},{},{},{},{},1]", c.slot, c.tval, c.sval, c.qty, c.price));
        }
        let shops: Vec<String> = by_shop.iter().map(|(s, w)| format!("[{s},[{}]]", w.join(","))).collect();
        out.push_str(&shops.join(","));
        out.push(']');
        out
    }

    /// A whole step as the producer will run it, written as a v4 line.
    fn produce(stock: &Stock, gold: i64, traits: &[i32], tryon: &[TryOn], unmet: &[Need]) -> (String, PlanRecord) {
        let facts = Facts { gold, pack_capacity: 23, home_capacity: 24, carry: traits[BI_CARRY as usize], weight: 0 };
        let budget = Budget::build(&facts, stock, unmet, |c| {
            let (_, _, serves) = role(c.kind);
            Need::ALL.iter().enumerate().filter(|(b, _)| serves & (1 << b) != 0).map(|(_, &n)| n).collect()
        });
        let wares: Vec<WareFact> = stock
            .candidates
            .iter()
            .map(|c| {
                let (need, reduction, serves) = role(c.kind);
                WareFact {
                    shop: c.shop,
                    slot: c.slot,
                    kind: c.kind,
                    class: c.class,
                    slots_cost: c.slots_cost,
                    weight: c.weight,
                    need,
                    reduction,
                    serves,
                }
            })
            .collect();
        let limits = Limits::of(&budget);
        let out = step(&StepInput {
            candidates: stock.candidates.clone(),
            facts: &wares,
            tryon,
            limits,
            floors: budget.floors.clone(),
            params: params(),
            traits,
            power: 1000,
            carried: &carried(),
        });
        let mask = |ns: &[Need]| {
            ns.iter().fold(0u8, |m, n| m | (1 << Need::ALL.iter().position(|x| x == n).unwrap()))
        };
        let rec = PlanRecord {
            mode: 0,
            visit: 3,
            step: 1,
            gturn: 12345,
            power: 1000,
            params: params(),
            limits: [budget.gold, i64::from(budget.pack_slots), i64::from(budget.home_slots), budget.weight],
            caps: [23, 24],
            floors: budget
                .floors
                .iter()
                .map(|f| (stock.candidates[f.candidate].shop, stock.candidates[f.candidate].slot, mask(&f.needs)))
                .collect(),
            unmet: mask(&budget.unmet),
            wares,
            tryon: tryon.to_vec(),
            plan: out.plan.clone(),
            solve: out.solve,
            action: if out.plan.is_empty() { action::LEAVE } else { action::BUY },
            act: [-1, -1, -1, -1],
            for_need: -1,
            floor: false,
            displaced: out.displaced,
            guard: 0,
        };
        let mut line = v2_prefix(stock, traits, gold);
        rec.push_fields(&mut line);
        line.push('}');
        (line, rec)
    }

    /// The carried items as the step reads them: one torch (the light
    /// tval), matching v2_prefix's equipment row.
    fn carried() -> Vec<(u8, i32)> {
        vec![(crate::headers::list_tvals_h::TV_LIGHT as u8, 1)]
    }

    fn traits() -> Vec<i32> {
        let mut t = vec![0; BI_MAX as usize];
        t[BI_MAXDEPTH as usize] = 10; // depth target 15: two lights wanted
        t[BI_LIGHT as usize] = 1;
        t[BI_FOOD as usize] = 3; // seven rations short of ten
        t[BI_CARRY as usize] = 2000;
        t
    }

    /// The general store, the alchemist, an armoury and the Black Market.
    fn stock() -> Stock {
        let t = town([
            &[item(TORCH, 5, 2), item(RATION, 20, 4), item(PHASE, 10, 15)],
            &[item(ARMOUR, 1, 200), item(ARMOUR_2, 1, 260)],
            &[],
            &[],
            &[item(CLW, 5, 20)],
            &[],
            &[item(RATION, 5, 12)],
            &[],
        ]);
        build(&t, &[])
    }

    fn armour_tryon(s: &Stock) -> Vec<TryOn> {
        s.candidates
            .iter()
            .filter(|c| c.kind == ARMOUR || c.kind == ARMOUR_2)
            .map(|c| TryOn { shop: c.shop, slot: c.slot, eq_slot: 6, value: if c.kind == ARMOUR { 30 } else { 34 } })
            .collect()
    }

    /// The contract: a record replays to its own plan, solve and
    /// runner-up, from the line alone.
    #[test]
    fn a_record_replays_to_its_own_plan() {
        let s = stock();
        for gold in [0, 9, 60, 300, 2_000] {
            let (line, rec) = produce(&s, gold, &traits(), &armour_tryon(&s), &[Need::Food, Need::Poison]);
            assert!(replays(&line).unwrap(), "gold {gold}: {line}");
            let (_, out) = replay(&line).unwrap();
            assert_eq!(out.plan, rec.plan);
        }
    }

    /// The plan is not empty and buys what the deficits want: rations
    /// for the food gap, a second light for the depth, and the better
    /// armour rather than both (Q1).
    #[test]
    fn the_step_plans_what_the_deficits_want() {
        let s = stock();
        let (_, rec) = produce(&s, 2_000, &traits(), &armour_tryon(&s), &[]);
        assert!(!rec.plan.is_empty());
        let slot_picks: Vec<&PlanEntry> = rec.plan.iter().filter(|e| e.key_kind == 1).collect();
        assert_eq!(slot_picks.len(), 1, "one armour for one slot");
        assert_eq!(slot_picks[0].value, 34);
        assert!(rec.solve[3] & flag::GOLD_EXACT != 0);
    }

    /// A record whose inputs were tampered with does not replay to its
    /// plan: replay is a real check, not an echo.
    #[test]
    fn a_tampered_record_does_not_replay() {
        let s = stock();
        let (line, _) = produce(&s, 300, &traits(), &armour_tryon(&s), &[]);
        // the 1x rations now cost 40
        let tampered = line.replacen("[1,2,0,20,4,1]", "[1,2,0,20,40,1]", 1);
        assert_ne!(tampered, line, "the ware was found");
        assert!(!replays(&tampered).unwrap());
        // so does a different arm
        let (rec, _) = replay(&line).unwrap();
        let mut p = rec.params;
        p[1] = 0; // w_light
        let mut l2 = line.clone();
        let from = format!("\"params\":[{}]", rec.params.map(|x| x.to_string()).join(","));
        l2 = l2.replacen(&from, &format!("\"params\":[{}]", p.map(|x| x.to_string()).join(",")), 1);
        assert!(!replays(&l2).unwrap());
    }

    /// The v4 fields round-trip through the reader exactly.
    #[test]
    fn the_fields_round_trip() {
        let s = stock();
        let (line, rec) = produce(&s, 300, &traits(), &armour_tryon(&s), &[Need::Food]);
        let back = PlanRecord::from_json(&Json::parse(&line).unwrap()).unwrap();
        assert_eq!(back, rec);
    }

    /// P13's stable field order: v2's fields first, unchanged, then the
    /// plan's, in this order; and no key appears twice.
    #[test]
    fn the_field_order_is_fixed() {
        let s = stock();
        let (line, _) = produce(&s, 300, &traits(), &armour_tryon(&s), &[]);
        let j = Json::parse(&line).unwrap();
        let keys = j.keys();
        let v4 = [
            "mode", "visit", "step", "gturn", "power", "params", "limits", "caps", "floors", "unmet", "wares",
            "tryon", "plan", "solve", "paction", "act", "for", "floor", "displaced", "guard",
        ];
        assert_eq!(&keys[keys.len() - v4.len()..], v4);
        assert_eq!(&keys[..2], ["v", "seq"]);
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), keys.len(), "a key appears twice");
    }

    /// The action codes: v2's five unchanged, the plan's three after them.
    #[test]
    fn the_action_codes_extend_v2s() {
        let codes = [
            action::VISIT,
            action::BUY,
            action::TAKE_HOME,
            action::SELL,
            action::SELL_HOME,
            action::STORE,
            action::DROP,
            action::LEAVE,
        ];
        assert_eq!(codes, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    /// The step's solution and groups are the ones its plan and solve
    /// rows were read from.
    #[test]
    fn the_step_output_agrees_with_itself() {
        let s = stock();
        let (line, _) = produce(&s, 300, &traits(), &armour_tryon(&s), &[]);
        let (rec, out) = replay(&line).unwrap();
        assert_eq!(out.solution.value, rec.solve[0]);
        assert_eq!(out.solution.bound, rec.solve[1]);
        let chosen = out.solution.chosen.iter().filter(|&&o| o != 0).count();
        assert_eq!(chosen, rec.plan.len());
        assert!(out.groups.len() >= chosen);
    }

    /// The step's action: a floor before anything else; else the plan's
    /// first ranked purchase; else leave.
    #[test]
    fn the_first_action_is_the_floor_then_the_plan_then_leave() {
        let s = stock();
        let wares = |s: &Stock| -> Vec<WareFact> {
            s.candidates
                .iter()
                .map(|c| {
                    let (need, reduction, serves) = role(c.kind);
                    WareFact { shop: c.shop, slot: c.slot, kind: c.kind, class: c.class, slots_cost: 1, weight: 10, need, reduction, serves }
                })
                .collect()
        };
        let run = |gold: i64, unmet: &[Need]| {
            let (line, _) = produce(&s, gold, &traits(), &[], unmet);
            let (rec, out) = replay(&line).unwrap();
            let idx: BTreeMap<(u8, u8), usize> = s.candidates.iter().enumerate().map(|(i, c)| ((c.shop, c.slot), i)).collect();
            let floors: Vec<FloorPurchase> = rec
                .floors
                .iter()
                .map(|&(sh, sl, n)| FloorPurchase {
                    candidate: idx[&(sh, sl)],
                    needs: Need::ALL.iter().enumerate().filter(|(b, _)| n & (1 << b) != 0).map(|(_, &x)| x).collect(),
                })
                .collect();
            first_action(&s.candidates, &floors, &out, &wares(&s))
        };
        // the food floor unmet: the floor purchase comes first
        let (a, act, need, floor) = run(300, &[Need::Food]);
        assert_eq!((a, need, floor), (action::BUY, 1, true));
        assert_eq!(act[0], 0, "the general store's ration");
        // no floor: the plan's first purchase
        let (a, _, _, floor) = run(300, &[]);
        assert_eq!((a, floor), (action::BUY, false));
        // nothing affordable: leave
        let (a, act, _, _) = run(0, &[]);
        assert_eq!((a, act), (action::LEAVE, [-1, -1, -1, -1]));
    }

    /// The light need counts light sources: the replay reads them from
    /// v2's pack and equipment rows, so a line with a spare torch in the
    /// pack prices one light less than one without.
    #[test]
    fn the_replay_counts_the_carried_lights() {
        let s = stock();
        let (line, _) = produce(&s, 2_000, &traits(), &[], &[]);
        let (_, out) = replay(&line).unwrap();
        let spare = line.replacen("\"pack\":[]", "\"pack\":[[0,19,0,1,0,1]]", 1);
        assert_ne!(spare, line);
        let (_, out2) = replay(&spare).unwrap();
        let light = |o: &StepOutput| o.plan.iter().find(|e| e.key_kind == 0 && e.key == 0).map(|e| e.value);
        assert_eq!(light(&out), Some(100), "one torch of two: the second is worth the whole gap");
        assert_eq!(light(&out2), None, "two carried: light is met");
    }

    /// Every record of a real run replays to its own plan. Run by hand on
    /// a file written with `BORG_TOWN_PLAN_RECORD`:
    /// `PLAN_RECORD_FILE=... cargo test --release -- --ignored --nocapture
    /// a_run_s_records_all_replay`.
    #[test]
    #[ignore]
    fn a_run_s_records_all_replay() {
        let path = std::env::var("PLAN_RECORD_FILE").expect("PLAN_RECORD_FILE");
        let text = std::fs::read_to_string(&path).unwrap();
        let (mut n, mut planned, mut floors) = (0, 0, 0);
        for (i, line) in text.lines().enumerate() {
            let (rec, out) = replay(line).unwrap_or_else(|e| panic!("line {}: {e}", i + 1));
            assert_eq!(out.plan, rec.plan, "line {}: plan", i + 1);
            assert_eq!(out.solve, rec.solve, "line {}: solve", i + 1);
            assert_eq!(out.displaced, rec.displaced, "line {}: runner-up", i + 1);
            n += 1;
            planned += usize::from(!rec.plan.is_empty());
            floors += usize::from(rec.floor);
        }
        assert!(n > 0, "no records");
        println!("replayed {n} records: {planned} with a plan, {floors} taking a floor first");
    }

    /// A version-2 line is not read as a plan record.
    #[test]
    fn a_v2_line_is_refused() {
        let j = Json::parse("{\"v\":2,\"seq\":0}").unwrap();
        assert!(PlanRecord::from_json(&j).is_err());
    }

    /// The reader takes what the writer writes, and nothing else.
    #[test]
    fn the_reader_rejects_malformed_input() {
        assert!(Json::parse("{\"a\":[1,-2,[3]]}").is_ok());
        assert!(Json::parse("{\"a\":1,}").is_err());
        assert!(Json::parse("{\"a\":1} x").is_err());
        assert!(Json::parse("[1,2").is_err());
    }
}
