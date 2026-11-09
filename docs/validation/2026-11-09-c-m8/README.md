# Carry C milestone8 fixes into C oracle and Rust

Owner direction (2026-11-09): after `4.2.6-impact-v7`, continue with C
milestone8. Tracking [Rust#19](https://github.com/diegoami/borg-rs/issues/19);
record [adoption/c-m8](../../adoption/c-m8/README.md). Target better-borg-c
`017e038e8726a7a423887a366b5d40e89c0550a1` (`4.2.6-impact-v8`): patches
0078-0089, plus the owner's munchkin-by-class change (PR302), carried on the
owner's decision. Start: Rust main `ce27f5e` (`4.2.6-impact-v7`).
Implementation commit `7fe1482`. This is author validation, not independent
review or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
`cf20c85` (`git_dirty=no`, `inputs_dirty=no`). `cf20c85` equals `7fe1482` for
every Borg source and for `harness/borg.txt`; the commits in between add only
the fixture import, docs and removal units. `harness/check-no-c-borg.sh`:
there is no C Borg code in the Rust image. Builder: Ubuntu cc 13.3.0 and
rustc 1.97.0, as recorded in the BUILD_IDs.

`harness/borg.txt`, the stock configuration of every planned run, now equals
the consumer's `borg.txt`. It adds the two munchkin-by-class settings, so the
suite manifests' `borg_txt_sha256` differs from the milestone7 run.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 101 passed, 1 existing ignored / no warnings. New `munchkin_class_rule` (PR302); `default_file_matches_c` expects the two new settings |
| **C-oracle** native fixtures, 12 fixtures / 31 modes ([native-comparison.json](native-comparison.json)) | before 19/31, after **31/31**; 12 fail→pass, 19 pass on both sides, none pass→fail. Every outcome row equals the row the consumer's verify script asserts. Controls are byte-identical, including all modes of the two cleanups (neckdead, hurtstun). The spasticscore product fixture's `mismatch` mode passes in both phases by its own contract, and its outcome flips as the consumer asserts (`rc=0 goal_x=12` → `rc=1 goal_x=13`). A second C pass is byte-identical (0 log mismatches). These fixtures read C-private state, so they cover the C side only |
| Existing native checker and `CHECK_UBSAN=1` | pass. The spatial kernel compares the actual C and Rust `borg_flow_spread`, with a randomised goal shop, shop numbers and terrain, so it is direct C/Rust evidence for the 0080 shop-entrance test |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | C and Rust 131/131 completed (DONE). **131/131 C/Rust parity** on keys, messages, state and full native saves; `harness/diff` exit 0; `matrix.py` PASS |
| Behaviour change vs the milestone7 run on the same plan | 117/131 cases differ from the local milestone7 run (`harness/out/c-m7/g-c`). Much of this is PR302. Munchkin-mode messages now appear in all 9 Druid runs, all 9 Ranger runs and 34 of 50 Warrior runs (at milestone7: 0, 0 and 2, the explicit munchkin-config cases). The other classes are unchanged in this respect: only their existing munchkin-config cases show it. The 16 Warrior runs without munchkin text are the `end-*` and `glyph-*` fixtures (which start deep), `natural-warrior-78` and `opt-scum-war`. [munchkin-message-cases.json](munchkin-message-cases.json) records the per-case set for both runs. Classes are the class at the end of each run (`summary.json`), and the breakdown uses the milestone8 classes; one case, `opt-respawn-random-druid` (random respawn class), ends as a Priest at milestone7 and a Druid at milestone8. Deaths: 20 (M7) → 18 (M8). By class: Warriors 2 → 2 (50 cases), Druids 4 → 3 (9), Rangers 4 → 2 (9), the other classes 10 → 11 (63). These are observations, not a survival claim. [reference-m7-c-summary.json](reference-m7-c-summary.json) holds that run's per-case stream digests and stop/death fields; it re-derives the 117-case set and the 20 deaths without the raw run |
| Per-fix Rust fault controls ([fault-suite](fault-suite), a 23-case **subset**, built at `7fe1482`) | all 13 units DONE. **6 detected**: 0081 (16/23), 0080 (15/23), PR302 (15/23), 0089 (12/23), 0084 (4/23), 0082 (2/23). 0078, 0079, 0083, 0085, 0086, 0087 and 0088 are not detected on this subset. 0083 and 0085 remove dead code, so they cannot be detected. Non-detection is not proof of non-execution. The full disk interrupted 0089 and pr302 in the first pass; both were rerun cleanly on their own (`harness/out/c-m8/fault-rerun.sh`, outside Git) |
| Removal units | 26 (13 per side). Every hunk of the implementation diff is in exactly one unit. Each unit reverse-applies on its own on the final tree. Reversing all of them restores `4.2.6-impact-v7` exactly for the changed C, Rust and `harness/borg.txt` files |

Rust-side evidence beyond the parity matrix:
- PR302: the `munchkin_class_rule` unit test plus the fault control.
- 0080: the native checker plus the fault control.
- 0081, 0082, 0084 and 0089: the fault controls.
- 0078, 0079, 0086, 0087 and 0088: translation review and the parity matrix.
  This is the thinnest part of the batch.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; maximum Rust wall time 231.5 s.
The game matrix ran once per image, and no new target reference is exported.
Raw outputs are in `harness/out/c-m8`, outside Git. To free the session's
disk allowance, the raw outputs of the accepted milestones 4-6 and the
milestone7 Rust/fault/rerun directories were deleted. Their committed digests
(matrix.json, reference summaries, rust-reruns.json) remain.
