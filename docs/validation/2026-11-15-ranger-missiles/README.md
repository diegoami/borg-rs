# Ranger missile calibration and Necromancer torch suitability

Owner request: give Ranger missiles substantially higher utility, calibrate purchases against the existing shopper, and prevent Necromancers buying torches. This is a bounded local calibration, not a fresh zoo efficacy experiment.

Selected settings:

```text
borg_town_w_ranger_missiles = 250
borg_town_ranger_missile_turns = 50
```

Only Rangers receive this extra utility. Each known, non-cursed missile matching the current launcher supplies one usable shot. Existing pack/quiver/Home stock lowers marginal demand; oil, devices and spells cannot fill this reserve. Demand follows planned trip duration / interval, at least one shot; weight follows trip exposure. Weight 0 restores generic attack valuation. The interval is an explicit policy estimate, held at 50 in this weight sweep. Price, gold, carry capacity, healing and escape still constrain buying.

Native PF_UNLIGHT disables fuel utility and excludes torches before learning/equipment scores, including urgent Home candidates. Other equipment lights may have real gear value. This does not change birth kit or dungeon light handling.

## Runs and selection

Seven arms on the same 11 Ranger starts, 100000 world-turn cap each: existing shopper/returns; generic utility/trip with reserve disabled; reserve weights 125, 250, 500, 1000, 4000 with interval 50. Plus seven reused Necromancer live-policy starts: **84 executions**. Six Ranger starts were selected during the earlier zoo death investigation. Human seed 508 was reused; 509–512 were not used for Ranger in the previous matrix. All are calibration data after inspection, not held-out confirmation. Initial risk 1000, no buy-to-identify executor, first-copy knowledge weight 25.

Selection after inspecting the complete sweep: 250 is the smallest tested positive weight reaching at least the existing shopper's whole-run arrow purchases in 10 of 11 pairs; 125 reaches that in eight. Weights 500/1000/4000 also reach ten but each has five deaths; 250 has three. This is a transparent post-hoc calibration choice and cannot establish a survival benefit. The existing shopper bought arrows only after the first descent because it has starting ammunition; zero early purchases does not mean an empty quiver.

| Policy / reserve weight | Bought arrows | Bought before first DL1 | Wound cures bought | Phase Door bought | Deaths | Mean CL | Mean max DL | Reached DL3+ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| existing | 397 | 0 | 28 | 134 | 5 | 5.00 | 5.27 | 4 |
| generic | 1025 | 15 | 59 | 80 | 3 | 5.45 | 2.55 | 4 |
| missiles125 | 883 | 47 | 33 | 34 | 6 | 4.73 | 2.55 | 3 |
| missiles250 | 1616 | 62 | 36 | 50 | 3 | 5.55 | 2.82 | 5 |
| missiles500 | 1833 | 73 | 34 | 60 | 5 | 4.36 | 2.09 | 3 |
| missiles1000 | 1963 | 76 | 38 | 38 | 5 | 4.91 | 2.64 | 5 |
| missiles4000 | 2949 | 91 | 38 | 46 | 5 | 4.36 | 1.91 | 2 |

The selected arm buys **1616 versus 397** arrows over the full runs, and at least as many at the shared elapsed-world-turn horizon in **11/11 pairs**. Whole-run purchases reach the target in **10/11**: seed 285192323 dies early with 26 bought arrows versus the surviving existing shopper's 68. By that death turn the existing shopper had bought zero. This is not a guarantee of matching every future run or of improved survival.

Phase Door purchases fall from 134 (existing) / 80 (generic utility) to 50; wound-cure purchases are 36 versus 28 / 59. The selected arm still has two new deaths relative to generic utility. These allocation and tactical risks must remain visible before promotion.

| Seed | Existing arrows | Selected arrows | Shared turn horizon | Existing / selected arrows by horizon | Existing / selected death |
| --- | ---: | ---: | ---: | --- | --- |
| 1632766293 | 0 | 163 | 100001 | 0 / 163 | 0 / 0 |
| 285192323 | 68 | 26 | 2225 | 0 / 26 | 0 / 1 |
| 1401471427 | 177 | 251 | 100005 | 177 / 251 | 0 / 0 |
| 1770524781 | 0 | 175 | 23425 | 0 / 81 | 1 / 0 |
| 1021027760 | 60 | 182 | 100004 | 60 / 182 | 0 / 0 |
| 763715166 | 0 | 34 | 3854 | 0 / 34 | 1 / 1 |
| 508 | 23 | 98 | 100001 | 23 / 98 | 0 / 0 |
| 509 | 0 | 132 | 31494 | 0 / 65 | 1 / 0 |
| 510 | 69 | 254 | 98040 | 69 / 254 | 1 / 0 |
| 511 | 0 | 190 | 80371 | 0 / 190 | 1 / 1 |
| 512 | 0 | 111 | 100002 | 0 / 111 | 0 / 0 |

## What was bought and what died

[Actual selected-policy shopping tables](shopping.md) · [all confirmed transactions](transactions.csv) · [per-run identity/outcomes](runs.json) · [all 33 calibration deaths](deaths.md). Native retail receipt and wallet verification are required; no planned purchase is counted. Initial ledger generation missed three Home deposits because shop navigation context was stale; the parser now uses the live executor's explicit shop, with a regression test. All three native deposits are recovered; zero unmatched commands remain. Arrow counts never depended on that correction.

Seven Necromancer audits bought **zero torches**. Across 1314 town records, fuel weight is zero and all **1606 observed torch offers** have effective availability zero: [native unlight audit](unlight-audit.json). The one lava death repeats the earlier diagnostic trajectory; no torch-buying failure is claimed.

## Identity, verification and retained evidence

Sweep runtime: **207795e7c0024b60a8153c39b70bb0efbd1b370b**; [exact binary/source receipt](sweep-build.json). All weights were explicit configuration values. Selected-default runtime: **5d8511c9478ceae7faf6067f2b361fe72ebe5416**; [exact binary/source receipt](selected-build.json). Eleven Ranger and seven Necromancer default-setting repetitions match all keys/messages/states: [18 repeat receipts](selected-runs.json). These 18 repeats, three recording-off checks and smoke/coverage checks are verification attempts, not new survival samples.

- 287 Rust tests and 26 Python tests pass; clippy warnings denied on Rust 1.98.1 and 1.100.0.
- Native nine-class ordinary-shop coverage passes (104 kinds accounted for).
- Native partial-rune, full-pack/quiver/Home, exact-sale and rod regression fixtures pass on the final runtime inputs ([receipt](native-fixture.json)).
- Corrected C/Rust smoke: 3/3 key/message/state matches.
- All 39341 schema-10 sweep records replay on the final runtime; all 7953 preserved schema-9 records replay.
- Twelve disabled-policy/generic-reserve-zero runs reproduce the previous candidate streams exactly.
- Three final native recording-on/off runs match keys/messages/states.

Schema 10 appends the Ranger usable-shot curve at ID57, retaining schemas 4–9 replay. Historical curves remain frozen in their records. Calibration settings are registered in both backends (used by the Rust live planner) and exported by the profile tooling.

Raw binaries, configs, frames, native logs, final saves, every executed cell, failed initial report attempt and orchestration scripts are preserved outside Git under:

```text
/home/diego/projects/borg-rs/harness/out/ranger-missile-calibration-2026-11-15
/home/diego/projects/borg-rs/harness/out/ranger-missile-calibration-2026-11-15-low
/home/diego/projects/borg-rs/harness/out/ranger-missile-calibration-2026-11-15-selected
```

No zoo workflow/specimen pin, accepted tag or experiment branch was changed. PR85 remains a draft for independent review.
