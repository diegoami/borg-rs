# better-borg-c milestone4 adoption

Owner direction, 2026-11-08: after tag `4.2.6-impact-v3` (C milestone3,
[Rust#8](https://github.com/diegoami/borg-rs/issues/8)), proceed to C
milestone4. Tracking: [Rust#11](https://github.com/diegoami/borg-rs/issues/11).

Target: better-borg-c `70993393a1c946ce8cd38ee558bf1ce6464350e7`, tagged
`4.2.6-impact-v4` there (patches 0035-0042, consumer issues 133/137/141/144/
148/157/167/176), after the milestone3 target `702f091f6`. C review:
[better-borg-c#184](https://github.com/diegoami/better-borg-c/issues/184#issuecomment-6712503043)
("substantially sound, no blocker"); its eight documentation findings were
[applied](https://github.com/diegoami/better-borg-c/issues/184#issuecomment-6712604871).
Package: `docs/measurement/IMPACT-MILESTONE4.md` there. C approval qualifies
the target; it is not Rust signoff. Engine base stays `e473720ed`; no engine,
data, header or FFI change.

Rust starts from main `3211e91c39ddf8480357193a9ea0a426e93a7c61`
(`4.2.6-impact-v3`).

## Reconciliation

[series.json](series.json) and [patches](patches) are the consumer's records
for 0035-0042, byte-for-byte from `70993393a`. [ledger.json](ledger.json) adds
the Rust disposition. All eight are new (absent from pre-adoption C and Rust)
and are carried in both:

| Patch | Correction | C / Rust site |
|---|---|---|
| 0035 brand-240 | The brand-ammo need is counted once per stack (the duplicate block after the ammo-power test is removed) | `borg_notice_ammo` / `borg_trait.rs` |
| 0036 stairtrack-232 | The "known up staircase" loop `break`s on a match, so a known staircase is not appended again | `borg_think_stair_scum` / `borg_think_dungeon_util.rs` |
| 0037 acidarc-252 | Ball and arc blast loops include the south/east radius edges (`<=`) | `borg_launch_bolt_at_location`, `borg_launch_arc_at_location` / `borg_fight_attack.rs` |
| 0038 outoflight-229 | A dying light (timeout <= 3, not `NO_FUEL`) starts the out-of-light maneuvers | `borg_think_dungeon_light` / `borg_think_dungeon_util.rs` |
| 0039 mapwall-212 | `old_wall` is taken before the map feature is overwritten, so wall/door changes are noticed | `borg_update_map` / `borg_update.rs` |
| 0040 CBUG-0065 (source81) | A recognised Enlightenment potion is quaffed in the dungeon (never in town) | `borg_use_things` / `borg_item_use.rs` |
| 0041 stairreturn-230 | The out-of-light stair test returns after queuing `<` | `borg_think_dungeon_light` / `borg_think_dungeon_util.rs` |
| 0042 blowname-247 | The blow-name search stops before the NULL end entry (an unknown name gives `MONBLOW_NONE` instead of a crash) | `borg_mon_blow_effect` / `borg_fight_attack.rs` |

After the change the C files equal the consumer blobs at `70993393a`, except the
seven files that hold the retained local corrections, the same set as milestone3:
#287 (`borg-io.c/.h`, `borg-messages.c`, `borg-think.c`), #295
(`borg-update.c`), #322 (`borg-fight-attack.c` comment text) and #345
(`borg-flow-glyph.c`). The consumer diff applied cleanly next to them.

## Rust translation notes

- 0037: the arc loop now reaches `y1 + max`. `max` is clamped to 20 in
  `borg_launch_arc`, so the angle-table index `ry - y1 + 20` stays in 0..40 of
  the 41×41 table (the same bound as C).
- 0038: `NO_FUEL` is tested with `engine::flag_has`, as elsewhere in the port.
- 0039: Rust takes `old_wall` from the grid it already borrows, directly after
  `BORG_OKAY` is set, at the same point as C.
- 0042: the Rust table keeps its `None` end entry, and the search iterates over
  all entries except that one, matching C's `i + 1 < size`.

Per-fix removal units are in [removals](removals) (`git apply -R`). Each
reverse-applies on its own on the final tree. Reversing all 16 in sequence
restores `4.2.6-impact-v3` exactly for the changed C and Rust files.

## Validation

See [the validation receipt](../../validation/2026-11-08-c-m4/README.md). The
native fixtures are in `harness/adoption/m4/fixtures` and run with the
milestone3 runner (`harness/adoption/m3/native.py --fixtures ...`).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance. Review and owner acceptance are recorded on Rust#11.
