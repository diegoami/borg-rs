#!/usr/bin/env python3
"""Bounded private-Xvfb GUI recording controls; never touch owner profiles/display."""
import importlib.util
import inspect
import json
import os
from pathlib import Path
import selectors
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools'))
spec = importlib.util.spec_from_file_location('existing_gui_controls', ROOT / 'harness/regress/gui-replay/run.py')
gui_controls = importlib.util.module_from_spec(spec); spec.loader.exec_module(gui_controls)


def main():
    out = Path(sys.argv[1]).resolve(); out.mkdir(parents=True, exist_ok=False)
    log = (out / 'xvfb.log').open('w')
    number = str(200 + os.getpid() % 500)
    x = subprocess.Popen(['Xvfb', ':' + number, '-displayfd', '1', '-screen', '0', '5200x1200x24', '-nolisten', 'unix', '-listen', 'tcp', '-ac'], stdout=subprocess.PIPE, stderr=log, text=True)
    try:
        selector = selectors.DefaultSelector(); selector.register(x.stdout, selectors.EVENT_READ)
        if not selector.select(10):
            raise ValueError('Private Xvfb failed to allocate a display')
        actual = x.stdout.readline().strip()
        if actual != number:
            raise ValueError('Private Xvfb failed to bind requested display')
        display = '127.0.0.1:' + number
        env = {**os.environ, 'DISPLAY': display, 'BORG_DEATH_RECORDS_DIR': str(out / 'deaths'), 'BORG_PLAY_FONT': 'fixed'}
        profile = out / 'profile'
        results = []
        results.append(gui_controls.stage(ROOT, out, env, profile, 'fresh-save', character='ToolkitGui',
                                           options=('--seed', '42'), fresh=True, resize=True))
        results.append(gui_controls.stage(ROOT, out, env, profile, 'loaded-save', character='ToolkitGui'))
        # A real >255-column X11 window, not a synthetic frame fixture.
        wide_source = inspect.getsource(gui_controls.stage).replace("'640', '420'", "'4800', '700'")
        wide_namespace = dict(gui_controls.__dict__)
        exec(compile(wide_source, str(Path(__file__)), 'exec'), wide_namespace)
        results.append(wide_namespace['stage'](ROOT, out, env, profile, 'wide-window', character='ToolkitGui', resize=True))
        # Give the private window focus before the Borg-abort/manual keys.
        # Retain the inherited wait-boundary and full replay assertions.
        source = inspect.getsource(gui_controls.stage)
        marker = '            def key(value):'
        assert source.count(marker) == 1
        source = source.replace(marker, "            subprocess.run(['xdotool','windowfocus',window],env=env,check=True)\n\n" + marker)
        source = source.replace("window, value], env=env", "window, *([value, 'Escape'] if value == 'z' else [value])], env=env")
        namespace = dict(gui_controls.__dict__)
        exec(compile(source, str(Path(__file__)), 'exec'), namespace)
        results.append(namespace['stage'](ROOT, out, env, profile, 'loaded-borg-immediate-stop-save', character='ToolkitGui', borg=True))
        # Explicit invalid fonts must fail before any historical prefix/helper.
        invalid = subprocess.run([str(ROOT/'scripts/play-from-replay.py'),results[0]['bundle'],'--from-turn','latest','--output',str(out/'bad-font-branch')],
            env={**env,'BORG_PLAY_FONT':'borg-toolkit-font-that-does-not-exist'},capture_output=True,text=True)
        assert invalid.returncode != 0 and not (out/'bad-font-branch').exists()
        assert 'font' in invalid.stderr.lower() and 'Building historical' not in invalid.stdout
        (out/'invalid-font.json').write_text(json.dumps({'returncode':invalid.returncode,'stderr':invalid.stderr,'no_prefix_or_build':True},indent=2)+'\n')
        (out / 'results.json').write_text(json.dumps(results, indent=2)+'\n')
        print('Private display:', display, 'results:', out / 'results.json', flush=True)
    finally:
        x.terminate(); x.wait(timeout=10); log.close()


if __name__ == '__main__':
    main()
