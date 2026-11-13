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
//! Not yet: **power** (the wearables' try-on runs the engine's power code
//! on a mutated, restored state - its own slice, so the `tryon` field is
//! empty and no wearable is planned), and **knowledge** and **trip**,
//! which read 0 until the catalogue of useful kinds (#35 item 4) exists,
//! as phase 1 decided. A ware serves one need here; a food that also cures
//! poison is priced as food, and its poison floor is still in `serves`.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::borg_headers::borg_h::BORG_TOWN_PLAN;
use crate::borg_headers::borg_item_h::borg_item;
use crate::borg_headers::borg_trait_h::BI_CDEPTH;
use crate::borg_item::inven_wield;
use crate::borg_town_budget::Budget;
use crate::borg_town_record::{first_action, step, PlanRecord, StepInput, WareFact, PARAMS};
use crate::borg_town_solve::Limits;
use crate::borg_town_stock::{floors_served, Stock};
use crate::borg_town_usefulness::Need;
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
    // The floors' classification already mirrors borg_trait's counting
    // of food, cures and poison cures; its first need is the ware's.
    match floors_served(st, item).first() {
        Some(&n) => (need_index(n), 1),
        None => (-1, 0),
    }
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
pub fn shadow_visit(st: &BorgState) {
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

    let stock = Stock::of(st);
    let budget = Budget::of(st, &stock);
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
                serves: mask(&floors_served(st, item)),
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
        tryon: &[],
        limits,
        floors: budget.floors.clone(),
        params,
        traits: &st.borg.trait_,
        power: i64::from(st.borg.power),
        carried: &carried,
    });
    let (paction, act, for_need, floor) = first_action(&stock.candidates, &budget.floors, &out, &wares);

    let rec = PlanRecord {
        mode: 0,
        visit,
        step: step_no,
        gturn: i64::from(engine::game_turn()),
        power: i64::from(st.borg.power),
        params,
        limits: [budget.gold, i64::from(budget.pack_slots), i64::from(budget.home_slots), budget.weight],
        caps: [
            i64::from(engine::z_info().pack_size),
            i64::from(engine::z_info().store_inven_max),
        ],
        floors: budget
            .floors
            .iter()
            .map(|f| {
                let c = &stock.candidates[f.candidate];
                (c.shop, c.slot, mask(&f.needs))
            })
            .collect(),
        unmet: mask(&budget.unmet),
        wares,
        tryon: Vec::new(),
        plan: out.plan.clone(),
        solve: out.solve,
        action: paction,
        act,
        for_need,
        floor,
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
