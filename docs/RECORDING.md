# Recording and replay: what exists

The overview of every recording and replay feature in the repository, written
for [#62](https://github.com/diegoami/borg-rs/issues/62) (recordings with
branches). Each section points at the detailed document; this page only says
what each piece is, how it is switched on, and what it can and cannot do.

The short version:

* **The native recording** (§4) is the recording: it captures everything the
  game reads from outside (input events, the clock, the PID that seeds the
  RNG, the bytes of every file), so a run can be fed back and checked byte
  for byte.
* **The replay toolkit** (§5) is the user-facing layer on top of it: record a
  game, replay and verify it, view it frame by frame, and branch from a turn
  into a new human session.
* Everything else (§1-§3) is test infrastructure for C/Rust parity: logs and
  checkpoints for comparing two builds, not recordings for people.

| # | Feature | Switched on by | From the start? | Replay | Branch |
| --- | --- | --- | --- | --- | --- |
| 1 | Three-log oracle | the harness build (always) + `-keylog/-msglog/-statelog` | yes | none: rerun the seed and diff | no |
| 2 | Saved-character runs | `-load FILE` | starts from a save | fresh Borg from the save | - |
| 3 | Supervised checkpoints | `CHECKPOINT_EVERY=N` | yes, at a fixed cadence | from a checkpoint, C/C/Rust compared | from a checkpoint |
| 4 | Native recording | build `SUPPORT_BORG_RECORDING=ON`, run `BORG_RECORD_DIR` | yes (`record_init` in `main`) | `BORG_REPLAY_DIR`, verified | - |
| 5 | Replay toolkit | `scripts/start-borg.sh`, `scripts/run-borg-headless.py` | yes | `scripts/replay-borg.py` (verify, view) | `scripts/play-from-replay.py --play-from-turn N` |

## 1. The three-log oracle (parity)

* `angband/src/main-borgh.c` (the headless front end, `SUPPORT_BORG_HARNESS`,
  always on in `harness/build.sh`), driven by `harness/run.sh` and
  `harness/suite.sh`.
* `-seed N` (or `ANGBAND_SEED`) replaces the clock and the PID in the RNG's
  seeding; a fixed seed also fixes the date.
* Writes `keys.log` (every key), `messages.log` (every message), `state.log`
  (one engine-state digest per key) and `summary.json`.
* There is no replay: determinism is shown by running the same seed again and
  comparing with `harness/diff`. The stored references are in
  `harness/reference/`.
* Doc: [harness.md](harness.md).

## 2. Saved-character runs

* `-load FILE` starts the headless game from an engine save instead of a new
  character; `-save-on-stop FILE`, `-make-fixture DEPTH` and `-near-morgoth`
  make such saves. The fixtures are in `harness/saves/`
  (`harness/make-fixtures.sh`, `harness/scenarios/`).
* The Borg starts fresh at the save: its private memory (the map it has
  learned, its goals) is not in an engine save.
* Docs: [harness.md](harness.md), `harness/saves/README.md`,
  `harness/scenarios/README.md`.

## 3. Supervised checkpoints

* `CHECKPOINT_EVERY=N` (`harness/run.sh`, `harness/supervise.py`,
  `-checkpoint-every` in `main-borgh.c`): a save every N game turns, written
  by a forked child at a command boundary, with an index
  (`checkpoints/index.jsonl`, `checkpoints.json` with SHA-256).
* `harness/replay-checkpoint.py SOURCE CP OUT` runs from a checkpoint (C twice,
  Rust once) and reports MATCHED or FAILED.
* Doc: [checkpoints.md](checkpoints.md).

## 4. The native recording

* `angband/src/native-replay.c` / `.h`, with hooks in `main.c`, `ui-input.c`,
  `ui-term.c`, `ui-game.c`, `message.c`, `z-file.c`, `z-rand.c`, `savefile.c`.
* Build: CMake `SUPPORT_BORG_RECORDING` (default OFF; `scripts/run-rust-borg.sh`
  and the death workflow build with it ON, `harness/build.sh` does not). Run:
  `BORG_RECORD_DIR=<dir>`.
* Must be on from process start: `record_init` runs in `main`, before the RNG
  is seeded and before the first file is read.
* Records:
  * `events.txt`: every external input event;
  * `clock.txt`, `pid.txt`: the clock and the PID, which seed the RNG;
  * `file-inputs.txt`, `file-input-NNNNNN.bin`: the bytes of every file read
    (prefs, `borg.txt`, ...);
  * `actions.txt`: the Borg's keys;
  * `native-messages.jsonl`, `geometry.json`;
  * `frames.jsonl`: per frame, the screen delta, HP/SP/depth/gold, the full RNG
    state and a world fingerprint (a hash of a save written by a forked child:
    a fingerprint, not a resumable save);
  * `record-complete.json`: the footer.
* Replay: `BORG_REPLAY_DIR=<dir>` (plus `BORG_REPLAY_SYS`) feeds the recorded
  inputs, clock, PID and file bytes back. It never supplies the Borg's
  decisions or the RNG: those are recomputed, which is what makes the replay a
  check. A stream left unconsumed means the footer is not written and the
  verification fails.

## 5. The replay toolkit

* Launchers in `scripts/`: `start-borg.sh NAME` and `run-rust-borg.sh` (GUI),
  `run-borg-headless.py`, `replay-borg.py`, `play-from-replay.py`,
  `play-branch.py`, `produce_death.py`, `replay_next_death.py`; the library is
  `tools/` (`borg_runtime.py`, `borg_gui.py`, `toolkit_native.py`,
  `human_takeover.py`, `borg_replay_viewer.py`, `borg_saves.py`, ...).
* A recording is a bundle (`borg-rs-headless-recording@1`,
  `borg-rs-gui-recording@1`): `recording.json` freezes the executable, the
  data, the config, the origin save and the seed, and the bundle holds the
  native streams (§4), `keys.log`, `messages.log`, `frames.jsonl[.gz]`,
  `combat.jsonl`, `final.save` and `verification.json`. Bundles live under
  `.angband-rust/recordings/` (`BORG_PLAY_DATA_DIR`).
* `scripts/replay-borg.py BUNDLE`:
  * `--verify-only`: rerun the frozen executable from the start and require
    every channel and save to be byte-identical; writes a certificate;
  * `--view-only`, `--from-turn N`: a frame viewer with seek, inventory,
    spells and the combat log;
  * `--play-from-turn N` (also `scripts/play-from-replay.py --from-turn
    N|latest`): verify the prefix, take a living save at the first command
    boundary at or after turn N, and open it for a **human**, Borg off. The
    human session is recorded as well. `scripts/play-branch.py DIR` resumes
    such a branch.
* Docs: [REPLAY-TOOLKIT.md](REPLAY-TOOLKIT.md),
  [TOOLKIT-IMPLEMENTATION.md](TOOLKIT-IMPLEMENTATION.md),
  [tools/README.md](../tools/README.md).
* Tests: `harness/regress/{gui-replay,human-takeover,toolkit,launcher-save,death-recording}`.

## Not recordings

* The Borg's own logging (`borg_save_death`, `borg_dump_level`,
  `borg_autosave` in `borg.txt`) writes its log and map files; there is no
  save-scumming or rewind. `borg_cheat_death` is forced off by `harness/run.sh`.
* `harness/native-checker`: a C/Rust differential checker; "native" means
  native code, not the native recording.
* On branch `feature/town-inventory-strategy` only (frozen): the town state
  dump, the object log and the plan record. These are observations, not
  replayable recordings.

## What #62 adds on top

What exists already gives record → verified replay → one branch from a turn.
What is missing for #62:

* **one file** that holds the original and its branches as a tree, instead
  of a separate bundle per branch;
* **navigation**: a viewer that picks a branch, not only a frame;
* **portability**: a recording that replays on someone else's build of the
  same version, not only on the frozen local executable;
* **size**: the inputs plus periodic snapshots, with frames derived on replay;
* **starting from a save** (switching recording on in the middle of a game):
  see the next section.

## Starting a recording from a save

What a recording must hold so that "a save, then everything after it"
replays the rest of the game exactly. These findings come from reading the
source (2026-11-14); nothing here has been run yet.

### Already handled

| Input | Why it matters | Covered by |
| --- | --- | --- |
| The RNG | every roll | the save: block `"rng"` (`wr_randomizer` / `rd_randomizer`) restores `Rand_value`, `state_i` and `STATE[]`, over whatever the clock and PID seeded at start-up. The headless front end keeps it with `-keep-load-rng`, which the toolkit already passes (`save=origin.save,keep-load-rng`) |
| The options, messages, monster and object memory, quests, the player, gear, stores, the level, artifacts, history | the game state | the save's other blocks (`savefile.c`) |
| The flavour and randart seeds | object names, randarts | the save (`seed_flavor`, `seed_randart` in `rd_misc`) |
| The clock | `time()` is read during play, not only at start-up: `get_mon_num` (`mon-make.c`) reads the date on every monster generation (holiday monsters), as do the death screen and the file-dump names | the native recording wraps `time()` and records every call (`clock.txt`). The stream simply starts at the save |
| Key timing | lazy movement and instant scans (`inkey_scan`, `ui-input.c`) depend on whether a key arrived before a timeout | the native recording records every terminal poll, the empty ones included, with its wait/take mode and the key queue's head and tail (`events.txt`) |
| Files | keymaps, colours and window settings come from `.prf` files (`pref.prf`, `window.prf`, `user.prf`, the character's own `.prf`, read once at the end of start-up: `process_character_pref_files` in `ui_leave_init`, `ui-display.c`), and the Borg reads `borg.txt` | the native recording copies every file at the moment it is read (`file-inputs.txt`, `file-input-NNNNNN.bin`) |
| The terminal size | the screen layout, the panel, `-more-` prompts | recorded with every input event; replay resizes |

### Not in a save, and lost when the game is reloaded

* the keymaps (they are in no save block: they come from the `.prf` files,
  or from edits made in the game and not dumped);
* the current target (`target.c`'s statics) and the command repeat state
  (`cmd-core.c`);
* the `-more-` and menu state on the screen, and any half-typed command;
* the Borg's private memory: its map of the level, its goals and queues.

### The trap: saving changes the running game

The ordinary saver compacts the monster list (`compact_monsters` at the end
of `wr_dungeon`, `save.c`) and changes save flags in the live game. That is
why the checkpoints of §3 save in a forked child (see
[checkpoints.md](checkpoints.md)), so that the running game is left
unchanged. Either way, the running game and a fresh load of its save are
not the same state:

* with an in-process save, the running game has the compacted list, but it
  still has the target, the repeat state, the screen and the Borg's memory
  that a reload would not;
* with a forked save, the running game also keeps its uncompacted monster
  order, so its iteration order, and with it the order in which it consumes
  the RNG, can differ from the reloaded copy.

So a recording cannot just start "from now" in the running process: the
same keys, replayed from the save, could diverge from what the original
process went on to do.

### The answer: switch on = save and reload

Switching recording on in the middle of a game should do exactly what the
replay will do:

1. wait for a command boundary (no prompt open, the key queue empty);
2. save;
3. restart from that save, with the recording on from process start. This is
   the existing native recording, started on an existing save, which the
   toolkit already does for named characters.

The original game and every replay then start from the same loaded state,
so no key is lost: the recording begins with the state the next key acts
on. What the reload drops (the target, the repeat state, the Borg's memory)
is dropped in the original and in every replay alike, so it costs nothing
in determinism. For the Borg, it means a Borg restarted at that point,
which is also what branching from a turn does
(`scripts/play-from-replay.py`).

### Checked by running it (2026-11-14)

A recording that starts from a mid-game save replays exactly:

1. `scripts/run-borg-headless.py SaveTest --new --seed 4242 --race 0 --class 0
   --turns 20000`: a Human Warrior from birth, alive at clvl 10, depth 6.
2. `scripts/run-borg-headless.py SaveTest --turns 20000`: the same character,
   loaded from that save in a new process (`run_kind`: "save continuation;
   fresh Borg"; `rng_origin`: "saved engine RNG preserved"). Its frozen
   `origin.save` is byte-identical to run 1's `final.save` (SHA-256
   `225f17f3…`). It played game turns ~20,000 → ~38,000: 2,996 Borg keys,
   2,006 terminal events, 49 clock reads and 108 file reads, all recorded.
3. `scripts/replay-borg.py <run 2> --verify-only`: **VERIFIED**. The rerun
   from the frozen origin reproduced the keys, the messages, the player state,
   the terminal frames, the 18 scheduled world saves and the final save,
   byte for byte.

What this does not cover:

* it is one headless run, played by the Borg, one class. A human-played GUI
  recording from a save is the same mechanism, but it was not run;
* the headless front end pins the holiday date to the epoch, so the clock's
  effect on monster generation is not exercised here (the clock stream itself
  was recorded and fed back);
* the save came from a session that ended normally. The "switch on in game"
  trigger (save, then restart from it) does not exist yet. This check covers
  the restart half of it.

### Still to check

* Look for anything else the game reads from outside after a load. The
  read-time file capture and the `time()` wrapper should catch it, but an
  unconsumed or missing stream is the signal.
* The cost of the restart in the GUI: a visible flicker, or a reload in
  place through the game's own load path instead of a new process.
