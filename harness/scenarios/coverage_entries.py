#!/usr/bin/env python3
"""Measure the two target function entries separately for each scenario profile."""

import argparse, hashlib, json, subprocess, tempfile
from pathlib import Path

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("collection", type=Path)
p.add_argument("output", type=Path)
p.add_argument("--llvm-bin", required=True, type=Path)
a = p.parse_args()
bid = dict(
    l.split("=", 1) for l in (a.collection / "BUILD_ID").read_text().splitlines()
)
image = Path(bid["binary"])
assert hashlib.sha256(image.read_bytes()).hexdigest() == bid["binary_sha256"]
entries = json.loads((a.collection / "synthetic-tactics/profiles.json").read_text())
rows = {}
with tempfile.TemporaryDirectory(prefix="borg-synthetic-entries-") as directory:
    for name, entry in entries.items():
        prof = Path(directory) / "one.profdata"
        subprocess.run(
            [
                str(a.llvm_bin / "llvm-profdata"),
                "merge",
                "-sparse",
                entry["profile"],
                "-o",
                str(prof),
            ],
            check=True,
        )
        data = json.loads(
            subprocess.check_output(
                [
                    str(a.llvm_bin / "llvm-cov"),
                    "export",
                    str(image),
                    "-instr-profile=" + str(prof),
                ]
            )
        )["data"][0]
        counts = {
            label: next(
                f["count"] for f in data["functions"] if f["name"].endswith(symbol)
            )
            for label, symbol in [
                ("self_lunal", "24borg_think_dungeon_lunal"),
                ("glyph_builder", "15borg_flow_glyph"),
            ]
        }
        assert bool(counts["self_lunal"]) == (name == "synth-lunal-ready"), (
            name,
            counts,
        )
        assert bool(counts["glyph_builder"]) == (
            name in ("synth-glyph-ready", "synth-glyph-prelaid")
        ), (name, counts)
        rows[name] = counts
        print(name, counts, flush=True)
a.output.parent.mkdir(parents=True, exist_ok=True)
a.output.write_text(json.dumps(rows, indent=2, sort_keys=True) + "\n")
