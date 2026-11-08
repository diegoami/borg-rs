#!/usr/bin/env python3
"""Disposable fixed/random/named continuation and save-publication controls."""
import json
import os
from pathlib import Path
import shutil
import sys
from types import SimpleNamespace
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools'))
import borg_runtime as runtime
import borg_saves


def main():
    visible,out=map(lambda x:Path(x).resolve(),sys.argv[1:])
    out.mkdir(parents=True,exist_ok=False);(out/'save').mkdir()
    os.environ['BORG_PLAY_DATA_DIR']=str(out)
    os.environ['BORG_DEATH_RECORDS_DIR']=str(out/'deaths')
    results=[]
    def options(name,new=False,seed=42,race=None,cls=None,**kw):
        return SimpleNamespace(character=name,new=new,from_backup=kw.get('from_backup'),seed=seed,race=race,cls=cls,
            turns=300,keys=1000,wall_seconds=30,stall_seconds=5,checkpoint_every=100)
    for label,args in [('fixed',options('Fixed',True,race=0,cls=0)),
                       ('continuation',options('Fixed',seed=999)),
                       ('native-random',options('Random',True,seed=2624955987))]:
        bundle=runtime.record_run(args);record=runtime.validate(bundle)
        proof=runtime.verify(bundle,output=out/(label+'-verification'),timeout=120)
        if record['origin']:
            first=json.loads((bundle/'run/native/frames.jsonl').read_text().splitlines()[0])
            assert first['rng']==record['origin']['rng'] and first['rng_index']==record['origin']['rng_index']
        results.append({'case':label,'bundle':str(bundle),'verified':proof['verified'],'origin':record['origin'],'summary':record['summary']})
    shutil.copy2(visible,out/'save/FromVisible')
    bundle=runtime.record_run(options('FromVisible'));record=runtime.validate(bundle)
    proof=runtime.verify(bundle,output=out/'from-visible-verification',timeout=120)
    first=json.loads((bundle/'run/native/frames.jsonl').read_text().splitlines()[0])
    assert first['rng']==record['origin']['rng'] and first['rng_index']==record['origin']['rng_index']
    assert record['profile']['turn_limit']==record['origin']['turn']+300
    results.append({'case':'same-visible-save','bundle':str(bundle),'verified':proof['verified'],'saved_rng_preserved':True})
    save=out/'save/Fixed';folder=out/'save/backups/Fixed'
    corrupt=folder/'save.5';corrupt.write_bytes(b'corrupt backup native preflight control')
    before={p.name:runtime.sha(p) for p in folder.iterdir() if p.is_file()};named=runtime.sha(save)
    try:runtime.record_run(options('Fixed',from_backup=5))
    except ValueError as error:rejected=str(error)
    else:raise ValueError('Corrupt backup unexpectedly accepted')
    assert runtime.sha(save)==named and before=={p.name:runtime.sha(p) for p in folder.iterdir() if p.is_file()}
    results.append({'case':'corrupt-backup','rejected':True,'unchanged':True,'error':rejected})
    bundle=runtime.record_run(options('Fixed',from_backup=1));record=runtime.validate(bundle)
    preserved=list(folder.glob('before-restore-*.save'))
    assert preserved and any(runtime.sha(p)==named for p in preserved)
    results.append({'case':'explicit-restore','bundle':str(bundle),'preserved_prior_save':True})
    publish=borg_saves.publish
    outside=save.read_bytes()+b'outside writer control'
    def external_writer(target,final,initial):
        target.write_bytes(outside)
        return publish(target,final,initial)
    with patch.object(borg_saves,'publish',side_effect=external_writer):
        try:runtime.record_run(options('Fixed'))
        except ValueError as error:refused=str(error)
        else:raise ValueError('Noncooperating writer was overwritten')
    assert save.read_bytes()==outside
    results.append({'case':'fault-injected-outside-writer','refused':True,'preserved_external_bytes':True,'error':refused})
    runtime.write(out/'results.json',results)
    print('PASS: 5 successful native named sessions; corrupt backup and outside-writer publication refused',flush=True)


if __name__=='__main__':main()
