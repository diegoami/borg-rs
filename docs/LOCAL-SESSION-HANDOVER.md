# Local-session handover — 2026-11-06

Continue on `claude/serene-ramanujan-8hzsfm`. Production Borg inputs are
source `28b8170`; later commits add tooling, fixtures, receipts and docs.
All local validation jobs are finished. Main/tags are unchanged; the owner
creates tags. M4 remains open for wider E checks and full F measurement.

Read `CLAUDE.md`, `FORK-CORRECTIONS.md`, `M4-PLAN.md`, `REVIEW.md`,
`harness.md`, `checkpoints.md`, and the latest receipt:
`docs/validation/2026-11-06-melee/README.md`. Give frequent concrete updates.

## Authoritative owner policy

Crash-causing inherited C behavior and blatant play blockers may be fixed
now. Mirror narrow corrections in C/Rust, mark FORK, link the tracker and
validate. Other conflicts are case by case. Do not preserve crashes,
undefined bytes or known failures as milestone requirements. Broader
changes follow a verified milestone and owner-created tag. Preserve
original references; corrected-C references require two identical C runs.
File findings/updates in `diegoami/neo-angband-tracker`, never `angband/angband`.

## Corrections and verified outcome

* #290, `bd184cc`: reset stale offsets at equality with cave dimensions.
* #287, `26a96be`: use actual output byte capacity, reserve NUL and return
  empty text/error on invalid encoding. The transferred byte-limited fix
  had passed six cases but truncated UTF-8 in baseline seeds1001/1003;
  native failure/after controls and the restored baseline are retained.
* #322, `28b8170`: thrust requires distance exactly one. A stale monster on
  the player's square after teleport/blindness previously queued `+5` and
  stopped Warrior2000 at1,887,392. The real C body fixture fails before and
  passes after for center/distant rejection and all eight real neighbors,
  in simulation/apply modes. C2's first456,025 keys match the prior trace;
  the invalid direction5 becomes4 at the same turn. The abort guard stays.

At `28b8170`, no stale/unknown-implementation override is used:

* Fresh crash-fixes C/C and Rust/C: **6/6** keys/messages/player state.
* Both C and Rust vs original baseline references: **105/105 each**
  (smoke3, full45, deep9, classes16, config7, options25).
* Both builds vs original captured-Rogue references: **8/8 each**.
* Fresh extended C/C and Rust/C: **12/12**, all original turn limits
  reached. Warrior2000 finishes5,000,001 turns, clevel42/depth44;
  Warrior2007 reaches maxdepth99 and Blackguard3100 maxdepth51.
* All30 fork-isolated captures match metadata, SHA-256 and direct bytes
  across C1/C2/Rust; retained in `harness/saves/captured-extended`.
* Named natural suite C/C and Rust/C: **2/2**, 100k-turn fresh-Borg replays
  from Warrior78 and Blackguard40, reaching84/41. Separate helper replay
  matrices also have MATCHED. These are independent scenarios, not stitched
  continuations; engine saves do not contain private Borg memory.
* Release tests100 passed/one existing ignored; release all-target clippy
  with warnings denied; linkage check154 identical engine objects and
  actual link inputs proving no C Borg code in Rust. Native regressions pass.

Every accepted suite has DONE and successful process receipts. Captures
were every1M turns in private forked engine copies. Their deepest saved
current depth is78; source maxdepth99 is a separate fact. Characters are
Borg-played with cheat death, not unassisted-play or wizard-kit claims.

Current corrected C references: `crash-fixes`, `extended`, `natural`.
Historical source26 references: `crash-fixes-26a96be`, `extended-26a96be`;
initial byte-limited reference: `crash-fixes-bd184cc`. Original baseline and
endgame references retain their bytes. The source26 extended receipt
explicitly preserves Warrior2000's early stop, not a5M-turn success.

## E and F progress, with limits

`harness/native-checker/` compiles the real renamed C calculator beside
the unchanged Rust module in a shared explicit environment.30,000 cases
compare results, diagnostics and logical calculation/token contents.
Batches of47 exercise growth/reuse; bounded operands avoid signed overflow.
A temporary actual Rust addition-to-subtraction mutation is detected.
C UBSan30,000 and clippy pass. Reproduce:

```sh
CHECK_UBSAN=1 harness/native-checker/run.sh
```

Receipts: `docs/validation/2026-11-06-native-checker/`. This is an E
foundation for one kernel, not replacement of every historical private
combat/map/game-state cross-check. No local xcheck-wip/xcheck2/xcheck3
branches were found. Allocator source-reading leads #323–#325 are filed
separately; C is unchanged and no leak-sanitizer/game-exhaustion claim is made.

F pilot tooling is in `harness/coverage/`. An isolated source26a96be build
uses instrument-coverage/link-dead-code, matching LLVM22.1.8 and an explicit
profiling runtime plus coverage-only _exit flush shim. Smoke/full48 pass
their oracles and produce one profile per engine PID. Logic coverage:
**61.40% lines,77.03% LLVM function instances**; closures/monomorphizations
count separately, branch coverage is not measured. The interrupted deep
collection is excluded. The pilot does not cover the later melee change
or the whole corpus. Raw/local image: `/tmp/borg-rs-m4-coverage`; durable
export, merged profdata and ledgers: `docs/validation/2026-11-06-coverage-pilot/`.
Do not reuse partial groups or call this full M4 F.

## Next work

1. Extend the committed native checker to additional priority kernels and
   private combat/map state; use the reported gaps rather than claim all
   conversion-time checks are replaced.
2. Measure the latest complete corpus with the isolated coverage recipe.
   Natural is a new two-case suite with explicit SAVE_ROOT. The existing
   coverage runner covers the older nine groups (including five endgame
   cases via crash-fixes); add natural deliberately when extending F.
3. Add targeted coverage where measured gaps warrant it. Sea of Runes,
   self-lunal and a win are not established. Deep starts/function entries
   alone are not proof of completed tactics.
4. After further production-source changes, rebuild and checkpoint the
   affected corpus, update receipts, then present a verified milestone for
   the owner to tag. Do not automatically move main or create a tag.

Useful current reproduction commands (run separately; inspect exit codes):

```sh
harness/build.sh
BORG_IMPL=rust harness/build.sh
harness/regress/screen-text/run.sh
harness/regress/melee-adjacent/run.sh
harness/check-no-c-borg.sh
BORG_IMPL=rust JOBS=4 harness/suite.sh extended harness/out/reproduce/extended
harness/diff --suite extended --expect-impl rust harness/out/reproduce/extended harness/reference/extended
SAVE_ROOT=harness/saves/captured-extended BORG_IMPL=rust JOBS=2 harness/suite.sh natural harness/out/reproduce/natural
harness/diff --suite natural --save-root harness/saves/captured-extended --expect-impl rust harness/out/reproduce/natural harness/reference/natural
```

Local source28 outputs remain under `harness/out/local/melee-extended-*`,
`baseline-melee-*`, `melee-crash-c2`, `natural-*`. No jobs should be resumed
from their completed/partial predecessors. Read DONE/strict comparisons.
Current builds used GCC15.2.0 and rustc1.98.1. No cloud dependency paths or
identity overrides are needed.

Tracker #287/#290 updates link26a96be/0861e3f. #322 links the observed stop
and28b8170; the final full-replay receipt update follows publication.
No independent tracker verification or upstream/source resolution is claimed.
The previous session's native detection-crash observer raw trace was not
transferred; current receipts do not claim to recapture it.
