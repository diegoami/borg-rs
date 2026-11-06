#!/usr/bin/env bash
# Build synthetic endgame starting points with the C engine, twice.
# Usage: harness/make-fixtures.sh OUTDIR (must not already exist)
# Does not install fixtures or references. See docs/harness.md.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$(realpath -m "${1:?supply a fresh output directory}")"
BIN="$ROOT/build/c/game/angband"
ID="$ROOT/build/c/BUILD_ID"
[ ! -e "$OUT" ] || { echo "output already exists: $OUT" >&2; exit 2; }
[ -f "$ID" ] && [ -x "$BIN" ] || { echo "build C first" >&2; exit 2; }
get_id() { sed -n "s/^$1=//p" "$ID"; }
[ "$(get_id impl)" = c ] && [ "$("$ROOT/harness/identity.sh" binary-impl "$BIN")" = c ] || {
    echo "fixtures require the C build" >&2; exit 2; }
[ "$(get_id binary_sha256)" = "$(sha256sum < "$BIN" | cut -d' ' -f1)" ] &&
[ "$(get_id source_sha256)" = "$("$ROOT/harness/identity.sh" source-hash c)" ] || {
    echo "stale C build: rebuild first" >&2; exit 2; }
mkdir -p "$OUT"
cp "$ID" "$OUT/BUILD_ID"
cat > "$OUT/recipe.tsv" <<'EOF'
depth50 5000 50 -
depth98 5001 98 -
depth99 5002 99 -
depth100 5003 100 -
morgoth 5004 100 -near-morgoth
EOF
while read -r name seed depth extra; do
    opts=(-make-fixture "$depth")
    [ "$extra" = - ] || opts+=("$extra")
    for pass in 1 2; do
        BORG_IMPL=c ANGBAND_BIN="$BIN" "$ROOT/harness/run.sh" \
            "$OUT/pass$pass/$name" "$seed" 0 0 1 menu -- \
            "${opts[@]}" -save-on-stop "$OUT/pass$pass/$name.save"
        python3 - "$OUT/pass$pass/$name/summary.json" "$depth" <<'PY'
import json, sys
s = json.load(open(sys.argv[1]))
assert s['stop_reason'] == 'fixture' and s['depth'] == int(sys.argv[2])
assert s['clevel'] == 50 and not s['dead'] and s['keys'] == 0
PY
    done
    cmp "$OUT/pass1/$name.save" "$OUT/pass2/$name.save"
    for log in keys messages state; do
        cmp "$OUT/pass1/$name/$log.log" "$OUT/pass2/$name/$log.log"
    done
    python3 - "$OUT/pass1/$name/summary.json" "$OUT/pass2/$name/summary.json" <<'PY'
import json, sys
a, b = (json.load(open(p)) for p in sys.argv[1:])
a.pop('wall_seconds'); b.pop('wall_seconds')
assert a == b
PY
    cp "$OUT/pass1/$name.save" "$OUT/$name.save"
    echo "identical: $name"
done < "$OUT/recipe.tsv"
[ "$(get_id binary_sha256)" = "$(sha256sum < "$BIN" | cut -d' ' -f1)" ] || {
    echo "C binary changed during generation" >&2; exit 1; }
(cd "$OUT" && sha256sum ./*.save > SHA256SUMS)
date -u +%Y-%m-%dT%H:%M:%SZ > "$OUT/DONE"
