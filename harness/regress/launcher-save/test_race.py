"""Deterministic process/window race controls for the GUI driver."""
import importlib.util
from pathlib import Path
import subprocess
import unittest
from unittest.mock import Mock

spec = importlib.util.spec_from_file_location('launcher_roundtrip', Path(__file__).with_name('run.py'))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


class WindowExitRace(unittest.TestCase):
    def test_exited_game_receives_no_further_keys(self):
        process = Mock(returncode=0)
        process.poll.return_value = 0
        process.wait.return_value = 0
        send = Mock()
        driver.save_and_exit(process, 'owned-window', {}, send)
        send.assert_not_called()

    def test_bad_window_after_successful_exit_is_allowed(self):
        process = Mock(returncode=0)
        process.poll.return_value = None
        process.wait.return_value = 0
        send = Mock(return_value=subprocess.CompletedProcess([], 1, '', 'BadWindow'))
        driver.save_and_exit(process, 'owned-window', {}, send)
        send.assert_called_once()

    def test_bad_window_with_live_game_is_an_error(self):
        process = Mock()
        process.poll.return_value = None
        process.wait.side_effect = subprocess.TimeoutExpired('game', 1)
        send = Mock(return_value=subprocess.CompletedProcess([], 1, '', 'BadWindow'))
        with self.assertRaisesRegex(RuntimeError, 'remained alive'):
            driver.save_and_exit(process, 'owned-window', {}, send)

    def test_failed_game_exit_is_not_hidden(self):
        process = Mock(returncode=3)
        process.poll.return_value = None
        process.wait.return_value = 3
        send = Mock(return_value=subprocess.CompletedProcess([], 1, '', 'BadWindow'))
        with self.assertRaisesRegex(RuntimeError, 'status 3'):
            driver.save_and_exit(process, 'owned-window', {}, send)


if __name__ == '__main__':
    unittest.main()
