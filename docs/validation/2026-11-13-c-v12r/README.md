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

### Rust and link checks

- `cargo test --release --manifest-path borg-rs/Cargo.toml`: **122 passed, 0
  failed, 1 ignored** (the existing ignored test). The new test
  `borg_item_wear::tests::emergency_swap_chooses_the_lower_danger` pins the
  carried swap rule.
- `cargo clippy --release --all-targets --manifest-path borg-rs/Cargo.toml --
  -D warnings`: **7 errors, all pre-existing**. The same seven errors, with
  the same texts, appear on the base commit `4ed3263` with this sandbox's
  toolchain (rustc/clippy 1.100.0); they are in the family-lane code
  (`needless_range_loop`, `if_then_some_else_none`-style lints) and none of
  them is in the files this carry changes. The carry adds no clippy finding.
  The v11R receipt was written against rustc 1.98.1, where these lints are
  not raised.
- `harness/check-no-c-borg.sh`: **OK**; the Rust binary links no C Borg
  (154/154 engine objects byte-identical between the C and the Rust build).

### C/Rust parity

Both roles are clean builds of the commits named in the build identities
above (`git_dirty=no`, `inputs_dirty=no` in both), on the same pinned engine
and data.

| Suite | Cases | C/Rust parity |
|---|---|---|
| smoke | 3 | **3/3 identical** on keys, messages and state |
| full | 45 | **45/45 identical** on keys, messages and state |
| classes | 16 | **16/16 identical** on keys, messages and state |
| config-six | 6 | **6/6 identical** on keys, messages and state |

**70 of 70 runs identical**, with `messages.log` and `state.log` compared in
every run.

The suites are the standard early-game matrix (100k turns per case), the
`classes` suite with the corrected birth letters (the races and classes that
`full` never plays) and the six review runs of the config suite (munchkin
start and lunal mode, three characters each). The digests of every run's
`keys.log`, `messages.log`, `state.log`, `summary.json` and `config.txt`,
with both suite manifests, are in [suite-digests.json](suite-digests.json);
the raw runs stay under `harness/out/` outside Git.

The `config-six` suite is the `config` suite without its seventh run, the
1,000,000-turn `cfg-lunal-deep-warrior` case, which costs about two hours
per role on this sandbox; the suite file is
`harness/suites/config-six`. The `options` suite (25 runs) and the `deep`
suite (9 runs of 1,000,000 turns) were not run for the same reason.

### Source identity

- **C blobs.** Six of the seven changed Borg files are byte-identical to the
  consumer's blobs at `4e1b912cb` (`sha256` in the adoption record's
  `series.json`): `borg-flow-kill.c`, `borg-item-wear.c`,
  `borg-messages-react.c`, `borg-messages-react.h`, `borg-messages.c` and
  `borg.c`. `borg-caution.c` differs only in the position of
  `#include "borg-fight-attack.h"`, which borg-rs keeps in its sorted include
  block; the file is the same length and the difference is that include.
- **Engine tree.** 996 of the 1010 upstream files at `42cd156ac` are
  byte-identical in `angband/`. The 14 that differ are exactly borg-rs's own
  fork corrections and harness hooks (`CMakeLists.txt`, `cmd-wizard.c`,
  `game-world.c`, `main-test.c`, `main.c`, `main.h`, `message.c`,
  `mon-make.c`, `savefile.c`, `ui-game.c`, `ui-input.c`, `ui-term.c`,
  `z-file.c`, `z-rand.c`), and their local deltas are preserved line for
  line through the import: the changed-line set of each file against
  `e473720ed` before the import is identical to its changed-line set against
  `42cd156ac` after it.

### The standing oracle no longer applies

The official import changes engine behaviour, so the standing C reference
(`harness/reference/{smoke,full,classes,config,options}`, recorded on the
`e473720ed` engine) does not describe the imported Borg: the new C build
matches **0/45** of `harness/reference/full`. That is the expected
consequence of the import, not a regression of the carry — the same C build
is byte-identical to the consumer's v12 blobs, and the C/Rust parity above
holds. The standing reference is kept unchanged as history; a future carry
needs a fresh C oracle for the imported engine.

## Limits

- This receipt does **not** rerun the consumer's per-unit native fixtures or
  its 95-group/290-case matrix. Those remain the consumer's native evidence
  for the target; this repository checks the Rust translation against the
  consumer's C blobs through the shared harness.
- The `deep` suite (9 runs of 1,000,000 turns) and the `options` suite (25
  runs) were **not** run: on this two-core sandbox they cost about two hours
  and about forty minutes per role, and the suites above already cover the
  carried changes as a regression check. The `config` suite's seventh run
  (1,000,000 turns) was replaced by the six-run `config-six` suite for the
  same reason. The suites are a whole-program parity check, not a per-unit
  oracle: the changed branches (a newly seen wall-walker, an emergency swap,
  a knockout) are rare events, and the consumer's own fixtures remain the
  per-unit evidence.
- No cohort, per-mode effect, survival or progress claim is made. The
  consumer's package records every unit's per-mode effect as not measured.
- The v12R milestone is the Rust counterpart only. No `v12R` tag is placed;
  acceptance and any tag decision remain with the owner.
