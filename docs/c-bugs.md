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

## 3. Last token of each block not ordered (formulas)

* borg-formulas-calc.c, `adjust_order_operations()`: the blocks passed to
  `adjust_order_ops_depth_block(f, pdepth, start, itoken - 1)` are scanned
  with `tok < end`, so the last token of each block is never looked at.
  Harmless for well-formed formulas (a block ends in a value); a malformed
  one such as `(1 + 2 +) 3` parses and evaluates to 3 with "error
  calculating a formula value" notes. Only with `borg_uses_dynamic_calcs`;
  not seen in any suite. Pinned by a unit test in `borg_formulas_calc.rs`.
## 4. "West of the player" lights the wrong grids

* `borg-cave-light.c`, `borg_update_light`, every run with a radius-2+
  light (the full suite reaches this branch thousands of times).
* The "East" block lights (y, x+2), (y+1, x+2), (y-1, x+2), but the "West"
  block lights (y, x-2), (y+2, x-2), (y-2, x-2) instead of (y+1, x-2) and
  (y-1, x-2). So (y+-1, x-2) never get `BORG_LIGHT`, and (y+-2, x-2) can be
  added a second time to `borg_light_x/y` (already added by the South/North
  blocks), contrary to the macro's comment. Also, with radius 4+ the four
  (y+-3, x+-3) grids can be entered twice (once by the diagonal checks and
  once by the box scan). Duplicates only make readers of the array
  (`borg_flow_dark`, `borg_update`) visit a grid twice.
* Rust: kept as is in `borg_cave_light.rs`.

## 5. Spell status set from a bool; books 0-7 only (magic)

* `borg-magic.c`, `borg_init_spell()`, all caster seeds, startup:
  `spell->status = spell_okay_to_cast(player, spell_num);` stores a `bool`
  in a field that holds `BORG_MAGIC_*` values, so a spell starts as 0
  (`BORG_MAGIC_ICKY`) or 1 (`BORG_MAGIC_LOST`) until
  `borg_cheat_spells()` sees its book in the pack. Harmless in practice:
  every reader also requires the book to be carried, and carried books are
  re-cheated first.
* `borg_cheat_spells()` resets `borg.book_idx[0..9]` but only cheats books
  0..7 (`book_idx < 8`); no class has a ninth book.
* `borg_spell_stat()` tests `&books[0].spells[0] != NULL`, which is always
  true.
* Rust (`borg_magic.rs`): all kept (`u8::from(status)`, `0..8`, no check).

## 6. Formula error text lost to `format()`'s static buffer

* borg-formulas.c, `parse_value()` and `get_value_index()`, only when a
  borg.txt formula has a bad `value()` and `borg_uses_dynamic_calcs` is on
  (by code reading, then reproduced with the C file and stubs).
* Three callers pass `format(...)` as the `error` argument of
  `borg_formula_error()`: "value type (first parameter '%s') not found" and
  (twice) "value error for object type '%s' subtype '%s'". `format()`
  returns its static buffer, and `borg_formula_error()` formats its own
  notes with `format()` before it notes `error`. So the error note repeats
  the "** error for section value '...'" note and the caller's message is
  never shown. (That message also has an off-by-one: it prints
  `value_type_names[t].name`, but the table starts at `VT_NONE`, so it is
  the name of type `t - 1`, e.g. 'scroll' for a potion.) If the "error on
  line" note grew `format()`'s buffer (a line of more than ~1000 bytes), C
  would read freed memory instead; the Rust port assumes it does not.
* Rust: `formula_error_format()` in `borg_formulas.rs` notes the "error for
  section" text twice. Pinned by `errors_match_c`.

## 7. `range()` errors (formulas)

* borg-formulas.c, `parse_range()`, only with `borg_uses_dynamic_calcs`.
  A missing ')' is reported as "** first value is not a number", and a first
  value that really is not a number fails with no note at all, so the
  formulas are turned off silently. (The section printed in these errors is
  cut at the ',', where C wrote a NUL.) Kept as is.
## 8. Home power: spell-stat drop at +26 and a dead neck branch

* `borg-home-power.c`, `borg_power_home_aux1()`, by code reading (no suite
  run stores +26 of the spell stat at home; checked by fuzzing the c2rust
  version against the Rust one in-game).
* Spell stat: the last tier is `20 * 100L + 6 * 300L + (s - 26) * 5L`; the
  tier below it starts from `20 * 400L`. So the home value of the spell stat
  falls from 9500 at +25 to 3800 at +26, surely meant to be `20 * 400L`.
* Amulets: `if (num_neck > 3) ... else if (num_neck > 3) value -= (num_neck
  - 3) * 700L;` repeats the condition, so the 700 penalty never applies
  (the other item types use a lower second threshold, e.g. rings 6 / 4).
* Rust: both kept in `borg_home_power.rs` (the dead branch is left out, as
  clippy rejects an `if` with the same condition twice; a comment marks it).
## 9. Dark-grid test uses `==` instead of `&` (magic play)

* `borg-magic-play.c`, `borg_play_magic()`, every call (study and town
  spell testing): `if (borg_grids[borg.c.y][borg.c.x].info == BORG_DARK)`
  compares the whole info byte, so the "Dark" early return only fires when
  `BORG_DARK` is the grid's only flag. Every other reader of `BORG_DARK`
  (borg-commands.c, borg-think-dungeon-util.c) tests the bit. The player's
  grid normally also has `BORG_VIEW`/`BORG_MARK` etc., so in practice the
  check almost never triggers; `BI_LIGHT` is the real guard.
* Rust: kept as `== BORG_DARK` in `borg_magic_play.rs`.
