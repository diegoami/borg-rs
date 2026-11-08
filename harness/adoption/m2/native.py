#!/usr/bin/env python3
"""Run the 49 original native scenarios against local C oracle core objects.

Original fixtures/configs are hashed and immutable; only two historical fixed
profile path literals are relocated. Others use their original env parameter.
Unit-test exit0 alone never implies an assertion passed. Rust game validation
is separate: these fixtures directly inspect native C private Borg globals.
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, default=ROOT / 'build/c')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--expect-fixed', action='store_true')
    args = parser.parse_args()
    output = args.output.resolve();output.mkdir(parents=True, exist_ok=False)
    build = args.build.resolve()
    core = sorted((build / 'CMakeFiles/OurCoreLib.dir').rglob('*.o'))
    helpers = sorted((build / 'CMakeFiles/OurUnitTestLib.dir').rglob('*.o'))
    assert len(core) > 150 and helpers, 'Build C oracle and OurUnitTestLib first'
    manifest = json.loads((HERE / 'fixtures/manifest.json').read_text())
    report = {'consumer_pin': manifest['consumer_pin'], 'fixtures': {}, 'objects_sha256': {str(p): sha(p) for p in core + helpers},
              'source_sha256': subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'source-hash', 'c'], text=True).strip()}
    for fixture in manifest['fixtures']:
        name = fixture['name'];source = HERE / 'fixtures' / name / 'scenario.c'
        config = source.with_name('borg.txt')
        assert sha(source) == fixture['source_sha256'] and sha(config) == fixture['config_sha256']
        directory = output / name;directory.mkdir()
        profile = directory / 'profile'
        for folder in ('', 'save', 'panic', 'scores', 'archive'):
            (profile / folder).mkdir(exist_ok=True)
        (profile / 'borg.txt').write_bytes(config.read_bytes())
        text = source.read_text()
        relocations = {'swap': '/tmp/native-c-66-expanded', 'knockout': '/tmp/native-c-14-evidence'}
        if name in relocations:
            # Those fixtures append /profile themselves.
            text = text.replace(relocations[name], str(directory))
        adapted = directory / 'scenario.c';adapted.write_text(text)
        binary = directory / 'scenario'
        # Match the Release engine's flag API (debug flag helpers are absent).
        # Native unit-test require/eq assertions are independent of NDEBUG.
        assert 'assert(' not in text
        command = ['cc', '-g', '-DNDEBUG', '-DALLOW_BORG', '-I' + str(ROOT / 'angband/src'), '-I' + str(ROOT / 'angband/src/tests'),
                   str(adapted), *map(str, core + helpers), '-lm', '-o', str(binary)]
        with (directory / 'link.log').open('w') as log:
            subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
        rows = {}
        for mode in fixture['modes']:
            env = {k:v for k,v in os.environ.items() if not k.startswith(('BORG_', 'ANGBAND_', 'CBUG'))}
            env.update({fixture['environment']: mode, fixture['environment'].replace('_MODE', '_PROFILE'): str(profile), 'LC_ALL': 'C.UTF-8'})
            result = subprocess.run([str(binary), '-v'], cwd=build / 'game', env=env,
                                    capture_output=True, timeout=60)
            text = re.sub(rb'\x1b\[[0-9;]*m', b'', result.stdout + result.stderr)
            path = directory / (mode + '.log');path.write_bytes(text)
            rows[mode] = {'returncode': result.returncode, 'completed': b'finished:' in text,
                          'passed': b'finished: 1/1 passed' in text, 'log_sha256': sha(path)}
            print(name, mode, rows[mode], flush=True)
            assert result.returncode == 0 and rows[mode]['completed'], 'Native fixture did not complete'
            if args.expect_fixed:
                assert rows[mode]['passed'], (name, mode)
        report['fixtures'][name] = {'cases': rows, 'binary_sha256': sha(binary), 'adapted_source_sha256': sha(adapted),
                                    'original_source_sha256': fixture['source_sha256']}
        (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    report['cases'] = sum(len(row['cases']) for row in report['fixtures'].values())
    report['passed'] = sum(case['passed'] for row in report['fixtures'].values() for case in row['cases'].values())
    (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
