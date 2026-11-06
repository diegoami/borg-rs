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
| L4 | engine object handles are `Copy` without a lifetime | real, latent (none is stored today) | not changed; rule: never store a handle in `BorgState`. **Bridge removal:** still latent and documented. A lifetime would have to come from a borrow that ends when the engine can change the object, i.e. any engine call; tying it to the `&mut BorgState` would forbid the common "read object, update state" pattern. Checked: no struct of the crate has a field of a handle type (`GameObject`, `CarriedObject`, `StoreObject`, `FloorObject`, `AngFile`, `Menu`). **Fixed after the external review (finding 3):** the four object handles carry a lifetime `'a`, the shared borrow of a token field of the state (`st.access: engine::Access`) that their constructors take (`inven_object(&st.access, i)`, `equip_object`, `quiver_object`, `store_stock`, `top_floor_object`; `GameObject<'a>` from them or their `known`). Objects are only freed while the game runs, i.e. after the Borg returns a key or inside an engine call that lends the state; every such call takes `&mut BorgState`, which the borrow of `st.access` forbids, so **the compiler rejects a handle kept across a lending call** (or across any `&mut st` call), and a handle cannot outlive the entry or be stored in `BorgState` (no lifetime parameter). Only the borrow of that one field is shared, so "read object, update state" works with split borrows (`let BorgState { access, items, act, item_val, .. } = st;`: borg_inventory, borg_store; borg_flow_take looks the floor object up where it reads it, after the `&mut st` calls C makes in between, which do not change the floor). The constructors check at run time that the token is the root state's own field inside an entry (address compare), so a token of another `BorgState` or one moved out with `mem::swap` panics. Handles stay `Copy` (copies share the lifetime). `AngFile` (no public constructor, made by `ReadFile::getl` for one call), `TermHandle` (terms live for the game) and `Menu` (owned, never freed) need no lifetime |
| L5 | `&'static` refs into engine tables that change at runtime (`kind.ignore`, `cur_num`) | real, latent (all uses are transient) | not changed; same rule: never hold across a bridge or engine call. **Bridge removal:** still latent and documented (fixing means replacing 26 accessors and their users with copies). Checked: the 11 `k_info()` users outside engine.rs hold the reference at most within one function; the only engine calls made while one is held are notes (borg_flow_take's lunal note), whose message display does not write `k_info`; the only Borg writer of `kind.ignore`, `set_kind_ignore`, runs in borg_init with no reference held. (A nested Borg turn inside such a note could in theory reach it; the suites never nest). **Fixed after the external review (finding 3):** `k_info()`, `r_info()`, `a_info()` and `e_info()` are gone; `engine::kind(i)`, `race(i)`, `artifact(i)`, `ego(i)` return a copy of one entry (index asserted, `#[inline]`), `k_max()`/`r_max()` give the counts, and `FloorObject::kind()` returns a copy. These tables are written at run time (`object_desc` itself sets `kind->everseen` and `ego->everseen`; `aware`, `tried`, `ignore`; a race's `cur_num`/`max_num`; randarts rewrite `a_info`), so no `&'static` into them is handed out. The copy is taken in the statement that made the reference before, and the Borg makes no call that writes an entry between taking a copy and reading it (the fields it reads that change at all, `aware`, `ignore`, `max_num`, only change while the game runs), so the values read are the same. 46 call sites outside engine.rs changed mechanically (`&engine::r_info()[i]` -> `engine::race(i)`); `borg_lookup_kind` scans `kind(k)` pairs. `&'static` stays only for data nothing writes after loading, each with a SAFETY note saying so: `z_info`, `kb_info`, `player_race_info`/`player_class_info`, game-data name strings, quark strings (`FloorObject::note`). The private `player_ref()` (`&'static player`) is used inside engine.rs for the field reads of one wrapper only, never across an engine call |
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

## 6. External review of `4c06cec08`, findings 2 and 3

The external review (`docs/REVIEW-4c06cec08.md` on branch
`review/m3-4c06cec08`) reproduced, with its ASan reproducer, a read at
address `0x2` from the safe call `borg_rs::engine::flag_has(&[], 9)`: C's
`flag_has` only `assert`s the offset and the engine is built with
`-DNDEBUG`. Finding 3 is L4/L5 above plus the `static mut` root.

| id | finding | verdict | action |
|----|---------|---------|--------|
| F2 | safe wrappers pass unchecked flags/indices to C (`flag_has`, `flag_next`, `race_flag_has`, `player_has`, `spell_okay_to_cast`) | real, reproduced (not reachable from the Borg: no caller passes such a value) | every safe `pub fn` of engine.rs that passes an index, flag, count, grid or pointer to C was audited; each checks its precondition in Rust and panics, the SAFETY comments say what is checked. Changed: **`flag_has`** (FLAG_END = 0 is false, as C returns before reading, even on an empty set; otherwise `1 <= flag` and byte `(flag-1)/8` < len; negative panics) and **`flag_next`** (`flag >= FLAG_START`: C reads `flags[-1]` for FLAG_END, whose `FLAG_OFFSET` is `SIZE_MAX/8` stored in an `int`, and before the array for negatives; a start past the end gives FLAG_END as in C; `len*8+1` must fit an `int`) are now Rust transcriptions of z-bitflag.c (testable without the engine; differential check against the real `z-bitflag.c` on 875,000 in-range inputs: identical); **`race_flag_has`**, **`player_has`**, **`square_trap_is_glyph`** go through the checked `flag_has`; **`player_of_has`** checks the flag against OF_SIZE before the C call; **`spell_okay_to_cast`** asserts `0 <= index < total_spells` (C reads `p->spell_flags[index]`; its caller already passed `spell_by_index`'s check); **`randint0`** asserts `m <= 0x10000000` (`Rand_div` divides by zero above, its assert is compiled out); **`event_signal`**, **`event_signal_message_null`**, **`event_add_handler`**, **`event_remove_handler`** assert `type < N_GAME_EVENTS` (the handler table is indexed unchecked) and take a safe `extern "C" fn` handler; **`set_inkey_hack`** takes a safe `extern "C" fn` (an `unsafe fn` pointer would let safe code install a function with preconditions the engine does not meet); **`term_xtra`** only accepts `TERM_XTRA_NOISE`/`TERM_XTRA_DELAY` (other actions process input, which could reach the Borg without lending); **`project_from_player`** takes `Projection::{Beam, Arc}` (the Borg's two fixed argument sets) and a target in bounds, because `project` does not check `typ`, its 256-grid blast arrays or the path; **`ignore_level`/`set_ignore_level`**: `headers/obj_ignore_h.rs` declares the static with its C length (`[u8; ITYPE_MAX]`, was `[u8; 0]`, a read outside the declared static as in L6). Comments corrected where the C checks itself (NDEBUG keeps these): `square_monster`, `square_monster_midx`, `top_floor_object` (`square_monster`/`square_object` check bounds), `term_erase`, `term_putch`, `print_rel` (`Term_gotoxy` and panel checks), `spell_chance` (`spell_by_index` checks). Audited, already checked or no precondition: the `square_*`, `map_info`, `feat_*`, `cave_monster`, `class_book*`, `spell_by_index`, `player_spell_flags`, `player_timed`, `equip/quiver/store` descriptions, the `adj_*`/`dd*`/`extract_energy`/`get_angle_to_grid`/`all_letters*` tables (Rust array indexing on statics declared with their C size), `curse_name`, slays/brands, `race_blow*`, `pain_texts`, `screen_at`, `text_mbstowcs`, `wcstombs`, `get_string`, `file_getl`, `player_has_temporary_brand`/`_slay` and `obj_flag_power` (loops over bounded tables), `tval_find_name`, `message_str`, `target_set_interactive`, `panel_contains`. Unit tests: `flag_has(&[], 9)` and `flag_has(&[0], 9)` panic, `flag_has(&[], 0)` is false, `flag_has(&[0x02], 2)` and the other `FLAG_OFFSET`/`FLAG_BINARY` cases, `flag_next` from 0 or a negative flag panics, `flag_next` matches the C loop. The reviewer's reproducer, rebuilt against the new rlib, panics in Rust (`flag 9 out of range for a 0-byte flag set`) and ASan reports nothing |
| F3a | `engine::enter` is a safe `pub fn` that mutates the `static mut` root on a single-thread assumption | real | `enter` is `pub(crate)`; its four callers are the engine callbacks, three private `extern "C"` functions registered with the engine and `do_cmd_borg`, now a private `#[no_mangle]` symbol, so no code outside the crate can enter. Every entry asserts that the engine is set up (`z_info` and `player` allocated) and that it runs on the thread of the first entry (`ThreadId` compare). `lend` asserts that it lends the root state. Not enforced: the other safe wrappers read engine globals without a capability argument, so an external crate linking the rlib could still call one before the engine is set up or from another thread; inside the crate every caller runs inside an entry (Borg code is only reached from the callbacks). Giving every wrapper a token would touch ~200 wrappers and all their callers |
| F3b | L4, L5 | real | fixed, see L4 and L5 above |

The engine's own `flag_next(flags, size, FLAG_END)` reads `flags[-1]`
although its comment says FLAG_END starts at the beginning. It is engine
code, not Borg code, and no Borg caller passes FLAG_END (the C Borg starts
at FLAG_START), so it is not a `c-bugs.md` entry; the Rust wrapper panics
there.

After the fixes: `cargo build`/`test` (94 passed)/`clippy -D warnings`
clean; unsafe count 269 (engine.rs 241, headers 28); on `6afee5ab9`: smoke 3/3,
full 45/45, deep 9/9 (run in two halves of 5 and 4 runs, each diffed
against its reference runs), classes 16/16 identical, no nested entry,
`check-no-c-borg.sh` OK (154/154 engine objects byte-identical).
