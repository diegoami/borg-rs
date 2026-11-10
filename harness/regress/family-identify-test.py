#!/usr/bin/env python3
"""Validate the buy-to-identify runtime suite (borg-rs#27)."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CASES = {
    "identify-current-warrior": 0,
    "identify-reserve-warrior": 1,
    "identify-current-mage": 0,
    "identify-reserve-mage": 1,
}
LEARNING_KINDS = (
    "Cure Light Wounds",
    "Cure Serious Wounds",
    "Remove Hunger",
    "Detect Invisible",
    "Monster Confusion",
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, default=ROOT / "harness/out/family-identify", nargs="?")
    parser.add_argument("--baseline", type=Path, default=ROOT / "harness/out/family-buying")
    args = parser.parse_args()
    out = args.output.resolve()
    baseline = args.baseline.resolve()
    assert (out / "DONE").exists(), f"missing DONE in {out}"
    assert not (out / "FAILED").exists(), f"FAILED marker in {out}"

    for name, strategy in CASES.items():
        case = out / name
        summary = json.loads((case / "summary.json").read_text())
        settings = summary["effective_settings"]
        assert settings["borg_buy_strategy_default"] == strategy, name
        assert settings["borg_buy_identify"] == 1, name
        assert summary["stop_reason"] != "stalled", (name, summary["stop_reason"])
        assert (case / "final.save").is_file(), name
        text = (case / "messages.log").read_text()
        lines = text.splitlines()
        purchases = [line for line in lines if "# Buying" in line]
        bought = [kind for kind in LEARNING_KINDS if any(kind in line for line in purchases)]
        assert bought, f"no learning-list purchase in {name}"
        for kind in LEARNING_KINDS:
            # Goal selection may name the same unaware kind more than once
            # before the first completed sale. After the engine's first real
            # `You bought` event, kind->aware must suppress later unseen goals.
            actual = next((i for i, line in enumerate(lines) if "You bought" in line and kind in line), None)
            if actual is not None:
                assert any(kind in line for line in lines[actual:]), (name, kind)
                later_unseen = [line for line in lines[actual + 1:] if "# Buying" in line and kind in line and "{unseen}" in line]
                assert not later_unseen, (name, kind, later_unseen)

    changed = []
    for name in CASES:
        baseline_name = name.replace("identify-", "family-")
        on = (out / name / "keys.log").read_bytes()
        off = (baseline / baseline_name / "keys.log").read_bytes()
        if on != off:
            changed.append(name)
    assert changed, "identify changed no strategy case"
    independent = [
        name for name in CASES
        if any(
            "# Buying" in line and kind in line
            for line in (out / name / "messages.log").read_text().splitlines()
            for kind in ("Detect Invisible", "Monster Confusion", "Remove Hunger")
        )
    ]
    assert independent, "no independent (non-reserve) learning anchor"

    print(
        f"PASS: 4 buy-to-identify cases; on/off changed {len(changed)}/4; "
        f"independent learning anchors in {len(independent)} cases; no repeated flavour learning"
    )


if __name__ == "__main__":
    main()
