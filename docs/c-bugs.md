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

## 10. Unreachable and mislabelled preparedness checks (prepared)

* borg-prepared.c, `borg_prepared()`/`borg_prepared_aux()`/`borg_restock()`,
  static (non-formula) mode; found by code reading, confirmed by a
  randomized old-vs-new check (traits, config, uniques and home counts
  perturbed, 81M calls) that never produced these texts.
* `borg_prepared()` calls `borg_restock()` first and returns "Clevel <
  depth" when `max clevel < depth` (below 50), so in `borg_prepared_aux()`:
  "1 Lite" and "5 Fuel" (light 0 is already "restock light radius < 1"),
  "2 tele + teleport staffs" (same test as "restock tele + tele staff < 2")
  and the dlevel 10-19 class checks "dlevel - 4 >= clevel" and "dlevel >=
  clevel" never fire. "2 cures (clw + csw + ccw)" repeats the "2 cure"
  test exactly. The level-100 block ("10 restore mana", which tests for 15,
  "5 Heal", "15 *heal* or life", "10 speed potions") is dead because
  `borg_prepared_aux()` is only called for `depth <= 99`.
* `borg_restock()`: "restock fuel" and "restock fuel < 2" need light 0,
  which already returned "restock light radius < 1".
* "Scumming *Heal* potions (%d to go)" tests the home counts from before
  `borg_notice_home()` but prints `10 - num_ezheal` from after it, and
  ignores the Life potions and the ones carried.
* Rust (`borg_prepared.rs`): all kept.
## 11. Respawn picks the race/class with the wrong menu letters

* `borg-reincarnate.c`, `borg_reincarnate_start()`, only when the Borg
  respawns (game option `cheat_live` on and `borg_cheat_death` off, or
  `borg_respawn_winners`). It types `all_letters[race]` and
  `all_letters[class]`, but the birth menus use `all_letters_nohjkl`
  (ui-birth.c). So index 7 (Half-Troll, Ranger) types 'h', which is no
  menu letter, and indexes 8 and up select the entry before the one meant
  (race 8, Dunadan, types 'i' and gets a Half-Troll; class 8, Blackguard,
  gets the Ranger; race 10, Kobold, types 'k', no menu letter either).
  The random name is still made for the race meant. Seen in local respawn
  runs (seed 1011: race 8 chosen, a Half-Troll with a human name rolled).
* Rust: kept (`engine::all_letters`) in `borg_reincarnate.rs`.
## 12. Home notice: item sustains all count as STR; banishment never counted

* `borg-home-notice.c`, `borg_notice_home_aux()`, by code reading (any run
  that keeps or wears a sustain item reaches the first part).
* Sustains on items: the five `of_has(item->flags, OF_SUST_*)` tests all
  add to `num_sustain_str`, so `num_sustain_int/wis/dex/con` only ever count
  the player's own race/class sustains (added at the end with `rf_has`),
  and `borg_power_home()` sees too many STR sustains.
* `num_genocide` is reset but never counted (scrolls of Banishment are not
  looked for), so the home power reward for them is always 0. (The
  `num_brand_weapon` global is never written at all.)
* Rust: all kept in `borg_home_notice.rs`.

## 13. Log and status slips (log)

* `borg-log.c`, found by code reading.
* `borg_write_map()`: "Your hands are glowing dull red." tests
  `TMD_CONFUSED` again (it meant a different timed effect), so a confused
  Borg's map dump prints both lines. Seen in the full suite's death maps.
* `borg_write_map(true)` (the `^z` map command, not in any suite): when the
  file prompt is cancelled or the file cannot be opened, the file stays
  NULL. `file_putf()` ignores a NULL file, but `file_put()` (the "Swap
  ...: NONE" lines, with `borg_uses_swaps`) and the final `file_close()`
  dereference it, so the game crashes.
* `borg_display_status()`: for "Demon" and "Dragon" the slay's blue is
  always overwritten (`if (WS) blue; if (WK) green; else slate;`), so a
  plain slay shows as slate.
* Rust (`borg_log.rs`): all kept; the NULL-file crash is a panic.
## 14. `borg_dont_react` is never cleared

* `borg-messages.c` sets the global `borg_dont_react` on "You have been
  knocked out." and "You are paralyzed"; `borg_react()`
  (`borg-messages-react.c`) then queues nothing and only notes "# Ignoring
  messages.". The reset in `borg.c` (`borg_inkey_hack` path:
  `borg.dont_react = false;`) clears the unused struct field
  `borg.dont_react` instead, so once set the global stays true for the rest
  of the process: the Borg never again queues hit/miss/kill/SELF messages
  for `borg_update()`, and `borg_caution()` notes "# Borg ignoring messges."
  every turn. By code reading; the full suite never sets it (checked with
  a temporary log of the flag).
* Harmless, same file: `borg_react_prompted()` returns `key` (a pointer)
  from a `bool` function in three places; it is never NULL, so this is
  just `true`.
* Rust: kept (`st.msgs.dont_react` is only ever set; `st.borg.dont_react`
  is reset) in `borg_messages_react.rs` / `borg.rs`.
## 15. Inscribing a quiver item types an equipment letter

* `borg-item-id.c`, `borg_object_fully_id()`, by code reading. The loop
  runs to `QUIVER_END`, and every slot from `INVEN_WIELD` up is selected
  with `'/'` and `all_letters_nohjkl[i - INVEN_WIELD]`. For a quiver slot
  that is index 12 and up ('q', 'r', ...), past the 12 equipment slots;
  the quiver is selected with `'|'` and digits (as `borg_deinscribe()`
  does). Only a quiver item with tval in `TV_BOW..TV_RING` (a throwing
  weapon) that is an artifact or an ego with a random power is affected,
  and the function is only called from the Borg's debug commands
  (`borg-commands.c`), so no suite reaches it.
* Rust: kept in `borg_item_id.rs`.

## 16. Slot lookups: dead "largest pval" test; empty pack has no hole (inventory)

* `borg-inventory.c`, found by code reading.
* `borg_slot()`: the "Prefer largest pval (even if smaller pile)" test
  skips an item only when `item->iqty > borg_items[n].iqty`, but the
  "Prefer smallest pile" test just before has already skipped every such
  item. So pval is never compared: of equally small piles the last one
  wins.
* `borg_first_empty_inventory_slot()` scans down from the last pack slot
  for the last used one and returns the slot after it. With an empty pack
  there is no used slot and it returns -1 ("no room"), not 0, so the
  buy/wear/remove code that needs a hole gives up. (Unlikely in play:
  the Borg nearly always carries something.)
* Rust (`borg_inventory.rs`): both kept.
## 17. Danger slips (danger)

* `borg-danger.c`, found by code reading; all are in every danger
  calculation, so every suite runs through them.
* `borg_danger_physical()`:
  * COLD blows test the temporary *acid* resistance (`borg.temp.res_acid`),
    not `res_cold`.
  * LOSE_CON blows add the "below 8" fear when `BI_CSTR < 8` (strength).
  * HURT: the stun test `z > d_dice * d_side` comes right after
    `z = d_dice * d_side`, so it is never true (dead code).
  * The hit chance `150 - ((ac * 300) / 4 / 1 + (level + power) * 3)` is
    below 5 for any armour class of 2 or more, so partial damage
    (`full_damage == false`) is always 5% of the blow damage.
* `borg_danger_spell()`:
  * `bolt` is set by the first bolt spell and never reset, so every later
    spell of the same monster (bolt or not) has its damage zeroed when the
    monster has no clean shot at the Borg.
  * The summon cases' grid scan assigns to the `x`/`y` parameters
    (harmless: not read again). S_HI_DRAGON with protection from evil
    adds no fear before dividing; S_AINU tests `borg.near.unique` in both
    the first and the (then dead) second branch. STORM ends with a no-op
    `if (borg.trait[BI_RCONF]) break;`.
* `borg_danger_one_kill()`:
  * The second `borg_crush_spell` block (in the ranged part) clears `v1`
    again instead of `v2`.
  * The Teleport Other skip list is read with `ii <= borg_tp_other_n`, one
    entry past the count.
  * The first move simulation tests `feat == FEAT_GRANITE`, the second
    `feat >= FEAT_GRANITE` (so it also treats terrain above the permanent
    wall, e.g. lava and passable rubble, as wall).
* `borg_danger()` overwrites its `full_damage` argument with `true`, so the
  partial-damage paths above are only reached through direct
  `borg_danger_one_kill(..., false)` calls.
* Rust: all kept in `borg_danger.rs`.
