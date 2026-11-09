#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone7 carries (0064-0077).

Reads immutable Git objects only: scenario.c and the fixture's borg.txt
(at profile/borg.txt or borg.txt, as the consumer's verify script reads it)
at each fixture's recorded neo-angband-tracker evidence commit. Writes
fixtures/<name>/ and fixtures/manifest.json in the milestone3 runner's
format; refuses an existing directory. Run the fixtures with
harness/adoption/m3/native.py --fixtures harness/adoption/m7/fixtures.

Usage: import.py TRACKER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '3573d741e6dc3cafa4099fcef8b38fdb32952977'
PREVIOUS_PIN = 'b3c825c97c8aef18b252a7fb7d9b7dceef83d586'

# name, patch, source issue, consumer issue, evidence, prefix, env prefix,
# suite id, modes, sanitizer, config path inside the prefix
FIXTURES = [
    ('spastic', '0064', 146, 268, '33f288ef664527ddef80dcda928a8e8db6b1cff5', 'docs/audit/c-reports/spastic-146/', 'SP146', 'spastic-146',
     ['middle', 'corner', 'far'], None, 'profile/borg.txt'),
    ('glyphflow', '0065', 345, 269, '4695d78e4e6ed5e2f0361520029c86badfa6c2ba', 'docs/audit/c-reports/glyphflow-345/', 'GF345', 'glyphflow-345',
     ['center', 'offcenter'], None, 'profile/borg.txt'),
    ('swapacid', '0066', 199, 270, 'fbc0de07ffa7729f618ea64364086d8b80dd1254', 'docs/audit/c-reports/swapacid-199/', 'SA199', 'swapacid-199',
     ['elec', 'acid', 'fire', 'none'], None, 'profile/borg.txt'),
    ('staircycle', '0067', 403, 280, 'eb36b06b524992d112cef4fc9ec51f8ea701c015', 'docs/audit/c-reports/staircycle-403/', 'STAIR403', 'staircycle-403',
     ['main', 'clean'], None, 'profile/borg.txt'),
    ('text287', '0068', 287, 93, 'f9706564e0a667670fb024e9e1f87da137f21c9c', 'docs/audit/c-reports/text-287/', 'T287', 'text-287',
     ['multibyte', 'control', 'wide'], 'asan', 'borg.txt'),
    ('glowtrace', '0069', 213, 224, 'f2b402c92ce57cc85d9fda777de901c2f157c720', 'docs/audit/c-reports/glowtrace-213/', 'GLOW213', 'glowtrace-213',
     ['main', 'door', 'torchlit', 'unlit'], None, 'borg.txt'),
    ('iswarded', '0070', 198, 286, '5454144a624ef51ec68cbd37e8291b45d10d865d', 'docs/audit/c-reports/iswarded-198/', 'ISW198', 'iswarded-198',
     ['rune-town', 'hit-town', 'rune-inside', 'hit-inside', 'hit-noglyph'], 'asan', 'borg.txt'),
    ('painmsg', '0071', 197, 287, 'ffb74e54ad2ccf6a88e38c174322d04814bb3748', 'docs/audit/c-reports/painmsg-197/', 'PM197', 'painmsg-197',
     ['held', 'command', 'uncommand', 'shapefail', 'stun', 'hurt', 'puff'], None, 'borg.txt'),
    ('trollswap', '0073', 201, 289, '2f3a73f374746a9de0677184128520af52f34f0b', 'docs/audit/c-reports/trollswap-201/', 'TS201', 'trollswap-201',
     ['troll-orc', 'troll-plain'], None, 'borg.txt'),
    ('drophole', '0074', 136, 290, 'ac57d5169f76bcaaf184df9a8c7595681145e94e', 'docs/audit/c-reports/drophole-136/', 'DH136', 'drophole-136',
     ['salt-water', 'spare-esp', 'restore-mana', 'level30'], None, 'borg.txt'),
    ('wearskip', '0075', 85, 294, '3735a7e72849aa4fa95ddade1458fb27a95be866', 'docs/audit/c-reports/wearskip-85/', 'WS85', 'wearskip-85',
     ['tight', 'roomy', 'nocloak'], None, 'borg.txt'),
    ('lantern', '0076', 90, 296, '8ba3564002fe376a5c5142f1e2079e7704efeb6a', 'docs/audit/c-reports/lantern-90/', 'LN90', 'lantern-90',
     ['empty', 'single'], None, 'borg.txt'),
    ('homebanish', '0077', 54, 298, 'e24fa8ccabf7e86219898b7d777868887908c426', 'docs/audit/c-reports/homebanish-54/', 'HB54', 'homebanish-54',
     ['banish', 'mass'], None, 'borg.txt'),
]
# staircycle 'main' runs two tests; the consumer's verify script requires
# "2/2 passed" for it and the generic "finished: 1/1 passed" for 'clean'.
MODE_MARKERS = {'staircycle': {'main': 'native-c/staircycle-403 finished: 2/2 passed',
                               'clean': 'finished: 1/1 passed'}}
# text-287 includes "t287-variant.h", which the consumer's script writes per
# phase (0 = unsized borg_what_text before, 1 = sized after). borg-rs already
# carried the sized signature as local correction #287 before this batch, so
# both of its oracles take the destination size: the header is 1 for both.
EXTRA_FILES = {'text287': {'t287-variant.h': b'#define T287_SIZED 1\n'}}


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    tracker = Path(sys.argv[1])
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, patch, source, issue, evidence, prefix, env, suite, modes, sanitizer, config_path) in FIXTURES:
        full = git(tracker, 'rev-parse', evidence + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        scenario = git(tracker, 'show', f'{full}:{prefix}scenario.c')
        (directory / 'scenario.c').write_bytes(scenario)
        config = git(tracker, 'show', f'{full}:{prefix}{config_path}')
        (directory / 'borg.txt').write_bytes(config)
        entry = {'name': name, 'patch': patch, 'source_issue': source, 'consumer_issue': issue,
                 'evidence_commit': full, 'evidence_path': prefix + 'scenario.c',
                 'config_path': prefix + config_path,
                 'original_sha256': hashlib.sha256(scenario).hexdigest(),
                 'scenario_sha256': hashlib.sha256(scenario).hexdigest(),
                 'config_sha256': hashlib.sha256(config).hexdigest(),
                 'env_prefix': env, 'marker': f'native-c/{suite} finished: 1/1 passed',
                 'modes': modes, 'sanitizer': sanitizer}
        if name in MODE_MARKERS:
            entry['mode_markers'] = MODE_MARKERS[name]
        if name in EXTRA_FILES:
            entry['extra_files'] = {}
            for extra, data in EXTRA_FILES[name].items():
                (directory / extra).write_bytes(data)
                entry['extra_files'][extra] = hashlib.sha256(data).hexdigest()
        entries.append(entry)
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
