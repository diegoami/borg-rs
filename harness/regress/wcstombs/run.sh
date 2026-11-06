#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
mkdir -p "$ROOT/harness/out"
OUT=$(mktemp -d "$ROOT/harness/out/wcstombs.XXXXXX")
cc -std=c11 -O2 -Wall -Wextra -Werror "$ROOT/harness/regress/wcstombs/probe.c" -o "$OUT/probe"
"$OUT/probe"
