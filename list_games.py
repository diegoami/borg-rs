#!/usr/bin/env python3
"""List recorded games from a death collection with the commands to replay or play them.

Reads the collection's per-game reports, the death dossiers, the owner-review ledger
and the owner's manual takeover branches. It only prints; it never runs a game.
"""
# Adapted from better-borg-c list_games.py; see tools/IMPORTED.json/licence.
import argparse
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'tools'))
import death_workflow as workflow
import borg_deaths as deaths
import borg_runtime as runtime

DEATH = 'death'
WINDOW = 105


def read(path):
    try:
        return json.loads(Path(path).read_text())
    except (OSError, ValueError):
        return None


def collections_dir():
    return workflow.artifacts() / 'collections'


def pick_collection(value):
    base = collections_dir()
    if value:
        path = Path(value)
        if not path.is_dir():
            path = base / value
        if not path.is_dir():
            raise SystemExit(f'No collection {value!r} (looked in {base})')
        return path
    found = sorted(p for p in base.glob('*') if (p / 'collection.json').is_file())
    if not found:
        raise SystemExit(f'No collections under {base}')
    return found[-1]


def games_from_file(path):
    """Game directories named on 'Report:' lines of a saved produce_death transcript."""
    games = []
    for line in Path(path).read_text().splitlines():
        match = re.search(r'Report:\s+(\S+)/reports/summary\.md', line)
        if match and match.group(1) not in games:
            games.append(match.group(1))
    if not games:
        raise SystemExit(f'No "Report:" lines found in {path}')
    return [Path(g) for g in games]


def bundle_of(game):
    """A game's own profile directory holds its recording under recordings/<stamp>."""
    return next((game / 'recordings').glob('*'), None)


def dossiers(docs):
    """Map a raw recording bundle directory to its dossier record and assessment."""
    found = {}
    for record_path in sorted(docs.glob('D[0-9][0-9][0-9][0-9]-*/record.json')):
        record = read(record_path)
        if not record:
            continue
        assessment = read(record_path.with_name('assessment.json')) or {}
        for attempt in record.get('attempts', []):
            raw = attempt.get('raw_directory')
            if raw and raw not in found:
                found[raw] = {'case': record.get('id'), 'path': record_path.parent,
                              'family': workflow.family(record, assessment),
                              'classification': assessment.get('classification', 'unassessed')}
    return found


def manual_branches():
    """Owner takeover branches under .angband-rust/human-branches (or BORG_PLAY_DATA_DIR).

    borg-rs does not record a per-session outcome for a resumed manual branch
    (unlike better-borg-c's sessions/*/result.json), so only the branch point
    is shown, not what happened after.
    """
    found = {}
    for branch_path in sorted(runtime.data_root().glob('human-branches/*/human-branch.json')):
        branch = read(branch_path)
        if not branch:
            continue
        found.setdefault(branch.get('original_recording'), []).append(
            {'branch': branch_path.parent.name, 'turn': (branch.get('boundary') or {}).get('actual_turn')})
    return found


def describe(game, docs_map, reviews, branches):
    summary = read(game / 'reports/summary.json') or {}
    bundle = bundle_of(game)
    entry = {'directory': str(game), 'bundle': str(bundle) if bundle else None,
             'seed': summary.get('requested_seed', summary.get('seed')),
             'outcome': summary.get('outcome', 'unknown'),
             'character': summary.get('character', '?'), 'turn': summary.get('engine_ticks'),
             'level': summary.get('level'), 'max_depth': summary.get('max_depth'),
             'cause': summary.get('cause'), 'case': None, 'reviewed': None,
             'classification': None, 'manual': branches.get(str(bundle), []) if bundle else []}
    dossier = docs_map.get(str(bundle)) if bundle else None
    if dossier:
        entry['case'] = dossier['case']
        entry['classification'] = dossier['classification']
        entry['reviewed'] = bool(reviews.get(dossier['family'], {}).get('reviewed'))
    return entry


def relative(path):
    try:
        return str(Path(path).resolve().relative_to(workflow.primary_root()))
    except ValueError:
        return str(path)


def commands(entry):
    lines = []
    if entry['bundle'] is None:
        lines.append('watch:  unavailable (no captured recording)')
        lines.append('play:   unavailable (no captured recording)')
    else:
        bundle = relative(entry['bundle'])
        if entry['outcome'] == DEATH:
            start = max(1, (entry['turn'] or 0) - WINDOW)
            lines.append(f'watch:  ./replay-borg.py {bundle}   (opens at turn {start}; '
                         f'add --from-turn <turn> to start elsewhere)')
        else:
            lines.append(f'watch:  ./replay-borg.py {bundle} --view-only')
        lines.append(f'play:   ./play-from-replay.py {bundle} --from-turn <turn>|latest')
    if entry['case']:
        lines.append(f'review: ./replay_next_death.py --case {entry["case"]}'
                     f'   (mark done: ./replay_next_death.py --mark-reviewed {entry["case"]})')
    return lines


def render(entry, index):
    where = f"CL{entry['level']} maxDL{entry['max_depth']} turn {entry['turn']:,}" if entry['turn'] else ''
    what = entry['cause'] if entry['outcome'] == DEATH else entry['outcome']
    head = f"{index:>2}. seed {entry['seed']}  {entry['character']}  {where}  {what}"
    tags = []
    if entry['case']:
        tags.append(entry['case'] + (' reviewed' if entry['reviewed'] else ' NOT reviewed'))
    for branch in entry['manual']:
        tags.append(f"manual branch from turn {branch['turn']}: {branch['branch']}")
    out = [head]
    if tags:
        out.append('    ' + '; '.join(tags))
    out += ['    ' + line for line in commands(entry)]
    return '\n'.join(out)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    source = parser.add_mutually_exclusive_group()
    source.add_argument('--file', help='saved produce_death transcript; list only the games it names')
    source.add_argument('--collection', help='collection directory or its timestamp name (default: latest)')
    parser.add_argument('--pending', action='store_true', help='only deaths not yet reviewed by the owner')
    parser.add_argument('--deaths', action='store_true', help='only deaths')
    parser.add_argument('--json', action='store_true', help='machine-readable output')
    args = parser.parse_args(argv)

    if args.file:
        games = games_from_file(args.file)
        label = args.file
    else:
        collection = pick_collection(args.collection)
        games = sorted(p for p in collection.glob('game-*') if p.is_dir())
        label = str(collection)
    docs_map = dossiers(deaths.registry_root())
    reviews = workflow.reviews()
    branches = manual_branches()
    entries = [describe(g, docs_map, reviews, branches) for g in games]
    if args.deaths or args.pending:
        entries = [e for e in entries if e['outcome'] == DEATH]
    if args.pending:
        entries = [e for e in entries if not e['reviewed']]

    if args.json:
        print(json.dumps(entries, indent=2, default=str))
        return 0
    print(f'{len(entries)} game(s) from {relative(label)}')
    print('Run commands from the repository root. <turn> is an engine turn; play starts at the'
          ' first Borg decision at or after it.\n')
    for index, entry in enumerate(entries, 1):
        print(render(entry, index))
        print()
    return 0


if __name__ == '__main__':
    sys.exit(main())
