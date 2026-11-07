#!/usr/bin/env python3
"""Verify a frozen Rust GUI/headless recording, then inspect its original frames."""
import argparse
import json
from pathlib import Path
import sys
import subprocess

sys.path.insert(0, str(Path(__file__).resolve().parent / 'tools'))
import borg_runtime
import borg_gui
import borg_replay_viewer


def resolve_recording(path):
    path = path.resolve()
    if path.name == 'record.json' and path.is_file():
        path = path.parent
    if (path / 'recording.json').exists():
        return path
    if (path / 'record.json').exists():
        record = json.loads((path / 'record.json').read_text())
        reference = record.get('recording')
        if not reference:
            raise ValueError('This historical death has no complete recording; retain its original logs and assessments')
        bundle = Path(reference['raw_directory']).resolve()
        if not (bundle / 'recording.json').exists():
            raise ValueError('Restore the full original recording bundle: ' + str(bundle))
        if borg_runtime.sha(bundle / 'recording.json') != reference['manifest_sha256']:
            raise ValueError('Death dossier recording manifest differs')
        return bundle
    return path


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('recording', type=Path, help='recording bundle or death dossier directory')
    parser.add_argument('--from-turn', '--turn', dest='turn', type=int, help='first engine turn to inspect; default: last 105 turns')
    parser.add_argument('--dump-turn', type=int, help='print the recorded frame at/before this turn')
    parser.add_argument('--dump', action='store_true', help='print --from-turn/--turn without an interactive terminal')
    parser.add_argument('--verify-only', action='store_true')
    parser.add_argument('--view-only', '--recording-only', dest='recording_only', action='store_true', help='view with an UNVERIFIED badge')
    parser.add_argument('--output', type=Path, help='new directory retaining verification run/evidence')
    parser.add_argument('--timeout', type=int, help='positive wall limit for the verification run')
    args = parser.parse_args(argv)
    if args.verify_only and args.recording_only:
        parser.error('--verify-only cannot be combined with --view-only/--recording-only')
    if (args.timeout is not None and args.timeout < 1) or any(t is not None and t < 0 for t in (args.turn, args.dump_turn)):
        parser.error('Use nonnegative engine turns and a positive timeout')
    if args.recording_only and args.output:
        parser.error('--output retains verification evidence; omit it with --view-only')
    existed = args.output.exists() if args.output else False
    try:
        directory = resolve_recording(args.recording)
        schema = json.loads((directory / 'recording.json').read_text()).get('schema')
        backend = borg_gui if schema == borg_gui.SCHEMA else borg_runtime
        backend.validate(directory, require_complete=not args.recording_only)
        verified = not args.recording_only
        if verified:
            proof = backend.verify(directory, output=args.output, timeout=args.timeout)
            print('VERIFIED frozen-origin rerun: ' + proof['model'], flush=True)
        if not args.verify_only:
            frames = directory if (directory / 'frames.jsonl.gz').exists() else directory / 'run'
            borg_replay_viewer.show(frames, turn=args.dump_turn if args.dump_turn is not None else args.turn,
                                   verified=verified, dump=args.dump or args.dump_turn is not None)
        return 0
    except (ValueError, OSError, KeyError, AssertionError, subprocess.SubprocessError) as error:
        if args.output and not existed and args.output.exists():
            borg_runtime.write(args.output / 'verification.json', {'verified': False, 'error': str(error)})
        print(str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
