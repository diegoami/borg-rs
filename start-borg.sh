#!/usr/bin/env bash
# Named Rust launcher; workflow adapted from diegoami/better-borg-c.
set -euo pipefail
root=$(cd -- "$(dirname -- "$0")" && pwd)
if [ $# -gt 0 ] && [[ "$1" != -* ]]; then
    export BORG_PLAY_CHARACTER="$1"
    shift
fi
exec "$root/run-rust-borg.sh" "$@"
