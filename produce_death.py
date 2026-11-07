#!/usr/bin/env python3
"""Collect a natural recorded Rust Borg death within a prospective bounded plan."""
# Adapted from better-borg-c 22f817c96843; see tools/IMPORTED.json/licence.
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import secrets
import sys
import subprocess
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'tools'))
import death_workflow as workflow
import borg_runtime as runtime
import borg_deaths as deaths


def seeds_for(args):
    if args.seeds is not None:
        if args.games is not None or args.seed is not None:
            raise ValueError('Use --seeds OR --games/--seed')
        seeds = args.seeds
    else:
        first = args.seed if args.seed is not None else secrets.randbits(32)
        seeds = [(first + index) % (2**32) for index in range(args.games if args.games is not None else 3)]
    if not seeds or len(seeds) > 1000 or len(set(seeds)) != len(seeds) or any(not 0 <= seed <= 0xffffffff for seed in seeds):
        raise ValueError('Use 1..1000 distinct uint32 seeds')
    return seeds


def prospective_births(seeds, birth, commit):
    # The current harness selects explicit ordinary menus. Freeze varied
    # choices before outcomes; never consume or overwrite game RNG to pick them.
    names = []
    for filename in ('p_race.txt', 'class.txt'):
        text = workflow.git('show', commit + ':angband/lib/gamedata/' + filename)
        names.append([line.split(':', 1)[1] for line in text.splitlines() if line.startswith('name:')])
    if any(not 1 <= len(values) <= 26 for values in names):
        raise ValueError('Unsupported race/class menu size in pinned data')
    plan = []
    for seed in seeds:
        selection = hashlib.sha256(b'borg-rs prospective birth v1\0' + seed.to_bytes(4, 'big')).digest()
        race = int.from_bytes(selection[:16], 'big') % len(names[0]) if birth == 'random' else 0
        cls = int.from_bytes(selection[16:], 'big') % len(names[1]) if birth == 'random' else 0
        plan.append({'seed': seed, 'race': race, 'class': cls, 'race_name': names[0][race], 'class_name': names[1][cls]})
    return plan


def outcome(bundle):
    summary_path, process_path = bundle / 'run/summary.json', bundle / 'run/process.json'
    summary = workflow.read(summary_path) if summary_path.exists() else None
    process = workflow.read(process_path) if process_path.exists() else {}
    if summary and summary.get('dead') and summary.get('stop_reason') == 'death':
        result = 'retirement' if summary.get('death_cause') in ('Retiring', 'Quitting', 'Interrupting') else 'death'
    elif process.get('status') != 'success':
        result = process.get('status', 'harness_failure')
    else:
        result = {'turns': 'turn_cap', 'keys': 'key_cap', 'completion': 'completion'}.get(summary.get('stop_reason') if summary else None, 'unexpected_stop')
    return {'outcome': result, 'cause': summary.get('death_cause') if summary else None,
            'character': f"{summary['race']} {summary['class']}" if summary else None,
            'final': summary, 'process_status': process.get('status'),
            'qualification': 'recording/export validation pending'}


def collect(args, output, births, commit):
    hashes = workflow.tool_hashes()
    plan = {'schema': 'borg-rs/death-collection@1', 'created_at': workflow.now(), 'source_commit': commit,
            'seeds': [entry['seed'] for entry in births], 'prospective_births': births,
            'turn_cap': args.turns, 'key_cap': args.keys, 'wall_limit_per_game_seconds': args.timeout,
            'stall_timeout_seconds': args.stall_timeout, 'checkpoint_every': args.checkpoint_every,
            'stop_rule': 'first natural death, operational failure, or listed seeds exhausted',
            'birth_policy': args.birth, 'character_selection': 'prospective SHA-256 race/class selection; ordinary explicit menus/default point buy',
            'safety': 'cheat death/live and winner respawn off; native death evidence on',
            'tools_sha256': hashes, 'issue': args.death_issue, 'runs': [], 'counts': {}, 'status': 'preparing',
            'limit': 'Death collection, not a survival-rate cohort; no retrospective seed/budget enlargement'}
    contract = {key: value for key, value in plan.items() if key not in ('runs', 'counts', 'status')}
    workflow.write(output / 'plan.json', contract)
    plan['plan_sha256'] = workflow.sha(output / 'plan.json')
    workflow.write(output / 'collection.json', plan)
    try:
        print('Preparing the pinned Rust recording runtime...', flush=True)
        with workflow.locked(workflow.primary_root() / 'build/death-workflow/prepare.lock'):
            prepared = workflow.prepare_runtime(args, commit, hashes)
        plan.update(prepared_manifest=str(Path(prepared['cache']) / 'prepared.json'),
                    prepared_manifest_sha256=workflow.sha(Path(prepared['cache']) / 'prepared.json'), status='running')
        workflow.write(output / 'collection.json', plan)
        for index, birth in enumerate(births, 1):
            workflow.validate_prepared(prepared, commit, hashes)
            profile = output / f"game-{index:03d}-seed-{birth['seed']}"
            entry = {**birth, 'directory': str(profile), 'outcome': 'harness_failure'}
            plan['runs'].append(entry)
            workflow.write(output / 'collection.json', plan)
            options = SimpleNamespace(character='CollectedBorg', new=True, from_backup=None, seed=birth['seed'],
                race=birth['race'], cls=birth['class'], turns=args.turns, keys=args.keys, wall_seconds=args.timeout,
                stall_seconds=args.stall_timeout, checkpoint_every=args.checkpoint_every, death_issue=args.death_issue)
            print(f"Game {index}/{len(births)}; seed {birth['seed']}; {birth['race_name']} {birth['class_name']}; cap {args.turns}", flush=True)
            bundle = None
            try:
                if workflow.sha(output / 'plan.json') != plan['plan_sha256']:
                    raise ValueError('Prospective plan changed during collection')
                game_prepared = {**prepared, 'context': {**prepared.get('context', {}), 'collection_plan': contract, 'collection_plan_sha256': plan['plan_sha256']}}
                bundle = runtime.record_run(options, prepared=game_prepared, profile_root=profile, export_death=False)
                entry.update(outcome(bundle), recording=str(bundle))
                runtime.validate(bundle)
                workflow.validate_prepared(prepared, commit, hashes)
                entry['qualification'] = 'complete frozen recording; rerun verification and assessment remain separate'
                entry['recording_manifest_sha256'] = workflow.sha(bundle / 'recording.json')
                if entry['outcome'] == 'death':
                    plan['counts'] = dict(Counter(item['outcome'] for item in plan['runs']))
                    workflow.write(output / 'collection.json', plan)
                    dossier = deaths.maybe_register(bundle, issue=args.death_issue)
                    if dossier is None:
                        raise ValueError('Terminal death was not eligible for natural-death export')
                    entry.update(dossier=str(dossier), assessment='unassessed; owner replay and causal investigation pending')
                    plan.update(status='natural_death_recorded', counts=dict(Counter(item['outcome'] for item in plan['runs'])))
                    workflow.write(output / 'collection.json', plan)
                    print('Natural death:', dossier, flush=True)
                    print('Next: ./replay_next_death.py; viewing alone does not mark owner review complete.', flush=True)
                    return 0
                if entry['outcome'] not in ('turn_cap', 'key_cap', 'completion'):
                    raise ValueError('Operational stop: ' + entry['outcome'])
            except (Exception, KeyboardInterrupt) as error:
                candidates = list((profile / 'recordings').glob('*'))
                if bundle is None and len(candidates) == 1:
                    bundle = candidates[0]
                if bundle is not None:
                    entry.update(outcome(bundle), recording=str(bundle))
                entry['error'] = str(error)
                raise
            plan['counts'] = dict(Counter(item['outcome'] for item in plan['runs']))
            workflow.write(output / 'collection.json', plan)
        plan['status'] = 'limits_reached_without_death'
        workflow.write(output / 'collection.json', plan)
        print('No natural death within declared games/limits. No extra games or larger caps were added.', flush=True)
        return 0
    except (Exception, KeyboardInterrupt) as error:
        plan.update(status='failed', error=str(error), counts=dict(Counter(item['outcome'] for item in plan['runs'])))
        workflow.write(output / 'collection.json', plan)
        raise


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--birth', choices=('random', 'fixed'), default='random', help='prospectively varied race/class, or Human Warrior')
    parser.add_argument('--games', type=int, help='maximum games; default3')
    parser.add_argument('--seed', type=int, help='first uint32 seed; default random')
    parser.add_argument('--seeds', nargs='+', type=int)
    parser.add_argument('--turns', type=int, default=200000, help='absolute world-turn cap per new game')
    parser.add_argument('--timeout', type=int, default=120)
    parser.add_argument('--stall-timeout', type=int, default=30, help='seconds without advancement in the flushed key-log world turn')
    parser.add_argument('--ref', default='HEAD', help='committed borg-rs source pinned before collection')
    parser.add_argument('--build', type=Path, help='existing ordinary Rust harness build matching --ref')
    parser.add_argument('--output', type=Path, help='new directory under harness/out or BORG_DEATH_WORKFLOW_DIR')
    parser.add_argument('--death-issue', default=None, help='optional discussion link; never borrow a C repository issue')
    parser.add_argument('--keys', type=int, default=50000, help='independent Rust Borg event cap per game')
    parser.add_argument('--checkpoint-every', type=int, default=1000)
    args = parser.parse_args(argv)
    if (args.games is not None and not 1 <= args.games <= 1000) or not 1 <= args.turns <= 2147483647 or min(args.timeout, args.stall_timeout, args.checkpoint_every) < 1 or not 1 <= args.keys <= 999999999 or (args.seed is not None and not 0 <= args.seed <= 0xffffffff):
        parser.error('Use1..1000 games, uint32 seeds, positive limits and supported turn/key budgets')
    try:
        seeds = seeds_for(args)
        commit = workflow.source_commit(args.ref)
        births = prospective_births(seeds, args.birth, commit)
        output = (args.output or workflow.artifacts() / 'collections' / workflow.now().replace(':', '-')).resolve()
        allowed = workflow.primary_root() / 'harness/out'
        if not output.is_relative_to(allowed) and not output.is_relative_to(workflow.artifacts()):
            raise ValueError('Keep full output below primary harness/out or BORG_DEATH_WORKFLOW_DIR')
        output.mkdir(parents=True, exist_ok=False)
        print(f'Pinned source: {commit}\nProspective seeds: {seeds}\nEvidence: {output}', flush=True)
        return collect(args, output, births, commit)
    except (OSError, ValueError, KeyError, subprocess.SubprocessError, KeyboardInterrupt) as error:
        print(str(error) or 'Interrupted; partial evidence retained', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
