"""Real Git/ZIP controls for opt-in cemetery publication; no network services."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock
import zipfile

import cemetery_publish as pub
import recording_paths


def command(*args):
    return subprocess.run(list(map(str, args)), check=True, capture_output=True, text=True).stdout.strip()


class Publication(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='cemetery-publish-')
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.remote = self.base / 'remote.git'
        self.root = self.base / 'cemetery'
        command('git', 'init', '--bare', '-q', self.remote)
        command('git', 'init', '-q', '-b', 'main', self.root)
        command('git', '-C', self.root, 'config', 'user.name', 'Recorder Test')
        command('git', '-C', self.root, 'config', 'user.email', 'recorder@example.invalid')
        (self.root / 'README.md').write_text('original cemetery\n')
        command('git', '-C', self.root, 'add', 'README.md')
        command('git', '-C', self.root, 'commit', '-qm', 'Initial cemetery')
        command('git', '-C', self.root, 'remote', 'add', 'origin', self.remote)
        command('git', '-C', self.root, 'push', '-qu', 'origin', 'main')
        self.bundle = self.root / 'recordings/20260101-Test'
        self.bundle.mkdir(parents=True)
        files = {'runtime/game/angband': b'\x7fELFnot an executed test binary',
                 'runtime/game/lib/gamedata/test.txt': b'game data',
                 'runtime/harness/run.sh': b'#!/bin/sh\nexit 0\n',
                 'run/events.txt': b'poll\n', 'run/record-complete.json': b'{"inputs":1,"frames":1}\n',
                 'run/frames.jsonl': b'{"hp":20}\n', 'frames.jsonl.gz': b'compressed-frame fixture',
                 'final.save': b'saved character'}
        hashes = {}
        for name, data in files.items():
            path = self.bundle / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            hashes[name] = hashlib.sha256(data).hexdigest()
        (self.bundle / 'runtime/game/angband').chmod(0o500)
        (self.bundle / 'runtime/harness/run.sh').chmod(0o755)
        record = {'schema':'borg-rs-gui-recording@1','status':'complete','character':'Test',
                  'build_id':{'git_rev':'test-source','binary':str(self.bundle / 'runtime/game/angband')},
                  'files_sha256':hashes}
        (self.bundle / 'recording.json').write_text(json.dumps(record))

    def exports(self):
        return list((self.root / 'recordings').glob('*.zip'))

    def test_export_commits_only_archive_metadata_and_ignore_and_restores_exact_bytes(self):
        (self.root / 'save').mkdir()
        (self.root / 'save/live.save').write_text('must stay local')
        (self.root / 'private-note').write_text('must not be committed')
        pub.preflight(self.root)
        commit = pub.publish(self.bundle, self.root)
        self.assertEqual(command('git', '--git-dir', self.remote, 'rev-parse', 'refs/heads/main'), commit)
        paths = command('git', '-C', self.root, 'ls-tree', '-r', '--name-only', 'HEAD').splitlines()
        self.assertEqual(len(paths), 4)  # README, ignore, one archive, one index
        self.assertNotIn('private-note', paths)
        self.assertFalse(any(p.startswith('save/') or '/runtime/' in p for p in paths))
        archive = self.exports()[0]
        with zipfile.ZipFile(archive) as source:
            self.assertNotIn('session/runtime/game/angband', source.namelist())
            self.assertIn('session/run/events.txt', source.namelist())
        restored = pub.restore(archive, self.base / 'restored')
        for path in self.bundle.rglob('*'):
            if path.is_file():
                self.assertEqual(path.read_bytes(), (restored / path.relative_to(self.bundle)).read_bytes())
        self.assertTrue((restored / 'runtime/harness/run.sh').stat().st_mode & 0o111)
        self.assertEqual(pub.publish(self.bundle, self.root), commit)  # retry is idempotent

    def test_dirty_or_staged_owner_work_is_never_committed(self):
        (self.root / 'README.md').write_text('owner edit')
        with self.assertRaisesRegex(ValueError, 'Unrelated'):
            pub.publish(self.bundle, self.root)
        self.assertEqual(command('git', '-C', self.root, 'diff', '--cached', '--name-only'), '')
        command('git', '-C', self.root, 'add', 'README.md')
        with self.assertRaisesRegex(ValueError, 'staged'):
            pub.publish(self.bundle, self.root)
        self.assertEqual(command('git', '-C', self.root, 'diff', '--cached', '--name-only'), 'README.md')
        self.assertFalse(self.exports())

    def test_unpushed_owner_commit_cannot_be_sent_by_auto_push(self):
        (self.root / 'README.md').write_text('private committed edit')
        command('git', '-C', self.root, 'add', 'README.md')
        command('git', '-C', self.root, 'commit', '-qm', 'Private work')
        with self.assertRaisesRegex(ValueError, 'unpushed commits'):
            pub.publish(self.bundle, self.root)

    def test_push_failure_retains_the_commit_and_original_recording(self):
        command('git', '-C', self.root, 'remote', 'set-url', 'origin', self.base / 'missing.git')
        before = command('git', '-C', self.root, 'rev-parse', 'HEAD')
        with self.assertRaisesRegex(ValueError, 'local recording/commit retained'):
            pub.publish(self.bundle, self.root)
        self.assertNotEqual(command('git', '-C', self.root, 'rev-parse', 'HEAD'), before)
        self.assertTrue((self.bundle / 'runtime/game/angband').exists())
        self.assertEqual(command('git', '-C', self.root, 'diff', '--cached', '--name-only'), '')

    def test_rejected_commit_restores_staging_and_ignore_but_keeps_export(self):
        hook = self.root / '.git/hooks/pre-commit'
        hook.write_text('#!/bin/sh\nexit 1\n')
        hook.chmod(0o755)
        with self.assertRaisesRegex(ValueError, 'Git commit failed'):
            pub.publish(self.bundle, self.root)
        self.assertEqual(command('git', '-C', self.root, 'diff', '--cached', '--name-only'), '')
        self.assertFalse((self.root / '.gitignore').exists())
        self.assertTrue(self.exports())

    def test_incomplete_tampered_or_oversize_recording_is_not_committed(self):
        path = self.bundle / 'recording.json'
        original = path.read_text()
        record = json.loads(original)
        record['status'] = 'incomplete'
        path.write_text(json.dumps(record))
        with self.assertRaisesRegex(ValueError, 'completed'):
            pub.publish(self.bundle, self.root)
        path.write_text(original)
        with mock.patch.object(pub, 'MAX_ARCHIVE', 8), self.assertRaisesRegex(ValueError, '95 MiB'):
            pub.publish(self.bundle, self.root)
        self.assertFalse(self.exports())
        (self.bundle / 'run/events.txt').write_text('tampered')
        with self.assertRaisesRegex(ValueError, 'changed'):
            pub.publish(self.bundle, self.root)

    def test_only_the_associated_death_dossier_is_selected(self):
        death = self.root / 'deaths/D0001-recorded'
        death.mkdir(parents=True)
        (death / 'record.json').write_text(json.dumps({'attempts':[{'raw_directory':str(self.bundle)}], 'files_sha256':{}}))
        (death / 'assessment.json').write_text('{}')
        (death / 'report.md').write_text('curated case')
        (death / 'owner-note').write_text('not part of the dossier export')
        (self.root / 'deaths/README.md').write_text('index')
        (self.root / 'deaths/index.json').write_text('{}')
        pub.publish(self.bundle, self.root)
        names = command('git', '-C', self.root, 'ls-tree', '-r', '--name-only', 'HEAD').splitlines()
        self.assertIn('deaths/D0001-recorded/record.json', names)
        self.assertNotIn('deaths/D0001-recorded/owner-note', names)

    def test_restore_rejects_wrong_binary_and_escaping_members(self):
        archive, _ = pub.build_export(self.bundle, self.root / 'recordings')
        (self.bundle / 'runtime/game/angband').unlink()
        with self.assertRaisesRegex(ValueError, 'Matching executable'):
            pub.restore(archive, self.base / 'no-binary')
        self.assertFalse((self.base / 'no-binary').exists())
        bad = self.base / 'bad.zip'
        with zipfile.ZipFile(archive) as src, zipfile.ZipFile(bad, 'w') as dst:
            for name in src.namelist():
                if name == 'export.json':
                    info = json.loads(src.read(name)); info['excluded_binaries'] = {}
                    dst.writestr(name, json.dumps(info))
                else:
                    dst.writestr(name, src.read(name))
            dst.writestr('session/../../escaped', 'no')
        with self.assertRaisesRegex(ValueError, 'Unsafe'):
            pub.restore(bad, self.base / 'unsafe')
        self.assertFalse((self.base / 'escaped').exists())


class DefaultPaths(unittest.TestCase):
    def test_linked_worktree_resolves_the_sibling_cemetery_and_env_override(self):
        with tempfile.TemporaryDirectory() as folder, mock.patch.dict(os.environ, {}, clear=True):
            root = Path(folder) / 'borg-rs'; root.mkdir()
            command('git', 'init', '-q', '-b', 'main', root)
            command('git', '-C', root, 'config', 'user.name', 'Test')
            command('git', '-C', root, 'config', 'user.email', 'test@example.invalid')
            (root / 'file').touch(); command('git', '-C', root, 'add', 'file')
            command('git', '-C', root, 'commit', '-qm', 'Initial')
            tree = root / '.worktrees/task'
            command('git', '-C', root, 'worktree', 'add', '-q', '-b', 'task', tree)
            self.assertEqual(recording_paths.data_root_for(tree), Path(folder) / 'player-cemetery')
            with mock.patch.dict(os.environ, {'BORG_PLAY_DATA_DIR':str(Path(folder) / 'private')}):
                self.assertEqual(recording_paths.data_root_for(tree), Path(folder) / 'private')

class BatchControls(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import importlib.util
        path = Path(__file__).resolve().parents[1] / 'record-random-runs.py'
        spec = importlib.util.spec_from_file_location('random_recordings_test', path)
        cls.batch = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.batch)

    def test_bounded_fresh_random_births_and_opt_in_publication(self):
        returned = [Path('/tmp/synthetic-recording-' + str(i)) for i in range(3)]
        with mock.patch.object(self.batch.runtime, 'data_root', return_value=Path('/tmp/synthetic-cemetery')), \
             mock.patch.object(self.batch.runtime, 'record_run', side_effect=returned) as record, \
             mock.patch.object(self.batch.cemetery_publish, 'preflight') as preflight, \
             mock.patch.object(self.batch.cemetery_publish, 'publish') as publish:
            self.assertEqual(self.batch.main(['--runs','3','--seed','4294967295','--turns','100','--auto-push']), 0)
            self.assertEqual([c.args[0].seed for c in record.call_args_list], [4294967295,0,1])
            self.assertEqual(len({c.args[0].character for c in record.call_args_list}),3)
            self.assertTrue(all(c.args[0].new and c.args[0].race is None and c.args[0].cls is None for c in record.call_args_list))
            self.assertEqual(publish.call_args_list, [mock.call(p,Path('/tmp/synthetic-cemetery')) for p in returned])
            preflight.assert_called_once()

    def test_publication_is_not_enabled_by_default_and_errors_stop_the_batch(self):
        with mock.patch.object(self.batch.runtime, 'record_run', side_effect=ValueError('synthetic failure')) as record, \
             mock.patch.object(self.batch.cemetery_publish, 'publish') as publish:
            with self.assertRaises(SystemExit):
                self.batch.main(['--runs','3','--turns','100'])
            record.assert_called_once()
            publish.assert_not_called()
