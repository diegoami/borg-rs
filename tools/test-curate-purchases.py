#!/usr/bin/env python3
"""The tests for the purchase curation.

The curation's job is to find the purchases in a recorded session and to
survive a mistaken one. The tests are built from synthetic frames, so the
cases that matter (a bought item sold again, a stack bought twice, an item
used rather than bought) are stated exactly, and a real session is checked
against the game's own purchase messages.
"""

import importlib.util
import json
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def load():
    spec = importlib.util.spec_from_file_location(
        "curate", os.path.join(HERE, "curate-purchases.py")
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


curate = load()


def frame(index, turn, gold, pack, level=1, depth=0):
    """A frame with an inventory, as the recorder writes one."""
    return {
        "index": index,
        "turn": turn,
        "gold": gold,
        "level": level,
        "depth": depth,
        "inventory": {
            "pack": [
                {"label": chr(ord("a") + i), "description": d}
                for i, d in enumerate(pack)
            ],
            "equipment": [],
            "quiver": [],
        },
    }


def test_a_purchase_is_an_inventory_diff_with_a_gold_drop():
    frames = [
        frame(1, 10, 100, ["2 Rations of Food"]),
        frame(2, 10, 80, ["2 Rations of Food", "5 Flasks of oil"]),
    ]
    found = curate.purchases(frames)
    assert len(found) == 1, found
    assert found[0]["price"] == 20, found
    assert found[0]["items"] == ["5 Flasks of oil"], found


def test_using_an_item_is_not_a_purchase():
    """A potion drunk: the pack shrinks, the gold does not move."""
    frames = [
        frame(1, 10, 50, ["3 Potions of Cure Light Wounds"]),
        frame(2, 10, 50, ["2 Potions of Cure Light Wounds"]),
    ]
    assert curate.purchases(frames) == []


def test_buying_a_second_stack_of_the_same_item_is_a_purchase():
    """The pack does not grow, but the gold does drop: it is still a
    purchase, and the quantity says so."""
    frames = [
        frame(1, 10, 50, ["3 Potions of Cure Light Wounds"]),
        frame(2, 10, 30, ["5 Potions of Cure Light Wounds"]),
    ]
    found = curate.purchases(frames)
    assert len(found) == 1, found
    assert found[0]["price"] == 20, found
    assert found[0]["items"] == ["5 Potions of Cure Light Wounds"], found
    assert found[0]["pack"] == 1, found


def test_a_mistaken_purchase_sold_again_is_flagged_not_dropped():
    """The purchase commands are easy to get wrong, so a purchase that was
    immediately undone has to survive into the record - marked, so the
    consumer can drop it if it wants to."""
    frames = [
        frame(1, 10, 100, ["2 Rations of Food"]),
        # The mistaken buy: a shield, 36 gold.
        frame(2, 10, 64, ["2 Rations of Food", "a Small Metal Shield [5,+0]"]),
        # Sold again straight away: the gold comes back.
        frame(3, 10, 100, ["2 Rations of Food"]),
    ]
    found = curate.purchases(frames)
    assert len(found) == 1, found
    assert found[0]["price"] == 36, found
    assert "undone" in found[0], found
    assert found[0]["undone"]["items"] == ["a Small Metal Shield [5,+0]"], found


def test_frames_without_an_inventory_are_skipped():
    """The recorder only catches some frames with an inventory; the diff is
    taken between the two that have one."""
    frames = [
        frame(1, 10, 100, ["2 Rations of Food"]),
        {"index": 2, "turn": 10, "gold": 100, "inventory": None},
        frame(3, 10, 80, ["2 Rations of Food", "5 Flasks of oil"]),
    ]
    found = curate.purchases(frames)
    assert len(found) == 1, found
    assert found[0]["price"] == 20, found


def test_the_frame_zero_message_replay_is_not_a_purchase():
    """The game's message log is dumped at startup, and it holds purchases
    from an earlier session. The curation reads the frames, so the replay
    cannot be mistaken for this session's purchases."""
    frames = [
        frame(1, 1, 100, ["2 Rations of Food"]),
        frame(2, 1, 80, ["2 Rations of Food", "5 Flasks of oil"]),
    ]
    found = curate.purchases(frames)
    assert len(found) == 1, found
    assert found[0]["turn"] == 1


def test_the_real_session_matches_the_games_purchase_messages():
    """The end-to-end check: the purchases the curation finds are the
    purchases the game announced, at the same prices."""
    session = os.path.expanduser("~/angbsession/1")
    if not os.path.isdir(session):
        print("skipped: no session at ~/angbsession/1")
        return
    char_dir = curate.find_char_dir(session)
    frames = list(curate.read_jsonl(os.path.join(char_dir, "native", "frames.jsonl")))
    found = curate.purchases(frames)

    messages = list(
        curate.read_jsonl(os.path.join(char_dir, "native", "native-messages.jsonl"))
    )
    announced = []
    for message in messages:
        text = message.get("text", "")
        if "You bought" not in text:
            continue
        if message["frame"] == 0 and message["turn"] == 0:
            # The message log replay at startup: an earlier session.
            continue
        announced.append(text)

    assert len(found) == len(announced), (len(found), len(announced))
    # The prices: the game says "for N gold", the curation says the gold
    # dropped by N.
    import re

    prices = []
    for text in announced:
        match = re.search(r"for (\d+) gold", text)
        assert match, text
        prices.append(int(match.group(1)))
    assert [f["price"] for f in found] == prices, (
        [f["price"] for f in found],
        prices,
    )


def main():
    tests = [v for k, v in sorted(globals().items()) if k.startswith("test_")]
    failed = 0
    for test in tests:
        try:
            test()
            print(f"ok    {test.__name__}")
        except AssertionError as error:
            failed += 1
            print(f"FAIL  {test.__name__}: {error}")
    print(f"\n{len(tests) - failed}/{len(tests)} passed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
