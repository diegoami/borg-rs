#!/usr/bin/env python3
"""Repeat GUI save/reload on a private X server and disposable fixture profiles."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]


def save_and_exit(process, window, environment, send=subprocess.run):
    for _ in range(5):
        if process.poll() is not None:
            break
        key = send(['xdotool', 'key', '--window', window, 'ctrl+x'],
                   env=environment, capture_output=True, text=True)
        if key.returncode:
            # The window can disappear during xdotool itself. Only a confirmed
            # successful game exit may turn that key-send failure into success.
            try:
                process.wait(timeout=1)
            except subprocess.TimeoutExpired as error:
                raise RuntimeError('X key failed while the game remained alive: ' + key.stderr) from error
            break
        try:
            process.wait(timeout=.5)
            break
        except subprocess.TimeoutExpired:
            pass
    process.wait(timeout=5)
    if process.returncode != 0:
        raise RuntimeError(f'game exited with status {process.returncode}')


def stage(project, output, environment, profile, name):
    save = profile / 'save/RustBorg'
    before = save.stat().st_mtime_ns
    with (output / (name + '.log')).open('w') as log:
        game = subprocess.Popen([str(project / 'run-rust-borg.sh')], cwd='/tmp',
                                env={**environment, 'BORG_PLAY_DATA_DIR': str(profile)},
                                stdout=log, stderr=subprocess.STDOUT)
        try:
            window = None
            for _ in range(160):
                if game.poll() is not None:
                    raise RuntimeError(f'early game exit {game.returncode}')
                found = subprocess.run(['xdotool', 'search', '--name', '^Angband$'],
                                       env=environment, capture_output=True, text=True)
                if found.returncode == 0 and found.stdout.strip():
                    window = found.stdout.split()[0]
                    break
                time.sleep(.25)
            assert window, 'no normal X11 window'
            subprocess.run(['xdotool', 'key', '--window', window, 'space'],
                           env=environment, check=True)
            time.sleep(1)
            save_and_exit(game, window, environment)
            assert save.stat().st_size > 0 and save.stat().st_mtime_ns != before, 'save not replaced'
            character = (profile / 'user/CharOutput.txt').read_text()
            assert re.search(r'cLvl:\s*"50"', character), character
            return {'stage': name, 'bytes': save.stat().st_size,
                    'sha256': hashlib.sha256(save.read_bytes()).hexdigest(),
                    'exit_code': game.returncode, 'character': character}
        finally:
            if game.poll() is None:
                game.terminate()
                game.wait(timeout=5)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--project-root', type=Path, required=True,
                        help='isolated checkout whose launcher/build is tested')
    parser.add_argument('--runs', type=int, default=3)
    args = parser.parse_args()
    assert args.runs > 0
    project, output = args.project_root.resolve(), args.output.resolve()
    assert not output.exists(), 'refusing existing output'
    output.mkdir(parents=True)
    # Configure/build before the window timeout starts, in the supplied checkout.
    with (output / 'build.log').open('w') as log:
        subprocess.run([str(project / 'run-rust-borg.sh'), '--build-only'],
                       stdout=log, stderr=subprocess.STDOUT, check=True)
    proof = []
    expected = [row['sha256'] for row in json.loads(
        (ROOT / 'docs/validation/2026-11-07-launcher/save-roundtrip.json').read_text())]
    with (output / 'xvfb.log').open('w') as log:
        # A fixed private number also works when /tmp/.X11-unix is read-only:
        # Xvfb can keep its abstract listener, while auto-selection rejects it.
        number = str(1000 + os.getpid() % 30000)
        server = subprocess.Popen(['Xvfb', ':' + number, '-displayfd', '1', '-screen', '0',
                                   '1600x1000x24', '-nolisten', 'tcp'],
                                  stdout=subprocess.PIPE, stderr=log, text=True)
        try:
            with selectors.DefaultSelector() as ready:
                ready.register(server.stdout, selectors.EVENT_READ)
                assert ready.select(timeout=15), 'private X server startup timed out'
                display = server.stdout.readline().strip()
            assert display == number, 'private X server failed to start'
            environment = {**os.environ, 'DISPLAY': ':' + display}
            for number in range(args.runs):
                with tempfile.TemporaryDirectory(prefix='borg-launcher-proof-') as temp:
                    first, second = Path(temp) / 'first', Path(temp) / 'reload'
                    (first / 'save').mkdir(parents=True)
                    (second / 'save').mkdir(parents=True)
                    shutil.copyfile(project / 'harness/saves/depth50.save', first / 'save/RustBorg')
                    rows = [stage(project, output, environment, first, f'{number}-first')]
                    shutil.copyfile(first / 'save/RustBorg', second / 'save/RustBorg')
                    rows.append(stage(project, output, environment, second, f'{number}-reload'))
                    assert [row['sha256'] for row in rows] == expected, rows
                    proof.extend(rows)
                    print(f'PASS roundtrip {number + 1}/{args.runs}: both exact M4 save hashes', flush=True)
        finally:
            server.terminate()
            server.wait(timeout=5)
            server.stdout.close()
    (output / 'results.json').write_text(json.dumps(proof, indent=2) + '\n')
    (output / 'DONE').write_text(f'{args.runs} complete save/reload roundtrips; private display and fixture profiles only\n')


if __name__ == '__main__':
    main()
