//! Physical capabilities shared by purchases, dispositions and replay.
//!
//! Rows are engine facts, not borg_power bonuses. The eight legacy needs
//! remain intact for historical replay. New records carry these curves and
//! per-copy supply vectors, including every secondary effect.
use std::collections::{BTreeMap, BTreeSet};
use crate::borg_headers::{borg_h::*, borg_trait_h::*};
use crate::borg_town_dispose::{KitItem, Place};
use crate::borg_town_stock::Stock;
use crate::borg_town_utility::{Journey, UNIT};
use crate::borg_headers::borg_item_h::borg_item;
use crate::headers::list_tvals_h::*;
use crate::headers::list_effects_h::*;
use crate::headers::list_player_timed_h::*;
use crate::state::BorgState;
use crate::engine;

pub const PACK: i64 = 8;
pub const HOME: i64 = 9;
const LIGHT: usize = 0;
const FOOD: usize = 1;
const HEAL: usize = 2;
const POISON: usize = 3;
const PHASE: usize = 4;
const RECALL: usize = 5;
const ATTACK: usize = 6;
const TELEPORT: usize = 7;
const LEVEL: usize = 8;
const OTHER: usize = 9;
const DIG: usize = 10;
const TRAPS: usize = 11;
const MONSTERS: usize = 12;
const INVISIBLE: usize = 13;
const MAP: usize = 14;
const ILLUMINATE: usize = 15;
const SPEED: usize = 16;
const FIRE: usize = 17;
const COLD: usize = 18;
const RESPOIS: usize = 19;
const ACID: usize = 20;
const ELEC: usize = 21;
const BUFF: usize = 22;
const MANA: usize = 23;
const EXPERIENCE: usize = 24;
const CURSE: usize = 25;
const ENCHANT: usize = 26;
const RECHARGE: usize = 27;
const CONFUSION: usize = 28;
const BLIND: usize = 29;
const FEAR: usize = 30;
const GLYPH: usize = 31;
const STATS: usize = 32;
const BOOKS: usize = 38;
const BANISH: usize = 47;
const DESTROY: usize = 48;
const WOUNDS: usize = 49;
const IDENTIFY: usize = 50;
const GAIN_EXPERIENCE: usize = 51;
const RECHARGE_TARGET: usize = 52;
const COUNT: usize = NAMES.len();

pub const NAMES: [&str; 53] = ["light turns", "nutrition", "healing HP", "poison cure", "phase", "Recall",
    "ranged damage", "teleport", "level escape", "teleport other", "digging", "trap/door detection",
    "monster detection", "invisible detection", "mapping", "illumination", "speed", "resist fire",
    "resist cold", "resist poison", "resist acid", "resist electricity", "combat buff", "mana",
    "restore experience", "remove curse", "enchantment", "recharging", "confusion cure", "blindness cure",
    "fear cure", "glyph", "strength", "intelligence", "wisdom", "dexterity", "constitution", "charisma",
    "book 0", "book 1", "book 2", "book 3", "book 4", "book 5", "book 6", "book 7", "book 8",
    "banishment", "destruction", "wound/stun cure", "identify runes", "gain experience", "recharge target"];

/// Curves: [capability, weight in utility thousandths, demand, carried, Home].
/// Supplies: [scope (shop 0..7, pack 8, Home 9), slot, capability, units/copy].
/// All dynamic class/level/effect facts are frozen here for engine-free replay.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Model {
    pub curves: Vec<[i64; 5]>,
    pub supplies: Vec<[i64; 4]>,
}

fn coverage(row: &[i64; 5], amount: i64) -> i64 {
    if amount <= 0 || row[1] <= 0 { return 0; }
    // Negative demand is an actual one-time repair/access requirement,
    // not a stock quota: a second cure cannot restore the same lost XP.
    if row[2] < 0 { return row[1] * amount.min(-row[2]) / -row[2]; }
    row[1].saturating_mul(amount) / amount.saturating_add(row[2].max(1))
}

impl Model {
    pub fn units(&self, scope: i64, slot: i64, count: i64) -> Vec<(usize, i64)> {
        self.supplies.iter().filter(|r| r[0] == scope && r[1] == slot)
            .map(|r| (r[2] as usize, r[3] * count)).collect()
    }
    pub fn gain(&self, units: &[(usize, i64)]) -> i64 {
        let mut sum = BTreeMap::<usize, i64>::new();
        for &(id, n) in units { *sum.entry(id).or_default() += n; }
        sum.into_iter().map(|(id, n)| {
            let r = &self.curves[id];
            if id == RECHARGE && self.curves[RECHARGE_TARGET][3] == 0 { return 0; }
            coverage(r, (r[3] + n).max(0)) - coverage(r, r[3])
        }).sum()
    }
    pub fn ware(&self, shop: u8, slot: u8, count: i32) -> i64 {
        self.gain(&self.units(i64::from(shop), i64::from(slot), i64::from(count)))
    }
    pub fn scope(it: &KitItem) -> i64 { if it.place == Place::Pack { PACK } else { HOME } }
    pub fn keep(&self, it: &KitItem) -> i64 {
        -self.gain(&self.units(Self::scope(it), i64::from(it.slot), -i64::from(it.qty)))
    }
    pub fn take(&self, it: &KitItem) -> i64 {
        self.gain(&self.units(Self::scope(it), i64::from(it.slot), i64::from(it.qty)))
    }
    pub fn reserve(&self, it: &KitItem, sign: i64, discount: i64) -> i64 {
        self.units(Self::scope(it), i64::from(it.slot), i64::from(it.qty)).into_iter().map(|(id, n)| {
            let r = &self.curves[id];
            let from = r[4];
            let to = (from + sign * n).max(0);
            (coverage(r, to) - coverage(r, from)) * discount / UNIT
        }).sum()
    }
    pub fn change(&mut self, it: &KitItem, sign: i64) {
        for (id, n) in self.units(Self::scope(it), i64::from(it.slot), sign * i64::from(it.qty)) {
            self.curves[id][3] = (self.curves[id][3] + n).max(0);
        }
    }
    pub fn change_home(&mut self, it: &KitItem, sign: i64) {
        for (id, n) in self.units(Self::scope(it), i64::from(it.slot), sign * i64::from(it.qty)) {
            self.curves[id][4] = (self.curves[id][4] + n).max(0);
        }
    }
    pub fn mask(&self, shop: u8, slot: u8) -> u64 {
        self.units(i64::from(shop), i64::from(slot), 1).iter().fold(0, |m, &(id, _)| m | (1 << id))
    }
    pub fn valid(&self) -> bool {
        self.curves.len() == COUNT && self.curves.iter().enumerate().all(|(i, r)| r[0] == i as i64 && r[1] >= 0 && r[3] >= 0 && r[4] >= 0)
            && self.supplies.iter().all(|r| (0..=HOME).contains(&r[0]) && r[1] >= 0 && (0..COUNT as i64).contains(&r[2]) && r[3] > 0)
    }
}

/// Context for the pure effect classifier; engine facts are obtained once.
struct Context<'a> {
    st: &'a BorgState,
    journey: Journey,
    lantern: bool,
}

fn timed_cap(timed: u32) -> Option<usize> {
    match timed {
        TMD_FAST | TMD_SPRINT => Some(SPEED),
        TMD_OPP_FIRE => Some(FIRE), TMD_OPP_COLD => Some(COLD), TMD_OPP_POIS => Some(RESPOIS),
        TMD_OPP_ACID => Some(ACID), TMD_OPP_ELEC => Some(ELEC),
        TMD_HERO | TMD_SHERO | TMD_BOLD | TMD_BLESSED | TMD_SHIELD | TMD_STONESKIN | TMD_PROTEVIL
            | TMD_INVULN | TMD_ATT_CONF | TMD_ATT_ACID | TMD_ATT_ELEC | TMD_ATT_FIRE | TMD_ATT_COLD
            | TMD_ATT_POIS | TMD_ATT_EVIL | TMD_ATT_DEMON | TMD_ATT_VAMP | TMD_POWERSHOT => Some(BUFF),
        TMD_SINVIS => Some(INVISIBLE), TMD_TELEPATHY => Some(MONSTERS),
        _ => None,
    }
}

/// Effects with contextual relevance, in physical units. Metadata/control
/// effects do not earn utility by themselves. Harmful effects never do.
fn effect_supply(effects: &[(u16, i32, i64)], hp: i64) -> Vec<(usize, i64)> {
    use crate::headers::list_projections_h::*;
    let mut out = BTreeMap::<usize, i64>::new();
    for &(effect, subtype, amount) in effects {
        let n = amount.max(1) * UNIT;
        let role = match u32::from(effect) {
            EF_NOURISH if subtype != 1 && amount > 0 => Some((FOOD, n)), // A decreasing food effect is not nourishment.
            EF_HEAL_HP => Some((HEAL, amount.max(1).min(hp) * UNIT)),
            EF_CURE | EF_TIMED_DEC => match subtype as u32 {
                TMD_POISONED => Some((POISON, UNIT)), TMD_CONFUSED => Some((CONFUSION, UNIT)),
                TMD_BLIND => Some((BLIND, UNIT)), TMD_AFRAID => Some((FEAR, UNIT)),
                TMD_CUT | TMD_STUN => Some((WOUNDS, UNIT)),
                _ => None,
            },
            EF_TIMED_INC | EF_TIMED_INC_NO_RES | EF_TIMED_SET => timed_cap(subtype as u32).map(|cap| (cap, n)),
            EF_TELEPORT => Some((TELEPORT, UNIT)), EF_TELEPORT_LEVEL => Some((LEVEL, UNIT)),
            EF_TELEPORT_TO => Some((OTHER, UNIT)), EF_RECALL => Some((RECALL, UNIT)),
            EF_RESTORE_MANA => Some((MANA, n)), EF_RESTORE_EXP => Some((EXPERIENCE, UNIT)),
            EF_GAIN_EXP => Some((GAIN_EXPERIENCE, UNIT)),
            EF_IDENTIFY => Some((IDENTIFY, UNIT)),
            EF_RESTORE_STAT | EF_GAIN_STAT if (0..5).contains(&subtype) => Some((STATS + subtype as usize, UNIT)),
            EF_REMOVE_CURSE => Some((CURSE, n)), EF_ENCHANT | EF_BRAND_WEAPON | EF_BRAND_AMMO | EF_BRAND_BOLTS => Some((ENCHANT, UNIT)),
            EF_RECHARGE => Some((RECHARGE, UNIT)),
            EF_MAP_AREA | EF_LIGHT_LEVEL => Some((MAP, UNIT)), EF_LIGHT_AREA => Some((ILLUMINATE, UNIT)),
            EF_DETECT_TRAPS | EF_DETECT_DOORS | EF_DETECT_STAIRS => Some((TRAPS, UNIT)),
            EF_DETECT_EVIL | EF_DETECT_VISIBLE_MONSTERS | EF_DETECT_LIVING_MONSTERS | EF_DETECT_SOUL | EF_READ_MINDS => Some((MONSTERS, UNIT)),
            EF_DETECT_INVISIBLE_MONSTERS => Some((INVISIBLE, UNIT)),
            EF_DETECT_OBJECTS | EF_SENSE_OBJECTS | EF_DETECT_GOLD | EF_SENSE_GOLD | EF_DETECT_ORE | EF_PROBE => Some((MAP, UNIT)),
            EF_GLYPH => Some((GLYPH, UNIT)), EF_BANISH | EF_MASS_BANISH => Some((BANISH, UNIT)),
            EF_DESTRUCTION | EF_EARTHQUAKE => Some((DESTROY, UNIT)),
            EF_BOLT | EF_BEAM | EF_BOLT_OR_BEAM | EF_BOLT_AWARE | EF_LINE | EF_BALL | EF_TOUCH | EF_PROJECT_LOS | EF_PROJECT_LOS_AWARE
                if matches!(subtype as u32, PROJ_AWAY_ALL | PROJ_AWAY_EVIL | PROJ_AWAY_UNDEAD | PROJ_AWAY_SPIRIT) => Some((OTHER, UNIT)),
            EF_BOLT | EF_BEAM | EF_LINE | EF_TOUCH | EF_BOLT_AWARE if subtype as u32 == PROJ_KILL_WALL => Some((DIG, UNIT)),
            EF_BOLT | EF_BEAM | EF_TOUCH | EF_BOLT_AWARE if matches!(subtype as u32, PROJ_MON_HEAL | PROJ_MON_SPEED | PROJ_MON_CLONE | PROJ_MON_POLY) => None,
            EF_LINE | EF_BOLT | EF_BOLT_OR_BEAM | EF_BEAM | EF_BALL | EF_BREATH | EF_ARC | EF_SHORT_BEAM
                | EF_SWARM | EF_STRIKE | EF_STAR | EF_STAR_BALL | EF_SPOT | EF_SPHERE
                | EF_PROJECT_LOS | EF_PROJECT_LOS_AWARE | EF_BOLT_AWARE | EF_WONDER => Some((ATTACK, n)),
            EF_BOLT_STATUS | EF_BOLT_STATUS_DAM | EF_TOUCH | EF_TOUCH_AWARE | EF_MON_TIMED_INC => Some((BUFF, UNIT)),
            _ => None,
        };
        if let Some((cap, units)) = role { *out.entry(cap).or_default() += units; }
    }
    out.into_iter().collect()
}

fn supplies(item: &borg_item, ctx: &Context) -> Vec<(usize, i64)> {
    if item.iqty == 0 || item.kind == 0 { return Vec::new(); }
    let tval = u32::from(item.tval);
    // A shop teaches its kind on purchase; carried unknown flavours/devices
    // must not exploit diagnostic hidden kind data.
    if !item.aware { return Vec::new(); }
    let st = ctx.st;
    let qty = i64::from(item.iqty).max(1);
    let mut effects = engine::kind_effect_values(item.kind as usize, ctx.journey.level as i32);
    if tval == TV_SCROLL && i32::from(item.sval) == st.item_val.sv_scroll_phase_door {
        return vec![(PHASE, UNIT)]; // TELEPORT(10) is not full Teleportation.
    }
    if tval == TV_FLASK {
        let mut out = vec![(ATTACK, (i64::from(item.dd) * (i64::from(item.ds) + 1) * UNIT / 2).max(UNIT))];
        if ctx.lantern && item.pval > 0 { out.push((LIGHT, i64::from(item.pval) * UNIT)); }
        return out;
    }
    if matches!(tval, TV_ARROW | TV_BOLT | TV_SHOT) {
        if st.borg.tr(BI_AMMO_TVAL) != i32::from(item.tval) || item.cursed { return Vec::new(); }
        let damage = (i64::from(item.dd) * (i64::from(item.ds) + 1) * UNIT / 2
            + (i64::from(item.to_d) + i64::from(st.borg.tr(BI_BTODAM))) * UNIT).max(UNIT)
            * i64::from(st.borg.tr(BI_AMMO_POWER)).max(1);
        return vec![(ATTACK, damage)];
    }
    if tval == TV_LIGHT {
        return if item.timeout > 0 { vec![(LIGHT, i64::from(item.timeout) * UNIT)] } else { Vec::new() };
    }
    if tval == TV_DIGGING { return vec![(DIG, UNIT)]; }
    if engine::tval_is_book_k(item.kind as usize) {
        for b in 0..engine::class_num_books().min(9) {
            let book = engine::class_book(b);
            if book.tval != i32::from(item.tval) || book.sval != i32::from(item.sval) { continue; }
            let usable = (0..book.num_spells).map(|s| engine::class_book_spell(b, s))
                .filter(|s| s.slevel <= ctx.journey.level as i32 && s.smana <= st.borg.tr(BI_MAXSP)).count();
            if usable > 0 { return vec![(BOOKS + b as usize, UNIT)]; }
        }
        return Vec::new();
    }
    if !matches!(tval, TV_POTION | TV_SCROLL | TV_FOOD | TV_MUSHROOM | TV_WAND | TV_ROD | TV_STAFF) { return Vec::new(); }
    // Poisonous/hallucinatory nourishment must not be sold as safe food.
    if effects.iter().any(|&(e, sub, _)| matches!(u32::from(e), EF_TIMED_INC | EF_TIMED_INC_NO_RES) && matches!(sub as u32, TMD_POISONED | TMD_CONFUSED | TMD_IMAGE | TMD_PARALYZED)) {
        effects.retain(|&(e, _, _)| u32::from(e) != EF_NOURISH);
    }
    effects.retain(|&(e, sub, _)| {
        if (0..5).contains(&sub) {
            if u32::from(e) == EF_RESTORE_STAT { return st.borg.tr(BI_ISFIXSTR + sub as u32) > 0; }
            if u32::from(e) == EF_GAIN_STAT { return st.borg.tr(BI_CSTR + sub as u32) < 18 + 100; }
        }
        true
    });
    for (e, _, amount) in &mut effects {
        if u32::from(*e) == EF_RESTORE_MANA && *amount == 0 { *amount = i64::from(st.borg.tr(BI_MAXSP)); }
    }
    let mut out = effect_supply(&effects, ctx.journey.hp);
    let uses = if tval == TV_WAND || tval == TV_STAFF { i64::from(item.pval).max(0) * UNIT / qty }
        else if tval == TV_ROD {
            // MINIMISE/AVERAGE are pure; the rod's physical cooldown is not a level gate.
            let turns = engine::kind_recharge_average(item.kind as usize).max(1);
            (ctx.journey.turns / i64::from(turns)).max(1) * UNIT
        } else { UNIT };
    for (_, units) in &mut out { *units = *units * uses / UNIT; }
    out.retain(|&(_, units)| units > 0);
    if matches!(tval, TV_WAND | TV_STAFF) && item.pval > 0 {
        let maximum = engine::kind_value_average(engine::kind(item.kind as usize).charge).max(1);
        if i64::from(item.pval) < i64::from(maximum) * qty { out.push((RECHARGE_TARGET, UNIT)); }
    }
    out
}

/// Physical supply curves use the same forecast, gold opportunity cost and
/// capacity constraints as the core town model. No consumable calls borg_power.
pub fn observe(st: &BorgState, stock: &Stock, kit: &[KitItem], journey: Journey) -> Model {
    let t = |id| i64::from(st.borg.tr(id)).max(0);
    let cfg = |id| i64::from(st.cfg(id)).clamp(0, 1_000_000);
    let exposure = journey.exposure();
    let ordinary = cfg(BORG_TOWN_W_SUPPORT) * exposure;
    let risk = |id| cfg(id) * exposure;
    let support_demand = (journey.turns * UNIT / cfg(BORG_TOWN_SUPPORT_TURNS).max(1)).max(UNIT);
    let light_slot = crate::borg_item::inven_wield() as usize + 5;
    let light = st.items.items.get(light_slot).copied().unwrap_or_else(crate::borg_item::blank_item);
    let lantern = light.iqty > 0 && u32::from(light.tval) == TV_LIGHT && i32::from(light.sval) == st.item_val.sv_light_lantern;
    let ctx = Context { st, journey, lantern };
    let mut model = Model { curves: (0..COUNT).map(|i| [i as i64, ordinary, support_demand, 0, 0]).collect(), supplies: Vec::new() };
    let mut curve = |id: usize, weight: i64, demand: i64| { model.curves[id][1] = weight; model.curves[id][2] = demand; };
    curve(LIGHT, cfg(BORG_TOWN_W_LIGHT) * UNIT, journey.turns * UNIT);
    // Nutrition is the game's food-value unit (food_value=100), not an equal
    // copy whitelist. One ration's 40 nutrition supports food_turns actions.
    curve(FOOD, cfg(BORG_TOWN_W_FOOD) * journey.turns * UNIT / cfg(BORG_TOWN_FOOD_TURNS).max(1),
          journey.turns * 40 * UNIT / cfg(BORG_TOWN_FOOD_TURNS).max(1));
    curve(HEAL, risk(BORG_TOWN_W_BLEEDING), journey.hp * exposure);
    curve(WOUNDS, if t(BI_ISCUT) > 0 || t(BI_ISSTUN) > 0 { cfg(BORG_TOWN_W_BLEEDING) * UNIT } else { 0 }, -UNIT);
    curve(POISON, if t(BI_ISPOISONED) > 0 { cfg(BORG_TOWN_W_POISON) * UNIT } else { 0 }, -UNIT);
    curve(PHASE, risk(BORG_TOWN_W_ESCAPE), exposure);
    curve(RECALL, cfg(BORG_TOWN_W_TRIP) * UNIT * journey.walk / journey.turns.max(journey.walk).max(1), UNIT);
    let damage_scale = journey.level * 2 + 4;
    curve(ATTACK, risk(BORG_TOWN_W_ATTACK), damage_scale * journey.turns * UNIT / cfg(BORG_TOWN_ATTACK_TURNS).max(1));
    for id in [TELEPORT, LEVEL, OTHER] { curve(id, risk(BORG_TOWN_W_ESCAPE), exposure); }
    curve(MANA, if t(BI_MAXSP) > 0 { risk(BORG_TOWN_W_MAGIC) } else { 0 }, t(BI_MAXSP).max(1) * exposure);
    curve(IDENTIFY, if t(BI_ALL_NEED_ID) > 0 { cfg(BORG_TOWN_W_KNOWLEDGE) * UNIT } else { 0 }, -UNIT);
    curve(GAIN_EXPERIENCE, cfg(BORG_TOWN_W_RESTORE) * UNIT, -UNIT);
    curve(EXPERIENCE, if t(BI_ISFIXEXP) > 0 || t(BI_ISFIXLEV) > 0 { cfg(BORG_TOWN_W_RESTORE) * UNIT } else { 0 }, -UNIT);
    curve(CURSE, if t(BI_FIRST_CURSED) > 0 { cfg(BORG_TOWN_W_RESTORE) * UNIT } else { 0 }, -UNIT);
    curve(ENCHANT, if t(BI_NEED_ENCHANT_TO_A) + t(BI_NEED_ENCHANT_TO_H) + t(BI_NEED_ENCHANT_TO_D) > 0 { cfg(BORG_TOWN_W_SUPPORT) * UNIT } else { 0 }, UNIT);
    curve(RECHARGE, cfg(BORG_TOWN_W_SUPPORT) * UNIT, -UNIT);
    curve(RECHARGE_TARGET, 0, UNIT);
    for (cap, resistance) in [(FIRE, BI_RFIRE), (COLD, BI_RCOLD), (RESPOIS, BI_RPOIS), (ACID, BI_RACID), (ELEC, BI_RELEC)] {
        // Temporary resistance still stacks with permanent resistance in combat;
        // halve its priority rather than deleting its capability.
        curve(cap, ordinary / if t(resistance) > 0 { 2 } else { 1 }, journey.turns * UNIT);
    }
    for cap in [SPEED, BUFF] { curve(cap, ordinary, journey.turns * UNIT); }
    for (cap, condition) in [(CONFUSION, BI_ISCONFUSED), (BLIND, BI_ISBLIND), (FEAR, BI_ISAFRAID)] {
        curve(cap, if t(condition) > 0 { cfg(BORG_TOWN_W_RESTORE) * UNIT } else { ordinary }, if t(condition) > 0 { -UNIT } else { exposure });
    }
    for stat in 0..5 {
        let drained = t(BI_ISFIXSTR + stat as u32) > 0;
        curve(STATS + stat, if drained || t(BI_CSTR + stat as u32) < 18 + 100 { cfg(BORG_TOWN_W_RESTORE) * UNIT } else { 0 }, -UNIT);
    }
    for cap in BOOKS..BOOKS + 9 { curve(cap, cfg(BORG_TOWN_W_MAGIC) * UNIT, -UNIT); }
    // Per-item physical supply facts are independent of the quantities held.
    for c in &stock.candidates {
        let item = &st.store.shops[usize::from(c.shop)].ware[usize::from(c.slot)];
        for (cap, n) in supplies(item, &ctx) {
            model.supplies.push([i64::from(c.shop), i64::from(c.slot), cap as i64, n]);
        }
    }
    for it in kit {
        let item = if it.place == Place::Pack { &st.items.items[usize::from(it.slot)] }
            else { &st.store.shops[crate::borg_store::BORG_HOME].ware[usize::from(it.slot)] };
        for (cap, n) in supplies(item, &ctx) {
            model.supplies.push([Model::scope(it), i64::from(it.slot), cap as i64, n]);
            model.curves[cap][if it.place == Place::Pack { 3 } else { 4 }] += n * i64::from(it.qty);
        }
    }
    // Equipment/quiver supplies are not movable pack stacks, but do close
    // the same needs (especially current fuel and already-loaded arrows).
    for item in st.items.items.iter().skip(crate::borg_item::inven_wield() as usize).filter(|i| i.iqty > 0) {
        for (cap, n) in supplies(item, &ctx) { model.curves[cap][3] += n * i64::from(item.iqty); }
    }
    use crate::headers::list_object_flags_h::OF_NO_FUEL;
    if light.iqty > 0 && engine::flag_has(&light.flags, OF_NO_FUEL as i32) || t(BI_LIGHT) > 0 && light.iqty == 0 {
        model.curves[LIGHT][1] = 0;
    }
    // Reliable renewable spells cover a capability without pretending they
    // are 1000 physical potions. Books and mana must actually be available.
    for (si, spell) in st.magic.magics.iter().enumerate() {
        if spell.status < crate::borg_magic::BORG_MAGIC_TEST || spell.rating == 0
            || !crate::borg_magic::borg_spell_legal_fail(st, spell.spell_enum, 20) { continue; }
        let effects = engine::spell_effect_values(si as i32, journey.level as i32);
        // Only information/access services suppress duplicate purchases.
        // Mana-free attacks and healing remain valuable alongside spells.
        if effects.iter().any(|&(e, _, amount)| u32::from(e) == EF_TELEPORT && amount <= 10) { model.curves[PHASE][1] = 0; }
        for (cap, _) in effect_supply(&effects, journey.hp) {
            if matches!(cap, TRAPS | MONSTERS | INVISIBLE | MAP | ILLUMINATE | DIG | RECALL | FOOD) {
                model.curves[cap][1] = 0;
            }
        }
    }
    debug_assert!(model.valid());
    model
}

/// Optional engine-backed coverage diagnostic. These are hypothetical KNOWN
/// single items, never live inventory or receipts. It runs once per process,
/// after character birth, and mutates no player/Borg/game-RNG state.
pub fn audit(st: &BorgState, journey: Journey, model: &Model) {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::io::Write;
    static DONE: AtomicBool = AtomicBool::new(false);
    let Ok(path) = std::env::var("BORG_TOWN_CAPABILITY_AUDIT") else { return; };
    if DONE.swap(true, Ordering::Relaxed) { return; }
    let Ok(mut file) = std::fs::File::create(path) else { return; };
    let light_slot = crate::borg_item::inven_wield() as usize + 5;
    let light = st.items.items.get(light_slot).copied().unwrap_or_else(crate::borg_item::blank_item);
    let ctx = Context { st, journey, lantern: light.iqty > 0 && i32::from(light.sval) == st.item_val.sv_light_lantern };
    let _ = writeln!(file, "{{\"hypothetical_known_items\":1,\"class\":{},\"level\":{},\"curves\":{:?}}}", st.borg.tr(BI_CLASS), journey.level, model.curves);
    for kind in 1..usize::from(engine::z_info().k_max) {
        let k = engine::kind(kind);
        if k.name.is_null() { continue; }
        let mut item = crate::borg_item::blank_item();
        item.kind = kind as u32;
        item.tval = k.tval as u8; item.sval = k.sval as u8;
        item.iqty = 1; item.aware = true; item.flavour_aware = true;
        item.dd = k.dd as u8; item.ds = k.ds as u8;
        item.pval = engine::kind_value_average(k.pval) as i16;
        item.flags = k.flags;
        if matches!(k.tval as u32, TV_WAND | TV_STAFF) { item.pval = engine::kind_value_average(k.charge) as i16; }
        if k.tval as u32 == TV_LIGHT {
            item.timeout = if k.sval == st.item_val.sv_light_torch { engine::z_info().fuel_torch } else { engine::z_info().fuel_lamp } as i16;
        }
        let units = supplies(&item, &ctx);
        let units_json = units.iter().map(|(id, n)| format!("[{id},{n}]")).collect::<Vec<_>>().join(",");
        let _ = writeln!(file, "{{\"kind\":{},\"name\":{:?},\"tval\":{},\"sval\":{},\"supplies\":[{}],\"gain\":{},\"gear\":{}}}",
            kind, String::from_utf8_lossy(engine::kind_name(kind)), k.tval, k.sval, units_json,
            model.gain(&units), i32::from(crate::borg_inventory::borg_wield_slot(&item) >= 0));
    }
}

/// Merge intersecting capabilities, so substitute devices/consumables do
/// not each claim the same shortage. Preserve pure options plus a bounded
/// deterministic mixed path; the surrogate is not a global exact optimum.
pub fn shared_groups(groups: Vec<crate::borg_town_solve::Group>, input: &crate::borg_town_record::StepInput,
                     candidates: &[crate::borg_town_stock::Candidate], model: &Model, limits: &crate::borg_town_solve::Limits,
                     economics: (i64, i64, i64)) -> Vec<crate::borg_town_solve::Group> {
    let (space, carry, knowledge) = economics;
    use crate::borg_town_solve::{Choice, Group, GroupKey};
    let facts: BTreeMap<_, _> = input.facts.iter().map(|f| ((f.shop, f.slot), f)).collect();
    let mut out = Vec::new();
    let mut sets: Vec<(u64, Vec<Group>)> = Vec::new();
    for group in groups {
        if matches!(group.key, GroupKey::Slot(_)) { out.push(group); continue; }
        let mut mask = 0;
        for option in &group.options {
            for &(i, _) in &option.picks { let c = &candidates[i]; mask |= model.mask(c.shop, c.slot); }
        }
        if mask == 0 { out.push(group); continue; }
        let mut joined = vec![group];
        let mut again = true;
        while again {
            again = false;
            let mut i = 0;
            while i < sets.len() {
                if sets[i].0 & mask != 0 {
                    let (m, mut gs) = sets.remove(i); mask |= m; joined.append(&mut gs); again = true;
                } else { i += 1; }
            }
        }
        sets.push((mask, joined));
    }
    let value = |picks: Vec<(usize, i32)>| {
        let mut choice = Choice { picks, ..Choice::default() };
        let mut units = Vec::new();
        let mut slots = BTreeSet::new();
        let mut learned = BTreeSet::new();
        for &(i, count) in &choice.picks {
            let c = &candidates[i];
            choice.cost += i64::from(count) * i64::from(c.price);
            choice.weight += i64::from(count) * i64::from(c.weight);
            if c.slots_cost > 0 { slots.insert(c.class); }
            units.extend(model.units(i64::from(c.shop), i64::from(c.slot), i64::from(count)));
            if facts.get(&(c.shop, c.slot)).is_some_and(|f| f.serves & (1 << crate::borg_town_utility::index(crate::borg_town_usefulness::Need::Knowledge)) != 0) { learned.insert(c.kind); }
        }
        choice.slots = slots.len() as i32;
        choice.value = model.gain(&units) + learned.len() as i64 * knowledge * UNIT
            - crate::borg_town_utility::burden(space, choice.slots, choice.weight, limits.pack_slots, carry);
        choice
    };
    for (mask, groups) in sets {
        let mut options = vec![Choice::default()];
        for g in &groups { for option in g.options.iter().skip(1) { options.push(value(option.picks.clone())); } }
        let mut positions = vec![0; groups.len()];
        loop {
            let picks = |pos: &[usize]| groups.iter().zip(pos).flat_map(|(g, &o)| g.options[o].picks.clone()).collect();
            let current = value(picks(&positions));
            let mut best: Option<(usize, Choice)> = None;
            for (gi, g) in groups.iter().enumerate() {
                let oi = positions[gi] + 1;
                if oi >= g.options.len() { continue; }
                positions[gi] = oi;
                let next = value(picks(&positions));
                positions[gi] -= 1;
                let gain = next.value - current.value;
                let bm: i64 = next.picks.iter().filter(|&&(i, _)| candidates[i].is_black_market())
                    .map(|&(i, n)| i64::from(candidates[i].price) * i64::from(n)).sum();
                let caution = input.params[9].max(1);
                if gain <= 0 || next.cost > limits.gold || next.weight > limits.weight || next.slots > limits.pack_slots || bm.saturating_mul(caution) > limits.gold { continue; }
                let price = (next.cost - current.cost).max(1);
                if best.as_ref().is_none_or(|(_, b)| i128::from(gain) * i128::from((b.cost - current.cost).max(1)) > i128::from(b.value - current.value) * i128::from(price)) { best = Some((gi, next)); }
            }
            let Some((gi, next)) = best else { break; };
            positions[gi] += 1;
            options.push(next);
        }
        out.push(Group { key: GroupKey::Need(mask.trailing_zeros() as u8), options });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn model() -> Model {
        Model { curves: (0..COUNT).map(|i| [i as i64, 100 * UNIT, UNIT, 0, 0]).collect(), supplies: Vec::new() }
    }
    fn item(place: Place, slot: u8, count: i32) -> KitItem {
        KitItem { place, slot, kind: 1, qty: count, weight: 10, need: -1, reduction: 0,
            power: 0, proceeds: 0, sell_shop: 0, unknown: false, unknown_flavour: false, just_bought: false, serves: 0 }
    }
    #[test]
    fn wand_teleport_other_and_stone_to_mud_are_services_not_damage() {
        use crate::headers::list_projections_h::*;
        assert_eq!(effect_supply(&[(EF_BOLT as u16, PROJ_AWAY_ALL as i32, 100)], 20), [(OTHER, UNIT)]);
        assert_eq!(effect_supply(&[(EF_BOLT as u16, PROJ_KILL_WALL as i32, 30)], 20), [(DIG, UNIT)]);
        for projection in [PROJ_MON_HEAL, PROJ_MON_SPEED, PROJ_MON_CLONE, PROJ_MON_POLY] {
            assert!(effect_supply(&[(EF_BOLT as u16, projection as i32, 100)], 20).is_empty());
        }
    }
    #[test]
    fn secondary_cures_and_buffs_are_not_discarded_by_a_primary_healing_role() {
        let units = effect_supply(&[(EF_HEAL_HP as u16, 0, 60), (EF_CURE as u16, TMD_CONFUSED as i32, 0),
            (EF_CURE as u16, TMD_POISONED as i32, 0), (EF_CURE as u16, TMD_BLIND as i32, 0)], 25);
        assert!(units.contains(&(HEAL, 25 * UNIT)));
        for cap in [CONFUSION, POISON, BLIND] { assert!(units.contains(&(cap, UNIT))); }
        let units = effect_supply(&[(EF_TIMED_INC as u16, TMD_FAST as i32, 30),
            (EF_TIMED_INC as u16, TMD_OPP_FIRE as i32, 20)], 25);
        assert!(units.contains(&(SPEED, 30 * UNIT)) && units.contains(&(FIRE, 20 * UNIT)));
    }
    #[test]
    fn food_uses_actual_nutrition_and_crunch_is_only_a_message() {
        let biscuit = effect_supply(&[(EF_NOURISH as u16, 0, 10), (EF_CRUNCH as u16, 0, 0)], 20);
        assert_eq!(biscuit, [(FOOD, 10 * UNIT)]);
        assert_eq!(effect_supply(&[(EF_NOURISH as u16, 0, 40)], 20), [(FOOD, 40 * UNIT)]);
        assert!(effect_supply(&[(EF_NOURISH as u16, 1, 40)], 20).is_empty());
        assert!(effect_supply(&[(EF_NOURISH as u16, 2, 0)], 20).is_empty());
    }
    #[test]
    fn replenishment_competes_with_existing_and_substitute_stock() {
        let mut m = model();
        m.supplies = vec![[0, 0, ATTACK as i64, UNIT], [1, 0, ATTACK as i64, UNIT]];
        let first = m.ware(0, 0, 1);
        let two_kinds = m.gain(&[(ATTACK, UNIT), (ATTACK, UNIT)]);
        assert!(two_kinds < first * 2, "two kinds do not each claim the full shortage");
        m.curves[ATTACK][3] = 10 * UNIT;
        assert!(m.ware(0, 0, 1) < first);
        m.curves[ATTACK][2] = -UNIT; // one-time access/repair, such as a class book
        assert_eq!(m.ware(0, 0, 1), 0);
    }
    #[test]
    fn keeping_and_buying_use_identical_units_and_home_withdrawals_reprice() {
        let mut m = model();
        m.supplies = vec![[PACK, 0, ATTACK as i64, UNIT], [HOME, 0, ATTACK as i64, UNIT]];
        let pack = item(Place::Pack, 0, 1);
        let home = item(Place::Home, 0, 1);
        m.curves[ATTACK][3] = UNIT;
        let loss = m.keep(&pack);
        m.change(&pack, -1);
        assert_eq!(loss, m.take(&home));
        let first = m.take(&home);
        m.change(&home, 1);
        assert!(m.take(&home) < first);
        m.curves[ATTACK][4] = UNIT;
        assert!(m.reserve(&home, -1, 500) < 0);
    }
    #[test]
    fn malformed_replay_facts_cannot_index_outside_the_capability_table() {
        let mut m = model();
        assert!(m.valid());
        m.supplies.push([0, 0, COUNT as i64, UNIT]);
        assert!(!m.valid());
        m.supplies[0][2] = 0;
        m.supplies[0][3] = -UNIT;
        assert!(!m.valid());
    }
}
