# Handover

Local continuation: read `docs/LOCAL-SESSION-HANDOVER.md` first. It records
the owner's revised correction policy, transferred WIP and exact next checks.

Owner-authorized crash/play-blocker corrections are now allowed in C and
Rust; see `docs/FORK-CORRECTIONS.md`. Other behaviour conflicts are decided
case by case. Broader divergence follows a milestone and owner-created tag.
The supervisor and periodic forked checkpoint capture are implemented;
see `docs/checkpoints.md` for usage, fresh replay semantics and controls.
Read `docs/M4-RUNNING.md` for the running extended passes, endgame fixtures,
Rust NULL-note correction and C bugs 52-53 (tracker #287 and #290). Neither C nor D is
a passed checkpoint; main and tags have not moved.

For the next session working on borg-rs. Read this first, then
`CLAUDE.md`, `docs/REVIEW.md` and `docs/M4-PLAN.md`.

## What this project is

A faithful port to Rust of the Angband Borg, the game's built-in automatic
player (59 C modules, about 64k lines in `angband/src/borg/`, Angband
`e473720ed`, vendored as a git subtree and locally corrected only under `docs/FORK-CORRECTIONS.md`). The Rust Borg
must produce the same keypress stream as the C Borg, byte for byte, when it
drives the same C engine with the same seed. It is not a redesign: no new
tactics. Targeted crash/play-blocker fixes are now authorized. C bugs are recorded and listed in
`docs/c-bugs.md` (55 sections); from section 42 on each is also filed as a
lead in diegoami/neo-angband-tracker (`CLAUDE.md` has the rule and format).

## Where it stands

| milestone | state |
|-----------|-------|
| M1 deterministic C harness | done, tag `m1-harness` |
| M2 c2rust transpile behind the seam | done, tag `m2-c2rust` |
| M3 safe, idiomatic Rust | done, tag `m3` = `74fc8793c`, after three external reviews |
| M4 wider parity evidence | in progress: steps A (messages/state log) and B (options suite) done; C-F open (`docs/M4-PLAN.md`) |

- All 59 modules are safe Rust in `borg-rs/src/<module>.rs`. `unsafe` only
  in `engine.rs` (the FFI boundary, 240) and the engine's C declarations in
  `headers/` (28): `borg-rs/tools/count-unsafe.sh`. `lib.rs` denies unsafe
  elsewhere; every module is `pub(crate)` (no public safe facade; the only
  public item is `regress::note_alias_report`).
- Equivalence: on all 105 runs of the six suites (smoke 3, full 45, deep
  9, classes 16, config 7, options 25) the Rust build is byte-identical to
  the C references in its key stream, its message log (game messages and
  the Borg's notes) and a per-key player-state digest. Configurations: the
  stock borg.txt, munchkin and lunal mode, and 16 more options (money scum,
  dynamic calcs, worships, risky, stop limits, respawn...). No run goes
  deeper than dungeon level 27. Outside that (endgame, self-lunal, the
  Borg's internal state, the map/monsters) the evidence is reading and
  conversion-time cross-checks that were not committed: M4 steps C-F.
- Natural nested Borg entries happen in 11 runs (config lunal, several
  options runs); all identical to C.
- Reviews: three external reviews, each with a record and a response in
  `docs/` (`REVIEW-*.md`, `REVIEW-RESPONSE-*.md`; the first review's
  report is on branch `review/m3-4c06cec08`). The owner asked to keep them
  until they explicitly ask to remove them.
- Branches: work on `claude/serene-ramanujan-8hzsfm`; `main` is moved only
  to commits that passed a checkpoint (all suites identical). Local only:
  `xcheck-wip`, `xcheck2`, `xcheck3` (the borg_fight_attack conversion's
  cross-check code; a starting point for M4 step E).

## How to build and check

```
harness/build.sh                    # C Borg   -> build/c   (+ BUILD_ID)
BORG_IMPL=rust harness/build.sh     # Rust Borg -> build/rust (+ BUILD_ID)
BORG_IMPL=rust JOBS=2 harness/suite.sh full     # smoke full deep classes config options
harness/diff --suite full --expect-impl rust harness/out/rust/full harness/reference/full
harness/check-no-c-borg.sh
harness/regress/note-alias/run.sh   # C vs Rust note aliasing
harness/regress/nested/run.sh       # forced nested Borg entries
(cd borg-rs && cargo test --release && cargo clippy --release --all-targets -- -D warnings)
```

Suite wall times with `JOBS=2` on the 4-core container: smoke ~1 min,
full ~10 min, deep ~25-40 min, classes ~10 min, config ~4 min, options ~4 min.
The last line of `harness/diff` output is a note about the logs; the verdict
is the `N/M runs identical` line and the exit code (0 = identical). Details:
`docs/harness.md` (build identity, roles, `--suite` scenario checks, config
flags). Rust 1.97.0 is installed; 1.98.1 was used to check clippy.

## Rules that are easy to break

- Never make a diff pass by editing references, seeds or suites, or by
  special-casing anything; only make authorized, documented corrections. Targeted crash/play-blocker
  corrections are authorized under `docs/FORK-CORRECTIONS.md`; match C
  and Rust corrections and validate against corrected C twice.
- Mark authorized C/Rust crash corrections `FORK` and link tracker issues.
  Unrelated harness hooks remain under `#ifdef BORG_HARNESS` in engine/frontend files.
- New references are made with the C build, run twice, identical.
- C bug leads go to diegoami/neo-angband-tracker after the module is merged
  and pushed (`CLAUDE.md`). Never post anything to angband/angband.
- Tag pushes from the session are refused by the git proxy (HTTP 403): the
  owner creates tags (`git tag -a ... && git push origin <tag>`).
- Kill only exact PIDs you started (never `pkill`/`killall`): parallel
  agents run suites on the same machine.
- The container restarts without warning (it happened three times): commit
  WIP often, keep background steps under ~25 minutes, and rerun interrupted
  checks from scratch (never trust partial outputs; `suite.sh` writes a
  `DONE` marker last).

## How the work was organised

- Conversions and fixes were done by background agents in git worktrees
  (`.claude/worktrees/`, cleaned up after merging), three at a time, each
  with a temporary cross-check against the old code plus fuzzing, then
  cherry-picked by the lead, renumbered (PROGRESS rows, c-bugs sections),
  built, smoke-tested and pushed; a full/deep/classes checkpoint every few
  merges before moving `main`. The shared agent brief is described in
  `docs/PROGRESS.md` (Conventions) and was kept in the session scratchpad.
- Agents share one scratchpad: give each its own subdirectory.

## Open items

- M4 steps C-F (`docs/M4-PLAN.md`): C more seeds and longer runs, D
  endgame savefiles (the biggest remaining gap), E a committed native-C
  differential checker (start from local branches `xcheck-wip`, `xcheck2`,
  `xcheck3`), F coverage measurement. Options still not covered by a run
  are listed in `docs/harness.md` ("The options suite").
- Known latent items, documented: a few panics where C reads out of bounds
  on inputs no suite reaches (e.g. tracker #198); the engine handles and
  table copies are sound inside the crate but rely on all Borg code running
  inside `engine::enter` (the crate's own invariant).
- `docs/STATUS.md` has grown long; a short current summary at the top
  would help.

## Savefiles

Harness runs save into `harness/out/<impl>/<suite>/<run>/save/`
(throwaway). A normal in-place build uses `build/<impl>/game/lib/user/save/`;
`-dsave=<dir>` overrides. M4 step D will commit endgame saves, probably
under `harness/saves/`.
