# Interactive Rust launcher and larger layout

`./run-rust-borg.sh` builds the ordinary Rust-backed X11 game, verifies its
Rust identity, and stores saves/preferences in ignored `.angband-rust/`.
The initial stock borg.txt has cheat death FALSE; existing bytes are preserved.
Ctrl-Z then z starts the Borg manually; Ctrl-X saves/exits. Builds are separate
from the frozen coverage image. Production input hash remains unchanged.

The owner requested the appearance of better-borg-c/start-borg.sh. The
launcher now uses its exact28-pixel Courier font selection/override and
explicit one-main-window frontend option. A real Xvfb check from /tmp shows
1362x722 pixels with readable text (retained screenshot and xwininfo output).
The initial fixed-font window was482x314. An intermediate font-only change
still loaded the old6x13 saved tile dimensions and cropped the larger glyphs;
that trial's dimensions are in font-only.log. The explicit one-window option
uses the font-derived tile metrics. An existing saved FONT_0=fixed no longer
keeps the default launcher small. BORG_PLAY_FONT remains the font override.

These are startup/layout checks against the real native C X11 frontend with
the Rust Borg linked. They do not claim an autonomous game or GUI-code source
correction. The saved-preferences path behavior discovered during inspection
is separately recorded as a C source lead; no engine/Borg source is changed.

Save-path correction: the first launcher put -uRustBorg before the -dsave
option. The ordinary engine freezes savefile's path while parsing -u,
so that order retained the missing ./lib/save destination. Directory overrides
now precede -u. For the owner's already-running session, the missing generated
lib/save path was linked to the intended persistent directory; the retry
successfully created .angband-rust/save/RustBorg. No owner save is committed.

The corrected launcher passed a real GUI save/reload roundtrip in two fresh
temporary profiles: load the existing synthetic level50 fixture, Ctrl-X save
and exit0, copy only that save into a second fresh profile, load and save it
again. Both save files are92944 bytes and the regenerated CharOutput verifies
level50 after reload. The test does not open or modify the owner's save.
BORG_PLAY_DATA_DIR enables isolated profiles while the normal default remains
.angband-rust. Logs, hashes and the driver are retained here. Reproduce after
building with `./run-rust-borg.sh --build-only`, then run the driver from a
virtual X display (`xvfb-run -a -s '-screen 0 1600x1000x24' python3
docs/validation/2026-11-07-launcher/save-roundtrip.py`). Xdotool focus warnings
in the unmanaged virtual display do not prevent either exit0/save check.
