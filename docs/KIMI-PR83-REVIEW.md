## Verdict: **changes requested**

The candidate is not ready for claims of complete usable valuation or live-policy promotion. I confirmed the two disclosed defects and found additional correctness problems in execution, capability accounting, and learning-return suppression. Main/M13 integration remains untested.

Reviewed runtime candidate `dd3eeac646e097bcdd7c289c092e91400896a713`. PR head `67750cf7763ab2d602ef6f8aaba1b39505d46c28` changes only review documentation and diagnostics.

## Findings

### 1. [P2] A full-pack urgent purchase can repeatedly fail without reaching room-freeing dispositions

`borg-rs/src/borg_town_record.rs:885-889` emits the urgent shop purchase before every sale, drop, store, or Home withdrawal. The urgent budget is constrained only by gold (`borg-rs/src/borg_town_budget.rs:27-30`), and purchases receive no pack-room preflight in `borg-rs/src/borg_town_live.rs:107-136`.

With a full pack and an urgent item requiring a new stack, native code rejects the purchase at `angband/src/ui-store.c:671-675,701-705`. Nevertheless, `buy_selected` clears the goal and returns success unconditionally at `borg-rs/src/borg_store_buy.rs:1469-1480`. The unchanged state can therefore produce the same purchase again until the broad anti-bounce/stay guards intervene.

**Remedy:** perform room-freeing dispositions before an urgent purchase that cannot fit, preflight carrying capacity, and treat unchanged inventory/wallet state as a rejected transaction rather than completion.

### 2. [P2] Monster Confusion receives phantom combat value without a tactical consumer

`borg-rs/src/borg_town_capability.rs:147-177` maps `TMD_ATT_CONF` to `BUFF` and converts its magnitude into timed combat supply. The scroll’s 10,000-turn arming timer consequently becomes 10,000 combat-buff turns.

There is no tactical scroll-reading consumer for this effect. Native melee clears the armed status on the next applicable hit at `angband/src/player-attack.c:669-676`; it is not 10,000 turns of protection.

Fresh engine-backed audits reported `10,000,000` scaled buff units and positive gain. Across ten fresh enabled-policy games, six bought the scroll and none read it.

**Remedy:** remove this combat valuation until an enabled consumer exists. If implemented, value one armed melee effect using its actual confusion magnitude, separately from first-copy identification value.

### 3. [P2] Throwing and town allocation reserve oil against different trips

Town evaluation materializes the effective trip target at `borg-rs/src/borg_town_shadow.rs:336-343,411-436`. The throwing guard independently calls `Journey::of(st)` on the raw configuration at `borg-rs/src/borg_fight_attack.rs:2423-2434`.

I applied the disclosed temporary probe and reproduced:

- throwing forecast: **1,200 turns, zero reserved bottles**
- town forecast: **12,200 turns, one reserved bottle**

Thus throwing can release oil that the same plan allocated to lantern fuel. The diagnostic patch was reversed afterward.

**Remedy:** derive both decisions from one shared effective `Journey`, preserving explicit overrides, off-policy behavior, and replay inputs.

### 4. [P2] Projection fallbacks misclassify escape services and harmful effects

At `borg-rs/src/borg_town_capability.rs:194-203`:

- Wands and rods of Teleport Other use `EF_BOLT_STATUS:AWAY_ALL`, but that effect type is omitted from the `OTHER` branch. They receive unrelated `BUFF` supply despite real tactical consumers at `borg-rs/src/borg_fight_defend.rs:1187-1193,1315-1325`.
- Scrolls/Staffs of Darkness receive positive `ATTACK` supply although `SPHERE:DARK_WEAK` is non-damaging.
- Scrolls of Trap Creation receive positive `BUFF` supply through the generic `TOUCH` fallback.

Fresh native audit output confirmed all three mappings.

**Remedy:** include `EF_BOLT_STATUS` in the Teleport Other service mapping, and replace permissive attack/buff fallbacks with explicit damaging-effect and supported-status whitelists.

### 5. [P2] `RANDOM`/`SELECT` alternatives are summed as though every branch occurs

`engine::effect_values` flattens the linked effect chain at `borg-rs/src/engine.rs:2313-2335`, and `effect_supply` sums every recognized branch. Native execution selects one branch at `angband/src/effects.c:403-468`.

Examples:

- Potion of Dragon Breath selects one 80-damage branch but is valued as **160 attack**.
- Wand of Dragon’s Breath selects one of five 160-damage branches but can be valued as **800 attack per charge**.

The fresh audit reported exactly 160 attack for the potion.

**Remedy:** make effect-chain evaluation branch-aware. Use an explicit expectation or conservative value; do not sum mutually exclusive alternatives.

### 6. [P2] Partial timed decreases close full condition-repair requirements

`EF_TIMED_DEC` and full `EF_CURE` both produce one complete cure unit at `borg-rs/src/borg_town_capability.rs:171-175`. Negative one-time repair demands at `:323-324,346-348` therefore allow one partial reduction to satisfy the whole deficit.

For example, Cure Light Wounds reduces cut and confusion by only 20 turns (`angband/lib/gamedata/object.txt:3156-3173`; `angband/src/effect-handler-general.c:682-694`). A timer above 20 remains active, but another dose can receive zero marginal repair value.

**Remedy:** distinguish full cures from timed reductions and value the actual reduction against the remaining timer.

### 7. [P2] Stacked-rod readiness invents or erases available uses

The formula at `borg-rs/src/borg_town_capability.rs:284-292` averages stack timeout with:

```text
ready_after = timeout / quantity
```

Native semantics instead use `ceil(timeout / cooldown)` charging rods (`angband/src/obj-util.c:1020-1038`).

For two rods with 40-turn cooldown and combined timeout 40, one rod is ready:

- journey 10: the model reports zero uses, erasing the ready rod;
- journey 20: stack accounting can report two uses although only one is ready.

**Remedy:** calculate initial readiness from the number of charging rods, then forecast uses from that readiness and the cooldown schedule.

### 8. [P2] Sale budgets can spend more than native shops actually pay

`borg-rs/src/borg_town_shadow.rs:197-225` estimates proceeds as `value × quantity × 2/3`, explicitly omitting Black Market adjustment and native owner caps. Those proceeds enter spendable gold throughout `borg-rs/src/borg_town_utility.rs:261,312,320,348`.

Native sales halve Black Market proceeds and apply the proprietor’s per-copy cap at `angband/src/store.c:616-665`. With selling enabled, the solver can therefore justify dispositions and purchases using cash that never arrives.

**Remedy:** obtain a read-only native sale quote for the planned quantity and record that quote for engine-free replay.

### 9. [P2] Home supplies cannot reliably satisfy urgent needs before paid purchases

Two paths compound:

- Urgency considers only carried resources (`borg-rs/src/borg_town_usefulness.rs:120-133`), and urgent shop purchases precede ordinary Home withdrawals (`borg-rs/src/borg_town_record.rs:885-916`). A known Home cure is therefore bypassed for an unnecessary shop purchase. The PR discloses this source-pass gap; I confirmed it.
- Home withdrawals require `slots > 0` at `borg-rs/src/borg_town_utility.rs:325-338`, even when an existing compatible pack stack could absorb the item. The live guard explicitly recognizes that native possibility at `borg-rs/src/borg_town_live.rs:115-117`.

With no empty slot and no purse, this can leave a need unmet despite a free, feasible Home withdrawal.

**Remedy:** include Home sources in urgent planning and account for withdrawals that join existing stacks.

### 10. [P2] Wearable light and oil can claim the same light shortage independently

Wearable values include capability utility at `borg-rs/src/borg_town_record.rs:781-799`, but every equipment-slot group bypasses capability-group merging at `borg-rs/src/borg_town_capability.rs:445-446`.

A fueled light source and oil can consequently each be valued against the same initial `LIGHT` deficit, and the solver sums both gains instead of evaluating their aggregate coverage.

**Remedy:** merge overlapping slot-group capabilities while preserving the one-item-per-equipment-slot constraint, or evaluate those combinations against the shared light curve.

### 11. [P2] Partial rune learning makes an unchanged stack look like a new learning backlog

The learning key uses `ODESC_COMBAT` at `borg-rs/src/engine.rs:1781-1785`. Combat descriptions change as to-hit, to-damage, or AC knowledge becomes known (`angband/src/obj-desc.c:392-427`).

At `borg-rs/src/borg_trip.rs:156-167`, the changed key no longer matches the remembered unresolved item, so suppression is bypassed. Ordinary ranged attacks can cause exactly this partial learning (`angband/src/obj-knowledge.c:1940-1949`) while other runes remain unresolved.

Thus the same stack can trigger another return without a new stack or new town opportunity.

**Remedy:** use a stable player-visible task identity that does not change with partial combat-knowledge acquisition, with a native partial-rune regression test.

### 12. [P3] The Identify Rune spell sentinel falsely looks like a newly acquired resource

`BI_AID` includes a sentinel 1000 whenever Identify Rune is legal (`borg-rs/src/borg_trait.rs:2214-2217`). `TripState::new_opportunity` instead treats `BI_AID` as a comparable resource count at `borg-rs/src/borg_trip.rs:67-70`.

Departing with the spell and later picking up one scroll changes 1000 to 1001, reopening previously suppressed tasks despite the spell already providing identification. The resulting global flag is applied across old tasks at `:79-83,149-167`, including tasks the scroll cannot help.

**Remedy:** track actual scroll count separately from spell availability and reopen only tasks for which the new mechanism is relevant.

### 13. [P3] Sold Home supplies remain in the reserve model

When a Home “spare” is sold to free a deposit slot, `borg-rs/src/borg_town_utility.rs:307-314` does not remove its capability or legacy Home supply. Ordinary Home sales at `:345-349` have the same omission.

Later Home decisions still read those reserves at `:331-335`, potentially allowing another substitute to be sold or moved because the already-sold stack still contributes coverage.

**Remedy:** apply `change_home` and update `u.home` for every Home sale before evaluating subsequent dispositions.

## Unpromoted source leads

I did not promote these without a discriminating reproduction:

- possible false matches in the Home withdrawal’s approximate unknown-item stacking guard;
- incomplete try-on/try-off restoration of `st.home` or `st.magic`;
- whether every credited class book has an enabled tactical spell consumer;
- stale purchase quotes and possible sell/rebuy loops;
- hidden-`tval` influence on learning keys;
- finite identification resources being counted once per unresolved stack.

No additional concrete dice-FFI, RNG, input-queue, or schema-replay defect was established.

## Verification performed

At the unmodified candidate:

- **271 Rust tests passed**, four opt-in tests ignored.
- Clippy passed with `-D warnings`.
- **24 town-analysis** and **six trip-report** Python tests passed.
- Rust and C native builds succeeded.
- Nine native class contexts passed the 104-kind coverage gate.
- **238 fresh schema-8 coverage records** replayed exactly.
- **110 fresh schema-8 live-learning records** replayed exactly.
- **1,760 retained schema-7 records** replayed exactly.
- A 20,000-turn default run had identical complete key/message/state streams:
  - recording off versus on;
  - Rust versus separately built C control.
- A 30,000-turn enabled-policy seed-101 run had identical recorded/unrecorded streams and produced 12 unknown-return decisions.
- The disabled-shop seed-301 run produced zero unknown-return decisions; its streams matched the author receipt’s hashes.
- Regenerating the retained transaction report confirmed **530 purchases, 530 wallet checks, zero unconfirmed purchase commands**.
- The separate fuel probe failed as expected and was restored.
- The isolated worktree remains clean; no production files were changed.

The original `/tmp/trip-integrated-validation` files were unavailable, but the fresh seed-101 and disabled-shop runs reproduced their recorded stream hashes. The nine-start capability comparison still shows **two deaths versus one baseline death**, so neither survival improvement nor calibrated priorities are established.
