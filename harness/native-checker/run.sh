#!/usr/bin/env bash
# Positive parity, actual-source mutation rejection, and optional C UBSan control.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
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
if [ "${CHECK_UBSAN:-0}" = 1 ]; then
    BORG_XCHECK_UBSAN=1 CARGO_TARGET_DIR="$OUT/ubsan-target" \
        cargo run --release --manifest-path "$MANIFEST" -- 30000 >"$OUT/ubsan.log" 2>&1
fi
CARGO_TARGET_DIR="$OUT/target" cargo clippy --release --manifest-path "$MANIFEST" -- -D warnings \
    >"$OUT/clippy.log" 2>&1
sha256sum "$ROOT/borg-rs/src/borg_formulas_calc.rs" "$ROOT/borg-rs/src/cctype.rs" \
    "$ROOT/angband/src/borg/borg-formulas-calc.c" "$ROOT/angband/src/borg/borg-formulas.c" \
    "$ROOT/harness/native-checker/native.c" "$ROOT/harness/native-checker/src/main.rs" \
    "$ROOT/harness/native-checker/build.rs" >"$OUT/SHA256SUMS"
touch "$OUT/DONE"
cat "$OUT/parity.log"
echo "native-checker: positive parity and real-source fault control passed; receipts $OUT"
