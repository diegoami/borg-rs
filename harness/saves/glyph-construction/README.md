# Autonomous Rune construction inputs

Three selected engine saves were generated twice identically. The normal
production C/Rust Borgs perform the construction; no private Borg map, goal,
center or tracking field is seeded. Input: level50 Human Warrior with the
existing wizard kit, player(60,33), an open preparation room, awake Morgoth
at(190,55), and other uniques marked dead. Ready has40 known Rune scrolls
and zero glyphs; supply control has9; prelaid control has24 surrounding
warding glyphs and40 scrolls. Ordinary resources/actions create all new Runes.

Use suite glyph-construction and SAVE_ROOT=this directory. The explicit
rune-pattern flag stops on24 actual engine warding glyphs around the player,
before later combat can disturb them. It never creates a Rune or sets a
Borg field. The default frontend does not enable that stop. A separate500
turn bound handles incomplete controls. These are controlled preparation
states, not unassisted gameplay or a win.

generator.json pins native objects and constructor hash; the exact frozen
constructor is fixture-save-generation.c. Later diagnostic-only additions to
the live constructor do not alter these save bytes. SOURCES.json and hashes
bind all inputs. Reproduction details are in the post-M4 investigation receipt.
