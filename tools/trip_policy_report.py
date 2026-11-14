#!/usr/bin/env python3
"""Summarize actual town stays and observed inventory changes.

Usage: python3 tools/trip_policy_report.py RUN TRIP_JSONL TOWN_STATE_JSONL
Capture with BORG_TRIP_RECORD and BORG_TOWN_STATE. RUN is a harness run.
Native state.log defines town boundaries; all joins use native game turns.
Trip reasons describe sampled policy decisions, not every cause of a return.
No descent-speed score or automatic useful/useless classification is made.
"""
import argparse
from collections import Counter
import json
from pathlib import Path
import re


def read_jsonl(path):
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def town_stays(lines):
    """Use the final depth at each native tick; report transient changes apart."""
    stays = []
    active = None
    previous_turn = -1
    positions = []
    same_turn_depth_changes = 0
    for index, line in enumerate(lines):
        match = re.match(r"^(\d+)\s+\S+\s+d(\d+)/", line)
        if not match:
            raise ValueError(f"unrecognized state.log line {index + 1}")
        turn, depth = map(int, match.groups())
        if turn < previous_turn:
            raise ValueError("native game clock went backwards")
        if positions and turn == previous_turn:
            same_turn_depth_changes += depth != positions[-1][1]
            positions[-1] = (turn, depth)
        else:
            positions.append((turn, depth))
        previous_turn = turn
    for index, (turn, depth) in enumerate(positions):
        if depth == 0:
            if active is None:
                active = {"arrival_game_turn": turn, "initial": index == 0}
        elif active is not None:
            active["departure_game_turn"] = turn
            stays.append(active)
            active = None
    if active is not None:
        active["departure_game_turn"] = None
        stays.append(active)
    return stays, same_turn_depth_changes


def inventory(record):
    # Integer town-state schema: [slot, tval, sval, qty, value, awareness].
    # Awareness bit 0 = aware; bit 1 = needs_ident. This is the dump's
    # observed uncertainty, not proof of an identification transaction.
    pack = [item for item in record["pack"] if item[3] > 0]
    unknown = [item for item in pack if not item[5] & 1 or item[5] & 2]
    gear = [item for item in unknown if 2 <= item[1] <= 24]
    traits = dict(record["traits"])
    return {
        "game_turn": record["turn"],
        "occupied_pack_entries": len(pack),
        "unknown_entries": len(unknown),
        "unknown_gear_entries": len(gear),
        "food": traits.get(39, 0),
        "recall": traits.get(38, 0),
        "gold": record["gold"],
    }, Counter((item[1], item[2], item[3], item[5]) for item in gear)


def summarize(state_lines, trips, town):
    for row in trips:
        if row.get("v") != 2 or "game_turn" not in row:
            raise ValueError("trip records need v2 with game_turn; Borg turns cannot join native turns")
    for row in town:
        if row.get("v") != 3 or row.get("depth") != 0:
            raise ValueError("expected BORG_TOWN_STATE native-clock v3 records")
    for rows, clock in [(trips, "game_turn"), (town, "turn")]:
        if any(a[clock] > b[clock] for a, b in zip(rows, rows[1:])):
            raise ValueError("record clock went backwards; use one fresh run per file")
    stays, same_turn_depth_changes = town_stays(state_lines)
    previous_departure = None
    result = []
    for stay in stays:
        arrival = stay["arrival_game_turn"]
        departure = stay["departure_game_turn"]
        snapshots = [r for r in town if r["turn"] >= arrival
                     and (departure is None or r["turn"] < departure)]
        # Retain return votes anywhere in the inbound dungeon segment.
        # The last vote can change during the ascent or after consuming kit.
        inbound = [r for r in trips if r["game_turn"] < arrival
                   and (previous_departure is None or r["game_turn"] >= previous_departure)]
        votes = [r for r in inbound if r["decision"]["action"] == "return"]
        item = dict(stay)
        item["inbound_game_turns"] = (
            arrival - previous_departure if previous_departure is not None else None)
        item["policy_return_reasons_observed"] = sorted({
            reason for row in votes for reason in row["decision"]["reasons"]})
        item["town_snapshot_count"] = len(snapshots)
        item["first_observed"] = None
        item["last_observed"] = None
        item["observed_delta"] = None
        item["unknown_gear_unchanged_between_snapshots"] = None
        if snapshots:
            first, first_gear = inventory(snapshots[0])
            last, last_gear = inventory(snapshots[-1])
            item["first_observed"] = first
            item["last_observed"] = last
            item["observed_delta"] = {
                key: last[key] - first[key] for key in first if key != "game_turn"}
            # Empty inventories are not evidence of an unresolved backlog.
            if len(snapshots) >= 2:
                item["unknown_gear_unchanged_between_snapshots"] = bool(first_gear) and first_gear == last_gear
        result.append(item)
        previous_departure = departure
    return {"v": 1, "town_stays": result,
            "returns_observed": sum(not row["initial"] for row in result),
            "same_turn_depth_changes": same_turn_depth_changes,
            "limitations": [
                "Town boundaries use final depth per native tick; same-tick excursions are not separate stays.",
                "Initial shopping is not a return; a final town stay may be incomplete.",
                "Snapshots bracket observed states, not exact transaction boundaries.",
                "Fewer unknown entries can mean identification, disposal, use or equipping.",
                "Net food, Recall and gold changes do not count individual transactions.",
                "Policy return votes are observations, not causal attribution."]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("run", type=Path)
    parser.add_argument("trip", type=Path)
    parser.add_argument("town", type=Path)
    args = parser.parse_args()
    if not (args.run / "summary.json").exists():
        parser.error("run has no summary.json")
    summary = json.loads((args.run / "summary.json").read_text())
    with (args.run / "state.log").open() as stream:
        report = summarize(stream, read_jsonl(args.trip), read_jsonl(args.town))
    report["run"] = {key: summary[key] for key in
                     ("seed", "race_idx", "class_idx", "game_turns", "stop_reason")}
    report["run"]["trip_settings"] = {
        key: value for key, value in summary["effective_settings"].items()
        if key.startswith("borg_trip_")}
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
