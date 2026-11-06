# Corrected C/C/Rust validation, 2026-11-06

The unchanged six-case `crash-fixes` suite passes on clean source commit
`bd184cc4803554af120ce2250b5ad8c611a0c667`. C ran twice, followed by Rust;
both strict comparisons report **6/6 identical** in keys, messages and
per-key player state. Every pass has `DONE`, every process receipt records
success with exit 0, and all manifests record `overrides=none`. The
executable hashes remained unchanged during every run.

| scenario | final game turn | character level | maximum depth | keys |
|---|---:|---:|---:|---:|
| ext-rogue | 2,000,001 | 19 | 14 | 325,750 |
| end-depth50 | 100,096 | 50 | 76 | 57,796 |
| end-depth98 | 100,021 | 50 | 98 | 84,800 |
| end-depth99 | 100,001 | 50 | 99 | 102,937 |
| end-depth100 | 50,002 | 50 | 100 | 60,665 |
| end-near-morgoth | 20,001 | 50 | 100 | 15,553 |

Both builds were freshly made with GCC 15.2.0; Rust used rustc 1.98.1.
The source hashes equal the earlier dirty-source preflight hashes, now
bound to the published clean commit. `c-BUILD_ID` and `rust-BUILD_ID`
record exact source and binary hashes. C1/C2/Rust wall times were
155.0/147.5/150.0 seconds, with two jobs per suite. Each scenario had a
1,800-second wall-time limit; none timed out.

The new reference is `harness/reference/crash-fixes-bd184cc`, compressed from C1
only after C/C passed and rechecked against C2 after compression.
The original `harness/reference/endgame` is unchanged. No seed, fixture,
turn limit, configuration or oracle was weakened. `receipt.json` records
SHA-256 hashes of the three full logs from all 18 runs. The compressed C1
logs are retained with the reference; manifests, summaries, configuration,
supervisor receipts and stderr for all three passes are retained here.
The complete uncompressed local outputs are in `harness/out/local/`.

Reproduction, after building both implementations from the correction
commit (or identical build inputs):

```sh
BORG_IMPL=c JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/c1
BORG_IMPL=c JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/c2
harness/diff --suite crash-fixes --expect-impl c harness/out/reproduce/c1 harness/out/reproduce/c2
BORG_IMPL=rust JOBS=2 RUN_TIMEOUT_SECONDS=1800 harness/suite.sh crash-fixes harness/out/reproduce/rust
harness/diff --suite crash-fixes --expect-impl rust harness/out/reproduce/rust harness/out/reproduce/c1
harness/diff --suite crash-fixes --expect-impl c harness/out/reproduce/c2 harness/reference/crash-fixes-bd184cc
```

Inspect each exit status before proceeding. The saved strict diff logs
here are `crash-c-c.diff`, `crash-rust-c.diff` and `crash-reference.diff`.
The real C screen-text regression passes all four controls. Release Rust
tests pass (100 passed, one existing ignored), clippy with warnings denied
passes, and seven supervisor tests pass. `no-c-borg.log` confirms all 154
engine objects are identical and the Rust executable contains no C Borg
code, including the actual link-input check.

These runs validate the two matching fork corrections, not all of M4.
The endgame fixtures are synthetic level-50 Human Warriors with one
equipment kit. The near-Morgoth log contains attacks received, misses
and melee hits, plus "Not ready for Morgoth now" and leaving. A deep
starting level alone does not prove Sea of Runes, self-lunal or a win.
The full `extended` suite, a committed native differential checker and
Rust coverage measurement remain separate work. The prior session's
native crash observer is described in the handover, but its raw trace
was not transferred; this receipt does not claim a newly captured trace
or independent tracker/upstream verification.

## Subsequent baseline regression

The first 105-case baseline attempt stopped at `full`: corrected C and
Rust each match only 43/45 original cases, while matching each other
45/45. Seeds1001/1003 parse truncated UTF-8 text into phantom monster
names; seed1003 also changes keys and state. These six-case receipts
therefore do not establish a passed baseline. The regression and its
capacity-aware refinement are recorded separately; this reference keeps
its exact `bd184cc` source identity and is not the latest acceptance
reference. Original baseline references remain unchanged.
