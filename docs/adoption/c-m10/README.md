# better-borg-c milestone10 adoption

Owner direction, 2026-11-10: after `4.2.6-impact-v9` (C milestone9,
[Rust#21](https://github.com/diegoami/borg-rs/issues/21)), continue with C
milestone10. Tracking: [Rust#23](https://github.com/diegoami/borg-rs/issues/23).

Target: better-borg-c `6944d88e87d02e9d24e85438d25f0c6ef864c170`, tagged
`4.2.6-impact-v10` there. It carries no new bug ordinal: its units are two
owner-directed **local improvement units** (I-001, I-002; never proposed
upstream, charter in the consumer's PR 389) and one **crash correction to an
already-carried unit** (0099). Package:
`docs/measurement/IMPACT-MILESTONE10.md`, added after the snapshot. C review:
[better-borg-c#420](https://github.com/diegoami/better-borg-c/issues/420),
approve with corrections (M10-R-01..07), all applied in the snapshot commit
and re-reviewed approve at `46057bc4a`. The consumer's full prior-fixture
matrix at this tip is 77 groups, 234 cases, `ok`, with 183/234 byte-identical
to the committed v9 baselines. C approval qualifies the target; it is not
Rust signoff.

There are no engine, data or FFI-declaration changes. Rust starts from main
`8a7579d` (`4.2.6-impact-v9`).

## Unit kinds

Milestone 10 is not a bug-carry cycle. I-001 and I-002 change the Borg's
behaviour, so they are carried **only** to keep the Rust Borg's keypress
stream identical to the consumer's C Borg, and they are recorded here as
owner-directed improvement units rather than as upstream bug fixes. The
adoption contract's rule that an approved bug-fix milestone does not
authorize unrelated policy redesign is not engaged: these units are already
merged into the consumer's own product, so not carrying them would leave
Rust behind the accepted C target.

The consumer's ledger is unchanged by this cycle: `patches/series.json` still
holds its 98 entries and `patches/` its 98 patch files. The two improvement
units live in the consumer's separate `patches/improvements.json`.

## Reconciliation

[series.json](series.json) holds the two improvement units and the crash
correction, with the consumer's merge commits, acceptance evidence and
credit. [ledger.json](ledger.json) adds the Rust disposition. All three
units are carried:

| Unit | Correction | Rust site |
|---|---|---|
| I-001 one-blow kill | A lone adjacent known monster that one blow would kill does not justify a strategic retreat. Requires exactly one adjacent known kill, so a killable monster never masks a dangerous one | `borg_caution.rs`, `borg_fight_attack.rs` |
| I-002 flee-heal | `borg_caution()` tends wounds, poison and cuts while fleeing, when the grid is survivable this round (`pos_danger < BI_CURHP`). The flee goal itself is untouched | `borg_caution.rs` |
| 0099 crash correction | `square_in_bounds(cave, borg.c)` before the Necromancer lit-square test. **Already carried in borg-rs since milestone9**; the consumer has now adopted the same fix, so the C side needed a comment sync only | `borg_magic.c` (comment), Rust unchanged |

After the change the C Borg files equal the consumer blobs at `6944d88e8`,
with two exceptions:
- `borg-fight-attack.c` keeps local #322 (the stale-target comment and guard).
- `borg-caution.c`: the **functional** m10 code matches the consumer's
  exactly. The one difference is where the new
  `#include "borg-fight-attack.h"` sits: the consumer puts it immediately
  after `borg-caution.h`, borg-rs puts it in the local sorted include block
  after `borg-escape.h`. The placement has no behavioural effect, but it is
  a difference in the I-001 include hunk, not only "local history in the
  untouched regions". (Review finding M10-R-02, borg-rs#23; no C source
  change is needed.)

`borg-fight-attack.h` and `borg-magic.c` are byte-identical to the consumer's
blobs at `6944d88e8`.

### The 0099 crash correction

borg-rs carried this fix as a local crash correction in milestone9, after
the C SIGSEGV reported on
[better-borg-c#377](https://github.com/diegoami/better-borg-c/issues/377#issuecomment-6746360588).
The consumer has now adopted the identical guard in its own PR 398, so the
two trees agree. The only remaining difference was the comment above the
test, which borg-rs now takes from the consumer. The Rust side needed no
change: `engine::square_in_bounds()` already guards `borg_magic.rs`'s
lit-square test.

### I-001: the strategic-retreat guard only

The consumer's guard sits in the **strategic retreat** condition. The C
back-away condition, which shares the same guard clauses, does not carry it.

Rust factors the shared clauses into one helper, `may_move_away()`, used by
both the retreat and the back-away call. The Rust helper therefore takes a
`retreat: bool` that selects the I-001 face, so the back-away call keeps the
consumer's behaviour exactly. Getting this wrong would have widened the
change beyond the consumer's.

`borg_thrust_damage_one()` was `static` in C and is now exported, because
`borg_adjacent_kill_in_one_blow()` needs it from `borg-caution.c`. The Rust
equivalent was already a free function and only needed to become `pub`.

### I-002: the flee-heal branch

The consumer's branch is `else if (pos_danger < borg.trait[BI_CURHP])`,
placed after the existing `!borg.goal.fleeing` heal/defend block. It calls
the same `borg_heal(pos_danger)` with its own danger gates intact, so the
flee goal is untouched and only a survivable grid reaches it.

## Rust translation notes

- I-001: `borg_adjacent_kill_in_one_blow()` mirrors the C loop over
  `borg_kills[1..borg_kills_nxt]`. The C's `kill->r_idx >= z_info->r_max - 1`
  test is a signed `int` comparison in C (`r_max` is a `uint16_t` promoted to
  `int`); the Rust keeps it signed and handles the degenerate `r_max == 0`
  case the C's `-1` produces, instead of an unsigned underflow.
- I-001: `kill->power` is `int16_t` in C and `i16` in Rust; the comparison
  against the melee damage is widened on both sides.
- I-002: no translation difference. `borg_heal()` was already a faithful
  port, and the new branch calls it with the same argument.

## Removal units

Per-unit removal patches are in [removals](removals) (`git apply -R`),
three C units and two Rust units:
- `I-001-c.patch` / `I-001-rust.patch` (the helper, the guard, the exported
  `borg_thrust_damage_one`).
- `I-002-c.patch` / `I-002-rust.patch` (the flee-heal branch).
- `0099-comment-c.patch` (the comment sync above the lit-square test).

There is no Rust removal unit for the 0099 correction: the Rust guard was
already present and is unchanged.

The units were checked three ways:
- Each unit passes `git apply -R --check` on its own on the final tree.
- Applying all of them with `git apply -R` in a scratch worktree leaves
  `git diff 8a7579d -- angband/src borg-rs/src` empty.
- The implementation diff's hunks are partitioned across the units, with no
  hunk in two units and none left out.

## Native fixtures

[harness/adoption/m10/import.py](../../../harness/adoption/m10/import.py)
imports the two consumer fixtures from the milestone10 tag:
`necrolight-296-crash` (the 0099 crash correction, five modes) and
`fleeheal-408` (I-002, four modes). The consumer's kit keeps I-002's
scenario in its `tools/` tree and commits only the profile and the baseline
logs under `docs/audit/c-reports/fleeheal-408/`, so the import reads the
scenario from `tools/fleeing-heal-scenario.c` at the same pin.

I-001 has no native fixture: the consumer declares the absence, because its
guard never fires in any fixture (the consumer's own matrix shows zero row
changes). Its evidence is a targeted life and a paired cohort, both outside
the Rust lane's reach.

## Validation

See [the validation receipt](../../validation/2026-11-10-c-m10/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or acceptance.
