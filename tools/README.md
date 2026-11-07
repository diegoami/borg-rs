# Rust launch and recording adapters

Workflow source: diegoami/better-borg-c, committed record/replay feature
08a5cd019 (already merged in donor PR147). IMPORTED.json pins full donor
commit/path/SHA. Original licence/credit is preserved. Save helpers and the
terminal-frame viewer are reused; native engine diagnostics are adapted
without reading C Borg globals. The donor repository/worktrees are read only.

The Rust backend uses the existing verified harness and an engine-only save
metadata reader relinked with Rust's actual engine/staticlib objects. It never
links the C Borg. Names are validated; GUI/headless locks are shared; backup
restore preserves the current file and publication uses a content check plus
atomic replacement. An outside writer causes refusal rather than overwrite.
No tests open or modify owner saves.

Headless records live under BORG_PLAY_DATA_DIR/recordings (default
.angband-rust/recordings). Each bundle freezes the executable, runtime data,
runner/supervisor/config, seed/budgets and original save. Loaded games keep
their native engine RNG; a new process starts fresh Borg memory. Seeded
headless calendar context is pinned to the epoch by the existing harness.
This is not restoration of a running Borg or live checkpoint rewind.

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

Current scope is recorded **automated headless** play and terminal inspection.
The ordinary X11 launcher has names/locks/backups, but its human input stream
is not recorded. Do not label GUI sessions replayable by these adapters.

```sh
python3 tools/test_borg_tools.py
./run-borg-headless.py --help
./replay-borg.py --help
```

Tests cover five-slot rotation, restore preservation, outside-writer refusal,
shared locks, name traversal, exact real-engine replay, corrupt state/engine,
wrong replay seed and legacy refusal. Additional real integration checks
exercise named GUI restore/locks, three GUI save/reload roundtrips, death
publication/restore, and automatic non-Human/non-Warrior save continuation.
