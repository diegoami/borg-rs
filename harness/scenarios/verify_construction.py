#!/usr/bin/env python3
"""Verify autonomous engine Rune construction and complete C/C/Rust streams."""
import argparse
import hashlib
import json
import lzma
from pathlib import Path

p = argparse.ArgumentParser(description=__doc__)
for name in ('before', 'c1', 'c2', 'rust', 'observed', 'output'):
    p.add_argument(name, type=Path)
a = p.parse_args()
assert not a.output.exists()

def data(directory, name, stream):
    path = directory / name / stream
    if path.exists():
        return path.read_bytes()
    with lzma.open(str(path) + '.xz', 'rb') as source:
        return source.read()

before = json.loads((a.before / 'observations.json').read_text())
observed = json.loads((a.observed / 'observations.json').read_text())
rows = {}
for name in ('glyph-build-ready', 'glyph-build-supply', 'glyph-build-prelaid'):
    for directory in (a.c1, a.c2, a.rust):
        assert (directory / 'DONE').exists() and not (directory / 'FAILED').exists()
        assert 'overrides=none' in (directory / 'manifest').read_text().splitlines()
    streams = {}
    for stream in ('keys.log', 'messages.log', 'state.log', 'final.save'):
        blobs = [data(d, name, stream) for d in (a.c1, a.c2, a.rust, a.observed)]
        assert all(x == blobs[0] for x in blobs), (name, stream)
        streams[stream] = hashlib.sha256(blobs[0]).hexdigest()
    state = observed[name]['native_c_state']
    old = before[name]['native_c_state']
    lines = (a.observed / name / 'stderr.log').read_text().splitlines()
    formation = json.loads(next(x.removeprefix('SYNTH_FORMATION ') for x in lines if x.startswith('SYNTH_FORMATION ')))
    summary = json.loads((a.c1 / name / 'summary.json').read_text())
    assert not summary['dead'] and state['level'] == 50
    if name == 'glyph-build-ready':
        assert (old['engine_glyphs'], old['rune_scrolls']) == (0, 40)
        assert (state['engine_glyphs'], state['rune_scrolls']) == (25, 15)
        assert state['borg_map_glyphs'] == state['borg_tracked_glyphs'] == 25
        assert state['borg_morgoth_position'] == 1
        assert formation['surrounding_runes'] == 24 and formation['rune_under_player'] == 1
        assert summary['stop_reason'] == 'rune_pattern' and summary['game_turns'] < 500
    elif name == 'glyph-build-supply':
        assert state['engine_glyphs'] == 0 and state['rune_scrolls'] == 9
        assert summary['stop_reason'] == 'turns'
    else:
        assert state['engine_glyphs'] == 24 and state['rune_scrolls'] == 40
        assert formation['surrounding_runes'] == 24
        assert summary['stop_reason'] == 'rune_pattern' and summary['game_turns'] == 1
    rows[name] = {'before_native': old, 'after_native': state, 'engine_pattern': formation,
                  'summary': summary, 'C1_C2_Rust_observer_bytes': streams}
a.output.write_text(json.dumps(rows, indent=2, sort_keys=True) + '\n')
print('PASS three construction controls; actual complete Rune pattern; C/C/Rust/save/observer equality')
