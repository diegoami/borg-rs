#!/usr/bin/env bash
# Positive parity, actual-source mutation rejection, and optional C UBSan control.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
mkdir -p "$ROOT/harness/out"
OUT=$(mktemp -d "$ROOT/harness/out/native-checker.XXXXXX")
MANIFEST="$ROOT/harness/native-checker/Cargo.toml"
CARGO_TARGET_DIR="$OUT/target" cargo run --release --manifest-path "$MANIFEST" -- 30000 \
    >"$OUT/parity.log" 2>&1
python3 - "$ROOT/borg-rs/src/borg_formulas_calc.rs" "$OUT/fault.rs" <<'PY'
from pathlib import Path
import sys
s=Path(sys.argv[1]).read_text()
a='total = left_value.wrapping_add(right_value)'
b='total = left_value.wrapping_sub(right_value)'
assert s.count(a)==1
Path(sys.argv[2]).write_text(s.replace(a,b))
PY
set +e
BORG_XCHECK_RUST_SOURCE="$OUT/fault.rs" CARGO_TARGET_DIR="$OUT/fault-target" \
    cargo run --release --manifest-path "$MANIFEST" -- 1000 1511464998 --fault-control \
    >"$OUT/fault.log" 2>&1
rc=$?
set -e
[ "$rc" = 1 ] && rg -q '^MISMATCH case=' "$OUT/fault.log" || {
    cat "$OUT/fault.log" >&2; echo 'fault control did not detect the altered Rust addition' >&2; exit 1;
}
CARGO_TARGET_DIR="$OUT/target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 2000 \
    >"$OUT/spatial.log" 2>&1
CARGO_TARGET_DIR="$OUT/target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 2000 305441741 \
    >"$OUT/spatial-second.log" 2>&1
python3 "$ROOT/harness/native-checker/spatial_controls.py" "$ROOT" "$OUT"
CARGO_TARGET_DIR="$OUT/target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 2600 --glyph >"$OUT/glyph.log" 2>&1
CARGO_TARGET_DIR="$OUT/target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 1280 --lunal >"$OUT/lunal.log" 2>&1
for control in flow projection combat damage glyph glyph-distance glyph-tie glyph-old-tie glyph-cost lunal; do
    case "$control" in
        flow) override_name=BORG_XCHECK_FLOW_SOURCE;;
        projection) override_name=BORG_XCHECK_PROJECTION_SOURCE;;
        glyph*) override_name=BORG_XCHECK_GLYPH_SOURCE;;
        lunal) override_name=BORG_XCHECK_LUNAL_SOURCE;;
        *) override_name=BORG_XCHECK_COMBAT_SOURCE;;
    esac
    extra_args=()
    control_count=100
    if [[ "$control" = glyph* ]]; then extra_args=(--glyph); control_count=2600; fi
    if [ "$control" = lunal ]; then extra_args=(--lunal); fi
    set +e
    env "$override_name=$OUT/$control-fault.rs" CARGO_TARGET_DIR="$OUT/spatial-fault-target" \
        cargo run --release --manifest-path "$MANIFEST" --bin spatial -- "$control_count" "${extra_args[@]}" --fault-control \
        >"$OUT/$control-fault.log" 2>&1
    rc=$?
    set -e
    [ "$rc" = 1 ] && rg -q '^MISMATCH spatial case=' "$OUT/$control-fault.log" || {
        cat "$OUT/$control-fault.log" >&2; echo "spatial $control fault was not detected" >&2; exit 1;
    }
done
if [ "${CHECK_UBSAN:-0}" = 1 ]; then
    BORG_XCHECK_UBSAN=1 CARGO_TARGET_DIR="$OUT/ubsan-target" \
        cargo run --release --manifest-path "$MANIFEST" -- 30000 >"$OUT/ubsan.log" 2>&1
    BORG_XCHECK_UBSAN=1 CARGO_TARGET_DIR="$OUT/ubsan-target" \
        cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 2000 >"$OUT/spatial-ubsan.log" 2>&1
    BORG_XCHECK_UBSAN=1 CARGO_TARGET_DIR="$OUT/ubsan-target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 2600 --glyph >"$OUT/glyph-ubsan.log" 2>&1
    BORG_XCHECK_UBSAN=1 CARGO_TARGET_DIR="$OUT/ubsan-target" cargo run --release --manifest-path "$MANIFEST" --bin spatial -- 1280 --lunal >"$OUT/lunal-ubsan.log" 2>&1
fi
CARGO_TARGET_DIR="$OUT/target" cargo clippy --release --manifest-path "$MANIFEST" --all-targets -- -D warnings \
    >"$OUT/clippy.log" 2>&1
sha256sum "$ROOT/borg-rs/src/borg_formulas_calc.rs" "$ROOT/borg-rs/src/cctype.rs" \
    "$ROOT/angband/src/borg/borg-formulas-calc.c" "$ROOT/angband/src/borg/borg-formulas.c" \
    "$ROOT/harness/native-checker/native.c" "$ROOT/harness/native-checker/src/main.rs" \
    "$ROOT/harness/native-checker/build.rs" "$ROOT/harness/native-checker/native_spatial.c" \
    "$ROOT/harness/native-checker/src/bin/spatial.rs" "$ROOT/harness/native-checker/extract_spatial.py" \
    "$ROOT/borg-rs/src/borg_flow.rs" "$ROOT/borg-rs/src/borg_projection.rs" \
    "$ROOT/borg-rs/src/borg_fight_attack.rs" "$ROOT/borg-rs/src/borg_flow_glyph.rs" \
    "$ROOT/borg-rs/src/borg_think_dungeon.rs" "$ROOT/harness/native-checker/run.sh" \
    "$ROOT/harness/native-checker/spatial_controls.py" \
    "$ROOT/harness/native-checker/Cargo.toml" "$ROOT/harness/native-checker/Cargo.lock" >"$OUT/SHA256SUMS"
"$ROOT/harness/identity.sh" source-hash rust >"$OUT/RUST_INPUT_SHA256"
"$ROOT/harness/identity.sh" source-hash c >"$OUT/C_INPUT_SHA256"
find "$OUT/target/release/build" -name spatial-source-ledger.json -exec cp '{}' "$OUT/source-ledger.json" \;
touch "$OUT/DONE"
cat "$OUT/parity.log" "$OUT/spatial.log" "$OUT/spatial-second.log" "$OUT/glyph.log" "$OUT/lunal.log"
echo "native-checker: calculator/spatial/combat parity and eleven real-source fault controls passed; glyph26 layouts x100 variants; receipts $OUT"
