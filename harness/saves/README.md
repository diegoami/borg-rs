# Synthetic endgame starting points

These are wizard-created engine savefiles, not characters played to the
endgame by the Borg. They isolate deep-dungeon starting states for M4 D.
Only the engine/frontend under `BORG_HARNESS` creates them. Neither Borg
implementation is changed. All fixtures are Human Warriors, level 50,
with maximum natural stats, known runes, an artifact kit and healing,
speed, escape, banishment, recall and food supplies. There is no
invulnerability. Depth 100 marks Sauron dead; Morgoth remains alive.

| file | generation seed | starting depth | special setup |
|------|-----------------|----------------|---------------|
| depth50.save | 5000 | 50 | engine-generated dungeon |
| depth98.save | 5001 | 98 | engine-generated dungeon |
| depth99.save | 5002 | 99 | engine-generated dungeon |
| depth100.save | 5003 | 100 | engine-generated dungeon |
| morgoth.save | 5004 | 100 | player moved to an empty square adjacent to the generated Morgoth, who is awakened |

`harness/make-fixtures.sh OUTDIR` requires a current, verified C build. It
generates every save twice, compares the full save bytes, all three logs
and the summaries apart from elapsed time, and writes `SHA256SUMS`, its
recipe and C `BUILD_ID`. It does not install or replace these files.
Reproduction is separate from accepting new fixtures and references.

`harness/suites/endgame` replays these exact files. `save=NAME.save` copies
the fixture into the isolated run directory, records its SHA-256 in
`config.txt`, and omits `-n`. The frontend loads the character, checks its
race/class, and reseeds the engine immediately before starting the Borg.
It records `loaded` and `loaded_turn` in the summary. The turn limit is
absolute engine time, not additional turns after loading. `harness/diff
--suite endgame` requires the recorded hash to match the committed file.
The input files cannot be overwritten by the engine's autosaves.

Limits: one class, one equipment kit, no naturally developed endgame
character, no self-lunal starting point. A depth or a nearby monster alone
does not establish that a particular tactic (for example Sea of Runes)
ran. State such claims only when the replay logs provide evidence.
