# Rust toolkit qualification, 2026-11-08

Author-run checkpoint on `tooling/replay-toolkit`, based on main
`3211e91c39ddf8480357193a9ea0a426e93a7c61`. No gameplay source or historical
reference is changed. Main integration and independent acceptance are separate.
Read [the guide](../../REPLAY-TOOLKIT.md) and [the implementation record](../../TOOLKIT-IMPLEMENTATION.md).
[receipt.json](receipt.json) contains exact source/observer/binary/library/toolchain
pins, individual raw evidence paths, numerical totals and limits.
[source-files.json](source-files.json) binds the working source at this checkpoint.

## Executed checks

| Check | Actual result |
| --- | --- |
| Python controls, including real supplied death/GUI/prefix fixtures | 99 run: 96 passed, 3 historical external-fixture controls skipped |
| Rust release unit tests | 100 passed, 1 ignored |
| Rust Clippy, `-D warnings` | Passed |
| Full original-start Rust replay | 3/3 final games: one natural death, two living limits |
| Recorder non-interference | 3/3 same frozen Rust binary/seed/options pairs; keys/messages/state/primary frames/config/full final saves/summaries/checkpoints equal |
| C repeatability | 3/3 pairs; C/C/Rust collection totals 9 native executions |
| C/Rust smoke parity | 3/3 pairs; keys/messages/state/primary frames/full saves/summaries equal |
| Production link audit | Passed; 156/156 engine objects match; zero C Borg objects in Rust |
| Native negative controls | 20/20 rejected, including C-only substitution, altered RNG/frame/data/config/save, missing input, footer/schema/path/required-stream faults and active/crash refusal |
| Actual bounded batch | Seed42 natural death at2787, then both planned living caps; all three attempts/report/watch/manual paths retained |
| Original death frames | Actual death row; all7 exact offsets5/12/25/40/60/82/105 present |
| Simulated collection | 10/10 requested deaths separately exported; mixed caps/timeouts and crash/export failure after prior success checked; these are not native lives |
| Named integration | 5 successful native sessions; saved RNG/additional ticks/fixed and native random birth/visible-save load/explicit restore checked; corrupt backup and injected outside writer refused |
| Native wall limits | 2/2 attempts finalized consistently and collection continued; no full-replay claim |
| Strict takeover | Death latest2781, numeric100→living101, finalized timeout latest2501; original-prefix streams and selected native-save fingerprint match |
| Manual resume | 2 separately recorded save/resume sessions replay exactly, using the same prepared helper |
| Real PTY viewer | Seek/step/autoplay/log modes/i/e/s/Escape/q passed; original movie only |
| Native X11 | Fresh/save/load and an actual >255-column window replayed; earlier bounded manual activation/stop/save replayed; longer interruption tests failed and remain unqualified |

Living caps use the **next native top-level command** after the prospective
threshold, completing an already pending command. Final caps5000 produced
actual command boundaries5003 and5858. Tick/key thresholds and this rounding
rule were frozen before outcomes; no extra seeds or retrospective budgets were
added. Independent wall/stall limits still supervise the process.

Full replay compares the original frozen Rust execution. The C/C/Rust smoke
comparison is a different test, with separate source/build pins. Neither is
whole-private-Borg shadow execution, a survival estimate, a win, or cross-platform
bit equality. Short fixtures do not establish deep/endgame policy correctness.

## Retained failures and corrections

- The initial exporter omitted the target's newer `harness/measurement` build
  adapters. The failed build is retained; exports/source hashes now include them.
- A first headless prototype injected scripted terminal keys, skipping the
  frontend's Borg-start side effects. Its failed replay remains retained.
  Headless birth/frontend input is now recomputed and **strictly compared**;
  clock/file inputs are replayed. GUI terminal input is replayed normally.
- Metadata inspection initially lacked explicit frozen screens/help paths.
  The failed preflight left its named save intact. Inspection now uses the
  actual compiled headers and isolated directories.
- The first parity link audit found a real object-byte mismatch from using a
  compatible copy's different `__FILE__` path. The corrected check builds C
  against Rust's actual compiled source directory; no audit assertion was removed.
- A deliberately changed **unused** data file with re-signed file hashes was
  initially accepted. That negative evidence remains under toolkit-negatives1.
  Frozen runtime/build/origin binding checks now reject it. All20 final controls
  pass, including substitution of a real C executable.
- Historical overlay loading generated a Python bytecode file in an experimental
  bundle. The generated file was quarantined with the failed attempt; original
  recorded bytes and manifest hashes were preserved. Loading now uses compile/exec
  without writing bytecode into evidence.
- Failed GUI input waits, a missing timeout-helper receipt directory, and an
  intermediate safe-boundary C declaration error remain in their original
  failure directories/build logs. Receipt setup/declaration failures were fixed.
  GUI interruption is **not** declared fixed by changing a test predicate.

All raw task artifacts remain under `harness/out/toolkit-*` and immutable
`build/death-workflow/*` caches. Successful and failed helpers are dependencies,
not disposable cleanup candidates. Some early aggregate unit logs were replaced
by later invocations; the transcript retains those assertion/fixture diagnostics.
The final complete unit log and native failure artifacts are retained here/by path.

## Remaining work

1. Diagnose the longer X11 interruption failure on a private display with the
   exact captured source/input/observer pins. Do not presume a source bug, fix
   tactics, open a duplicate finding or erase failed movies. Natural visible death
   and general long GUI control remain unqualified.
2. Restore a specific hash-qualified historical bundle/source set before claiming
   legacy combat recovery or arbitrary takeover coverage. Original absent fields
   and save anchors cannot be invented. The three skipped M5 supplements require
   their named external continuation/legacy registry fixtures.
3. Prepare the repository's integration checkpoint/review with the bounded evidence
   here; full/deep main-integration gates were not rerun in this tooling lane.
   Milestone4 gameplay issue11 and its owner branch are separate. No owner review
   mark, acceptance, release or new C carry is inferred.

Post-commit control: [compatible build reuse](post-commit-reuse.json) requested committed tooling6477a7e, reused the exact native image compiled from3211e91 with --no-build, and passed one further full Rust self-replay. This is separate from the frozen three-game qualification batch.
