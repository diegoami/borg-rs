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
6. Start-up `-more-`: if, after `^Z` `z`, the game blocks on a `-more-`
   prompt before the Borg has returned any key, the frontend answers it
   with ESC (at most 16 times) and the summary gets
   `"startup_acks": N`. This is the borg.txt warning that `borg_init()`
   prints with `msg()` for `borg_uses_dynamic_calcs` ("Borg's dynamic
   calculations enabled...") before the keypress hook is installed;
   without the answer both Borgs stopped at turn 1 with zero keys. The ESC
   is not a Borg key, so it is not in the key log; both builds get it at
   the same point. A run without such a prompt never blocks there (it
   would have stopped as `borg_stopped` with zero keys), so no other key
   log changes.
7. `-cheat-live` (suite flag `cheatlive`): the frontend turns on the
   game option `cheat_live` just before `^Z`. With `borg_cheat_death`
   off, `borg_init()` then sets the Borg's own cheat-death status from the
   game option, and at death the Borg answers "Die?" with `y` and
   reincarnates (`borg_reincarnate_start`: death menu, new game, birth
   with `borg_respawn_race/class`). The frontend does not stop on death
   with this option; it counts deaths (`"cheat_live": true`,
   `"deaths": N` in the summary) and the run goes on with the new
   character until the turn limit (the game turn restarts at 1 with each
   new character, so `game_turns` is the last character's).

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
  `rust` for the Rust Borg). It always passes `-DBORG_IMPL`. After a
  successful build it writes `build/<impl>/BUILD_ID` (see "Build identity"
  below); it deletes the old one first, so a failed build leaves none. It
  fails if the build inputs changed during the build or if the linked
  binary does not read as the requested implementation.
* `harness/identity.sh`: the helpers behind the build identity:
  `source-hash IMPL`, `inputs IMPL`, `inputs-dirty IMPL`, `binary-impl BIN`.
* `harness/run.sh OUTDIR SEED RACE CLASS TURNS [FLAGS]`: one isolated run.
  It gets its own user, save, archive, panic and scores dirs, and a copy of
  `harness/borg.txt` with `borg_respawn_race/class` set to the run's race
  and class. `TZ=UTC`, `LC_ALL=C.UTF-8`. FLAGS (comma-separated, `-` for
  none): `cheat` sets `borg_cheat_death = TRUE`; `menu` passes
  `-menu-letters`; `cheatlive` passes `-cheat-live` (see "The borgh
  frontend"). Config flags each set one more option of the run's
  `borg.txt`:
  * `munchkin` (`borg_munchkin_start = TRUE`), `lunal`
    (`borg_lunal_mode = TRUE`), `selflunal` (`borg_self_lunal = TRUE`);
  * `NAME=VALUE`, any option: the line `NAME = ...` becomes
    `NAME = VALUE`, e.g. `borg_money_scum_amount=5000`,
    `borg_uses_swaps=FALSE`, `borg_no_deeper=3`. `NAME` is the option's
    full name as in `borg.txt`, `VALUE` is `TRUE`, `FALSE` or an integer
    (the C parser reads a boolean from its first letter, an integer with
    `atoi`). The run fails (exit 2) if `NAME` has not exactly one
    `NAME = ` line in `harness/borg.txt` (an unknown or misspelt option),
    if `VALUE` is not of that form, if two flags give one option
    different values, or for `borg_cheat_death` (use `cheat`). These
    lines are rewritten after the ones `run.sh` sets itself, so
    `borg_respawn_race=-1` overrides the run's race.

  The run fails if a config flag's line is not in `harness/borg.txt`.
  Without config flags the run's `borg.txt` is the same as before they
  existed, byte for byte, so the stock suites are unchanged. Unknown flags
  are an error. Each run writes `config.txt` (its flags and the sha256 of
  its `borg.txt`); the suite manifest copies it as
  `run_config.<run>=<flags> <sha256>`. `harness/diff --suite` checks that
  `config.txt` records exactly the suite line's flags (so a reference made
  with other options fails); the key oracle itself does not read the
  hash. A non-zero exit code or a
  missing summary fails the run. It does not check the build identity:
  a single run is a debugging tool, suite outputs are the evidence.
* `harness/suite.sh SUITE [OUTDIR]`: runs `harness/suites/SUITE` in
  parallel (`JOBS`, default nproc). It runs into `OUTDIR.partial.<pid>` and
  replaces `OUTDIR` at the end, so an output dir never mixes two passes.
  Before any run it checks the binary's implementation and build identity
  and refuses (exit 2, no output dir) on a mismatch; see "Build identity".
  It writes `manifest` (the binary's implementation as each source sees
  it, its sha256 and `BUILD_ID` fields, the checkout's source hash, the
  result of both checks, `overrides=`, the checkout's git revision and
  dirty flag, suite file and `borg.txt` sha256), `timing.txt` (always;
  failed runs are marked), and last `DONE` if every run succeeded or
  `FAILED` otherwise. A failed run does not stop the others; the suite
  then exits 1 and reports the missing runs as `MISSING`. The binary is
  hashed again at the end; if it changed during the suite, the output is
  marked `FAILED`.
* `harness/diff [--suite NAME] [--expect-impl c|rust] A B`: compares two logs or two suite
  directories (`.xz` is accepted). In directory mode it compares the union
  of the run names (a run on one side only fails) and prints
  `N/M runs identical (expected M)`; `--suite NAME` also requires exactly
  the suite's runs on both sides, and that they are the suite's
  scenarios: a suite output's manifest must record this suite file
  (`suite_sha256`), and every run's `summary.json` (both sides) must show
  the suite's seed, race and class indices, menu-letter flag and a final
  turn consistent with its turn limit, and a config run's `config.txt`
  the suite's flags. (References do not record cheat death or an exact
  upper turn bound; see `harness/diff`'s help.) Per run it checks that the key logs are
  byte-identical, that on each side the log has `summary.keys` lines and
  ends at `summary.game_turns`, and that the two `summary.json` match apart
  from `wall_seconds`. It refuses a suite output dir without `DONE`
  (`--allow-incomplete` turns that into a warning; reference dirs hold only
  `.xz` logs and need no marker). For a divergence it prints run, seed,
  game turn, key index, and 20 lines of context. For each suite output
  dir it prints the build identity from the manifest and fails (exit 1)
  if the manifest shows `BORG_IMPL` different from the binary's
  implementation, a recorded override, a failed check, a binary changed
  during the suite. The role of a dir comes from its manifest, not from
  the log format: a dir whose manifest has the build identity is a suite
  output even if its logs are compressed (so compressing an output does not
  drop its checks); a reference is a dir without such a manifest and
  without plain logs, and if it keeps the manifest of its C run (as
  `reference/classes` does), that manifest must record a C build.
  `--expect-impl X` requires that a verified suite output of X takes part
  (either operand); otherwise the diff is refused (exit 2), e.g. two
  references, or a C output compressed to look like one. A
  suite output dir without a manifest, or with a manifest from before the
  build identity (no `overrides=` line), is refused (exit 2).
  `--allow-unverified` turns all of these into warnings. Reference dirs
  (no manifest, `.xz` logs only) are accepted as before; they are the C
  oracle by definition.
* `harness/check-no-c-borg.sh`: checks that the Rust binary has no C Borg
  code: no `src/borg/*.c` compiled; every staticlib member is a rustc
  object or one of 35 known compiler-builtins C objects; no C-compiled
  symbol with a C Borg name in the binary; every engine object
  byte-identical between `build/c` and `build/rust` (only `buildid.c.o`
  may differ); and (step 5) the actual link inputs: it re-runs the build's
  own link command (`CMakeFiles/OurExecutable.dir/link.txt`) with ld's
  `--trace` given twice into a temporary file, requires the result to be
  byte-identical to the binary, and then requires every input file and
  archive member to be one of the engine objects of step 4, a staticlib
  member accepted by step 2, or a system file under `/usr/lib`, `/lib` or
  `/lib64` (crt objects, libc, libm, libgcc, X11). It fails on empty or
  implausibly small inputs (no C Borg objects, stripped binary). It
  matches by name and member, so C code renamed on purpose would not be
  caught, and system libraries are trusted as a class.

`harness/borg.txt` is the stock `src/borg/borg.txt` (cheat death off,
`borg_delay_factor = 0`).

## Build identity

The key oracle only proves something about the binary that produced the
logs. Since review finding 4 of the external review (`REVIEW-4c06cec08`),
the harness ties a suite output to a binary and the binary to the sources:

* **`BUILD_ID`.** `harness/build.sh` writes `build/<impl>/BUILD_ID` next to
  the binary: `impl`, `git_rev`, `git_dirty` (whole checkout),
  `inputs_dirty`, `source_sha256`, `binary`, `binary_sha256` (after
  linking), the C compiler and `rustc` versions and the build time.
* **Source hash.** `harness/identity.sh source-hash IMPL` is sha256 over
  `<git blob id> <path>` of every build input as it is in the working
  tree: the files git tracks plus untracked files not ignored, under
  `angband/` for `c`, and under `angband/`, `borg-rs/`, `Cargo.toml` and
  `Cargo.lock` for `rust`. It depends only on contents and names, so
  committing a change or editing `docs/` or `harness/` does not change it,
  and a Rust change does not make the C binary stale. (The binary itself
  embeds `git describe` of the checkout through Angband's `version.sh`,
  so the binary's sha256 does change with every commit after a rebuild.)
* **Implementation.** `harness/identity.sh binary-impl BIN` reads the
  executable: `rust` if its `.comment` section names rustc (the Rust
  staticlib is linked in), `c` if not and it defines `do_cmd_borg` (the C
  Borg is linked in), `unknown` otherwise.

`harness/suite.sh` checks, before any run, and refuses with exit 2:

| check | refused when | override (recorded in the manifest) |
|---|---|---|
| implementation | the executable, `CMakeCache.txt` and `BUILD_ID` of the binary's build dir disagree, or the implementation they give is not `BORG_IMPL` (default `c`) | none |
| implementation | none of the three knows the implementation (e.g. a stripped external binary) | `ALLOW_UNKNOWN_IMPL=1` |
| build identity | no `BUILD_ID`; the binary's sha256 differs from `BUILD_ID`'s (binary changed after the build); `BUILD_ID`'s `source_sha256` differs from the checkout's (stale binary: rebuild) | `ALLOW_STALE_BUILD=1` |

An override that was needed is recorded as `overrides=...` and in
`impl_check=` / `build_check=` of the manifest, and `harness/diff` then
fails on that output unless given `--allow-unverified`. So the wrong-binary
command of the review,
`BORG_IMPL=rust ANGBAND_BIN=build/c/game/angband harness/suite.sh smoke`,
now stops before running, and an output dir made with an override cannot
pass the diff by default. To check a Rust build:

    BORG_IMPL=rust harness/suite.sh full
    harness/diff --suite full --expect-impl rust harness/out/rust/full harness/reference/full

Limits: make and cargo decide what to rebuild from timestamps and
fingerprints, so `BUILD_ID` trusts the build system that the binary was
rebuilt from the inputs it hashes; the toolchain is recorded but not
part of the source hash; the checks protect against mistakes, not against
someone forging a manifest or a `BUILD_ID`. The references are made from
the C build; `smoke`, `full` and `deep` carry no manifest (they predate
it); `classes` keeps the manifest of the C run that made it (no build
identity), and `config` the full manifest of its C run (checked like a C
suite output).

## Suites and reference logs

| suite | runs | what it covers |
|---|---|---|
| `smoke` | 3 x 20k game turns | Human Warrior, Elf Mage, Dwarf Priest |
| `full` | 45 x 100k | early game: clevel 1-10 (median 4), max depth 1-5 (median 2), 15/45 die (all by clevel 5); 8 classes, 8 races (see the quirk) |
| `deep` | 9 x 1M, cheat death | clevel 8-30 (median 18), max depth 2-26 (median 7); 8 classes, 5 races |
| `classes` | 15 x 100k + 1 x 1M cheat death, flag `menu` | Blackguard with 7 races, Dunadan, High-Elf and Kobold with 7 other classes, a real Half-Elf Ranger; the 1M Dunadan Blackguard reaches clevel 26, max depth 22 |
| `config` | 6 runs of 100k/20k + 1 x 1M, flags `menu,cheat` plus `munchkin` or `lunal` | Borg config options: munchkin start and lunal mode (see below) |

`harness/reference/<suite>/<run>/{keys.log.xz,summary.json}` are the C
Borg's logs for each suite, made from the C build. Check a build with:

    BORG_IMPL=rust harness/suite.sh full
    harness/diff --suite full --expect-impl rust harness/out/rust/full harness/reference/full

The key oracle does not see Borg notes or screen output, Borg-side direct
writes to game state (they count only through later keys), anything after
the turn limit, or Borg config options other than `borg_cheat_death`,
`borg_munchkin_start` and `borg_lunal_mode` (the `config` suite; all other
suites run with the stock `borg.txt`).

### The config suite

`harness/suites/config` replays the runs the second external review
(`REVIEW-821cad310`) made by hand, plus one long run. Every run uses
`menu,cheat` and one config flag:

| runs | config | characters | turns |
|---|---|---|---|
| `cfg-munchkin-*` | `borg_munchkin_start = TRUE` | Human Warrior seed 42, Half-Elf Mage 12345, Dunadan Blackguard 1080 | 100k, 100k, 20k |
| `cfg-lunal-*` | `borg_lunal_mode = TRUE` | the same three | 100k, 100k, 20k |
| `cfg-lunal-deep-warrior` | `borg_lunal_mode = TRUE` | Human Warrior seed 2007 | 1M |

The Blackguard munchkin run stops at 20k game turns because it is slow:
137k keys by turn 20,011. The suite takes about 4 minutes with `JOBS=2`.

What it reaches (checked once with a temporarily instrumented C build, not
committed):

* Munchkin: `borg_think_dungeon_munchkin` runs in all three munchkin runs
  from dungeon level 1 on, and `munchkin_mode` changes the stair choice.
  The munchkin runs reach clevel 8-12, max depth 2-8.
* Lunal: `borg.lunal_mode` is set from the config at start and stays on in
  all these runs. It changes the flow code: down stairs are taken without
  the prepared check, and resting is skipped. But the lunal thinker
  `borg_think_dungeon_lunal` is never entered: its only caller is the
  self-lunal check in `borg_think_dungeon` (C `borg-think-dungeon.c:1439`),
  which needs `borg_self_lunal` and a borg standing at least 15 levels
  above its max depth, prepared for 15 levels deeper, and below
  clevel / 3.
* Self-lunal: not covered. With `selflunal` added, eight 1M cheat-death
  runs (seeds 2000, 2001, 2005, 2007 with and without `lunal`, and the
  `classes` Dunadan Blackguard 3100) never passed that check; the five
  without `lunal` have key logs identical to their `deep`/`classes`
  references, since `borg_self_lunal` is read nowhere else. So the
  `selflunal` flag exists, but no suite run uses it.

`harness/reference/config` was made like the other references: the C build
ran the suite twice, the two outputs were identical, and one of them was
stored as `.xz` logs, `summary.json`, `config.txt`, `timing.txt`, its
manifest (which records each run's flags and `borg.txt` hash as
`run_config.<run>`) and its `DONE` marker. Because the manifest has the
build identity, `harness/diff` treats this reference as a compressed C
suite output: it prints the C build's identity and checks it (C, no
override, `DONE`), unlike the older references, which have no such
manifest.
