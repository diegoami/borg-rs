#!/usr/bin/env python3
"""Same frozen Rust binary/seed/options: recorder on versus observer-only.

No AI decisions/RNG are injected. Every result and failed comparison is retained.
"""
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools'))
import borg_runtime as runtime
from toolkit_common import native_lock


def main():
    collection,output=map(lambda x:Path(x).resolve(),sys.argv[1:])
    output.mkdir(parents=True,exist_ok=False)
    plan=json.loads((collection/'collection.json').read_text())
    results=[]
    with native_lock():
        for i,entry in enumerate(plan['runs']):
            bundle=Path(entry['recording']); runtime.validate(bundle)
            record=json.loads((bundle/'recording.json').read_text())
            run=output/f'case-{i}'
            process=runtime.frozen_run(bundle,run,timeout=120,native_recording=False)
            (output/f'console-{i}.log').write_text(process.stdout)
            channels={name:runtime.sha(bundle/'run'/name)==runtime.sha(run/name) for name in ('keys.log','messages.log','state.log','frames.jsonl','final.save','config.txt')} if not process.returncode else {}
            before=record['summary'].copy(); after=json.loads((run/'summary.json').read_text()) if (run/'summary.json').exists() else {}
            before.pop('wall_seconds',None); after.pop('wall_seconds',None)
            channels['summary']=before==after
            captures=[]
            for path in (bundle/'run/checkpoints').glob('*.save'):
                captures.append(runtime.sha(path)==runtime.sha(run/'checkpoints'/path.name))
            results.append({'original':str(bundle),'binary_sha256':record['build_id']['binary_sha256'],
                'process_exit':process.returncode,'channels':channels,'checkpoint_count':len(captures),
                'all_checkpoints_equal':all(captures),'scope':'same frozen Rust binary; native recorder off versus on; original observations/keys/RNG summary and native save anchors'})
        (output/'results.json').write_text(json.dumps(results,indent=2)+'\n')
        if not all(r['process_exit']==0 and all(r['channels'].values()) and r['all_checkpoints_equal'] for r in results):
            raise ValueError('Recorder non-interference control failed; retained evidence')
        print('PASS recorder non-interference:',len(results),'actual Rust pairs',flush=True)


if __name__=='__main__':main()
