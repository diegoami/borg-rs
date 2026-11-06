#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SOURCE="${1:-$ROOT/angband/src/borg/borg-fight-attack.c}"
OUT=$(mktemp -d "$ROOT/harness/out/melee-adjacent.XXXXXX")
python3 - "$SOURCE" "$OUT/thrust.inc" <<'PY'
from pathlib import Path
import sys,hashlib
source=Path(sys.argv[1]);text=source.read_text()
a=text.index('static int borg_attack_aux_thrust(void)')
b=text.index('/* adapted from player_attack.c',a)
Path(sys.argv[2]).write_text(text[:text.index('#include')]+text[a:b])
print('C body source:',source,'sha256='+hashlib.sha256(source.read_bytes()).hexdigest())
PY
cc -Wall -Wextra -Werror -I"$OUT" "$ROOT/harness/regress/melee-adjacent/probe.c" -o "$OUT/probe"
"$OUT/probe"
