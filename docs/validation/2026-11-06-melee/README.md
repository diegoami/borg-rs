# Latest corrected checkpoint: production source28b8170

Matching C/Rust narrow corrections cover panel-offset equality (#290),
UTF-8 byte capacity/termination (#287), and same-square thrust targets (#322).
No new tactic, altered scenario/seed/limit, weakened oracle or identity
exception is used. Main and tags are unchanged; M4 is not complete.

## Completed checks

| scope | C/C | Rust/C | evidence |
|---|---:|---:|---|
| crash-fixes, six unchanged original scenarios |6/6|6/6|`crash-*`, `six-case-receipt.json`, strict diffs|
| original baseline, both builds vs stored references |105/105 C|105/105 Rust|`baseline/`|
| original captured-Rogue replay references |8/8 C|8/8 Rust|`baseline/`|
| extended, original12 scenarios and limits |12/12|12/12|`extended/`|
| natural fresh-Borg suite |2/2|2/2|`natural/`|

All comparisons include keys, messages and per-key player state. Every
suite has DONE, no overrides and successful process receipts; binary
hashes remain unchanged throughout each run. Build inputs match the
published source; documentation/evidence account for dirty checkout flags.
Build IDs retain exact source/binary hashes and toolchain versions
(GCC15.2.0, rustc1.98.1). The linkage check confirms154 identical engine
objects and actual link inputs containing no C Borg code in Rust. Release
tests:100 passed, one existing ignored. Clippy all-targets with warnings
denied passes. Native regressions pass.

## The newly observed play blocker

At the prior source26a96be, two C runs and Rust agree on12/12 extended
outputs, but Warrior2000 stops at1,887,392 turns. After teleportation and
blindness, the Borg retains a monster on the player's coordinates. Thrust
accepts distance zero, queues `+5`, and hits the unexpected-direction abort.
The historical receipt/reference preserves this failure explicitly.

The native test compiles the actual thrust body unchanged into controlled
scoring/engine stubs. Before: center target scores10 and emits `+5`;
after: center/distant targets rejected, all eight real neighbors retained
in apply and simulation modes. The narrow production correction in both
languages requires distance exactly one; the prompt-abort safeguard remains.
The live C2 prefix control (`prefix-control.json`) matches456,025 keys before
the invalid direction5 changes to4 at the same turn. It is an earlier
partial-prefix observation, not a substituted full replay.

The complete corrected Warrior2000 scenario now reaches **5,000,001 turns,
clevel42, maxdepth44, 1,511,970 keys** in both C passes and Rust. Every
extended scenario reaches its original limit. Warrior2007 reaches clevel50,
maximum depth99; Dunadan Blackguard3100 reaches clevel43, maximum depth51.
The completed strict comparisons, all summaries and full-log hashes are
retained under `extended/`. Current compressed C references (`extended`,
`crash-fixes`, `natural`) were made only after their C/C comparisons passed
and rechecked against C2 after compression. Earlier source26 references
remain separately identified; original baseline/endgame references are intact.

## Engine captures and independent natural replays

All30 completed fixed-cadence captures have identical metadata, sizes,
SHA-256 and direct saved bytes across C1/C2/Rust. `harness/saves/captured-extended`
retains C1 copies, SHA256SUMS and SOURCES.json with all source manifests.
Capturing forks a private engine copy; the ordinary saver's compaction does
not mutate the live trace. Save-byte equality covers serialized engine
state, not every private/runtime field or the Borg's own memory.

The deepest actual saved current depth is78; source maximum99 is a
different fact. These are ordinary Borg-played characters with cheat death,
not wizard kits or an unassisted-play claim. Two separate fresh-Borg replay
helper matrices are MATCHED, and the named `natural` suite reproduces them
with corrected C twice and Rust once. The Warrior at78 reaches84 over100k
additional turns; the Dunadan Blackguard at40 reaches41. They restore engine
state and reseed before starting fresh Borgs; traces are never stitched.

Reproduce the natural suite with the explicit fixture root:

```sh
SAVE_ROOT=harness/saves/captured-extended BORG_IMPL=c JOBS=2 harness/suite.sh natural harness/out/reproduce/natural-c1
SAVE_ROOT=harness/saves/captured-extended BORG_IMPL=c JOBS=2 harness/suite.sh natural harness/out/reproduce/natural-c2
harness/diff --suite natural --save-root harness/saves/captured-extended --expect-impl c harness/out/reproduce/natural-c1 harness/out/reproduce/natural-c2
SAVE_ROOT=harness/saves/captured-extended BORG_IMPL=rust JOBS=2 harness/suite.sh natural harness/out/reproduce/natural-rust
harness/diff --suite natural --save-root harness/saves/captured-extended --expect-impl rust harness/out/reproduce/natural-rust harness/reference/natural
```

Use separate commands and inspect each exit status. Other suite commands
are documented in `LOCAL-SESSION-HANDOVER.md` and `harness.md`.

## Remaining M4 scope

Native E foundation:30,000 real C/Rust calculator cases compare results,
diagnostics and logical token/calculation state; a temporary actual-source
addition-to-subtraction fault is detected. C UBSan30,000 and clippy pass.
This does not replace all private combat/map/state cross-checks.

Coverage F pilot: smoke/full48 at source26a96be, logic lines61.40%, LLVM
function instances77.03%, with strict oracle controls and per-process
profile provenance. The interrupted deep group is excluded. Full coverage
of the latest corpus/source remains open. Deep levels and function entries
alone do not establish Sea of Runes, self-lunal, a win or a winning strategy.
Independent tracker verification and upstream/source resolution are not claimed.
