#!/usr/bin/env python3
"""Prepare the exact M2 suite and private save union before running games."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

ROOT=Path(__file__).resolve().parents[3]
SUITES=('smoke','full','deep','classes','config','options','crash-fixes','glyph-recognition','glyph-construction','natural','normal-live')
SAVE_DIRS={'glyph-recognition':'glyph-recognition','glyph-construction':'glyph-construction','natural':'captured-extended'}


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('output',type=Path);args=p.parse_args()
    out=args.output.resolve();out.mkdir(parents=True,exist_ok=False);saves=out/'fixtures';saves.mkdir()
    rows=[]
    for name in SUITES:
        source=ROOT/'harness/suites'/name
        for line in source.read_text().splitlines():
            fields=line.split('#',1)[0].split()
            if not fields:continue
            flags=fields[5].split(',') if len(fields)>5 and fields[5]!='-' else []
            original=','.join(flags)
            if 'living-save' in flags:flags[flags.index('living-save')]='terminal-save'
            if not any(f in flags for f in ('finalsave','terminal-save')):flags.append('terminal-save')
            row={'line':' '.join([*fields[:5],','.join(flags)]),'source_suite':name,'suite_sha256':sha(source)}
            if 'living-save' in original.split(','):row['original_recording_flags']=original
            for flag in flags:
                if not flag.startswith('save='):continue
                file=ROOT/'harness/saves'/SAVE_DIRS.get(name,'')/flag[5:];assert file.is_file(),file
                target=saves/file.name
                if target.exists():assert sha(target)==sha(file),'Conflicting save filename'
                else:shutil.copy2(file,target)
                row.update(save_source=str(file),save_copy=str(target),save_sha256=sha(file))
            rows.append(row)
    assert len(rows)==131 and len({r['line'].split()[0] for r in rows})==131
    suite=out/'adoption-suite-final';suite.write_text('# Exact M2 case plan; terminal native saves for all131 cases.\n'+'\n'.join(r['line'] for r in rows)+'\n')
    plan={'cases':rows,'wall_seconds':300,'captures':'terminal native save for every case; all other flags/inputs/budgets unchanged',
          'effective_suite_file':str(suite),'effective_suite_sha256':sha(suite)}
    expected=ROOT/'docs/validation/2026-11-08-c-m2/input-plan.json'
    if expected.exists():
        recorded=json.loads(expected.read_text())
        assert [(r['line'],r.get('save_sha256'),r['suite_sha256']) for r in rows]==[(r['line'],r.get('save_sha256'),r['suite_sha256']) for r in recorded['cases']], 'M2 inputs changed'
    (out/'input-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    print('Prepared131 exact M2 cases and',len(list(saves.iterdir())),'hashed native save inputs:',out)


if __name__=='__main__':main()
