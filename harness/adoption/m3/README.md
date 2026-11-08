# Native C milestone3 adoption checks

Scope/provenance: [docs/adoption/c-m3](../../../docs/adoption/c-m3/README.md),
[Rust#8](https://github.com/diegoami/borg-rs/issues/8).

`fixtures/manifest.json` pins the 26 native fixtures behind consumer patches
0008-0034 (all except 0025, whose fixture calls the five-argument
`borg_what_text()` that local #287 replaced). 23 are the original
neo-angband-tracker evidence files at their recorded commits; glyph-tracker,
dispel-filter and blind-frighten are the consumer's adapted fixtures at
`702f091f6`, exactly as its own verification scripts use them. Fixture bytes are
unchanged; `config_overrides` records the one profile edit an evidence script
makes when materializing borg.txt (munchkin-244 enables `borg_munchkin_start`).
`import.py TRACKER_CLONE CONSUMER_CLONE` regenerates the copies from Git objects
only; normal validation needs neither clone nor network.

The fixtures inspect C private Borg globals, so they run on the C oracle (the
Rust production image excludes C Borg code). Rust is checked by the C/Rust game
matrix. A mode passes only when its own `native-c/<id> finished: 1/1 passed`
marker is printed; exit status alone is never a pass.

Build four oracle directories (core and unit-test objects; game data is taken
from an ordinary `harness/build.sh` image, identical for both sides):

```sh
git worktree add ../borg-rs-before 0bc7a09fb41a01cfbc4f5bb86b89002b074f3712
BORG_IMPL=c harness/build.sh
O=harness/out/c-m3/oracles
for spec in "../borg-rs-before before" ". after"; do set -- $spec
  cmake -S $1/angband -B $O/$2 -DCMAKE_BUILD_TYPE=Release -DSUPPORT_BORG_HARNESS=ON -DBORG_IMPL=c
  cmake -S $1/angband -B $O/$2-asan -DCMAKE_BUILD_TYPE=Release -DSUPPORT_BORG_HARNESS=ON -DBORG_IMPL=c \
    "-DCMAKE_C_FLAGS=-fsanitize=address -fno-omit-frame-pointer"
done
for b in before after before-asan after-asan; do
  make -C $O/$b -j4 OurCoreLib OurUnitTestLib OurExecutable
  ln -sfn "$PWD/build/c/game/lib" $O/$b/game/lib
done
python3 harness/adoption/m3/native.py --build $O/before --asan-build $O/before-asan --output harness/out/c-m3/native-before
python3 harness/adoption/m3/native.py --build $O/after --asan-build $O/after-asan --output harness/out/c-m3/native-after
```

Expected aggregate: 37/64 normal modes before, 62/64 after; 26 modes change
from fail to pass, 36 pass on both sides, and the two dispel-259 modes are red
after at the values the consumer package pins as a deliberate cross-carry
interaction (`reserve` −10331, `clear` 169: losdispel-326 adds the previously
leaked contribution). ASan/LSan phases: glyph-tracker `stairs`
(heap-buffer-overflow), missile-254 `no-target` (heap-buffer-overflow) and
glowoverflow-311 `glow` (global-buffer-overflow) report before and are clean
after; calcreload-325 leaks 320 bytes/4 allocations before, opblock-324 and
calfree-323 160 bytes/2 allocations each; no leak is reported after.

The game matrix reuses the exact milestone2 131-case plan:

```sh
python3 harness/adoption/m2/inputs.py harness/out/c-m3/plan
for impl in c rust; do
  BORG_IMPL=$impl SAVE_ROOT="$PWD/harness/out/c-m3/plan/fixtures" JOBS=3 RUN_TIMEOUT_SECONDS=600 \
    harness/suite.sh harness/out/c-m3/plan/adoption-suite-final harness/out/c-m3/g-$impl
done
python3 harness/adoption/m3/matrix.py
```

`matrix.py` compares C and Rust per case (keys, messages, state, full
terminal native save) and C against the committed milestone2 reference to
count the cases whose behaviour this batch changes.

Per-fix Rust fault controls: in a separate worktree at the implementation
commit, for each `docs/adoption/c-m3/removals/NNNN-rust.patch` restore
`borg-rs/src`, `git apply -R` that one unit, `BORG_IMPL=rust harness/build.sh`,
run the 23-case `docs/validation/2026-11-08-c-m3/fault-suite` into
`harness/out/c-m3/faults/NNNN`, then rerun `matrix.py` (it reports, per fix,
the cases that differ from the corrected C run). Fault outputs are never
positive parity evidence.
