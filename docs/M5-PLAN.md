# M5 cutover: post-M4 corrections and operational tooling

Owner approved this checkpoint and the subsequent better-borg-c milestone
adoption path on 2026-11-07. Review: [borg-rs#2](https://github.com/diegoami/borg-rs/issues/2).

The candidate is frozen at **28468aa9fdc74cbb65d351349b973835d8c42af9**.
The comparison base is accepted M4 **3923a21e75dfb94053d4cc787d81c5baf638d62e**.
Current status: author validation complete for the declared receipts;
independent M5 review and owner acceptance pending. Documentation added after
the candidate explains the process; it does not silently change reviewed code.
No M5 tag or first better-borg-c gameplay milestone is declared yet.

M1-M4 remain historical accepted/recorded checkpoints. M4's qualified pass,
closed review#1 and its D/E limits remain intact. This review assesses the
later delta and relevant inherited seams; no new full M4 coverage recollection
is claimed. The old M4-only reviewer instructions apply to that old review.

## Frozen scope

| Change | Commits | Evidence at the candidate |
| --- | --- | --- |
| Known Rune recognition independent of harmful traps | da39bf5, 4af64a6 | validation/2026-11-07-glyph-recognition; C/C/Rust recognition and hidden/trap controls, complete saves |
| Review-tool follow-ups | c2b52d9, f772eb6 | validation/2026-11-07-m4-review-followup and m4-review-residual; distance/tie/cost boundaries, strict oracles, coverage join and launcher race controls |
| Rune reverse-flow correction and bounded play investigation | d0803c6 | validation/2026-11-07-post-m4-investigation; before/after/control construction, kernel faults, lunal budgets, ordinary nine-class cohort |
| Named GUI/headless launch, backups, restore and frozen headless replay | 1be58fd | validation/2026-11-07-launch-tool-port |
| GUI external-input recording and verified replay | 75fbcc6 | validation/2026-11-07-gui-recording |
| Death dossiers, original observation windows, compatible replay CLI | 28468aa | validation/2026-11-07-death-recording |

Links/receipts are relative to docs/. Review production source, test adapters,
negative controls and the artifacts; receipts alone are author evidence.
Tools/IMPORTED.json records donor commits, original paths/hashes and licence.
The tool donor commits are not adopted C gameplay milestones.

## Acceptance questions

1. Are #295 and #345 narrow, correct C/Rust changes with actual expected
   outcomes and fail-before/pass-after/negative controls? Inspect knowledge
   boundaries, real reverse flow, rank/cutoff adapters and save equality.
2. Does claimed construction establish the finite prepared-room result?
   Keep it separate from a naturally earned character, combat benefit or win.
   Lunal reroll behavior is diagnosed and bounded; strategy is unchanged.
3. Do named launches preserve save/settings integrity, lock lifetimes,
   backups and outside-writer protection? Does headless continuation detect
   actual race/class and preserve engine RNG while starting a fresh Borg?
4. Does replay rerun actual Rust from frozen inputs, reject changed/missing/
   partial/divergent streams, retain failures and compare the declared world
   state channels? Exercise GUI stop/manual input/preferences/geometry. Check
   fork-private saves and observation noninterference; no C Borg link leakage.
5. Do death records preserve actual version/input identity, raw cause and
   observations while allowing separate assessment/action updates? Verify
   real deaths versus caps/retirement/failure, family/attempt deduplication,
   exact windows versus nearby anchors, and truthful legacy qualifications.
6. Is this an adequately evidenced cutover checkpoint with remaining risks
   and acceptance blockers clearly identified?

Author checks span corrected C twice/Rust scenario logs and full saves,
normal regression suites, C UBSan/kernel fault controls, release tests/clippy,
154 engine-object/link checks, GUI six-case regression, real GUI/headless
deaths, and 18 death/CLI plus 10 headless and 10 GUI controls. These checks
were run at the source identities in their respective receipts. Earlier
source results and M4 coverage are not new measurements of 28468aa.

## Review and evidence rules

Use an isolated checkout at the full candidate SHA. Never use owner saves,
preferences, GUI display or playable builds. GUI tests require a disposable
--project-root, private Xvfb and xdotool; death tests use a private registry
via BORG_DEATH_RECORDS_DIR. Preserve the three earlier external-review
worktrees and unique evidence. Large frozen bundles remain outside Git;
receipt exports with hashes are not standalone executable replay bundles.

Post reviewer start, material progress/questions and the final verdict on
issue#2. Give severity, exact code locations, reproduction, checks rerun
versus evidence inspected, and pass/qualified pass/fail/insufficient evidence.
The author posts handoff and findings dispositions there; claimed fixes need
explicit reviewer verification. Reviewers do not implement/push fixes, move
tags, close the issue or post to angband/angband. No agent impersonates an
independent reviewer. The owner accepts the result and decides final tagging.
Any code fixes use separate identified commits; the original candidate stays
visible and acceptance names the exact reviewed/follow-up source.

Review limitations remain explicit: sampled kernels are not whole-private-
Borg shadowing; GUI world fingerprints are 64-bit saved-state hashes and
final saves are byte comparisons; captures do not serialize running Borg
memory; headless native-log date headers are separately reported exceptions
while other log bytes and gameplay channels remain exact; old deaths lack
movies; GUI menus before character generation/auxiliary pixel surfaces are
not visual recordings. No unassisted win or universal correctness is claimed.

## Cutover sequence

1. Finish this focused independent review and dispositions; obtain explicit
   owner acceptance of the actual M5 source. Do not infer it from silence.
2. Record the owner's final tag/checkpoint decision without moving M4 or
   changing historical references. No tag is created by this planning change.
3. Follow [C-CONSUMER-ADOPTION.md](C-CONSUMER-ADOPTION.md): choose an accepted
   better-borg-c bug-fixing milestone and pin its exact source/engine/data/
   options/ABI and patch ledger. Leave the first target unselected until real
   acceptance evidence is identified; a recent branch head is insufficient.
4. Reconcile existing local corrections with the C milestone, plan a bounded
   adoption batch, validate corrected C twice and Rust, and retain separate
   target-specific references. Existing neo links stay provenance; that
   tracker's queue no longer independently schedules Rust adoption.

The cutover tracking record is adoption/status.json. It records a pending
transition, not completed C adoption or a new gameplay implementation queue.
