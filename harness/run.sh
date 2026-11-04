#!/usr/bin/env bash
# One deterministic Borg run.
# Usage: harness/run.sh OUTDIR SEED RACE CLASS TURNS [extra borgh options]
#   Env: BORG_IMPL=c|rust (selects build/<impl>), ANGBAND_BIN (override binary)
# Produces OUTDIR/keys.log (the oracle), OUTDIR/summary.json, OUTDIR/stderr.log
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMPL="${BORG_IMPL:-c}"
BIN="${ANGBAND_BIN:-$ROOT/build/$IMPL/game/angband}"
[ -x "$BIN" ] || { echo "missing $BIN (run harness/build.sh)" >&2; exit 2; }
OUT="$(realpath -m "$1")"; SEED="$2"; RACE="$3"; CLASS="$4"; TURNS="$5"; shift 5
rm -rf "$OUT"
mkdir -p "$OUT"/{user,save,archive,panic,scores}
# Pinned config; respawn race/class follow the run (cheat death stays off)
sed -e "s/^borg_respawn_race = .*/borg_respawn_race = $RACE/" \
    -e "s/^borg_respawn_class = .*/borg_respawn_class = $CLASS/" \
    "$ROOT/harness/borg.txt" > "$OUT/user/borg.txt"
cd "$(dirname "$BIN")"   # the game finds ./lib relative to the cwd
set +e
LANG=C.UTF-8 LC_ALL=C.UTF-8 "$BIN" -mborgh -n -uBorg \
    -duser="$OUT/user" -dsave="$OUT/save" -darchive="$OUT/archive" \
    -dpanic="$OUT/panic" -dscores="$OUT/scores" -- \
    -seed "$SEED" -race "$RACE" -class "$CLASS" -turns "$TURNS" \
    -keylog "$OUT/keys.log" -summary "$OUT/summary.json" -screen "$@" \
    2>"$OUT/stderr.log"
RC=$?
set -e
if [ $RC -ne 0 ] || [ ! -s "$OUT/summary.json" ]; then
    echo "run failed (rc=$RC): $OUT" >&2; exit 1
fi
