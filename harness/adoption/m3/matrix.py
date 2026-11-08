#!/usr/bin/env python3
"""Compare the milestone3 C/Rust game matrix and its change against milestone2.

For every case of the plan: C and Rust must both have run (summary.json,
process exit recorded by the suite) and agree byte-for-byte on keys.log,
messages.log, state.log and the terminal native save (final.save). The C run
is then compared with the committed milestone2 reference (same inputs,
pre-milestone3 C/Rust) to list the cases whose behaviour changed.
harness/diff is also run on the two suite directories. Writes matrix.json.

Exit status is 0 only when both suite directories carry DONE, every planned
case ran in both roles with parity, harness/diff succeeded and, when fault
controls are present, every fault suite carries DONE with the same case count
as the first. Otherwise it exits 1 after writing the diagnostics. Fault
detection counts are reported, never gated: a fault output is not parity
evidence, and an undetected fault means "not detected on this subset".
"""
import argparse
import hashlib
import json
import lzma
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
REF = ROOT / 'harness/reference/better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f'
STREAMS = ('keys.log', 'messages.log', 'state.log', 'final.save')


def digest(path):
    if path.exists():
        return hashlib.sha256(path.read_bytes()).hexdigest()
    xz = path.with_name(path.name + '.xz')
    if xz.exists():
        return hashlib.sha256(lzma.decompress(xz.read_bytes())).hexdigest()
    return None


def main():
    p = argparse.ArgumentParser(description=__doc__)
    out = ROOT / 'harness/out/c-m3'
    p.add_argument('--plan', type=Path, default=out / 'plan/adoption-suite-final')
    p.add_argument('--c', type=Path, default=out / 'g-c')
    p.add_argument('--rust', type=Path, default=out / 'g-rust')
    p.add_argument('--output', type=Path, default=out / 'matrix.json')
    p.add_argument('--faults', type=Path, default=out / 'faults', help='per-fix Rust fault-control runs')
    p.add_argument('--reference', type=Path, default=REF,
                   help='previous-target case directories (default: committed milestone2 reference)')
    args = p.parse_args()
    cases = [line.split()[0] for line in args.plan.read_text().splitlines()
             if line.split('#', 1)[0].strip()]
    rows, parity, changed, missing = {}, 0, [], []
    for name in cases:
        c, r, ref = args.c / name, args.rust / name, args.reference / name
        row = {}
        if not (c / 'summary.json').exists() or not (r / 'summary.json').exists():
            row['missing'] = True
            missing.append(name)
            rows[name] = row
            continue
        cs = json.loads((c / 'summary.json').read_text())
        rs = json.loads((r / 'summary.json').read_text())
        for key in ('wall_seconds',):
            cs.pop(key, None)
            rs.pop(key, None)
        row['streams'] = {s: [digest(c / s), digest(r / s)] for s in STREAMS}
        row['summary_equal'] = cs == rs
        row['parity'] = row['summary_equal'] and all(a is not None and a == b for a, b in row['streams'].values())
        parity += row['parity']
        if ref.exists():
            row['changed_vs_m2'] = [s for s in STREAMS if digest(ref / s) != digest(c / s)]
            if row['changed_vs_m2']:
                ref_summary = json.loads((ref / 'summary.json').read_text())
                row['m2'] = {k: ref_summary.get(k) for k in ('stop_reason', 'game_turns', 'clevel', 'max_depth', 'dead', 'death_cause')}
                row['m3'] = {k: cs.get(k) for k in row['m2']}
                changed.append(name)
        rows[name] = row
    diff = subprocess.run([str(ROOT / 'harness/diff'), str(args.c), str(args.rust)],
                          stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    report = {'cases': len(cases), 'both_ran': len(cases) - len(missing), 'parity': parity,
              'missing': missing, 'changed_vs_m2': changed, 'harness_diff_exit': diff.returncode,
              'harness_diff_tail': diff.stdout.splitlines()[-5:], 'rows': rows}
    if args.faults.is_dir():
        report['fault_controls'] = faults(args.c, args.faults)
    args.output.write_text(json.dumps(report, indent=1) + '\n')
    print(f"cases {len(cases)}; both ran {report['both_ran']}; C/Rust parity {parity}; "
          f"changed vs reference {len(changed)}; harness/diff exit {diff.returncode}")
    for name in changed:
        print(' ', name, rows[name]['m2'], '->', rows[name]['m3'])
    for fix, f in report.get('fault_controls', {}).items():
        print(f"fault {fix}: {'DETECTED' if f['detected'] else 'not detected on this subset'} "
              f"({len(f['differing'])}/{f['cases']} cases differ; suite DONE={f['suite_done']})")
    problems = []
    for role, path in (('C', args.c), ('Rust', args.rust)):
        if not (path / 'DONE').exists():
            problems.append(f'{role} suite has no DONE marker')
    if missing:
        problems.append(f'{len(missing)} planned cases missing in a role')
    if parity != len(cases):
        problems.append(f'parity {parity}/{len(cases)}')
    if diff.returncode != 0:
        problems.append(f'harness/diff exit {diff.returncode}')
    fault_rows = report.get('fault_controls', {})
    if fault_rows:
        counts = {f['cases'] for f in fault_rows.values()}
        if len(counts) != 1 or not all(f['suite_done'] for f in fault_rows.values()):
            problems.append('incomplete fault-control suites')
    if problems:
        print('FAIL: ' + '; '.join(problems))
        sys.exit(1)
    print('PASS')


def faults(c_dir, faults_dir):
    """Per-fix Rust fault controls: each faults/NNNN is the Rust image with
    only that fix's removal unit reversed, run on the same inputs. A control
    is 'detected' when any case differs from the corrected C run."""
    result = {}
    for variant in sorted(p for p in Path(faults_dir).iterdir() if p.is_dir() and (p / 'manifest').exists()):
        cases = sorted(p.name for p in variant.iterdir() if (p / 'summary.json').exists())
        differ = [n for n in cases if any(digest(variant / n / s) != digest(Path(c_dir) / n / s) for s in STREAMS)]
        result[variant.name] = {'cases': len(cases), 'differing': differ, 'detected': bool(differ),
                                'suite_done': (variant / 'DONE').exists()}
    return result


if __name__ == '__main__':
    main()
