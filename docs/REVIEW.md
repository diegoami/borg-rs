# Guide for reviewers

The owner's current request is **independent M4 review only**, coordinated
in [borg-rs#1](https://github.com/diegoami/borg-rs/issues/1). Review pinned
tag `m4` / `3923a21e75dfb94053d4cc787d81c5baf638d62e`, using isolated
builds. The issue contains the scope, pinned evidence and required comment
workflow for both reviewer and implementation agent. The
[independent verdict](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707067352)
is qualified pass with no acceptance blockers; D/E scope remains qualified.
The reviewer verified the first tooling fixes; two further small cost75/
fixture-portability fixes await follow-up verification. See STATUS and issue#1.
The corpus results below retain their original source/scope; the checkpoint
audit itself was author-run. Do not include later #295 behavior in this verdict.


Owner update (2026-11-06): targeted C/Rust corrections for crashes and
blatant play blockers are allowed; other conflicts are case by case.
See `docs/FORK-CORRECTIONS.md`, which supersedes earlier absolute fidelity
restrictions for those corrections. Broader divergence follows a milestone
and owner-created tag.
This repository ports the Angband Borg (the game's built-in automatic player,
about 64k lines of C in `angband/src/borg/`) to Rust. It is a faithful port:
the Rust Borg must produce the same keypress stream as the C Borg,
byte for byte, while driving the same unmodified C game engine with the same
seed. New tactics remain outside this milestone. Narrow owner-authorized
crash/play-blocker corrections are mirrored in C/Rust and documented in
`docs/FORK-CORRECTIONS.md`, with corrected C run twice and original
references preserved. Other known C bugs remain recorded in `docs/c-bugs.md`.

The declared M4 checkpoint is complete at tag `m4`. The full coverage corpus
contains 139 successful oracle-checked executions in 11 groups, comparing
keys, messages and per-key player state. Production inputs at the M4 tag are
`28b8170`; later commits add tooling, fixtures, receipts and the interactive
launcher. Original references remain separate from corrected C references.
The subsequent #295 recognition correction and its new-source checkpoint
are recorded separately in STATUS; the M4 percentages belong to its tag.

Extended games reach logged current depth 99; synthetic endgame starts
include depth 100. Six targeted engine cases establish actual self-lunal
entry/first descent and glyph creation, with complete final engine-save
equality between C twice and Rust. They do not establish autonomous rune
formation, useful unbounded scumming or an unassisted win. Persistent native
checks compare declared kernels and logical state beyond the game oracles,
with explicit environment adapters and fault controls.

Start with [STATUS](STATUS.md), [the M4 plan](M4-PLAN.md),
[full coverage](validation/2026-11-07-coverage/README.md),
[synthetic cases](validation/2026-11-07-synthetic-tactics/README.md) and
[native checks](validation/2026-11-06-native-expanded/README.md).
The earlier external review and response remain historical evidence
(`docs/REVIEW-RESPONSE-4c06cec08.md`).

## Where things stand

| milestone | state | evidence |
|-----------|-------|----------|
| M1 deterministic C harness | done (tag `m1-harness`) | `full` run twice, 45/45 logs identical; `docs/STATUS.md` |
| M2 c2rust transpile behind the seam | done (tag `m2-c2rust`) | `harness/check-no-c-borg.sh`; `full` 45/45 identical to the C reference |
| M3 safe, idiomatic Rust | done (tag `m3` = `74fc8793c`) | `docs/PROGRESS.md` (one row per module, checkpoints, unsafe count); final checkpoint in `docs/STATUS.md` |
| M4 wider parity evidence | independent qualified pass, no acceptance blockers; tag `m4` = `3923a21`; follow-up verification pending in #1 | 139 engine executions; persistent native checks; linked receipts above |

The owner-authorized main integration includes the former work-branch tip
`40194d8`; `claude/serene-ramanujan-8hzsfm` is historical. Continue from main.
The owner explicitly delegated creation of `m4` on resume; the pushed tag
identifies author-audited main commit `3923a21`, before broader behavior changes.
See M4-PLAN.md for the next-stage sequence.

## Layout

```
angband/             upstream Angband e473720ed (git subtree), plus small
                     #ifdef BORG_HARNESS hooks and documented FORK corrections
harness/             build.sh, run.sh, suite.sh, diff, check-no-c-borg.sh,
                     suites/, reference/ (C oracle logs, xz), native-checker/,
                     coverage/, scenarios/ and saves/
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
   run logs every key the Borg returns to the game (the oracle), every
   message added to the game's message log, Borg notes included
   (`messages.log`), and per key a digest of the engine state: position,
   depth, HP, SP, food, gold, exp, level, timed effects, gear timeouts and
   hashes of pack, equipment and rune knowledge (`state.log`). Before
   running it refuses a binary whose implementation (read from the
   executable, its CMake cache and its `BUILD_ID`) is not `BORG_IMPL`, or
   whose `BUILD_ID` does not match the binary or the current sources
   (overrides are recorded in the manifest). See `docs/harness.md`
   ("Build identity").
3. `harness/diff --suite <suite> [--expect-impl rust] A B` compares logs. It
   requires every run of
   the suite on both sides, byte-identical key, message and state logs,
   logs consistent with their `summary.json`, and equal summaries (except
   wall time). It prints the first divergence of each log with seed, game
   turn, key index and context. It
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
   * `config`: the same harness with one Borg option changed (flags
     `munchkin`, `lunal`): munchkin start and lunal mode, each on Human
     Warrior, Half-Elf Mage and Dunadan Blackguard (the runs of the second
     external review), plus one 1M-turn lunal run that reaches dungeon
     level 27. See "Coverage" below and `docs/harness.md`.
   * `options`: 25 runs, each with one other `borg.txt` option set
     (`NAME=VALUE` flags): money scum, dynamic calcs, worships, risky,
     uniques, swaps off, stops, autosave, verbose, chests, munchkin
     limits, respawn. Added in M4 (step B).
   * M4 adds `captured-rogue`, `crash-fixes`, `extended`, `natural` and
     `synthetic-tactics`. Together with the six baseline suites these form
     the 139-execution coverage corpus. Synthetic tactics compares complete
     final saves as well as the three logs. See the linked receipts for
     recipes, source identities, capture provenance and finite stops.
   * The C references are in `harness/reference/` and checked deterministic
     (two C runs, identical, for every suite; for `smoke` this was first
     recorded after the review). Original references use the original C
     behavior; corrected references are separately identified in
     FORK-CORRECTIONS.md.
     Since M4 step A each reference run also has `messages.log.xz` and
     `state.log.xz`, made the same way (C twice, identical; the stored
     keys and summaries reproduced byte for byte).
5. Per module, the converting agent also ran a temporary cross-check. The old
   c2rust code ran next to the new code on every real call, and on fuzzed
   inputs that reach code the suites never touch, with state compared field by
   field. These checks were not committed, so a reviewer cannot rerun them;
   each PROGRESS row says what was compared and how many calls. The committed
   M4 native checker adds reproducible C/Rust calculator, spatial/flow/melee,
   glyph and lunal checks, with source hashes and seven actual-source faults
   rejected. Its declared state footprint is not whole-Borg shadow execution.

To reproduce:

```
harness/build.sh && BORG_IMPL=rust harness/build.sh
for s in smoke full deep classes config options; do
    BORG_IMPL=rust harness/suite.sh $s
    harness/diff --suite $s --expect-impl rust harness/out/rust/$s harness/reference/$s
done
harness/check-no-c-borg.sh
```

The commands above reproduce the six baseline suites. For the full 11-group
measurement use `harness/coverage/README.md`; synthetic fixture generation and
C/C/Rust replay are documented in `harness/scenarios/README.md`. Persistent
kernel checks run with `CHECK_UBSAN=1 harness/native-checker/run.sh`.

## What to look at critically

A self-review (three independent read-only reviews) was done before the
external one; its findings, verdicts and fixes are in `docs/SELF-REVIEW.md`.

* **Coverage.** The keypress oracle only covers what the suites reach.
  The full game corpus covers 29550/34610 Borg-logic lines (85.38%) and
  1269/1380 LLVM function instances (91.96%); closures and monomorphizations
  count separately, and branch coverage is unmeasured. Per-file metrics,
  function counts and gaps are retained in the full coverage receipt.
  Native checker coverage is measured separately and never merged into
  these profiles. Entry alone does not establish completion of a tactic.
* **What the oracle cannot see.** It compares the keys the Borg returns,
  the messages (game messages and Borg notes) and a per-key digest of
  player state, up to the turn limit (M4 step A; see `docs/harness.md`).
  It does not cover screen output, the Borg's own windows or the death
  files; the Borg's internal state (its map, its idea of the monsters and
  items) except through keys and notes; engine state outside the digest
  (the cave, monsters, objects on the floor, stores) except through later
  keys, messages or player state and the explicitly compared serialized
  engine captures/final saves. Saves do not preserve private Borg memory.
  Native checks cover their declared kernel state; the separate read-only
  glyph observer samples the stop boundary, not every running private field.
  Other limits include Borg-side direct writes to game state
  (cheat-death cure, `option_set`, the RNG swap) except through the digest
  or later keys; anything
  after the turn limit; and most config-dependent paths. The `config`
  suite turns on `borg_munchkin_start` (the munchkin thinker runs) or
  `borg_lunal_mode` (lunal mode changes the stair and rest choices in the
  flow code). It does not reach `borg_think_dungeon_lunal`: that needs
  `borg_self_lunal` and a deep character well above its max depth, and
  eight 1M-turn tries with `borg_self_lunal` on never got there. The separate
  `synthetic-tactics` suite now records 155 actual self-lunal entries and
  8 glyph-builder entries, with zero-entry controls. Its accepted lunal
  case stops at the first descent; longer stair-cycling pilots failed and
  are excluded. Existing glyph-recognition #295 and reverse-flow #345
  remain unresolved; full rune formation is not claimed. The
  `options` suite (M4 step B) sets the other options one per run: money
  scum and self scum, dynamic calcs (stock formulas), the six worships,
  risky play, unique hunting, swaps off, depth/level stops, autosave,
  verbose, chest tolerance, munchkin limits and respawn (game
  `cheat_live` with Borg cheat death off). 20 of its 25 runs change the
  key stream; worships speed and hp, swaps off, verbose and chest
  tolerance are read but change no key in these games, so their
  effective branches rest on reading. Not covered: respawn winners,
  stop king, strange options, bell, ignore-settings restore, non-stock
  formulas (`docs/harness.md`, "The options suite").
* **The state root and nested entry.** `engine::enter` is the only place
  that hands out `&mut BorgState` (to the four engine callbacks). An engine
  call that waits for a key can run the Borg again inside the outer Borg
  call, as in C; the engine wrappers that can do that take the caller's
  state and lend it to the nested entry (`engine::lend`), so the nested
  Borg works on a reborrow, never on an alias. This happens in real play:
  two runs of the `config` suite (lunal mode) each report one nested entry
  in their `stderr.log`, and both match C's keys; `harness/regress/nested`
  forces it in the smoke games. The transitional `bridge.rs` (a global
  `st()`, C-ABI shims and `escape(st)` around calls into c2rust code) is
  gone with the last c2rust module.
* **`borg_item.note`** is a `NoteRef`: the live element it points into
  (`borg_items[i]` or a shop ware) plus an offset, resolved against that
  element's current `desc` at every read, as C's pointer is (struct copies
  copy the pointer, so a copy reads the origin's bytes, e.g. in wear
  trials). The bridge removal had used a plain offset into the copy's own
  `desc`; the external review showed that differs in a wear trial (fixed,
  PROGRESS row 62; C-vs-Rust differential `harness/regress/note-alias`).
* **Panics vs. undefined behaviour.** Where the C code would read out of
  bounds or divide by zero on impossible inputs, the Rust code panics. Each
  case is listed in PROGRESS or c-bugs.
* **Harness quirk.** By default the birth script picks races and classes
  with index 7 and up with the wrong menu letters, so `full` and `deep`
  never play a Blackguard, Dunadan, High-Elf or Kobold (their "ranger" runs
  are Warriors, their "blackguard" runs are Rangers). This does not affect
  C/Rust equivalence, but it limits coverage; the `classes` suite fills the
  gap (`docs/harness.md`, `docs/STATUS.md`).
* **Unsafe count.** `borg-rs/tools/count-unsafe.sh`: 267 after #295 (268
  at M4), all at the FFI boundary: 239 in `engine.rs` and 28 in `headers/` (the
  `unsafe extern "C" fn` pointer types of engine structs and callbacks).
  `#![deny(unsafe_code)]` allows it only in those two modules. After the
  external review of `4c06cec08` (SELF-REVIEW section 6): every safe
  wrapper checks the index/flag/grid preconditions C only `assert`s
  (`-DNDEBUG`); engine object handles borrow the state's `access` token,
  so the compiler rejects keeping one across an engine call that lends the
  state (L4); engine tables the game writes are read through copies, not
  `&'static` references (L5); `engine::enter` is `pub(crate)` and checks
  the engine is set up and on one thread. After the second review
  (`821cad310`) every module is crate-private: no safe Rust outside the
  crate can reach an engine wrapper (the only public item is the pure
  `regress::note_alias_report`); the engine enters the Borg only through
  its C-ABI symbols. Not enforced by the compiler: inside the crate the
  wrappers assume they run inside an entry (true for all Borg code, which
  runs only from those callbacks).
