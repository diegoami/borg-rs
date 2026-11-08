#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone6 carries (0051-0063).

Reads immutable Git objects only: scenario.c/borg.txt at each fixture's
recorded neo-angband-tracker evidence commit (these fixtures keep their
config at profile/borg.txt). Writes fixtures/<name>/ and
fixtures/manifest.json in the milestone3 runner's format; refuses an
existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m6/fixtures.

Usage: import.py TRACKER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = 'b3c825c97c8aef18b252a7fb7d9b7dceef83d586'
PREVIOUS_PIN = '5384f12ed51e8fef6f31353fd2daaf0e7fb43856'

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# marker id, modes, sanitizer, asan options (None = runner default)
FIXTURES = [
    ('stairrange', '0051', 245, 242, '5da8217435bb693c3aabbff9d75520f96f355c61', 'docs/audit/c-reports/stairrange-245/', 'SR245', 'stairrange-245',
     ['main'], None, None),
    ('clockreset', '0052', 246, 245, '6d4859904037aa987577a8bcab522d5ebd4ae17a', 'docs/audit/c-reports/clockreset-246/', 'CR246', 'clockreset-246',
     ['main'], None, None),
    ('dangerflee', '0053', 156, 247, '7d796742e9ad496695baa24f9b076f203a5d74c6', 'docs/audit/c-reports/dangerflee-156/', 'DF156', 'dangerflee-156',
     ['excessive', 'morgoth', 'hurt'], None, None),
    ('recallmana', '0054', 157, 250, '5a8572e08e24321cce00e65a4c2c7832c4a1315d', 'docs/audit/c-reports/recallmana-157/', 'RM157', 'recallmana-157',
     ['lowmana', 'halfmana'], None, None),
    ('deskstair', '0055', 153, 252, '993a75f8a3eef3dcc073e7dbb6b69359dd8ab635', 'docs/audit/c-reports/deskstair-153/', 'DS153', 'deskstair-153',
     ['twostairs', 'twodown', 'onestair'], None, None),
    ('backaway', '0056', 155, 254, 'c7bd31508cf6a8f001b408b810263fb297d0ddd2', 'docs/audit/c-reports/backaway-155/', 'BA155', 'backaway-155',
     ['monster', 'trap', 'healthy'], None, None),
    ('cfghang', '0057', 141, 257, '096ee66813dad5a32f9468d5fff2c48d2a46077c', 'docs/audit/c-reports/cfghang-141/', 'CH141', 'cfghang-141',
     ['word', 'suffix', 'valid'], None, None),
    ('cfgminus', '0058', 142, 258, '0a21f5ce0cfdadfb1d3520480425a337bab5f752', 'docs/audit/c-reports/cfgminus-142/', 'CN142', 'cfgminus-142',
     ['minus', 'valid', 'negative'], None, None),
    ('cfgmissing', '0059', 144, 259, 'f37cc9b515e361913a6c680451c4ceab62e3f51c', 'docs/audit/c-reports/cfgmissing-144/', 'CM144', 'cfgmissing-144',
     ['missing', 'present'], None, None),
    ('countsell', '0060', 134, 261, '6aedfa6f16479584a9c59c6bd6dbdc8408f624a8', 'docs/audit/c-reports/countsell-134/', 'CS134', 'countsell-134',
     ['full-unid', 'full', 'light'], None, None),
    ('drainedive', '0061', 235, 263, '300d18cdd256f6055f2ae905057e0254810c9b07', 'docs/audit/c-reports/drainedive-235/', 'DD235', 'drainedive-235',
     ['main'], None, None),
    ('respois', '0062', 184, 266, 'c94b12b3c58caeb3310a772f2322b76baa711ebe', 'docs/audit/c-reports/respois-184/', 'RP184', 'respois-184',
     ['apples', 'rations'], None, None),
    ('respoiswar', '0063', 185, 267, '58a366e8b9bdc886c9c94292271ea0313e678906', 'docs/audit/c-reports/respoiswar-185/', 'RW185', 'respoiswar-185',
     ['cold', 'plain'], None, None),
]
# Single-case fixtures (stairrange, clockreset, drainedive) read no mode
# variable or ignore it; 'main' is a label.
# recallmana-157 is also run adapted, exactly as the consumer's verify script
# does: one line changed so the 0054 path is reachable after 0053 (layered
# interaction). The original's lowmana mode stays red before and after.
ADAPTED = {'recallmana-adapted': ('recallmana', b'borg.ready_morgoth = 0;', b'borg.ready_morgoth = 1;')}


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
            config = git(tracker, 'show', f'{full}:{prefix}profile/borg.txt')
        except subprocess.CalledProcessError:
            config = None
        if config is not None:
            (directory / 'borg.txt').write_bytes(config)
            entry['config_sha256'] = hashlib.sha256(config).hexdigest()
        entries.append(entry)
    for name, (base, old, new) in ADAPTED.items():
        entry = dict(next(e for e in entries if e['name'] == base))
        scenario = (out / base / 'scenario.c').read_bytes()
        assert scenario.count(old) == 1
        adapted = scenario.replace(old, new)
        directory = out / name
        directory.mkdir()
        (directory / 'scenario.c').write_bytes(adapted)
        (directory / 'borg.txt').write_bytes((out / base / 'borg.txt').read_bytes())
        entry.update(name=name, scenario_sha256=hashlib.sha256(adapted).hexdigest(),
                     transform={'replace': old.decode(), 'with': new.decode()})
        entries.append(entry)
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
