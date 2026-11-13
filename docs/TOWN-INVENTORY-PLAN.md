# The town inventory strategy: the implementation plan

Status: **accepted**. Phases 0-2 landed 2026-11-12, phase 3 on 2026-11-13; the owner decisions of
2026-11-12 (the epic log, full record on PR #42) are applied inline.

Epic: borg-rs#36. Issues: #37 (the extraction), #35 (the usefulness function), #38 (the architecture), #39 (the optimisation problem), #40 (the joint treatment).
Branch: `feature/town-inventory-strategy`.

## What is being built

One town visit becomes one plan. For every item the character has and every ware the shops hold, the plan decides **keep, store, sell or drop**, and then chooses **what to buy**, subject to one gold budget, 23 pack slots counted per distinct stack, the Home's slots, the carry allowance, and the floors - the kit predicates: at least one food, at least one light, a cure for the condition the character has, and the poison-cure capability - which are bought first and never traded. A floor the shops cannot meet does not block the visit: the plan buys what is possible, records the unmet floor, and #37's Stay rules decide whether the character leaves town with it unmet.

The judgement is a **usefulness function**: an item's worth is the fraction of a *deficit* it closes, and the deficits are light, food, bleeding, poison, escape, knowledge, power and trip. One weight vector over those needs is one zoo arm; everything else is derived from the state.

This plan implements the decisions in the epic's log. It does not re-open them.

## The parameter vector, and the six places a setting has to land

Every arm is a point in this space:

| setting | default | meaning |
|---|---|---|
| `borg_town_plan` | 0 | 0 = today's behaviour, 1 = the plan |
| `borg_town_w_light` | 100 | the weight of the light need |
| `borg_town_w_food` | 100 | the weight of the food need |
| `borg_town_w_bleeding` | 100 | the weight of the bleeding need |
| `borg_town_w_poison` | 100 | the weight of the poison need |
| `borg_town_w_escape` | 50 | the weight of the escape need |
| `borg_town_w_knowledge` | 25 | the weight of the knowledge need |
| `borg_town_w_power` | 50 | the weight of the power need |
| `borg_town_w_trip` | 50 | the weight of the trip need |
| `borg_town_share` | 10 | the Black Market's share cap `k`: `cost <= purse / k` |
| `borg_town_light_depth` | 15 | the descent target at which a second light is wanted (provisional) |
| `borg_town_food_trip` | 10 | rations per expected trip (provisional) |

`w_light = 100` is the numeraire, so the vector has seven free weights. A
thirteenth setting, `borg_town_no_stores`, is **not** part of the vector: it
is the negative-baseline control (see "The negative baseline"), incompatible
with money-scumming, and it lands in the same six places.

**A setting has to land in six places, or the parity check breaks.**

1. `angband/src/borg/borg.h` - the `BORG_*` enum. The Rust table must match it; `table_matches_enum` asserts that they do.
2. `angband/src/borg/borg-init.c` - the C parser table. The C Borg **registers and ignores** these, exactly as it does the seven family settings. This is not optional: the parity compares the two backends' *effective settings*, so a setting the C side does not know would make the two disagree.
3. `borg-rs/src/borg_headers/borg_h.rs` and `borg-rs/src/borg_init.rs` - the Rust enum mirror, the Rust table and the defaults.
4. `harness/borg.txt` - the pinned config, with the comment lines, because the harness rewrites one config line per flag and **fails the run if a flag did not change a line**.
5. `angband/src/borg/borg.txt` - the shipped config. The two `borg.txt` copies are read by different tools, and `the_borg_txt_copies_agree_and_name_every_setting` fails when they drift.
6. The declaration: `NEW_SETTINGS` in `borg-rs/src/borg_init.rs`, with `docs/ZOO-INTERFACE.md` as its prose twin. Since the defaults audit (merged 2026-11-12) `every_setting_is_declared` fails when a setting is added without a declaration - adding a setting is a deliberate act.

The twelve settings are the price of a swept parameter vector. The alternative - a per-arm settings file the harness reads - would keep them out of the parser, but then the arms would not be settings at all, and the C Borg would have to be told to ignore a file it does not understand.

## The phases

Each phase is one commit and each lands with its own suite. "Independently
measurable" holds at the unit level for phases 0-3; from phase 4 on the arms
are the measurement (S8).

### Phase 0 - the settings (no behaviour change)

The twelve settings in the six places above, the defaults in both `borg.txt`
copies, the declaration rows, and the null arm's gate test: the gate is
extracted pure (`null_arm_gate`) so the test binary can link it, and
`no_stores_fires_and_clears_the_goal` pins the contract that no TownAction is
ever executed. **Proves:** the parameter vector is expressible, and the two
backends report the same effective settings. **Proves nothing about
behaviour**, which is the point.

Phase 0 is where the receipt is re-run and re-earned: the settings plumbing
changes the corpus identity, so the phase-0 commit re-runs the headless
comparison with the new settings in the pinned config and the plan off.

Landed 2026-11-12.

### Phase 1 - the usefulness function

`borg-rs/src/borg_town_usefulness.rs`: the needs, the deficits, the weights,
the caution, and the value curve - `U_K(n) = w_K * min(1, n*r/g)`, flat then
capped. The marginal value of a copy is the constant `w*r/g` until the
deficit closes, and zero at or above the target: a met need prices nothing.
The curve is weakly concave, which is what keeps phase 3's DP exact (P15).
Nothing calls the module yet; every public item carries a self-removing
`expect(dead_code)` marker that the first production caller fulfils.

Its suite asserts the **values** for fixed states - a table of states and
their expected usefulness - so a failure points at the judgement and not at
the planner, and the curve's shape as properties: flat then capped, zero at
or above the target, weakly concave and capped by the weight. This is the
first artefact in the plan that carries the owner's judgement, and it is the
one to read first.

**Depends on:** phase 0. The trip model and the power target are phase-1
dependencies and landed with it, provisionally (see "The provisional
values").

Landed 2026-11-12.

### Phase 2 - the stock and the budget

`borg-rs/src/borg_town_stock.rs` and `borg-rs/src/borg_town_budget.rs`: the candidates (every ware in every shop, the pack and the Home as groups of stacks, and the alternatives - kind-only, priced cumulative-cheapest-n across the shops), and the budget (the floors, the gold, the pack slots per distinct stack, the Home's slots, the carry allowance). The slot accounting (S3) - 23 pack slots counted per distinct stack - is pinned by a contract test (`pack_slots_are_counted_per_distinct_stack`).

The floors' pass is exact over the at most four unmet floors - the most floors met, then the least gold, then the fewest wares - and bounded by the gold alone; a floor it cannot meet is recorded as unmet, not blocking. The reduction table is **not** in this phase (corrected 2026-11-12): it keys on the catalogue of useful kinds (#35 item 4), which is still open, so the candidates carry kind, tval and sval for it and the table lands with the catalogue. The floors' classification, the one piece of it the budget needs, mirrors `borg_trait`'s counting.

**Depends on:** phase 1.

Landed 2026-11-12.

### Phase 3 - the solver

`borg-rs/src/borg_town_solve.rs`: the exact DP over the gold (the group knapsack with weakly concave per-kind values), the Lagrangian sweep on the kind count for the pack slots, the repair for the slot and weight constraints, and the floors as a lexicographic first pass. The gold bound is `min(purse + proceeds, the sum of all candidate costs)`.

Its suite asserts the chosen set for a small instance with a known optimum, and the instance from the worked example - the greedy scores 12 where the optimum is 18 - so the greedy cannot pass it by accident; plus the boundary cases (an infeasible floor, a forbidden ware, a purse that covers everything) and a determinism test.

**Depends on:** phases 1-2. **Entry criterion:** the B2 write-up - the
kind-only alternatives and the separability they leave the DP - lands
first; drafted in `docs/TOWN-INVENTORY-B2.md` for owner acceptance.
(Corrected 2026-11-12: this said "the decision record for the value
curve", but the value curve is B1 and already decided; B2 is the
alternatives, per the epic's log and the PR #42 record.)
**Met 2026-11-13**: the owner accepted the B2 write-up with its three
recommendations (Q1 wearables as one multiple-choice group per equipment
slot; Q2 the caution per group; Q3 exact for the separable surrogate).

Landed 2026-11-13. As built: the solver takes groups of options (a stack
class's 0..n cheapest copies, or one equipment slot's "none or one ware")
and the limits after the floors, so the floors' "lexicographic first pass"
is phase 2's budget, and `groups()` removes the floors' copies before it
builds the tables. The values come in as a closure (the reduction table
keys on the still-open catalogue, #35 item 4; phase 4 supplies it from the
usefulness function). The gold bound is `min(gold, the sum of every
group's dearest option)`; the proceeds join it in phase 5. Above 65,536
gold the costs are scaled, rounded up, and the solution says it is not
exact; a negative slot limit buys only what stacks, and says so. The
Lagrangian sweep is not guaranteed exact (P15: "usually exact or within
one item"); the suite checks it against brute force on a binding instance
and the gold DP against brute force on 200 seeded instances.

### Phase 4 - the plan

`borg-rs/src/borg_town_plan.rs`: the decision cycle, the action enum, and the record - for each action, what it was for and what it displaced. `borg_think_store.rs` gains the gate: with `borg_town_plan` off, the old path runs untouched.

**The test that matters here:** with the plan off, the **key log is byte-identical** to today's. That is the strongest guarantee available, and it is what keeps `main`'s receipt valid while the branch is built.

**Depends on:** phase 3. **Entry criteria:** #37's Stay rules and the floors'
infeasibility outcome are decided; the record schema (P13) is fixed before
the phase starts; the sell-side suite moves into this phase; and the
shadow-plan mode lands here - the plan computed but not executed, the record
diffed against the current policy's visit.

### Phase 5 - the joint treatment

The iterated form of #40: alternate between solving the purchase knapsack and re-deciding the dispositions against what the money would buy, with the round cap and the tie-break fixed.

**Depends on:** phase 4. **Entry criterion:** #40's decisions - the round cap
and the tie-break - land first.

### Phase 6 - the arms

The harness suites - the reworked `family-buying` and `family-identify`, and a new suite for the plan - the zoo specimen, and the prespecified endpoint.

**Depends on:** phase 5.

## The provisional values

The trip model and the deficits' arithmetic landed provisionally with phase 1 (#35's open items 1 and 3, and #38's P2 and P3 stay open until the zoo argues them). They are explicit provisional values, and the two that are **settings** can be swept:

| quantity | provisional | note |
|---|---|---|
| the trip's depth target | the deepest level visited plus five | the trip model, stated once |
| the trip's turns per level | 1000 | anchored on the 2000-turn give-up |
| `required_light(depth)` | 1 while the descent target is below depth 15, 2 at or above it | `borg_town_light_depth` |
| `required_food(trip)` | 10 rations per expected trip | `borg_town_food_trip` |
| `escapes_needed(ahead)` | 1 Phase Door per 10 levels of the expected trip | |
| the power target | 2000 x the depth target | the weakest of the provisional values |
| the bleed deficit | 1 while `BI_ISCUT` is set | the game's own bleed rate is the better number |
| the poison deficit | 1 while `BI_ISPOISONED` is set, plus a capability deficit while `BI_ACUREPOIS == 0` | the game's poison damage rate is the better number |

These are the values to argue about; the machinery around them is not in question.

## The test strategy

- **each phase lands with its own suite**, and the suite asserts the layer's own contract: values for the usefulness function, the chosen set for the solver, the byte-identical key log for the gate;
- **the key log is the observable** for the arms, and the record has to show a plan - what each action was for and what it displaced - not just what happened;
- **the suites that assert the current algorithm's shape are reworked, not extended**: `family-buying` counts purchase anchors, `family-identify` asserts that the learning purchase happens *first*;
- **the sell side gets a suite of its own**, because there is none today - only the parity receipt's key-log comparison covers it;
- **an arm is a measurement request until an endpoint is prespecified** and a confirmation suite agrees (the zoo's RULES 8).

## The parity and the receipt

The plan is off by default, so `main` and the settings receipt are untouched while the branch is built - the receipt is re-run and re-earned at phase 0 (now 57/57), because the settings plumbing changes the corpus identity. With the plan on, the key log diverges from the C Borg by construction - the town parity is void for the arm - and the arm is measured against the Rust default. The receipt is re-earned when the default moves.

## The negative baseline

The redesign needs a floor under its own measurement: an arm in which the town inventory strategy does **nothing** at all.

`borg_town_no_stores = TRUE` (a new setting, default FALSE) makes the Borg perform **no store interaction of any kind**: it never buys, never sells, never uses the home as a stash, never grabs from the home, never swaps a spare weapon or armour out of the home, and never walks into a shop. Everything else about it is unchanged - it still recalls to town, still equips itself, still dives.

The gate is one check at the top of `borg_choose_shop` (`borg-rs/src/borg_think_store.rs`): with the setting on it returns false before any goal is set, so the whole store sequence (steps 1 to 7 and the immediate-buy path) never starts. Because the Borg never sets `goal.shop`, it never enters a shop in the first place - the walk to the shop is skipped too, not just the trade.

The contract is that **no TownAction is ever executed**. The check itself is
extracted pure (`null_arm_gate`) so a test binary can link it, and
`no_stores_fires_and_clears_the_goal` pins it: the visit is forbidden and a
standing shop goal is cleared. The wiring is the zoo endpoint's invariant -
`goal.shop` stays -1 for the whole run (borg-zoo#81). The arm's scope is
trade and stash combined: no buying, no selling, no home use. And it is
incompatible with money-scumming: an arm that cannot sell cannot scum.

This is the **null arm**: it measures what the town economy is worth, with nothing else changed. It is not a candidate strategy and it is not expected to be competitive - a Borg that cannot sell keeps a full pack of loot and cannot buy food, light or gear. What it is good for is the other direction: if an arm's gain over the control is real, this arm shows how much of the effect comes from the town at all, and it exposes the degenerate cases (a character that dies before it can trade anything) that a per-class comparison would otherwise average away.

The C Borg registers the setting and ignores it, like `borg_buy_strategy` and `borg_buy_identify`; the Rust Borg is the only implementation. The arm is selected with the harness flag `borg_town_no_stores=TRUE`, which rewrites the one `borg_town_no_stores = ` line in `harness/borg.txt`.

## The risks

- **the trip model is provisional** - it decides every deficit, and it is the biggest unknown in the plan;
- **the solver's cost per decision** - estimated at about 10ms for the 18M-operation DP, so 0.1-0.75s per case, assuming 1-5 town visits per decision point and at most 15 re-solves per visit; to be measured, not assumed. **Measured 2026-11-13** (phase 3, `solve_cost_at_full_size`, a 2.3 GHz Xeon, release): the plan's own size - 30,000 gold x 30 kinds x 20 copies, the slots binding - takes **199 ms** per solve, about 7 sweep passes of 28 ms; a pessimistic 168 groups x 5 options at 65,536 gold takes 680 ms. A cold sweep on every re-solve is therefore above the estimate (15 re-solves ~ 3 s a visit); the obvious remedy - starting each re-solve's sweep from the previous lambda - belongs to phase 4's re-solve loop;
- **the try-on's mutate/restore cost** - the plan evaluates candidates against a mutated copy of the state, and the restore has to be exact or the visit leaks into the next one;
- **the joint treatment's convergence** - the round cap and the tie-break are not decided (#40);
- **the option value of an unidentified item is a range, not a number**, and does not fit the objective cleanly;
- **the two-stage loss** - 350 against 300 in the worked example - if phase 5 is not implemented;
- **the settings must land in six places**, or the parity's effective-settings comparison breaks;
- **the harness's per-run timeout** (180s in the measurement controls) against the solver's cost.

## What a reviewer should check

1. the **phase order** and the dependencies between them;
2. the **provisional values** - they are the judgement, and they are the part most worth arguing about;
3. the **settings list** and the six places each has to land;
4. that **each phase is independently measurable** at the unit level for phases 0-3, and that phases 0-4 do not change the default path;
5. that the **record** (what each action was for) is enough to read a plan out of the key log.

## What is still under-specified

The provisional values stand in for the missing pieces, so nothing here blocks the landed phases. Still open, by the phase that needs it:

- **phase 2** (landed): the alternatives are kind-only (decided 2026-11-12, #35 item 5); the catalogue of useful kinds (#35 item 4), and the reduction table that keys on it, moved past phase 2 and are needed by phase 3's evaluation;
- **phase 3**: the B2 write-up - accepted 2026-11-13 with its three recommendations (`docs/TOWN-INVENTORY-B2.md`); the kind-only alternatives and what they leave the solver (corrected 2026-11-12: this line called it "the value-curve decision record", but the value curve is B1, decided; B2 is the alternatives, per the epic's log and PR #42);
- **phase 4**: the floors' infeasibility outcome (#37's Stay rules), the record schema (P13), the sell-side suite, the shadow-plan mode;
- **phase 5**: the opportunity value of a gold, the round cap and the tie-break (#40);
- **phase 6**: the endpoint for the arms, and how many arms the zoo can afford (#38 P14, #36);
- **any phase**: the trip model's arithmetic (#35 item 3, #38 P2) and the deficits' arithmetic in the game's own numbers rather than booleans (#35 item 1, #38 P3) - the provisionals stand until the zoo argues them; the fraction of the whole gap or of the gap above the floor (#35 item 6).
