#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone8 carries (0078-0089).

Reads immutable Git objects only: scenario.c and borg.txt at each fixture's
recorded evidence commit, in neo-angband-tracker or, for the
consumer-designed spasticscore fixture, better-borg-c (the fixture the
consumer's own verify script uses). Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m8/fixtures.

Usage: import.py TRACKER_CLONE CONSUMER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '017e038e8726a7a423887a366b5d40e89c0550a1'
PREVIOUS_PIN = '3573d741e6dc3cafa4099fcef8b38fdb32952977'
REPOSITORIES = {'tracker': 'diegoami/neo-angband-tracker', 'consumer': 'diegoami/better-borg-c'}

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# suite id, modes, repository
FIXTURES = [
    ('banisharte', '0078', 37, 305, 'bbc5de859c7b27d02d88c1bb84ec46d796de8716', 'docs/audit/c-reports/banisharte-37/', 'BA37',
     'banisharte-37', ['main', 'hurt'], 'tracker'),
    ('restmult', '0079', 149, 306, 'c6052b4a76ea641b42844ce8e7c984cffbdc5f31', 'docs/audit/c-reports/restmult-149/', 'RM149',
     'restmult-149', ['sameturn', 'recent'], 'tracker'),
    ('shopline', '0080', 125, 307, '9ce7a0a4752d7564a246ee6508f1e523bc54ffae', 'docs/audit/c-reports/shopline-125/', 'SL125',
     'shopline-125', ['inline', 'offrow'], 'tracker'),
    ('lightwest', '0081', 40, 309, '8e9e1da6a5729a619e908e8aa351e25f8a8aa90d', 'docs/audit/c-reports/lightwest-40/', 'LW40',
     'lightwest-40', ['west', 'east'], 'tracker'),
    ('modcredit', '0082', 70, 311, '0af6c5af047a55d80a228c431509c6bab07b1168', 'docs/audit/c-reports/modcredit-70/', 'MC70',
     'modcredit-70', ['statring', 'speed', 'nomods'], 'tracker'),
    ('neckdead', '0083', 47, 312, '3885bfcd07dd13063c361002e8f5048a1df19325', 'docs/audit/c-reports/neckdead-47/', 'ND47',
     'neckdead-47', ['neck2', 'neck3', 'neck4', 'neck5'], 'tracker'),
    ('lightbeam', '0084', 88, 316, '90e5fc16bfad3e7fd9ea4eff35ec9e95271b382a', 'docs/audit/c-reports/lightbeam-88/', 'LB88',
     'lightbeam-88', ['scrolled', 'scrolled_south', 'top'], 'tracker'),
    ('hurtstun', '0085', 62, 329, '80318246564ab617197c01032a0368b2218555a5', 'docs/audit/c-reports/hurtstun-62/', 'HS62',
     'hurtstun-62', ['mystic', 'ant', 'orc', 'giant'], 'tracker'),
    ('dimdoor', '0086', 21, 335, 'f8b7d662b62a97219095914150d6d577851cb3ac', 'docs/audit/c-reports/dimdoor-21/', 'DD21',
     'dimdoor-21', ['granite', 'open'], 'tracker'),
    ('urgentshop', '0087', 23, 336, 'e0ecd1ef777447ab02e84255be6717fea303921f', 'docs/audit/c-reports/urgentshop-23/', 'US23',
     'urgentshop-23', ['poisoned', 'bleeding', 'healthy'], 'tracker'),
    ('deadtake', '0088', 72, 338, '6332987b90c9eeb55a4f90014a1e4e22cef18e0e', 'docs/audit/c-reports/deadtake-72/', 'DT72',
     'deadtake-72', ['dead-entry', 'live-only'], 'tracker'),
    ('spasticscore', '0089', 147, 288, '3d6a0953cb9127fcc0aa000beddfd1ebd5b3ab44', 'docs/audit/c-reports/spasticscore-147-product/',
     'PS147', 'spasticscore-147-product', ['mismatch', 'consistent'], 'consumer'),
]


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    repos = {'tracker': Path(sys.argv[1]), 'consumer': Path(sys.argv[2])}
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, patch, source, issue, evidence, prefix, env, suite, modes, repo) in FIXTURES:
        clone = repos[repo]
        full = git(clone, 'rev-parse', evidence + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        scenario = git(clone, 'show', f'{full}:{prefix}scenario.c')
        config = git(clone, 'show', f'{full}:{prefix}borg.txt')
        (directory / 'scenario.c').write_bytes(scenario)
        (directory / 'borg.txt').write_bytes(config)
        entries.append({'name': name, 'patch': patch, 'source_issue': source, 'consumer_issue': issue,
                        'evidence_repository': REPOSITORIES[repo], 'evidence_commit': full,
                        'evidence_path': prefix + 'scenario.c', 'config_path': prefix + 'borg.txt',
                        'original_sha256': hashlib.sha256(scenario).hexdigest(),
                        'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                        'config_sha256': hashlib.sha256(config).hexdigest(),
                        'env_prefix': env, 'marker': f'native-c/{suite} finished: 1/1 passed',
                        'modes': modes, 'sanitizer': None})
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
