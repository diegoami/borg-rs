#!/usr/bin/env python3
"""Deserialize a save with the native engine and report state before Borg entry."""

import argparse, json, os, subprocess, tempfile
from pathlib import Path

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("generator", type=Path)
p.add_argument("saves", nargs="+", type=Path)
a = p.parse_args()
binary = a.generator.resolve()
with tempfile.TemporaryDirectory(prefix="borg-scenario-inspect-") as directory:
    for save in a.saves:
        proc = subprocess.run(
            [
                str(binary),
                "-mborgh",
                "-duser=" + directory,
                "--",
                "-seed",
                "7100",
                "-load",
                str(save.resolve()),
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
        lines = [
            l.removeprefix("SYNTH_STATE ")
            for l in proc.stderr.splitlines()
            if l.startswith("SYNTH_STATE ")
        ]
        assert len(lines) == 1, proc.stderr
        print(
            json.dumps(
                {"save": str(save), "engine": json.loads(lines[0])}, sort_keys=True
            )
        )
