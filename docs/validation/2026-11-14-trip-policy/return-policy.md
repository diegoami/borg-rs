# Knowledge-return refinement (2026-11-14)

This follows the merge validation; it is a new candidate, not an extension
of that candidate's 84-case receipt. The policy remains off by default.
The owner left shared trip-model integration to the buying session.

## Reason for the change

A flat unknown-stack count treats early potions and scrolls like reusable
equipment hiding unknown powers. The replacement counts unknown equipment,
ammunition and devices (`TV_SHOT..=TV_ROD`) toward the existing threshold
of four stacks. Consumables and books receive no independent knowledge
vote, but still consume pack slots. This is a provisional categorical
weighting from visible item type, not a fitted value or owner acceptance
of the formula. No hidden kind level or true object value is read.

Record version 2 retains all unknown stacks and adds `unknown_gear_stacks`.
Records now include input changes even when the decision is unchanged;
otherwise the excluded unknown stacks disappear from the observations.
Gameplay notes still change only with the decision/reasons or level.

## Validation

- Release tests: 226 passed, none failed, 4 ignored; clippy clean.
- Both engine implementations built. C/Rust smoke: 3/3 identical keys,
  messages and state, with no identity overrides.
- Seed 101's ordinary 20,000-turn smoke key hash remains
  `8d3029a7a4558e087ce1e6e51b51f2f2c777dbbb7ad7f8be65ee9c956108a6c5`.
- Final recording check: C and policy-off Rust with recording enabled
  produce identical keys, messages and state over 100,000 turns on seed
  101 (Human Warrior). All recorded v2 counts satisfy
  `0 <= unknown_gear_stacks <= unknown_stacks`.
- Focused tests cover a roomy pack of unknown consumables, a mixed pack
  crossing the gear threshold, pack pressure despite excluded consumables,
  visible category boundaries, disabled threshold and priority/route rules.

Bounded observations (Human, seeds fixed before running; no parameter sweep):

| Seed / class | Turns requested | Policy off: clevel / max depth | Policy on: clevel / max depth |
|---|---:|---:|---:|
| 101 / Warrior | 100,000 | 9 / 5 | 7 / 3 |
| 301 / Mage | 20,000 | 2 / 1 | 2 / 1 |
| 401 / Priest | 20,000 | 2 / 1 | 2 / 1 |

All six runs stopped on the turn bound without death. On seed 101, at
recorded turn 644 both arms have four unknown stacks but zero unknown gear:
the old trigger would request return, the new one requests descent. Across
that seed's records there are at most five unknown stacks but only one
unknown gear stack. The new gear threshold never fires; these runs validate
the exclusion and recording, not a useful runtime return for unknown gear.
The owner clarified that slower progress is intended: the depth/level
differences above are context, not evidence against the policy. Return records in this seed cite pack pressure; record counts are
snapshots, not a count of independent town trips.

Raw outputs: `/tmp/borg-trip-return/final/`, plus C smoke under
`/tmp/borg-trip-return/c/smoke`. Tests/build logs use `/tmp/borg-trip-return-*`.
They are local artifacts outside Git. The final Rust smoke manifest binds
the runtime source hash; builds were made with the pending changes present.

## Still open

No full/deep/options/synthetic suite was re-earned for this refinement.
No claim of improved survival or useful backlog resolution follows from
three seeds. Faster descent is not an acceptance criterion (owner
clarification, 2026-11-14); slower progress is intended.
The gear threshold needs a deeper runtime case. Town identification is not
guaranteed, and unchanged unresolved inventory could cause repeat returns.
Useful inventory resolution after returning and repeated returns with no
inventory change still need examination. Formula acceptance, threshold fitting on disjoint
seed sets, zoo migration and default activation remain open.
