# better-borg-c milestone7 adoption

Owner direction, 2026-11-09: after `4.2.6-impact-v6` (C milestone6,
[Rust#15](https://github.com/diegoami/borg-rs/issues/15)), proceed to C
milestone7. Tracking: [Rust#17](https://github.com/diegoami/borg-rs/issues/17).

Target: better-borg-c `3573d741e6dc3cafa4099fcef8b38fdb32952977`, tagged
`4.2.6-impact-v7` there (patches 0064-0071 and 0073-0077, consumer issues
268/269/270/280/93/224/286/287/289/290/294/296/298), after the milestone6
target `b3c825c97`. Package: `docs/measurement/IMPACT-MILESTONE7.md`, from the
docs-only commit `f9f02be3` that follows the runtime snapshot. C review:
[better-borg-c#301](https://github.com/diegoami/better-borg-c/issues/301)
approved it with corrections, applied in PR303. The accumulated prior-fixture
regression matrix (225 cases) was rerun at the tip. 0072 (spasticscore-147)
was deferred by the consumer before its PR and is not in the series. The C
package marks the 0064/0065 reverse-flow pair and 0071 as behaviour-active.
C approval qualifies the target; it is not Rust signoff.

There are no engine, data, header or FFI changes. Rust starts from main
`e5cb263` (`4.2.6-impact-v6`).

## Reconciliation

[patches](patches) holds the consumer's patch files, byte-for-byte from
`3573d741e`. [series.json](series.json) holds those thirteen entries
unchanged: a filtered subset of the consumer file, plus a `consumer_pin` key.
[ledger.json](ledger.json) adds the Rust disposition. All thirteen are carried
in C and Rust:

| Patch | Correction | Rust site |
|---|---|---|
| 0064 spastic-146 | Spastic reverse flow runs with `optimize` false | `borg_flow_misc.rs` |
| 0065 glyphflow-345 | Glyph reverse flow with `optimize` false (**supersedes local #345**; same call, so no behaviour change) | `borg_flow_glyph.rs` (comment only) |
| 0066 swapacid-199 | The swap weapon's acid resist reads `ELEM_ACID` (was `ELEM_ELEC`) | `borg_trait_swap.rs` |
| 0067 staircycle-403 | Munchkin mode ends while poisoned or hallucinating | `borg_think_dungeon.rs` |
| 0068 text-287 | `borg_what_text` bounded by the destination size (**supersedes local #287**) | `borg_io.rs` |
| 0069 glowtrace-213 | Floors glow from `square_isglow` (corrects impact v5's 0050, which used `square_islit`) | `borg_update.rs` |
| 0070 iswarded-198 | `square_in_bounds` before both `square_iswarded(borg.c)` checks | `borg_messages.rs` |
| 0071 painmsg-197 | Pain buffer has room for the leading space and terminator (last character no longer cut) | `borg_messages.rs` |
| 0073 trollswap-201 | First-half troll reward uses the real slay multiplier | `borg_trait_swap.rs` |
| 0074 drophole-136 | Drop/hold device and potion tests use `==` (were `!=`, always true) | `borg_junk.rs` |
| 0075 wearskip-85 | `skipping` set once, before the loop | `borg_item_wear.rs` |
| 0076 lantern-90 | Refuel scan finds the first lantern | `borg_light.rs` |
| 0077 homebanish-54 | Scrolls of Banishment count into `num_genocide` | `borg_home_notice.rs` |

After the change the C Borg files equal the consumer blobs at `3573d741e`,
except `borg-fight-attack.c` (local #322, comment text) and `borg-update.c`
(local #295). The files that held local #287 (`borg-io.c`, `borg-io.h`,
`borg-messages.c`) and #345 (`borg-flow-glyph.c`) now equal the consumer's.

### Superseded local corrections

- **#287 → 0068.** The consumer's 0068 corrects its own 0025 to pass the
  buffer size, as borg-rs's #287 already did. Taking the consumer's version
  changes one path: when `wcstombs()` fails, the text is now terminated at 0
  and the function goes on to its normal return, where #287 returned an
  error at once. Rust
  `borg_what_text` follows; its empty-buffer guard is kept because it guards
  an index underflow in Rust.
- **#345 → 0065.** Same call (`borg_flow_reverse(250, false, ...)`). C takes
  the consumer blob (comment text) and Rust the matching comment.

## Rust translation notes

- 0066: the `note` helper no longer takes an acid element; it always reads
  `ELEM_ACID`, as C now does in every block.
- 0067: `BI_ISIMAGE` is imported; the exit note text equals C's.
- 0068: the conversion-failure path writes the terminator at index 0 instead
  of returning.
- 0071: the unit test `parsed_pain` pinned the cut last character; it now
  expects the full text.
- 0074: the 24 sval comparisons in the five drop/hold tests changed to `==`.
- 0076: the first lantern is found with a forward scan of the pack; −1 when
  there is none, as in C.
- Comments that described the pre-fix behaviour (`borg_trait_swap.rs`,
  `borg_item_wear.rs`, `borg_flow_glyph.rs`) now describe the fixed behaviour.
  These edits are part of the corresponding removal units.

Per-fix removal units are in [removals](removals) (`git apply -R`): 13 per
side. 0066/0073 split `borg-trait-swap.c`/`borg_trait_swap.rs` by hunk, and
0068/0070/0071 split `borg-messages.c`/`borg_messages.rs`. Every hunk of the
implementation diff is in exactly one unit. Each unit reverse-applies on its
own on the final tree. Reversing all of them restores `4.2.6-impact-v6`
exactly for the changed C and Rust files.

## Native fixtures

[harness/adoption/m7/import.py](../../../harness/adoption/m7/import.py)
imports the thirteen fixtures from their evidence commits. Two runner
features were added to `harness/adoption/m3/native.py`, and neither changes
earlier fixtures:

- per-mode markers: staircycle's `main` mode runs two tests and passes on
  `2/2 passed`, exactly as the consumer's verify script requires;
- extra files: text-287 includes `t287-variant.h`. The consumer writes
  `T287_SIZED 0` before and `1` after, because its before core had the
  five-argument API. borg-rs already had the sized API (local #287) before
  this batch, so the header is `1` for both oracles.

## Validation

See [the validation receipt](../../validation/2026-11-09-c-m7/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
