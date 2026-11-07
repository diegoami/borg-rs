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

* `angband/`: upstream Angband at `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
  (git subtree), plus small `#ifdef BORG_HARNESS` harness hooks and the documented
  matching fork corrections in `docs/FORK-CORRECTIONS.md`
* `harness/`: build/run/suite scripts, seed suites, diff tool, pinned
  `borg.txt`, C reference logs
* `docs/`: `REVIEW.md` (start here), `STATUS.md`, `PROGRESS.md`, `c-bugs.md`, `harness.md`

Licence: GPLv2 (see `LICENSE`). Angband itself is dual-licensed GPLv2 / Angband
licence, and upstream copyright headers are kept.
