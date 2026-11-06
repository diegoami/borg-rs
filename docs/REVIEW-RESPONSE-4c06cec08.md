# Response to the review of 4c06cec08

The review is `docs/REVIEW-4c06cec08.md` with its evidence in
`docs/review-evidence-4c06cec08/`, both on branch `review/m3-4c06cec08`
(kept there; not merged). This file gives a verdict for each finding, what
was changed, and how it was checked. "Synthetic" means a constructed
counterexample; "gameplay" means it happened in a real seeded game.

## Summary

| # | finding | verdict | change |
|---|---------|---------|--------|
| 1 | note offsets lose pointer aliasing in wear trials | **confirmed** (synthetic reproduced; not hit in any suite game) | fixed: notes reference their live origin |
| 2 | safe flag wrappers trust compiled-out C asserts | **confirmed** (ASan reproduced; no gameplay caller passes a bad flag) | fixed: preconditions checked in Rust for every audited wrapper |
| 3 | L4/L5 unenforced lifetime preconditions | **confirmed** as unsound contracts (no UB execution in the Borg's own callers) | fixed: table copies, lifetime-bound handles, `pub(crate) enter` |
| 4 | a Rust-labelled suite passes with the C binary | **confirmed** (reproduced) | fixed: build identity checks |
| 5 | untested domains have no replayable oracle | **confirmed** (evidence gap) | partly addressed: committed note and nested-entry regressions; equivalence claims scoped |
| 6 | clippy fails on Rust 1.98.1 | **confirmed** (reproduced on 1.98.1) | fixed; clean on 1.97.0 and 1.98.1 |
| 7 | stale note comments | **confirmed** | fixed |
| — | stale-position glyph panic (#198) | **agree: not established as normal play** | no change; documented |

## 1. Item notes and wear trials — confirmed, fixed

**Reproduced.** The reviewer's programs give, on this machine:
`C: hole note=[ longer {??}] needs_id=1`, `Rust (4c06cec08): hole note=[]
needs_id=false`.

**Why the M3 argument was wrong.** In C, `borg_item_analyze` is the only
place that sets `note`, and only on `borg_items[i]` or
`borg_shops[s].ware[j]` (borg-inventory.c, borg-store.c), as a pointer
into *that live element's* `desc`. The snapshots `safe_items`/`safe_shops`
are struct copies, so their notes also point into the live arrays. The
wear trial (borg-item-wear.c:857-875) copies `safe_items[slot]` into the
pack hole, copies `safe_items[i]` over `borg_items[slot]`, and calls
`borg_notice(true)` before restoring; the hole's note now reads the
candidate's bytes. The bridge-removal proof (PROGRESS row 60) assumed a
copy's note points into the copy's *source* element and that sources are
unchanged while copies are read; it missed that the source of a snapshot
copy is the live slot, which the trial overwrites.

**Fix** (`f9c2ef622`). `note` is now `Option<NoteRef { origin: Item(i) |
Ware { shop, slot }, offset }>`; `borg_item_analyze` records the live
element it analyzed, and every read resolves against that element's
*current* `desc` (bytes from the offset to the next NUL; a missing NUL
panics where C would read past `desc`). This reproduces C's pointer
semantics for every copy, including the ring-hole and home-sell cases the
old proof listed. Readers (`item_note`, `borg_get_note`,
`borg_item_note_needs_id`, `borg_object_similar`, the de-inscribe test)
take the state.

**Regression coverage.**
- `harness/regress/note-alias/run.sh`: the same struct copies (wear-trial
  hole, the reverse case, a ware copied into the pack then the ware
  re-read, an unchanged origin, NULL and `needs_ident`) are run through the
  original C `borg-item.c`/`borg-item-id.c` and through the Rust port; the
  outputs are compared byte for byte. Result: 11 lines, identical.
- Unit tests `borg_item::tests::wear_trial_hole_reads_live_origin` and
  `ware_copy_reads_live_ware`.

**Was gameplay affected?** A temporary instrumented build (not committed)
compared, on every real note read, C's aliasing (the fixed model) with
what the 4c06cec08 offset model read. Across `full` (45 runs), `deep` (9)
and `classes` (16): **0 reads differed**, 0 needs-ID results differed, and
the keys stayed identical. So the divergence is real and reachable in
principle (a worn item replaced in a trial by a candidate whose `{??}` lies
beyond the old note offset, with `ident` and `needs_ident` both false), but
no game in the suites reached it; the M3 suite results stand, and the fix
removes the difference.

## 2. Unchecked FFI inputs — confirmed, fixed

**Reproduced.** The reviewer's `borg-review-flag.rs` against the old rlib:
`AddressSanitizer: SEGV on unknown address 0x000000000002 ... in flag_has`
(here ASan needs `LD_PRELOAD` of `libasan.so`; with `-lasan` alone ASan
aborts at start). The C one-byte variant: `heap-buffer-overflow`. No Borg
caller passes an invalid flag (the suites and the cross-checks never hit
it), so this is unsoundness of a safe API, not a gameplay divergence.

**Fix** (`44bf5a210`). Every safe wrapper in `engine.rs` that passes an
index, flag, count or grid to C was audited (list in
`docs/SELF-REVIEW.md` section 6). `flag_has` and `flag_next` are now Rust
copies of `z-bitflag.c` (flag 0 is false as in C; a flag outside the slice,
a negative flag or a start below 1 panics), checked against the C routine
on 875,000 valid inputs: identical. `race_flag_has`, `player_has`,
`square_trap_is_glyph` go through it; `player_of_has`,
`spell_okay_to_cast` (`[0, total_spells)`), `randint0` (`m <= 0x10000000`,
C divides by zero above), the event functions (type inside the handler
table), `term_xtra` (noise and delay only), `project_from_player` (the
Borg's two fixed argument sets, target in bounds) check their inputs;
`set_inkey_hack` and the event handlers take safe functions only. After
the fix the reviewer's program panics in Rust (`flag 9 out of range for a
0-byte flag set`) and ASan reports nothing. Nine new boundary unit tests.
Not a Borg finding but noted: the engine's own `flag_next(flags, size,
FLAG_END)` reads `flags[-1]`; no Borg caller passes it.

## 3. L4/L5 and the state root — confirmed, fixed

The review is right that these were unsound safe APIs kept by convention.
- **L5:** `k_info()`, `r_info()`, `a_info()`, `e_info()` are gone;
  `engine::kind(i)`, `race(i)`, `artifact(i)`, `ego(i)` return a copy of one
  entry (these tables really are written at run time: `object_desc` sets
  `kind->everseen`, races' `cur_num` change, ...). Each copy is taken where
  the old reference was taken and nothing writes the entry before it is
  read, so the values are the same (46 call sites). `&'static` remains only
  for data never written after loading, each with a SAFETY note.
- **L4:** the object handles carry the lifetime of a borrow of a new field
  `BorgState::access`; the compiler now rejects holding a handle across any
  call that takes `&mut BorgState` (every call that lends the state to the
  engine, the only place objects can be freed), past the engine callback,
  or in the state. The constructors check at run time that the token is the
  real state's.
- **Root:** `engine::enter` is `pub(crate)`; `do_cmd_borg` is private (the
  symbol stays exported); every entry asserts the engine is initialised and
  runs on the thread of the first entry; lending asserts the root state.

Still not compiler-enforced: the other safe wrappers read engine globals
without a capability token, so an *external* crate linking the rlib could
call one before the engine is set up or from another thread (all Borg code
runs inside an entry); the private `player_ref()` returns `&'static` for
immediate reads inside `engine.rs`; the thread check cannot see two threads
racing on the very first entry.

## 4. Wrong binary accepted — confirmed, fixed

**Reproduced** at 4c06cec08 (3/3 "identical", exit 0, with a manifest
saying `borg_impl_env=rust`, `binary_cmake_borg_impl=c`).

**Fix** (harness commits, see `docs/harness.md`, "Build identity").
`harness/build.sh` records `build/<impl>/BUILD_ID` (implementation, git
revision, dirty flags, a hash of the source inputs, the binary's sha256,
compiler versions). `harness/suite.sh` determines the binary's
implementation from the executable itself (rustc `.comment` / a defined
`do_cmd_borg`), its CMake cache and `BUILD_ID`, refuses before any run on a
mismatch with `BORG_IMPL` (no override), refuses a stale or unrecorded
binary unless `ALLOW_STALE_BUILD=1` (recorded), and re-hashes the binary
after the runs. `harness/diff` prints the identity, fails on a mismatch,
a recorded override or a wrong `--expect-impl`, and refuses old-format
manifests. `check-no-c-borg.sh` gained a step that re-runs the link with
`--trace` and accounts for every link input. Now the reviewer's command is
refused: `REFUSED: BORG_IMPL=rust but .../build/c/game/angband is a c build
(executable: c, CMakeCache: c, BUILD_ID: c)`. Limits: the checks catch
mistakes, not a forged manifest; `run.sh` alone has no identity check;
references keep no provenance record.

## 5. Untested domains — confirmed evidence gap, partly addressed

The review is right that the per-module cross-checks were not committed
and that the suites cannot reach depth 50+, Morgoth, munchkin/lunal mode,
most debug commands, or note/state differences that do not change a key.
Added:
- the note-alias differential (section 1), which exercises C directly;
- `harness/regress/nested/run.sh`: builds with the test-only cargo feature
  `test-force-nested` (two 70-byte messages every 500 think steps), runs
  the smoke games and requires completion and at least one nested entry.
  Result: 7 nested entries, all games completed (keys not compared, the
  forced messages change them).

Not done: replayable cross-check tooling for the converted modules, config
suites (munchkin, lunal, dynamic calcs, swaps), an engine-state oracle, an
endgame save. The equivalence claim in `docs/REVIEW.md` and
`docs/STATUS.md` is scoped to the suites' seeds, configuration (stock
`borg.txt`) and turn limits.

## 6, 7. Clippy on 1.98.1, stale comments — confirmed, fixed

`borg_inc_motion` initialises `(dy, sy)` and `(dx, sx)` as tuples (pure
arithmetic, same order). Clippy `-D warnings` and the tests are clean on
Rust 1.97.0 (the project's machine) and 1.98.1 (the reviewer's). The wear
and store-buy module comments now describe the note reference.

## Stale-position glyph panic (#198)

Agreed: not established as a normal-play failure. The tracker issue reports
a fuzz lead only, and the reviewer's points (message flushes on level
change, monster energy on a new level) argue against the simple path. The
bounds-checked panic stays; replacing the stale position would silently
change C behaviour, and C's out-of-bounds read has no defined result to
reproduce. A native-C seed/save that reaches it would be needed to decide
more.
