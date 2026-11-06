# Guide for reviewers

This repository ports the Angband Borg (the game's built-in automatic player,
about 64k lines of C in `angband/src/borg/`) to Rust. It is a faithful port:
the Rust Borg must produce the same keypress stream as the C Borg,
byte for byte, while driving the same unmodified C game engine with the same
seed. It is not a redesign: no new tactics and no fixes to Borg behaviour.
Known C bugs are kept on purpose (see `docs/c-bugs.md`).

## Where things stand

| milestone | state | evidence |
|-----------|-------|----------|
| M1 deterministic C harness | done (tag `m1-harness`) | `full` run twice, 45/45 logs identical; `docs/STATUS.md` |
| M2 c2rust transpile behind the seam | done (tag `m2-c2rust`) | `harness/check-no-c-borg.sh`; `full` 45/45 identical to the C reference |
| M3 safe, idiomatic Rust | done (`dd885b6dd`; tag `m3` by the owner) | `docs/PROGRESS.md` (one row per module, checkpoints, unsafe count); final checkpoint in `docs/STATUS.md` |

`main` is moved forward only to commits that passed a full checkpoint (`full` 45/45
and `deep` 9/9 identical to the C references). The work branch
(`claude/serene-ramanujan-8hzsfm`) has every intermediate commit, one per
converted module.

## Layout

```
angband/             upstream Angband e473720ed (git subtree), plus small
                     #ifdef BORG_HARNESS hooks (see docs/harness.md)
harness/             build.sh, run.sh, suite.sh, diff, check-no-c-borg.sh,
                     suites/{smoke,full,deep,classes}, reference/ (C oracle
                     logs, xz)
borg-rs/src/         the Rust Borg (staticlib linked into the engine)
  <module>.rs        safe Rust, one per original .c file (all 59 converted)
  engine.rs          the FFI boundary: safe wrappers over the engine, the
                     engine's callbacks into the Borg, the root of the state
  headers/           C declarations of the engine (raw FFI types)
  borg_headers/      declarations of the Borg's own headers (not FFI)
  state.rs           BorgState: all former C globals, one sub-struct per module
docs/                STATUS.md, PROGRESS.md, c-bugs.md, harness.md, this file
```

## How equivalence is checked

1. `harness/build.sh` builds the engine with the C Borg (`build/c`).
   `BORG_IMPL=rust harness/build.sh` builds it with the Rust Borg (`build/rust`).
   Each build writes a `BUILD_ID` next to the binary: implementation, git
   revision, a hash of the source inputs and the binary's sha256.
2. `harness/suite.sh <suite>` runs a seed suite headless and in parallel. Each
   run logs every key the Borg returns to the game (the oracle). Before
   running it refuses a binary whose implementation (read from the
   executable, its CMake cache and its `BUILD_ID`) is not `BORG_IMPL`, or
   whose `BUILD_ID` does not match the binary or the current sources
   (overrides are recorded in the manifest). See `docs/harness.md`
   ("Build identity").
3. `harness/diff --suite <suite> [--expect-impl rust] A B` compares logs. It
   requires every run of
   the suite on both sides, byte-identical key logs, logs consistent with
   their `summary.json`, and equal summaries (except wall time). It prints
   the first divergence with seed, game turn, key index and context. It
   prints each output's build identity from its manifest and fails on an
   implementation mismatch or a recorded override.
4. Suites (the birth-script quirk below means `full` and `deep` play 8
   classes and fewer races than their names say):
   * `smoke`: 3 runs × 20k game turns, run after every module.
   * `full`: 45 runs × 100k game turns, early game: clevel 1-10 (median 4),
     dungeon level ≤ 5 (median 2), 15/45 die by clevel 5. Classes played:
     Warrior (10 runs) and the 7 others except Blackguard; 8 races.
   * `deep`: 9 runs × 1M game turns with Borg cheat death. The best run
     reaches clevel 30 and dungeon level 26; the median is clevel 18,
     dungeon level 7. Added in M3 for coverage.
   * `classes`: 15 runs × 100k and one 1M cheat-death run, with the correct
     menu letters: Blackguard, Dunadan, High-Elf and Kobold, which `full`
     and `deep` never play. Added after the harness review.
   * The C references are in `harness/reference/`. They were made from the
     unmodified C build and checked deterministic (two C runs, identical, for
     every suite; for `smoke` this was first recorded after the review).
5. Per module, the converting agent also ran a temporary cross-check. The old
   c2rust code ran next to the new code on every real call, and on fuzzed
   inputs that reach code the suites never touch, with state compared field by
   field. These checks were not committed, so a reviewer cannot rerun them;
   each PROGRESS row says what was compared and how many calls. For code the
   suites never reach, they are the only evidence besides reading the code.

To reproduce:

```
harness/build.sh && BORG_IMPL=rust harness/build.sh
for s in smoke full deep classes; do
    BORG_IMPL=rust harness/suite.sh $s
    harness/diff --suite $s --expect-impl rust harness/out/rust/$s harness/reference/$s
done
harness/check-no-c-borg.sh
```

## What to look at critically

A self-review (three independent read-only reviews) was done before the
external one; its findings, verdicts and fixes are in `docs/SELF-REVIEW.md`.

* **Coverage.** The keypress oracle only covers what the suites reach.
  Deep-dungeon code (below level 26), rare items and the Morgoth endgame are
  checked only by the per-module fuzz cross-checks and by reading the code.
  `docs/PROGRESS.md` lists the never-exercised paths for each module.
* **What the key oracle cannot see.** It compares only the keys the Borg
  returns, up to the turn limit. It does not cover Borg notes, screen output
  or the death files; Borg-side direct writes to game state (cheat-death
  cure, `option_set`, the RNG swap) except through later keys; anything
  after the turn limit; and config-dependent paths (all suites use the
  stock `borg.txt`; munchkin, dynamic calcs, money scum, respawn etc. are
  off, only `borg_cheat_death` changes).
* **The state root and nested entry.** `engine::enter` is the only place
  that hands out `&mut BorgState` (to the four engine callbacks). An engine
  call that waits for a key can run the Borg again inside the outer Borg
  call, as in C; the engine wrappers that can do that take the caller's
  state and lend it to the nested entry (`engine::lend`), so the nested
  Borg works on a reborrow, never on an alias. The suites never nest
  (`stderr.log` would say so). The transitional `bridge.rs` (a global
  `st()`, C-ABI shims and `escape(st)` around calls into c2rust code) is
  gone with the last c2rust module.
* **`borg_item.note`** is an offset into the item's `desc`. In C it is a
  pointer, and a copy of an item points into the *original's* `desc`; the
  offset reads the copy's own `desc`. Why that gives the same bytes, and the
  differential check behind it, is in PROGRESS ("bridge removal").
* **Panics vs. undefined behaviour.** Where the C code would read out of
  bounds or divide by zero on impossible inputs, the Rust code panics. Each
  case is listed in PROGRESS or c-bugs.
* **Harness quirk.** By default the birth script picks races and classes
  with index 7 and up with the wrong menu letters, so `full` and `deep`
  never play a Blackguard, Dunadan, High-Elf or Kobold (their "ranger" runs
  are Warriors, their "blackguard" runs are Rangers). This does not affect
  C/Rust equivalence, but it limits coverage; the `classes` suite fills the
  gap (`docs/harness.md`, `docs/STATUS.md`).
* **Unsafe count.** `borg-rs/tools/count-unsafe.sh`: 270, all at the FFI
  boundary: 242 in `engine.rs` and 28 in `headers/` (the
  `unsafe extern "C" fn` pointer types of engine structs and callbacks).
  `#![deny(unsafe_code)]` allows it only in those two modules. Engine
  object handles carry no lifetime and some accessors return `&'static`
  references into engine tables that the game can change (SELF-REVIEW L4,
  L5): the rule is never to keep either across an engine call or in the
  state, which holds today.
