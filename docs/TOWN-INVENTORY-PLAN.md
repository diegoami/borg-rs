# The town inventory strategy: the implementation plan

Status: **for review**.

> **Superseded in part, 2026-11-12.** The owner decision entry in #36 (full record on PR #42) supersedes parts of this plan: the value curve is flat-then-capped, `U_K(n) = w_K · min(1, n·r/g)`; the alternatives are kind-only, priced cumulative-cheapest-n across shops; the floors are kit predicates with a defined infeasibility outcome (#37's Stay rules - buy what is possible, record the unmet floor, the give-up limits take the character out of town); the record schema (P13) is fixed before phase 4 and the sell-side suite moves into phase 4; the shadow-plan mode lands in phase 4; the settings count is thirteen - `borg_town_no_stores` is a control *outside* the parameter vector; the trip model and the power target are phase-1 dependencies; `w_light = 100` is the numeraire; "each phase is independently measurable" holds at the unit level for phases 0-3 only; phase 0 is where the receipt is re-run and re-earned. The full amendments land with the phase-1 commit; until then the decision log is authoritative.

Epic: borg-rs#36. Issues: #37 (the extraction), #35 (the usefulness function), #38 (the architecture), #39 (the optimisation problem), #40 (the joint treatment).
Branch: `feature/town-inventory-strategy`.

## What is being built

One town visit becomes one plan. For every item the character has and every ware the shops hold, the plan decides **keep, store, sell or drop**, and then chooses **what to buy**, subject to one gold budget, 23 pack slots counted per distinct stack, the Home's slots, the carry allowance, and the floors - food > 0, light > 0, no cut, no poison - which are bought first and never traded.

The judgement is a **usefulness function**: an item's worth is the fraction of a *deficit* it closes, and the deficits are light, food, bleeding, poison, escape, knowledge, power and trip. One weight vector over those needs is one zoo arm; everything else is derived from the state.

This plan implements the decisions in the epic's log. It does not re-open them.

## The parameter vector, and the four places a setting has to land

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
| `borg_town_light_depth` | 15 | the depth at which a second light is wanted (provisional) |
| `borg_town_food_trip` | 10 | rations per expected trip (provisional) |

**A setting has to land in four places, or the parity check breaks.**

1. `angband/src/borg/borg.h` - the `BORG_*` enum. The Rust table must match it; `borg_init.rs:1017` (`table_matches_enum`) asserts that they do.
2. `angband/src/borg/borg-init.c` - the C parser table. The C Borg **registers and ignores** these, exactly as it does the seven family settings (`borg-init.c:97-103`). This is not optional: the parity compares the two backends' *effective settings*, so a setting the C side does not know would make the two disagree.
3. `borg-rs/src/borg_headers/borg_h.rs` and `borg-rs/src/borg_init.rs` - the Rust enum mirror, the Rust table and the defaults (the family settings sit at `borg_init.rs:131-137`).
4. `harness/borg.txt` - the pinned config, with the comment lines, because the harness rewrites one config line per flag and **fails the run if a flag did not change a line**.

The twelve settings are the price of a swept parameter vector. The alternative - a per-arm settings file the harness reads - would keep them out of the parser, but then the arms would not be settings at all, and the C Borg would have to be told to ignore a file it does not understand.

## The phases

Each phase is one commit, each lands with its own suite, and each is independently measurable.

### Phase 0 - the settings (no behaviour change)

The twelve settings in the four places above, the defaults in `borg.txt`, and `table_matches_enum` still passing. **Proves:** the parameter vector is expressible, and the two backends report the same effective settings. **Proves nothing about behaviour**, which is the point.

### Phase 1 - the usefulness function

`borg-rs/src/borg_town_usefulness.rs`: the needs, the deficits, the weights, the caution. Nothing calls it yet.

Its suite asserts the **values** for fixed states - a table of states and their expected usefulness - so a failure points at the judgement and not at the planner. This is the first artefact in the plan that carries the owner's judgement, and it is the one to read first.

**Depends on:** phase 0.

### Phase 2 - the stock and the budget

`borg-rs/src/borg_town_stock.rs` and `borg-rs/src/borg_town_budget.rs`: the candidates (every ware in every shop, the pack and the Home as groups of stacks, and the alternatives - the same kind at 1x elsewhere), and the budget (the floors, the gold, the pack slots per distinct stack, the Home's slots, the carry allowance).

**Depends on:** phase 1.

### Phase 3 - the solver

`borg-rs/src/borg_town_solve.rs`: the exact DP over the gold (the group knapsack with concave per-kind values), the Lagrangian sweep on the kind count for the pack slots, the repair for the slot and weight constraints, and the floors as a lexicographic first pass.

Its suite asserts the chosen set for a small instance with a known optimum, and the instance from the worked example - the greedy scores 10 where the optimum is 11 - so the greedy cannot pass it by accident.

**Depends on:** phases 1-2.

### Phase 4 - the plan

`borg-rs/src/borg_town_plan.rs`: the decision cycle, the action enum, and the record - for each action, what it was for and what it displaced. `borg_think_store.rs` gains the gate: with `borg_town_plan` off, the old path runs untouched.

**The test that matters here:** with the plan off, the **key log is byte-identical** to today's. That is the strongest guarantee available, and it is what keeps `main`'s receipt valid while the branch is built.

**Depends on:** phase 3.

### Phase 5 - the joint treatment

The iterated form of #40: alternate between solving the purchase knapsack and re-deciding the dispositions against what the money would buy, with the round cap and the tie-break fixed.

**Depends on:** phase 4.

### Phase 6 - the arms

The harness suites - the reworked `family-buying` and `family-identify`, and a new suite for the plan - the zoo specimen, and the prespecified endpoint.

**Depends on:** phase 5.

## The provisional values

The trip model and the deficits' arithmetic do not exist yet (#35's open items 1 and 3, and #38's P2 and P3). The plan proceeds with explicit provisional values so that phase 1 can start, and they are **settings**, so they can be swept:

| quantity | provisional | note |
|---|---|---|
| `required_light(depth)` | 1 below depth 15, 2 at or above it | `borg_town_light_depth` |
| `required_food(trip)` | 10 rations per expected trip | `borg_town_food_trip`; the expected trip is assumed to be the last dive plus five levels |
| `escapes_needed(ahead)` | 1 Phase Door per 10 levels of the expected trip | |
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

The plan is off by default, so `main` and the 45/45 receipt are untouched while the branch is built. With the plan on, the key log diverges from the C Borg by construction - the town parity is void for the arm - and the arm is measured against the Rust default. The receipt is re-earned when the default moves.

## The negative baseline

The redesign needs a floor under its own measurement: an arm in which the town inventory strategy does **nothing** at all.

`borg_town_no_stores = TRUE` (a new setting, default FALSE) makes the Borg perform **no store interaction of any kind**: it never buys, never sells, never uses the home as a stash, never grabs from the home, never swaps a spare weapon or armour out of the home, and never walks into a shop. Everything else about it is unchanged - it still recalls to town, still equips itself, still dives.

The gate is one check at the top of `borg_choose_shop` (`borg-rs/src/borg_think_store.rs`): with the setting on it returns false before any goal is set, so the whole store sequence (steps 1 to 7 and the immediate-buy path) never starts. Because the Borg never sets `goal.shop`, it never enters a shop in the first place - the walk to the shop is skipped too, not just the trade.

This is the **null arm**: it measures what the town economy is worth, with nothing else changed. It is not a candidate strategy and it is not expected to be competitive - a Borg that cannot sell keeps a full pack of loot and cannot buy food, light or gear. What it is good for is the other direction: if an arm's gain over the control is real, this arm shows how much of the effect comes from the town at all, and it exposes the degenerate cases (a character that dies before it can trade anything) that a per-class comparison would otherwise average away.

The C Borg registers the setting and ignores it, like `borg_buy_strategy` and `borg_buy_identify`; the Rust Borg is the only implementation. The arm is selected with the harness flag `borg_town_no_stores=TRUE`, which rewrites the one `borg_town_no_stores = ` line in `harness/borg.txt`.

## The risks

- **the trip model is provisional** - it decides every deficit, and it is the biggest unknown in the plan;
- **the solver's cost per decision** - estimated at about 10ms for the 18M-operation DP, so 0.1-0.75s per case; to be measured, not assumed;
- **the joint treatment's convergence** - the round cap and the tie-break are not decided (#40);
- **the option value of an unidentified item is a range, not a number**, and does not fit the objective cleanly;
- **the two-stage loss** - 350 against 300 in the worked example - if phase 5 is not implemented;
- **the settings must land in four places**, or the parity's effective-settings comparison breaks;
- **the harness's per-run timeout** (180s in the measurement controls) against the solver's cost.

## What a reviewer should check

1. the **phase order** and the dependencies between them;
2. the **provisional values** - they are the judgement, and they are the part most worth arguing about;
3. the **settings list** and the four places each has to land;
4. that **each phase is independently measurable**, and that phases 0-4 do not change the default path;
5. that the **record** (what each action was for) is enough to read a plan out of the key log.

## What is still under-specified

Nothing here blocks phase 1, because the provisional values stand in for the missing pieces. Still open, and worth settling before phase 6:

- the trip model's arithmetic (#35 item 3, #38 P2);
- the deficits' arithmetic, in the game's own numbers rather than booleans (#35 item 1, #38 P3);
- the opportunity value of a gold, the round cap and the tie-break (#40);
- the fraction of the whole gap or of the gap above the floor (#35 item 6);
- the catalogue of useful kinds, and the alternative comparison (#35 items 4 and 5);
- the endpoint for the arms, and how many arms the zoo can afford (#38 P14, #36).

## What a reviewer should check

(see the list above; the review and the verdict are on PR #42 and #41)
