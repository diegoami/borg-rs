# Full M4 Rust coverage:139 executions

All11 declared suites complete and match corrected C on keys/messages/player
state, with successful process receipts and one nonempty profile per engine PID.
The six synthetic-tactics cases also match complete final engine-save bytes.

| scope | covered lines | covered LLVM function instances |
|---|---:|---:|
| All project Rust |30819/36184 (85.17%)|1543/1709 (90.29%)|
| Borg logic, excluding FFI/generated/regression modules |29550/34610 (85.38%)|1269/1380 (91.96%)|

LLVM function instances include closures/monomorphizations. Branch coverage is
unmeasured. Per-file metrics, the complete function list and raw LLVM export
are retained; nonzero entry alone does not prove a tactic completed.

The original ten-suite133 baseline measured29296/34610 logic lines (84.65%)
and1264/1380 function instances (91.59%). Its immutable report/export is under
original-133. The new six engine cases add155 self-lunal thinker entries and8
glyph-builder entries, with zero-entry controls; they do not change production
inputs or the instrumented image. The selected native checker coverage is
separate and never merged into these game-engine profiles.

The image was built from f987c077 (production input bytes identical to28b8170
and the final work branch), rustc1.98.1/GCC15.2.0 with LLVM22.1.8 tools.
Actual Cargo fingerprint flags are instrument-coverage/link-dead-code; the
explicit Rust profiler runtime and coverage-only _exit flush shim are hashed
in instrumentation.json. Every accepted run has captures disabled, no identity
overrides, unchanged executable hashes before/after and exit0. Profile write
failure is exit4. The ordinary GUI/harness never links this coverage shim.

COLLECTIONS.json records the11 completed groups selected from one frozen image.
The original coordinator suffered a shell-footer parse error after all ten
groups had passed, because its on-disk script was edited while running. It did
not get a full-matrix DONE and was not accepted as a complete collection. The
strict join independently verified every complete group, successful process,
recipe/source/binary hash and profile byte hash before creating the assembled
DONE. No partial/failed engine group is counted. Future collectors execute an
immutable startup script snapshot; the disposable live-edit smoke control is
retained. Independent duplicate extended/natural groups also passed but are not
merged twice. Native fault-control/build profiles are excluded.

profiles.tar.xz retains all139 raw profiles; coverage.profdata and coverage.json.xz
retain the merge/export. Per-run metadata and RUN_LOG_HASHES.json bind all three
full log streams; identical C reference bytes remain in harness/reference.
The local image/data are under /tmp/borg-rs-m4-coverage and matching tools under
/tmp/borg-llvm-tools. Reproduce via harness/coverage/README.md; preserve the exact
image to re-export these particular profiles. A fresh build can recollect the
same declared scenarios. GAPS.md states the finite coverage and tactic limits.
