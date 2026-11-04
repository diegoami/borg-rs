# Known C Borg bugs (reproduced faithfully)

The Rust port keeps each of these exactly as the C behaves. They are recorded
here so nobody "fixes" one by accident.

Format: file, function, (seed, turn when observed), explanation.

## 1. `borg.has[BI_CURHP]` instead of `borg.trait[BI_CURHP]`

* File: `angband/src/borg/borg-fight-defend.c:2498`,
  `borg_defend_aux_banishment()`
* Seen: by code reading during M2/M3 (not tied to a seed). The banishment
  path needs a LOSKILL activation item, which the suites have probably never
  reached.
* `using_artifact = borg_equips_item(act_loskill, true) && borg.has[BI_CURHP] > 100;`
  `borg.has[]` is indexed by object kind, so `BI_CURHP` is treated as a
  k_idx. The test reads how many of some unrelated object kind the Borg
  carries, when the intended check is surely "current HP > 100" (as the
  comment-free sibling checks elsewhere do with `borg.trait[BI_CURHP]`).
* Rust: `crate::bridge::st().borg.has[BI_CURHP as usize]` (same index into the
  same table).

## 2. Copy-and-paste slips in sval lookups

* `borg-item-val.c`, `borg_init_item_val`, all seeds, startup:
  `sv_wand_confuse_monster` is looked up as "Scare Monster" (the same as
  `sv_wand_fear_monster`) and `sv_wand_elec_ball` as "Lightning Bolts" (the
  same as `sv_wand_elec_bolt`); copy-and-paste slips. `sv_mush_cure_mind` is
  "Clear Mind" like `sv_mush_mana` and `sv_potion_berserk` is looked up twice
  (both harmless). Kept as is in `borg_item_val.rs`.
