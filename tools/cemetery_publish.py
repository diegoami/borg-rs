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
MAX_EXPORT_JSON = 1024 * 1024
LOCAL_IGNORES = ('/save/', '/user/', '/archive/', '/panic/', '/scores/', '/recordings/*/', '/recordings/.export-*.tmp', '/deaths/.registry.lock')
SCHEMAS = {'borg-rs-gui-recording@1', 'borg-rs-headless-recording@1'}


# Output IO is anchored to no-follow directory descriptors. Validating only a
# resolved Path after a write is too late: it can already have followed a link.
@contextmanager
def directory_fd(path, create=False):
    path = Path(path).expanduser().absolute()
    descriptor = os.open(path.anchor, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        for part in path.parts[1:]:
            if create:
                try:
                    os.mkdir(part, dir_fd=descriptor)
                except FileExistsError:
                    pass
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
            os.close(descriptor)
            descriptor = child
        yield descriptor
    except OSError as error:
        raise ValueError('Cemetery output directories must not be symlinks or special files') from error
    finally:
        os.close(descriptor)


def read_at(directory, name, missing=False):
    try:
        descriptor = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory)
    except FileNotFoundError:
        if missing:
            return None
        raise
    except OSError as error:
        raise ValueError('Cemetery output is not a regular file: ' + name) from error
    with os.fdopen(descriptor, 'rb') as stream:
        if not stat.S_ISREG(os.fstat(stream.fileno()).st_mode):
            raise ValueError('Cemetery output is not a regular file: ' + name)
        return stream.read()


def generation_at(directory, name):
    try:
        before = os.stat(name, dir_fd=directory, follow_symlinks=False)
    except FileNotFoundError:
        return None
    data = read_at(directory, name)
    after = os.stat(name, dir_fd=directory, follow_symlinks=False)
    fields = lambda s: (s.st_dev,s.st_ino,s.st_mode,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
    if fields(before) != fields(after):
        raise ValueError('Cemetery file changed while being inspected: ' + name)
    return fields(after) + (hashlib.sha256(data).hexdigest(),)


def atomic_at(directory, name, data, replace=False, mode=0o644):
    # The generated name is private to this transaction; no existing path is
    # truncated, and a symlink cannot redirect a read/write outside the checkout.
    token = '.recorder-' + os.urandom(12).hex() + '.tmp'
    descriptor = os.open(token, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode, dir_fd=directory)
    try:
        with os.fdopen(descriptor,'wb') as stream:
            stream.write(data);stream.flush();os.fsync(stream.fileno())
        if replace:
            os.replace(token,name,src_dir_fd=directory,dst_dir_fd=directory)
        else:
            os.link(token,name,src_dir_fd=directory,dst_dir_fd=directory,follow_symlinks=False)
    finally:
        try: os.unlink(token,dir_fd=directory)
        except FileNotFoundError: pass


@contextmanager
def file_lock(directory, name):
    descriptor = os.open(name, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600, dir_fd=directory)
    with os.fdopen(descriptor,'r+b') as held:
        if not stat.S_ISREG(os.fstat(held.fileno()).st_mode):
            raise ValueError('Cemetery lock is not a regular file')
        fcntl.flock(held,fcntl.LOCK_EX)
        yield


def write_diagnostic(root, data):
    result = subprocess.run(['git','-C',str(root),'rev-parse','--absolute-git-dir'],capture_output=True,text=True)
    if result.returncode == 0:
        try:
            with directory_fd(result.stdout.strip()) as directory:
                generation_at(directory,'recorder-publish.log')
                atomic_at(directory,'recorder-publish.log',data.encode(),replace=True,mode=0o600)
        except (OSError,ValueError):
            pass  # Never follow an unsafe diagnostic path while reporting failure.


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def git(root, *args, check=True, index_file=None, input_bytes=None):
    env={**os.environ,'GIT_TERMINAL_PROMPT':'0','GIT_OPTIONAL_LOCKS':'0'}
    if index_file is not None:env['GIT_INDEX_FILE']=str(index_file)
    try:
        result=subprocess.run(['git','-C',str(root),*args],capture_output=True,
                              text=input_bytes is None,input=input_bytes,env=env,timeout=120)
    except subprocess.TimeoutExpired as error:
        raise ValueError(f'Git {args[0]} timed out; local recording/commit retained') from error
    if isinstance(result.stdout,bytes):
        result.stdout=result.stdout.decode(errors='replace');result.stderr=result.stderr.decode(errors='replace')
    if check and result.returncode:
        write_diagnostic(root,result.stdout+result.stderr)
        raise ValueError(f'Git {args[0]} failed; local recording/commit retained. See .git/recorder-publish.log')
    return result


def root_info(cemetery):
    supplied=Path(cemetery).expanduser()
    if not stat.S_ISDIR(supplied.lstat().st_mode):raise ValueError('Cemetery root must be a real directory')
    root = supplied.resolve()
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


def validate_bundle(bundle):
    import borg_gui
    import borg_runtime
    import borg_replay_viewer
    record=json.loads(safe_path(bundle,'recording.json').read_text())
    if record.get('status')!='complete':raise ValueError('Only completed recording bundles can be published')
    if record.get('schema')==borg_gui.SCHEMA:
        value=borg_gui.validate(bundle)
        frames=borg_replay_viewer.Frames(bundle);frames.close()
    elif record.get('schema')==borg_runtime.SCHEMA:
        value=borg_runtime.validate(bundle)
    else:raise ValueError('Unsupported recording schema')
    if not value.get('build_id'):
        raise ValueError('A published recording requires its actual executable identity')
    return value

def build_export(bundle, destination):
    bundle = Path(bundle).resolve()
    record_path = safe_path(bundle, 'recording.json')
    manifest_hash = sha(record_path)
    record = validate_bundle(bundle)
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
    archive_path = destination / (identifier + '.zip')
    index_path = destination / (identifier + '.json')
    info = {'schema': 'borg-cemetery-recording-export@1', 'id': identifier,
            'recording_schema': record['schema'], 'character': record.get('character'),
            'status': record['status'], 'source_bundle': str(bundle),
            'recording_manifest_sha256': manifest_hash, 'build_id': record.get('build_id'),
            'excluded_binaries': excluded,
            'qualification': 'Recorded session; export does not claim an independent verified replay.',
            'restore': 'Use cemetery_publish.py restore ARCHIVE --output DIR; matching binaries are required for verified replay.'}
    with directory_fd(destination,create=True) as directory:
        # Existing leaf entries are validated before creating even a temporary.
        generation_at(directory,archive_path.name);generation_at(directory,index_path.name)
        token='.export-'+os.urandom(12).hex()+'.tmp'
        descriptor=os.open(token,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=directory)
        try:
            with os.fdopen(descriptor,'wb') as stream:
                with zipfile.ZipFile(stream,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as archive:
                    member(archive,'session/recording.json',record_path,manifest_hash)
                    for path,digest in included.items():member(archive,'session/'+path,bundle/path,digest)
                    archive.writestr(zipfile.ZipInfo('export.json',(1980,1,1,0,0,0)),json.dumps(info,sort_keys=True,indent=2)+'\n')
                stream.flush();os.fsync(stream.fileno())
            temporary=read_at(directory,token)
            if len(temporary)>MAX_ARCHIVE:raise ValueError('Export exceeds the 95 MiB Git limit; recording retained locally. Publish a cemetery release asset separately.')
            digest=hashlib.sha256(temporary).hexdigest()
            existing=read_at(directory,archive_path.name,missing=True)
            if existing is not None:
                if hashlib.sha256(existing).hexdigest()!=digest:raise ValueError('An existing cemetery export differs; it is not overwritten')
            else:os.link(token,archive_path.name,src_dir_fd=directory,dst_dir_fd=directory,follow_symlinks=False)
            info.update(archive=archive_path.name,archive_sha256=digest,archive_bytes=len(temporary))
            encoded=(json.dumps(info,sort_keys=True,indent=2)+'\n').encode()
            existing=read_at(directory,index_path.name,missing=True)
            if existing is not None and existing!=encoded:raise ValueError('An existing cemetery index differs; it is not overwritten')
            if existing is None:atomic_at(directory,index_path.name,encoded)
            return archive_path,index_path
        finally:
            os.unlink(token,dir_fd=directory)


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
    common=git(root,'rev-parse','--path-format=absolute','--git-common-dir').stdout.strip()
    with directory_fd(common) as directory,file_lock(directory,'recorder-publish.lock'):
        yield

def _checked_files(root, names):
    result = {}
    for name in sorted(set(names)):
        parent = root / Path(name).parent
        with directory_fd(parent) as descriptor:
            generation = generation_at(descriptor,Path(name).name)
            if generation is None:
                raise ValueError('Selected cemetery file disappeared: ' + name)
            data = read_at(descriptor,Path(name).name)
            if len(data)>MAX_ARCHIVE:
                raise ValueError('Selected cemetery file exceeds the Git size bound')
            result[name] = (generation,data)
    return result


def _unchanged(root, snapshot):
    for name,(generation,_) in snapshot.items():
        with directory_fd(root / Path(name).parent) as descriptor:
            if generation_at(descriptor,Path(name).name)!=generation:
                raise ValueError('Concurrent cemetery edit; publication refused: ' + name)


def commit_selected(root, branch, snapshot):
    """A private index freezes exact blobs, never the owner's working/index state.

    Briefly take Git's normal index.lock while committing/synchronizing. User
    edits cannot enter our blobs; every failure leaves the original index intact.
    Hooks run normally on the private index. Unexpected hook staging is rejected
    before push and the owned, unpublished ref update is rolled back by CAS.
    """
    gitdir = Path(git(root,'rev-parse','--absolute-git-dir').stdout.strip())
    before = git(root,'rev-parse','HEAD').stdout.strip()
    ref = 'refs/heads/' + branch
    index_lock = gitdir / 'index.lock'
    descriptor = os.open(index_lock,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
    fd,index_name = tempfile.mkstemp(prefix='recorder-index-',dir=gitdir)
    os.close(fd);os.unlink(index_name)
    private = Path(index_name)
    committed = None
    try:
        if git(root,'diff','--cached','--name-only','-z').stdout:
            raise ValueError('Owner staging changed before publication; index preserved')
        if git(root,'rev-parse','HEAD').stdout.strip()!=before:
            raise ValueError('Cemetery HEAD changed before publication')
        _unchanged(root,snapshot)
        git(root,'read-tree',before,index_file=private)
        blobs = {}
        for name,(_,data) in snapshot.items():
            blob = git(root,'hash-object','-w','--stdin',input_bytes=data).stdout.strip()
            blobs[name] = blob
            git(root,'update-index','--add','--cacheinfo','100644,'+blob+','+name,index_file=private)
        delta = git(root,'diff','--cached','--name-only','-z',index_file=private).stdout
        if not delta:
            return before
        git(root,'commit','-m','Record session '+next(n for n in snapshot if n.endswith('.zip')).split('/')[-1],index_file=private)
        committed = git(root,'rev-parse','HEAD').stdout.strip()
        expected_tree = git(root,'rev-parse',before+'^{tree}').stdout.strip()
        # Reconstruct the intended tree after hooks; a hook cannot silently add
        # another path or replace a selected blob with a live owner edit.
        fd,verify_name=tempfile.mkstemp(prefix='recorder-verify-index-',dir=gitdir)
        os.close(fd);os.unlink(verify_name)
        verify_index=Path(verify_name)
        try:
            git(root,'read-tree',expected_tree,index_file=verify_index)
            for name,blob in blobs.items():
                git(root,'update-index','--add','--cacheinfo','100644,'+blob+','+name,index_file=verify_index)
            wanted=git(root,'write-tree',index_file=verify_index).stdout.strip()
        finally:
            verify_index.unlink(missing_ok=True)
        if git(root,'rev-parse',committed+'^{tree}').stdout.strip()!=wanted:
            raise ValueError('Commit hook changed the selected export scope; push refused')
        _unchanged(root,snapshot)
        with os.fdopen(descriptor,'wb',closefd=False) as out:
            out.write(private.read_bytes());out.flush();os.fsync(out.fileno())
        os.replace(index_lock,gitdir/'index')
        return committed
    except BaseException:
        if committed:
            git(root,'update-ref',ref,before,committed)
        raise
    finally:
        os.close(descriptor)
        index_lock.unlink(missing_ok=True)
        private.unlink(missing_ok=True)
        Path(str(private)+'.lock').unlink(missing_ok=True)


def publish(bundle, cemetery):
    bundle = Path(bundle).resolve()
    root, _ = root_info(cemetery)
    with publish_lock(root), directory_fd(root) as root_fd, directory_fd(root/'recordings',create=True) as recording_fd, directory_fd(root/'deaths',create=True) as deaths_fd, file_lock(deaths_fd,'.registry.lock'):
        root,branch=root_info(root)
        # Reject output aliases BEFORE inspecting content or making an archive.
        original_ignore=read_at(root_fd,'.gitignore',missing=True)
        ignore_generation=generation_at(root_fd,'.gitignore')
        if original_ignore is not None and git(root,'ls-files','--error-unmatch','--','.gitignore',check=False).returncode:
            raise ValueError('Commit the existing untracked .gitignore before publishing')
        deaths=related_deaths(root,bundle)
        selected_cases={p.parent.name for p in deaths}
        for path in (root/'deaths').glob('D*/record.json'):
            tracked=git(root,'ls-files','--error-unmatch','--',str(path.relative_to(root)),check=False).returncode==0
            if not tracked and path.parent.name not in selected_cases:
                raise ValueError('Another unpublished death dossier exists; review/commit pending cases explicitly first')
        permitted={str(p.relative_to(root)) for p in deaths}
        if deaths:permitted.update('deaths/'+n for n in ('README.md','index.json'))
        changed=set(git(root,'diff','--name-only','-z').stdout.rstrip('\0').split('\0'))-{''}
        if not changed<=permitted:raise ValueError('Unrelated cemetery edits exist; nothing is staged or committed')
        shared=_checked_files(root,list(permitted))
        archive,index=build_export(bundle,root/'recordings')
        if generation_at(root_fd,'.gitignore')!=ignore_generation:
            raise ValueError('Concurrent cemetery edit; publication refused: .gitignore')
        _unchanged(root,shared)
        # Also check directory identities: export IO used the anchored fd even
        # if another actor tried to swap the pathname during compression.
        if os.stat(root/'recordings',follow_symlinks=False).st_ino!=os.fstat(recording_fd).st_ino:
            raise ValueError('Recording directory changed during export')
        text=(original_ignore or b'').decode()
        missing=[line for line in LOCAL_IGNORES if line not in text.splitlines()]
        generated=original_ignore
        wrote_ignore=False
        try:
            if missing:
                generated=(text.rstrip()+'\n\n# Local recorded-play state; publish only session archives.\n'+'\n'.join(missing)+'\n').encode()
                atomic_at(root_fd,'.gitignore',generated,replace=original_ignore is not None)
                wrote_ignore=True
            names=[str(p.relative_to(root)) for p in (archive,index,*deaths)]+['.gitignore']
            if deaths:names+=['deaths/'+n for n in ('README.md','index.json') if (root/'deaths'/n).exists()]
            snapshot=_checked_files(root,names)
            current=set(git(root,'diff','--name-only','-z').stdout.rstrip('\0').split('\0'))-{''}
            if not current<=permitted|{'.gitignore'}:
                raise ValueError('Concurrent unrelated edits appeared; publication refused')
            commit=commit_selected(root,branch,snapshot)
        except BaseException:
            # Do not overwrite an editor's later change during rollback.
            if wrote_ignore and read_at(root_fd,'.gitignore',missing=True)==generated:
                if original_ignore is None:os.unlink('.gitignore',dir_fd=root_fd)
                else:atomic_at(root_fd,'.gitignore',original_ignore,replace=True)
            raise
    try:
        # Push the verified commit, not a HEAD another owner may advance later.
        git(root,'push','origin',commit+':refs/heads/'+branch)
    except ValueError as error:
        raise ValueError(f'{error}. Retry: git -C {root} push origin {commit}:refs/heads/{branch}') from error
    print(f'Published {archive.name} to cemetery at {commit}',flush=True)
    return commit



def restore(archive, output, executable=None):
    archive, output = Path(archive).resolve(), Path(output).expanduser().absolute()
    with zipfile.ZipFile(archive) as source:
        members = source.infolist()
        if len({m.filename for m in members}) != len(members) or sum(m.file_size for m in members) > 2 * 1024**3:
            raise ValueError('Archive has duplicate members or exceeds the 2 GiB extraction bound')
        metadata=next((m for m in members if m.filename=='export.json'),None)
        if metadata is None or metadata.file_size>MAX_EXPORT_JSON:
            raise ValueError('Archive export metadata exceeds its size bound or is absent')
        with source.open(metadata) as stream:data=stream.read(MAX_EXPORT_JSON+1)
        if len(data)>MAX_EXPORT_JSON:raise ValueError('Archive export metadata exceeds its size bound')
        info=json.loads(data)
        if not isinstance(info,dict) or info.get('schema')!='borg-cemetery-recording-export@1':
            raise ValueError('Unsupported cemetery archive')
        if not isinstance(info.get('excluded_binaries'),dict) or not isinstance(info.get('source_bundle'),str):
            raise ValueError('Malformed export metadata')
        for item in members:
            if item.filename=='export.json':continue
            relative=PurePosixPath(item.filename)
            if not item.filename.startswith('session/') or '..' in relative.parts or stat.S_ISLNK(item.external_attr>>16):
                raise ValueError('Unsafe archive member')
        if os.path.lexists(output):
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
        with directory_fd(output,create=True):
            pass
        for item in members:
            if item.filename == 'export.json':
                continue
            if not item.filename.startswith('session/') or stat.S_ISLNK(item.external_attr >> 16):
                raise ValueError('Unsafe archive member')
            name = item.filename.removeprefix('session/')
            target = safe_path(output, name)
            with directory_fd(target.parent,create=True) as parent:
                descriptor=os.open(target.name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=parent)
                with source.open(item) as inp, os.fdopen(descriptor,'wb') as out:
                    shutil.copyfileobj(inp,out);os.fchmod(out.fileno(),(item.external_attr>>16)&0o777)
        for name, original in binaries.items():
            target = safe_path(output, name)
            with directory_fd(target.parent,create=True) as parent:
                descriptor=os.open(target.name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o500,dir_fd=parent)
                with original.open('rb') as inp,os.fdopen(descriptor,'wb') as out:shutil.copyfileobj(inp,out)
        record = json.loads((output / 'recording.json').read_text())
        if sha(output / 'recording.json') != info['recording_manifest_sha256']:
            raise ValueError('Restored recording manifest differs')
        for name, digest in record['files_sha256'].items():
            if sha(safe_path(output, name)) != digest:
                raise ValueError('Restored recording file differs: ' + name)
    # Empty profile directories are producer state too, although Git/file-hash
    # manifests contain files only. Recreate their fixed GUI layout for replay.
    if record.get('schema')=='borg-rs-gui-recording@1':
        for name in ['user','save','panic','archive','scores']:
            with directory_fd(output/'initial-profile'/name,create=True):pass
    validate_bundle(output)
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
