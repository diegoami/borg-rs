"""Inject an interrupt after a real engine capture; replay the retained save.

This is a failure-injection control, not an accepted Borg oracle scenario.
Only the supervisor belonging to this newly launched suite is signalled.
"""
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[3]
out = Path(sys.argv[1]).resolve()
suite = out / "interrupt-suite"
suite.write_text("interrupted 101 0 0 1000000 menu,cheat\n")
destination = out / "interrupted-source"
env = os.environ.copy()
env.update(BORG_IMPL="c", JOBS="1", CHECKPOINT_EVERY="5000", RUN_TIMEOUT_SECONDS="0")
with (out / "interrupted-source.log").open("w") as log:
    process = subprocess.Popen([str(ROOT / "harness/suite.sh"), str(suite), str(destination)],
                               env=env, stdout=log, stderr=subprocess.STDOUT)
    deadline = time.monotonic() + 60
    interrupted = False
    while process.poll() is None and time.monotonic() < deadline:
        for working in out.glob("interrupted-source.partial.*"):
            run = working / "interrupted"
            index, receipt = run / "checkpoints/index.jsonl", run / "process.json"
            if index.is_file() and receipt.is_file():
                rows = index.read_bytes().splitlines(keepends=True)
                if rows and rows[0].endswith(b"\n") and receipt.stat().st_size:
                    status = json.loads(receipt.read_text())
                    assert status["status"] == "running"
                    assert status["command"][0] == str(ROOT / "build/c/game/angband")
                    os.kill(status["supervisor_pid"], signal.SIGTERM)
                    interrupted = True
                    break
        if interrupted:
            break
        time.sleep(0.02)
    if not interrupted:
        # Forward cancellation only to a supervisor in this owned suite.
        for working in out.glob("interrupted-source.partial.*"):
            receipt = working / "interrupted/process.json"
            if receipt.is_file() and receipt.stat().st_size:
                status = json.loads(receipt.read_text())
                if status["status"] in ("starting", "running"):
                    os.kill(status["supervisor_pid"], signal.SIGTERM)
        process.terminate()  # exact suite process this test started
        process.wait(timeout=10)
        raise AssertionError("engine did not produce a capture before the test deadline")
    assert process.wait(timeout=15) == 1
assert (destination / "FAILED").is_file() and not (destination / "DONE").exists()
run = destination / "interrupted"
receipt = json.loads((run / "process.json").read_text())
assert receipt["status"] == "interrupted" and receipt["checkpoint_count"] > 0
capture = json.loads((run / "checkpoints.json").read_text())["captures"][0]
replay = out / "replay-from-failure"
subprocess.run([str(ROOT / "harness/replay-checkpoint.py"), str(run), capture["file"],
                str(replay), "--seed", "7201", "--turns", "12000"], check=True)
assert json.loads((replay / "SOURCE.json").read_text())["source_suite_failed"]
assert (destination / "FAILED").is_file() and not (destination / "DONE").exists()
print("OK: interrupted source remains FAILED; its capture replays C/C/Rust strictly")
