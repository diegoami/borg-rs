#!/usr/bin/env python3
"""Run the C milestone3 native fixtures against a local C oracle build.

Each fixture is linked unchanged against the core and unit-test helper
objects of one C build directory, and every mode is run with its original
<PREFIX>_MODE/<PREFIX>_PROFILE environment. A mode passes only when its
own "finished: 1/1 passed" marker is printed (or a per-mode marker from
'mode_markers'); exit status alone is never taken as a pass. Fixtures with a sanitizer column are also linked/run in
an AddressSanitizer build (--asan-build): 'asan' fixtures run with
detect_leaks=0 and record any AddressSanitizer report, 'lsan' fixtures run
their first mode with detect_leaks=1 and record the leak summary.

These fixtures inspect native C private Borg state, so they run on the C
oracle that is excluded from the Rust production image. Rust behaviour is
checked separately by the C/Rust game parity suites.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def objects(build):
    core = sorted((build / 'CMakeFiles/OurCoreLib.dir').rglob('*.o'))
    helpers = sorted((build / 'CMakeFiles/OurUnitTestLib.dir').rglob('*.o'))
    assert len(core) > 150 and helpers, f'build OurCoreLib and OurUnitTestLib in {build}'
    return core, helpers


def run_phase(fixture, directory, build, phase, asan):
    core, helpers = objects(build)
    binary = directory / f'{phase}-scenario'
    command = ['cc', '-g', '-DNDEBUG', '-DALLOW_BORG', '-I' + str(ROOT / 'angband/src'), '-I' + str(ROOT / 'angband/src/tests')]
    if asan:
        command += ['-fsanitize=address', '-fno-omit-frame-pointer']
    command += [str(directory / 'scenario.c'), *map(str, core + helpers), '-lm', '-o', str(binary)]
    with (directory / f'{phase}-link.log').open('w') as log:
        linked = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    rows = {'link_exit': linked.returncode, 'cases': {}}
    if linked.returncode:
        return rows
    modes = fixture['modes']
    if asan and fixture['sanitizer'] == 'lsan':
        modes = modes[:1]
    for mode in modes:
        env = dict(os.environ, LC_ALL='C.UTF-8')
        env[fixture['env_prefix'] + '_MODE'] = mode
        env[fixture['env_prefix'] + '_PROFILE'] = str(directory / 'profile')
        if asan:
            leaks = fixture['sanitizer'] == 'lsan'
            env['ASAN_OPTIONS'] = fixture.get('asan_options') or (
                'detect_leaks=1' if leaks else 'detect_leaks=0:intercept_strcmp=0')
        result = subprocess.run([str(binary), '-v'], cwd=build / 'game', env=env,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=300)
        text = re.sub(r'\x1b\[[0-9;]*m', '', result.stdout.decode(errors='replace'))
        log = directory / f'{phase}-{mode}.log'
        log.write_text(text)
        marker = fixture.get('mode_markers', {}).get(mode, fixture['marker'])
        row = {'exit': result.returncode, 'passed': marker in text,
               'asan_report': 'ERROR: AddressSanitizer' in text, 'log_sha256': sha(log)}
        summary = re.search(r'SUMMARY: AddressSanitizer: (.*)', text)
        if summary:
            row['sanitizer_summary'] = summary[1].strip()
        outcome = re.search(r'OUTCOME (.*)', text)
        if outcome:
            row['outcome'] = outcome[1].strip()
        rows['cases'][mode] = row
        print(fixture['name'], phase, mode, {k: v for k, v in row.items() if k != 'log_sha256'}, flush=True)
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, required=True, help='C build directory with core/unit-test objects')
    parser.add_argument('--asan-build', type=Path, help='matching AddressSanitizer C build directory')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--only', action='append', help='fixture name (repeatable)')
    parser.add_argument('--fixtures', type=Path, default=HERE / 'fixtures',
                        help='fixture directory with manifest.json (milestone4: harness/adoption/m4/fixtures)')
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    fixtures_dir = args.fixtures.resolve()
    manifest = json.loads((fixtures_dir / 'manifest.json').read_text())
    build = args.build.resolve()
    report = {'consumer_pin': manifest['consumer_pin'], 'build': str(build),
              'build_flags_sha256': sha(build / 'CMakeFiles/OurCoreLib.dir/flags.make'),
              'objects_sha256': {str(p): sha(p) for p in sum(objects(build), [])},
              'fixtures': {}}
    if args.asan_build:
        asan = args.asan_build.resolve()
        assert '-fsanitize=address' in (asan / 'CMakeFiles/OurCoreLib.dir/flags.make').read_text()
        report['asan_build'] = str(asan)
    for fixture in manifest['fixtures']:
        name = fixture['name']
        if args.only and name not in args.only:
            continue
        source = fixtures_dir / name / 'scenario.c'
        assert sha(source) == fixture['scenario_sha256'], name
        directory = output / name
        (directory / 'profile').mkdir(parents=True)
        for folder in ('save', 'panic', 'scores', 'archive'):
            (directory / 'profile' / folder).mkdir()
        config = source.with_name('borg.txt')
        if config.exists():
            assert sha(config) == fixture['config_sha256'], name
            text = config.read_bytes()
            for old, new in fixture.get('config_overrides', []):
                assert text.count(old.encode()) == 1, (name, old)
                text = text.replace(old.encode(), new.encode())
            (directory / 'profile/borg.txt').write_bytes(text)
        (directory / 'scenario.c').write_bytes(source.read_bytes())
        # Headers a scenario includes by quoted name (milestone7: the text-287
        # API-shape header); they sit next to the copied scenario.c.
        for extra, digest in fixture.get('extra_files', {}).items():
            data = (fixtures_dir / name / extra).read_bytes()
            assert hashlib.sha256(data).hexdigest() == digest, (name, extra)
            (directory / extra).write_bytes(data)
        entry = {'normal': run_phase(fixture, directory, build, 'normal', False)}
        if args.asan_build and fixture['sanitizer']:
            entry['asan'] = run_phase(fixture, directory, asan, 'asan', True)
        report['fixtures'][name] = entry
        (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    total = sum(len(e['normal']['cases']) for e in report['fixtures'].values())
    passed = sum(c['passed'] for e in report['fixtures'].values() for c in e['normal']['cases'].values())
    print(f'normal: {passed}/{total} modes passed')


if __name__ == '__main__':
    main()
