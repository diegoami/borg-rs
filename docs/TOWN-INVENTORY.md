# The town inventory strategy: architecture

Status: design in progress. This document describes the shape of the redesign agreed so far and names the parts that still have to be defined before the first commit on `feature/town-inventory-strategy`.

The decisions this rests on are recorded in borg-rs#36 (the epic), #37 (the extraction of the old strategy) and #35 (the evaluation). Where this document and an issue disagree, the issue wins and this document is wrong.

## The shape

One town visit is one plan. The plan answers, for every item the character has and every ware the shops hold:

- **keep** it (carry it),
- **store** it (put it in the Home, which is free and frees a pack slot),
- **sell** it (for gold),
- **drop** it,

and then it chooses **what to buy**, subject to the same budget. The plan is re-solved after every action, because every action changes the state.

Three things separate this from the strategy it replaces:

1. **The floors come first.** Food > 0, light > 0, no cut, no poison. They are a constraint on the plan, never traded, and not priced - surviving is invaluable.
2. **The evaluation is a parameter.** Four axes - survival, power, knowledge, trip - combined by weights. One weight vector is one zoo arm; the mechanism is written once and the judgement is measured.
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

### 3. The evaluation - what it is worth

`borg_town_eval.rs` (new)

One function, four axes, one decay, one caution.

```rust
pub struct Weights {
    pub survival: i64,   // poison and wound coverage, food, light
    pub power: i64,      // what the item does for this character now
    pub knowledge: i64,  // a still-unaware useful kind
    pub trip: i64,       // what this completes in the coming trip
    pub caution: i64,    // the Black Market's share of the purse
}

pub fn utility(w: &Candidate, st: &BorgState, weights: &Weights) -> i64;
pub fn marginal(kind: u16, count: usize, st: &BorgState, weights: &Weights) -> i64;
```

- **the decay** - the k-th copy of kind K is worth `utility(K, n_K + k)`, decreasing in k. The current model has hard caps instead (`count_up`, `borg_power.rs:90-95`);
- **the power axis** - the existing try-on (`borg_store_buy.rs:457-696`) already computes the marginal power of a candidate; the axis reuses it;
- **the knowledge axis** - two routes, compared: buy an unidentified copy of the kind (the price of the item, and the purchase identifies every copy of that kind the character carries) against an Identify Rune (200 at base, one unknown per scroll, and **no shop sells one** - it is a find, or a 600-gold Black Market purchase);
- **the caution** - a shop 6 candidate has to clear a share cap on the purse (`cost <= purse / k`) on top of the ordinary utility-versus-cost test.

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
- the **record** is the key log, and it has to show a *plan*: for each action, what it was for and what it displaced. The current notes (`# Buying '<item>'(<letter>) at '<shop>'`) say what happened; the redesign adds why.

### 7. The configuration

- a setting that turns the plan on, **default off**, so `main` keeps its behaviour, its suites and its validation receipt while the redesign is built;
- the weight vector, and the parameters (`k` for the share cap, the decay shape), settable per arm.

## The parts that are to be defined

These are the specifications the redesign needs and does not have yet. Each is a decision, not a line of code.

| # | part | what is missing | decided by |
|---|---|---|---|
| P1 | **the floor set** | exactly which floors, and how each is computed: food > 0 and light > 0 are minima, but "enough food for the trip" is a need, and the line between them is nowhere in the current code | the owner |
| P2 | **the trip model** | the plan prices `trip_value`, so it needs to know where the character is going: the depth target, the expected turns, the expected fights. None of that exists today | the owner, then the implementation |
| P3 | **the axes** | the numerical meaning of `survival_value`, `power_value`, `knowledge_value` and `trip_value` - they are in different units, and how they are normalised before the weights combine them is undefined | the owner |
| P4 | **the decay** | its shape per kind (linear to zero, exponential, a cap with a taper) and whether it is uniform across kinds - a food, a potion and a wand do not decay alike | the owner; then the sweep |
| P5 | **the alternatives** | how the plan decides that two wares are alternatives: by kind, by effect, by what they do for this character. The same kind at 1x is obvious; a different potion with the same effect is not | the implementation, then the sweep |
| P6 | **the caution** | the exact shape of the share cap (`cost <= purse / k`, with `k` as a parameter) and whether it applies to every shop 6 candidate or only to the ones whose kind is otherwise available | the owner |
| P7 | **keep, store, sell, drop** | the thresholds: what makes an item worth storing rather than selling, what makes it worth keeping rather than storing. The Home's slots and the gold are the two currencies | the owner |
| P8 | **the unidentified item** | how the range of possible values of an unidentified item is priced, so that the plan can decide between selling it, keeping it, and buying a copy to learn the kind | the owner |
| P9 | **the identification policy** | when to buy an unidentified copy and when to spend an Identify Rune, and which kinds are worth learning at all - "all useful kinds" is a catalogue that has to be derived, not written | the implementation, from the catalogue |
| P10 | **the equipment** | whether the plan manages the worn kit - the try-on evaluates a candidate by swapping the equipment out, and the plan has to decide whether to keep that or to treat the worn kit as fixed | the implementation |
| P11 | **the Home** | overflow of the pack, or depot for the next trip - two different objectives - and what happens when the Home is full | the owner |
| P12 | **the routing** | whether the floors keep the fast route they have today (food and light from the General store, cures from the Alchemy shop only) or whether the plan routes them like any other purchase | the implementation, from the floors |
| P13 | **the observability** | exactly what the key log records - the plan, the displacement, the utility values, the solver's choice - and what the new suites assert. The sell side has no suite today at all | the owner, then the implementation |
| P14 | **the migration** | when the default moves, and what the receipt requires to be re-earned. The v11R receipt is 45/45 on the full suite and 105/105 runs; a new default voids it | the owner |

## What is not in the redesign

- the dungeon behaviour: the character still uses what it carries, and the decisions about what to *use* are not part of this;
- the C Borg: it stays the oracle for the old algorithm, and the Rust port's divergence from it is a decision to be taken separately (see the epic);
- the money-scum mode, unless the owner decides otherwise (see #37);
- the harness suites themselves: they are reworked, not rewritten, and the rework happens with the phases.

## Compatibility

- the plan is **off by default** on the branch; `main` is untouched and keeps its receipt;
- the two existing gates (`borg_buy_strategy`, `borg_buy_identify`) stay as they are until the plan replaces them, and the plan's arms are what the zoo measures;
- the suites that assert the current algorithm's shape - `family-buying`, `family-identify` - are reworked when the plan lands, not extended;
- the C files the port touches stay byte-identical unless the redesign needs to change a shared rule, in which case the receipt is re-earned.
