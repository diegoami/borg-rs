# M4: wider parity evidence

M3 shows C/Rust key equivalence on the suites' seeds, the stock `borg.txt`
plus munchkin and lunal mode, up to dungeon level 27. All three external
reviews name the same open gap: the endgame, most configuration options,
and state that does not change a key are not compared. M4 widens the evidence. Owner-authorized crash and blatant play-blocker
corrections are allowed in both C and Rust; other conflicts are decided
case by case. See `docs/FORK-CORRECTIONS.md`. Original references remain
separate from any new corrected-C references.

| step | what | evidence it adds | status |
|------|------|------------------|--------|
| A | messages and engine-state log beside the key log, in both builds | Borg notes, game messages and per-key engine state (position, HP, depth, gold, pack...) compared, not only keys | done: `messages.log` and `state.log` per run (engine-side hooks, both builds), checked by `harness/diff`; added to the smoke, full, deep, classes and config references (C twice identical, keys and summaries unchanged), Rust 80/80 identical on all three logs; see STATUS |
| B | an `options` suite: one run per remaining `borg.txt` option (money scum, dynamic calcs, worships, risky play, swaps off, respawn...) | configuration-dependent code paths | done: `harness/suites/options` (25 runs, `NAME=VALUE` and `cheatlive` flags, start-up `-more-` answered), C twice identical, Rust 25/25; see STATUS |
| C | more seeds and longer cheat-death runs | more of the mid game, deeper levels | done for declared suite at28b8170: C/C and Rust/C12/12, all original limits reached; Warrior2000's same-square melee blocker (#322) corrected narrowly in C/Rust. Audited deepest logged depths52/99/59 for the three long cases; final max_depth fields44/99/51 can reset with recall. Thirty capture metadata/save-byte sets match C/C/Rust; separate current extended reference |
| D | endgame starting points and replay | deep-dungeon/endgame starts with explicit tactic limits | done for declared five synthetic starts, two Borg-played cheat-death replay starts and six targeted engine cases. Real self-lunal entry/first descent and one glyph creation verified. Full rune formation, unbounded scumming and a win remain unestablished |
| E | a committed native-C differential checker with fuzzed shared inputs and logical state comparisons | persistent checks for code and state beyond game oracles | done for declared kernels: calculator30k;4k cave/projection/direction/flow/melee states;1,600 glyph and1,280 lunal states; complete declared map/queue/tracking footprints and callbacks. Seven real-source faults rejected, C UBSan and clippy pass. Native-body coverage and unshadowed whole-Borg boundaries are explicit |
| F | coverage measurement of the Rust build over all suites | states coverage as a number; guides D and E | done:11 groups/139 successful oracle-checked engine runs, one frozen image; logic29550/34610 lines (85.38%),1269/1380 LLVM function instances (91.96%). New cases give155 lunal/8 glyph entries. Original133 baseline and source26 pilot remain separately labelled; branch coverage unmeasured |

Each step lands with its own C references (made with the C build, twice,
identical) and a checkpoint of all suites.

Checkpoint after A and B (PROGRESS row 65): Rust = C on keys, messages
and state in all 105 runs of the six suites.

## Historical Step C run plan (2026-11-06)

`harness/suites/extended` uses `menu,cheat` throughout, with the stock
configuration. Seeds 4000-4008 play all nine actual classes for 2M turns
each: Human Warrior, High-Elf Mage, Half-Elf Druid, Dwarf Priest, Kobold
Necromancer, Dunadan Paladin, Hobbit Rogue, Half-Elf Ranger, Half-Troll
Blackguard. Human Warrior seeds 2000 and 2007 and Dunadan Blackguard
3100 run for 5M turns each. Existing suites and references are unchanged.

Before running: both builds and both smoke suites passed (3/3 keys,
messages and state identical to C references, no identity overrides),
and `check-no-c-borg.sh` passed. The initial budget was 4-8 machine hours
at two jobs per pass, including two C passes, Rust, comparisons and the
full checkpoint; long-run timing is provisional. Output directories are
separate for each pass. A partial output is not evidence: accept only
completed suites with `DONE`, then compare C/C before storing any new
references. Record reached depths and stop reasons rather than assuming
that more turns guarantee more coverage. Main and tags stay unchanged
until a checkpoint; tags are created by the owner.

## Current review request

The owner narrowed the current task to an independent review of M4 only.
[borg-rs#1](https://github.com/diegoami/borg-rs/issues/1) pins `m4` at
`3923a21e75dfb94053d4cc787d81c5baf638d62e`, with the scope and evidence.
The independent review is complete: qualified pass with no acceptance
blockers ([final review](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707067352)).
The reviewer verified c2b52d9's fixes, recording only a new low cost75 test
gap and an oracle-test portability note. Tested author follow-ups now cover
cost74/75/76 and use M4-only oracle fixtures; see the residual receipt.
Their independent verification and owner acceptance remain pending.
Both parties must post progress,
questions, finding dispositions and follow-up verification in that issue.
M1-M3 supply background only; post-M4 #295 is excluded from this verdict.
The previous technical sequence below remains queued for later work.

## Agreed next-session order

The owner accepted this sequence and deferred new development until the next
session, then explicitly resumed and delegated milestone tag creation.

1. Main integration and the author checkpoint audit are complete. The owner explicitly
   delegated tag creation on resume: `m4` = `3923a21` is pushed and verified,
   before broader behavior changes. See validation/2026-11-07-main-review.md.
2. Glyph-recognition #295 is corrected and checkpointed after the tag;
   see validation/2026-11-07-glyph-recognition. Next address reverse-flow
   finding#345; require autonomous complete rune building before claiming
   the full tactic works. Recognition of a prelaid ring is a separate result.
3. Investigate self-lunal stair cycling. Current evidence establishes entry
   and first descent; failed longer clock-bound pilots remain excluded.
4. Run bounded normal games with cheat death off and analyze natural deaths.
5. Review the game-start scripts too (owner's additional next-session topic).
   Current Rust commands are in README; the local native C reference and its
   save workflow are recorded in LOCAL-SESSION-HANDOVER. Further desired
   script changes were not specified; preserve existing characters.

These are next-stage tasks, not additional evidence claimed for the completed
bounded M4 corpus. Continue to apply the owner's correction policy, preserve
original references, and establish any corrected C reference with two identical
C runs before checkpointing Rust. The first post-tag correction #295 is complete;
the next technical priority is #345.
