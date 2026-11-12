#!/usr/bin/env python3
"""Render a town-visit dump as a readable sheet.

The dump itself is written by the Rust Borg (borg-rs/src/borg_town_dump.rs);
see docs/TOWN-DUMP.md. This reads the JSONL it writes and prints one block
per town visit: the state, the shop stock, and the decision the current
policy took.

Usage:
    tools/town_dump_report.py DUMP [--limit N] [--action A] [--distinct] [--full] [--markdown]
    tools/town_dump_report.py --integers DUMP [--limit N] [--markdown]

    --limit N   only the first N visits (default: all of them)
    --action A  only the visits whose decision is A (buy, sell, sell_home,
                take_home, visit)
    --distinct  only the first visit of each distinct decision. The Borg
                re-decides the same thing every turn while it walks to the
                shop, and only the first one carries the reason; the rest
                are the same decision repeated.
    --full      every shop's stock, not only the shop of the decision
    --markdown  Markdown (for an issue thread); the default is plain text
    --integers  the file is the integer projection (BORG_TOWN_DUMP_INTEGERS):
                every value is an integer and the field order is fixed, so
                two runs' projections diff as numbers
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
parser.add_argument('--integers', action='store_true', help='the file is the integer projection')
args = parser.parse_args()

# The action codes of the integer projection (borg_town_dump.rs).
ACTION_CODES = {0: 'visit', 1: 'buy', 2: 'take_home', 3: 'sell', 4: 'sell_home'}


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

    # The state and the stock are the context; the decision and the reason
    # are what the owner reads. In Markdown the context is folded away, so a
    # batch of visits can be scanned in one screen.
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
    else:
        lines.append(f"Visit {record.get('seq')} - {state}")
        lines.extend(f"  {line}" for line in detail)
        lines.append(f"  decision: {decision_line}")
        lines.append(f"  reason: {reason}")
    lines.append("")
    return "\n".join(lines) + "\n"


def render_integer(record, markdown):
    """One integer record: a compact line, for diffing two runs.

    The integer projection has no names, so there is nothing to read but the
    numbers: the state, the action, and what the character carried.

    The file holds two record shapes - the decision (a town visit) and the
    purchase (a transaction that was carried out) - and they are told apart
    by the fields they carry, not by a type tag: a decision record has an
    `action`, a purchase record has the gold before it.
    """
    if 'action' not in record:
        return render_integer_purchase(record, markdown)

    action = ACTION_CODES.get(record.get('action'), '?')
    pack = record.get('pack') or []
    equip = record.get('equip') or []
    home = record.get('home') or []
    shops = record.get('shops') or []
    wares = sum(len(s[1]) for s in shops if len(s) > 1)
    line = (f"seq {record.get('seq')} | turn {record.get('turn')} | depth {record.get('depth')} "
            f"| level {record.get('level')} | gold {record.get('gold')} | hp {record.get('chp')}/{record.get('mhp')} "
            f"| light {record.get('light')} food {record.get('food')} cut {record.get('cut')} "
            f"poison {record.get('poison')} | {action} shop {record.get('shop')} "
            f"ware {record.get('ware')} item {record.get('item')} price {record.get('price')} "
            f"| pack {len(pack)} equip {len(equip)} home {len(home)} wares {wares}")
    if markdown:
        return f"`{line}`\n"
    return f"{line}\n"


def render_integer_purchase(record, markdown):
    """One purchase in the integer projection: the transaction, in numbers.

    A purchase carries no state and no action - the transaction is the whole
    record - so it gets its own line rather than a decision line full of
    absent values.
    """
    ware = record.get('ware') or []
    if len(ware) == 5:
        ware_line = (f"ware slot {ware[0]} kind {ware[1]} subkind {ware[2]} "
                     f"qty {ware[3]} price {ware[4]}")
    else:
        ware_line = f"ware {ware}"
    line = (f"seq {record.get('seq')} | purchase | turn {record.get('turn')} "
            f"| shop {record.get('shop')} | price {record.get('price')} "
            f"| gold_before {record.get('gold_before')} | {ware_line}")
    if markdown:
        return f"`{line}`\n"
    return f"{line}\n"


records = read_records(args.dump)

if args.integers:
    out = [render_integer(record, args.markdown) for record in records]
    if args.limit:
        out = out[:args.limit]
    print(''.join(out))
    raise SystemExit(0)

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
