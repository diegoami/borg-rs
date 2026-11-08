# better-borg-c milestone5 adoption

Owner direction, 2026-11-08: after `4.2.6-impact-v4` (C milestone4,
[Rust#11](https://github.com/diegoami/borg-rs/issues/11)), proceed to C
milestone5. Tracking: [Rust#13](https://github.com/diegoami/borg-rs/issues/13).

Target: better-borg-c `5384f12ed51e8fef6f31353fd2daaf0e7fb43856`, tagged
`4.2.6-impact-v5` there (patches 0043-0050, consumer issues 188/196/201/205/
214/216/212/224), after the milestone4 target `70993393a`. C review:
[better-borg-c#229](https://github.com/diegoami/better-borg-c/issues/229), which
approved it with one required documentation fix; three findings were applied
(PR232). Package: `docs/measurement/IMPACT-MILESTONE5.md` there. The C package
marks 0050 as behaviour-active. C approval qualifies the target; it is not
Rust signoff. The engine base stays `e473720ed` plus the retained cure-all
correction. The batch adds one Rust FFI binding (`square_islit`, an existing
engine function); there are no engine, data or C header changes.

Rust starts from main `d3e94d3eb939ea9bf22a1a1ab2e8e38d60c78cf4`
(`4.2.6-impact-v4`).

## Reconciliation

[patches](patches) are the consumer's patch files for 0043-0050, byte-for-byte
from `5384f12ed`. [series.json](series.json) holds those eight entries
unchanged: a filtered subset of the consumer file, plus a `consumer_pin` key.
[ledger.json](ledger.json) adds the Rust disposition. All eight are new
(absent from pre-adoption C and Rust) and are carried in both:

| Patch | Correction | C / Rust site |
|---|---|---|
| 0043 excavate-236 | The excavated vault grid is marked `BORG_DARK` after clearing `BORG_GLOW` (it used to set `BORG_GLOW` again) | `borg_excavate_vault` / `borg_think_dungeon_util.rs` |
| 0044 calllight-231 | The Call Light scan includes the Borg's own grid (`ii <= 8`), so the open-door-underfoot case is reachable | `borg_think_dungeon_light` / same |
| 0045 gloves-205 | Gloves are armour-swap candidates | `borg_notice_armour_swap` / `borg_trait_swap.rs` (`is_armour`) |
| 0046 detectcarry-219 | Object-detection panel flags are cleared on a new level, like the other detections | `borg_update` / `borg_update.rs` |
| 0047 maimfoe-256 | A real Maim Foe call returns the best score `b_d`, not the last scanned `d` | `borg_attack_aux_maim_foe` / `borg_fight_attack.rs` |
| 0048 recalldepth-310 | At max depth 99 with fewer than 5 live uniques, the recall-depth prompt is answered `y`, as its comment says | `borg_recall` / `borg_escape.rs` |
| 0049 flowcache-404 | Flows that skip the danger check (desperate, lunal, munchkin, digging) no longer mark grids as checked, so a later normal flow assesses them | `borg_flow_spread`, `borg_flow_enqueue_grid`, new `borg_flow_ignores_danger` / `borg_flow.rs` |
| 0050 glowtrace-213 | The map update's Floors branch is live (`FEAT_FLOOR`); a floor is dark or glowing by `square_islit()` | `borg_update_map` / `borg_update.rs` (+ FFI binding) |

After the change the C Borg files equal the consumer blobs at `5384f12ed`,
except the same seven files that hold the retained local corrections. The
consumer diff applied cleanly next to them.

## Rust translation notes

- 0049: when the mode skips the danger check, both C and Rust now also skip
  the `borg_danger` call in `borg_flow_enqueue_grid`, not only the icky
  marking. The two nested conditions in `borg_flow_spread` are written as one
  `&&` (clippy); the behaviour is the same.
- 0050: Rust previously left out C's dead Floors branch, with only a comment.
  It is now translated, and calls the engine through the new
  `engine::square_islit` binding, so both images read the same engine state.
  The C path's blindness branch is empty, as before.
- 0044: `ddx_ddd`/`ddy_ddd` have 9 entries in the engine and in Rust's
  binding; index 8 is (0, 0), the Borg's own grid. Only the Call Light loop
  changes; the earlier neighbour scan in that function keeps `0..8`.
- 0045: a Rust unit test (`tvals`) that pinned gloves as non-armour now pins
  them as armour.

Per-fix removal units are in [removals](removals) (`git apply -R`). Each
reverse-applies on its own on the final tree. Reversing all 16 restores
`4.2.6-impact-v4` exactly for the changed C and Rust files.

## Validation

See [the validation receipt](../../validation/2026-11-08-c-m5/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
