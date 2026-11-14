# Town capability coverage repair

The initial live utility model removed consumable inventory-power bonuses but
replaced only a subset of the capabilities they represented. Its catalogue
already listed missing ammunition, devices, books and fuel, but those omissions
were not made a release gate. That was the implementation and validation error;
no parameter tuning could repair it.

The replacement uses one physical supply model for purchases, pack retention,
Home storage and withdrawals. It starts from the combined trip/town runtime
`d9a56f4`; it does not modify that frozen candidate or its zoo studies. Live
execution remains opt-in (`borg_town_plan = 1`), and the shared trip forecast
continues to come from the trip policy when enabled.

## Capabilities

- Ammunition supplies expected ranged damage, only for a matching launcher.
  Existing quiver stock counts. Oil supplies thrown damage and, when carrying a
  lantern, fuel. Attack supplies share coverage with offensive devices.
- Known devices supply their native effects using actual charges or expected
  rod uses from the trip duration and native cooldown. Teleport Other and
  Stone to Mud are services, not damage; monster-healing/haste/clone/poly devices
  get no attack reward.
- Class books supply spell access when a spell is usable at the current level
  and maximum mana. An owned book closes the access requirement; copies cannot
  unlock the same spell access repeatedly.
- Native effects cover stronger escapes, cures, mana, restoration, curse
  removal, detection, mapping, illumination, temporary resistance, speed,
  combat preparation/control, enchantment, recharging, glyphs and banishment.
  Multi-effect supplies retain their secondary cures. Repair priorities depend
  on a relevant current deficit; mana has no value to a non-caster.
- Food uses native nutrition rather than a ten-kind whitelist. Harmless
  `CRUNCH` messages do not disqualify Hard Biscuits. Whisky's confusion does
  disqualify it as safe food. Fuel uses remaining lifetime, including the worn
  light, and a permanent light suppresses fuel utility.
- Reliable usable information/food/Recall spells suppress redundant purchases
  for those services. Mana-free attack and healing supplies remain valuable
  alongside spells. Unknown carried flavours are not valued using hidden kinds.

Positive supply coverage has decreasing marginal returns. One-time access and
repairs saturate when the actual requirement is met; these are not stock quotas.
The solver merges overlapping capability groups, keeping pure options and a
bounded mixed path. This remains an explicitly approximate option set, not a
claim of a globally optimal kit. Gold, space, weight and Home-delay costs still
apply. The diagnostic catalogue includes possible Black Market kinds, not a
promise every kind can appear or that every special effect is purchased.

## Calibration

Existing trip, risk, gold and room parameters still apply. Six added settings:

| Setting | Default | Meaning |
| --- | ---: | --- |
| `borg_town_w_attack` | 100 | Ranged damage coverage priority |
| `borg_town_w_magic` | 100 | Class-book access and mana priority |
| `borg_town_w_support` | 50 | Detection, tools and combat support priority |
| `borg_town_w_restore` | 100 | Relevant restoration and stat-growth priority |
| `borg_town_attack_turns` | 100 | Forecast actions per ranged attack opportunity |
| `borg_town_support_turns` | 1000 | Forecast actions per support-service opportunity |

These rates are provisional and calibratable, not measured survival estimates.
Class differences come from the actual launcher, usable spells, class books,
mana, resistances and condition deficits, rather than nine separate stock lists.
Longer/deeper trips raise exposure; replenishment loses value as coverage grows.

## Records and the coverage gate

Schema 7 records contain `capability_curves` and `capability_supplies`: evaluated
weights/demands, carried/Home baselines and per-copy engine facts. Replay uses
those facts without loading game data. Historical schema 4/5 decisions retain
their original model. Native effective settings report the six new parameters.

`BORG_TOWN_CAPABILITY_AUDIT=<file>` writes one engine-backed catalogue per
process at the first town evaluation: hypothetical known single items, their
native supply vectors and marginal values in the current class context. It does
not insert items into the game or claim purchases. Coverage checks join this
with ordinary shop stock and require a capability, gear evaluation or an
explicit contextual exclusion. Actual purchases require native retail receipts
and wallet changes; neither this diagnostic nor a planner record is a receipt.

See [the item catalogue](TOWN-ITEM-CATALOGUE.md). Validation results and known
limits belong in the accompanying dated receipt before this is promoted.

The opt-in throwing path also reserves the last oil only for a fuel-burning
lantern. The control retains its original gate, which could otherwise prevent
throwing bought oil with a torch and no spare fuel. Device replenishment and
recharging remain separate: recharging utility requires a depleted, still-carried
usable device; selling or storing that target removes the demand. Empty devices
are not automatically refurbished by this policy.
