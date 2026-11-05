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
case "$IMPL" in c|rust) ;; *) echo "BORG_IMPL must be c or rust" >&2; exit 2;; esac
# Always pass BORG_IMPL: it is a CMake cache variable, so a build dir once
# configured with another value would otherwise keep it.
cmake -S "$ROOT/angband" -B "$BUILD" -DCMAKE_BUILD_TYPE=Release \
    -DSUPPORT_BORG_HARNESS=ON -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
    -DBORG_IMPL="$IMPL" >"$BUILD.cmake.log" 2>&1 || {
    cat "$BUILD.cmake.log"; exit 1; }
make -C "$BUILD" -j"$(nproc)" >"$BUILD.make.log" 2>&1 || {
    tail -50 "$BUILD.make.log"; exit 1; }
echo "$BUILD/game/angband"
