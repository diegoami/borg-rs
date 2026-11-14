#!/usr/bin/env python3
"""Publish one completed recording, without binaries or unrelated Git changes."""
from contextlib import contextmanager
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import subprocess
import tempfile
import zipfile

from recording_paths import data_root_for

MAX_ARCHIVE = 95 * 1024 * 1024
LOCAL_IGNORES = ('/save/', '/user/', '/archive/', '/panic/', '/scores/', '/recordings/*/', '/recordings/.export-*.tmp')
SCHEMAS = {'borg-rs-gui-recording@1', 'borg-rs-headless-recording@1'}


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def git(root, *args, check=True):
    try:
        result = subprocess.run(['git', '-C', str(root), *args], capture_output=True, text=True,
                                env={**os.environ, 'GIT_TERMINAL_PROMPT':'0'}, timeout=120)
    except subprocess.TimeoutExpired as error:
        raise ValueError(f'Git {args[0]} timed out; local recording/commit retained') from error
    if check and result.returncode:
        # Keep Git diagnostics private; a configured remote may contain credentials.
        common = subprocess.run(['git', '-C', str(root), 'rev-parse', '--absolute-git-dir'], capture_output=True, text=True)
        if common.returncode == 0:
            (Path(common.stdout.strip()) / 'recorder-publish.log').write_text(result.stdout + result.stderr)
        raise ValueError(f'Git {args[0]} failed; local recording/commit retained. See .git/recorder-publish.log')
    return result


def root_info(cemetery):
    root = Path(cemetery).expanduser().resolve()
    top = git(root, 'rev-parse', '--show-toplevel').stdout.strip()
    if Path(top).resolve() != root:
        raise ValueError('The cemetery must be the root of its own Git checkout')
    git(root, 'rev-parse', '--verify', 'HEAD')
    branch = git(root, 'symbolic-ref', '--short', 'HEAD').stdout.strip()
    git(root, 'remote', 'get-url', 'origin')
    remote = 'refs/remotes/origin/' + branch
    git(root, 'rev-parse', '--verify', remote)
    if git(root, 'rev-list', remote + '..HEAD').stdout.strip():
        raise ValueError('Cemetery has unpushed commits; push/review them explicitly before automatic publication')
    git(root, 'var', 'GIT_AUTHOR_IDENT')
    if git(root, 'ls-files', '-u').stdout:
        raise ValueError('Resolve the cemetery merge before publishing')
    if git(root, 'diff', '--cached', '--name-only', '-z').stdout:
        raise ValueError('Cemetery has staged changes; publishing leaves them untouched')
    return root, branch


def preflight(cemetery):
    root, branch = root_info(cemetery)
    if git(root, 'status', '--porcelain', '--untracked-files=no').stdout:
        raise ValueError('Commit or resolve existing cemetery edits before --auto-push')
    return root, branch


def safe_path(root, name):
    relative = PurePosixPath(name)
    if relative.is_absolute() or not relative.parts or any(p in ('..', '.') for p in relative.parts):
        raise ValueError('Unsafe recording member: ' + name)
    path = root.joinpath(*relative.parts)
    if not path.resolve().is_relative_to(root) or any(p.is_symlink() for p in [path, *path.parents] if p != root.parent):
        raise ValueError('Recording symlinks are not publishable: ' + name)
    return path


def binary(path, name):
    if not name.startswith('runtime/'):
        return False
    with path.open('rb') as stream:
        magic = stream.read(4)
    return name == 'runtime/game/angband' or magic[:2] == b'MZ' or magic == b'\x7fELF' or magic in (b'\xfe\xed\xfa\xce', b'\xfe\xed\xfa\xcf', b'\xcf\xfa\xed\xfe', b'\xce\xfa\xed\xfe')


def member(archive, name, source, expected):
    info = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
    info.compress_type = zipfile.ZIP_DEFLATED
    info.external_attr = (stat.S_IFREG | (source.stat().st_mode & 0o777)) << 16
    digest = hashlib.sha256()
    with source.open('rb') as inp, archive.open(info, 'w', force_zip64=True) as out:
        for chunk in iter(lambda: inp.read(1024 * 1024), b''):
            digest.update(chunk)
            out.write(chunk)
    if digest.hexdigest() != expected:
        raise ValueError('Recording changed while archiving: ' + str(source))


def build_export(bundle, destination):
    bundle = Path(bundle).resolve()
    record_path = safe_path(bundle, 'recording.json')
    manifest_hash = sha(record_path)
    record = json.loads(record_path.read_text())
    hashes = record.get('files_sha256')
    if record.get('schema') not in SCHEMAS or record.get('status') != 'complete' or not isinstance(hashes, dict):
        raise ValueError('Only completed recording bundles can be published')
    footer = 'run/record-complete.json' if record['schema'] == 'borg-rs-gui-recording@1' else 'run/native/record-complete.json'
    if not {footer, 'frames.jsonl.gz', 'runtime/game/angband'} <= hashes.keys():
        raise ValueError('Recording lacks complete streams, frames or build identity')
    excluded = {}
    included = {}
    for name, digest in sorted(hashes.items()):
        path = safe_path(bundle, name)
        if sha(path) != digest:
            raise ValueError('Recording file changed: ' + name)
        (excluded if binary(path, name) else included)[name] = digest
    identifier = re.sub(r'[^A-Za-z0-9_.-]', '-', bundle.name) + '-' + manifest_hash[:12]
    destination.mkdir(parents=True, exist_ok=True)
    archive_path = destination / (identifier + '.zip')
    index_path = destination / (identifier + '.json')
    info = {'schema': 'borg-cemetery-recording-export@1', 'id': identifier,
            'recording_schema': record['schema'], 'character': record.get('character'),
            'status': record['status'], 'source_bundle': str(bundle),
            'recording_manifest_sha256': manifest_hash, 'build_id': record.get('build_id'),
            'excluded_binaries': excluded,
            'qualification': 'Recorded session; export does not claim an independent verified replay.',
            'restore': 'Use cemetery_publish.py restore ARCHIVE --output DIR; matching binaries are required for verified replay.'}
    fd, name = tempfile.mkstemp(prefix='.export-', suffix='.tmp', dir=destination)
    os.close(fd)
    temporary = Path(name)
    try:
        with zipfile.ZipFile(temporary, 'w', compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
            member(archive, 'session/recording.json', record_path, manifest_hash)
            for path, digest in included.items():
                member(archive, 'session/' + path, bundle / path, digest)
            archive.writestr(zipfile.ZipInfo('export.json', (1980, 1, 1, 0, 0, 0)), json.dumps(info, sort_keys=True, indent=2) + '\n')
        if temporary.stat().st_size > MAX_ARCHIVE:
            raise ValueError('Export exceeds the 95 MiB Git limit; recording retained locally. Publish a cemetery release asset separately.')
        digest = sha(temporary)
        if archive_path.exists():
            if sha(archive_path) != digest:
                raise ValueError('An existing cemetery export differs; it is not overwritten')
        else:
            temporary.replace(archive_path)
        info.update(archive=archive_path.name, archive_sha256=digest, archive_bytes=archive_path.stat().st_size)
        encoded = json.dumps(info, sort_keys=True, indent=2) + '\n'
        if index_path.exists() and index_path.read_text() != encoded:
            raise ValueError('An existing cemetery index differs; it is not overwritten')
        index_path.write_text(encoded)
        return archive_path, index_path
    finally:
        temporary.unlink(missing_ok=True)


def related_deaths(root, bundle):
    selected = []
    for path in (root / 'deaths').glob('D*/record.json'):
        record = json.loads(path.read_text())
        directories = [a.get('raw_directory') for a in record.get('attempts', [])]
        directories.append((record.get('recording') or {}).get('raw_directory'))
        if any(d and Path(d).resolve() == bundle for d in directories):
            members = record.get('files_sha256', {})
            for name, digest in members.items():
                candidate = safe_path(path.parent, name)
                if sha(candidate) != digest:
                    raise ValueError('Death dossier evidence changed: ' + name)
                selected.append(candidate)
            selected.extend(path.parent / name for name in ('record.json', 'assessment.json', 'report.md') if (path.parent / name).is_file())
    return selected


@contextmanager
def publish_lock(root):
    common = git(root, 'rev-parse', '--path-format=absolute', '--git-common-dir').stdout.strip()
    with (Path(common) / 'recorder-publish.lock').open('a') as held:
        fcntl.flock(held, fcntl.LOCK_EX)
        yield


def publish(bundle, cemetery):
    bundle = Path(bundle).resolve()
    root, _ = root_info(cemetery)
    with publish_lock(root):
        root, branch = root_info(root)
        deaths = related_deaths(root, bundle)
        selected_cases = {p.parent.name for p in deaths}
        # The registry index can only be committed when it describes no other
        # unpublished case. Concurrent/unrelated edits are preserved and refused.
        for path in (root / 'deaths').glob('D*/record.json'):
            name = str(path.relative_to(root))
            tracked = git(root, 'ls-files', '--error-unmatch', '--', name, check=False).returncode == 0
            if not tracked and path.parent.name not in selected_cases:
                raise ValueError('Another unpublished death dossier exists; publish it separately first')
        permitted = {str(p.relative_to(root)) for p in deaths}
        if deaths:
            permitted.update('deaths/' + n for n in ('README.md', 'index.json'))
        changed = set(git(root, 'diff', '--name-only', '-z').stdout.rstrip('\0').split('\0')) - {''}
        if not changed <= permitted:
            raise ValueError('Unrelated cemetery edits exist; nothing is staged or committed')
        ignore = root / '.gitignore'
        if ignore.exists() and git(root, 'ls-files', '--error-unmatch', '--', '.gitignore', check=False).returncode:
            raise ValueError('Commit the existing untracked .gitignore before publishing')
        archive, index = build_export(bundle, root / 'recordings')
        existed_ignore = ignore.exists()
        text = ignore.read_text() if existed_ignore else ''
        missing = [line for line in LOCAL_IGNORES if line not in text.splitlines()]
        if missing:
            ignore.write_text(text.rstrip() + '\n\n# Local recorded-play state; publish only session archives.\n' + '\n'.join(missing) + '\n')
        paths = [str(p.relative_to(root)) for p in (archive, index, ignore, *deaths)]
        if deaths:
            paths += ['deaths/' + n for n in ('README.md', 'index.json') if (root / 'deaths' / n).exists()]
        for name in paths:
            if safe_path(root, name).is_symlink():
                raise ValueError('Cemetery export paths cannot be symlinks')
        git(root, 'add', '--', *paths)
        staged = git(root, 'diff', '--cached', '--name-only', '-z').stdout.rstrip('\0').split('\0')
        if set(staged) - {''} - set(paths):
            raise ValueError('Other staged changes appeared; commit refused')
        if any(staged):
            try:
                git(root, 'commit', '-m', 'Record session ' + index.stem)
            except ValueError:
                git(root, 'restore', '--staged', '--', *paths)
                if missing:
                    generated = text.rstrip() + '\n\n# Local recorded-play state; publish only session archives.\n' + '\n'.join(missing) + '\n'
                    if ignore.read_text() == generated:
                        if existed_ignore:
                            ignore.write_text(text)
                        else:
                            ignore.unlink()
                raise
        commit = git(root, 'rev-parse', 'HEAD').stdout.strip()
        try:
            git(root, 'push', 'origin', 'HEAD:refs/heads/' + branch)
        except ValueError as error:
            raise ValueError(f'{error}. Retry: git -C {root} push origin HEAD:refs/heads/{branch}') from error
        print(f'Published {archive.name} to cemetery at {commit}', flush=True)
        return commit


def restore(archive, output, executable=None):
    archive, output = Path(archive).resolve(), Path(output).resolve()
    with zipfile.ZipFile(archive) as source:
        info = json.loads(source.read('export.json'))
        if info.get('schema') != 'borg-cemetery-recording-export@1':
            raise ValueError('Unsupported cemetery archive')
        members = source.infolist()
        if len({m.filename for m in members}) != len(members) or sum(m.file_size for m in members) > 2 * 1024**3:
            raise ValueError('Archive has duplicate members or exceeds the 2 GiB extraction bound')
        if output.exists():
            raise ValueError('Restore output must not already exist')
        binaries = {}
        for name, digest in info['excluded_binaries'].items():
            candidates = [Path(executable)] if executable and name == 'runtime/game/angband' else []
            candidates.append(Path(info['source_bundle']) / name)
            original = (info.get('build_id') or {}).get('binary')
            if original and name == 'runtime/game/angband':
                candidates.append(Path(original))
            match = next((p for p in candidates if p.is_file() and sha(p) == digest), None)
            if match is None:
                raise ValueError('Matching executable required for ' + name + '; use --binary PATH. The archive contains no game binaries.')
            binaries[name] = match
        output.mkdir(parents=True)
        for item in members:
            if item.filename == 'export.json':
                continue
            if not item.filename.startswith('session/') or stat.S_ISLNK(item.external_attr >> 16):
                raise ValueError('Unsafe archive member')
            name = item.filename.removeprefix('session/')
            target = safe_path(output, name)
            target.parent.mkdir(parents=True, exist_ok=True)
            with source.open(item) as inp, target.open('xb') as out:
                shutil.copyfileobj(inp, out)
            target.chmod((item.external_attr >> 16) & 0o777)
        for name, original in binaries.items():
            target = safe_path(output, name)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(original, target)
        record = json.loads((output / 'recording.json').read_text())
        if sha(output / 'recording.json') != info['recording_manifest_sha256']:
            raise ValueError('Restored recording manifest differs')
        for name, digest in record['files_sha256'].items():
            if sha(safe_path(output, name)) != digest:
                raise ValueError('Restored recording file differs: ' + name)
    print('Restored recording:', output)
    return output


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='action', required=True)
    pub = sub.add_parser('publish')
    pub.add_argument('bundle', type=Path)
    pub.add_argument('--cemetery', type=Path, default=None)
    res = sub.add_parser('restore')
    res.add_argument('archive', type=Path)
    res.add_argument('--output', type=Path, required=True)
    res.add_argument('--binary', type=Path)
    args = p.parse_args()
    try:
        if args.action == 'publish':
            publish(args.bundle, args.cemetery or data_root_for())
        else:
            restore(args.archive, args.output, args.binary)
    except (OSError, ValueError, KeyError, zipfile.BadZipFile) as error:
        p.exit(1, str(error) + '\n')


if __name__ == '__main__':
    main()
