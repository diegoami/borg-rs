#!/usr/bin/env python3
"""Render a town-visit dump as a sheet the owner can label.

The dump itself is written by the Rust Borg (borg-rs/src/borg_town_dump.rs);
see docs/TOWN-DUMP.md. This reads the JSONL it writes and prints one block
per town visit: the state, the shop stock, the decision the current policy
took, and an empty label line for the owner to fill in.

Usage:
    tools/town_dump_report.py DUMP [--limit N] [--action A] [--distinct] [--full] [--markdown]

    --limit N   only the first N visits (default: all of them)
    --action A  only the visits whose decision is A (buy, sell, sell_home,
                take_home, visit)
    --distinct  only the first visit of each distinct decision. The Borg
                re-decides the same thing every turn while it walks to the
                shop, and only the first one carries the reason; the rest
                are the same decision repeated.
    --full      every shop's stock, not only the shop of the decision
    --markdown  Markdown (for an issue thread); the default is plain text
"""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('dump', type=Path)
parser.add_argument('--limit', type=int, default=0, help='only the first N visits (0 = all)')
parser.add_argument('--action', default='', help='only the visits with this decision action')
parser.add_argument('--distinct', action='store_true', help='only the first visit of each distinct decision')
parser.add_argument('--full', action='store_true', help='every shop\'s stock, not only the decision\'s shop')
parser.add_argument('--markdown', action='store_true', help='Markdown output for an issue thread')
args = parser.parse_args()


def read_records(path):
    """The dump's records, in order. A malformed line is reported, not fatal."""
    records = []
    for number, line in enumerate(path.read_text(errors='replace').splitlines(), start=1):
        line = line.strip()
        if not line:
            continue
        try:
            records.append(json.loads(line))
        except json.JSONDecodeError as err:
            print(f'{path}:{number}: {err}', file=__import__('sys').stderr)
    return records


def item_line(item):
    """One item as a single line."""
    desc = item.get('desc') or '(no description)'
    price = item.get('price', item.get('cost'))
    flags = []
    if item.get('needs_ident'):
        flags.append('unidentified')
    if item.get('aware'):
        flags.append('known')
    mark = f" [{', '.join(flags)}]" if flags else ''
    return f"{desc} (slot {item.get('slot')}, qty {item.get('qty')}, {price}{mark})"


def render(record, index, markdown):
    """One town visit."""
    if record.get('type') == 'purchase':
        ware = record.get('ware', {})
        line = (f"**Purchase {record.get('seq')}** - {ware.get('desc', '?')} "
                f"for {ware.get('price')} (gold {record.get('gold_before')})")
        return f'### {line}\n' if markdown else f'{line}\n'

    decision = record.get('decision', {})
    traits = record.get('traits', {})
    state = (f"turn {record.get('turn')}, {record.get('class')} level {record.get('level')}, "
             f"depth {record.get('depth')}, gold {record.get('gold')}, hp {record.get('hp')}")

    # The state and the stock are the context; the decision, the reason and
    # the label are what the owner reads. In Markdown the context is folded
    # away, so a batch of visits can be scanned in one screen.
    detail = []
    detail.append(f"traits: light {traits.get('light')}, food {traits.get('food')}, "
                  f"cut {traits.get('cut')}, poisoned {traits.get('poisoned')}")

    pack = record.get('pack') or []
    equip = record.get('equipment') or []
    home = record.get('home') or []
    detail.append(f"pack ({len(pack)}): " + ("; ".join(item_line(i) for i in pack) or "empty"))
    detail.append(f"equipment ({len(equip)}): " + ("; ".join(item_line(i) for i in equip) or "empty"))
    detail.append(f"home ({len(home)}): " + ("; ".join(item_line(i) for i in home) or "empty"))

    # The stock: the shop of the decision always, every shop with --full.
    shops = record.get('shops') or []
    chosen = decision.get('shop')
    for shop in shops:
        if not args.full and shop.get('shop') != chosen:
            continue
        wares = shop.get('wares') or []
        mark = ' (decision)' if shop.get('shop') == chosen else ''
        detail.append(f"shop {shop.get('shop')} {shop.get('name')}{mark} ({len(wares)} wares):")
        for ware in wares:
            detail.append(f"  - {item_line(ware)}")

    decision_line = (f"{decision.get('action')} {decision.get('ware_desc') or decision.get('item_desc') or ''}"
                     f" at {decision.get('shop_name')}")
    reason = decision.get('reason') or '(none recorded)'

    lines = []
    if markdown:
        lines.append(f"### Visit {record.get('seq')} - {state}")
        lines.append("")
        lines.append("<details><summary>state and shop stock</summary>")
        lines.append("")
        lines.extend(detail)
        lines.append("")
        lines.append("</details>")
        lines.append("")
        lines.append(f"**decision**: {decision_line}")
        lines.append(f"**reason**: {reason}")
        lines.append("**label**:")
    else:
        lines.append(f"Visit {record.get('seq')} - {state}")
        lines.extend(f"  {line}" for line in detail)
        lines.append(f"  decision: {decision_line}")
        lines.append(f"  reason: {reason}")
        lines.append("  label: " + ("_" * 60))
    lines.append("")
    return "\n".join(lines) + "\n"


records = read_records(args.dump)
decisions = [r for r in records if r.get('type') == 'decision']
if args.action:
    decisions = [r for r in decisions if r.get('decision', {}).get('action') == args.action]

# The Borg re-decides the same thing every turn while it walks to the shop.
# Only the first visit of a decision carries its reason, so the repeats are
# collapsed (and counted).
repeats = {}
kept = []
for record in decisions:
    decision = record.get('decision', {})
    key = (decision.get('action'), decision.get('shop'), decision.get('ware_desc'))
    if key in repeats:
        repeats[key] += 1
        continue
    repeats[key] = 1
    kept.append(record)

collapsed = len(decisions) - len(kept)
if args.distinct:
    decisions = kept
if args.limit:
    decisions = decisions[:args.limit]

out = []
for index, record in enumerate(decisions, start=1):
    out.append(render(record, index, args.markdown))

purchases = [r for r in records if r.get('type') == 'purchase']
if purchases and not args.limit:
    out.append('\n'.join(render(r, i, args.markdown) for i, r in enumerate(purchases, start=1)))

if collapsed and args.distinct:
    out.append(f'{collapsed} repeated decisions collapsed.\n')

print(''.join(out))
