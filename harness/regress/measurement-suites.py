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

ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('baseline', type=Path, help='completed default M2 suite output')
p.add_argument('--jobs', type=int, default=2)
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
    subprocess.run([str(ROOT / 'harness/run.sh'), str(out), seed, race, cls, turns, flags, '--', *extra],
                   env={**os.environ, 'BORG_IMPL': impl, 'RUN_TIMEOUT_SECONDS': '900'}, check=True)
    summary = json.loads((out / 'summary.json').read_text())
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
            'stop_reason': summary['stop_reason'], 'game_turns': summary['game_turns']}


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
        if option == 'states':
            assert (c / 'states.log').read_bytes() == (rust / 'states.log').read_bytes()
(args.output / 'DONE').touch()
print('PASS:', len(receipts), 'measurement runs')
