#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone9 carries (0090-0099).

Reads immutable Git objects only: scenario.c and borg.txt at each fixture's
recorded neo-angband-tracker evidence commit, plus borg-rs's own
staleglyph fixture (local/staleglyph), which reproduces the NULL read of
patch 0098 and checks the local guard. Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m9/fixtures.

Usage: import.py TRACKER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '33207fbeeb582e67d6b18e9f40613617667905ae'
PREVIOUS_PIN = '017e038e8726a7a423887a366b5d40e89c0550a1'
REPOSITORIES = {'tracker': 'diegoami/neo-angband-tracker', 'consumer': 'diegoami/better-borg-c'}

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# suite id, modes, sanitizer
FIXTURES = [
    ('starvealch', '0090', 18, 359, '99db3c5ca26afceb280b0f49fb48f2756176488e', 'docs/audit/c-reports/starvealch-18/', 'SA18',
     'starvealch-18', ['starving', 'general_food', 'fed'], None),
    ('torodvalue', '0091', 549, 361, '8b30cf629baf9ee0b2cc4ee381c402bcc293598a', 'docs/audit/c-reports/torodvalue-549/', 'TR549',
     'torodvalue-549', ['rod', 'wand', 'weak'], None),
    ('healchance', '0092', 364, 363, 'f88fee1e3ace2b5f34f6fc8a1b4fbc30d3f94da1', 'docs/audit/c-reports/healchance-364/', 'HC364',
     'healchance-364', ['danger', 'unique', 'risky', 'control'], None),
    ('homebuyqty', '0093', 19, 365, 'fbd96fa79610d00a148372efd55a63ee5da13c66', 'docs/audit/c-reports/homebuyqty-19/', 'HB19',
     'homebuyqty-19', ['single', 'pair', 'roomy', 'stack'], None),
    ('tradehist', '0094', 20, 367, 'ddb06e523f711bf0780320926e66aca42d48f475', 'docs/audit/c-reports/tradehist-20/', 'TH20',
     'tradehist-20', ['rations-only', 'with-recall'], None),
    ('torodpower', '0095', 560, 369, '4141bfa68bf29936f2a97dbad637d7f8a7555738', 'docs/audit/c-reports/torodpower-560/', 'TP560',
     'torodpower-560', ['rod', 'wand', 'mage'], None),
    ('tporod', '0096', 17, 371, 'a85a78156f53265c85ae09d3281c0d8fd401070b', 'docs/audit/c-reports/tporod-17/', 'TP17',
     'tporod-17', ['warrior', 'rogue', 'warrior-buy', 'mage'], None),
    ('panelrows', '0097', 348, 373, '24159739b3b1642615afb362217213f290cf0127', 'docs/audit/c-reports/panelrows-348/', 'PR348',
     'panelrows-348', ['light-tall', 'light-wide', 'light-normal', 'light-fit', 'vault-tall', 'vault-wide', 'vault-normal',
                       'dig-tall', 'dig-wide', 'dig-normal'], 'asan'),
    ('runeglyph', '0098', 295, 375, 'dae7a1bd6da9a78b92229534fa2b03630e01da1d', 'docs/audit/c-reports/runeglyph-295/', 'RG295',
     'runeglyph-295', ['rune', 'floor', 'trap'], None),
    ('necrolight', '0099', 296, 377, 'ee195556f3f0eb811f5b09638a433280223a0dcc', 'docs/audit/c-reports/necrolight-296/', 'N296',
     'necrolight-296', ['lit', 'dark', 'mage'], None),
]
# Local fixture (borg-rs): the 0098 guard. Copied from local/<name>.
LOCAL = [('staleglyph', '0098', 'SG', 'staleglyph-0098', ['stale', 'fresh'], 'asan')]

def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    clone = Path(sys.argv[1])
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, patch, source, issue, evidence, prefix, env, suite, modes, sanitizer) in FIXTURES:
        full = git(clone, 'rev-parse', evidence + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        scenario = git(clone, 'show', f'{full}:{prefix}scenario.c')
        config = git(clone, 'show', f'{full}:{prefix}borg.txt')
        (directory / 'scenario.c').write_bytes(scenario)
        (directory / 'borg.txt').write_bytes(config)
        entries.append({'name': name, 'patch': patch, 'source_issue': source, 'consumer_issue': issue,
                        'evidence_repository': REPOSITORIES['tracker'], 'evidence_commit': full,
                        'evidence_path': prefix + 'scenario.c', 'config_path': prefix + 'borg.txt',
                        'original_sha256': hashlib.sha256(scenario).hexdigest(),
                        'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                        'config_sha256': hashlib.sha256(config).hexdigest(),
                        'env_prefix': env, 'marker': f'native-c/{suite} finished: 1/1 passed',
                        'modes': modes, 'sanitizer': sanitizer})
    for (name, patch, env, suite, modes, sanitizer) in LOCAL:
        directory = out / name
        directory.mkdir()
        scenario = (HERE / 'local' / name / 'scenario.c').read_bytes()
        config = (HERE / 'local' / name / 'borg.txt').read_bytes()
        (directory / 'scenario.c').write_bytes(scenario)
        (directory / 'borg.txt').write_bytes(config)
        entries.append({'name': name, 'patch': patch, 'evidence_repository': 'diegoami/borg-rs (local fixture)',
                        'evidence_path': f'harness/adoption/m9/local/{name}/scenario.c',
                        'original_sha256': hashlib.sha256(scenario).hexdigest(),
                        'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                        'config_sha256': hashlib.sha256(config).hexdigest(),
                        'env_prefix': env, 'marker': f'native-c/{suite} finished: 1/1 passed',
                        'modes': modes, 'sanitizer': sanitizer})
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
