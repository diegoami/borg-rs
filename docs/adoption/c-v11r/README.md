# better-borg-c milestone 11, carried as v11R

Owner direction, 2026-11-10: carry the changes from better-borg-c milestone 11
into borg-rs as **milestone v11R**, the Rust counterpart of the consumer's
`4.2.6-impact-v11`. This is an explicit owner-directed carry, not an automatic
post-M10 adoption queue. The family work remains on its own branches.

Target: better-borg-c `39268ca8a470d1cbd44de27bf1fd1b3ee403bada`, tagged
`4.2.6-impact-v11` there. The previous accepted baseline is
`4.2.6-impact-v10` (consumer `6944d88e87d02e9d24e85438d25f0c6ef864c170`;
borg-rs commit `399c907950b1fd474ddfc1c5a5d4f86063778aac`). Review input:
[better-borg-c#471](https://github.com/diegoami/better-borg-c/issues/471);
package PR [better-borg-c#472](https://github.com/diegoami/better-borg-c/pull/472);
tag record [better-borg-c#474](https://github.com/diegoami/better-borg-c/pull/474).

## Scope

Exactly the 18 runtime units of the consumer's milestone 11. Their 19 patch
files are `0100`-`0117` (`0101a` corrects `0101`). No other C Borg change is
carried. The consumer's full runtime diff is 102 added and 75 removed lines in
11 files.

The C Borg sources in this tree are updated to the same consumer blobs at
`39268ca8a` for those 11 files, so the existing C oracle checks the Rust
changes. The only expected C-side local difference is the family parser
settings already added to `borg.h` outside this diff.

## Reconciliation

| Unit | Source | Correction | Rust site |
|---|---|---|---|
| 0100 STAIRPHASE-34 | 34 | A stair escape is a complete alternative at danger levels 3.1, 4.1, 5.1, 6.1 and 8 | `borg_escape.rs` |
| 0101/0101a LUNALTAKE-73 | 73 | Skip `item_bad` objects outside munchkin mode; gate on runtime `borg.munchkin_mode` | `borg_flow_take.rs` |
| 0102 FLOWKILLHALL-98 | 98 | Reset `skip_monster` and `hall_walls` per monster; test the wall count after the scan | `borg_flow_kill.rs` |
| 0103 HITCHANCE-63 | 63 | Divide the armour term by the monster's to-hit; cap the partial chance at 100% | `borg_danger.rs` |
| 0104 TAKESTAIR-71 | 71 | Keep the nearest up staircase in all three item flows | `borg_flow_take.rs` |
| 0105 RESTPREP-108 | 108 | Widen `no_rest_prep` to `int`/`i32` so it cannot wrap | `borg.rs`, `borg_fight_defend.rs`, `borg_fight_perm.rs`, `borg_flow_stairs.rs`, `borg_update.rs` |
| 0106 PREPREASON-234 | 234 | Copy the current depth's preparedness reason before later calls overwrite the buffer | `borg_think_dungeon_util.rs` |
| 0107 LOSECON-61 | 61 | Test constitution, not strength, for the constitution-drain blow fear | `borg_danger.rs` |
| 0108 CRUSHV2-66 | 66 | Clear the ranged danger (`v2`) when Crush would kill the monster | `borg_danger.rs` |
| 0109 GRANITESIM-68 | 68 | Test exactly granite in the ranged move simulation, not everything above it | `borg_danger.rs` |
| 0110 SCOOTIDX-100 | 100 | Shoot'N'Scoot reads the adjacent monster's danger, not the direction index | `borg_flow_kill.rs` |
| 0111 EMPTYSLOT-60 | 60 | An empty pack has first empty slot 0 | `borg_inventory.rs` |
| 0112 EATHURT-80 | 80 | Drink cures for food only when current HP is below maximum | `borg_item_use.rs` |
| 0113 RINGSLOT-86 | 86 | Remove the stale left-ring duplicate check; the right ring keeps it | `borg_item_wear.rs` |
| 0114 SWAPTRIAL-31 | 31 | Restore the trial pack slot from `safe_items` after each home swap | `borg_store_buy.rs` |
| 0115 HOMESKIP-29 | 29 | Reset the home-grab `skip_it` flag for each ware | `borg_store_buy.rs` |
| 0116 SELLCHARGES-132 | 132 | Restore staff/wand charges with the quantity after each home-sell trial | `borg_store_sell.rs` |
| 0117 EGOSELL-133 | 133 | Check the sold item's own ego for unknown random powers | `borg_store_sell.rs` |

`series.json` is the machine-readable unit list: consumer merge commits,
patches, C files, Rust files and dispositions.

## Rust translation notes

- **0104:** C repeats the nearest-stair scan in three functions. Rust already
  factors it into `nearest_up_stair()`, so the one helper is corrected and all
  three flows inherit it. The helper keeps the first of equally near stairs.
- **0109:** Rust's `wall_like()` had a `granite_or_more` boolean because the
  two C loops differed. Both now test exactly `FEAT_GRANITE` (plus the same
  seam types), so the boolean is removed.
- **0105:** the Rust field was `i16` and two assignment sites cast to it. The
  field is now `i32`; the truncating casts and the comments that documented
  them are gone. C uses plain `int`.
- **0106:** Rust's `PreparedC::Buffer` models C's static buffer. The current
  depth's text is copied right after the first `borg_prepared()` call, before
  the `depth + 1`, `depth + 5` and `max_depth * 5 / 10` calls.
- **0114:** Rust's `item_from_safe(st, hole, hole)` is the `memcpy` from
  `safe_items[hole]`; it replaces `item_mut(st, hole).iqty = 0`.
- **0116:** Rust's `is_charged()` is the C `TV_STAFF || TV_WAND` test; the
  charges are restored with `charge_each` beside `iqty++`.
- **Review F1:** the `borg_flow_take_lunal()` doc comment now says the bad-item
  verdict is used outside munchkin mode, and the no-rest timer no longer
  wrapping at level 33 has a focused unit test.

## Unit kinds and limits

These are bug-fix carries, so the consumer's approved milestone authorizes
them. Every unit records its per-mode effect as **not measured** in the
consumer's package; this carry does not add a cohort, a per-mode claim or a
survival/progress benefit. The consumer's own matrix and fixtures remain the
native evidence; this repository's check is C/Rust behavioural parity on the
pinned engine and the Rust unit tests.

The consumer's package also notes that 0101a is a correction of 0101 and that
the mode gate is the only unit with a two-mode fixture. The Rust code carries
the corrected form.

## Independent review

[PR #30](https://github.com/diegoami/borg-rs/pull/30) was reviewed
independently: **approve**, with one minor doc finding (F1, stale Lunal
comment) fixed in `e8f939a`. The reviewer reproduced the C blob identity, all
18 unit mappings, 104 tests, and C/Rust parity on smoke 3/3, full 45/45, deep
9/9, classes 16/16, config 7/7 and options 25/25. Owner acceptance and any
v11R tag decision remain.

## Validation

See [the validation receipt](../../validation/2026-11-10-c-v11r/README.md).
