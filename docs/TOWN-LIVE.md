# Experimental live town executor and local comparison

Owner-directed on 2026-11-14: execute the utility planner and produce a matched
full-suite outcome table before choosing a small parameter subset for the zoo.
`borg_town_plan = 1` now enables the experimental executor; **the default remains
0**, the unchanged current-policy control. `borg_town_no_stores` takes precedence.
This is not a default change, derived tag, or authorisation to merge PR #42.

The executor shares the shadow producer/solver, routes through existing shop
goals and existing key executors, and replans on arrival and after transactions.
Purchases execute one copy at a time; sales/deposits use the selected stack
quantity. Known urgent Home cures are taken before paying for a shop cure.
Home withdrawals intended for sale do not acquire the just-bought veto.
Buy/sell veto history is scoped to the town visit; the existing 2000-turn and
1350 anti-bounce guards remain. Drop commands execute outside the store UI and
mark junk to prevent immediate pickup. The dungeon tactics and depth policy
are unchanged.

## Integration checks and quote correction

Unit checks cover current/null-arm gating, valid shop/pack/Home goal binding,
whole-stack quantities, stale intent rejection, urgent Home cures and the
sold-kind veto. A ten-thousand-turn Priest pilot exposed a zero-price trap:
the legacy Borg appraisal returned zero for an unidentified negative-bonus
Black Market ring, while the game charged a positive price. The live planner
now reads the actual shop UI quote through the engine's `price_item` function.
The control retains its historical appraisal, preserving its parity surface.
The corrected Priest rerun has no insufficient-gold purchase loop.

Preflight outputs remain outside Git under `harness/out/rust/town-live-*`;
they are integration attempts, not entries in the measured full-suite table.

## Record revision 6

New records use schema 6, retaining the 24 parameters and row layouts. It
identifies live execution/actual quote semantics and avoids a known empty-value
Home round-trip: unprotected known junk with no current/future value is sold or
dropped, rather than stored solely for a small slot-pressure difference.
Just-bought Home stocks also keep the sale veto. Historical schemas 4 and 5
remain readable and use their original disposition semantics during replay.
Mode 0 is observation, mode 1 is live; the optional record file never enables
or disables execution. Live action quantities reflect one-copy purchases.
The core plan remains a prospective kit; it is re-solved after each action.

## Frozen local suite

`tools/town-plan/run_live_suite.py` runs the 45 existing full-suite seed/race/
class rows at 100000 turns, with explicit menu-letter selection so all nine
classes and intended races are played. This corrected birth contract is written
in the manifest; the old full suite's index-7/8 quirk is not silently reused.
Four arms share the same runtime and starts: current, no shops/Home, utility
reference, utility cash priority 50. All modes are explicit, cheat death is off,
and a 180-second per-cell wall limit is retained. No class-specific winner is
assumed. Every attempted result is retained, including failures.

Report deaths, terminal character level, maximum dungeon depth, DL3+ counts,
and per-class outcomes, with valid denominators and operational diagnostics.
This is a new matched town-branch comparison, not a relabelling of historical
milestone-12/class-ladder results. The branch's historical v11R base is preserved;
no unrelated v12R/class-ladder source is merged from another session.

After the local results, choose a small parameter subset based on provisioning,
spending, Home traffic and failures. Freeze a separate zoo development grid and
fresh-seed confirmation plan. Do not start a large hosted workflow or claim
improvement from five starts per class alone.
