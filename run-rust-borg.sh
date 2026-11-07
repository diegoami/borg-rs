#!/usr/bin/env bash
# Build and launch interactive Angband with the Rust Borg backend.
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PLAY_BUILD="$PROJECT_ROOT/build/play-rust"
PLAY_DATA="$PROJECT_ROOT/.angband-rust"
BUILD_ONLY=0

case "${1:-}" in
    -h|--help)
        cat <<'EOF'
Usage: ./run-rust-borg.sh [--build-only] [Angband arguments]

Builds the Rust Borg and starts Angband's graphical X11 frontend.
Create or load a character, then press Ctrl-Z followed by z to start the Borg.
Confirm the initial Borg warning if shown. Press any key to stop the Borg.

Saves and settings: .angband-rust/ (preserved between builds).
The initial borg.txt uses the stock settings, with cheat death off.
Pass -n to start a new character, or -uNAME to select another save name.
--build-only builds and checks the executable without opening a window.
Set JOBS to control build parallelism (default: 4).

Requires CMake, a C compiler, Cargo/Rust, and X11 development libraries.
EOF
        exit 0
        ;;
    --build-only) BUILD_ONLY=1; shift ;;
esac

mkdir -p "$PLAY_BUILD"
printf 'Building Angband with the Rust Borg...\n'
CONFIGURE=0
for option in 'BORG_IMPL:STRING=rust' 'SUPPORT_BORG:BOOL=ON' \
    'SUPPORT_X11_FRONTEND:BOOL=ON' 'SUPPORT_BORG_HARNESS:BOOL=OFF' \
    'CMAKE_BUILD_TYPE:STRING=Release' 'SC_INSTALL:BOOL=ON' 'BORG_RS_FEATURES:STRING='; do
    if ! grep -Fxq -- "$option" "$PLAY_BUILD/CMakeCache.txt" 2>/dev/null; then
        CONFIGURE=1
        break
    fi
done
# The build system automatically reconfigures when its CMake inputs change.
if [ "$CONFIGURE" = 1 ]; then
    cmake -S "$PROJECT_ROOT/angband" -B "$PLAY_BUILD" \
    -DCMAKE_BUILD_TYPE=Release -DBORG_IMPL=rust -DSUPPORT_BORG=ON \
    -DSUPPORT_X11_FRONTEND=ON -DSUPPORT_GCU_FRONTEND=OFF \
    -DSUPPORT_NCURSES_FRONTEND=OFF -DSUPPORT_SDL_FRONTEND=OFF \
    -DSUPPORT_SDL2_FRONTEND=OFF -DSUPPORT_BORG_HARNESS=OFF \
    -DSUPPORT_TEST_FRONTEND=OFF -DBUILD_DOC=OFF -DSC_INSTALL=ON -DBORG_RS_FEATURES= \
    >"$PLAY_BUILD/configure.log" 2>&1 || {
        cat "$PLAY_BUILD/configure.log" >&2
        exit 1
    }
fi
cmake --build "$PLAY_BUILD" --parallel "${JOBS:-4}" \
    >"$PLAY_BUILD/build.log" 2>&1 || {
        tail -n 60 "$PLAY_BUILD/build.log" >&2
        exit 1
    }
if [ "$("$PROJECT_ROOT/harness/identity.sh" binary-impl "$PLAY_BUILD/game/angband")" != rust ]; then
    printf 'The built executable could not be verified as the Rust Borg.\n' >&2
    exit 1
fi

if [ "$BUILD_ONLY" = 1 ]; then
    printf 'Built: %s\n' "$PLAY_BUILD/game/angband"
    exit 0
fi

mkdir -p "$PLAY_DATA"/{user,save,archive,panic,scores}
if [ ! -e "$PLAY_DATA/user/borg.txt" ]; then
    cp "$PROJECT_ROOT/angband/src/borg/borg.txt" "$PLAY_DATA/user/borg.txt"
fi
# X11's built-in fixed font works even without the optional bitmap-font packs.
# Saved preferences and per-window ANGBAND_X11_FONT_n settings take precedence.
export ANGBAND_X11_FONT="${ANGBAND_X11_FONT:-fixed}"
printf 'Create/load a character; Ctrl-Z then z starts the Rust Borg. Any key stops it.\n'
cd "$PLAY_BUILD/game"
exec ./angband -mx11 -uRustBorg \
    "-duser=$PLAY_DATA/user" "-dsave=$PLAY_DATA/save" \
    "-darchive=$PLAY_DATA/archive" "-dpanic=$PLAY_DATA/panic" \
    "-dscores=$PLAY_DATA/scores" "$@"
