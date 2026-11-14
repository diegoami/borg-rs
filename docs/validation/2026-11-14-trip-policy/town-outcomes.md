# Return outcomes: bounded seed-101 trace (2026-11-14)

The owner's objective permits slower progress and earlier returns. This
check asks what actually changes in town and whether an unresolved gear
backlog persists across a return. It does not score descent speed.

## Method

Added `game_turn` to trip v2 records; `turn` remains the Borg clock.
`tools/trip_policy_report.py` joins those decisions, native `state.log`,
and the existing `BORG_TOWN_STATE` v3 snapshots on native game turns.
It marks initial shopping and incomplete stays, keeps missing snapshots
explicit, and reports inventory deltas without assigning a usefulness score.
Town snapshots count occupied pack entries, not the policy's grouped slot
estimate. Their unknown flags are the town dump's awareness/needs-ident
bits; they are not a new source of private object information.

Multiple depth transitions can occur within one native tick. The report
uses the final depth per tick and reports within-tick changes separately.
Thus a transient town/dungeon/town transition does not split one stay into
several trips. This convention can omit a real zero-duration excursion;
it is explicit and covered by a regression control.

Three Human Warrior arms use seed 101, 100,000 requested game turns:

| Arm | Policy | Unknown-gear threshold | Later town stays observed |
|---|---:|---:|---:|
| Off control | 0 | 4 | 16 |
| Proposed settings | 1 | 4 | 20 |
| Functional knowledge-trigger check | 1 | 1 | 19 |

The threshold-one arm exercises a path that did not fire at four. It is
not a fitted recommendation or a change of defaults. Return counts use the
native-tick convention above; more or fewer returns are not themselves a
success criterion. All runs stopped at the turn bound without death.

## Concrete outcomes

At the proposed settings, one town stay follows recorded pack-pressure
votes (`free 2 <= 2`, then `free 1 <= 2`):

| Observed state | First town snapshot, turn 65847 | Last, turn 68651 |
|---|---:|---:|
| Occupied pack entries | 20 | 11 |
| Unknown entries | 5 | 0 |
| Unknown gear entries | 1 | 0 |
| Recall capability (`BI_RECALL`) | 0 | 1 |

Departure is first observed at native turn 68661. The game message at turn
67023 says `You had a Staff of Light (24 charges) (i).` after the Borg's
Magic Shop sale of the unknown Walnut Staff. This supplies item-specific
evidence of resolution; the aggregate unknown-count drop alone would not.
The visit also contains actual game messages for purchases of Recall and
boots, followed by wearing Boots of Slow Descent. These facts describe one
return, not a general benefit claim or approval of all town dispositions.

In the threshold-one arm, the first knowledge-only return vote is at
native turn 57692 on depth 2 with 18 occupied policy slots; pack pressure
does not yet fire. Town arrival is at 95230 and departure at 97323. The
town snapshots go from 21 to 11 occupied pack entries, eight to zero
unknown entries, one to zero unknown gear entries, and zero to five Recall
capability. No observed stay in either enabled arm retains an identical
unknown-gear multiset between its first/last snapshots. That is not proof
that an unchanged-backlog loop is impossible with other items or seeds.

The interval between requesting return and arriving in town is explained
in part by existing routing: at 57952 the character takes up stairs; at
58462 the policy still requests return, now with route `search`. The
executor logs `no up stairs found, going down`. This is the deliberate
`borg_leave_level` fallback that goes down and back up to regenerate the
level when no up staircase is known. Repeated return votes in this interval
are not repeated completed town trips. No routing or anti-repeat rule was
changed on this evidence.

## Validation and reproduction

- Release tests: 226 passed, 4 ignored; clippy clean.
- Report tests: 5 passed, covering native/Borg clock confusion, repeated
  votes, transient depth changes, missing snapshots, incomplete stays,
  unchanged gear and backwards clocks from appended/mixed runs.
- Smoke C/Rust parity: 3/3 identical keys, messages and state.
- Both policy-off and policy-on threshold-four traces match their earlier
  100,000-turn streams byte for byte on keys, messages and state. The new
  clock field and town-state logging do not change those runs.

Use fresh paths for each process:

```bash
BORG_IMPL=rust BORG_TRIP_RECORD=/tmp/trip-case/trip.jsonl \
  BORG_TOWN_STATE=/tmp/trip-case/town.jsonl \
  bash harness/run.sh /tmp/trip-case/run 101 0 0 100000 borg_trip_policy=1
python3 tools/trip_policy_report.py /tmp/trip-case/run \
  /tmp/trip-case/trip.jsonl /tmp/trip-case/town.jsonl
python3 -m unittest discover -s tools -p test_trip_policy_report.py
```

Raw logs/reports are outside Git in `/tmp/borg-trip-outcomes/final/`.
Runs use source based on `7796c8b` plus the pending record-clock change;
the smoke manifest binds the runtime source fingerprint. No full suite was
re-earned for this logging-only change. Town buying/disposition behavior,
shared `PlannedTrip` integration, policy acceptance and defaults are unchanged.
