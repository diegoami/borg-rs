#!/usr/bin/env python3
"""Synthetic controls for harness/diff's last-key/game_turns consistency check.

A run's last key must be at summary.game_turns, except for an exact turn
stop, a stall stop or a Borg self-stop (borg_stopped), where the game may
finish the in-flight turn after the last key. The exception never allows a
key after the recorded end, and other stop reasons stay strict.
Run: python3 harness/regress/diff-consistency-test.py
"""
import importlib.machinery
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
diff = importlib.machinery.SourceFileLoader('harness_diff', str(ROOT / 'harness/diff')).load_module()


def keys(*turns):
    return ''.join(f'{t} 1 35 0\n' for t in turns).encode()


def problems(data, **summary):
    summary.setdefault('keys', len(data.splitlines()))
    return [p for p in diff.consistency('t', data, summary) if 'last key' in p]


data = keys(30096, 30106)
# milestone4 opt-stopclevel-war: auto-stop inside a throw, game ends one turn later
assert not problems(data, stop_reason='borg_stopped', game_turns=30116)
# a self-stop on the key turn stays valid
assert not problems(data, stop_reason='borg_stopped', game_turns=30106)
# never a key after the recorded end
assert problems(data, stop_reason='borg_stopped', game_turns=30100)
# other stop reasons remain strict
assert problems(data, stop_reason='death', game_turns=30116)
assert problems(data, stop_reason='turns', game_turns=30116)
# the existing exact-turns and stall exceptions are unchanged
assert not problems(data, stop_reason='turns', exact_turns=True, game_turns=30116)
assert not problems(data, stop_reason='stalled', game_turns=30116)
print('diff consistency controls: 7/7 passed')
