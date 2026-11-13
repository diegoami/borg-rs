//! Phase 3 of the town inventory redesign (borg-rs#36): the solver - what
//! to buy.
//!
//! The form is the one the B2 write-up (`docs/TOWN-INVENTORY-B2.md`,
//! accepted 2026-11-13) derives:
//!
//! * the **groups**: one per stack class for the wares that stack (food,
//!   potions, scrolls - class = kind there), whose options are 0..n copies
//!   bought cheapest-first; and, for the wares a character wears, **one
//!   multiple-choice group per equipment slot** (Q1), whose options are
//!   "none" or exactly one of the wares that fit it;
//! * the **costs**: cumulative cheapest-n within a class, so the Black
//!   Market's 3x binds last; the caution (`cost <= purse / k`) is applied
//!   **per group** (Q2), to the Black Market copies of the whole option,
//!   and truncates the table there;
//! * the **values**: each group priced on its own, against the deficit as
//!   the carried kit stands - exact for that **separable surrogate** (Q3);
//!   the re-solve after every action prices what is left;
//! * the **DP**: the exact group knapsack over the gold, with the gold
//!   bound `min(gold, the sum of every group's dearest option)`;
//! * the **pack slots**: the Lagrangian sweep, a penalty `lambda` per slot
//!   an option opens, binary-searched to the smallest `lambda` whose
//!   solution fits (P15's recommendation; usually exact or within one
//!   option, and not guaranteed exact - the solution says so);
//! * the **weight**: a repair after the sweep - downgrade the option whose
//!   loss per weight freed is least, until the carry allowance holds;
//! * the **ties**, deterministic: more usefulness, then less gold, then
//!   fewer slots, then the earlier group's choice kept.
//!
//! The floors are not here: the budget (phase 2) bought them first, and
//! [`groups`] removes their copies from the wares before it builds the
//! tables. Nothing in production calls this module yet; every public item
//! carries the self-removing `expect(dead_code)` marker of phases 1-2.

use std::collections::BTreeMap;

use crate::borg_town_budget::Budget;
use crate::borg_town_stock::{Candidate, Stock};

/// The largest gold bound the DP runs on as is. Above it the costs are
/// scaled down (each rounded *up*, so a solution never overspends) and
/// the solution is marked inexact.
#[cfg_attr(not(test), expect(dead_code))]
pub const GOLD_LIMIT: i64 = 1 << 16;

/// What a group is: one stack class, or one equipment slot.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GroupKey {
    /// The wares of one stack class; option `n` is `n` copies.
    Class(u16),
    /// The wares that fit one equipment slot; at most one is bought.
    Slot(u8),
}

/// One option of a group: what it buys and what it costs.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Choice {
    /// (candidate index, copies) bought by this option.
    pub picks: Vec<(usize, i32)>,
    pub cost: i64,
    /// The usefulness this option adds (the surrogate's value).
    pub value: i64,
    /// The pack slots it opens.
    pub slots: i32,
    pub weight: i64,
}

/// A group: its key and its options, option 0 always "buy nothing".
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub key: GroupKey,
    pub options: Vec<Choice>,
}

/// The constraints the solver respects (the budget after the floors).
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub gold: i64,
    pub pack_slots: i32,
    pub weight: i64,
}

impl Limits {
    /// The limits a budget leaves.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn of(budget: &Budget) -> Self {
        Limits { gold: budget.gold, pack_slots: budget.pack_slots, weight: budget.weight }
    }
}

/// The chosen set.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Solution {
    /// The option chosen in each group, by group index (0 = nothing).
    pub chosen: Vec<usize>,
    pub value: i64,
    pub cost: i64,
    pub slots: i32,
    pub weight: i64,
    /// The slot penalty the sweep settled on (0 when slots never bound).
    pub lambda: i64,
    /// False when the gold had to be scaled (bound above [`GOLD_LIMIT`]).
    pub gold_exact: bool,
    /// False when the slot limit could not be met (it was negative: the
    /// dispositions must free room first) - nothing slot-opening is
    /// bought then.
    pub slots_met: bool,
    /// False when the weight repair had to drop or reduce an option.
    pub weight_repaired: bool,
}

impl Solution {
    /// The chosen options as (group, option), ranked: the most useful
    /// first, then the cheaper, then the earlier group. The plan (phase 4)
    /// executes the first and re-solves.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn ranked(&self, groups: &[Group]) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = self
            .chosen
            .iter()
            .enumerate()
            .filter(|&(_, &o)| o != 0)
            .map(|(g, &o)| (g, o))
            .collect();
        out.sort_by_key(|&(g, o)| {
            let c = &groups[g].options[o];
            (std::cmp::Reverse(c.value), c.cost, g)
        });
        out
    }
}

/// A DP cell's score: the penalised value, then less gold, then fewer
/// slots, packed into one integer so that a cell update is one add and
/// one compare. The packing is exact: the cost and the slots of any
/// selection fit below their fields (a cost below 2^47, at most 2^16
/// slots), so the lexicographic order and its addition carry over to the
/// integer. Lexicographic order is translation-invariant, so the group DP
/// stays exact for it.
type Score = i128;

/// The score of one option.
fn score(value: i64, cost: i64, slots: i32) -> Score {
    (i128::from(value) << 64) - (i128::from(cost) << 16) - i128::from(slots)
}

/// The exact group DP at slot penalty `lambda`, on costs divided by
/// `scale` (rounded up) under the scaled bound. Returns the option chosen
/// per group.
fn dp(groups: &[Group], bound: i64, scale: i64, lambda: i64) -> Vec<usize> {
    let cap = usize::try_from(bound / scale).unwrap_or(0);
    let scaled = |c: i64| (c + scale - 1) / scale;
    let mut best: Vec<Score> = vec![0; cap + 1];
    let mut arg: Vec<Vec<u16>> = Vec::with_capacity(groups.len());
    for g in groups {
        // The options this group can afford at all, scaled and penalised.
        let opts: Vec<(u16, usize, Score)> = g
            .options
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(oi, o)| {
                let c = usize::try_from(scaled(o.cost)).ok().filter(|&c| c <= cap)?;
                let add = score(o.value - lambda * i64::from(o.slots), o.cost, o.slots);
                Some((oi as u16, c, add))
            })
            .collect();
        let mut pick = vec![0u16; cap + 1];
        // In place, the budget downward: every option of cost c >= 1 reads
        // best[b - c], which this group has not written yet, so the row
        // never needs copying. A cost-0 option reads the old best[b],
        // kept in `old`. The first option to reach a score keeps it.
        for b in (0..=cap).rev() {
            let old = best[b];
            let mut cur = old;
            for &(oi, c, add) in &opts {
                if c > b {
                    continue;
                }
                let cand = if c == 0 { old } else { best[b - c] } + add;
                if cand > cur {
                    cur = cand;
                    pick[b] = oi;
                }
            }
            best[b] = cur;
        }
        arg.push(pick);
    }
    // Walk back from the full bound.
    let mut chosen = vec![0usize; groups.len()];
    let mut b = cap;
    for (gi, g) in groups.iter().enumerate().rev() {
        let oi = usize::from(arg[gi][b]);
        chosen[gi] = oi;
        if oi != 0 {
            b -= usize::try_from(scaled(g.options[oi].cost)).unwrap_or(0);
        }
    }
    chosen
}

/// The totals of a choice.
fn totals(groups: &[Group], chosen: &[usize]) -> (i64, i64, i32, i64) {
    chosen.iter().enumerate().fold((0, 0, 0, 0), |(v, c, s, w), (g, &o)| {
        let x = &groups[g].options[o];
        (v + x.value, c + x.cost, s + x.slots, w + x.weight)
    })
}

/// Solve: the chosen option per group under the limits.
#[cfg_attr(not(test), expect(dead_code))]
pub fn solve(groups: &[Group], limits: &Limits) -> Solution {
    // The gold bound: the purse, or less when every group's dearest
    // option together costs less.
    let dearest: i64 = groups
        .iter()
        .map(|g| g.options.iter().map(|o| o.cost).max().unwrap_or(0))
        .sum();
    let bound = limits.gold.max(0).min(dearest);
    let scale = if bound > GOLD_LIMIT { (bound + GOLD_LIMIT - 1) / GOLD_LIMIT } else { 1 };

    // The slots: the sweep, when the gold-only optimum does not fit.
    let slot_limit = limits.pack_slots.max(0);
    let fits = |ch: &[usize]| totals(groups, ch).2 <= slot_limit;
    let mut lambda = 0;
    let mut chosen = dp(groups, bound, scale, 0);
    if !fits(&chosen) {
        // Above the largest option value, nothing slot-opening pays.
        let mut hi = groups
            .iter()
            .flat_map(|g| g.options.iter().map(|o| o.value))
            .max()
            .unwrap_or(0)
            + 1;
        // (Computed only if the search never finds a smaller one.)
        let mut hi_choice = None;
        let mut lo = 0;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let c = dp(groups, bound, scale, mid);
            if fits(&c) {
                hi = mid;
                hi_choice = Some(c);
            } else {
                lo = mid;
            }
        }
        lambda = hi;
        chosen = hi_choice.unwrap_or_else(|| dp(groups, bound, scale, hi));
    }

    // The weight: downgrade until it holds.
    let mut weight_repaired = false;
    loop {
        let (_, _, _, w) = totals(groups, &chosen);
        if w <= limits.weight {
            break;
        }
        // The downgrade losing the least value per weight freed; ties to
        // the earlier group, then the larger remaining option.
        let mut pick: Option<(usize, usize, i64, i64)> = None;
        for (g, &o) in chosen.iter().enumerate() {
            let cur = &groups[g].options[o];
            for (j, alt) in groups[g].options.iter().enumerate().take(o) {
                let freed = cur.weight - alt.weight;
                if freed <= 0 {
                    continue;
                }
                let loss = cur.value - alt.value;
                let better = match pick {
                    None => true,
                    Some((_, _, l, f)) => i128::from(loss) * i128::from(f) < i128::from(l) * i128::from(freed),
                };
                if better {
                    pick = Some((g, j, loss, freed));
                }
            }
        }
        let Some((g, j, _, _)) = pick else { break };
        chosen[g] = j;
        weight_repaired = true;
    }

    let (value, cost, slots, weight) = totals(groups, &chosen);
    Solution {
        chosen,
        value,
        cost,
        slots,
        weight,
        lambda,
        gold_exact: scale == 1,
        slots_met: limits.pack_slots >= 0 || slots == 0,
        weight_repaired,
    }
}

/// Build the groups from the stock, after the budget's floors.
///
/// * `caution` is `k` of the share cap (`borg_town_share`; 0 disables
///   it): an option's Black Market copies may cost at most `purse / k`
///   together (per group, Q2), `purse` the budget's gold;
/// * `wearable(c)` names the equipment slot a ware is worn in and the
///   value of wearing it (the try-on), or `None` for a ware that stacks;
/// * `value(c, n)` is the usefulness of `n` copies of `c`'s class - the
///   flat-then-capped curve of phase 1 in production. The table stops
///   where it stops rising (the cap), and where the copies run out.
#[cfg_attr(not(test), expect(dead_code))]
pub fn groups(
    stock: &Stock,
    budget: &Budget,
    caution: i64,
    wearable: impl Fn(&Candidate) -> Option<(u8, i64)>,
    value: impl Fn(&Candidate, i32) -> i64,
) -> Vec<Group> {
    // The copies left after the floors.
    let mut qty: Vec<i32> = stock.candidates.iter().map(|c| c.qty).collect();
    for f in &budget.floors {
        qty[f.candidate] -= 1;
    }
    let purse = budget.gold.max(0);
    let allowed = |bm_cost: i64| caution <= 0 || bm_cost.saturating_mul(caution) <= purse;

    let mut by_class: BTreeMap<u16, Vec<usize>> = BTreeMap::new();
    let mut by_slot: BTreeMap<u8, Vec<(usize, i64)>> = BTreeMap::new();
    for (i, c) in stock.candidates.iter().enumerate() {
        if qty[i] < 1 {
            continue;
        }
        match wearable(c) {
            Some((slot, v)) => by_slot.entry(slot).or_default().push((i, v)),
            None => by_class.entry(c.class).or_default().push(i),
        }
    }

    let mut out = Vec::new();
    for (class, mut wares) in by_class {
        let cands = &stock.candidates;
        wares.sort_by_key(|&i| (cands[i].price, cands[i].shop, cands[i].slot));
        // A class opens a slot unless a carried stack absorbs it.
        let opens = i32::from(wares.iter().all(|&i| cands[i].slots_cost != 0));
        let rep = &cands[wares[0]];
        let mut options = vec![Choice::default()];
        let mut prev = 0;
        let mut n = 0;
        loop {
            n += 1;
            // The n cheapest copies.
            let mut left = n;
            let mut picks = Vec::new();
            for &i in &wares {
                if left == 0 {
                    break;
                }
                let take = left.min(qty[i]);
                if take > 0 {
                    picks.push((i, take));
                    left -= take;
                }
            }
            if left > 0 {
                break; // the copies ran out
            }
            let v = value(rep, n);
            if v <= prev {
                break; // the cap: more copies add nothing
            }
            let cost = picks.iter().map(|&(i, k)| i64::from(cands[i].price) * i64::from(k)).sum();
            let bm: i64 = picks
                .iter()
                .filter(|&&(i, _)| cands[i].is_black_market())
                .map(|&(i, k)| i64::from(cands[i].price) * i64::from(k))
                .sum();
            if !allowed(bm) {
                break; // the caution truncates the table
            }
            let weight = picks.iter().map(|&(i, k)| i64::from(cands[i].weight) * i64::from(k)).sum();
            options.push(Choice { picks, cost, value: v, slots: opens, weight });
            prev = v;
        }
        if options.len() > 1 {
            out.push(Group { key: GroupKey::Class(class), options });
        }
    }
    for (slot, wares) in by_slot {
        let cands = &stock.candidates;
        let mut options = vec![Choice::default()];
        for (i, v) in wares {
            let c = &cands[i];
            let cost = i64::from(c.price);
            if v <= 0 || (c.is_black_market() && !allowed(cost)) {
                continue;
            }
            options.push(Choice {
                picks: vec![(i, 1)],
                cost,
                value: v,
                slots: c.slots_cost,
                weight: i64::from(c.weight),
            });
        }
        if options.len() > 1 {
            out.push(Group { key: GroupKey::Slot(slot), options });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borg_town_budget::{Facts, FloorPurchase};
    use crate::borg_town_stock::tests::{build, item, town};
    use crate::borg_town_usefulness::value as curve;

    fn opt(cost: i64, value: i64, slots: i32, weight: i64) -> Choice {
        Choice { picks: Vec::new(), cost, value, slots, weight }
    }

    /// A class group from (cost, value) per copy count, one slot.
    fn class(key: u16, table: &[(i64, i64)]) -> Group {
        let mut options = vec![Choice::default()];
        options.extend(table.iter().map(|&(c, v)| opt(c, v, 1, 0)));
        Group { key: GroupKey::Class(key), options }
    }

    fn limits(gold: i64) -> Limits {
        Limits { gold, pack_slots: 23, weight: 1_000_000 }
    }

    /// Brute force over every combination: the best value under gold
    /// and slots (and the tie order).
    fn brute(groups: &[Group], l: &Limits) -> i64 {
        fn go(g: &[Group], i: usize, gold: i64, slots: i32, acc: i64, best: &mut i64) {
            if i == g.len() {
                *best = (*best).max(acc);
                return;
            }
            for o in &g[i].options {
                if o.cost <= gold && o.slots <= slots {
                    go(g, i + 1, gold - o.cost, slots - o.slots, acc + o.value, best);
                }
            }
        }
        let mut best = 0;
        go(groups, 0, l.gold, l.pack_slots, 0, &mut best);
        best
    }

    /// P15, the worked instance: A worth 12 for 6 (cap 1), B worth 9 a
    /// copy for 5 (cap 2), purse 10. The greedy by ratio takes A and
    /// scores 12; the optimum is two B for 18.
    #[test]
    fn p15_the_greedy_scores_12_the_optimum_is_18() {
        let g = [class(0, &[(6, 12)]), class(1, &[(5, 9), (10, 18)])];
        let s = solve(&g, &limits(10));
        assert_eq!(s.value, 18);
        assert_eq!(s.chosen, [0, 2]);
        assert_eq!(s.cost, 10);
        assert!(s.gold_exact);
    }

    /// Exact against brute force on small seeded instances, gold-only.
    #[test]
    fn the_gold_dp_is_exact() {
        let mut seed: u64 = 0x5eed;
        let mut rnd = |m: i64| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((seed >> 33) as i64) % m
        };
        for _ in 0..200 {
            let groups: Vec<Group> = (0..4)
                .map(|k| {
                    let (mut c, mut v) = (0, 0);
                    let table: Vec<(i64, i64)> = (0..1 + rnd(3))
                        .map(|_| {
                            c += 1 + rnd(9);
                            v += 1 + rnd(9);
                            (c, v)
                        })
                        .collect();
                    class(k, &table)
                })
                .collect();
            let l = limits(rnd(30));
            let s = solve(&groups, &l);
            assert_eq!(s.value, brute(&groups, &l), "{groups:?} {l:?}");
            assert!(s.cost <= l.gold);
        }
    }

    /// The pack slots bind: the gold-only optimum takes three kinds, two
    /// slots are free, and the sweep finds the best two.
    #[test]
    fn the_sweep_fits_the_slots() {
        let g = [class(0, &[(1, 10)]), class(1, &[(1, 9)]), class(2, &[(1, 8)])];
        let mut l = limits(100);
        assert_eq!(solve(&g, &l).slots, 3);
        l.pack_slots = 2;
        let s = solve(&g, &l);
        assert_eq!(s.slots, 2);
        assert_eq!(s.value, 19);
        assert_eq!(s.value, brute(&g, &l));
        assert!(s.lambda > 0);
        assert!(s.slots_met);
    }

    /// A copy that joins a carried stack opens no slot, so it is free
    /// under the slot limit.
    #[test]
    fn a_stacking_group_costs_no_slot() {
        let mut free = class(1, &[(1, 5)]);
        free.options[1].slots = 0;
        let g = [class(0, &[(1, 10)]), free];
        let l = Limits { gold: 100, pack_slots: 1, weight: 1_000_000 };
        let s = solve(&g, &l);
        assert_eq!(s.chosen, [1, 1]);
        assert_eq!(s.slots, 1);
    }

    /// A negative slot limit (the floors overfilled the pack): nothing
    /// slot-opening is bought, and the solution says the limit was not
    /// met rather than pretending.
    #[test]
    fn a_negative_slot_limit_buys_only_what_stacks() {
        let mut free = class(1, &[(1, 5)]);
        free.options[1].slots = 0;
        let g = [class(0, &[(1, 10)]), free];
        let l = Limits { gold: 100, pack_slots: -1, weight: 1_000_000 };
        let s = solve(&g, &l);
        assert_eq!(s.chosen, [0, 1]);
        assert!(s.slots_met, "zero slots opened");
    }

    /// The weight repair drops the least value per weight freed.
    #[test]
    fn the_weight_repair_drops_the_cheapest_loss() {
        let g = [
            Group { key: GroupKey::Class(0), options: vec![Choice::default(), opt(1, 10, 0, 50)] },
            Group { key: GroupKey::Class(1), options: vec![Choice::default(), opt(1, 9, 0, 10)] },
        ];
        let l = Limits { gold: 100, pack_slots: 23, weight: 55 };
        let s = solve(&g, &l);
        // dropping the heavy one loses 10 for 50 freed; the light one
        // would free too little
        assert_eq!(s.chosen, [0, 1]);
        assert!(s.weight_repaired);
        assert!(s.weight <= 55);
    }

    /// A purse that covers everything buys every useful option.
    #[test]
    fn a_purse_that_covers_everything() {
        let g = [class(0, &[(5, 3), (10, 4)]), class(1, &[(7, 9)])];
        let s = solve(&g, &limits(1_000));
        assert_eq!(s.chosen, [2, 1]);
        assert_eq!(s.cost, 17);
    }

    /// Ties go to less gold, then fewer slots, then the earlier group.
    #[test]
    fn the_ties_are_deterministic() {
        let g = [class(0, &[(5, 10)]), class(1, &[(4, 10)]), class(2, &[(4, 10)])];
        let s = solve(&g, &limits(4));
        assert_eq!(s.chosen, [0, 1, 0], "the cheaper, then the earlier");
        assert_eq!(solve(&g, &limits(4)), s);
    }

    /// Above the gold limit the costs are scaled up-rounded: the
    /// solution never overspends, and says it is not exact.
    #[test]
    fn a_large_purse_is_scaled_and_marked() {
        let g = [class(0, &[(70_001, 10)]), class(1, &[(70_001, 9)])];
        let s = solve(&g, &limits(140_002));
        assert!(!s.gold_exact);
        assert!(s.cost <= 140_002);
        let s = solve(&g, &limits(100_000));
        assert_eq!(s.chosen, [1, 0]);
    }

    /// The ranking: the most useful first.
    #[test]
    fn the_solution_is_ranked() {
        let g = [class(0, &[(1, 3)]), class(1, &[(1, 9)]), class(2, &[(2, 9)])];
        let s = solve(&g, &limits(10));
        assert_eq!(s.ranked(&g), [(1, 1), (2, 1), (0, 1)]);
    }

    /// The cost of a solve at the size the plan budgets for (risk list:
    /// "to be measured, not assumed"): 168 groups - every ware of seven
    /// shops - of five options, the gold at the limit, and the slots
    /// binding so the sweep runs its full binary search. Run by hand:
    /// `cargo test --release -- --ignored --nocapture solve_cost`.
    #[test]
    #[ignore]
    fn solve_cost_at_full_size() {
        let groups: Vec<Group> = (0..168u16)
            .map(|k| {
                let t: Vec<(i64, i64)> =
                    (1..=5).map(|n| (n * (50 + i64::from(k) * 7), n * (1 + i64::from(k % 13)))).collect();
                class(k, &t)
            })
            .collect();
        let l = Limits { gold: GOLD_LIMIT, pack_slots: 10, weight: 1_000_000 };
        let start = std::time::Instant::now();
        let s = solve(&groups, &l);
        let took = start.elapsed();
        assert!(s.slots <= 10 && s.cost <= GOLD_LIMIT);
        println!("solve: 168 groups x 5 options, gold {GOLD_LIMIT}, lambda {}: {took:?}", s.lambda);

        // The plan's own estimate (P15): 30,000 gold x 30 kinds x 20
        // copies, the slots binding as well.
        let groups: Vec<Group> = (0..30u16)
            .map(|k| {
                let t: Vec<(i64, i64)> =
                    (1..=20).map(|n| (n * (40 + i64::from(k) * 11), n * (1 + i64::from(k % 7)))).collect();
                class(k, &t)
            })
            .collect();
        let l = Limits { gold: 30_000, pack_slots: 10, weight: 1_000_000 };
        let start = std::time::Instant::now();
        let s = solve(&groups, &l);
        let took = start.elapsed();
        assert!(s.slots <= 10 && s.cost <= 30_000);
        println!("solve: 30 groups x 20 options, gold 30000, lambda {}: {took:?}", s.lambda);
    }

    // --- the groups from the stock (B2 section 7) ---

    fn budget(gold: i64) -> Budget {
        let f = Facts { gold, pack_capacity: 23, home_capacity: 24, carry: 2000, weight: 0 };
        let s = build(&town([&[], &[], &[], &[], &[], &[], &[], &[]]), &[]);
        Budget::build(&f, &s, &[], |_| vec![])
    }

    /// A value table: `cap` copies, `per` each (flat then capped).
    fn capped(cap: i32, per: i64) -> impl Fn(&Candidate, i32) -> i64 {
        move |_, n| per * i64::from(n.min(cap))
    }

    /// The cap closes before the 1x copies run out: the 3x price is
    /// never in a table.
    #[test]
    fn the_black_market_is_not_paid_when_the_cap_closes_first() {
        let t = town([&[], &[], &[], &[], &[item(1, 2, 20)], &[], &[item(1, 3, 60)], &[]]);
        let s = build(&t, &[]);
        let g = groups(&s, &budget(1_000), 10, |_| None, capped(2, 50));
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].options.len(), 3);
        assert!(g[0].options.iter().flat_map(|o| &o.picks).all(|&(i, _)| !s.candidates[i].is_black_market()));
    }

    /// A deficit of four: the third and fourth copies come from the
    /// Black Market, within the caution (per group: 120 * 10 <= 1200);
    /// a smaller purse truncates the table there.
    #[test]
    fn the_black_market_is_paid_within_the_caution_per_group() {
        let t = town([&[], &[], &[], &[], &[item(1, 2, 20)], &[], &[item(1, 3, 60)], &[]]);
        let s = build(&t, &[]);
        let g = groups(&s, &budget(1_200), 10, |_| None, capped(4, 50));
        assert_eq!(g[0].options.len(), 5);
        assert_eq!(g[0].options[4].cost, 20 + 20 + 60 + 60);
        // purse 1199: two BM copies (120) break the cap, one (60) does not
        let g = groups(&s, &budget(1_199), 10, |_| None, capped(4, 50));
        assert_eq!(g[0].options.len(), 4);
        // caution 0 disables the cap
        let g = groups(&s, &budget(1), 0, |_| None, capped(4, 50));
        assert_eq!(g[0].options.len(), 5);
    }

    /// Two rings of one kind with different bonuses are two classes,
    /// hence two groups - never one cheapest-n curve.
    #[test]
    fn a_split_class_is_two_groups() {
        let mut better = item(1, 1, 500);
        better.to_h = 5;
        let t = town([&[item(1, 1, 100), better], &[], &[], &[], &[], &[], &[], &[]]);
        let s = build(&t, &[]);
        let g = groups(&s, &budget(1_000), 10, |_| None, capped(1, 30));
        assert_eq!(g.len(), 2);
        assert_ne!(g[0].key, g[1].key);
    }

    /// Q1: two wearables for one equipment slot are one group, and at
    /// most one is ever chosen, whatever the purse.
    #[test]
    fn two_wearables_for_one_slot_are_never_both_chosen() {
        let t = town([&[], &[item(7, 1, 100), item(8, 1, 120)], &[], &[], &[], &[], &[], &[]]);
        let s = build(&t, &[]);
        let wear = |c: &Candidate| match c.kind {
            7 => Some((3u8, 40)),
            8 => Some((3u8, 45)),
            _ => None,
        };
        let g = groups(&s, &budget(10_000), 10, wear, |_, _| 0);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].key, GroupKey::Slot(3));
        let sol = solve(&g, &limits(10_000));
        assert_eq!(sol.chosen, [2], "the better armour, not both");
        assert_eq!(sol.value, 45);
    }

    /// The floors' copies are gone before the tables are built.
    #[test]
    fn the_floors_copies_are_removed() {
        let t = town([&[item(1, 2, 5)], &[], &[], &[], &[], &[], &[], &[]]);
        let s = build(&t, &[]);
        let mut b = budget(100);
        b.floors = vec![FloorPurchase { candidate: 0, needs: vec![] }];
        let g = groups(&s, &b, 10, |_| None, capped(5, 10));
        assert_eq!(g[0].options.len(), 2, "one copy left after the floor");
    }

    /// Q3, the surrogate pinned as intended: two kinds of one need, each
    /// priced against the whole gap (2), are both chosen in one solve;
    /// after the first purchase closes half the gap, the re-solve prices
    /// the second against what is left (1) and stops at the cap.
    #[test]
    fn the_surrogate_and_the_re_solve() {
        let t = town([&[], &[], &[], &[], &[item(1, 5, 20), item(2, 5, 30)], &[], &[], &[]]);
        let s = build(&t, &[]);
        let gap = std::cell::Cell::new(2i64);
        let need = |_: &Candidate, n: i32| curve(100, 1, gap.get(), i64::from(n));
        let g = groups(&s, &budget(1_000), 10, |_| None, need);
        let first = solve(&g, &Limits::of(&budget(1_000)));
        assert_eq!(first.chosen, [2, 2], "both kinds, each against the whole gap");
        // one copy bought: the gap is 1 now
        gap.set(1);
        let g = groups(&s, &budget(1_000), 10, |_| None, need);
        let again = solve(&g, &Limits::of(&budget(1_000)));
        let copies: i32 = again
            .chosen
            .iter()
            .enumerate()
            .flat_map(|(gi, &o)| g[gi].options[o].picks.iter().map(|&(_, k)| k))
            .sum();
        assert_eq!(copies, 2, "one copy per kind against the gap that is left");
    }
}
