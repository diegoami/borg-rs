#!/usr/bin/env python3
"""Check native recorder round trips from birth and from an existing save."""
import argparse
from collections import Counter
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = 'angband-recorder-streams@2'


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def run_game(exe, out, mode, *, cap, load=None, save=None):
    env = dict(os.environ, BORG_TOOLKIT_HEADLESS='1')
    for name in ['BORG_RECORD_DIR', 'BORG_REPLAY_DIR', 'ANGBAND_RECORD_DIR', 'ANGBAND_REPLAY_DIR']:
        env.pop(name, None)
    env['ANGBAND_RECORD_DIR'] = str(out / mode)
    if mode == 'replay':
        env['ANGBAND_REPLAY_DIR'] = str(out / 'record')
    command = [str(exe), '-mborgh']
    if load is None:
        command.append('-n')
    command += ['-uRecorderCheck']
    command += ['-d' + k + '=' + str(out / 'profile' / k)
                for k in ['user', 'save', 'archive', 'panic', 'scores']]
    command += ['--', '-seed', '42', '-race', '0', '-class', '0', '-turns', str(cap),
                '-summary', str(out / (mode + '-summary.json')),
                '-save-on-stop', str(save or out / (mode + '.save'))]
    if load is not None:
        command += ['-load', str(load), '-keep-load-rng']
    checkpoints = out / (mode + '-checkpoints')
    checkpoints.mkdir()
    command += ['-checkpoint-every', '50', '-checkpoint-dir', str(checkpoints)]
    with (out / (mode + '.log')).open('w') as log:
        with subprocess.Popen(command, cwd=exe.parent, env=env, stdin=subprocess.DEVNULL,
                              stdout=log, stderr=subprocess.STDOUT, start_new_session=True) as child:
            try:
                status = child.wait(timeout=30)
            except BaseException:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
                raise
    require(status == 0, f'{mode} failed; inspect {out / (mode + ".log")}')
    require((out / mode / 'complete.json').exists(), f'{mode}: missing completion receipt')
    identity = json.loads((out / mode / 'recording.json').read_text())
    require(identity['schema'] == SCHEMA, f'{mode}: wrong recorder schema')
    summary = json.loads((out / (mode + '-summary.json')).read_text())
    require(not summary['dead'], 'short integration control unexpectedly died')
    require(summary['game_turns'] >= cap, f'{mode}: game stopped before its turn cap')
    if load is not None:
        require(summary.get('loaded') and summary['loaded_turn'] > 0, 'silently started a new game')
    return summary


def verify_case(exe, out, reader, *, cap, load=None):
    out.mkdir()
    target = out / 'loaded-character.save' if load else None
    if target:
        shutil.copyfile(load, target)
        panic = out / 'profile/panic' / target.name
        panic.parent.mkdir(parents=True)
        shutil.copyfile(load, panic)
        os.utime(panic, (1, 1))
        os.utime(target, (2, 2))
    run_game(exe, out, 'record', cap=cap, load=target, save=target)
    record_save = out / 'record.save'
    if target:
        # The original target existed at save time. In replay it must be
        # absent, while the input save is supplied from captured bytes.
        # Replaying a recorded file_exists(True) here used to break saving.
        target.rename(record_save)
        require(not target.exists(), 'loaded replay save target is still present')
    run_game(exe, out, 'replay', cap=cap, load=target, save=target)
    replay_save = target or out / 'replay.save'
    require(record_save.read_bytes() == replay_save.read_bytes(), 'final engine saves differ')
    checkpoints = [{p.name: p.read_bytes() for p in (out / (mode + '-checkpoints')).glob('*.save')}
                   for mode in ['record', 'replay']]
    require(checkpoints[0] and checkpoints[0] == checkpoints[1], 'forked checkpoints differ')
    events = [json.loads(line) for line in (out / 'record/inputs.jsonl').read_text().splitlines()]
    kinds = Counter(event['kind'] for event in events)
    require(kinds['bot'] > 0, 'no bot actions captured')
    require(kinds['terminal_init'] == 1, 'terminal initialization was not recorded exactly once')
    if target:
        require(kinds['newer'] > 0, 'panic recovery did not record its file-date decision')
    require((out / 'record/frames.jsonl').stat().st_size > 0, 'no observed frames')
    subprocess.run(['cargo', 'run', '--quiet', '--locked', '--manifest-path', str(reader / 'Cargo.toml'),
                    '--target-dir', str(reader.parent / 'reader-build'), '--', 'verify',
                    str(out / 'record'), str(out / 'replay')], check=True)
    return {'case': 'loaded-save' if target else 'fresh-birth', 'schema': SCHEMA,
            'final_save_sha256': hashlib.sha256(record_save.read_bytes()).hexdigest(),
            'final_save_bytes_match': True, 'matching_checkpoints': len(checkpoints[0]),
            'input_families': dict(kinds), 'loaded_target_removed_before_replay': bool(target)}


def verify_save_menu(exe, out, reader, load):
    """Replay save-menu enumeration when its live directory is empty."""
    out.mkdir()
    target = out / 'profile/save' / f'{os.getuid()}.RecorderCheck'
    target.parent.mkdir(parents=True)
    shutil.copyfile(load, target)
    command = [str(exe), '-mtest', '-c', '-uRecorderCheck']
    command += ['-d' + k + '=' + str(out / 'profile' / k)
                for k in ['user', 'save', 'archive', 'panic', 'scores']]
    keys = 'key C-[\n' * 4 + 'key C-x\n'
    for mode in ['record', 'replay']:
        env = dict(os.environ)
        for name in ['BORG_RECORD_DIR', 'BORG_REPLAY_DIR', 'ANGBAND_RECORD_DIR',
                     'ANGBAND_REPLAY_DIR', 'BORG_TOOLKIT_HEADLESS']:
            env.pop(name, None)
        env['ANGBAND_RECORD_DIR'] = str(out / mode)
        if mode == 'replay':
            env['ANGBAND_REPLAY_DIR'] = str(out / 'record')
        with (out / (mode + '.log')).open('w') as log:
            with subprocess.Popen(command, cwd=exe.parent, env=env, stdin=subprocess.PIPE,
                                  stdout=log, stderr=subprocess.STDOUT,
                                  start_new_session=True, text=True) as child:
                try:
                    child.communicate(keys if mode == 'record' else '', timeout=30)
                except BaseException:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                    raise
        require(child.returncode == 0, f'save-menu {mode} failed; inspect {out / (mode + ".log")}')
        require((out / mode / 'complete.json').exists(), f'save-menu {mode}: incomplete recording')
        if mode == 'record':
            target.unlink()
        else:
            require(not target.exists(), 'menu-only replay unexpectedly created a game save')
    kinds = Counter(json.loads(line)['kind'] for line in (out / 'record/inputs.jsonl').read_text().splitlines())
    require(kinds['directory_open'] and kinds['directory_read'], 'save-menu enumeration hooks were not reached')
    subprocess.run(['cargo', 'run', '--quiet', '--locked', '--manifest-path', str(reader / 'Cargo.toml'),
                    '--target-dir', str(reader.parent / 'reader-build'), '--', 'verify',
                    str(out / 'record'), str(out / 'replay')], check=True)
    require(any(event['kind'] == 'directory_read' and event.get('name') == list(target.name.encode())
                for event in map(json.loads, (out / 'record/inputs.jsonl').read_text().splitlines())),
            'save-menu did not enumerate the fixture save')
    return {'case': 'save-menu-enumeration', 'schema': SCHEMA, 'input_families': dict(kinds),
            'live_save_directory_empty_before_replay': True, 'enumerated_save_name': target.name}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    exe = args.binary.resolve()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    with open('/tmp/angband-validation.lock', 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        # Keep Cargo's generic reader outside the consumer workspace.
        reader = out / 'reader-source'
        shutil.copytree(ROOT / 'vendor/angband-recorder', reader,
                        ignore=shutil.ignore_patterns('target', '__pycache__', '.git'))
        fresh = verify_case(exe, out / 'fresh', reader, cap=300)
        start = json.loads((out / 'fresh/record-summary.json').read_text())['game_turns']
        loaded = verify_case(exe, out / 'loaded', reader, cap=start + 300,
                             load=out / 'fresh/record.save')
        menu = verify_save_menu(exe, out / 'save-menu', reader, out / 'fresh/record.save')
        result = {'binary_sha256': hashlib.sha256(exe.read_bytes()).hexdigest(),
                  'recorder_pin': json.loads((ROOT / 'recorder-pin.json').read_text())['commit'],
                  'cases': [fresh, loaded, menu]}
        (out / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
