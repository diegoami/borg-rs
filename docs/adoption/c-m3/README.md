# better-borg-c milestone3 adoption

Owner direction, 2026-11-08: after the milestone2 carry
([Rust#6](https://github.com/diegoami/borg-rs/issues/6)), proceed to C
milestone3. Tracking: [Rust#8](https://github.com/diegoami/borg-rs/issues/8).

Target: better-borg-c runtime snapshot
`702f091f6ac06cae1014df2f85ac218be0189813` (patches 0008-0034, consumer issues
40-124), after the milestone2 target `2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f`.
C review: milestone3 [approved](https://github.com/diegoami/better-borg-c/issues/128#issuecomment-6705894431)
with evidence-durability/metadata findings (follow-up better-borg-c#130); package
`docs/measurement/IMPACT-MILESTONE3.md` there. C approval qualifies the target;
it is not Rust signoff or upstream incorporation. Engine base stays
`e473720ed88a0f0c7ca66815cef3000eb4a38bc3`; no engine, data or FFI change.

Rust starts from main `0bc7a09fb41a01cfbc4f5bb86b89002b074f3712` (milestone2
carried, independent review qualified pass, owner acceptance separate). The
milestone2 review follow-up on `harness-zoo-measurement` (another lane) is not
part of this batch.

## Reconciliation

[series.json](series.json) and [patches](patches) are the consumer's own
records for 0008-0034, byte-for-byte from `702f091f6`, with source/consumer
issues, commits, evidence pins and credit. [ledger.json](ledger.json) adds the
Rust disposition of every patch:

| Disposition | Patches |
|---|---|
| Carried in C and Rust (22) | 0008 shot path, 0009 per-spell bolt, 0010 glyph tracker, 0011 missile without target, 0012 Teleport Other skip, 0013 Dispel Life filter, 0014 arc angle, 0015 munchkin stair grid, 0016 Vampire Strike distance, 0017 slay traits, 0018 detection panel bounds, 0019 activation lookups, 0020 reserve-bolt veto, 0021 dispel reserve penalty, 0022 LOS-dispel arrival, 0023 Staff Holy radius, 0024 Blind Creature fear, 0028 glow bound, 0029 light modifier once, 0030 surrounded traps, 0031 home blows, 0032 lunal flags |
| C only; no Rust counterpart (3) | 0026, 0033, 0034: C allocation releases in the formula calculator. Rust's calculator owns `Vec` storage |
| Equivalent local correction retained (1) | 0027 = local #322 (`!= 1` thrust distance), C and Rust; not counted twice |
| Covered by local correction, not imported (1) | 0025: local #287 already bounds `borg_what_text()` by the caller's byte capacity (C/Rust); the consumer's `ABS(n) * MB_CUR_MAX` bound would conflict with that signature |

The changed C files equal the consumer blobs at `702f091f6`, except where the
retained local corrections live: `borg-fight-attack.c` keeps the local #322
text, `borg-io.c`/`borg-io.h`/`borg-messages.c`/`borg-think.c` keep the #287 signature,
`borg-update.c` keeps #295 and `borg-flow-glyph.c` keeps #345.

**Local #290 is superseded.** Patch0018 fixes the same source issue by
bounds-checking every queued detection mark and the "cackles" trap reset
against the current panel count. The earlier local correction instead reset
offsets equal to the cave size to 0, which turns a stale reaction into a mark
on the new cave's panel (0,0); the consumer's detect-290 `stale` fixture
asserts that nothing is marked. With both in place the fixture fails, so the
local equality reset is reverted to the pristine `>` test in C and Rust and the
consumer guard alone handles stale offsets. The four detection-array writes
that use the old offsets are all inside that guard. The patch0018 removal
units include this reversion. The original seed4006 2M crash case is part of
the parity matrix below.

## Rust translation

Each Rust change mirrors the C hunk in the same function; old Rust comments
that described the kept C bug are removed. Per-fix removal units are in
[removals](removals) (`NNNN-c.patch`, `NNNN-rust.patch`; `git apply -R`).
Each reverse-applies on its own on the final tree, and reversing all 47 in
sequence restores `0bc7a09` exactly for the changed C and Rust files.

Notes on non-mechanical translations:

- 0016: the C `dist` variable stays as the last scanned distance; Rust's loop
  local is scoped inside the loop because only `best_dist` is read after it.
- 0018: the guard is a helper `detect_panel_ok()` used by `borg_handle_self()`
  (all seven detection branches) and the "cackles" reset; clippy's preferred
  `q_x < panels.x` is the same test as C's `q_x + 1 <= panels.x`.
- 0017: `|=` on the integer slay multipliers is bitwise, exactly as C (a 2 and
  a 3 combine to 3); `BI_WK_*` are set to 1 (C `true`).
- 0028: `glow_n` is compared with `AUTO_LIGHT_MAX` before the store; before the
  fix Rust panicked where C wrote out of bounds.

## Validation

See [the validation receipt](../../validation/2026-11-08-c-m3/README.md) and
[harness/adoption/m3](../../../harness/adoption/m3/README.md).

Not claimed: a population survival or performance benefit, whole-private-Borg
equivalence, or Rust milestone acceptance. Owner acceptance and any independent
review are recorded on Rust#8.
