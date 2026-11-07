# C-compatible turn inspection and Rust death records

Owner request: match better-borg-c's parameter for viewing from a specific
turn, and adopt its death record logging. Donor pinned at f3ced6219768
(merged PR152); source/path/hash/licence receipts are in tools/IMPORTED.json.
Only generic workflow/viewer pieces are adapted; the C measurement backend
and C Borg are not linked into Rust.

## Delivered behavior

Replay supports --from-turn, --dump-turn, --view-only, --verify-only,
--output NEW_DIR and --timeout, preserving --turn/--dump/--recording-only.
The turn selects original observations; verification reruns the whole frozen
origin, preserving prior private Borg history. Retained output is exclusive,
keeps real rerun files/proof and is not overwritten. A dossier path resolves
only to its hash-matching original bundle. Viewer gets the donor's CSI and
application-cursor navigation fixes; Escape returns to the game panel.
Headless --timeout aliases its wall limit and --death-issue attaches discussion.
Rust defaults/extra key/checkpoint controls remain; donor --stall-timeout is
not ported. No claim of fully identical launcher argument sets.

Completed natural terminal deaths from either launcher are automatically
exported under docs/deaths, overridable with BORG_DEATH_RECORDS_DIR. GUI
BORG_DEATH_ISSUE or headless --death-issue supplies an optional discussion;
unassigned stays unassigned. The logger posts no issue, claims no causal
bug/fix and changes no gameplay. Caps, failures, retirement and cheat-enabled
cases are not natural-death dossiers. GUI/mixed control and loaded branches
retain their actual origin and qualification.

Registry allocation is locked; new dossiers are staged and atomically
published. Existing identity/build/input and original observations are not
replaced by later code. Assessment/actions remain curated; discussion metadata
and qualified repeat evidence may be added without dropping old attempts.
Artifacts are hashed and checked on reexport. Aggregation deduplicates process
receipts, groups starting conditions and canonical actions, and does not count
replay/backup branches as independent mortality samples.

Dossiers include actual source/input/binary/config hashes and limits, native
maps/borg.dat/borg-log.txt, bounded key/message/state/action tails, actual
terminal cells, living-checkpoint metadata with explicit load-validation
status, and unassessed findings/feedback/actions. Full raw runs, executables
and living saves remain in the original bundle outside Git. Loading a living
save starts fresh Borg memory; it is not an instant faithful rewind.

Offsets5/12/25/40/60/82/105 are engine world turns. Exact original frames are
selected when present. Missing exact turns are listed unavailable. Separate
nearby anchors retain their actual turn/phase/offset and group requested
offsets sharing a boundary; no interpolation or invented every-turn state.

GUI archives are isolated per session, so concurrent named games cannot mix
Borg.map/death logs. Changed/newly written files are retained in the bundle,
while stale files are not attributed to a new death. Its replay compares
archive bytes exactly. Existing headless engines pin seasonal availability,
but their Rust death log header uses the real wall clock. Their original
bytes/hashes stay immutable; verification records differences only between
recognized timestamp-header lines and requires every other log byte, map,
Borg data file, game log/frame and native save to match exactly. This is an
explicit new comparison qualification, not a weaker game oracle. The changed
native-log-body negative control is rejected even with a rehashed manifest.

The four earlier cohort deaths are migrated without altering the old reports
or receipts: D0001 Warrior8400, D0002 Druid8402, D0003 Rogue8406, D0004
Blackguard8408. Registry: four cases/four inputs/twelve C/C/Rust process
attempts. The original nine-birth denominator (four deaths/five capped alive)
remains in cohort-2026-11-07.md. Old exact build hashes and dirty-source state
are preserved; these cases have no invented movie. No M4 tag/review change.

## Actual validation

- Eighteen death/CLI/navigation controls pass on a real death recording:
  export and original windows/anchors; nondeath/inconsistent summary refusal;
  raw-log corruption; immutable build identity and preserved assessment;
  repeat fidelity and reexport deduplication; duplicate-family/attempt counts;
  dossier manifest resolution and legacy refusal; issue-metadata attachment;
  curated evidence tamper rejection; C-compatible turn aliases; retained
  real verification output/no overwrite; invalid flags; raw CSI/application
  cursor navigation; native death-log body mutation after manifest rehash.
- Existing headless ten and GUI ten controls pass. These are assertions over
  reused fixtures, not independent gameplay populations.
- Fresh headless seed8406 Hobbit Rogue dies naturally at5600, level2/depth1;
  its real dead save is published, latest living checkpoint retained, dossier
  exported automatically and full replay verifies. It groups with the earlier
  Rogue's starting-condition family; this is not another independent seed.
- On private Xvfb, ordinary Rust Borg play from the actual live Druid5002
  checkpoint dies at10668, level1/depth1, to a giant yellow centipede. Ordinary
  quit confirmation completes capture/export. Original and replay match the
  real actions/inputs/frames/RNG/native final save and native maps/death logs.
  GUI shared archive stays untouched. This is a loaded/fresh-Borg diagnostic
  branch, not the old Druid death or a survival improvement. Exact default
  windows are unavailable; seven separately labeled original anchors are at
  turns10660/10650/10640/10620/10600/10580/10560 (actual offsets8/18/28/48/68/88/108).
- The six root-script GUI regressions still pass: load/save and reload retain
  both exact M4 save hashes; backup restore, real Borg interruption/manual
  play/live preferences/resize, loaded Rogue and fresh birth replay exactly.
- Normal C and Rust smoke3/3 match the original oracle on all three channels.
  The no-C-Borg checker passes154 identical engine objects plus actual linked
  inputs. Shell/Python compilation and source whitespace checks pass.

Tests use disposable profiles/build/private display and registry overrides;
no owner save/settings/display/build is used. No artificial kill is used to
claim death evidence. An early GUI driver hit the death menu's quit-confirm
prompt; q/y handling corrected it. An exploratory exact headless death-archive
comparison exposed the wall-clock header, now explicitly qualified above.
A final internal-API check also exposed relative bundle paths reaching the
supervisor as relative executable paths. Normalizing the bundle before launch
fixes it; an actual relative-path control and five consecutive replays pass.
Failed pilots are excluded from accepted receipts.

## Reproduction and retained evidence

```
BORG_PLAY_DATA_DIR=/tmp/rust-death-profile BORG_DEATH_RECORDS_DIR=/tmp/rust-death-records ./run-borg-headless.py Test --new --seed 8406 --race 3 --class 6 --turns 10000 --timeout 60
python3 harness/regress/death-recording/run.py /tmp/gui-death --project-root /tmp/borg-rs-gui-tools
BORG_DEATH_TEST_RECORDING=/path/to/complete/death-bundle python3 -m unittest discover -s tools -p 'test_death_records.py'
./replay-borg.py /path/to/dossier --from-turn 5500
./replay-borg.py /path/to/dossier --dump-turn 5596 --timeout 30 --output /tmp/replay-proof
```

This directory retains reviewed logs, process/gameplay proof metadata,
original bundle manifests/proofs, curated death dossiers and source ledgers. Failed default verification attempts now retain diagnostics
automatically; successful temporary workspaces are removed.
Full frozen bundles remain under harness/out/death-tooling as identified by
results.json. These receipt exports omit executables/data and are not standalone
replay bundles. Existing historical validation receipts are unchanged.
