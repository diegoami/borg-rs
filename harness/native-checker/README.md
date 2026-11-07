# Native C/Rust differential checks (M4 E)

```sh
harness/native-checker/run.sh
CHECK_UBSAN=1 harness/native-checker/run.sh
```

The standalone checker links original native C code beside production Rust
items, compares logical state and outputs, and retains source SHA-256
ledgers. It needs a native C compiler and stable Rust; optional C UBSan
requires the compiler's runtime. It has its own Cargo workspace and never
links C Borg code into the production Rust Borg.

| kernel | native code and Rust code | comparison |
|---|---|---|
| Calculator | actual renamed C calculator, real C array helper, unchanged Rust module by path | results/index, diagnostic order/text, complete logical calculation list and every token kind/negation/depth/payload |
| Projection/cave/directions | original C projection/cave helpers, verbatim Rust items and distance helper | LOS/projection decisions, incremental paths, direction choice, wall guesses, target state and emitted keys/notes |
| Flow | original renamed C flow functions, verbatim Rust flow state/helpers | full cost/hard/known/icky/current-flow matrices, circular queue bytes/head/tail, goal type and wipe flag after clear, enqueue and spread/commit; ordered danger calls |
| Melee | actual C/Rust physical-damage and thrust-selection bodies | slays/brands/immunities, armor/hit chance, HP cap, class/mana and unique/summoner/questor arithmetic; scores, chosen goals, simulation/apply behavior, callbacks and actions |
| Lunal thinker | original C/Rust thinker bodies and stair helpers | town/weak/self-lunal exits, recovery gates, stair tracking and choice, fleeing/less/lunal flags, forced descent, callbacks/notes/keys |
| Rune builder | actual C/Rust glyph-builder bodies with real flow calculations | center/outlying decisions, creation/duplicate/full-capacity tracking, centers/goals, complete maps/queues and ordered spell/item/step boundary calls |

C modules are compiled with renamed entry points. The static C melee bodies
and Rust items are selected verbatim by the source extractor; only a fixture
entry name is changed to keep the physical scorer distinct from the shared
score-input control. The ledger binds each item and original file to hashes.
Fixture-only constructors and type/environment adapters supply dimensions,
engine predicates, item/race data and callback outcomes. Numerical engine
flag bindings are also compared directly. Production algorithms are not
reimplemented as expected-value functions.

The environment boundary is explicit. Flow receives a shared danger grid;
selection is tested with shared damage inputs and with the actual physical
scorer. Rune spell/item/step calls receive controlled outcomes; their
bookkeeping does not establish that a tactic completed inside the engine.
The checker does not shadow every private field of an entire running Borg.
Game-corpus oracles and serialized capture equality remain separate evidence.

## Corpus and controls

* Calculator:30,000 deterministic cases, seed1511464998; batches of47
  exercise array growth/reuse. At most four bounded operands avoid int32
  overflow. Operators, precedence, negative/negated values, parentheses,
  ranges and divide-by-zero diagnostics are compared.
* Spatial/combat:2,000 states each for seeds1832132646 and305441741;
  dungeon/town bounds, open/sparse/corridor/unknown terrain, traps, glyphs,
  monsters, health/fear/status thresholds, stairs, optimization/tunneling/
  avoidance/sneaking, and queue wrap/overflow. Snapshots include all map
  cells and queue slots, including inactive ones and intermediate stages.
* Rune builder after review#1:2,600 executions =26 hand-built layouts ×100
  packet/environment variants per layout, not2,600 independently generated
  layouts. M4 had16 layouts ×100. Added distance49/50/51 arrival boundaries,
  equal-score choices for both new-center/outlying ranking branches and
  unequal-score controls. Cases23-25 additionally exercise candidate costs
  74/75/76: only74 is accepted. Whole declared state is compared in each execution.
  Candidate-cost cases19-25 use a shared, explicit adapter: after real reverse
  flow, replace the cost plane with only the declared ranking/cutoff candidates
  and origin reachable. This keeps the selection contracts independent of
  reachability. Other cases use real reverse flow, now corrected under #345.
  These cases test the production ranking body under a
  controlled environment, not naturally reachable engine tie outcomes.
* Lunal thinker:1,280 controlled states over40 exit, recovery, self-scumming,
  stair capacity/duplicate, climb/dive and forced-descend scenarios. Shared
  environment callbacks supply preparedness, light, rest, recovery and flow
  outcomes (including separate object/fleeing flow results); the production
  decision/state-transition code is unchanged.
* Source faults must be rejected: actual Rust calculator addition,
  projection adjacency result, unused border-map flag, same-square melee
  guard, damage multiplier, glyph target bookkeeping and self-lunal flag reset
  are changed in temporary copies. Three review follow-up controls additionally
  mutate the distance50 boundary, both tie comparisons and the old-center tie
  comparison alone. Cost75 and the #345 reverse-flow regression are additional
  controls, twelve total. Glyph
  controls request the full2,600-execution corpus (and fail as soon as a
  mismatch is found); other controls retain their bounded subsets.
  Overrides cannot produce valid PASS evidence.
* Optional non-recovering C UBSan runs calculator, spatial/combat and
  glyph and lunal corpora. All-target clippy runs with warnings denied.

A fresh `harness/out/native-checker.*` directory records logs, hashes and
ledgers. DONE is written only after all requested checks and controls pass.
No production input is changed. Arbitrary malformed states and undefined
C arithmetic are outside the declared domain; valid tracked counts are
0..200 and all map accesses use allocated pinned-size storage.

Allocator leads #323–#325 and the separately documented glyph reverse-flow
lead remain reported findings. This checker establishes C/Rust equivalence
for the stated kernels/domain, not correctness of every inherited C tactic. The controlled lunal entry does
not claim that an ordinary-game source trace entered self-lunal mode.

Supplemental native-body coverage in the expanded validation receipt reports
physical melee84/84 lines, thrust52/52, spread73/73, glyph121/124 and
lunal154/157. It uses the same positive C/Rust comparisons under Rust
instrumentation. This is separate from the game-corpus F measurement;
C coverage and branch coverage are unmeasured. The raw LLVM export and
selected-body counts make remaining gaps reviewable.
Those percentages describe the tagged M4 checker corpus. No updated body
coverage percentage is inferred from the later review-follow-up scenarios.
