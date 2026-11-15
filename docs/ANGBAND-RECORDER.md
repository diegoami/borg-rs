# Standalone recorder integration (#70)

The native recording core is angband-recorder v0.2.0 at
ee8b133ad22f66e7dad6e1b63a2d3e40b9a41ebd (adoption #89). `recorder-pin.json` seals every
vendored file. The independent checker lives in borg-rs/check-recorder-pin.py;
Cargo and recorder-enabled CMake builds fail on missing/changed/extra vendor
files. The Rust `recording` feature links through libborg_rs.a. C oracle and
parity builds keep SUPPORT_BORG_RECORDING=OFF; C+recording is rejected.

The existing C hook sites remain. native-replay.c/.h and the shared observation
headers are forwarding adapters to the pin. The recorder owns stream files,
clock/PID/read capture, ordered replay, action checking, observations and the
footer. Early initialization runs before frontend configuration. File-date
and save-menu directory results are recorded alongside file reads and existence
checks. The session's own save target is live output state, checked directly
in both capture and replay. Compatibility text streams
and observation fields preserve existing consumer launch/view/replay tools.
The deterministic borgh frontend is recomputed and checked during headless
replay, preserving its Borg activation side effects.

New native streams use `angband-recorder-streams@2`; v0.1.0 recordings require
their original frozen v0.1.0 runtime and reader. The adoption does not rewrite
historical captures. The upstream Cargo package version remains `0.1.0` in
the immutable v0.2.0 release; the tag, commit and file manifest identify this pin.

Consumer tools still own frozen build preparation, named characters, death
workflow and parity instrumentation. Source seals and archives now include the
vendor/pin, with a conditional path list for historical pre-extraction commits.
Modern toolkit/takeover adapters leave vendor bytes untouched; historical
recordings still rebuild their own old C recorder. Existing IMPORTED.json
credits/tags and all historical bundles remain unchanged in meaning.

The standalone initial release is a stream-directory format. The one-file
branch tree, rolling saves, in-game menu and portable slim recordings remain
future work in angband-recorder; no interactive-branch feature is claimed here.

Existing harness checkpoint children explicitly call ar_snapshot_enter before
saving. Otherwise a child can append to inherited recording streams or consume
its own copy of a buffered replay cursor. The integration test exercises actual
forked checkpoints and compares their save bytes as well as native streams.
It covers fresh birth and a loaded save, including replay with the original
live save target removed and a panic-recovery file-date decision.
