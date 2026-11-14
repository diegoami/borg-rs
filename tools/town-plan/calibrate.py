#!/usr/bin/env python3
"""Bounded shadow-calibration harness; portable zoo parameter dictionaries.

Print commands by default. --run executes the explicitly named arm(s),
seven fixed seed/class pairs, and never enables the live planner.
This is not an importer for borg-zoo's own profile schema.
"""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PROFILES = Path(__file__).with_name('zoo-profiles.json')


def main():
    profiles = json.loads(PROFILES.read_text())
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--arm', action='append', choices=list(profiles['arms']), required=True)
    parser.add_argument('--run', action='store_true', help='execute; otherwise print commands')
    parser.add_argument('--turns', type=int, default=30000)
    parser.add_argument('--out', type=Path, default=ROOT / 'harness/out/rust/town-utility')
    args = parser.parse_args()
    if not 1 <= args.turns <= 300000:
        parser.error('--turns must be 1..300000 for a bounded calibration')
    for arm in dict.fromkeys(args.arm):
        settings = profiles['base'] | profiles['arms'][arm]
        assert settings['borg_town_plan'] == 0
        flags = ','.join(f'{key}={"TRUE" if val is True else "FALSE" if val is False else val}'
                         for key, val in settings.items())
        for cls, seed in enumerate(range(501, 508)):
            out = (args.out / arm / f'{seed}-class{cls}').resolve()
            env = os.environ | {'BORG_IMPL': 'rust', 'BORG_TOWN_PLAN_RECORD': str(out / 'plan.jsonl')}
            cmd = [str(ROOT / 'harness/run.sh'), str(out), str(seed), '0', str(cls), str(args.turns), flags]
            print(f'BORG_IMPL=rust BORG_TOWN_PLAN_RECORD={shlex.quote(env["BORG_TOWN_PLAN_RECORD"])} '
                  + shlex.join(cmd), flush=True)
            if args.run:
                if out.exists():
                    raise SystemExit(f'refusing to overwrite existing evidence: {out}')
                out.parent.mkdir(parents=True, exist_ok=True)
                subprocess.run(cmd, cwd=ROOT, env=env, check=True)
                (out / 'town-parameters.json').write_text(json.dumps(settings, indent=2) + '\n')


if __name__ == '__main__':
    main()
