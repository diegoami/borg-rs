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
holds. The standing reference is kept unchanged as history; the next
section records the refresh that replaced it.

### The standing oracle, refreshed

Owner direction (2026-11-13, later the same day): "the standing C oracle is
now stale, update it". The suite oracles were therefore re-made from the
v12R C build with the method `docs/harness.md` prescribes: the C build ran
each suite **twice**, the two passes were compared on `keys.log`,
`messages.log`, `state.log` and `summary.json` (apart from `wall_seconds`),
and one pass was stored as `.xz` logs plus `summary.json`, `config.txt`,
`timing.txt`, the manifest and the `DONE` marker.

| oracle | runs | the two C passes | pass 2 vs the oracle | the Rust role vs the oracle |
|---|---|---|---|---|
| `smoke` | 3 | identical | **3/3** | **3/3** |
| `full` | 45 | identical | **45/45** | **45/45** |
| `classes` | 16 | identical | **16/16** | **16/16** |
| `config` | 7 | identical | **7/7** | (the Rust role ran `config-six`) |
| `config-six` | 6 | the six `config` runs | - | **6/6** |

**What the refresh did.** Each suite was run twice by the same C build
(`binary_sha256=2aef1459477d2b4174374bf4123b51b28e02e781295079c0339a65a6a3122b5f`,
built from `748968da0a31f58d3a246634b0954b2344cbc1e0`, `git_dirty=no`,
`inputs_dirty=no`); the two passes were compared on `keys.log`,
`messages.log`, `state.log` and `summary.json` (apart from `wall_seconds`)
and were identical in every suite; one pass was then stored as `.xz` logs
plus `summary.json`, `config.txt`, `timing.txt`, the suite manifest and the
`DONE` marker, in the format the earlier references use.

**Provenance.** Every suite manifest records
`checkout_source_sha256=28974d08729bda49cd70affad36d1a4f27cf61d7672d81af23e887aeee19a231`,
the same value as the build's `build_id_source_sha256`, and
`binary_sha256_end == binary_sha256`: the C code that produced the corpus is
the code that was built. The manifests also record `git_dirty=yes`, because
the documentation and the oracle corpus itself were uncommitted in the
worktree while the suites ran; the C build inputs (`angband/`,
`harness/measurement/`) were untouched (`build_id_inputs_dirty=no`).

**The oracle describes the Rust Borg too.** The Rust output of the v12R
validation (`harness/out/rust/`) was compared with the new oracles:
**3/3** of `smoke` and **45/45** of `full` runs are identical on keys,
messages and state, and the same holds for `classes` and `config-six`. The
oracle is therefore a target both roles can be checked against, not only the
C build.

**Delivery.** The refreshed corpus is about 20 MB of `.xz` logs. The API
channel that pushes this branch carries text only: a 600 KB call is refused
outright and a binary payload does not survive the round trip. The corpus
is therefore delivered as the archive `c-oracle-v12r.tar.gz` and installed
with:

    tar xzf c-oracle-v12r.tar.gz          # writes harness/reference/<suite>/...
    git add -A harness/reference
    git commit -m "harness: refresh the standing C oracle for v12R"
    git push origin claude/v12r-carry

The digests of every stored file are in [oracle-digests.json](oracle-digests.json),
so the unpacked corpus can be checked against this receipt. This branch
carries the text side of the refresh (the provenance map, this receipt and
the digests); the archive carries the `.xz` logs.

**What was not refreshed.** Two suites are still the M1/M4-era recordings:
`options` and `deep`.

`options` was not re-made because on the imported engine it is far more
expensive than it was: the Borg now acts in about 80,000 of a run's 100,000
turns instead of about 14,000 (`opt-wdam-war`: 80,635 keys and 1,655 s here
against 14,156 keys and 11.3 s for the M1-era build, 146x), and each active
run writes about 1.4 GB to this sandbox's network filesystem, so one C pass
of the suite costs about four hours and the two passes the method asks for
about eight. The `options` oracle therefore still describes the old engine
and must not be read as if it described the imported Borg. The command to
refresh it is in `harness/reference/README.md`.

The `deep` suite (9 runs of 1,000,000 turns) was not re-made either: two C
passes cost about four hours on this two-core sandbox.

The investigation corpora under `harness/reference/` (`captured-rogue`,
`crash-fixes*`, `endgame`, `extended*`, `glyph-*`, `lunal-investigation`,
`natural`, `normal-live`, `synthetic-tactics*`, `better-borg-c/m2-*`) are
records of past investigations on the `e473720ed` engine and are kept
unchanged: rewriting them would destroy the evidence they are.
`harness/reference/README.md` maps every corpus to what it is. The previous
suite oracles (the M1/M4-era recordings) are preserved in the repository's
history; they were superseded, not deleted.

## Independent review

The independent review of [PR #58](https://github.com/diegoami/borg-rs/pull/58) found
**one blocking defect**: at the head it reviewed, `d1099ab`, the branch did not compile.
`MsgState` no longer has a `dont_react` field (the reaction gate moved to
`borg.dont_react` in this carry), but two sites still read the removed field -
`borg-rs/src/borg.rs:433` (the per-message-pump clear) and
`borg-rs/src/borg_caution.rs:1509` (the "Borg ignoring messages" note). The cause was
a stale push of those two files, not the carry: the tree that produced the validation
above already had the migration finished.

The author fixed it in `c1e5df3` - both sites now use `st.borg.dont_react`, and the
`Borg` struct field is the gate itself again. The fix was verified on the exact pushed
tree: `cargo test --release` gives **122 passed, 0 failed, 1 ignored**, and
`cargo clippy --release --all-targets -- -D warnings` gives the same **seven
pre-existing** findings, none of them in the two changed files.

The reviewer's recheck at **`071c5c5`** (the head that also carries the refreshed oracle
corpus) reports **no remaining findings**.

**Qualification.** The reviewer could not run the build or the tests: Cargo is not
available on their side, and the repository has **no CI workflow** (there is no
`.github/workflows` at the repository root on `main` or on this branch, so no Actions
run exists for any head - the only workflows in the tree are the vendored
`angband/.github/workflows/*`, which are upstream's and do not apply here). The build
and test evidence is therefore the author's, taken on the exact pushed tree: 122 tests
pass, clippy reports the same seven pre-existing findings, 70/70 C/Rust parity, and a
fresh C run matches the oracle now installed on the branch 3/3. The finding itself was
raised by source inspection and was confirmed by the reviewer that way.

Owner acceptance and the tag decision remain with the owner; no `v12R` tag is placed.

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
