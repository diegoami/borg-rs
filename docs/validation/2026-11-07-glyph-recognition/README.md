# Post-M4 Rune recognition correction (#295)

The owner explicitly resumed the agreed next-stage work and delegated the
creation of milestone tag m4. That tag identifies reviewed main commit
3923a21, before this correction. Its production inputs, references and
coverage reports remain unchanged. The new behavior is marked FORK in
both implementations and linked to existing tracker finding295.

## Behavior and native engine controls

The former glyph writer was nested under square_isdisarmabletrap, which
requires TRAP. Ordinary Rune and decoy kinds have GLYPH|VISIBLE|FLOOR.
The correction reads the glyph flag from map_info's remembered trap,
independently of harmful-player-trap detection. It does not scan hidden
live traps to learn runes. Rust copies that known flag inside the existing
map_info FFI boundary, removing the old first-live-trap helper.

| bounded scenario | M4 native map glyphs | corrected native map glyphs | control/result |
|---|---:|---:|---|
| known Rune |0|1|one tracked Rune, nine scrolls preserved |
| known decoy |0|1|preserves the C GLYPH category, nine scrolls preserved |
| revealed harmful pit |0|0|one harmful-trap map flag before and after |
| Rune behind opaque wall |0|0|real engine Rune stays unknown, nine scrolls preserved |
| empty floor |0|0|no glyph/trap knowledge invented |
| prelaid 24-rune ring |0|24|24 tracked glyphs, Morgoth position true; 20 scrolls preserved |

The M4 ring outcome was 25 engine glyphs, 19 scrolls, map0/tracked1 and
Morgoth position false. Corrected C/Rust preserve the prelaid 24 glyphs and
20 scrolls. This proves recognition of an existing formation; it is not
autonomous construction from empty floor, tactical benefit in combat or a win.
Known Rune/decoy controls can be outside current view at the stop; their
remembered knowledge remains valid. The hidden control was never observed.

All inputs are constructed level50 Human Warriors at depth100, with the
existing artifact/healing kit, Morgoth alive but far away and other uniques
marked dead to isolate preparation. Nine scrolls keep the five isolated
perception controls below the caller's casting threshold. No cheat death,
TRAP bit added to a Rune, or private Borg field is used. The hidden Rune
is behind an opaque wall within the display panel.

The canonical saves were generated twice at the tagged M4 inputs. Existing
low-supply/prelaid saves match the M4 copies byte for byte. The same saved
bytes drive before/after native C and corrected Rust. Each observer is a
separately relinked diagnostic executable using its source's actual native
C objects. It reads fields only at the final-save boundary; all keys,
messages, player-state and complete final engine-save bytes match its
unobserved normal C run in every case, both before and after correction.
Generator metadata pins constructor, link inputs and original object hashes.
Private map counts are sampled in native C. Rust is checked through source
review and complete key/message/player-state/final-save parity; this receipt
does not claim a separate running-Rust private-field observer.

The six new cases pass C/C and Rust/C with complete final saves. The original
six synthetic cases also pass C/C/Rust with the corrected recognition outcome.
The current strict diff rechecks both exported corrected C2 references:
glyph-recognition and synthetic-tactics-glyph-295. Original M4 oracles stay
at their original paths. Outcomes JSON and raw compressed logs/process/save
receipts are retained here. No coverage percentage is transferred from the
M4 image to this new production source.

## Reproduce

Build both normal harness implementations. Generate fixtures separately, or
use the committed six selected saves in harness/saves/glyph-recognition:

```sh
python3 harness/scenarios/build.py NEW_GENERATOR
python3 harness/scenarios/generate.py NEW_GENERATOR/angband NEW_FIXTURES --glyph-recognition
SAVE_ROOT=harness/saves/glyph-recognition BORG_IMPL=c JOBS=3 harness/suite.sh glyph-recognition NEW_C1
SAVE_ROOT=harness/saves/glyph-recognition BORG_IMPL=c JOBS=3 harness/suite.sh glyph-recognition NEW_C2
SAVE_ROOT=harness/saves/glyph-recognition BORG_IMPL=rust JOBS=3 harness/suite.sh glyph-recognition NEW_RUST
harness/diff --suite glyph-recognition --save-root harness/saves/glyph-recognition --expect-impl c NEW_C1 NEW_C2
harness/diff --suite glyph-recognition --save-root harness/saves/glyph-recognition --expect-impl rust NEW_RUST NEW_C1
python3 harness/scenarios/observe.py NEW_GENERATOR/angband NEW_C1 NEW_OBSERVED --suite glyph-recognition --save-root harness/saves/glyph-recognition
```

For the before control, use an isolated checkout at m4 and copy only the
fixture constructor/generator, observer and glyph-recognition suite there;
leave production source unchanged. Build its C harness and relink its own
diagnostic generator, then replay the same saves. Do not bypass suite source
identity or label a diagnostic executable as a verified production build.

The retained proof can be checked without rerunning games (choose a fresh
output JSON path):

```sh
python3 harness/scenarios/verify_recognition.py \
  docs/validation/2026-11-07-glyph-recognition/before-c \
  docs/validation/2026-11-07-glyph-recognition/before-observer \
  docs/validation/2026-11-07-glyph-recognition/c1 \
  docs/validation/2026-11-07-glyph-recognition/c2 \
  docs/validation/2026-11-07-glyph-recognition/rust \
  docs/validation/2026-11-07-glyph-recognition/after-observer \
  NEW_OUTCOMES.json
```

This requires the M4 before state to fail Rune recognition and the corrected
state to pass all six outcomes, as well as direct C/C/Rust stream and both
observer noninterference comparisons. Separate broader checkpoint results
are recorded alongside these targeted results.

The fresh existing-reference checkpoint passes in both C and Rust:

| suite | C/reference | Rust/reference |
|---|---:|---:|
| smoke |3/3|3/3|
| full |45/45|45/45|
| deep |9/9|9/9|
| classes |16/16|16/16|
| config |7/7|7/7|
| options |25/25|25/25|
| captured-rogue |8/8|8/8|
| crash-fixes |6/6|6/6|
| natural |2/2|2/2|
| total |121/121|121/121|

Every group has DONE, successful process receipts, current source/image
identity and no overrides. checkpoint/ retains group manifests and strict
three-log comparison outputs; checkpoint-metadata.tar.xz retains every
per-case summary and process/config/stderr file;
RUN_LOG_HASHES.json binds all complete raw streams, whose identical reference
bytes remain in harness/reference. New targeted streams/final saves are
retained directly here. Both checkpoint orchestrators exit0. The extended12
long games and coverage were not recollected for this correction; their
verified M4 results stay pinned to m4. This is a focused correction checkpoint,
not a new claim of 139 measured executions for the later source.

All nine fixtures were also generated twice with corrected C; every full save
matches the tagged M4 generation byte for byte. The unchanged builder's
1,600 native C/Rust cases pass, release tests pass100/one existing ignored,
all-target release clippy with warnings denied passes, and all154 engine
objects match with actual-link-input proof that Rust contains no C Borg.
The ordinary interactive Rust build was refreshed with --build-only, before
the script's data-directory creation or any game launch. Unsafe count is
267 (239 engine/28 headers), down from268 after deleting the old helper.
An optional repository-wide rustfmt check reports existing formatting debt
across untouched files; no repository-wide formatting change is included.
