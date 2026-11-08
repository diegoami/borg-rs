#!/usr/bin/env python3
"""Export immutable M2 patches/fixtures from a read-only C consumer clone."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

PIN = '2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f'
ROOT = Path(__file__).resolve().parents[3]
FIXTURES = [
    ('swap', 'CBUG-0066', '1b67370d0dc56d9b7ef34e5356e63d21a7095545', 'expanded/scenario.c',
     'armour-better weapon-better armour-only weapon-only tie no-benefit fire-armour electric-weapon both-helpful'),
    ('knockout', 'CBUG-0002', '3de1e3123915587483395dd40c4d2e75704e65f4', 'scenario.c', 'knockout heavy-stun paralysis none'),
    ('preparation', 'CBUG-0094', 'a19c8733c0dfa8139b04eecd8117429ec5f55cd6', 'scenario.c',
     'regen-only speed-wins speed-only already-active unlearned blind confused town low-mana'),
    ('cold', 'CBUG-0004', 'b0b0fa2471a11e91838bcbc3d980f4b04f8c6b01', 'scenario.c',
     'acid-only cold-only neither both physical-acid-control physical-cold-control'),
    ('rings', 'CBUG-0062', 'bfe3fc402e8ae6cdec74d8b35fa8f50e612f2a8e', 'scenario.c',
     'left-ready right-charging-left-ready right-control both-ready left-charging no-ring blind-control'),
    ('potion', 'CBUG-0061', 'cce1d5d92079e78e939126d3cf35e24121442734', 'scenario.c',
     'long-gap recent boundary-four same-turn bypass-control reserve-control empty-control first-use first-turn'),
    ('name', 'CBUG-0086', 'b61f6b611600fdbc4c6370fff3ac2835ee77017f', 'scenario.c',
     'named-dragon ogre-control unseen-control it-control lower-it-control'),
]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('donor', type=Path)
    args = parser.parse_args()
    def read(pin, path):
        return subprocess.check_output(['git', '-C', str(args.donor), 'show', pin + ':' + path])
    output = ROOT / 'docs/adoption/c-m2'
    output.mkdir(parents=True, exist_ok=False)
    patches = output / 'patches';patches.mkdir()
    ledger = read(PIN, 'patches/series.json')
    (output / 'series.json').write_bytes(ledger)
    for row in json.loads(ledger)['patches']:
        (patches / Path(row['patch']).name).write_bytes(read(PIN, row['patch']))
    scenarios = ROOT / 'harness/adoption/m2/fixtures'
    scenarios.mkdir(exist_ok=False)
    manifest = {'consumer_pin': PIN, 'baseline': json.loads(ledger)['baseline'], 'fixtures': []}
    for name, identifier, pin, source, modes in FIXTURES:
        directory = scenarios / name;directory.mkdir()
        prefix = 'docs/audit/c-reports/' + identifier + '/'
        original = read(pin, prefix + source)
        config = read(pin, prefix + 'borg.txt')
        (directory / 'scenario.c').write_bytes(original)
        (directory / 'borg.txt').write_bytes(config)
        manifest['fixtures'].append({'name': name, 'id': identifier, 'source_commit': pin, 'source_path': prefix + source,
            'source_sha256': hashlib.sha256(original).hexdigest(), 'config_sha256': hashlib.sha256(config).hexdigest(),
            'modes': modes.split(), 'environment': identifier.replace('-', '') + '_MODE'})
    (scenarios / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
