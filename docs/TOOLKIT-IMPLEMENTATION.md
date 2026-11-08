# Recording toolkit implementation checkpoint

Scope: owner-requested tooling handoff; no gameplay carry, release, cleanup or
review acceptance. Task branch `tooling/replay-toolkit` starts at
`3211e91c39ddf8480357193a9ea0a426e93a7c61`. Local initial main was
`74fc879`; it was stale. Instructions read: CLAUDE.md, REVIEW, STATUS, M5-PLAN,
C-CONSUMER-ADOPTION, tools/README and death feedback/template.

Live ownership checked 2026-11-08: GitHub issues 3/4/5 own death tooling and
case investigations; issue 11 and branch claude/wonderful-bell-3a7j6m own the
separate milestone-4 gameplay carry. No open PR returned by GitHub REST. No
other task worktree exists in this checkout. Those sources/cases stay intact.

Handoff/documentation commit: a9fb21bc006f71b639ae38b3032583271411581e.
Implementation reference: a4084032485a7c7f29153d0b37ce63dede3e4d5a.
Both downloaded as separate pinned archives; documentation is not attributed
to the earlier implementation commit. C archive source is read-only.

## Initial capability inventory

| Capability | Existing target implementation | Delta required |
| --- | --- | --- |
| Named visible/headless, locks, five backups | run-rust-borg.sh, borg_runtime, borg_saves | font probe; native random birth |
| Frozen Rust self-replay | borg_runtime, borg_gui, native-replay | full headless external streams; counts/integrity; first difference |
| Movie viewer, pack/equipment/prayers | borg_replay_viewer, replay_observations | combat log, geometry validation |
| Human takeover | human_takeover, play-branch | latest, offers, timeout prefix and public wrapper |
| Bounded collector | produce_death, death_workflow | continue every exported death/timeout; build offers; reports |
| Death curation/owner queue | borg_deaths, replay_next_death | honest corrupt cases; manual exclusions |

Rust owns its BorgState including caches, map, goals and local RNG. Its FFI
entry is single-thread checked and supports nested reborrowing. The native
save contains engine state, not private BorgState/stack. Reconstruction always
executes the original Rust library from the session origin. C-only bundles
are not Rust recordings. Calendar, libc time, config reads, native RNG and
terminal polling are separate deterministic inputs/observations; hidden
diagnostics are output only. No strategy is changed.

## Delivered implementation

The [user guide](REPLAY-TOOLKIT.md) connects named visible/headless play, immutable
Rust recording builds, bounded all-attempt collection, reports, original-movie
verification/viewing, native fighting logs, source-pinned human alternatives and
owner review/curation. New observers are applied only in disposable exports;
checked-in `angband/` and `borg-rs/` gameplay sources are unchanged.

Implementation adds native seeded `*` births, source/Rust/toolchain-aware build
offers/reuse, independent actual-clock/wall/key supervision, living save
publication only at a top-level command, report/command backfill, per-death
export/continuation, native pre-coalescing messages/HP loss/speed/energy budget,
every-tick and terminal frames, strict runtime/build/origin bindings and counts,
and GUI/headless/timeout-prefix takeover with exact-helper resume. Existing
knowledge headers match the pinned C reference except for their credit line.

Compatibility is explicit: existing Rust schemas remain readable; the declared
native toolkit extension requires its full new channel set. The C donor schema,
C-only binaries, unknown extension versions and corrupt/incomplete full replay
are refused. Historical certificates do not qualify new channels. Missing
legacy observations are unavailable; arbitrary legacy takeover without an
original selected save anchor is refused. Full replay and prefix proof remain
separate from a manual branch or restoration of private Borg memory.

See the [curated checkpoint](validation/2026-11-08-toolkit/README.md),
[machine receipt](validation/2026-11-08-toolkit/receipt.json) and
[source hashes](validation/2026-11-08-toolkit/source-files.json). Full raw builds,
movies, negative controls, failed prototypes and helpers remain under ignored
`build/death-workflow/` and `harness/out/toolkit-*`; no cleanup was performed.

## Qualification and limits

Final native core: three full original-start Rust self-replays and three
recorder/non-interference pairs passed; one natural death was exported before
both remaining planned living limits. Every requested exact pre-death window
is captured. C/C repeatability and C/Rust smoke parity are separate 3/3 pair
checks (9 executions); the actual link audit has 156 matching engine objects and
zero C Borg objects in Rust. Rust unit tests: 100 passed, one ignored and Clippy pass.
Python/native controls and exact per-test totals are in the receipt.

Fresh/loaded/manual GUI recordings and a real wide window have qualified paths,
but longer visible Borg interruption tests did not consistently return to input.
Their cause is unresolved, not established as a source bug or a fixed driver.
Visible natural-death qualification, missing historical bundles/observer sources,
legacy combat recovery, and arbitrary legacy prefixes without save anchors remain
gaps. Three historical M5 supplementary fixtures are unavailable. No cross-platform,
whole-private-state, survival/progress benefit or unassisted-win claim is made.

The final live check still has main3211e91 and open milestone4 issue11, with its
separate owner branch at80614e4260e9a61750d332fbdb2346c445762c6b. This tooling lane
does not carry that gameplay work or modify owner cases. Next action is a bounded
review/integration checkpoint plus investigation of GUI interruption using retained
private-display evidence. Full/deep main-integration gates and independent review
are not claimed; no acceptance, tag or owner-review mark is inferred.
