"""Rust adapters for bounded collection and the explicit owner-review queue.

Adapted from better-borg-c 22f817c96843; GPL-2.0-or-later or Angband licence.
See IMPORTED.json and angband/docs/copying.rst. No gameplay decisions here.
"""
from contextlib import contextmanager
from datetime import datetime, timezone
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import uuid

import borg_runtime as runtime
import borg_deaths as deaths

ROOT = runtime.ROOT
TOOLS = ('produce_death.py', 'replay_next_death.py', 'replay-borg.py',
         'tools/death_workflow.py', 'tools/borg_runtime.py', 'tools/borg_deaths.py',
         'tools/borg_saves.py', 'tools/borg_gui.py', 'tools/borg_replay_viewer.py',
         'harness/run.sh', 'harness/supervise.py', 'harness/identity.sh')
LOADED_TOOLS = {name: runtime.sha(ROOT / name) for name in TOOLS}


def read(path):
    return json.loads(path.read_text())


sha = runtime.sha


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + '.' + uuid.uuid4().hex + '.tmp')
    runtime.write(temporary, value)
    temporary.replace(path)


def now():
    return datetime.now(timezone.utc).isoformat()


def git(*args, binary=False):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], text=not binary)


def primary_root():
    common = Path(git('rev-parse', '--git-common-dir').strip())
    return (ROOT / common).resolve().parent


def artifacts():
    override = os.environ.get('BORG_DEATH_WORKFLOW_DIR')
    return Path(override).resolve() if override else primary_root() / 'harness/out/death-workflow'


@contextmanager
def locked(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('a') as handle:
        fcntl.flock(handle, fcntl.LOCK_EX)
        yield


def source_commit(ref):
    return git('rev-parse', '--verify', '--end-of-options', ref + '^{commit}').strip()


def source_inputs(commit):
    rows = []
    for item in git('ls-tree', '-r', '-z', commit, '--', 'angband', 'borg-rs', 'Cargo.toml', 'Cargo.lock', binary=True).split(b'\0'):
        if not item:
            continue
        metadata, name = item.split(b'\t', 1)
        _, kind, blob = metadata.split()
        if kind != b'blob':
            raise ValueError('Unsupported non-blob source input: ' + name.decode())
        rows.append((name, blob))
    if not rows:
        raise ValueError('No Rust engine inputs at selected source')
    payload = b'borg-rs build inputs v1 impl=rust\n' + b''.join(blob + b' ' + name + b'\n' for name, blob in sorted(rows))
    return hashlib.sha256(payload).hexdigest()


def tool_hashes():
    current = {name: sha(ROOT / name) for name in TOOLS}
    if current != LOADED_TOOLS:
        raise ValueError('Observation tools changed after this process loaded them')
    return current


def family(record, assessment):
    if assessment.get('encounter_family_qualification', {}).get('status') == 'verified':
        return assessment.get('encounter_family') or deaths.canonical_identity(record)['encounter_family']
    return deaths.canonical_identity(record)['encounter_family']


def reviews():
    baseline = deaths.registry_root() / 'owner-reviews.json'
    values = dict(read(baseline).get('reviews', {})) if baseline.exists() else {}
    local = artifacts() / 'owner-reviews.json'
    if local.exists():
        values.update(read(local)['reviews'])
    return values


def cases():
    result = []
    for path in sorted(deaths.registry_root().glob('D*/record.json')):
        record = read(path)
        if record['schema'] != deaths.SCHEMA:
            raise ValueError('Unknown death dossier schema: ' + str(path))
        assessment = read(path.with_name('assessment.json'))
        recording = record.get('recording')
        status = 'legacy: no full recording'
        if recording:
            bundle = Path(recording['raw_directory']).resolve()
            try:
                if sha(bundle / 'recording.json') != recording['manifest_sha256']:
                    raise ValueError('recording manifest changed')
                manifest = read(bundle / 'recording.json')
                if manifest['schema'] == runtime.SCHEMA:
                    runtime.validate(bundle)
                else:
                    import borg_gui
                    borg_gui.validate(bundle)
                if not (bundle / 'frames.jsonl.gz').is_file() or not (bundle / 'runtime/game/angband').is_file():
                    raise ValueError('full runtime/frames missing')
                status = 'recording available'
            except (OSError, ValueError, KeyError) as error:
                status = 'recording unavailable; restore original bundle: ' + str(error)
        result.append({'case': record['id'], 'directory': str(path.parent), 'family': family(record, assessment),
                       'cause': record['raw_cause'], 'turn': record['identity']['turn'], 'recording': status})
    return result


def queue():
    reviewed, grouped = reviews(), {}
    for case in cases():
        group = grouped.setdefault(case['family'], {'family': case['family'], 'cases': [],
            'reviewed': reviewed.get(case['family'], {}).get('reviewed', False), 'playable': None})
        group['cases'].append(case)
        if group['playable'] is None and case['recording'] == 'recording available':
            group['playable'] = case
    return list(grouped.values())


def choose_case(case_id):
    matches = [case for case in cases() if case_id in (case['case'], Path(case['directory']).name)]
    if len(matches) != 1:
        raise ValueError('Choose an existing case ID, e.g. D0001')
    return matches[0]


def mark(case_id, reviewed):
    case = choose_case(case_id)
    path = artifacts() / 'owner-reviews.json'
    with locked(path.with_suffix('.lock')):
        ledger = read(path) if path.exists() else {'schema': 'borg-rs/owner-reviews@1', 'reviews': {}}
        ledger['reviews'][case['family']] = {'reviewed': reviewed, 'case': case['case'],
            'recorded_at': now(), 'reason': 'explicit owner command; separate from technical assessment'}
        write(path, ledger)
    return case


def validate_build(directory, expected):
    identity = dict(line.split('=', 1) for line in (directory / 'BUILD_ID').read_text().splitlines())
    binary = directory / 'game/angband'
    cache = (directory / 'CMakeCache.txt').read_text()
    if identity.get('impl') != 'rust' or identity.get('source_sha256') != expected or sha(binary) != identity.get('binary_sha256'):
        raise ValueError('Build identity/binary must match the committed Rust source pin')
    if 'SUPPORT_BORG_HARNESS:BOOL=ON' not in cache or re.search(r'^BORG_RS_FEATURES:STRING=.+$', cache, re.M):
        raise ValueError('Use an ordinary recording-capable Rust harness build without test features')
    if subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'binary-impl', str(binary)], text=True).strip() != 'rust':
        raise ValueError('Executable is not the Rust Borg')
    return binary, identity


def prepare_runtime(args, commit, hashes):
    """Freeze committed source/data and current observation tools before play."""
    expected = source_inputs(commit)
    fingerprint = hashlib.sha256(json.dumps({'commit': commit, 'tools': hashes,
                         'explicit_build': str(args.build.resolve()) if args.build else None}, sort_keys=True).encode()).hexdigest()[:20]
    cache = primary_root() / 'build/death-workflow' / fingerprint
    manifest_path = cache / 'prepared.json'
    if manifest_path.exists():
        manifest = read(manifest_path)
        validate_prepared(manifest, commit, hashes)
        return manifest
    explicit = validate_build(args.build.resolve(), expected) if args.build else None
    cache.mkdir(parents=True, exist_ok=True)
    source = cache / 'source'
    if source.exists():
        raise ValueError('Incomplete prepared build retained; inspect before choosing a new cache path: ' + str(cache))
    source.mkdir()
    tool_source = cache / 'tool-source'
    for name, digest in hashes.items():
        target = tool_source / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / name, target)
        if sha(target) != digest:
            raise ValueError('Observation tool changed while freezing: ' + name)
    archive = cache / 'source.tar'
    subprocess.run(['git', '-C', str(ROOT), 'archive', '--format=tar', '-o', str(archive), commit,
                    'angband', 'borg-rs', 'Cargo.toml', 'Cargo.lock', 'harness/borg.txt'], check=True)
    with tarfile.open(archive) as saved:
        saved.extractall(source, filter='data')
    frontend = (source / 'angband/src/main-borgh.c').read_text()
    for flag in ('-frames', '-keys', '-save-final-state'):
        if flag not in frontend:
            raise ValueError('Selected source lacks required headless recording support: ' + flag)
    if args.build:
        directory = args.build.resolve()
        binary, identity = explicit
    else:
        directory = cache / 'build'
        # The snapshot stamp is build metadata, not a game-source alteration.
        (source / 'angband/version').write_text(commit + '-death-workflow')
        env = {**os.environ, 'GIT_CEILING_DIRECTORIES': str(source), 'LC_ALL': 'C.UTF-8', 'LANG': 'C.UTF-8'}
        commands = [ ['cmake', '-S', str(source / 'angband'), '-B', str(directory), '-DCMAKE_BUILD_TYPE=Release',
                     '-DBORG_IMPL=rust', '-DSUPPORT_BORG_HARNESS=ON', '-DSUPPORT_BORG=ON', '-DBORG_RS_FEATURES=',
                     '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON', '-DSC_INSTALL=ON', '-DBUILD_DOC=OFF'],
                    ['cmake', '--build', str(directory), '--target', 'OurExecutable', '--parallel', os.environ.get('JOBS', '4')] ]
        for label, command in zip(('configure', 'build'), commands):
            print(label, directory, flush=True)
            with (cache / (label + '.log')).open('w') as log:
                subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
        binary = directory / 'game/angband'
        identity = {'impl': 'rust', 'git_rev': commit, 'inputs_dirty': 'no', 'git_dirty': 'no',
                    'source_sha256': expected, 'binary_sha256': sha(binary), 'binary': str(binary),
                    'source_snapshot_sha256': sha(archive), 'build_stamp': commit + '-death-workflow'}
        (directory / 'BUILD_ID').write_text(''.join(f'{key}={value}\n' for key, value in identity.items()))
        binary, identity = validate_build(directory, expected)
    frozen = cache / 'game/angband'
    frozen.parent.mkdir()
    shutil.copy2(binary, frozen)
    runner = cache / 'runner'
    runner.mkdir()
    for name in ('run.sh', 'supervise.py'):
        shutil.copy2(ROOT / 'harness' / name, runner / name)
    config = cache / 'borg.txt'
    text = (source / 'harness/borg.txt').read_text()
    for name, value in [('borg_cheat_death', 'FALSE'), ('borg_respawn_winners', 'FALSE'), ('borg_save_death', '1')]:
        text, count = re.subn(r'^' + name + r' = .*$', name + ' = ' + value, text, flags=re.M)
        if count != 1:
            raise ValueError('Missing/ambiguous native safety setting: ' + name)
    config.write_text(text)
    manifests = {str(p.relative_to(cache)): sha(p) for p in cache.rglob('*') if p.is_file() and p != manifest_path
                 and not p.is_relative_to(directory)}
    manifest = {'schema': 'borg-rs/death-prepared-runtime@1', 'source_commit': commit, 'source_inputs_sha256': expected,
                'tools_sha256': hashes, 'build_directory': str(directory), 'binary': str(frozen), 'build_id': identity,
                'source_root': str(source), 'runner_root': str(runner), 'tool_source_root': str(tool_source), 'config': str(config), 'cache': str(cache),
                'files_sha256': manifests, 'complete': True,
                'context': {'committed_source': commit, 'source_inputs_sha256': expected, 'tools_sha256': hashes,
                            'character_selection': 'prospective SHA-256-derived race/class; explicit ordinary native menus'}}
    write(manifest_path, manifest)
    validate_prepared(manifest, commit, hashes)
    return manifest


def validate_prepared(manifest, commit, hashes):
    if manifest.get('schema') != 'borg-rs/death-prepared-runtime@1' or not manifest.get('complete') or manifest.get('source_commit') != commit or manifest.get('tools_sha256') != hashes:
        raise ValueError('Prepared recording runtime/source/tools do not match')
    if manifest['source_inputs_sha256'] != source_inputs(commit):
        raise ValueError('Prepared source input identity differs')
    base = Path(manifest['cache']).resolve()
    build_directory = Path(manifest['build_directory']).resolve()
    actual = {str(p.relative_to(base)) for p in base.rglob('*') if p.is_file() and p != base / 'prepared.json' and not p.is_relative_to(build_directory)}
    if actual != set(manifest['files_sha256']):
        raise ValueError('Prepared input file set changed')
    for name, digest in manifest['files_sha256'].items():
        path = (base / name).resolve()
        if not path.is_relative_to(base) or sha(path) != digest:
            raise ValueError('Prepared input changed: ' + name)
    if sha(Path(manifest['binary'])) != manifest['build_id']['binary_sha256']:
        raise ValueError('Prepared Rust binary changed')
    if tool_hashes() != hashes:
        raise ValueError('Observation tools changed during collection')
