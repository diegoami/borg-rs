# better-borg-c milestone9 adoption

Owner direction, 2026-11-09: after `4.2.6-impact-v8` (C milestone8,
[Rust#19](https://github.com/diegoami/borg-rs/issues/19)), continue with C
milestone9. Tracking: [Rust#21](https://github.com/diegoami/borg-rs/issues/21).

Target: better-borg-c `33207fbeeb582e67d6b18e9f40613617667905ae`, tagged
`4.2.6-impact-v9` there. It carries patches 0090-0099 (consumer issues
359/361/363/365/367/369/371/373/375/377). Package:
`docs/measurement/IMPACT-MILESTONE9.md`, added after the snapshot. C review:
[better-borg-c#380](https://github.com/diegoami/better-borg-c/issues/380),
approve with corrections. The consumer reran the 225-case prior-fixture
regression matrix at the tip. The pinned tree has no source changes outside
the patch series. C approval qualifies the target; it is not Rust signoff.

There are no engine, data or FFI-declaration changes. The Rust engine
wrapper gains `square_trap_is_glyph()` again; it reads the existing `square`
and `trap_info` bindings. Rust starts from main `286215c`
(`4.2.6-impact-v8`).

## Reconciliation

[patches](patches) are the consumer's patch files, byte-for-byte from
`33207fbee`. [series.json](series.json) holds the ten entries unchanged,
plus a `consumer_pin` key. [ledger.json](ledger.json) adds the Rust
disposition. All ten are carried in C and Rust:

| Patch | Correction | Rust site |
|---|---|---|
| 0090 starvealch-18 | A Borg with no food may shop at the Alchemist; the Remove Hunger "immediate shopping" test is fixed; Remove Hunger scrolls count as food when there is none | `borg_store_buy.rs`, `borg_trait.rs` |
| 0091 torodvalue-549 | The Teleport Other defence also zaps a Rod of Teleport Other | `borg_fight_defend.rs` |
| 0092 healchance-364 | The three healing-chance adjustments take the direction their comments state | `borg_caution.rs` |
| 0093 homebuyqty-19 | A skipped home-buy trial restores the home ware | `borg_store_buy.rs` |
| 0094 tradehist-20 | `bought_item_num` and `sold_item_num` count the newest entry | `borg_store_buy.rs`, `borg_store_sell.rs` |
| 0095 torodpower-560 | A Rod of Teleport Other counts in `BI_ATPORTOTHER` | `borg_trait.rs` |
| 0096 tporod-17 | Every class but Mage and Rogue may buy the Rod of Teleport Other (`!=`, was `==`) | `borg_store_buy.rs` |
| 0097 panelrows-348 | Panel scans stop at the edge of the Borg's map on large terminals (memory safety) | `borg_flow_misc.rs`, `borg_think_dungeon_util.rs` |
| 0098 runeglyph-295 | A visible Rune of Protection is recorded on the Borg's map. **Supersedes local #295; carried with a local NULL guard** (below) | `borg_update.rs`, `engine.rs` |
| 0099 necrolight-296 | The Necromancer spell penalty applies on any lit square (`square_islit`). **Carried with a local bounds guard** (below) | `borg_magic.rs` |

After the change the C Borg files equal the consumer blobs at `33207fbee`,
with three exceptions:
- `borg-fight-attack.c` keeps local #322 (comment text).
- `borg-update.c` differs from the consumer only by the 0098 guard. Local
  #295, which this file used to carry, is retired in favour of 0098.
- `borg-magic.c` differs only by the 0099 guard.

### 0098 and the guard

Local #295 and consumer 0098 fix the same source finding (tracker #295), in
different ways:

- On a glyph grid, #295 set `trap = false`; 0098 leaves the previous `trap`
  value.
- #295 took the glyph from the trap the player remembers. 0098 enters on the
  remembered glyph, then reads the cave's trap: `square(cave, l)->trap->t_idx`.

The second point fails when a rune is broken out of view. The engine
refreshes the player's memory only for seen grids, so the glyph stays
remembered while the cave trap is NULL, and 0098 dereferences NULL.

The borg-rs fixture [staleglyph](../../../harness/adoption/m9/local/staleglyph/scenario.c)
reproduces this. It removes a remembered rune out of view, as `mon-move.c`
does, then runs `borg_update()`.
- On the consumer core, mode `stale` gets SIGSEGV, and ASan reports a SEGV in
  `borg_update`.
- Control mode `fresh` passes.
- The v8 tree (local #295) passes both modes.

On the owner's decision (2026-11-09: reproduce, then guard; report upstream),
0098 is carried with one narrow guard. In C, the cave trap is checked for
NULL before its index is read. In Rust, `engine::square_trap_is_glyph()`
returns false without a trap; before #295 it asserted on a missing trap. With
the guard, both modes pass in normal and ASan builds, and the consumer's
runeglyph kit rows are unchanged. The lead and the fixture are reported on
[better-borg-c#375](https://github.com/diegoami/better-borg-c/issues/375#issuecomment-6743759756)
for the consumer's own process. The guard sits inside the 0098 removal unit;
the unguarded consumer behaviour is documented by the fixture run, not by a
separate unit.

### 0099 and the guard

0099 calls `square_islit(cave, borg.c)` from `borg_spell_fail_rate()`. On a
new level, `borg.c` is set only by the next map update, and before that
`borg_think()` → `borg_notice()` → `borg_spell_legal_fail()` reaches the new
test with the previous level's position. When the new level is smaller
(dungeon → town), the position is out of bounds, and C reads outside the
square array (its asserts are compiled out).

In the first 131-case run on the unguarded build, nine cases failed in both
roles: all eight Necromancer cases and `cfg-munchkin-dunadan-bg`. They failed in
C with SIGSEGV and in Rust with an abort on its bounds assert. A gdb trace of one
case gives `square_islit` ← `borg_spell_fail_rate` ← `borg_spell_legal_fail`
← `borg_notice` ← `borg_think`. A temporary print (reverted) showed
`borg.c=(117,47)` on the 66×22 town.

Under the standing crash-correction policy, and following the owner's
decision for the 0098 guard, 0099 is carried with one narrow guard: the
penalty applies only when `borg.c` is in bounds, in C and Rust. With the
guard, the same game reaches the turn limit, and the necrolight kit rows are
unchanged. Reported on
[better-borg-c#377](https://github.com/diegoami/better-borg-c/issues/377#issuecomment-6746360588).
The guard is part of the 0099 removal unit.

## Rust translation notes

- 0090: C counts Remove Hunger scrolls in a local variable of
  `borg_notice_inventory()`. Rust's per-item helper `notice_pack_item()` takes
  it as a `&mut i32`, and the food fallback follows C's position, after the
  high- and low-calorie food is added.
- 0091: only `borg_defend_aux_tele_away()` changes. The depth-100 Teleport
  Other site in the same file is unchanged in C, and is unchanged in Rust.
- 0093: both "require two empty slots" tests now restore the ware with the
  existing `restore_ware()` helper, and the two stale "C leaves the home ware
  changed" comments are removed.
- 0094: the field docs of `bought_item_num`/`sold_item_num` now describe a
  count.
- 0095: the rod svals copy (`RodSvals`) gains `tele_other`. The new branch
  sits after the speed rod, as in C.
- 0097: Rust already panicked at the map edge rather than reading past it.
  The loops now stop at `AUTO_MAX_Y`/`AUTO_MAX_X`, as in C.
- 0099: `engine::square_islit()` (the M5 binding) replaces the `BORG_LIGHT`
  test, and the now-unused `BORG_LIGHT` constant in `borg_magic.rs` is
  removed.

Per-fix removal units are in [removals](removals) (`git apply -R`), 10 per
side:
- 0090/0093/0094/0096 split `borg-store-buy.c` and `borg_store_buy.rs` by
  hunk.
- 0090/0095 split `borg-trait.c` and `borg_trait.rs` by hunk.
- Every hunk of the implementation diff is in exactly one unit.
- Each unit reverse-applies on its own on the final tree.
- Reversing all of them restores `4.2.6-impact-v8` exactly for the changed C
  and Rust files.

## Native fixtures

[harness/adoption/m9/import.py](../../../harness/adoption/m9/import.py)
imports the ten consumer fixtures from their neo-angband-tracker evidence
commits, plus the local staleglyph fixture. panelrows and staleglyph also run
under AddressSanitizer. No runner change was needed.

## Validation

See [the validation receipt](../../validation/2026-11-09-c-m9/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
