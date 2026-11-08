# Carry C milestone4 fixes into C oracle and Rust

Owner direction (2026-11-08): with `4.2.6-impact-v3` tagged, proceed to C
milestone4. Tracking [Rust#11](https://github.com/diegoami/borg-rs/issues/11);
record [adoption/c-m4](../../adoption/c-m4/README.md). Target better-borg-c
`70993393a1c946ce8cd38ee558bf1ce6464350e7` (`4.2.6-impact-v4`). Start: Rust
main `3211e91` (`4.2.6-impact-v3`). Implementation commit `a9bfd80`. This is
author validation, not independent review or acceptance.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
the committed `a9bfd80` (`git_dirty=no`). Later commits change only docs and
harness reporting. `harness/check-no-c-borg.sh`: no C Borg code in the Rust
image. No engine, data, header or FFI change.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 100 passed, 1 existing ignored / no warnings |
| Native C fixtures, 8 fixtures / 24 modes ([native-comparison.json](native-comparison.json)) | before 15/24, after **24/24**; 9 fail→pass, 15 pass on both sides with byte-identical control logs |
| ASan phase (enlightenment, blowname) | enlightenment: all six modes pass after, no report. blowname: before, exit −11 (SEGV in `strcmp` via `borg_mon_blow_effect`); after, exit 0 and pass. The remaining ASan report after is the pre-existing init-time `borg_sort_comp_hook` overflow, owned separately as source #399, which the consumer package also asserts stays |
| Existing native checker and `CHECK_UBSAN=1` | pass. These checkers extract no function that M4 changes, so this is regression evidence only |
| 131-case milestone2 plan ([matrix.json](matrix.json)) | **130/131 ran in both roles with C/Rust parity** on keys, messages, state and full native saves. `cfg-lunal-deep-warrior` crashes in **both** C and Rust (see below); `matrix.py` exits 1, as designed |
| Behaviour change vs the milestone3 run on the same plan | 116/130 cases differ from the local milestone3 merged run (`harness/out/c-m3/g-c`, not a committed reference). Deaths 23 → 28 among comparable cases. This is a regression observation, not a survival claim |
| Per-fix Rust fault controls (same 23-case subset as M3) | all 8 suites DONE; **3 detected**: 0039 (23/23 cases), 0037 (3), 0038 (1). 0035, 0036, 0040, 0041, 0042 not detected on this subset |
| Removal units | all 16 reverse-apply alone on the final tree; reversing all restores `4.2.6-impact-v3` exactly for the changed files |

## Open finding: engine crash in `cfg-lunal-deep-warrior`

Case: seed 2007, Human Warrior, 1M turns, flags `menu,cheat,lunal`. It
completed under milestone3. Under milestone4, C and Rust both stop with
SIGSEGV at about game turn 20076, with byte-identical messages up to the
crash. Rust also logs "nested Borg entry (depth 2)" first.

Debug C build backtrace ([crash-gdb-backtrace.log](crash-gdb-backtrace.log)):
`do_cmd_read_scroll` → `effect_handler_REMOVE_CURSE` → `uncurse_object` →
`get_curse` → `textui_get_curse` → `curse_menu` (`ui-curse.c:109`
`available[count].power = obj->curses[i].power`). The chosen object has
`obj->curses == NULL` while `obj->known->curses[11]` still has power > 0. The
menu filters on the known list and reads the real one, so it dereferences NULL.
The Borg read an unidentified scroll (identify-by-use), which turned out to be
Remove Curse.

This is engine code, unchanged on official Angband master
(`e473720ed`, checked 2026-11-08). Milestone4 changed the trajectory so the
Borg reaches it; it is not a C/Rust translation difference.

**Root cause (owner chose "root cause first").** A hardware watchpoint on the
crashing object's `curses` field ([root-cause-watchpoint.gdb](root-cause-watchpoint.gdb),
[log](root-cause-watchpoint.log), debug C build) shows:

1. At turn 19210 learning the siren-curse rune runs `player_know_object`.
   That allocates the known list from the real one, which is non-NULL then.
2. At turn 20270 a tengu's blow triggers `take_hit` → "Die? [y/n]". Because the
   case runs with `borg_cheat_death` (the suite's `cheat` flag),
   `borg_react_prompted` calls `do_cmd_wiz_cure_all`. At `cmd-wizard.c:946`
   that frees `obj->curses` and sets it to NULL for every worn item, but leaves
   `obj->known->curses` as it was.
3. Later, Remove Curse filters on the stale known list and `curse_menu`
   dereferences the NULL real list.

The Rust Borg calls the same engine function through FFI
(`engine::do_cmd_wiz_cure_all`, `borg_messages_react.rs`), so both crash
identically. The defect is in the engine's wizard "cure all" command, which
official master has unchanged. It is reachable only through that debug command
or the Borg's cheat-death path, never in normal play. 50 of the 131 plan
cases use `cheat`. No engine correction or cross-repository report has been
made; the owner decides the next step. Reproduce it with the case line from the plan
(`cfg-lunal-deep-warrior 2007 0 0 1000000 menu,cheat,lunal,terminal-save`)
on either image.

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; the maximum Rust wall time is
255.4 s. Only one C pass was run, and no new target reference is exported.
Raw outputs are in `harness/out/c-m4` (native, g-c, g-rust, faults, debug
build), outside Git.
