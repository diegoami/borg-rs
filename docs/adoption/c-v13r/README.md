# better-borg-c milestone 13, carried as v13R

Owner direction, 2026-11-14 ("you can do v13r"): carry the changes from
better-borg-c milestone 13 into borg-rs as **v13R**, the Rust counterpart of
the consumer's `4.2.6-impact-v13`. This is an explicit owner-directed carry,
not an automatic post-M10 adoption queue; the family work remains on its own
branches.

Target: better-borg-c `9ccb2aaeebc56ad6aeb35b668f112f0f58b142f8`, tagged
`4.2.6-impact-v13` there (the tag was placed on that snapshot the same day).
The previous accepted baseline is `4.2.6-impact-v12` (consumer
`4e1b912cbf6d2b8a0243b19ebddfb2d26e7509f2`; borg-rs PR #58, tagged
`4.2.6-impact-v12`). Review input: the consumer's milestone-13 package
(`docs/measurement/IMPACT-MILESTONE13.md`, review map
`docs/review/MILESTONE13.md`, package PR
[better-borg-c#520](https://github.com/diegoami/better-borg-c/pull/520)) and
its review issue [#521](https://github.com/diegoami/better-borg-c/issues/521).
**The consumer's verdict landed the same day: approve with corrections.**
The corrections (M13-F1..F3, merged in consumer PR 531, `72cdf63bb`) are
docs/fixture-tooling only on the consumer's side - the fixtures gain a
portable profile root, the review map and package ledger get doc fixes -
and postdate the tag, so they belong to the consumer's milestone-14
window. No `src/borg` code changed: the carried snapshot `9ccb2aaee` is
exactly the approved state. borg-rs's own independent review happens on
[the PR](https://github.com/diegoami/borg-rs/pull/72).

## Scope

Exactly the consumer's tagged milestone 13: **nineteen carry units**
(patches 0119-0137), twelve from the consumer's issue-426 queue and seven
from the owner-directed source-board pairs. The consumer's runtime surface
is 16 Borg files, +155/-124. The C blobs in this tree are the consumer's
v13 files; the Rust carries the same nineteen units.

| Patch | Source | Consumer PR | What it fixes | Rust site |
| --- | --- | --- | --- | --- |
| 0119 | 116 (CBUG-0100) | 483 | a Sea-of-Runes read on an already-tracked grid reports success | `borg_flow_glyph.rs` |
| 0120 | 193 | 485 | Lunal mode is left when a monster teleports the Borg | `borg_update.rs` |
| 0121 | 101 (CBUG-0085) | 487 | the dead-unique seeding test is uninverted | `borg_flow_kill.rs` |
| 0122 | 94 (CBUG-0078) | 489 | race deaths are indexed by race, not by kill record | `borg_flow_kill.rs` |
| 0123 | 52 (CBUG-0036) | 491 | respawn types the birth menu's own letters | `borg_reincarnate.rs` |
| 0124 | 69 | 493 | comment: grid danger always rates the full damage on purpose | `borg_danger.rs` |
| 0125 | 111 | 495 | the dead Venom mana-cost test is removed | `borg_fight_perm.rs` |
| 0126 | 115 | 497 | the always-zero glyph count is dropped from the outlying score | `borg_flow_glyph.rs` |
| 0127 | 122 | 499 | the unreachable visible-trap exploration case is removed | `borg_flow_dark.rs` |
| 0128 | 126 | 501 | the unreachable 255-door cap is removed | `borg_flow.rs` |
| 0129 | 176 | 503 | the unreachable Teleport Other fail tier is removed | `borg_fight_defend.rs` |
| 0130 | 486 | 505 | comment: why rest-danger rates a monster at its own grid | `borg_flow_misc.rs` |
| 0131 | 38 (CBUG-0022) | 507 | Confuse Monster / Lightning Balls wands looked up by name | `borg_item_val.rs` |
| 0132 | 202 | 509 | fear protection is recorded on swap weapons and armour | `borg_trait_swap.rs` |
| 0133 | 592 | 511 | fixed 20/40/60 heal amounts replace the 3.x percentages | `borg_caution.rs` |
| 0134 | 596 | 513 | the stun and blindness cure lists drop what cannot cure | `borg_recover.rs` |
| 0135 | 589 | 515 | protection from stun modelled separately (`BI_PSTUN`) | `borg_trait_h.rs`, `borg_trait.rs`, `borg_danger.rs` |
| 0136 | 594 | 517 | a vulnerability cancels the matching permanent resistance | `borg_trait.rs` |
| 0137 | 610 | 519 | unresisted elemental vulnerability x4/3 in `borg_danger` (stacked on 0136) | `borg_danger.rs` |

`series.json` is the machine-readable unit list.

## Not taken, and why

- **Consumer carries 0138-0140** (sources 154, 590, 591, merged after the
  v13 tag on the same day; 0141 and more followed): post-tag work, the
  consumer's own review issue scopes them to milestone 14. They are not
  part of `4.2.6-impact-v13` and are not carried.
- **source616**: verified by the consumer to be a byte-identical duplicate
  of carried patch 0004; not carried there, not carried here.
- **Sources 13, 249, 250**: no released source fix (250 closed upstream as
  a duplicate of the carried #67).
- **Sources 595 and 615**: classified by the consumer lane but explicitly
  not carried there yet ("await the owner's direction").

## Local deltas preserved

Six Borg files keep their deliberate local state, none of which a v13
hunk touches:

- `borg-caution.c` — the include order (v12R).
- `borg-update.c` — the 0098 guard (milestone 9; better-borg-c#375).
- `borg-fight-attack.c` — the stale-target fork comment
  (`docs/c-bugs.md` section 54).
- `borg-init.c`, `borg.h`, `borg.txt` — the munchkin-off default and the
  family-lane buy-strategy settings (owner decisions, not carried
  upstream).

## The trait-enum shift

Patches 0135 and 0137 insert five traits into `borg-trait.h`
(`BI_PSTUN` after `BI_RDIS`; `BI_VFIRE`/`BI_VELEC`/`BI_VCOLD`/`BI_VACID`
after `BI_CRSAVULN`). The Rust mirror
(`borg_headers/borg_trait_h.rs`) holds every trait index as a literal, so
it was regenerated from the carried header; every constant after the
insertion points shifted (+1 and +4). This is the header-shift hazard the
consumer's own review map names for these two patches. The Rust
`prefix_pref` mirror in `borg_trait.rs` received the same two insertions
at the same positions (`prot stun`; the four `unresisted vuln` entries).

## Validation

See [the validation receipt](../../validation/2026-11-14-c-v13r/README.md).

## Owner acceptance

Owner direction (2026-11-14): accept and merge. PR #72 was merged at
`5dcc1db` and the annotated tag **`4.2.6-impact-v13`** marks the merged
main commit, continuing the series after `4.2.6-impact-v12`. The GitHub
milestone "4.2.6 impact v13" is created and closed with tracking issue
[#73](https://github.com/diegoami/borg-rs/issues/73). The post-M10
family lane is separate and unaffected.
