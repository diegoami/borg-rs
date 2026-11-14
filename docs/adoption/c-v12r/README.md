# better-borg-c milestone 12, carried as v12R

Owner direction, 2026-11-13: carry the changes from better-borg-c milestone
12 into borg-rs as **milestone v12R**, the Rust counterpart of the consumer's
`4.2.6-impact-v12`. This is an explicit owner-directed carry, not an automatic
post-M10 adoption queue; the family work remains on its own branches.

Target: better-borg-c `4e1b912cbf6d2b8a0243b19ebddfb2d26e7509f2`, tagged
`4.2.6-impact-v12` there. The previous accepted baseline is `4.2.6-impact-v11`
(consumer `39268ca8a470d1cbd44de27bf1fd1b3ee403bada`; borg-rs PR #30 at
`f4149b4`). Review input: the consumer's milestone-12 package and review map,
`docs/measurement/IMPACT-MILESTONE12.md` and `docs/review/MILESTONE12.md`;
package PR [better-borg-c#478](https://github.com/diegoami/better-borg-c/pull/478)
(the official import) and [#476](https://github.com/diegoami/better-borg-c/pull/476)
(0118 NEWKILL-95).

## Scope

Exactly the consumer's milestone 12, which is two changes in one window from
`4.2.6-impact-v11` to `4e1b912cb`:

| Change | Source | PR / merge | What it does |
| --- | --- | --- | --- |
| 0118 NEWKILL-95 | tracker 95 (CBUG-0079) | 476 / `a39d23d8f` | `borg_new_kill()` no longer forces a newly seen wall-walker's grid to floor (the race was read from the slot before the new race was stored) |
| Official import | `angband/angband` `42cd156ac` | 478 / `4e1b912cb` | merges 12 official commits past `e473720ed` (5 Borg, 7 non-Borg); retires the consumer's carries 0001 and 0002 |

No other C Borg change is carried. The consumer's runtime diff for the window
is 32 files, +674/-128: one carry (`borg-flow-kill.c`, -18) and the import.

## Reconciliation

| Unit | Source | Change | Rust site |
|---|---|---|---|
| 0118 NEWKILL-95 | tracker 95 / CBUG-0079 | Drop the "force the monster to floor / granite" step in `borg_new_kill()`; the slot's previous race is no longer read | `borg_flow_kill.rs` |
| import/0001 SAFERSWAP-0066 | tracker 82 / CBUG-0066 | Upstream `bc67c099e`: both swap candidates start at 9999, the early "no appropriate swap" return is gone, the lower danger wins with the weapon on a tie | `borg_item_wear.rs` |
| import/0002 KNOCKOUT-0002 | tracker 14 / CBUG-0002 | Upstream `9d062a64d` + `8c75bdccb` + `42cd156ac`: the reaction gate is unified on the borg struct field `borg.dont_react`; the static global is gone and the reset in `borg.c` clears the field that `borg_react()` reads | `borg.rs`, `borg_messages.rs`, `borg_messages_react.rs`, `borg_caution.rs` |
| import/engine OFFICIAL-IMPORT | `angband/angband` `42cd156ac` | The vendored engine moves from `e473720ed` to `42cd156ac`: the complete-rest flag on timed effects, the command-queue flush on deep descent, the rest-command background marking, the interrupted-rest state, the terminal clear/fresh split, the Windows open dialog | engine-only; the Rust header mirror follows the `term` rename |

`series.json` is the machine-readable unit list: consumer merges, upstream
commits, patches, C files, Rust files and dispositions.

## The two retirements

The consumer retired its own carries 0001 and 0002 because upstream master now
carries that logic. borg-rs carries the same retirements.

**0001 (CBUG-0066, safer emergency swap).** The consumer's local patch
compared the two candidates as
`if (weapon_swap && (!armour_swap || b_p2 <= b_p1))` with `b_p1 = b_p2 = 0` and
an early `if (!armour_swap && !weapon_swap) return false;`. Upstream's
`bc67c099e` is the same choice rule written as `if (b_p2 <= b_p1)` with
`b_p1 = b_p2 = 9999` and without the early return: the lower danger among the
candidates that actually exist wins, the weapon wins a tie, and an absent
candidate keeps the 9999 sentinel, so it can only be picked in a 9999 tie,
where the acceptance threshold (`b_p <= avoidance/2`) rejects the swap
anyway. The two forms agree on every reachable case; the case the consumer's
form could not reach (neither candidate present) ends in the upstream form
with `b_p = 9999`, which fails the acceptance threshold and queues no key.

**0002 (CBUG-0002, knockout reaction gate).** The consumer's local patch
reset the *global* `borg_dont_react` in `borg.c`, because that was the copy
`borg_react()` read; the struct field `borg.dont_react` it cleared instead was
unused, so a knockout silenced the Borg's reactions for the rest of the
process. Upstream's final form (`42cd156ac`) removes the global entirely and
makes the struct field the single gate: the knockout and paralyzed handlers
set `borg.dont_react`, `borg_react()` and `borg_caution()` read it, and the
message pump clears it. That is the same correction the local carry made,
expressed as one field.

## The engine import

The vendored `angband/` tree moves from `e473720ed` to the imported
`42cd156ac`. All 12 upstream commits are applied, including the seven
non-Borg ones that change engine behaviour the Borg depends on: the
complete-rest flag on timed effects (`player-timed.c`, `player-util.c` and
`lib/gamedata/player_timed.txt`), the command-queue flush when deep descent
triggers (`game-world.c`), the rest command marked as a background command
(`cmd-cave.c`), the cleared state of an interrupted rest, the split of
`TERM_XTRA_CLEAR` from a full erase (`ui-term.c`) and the Windows open
dialog.

borg-rs's own deltas survive the import and are unchanged by it: the
`do_cmd_wiz_cure_all()` curse correction (`docs/FORK-CORRECTIONS.md`, tracker
#474), the 0098 trap guard, the stale-target correction, the family
configuration settings in `borg.h`, `borg-init.c` and `borg.txt`, and the
harness hooks and files (`main-borgh.c`, `native-replay.c`, the key/message
/state log hooks). The tree now matches upstream `42cd156ac` byte for byte
outside those deltas; the only textual difference in the imported Borg files
is borg-rs's sorted include block in `borg-caution.c`.

## Rust translation notes

- **0118:** the Rust port had the same defect and even documented it ("both by
  the slot's old race, see above"). The block that forced the grid is removed
  with the stale-race read.
- **import/0001:** `borg_backup_swap()` keeps the two danger estimates as
  `9999`, drops the early return, and selects the lower one. The Rust
  `swap_letter()` still maps an absent candidate to the 0 key, but an absent
  candidate is only selected in a 9999 tie, where the acceptance threshold
  rejects the swap first, so no key is queued from it.
- **import/0002:** the Rust gate moves from `MsgState.dont_react` (the Rust
  model of the retired C global) to `st.borg.dont_react` (the C struct field).
  The separate copy is removed, so the Rust reads and writes exactly the
  field the C reads and writes, at the same three sites: the knockout and
  paralyzed handlers, `borg_react()`, and the reset in `generate_keypress()`.
  `borg_caution()` reads the same field.
- **import/engine:** the Rust Borg drives the engine through the same FFI, so
  the engine-side changes need no Rust translation. The one declaration the
  Rust mirrors by hand changed name: `struct term`'s `unused_flag` is now
  `xtra_clear` (`ui_term_h.rs`); the layout is unchanged.

## Unit kinds and limits

0118 is a bug-fix carry, so the consumer's approved milestone authorizes it.
The import is an engine and Borg upstream merge, so its Borg-side changes
enter the Rust the same way.

Every unit records its per-mode effect as **not measured** in the consumer's
package; this carry does not add a cohort, a per-mode claim or a
survival/progress benefit. The consumer's own matrix and fixtures remain the
native evidence; this repository's check is C/Rust behavioural parity on the
pinned engine and the Rust unit tests.

## Validation

See [the validation receipt](../../validation/2026-11-13-c-v12r/README.md).

## Independent review

[PR #58](https://github.com/diegoami/borg-rs/pull/58) was reviewed
independently and returned **one blocking defect**: at the head it
reviewed, `d1099ab`, the branch did not compile. `MsgState` no longer has
a `dont_react` field - the reaction gate moved to `borg.dont_react` in this
carry - but `borg-rs/src/borg.rs:433` (the per-message-pump clear) and
`borg-rs/src/borg_caution.rs:1509` (the "Borg ignoring messages" note)
still read the removed field. The cause was a stale push of those two
files, not the carry: the tree that produced the validation below already
had the migration finished.

The author fixed it in `c1e5df3` - both sites use `st.borg.dont_react`
and the `Borg` struct field is the gate itself again - and verified it on
the exact pushed tree: 122 tests pass (one existing ignored) and clippy
reports the same seven pre-existing findings, none of them in the two
changed files. The reviewer's recheck at `071c5c5` (the head that also
carries the refreshed oracle corpus) reports **no remaining findings**.

The reviewer could not run the build or the tests: Cargo is not available
on their side and the repository has **no CI workflow** (no
`.github/workflows` at the repository root on `main` or on this branch, so
no Actions run exists for any head), so the build and test evidence is
the author's - taken on the exact pushed tree and recorded in [the
receipt](../../validation/2026-11-13-c-v12r/README.md). The finding itself
was raised by source inspection.

The implementation branch is `claude/v12r-carry`; the tracking issue is
[borg-rs#57](https://github.com/diegoami/borg-rs/issues/57). Owner
acceptance and any v12R tag decision remain with the owner.
