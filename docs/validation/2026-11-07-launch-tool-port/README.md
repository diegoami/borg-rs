# Native workflow tools adapted for the Rust Borg

Donor: diegoami/better-borg-c record/replay feature08a5cd019 (merged PR147).
IMPORTED.json pins full source commit/path/hashes and licence/credit. Generic
save helpers and terminal viewer are reused; engine diagnostics are adapted
without C Borg globals. The native C measurement/controller backend is not
linked or imported as a Rust controller. Source donor/worktrees are read only.

Delivered root entry points:
- start-borg.sh NAME / existing run-rust-borg.sh: named X11 launch, shared
  save locks and five backups; --from-backup[=1..5] preserves the current file.
- run-borg-headless.py NAME: automatic actual race/class detection on load,
  saved engine RNG preservation, fresh Borg, three independent budgets,
  frozen recording and atomic guarded native-save publication.
- replay-borg.py RECORDING --turn N: rerun frozen Rust from the original
  start, compare actual streams/saves, then step actual terminal frames.
  --dump works without curses; --recording-only remains UNVERIFIED.

Scope: automated headless recordings only. Human interactive GUI inputs and
C-only private Borg diagnostic fields are not recorded. Diagnostic actual
engine gear/monsters/timed effects are observer output and never extra AI
input. Verification compares keys/messages/player state/frames/configuration,
normalized summaries, final native saves and the exact scheduled world-save
set/bytes, with source/runtime/config/origin hashes. It does not shadow every
private Borg field or all engine state between saved boundaries. Historical
unrecorded games cannot be promoted to exact replay. Replays of deaths or
explicit backup branches are not additional independent survival samples.

## Actual checks

- Ten tool tests pass: five-slot rotation, restore preservation, outside-writer
  refusal, lock conflict, name traversal, real-engine exact replay, changed
  state/engine rejection, wrong-seed divergence, legacy refusal and incomplete
  unverified-only handling. Tests share a real disposable fixture; counts are
  tests, not independent gameplay samples.
- Real named headless birth, save continuation and backup restore pass in
  isolated profiles. A non-Human/non-Warrior Hobbit/Rogue save is detected
  automatically, its saved engine RNG is retained and the recording replays
  exactly. Native metadata reader exits after engine deserialize before any
  Borg/world action and is linked with the actual Rust engine objects.
- A real seed8406 Rogue death is recorded, published as a dead native save,
  and reruns byte-for-byte, including real frames and scheduled world saves.
  Attempting silent dead-save continuation fails. Explicit backup1 restore
  preserves the dead current file and resumes its real last living checkpoint.
- Three private-display GUI save/reload roundtrips pass both exact M4 hashes
  (six stages). A separate actual NamedHero GUI restore passes: prior depth98
  save preserved, depth50 backup loaded/saved, and the same-character headless
  lock conflicts while the GUI holds fd9. No owner display/save/build is used.
- Rust smoke3/3 matches original C logs. 154 C/Rust engine objects are identical
  with actual-link-input proof that no C Borg code is in the Rust executable.
- A recording-disabled control on the same frozen saved-origin executable
  produces identical keys/messages/player state and final save. Frame/engine
  diagnostics are observational, not additional input to the playing Borg.

The new engine options are headless/harness-only and default off: frame logs,
explicit final-state saves (including real death flags) and preserved load
RNG. Original strict finalsave still rejects dead characters. General GUI
save behavior and the M4 tag/receipts are unchanged. The UI wrapper supports
old -uNAME as well as the new named alias; build-only touches no profile.

## Evidence and reproduction

Logs, GUI hash results, immutable recording file/binary hashes and verifier
proofs are retained here. Complete local bundles remain under harness/out/
tools-*; real user bundles go to .angband-rust/recordings. Each bundle includes
its own frozen executable/data/runner/config/origin. Large executable/runtime
copies are not committed to Git. Re-run tools/test_borg_tools.py and the root
headless/replay commands in an isolated BORG_PLAY_DATA_DIR to recollect them.

Early prototype attempts (missing inspector includes and source-pin selection
before the donor feature commit) were corrected before acceptance. No partial
run or failed recorder is labelled a verified replay. The base C tool's full
GUI recorder needs a separate Rust-safe input/clock/file-read bridge; that
capability is explicitly deferred rather than inferred from headless data.
