#!/usr/bin/env bash
# Capture is observational on these controls; a replay is a separate scenario.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
mkdir -p "$ROOT/harness/out"
OUT=$(mktemp -d "$ROOT/harness/out/checkpoint-regress.XXXXXX")
export SAVE_ROOT="$ROOT/harness/saves"
for spec in c-plain:c:0 c-capture1:c:5000 c-capture2:c:5000 rust-plain:rust:0 rust-capture:rust:5000; do
    IFS=: read -r label impl every <<< "$spec"
    BORG_IMPL="$impl" JOBS=2 CHECKPOINT_EVERY="$every" \
        "$ROOT/harness/suite.sh" smoke "$OUT/$label" >"$OUT/$label.log" 2>&1
done
compare() { "$ROOT/harness/diff" --suite smoke --expect-impl "$1" "$OUT/$2" "$OUT/$3"; }
compare c c-plain c-capture1
compare c c-capture1 c-capture2
compare rust rust-plain rust-capture
compare rust rust-capture c-capture1
"$ROOT/harness/diff" --suite smoke --expect-impl c "$OUT/c-plain" "$ROOT/harness/reference/smoke"
"$ROOT/harness/diff" --suite smoke --expect-impl rust "$OUT/rust-plain" "$ROOT/harness/reference/smoke"
python3 - "$OUT" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
for run in ('smoke-warrior-1', 'smoke-mage-2', 'smoke-priest-3'):
    rows = [json.loads((root / label / run / 'checkpoints.json').read_text())['captures']
            for label in ('c-capture1', 'c-capture2', 'rust-capture')]
    assert rows[0] and rows[0] == rows[1] == rows[2], run
    assert all(r['game_turn'] >= r['due_turn'] for r in rows[0])
PY
name=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["captures"][0]["file"])' \
    "$OUT/c-capture1/smoke-warrior-1/checkpoints.json")
"$ROOT/harness/replay-checkpoint.py" "$OUT/c-capture1/smoke-warrior-1" "$name" \
    "$OUT/fresh-replay" --seed 7200 --turns 12000
# A deeper engine state exercises saving live monster/object lists. This
# is an observer control, not a replacement for the endgame suite.
echo 'deep-control 6000 0 0 20000 menu,cheat,save=depth50.save' > "$OUT/deep-control"
for spec in deep-plain:0 deep-capture:5000; do
    IFS=: read -r label every <<< "$spec"
    BORG_IMPL=c JOBS=1 CHECKPOINT_EVERY="$every" \
        "$ROOT/harness/suite.sh" "$OUT/deep-control" "$OUT/$label" >"$OUT/$label.log" 2>&1
done
"$ROOT/harness/diff" --suite "$OUT/deep-control" --expect-impl c "$OUT/deep-plain" "$OUT/deep-capture"
# Uniform timeout control: every scenario must still be attempted and the
# suite must fail, without fabricating summaries or a DONE marker.
if BORG_IMPL=c JOBS=1 CHECKPOINT_EVERY=0 RUN_TIMEOUT_SECONDS=0.001 \
    "$ROOT/harness/suite.sh" smoke "$OUT/timeouts" >"$OUT/timeouts.log" 2>&1; then
    echo 'FAIL: timed-out suite passed' >&2; exit 1
fi
[ -f "$OUT/timeouts/FAILED" ] && [ ! -e "$OUT/timeouts/DONE" ]
if "$ROOT/harness/diff" --suite smoke --expect-impl c "$OUT/timeouts" "$OUT/c-plain" \
    >"$OUT/timeout-refusal.log" 2>&1; then
    echo 'FAIL: timed-out suite accepted by strict diff' >&2; exit 1
fi
grep -q 'marked FAILED' "$OUT/timeout-refusal.log"
python3 - "$OUT/timeouts" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
for name in ('smoke-warrior-1', 'smoke-mage-2', 'smoke-priest-3'):
    row = json.loads((root / name / 'process.json').read_text())
    assert row['status'] == 'timeout' and row['exit_code'] == 124
PY
python3 "$ROOT/harness/regress/checkpoint/interrupt.py" "$OUT"
echo "OK: supervisor-backed runs, observational capture, identical saves and fresh replay ($OUT)"
