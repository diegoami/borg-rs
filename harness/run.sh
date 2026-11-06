#!/usr/bin/env bash
# One deterministic Borg run.
# Usage: harness/run.sh OUTDIR SEED RACE CLASS TURNS [FLAGS] [-- extra borgh options]
#   FLAGS: comma-separated, "-" for none.
#     cheat  sets borg_cheat_death = TRUE
#     menu   passes -menu-letters: race/class are picked with the birth
#            menu's own letters, so the indices mean p_race.txt/class.txt
#            order (without it, indices >= 7 pick other entries; see
#            docs/harness.md)
#     cheatlive  passes -cheat-live: the game option cheat_live is on, so with
#            borg_cheat_death off (no cheat flag) the Borg reincarnates
#            at death (borg_respawn_race/class) and the run goes on
#   Config flags (each rewrites one line of the run's user/borg.txt; with
#   none of them the file is the same as before, byte for byte):
#     munchkin   sets borg_munchkin_start = TRUE
#     lunal      sets borg_lunal_mode = TRUE
#     selflunal  sets borg_self_lunal = TRUE
#     NAME=VALUE sets any option of borg.txt: the line "NAME = ..." becomes
#                "NAME = VALUE", e.g. borg_money_scum_amount=5000 or
#                borg_uses_swaps=FALSE. NAME must be the option's full
#                name and have exactly one "NAME = " line in
#                harness/borg.txt (else the run fails); VALUE is TRUE,
#                FALSE or an integer. Applied after the lines run.sh sets
#                itself (respawn race/class); borg_cheat_death is set
#                with the cheat flag only.
#   Env: BORG_IMPL=c|rust (selects build/<impl>), ANGBAND_BIN (override binary)
# Produces OUTDIR/keys.log (the oracle), OUTDIR/messages.log (every message
# added to the message log, Borg notes included), OUTDIR/state.log (one
# engine-state line per key), OUTDIR/summary.json, OUTDIR/stderr.log,
# OUTDIR/config.txt (flags and sha256 of the run's user/borg.txt; not part of
# the oracle, harness/diff does not read it)
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
SAVE_NAME=""
BIRTH_ARGS=(-n)
BORGH_FLAGS=()
CONFIG_SED=()   # extra borg.txt rewrites from config flags
SET_SED=()      # NAME=VALUE flags, applied last
SET_CHECK=()    # the lines they must produce
if [ "$FLAGS" != "-" ]; then
    IFS=, read -r -a FL <<< "$FLAGS"
    for fl in "${FL[@]}"; do
        case "$fl" in
            cheat) CHEAT=TRUE;;
            menu) BORGH_FLAGS+=(-menu-letters);;
            cheatlive) BORGH_FLAGS+=(-cheat-live);;
            munchkin) CONFIG_SED+=(-e "s/^borg_munchkin_start = .*/borg_munchkin_start = TRUE/");;
            lunal) CONFIG_SED+=(-e "s/^borg_lunal_mode = .*/borg_lunal_mode = TRUE/");;
            selflunal) CONFIG_SED+=(-e "s/^borg_self_lunal = .*/borg_self_lunal = TRUE/");;
            save=*)
                [ -z "$SAVE_NAME" ] || { echo "run.sh: duplicate save flag" >&2; exit 2; }
                SAVE_NAME="${fl#save=}"
                [[ "$SAVE_NAME" =~ ^[a-zA-Z0-9_-]+\.save$ ]] || {
                    echo "run.sh: save must be a filename under harness/saves" >&2; exit 2; }
                [ -s "$ROOT/harness/saves/$SAVE_NAME" ] || {
                    echo "run.sh: missing fixture $SAVE_NAME" >&2; exit 2; }
                ;;
            *=*)
                name="${fl%%=*}"; val="${fl#*=}"
                [[ "$name" =~ ^borg_[a-z_]+$ ]] || {
                    echo "run.sh: flag '$fl': bad option name '$name'" >&2; exit 2; }
                [[ "$val" =~ ^(TRUE|FALSE|-?[0-9]+)$ ]] || {
                    echo "run.sh: flag '$fl': value must be TRUE, FALSE or an integer" >&2; exit 2; }
                [ "$name" != borg_cheat_death ] || {
                    echo "run.sh: flag '$fl': use the cheat flag for borg_cheat_death" >&2; exit 2; }
                n=$(grep -c "^$name = " "$ROOT/harness/borg.txt" || true)
                [ "$n" = 1 ] || {
                    echo "run.sh: flag '$fl': $n lines '$name = ' in harness/borg.txt (unknown option?)" >&2; exit 2; }
                SET_SED+=(-e "s/^$name = .*/$name = $val/")
                SET_CHECK+=("$name = $val");;
            *) echo "run.sh: unknown flag '$fl'" >&2; exit 2;;
        esac
    done
fi
rm -rf "$OUT"
mkdir -p "$OUT"/{user,save,archive,panic,scores}
if [ -n "$SAVE_NAME" ]; then
    # The engine may autosave: never give it the committed fixture's path.
    cp "$ROOT/harness/saves/$SAVE_NAME" "$OUT/save/input.save"
    BIRTH_ARGS=()
    BORGH_FLAGS+=(-load "$OUT/save/input.save")
fi
# Pinned config; respawn race/class follow the run (cheat death stays off)
sed -e "s/^borg_respawn_race = .*/borg_respawn_race = $RACE/" \
    -e "s/^borg_respawn_class = .*/borg_respawn_class = $CLASS/" \
    -e "s/^borg_cheat_death = .*/borg_cheat_death = $CHEAT/" \
    "${CONFIG_SED[@]}" \
    "$ROOT/harness/borg.txt" > "$OUT/user/borg.txt"
if [ ${#SET_SED[@]} -gt 0 ]; then
    sed -i "${SET_SED[@]}" "$OUT/user/borg.txt"
    for line in "${SET_CHECK[@]}"; do
        grep -qx "$line" "$OUT/user/borg.txt" || {
            echo "run.sh: no '$line' line in the run's borg.txt" >&2; exit 2; }
    done
fi
# Each config flag must have changed its line (a renamed option would
# otherwise be silently ignored).
for fl in munchkin:borg_munchkin_start lunal:borg_lunal_mode selflunal:borg_self_lunal; do
    case ",$FLAGS," in
        *",${fl%%:*},"*)
            grep -qx "${fl#*:} = TRUE" "$OUT/user/borg.txt" || {
                echo "run.sh: flag ${fl%%:*}: no '${fl#*:} = TRUE' line in borg.txt" >&2; exit 2; };;
    esac
done
{
    echo "flags=$FLAGS"
    echo "borg_txt_sha256=$(sha256sum < "$OUT/user/borg.txt" | cut -d' ' -f1)"
    if [ -n "$SAVE_NAME" ]; then
        echo "save_sha256=$(sha256sum < "$OUT/save/input.save" | cut -d' ' -f1)"
    fi
} > "$OUT/config.txt"
cd "$(dirname "$BIN")"   # the game finds ./lib relative to the cwd
set +e
TZ=UTC LANG=C.UTF-8 LC_ALL=C.UTF-8 "$BIN" -mborgh "${BIRTH_ARGS[@]}" -uBorg \
    -duser="$OUT/user" -dsave="$OUT/save" -darchive="$OUT/archive" \
    -dpanic="$OUT/panic" -dscores="$OUT/scores" -- \
    -seed "$SEED" -race "$RACE" -class "$CLASS" -turns "$TURNS" \
    -keylog "$OUT/keys.log" -msglog "$OUT/messages.log" -statelog "$OUT/state.log" \
    -summary "$OUT/summary.json" -screen "${BORGH_FLAGS[@]}" "$@" \
    2>"$OUT/stderr.log"
RC=$?
set -e
if [ $RC -ne 0 ] || [ ! -s "$OUT/summary.json" ]; then
    echo "run failed (rc=$RC): $OUT" >&2; exit 1
fi
