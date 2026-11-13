#!/usr/bin/env bash
# An interactive session: the Borg plays the dungeon, the player plays the town.
#
# Usage: harness/play.sh OUTDIR [CHARACTERS] [FLAGS]
#
#   OUTDIR      where the session is recorded (it must not exist)
#   CHARACTERS  how many characters to play (default 1)
#   FLAGS       NAME=VALUE rewrites one line of harness/borg.txt, the way
#               run.sh's config flags do: "NAME=VALUE,NAME=VALUE".
#
# The handover (`BORG_TOWN_PLAYER=1`): while the character is in town, the
# Borg's input hook gives the keyboard back and the engine reads it, so the
# player has the controls in town and the Borg has them in the dungeon. The
# Borg does nothing at all in town - no shop, no equipment, no decision - so
# the player's keys and the Borg's never mix. The player recalls, shops and
# dives; the Borg plays the dungeon and recalls when it wants to trade.
#
# In the game: CTRL-Z asks whether to use the Borg commands (y), then the
# Borg command prompt takes "z" to activate the Borg. The handover starts
# as soon as the Borg is active and the character is in town.
#
# What is recorded, per character, in OUTDIR/char-N:
#
#   native/       the native recording: the inputs, the frames and the
#                 messages. The player's purchases are in the messages
#                 (native-messages.jsonl), and the frames carry the actual
#                 state of the character at every frame. This needs a build
#                 with the recording: BORG_RECORDING=1 harness/build.sh
#   state.jsonl   the states the character was in, in town, in the integer
#                 record's shape (schema version 3: the state on its own,
#                 with no decision in it).
#   dump.jsonl    the Borg's own town decisions (none: with the handover the
#                 Borg makes no decision in town).
#
# The player quits the game when the character is done; the next character
# starts then, in its own directory.
#
# The front end: the interactive front end this build has is X11 (there is
# no terminal front end in the cmake build), so the game needs a display.
# BORG_UI_MODE=x11 is the default.
#
# The font and the window size. The X11 driver's own default for the main
# window is 10x20, the legacy bitmap font, at 80x24 cells - small on a
# modern screen. ANGBAND_X11_FONT overrides the font for every window;
# the default here is 12x24, the largest of the legacy bitmaps xfonts-base
# ships. ANGBAND_X11_COLS_0 and ANGBAND_X11_ROWS_0 set the main window's
# cell count; the default here is 120x40.
#
# Each is a default, not a requirement: a value already in the environment
# wins, so an ANGBAND_X11_FONT=10x20 (or any other) is left alone.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMPL="${BORG_IMPL:-rust}"
UI_MODE="${BORG_UI_MODE:-x11}"
BIN="${ANGBAND_BIN:-$ROOT/build/$IMPL/game/angband}"
[ -x "$BIN" ] || { echo "missing $BIN (run harness/build.sh)" >&2; exit 2; }
if [ "$UI_MODE" = "x11" ]; then
    if [ -z "${ANGBAND_X11_FONT:-}" ]; then
        ANGBAND_X11_FONT=12x24
        export ANGBAND_X11_FONT
    fi
    if [ -z "${ANGBAND_X11_COLS_0:-}" ]; then
        ANGBAND_X11_COLS_0=120
        export ANGBAND_X11_COLS_0
    fi
    if [ -z "${ANGBAND_X11_ROWS_0:-}" ]; then
        ANGBAND_X11_ROWS_0=40
        export ANGBAND_X11_ROWS_0
    fi
fi
OUT="$(realpath -m "$1")"; shift
CHARACTERS=1
if [ $# -gt 0 ] && [[ "$1" =~ ^[0-9]+$ ]]; then CHARACTERS="$1"; shift; fi
FLAGS=""
if [ $# -gt 0 ]; then FLAGS="$1"; shift; fi
[ $# -eq 0 ] || { echo "play.sh: too many arguments" >&2; exit 2; }
[ ! -e "$OUT" ] || { echo "play.sh: output exists; use a new directory: $OUT" >&2; exit 2; }
mkdir -p "$OUT"

# The native recording is an opt-in build (the C side's SUPPORT_BORG_RECORDING).
# The grep is not in a pipeline: `grep -q` exits as soon as it matches, which
# kills `strings` with SIGPIPE, and under `pipefail` that makes the whole
# pipeline look like a failure - a recording build would then be reported as
# having no recording. The match is taken with a command substitution.
if grep -qa "Cannot open recording" "$BIN"; then
    RECORDING=yes
else
    RECORDING=no
    echo "play.sh: this build has no native recording (BORG_RECORDING=1 harness/build.sh)."
    echo "play.sh: the state dump still works; the player's inputs will not be logged."
fi

# The pinned config, with the player's flags applied.
SET_SED=()
SET_CHECK=()
if [ -n "$FLAGS" ]; then
    IFS=, read -r -a FL <<< "$FLAGS"
    for fl in "${FL[@]}"; do
        name="${fl%%=*}"
        val="${fl#*=}"
        n=$(grep -c "^$name = " "$ROOT/harness/borg.txt" || true)
        [ "$n" = 1 ] || {
            echo "play.sh: flag '$fl': $n lines '$name = ' in harness/borg.txt" >&2; exit 2; }
        SET_SED+=(-e "s/^$name = .*/$name = $val/")
        SET_CHECK+=("$name = $val")
    done
fi

cd "$(dirname "$BIN")"   # the game finds ./lib relative to the cwd
for ((c = 1; c <= CHARACTERS; c++)); do
    DIR="$OUT/char-$c"
    mkdir -p "$DIR"/{user,save,archive,panic,scores,native}
    cp "$ROOT/harness/borg.txt" "$DIR/user/borg.txt"
    if [ ${#SET_SED[@]} -gt 0 ]; then
        sed -i "${SET_SED[@]}" "$DIR/user/borg.txt"
        for line in "${SET_CHECK[@]}"; do
            grep -qx "$line" "$DIR/user/borg.txt" || {
                echo "play.sh: no '$line' line in the character's borg.txt" >&2; exit 2; }
        done
    fi
    {
        echo "impl=$IMPL"
        echo "ui_mode=$UI_MODE"
        echo "character=$c"
        echo "flags=$FLAGS"
        echo "native_recording=$RECORDING"
        echo "borg_txt_sha256=$(sha256sum < "$DIR/user/borg.txt" | cut -d' ' -f1)"
    } > "$DIR/config.txt"

    echo "play.sh: character $c of $CHARACTERS - CTRL-Z, then y, then z starts the Borg."
    echo "play.sh: the player has the controls in town; the Borg plays the dungeon."
    BORG_TOWN_PLAYER=1 \
    BORG_RECORD_DIR="$DIR/native" \
    BORG_TOWN_STATE="$DIR/state.jsonl" \
    BORG_TOWN_DUMP="$DIR/dump.jsonl" \
    BORG_TOWN_DUMP_INTEGERS="$DIR/dump.i32.jsonl" \
        "$BIN" "-m$UI_MODE" -uBorg \
        -duser="$DIR/user" -dsave="$DIR/save" -darchive="$DIR/archive" \
        -dpanic="$DIR/panic" -dscores="$DIR/scores" \
        || echo "play.sh: character $c ended with rc=$?"
done
echo "play.sh: done - $OUT"
