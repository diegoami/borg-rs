#!/usr/bin/env python3
"""Verify the portable source receipt against immutable git blobs."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--repo', type=Path, default=HERE.parents[2])
args = parser.parse_args()
metadata = json.loads((HERE / 'historical-pin.json').read_text())
count = 0
for row in (HERE / metadata['ledger']).read_text().splitlines():
    expected, path = row.split('  ', 1)
    blob = subprocess.check_output(['git', '-C', str(args.repo), 'show', metadata['source_commit'] + ':' + path])
    if hashlib.sha256(blob).hexdigest() != expected:
        raise SystemExit('Source blob differs: ' + path)
    count += 1
print(f'{count} source blobs match {metadata["source_commit"]}; current worktree bytes are irrelevant')
