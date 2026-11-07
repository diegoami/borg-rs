from pathlib import Path
import hashlib,json,os,re,shutil,subprocess,tempfile,time
root=Path(__file__).resolve().parents[3];out=root/'harness/out/local'
out.mkdir(parents=True,exist_ok=True)
proof=[]
def run(profile,stage):
    environment=os.environ.copy();environment['BORG_PLAY_DATA_DIR']=str(profile)
    save=profile/'save/RustBorg';before=save.stat().st_mtime_ns
    with (out/f'play-rust-save-{stage}.log').open('w') as log:
        p=subprocess.Popen([str(root/'run-rust-borg.sh')],cwd='/tmp',env=environment,stdout=log,stderr=subprocess.STDOUT)
        try:
            window=None
            for _ in range(160):
                if p.poll() is not None:raise RuntimeError(f'early exit {p.returncode}')
                r=subprocess.run(['xdotool','search','--name','^Angband$'],capture_output=True,text=True)
                if r.returncode==0 and r.stdout.strip():window=r.stdout.split()[0];break
                time.sleep(.25)
            assert window,'no normal X11 window'
            subprocess.run(['xdotool','key','--window',window,'space'],check=True)
            time.sleep(1)
            subprocess.run(['import','-window','root',str(out/f'play-rust-save-{stage}.png')],check=True)
            for _ in range(5):
                if p.poll() is not None:break
                subprocess.run(['xdotool','key','--window',window,'ctrl+x'],check=True)
                time.sleep(.5)
            try:p.wait(timeout=5)
            except subprocess.TimeoutExpired:
                subprocess.run(['import','-window','root',str(out/f'play-rust-save-{stage}-failure.png')],check=True)
                raise RuntimeError('save-and-exit did not complete')
            assert p.returncode==0,p.returncode
            assert save.stat().st_size>0 and save.stat().st_mtime_ns!=before,'save not replaced'
            character=(profile/'user/CharOutput.txt').read_text()
            assert re.search(r'cLvl:\s*"50"',character),character
            proof.append({'stage':stage,'save_path':str(save),'bytes':save.stat().st_size,'sha256':hashlib.sha256(save.read_bytes()).hexdigest(),'exit_code':p.returncode,'character':character})
            print(stage,'PASS loaded level50 fixture, Ctrl-X saved and exited; save bytes',save.stat().st_size,flush=True)
        finally:
            if p.poll() is None:p.terminate();p.wait(timeout=5)
with tempfile.TemporaryDirectory(prefix='borg-launcher-save-',dir='/tmp') as temp:
    first=Path(temp)/'first';(first/'save').mkdir(parents=True)
    shutil.copyfile(root/'harness/saves/depth50.save',first/'save/RustBorg')
    run(first,'first')
    second=Path(temp)/'reload';(second/'save').mkdir(parents=True)
    shutil.copyfile(first/'save/RustBorg',second/'save/RustBorg')
    run(second,'reload')
(out/'play-rust-save-roundtrip.json').write_text(json.dumps(proof,indent=2,sort_keys=True)+'\n')
print('PASS real GUI save/reload roundtrip in isolated profiles; owner save not opened or modified')
