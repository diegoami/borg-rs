# Real engine glyph and self-lunal scenarios

Six new scenarios pass native C twice and normal Rust on keys, messages,
per-key player state and complete final engine-save bytes. The same six pass
with the frozen coverage Rust image, without stale/unknown-build overrides.
Per-case profiles verify the intended positive and zero-entry controls.

| scenario | lunal entries | glyph-builder entries | actual native outcome |
|---|---:|---:|---|
| self-lunal ready |155|0|depth20 to21; configured depth stop at game turn371 |
| option off |0|0|ordinary descent to21, turn355 |
| insufficient depth history |0|0|ordinary descent to21, turn355 |
| glyph ready |0|4|engine glyphs0 to1; Rune scrolls20 to19 |
| low glyph supply |0|0|engine glyphs remain0; scrolls remain9 |
| prelaid24-glyph ring |0|4|engine glyphs24 to25; scrolls20 to19; exposes existing#295 |

Fixtures are constructed level50 Human Warriors with the existing artifact/
healing kit. They are not naturally attained characters. Lunal starts at20
with maximum99 or34; the actual caller requires current depth greater than
clevel/3, preparedness for15 deeper, and sufficient recorded depth history.
The glyph setup supplies9/20 known Rune scrolls at100, a central open room,
Morgoth alive but far away and other uniques marked dead to isolate preparation.
No cheat-death flag is enabled. No Borg map, goal or tracking field is seeded.

The constructor relinks unchanged native C engine/Borg objects with a test-only
save hook. Its source/hash/link argv and each original object hash are pinned
in the canonical fixture generator.json. It calls native object/terrain/level
APIs and explicitly initializes engine history/quantity fields. Every fixture
was generated twice with identical saved bytes, logs and normalized summaries.
The unchanged production C/Rust binaries perform all accepted replays; the
constructor is not presented as a production build or used in coverage replays.

The first game-clock pilot and later-depth-stop attempt timed out while rapid
stair changes continued at a fixed game clock. Their FAILED/process/tail receipts
are retained under failed-pilots and excluded. The accepted lunal case deliberately
stops at its first descent to21. It establishes real entry/decision/action,
not successful unbounded scumming. Glyph cases stop after the creation window;
no completed autonomous Sea of Runes or win is claimed.

The final native C state probe is read-only and separately built. Its keys,
messages, player state and serialized engine bytes exactly match the normal C
run in all six cases. On the prelaid case it observes25 engine glyphs,23 visible,
but zero Borg map glyph tiles, one tracked cast and morgoth_position false.
Ordinary glyphs lack TRAP, while the discovery guard requires a disarmable player
trap. This corroborates existing tracker#295; it is separate from reverse-flow
lead#345. No source correction, combat benefit or independent tracker verification
is claimed. The probe never adds TRAP or writes Borg state.

outcomes.json, function-entries.json, native-c-map-observations.json and raw
C/C/Rust/coverage logs/saves retain the observations. finalsave-controls.log
shows missing/changed final engine saves are rejected even with identical three
logs. The complete139-execution coverage export includes these six profiles;
the original133 baseline and native checker coverage remain separately labelled.
Reproduction commands are in harness/scenarios/README.md.

[Published additional native evidence on existing tracker295](https://github.com/diegoami/neo-angband-tracker/issues/295#issuecomment-6700494859) links184ed94 and was read back. Source-side verification and adoption remain separate.
