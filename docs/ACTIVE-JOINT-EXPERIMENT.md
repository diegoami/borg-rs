# Active combined trip/inventory study

Owner clarification, 2026-11-14: the zoo already has dispatched experiments
varying both live policies. Use these as the current combined-policy outcome
study before deciding on another parameter grid.

## Exact pins and runs

- Borg: `experiment/joint-policy-m12`,
  `d9a56f48bac8675c91f551619492af36b698b4cf`, milestone-12 engine.
- Zoo catalogue/workflow: `abe128d62ef05f8ccce289e9c16d3b68469f3be2`.
- [Development, 10800 games](https://github.com/diegoami/borg-zoo/actions/runs/43643719860):
  six profiles on `classes-1800`, 100000 turns, 200 starts/class across all races.
- [Fixed confirmation, 5400 games](https://github.com/diegoami/borg-zoo/actions/runs/43643769983):
  three profiles on fresh `classes-1800-b`.
- [Merged zoo request PR #91](https://github.com/diegoami/borg-zoo/pull/91) and
  [pinned study definition](https://github.com/diegoami/borg-zoo/blob/abe128d62ef05f8ccce289e9c16d3b68469f3be2/docs/RS-JOINT-M12.md).

Both runs were **queued** at inspection; the linked live status is authoritative.
No completed study outcome is inferred.

Every candidate has `borg_town_plan=1` and `borg_trip_policy=1`. Only the
existing-behaviour control switches both off. Development compares untuned
risk forecast 1000, neighbours 1500/2500, selected 2000, and 2000 with earlier
returns (four free pack spaces / two unknown gear stacks). The fixed
confirmation compares existing behaviour, untuned live policies and the 2000
candidate. Its primary paired natural-death endpoint and fixed candidate must
not be replaced by a development winner after results are seen.

## Relationship to this branch

The trip branch already merged town utility `785f506` and implemented its own
live integration/shared target wiring before the M12 experiment was pinned.
Thus the requested combined runtime already exists. A scratch attempt to merge
the latest town-only branch into the older trip branch was cleared without
publishing another runtime or changing either active pin.

The [local town-only full comparison](validation/2026-11-14-town-live-full/README.md)
and [actual purchase ledger](town-transactions/README.md) describe the standalone
v11R town runtime at `ac63c5e`. They are useful diagnostic evidence but are not
the milestone-12 combined candidate's outcomes or purchases. The later
town-only executor and combined executor also differ in transaction details;
do not attribute the standalone ration/learning sequences to the active
candidate without checking its own trace.

The Knowledge 5/25 × Home discount 250/500 six-arm proposal remains an unrun
hypothesis, deferred while these existing studies are pending. No second large
grid or replacement confirmation is dispatched from this branch.

## Seeing actual combined purchases

At the pinned zoo adapter/workflow, native parsed settings are verified and
outcomes/depth summaries are retained. The adapter does not request `-msglog`
or enable `BORG_TOWN_PLAN_RECORD` / `BORG_TRIP_RECORD`; the workflow sets none
of those recording variables. Consequently the existing outputs do not supply
a full actual-purchase ledger. Summary gold changes cannot identify purchased
items or distinguish them from pickups, theft and sales.

For transaction-level explanation after the study, retain the exact specimen,
resolved profile, configuration hash, assigned birth and seed. A separately
labelled diagnostic rerun can add message/trip/town recording; compare its play
hash to the original cell before treating it as an explanation of that measured
trajectory. This is a new process attempt, not another independent life or a
replacement result. Keep failed/mismatched diagnostic attempts explicit.
No active workflow, frozen source or experiment endpoint is changed to add
logging mid-study, and no unattended monitoring is installed.
