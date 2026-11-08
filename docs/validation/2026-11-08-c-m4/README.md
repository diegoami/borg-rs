# Carry C milestone4 fixes into C oracle and Rust

Owner direction (2026-11-08): with `4.2.6-impact-v3` tagged, proceed to C
milestone4. Tracking [Rust#11](https://github.com/diegoami/borg-rs/issues/11);
record [adoption/c-m4](../../adoption/c-m4/README.md). Target better-borg-c
`70993393a1c946ce8cd38ee558bf1ce6464350e7` (`4.2.6-impact-v4`). Start: Rust
main `3211e91` (`4.2.6-impact-v3`). Carries `a9bfd80`; engine correction
`eb29098` with the tracker link in `9a78e32`. This is author validation.
The independent review (qualified pass) is on #11; owner acceptance is
separate.

## Identities

[C-BUILD_ID](C-BUILD_ID) and [RUST-BUILD_ID](RUST-BUILD_ID) are clean builds of
the committed `9a78e32` (`git_dirty=no`, `inputs_dirty=no`; source hashes C
`8c039db9…`, Rust `f8f7e781…`, reproduced by `harness/identity.sh` at the
receipt commit). They supersede the earlier receipt pair from `a9bfd80`
(C `b47e1d5d…`, Rust `2a87c71d…`). The difference is the engine correction
below, so **the engine input hash changed in this batch**: `cmd-wizard.c` is
the only engine file touched. No data, header or FFI change.
`harness/check-no-c-borg.sh`: no C Borg code in the Rust image, and the engine
objects are byte-identical between the two images.

## Engine correction: wizard cure-all leaves known curses

Case `cfg-lunal-deep-warrior` (seed 2007, Human Warrior, 1M turns,
`menu,cheat,lunal`) completed under milestone3. After the milestone4 carries,
C and Rust both stopped with SIGSEGV at game turn 20076, with byte-identical
keys, messages and state up to the crash.

Root cause, from a hardware watchpoint on the crashing item's `curses` field in
a debug C build ([root-cause-watchpoint.gdb](root-cause-watchpoint.gdb),
[log](root-cause-watchpoint.log); first backtrace in
[crash-gdb-backtrace.log](crash-gdb-backtrace.log)):

1. Turn 19210: learning the siren-curse rune runs `player_know_object`, which
   fills the item's known curse list from the real one.
2. Turn 20270: a tengu's blow triggers `take_hit` → "Die? [y/n]". With
   `borg_cheat_death` (the suite's `cheat` flag), `borg_react_prompted` calls
   `do_cmd_wiz_cure_all`. At `cmd-wizard.c:946` that frees the real `obj->curses`
   and sets it to NULL, but leaves `obj->known->curses`.
3. A later Remove Curse (an unidentified scroll read to identify it) offers the
   item from the stale known list, and `curse_menu` (`ui-curse.c:109`) reads the
   NULL real list.

The Rust Borg calls the same engine function through FFI, so both images crash
identically. The engine code is unchanged on official master `e473720ed`.
Normal play reaches it only through the debug command. The independent review
reached the same root cause by a separate route.

Correction (owner-directed, narrow): `do_cmd_wiz_cure_all()` also frees
`obj->known->curses` (guarded) and sets it to NULL. It is recorded in
[FORK-CORRECTIONS.md](../../FORK-CORRECTIONS.md) and `c-bugs.md` §59, and
reported upstream as
[neo-angband-tracker#474](https://github.com/diegoami/neo-angband-tracker/issues/474)
for that repository's own process. With it, the case completes 1,000,002 turns
in both images with identical logs and saves. **No other plan case changes**:
the other 130 C cases are byte-identical on all four streams to the pre-fix
run.

## Checks

| Check | Result |
|---|---|
| `cargo test --release` / clippy `--all-targets` | 100 passed, 1 existing ignored / no warnings |
| **C-oracle** native fixtures, 8 fixtures / 24 modes ([native-comparison.json](native-comparison.json)) | before 15/24, after **24/24**; 9 fail→pass, 15 pass on both sides with byte-identical control logs. The fixtures read C-private Borg state, so they cover the C side of each carry only. Rust coverage comes from translation review plus the parity matrix; for 0035, 0036, 0040, 0041 and 0042 there is no Rust-side behavioural control (see the fault controls row) |
| ASan phase (enlightenment, blowname) | enlightenment: all six modes pass after, with no report. blowname: before, exit −11 (SEGV in `strcmp` via `borg_mon_blow_effect`); after, exit 0 and pass. The remaining ASan report after is the pre-existing init-time `borg_sort_comp_hook` overflow (source #399, owned separately), which the consumer package also asserts stays |
| Existing native checker and `CHECK_UBSAN=1` | pass. It extracts no function that M4 changes, so this is regression evidence only |
| 131-case milestone2 plan at `9a78e32` ([matrix.json](matrix.json)) | C and Rust 131/131 completed (DONE). **131/131 C/Rust parity** on keys, messages, state and full native saves, and the `harness/diff` stream comparisons agree. `harness/diff` still **exits 1** for one consistency check (next row), so `matrix.py` exits 1 |
| `harness/diff` consistency: `opt-stopclevel-war` | in **both** images identically, the Borg's clevel auto-stop fires inside a throw command (the flask kills the ant, level 5, "Aborting (Auto-stop…)", hook removed). The game finishes that turn without a key, so the last key is turn 30106 and the summary says 30116. The check requires them to be equal for `borg_stopped`. Under milestone3 the stop happened on a key turn. This is a harness invariant meeting a new legitimate trajectory, not a C/Rust difference; the check was **not** relaxed. The independent review reported `harness/diff` exit 0 on the corrected tree; I could not reproduce that with these committed builds |
| Behaviour change vs the milestone3 run on the same plan | 117/131 cases differ from the local milestone3 merged run (`harness/out/c-m3/g-c`, recorded in matrix.json as `reference`; not a committed reference). Deaths over the whole plan: 23 (M3) → 28 (M4); among the 117 changed cases, 21 → 26. This is a regression observation, not a survival claim |
| Per-fix Rust fault controls (same 23-case subset as M3, built at `a9bfd80`) | all 8 suites DONE; **3 detected**: 0039 (23/23 cases), 0037 (3), 0038 (1). 0035, 0036, 0040, 0041 and 0042 were not detected on this subset; non-detection is not proof of non-execution |
| Removal units | all 16 reverse-apply alone on the final tree; reversing all of them restores `4.2.6-impact-v3` exactly for the changed Borg C/Rust files. The engine correction is separate |

Budget: `JOBS=3`, `RUN_TIMEOUT_SECONDS=600`; maximum Rust wall time 261.4 s.
Only one C pass was run, and no new target reference is exported. Raw outputs
are in `harness/out/c-m4`, outside Git: native, `g-c`/`g-rust` (corrected),
`g-*-precurse` (pre-fix), faults, and the debug build.
