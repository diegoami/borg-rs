#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone10 carries (I-001, I-002, 0099).

Reads immutable Git objects only: the consumer's committed fixture kits at
the milestone10 tag, plus the I-002 scenario source at
tools/fleeing-heal-scenario.c (the consumer's kit keeps the scenario in
tools/, and commits only its baseline logs and profile under
docs/audit/c-reports/fleeheal-408/). Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m10/fixtures.

Usage: import.py CONSUMER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '6944d88e87d02e9d24e85438d25f0c6ef864c170'
PREVIOUS_PIN = '33207fbeeb582e67d6b18e9f40613617667905ae'
REPOSITORIES = {'consumer': 'diegoami/better-borg-c'}

# name, unit, consumer issue, env prefix, suite id, modes, sanitizer,
# scenario path in the consumer tree, config path, extra source paths
FIXTURES = [
    # The 0099 crash correction. The consumer's kit holds the scenario, the
    # profile and the baseline logs; the fix is attached to the existing
    # ledger entry 0099, so this is not a new ordinal.
    ('necrolight-crash', '0099', 377, 'NL296', 'necrolight-296-crash',
     ['stale-far', 'stale-town', 'lit', 'dark', 'mage-lit'], 'asan',
     'docs/audit/c-reports/necrolight-296-crash/scenario.c',
     'docs/audit/c-reports/necrolight-296-crash/profile/borg.txt',
     []),
    # I-002: borg_caution() heals while fleeing at survivable danger. The
    # scenario source lives in the consumer's tools/ tree; the kit holds the
    # profile and the baseline logs.
    ('fleeheal', 'I-002', 408, 'IM2', 'fleeheal-408',
     ['flee-low', 'flee-highdanger', 'noflee-wounded', 'flee-full'], None,
     'tools/fleeing-heal-scenario.c',
     'docs/audit/c-reports/fleeheal-408/profile/borg.txt',
     []),
]


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    clone = Path(sys.argv[1])
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, unit, issue, prefix, suite, modes, sanitizer, scenario_path,
         config_path, extras) in FIXTURES:
        full = git(clone, 'rev-parse', CONSUMER_PIN + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        scenario = git(clone, 'show', f'{full}:{scenario_path}')
        config = git(clone, 'show', f'{full}:{config_path}')
        (directory / 'scenario.c').write_bytes(scenario)
        (directory / 'borg.txt').write_bytes(config)
        entry = {'name': name, 'unit': unit, 'consumer_issue': issue,
                 'evidence_repository': REPOSITORIES['consumer'],
                 'evidence_commit': full,
                 'evidence_path': scenario_path,
                 'config_path': config_path,
                 'original_sha256': hashlib.sha256(scenario).hexdigest(),
                 'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                 'config_sha256': hashlib.sha256(config).hexdigest(),
                 'env_prefix': prefix,
                 'marker': f'native-c/{suite} finished: 1/1 passed',
                 'modes': modes, 'sanitizer': sanitizer}
        entries.append(entry)
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3',
                'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(f'imported {len(entries)} fixtures into {out}')


if __name__ == '__main__':
    main()
