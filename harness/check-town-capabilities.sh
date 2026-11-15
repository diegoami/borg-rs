#!/usr/bin/env bash
# Bounded engine-backed regression gate; requires a current Rust harness build.
# Every output is fresh. These nine 1000-turn cases are wiring checks, not a
# survival experiment. Run under the local validation lock when used locally.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$(realpath -m "${1:-$ROOT/harness/out/town-capability-check}")"
[ ! -e "$OUT" ] || { echo "use a fresh capability-check output: $OUT" >&2; exit 2; }
mkdir -p "$OUT"
cd "$ROOT"
python3 tools/town-plan/catalogue.py
git diff --exit-code -- docs/TOWN-ITEM-CATALOGUE.md docs/town-item-catalogue.html
python3 -m unittest discover -s tools/town-plan -p 'test_*.py'
for CLASS_IDX in {0..8}; do
    CASE_OUT="$OUT/class-$CLASS_IDX"
    BORG_IMPL=rust RUN_TIMEOUT_SECONDS=30 \
      BORG_TOWN_PLAN_RECORD="$CASE_OUT/plan.jsonl" \
      BORG_TRIP_RECORD="$CASE_OUT/trip.jsonl" \
      BORG_TOWN_CAPABILITY_AUDIT="$CASE_OUT/capability-audit.jsonl" \
      harness/run.sh "$CASE_OUT" "$((501 + CLASS_IDX))" 0 "$CLASS_IDX" 1000 \
      menu,borg_town_plan=1,borg_trip_policy=1
 done
python3 tools/town-plan/check-coverage.py "$OUT"/*/capability-audit.jsonl --out "$OUT/coverage.json"
python3 - "$OUT" <<'PY'
import json
from pathlib import Path
import sys
root = Path(sys.argv[1])
classes = [line[5:] for line in Path('angband/lib/gamedata/class.txt').read_text().splitlines() if line.startswith('name:')]
for cls in range(9):
    summary = json.loads((root / f'class-{cls}' / 'summary.json').read_text())
    assert summary['class'] == classes[cls]
    assert summary['stop_reason'] in ('turns', 'death')
    assert summary['effective_settings']['borg_town_plan'] == 1
    assert summary['effective_settings']['borg_trip_policy'] == 1
print('Nine native class contexts and ordinary shop coverage passed')
PY
