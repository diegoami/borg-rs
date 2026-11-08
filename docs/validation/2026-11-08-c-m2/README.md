# Carry seven C milestone2 fixes into Rust

Owner selected C milestone2 and requested [Rust issue6](https://github.com/diegoami/borg-rs/issues/6).
Runtime target2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f; distinct reviewed
package9929eb4eb16412ef2415a5711a3eecab92fa1288; C review38/comment6705894080
approved with observations only. Prior Rust acceptance remains m5/0203227;
starting main3797282 also has later tooling/death analysis. This is author
validation for the new batch, not independent Rust signoff.

## Changed behavior and provenance

The seven independent corrections cover safer available emergency swaps,
knockout reaction recovery, side-effect-free regeneration scoring, cold melee
protection, both ring slots in readiness/activation, recent actual CCW drink
timing and exact unseen-name matching. C/Rust change at corresponding sites;
all six changed C files match the target blobs. Original ledger/patch headers,
credit, source/evidence/applied pins and14 independently checked effective
C/Rust removal patches are retained in [adoption/c-m2](../../adoption/c-m2/README.md).
Local287/290/322/295/345 remain; no later C milestone or new tactic is imported.

`validation.json` binds exact source/tool/artifact hashes and counts. Engine/
data qualification binds all354 consumer lib files (identical) and97 unchanged
Rust engine headers. Actual production C/Rust engine link objects match154/154;
two additional unlinked unit-test helpers make156 matching stored objects.
The Rust image contains no C Borg code. C/Rust source and binary identities
are in the copied BUILD_ID files; their precommit dirty/main3797282 stamps
remain truthful, with final effective source hashes rather than relabelled IDs.

## Executed checks

- Original native C correctness49:32pass/17expected failures before;49pass in
  each corrected-C repeat. Full logs match after aliasing only the literal
  isolated PROFILE prefix. There are32 unchanged controls and17 fail-before/
  pass-after cases. Assertions, seeds, actions and native damage/mana rules are
  unchanged. Release fixture compilation matches NDEBUG's flag API; native
  require/eq assertions remain active.
- Selected actual production-body/seam checks:1,785 C/Rust comparisons with C
  UBSan, including potion RNG consumption, first-use/same-clock/elapsed1/4/5/
  negative history, both independent ring gates, regeneration scoring versus
  casting, available/tied swaps, cold/acid guards, message-reset fields and C
  NUL termination. Nine actual Rust fault variants are rejected. These use
  controlled environment callbacks; they are not the full native49 on Rust
  or a shadow of every private state field.
- Real game cases:131/131 strict C/C/Rust matches in keys/messages/player-state
  logs, input/config and summary. All131 full terminal native saves and the
  retained private autosave copies match across the three roles. World-turn,
  seed/race/class, key caps/config and300s per-case wall limits are checked.
- Ordinary9 were uniformly repeated with terminal-save replacing living-save,
  preserving all other flags/inputs/budgets.27 three-log/summary comparisons
  plus existing living-save bytes are unchanged. Three dead profiles now have
  full terminal saves. This is capture noninterference, not extra independent
  lives or outcome-based seed/budget selection.
- Eight join controls on disposable views of four real cases: valid captures
  qualify; missing messages, changed state/full save/private autosave, failed
  processes, altered wall limits and one C execution reused as two do not.
  Real comparisons are rerun; saved PASS output is never an acceptance input.
- Exported target reference:131/131 passes strict input-aware harness/diff;
  all524 compressed streams/full saves decompress to original selected hashes.
- Existing native checker:30,000 calculator, two2,000-state spatial/combat
  batches,2,600 glyph and1,280 lunal states,12fault controls, C UBSan and its
  clippy pass. Rust release100pass/one existing ignored; production all-target
  release clippy, no-C-Borg/link audit and Python compilation pass. Authored
  production/tool changes pass the whitespace check; byte-exact imported patch
  context/signatures, fixture/config files and archived raw test logs retain
  their original formatting and are explicitly excluded from that check.
  No old whole-game coverage percentage is inherited.

## Retained failures and repeats

Initial concurrent C/C/initial-Rust sweeps each retain3wall timeouts (deep
Ranger, Blackguard munchkin,2M Rogue) and11save-directory preflight failures.
The initial Rust name-guard variant is not final-source evidence. Exact same-
input/budget lower-concurrency C repeats reach every original long-case limit;
final-source Rust reaches them within300s. The11cases use the original hashed
suite-specific saves through a private union. No original failed suite gets
a DONE marker or is counted as an uninterrupted full pass.

The final attributed join selects successful executions by exact input/source/
binary/config/budget identity and reruns actual strict diffs and full-save
comparisons. It records original FAILED/DONE statuses and excluded preflight/
timeout origins in `case-qualification.json`. Raw suite invocations/repeats
remain distinct. Timings are not survival or speedup claims.

Other retained diagnostics: first fixture link attempt required matching the
Release flag API; initial exact-name translation failed the NUL boundary and
was corrected; first link audit saw only C's extra unit-helper objects until
both test-helper targets were built; aggregation initially misinterpreted
renamed/mutable private input saves and living-only capture. These were tooling
or author-translation failures, not hidden donor outcomes. The final checks use
pre-launch input hashes plus full cross-role autosave comparisons, and capture
all ordinary9 uniformly. Initial diagnostics remain in harness/out/c-m2.

## Scope and reproduction

[Checker commands and boundaries](../../../harness/adoption/m2/README.md),
[exact effective game inputs](input-plan.json), and
[target oracles](../../../harness/reference/better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f/README.md)
are committed. Raw bundles/objects/executables/private profiles remain in
harness/out/c-m2 and build/rust-m2. Owner saves/settings/display/build/play-rust,
old review worktrees, death dossiers, original references and m4/m5 tags are
preserved. No C donor checkout mutation or official Angband post is made.

This establishes the bounded correction/domain and C/Rust parity for this
target, not population survival improvement, an autonomous win, uninterrupted
save/load equivalence, whole private memory equivalence or universal correctness.
The donor's disclosed unfavorable potion-conservation/RNG tradeoffs remain.
Batch is ready for accumulated Rust review on issue6; no new acceptance/tag
is inferred from implementation.
