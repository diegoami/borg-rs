#!/usr/bin/env bash
# Run a seed suite in parallel.
# Usage: harness/suite.sh SUITE [OUTDIR]     (SUITE = name in harness/suites/)
#   Env: BORG_IMPL=c|rust, JOBS (default nproc)
# Suite file lines: NAME SEED RACE CLASS TURNS [FLAGS]   ('#' comments allowed)
# Writes OUTDIR/<NAME>/{keys.log,summary.json} and OUTDIR/timing.txt
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUITE="$1"
SFILE="$ROOT/harness/suites/$SUITE"
[ -f "$SFILE" ] || { echo "no suite $SFILE" >&2; exit 2; }
OUT="$(realpath -m "${2:-$ROOT/harness/out/${BORG_IMPL:-c}/$SUITE}")"
JOBS="${JOBS:-$(nproc)}"
mkdir -p "$OUT"
START=$(date +%s.%N)
grep -v '^\s*#' "$SFILE" | grep -v '^\s*$' | \
  xargs -P "$JOBS" -L 1 bash -c \
    '"$0" "$1/$2" "$3" "$4" "$5" "$6" "${7:--}" && echo "done $2"' \
    "$ROOT/harness/run.sh" "$OUT"
END=$(date +%s.%N)
FAIL=0
for d in $(grep -v '^\s*#' "$SFILE" | awk 'NF{print $1}'); do
    [ -s "$OUT/$d/summary.json" ] || { echo "MISSING $d" >&2; FAIL=1; }
done
WALL=$(echo "$END - $START" | bc)
{
  echo "suite=$SUITE impl=${BORG_IMPL:-c} jobs=$JOBS wall_seconds=$WALL"
  for d in $(grep -v '^\s*#' "$SFILE" | awk 'NF{print $1}'); do
    python3 -c 'import json,sys; s=json.load(open(sys.argv[1])); print(sys.argv[2], s["stop_reason"], "turns=%d"%s["game_turns"], "pturns=%d"%s["player_turns"], "clev=%d"%s["clevel"], "maxdepth=%d"%s["max_depth"], "keys=%d"%s["keys"], "wall=%.1f"%s["wall_seconds"])' "$OUT/$d/summary.json" "$d" 2>/dev/null || echo "$d MISSING"
  done
} > "$OUT/timing.txt"
cat "$OUT/timing.txt"
exit $FAIL
