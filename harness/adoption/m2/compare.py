#!/usr/bin/env python3
"""Qualify a case join by fresh strict C/C/Rust diff and full-save comparison.

Original failed suites stay failed. A join records every successful case's
source run and all excluded failures, and rejects source/input/budget changes.
It never accepts a saved textual PASS. Export oracles only after comparisons.
"""
import argparse
import hashlib
import json
import lzma
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[3]


def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
    return h.hexdigest()


def manifest(path):
    return dict(line.split('=',1) for line in path.read_text().splitlines() if '=' in line)


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--plan',type=Path,required=True)
    p.add_argument('--c1',type=Path,action='append',required=True)
    p.add_argument('--c2',type=Path,action='append',required=True)
    p.add_argument('--rust',type=Path,action='append',required=True)
    p.add_argument('--save-root',type=Path,required=True)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--export-reference',type=Path)
    args=p.parse_args();plan=json.loads(args.plan.read_text());out=args.output.resolve()
    out.mkdir(parents=True,exist_ok=False)
    report={'verified':False,'schema':'borg-rs/c-m2-case-join@1','plan_sha256':sha(args.plan),'cases':{},'source_suites':{},
            'scope':'fresh comparison of qualified case inputs, logs, states and full native saves; original failed suites remain failed'}
    try:
        for role,groups,impl in [('c1',args.c1,'c'),('c2',args.c2,'c'),('rust',args.rust,'rust')]:
            view=out/role;view.mkdir();identities=[]
            current=subprocess.check_output([str(ROOT/'harness/identity.sh'),'source-hash',impl],text=True).strip()
            for group in groups:
                group=group.resolve()
                assert (group/'DONE').exists() or (group/'FAILED').exists(),('Suite still running',group)
                meta=manifest(group/'manifest');identities.append(meta)
                assert meta['overrides']=='none' and meta['build_check']=='ok' and meta['impl_check']=='ok'
                assert meta['binary_impl']==impl and meta['build_id_source_sha256']==current,(group,'source mismatch')
                assert meta['binary_sha256']==meta['binary_sha256_end']
                assert meta['build_id_binary_sha256']==meta['binary_sha256']
                suite={'marker':'DONE' if (group/'DONE').exists() else 'FAILED','manifest_sha256':sha(group/'manifest'),
                       'failures':{f.parent.name:json.loads(f.read_text())['status'] for f in group.glob('*/process.json')
                                   if json.loads(f.read_text())['status']!='success'},
                       'preflight_failures':[k.removeprefix('run_config.') for k,v in meta.items()
                                             if k.startswith('run_config.') and v=='missing']}
                report['source_suites'][str(group)]=suite
            assert len({m['binary_sha256'] for m in identities})==1,'Mixed binaries in one role'
            for row in plan['cases']:
                fields=row['line'].split();name=fields[0];selected=None
                for directory,meta in zip(groups,identities):
                    directory=directory.resolve()/name;process=directory/'process.json'
                    if not process.exists():continue
                    process=json.loads(process.read_text())
                    if process['status']!='success':continue
                    assert process['engine_returncode']==0
                    assert process['executable_sha256_before']==meta['binary_sha256']
                    assert process['timeout_seconds']==plan['wall_seconds'],'Changed repeat budget'
                    cmd=process['command']
                    for option,value in zip(('-seed','-race','-class','-turns'),fields[1:5]):
                        assert cmd[cmd.index(option)+1]==value,(name,option)
                    cfg=manifest(directory/'config.txt');assert cfg['flags']==fields[5]
                    assert cfg['borg_txt_sha256']==sha(directory/'user/borg.txt')
                    if row.get('save_source'):
                        # suite.sh atomically renames its .partial directory.
                        # Rebase to the retained private game save. It may have
                        # been autosaved: config.txt records its initial hash.
                        original_case=Path(next(a.removeprefix('-duser=') for a in cmd if a.startswith('-duser='))).parent
                        relative=Path(cmd[cmd.index('-load')+1]).relative_to(original_case)
                        loaded=(directory/relative).resolve();assert loaded.is_relative_to(directory.resolve())
                        assert loaded.is_file()
                        assert cfg['save_sha256']==row['save_sha256']
                        assert sha(args.save_root/Path(row['save_source']).name)==row['save_sha256']
                    for required in ('keys.log','messages.log','state.log','summary.json','final.save'):
                        assert (directory/required).is_file(),(name,required)
                    selected=directory;break
                assert selected is not None,('No qualified successful case',role,name)
                (view/name).symlink_to(selected,target_is_directory=True)
                report['cases'].setdefault(name,{})[role]={'directory':str(selected),
                    'files_sha256':{f:sha(selected/f) for f in ('keys.log','messages.log','state.log','summary.json','final.save','config.txt','process.json')}}
                if row.get('save_source'):
                    report['cases'][name][role]['initial_save_sha256']=row['save_sha256']
                    report['cases'][name][role]['private_game_save_after_run_sha256']=sha(loaded)
            meta={k:v for k,v in identities[0].items() if not k.startswith('run_config.') and k not in ('finished_utc','started_utc','jobs','save_root')}
            meta.update(save_root=str(args.save_root.resolve()),qualified_case_join='true',qualification_result='provisional')
            if plan.get('effective_suite_file'):
                suite=Path(plan['effective_suite_file']);assert sha(suite)==plan['effective_suite_sha256']
                meta.update(suite=suite.name,suite_sha256=plan['effective_suite_sha256'])
            for row in plan['cases']:
                name=row['line'].split()[0];cfg=manifest(view/name/'config.txt')
                meta['run_config.'+name]=cfg['flags']+' '+cfg['borg_txt_sha256']
            # These are new qualified case views, not a relabel of a failed
            # original suite. Provisional markers are removed on any failure.
            (view/'manifest').write_text('\n'.join(k+'='+v for k,v in meta.items())+'\n')
            (view/'DONE').write_text('Provisional case view; only join verification.json establishes parity\n')
        for left,right,impl in [('c1','c2','c'),('c1','rust','rust')]:
            result=subprocess.run([str(ROOT/'harness/diff'),'--save-root',str(args.save_root.resolve()),'--expect-impl',impl,str(out/left),str(out/right)],capture_output=True,text=True)
            (out/(left+'-'+right+'.diff')).write_text(result.stdout+result.stderr)
            assert result.returncode==0,('Strict actual diff failed',left,right)
        for name,rows in report['cases'].items():
            assert rows['c1']['directory']!=rows['c2']['directory'],('C repeat reuses one execution',name)
            assert len({r['files_sha256']['final.save'] for r in rows.values()})==1,('Full save differs',name)
            assert len({r.get('private_game_save_after_run_sha256') for r in rows.values()})==1,('Private autosave differs',name)
        report.update(verified=True,compared_cases=len(report['cases']),full_saves_identical=True)
        (out/'verification.json').write_text(json.dumps(report,indent=2)+'\n')
        for role in ('c1','c2','rust'):
            path=out/role/'manifest'
            path.write_text(path.read_text().replace('qualification_result=provisional','qualification_result=OK'))
            (out/role/'DONE').write_text('Qualified case join; actual C/C/Rust diff and full native save equality; original failed suites preserved\n')
        if args.export_reference:
            ref=args.export_reference.resolve();ref.mkdir(parents=True,exist_ok=False)
            for name,rows in report['cases'].items():
                source=Path(rows['c1']['directory']);dest=ref/name;dest.mkdir()
                for f in ('keys.log','messages.log','state.log','final.save'):
                    with (source/f).open('rb') as before,lzma.open(dest/(f+'.xz'),'wb',preset=3) as after:
                        shutil.copyfileobj(before,after)
                for f in ('summary.json','config.txt','process.json'):
                    shutil.copy2(source/f,dest/f)
            shutil.copy2(out/'c1/manifest',ref/'manifest')
            (ref/'DONE').write_text('Qualified C/C/Rust case join; see QUALIFICATION.json for exact sources and retained failed suites\n')
            shutil.copy2(out/'verification.json',ref/'QUALIFICATION.json')
            (ref/'SHA256SUMS').write_text(''.join(sha(f)+'  '+str(f.relative_to(ref))+'\n' for f in sorted(ref.rglob('*')) if f.is_file()))
        print('PASS',len(report['cases']),'qualified C/C/Rust cases and full native saves')
    except BaseException as error:
        for role in ('c1','c2','rust'):(out/role/'DONE').unlink(missing_ok=True)
        report['error']=str(error);(out/'verification.json').write_text(json.dumps(report,indent=2)+'\n')
        raise


if __name__=='__main__':main()
