# Post-M5 death collection and owner-review queue

Owner explicitly requested root produce_death.py and replay_next_death.py
matching the maintained C workflows. Donor pinned at22f817c96843cc45da41dcefb0856ca507d8c66a;
paths/original hashes/licence and adaptations are in tools/IMPORTED.json.
The accepted m5 tag remains0203227, and no C gameplay milestone is adopted.

## Delivered behavior and scope

produce_death.py freezes a committed Rust --ref (default HEAD), prospective
seeds/character choices and world/key/wall/stall/checkpoint limits before any
outcome or build. Defaults match the bounded C workflow where applicable:
3games, random first uint32 seed and consecutive successors,200000world-turn
cap,120wall seconds/game and30seconds without observed world-turn progress.
--seeds excludes --games/--seed. All outcomes/failures remain recorded; no
retrospective seed or budget expansion is performed. Build time is separate.
The collector stops at the first natural death/failure or declared exhaustion.
A cap/key cap/timeout/stall/unexpected Borg stop is never relabeled a death.

--birth fixed uses Human Warrior. Random exploration freezes SHA-256-derived
race/class indices then uses ordinary explicit native menus/default point buy.
This differs from the donor's native random-menu RNG draws; identical seeds do
not assert identical cross-project birth choices or trajectories. Existing
seeded fixed-birth gameplay is preserved. No extra input or RNG is supplied
to an active Borg. Cheat death/live and winner respawn are off; death evidence
is on. Rust adds an independent --keys cap and checkpoint interval.

Source/data snapshots and default builds are cached under primary
build/death-workflow. The archived production tree and source-input hash bind
the selected commit; the snapshot version stamp is explicit build metadata.
--build reuses only a matching ordinary Rust harness (source hash, image,
implementation, features). Current observation runners/config and exact tool source snapshots are frozen
and fingerprinted separately; immutable plan.json/hash and each recording
retain the complete prospective contract; changes or extra/missing cached input files fail
validation. Loaded-tool and per-game integrity checks reject live tool edits.
Failures/incomplete builds are retained, not counted as successful preparation.
Each attempt uses a new private profile and frozen recording outside Git.

replay_next_death.py lists/groups canonical encounter families, including
legacy continuation seed aliases and verified reference-family qualifications.
It skips legacy/missing/corrupt full recordings and chooses the first unreviewed
playable encounter. --case overrides selection. Default verification executes
the original frozen start and retains output; --view-only opens UNVERIFIED
original frames and correctly omits verification-output arguments.

Personal owner review is separate from technical assessment. Only explicit
--mark-reviewed/--mark-unreviewed commands alter the local locked atomic family
ledger. Viewer exit, successful verification and technical analysis do not.
Committed baseline owner marks start empty; C owner marks are not imported.
Tests/private workflows override BORG_DEATH_RECORDS_DIR and
BORG_DEATH_WORKFLOW_DIR. Dossier/assessment bytes remain untouched by marks.

The optional supervisor stall watchdog watches latest complete flushed key-log
world turns, not mere byte growth. Trace buffering means key-boundary progress
is observed, not each internal world tick. A stalled clock is an operational
failure with retained process/checkpoint receipts. Signals target only the
supervisor's own child. Named headless launch supports --stall-timeout too,
default off; legacy frozen runners and recordings keep their old semantics.

## Actual checks

- Nineteen workflow controls pass: prospective seed modes/wrap/limits; varied
  deterministic birth choices; invalid build rejection; source/tool/input
  corruption; bounded first-death/no-death/key-cap/stall/export-failure handling and retirement
  exclusion;
  queue grouping, explicit marking/unmarking, legacy loaded-seed canonicalization,
  missing/corrupt bundles, explicit reviewed-case replay and view-only routing.
  Two controls execute real supervisor children: repeated writes at fixed turn1
  produce status stall/exit125, while advancing turns complete successfully.
  Simulated collection outcomes are unit fixtures, not natural lives.
- Existing ten headless/save controls and nineteen death/CLI controls pass.
  The latter use the newly collected Warrior recording, not a hardcoded cause.
  Existing ten GUI recording controls and four M5 continuation/restore
  regression tests pass. Source Python/shell/whitespace
  checks pass. Normal Rust smoke3/3 matches original keys/messages/state.
- A default clean snapshot build pins Rust source08eaa13100b67a110d5afc1595f0e9586e38613d.
  Declared seeds8400/8401, fixed Human Warrior,100k world-turn cap. Seed8400
  dies naturally to a pseudo-dragon at37680 (level8/depth13); no second game
  is started. The full recording, real dead save, living checkpoints and an
  unassessed dossier are retained. Original normal-live keys/messages/state
  hashes match byte for byte. This is a replay of an existing input, not new
  independent survival evidence or a performance improvement.
- replay_next_death --verify-only selects that dossier and verifies its frozen
  rerun. It creates no owner review mark. A real PTY view-only session displays
  UNVERIFIED and exits on q without marking. Explicit mark/unmark then moves
  the family in/out of the queue, with assessment bytes unchanged.
- Two actual fixed-birth games (seeds42/43,1000-turn cap) produce exactly two
  turn_cap outcomes, with no death/export or extra games.
- Two actual prospectively varied births (seeds123/124) produce the planned
  race/class indices(9,8) and(10,1); both cap with no escalation.
- An actual --keys1 run ends as key_cap, not death. An explicit C --build is
  rejected before any game; its collection remains failed with zero attempts.

All games use isolated profiles/registries; owner saves/settings/display/build
and all review worktrees stay untouched. The pinned clean collector build is
separate. An initial default-build prototype put a newline in the snapshot
version stamp, splitting CMake compiler definitions and failing before play;
that stamp is corrected. The failed build/collection is retained and excluded
from accepted execution. No C/Rust production source changes or new coverage/
win/whole-private-memory claim is made. Full raw evidence remains under
harness/out/death-workflow-validation; Git exports are partial review receipts,
not executable replay bundles. Build caches are private observations, not a
cryptographic attestation.

## Reproduction

```
./produce_death.py --birth fixed --seeds 8400 8401 --turns 100000
./produce_death.py --birth random --seeds 123 124 --turns 1000 --build build/rust
./replay_next_death.py --list --json
./replay_next_death.py --verify-only
./replay_next_death.py --view-only --from-turn 37000
./replay_next_death.py --mark-reviewed D0005
./replay_next_death.py --mark-unreviewed D0005
python3 tools/test_death_workflow.py
```

--output must be a new path under primary harness/out or the explicit workflow
override. --death-issue attaches a discussion URL but posts no issue/comment.
A dossier without a complete original movie remains unavailable. This feature
adds orchestration on top of accepted recording/death tooling, not a new native
Borg strategy or automatic source-target change.
