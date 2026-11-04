#!/usr/bin/env bash
# Build the harness binary.
#   BORG_IMPL=c    (default) engine + C Borg            -> build/c
#   BORG_IMPL=rust engine + Rust Borg staticlib         -> build/rust
# Usage: harness/build.sh            (uses $BORG_IMPL)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMPL="${BORG_IMPL:-c}"
BUILD="$ROOT/build/$IMPL"
mkdir -p "$ROOT/build"
EXTRA=()
if [ "$IMPL" = rust ]; then
    EXTRA+=(-DBORG_IMPL=rust)
fi
cmake -S "$ROOT/angband" -B "$BUILD" -DCMAKE_BUILD_TYPE=Release \
    -DSUPPORT_BORG_HARNESS=ON "${EXTRA[@]}" >"$BUILD.cmake.log" 2>&1 || {
    cat "$BUILD.cmake.log"; exit 1; }
make -C "$BUILD" -j"$(nproc)" >"$BUILD.make.log" 2>&1 || {
    tail -50 "$BUILD.make.log"; exit 1; }
echo "$BUILD/game/angband"
