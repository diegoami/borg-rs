#!/usr/bin/env python3
"""Run the five suites outside the current M2 reference corpus.

Compare both rebuilt backends against the preserved pre-harness M2 binary.
Historical M4 references remain untouched: they predate gameplay adoption.
"""
# Historical (borg-rs#60): this verifies a receipt recorded before the munchkin
# and lunal modes were removed, and reads the lunal-investigation suite and the
# synth-lunal cases, which the current tree no longer has. Run it from the
# checkout the receipt names.

import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('--baseline', type=Path, required=True)
p.add_argument('--jobs', type=int, default=6)
p.add_argument('--timeout', type=int, default=7200)
p.add_argument('--reuse', type=Path, help='reuse only qualified successful cases; preserve original attempts')
p.add_argument('--baseline-ref', default='0bc7a09fb41a01cfbc4f5bb86b89002b074f3712')
p.add_argument('--measurement-ref', help='bind frozen measured images to this committed input tree')
p.add_argument('--verify-only', action='store_true', help='requalify an existing collection without launching games')
a = p.parse_args()
out = a.output.resolve()
out.mkdir(parents=True, exist_ok=a.verify_only)
binary = a.baseline.resolve()
build_id = dict(line.split('=', 1) for line in (binary.parents[1] / 'BUILD_ID').read_text().splitlines())
assert hashlib.sha256(binary.read_bytes()).hexdigest() == build_id['binary_sha256']
assert build_id['impl'] == 'rust'
# Bind dirty-build stamps to exact committed input trees, not their old
# git_rev metadata alone. The baseline predates the measurement adapter.
def committed_source_hash(ref, impl):
    paths = ['angband', 'harness/measurement']
    if impl == 'rust':
        paths += ['borg-rs', 'Cargo.toml', 'Cargo.lock']
    entries = subprocess.check_output(['git', 'ls-tree', '-rz', ref, '--', *paths], cwd=ROOT)
    pairs = []
    for entry in entries.split(b'\0'):
        if entry:
            meta, path = entry.split(b'\t')
            pairs.append((path, meta.split()[-1]))
    source = ('borg-rs build inputs v1 impl=' + impl + '\n').encode()
    source += b''.join(sha + b' ' + path + b'\n' for path, sha in sorted(pairs))
    return hashlib.sha256(source).hexdigest()


assert committed_source_hash(a.baseline_ref, 'rust') == build_id['source_sha256']
build_id['verified_source_ref'] = a.baseline_ref
(out / 'baseline-identity.json').write_text(json.dumps(build_id, indent=2) + '\n')
suites = ['captured-rogue', 'endgame', 'extended', 'lunal-investigation', 'synthetic-tactics']
identities = {'baseline': build_id}
for impl in ['c', 'rust']:
    image = ROOT / 'build' / impl / 'game/angband'
    identity = dict(line.split('=', 1) for line in (image.parents[1] / 'BUILD_ID').read_text().splitlines())
    assert identity['impl'] == impl
    assert hashlib.sha256(image.read_bytes()).hexdigest() == identity['binary_sha256']
    source_hash = (committed_source_hash(a.measurement_ref, impl) if a.measurement_ref else
                   subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'source-hash', impl], text=True).strip())
    assert source_hash == identity['source_sha256']
    if a.measurement_ref:
        identity['verified_source_ref'] = a.measurement_ref
    identities[impl] = identity
(out / 'build-identities.json').write_text(json.dumps(identities, indent=2) + '\n')



def run(job):
    role, suite, fields = job
    name, seed, race, cls, turns, *flag = fields
    flags = flag[0] if flag else '-'
    target = out / role / suite / name
    target.parent.mkdir(parents=True, exist_ok=True)
    impl = 'rust' if role == 'baseline' else role
    executable = binary if role == 'baseline' else ROOT / 'build' / role / 'game/angband'
    identity = identities[role]
    if a.reuse:
        existing = a.reuse.resolve() / role / suite / name
        while (existing / 'process.json').exists():
            prior_process = json.loads((existing / 'process.json').read_text())
            deadline = prior_process['started_unix'] + prior_process['timeout_seconds'] + 30
            if prior_process['status'] != 'running' or time.time() >= deadline:
                break
            time.sleep(1)
        if (existing / 'process.json').exists() and (existing / 'summary.json').exists():
            process = json.loads((existing / 'process.json').read_text())
            summary = json.loads((existing / 'summary.json').read_text())
            config = dict(line.split('=', 1) for line in (existing / 'config.txt').read_text().splitlines())
            command = process['command']
            valid = (process['status'] == 'success' and
                     process['engine_returncode'] == process['exit_code'] == 0 and
                     process['executable_sha256_before'] == process['executable_sha256_after'] == identity['binary_sha256'] and
                     config['flags'] == flags and
                     (summary['seed'], summary['race_idx'], summary['class_idx']) == (int(seed), int(race), int(cls)) and
                     command[command.index('-turns') + 1] == turns)
            if valid:
                target.symlink_to(existing, target_is_directory=True)
                return {'role': role, 'suite': suite, 'name': name, 'origin': str(existing), 'reused': True}
    env = {**os.environ, 'BORG_IMPL': impl, 'RUN_TIMEOUT_SECONDS': str(a.timeout),
           'ANGBAND_BIN': str(executable)}
    save_root = ROOT / 'harness/saves'
    if suite in ['synthetic-tactics', 'lunal-investigation']:
        save_root /= 'synthetic-tactics'
    env['SAVE_ROOT'] = str(save_root)
    subprocess.run([str(ROOT / 'harness/run.sh'), str(target), *fields[1:]], env=env, check=True)
    process = json.loads((target / 'process.json').read_text())
    assert process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0
    assert process['executable_sha256_before'] == process['executable_sha256_after'] == identity['binary_sha256']
    return {'role': role, 'suite': suite, 'name': name, 'origin': str(target), 'reused': False}


jobs = []
for suite in suites:
    for line in (ROOT / 'harness/suites' / suite).read_text().splitlines():
        fields = line.split('#', 1)[0].split()
        if fields:
            jobs.extend((role, suite, fields) for role in ['baseline', 'c', 'rust'])
if a.verify_only:
    origins = json.loads((out / 'execution-origins.json').read_text())
    assert {(r['role'], r['suite'], r['name']) for r in origins} == {
        (role, suite, fields[0]) for role, suite, fields in jobs}
else:
    origins = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=a.jobs) as pool:
        for result in pool.map(run, jobs):
            origins.append(result)
            (out / 'execution-origins.json').write_text(json.dumps(origins, indent=2) + '\n')
            print('PASS', result, flush=True)

# These are explicitly attributed case joins, not fabricated suite.sh runs.
# Construct the shared build-identity manifest only after checking every
# selected process, command, configuration and source binding.
process_ids = set()
for role in ['baseline', 'c', 'rust']:
    identity = identities[role]
    executable = binary if role == 'baseline' else ROOT / 'build' / role / 'game/angband'
    impl = subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'binary-impl', str(executable)], text=True).strip()
    assert impl == identity['impl']
    cache_impl = next(line.split('=', 1)[1] for line in
                      (executable.parents[1] / 'CMakeCache.txt').read_text().splitlines()
                      if line.startswith('BORG_IMPL:'))
    assert cache_impl == impl
    for suite in suites:
        directory = out / role / suite
        rows = [(fields, directory / fields[0]) for job_role, job_suite, fields in jobs
                if job_role == role and job_suite == suite]
        manifest = {
            'producer': 'harness/regress/measurement-remaining.py attributed case join',
            'case_join': 'true', 'suite': suite,
            'suite_sha256': hashlib.sha256((ROOT / 'harness/suites' / suite).read_bytes()).hexdigest(),
            'binary': str(executable), 'binary_impl': impl, 'binary_elf_impl': impl,
            'binary_cmake_borg_impl': cache_impl, 'build_id_impl': impl, 'borg_impl_env': impl,
            'binary_sha256': identity['binary_sha256'], 'binary_sha256_end': identity['binary_sha256'],
            'build_check': 'ok', 'impl_check': 'ok', 'overrides': 'none',
            'source_verification_ref': identity.get('verified_source_ref', a.measurement_ref or 'current checked tree'),
            'git_rev': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
            'git_dirty': 'yes' if subprocess.check_output(['git', 'status', '--porcelain'], cwd=ROOT) else 'no',
            'execution_origins_sha256': hashlib.sha256((out / 'execution-origins.json').read_bytes()).hexdigest(),
        }
        for key in ['git_rev', 'git_dirty', 'inputs_dirty', 'source_sha256', 'binary_sha256', 'cc', 'rustc', 'built_utc']:
            if key in identity:
                manifest['build_id_' + key] = identity[key]
        budgets = set()
        for fields, row in rows:
            name, seed, race, cls, turns, *flag = fields
            flags = flag[0] if flag else '-'
            process = json.loads((row / 'process.json').read_text())
            command = process['command']
            config = dict(line.split('=', 1) for line in (row / 'config.txt').read_text().splitlines())
            assert process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0
            assert process['executable_sha256_before'] == process['executable_sha256_after'] == identity['binary_sha256']
            for option, expected in [('-seed', seed), ('-race', race), ('-class', cls), ('-turns', turns)]:
                assert command[command.index(option) + 1] == expected
            assert config['flags'] == flags
            assert config['borg_txt_sha256'] == hashlib.sha256((row / 'user/borg.txt').read_bytes()).hexdigest()
            token = (process['child_pid'], process['started_unix'])
            assert token not in process_ids
            process_ids.add(token)
            budgets.add(process['timeout_seconds'])
            manifest['run_config.' + name] = flags + ' ' + config['borg_txt_sha256']
        manifest['case_wall_budgets_seconds'] = ','.join(map(str, sorted(budgets)))
        (directory / 'manifest').write_text(''.join(key + '=' + str(value) + '\n' for key, value in manifest.items()))
        (directory / 'DONE').touch()

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
