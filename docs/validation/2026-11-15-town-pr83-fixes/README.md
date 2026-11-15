# PR83 correction validation — 2026-11-15

Runtime source: **`e263db712f1b09f456e8b8de760a2614712f1134`**, based on
`67750cf7763ab2d602ef6f8aaba1b39505d46c28` (runtime `dd3eeac`).
Later commits in this PR add receipts, ledger-reader/CI checks and documentation;
the runtime input hashes remain those below. No frozen zoo pin or accepted tag
is changed. The PR is a review candidate, not an accepted default.

| Build | Source input SHA-256 | Binary SHA-256 |
| --- | --- | --- |
| Rust | `11eca0153b83489bc2bb4dd0c2d5539c440fea0064d6a552606b99f7d2893338` | `ac720a200f1435bb8008d0c3bc775fb9e91aa0f36f47794dde3c3ce75482faf3` |
| C | `a139daf21b667a0c367b092fea543563be3e47fc3a4826b31dd05b2fa653229d` | `71755c3ae96d1f12b482c05309585c19bd8c12f2f132a7b85091bcd2b84888fb` |

The full [Rust BUILD_ID](rust-BUILD_ID), [C BUILD_ID](c-BUILD_ID) and retained
[image identity](build.json) are authoritative. Both builds report
`inputs_dirty=no`; `git_dirty=yes` reflects unfinished documentation only.
Normal C/Rust binaries contain no test-only learning-key bridge.

## Correctness checks

- [285 Rust tests pass](unit-tests.log), four opt-in tests remain ignored by
  the default command. No cargo fmt was run.
- Clippy passes with warnings denied on [1.98.1](clippy-1.98.1.log), including
  the test fixture feature, and [1.100.0](clippy-1.100.0.log), using separate
  target directories. [25 Python tests pass](python-tests.log).
- The [native fixture](native-fixture.json) calls the real engine: partial
  ranged-combat rune learning changes the description, leaves runes unknown,
  and preserves the production learning key. A full pack accepts 38 arrows
  into a two-arrow quiver; expansion after 40 requires a slot. Existing pack
  joins permit Home withdrawals. Native sale quotes match split charged
  devices, Black Market pricing, owner caps and no-selling; charging counts
  match the rod schedule's short-horizon controls. The bridge is feature-gated.
- The [actual CI catalogue command passes](ci-coverage.log): nine native
  1000-turn class contexts, all 104 ordinary shop kinds accounted for.
  The longer matrix's [coverage](coverage.json) has 94 gear/capability-covered
  kinds and ten explicit contextual/consumer exclusions. Monster Confusion is
  now deliberately excluded from combat supply; learning value is separate.
- Rust and C harness builds pass. [3/3 smoke runs match](smoke-diff.log) complete
  key, message and state streams on the corrected source.
- [Nine invariance checks](invariance.json) pass: five recorded/unrecorded
  comparisons (three live, two control), plus two corrected-control cases
  each reproduced twice in C. Full keys, messages and state logs match.
- [7,953 current schema-9 records replay](current-replay.log), including both
  enabled and shadow/control cases. [3,888 historical records replay](historical-replay.log):
  888 schema 5, and 1000 each of schemas 6, 7 and 8. Their
  [selection receipt](historical-supported-selection.json) names the sources.
  Schema 4 is covered by the fixture-based Rust replay tests; no old live v4
  raw file was available in this local archive. A preliminary broad archive
  scan included an explicitly unsealed development file with 52 capability
  rows, predating the supported schema-7 table (53+). It was rejected as invalid;
  that failed attempt remains in the raw root and is not silently counted as
  a supported historical replay.

## Bounded native gameplay and purchases

All **33 matrix attempts** ended in valid cap stops or natural deaths. Twelve
selected original paired Necromancer/Ranger cases were rerun in both arms
(24 games, 100000-turn caps), plus nine Human class starts, seeds 501–509,
30000-turn caps, both policies enabled. Pilot profile/race/class/configuration
hashes match the original zoo inputs. All original sources and configurations
are retained; the combined runtime is a separate image.

These are diagnostic seeds, including cases chosen by their original deaths.
They are **not** a fresh survival experiment. Do not pool selected pilots with
the nine class wiring checks or select default parameters from these results.

| Selected class pilots | Control deaths / 6 | Live deaths / 6 | Live mean terminal CL | Live mean max DL |
| --- | ---: | ---: | ---: | ---: |
| Necromancer | 2 | 1 | 4.17 | 1.33 |
| Ranger | 2 | 2 | 4.67 | 2.17 |

| Nine class wiring checks | Terminal CL | Max DL | Result |
| --- | ---: | ---: | --- |
| Warrior | 5 | 3 | cap |
| Mage | 3 | 1 | cap |
| Druid | 2 | 1 | death |
| Priest | 4 | 1 | cap |
| Necromancer | 2 | 1 | cap |
| Paladin | 3 | 2 | cap |
| Rogue | 3 | 1 | cap |
| Ranger | 3 | 2 | cap |
| Blackguard | 4 | 1 | cap |

[All per-run identities and outcomes](runs.json) ·
[confirmed per-class shopping](shopping.md) ·
[complete transaction CSV](transactions.csv) ·
[transaction audit](transaction-audit.json).

All **2,153 retail purchases** have native receipts and matching wallet changes;
all **1,410 live-policy buys** match planner records. There are zero unconfirmed
commands in this matrix. The ledger also records 271 confirmed Home stores,
seven Home withdrawals and 488 market sales. Schema 9's `# Buying` confirmation
can follow the native receipt: the reader anchors the transaction to its prior
executor intent, checks the receipt/wallet, then attaches the confirmed shop
log without inventing a second purchase.

[All eight natural deaths are assessed](deaths.md), with original native tails,
[actual before-death observations](death-observations.json), source/build/input
hashes, and qualified follow-ups. The combined candidate newly kills the selected
Ranger seed 1021027760 that survived the original default. Seed 285192323 still
dies poisoned here despite surviving the simpler quiver-only diagnostic; the
combined repairs are not interchangeable with isolated probes. The Necromancer
lava death exposes an additional retreat/terrain question; it is not attributed
to shopping from a last inventory alone. No unrelated tactical correction or
new zoo experiment is folded into these results.

## Attempts, raw evidence and reproduction

Raw root:
`/home/diego/projects/borg-rs/harness/out/town-pr83-fixes-2026-11-15-final`.
It contains the sealed runtime image, full 33-game plan/receipts, original
configuration files, native key/message/state/frame streams, final saves,
ledger JSON, concatenated replay files and nine invariance attempts. Copies
and reruns do not create extra independent lives. Smoke, nine short CI contexts
and the controlled native fixture are separate correctness checks. Preliminary
fixture setup attempts (missing runtime assets, then an unassessed synthetic
object) failed before the final checks; all were retained and no game outcome
was promoted from those attempts. The initial CMake test target typo was fixed
to `OurExecutable`. The first 33-game source freeze at `665828b` is retained
separately in the sibling raw root without `-final`, with its nine invariance
attempts and preliminary setup/archive-scan failures. Final transaction details
add exact one-copy device quotes and defer money-scumming updates until observed
purchase completion. [All 33 final key/message/state streams equal that first
freeze](final-refinement-invariance.json); repeated deaths do not create extra
independent cases. The final image has its own complete 33-game matrix and nine
invariance attempts. No raw evidence was deleted.

```sh
CARGO_TARGET_DIR=/tmp/town-unit cargo test --release --manifest-path borg-rs/Cargo.toml
CARGO_TARGET_DIR=/tmp/town-unit cargo clippy --release --all-targets --features test-town-review --manifest-path borg-rs/Cargo.toml -- -D warnings
CARGO_TARGET_DIR=/tmp/town-clippy-100 cargo +1.100.0 clippy --release --all-targets --manifest-path borg-rs/Cargo.toml -- -D warnings
BORG_IMPL=rust harness/build.sh
BORG_IMPL=c harness/build.sh
BORG_IMPL=c harness/suite.sh smoke /absolute/C-SMOKE
BORG_IMPL=rust harness/suite.sh smoke /absolute/RS-SMOKE
python3 harness/diff --suite smoke --expect-impl rust /absolute/C-SMOKE /absolute/RS-SMOKE
bash harness/check-town-capabilities.sh /absolute/FRESH-COVERAGE
cmake -S angband -B build/review-probe -DCMAKE_BUILD_TYPE=Release -DSUPPORT_BORG_HARNESS=ON -DBORG_IMPL=rust -DBORG_RS_FEATURES=test-town-review
cmake --build build/review-probe --target OurExecutable --parallel 4
python3 harness/regress/town-review/run.py build/review-probe /absolute/FRESH-NATIVE
PLAN_RECORD_FILE=/absolute/PLAN.jsonl CARGO_TARGET_DIR=/tmp/town-unit cargo test --release --manifest-path borg-rs/Cargo.toml -- --ignored --nocapture a_run_s_records_all_replay
```

The native fixture is wired into the existing coverage CI job. Use fresh output
paths and the shared local validation lock for bounded native campaigns.
[Review disposition](../../TOWN-PR83-FIXES.md) ·
[review prompt](../../REVIEW-TOWN-PR83-FIXES-PROMPT.md) ·
[class investigation](../../TOWN-CLASS-INVESTIGATION-43667787016.md).
