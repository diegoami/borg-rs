#!/usr/bin/env python3
"""Confirmed town ledger from original messages, checked against per-key gold.

Never turn a plan or an executor log into a completed purchase. Retail buys
require the game's 'You bought ... for N gold' receipt. Home stores and market
sales require their own native confirmation. Keep unconfirmed commands separate.
"""
import argparse
from collections import defaultdict, Counter
import csv
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
SHOPS = ['General Store','Armoury','Weaponsmith','Bookseller','Alchemist','Magic Shop','Black Market','Home']
NEEDS = ['Light','Food','Healing','Poison','Escape','Knowledge','Power','Recall / Trip']
LINE = re.compile(r'^(\d+) (\d+) (\d+) (.*)$')
EXECUTE = re.compile(r'^# Town plan executing action (\d+) at shop (-?\d+) slot (\d+)')
CAP_NAMES = json.loads(re.search(r'pub const NAMES:.*?= (\[.*?\]);', (ROOT/'borg-rs/src/borg_town_capability.rs').read_text(), re.S)[1])
INTENT = re.compile(r'^# Town plan live: action=(\d+) shop=(-?\d+) slot=(\d+) quantity=(\d+) need=(-?\d+)')


def quantity(text):
    if re.match(r'^(?:a|an|the) ', text, re.I): return 1
    m = re.match(r'^(\d+) ',text)
    # obj_desc_name_prefix prints an explicit number for every stack >1;
    # armour/mass nouns can have no article when the physical count is one.
    return int(m[1]) if m else 1


def label(text):
    text = re.sub(r'\s*\{(?:unseen|\?\?)\}', '', text).strip().rstrip('.')
    text = re.sub(r'^(?:a|an|the|\d+) ', '', text, flags=re.I)
    for plural,singular in [('Iron Shots','Iron Shot'),('Pints','Pint'),('Potions','Potion'),('Scrolls','Scroll'),('Rations','Ration'),
                            ('Wooden Torches','Wooden Torch'),('Lanterns','Lantern'),
                            ('Flasks','Flask'),('Cloaks','Cloak'),('Daggers','Dagger'),
                            ('Rings','Ring'),('Amulets','Amulet'),('Wands','Wand'),('Staves','Staff'),
                            ('Mushrooms','Mushroom'),('Arrows','Arrow'),('Bolts','Bolt'),('Shots','Shot')]:
        if text.startswith(plural): text=singular+text[len(plural):];break
    return text


def states(path):
    result=[]
    for line in path.read_text().splitlines():
        m=re.search(r'\bau(-?\d+)\b',line);lv=re.search(r'\blv(\d+)\b',line)
        result.append({'gold':int(m[1]) if m else None,'level':int(lv[1]) if lv else None})
    return result


def components(record):
    if record is None:return None,None
    shop,slot=record['act'][:2]
    fact=next((w for w in record['wares'] if w[:2]==[shop,slot]),None)
    if fact is None:return None,None
    known=bool(fact[8] & (1<<5))
    power=any(t[0]==shop and t[1]==slot and t[3]>0 for t in record['tryon'])
    if record.get('v', 0) >= 7:
        ids = sorted({row[2] for row in record['capability_supplies'] if row[:2] == [shop, slot]})
        roles = [CAP_NAMES[i].capitalize() if i < len(CAP_NAMES) else f'Capability {i}' for i in ids]
        if power: roles.append('Power')
        role = ' + '.join(roles) or ('Knowledge only' if known else 'No positive capability')
    else:
        role=NEEDS[fact[6]] if fact[6]>=0 else 'Power' if power else 'Knowledge only' if known else 'Unmapped'
    return role,known


def parse_run(path, arm, name, cls):
    frames=states(path/'state.log')
    records=[json.loads(line) for line in (path/'plan.jsonl').read_text().splitlines()] if (path/'plan.jsonl').exists() else []
    cursor=0;intent=None;store=None;pending=None;events=[];unconfirmed=[]
    def frame(key):return frames[key] if 0<=key<len(frames) else {'gold':None,'level':None}
    def event(kind,key,turn,native,qty,gold,where):
        nonlocal pending
        meta=pending or {};rec=meta.get('record')
        desc=meta.get('description',native)
        role,learning=components(rec) if kind=='buy' else ((NEEDS[rec['for']] if rec and 0 <= rec['for'] < len(NEEDS) else 'Unmapped (live)' if rec else None),None)
        before=frame(meta.get('key',key-1))['gold'];after=frame(key)['gold']
        wallet_ok=(before is not None and after is not None and before-after==gold) if kind=='buy' else None
        row={'arm':arm,'run':name,'class':cls,'kind':kind,'game_turn':turn,'key_index':key,
             'shop':SHOPS[where] if where is not None else 'Unknown','item':label(desc),
             'shop_description':desc,'native_receipt':native,'inventory_after':None,'quantity':qty,'gold_spent':gold if kind=='buy' else 0,
             'gold_received':gold if kind=='sell' else 0,'gold_before':before,'gold_after':after,
             'wallet_verified':wallet_ok,'character_level':frame(key)['level'],
             'unseen_or_unknown_at_purchase':bool('{unseen}' in desc or '{??}' in desc) if kind=='buy' else None,
             'need_mapping':role,'knowledge_component':learning,
             'plan_seq':rec['seq'] if rec else None,'message_line':meta.get('confirmation_line'),
             'quantity_evidence':'native receipt' if kind!='home_take' else meta.get('quantity_evidence','unknown'),
             'source':str(path/'messages.log')}
        if kind!='buy': row['item']=label(native)
        if kind=='buy' and not wallet_ok:raise ValueError(f'purchase wallet mismatch {path} key {key}: {before}->{after}, receipt {gold}')
        events.append(row);pending=None
    for line_no,line in enumerate((path/'messages.log').read_text(errors='replace').splitlines(),1):
        m=LINE.match(line.rstrip('\n'))
        if not m:continue
        key,turn=int(m[1]),int(m[2]);text=m[4]
        if pending and key-pending['key']>24:
            unconfirmed.append(pending);pending=None
        entered=re.search(r"# Entering a '(\d+)' shop",text)
        preferred=re.search(r"# Currently in preferred store '(\d+)'",text)
        if entered:store=int(entered[1])
        if preferred:store=int(preferred[1])-1
        live=INTENT.match(text)
        combined=EXECUTE.match(text)
        if live or combined:
            if live: code,shop,slot,q,need=map(int,live.groups())
            else:
                code,shop,slot=map(int,combined.groups());q=1
            intent=None
            for idx in range(cursor,len(records)):
                r=records[idx]
                rshop=7 if r['paction'] in (2,4,5) else r['act'][0]
                if r['gturn']==turn and r['paction']==code and rshop==shop and r['act'][1]==slot and r['act'][2]==q:
                    intent={'record':r,'record_index':idx,'quantity':q,'code':code};cursor=idx+1;break
            if intent and intent['record'].get('v',0)>=9 and code in (1,2,4):
                if pending: unconfirmed.append(pending)
                pending={'command':'Buying','key':key,'turn':turn,'store':shop,**intent}
            continue
        command=re.match(r'^# (Buying|Selling) (.+)$',text)
        if command and not command[2].startswith(("'",'time ')):
            # Schema 9 emits Buying only after native progress. The earlier
            # executor intent supplies the wallet anchor; this later log adds
            # the original shop description, never a second transaction.
            if command[1]=='Buying' and intent and intent['record'].get('v',0)>=9:
                if events and events[-1]['plan_seq']==intent['record']['seq'] and events[-1]['kind'] in ('buy','home_take'):
                    events[-1]['shop_description']=command[2].removesuffix('.')
                    if events[-1]['kind']=='buy':
                        events[-1]['unseen_or_unknown_at_purchase']=any(x in command[2] for x in ('{unseen}','{??}'))
                continue
            if pending:unconfirmed.append(pending)
            pending={'command':command[1],'description':command[2].removesuffix('.'),
                     'key':key,'turn':turn,'store':store,**(intent or {})}
            continue
        buy=re.match(r'^You bought (.+) for (\d+) gold\.$',text)
        if buy:
            if pending:pending['confirmation_line']=line_no
            event('buy',key,turn,buy[1],quantity(buy[1]),int(buy[2]),store)
            continue
        inventory=re.match(r'^You have (.+) \([^)]+\)\.$',text)
        if inventory and events and events[-1]['kind']=='buy' and events[-1]['key_index']==key:
            events[-1]['inventory_after']=inventory[1]
            events[-1]['item']=label(inventory[1])
        if not pending:continue
        pending['confirmation_line']=line_no
        sold=re.match(r'^You sold (.+) \([^)]+\) for (\d+) gold\.$',text)
        had=re.match(r'^You had (.+) \([^)]+\)\.$',text)
        dropped=re.match(r'^You drop (.+) \([^)]+\)\.$',text)
        take=re.match(r'^You have (.+) \([^)]+\)\.$',text)
        if pending['command']=='Selling' and store!=7 and (sold or had):
            native=(sold or had)[1];event('sell',key,turn,native,quantity(native),int(sold[2]) if sold else 0,store)
        elif pending['command']=='Selling' and store==7 and dropped:
            event('home_store',key,turn,dropped[1],quantity(dropped[1]),0,7)
        elif pending['command']=='Buying' and store==7 and take:
            q=None
            if pending.get('record') is not None:
                idx=pending['record_index'];r=records[idx]
                slot=r['act'][1];home=next((k for k in r['kit'] if k[0]==1 and k[1]==slot),None)
                if home and idx+1<len(records):
                    before=sum(k[3] for k in r['kit'] if k[0]==1 and k[2]==home[2])
                    after=sum(k[3] for k in records[idx+1]['kit'] if k[0]==1 and k[2]==home[2])
                    if before>after:q=before-after;pending['quantity_evidence']='observed Home stock delta'
            else:
                # The legacy executor enters no quantity: UI default is one.
                # Keep this inference explicit, separate from native receipt qty.
                q=1;pending['quantity_evidence']='confirmed transfer; legacy key executor/UI default of one'
            event('home_take',key,turn,take[1],q,0,7)
    if pending:unconfirmed.append(pending)
    return events,unconfirmed


def aggregate(events):
    result=defaultdict(lambda:{'units':0,'transactions':0,'gold':0,'unseen':0,'unknown_quantity':0})
    for e in events:
        key=(e['arm'],e['class'],e['kind'],e['item'])
        a=result[key];a['transactions']+=1;a['units']+=e['quantity'] or 0
        a['unknown_quantity']+=e['quantity'] is None;a['gold']+=e['gold_spent'];a['unseen']+=bool(e['unseen_or_unknown_at_purchase'])
    return result


def md_cell(value):return str(value).replace('|','\\|').replace('\n',' ')


def write_report(out, events, unconfirmed, runs):
    out.mkdir(parents=True,exist_ok=True)
    with (out/'transactions.csv').open('w',newline='') as f:
        fields=list(events[0]);writer=csv.DictWriter(f,fieldnames=fields);writer.writeheader();writer.writerows(events)
    stats={a:{'confirmed_buys':sum(e['arm']==a and e['kind']=='buy' for e in events),
              'units_bought':sum(e['quantity'] or 0 for e in events if e['arm']==a and e['kind']=='buy'),
              'gold_spent':sum(e['gold_spent'] for e in events if e['arm']==a),
              'confirmed_home_stores':sum(e['arm']==a and e['kind']=='home_store' for e in events),
              'confirmed_home_takes':sum(e['arm']==a and e['kind']=='home_take' for e in events),
              'confirmed_market_sales':sum(e['arm']==a and e['kind']=='sell' for e in events)}
           for a in dict.fromkeys(r['arm'] for r in runs)}
    (out/'audit.json').write_text(json.dumps({'runs':runs,'totals':stats,
        'confirmed_buy_wallet_checks':sum(e['kind']=='buy' and e['wallet_verified'] for e in events),
        'unconfirmed_commands':[{'arm':u['arm'],'run':u['run'],'key':u['key'],'description':u['description'],'store':u['store']} for u in unconfirmed]},indent=2)+'\n')
    grouped=aggregate(events)
    for arm in stats:
        if arm=='no-shops':continue
        folder=out/arm;folder.mkdir(exist_ok=True)
        for cls in dict.fromkeys(r['class'] for r in runs):
            subset=[e for e in events if e['arm']==arm and e['class']==cls]
            lines=[f'# {cls}: {arm} town transactions','',
                   'Five full-suite starts, 100000 turns each. Purchases below require native shop receipts and matching recorded gold changes. Home movements are separate.','',
                   '| Item | Units bought | Gold spent | Confirmed purchases | Unseen/unknown purchases |',
                   '| --- | ---: | ---: | ---: | ---: |']
            values=[(key,a) for key,a in grouped.items() if key[:3]==(arm,cls,'buy')]
            for key,a in sorted(values,key=lambda x:(-x[1]['gold'],x[0][3])):
                lines.append(f'| {md_cell(key[3])} | {a["units"]} | {a["gold"]} | {a["transactions"]} | {a["unseen"]} |')
            lines+=['','## Chronological confirmed trades','',
                    'Home entries cost no gold. “Knowledge only” describes the current evaluator’s mapping, not a claim that the item has no tactical use. The CSV retains the exact game confirmation, including flavour names before identification.','']
            for run in [r['name'] for r in runs if r['arm']==arm and r['class']==cls]:
                lines+=[f'### {run}','','| Game turn | Action | Shop | Item | Qty | Gold | Purse before → after | Mapping / identification |',
                        '| ---: | --- | --- | --- | ---: | ---: | --- | --- |']
                for e in [x for x in subset if x['run']==run]:
                    mapping=(e['need_mapping'] or 'legacy / transfer')+(' + identification bonus' if e['knowledge_component'] else '')
                    q=e['quantity'] if e['quantity'] is not None else 'unknown'
                    lines.append('| '+ ' | '.join(map(md_cell,[e['game_turn'],e['kind'],e['shop'],e['item'],q,e['gold_spent'] if e['kind']=='buy' else e['gold_received'],f"{e['gold_before']} → {e['gold_after']}",mapping]))+' |')
            (folder/f'{cls.lower()}.md').write_text('\n'.join(lines)+'\n')
    lines=['# What the Borg actually bought in town','',
           'Confirmed transactions from the corrected live full suite: 45 starts per arm, 100000 turns, nine classes. Source runtime ac63c5e. Every retail purchase has a native “You bought … for … gold” receipt and a matching per-key purse decrease. These are executed purchases, not plans.','',
           '| Arm | Confirmed purchases | Units | Gold spent | Home deposits¹ | Home withdrawals¹ | Market sales |',
           '| --- | ---: | ---: | ---: | ---: | ---: | ---: |']
    for arm,m in stats.items():lines.append('| '+arm+' | '+' | '.join(str(m[k]) for k in ['confirmed_buys','units_bought','gold_spent','confirmed_home_stores','confirmed_home_takes','confirmed_market_sales'])+' |')
    example=[e for e in events if e['arm']=='utility-reference' and e['run']=='full-0-warrior-0'][:18]
    lines+=['','## Concrete example: Human Warrior, seed 1000','',
            'These are the first confirmed transactions, including the first seven purchases. Short dungeon detours occur between some town actions; this is not labelled as one uninterrupted visit.',
            '', '| Game turn | Action | Item | Qty | Gold paid | Purse before → after |',
            '| ---: | --- | --- | ---: | ---: | --- |']
    for e in example:
        lines.append('| '+' | '.join(map(md_cell,[e['game_turn'],e['kind'],e['item'],e['quantity'],e['gold_spent'],f"{e['gold_before']} → {e['gold_after']}"]))+' |')
    lines+=['', 'The trace shows Boldness, Detect Invisible and Blessing bought and then deposited. It also deposits two rations and shortly buys rations again. These behaviours are visible in the original receipts; the report does not hide them or claim that the planner is already efficient.','']
    lines+=['','## Open a class’s actual shopping ledger','','Each page has item totals and all confirmed retail purchases and the normal trade-executor sales/Home transfers in order, separated by seed.','',
            '| Class | Current | Utility reference | Cash priority 50 |','| --- | --- | --- | --- |']
    for cls in dict.fromkeys(r['class'] for r in runs):
        lines.append(f'| {cls} | '+ ' | '.join(f'[{a}]({a}/{cls.lower()}.md)' for a in ['current','utility-reference','utility-cash50'])+' |')
    for arm in ['current','utility-reference','utility-cash50']:
        lines+=['',f'## Most gold spent: {arm}','','| Item / variant | Units | Gold | Purchases |','| --- | ---: | ---: | ---: |']
        merged=defaultdict(lambda:[0,0,0])
        for e in events:
            if e['arm']==arm and e['kind']=='buy':
                a=merged[e['item']];a[0]+=e['quantity'] or 0;a[1]+=e['gold_spent'];a[2]+=1
        for name,a in sorted(merged.items(),key=lambda x:-x[1][1])[:25]:lines.append(f'| {md_cell(name)} | {a[0]} | {a[1]} | {a[2]} |')
    lines+=['','¹ Home columns cover confirmed transfers through the normal trade executors. The separate equipment optimiser can transfer worn gear directly without these commands; those auto-swaps are not claimed complete here. Other inventory messages are not automatically labelled as Home transfers.', '']
    lines+=['','## Evidence and limits','',
            '[Complete machine-readable transactions](transactions.csv) · [audit and original log checksums](audit.json).',
            'The engine prints a numeric quantity for every stack larger than one; an unprefixed armour/mass-noun receipt therefore means one physical item. Post-purchase inventory descriptions supply identified names/modifiers when available. Their resulting stack counts are never reused as purchase quantities.',
            'The ledger uses original messages/state logs from the corrected batch, not newly simulated visits. Inventory descriptions in Borg shop notes may be plural because they describe shop stock; quantities come from native receipts. Unknown flavour names in receipts are paired with the Borg’s contemporaneous shop description.',
            'Market “You had …” receipts mean zero selling proceeds under birth_no_selling. Home deposits require native “You drop …” confirmation after a deposit command; they are not sales. Home withdrawal quantities use observed Home stock deltas for live arms, or the explicitly labelled legacy default of one; an unverified quantity is left unknown. A Home stock delta is not inferred from a resulting pack total.',
            f'Unconfirmed commands retained separately: {len(unconfirmed)}. They are excluded from completed transaction totals. Source line numbers and exact confirmation text remain in the CSV. Gear variants retain their modifiers rather than being merged into an invented average item.','']
    (out/'README.md').write_text('\n'.join(lines))
    return stats


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,required=True);parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
    manifest=json.loads((args.root/'experiment.json').read_text());events=[];unconfirmed=[];runs=[]
    for cell in manifest['cells']:
        p=args.root/cell['arm']/cell['name']
        ev,miss=parse_run(p,cell['arm'],cell['name'],cell['class'])
        events+=ev;unconfirmed += [u|{'arm':cell['arm'],'run':cell['name']} for u in miss]
        runs.append({k:cell[k] for k in ['arm','name','class','seed','race_idx','class_idx']}|
                    {'messages_sha256':hashlib.sha256((p/'messages.log').read_bytes()).hexdigest(),
                     'state_sha256':hashlib.sha256((p/'state.log').read_bytes()).hexdigest()})
    stats=write_report(args.out,events,unconfirmed,runs)
    print(json.dumps(stats,indent=2));print('unconfirmed commands',len(unconfirmed))


if __name__=='__main__':main()
