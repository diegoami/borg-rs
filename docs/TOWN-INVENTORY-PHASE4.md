# Phase 4 readiness: the Stay rules as the plan implements them, and the plan record (P13)

Status: **draft, for owner acceptance** (2026-11-13). Epic borg-rs#36; the
phase is `docs/TOWN-INVENTORY-PLAN.md`, "Phase 4 - the plan". Where this
document and the epic's decision log disagree, the log wins and this
document is wrong.

Phase 4's entry criteria, and where each stands:

| criterion | state |
|---|---|
| #37's Stay rules | **drafted** in #37's table; not yet accepted. Part A below is the table as phase 4 would implement it, reconciled with the decisions taken after it (B3, P1). |
| the floors' infeasibility outcome | **decided** (B3, 2026-11-12): it falls back to #37's Stay rules - so it is settled once Part A is accepted. |
| the record schema (P13) | **partly done**: #46 froze the integer projection at version 2 (the state and today's decision). P13 also wants the plan's decision, what it displaced, and replay from the record's own inputs. Part B drafts version 4. |
| the sell-side suite, the shadow-plan mode | land *in* phase 4 (not entry criteria) |
| (from the phase-3 re-review) the solver's refusal and reading `optimal` | to decide *in* phase 4; Part C |

Accepting Parts A and B - as they stand or amended - is what starts phase 4.

## Part A - the Stay rules, as the plan implements them

#37's table rules every old town rule Stay, Go or Drop. Its Go rows become
usefulness terms and its Drop rows disappear; this part is about the
**Stay** rows only: where each lands in the plan, and where the decisions
taken after #37 change it.

### A.1 The emergency path

| #37 row (Stay) | in the plan | change from today |
|---|---|---|
| Cut or poisoned → the Alchemy shop (`borg_think_store.rs:153-157`) | the **Bleeding** and **Poison** floors, bought by the budget's floor pass (phase 2) before anything else | the floors are **kit predicates** (B3): "a cure for the current condition" and "a poison-cure capability". The capability is wanted even when **not** poisoned - broader than today, which acts only on the condition. The shop is not a rule any more: the floor pass takes the cheapest source that stocks the cure, which today is the Alchemist. No Use action (B3): the plan buys; quaffing stays the dungeon code's |
| No food, or no light at CL ≥ 2 → the General store (`:158-163`) | the **Food** and **Light** floors (≥ 1 each) | **the CL ≥ 2 condition goes**: P1 says at least one light, unconditionally. A CL1 character starts with a torch, so this should not bite; it is listed because it is a literal change |
| Instant shopping - the emergency jumps the step order (`:169-185`) | **floors first**: the plan cycle's `first(floors, chosen)`, ahead of any sale, store or purchase; and the floors **protect the kit**: a disposition (sell, store, drop) that would leave a floor unmet is not allowed, because the predicates are computed on the planned kit | the clause "light == 1 and gold ≥ 100 and CL < 10" is the lantern rule, which #37 rules **Go** (a term), so it leaves the emergency path |
| If the town has no healing left, try the house (`:186-194`) | **the Home is a floor source**: its wares join the floor pass at price 0 (the action is Take) | **phase 2's floor pass looks at the shops only** - this is a gap phase 4 must close, with a test |
| Give up after 2000 turns in town, or 1350 anti-bounces (`:141-145`) | **unchanged**: the plan sits behind the same guards in `borg_choose_shop` | the numbers are #37's open question 3 (A.4) |

### A.2 The buy decision

| #37 row (Stay) | in the plan |
|---|---|
| One empty slot, per **stack** (`borg_store_buy.rs:466-469`) | the solver's pack-slot limit (phase 3), counted per distinct stack (phase 2's S3 contract) |
| Two empty slots to wield (`:565-566`) | a wearable option's slot cost: the new item goes on, the old one into the pack, so one net pack slot unless the old item stacks. Phase 3 charges the ware's `slots_cost`; phase 4 makes it "the replaced item's slot", with a test (the reviewers' modelling note) |
| Only certain stores when in bad shape (`:490-497`) | **implemented by the order, not by a filter**: the floors are bought first, and the re-solve after them is no longer "in bad shape". Keeping the filter would block buying elsewhere once the floors are met |
| Cures for poison and cuts come from the Alchemy shop only (`:498-502`) | a fact about the stock, which the stock model sees; not a rule in the plan |
| Require sufficient cash (`:523-529`) | the solver's gold limit (phase 3) |
| The immediate-shopping special case (`:531-538`) | as the emergency path (A.1) |

### A.3 The veto, the sell side and the Home

| #37 row (Stay) | in the plan |
|---|---|
| Don't buy what you just sold to that shop (`borg_store_buy.rs:321-329`) | a **candidate filter** before `groups()`: a ware of the kind sold to that shop this visit is not a candidate (the Borg's `sold_item_*` lists already hold it) |
| Never sell worthless wares (`borg_store_sell.rs:853-859`) | a worthless item is never a **Sell** candidate; it may be a **Drop** |
| The shop has to buy that kind (`:862-865`) | a Sell candidate is offered only to shops that buy its kind |
| Never sell unidentified artifacts or random-power egos (`:928-941`) | excluded from **Sell** (they may still be kept, stored or dropped) |
| Don't sell what you just bought (`:944-953`) | excluded from **Sell** this visit (`bought_item_*`) |
| The Home is a stash, not a bank (`borg_store.rs:76-82`) | the design's premise: **Store** is free and frees a pack slot, bounded by the Home's slots (phase 2's budget); gold is never parked |

### A.4 #37's four open questions

| question | recommendation |
|---|---|
| 1. Where is the line between a floor and a need? | **Already decided** by P1 and B1 (2026-11-12): the floor is the kit predicate's minimum (one food, one light, one cure, the capability); everything above it is the target, priced by the usefulness function. Record it as decided |
| 2. The money-scum mode: dropped outright? | **Not by phase 4.** With `borg_town_plan` off nothing changes. With it on, the scum loop is not run and the plan's record says so - the mode's fate is decided when the default moves (P14), not now |
| 3. The guardrails (2000 turns, 1350 anti-bounces) | **Keep the numbers for phase 4** and make every visit that hits one an observable in the record (Part B, `guard`). Size them from the shadow-mode runs, not by argument |
| 4. Does "buy for the home" survive? | **Drop for phase 4**: the Home is the pack's overflow only. A depot for the next trip is a trip plan of its own (P11), and phase 4 is not the place to add one |

## Part B - the plan record (P13), version 4

P13 asks for a record that is "versioned, machine-readable, integers only,
stable field order, total ordering", that shows **what each action was for
and what it displaced**, and that a plan can be **replayed from its own
logged inputs**. Version 2 (frozen, #46) carries the state and today's
decision; version 3 (#47) is the handover's state record. Version 4 is the
plan's record: **version 2's fields, unchanged and in the same order, then
the plan's**. A v2 reader that stops at the v2 fields reads a v4 line.

One record per **plan step**: one solve and the action taken from it. In
shadow mode (the plan computed, not executed) the record is written next to
today's v2 decision record at the same state, and the two pair on `turn`.

### B.1 What v4 adds

| field | holds | why |
|---|---|---|
| `mode` | 0 shadow, 1 live | the two sides of a shadow run are told apart by it |
| `visit`, `step` | the town visit's number in the run, the step's number in the visit | a plan is a sequence; this is the sequence |
| `gturn` | the **game's** turn (v2's `turn` is the Borg's clock) | the clock the message log is on, as in v3 |
| `power` | `borg.power` | the power need reads it and it is not a trait; without it no record replays the power deficit |
| `params` | the twelve town settings, in their `NEW_SETTINGS` order | the arm: the weights, `k`, the trip model's two settings, `borg_town_plan` |
| `limits` | `[gold, pack_slots, home_slots, weight]` after the floors, and the raw `[pack_capacity, home_capacity]` | the solver's inputs that the budget computed |
| `floors` | `[[shop, slot, needs], ...]`, `needs` a bitmask in `Need` order; and `unmet`, a bitmask | what the floor pass bought, and what it could not |
| `tryon` | `[[shop, slot, value], ...]` for every wearable option | the only value the record cannot recompute: the try-on runs the engine's power code |
| `plan` | `[[key_kind, key, option, copies, cost, value, slots], ...]`, ranked | the chosen set, in the order the plan executes it |
| `solve` | `[value, bound, lambda, flags]`, flags a bitmask: optimal, gold_exact, slots_met, weight_met, weight_repaired | how good the set is known to be (phase 3's flags) |
| `action` | v2's codes, extended: 0 visit, 1 buy, 2 take_home, 3 sell, 4 sell_home, **5 store, 6 drop, 7 leave** | the action this step took |
| `act` | `[shop, slot_or_item, copies, price]` | its target |
| `for` | the need it served (0..7 in `Need` order), and `floor` 0/1 | **what it was for** |
| `displaced` | `[key_kind, key, option, value, cost]` of the best option the plan did **not** take, or `[-1]` | **what it displaced** - see B.2 |
| `guard` | 0 none, 1 the 2000-turn give-up, 2 the anti-bounce give-up, 3 the solver refused | A.4 question 3, and Part C |

`-1` for an absent value, as in v2. Integers only; field order as listed;
`seq` totally orders the file, as in v2.

### B.2 "What it displaced": two readings

P13 does not say which, and they record different things:

- **the runner-up** (proposed): the best option the solve left out for lack of gold, slots or weight. It is always defined, and it is computable from the record (replay the solve without the limit that bound). It answers "what did this purchase cost us".
- **today's choice**: the action the current policy would have taken at the same state. It answers "what did the plan change", but it is only known in shadow mode, where the v2 record next to it already carries it.

So v4 records the runner-up, and shadow mode gets today's side from the
paired v2 record rather than from a second copy inside v4.

### B.3 Replay

A v4 record replays when the stock, the budget, the groups and the solve
can be recomputed from it and give the same `plan` and `solve`:

- the stock and the budget come from v2's `pack`, `equip`, `home`, `shops`
  and `traits`, plus v4's `limits`;
- the usefulness comes from `traits`, `power` and `params`;
- the wearables' values come from `tryon`;
- the stacking predicate is not in the record: replay uses the record's
  stack classes, which v4 therefore adds to each ware of `shops` as a
  trailing element (a v2 reader ignores it). **This one is a change to
  v2's ware tuple**, which is why it is called out.

Phase 4's suite then gets its strongest test for free: **replay every
record a run writes and require the same plan**.

## Part C - to decide in phase 4 (from the phase-3 re-review)

Not entry criteria; recorded so that they are decided in the phase, not
discovered after it:

- **the solver's refusal**: past its packed score's fields it panics,
  which the release profile turns into an abort. Proposed: keep the abort
  (the game's inputs are far inside the fields) and record `guard = 3` if
  it is ever reached in a test build;
- **reading `optimal`**: the plan treats `optimal = false` as information,
  not as an error - it executes the first ranked action either way, and
  the record's `solve.flags` say why the set was not proved.

## What accepting this means

- **Part A accepted**: #37's Stay rows are the plan's floors, guards and
  exclusions exactly as tabled, the two literal changes (the light floor
  without CL ≥ 2; the poison capability when not poisoned) are intended,
  and the Home becomes a floor source.
- **Part B accepted**: version 4 is the plan record's shape, "displaced"
  means the runner-up, and the ware tuple gains its stack class.
- Either can be amended row by row; an amendment is a dated entry here and
  in the epic's log.
