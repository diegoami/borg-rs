"""Strict validation shared by coverage reporting and independent-group joins."""
import hashlib
import json
from pathlib import Path

ALL_SUITES = ('smoke', 'full', 'deep', 'classes', 'config', 'options',
              'captured-rogue', 'crash-fixes', 'extended', 'natural')
ROOT = Path(__file__).resolve().parents[2]


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def values(path):
    return dict(line.split('=', 1) for line in path.read_text().splitlines()
                if '=' in line)


def validate_group(matrix, suite, build_id):
    assert suite in ALL_SUITES, suite
    directory = matrix / suite
    profile_directory = directory.resolve().parent / 'profiles' / suite
    assert (directory / 'DONE').exists() and not (directory / 'FAILED').exists(), suite
    recipe = ROOT / 'harness' / 'suites' / suite
    names = [line.split('#', 1)[0].split()[0] for line in recipe.read_text().splitlines()
             if line.split('#', 1)[0].strip()]
    manifest = values(directory / 'manifest')
    assert manifest['suite_sha256'] == sha256(recipe), suite
    for key in ('binary_sha256', 'binary_sha256_end', 'build_id_binary_sha256'):
        assert manifest[key] == build_id['binary_sha256'], (suite, key)
    for key in ('build_id_source_sha256', 'checkout_source_sha256'):
        assert manifest[key] == build_id['source_sha256'], (suite, key)
    assert manifest['overrides'] == 'none' and manifest['checkpoint_every'] == '0', suite
    assert manifest['borg_impl_env'] == manifest['binary_impl'] == 'rust', suite
    entries = json.loads((directory / 'profiles.json').read_text())
    assert len(names) == len(set(names)) and set(entries) == set(names), suite
    assert {p.name for p in directory.iterdir() if p.is_dir()} == set(names), suite
    diff = (matrix / (suite + '.diff')).read_text()
    for text in (f'{len(entries)}/{len(entries)} runs identical',
                 f'messages.log compared in {len(entries)} run(s)',
                 f'state.log compared in {len(entries)} run(s)'):
        assert text in diff, (suite, text)
    profiles = []
    for name, item in entries.items():
        process = json.loads((directory / name / 'process.json').read_text())
        assert process['status'] == 'success', (suite, name)
        assert process['engine_returncode'] == process['exit_code'] == 0, (suite, name)
        assert process['checkpoint_count'] == 0 and not process['checkpoint_errors'], (suite, name)
        assert process['executable_sha256_before'] == process['executable_sha256_after'] == build_id['binary_sha256'], (suite, name)
        assert process['child_pid'] == item['child_pid'], (suite, name)
        profile = Path(item['profile'])
        assert profile.parent.resolve() == profile_directory.resolve(), (suite, name, 'profile group')
        assert profile.name.endswith('-%d.profraw' % process['child_pid']), (suite, name)
        assert profile.stat().st_size == item['bytes'] > 0 and sha256(profile) == item['sha256'], (suite, name)
        profiles.append(profile)
    assert len(set(profiles)) == len(names), suite
    assert {p.resolve() for p in profile_directory.glob('*.profraw')} == {p.resolve() for p in profiles}, (suite, 'unexpected or missing process profile')
    return profiles
