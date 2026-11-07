#!/usr/bin/env python3
"""Report only completed, oracle-checked coverage groups; name pilot scope explicitly."""
import argparse
import hashlib
import json
import lzma
from pathlib import Path
import subprocess

from validate import ALL_SUITES, reference_root, validate_group

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('matrix', type=Path)
p.add_argument('output', type=Path)
p.add_argument('--llvm-bin', type=Path, required=True)
p.add_argument('--suites', nargs='+', help='Explicit pilot subset; otherwise require the full matrix DONE')
p.add_argument('--reference-root', type=Path, help='C oracle root; defaults to the recorded root or harness/reference')
a = p.parse_args()
suites = a.suites or ALL_SUITES
assert len(suites) == len(set(suites)), "duplicate suite"
if not a.suites:
    assert (a.matrix/'DONE').exists(), 'full matrix is incomplete'
assert not a.output.exists(), 'refusing existing report output'
bid = dict(line.split('=', 1) for line in (a.matrix/'BUILD_ID').read_text().splitlines())
image = Path(bid['binary'])
assert hashlib.sha256(image.read_bytes()).hexdigest() == bid['binary_sha256']
references = reference_root(a.matrix, a.reference_root)
profiles, count, oracles = [], 0, {}
for suite in suites:
    group_profiles = validate_group(a.matrix, suite, bid, references, oracles)
    profiles.extend(str(f) for f in group_profiles)
    count += len(group_profiles)
assert len(set(profiles)) == len(profiles), 'profile reused across groups'
a.output.mkdir(parents=True)
(a.output/'ORACLES.json').write_text(json.dumps(oracles, indent=2, sort_keys=True)+'\n')
(a.output/'REFERENCE_ROOT').write_text(str(references)+'\n')
profdata = a.output/'coverage.profdata'
subprocess.run([str(a.llvm_bin/'llvm-profdata'),'merge','-sparse',*profiles,'-o',str(profdata)],check=True)
raw = subprocess.check_output([str(a.llvm_bin/'llvm-cov'),'export','-instr-profile='+str(profdata),str(image)])
with lzma.open(a.output/'coverage.json.xz','wb') as f:
    f.write(raw)
data = json.loads(raw)['data'][0]
marker = '/borg-rs/src/'
def relative(name):
    return name.split(marker,1)[1] if marker in name else None
def logic(name):
    return '/' not in name and name not in ('engine.rs','regress.rs')
rows = []
for f in data['files']:
    name = relative(f['filename'])
    if name is not None:
        rows.append({'file':name,'scope':'logic' if logic(name) else 'ffi_or_support','summary':f['summary']})
def totals(selected):
    result = {}
    for key in ['lines','functions','regions']:
        n = sum(x['summary'][key]['count'] for x in selected)
        covered = sum(x['summary'][key]['covered'] for x in selected)
        result[key] = {'count':n,'covered':covered,'percent':100*covered/n if n else None}
    return result
functions = []
for f in data['functions']:
    for r in f['regions']:
        name = relative(f['filenames'][r[5]])
        if name is not None and r[7] == 0:
            functions.append({'file':name,'line':r[0],'symbol':f['name'],'count':f['count'],'scope':'logic' if logic(name) else 'ffi_or_support'})
            break
try:
    demangled = subprocess.run(['c++filt','--format=rust'],input='\n'.join(f['symbol'] for f in functions)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
    if len(demangled)==len(functions):
        for item,name in zip(functions,demangled): item['name']=name
except (OSError,subprocess.CalledProcessError):
    pass
result = {'scope':'pilot_subset' if a.suites else 'complete_matrix','suites':suites,'runs':count,'build_id':bid,
          'llvm_version':subprocess.check_output([str(a.llvm_bin/'llvm-cov'),'--version'],text=True).strip(),
          'totals_project':totals(rows),'totals_logic':totals([x for x in rows if x['scope']=='logic']),
          'files':sorted(rows,key=lambda x:x['file'])}
(a.output/'summary.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
(a.output/'functions.json').write_text(json.dumps(functions,indent=2,sort_keys=True)+'\n')
lines=[f"# Rust coverage: {result['scope']}", '', f"Source `{bid['git_rev']}`; suites {', '.join(suites)}; {count} successful oracle-checked runs.",
       'Only the selected completed groups are merged. Interrupted or failed groups are excluded.', '',
       '| scope | covered lines | covered LLVM functions |', '|---|---:|---:|']
for label,key in [('All project Rust','totals_project'),('Borg logic (excluding FFI/generated/regression modules)','totals_logic')]:
    t=result[key]
    lines.append(f"| {label} | {t['lines']['covered']}/{t['lines']['count']} ({t['lines']['percent']:.2f}%) | {t['functions']['covered']}/{t['functions']['count']} ({t['functions']['percent']:.2f}%) |")
lines += ['', 'LLVM function counts include monomorphizations and closures. Branch coverage is not measured.', '',
          '| Rust source | covered lines |', '|---|---:|']
for f in result['files']:
    v=f['summary']['lines']
    if v['count']: lines.append(f"| {f['file']} | {v['covered']}/{v['count']} ({v['percent']:.2f}%) |")
lines += ['', '## Unentered logic functions (first 40 by source location)', '', 'The complete list and counts are in `functions.json`. A nonzero entry count alone does not prove a tactic completed.', '']
gaps=sorted([f for f in functions if f['scope']=='logic' and f['count']==0],key=lambda f:(f['file'],f['line']))
for f in gaps[:40]:
    url=f"https://github.com/diegoami/borg-rs/blob/{bid['git_rev']}/borg-rs/src/{f['file']}#L{f['line']}"
    lines.append(f"- [{f['file']}:{f['line']}]({url}): `{f.get('name',f['symbol'])}`")
(a.output/'REPORT.md').write_text('\n'.join(lines)+'\n')
print(json.dumps({key:result[key] for key in ['scope','runs','totals_project','totals_logic']},indent=2))
