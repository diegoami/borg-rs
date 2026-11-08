# better-borg-c milestone6 adoption

Owner direction, 2026-11-08: after `4.2.6-impact-v5` (C milestone5,
[Rust#13](https://github.com/diegoami/borg-rs/issues/13)), proceed to C
milestone6. Tracking: [Rust#15](https://github.com/diegoami/borg-rs/issues/15).

Target: better-borg-c `b3c825c97c8aef18b252a7fb7d9b7dceef83d586`, tagged
`4.2.6-impact-v6` there (patches 0051-0063, consumer issues 242/245/247/250/252/
254/257/258/259/261/263/266/267), after the milestone5 target `5384f12ed`. C
review: [better-borg-c#273](https://github.com/diegoami/better-borg-c/issues/273)
approved it with two required documentation corrections and one verification
condition, applied or recorded in PR274. The C lane's accumulated regression
matrix was not rerun at its tip. Package:
`docs/measurement/IMPACT-MILESTONE6.md` there. The C package marks 0053 and
0056 as behaviour-active and discloses a layered 0054/0053 interaction (see
below). The package's "source250 overlap" is the Teleport Other reader fix
already in consumer patch 0012, which borg-rs carried in impact v3, so nothing
new is needed. C approval qualifies the target; it is not Rust signoff.

There are no engine, data, header or FFI changes. Rust starts from main
`6b58a46444ab3f5d28cfd86c7ac820d54c762c47` (`4.2.6-impact-v5`).

## Reconciliation

[patches](patches) are the consumer's patch files for 0051-0063, byte-for-byte
from `b3c825c97`. [series.json](series.json) holds those thirteen entries
unchanged: a filtered subset of the consumer file, plus a `consumer_pin` key.
[ledger.json](ledger.json) adds the Rust disposition. All thirteen are new and
carried in C and Rust:

| Patch | Correction | Rust site |
|---|---|---|
| 0051 stairrange-245 | Munchkin climb range only widens when the clevel term is positive | `borg_think_dungeon.rs` |
| 0052 clockreset-246 | Clock "Step 2" resets the clock and keeps playing instead of aborting | `borg_think_dungeon.rs` |
| 0053 dangerflee-156 | Flee on excessive danger unless ready for Morgoth (`!= 1`, was `== 1`) | `borg_caution.rs` |
| 0054 recallmana-157 | Recall-wait mana test compares with `MAXSP / 4` (was `CURSP / 4`) | `borg_caution.rs` |
| 0055 deskstair-153 | Desperate flight picks the **nearest** free stairs (both scans) | `borg_caution.rs` (`nearest_free_stair`) |
| 0056 backaway-155 | Back-away scan: a trapped grid is skipped (not the end of the scan), and the adjacent-monster flag resets per grid | `borg_caution.rs` |
| 0057 cfghang-141 | A non-value character ends the setting parse (keeps the default) instead of looping forever | `borg_init.rs` |
| 0058 cfgminus-142 | A `-` only negates a number that follows it | `borg_init.rs` |
| 0059 cfgmissing-144 | Defaults are set before borg.txt is read; an unreadable rewritten borg.txt warns and keeps them | `borg_init.rs` |
| 0060 countsell-134 | Count-sell skips an unidentified single item instead of returning 0 | `borg_store_sell.rs` |
| 0061 drainedive-235 | Do not dive whenever experience is drained (dead `g` gate removed) | `borg_think_dungeon_util.rs` |
| 0062 respois-184 | Resist Poison reward counter starts at 0 | `borg_power.rs` |
| 0063 respoiswar-185 | Warrior Resist Poison reward counter starts at 0 | `borg_power.rs` |

After the change the C Borg files equal the consumer blobs at `b3c825c97`,
except the same seven files that hold the retained local corrections.

## Rust translation notes

- 0055: Rust had folded both of C's stair scans into one helper; it is
  renamed `nearest_free_stair` and keeps C's `b <= j && b != -1` test.
- 0056: Rust's `adjacent_monster` is declared without an initial value and set
  to false at C's reset point, so the compiler rejects any read before the
  per-grid reset.
- 0057/0058: these touch the same lines of `borg_proc_setting` in C and Rust,
  so they share one removal unit (`0057-0058`).
- 0059: Rust filled in its defaults inside `read_txt_settings`. They are now
  also set at the start of `borg_init_txt_file`, as in C, so every early
  return leaves them in place; the later re-fill before parsing is
  idempotent. The re-read failure that used to panic now warns and keeps the
  defaults.
- Unit tests: `parses_like_c` pinned the old `borg_no_deeper -` → −127. It now
  expects the default 127 (0058), and gains a 0057 case (`borg_respawn_race = x`
  keeps the default, and the next line still parses).

Per-fix removal units are in [removals](removals) (`git apply -R`): 12 per side.
Each reverse-applies on its own on the final tree. Reversing all of them
restores `4.2.6-impact-v5` exactly for the changed C and Rust files.

## Layered 0054/0053 interaction

Under 0053, the original recall-mana fixture (`ready_morgoth = 0`) now flees
before reaching the 0054 test, so its `lowmana` mode stays red before and
after, exactly as the consumer asserts. The consumer's adapted scenario
changes that one line to `ready_morgoth = 1`. It is imported as
`recallmana-adapted`, with the transform recorded in the manifest, and its
`lowmana` mode goes from fail to pass (`restore_mana` 0 → 1).

## Validation

See [the validation receipt](../../validation/2026-11-08-c-m6/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
