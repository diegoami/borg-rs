#!/usr/bin/env python3
"""Build-only observation adapter. Never edits the checked-in Rust Borg.

Fail closed if the two instrumentation sites change. Copy all other bytes
unchanged; avoid touching unchanged files so Cargo can reuse its artifacts.
"""
from pathlib import Path
import sys

root, out = map(Path, sys.argv[1:])
files = [root / 'Cargo.toml', root / 'Cargo.lock']
files += sorted(p for p in (root / 'borg-rs').rglob('*')
                if p.is_file() and 'target' not in p.relative_to(root).parts)
files += sorted(p for p in (root / 'vendor/angband-recorder').rglob('*') if p.is_file() and '__pycache__' not in p.parts and 'target' not in p.parts)
files += [root / 'recorder-pin.json']
for source in files:
    relative = source.relative_to(root)
    data = source.read_bytes()
    if relative == Path('borg-rs/src/borg.rs'):
        old = b'while !crate::borg_think::borg_think(st) {}'
        new = b'''while {
        engine::borgh_measure(st, true);
        !crate::borg_think::borg_think(st)
    } {}'''
        assert data.count(old) == 1, 'Rust decision site changed'
        data = data.replace(old, new)
        old = b'st.core.rand_local = engine::rand_state().1;'
        assert data.count(old) == 1, 'Rust RNG site changed'
        data = data.replace(old, old + b'\n    engine::borgh_measure(st, false);')
    if relative == Path('borg-rs/src/engine.rs'):
        data += b'''
// Build-only BORG_HARNESS observation bridge. C copies values synchronously;
// no references to BorgState cross the call and no callback reenters Rust.
pub(crate) fn borgh_measure(st: &BorgState, decision: bool) {
    unsafe extern "C" {
        fn borgh_setting(index: i32, name: *const libc::c_char, value: i32);
        fn borgh_decision(rng: u32);
        fn borgh_rng(rng: u32);
    }
    unsafe {
        for (index, setting) in crate::borg_init::BORG_SETTINGS.iter().enumerate() {
            borgh_setting(index as i32, setting.name.as_ptr(), st.core.cfg[index]);
        }
        if decision { borgh_decision(st.core.rand_local); }
        else { borgh_rng(st.core.rand_local); }
    }
}
'''
    target = out / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    if not target.exists() or target.read_bytes() != data:
        target.write_bytes(data)
