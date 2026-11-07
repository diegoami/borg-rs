#!/usr/bin/env python3
"""Controlled native spell/knowledge/RNG probes; no gameplay or owner saves."""
from pathlib import Path
import argparse
import json
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
args = parser.parse_args()
out = args.output.resolve();out.mkdir(parents=True, exist_ok=False)
build = ROOT / 'build/rust';source = Path(__file__).with_name('probe.c');obj = out / 'probe.o';binary = out / 'probe'
subprocess.run(['cc','-O2','-DNDEBUG','-DALLOW_BORG','-DBORG_HARNESS','-I'+str(ROOT/'angband/src'),'-c',str(source),'-o',str(obj)],check=True)
command=[v for v in shlex.split((build/'CMakeFiles/OurExecutable.dir/link.txt').read_text()) if not v.startswith('-Wl,--dependency-file=')]
command[command.index('-o')+1]=str(binary);command +=[str(obj),'-Wl,--wrap=savefile_load'];subprocess.run(command,cwd=build,check=True)
rows=[]
for fixture in ('docs/validation/2026-11-07-post-m4-investigation/death-checkpoints/live-druid.save','harness/saves/depth50.save'):
    with tempfile.TemporaryDirectory(prefix='borg-player-probe-') as private:
        result=subprocess.run([str(binary),'-mborgh','-duser='+private,'--','-seed','1','-race','0','-class','0','-load',str(ROOT/fixture)],cwd=build/'game',capture_output=True,text=True,timeout=15)
        assert result.returncode==0,(result.returncode,result.stderr,result.stdout[-500:])
        rows +=[json.loads(line) for line in result.stdout.splitlines()]
expected={'ready':'Ready','low_mana':'Low mana','blind':'Cannot see','confused':'Confused','no_book':'No book','forgotten':'Forgotten','dead':'Dead'}
for row in rows:
    if row['case']=='noncaster':
        assert not row['spells']['caster'] and not row['spells']['learned']
    else:
        assert row['spells']['learned'][0]['status']==expected[row['case']],row
        if row['case']=='ready':assert row['spells']['learned'][0]['castable']
        if row['case'] in ('blind','confused','no_book','forgotten','dead'):assert not row['spells']['learned'][0]['castable']
assert any(row['unknown_descriptions_checked'] > 0 for row in rows)
(out/'native-probe.json').write_text(json.dumps(rows,indent=2)+'\n')
print('8 native states: availability, player-known descriptions, RNG and seen-flag preservation pass')
