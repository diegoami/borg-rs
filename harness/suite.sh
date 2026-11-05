#!/usr/bin/env bash
# Run a seed suite in parallel.
# Usage: harness/suite.sh SUITE [OUTDIR]
#   SUITE = name in harness/suites/, or a path to a suite file
#   Env: BORG_IMPL=c|rust, JOBS (default nproc), ANGBAND_BIN (as run.sh)
# Suite file lines: NAME SEED RACE CLASS TURNS [FLAGS]
#   '#' starts a comment (whole line or end of line); FLAGS as in run.sh.
# The suite runs into a fresh OUTDIR.partial.<pid> and replaces OUTDIR when
# it ends, so OUTDIR never mixes results of two passes. It writes
#   OUTDIR/<NAME>/{keys.log,summary.json,stderr.log}
#   OUTDIR/manifest    git revision, binary sha256, BORG_IMPL, suite sha256
#   OUTDIR/timing.txt  one line per run (FAILED/MISSING marked)
#   OUTDIR/DONE        last, only if every run succeeded (harness/diff
#                      refuses an output dir without it)
#   OUTDIR/FAILED      instead of DONE if any run failed
# A failed run does not stop the others. Exit status: 0 if all runs
# succeeded, 1 otherwise, 2 on bad input.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUITE="$1"
SFILE="$ROOT/harness/suites/$SUITE"
case "$SUITE" in */*) SFILE="$(realpath -m "$SUITE")"; SUITE="$(basename "$SUITE")";; esac
[ -f "$SFILE" ] || { echo "no suite $SFILE" >&2; exit 2; }
IMPL="${BORG_IMPL:-c}"
BIN="${ANGBAND_BIN:-$ROOT/build/$IMPL/game/angband}"
[ -x "$BIN" ] || { echo "missing $BIN (run harness/build.sh)" >&2; exit 2; }
OUT="$(realpath -m "${2:-$ROOT/harness/out/$IMPL/$SUITE}")"
JOBS="${JOBS:-$(nproc)}"
WORK="$OUT.partial.$$"

# Parse the suite file: strip comments and whitespace, check field counts.
RUNS=()       # "NAME SEED RACE CLASS TURNS FLAGS", single-spaced
NAMES=()
lineno=0
while IFS= read -r line || [ -n "$line" ]; do
    lineno=$((lineno + 1))
    line="${line%%#*}"
    read -r -a f <<< "$line" || true
    [ ${#f[@]} -eq 0 ] && continue
    if [ ${#f[@]} -lt 5 ] || [ ${#f[@]} -gt 6 ]; then
        echo "$SFILE:$lineno: expected NAME SEED RACE CLASS TURNS [FLAGS]" >&2
        exit 2
    fi
    for n in "${NAMES[@]}"; do
        [ "$n" = "${f[0]}" ] && { echo "$SFILE:$lineno: duplicate run ${f[0]}" >&2; exit 2; }
    done
    NAMES+=("${f[0]}")
    RUNS+=("${f[0]} ${f[1]} ${f[2]} ${f[3]} ${f[4]} ${f[5]:--}")
done < "$SFILE"
[ ${#RUNS[@]} -gt 0 ] || { echo "suite $SUITE has no runs" >&2; exit 2; }

rm -rf "$WORK"
mkdir -p "$WORK"

# Provenance
BUILD_DIR="$(dirname "$(dirname "$BIN")")"
CACHE_IMPL=$(sed -n 's/^BORG_IMPL:[A-Z]*=//p' "$BUILD_DIR/CMakeCache.txt" 2>/dev/null || true)
GIT_REV=$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo unknown)
if [ -n "$(git -C "$ROOT" status --porcelain 2>/dev/null)" ]; then DIRTY=yes; else DIRTY=no; fi
{
    echo "suite=$SUITE"
    echo "suite_sha256=$(sha256sum < "$SFILE" | cut -d' ' -f1)"
    echo "borg_impl_env=$IMPL"
    echo "binary=$BIN"
    echo "binary_sha256=$(sha256sum < "$BIN" | cut -d' ' -f1)"
    echo "binary_cmake_borg_impl=${CACHE_IMPL:-unknown}"
    echo "git_rev=$GIT_REV"
    echo "git_dirty=$DIRTY"
    echo "jobs=$JOBS"
    echo "started_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "$WORK/manifest"

export BIN
START=$(date +%s.%N)
set +e
printf '%s\n' "${RUNS[@]}" | \
  xargs -P "$JOBS" -n 6 bash -c \
    'if ANGBAND_BIN="$BIN" "$0" "$1/$2" "$3" "$4" "$5" "$6" "$7"; then echo "done $2"; else echo "FAILED $2" >&2; exit 1; fi' \
    "$ROOT/harness/run.sh" "$WORK"
XRC=$?
set -e
END=$(date +%s.%N)

FAIL=0
[ $XRC -eq 0 ] || FAIL=1
for d in "${NAMES[@]}"; do
    [ -s "$WORK/$d/summary.json" ] || { echo "MISSING $d" >&2; FAIL=1; }
done
WALL=$(echo "$END - $START" | bc)
if [ $FAIL = 0 ]; then RESULT=OK; else RESULT=FAILED; fi
{
  echo "suite=$SUITE impl=$IMPL jobs=$JOBS wall_seconds=$WALL result=$RESULT"
  for d in "${NAMES[@]}"; do
    python3 -c 'import json,sys; s=json.load(open(sys.argv[1])); print(sys.argv[2], s["stop_reason"], "turns=%d"%s["game_turns"], "pturns=%d"%s["player_turns"], "clev=%d"%s["clevel"], "maxdepth=%d"%s["max_depth"], "keys=%d"%s["keys"], "wall=%.1f"%s["wall_seconds"], s["race"], s["class"])' "$WORK/$d/summary.json" "$d" 2>/dev/null || echo "$d MISSING/FAILED"
  done
} > "$WORK/timing.txt"
echo "finished_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ) result=$RESULT" >> "$WORK/manifest"

# Replace OUTDIR; the marker is written last.
rm -rf "$OUT"
mv "$WORK" "$OUT"
if [ $FAIL = 0 ]; then
    date -u +%Y-%m-%dT%H:%M:%SZ > "$OUT/DONE"
else
    echo "suite $SUITE failed (exit $XRC)" > "$OUT/FAILED"
fi
cat "$OUT/timing.txt"
exit $FAIL
