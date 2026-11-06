# Local-session handover — 2026-11-06

Continue on `claude/serene-ramanujan-8hzsfm`. Production Borg inputs are
source `28b8170`; later commits add tooling, fixtures, receipts and docs.
Expanded native E checks are complete for their declared kernels. Full F
coverage collection is active. Main/tags are unchanged; the owner creates
tags. M4 remains open until the complete coverage matrix and report finish.

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

The persistent native checker now covers actual C/Rust calculator30k,
cave/projection/direction/flow/physical-melee4k, glyph900 and lunal1,024
states. It compares the declared complete logical maps, flow matrices,
queues, tracking, decisions and ordered callbacks/notes/keys. Seven
actual-source faults are rejected, including private-state-only mutations.
Non-recovering C UBSan, all-target release clippy and a real dependency
cache invalidation control pass. Source and item ledgers bind the code;
production input hashes still match the frozen coverage image (source28).
See `harness/native-checker/README.md` and
`docs/validation/2026-11-06-native-expanded/`. Shared engine/environment
boundaries are explicit; this is not whole-Borg shadow execution or proof
of every inherited tactic. Actual-game self-lunal/Sea of Runes/win remain
unestablished. Calculator allocator leads #323–#325 and glyph reverse-flow
lead #345 are filed; no gameplay change for these leads.

F's isolated source28 image is frozen at build revisionf987c07 in
`/tmp/borg-rs-m4-coverage`, with matching LLVM22.1.8 tools under
`/tmp/borg-llvm-tools`. The same coverage-only runtime/_exit flush recipe
as the earlier pilot is used. Current full collection:

* `harness/out/local/coverage-current.log` and isolated
  `harness/out/coverage/current-matrix`:119 accepted oracle-checked runs
  through crash-fixes; extended then natural are running/pending.
* `harness/out/local/coverage-current-long.log` and isolated
  `harness/out/coverage/current-long`: independent extended/natural
  collection from the same image. An automatic approval review rejected
  interrupting the first collector after crash-fixes, so both remain active.
* The completed113-run subset report is local only at
  `harness/out/local/coverage-accepted-baseline`:77.56% logic lines,
  90.14% LLVM function instances. It is not full F.

The coverage runner supports `COVERAGE_SUITES` for independent groups;
only all ten default groups create full matrix DONE. Shared validation
checks recipe names/hashes, no overrides/captures, every successful engine
PID, exact frozen binary/source hashes and profile bytes. `join.py` can
join exactly one complete copy of each group from the same image, but
rejects duplicates and partial groups. Never accept an interrupted run.
The previous source26 smoke/full48 pilot remains historical and separate.

A separate instrument-coverage-only image retained the same36,184 mapped
lines/1,709 LLVM functions and strict smoke parity. Four serial A/B runs
all matched the C oracles; the modest variable speed gain did not justify
mixing images during this collection. Current full profiles consistently
use instrument-coverage/link-dead-code. Branch coverage is unmeasured.

## Next work

1. Let the active full collection finish extended/natural and require its
   own DONE. Keep the duplicate collector's provenance separate.
2. Run `harness/coverage/report.py` with no subset flag against the complete
   matrix, retaining the133 successful process/profile receipts and exact
   instrumentation metadata. Review measured gaps; never infer completed
   tactics from nonzero function entries. A prepared local export helper
   is `/tmp/export-borg-m4-coverage.py` (requires complete report/matrix).
3. Update final milestone/status/receipt documentation from the actual
   full report. Production source has not changed, so the completed
   corrected C/C and Rust/C corpus remains the same source28 checkpoint.
4. Present the verified declared M4 scope for the owner to tag. Do not
   automatically move main or create a tag.

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
`baseline-melee-*`, `melee-crash-c2`, `natural-*`. No old gameplay jobs should be resumed
from their completed/partial predecessors; the coverage jobs above are new. Read DONE/strict comparisons.
Current builds used GCC15.2.0 and rustc1.98.1. No cloud dependency paths or
identity overrides are needed.

Tracker #287/#290 updates link26a96be/0861e3f. #322 links the observed stop
and28b8170; its [final full-replay receipt update](https://github.com/diegoami/neo-angband-tracker/issues/322#issuecomment-6695689265)
is published and was read back.
No independent tracker verification or upstream/source resolution is claimed.
The previous session's native detection-crash observer raw trace was not
transferred; current receipts do not claim to recapture it.
