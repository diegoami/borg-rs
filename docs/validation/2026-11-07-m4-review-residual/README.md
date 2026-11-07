# M4 review: cost cutoff and portable oracle fixtures

The [independent follow-up](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707651371)
verifies findings2-5 and the original glyph mutations fixed at c2b52d9.
The verdict remains qualified pass with no acceptance blockers. These two
small author follow-ups address its new low cost75 residual and its note
that the oracle tests depended on post-M4 #295 fixtures. Independent
verification of these changes and owner acceptance remain separate.

## Flow-cost cutoff

The original23-scenario checker lets cost>=75 to cost>75 survive all2,300
executions (exit2, fault source unexpectedly matched). The new cases23/24/25
provide one candidate at cost74/75/76 respectively. Actual native C must
accept only74 (target x101); at75/76 it must return false and leave target0.
These expected outcomes are asserted independently of C/Rust equality.

The shared candidate-cost adapter supplies these values after real reverse
flow, extending the disclosed ranking adapter. This does not change #345
or establish that such costs are reached naturally in an engine game.

The full checker now executes26 hand-built scenarios ×100 packet/environment
variants =2,600 glyph executions. The cost-cutoff mutant is an eleventh
permanent real-source fault control. Glyph controls request the full2,600
corpus, failing at the first mismatch; other controls keep their bounded
subsets. The new mutant fails at case24 in both ordinary and nonrecovering
C UBSan builds. Equivalent v<=0 to v<0 is not treated as a missing witness:
v=0 is outside the reviewed domain, so no artificial zero-score case is added.

CHECK_UBSAN=1 passes at exact M4 production inputs in the implementation
agent's own isolated checkout: calculator30k, spatial2×2k, glyph2,600,
lunal1,280, eleven controls and all-target clippy. Source hashes remain
C3e9ac574/Rust8ac76b00. Logs and item/source ledgers are retained under native/.

## Portable oracle regressions

The five missing-log/role tests now use the original M4 synthetic-tactics
receipt and reference, replacing the post-M4 glyph-recognition paths.
All nine oracle tests pass both on main and on a tooling-only overlay at
tag m4. The overlay has neither post-M4 #295 fixture directory and retains
the exact M4 production hashes. These are regression tests on disposable
copies of committed streams, not new engine executions.

No production source, launcher, suite recipe, reference, committed save,
older receipt or tag is changed. The three reviewer worktrees remain
preserved. No game corpus, coverage measurement or GUI test is rerun for
these unrelated native-input/test-fixture changes.

## Recheck

```sh
python3 harness/regress/oracle/test_oracles.py
CHECK_UBSAN=1 harness/native-checker/run.sh
```

For exact-M4 reproduction, overlay only the changed native-checker,
oracle-test and supporting diff/coverage tooling on an isolated M4 checkout.
The tests require no #295 source, reference or receipt. The preceding review
receipt remains unchanged and describes the earlier23-scenario corpus.
