#!/usr/bin/env python3
"""Read native C Borg state at stop; require noninterference with a normal C run."""

import argparse, json, os, shutil, subprocess
from pathlib import Path

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument("generator", type=Path)
p.add_argument("normal_c", type=Path)
p.add_argument("output", type=Path)
p.add_argument("--suite", default="synthetic-tactics")
p.add_argument("--save-root", type=Path, default=root / "harness/saves/synthetic-tactics")
a = p.parse_args()
binary = a.generator.resolve()
out = a.output.resolve()
assert not out.exists()
out.mkdir(parents=True)
shutil.copy2(binary.parent / "generator.json", out / "generator.json")
rows = {}
for line in (root / "harness/suites" / a.suite).read_text().splitlines():
    fields = line.split("#")[0].split()
    if not fields:
        continue
    name, *arguments = fields
    run = out / name
    environment = {
        k: v for k, v in os.environ.items() if not k.startswith("BORG_SYNTHETIC_")
    }
    environment.update(
        BORG_IMPL="c",
        ANGBAND_BIN=str(binary),
        BORG_SYNTHETIC_OBSERVE="1",
        SAVE_ROOT=str(a.save_root.resolve()),
        RUN_TIMEOUT_SECONDS="15",
    )
    subprocess.run(
        [str(root / "harness/run.sh"), str(run), *arguments],
        env=environment,
        check=True,
        stdout=subprocess.DEVNULL,
    )
    for name_of_file in ("keys.log", "messages.log", "state.log", "final.save"):
        assert (run / name_of_file).read_bytes() == (
            a.normal_c / name / name_of_file
        ).read_bytes(), (name, name_of_file)
    states = [
        json.loads(l.removeprefix("SYNTH_STATE "))
        for l in (run / "stderr.log").read_text().splitlines()
        if l.startswith("SYNTH_STATE ")
    ]
    assert len(states) == 1
    rows[name] = {
        "native_c_state": states[0],
        "normal_c_keys_messages_state_save_identical": True,
    }
    trap_states = [json.loads(l.removeprefix("SYNTH_TRAPS "))
                   for l in (run / "stderr.log").read_text().splitlines()
                   if l.startswith("SYNTH_TRAPS ")]
    if trap_states:
        assert len(trap_states) == 1
        rows[name]["native_c_traps"] = trap_states[0]
(out / "observations.json").write_text(
    json.dumps(rows, indent=2, sort_keys=True) + "\n"
)
(out / "OBSERVATIONS_DONE").write_text(
    f"{len(rows)} read-only stop observations; normal C outputs unchanged\n"
)
print(
    f"PASS {len(rows)} read-only native C observations; all four normal-C byte streams unchanged"
)
