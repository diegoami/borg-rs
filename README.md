# borg-rs

A faithful Rust port of the Angband Borg (`angband/src/borg/`). It is verified by
checking that, for the same seed and the same unmodified C engine, the Rust Borg
produces a byte-identical keypress stream to the C Borg.

* `angband/`: upstream Angband at `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
  (git subtree), plus small `#ifdef BORG_HARNESS` harness hooks
* `harness/`: build/run/suite scripts, seed suites, diff tool, pinned
  `borg.txt`, C reference logs
* `docs/`: `REVIEW.md` (start here), `STATUS.md`, `PROGRESS.md`, `c-bugs.md`, `harness.md`

Licence: GPLv2 (see `LICENSE`). Angband itself is dual-licensed GPLv2 / Angband
licence, and upstream copyright headers are kept.
