#!/usr/bin/env bash
set -euo pipefail
cd /home/diego/projects/borg-rs
impl="$1"
for suite in smoke full deep classes config options captured-rogue crash-fixes natural; do
    out="harness/out/glyph-295/checkpoint-$impl/$suite"
    case "$suite" in
        natural) saves="$PWD/harness/saves/captured-extended";;
        *) saves="$PWD/harness/saves";;
    esac
    SAVE_ROOT="$saves" BORG_IMPL="$impl" JOBS=4 RUN_TIMEOUT_SECONDS=900 harness/suite.sh "$suite" "$out" > "/tmp/borg-rs-glyph-$impl-$suite.run.log" 2>&1
    harness/diff --suite "$suite" --save-root "$saves" --expect-impl "$impl" "$out" "harness/reference/$suite" > "/tmp/borg-rs-glyph-$impl-$suite.diff.log" 2>&1
    echo "PASS $impl $suite"
done
