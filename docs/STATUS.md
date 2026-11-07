# borg-rs status

## Current task: M5 review, then C-consumer milestone adoption

Owner approved preparing M5 at **28468aa9fdc74cbb65d351349b973835d8c42af9**
and the subsequent tracking pivot. [Review#2](https://github.com/diegoami/borg-rs/issues/2)
is open; the [independent follow-up](https://github.com/diegoami/borg-rs/issues/2#issuecomment-6712010550) records **pass with no open findings**
for candidate28468aa plus tooling source0203227. All three findings are verified
fixed and informational qualifications verified. Owner acceptance and final
M5 tag/closure decisions are pending. M4 remains accepted.
No M5 tag or first better-borg-c gameplay milestone is selected. See
[M5 plan](M5-PLAN.md), [reviewer prompt](REVIEW-M5-PROMPT.md),
[adoption contract](C-CONSUMER-ADOPTION.md) and [tracking state](adoption/status.json).

After M5 acceptance, Rust adopts exact accepted better-borg-c bug-fixing
milestones with a patch inventory, linked local issues, controlled evidence,
C twice/Rust parity and distinct references. neo-angband-tracker remains source/
historical provenance and stops independently scheduling Rust adoption.

## Current owner-accepted milestone and post-M4 investigation

The owner accepted the qualified M4 outcome and requested closure of
[review#1](https://github.com/diegoami/borg-rs/issues/1); it is closed as
completed. The qualifications and tag m4/3923a21 remain unchanged.

The post-M4 investigation is published at d0803c6, with the #345 source
tracker update posted/read back. Requested native workflow tooling is now
adapted for Rust: root start-borg.sh, run-borg-headless.py and replay-borg.py,
shared name locks, five backups/restore preservation, frozen headless origins
and verified reruns plus actual terminal/engine diagnostic inspection.
GUI human-input recording/replay is now included: terminal polls/events,
clock values from C/Rust, RNG initialization PID, frozen timezone/file reads,
actual Rust actions, main-terminal game frames and fork-private world saves.
Six real GUI cases (including fresh birth, loaded Rogue, restore, live
preferences and resize) replay exactly. Ten GUI controls and ten headless
controls pass; observer-disabled actions/final save match. See tools/README.md
and validation/2026-11-07-gui-recording. C Borg private state remains omitted;
original saves/data are preserved in testing.

Replay now also accepts better-borg-c's turn/view/output/timeout flags. Both
launchers export completed natural deaths with immutable identities, native
maps/logs, exact original windows or labeled nearby anchors, checkpoint metadata
and unassessed findings/actions. The four earlier deaths are indexed with their
actual twelve C/C/Rust process attempts and four distinct inputs; no historical
movies are invented. GUI archive bytes verify exactly; headless wall-clock log
header differences are explicitly reported while other bytes remain exact.
See docs/deaths and validation/2026-11-07-death-recording.

Post-M4 #345 now builds a complete25-Rune formation autonomously in a
constructed preparation room by turn177; C twice and Rust match3/3 logs
and full final saves. Self-lunal's repeated connected-stair rerolls are
bounded by a new independent key limit; the tactic itself is unchanged.
The first nine-class ordinary cohort (no wizard/load/cheat death) has four
natural deaths and five alive at100k, matching C/C/Rust9/9 and540 full
capture sets. See [the investigation](validation/2026-11-07-post-m4-investigation/README.md)
and [individual deaths](deaths/README.md). These are later source/evidence,
not an update of M4 coverage or proof of unassisted winning play.

## Independent M4 review: qualified pass, no acceptance blockers

[Review issue #1](https://github.com/diegoami/borg-rs/issues/1) is the shared
thread for the implementation agent and independent reviewer. Target:
`m4` = `3923a21e75dfb94053d4cc787d81c5baf638d62e`. The
[final independent review](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707067352)
records qualified pass with no acceptance blockers: A/B/C/F pass for the
declared scope; D/E retain explicit strategy/private-state/domain limits.
The reviewer reran the full C/C/Rust corpus and independently rederived F.
M1-M3 are background and post-M4 #295 is outside that verdict.

The [author response](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707181455)
agrees with all five findings. Post-review follow-ups expand the native
glyph cases/controls, document the historical endgame mismatch, require
extra logs in compressed outputs, rerun coverage oracles and replace the
racy GUI proof driver. See the [follow-up receipt](validation/2026-11-07-m4-review-followup/README.md).
The [independent follow-up](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707651371)
verifies findings2-5 and the originally reported glyph mutants as fixed.
Its new low cost75 boundary gap and oracle-test portability note are addressed
in a [second author follow-up](validation/2026-11-07-m4-review-residual/README.md):
26 scenarios ×100 variants, eleven fault controls and nine oracle tests
passing both on main and an exact-M4 tooling overlay. Verification of these
two small follow-ups remains optional; the owner has accepted M4 and closed #1.
Fixes and receipts are published at `c2b52d9`; the [author follow-up](https://github.com/diegoami/borg-rs/issues/1#issuecomment-6707502737)
was posted/read back and requests independent verification of that commit.
The verdict is not self-promoted to an unqualified pass. The tag, old receipts,
recipes/references and production source are preserved. All three reviewer
worktrees remain available. Current work stays on review follow-up rather
than the tactical queue; discussion and finding dispositions belong in #1.

## Post-M4 glyph recognition corrected (2026-11-07)

After the explicitly delegated `m4` tag, the agreed #295 correction is
mirrored in C/Rust: map_info's known glyph is recognized independently of
harmful player traps. Six real engine before/after controls cover a known
Rune, decoy, harmful pit, hidden Rune, empty floor and prelaid24-rune ring.
The corrected native C ring has map24/tracked24, Morgoth position true and
20 scrolls preserved; M4 had map0/tracked1 and19 scrolls. The hidden Rune
stays unknown and the pit stays a trap. Both C observers preserve all normal
key/message/state/save bytes. Running private-map counts are sampled in C;
Rust matches all three public logs and complete final engine saves.

New recognition6 and original synthetic6 pass corrected C twice and Rust.
Separate C2 references preserve the original M4 oracle. Both implementations
also match all105 original baseline cases, captured-Rogue8, crash-fixes6 and
natural2:121 existing-reference replays each. Tests100/one existing ignored,
clippy, native glyph1,600 and154 engine-object/actual-link-input checks pass.
The interactive Rust build is refreshed; existing game data is untouched.

See the [recognition receipt](validation/2026-11-07-glyph-recognition/README.md).
The extended12 long games and coverage were not recollected for this fix;
their completed M4 results below belong to the tag's production inputs.
Recognition of a prebuilt ring does not establish autonomous construction.
Next is reverse-flow #345, then actual full rune construction, self-lunal
cycling and bounded normal games without cheat death.

## M4 verified for the declared corpus (2026-11-07)

At the M4 tag, production Borg/engine inputs are `28b8170`. The owner-authorized
matching C/Rust corrections address panel-offset equality (#290), UTF-8
output capacity/NUL termination (#287), and same-square melee targets (#322).
The owner explicitly requested main integration: the verified work was
fast-forwarded to main at40194d8. On resume the owner explicitly delegated
tag creation: annotated tag `m4` = `3923a21` is pushed and verified, before
post-M4 tactical changes. See the [main review](validation/2026-11-07-main-review.md).

Verified evidence, with strict build/process checks and no overrides:

* Ordinary C/C and Rust/C crash-fixes6/6, extended12/12 and natural2/2;
  both builds also match105 original baseline and8 captured-Rogue cases.
  All extended2M/5M limits are reached. Warrior2000 finishes5,000,001 turns.
* All30 fork-private engine captures have identical metadata and complete
  C/C/Rust saved bytes. Natural fresh replays start at78/40 and reach84/41.
  Deepest logged current depths in the three long cases are52/99/59;
  resettable final max_depth fields44/99/51 are separately labelled.
* Persistent native C/Rust checker:30k calculator,4k spatial/flow/melee,
  1,600 glyph and1,280 lunal states. Seven actual-source faults rejected;
  non-recovering C UBSan, clippy and dependency-cache controls pass.
* Six new synthetic engine cases pass C/C and normal/coverage Rust/C,
  including complete final engine saves. Real self-lunal entry/first descent
  and one real glyph creation are verified with zero-entry controls.
  Per-case profiles record155 lunal thinker and8 glyph-builder entries.
* Full11-group coverage:139 successful oracle-checked executions from one
  frozen image. Logic29296/34610 lines was84.65% before the new scenarios;
  the combined result is **29550/34610 (85.38%) lines** and
  **1269/1380 (91.96%) LLVM function instances**. Branch coverage is unmeasured.
* Release tests100 passed/one pre-existing ignored, all-target clippy and
 154 identical engine-object/actual-link-input checks pass at the unchanged
  production source. New harness/launcher checks are recorded separately.

The synthetic cases are wizard states, not naturally attained characters.
They establish bounded entry/actions, not unbounded scumming, autonomous
completion of a Sea of Runes, or an unassisted win. Existing glyph recognition
finding#295 and reverse-flow lead#345 remain explicit; no tactical source
correction is inferred. The native checker does not shadow every private
field of an entire running Borg or replace all historical temporary checks.

Receipts: [full coverage](validation/2026-11-07-coverage/README.md),
[synthetic cases](validation/2026-11-07-synthetic-tactics/README.md),
[native checks](validation/2026-11-06-native-expanded/README.md),
[ordinary checkpoint](validation/2026-11-06-melee/README.md),
[depth audit](validation/2026-11-07-depth-audit/README.md).
One original collector footer failed after its completed groups passed;
strict independent group validation/join supplied the final assembled DONE.
The coordinator now snapshots its script and its live-edit control passes.
No failed/partial engine group is accepted.

Interactive use: `./run-rust-borg.sh` opens one X11 window with28-pixel
Courier; Ctrl-Z then z starts the Rust Borg, Ctrl-X saves/exits. Initial
cheat death is off. The default save is `.angband-rust/save/RustBorg`.
The save-path ordering fix and real GUI save/reload roundtrip are verified.

## Milestones

| milestone | state |
|-----------|-------|
| M4 wider parity evidence | independent qualified pass, no acceptance blockers; tag `m4` = `3923a21`; D/E limits explicit; follow-up verification pending in #1 |
| M1 deterministic C harness | **done** (check passed 2026-11-04) |
| M2 c2rust transpile behind the seam | **done** (check passed 2026-11-04) |
| M3 safe idiomatic Rust | done (tag `m3` = `74fc8793c`, after the three external reviews): all 59 modules converted, transitional bridge removed, `unsafe` only in `engine.rs` and the engine declarations in `headers/` (270 at `dd885b6dd`, from 848 at M2; 268 at M4;267 after #295, see PROGRESS), clippy `-D warnings` clean; final checkpoint smoke 3/3, full 45/45, deep 9/9, classes 16/16 identical, `check-no-c-borg.sh` OK (see below and PROGRESS.md); after the external review of `4c06cec08` (findings fixed, see `docs/REVIEW-RESPONSE-4c06cec08.md`) the same checkpoint passed again on `e2020e4aa`, see PROGRESS row 62 |

## Original M1-M3 machine

4 cores, 15 GB RAM. gcc 13, clang/LLVM 18, rustc/cargo 1.97.

## M1: deterministic C harness

See `docs/harness.md` for the design. Decisions:

* Headless frontend `src/main-borgh.c` (`-mborgh`), no curses and no tmux.
  The terminal is a fixed 80x24.
* Seed: `-seed N` or `$ANGBAND_SEED`. `Rand_init()` seeds the WELL RNG
  from it. `borg_rand_local` comes from `randint1()` in `borg_init()`, so
  it is deterministic as well.
* Extra nondeterminism found and removed: `get_mon_num()` reads
  `localtime()` for the Dec 24-26 holiday-monster rule. With a fixed seed
  the date is pinned to the epoch.
* Character creation uses scripted birth keys (race letter, class letter,
  point-based stats, default name "Borg"). Script keys are only fed on
  *blocking* waits. A non-blocking poll during level generation ate `^Z`
  in an early attempt.
* The Borg is started with `^Z z` after setting `NOSCORE_BORG` (skips the
  confirm prompt) and `delay_factor = 0`. `borg.txt` has
  `borg_delay_factor = 0` and `borg_cheat_death = FALSE`.
* Key log: every value returned by `inkey_hack`, including `EVT_NONE`
  (Borg idle while resting). One line each:
  `turn type code mods`. The log hook is in `ui-input.c`, i.e. engine
  side, so it does not depend on which Borg implementation is linked.
* Race per run in `full`: `(class + k) % 11` as requested indices. Because
  of the birth-letter quirk (below) only 8 races and 8 classes are really
  played: no Dunadan, High-Elf, Kobold or Blackguard.
* Death ends a run (cheat death off). 15/45 `full` runs end in an early
  death, all by clevel 5, mostly at clevel 1-3.

### Check M1 output

`full` run twice (`harness/suite.sh full harness/out/c/full-1`, then
`full-2`) and compared:

```
$ harness/diff harness/out/c/full-1 harness/out/c/full-2 | tail -1
45/45 runs identical
$ md5sum harness/out/c/full-1/*/keys.log | awk '{print $1}' | sort | uniq -d | wc -l
0        # all 45 logs (45 distinct seeds) are different from each other
$ harness/diff harness/out/c/full-2 harness/reference/full | tail -1
45/45 runs identical
```

Timings (4 jobs in parallel), full-1:

```
suite=full impl=c jobs=4 wall_seconds=137.850300316
full-0-warrior-0 turns turns=100007 pturns=10239 clev=7 maxdepth=3 keys=13867 wall=25.2
full-0-warrior-1 turns turns=100001 pturns=10167 clev=9 maxdepth=5 keys=13790 wall=28.4
full-0-warrior-2 turns turns=100009 pturns=10252 clev=6 maxdepth=2 keys=15939 wall=24.8
full-0-warrior-3 turns turns=100001 pturns=10155 clev=9 maxdepth=4 keys=14226 wall=21.7
full-0-warrior-4 turns turns=100008 pturns=10292 clev=7 maxdepth=4 keys=14158 wall=21.4
full-1-mage-0 turns turns=100009 pturns=10320 clev=5 maxdepth=1 keys=6412 wall=5.5
full-1-mage-1 death turns=14534 pturns=1487 clev=2 maxdepth=1 keys=1314 wall=1.2
full-1-mage-2 turns turns=100004 pturns=10290 clev=4 maxdepth=2 keys=6997 wall=8.9
full-1-mage-3 death turns=9782 pturns=1002 clev=2 maxdepth=1 keys=1050 wall=1.0
full-1-mage-4 turns turns=100321 pturns=10282 clev=3 maxdepth=2 keys=6904 wall=9.3
full-2-druid-0 death turns=78062 pturns=7976 clev=5 maxdepth=3 keys=9375 wall=12.8
full-2-druid-1 death turns=5865 pturns=609 clev=1 maxdepth=1 keys=881 wall=1.6
full-2-druid-2 death turns=7367 pturns=767 clev=1 maxdepth=1 keys=1085 wall=1.6
full-2-druid-3 death turns=5605 pturns=565 clev=1 maxdepth=1 keys=833 wall=1.1
full-2-druid-4 turns turns=101191 pturns=10239 clev=6 maxdepth=2 keys=10173 wall=12.1
full-3-priest-0 death turns=22136 pturns=2275 clev=2 maxdepth=1 keys=2527 wall=3.7
full-3-priest-1 death turns=11722 pturns=1219 clev=1 maxdepth=1 keys=1434 wall=3.3
full-3-priest-2 turns turns=100008 pturns=10189 clev=3 maxdepth=1 keys=6658 wall=5.2
full-3-priest-3 turns turns=100201 pturns=10351 clev=3 maxdepth=2 keys=8161 wall=9.2
full-3-priest-4 turns turns=100703 pturns=10302 clev=4 maxdepth=2 keys=6252 wall=9.9
full-4-necro-0 turns turns=100352 pturns=10114 clev=4 maxdepth=2 keys=4697 wall=4.2
full-4-necro-1 turns turns=100007 pturns=10075 clev=3 maxdepth=1 keys=5299 wall=5.3
full-4-necro-2 death turns=6416 pturns=650 clev=1 maxdepth=1 keys=639 wall=0.5
full-4-necro-3 death turns=23381 pturns=2361 clev=3 maxdepth=1 keys=1195 wall=0.8
full-4-necro-4 turns turns=100245 pturns=10109 clev=5 maxdepth=2 keys=6617 wall=6.3
full-5-paladin-0 turns turns=100001 pturns=10249 clev=4 maxdepth=2 keys=7386 wall=13.2
full-5-paladin-1 turns turns=100352 pturns=10177 clev=5 maxdepth=4 keys=7159 wall=13.1
full-5-paladin-2 death turns=5021 pturns=516 clev=1 maxdepth=1 keys=670 wall=0.6
full-5-paladin-3 turns turns=100009 pturns=10223 clev=7 maxdepth=4 keys=9640 wall=10.1
full-5-paladin-4 turns turns=100024 pturns=10221 clev=4 maxdepth=2 keys=6337 wall=7.3
full-6-rogue-0 turns turns=100007 pturns=10190 clev=6 maxdepth=2 keys=15617 wall=19.6
full-6-rogue-1 turns turns=100001 pturns=10205 clev=6 maxdepth=2 keys=12596 wall=27.7
full-6-rogue-2 turns turns=100004 pturns=10199 clev=7 maxdepth=2 keys=15251 wall=24.1
full-6-rogue-3 death turns=3376 pturns=355 clev=1 maxdepth=1 keys=564 wall=0.6
full-6-rogue-4 turns turns=100002 pturns=10314 clev=7 maxdepth=2 keys=13860 wall=29.1
full-7-ranger-0 turns turns=100008 pturns=10145 clev=10 maxdepth=4 keys=14392 wall=21.1
full-7-ranger-1 turns turns=100003 pturns=10274 clev=6 maxdepth=3 keys=12984 wall=19.3
full-7-ranger-2 turns turns=100008 pturns=10284 clev=7 maxdepth=4 keys=13662 wall=20.4
full-7-ranger-3 turns turns=100003 pturns=10237 clev=6 maxdepth=2 keys=12837 wall=22.7
full-7-ranger-4 turns turns=100000 pturns=10113 clev=9 maxdepth=4 keys=14112 wall=15.1
full-8-blackguard-0 turns turns=100009 pturns=10271 clev=6 maxdepth=3 keys=14579 wall=23.4
full-8-blackguard-1 death turns=2681 pturns=287 clev=1 maxdepth=1 keys=528 wall=0.6
full-8-blackguard-2 death turns=5757 pturns=607 clev=1 maxdepth=1 keys=992 wall=1.1
full-8-blackguard-3 turns turns=100003 pturns=10239 clev=7 maxdepth=3 keys=13973 wall=21.1
full-8-blackguard-4 death turns=9495 pturns=995 clev=2 maxdepth=2 keys=1466 wall=1.4
```
full-2: `suite=full impl=c jobs=4 wall_seconds=130.697492803`

`smoke` (3 runs x 20k turns): about 5 s wall time.

The default build (`cmake -S angband -B b -DSUPPORT_GCU_FRONTEND=ON`, no
harness) still builds and has no `borgh` symbols.

Speed: about 4-5k game turns/s per run for fighting classes, more for
classes that rest a lot. A whole `full` pass takes about 2m15s on 4 cores.

Reference: `harness/reference/full/` (xz-compressed, 844 KB).

## M2: c2rust transpile behind the seam

* Tooling: c2rust 0.22.1 built against LLVM 18 (`apt install llvm-18-dev
  libclang-18-dev`, then `cargo install --locked c2rust`).
* `borg-rs/tools/transpile.sh` regenerates `borg-rs/src/c2rust/*.rs` (one
  module per `src/borg/*.c`) from the harness build's
  `compile_commands.json`, without `-O3` or the git build id. Flags kept:
  `-DALLOW_BORG -DNDEBUG` (asserts are compiled out, as in the C reference
  build).
* Hand fixes c2rust needed: one only. c2rust emits
  `#![feature(extern_types)]` for opaque engine structs (`dice_s`,
  `history_info`, `monster_group`, `trap`, ...). `tools/postprocess.py`
  turns them into zero-sized `#[repr(C)]` structs, so the crate builds on
  **stable** Rust 1.97 (`raw_ref_op` is stable). Nothing else had to be
  hand-translated: the Borg has no variadic definitions, no
  setjmp/longjmp, no gotos.
* Engine structs: c2rust writes its own `#[repr(C)]` copies of the engine
  header types into each module, from the same headers and flags. This
  amounts to bindgen output, duplicated per module. M3 will collapse them
  into one `ffi` module.
* Build: `angband/CMakeLists.txt` has `BORG_IMPL` (`c` default, or
  `rust`). With `rust`, `src/borg/*.c` is not added to `OurCoreLib`.
  cargo builds the `borg-rs` staticlib (`target-dir build/rust/cargo`) and
  it is linked into the executable. The only engine-to-Borg symbol is
  `do_cmd_borg`. Everything else goes Borg-to-engine (`extern "C"`).
* `borg_engine_date` (`__DATE__` in borg.c) becomes the transpile date
  string. It is shown only by the Borg's version command and never affects
  keys.

### Check M2 output

Output at M2 time (old version of the script; its "exported by the Rust
crate" list matched every archive symbol, so that number meant nothing):

```
$ harness/check-no-c-borg.sh
rust build: src/borg/*.c entries in compile_commands.json: 0
rust build: src/borg object files: 0
C Borg symbols: 1174; present in rust binary: 983; of those exported by the Rust crate: 982
OK: no C Borg code in /home/user/borg-rs/build/rust/game/angband
```

The script was rewritten after the harness review; see "Harness review
fixes" below for its current output.

`full` with the Rust Borg against the C reference:

```
$ BORG_IMPL=rust harness/suite.sh full
$ harness/diff harness/out/rust/full harness/reference/full | tail -1
45/45 runs identical
$ harness/diff harness/out/c/full harness/reference/full | tail -1      # C build after the CMake change
45/45 runs identical
```

Rust timing: `suite=full impl=rust jobs=4 wall_seconds=138.822782034` (C: about 133 s).

## M3: safe, idiomatic Rust

Done on `dd885b6dd` (2026-11-06). Check, run on that commit (suite
manifests name it):

```
smoke    3/3 runs identical (expected 3)
full    45/45 runs identical (expected 45)
deep     9/9 runs identical (expected 9)
classes 16/16 runs identical (expected 16)
nested Borg entries: 0
check-no-c-borg.sh: OK (154/154 engine objects byte-identical)
cargo clippy --release -- -D warnings: clean (whole crate)
unsafe: 270, all in engine.rs (242) and the engine declarations in headers/ (28)
```

Every module was converted with a temporary cross-check against the old
c2rust code (real calls plus fuzz; numbers in each PROGRESS row) and a
checkpoint every few merges. 51 sections of C bugs are kept on purpose
(`docs/c-bugs.md`); from section 42 on each slip is also filed in
diegoami/neo-angband-tracker (`CLAUDE.md`). Open items for the external
review are in `docs/REVIEW.md` and `docs/SELF-REVIEW.md` (L4, L5, the
nested-entry panic, the never-exercised paths listed per module).

## Extra suite: deep (added during M3)

`full` only covers the early game: clevel 1-10 (median 4), max depth 1-5
(median 2), and 15/45 runs die early. The Borg's deep-dungeon, store, spell
and recovery code would go untested. `harness/suites/deep` adds 9 runs
(one per requested class index) with 1,000,000 game turns and
`borg_cheat_death = TRUE` (flag column `cheat` in the suite file). Because
of the quirk below it plays 8 classes (Warrior twice, no Blackguard) and 5
races. It reaches clevel 8-30 (median 18) and max depth 2-26 (median 7).
Its reference comes from the
unmodified C build and was checked deterministic (two C runs, 9/9
identical). It is in `harness/reference/deep/`. `deep` is run alongside
`full` at every M3 checkpoint. This adds to the required suites; it
replaces none of them.

## Known harness quirk: race and class letters in the birth script

`main-borgh.c` picks the race and class in the birth menus with `a`..`z`, but
the menus use `all_letters_nohjkl` (they skip h, j, k, l). So a requested race
or class index >= 7 selects a different entry than the suite file says
(the Borg's own respawn code has the same C bug, see c-bugs.md #11):
class 7 stays Warrior, class 8 becomes Ranger; race 7, 9 and 10 stay Human,
race 8 becomes Half-Troll. So `full` and `deep` never play a Blackguard,
Dunadan, High-Elf or Kobold. This does not affect C/Rust equivalence: both
builds get the same scripted keys, and every `summary.json` records the
race and class actually played. The default is left as is so the existing
reference logs stay valid; the suite names describe the requested indices,
not what was played. The `classes` suite uses the new `-menu-letters`
option (suite flag `menu`) to cover what is missing; see `docs/harness.md`.

## Observation: unexplained early exits (watching)

While converting borg_danger with a heavy cross-check compiled in, an agent
saw two `full` runs (`full-8-blackguard-0` and `-3`, seeds 1080 and 1083;
Rangers in fact, see the quirk) end with exit code 0 and no
`summary.json`. Rerun on their own with the same build, both matched the
reference exactly, and the clean `full` suite was 45/45. Likely cause: the
process was killed from outside under memory or CPU pressure (several
agents run suites at once), but this is not proven. `run.sh` fails such a
run (no summary), `harness/suite.sh` reports it as MISSING and marks the
output dir FAILED, and `harness/diff` refuses that dir, so it cannot pass
silently. (Before the harness review fixes, `suite.sh` stopped at the first
failed run without the MISSING report, and `harness/diff` only compared the
runs present in its first argument.) If it happens
again, check `stderr.log` and `dmesg` for the run.

Update: during the borg_store conversion, two `deep` runs in another
worktree stopped at the same millisecond with a save, the signature of an
external SIGTERM/SIGHUP. Most likely one agent ran `pkill angband`, which
also hits the other worktrees' games. Their key logs matched the
reference up to the stop. From now on, agent instructions forbid killing
processes they did not start (kill by PID only).

## Container restart (during M3, after 27 modules)

The session's container was restarted while three module conversions
(borg_flow_kill, borg_item_use, borg_item_wear) were in progress in agent
worktrees. The main branch was committed and pushed at module 27, so nothing
merged was lost. The worktrees, including their uncommitted edits, survived
on disk, and the conversions were resumed from that state.
Everything merged so far is pushed; only work-in-progress in worktrees is ever
at risk.

## Harness review fixes (2026-11-05)

An independent review of the harness found gaps in the tooling and in the
coverage claims. Fixed:

* `harness/diff` compares the union of run names (`--suite` requires the
  exact suite), compares raw bytes, cross-checks each key log against its
  `summary.json` (`keys`, last `game_turns`), compares the summaries (except
  `wall_seconds`), and refuses a suite output dir without `DONE`. Before,
  it compared only the runs present in its first argument, so a missing
  run passed.
* `harness/suite.sh` runs into a fresh dir, writes `manifest`
  (git rev, binary sha256, `BORG_IMPL`, suite sha256) and `DONE`/`FAILED`
  last, keeps going after a failed run, and always writes `timing.txt`.
* `harness/check-no-c-borg.sh` rewritten (see `docs/harness.md`): staticlib
  members checked by name and `.comment`, empty inputs fail, engine objects
  compared byte for byte.
* `build.sh` always passes `-DBORG_IMPL`; `run.sh` sets `TZ=UTC`;
  `main-borgh.c` exits 3 if the key log or summary cannot be written, and
  has the opt-in `-menu-letters` (suite flag `menu`).
* New suite `classes` with the correct menu letters, covering Blackguard,
  Dunadan, High-Elf and Kobold (never played by `full`/`deep`).

Evidence (worktree build of 590783e1f plus these harness commits):

```
$ harness/check-no-c-borg.sh
1. rust build: src/borg/*.c entries in compile_commands.json: 0
   rust build: src/borg object files: 0
2. staticlib members: 344; rustc objects: 309 (borg_rs crate: 17); compiler-builtins C objects: 35
3. C Borg symbols: 1174; rust binary symbols: 8943; borg_rs exports: 1354; C Borg names in the rust binary: 207, of those exported by borg_rs: 206
4. engine objects: 154 in build/c, byte-identical in build/rust: 154 (buildid.c.o may differ)
OK: no C Borg code in .../build/rust/game/angband
```

The script was also shown to fail on broken inputs (scratchpad, not
committed): a gcc object named like a `borg_rs` rcgu member, a gcc object
named like a compiler-builtins member, the C binary in place of the Rust
one, a stripped binary, a C build dir without Borg objects, one engine
object changed, and a C build dir configured as `rust`.

With the new `main-borgh.c` (C build) and the new `harness/diff --suite`:

| suite | C vs reference | second C run | Rust vs reference |
|---|---|---|---|
| smoke | 3/3 | 3/3 (smoke determinism, first recorded here) | 3/3 |
| full | 45/45 | - | 45/45 |
| deep | 9/9 | - | 9/9 |
| classes | 16/16 (it is the reference) | 16/16 | 16/16 |

The older outputs in the main checkout (`harness/out/c/{smoke,full,full-1,
full-2,deep-1,deep-2}`, `harness/out/rust/{smoke,full,deep}`) also pass the
new diff against the references (with `--allow-incomplete`, as they have no
`DONE` marker). Every reference passes the key-log/summary cross-check.

Valgrind memcheck (`--track-origins=yes`) on the three `smoke` runs with
the C build: 0 errors each, and the key logs are identical to the
reference. Memcheck does not see reads past the end of a static array
inside the same object (like the `all_letters_nohjkl[-1]` read in
c-bugs #21), so that class of bug is not covered.

## Build identity (external review finding 4, 2026-11-06)

The external review of `4c06cec08` (finding 4) showed a Rust-labelled suite
passing with the C binary: `BORG_IMPL=rust ANGBAND_BIN=build/c/game/angband
harness/suite.sh smoke` wrote `DONE`, and `harness/diff` reported 3/3,
exit 0. Reproduced here before the fix. Now `harness/build.sh` writes
`build/<impl>/BUILD_ID`, `harness/suite.sh` refuses a binary of the wrong
or an unknown implementation, or one that does not match its `BUILD_ID` or
the current sources, before any run (overrides `ALLOW_UNKNOWN_IMPL=1`,
`ALLOW_STALE_BUILD=1` are recorded in the manifest), and `harness/diff`
prints each output's build identity and fails on a mismatch or override
(`--expect-impl c|rust`). `check-no-c-borg.sh` gained step 5, the actual
link inputs from `ld --trace` of a relink that reproduces the binary
byte for byte. Details in `docs/harness.md` ("Build identity") and
`docs/SELF-REVIEW.md` section 5.

Evidence (worktree build of `4c06cec08`, harness changes on top, `JOBS=2`):

```
wrong-binary command                     suite: REFUSED, exit 2, no output dir
                                         (before the fix: exit 0, diff 3/3 exit 0)
C smoke   --expect-impl c                3/3 runs identical, exit 0
Rust smoke --expect-impl rust            3/3 runs identical, exit 0
Rust full  --expect-impl rust            45/45 runs identical, exit 0
check-no-c-borg.sh                       OK; 5. link inputs: engine objects 154,
                                         staticlib members pulled in 31, system 28
```

Negative checks (scratchpad, not committed): stale sources (an untracked
file under `angband/src`, an edited tracked file) refused for C and Rust;
a file added under `borg-rs/` makes only the Rust build stale; a binary
changed after the build, a build dir without `BUILD_ID`, and a Rust
executable in a C build dir refused (the last one also with both
overrides); a stripped external binary is `unknown` and runs only with
`ALLOW_UNKNOWN_IMPL=1 ALLOW_STALE_BUILD=1`, both recorded; `harness/diff`
fails (exit 1) on outputs with an override, an edited `borg_impl_env`, and
`--expect-impl rust` on a C output, and refuses (exit 2) an output without
a manifest or with an old manifest. `check-no-c-borg.sh` step 5 fails on an
extra C object in the link, a binary the link command does not reproduce,
and a gcc object in the staticlib under a rustc member name.

## Config suite (second external review, finding 4, 2026-11-06)

The review of `821cad310` noted that every suite used the stock
`borg.txt`, and ran munchkin start and lunal mode by hand. `harness/run.sh`
now has config flags `munchkin`, `lunal` and `selflunal`. Each sets one
option of the run's `borg.txt` to `TRUE`; with none of them the file is
unchanged, byte for byte. Each run records its flags and the sha256 of its
`borg.txt` in `config.txt`, and the suite manifest copies them as
`run_config.<run>`. `harness/suites/config` has the review's six runs
(munchkin and lunal, each on Human Warrior 42, Half-Elf Mage 12345,
Dunadan Blackguard 1080, flags `menu,cheat`, 100k/100k/20k turns) and one
1M-turn lunal run (Human Warrior 2007, clevel 24, max depth 27). It takes
about 4 minutes with `JOBS=2`.

The munchkin thinker runs in all three munchkin runs; lunal mode stays on
in all four lunal runs, but the lunal thinker is never entered. Self-lunal
was not reached by configuration alone: eight 1M cheat-death runs with
`selflunal` (with and without `lunal`) never passed its gate, so no suite
run uses that flag. Details in `docs/harness.md` ("The config suite").

Evidence (`JOBS=2`):

```
C config, twice (C build of c20622542)    7/7 runs identical; reference/config
Rust config --expect-impl rust            7/7 runs identical, exit 0
Rust smoke, C smoke (stock suites)        3/3 each against reference/smoke, exit 0
```

## Options suite (M4 step B, 2026-11-06)

`harness/run.sh` takes a config flag `NAME=VALUE` for any `borg.txt`
option (`TRUE`, `FALSE` or an integer; an option without exactly one line
in `harness/borg.txt` fails the run), and `cheatlive` (frontend
`-cheat-live`: game option `cheat_live` on, no stop at death, so with Borg
cheat death off the Borg reincarnates). The borgh frontend answers, with
ESC, a `-more-` shown before the Borg's first key: the start-up warning
of `borg_uses_dynamic_calcs`, which made both Borgs stop at turn 1 with
zero keys. Stock runs are unchanged: their generated `borg.txt` has the
same sha256 as with the previous `run.sh`, and C and Rust smoke pass.
`harness/diff --suite` checks the new flags against `config.txt` and
`cheat_live` against the summary.

`harness/suites/options` has 25 runs (22 x 100k, 200k, 400k, 1M; about
4 minutes with `JOBS=2`): money scum with and without self scum, including
the "Money Scum complete" stop; dynamic calcs with the stock formulas on
three classes; the six worships; risky play; unique hunting; swaps off;
`borg_no_deeper`, `borg_stop_dlevel`, `borg_stop_clevel`; autosave;
verbose; chest tolerance; munchkin level/depth limits; respawn with fixed
and random race/class (five and one reincarnations). A temporarily
instrumented C build showed each option read where it is used; 20 of
the 25 runs change the key stream against the same character with the
stock `borg.txt`. Worships speed and hp, swaps off, verbose and chest
tolerance are read but change no key in any game tried. Details and the
options not covered in `docs/harness.md` ("The options suite").

Evidence (`JOBS=2`):

```
C options, twice (C build of fd6f6b120)   25/25 runs identical; reference/options
Rust options --expect-impl rust           25/25 runs identical, exit 0
Rust config --expect-impl rust            7/7 runs identical, exit 0
Rust smoke, C smoke (stock suites)        3/3 each against reference/smoke, exit 0
```

## Messages and state log (M4 step A, 2026-11-06)

Before, a difference in Borg notes or in game state that never changed a
key was invisible. Each run now writes two more logs beside `keys.log`
(details in `docs/harness.md`, "The borgh frontend"):

* `messages.log`: every `message_add()` call (repeats included), with the
  key count, game turn and message type. The hook is in the engine's
  `message.c` (`#ifdef BORG_HARNESS`). Game messages and Borg notes both
  go through `message_add()`: C `borg_note()` calls it directly,
  `borg_warning()` through `msg()`, and the Rust `borg_io.rs` calls the
  same engine functions, so the hook is the same code for both builds.
* `state.log`: one line per key (line i is key i), written by the borgh
  frontend: grid, depth and max depth, HP, SP, food, gold, exp, level,
  the set timed effects, the gear timeouts, and FNV-1a hashes of the
  pack/quiver and of the equipment (kind, number, pval, ac, to_h, to_d,
  to_a, notice, flags, modifiers, element info, curses, and the same of
  each object's known version: its learnt runes) and of the player's rune
  knowledge. Read only: no RNG, no state-changing call.

`harness/diff` compares both byte for byte (and checks the state log
against the key log line by line). A log on one side only fails the run,
unless that side is a stored dir made before the logs existed (then it is
noted, not compared). The C Borg sources are untouched.

Evidence (C build and Rust build of `10303cba9`, `JOBS=2`):

```
C pass 1 and pass 2, every suite      keys.log and summary.json (minus wall_seconds)
                                      byte-identical to the stored references:
                                      smoke 3/3, full 45/45, deep 9/9, classes 16/16, config 7/7
C pass 1 vs C pass 2 (harness/diff)   80/80 runs identical, messages and state compared in all
Rust vs reference (--expect-impl rust) smoke 3/3, full 45/45, deep 9/9, classes 16/16,
                                      config 7/7: keys, messages and state identical, exit 0
```

So the new hooks did not change any key stream, and the Rust Borg writes
the same notes and leaves the same per-key player state as the C Borg on
all 80 runs. C pass 1's logs were added to the references as
`messages.log.xz` and `state.log.xz` (xz -9): 10.9 MB and 6.9 MB, beside
2.4 MB of key logs; the largest file is 1.4 MB
(`deep/deep-7-ranger/messages.log.xz`). Keys, summaries, config files and
manifests of the references are unchanged.

Limits: the digest covers the player, not the cave, the monsters, floor
objects or stores; the hashes are 64-bit (a collision could hide a gear
difference on one line, not a lasting one); messages after the run's
last key are not logged (the run exits on that key); the options suite
(step B) reference predates these logs, so `harness/diff` notes and skips
them there until its reference gets them.
