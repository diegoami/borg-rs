"""Real archived streams, disposable copies: missing logs and forged PASS controls."""
import hashlib
import json
import lzma
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'harness/coverage'))
from validate import validate_group, values


class MissingLogs(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='borg-oracle-')
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.output = self.base / 'output'
        self.reference = self.base / 'reference'
        shutil.copytree(ROOT / 'docs/validation/2026-11-07-synthetic-tactics/rust', self.output)
        shutil.copytree(ROOT / 'harness/reference/synthetic-tactics', self.reference)
        self.case = 'synth-glyph-ready'

    def compare(self, legacy=False):
        return subprocess.run([str(ROOT / 'harness/diff'), '--suite', 'synthetic-tactics',
                               '--save-root', str(ROOT / 'harness/saves/synthetic-tactics'),
                               '--expect-impl', 'rust',
                               *(['--allow-legacy-logs'] if legacy else []),
                               str(self.output), str(self.reference)], capture_output=True, text=True)

    def test_compressed_complete_output(self):
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertNotIn('NOTE:', result.stdout)

    def test_each_missing_output_stream_fails(self):
        for filename in ('messages.log.xz', 'state.log.xz'):
            with self.subTest(filename=filename):
                path = self.output / self.case / filename
                data = path.read_bytes()
                path.unlink()
                result = self.compare(legacy=True)
                self.assertEqual(result.returncode, 1, result.stdout)
                self.assertIn('missing required', result.stdout)
                path.write_bytes(data)

    def test_failed_marker_cannot_be_hidden_by_done(self):
        (self.output / 'FAILED').write_text('failed outcome\n')
        result = self.compare()
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn('marked FAILED', result.stdout)

    def test_both_operands_missing_streams_fail(self):
        for directory in (self.output, self.reference):
            for filename in ('messages.log.xz', 'state.log.xz'):
                (directory / self.case / filename).unlink()
        self.assertEqual(self.compare().returncode, 1)
        self.assertEqual(self.compare(legacy=True).returncode, 1)

    def test_legacy_reference_requires_explicit_option(self):
        (self.reference / 'manifest').unlink()
        for filename in ('messages.log.xz', 'state.log.xz'):
            (self.reference / self.case / filename).unlink()
        self.assertEqual(self.compare().returncode, 1)
        result = self.compare(legacy=True)
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn('NOTE:', result.stdout)


class CoverageOracle(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='borg-coverage-oracle-')
        self.addCleanup(self.tmp.cleanup)
        self.matrix = Path(self.tmp.name)
        saved = ROOT / 'docs/validation/2026-11-07-coverage'
        self.build = values(saved / 'BUILD_ID')
        shutil.copytree(saved / 'smoke', self.matrix / 'smoke')
        self.references = self.matrix / 'references'
        shutil.copytree(ROOT / 'harness/reference/smoke', self.references / 'smoke')
        entries = json.loads((self.matrix / 'smoke/profiles.json').read_text())
        profiles = self.matrix / 'profiles/smoke'
        profiles.mkdir(parents=True)
        with tarfile.open(saved / 'profiles.tar.xz') as archive:
            for name, item in entries.items():
                filename = Path(item['profile']).name
                target = profiles / filename
                target.write_bytes(archive.extractfile('smoke/' + filename).read())
                self.assertEqual(hashlib.sha256(target.read_bytes()).hexdigest(), item['sha256'])
                item['profile'] = str(target)
                for stream in ('keys.log.xz', 'messages.log.xz', 'state.log.xz'):
                    shutil.copy2(self.references / 'smoke' / name / stream, self.matrix / 'smoke' / name / stream)
        (self.matrix / 'smoke/profiles.json').write_text(json.dumps(entries))
        self.case = next(iter(entries))
        (self.matrix / 'smoke.diff').write_text('3/3 runs identical\nmessages.log compared in 3 run(s)\nstate.log compared in 3 run(s)\n')

    def validate(self):
        return validate_group(self.matrix, 'smoke', self.build, self.references)

    def test_actual_oracle_passes_without_saved_pass_text(self):
        (self.matrix / 'smoke.diff').unlink()
        self.assertEqual(len(self.validate()), 3)

    def test_forged_pass_cannot_hide_corrupt_messages(self):
        path = self.matrix / 'smoke' / self.case / 'messages.log.xz'
        with lzma.open(path, 'rb') as source:
            data = source.read()
        with lzma.open(path, 'wb') as target:
            target.write(data + b'corrupted message\n')
        with self.assertRaisesRegex(AssertionError, 'oracle comparison failed'):
            self.validate()

    def test_forged_pass_cannot_hide_missing_state(self):
        (self.matrix / 'smoke' / self.case / 'state.log.xz').unlink()
        with self.assertRaisesRegex(AssertionError, 'oracle comparison failed'):
            self.validate()

    def test_wrong_selected_reference_fails(self):
        path = self.references / 'smoke' / self.case / 'messages.log.xz'
        with lzma.open(path, 'wb') as target:
            target.write(b'wrong oracle\n')
        with self.assertRaisesRegex(AssertionError, 'oracle comparison failed'):
            self.validate()


if __name__ == '__main__':
    unittest.main()
