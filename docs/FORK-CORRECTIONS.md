# Narrow fork corrections before the divergence milestone

Owner direction, 2026-11-06: inherited C behavior may be corrected when
it crashes or is a blatant play blocker. Mirror the narrow correction
in C/Rust where applicable, mark FORK, link the tracker and test it.
Other conflicts are decided case by case. Do not preserve crashes,
undefined bytes or known failures as milestone requirements. Broader
changes follow a verified milestone and an owner-created tag.

This supersedes older absolute no-C-changes/no-behavior-changes wording
for these corrections. Pinned upstream stays e473720ed. Original references
remain separate; new corrected-C references require two identical C runs.
No official Angband issue/PR/comment or automatic main/tag move is authorized.

## Validated production corrections at28b8170

| tracker | correction | evidence |
|---|---|---|
| [#290](https://github.com/diegoami/neo-angband-tracker/issues/290) | reset stale offsets at equality with cave dimensions before queued detection writes | same C/Rust guard; original Rogue4006 scenario now finishes its2M limit in C twice and Rust, matching all three logs |
| [#287](https://github.com/diegoami/neo-angband-tracker/issues/287) | use actual caller byte capacity, reserve NUL, return empty text/error on invalid conversion | real-function native before/after controls, full UTF-8/boundary/zero-one-capacity/error checks; original full baseline restored; all five endgame cases match C/C/Rust |
| [#322](https://github.com/diegoami/neo-angband-tracker/issues/322) | require thrust targets at distance exactly one | actual C body fixture rejects center/distant targets and retains all eight neighbors; source26 Warrior2000's `+5` stop at1,887,392 becomes a valid adjacent attack; all three corrected runs reach5,000,001 |

Latest receipt: [2026-11-06-melee](validation/2026-11-06-melee/README.md).
Fresh C/C and Rust/C crash-fixes6/6, extended12/12 and natural2/2 compare
keys/messages/player state, with DONE, successful process outcomes and no
overrides. Every extended scenario reaches its original limit. Both builds
also match105 original baseline cases and8 captured-Rogue references.
Thirty forked engine captures have identical metadata and direct saved
bytes across C1/C2/Rust. They restore engine state for fresh replays, not
private Borg memory. Current corrected references are crash-fixes,
extended and natural; original baseline/endgame bytes are preserved.

The prior session described an unchanged-C observer locating the detection
next-row write with q_y=1, q_x=0 and a NULL row pointer. Its raw trace was
not transferred; current receipts do not claim to recapture that trace or
independently verify the tracker findings upstream.

## Why the UTF-8 correction was refined

The transferred bd184cc byte-limited fix passed six cases, but both builds
matched only43/45 original full cases (while matching each other45/45).
Screen columns were still used as UTF-8 byte limits, truncating Sméagol
messages into phantom names; seed1003 also changed keys/state. The native
full-message regression reproduces the truncation.

Commit26a96be supplies actual C buffer capacity (`sizeof` at all five
callers); Rust uses its slice length. Full105-case parity returned. That
source's extended matrix then matched12/12 but exposed the separate melee
stop. Commit28b8170 corrects only that invalid target eligibility; the
unexpected-direction abort safeguard remains.

Historical references/receipts are explicit: crash-fixes-bd184cc,
crash-fixes-26a96be, extended-26a96be and their source-qualified validation
directories. The latest six-case logs are unchanged from the valid26
reference, but their current manifest/receipts bind fresh runs to28b8170.
Earlier interrupted cloud batches remain incomplete, not passes.

## Tracker and unresolved scope

Published #287/#290 updates:
[#287 comment](https://github.com/diegoami/neo-angband-tracker/issues/287#issuecomment-6693735109),
[#290 comment](https://github.com/diegoami/neo-angband-tracker/issues/290#issuecomment-6693741157).
#322 records the observed stop and narrow correction; its [final receipt
update](https://github.com/diegoami/neo-angband-tracker/issues/322#issuecomment-6695689265)
links the completed75d6ced checkpoint and was read back. Independent tracker verification and upstream
source resolution remain explicit, not inferred from consumer validation.

Allocator code-reading leads #323–#325 are separate reported findings in
c-bugs section55; their C implementations are unchanged. The committed
native calculator checker is an E foundation. The48-run coverage pilot is
not full F or coverage of the latest melee correction. M4 remains open;
Sea of Runes, self-lunal and a win are not established by a depth alone.
