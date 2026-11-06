# Native calculator checker controls

The initial M4 E checker in `harness/native-checker` passes30,000 generated
cases with identical C/Rust results, diagnostics and complete retained
calculation/token state, seed1511464998. Batches of47 exercise growth and
clear/reuse; at most four bounded operands avoid int32 overflow, with
additional parenthesis depths. The actual C source is renamed and linked
beside the unchanged Rust module; the explicit value/diagnostic environment
is shared. It is not a full private Borg game-state checker.

The negative control compiles an actual temporary copy of the Rust module
whose addition is replaced with subtraction. It must exit1 with a mismatch;
source overrides can never produce valid PASS evidence. The C UBSan run
also passes30,000 cases, with non-recovering checks; no C undefined behavior
is observed in this bounded corpus. Clippy with warnings denied passes.
Logs, source hashes and the completed runner marker are retained here.

Reproduce: `CHECK_UBSAN=1 harness/native-checker/run.sh`.
The three native C allocator code-reading leads (#323–#325) remain reported
leads; no leak-sanitizer or gameplay-memory-exhaustion claim is made.
