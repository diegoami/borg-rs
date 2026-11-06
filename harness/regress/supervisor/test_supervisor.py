"""Execution-wrapper tests with deliberately failing child processes, not Borg oracles."""
import hashlib
import json
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[3]
SUPERVISOR = ROOT / "harness/supervise.py"


class SupervisorTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)

    def command(self, out, code, timeout=0):
        out.mkdir(exist_ok=True)
        return [sys.executable, str(SUPERVISOR), "--receipt", str(out / "process.json"),
                "--diagnostics", str(out), "--timeout", str(timeout), "--",
                sys.executable, "-c", code]

    def invoke(self, out, code, timeout=0):
        result = subprocess.run(self.command(out, code, timeout), capture_output=True, timeout=10)
        return result.returncode, json.loads((out / "process.json").read_text())

    def test_failure_is_not_retried_and_next_process_runs(self):
        out = self.root / "bad"
        attempts = out / "attempts"
        rc, receipt = self.invoke(out, f"from pathlib import Path; Path({str(attempts)!r}).write_text('one'); raise SystemExit(7)")
        self.assertEqual((rc, receipt["engine_returncode"], receipt["status"]), (7, 7, "exit_failure"))
        self.assertEqual(attempts.read_text(), "one")
        self.assertFalse((out / "summary.json").exists())
        good = self.root / "good"
        rc, receipt = self.invoke(good, f"from pathlib import Path; Path({str(good / 'summary.json')!r}).write_text('{{}}')")
        self.assertEqual((rc, receipt["status"]), (0, "success"))

    def test_signal_and_missing_summary_stay_failures(self):
        rc, receipt = self.invoke(self.root / "signal", "import os,signal; os.kill(os.getpid(),signal.SIGTERM)")
        self.assertEqual((rc, receipt["engine_returncode"], receipt["signal"]), (143, -15, 15))
        rc, receipt = self.invoke(self.root / "empty", "pass")
        self.assertEqual((rc, receipt["status"]), (1, "missing_summary"))

    def test_timeout_does_not_signal_an_unrelated_process(self):
        other = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(60)"])
        try:
            rc, receipt = self.invoke(self.root / "timeout", "import time; time.sleep(60)", 0.2)
            self.assertEqual((rc, receipt["status"], receipt["timed_out"]), (124, "timeout", True))
            self.assertIsNone(other.poll())
        finally:
            other.terminate()  # exact process this test started
            other.wait(timeout=5)

    def test_interrupt_is_forwarded_and_receipt_survives(self):
        out = self.root / "interrupt"
        child = subprocess.Popen(self.command(out, "import time; time.sleep(60)"))
        try:
            deadline = time.monotonic() + 5
            while time.monotonic() < deadline:
                path = out / "process.json"
                if path.exists() and path.stat().st_size:
                    if json.loads(path.read_text())["status"] == "running":
                        break
                time.sleep(0.01)
            else:
                self.fail("supervisor did not start")
            child.send_signal(signal.SIGTERM)
            self.assertEqual(child.wait(timeout=5), 143)
            receipt = json.loads((out / "process.json").read_text())
            self.assertEqual((receipt["status"], receipt["interrupted_signal"]), ("interrupted", 15))
        finally:
            if child.poll() is None:
                child.kill()
                child.wait()

    def test_checkpoint_is_retained_when_child_crashes(self):
        out = self.root / "capture"
        name = "checkpoint-000001-t0000005000.save"
        row = dict(file=name, due_turn=5000, game_turn=5000, keys=123, seed=42,
                   race_idx=0, class_idx=0, depth=2, max_depth=3, clevel=5)
        code = ("from pathlib import Path; import os,signal; "
                f"p=Path({str(out / 'checkpoints')!r}); p.mkdir(); "
                f"(p/{name!r}).write_bytes(b'test payload'); "
                f"(p/'index.jsonl').write_text({(json.dumps(row) + chr(10))!r}); "
                "os.kill(os.getpid(),signal.SIGTERM)")
        rc, receipt = self.invoke(out, code)
        self.assertEqual((rc, receipt["checkpoint_count"]), (143, 1))
        captures = json.loads((out / "checkpoints.json").read_text())
        self.assertEqual(captures["source_process_status"], "signal")
        self.assertEqual(captures["captures"][0]["save_sha256"], hashlib.sha256(b"test payload").hexdigest())
        self.assertFalse((out / "summary.json").exists())

    def test_partial_index_prevents_success(self):
        out = self.root / "partial"
        code = ("from pathlib import Path; "
                f"p=Path({str(out)!r}); (p/'checkpoints').mkdir(); "
                "(p/'checkpoints/index.jsonl').write_text('{'); "
                "(p/'summary.json').write_text('{}')")
        rc, receipt = self.invoke(out, code)
        self.assertEqual((rc, receipt["status"]), (3, "checkpoint_error"))
        self.assertTrue(receipt["checkpoint_errors"])

    def test_existing_receipt_is_never_overwritten(self):
        out = self.root / "existing"
        self.invoke(out, "raise SystemExit(9)")
        before = (out / "process.json").read_bytes()
        result = subprocess.run(self.command(out, "raise SystemExit(0)"), capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((out / "process.json").read_bytes(), before)


if __name__ == "__main__":
    unittest.main()
