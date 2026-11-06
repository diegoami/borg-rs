#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
OUT=$(mktemp -d "$ROOT/harness/out/screen-text.XXXXXX")
cc -DALLOW_BORG -ffunction-sections -fdata-sections -I"$ROOT/angband/src" \
    -c "$ROOT/angband/src/borg/borg-io.c" -o "$OUT/borg-io.o"
cc -DALLOW_BORG -I"$ROOT/angband/src" -Wl,--gc-sections \
    "$ROOT/harness/regress/screen-text/probe.c" "$OUT/borg-io.o" -o "$OUT/probe"
"$OUT/probe"
