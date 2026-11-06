# Response to the second review (821cad310)

The review is `docs/REVIEW-821cad310.md` (a condensed record of the text
the owner pasted). This file gives a verdict for each finding, what was
changed and how it was checked. "Synthetic" means a constructed
counterexample; "gameplay" means it happened in a real seeded game.

## Summary

| # | finding | verdict | change |
|---|---------|---------|--------|
| 1 | public safe wrappers reach UB without an initialised engine | **confirmed** (synthetic external client; by reading, the same for `player_ref`, RNG and `check_access`) | fixed: no public safe facade; every module is crate-private |
| 2 | `distance`, `modify_stat_value` let C signed arithmetic overflow | **confirmed** (synthetic API inputs) | fixed: computed in Rust with checked arithmetic, identical to C on the valid domain; audit extended to C arithmetic |
| 3 | compression bypasses `--expect-impl rust` | **confirmed** (reproduced) | fixed: roles come from the manifest; `--expect-impl` needs a verified output |
| 4 | no replayable endgame / state oracle | **confirmed** (acknowledged gap) | partly: a committed configuration suite (see section 4); endgame and state oracle still missing |
| 5 | stale summary wording | **confirmed** | fixed |

No finding was a divergence in a real game: every one is an unsound API,
an evidence/tooling gap, or wording.

## 1. Public safe facade — confirmed, fixed

`borg_rs::engine::z_info()` built a reference from the engine's global
without any check that the engine was initialised; `enter`'s readiness and
thread checks only guard the engine callbacks. The same holds for every
public function that reads engine state, in any module (not only
`engine`): an external crate could call them before initialisation or from
another thread.

**Fix** (`76834a370`). Every module is `pub(crate)`. The library is used
only as a staticlib through its C-ABI symbols (`do_cmd_borg`, the input
hook and event handlers that `engine` installs), which the game calls after
initialisation on its thread; `engine::enter` checks both. Calling those
symbols from Rust needs `unsafe` extern declarations, so no safe code
outside the crate can reach the engine. The only public Rust item left is
`regress::note_alias_report`, a pure function used by
`harness/regress/note-alias` (no engine call). The reviewer's client now
fails to compile: `error[E0603]: module 'engine' is private`.

Inside the crate the convention remains (all Borg code runs inside an
entry, on the engine thread); that is the crate's own invariant, not a
public contract. Making the modules private surfaced dead code: the header
mirrors (`headers/`, `borg_headers/`) are marked `allow(dead_code)`; twelve
unused public wrappers were deleted (none was on a C code path: the
formula and calculation code runs through its generic `*_in` functions);
C fields and constants that C never reads either keep a targeted `allow`
with the reason (e.g. `MapGrid::lighting`, unread because of C bug 46).

## 2. C arithmetic preconditions — confirmed, fixed

`distance` (cave-view.c) and `modify_stat_value` (player-util.c) are now
computed in Rust exactly as C writes them, with checked arithmetic; the
reviewer's three inputs (`x = i32::MIN`, `value = i32::MAX`,
`amount = i32::MIN`) panic instead of reaching C's undefined behaviour. A
temporary differential against the C functions (extracted verbatim from
the engine sources) on 547,157 valid inputs, including the range where C's
`int16_t` conversion wraps, gave identical results. Four unit tests keep
the values and the panics.

The audit was extended from bounds to arithmetic for every C function the
engine wrappers call with integer arguments (105 functions):
`object_value`/`object_value_real` compute `value * qty` and divide by
`obj->number` in plain `int`, so they now require `qty` 0 or 1 and a
non-empty stack (the Borg only prices one item of a ware); `Rand_div` was
already bounded; `randcalc` only receives game data; `panel_contains` uses
unsigned arithmetic; `get_quantity` clamps; the rest pass coordinates or
indices that C or the earlier fix checks.

## 3. Compressed outputs and `--expect-impl` — confirmed, fixed

**Reproduced:** an xz-compressed copy of the C smoke output passed
`--expect-impl rust` against the reference (3/3, exit 0), because a dir
without plain `keys.log` was taken for a reference before any check.

**Fix** (`c20622542`). The role comes from the manifest, not the log
format: a dir whose manifest has the build identity is a suite output
whatever its compression (DONE, overrides, implementation and build checks
apply); a reference has no such manifest (a kept C-run manifest, as in
`reference/classes`, must record C). `--expect-impl X` requires a verified
suite output of X among the operands. Checked: the reviewer's command is
refused (exit 2); a compressed Rust output passes; a compressed output with
an override is `UNVERIFIED` and refused; one without DONE is refused; two
references with `--expect-impl` are refused; normal outputs and the classes
reference pass. A forged manifest is still out of scope (documented).

## 4. Endgame and state oracle — acknowledged, partly addressed

Agreed: the stored references stop at dungeon level 26, keys do not show
every state change, and the per-module cross-check tooling was not
committed. Added: CONFIG-SUITE-PENDING. Still missing: endgame saves
(depth 50+, 99, 100, Morgoth), a state/note oracle, replayable cross-check
drivers. The scope statement in `docs/REVIEW.md` stays.

## 5. Editorial — confirmed, fixed

`docs/STATUS.md` gives the current unsafe count next to the historical
one; the `ItemState` comment describes the note reference; PROGRESS and
REVIEW point to SELF-REVIEW section 6 for the wrapper audit.

## Panics where C has undefined behaviour

Agreed with the review's list and its conclusion: none was reproduced in a
real game (none occurred in any suite or in the reviewer's configuration
runs); each stays a checked panic, documented as a safe departure from C
where C leaves its defined domain.

## Checks after the fixes

On `c20622542` (binaries built by `harness/build.sh`, `impl_check` and
`build_check` ok): C smoke 3/3; Rust smoke 3/3, `full` 45/45, `deep` 9/9,
`classes` 16/16 identical (`harness/diff --expect-impl`); no nested entry
and no panic in any run; `check-no-c-borg.sh` OK including the link-input
step (154 engine objects, 31 staticlib members, 28 system files);
`harness/regress/note-alias` identical (11 lines); `harness/regress/nested`
OK (7 nested entries, all games completed); `cargo test` 100 passed;
clippy `-D warnings` clean (also `--all-targets`) on Rust 1.97.0 and
1.98.1; unsafe 268.
