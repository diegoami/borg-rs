# Carry C milestone10 units into the C oracle and Rust

Owner direction (2026-11-10): after `4.2.6-impact-v9`, continue with C
milestone10. Tracking [Rust#23](https://github.com/diegoami/borg-rs/issues/23);
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

There are **two** C identities in this receipt, and they are not the same
binary. Both record the same C `source_sha256`
(`eb0c22979558759458b2b2c1b4330ef9eed39b43e2c80f9a56c448fe132ca8f9`), so the
C source was byte-identical in both builds.

- [MATRIX-C-BUILD_ID](MATRIX-C-BUILD_ID) is the C binary the **131-case
  matrix actually ran against**, read from the raw suite manifest. It is
  binary `9a7fb7c4…`, built at 12:22:54 from a working tree that carried the
  m10 source changes but was not yet committed (`build_id_git_rev` is the
  milestone9 tip `8a7579d`, with `git_dirty=yes` and `inputs_dirty=yes`).
  [MATRIX-RUST-BUILD_ID](MATRIX-RUST-BUILD_ID) is the Rust role's identity
  and **is** the clean build below.
- [C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are the later
  **clean** builds (`ef8b063`, `git_dirty=no`, `inputs_dirty=no`), made after
  the matrix for the remaining checks: the native fixtures, the no-C-Borg
  audit and the removal-unit checks. The builder is Ubuntu cc 15.2.0 and
  rustc 1.98.1.

This distinction was missed in the first version of this receipt, which
presented the clean `C-BUILD_ID` as the matrix's identity. Review finding
M10-R-01 (borg-rs#23) corrected it; the correction adds the matrix
identities and changes no result. The matrix compares the C and Rust runs
per case on their streams, not on their binaries, and the independent review
verified that its own clean C build reproduced all 23 rerun cases' streams
exactly and that all 131 raw C process records have stable before/after
executable hashes.

The implementation commit is `4f7152d`; `8a135f1` adds the removal units
and the adoption record, and `ef8b063` the status update. Only the
documentation commits follow `4f7152d`, so the build inputs (the
`source_sha256` values) are the same for all three.

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

## Independent review

Independent review: **qualified pass, no acceptance blockers**
([borg-rs#23](https://github.com/diegoami/borg-rs/issues/23#issuecomment-6770750242)).
The reviewer found **no C/Rust semantic mismatch** in the three units and
judged the parity reasoning for the two improvement units sound: they are
named, owner-directed units already merged and reviewed in the consumer's
product, not a policy redesign invented on the Rust side.

The reviewer executed, independently of this receipt: the re-derivation of
the full retained matrix over the retained C/Rust outputs and both retained
fault suites (regenerating an artefact exactly equal to the committed
`matrix.json`); a fresh corrected C and Rust build pair completing the
23-case plan 23/23 with full parity; an independent I-001 fault rerun in a
separate clone (exactly the recorded 13 cases differ, with the recorded
streams per case); all nine normal native fixture modes on fresh M9 and M10
C builds (8/9 before, 9/9 after, all 18 logs byte-identical to the copies
here); the removal-unit checks on another scratch clone; the Rust build
checks; and a hash of every raw stream file against the recorded rows.

Findings and what was done:

| Finding | Severity | Disposition |
|---|---|---|
| **M10-R-01** the committed `C-BUILD_ID` is not the 131-case C suite's actual binary identity (the matrix ran against the earlier dirty build) | minor | **Fixed.** Added [MATRIX-C-BUILD_ID](MATRIX-C-BUILD_ID) and [MATRIX-RUST-BUILD_ID](MATRIX-RUST-BUILD_ID), added a `build_id` field to [current-m10-c-summary.json](current-m10-c-summary.json), and corrected the Identities section above. No gate was rerun and no original evidence was relabelled |
| **M10-R-02** `borg-caution.c`'s m10 hunks are not all byte-identical as claimed: the new include sits in the local sorted block, not immediately after `borg-caution.h` | minor | **Fixed** in [docs/adoption/c-m10/README.md](../../adoption/c-m10/README.md) by narrowing the wording to "the functional m10 code matches; the include is placed in the local sorted include block". No C source change is needed |
| **M10-R-03** I-001's evidence is adequate for a bounded parity adoption but is not a direct edge-case or policy-benefit oracle | observation | **No correction required.** The stated limitation is preserved. A focused I-001 fixture would be needed before making the stronger claims |
| **M10-R-04** `status.json` called the fault-detected cases "units" | note | **Fixed** to "cases" in [docs/adoption/status.json](../../adoption/status.json) |

The reviewer's own limits are recorded with the comment: the consumer's C
review and its targeted-life, cohort and matrix narratives were taken on
trust; the **execution** of the complete 131-case plan remains author
evidence (the reviewer rehashed and compared every retained output and
independently reran 23 cases); I-002's fault control was re-derived from its
retained raw outputs rather than independently rebuilt and rerun; the five
ASan rows were inspected, not rerun; and the discarded first Rust suite's
own logs are not committed.

Owner acceptance remains a separate decision. No tag is placed.

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
