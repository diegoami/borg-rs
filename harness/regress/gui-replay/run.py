#!/usr/bin/env python3
"""GUI recording/replay on private Xvfb; fixtures and fresh birth, never owner saves."""
import argparse
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
import borg_gui
import borg_replay_viewer


def frames(bundle):
    path = bundle / 'run/frames.jsonl'
    if not path.exists():
        return []
    return [json.loads(line) for line in path.read_bytes().splitlines() if line.endswith(b'}')]


def stage(project, output, env, profile, name, character='GuiHero', options=(), borg=False, fresh=False, resize=False, reload_preferences=False):
    before = set((profile / 'recordings').glob('*')) if (profile / 'recordings').exists() else set()
    with (output / (name + '.log')).open('w') as log:
        game = subprocess.Popen([str(project / 'scripts/start-borg.sh'), character, *options], cwd='/tmp',
                                env={**env, 'BORG_PLAY_DATA_DIR': str(profile)}, stdout=log, stderr=subprocess.STDOUT)
        try:
            bundle = None
            for _ in range(200):
                assert game.poll() is None, (output / (name + '.log')).read_text()
                found = subprocess.run(['xdotool', 'search', '--name', '^Angband$'], env=env, capture_output=True, text=True)
                candidates = set((profile / 'recordings').glob('*')) - before
                if candidates:
                    bundle = candidates.pop()
                if found.returncode == 0 and bundle and (bundle / 'run/events.txt').exists() and (bundle / 'run/events.txt').stat().st_size > 0:
                    window = found.stdout.split()[0]
                    break
                time.sleep(.2)
            else:
                raise AssertionError('No private GUI window/recording')

            def key(value):
                sent = subprocess.run(['xdotool', 'key', '--window', window, value], env=env, capture_output=True, text=True)
                if sent.returncode:
                    # X may destroy the window during xdotool. Only confirmed
                    # success from the waiting launcher makes that acceptable.
                    game.wait(timeout=5)
                    assert game.returncode == 0, sent.stderr
                time.sleep(.25)

            key('space')
            if fresh:
                for _ in range(25):
                    if frames(bundle):
                        break
                    key('Return')
                assert frames(bundle), 'Fresh birth never reached the game'
                assert frames(bundle)[0]['level'] == 1
            time.sleep(.5)
            # The shell lock must span both the game and finalization.
            save = profile / 'save' / character
            with save.parent.joinpath('.locks', character + '.lock').open('a') as held:
                import fcntl
                try:
                    fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except BlockingIOError:
                    pass
                else:
                    raise AssertionError('Named save lock was released while GUI was open')
            if borg:
                key('ctrl+z')
                key('space')
                key('space')
                key('y')
                key('z')
                for _ in range(200):
                    if (bundle / 'run/actions.txt').stat().st_size > 0:
                        break
                    assert game.poll() is None, 'Game exited before Borg produced a key'
                    time.sleep(.1)
                before_stop = len(frames(bundle))
                key('Escape')
                for _ in range(200):
                    rows = frames(bundle)
                    if len(rows) > before_stop and rows[-1]['phase'] == 'wait':
                        break
                    time.sleep(.1)
                else:
                    raise AssertionError('Rust Borg did not return to a human input boundary')
                # Require real nonempty Rust hook output, not only menu events.
                assert (bundle / 'run/actions.txt').stat().st_size > 0
            if reload_preferences:
                config = profile / 'user/borg.txt'
                config.write_text(config.read_text() + '\n# changed while GUI is running\n')
                key('ctrl+z')
                key('dollar')
            if resize:
                subprocess.run(['xdotool', 'windowsize', window, '640', '420'], env=env, check=True)
                time.sleep(.5)
            if borg or fresh or resize:
                key('5')
            for _ in range(8):
                if game.poll() is not None:
                    break
                key('ctrl+x' if _ == 0 else 'Escape')
            game.wait(timeout=10)
            assert game.returncode == 0, (output / (name + '.log')).read_text()
            record = borg_gui.validate(bundle)
            proof = borg_gui.verify(bundle)
            assert proof['verified']
            rows = frames(bundle)
            if resize:
                assert len({(f['width'], f['height']) for f in rows}) > 1, 'Resize was not observed'
            if reload_preferences:
                assert any(b'# changed while GUI is running' in p.read_bytes() for p in (bundle / 'run').glob('file-input-*.bin')), 'Live preference reload was not captured'
            viewer = borg_replay_viewer.Frames(bundle)
            try:
                first, last = viewer.get(0)[0], viewer.get(len(viewer.index) - 1)[0]
            finally:
                viewer.close()
            result = {'case': name, 'bundle': str(bundle), 'verified': True, 'origin_sha256': record['origin_sha256'],
                      'final_sha256': record['final_sha256'], 'first': {k: first[k] for k in ('turn', 'level', 'depth')},
                      'last': {k: last[k] for k in ('turn', 'level', 'depth')}, 'frames': len(rows),
                      'borg_actions': len((bundle / 'run/actions.txt').read_text().splitlines())}
            print('PASS', name, json.dumps(result), flush=True)
            return result
        finally:
            if game.poll() is None:
                game.terminate()
                game.wait(timeout=15)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--project-root', type=Path, required=True, help='Disposable checkout for the GUI build')
    parser.add_argument('--skip-build', action='store_true')
    args = parser.parse_args()
    project, output = args.project_root.resolve(), args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    if not args.skip_build:
        with (output / 'build.log').open('w') as log:
            subprocess.run([str(project / 'scripts/run-rust-borg.sh'), '--build-only'], stdout=log, stderr=subprocess.STDOUT, check=True)
    number = str(1000 + os.getpid() % 30000)
    with (output / 'xvfb.log').open('w') as log:
        server = subprocess.Popen(['Xvfb', ':' + number, '-displayfd', '1', '-screen', '0', '1600x1000x24', '-nolisten', 'tcp'],
                                  stdout=subprocess.PIPE, stderr=log, text=True)
        try:
            with selectors.DefaultSelector() as ready:
                ready.register(server.stdout, selectors.EVENT_READ)
                assert ready.select(15) and server.stdout.readline().strip() == number, 'Private X server failed'
            env = {**os.environ, 'DISPLAY': ':' + number, 'BORG_PLAY_FONT': 'fixed'}
            profile = output / 'profile'
            (profile / 'save').mkdir(parents=True)
            shutil.copy2(project / 'harness/saves/depth50.save', profile / 'save/GuiHero')
            proof = [stage(project, output, env, profile, 'load-save')]
            proof.append(stage(project, output, env, profile, 'reload-save'))
            other = project / 'harness/saves/depth98.save'
            shutil.copy2(other, profile / 'save/GuiHero')
            proof.append(stage(project, output, env, profile, 'restore-save', options=('--from-backup=1',)))
            preserved = list((profile / 'save/backups/GuiHero').glob('before-restore-*.save'))
            assert len(preserved) == 1 and preserved[0].read_bytes() == other.read_bytes()
            # A clean fixture gives the first-use warning and real Borg keys.
            shutil.copy2(project / 'harness/saves/depth50.save', profile / 'save/GuiHero')
            proof.append(stage(project, output, env, profile, 'borg-intervene-reload-resize', borg=True, resize=True, reload_preferences=True))
            shutil.copy2(project / 'harness/saves/rogue-4006-cp01-t0000100001.save', profile / 'save/RogueHero')
            proof.append(stage(project, output, env, profile, 'loaded-rogue', character='RogueHero'))
            proof.append(stage(project, output, env, profile, 'fresh-birth', character='NewHero', fresh=True))
            (output / 'results.json').write_text(json.dumps(proof, indent=2) + '\n')
            (output / 'DONE').write_text('6 actual GUI sessions, exact replay, private display/profiles; lock, restore, Borg, manual input, preferences, geometry, fresh birth\n')
        finally:
            server.terminate()
            server.wait(timeout=5)
            server.stdout.close()


if __name__ == '__main__':
    main()
