# Town strategy integration with main

Owner direction: merge the town/trip feature into main, with both policies
remaining opt-in. Integration starts from main **58debd76b6340585b7a511f16962981a8dac846c**
and feature **71cff24ef45731e55aa9687930cd2d766dd0695e**. PR42/PR83 remain
historical review vehicles and are not used for this merge.

## Integration decisions

- Retain main's accepted M13 carry and recording/cemetery work. Keep the
  recorder and tools CI jobs and append the town/native capability job.
- Resolve the delivered configuration and registry tests consistently with
  the feature's owner-approved retirement of Rust munchkin/lunal modes.
  Old explicit on settings are rejected; off values warn. The C oracle
  retains its historical modes. Historical mode-on oracle cases are not
  claimed as current Rust parity coverage.
- Retain the configurable recording build flag, default off, so normal
  harness builds remain recording-free and recorder builds can opt in.
- Retire only M13 patch0120's Rust lunal-state clear: its entire mode/state
  was removed by the feature. The C M13 implementation remains intact.
- M13 patch0131 fixes the Confuse Monster wand lookup. Current shop utility
  therefore enables its actual wand consumer; the unsupported confusion
  staff and stun wand remain excluded. Native coverage requires positive
  wand control and zero unsupported-device supplies. The M12 review and
  calibration artifacts retain their historical meaning.
- M13 inserted five traits into the native enum. Town serialization keeps
  M12 IDs0..264 stable, appending stun protection and four vulnerabilities
  at265..269; replay translates these IDs into the current engine vector.
  New plan records use schema11; schemas4..10 retain their old projection.
  The v2/v3 row layout is preserved. New traits are only accepted from v11
  plan records. Archived records are not rewritten.

`borg_town_plan=0` and `borg_trip_policy=0` remain the configured and parser
defaults. This code integration does not enable a new policy by default or
claim a survival/progress improvement.

## Existing experiment

[Zoo run43720870587](https://github.com/diegoami/borg-zoo/actions/runs/43720870587)
is pinned to the earlier M12-based feature source71cff24 and Zoo faae32a.
Its six arms and owner-selected return pairs are unchanged. This integration
runs on M13 and is a different build; the existing experiment must not be
silently repinned or used as an M13 performance result.

## Verification

The integration receives current unit/tool/clippy checks, archived plan
replay, native catalogue/transaction fixtures, policy-off C/Rust parity,
opt-in town/trip wiring, and the native recording round trip. Build/run
receipts and any observed limitations are attached after verification.
Historical archived CSV/log whitespace is preserved rather than rewriting
published evidence; runtime-source whitespace is checked separately.
