from pathlib import Path
import json,sys,shutil
root=Path('/home/diego/projects/borg-rs');sys.path.insert(0,str(root/'tools'));import borg_deaths as deaths
receipt=root/'docs/validation/2026-11-07-post-m4-investigation';records=root/'docs/deaths';inventory=deaths.read(receipt/'death-inventories.json')
builds={lane:dict(line.split('=',1) for line in (receipt/f'{lane}-BUILD_ID').read_text().splitlines()) for lane in ('c','rust')}
for case,slug in [('D0001','warrior'),('D0002','druid'),('D0003','rogue'),('D0004','blackguard')]:
 key='live-'+slug;original=receipt/'normal-rust'/key;summary=deaths.read(original/'summary.json');config=dict(line.split('=',1) for line in (original/'config.txt').read_text().splitlines());seed=summary['seed'];folder=records/(case+'-seed'+str(seed)+'-'+slug)
 identity={'source':builds['rust']['git_rev'],'source_sha256':builds['rust']['source_sha256'],'binary':builds['rust']['binary_sha256'],'seed':seed,'start_save':None,'config_sha256':config['borg_txt_sha256'],'turn':summary['game_turns'],'cause':summary['death_cause'],'encounter_family':deaths.family(seed,summary['race_idx'],summary['class_idx'],config['borg_txt_sha256'],None)}
 if folder.exists():
  assert deaths.read(folder/'record.json')['identity']==identity
  continue
 folder.mkdir()
 assessment_file=root/f'docs/deaths/seed{seed}-{slug}.md'
 (folder/'report.md').write_text(assessment_file.read_text().replace('../validation/','../../validation/')+'\n\nLegacy dossier: original movies were not captured. See immutable record.json and the original cohort receipt; no new replay or life is claimed.\n')
 deaths.runtime.write(folder/'build.json',builds['rust']);shutil.copy2(original/'summary.json',folder/'summary.json');shutil.copy2(original/'config.txt',folder/'config.txt');deaths.runtime.write(folder/'inventory.json',inventory[key])
 attempts=[]
 for role,lane,label in [('original','rust','normal-rust'),('repeat','c','normal-c1'),('repeat','c','normal-c2')]:
  location=receipt/label/key
  attempts.append({'role':role,'outcome':'death','source_commit':builds[lane]['git_rev'],'binary_sha256':builds[lane]['binary_sha256'],'raw_directory':str(location), 'result_sha256':deaths.fingerprint({'summary':deaths.runtime.sha(location/'summary.json'),'process':deaths.runtime.sha(location/'process.json'),'binary':builds[lane]['binary_sha256']})})
 record={'schema':deaths.SCHEMA,'id':case,'identity':identity,'encounter_family':identity['encounter_family'],'raw_cause':identity['cause'],'character':f'{summary["race"]} {summary["class"]}', 'terminal':{'turn':identity['turn'],'level':summary['clevel'],'depth':summary['depth'],'dead':True,'cause':identity['cause']},'recording':None,'attempts':attempts,'issue':None,'investigation':None,'run_kind':'historical ordinary nine-class cohort; C/C/Rust three-log equality','independence_note':'One original input; repeated oracle verification is not three independent deaths. Cohort denominator remains nine starts.','living_checkpoints':[inventory[key]['checkpoint']], 'historical_evidence':{'receipt':str(receipt),'assessment':str(assessment_file),'observations_sha256':{str(p.relative_to(root)):deaths.runtime.sha(p) for p in original.iterdir() if p.is_file()}}, 'files_sha256':{p.name:deaths.runtime.sha(p) for p in folder.iterdir() if p.name!='report.md'}}
 deaths.runtime.write(folder/'record.json',record)
 text=assessment_file.read_text().split('## Evaluation and next action\n\n',1)[1].split('\n\n',1)[0].replace('\n',' ')
 deaths.runtime.write(folder/'assessment.json',{'classification':'C/Rust parity observed; defect not established','confidence':'Actual original cohort and before-death native inventory; causal fix unproved','findings':[],'feedback':[],'actions':[{'key':'legacy-'+slug,'summary':text,'status':'recorded; needs causal investigation','issue':None}]})
with deaths.registry_lock(records):deaths.update_index(records)
print(json.dumps(deaths.read(records/'index.json'),indent=2))
