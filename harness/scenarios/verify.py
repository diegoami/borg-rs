#!/usr/bin/env python3
"""Check native engine outcomes and exact C/C/Rust saves."""

import argparse, hashlib, json, os, subprocess, tempfile
from pathlib import Path

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument("generator", type=Path)
p.add_argument("c1", type=Path)
p.add_argument("c2", type=Path)
p.add_argument("rust", type=Path)
p.add_argument("output", type=Path)
p.add_argument("--recognition-fixed", action="store_true",
               help="expect the post-M4 #295 prelaid ring outcome (24 glyphs, no new cast)")
a = p.parse_args()
assert not a.output.exists()
binary = a.generator.resolve()
results = {}
names = [
    l.split("#")[0].split()[0]
    for l in (root / "harness/suites/synthetic-tactics").read_text().splitlines()
    if l.split("#")[0].strip()
]
for directory in (a.c1, a.c2, a.rust):
    assert (directory / "DONE").exists() and not (directory / "FAILED").exists()
with tempfile.TemporaryDirectory(prefix="borg-scenario-verify-") as tmp:
    for name in names:
        blobs = [(d / name / "final.save").read_bytes() for d in (a.c1, a.c2, a.rust)]
        assert blobs[0] == blobs[1] == blobs[2], (name, "complete engine saves differ")
        proc = subprocess.run(
            [
                str(binary),
                "-mborgh",
                "-duser=" + tmp,
                "--",
                "-seed",
                "7100",
                "-load",
                str((a.c1 / name / "final.save").resolve()),
                "-race",
                "0",
                "-class",
                "0",
                "-turns",
                "1000000000",
            ],
            cwd=binary.parent,
            env={**os.environ, "BORG_SYNTHETIC_INSPECT": "1"},
            capture_output=True,
            text=True,
            check=True,
        )
        line = next(s for s in proc.stderr.splitlines() if s.startswith("SYNTH_STATE "))
        engine = json.loads(line.removeprefix("SYNTH_STATE "))
        summary = json.loads((a.c1 / name / "summary.json").read_text())
        messages = (a.c1 / name / "messages.log").read_text()
        assert not summary["dead"] and engine["level"] == 50
        if name.startswith("synth-lunal"):
            assert summary["stop_reason"] == "borg_stopped" and engine["depth"] == 21
            assert ("# Fleeing level. Lunal Mode" in messages) == (
                name == "synth-lunal-ready"
            )
        elif name == "synth-glyph-ready":
            assert (engine["engine_glyphs"], engine["rune_scrolls"]) == (1, 19)
        elif name == "synth-glyph-low-supply":
            assert (engine["engine_glyphs"], engine["rune_scrolls"]) == (0, 9)
        elif name == "synth-glyph-prelaid":
            expected = (24, 20) if a.recognition_fixed else (25, 19)
            assert (engine["engine_glyphs"], engine["rune_scrolls"]) == expected
        results[name] = {
            "engine": engine,
            "final_save_sha256": hashlib.sha256(blobs[0]).hexdigest(),
            "c1_c2_rust_save_bytes_identical": True,
            "stop_reason": summary["stop_reason"],
            "keys": summary["keys"],
        }
a.output.parent.mkdir(exist_ok=True, parents=True)
a.output.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n")
print("PASS six real engine outcomes and complete C/C/Rust saved-byte equality")
