#!/usr/bin/env python3
"""Private X11 takeover, human action, save/reload and frozen GUI replay.

Requires a prepared branch. Uses a disposable copy and its own Xvfb; never the
owner display/profile. Does not prepare a new original death or mark review.
"""
import argparse
import fcntl
import json
import os
from pathlib import Path
import selectors
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools'))
import borg_gui as gui
import borg_runtime as runtime
import borg_replay_viewer
import human_takeover as human


def stage(branch, output, env, name, manual=False, abort=False):
    profile = branch / 'profile'
    before = set((profile / 'recordings').glob('*'))
    logpath = output / (name + '.log')
    with logpath.open('w') as log:
        child = subprocess.Popen([str(ROOT / 'play-branch.py'), str(branch)], env=env, stdout=log, stderr=subprocess.STDOUT)
        try:
            bundle = None
            window = None
            deadline = time.monotonic() + 35
            while time.monotonic() < deadline:
                assert child.poll() is None, logpath.read_text()
                candidates = set((profile / 'recordings').glob('*')) - before
                if candidates:
                    bundle = candidates.pop()
                found = subprocess.run(['xdotool', 'search', '--name', '^Angband$'], env=env, capture_output=True, text=True)
                if found.returncode == 0:
                    window = found.stdout.split()[0]
                if bundle and (bundle / 'run/human-approved').exists():
                    break
                if window:
                    # Acknowledge the native startup screen only. The command
                    # handshake blocks all actual commands until validated.
                    subprocess.run(['xdotool', 'key', '--window', window, 'space'], env=env, capture_output=True)
                time.sleep(.2)
            else:
                raise AssertionError('No approved private GUI: ' + logpath.read_text())
            assert window and bundle
            ready = json.loads((bundle / 'run/human-ready.json').read_text())
            assert not ready['borg_active'] and not ready['dead']
            with (profile / 'save/.locks/HumanBorg.lock').open('a') as held:
                try:
                    fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except BlockingIOError:
                    pass
                else:
                    raise AssertionError('Human launcher released its save lock')
            if abort:
                child.terminate()
                child.wait(timeout=20)
                assert child.returncode != 0, 'Interrupted launcher reported success'
                with (profile / 'save/.locks/HumanBorg.lock').open('a') as held:
                    fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
                found = subprocess.run(['xdotool', 'search', '--name', '^Angband$'], env=env, capture_output=True)
                assert found.returncode != 0, 'Interrupted launcher left its GUI child running'
                try:
                    gui.validate(bundle)
                except ValueError:
                    pass
                else:
                    raise AssertionError('Interrupted GUI accepted as a complete recording')
                return {'case': name, 'intentional_interrupt': True, 'child_stopped': True,
                        'lock_released_after_child_exit': True, 'incomplete_rejected': True}
            if manual:
                # One explicit player wait at a safe test turn, no Borg input.
                subprocess.run(['xdotool', 'key', '--window', window, '5'], env=env, check=True)
                time.sleep(.3)
            subprocess.run(['xdotool', 'key', '--window', window, 'ctrl+x'], env=env, capture_output=True)
            # Native close_game(): Escape exits; Return opens the score list.
            for _ in range(50):
                if child.poll() is not None:
                    break
                view = borg_replay_viewer.Frames(bundle / 'run')
                try:
                    _, chars, _, _ = view.get(len(view.index) - 1)
                    text = ''.join(map(chr, chars))
                finally:
                    view.close()
                if 'Press Return (or Escape)' in text or 'Angband Hall of Fame' in text:
                    subprocess.run(['xdotool', 'key', '--window', window, 'Escape'], env=env, capture_output=True)
                time.sleep(.1)
            child.wait(timeout=15)
            assert child.returncode == 0, logpath.read_text()
            record = gui.validate(bundle)
            assert not (bundle / 'run/actions.txt').read_text().strip(), 'Unexpected Borg action'
            assert record['session_context']['independent_life'] is False
            assert gui.verify(bundle, output=output / (name + '-replay'), timeout=60)['verified']
            final = human.inspect(branch, profile / 'save/HumanBorg')
            if manual:
                assert final['total_energy'] > ready['total_energy'], 'Manual wait never consumed a turn'
            return {'case': name, 'bundle': str(bundle), 'ready': ready, 'final': final,
                    'frozen_gui_replay': True, 'borg_actions': 0, 'final_sha256': record['final_sha256']}
        finally:
            if child.poll() is None:
                child.terminate()
                child.wait(timeout=15)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('branch', type=Path, help='prepared living branch; use a safe town turn for --manual')
    parser.add_argument('output', type=Path)
    parser.add_argument('--manual', action='store_true')
    parser.add_argument('--abort-check', action='store_true', help='also verify SIGTERM stops only the owned child and releases its lock')
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    branch = output / 'branch'
    shutil.copytree(args.branch.resolve(), branch, ignore=shutil.ignore_patterns('prefix', 'recordings', 'capture-console.log'))
    number = str(1000 + os.getpid() % 30000)
    with (output / 'xvfb.log').open('w') as log:
        server = subprocess.Popen(['Xvfb', ':' + number, '-displayfd', '1', '-screen', '0', '1600x1000x24', '-nolisten', 'tcp'],
                                  stdout=subprocess.PIPE, stderr=log, text=True)
        try:
            with selectors.DefaultSelector() as ready:
                ready.register(server.stdout, selectors.EVENT_READ)
                assert ready.select(15) and server.stdout.readline().strip() == number, 'Private X server failed'
            env = {**os.environ, 'DISPLAY': ':' + number, 'BORG_PLAY_FONT': 'fixed',
                   'BORG_DEATH_RECORDS_DIR': str(output / 'must-not-register-deaths')}
            first = stage(branch, output, env, 'human-save', manual=args.manual)
            second = stage(branch, output, env, 'human-resume')
            human.same_state(first['final'], second['ready'])
            assert not (output / 'must-not-register-deaths').exists(), 'Human counterfactual registered as an independent life'
            results = {'passed': True, 'stages': [first, second]}
            if args.abort_check:
                results['interrupt_control'] = stage(branch, output, env, 'human-interrupt', abort=True)
            runtime.write(output / 'results.json', results)
            print('PASS human action/save/resume, initial state/RNG/Borg-off, locks and both frozen GUI replays')
        finally:
            server.terminate()
            server.wait(timeout=5)
            server.stdout.close()


if __name__ == '__main__':
    main()
