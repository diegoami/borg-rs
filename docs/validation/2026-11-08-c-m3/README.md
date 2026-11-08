# Carry C milestone3 fixes into C oracle and Rust

Owner direction (2026-11-08): proceed from the milestone2 carry to C
milestone3. Tracking [Rust#8](https://github.com/diegoami/borg-rs/issues/8);
batch record [adoption/c-m3](../../adoption/c-m3/README.md). Target better-borg-c
`702f091f6ac06cae1014df2f85ac218be0189813`, approved with findings on
[better-borg-c#128](https://github.com/diegoami/better-borg-c/issues/128#issuecomment-6705894431).
Start: Rust main `0bc7a09` (milestone2 carried). Implementation commit
`4fe376e`. This is author validation, not independent review or acceptance.

## Identities

`C-BUILD_ID`/`RUST-BUILD_ID` were written from the working tree before the
implementation commit, so they read `git_rev=0bc7a09`, dirty. Their
`source_sha256` values (C `89cfb798…`, Rust `1885ad89…`) equal
`harness/identity.sh source-hash` of the committed `4fe376e` inputs (checked
after commit; only docs/harness files were untracked). Binaries: C
`55f67326…`, Rust `aac44ccd…`. `harness/check-no-c-borg.sh`: the Rust image
contains no C Borg code; 154/154 engine objects byte-identical. No engine,
data, header or FFI change.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` | 100 passed, 1 ignored (existing) |
| `cargo clippy --release --all-targets` | no warnings |
| Native C fixtures, 26 fixtures / 64 modes ([native-comparison.json](native-comparison.json)) | before 37/64, after 62/64; 26 fail→pass, 36 pass on both sides |
| — the two remaining reds | dispel-259 `reserve` −10331 (red before too) and `clear` 169 (pass→fail): exactly the post-losdispel values the C package pins as asserted-red cross-carry interaction |
| ASan phase (detect_leaks=0) | before: glyph-tracker `stairs` heap-buffer-overflow, missile-254 `no-target` heap-buffer-overflow, glow-311 `glow` global-buffer-overflow; after: no report, all pass. tp-other and detect-290 asan modes flip fail→pass without a report |
| LSan phase | before: calc-reload 320 B/4 allocations, opblock 160 B/2, calfree 160 B/2; after: no leaks reported |
| Existing native checker (`harness/native-checker/run.sh`, plus `CHECK_UBSAN=1`) | pass: 30000 calculator, 2×2000 spatial (live-extracted `borg_projectable_pure`, thrust), 2600 glyph-builder, 1280 lunal-thinker states (live-extracted `borg_think_stair_scum` with the new flag clear), twelve real-source fault controls |
| 131-case game matrix, exact milestone2 plan ([adoption-suite](adoption-suite), [matrix.json](matrix.json)) | C 131/131 and Rust 131/131 completed, DONE; **131/131 C/Rust parity** on keys, messages, state and full terminal native save; `harness/diff` exit 0 |
| Behaviour change vs milestone2 reference | 79/131 cases differ from the committed M2 target reference, 52 unchanged. Deaths 24 (M2) → 23 (M3); this is a regression observation, not a survival claim |
| Per-fix Rust fault controls ([fault-suite](fault-suite), 23 changed cases) | each of the 22 Rust removal units reversed alone, rebuilt, all 22 suites DONE; **6 detected** by C mismatch: 0008 (19 cases), 0029 (14), 0017 (3), 0020, 0022, 0030 (1 each). 16 not exercised by this subset |
| Removal units | all 47 reverse-apply alone on the final tree; reversing all restores `0bc7a09` exactly for the changed files |

Fault-control interpretation: the 131-case parity establishes Rust = C on
every path those games reach, including the 79 changed cases. The six
detected controls prove the matrix sees those translations. The other 16
fixes (mostly rare spell/activation/edge paths) are not reached by the 23-case
subset; for them the evidence is the C native fixtures plus line-by-line
translation review against the removal units. No Rust-side native fixture
exists for them (the fixtures inspect C private state). This is the main
limitation for review.

Game budgets: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600` (milestone2 used 300; turn
caps, seeds and inputs are identical). Maximum Rust wall time 264.6 s, so no
case relied on the larger budget. Only one C pass was run; no new target
reference is exported, so the two-pass oracle rule for references is not
claimed.

## Reconciliation decisions to review

- Local #290 equality reset retired in favour of consumer patch0018 (reason in
  the batch README). seed4006 `ext-rogue` (2M turns, the original crash case)
  completes in C and Rust with identical logs and save.
- 0025 not imported (local #287 is stronger); 0027 = local #322.
- 0026/0033/0034 C-only; Rust has no manual allocation there.

Raw outputs: `harness/out/c-m3` (oracles, native-before/after, g-c, g-rust,
faults), outside Git.
