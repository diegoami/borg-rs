# Trip policy integrated with the finished inventory model

Tested runtime: `1651f808219e`, built on finished inventory `d489117`.
The capability model and its calibration settings remain in place. No zoo
pin or policy default changed.

Every carried unknown item type can now justify a learning return, including
food, mushrooms, equipment and quiver ammunition. Allowed sales can teach
consumable flavours even without gold. Unknown consumables with no buyer are
kept or stored. Home storage does not identify items. Unknown reusable gear
and devices remain protected rather than being sacrificed just for learning.

The last-town snapshot prevents unchanged unresolved stacks from triggering
repeated returns on their own. New unknowns, more identification resources,
a newly legal spell or enough money for an observed offer can reopen the
reason. New unknowns may justify an exploratory shopping visit. The dungeon
never reads fresh hidden stock; cached prices may be stale. Cache state is
process-local and is not persisted in saves.

## Validation

- 271 Rust tests pass, four opt-in tests ignored; clippy passes.
- Six trip-report tests and 24 town-analysis tests pass.
- Native coverage still accounts for all 104 ordinary shop kinds across nine
  classes, with the inventory branch's contextual exclusions.
- Both builds succeed; all three 20,000-turn C/Rust controls have identical
  key, message and native state streams.
- Three 30,000-turn Human Warrior starts (101, 301, 401) enable both policies
  and lower the unknown threshold to one. All reach the cap. They produce
  12, four and five unknown-return decisions respectively, including unknown
  consumables with zero unknown gear. These are policy votes, not independent
  visits or lives.
- Seed 101 matches complete keys/messages/state with recording enabled and
  disabled. A disabled-shop seed-301 case reaches the cap with zero exploratory
  tasks and zero unknown-return decisions.
- 570 new schema-8 plan records replay exactly. 275 historical schema-5 records
  replay exactly; a focused regression preserves schema-7's old no-buyer
  decision. Schema 8 retains capability facts but protects unknown consumables
  without buyers. Trip records are v3, while the report still accepts v2.

All-family collection and unchanged-backlog suppression are covered by pure
regressions. The natural gameplay traces exercise potions and scrolls; they
are not evidence of a naturally found mushroom sale or improved survival.
These are integration checks, not parameter selection. Frozen zoo results
still measure d9a56f4.

[Per-game receipt and build identities](receipt.json) · [Coverage](coverage.json) ·
[Parity](parity.log) · [Rust tests](rust-tests.log) · [Clippy](clippy.log) ·
[Historical replay](historical-replay.log) · [Current replay](current-replay.log).

Raw outputs: `/tmp/trip-integrated-validation`. Twenty games: nine coverage,
six controls, four live including the recording pair, and one disabled-shop
case. Two post-run bookkeeping errors (a glob matching log files and a field
absent from C summaries) were corrected against original outputs; no games
were rerun because of them.
