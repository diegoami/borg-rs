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
