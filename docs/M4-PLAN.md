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
| C | more seeds and longer cheat-death runs | more of the mid game, deeper levels | done for declared suite at28b8170: C/C and Rust/C12/12, all original limits reached; Warrior2000's same-square melee blocker (#322) corrected narrowly in C/Rust. Natural maxima99/51. Thirty capture metadata/save-byte sets match C/C/Rust; separate current extended reference |
| D | endgame starting points and replay | deep-dungeon/endgame starts with explicit tactic limits | done for declared five synthetic starts plus two Borg-played cheat-death replay starts: crash-fixes6/6 and natural2/2 C/C/Rust; Warrior78 reaches84 and Blackguard40 reaches41. Deep play and Morgoth melee observed; Sea of Runes/self-lunal/win remain unestablished |
| E | a committed native-C differential checker with fuzzed shared inputs and logical state comparisons | persistent checks for code and state beyond game oracles | implemented for declared kernels: calculator30k;4k cave/projection/direction/flow/melee states over two seeds;1,600 glyph-builder states and1,280 lunal-thinker states; full private map/flow/queue/tracking footprints and ordered callbacks/notes/keys. Seven actual-source faults detected, C UBSan and clippy pass. Engine/environment boundaries and unshadowed whole-Borg state remain explicit in the checker README |
| F | coverage measurement of the Rust build over all suites | states coverage as a number; guides D and E | current-source collection active:119 accepted runs through crash-fixes; extended/natural pending. Completed113-run subset measures77.56% logic lines and90.14% LLVM function instances. Full F remains open; older48-run source26 pilot retained separately |

Each step lands with its own C references (made with the C build, twice,
identical) and a checkpoint of all suites.

Checkpoint after A and B (PROGRESS row 65): Rust = C on keys, messages
and state in all 105 runs of the six suites.

## Step C run plan (2026-11-06)

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
