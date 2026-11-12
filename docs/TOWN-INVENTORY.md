# The town inventory strategy: architecture

Status: design in progress. This document describes the shape of the redesign agreed so far and names the parts that still have to be defined before the first commit on `feature/town-inventory-strategy`.

> **Superseded in part, 2026-11-12.** The owner decision entry in #36 (full record on PR #42) supersedes: the "decay comes for free" paragraph and P4 - the value curve is flat-then-capped, `U_K(n) = w_K · min(1, n·r/g)`, constant marginal value up to the cap, then zero; every "concave in n - the first copy is worth more than the second" phrasing (the curve is only weakly concave); "marginal ordering - exact when each kind's value decays" (the group DP is exact, the greedy is a heuristic, and P15's greedy instance is to be restated with flat values and caps, its second copy dearer through the 1x-then-3x alternative); the floor set ("not bleeding, not poisoned" becomes kit predicates - at least 1 food, at least 1 light, a cure for the current condition, a poison-cure capability - and a character may leave town with an unmet floor, per #37's Stay rules); and the alternatives (kind-only). The full amendments land with the phase-1 commit; until then the decision log is authoritative.

The decisions this rests on are recorded in borg-rs#36 (the epic), #37 (the extraction of the old strategy) and #35 (the evaluation). Where this document and an issue disagree, the epic's decision log (#36) wins and this document is wrong.

## The shape

One town visit is one plan. The plan answers, for every item the character has and every ware the shops hold:

- **keep** it (carry it),
- **store** it (put it in the Home, which is free and frees a pack slot),
- **sell** it (for gold),
- **drop** it,

and then it chooses **what to buy**, subject to the same budget. The plan is re-solved after every action, because every action changes the state.

Three things separate this from the strategy it replaces:

1. **The floors come first.** Food > 0, light > 0, no cut, no poison. They are a constraint on the plan, never traded, and not priced - surviving is invaluable.
2. **The usefulness function is a parameter.** Worth is not a property of an object: it is the fraction of a **deficit** the item closes. The needs are light, food, bleeding, poison, escape, knowledge, power and trip, each with its own deficit arithmetic, and the weights are the owner's priorities over them. One weight vector is one zoo arm; the mechanism is written once and the judgement is measured.
3. **Everything is one optimisation.** The greedy comparison, the two gates in front of it, the exclusion-based sell decision and the Home's local search all go. What replaces them is a candidate list, a budget, an evaluation and a solver.

## The components

Five new modules in `borg-rs/src/`, and the existing code they replace or reuse.

### 1. The stock - what exists

`borg_town_stock.rs` (new)

Turns the world into the plan's input: the wares of every shop, the pack, the Home, and the character.

- the shop model already exists and is refreshed wholesale: `borg_notice_store` (`borg_store.rs:159-199`) reads every store's live stock and stores each ware's price (`borg_price_item`, `:193`). 8 shops x 24 slots (`store:inven-max:24`), of which seven are purchasable;
- the pack and the Home become **groups of stacks**, not lists of items: the Borg already knows whether two items stack - `borg_object_similar` (`borg_store_sell.rs:180`) - and that predicate is what decides whether a candidate costs a slot;
- the **alternatives** are computed here too: the same kind at 1x in another shop is an alternative the plan compares against, which is what makes the Black Market's 3x price meaningful rather than arbitrary.

```rust
pub struct Candidate {
    pub shop: u8,        // 0..6; the Home is not a shop
    pub slot: u8,        // the slot in that shop's stock
    pub kind: u16,
    pub price: i32,      // what this shop charges (3x at the Black Market)
    pub slots_cost: i32, // 0 if it stacks into an occupied slot, else 1
    pub stack_count: i32,// how many of this kind are already carried
    pub utility: i64,    // filled in by the evaluation
}
```

### 2. The budget - what is feasible

`borg_town_budget.rs` (new)

Turns the state into the constraints the solver respects.

- **the floors** - computed first, from the character's needs, and never traded;
- **gold** - all the gold, minus what the floors cost. There is no trip reserve and no savings: the character arrives in the dungeon with an empty purse and a full plan;
- **pack slots** - 23 (`carry-cap:pack-size:23`), counted **per distinct stack**, not per copy;
- **home slots** - the overflow is bounded by the Home's slots, which the plan currently cannot see;
- **weight** - the carry allowance minus the carried weight; encumbrance is already priced into the power function (`borg_power.rs:1773-1824`), so this is a second, hard constraint on top of a soft one.

```rust
pub struct Budget {
    pub gold: i32,
    pub pack_slots: i32,
    pub home_slots: i32,
    pub weight: i32,
    pub floors: Vec<Candidate>,   // bought first, never traded
}
```

### 3. The usefulness function - what the character lacks

`borg_town_usefulness.rs` (new)

One function: the needs, their deficits, the weights, one caution.

```rust
pub enum Need { Light, Food, Bleeding, Poison, Escape, Knowledge, Power, Trip }

pub struct Weights {
    pub light: i64, pub food: i64, pub bleeding: i64, pub poison: i64,
    pub escape: i64, pub knowledge: i64, pub power: i64, pub trip: i64,
    pub caution: i64,    // the Black Market's share of the purse
}

pub fn usefulness(w: &Candidate, st: &BorgState, weights: &Weights) -> i64;
pub fn deficit(need: Need, st: &BorgState) -> i64;      // the target gap
pub fn floor(need: Need, st: &BorgState) -> i64;         // the minimum that keeps it alive
pub fn reduction(w: &Candidate, need: Need, st: &BorgState) -> i64;  // how much this closes
```

**The units.** The reductions are in different units - light in torches, food in rations, bleeding in wounds, power in power points - so they are normalised to **the fraction of the need closed** and the weight is what the *whole* need is worth. Each need is also split in two: the **floor** is the minimum that keeps the character alive (light at least 1, food at least 1, not bleeding, not poisoned) and is a constraint that is never traded; the **target** is what the coming trip needs, and the gap between what the character has and the target is what gets priced. That split is the line the old emergency path drew by accident, and it is what makes the weights comparable. **Decided 2026-11-11** (the alternatives - gold as the unit, or the natural unit with the weight as the conversion - are not used); see #35.

- **the deficit** - how much of each need is unmet, from the character's state: `required_light(depth) - BI_LIGHT`, `required_food(trip) - BI_FOOD`, the bleed (`BI_ISCUT`), the poison (`BI_ISPOISONED` and `BI_ACUREPOIS == 0`), `escapes_needed(ahead) - phase_doors_carried`, the still-unaware useful kinds, the marginal power, and the supplies the descent ahead needs. The raw material is already tracked (`borg_trait.rs`);
- **the reduction** - what the item closes. Each need defines its own arithmetic: light in torches, bleeding as a condition, power as a difference;
- **the decay comes for free** - the second copy is worth what is left of the deficit after the first, so the value falls by itself. The current model has hard caps instead (`count_up`, `borg_power.rs:90-95`), which are an ad-hoc guess at "enough for a trip";
- **the power need** - the existing try-on (`borg_store_buy.rs:457-696`) already computes the marginal power of a candidate; the need reuses it;
- **the knowledge need** - two routes, compared: buy an unidentified copy of the kind (the price of the item, and the purchase identifies every copy of that kind the character carries) against an Identify Rune (200 at base, one unknown per scroll, and **no shop sells one** - it is a find, or a 600-gold Black Market purchase);
- **the caution** - a shop 6 candidate has to clear a share cap on the purse (`cost <= purse / k`) on top of the ordinary usefulness-versus-cost test.

### 4. The solver - what to buy

`borg_town_solve.rs` (new)

Given the candidates and the budget, return the chosen set, ranked.

- **marginal ordering** - exact when each kind's value decays: list each kind's copies by decreasing marginal value and take them while the running cost fits. This is the natural fit for the decay, and it is what makes the second copy of a kind free in slots and costed only in gold;
- **knapsack** - exact for arbitrary separable evaluations, with gold, pack slots and weight as constraints. About 168 candidates against tens of thousands of gold is a few million cells: milliseconds, and irrelevant next to the Borg's one decision per turn.

The solver is re-run after every action, so it never has to be trusted with a plan it computed before the state changed.

### 5. The plan - what to do next

`borg_town_plan.rs` (new)

The orchestrator, and the only place that decides.

```rust
pub enum TownAction {
    Buy  { shop: u8, slot: u8 },
    Sell { shop: u8, item: usize },
    Store{ item: usize },
    Take { item: usize },
    Drop { item: usize },
}

pub fn next_action(st: &mut BorgState) -> Option<TownAction>;
```

The cycle:

```
loop:
    candidates = stock()                       # every ware, every stack, every alternative
    floors    = budget::floors(state)          # the needs, computed first
    budget    = budget::of(state, floors)      # gold, slots, weight - after the floors
    chosen    = solve(candidates, budget)
    action    = first(floors, chosen)          # the floors come first
    if action is none:  leave the town;  break
    record(action)                             # what it was for, and what it displaced
    execute(action)                            # the existing goal interface
```

`borg_think_store.rs` keeps its job - it is the caller - and the old step order (deposit, sell, buy, take, grab, stock, swap, swap) is replaced by this one.

### 6. The executor and the record - what already exists

- the executors are reused as they are: a decision names a `(shop, ware)` goal and the existing code presses the keys (`borg_think_shop_buy`, `borg_store_buy.rs:1388`; `borg_think_shop_sell`, `borg_store_sell.rs:1117`);
- the **record** is the key log, and it has to show a *plan*: for each action, what it was for and what it displaced. The current notes (`# Buying '<item>'(<letter>) at '<shop>'`) say what happened; the redesign adds why;
- the key log is also the **parity surface**. The receipt compares the C's and the Rust's complete key logs - `harness/diff` over `keys.log`, `messages.log` and `state.log`, driven by `measurement-receipt.py` (`m2-parity`, and `parity-<suite>` for captured-rogue, endgame, extended, lunal-investigation and synthetic-tactics) and by `measurement-test.py`, which asserts `log('c', name) == log('rust', name)` for every configuration. **Every sale and every purchase the Borg makes is pinned byte-for-byte against the C Borg.** Changing the town strategy therefore breaks parity by construction - which is the real cost of the redesign, and the reason the arms are Rust-only.

### 7. The configuration

- a setting that turns the plan on, **default off**, so `main` keeps its behaviour, its suites and its validation receipt while the redesign is built;
- the weight vector, and the parameters (`k` for the share cap, the decay shape), settable per arm.

## The optimisation problem

## The optimisation problem

The whole town visit reduces to **one program over a decision vector**, with the arms differing only in a vector of parameters. This is the specification the implementation follows.

### The state (given)

The character, its pack, the Home, and every shop's stock and prices - all already available: `borg_notice_store` (`borg_store.rs:159-199`) refreshes the shops, and the Borg's traits carry the rest.

### The variables

| variable | domain | what it decides |
|---|---|---|
| `q_w` | 0 .. the stack of that ware | how many copies of each ware to buy |
| `d_i` | keep, store, sell, drop | what to do with each item in the pack |
| `e_j` | keep, take, sell | what to do with each item in the Home |
| `n_K` | 0 .. n | how many copies of each kind the character ends up carrying |

### The objective

```
maximise   Σ_K U_K(n_K)  +  Σ_{equipment} U(w)
```

where `U_K` is the usefulness of carrying `n` copies of kind `K`, and `U_K` is **concave in `n`** - the first copy is worth more than the second, because the deficit it closes is smaller by then.

The objective is evaluated on the **final kit**: what the character carries into the dungeon, after the sales, the stores, the withdrawals and the purchases. That is what makes the dispositions and the purchases *one* problem rather than two - selling an item is not a decision about that item, it is a decision to convert it into something else.

### The constraints

| constraint | statement |
|---|---|
| **gold** | `Σ_w q_w · price(w) ≤ gold + Σ_i [d_i = sell]·price(i) + Σ_j [e_j = sell]·price(j)` |
| **pack slots** | `#{K : n_K > 0} + #{equipment} ≤ free_pack_slots` (23, counted per distinct stack) |
| **home slots** | the number of items stored in the Home ≤ the Home's slots |
| **weight** | the carried weight ≤ the carry allowance (encumbrance, already priced by `borg_power.rs:1773-1824`) |
| **floors** | `n_food ≥ 1`, `n_light ≥ 1`, not bleeding, not poisoned |

The gold constraint is where the two sides couple: the proceeds of a sale are budget for a purchase.

### The parameters (the arm)

| parameter | what it is | swept? |
|---|---|---|
| `w_light, w_food, w_bleeding, w_poison, w_escape, w_knowledge, w_power, w_trip` | the owner's priority per need; the weight is what the **whole** need is worth | **yes** |
| `k_share` | the Black Market's share cap: `cost ≤ purse / k` | **yes** |
| the trip model's parameters | how `required_food(trip)`, `required_light(depth)` and `escapes_needed(ahead)` are computed | **yes** |
| the floor levels | light ≥ 1, food ≥ 1, not bleeding, not poisoned | decided |
| the catalogue of useful kinds | which kinds the knowledge need counts | derived |
| the decay | the fall of `U_K` - it is the deficit closing, not a rule | derived |

**One arm is one parameter vector.** Everything else is derived from the state, so the experiment is clean: the same mechanism, the same seeds, the same suite, and a different point in the parameter space.

### The solver

The objective is **separable and concave per kind**, which is the structure that makes this tractable:

- **gold only** - a group knapsack with concave group values: for each kind, try every count from 0 up, and DP over the budget. `O(budget × kinds × copies)` ≈ 30,000 × 30 × 20 = 18M operations, milliseconds;
- **the floors** - solved first as a feasibility pass: the minimum purchase set that satisfies them, whose cost comes off the budget. That is the "floors first" rule, expressed as a lexicographic constraint rather than as a price;
- **the pack slots** - a second dimension, `#{K : n_K > 0} ≤ slots`. Adding it to the DP makes it `O(budget × slots × kinds × copies)` ≈ 400M, which is too slow for a decision per turn;
- **the weight** - a third dimension, and the same problem.

So there are two candidate solvers, and the choice is a build decision that the zoo can measure:

1. **exact** - the DP over the budget, with the floors first and the slot and weight constraints checked afterwards and repaired if they fail;
2. **near-exact** - the greedy over marginal gains (take the copies with the best marginal usefulness per gold, respecting the floors), then a repair pass that drops or swaps what breaks the slot or weight constraint.

The greedy is the one the old strategy approximates, except that the old one takes the single biggest gain and re-evaluates, which is neither of these.

### What the reduction does not settle

- **the trip model** - the target levels come from it, and it does not exist yet;
- **the joint treatment** - the program above is joint (the dispositions and the purchases share the gold constraint). Solving it in two stages - decide the dispositions, then buy with what they fetch - is simpler and loses the trades that only exist jointly, such as selling a nearly worthless item to buy the last torch. Whether the two-stage form is good enough is measurable;
- **the option value of an unidentified item** - a range rather than a number, so it does not fit the objective cleanly.

## Worked examples

### P15 - what the solver's form actually costs

The objective is separable and concave per kind, so for each kind the plan knows a table: `n` copies of kind `K` cost `n · price(K)` and are worth `U_K(n)`, with `U_K` concave.

**The gold-only problem is an exact DP.** With `dp[b]` = the best usefulness obtainable with gold `≤ b` from the kinds processed so far, and each kind as a group:

```
dp_new[b] = max over n in 0..copies_of(K) of  dp_old[b - n·price(K)] + U_K(n)
```

`O(budget × kinds × copies)` ≈ 30,000 × 30 × 20 = **18M operations - milliseconds**, and it is exact, because the groups do not interact.

**The pack-slot constraint is a different kind of constraint.** It counts *distinct kinds*, not copies, and the first copy of a kind costs a slot while the second does not. Adding it to the DP means a second dimension, `dp[b][k]` over the kinds used, with the transition tracking whether kind `K` was already started:

```
dp_new[b][k] = max( dp_old[b][k],                                   # take none of K
                    max over n>=1 of dp_old[b - n·price(K)][k-1] + U_K(n) )
```

`O(budget × slots × kinds × copies)` ≈ 30,000 × 23 × 30 × 20 = **414M - seconds, per re-solve**. The plan re-solves after every action, and the harness runs 165 cases per backend, so the exact DP is not affordable at one decision per turn.

Three ways to handle the slot and weight constraints:

1. **exact DP + repair** - solve the gold-only DP exactly, then drop or swap the kinds that break the slot and weight limits;
2. **Lagrangian relaxation on the kind count** - solve the gold-only DP with a penalty `λ` per kind, and sweep `λ` until the kind count fits. The gold-only DP is cheap, so a binary search over `λ` costs a handful of 18M passes. This is the standard treatment of a knapsack with a cardinality constraint, and it is usually exact or within one item;
3. **greedy over marginal gains + repair** - take the copies with the best marginal usefulness per gold, then repair. This is the cheap one, and it is *not* exact even for the gold-only case.

**What the greedy gives up.** With concave per-kind values, taking the best ratio first can be wrong: kind A gives 10 usefulness for 6 gold and another 10 for 10 gold; kind B gives 11 for 11 gold. The greedy takes A (10/6), then cannot afford B, and scores 10 - where the optimum is B alone, 11. So the greedy is a heuristic, not a bound; the exact DP is worth its 18M when the constraints allow it.

Which is why P15 is a **build decision the zoo can measure** rather than a preference: the exact form and the near-exact form produce different kits, and the difference shows up in the key log.

### P16 - what the two-stage form loses

The program above is joint because of one line: the proceeds of a sale are budget for a purchase. Deciding the dispositions first, without knowing what will be bought, throws that away.

A concrete case. The character carries an item worth **100** in use and sells for **50**, and has **20 gold**. The shop has one item worth **200** for **20 gold**, and another worth **150** for **20 gold**.

- **joint**: sell the carried item, so the purse is 70; buy both shop items; the kit is worth `0 + 200 + 150 = 350`;
- **two stages**: stage 1 sees use value 100 against a price of 50 and **keeps** it, because it is worth more than it sells for; stage 2 has 20 gold, buys one of the two; the kit is worth `100 + 200 = 300`.

**The two-stage solver never sells an item whose use value exceeds its price**, and that is exactly the "keep the valuable unidentified ware" veto the owner decided to remove. Only a joint program - or a first stage that knows what the money would buy - lets the plan trade a valuable item away when the gold buys more elsewhere.

So P16 is where that decision actually bites: with the veto removed in the extraction, the mechanism that honours it has to be the joint program, not a per-item rule.

## The parts that are to be defined

These are the specifications the redesign needs and does not have yet. Each is a decision, not a line of code.

| # | part | what is missing | decided by |
|---|---|---|---|
| P1 | **the floor set** | exactly which floors, and how each is computed: food > 0 and light > 0 are minima, but "enough food for the trip" is a need, and the line between them is nowhere in the current code | the owner |
| P2 | **the trip model** | the plan prices `trip_value`, so it needs to know where the character is going: the depth target, the expected turns, the expected fights. None of that exists today | the owner, then the implementation |
| P3 | **the needs and their deficits** | the arithmetic behind each deficit - `required_light(depth)`, `required_food(trip)`, the bleed rate, the poison damage rate. The **units are decided**: the fraction of the need closed, the weight the worth of the whole need, and every need split into a floor and a target | the owner |
| P4 | **the decay** | no longer a thing to specify: with a usefulness function the value of the k-th copy is what is left of the deficit after the first, so it falls by itself. What remains is whether the deficits are right - bleeding and poison are conditions today, not numbers | the owner |
| P5 | **the alternatives** | how the plan decides that two wares are alternatives: by kind, by effect, by what they do for this character. The same kind at 1x is obvious; a different potion with the same effect is not | the implementation, then the sweep |
| P6 | **the caution** | the exact shape of the share cap (`cost <= purse / k`, with `k` as a parameter) and whether it applies to every shop 6 candidate or only to the ones whose kind is otherwise available | the owner |
| P7 | **keep, store, sell, drop** | the thresholds: what makes an item worth storing rather than selling, what makes it worth keeping rather than storing. The Home's slots and the gold are the two currencies | the owner |
| P8 | **the unidentified item** | how the range of possible values of an unidentified item is priced, so that the plan can decide between selling it, keeping it, and buying a copy to learn the kind | the owner |
| P9 | **the identification policy** | when to buy an unidentified copy and when to spend an Identify Rune, and which kinds are worth learning at all - "all useful kinds" is a catalogue that has to be derived, not written | the implementation, from the catalogue |
| P10 | **the equipment** | whether the plan manages the worn kit - the try-on evaluates a candidate by swapping the equipment out, and the plan has to decide whether to keep that or to treat the worn kit as fixed | the implementation |
| P11 | **the Home** | overflow of the pack, or depot for the next trip - two different objectives - and what happens when the Home is full | the owner |
| P12 | **the routing** | whether the floors keep the fast route they have today (food and light from the General store, cures from the Alchemy shop only) or whether the plan routes them like any other purchase | the implementation, from the floors |
| P13 | **the observability** | exactly what the key log records - the plan, the displacement, the utility values - and what the new suites assert. There is no *content* assertion about selling anywhere: the family validators count purchase anchors (`# Buying Scrolls of Phase Door`, `# Buying Potions of Cure Light Wounds`) and nothing checks a sale. The sell behaviour is pinned only transitively, by the parity receipt's key-log comparison | the owner, then the implementation |
| P15 | **the solver's exact form** | the objective is separable and concave per kind, so the gold-only problem is an exact group knapsack (`O(budget × kinds × copies)` ≈ 18M) - but the pack-slot constraint adds a second dimension (≈ 400M, too slow per decision) and the weight a third. Whether the plan uses the exact DP with a repair pass, or the greedy over marginal gains with a repair pass, is a build decision the zoo can measure | the implementation |
| P16 | **the joint treatment** | whether the dispositions and the purchases are solved jointly (one program, sharing the gold constraint) or in two stages, and what the two-stage form loses - the trades that only exist jointly, such as selling a nearly worthless item to buy the last torch | the implementation, then the sweep |
| P14 | **the migration** | when the default moves, and what the receipt requires to be re-earned. The v11R receipt is 45/45 on the full suite and 105/105 runs; a new default voids it | the owner |

## What is not in the redesign

- the dungeon behaviour: the character still uses what it carries, and the decisions about what to *use* are not part of this;
- the C Borg: it stays the oracle for the old algorithm, and the Rust port's divergence from it is a decision to be taken separately (see the epic);
- the money-scum mode, unless the owner decides otherwise (see #37);
- the harness suites themselves: they are reworked, not rewritten, and the rework happens with the phases.

## Compatibility

- the plan is **off by default** on the branch; `main` is untouched and keeps its receipt;
- **the redesign voids the C/Rust town parity, and that is not a choice.** The receipt compares the complete key logs, so any change to what the Borg buys or sells diverges from the C Borg by construction. The C Borg stays the oracle for the old algorithm; the new town behaviour has no C counterpart, which is why the arms are Rust-only and the Rust default is their control;
- the two existing gates (`borg_buy_strategy`, `borg_buy_identify`) stay as they are until the plan replaces them, and the plan's arms are what the zoo measures;
- the suites that assert the current algorithm's shape - `family-buying`, `family-identify` - are reworked when the plan lands, not extended;
- the C files the port touches stay byte-identical unless the redesign needs to change a shared rule, in which case the receipt is re-earned.
