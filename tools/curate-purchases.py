#!/usr/bin/env python3
"""Curate a recorded session into a purchase dataset.

The input is a session directory (the one `harness/play.sh` writes):

    SESSION/char-N/native/frames.jsonl      the frames, with the inventory
    SESSION/char-N/native/native-messages.jsonl   the messages, with their frames

The output is one JSON object per purchase:

    {"turn": 111, "gold": 262, "items": ["3 Potions of Cure Light Wounds"],
     "pack": 5, "price": 60}

**Where the purchases come from.** Not from the messages. The frames carry the
inventory, and a purchase is an inventory diff in which the gold drops. The
messages are only consulted for the frame-0 replay, which is the game's
message log dumped at startup and holds purchases from an *earlier* session;
those are not this session's purchases and are dropped.

The gold is the discriminator: a purchase spends gold, and a use does not.
Looking at the pack alone would count every potion drunk and every torch
burned as a purchase; with the gold it is exact.

**Noise.** The purchase commands are not friendly and it is easy to buy
something by mistake, so the record has to survive a mistaken purchase. Two
things are done about it:

  * a purchase that was immediately undone - sold again, or dropped - is
    recorded, and marked, rather than silently dropped;
  * a purchase of something the character already had a stack of is
    recorded with the quantity, so a duplicate stack is visible as a
    quantity change rather than a new item.

Whether a purchase was a mistake is not something the recording can say: the
recording has the outcome, not the intent. So nothing is filtered. The
consumer decides.
"""

import argparse
import json
import os
import sys
from collections import Counter


def read_jsonl(path):
    with open(path, encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if line:
                yield json.loads(line)


def pack_bag(frame):
    """The pack of a frame, as a count of the item descriptions."""
    inventory = frame.get("inventory") or {}
    return Counter(item["description"] for item in inventory.get("pack", []))


def find_char_dir(session):
    """The character directory of a session: `char-1`, `char-2`, ..."""
    if os.path.isdir(os.path.join(session, "char-1")):
        return os.path.join(session, "char-1")
    names = sorted(n for n in os.listdir(session) if n.startswith("char-"))
    if not names:
        raise SystemExit(f"no character directory in {session}")
    return os.path.join(session, names[0])


def purchases(frames):
    """The purchases of a session, in order.

    A purchase is an inventory diff in which the gold drops. The frames
    that carry an inventory are the ones the recorder caught with one; the
    ones without are skipped, so a diff is taken between the two frames
    that do have one.

    A purchase that was undone - sold again, or dropped, with the gold
    going back up - is kept and marked with `undone`. It is not filtered:
    the recording has the outcome, not the intent, so the consumer decides.
    """
    with_inventory = [f for f in frames if f.get("inventory")]
    found = []
    previous = None
    for frame in with_inventory:
        if previous is not None:
            added = pack_bag(frame) - pack_bag(previous)
            spent = (previous.get("gold") or 0) - (frame.get("gold") or 0)
            if added and spent > 0:
                found.append(
                    {
                        "frame": frame["index"],
                        "turn": frame["turn"],
                        "gold": frame.get("gold"),
                        "price": spent,
                        "items": sorted(added.elements()),
                        "pack": len(frame["inventory"].get("pack", [])),
                        "level": frame.get("level"),
                        "depth": frame.get("depth"),
                    }
                )
        previous = frame
    mark_undone(with_inventory, found)
    return found


def mark_undone(with_inventory, bought):
    """Mark the purchases that were undone: sold again, or dropped, with the
    gold going back up.

    The match is by frame order, not by turn: the sale happens a frame or
    two after the purchase, and they share a turn (store commands do not
    consume turns).
    """
    sales = []
    previous = None
    for frame in with_inventory:
        if previous is not None:
            removed = pack_bag(previous) - pack_bag(frame)
            gained = (frame.get("gold") or 0) - (previous.get("gold") or 0)
            if removed and gained > 0:
                sales.append(
                    {
                        "frame": frame["index"],
                        "turn": frame["turn"],
                        "items": sorted(removed.elements()),
                        "gold_back": gained,
                    }
                )
        previous = frame

    for sale in sales:
        for bought_at in bought:
            if bought_at["frame"] >= sale["frame"]:
                continue
            shared = set(bought_at["items"]) & set(sale["items"])
            if shared:
                bought_at["undone"] = {
                    "frame": sale["frame"],
                    "items": sorted(shared),
                    "gold_back": sale["gold_back"],
                }
    return sales


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("session", help="the session directory")
    parser.add_argument(
        "-o", "--out", help="write the purchases here (default: stdout)"
    )
    parser.add_argument(
        "--format",
        choices=("json", "jsonl", "text"),
        default="json",
        help="the output format (default: json)",
    )
    args = parser.parse_args()

    char_dir = find_char_dir(args.session)
    frames_path = os.path.join(char_dir, "native", "frames.jsonl")
    if not os.path.exists(frames_path):
        raise SystemExit(f"no frames in {char_dir}: {frames_path}")

    frames = list(read_jsonl(frames_path))
    bought = purchases(frames)

    if args.format == "json":
        payload = {"session": os.path.abspath(args.session), "purchases": bought}
        text = json.dumps(payload, indent=2)
    elif args.format == "jsonl":
        text = "\n".join(json.dumps(b) for b in bought)
    else:
        lines = []
        for b in bought:
            flag = "  UNDONE" if b.get("undone") else ""
            lines.append(
                f"turn {b['turn']:>6}  price {b['price']:>4}  pack {b['pack']:>2}  "
                f"{', '.join(b['items'])}{flag}"
            )
        text = "\n".join(lines)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as handle:
            handle.write(text + "\n")
        print(f"{len(bought)} purchases -> {args.out}", file=sys.stderr)
    else:
        print(text)


if __name__ == "__main__":
    main()
