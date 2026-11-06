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

## Corrections under validation

| issue | correction | evidence |
|---|---|---|
| [#290](https://github.com/diegoami/neo-angband-tracker/issues/290) | reset stale panel offsets at equality with cave width/height, before allocating and writing detection arrays; same guard in C/Rust | original seed4006 crashes in both C passes and Rust; separate unchanged-C signal observer locates the fault in borg_update's borg_detect_wall write, q_y=1, q_x=0, next-row pointer NULL; full 2M-turn corrected replay pending |
| [#287](https://github.com/diegoami/neo-angband-tracker/issues/287) | reserve one byte for NUL in multibyte screen conversion; return empty text/error on invalid encoding; same conversion contract in C/Rust | native regression links the real C function: fails before and passes after for ASCII, UTF-8, output boundary and invalid character; full five-case endgame replay pending |

`harness/suites/crash-fixes` contains the exact original Rogue and five
endgame scenarios, including the original seeds, saves and turn limits.
Validation uses corrected C twice and corrected Rust once, with full
key/message/state comparisons. The 105-case baseline checkpoint follows. Earlier background six-case
batches were interrupted without completed receipts; rerun in bounded
batches rather than treating their partial outputs as completed tests.
Any new failure is investigated rather than exempted automatically.

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
