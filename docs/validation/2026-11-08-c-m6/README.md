# Carry C milestone6 fixes into C oracle and Rust

Owner direction (2026-11-08): after `4.2.6-impact-v5`, proceed to C
milestone6. Tracking [Rust#15](https://github.com/diegoami/borg-rs/issues/15);
record [adoption/c-m6](../../adoption/c-m6/README.md). Target better-borg-c
`b3c825c97c8aef18b252a7fb7d9b7dceef83d586` (`4.2.6-impact-v6`). Start: Rust
main `6b58a46` (`4.2.6-impact-v5`). Implementation commit `30d1e4e`. This
is author validation, not independent review or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
the committed `30d1e4e` (`git_dirty=no`, `inputs_dirty=no`). Later commits
change only docs and evidence. `harness/check-no-c-borg.sh`: no C Borg code in
the Rust image, and the engine objects are byte-identical between the two
images except `buildid.c.o`, which the script expects to differ. There are no
engine, data, header or FFI changes. Builder: Ubuntu cc 13.3.0, rustc 1.97.0
(recorded in the BUILD_IDs). `binary_sha256` is specific to that toolchain;
`source_sha256` is the reproducibility anchor across environments.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 100 passed, 1 existing ignored / no warnings. `parses_like_c` now expects 127 for `borg_no_deeper -` (0058) and has a 0057 case; these are Rust-side controls for the config fixes |
| **C-oracle** native fixtures, 14 fixture entries / 31 modes ([native-comparison.json](native-comparison.json)) | before 13/31, after **30/31**; 17 fail→pass, 13 pass on both sides. The one red-both mode is the original recall-mana `lowmana`, which the consumer also asserts red because 0053 flees first; the adapted fixture (`recallmana-adapted`, one recorded line change) flips `lowmana` fail→pass, `restore_mana` 0→1. The config fixtures' bad-input modes go from exit 2 (hang caught) and exit 3 (crash) to exit 0. A second C pass is byte-identical (0 log mismatches). These fixtures read C-private state, so they cover the C side only |
| Control-mode logs | most controls are byte-identical before and after. Some differ (e.g. recall-mana `halfmana`, count-sell `light`) because "before" is the whole impact-v5 tree, so it lacks 0053's flee and other M6 changes, whereas the consumer compared each carry against the tree just before it. Those modes still pass, with the same outcomes |
| Existing native checker and `CHECK_UBSAN=1` | pass (regression only: no function M6 changes is extracted) |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | C and Rust 131/131 completed (DONE). **131/131 C/Rust parity** on keys, messages, state and full native saves; `harness/diff` exit 0; `matrix.py` PASS |
| Behaviour change vs the milestone5 run on the same plan | 109/131 cases differ from the local milestone5 run (`harness/out/c-m5/g-c`, recorded as `reference`). Deaths: 26 (M5) → 25 (M6). This is an observation, not a survival claim. [reference-m5-c-summary.json](reference-m5-c-summary.json) holds that run's per-case stream digests and stop/death fields, and re-derives the same 109-case set and the 26 deaths without the raw run |
| Per-fix Rust fault controls ([fault-suite](fault-suite), 23-case **subset**, built at `30d1e4e`) | all 12 units DONE; **4 detected**: 0053 (20/23), 0055 (18/23), 0056 (17/23), 0060 (1/23). 0051, 0052, 0054, 0057-0058, 0059, 0061, 0062 and 0063 are not detected on this subset; non-detection is not proof of non-execution. These are subset counts, not full-plan detection power |
| Removal units | 24 (12 per side; 0057/0058 share one). Each reverse-applies on its own on the final tree; reversing all of them restores `4.2.6-impact-v5` exactly for the changed C/Rust files |

Rust-side controls beyond the parity matrix: 0057 and 0058 (`parses_like_c`).
For 0051, 0052, 0054, 0059, 0061, 0062 and 0063 the Rust evidence is
translation review plus the parity matrix; this is the thinnest part of the
batch. Follow-ups from the review: a direct Rust test of the 0052 clock-reset
branch and a seam test of the 0059 reopen failure. Both need engine state
(the think loop, file-open failure injection), so they are recorded rather
than added here.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; maximum Rust wall time 206.5 s.
The game matrix was run once per image, and no new target reference is
exported. Raw outputs are in `harness/out/c-m6`, outside Git.
