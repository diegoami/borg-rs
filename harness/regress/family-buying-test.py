#!/usr/bin/env python3
"""Validate the bounded Borg-family buying suite (borg-rs#25).

Run first with:
  BORG_IMPL=rust harness/suite.sh family-buying harness/out/family-buying
Then call this script with the output directory (that is its default).
"""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STREAMS = ("keys.log", "messages.log", "state.log", "final.save")
EXPECTED = {
    "family-current-warrior": {"borg_buy_strategy_default": 0},
    "family-reserve-warrior": {"borg_buy_strategy_default": 1},
    "family-reserve-warrior-selected": {
        "borg_buy_strategy_default": 0,
        "borg_buy_strategy_reserve410_munchkin_classes": 1,
    },
    "family-current-warrior-selected": {
        "borg_buy_strategy_default": 1,
        "borg_buy_strategy_current_munchkin_classes": 1,
    },
    "family-current-mage": {"borg_buy_strategy_default": 0},
    "family-reserve-mage": {"borg_buy_strategy_default": 1},
    "family-reserve-mage-selected": {
        "borg_buy_strategy_default": 0,
        "borg_buy_strategy_reserve410_classes": 2,
    },
    "family-current-mage-selected": {
        "borg_buy_strategy_default": 1,
        "borg_buy_strategy_current_classes": 2,
    },
}
EQUIVALENT = [
    ("family-current-warrior", "family-current-warrior-selected"),
    ("family-reserve-warrior", "family-reserve-warrior-selected"),
    ("family-current-mage", "family-current-mage-selected"),
    ("family-reserve-mage", "family-reserve-mage-selected"),
]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, default=ROOT / "harness/out/family-buying", nargs="?")
    args = parser.parse_args()
    out = args.output.resolve()
    assert (out / "DONE").exists(), f"missing DONE in {out}"
    assert not (out / "FAILED").exists(), f"FAILED marker in {out}"

    for name, expected in EXPECTED.items():
        case = out / name
        summary = json.loads((case / "summary.json").read_text())
        settings = summary["effective_settings"]
        for key, value in expected.items():
            assert settings[key] == value, (name, key, settings[key], value)
        assert summary["stop_reason"] != "stalled", (name, summary["stop_reason"])
        for stream in STREAMS:
            assert (case / stream).is_file(), (name, stream)

    for direct, selected in EQUIVALENT:
        for stream in STREAMS:
            a = (out / direct / stream).read_bytes()
            b = (out / selected / stream).read_bytes()
            assert a == b, (direct, selected, stream)

    # Both explicit variants must be reached, not merely accepted by the parser.
    for cls in ("warrior", "mage"):
        current = (out / f"family-current-{cls}/keys.log").read_bytes()
        reserve = (out / f"family-reserve-{cls}/keys.log").read_bytes()
        assert current != reserve, f"current and reserve410 are identical for {cls}"
        text = (out / f"family-reserve-{cls}/messages.log").read_text()
        phase = text.count("# Buying Scrolls of Phase Door")
        cures = text.count("# Buying Potions of Cure Light Wounds")
        assert phase >= 4, (cls, "Phase Door purchase anchors", phase)
        assert cures >= 3, (cls, "cure purchase anchors", cures)

    print(
        f"PASS: {len(EXPECTED)} completed cases; {len(EQUIVALENT)} direct/selected "
        "stream identities; Warrior and Mage reserve410 purchase anchors"
    )


if __name__ == "__main__":
    main()
