from pathlib import Path
import json,sys,shutil,subprocess,os,hashlib
sys.path.insert(0,'/home/diego/projects/borg-rs/tools');import borg_gui as g
root=Path('/home/diego/projects/borg-rs');project=Path('/tmp/borg-rs-gui-tools');out=root/'harness/out/gui-tooling/observer-control-final';out.mkdir()
source=project/'angband/src/native-replay.c';original=source.read_text();needle='void record_frame(const char *phase, long decisions)\n{';assert needle in original
source.write_text(original.replace(needle,needle+'\n    return; /* disposable observer noninterference control */',1))
try:
 with (out/'mutant-build.log').open('w') as log:
  subprocess.run(['cmake','--build',str(project/'build/play-rust'),'--parallel','4'],check=True,stdout=log,stderr=subprocess.STDOUT)
 binary=out/'observer-disabled-angband';shutil.copy2(project/'build/play-rust/game/angband',binary)
finally:
 source.write_text(original)
 with (out/'restore-build.log').open('w') as log:
  subprocess.run(['cmake','--build',str(project/'build/play-rust'),'--parallel','4'],check=True,stdout=log,stderr=subprocess.STDOUT)
case=next(r for r in json.loads((root/'harness/out/gui-tooling/integration-final/results.json').read_text()) if r['case']=='borg-intervene-reload-resize');bundle=Path(case['bundle']);record=g.validate(bundle)
profile=out/'profile';shutil.copytree(bundle/'initial-profile',profile);run=out/'run';run.mkdir();geometry=json.loads((bundle/'run/geometry.json').read_text());env=g.environment();env.update(BORG_RECORD_DIR=str(run),BORG_REPLAY_DIR=str(bundle/'run'),BORG_REPLAY_SYS='x11',TZ=':'+str(bundle/'runtime/timezone'))
for k in ('width','height','key_size','soft_cursor','complex_input'):env['BORG_REPLAY_'+k.upper()]=str(geometry[k])
frozen=bundle/'runtime/game'
result=subprocess.run(g.command(binary,frozen/'lib',profile,record['character'],[],'test'),cwd=frozen,env=env,capture_output=True,text=True,timeout=30,stdin=subprocess.DEVNULL)
assert result.returncode==0,result.stderr
proof={'source':str(bundle),'observer_enabled_binary':g.runtime.sha(frozen/'angband'),'observer_disabled_binary':g.runtime.sha(binary),'compared':{},'private_mutant_only':'record_frame early return; clocks/terminal/file input capture and actual Rust decision code unchanged'}
for name in ('events.txt','clock.txt','pid.txt','actions.txt','file-inputs.txt','geometry.json'):
 a,b=g.runtime.sha(bundle/'run'/name),g.runtime.sha(run/name);assert a==b,name;proof['compared'][name]=a
assert (run/'frames.jsonl').stat().st_size==0
final=profile/'save'/record['character'];assert g.runtime.sha(final)==record['final_sha256'];proof['compared']['final.save']=g.runtime.sha(final)
original_footer=json.loads((bundle/'run/record-complete.json').read_text());footer=json.loads((run/'record-complete.json').read_text());assert original_footer['inputs']==footer['inputs'] and footer['frames']==0
proof['all_inputs_consumed']=True
(out/'result.json').write_text(json.dumps(proof,indent=2)+'\n');print(json.dumps(proof,indent=2))
