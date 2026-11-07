# Synthetic engine scenarios

These fixtures reach the real self-lunal and glyph-builder callers with
unchanged production Borg/engine objects. The six cases compare native C twice
and Rust on keys, messages, player state and complete final engine-save bytes.
No cheat-death flag is enabled. Characters/history/resources are constructed
wizard states; this is bounded path validation, not an unassisted win.

From the project root, build the usual C/Rust harnesses, then run separately:

```sh
python3 harness/scenarios/build.py NEW_GENERATOR_DIRECTORY
python3 harness/scenarios/generate.py NEW_GENERATOR_DIRECTORY/angband NEW_FIXTURE_DIRECTORY
SAVE_ROOT=harness/saves/synthetic-tactics BORG_IMPL=c JOBS=3 harness/suite.sh synthetic-tactics NEW_C1
SAVE_ROOT=harness/saves/synthetic-tactics BORG_IMPL=c JOBS=3 harness/suite.sh synthetic-tactics NEW_C2
SAVE_ROOT=harness/saves/synthetic-tactics BORG_IMPL=rust JOBS=3 harness/suite.sh synthetic-tactics NEW_RUST
harness/diff --suite synthetic-tactics --save-root harness/saves/synthetic-tactics --expect-impl c NEW_C1 NEW_C2
harness/diff --suite synthetic-tactics --save-root harness/saves/synthetic-tactics --expect-impl rust NEW_RUST NEW_C1
python3 harness/scenarios/verify.py NEW_GENERATOR_DIRECTORY/angband NEW_C1 NEW_C2 NEW_RUST NEW_RESULTS.json
```

The generator relinks original native C objects with a test-only save hook.
Its construction runs after the existing wizard fixture setup and before
serialization; no Borg algorithm is replaced or private Borg field forced.
The generator is never used as the production replay executable. Build identity
checks remain strict on every normal C/Rust suite. Regeneration refuses existing
outputs and requires two identical C saves/log sets before DONE.

| case | input/change | measured outcome |
|---|---|---|
| self-lunal ready | depth20/max99, option on |155 thinker entries; native descent to21; configured depth-stop |
| option off | identical engine save, option off |0 thinker entries |
| shallow history | max34, option on |0 thinker entries |
| glyph ready | depth100,20 Rune scrolls |4 builder entries; one real glyph; scrolls20 to19 |
| low supply |9 Rune scrolls |0 builder entries; zero glyphs; scrolls remain9 |
| prelaid ring |24 real engine glyphs,20 scrolls |4 builder entries;25 engine glyphs/19 scrolls; exposes existing#295 |

The original game-turn-only lunal pilot and later-depth-stop attempt timed out
while free stair transitions continued at a fixed game clock. They remain FAILED
and are excluded. The accepted case deliberately stops at the first descent to21;
it does not claim completed unbounded scumming. Glyph runs stop after the first
creation window, not after an entire Sea of Runes or a fight.

inspect_state.py uses native deserialization, reports fields and exits before
any Borg/world action. observe.py records native C Borg fields only at the stop
save boundary and requires normal-C keys/messages/state/save bytes to remain
identical. It is a separate diagnostic executable, not a verified production
suite build. It never writes Borg glyph/goal/tracking state. The prelaid case
shows engine-visible glyphs absent from the Borg map; its existing source
finding is tracker#295. No tactical correction is applied.

coverage_entries.py reads each successful scenario's isolated LLVM profile and
asserts positive/zero entries for the named controls. Use the same frozen Rust
coverage image as the existing corpus; fixture/harness data changes do not
change production inputs. The combined11-group corpus has139 executions.
