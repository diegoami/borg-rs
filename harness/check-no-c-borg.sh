#!/usr/bin/env bash
# M2 check: the BORG_IMPL=rust binary contains no C Borg code, and its
# engine is the same as the C build's.
#  1. The rust build compiles no src/borg/*.c (compile_commands.json and
#     object files).
#  2. Every member of the Rust staticlib is a rustc object (`*.rcgu.o`
#     whose .comment says "rustc") or one of a fixed list of
#     compiler-builtins C objects (clang-compiled integer/float helpers).
#     Any other member (e.g. a C object added by a build.rs) fails.
#  3. Every symbol that a C Borg object defines (global or local) and that
#     appears in the rust binary is exported by the crate's own borg_rs
#     rcgu objects, or is also defined by the engine's own C objects.
#     Inputs that would make this vacuous fail: too few C Borg symbols
#     (build/c not a C Borg build), or a rust binary without a symbol table.
#  4. Every engine object file (everything outside src/borg) is
#     byte-identical between build/c and build/rust, except buildid.c.o
#     (it embeds the build id).
# Needs both builds: harness/build.sh and BORG_IMPL=rust harness/build.sh
# Env: C_BUILD, RUST_BUILD (default build/c, build/rust), RUST_LIB.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
C="${C_BUILD:-$ROOT/build/c}"; R="${RUST_BUILD:-$ROOT/build/rust}"
BIN="$R/game/angband"
LIB="${RUST_LIB:-$R/cargo/release/libborg_rs.a}"
fail=0
for f in "$BIN" "$LIB" "$R/compile_commands.json" "$C/compile_commands.json" \
         "$C/CMakeCache.txt"; do
    [ -e "$f" ] || { echo "missing $f"; exit 2; }
done
grep -qx 'BORG_IMPL:STRING=c' "$C/CMakeCache.txt" || {
    echo "FAIL: $C is not configured with BORG_IMPL=c"; exit 1; }
grep -qx 'BORG_IMPL:STRING=rust' "$R/CMakeCache.txt" 2>/dev/null || {
    echo "FAIL: $R is not configured with BORG_IMPL=rust"; exit 1; }

# compiler-builtins C members of the Rust staticlib (prefix: a hash)
BUILTINS="absvdi2 absvsi2 absvti2 addvdi3 addvsi3 addvti3 cmpdi2 cmpti2
divdc3 divsc3 ffsti2 int_util muldc3 mulsc3 mulvdi3 mulvsi3 mulvti3 negdf2
negdi2 negsf2 negti2 negvdi2 negvsi2 negvti2 paritydi2 paritysi2 parityti2
popcountdi2 popcountsi2 popcountti2 subvdi3 subvsi3 subvti3 ucmpdi2 ucmpti2"

# 1. no src/borg/*.c compiled
n=$(grep -c '"file": ".*/src/borg/.*\.c"' "$R/compile_commands.json" || true)
echo "1. rust build: src/borg/*.c entries in compile_commands.json: $n"
[ "$n" = 0 ] || fail=1
n=$(find "$R/CMakeFiles" -path '*src/borg/*' -name '*.o' | wc -l)
echo "   rust build: src/borg object files: $n"
[ "$n" = 0 ] || fail=1

TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT

# 2. staticlib members
mkdir "$TMP/ar"
( cd "$TMP/ar" && ar x "$LIB" )
nmem=$(ar t "$LIB" | wc -l)
nuniq=$(ar t "$LIB" | sort -u | wc -l)
[ "$nmem" = "$nuniq" ] || { echo "   FAIL: duplicate member names in $LIB"; fail=1; }
: > "$TMP/badmem"; nrust=0; ncrate=0; nbuiltin=0
while read -r m; do
    if [[ "$m" =~ ^[A-Za-z0-9_]+-[0-9a-f]+\.[A-Za-z0-9_.-]+\.rcgu\.o$ ]]; then
        cm=$(readelf -p .comment "$TMP/ar/$m" 2>/dev/null | sed -n 's/^ *\[ *[0-9a-f]*\]  //p')
        if grep -q '^rustc version ' <<< "$cm" && ! grep -qv '^rustc version ' <<< "$cm"; then
            nrust=$((nrust + 1))
            if [[ "$m" =~ ^borg_rs- ]]; then
                ncrate=$((ncrate + 1)); echo "$m" >> "$TMP/cratemem"
            fi
        else
            echo "$m (.comment: ${cm:-none})" >> "$TMP/badmem"
        fi
    elif [[ "$m" =~ ^[0-9a-f]+-([a-z0-9_]+)\.o$ ]] && \
         grep -qw "${BASH_REMATCH[1]}" <<< "$BUILTINS"; then
        nbuiltin=$((nbuiltin + 1))
    else
        echo "$m (not a rustc object nor a known compiler-builtins member)" >> "$TMP/badmem"
    fi
done < <(ar t "$LIB")
echo "2. staticlib members: $nmem; rustc objects: $nrust (borg_rs crate: $ncrate); compiler-builtins C objects: $nbuiltin"
if [ -s "$TMP/badmem" ]; then
    echo "   FAIL: unexpected staticlib members:"; sed 's/^/     /' "$TMP/badmem"; fail=1
fi
[ "$ncrate" -gt 0 ] || { echo "   FAIL: no borg_rs objects in $LIB"; fail=1; }

# 3. C Borg symbol names in the rust binary
nm --defined-only "$C"/CMakeFiles/OurCoreLib.dir/src/borg/*.o \
    2>/dev/null | awk 'NF==3{print $3}' | sed 's/\..*$//' | sort -u > "$TMP/cborg" || true
ncobj=$(find "$C/CMakeFiles/OurCoreLib.dir/src/borg" -name '*.o' 2>/dev/null | wc -l)
if [ "$ncobj" -lt 50 ]; then
    echo "   FAIL: only $ncobj C Borg object files in $C (expected 59)"; fail=1
fi
nm "$BIN" 2> "$TMP/nmerr" | awk 'NF>=2{print $NF}' > "$TMP/binall" || true
sort -u "$TMP/binall" > "$TMP/bin"
# global symbols defined by the crate's own rcgu objects
touch "$TMP/cratemem"
( cd "$TMP/ar" && xargs -r nm --defined-only < "$TMP/cratemem" ) \
    | awk 'NF==3 && $2 ~ /^[TDBRV]$/ {print $3}' | sort -u > "$TMP/rust"
nc=$(wc -l < "$TMP/cborg"); nb=$(wc -l < "$TMP/bin"); nr=$(wc -l < "$TMP/rust")
if [ "$nc" -lt 1000 ]; then
    echo "   FAIL: only $nc C Borg symbols in $C (expected > 1000)"; fail=1
fi
if [ "$nb" -lt 5000 ] || [ -s "$TMP/nmerr" ]; then
    echo "   FAIL: $BIN has $nb symbols (stripped?)"; cat "$TMP/nmerr"; fail=1
fi
if [ "$nr" -lt 100 ]; then
    echo "   FAIL: only $nr symbols exported by the borg_rs objects"; fail=1
fi
grep -qx do_cmd_borg "$TMP/rust" || { echo "   FAIL: do_cmd_borg not defined by borg_rs"; fail=1; }
comm -12 "$TMP/cborg" "$TMP/bin" > "$TMP/both"
nm --defined-only "$R"/CMakeFiles/OurCoreLib.dir/src/*.o \
    "$R"/CMakeFiles/OurExecutable.dir/src/*.o \
    | awk 'NF==3{print $3}' | sort | uniq -c | awk '{print $2, $1}' > "$TMP/engcount"
sort "$TMP/binall" | uniq -c | awk '{print $2, $1}' > "$TMP/bincount"
: > "$TMP/bad"
while read -r sym; do
    grep -qxF "$sym" "$TMP/rust" && continue
    e=$(awk -v s="$sym" '$1==s{print $2}' "$TMP/engcount"); e=${e:-0}
    b=$(awk -v s="$sym" '$1==s{print $2}' "$TMP/bincount"); b=${b:-0}
    if [ "$b" -gt "$e" ]; then echo "$sym (binary $b, engine $e)" >> "$TMP/bad"; fi
done < "$TMP/both"
echo "3. C Borg symbols: $nc; rust binary symbols: $nb; borg_rs exports: $nr;" \
     "C Borg names in the rust binary: $(wc -l < "$TMP/both")," \
     "of those exported by borg_rs: $(comm -12 "$TMP/both" "$TMP/rust" | wc -l)"
if [ -s "$TMP/bad" ]; then
    echo "   FAIL: C Borg symbols in the rust binary NOT provided by borg_rs:"
    sed 's/^/     /' "$TMP/bad"; fail=1
fi

# 4. engine objects identical
( cd "$C/CMakeFiles" && find . -name '*.o' ! -path '*/src/borg/*' | sort ) > "$TMP/cobj"
( cd "$R/CMakeFiles" && find . -name '*.o' ! -path '*/src/borg/*' | sort ) > "$TMP/robj"
same=0; : > "$TMP/objdiff"
if ! cmp -s "$TMP/cobj" "$TMP/robj"; then
    echo "   engine object lists differ:" >> "$TMP/objdiff"
    diff "$TMP/cobj" "$TMP/robj" | sed 's/^/     /' >> "$TMP/objdiff" || true
fi
while read -r o; do
    [ -f "$R/CMakeFiles/$o" ] || continue
    if cmp -s "$C/CMakeFiles/$o" "$R/CMakeFiles/$o"; then
        same=$((same + 1))
    elif [ "$(basename "$o")" != buildid.c.o ]; then
        echo "   differs: $o" >> "$TMP/objdiff"
    fi
done < "$TMP/cobj"
echo "4. engine objects: $(wc -l < "$TMP/cobj") in build/c, byte-identical in build/rust: $same (buildid.c.o may differ)"
[ "$(wc -l < "$TMP/cobj")" -ge 100 ] || { echo "   FAIL: too few engine objects"; fail=1; }
if [ -s "$TMP/objdiff" ]; then
    echo "   FAIL:"; cat "$TMP/objdiff"; fail=1
fi

# 5. actual link inputs of the rust binary
# Re-run the build's own link command with ld's --trace given twice (it
# then also names the archive members it pulls in) into a temporary file.
# The result must be byte-identical to $BIN, so the trace describes the
# binary that is checked. Every input must be an engine object of the
# build tree (listed in 4, so byte-identical to build/c), a member of the
# Rust staticlib that passed 2 (a rustc object or a known compiler-builtins
# object), or a system file (crt objects, libc/libm/libgcc, X11 libraries)
# under /usr/lib, /lib or /lib64. Anything else fails.
LINK="$R/CMakeFiles/OurExecutable.dir/link.txt"
if [ ! -f "$LINK" ] || ! grep -q ' -o game/angband ' "$LINK"; then
    echo "5. FAIL: no usable link command in $LINK"; fail=1
else
    lcmd=$(sed "s| -o game/angband | -Wl,--trace,--trace -o $TMP/relink |" "$LINK")
    if ! ( cd "$R" && eval "$lcmd" ) > "$TMP/trace" 2> "$TMP/linkerr"; then
        echo "5. FAIL: relinking failed:"; sed 's/^/     /' "$TMP/linkerr"; fail=1
    elif ! cmp -s "$TMP/relink" "$BIN"; then
        echo "5. FAIL: relinking does not reproduce $BIN byte for byte (stale or modified binary?)"; fail=1
    else
        ( cd "$R/CMakeFiles" && sed 's|^\./||' "$TMP/robj" ) > "$TMP/robjrel"
        ar t "$LIB" | sort > "$TMP/members"
        sed 's/ (.*$//' "$TMP/badmem" > "$TMP/badnames"
        realLIB=$(realpath "$LIB")
        : > "$TMP/badlink"; neng=0; nlib=0; nsys=0
        while IFS= read -r l; do
            [ -z "$l" ] && continue
            case "$l" in
                CMakeFiles/*.o)
                    o="${l#CMakeFiles/}"
                    if [[ "$o" == */src/borg/* ]] || ! grep -qxF "$o" "$TMP/robjrel"; then
                        echo "$l (not an engine object of $R)" >> "$TMP/badlink"
                    else neng=$((neng + 1)); fi;;
                \(*\)*)
                    a="${l#(}"; a="${a%%)*}"; m="${l#*)}"
                    case "$a" in
                        /usr/lib/*|/lib/*|/lib64/*) nsys=$((nsys + 1));;
                        *)
                            if [ "$(cd "$R" && realpath "$a")" != "$realLIB" ]; then
                                echo "$l (archive is not $LIB)" >> "$TMP/badlink"
                            elif ! grep -qxF "$m" "$TMP/members" || grep -qxF "$m" "$TMP/badnames"; then
                                echo "$l (not an accepted staticlib member, see 2)" >> "$TMP/badlink"
                            else nlib=$((nlib + 1)); fi;;
                    esac;;
                /usr/lib/*|/lib/*|/lib64/*) nsys=$((nsys + 1));;
                *)
                    if [ "$(cd "$R" && realpath "$l" 2>/dev/null)" = "$realLIB" ]; then :
                    else echo "$l (unknown link input)" >> "$TMP/badlink"; fi;;
            esac
        done < "$TMP/trace"
        echo "5. link inputs (ld --trace of a relink identical to the binary): engine objects: $neng;" \
             "staticlib members pulled in: $nlib; system files and members: $nsys"
        [ "$neng" -ge 100 ] || { echo "   FAIL: too few engine objects linked"; fail=1; }
        [ "$nlib" -gt 0 ] || { echo "   FAIL: no staticlib members linked"; fail=1; }
        if [ -s "$TMP/badlink" ]; then
            echo "   FAIL: unexpected link inputs:"; sed 's/^/     /' "$TMP/badlink"; fail=1
        fi
    fi
fi

if [ $fail = 0 ]; then echo "OK: no C Borg code in $BIN"; else echo "FAIL"; fi
exit $fail
