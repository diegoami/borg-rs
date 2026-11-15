# Class-profile validation, 2026-11-14

Zoo integration request: [borg-zoo#88](https://github.com/diegoami/borg-zoo/issues/88).

Shared defaults and optional class overrides work through the existing zoo
selector. The shared reference is unchanged for all nine classes. The probe
changes one parameter each for Mage (cash 50), Ranger (Home discount 750), and
Blackguard (risk 1500); these values were fixed before the runs and are not
accepted class policies.

## Native zoo integration

The real `catalogue.py`, `borgrs.py` and `stall.py` from zoo commit
`7ae123b8240e92428470e9c2b6145eebc9ef614b` were copied unchanged into a scratch
package. Their Git blob hashes are checked before running; no zoo source or
catalogue was edited. The real `catalogue.experiment`, `cells`,
`resolve_profile`, and `borgrs.run_cell` consumed the exported native profiles.

Prospectively frozen matrix: **9 classes × 2 seeds (901, 1901) × 2 arms = 36
cells**, Human, normal mode, cap 2000 game turns, 120-second wall limit,
no cheat death. The runtime is unchanged from borg-rs `785f506`; the existing
Rust executable was reused after checking its runtime-input and binary hashes.
Its original pre-commit BUILD_ID remains in [build-identity.json](build-identity.json).
No new tag or specimen was published.

Results:

- **36/36** class/profile selections, profile hashes and written configurations pass.
- The game’s **parsed `effective_settings`** matches every resolved setting.
- Every v5 record’s full 24-parameter vector matches the actual runtime values.
- Actual played class matches all nine intended indices, including Ranger and Blackguard.
- All **18/18** matched class/seed pairs have identical play hashes, final
  fields, outcomes and full gameplay summaries/RNG states (excluding settings
  and wall time).
- **1370/1370** plan records replay to the same plan, solve, runner-up and dispositions.
- 32 executions reach the cap; four natural-death executions are two repeated
  starting conditions: Druid 1901, crow, turn 1164; Priest 1901, Grip, turn 1849.
  No crash, timeout or config error is omitted or relabelled as survival.

[Original frozen matrix](native-frozen-plan.json) ·
[verification](native-verification.json) · [curated cells](native-cells.json) ·
[full gameplay-summary/RNG checks](gameplay-pairs.json).
The final exporter/resolver was also checked against every retained native
cell, and its generated catalogue equals the catalogue the zoo consumed.

The upstream adapter keeps keys for deaths and discards survivor keys/state
logs after computing the hashes and depth summaries. That behavior was retained.
Raw summaries, configs, archives, stderr, profiles and plan records remain at
`harness/out/rust/town-class-native`; this is no claim of a complete native movie.

## Local CLI and reporting

A separate prospectively chosen seed, **2901**, cap 1000, all nine classes,
reference and probe: **18/18** cells complete at the turn cap. The local
runner resolves the same vectors; all nine matched pairs have identical keys,
messages, state and gameplay summaries. All 18 groups remain visible in the
per-class report. **582/582** additional records replay.

The CLI uses menu-letter birth for every class and records the complete
experiment manifest, selected profile, source of each setting and hashes.
`report.py` dispatches new manifests to the class report instead of silently
limiting them to the historical seven classes. Missing/failed cases remain in
the denominator. [Frozen CLI plan](cli-frozen-plan.json) ·
[CLI verification](cli-verification.json).
Raw evidence: `harness/out/rust/town-class-cli`.

**14 Python tests pass**, covering precedence, shared fallback, class isolation,
export equivalence, legacy-format loading, duplicate/invalid input rejection,
explicit modes, execution gating, all-nine-class seed pairing, incomplete-cell
reporting and accidental fixed-depth progression rejection. No Rust/game source
changed in this tranche, so its existing 229-test/C-Rust parity receipt remains
the runtime baseline rather than a newly claimed test run.

## Death evidence

[Assessments](death-assessments.json) preserve actual build/input/profile/key
identity and the two original natural-death conditions. Bounded key tails are
retained alongside them; the matching probe executions are repeats, not new
independent lives. The original death keys, summaries and archives remain in
the raw native directories. No native recording manifest or frame windows
exist, so these are qualified zoo/harness observations, not replay-qualified
native death dossiers. No tactical defect, causal explanation or guaranteed
rescue is inferred, and no death issue/comment is posted. Further investigation
would use the existing borg-rs#3 feedback hub on the owner's instruction.

## Reproduction

```sh
python3 -m unittest discover -s tools/town-plan -p 'test_*.py'
# Supply a read-only zoo checkout at the pinned commit, or the three modules
# with the exact blob hashes listed by the validator. Use a fresh output path.
flock /tmp/angband-validation.lock python3 tools/town-plan/validate_class_profiles.py \
  --zoo-root /path/to/pinned-borg-zoo --out /tmp/NEW-CLASS-PROFILE-CHECK
PLAN_RECORD_FILE=/tmp/NEW-CLASS-PROFILE-CHECK/all-records.jsonl \
  cargo test --release --manifest-path borg-rs/Cargo.toml -- --ignored --nocapture a_run_s_records_all_replay
```

This is a class-configuration and observation check. Fitness calibration,
accepted per-class overrides, live-policy comparison, PR merge and large zoo
workflow dispatch remain separate steps. The accompanying request to the zoo
must retain that boundary.
