#!/usr/bin/env python3
"""Finite C/C/Rust smoke comparison on one identical frozen instrumented source.

This parity check is separate from frozen Rust self-replay. No historical oracle
or build is updated. Source: a completed toolkit collection's prepared manifest.
"""
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools'))
import borg_runtime as runtime
from toolkit_common import native_lock


def main():
    collection, output = map(lambda x:Path(x).resolve(),sys.argv[1:])
    output.mkdir(parents=True,exist_ok=False)
    plan=json.loads((collection/'collection.json').read_text())
    prepared=json.loads(Path(plan['prepared_manifest']).read_text())
    rust=Path(prepared['binary']); directory=output/'c-build'
    cache=(Path(prepared['build_directory'])/'CMakeCache.txt').read_text().splitlines()
    source=Path(next(line.split('=',1)[1] for line in cache if line.startswith('CMAKE_HOME_DIRECTORY:INTERNAL='))).parent
    build_env={**os.environ,'GIT_CEILING_DIRECTORIES':str(source)}
    with native_lock():
        commands=[['cmake','-S',str(source/'angband'),'-B',str(directory),'-DBORG_IMPL=c',
            '-DSUPPORT_BORG=ON','-DSUPPORT_BORG_HARNESS=ON','-DSUPPORT_BORG_RECORDING=ON',
            '-DSUPPORT_TEST_FRONTEND=ON','-DSUPPORT_X11_FRONTEND=ON','-DCMAKE_BUILD_TYPE=Release',
            '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON','-DSC_INSTALL=ON','-DBUILD_DOC=OFF'],
            ['cmake','--build',str(directory),'--target','OurExecutable','--parallel','4']]
        for i,cmd in enumerate(commands):
            with (output/f'build-{i}.log').open('w') as log:
                subprocess.run(cmd,env=build_env,stdout=log,stderr=subprocess.STDOUT,check=True)
        results=[]
        for label,binary in [('c1',directory/'game/angband'),('c2',directory/'game/angband'),('rust',rust)]:
            for name,seed,race,cls,turns in [('warrior',101,0,0,20000),('mage',102,2,1,20000),('priest',103,5,3,20000)]:
                run=output/label/name
                env={k:v for k,v in os.environ.items() if not k.startswith(('BORG_','ANGBAND_','LLVM_PROFILE'))}
                env.update(ANGBAND_BIN=str(binary),BORG_IMPL='rust' if label=='rust' else 'c',
                    BORGH_DATA_ROOT=str(source/'angband/lib'),SAVE_ROOT=str(source/'harness/saves'),
                    RUN_TIMEOUT_SECONDS='120',CHECKPOINT_EVERY='0')
                with (output/f'{label}-{name}.log').open('w') as log:
                    subprocess.run([str(ROOT/'harness/run.sh'),str(run),str(seed),str(race),str(cls),str(turns),'menu,terminal-save,frames'],env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
                results.append({'backend':label,'case':name,'summary':json.loads((run/'summary.json').read_text())})
        comparisons=[]
        for name in ('warrior','mage','priest'):
            for left,right in [('c1','c2'),('c1','rust')]:
                channels={f:runtime.sha(output/left/name/f)==runtime.sha(output/right/name/f) for f in ('keys.log','messages.log','state.log','frames.jsonl','final.save')}
                a,b=[json.loads((output/x/name/'summary.json').read_text()) for x in (left,right)]
                a.pop('wall_seconds',None);b.pop('wall_seconds',None)
                channels['summary']=a==b
                comparisons.append({'case':name,'left':left,'right':right,'channels':channels})
        env={**os.environ,'C_BUILD':str(directory),'RUST_BUILD':prepared['build_directory']}
        with (output/'no-c-borg.log').open('w') as log:
            audit=subprocess.run([str(ROOT/'harness/check-no-c-borg.sh')],env=env,stdout=log,stderr=subprocess.STDOUT)
        receipt={'source_commit':prepared['source_commit'],'source_inputs_sha256':prepared['source_inputs_sha256'],
            'overlay':prepared['build_id'].get('overlay_sha256'),'rust_binary_sha256':runtime.sha(rust),
            'c_binary_sha256':runtime.sha(directory/'game/angband'),'executions':results,'comparisons':comparisons,
            'no_c_borg_exit':audit.returncode,'scope':'three finite smoke cases; C/C reproducibility and C/Rust parity, not whole-private-state equality or survival benefit'}
        (output/'results.json').write_text(json.dumps(receipt,indent=2)+'\n')
        if not all(all(c['channels'].values()) for c in comparisons) or audit.returncode:
            raise ValueError('A parity/link control failed; see retained results/logs')
        print('PASS: 3 C/C + 3 C/Rust comparisons, 9 native executions; link audit passed',flush=True)


if __name__=='__main__':main()
