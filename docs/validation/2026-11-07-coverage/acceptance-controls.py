"""Acceptance refusal controls against a real successful smoke coverage group."""
from pathlib import Path
import json,subprocess,sys,tempfile
sys.path.insert(0,str(Path.cwd()/'harness/coverage'))
from validate import validate_group,values
matrix=Path(sys.argv[1]);bid=values(matrix/'BUILD_ID')
def refused(label,fn):
    try:fn()
    except (AssertionError,FileNotFoundError):print('REJECTED',label)
    else:raise RuntimeError('incorrectly accepted '+label)
assert len(validate_group(matrix,'smoke',bid))==3
print('ACCEPTED original smoke group:3 exact process/profile pairs')
refused('mismatched coverage binary',lambda:validate_group(matrix,'smoke',{**bid,'binary_sha256':'0'*64}))
with tempfile.TemporaryDirectory(prefix='coverage-acceptance-',dir='/tmp') as tmp:
    root=Path(tmp);d=root/'smoke';d.mkdir()
    (root/'BUILD_ID').symlink_to(matrix/'BUILD_ID')
    for f in (matrix/'smoke').iterdir():
        if f.name != 'profiles.json':(d/f.name).symlink_to(f,target_is_directory=f.is_dir())
    (root/'smoke.diff').symlink_to(matrix/'smoke.diff')
    original=json.loads((matrix/'smoke/profiles.json').read_text());name=next(iter(original))
    profile_dir=root/'profiles/smoke';profile_dir.mkdir(parents=True)
    for item in original.values():
        source=Path(item['profile']);alias=profile_dir/source.name;alias.symlink_to(source);item['profile']=str(alias)
    (d/'profiles.json').write_text(json.dumps(original))
    assert len(validate_group(root,'smoke',bid))==3
    print('ACCEPTED relocated complete profile group')
    for kind in ('pid','hash'):
        entries=json.loads(json.dumps(original))
        if kind=='pid':entries[name]['child_pid']+=1
        else:entries[name]['sha256']='0'*64
        (d/'profiles.json').write_text(json.dumps(entries))
        refused('incorrect profile '+kind,lambda:validate_group(root,'smoke',bid))
    (d/'profiles.json').write_text(json.dumps(original))
    extra=profile_dir/'unexpected-0.profraw';extra.write_bytes(b'unexpected')
    refused('unexpected process profile',lambda:validate_group(root,'smoke',bid));extra.unlink()
    wrong=json.loads(json.dumps(original));wrong[name]['profile']=json.loads((matrix/'smoke/profiles.json').read_text())[name]['profile']
    (d/'profiles.json').write_text(json.dumps(wrong))
    refused('profile assigned to a different group directory',lambda:validate_group(root,'smoke',bid))
    (d/'profiles.json').write_text(json.dumps(original))
    refused('missing group',lambda:validate_group(root,'full',bid))
    out=root/'joined'
    p=subprocess.run(['python3','harness/coverage/join.py',str(out),str(root)],capture_output=True,text=True)
    assert p.returncode==1 and 'exactly one accepted complete group' in p.stderr and not out.exists(),p.stderr
    print('REJECTED incomplete join before creating output')
    p=subprocess.run(['python3','harness/coverage/report.py',str(root),str(out),'--llvm-bin','/tmp/unneeded-for-refusal'],capture_output=True,text=True)
    assert p.returncode==1 and 'full matrix is incomplete' in p.stderr and not out.exists(),p.stderr
    print('REJECTED full report without matrix DONE')
print('PASS acceptance refusal controls')
