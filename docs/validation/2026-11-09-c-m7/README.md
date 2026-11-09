# Carry C milestone7 fixes into C oracle and Rust

Owner direction (2026-11-09): after `4.2.6-impact-v6`, proceed to C
milestone7. Tracking [Rust#17](https://github.com/diegoami/borg-rs/issues/17);
record [adoption/c-m7](../../adoption/c-m7/README.md). Target better-borg-c
`3573d741e6dc3cafa4099fcef8b38fdb32952977` (`4.2.6-impact-v7`). Start: Rust
main `e5cb263` (`4.2.6-impact-v6`). Implementation commit `54e3d1e`. This is
author validation, not independent review or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
`c9a198c` (`git_dirty=no`, `inputs_dirty=no`), which equals `54e3d1e` for every
Borg source; `c9a198c` only adds the fixture import and runner. The game
matrix and the fault controls ran on these builds. `harness/check-no-c-borg.sh`
reports no C Borg code in the Rust image. The engine objects are byte-identical
between the two images, except `buildid.c.o`, which the script allows to
differ. There are no engine, data, header or FFI changes. Builder: Ubuntu cc
13.3.0 and rustc 1.97.0, as recorded in the BUILD_IDs.

`7efe65c` then adds one Rust comment, the Rust side of 0065. The Rust image
was rebuilt cleanly at that commit
([RUST-BUILD_ID-7efe65c](RUST-BUILD_ID-7efe65c)). Its `binary_sha256` differs
because the build identity is linked in. The Rust side of the full plan was
therefore rerun on it: all 131 cases have byte-identical `keys.log`,
`messages.log`, `state.log` and `final.save` to the `c9a198c` Rust run. Only
`wall_seconds` differs. Later commits change only docs.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 100 passed, 1 existing ignored / no warnings. `parsed_pain` now expects the full text (0071) |
| **C-oracle** native fixtures: 13 fixtures, 43 normal modes and 8 ASan modes ([native-comparison.json](native-comparison.json)) | normal: before 24/43, after **42/43**. 18 go from fail to pass, 24 pass on both sides, and none goes from pass to fail. The one mode that fails on both sides is painmsg `puff`, which the consumer's kit also asserts fails in both phases. ASan: before 6/8, after **8/8**. A second C pass of the after build is byte-identical (0 log mismatches). These fixtures read C-private state, so they cover the C side only |
| Differences from the consumer's rows | glyphflow `center` and every text-287 mode already pass **before**, because borg-rs carried local #345/#287 before this batch. text-287's `t287-variant.h` is `1` for both oracles, since borg-rs's before tree already had the sized API. iswarded `rune-town`/`hit-town` crash before the fix with SIGSEGV in the normal phase, where the consumer gets SIGABRT: our oracles are not built with assertions. Under ASan they report the heap-buffer-overflow. After the fix all modes pass. Every other outcome row equals the consumer's |
| Existing native checker and `CHECK_UBSAN=1` | pass. This is a regression check only: none of the functions M7 changes is extracted |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | C and Rust 131/131 completed (DONE). **131/131 C/Rust parity** on keys, messages, state and full native saves; `harness/diff` exit 0; `matrix.py` PASS |
| Behaviour change vs the milestone6 run on the same plan | 119/131 cases differ from the local milestone6 run (`harness/out/c-m6/g-c`, recorded as `reference`). Deaths: 25 (M6) → 20 (M7). This is an observation, not a survival claim. [reference-m6-c-summary.json](reference-m6-c-summary.json) holds that run's per-case stream digests and stop/death fields, enough to re-derive the 119-case set and the 25 deaths without the raw run |
| Per-fix Rust fault controls ([fault-suite](fault-suite), a 23-case **subset**, built at `c9a198c`) | all 12 behavioural units DONE; **3 detected**: 0064 (23/23), 0071 (19/23), 0069 (13/23). 0066, 0067, 0068, 0070, 0073, 0074, 0075, 0076 and 0077 are not detected on this subset; non-detection is not proof of non-execution. 0065's Rust unit is the comment only (it is absent from the `c9a198c` worktree, hence APPLY-FAIL), so there is nothing to fault |
| Removal units | 26 (13 per side). Every hunk of the implementation diff is in exactly one unit. Each unit reverse-applies on its own on the final tree. Reversing all of them restores `4.2.6-impact-v6` exactly for the changed C and Rust files |

Rust-side controls beyond the parity matrix: 0071 (`parsed_pain`). For the
other fixes, the Rust evidence is translation review plus the parity matrix,
and for 0064/0069/0071 also the fault controls. The thinnest are 0066, 0067,
0068, 0070 and 0073-0077. Of those, 0068 and 0070 change only error and
out-of-level paths, which the game plan is not expected to reach.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; maximum Rust wall time 210.9 s.
The game matrix ran once per image, plus the Rust rerun at `7efe65c`. No new
target reference is exported. Raw outputs are in `harness/out/c-m7`, outside
Git.
