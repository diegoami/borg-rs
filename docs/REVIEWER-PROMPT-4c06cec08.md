# Independent adjudication of the M3 review

You are reviewing diegoami/borg-rs code at commit
`4c06cec087c11ef03696c8e62fdbc7b854b84d4f`.
Temporary review documentation and evidence are on branch
`review/m3-4c06cec08`; its changes are documentation only. Read:

- `docs/REVIEW-4c06cec08.md`
- `docs/review-evidence-4c06cec08/REPRODUCE.md` and the accompanying sources, logs and suite manifests
- The original review guidance in `docs/REVIEW.md`, plus `docs/PROGRESS.md`, `docs/STATUS.md`, `docs/SELF-REVIEW.md`, `docs/c-bugs.md`, `docs/harness.md` and `CLAUDE.md`

Independently adjudicate the report. Do not assume either its conclusions or the project's defenses are correct. This is a read-only review: do not change implementation code, push, open PRs, or post issues/comments. Never open issues, PRs or comments on angband/angband. Write your response to a local Markdown file and return its path.

Prioritize these questions:

1. Reproduce finding 1 using the actual native C and Rust helpers. Trace the ordinary wear trial through `borg_notice(true)` and the ID/power calculation. Is the note-origin counterexample real? Can `ident=false, needs_ident=false` and the necessary description lengths arise from valid engine objects? Distinguish synthetic helper divergence, reachable state/scoring divergence, and demonstrated seeded key-stream divergence. Try to construct an engine-backed fixture or save/seed reproducer. If it cannot happen, prove the invariant that excludes it. Evaluate the row-60 note argument, rather than treating the previous zero-mismatch instrumentation as proof.
2. Reproduce finding 2 with `-DNDEBUG`. Audit the related `flag_next`, `race_flag_has`, `player_has` and `spell_okay_to_cast` wrappers. Separate safe-API unsoundness for invalid inputs from actual current-game reachability. Check the SAFETY arguments and L4/L5 lifetime/aliasing concerns, and whether lending covers engine calls that can re-enter the Borg.
3. Reproduce wrong-binary acceptance. Assess how serious it is for the documented default workflow versus the explicit `ANGBAND_BIN` override. Check what manifests and `check-no-c-borg.sh` actually prove about the executable.
4. Assess the stale-position `square_iswarded` lead (#198) using message-flush order, monster energy on new levels, persistent levels and nested entry. Do not turn an unconstrained fuzz state into a claimed normal-play crash without a valid path.
5. Check the recorded suite/test results and the clippy toolchain distinction. Re-run targeted checks first; run the full suites if needed. Do not present a Rust 1.98 lint failure as disproving a historical clean Rust 1.97 run.

For every numbered report finding, give a verdict (confirmed, partially confirmed, rejected, or unresolved), severity, exact file:line references, reproduction or counterevidence, and a suggested faithful fix if warranted. Explain any severity change. Identify any additional concrete defects you discover. Do not silently fix intentional C Borg behavior.

Finish with separate verdicts on finite-suite equivalence, general faithfulness, FFI soundness, and whether declaring M3 complete is justified. List checks performed and remaining limitations. Preserve the distinction between a proved defect and a plausible gameplay consequence.

These documentation artifacts are temporary; leave deletion to the owner when adjudication is complete.
