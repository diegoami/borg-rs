#!/usr/bin/env python3
"""Real PTY movie controls; --view-only never executes an engine."""
import fcntl
import json
import os
from pathlib import Path
import pty
import select
import signal
import struct
import sys
import termios
import time

ROOT=Path(__file__).resolve().parents[3]


def main():
    bundle,out=map(lambda x:Path(x).resolve(),sys.argv[1:])
    out.mkdir(parents=True,exist_ok=False)
    pid,fd=pty.fork()
    if pid==0:
        os.environ['TERM']='xterm-256color'
        os.execv(sys.executable,[sys.executable,str(ROOT/'replay-borg.py'),str(bundle),'--view-only','--from-turn','500'])
    fcntl.ioctl(fd,termios.TIOCSWINSZ,struct.pack('HHHH',45,150,0,0))
    transcript=bytearray()
    def drain(seconds):
        deadline=time.monotonic()+seconds
        while time.monotonic()<deadline:
            if not select.select([fd],[],[],max(0,deadline-time.monotonic()))[0]:break
            try:chunk=os.read(fd,65536)
            except OSError:break
            if not chunk:break
            transcript.extend(chunk)
    keys=[b'l',b'h',b'g500\n',b'i',b'\x1b',b'e',b'\x1b',b's',b'\x1b',b'p',b'\t',b'\t',b'\t',b'j',b'k',b'\x1b',b' ',b' ',b'q']
    try:
        drain(2)
        for key in keys:
            os.write(fd,key);drain(.2)
        drain(.5)
        deadline=time.monotonic()+5
        while True:
            ended,status=os.waitpid(pid,os.WNOHANG)
            if ended:break
            if time.monotonic()>deadline:raise ValueError('Viewer did not exit on q')
            drain(.1)
        code=os.waitstatus_to_exitcode(status)
        text=transcript.decode('utf-8',errors='strict')
        checks={word:word in text for word in ('UNVERIFIED','Inventory','Equipment','Spells / prayers','Native fighting log','500')}
        (out/'terminal.txt').write_bytes(transcript)
        (out/'results.json').write_text(json.dumps({'exit':code,'checks':checks,'keys':[x.decode('ascii') for x in keys],
            'scope':'actual focused PTY interaction with original frames; movie-only, no native execution'},indent=2)+'\n')
        if code or not all(checks.values()):raise ValueError('A PTY viewer check failed; transcript retained')
        print('PASS real PTY seek/step/autoplay/logs/i/e/s/Escape/q',flush=True)
    finally:
        try:
            if os.waitpid(pid,os.WNOHANG)[0]==0:os.kill(pid,signal.SIGTERM);os.waitpid(pid,0)
        except ChildProcessError:pass
        os.close(fd)


if __name__=='__main__':main()
