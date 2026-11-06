# Self-review before the external review

Three independent read-only reviews were run on the work branch at
`590783e1f` (44 modules converted). Each reviewer got the code, not the
author's conclusions:

1. **Faithfulness**: converted modules vs the C, line by line (borg_danger,
   borg_prepared, borg_escape, borg_flow, borg_item_analyze,
   borg_home_power in full; spot checks in six more).
2. **FFI and unsafe**: soundness of `engine.rs`, `bridge.rs`, `state.rs`
   and the entry point.
3. **Harness and evidence**: whether the harness, the references and the
   docs support the claim "Rust Borg == C Borg, key for key".

Every finding was checked again before it was acted on. This file lists
each one with its verdict and what was done. "Fixed in" names the commit.

## 1. Faithfulness

**No behavioural divergence found.** Every C bug listed in `c-bugs.md` for
the reviewed modules is present in the Rust as described.

| id | finding | verdict | action |
|----|---------|---------|--------|
| S1 | `borg_danger_one_kill`: the Teleport Other skip list is read with `<=` (C bug 17); with 255 entries C reads one `int` past `int[255]`, Rust panics | real, not reachable (needs 255 candidates in one scan) | documented in c-bugs 17 (`c4969c0c5`) |
| S2 | `bridge::borg_detect_wall` checked against `borg.panels`, the arrays are sized by `old_panels` | real, latent | function removed, callers use `borg_detect` which checks `old_panels` (`c4969c0c5`) |
| S3 | `-1` index in flow_glyph / flow panics instead of reading out of bounds | already documented (c-bugs 29, 32) | none |
| C1 | `borg_prepared` built unique names with `from_utf8_lossy` | cosmetic (names are UTF-8, so output was identical) | reason texts are bytes now (`c4969c0c5`) |
| C2 | `count_scary` declares `x`/`y` per attempt, C at function scope | equivalent (always assigned before read) | none |
| C3 | `borg_danger` reads some c2rust globals once per monster | equivalent today (nothing in danger writes them) | none; noted here for whoever converts `borg_fight_attack` |

## 2. FFI boundary and unsafe code

Panics across FFI are not an issue: both profiles use `panic = "abort"`.

| id | finding | verdict | action |
|----|---------|---------|--------|
| U1 | Re-entrancy: a Borg `msg()` (warning) with `auto_more` off can reach `msg_flush` -> `anykey` -> `inkey_ex` -> the Borg hook, running a nested Borg turn while the outer one holds `&mut BorgState` | real path; C has the same nesting (no guard in `inkey_ex`), so a guard would change behaviour | nesting kept as in C. Engine callbacks enter through `bridge::with_entry_state`, which **reports a nested entry on stderr** (kept per run in `stderr.log`). `msg`/`get_string`/message-flush calls from Borg code go through bridge wrappers that `escape` the state; the warning shim copies its text (it may live in `format()`'s buffer). Nested entries seen: none in smoke, full, deep (see below) (`c4969c0c5`). **Bridge removal:** the nesting is now sound: the engine wrappers that can reach the hook (or the bell / a message) take the caller's state and lend it, and the nested `engine::enter` reborrows the lent reference; a nested entry from a non-lending call panics after the stderr report. Shown on a local smoke run with a forced long `msg` pair every 500 turns (7 nested entries, run completes) and with the lend removed from `msg` (panic) |
| U2 | `bridge::st()` was a safe `pub fn` returning `&'static mut` | real (formally UB under Stacked/Tree Borrows when two are live) | now `pub(crate) unsafe fn` with a `# Safety` contract; `state_ptr()` for raw access. The st()+`escape` scheme stays formally UB until c2rust is gone; `escape` only stops today's optimiser. This is transitional and disappears with `bridge.rs` (`c4969c0c5`). **Bridge removal:** done. `st()`, `state_ptr()`, `escape` and `bridge.rs` are gone; `engine::enter` is the only root and Borg code only receives `&mut BorgState` |
| U3 | `square_isvault` / `square_isglow`: no bounds check (the engine's `assert` is compiled out by `-DNDEBUG`) | real | `assert!(square_in_bounds)` (`c4969c0c5`) |
| U4 | `screen_at`: no bounds check | real | asserts `0 <= x < wid`, `0 <= y < hgt` (`c4969c0c5`) |
| U5 | `class_book_dungeon`: `debug_assert!` only | real | goes through the checked `class_book_ref` (`c4969c0c5`) |
| U6 | `feat_is_trap_holding`: unchecked feature index | real | range assert as in `feat_is_shop` (`c4969c0c5`) |
| U7 | safe fns dereference the raw `borg_item.note` | real, transitional | **Fixed at the bridge removal:** `note` is an offset into `desc` (`Option<u8>`). C semantics (a copy's note reads the original's `desc`) are kept because the original's `desc` never changes while a copy is read; argument and a differential check (old pointer vs offset on every real read, 0 mismatches) in PROGRESS "bridge removal" |
| L1/L2 | `borg_item_analyze_ware` called a `st()` shim without `escape`, passing a pointer into the state | real, latent | calls the converted analyzer directly with split field borrows; the shim now borrows only the two tables (`c4969c0c5`) |
| L3 | `borg_detect_wall` checks the wrong size | real, latent | = S2 |
| L4 | engine object handles are `Copy` without a lifetime | real, latent (none is stored today) | not changed; rule: never store a handle in `BorgState`. **Bridge removal:** still latent and documented. A lifetime would have to come from a borrow that ends when the engine can change the object, i.e. any engine call; tying it to the `&mut BorgState` would forbid the common "read object, update state" pattern. Checked: no struct of the crate has a field of a handle type (`GameObject`, `CarriedObject`, `StoreObject`, `FloorObject`, `AngFile`, `Menu`) |
| L5 | `&'static` refs into engine tables that change at runtime (`kind.ignore`, `cur_num`) | real, latent (all uses are transient) | not changed; same rule: never hold across a bridge or engine call. **Bridge removal:** still latent and documented (fixing means replacing 26 accessors and their users with copies). Checked: the 11 `k_info()` users outside engine.rs hold the reference at most within one function; the only engine calls made while one is held are notes (borg_flow_take's lunal note), whose message display does not write `k_info`; the only Borg writer of `kind.ignore`, `set_kind_ignore`, runs in borg_init with no reference held. (A nested Borg turn inside such a note could in theory reach it; the suites never nest) |
| L6 | `all_letters*` declared as `[c_char; 0]` | real (reads outside the static's provenance) | declared with their C lengths, 53 and 49 (`c4969c0c5`) |
| L7 | shims pass a slice into the state together with a fresh `st()` | real, latent | the texts are copied first (`c4969c0c5`) |
| L8 | `msg_buf()` was a safe fn calling `st()` | real | `unsafe fn` (`c4969c0c5`) |
| C1-C2, C4-C5 | comment and documentation precision | cosmetic | partly addressed in the touched code |
| C3 | the "no unsafe in converted modules" rule was not enforced | real gap | `#![deny(unsafe_code)]` at the crate root, allowed only in `engine`, `headers`, `bridge`, `c2rust` (`c4969c0c5`); since the bridge removal only in `engine` and `headers` |

## 3. Harness and evidence

The references hold up: every stored C output still diffs identical to
them, 153 of 154 engine objects were byte-identical between the two builds
(only `buildid.c.o` differed), and the reference history shows no edits.
But the evidence had real gaps.

| id | finding | verdict | action |
|----|---------|---------|--------|
| E6 | The birth-script letter quirk hits classes too: class 7 stays Warrior, class 8 plays Ranger. **Blackguard was never played**; Dunadan, High-Elf and Kobold never appeared. "9 classes" / "every race" were wrong | real coverage gap | opt-in `-menu-letters` (default unchanged, so old references stay valid); new suite `classes` (16 runs: Blackguard 8 times incl. a 1M-turn cheat-death run, the three missing races with 7 classes) with its own C reference, made twice (identical). **Rust: 16/16 identical** (`a0858e744`, `fe38227de`) |
| E1 | `harness/diff` compared only the runs present in its first argument; a missing Rust run passed silently | real | compares the union, `--suite NAME` requires the exact run set, prints `(expected N)` (`f8633d84e`) |
| E2, E3 | `suite.sh` reused its output dir (stale runs could pass) and stopped at the first failed run (no MISSING report) | real | fresh dir moved into place, `manifest` (git rev, binary sha256, impl, suite hash) and `DONE`/`FAILED` marker; keeps going after failures; diff refuses dirs without `DONE` (`b1775ac54`, `f8633d84e`) |
| E4 | `check-no-c-borg.sh`: its whitelist grep matched every archive member, the strings check could never fire, empty inputs passed | real | archive members filtered by rustc name and `.comment`, explicit compiler-builtins list, fails on empty or stripped inputs, compares all engine objects byte for byte; shown to fail on 7 broken inputs (`a3ef10397`). Not done: a link-map check, so deliberately renamed C code would still pass |
| E5 | references do not record their provenance; `build.sh` never passed `-DBORG_IMPL=c` | real | `build.sh` always passes it; suite outputs carry a manifest. Old references keep no binary hash (they predate this) |
| M1, M3, M4 | diff was line-based, ignored `summary.json`, did not cross-check key counts | minor | raw byte compare; summaries compared (except `wall_seconds`); log lines = `keys` and last turn = `game_turns` on both sides |
| M2 | trailing whitespace in a suite file could merge lines | minor | suite files read with `read`, comments and field counts checked |
| M5 | TZ not pinned | minor (harmless) | `TZ=UTC` |
| M6 | no uninitialised-memory audit of the C oracle | minor | C smoke under valgrind memcheck: 0 errors, keys identical |
| M7 | smoke reference determinism not shown | minor | C smoke run twice, identical |
| E7 | the oracle sees only keys: notes, direct state writes and config-dependent paths (munchkin, dynamic calcs, respawn) are not checked | real, open | documented in REVIEW.md. A notes log and a `config` suite are possible next steps |
| E8 | per-module cross-checks were never committed | real, open | documented in REVIEW.md; for code the suites never reach they are the only evidence |
| I1-I10 | wrong or stale claims in STATUS/REVIEW/harness.md/PROGRESS (coverage numbers, C bug numbers off by one or two, rows out of order, unsafe table wrong in most rows, a checkpoint label) | real | all corrected; the unsafe table is now recomputed from each commit (`c6d61faa6` and earlier docs commits, `a59054946`) |

## 4. Checks after the fixes

On `c4969c0c5` (45 modules plus the fixes above): `cargo test` and
`clippy` clean, smoke 3/3, `full` 45/45 and `deep` 9/9 identical to the C
references. No run of any suite printed a nested Borg entry, so the U1 path
is not reached by the suites. `main` was moved to this commit.

After the harness fixes were merged (branch tip `ed86d53c8`, 46 modules):
the new `check-no-c-borg.sh` passes (309 rustc members, 35 known
compiler-builtins members, 153/154 engine objects byte-identical plus
`buildid.c.o`), `smoke` 3/3 and the new `classes` suite 16/16 identical to
the C references, no nested Borg entry.

## 5. External review `REVIEW-4c06cec08`, finding 4

**Finding:** a Rust-labelled suite passed with the C binary
(`BORG_IMPL=rust ANGBAND_BIN=build/c/game/angband harness/suite.sh smoke`,
then `harness/diff`: 3/3, exit 0), although the manifest said
`borg_impl_env=rust` and `binary_cmake_borg_impl=c`; `harness/diff` never
read the manifest, and nothing tied the binary to the recorded source
revision.

**Verdict:** confirmed. Reproduced on `4c06cec08` in this worktree: suite
exit 0 with `DONE`, diff `3/3 runs identical (expected 3)`, exit 0.

**Fix** (`docs/harness.md`, "Build identity"):

* `harness/build.sh` writes `build/<impl>/BUILD_ID`: implementation, git
  revision and dirty flags, sha256 of the build inputs as they are in the
  working tree (`harness/identity.sh source-hash`: `angband/` for C, plus
  `borg-rs/`, `Cargo.toml`, `Cargo.lock` for Rust), and the linked
  binary's sha256.
* `harness/suite.sh` refuses before any run (exit 2, no output dir) when
  the implementation read from the executable (`.comment` names rustc, or
  `do_cmd_borg` without rustc), its `CMakeCache.txt` and its `BUILD_ID`
  disagree or differ from `BORG_IMPL`; when none knows it
  (`ALLOW_UNKNOWN_IMPL=1` overrides); and when `BUILD_ID` is missing, does
  not match the binary's sha256, or records other sources than the
  checkout's (`ALLOW_STALE_BUILD=1` overrides). Overrides are recorded in
  the manifest. The binary is hashed again after the runs.
* `harness/diff` prints each suite output's implementation and build
  identity, fails on an implementation mismatch, a recorded override or
  (with `--expect-impl c|rust`) another implementation, and refuses an
  output dir without a manifest or with one from before this change.
  `--allow-unverified` makes these warnings. References stay accepted.
* `harness/check-no-c-borg.sh` step 5 (the review's link-provenance
  remark): it re-runs the build's link command with `ld --trace --trace`,
  requires the result to be byte-identical to the binary, and requires
  every link input to be an engine object of step 4, a staticlib member
  accepted by step 2, or a system file.

No key stream, reference, seed, suite file or `summary.json` field
changed; nothing was added to the binary.
