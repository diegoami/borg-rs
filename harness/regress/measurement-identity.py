#!/usr/bin/env python3
"""Build-input hashes are stable in parallel and fail closed on read errors."""
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(tempfile.mkdtemp(prefix='identity-', dir=ROOT / 'harness/out'))
expected = {}
for impl in ['c', 'rust']:
    paths = ['angband', 'harness/measurement']
    if impl == 'rust':
        paths += ['borg-rs', 'Cargo.toml', 'Cargo.lock']
    entries = subprocess.check_output(['git', 'ls-tree', '-rz', 'HEAD', '--', *paths], cwd=ROOT)
    pairs = []
    for entry in entries.split(b'\0'):
        if entry:
            meta, path = entry.split(b'\t')
            pairs.append((path, meta.split()[-1]))
    data = ('borg-rs build inputs v1 impl=' + impl + '\n').encode()
    data += b''.join(sha + b' ' + path + b'\n' for path, sha in sorted(pairs))
    expected[impl] = hashlib.sha256(data).hexdigest()


def check(impl):
    result = subprocess.run([str(ROOT / 'harness/identity.sh'), 'source-hash', impl],
                            capture_output=True, text=True, check=True)
    assert not result.stderr, result.stderr
    assert result.stdout.strip() == expected[impl], result.stdout
    return impl


with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    assert len(list(pool.map(check, ['c', 'rust'] * 20))) == 40
real_git = shutil.which('git')
shim = OUT / 'shim'
shim.mkdir()
script = shim / 'git'
script.write_text('#!/bin/sh\nif [ "$1" = hash-object ]; then\n'
                  '  echo "forced blob-read failure" >&2\n  exit 23\nfi\n'
                  'exec "$BORGH_REAL_GIT" "$@"\n')
script.chmod(0o755)
failed = subprocess.run([str(ROOT / 'harness/identity.sh'), 'source-hash', 'c'],
                        env={**os.environ, 'PATH': str(shim) + os.pathsep + os.environ['PATH'],
                             'BORGH_REAL_GIT': real_git}, capture_output=True, text=True)
assert failed.returncode != 0 and not failed.stdout and 'forced blob-read failure' in failed.stderr
(OUT / 'result.json').write_text(json.dumps({'expected': expected, 'parallel_checks': 40,
                                           'read_error_rejected': True,
                                           'failed_read_returncode': failed.returncode}, indent=2) + '\n')
print('PASS: 40 parallel hashes match committed inputs; read failure returns no hash:', OUT)
