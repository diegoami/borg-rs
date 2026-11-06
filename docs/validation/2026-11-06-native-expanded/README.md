# Expanded native C/Rust checker (M4 E)

`harness/native-checker/run.sh` now verifies real C/Rust calculator,
cave/projection/direction, flow, physical melee, target selection and
rune-builder and lunal-thinker code. Production items are compiled with source/item hash
ledgers; C entry points are renamed and linked beside Rust. Fixture
constructors and engine callback data are explicit adapters. See the
checker README for the precise state footprint and bounded input domain.

Completed controls (`DONE`, logs and SHA256SUMS retained here):

* 30,000 calculator cases, including logical token/calculation state.
* 4,000 spatial/combat states over two seeds, including town/dungeon bounds,
  all grid fields, staged cost/hard/known/icky/current-flow arrays, full
  circular queues, targeting, physical melee scores, chosen goals,
  simulation/apply behavior and ordered callbacks/notes/keys.
* 1,600 glyph-builder states over16 creation/duplicate/pattern/no-room/
  distant-center/capacity scenarios, with complete tracking/map/flow state.
* 1,280 lunal-thinker states over40 exit/recovery/stair/flag scenarios,
  including track capacity/duplicates, forced descent and distinct
  object/fleeing flow results.
* Seven actual Rust source faults detected: arithmetic, LOS result, unused
  border-map flag, same-square eligibility, physical multiplier and glyph
  bookkeeping and lunal exit flags. Overrides cannot produce valid PASS evidence.
* Non-recovering C UBSan:30,000 calculator,2,000 spatial,1,600 glyph and1,280 lunal states.
* All-target release clippy with warnings denied passes.

The logical engine environment is shared: dimensions/predicates, race/item
inputs, danger data, and controlled glyph item/spell/follow-step outcomes.
This is not whole-Borg shadow execution or proof of every engine callback.
Direct engine/game evidence remains the separately checked full corpus and
serialized capture equality. The glyph and lunal fixtures do not establish live
Sea of Runes/Morgoth strategy, actual-game self-lunal entry or a win. The separate source lead about
optimized reverse flow is reported in c-bugs section56, without changing
C or Rust gameplay. Production inputs remain unchanged from28b8170.

Reproduce: `CHECK_UBSAN=1 harness/native-checker/run.sh`.

Supplemental native Rust coverage: physical melee84/84 lines, thrust52/52,
spread73/73, glyph121/124 (97.58%) and lunal154/157 (98.09%). Measured gaps
prompted the additional outlying/failed creation/failed commit/failed step
and lunal fleeing-flow scenarios. The previous glyph/lunal measurements
were101/124 and146/157 lines. See `native-coverage/README.md` for scope,
raw LLVM export, five retained positive profiles and function counts.
These numbers are not merged with game-corpus F coverage.
