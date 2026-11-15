# PR85 qualified-review follow-up

Review: [Sonnet qualified pass](https://github.com/diegoami/borg-rs/pull/85#issuecomment-6894417316) of head **434aa91f8e32b04d51a67779e4970f3eed64c157**. This is Sonnet's review, not Kimi's. The verdict does not automatically cover the subsequent fixes.

Runtime with these corrections: **d99e4c0f09bd** (full immutable identity in [build.json](build.json)). This follow-up changes the Rust live planner/throwing forecast and tooling/documentation. It does not modify the native C oracle or expand the tactical consumers.

## Review dispositions

1. **Confirmed unused control devices — fixed.** Confuse Monster wand, Confuse Monsters staff and Stun Monster wand have no positive known-use supply. Unsupported confusion/stun and unconsumed sleep/turn subtypes are excluded. Slow, hold, sleep-all/sleep-evil and fear-all remain. Scare Monster has a real BF_WAND_FEAR_MONSTER consumer, so TURN_ALL is deliberately retained; removing it would be an unsupported loss of capability. The inherited Confuse Monster lookup remains unchanged and first-copy learning is still separate. The native coverage gate now fails if a listed unsupported ordinary-shop device regains supply utility.

2. **Blocked first urgent hides a later feasible emergency — fixed.** Live action selection scans the urgent list for the first item fitting current room. A new food stack cannot hide a poison cure or Home withdrawal that joins existing stock. When all urgent options are blocked, existing feasible dispositions still free room before leaving. The new regression tests cover a full pack, later shop cure, later Home source and the all-blocked case; historical room-unfiltered selection keeps its order.

3. **Heap-address reuse — documented limitation.** The engine wrapper and correction map now state that a freed/reused address can alias an unresolved task. No frequency or reproduction claim is made. Generation-safe identity remains open; partial-rune stability does not establish allocation-generation safety.

4. **Forecast on every object-attack evaluation — fixed.** A forecast is calculated lazily, once within that evaluation, only for a viable throwable flask when a fuel-burning lantern is worn and the live town policy is enabled. No cross-turn cache or performance percentage claim is introduced.

5. **Quiver accounting seam — documented limitation.** Native forty-unit accounting belongs to the town planner. Other Borg consumers still count BI_QUIVER_SLOTS stacks. Their integration remains a known gap; no loop is proven and off-policy rules are preserved.

6. **Unlight Lantern eligibility — clarified, no broader ban.** Torches are excluded as requested. Unlight suppresses fuel utility for both torches and Lanterns; a different equipment light can still earn actual gear value. Native known-item audits show zero Torch/Lantern fuel gain in the Necromancer context. That is not a claim that all future enchanted Lanterns are forbidden.

## Validation

- **289 Rust tests**, **26 Python tests** pass. Clippy with warnings denied passes on Rust 1.98.1 and 1.100.0.
- Native partial-rune, full-pack/quiver/Home, exact-sale and rod fixtures pass.
- Nine native class contexts and all 104 ordinary-shop kinds are accounted for. Confuse Monster is now an explicit exclusion rather than a covered control purchase. [Known-item audits](known-use-audit.json) are hypothetical native facts, not completed purchases.
- **39341 preserved schema-10** calibration records and **7953 schema-9** records replay. Producer classification changes do not rewrite historical frozen supply vectors.
- **3/3 corrected C/Rust smoke** runs have matching keys/messages/states; C runtime inputs were unchanged.
- **18 bounded native reruns** (11 selected Rangers, seven Necromancers, 100000-turn caps) reproduce the previous candidate's keys/messages/states exactly. Actual purchases and outcomes therefore preserve the published calibration: 1616 Ranger arrows, zero Necromancer torch buys. These reruns are verification attempts, not new independent lives.

Four natural deaths repeat the previous trajectories (three Rangers and the Necromancer lava case). All four prior native observations/assessments remain applicable because the full native streams match; see [calibration death assessments](../2026-11-15-ranger-missiles/deaths.md) and the exact per-attempt [new build/input receipts](runs.json). New cases are not added to the survival sample.

## Limits and review boundary

This follow-up does not establish improved survival or progress, close the heap-generation/quiver seams, ban all equipment lights, or implement tactical consumer issues #79/#80. The original qualified pass is recorded against its exact old head; the new delta remains available for independent review. No discussion comment, new issue, merge, accepted tag, zoo pin or zoo run was created.

Raw trajectories, final saves, binaries and inputs are preserved at:

```text
/home/diego/projects/borg-rs/harness/out/pr85-sonnet-followup-2026-11-15
```
