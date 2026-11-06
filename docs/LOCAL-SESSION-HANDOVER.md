# Local-session handover — 2026-11-06

Continue on `claude/serene-ramanujan-8hzsfm`. M4 is unfinished. Main and
tags are unchanged; the owner creates tags. Read this file, `CLAUDE.md`,
`FORK-CORRECTIONS.md`, `M4-PLAN.md`, `REVIEW.md`, `harness.md` and
`checkpoints.md`. Give frequent concrete progress updates.

## Owner policy

Crash-causing C behavior and blatant bugs that prevent play may be
corrected now. Mirror the narrow correction in C/Rust where applicable,
mark it `FORK` and link the tracker. Other conflicts are case by case.
Do not preserve crashes, undefined bytes or known failures as milestone
requirements. Broader divergence follows a verified milestone and an
owner-created tag. Original references remain separate; corrected-C
references require two identical C runs. File C findings/updates in
`diegoami/neo-angband-tracker`, never `angband/angband`.

## Current code and completed validation

`bd184cc48035` corrected stale panel-offset equality (#290) and screen-text
termination (#287). Its six-case C/C/Rust matrix passed, but the next full
baseline check exposed UTF-8 truncation on seeds1001/1003: both builds
matched each other 45/45 but only 43/45 original references, with phantom
Sméagol names and changed keys/state on seed1003.

`26a96bee4570` refines #287: C takes actual caller byte capacity (all five
call sites supply `sizeof` their output buffer); Rust uses its slice
length. Both reserve NUL, preserve full UTF-8 when it fits and return
empty text/error on invalid encoding. The real-C-function full-message
regression fails before and passes after; zero/one-byte capacity,
multibyte-boundary and invalid-encoding controls pass.

Fresh builds at `26a96bee4570`, with clean build inputs, pass:

* Unchanged six-case `crash-fixes`: corrected C twice, then Rust; C/C and
  Rust/C each **6/6 identical** in keys/messages/player state. All three
  have `DONE`, no overrides, unchanged executable hashes and 18 successful
  process receipts. Rogue4006 finishes 2,000,352 turns, clevel21, depth6;
  all five original endgame cases finish, depth50 reaching 69.
* Both C and Rust vs original baseline references: smoke3, full45, deep9,
  classes16, config7, options25: **105/105 each**, all three logs compared.
* Both builds vs original `captured-rogue`: **8/8 each**. These fresh-Borg
  replays do not restore private Borg state or form a continuous trace.
* Release Rust tests: 100 passed, one existing ignored; release all-target
  clippy with warnings denied; linkage check: 154 engine objects identical,
  actual link inputs prove no C Borg code in Rust. Seven supervisor tests
  passed before the capacity refinement; no supervisor code changed.

Receipts, manifests, summaries, log hashes, process outcomes and strict
comparisons: `docs/validation/2026-11-06-capacity/`. Current corrected C
reference: `harness/reference/crash-fixes`, rechecked against C2 after
compression. Original references (including `endgame`) are unchanged.
Initial `bd184cc` results and strict baseline failures remain separately
identified at `docs/validation/2026-11-06-corrected/` and
`harness/reference/crash-fixes-bd184cc`; they are historical evidence,
not acceptance of the latest sources.

The prior session described an unchanged-C crash observer at the detection
next-row write (`q_y=1`, `q_x=0`, NULL next-row pointer). Its raw trace was
not transferred. Current receipts do not claim a newly captured native
trace, tracker-independent verification or upstream/source resolution.

## Active extended matrix

The unchanged `extended` suite has nine actual classes on seeds4000–4008
at 2M turns and seeds2000/2007/3100 at 5M turns: 12 cases, 33M turns per
pass. Corrected C1, C2 and Rust are running concurrently, each with four
jobs and `RUN_TIMEOUT_SECONDS=1800` per scenario. Build inputs stay frozen.

Local output targets and logs:

* `harness/out/local/extended-c1`, `extended-c1.log`
* `harness/out/local/extended-c2`, `extended-c2.log`
* `harness/out/local/extended-rust`, `extended-rust.log`

Recheck live process receipts before starting another pass; do not duplicate
active jobs. A `.partial.*` directory is not a completed suite. Inspect
`process.json` and signal only exact PIDs started for this work. Require
all three `DONE` markers, then run separately and inspect each exit code:

```sh
harness/diff --suite extended --expect-impl c harness/out/local/extended-c1 harness/out/local/extended-c2
harness/diff --suite extended --expect-impl rust harness/out/local/extended-rust harness/out/local/extended-c1
```

No extended reference has been accepted. Promote a separately identified
corrected-C reference only after C/C passes, then recheck it against C2.
Record actual reached depths and stop reasons. Investigate any new failure
or mismatch; do not exempt it automatically or change seeds/limits/oracles.

## Remaining M4 work

1. Finish the extended matrix above (step C).
2. Step D's declared five synthetic Warrior starting points now have
   strict parity. Logs establish deep play and Morgoth melee, alongside
   "Not ready for Morgoth now" and leaving; they do not prove Sea of Runes,
   self-lunal, a win, other equipment kits or other endgame classes.
3. Step E: implement/commit a native C/Rust differential checker. Historical
   `xcheck-wip`, `xcheck2`, `xcheck3` branches are absent here; build anew
   unless another local repository supplies them.
4. Step F: measure Rust coverage, report gaps and add targeted checks.
5. After further source changes, rebuild and run the appropriate final
   checkpoint, update documentation and tracker receipts, and present the
   verified milestone to the owner for tagging. No main/tag move is automatic.

Both correction tracker issues (#287/#290) still need published patch and
validation links from this continuation; do not claim that update until
it exists. Local binaries were built with GCC15.2.0 and rustc1.98.1.
Use the repository build scripts, without cloud `/tmp` dependency paths or
stale-build overrides. The complete local outputs remain under
`harness/out/local/`; durable validation evidence is committed in `docs/`.
