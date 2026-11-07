# M5 cutover review preparation

Owner approved preparing M5 at28468aa and, after acceptance, following
verified better-borg-c bug-fixing milestones. Review issue:
https://github.com/diegoami/borg-rs/issues/2.

Frozen candidate: 28468aa9fdc74cbb65d351349b973835d8c42af9.
Historical accepted M4: 3923a21e75dfb94053d4cc787d81c5baf638d62e.

This preparation adds only documentation/policy and a tracking record. It does
not change the candidate, production source, references, historical receipts,
existing death identities or tags. Independent review and owner acceptance
remain pending. No first accepted C gameplay milestone or adoption batch is
selected; tooling donor pins are not consumer gameplay acceptance.

Read docs/M5-PLAN.md, REVIEW-M5-PROMPT.md, C-CONSUMER-ADOPTION.md and
adoption/status.json. CLAUDE/REVIEW/STATUS/handover now lead to the M5 task;
the old direct source-tracker workflow is clearly historical for the future
post-M5 lane. Existing source issues and milestones retain their provenance.
No external C repository policy, source or issues are modified.

The author rechecked1,457 top-level archived artifact-hash entries across
seven receipt groups: glyph-recognition, M4 follow-up/residual, post-M4
investigation, launch tools, GUI recording and death logging. All match.
Production paths angband/, borg-rs/, Cargo.toml and Cargo.lock have no diff
against the candidate; recorded C/Rust input hashes match its later frozen
receipt identities. receipt-audit.json contains exact ledgers/counts/status.

This is a read-only author integrity/source-pin audit, not independent review,
new gameplay execution, fresh native/kernel tests or coverage measurement.
SOURCE_SHA256SUMS inside older exports identifies historical sources; it must
not be rewritten or treated as an assertion that today's docs match old docs.
The reviewer should independently rerun the checks appropriate to the delta,
post material findings on issue#2, and identify unavailable local raw bundles.
Git exports containing frame/manifest receipts omit full executables/data.

M5 final tagging and acceptance remain explicit owner decisions. The review
issue contains the author/reviewer comment protocol; an empty reviewer thread
never means approval. Keep the M4 review closed and tag/source unchanged.
