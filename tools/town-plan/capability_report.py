#!/usr/bin/env python3
"""Report a retained capability validation receipt using completed native trades.

Input: receipt.json from the bounded matched run, including explicit paths for
reused baseline references. Reference repeats are never pooled as extra lives.
"""
import argparse
from collections import defaultdict
import csv
import hashlib
import json
from pathlib import Path
import statistics
from transactions import parse_run, md_cell


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def report(root, out):
    records = json.loads((root/'receipt.json').read_text())
    out.mkdir(parents=True, exist_ok=True)
    for label in ('candidate','baseline'):
        runtime=root/'runtime'/label
        build=dict(line.split('=',1) for line in (runtime/'BUILD_ID').read_text().splitlines() if '=' in line)
        assert sha(runtime/'game/angband') == build['binary_sha256']
    rows=[];unconfirmed=[];cells=[]
    for cell in records:
        assert cell['returncode']==0
        p=Path(cell['out']); summary=json.loads((p/'summary.json').read_text())
        assert summary == cell['summary']
        assert all(sha(p/name)==digest for name,digest in cell['hashes'].items())
        assert summary['effective_settings']['borg_town_plan']==cell['active']
        assert summary['effective_settings']['borg_trip_policy']==cell['active']
        if cell['arm'] not in ('live','baseline-live'):continue
        ev,missing=parse_run(p,cell['arm'],p.name,summary['class'])
        buys=[r for r in ev if r['kind']=='buy']
        assert all(r['wallet_verified'] and r['plan_seq'] is not None for r in buys)
        rows+=ev;unconfirmed += [u|{'arm':cell['arm'],'class':summary['class']} for u in missing]
        messages=(p/'messages.log').read_text(errors='replace')
        cells.append({'arm':cell['arm'],'class':summary['class'],'seed':cell['seed'],
                      'outcome':summary['stop_reason'],'dead':summary['dead'],
                      'level':summary['clevel'],'max_depth':summary['max_depth'],
                      'game_turns':summary['game_turns'],'cause':summary.get('death_cause'),
                      'purchases':len(buys),'gold_spent':sum(r['gold_spent'] for r in buys),
                      'oil_bought':sum(r['quantity'] for r in buys if 'oil' in r['item'].lower()),
                      'oil_hit_messages':messages.count('Your Flask of oil hits '),
                      'out':str(p),'hashes':cell['hashes']})
    (out/'cells.json').write_text(json.dumps(cells,indent=2)+'\n')
    (out/'unconfirmed.json').write_text(json.dumps(unconfirmed,indent=2)+'\n')
    with (out/'transactions.csv').open('w',newline='') as f:
        writer=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator="\n");writer.writeheader();writer.writerows(rows)
    table=['| Arm | Starts | Deaths | Mean CL | Mean max DL | DL3+ | Purchases | Gold spent |',
           '| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
    for arm in ('baseline-live','live'):
        subset=[c for c in cells if c['arm']==arm]
        table.append(f"| {arm} | {len(subset)} | {sum(c['dead'] for c in subset)} | {statistics.mean(c['level'] for c in subset):.2f} | {statistics.mean(c['max_depth'] for c in subset):.2f} | {sum(c['max_depth']>=3 for c in subset)} | {sum(c['purchases'] for c in subset)} | {sum(c['gold_spent'] for c in subset)} |")
    (out/'table.md').write_text('\n'.join(table)+'\n')
    shopping=['# Confirmed purchases by class','',
              'One Human start per class, seeds 501–509, 30000 game turns. Both policies enabled. Native retail receipts and matching wallet deductions are required. Plans and unconfirmed commands are not counted.','',
              '| Class | Baseline ledger | Capability ledger | New oil units | Native oil-hit messages |',
              '| --- | --- | --- | ---: | ---: |']
    for cls in dict.fromkeys(c['class'] for c in cells):
        current=next(c for c in cells if c['arm']=='live' and c['class']==cls)
        shopping.append(f"| {cls} | [old](baseline-live/{cls.lower()}.md) | [new](live/{cls.lower()}.md) | {current['oil_bought']} | {current['oil_hit_messages']} |")
        for arm in ('baseline-live','live'):
            subset=[r for r in rows if r['arm']==arm and r['class']==cls]
            totals=defaultdict(lambda:[0,0])
            for r in subset:
                if r['kind']=='buy': totals[r['item']][0]+=r['quantity'];totals[r['item']][1]+=r['gold_spent']
            lines=[f'# {cls}: {arm}','','One matched start, 30000-turn cap. Completed retail purchases and confirmed normal-executor Home transfers follow.','',
                   '| Item | Units bought | Gold spent |','| --- | ---: | ---: |']
            for name,(units,gold) in sorted(totals.items()):lines.append(f'| {md_cell(name)} | {units} | {gold} |')
            lines+=['','| Turn | Action | Item | Qty | Gold paid | Purse before → after | Capability |',
                    '| ---: | --- | --- | ---: | ---: | --- | --- |']
            for r in subset:
                lines.append('| '+' | '.join(map(md_cell,[r['game_turn'],r['kind'],r['item'],r['quantity'],r['gold_spent'],f"{r['gold_before']} → {r['gold_after']}",r['need_mapping'] or 'transfer']))+' |')
            folder=out/arm;folder.mkdir(exist_ok=True)
            (folder/f'{cls.lower()}.md').write_text('\n'.join(lines)+'\n')
    shopping+=['','[All confirmed transactions](transactions.csv) · [Unconfirmed commands](unconfirmed.json).',
               'Home transfers through the normal trade executors are included separately. Direct gear auto-swaps are not claimed complete. Home withdrawal quantities are explicitly inferred from observed stock deltas or left unknown. Oil-hit counts are native messages, not a damage-total estimate.']
    (out/'shopping.md').write_text('\n'.join(shopping)+'\n')
    deaths=['# Natural deaths in the paired wiring cohort','',
            'Every death in the two nine-start cohorts is listed. Native key/message/state streams and executable identities are retained at the source paths in cells.json. These are harness traces, not native movies; exact world-turn movie frames are unavailable. No causal defect or counterfactual survival is established.','']
    for c in cells:
        if not c['dead']:continue
        p=Path(c['out']);tail=(p/'messages.log').read_text(errors='replace').splitlines()[-70:]
        deaths += [f"## {c['arm']}: {c['class']}, seed {c['seed']}",'',
                   f"Native cause: {c['cause']}. Turn {c['game_turns']}; level {c['level']}; deepest DL{c['max_depth']}.",'',
                   'Assessment: lethal encounter observed. The trace shows the actual tactical actions; the shopping ledger shows preceding trades. Inventory at a prior town visit does not establish unused supplies at death. Separating spending priorities from unchanged tactical decisions needs a controlled counterfactual, not an attribution from death alone.','',
                   '```text',*tail,'```','']
    (out/'deaths.md').write_text('\n'.join(deaths).rstrip()+'\n')
    return {'cells':len(cells),'confirmed_buys':sum(r['kind']=='buy' for r in rows),
            'wallet_checks':sum(r['kind']=='buy' and r['wallet_verified'] for r in rows),
            'unconfirmed_commands':len(unconfirmed)}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,required=True);parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args();print(json.dumps(report(args.root,args.out),indent=2))


if __name__=='__main__':main()
