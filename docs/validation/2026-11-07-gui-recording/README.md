# Interactive Rust Borg recording and verified replay

The remaining requested tool import is complete. Root `start-borg.sh NAME`
and `run-rust-borg.sh` now record actual X11 sessions by default, print the
recording/replay command and retain the ordinary named save/settings workflow.
Named locks, five backups and restore preservation remain shared with headless
play. The waiting parent and its sole game child retain the character lock;
shutdown handling terminates only that child and waits before releasing it.

Generic recorder adapted from better-borg-c commit08a5cd019 (merged PR147),
with source/licence in tools/IMPORTED.json. No C Borg globals or controller
are linked. `SUPPORT_BORG_RECORDING` defaults OFF; the GUI launcher enables it
and the dummy test frontend. Ordinary harness builds remain unchanged. The
unit executables also link the actual Rust staticlib when using Rust core
objects; recording support/wrapped time links propagate to those targets.

GUI bundles freeze executable, game data, initial profile/save and timezone
rules; capture terminal polls/events (including empty polls, peek/take,
geometry/mapping and disconnect), actual Rust Borg hook output, calendar
reads from both C and Rust, the PID used in engine RNG, and file-read bytes
including live preference reloads. Replay executes actual Rust with those
external inputs; it supplies no future engine RNG state or Borg decisions.
Main-terminal frames contain engine RNG, diagnostics and world fingerprints
from native saves in fork-private children. Such saving cannot compact the
live monster list, and child snapshots do not overwrite CharOutput.txt.
The terminal-before-input boundary is recorded so the regression waits for
actual human input readiness after stopping Borg.

Verification requires successful process completion, exact external input
and action streams, file-read blob set/bytes, frames/world fingerprints,
completion counts and final native save. Unconsumed suffix input fails.
All retained files are hashed; missing required streams or a changed file
set fail. Cached verification binds manifest and verifier/helper source.
The headless schema and older complete frozen headless bundles still work.
Incomplete streams require explicit --recording-only and show UNVERIFIED.

## Checks

Six actual GUI sessions pass through root scripts in disposable checkout
/tmp/borg-rs-gui-tools, on a private fixed-number Xvfb server, with isolated
profiles under harness/out/gui-tooling/integration-final:

1. Existing Warrior load/save; exact accepted M4 hash59cb56c6…ab759.
2. Saved Warrior reload/save; exact M4 hash3920bea0…29bd9c.
3. Explicit backup1 restore: prior depth98 file preserved byte-identically
   as before-restore, actual depth50 backup loads/saves and replays exactly.
4. Actual Rust Borg activation (10 returned keys), manual interruption,
   manual wait, live borg.txt reload and resize from80x24 to106x32. Every
   stream/frame/final save matches; the late comment in borg.txt appears in
   a captured read blob. Final turn10, level50/depth50; no survival claim.
5. Loaded Hobbit Rogue: turn100001, level6/depth1, exact replay/native save.
6. Fresh birth: no starting save, level1/depth0, manual commands to turn11,
   exact replay despite different replay PID/time. Borg memory is fresh.

Ten GUI controls pass, using the real Borg-intervention recording: original
rerun, changed runtime, missing events, extra suffix events (with rehashed
manifest), exhausted clock, changed action (rehash), changed saved-origin
read bytes (rehash), changed final native save (rehash), incomplete stream
unverified-only handling, and omitted required manifest stream. The initial
prototype's first file-input mutation changed the intro/news text and did
not affect the observed game; the final meaningful control corrupts the
actual save input. No claim is made that semantically irrelevant file edits
must change gameplay.

The observer-disabled control recompiles only record_frame with an early
return in the disposable checkout. The same external inputs are consumed,
with zero frame rows, and exact events/clock/PID/actions/file-input/geometry
streams plus native final save match the observed run. Original and control
binary hashes and results are retained. Production source is restored and
rebuilt immediately; no such bypass exists in delivered source.

Ten existing real headless tool tests pass. C and Rust smoke3/3 match the
original C reference on keys/messages/player state, with no identity override.
The normal no-C-Borg checker confirms154 byte-identical engine objects and
actual link inputs. GUI ELF identifies as Rust and its compile/link recipes
contain zero src/borg C objects. The recording-enabled native CMake unit
path-normalize target builds and passes7/7. Shell syntax/Python compilation
and source whitespace checks pass. No Borg tactical Rust source changes.

## Scope, receipts and reproduction

This is Linux/X11 character-mode recording of main-terminal game frames.
Birth/menu screens before character generation and graphics/auxiliary window
pixels are not visual recordings; external events and final state cover the
birth. Private Borg memory is not shadowed. GUI intermediate worlds are
64-bit native-save fingerprints in the compared frames; final saves are
compared in full. Replay needs a compatible Linux runtime and no display.
A long session adds snapshots/frames and a frozen runtime to disk. Replays,
restores and constructed fixtures are not independent survival samples.
The immutable M4 tag and historical receipts are unchanged.

The manifests, exact proof metadata, compressed frame streams, event/action/
clock ledgers, final native saves, tool logs and source/binary identities are
archived here. These are review receipts, not full standalone replay bundles;
the complete executable/data/input blobs remain in the local directories named
by results.json. Do not run replay-borg.py on a partial receipt export.
No owner character, preferences, display or playable build is used in testing;
other review worktrees remain untouched.

Reproduce in an isolated checkout/profile with Xvfb and xdotool installed:

```
python3 harness/regress/gui-replay/run.py /tmp/gui-proof --project-root /tmp/borg-rs-gui-tools
BORG_GUI_TEST_RECORDING=/path/to/borg-intervention-bundle python3 tools/test_gui_recording.py
python3 tools/test_borg_tools.py
./replay-borg.py /path/to/complete/gui-bundle --verify-only
./replay-borg.py /path/to/complete/gui-bundle --turn 10 --dump
```

Earlier prototypes exposed test-driver timing (messages consume keys, initial
Borg warnings require both pages, and keys queued before Borg stops are
flushed), the test frontend clearing the selected save name, the native `-d`
option spelling, and the Rust CMake unit link omission. These are corrected;
failed/partial pilots are excluded from the accepted six-case corpus.
