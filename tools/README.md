# Rust launch and recording adapters

Workflow source: diegoami/better-borg-c, committed record/replay feature
08a5cd019 (already merged in donor PR147). IMPORTED.json pins full donor
commit/path/SHA. Original licence/credit is preserved. Save helpers and the
terminal-frame viewer are reused; native engine diagnostics are adapted
without reading C Borg globals. The donor repository/worktrees are read only.

The headless Rust backend uses the existing verified harness and an engine-only save
metadata reader relinked with Rust's actual engine/staticlib objects. It never
links the C Borg. Names are validated; GUI/headless locks are shared; backup
restore preserves the current file. Headless publication uses a content check
plus atomic replacement; an outside writer causes refusal rather than overwrite.
GUI play uses the ordinary native save path and retains the character lock
in both the waiting launcher and game process until they exit.
No tests open or modify owner saves.

Headless records live under BORG_PLAY_DATA_DIR/recordings (default
.angband-rust/recordings). Each bundle freezes the executable, runtime data,
runner/supervisor/config, seed/budgets and original save. Loaded games keep
their native engine RNG; a new process starts fresh Borg memory. Seeded
headless calendar context is pinned to the epoch by the existing harness.
This is not restoration of a running Borg or live checkpoint rewind.

GUI recordings use a separate `borg-rs-gui-recording@1` schema. The ordinary
X11 launcher enables the optional Linux engine recorder; ordinary harness
builds leave it off. GUI play freezes executable/data/initial profile and
captures terminal poll results (including empty polls, peek/take, geometry,
disconnects), actual Rust hook output, calendar calls from C and Rust, the
PID used only for RNG initialization, local timezone rules, and file-read
bytes at every native read, including live preference reloads. It records
main-terminal game frames, engine RNG and fork-private native-save world
fingerprints at command/input boundaries. It supplies no future RNG or Borg
decisions during replay. The normal user/preferences directory stays live.
GUI recordings are completed only after successful child exit and the engine
footer. Replay runs the same frozen executable with a dummy display, preserving
the save name, terminal geometry/input conventions and X11 preference identity;
no X server is needed. It compares all external input/action streams, exact
frames (including world fingerprints), completion counts and final native save.
Extra unconsumed input, missing streams, changed files and divergent output fail.

The default replay executes the frozen Rust Borg from its original start and
requires byte-identical keys/messages/player state, terminal frames, final
native save, scheduled world-save set/bytes and run configuration. It also
compares normalized summaries and checks process success and all recorded
file hashes. Cached verification is invalidated by manifest or verifier-code
changes. It does not prove every private Borg field or unseen engine state
between saved boundaries. Actual inventory/monsters/timed effects are
diagnostic-only frame output, not additional AI input; C Borg private fields
are omitted. Verification is scoped to these channels and finite budgets.

--recording-only views incomplete/failed frame streams with an UNVERIFIED
badge; it never promotes them to a verified rerun. Modified files, unsupported
legacy schemas and divergent reruns are refused. Completed run evidence is
retained even if named-save publication loses a race. Death/replay branches
are not new independent natural-death samples.

Both new GUI and headless sessions are replayable. GUI support is Linux/X11
character mode with main-terminal game frames; graphics/auxiliary window pixels
and pre-character birth/menu screens are not visual recordings. External input
and final character state still cover a fresh birth. This is a frozen-origin
rerun, not arbitrary mid-session restoration of private Borg memory. Initial
bundle hashes protect retained evidence, not provide a cryptographic signature.
Frozen executables/data add disk usage per session; long GUI recordings also
include a forked engine snapshot per command/input boundary. No automatic
recording cleanup deletes historical evidence.

```sh
python3 tools/test_borg_tools.py
python3 harness/regress/gui-replay/run.py /tmp/gui-proof --project-root /tmp/borg-rs-test-checkout
BORG_GUI_TEST_RECORDING=/path/to/borg-intervention-recording python3 tools/test_gui_recording.py
./run-borg-headless.py --help
./replay-borg.py --help
```

Tests cover five-slot rotation, restore preservation, outside-writer refusal,
shared locks, name traversal, exact real-engine replay, corrupt state/engine,
wrong replay seed and legacy refusal. Additional real integration checks
exercise named GUI restore/locks, three GUI save/reload roundtrips, death
publication/restore, and automatic non-Human/non-Warrior save continuation.

GUI regression covers six real sessions: load/save, reload, backup restore,
Rust Borg activation/interruption plus manual play/live borg.txt reload/resize,
a loaded Hobbit Rogue, and fresh birth. Negative controls exercise runtime/input/
action/save corruption, incomplete streams and extra terminal input. Tests run
on a private Xvfb display and disposable profiles/build, never owner saves.
