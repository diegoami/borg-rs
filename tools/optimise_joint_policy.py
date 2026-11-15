#!/usr/bin/env python3
"""Bounded joint parameter search on all 45 full-policy cases.

Primary objective: fewer deaths. Ties: greater mean final character level,
mean deepest level, then count reaching DL3+. Incomplete suites cannot rank.
Ten initial configurations, at most two measured combinations, then the two
best live configurations and control on 45 disjoint confirmation cases.
No settings are installed as defaults.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
BASE = {
    "borg_trip_policy": 1, "borg_town_plan": 1, "borg_town_no_stores": 0,
    "borg_buy_strategy_default": 0, "borg_buy_identify": 0,
    "borg_trip_depth_margin": 0, "borg_trip_pack_slack": 2,
    "borg_trip_unknown_stacks": 4, "borg_trip_return_gold": 0,
    "borg_town_target_depth": 0, "borg_town_trip_turns": 0,
    "borg_town_depth_gain": 1, "borg_town_w_gold": 25,
    "borg_town_w_bleeding": 100, "borg_town_w_knowledge": 25,
    "borg_town_risk": 1000,
}
PROBES = {
    "earlier-returns": {"borg_trip_pack_slack": 4, "borg_trip_unknown_stacks": 2},
    "shallower-trip": {"borg_trip_depth_margin": 2},
    "spend-more": {"borg_town_w_gold": 10},
    "retain-more-gold": {"borg_town_w_gold": 50},
    "prioritise-healing": {"borg_town_w_bleeding": 200},
    "no-learning-purchases": {"borg_town_w_knowledge": 0},
    "higher-risk-budget": {"borg_town_risk": 2000},
    "longer-supply-forecast": {"borg_town_trip_turns": 3000},
}
CONTROL = BASE | {"borg_trip_policy": 0, "borg_town_plan": 0}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def score(row):
    if not row["valid"]:
        return (float("inf"), 0, 0, 0)
    return (row["deaths"], -row["mean_character_level"], -row["mean_deepest_level"], -row["reached_dl3"])


def aggregate(cases):
    valid = len(cases) == 45 and all(c["valid"] for c in cases)
    summaries = [c["summary"] for c in cases if "summary" in c]
    return {"valid": valid, "completed": len(summaries),
            "invalid_cases": [c["name"] for c in cases if not c["valid"]],
            "deaths": sum(s["dead"] for s in summaries),
            "mean_character_level": statistics.mean(s["clevel"] for s in summaries) if summaries else 0,
            "mean_deepest_level": statistics.mean(s["max_depth"] for s in summaries) if summaries else 0,
            "reached_dl3": sum(s["max_depth"] >= 3 for s in summaries)}


def cases():
    result = []
    for line in (ROOT / "harness/suites/full-policy").read_text().splitlines():
        fields = line.split("#", 1)[0].split()
        if fields:
            name, seed, race, cls, turns, flags = fields
            assert flags == "menu"
            result.append((name, int(seed), int(race), int(cls), int(turns)))
    assert len(result) == 45
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--jobs", type=int, default=8)
    parser.add_argument("--c-baseline", type=Path)
    args = parser.parse_args()
    if not 1 <= args.jobs <= 12 or args.out.exists():
        parser.error("use 1..12 workers and a fresh output directory")
    args.out.mkdir(parents=True)
    corpus = cases()
    build = dict(line.split("=", 1) for line in (ROOT / "build/rust/BUILD_ID").read_text().splitlines() if "=" in line)
    source = subprocess.check_output(["bash", "harness/identity.sh", "source-hash", "rust"], cwd=ROOT, text=True).strip()
    assert source == build["source_sha256"]
    receipt = {"objective": ["min deaths", "max mean final character level", "max mean deepest level", "max DL3+ count"],
               "engine_milestone": "4.2.6-impact-v12", "candidate_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
               "suite_sha256": digest(ROOT / "harness/suites/full-policy"), "build": build,
               "parameter_probes": PROBES, "search": {}, "confirmation": {}}

    def save():
        (args.out / "results.json").write_text(json.dumps(receipt, indent=2) + "\n")
        lines = ["# Full-suite joint parameter search", "", "Fewer deaths first; averages include all 45 characters, including those that died.", ""]
        for phase in ("search", "confirmation"):
            lines += [f"## {phase.capitalize()}", "", "| Settings | Valid | Deaths | Mean character level | Mean deepest level | Reached DL3+ |", "|---|---:|---:|---:|---:|---:|"]
            for name, r in sorted(receipt[phase].items(), key=lambda kv: (score(kv[1]), kv[0])):
                metrics = f"{r['deaths']} | {r['mean_character_level']:.2f} | {r['mean_deepest_level']:.2f} | {r['reached_dl3']}" if r["valid"] else "— | — | — | —"
                lines.append(f"| {name} | {r['completed']}/45{' incomplete' if not r['valid'] else ''} | {metrics} |")
            lines.append("")
        (args.out / "results.md").write_text("\n".join(lines))

    def run_case(spec):
        phase, arm, settings, case, offset = spec
        name, seed, race, cls, turns = case
        seed += offset
        path = (args.out / phase / arm / name).resolve()
        path.parent.mkdir(parents=True, exist_ok=True)
        flags = "menu,key-limit=50000," + ",".join(f"{k}={v}" for k, v in settings.items())
        env = os.environ | {"BORG_IMPL": "rust", "ANGBAND_BIN": str(ROOT / "build/rust/game/angband"),
                            "BORG_OPTIONS_FILE": "", "BORG_TRIP_RECORD": "", "BORG_TOWN_PLAN_RECORD": "",
                            "BORG_TOWN_STATE": "", "BORG_TOWN_DUMP": "", "BORG_OBJECT_LOG": "",
                            "ALLOW_STALE_BUILD": "0", "ALLOW_UNKNOWN_IMPL": "0", "RUN_TIMEOUT_SECONDS": "180"}
        started = time.monotonic()
        p = subprocess.run(["bash", "harness/run.sh", str(path), str(seed), str(race), str(cls), str(turns), flags],
                           cwd=ROOT, env=env, capture_output=True, text=True)
        (path.parent / f"{name}-launcher.log").write_text(p.stdout + p.stderr)
        out = {"name": name, "seed": seed, "race_idx": race, "class_idx": cls,
               "valid": False, "returncode": p.returncode, "wall_seconds": time.monotonic() - started}
        if (path / "summary.json").exists():
            summary = json.loads((path / "summary.json").read_text())
            out["summary"] = {k: summary[k] for k in ("seed", "race_idx", "class_idx", "class", "race", "clevel", "max_depth", "dead", "death_cause", "stop_reason", "game_turns", "keys")}
            correct = summary["seed"] == seed and summary["race_idx"] == race and summary["class_idx"] == cls
            correct &= all(summary["effective_settings"][k] == v for k, v in settings.items())
            process = json.loads((path / "process.json").read_text())
            correct &= process["executable_sha256_before"] == build["binary_sha256"]
            out["valid"] = bool(p.returncode == 0 and correct and summary["stop_reason"] in ("turns", "death"))
            out["channels"] = {f: digest(path / f) for f in ("keys.log", "messages.log", "state.log")}
        return out

    def run_arms(phase, arms, offset=0):
        for arm, settings in arms.items():
            print(f"starting {phase}/{arm}: 45 cases", flush=True)
            results = []
            with ThreadPoolExecutor(max_workers=args.jobs) as pool:
                pending = [pool.submit(run_case, (phase, arm, settings, case, offset)) for case in corpus]
                for job in as_completed(pending):
                    results.append(job.result())
                    if len(results) % 15 == 0:
                        print(f"{phase}/{arm}: {len(results)}/45", flush=True)
            results.sort(key=lambda x: x["name"])
            row = aggregate(results) | {"settings": settings, "cases": results}
            receipt[phase][arm] = row
            save()
            print(f"completed {phase}/{arm}: {row['deaths']} deaths, CL {row['mean_character_level']:.2f}, DL {row['mean_deepest_level']:.2f}, DL3+ {row['reached_dl3']}, valid={row['valid']}", flush=True)

    save()
    run_arms("search", {"control": CONTROL})
    if args.c_baseline:
        assert receipt["search"]["control"]["valid"], "control suite incomplete"
        for name, *_ in corpus:
            for channel in ("keys.log", "messages.log", "state.log"):
                assert (args.out / "search/control" / name / channel).read_bytes() == (args.c_baseline / name / channel).read_bytes(), (name, channel, "C/Rust control mismatch")
        receipt["control_parity"] = "45/45 C/Rust keys/messages/state identical"
        save()
    run_arms("search", {"reference": BASE} | {name: BASE | delta for name, delta in PROBES.items()})
    ref = receipt["search"]["reference"]
    improving = sorted(((n, r) for n, r in receipt["search"].items() if n in PROBES and r["valid"] and score(r) < score(ref)), key=lambda kv: (score(kv[1]), kv[0]))
    # Resolve competing alternatives for a knob by the better individual
    # measured score. Measure interactions; never assume gains add together.
    compatible, occupied = [], set()
    for name, row in improving:
        delta = PROBES[name]
        if not occupied.intersection(delta):
            compatible.append((name, delta)); occupied.update(delta)
    combinations = {}
    seen = {json.dumps(r["settings"], sort_keys=True) for r in receipt["search"].values()}
    for name, count in [("combined-best-two", 2), ("combined-compatible", len(compatible))]:
        if len(compatible) < 2:
            break
        settings = dict(BASE)
        for _, delta in compatible[:count]:
            settings.update(delta)
        key = json.dumps(settings, sort_keys=True)
        if key not in seen:
            combinations[name] = settings; seen.add(key)
    receipt["combination_inputs"] = [name for name, _ in compatible]
    save()
    run_arms("search", combinations)
    ranked = sorted(((n, r) for n, r in receipt["search"].items() if n != "control" and r["valid"]), key=lambda kv: (score(kv[1]), kv[0]))
    assert ranked, "no complete live candidate"
    selected = dict((name, row["settings"]) for name, row in ranked[:2])
    receipt["selected_for_confirmation"] = list(selected)
    receipt["confirmation_seed_offset"] = 10000
    save()
    run_arms("confirmation", {"control": CONTROL} | selected, offset=10000)
    assert source == subprocess.check_output(["bash", "harness/identity.sh", "source-hash", "rust"], cwd=ROOT, text=True).strip(), "runtime sources changed during experiment"
    save()
    print("finished; results:", args.out / "results.md", flush=True)


if __name__ == "__main__":
    main()
