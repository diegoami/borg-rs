#!/usr/bin/env python3
"""Curate the baseline of inventory choices.

The town-visit dump records what the Borg decided at every town visit. The
Borg re-decides the same thing every turn while it walks to the shop, so
the raw dump repeats each decision a dozen times; what is worth keeping is
the **distinct decision** - the choice, and the state it was made in.

This turns a set of runs into the baseline:

- a **fixture** - the distinct decisions, in the integer projection's
  frozen shape (schema version 2), one JSON object per line. It is
  replayable: it carries the whole trait vector, the inventory, the stock
  and the control's choice, so a decision function can be run against it
  and compared with what the control did.
- a **summary** - the same baseline for a reader: the distinct choices with
  the names, the gold, the depth and the Borg's own reason, and the
  provenance of every run.

The fixture is regenerated from the runs, never edited by hand: the summary
carries the command that made it.

Usage:
    tools/town_baseline.py --run LABEL INT_DUMP DESC_DUMP COMMAND [--run ...] \\
        --control COMMIT --out FIXTURE.jsonl --summary SUMMARY.md
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
parser.add_argument('--out', type=Path, required=True, help='the fixture to write')
parser.add_argument('--summary', type=Path, required=True, help='the summary to write')
args = parser.parse_args()

ACTION_NAMES = {0: 'visit', 1: 'buy', 2: 'take_home', 3: 'sell', 4: 'sell_home'}


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


def decision_key(record):
    """What makes a decision the same decision: the action and its target."""
    return (record.get('action'), record.get('shop'),
            record.get('ware'), record.get('item'))


def state_signature(record):
    """The state of a decision, without the run-specific bookkeeping.

    Two records with the same signature are the same decision made in the
    same state - which is what makes the repeats redundant.
    """
    return json.dumps({k: v for k, v in sorted(record.items())
                       if k not in ('v', 'seq')}, sort_keys=True)


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

# The distinct decisions, in the order they were first made.
distinct = []
seen = {}
for run in runs:
    for record in run['decisions']:
        key = decision_key(record)
        entry = seen.get(key)
        if entry is None:
            entry = {
                'key': key,
                'record': record,
                'label': run['label'],
                'seq': record.get('seq'),
                'times': 0,
                'signatures': set(),
                'desc': run['descriptive_by_seq'].get(record.get('seq'), {}),
            }
            seen[key] = entry
            distinct.append(entry)
        entry['times'] += 1
        entry['signatures'].add(state_signature(record))

# The fixture: the distinct decisions, run by run, in the order they were
# first made. The seq is the one the run gave it, so the fixture keeps the
# provenance of the run it came from.
fixture = [entry['record'] for entry in distinct]
args.out.write_text(''.join(json.dumps(record, separators=(',', ':')) + '\n'
                            for record in fixture))

# The summary.
lines = []
lines.append('# The baseline of inventory choices')
lines.append('')
lines.append('What the current Borg does in town, at real states, so that the')
lines.append('new algorithm can be checked against it. The fixture is the')
lines.append('machine-readable half (`%s`); this is the readable half.' % args.out.name)
lines.append('')
lines.append('## The runs')
lines.append('')
lines.append(f"Harvested at the control commit `{args.control}`, with the Rust Borg and")
lines.append('the dump on. The dump is observation only, so the control policy is')
lines.append("today's behaviour and the keypress stream is the same as the C Borg's.")
lines.append('')
lines.append('| run | visits | distinct decisions |')
lines.append('| --- | --- | --- |')
for run in runs:
    keys = {decision_key(r) for r in run['decisions']}
    lines.append(f"| `{run['label']}` | {len(run['decisions'])} | {len(keys)} |")
lines.append('')
lines.append(f"Together: **{sum(len(r['decisions']) for r in runs)} visits, "
             f"{len(distinct)} distinct decisions**.")
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
lines.append('| choice | target | shop | gold | depth | the Borg\'s reason | seen |')
lines.append('| --- | --- | --- | --- | --- | --- | --- |')
for entry in distinct:
    record = entry['record']
    desc = entry['desc']
    decision = desc.get('decision', {}) if desc else {}
    action = ACTION_NAMES.get(record.get('action'), '?')
    target = (decision.get('ware_desc') or decision.get('item_desc') or '').strip()
    shop = decision.get('shop_name') or ''
    reason = (decision.get('reason') or '').strip()
    lines.append('| {} | {} | {} | {} | {} | {} | {} |'.format(
        action, target or '-', shop or '-', record.get('gold'), record.get('depth'),
        reason or '-', entry['times']))
lines.append('')
lines.append('## How the fixture was made')
lines.append('')
lines.append('The Borg re-decides the same thing every turn while it walks to the')
lines.append('shop, so the raw dump repeats each decision. A repeat is the same')
lines.append('decision in the same state: the state signature - the record without')
lines.append('its seq - is the same for all the repeats of a decision in all but')
lines.append('one case.')
lines.append('')
repeat_groups = sum(1 for e in distinct if e['times'] > 1)
same_state = sum(1 for e in distinct if len(e['signatures']) == 1)
lines.append(f'- **{repeat_groups}** of the {len(distinct)} distinct decisions were made more than once;')
lines.append(f'- **{same_state}** of them were made in exactly one state (every repeat had the same');
lines.append('  state signature), so the repeats carry no new information.')
lines.append('')
lines.append('The fixture keeps the first record of each distinct decision, in the')
lines.append('order the runs were made. The `seq` is the one the run gave it, so')
lines.append('each record points back at the run it came from:')
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
command.append('--out {}'.format(args.out))
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

print(f'{len(distinct)} distinct decisions from '
      f'{sum(len(r["decisions"]) for r in runs)} visits')
print(f'fixture: {args.out} ({args.out.stat().st_size} bytes)')
print(f'summary: {args.summary}')
