#!/usr/bin/env python3
"""Retained native negative controls. Derived copies never overwrite originals."""
import gzip
import json
from pathlib import Path
import shutil
import sys

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools'))
import borg_runtime as runtime


def main():
    original,output=map(lambda x:Path(x).resolve(),sys.argv[1:3])
    output.mkdir(parents=True,exist_ok=False)
    runtime.validate(original)
    results=[]
    cases=['seed','missing-build','build','data-hash','config-hash','input-missing','input-altered','frame-hash',
           'rng-resigned','frame-midstream-resigned','native-save-hash','footer-count-resigned',
           'schema','path','required-stream-removed','crash','active','config-resigned','data-resigned']
    if len(sys.argv) > 3:
        cases.append('c-only')
    for name in cases:
        folder=output/name;bundle=folder/'recording';shutil.copytree(original,bundle)
        manifest=json.loads((bundle/'recording.json').read_text())
        def flip(path):
            data=bytearray(path.read_bytes()); data[len(data)//2]^=1; path.write_bytes(data)
        resign=False
        if name=='c-only':
            shutil.copy2(Path(sys.argv[3]).resolve(), bundle/'runtime/game/angband')
            manifest['build_id']['binary_sha256']=runtime.sha(bundle/'runtime/game/angband')
            identity=bundle/'runtime/build-id.json'
            if identity.exists():runtime.write(identity,manifest['build_id'])
            for file in manifest.get('runtime_sha256',{}):manifest['runtime_sha256'][file]=runtime.sha(bundle/file)
            resign=True
        elif name=='missing-build':manifest.pop('build_id',None)
        elif name=='seed': manifest['profile']['seed']^=1
        elif name=='build': manifest['build_id']['binary_sha256']='0'*64
        elif name=='data-hash': flip(bundle/'runtime/game/lib/gamedata/monster.txt')
        elif name=='config-hash': flip(bundle/'runtime/harness/borg.txt')
        elif name=='input-missing': (bundle/'run/native/events.txt').unlink()
        elif name=='input-altered': flip(bundle/'run/native/events.txt')
        elif name=='frame-hash': flip(bundle/'run/native/frames.jsonl')
        elif name in ('rng-resigned','frame-midstream-resigned'):
            p=bundle/'run/native/frames.jsonl';rows=p.read_text().splitlines(keepends=True)
            index=0 if name=='rng-resigned' else len(rows)//2
            row=json.loads(rows[index]);row['rng']='0'*256 if name=='rng-resigned' else row['rng']
            if name=='frame-midstream-resigned':row['hp']+=1
            rows[index]=json.dumps(row,separators=(',',':'))+'\n';p.write_text(''.join(rows))
            with gzip.open(bundle/'frames.jsonl.gz','wb') as stream:stream.write(p.read_bytes())
            resign=True
        elif name=='native-save-hash': flip(bundle/'run/final.save')
        elif name=='footer-count-resigned':
            p=bundle/'run/native/record-complete.json';value=json.loads(p.read_text());value['frames']+=1;runtime.write(p,value);resign=True
        elif name=='schema': manifest['schema']='borg-recording@1'
        elif name=='path': manifest['files_sha256']['../outside']='0'*64
        elif name=='required-stream-removed':
            del manifest['files_sha256']['run/state.log'];(bundle/'run/state.log').unlink()
        elif name in ('crash','active'): manifest['status']='failed' if name=='crash' else 'running'
        elif name=='config-resigned':
            p=bundle/'runtime/harness/borg.txt';p.write_text(p.read_text().replace('borg_uses_swaps = TRUE','borg_uses_swaps = FALSE'));resign=True
        elif name=='data-resigned':
            p=bundle/'runtime/game/lib/gamedata/monster.txt';text=p.read_text();p.write_text(text.replace('name:Farmer Maggot','name:Changed Maggot'));resign=True
        if resign:
            for path,digest in list(manifest['files_sha256'].items()):
                manifest['files_sha256'][path]=runtime.sha(bundle/path)
        runtime.write(bundle/'recording.json',manifest)
        proof=folder/'verification'
        try:
            runtime.verify(bundle,output=proof,timeout=120)
        except (OSError,ValueError,KeyError,AssertionError) as error:
            result={'case':name,'rejected':True,'error':str(error),'scope':'derived corruption; not a natural game'}
            if not proof.exists():proof.mkdir()
            runtime.write(proof/'negative-control.json',result)
        else:
            result={'case':name,'rejected':False,'scope':'derived corruption unexpectedly accepted'}
        results.append(result)
        print(name,'REJECTED' if result['rejected'] else 'UNEXPECTED PASS',flush=True)
    runtime.validate(original)
    runtime.write(output/'results.json',results)
    if not all(x['rejected'] for x in results):raise ValueError('At least one negative control was accepted')
    print('PASS',len(results),'native negative controls; original hashes unchanged',flush=True)


if __name__=='__main__':main()
