# Live trip and inventory integration: technical evidence

Owner direction (2026-11-14): merge the inventory branch into the trip
branch and try settings for both policies in actual play. This supersedes
the earlier assignment of integration to a separate buying session.
Slower progress and earlier returns are intended; descent speed is not a
selection criterion.

The integration includes town utility revision `785f506`, merged at
`d50fe0d`, with shared settings reconciled by name. The Rust/C settings
tables keep their intentional difference for retired modes. The town
parameter vector stays contiguous and its report derives indices by name;
the old hardcoded indices would have become invalid after mode removal.

## Running the actual policies

```bash
BORG_IMPL=rust bash harness/run.sh /tmp/joint-example 101 0 0 100000 \
  borg_trip_policy=1,borg_town_plan=1
```

- `borg_trip_policy=1`: the experimental dungeon return/depth decision acts.
- `borg_town_plan=1`: the utility plan chooses actual town actions. It does
  not require a recording environment variable.
- Both zero: the historical control. With the inventory plan zero,
  `BORG_TOWN_PLAN_RECORD` remains an optional observation comparison.
- `borg_town_no_stores=TRUE` still forbids all store/Home strategy actions.
- Manual shop overrides still take precedence over automatic destinations.

Live execution buys, sells, stores or withdraws one copy, then recomputes.
The solver's full kit is a forecast, not a cached shopping script. It
recomputes on entering a shop so stock and inventory indices are fresh.
Home sales first withdraw from Home without marking the item as a new
keeper, then the next solve sees the carried item eligible for sale.
Drops execute outside the store UI. Pack drops from the old town-inventory
heuristics give way to the new dispositions while live mode is enabled;
dungeon junk handling remains the existing code.

An impossible Home withdrawal without pack room is rejected explicitly.
The existing town-time and antibounce limits remain. These are execution
guards, not claims that every proposed plan is useful.

## Shared trip assumption

With the trip policy acting and `borg_town_target_depth=0`, the inventory
evaluator reads the trip policy's planned target and levels. The v5 utility
model supports target depths 1..100, so its effective target is clamped to
that range. An explicit town target remains an explicit forecast override.
With the trip policy off, the inventory model retains its own frontier
forecast (`max_depth + depth_gain`).

The record's `params` contain the effective model inputs so its solve can
replay without the engine. `configured_params` preserve what the user
actually configured, including a zero automatic target. `mode=1` marks a
live selection; native game messages and inventory changes establish actual
execution. A selection record alone is not a completed transaction.

Recall is still valued by v5 marginal travel utility. The older proposed
`PlannedTrip.recall_carry` value does not impose a fixed buying quota. The
town's duration/risk/food/light settings are explicit planning assumptions;
they do not order the character to remain underground for that duration.

## Bounded exploratory combinations

`tools/joint_policy_trials.py` prints the full settings by default and runs
them only with `--run`. It refuses an existing evidence directory, limits
each run to at most 100,000 turns and uses at most four workers. Each child
also has a 50,000-key bound and a 180-second process bound. A key/time stop
is an incomplete comparison, not a successful turn-bounded observation.

| Arm | Trip choices | Inventory choices |
|---|---|---|
| old-control | off | historical chooser |
| integrated-reference | margin 0, 2 free slots, 4 unknown gear stacks | utility defaults |
| early-return | margin 2, 4 free slots, 2 unknown gear stacks | utility defaults |
| cash-retaining | reference trip | gold weight 50 rather than 25 |
| learning-priority | return at 1 unknown gear stack | knowledge weight 50 rather than 25 |
| supply-cautious | margin 2, 3 free slots, 2 unknown gear stacks | 5,000-action forecast, risk 1,500 rather than 1,000 |

Cases: Human Warrior 101, Mage 301 and Priest 401. These previously observed
seeds are exploratory cases, not independent confirmation seeds. Some arms
change multiple settings: comparisons describe whole combinations, not the
causal contribution of an individual parameter. No arm automatically becomes
an accepted default.

The reference and cash-retaining arms were subsequently checked on disjoint
Warrior 202, Mage 302 and Priest 402 cases:

```bash
python3 tools/joint_policy_trials.py --out /tmp/joint-holdout --run \
  --arm integrated-reference --arm cash-retaining --case-set holdout
```

```bash
python3 tools/joint_policy_trials.py --out /tmp/joint-trials
python3 tools/joint_policy_trials.py --out /tmp/joint-trials --run
```

The report records native-confirmed transactions, town inventory/knowledge
changes, supply capability at observed town states, unresolved gear across
visits and any deaths/stops. Return counts use the documented final-depth-
per-native-tick convention. It makes no automatic "useful" classification
from a count reduction, no descent-speed score, and no survival claim from
these few runs. A reduction in unknowns can also mean use, disposal or
equipping. Further confirmation must use a disjoint seed set.

## Validation state

The final candidate passes 254 release tests (four ignored), clippy, six
Python outcome-report tests, C/Rust smoke 3/3 and full 45/45, and the no-C-Borg
audit (154 engine objects byte-identical). All 4,051 records across 28 plan
files replay their plan, solve, dispositions and runner-up. A live
20,000-turn run with no record files made 21 native-confirmed purchases and
matches its logged counterpart on keys, messages and state. The live
no-stores canary made no purchases and emitted no executor actions.

## Results and plausible choices

The final comparison contains 18 exploratory runs and six holdout runs.
Every final comparison stopped on its turn bound or natural death; none
hit a key/time bound, and no completed live trial logged an unaffordable
purchase or a merchant refusal. Native-confirmed purchases, sales, Home
deposits/withdrawals and drops occurred. The source-linked numbers and
complete stop/death outcomes are in
[joint-live-results.json](joint-live-results.json).

First town departures, ordered Warrior/Mage/Priest on seeds 101/301/401:

| Configuration | Gold retained | Carried food equivalents | Known healing potions |
|---|---|---|---|
| integrated reference | 41 / 63 / 23 | 2 / 3 / 1 | 2 / 3 / 2 |
| earlier returns | 41 / 63 / 23 | 2 / 3 / 1 | 2 / 3 / 2 |
| gold weight 50 | 61 / 133 / 43 | 2 / 3 / 1 | 1 / 3 / 2 |
| learning weight 50, gear threshold 1 | 31 / 44 / 23 | 2 / 2 / 1 | 1 / 2 / 2 |
| 5,000-action forecast and risk 1,500 | 10 / 30 / 11 | 5 / 8 / 5 | 4 / 9 / 3 |

The early-return profile changes the journey after the initial shopping:
Warrior 101 has 29 native-clock returns versus 22 for the reference. Those
counts do not say every return was useful or caused by this seam; other
cases do not establish a general return-rate improvement.

These are the useful distinctions for an owner choice:

- **Earlier returns with the ordinary inventory balance:** margin 2,
  pack slack 4, unknown gear threshold 2, gold/knowledge weights 25. This
  is the tested early-return arm, not a claim of an optimal threshold.
- **More cash retention has a real stock cost:** gold weight 50 leaves
  more gold but the Warrior carries one healing potion instead of two.
  The holdout shows that same tradeoff: Warrior gold 46 to 66, healing
  potions two to one. It is not a free improvement in budgeting.
- **Learning competes with readiness:** with knowledge set to 50, these
  departures carry fewer healing potions and,
  for the Mage, one fewer ration. It is an explicit preference to inspect,
  not automatically a more cautious inventory.
- **Long-trip provisioning is a different assumption:** 5,000 actions
  buys substantially more food/healing and leaves less cash. Choose it
  when the intended trip is longer; the experimental name "supply-cautious"
  does not establish that it is safer or suitable for frequent short returns.

Several live cases died. The exploratory cash-retaining arm reached the
bound in all three cases, but two cash-retaining holdout cases died. There
is no supported survival winner and no automatic default selection. No
descent-speed metric is used to choose a configuration. Trial return/unknown
counts describe observations, not independent lives, guaranteed upgrades or
proof of identification. The report now includes Home knowledge so simply
storing unknown gear is not mistaken for learning it.

The shallow comparisons do not fit the gear-count threshold. A separate
functional replay of the retained depth-24 Warrior uses pack slack zero
and unknown threshold one: its first vote is solely
`unknown-gear(1 >= 1)`, via Recall, with one free policy slot. It reaches
town, and the unknown staff is protected by the live model. Shopping is
still incomplete at the 1,500-key bound; it does not prove the staff's
knowledge was resolved. This historical cheat-death checkpoint is not an
independent natural life or another parameter-selection case.

## Execution defects exposed before the final comparison

Two earlier batches are diagnostic evidence, excluded from the final
parameter comparison:

1. Known worthless Blindness potions were marked sellable by type, causing
   repeated rejected offers. The producer now excludes apparently worthless
   merchant offers, and an unchanged inventory receipt excludes a rejected
   sale for that visit. [Curated input](live-sale-loop.json).
2. The legacy Borg approximation priced an unseen Ring of the Mouse at zero,
   but the native shop demanded more than the character's 34 gold. Live
   stock now uses the public native `price_item` quote; the historical
   chooser retains its original pricing for parity.
   [Curated input](live-price-loop.json).

The producer also now protects unaware durable items even when no rune is
missing. A full Home's sale prerequisite is selected before a deposit;
historical comparison records retain their original action order. These
are covered by regression tests. The final native samples exercised basic
Home deposits and withdrawals, but not the complete Home-sale-to-merchant
chain or every full-Home contingency; those remain explicit validation gaps.

Defaults remain zero for both policies. Weighted knowledge/opportunity
refinement remains a proposal, and these bounded runs are not owner
acceptance or zoo evidence. Raw comparison files remain outside Git under
`/tmp/borg-joint-live-quoted/` and `/tmp/borg-joint-live-holdout/`; diagnostic
batches are under `/tmp/borg-joint-live-trials/` and
`/tmp/borg-joint-live-corrected/`. The curated receipt binds the actual
binary and source fingerprints used.
