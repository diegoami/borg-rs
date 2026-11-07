"""Rust-Borg adapters for the imported native save and terminal viewer tools.

Record automated headless runs, freeze their origin/runtime and verify reruns.
Human GUI interventions require a separate recorder; they are not inferred.
"""
from contextlib import contextmanager
from datetime import datetime, timezone
import fcntl
import gzip
import hashlib
import json
import itertools
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile

import borg_saves as saves

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = 'borg-rs-headless-recording@1'


def write(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def data_root():
    return Path(os.environ.get('BORG_PLAY_DATA_DIR', ROOT / '.angband-rust')).resolve()


def character_name(name):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,63}', name):
        raise ValueError('Character name must be 1-64 letters/digits/underscore/dot/hyphen, starting with a letter or digit')
    return name


@contextmanager
def character_lock(save):
    lock = save.parent / '.locks' / (save.name + '.lock')
    lock.parent.mkdir(parents=True, exist_ok=True)
    with lock.open('a') as f:
        try:
            fcntl.flock(f, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise ValueError('Character is already open through a launcher') from error
        yield


def build():
    directory = ROOT / 'build/rust'
    current = subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'source-hash', 'rust'], text=True).strip()
    identity = directory / 'BUILD_ID'
    values = dict(line.split('=', 1) for line in identity.read_text().splitlines()) if identity.exists() else {}
    binary = directory / 'game/angband'
    if values.get('impl') != 'rust' or values.get('source_sha256') != current or not binary.exists() or sha(binary) != values.get('binary_sha256'):
        print('Building verified Rust headless engine...', flush=True)
        subprocess.run([str(ROOT / 'harness/build.sh')], env={**os.environ, 'BORG_IMPL': 'rust'}, check=True)
        values = dict(line.split('=', 1) for line in identity.read_text().splitlines())
    assert subprocess.check_output([str(ROOT / 'harness/identity.sh'), 'binary-impl', str(binary)], text=True).strip() == 'rust'
    return directory, binary, values


def inspect_save(save, directory, binary):
    """Native deserialize, engine fields only, exit before Borg/world execution."""
    source = ROOT / 'tools/inspect-save.c'
    recipe = ['cc', '-O2', '-DALLOW_BORG', '-DBORG_HARNESS', '-DNDEBUG', '-I' + str(ROOT / 'angband/src')]
    key = hashlib.sha256((sha(source) + sha(binary) + repr(recipe)).encode()).hexdigest()[:20]
    output = ROOT / 'build/save-inspect' / key
    reader = output / 'angband'
    if not reader.exists():
        output.mkdir(parents=True, exist_ok=True)
        obj = output / 'inspect.o'
        subprocess.run([*recipe, '-c', str(source), '-o', str(obj)], check=True)
        argv = [v for v in shlex.split((directory / 'CMakeFiles/OurExecutable.dir/link.txt').read_text())
                if not v.startswith('-Wl,--dependency-file=')]
        argv[argv.index('-o') + 1] = str(reader)
        argv += [str(obj), '-Wl,--wrap=savefile_load']
        subprocess.run(argv, cwd=directory, check=True)
        write(output / 'identity.json', {'game': sha(binary), 'source': sha(source), 'reader': sha(reader)})
    meta = json.loads((output / 'identity.json').read_text())
    if meta['game'] != sha(binary) or meta['source'] != sha(source) or meta['reader'] != sha(reader):
        raise ValueError('Save metadata reader changed; preserve cache and investigate')
    with tempfile.TemporaryDirectory(prefix='borg-save-info-') as tmp:
        cmd = [str(reader), '-mborgh', '-duser=' + tmp, '-dgamedata=' + str(ROOT / 'angband/lib/gamedata'),
               '--', '-seed', '1', '-race', '0', '-class', '0', '-turns', '2147483647', '-load', str(save.resolve())]
        result = subprocess.run(cmd, cwd=directory / 'game', capture_output=True, text=True, timeout=15)
        if result.returncode:
            raise ValueError('Cannot inspect this save; restore a living backup or start --new: ' + result.stderr[-500:])
        return json.loads(result.stdout)


def frozen_run(bundle, output, timeout=None, *, engine=None, extra_observations=True, takeover=None):
    bundle = bundle.resolve()
    record = json.loads((bundle / 'recording.json').read_text())
    profile = record['profile']
    runtime = bundle / 'runtime'
    env = {k: v for k, v in os.environ.items() if not k.startswith(('BORG_', 'ANGBAND_', 'LLVM_PROFILE'))}
    env.update(BORG_IMPL='rust', ANGBAND_BIN=str(Path(engine).resolve() if engine is not None else runtime / 'game/angband'),
               SAVE_ROOT=str(runtime / 'harness/saves'), BORGH_DATA_ROOT=str(runtime / 'game/lib'),
               RUN_TIMEOUT_SECONDS=str(min(profile['wall_seconds'], timeout) if timeout is not None else profile['wall_seconds']), CHECKPOINT_EVERY=str(profile['checkpoint_every']),
               STALL_TIMEOUT_SECONDS=str(profile.get('stall_seconds', 0)),
               LC_ALL='C.UTF-8', LANG='C.UTF-8', TZ='UTC')
    if not extra_observations:
        env['BORG_NO_EXTRA_OBSERVATIONS'] = '1'
    if takeover is not None:
        env.update(BORG_TAKEOVER_TURN=str(takeover['turn']), BORG_TAKEOVER_DIR=str(takeover['directory']))
    return subprocess.run([str(runtime / 'harness/run.sh'), str(output), str(profile['seed']),
                           str(profile['race']), str(profile['class']), str(profile['turn_limit']), profile['flags']],
                          env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)


def record_run(args, *, prepared=None, profile_root=None, export_death=True):
    data_directory = Path(profile_root).resolve() if profile_root is not None else data_root()
    source_root = Path(prepared['source_root']) if prepared else ROOT
    runner_root = Path(prepared['runner_root']) if prepared else ROOT / 'harness'
    save = data_directory / 'save' / character_name(args.character)
    with character_lock(save):
        if prepared is None:
            directory, binary, identity = build()
        else:
            if not args.new or args.from_backup is not None:
                raise ValueError('Prepared collection runtimes only support fresh births')
            directory, binary, identity = Path(prepared['build_directory']), Path(prepared['binary']), prepared['build_id']
        if args.from_backup is not None and args.from_backup not in range(1, 6):
            raise ValueError('Backup slot must be 1 through 5')
        candidate = save.parent / 'backups' / save.name / f'save.{args.from_backup}' if args.from_backup is not None else save
        if args.from_backup is not None and not candidate.is_file():
            raise ValueError('Backup does not exist: ' + str(candidate))
        origin = None
        with tempfile.TemporaryDirectory(prefix='borg-save-preflight-') as temporary:
            snapshot = Path(temporary) / 'origin.save'
            if candidate.exists() and not args.new:
                shutil.copy2(candidate, snapshot)
                origin = inspect_save(snapshot, directory, binary)
                if origin['dead']:
                    raise ValueError('Character is dead; use --from-backup or --new')
                if (args.race is not None and args.race != origin['race']) or (args.cls is not None and args.cls != origin['class']):
                    raise ValueError('Requested race/class differs from the loaded character')
            if args.from_backup is not None:
                saves.restore(save, args.from_backup, validated_source=snapshot)
        initial = saves.backup(save)
        if origin:
            origin = inspect_save(save.parent / 'backups' / save.name / 'save.1', directory, binary)
            if origin['dead']:
                raise ValueError('Character changed to a dead save; named file preserved')
        stamp = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
        bundle = data_directory / 'recordings' / (stamp + '-' + args.character)
        runtime = bundle / 'runtime'
        (runtime / 'harness/saves').mkdir(parents=True)
        (runtime / 'game/lib').mkdir(parents=True)
        shutil.copy2(binary, runtime / 'game/angband')
        for name in ('gamedata', 'customize', 'screens', 'help'):
            shutil.copytree(source_root / 'angband/lib' / name, runtime / 'game/lib' / name)
        for name in ('run.sh', 'supervise.py'):
            shutil.copy2(runner_root / name, runtime / 'harness' / name)
        if prepared:
            shutil.copytree(Path(prepared['tool_source_root']), runtime / 'tool-source')
        config = data_directory / 'user/borg.txt'
        shutil.copy2(Path(prepared['config']) if prepared else config if config.exists() else ROOT / 'harness/borg.txt', runtime / 'harness/borg.txt')
        cfg = runtime / 'harness/borg.txt'
        cfg.write_text(re.sub(r'^borg_delay_factor = .*$', 'borg_delay_factor = 0', cfg.read_text(), flags=re.M))
        flags = f'menu,terminal-save,frames,key-limit={args.keys}'
        if origin:
            shutil.copy2(save.parent / 'backups' / save.name / 'save.1', runtime / 'harness/saves/origin.save')
            if sha(runtime / 'harness/saves/origin.save') != initial:
                raise ValueError('Frozen starting save differs from its verified backup')
            flags += ',save=origin.save,keep-load-rng'
        profile = {'seed': args.seed, 'race': origin['race'] if origin else (args.race or 0),
                   'class': origin['class'] if origin else (args.cls or 0),
                   'turn_limit': (origin['turn'] if origin else 0) + args.turns,
                   'flags': flags, 'wall_seconds': args.wall_seconds, 'checkpoint_every': args.checkpoint_every}
        if getattr(args, 'stall_seconds', 0):
            profile['stall_seconds'] = args.stall_seconds
        record = {'schema': SCHEMA, 'status': 'running', 'character': args.character,
                  'profile': profile, 'build_id': identity, 'origin': origin,
                  'original_named_sha256': initial, 'run_kind': 'backup branch' if args.from_backup is not None else 'save continuation; fresh Borg' if origin else 'new birth',
                  'rng_origin': 'saved engine RNG preserved; fresh Borg' if origin else 'seeded birth',
                  'calendar': 'seeded headless harness holiday context pinned to epoch',
                  'verification_model': 'rerun original Rust Borg from original origin; exact keys/messages/player-state/terminal frames/final save and scheduled world saves; not whole-Borg private-state shadow',
                  'runtime_sha256': {str(p.relative_to(bundle)): sha(p) for p in runtime.rglob('*') if p.is_file()}}
        if prepared:
            record['collection_context'] = prepared['context']
        write(bundle / 'recording.json', record)
        result = frozen_run(bundle, bundle / 'run')
        (bundle / 'console.log').write_text(result.stdout)
        if result.returncode:
            record['status'] = 'failed'
            write(bundle / 'recording.json', record)
            raise ValueError(f'Headless run failed; named save preserved. Evidence: {bundle}')
        summary = json.loads((bundle / 'run/summary.json').read_text())
        process = json.loads((bundle / 'run/process.json').read_text())
        assert process['status'] == 'success' and process['engine_returncode'] == 0
        final = bundle / 'run/final.save'
        assert final.is_file() and final.stat().st_size
        for name, digest in record['runtime_sha256'].items():
            if sha(bundle / name) != digest:
                raise ValueError('Frozen runtime input changed; named save preserved: ' + name)
        frames = bundle / 'run/frames.jsonl'
        compressed = bundle / '.frames.jsonl.gz.partial'
        with frames.open('rb') as source, gzip.open(compressed, 'wb') as target:
            shutil.copyfileobj(source, target)
        with gzip.open(compressed, 'rb') as source:
            assert hashlib.sha256(source.read()).hexdigest() == sha(frames)
        os.replace(compressed, bundle / 'frames.jsonl.gz')
        record.update(status='complete', summary=summary, publication='pending')
        record['files_sha256'] = {str(p.relative_to(bundle)): sha(p) for p in bundle.rglob('*')
                                 if p.is_file() and p.name != 'recording.json'}
        write(bundle / 'recording.json', record)
        try:
            caps = json.loads((bundle / 'run/checkpoints.json').read_text())['captures'] if (bundle / 'run/checkpoints.json').exists() else []
            if caps and (summary['dead'] or initial is None):
                recovery = caps[-1] if summary['dead'] else caps[0]
                saves.backup(save, bundle / 'run/checkpoints' / recovery['file'])
                record['recovery_backup'] = {'slot': 1, 'turn': recovery['game_turn'],
                                             'kind': 'last living checkpoint' if summary['dead'] else 'first recorded checkpoint'}
            saves.publish(save, final, initial)
        except (OSError, ValueError) as error:
            record['publication'] = 'refused: ' + str(error)
            write(bundle / 'recording.json', record)
            raise ValueError(f'{error}; complete recording retained at {bundle}') from error
        record.update(publication='published', published_save_sha256=sha(save))
        write(bundle / 'recording.json', record)
        print(f"{summary['stop_reason']}: level{summary['clevel']}, depth{summary['depth']}, dead={summary['dead']}")
        print('Save:', save)
        print('Recording:', bundle)
        print('Replay:', ROOT / 'replay-borg.py', bundle)
        if export_death:
            import borg_deaths
            borg_deaths.maybe_register(bundle, issue=getattr(args, 'death_issue', None) or os.environ.get('BORG_DEATH_ISSUE'))
        return bundle


def validate(bundle, require_complete=True):
    bundle = bundle.resolve()
    record = json.loads((bundle / 'recording.json').read_text())
    if record.get('schema') != SCHEMA or (require_complete and record.get('status') != 'complete'):
        raise ValueError('Unsupported or incomplete recording; no verified replay claim is available')
    if not isinstance(record.get('files_sha256'), dict):
        if not require_complete:
            return record
        raise ValueError('Recording has no finalized file hashes')
    for name, digest in record['files_sha256'].items():
        path = (bundle / name).resolve()
        if not path.is_relative_to(bundle) or sha(path) != digest:
            raise ValueError('Recording file changed: ' + name)
    return record


@contextmanager
def replay_workspace(output):
    temporary = output is None
    workspace = Path(tempfile.mkdtemp(prefix='borg-rust-replay-')) if temporary else output.resolve()
    if not temporary:
        workspace.mkdir(parents=True, exist_ok=False)
    try:
        yield workspace
    except BaseException as error:
        write(workspace / 'verification.json', {'verified': False, 'error': str(error)})
        if temporary and isinstance(error, Exception):
            raise ValueError(f'{error}; failed replay evidence retained at {workspace}') from error
        raise
    else:
        if temporary:
            shutil.rmtree(workspace)



OBSERVATION_FIELDS = frozenset(('inventory', 'spells', 'max_hp', 'max_sp', 'caster'))


def compare_observation_extension(original, extended):
    """Only absent new observation fields may be added; every old value stays."""
    with original.open() as before, extended.open() as after:
        for number, (left, right) in enumerate(itertools.zip_longest(before, after), 1):
            if left is None or right is None:
                raise ValueError(f'Observation replay frame count diverges at row {number}')
            expected, actual = json.loads(left), json.loads(right)
            for field in OBSERVATION_FIELDS - expected.keys():
                actual.pop(field, None)
            if expected != actual:
                fields = sorted(key for key in expected.keys() | actual.keys() if expected.get(key) != actual.get(key))
                raise ValueError(f'Observation replay original frame diverges at row {number}: {fields}')


def verify(bundle, output=None, timeout=None, *, engine=None, observation_extension=False):
    if (engine is not None) != observation_extension or (observation_extension and output is None):
        raise ValueError('Observation rebuild verification requires its engine and retained output')
    bundle = bundle.resolve()
    record = validate(bundle)
    source_hash = sha(Path(__file__))
    manifest_hash = sha(bundle / 'recording.json')
    cached = bundle / 'verification.json'
    if engine is None and output is None and timeout is None and cached.exists():
        prior = json.loads(cached.read_text())
        if prior.get('verified') is True and prior.get('manifest_sha256') == manifest_hash and prior.get('verifier_sha256') == source_hash:
            return prior
    with replay_workspace(output) as workspace:
        output_run = workspace / 'run'
        result = frozen_run(bundle, output_run, timeout=timeout, **({"engine": engine} if engine is not None else {}))
        output_run_path = output_run
        if result.returncode:
            raise ValueError('Replay failed; original evidence preserved: ' + result.stdout[-1500:])
        process = json.loads((output_run_path / 'process.json').read_text())
        if process['status'] != 'success' or process['engine_returncode'] != 0:
            raise ValueError('Replay process did not complete successfully')
        for filename in ('keys.log', 'messages.log', 'state.log', 'frames.jsonl', 'final.save', 'config.txt'):
            if filename == 'frames.jsonl' and observation_extension:
                compare_observation_extension(bundle / 'run' / filename, output_run_path / filename)
                continue
            if sha(output_run_path / filename) != sha(bundle / 'run' / filename):
                raise ValueError('Replay diverges: ' + filename)
        native_original = {str(p.relative_to(bundle / 'run/archive')): p for p in (bundle / 'run/archive').rglob('*') if p.is_file()}
        native_repeated = {str(p.relative_to(output_run_path / 'archive')): p for p in (output_run_path / 'archive').rglob('*') if p.is_file()}
        if native_original.keys() != native_repeated.keys():
            raise ValueError('Replay native Borg archive set diverges')
        native_clock_differences = []
        for name, original in native_original.items():
            repeated = native_repeated[name]
            if sha(original) == sha(repeated):
                continue
            # Historical headless engines pin seasonal monster availability,
            # but their Rust death-log header reads the real wall clock. Keep
            # those original bytes/hashes; compare every other log byte.
            if name != 'borg-log.txt':
                raise ValueError('Replay native Borg archive diverges: ' + name)
            left, right = original.read_bytes().splitlines(keepends=True), repeated.read_bytes().splitlines(keepends=True)
            if len(left) != len(right):
                raise ValueError('Replay native death log length diverges')
            for number, (a, b) in enumerate(zip(left, right), 1):
                if a == b:
                    continue
                if number != 1 or not all(re.fullmatch(rb'[0-9]{4}/[0-9]{2}/[0-9]{2} [0-9]{2}:[0-9]{2}\n', value) for value in (a, b)):
                    raise ValueError('Replay native death log body diverges')
                native_clock_differences.append({'line': number, 'original': a.decode().strip(), 'rerun': b.decode().strip()})
        before, after = (json.loads((p / 'summary.json').read_text()) for p in (bundle / 'run', output_run_path))
        before.pop('wall_seconds', None);after.pop('wall_seconds', None)
        if before != after:
            raise ValueError('Replay summary diverges')
        originals = set(p.name for p in (bundle / 'run/checkpoints').glob('*.save'))
        repeated = set(p.name for p in (output_run_path / 'checkpoints').glob('*.save'))
        if originals != repeated:
            raise ValueError('Replay checkpoint set diverges')
        for filename in (bundle / 'run/checkpoints').glob('*.save'):
            if sha(filename) != sha(output_run_path / 'checkpoints' / filename.name):
                raise ValueError('Replay world snapshot diverges: ' + filename.name)
    proof = {'verified': True, 'manifest_sha256': manifest_hash, 'verifier_sha256': source_hash,
             'model': record['verification_model'], 'original_death_not_new_sample': record['summary']['dead'],
             'native_archive_body_verified': True, 'native_death_log_wall_clock_differences': native_clock_differences}
    if observation_extension:
        proof['observation_extension'] = {'fields': sorted(OBSERVATION_FIELDS), 'engine_sha256': sha(Path(engine)), 'original_engine_sha256': sha(bundle / 'runtime/game/angband'), 'scope': 'historical-source observation-only rebuild; every original channel/value and native save matches'}
    else:
        write(cached, proof)
    if output is not None:
        write(output.resolve() / 'verification.json', proof)
    return proof
