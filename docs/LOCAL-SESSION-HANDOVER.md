# Local-session handover — 2026-11-06

Continue on `claude/serene-ramanujan-8hzsfm`. M4 is unfinished. Main and
tags have not moved. This is a WIP transfer, not a passed milestone.

## Owner policy — authoritative update

Crash-causing C behavior and blatant bugs that prevent play **may be
corrected now**. Match the narrow correction in C and Rust where applicable.
Other conflicts are decided case by case. Do not insist on preserving
crashes, undefined bytes, or known failures as milestone requirements.
Broader divergence follows a verified milestone and owner-created tag.
See `FORK-CORRECTIONS.md`; this overrides older absolute fidelity wording.

Keep original references separate. New corrected-C references require two
identical C runs. File C findings/updates in `diegoami/neo-angband-tracker`;
never post to `angband/angband`. The owner creates tags.

## Read first

1. This file, then `docs/HANDOVER.md` and `CLAUDE.md`.
2. `docs/FORK-CORRECTIONS.md`, `docs/M4-PLAN.md`, `docs/REVIEW.md`.
3. `docs/harness.md` and `docs/checkpoints.md`.

Give the owner frequent concrete progress updates. Do not say a long run
passed until its `DONE` marker and strict comparison exist. The cloud UI
stopped rounds during automatic review without providing this worker a
specific rejected action/reason; background process handles also became
unavailable. Prefer foreground, logged, bounded local batches.

## Committed implementation

* `angband/src/borg/borg-update.c` and `borg-rs/src/borg_update.rs`:
  stale offsets reset at `>=` cave size rather than only `>`.
  Tracker: https://github.com/diegoami/neo-angband-tracker/issues/290.
* `angband/src/borg/borg-io.c` and `borg-rs/src/borg_io.rs`:
  multibyte screen conversion reserves NUL space and returns empty text/error
  on invalid encoding. Tracker:
  https://github.com/diegoami/neo-angband-tracker/issues/287.
* Both corrections are marked `FORK`. The actual C-function regression is
  `harness/regress/screen-text/run.sh`.
* `harness/suites/crash-fixes` contains the original Rogue seed4006 at its
  full 2M-turn limit plus all five original endgame scenarios, unchanged.
* Eight natural Rogue savefiles, source provenance and the
  `captured-rogue` suite/reference are committed. Two C capture passes
  produced identical save bytes; every fresh replay matched C/C/Rust in
  keys, messages and player state before the new corrections. These are
  fresh Borgs, not a resumed uninterrupted 2M-turn trace. Recheck them with
  the corrected builds.
* Previous commit `e0f93e7` added one-shot process supervision, timeouts,
  fork-isolated engine checkpoints and `harness/replay-checkpoint.py`.

## What actually passed

Corrected builds compile. C smoke 3/3 and Rust smoke 3/3 match the original
references in all three logs. Release Rust tests: 100 passed, 1 existing
ignored. Release clippy with warnings denied passed. The real C screen-text
regression failed before the correction and passes afterward, including
UTF-8, output-boundary and invalid-character controls.

Saved logs and source/binary hashes are under
`docs/validation/2026-11-06-preflight/`. They were built from dirty sources
at `e0f93e7` before committing the corrections; the receipt records that
honestly. Rebuild locally to bind validation to the transferred commit.

**Not established:** a completed corrected Rogue 2M run, full corrected
endgame C/C/Rust parity, the post-change 105-case checkpoint, a full
extended-suite pass, or M4 completion. Cloud `.partial.*` directories and
isolated completed cases do not establish those results. No corrected-C
six-case reference has been promoted.

## Start locally (Linux or WSL)

Use a normal clone with GCC/make, CMake, X11 development headers, Python3,
`bc`, `xz`, and Rust/cargo/clippy available. The harness is POSIX-based;
native Windows execution is not validated. Rust 1.97.0 was used here.
Do not copy cloud `/tmp` dependency paths into the local setup.

```sh
git fetch origin
git switch claude/serene-ramanujan-8hzsfm
git pull --ff-only origin claude/serene-ramanujan-8hzsfm

mkdir -p harness/out/local
harness/build.sh >harness/out/local/build-c.log 2>&1
BORG_IMPL=rust harness/build.sh >harness/out/local/build-rust.log 2>&1
harness/regress/screen-text/run.sh
harness/check-no-c-borg.sh
```

Use separate shell commands and inspect each exit code. Do not run a later
step if the preceding build/check fails.

Run the targeted full scenarios first, sequentially, with logs:

```sh
BORG_IMPL=c JOBS=2 harness/suite.sh crash-fixes harness/out/local/crash-c1 >harness/out/local/crash-c1.log 2>&1
BORG_IMPL=c JOBS=2 harness/suite.sh crash-fixes harness/out/local/crash-c2 >harness/out/local/crash-c2.log 2>&1
harness/diff --suite crash-fixes --expect-impl c harness/out/local/crash-c1 harness/out/local/crash-c2

BORG_IMPL=rust JOBS=2 harness/suite.sh crash-fixes harness/out/local/crash-rust >harness/out/local/crash-rust.log 2>&1
harness/diff --suite crash-fixes --expect-impl rust harness/out/local/crash-rust harness/out/local/crash-c1
```

The old `endgame` C reference contains the original depth98 phantom-message
defect. Do not expect the corrected C message stream to reproduce it or
overwrite it silently. Accept a new, separately identified corrected-C
reference only after the two C runs match. Any remaining Rust/C mismatch
must be investigated, not assumed to be the same known defect.

## Remaining roadmap

1. Finish the six-case validation above; investigate any residual fault.
2. Recheck the baseline suites `smoke full deep classes config options`
   (105 cases) against their existing references. Run release tests and
   clippy again after any code changes.
3. Finish M4 C: `extended` has seeds4000–4008 at 2M turns, and seeds2000,
   2007 and3100 at 5M turns. Run corrected C twice and Rust once. Early
   runs reached depth34; no complete extended reference is accepted yet.
4. Finish M4 D: five synthetic saves at depth50,98,99,100 and near Morgoth.
   State which tactics actually execute; depth alone does not prove Sea
   of Runes, self-lunal or other particular endgame behavior.
5. M4 E: implement and commit a native C/Rust differential checker.
   Historical `xcheck-wip`, `xcheck2`, `xcheck3` branches are mentioned in
   older docs but were absent from this cloud checkout; locate them in
   the owner's local repository, or build the checker anew.
6. M4 F: measure Rust coverage, report gaps and add targeted checks.
7. Complete the final checkpoint, update docs and tracker receipts, then
   present the milestone for the owner to tag before broader divergence.

The two tracker issues were filed earlier, but have not yet received the
new patch links/native crash trace and correction validation. Update them
after the correction commit is published. Leave upstream/source resolution
and independent tracker verification explicit; do not claim either.
