# Synthetic engine starting points

Five native-engine saves were generated twice identically, then replayed by
fresh C/Rust Borgs. SOURCES.json records engine values and hashes; generator.json
binds the fixture constructor and every unchanged native C engine/Borg object.
These are wizard-created level50 Human Warriors with the existing artifact/
healing kit, not naturally attained characters or restored private Borg memory.

Lunal states start at depth20 with recorded maximum99 or34. Glyph states start
at depth100 in an open central room with9 or20 known Rune of Protection scrolls.
Morgoth is alive and parked far away; other uniques are marked dead to isolate
preparation. The prelaid case has24 actual engine warding glyphs around the
center. No Borg map/tracking/goal field is seeded. Fixture construction uses
native object/level/terrain APIs plus explicit engine history/quantity fields.

The matching suite is harness/suites/synthetic-tactics. Supply this directory
as SAVE_ROOT to run and --save-root to diff. Full preparation and results are
in harness/scenarios/README.md and the synthetic-tactics validation receipt.
