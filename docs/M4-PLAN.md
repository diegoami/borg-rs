# M4: wider parity evidence

M3 shows C/Rust key equivalence on the suites' seeds, the stock `borg.txt`
plus munchkin and lunal mode, up to dungeon level 27. All three external
reviews name the same open gap: the endgame, most configuration options,
and state that does not change a key are not compared. M4 widens the
evidence; it does not change the Borg (no behaviour change, the C Borg
stays unmodified, existing references stay as they are).

| step | what | evidence it adds | status |
|------|------|------------------|--------|
| A | messages and engine-state log beside the key log, in both builds | Borg notes, game messages and per-key engine state (position, HP, depth, gold, pack...) compared, not only keys | done: `messages.log` and `state.log` per run (engine-side hooks, both builds), checked by `harness/diff`; added to the smoke, full, deep, classes and config references (C twice identical, keys and summaries unchanged), Rust 80/80 identical on all three logs; see STATUS |
| B | an `options` suite: one run per remaining `borg.txt` option (money scum, dynamic calcs, worships, risky play, swaps off, respawn...) | configuration-dependent code paths | done: `harness/suites/options` (25 runs, `NAME=VALUE` and `cheatlive` flags, start-up `-more-` answered), C twice identical, Rust 25/25; see STATUS |
| C | more seeds and longer cheat-death runs | more of the mid game, deeper levels | planned |
| D | endgame starting points: savefiles at depth 50+, 98-100 and next to Morgoth, loaded by both builds with the same seed | the endgame (Sea of Runes, Morgoth, deep swaps, self-lunal) | planned |
| E | a committed native-C differential checker (the original C Borg linked with renamed symbols beside the Rust one, called on the same state, with fuzzed inputs) | replaces the uncommitted conversion cross-checks; reaches code no game reaches | planned |
| F | coverage measurement of the Rust build over all suites | states coverage as a number; guides D and E | planned |

Each step lands with its own C references (made with the C build, twice,
identical) and a checkpoint of all suites.
