#!/usr/bin/env bash
# Start a recorded Angband session; human play or optional Rust Borg.
set -euo pipefail
root=$(cd -- "$(dirname -- "$0")" && pwd)
exec python3 "$root/tools/start_recorded_borg.py" "$@"
