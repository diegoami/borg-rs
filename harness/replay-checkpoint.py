#!/usr/bin/env python3
"""Create a separate fresh-Borg replay matrix from a verified C capture.

Runs C twice and Rust once, then the unchanged strict three-log diff.
Never resumes a source trace, edits references, retries a failure or promotes
the source scenario. Output must be new. --turns is absolute engine time.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

from supervise import atomic_json, file_hash

ROOT = Path(__file__).resolve().parent.parent


def records(path):
    return dict(line.split("=", 1) for line in path.read_text().splitlines() if "=" in line)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_run", type=Path)
    parser.add_argument("checkpoint")
    parser.add_argument("output", type=Path)
    parser.add_argument("--turns", required=True, type=int)
    parser.add_argument("--seed", type=int)
    args = parser.parse_args()
    try:
        source = args.source_run.resolve()
        parent = source.parent
        if not ((parent / "DONE").exists() or (parent / "FAILED").exists()):
            raise ValueError("source suite is not finished; keep waiting for DONE or FAILED")
        manifest = records(parent / "manifest")
        if (manifest.get("binary_impl") != "c" or manifest.get("impl_check") != "ok"
                or manifest.get("build_check") != "ok" or manifest.get("overrides") != "none"):
            raise ValueError("source capture must come from a verified C suite without overrides")
        process = json.loads((source / "process.json").read_text())
        digest = manifest["binary_sha256"]
        if manifest.get("binary_sha256_end") != digest:
            raise ValueError("source suite binary changed during execution")
        if (process.get("status") in (None, "starting", "running", "supervisor_error")
                or process.get("executable_sha256_before") != digest
                or process.get("executable_sha256_after") != digest):
            raise ValueError("capture process is incomplete or its binary identity differs")
        captures = json.loads((source / "checkpoints.json").read_text())
        rows = [r for r in captures["captures"] if r["file"] == args.checkpoint]
        if len(rows) != 1 or not re.fullmatch(r"checkpoint-[0-9]+-t[0-9]+\.save", args.checkpoint):
            raise ValueError("select one indexed checkpoint filename")
        row = rows[0]
        save = source / "checkpoints" / args.checkpoint
        if save.is_symlink() or file_hash(save) != row["save_sha256"]:
            raise ValueError("capture SHA-256 differs from its completed index")
        if args.turns <= row["game_turn"]:
            raise ValueError("--turns must be after the captured engine turn")
        seed = args.seed if args.seed is not None else row["seed"]
        if not 0 <= seed <= 0xffffffff:
            raise ValueError("seed must be a uint32")
        flags = [f for f in records(source / "config.txt")["flags"].split(",")
                 if f and f != "-" and not f.startswith("save=") and f != "menu"]
        flags += ["menu", "save=" + args.checkpoint]
        out = args.output.resolve()
        out.mkdir(parents=True, exist_ok=False)
        fixtures = out / "fixtures"
        fixtures.mkdir()
        shutil.copyfile(save, fixtures / args.checkpoint)
        if file_hash(fixtures / args.checkpoint) != row["save_sha256"]:
            raise ValueError("checkpoint changed while copying")
        suite = out / "replay"
        suite.write_text("captured %d %d %d %d %s\n" % (
            seed, row["race_idx"], row["class_idx"], args.turns, ",".join(flags)))
        atomic_json(out / "SOURCE.json", {
            "source_run": str(source), "source_suite_failed": (parent / "FAILED").exists(),
            "source_process_status": process["status"], "capture": row,
            "source_manifest_sha256": file_hash(parent / "manifest"),
            "scenario": "fresh_borg_not_continuation", "replay_seed": seed,
            "absolute_turn_limit": args.turns,
        })
        env = os.environ.copy()
        env.update(SAVE_ROOT=str(fixtures), CHECKPOINT_EVERY="0", JOBS="1")
        # No ANGBAND_BIN override: each implementation gets its own verified
        # build. A caller's debug override must not substitute the wrong one.
        env.pop("ANGBAND_BIN", None)
        env.pop("ALLOW_STALE_BUILD", None)
        env.pop("ALLOW_UNKNOWN_IMPL", None)
        results = {}
        for label, impl in (("c1", "c"), ("c2", "c"), ("rust", "rust")):
            env["BORG_IMPL"] = impl
            command = [str(ROOT / "harness/suite.sh"), str(suite), str(out / label)]
            with (out / (label + ".log")).open("w") as log:
                results[label] = subprocess.run(command, env=env, stdout=log,
                                                stderr=subprocess.STDOUT).returncode
                if results[label] == 0:
                    summary = json.loads((out / label / "captured/summary.json").read_text())
                    if not summary.get("loaded") or summary.get("loaded_turn") != row["game_turn"]:
                        results[label] = 1
                        log.write("FAIL: loaded engine turn differs from capture metadata\n")
            atomic_json(out / "results.json", results)
        for label, a, b, impl in (("c_c", "c1", "c2", "c"), ("rust_c", "rust", "c1", "rust")):
            command = [str(ROOT / "harness/diff"), "--suite", str(suite),
                       "--save-root", str(fixtures), "--expect-impl", impl,
                       str(out / a), str(out / b)]
            with (out / (label + ".diff")).open("w") as log:
                results[label] = subprocess.run(command, stdout=log,
                                                stderr=subprocess.STDOUT).returncode
            atomic_json(out / "results.json", results)
        passed = all(rc == 0 for rc in results.values())
        (out / ("MATCHED" if passed else "FAILED")).write_text(
            "Separate fresh-Borg replay; source result is unchanged.\n")
        print(("MATCHED" if passed else "FAILED") + ": " + str(out))
        return 0 if passed else 1
    except (OSError, ValueError, KeyError, TypeError) as exc:
        print("checkpoint replay: " + str(exc), file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
