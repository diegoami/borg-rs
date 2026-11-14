#!/usr/bin/env python3
"""Two manual sessions on a prepared branch on a private loopback Xvfb."""
import json
import os
from pathlib import Path
import selectors
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools'))
import borg_gui


def main():
    branch, out = map(lambda x: Path(x).resolve(), sys.argv[1:])
    out.mkdir(parents=True, exist_ok=False)
    number = str(200 + os.getpid() % 500)
    with (out / 'xvfb.log').open('w') as log:
        x = subprocess.Popen(['Xvfb', ':'+number, '-displayfd','1','-screen','0','1600x1000x24', '-nolisten','unix','-listen','tcp','-ac'], stdout=subprocess.PIPE, stderr=log, text=True)
        try:
            with selectors.DefaultSelector() as select:
                select.register(x.stdout,selectors.EVENT_READ)
                if not select.select(10) or x.stdout.readline().strip() != number:
                    raise ValueError('Private display failed')
            env = {**os.environ, 'DISPLAY':'127.0.0.1:'+number, 'BORG_PLAY_FONT':'fixed'}
            results = []
            profile = branch / 'profile'
            for index in range(2):
                before=set((profile/'recordings').glob('*'))
                with (out/f'session-{index}.log').open('w') as session_log:
                    child=subprocess.Popen([str(ROOT/'scripts/play-branch.py'),str(branch)],env=env,stdout=session_log,stderr=subprocess.STDOUT)
                    try:
                        splash_dismissed = False
                        for _ in range(200):
                            if child.poll() is not None:
                                raise ValueError('Manual launcher failed: '+str(out/f'session-{index}.log'))
                            found=subprocess.run(['xdotool','search','--name','^Angband$'],env=env,capture_output=True,text=True)
                            candidates=set((profile/'recordings').glob('*'))-before
                            if found.returncode == 0 and candidates:
                                bundle=next(iter(candidates))
                                if not splash_dismissed and (bundle/'run/events.txt').exists():
                                    subprocess.run(['xdotool','key','--window',found.stdout.split()[0],'space'],env=env,check=True,capture_output=True)
                                    splash_dismissed = True
                                if (bundle/'run/human-approved').exists():
                                    window=found.stdout.split()[0]; break
                            time.sleep(.1)
                        else: raise ValueError('Manual readiness timeout')
                        def key(value):
                            subprocess.run(['xdotool','key','--window',window,value],env=env,check=True,capture_output=True)
                            time.sleep(.2)
                        key('5')
                        key('ctrl+x')
                        for _ in range(8):
                            if child.poll() is not None: break
                            key('Escape')
                        if child.wait(timeout=10): raise ValueError('Manual game exit failure')
                        record=borg_gui.validate(bundle)
                        proof=borg_gui.verify(bundle)
                        assert record['session_context']['independent_life'] is False
                        results.append({'session':str(bundle),'verified':proof['verified'],'final_save':record.get('final_sha256')})
                    finally:
                        if child.poll() is None: child.terminate(); child.wait(timeout=10)
            (out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
            print('Two manual sessions saved and replayed:',out/'results.json',flush=True)
        finally:
            x.terminate(); x.wait(timeout=10)


if __name__ == '__main__': main()
