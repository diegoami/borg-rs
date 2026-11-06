# Third external review: borg-rs at 10c7f00ba

(Temporary review document, kept until the owner removes it. The owner
pasted the review into the session; this is a condensed record: verdicts,
findings with locations, scenarios and fixes, and reproduced results. The
project's answer is `docs/REVIEW-RESPONSE-10c7f00ba.md`.)

Reviewed 6 November 2026, commit `10c7f00bab833f45ee0163fffe62eb0a8b7a7d8c`.
No C/Rust divergence in the requested seeded games, no new undefined
behaviour reproduced.

## Verdicts

| Claim | Verdict |
|---|---|
| Key equivalence within documented seeds, configurations, limits | Holds for reproduced evidence (all 80 Rust runs match, summaries too) |
| Known C behaviour retained | Partially holds: verified for audited cases |
| 59 modules, unsafe confined, 268 | Holds structurally (engine 240, headers 28) |
| No public safe engine facade | Holds |
| Internal entry/lending invariant sound | Partially holds: verified for audited cases (natural and forced nesting work; extracted mechanism passes Miri Stacked and Tree Borrows) |
| M3 done | Holds as the documented conversion/checkpoint milestone |
| Evidence stops at depth 27, excludes the lunal thinker | Holds |

Second-review fixes: 1 (public facade) holds; 2 (arithmetic) holds on the
defined domain (1,659,644 valid inputs agree, six overflow cases panic);
3 (compression) holds; 4 (endgame/state) partially holds; 5 (editorial)
partially holds.

## Findings

1. **Medium, evidence gap: `--suite` validates names, not scenarios**
   (`harness/diff` `suite_names()` discards everything after the name;
   `suite_sha256` and per-run config not checked). Reproduced: a copy of
   the smoke suite with +1000 on every seed and turn limit, same names,
   compared against the untouched outputs: 3/3 identical, exit 0. Fix:
   check `suite_sha256`, per-run parameters against summaries, config
   records against flags.
2. **Medium, acknowledged gap: endgame and self-lunal unobserved**
   (`borg_think_dungeon.rs:1439`, `:1810`, `borg_flow_glyph.rs:119`,
   `borg_fight_attack.rs:1047`, `borg_prepared.rs:723`,
   `borg_trait_swap.rs:576`, `:701`). Reviewer added native-C fixtures
   (preparedness 272,160 cases, formulas 6,348, respawn 1,200), all C
   twice = Rust, but no endgame save or self-lunal execution.
3. **Low, inaccuracy: stale reviewer guidance**: REVIEW.md "the suites
   never nest", "269 / 241", "an external crate could call wrappers",
   depth 26; harness.md "references carry no manifest"; `harness/diff`
   help describes the old compression policy.

## Reproduced validation (summary)

C and Rust builds with clean identities; C smoke 3/3; Rust smoke, full,
deep, classes, config 3/3, 45/45, 9/9, 16/16, 7/7; fresh C full twice
45/45 in every comparison; check-no-c-borg pass; note-alias 11 lines;
nested regression 7 entries; tests 100 on Rust 1.97.0 and 1.98.1;
clippy all targets clean; unsafe 268. Natural nesting confirmed in C too
(observer build): one nested hook at turns 70014 and 75731 in the two
lunal runs. Extra experiments, C twice = Rust: money scumming (3 x 100k),
dynamic formulas (stock and minimal, 3 x 100k each, with a frontend that
acknowledges the start-up warning), debug commands `p P w v C ! 0 y $ h S`
through 20k turns (only `v` differs on screen: the disclosed build date).
Kept bugs confirmed in sections 3, 6, 7, 10, 11, 41, 45. Could not check:
endgame/self-lunal gameplay, whole-engine safety, whole-state equivalence,
all bug sections and configurations, other C implementations/platforms.
