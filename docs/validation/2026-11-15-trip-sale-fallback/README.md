# Selling-pressure fallback and incremental return observations

Owner direction: restore the inherited selling-pressure backstop, then test exactly `(pack_slack, unknown_stacks) = (0,0), (1,2), (1,4), (2,4)`. PR85 was already merged at 652dc8c. This follow-up restores the policy-on path before the Zoo study is dispatched.

Runtime: **6d3b345ebdc73ee17b404b0f5e564b607b707159**, with [build identity](build.json). The C oracle inputs are unchanged.

## Changes

- Restore the old level/boredom-dependent 12-sellable-stack return checks with the trip policy on.
- Restore the old low-level/low-gold 13-stack descent hold with the trip policy on.
- Keep restocking, preparedness, caution, restoration and periodic return logic. A trip decision cannot cancel an already-rising goal.
- Add optional v4 application observations to BORG_TRIP_RECORD, alongside the existing v3 recommendations. The before/after rising flag shows whether this policy newly sets the town-return goal; an existing flag may originate from earlier policy work and is not called a legacy cause.
- `unknown_stacks=0` is explicitly disabled; `pack_slack=0` still triggers on a full pack. Unit tests cover zero and one separately.

## Validation

291 Rust tests pass. Clippy warnings denied on 1.98.1 and 1.100.0 passes. C/Rust smoke is 3/3 identical in keys/messages/states, preserving policy-off behavior.

Fifty-four Zoo adapter canaries cover all six study profiles and all nine classes at 2000 turns; all reach the cap with native effective settings and complete retained evidence. These are local wiring checks using the exported study setup, not performance data. The Zoo tooling/catalogue change is separately published with the study; these canaries run the pre-merge runtime pin above.

Three Human diagnostic starts (Warrior 501, Necromancer 505, Ranger 508) run to 30000 turns both with and without town/trip recording. All three full key/message/state streams match, and all six attempts reach the cap. Their native goal observations are illustrative, not independent evidence of policy fitness.

| Diagnostic | Threshold pair | Native town arrivals | New custom goals | Arrivals after a custom goal | Unfinished trips with custom goal | Journeys with scared notes |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| warrior | (2,4) | 25 | 1 | 0 | 1 | 12 |
| necro | (0,0) | 18 | 0 | 0 | 0 | 11 |
| ranger | (1,2) | 9 | 1 | 1 | 0 | 6 |

The Necromancer still returns eighteen times with learning returns disabled and no new custom goal recorded. Those returns must not be attributed to an always-true unknown threshold. The Ranger has nine actual returns but only one arrival after a newly set custom return goal. Scared notes are observations on the same journey, not exclusive causal diagnoses. None of these runs hits a logged selling-pressure return; restoration of that path is supported by the unchanged original predicates/source and gate regression tests, not a claimed native activation in these examples.

A newly set goal is an actual transition at that consultation, not proof the character would never go home under another policy. Failed/unfinished requests are distinct from arrivals. Recommendation records and raw consultation counts are not return counts. The Zoo reports all of these separately, alongside selling/food/periodic/scared notes.

Raw binaries, configs, key/message/state/frame logs and final native saves remain at:

```text
/home/diego/projects/borg-rs/harness/out/trip-sale-fallback-2026-11-15
```

No natural deaths occurred in these bounded diagnostics or adapter canaries. No new Zoo study was dispatched before this correction. No accepted tag, main branch, historical experiment pin, PR42 or PR83 was changed.
