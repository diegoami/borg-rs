# Carry C milestone9 fixes into C oracle and Rust

Owner direction (2026-11-09): after `4.2.6-impact-v8`, continue with C
milestone9. Tracking [Rust#21](https://github.com/diegoami/borg-rs/issues/21);
record [adoption/c-m9](../../adoption/c-m9/README.md). Target better-borg-c
`33207fbeeb582e67d6b18e9f40613617667905ae` (`4.2.6-impact-v9`): patches
0090-0099. 0098 replaces local #295. 0098 and 0099 each carry a local crash
guard (FORK-CORRECTIONS). Start: Rust main `286215c` (`4.2.6-impact-v8`).
Implementation commits: `d7e5985` (the carry and the 0098 guard) and
`b89c106` (the 0099 guard). This is author validation, not independent review
or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
`9e34120` (`git_dirty=no`, `inputs_dirty=no`). For every build input it equals
`b89c106`; the commit in between changes only the 0099 removal units.
`harness/check-no-c-borg.sh` finds no C Borg code in the Rust image. The
builder is Ubuntu cc 13.3.0 and rustc 1.97.0, as recorded in the BUILD_IDs.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 101 passed, 1 existing ignored / no warnings |
| **C-oracle** native fixtures: 11 fixtures (ten consumer, one local), 41 normal and 12 ASan modes ([native-comparison.json](native-comparison.json)) | normal: before 23/41, after **39/41**; 16 fail→pass, 23 pass on both, none pass→fail. The two modes red on both sides are runeglyph `floor`/`trap`, which the consumer's verify script also asserts red in both phases (disclosed there). ASan: before 6/12, after **12/12** (the panelrows tall/wide overruns). Every outcome row the consumer's scripts assert matches (60 checks), except one expected difference: runeglyph `rune` already passes **before**, because borg-rs's before tree carried local #295. A second C pass of the after build is byte-identical. These fixtures read C-private state, so they cover the C side only |
| 0098 guard (local fixture `staleglyph`) | On the consumer core without the guard, mode `stale` crashes (SIGSEGV; ASan SEGV in `borg_update`) and `fresh` passes. On the v8 tree (#295) both pass, and with the guard both pass, normal and ASan. The unguarded rows are recorded in native-comparison.json |
| 0099 guard | The first 131-case run, on the build without it, failed 9 cases in both roles: the eight Necromancer cases and `cfg-munchkin-dunadan-bg`. C crashed with SIGSEGV, Rust aborted on its bounds assert ([matrix-unguarded0099.json](matrix-unguarded0099.json)). A gdb trace and a temporary print showed `borg.c` from the previous level after a level change. With the guard, all eight Necromancer cases complete with C/Rust parity |
| Existing native checker and `CHECK_UBSAN=1` | pass; run before the 0099 guard, which touches no extracted function |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | **130/131 cases complete with full C/Rust parity** on keys, messages, state and native saves. `harness/diff --allow-incomplete`: 130/131 runs identical. The remaining case, `cfg-munchkin-dunadan-bg`, timed out in **both** roles (600 s, rc=124) in a munchkin stair livelock; see below. With that case incomplete, `harness/diff` without `--allow-incomplete` refuses (exit 2), and `matrix.py` reports FAIL by its all-complete rule |
| `cfg-munchkin-dunadan-bg` livelock ([livelock-cfg-munchkin-dunadan-bg.json](livelock-cfg-munchkin-dunadan-bg.json)) | The game turn freezes at 19045. The Borg is blind (`TMD_BLIND`, 12 turns left), flees every level in munchkin mode, and takes the stair it stands on; stair travel does not age the world clock, so the blindness cannot wear off. C and Rust are identical for every line up to the Rust kill (415,527 keys, 1,054,501 messages, 415,528 state lines); they differ only at the kill. The M9 game diverges from M8 at turn 3298, where 0092 quaffs Cure Light Wounds instead of retreating (0092 as intended). This is pre-existing munchkin behaviour, the same family as 0067 (poison/hallucination), not covered for blindness. Owner decision (2026-11-09): record and report, no behaviour change. Reported on [better-borg-c#204](https://github.com/diegoami/better-borg-c/issues/204#issuecomment-6749862654) |
| Behaviour change vs the milestone8 run on the same plan | 57/130 completed cases differ from the local milestone8 run (`harness/out/c-m8/g-c`). Deaths over those 130: 18 (M8) → 13 (M9). This is an observation, not a survival claim. [reference-m8-c-summary.json](reference-m8-c-summary.json) holds that run's per-case digests and stop/death fields |
| Per-fix Rust fault controls ([fault-suite](fault-suite), 23-case **subset**, built at `b89c106`) | All 10 units DONE; **2 detected**: 0094 (3/23) and 0092 (1/23). 0090, 0091, 0093, 0095, 0096, 0097, 0098 and 0099 are not detected on this subset; non-detection is not proof of non-execution. The subset has no Necromancer case and no large-terminal case, so neither 0099 nor 0097 is reached; the 0099 guard's effect shows in the full plan above |
| Removal units | 20 (10 per side). Every hunk of the implementation diff is in exactly one unit. The 0098 and 0099 units include their guards. Each unit reverse-applies on its own on the final tree. Reversing all of them restores `4.2.6-impact-v8` exactly for the changed C and Rust files |

Rust-side evidence beyond the parity matrix:
- 0092 and 0094: the fault controls.
- 0099 and its guard: the full plan (8 Necromancer cases; Rust aborted without the guard).
- 0098 guard: translation review; the staleglyph fixture covers C only.
- 0090, 0091, 0093, 0095, 0096 and 0097: translation review plus the parity matrix. This is the thinnest part of the batch.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; the maximum Rust wall time of a
completed case is 222.3 s. The game matrix ran twice per image: once without
the 0099 guard (kept as evidence) and once with it. No new target reference
is exported. Raw outputs are in `harness/out/c-m9`, outside Git.
