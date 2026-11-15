# Trip utility and zoo parameters: bounded validation, 2026-11-14

New shadow records use trip-relative marginal utility, shared supply coverage,
retained-cash utility and discounted Home reserves. All twelve new calibration
settings are exposed in both parsers/configs and recorded in schema 5.
[Model and parameters](../../TOWN-UTILITY.md) ·
[item table](../../TOWN-ITEM-CATALOGUE.md) ·
[sample arms](../../../tools/town-plan/zoo-profiles.json).

## Checks

- `cargo test --release`: **229 passed, 4 ignored**.
- Clippy, `--release --all-targets -- -D warnings`: clean with stable
  **1.98.1** and **1.100.0**, using a separate target directory for 1.100.0.
- Both explicitly selected C and Rust harness builds pass.
- C/Rust smoke: **3/3 identical** on keys, messages and state, checked by
  `harness/diff --suite smoke --expect-impl rust`.
- Three additional manual smoke runs with the v5 observer on match those
  same C and Rust-off streams byte for byte on all three channels. Their
  process receipts report success, exit 0 and an unchanged executable hash.
  All 225 observation records also replay exactly.
- The catalogue renders 369 rows in headless Chromium. Search, need/stock
  filters, six HP-potency potion rows and Remove Hunger's Food mapping pass.
- All eight supplied parameter dictionaries resolve to declared settings;
  the bounded runner prints seven commands by default and requires `--run`.

[Build identities and channel hashes](build-and-parity.json) retain actual
pre-commit/dirty build identity and input hashes. These runs used the exact
source inputs identified there; documentation and the final commit were
written afterwards. Raw binaries and BUILD_ID copies are preserved outside
Git under the receipt's `raw_evidence` path.

The manual observation directory is not a harness suite and has no suite
manifest/DONE marker. It was compared directly, with per-run process receipts;
no synthetic suite marker or unverified `harness/diff` pass is claimed.

## Calibration runs

Seeds **501–507**, race **0**, classes **0–6**, cap **30000 game turns**, for
three explicitly configured shadow arms. Configured parameters are checked
against engine `effective_settings` and every record's complete 24-value vector.

| Arm | Completed executions | Decision snapshots | Median planned spend / available gold | Minimum retained from a positive purse | Positive purses exhausted by discretionary plans | Snapshots buying Recall | Snapshots storing Recall |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Reference | 7 | 609 | 54.815% | 2 | 0 | 0 | 50 |
| Explicit depth 2, duration 1000 | 7 | 609 | 54.631% | 2 | 0 | 0 | 50 |
| Cash priority 50 instead of 25 | 7 | 609 | 40.323% | 2 | 0 | 0 | 50 |

These are **snapshot plans**, not executed transactions. Repeated suggestions
to store the same carried Recall do not count as independent deposits. The
minimum two gold is an observed result, not a configured reserve. The shallow
arm fixes depth 2 and duration 1000; the reference changes destination with
progress. This is not a duration-only comparison.

Concrete matched-state example: seed 501, sequence 0, game turn 1, max depth 0,
149 gold. The reference stores its one Recall for future use and plans 74 gold
of shop purchases, leaving 75. Increasing `w_gold` to 50 changes the purchase
plan to 54 gold, leaving 95. Neither plan buys Recall.

All **1827** v5 records replay exactly to their plans, net solve values,
runner-up and dispositions. All mapped consumables' recorded inventory-power
contributions are zero. Historical v4 replay fixtures still pass.

For each of the seven seed/class pairs, all three arms have **identical keys,
messages, state and gameplay summaries** (excluding settings and wall time).
This establishes observation-only behavior across the calibration parameters;
it is not a survival-benefit experiment. Six distinct trajectories reach the
turn cap; seed 504 dies at turn 1765 to Fang. The three executions of that
trajectory are one set of starting conditions, not three independent lives.

[Machine-readable results](calibration-summary.json) include channel/record
hashes and per-run outcomes. Raw logs/configs are in
`harness/out/rust/town-utility-publish/{reference,shallow,cash-cautious}` in the retained
worktree. This is early-game runtime coverage; deep Recall withdrawal and
stronger-potion/HP scaling are synthetic unit cases, not deep-game outcomes.

## Death evidence and limits

[Seed 504's assessment and original identity](seed504-death.json) records the
unchanged control's natural Priest death to Fang. The bounded original log
tails are retained alongside it. Full original logs, archive files, configured
inputs and repeated trajectories remain outside Git. There is no native movie
or original-frame window, and no claim of replay-qualified private-state
causality or a guaranteed rescue. No tactical change or issue/comment is filed
from this observation. It is not silently promoted to a stronger native death
dossier. The existing [Rust feedback hub](https://github.com/diegoami/borg-rs/issues/3)
is the discussion destination if the owner requests investigation.

## Reproduce

```sh
cargo test --release --manifest-path borg-rs/Cargo.toml
cargo clippy --release --all-targets --manifest-path borg-rs/Cargo.toml -- -D warnings
CARGO_TARGET_DIR=/tmp/town-clippy-1.100 cargo +1.100.0 clippy --release --all-targets --manifest-path borg-rs/Cargo.toml -- -D warnings
BORG_IMPL=c harness/build.sh
BORG_IMPL=rust harness/build.sh
BORG_IMPL=c harness/suite.sh smoke
BORG_IMPL=rust harness/suite.sh smoke harness/out/rust/town-utility-publish-smoke
python3 harness/diff --suite smoke --expect-impl rust harness/out/c/smoke harness/out/rust/town-utility-publish-smoke
python3 tools/town-plan/calibrate.py --arm reference --arm shallow --arm cash-cautious --out harness/out/rust/town-utility-publish --run
python3 tools/town-plan/report.py --root harness/out/rust/town-utility-publish --records-out /tmp/town-calibration.jsonl > /tmp/town-calibration-summary.json
PLAN_RECORD_FILE=/tmp/town-calibration.jsonl cargo test --release --manifest-path borg-rs/Cargo.toml -- --ignored --nocapture a_run_s_records_all_replay
```

Use a fresh `--out` for calibration reruns; the runner preserves existing
outputs. The old ignored `export_visits` simulator is explicitly v4-only;
v5 evidence is the recorded one-step plan, not a simulated complete visit.
No live execution, default-backend change, zoo tag or PR merge is made.
