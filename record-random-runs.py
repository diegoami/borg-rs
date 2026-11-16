#!/usr/bin/env python3
"""Record a bounded sequence of fresh random-birth headless Borg games."""
import argparse
import os
from pathlib import Path
import secrets
import signal
import subprocess
import sys
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parent / 'tools'))
import borg_runtime as runtime
import cemetery_publish


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runs', type=int, default=1, help='number of new games (default: 1)')
    parser.add_argument('--turns', type=int, default=20000, help='world-turn budget per game')
    parser.add_argument('--keys', type=int, default=50000, help='independent key-event budget per game')
    parser.add_argument('--wall-seconds', type=int, default=300, help='wall-time limit per game')
    parser.add_argument('--checkpoint-every', type=int, default=1000)
    parser.add_argument('--seed', type=int, help='first birth seed; subsequent games increment it modulo 2^32')
    parser.add_argument('--auto-push', '--auto-commit-and-push', dest='auto_push', action='store_true', help='commit and push each completed session export to the cemetery')
    args = parser.parse_args(argv)
    try:
        if not 1 <= args.runs <= 10000 or min(args.turns, args.keys, args.wall_seconds, args.checkpoint_every) < 1:
            raise ValueError('Use 1..10000 runs and positive per-game budgets')
        if args.turns > 2147483647 or args.keys > 999999999:
            raise ValueError('Turn or key budget exceeds the engine limits')
        if args.seed is not None and not 0 <= args.seed <= 0xffffffff:
            raise ValueError('Seed must be uint32')
        seed = args.seed if args.seed is not None else secrets.randbits(32)
        cemetery = runtime.data_root()
        if args.auto_push:
            cemetery_publish.preflight(cemetery)
        batch = secrets.token_hex(4)
        for number in range(args.runs):
            run_seed = (seed + number) & 0xffffffff
            name = f'Random-{batch}-{number + 1:04d}-{run_seed}'
            print(f'Recording {number + 1}/{args.runs}: seed={run_seed}, random race/class, character={name}', flush=True)
            options = SimpleNamespace(character=name, new=True, from_backup=None, seed=run_seed,
                race=None, cls=None, turns=args.turns, keys=args.keys, wall_seconds=args.wall_seconds,
                checkpoint_every=args.checkpoint_every, stall_seconds=0, death_issue=os.environ.get('BORG_DEATH_ISSUE'))
            bundle = runtime.record_run(options)
            if args.auto_push:
                cemetery_publish.publish(bundle, cemetery)
        return 0
    except (OSError, ValueError, AssertionError, subprocess.SubprocessError) as error:
        parser.exit(1, str(error) + '\nBatch stopped; existing recording evidence is retained.\n')


if __name__ == '__main__':
    def interrupted(signum, frame):
        raise InterruptedError('Recording batch interrupted; existing evidence retained')
    signal.signal(signal.SIGTERM, interrupted)
    raise SystemExit(main())
