# Standalone recorder integration (#70)

The native recording core is now angband-recorder v0.1.0 at
c4986b26b9fd1c14852df4f1b7393d9030cf812a. `recorder-pin.json` seals every
vendored file. The independent checker lives in borg-rs/check-recorder-pin.py;
Cargo and recorder-enabled CMake builds fail on missing/changed/extra vendor
files. The Rust `recording` feature links through libborg_rs.a. C oracle and
parity builds keep SUPPORT_BORG_RECORDING=OFF; C+recording is rejected.

The existing C hook sites remain. native-replay.c/.h and the shared observation
headers are forwarding adapters to the pin. The recorder owns stream files,
clock/PID/read capture, ordered replay, action checking, observations and the
footer. File-existence checks are now recorded too. Compatibility text streams
and observation fields preserve existing consumer launch/view/replay tools.
The deterministic borgh frontend is recomputed and checked during headless
replay, preserving its Borg activation side effects.

Consumer tools still own frozen build preparation, named characters, death
workflow and parity instrumentation. Source seals and archives now include the
vendor/pin, with a conditional path list for historical pre-extraction commits.
Modern toolkit/takeover adapters leave vendor bytes untouched; historical
recordings still rebuild their own old C recorder. Existing IMPORTED.json
credits/tags and all historical bundles remain unchanged in meaning.

The standalone initial release is a stream-directory format. The one-file
branch tree, rolling saves, in-game menu and portable slim recordings remain
future work in angband-recorder; no interactive-branch feature is claimed here.
