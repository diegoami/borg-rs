#!/usr/bin/env python3
"""Bounded live trip/inventory trials; descriptive outcomes, no speed score.

Print settings by default; --run executes six arms on three fixed cases.
Raw evidence stays in the required --out directory, outside Git.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
from collections import Counter
import json
import os
from pathlib import Path
import re
import subprocess

from trip_policy_report import read_jsonl, summarize

ROOT = Path(__file__).resolve().parents[1]
BASE = {
    "borg_town_plan": 1, "borg_trip_policy": 1, "borg_town_no_stores": 0,
    "borg_trip_depth_margin": 0, "borg_trip_pack_slack": 2,
    "borg_trip_unknown_stacks": 4, "borg_trip_return_gold": 0,
    "borg_town_target_depth": 0, "borg_town_depth_gain": 1,
    "borg_town_trip_turns": 0, "borg_town_risk": 1000,
    "borg_town_w_gold": 25, "borg_town_w_knowledge": 25,
}
ARMS = {
    "old-control": {"borg_town_plan": 0, "borg_trip_policy": 0},
    "integrated-reference": {},
    "early-return": {"borg_trip_depth_margin": 2, "borg_trip_pack_slack": 4, "borg_trip_unknown_stacks": 2},
    "cash-retaining": {"borg_town_w_gold": 50},
    "learning-priority": {"borg_trip_unknown_stacks": 1, "borg_town_w_knowledge": 50},
    "supply-cautious": {"borg_trip_depth_margin": 2, "borg_trip_pack_slack": 3,
                        "borg_trip_unknown_stacks": 2, "borg_town_trip_turns": 5000,
                        "borg_town_risk": 1500},
}
CASE_SETS = {
    "exploratory": [(101, 0), (301, 1), (401, 3)],
    "holdout": [(202, 0), (302, 1), (402, 3)],
}


def completed_transactions(path):
    """Pair executor notes with native UI confirmations, within 20 keys.

    A selection or '# Buying' note alone is not a completed purchase.
    Counts are transactions, not distinct kinds or quantities.
    """
    counts = Counter()
    pending = None
    spend = 0
    for line in path.read_text().splitlines():
        parts = line.split(maxsplit=3)
        if len(parts) != 4:
            continue
        key, text = int(parts[0]), parts[3]
        match = re.search(r"# Town plan executing action (\d+)", text)
        if match:
            pending = (int(match[1]), key)
        elif text.startswith("# Town plan dropping"):
            pending = (6, key)
        purchase = re.search(r"^You bought .+ for (\d+) gold\.", text)
        if purchase:
            counts["native_purchases"] += 1
            spend += int(purchase[1])
        if pending is None:
            continue
        code, began = pending
        if key - began > 20:
            pending = None
            continue
        confirmed = None
        if code == 1 and purchase:
            confirmed = "planner_purchases"
        elif code == 3 and text.startswith("You had "):
            confirmed = "planner_sales"
        elif code == 5 and text.startswith("You drop "):
            confirmed = "planner_home_deposits"
        elif code in (2, 4) and text.startswith("You have "):
            confirmed = "planner_home_withdrawals"
        elif code == 6 and text.startswith("You drop "):
            confirmed = "planner_drops"
        if confirmed:
            counts[confirmed] += 1
            pending = None
    return dict(counts) | {"native_purchase_gold": spend}


def inspect_run(path, settings):
    summary = json.loads((path / "summary.json").read_text())
    assert all(summary["effective_settings"][k] == v for k, v in settings.items())
    trips = read_jsonl(path / "trip.jsonl") if (path / "trip.jsonl").exists() else []
    town = read_jsonl(path / "town.jsonl") if (path / "town.jsonl").exists() else []
    with (path / "state.log").open() as stream:
        visits = summarize(stream, trips, town)
    (path / "town-outcomes.json").write_text(json.dumps(visits, indent=2) + "\n")
    records = read_jsonl(path / "plan.jsonl") if (path / "plan.jsonl").exists() else []
    for record in records:
        assert record["mode"] == settings["borg_town_plan"]
        available = record["limits"][0] + sum(k[8] for k, d in zip(record["kit"], record["dispose"]) if d == 2)
        assert 0 <= sum(entry[4] for entry in record["plan"]) <= available
    later = [v for v in visits["town_stays"] if not v["initial"]]
    observed = [v for v in later if v["observed_delta"] is not None]
    return {
        "settings": settings, "seed": summary["seed"], "class": summary["class"],
        "stop_reason": summary["stop_reason"], "dead": summary["dead"],
        "death_cause": summary.get("death_cause"), "game_turns": summary["game_turns"],
        "plan_records": len(records), "native_clock_returns": len(later),
        "returns_with_inventory_snapshots": len(observed),
        "returns_reducing_unknown_entries": sum(v["observed_delta"]["unknown_entries"] < 0 for v in observed),
        "returns_reducing_combined_unknown_gear": sum(v["observed_delta"]["unknown_gear_entries_total"] < 0 for v in observed),
        "returns_reducing_pack_entries": sum(v["observed_delta"]["occupied_pack_entries"] < 0 for v in observed),
        "unchanged_unknown_gear_visits": sum(v["unknown_gear_unchanged_between_snapshots"] is True for v in observed),
        "last_town_food_capability": dict(town[-1]["traits"]).get(39, 0) if town else None,
        "transactions": completed_transactions(path / "messages.log"),
        "final_town_snapshot": town[-1] if town else None,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--arm", action="append", choices=ARMS)
    parser.add_argument("--turns", type=int, default=100000)
    parser.add_argument("--jobs", type=int, default=3)
    parser.add_argument("--case-set", choices=CASE_SETS, default="exploratory")
    args = parser.parse_args()
    if not 1 <= args.turns <= 100000 or not 1 <= args.jobs <= 4:
        parser.error("bounded trials require 1..100000 turns and 1..4 jobs")
    arms = args.arm or list(ARMS)
    cases = CASE_SETS[args.case_set]
    specs = [(arm, seed, cls, BASE | ARMS[arm]) for arm in dict.fromkeys(arms) for seed, cls in cases]
    if not args.run:
        print(json.dumps({"arms": {a: BASE | ARMS[a] for a in arms}, "cases": cases, "turns": args.turns}, indent=2))
        return
    if args.out.exists():
        parser.error("use a fresh evidence directory")
    args.out.mkdir(parents=True)

    def run(spec):
        arm, seed, cls, settings = spec
        path = (args.out / arm / str(seed)).resolve()
        path.parent.mkdir(parents=True, exist_ok=True)
        flags = ",".join(f"{k}={v}" for k, v in settings.items()) + ",key-limit=50000"
        env = os.environ | {"BORG_IMPL": "rust", "RUN_TIMEOUT_SECONDS": "180",
                            "BORG_TOWN_PLAN_RECORD": str(path / "plan.jsonl"),
                            "BORG_TRIP_RECORD": str(path / "trip.jsonl"),
                            "BORG_TOWN_STATE": str(path / "town.jsonl")}
        print(f"running {arm} seed {seed}", flush=True)
        proc = subprocess.run(["bash", "harness/run.sh", str(path), str(seed), "0", str(cls), str(args.turns), flags],
                              cwd=ROOT, env=env, capture_output=True, text=True)
        (path.parent / f"{seed}-launcher.log").write_text(proc.stdout + proc.stderr)
        if proc.returncode:
            return f"{arm}/{seed}", {"error": "harness failed", "returncode": proc.returncode}
        (path / "trial-parameters.json").write_text(json.dumps(settings, indent=2) + "\n")
        result = inspect_run(path, settings)
        print(f"completed {arm} seed {seed}: {result['stop_reason']}, transactions {result['transactions']}", flush=True)
        return f"{arm}/{seed}", result

    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        runs = dict(pool.map(run, specs))
    report = {"case_set": args.case_set, "scope": "Exploratory live combinations, not independent lives, parameter acceptance or causal comparisons.",
              "criteria": "Useful returns, inventory resolution, supply coverage and actual purchases/dispositions; no descent-speed score.",
              "runs": runs}
    (args.out / "trials-report.json").write_text(json.dumps(report, indent=2) + "\n")
    if any("error" in r for r in runs.values()):
        raise SystemExit("one or more trials failed; see report and launcher logs")


if __name__ == "__main__":
    main()
