#!/usr/bin/env python3
"""Validate the buy-to-identify runtime suite (borg-rs#27)."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# case -> (strategy, expected independent learning, expected on/off change).
# The reserve preempts in reserve410/Warrior, so that case must stay unchanged.
CASES = {
    "identify-current-warrior": (0, True, True),
    "identify-reserve-warrior": (1, False, False),
    "identify-current-mage": (0, True, True),
    "identify-reserve-mage": (1, True, True),
}
LEARNING_KINDS = (
    "Cure Light Wounds",
    "Cure Serious Wounds",
    "Remove Hunger",
    "Detect Invisible",
    "Monster Confusion",
)
# Kinds outside the #410 reserve. CLW/CSW are learned too, but the reserve can
# buy them, so they cannot prove the independent identify policy ran.
INDEPENDENT_KINDS = ("Remove Hunger", "Detect Invisible", "Monster Confusion")


def is_executed_purchase(line):
    """The real buy line is unquoted and ends with a period.

    In-shop goal notes quote the ware, e.g.
    `# Buying 'Potions of Cure Light Wounds {unseen}'(a) at ...`.
    """
    return "# Buying '" not in line


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, default=ROOT / "harness/out/family-identify", nargs="?")
    parser.add_argument("--baseline", type=Path, default=ROOT / "harness/out/family-buying")
    args = parser.parse_args()
    out = args.output.resolve()
    baseline = args.baseline.resolve()
    assert (out / "DONE").exists(), f"missing DONE in {out}"
    assert not (out / "FAILED").exists(), f"FAILED marker in {out}"

    independent_cases = 0
    for name, (strategy, expect_independent, expect_change) in CASES.items():
        case = out / name
        summary = json.loads((case / "summary.json").read_text())
        settings = summary["effective_settings"]
        assert settings["borg_buy_strategy_default"] == strategy, name
        assert settings["borg_buy_identify"] == 1, name
        assert summary["stop_reason"] != "stalled", (name, summary["stop_reason"])
        assert (case / "final.save").is_file(), name

        lines = (case / "messages.log").read_text().splitlines()
        buys = [line for line in lines if "# Buying" in line and is_executed_purchase(line)]

        for kind in LEARNING_KINDS:
            # One real (unseen) purchase per kind. Later copies are allowed only
            # after the engine marks the kind aware, so they carry no {unseen}.
            unseen = [line for line in buys if kind in line and "{unseen}" in line]
            assert len(unseen) <= 1, (name, kind, unseen)
            if unseen:
                bought_at = next(i for i, line in enumerate(lines) if line == unseen[0])
                # After the engine's completed sale, kind->aware must suppress
                # every later {unseen} goal for the same kind.
                later_unseen_goals = [
                    line
                    for line in lines[bought_at + 1:]
                    if "# Buying '" in line and kind in line and "{unseen}" in line
                ]
                assert not later_unseen_goals, (name, kind, later_unseen_goals)

        independent = any(
            kind in line and "{unseen}" in line
            for line in buys
            for kind in INDEPENDENT_KINDS
        )
        assert independent == expect_independent, (name, independent)
        if independent:
            independent_cases += 1

        on = (case / "keys.log").read_bytes()
        off = (baseline / name.replace("identify-", "family-") / "keys.log").read_bytes()
        assert (on != off) == expect_change, (name, on != off)

    assert independent_cases == 3, independent_cases

    print(
        "PASS: 4 buy-to-identify cases; effective settings, one unseen purchase "
        "per kind, no unseen goal after awareness, independent learning in 3 "
        "cases, and only the documented reserve preemption unchanged"
    )


if __name__ == "__main__":
    main()
