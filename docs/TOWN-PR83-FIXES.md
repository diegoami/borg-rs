# PR83 review corrections

This bundle addresses [Kimi’s changes-requested review](https://github.com/diegoami/borg-rs/pull/83#issuecomment-6892393453)
and the Necromancer/Ranger investigation of zoo run **43667787016**.
The base is `67750cf7763ab2d602ef6f8aaba1b39505d46c28` (runtime `dd3eeac`).
This is a new review candidate against `feature/town-inventory-strategy`.
PR83, its frozen experiment base, PR42 and the original zoo specimen remain unchanged.
Do not merge the historical review vehicles or change the frozen zoo pins.

## Review disposition

| Kimi finding | Correction | Focused verification |
| --- | --- | --- |
| 1. Urgent purchase loops with a full pack | Free a feasible pack slot before an urgent purchase; preflight the actual native one-copy carry; quarantine unchanged failed offers until inventory, gold or stock changes. Purchase history and purchase receipts require observed inventory/wallet progress. | Urgent action ordering and rejection-context tests; native full-pack fixture. |
| 2. Monster Confusion phantom buff | `ATT_CONF` supplies no combat utility. First-copy identification remains separate. | Effect classifier test; explicit ordinary-shop coverage exclusion. Tactical consumer remains issue #80. |
| 3. Different oil forecasts | Town and throwing use the shared trip forecast and `Journey::for_trip`, with explicit depth/duration overrides preserved. | Shared forecast/override and oil reservation tests. |
| 4. Projection fallbacks | Teleport Other includes `BOLT_STATUS`; explicit damaging/control whitelists replace generic attack/buff fallback. Darkness and Trap Creation supply neither. | Projection classification tests and native catalogue audit. |
| 5. Random alternatives added together | `RANDOM`/`SELECT` use the minimum shared capability across alternatives; subsequent sequential effects still add. This is a conservative service forecast, not an expectation. | Two-branch damage and incompatible-service tests. |
| 6. Partial decreases counted as full cures | Active deficits use native remaining timers. Full cures cover that timer; timed decreases cover only their actual reduction, including proportional native reductions. Cut and stun have separate appended capability IDs. | Multi-dose partial-repair test. |
| 7. Stacked rods | Readiness uses `ceil(timeout / cooldown)` charging rods, then follows native aggregate timeout decrement per charging rod, with at most one use per action. Average cooldown is a forecast assumption. | Short-horizon readiness tests and native charging fixture. |
| 8. Optimistic sales | A read-only native quote copies the exact offered quantity using `object_copy_amt`, then applies the real sale quote, charge splitting, Black Market adjustment, owner cap and no-selling option. Recorded proceeds feed replay. Live transactions still sell one and refresh. | Native charged-device, cap, Black Market and no-selling fixture. |
| 9. Urgent Home sources / full-pack joins | Known Home offers compete at zero purchase price in the urgent pass. Compatible withdrawals consume zero additional slots. Urgent Home stock is accounted before later valuations, and the same stack is reconsidered only after refresh. | Urgent Home selection, action mapping and full-pack withdrawal tests; native join fixture. |
| 10. Light/oil overlap | Equipment slot options join overlapping capability groups. Aggregate supply gets one marginal evaluation while each equipment slot still selects at most one item. | Light/oil overlap and slot-exclusivity test. |
| 11. Partial runes create new tasks | Learning tasks use opaque ownership identity, independent of combat descriptions and rune knowledge. It exposes no hidden kind, bonuses or generation metadata. | Native ranged-attack learning changes description while runes remain unknown and the production learning key stays equal. |
| 12. Identify sentinel / irrelevant opportunities | Separate finite Identify stock (`BI_AID % 1000`) from legal spells. More scrolls or gold do not reopen rune work already covered by the same spell; flavour work only reopens for a newly affordable flavour opportunity. | Opportunity regression tests. |
| 13. Sold Home reserves stay in baseline | Remove Home supplies from reserve curves immediately when planning their sale, including a sale that makes room for a pack deposit. | Reserve marginal regression test; revision-gated historical replay. |

## Class defects

An empty Necromancer light slot was mistaken for an expiring light. During
Recall the light helper consequently queued repeated rests before caution could
heal or escape. The narrow `iqty != 0` guard is mirrored in C and Rust.
See [the class investigation](TOWN-CLASS-INVESTIGATION-43667787016.md).

Ammo joining an existing quiver stack now uses native compatibility and recorded
quiver packing facts. Pack capacity uses the native forty-unit reservation bands,
including weighted throwing items. Overlapping ammo groups share reserved slack
once. Existing joins are bounded by compatible stack room; new ammo classes are
conservatively bounded to one quiver block and charged a slot. Live execution
buys one and refreshes. This fixes the missing quiver join without claiming
unbounded free ammunition storage.

## Records and limits

The initial correction candidate used **schema 9**; the current owner-requested Ranger calibration appends schema 10 (see below). `packing=[block,used]`, `quiver_classes=[class,room]`
and `available=[shop,slot,quantity]` freeze native capacity and rejected-offer facts
for engine-free replay. Kit flags add bit 3 for a whole-stack Home join. The
new disposition/order logic is revision-gated; schemas 4–8 retain their old model.
Historical records and original zoo results are never rewritten.

Opaque identity stays stable during partial learning and slot movement, but a
split/newly acquired object is a new task. Identities are process-local and are
not persisted across save/load; this is the existing in-memory suppression scope.
The mixed capability solver remains a bounded surrogate, not a global optimum.
Knowledge utility may still justify buying an otherwise unsupported unknown kind.
Issues #79/#80 cover additional emergency tactical use; this change does not
pretend those new consumers exist or close them.

See [validation](validation/2026-11-15-town-pr83-fixes/README.md) for the final
source/build identities, checks, native receipts and bounded gameplay results.
These checks cannot establish a fresh zoo survival benefit.

## Owner-requested Ranger calibration and Necromancer torch guard

The quiver correction alone did not establish adequate missile priority.
Rangers now also value a trip-relative usable-shot reserve; only compatible,
non-cursed missiles supply it. Oil/devices retain damage utility without
substituting for that reserve. The calibrated defaults are weight 250 and
50 planned actions per shot, exposed as `borg_town_w_ranger_missiles` and
`borg_town_ranger_missile_turns`; weight 0 disables the added preference.
Native unlight disables fuel value and excludes torch purchase candidates
before gear/learning scores. Other classes retain their existing attack policy.

[Complete calibration, actual shopping and validation](validation/2026-11-15-ranger-missiles/README.md)
records seven Ranger arms on eleven starts and seven Necromancer audits.
Selected arrows bought: 1616 versus 397 for the existing shopper; at the shared
elapsed-turn horizon every pair meets the purchase target. One early-death
case does not meet the surviving control's whole-run count. All seven
Necromancers bought zero torches; all observed torch offers were excluded.
This is calibration evidence, not survival acceptance: the selected arm
rescues two but newly kills two relative to generic utility. Full death and
resource observations remain visible. The initial correction validation above
belongs to runtime e263db7; the addendum's selected defaults are runtime 5d8511c.
