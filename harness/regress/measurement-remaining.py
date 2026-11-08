#!/usr/bin/env python3
"""Run the five suites outside the current M2 reference corpus.

Compare both rebuilt backends against the preserved pre-harness M2 binary.
Historical M4 references remain untouched: they predate gameplay adoption.
"""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('--baseline', type=Path, required=True)
p.add_argument('--jobs', type=int, default=3)
a = p.parse_args()
out = a.output.resolve()
out.mkdir(parents=True, exist_ok=False)
binary = a.baseline.resolve()
build_id = dict(line.split('=', 1) for line in (binary.parents[1] / 'BUILD_ID').read_text().splitlines())
assert hashlib.sha256(binary.read_bytes()).hexdigest() == build_id['binary_sha256']
(out / 'baseline-identity.json').write_text(json.dumps(build_id, indent=2) + '\n')
suites = ['captured-rogue', 'endgame', 'extended', 'lunal-investigation', 'synthetic-tactics']


def run(job):
    role, suite = job
    env = {**os.environ, 'BORG_IMPL': 'rust' if role == 'baseline' else role,
           'RUN_TIMEOUT_SECONDS': '900'}
    if role == 'baseline':
        env['ANGBAND_BIN'] = str(binary)
    save_root = ROOT / 'harness/saves'
    if suite in ['synthetic-tactics', 'lunal-investigation']:
        save_root /= 'synthetic-tactics'
    env['SAVE_ROOT'] = str(save_root)
    for line in (ROOT / 'harness/suites' / suite).read_text().splitlines():
        fields = line.split('#', 1)[0].split()
        if not fields:
            continue
        name, *params = fields
        subprocess.run([str(ROOT / 'harness/run.sh'), str(out / role / suite / name), *params],
                       env=env, check=True)
        print('DONE', role, suite, name, flush=True)
    (out / role / suite / 'DONE').touch()
    return role, suite


with concurrent.futures.ThreadPoolExecutor(max_workers=a.jobs) as pool:
    list(pool.map(run, [(role, suite) for suite in suites for role in ['baseline', 'c', 'rust']]))
for suite in suites:
    for role in ['c', 'rust']:
        with (out / (role + '-' + suite + '.diff')).open('w') as log:
            subprocess.run([str(ROOT / 'harness/diff'), '--suite', suite,
                            '--save-root', str(ROOT / 'harness/saves' / 'synthetic-tactics')
                            if suite in ['synthetic-tactics', 'lunal-investigation']
                            else str(ROOT / 'harness/saves'),
                            str(out / role / suite), str(out / 'baseline' / suite)],
                           stdout=log, stderr=subprocess.STDOUT, check=True)
    # Every added field is also compared between the new C/Rust backends.
    with (out / ('parity-' + suite + '.diff')).open('w') as log:
        subprocess.run([str(ROOT / 'harness/diff'), str(out / 'c' / suite), str(out / 'rust' / suite)],
                       stdout=log, stderr=subprocess.STDOUT, check=True)
(out / 'DONE').touch()
print('PASS: all five remaining suites against preserved pre-harness binary, C/Rust parity')
