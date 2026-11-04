#!/usr/bin/env bash
# M2 check: the BORG_IMPL=rust binary contains no C Borg code.
#  1. the rust build compiles no src/borg/*.c (compile_commands.json and
#     object files);
#  2. every symbol that a C Borg object file defines (global or local) and
#     that appears in the rust binary is a symbol exported by the Rust
#     staticlib (#[no_mangle] items), never a C-compiled one.
# Needs both builds: harness/build.sh and BORG_IMPL=rust harness/build.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
C="$ROOT/build/c"; R="$ROOT/build/rust"
BIN="$R/game/angband"
LIB="$R/cargo/release/libborg_rs.a"
fail=0
for f in "$BIN" "$LIB" "$R/compile_commands.json" "$C/compile_commands.json"; do
    [ -e "$f" ] || { echo "missing $f"; exit 2; }
done

n=$(grep -c '"file": ".*/src/borg/.*\.c"' "$R/compile_commands.json" || true)
echo "rust build: src/borg/*.c entries in compile_commands.json: $n"
[ "$n" = 0 ] || fail=1
n=$(find "$R/CMakeFiles" -path '*src/borg/*' -name '*.o' | wc -l)
echo "rust build: src/borg object files: $n"
[ "$n" = 0 ] || fail=1

TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT
# all symbols defined by the C Borg objects (any binding)
nm --defined-only "$C"/CMakeFiles/OurCoreLib.dir/src/borg/*.o 2>/dev/null \
    | awk 'NF==3{print $3}' | sed 's/\..*$//' | sort -u > "$TMP/cborg"
# all symbols in the rust binary
nm "$BIN" | awk 'NF>=2{print $NF}' | sort -u > "$TMP/bin"
# global symbols exported by the Rust staticlib's borg_rs object(s)
nm --defined-only -A "$LIB" 2>/dev/null | grep 'borg_rs' \
    | awk '$2 ~ /^[TDBRV]$/ {print $3}' | sort -u > "$TMP/rust"
comm -12 "$TMP/cborg" "$TMP/bin" > "$TMP/both"
# names that the engine's own (non-Borg) C objects also define, e.g. a
# file-local "effects" table: allowed only as many times as the engine
# defines them
nm --defined-only "$R"/CMakeFiles/OurCoreLib.dir/src/*.o \
    "$R"/CMakeFiles/OurExecutable.dir/src/*.o 2>/dev/null \
    | awk 'NF==3{print $3}' | sort | uniq -c | awk '{print $2, $1}' > "$TMP/engcount"
nm "$BIN" | awk 'NF>=2{print $NF}' | sort | uniq -c | awk '{print $2, $1}' > "$TMP/bincount"
: > "$TMP/bad"
while read -r sym; do
    grep -qxF "$sym" "$TMP/rust" && continue
    e=$(awk -v s="$sym" '$1==s{print $2}' "$TMP/engcount"); e=${e:-0}
    b=$(awk -v s="$sym" '$1==s{print $2}' "$TMP/bincount"); b=${b:-0}
    if [ "$b" -gt "$e" ]; then echo "$sym (binary $b, engine $e)" >> "$TMP/bad"; fi
done < "$TMP/both"
echo "C Borg symbols: $(wc -l < "$TMP/cborg"); present in rust binary: $(wc -l < "$TMP/both"); of those exported by the Rust crate: $(comm -12 "$TMP/both" "$TMP/rust" | wc -l)"
if [ -s "$TMP/bad" ]; then
    echo "C Borg symbols in the rust binary NOT provided by the Rust crate:"
    cat "$TMP/bad"; fail=1
fi
# no C debug info / file names from src/borg
if strings "$BIN" | grep -qE 'src/borg/borg[-a-z]*\.c'; then
    echo "rust binary mentions src/borg/*.c paths:"
    strings "$BIN" | grep -E 'src/borg/borg[-a-z]*\.c' | sort -u | head; fail=1
fi
if [ $fail = 0 ]; then echo "OK: no C Borg code in $BIN"; else echo "FAIL"; fi
exit $fail
