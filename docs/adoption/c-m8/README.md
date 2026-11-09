# better-borg-c milestone8 adoption

Owner direction, 2026-11-09: after `4.2.6-impact-v7` (C milestone7,
[Rust#17](https://github.com/diegoami/borg-rs/issues/17)), continue with C
milestone8. Tracking: [Rust#19](https://github.com/diegoami/borg-rs/issues/19).

Target: better-borg-c `017e038e8726a7a423887a366b5d40e89c0550a1`, tagged
`4.2.6-impact-v8` there. It carries patches 0078-0089 (consumer issues
305/306/307/309/311/312/316/329/335/336/338/288). Package:
`docs/measurement/IMPACT-MILESTONE8.md`, added after the snapshot (PR345).
An independent review approved it with corrections, which were applied in
PR355. The consumer reran the accumulated 225-case prior-fixture regression
matrix at the tip. Three things in the batch need a note:

- 0083 and 0085 are owner-decided cleanups that remove dead code, with no
  behaviour change.
- 0089 resolves the milestone7 deferral of spasticscore-147. It is verified
  on a consumer-designed product-side fixture.
- regensim-516 was resolved by the consumer as already carried in patch 0003,
  so it has no patch.

C approval qualifies the target; it is not Rust signoff.

**Owner change outside the patch series.** The pinned tree also contains the
owner's "munchkin start by class" change (better-borg-c PR302, `fd683ced`).
New settings `borg_munchkin_by_class` (default TRUE) and
`borg_munchkin_class_depth` (default 6) make Warriors, Druids and Rangers take
the munchkin start when borg.txt leaves `borg_munchkin_start` off. The rule is
applied when borg.txt is read and again after a reincarnation. On the owner's
decision (2026-11-09: carry the whole pinned tree), it is carried in C and
Rust, and `harness/borg.txt` follows the consumer's `borg.txt`.

There are no engine, data or FFI-declaration changes. The Rust engine wrapper
gains `class_cidx_if_set()`, which reads `player->class` only when it is set.
Rust starts from main `ce27f5e` (`4.2.6-impact-v7`).

## Reconciliation

[patches](patches) are the consumer's patch files, byte-for-byte from
`017e038e8`. [series.json](series.json) holds the twelve entries unchanged,
plus a `consumer_pin` key. [ledger.json](ledger.json) adds the Rust
disposition and records the owner change. All of them are carried in C and
Rust:

| Unit | Correction | Rust site |
|---|---|---|
| 0078 banisharte-37 | Banishment artifact test reads `borg.trait[BI_CURHP]` (was `borg.has`) | `borg_fight_defend.rs` |
| 0079 restmult-149 | `borg_check_rest()` no longer clears `last_kill_mult` | `borg_flow_misc.rs` |
| 0080 shopline-125 | A non-goal shop entrance is skipped on any grid but the Borg's (`||`, was `&&`) | `borg_flow.rs` |
| 0081 lightwest-40 | The west radius-2 light marks (y±1, x-2), was (y±2, x-2) | `borg_cave_light.rs` |
| 0082 modcredit-70 | Item value credits the positive stat modifiers, and no longer skips every credit when `pval` is 0 | `borg_item_analyze.rs` |
| 0083 neckdead-47 | Dead `else if (num_neck > 3)` branch removed (cleanup) | `borg_home_power.rs` |
| 0084 lightbeam-88 | Light beam edge test uses the Borg's row on the panel (`y - offset_y`) | `borg_light.rs` |
| 0085 hurtstun-62 | Dead HURT stun test removed (cleanup) | `borg_danger.rs` |
| 0086 dimdoor-21 | Dimension Door aims only at a known floor grid without a monster | `borg_escape.rs` |
| 0087 urgentshop-23 | Poisoned or bleeding: go to and buy from the Alchemy shop (4, was 3) | `borg_think_store.rs`, `borg_store_buy.rs` |
| 0088 deadtake-72 | Lunal take skips dead entries (`kidx == 0`) | `borg_flow_take.rs` |
| 0089 spasticscore-147 | Spastic value uses the candidate grid's own search count | `borg_flow_misc.rs` |
| PR302 munchkin-by-class | Munchkin start by class (owner change, not a fix) | `borg_init.rs`, `borg_headers/borg_h.rs`, `engine.rs` |

After the change, the C Borg files equal the consumer blobs at `017e038e8`,
except `borg-fight-attack.c` (local #322) and `borg-update.c` (local #295).
Neither file is touched by this batch.

## Rust translation notes

- 0082: Rust had folded C's `if (!pval) break;` into an `if pval != 0`
  block around all the credits. That block is removed, and the stat loop now
  reads `modifiers[OBJ_MOD_*] > 0` (C's new test). Each credit still goes
  through the same `long` sum truncated to `int32_t`.
- 0086: the new test reads the Borg's map grid (`st.cave.grid`), in C's
  order: unknown, lava, not a floor grid, monster.
- 0088: Rust's lunal take loop gains the `kidx == 0` skip that the other two
  take loops already had; the comment saying C never skips is removed.
- 0089: `spastic_value()` receives the candidate grid's `xtra`; the comment
  about C's stale `ag` is replaced.
- PR302: the C statics `borg_munchkin_start_setting` and
  `borg_munchkin_depth_setting` live in `InitState`. The rule needs the
  player's class, so the borg.txt reader gets it through `TxtEnv::class_cidx()`.
  In the game that is `engine::class_cidx_if_set()`, which mirrors C's
  `!player || !player->class` guard; the unit-test fake supplies it directly.
  The unit test `munchkin_class_rule` covers the three classes, the other
  classes, no class, the rule off, an explicit munchkin start, the depth
  setting and its clamp, and a reincarnation into another class.
  `default_file_matches_c` now expects the two new settings at the end of
  the default file.
- Comments that described the pre-fix behaviour (0078, 0081, 0083, 0088,
  0089) are removed or rewritten; these edits are part of the corresponding
  removal units.

Per-fix removal units are in [removals](removals) (`git apply -R`): 13 per
side (twelve patches plus `pr302`). 0079 and 0089 split `borg-flow-misc.c`
and `borg_flow_misc.rs` by hunk. Every hunk of the implementation diff is in
exactly one unit. Each unit reverse-applies on its own on the final tree.
Reversing all of them restores `4.2.6-impact-v7` exactly for the changed C,
Rust and `harness/borg.txt` files.

## Native fixtures

[harness/adoption/m8/import.py](../../../harness/adoption/m8/import.py)
imports the twelve fixtures from their evidence commits: eleven in
neo-angband-tracker, and the spasticscore product fixture in better-borg-c,
which is the fixture the consumer's own verify script uses. No runner change
was needed.

## Validation

See [the validation receipt](../../validation/2026-11-09-c-m8/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
