# Live town utility: matched local full suite, 2026-11-14

This report is the standalone town-branch experiment. The later
[active combined M12 zoo studies](../../ACTIVE-JOINT-EXPERIMENT.md) use a different
pinned runtime; these results and ledgers must not be presented as that study.

This is an actual live-policy comparison, not shadow calibration. Measured
runtime: `ac63c5e` on `feature/town-inventory-strategy`; the branch's existing
v11R base is preserved. Source/binary/build identities are in
[frozen-experiment.json](frozen-experiment.json). All four arms share the same
runtime, compiler, starts, fixed calendar and observation contract.

## Main result

45 full-suite starts per arm, 100000 game turns, no cheat death, explicit normal
mode. Menu-letter birth plays all nine intended classes/races; this corrects the
historical full suite's documented indices-7/8 quirk. It is a fresh matched
comparison and must not be relabelled as the historical M12/class-ladder table.

| Arm | Deaths | Mean character level | Mean deepest level | Reached DL3+ |
| --- | ---: | ---: | ---: | ---: |
| Current | 13 | 4.11 | 2.24 | 9 |
| No shops or Home | 19 | 4.27 | 2.42 | 6 |
| Utility reference | 11 | 4.29 | 2.18 | 7 |
| Utility, cash priority 50 | 11 | 3.71 | 1.71 | 1 |

**All 180/180 executions are valid: no timeout, crash, missing result or
configuration failure.** Means use all 45 deaths/capped survivors per arm;
character level is terminal `clevel`, deepest level is `max_depth`, DL3+
counts any run that reached depth 3, including a later death. Survivors are
censored at the cap, not wins. There are 45 paired starting conditions, not
180 independent seed draws.

The reference has two fewer deaths but slightly less depth progress than the
control. It is **not established as better**. Its 45 matched pairs contain
eight control-death/arm-survival cases and six opposite cases; the two-sided
exact paired death test is about 0.79. Cash priority 50 preserves neither the
reference's progress nor a clear class-wide benefit. Five starts per class
are not enough to adopt class-specific winners.

[Every measured cell](cells.csv) · [machine-readable results](summary.json) ·
[per-class deaths and DL3+](classes.md). No failures were removed. No-shops
issues zero actual buy/sell commands; the report excludes the unrelated
"Buying time waiting for Recall" note.

## What was actually bought

The [confirmed town ledger](../../town-transactions/README.md) shows every
retail purchase with quantity, native receipt, exact purse decrease and
per-class/seed chronology, plus confirmed normal-executor Home transfers.
It includes buy-then-store behaviour and the ration deposit/rebuy sequence.
The receipt audit verifies all 4454 retail purchases across the shopping arms.

## Why these parameters next

| Diagnostic over all 45 starts | Current | Utility reference | Cash50 |
| --- | ---: | ---: | ---: |
| Mean fraction of observed world turns in town | 22.6% | 30.5% | 32.5% |
| Buy commands involving unseen/unknown wares | 170 | 584 | 595 |
| Home deposit commands | 176 | 490 | 505 |
| Mean last-observed gold | 274.6 | 491.2 | 338.5 |

Commands are attempts, not successful-item counts, and Home transfers are not
cash purchases. The unseen count includes useful first purchases such as CLW;
it does not establish that every identification purchase was waste. Gold is the
last logged key state because this harness summary omits terminal gold. Town
fractions integrate recorded world turns, not zero-time store keystrokes.

The focused zoo development grid varies only **Knowledge priority 5/25** and
**Home discount 250/500**. These directly address the large learning/deposit
load while retaining buy-to-identify and future storage. Keep cash priority 25:
50 lost substantial progress in this pilot. Hold physical healing capacities,
food/light coverage units, risk and adaptive ambition fixed for this grid.
Do not tune physical units or freeze a character at depth 2 to fit the table.
See [the proposed grid](../../../tools/town-plan/zoo-live-grid-experiment.json)
and [profile configurations](../../../tools/town-plan/zoo-live-grid-profiles.json).

The four utility corners plus current/no-shops are six arms. Existing zoo
`classes-1800` development means **10800 cells** (1800 starts × 6). A catalogue
smoke on `classes-18` would be 108 cells first. Large workflows are not dispatched
here. Fit class overrides on development only, then freeze candidates and use
fresh `classes-1800-b` confirmation before any improvement claim.

## Corrections and evidence retained

The preflight Priest uncovered the legacy zero appraisal of an unidentified
negative-bonus Black Market ring; the live executor now uses the game's visible
quote. The first complete 180-game batch then exposed attempts to sell worthless
cursed gear. Known worthless items are excluded; observed unchanged holdings
block a refused buyer for the visit. Both fixes concern execution integrity,
not parameter tuning. The corrected full batch repeats exactly the same starts
and vectors. The original batch is retained and its separate
[summary](initial-batch-summary.json) is not mixed with the main result.

Both current and no-shop controls repeat **90/90** cases byte-identically on
keys, messages and state between batches. All **15244** corrected live records
replay exactly. No insufficient-gold messages occur in any corrected arm.
235 Rust tests and both Clippy 1.98.1/1.100.0 checks pass; 17 Python tests pass.
The current control also retains C/Rust smoke 3/3 equality. The subsequent
iterator-only cleanup required by Clippy 1.100 does not change policy; actual
measurement identity remains the committed `ac63c5e` and retained executable.

[Death assessments](death-assessments.json) and bounded original tails preserve
each of the 54 measured natural-death observations. Full original logs/configs/
archives remain outside Git. These are qualified harness observations, without
a native movie or private branch trace; no causal tactical bug or guaranteed
rescue is inferred. The original failed/preflight attempts remain separate.
No death issues/comments are created automatically; focused review belongs in
the existing borg-rs#3 hub when the owner requests it.

## Reproduction and recovery

```sh
BORG_IMPL=rust harness/build.sh
flock /tmp/angband-validation.lock python3 tools/town-plan/run_live_suite.py \
  --out /absolute/path/to/NEW-LIVE-FULL --jobs 4 --run
python3 tools/town-plan/summarize_live_suite.py \
  --root /absolute/path/to/NEW-LIVE-FULL --out /tmp/live-full-report
```

Use a fresh output root. The manifest is written before observation and includes
all starts, parameters, source/build identity, caps and failure policy. The
measured root is `harness/out/rust/town-live-full-corrected` in the worktree.
A persistent ignored copy, preserved binary/data and the first-batch archive
are under `/home/diego/projects/borg-rs/harness/out/town-live-2026-11-14`.
[Recovery checksums](retention.json) identify them. Reuse original configs/seeds,
restore the retained executable/data or build the pinned source, and verify
original streams before claiming a qualified replay. Dead saves are terminal
evidence and do not restore prior Borg memory.
