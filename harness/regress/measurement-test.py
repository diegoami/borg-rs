#!/usr/bin/env python3
"""Integration controls for borgh measurement options (build C and Rust first).

Runs into a fresh output directory. Native birth expectations are from the
better-borg-c observer, not another invocation of borgh.
"""
import concurrent.futures
import importlib.machinery
import json
import os
import shutil
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(tempfile.mkdtemp(prefix='measurement-', dir=ROOT / 'harness/out'))


def run(impl, label, seed=101, turns=20000, flags='-', extra=(), save_root=None):
    out = OUT / impl / label
    args = [str(ROOT / 'harness/run.sh'), str(out), str(seed), '0', '0', str(turns), flags, '--', *extra]
    subprocess.run(args, env={**os.environ, 'BORG_IMPL': impl, 'RUN_TIMEOUT_SECONDS': '180',
                        'SAVE_ROOT': str(save_root or ROOT / 'harness/saves')},
                   check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return json.loads((out / 'summary.json').read_text())


def log(impl, name, channel='keys.log'):
    return (OUT / impl / name / channel).read_bytes()


def states(path):
    rows = []
    for line in path.read_text().splitlines():
        prefix, kind, *values = line.split()
        assert prefix == 'STATE'
        row = dict(value.split('=', 1) for value in values)
        assert len(row['rng']) == 32 * 8 and all(c in '0123456789abcdef' for c in row['rng'])
        rows.append((kind, {key: int(value) if key != 'rng' else value for key, value in row.items()}))
    return rows


def check_impl(impl):
    plain = run(impl, 'plain')
    path = OUT / impl / 'states' / 'states.log'
    measured = run(impl, 'states', extra=['-states', str(path)])
    stalled = run(impl, 'stall-off-boundary', extra=['-stall-decisions', '1000000000'])
    for name, summary in [('states', measured), ('stall-off-boundary', stalled)]:
        assert log(impl, 'plain') == log(impl, name)
        for channel in ['messages.log', 'state.log']:
            assert log(impl, 'plain', channel) == log(impl, name, channel)
        assert {k: v for k, v in plain.items() if k != 'wall_seconds'} == {
            k: v for k, v in summary.items() if k != 'wall_seconds'}
    rows = states(path)
    assert rows[0][0] == 'start' and rows[0][1]['decisions'] == 0
    assert rows[-1][0] == plain['stop_reason']
    assert [r['decisions'] for kind, r in rows if kind == 'progress'] == list(
        range(100, plain['borg_decisions'] + 1, 100))
    last = rows[-1][1]
    for field, summary in [('turn', 'game_turns'), ('energy', 'player_energy'),
                           ('decisions', 'borg_decisions'), ('max_level', 'max_clevel'),
                           ('hp', 'hp'), ('max_hp', 'max_hp'), ('borg_rng', 'borg_rng')]:
        assert last[field] == plain[summary], field
    assert plain['calendar'] == 'fixed'
    assert last['rng'] == plain['game_rng']['rng']
    exact = run(impl, 'exact', extra=['-exact-turns'])
    assert exact['game_turns'] == 20000 and exact['stop_reason'] == 'turns'
    assert log(impl, 'plain').startswith(log(impl, 'exact'))
    assert exact['keys'] < plain['keys']
    stall = run(impl, 'stall', extra=['-stall-decisions', '1'])
    assert stall['stop_reason'] == 'stalled' and stall['stall_decisions'] == 1
    assert stall['stall_turn'] == stall['game_turns']
    assert log(impl, 'plain').startswith(log(impl, 'stall'))
    lunal_flags = 'menu,selflunal,key-limit=3000,save=lunal-ready.save'
    fixture_root = ROOT / 'harness/saves/synthetic-tactics'
    run(impl, 'lunal-base', seed=7200, turns=10000, flags=lunal_flags, save_root=fixture_root)
    lunal = run(impl, 'lunal-stall', seed=7200, turns=10000, flags=lunal_flags,
                extra=['-stall-decisions', '100'], save_root=fixture_root)
    assert lunal['stop_reason'] == 'stalled' and lunal['stall_decisions'] == 100
    assert lunal['stall_energy_delta'] > 0
    assert log(impl, 'lunal-base').startswith(log(impl, 'lunal-stall'))
    configured = run(impl, 'config', turns=1000, flags='munchkin,borg_munchkin_level=7')
    assert configured['effective_settings']['borg_munchkin_start'] == 1
    assert configured['effective_settings']['borg_munchkin_level'] == 7
    # Frozen native-observer result.json births (source pins in birth-reference.json).
    for ref in json.loads((Path(__file__).with_name('measurement-birth-reference.json')).read_text()):
        label = 'random-' + str(ref['seed'])
        summary = run(impl, label, seed=ref['seed'], turns=100,
                      extra=['-random-birth', '-exact-turns'])
        assert (summary['race'], summary['class']) == (ref['race'], ref['class']), summary
        assert summary['random_birth'] and summary['game_turns'] == 100
    for label, turns, extra in [
            ('bad-stall', '100', ['-stall-decisions', '0']),
            ('bad-exact', '0', ['-exact-turns']),
            ('bad-stream', '100', ['-states', '/dev/full'])]:
        out = OUT / impl / label
        result = subprocess.run([str(ROOT / 'harness/run.sh'), str(out), '101', '0', '0', turns,
                                 '--', *extra], env={**os.environ, 'BORG_IMPL': impl},
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        assert result.returncode != 0, label
        process = json.loads((out / 'process.json').read_text())
        assert process['engine_returncode'] != 0, (label, process)
    return plain


# Exercise the actual production decision function with a frozen world clock
# and rising energy, including turn advance/backwards and an inner no-key loop.
source = (ROOT / 'angband/src/main-borgh.c').read_text()
body = source.split('void borgh_decision(uint32_t rng)\n', 1)[1].split('\nstatic void borgh_tick', 1)[0]
fixture = r'''
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
struct player { unsigned long total_energy; } p, *player = &p;
static bool borg_started = true;
static uint32_t borg_rng;
static unsigned long decisions, stall_start_decisions, stall_start_energy;
static long turn = 1045, stall_turn, opt_stall_decisions = 3;
static int stopped;
static void borgh_measure_state(const char *kind) { (void)kind; }
static void borgh_finish(const char *reason) {
    assert(!strcmp(reason, "stalled")); ++stopped;
}
void borgh_decision(uint32_t rng)
''' + body + r'''
int main(void) {
    borgh_decision(17);
    p.total_energy += 100; borgh_decision(18);
    p.total_energy += 100; borgh_decision(19);
    assert(!stopped);
    p.total_energy += 100; borgh_decision(20);
    assert(stopped == 1 && decisions - stall_start_decisions == 3);
    assert(p.total_energy - stall_start_energy == 300 && borg_rng == 20);
    ++turn; borgh_decision(21); assert(stopped == 1);
    --turn; borgh_decision(22); assert(stopped == 1);
    opt_stall_decisions = 0;
    for (int i = 0; i < 200; ++i) borgh_decision(23);
    assert(stopped == 1);
    return 0;
}
'''
(OUT / 'stall-fixture.c').write_text(fixture)
subprocess.run(['cc', '-std=c99', '-Wall', '-Wextra', '-Werror', str(OUT / 'stall-fixture.c'),
                '-o', str(OUT / 'stall-fixture')], check=True)
subprocess.run([str(OUT / 'stall-fixture')], check=True)
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
    summaries = list(pool.map(check_impl, ['c', 'rust']))
assert {k: v for k, v in summaries[0].items() if k != 'wall_seconds'} == {
    k: v for k, v in summaries[1].items() if k != 'wall_seconds'}
assert log('c', 'states', 'states.log') == log('rust', 'states', 'states.log')
for name in ['plain', 'states', 'exact', 'stall', 'lunal-base', 'lunal-stall', 'config', 'random-2624955987',
             'random-2624955988', 'random-3167239341']:
    assert log('c', name) == log('rust', name), name
diff = importlib.machinery.SourceFileLoader('harness_diff', str(ROOT / 'harness/diff')).load_module()
summary = summaries[0]
assert not diff.consistency('valid', log('c', 'plain'), summary)
corrupt = dict(summary)
del corrupt['effective_settings']
assert any('missing measurement field effective_settings' in p
           for p in diff.consistency('corrupt', log('c', 'plain'), corrupt))
corrupt = dict(summary)
del corrupt['calendar']
assert diff.consistency('missing calendar', log('c', 'plain'), corrupt)
legacy = {k: v for k, v in summary.items() if k not in diff.MEASUREMENT_FIELDS}
assert not diff.consistency('legacy', log('c', 'plain'), legacy)
corrupt = dict(summary, borg_rng=summary['borg_rng'] + 1)
corrupt_dir = OUT / 'corrupt-rng-control'
shutil.copytree(OUT / 'c/states', corrupt_dir)
(corrupt_dir / 'summary.json').write_text(json.dumps(corrupt))
assert not diff.compare_run('changed RNG', str(OUT / 'c/plain'), str(corrupt_dir))
print('PASS: C/Rust stream, RNG, settings, frozen-clock stall, exact cap, three native births:', OUT)
