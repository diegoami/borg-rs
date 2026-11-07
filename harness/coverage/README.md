# Rust coverage tooling

Use an isolated checkout so instrumentation never replaces a binary used
by an active ordinary suite. The first measurement is a **pilot** at
`26a96bee4570`: smoke/full only, 48 successful oracle-checked runs. Full
M4 F coverage remains open; the later melee correction is not part of that
pilot image.

## Build an isolated coverage image

Use the same source revision as the scenario references. The LLVM tools
must match rustc's LLVM version (`rustup component add llvm-tools-preview`,
or the exact component URL/hash in the installed Rust channel manifest).
The 1.98.1 session used LLVM22.1.8 and verified component SHA-256
`56d58ca2dfa848359ab23c872791509c6f592448d2f6c146ed530ec50ee45371`.

Example on Linux; run each command separately and inspect its exit status:

```sh
git worktree add --detach /tmp/borg-coverage SOURCE_REF
mkdir -p /tmp/borg-coverage/harness/out/coverage
cc -O2 -c harness/coverage/exit-flush.c -o /tmp/borg-coverage/harness/out/coverage/exit-flush.o
```

Locate `libprofiler_builtins-*.rlib` under
`$(rustc --print sysroot)/lib/rustlib/<host>/lib/`. Configure the isolated
Rust build with these CMake options (substitute its exact archive path):

```sh
cmake -S /tmp/borg-coverage/angband -B /tmp/borg-coverage/build/rust \
  -DCMAKE_BUILD_TYPE=Release -DSUPPORT_BORG_HARNESS=ON \
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DBORG_IMPL=rust \
  '-DCMAKE_EXE_LINKER_FLAGS=-Wl,-u,__llvm_profile_runtime -Wl,--wrap=_exit /tmp/borg-coverage/harness/out/coverage/exit-flush.o /EXACT/PATH/libprofiler_builtins-HASH.rlib'
```

Then, **inside the isolated checkout**, run:

```sh
BORG_IMPL=rust RUSTFLAGS='-C instrument-coverage -C link-dead-code' harness/build.sh
```

Rust's staticlib needs an explicit profiling runtime when linked by the C
engine. `borgh_finish` uses `_exit`, bypassing atexit: the coverage-only
`exit-flush.c` shim writes the profile just before a successful `_exit`.
A profile write error becomes exit4, so a failed measurement cannot pass.
The ordinary harness never links this shim. Do not enable checkpoint
capture while measuring with the shim; a forked save child is not an
independent measurement run.

## Collect and report

Run the outer checkout's tooling against the instrumented checkout and
accepted references. Use a fresh output path:

```sh
harness/coverage/run.sh /tmp/borg-coverage /ABS/REPO/harness/reference /tmp/borg-coverage/harness/out/coverage/matrix
python3 harness/coverage/report.py /tmp/borg-coverage/harness/out/coverage/matrix NEW_REPORT --llvm-bin /PATH/TO/MATCHING/LLVM/BIN
```

Collection uses the six baseline suites, captured-Rogue, crash-fixes,
extended, natural and synthetic-tactics:139 suite executions. The five endgame scenarios are
represented in crash-fixes; the Rogue crash case also occurs in extended.
Natural uses its explicit captured-extended save root; synthetic-tactics uses
its generated fixture root. Both are passed explicitly for collection and diff.
Every suite must have `DONE`, strict key/message/state parity and exactly
one nonempty profile tied to each successful engine PID. The full matrix
gets its own `DONE` only after all groups pass. No stale-build override is
used; preserve `BUILD_ID`, actual RUSTFLAGS, linker flags and shim/runtime
hashes in the validation receipt.

Independent groups can use `COVERAGE_SUITES='extended natural'` with a fresh
output directory. Selected collection writes `GROUPS_DONE`; only the full
default list writes matrix `DONE`. The coordinator snapshots its script at
startup so live source edits cannot corrupt its final completion step. To assemble independent collections:

```sh
python3 harness/coverage/join.py NEW_MATRIX COLLECTION_ONE COLLECTION_TWO
```

Joining requires exactly one complete accepted copy of every group and an
identical BUILD_ID/image across collections. Partial groups are excluded.
Both joining and reporting verify the expected recipe/run names, manifests,
binary/source hashes, no overrides/captures, successful process outcomes,
profile-to-PID identity and profile hashes. They reject reused profiles.
Joined groups are symlinks to retained collections, recorded in COLLECTIONS.json;
keep those collections while reporting or exporting the result.

The current measurement retains `link-dead-code` for one consistent image.
A separate instrument-coverage-only build retained the same36,184 mapped
project lines and1,709 LLVM function instances, passed smoke3/3, and matched
all three C oracles in four serial A/B benchmark runs. That flag is not
required for future collections; do not mix their profiles with this image.

For an explicitly bounded pilot, reporting accepts `--suites smoke full`.
It labels the result `pilot_subset`, requires those complete validated
groups and excludes interrupted groups. It never promotes partial runs.
Reports include compressed raw LLVM export, per-file metrics and function
counts/gaps. Borg-logic totals exclude FFI/generated/regression modules.
LLVM function instances include monomorphizations/closures; branch coverage
is not measured. An entered function alone does not prove a tactic executed.

The final source28 measurement is in docs/validation/2026-11-07-coverage:
139 engine executions, logic85.38% lines/91.96% LLVM function instances.
The original133 baseline remains under original-133. Synthetic cases add actual
lunal155/glyph8 entries with zero-entry controls; native-body checker profiles
are never mixed into this result. When reusing the historical frozen image,
only new harness scripts/suite files/fixture data were copied into that checkout;
production source hash and actual image hash stayed constant. A fresh checkout
at the completed work-branch revision already includes those files.
