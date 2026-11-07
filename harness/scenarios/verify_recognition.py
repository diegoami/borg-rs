#!/usr/bin/env python3
"""Assert real before/after perception outcomes and C/C/Rust byte equality."""
import argparse
import hashlib
import json
import lzma
from pathlib import Path

p = argparse.ArgumentParser(description=__doc__)
for name in ("before_c", "before_observed", "after_c1", "after_c2", "after_rust", "after_observed", "output"):
    p.add_argument(name, type=Path)
a = p.parse_args()
assert not a.output.exists()
names = ["glyph-known-rune", "glyph-known-decoy", "glyph-known-trap",
         "glyph-hidden-rune", "glyph-empty-floor", "glyph-prelaid-ring"]
streams = ("keys.log", "messages.log", "state.log", "final.save")


def stream_bytes(directory, name, stream):
    path = directory / name / stream
    if path.exists():
        return path.read_bytes()
    with lzma.open(str(path) + ".xz", "rb") as source:
        return source.read()


def probe(directory, name):
    lines = (directory / name / "stderr.log").read_text().splitlines()
    state = [json.loads(s.removeprefix("SYNTH_STATE ")) for s in lines
             if s.startswith("SYNTH_STATE ")]
    traps = [json.loads(s.removeprefix("SYNTH_TRAPS ")) for s in lines
             if s.startswith("SYNTH_TRAPS ")]
    assert len(state) == len(traps) == 1
    return {**state[0], **traps[0]}


for directory in (a.before_c, a.after_c1, a.after_c2, a.after_rust):
    assert (directory / "DONE").exists() and not (directory / "FAILED").exists()
    assert "overrides=none" in (directory / "manifest").read_text().splitlines()

rows = {}
for name in names:
    for normal, observed in ((a.before_c, a.before_observed), (a.after_c1, a.after_observed)):
        for stream in streams:
            assert stream_bytes(normal, name, stream) == stream_bytes(observed, name, stream), (name, stream, "observer changed output")
        process = json.loads((observed / name / "process.json").read_text())
        assert process["status"] == "success" and process["exit_code"] == 0
    hashes = {}
    for stream in streams:
        blobs = [stream_bytes(d, name, stream) for d in (a.after_c1, a.after_c2, a.after_rust)]
        assert blobs[0] == blobs[1] == blobs[2], (name, stream)
        hashes[stream] = hashlib.sha256(blobs[0]).hexdigest()
    before, after = (probe(d, name) for d in (a.before_observed, a.after_observed))
    assert before["borg_map_glyphs"] == 0
    assert after["borg_map_traps"] == before["borg_map_traps"] == int(name == "glyph-known-trap")
    if name in ("glyph-known-rune", "glyph-known-decoy"):
        assert before["engine_glyphs"] == 1 and before["borg_tracked_glyphs"] == 0
        assert before["rune_scrolls"] == 9
        assert after["engine_glyphs"] == after["borg_map_glyphs"] == after["borg_tracked_glyphs"] == 1
        assert after["rune_scrolls"] == 9
    elif name == "glyph-prelaid-ring":
        assert (before["engine_glyphs"], before["rune_scrolls"], before["borg_tracked_glyphs"], before["borg_morgoth_position"]) == (25, 19, 1, 0)
        assert after["engine_glyphs"] == after["borg_map_glyphs"] == after["borg_tracked_glyphs"] == 24
        assert after["rune_scrolls"] == 20 and after["borg_morgoth_position"] == 1
    else:
        assert before["engine_glyphs"] == int(name == "glyph-hidden-rune")
        assert before["rune_scrolls"] == 9 and before["borg_tracked_glyphs"] == 0
        assert after["borg_map_glyphs"] == after["borg_tracked_glyphs"] == 0
        assert after["rune_scrolls"] == 9
        assert after["engine_glyphs"] == int(name == "glyph-hidden-rune")
        assert after["visible_engine_glyphs"] == 0
    rows[name] = {"before": before, "after": after,
                  "c1_c2_rust_streams_identical": hashes,
                  "both_observers_preserve_normal_streams": True}
a.output.write_text(json.dumps(rows, indent=2, sort_keys=True) + "\n")
print("PASS six recognition outcomes; before fails rune knowledge; C/C/Rust and observer bytes match")
