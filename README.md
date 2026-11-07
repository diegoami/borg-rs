# borg-rs

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
./replay-borg.py .angband-rust/recordings/RUN --turn 5000
./replay-borg.py .angband-rust/recordings/RUN --turn 5000 --dump
```

GUI/headless launchers share a per-character lock and five rolling backups
under `.angband-rust/save/backups/NAME/save.1` through `save.5`. Restore keeps
the current save as `before-restore-TIMESTAMP.save`. Headless play preserves
the saved engine RNG, starts a fresh Borg, and records its origin, frozen
Rust executable/data/config, inputs, frames and engine saves. It uses world-
turn, key and wall budgets; a death publishes a dead native save and retains
the latest living checkpoint in backup1 when available. Resume a dead character
only with explicit backup restore or `--new`.

Replay verifies a rerun from the original start before showing actual frames;
use left/right to step, space to play, g to jump, i/m for recorded engine
inventory/monster diagnostics, and q to quit. Diagnostic hidden information
is never supplied to the Borg. This is **new headless recordings only**:
interactive GUI input recording and C-only private Borg diagnostic fields
are not ported. Historical runs without a recording cannot claim exact replay.
See [tools/README.md](tools/README.md) for verification scope and provenance.

* `angband/`: upstream Angband at `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
  (git subtree), plus small `#ifdef BORG_HARNESS` harness hooks and the documented
  matching fork corrections in `docs/FORK-CORRECTIONS.md`
* `harness/`: build/run/suite scripts, seed suites, diff tool, pinned
  `borg.txt`, C reference logs
* `docs/`: `REVIEW.md` (start here), `STATUS.md`, `PROGRESS.md`, `c-bugs.md`, `harness.md`

Licence: GPLv2 (see `LICENSE`). Angband itself is dual-licensed GPLv2 / Angband
licence, and upstream copyright headers are kept.
