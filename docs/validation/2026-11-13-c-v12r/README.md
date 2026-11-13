# Carry better-borg-c milestone 12 as v12R

Owner direction (2026-11-13): carry the consumer's milestone 12
(`4.2.6-impact-v12`, runtime snapshot `4e1b912cbf6d2b8a0243b19ebddfb2d26e7509f2`)
into borg-rs as **v12R**, the Rust counterpart of its one carry unit
(0118 NEWKILL-95) and of the official angband import since impact v11. Record:
[adoption/c-v12r](../../adoption/c-v12r/README.md). This is author validation,
not independent review or acceptance.

Start: Rust main `4ed3263ba340ce907484465f2d3addbdf5eab8ed`
(`4.2.6-impact-v11` merge, after PR #30). Implementation branch
`claude/v12r-carry`, implementation commit `b938452`.

The consumer's milestone 12 was independently reviewed on its side (package
`docs/measurement/IMPACT-MILESTONE12.md`, review map
`docs/review/MILESTONE12.md` in the consumer). The consumer retired its own
carries 0001 and 0002 because upstream now carries that logic, and borg-rs
carries the same retirements.

## Scope

One consumer carry unit and the official import:

- **0118 NEWKILL-95** (`src/borg/borg-flow-kill.c`): `borg_new_kill()` no
  longer forces a newly seen monster's grid to floor/granite from the slot's
  previous race.
- **Official import** (`angband/angband` `42cd156ac`, 12 upstream commits past
  `e473720ed`, 5 of them Borg): the reaction gate is unified on the borg struct
  field `borg.dont_react` (retiring the consumer's carry 0002), and the
  emergency swap compares its candidates by lower danger (retiring the
  consumer's carry 0001).

The C side is the consumer's v12 blobs for the seven changed Borg files, and
the vendored engine moves to the imported `42cd156ac`. The Rust side is three
translations; see the adoption record.

## Identities

- [C-BUILD_ID](C-BUILD_ID)
- [RUST-BUILD_ID](RUST-BUILD_ID)

## Checks

_Pending: filled from the suite runs._

## Limits

- This receipt does **not** rerun the consumer's per-unit native fixtures or
  its 95-group/290-case matrix. Those remain the consumer's native evidence
  for the target; this repository checks the Rust translation against the
  consumer's C blobs through the shared harness.
- No cohort, per-mode effect, survival or progress claim is made. The
  consumer's package records every unit's per-mode effect as not measured.
- The v12R milestone is the Rust counterpart only. No `v12R` tag is placed;
  acceptance and any tag decision remain with the owner.
