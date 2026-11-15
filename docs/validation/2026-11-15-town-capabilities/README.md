# Town capability coverage validation — 2026-11-15

Runtime: `c91fc3640f22155218d1299c81d2a2a1d63ed167`, on the combined M12
trip/town policy. Baseline: frozen `d9a56f48bac8675c91f551619492af36b698b4cf`.
Both live policies are enabled in each gameplay arm. The current zoo studies
remain pinned to their original source; this repair is a separate candidate.

The missing supply mappings are repaired, but improved survival is **not
established**. This is a bounded wiring check: one Human start per class,
seeds 501–509, 30000 game turns, normal menu-letter birth, no cheats, after-key
cap. The final candidate was frozen before its run. Baseline records are reused
from their explicitly retained diagnostic run; they are not extra lives.

| Arm | Starts | Deaths | Mean CL | Mean max DL | DL3+ | Purchases | Gold spent |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline-live | 9 | 1 | 2.44 | 1.22 | 0 | 172 | 3384 |
| capability repair | 9 | 2 | 2.56 | 1.44 | 0 | 358 | 4467 |

The Druid dies in both arms; the Necromancer is newly killed in the repair.
There are no rescues in this small cohort. More purchasing and a slightly
higher terminal level/depth do not establish a better policy. Defaults remain
provisional; do not select a zoo winner from these nine cases.

## What it actually buys

[Per-class shopping ledgers](shopping.md) show every confirmed normal-executor
trade in order. All **530 retail purchases** across the two arms have native
receipts, matching wallet deductions, and a matched planner record. Unconfirmed
commands: zero. Native receipts determine purchased quantities; resulting pack
stack counts are not reused as purchase counts.

The repair buys **80 oil flasks**, compared with zero in the baseline, and the
native logs contain **30 oil-hit messages**. The Warrior buys 22 iron shots and
four flasks; the Ranger buys 18 arrows and a usable Nature book. The ledgers also
show spending on learning and support supplies, allowing their costs to be
reviewed rather than inferred from a planner score. This is evidence of execution,
not proof that the quantities or priorities are well calibrated.

## Coverage and checks

- Engine-backed [ordinary-stock coverage](coverage.json): all 104 kinds
  accounted for. 95 have capability/gear coverage; nine have explicit exclusions
  in the audited level-1 contexts: six advanced books, two ammunition kinds with
  no matching starting launcher, and confusing Whisky. Coverage does not mean
  the planner should purchase every item. The generated catalogue also lists
  possible Black Market kinds and explicit consumer/policy exclusions.
- CI now runs the native catalogue gate through `harness/check-town-capabilities.sh`:
  nine bounded 1000-turn cases. The [local CI-command run passed](ci-coverage.log).
  A [synthetic negative control](coverage-negative-control.json) removing Arrow
  supplies fails the gate even though the Ranger has a matching launcher.
  The workflow also requires the generated catalogue to match the committed page.
- 267 Rust tests pass; four opt-in tests remain ignored in the default command.
  [Python tests](python-tests.log): 24 pass.
- Clippy passes on [1.98.1](clippy-1.98.1.log) and
  [1.100.0](clippy-1.100.0.log), using separate target directories.
- Rust and C harness builds and smoke runs pass. The latest Rust runtime and
  the C oracle produce [3/3 identical smoke key/message/state streams](smoke-diff.log).
  The C build is at 5a209f3; its C input hash is unchanged by c91fc36's Rust-only
  corrections.
- [Seven invariance pairs](invariance.json) match complete key, message and
  state streams: three controls against pinned d9a56f4, two controls with/without
  recording, and two live cases with/without recording. Runtime settings are
  verified from native summaries.
- [9791 historical records replay](historical-replay.log): schema 5, schema 6,
  and the earlier committed schema-7 table. [1522 current records replay](current-replay.log),
  including the two shadow controls. Replay reproduces dispositions, selected
  plans, solve certificates and alternatives. All game-process outcomes in the
  final 16 new attempts and 12 reused references are valid deaths/cap stops.

## Death evidence and attempt accounting

[Primary death assessments and original message tails](deaths.md) cover every
natural death in the two nine-start cohorts. They are harness traces, not native
movies; exact world-turn movie frames are unavailable. No causal C/Rust defect or
counterfactual survival is established. A prior town inventory cannot prove that
supplies remained unused at death.

[All cohort/development attempts](all-attempts.json) retain failures and repeats:
71 attempts in these campaigns, comprising nine output-setup failures before
any game, and 62 actual games across development, comparison and invariance
checks. Routine unit/smoke/CI-coverage checks are separate. These campaigns contain nine
natural-death attempts across repeated source/seed combinations; they are
[assessed separately](development-deaths.md), not pooled into the table.
The first two ad-hoc development batches did not seal source/binaries. Their
native traces and binary hashes survive, but exact reproduction is unavailable;
they are excluded from performance/replay claims. Committed-runtime comparisons
retain complete runtime images and source/build/input identity.

## Reproduce or inspect

[Build identities](builds.json) · [run receipts](run-receipt.json) ·
[per-cell outcomes and hashes](cells.json) · [transactions CSV](transactions.csv).
Raw candidate runs and retained runtime images are under
`/home/diego/projects/borg-rs/harness/out/town-capabilities-2026-11-15-final`;
reused baseline paths are explicit in the receipts. The prior sealed batch is
`town-capabilities-2026-11-15`; early diagnostics are retained in the final
root's `development/` directory. Full raw runs and binaries stay outside Git.

For a fresh single case, use a non-existing output directory:

```sh
BORG_IMPL=rust BORG_TOWN_PLAN_RECORD=/absolute/OUT/plan.jsonl \
  BORG_TRIP_RECORD=/absolute/OUT/trip.jsonl \
  BORG_TOWN_CAPABILITY_AUDIT=/absolute/OUT/capability-audit.jsonl \
  harness/run.sh /absolute/OUT 501 0 0 30000 menu,borg_town_plan=1,borg_trip_policy=1
python3 tools/town-plan/check-coverage.py /absolute/RUNS/*/capability-audit.jsonl
PLAN_RECORD_FILE=/absolute/OUT/plan.jsonl cargo test --release -- --ignored --nocapture a_run_s_records_all_replay
```

The audit contains hypothetical known items, not purchased inventory. Regenerate
this shopping/outcome report from the retained receipt with
`python3 tools/town-plan/capability_report.py --root RAW_ROOT --out REPORT_DIR`.
