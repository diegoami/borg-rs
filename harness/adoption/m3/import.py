#!/usr/bin/env python3
"""Import the native fixtures behind the C milestone3 carries.

Reads immutable Git objects only: each fixture's scenario.c/borg.txt from its
recorded evidence commit in a neo-angband-tracker clone, or (three adapted
fixtures) from better-borg-c at the milestone3 runtime snapshot. Writes
fixtures/<name>/ and fixtures/manifest.json; refuses an existing directory.
Fixture bytes are not edited. Optional provenance regeneration: normal
validation uses the committed copies and needs no clone or network.

Usage: import.py TRACKER_CLONE CONSUMER_CLONE
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONSUMER_PIN = '702f091f6ac06cae1014df2f85ac218be0189813'
PREVIOUS_PIN = '2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f'

# name, patch, source issue, consumer issue, evidence commit, prefix,
# env prefix, pass marker id, modes, adapted fixture (consumer path) or None,
# sanitizer: None | 'asan' (memory errors) | 'lsan' (leaks)
FIXTURES = [
    ('shot-path', '0008', 279, 40, 'cef0b118e02976a3384d4c348ba4cd0210c1f492', 'docs/audit/c-reports/shot-path-279/', 'SHOTPATH279', 'shot-path-279',
     ['clear-path', 'blocked', 'ball-control', 'direct-control', 'boulder-control', 'boulder-blocked'], None, None),
    ('per-spell', '0009', 64, 44, '0507c24cccd2131710e4e6cd566f901bad94e081', 'docs/audit/c-reports/CBUG-0048/', 'CBUG0048', 'CBUG-0048',
     ['clear-path', 'blocked', 'ball-control', 'direct-control', 'boulder-control'], None, None),
    ('glyph-tracker', '0010', 118, 48, 'f1f53106526412cfe44fa01c73d00e7dbf4dbeed', 'docs/audit/c-reports/CBUG-0102/', 'CBUG0102', 'CBUG-0102',
     ['stairs', 'clear-floor', 'pairing'], 'tools/glyph-tracker-scenario.c', 'asan'),
    ('missile-no-target', '0011', 254, 51, '8b30eb584', 'docs/audit/c-reports/missile-254/', 'M254', 'missile-254',
     ['no-target', 'live-shot', 'simulated'], None, 'asan'),
    ('tp-other-skip', '0012', 67, 54, '83cedb3fe', 'docs/audit/c-reports/CBUG-0051/', 'CBUG0051', 'CBUG-0051',
     ['two-phase', 'no-morgoth-phase', 'single'], None, 'asan'),
    ('dispel-filter', '0013', 248, 57, '6df50d2e', 'docs/audit/c-reports/dispel-248/', 'D248', 'dispel-248',
     ['living', 'undead', 'bolt'], 'tools/dispel-filter-scenario.c', None),
    ('arc-angle', '0014', 253, 60, '6f1f66446', 'docs/audit/c-reports/arc-253/', 'A253', 'arc-253',
     ['arc-bystander', 'arc-both'], None, None),
    ('munchkin-stair', '0015', 244, 63, 'f5b3c50e2', 'docs/audit/c-reports/munchkin-244/', 'M244', 'munchkin-244',
     ['phantom', 'on-stair'], None, None),
    ('vampire-strike', '0016', 257, 66, 'e41e44d96', 'docs/audit/c-reports/vampire-257/', 'V257', 'vampire-257',
     ['near-far', 'near-near', 'far'], None, None),
    ('slay-traits', '0017', 239, 69, '25b011863', 'docs/audit/c-reports/slay-239/', 'S239', 'slay-239',
     ['animal', 'nonanimal'], None, None),
    ('detect-panel', '0018', 290, 72, 'd9d4c7186', 'docs/audit/c-reports/detect-290/', 'D290', 'detect-290',
     ['stale', 'fresh'], None, 'asan'),
    ('activ-lookup', '0019', 260, 75, 'f0e856241', 'docs/audit/c-reports/activ-260/', 'A260', 'activ-260',
     ['miswired', 'control'], None, None),
    ('reserve-bolt', '0020', 258, 78, '359882c25', 'docs/audit/c-reports/reserve-258/', 'R258', 'reserve-258',
     ['veto', 'control'], None, None),
    ('dispel-reserve', '0021', 259, 81, 'f0ed9115c', 'docs/audit/c-reports/dispel-259/', 'D259', 'dispel-259',
     ['reserve', 'clear'], None, None),
    ('losdispel', '0022', 326, 84, '9ae8856aa', 'docs/audit/c-reports/losdispel-326/', 'L326', 'losdispel-326',
     ['storm', 'control'], None, None),
    ('staff-holy', '0023', 262, 87, 'ba63014b3', 'docs/audit/c-reports/staff-262/', 'S262', 'staff-262',
     ['holy', 'control'], None, None),
    ('blind-frighten', '0024', 261, 90, 'cb8554e07', 'docs/audit/c-reports/blind-261/', 'B261', 'blind-261',
     ['blind', 'control'], 'tools/blind-frighten-scenario.c', None),
    ('calc-reload', '0026', 325, 96, '8764d94b8', 'docs/audit/c-reports/calcreload-325/', 'T325', 'calcreload-325',
     ['reload', 'control'], None, 'lsan'),
    ('stale-target', '0027', 322, 99, '929764f4', 'docs/audit/c-reports/staletarget-322/', 'T322', 'staletarget-322',
     ['stale', 'control'], None, None),
    ('glow-overflow', '0028', 311, 102, '1a9946c6', 'docs/audit/c-reports/glowoverflow-311/', 'T311', 'glowoverflow-311',
     ['glow', 'control'], None, 'asan'),
    ('light-modifier', '0029', 238, 105, 'f1e899b6', 'docs/audit/c-reports/light-238/', 'T238', 'light-238',
     ['phial', 'torch'], None, None),
    ('surrounded-traps', '0030', 312, 109, '44334c6f', 'docs/audit/c-reports/surrounded-312/', 'T312', 'surrounded-312',
     ['traps', 'walls'], None, None),
    ('home-blows', '0031', 313, 113, 'd73c67ab', 'docs/audit/c-reports/homeblows-313/', 'T313', 'homeblows-313',
     ['stack', 'single'], None, None),
    ('lunal-state', '0032', 314, 117, 'd50baba7', 'docs/audit/c-reports/lunal-314/', 'T314', 'lunal-314',
     ['town', 'pack'], None, None),
    ('opblock-leak', '0033', 324, 121, '6cbb0347', 'docs/audit/c-reports/opblock-324/', 'T324', 'opblock-324',
     ['operators', 'control'], None, 'lsan'),
    ('calfree-tokens', '0034', 323, 124, '0799d463', 'docs/audit/c-reports/calfree-323/', 'T323', 'calfree-323',
     ['free', 'control'], None, 'lsan'),
]
# Profile edits applied when a run materializes borg.txt; the committed
# fixture config stays byte-identical. munchkin-244's evidence script
# enables the munchkin start the same way (its pinned config ships FALSE).
CONFIG_OVERRIDES = {'munchkin-stair': [['borg_munchkin_start = FALSE', 'borg_munchkin_start = TRUE']]}

# Patch 0025 (source287/consumer93) is not imported: its fixture calls the
# five-argument borg_what_text(), which local correction #287 replaced with
# a capacity-taking signature that already carries its own native evidence.


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def main():
    tracker, consumer = map(Path, sys.argv[1:3])
    out = HERE / 'fixtures'
    out.mkdir(exist_ok=False)
    entries = []
    for (name, patch, source, issue, evidence, prefix, env, marker, modes, adapted, sanitizer) in FIXTURES:
        full = git(tracker, 'rev-parse', evidence + '^{commit}').decode().strip()
        directory = out / name
        directory.mkdir()
        original = git(tracker, 'show', f'{full}:{prefix}scenario.c')
        entry = {'name': name, 'patch': patch, 'source_issue': source, 'consumer_issue': issue,
                 'evidence_commit': full, 'evidence_path': prefix + 'scenario.c',
                 'original_sha256': hashlib.sha256(original).hexdigest(),
                 'env_prefix': env, 'marker': f'native-c/{marker} finished: 1/1 passed',
                 'modes': modes, 'sanitizer': sanitizer}
        if adapted:
            scenario = git(consumer, 'show', f'{CONSUMER_PIN}:{adapted}')
            entry['adapted_from'] = {'repository': 'diegoami/better-borg-c', 'commit': CONSUMER_PIN, 'path': adapted}
        else:
            scenario = original
        (directory / 'scenario.c').write_bytes(scenario)
        entry['scenario_sha256'] = hashlib.sha256(scenario).hexdigest()
        try:
            config = git(tracker, 'show', f'{full}:{prefix}borg.txt')
        except subprocess.CalledProcessError:
            config = None
        if config is not None:
            (directory / 'borg.txt').write_bytes(config)
            entry['config_sha256'] = hashlib.sha256(config).hexdigest()
            if name in CONFIG_OVERRIDES:
                entry['config_overrides'] = CONFIG_OVERRIDES[name]
        entries.append(entry)
    manifest = {'consumer_pin': CONSUMER_PIN, 'previous_consumer_pin': PREVIOUS_PIN,
                'baseline': 'e473720ed88a0f0c7ca66815cef3000eb4a38bc3', 'fixtures': entries}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    main()
