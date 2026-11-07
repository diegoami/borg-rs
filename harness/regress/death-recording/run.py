#!/usr/bin/env python3
"""Natural GUI death/export/replay from a living checkpoint on private Xvfb."""
from pathlib import Path
import os,subprocess,time,selectors,shutil,json,sys,argparse
root=Path(__file__).resolve().parents[3]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('output',type=Path)
parser.add_argument('--project-root',type=Path,required=True,help='Disposable checkout used for GUI build')
args=parser.parse_args()
project=args.project_root.resolve();out=args.output.resolve();out.mkdir(parents=True);profile=out/'profile';(profile/'save').mkdir(parents=True)
with (out/'build.log').open('w') as log:
 subprocess.run([str(project/'run-rust-borg.sh'),'--build-only'],stdout=log,stderr=subprocess.STDOUT,check=True)
shutil.copy2(root/'docs/validation/2026-11-07-post-m4-investigation/death-checkpoints/live-druid.save',profile/'save/DeathTest')
with (out/'xvfb.log').open('w') as log:
 number=str(1000+os.getpid()%30000);server=subprocess.Popen(['Xvfb',':'+number,'-displayfd','1','-screen','0','1600x1000x24','-nolisten','tcp'],stdout=subprocess.PIPE,stderr=log,text=True)
 try:
  with selectors.DefaultSelector() as ready:
   ready.register(server.stdout,selectors.EVENT_READ);assert ready.select(15) and server.stdout.readline().strip()==number
  env={**os.environ,'DISPLAY':':'+number,'BORG_PLAY_FONT':'fixed','BORG_PLAY_DATA_DIR':str(profile),'BORG_DEATH_RECORDS_DIR':str(out/'deaths')}
  with (out/'launcher.log').open('w') as log:
   game=subprocess.Popen([str(project/'start-borg.sh'),'DeathTest'],env=env,stdout=log,stderr=subprocess.STDOUT)
   try:
    for _ in range(150):
     assert game.poll() is None,(out/'launcher.log').read_text()
     found=subprocess.run(['xdotool','search','--name','^Angband$'],env=env,capture_output=True,text=True)
     bundles=list((profile/'recordings').glob('*'))
     if found.returncode==0 and bundles:
      bundle=bundles[0]
      if (bundle/'run/events.txt').exists() and (bundle/'run/events.txt').stat().st_size:break
     time.sleep(.2)
    else:raise AssertionError('No window')
    window=found.stdout.split()[0]
    def key(k):
     if game.poll() is not None:return
     r=subprocess.run(['xdotool','key','--window',window,k],env=env,capture_output=True)
     if r.returncode:
      game.wait(5);assert game.returncode==0,r.stderr
     time.sleep(.3)
    def frames():
     p=bundle/'run/frames.jsonl';return [json.loads(l) for l in p.read_bytes().splitlines() if l.endswith(b'}')]
    key('space');time.sleep(.5);key('ctrl+z');key('z')
    for _ in range(400):
     rows=frames()
     if rows and rows[-1].get('dead'):break
     if game.poll() is not None:break
     time.sleep(.1)
    else:raise AssertionError('Bound reached without a natural death; no artificial kill is introduced')
    rows=frames();assert any(f.get('dead') and f.get('cause') for f in rows),'No native death record'
    for _ in range(12):
     if game.poll() is not None:break
     key('q' if _ % 2 == 0 else 'y')
    game.wait(10);assert game.returncode==0,(out/'launcher.log').read_text()
    record=json.loads((bundle/'recording.json').read_text());assert record['status']=='complete'
    assert not (profile/'archive/Borg.map').exists(), 'GUI polluted the shared archive'
    assert (bundle/'run/archive/Borg.map').is_file(), 'Missing per-session native map'
    subprocess.run([str(root/'replay-borg.py'),str(bundle),'--verify-only','--output',str(out/'verification')],check=True)
    dossier=next((out/'deaths').glob('D*/record.json'));case=json.loads(dossier.read_text());assert case['terminal']['dead']
    (out/'result.json').write_text(json.dumps({'bundle':str(bundle),'dossier':str(dossier.parent),'terminal':case['terminal'],'verified':True,'qualification':'loaded living Druid checkpoint; fresh Borg diagnostic branch, not original cohort replay or independent survival sample'},indent=2)+'\n')
    print((out/'result.json').read_text())
   finally:
    if game.poll() is None:game.terminate();game.wait(15)
 finally:server.terminate();server.wait(5);server.stdout.close()
