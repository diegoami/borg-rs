#!/usr/bin/env python3
"""Create each native engine fixture twice, requiring identical bytes and logs."""

import argparse, hashlib, json, os, shutil, subprocess
from pathlib import Path

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument("generator", type=Path)
p.add_argument("output", type=Path)
p.add_argument("--glyph-recognition", action="store_true",
               help="also generate Rune, decoy, harmful-trap and hidden-Rune controls")
p.add_argument("--glyph-construction", action="store_true",
               help="also generate bounded autonomous construction and supply/ring controls")
a = p.parse_args()
binary = a.generator.resolve()
out = a.output.resolve()
assert not out.exists(), "refusing existing fixture output"
meta = json.loads((binary.parent / "generator.json").read_text())
sha = lambda f: hashlib.sha256(f.read_bytes()).hexdigest()
assert sha(binary) == meta["binary_sha256"]
assert (
    sha(root / "harness/scenarios/fixture-save.c") == meta["constructor_source_sha256"]
)
out.mkdir(parents=True)
shutil.copy2(binary.parent / "generator.json", out / "generator.json")
sources = {}
cases = [
    ("lunal-ready", 20, 99),
    ("lunal-shallow-history", 20, 34),
    ("glyph-ready", 100, 100),
    ("glyph-low-supply", 100, 100),
    ("glyph-complete", 100, 100),
]
if a.glyph_recognition:
    cases += [(name, 100, 100) for name in (
        "glyph-visible-rune", "glyph-visible-trap", "glyph-hidden-rune", "glyph-decoy")]
if a.glyph_construction:
    cases += [(name, 100, 100) for name in (
        "glyph-build-ready", "glyph-build-low-supply", "glyph-build-complete")]
for name, depth, maximum in cases:
    states = []
    for number in (1, 2):
        run = out / f"pass{number}" / name
        save = out / f"pass{number}" / (name + ".save")
        env = {
            **os.environ,
            "BORG_IMPL": "c",
            "ANGBAND_BIN": str(binary),
            "BORG_SYNTHETIC_FIXTURE": name,
        }
        subprocess.run(
            [
                str(root / "harness/run.sh"),
                str(run),
                "7100",
                "0",
                "0",
                "1",
                "menu",
                "--",
                "-make-fixture",
                "100",
                "-save-on-stop",
                str(save),
            ],
            env=env,
            check=True,
            stdout=subprocess.DEVNULL,
        )
        s = json.loads((run / "summary.json").read_text())
        assert (
            s["stop_reason"] == "fixture"
            and s["keys"] == 0
            and s["depth"] == depth
            and s["max_depth"] == maximum
            and s["clevel"] == 50
            and not s["dead"]
        )
        lines = [
            l.removeprefix("SYNTH_STATE ")
            for l in (run / "stderr.log").read_text().splitlines()
            if l.startswith("SYNTH_STATE ")
        ]
        assert len(lines) == 1
        states.append(json.loads(lines[0]))
    first = out / "pass1" / (name + ".save")
    second = out / "pass2" / (name + ".save")
    assert first.read_bytes() == second.read_bytes() and states[0] == states[1], name
    for log in ("keys.log", "messages.log", "state.log"):
        assert (out / "pass1" / name / log).read_bytes() == (
            out / "pass2" / name / log
        ).read_bytes(), (name, log)
    x, y = (
        json.loads((out / f"pass{i}" / name / "summary.json").read_text())
        for i in (1, 2)
    )
    x.pop("wall_seconds")
    y.pop("wall_seconds")
    assert x == y
    shutil.copy2(first, out / (name + ".save"))
    sources[name] = {"seed": 7100, "sha256": sha(first), "engine": states[0]}
    print("IDENTICAL", name, states[0], flush=True)
(out / "SOURCES.json").write_text(json.dumps(sources, indent=2, sort_keys=True) + "\n")
(out / "SHA256SUMS").write_text(
    "".join(v["sha256"] + "  " + k + ".save\n" for k, v in sources.items())
)
(out / "DONE").write_text(f"all {len(cases)} engine fixtures generated twice identically\n")
