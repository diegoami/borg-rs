# The trip policy: one play mode, one dungeon trip and return policy

Tracking issue: [borg-rs#60](https://github.com/diegoami/borg-rs/issues/60).
Coordinates with the town plan ([borg-rs#36](https://github.com/diegoami/borg-rs/issues/36),
whose decision log wins where documents disagree) and disposes of the
pickup loop [borg-rs#59](https://github.com/diegoami/borg-rs/issues/59).
Branch: `feature/trip-policy`, from `feature/town-inventory-strategy` at `c62e239`.
Updated with town branch `b3f4285` by merge `ee54f13` (2026-11-14).

Status (2026-11-14): **the modes are removed; the policy seam exists and
records; its formula and parameters are a proposal, not accepted.** The
policy is off by default (`borg_trip_policy = 0`), and the default
configuration's key log is still byte-identical to the C oracle's.

## Contents

1. What changed, in one table
2. The removal inventory: what the modes reached, and what happened to each
3. Migration of old configurations
4. The C oracle's boundary
5. The current dive/return decision map
6. The replacement policy (proposal): inputs, units, priorities, parameters
7. The seam: deciding is not moving
8. Logging: `BORG_TRIP_RECORD` and the `# Trip:` note
9. Coordination with the town plan (#35/#36/#38/#39/#41)
10. Evaluation plan
11. #59's disposition
12. What needs the owner

## 1. What changed

| Before | After |
|---|---|
| Four entries into stair-scumming modes: `borg_munchkin_start`, `borg_munchkin_by_class` (Warriors, Druids, Rangers), `borg_lunal_mode`, `borg_self_lunal` | One play mode. The mode state (`lunal_mode`, `munchkin_mode`, `goal.fleeing_lunal`, `goal.fleeing_munchkin`), the two thinkers, the munchkin attacks and the lunal pickup selector are deleted |
| Ten mode settings | Retired: an old borg.txt that says "off" starts with a warning, one that would turn a mode on is refused, naming the setting (§3) |
| Buying strategy selected by class *and* by munchkin/non-munchkin | By class, then the global default |
| Dive/return decisions spread over `borg_think_dungeon`, `borg_leave_level`, `borg_flow_stairs` | The same code, plus one seam (`borg-rs/src/borg_trip.rs`) that states the decision with its reasons; the four copies of the "needing to sell" hold are one function |
| No record of why a character went back | `BORG_TRIP_RECORD=<file>`: one JSON line per decision change, with the inputs and the effective parameters |

## 2. The removal inventory

Every munchkin/lunal dependency found on the branch, and its disposition.

### Rust runtime (`borg-rs/src`)

| Where | What it did | Disposition |
|---|---|---|
| `borg_think_dungeon.rs`: `borg_think_dungeon_lunal`, `borg_think_dungeon_munchkin`, `munchkin_climb`, the activation blocks in `borg_think_dungeon` | the two stair-scumming thinkers and their entry | deleted |
| `borg_think_dungeon.rs`: `track_stairs_under`, `track_stair`, `stairs`, `stairs_mut`, `enum Stairs`, `inven_light` | helpers only the thinkers used | deleted; `nearest_stair` became `nearest_up_stair` (the short leash's only use) |
| `borg_attack_munchkin.rs` (+ header) | `borg_munchkin_mage`, `borg_munchkin_melee` | deleted |
| `borg_flow_take.rs`: `borg_flow_take_lunal`, `lunal_select`, `lunal_flow` | the "munchkin item" pickup selector - the #59 loop | deleted (§11) |
| `borg.rs` | `lunal_mode`, `munchkin_mode`, `goal.fleeing_lunal`, `goal.fleeing_munchkin` | deleted |
| `borg_caution.rs` | stairs taken while fleeing lunal/munchkin; leave-level spells skipped then; no Recall while fleeing munchkin | the conditions dropped (they were false) |
| `borg_escape.rs` | "Resetting recall depth during munchkin mode" | dropped |
| `borg_flow.rs`, `borg_flow_misc.rs`, `borg_flow_stairs.rs`, `borg_flow_kill.rs`, `borg_fight_attack.rs`, `borg_recover.rs`, `borg_magic_play.rs`, `borg_junk.rs`, `borg_update.rs` | flow avoidance, resting, sleeping-monster caution, spell testing, junk, autosave under the modes | the mode branches dropped; the ordinary branch is what ran with the modes off |
| `borg_think_dungeon_util.rs` | `borg_think_stair_scum` cleared the lunal pair on leaving | the clearing dropped; **the stair scum itself is kept** (below) |
| `borg_messages.rs` | "# Disconnecting Lunal Mode due to monster spell." on every teleport-to / teleport-away spell | the flag is gone; **the note is kept**: it is written whatever the mode, the C oracle writes it, and `messages.log` is compared byte for byte |
| `borg_trait.rs` (mushrooms) | under the munchkin start, three mushrooms were not counted until the munchkin level and six combat mushrooms counted in `BI_ASHROOM` after it | both rules dropped: no mushroom skipped, `BI_ASHROOM` stays 0 - what the start set FALSE gave |
| `borg_power.rs` | "do not carry these until later" under the munchkin start (resist potions, PFE, Recall, Phase, escapes, cures, speed) | dropped: the items are always valued, as with the start off |
| `borg_buy_strategy.rs` | the munchkin override and the class+munchkin masks | dropped; class masks then default |
| `borg_init.rs`, `borg_headers/borg_h.rs` | the ten settings, the class rule, the munchkin range checks, `lunal_mode` from the setting | retired (§3) |

**Retained as ordinary tactics, not modes** (audited, unchanged):

* `borg_think_stair_scum` - after the Borg flees a scary town monster down
  to level 1 (`goal.fleeing_to_town`), it grabs a near item and goes back
  up. A short escape-and-return, entered only from that flight.
* `borg_self_scum` - town store-scumming for a book or teleport scroll the
  Borg cannot otherwise buy (`borg_store_buy`), and the money-scum stop
  (`borg.rs`). Town only; no stair-scumming entry.
* `borg_money_scum_amount` - staying in town to scum gold. Town only.
* Fleeing, stair use under danger, Recall under threat (`borg_caution`),
  the short leash, "keep on a suitable level", power dives and the
  endgame crawl: unchanged.

### Configuration, launchers, tests, documents

| Where | Disposition |
|---|---|
| `harness/borg.txt`, `angband/src/borg/borg.txt` | the Lunal and Munchkin sections and the munchkin buying lines replaced by a "Removed modes" note; the trip section added; the dynamic formulas' `(!value(config, munchkin_start) or ...)` clauses removed (always true without the mode; with dynamic calculations off by default these do not run) |
| `harness/run.sh` | `munchkin`, `lunal`, `selflunal` flags **refused** with the reason; a mode setting has no line to rewrite, so `NAME=VALUE` refuses it too |
| `harness/suites/config`, `harness/suites/lunal-investigation` | removed (every case was a mode case) |
| `harness/suites/options`, `harness/suites/synthetic-tactics` | the mode cases removed (`opt-munchkin-limits-war`, `synth-lunal-*`) |
| `harness/suites/family-buying` | the warrior "selected" pair moved from the munchkin masks to the class masks |
| `harness/regress/family-buying-test.py`, `measurement-test.py`, `measurement-suites.py` | follow the suites; the effective-settings check now uses a trip setting |
| `harness/regress/measurement-receipt.py`, `measurement-remaining.py` | historical receipt checkers: marked historical (run from the receipt's checkout) |
| `harness/diff` | the C oracle boundary (§4) |
| `harness/native-checker` | the lunal thinker control retired (its README carries the dated entry) |
| `produce_death.py --munchkin`, `tools/borg_runtime.py` | refused with the reason |
| `tools/death_workflow.py` | keeps forcing `borg_munchkin_start = FALSE` on a *pre-removal* source; a no-op on later ones |
| `docs/ZOO-INTERFACE.md`, `docs/FAMILY.md`, `docs/harness.md` | dated 2026-11-14 entries; the old text stays as the record |

**Preserved as history, unchanged:** the tags, the adoption records and
their fixtures (`docs/adoption/`, `harness/adoption/`, including the M3
munchkin-stair and lunal-state fixtures), `harness/reference/` (the
`config`, `lunal-investigation`, `synth-lunal-*` and `opt-munchkin-*`
references), `docs/validation/`, `docs/c-bugs.md` and `docs/deaths/`. They
record what earlier trees did; reproducing them needs those trees.

## 3. Migration of old configurations

`borg_init::RETIRED_SETTINGS` lists the ten retired settings and the one
value each may still have:

| Setting | Accepted (inert) | Refused | Instead |
|---|---|---|---|
| `borg_munchkin_start` | FALSE | TRUE | - |
| `borg_munchkin_level` | any | - | - |
| `borg_munchkin_depth` | any | - | `borg_trip_depth_margin`, `borg_no_deeper` |
| `borg_lunal_mode` | FALSE | TRUE | - |
| `borg_self_lunal` | FALSE | TRUE | - |
| `borg_munchkin_by_class` | FALSE | TRUE | - |
| `borg_munchkin_class_depth` | any | - | - |
| `borg_buy_strategy_munchkin` | -1 | 0, 1 | `borg_buy_strategy_default` |
| `borg_buy_strategy_current_munchkin_classes` | 0 | any other | `borg_buy_strategy_current_classes` |
| `borg_buy_strategy_reserve410_munchkin_classes` | 0 | any other | `borg_buy_strategy_reserve410_classes` |

* **Accepted**: start-up goes on, with one warning per line
  (`** borg.txt: <name> is retired and ignored (borg-rs#60): <advice>`); the
  effective settings are as if the line were absent.
* **Refused**: start-up fails like any configuration error
  (`**STARTUP FAILURE** borg.txt: <name> = <value> would enable a removed
  mode (borg-rs#60): <advice>`). A later "off" line does not undo it.
* `harness/run.sh` refuses the `munchkin`/`lunal`/`selflunal` flags and
  `NAME=VALUE` for a retired name (it has no line to rewrite).

**The borg-zoo profiles need the owner's migration** (this branch does not
edit borg-zoo). `normal`, the `rs-buy-*` profiles and `town-negative`
(borg-zoo PR #83) pin `borg_munchkin_*` / lunal settings. At their "off"
values they will start with warnings, so the runs go on, but the warnings
are noise and the profiles describe settings that no longer exist. A
profile that turns a mode **on** - the `munchkin-class` profile that
`docs/ZOO-INTERFACE.md` names, if it pins `borg_munchkin_by_class = TRUE`,
and any `rs-buy-*` profile that sets a munchkin mask - will be **refused**.
(This branch did not read the borg-zoo profiles; the owner's check decides
which.) The proposed migration: delete the mode lines from every profile;
retire a profile whose purpose was a mode (its lineage stays as the
record); move any munchkin mask's bits to the class masks. The zoo
contract's rule 1 ("every profile names every mode flag") has no mode
flags left to name. The zoo's `config_error` rows for
the refused profiles are the actionable rejection working as intended.

## 4. The C oracle's boundary

The C Borg under `angband/src/borg/` is **not** changed in behaviour. It is
the oracle for the old algorithm and keeps the munchkin and lunal modes. Two
consequences:

* Its parser table keeps the ten retired settings; it gains the five
  `borg_trip_*` settings, registered and ignored like the town settings, so
  both backends read the same file. `the_c_table_matches_the_rust_table`
  now asserts: the C table is the Rust table plus exactly the retired
  entries, each at its inert default.
* Its `effective_settings` still lists the retired settings. `harness/diff`
  drops a retired setting that only one side reports **at its inert value**
  (`RETIRED_SETTINGS` in `harness/diff`); any other value means the C side
  ran a removed mode, and is reported as a difference.

**The parity covers the default play only.** With every mode off, the C
oracle and the Rust Borg produce the same streams (§10 has the receipts).
The C oracle does not implement the trip policy, and with
`borg_trip_policy = 1` the Rust Borg is a family variant with no C
counterpart. A run that enables a mode in C has no Rust counterpart either.

## 5. The current dive/return decision map

Survival is first in the turn order: `borg_caution` (`borg_caution.rs`) runs
before any of this and may flee the level, take stairs, or read Recall
(`# Recalling to town`) whatever the trip wants. The rest, in the order the
Borg reaches it each turn in the dungeon (line numbers at this branch's
tip):

### `borg_think_dungeon` (`borg_think_dungeon.rs`)

| Where | Condition | Effect |
|---|---|---|
| :619 | 10,000 turns on the level | leave and flee ("boredom") |
| :641 | depth 1 after fleeing town (`fleeing_to_town`) | stair-scum near items, then leave ("finish shopping") |
| `borg_check_bounciness` | anti-bounce counter >= 700 | leave and flee |
| `borg_check_breeders` | breeders >= min(clevel+2, 5), no Recall or clevel < 35 | ignore breeders after 2,500 turns; leave and flee ("no recall") |
| :667 short leash | up stairs known, (max HP < 30 or clevel < 15), depth >= clevel - 5 | return to the up stair beyond `3*clevel + 14` grids |
| :702 suitable level | clevel < 10, not prepared for the depth | head up (`goal.less`), take `<` if on it |
| :749 bad shape | no light, cut, poisoned or no food | refuel, wear, recover, crush, shop |
| :787 | `borg_must_return_to_town` (after 100 turns on a level: `borg_restock`) | `borg_leave_level` |
| :1163 leave | `goal.leaving` (no recall pending, no unique) **or the sale hold** | up stairs unless ready for Morgoth; down only if prepared for depth+1 and not the sale hold |
| :1201 power dive | prepared for depth+5, not heading up | down stairs, unless the sale hold |
| :1310, :1364 | after exploring (and money scum not wanted) | `borg_leave_level(bored)` |
| :1555 | twitchy, nothing else to do | Recall to town |

The **sale hold** (`borg_trip::legacy_sale_hold`; four copies before this
branch, in `borg_think_dungeon` and `borg_flow_stairs`): in the dungeon,
clevel < 25, gold < 25,000 and `borg_count_sell >= 13` - do not go down, go
up.

### `borg_leave_level` (`borg_think_dungeon_util.rs`)

Computes a direction `g` (-1 up, 0 stay, +1 down) and the `goal.rising`
flag (go to town), then executes:

| Line | Condition | Effect |
|---|---|---|
| :772 | not prepared for the current depth | g = -1 |
| :783 | `borg_count_sell >= 12` | try not to descend |
| :788 | experience drained | try not to descend |
| :795 | bored, not prepared for depth+1 | g = -1 |
| :800 | bored and spastic | -1 if not prepared for depth+1, else +1 |
| :814 | prepared for depth+5, sellable < 13 | g = +1 ("power dive, playing too shallow") |
| :822 | prepared for depth+1, depth 75..99 | g = +1 |
| :834 | not prepared here, no unique: depth > 65 and not prepared for half the max depth | rising ("too deep") |
| :848 | `borg_must_return_to_town` | rising, restock |
| :854 | the reason is "Collect from house" | rising, restock |
| :867 | prepared for depth+20 and 60% of max depth, max depth > depth+20, Recall >= 3 or gold > 2000 | rising ("too shallow": recall back down) |
| :878 | (worships gold or max clevel < 15), max clevel <= 25, sellable >= 12 | rising ("Sell Stuff, Worshipping Gold") |
| :884 | bored, max clevel >= 26, sellable >= 12 | rising ("Sell Stuff") |
| :890, :896 | level drained / experience drained (bored) | rising |
| :909 | bored, no vault, no unique near, 8,000 turns since town | rising ("I miss my home") |
| :919 | max depth >= depth+10, depth <= 12, 3,500 turns since town | rising ("scumming check") |
| :928 | big-fight prep with 3+ *Healing* or a Life potion | rising (drop off potions) |
| :938-:947 | Sauron dead at 99, Morgoth dead, prepared for 99 | endgame overrides |
| :955 | depth > `borg_no_deeper` | g = -1 |
| :964 | clevel > 10, depth 1, < 200 turns | stay to rotate shops |
| :981 | too long on the level (`borg_time_to_stay_on_level`, by level feeling) | -1 if trying not to descend, else a coin flip |
| :993 | **the trip policy, acting** (§7) | rising / g |
| :1027-:1052 | g < 0 | Recall if rising and depth >= 5 (and 200 turns since town), or restocking; else the up stairs; Recall after 1,000 turns with no stairs; with no up stairs known, go down |
| g > 0 | | the down stairs |

### Out of town (`leave_town`)

Recall down when max depth >= 8 (10 when worshipping gold), **Recall >= 3**,
and prepared for 60% of the max depth; otherwise walk down.

### What the map shows

* **No planned depth.** The depth falls out of `borg_prepared` (the class's
  and level's readiness tests) one level at a time, plus a power dive when
  prepared five levels deeper.
* **Pack pressure counts sellable items, not slots.** `borg_count_sell`
  (twelve or thirteen "worth selling" items, by a greed threshold) stands
  in for "the pack is full"; a pack full of things it keeps does not send
  it back, and it does not count stacks.
* **Unidentified items do not send it back at all**, except as part of the
  sellable count (a valuable unidentified item is not sellable unless
  stacked). The owner's level-300 case ("plenty of unidentified objects,
  I would have come back much earlier") is outside the map.
* **Gold enters only as a cap** (the sale hold and "Sell Stuff" stop at
  25,000 gold or clevel 25) and as the "too shallow" Recall allowance.
* The same conditions recur with different thresholds (12 vs 13 sellables,
  clevel 15/25/26) and in different places (four copies of the sale hold).

## 6. The replacement policy (proposal)

**Nothing here is accepted.** It is the concrete form the issue asks for, to
be fitted and confirmed (§10) before it can become the default.

### Inputs (`borg_trip::TripInputs`, player-known only)

| Input | Unit | Source |
|---|---|---|
| `clevel`, `max_clevel` | levels | traits |
| `class`, `race` | indices | recorded; not read by the decision yet (class acts through preparation; a race term needs evidence) |
| `depth`, `max_depth` | dungeon levels | traits |
| `prepared_depth` | dungeon levels | the deepest level `borg_prepared` accepts, scanning down from 1 (side effects restored) |
| `prepared_here` | bool | `borg_prepared(depth)` |
| `restock` | bool | `borg_must_return_to_town` |
| `gold` | gold | `BI_GOLD` |
| `sale_value` | gold | sum of the Borg's own value of each carried stack x quantity: an estimate from what it knows; the shops pay less |
| `sellable` | items | `borg_count_sell`, as `borg_leave_level` counted it (recorded) |
| `pack_capacity`, `pack_used` | pack slots, **per distinct stack** | `borg_town_budget::Facts`, `borg_town_stock::pack_slots_used_of` - the town plan's count |
| `unknown_stacks` | stacks | carried stacks with an unaware flavour or unknown runes (`borg_item_note_needs_id`) |
| `recall` | scrolls | `BI_RECALL` |
| `up_stairs_known`, `down_stairs_known` | bool | the stair lists |

### Parameters (`borg_trip_*`, the six places)

| Setting | Unit | Default | Meaning |
|---|---|---|---|
| `borg_trip_policy` | 0/1 | 0 | 0: today's behaviour (the policy only records); 1: it acts |
| `borg_trip_depth_margin` | levels | 0 | the **depth/return factor**: plan this many levels above `prepared_depth` |
| `borg_trip_pack_slack` | pack slots | 2 | return when free slots <= this |
| `borg_trip_unknown_stacks` | stacks | 4 | return when unknown stacks >= this (0 = off) |
| `borg_trip_return_gold` | gold per clevel | 0 | return when `gold + sale_value >= this x clevel` (0 = off) |

The defaults of 2, 4 and 0 are placeholders chosen to be plausible, not
fitted.

### The decision (`borg_trip::decide`)

`planned depth = clamp(prepared_depth - depth_margin, 1, 127)`.

Priorities, highest first:

0. **Survival** - not in the policy. `borg_caution` runs first every turn,
   flees, escapes, and reads Recall under threat; nothing here delays it.
1. **Restock** - a crucial supply is out: **return**.
2. **Unprepared** - not ready for this level: **ascend** one level (the
   return reasons below are still recorded).
3. **The value of going back** - **return**, with every one that fired:
   * pack pressure: free slots (per distinct stack) <= `pack_slack`;
   * knowledge: unknown stacks >= `unknown_stacks` - town can identify
     them (buying a copy identifies the kind; the Identify Rune; sell and
     rebuy), sell what is junk, and free the slots;
   * purse: gold + estimated sale >= `return_gold x clevel` - what the
     character could usefully spend scales with its level.
4. **The planned depth** - **descend** above it, **continue** at it,
   **ascend** below it.

The decision carries a **route**: `recall` (a return from depth >= 5 with
a scroll), `stairs` (a known staircase), or `search` (no route known: the
decision stands, and the executor explores until it sees one). A return
that cannot be executed yet is not a different decision; it is recorded as
`search` until a route appears, and the decision is asked again.

**Why these factors** (owner's list in #60): level and class through
`prepared_depth` (the preparation tests are by class and level); gold and
spending opportunity through the purse term; sale inventory through
`sale_value`; pack pressure through slots per stack; unidentified items
through `unknown_stacks`. Race is recorded only.

**First observation (2026-11-14, one seed, not evidence):** with
`BORG_TRIP_RECORD` on seed 101 (Human Warrior, 20,000 turns, policy off),
the policy would have returned at clevel 1 on level 1 for `unknown(4 >= 4)`:
early in the game nearly every potion and scroll flavour is unknown, so a
flat stack count fires on routine finds. With `borg_trip_policy = 1` the same
seed ends at max depth 1 instead of 2 (clevel 3 either way). The knowledge
reason needs weighting - by the depth the item came from, by tval, or
against what the town can identify for the purse - before it is fitted; it
is the first factor the evaluation should look at.

**Not yet:** a value for the unknowns (by depth found, by tval); the town
plan's own estimate of what the purse would buy (§9); trip duration. These
are the next factors to consider once runs show they matter.

## 7. The seam: deciding is not moving

`borg-rs/src/borg_trip.rs`:

* `decide(&TripInputs, &TripParams) -> Decision` - pure; the unit tests
  cover level/class (through preparation), the margin, gold and the purse,
  sale inventory, unidentified burden (including the owner's case:
  returning earlier with unidentified items), pack pressure counted per
  stack, unprepared vs restock, every reason kept, unavailable routes, and
  town;
* `observe(st, sellable) -> TripInputs` - reads the state, restoring the
  two side effects of `borg_prepared` (`ready_morgoth`, the Home notice);
* `consult(st, sellable) -> Option<Decision>` - called from
  `borg_leave_level` in the dungeon; records; returns the decision only
  when `borg_trip_policy = 1`;
* `legacy_sale_hold(st)` - the one copy of the sale hold, in the C
  evaluation order (so `borg_count_sell` runs exactly when it did); off
  when the policy acts.

Executing is unchanged: `borg_leave_level` turns the decision into its own
`g`/`goal.rising` (return: rising and up; ascend: up; continue: no
descent - a bored Borg at the planned depth goes up a level rather than
past it), and walks or recalls as before. With the policy acting, the two
"Sell Stuff" rising rules and the sale hold give way to the policy's pack
and purse reasons; every other rule of the map stands.

## 8. Logging

* `BORG_TRIP_RECORD=<file>`: one JSON line (version 1) each time the
  decision, the level or the reasons change: the turn, the effective
  parameters, every input and the decision with its route, planned depth
  and reasons. With the policy off it **observes only**: it presses no key
  and writes no message, and `borg_prepared`'s side effects are restored,
  so the play is the same with and without it. Example:

  ```json
  {"v":1,"turn":1234,"params":{"policy":0,"depth_margin":0,"pack_slack":2,"unknown_stacks":4,"return_gold":0},"inputs":{"clevel":10,...,"unknown_stacks":5,"recall":1,...},"decision":{"action":"return","route":"recall","target":8,"reasons":["unknown(5 >= 4)"]}}
  ```
* With the policy acting, the same change also writes a note in the
  message log: `# Trip: return via recall (plan 8): unknown(5 >= 4)`.

Owner feedback on a case should quote the record line; it names the input
that decided and the threshold it was held to.

## 9. Coordination with the town plan

**Owner decision (2026-11-14, resumed local session): leave shared
`PlannedTrip` integration to the buying session.** This session updates
and validates the trip branch; it does not wire the town-side model.

1. **One trip model.** The policy owns the planned trip:
   `borg_trip::plan_trip -> PlannedTrip { depth_target, levels,
   recall_scrolls, recall_carry }`. Proposed read by the town plan:
   `Trip::of` takes `depth_target` from `plan_trip` instead of
   `BI_MAXDEPTH + 5`, and `levels_per_trip` from `levels`; turns per level
   stay the town's until trip duration is measured. Not wired yet: the
   town side is another session's, and the change moves the town plan's
   shadow records.
2. **One inventory valuation.** Pack pressure uses the town side's slot
   count (one per distinct stack, `pack_slots_used_of` in
   `borg_town_stock.rs`, the capacity from `Facts`). The dungeon and the
   town agree when a pack is full.
3. **Word of Recall.** A trip reads one scroll down when the recall depth
   is >= 5 and one back when the target is: `recall_scrolls`. It carries
   one spare: `recall_carry` = reads + 1 (3 for the usual round trip, which
   is what `leave_town` already demands before recalling down). The town
   plan's trip need would price `recall_carry - BI_RECALL` under this
   proposal. **Town-side owner decision, merged 2026-11-14:** the town
   model now prices `RECALL_PER_TRIP = 2` (one return, one spare) against
   `BI_RECALL`, which also counts the rod and spell. It does not read
   `PlannedTrip` yet. Reconcile the two-scroll town target with the
   three-scroll round-trip proposal before wiring them together; the
   proposal does not supersede the town owner's decision.
4. **More frequent returns cost town solves.** Each return is a town visit
   (~0.2-1 s per cold solve, `docs/TOWN-INVENTORY-PLAN.md`). The evaluation
   counts visits (§10); the warm start on the town side becomes more
   urgent if the fitted policy returns more often.
5. **The six places.** The five `borg_trip_*` settings are in all six
   (`borg.h`, `borg-init.c`, `borg_h.rs`/`borg_init.rs`, both `borg.txt`,
   `NEW_SETTINGS`); the ten retired ones are out of the Rust four and stay
   in the C two (§4).

The earlier handover's town calibration questions have moved on. The
merged town model now values five wound potions per trip even while uncut;
the recorded town-side shadow runs report no Recall or Cure Light Wounds
sales. Under no-selling rules, worthless Home stacks stay unless their
slots are needed. See `TOWN-INVENTORY-PLAN.md`, phase 4 slice 3, for the
owner decisions and their evidence. These are town-side results, not a
validation of the trip-policy formula or of a shared trip model.

## 10. Evaluation plan

Bounded; nothing is tuned on this branch.

* **Arms:** the Rust default (`borg_trip_policy = 0`) as control; the
  policy at the provisional values; then one parameter at a time
  (`unknown_stacks`: 2, 4, 8; `pack_slack`: 1, 2, 4; `return_gold`: 0, 100,
  500; `depth_margin`: 0, 2, 5) - at most a dozen arms, within the zoo's
  budget.
* **Endpoints, prespecified per comparison:** survival (deaths per 10^6
  game turns), progress (max clevel, max depth at fixed turns), trip
  duration (turns between town visits), useful town visits (visits with at
  least one sale, purchase or identification - the town record says),
  inventory pressure at return (free slots, unknown stacks), and stalls.
* **Fitting and confirmation are separate:** fit on one seed set, confirm
  on a disjoint one; the owner's level-300 save is a case to explain with
  the record, not a fitting target.
* **Factors added only from evidence:** a factor enters the policy when a
  record shows a return (or a missing one) it would have changed.
* **Receipts on every change:** the default-config parity (smoke 3/3,
  seed 101 hash) must hold as long as the policy is off by default.

## 11. #59's disposition

The loop was in `lunal_select`: in munchkin mode it skipped the "bad item"
rejection, so a full-pack character kept targeting an uncarryable Scroll of
Blessing. **The path is removed**: `borg_flow_take_lunal` and its selector
are deleted, and nothing enters munchkin mode. The ordinary selector
(`take_select`) targets nothing at all unless the pack has an empty slot
("Require one empty slot"), so a full pack cannot loop on a floor item
through it. What is not done: a
regression replay of the #59 save on this branch (the save is in the
owner's local bundle, not in the repository). Proposed disposition: close
#59 as fixed by removal once the owner replays the bundle's command on this
branch without the (now refused) `borg_munchkin_by_class=TRUE` flag and
confirms the ordinary play does not loop; and keep the C-side gate
(`borg-flow-take.c:842`) recorded as a C oracle bug in the mode it belongs
to.

## 12. What needs the owner

1. **The policy's formula and parameters** (§6): accept, change or reject
   the priorities, the three return reasons and the planned depth; then the
   evaluation plan (§10) before any default changes.
2. **The zoo migration** (§3): delete the mode lines from `normal`,
   `rs-buy-*` and `town-negative` (borg-zoo PR #83); retire
   `munchkin-class`.
3. **#59** (§11): the replay and the close.
4. **The town plan's read of `PlannedTrip`** (§9): assigned by the owner
   to the buying session on 2026-11-14; timing and Recall reconciliation
   remain with that session.
5. **The parity receipt:** the settings corpus changed (ten settings out of
   the Rust Borg, five in both); the full suite receipt is to be re-earned
   on the owner's schedule. **Re-earned after the town merge on
   2026-11-14:** smoke 3/3, full 45/45, deep 9/9, options 24/24 and
   remaining synthetic tactics 3/3; family buying 8/8 and the seed-101 hash
   also pass. See [the merge validation receipt](validation/2026-11-14-trip-policy/README.md).
