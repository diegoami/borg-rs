#!/usr/bin/env bash
# One deterministic Borg run.
# Usage: harness/run.sh OUTDIR SEED RACE CLASS TURNS [FLAGS] [-- extra borgh options]
#   FLAGS: comma-separated, "-" for none.
#     cheat  sets borg_cheat_death = TRUE
#     menu   passes -menu-letters: race/class are picked with the birth
#            menu's own letters, so the indices mean p_race.txt/class.txt
#            order (without it, indices >= 7 pick other entries; see
#            docs/harness.md)
#   Env: BORG_IMPL=c|rust (selects build/<impl>), ANGBAND_BIN (override binary)
# Produces OUTDIR/keys.log (the oracle), OUTDIR/summary.json, OUTDIR/stderr.log
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMPL="${BORG_IMPL:-c}"
BIN="${ANGBAND_BIN:-$ROOT/build/$IMPL/game/angband}"
[ -x "$BIN" ] || { echo "missing $BIN (run harness/build.sh)" >&2; exit 2; }
OUT="$(realpath -m "$1")"; SEED="$2"; RACE="$3"; CLASS="$4"; TURNS="$5"; shift 5
FLAGS="-"
if [ $# -gt 0 ] && [ "$1" != "--" ]; then FLAGS="$1"; shift; fi
[ "${1:-}" = "--" ] && shift
CHEAT=FALSE
BORGH_FLAGS=()
if [ "$FLAGS" != "-" ]; then
    IFS=, read -r -a FL <<< "$FLAGS"
    for fl in "${FL[@]}"; do
        case "$fl" in
            cheat) CHEAT=TRUE;;
            menu) BORGH_FLAGS+=(-menu-letters);;
            *) echo "run.sh: unknown flag '$fl'" >&2; exit 2;;
        esac
    done
fi
rm -rf "$OUT"
mkdir -p "$OUT"/{user,save,archive,panic,scores}
# Pinned config; respawn race/class follow the run (cheat death stays off)
sed -e "s/^borg_respawn_race = .*/borg_respawn_race = $RACE/" \
    -e "s/^borg_respawn_class = .*/borg_respawn_class = $CLASS/" \
    -e "s/^borg_cheat_death = .*/borg_cheat_death = $CHEAT/" \
    "$ROOT/harness/borg.txt" > "$OUT/user/borg.txt"
cd "$(dirname "$BIN")"   # the game finds ./lib relative to the cwd
set +e
TZ=UTC LANG=C.UTF-8 LC_ALL=C.UTF-8 "$BIN" -mborgh -n -uBorg \
    -duser="$OUT/user" -dsave="$OUT/save" -darchive="$OUT/archive" \
    -dpanic="$OUT/panic" -dscores="$OUT/scores" -- \
    -seed "$SEED" -race "$RACE" -class "$CLASS" -turns "$TURNS" \
    -keylog "$OUT/keys.log" -summary "$OUT/summary.json" -screen "${BORGH_FLAGS[@]}" "$@" \
    2>"$OUT/stderr.log"
RC=$?
set -e
if [ $RC -ne 0 ] || [ ! -s "$OUT/summary.json" ]; then
    echo "run failed (rc=$RC): $OUT" >&2; exit 1
fi
