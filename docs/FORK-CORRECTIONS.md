# Crash corrections before the divergence milestone

Owner direction, 2026-11-06: behaviour inherited from C may be corrected
when it causes crashes or is a blatant bug that prevents playing. Other
conflicts are decided case by case. A documented C failure need not be
preserved as an acceptance requirement. The broader divergence follows
a verified milestone and an owner-created tag.

This supersedes the earlier absolute no-C-changes/no-behaviour-changes
wording in HANDOVER, CLAUDE and REVIEW for these narrowly scoped fixes.
The pinned upstream source remains e473720ed; local corrections must be
marked FORK, mirrored in C and Rust where applicable, linked to tracker
issues, and tested before/after. Original references are not silently
replaced. New corrected-C references require two identical C runs.
Do not post to angband/angband. No tag or main movement is automatic.

## Validated corrections

| issue | correction | evidence |
|---|---|---|
| [#290](https://github.com/diegoami/neo-angband-tracker/issues/290) | reset stale panel offsets at equality with cave width/height before queued detection writes; same guard in C/Rust | original seed4006 crashes in both C passes and Rust. Current corrected C twice and Rust once finish the unchanged 2M-turn scenario at 2,000,352 turns, with identical keys/messages/state |
| [#287](https://github.com/diegoami/neo-angband-tracker/issues/287) | convert using the caller's actual byte capacity, reserve NUL and return empty text/error on invalid encoding; same contract in C/Rust | actual-function regression fails before and passes after; all five unchanged endgame cases match corrected C/C/Rust in all three logs |

Fresh builds of `26a96bee4570` have clean build inputs; uncommitted docs
and evidence account for `git_dirty=yes`. All three six-case passes have
`DONE`, all 18 processes exit 0, and strict C/C and Rust/C comparisons each
report **6/6 identical**, with no overrides. The compressed corrected-C
reference `harness/reference/crash-fixes` also matches C2. Original
references, including `endgame`, retain their original bytes. See the
[current receipt](validation/2026-11-06-capacity/README.md).

The suite keeps the exact original Rogue seed, five endgame seeds/saves,
configuration and turn limits. Earlier interrupted cloud batches remain
incomplete, not passes. The prior session describes an unchanged-C signal
observer locating a NULL next-row detection write (`q_y=1`, `q_x=0`), but
its raw trace was not transferred; the current receipt does not claim a
new native crash trace or independent tracker verification.

## UTF-8 capacity refinement

The initial `bd184cc` correction passed six-case C/C/Rust, but matched only
43/45 original `full` cases in both builds (while C/Rust matched each other
45/45). Seeds1001/1003 parsed phantom Sméagol names, and seed1003 changed
keys and state: limiting conversion to `n` bytes still truncated UTF-8,
because `n` counts screen columns. A native regression against the real
function reproduces the truncation.

Commit `26a96bee4570` supplies actual output capacity (`sizeof` every C
caller's buffer; Rust's slice length). It preserves complete UTF-8 when it
fits without changing parser logic or tactics. The expanded regression
passes, and both implementations now match all 45 original `full` cases
in keys/messages/state. The separate initial reference
`harness/reference/crash-fixes-bd184cc` and
[its receipt](validation/2026-11-06-corrected/README.md) retain the exact
source identity and strict baseline failures. They are historical evidence,
not acceptance of the latest sources.

Both builds now match all 105 original baseline cases and all eight
captured Rogue replays in all three logs (see the current receipt). The
full extended matrix is running; the native differential checker and
coverage measurement remain separate M4 work.

## Independent saved-character coverage

Eight natural Rogue captures produced identical saved bytes in two C
source passes. `harness/suites/captured-rogue` replays every capture to
the next fixed 100k boundary. All eight original C/C/Rust replays match
in keys, messages and player state. They restore engine state and start
fresh Borgs, so they add independent coverage, not uninterrupted recovery.
This fallback remains useful but no longer substitutes for correcting the
confirmed crash. See `harness/saves/captured-rogue-SOURCES.json`.

M4 E (native differential checker) and F (coverage measurement) remain
separate steps. C/D fixes alone do not complete the entire M4 milestone.

## Tracker receipts

Published correction/validation updates were read back on
[#287](https://github.com/diegoami/neo-angband-tracker/issues/287#issuecomment-6693735109)
and [#290](https://github.com/diegoami/neo-angband-tracker/issues/290#issuecomment-6693741157).
They link `26a96bee4570` and validation checkpoint `0861e3f`, keep consumer
correction separate from independent tracker verification and make no
upstream/source-resolution claim. IDs are retained in
`validation/2026-11-06-capacity/tracker-updates.json`.

## Same-square melee play blocker (#322)

The source26a96be extended matrix matches C/C/Rust12/12, but Warrior2000
stops at1,887,392 turns after teleport/blindness leaves a stale monster on
the player's square. Thrust accepts distance zero, queues `+5` and hits the
unexpected-direction abort. Commit28b8170 narrowly requires distance exactly
one in C/Rust. The actual C body fixture fails before and passes after for
center/distant rejection and all eight adjacent targets in both modes.
The prompt-abort safeguard is unchanged. Full new-source replay validation
is running; the source26 historical reference is separately identified.
Tracker: https://github.com/diegoami/neo-angband-tracker/issues/322.
The three calculator allocation leads (#323–#325) are source-reading leads
only; their C implementations have not been changed.
