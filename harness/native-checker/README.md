# Native calculator differential checker (M4 E foundation)

Run from the repository root:

```sh
harness/native-checker/run.sh
CHECK_UBSAN=1 harness/native-checker/run.sh
```

The runner records logs and source SHA-256 hashes under a new
`harness/out/native-checker.*` directory and writes `DONE` only after all
requested controls pass. It needs a native C compiler, stable Rust and,
for `CHECK_UBSAN=1`, the C compiler's UBSan runtime.

The checker compiles the actual `borg-formulas-calc.c` under renamed
symbols beside the unchanged Rust `borg_formulas_calc.rs` module. C uses
the real `borg_array_add` implementation. The Rust module is imported
by path, not copied or translated into another implementation.

Both sides receive the same explicit calculator environment: `value(0..9)`
is `index * 7 - 10`, `value(99)` is the supplied range index. The C value
lookup and diagnostic callbacks are stubs; Rust uses the module's existing
`CalcEnv` boundary. The small `ValueSec`/`BorgState` records satisfy that
module's type dependencies, not a complete game-state model.

For every case the checker compares result/index, diagnostic order/text,
the entire retained calculation list, maximum depths and every token's
kind, negation, depth and payload. Batches of 47 calculations exercise
array growth and clear/reuse. Thirty thousand deterministic expressions
(seed1511464998) cover all operators, negative/negated operands, precedence,
redundant parentheses, range values and divide-by-zero diagnostics.
Expressions have at most four operands with absolute value at most53, so
intermediate signed arithmetic remains within int32. Arbitrary malformed
input and C undefined arithmetic are outside this corpus.

The negative control makes a temporary copy of the actual Rust source,
changes its addition operation to subtraction, compiles that source and
requires a mismatch. Source overrides are explicitly marked and can never
produce valid `PASS` evidence. Optional UBSan runs the native C side with
non-recovering undefined-behavior checks. The ordinary runner also checks
clippy with warnings denied.

This is a committed, reproducible native checker for one selected kernel.
It does not compare the entire Borg's private game state, combat scoring,
map or monster models, nor the production value-name resolver. It is an
M4 E foundation rather than a claim that all historical conversion checks
have been replaced. The original C allocator leads discovered during this
work are recorded in `docs/c-bugs.md` section55 and tracker #323–#325;
the checker does not change their C implementations.
