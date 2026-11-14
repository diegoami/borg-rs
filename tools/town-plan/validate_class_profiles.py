#!/usr/bin/env python3
"""Validate exported class profiles through a pinned, unmodified zoo adapter.

Run under flock /tmp/angband-validation.lock. This is a 36-cell shadow
integration probe, not a live policy or per-class fitness experiment.
"""
import argparse
import hashlib
import importlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from class_profiles import CLASSES, ROOT, export_zoo, fingerprint, load_profiles, resolve, selected_profile

ZOO_PIN = '7ae123b8240e92428470e9c2b6145eebc9ef614b'
ZOO_BLOBS = {'catalogue.py': '7b3a074d31a1519e63aca22d5f52ac6907db5794',
             'borgrs.py': 'bfdcc7d3c2104bda6d5096122a376dd37506ec60',
             'stall.py': '6e20567b44d7df379fb49b473123f336eb6437e3'}
SEEDS = (901, 1901)  # predeclared wiring check + fresh-seed repeat, never used to fit settings
ARMS = ('reference', 'class-selection-probe')
CAP = 2000


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--zoo-root', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    if args.out.exists():
        parser.error('use a fresh output directory; original evidence is preserved')
    for module, expected in ZOO_BLOBS.items():
        data = (args.zoo_root / 'zoo' / module).read_bytes()
        actual = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
        if actual != expected:
            parser.error(f'{module} does not match zoo {ZOO_PIN}')
    sys.path.insert(0, str(args.zoo_root.resolve()))
    catalogue = importlib.import_module('zoo.catalogue')
    borgrs = importlib.import_module('zoo.borgrs')
    if tuple(catalogue.CLASSES) != CLASSES:
        parser.error('Borg and zoo class order disagree')
    profiles = load_profiles()
    exported = export_zoo(profiles, ARMS)
    args.out.mkdir(parents=True)
    save(args.out / 'profiles.json', exported)
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    identity = dict(line.split('=', 1) for line in (ROOT / 'build/rust/BUILD_ID').read_text().splitlines() if '=' in line)
    inputs = subprocess.check_output(['harness/identity.sh', 'source-hash', 'rust'], cwd=ROOT, text=True).strip()
    if inputs != identity['source_sha256'] or sha(ROOT / 'build/rust/game/angband') != identity['binary_sha256']:
        parser.error('rebuild Rust: source or executable does not match BUILD_ID')
    build = args.out / 'build'
    build.mkdir()
    # Reuse the verified binary/data instead of compiling an identical runtime.
    # The original BUILD_ID stays visible; source_commit binds equal input blobs.
    (build / 'game').symlink_to(ROOT / 'build/rust/game', target_is_directory=True)
    shutil.copy2(ROOT / 'harness/borg.txt', build / 'borg.txt')
    build_meta = {'kind': 'borg-rs', 'impl': 'rust', 'source_commit': head,
                  'binary_sha256': identity['binary_sha256'], 'build_id': identity,
                  'runtime_inputs_sha256': inputs, 'complete': True,
                  'qualification': 'reused executable; exact runtime-input and binary hashes checked; original pre-commit build identity retained'}
    save(build / 'build.json', build_meta)
    specimen_name = 'rs-town-class-shadow'
    spec = {'kind': 'borg-rs', 'impl': 'rust', 'repository': 'diegoami/borg-rs',
            'ref': head, 'mode_side': 'post', 'depth_stream': True}
    suites = {f'town-class-{cls.lower()}': {
        'birth': 'assigned', 'seeds': list(SEEDS), 'cap': CAP, 'wall_seconds': 120,
        'assigned_characters': {str(seed): ['Human', cls] for seed in SEEDS},
        'role': 'integration probe; fresh-seed repeat is not fitness confirmation'} for cls in CLASSES}
    cat = {'specimens': {'specimens': {specimen_name: spec}}, 'profiles': exported['profiles'], 'suites': suites}
    exp = {'schema': 'borg-zoo/experiment@1', 'name': 'town-class-profile-integration',
           'specimens': [specimen_name], 'profiles': [f'rs-town-{a}-by-class-normal' for a in ARMS],
           'suites': list(suites), 'control': specimen_name,
           'control_profile': 'rs-town-reference-by-class-normal'}
    save(args.out / 'experiment.json', exp)
    catalogue.experiment(args.out / 'experiment.json', cat)
    cells = catalogue.cells(exp, cat)
    save(args.out / 'frozen-plan.json', {
        'zoo_commit': ZOO_PIN, 'module_blobs': ZOO_BLOBS, 'source_commit': head,
        'profiles_sha256': fingerprint(profiles), 'cells': cells, 'cap': CAP,
        'seeds': list(SEEDS), 'classes': list(CLASSES), 'mode': 'normal; shadow only',
        'endpoint': 'correct selection/written+parsed settings/replay; gameplay identical within class+seed',
        'selection_rule': 'prespecified probe overrides; no parameters selected using outcomes'})
    results, failures, replay_lines = [], [], []
    previous = os.environ.get('BORG_TOWN_PLAN_RECORD')
    try:
        for cell in cells:
            native_out = args.out / 'work' / catalogue.cell_key(cell)
            assert not native_out.exists()
            os.environ['BORG_TOWN_PLAN_RECORD'] = str(native_out / 'plan.jsonl')
            try:
                record = borgrs.run_cell(cell, build, args.out / 'work', cat)
                expected_profile = catalogue.resolve_profile(cat, cell)[0]
                profile = exported['profiles'][expected_profile]
                summary = json.loads((native_out / 'summary.json').read_text())
                checks = {
                    'profile': record['profile_applied'] == expected_profile,
                    'profile_hash': record['profile_sha256'] == fingerprint(profile),
                    'written_config': record['config_error'] is None,
                    'parsed_settings': all(summary['effective_settings'].get(k) == int(v) for k, v in profile['settings'].items()),
                    'played_class': summary['class'] == cell['class'],
                    'outcome': record['outcome'] in ('death', 'turn_cap', 'completion'),
                }
                arm = ARMS[exp['profiles'].index(cell['profile'])]
                settings, sources = resolve(profiles, arm, cell['class'])
                checks['export_matches_resolver'] = profile['settings'] == settings
                checks['selector_matches_export'] = expected_profile == selected_profile(profiles, arm, cell['class'])
                lines = (native_out / 'plan.jsonl').read_text().splitlines()
                rows = [json.loads(line) for line in lines]
                checks['observed'] = bool(rows) and all(r['v'] == 5 for r in rows)
                # Parameters are contiguous 45..68 in the actual engine table.
                all_town = [name for name in summary['effective_settings'] if name.startswith('borg_town_') and name != 'borg_town_no_stores']
                checks['recorded_vector'] = all(len(r['params']) == 24 and r['params'] == [summary['effective_settings'][k] for k in all_town] for r in rows)
                replay_lines += lines
                save(native_out / 'profile-selection.json', {'selected': expected_profile, 'settings': settings, 'sources': sources, 'checks': checks})
                result = {'class': cell['class'], 'seed': cell['seed'], 'arm': arm,
                          'profile_applied': expected_profile, 'settings_sha256': fingerprint(settings),
                          'checks': checks, 'records': len(rows), 'raw_directory': str(native_out),
                          'plan_sha256': sha(native_out / 'plan.jsonl'), 'cell': record}
                results.append(result)
                if not all(checks.values()):
                    failures.append({'cell': cell, 'checks': checks})
                print(cell['class'], cell['seed'], arm, record['outcome'], 'PASS' if all(checks.values()) else 'FAIL', flush=True)
            except Exception as exc:
                failures.append({'cell': cell, 'error': repr(exc)})
                print('FAILED', cell, repr(exc), flush=True)
    finally:
        if previous is None:
            os.environ.pop('BORG_TOWN_PLAN_RECORD', None)
        else:
            os.environ['BORG_TOWN_PLAN_RECORD'] = previous
    pairs, class_rows = [], {}
    for cls in CLASSES:
        for seed in SEEDS:
            matched = [r for r in results if r['class'] == cls and r['seed'] == seed]
            if len(matched) != 2:
                failures.append({'class': cls, 'seed': seed, 'error': 'missing paired attempt'})
                continue
            a, b = matched
            same = a['cell']['play_sha256'] == b['cell']['play_sha256'] and a['cell']['final'] == b['cell']['final'] and a['cell']['outcome'] == b['cell']['outcome']
            pairs.append({'class': cls, 'seed': seed, 'gameplay_identical': same})
            if not same:
                failures.append({'class': cls, 'seed': seed, 'error': 'shadow parameter changed gameplay'})
            class_rows.setdefault(cls, []).append({'seed': seed, 'control_profile': a['profile_applied'], 'probe_profile': b['profile_applied'], 'outcome': a['cell']['outcome'], 'gameplay_identical': same})
    (args.out / 'all-records.jsonl').write_text('\n'.join(replay_lines) + '\n')
    save(args.out / 'verification.json', {'zoo_commit': ZOO_PIN, 'source_commit': head,
         'planned_attempts': len(cells), 'retained_attempts': len(results), 'pairs': pairs,
         'per_class': class_rows, 'records': len(replay_lines), 'failures': failures,
         'interpretation': 'class selection and parser/record fidelity only; no tuned class policy or survival benefit established'})
    save(args.out / 'cells.json', results)
    if failures:
        raise SystemExit(1)
    print(f'PASS {len(results)} native zoo cells, {len(pairs)} pairs, {len(replay_lines)} records', flush=True)


if __name__ == '__main__':
    main()
