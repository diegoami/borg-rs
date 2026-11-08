# borg-rs

Current checkpoint: owner-accepted [M5](docs/M5-PLAN.md), tag `m5` at
`0203227`, with independent pass and [review#2 closed](https://github.com/diegoami/borg-rs/issues/2).
Subsequent development follows [verified better-borg-c milestones](docs/C-CONSUMER-ADOPTION.md).
M4 remains accepted. The owner selected C milestone2's seven fixes at
`2675d198e`; adoption/validation is tracked on [issue#6](https://github.com/diegoami/borg-rs/issues/6)
and [the batch ledger](docs/adoption/c-m2/README.md). No new Rust milestone
acceptance is inferred.

A faithful Rust port of the Angband Borg (`angband/src/borg/`). It is verified by
checking that, for the same seed and the same unmodified C engine, the Rust Borg
produces a byte-identical keypress stream to the C Borg.

Run `./run-rust-borg.sh` to build and open the graphical X11 game with the
Rust Borg. Create/load a character, then press **Ctrl-Z**, **z** (confirm
the initial warning if shown). Any key stops the Borg. Saves and settings
stay in `.angband-rust/`; the initial configuration has cheat death off.
Requires CMake, a C compiler, Rust/Cargo and X11 development libraries.
Use `./run-rust-borg.sh --help` for build and game arguments.
The main window uses the same 28-pixel Courier font as `better-borg-c/start-borg.sh`;
set `BORG_PLAY_FONT` to choose another X11 font. **Ctrl-X** saves and exits;
the default save is `.angband-rust/save/RustBorg` (`-uNAME` selects another).

From this project root:

```sh
./run-rust-borg.sh                   # Resume RustBorg, or create it if absent
./run-rust-borg.sh -uMyCharacter     # Select another named save
./run-rust-borg.sh -n -uNewCharacter # Create a new named character
./run-rust-borg.sh --build-only     # Build without opening a window
```

Named saves use `.angband-rust/save/NAME`; settings are in `.angband-rust/user/`.

Named launch, backup restore, headless play and replay (adapted from
`better-borg-c`):

```sh
./start-borg.sh MyCharacter
./start-borg.sh MyCharacter --from-backup=1
./run-borg-headless.py MyCharacter --turns 100000
./run-borg-headless.py NewCharacter --new --seed 42 --class 0 --race 0
./replay-borg.py .angband-rust/recordings/RUN --from-turn 5000
./replay-borg.py .angband-rust/recordings/RUN --verify-only
./replay-borg.py .angband-rust/recordings/RUN --dump-turn 5000
```

Bounded death collection and an explicit owner-review queue:

```sh
./produce_death.py                      # Up to 3 planned games; first death/failure stops
./produce_death.py --birth fixed --seeds 8400 8401 --turns 100000
./replay_next_death.py --list
./replay_next_death.py --from-turn 37000 # Verify and view the next unreviewed recording
./replay_next_death.py --mark-reviewed D0005
```

Closing replay does not mark it reviewed. Collection uses private fresh profiles
and keeps every attempted outcome; it does not enlarge limits to force a death.
See [workflow parameters and scope](tools/README.md#collecting-a-death-and-reviewing-the-next-encounter).

GUI/headless launchers share a per-character lock and five rolling backups
under `.angband-rust/save/backups/NAME/save.1` through `save.5`. Restore keeps
the current save as `before-restore-TIMESTAMP.save`. Headless play preserves
the saved engine RNG, starts a fresh Borg, and records its origin, frozen
Rust executable/data/config, inputs, frames and engine saves. It uses world-
turn, key and wall budgets; a death publishes a dead native save and retains
the latest living checkpoint in backup1 when available. Resume a dead character
only with explicit backup restore or `--new`.

GUI sessions now print a recording path too. They preserve the original save,
executable/data, terminal polls and human input, clock values, startup PID used
by engine RNG, timezone rules, and bytes of files read during play (including
later preference reloads). They use the ordinary save path and keep your user
settings. The recorder is Linux/X11 text mode; headless replay needs no display.
Recordings include a frozen executable and can grow during long sessions.

Replay verifies a rerun from the original start before showing actual frames;
use left/right to step, space to play, g to jump, **i/e** for player-known
inventory/equipment, **s** for native spell readiness, **p** for action/message
history (Tab changes source), **d/m** for diagnostic gear/monsters, and q to quit. Diagnostic hidden information
is never supplied to the Borg. Both new GUI and headless recordings work.
C-only private Borg diagnostic fields are omitted. Historical runs without
a recording cannot claim exact replay; interrupted recordings can be viewed
with `--recording-only` and an explicit UNVERIFIED badge.
See [tools/README.md](tools/README.md) for verification scope and provenance.

Replay accepts better-borg-c's `--from-turn`, `--dump-turn`, `--view-only`,
`--verify-only`, `--output` and `--timeout`. Older Rust `--turn`, `--dump`
and `--recording-only` forms remain supported. The turn selects **viewing**;
verification reruns the full original session, preserving private Borg history.

To try your own choices from a recorded headless game:

```sh
./replay_next_death.py --case D0006 --play-from-turn 22794
# Or: ./replay-borg.py docs/deaths/D0005-recorded --play-from-turn 36094
```

This verifies the historical run, copies a living native save at the first
command boundary **at or after** your requested turn, and opens it with the
Borg off. It prints the actual turn, save path and `play-branch.py` resume
command. These are separate game sessions/saves, with originals preserved.
Default copies live in `.angband-rust/human-branches/`; use `--output NEW_DIR`
to choose a directory, or `--prepare-only` to create the save without opening
X11. Ctrl-X saves; Escape closes the native exit screen. See
[human takeover scope](tools/README.md#playing-from-a-recorded-turn).

Completed natural deaths automatically create a dossier in `docs/deaths/`,
with native maps/death logs, build/input hashes, actual pre-death observations,
checkpoint metadata and an unassessed diagnosis/action file. Replay accepts a
dossier directory too. [Registry and existing cases](docs/deaths/README.md).
[Add owner feedback on death-review hub #3](https://github.com/diegoami/borg-rs/issues/3).
Set `BORG_DEATH_RECORDS_DIR` for a separate registry; attach a discussion with
headless `--death-issue URL` or GUI `BORG_DEATH_ISSUE`. No issue is posted
automatically, and replays/backup branches do not inflate independent deaths.

* `angband/`: upstream Angband at `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
  (git subtree), plus optional harness/GUI recording hooks and the documented
  matching fork corrections in `docs/FORK-CORRECTIONS.md`
* `harness/`: build/run/suite scripts, seed suites, diff tool, pinned
  `borg.txt`, C reference logs
* `docs/`: `REVIEW.md` (start here), `STATUS.md`, `PROGRESS.md`, `c-bugs.md`, `harness.md`

Licence: GPLv2 (see `LICENSE`). Angband itself is dual-licensed GPLv2 / Angband
licence, and upstream copyright headers are kept.
