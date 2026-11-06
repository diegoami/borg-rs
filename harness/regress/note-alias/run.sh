#!/usr/bin/env bash
# Differential check of borg_item.note aliasing, C vs Rust (review of
# 4c06cec08, finding 1). Builds note_alias.c against the original C
# borg-item.c / borg-item-id.c and the Rust example borg-rs/examples/
# note_alias.rs, runs both and compares their output byte for byte.
# Exit 0 = identical. Usage: harness/regress/note-alias/run.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DIR="$ROOT/harness/regress/note-alias"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cc -DALLOW_BORG -DNDEBUG -ffunction-sections -fdata-sections \
    -I"$ROOT/angband/src" "$DIR/note_alias.c" \
    "$ROOT/angband/src/borg/borg-item.c" "$ROOT/angband/src/borg/borg-item-id.c" \
    -Wl,--gc-sections -o "$TMP/c"
(cd "$ROOT/borg-rs" && cargo run --quiet --release --example note_alias) > "$TMP/rust.out"
"$TMP/c" > "$TMP/c.out"

if cmp -s "$TMP/c.out" "$TMP/rust.out"; then
    echo "note-alias: C and Rust identical ($(wc -l < "$TMP/c.out") lines)"
    cat "$TMP/c.out"
else
    echo "note-alias: C and Rust DIFFER" >&2
    diff "$TMP/c.out" "$TMP/rust.out" >&2 || true
    exit 1
fi
