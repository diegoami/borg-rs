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
| M3 safe, idiomatic Rust | in progress | `docs/PROGRESS.md` (one row per converted module, checkpoints, unsafe count) |

`main` is moved forward only to commits that passed a full checkpoint (`full` 45/45
and `deep` 9/9 identical to the C references). The work branch
(`claude/serene-ramanujan-8hzsfm`) has every intermediate commit, one per
converted module.

## Layout

```
angband/             upstream Angband e473720ed (git subtree), plus small
                     #ifdef BORG_HARNESS hooks (see docs/harness.md)
harness/             build.sh, run.sh, suite.sh, diff, check-no-c-borg.sh,
                     suites/{smoke,full,deep}, reference/ (C oracle logs, xz)
borg-rs/src/         the Rust Borg (staticlib linked into the engine)
  <module>.rs        converted modules: safe Rust, one per original .c file
  c2rust/            modules not converted yet (unsafe c2rust output)
  headers/           C declarations of the engine (raw FFI types)
  engine.rs          the permanent FFI boundary: safe wrappers over the engine
  bridge.rs          TRANSITIONAL glue between converted and c2rust code
  state.rs           BorgState: all former C globals, one sub-struct per module
docs/                STATUS.md, PROGRESS.md, c-bugs.md, harness.md, this file
```

## How equivalence is checked

1. `harness/build.sh` builds the engine with the C Borg (`build/c`).
   `BORG_IMPL=rust harness/build.sh` builds it with the Rust Borg (`build/rust`).
2. `harness/suite.sh <suite>` runs a seed suite headless and in parallel. Each
   run logs every key the Borg returns to the game (the oracle). See
   `docs/harness.md`.
3. `harness/diff A B` compares logs. It prints the first divergence with seed,
   game turn, key index and context.
4. Suites:
   * `smoke`: 3 runs × 20k game turns, run after every module.
   * `full`: 9 classes × 5 seeds × 100k game turns (early game, about clevel 10).
   * `deep`: 9 classes × 1M game turns with Borg cheat death, reaching clevel
     30 and dungeon level 26. Added in M3 for coverage.
   * The C references are in `harness/reference/`. They were made once from the
     unmodified C build and checked deterministic (two runs, identical).
5. Per module, the converting agent also ran a temporary cross-check. The old
   c2rust code ran next to the new code on every real call, and on fuzzed
   inputs that reach code the suites never touch, with state compared field by
   field. These checks were not committed; each PROGRESS row says what was
   compared and how many calls.

To reproduce:

```
harness/build.sh && BORG_IMPL=rust harness/build.sh
BORG_IMPL=rust harness/suite.sh full
harness/diff harness/out/rust/full harness/reference/full
harness/check-no-c-borg.sh
```

## What to look at critically

* **Coverage.** The keypress oracle only covers what the suites reach.
  Deep-dungeon code (below level 26), rare items and the Morgoth endgame are
  checked only by the per-module fuzz cross-checks and by reading the code.
  `docs/PROGRESS.md` lists the never-exercised paths for each module.
* **`bridge.rs`.** This is transitional unsafe code. It holds C-ABI shims under
  the old names (so unconverted c2rust code can call converted code), and
  wrappers that let converted code call c2rust code. The wrappers call
  `escape(st)` (a `black_box`) around each call, because without it LLVM's
  `noalias` assumptions on `&mut BorgState` caused real divergences (see the
  PROGRESS conventions). It shrinks as modules convert and goes away at the
  end of M3.
* **`borg_item.note`** is still a raw pointer into the item's own `desc`
  buffer, because copies of items keep pointing at the original, as in C. It
  becomes safe once the last c2rust readers are converted.
* **Panics vs. undefined behaviour.** Where the C code would read out of
  bounds or divide by zero on impossible inputs, the Rust code panics. Each
  case is listed in PROGRESS or c-bugs.
* **Harness quirk.** The birth script picks races 7 and up with the wrong
  menu letters. This does not affect C/Rust equivalence (`docs/STATUS.md`).
* **Unsafe count.** `borg-rs/tools/count-unsafe.sh`. It has not fallen yet,
  because the bridge grows while modules convert. The M3 target is `unsafe`
  only in `engine.rs` (the FFI boundary).
