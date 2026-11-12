#!/usr/bin/env python3
"""Curate the baseline of inventory choices.

The town-visit dump records what the Borg decided at every town visit. The
Borg re-decides the same thing every turn while it walks to the shop, and
the state does not change while it walks, so the raw dump repeats each
decision many times over. What is worth keeping is the **distinct state**:
the state, and the choice the control made in it.

This turns a set of runs into the baseline:

- a **fixture** - the distinct (state, choice) pairs, in the integer
  projection's frozen shape (schema version 2), one JSON object per line.
  It is replayable: it carries the whole trait vector, the inventory, the
  stock and the control's choice, so a decision function can be run against
  it and compared with what the control did.
- a **summary** - the same baseline for a reader: the distinct choices with
  the names, the gold, the depth and the Borg's own reason, and the
  provenance of every run.

The fixture is regenerated from the runs, never edited by hand: the summary
carries the command that made it.

The fixture is written **one file per group**, the group being the first
word of the run's label (the class, here). One file for the whole corpus
is several hundred kilobytes, which is more than a pull request wants to
carry in one piece.

Usage:
    tools/town_baseline.py --run LABEL INT_DUMP DESC_DUMP COMMAND [--run ...] \\
        --control COMMIT --out-dir DIR --summary SUMMARY.md
"""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--run', nargs=4, action='append', required=True,
                    metavar=('LABEL', 'INTEGERS', 'DESCRIPTIVE', 'COMMAND'),
                    help='one run: a label, its integer dump, its descriptive dump, '
                         'and the command that produced them')
parser.add_argument('--control', required=True,
                    help='the commit the control policy was harvested at')
parser.add_argument('--out-dir', type=Path, required=True,
                    help='the directory the fixture files are written to')
parser.add_argument('--summary', type=Path, required=True, help='the summary to write')
args = parser.parse_args()

ACTION_NAMES = {0: 'visit', 1: 'buy', 2: 'take_home', 3: 'sell', 4: 'sell_home'}

# The bookkeeping: the version, the record number and the turn. Two records
# that differ only in those are the same decision in the same state.
BOOKKEEPING = ('v', 'seq', 'turn')


def read_records(path):
    """The records of one file, in order. A malformed line is reported."""
    records = []
    for number, line in enumerate(Path(path).read_text(errors='replace').splitlines(), start=1):
        line = line.strip()
        if not line:
            continue
        try:
            records.append(json.loads(line))
        except json.JSONDecodeError as err:
            print(f'{path}:{number}: {err}')
    return records


def signature(record):
    """The record without its bookkeeping: the state and the choice."""
    return json.dumps({k: v for k, v in sorted(record.items())
                       if k not in BOOKKEEPING}, sort_keys=True)


def state_signature(record):
    """The state alone, without the choice.

    The control's choice is not a function of the state: it carries a
    standing goal from the turn before. So a state can carry more than one
    choice, and the two are kept apart.
    """
    return json.dumps({k: v for k, v in sorted(record.items())
                       if k not in BOOKKEEPING and k not in ('action', 'shop', 'ware',
                                                            'item', 'price')},
                      sort_keys=True)


def choice_identity(record):
    """What the choice *is*, by the kind of the thing chosen.

    The slot is not the identity: a shop's stock shifts, so the same kind
    can sit in different slots in different states, and two different kinds
    can sit in the same slot in two different states.
    """
    action = record.get('action')
    if action in (1, 2):
        for shop in record.get('shops') or []:
            if shop and shop[0] == record.get('shop'):
                for ware in shop[1]:
                    if ware[0] == record.get('ware'):
                        return (action, record.get('shop'), 'ware', ware[1], ware[2])
        return (action, record.get('shop'), 'ware', None, None)
    if action in (3, 4):
        for group in ('pack', 'equip', 'home'):
            for item in record.get(group) or []:
                if item[0] == record.get('item'):
                    return (action, record.get('shop'), 'item', item[1], item[2])
        return (action, record.get('shop'), 'item', None, None)
    return (action, record.get('shop'), 'visit', None, None)


runs = []
for label, int_path, desc_path, command in args.run:
    integers = read_records(int_path)
    descriptive = {r.get('seq'): r for r in read_records(desc_path)
                    if r.get('type') == 'decision'}
    decisions = [r for r in integers if 'action' in r]
    runs.append({
        'label': label,
        'integers': int_path,
        'descriptive': desc_path,
        'command': command,
        'decisions': decisions,
        'descriptive_by_seq': descriptive,
    })

# The distinct (state, choice) pairs, in the order they were first made.
distinct = []
seen = {}
for run in runs:
    for record in run['decisions']:
        key = signature(record)
        entry = seen.get(key)
        if entry is None:
            entry = {
                'record': record,
                'label': run['label'],
                'seq': record.get('seq'),
                'times': 0,
                'states': set(),
                'desc': run['descriptive_by_seq'].get(record.get('seq'), {}),
            }
            seen[key] = entry
            distinct.append(entry)
        entry['times'] += 1
        entry['states'].add(state_signature(record))

# The distinct choices, by what they are.
choices = {}
for entry in distinct:
    identity = choice_identity(entry['record'])
    choice = choices.get(identity)
    if choice is None:
        choice = {'identity': identity, 'record': entry['record'], 'desc': entry['desc'],
                  'states': 0, 'visits': 0, 'gold': [], 'depth': []}
        choices[identity] = choice
    choice['states'] += 1
    choice['visits'] += entry['times']
    choice['gold'].append(entry['record'].get('gold'))
    choice['depth'].append(entry['record'].get('depth'))

# The fixture: the distinct pairs, run by run, in the order they were first
# made. The seq is the one the run gave it, so each record points back at
# the run it came from.
#
# One file per group: the group is the first word of the run's label, and
# the fixture file is named after it.
fixture = [entry['record'] for entry in distinct]
group_of = {id(entry['record']): entry['label'].split('-')[0] for entry in distinct}
args.out_dir.mkdir(parents=True, exist_ok=True)
fixture_files = []
for group in sorted({g for g in group_of.values()}):
    records = [r for r in fixture if group_of[id(r)] == group]
    path = args.out_dir / f'town-baseline-{group}.jsonl'
    path.write_text(''.join(json.dumps(record, separators=(',', ':')) + '\n'
                            for record in records))
    fixture_files.append((group, path, len(records)))

# The summary.
lines = []
lines.append('# The baseline of inventory choices')
lines.append('')
lines.append('What the current Borg does in town, at real states, so that the new')
lines.append('algorithm can be checked against it. The fixture is the')
lines.append('machine-readable half; this is the readable half.')
lines.append('')
lines.append('| fixture | records |')
lines.append('| --- | --- |')
for group, path, count in fixture_files:
    lines.append(f'| `{path.name}` | {count} |')
lines.append('')
lines.append('## The runs')
lines.append('')
lines.append(f"Harvested at the control commit `{args.control}`, with the Rust Borg and")
lines.append('the dump on. The dump is observation only, so the control policy is')
lines.append("today's behaviour and the keypress stream is the same as the C Borg's.")
lines.append('')
lines.append('| run | visits | distinct states |')
lines.append('| --- | --- | --- |')
for run in runs:
    keys = {signature(r) for r in run['decisions']}
    lines.append(f"| `{run['label']}` | {len(run['decisions'])} | {len(keys)} |")
lines.append('')
visits = sum(len(r['decisions']) for r in runs)
states = {state_signature(e['record']) for e in distinct}
lines.append(f"Together: **{visits} visits, {len(distinct)} distinct (state, choice) pairs, "
             f"{len(states)} distinct states, {len(choices)} distinct choices**.")
lines.append('')
lines.append('Each run was:')
lines.append('')
lines.append('```sh')
for run in runs:
    lines.append(run['command'])
lines.append('```')
lines.append('')
lines.append('## The distinct choices')
lines.append('')
lines.append('What the control actually chooses, by the kind of the thing chosen. The')
lines.append('slot is not the identity: a shop\'s stock shifts, so the same kind sits')
lines.append('in different slots in different states.')
lines.append('')
lines.append('| choice | target | shop | gold | depth | the Borg\'s reason | states | visits |')
lines.append('| --- | --- | --- | --- | --- | --- | --- | --- |')
for choice in choices.values():
    record = choice['record']
    desc = choice['desc']
    decision = desc.get('decision', {}) if desc else {}
    action = ACTION_NAMES.get(record.get('action'), '?')
    target = (decision.get('ware_desc') or decision.get('item_desc') or '').strip()
    shop = decision.get('shop_name') or ''
    reason = (decision.get('reason') or '').strip()
    gold = sorted(set(g for g in choice['gold'] if g is not None))
    depth = sorted(set(d for d in choice['depth'] if d is not None))
    gold_text = str(gold[0]) if len(gold) == 1 else f"{gold[0]}-{gold[-1]}"
    depth_text = str(depth[0]) if len(depth) == 1 else f"{depth[0]}-{depth[-1]}"
    lines.append('| {} | {} | {} | {} | {} | {} | {} | {} |'.format(
        action, target or '-', shop or '-', gold_text, depth_text,
        reason or '-', choice['states'], choice['visits']))
lines.append('')
lines.append('## How the fixture was made')
lines.append('')
lines.append('The Borg re-decides the same thing every turn while it walks to the')
lines.append('shop, and the state does not change while it walks, so the raw dump')
lines.append('repeats each decision. The fixture keeps one record per distinct')
lines.append('(state, choice) pair: the state signature - the record without its')
lines.append('version, its record number and its turn - is what decides.')
lines.append('')
lines.append(f'- **{visits} visits** became **{len(distinct)} records**;')
lines.append(f'- those are **{len(states)} distinct states**, and **{len(choices)} distinct choices**.')
lines.append('')
ambiguous = len(states) - len({state_signature(e['record']) for e in distinct})
multi = sum(1 for s in states
            if len({e['record'].get('action') for e in distinct
                    if state_signature(e['record']) == s}) > 1)
lines.append(f'**The control\'s choice is not a function of the state.** It carries a')
lines.append('standing goal from the turn before, so the same state can carry two')
lines.append(f'choices: **{multi}** of the {len(states)} states do. The fixture keeps')
lines.append('both, because each is a fact about the control. A new algorithm that')
lines.append('is a function of the state will agree with at most one of them, and')
lines.append('that is the comparison.')
lines.append('')
lines.append('The fixture keeps the first record of each pair, in the order the runs')
lines.append('were made. The `seq` is the one the run gave it, so each record points')
lines.append('back at the run it came from:')
lines.append('')
lines.append('| run | the fixture records it contributed |')
lines.append('| --- | --- |')
for run in runs:
    contributed = [e for e in distinct if e['label'] == run['label']]
    if not contributed:
        continue
    lines.append('| `{}` | {} |'.format(run['label'],
                 ', '.join(str(e['seq']) for e in contributed)))
lines.append('')
lines.append('Regenerate it with:')
lines.append('')
lines.append('```sh')
command = ['tools/town_baseline.py', '--control', args.control]
for label, int_path, desc_path, run_command in args.run:
    command.append('--run {} {} {} "{}"'.format(label, int_path, desc_path, run_command))
command.append('--out-dir {}'.format(args.out_dir))
command.append('--summary {}'.format(args.summary))
lines.append(' \\\n    '.join(command))
lines.append('```')
lines.append('')
lines.append('The raw dumps the fixture is made from stay outside the repository:')
lines.append('they are runs, not source. The fixture is the curated part.')
lines.append('')
lines.append('## What it can and cannot settle')
lines.append('')
lines.append('- It **can** say where the new algorithm disagrees with the current')
lines.append('  one: the fixture is a state and a choice, and the new algorithm')
lines.append('  answers the same question about the same state.')
lines.append('- It **cannot** be the whole comparison. The power need prices a')
lines.append('  candidate with the try-on, which reads the item bonuses the engine')
lines.append('  holds - the record does not carry those. So the full comparison')
lines.append('  runs in game, where the engine is there: the shadow-plan mode')
lines.append('  (phase 4) puts the two decisions side by side in one record.')
lines.append('- It **does not** claim the current choices are right. It says what')
lines.append('  they are.')
lines.append('')
args.summary.write_text('\n'.join(lines) + '\n')

print(f'{visits} visits -> {len(distinct)} distinct (state, choice) pairs, '
      f'{len(states)} states, {len(choices)} choices')
for group, path, count in fixture_files:
    print(f'fixture: {path} ({count} records, {path.stat().st_size} bytes)')
print(f'summary: {args.summary}')
