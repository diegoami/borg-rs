from pathlib import Path
from argparse import Namespace
import json,os,sys,shutil
root=Path(__file__).resolve().parents[3];sys.path.insert(0,str(root/'tools'));import borg_runtime as runtime
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True);profile=out/'profile';(profile/'save').mkdir(parents=True);registry=out/'deaths'
os.environ.update(BORG_PLAY_DATA_DIR=str(profile),BORG_DEATH_RECORDS_DIR=str(registry))
save=profile/'save/Dr';fixture=root/'docs/validation/2026-11-07-post-m4-investigation/death-checkpoints/live-druid.save';shutil.copy2(fixture,save)
def args(seed,slot=None,race=None):return Namespace(character='Dr',new=False,from_backup=slot,seed=seed,race=race,cls=None,turns=100000,keys=50000,wall_seconds=60,checkpoint_every=1000,death_issue=None)
first=runtime.record_run(args(1));second=runtime.record_run(args(2,2));a,b=(json.loads((p/'recording.json').read_text()) for p in (first,second));comparisons={name:runtime.sha(first/'run'/name)==runtime.sha(second/'run'/name) for name in ('keys.log','messages.log','state.log','frames.jsonl','final.save')}
result={'first':str(first),'second':str(second),'same_origin':a['original_named_sha256']==b['original_named_sha256'],'gameplay_equal':comparisons,'index':json.loads((registry/'index.json').read_text())}
# A wrong-race request using the original living backup must be refused.
before=runtime.sha(save);backups=profile/'save/backups/Dr';inventory=lambda:{str(p.relative_to(backups)):runtime.sha(p) for p in backups.rglob('*') if p.is_file()};before_backups=inventory()
try:runtime.record_run(args(3,2,3))
except ValueError as error:result['refusal']=str(error)
else:raise AssertionError('Race mismatch was not refused')
result['refusal_preserved_save']=before==runtime.sha(save);result['refusal_preserved_backups']=before_backups==inventory()
(out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
