# Deterministic Borg harness

The harness runs the unmodified Angband engine (pinned upstream
`e473720ed88a0f0c7ca66815cef3000eb4a38bc3`, in `angband/`) with a Borg
behind the `inkey_hack` seam. It records every key the Borg returns. Two
Borg implementations (C and Rust) are equivalent when their key logs are
byte-identical for the same seeds.

## Engine changes (all under `#ifdef BORG_HARNESS`; default build unchanged)

Enabled with `cmake -DSUPPORT_BORG_HARNESS=ON`. That defines `BORG_HARNESS`
for the core library and the executable, and adds the `borgh` frontend.

| file | change |
|------|--------|
| `src/main-borgh.c`, `src/main-borgh.h` | new headless frontend `-mborgh` (80x24, no drawing, no sleeping) |
| `src/main.c`, `src/main.h` | register the `borgh` module (`USE_BORGH`) |
| `src/z-rand.c` | `Rand_init()` uses `borgh_seed` when it is set, instead of `time()`/`getpid()` |
| `src/mon-make.c` | `get_mon_num()` uses a fixed date when the seed is fixed (no Christmas monsters) |
| `src/ui-input.c` | `borgh_key_hook` is called with every key returned by `inkey_hack` |
| `CMakeLists.txt` | option `SUPPORT_BORG_HARNESS` |

No file in `src/borg/` is touched. `borg_rand_local` is initialised from
`randint1()` in `borg_init()`, so it follows the game seed.

## The borgh frontend

`angband -mborgh -n -uBorg -d... -- -seed N -race R -class C -turns T -keylog F -summary J`

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
   writes the summary JSON and calls `_exit(0)` (no save).
5. `-screen` dumps the 80x24 screen to stderr on any stop that is not a
   turn-limit stop. `BORGH_DEBUG=1` dumps the screen before each scripted
   key.

## Scripts

* `harness/build.sh`: builds into `build/$BORG_IMPL` (`c` by default,
  `rust` for M2).
* `harness/run.sh OUTDIR SEED RACE CLASS TURNS`: one isolated run. It gets
  its own user, save, archive, panic and scores dirs, and a copy of
  `harness/borg.txt` with `borg_respawn_race/class` set to the run's
  race and class.
* `harness/suite.sh SUITE [OUTDIR]`: runs `harness/suites/SUITE` in
  parallel (`JOBS`, default nproc). It writes `timing.txt`.
* `harness/diff A B`: compares two logs or two suite directories (`.xz`
  is accepted). It prints the first divergence: run, seed, game turn, key
  index, and 20 lines of context.

`harness/borg.txt` is the stock `src/borg/borg.txt` (cheat death off,
`borg_delay_factor = 0`). Races are indexed as in `p_race.txt` (0 Human …
10 Kobold) and classes as in `class.txt` (0 Warrior … 8 Blackguard).

## Reference logs

`harness/reference/full/<run>/{keys.log.xz,summary.json}` are the C
Borg's logs for the `full` suite. Check a new build with:

    BORG_IMPL=rust harness/suite.sh full
    harness/diff harness/out/rust/full harness/reference/full
