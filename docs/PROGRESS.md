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

## Unsafe count

`borg-rs/tools/count-unsafe.sh` counts the token `unsafe` (fns, blocks,
impls), comments excluded.

| after | total unsafe | c2rust modules left |
|-------|--------------|---------------------|
| M2 (c2rust) | 742 | 59 |
| borg_util + borg_cave | 745 | 57 |
| borg_cave_util | 742 | 56 |
| borg_io | 757 | 55 |

## Suite timings

| when | suite | impl | wall |
|------|-------|------|------|
| M2 | full | rust | 139 s |
