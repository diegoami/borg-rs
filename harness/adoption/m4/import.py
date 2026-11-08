#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone4 carries (0035-0042).

Reads immutable Git objects only: scenario.c/borg.txt at each fixture's
recorded neo-angband-tracker evidence commit. Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m4/fixtures.

Usage: import.py TRACKER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '70993393a1c946ce8cd38ee558bf1ce6464350e7'
PREVIOUS_PIN = '702f091f6ac06cae1014df2f85ac218be0189813'

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# marker id, modes, sanitizer, asan options (None = runner default)
FIXTURES = [
    ('brand-need', '0035', 240, 133, 'd9a848a8e89e0a0e7843c44e699631fb0ee392fc', 'docs/audit/c-reports/brand-240/', 'N240', 'brand-240',
     ['brand', 'control'], None, None),
    ('stairtrack', '0036', 232, 137, '07a71c16084398643ac09da9cc56e3b2b5b6ea6b', 'docs/audit/c-reports/stairtrack-232/', 'T232', 'stairtrack-232',
     ['known', 'fresh'], None, None),
    ('acidarc', '0037', 252, 141, 'e40c642a0a7906d8d9caadb803ed6a5adb70bbc5', 'docs/audit/c-reports/acidarc-252/', 'AC252', 'acidarc-252',
     ['edge', 'near', 'empty'], None, None),
    ('outoflight', '0038', 229, 144, 'e1638569ea292812dd6633e3968b99f78c18c2f4', 'docs/audit/c-reports/outoflight-229/', 'OL229', 'outoflight-229',
     ['dying', 'lit', 'everburn', 'dead'], None, None),
    ('mapwall', '0039', 212, 148, '9fe10e42709c480e7b09c8472f9205b9281d6a1c', 'docs/audit/c-reports/mapwall-212/', 'MW212', 'mapwall-212',
     ['door-close', 'door-open', 'no-change'], None, None),
    ('enlightenment', '0040', 81, 157, '8eb4b9951cdd8a3cc3a702ce0109643dba1f98f1', 'docs/audit/c-reports/CBUG-0065/', 'CBUG0065', 'CBUG-0065',
     ['dungeon', 'town-control', 'unknown-control', 'unrelated-control', 'augmentation-dungeon', 'augmentation-town'], 'asan', None),
    ('stairreturn', '0041', 230, 167, '484c534d59c47a4a84273b7651b0c483cf8b5b67', 'docs/audit/c-reports/stairreturn-230/', 'SR230', 'stairreturn-230',
     ['stairs-dead-torch', 'lit', 'dead-no-stairs'], None, None),
    # Single-case fixture: it reads no mode variable ('default' is a label).
    # Its evidence script keeps ASan reporting the NULL strcmp read.
    ('blowname', '0042', 247, 176, 'eea8f6e1cc24ef72fb0dc07ff6df888e100bf3f9', 'docs/audit/c-reports/blowname-247/', 'BW247', 'blowname-247',
     ['default'], 'asan', 'halt_on_error=0:detect_leaks=0'),
]


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    tracker = Path(sys.argv[1])
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, patch, source, issue, evidence, prefix, env, marker, modes, sanitizer, asan_options) in FIXTURES:
        full = git(tracker, 'rev-parse', evidence + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        scenario = git(tracker, 'show', f'{full}:{prefix}scenario.c')
        (directory / 'scenario.c').write_bytes(scenario)
        entry = {'name': name, 'patch': patch, 'source_issue': source, 'consumer_issue': issue,
                 'evidence_commit': full, 'evidence_path': prefix + 'scenario.c',
                 'original_sha256': hashlib.sha256(scenario).hexdigest(),
                 'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                 'env_prefix': env, 'marker': f'native-c/{marker} finished: 1/1 passed',
                 'modes': modes, 'sanitizer': sanitizer}
        if asan_options:
            entry['asan_options'] = asan_options
        try:
            config = git(tracker, 'show', f'{full}:{prefix}borg.txt')
        except subprocess.CalledProcessError:
            config = None
        if config is not None:
            (directory / 'borg.txt').write_bytes(config)
            entry['config_sha256'] = hashlib.sha256(config).hexdigest()
        entries.append(entry)
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
