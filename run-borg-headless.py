#!/usr/bin/env python3
"""Run a named Rust Borg headlessly with backups and a replayable recording."""
import argparse
from pathlib import Path
import sys
import subprocess

sys.path.insert(0, str(Path(__file__).resolve().parent / 'tools'))
import borg_runtime

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('character', nargs='?', default='RustBorg')
parser.add_argument('--new', action='store_true', help='start a fresh birth; preserve the previous save in backups')
parser.add_argument('--from-backup', nargs='?', type=int, const=1, choices=range(1, 6))
parser.add_argument('--seed', type=int, default=42)
parser.add_argument('--race', type=int, choices=range(11))
parser.add_argument('--class', dest='cls', type=int, choices=range(9))
parser.add_argument('--turns', type=int, default=100000, help='additional world turns (not player actions)')
parser.add_argument('--keys', type=int, default=50000, help='independent Borg event budget, including zero-clock stair cycling')
parser.add_argument('--wall-seconds', type=int, default=300)
parser.add_argument('--checkpoint-every', type=int, default=1000)
args = parser.parse_args()
try:
    if args.new and args.from_backup is not None:
        raise ValueError('Choose --new or --from-backup')
    if min(args.turns, args.keys, args.wall_seconds, args.checkpoint_every) <= 0:
        raise ValueError('Budgets and checkpoint interval must be positive')
    if args.keys > 999999999 or not 0 <= args.seed <= 4294967295:
        raise ValueError('Key budget or seed is outside the supported range')
    borg_runtime.record_run(args)
except (ValueError, OSError, AssertionError, subprocess.SubprocessError) as error:
    parser.exit(1, str(error) + '\n')
