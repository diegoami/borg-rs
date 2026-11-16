#!/usr/bin/env python3
"""Native town regressions (PR83 and PR92). Pass an isolated test-town-review CMake build."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import tempfile
ROOT=Path(__file__).resolve().parents[3]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('build',type=Path);p.add_argument('output',type=Path)
a=p.parse_args();build=a.build.resolve();out=a.output.resolve();out.mkdir(parents=True,exist_ok=False)
source=Path(__file__).with_name('probe.c');obj=out/'probe.o';binary=out/'probe'
subprocess.run(['cc','-O2','-DNDEBUG','-DALLOW_BORG','-DBORG_HARNESS','-I'+str(ROOT/'angband/src'),'-c',str(source),'-o',str(obj)],check=True)
cmd=[v for v in shlex.split((build/'CMakeFiles/OurExecutable.dir/link.txt').read_text()) if not v.startswith('-Wl,--dependency-file=')]
cmd[cmd.index('-o')+1]=str(binary);cmd +=[str(obj),'-Wl,--wrap=savefile_load'];subprocess.run(cmd,cwd=build,check=True)
(out/'lib').symlink_to(build/'game/lib',target_is_directory=True)
fixture=ROOT/'docs/validation/2026-11-07-post-m4-investigation/death-checkpoints/live-druid.save'
with tempfile.TemporaryDirectory(prefix='town-review-user-') as private:
 result=subprocess.run([str(binary),'-mborgh','-duser='+private,'--','-seed','1','-race','0','-class','0','-load',str(fixture)],cwd=out,capture_output=True,text=True,timeout=20)
(out/'stdout.log').write_text(result.stdout);(out/'stderr.log').write_text(result.stderr)
assert result.returncode==0,(result.returncode,result.stderr,result.stdout[-1000:])
rows=[json.loads(line) for line in result.stdout.splitlines() if line.startswith('{')]
assert {r['case'] for r in rows}=={'partial-rune','native-capacity','native-sale','native-rods','home-history'},rows
(out/'receipt.json').write_text(json.dumps({'checks':rows,'probe_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()},indent=2)+'\n')
print('Native partial runes, full pack/quiver/Home joins, sale quotes, rod charging and confirmed Home history passed')
