#!/usr/bin/env python3
"""Freeze and run the matched local full suite before choosing a zoo parameter grid.

45 frozen seed/race/class starts, 100000 turns, four arms. Every class uses
menu-letter birth; this corrects the legacy full suite's documented 7/8 quirk.
Run under flock /tmp/angband-validation.lock. Existing evidence is preserved.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import hashlib
import json
import os
from pathlib import Path
import subprocess

from class_profiles import CLASSES, ROOT, export_zoo, fingerprint, load_profiles


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--run', action='store_true')
    parser.add_argument('--jobs', type=int, default=4)
    args = parser.parse_args()
    args.out = args.out.resolve()
    if not 1 <= args.jobs <= 8: parser.error('--jobs must be 1..8')
    profiles = load_profiles()
    native = export_zoo(profiles, ['reference', 'cash-cautious'], execution='live')['profiles']
    base = profiles['base']
    arms = {'current': base | {'borg_town_plan': 0, 'borg_town_no_stores': False},
            'no-shops': base | {'borg_town_plan': 0, 'borg_town_no_stores': True},
            'utility-reference': native['rs-town-live-reference-normal']['settings'],
            'utility-cash50': native['rs-town-live-cash-cautious-normal']['settings']}
    source = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    identity = dict(line.split('=', 1) for line in (ROOT / 'build/rust/BUILD_ID').read_text().splitlines() if '=' in line)
    inputs = subprocess.check_output(['harness/identity.sh', 'source-hash', 'rust'], cwd=ROOT, text=True).strip()
    if inputs != identity['source_sha256']: parser.error('Rust build is stale')
    cases = []
    for line in (ROOT / 'harness/suites/full').read_text().splitlines():
        if not line.strip() or line.lstrip().startswith('#'): continue
        name, seed, race, cls, turns = line.split()
        cases.append({'name': name, 'seed': int(seed), 'race_idx': int(race), 'class_idx': int(cls),
                      'class': CLASSES[int(cls)], 'turn_cap': int(turns)})
    assert len(cases) == 45
    cells = [{'arm': arm, **case, 'settings': settings, 'settings_sha256': fingerprint(settings)} for arm, settings in arms.items() for case in cases]
    frozen = {'schema': 'borg-rs/town-live-full@1', 'source_commit': source, 'build': identity,
              'runtime_inputs_sha256': inputs, 'birth': 'full-suite seed/race/class rows with menu-letter selection',
              'cap': 100000, 'wall_seconds': 180, 'jobs': args.jobs, 'cells': cells,
              'endpoint': 'deaths, terminal character level, maximum dungeon depth, count reaching DL3+; failures visible per arm/class',
              'boundary': 'after-key; actual game turns retained; no cheat death',
              'tuning_rule': 'initial comparison only; choose a small parameter subset after inspecting these results, no improvement claim from 5 starts/class'}
    if not args.run:
        print(json.dumps(frozen, indent=2)); return
    if args.out.exists(): parser.error('use a fresh output root')
    args.out.mkdir(parents=True)
    save(args.out / 'experiment.json', frozen)
    save(args.out / 'live-profiles.json', native)
    # Preserve this executable, not merely a pointer to a future build.
    import shutil
    shutil.copy2(ROOT / 'build/rust/game/angband', args.out / 'angband-rust')
    shutil.copy2(ROOT / 'build/rust/BUILD_ID', args.out / 'BUILD_ID')
    def run(cell):
        out = args.out / cell['arm'] / cell['name']; out.parent.mkdir(parents=True, exist_ok=True)
        flags = ','.join(['menu'] + [f'{k}={"TRUE" if v is True else "FALSE" if v is False else v}' for k,v in cell['settings'].items()])
        env = os.environ | {'BORG_IMPL': 'rust', 'RUN_TIMEOUT_SECONDS': '180'}
        env.pop('BORG_TOWN_PLAN_RECORD', None)
        if cell['settings']['borg_town_plan'] == 1: env['BORG_TOWN_PLAN_RECORD'] = str(out / 'plan.jsonl')
        with (args.out / f'{cell["arm"]}-{cell["name"]}.log').open('w') as log:
            result = subprocess.run([str(ROOT/'harness/run.sh'), str(out), str(cell['seed']), str(cell['race_idx']), str(cell['class_idx']), str(cell['turn_cap']), flags], cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
        if out.exists(): save(out / 'frozen-cell.json', cell)
        return {'arm': cell['arm'], 'name': cell['name'], 'exit_code': result.returncode}
    attempts = []
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for future in as_completed([pool.submit(run, c) for c in cells]):
            attempt = future.result(); attempts.append(attempt)
            save(args.out/'attempts.json', attempts)
            print('done', attempt['arm'], attempt['name'], attempt['exit_code'], flush=True)
    save(args.out/'DONE.json', {'attempted': len(attempts), 'nonzero_exits': [a for a in attempts if a['exit_code']]})


if __name__ == '__main__': main()
