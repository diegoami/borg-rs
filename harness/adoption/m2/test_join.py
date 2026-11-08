"""Join controls on disposable views of four real captured game cases.

Not new game executions. Mutations never touch the original run directories.
Set BORG_M2_TEST_DIR to the validation matrix's root.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

HERE=Path(__file__).resolve().parent


@unittest.skipUnless(os.environ.get('BORG_M2_TEST_DIR'),'set BORG_M2_TEST_DIR to real case captures')
class JoinControls(unittest.TestCase):
    def setUp(self):
        self.base=Path(os.environ['BORG_M2_TEST_DIR']).resolve()
        self.tmp=tempfile.TemporaryDirectory(prefix='m2-join-control-');self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name)
        plan=json.loads((self.base/'input-plan.json').read_text())
        plan['cases']=[*plan['cases'][:3],next(r for r in plan['cases'] if r['line'].startswith('end-depth50 '))]
        self.plan=self.root/'plan.json';self.plan.write_text(json.dumps(plan))
        self.names=[r['line'].split()[0] for r in plan['cases']]
        self.groups={}
        for role,name in [('c1','games-c1'),('c2','games-c2'),('rust','games-rust-final')]:
            source=self.base/name
            if not source.exists():source=next(self.base.glob(name+'.partial.*'))
            target=self.root/role;target.mkdir()
            text=(source/'manifest').read_text()
            if 'binary_sha256_end=' not in text:
                digest=[r.split('=',1)[1] for r in text.splitlines() if r.startswith('binary_sha256=')][0]
                text+='\nbinary_sha256_end='+digest+'\n'
            (target/'manifest').write_text(text+'control_fixture=disposable-view-of-four-real-completed-cases\n')
            (target/'DONE').write_text('Synthetic control view, not an additional execution\n')
            for case in self.names:shutil.copytree(source/case,target/case)
            self.groups[role]=target

    def run_join(self,c2=None):
        output=self.root/'joined'
        result=subprocess.run(['python3',str(HERE/'compare.py'),'--plan',str(self.plan),'--save-root',str(self.base/'fixtures'),
            '--c1',str(self.groups['c1']),'--c2',str(c2 or self.groups['c2']),'--rust',str(self.groups['rust']),
            '--output',str(output)],capture_output=True,text=True)
        return result,output

    def rejected(self,c2=None):
        result,output=self.run_join(c2)
        self.assertNotEqual(result.returncode,0,result.stdout)
        self.assertFalse(json.loads((output/'verification.json').read_text())['verified'])
        self.assertFalse(any((output/r/'DONE').exists() for r in self.groups))

    def test_actual_captures_compare(self):
        result,output=self.run_join();self.assertEqual(result.returncode,0,result.stdout+result.stderr)
        self.assertEqual(json.loads((output/'verification.json').read_text())['compared_cases'],4)

    def test_mutated_frozen_input_save_rejected_after_suite_rename(self):
        with (self.groups['rust']/'end-depth50/save/input.save').open('ab') as out:out.write(b'changed')
        self.rejected()

    def test_missing_message_stream_rejected(self):
        (self.groups['rust']/self.names[0]/'messages.log').unlink();self.rejected()

    def test_changed_state_stream_rejected(self):
        with (self.groups['rust']/self.names[0]/'state.log').open('ab') as out:out.write(b'changed\n')
        self.rejected()

    def test_changed_full_save_rejected(self):
        with (self.groups['rust']/self.names[0]/'final.save').open('ab') as out:out.write(b'changed\n')
        self.rejected()

    def test_failed_process_cannot_pass(self):
        path=self.groups['rust']/self.names[0]/'process.json';value=json.loads(path.read_text());value['status']='timeout';path.write_text(json.dumps(value));self.rejected()

    def test_altered_budget_rejected(self):
        path=self.groups['rust']/self.names[0]/'process.json';value=json.loads(path.read_text());value['timeout_seconds']=301;path.write_text(json.dumps(value));self.rejected()

    def test_reusing_one_c_execution_rejected(self):
        self.rejected(c2=self.groups['c1'])


if __name__=='__main__':unittest.main()
