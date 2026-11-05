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
| U1 | Re-entrancy: a Borg `msg()` (warning) with `auto_more` off can reach `msg_flush` -> `anykey` -> `inkey_ex` -> the Borg hook, running a nested Borg turn while the outer one holds `&mut BorgState` | real path; C has the same nesting (no guard in `inkey_ex`), so a guard would change behaviour | nesting kept as in C. Engine callbacks enter through `bridge::with_entry_state`, which **reports a nested entry on stderr** (kept per run in `stderr.log`). `msg`/`get_string`/message-flush calls from Borg code go through bridge wrappers that `escape` the state; the warning shim copies its text (it may live in `format()`'s buffer). Nested entries seen: none in smoke, full, deep (see below) (`c4969c0c5`) |
| U2 | `bridge::st()` was a safe `pub fn` returning `&'static mut` | real (formally UB under Stacked/Tree Borrows when two are live) | now `pub(crate) unsafe fn` with a `# Safety` contract; `state_ptr()` for raw access. The st()+`escape` scheme stays formally UB until c2rust is gone; `escape` only stops today's optimiser. This is transitional and disappears with `bridge.rs` (`c4969c0c5`) |
| U3 | `square_isvault` / `square_isglow`: no bounds check (the engine's `assert` is compiled out by `-DNDEBUG`) | real | `assert!(square_in_bounds)` (`c4969c0c5`) |
| U4 | `screen_at`: no bounds check | real | asserts `0 <= x < wid`, `0 <= y < hgt` (`c4969c0c5`) |
| U5 | `class_book_dungeon`: `debug_assert!` only | real | goes through the checked `class_book_ref` (`c4969c0c5`) |
| U6 | `feat_is_trap_holding`: unchecked feature index | real | range assert as in `feat_is_shop` (`c4969c0c5`) |
| U7 | safe fns dereference the raw `borg_item.note` | real, transitional | **not fixed yet.** Making them `unsafe` would put `unsafe` into converted modules. `note` becomes an offset when the last c2rust reader of it is converted; tracked in `REVIEW.md` |
| L1/L2 | `borg_item_analyze_ware` called a `st()` shim without `escape`, passing a pointer into the state | real, latent | calls the converted analyzer directly with split field borrows; the shim now borrows only the two tables (`c4969c0c5`) |
| L3 | `borg_detect_wall` checks the wrong size | real, latent | = S2 |
| L4 | engine object handles are `Copy` without a lifetime | real, latent (none is stored today) | not changed; rule: never store a handle in `BorgState` |
| L5 | `&'static` refs into engine tables that change at runtime (`kind.ignore`, `cur_num`) | real, latent (all uses are transient) | not changed; same rule: never hold across a bridge or engine call |
| L6 | `all_letters*` declared as `[c_char; 0]` | real (reads outside the static's provenance) | declared with their C lengths, 53 and 49 (`c4969c0c5`) |
| L7 | shims pass a slice into the state together with a fresh `st()` | real, latent | the texts are copied first (`c4969c0c5`) |
| L8 | `msg_buf()` was a safe fn calling `st()` | real | `unsafe fn` (`c4969c0c5`) |
| C1-C2, C4-C5 | comment and documentation precision | cosmetic | partly addressed in the touched code |
| C3 | the "no unsafe in converted modules" rule was not enforced | real gap | `#![deny(unsafe_code)]` at the crate root, allowed only in `engine`, `headers`, `bridge`, `c2rust` (`c4969c0c5`) |

## 3. Harness and evidence

PENDING (harness agent)

## 4. Checks after the fixes

On `c4969c0c5` (45 modules plus the fixes above): `cargo test` and
`clippy` clean, smoke 3/3, `full` 45/45 and `deep` 9/9 identical to the C
references. No run of any suite printed a nested Borg entry, so the U1 path
is not reached by the suites. `main` was moved to this commit.
