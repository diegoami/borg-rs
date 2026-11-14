#!/usr/bin/env python3
"""Summarise every frozen local live-suite attempt and paired per-class outcomes."""
import argparse
from collections import Counter
import csv
import hashlib
import json
from pathlib import Path
import re
import statistics


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def diagnostics(path, cap):
    buying = selling = refusals = live = learning = home_buys = home_sales = 0
    store = None
    if (path/'messages.log').exists():
        with (path/'messages.log').open(errors='replace') as stream:
            for line in stream:
                match = re.search(r"Currently in preferred store '(\d+)'", line)
                if match: store = int(match[1])
                if '# Buying ' in line and not line.split('# Buying ',1)[1].startswith(("'", 'time ')):
                    buying += 1
                    learning += '{unseen}' in line or '{??}' in line
                    home_buys += store == 8
                if '# Selling ' in line and not line.split('# Selling ',1)[1].startswith("'"):
                    selling += 1
                    home_sales += store == 8
                refusals += 'not have enough gold' in line
                live += '# Town plan live:' in line
    previous = None; town = elapsed = 0
    if (path/'state.log').exists():
        with (path/'state.log').open() as stream:
            for line in stream:
                m = re.match(r'^(\d+) \S+ d(\d+)/', line)
                if not m: continue
                turn, depth = min(int(m[1]), cap), int(m[2])
                if previous is not None:
                    dt = max(0, turn-previous[0]); elapsed += dt
                    if previous[1] == 0: town += dt
                previous = turn, depth
    return {'buy_commands': buying, 'sell_commands': selling, 'insufficient_gold_messages': refusals,
            'live_intents': live, 'learning_buy_commands': learning, 'home_take_commands': home_buys, 'home_store_commands': home_sales, 'town_world_turns': town, 'observed_world_turns': elapsed,
            'town_fraction': town/elapsed if elapsed else None}


def metrics(rows):
    valid = [r for r in rows if r['outcome'] in ('death','turn_cap')]
    return {'attempts':len(rows),'valid':len(valid),'deaths':sum(r['outcome']=='death' for r in rows),
            'failures':dict(Counter(r['outcome'] for r in rows if r['outcome'] not in ('death','turn_cap'))),
            'mean_character_level':statistics.mean(r['clevel'] for r in valid) if valid else None,
            'mean_deepest_level':statistics.mean(r['max_depth'] for r in valid) if valid else None,
            'reached_dl3':sum(r['max_depth']>=3 for r in valid),
            'mean_last_observed_gold':statistics.mean(r['gold'] for r in valid) if valid else None,
            'mean_town_fraction':statistics.mean(r['diagnostics']['town_fraction'] for r in valid if r['diagnostics']['town_fraction'] is not None) if valid else None,
            'buy_commands':sum(r.get('diagnostics',{}).get('buy_commands',0) for r in rows),
            'sell_commands':sum(r.get('diagnostics',{}).get('sell_commands',0) for r in rows),
            'learning_buy_commands':sum(r.get('diagnostics',{}).get('learning_buy_commands',0) for r in rows),
            'home_take_commands':sum(r.get('diagnostics',{}).get('home_take_commands',0) for r in rows),
            'home_store_commands':sum(r.get('diagnostics',{}).get('home_store_commands',0) for r in rows),
            'insufficient_gold_messages':sum(r.get('diagnostics',{}).get('insufficient_gold_messages',0) for r in rows)}


def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--root',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
    manifest=json.loads((args.root/'experiment.json').read_text());results=[]
    for cell in manifest['cells']:
        path=args.root/cell['arm']/cell['name'];r={k:cell[k] for k in ('arm','name','seed','race_idx','class_idx','class','turn_cap','settings_sha256')}
        r['raw_directory']=str(path.resolve())
        try:
            process=json.loads((path/'process.json').read_text())
            if process['status']!='success' or process['exit_code']!=0:
                r.update(outcome='timeout' if process.get('timed_out') else 'harness_failure',process=process)
            else:
                s=json.loads((path/'summary.json').read_text())
                assert (s['seed'],s['race_idx'],s['class_idx'])==(cell['seed'],cell['race_idx'],cell['class_idx'])
                assert s['class']==cell['class']
                assert all(s['effective_settings'].get(k)==int(v) for k,v in cell['settings'].items())
                assert s['effective_settings']['borg_cheat_death']==0
                assert s['effective_settings']['borg_respawn_winners']==0
                assert process['executable_sha256_before']==process['executable_sha256_after']==manifest['build']['binary_sha256']
                outcome='death' if s['dead'] else 'turn_cap' if s['stop_reason']=='turns' else s['stop_reason']
                r.update(outcome=outcome,clevel=s['clevel'],max_depth=s['max_depth'],game_turns=s['game_turns'],
                         gold=s.get('gold',0),cause=s.get('death_cause'),summary_sha256=sha(path/'summary.json'),
                         config_sha256=sha(path/'user/borg.txt'),keys_sha256=sha(path/'keys.log'),
                         diagnostics=diagnostics(path,cell['turn_cap']))
                # summary has no gold on older harness pins: final state does.
                if 'gold' not in s:
                    with (path/'state.log').open() as f:
                        last=None
                        for line in f:
                            m=re.search(r'\bau(-?\d+)\b',line)
                            if m:last=int(m[1])
                    r['gold']=last
        except FileNotFoundError as exc:r.update(outcome='missing',error=str(exc))
        except (ValueError,KeyError,AssertionError) as exc:r.update(outcome='config_error',error=repr(exc))
        results.append(r)
    arms=list(dict.fromkeys(c['arm'] for c in manifest['cells']))
    table={a:metrics([r for r in results if r['arm']==a]) for a in arms}
    perclass={a:{cls:metrics([r for r in results if r['arm']==a and r['class']==cls]) for cls in dict.fromkeys(r['class'] for r in results)} for a in arms}
    controls={r['name']:r for r in results if r['arm']=='current'};paired={}
    for arm in arms[1:]:
        pairs=[(controls[r['name']],r) for r in results if r['arm']==arm and r['outcome'] in ('death','turn_cap') and controls[r['name']]['outcome'] in ('death','turn_cap')]
        paired[arm]={'pairs':len(pairs),'control_died_arm_alive':sum(a['outcome']=='death' and b['outcome']=='turn_cap' for a,b in pairs),
                     'control_alive_arm_died':sum(a['outcome']=='turn_cap' and b['outcome']=='death' for a,b in pairs),
                     'identical_keys':sum(a['keys_sha256']==b['keys_sha256'] for a,b in pairs),
                     'mean_depth_change':statistics.mean(b['max_depth']-a['max_depth'] for a,b in pairs) if pairs else None}
    out={'schema':'borg-rs/town-live-summary@1','source_commit':manifest['source_commit'],'runtime_inputs_sha256':manifest['runtime_inputs_sha256'],
         'binary_sha256':manifest['build']['binary_sha256'],'arms':table,'per_class':perclass,'paired':paired,'cells':results,
         'interpretation':'45 paired starts per arm, corrected menu birth, 100000 turns; exploratory, not an improvement claim'}
    args.out.mkdir(parents=True,exist_ok=True);(args.out/'summary.json').write_text(json.dumps(out,indent=2)+'\n')
    flat=[{k:v for k,v in r.items() if k not in ('diagnostics','process')} for r in results]
    with (args.out/'cells.csv').open('w',newline='') as f:
        fields=list(dict.fromkeys(k for r in flat for k in r));writer=csv.DictWriter(f,fieldnames=fields);writer.writeheader();writer.writerows(flat)
    lines=['| Arm | Deaths | Mean character level | Mean deepest level | Reached DL3+ | Valid | Failures |','| --- | ---: | ---: | ---: | ---: | ---: | --- |']
    for arm,m in table.items():
        level=f"{m['mean_character_level']:.2f}" if m['mean_character_level'] is not None else 'missing'
        depth=f"{m['mean_deepest_level']:.2f}" if m['mean_deepest_level'] is not None else 'missing'
        lines.append(f"| {arm} | {m['deaths']} | {level} | {depth} | {m['reached_dl3']} | {m['valid']}/{m['attempts']} | {m['failures'] or 'none'} |")
    (args.out/'table.md').write_text('\n'.join(lines)+'\n');print('\n'.join(lines))


if __name__=='__main__':main()
