# Carry C milestone5 fixes into C oracle and Rust

Owner direction (2026-11-08): after `4.2.6-impact-v4`, proceed to C
milestone5. Tracking [Rust#13](https://github.com/diegoami/borg-rs/issues/13);
record [adoption/c-m5](../../adoption/c-m5/README.md). Target better-borg-c
`5384f12ed51e8fef6f31353fd2daaf0e7fb43856` (`4.2.6-impact-v5`). Start: Rust
main `d3e94d3` (`4.2.6-impact-v4`). Implementation commit `7a0728f`. This
is author validation, not independent review or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
the committed `7a0728f` (`git_dirty=no`, `inputs_dirty=no`). Later commits
change only docs, the batch record and the native checker's extraction list.
`harness/check-no-c-borg.sh`: no C Borg code in the Rust image, and the engine
objects are byte-identical between the two images. There is no engine C
change. The only interface addition is the Rust FFI binding for the existing
engine function `square_islit`.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 100 passed, 1 existing ignored / no warnings. The `tvals` unit test now pins gloves as armour (0045) |
| **C-oracle** native fixtures, 7 fixtures / 16 modes ([native-comparison.json](native-comparison.json)) | before 9/16, after **16/16**; 7 fail→pass, and the 9 controls pass on both sides with byte-identical logs. glowtrace-213 reproduces the consumer's `glow_n=484` after. These fixtures read C-private Borg state, so they cover the C side only |
| flowcache-404 contract (0049), 30 cases ([before](flowcache-before.json), [after](flowcache-after.json)) | run with the evidence's own case runner and `--wrap` flags. Before: 19/30, failing exactly the evidence's `EXPECTED_FAIL["before"]` (the 11 contract cases); after: **30/30**, matching `EXPECTED_FAIL["after"]` (empty); every process exits 0. The consumer's partial-fix variants (neg-*, alt-wipe) are not rerun here |
| Existing native checker and `CHECK_UBSAN=1` | pass: 30,000 calculator cases, 2×2,000 spatial states, 2,600 glyph-builder runs and 1,280 lunal-thinker states; twelve real-source fault controls rejected. The spatial kernel compares the actual C and Rust `borg_flow_spread`/`borg_flow_enqueue_grid` with randomised desperate/digging/lunal/munchkin flags and full flow matrices, so it is **direct C/Rust evidence for 0049**. The checker's extraction list needed the new Rust helper (`borg_flow_ignores_danger`); that was the only change |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | C and Rust 131/131 completed (DONE). **131/131 C/Rust parity** on keys, messages, state and full native saves; `harness/diff` exit 0; `matrix.py` PASS |
| Behaviour change vs the milestone4 run on the same plan | 121/131 cases differ from the local milestone4 run (`harness/out/c-m4/g-c`, recorded as `reference`). Deaths over the plan: 28 (M4) → 26 (M5). This is an observation, not a survival claim |
| Per-fix Rust fault controls ([fault-suite](fault-suite), 23 cases, built at `7a0728f`) | all 8 suites DONE; **2 detected**: 0050 (23/23 cases), 0049 (6). 0043–0048 are not detected on this subset; non-detection is not proof of non-execution. 0049 also has the kernel-level evidence above |
| Removal units | all 16 reverse-apply alone on the final tree; reversing all of them restores `4.2.6-impact-v4` exactly for the changed C/Rust files (including the FFI binding) |

Rust coverage for 0043–0048 rests on translation review plus the parity
matrix; there is no Rust-side behavioural control for them.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; maximum Rust wall time 239.3 s.
Only one C pass was run, and no new target reference is exported. Raw outputs
are in `harness/out/c-m5`, outside Git.
