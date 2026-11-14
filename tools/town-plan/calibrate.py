#!/usr/bin/env python3
"""Bounded class-profile calibration; print commands unless --run is supplied."""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess
from class_profiles import CLASSES, PROFILES, ROOT, export_zoo, fingerprint, load_profiles, resolve, selected_profile


def plan(profiles, arms, classes, seeds, turns, out):
    cells = []
    # Same seed set per class/arm for paired comparisons. Without --seeds,
    # preserve the historical 501+class assignment, now covering all 9 classes.
    for arm in dict.fromkeys(arms):
        for cls in classes:
            index = CLASSES.index(cls)
            settings, sources = resolve(profiles, arm, cls)
            profile = selected_profile(profiles, arm, cls)
            for seed in seeds if seeds is not None else [501 + index]:
                target = (out / arm / f'{seed}-class{index}').resolve()
                flags = ','.join(['menu'] + [f'{key}={"TRUE" if val is True else "FALSE" if val is False else val}'
                                            for key, val in settings.items()])
                command = [str(ROOT / 'harness/run.sh'), str(target), str(seed), '0', str(index), str(turns), flags]
                cells.append({'arm': arm, 'class': cls, 'class_idx': index, 'seed': seed,
                              'out': str(target), 'profile_applied': profile,
                              'settings': settings, 'setting_sources': sources,
                              'settings_sha256': fingerprint(settings), 'command': command})
    return {'schema': 'borg-rs/town-class-plan@1', 'turn_cap': turns,
            'purpose': 'shadow configuration/sensitivity; no survival ranking',
            'profiles_sha256': fingerprint(profiles), 'cells': cells}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profiles', type=Path, default=PROFILES)
    parser.add_argument('--arm', action='append', required=True)
    parser.add_argument('--classes', nargs='+', choices=CLASSES, default=list(CLASSES))
    parser.add_argument('--seeds', type=int, nargs='+')
    parser.add_argument('--run', action='store_true')
    parser.add_argument('--turns', type=int, default=30000)
    parser.add_argument('--wall-seconds', type=int, default=120)
    parser.add_argument('--out', type=Path, default=ROOT / 'harness/out/rust/town-class-calibration')
    parser.add_argument('--export-zoo', type=Path)
    args = parser.parse_args()
    if not 1 <= args.turns <= 300000:
        parser.error('--turns must be 1..300000')
    if not 1 <= args.wall_seconds <= 3600:
        parser.error('--wall-seconds must be 1..3600')
    if args.seeds and (len(set(args.seeds)) != len(args.seeds) or any(not 1 <= s <= 2147483647 for s in args.seeds)):
        parser.error('--seeds must be unique positive 31-bit integers')
    if len(set(args.classes)) != len(args.classes):
        parser.error('--classes must be unique')
    try:
        profiles = load_profiles(args.profiles)
        frozen = plan(profiles, args.arm, args.classes, args.seeds, args.turns, args.out)
        frozen['wall_seconds'] = args.wall_seconds
    except (ValueError, KeyError) as exc:
        parser.error(str(exc))
    if args.export_zoo:
        if args.export_zoo.exists():
            parser.error('refusing to overwrite an existing zoo export')
        args.export_zoo.write_text(json.dumps(export_zoo(profiles, args.arm), indent=2) + '\n')
    for cell in frozen['cells']:
        record = str(Path(cell['out']) / 'plan.jsonl')
        print(f'BORG_IMPL=rust BORG_TOWN_PLAN_RECORD={shlex.quote(record)} ' + shlex.join(cell['command']), flush=True)
    if not args.run:
        return
    if args.out.exists():
        parser.error(f'refusing to overwrite existing evidence: {args.out}')
    args.out.mkdir(parents=True)
    (args.out / 'experiment.json').write_text(json.dumps(frozen, indent=2) + '\n')
    (args.out / 'profiles.json').write_text(json.dumps(profiles, indent=2) + '\n')
    failed = []
    for cell in frozen['cells']:
        out = Path(cell['out'])
        out.parent.mkdir(parents=True, exist_ok=True)
        env = os.environ | {'BORG_IMPL': 'rust', 'BORG_TOWN_PLAN_RECORD': str(out / 'plan.jsonl'),
                            'RUN_TIMEOUT_SECONDS': str(args.wall_seconds)}
        result = subprocess.run(cell['command'], cwd=ROOT, env=env, check=False)
        if out.exists():
            (out / 'town-parameters.json').write_text(json.dumps(cell['settings'], indent=2) + '\n')
            (out / 'profile-selection.json').write_text(json.dumps(cell, indent=2) + '\n')
        if result.returncode:
            failed.append({'arm': cell['arm'], 'class': cell['class'], 'seed': cell['seed'], 'exit_code': result.returncode})
    (args.out / 'failures.json').write_text(json.dumps(failed, indent=2) + '\n')
    if failed:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
