# Screen-text capacity correction and current validation

At `26a96bee4570`, the unchanged six-case `crash-fixes` suite passes
corrected C twice and Rust once: **6/6 identical** in keys, messages and
per-key player state for both strict comparisons. Every suite has `DONE`,
all 18 processes exit 0, hashes remain unchanged during execution and no
override is used. Build inputs match the commit (`inputs_dirty=no`);
uncommitted documentation/reference evidence makes `git_dirty=yes`.

| scenario | final game turn | character level | maximum depth | keys |
|---|---:|---:|---:|---:|
| ext-rogue | 2,000,352 | 21 | 6 | 308,698 |
| end-depth50 | 100,002 | 50 | 69 | 75,309 |
| end-depth98 | 100,003 | 50 | 98 | 107,789 |
| end-depth99 | 100,079 | 50 | 99 | 103,602 |
| end-depth100 | 50,043 | 50 | 100 | 52,257 |
| end-near-morgoth | 20,001 | 50 | 100 | 15,553 |

The build IDs record GCC 15.2.0, rustc 1.98.1 and complete source/binary
hashes. C1/C2/Rust wall times were 136.0/135.0/137.9 seconds with two
jobs per suite and a 1,800-second per-scenario timeout. None timed out.
`receipt.json` records all 18 summaries and three-log SHA-256 hashes;
the per-pass directories retain manifests, `DONE`, timing, configuration,
process receipts and stderr. The separately identified current C reference
is `harness/reference/crash-fixes-26a96be`, compressed from C1 after C/C passed
and strictly rechecked against C2 (`reference.diff`). All original
references, including `endgame`, are unchanged.

## Why the transferred correction needed refinement

The initial `bd184cc` fix passed its six targeted cases, but both builds
matched only 43/45 original `full` cases, while matching each other 45/45.
Seeds1001/1003 exposed a remaining confusion between screen columns and
UTF-8 bytes: reserving NUL but limiting output to `n` bytes truncated
messages containing `Sméagol`. The Borg parsed phantom names; seed1003
also changed keys and state. Those strict failures and the initial
six-case results remain in `../2026-11-06-corrected/`, with their own
`harness/reference/crash-fixes-bd184cc` reference. They are historical
evidence, not acceptance of the current source.

The C function now takes the caller's actual byte capacity, supplied as
`sizeof` each output buffer. Rust uses the output slice length. Both
reserve NUL, preserve complete UTF-8 text when it fits and return empty
text/error on invalid encoding. `n` still selects screen columns. No
parser, tactic, seed, fixture, turn limit or oracle filtering changed.

The added real-C-function test fails before the refinement and passes
after it. `screen-text-before.c` preserves the old-signature failing probe;
compile it against the `bd184cc` C source/header tree to reproduce.
The current probe in `harness/regress/screen-text/` also checks ASCII,
full UTF-8 expansion, no partial multibyte character at a byte boundary,
zero/one-byte capacities and invalid encoding. Saved logs are
`screen-text-capacity-before.log` and `screen-text-capacity-after.log`.
The two affected C scenarios then match their original references 2/2
in all three logs (`utf8-c-capacity.diff`, `utf8-recheck-suite`).

## Reproduce the six-case matrix

Build both implementations with `harness/build.sh` and
`BORG_IMPL=rust harness/build.sh`, then run these separately, checking each
exit status before continuing:

```sh
harness/regress/screen-text/run.sh
harness/check-no-c-borg.sh
BORG_IMPL=c JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/c1
BORG_IMPL=c JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/c2
harness/diff --suite crash-fixes --expect-impl c harness/out/reproduce/c1 harness/out/reproduce/c2
BORG_IMPL=rust JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/rust
harness/diff --suite crash-fixes --expect-impl rust harness/out/reproduce/rust harness/out/reproduce/c1
harness/diff --suite crash-fixes --expect-impl c harness/out/reproduce/c2 harness/reference/crash-fixes-26a96be
```

Release Rust tests pass (100 passed, one existing ignored), and release
all-target clippy with warnings denied passes. The linkage check confirms
154 identical engine objects and no C Borg code in the Rust executable,
including the actual link-input check. Logs are retained here.

This is not M4 completion. The synthetic Warrior fixtures cover deep play
and Morgoth melee, not a win, Sea of Runes or self-lunal. Both implementations pass all 105 original baseline cases (smoke 3,
full 45, deep 9, classes 16, config 7, options 25) and all eight captured
Rogue replays, comparing all three logs. `baseline/` retains each suite
manifest, `DONE`, timing and strict comparison; `baseline/receipt.json`
records per-run summaries, log hashes, successful process status and nested
entry counts. Rust reports 12 natural nested entries across
config/options, while preserving parity. C has no corresponding entry
diagnostic; its zero diagnostic count does not establish zero nesting. The extended matrix, native differential checker and
coverage measurement need their own receipts. The prior session described a native detection-crash
observer, but its raw trace was not transferred. This session does not
claim to recapture that trace or independently verify either tracker
finding upstream.
