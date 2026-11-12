//! Phase 1 of the town inventory redesign (borg-rs#36): the usefulness
//! function, the swept judgement of the plan.
//!
//! One town visit produces one plan: what to keep, store, sell and drop,
//! and what to buy, under one gold budget. Every candidate purchase is a
//! (need, reduction) pair priced by [`usefulness`]: the deficit of a
//! [`Need`] in the game's own units, weighted and discounted by the
//! flat-then-capped value curve decided on 2026-11-12 (B1):
//!
//! ```text
//! U_K(n) = w_K * min(1, n*r / g),    g = target - carried
//! ```
//!
//! The marginal value of a copy is the constant `w*r/g` until the deficit
//! closes, and zero at or above the target: a met need prices nothing.
//! (In integers the floor division lets a marginal exceed its predecessor
//! by at most one — rounding noise, not shape.) The curve is weakly
//! concave, so the group knapsack DP of phase 3 is exact (P15).
//!
//! Nothing calls this module yet: the plan itself is phase 3, and the
//! reduction table that maps a ware to its (need, reduction) pairs is
//! phase 2. Every public item therefore carries
//! `#[cfg_attr(not(test), expect(dead_code))]`: a marker that removes
//! itself when the first production caller lands.

use crate::borg_headers::borg_h::{
    BORG_TOWN_FOOD_TRIP, BORG_TOWN_LIGHT_DEPTH, BORG_TOWN_SHARE, BORG_TOWN_W_BLEEDING,
    BORG_TOWN_W_ESCAPE, BORG_TOWN_W_FOOD, BORG_TOWN_W_KNOWLEDGE, BORG_TOWN_W_LIGHT,
    BORG_TOWN_W_POISON, BORG_TOWN_W_POWER, BORG_TOWN_W_TRIP,
};
use crate::borg_headers::borg_trait_h::{
    BI_ACCW, BI_ACLW, BI_ACSW, BI_ACUREPOIS, BI_APHASE, BI_FOOD, BI_ISCUT, BI_ISPOISONED,
    BI_LIGHT, BI_MAXDEPTH,
};
use crate::state::BorgState;

/// The eight needs a purchase can serve, as the epic log decided them on
/// 2026-11-12. The first four are the floors: kit predicates the pack
/// must satisfy. An unmet floor is not a purchase candidate; it feeds
/// #37's Stay rules, and the character may leave town with the floor
/// unmet.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Need {
    /// Light sources carried; the descent target decides how many.
    Light,
    /// Food rations carried.
    Food,
    /// A cure for the bleeding the character suffers now.
    Bleeding,
    /// The capability to cure poison, carried whether or not the
    /// character is poisoned now.
    Poison,
    /// Phase Door scrolls, the per-level escape.
    Escape,
    /// Identify and friends. The catalogue (#35 item 4) prices these in
    /// phase 2; until then the deficit reads 0.
    Knowledge,
    /// Borg power against the trip's depth target.
    Power,
    /// Word of Recall and the trip itself. Priced from the phase-2 stock
    /// table; until then the deficit reads 0.
    Trip,
}

impl Need {
    /// All needs, in enum order.
    #[cfg_attr(not(test), expect(dead_code))]
    pub const ALL: [Need; 8] = [
        Need::Light,
        Need::Food,
        Need::Bleeding,
        Need::Poison,
        Need::Escape,
        Need::Knowledge,
        Need::Power,
        Need::Trip,
    ];
}

/// The floors are Light, Food, Bleeding and Poison.
#[cfg_attr(not(test), expect(dead_code))]
pub fn is_floor(need: Need) -> bool {
    matches!(need, Need::Light | Need::Food | Need::Bleeding | Need::Poison)
}

/// The floors the current kit fails, in enum order. Light and Food are
/// "at least one carried"; Bleeding is "cut, and no cure of any strength
/// carried"; Poison is "no poison-cure capability carried", poisoned or
/// not.
#[cfg_attr(not(test), expect(dead_code))]
pub fn unmet_floors(st: &BorgState) -> Vec<Need> {
    let mut out = Vec::new();
    if st.borg.tr(BI_LIGHT) < 1 {
        out.push(Need::Light);
    }
    if st.borg.tr(BI_FOOD) < 1 {
        out.push(Need::Food);
    }
    if st.borg.tr(BI_ISCUT) != 0
        && st.borg.tr(BI_ACLW) + st.borg.tr(BI_ACSW) + st.borg.tr(BI_ACCW) < 1
    {
        out.push(Need::Bleeding);
    }
    if st.borg.tr(BI_ACUREPOIS) < 1 {
        out.push(Need::Poison);
    }
    out
}

/// The trip model, stated once (decided 2026-11-12): the descent target
/// is five levels past the deepest the borg has been, each level is
/// expected to take a thousand turns (anchored on the 2000-turn give-up),
/// and the power target is two thousand per level of depth — the weakest
/// of the provisional values, to be revisited when the zoo sweeps.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug)]
pub struct Trip {
    /// Where the next trip aims: `BI_MAXDEPTH + LEVELS_PER_TRIP`.
    pub depth_target: i32,
    /// Levels per trip: 5.
    pub levels_per_trip: i32,
    /// Expected turns per level: 1000.
    pub turns_per_level: i32,
    /// `borg_town_light_depth`: the descent target from which two light
    /// sources are required.
    pub light_depth: i32,
    /// `borg_town_food_trip`: the food rations carried per trip.
    pub food_trip: i32,
}

impl Trip {
    /// Levels per trip, and the depth gain the descent target assumes.
    pub const LEVELS_PER_TRIP: i32 = 5;
    /// Expected turns per level, anchored on the 2000-turn give-up.
    pub const TURNS_PER_LEVEL: i32 = 1000;
    /// The provisional power target per level of depth.
    pub const POWER_PER_DEPTH: i32 = 2000;

    /// The trip the current state plans for.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn of(st: &BorgState) -> Self {
        Trip {
            depth_target: st.borg.tr(BI_MAXDEPTH) + Self::LEVELS_PER_TRIP,
            levels_per_trip: Self::LEVELS_PER_TRIP,
            turns_per_level: Self::TURNS_PER_LEVEL,
            light_depth: st.cfg(BORG_TOWN_LIGHT_DEPTH),
            food_trip: st.cfg(BORG_TOWN_FOOD_TRIP),
        }
    }

    /// One light source, two once the descent target reaches
    /// `light_depth`.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn required_light(&self) -> i32 {
        if self.depth_target < self.light_depth {
            1
        } else {
            2
        }
    }

    /// The food rations carried per trip.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn required_food(&self) -> i32 {
        self.food_trip
    }

    /// One escape per ten levels of trip, rounded up.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn escapes_needed(&self) -> i32 {
        (self.levels_per_trip + 9) / 10
    }

    /// `POWER_PER_DEPTH * depth_target`.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn power_target(&self) -> i32 {
        Self::POWER_PER_DEPTH * self.depth_target
    }
}

/// The free parameters the zoo sweeps in phase 4: the eight need weights
/// (`w_light = 100` is the numeraire, so seven are free) and the caution
/// `k`, the purse-share cap of a single purchase.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug)]
pub struct Weights {
    /// `borg_town_w_light` (numeraire, 100).
    pub light: i64,
    /// `borg_town_w_food` (100).
    pub food: i64,
    /// `borg_town_w_bleeding` (100).
    pub bleeding: i64,
    /// `borg_town_w_poison` (100).
    pub poison: i64,
    /// `borg_town_w_escape` (50).
    pub escape: i64,
    /// `borg_town_w_knowledge` (25).
    pub knowledge: i64,
    /// `borg_town_w_power` (50).
    pub power: i64,
    /// `borg_town_w_trip` (50).
    pub trip: i64,
    /// `borg_town_share`: a single purchase may cost at most purse / k.
    pub caution: i64,
}

impl Weights {
    /// The weights as borg.txt gives them.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn of(cfg: &[i32]) -> Self {
        Weights {
            light: i64::from(cfg[BORG_TOWN_W_LIGHT as usize]),
            food: i64::from(cfg[BORG_TOWN_W_FOOD as usize]),
            bleeding: i64::from(cfg[BORG_TOWN_W_BLEEDING as usize]),
            poison: i64::from(cfg[BORG_TOWN_W_POISON as usize]),
            escape: i64::from(cfg[BORG_TOWN_W_ESCAPE as usize]),
            knowledge: i64::from(cfg[BORG_TOWN_W_KNOWLEDGE as usize]),
            power: i64::from(cfg[BORG_TOWN_W_POWER as usize]),
            trip: i64::from(cfg[BORG_TOWN_W_TRIP as usize]),
            caution: i64::from(cfg[BORG_TOWN_SHARE as usize]),
        }
    }

    /// The weight of a need.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn get(&self, need: Need) -> i64 {
        match need {
            Need::Light => self.light,
            Need::Food => self.food,
            Need::Bleeding => self.bleeding,
            Need::Poison => self.poison,
            Need::Escape => self.escape,
            Need::Knowledge => self.knowledge,
            Need::Power => self.power,
            Need::Trip => self.trip,
        }
    }

    /// The caution cap: `cost <= purse / k`, so `cost * k <= purse`.
    /// `k = 0` disables the cap.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn caution_allows(&self, cost: i64, purse: i64) -> bool {
        cost.saturating_mul(self.caution) <= purse
    }
}

/// The value curve (B1): `w * min(1, n*r / g)` in integers,
/// `w * min(n*r, g) / g`. Zero weight, reduction, gap or count prices
/// nothing; at or above the target the gap is closed and nothing is
/// priced.
#[cfg_attr(not(test), expect(dead_code))]
pub fn value(weight: i64, reduction: i64, gap: i64, copies: i64) -> i64 {
    if weight <= 0 || reduction <= 0 || gap <= 0 || copies <= 0 {
        return 0;
    }
    let closed = reduction.saturating_mul(copies).min(gap);
    weight.saturating_mul(closed) / gap
}

/// The marginal value of copy `copy` (1-based): constant `w*r/g` while
/// the deficit is open, zero once it closes.
#[cfg_attr(not(test), expect(dead_code))]
pub fn marginal(weight: i64, reduction: i64, gap: i64, copy: i64) -> i64 {
    value(weight, reduction, gap, copy) - value(weight, reduction, gap, copy - 1)
}

/// The deficit of a need in the game's own units: how far the carried
/// kit is below the target. Never negative; a met need has no deficit and
/// prices nothing. Bleeding and Poison are the provisional booleans
/// decided on 2026-11-12: 1 while cut; 1 while poisoned, plus 1 while no
/// poison-cure capability is carried. Knowledge reads 0 until the
/// catalogue (#35 item 4) prices it in phase 2, Trip reads 0 until the
/// phase-2 stock table prices the recall.
#[cfg_attr(not(test), expect(dead_code))]
pub fn deficit(need: Need, st: &BorgState, trip: &Trip) -> i64 {
    let d = match need {
        Need::Light => trip.required_light() - st.borg.tr(BI_LIGHT),
        Need::Food => trip.required_food() - st.borg.tr(BI_FOOD),
        Need::Bleeding => (st.borg.tr(BI_ISCUT) != 0) as i32,
        Need::Poison => {
            (st.borg.tr(BI_ISPOISONED) != 0) as i32 + (st.borg.tr(BI_ACUREPOIS) == 0) as i32
        }
        Need::Escape => trip.escapes_needed() - st.borg.tr(BI_APHASE),
        Need::Knowledge => 0,
        Need::Power => trip.power_target() - st.borg.power,
        Need::Trip => 0,
    };
    i64::from(d).max(0)
}

/// The usefulness of `copies` of a purchase that reduces `need`'s
/// deficit by `reduction` each: `value(w, reduction, deficit, copies)`.
#[cfg_attr(not(test), expect(dead_code))]
pub fn usefulness(
    need: Need,
    reduction: i64,
    copies: i64,
    st: &BorgState,
    trip: &Trip,
    weights: &Weights,
) -> i64 {
    value(weights.get(need), reduction, deficit(need, st, trip), copies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borg_headers::borg_trait_h::BI_MAX;
    use crate::borg_init::BORG_SETTINGS;

    /// The borg.txt defaults as the parser reads them.
    fn cfg() -> Vec<i32> {
        BORG_SETTINGS.iter().map(|s| s.default_value).collect()
    }

    /// An empty borg with the default settings and a zeroed trait array.
    fn state() -> BorgState {
        let mut st = BorgState::default();
        st.core.cfg = cfg();
        st.borg.trait_ = vec![0; BI_MAX as usize];
        st
    }

    /// w=100, r=2, g=10: the value rises by the constant w*r/g = 20 per
    /// copy until the deficit closes at five copies, then stays at w.
    #[test]
    fn curve_is_flat_then_capped() {
        let got: Vec<i64> = (0..=6).map(|n| value(100, 2, 10, n)).collect();
        assert_eq!(got, [0, 20, 40, 60, 80, 100, 100]);
    }

    /// At or above the target the gap is closed and nothing is priced; a
    /// zero weight, reduction or count prices nothing either.
    #[test]
    fn curve_prices_nothing_at_or_above_target() {
        assert_eq!(value(100, 2, 0, 3), 0);
        assert_eq!(value(100, 2, -3, 3), 0);
        assert_eq!(value(100, 2, 10, 0), 0);
        assert_eq!(value(0, 2, 10, 3), 0);
        assert_eq!(value(100, 0, 10, 3), 0);
    }

    /// The curve is nondecreasing, capped by the weight, and weakly
    /// concave up to the integer rounding of the floor division: below
    /// the cap a marginal is `floor(w*r/g)` or one more, so a marginal
    /// can exceed its predecessor by at most one; once the deficit is
    /// closed the marginal is exactly zero and the total exactly w.
    #[test]
    fn curve_is_weakly_concave_up_to_rounding_and_capped() {
        let cases: [(i64, i64, i64); 4] = [(100, 2, 10), (50, 1, 3), (25, 7, 4), (100, 1, 1)];
        for (w, r, g) in cases {
            let mut prev = i64::MAX;
            let cap = g.div_euclid(r) + i64::from(g % r != 0); // the first n with n*r >= g
            for n in 1..=12 {
                let v = value(w, r, g, n);
                assert!(v <= w, "value above weight at n={n}");
                assert!(v >= value(w, r, g, n - 1), "value fell at n={n}");
                let m = marginal(w, r, g, n);
                if n > 1 {
                    assert!(m <= prev + 1, "marginal rose by more than rounding at n={n} for w={w} r={r} g={g}");
                }
                if n > cap {
                    assert_eq!(m, 0, "marginal past the cap at n={n}");
                }
                if n >= cap {
                    assert_eq!(v, w, "below the weight at the cap, n={n}");
                }
                prev = m;
            }
        }
    }

    /// The trip model: depth target = maxdepth + 5, a thousand turns per
    /// level, light keyed to the descent target, food the per-trip
    /// ration, one escape per ten levels, power = 2000 x depth.
    #[test]
    fn trip_model_reads_state_and_settings() {
        let mut st = state();
        st.borg.trait_[BI_MAXDEPTH as usize] = 10;
        let trip = Trip::of(&st);
        assert_eq!(trip.depth_target, 15);
        assert_eq!(trip.levels_per_trip, 5);
        assert_eq!(trip.turns_per_level, 1000);
        assert_eq!(trip.light_depth, 15);
        assert_eq!(trip.food_trip, 10);
        // at the boundary the second light is already required
        assert_eq!(trip.required_light(), 2);
        assert_eq!(trip.required_food(), 10);
        assert_eq!(trip.escapes_needed(), 1);
        assert_eq!(trip.power_target(), 30000);
        // a shallower target needs one light
        st.borg.trait_[BI_MAXDEPTH as usize] = 4;
        assert_eq!(Trip::of(&st).required_light(), 1);
    }

    #[test]
    fn weights_read_the_settings() {
        let w = Weights::of(&cfg());
        assert_eq!(Need::ALL.len(), 8);
        assert_eq!(w.get(Need::Light), 100);
        assert_eq!(w.get(Need::Food), 100);
        assert_eq!(w.get(Need::Bleeding), 100);
        assert_eq!(w.get(Need::Poison), 100);
        assert_eq!(w.get(Need::Escape), 50);
        assert_eq!(w.get(Need::Knowledge), 25);
        assert_eq!(w.get(Need::Power), 50);
        assert_eq!(w.get(Need::Trip), 50);
        assert_eq!(w.caution, 10);
    }

    /// k=10: a purchase may cost at most a tenth of the purse; k=0
    /// disables the cap.
    #[test]
    fn caution_is_a_share_cap() {
        let w = Weights::of(&cfg());
        assert!(w.caution_allows(600, 6000));
        assert!(!w.caution_allows(601, 6000));
        let mut uncapped = w;
        uncapped.caution = 0;
        assert!(uncapped.caution_allows(999_999, 1));
    }

    #[test]
    fn floors_are_kit_predicates() {
        // the floors are exactly Light, Food, Bleeding and Poison
        for need in Need::ALL {
            assert_eq!(
                is_floor(need),
                matches!(need, Need::Light | Need::Food | Need::Bleeding | Need::Poison),
            );
        }
        // the empty kit fails light, food and the poison capability
        let st = state();
        assert_eq!(unmet_floors(&st), [Need::Light, Need::Food, Need::Poison]);
        // a cut with no cure of any strength carried adds Bleeding
        let mut st = state();
        st.borg.trait_[BI_LIGHT as usize] = 1;
        st.borg.trait_[BI_FOOD as usize] = 1;
        st.borg.trait_[BI_ACUREPOIS as usize] = 1;
        st.borg.trait_[BI_ISCUT as usize] = 1;
        assert_eq!(unmet_floors(&st), [Need::Bleeding]);
        // any cure strength discharges the bleeding floor
        st.borg.trait_[BI_ACSW as usize] = 1;
        assert_eq!(unmet_floors(&st), []);
        // no cut, no floor
        st.borg.trait_[BI_ACSW as usize] = 0;
        st.borg.trait_[BI_ISCUT as usize] = 0;
        assert_eq!(unmet_floors(&st), []);
    }

    /// maxdepth 10, so the depth target is 15: one more light (gap 1,
    /// worth 100), six more rations (gap 6: 16 for the first, 100 at six
    /// and no more after), escapes already met (gap 0), and power 10000
    /// short of the 30000 target: a +1500 item is worth 50*1500/10000 = 7.
    #[test]
    fn usefulness_values_for_fixed_states() {
        let mut st = state();
        st.borg.trait_[BI_MAXDEPTH as usize] = 10;
        st.borg.trait_[BI_LIGHT as usize] = 1;
        st.borg.trait_[BI_FOOD as usize] = 4;
        st.borg.trait_[BI_APHASE as usize] = 1;
        st.borg.power = 20000;
        let trip = Trip::of(&st);
        let w = Weights::of(&cfg());
        assert_eq!(usefulness(Need::Light, 1, 1, &st, &trip, &w), 100);
        assert_eq!(usefulness(Need::Food, 1, 1, &st, &trip, &w), 16);
        assert_eq!(usefulness(Need::Food, 1, 6, &st, &trip, &w), 100);
        assert_eq!(usefulness(Need::Food, 1, 7, &st, &trip, &w), 100);
        assert_eq!(usefulness(Need::Escape, 1, 1, &st, &trip, &w), 0);
        assert_eq!(usefulness(Need::Power, 1500, 1, &st, &trip, &w), 7);
        // Knowledge and Trip read 0 until phase 2 prices them
        assert_eq!(usefulness(Need::Knowledge, 1, 1, &st, &trip, &w), 0);
        assert_eq!(usefulness(Need::Trip, 1, 1, &st, &trip, &w), 0);
    }

    /// The provisional booleans: a cut is a bleeding deficit of 1 (the
    /// first cure is worth the full 100); poisoned with no cure carried
    /// is a poison deficit of 2 (one cure 50, two 100).
    #[test]
    fn usefulness_values_for_condition_states() {
        let mut st = state();
        let trip = Trip::of(&st);
        let w = Weights::of(&cfg());
        // a cut: gap 1, the first cure is worth the full weight
        st.borg.trait_[BI_ISCUT as usize] = 1;
        assert_eq!(usefulness(Need::Bleeding, 1, 1, &st, &trip, &w), 100);
        // not cut: no bleeding deficit
        st.borg.trait_[BI_ISCUT as usize] = 0;
        assert_eq!(usefulness(Need::Bleeding, 1, 1, &st, &trip, &w), 0);
        // poisoned, no capability: gap 2
        st.borg.trait_[BI_ISPOISONED as usize] = 1;
        assert_eq!(usefulness(Need::Poison, 1, 1, &st, &trip, &w), 50);
        assert_eq!(usefulness(Need::Poison, 1, 2, &st, &trip, &w), 100);
        // the capability alone halves it, curing the condition closes it
        st.borg.trait_[BI_ACUREPOIS as usize] = 1;
        assert_eq!(usefulness(Need::Poison, 1, 1, &st, &trip, &w), 100);
        st.borg.trait_[BI_ISPOISONED as usize] = 0;
        assert_eq!(usefulness(Need::Poison, 1, 1, &st, &trip, &w), 0);
    }
}
