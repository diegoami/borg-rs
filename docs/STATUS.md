# borg-rs status

## Milestones

| milestone | state |
|-----------|-------|
| M1 deterministic C harness | **done** (check passed 2026-11-04) |
| M2 c2rust transpile behind the seam | in progress |
| M3 safe idiomatic Rust | not started |

## Machine

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
* Race per run in `full`: `(class + k) % 11`, so every race appears.
* Death ends a run (cheat death off). 15/45 `full` runs end in an early
  death, mostly at clevel 1-3.

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

## M2 notes

(see below as work proceeds)
