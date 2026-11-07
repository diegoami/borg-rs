"""C-compatible replay parameters and terminal-navigation controls."""
from collections import deque
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
import curses

import borg_replay_viewer as viewer

ROOT = Path(__file__).resolve().parents[1]


class Navigation(unittest.TestCase):
    def test_csi_and_application_cursor_sequences(self):
        for encoded, expected in [('\x1b[C', curses.KEY_RIGHT), ('\x1bOD', curses.KEY_LEFT), ('\x1b[5~', curses.KEY_PPAGE), ('\x1b[6~', curses.KEY_NPAGE), ('\x1b', 27), ('g', ord('g'))]:
            queue = deque(map(ord, encoded))
            class Screen:
                def getch(self):
                    return queue.popleft() if queue else -1
                def timeout(self, value):
                    pass
            with self.subTest(encoded=encoded):
                self.assertEqual(viewer.read_key(Screen()), expected)


@unittest.skipUnless(os.environ.get('BORG_DEATH_TEST_RECORDING'), 'Set BORG_DEATH_TEST_RECORDING to a real bundle')
class CompatibleCli(unittest.TestCase):
    def run_cli(self, *args):
        return subprocess.run([str(ROOT / 'replay-borg.py'), os.environ['BORG_DEATH_TEST_RECORDING'], *args], capture_output=True, text=True)

    def test_from_turn_and_old_turn_choose_identical_original_frame(self):
        first = self.run_cli('--from-turn', '5596', '--view-only', '--dump')
        old = self.run_cli('--turn', '5596', '--recording-only', '--dump')
        dump = self.run_cli('--dump-turn', '5596', '--view-only')
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual(first.stdout, old.stdout)
        self.assertEqual(first.stdout, dump.stdout)
        self.assertIn('NOT RERUN-VERIFIED', first.stdout)
        self.assertIn('turn=5596', first.stdout)

    def test_retained_output_is_real_verified_rerun_and_not_overwritten(self):
        with tempfile.TemporaryDirectory(prefix='borg-cli-control-') as temporary:
            output = Path(temporary) / 'verification'
            result = self.run_cli('--verify-only', '--timeout', '30', '--output', str(output))
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue((output / 'run/final.save').is_file())
            before = (output / 'verification.json').read_bytes()
            repeated = self.run_cli('--verify-only', '--output', str(output))
            self.assertNotEqual(repeated.returncode, 0)
            self.assertEqual((output / 'verification.json').read_bytes(), before)

    def test_native_death_log_body_is_compared_after_manifest_rehash(self):
        import json
        import shutil
        import borg_runtime as runtime
        with tempfile.TemporaryDirectory(prefix='borg-native-log-control-') as temporary:
            bundle = Path(temporary) / 'bundle'
            shutil.copytree(os.environ['BORG_DEATH_TEST_RECORDING'], bundle)
            log = bundle / 'run/archive/borg-log.txt'
            with log.open('a') as output:
                output.write('changed death facts\n')
            record = json.loads((bundle / 'recording.json').read_text())
            record['files_sha256']['run/archive/borg-log.txt'] = runtime.sha(log)
            runtime.write(bundle / 'recording.json', record)
            with self.assertRaisesRegex(ValueError, 'native death log'):
                runtime.verify(bundle)

    def test_relative_bundle_path_replays_with_absolute_engine_path(self):
        import borg_runtime as runtime
        relative = Path(os.path.relpath(os.environ['BORG_DEATH_TEST_RECORDING']))
        self.assertTrue(runtime.verify(relative, timeout=30)['verified'])

    def test_invalid_flags_rejected_before_running(self):
        for args in [('--timeout', '0'), ('--dump-turn', '-1'), ('--from-turn', '-1'), ('--verify-only', '--view-only')]:
            with self.subTest(args=args):
                self.assertNotEqual(self.run_cli(*args).returncode, 0)


if __name__ == '__main__':
    unittest.main()
