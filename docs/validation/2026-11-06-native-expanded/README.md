# Expanded native C/Rust checker (M4 E)

`harness/native-checker/run.sh` now verifies real C/Rust calculator,
cave/projection/direction, flow, physical melee, target selection and
rune-builder code. Production items are compiled with source/item hash
ledgers; C entry points are renamed and linked beside Rust. Fixture
constructors and engine callback data are explicit adapters. See the
checker README for the precise state footprint and bounded input domain.

Completed controls (`DONE`, logs and SHA256SUMS retained here):

* 30,000 calculator cases, including logical token/calculation state.
* 4,000 spatial/combat states over two seeds, including town/dungeon bounds,
  all grid fields, staged cost/hard/known/icky/current-flow arrays, full
  circular queues, targeting, physical melee scores, chosen goals,
  simulation/apply behavior and ordered callbacks/notes/keys.
* 900 glyph-builder states over nine creation/duplicate/pattern/no-room/
  distant-center/capacity scenarios, with complete tracking/map/flow state.
* Six actual Rust source faults detected: arithmetic, LOS result, unused
  border-map flag, same-square eligibility, physical multiplier and glyph
  bookkeeping. Overrides cannot produce valid PASS evidence.
* Non-recovering C UBSan:30,000 calculator,2,000 spatial and900 glyph states.
* All-target release clippy with warnings denied passes.

The logical engine environment is shared: dimensions/predicates, race/item
inputs, danger data, and controlled glyph item/spell/follow-step outcomes.
This is not whole-Borg shadow execution or proof of every engine callback.
Direct engine/game evidence remains the separately checked full corpus and
serialized capture equality. The glyph fixture does not establish a live
Sea of Runes/Morgoth strategy or a win. The separate source lead about
optimized reverse flow is reported in c-bugs section56, without changing
C or Rust gameplay. Production inputs remain unchanged from28b8170.

Reproduce: `CHECK_UBSAN=1 harness/native-checker/run.sh`.
