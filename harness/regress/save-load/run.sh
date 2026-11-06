#!/usr/bin/env bash
# Replay the same fixtures twice with C and once with Rust; reject bad inputs.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
mkdir -p "$ROOT/harness/out"
OUT=$(mktemp -d "$ROOT/harness/out/save-load.XXXXXX")
cat > "$OUT/replay" <<'EOF'
saved-depth50 7100 0 0 1000 menu,cheat,save=depth50.save
saved-morgoth 7101 0 0 1000 menu,cheat,save=morgoth.save
EOF
for pass in c1 c2 rust; do
    impl=c; [ "$pass" != rust ] || impl=rust
    BORG_IMPL="$impl" JOBS=1 "$ROOT/harness/suite.sh" "$OUT/replay" "$OUT/$pass"
done
"$ROOT/harness/diff" --suite "$OUT/replay" --expect-impl c "$OUT/c1" "$OUT/c2"
"$ROOT/harness/diff" --suite "$OUT/replay" --expect-impl rust "$OUT/rust" "$OUT/c1"
if BORG_IMPL=c "$ROOT/harness/run.sh" "$OUT/missing" 7100 0 0 1000 \
    menu,cheat,save=does-not-exist.save >"$OUT/missing.log" 2>&1; then
    echo "FAIL: missing save accepted" >&2; exit 1
fi
if BORG_IMPL=c "$ROOT/harness/run.sh" "$OUT/wrong-race" 7100 1 0 1000 \
    menu,cheat,save=depth50.save >"$OUT/wrong-race.log" 2>&1; then
    echo "FAIL: wrong race accepted" >&2; exit 1
fi
cfg="$OUT/c1/saved-depth50/config.txt"
cp "$cfg" "$OUT/original-config.txt"
sed -i 's/^save_sha256=.*/save_sha256=incorrect/' "$cfg"
if "$ROOT/harness/diff" --suite "$OUT/replay" "$OUT/c1" "$OUT/c2" >"$OUT/bad-hash.log" 2>&1; then
    echo "FAIL: incorrect save hash accepted" >&2; exit 1
fi
grep -q 'save_sha256 does not match' "$OUT/bad-hash.log"
cp "$OUT/original-config.txt" "$cfg"
echo "OK: save replay, C determinism, Rust parity, missing file, race and hash checks ($OUT)"
