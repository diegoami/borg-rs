# Native C milestone2 adoption checks

Scope/provenance: [docs/adoption/c-m2](../../../docs/adoption/c-m2/README.md),
[Rust issue6](https://github.com/diegoami/borg-rs/issues/6).

`fixtures/manifest.json` pins all seven original native fixture/config
sources:49 cases. Files are imported byte-for-byte; only the two historical
absolute profile prefixes are relocated in generated copies. Other fixtures
use their existing profile environment variable. Native assertions, rules,
seeds, thresholds, calls and input schedules are unchanged. The Release
adapter defines NDEBUG to match the actual engine flag API; fixture assertions
use the independent native unit-test require/eq macros. The full native49
fixtures inspect C Borg globals, so they run on the C oracle, not the Rust
production image. They are separate from the Rust checks below.

```sh
BORG_IMPL=c harness/build.sh
cmake --build build/c --target OurUnitTestLib --parallel 4
python3 harness/adoption/m2/native.py --expect-fixed --output harness/out/m2-native-c1
python3 harness/adoption/m2/native.py --expect-fixed --output harness/out/m2-native-c2
python3 harness/adoption/m2/kernels.py --ubsan --output harness/out/m2-kernels
python3 harness/adoption/m2/kernels.py --fault ring-activate --output harness/out/m2-fault-ring-activate
```

To reproduce before/after, also build the pre-adoption local C source at
379728289dbce6c2ba2d017073d5d639bcbbd65c and pass that C build directory to
`native.py --build ...` without `--expect-fixed`. The driver and fixed engine
use unchanged game headers and unit helpers. The expected aggregate is32/49
before versus49/49 after. Verify full logs, successful process completion and
the assertion counts, not process exit0 alone. Isolated PROFILE directory
literals differ between runs; comparison aliases only that prefix and retains
the original logs. There are17 expected-choice changes and32 unchanged controls.

`kernels.py` selects actual production C/Rust code with source/item hashes.
It compares three complete item-use functions, regeneration with its Rust
helpers, the swap-selection tail with shared danger-trial inputs, the cold
blow arm, the message-batch parse/reset seam and the unseen-name guard. Fixture
types/callbacks supply trait values, items, eligibility, notes/keys and fixed
RNG results. Outputs include decisions, callback/RNG counts, no-rest bookkeeping
and emitted keys. This does not simulate native spell/damage rules or shadow
all private Borg state. Native49 supplies the engine-backed C correctness
evidence; the separate full-game matrix checks real C/Rust traces/saves.

The fixed kernel corpus has1,785 comparisons, including first-use/elapsed0/1/4/5/
negative potion history, reserve/bypass/74/75 boundaries, independent ring
readiness and activation gates, simulation without casts, unavailable/tied
swaps, cold versus acid and C-string NUL termination. Nine fault controls
restore each old Rust defect (both ring loops independently) and must produce
a mismatch. Fault outputs cannot be used as positive parity evidence.

`import.py DONOR_CLONE` is optional provenance regeneration and only reads
immutable Git objects in the C clone, including the original source-evidence
commits. All imported fixtures/patches needed to run are committed here; normal
validation does not need the donor checkout or network. It refuses existing
destination directories. Original patch credit/licences remain intact.

`compare.py` validates an attributed case join from completed source suites.
Failed originals retain their FAILED markers. It checks actual process success,
binary/source, inputs/config and unchanged wall budgets, reruns harness/diff,
compares all full saves and mutable autosaves, and rejects reusing one C
execution as its repeat. Initial load hashes come from pre-launch config.txt;
private input.save can subsequently be autosaved and its path moves when a
suite's .partial directory is atomically renamed. Eight real-capture controls
cover these boundaries and omitted/modified streams, failed processes and
altered budgets. `test_join.py` creates disposable views, not new game runs.

```sh
BORG_M2_TEST_DIR=harness/out/c-m2 python3 -m unittest discover -s harness/adoption/m2 -p test_join.py -v
```

All131 effective game inputs and per-case origins are retained in the receipt's
input-plan.json. Ordinary9 inputs were uniformly repeated with terminal-save
instead of living-save after the initial capture-only omission;27 three-log/
summary comparisons prove no action/state change and retain all native saves.

For a fresh complete matrix, prepare the correct save union first and run the
three roles sequentially with bounded concurrency. There is no need to
reproduce the initial path/concurrency failures:

```sh
python3 harness/adoption/m2/inputs.py harness/out/m2-reproduce
BORG_IMPL=c SAVE_ROOT="$PWD/harness/out/m2-reproduce/fixtures" JOBS=2 RUN_TIMEOUT_SECONDS=300 harness/suite.sh harness/out/m2-reproduce/adoption-suite-final harness/out/m2-reproduce/c1
BORG_IMPL=c SAVE_ROOT="$PWD/harness/out/m2-reproduce/fixtures" JOBS=2 RUN_TIMEOUT_SECONDS=300 harness/suite.sh harness/out/m2-reproduce/adoption-suite-final harness/out/m2-reproduce/c2
BORG_IMPL=rust SAVE_ROOT="$PWD/harness/out/m2-reproduce/fixtures" JOBS=2 RUN_TIMEOUT_SECONDS=300 harness/suite.sh harness/out/m2-reproduce/adoption-suite-final harness/out/m2-reproduce/rust
python3 harness/adoption/m2/compare.py --plan harness/out/m2-reproduce/input-plan.json --save-root harness/out/m2-reproduce/fixtures --c1 harness/out/m2-reproduce/c1 --c2 harness/out/m2-reproduce/c2 --rust harness/out/m2-reproduce/rust --output harness/out/m2-reproduce/qualified
```

Both production images need current BUILD_IDs (`BORG_IMPL=c harness/build.sh`
and `BORG_IMPL=rust harness/build.sh`). The input generator checks exact
suite/save hashes against the committed receipt and refuses existing output.
