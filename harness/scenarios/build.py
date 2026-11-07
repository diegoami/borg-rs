#!/usr/bin/env python3
"""Relink unchanged native C engine/Borg objects with a fixture-only save hook."""

import argparse, hashlib, json, shlex, subprocess
from pathlib import Path

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument("output", type=Path)
a = p.parse_args()
out = a.output.resolve()
assert not out.exists(), "refusing existing build directory"
build = root / "build/c"
bid = dict(l.split("=", 1) for l in (build / "BUILD_ID").read_text().splitlines())
sha = lambda f: hashlib.sha256(f.read_bytes()).hexdigest()
assert bid["impl"] == "c" and sha(build / "game/angband") == bid["binary_sha256"]
assert (
    subprocess.check_output(
        [str(root / "harness/identity.sh"), "source-hash", "c"], text=True
    ).strip()
    == bid["source_sha256"]
), "stale C build"
out.mkdir(parents=True)
source = root / "harness/scenarios/fixture-save.c"
snapshot_source = out / "fixture-save.c"
snapshot_source.write_bytes(source.read_bytes())
obj = out / "fixture-save.o"
compile_args = [
    "cc",
    "-O2",
    "-DNDEBUG",
    "-DALLOW_BORG",
    "-DBORG_HARNESS",
    "-I" + str(root / "angband/src"),
    "-c",
    str(snapshot_source),
    "-o",
    str(obj),
]
subprocess.run(compile_args, check=True)
args = [
    v
    for v in shlex.split((build / "CMakeFiles/OurExecutable.dir/link.txt").read_text())
    if not v.startswith("-Wl,--dependency-file=")
]
args[args.index("-o") + 1] = str(out / "angband")
objects = [(build / v).resolve() for v in args if v.endswith(".o")]
before = {str(f): sha(f) for f in objects}
args.extend([str(obj), "-Wl,--wrap=savefile_save", "-Wl,--wrap=savefile_load"])
subprocess.run(args, cwd=build, check=True)
assert before == {str(f): sha(f) for f in objects}, "production object changed"
assert sha(build / "game/angband") == bid["binary_sha256"], "production binary changed"
assert (
    subprocess.check_output(
        [str(root / "harness/identity.sh"), "binary-impl", str(out / "angband")],
        text=True,
    ).strip()
    == "c"
)
(out / "lib").symlink_to(build / "game/lib", target_is_directory=True)
meta = {
    "test_only": True,
    "base_build_id": bid,
    "constructor_source_sha256": sha(snapshot_source),
    "compile_argv": compile_args,
    "link_argv": args,
    "objects_unchanged": before,
    "binary_sha256": sha(out / "angband"),
}
(out / "generator.json").write_text(json.dumps(meta, indent=2, sort_keys=True) + "\n")
print(out / "angband")
