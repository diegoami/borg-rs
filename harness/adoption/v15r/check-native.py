#!/usr/bin/env python3
"""Run the donor's pinned native C fixtures on a borg-rs C build."""
import argparse
import os
import re
import subprocess
from pathlib import Path

DONOR_V15 = "646ebfc2a3a80a0caf87f1ab72fe4bcee0d94343"
HEAL_FIXTURE = "f88fee1e3ace2b5f34f6fc8a1b4fbc30d3f94da1"
SLAY_FIXTURE = "25b0118631f108edc926856c8f4b3c35d3611bf9"

# Fixture, source commit, environment prefix, regression mode(s), control.
FIXTURES = (
    ("retreatadj-154", DONOR_V15, "RA154", ("spider",), "ant"),
    ("deepdescent-590", DONOR_V15, "DD590", ("dd-phase",), "control"),
    ("mindblast-591", DONOR_V15, "MB591", ("nar",), "control"),
    ("rubblemark-192", DONOR_V15, "M192", ("dark",), "control"),
    ("resielec-174", DONOR_V15, "R174", ("elec",), "control"),
    ("tportcheck-593", DONOR_V15, "TC593", ("scrolls",), "control"),
    ("healchance-364", HEAL_FIXTURE, "HC364", ("danger", "unique", "risky"), "control"),
    ("slay-239", SLAY_FIXTURE, "S239", ("animal",), "nonanimal"),
)


def run_fixtures(source, build, output, before):
    cache = dict(
        line.split("=", 1) for line in (build / "CMakeCache.txt").read_text().splitlines()
        if line and line[0] not in "#/" and "=" in line
    )
    if cache.get("BORG_IMPL:STRING") != "c":
        raise ValueError(f"Expected a C Borg build at {build}")
    engine = Path(cache["CMAKE_HOME_DIRECTORY:INTERNAL"]) / "src"
    objects = sorted((build / "CMakeFiles/OurCoreLib.dir").rglob("*.o"))
    helpers = sorted((build / "CMakeFiles/OurUnitTestLib.dir").rglob("*.o"))
    if not objects or not helpers:
        raise ValueError(f"Build the engine and allunittests before running against {build}")

    failures = []
    for name, commit, env_prefix, regression_modes, control in FIXTURES:
        work = output / name
        profile = work / "profile"
        profile.mkdir(parents=True, exist_ok=False)
        profile_source = "profile/borg.txt" if commit == DONOR_V15 else "borg.txt"
        for original, local in (("scenario.c", work / "scenario.c"),
                                (profile_source, profile / "borg.txt")):
            ref = f"{commit}:docs/audit/c-reports/{name}/{original}"
            local.write_bytes(subprocess.check_output(["git", "-C", source, "show", ref]))

        binary = work / "scenario"
        includes = ["-include", "obj-tval.h"] if name == "healchance-364" else []
        subprocess.run([
            "cc", "-DALLOW_BORG", "-I", engine, "-I", engine / "tests", *includes,
            work / "scenario.c", *objects, *helpers, "-lm", "-o", binary,
        ], check=True)

        for mode in (*regression_modes, control):
            env = dict(os.environ, LC_ALL="C.UTF-8")
            env[f"{env_prefix}_MODE"] = mode
            env[f"{env_prefix}_PROFILE"] = str(profile)
            run = subprocess.run(
                [binary, "-v"], cwd=build / "game", env=env, text=True,
                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60,
            )
            log = re.sub(r"\x1b\[[0-9;]*m", "", run.stdout)
            (work / f"{mode}.log").write_text(log)
            expected = 0 if before and commit == DONOR_V15 and mode != control else 1
            matched = (run.returncode == 0
                       and f"native-c/{name} finished: {expected}/1 passed" in log)
            if not matched:
                failures.append(f"{name}/{mode}")
            print(f"{name}/{mode} {'OK' if matched else 'FAILED'}")

    if failures:
        raise SystemExit("Unexpected fixture results: " + ", ".join(failures))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True, help="donor Git checkout with fixture commits")
    parser.add_argument("--build", type=Path, required=True, help="completed native C build")
    parser.add_argument("--output", type=Path, required=True, help="new directory for fixture inputs and logs")
    parser.add_argument("--before", action="store_true", help="expect v14 regressions on the v13R baseline")
    args = parser.parse_args()
    run_fixtures(args.source.resolve(), args.build.resolve(), args.output.resolve(), args.before)


if __name__ == "__main__":
    main()
