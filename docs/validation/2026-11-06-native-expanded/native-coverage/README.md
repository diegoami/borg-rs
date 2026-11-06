# Supplemental native checker coverage

All five positive runs passed their actual native C/Rust comparisons:
calculator30,000; spatial2,000 for each of two seeds; glyph1,600; lunal1,280.
Only those five process profiles are merged. Build-script profiles and all
fault-control variants are excluded. C code is uninstrumented. These are
Rust measurements of the extracted bodies in the explicit fixture environment,
separate from game-corpus M4 F. Branch coverage is not measured.

| extracted production body | covered Rust lines | covered regions |
|---|---:|---:|
| Physical melee damage |84/84 (100%)|134/134|
| Thrust selection |52/52 (100%)|116/116|
| Flow spread |73/73 (100%)|114/114|
| Glyph builder |121/124 (97.58%)|188/194|
| Lunal thinker |154/157 (98.09%)|269/284|

The earlier nine-glyph/32-lunal scenario corpus covered101/124 and146/157
lines respectively (`functions-report-before.txt`). Those gaps prompted
seven extra glyph scenarios and eight lunal scenarios. The new cases cover
outlying selection, an already-glyphed outlying cell, incomplete/complete
new centers, unsuccessful creation, an unreachable completed center and
unsuccessful follow-step. Lunal flow outcomes now distinguish GOAL_TAKE
from GOAL_FLEE, reaching otherwise masked fleeing returns, plus depth-one
selling and unsafe stair descent. Production algorithms are unchanged.

`functions-report.txt` is the original LLVM output for generated items. Its
aggregate also includes fixture constructors/adapters and is not a whole-Borg
percentage. `selected-kernels.json` identifies the five rows and entry counts.
`coverage.json.xz`, `coverage.profdata`, `profiles.tar.xz`, generated items,
source/item ledger in the parent receipt and binary/profile hashes retain
measurement inputs/results. Nonzero entry counts do not prove a game tactic.
The remaining lines/regions are explicit rather than rounded to full coverage.

To reproduce, use the committed native checker source and matching LLVM tools
(LLVM22.1.8 for rustc1.98.1 here). Build both checker binaries with
`RUSTFLAGS='-C instrument-coverage'` in a fresh CARGO_TARGET_DIR; direct build
profiles into a separate directory with LLVM_PROFILE_FILE. Then execute the
built binaries directly with LLVM_PROFILE_FILE set to a fresh directory's
`%m-%p.profraw`: calculator30000; spatial2000; spatial2000 305441741;
spatial1600 --glyph; spatial1280 --lunal. Each must exit0 and print PASS.
Merge only those five profiles with llvm-profdata. Export with llvm-cov using
the spatial executable and `--object` for the calculator executable. Use
`llvm-cov report --show-functions` against that build's generated
`spatial_items.rs` to obtain the selected-function line/region counts.
