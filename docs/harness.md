# Deterministic Borg harness

The harness runs the unmodified Angband engine (pinned upstream
`e473720ed88a0f0c7ca66815cef3000eb4a38bc3`, in `angband/`) with a Borg
behind the `inkey_hack` seam. It records every key the Borg returns. Two
Borg implementations (C and Rust) are equivalent when their key logs are
byte-identical for the same seeds.

## Engine changes

Enabled with `cmake -DSUPPORT_BORG_HARNESS=ON`. That defines `BORG_HARNESS`
for the core library and the executable, and adds the `borgh` frontend.
The hooks are under `#ifdef BORG_HARNESS` (or `USE_BORGH`). Two changes are
unconditional, and both are equivalent to upstream in a default build:
the CMake restructure (`BORG_IMPL`, sources moved to `target_sources`; with
the default `BORG_IMPL=c` it compiles the same files) and, in `mon-make.c`,
`localtime()` moved out of the declaration of `date`.

| file | change |
|------|--------|
| `src/main-borgh.c`, `src/main-borgh.h` | new headless frontend `-mborgh` (80x24, no drawing, no sleeping) |
| `src/main.c`, `src/main.h` | register the `borgh` module (`USE_BORGH`) |
| `src/z-rand.c` | `Rand_init()` uses `borgh_seed` when it is set, instead of `time()`/`getpid()` |
| `src/mon-make.c` | `get_mon_num()` uses a fixed date when the seed is fixed (no Christmas monsters) |
| `src/ui-input.c` | `borgh_key_hook` is called with every key returned by `inkey_hack` |
| `CMakeLists.txt` | option `SUPPORT_BORG_HARNESS`; `BORG_IMPL` (`c` or `rust`) |

No file in `src/borg/` is touched. `borg_rand_local` is initialised from
`randint1()` in `borg_init()`, so it follows the game seed.

## The borgh frontend

`angband -mborgh -n -uBorg -d... -- -seed N -race R -class C -turns T -keylog F -summary J [-menu-letters]`

1. It feeds a scripted key sequence, and only on blocking waits:
   splash ESC, race letter + Enter, class letter + Enter, Enter (point
   based), Enter (accept points), Enter (default name), `y` (history),
   Enter (final confirm). It then sets `NOSCORE_BORG` and
   `delay_factor = 0` and sends `^Z` `z` (start the Borg).
2. Once the script is used up it never produces a key, so the Borg's
   `user_abort()` never fires.
3. Key log: one line per key returned by `inkey_hack`:
   `<game turn> <event type> <keycode hex> <mods hex>`.
   `EVT_NONE` returns (the Borg idling while resting or repeating) are
   logged too.
4. Stops when, after a key, `turn >= -turns`, the player is dead,
   `lev >= -clev` or `max_depth >= -depth`. It also stops when the game
   blocks for a real key (`death`, `borg_stopped` or `blocked`). It then
   writes the summary JSON and calls `_exit(0)` (no save). If writing the
   key log or the summary fails it exits with 3.
5. `-screen` dumps the 80x24 screen to stderr on any stop that is not a
   turn-limit stop. `BORGH_DEBUG=1` dumps the screen before each scripted
   key.

### Race and class letters (known quirk)

By default the script picks race and class with `"abc...z"[index]`. The
birth menus use `all_letters_nohjkl` (`abcdefgimn...`, no h, j, k, l). So
for an index >= 7 the default picks a different entry, for races and
classes alike:

| index | letter sent | race picked | class picked |
|---|---|---|---|
| 0-6 | a-g | as requested | as requested |
| 7 | h (not a menu tag) | stays Human | stays Warrior |
| 8 | i (menu item 7) | Half-Troll | Ranger |
| 9 | j (not a menu tag) | stays Human | - |
| 10 | k (not a menu tag) | stays Human | - |

So `full` and `deep` never play a Blackguard, a Dunadan, a High-Elf or a
Kobold: their `*-ranger-*` runs are Warriors and their `*-blackguard-*`
runs are Rangers. (The header comments of `harness/suites/full` and
`deep`, "all 9 classes", "all 11 races", "one run per class", describe the
requested indices; the suite files are left unchanged.) The default is kept so the existing reference logs stay
valid. `-menu-letters` (suite flag `menu`) sends `all_letters_nohjkl[index]`
instead, so the indices mean `p_race.txt` order (0 Human ... 7 Half-Troll,
8 Dunadan, 9 High-Elf, 10 Kobold) and `class.txt` order (0 Warrior ...
7 Ranger, 8 Blackguard), and adds `"menu_letters": true` to the summary.
Every `summary.json` records the race and class actually played. The
Borg's own respawn code has the same C bug (`docs/c-bugs.md` #11).

## Scripts

* `harness/build.sh`: builds into `build/$BORG_IMPL` (`c` by default,
  `rust` for the Rust Borg). It always passes `-DBORG_IMPL`.
* `harness/run.sh OUTDIR SEED RACE CLASS TURNS [FLAGS]`: one isolated run.
  It gets its own user, save, archive, panic and scores dirs, and a copy of
  `harness/borg.txt` with `borg_respawn_race/class` set to the run's race
  and class. `TZ=UTC`, `LC_ALL=C.UTF-8`. FLAGS (comma-separated, `-` for
  none): `cheat` sets `borg_cheat_death = TRUE`; `menu` passes
  `-menu-letters`. Unknown flags are an error. A non-zero exit code or a
  missing summary fails the run.
* `harness/suite.sh SUITE [OUTDIR]`: runs `harness/suites/SUITE` in
  parallel (`JOBS`, default nproc). It runs into `OUTDIR.partial.<pid>` and
  replaces `OUTDIR` at the end, so an output dir never mixes two passes.
  It writes `manifest` (git revision and dirty flag, binary sha256, the
  binary's CMake `BORG_IMPL`, suite file sha256), `timing.txt` (always;
  failed runs are marked), and last `DONE` if every run succeeded or
  `FAILED` otherwise. A failed run does not stop the others; the suite
  then exits 1 and reports the missing runs as `MISSING`.
* `harness/diff [--suite NAME] A B`: compares two logs or two suite
  directories (`.xz` is accepted). In directory mode it compares the union
  of the run names (a run on one side only fails) and prints
  `N/M runs identical (expected M)`; `--suite NAME` also requires exactly
  the suite's runs on both sides. Per run it checks that the key logs are
  byte-identical, that on each side the log has `summary.keys` lines and
  ends at `summary.game_turns`, and that the two `summary.json` match apart
  from `wall_seconds`. It refuses a suite output dir without `DONE`
  (`--allow-incomplete` turns that into a warning; reference dirs hold only
  `.xz` logs and need no marker). For a divergence it prints run, seed,
  game turn, key index, and 20 lines of context.
* `harness/check-no-c-borg.sh`: checks that the Rust binary has no C Borg
  code: no `src/borg/*.c` compiled; every staticlib member is a rustc
  object or one of 35 known compiler-builtins C objects; no C-compiled
  symbol with a C Borg name in the binary; every engine object
  byte-identical between `build/c` and `build/rust` (only `buildid.c.o`
  may differ). It fails on empty or implausibly small inputs (no C Borg
  objects, stripped binary). It matches by name and member, so C code
  renamed on purpose would not be caught.

`harness/borg.txt` is the stock `src/borg/borg.txt` (cheat death off,
`borg_delay_factor = 0`).

## Suites and reference logs

| suite | runs | what it covers |
|---|---|---|
| `smoke` | 3 x 20k game turns | Human Warrior, Elf Mage, Dwarf Priest |
| `full` | 45 x 100k | early game: clevel 1-10 (median 4), max depth 1-5 (median 2), 15/45 die (all by clevel 5); 8 classes, 8 races (see the quirk) |
| `deep` | 9 x 1M, cheat death | clevel 8-30 (median 18), max depth 2-26 (median 7); 8 classes, 5 races |
| `classes` | 15 x 100k + 1 x 1M cheat death, flag `menu` | Blackguard with 7 races, Dunadan, High-Elf and Kobold with 7 other classes, a real Half-Elf Ranger; the 1M Dunadan Blackguard reaches clevel 26, max depth 22 |

`harness/reference/<suite>/<run>/{keys.log.xz,summary.json}` are the C
Borg's logs for each suite, made from the C build. Check a build with:

    BORG_IMPL=rust harness/suite.sh full
    harness/diff --suite full harness/out/rust/full harness/reference/full

The key oracle does not see Borg notes or screen output, Borg-side direct
writes to game state (they count only through later keys), anything after
the turn limit, or Borg config options other than `borg_cheat_death` (all
suites run with the stock `borg.txt`).
