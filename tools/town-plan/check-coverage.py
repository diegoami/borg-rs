#!/usr/bin/env python3
"""Gate ordinary-shop coverage against engine-backed hypothetical-item audits.

This checks capability wiring, not affordability, executed buys or survival.
Each audit must come from BORG_TOWN_CAPABILITY_AUDIT on the tested build.
"""
import argparse
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]


def clean(name):
    return name.removeprefix('& ').replace('~', '').replace('|', '').strip().lower()


def check(paths):
    source = (ROOT / 'docs/town-item-catalogue.html').read_text()
    rows = json.loads(re.search(r'const data=(.*);\n', source)[1])
    values = {name.lower().replace('_', ' '): int(value) for name, value in re.findall(
        r'pub const TV_(\w+):[^=]+ = (\d+);', (ROOT / 'borg-rs/src/headers/list_tvals_h.rs').read_text())}
    native = {}
    contexts = []
    for path in paths:
        records = [json.loads(line) for line in Path(path).read_text().splitlines()]
        context = records[0]
        plan = Path(path).with_name('plan.jsonl')
        first = json.loads(plan.open().readline())
        trait_source = (ROOT / 'borg-rs/src/borg_headers/borg_trait_h.rs').read_text()
        ammo_trait = int(re.search(r'pub const BI_AMMO_TVAL:[^=]+ = (\d+);', trait_source)[1])
        context['ammo_tval'] = dict(first['traits']).get(ammo_trait, 0)
        contexts.append(context)
        if records[0].get('hypothetical_known_items') != 1:
            raise ValueError(f'not a native hypothetical-item audit: {path}')
        for row in records[1:]:
            native.setdefault((row['tval'], clean(row['name'])), []).append(row)
    failures, exclusions, covered = [], [], []
    for path, context in zip(paths, contexts):
        if context['curves'][23][1] > 0:
            items = [json.loads(line) for line in Path(path).read_text().splitlines()[1:]]
            if not any(any(38 <= cap <= 46 for cap, _ in row['supplies']) for row in items):
                failures.append(f"class {context['class']}: no usable class-book access in a mana-capable context")
    for row in rows:
        if not any(x in row['shops'] for x in ('(always)', '(normal)')):
            continue
        key = values[{'digger': 'digging'}.get(row['type'], row['type'])], clean(row['kind'])
        matches = native.get(key, [])
        if not matches:
            failures.append(f"{row['type']} {row['kind']}: absent from native audits")
        elif row['type'] in ('wand', 'staff') and row['kind'] in ('Confuse Monster', 'Confuse Monsters', 'Stun Monster'):
            if any(r['supplies'] for r in matches):
                failures.append(f"{row['type']} {row['kind']}: unsupported device earns supply utility")
            else:
                exclusions.append([row['kind'], 'no enabled device consumer; first-copy learning remains separate'])
        elif any(r['gear'] or r['supplies'] for r in matches):
            covered.append(row['kind'])
        elif row['type'].endswith('book'):
            exclusions.append([row['kind'], 'class/level/mana access unavailable in these contexts'])
        elif row['kind'] == 'Whisky' or row['kind'] == 'Flask of Whisky':
            exclusions.append([row['kind'], 'confusion makes it unsafe food'])
        elif row['kind'] == 'Monster Confusion':
            exclusions.append([row['kind'], 'no enabled tactical consumer (#80); first-copy learning remains separate'])
        elif row['kind'] == 'Purging':
            exclusions.append([row['kind'], 'restoration requires a drained stat; purging is not nourishment'])
        elif row['type'] in ('arrow', 'bolt', 'shot') and not any(c['ammo_tval'] == key[0] for c in contexts):
            exclusions.append([row['kind'], 'no matching launcher in the audited contexts'])
        else:
            failures.append(f"{row['type']} {row['kind']}: no native capability or explicit contextual exclusion")
    return {'ordinary_kinds': len(covered) + len(exclusions) + len(failures),
            'contexts': [{'class': r['class'], 'level': r['level'], 'ammo_tval': r['ammo_tval']} for r in contexts],
            'covered': covered, 'contextual_exclusions': exclusions, 'failures': failures}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('audits', nargs='+', type=Path)
    parser.add_argument('--out', type=Path)
    args = parser.parse_args()
    result = check(args.audits)
    text = json.dumps(result, indent=2) + '\n'
    if args.out:
        args.out.write_text(text)
    else:
        print(text, end='')
    raise SystemExit(bool(result['failures']))


if __name__ == '__main__':
    main()
