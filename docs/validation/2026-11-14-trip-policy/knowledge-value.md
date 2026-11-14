# Inventory value and a proposed knowledge-return rule

Status: **proposal for owner review, not a new acting formula** (2026-11-14).
The task was to examine deeper inventories, then propose a rule covering
learning/kit improvement, pack pressure and the town's ability to resolve
the backlog. Shared town-model integration remains with the buying session.

## What the deeper cases actually show

Seven retained checkpoint replays were attempted. Each preserves the saved
RNG, starts fresh Borg memory, uses the policy off and stops at 2,000 keys
(with a 60-second process bound). The first three were Warrior depth 78,
Blackguard depth 40 and Mage depth 22. Because the Mage stopped before a
policy sample, four additional checkpoints cover Warrior depths 42, 32,
24 and Mage depth 12. Both Mage saves stopped with zero keys; neither is
decision evidence. No attempt is counted as a new independent life.

The checkpoints come from the historical `extended` **cheat-death coverage
suite** at source `28b8170`, not an unassisted survival cohort. Original
identity and construction receipts remain in
`harness/saves/captured-extended/SOURCES.json`. The five sampled saves'
hashes were rechecked against it before curating six snapshots in
[knowledge-cases.json](knowledge-cases.json).

| Case / sample | Policy pack free | Unknown reusable stacks | Relevant observation | Existing decision |
|---|---:|---:|---|---|
| Warrior, depth 78 | 1 | 1 | Unknown staff, estimate 70; Recall capability 100 | Return: pack |
| Blackguard, depth 40 | 2 | 0 | Prepared only through depth 30 | Ascend: unprepared, pack reason retained |
| Warrior, depth 42, first | 2 | 0 | Already at the planned depth | Return: pack |
| Warrior, depth 42, later | 1 | 1 | Unknown rod, estimate 90 | Return: pack |
| Warrior, depth 32 | 1 | 0 | Prepared only through depth 29 | Ascend: unprepared, pack reason retained |
| Warrior, depth 24 | 1 | 1 | Unknown staff, estimate 70; three potions in one other unknown stack | Return: pack |

All five sampled replays reached the key bound without death. The table
describes sampled decisions with the policy off, not what an enabled
variant would execute. The two preparation cases preserve the proposal's
existing priority; this exercise does not accept that priority.

**No sampled case needs a new value weight to cause a return:** the useful
device examples already trigger pack pressure. There is no roomy, valuable
unknown-gear counterexample in this sample. It would be misleading to call
the table validation of a weighted return formula.

The separate knowledge flags expose a more useful distinction: every
unknown stack in the curated snapshots has an **unknown flavour**, with
`runes_unknown = false`. An Identify Rune resource would not address the
missing knowledge in these examples. The return needs a matching flavour-
learning opportunity, not just a generic promise of identification.

## Why base-price weights are not the proposed solution

`borg_object_value_guess` assigns generic guesses of 70 to unknown staffs,
50 to wands, 90 to rods, 45 to jewellery and usually 20 to other unknown
types. Those are not the learned device's actual effect or usefulness.
For aware but incompletely identified items, the estimate can be the known
base kind's cost, which does not price an unresolved rune or random power.
A cheap base item may therefore conceal an important capability. The
previous suggestion to distinguish a "cheap weapon" from a "valuable
device" by these numbers was too strong: the device's value is still unknown.

Recommendation: record the estimate for inspection, but do not turn
`70 / 20` into a knowledge weight or multiply it by ammunition quantity.
Learning that a stack is junk can itself be useful. Selling for zero gold
under no-selling rules can still identify and clear a stack, as the earlier
staff trace demonstrated. Conversely, merely spending gold in town is not
evidence of a useful return.

## Concrete proposed rule

Keep survival outside the policy, existing restock/preparation priorities,
and pack-pressure returns. Refine the **knowledge** vote in this order:

1. **Name the missing knowledge per stack:** unknown flavour/effect versus
   unknown runes/powers. Do not assume Identify Rune reveals an unknown
   device flavour. The record now separates `flavour_known` and
   `runes_unknown`, retaining the aggregate `unknown` bit.
2. **Name a town-specific resolution opportunity.** Examples are a known
   affordable copy whose purchase teaches the missing knowledge, an
   available rune-identification resource for missing runes, or an
   owner-permitted transaction that identifies/disposes of the item.
   A resource already usable safely here is not extra value created by
   going to town; let the existing identification/use logic act first.
   Disabled stores rule out shopping-based resolution. Unknown shop stock
   or unknown prices are **unconfirmed opportunity**, not proof of a fix.
   Keep availability three-valued: confirmed, plausible/unconfirmed, or
   unavailable. Unknown stock alone must not forbid an exploratory return;
   it also must not be logged as a confirmed benefit.
3. **Prioritize what resolving it enables.** A player-known artifact or
   random-power ego with unresolved powers and a matching town opportunity
   can justify a return by itself. This follows the existing identification
   priority in `borg_test_stuff`; it does not read the hidden power or
   guarantee an upgrade. Ordinary unresolved reusable stacks contribute
   one each toward the provisional backlog threshold (currently four),
   when a matching resolution mechanism is at least plausible. Mark returns
   based on unconfirmed stock as exploratory. Routine consumables
   still count toward pack pressure, without an independent gear vote.
4. **Make feasibility explicit.** For Recall, report the available return
   capability and the use required; do not confuse `BI_RECALL` with a literal
   scroll count. For stairs/search, retain the known route and current
   supply/restock assessment. Supply trouble strengthens the need to get
   home; it must not become a reason to stay for more loot. There is no
   penalty for lost descent speed. Numerical travel-risk weights remain
   unmeasured and are not invented here.
5. **Check the outcome.** Match arrival and departure inventory/knowledge
   and actual transactions. A useful result can be learning, freeing space,
   improving the kit or replenishing required supplies. If the same unknown
   backlog remains, record the failed expectation. Repeating a knowledge
   return would then require a new item or a changed resolution opportunity,
   not simply the same aggregate count. No anti-repeat guard is added before
   such a case is observed; the earlier trace did not show one.

The resulting decision table is reviewable without pretending to have
measured probabilities or gold weights:

| Knowledge case | Proposed standalone vote |
|---|---|
| One known artifact/random-power ego, matching town opportunity | Return |
| At least four ordinary reusable unknown stacks with confirmed opportunities | Return |
| At least four ordinary reusable unknown stacks with a plausible mechanism but unconfirmed stock | Exploratory return, uncertainty recorded |
| Only unknown consumables, roomy pack | No gear vote |
| No plausible matching mechanism, or the required service is disabled | No standalone knowledge vote; record why |
| Identification already safely available here | Resolve here before considering a knowledge trip |
| Pack nearly full or crucial supplies out | Existing independent reasons still apply |

The high-priority case and opportunity gates are **not implemented in
`decide()`**. The trip module currently lacks the town opportunity facts;
guessing them would make a promise it cannot support. The acting experimental
policy still uses the raw reusable-stack threshold. The next implementation
boundary is a read-only opportunity description from the buying session:
missing-knowledge type, proposed mechanism, known cost/availability,
whether the item is preserved, and when that stock observation was made.
The trip policy owns the return decision; the town session owns the action
and must reconcile it with its rule keeping unknown gear until known.
In particular, a forced sale of protected unknown gear is not authorized by
this proposal.

## Evidence support added in this change

Trip v2 records gain a numeric/boolean `pack` projection: slot, visible
item type, quantity, the Borg estimate, flavour awareness, unresolved-rune
flag and aggregate unknown bit. No hidden item generation level, true
value, private kind or hidden powers are recorded. Pack evidence changes
are logged even if aggregate counts and the recommendation stay unchanged.

Raw replay/log files stay outside Git at `/tmp/borg-trip-values/`; the
final build's samples are under `/tmp/borg-trip-values/final/`. The curated
JSON retains input-save identities and the sampled policy facts, not saves
or binaries. Final checks: release tests 227 passed, 4 ignored; clippy clean;
C/Rust smoke 3/3 identical; a 100,000-turn policy-off seed-101 recording
comparison preserves keys, messages and state byte for byte. The five
deeper replays also preserve those streams when the separate knowledge flags
are added. Curated pack projections reproduce the aggregate unknown counts;
a unit control changes hidden generation metadata without changing the
projection, and verifies that a changed known estimate does change it.
No full parity receipt was re-earned for this evidence-only change.
This is a bounded investigation and a concrete proposal, not fitting,
acceptance of parameters, zoo evidence or default activation.
