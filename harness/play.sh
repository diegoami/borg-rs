#!/usr/bin/env bash
# An interactive session: the Borg plays the dungeon, the player plays the town.
#
# Usage: harness/play.sh OUTDIR [CHARACTERS] [FLAGS] [OPTIONS]
#
#   OUTDIR      where the session is recorded (it must not exist)
#   CHARACTERS  how many characters to play (default 1)
#   FLAGS       NAME=VALUE rewrites one line of harness/borg.txt, the way
#               run.sh's config flags do: "NAME=VALUE,NAME=VALUE".
#
# Options (X11 only):
#
#   --font-size=N  the font size in pixels (default 30). The font is a
#                  scalable one, so the size is a number, not a name from a
#                  fixed list.
#   --font=NAME    the X11 font name to use instead of the scalable default
#   --cols=N       the main window's width in cells (default 80)
#   --rows=N       the main window's height in cells (default 30)
#
# The window's pixel size follows from the font metrics and cell count.
# The window can be resized in the window manager afterwards.
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
#   objects.log   the objects the run touched: what the Borg used, bought
#                 and picked up, numbered. The same lines are printed on
#                 the console while the run goes on.
#   feedback.md   the owner's feedback on the run, asked for when the
#                 character ends: how the run went, what they thought of
#                 the buying strategy, and the corrections.
#   corrections.jsonl
#                 the corrections on their own, one record each, with the
#                 number of the object log line they are about.
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
# modern screen.
#
# The default font is scalable Adobe Courier at 30px, with 80x30 cells.
# --font-size=N changes Courier's pixel size; --font=NAME selects another
# X11 font. --cols=N and --rows=N change the main window's cell count.
# Pixel dimensions depend on the loaded font's actual character metrics.
#
# Each setting is a default and not a requirement: a value already in the
# environment wins, so ANGBAND_X11_FONT (or ANGBAND_X11_FONT_0),
# ANGBAND_X11_COLS_0 and ANGBAND_X11_ROWS_0 are left alone.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMPL="${BORG_IMPL:-rust}"
UI_MODE="${BORG_UI_MODE:-x11}"
BIN="${ANGBAND_BIN:-$ROOT/build/$IMPL/game/angband}"
[ -x "$BIN" ] || { echo "missing $BIN (run harness/build.sh)" >&2; exit 2; }

FONT_SIZE="${BORG_FONT_SIZE:-30}"
FONT_NAME="${BORG_FONT_NAME:-}"
FONT_COLS="${BORG_FONT_COLS:-80}"
FONT_ROWS="${BORG_FONT_ROWS:-30}"

# The options are parsed before the positional arguments, so they can be
# written in any order.
ARGS=()
while [ $# -gt 0 ]; do
    case "$1" in
        --font-size=*)
            FONT_SIZE="${1#*=}"
            [[ "$FONT_SIZE" =~ ^[0-9]+$ ]] && [ "$FONT_SIZE" -ge 8 ] || {
                echo "play.sh: --font-size must be a number >= 8" >&2; exit 2; }
            shift;;
        --font=*)
            FONT_NAME="${1#*=}"
            shift;;
        --cols=*)
            FONT_COLS="${1#*=}"
            [[ "$FONT_COLS" =~ ^[0-9]+$ ]] || {
                echo "play.sh: --cols must be a number" >&2; exit 2; }
            shift;;
        --rows=*)
            FONT_ROWS="${1#*=}"
            [[ "$FONT_ROWS" =~ ^[0-9]+$ ]] || {
                echo "play.sh: --rows must be a number" >&2; exit 2; }
            shift;;
        --) shift; break;;
        *) ARGS+=("$1"); shift;;
    esac
done
set -- "${ARGS[@]+"${ARGS[@]}"}"

if [ "$UI_MODE" = "x11" ]; then
    # Scalable Courier keeps --font-size independent of bitmap font sizes.
    if [ -z "${ANGBAND_X11_FONT:-}" ]; then
        if [ -n "$FONT_NAME" ]; then
            ANGBAND_X11_FONT="$FONT_NAME"
        else
            ANGBAND_X11_FONT="-adobe-courier-medium-r-normal--${FONT_SIZE}-*-*-*-p-*-iso8859-1"
        fi
        export ANGBAND_X11_FONT
    fi
    # The cell count: the default keeps the window inside a 2560 screen at
    # the default font size.
    if [ -z "${ANGBAND_X11_COLS_0:-}" ]; then
        ANGBAND_X11_COLS_0="${FONT_COLS:-80}"
        export ANGBAND_X11_COLS_0
    fi
    if [ -z "${ANGBAND_X11_ROWS_0:-}" ]; then
        ANGBAND_X11_ROWS_0="${FONT_ROWS:-30}"
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
    # The game's options (the store's direct buy, for one) can be set from a
    # file: the game reads customized_interface_options.txt when the player
    # is made, so the file has to be there before the game starts.
    if [ -n "${BORG_OPTIONS_FILE:-}" ]; then
        [ -f "$BORG_OPTIONS_FILE" ] || {
            echo "play.sh: no such options file: $BORG_OPTIONS_FILE" >&2; exit 2; }
        cp "$BORG_OPTIONS_FILE" "$DIR/user/customized_interface_options.txt"
    fi
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
        echo "font=${ANGBAND_X11_FONT:-<driver default>}"
        echo "cols=${ANGBAND_X11_COLS_0:-<driver default>}"
        echo "rows=${ANGBAND_X11_ROWS_0:-<driver default>}"
        echo "borg_txt_sha256=$(sha256sum < "$DIR/user/borg.txt" | cut -d' ' -f1)"
    } > "$DIR/config.txt"

    echo "play.sh: character $c of $CHARACTERS - CTRL-Z, then y, then z starts the Borg."
    echo "play.sh: the player has the controls in town; the Borg plays the dungeon."
    BORG_TOWN_PLAYER=1 \
    BORG_RECORD_DIR="$DIR/native" \
    BORG_TOWN_STATE="$DIR/state.jsonl" \
    BORG_TOWN_DUMP="$DIR/dump.jsonl" \
    BORG_TOWN_DUMP_INTEGERS="$DIR/dump.i32.jsonl" \
    BORG_OBJECT_LOG="$DIR/objects.log" \
        "$BIN" "-m$UI_MODE" -uBorg \
        -duser="$DIR/user" -dsave="$DIR/save" -darchive="$DIR/archive" \
        -dpanic="$DIR/panic" -dscores="$DIR/scores" \
        || echo "play.sh: character $c ended with rc=$?"
    # The end of the run: the owner's feedback on it.
    bash "$ROOT/harness/feedback.sh" "$DIR"
done
echo "play.sh: done - $OUT"
