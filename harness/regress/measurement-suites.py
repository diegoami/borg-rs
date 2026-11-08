#!/usr/bin/env python3
"""Check opt-in measurement on the unchanged smoke/config suite inputs.

Uses fresh output paths and compares against already completed default runs
from the M2 corpus. Options do not change Borg choices: observational runs
match all three streams; exact caps retain the same key prefix.
"""
import argparse
import concurrent.futures
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('baseline', type=Path, help='completed default M2 suite output')
p.add_argument('--jobs', type=int, default=6)
p.add_argument('--reuse', type=Path, help='qualify prior successful cases, waiting for active cases to finish')
args = p.parse_args()
args.output = args.output.resolve()
args.output.mkdir(parents=True, exist_ok=False)
rows = []
for suite in ('smoke', 'config'):
    for line in (ROOT / 'harness/suites' / suite).read_text().splitlines():
        fields = line.split('#', 1)[0].split()
        if fields:
            rows.append(fields + (['-'] if len(fields) == 5 else []))


def run(job):
    impl, fields, option = job
    name, seed, race, cls, turns, flags = fields
    out = args.output / impl / name / option
    extra = {'states': ['-states', str(out / 'states.log')],
             'stall': ['-stall-decisions', '1000000000'],
             'exact': ['-exact-turns']}[option]
    reused = False
    if args.reuse:
        existing = args.reuse.resolve() / impl / name / option
        deadline = time.monotonic() + 950
        while (existing / 'process.json').exists():
            process = json.loads((existing / 'process.json').read_text())
            if process['status'] != 'running' or time.monotonic() >= deadline:
                break
            time.sleep(1)
        if (existing / 'process.json').exists() and (existing / 'summary.json').exists():
            process = json.loads((existing / 'process.json').read_text())
            prior = json.loads((existing / 'summary.json').read_text())
            config = dict(line.split('=', 1) for line in (existing / 'config.txt').read_text().splitlines())
            identity = dict(line.split('=', 1) for line in (ROOT / 'build' / impl / 'BUILD_ID').read_text().splitlines())
            if (process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0 and
                    process['executable_sha256_before'] == process['executable_sha256_after'] == identity['binary_sha256'] and
                    config['flags'] == flags and (prior['seed'], prior['race_idx'], prior['class_idx']) == (int(seed), int(race), int(cls)) and
                    process['command'][process['command'].index('-turns') + 1] == turns):
                out.parent.mkdir(parents=True, exist_ok=True)
                out.symlink_to(existing, target_is_directory=True)
                reused = True
    if not reused:
        subprocess.run([str(ROOT / 'harness/run.sh'), str(out), seed, race, cls, turns, flags, '--', *extra],
                       env={**os.environ, 'BORG_IMPL': impl, 'RUN_TIMEOUT_SECONDS': '900'}, check=True)
    summary = json.loads((out / 'summary.json').read_text())
    process = json.loads((out / 'process.json').read_text())
    command = process['command']
    assert process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0
    assert process['executable_sha256_before'] == process['executable_sha256_after']
    if option == 'states':
        assert '-states' in command
    elif option == 'stall':
        assert command[command.index('-stall-decisions') + 1] == '1000000000'
    else:
        assert '-exact-turns' in command and summary['exact_turns'] is True

    base = args.baseline / name
    base_summary = json.loads((base / 'summary.json').read_text())
    keys, base_keys = (out / 'keys.log').read_bytes(), (base / 'keys.log').read_bytes()
    if option == 'exact':
        assert base_keys.startswith(keys), (impl, name, option)
        if summary['stop_reason'] == 'turns':
            assert summary['game_turns'] == int(turns)
    else:
        for stream in ['keys.log', 'messages.log', 'state.log']:
            assert (out / stream).read_bytes() == (base / stream).read_bytes(), (impl, name, option, stream)
        for key in base_summary:
            if key != 'wall_seconds':
                assert base_summary[key] == summary[key], (impl, name, option, key)
    return {'impl': impl, 'run': name, 'option': option, 'keys': summary['keys'],
            'stop_reason': summary['stop_reason'], 'game_turns': summary['game_turns'],
            'origin': str(out.resolve()), 'reused': reused}


jobs = [(impl, fields, option) for fields in rows for impl in ['c', 'rust']
        for option in ['states', 'stall', 'exact']]
receipts = []
with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
    for receipt in pool.map(run, jobs):
        receipts.append(receipt)
        (args.output / 'results.json').write_text(json.dumps(receipts, indent=2) + '\n')
        print('PASS', receipt, flush=True)
for fields in rows:
    for option in ['states', 'stall', 'exact']:
        c = args.output / 'c' / fields[0] / option
        rust = args.output / 'rust' / fields[0] / option
        assert (c / 'keys.log').read_bytes() == (rust / 'keys.log').read_bytes()
        cs = json.loads((c / 'summary.json').read_text())
        rs = json.loads((rust / 'summary.json').read_text())
        assert {k: v for k, v in cs.items() if k != 'wall_seconds'} == {
            k: v for k, v in rs.items() if k != 'wall_seconds'}, (fields[0], option)

        if option == 'states':
            assert (c / 'states.log').read_bytes() == (rust / 'states.log').read_bytes()
(args.output / 'DONE').touch()
print('PASS:', len(receipts), 'measurement runs')
