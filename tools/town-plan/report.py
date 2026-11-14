#!/usr/bin/env python3
"""Summarise completed shadow runs; verify matched gameplay and parameters.

Counts describe decision snapshots, not executed purchases or independent lives.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import statistics

ROOT = Path(__file__).resolve().parents[2]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT / 'harness/out/rust/town-utility')
    parser.add_argument('--records-out', type=Path)
    args = parser.parse_args()
    if (args.root / 'experiment.json').exists():
        from class_report import report
        result = report(args.root)
        if args.records_out:
            lines = []
            for cell in result['cells']:
                source = args.root / cell['arm'] / f"{cell['seed']}-class{cell['class_idx']}" / 'plan.jsonl'
                if source.exists():
                    lines.extend(source.read_text().splitlines())
            args.records_out.write_text('\n'.join(lines) + '\n')
        print(json.dumps(result, indent=2))
        return
    constants = (ROOT / 'borg-rs/src/borg_headers/borg_h.rs').read_text()
    names = {int(n): key.lower() for key, n in re.findall(r'pub const (BORG_TOWN_\w+):[^=]+ = (\d+);', constants)}
    parameter_names = [names[i] for i in range(45, 69)]
    tvals = {key: int(n) for key, n in re.findall(r'pub const (TV_\w+):[^=]+ = (\d+);', (ROOT / 'borg-rs/src/headers/list_tvals_h.rs').read_text())}
    consumables = {tvals[k] for k in ('TV_FOOD', 'TV_MUSHROOM', 'TV_POTION', 'TV_SCROLL', 'TV_FLASK')}
    result = {'scope': 'shadow decision snapshots, not executed transactions or a survival comparison',
              'parameter_names': parameter_names, 'arms': {}, 'runs': {}}
    joined = []
    reference = {}
    for arm in ('reference', 'shallow', 'cash-cautious'):
        shares, left, snapshots = [], [], []
        recall_buys = recall_stores = exhausted = 0
        for cls, seed in enumerate(range(501, 508)):
            run = args.root / arm / f'{seed}-class{cls}'
            summary = json.loads((run / 'summary.json').read_text())
            process = json.loads((run / 'process.json').read_text())
            assert process['status'] == 'success' and process['exit_code'] == 0
            settings = json.loads((run / 'town-parameters.json').read_text())
            assert all(summary['effective_settings'][k] == int(v) for k, v in settings.items())
            hashes = {f: digest(run / f) for f in ('keys.log', 'messages.log', 'state.log')}
            actual = {k: v for k, v in summary.items() if k not in ('wall_seconds', 'effective_settings')}
            if arm == 'reference':
                reference[seed] = (hashes, actual)
            else:
                assert (hashes, actual) == reference[seed], (arm, seed, 'gameplay changed')
            lines = (run / 'plan.jsonl').read_text().splitlines()
            rows = [json.loads(x) for x in lines]
            for r in rows:
                assert r['v'] == 5 and len(r['params']) == 24
                assert all(r['params'][i] == summary['effective_settings'][key] for i, key in enumerate(parameter_names))
                assert all(len(k) == 12 for k in r['kit'])
                gold = r['limits'][0] + sum(k[8] for k, d in zip(r['kit'], r['dispose']) if d == 2)
                spent = sum(e[4] for e in r['plan'])
                assert 0 <= spent <= gold
                if gold > 0:
                    shares.append(100 * spent / gold)
                    left.append(gold - spent)
                    exhausted += spent == gold
                recall_buys += any(e[0] == 2 and e[1] == 7 for e in r['plan'])
                recall_stores += any(k[5] == 7 and d == 1 for k, d in zip(r['kit'], r['dispose']))
                by_place = {0: {item[0]: item[1] for item in r['pack']},
                            1: {item[0]: item[1] for item in r['home']}}
                for k in r['kit']:
                    if by_place[k[0]].get(k[1]) in consumables:
                        assert k[7] == 0, 'consumable inherited inventory power'
            joined += lines
            snapshots += rows
            result['runs'][f'{arm}/{seed}'] = {
                'seed': seed, 'class_idx': cls, 'class': summary['class'], 'records': len(rows),
                'stop_reason': summary['stop_reason'], 'game_turns': summary['game_turns'],
                'death_cause': summary.get('death_cause'), 'plan_sha256': digest(run / 'plan.jsonl'),
                'channels': hashes, 'binary_sha256': process['executable_sha256_before']}
        result['arms'][arm] = {
            'completed_runs': 7, 'snapshots': len(snapshots),
            'median_planned_spend_percent_of_available_gold': round(statistics.median(shares), 3),
            'minimum_gold_retained_from_positive_purse': min(left),
            'positive_purse_exhausted_by_discretionary_plan': exhausted,
            'snapshots_planning_recall_purchase': recall_buys,
            'snapshots_planning_recall_storage': recall_stores}
    result['matched_gameplay'] = '7/7 seed/class pairs identical across all 3 parameter settings on keys/messages/state and gameplay summaries'
    if args.records_out:
        args.records_out.write_text('\n'.join(joined) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
