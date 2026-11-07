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


def frozen_run(bundle, output):
    record = json.loads((bundle / 'recording.json').read_text())
    profile = record['profile']
    runtime = bundle / 'runtime'
    env = {k: v for k, v in os.environ.items() if not k.startswith(('BORG_', 'ANGBAND_', 'LLVM_PROFILE'))}
    env.update(BORG_IMPL='rust', ANGBAND_BIN=str(runtime / 'game/angband'),
               SAVE_ROOT=str(runtime / 'harness/saves'), BORGH_DATA_ROOT=str(runtime / 'game/lib'),
               RUN_TIMEOUT_SECONDS=str(profile['wall_seconds']), CHECKPOINT_EVERY=str(profile['checkpoint_every']),
               LC_ALL='C.UTF-8', LANG='C.UTF-8', TZ='UTC')
    return subprocess.run([str(runtime / 'harness/run.sh'), str(output), str(profile['seed']),
                           str(profile['race']), str(profile['class']), str(profile['turn_limit']), profile['flags']],
                          env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)


def record_run(args):
    save = data_root() / 'save' / character_name(args.character)
    with character_lock(save):
        if args.from_backup is not None:
            saves.restore(save, args.from_backup)
        directory, binary, identity = build()
        origin = inspect_save(save, directory, binary) if save.exists() and not args.new else None
        if origin and origin['dead']:
            raise ValueError('Character is dead; use --from-backup or --new')
        if origin and ((args.race is not None and args.race != origin['race']) or (args.cls is not None and args.cls != origin['class'])):
            raise ValueError('Requested race/class differs from the loaded character')
        initial = saves.backup(save)
        if origin:
            origin = inspect_save(save.parent / 'backups' / save.name / 'save.1', directory, binary)
            if origin['dead']:
                raise ValueError('Character changed to a dead save; named file preserved')
        stamp = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
        bundle = data_root() / 'recordings' / (stamp + '-' + args.character)
        runtime = bundle / 'runtime'
        (runtime / 'harness/saves').mkdir(parents=True)
        (runtime / 'game/lib').mkdir(parents=True)
        shutil.copy2(binary, runtime / 'game/angband')
        for name in ('gamedata', 'customize', 'screens', 'help'):
            shutil.copytree(ROOT / 'angband/lib' / name, runtime / 'game/lib' / name)
        for name in ('run.sh', 'supervise.py'):
            shutil.copy2(ROOT / 'harness' / name, runtime / 'harness' / name)
        config = data_root() / 'user/borg.txt'
        shutil.copy2(config if config.exists() else ROOT / 'harness/borg.txt', runtime / 'harness/borg.txt')
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
        record = {'schema': SCHEMA, 'status': 'running', 'character': args.character,
                  'profile': profile, 'build_id': identity, 'origin': origin,
                  'original_named_sha256': initial, 'run_kind': 'backup branch' if args.from_backup is not None else 'save continuation; fresh Borg' if origin else 'new birth',
                  'rng_origin': 'saved engine RNG preserved; fresh Borg' if origin else 'seeded birth',
                  'calendar': 'seeded headless harness holiday context pinned to epoch',
                  'verification_model': 'rerun original Rust Borg from original origin; exact keys/messages/player-state/terminal frames/final save and scheduled world saves; not whole-Borg private-state shadow',
                  'runtime_sha256': {str(p.relative_to(bundle)): sha(p) for p in runtime.rglob('*') if p.is_file()}}
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


def verify(bundle):
    record = validate(bundle)
    source_hash = sha(Path(__file__))
    manifest_hash = sha(bundle / 'recording.json')
    cached = bundle / 'verification.json'
    if cached.exists():
        prior = json.loads(cached.read_text())
        if prior.get('verified') is True and prior.get('manifest_sha256') == manifest_hash and prior.get('verifier_sha256') == source_hash:
            return prior
    with tempfile.TemporaryDirectory(prefix='borg-rust-replay-') as temp:
        output = Path(temp) / 'run'
        result = frozen_run(bundle, output)
        if result.returncode:
            raise ValueError('Replay failed; original evidence preserved: ' + result.stdout[-1500:])
        process = json.loads((output / 'process.json').read_text())
        if process['status'] != 'success' or process['engine_returncode'] != 0:
            raise ValueError('Replay process did not complete successfully')
        for filename in ('keys.log', 'messages.log', 'state.log', 'frames.jsonl', 'final.save', 'config.txt'):
            if sha(output / filename) != sha(bundle / 'run' / filename):
                raise ValueError('Replay diverges: ' + filename)
        before, after = (json.loads((p / 'summary.json').read_text()) for p in (bundle / 'run', output))
        before.pop('wall_seconds', None);after.pop('wall_seconds', None)
        if before != after:
            raise ValueError('Replay summary diverges')
        originals = set(p.name for p in (bundle / 'run/checkpoints').glob('*.save'))
        repeated = set(p.name for p in (output / 'checkpoints').glob('*.save'))
        if originals != repeated:
            raise ValueError('Replay checkpoint set diverges')
        for filename in (bundle / 'run/checkpoints').glob('*.save'):
            if sha(filename) != sha(output / 'checkpoints' / filename.name):
                raise ValueError('Replay world snapshot diverges: ' + filename.name)
    proof = {'verified': True, 'manifest_sha256': manifest_hash, 'verifier_sha256': source_hash,
             'model': record['verification_model'], 'original_death_not_new_sample': record['summary']['dead']}
    write(cached, proof)
    return proof
