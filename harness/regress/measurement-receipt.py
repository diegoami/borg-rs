#!/usr/bin/env python3
"""Recheck the complete measured corpus and export a small, source-bound receipt."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('runs', type=Path)
p.add_argument('receipt', type=Path)
a = p.parse_args()
runs = a.runs.resolve()
reference = ROOT / 'harness/reference/better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f'
suite = ROOT / 'harness/out/c-m2/adoption-suite-final'
fixtures = ROOT / 'harness/out/c-m2/fixtures'
extra = runs / 'remaining-checked'
options = runs / 'options-checked'
assert (extra / 'DONE').exists() and (options / 'DONE').exists()
identities = json.loads((runs / 'source-identity.json').read_text())
images = identities['builds']
baseline_id = json.loads((extra / 'baseline-identity.json').read_text())
expected_hash = {**{role: images[role]['binary_sha256'] for role in ['c', 'rust']},
                 'baseline': baseline_id['binary_sha256']}


def diff(label, *args):
    path = runs / (label + '.diff')
    with path.open('w') as log:
        subprocess.run([str(ROOT / 'harness/diff'), *map(str, args)], cwd=ROOT,
                       stdout=log, stderr=subprocess.STDOUT, check=True)
    return hashlib.sha256(path.read_bytes()).hexdigest()


def describe(root, role):
    rows = sorted(p for p in root.iterdir() if p.is_dir() and (p / 'summary.json').exists())
    assert rows
    result = {'cases': len(rows), 'stream_trees': {}}
    for stream in ['keys.log', 'messages.log', 'state.log']:
        tree = b''.join(p.name.encode() + b'\0' + hashlib.sha256((p / stream).read_bytes()).digest()
                        for p in rows)
        result['stream_trees'][stream] = hashlib.sha256(tree).hexdigest()
    summaries = []
    processes = []
    for row in rows:
        summary = json.loads((row / 'summary.json').read_text())
        summary.pop('wall_seconds')
        summaries.append([row.name, summary])
        process = json.loads((row / 'process.json').read_text())
        assert process['status'] == 'success' and process['engine_returncode'] == process['exit_code'] == 0
        assert process['executable_sha256_before'] == process['executable_sha256_after'] == expected_hash[role]
        processes.append((process['child_pid'], process['started_unix']))
    assert len(set(processes)) == len(rows)
    result['summary_tree'] = hashlib.sha256(json.dumps(summaries, sort_keys=True).encode()).hexdigest()
    return result


receipt = {'implementation_commit': identities['implementation_commit'], 'builds': images,
           'baseline_ref': identities['baseline_ref'], 'baseline_image': baseline_id,
           'groups': {}, 'diffs': {}, 'options': {}, 'limits': [
               'Existing M2 gameplay adoption is inherited; no new gameplay changes.',
               'The additional suites and option matrix select qualified successful executions; original scheduling attempts remain preserved.',
               'Random birth changes a fixed character; exact/stall limits retain the key prefix.',
               'These are repeated oracle/regression executions, not independent survival lives.']}
for role in ['c', 'rust']:
    root = runs / (role + '-default')
    assert (root / 'DONE').exists()
    receipt['diffs'][role + '-m2-reference'] = diff(role + '-m2-reference', '--suite', suite,
                                                   '--save-root', fixtures, '--expect-impl', role,
                                                   root, reference)
    receipt['groups'][role + '-m2'] = describe(root, role)
    assert receipt['groups'][role + '-m2']['cases'] == 131
receipt['diffs']['m2-parity'] = diff('m2-parity', '--suite', suite, '--save-root', fixtures,
                                   runs / 'c-default', runs / 'rust-default')
for suite_name in ['captured-rogue', 'endgame', 'extended', 'lunal-investigation', 'synthetic-tactics']:
    save_root = ROOT / 'harness/saves'
    if suite_name in ['lunal-investigation', 'synthetic-tactics']:
        save_root /= 'synthetic-tactics'
    for role in ['c', 'rust']:
        label = role + '-' + suite_name
        receipt['diffs'][label] = diff(label, '--suite', suite_name, '--save-root', save_root,
                                       extra / role / suite_name, extra / 'baseline' / suite_name)
    receipt['diffs']['parity-' + suite_name] = diff('parity-' + suite_name,
                                                  extra / 'c' / suite_name, extra / 'rust' / suite_name)
    for role in ['baseline', 'c', 'rust']:
        receipt['groups'][role + '-' + suite_name] = describe(extra / role / suite_name, role)
rows = json.loads((options / 'results.json').read_text())
assert len(rows) == 60
receipt['options'] = {'executions': len(rows), 'results_sha256': hashlib.sha256((options / 'results.json').read_bytes()).hexdigest(),
                      'checks': 'three-log equality for states/nontriggering stall; exact key prefixes and exact caps; C/Rust summaries and STATE bytes'}
for role in ['c', 'rust']:
    assert sum(value['cases'] for name, value in receipt['groups'].items() if name.startswith(role + '-')) == 165
receipt['observer_contract'] = json.loads((runs / 'observer-contract.json').read_text())
receipt['nonharness_check'] = json.loads((runs / 'nonharness-check.json').read_text())
for name in ['default-scheduling.json', 'remaining-reschedule.json', 'options-reschedule.json']:
    receipt[name.removesuffix('.json')] = json.loads((runs / name).read_text())
a.receipt.parent.mkdir(parents=True, exist_ok=True)
a.receipt.write_text(json.dumps(receipt, indent=2) + '\n')
print('PASS: 165 cases per backend, 34 preserved-baseline cases, 60 option executions; fresh strict diffs:', a.receipt)
