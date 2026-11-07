# Author follow-up to independent M4 review #1

The [independent review](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707067352)
of m4 /3923a21 is **qualified pass, no acceptance blockers**. The
[author disposition](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707181455)
agrees with all five findings. These are post-tag tooling/documentation
follow-ups; independent verification of the fixes is still pending. The
tag, existing receipts, recipes, references and production algorithms are
unchanged by this follow-up. Both reviewer worktrees are preserved.

## 1. Glyph checker domain and surviving mutants

The original corpus was16 hand-built scenarios ×100 packet/environment
variants, not1,600 independently generated layouts. Both independent
mutations (distance >=50 to >50, and tie < to <=) also survive the original
checker in the retained before controls (exit2, fault source unexpectedly
matched). The glyph kernel files are unchanged from m4.

Current corpus:23 hand-built scenarios ×100 variants =2,300 executions.
New cases16/17/18 exercise arrival distances49/50/51. At49 the center stays
51; at50/51 it resets to100. Cases19/20 check equal-score choice for both
ranking branches (later grid x101 wins); cases21/22 check unequal scores
(better earlier grid x99 wins).

Ranking cases19-22 explicitly provide two candidate costs after real reverse
flow. Inherited #345 otherwise leaves only the origin reachable, blocking
ties. This shared adapter tests the unchanged production ranking body under
a controlled environment, not a naturally reached game outcome or a fix to
#345. Other scenarios use real reverse flow without cost injection.

The full CHECK_UBSAN=1 checker was run in our own isolated checkout at the
exact reviewed M4 production inputs. C hash3e9ac574 and Rust hash8ac76b00
match the review; ledgers and logs are under native/. Calculator30k,
spatial2×2k, glyph2,300 and lunal1,280 pass, with nonrecovering C UBSan and
all-target clippy. All ten real-source fault controls are rejected. Newly
added mutants fail at case17 (distance),19 (both tie comparisons) and20
(old-center tie alone). Original native-body percentages are not relabelled
as measurements of these expanded cases; whole-Borg memory remains unshadowed.

## 2. Historical endgame oracle

docs/harness.md and checkpoints.md now explain the expected4/5 identical,
exit1 result against the original endgame reference after #287: only
depth98's message oracle differs; keys/state match all five. Corrected C/C
and Rust/C pass, and crash-fixes supplies the corrected checkpoint oracle.
No suite-file comment was changed because recipes are hashed in receipts.
No reference was rewritten. This documentation response relies on the
reviewer's independently reproduced five-case result, not a new game run.

## 3. Compressed missing logs

harness/diff now requires message/state logs in directory mode regardless
of compression, even when both operands omit them. Explicit
--allow-legacy-logs can only excuse a historical reference without modern
build identity, never a suite output. Skips remain labelled as NOTE and
cannot establish full three-log parity. Regression controls exercise all
these roles using disposable copies of committed real streams.

## 4. Coverage acceptance

Join/report validation now reruns strict diff against the selected C oracle,
records its tree fingerprint and the diff-tool hash, and retains fresh
results in ORACLES.json. Saved PASS text is not consulted. An explicit
--reference-root selects the oracle; new collections record REFERENCE_ROOT.

Nine oracle regression tests pass: complete compressed output, each missing
output stream, both operands missing streams, explicit legacy reference,
positive coverage validation with no saved PASS file, forged PASS with
corrupted messages or missing state, and a wrong selected reference. An
additional contradictory-marker check ensures FAILED wins over DONE.

The strengthened join re-diffs all11 frozen-image groups/139 executions
against the canonical M4 references: no skips and all process/profile checks
pass. Fresh comparison outputs and oracle fingerprints are under coverage-139/.
This is a fresh comparison of retained engine runs, not139 new executions.
A smoke-only report exercises the report path; its pilot summary is separate
from M4's full coverage figures. The independent reviewer already rederived
the full measurement; no new full-coverage percentage is claimed here.

## 5. GUI race

The maintained driver is harness/regress/launcher-save/run.py. The archived
M4 driver remains unchanged. Four deterministic controls distinguish an
already-exited process, BadWindow followed by exit0, BadWindow with a live
child, and a failed child. All pass.

Five complete GUI save/reload roundtrips (ten stages) pass on the exact M4
build in our own isolated checkout, using a private Xvfb display and fresh
fixture profiles. Every first save matches59cb56c6ee2c6f0f0a1ad116b25a59cc170aeab864c3bcd4d34c11b3101ab759;
every reload save matches3920bea0dd93d18d7c26f74d97c6e8cbbfc7d42a9a379e0d5eba12b06229bd9c,
both92944 bytes, exit0 and level50. Results and stage/build logs are under gui/.
The owner's display, playable build and save were not used.

Excluded attempts are retained: an initial Rust test-helper type annotation
error; native runner setup before adding fresh-output-parent creation; and
private Xvfb auto-selection failures because /tmp/.X11-unix is read-only in
the execution environment. The final driver uses a fixed private number and
readiness pipe, allowing its own abstract listener. It does not change shared
directory permissions or owner display sockets. These setup failures are not
accepted GUI runs.

## Recheck

```sh
python3 harness/regress/oracle/test_oracles.py
python3 harness/regress/launcher-save/test_race.py
CHECK_UBSAN=1 harness/native-checker/run.sh
python3 harness/coverage/join.py NEW_JOIN /tmp/borg-rs-m4-coverage/harness/out/coverage/complete-139 --reference-root harness/reference
python3 harness/regress/launcher-save/run.py NEW_GUI_OUTPUT --project-root ISOLATED_M4_CHECKOUT --runs 5
```

For exact-M4 native reproduction, copy only the updated harness/native-checker
directory into an isolated tag checkout, preserving production source. No
acceptance claim uses a stale-build override. Independent follow-up comments
and final finding closure belong on issue#1, not in an author-only conclusion.
