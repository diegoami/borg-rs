#!/usr/bin/env python3
"""Retained final native Rust replay, recorder and C/Rust parity qualification."""
import json
from pathlib import Path
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools'))
import borg_runtime as runtime
import borg_replay_viewer as viewer


def main():
    collection,output=map(lambda x:Path(x).resolve(),sys.argv[1:])
    output.mkdir(parents=True,exist_ok=False)
    plan=json.loads((collection/'collection.json').read_text())
    results=[]
    for index,entry in enumerate(plan['runs']):
        bundle=Path(entry['recording'])
        proof=runtime.verify(bundle,output=output/f'self-replay-{index}',timeout=120)
        frames=viewer.Frames(bundle)
        last=frames.get(len(frames.index)-1)[0]
        if entry['outcome'] in ('turn_cap','key_cap','wall_timeout'):
            prior=frames.get(len(frames.index)-2)[0]
            assert prior['phase']=='command' and prior['turn']==last['turn'], 'Living publication must follow a top-level native command boundary'
        frames.close()
        assert last['turn']==entry['final']['game_turns']
        if entry['outcome']=='death': assert last['dead'] and last['cause']==entry['cause']
        windows=None
        if entry.get('dossier'):
            case=json.loads((Path(entry['dossier'])/'record.json').read_text())
            windows=len(case['recording']['selected_original_frames'])
        results.append({'bundle':str(bundle),'verified':proof['verified'],'last_frame':{k:last[k] for k in ('turn','phase','hp','dead')},'exact_death_windows':windows})
    runtime.write(output/'self-replays.json',results)
    for name in ('noninterference','parity'):
        with (output/f'{name}.log').open('w') as log:
            subprocess.run([sys.executable,str(ROOT/f'harness/regress/toolkit/{name}.py'),str(collection),str(output/name)],stdout=log,stderr=subprocess.STDOUT,check=True)
    print('PASS: final 3 Rust self-replays; recorder pairs; distinct C/C/Rust smoke and link controls',flush=True)


if __name__=='__main__':main()
