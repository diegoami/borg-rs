# Necromancer and Ranger: rs-kit-trip-confirm

The Necromancer regression is not explained by buying or wearing torches.
The exact original traces instead expose an empty-light-slot bug that can force
resting ahead of healing during Recall. Ranger traces confirm a separate quiver
packing error; lack of arrows is a contributor in some cases, not a complete
explanation of the class result.

## Original comparison and reconstruction

Zoo run **43667787016**, `rs-kit-trip-confirm`, compared two configurations of
one exact build, `dd3eeac646e097bcdd7c289c092e91400896a713`:
control has `borg_town_plan=0,borg_trip_policy=0`; default has both set to 1.
Both use risk 1000 and `borg_buy_identify=0`. That latter flag does **not**
disable the town model's independent knowledge priority (25).
The zoo source was `091e7eee87f29a81a49f4a35b7d391764ee2cf7c`.

| Class | Paired starts | Control deaths | Default deaths | Rescued | Newly killed |
| --- | ---: | ---: | ---: | ---: | ---: |
| Necromancer | 200 | 75 | 99 | 41 | 65 |
| Ranger | 200 | 56 | 66 | 41 | 51 |

Across all 1800 paired starts, deaths are 516 versus 471; the exact paired
McNemar two-sided p-value is 0.08263. The overall reduction does not establish
a 0.05-level confirmation, and neither it nor the class slices explain causes.

The original GitHub build artifact (14143166152) was recovered. Its binary hash
is `aad647601182e14de7e75c575e9ecd7aa9afb989945156c777478dd70d6aa646`.
The 24 selected paired traces and 110 additional newly-killed traces reproduce
the original zoo key hashes exactly; configuration hashes also match. They
cover all 65 newly-killed Necromancers and 51 newly-killed Rangers. Added native
frames expose carried items, equipment, quiver and spells; diagnostic hidden
fields were not fed into decisions. Reruns are reconstructions, not extra lives.
[Build identities](validation/2026-11-15-class-investigation/builds.json),
[all newly-killed observations](validation/2026-11-15-class-investigation/newly-killed.csv)
and [selected original town purchases](validation/2026-11-15-class-investigation/selected-town-purchases.md)
are retained for review.

## Necromancer

Of the 65 newly-killed cases, **none bought or equipped a torch or lantern**.
Fourteen bought illumination (13 Scroll of Light, one Staff of Light); none
used it in these traces. Every case had known healing stock at its last recorded
live frame, and 41 had Phase Door. These frames precede death by a median of
8 world turns (maximum 35), so they are explicitly observations before death,
not invented inventories at the fatal action. Recent Recall activity occurs
in 51 cases within 300 world turns of death.

Necromancers have `PF_UNLIGHT`, start without a light, and the Borg reports
`BI_LIGHT=1` in that state. The light emergency helper nevertheless tested the
empty slot's timeout (zero) as a dying lamp. During Recall it queued `R9` and
returned before caution could heal or escape. The same mistake exists in C.
The correction requires a real item (`iqty != 0`) before treating a lamp as
expiring; an actual absence of needed light still follows the emergency path.

Dwarf seed **1405288147** illustrates the mechanism: default dies to a wild cat
at turn 1175. At the last live frame (1171) it has 2/13 HP, four known CLW and
one Phase Door, no equipped light, and repeated Recall rests. Its first shopping
spends 291 of 360 gold, including four CLW, fifteen oil flasks and support/
knowledge purchases. Control buys substantially more armour and survives the
cap. That spending difference is observed; it is not proof that buying one
particular armour item would rescue this death.

An independently rebuilt unmodified runtime reproduced **all 200** original
Necromancer default key hashes. On the same 200 cases, the isolated empty-light
correction changes deaths **99 → 52** (63 rescues, 16 new deaths; 29 unchanged
key streams). [Per-case diagnostic results](validation/2026-11-15-class-investigation/empty-light-probe-all.csv)
retain all outcomes. These are post-hoc, same-seed diagnostics, not fresh
confirmation. The bug is shared by control: comparing corrected default's 52
against uncorrected control's 75 would be an unfair policy comparison. Future
paired experiments must include the correction in both arms.

## Ranger

At the last live frame, 18 of 51 newly-killed Rangers have zero compatible
missiles, four have 1–4 and 29 have at least five. Forty-three have healing stock
and nine have Phase Door. Median frame-to-death gap is two world turns (maximum
17). Many deaths therefore need explanations beyond ammunition quantity.

A concrete planner error is visible in **seed 285192323**: at turn 23768,
key 3289, the arrow offer is charged one pack slot. Native frames before/after
the purchase show quiver arrows **2 → 3**, with the pack still occupying eight
slots. Existing compatible quiver room must be included. The production fix
records native compatible room and accounts for aggregate forty-unit quiver
expansion; it does not make arbitrarily many arrows free of space costs.

An isolated, deliberately simpler quiver-join diagnostic reproduces all six
selected original Ranger baselines before changing packing. On those six it
rescues two of the three newly-killed examples; the earliest poisoned Ranger
is unchanged. [Diagnostic cases](validation/2026-11-15-class-investigation/quiver-probe-pilot.csv)
are selected probes, not a representative survival estimate, and the production
packing implementation adds capacity limits absent from that simple probe.

Dunadan seed **1632766293** dies poisoned at turn 1450 with 18 arrows, no
healing and no Phase Door: increasing attack priority/demand gives exactly the
same keys. Seed **1401471427** dies to a pseudo-dragon at CL6/DL4 with no arrows
and 20 CLW; default bought 78 arrows versus control's 177. These cases support
inspecting allocation and consumption together, not a blanket class multiplier.

Thirty-eight of the 51 traces also contain the inherited `restock food < 3`
message. The utility shopper can prefer fewer nutrition-rich supplies while
legacy preparation still demands three counted food sources. In three
selected newly-killed cases a food-duration probe of 500 removes this signal
and survives, but it changes spending as well. That is a remaining policy
coordination/calibration question, not grounds to declare 500 a proven default.

## Scope and evidence

Kimi's 13 review findings and both confirmed class defects are handled in
[the bundled corrections](TOWN-PR83-FIXES.md). The final combined candidate
has its own validation; the isolated diagnostic outcomes above must not be
presented as that candidate's performance.

Raw images, manifests, configurations, native messages, key/state/frame logs,
all diagnostic attempts and final saves are preserved outside Git at
`/home/diego/projects/borg-rs/harness/out/class-analysis-43667787016`.
The original build, rebuilt baseline, empty-light and quiver diagnostic images
have separate identities. Death causes and before-death resources are preserved
per case, including every diagnostic death; causes remain provisional beyond
the confirmed source mechanisms. No new zoo issue, workflow or comment is
posted by this change.

The retained [light](validation/2026-11-15-class-investigation/empty-light-diagnostic.patch) and [quiver](validation/2026-11-15-class-investigation/quiver-cost-diagnostic.patch) diagnostic patches are independently relative to frozen `dd3eeac`. Apply either with `git apply --unidiff-zero` in an isolated checkout; they are diagnostics, not the complete production bundle.
