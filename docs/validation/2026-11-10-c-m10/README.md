# Carry C milestone10 units into the C oracle and Rust

Owner direction (2026-11-10): after `4.2.6-impact-v9`, continue with C
milestone10. Tracking [Rust#24](https://github.com/diegoami/borg-rs/issues/24);
record [adoption/c-m10](../../adoption/c-m10/README.md). Target better-borg-c
`6944d88e87d02e9d24e85438d25f0c6ef864c170` (`4.2.6-impact-v10`). Start: Rust
main `8a7579d` (`4.2.6-impact-v9`). This is author validation, not independent
review or acceptance.

Milestone 10 carries **no new bug ordinal**. Its units are two owner-directed
local improvement units (I-001, I-002; never proposed upstream) and one crash
correction attached to the already-carried 0099 entry. The 0099 guard was
already in borg-rs since milestone9, so the C side needed a comment sync only
and the Rust side needed no change.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds
of `ef8b063` (`git_dirty=no`, `inputs_dirty=no`). The builder is Ubuntu cc
15.2.0 and rustc 1.98.1, as recorded in the BUILD_IDs. The implementation
commit is `4f7152d`; `8a135f1` adds the removal units and the adoption
record, and `ef8b063` the status update. Only the documentation commits
follow `4f7152d`, so the build inputs (the `source_sha256` values) are the
same for all three.

`harness/check-no-c-borg.sh` finds no C Borg code in the Rust image.

The first Rust suite pass had to be discarded and rerun: the author rebuilt
the Rust image while it was running, and three cases
(`opt-dyn-priest`, `opt-dyn-war`, `cfg-lunal-deep-warrior`) were recorded as
`binary_changed` by the suite's own guard. Those three runs had themselves
succeeded (engine exit 0, complete key logs); the suite refused to write its
DONE marker because the executable's hash changed under it. The rerun below
is the recorded one.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 103 passed, 1 existing ignored / no warnings. The count and the ignored test are unchanged from the milestone9 baseline; the m10 units add no Rust unit test, because I-001 has no native fixture and I-002's fixture reads C-private state |
| **C-oracle** native fixtures: 2 fixtures, 9 normal and 5 ASan modes ([native-comparison.json](native-comparison.json), logs under [native-logs](native-logs)) | normal: before 8/9, after **9/9**; 1 fail→pass, 0 pass→fail. The fail→pass is `fleeheal` `flee-low`, the I-002 fail-before anchor: before, caution falls through to the recall rest (`acted=1 heal=0 rest=1 keys=5`); after, it queues the Herbal Curing cure (`acted=1 heal=1 rest=0 keys=3`). The three I-002 controls (`flee-highdanger`, `noflee-wounded`, `flee-full`) are **byte-identical** across the two builds. The five `necrolight-296-crash` modes are identical on both sides, because borg-rs already carried the 0099 guard from milestone9. ASan: 5/5 modes pass with **no AddressSanitizer report**. These fixtures read C-private state, so they cover the C side only |
| Removal units | 5 (three C, two Rust). Every hunk of the implementation diff is in exactly one unit. The 0099 unit is the C comment sync only; there is no Rust unit for the 0099 correction, because the Rust guard was already present and is unchanged. Each unit reverse-applies on its own on the final tree. Reversing all of them restores `8a7579d` exactly for the changed C and Rust files |
| `harness/check-no-c-borg.sh` | OK: no C Borg code in the Rust image |
| `harness/regress/diff-consistency-test.py` | 7/7 controls pass |

## The 131-case plan

See [matrix.json](matrix.json) for the per-case digests.

The plan is the exact milestone2 131-case plan
(`harness/out/c-m10/plan/adoption-suite-final`), with the same seeds, the
same budgets (`RUN_TIMEOUT_SECONDS=600`) and terminal native saves for every
case. `JOBS=3`.

| 131-case milestone2 plan ([matrix.json](matrix.json)) | **131/131 cases complete in both roles, all 131 with full C/Rust parity** on `keys.log`, `messages.log`, `state.log` and the terminal native save. `harness/diff` exit 0: **131/131 runs identical**, with `messages.log` and `state.log` compared in all 131 runs. No case is missing and no case timed out |

## Behaviour change against milestone 9

[current-m10-c-summary.json](current-m10-c-summary.json) holds the milestone10
C run's per-case stream digests. Compared with the recorded milestone9 C run
([current-m9-c-summary.json](../2026-11-09-c-m9/current-m9-c-summary.json)),
**94 of the 131 cases changed**. Deaths over the 130 cases that completed in
the milestone9 run: 13 (M9) → 14 (M10). This is an observation, not a
survival claim: both units change Borg behaviour by design, so a large
behaviour delta is the expected result, and three fixed-ish seeds and one
suite are not statistics.

The `cfg-munchkin-dunadan-bg` case, the pre-existing munchkin stair livelock
recorded in the milestone9 receipt, now completes in both roles
(`stop_reason=turns`, `game_turns` 20112, no death) and is part of the
131/131 parity. The livelock mechanism is unchanged in the code; the
improvement units simply reach a different path, as they did for the
milestone8 → milestone9 transition. The earlier report on
[better-borg-c#204](https://github.com/diegoami/better-borg-c/issues/204)
stands as the record of the mechanism.

The raw runs (`harness/out/c-m10`) are not in Git. The two summary files
hold each run's per-case stream digests and stop/death fields, and together
give the 94-case set and the two death counts.## Per-unit Rust fault controls

[fault-controls.json](fault-controls.json). Method: for each Rust removal
unit, a separate worktree at the implementation commit with that one unit
reverse-applied, `BORG_IMPL=rust harness/build.sh`, then the 23-case
`docs/validation/2026-11-08-c-m3/fault-suite` run on the same inputs. A
control is *detected* when a case differs from the corrected C run.

| Unit | Detected |
|---|---|
| I-001 | **13/23** cases differ |
| I-002 | **14/23** cases differ |

Both units are reached by the ordinary suite, so neither rests on the
translation review alone. Fault outputs are never positive parity evidence.

## I-001 has no native fixture

The consumer declares the absence: its guard never fires in any of its
fixtures, and its own matrix shows zero row changes for the unit. Its
evidence is a targeted life (seed 1907608237) and a paired cohort (seeds
11/29/47), both of which are the consumer's own runs and outside the Rust
lane's reach. The Rust coverage for I-001 is therefore the C/Rust parity
matrix, the 13/23 fault control, and the translation review. This is the
thinnest part of the batch, and the record says so.

## Not claimed

- A population survival or progress benefit for either improvement unit. The
  consumer's own cohort evidence is directional at n=3 and says so; this
  lane adds no cohort of its own.
- Whole-private-Borg equivalence.
- That the consumer's fixture baselines and the borg-rs fixture baselines are
  the same files: the borg-rs copies are the consumer's committed kits at the
  milestone10 pin, and the baseline logs were not re-derived here.
- Independent review or owner acceptance.

## Raw outputs

`harness/out/c-m10`, outside Git. The native fixture logs are copied into
[native-logs](native-logs) because they are small and are the fail-before
evidence for I-002.
