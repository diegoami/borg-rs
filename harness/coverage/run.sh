#!/usr/bin/env bash
# Collect coverage from an isolated instrumented checkout, checking every oracle.
# Usage: run.sh COVERAGE_CHECKOUT REFERENCE_ROOT NEW_OUTPUT_DIRECTORY
# See README.md for the instrumentation/runtime/exit-flush build recipe.
set -euo pipefail
# Keep the coordinator immutable while long groups run. Bash can otherwise read
# a changed on-disk footer after completing an already-parsed loop.
if [ "${1:-}" != --internal-script-snapshot ]; then
    COVERAGE_SCRIPT_SOURCE=$(cat -- "$0")
    exec bash -c "$COVERAGE_SCRIPT_SOURCE" "$0" --internal-script-snapshot "$@"
fi
shift
WORKTREE=$(realpath "$1")
REFERENCES=$(realpath "$2")
OUT=$(realpath -m "$3")
[ ! -e "$OUT" ] || { echo "refusing existing output: $OUT" >&2; exit 2; }
[ -f "$WORKTREE/build/rust/BUILD_ID" ] || { echo "missing coverage build" >&2; exit 2; }
mkdir -p "$OUT/profiles"
cp "$WORKTREE/build/rust/BUILD_ID" "$OUT/BUILD_ID"
printf '%s\n' "$REFERENCES" > "$OUT/REFERENCE_ROOT"
cd "$WORKTREE"
ALL_SUITES='smoke full deep classes config options captured-rogue crash-fixes extended natural synthetic-tactics'
SELECTED_SUITES=${COVERAGE_SUITES:-$ALL_SUITES}
for suite in $SELECTED_SUITES; do
    case " $ALL_SUITES " in
        *" $suite "*) ;;
        *) echo "unknown coverage suite: $suite" >&2; exit 2;;
    esac
done
for suite in $SELECTED_SUITES; do
    echo "START $suite $(date -u +%FT%TZ)"
    mkdir "$OUT/profiles/$suite"
    SAVE_ROOT="$WORKTREE/harness/saves"
    DIFF_SAVE_ARGS=()
    if [ "$suite" = natural ]; then
        SAVE_ROOT="$WORKTREE/harness/saves/captured-extended"
        DIFF_SAVE_ARGS=(--save-root "$SAVE_ROOT")
    fi
    if [ "$suite" = synthetic-tactics ]; then
        SAVE_ROOT="$WORKTREE/harness/saves/synthetic-tactics"
        DIFF_SAVE_ARGS=(--save-root "$SAVE_ROOT")
    fi
    SAVE_ROOT="$SAVE_ROOT" LLVM_PROFILE_FILE="$OUT/profiles/$suite/%m-%p.profraw" CHECKPOINT_EVERY=0 \
      BORG_IMPL=rust JOBS="${JOBS:-4}" RUN_TIMEOUT_SECONDS="${RUN_TIMEOUT_SECONDS:-1800}" \
      harness/suite.sh "$suite" "$OUT/$suite" >"$OUT/$suite.log" 2>&1
    test -f "$OUT/$suite/DONE"
    harness/diff --suite "$suite" "${DIFF_SAVE_ARGS[@]}" --expect-impl rust "$OUT/$suite" "$REFERENCES/$suite" \
      >"$OUT/$suite.diff" 2>&1
    python3 - "$OUT" "$suite" <<'PY'
import hashlib, json, sys
from pathlib import Path
out, suite = Path(sys.argv[1]), sys.argv[2]
profiles = out / 'profiles' / suite
runs = [p for p in (out/suite).iterdir() if p.is_dir()]
files = sorted(profiles.glob('*.profraw'))
assert len(files) == len(runs), (suite, len(files), len(runs))
receipt = {}
for run in sorted(runs):
    process = json.loads((run/'process.json').read_text())
    assert process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0
    assert process['executable_sha256_before'] == process['executable_sha256_after']
    matches = list(profiles.glob('*-%d.profraw' % process['child_pid']))
    assert len(matches) == 1 and matches[0].stat().st_size > 0, run
    profile = matches[0]
    receipt[run.name] = {'child_pid': process['child_pid'], 'profile': str(profile),
                         'bytes': profile.stat().st_size,
                         'sha256': hashlib.sha256(profile.read_bytes()).hexdigest()}
(out/suite/'profiles.json').write_text(json.dumps(receipt, indent=2, sort_keys=True)+'\n')
print('%s: %d successful engine PIDs, %d nonempty profiles' % (suite, len(runs), len(files)))
PY
    tail -3 "$OUT/$suite.diff"
    echo "PASS $suite $(date -u +%FT%TZ)"
done
date -u +%FT%TZ > "$OUT/GROUPS_DONE"
if [ "$SELECTED_SUITES" = "$ALL_SUITES" ]; then
    cp "$OUT/GROUPS_DONE" "$OUT/DONE"
fi
