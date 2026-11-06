# Bounded Rust coverage pilot, source26a96be

This pilot merges only the completed smoke/full groups: 48 successful
processes, each bound to a nonempty profile and strict key/message/state
parity. Borg logic covers21,251/34,610 lines (61.40%) and1,063/1,380 LLVM
function instances (77.03%). Project-wide Rust totals include generated,
FFI and regression modules and are reported separately in `REPORT.md`.
Functions include monomorphizations/closures; branch coverage is not measured.

Full M4 F is **not established**. Collection was interrupted in deep when
the production extended run exposed the same-square melee play blocker.
That incomplete group is excluded, including any individually completed
profiles. These numbers do not describe the later melee correction or the
whole corpus. The pilot image and its input identity are source26a96be.

`instrumentation.json` records RUSTFLAGS, actual linker flags, the matching
LLVM component URL/SHA-256, and profiling-runtime/exit-shim hashes. The
isolated C-linked Rust staticlib needs its profiling runtime explicitly;
`borgh_finish` uses `_exit`, so a coverage-only link shim flushes profiles
on successful termination. A write error becomes exit4. Ordinary builds
never link the shim. The smoke/full oracles verify observation equivalence
for this measured subset, not arbitrary states.

Retained: image BUILD_ID, per-suite manifest/DONE/comparison/profile ledger,
merged LLVM profdata, compressed raw export, normalized per-file summaries
and function-entry counts/gaps. Raw profiles and the instrumented image
remain in `/tmp/borg-rs-m4-coverage/harness/out/coverage/` and its build tree.
Reproduction/tooling: `harness/coverage/README.md`, `run.sh`, `report.py`.
