#!/usr/bin/env python3
"""Curate the recorded sessions into a purchase dataset.

The input is a corpus root - a directory holding one directory per recorded
session. Each session is one `harness/play.sh` run:

    CORPUS/1/char-N/native/frames.jsonl      the frames, with the inventory
    CORPUS/2/char-N/native/frames.jsonl      and so on

The corpus root is found by looking for the sessions: a directory that holds
a `char-N` directory *is* a session, and a directory that holds sessions
*is* the corpus. So both of these work:

    python3 tools/curate-purchases.py ~/angbsession     # the whole corpus
    python3 tools/curate-purchases.py ~/angbsession/1   # one session

The output is one JSON object per purchase:

    {"session": "1", "char": 1, "turn": 111, "gold": 262,
     "items": ["3 Potions of Cure Light Wounds"], "pack": 5, "price": 60}

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


def is_session(path):
    """Whether a directory is a session: it holds a `char-N` directory."""
    if not os.path.isdir(path):
        return False
    for name in os.listdir(path):
        if name.startswith("char-") and os.path.isdir(os.path.join(path, name)):
            return True
    return False


def find_sessions(root):
    """The sessions under a root.

    A root that *is* a session gives that one session; a root that holds
    sessions gives them all, sorted by name. The session's name is its
    directory name relative to the root, so a corpus written as
    `~/angbsession/1/char-1` reports the session as `1`.
    """
    if is_session(root):
        return [(os.path.basename(os.path.normpath(root)), root)]
    sessions = []
    for name in sorted(os.listdir(root)):
        path = os.path.join(root, name)
        if is_session(path):
            sessions.append((name, path))
    return sessions


def char_dirs(session):
    """The character directories of a session: `char-1`, `char-2`, ..."""
    names = sorted(
        n for n in os.listdir(session) if n.startswith("char-")
        and os.path.isdir(os.path.join(session, n))
    )
    if not names:
        raise SystemExit(f"no character directory in {session}")
    return [(n, os.path.join(session, n)) for n in names]


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


def curate(root):
    """Every purchase of every session under a root."""
    sessions = find_sessions(root)
    if not sessions:
        raise SystemExit(f"no sessions under {root}")
    all_purchases = []
    for session_name, session_path in sessions:
        for char_name, char_path in char_dirs(session_path):
            frames_path = os.path.join(char_path, "native", "frames.jsonl")
            if not os.path.exists(frames_path):
                print(
                    f"curate: skipping {session_name}/{char_name}: no frames",
                    file=sys.stderr,
                )
                continue
            frames = list(read_jsonl(frames_path))
            for bought in purchases(frames):
                bought["session"] = session_name
                bought["char"] = char_name
                all_purchases.append(bought)
    return all_purchases


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("root", help="the corpus root, or one session")
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

    root = os.path.expanduser(args.root)
    if not os.path.isdir(root):
        raise SystemExit(f"no such directory: {root}")

    bought = curate(root)

    if args.format == "json":
        payload = {"root": os.path.abspath(root), "purchases": bought}
        text = json.dumps(payload, indent=2)
    elif args.format == "jsonl":
        text = "\n".join(json.dumps(b) for b in bought)
    else:
        lines = []
        for b in bought:
            flag = "  UNDONE" if b.get("undone") else ""
            lines.append(
                f"{b['session']}/{b['char']} turn {b['turn']:>6}  "
                f"price {b['price']:>4}  pack {b['pack']:>2}  "
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
