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
| C | more seeds and longer cheat-death runs | more of the mid game, deeper levels | in progress: source26a96be C/C and Rust/C12/12 match, with natural Warrior depth99 and Blackguard51, but Warrior2000 stops early on same-square melee (#322). Narrow C/Rust correction28b8170 passes native before/after control; fresh full matrix plus forked1M captures is running. Historical reference remains separate |
| D | endgame starting points: savefiles at depth 50+, 98-100 and next to Morgoth, loaded by both builds with the same seed | the endgame (Sea of Runes, Morgoth, deep swaps, self-lunal) | done for the declared five starting points: all five unchanged synthetic fixtures now match corrected C/C/Rust in keys/messages/state within the six-case `crash-fixes` suite; the corrected reference is separate from original `endgame`. Deep play and Morgoth melee are observed; Sea of Runes, self-lunal and a win are not established. Both builds pass all 105 original baseline cases and eight captured Rogue cases in all three logs |
| E | a committed native-C differential checker (the original C Borg linked with renamed symbols beside the Rust one, called on the same state, with fuzzed inputs) | replaces the uncommitted conversion cross-checks; reaches code no game reaches | in progress: `harness/native-checker` compares real renamed C calculator code with the unchanged Rust module, results/diagnostics/full token state;30k cases, real-source fault control and C UBSan pass. Other private combat/map state not covered |
| F | coverage measurement of the Rust build over all suites | states coverage as a number; guides D and E | pilot only: smoke/full48 at26a96be, logic lines61.40%, LLVM function instances77.03%; strict oracles and per-process profiles. Interrupted deep group excluded; full/new-source coverage remains open |

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
