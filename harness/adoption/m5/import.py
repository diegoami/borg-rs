#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone5 carries (0043-0050).

Reads immutable Git objects only: scenario.c/borg.txt at each fixture's
recorded neo-angband-tracker evidence commit. Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m5/fixtures.

Usage: import.py TRACKER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '5384f12ed51e8fef6f31353fd2daaf0e7fb43856'
PREVIOUS_PIN = '70993393a1c946ce8cd38ee558bf1ce6464350e7'

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# marker id, modes, sanitizer, asan options (None = runner default)
FIXTURES = [
    ('excavate', '0043', 236, 188, '793f723ebb684f1f2abcdb267dc759861bc7874d', 'docs/audit/c-reports/excavate-236/', 'EV236', 'excavate-236',
     ['excavate', 'no-target'], None, None),
    ('calllight', '0044', 231, 196, '85de924fa8c008a86a2ab3ff6340bfd157a69b7a', 'docs/audit/c-reports/calllight-231/', 'CL231', 'calllight-231',
     ['own-open-door', 'floor'], None, None),
    ('gloves', '0045', 205, 201, '7ff787acc6250893e4391e0998c46eeecfb67665', 'docs/audit/c-reports/gloves-205/', 'GL205', 'gloves-205',
     ['glove-fa', 'plain', 'have-fa'], None, None),
    ('detectcarry', '0046', 219, 205, '52cc5bc423dfb925f4b95b6df01357c5e23053b8', 'docs/audit/c-reports/detectcarry-219/', 'DC219', 'detectcarry-219',
     ['new-level', 'same-level'], None, None),
    ('maimfoe', '0047', 256, 214, '01ddb076c721ca131b758d303548c66a45758f40', 'docs/audit/c-reports/maimfoe-256/', 'MF256', 'maimfoe-256',
     ['wrong-return', 'single', 'none'], None, None),
    ('recalldepth', '0048', 310, 216, '1f0425e12e9af9e31b15a0973aa8e805fe44c240', 'docs/audit/c-reports/recalldepth-310/', 'RD310', 'recalldepth-310',
     ['endgame', 'generic', 'mismatch'], None, None),
    # Single-case fixture: it reads no mode variable ('main' is a label).
    ('glowtrace', '0050', 213, 224, '5e6f2ddb5296922905ad47207ed6dcc166ff3233', 'docs/audit/c-reports/glowtrace-213/', 'GLOW213', 'glowtrace-213',
     ['main'], None, None),
]
# 0049 (source404, consumer212) uses its own 150-scenario case runner: its
# files are imported unchanged under fixtures/flowcache and run by
# harness/adoption/m5/flowcache.py, not by the generic runner.
FLOWCACHE = ('89326f5216148ab47a18585236678c391bb4e559', 'docs/audit/c-reports/flowcache-404/',
             ('scenario.c', 'borg.txt', 'run-cases.py', 'reproduce.py'))


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
    evidence, prefix, names = FLOWCACHE
    full = git(tracker, 'rev-parse', evidence + '^{commit}').decode().strip()
    directory = out / 'flowcache'
    directory.mkdir()
    flowcache = {'patch': '0049', 'source_issue': 404, 'consumer_issue': 212, 'evidence_commit': full, 'files': {}}
    for name in names:
        data = git(tracker, 'show', f'{full}:{prefix}{name}')
        (directory / name).write_bytes(data)
        flowcache['files'][name] = hashlib.sha256(data).hexdigest()
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries,
                'flowcache': flowcache}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
