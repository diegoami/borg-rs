//! Phase 4 of the town inventory redesign (borg-rs#36), slice 2: the plan
//! in **shadow mode** - computed at every town decision, recorded, never
//! executed.
//!
//! Enabled with `BORG_TOWN_PLAN_RECORD=<file>`. Unset, this module does
//! nothing at all. Set, it reads the state the current policy just decided
//! in (the same call that writes the v2 decision record,
//! `borg_town_dump::dump_town_visit`), runs the plan's step on it, and
//! writes one plan record (integer version 4, `borg_town_record`): v2's
//! fields for that state - today's decision included - then the plan's.
//! The two sides of a shadow comparison are therefore on one line.
//!
//! **Observation only.** The step reads the state and the stores and
//! writes the file; it presses no key, sets no goal, touches no setting
//! and draws no random number. The keypress stream with the record on is
//! the stream without it, and the parity suites stay valid.
//!
//! What the step can price today:
//!
//! * **light** - light sources, worn and spare (the need counts sources,
//!   decided 2026-11-13); a ware of the light tval closes one;
//! * **food** - what `borg_trait` counts in `BI_FOOD`, one per copy;
//! * **bleeding** - Cure Light, Serious or Critical Wounds, one per copy;
//! * **poison** - Neutralize Poison or a poison-curing food, one per copy;
//! * **escape** - Phase Door, one per copy (`BI_APHASE`).
//!
//! **Power** comes from the try-on ([`try_on`]): each wearable is put on
//! a copy-and-restore of the state and `borg_power` measures the gain,
//! which the power need's curve prices. Not yet: **knowledge** and **trip**,
//! which read 0 until the catalogue of useful kinds (#35 item 4) exists,
//! as phase 1 decided. A ware serves one need here; a food that also cures
//! poison is priced as food, and its poison urgent need is still in `serves`.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::borg_headers::borg_h::BORG_TOWN_PLAN;
use crate::borg_headers::borg_item_h::borg_item;
use crate::borg_headers::borg_trait_h::BI_CDEPTH;
use crate::borg_item::inven_wield;
use crate::borg_town_budget::Budget;
use crate::borg_store::BORG_HOME;
use crate::borg_town_dispose::{KitItem, Place};
use crate::borg_town_record::{first_action, step, PlanRecord, StepInput, TryOn, WareFact, PARAMS};
use crate::borg_town_solve::Limits;
use crate::borg_town_stock::{needs_served, Stock};
use crate::borg_town_usefulness::{Need, Trip};
use crate::engine;
use crate::state::BorgState;

/// The index of a need in [`Need::ALL`].
fn need_index(need: Need) -> i8 {
    Need::ALL.iter().position(|&n| n == need).map_or(-1, |i| i as i8)
}

/// The need a ware serves and what one copy closes of it - the reduction
/// table, as far as the needs that are priced today go (see the module
/// doc). `(-1, 0)` for a ware that serves none of them.
fn ware_role(st: &BorgState, item: &borg_item) -> (i8, i64) {
    use crate::headers::list_tvals_h::{TV_LIGHT, TV_SCROLL};
    let tval = u32::from(item.tval);
    if tval == TV_LIGHT {
        return (need_index(Need::Light), 1);
    }
    if tval == TV_SCROLL && i32::from(item.sval) == st.item_val.sv_scroll_phase_door {
        return (need_index(Need::Escape), 1);
    }
    // The urgent needs' classification already mirrors borg_trait's counting
    // of food, cures and poison cures; its first need is the ware's.
    match needs_served(st, item).first() {
        Some(&n) => (need_index(n), 1),
        None => (-1, 0),
    }
}

/// The try-on: for every wearable ware, the power gained by wearing it -
/// the ware put on in its equipment slot, the worn item moved into a free
/// pack slot, `borg_notice` and `borg_power` run, exactly as the live
/// strategy evaluates a purchase (`borg_think_shop_buy_useful`). Returns
/// the try-ons and, per wearable candidate, its slot cost: one pack slot
/// if something is worn there now (it moves to the pack), none if the
/// slot is empty.
///
/// **The state is restored by assignment, not recomputed**: every part a
/// try-on can write - `borg` (traits, `has`, activations, the stat-gain
/// arrays, `ready_morgoth`), `traits`, `trait_swap`, the items,
/// `prepared.buffer` and `think.do_spell` - is snapshotted first and put
/// back after, so the observation stays observation. Light sources are not
/// tried on (the light need prices them), nor diggers (the live strategy
/// keeps them as a back-up, not as a weapon). Without a free pack slot
/// there is nowhere to put the worn item, and nothing is tried on.
fn try_on(st: &mut BorgState, stock: &Stock) -> (Vec<TryOn>, Vec<(usize, i32)>) {
    use crate::headers::list_tvals_h::{TV_DIGGING, TV_LIGHT};
    let hole = crate::borg_inventory::borg_first_empty_inventory_slot(st);
    let Ok(hole) = usize::try_from(hole) else {
        return (Vec::new(), Vec::new());
    };
    let wield = inven_wield() as usize;
    let wearables: Vec<(usize, usize)> = stock
        .candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let w = &st.store.shops[usize::from(c.shop)].ware[usize::from(c.slot)];
            let tval = u32::from(w.tval);
            if tval == TV_LIGHT || tval == TV_DIGGING {
                return None;
            }
            let eq = crate::borg_inventory::borg_wield_slot(w);
            usize::try_from(eq).ok().map(|eq| (i, eq))
        })
        .collect();
    if wearables.is_empty() {
        return (Vec::new(), Vec::new());
    }

    // The snapshot of everything a try-on can write.
    let borg = st.borg.clone();
    let traits = st.traits.clone();
    let trait_swap = st.trait_swap.clone();
    let items = st.items.items.clone();
    let buffer = st.prepared.buffer.clone();
    let do_spell = st.think.do_spell;

    let base = i64::from(crate::borg_power::borg_power(st));
    let mut tryon = Vec::new();
    let mut slots = Vec::new();
    for (i, eq) in wearables {
        let c = &stock.candidates[i];
        let mut ware = st.store.shops[usize::from(c.shop)].ware[usize::from(c.slot)];
        ware.iqty = 1;
        let worn = items[eq];
        st.items.items[hole] = worn;
        st.items.items[eq] = ware;
        crate::borg_trait::borg_notice(st, true);
        let p = i64::from(crate::borg_power::borg_power(st));
        st.items.items[eq] = items[eq];
        st.items.items[hole] = items[hole];
        tryon.push(TryOn { shop: c.shop, slot: c.slot, eq_slot: (eq - wield) as u8, value: p - base });
        slots.push((i, i32::from(worn.iqty != 0)));
    }

    st.borg = borg;
    st.traits = traits;
    st.trait_swap = trait_swap;
    st.items.items = items;
    st.prepared.buffer = buffer;
    st.think.do_spell = do_spell;
    (tryon, slots)
}

/// The kit: the pack's and the Home's stacks, with what the dispositions
/// read (slice 3). The power of a pack stack is a **try-off** - its
/// quantity set to 0, `borg_notice` and `borg_power` run - and of a Home
/// stack a try-on into a free pack slot; both under the same snapshot and
/// restore-by-assignment as [`try_on`]. A Home stack with no free pack slot
/// to try it in gets no power. The proceeds are what a shop pays: 0 under
/// `birth_no_selling` (the game's default), else two thirds of the value
/// (the Borg's own estimate, `borg_price_item`, without the Black Market's
/// halving). The shop that buys it is the first that `borg_store_buys`
/// names.
fn kit_of(st: &mut BorgState) -> Vec<KitItem> {
    use crate::headers::list_options_h::OPT_birth_no_selling;
    use crate::headers::list_tvals_h::{TV_FLASK, TV_FOOD, TV_MUSHROOM, TV_POTION, TV_SCROLL};
    let no_selling = engine::player_opt(OPT_birth_no_selling);
    let pack_end = inven_wield() as usize;
    let fact = |st: &BorgState, it: &borg_item, place: Place, slot: usize, power: i64| -> KitItem {
        let (need, reduction) = ware_role(st, it);
        let tval = u32::from(it.tval);
        let flavoured = tval == TV_POTION || tval == TV_SCROLL;
        let consumable = flavoured || tval == TV_FOOD || tval == TV_MUSHROOM || tval == TV_FLASK;
        let bought = &st.buy;
        let just_bought = (0..usize::try_from(bought.bought_item_num).unwrap_or(0).min(10))
            .any(|i| bought.bought_item_tval[i] == i32::from(it.tval) && bought.bought_item_sval[i] == i32::from(it.sval));
        KitItem {
            place,
            slot: slot as u8,
            kind: it.kind,
            qty: i32::from(it.iqty),
            weight: i32::from(it.weight),
            need,
            reduction,
            power,
            proceeds: if no_selling { 0 } else { i64::from(it.value) * i64::from(it.iqty) * 2 / 3 },
            sell_shop: (0..7i8)
                .find(|&who| crate::borg_store_sell::borg_store_buys(it, i32::from(who)))
                .unwrap_or(-1),
            unknown: it.needs_ident && !consumable,
            unknown_flavour: !it.aware && flavoured,
            just_bought,
        }
    };

    // The snapshot of everything a try-off can write (as try_on).
    let borg = st.borg.clone();
    let traits = st.traits.clone();
    let trait_swap = st.trait_swap.clone();
    let items = st.items.items.clone();
    let buffer = st.prepared.buffer.clone();
    let do_spell = st.think.do_spell;
    let hole = usize::try_from(crate::borg_inventory::borg_first_empty_inventory_slot(st)).ok();
    let base = i64::from(crate::borg_power::borg_power(st));

    let mut kit = Vec::new();
    for (slot, &it) in items.iter().enumerate().take(pack_end) {
        if it.iqty == 0 {
            continue;
        }
        st.items.items[slot].iqty = 0;
        crate::borg_trait::borg_notice(st, true);
        let loss = base - i64::from(crate::borg_power::borg_power(st));
        st.items.items[slot] = it;
        kit.push(fact(st, &it, Place::Pack, slot, loss));
    }
    let home: Vec<borg_item> = st.store.shops.get(BORG_HOME).map_or(Vec::new(), |h| h.ware.clone());
    for (slot, it) in home.iter().enumerate() {
        if it.iqty == 0 {
            continue;
        }
        let gain = match hole {
            Some(h) => {
                st.items.items[h] = *it;
                crate::borg_trait::borg_notice(st, true);
                let p = i64::from(crate::borg_power::borg_power(st));
                st.items.items[h] = items[h];
                p - base
            }
            None => 0,
        };
        kit.push(fact(st, it, Place::Home, slot, gain));
    }

    st.borg = borg;
    st.traits = traits;
    st.trait_swap = trait_swap;
    st.items.items = items;
    st.prepared.buffer = buffer;
    st.think.do_spell = do_spell;
    kit
}

/// The record number over the run.
static SEQ: AtomicU64 = AtomicU64::new(0);
/// The town visit's number, and the level clock it was counted at.
static VISIT: AtomicU64 = AtomicU64::new(0);
static VISIT_CLOCK: AtomicU64 = AtomicU64::new(u64::MAX);
/// The step's number within the visit.
static STEP: AtomicU64 = AtomicU64::new(0);

/// Shadow the town decision just taken: compute the plan's step on the
/// same state and record it. Does nothing unless `BORG_TOWN_PLAN_RECORD`
/// is set, or outside the town.
pub fn shadow_visit(st: &mut BorgState) {
    let Some(file) = crate::borg_town_dump::plan_record_file() else {
        return;
    };
    if st.borg.tr(BI_CDEPTH) != 0 {
        return;
    }

    // A new visit when the level clock moved (new_level restarts it on
    // every arrival, the town included).
    let clock = st.borg.time.level as u64;
    if VISIT_CLOCK.swap(clock, Ordering::Relaxed) != clock {
        VISIT.fetch_add(1, Ordering::Relaxed);
        STEP.store(0, Ordering::Relaxed);
    }
    let visit = VISIT.load(Ordering::Relaxed);
    let step_no = STEP.fetch_add(1, Ordering::Relaxed);

    // A curing spell cures cuts and poison, and no trait counts it.
    let cure_spell = crate::borg_magic::borg_spell_legal_fail(
        st,
        crate::borg_headers::borg_magic_h::HERBAL_CURING,
        80,
    );
    let mut trip = Trip::of(st);
    trip.cure_spell = cure_spell;
    let mut stock = Stock::of(st);
    // The wearables: their power gain, and their slot cost (the worn item
    // moves to the pack).
    let (tryon, wear_slots) = try_on(st, &stock);
    let kit = kit_of(st);
    for &(i, cost) in &wear_slots {
        stock.candidates[i].slots_cost = cost;
    }
    let st: &BorgState = st;
    let budget = Budget::of(st, &stock, &trip);
    let mask = |ns: &[Need]| ns.iter().fold(0u8, |m, &n| m | (1 << need_index(n)));
    let wares: Vec<WareFact> = stock
        .candidates
        .iter()
        .map(|c| {
            let item = &st.store.shops[usize::from(c.shop)].ware[usize::from(c.slot)];
            let (need, reduction) = ware_role(st, item);
            WareFact {
                shop: c.shop,
                slot: c.slot,
                kind: c.kind,
                class: c.class,
                slots_cost: c.slots_cost,
                weight: c.weight,
                need,
                reduction,
                serves: mask(&needs_served(st, item)),
            }
        })
        .collect();
    let mut params = [0i64; PARAMS];
    for (i, p) in params.iter_mut().enumerate() {
        *p = i64::from(st.cfg(BORG_TOWN_PLAN + i as u32));
    }
    // The pack and the equipment, as v2 writes them: the light need
    // counts the light sources among them.
    let carried: Vec<(u8, i32)> = st
        .items
        .items
        .iter()
        .take(inven_wield() as usize + 12)
        .filter(|it| it.iqty != 0)
        .map(|it| (it.tval, i32::from(it.iqty)))
        .collect();
    let limits = Limits::of(&budget);
    let out = step(&StepInput {
        candidates: stock.candidates.clone(),
        facts: &wares,
        tryon: &tryon,
        limits,
        urgent: budget.urgent.clone(),
        params,
        traits: &st.borg.trait_,
        power: i64::from(st.borg.power),
        carried: &carried,
        cure_spell,
        kit: &kit,
        home_slots: budget.home_slots,
    });
    let (paction, act, for_need, now) = first_action(&stock.candidates, &budget.urgent, &out, &wares, &kit);

    let rec = PlanRecord {
        mode: 0,
        visit,
        step: step_no,
        gturn: i64::from(engine::game_turn()),
        power: i64::from(st.borg.power),
        spells: u8::from(cure_spell),
        params,
        limits: [budget.gold, i64::from(budget.pack_slots), i64::from(budget.home_slots), budget.weight],
        caps: [
            i64::from(engine::z_info().pack_size),
            i64::from(engine::z_info().store_inven_max),
        ],
        urgent: budget
            .urgent
            .iter()
            .map(|f| {
                let c = &stock.candidates[f.candidate];
                (c.shop, c.slot, mask(&f.needs))
            })
            .collect(),
        unmet: mask(&budget.unmet),
        wares,
        tryon,
        dispose: out.dispositions.iter().map(|&d| crate::borg_town_record::dispose_code(d)).collect(),
        kit,
        plan: out.plan.clone(),
        solve: out.solve,
        action: paction,
        act,
        for_need,
        now,
        displaced: out.displaced,
        guard: 0,
    };

    // v2's fields for this state (today's decision among them), with the
    // version tag 4, then the plan's.
    let mut line = String::with_capacity(4096);
    crate::borg_town_dump::push_integer_record_at(
        &mut line,
        st,
        SEQ.fetch_add(1, Ordering::Relaxed),
        crate::borg_town_record::PLAN_SCHEMA,
        i64::from(st.borg.time.now),
        true,
    );
    if line.ends_with('}') {
        line.pop();
    }
    rec.push_fields(&mut line);
    line.push('}');
    crate::borg_town_dump::write_record(Some(file), &line);
}
