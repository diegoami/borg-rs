#!/usr/bin/env python3
"""Verify an original Rust prefix, prepare an isolated human save, or resume it."""
import argparse
from pathlib import Path
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import human_takeover as human
import display_font
import importlib.util
spec = importlib.util.spec_from_file_location('replay_cli', ROOT / 'scripts/replay-borg.py')
replay = importlib.util.module_from_spec(spec); spec.loader.exec_module(replay)


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('recording', nargs='?', type=Path)
    p.add_argument('--from-turn', default='latest')
    p.add_argument('--resume', type=Path)
    p.add_argument('--prepare-only', action='store_true')
    p.add_argument('--output', type=Path)
    p.add_argument('--timeout', type=int, default=600)
    p.add_argument('--yes', action='store_true')
    p.add_argument('--no-build', action='store_true')
    a = p.parse_args(argv)
    if a.yes and a.no_build:
        p.error('Choose --yes or --no-build')
    try:
        if a.timeout < 1 or a.recording is None and a.resume is None or a.recording is not None and a.resume is not None:
            raise ValueError('Choose a recording or --resume, with a positive timeout')
        if a.resume:
            return 0 if a.prepare_only else (human.launch(a.resume) and 0)
        if not a.prepare_only:
            display_font.probe()
        turn = a.from_turn if a.from_turn == 'latest' else int(a.from_turn)
        bundle = replay.resolve_recording(a.recording)
        # Offer before taking a shared lock. Exact existing helpers are
        # checked by the preparer; --no-build never silently compiles one.
        human.BUILD_POLICY = 'no-build' if a.no_build else 'yes' if a.yes else 'ask'
        branch = human.prepare(bundle, turn, output=a.output, timeout=a.timeout)
        if not a.prepare_only:
            human.launch(branch)
        return 0
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        print(str(error), file=sys.stderr); return 1


if __name__ == '__main__':
    raise SystemExit(main())
