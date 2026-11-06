#!/usr/bin/env bash
# Nested Borg entry test (review of 4c06cec08, finding 5; docs/SELF-REVIEW.md
# U1). Builds the Rust Borg with the test-only cargo feature
# test-force-nested (every 500th think step shows two long messages, so the
# engine waits for a key and calls the Borg's input hook again inside the
# running turn) into build/rust-nested, runs the three smoke games and
# checks that every game completes (exit 0, summary.json written, no panic)
# and that nested entries really happened (reported on stderr).
#
# The forced messages change the key stream, so the keys are not compared
# with the references: this tests that a nested entry through a lending
# engine call is handled, not equivalence. Usage: harness/regress/nested/run.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
BUILD="$ROOT/build/rust-nested"
OUT="$(mktemp -d)"
trap 'rm -rf "$OUT"' EXIT

cmake -S "$ROOT/angband" -B "$BUILD" -DCMAKE_BUILD_TYPE=Release \
    -DSUPPORT_BORG_HARNESS=ON -DBORG_IMPL=rust \
    -DBORG_RS_FEATURES=test-force-nested >"$BUILD.cmake.log" 2>&1 || {
    cat "$BUILD.cmake.log"; exit 1; }
make -C "$BUILD" -j"$(nproc)" >"$BUILD.make.log" 2>&1 || {
    tail -50 "$BUILD.make.log"; exit 1; }

fail=0
total=0
while read -r name seed race class turns _; do
    case "$name" in ''|'#'*) continue ;; esac
    if ! ANGBAND_BIN="$BUILD/game/angband" BORG_IMPL=rust \
        "$ROOT/harness/run.sh" "$OUT/$name" "$seed" "$race" "$class" "$turns" >/dev/null 2>&1; then
        echo "nested: $name FAILED (see run output)"; fail=1; continue
    fi
    if grep -q "panicked" "$OUT/$name/stderr.log"; then
        echo "nested: $name panicked"; fail=1; continue
    fi
    n=$(grep -c "nested Borg entry" "$OUT/$name/stderr.log" || true)
    total=$((total + n))
    echo "nested: $name completed, $n nested entries"
done < "$ROOT/harness/suites/smoke"

if [ "$fail" -ne 0 ]; then echo "nested: FAILED"; exit 1; fi
if [ "$total" -eq 0 ]; then echo "nested: FAILED (no nested entry happened)"; exit 1; fi
echo "nested: OK ($total nested entries, all games completed)"
