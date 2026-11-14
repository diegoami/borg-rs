#!/usr/bin/env python3
"""Check an unpacked oracle corpus against the receipt's digests.

Run from the repository root (the directory that holds
docs/validation/2026-11-13-c-v12r/oracle-digests.json):

    python3 check_oracle_digests.py

Exits 0 when every file of the digest list is present with the recorded
sha256 and size, 1 otherwise.
"""
import hashlib
import json
import pathlib
import sys

DIGESTS = 'docs/validation/2026-11-14-munchkin-off/oracle-digests.json'


def main():
    digests = json.loads(pathlib.Path(DIGESTS).read_text())
    problems = []
    for rel, meta in sorted(digests.items()):
        path = pathlib.Path(rel)
        if not path.is_file():
            problems.append((rel, 'missing'))
            continue
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != meta['sha256']:
            problems.append((rel, 'sha256 mismatch'))
        elif path.stat().st_size != meta['bytes']:
            problems.append((rel, 'size mismatch'))
    print(f'checked {len(digests)} files against {DIGESTS}')
    if problems:
        print(f'{len(problems)} problem(s):')
        for rel, why in problems[:20]:
            print('  ', rel, '-', why)
        return 1
    print('every file matches the receipt')
    return 0


if __name__ == '__main__':
    sys.exit(main())
