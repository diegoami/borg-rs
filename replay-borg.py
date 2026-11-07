#!/usr/bin/env python3
"""Verify a frozen Rust headless recording and inspect its actual terminal frames."""
import argparse
from pathlib import Path
import sys
import subprocess

sys.path.insert(0, str(Path(__file__).resolve().parent / 'tools'))
import borg_runtime
import borg_replay_viewer

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('recording', type=Path)
parser.add_argument('--turn', type=int)
parser.add_argument('--dump', action='store_true', help='print one frame without an interactive terminal')
parser.add_argument('--verify-only', action='store_true')
parser.add_argument('--recording-only', action='store_true', help='view recorded frames with an UNVERIFIED badge')
args = parser.parse_args()
if args.verify_only and args.recording_only:
    parser.error('--verify-only cannot be combined with --recording-only')
try:
    directory = args.recording.resolve()
    borg_runtime.validate(directory, require_complete=not args.recording_only)
    verified = not args.recording_only
    if verified:
        proof = borg_runtime.verify(directory)
        print('VERIFIED frozen-origin rerun: keys, messages, player state, frames, final and scheduled engine saves', flush=True)
    if not args.verify_only:
        frames = directory if (directory / 'frames.jsonl.gz').exists() else directory / 'run'
        borg_replay_viewer.show(frames, turn=args.turn, verified=verified, dump=args.dump)
except (ValueError, OSError, AssertionError, subprocess.SubprocessError) as error:
    parser.exit(1, str(error) + '\n')
