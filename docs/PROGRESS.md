# M3 progress: safe, idiomatic Rust

## Conventions

* One original `src/borg/<x>.c` maps to `borg-rs/src/<x>.rs` once converted.
  Until then it lives in `borg-rs/src/c2rust/<x>.rs` (unsafe c2rust code).
  Converted modules contain **no `unsafe`**.
* `engine.rs` is the permanent FFI boundary: safe accessors for engine
  globals and functions. `headers/` holds the raw C declarations.
* `bridge.rs` is transitional and goes away at the end. It holds:
  * `st()`: the single `BorgState` for c2rust code;
  * raw-pointer views into converted state (e.g. `grid(x, y)`);
  * safe wrappers that converted code uses to call still-c2rust functions
    (e.g. `borg_note`).
* State: each converted module owns a sub-struct of `state::BorgState`
  (e.g. `CaveState`). Converted functions take `st: &mut BorgState`.
* Bridge wrappers that call into c2rust code take the caller's
  `&mut BorgState` (even if unused), so the borrow visibly escapes into the call.
  Otherwise LLVM's `noalias` on `&mut` could cache state that the c2rust code
  changes through the global pointer.
* C ctype classes come from `cctype.rs` (as measured under `C.UTF-8`).
* Behaviour stays exact: the Borg's own algorithms (e.g. its quicksort,
  whose tie order matters) are kept, not replaced with std equivalents.
* `tools/transpile.sh` / `tools/reorganize.py` produced the starting point.
  **Do not re-run them**: they would overwrite hand-converted code.
* Checks: smoke after every module commit, full after every 5 and at the
  end. `cargo clippy` must be clean for converted files (c2rust, headers
  and bridge are exempt with `#[allow(clippy::all)]`).

## Modules

| # | module | commit | notes |
|---|--------|--------|-------|
| 0 | (dedup headers) | M3 prep | 173k -> 95k lines, cross-module Rust paths |
| 1 | borg_util | with 2 | `borg_sort` is a `BorgSortTarget` trait; `borg_trim` works in place on a byte buffer |
| 2 | borg_cave | with 1 | `CaveState` (flat `Vec<BorgGrid>`); 320 c2rust grid sites use `bridge::grid` |
| 3 | borg_cave_util | own | grid predicates take `&BorgGrid`; panel size via `engine::term_geometry()`; `tools/m3_calls.py` rewrites call sites |
| 4 | borg_io | own | `IoState`; C-ABI shims under the old names in `bridge::io` (re-exported from `headers/borg_io_h.rs`) so 1,100 c2rust call sites stay as they are until their module converts; engine terminal/message/event/wcstombs wrappers |
| 5 | borg (borg.c) | own | `Borg` struct + `CoreState` (cfg, rand_local, step, w_x/w_y) in `BorgState`; `trait`/`has`/`activation` are `Vec<i32>`; about 5,000 c2rust uses rewritten to `bridge::st().borg...`; the entry point (`inkey_hack` callback) is safe Rust; engine wrappers for the player, Term_inkey, RNG swap and inkey_hack |
| 6 | borg_item_activation | own | `ActivationIds` (164 `act_*`); 547 c2rust uses rewritten |
| 7 | borg_cave_view | own | `ViewState` (`view_n`, `view_x`/`view_y` as `Vec<u8>`); C-ABI shims in `bridge::cave_view`; `bridge::borg_los` wrapper; the 8 copied scan loops share `scan_ray`/`scan_strip` helpers (same order); 3 c2rust uses in borg_caution rewritten |
| 8 | borg_item_val | own | `ItemValState` (198 `sv_*`/`kv_*`, C names kept as fields); 750 c2rust uses rewritten to `bridge::st().item_val...`; shims in `bridge::item_val`; engine wrappers `k_info()`, `tval_find_idx`, `tval_find_name`, `lookup_sval` |
| 9 | borg_formulas_calc | own | `CalcState` (`calculations: Vec<Calculation>`, tokens are an enum); calls out go through a `CalcEnv` trait (bridge wrappers for `parse_value`, `calculate_from_value`, `borg_formula_error`) so the calculator is unit tested; C-ABI shims in `bridge::formulas_calc`; checked against the C file on 40k random formulas and with `borg_uses_dynamic_calcs` on (3 runs, local harness tweak, not committed) |
| 10 | borg_projection | own | `ProjectionState` (`target_loc` = `borg_target_loc`, 1 c2rust use in borg_fight_defend rewritten); `borg_inc_motion(&mut y, &mut x, ..)`; `borg_los` core takes the origin check and floor test as closures (unit-tested on hand-made maps); C-ABI shims in `bridge::projection`; `bridge::borg_los` removed (borg_cave_view calls the safe one); bridge reads of `borg_fear_region`, `borg_kills[i]`, `borg_race_name`; engine wrappers `distance`, `r_info()`, `race_flag_has`; full 45/45 identical |
| 11 | borg_cave_light | own | `LightState` (`light_*`, `glow_*` as `Vec<u8>`); shim in `bridge::cave_light`; `bridge::item(st, i)` copies a `borg_items` entry (transitional, until borg_item converts); 16 c2rust uses in borg_flow_dark/borg_flow_misc/borg_update rewritten to `bridge::st().light...`; C bug 3 (west-side grids) kept |
| 14 | borg_home_power | own | no module state; the repeated "spare item" / stat / stack blocks share small helpers (`spare`, `too_many`, `stat_tiers`, `stack`) that keep the C `long`-then-truncate-to-`int32_t` arithmetic; transitional `bridge::home_notice::*` accessors copy the `int16_t` counters of borg_home_notice (macro-generated), `bridge::borg_spell_stat`; engine `kb_info()`; shim in `bridge::home_power`; checked in-game against the c2rust version on every call (full suite) and on 2M fuzzed counter sets per smoke run (local, not committed); C bug 5 (spell stat +26, dead neck branch) kept |

**Checkpoint after module 11** (5 more module merges): `full` 45/45 identical, `deep` 9/9 identical.
| 12 | borg_magic | own | `MagicState` (`magics: Vec<BorgMagic>` = `borg_magics`, empty = NULL; `ratings` = `borg_spell_ratings`); `BorgMagic.name` is a `&'static CStr` (c2rust format sites use `.name.as_ptr()`); `borg_magic`/`borg_spell_rating` are type aliases in the header; 5 c2rust `borg_magics` uses rewritten to `bridge::st().magic.magics[..]`; shims in `bridge::magic` (`borg_get_spell_entry` returns a pointer into the Vec); bridge `borg_item_tval_sval`; engine wrappers for class books/spells (`class_book`, `class_book_spell`, `spell_by_index`, `spell_effect_indices`, `spell_okay_to_cast`, `player_spell_flags`, `player_has`, `no_light`, `all_letters_nohjkl`, ...); smoke 3/3, full 45/45, deep 9/9 identical |
| 13 | borg_formulas | own | `FormulasState` (power/depth/restock lines as `Vec`s of structs; a depth line keeps its reason as a `CString` for the C callers); parsers work on byte buffers with C string semantics so error notes match byte for byte; engine and lookups behind a `FormulaEnv` trait (fake in unit tests); the calculator calls `parse_value`/`calculate_from_value`/`borg_formula_error` directly (bridge wrappers removed); C-ABI shims in `bridge::formulas`; bridge wrappers for `prefix_pref`, `borg_settings`, `borg_items`, `borg_item_weight`, `borg_ego_has_random_power`, `borg_spell_stat`, `borg_can_cast`, `borg_get_spell_entry`; engine wrappers `AngFile`/`file_getl`, `find_class`, `class_spell_weight`, `class_book_dungeon`, `player_has`, `spell_chance`, `lookup_sval_bytes`; checked against the C file with stubs on 10k random formula sections (identical notes, parses and evaluations) and with `borg_uses_dynamic_calcs` on (5 runs x 100k turns, identical keys; local harness tweak feeds ESC after `z`, not committed) |
| 15 | borg_magic_play | own | no module state; the c2rust copy of `effects[]` (112 handler pointers) is replaced by `effect_aim()`, the `aim` column of list-effects.h (unit-tested: 22 aimed effects); calls `borg_magic::borg_spell` and `engine::{class_total_spells, player_has, all_letters_nohjkl}` directly; shim in `bridge::magic_play` (3 c2rust calls in borg_think_dungeon); C bug 6 (`info == BORG_DARK`) kept; smoke 3/3, full 45/45, deep 9/9 identical |
| 16 | borg_prepared | own | `PreparedState` (`borg_numb_live_unique`, `borg_first_living_unique`, `borg_depth_hunted_unique`; 19 c2rust uses in borg_update/borg_escape rewritten to `bridge::st().prepared...`); functions return `Option<Reason>` (`Text(&'static str)`, `Formatted(String)`, formula line index `Depth(i)`/`Restock(i)`); C-ABI shims in `bridge::prepared` keep C's pointers: interned literals, one static `borg_prepared_buffer` for formatted reasons (callers keep a reason across calls), formula reasons point into `st.formulas`; bridge `prepared_home_heals` (`num_ezheal`/`num_life`/`*_true`) and `prepared_notice_home`; engine `race_name`; C bug 8 kept; checked against the old c2rust code with randomized traits/config/uniques/home counts at depths -1..130 (81M calls, 2 runs, local tweak, not committed); smoke 3/3, full 45/45, deep 9/9 identical |

**Checkpoint after module 6** (5 module commits): `full` 45/45 identical, `deep` 9/9 identical.

## Unsafe count

`borg-rs/tools/count-unsafe.sh` counts the token `unsafe` (fns, blocks,
impls), comments excluded.

| after | total unsafe | c2rust modules left |
|-------|--------------|---------------------|
| M2 (c2rust) | 742 | 59 |
| borg_util + borg_cave | 745 | 57 |
| borg_cave_util | 742 | 56 |
| borg_io | 757 | 55 |
| borg | 761 | 54 |
| borg_cave_view | 762 | 52 |
| borg_item_val | 765 | 52 |
| borg_formulas_calc | 751 | 52 |
| borg_projection | 770 | 50 |
| borg_cave_light | 766 | 50 |
| borg_magic | 769 | 48 |
| borg_formulas | 746 | 49 |
| borg_home_power | 763 | 47 |
| borg_magic_play | 656 | 46 |
| borg_prepared | 761 | 45 |

## Suite timings

| when | suite | impl | wall |
|------|-------|------|------|
| M2 | full | rust | 139 s |
| M3 #6 | full | rust | 145 s |
| M3 #6 | deep | rust | 326 s (C: 304 s) |
