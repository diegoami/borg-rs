#!/usr/bin/env python3
"""Run the source404 flow-cache fixture (patch 0049) before/after on C oracles.

Links the unchanged fixture against one C build's core and unit-test objects
with the evidence's own --wrap flags, then runs the evidence's run-cases.py
(30 cases). The expected failures are the evidence's own EXPECTED_FAIL for the
"before" and "after" variants (reproduce.py, imported only for that table).
The consumer's partial-fix variants (neg-*, alt-wipe) stay its own evidence.

Usage: flowcache.py --build BEFORE_OR_AFTER_C_BUILD --variant before|after --output DIR
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent / 'fixtures/flowcache'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--build', type=Path, required=True)
    p.add_argument('--variant', choices=('before', 'after'), required=True)
    p.add_argument('--output', type=Path, required=True)
    args = p.parse_args()
    manifest = json.loads((HERE.parent / 'manifest.json').read_text())['flowcache']
    for name, digest in manifest['files'].items():
        assert sha(HERE / name) == digest, name
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    (out / 'profile').mkdir()
    (out / 'profile/borg.txt').write_bytes((HERE / 'borg.txt').read_bytes())
    spec = importlib.util.spec_from_file_location('flowcache_contract', HERE / 'reproduce.py')
    source = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(source)
    build = args.build.resolve()
    objects = sorted((build / 'CMakeFiles/OurCoreLib.dir').rglob('*.o'))
    helpers = sorted((build / 'CMakeFiles/OurUnitTestLib.dir').rglob('*.o'))
    assert len(objects) >= 150 and helpers
    game = out / args.variant / 'game'
    game.mkdir(parents=True)
    (game / 'lib').symlink_to(build / 'game/lib', target_is_directory=True)
    executable = game / 'FC404Observer'
    command = ['cc', '-g', '-DNDEBUG', '-DALLOW_BORG', '-I' + str(ROOT / 'angband/src'),
               '-I' + str(ROOT / 'angband/src/tests'), str(HERE / 'scenario.c'),
               *map(str, objects + helpers), '-lm', '-Wl,--wrap=borg_danger', '-Wl,--wrap=borg_note',
               '-o', str(executable)]
    with (out / 'link.log').open('w') as log:
        subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
    subprocess.run([sys.executable, str(HERE / 'run-cases.py'), args.variant, str(executable), str(out)],
                   cwd=game, check=True)
    rows = json.loads((out / (args.variant + '-results.json')).read_text())
    failed = sorted(c for c, r in rows.items() if not r['passed'])
    expected = sorted(source.EXPECTED_FAIL[args.variant])
    report = {'variant': args.variant, 'build': str(build), 'cases': len(rows),
              'failed': failed, 'expected_failed': expected,
              'all_exit_zero': all(r['exit'] == 0 for r in rows.values()),
              'matches_contract': len(rows) == 30 and failed == expected,
              'binary_sha256': sha(executable)}
    (out / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f"flowcache {args.variant}: {len(rows) - len(failed)}/{len(rows)} pass; "
          f"failed matches contract: {report['matches_contract']}; all exit 0: {report['all_exit_zero']}")
    sys.exit(0 if report['matches_contract'] and report['all_exit_zero'] else 1)


if __name__ == '__main__':
    main()
