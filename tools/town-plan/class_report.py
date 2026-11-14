#!/usr/bin/env python3
"""Report every manifest cell per class; shadow metrics never rank survival fitness."""
import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import statistics


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def report(root):
    root = Path(root)
    frozen = json.loads((root / 'experiment.json').read_text())
    if frozen.get('schema') != 'borg-rs/town-class-plan@1':
        raise ValueError('expected the class calibration runner manifest')
    results, groups, paired = [], defaultdict(list), defaultdict(dict)
    for cell in frozen['cells']:
        out = root / cell['arm'] / f'{cell["seed"]}-class{cell["class_idx"]}'
        item = {'arm': cell['arm'], 'class': cell['class'], 'class_idx': cell['class_idx'], 'seed': cell['seed'],
                'profile_applied': cell['profile_applied'], 'settings_sha256': cell['settings_sha256']}
        try:
            process = json.loads((out / 'process.json').read_text())
            if process['status'] != 'success' or process['exit_code'] != 0:
                item.update(outcome='harness_failure', process=process)
            else:
                summary = json.loads((out / 'summary.json').read_text())
                settings = json.loads((out / 'town-parameters.json').read_text())
                if summary['class'] != cell['class'] or summary['class_idx'] != cell['class_idx'] or summary['seed'] != cell['seed']:
                    raise ValueError('actual birth does not match the frozen cell')
                if settings != cell['settings'] or any(summary['effective_settings'].get(k) != int(v) for k, v in settings.items()):
                    raise ValueError('configured/parsed settings differ from frozen resolution')
                vectors = [v for k, v in summary['effective_settings'].items() if k.startswith('borg_town_') and k != 'borg_town_no_stores']
                spends, retained = [], []
                records = [json.loads(line) for line in (out / 'plan.jsonl').read_text().splitlines()]
                for r in records:
                    if r['v'] != 5 or r['params'] != vectors:
                        raise ValueError('recorded parameters differ from actual runtime')
                    gold = r['limits'][0] + sum(k[8] for k, d in zip(r['kit'], r['dispose']) if d == 2)
                    spent = sum(e[4] for e in r['plan'])
                    if not 0 <= spent <= gold:
                        raise ValueError('plan overspends')
                    spends.append(spent)
                    retained.append(gold - spent)
                item.update(outcome=summary['stop_reason'], snapshots=len(records),
                            median_planned_spend=statistics.median(spends) if spends else None,
                            median_retained_gold=statistics.median(retained) if retained else None,
                            channels={name: digest(out / name) for name in ('keys.log', 'messages.log', 'state.log')},
                            gameplay_summary={k: v for k, v in summary.items() if k not in ('wall_seconds', 'effective_settings')})
        except FileNotFoundError as exc:
            item.update(outcome='incomplete' if out.exists() else 'not_run', error=str(exc))
        except (ValueError, KeyError) as exc:
            item.update(outcome='config_error', error=str(exc))
        results.append(item)
        groups[(cell['arm'], cell['class'])].append(item)
        paired[(cell['class'], cell['seed'])][cell['arm']] = item
    comparisons = []
    for (cls, seed), arms in paired.items():
        reference = arms.get('reference')
        for arm, row in arms.items():
            if arm == 'reference' or reference is None:
                continue
            valid = 'channels' in reference and 'channels' in row
            comparisons.append({'class': cls, 'seed': seed, 'arm': arm,
                                'comparable': valid,
                                'gameplay_identical': valid and reference['channels'] == row['channels'] and reference['gameplay_summary'] == row['gameplay_summary']})
    per_class = []
    for (arm, cls), items in groups.items():
        per_class.append({'arm': arm, 'class': cls, 'planned_cells': len(items),
                          'outcomes': dict(Counter(x['outcome'] for x in items)),
                          'snapshots': sum(x.get('snapshots', 0) for x in items),
                          'median_planned_spend': statistics.median([x['median_planned_spend'] for x in items if x.get('median_planned_spend') is not None])
                          if any(x.get('median_planned_spend') is not None for x in items) else None})
    return {'schema': 'borg-rs/town-class-report@1', 'purpose': frozen['purpose'],
            'planned_cells': len(frozen['cells']), 'retained_cells': len(results),
            'outcomes': dict(Counter(x['outcome'] for x in results)),
            'per_class': per_class, 'paired_comparisons': comparisons, 'cells': results,
            'warning': 'Snapshot spending is diagnostic, not an objective for choosing class winners; all failures/incomplete cells remain visible.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(report(args.root), indent=2))


if __name__ == '__main__':
    main()
