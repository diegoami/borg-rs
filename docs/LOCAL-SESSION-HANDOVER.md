# Local checkpoint — 2026-11-07

Work branch: `claude/serene-ramanujan-8hzsfm`. Production inputs remain at
`28b8170`; later work is tooling, fixtures, references, receipts and the
requested root launcher. Declared M4 A-F validation is complete, including
the owner's additional synthetic engine scenarios. Main/tags are unchanged;
the owner creates the milestone tag. No validation jobs remain active.

Read CLAUDE.md, docs/FORK-CORRECTIONS.md, M4-PLAN.md, STATUS.md and receipts:

* `docs/validation/2026-11-07-coverage/README.md`
* `docs/validation/2026-11-07-synthetic-tactics/README.md`
* `docs/validation/2026-11-06-native-expanded/README.md`
* `docs/validation/2026-11-07-launcher/README.md`

## Owner policy

Crash-causing inherited C behavior and blatant play blockers may be fixed
narrowly in C/Rust, marked FORK, linked to the tracker and validated. Other
conflicts are case by case; broader divergence follows the verified milestone
and owner-created tag. Original references are preserved; corrected C references
require two identical C passes. Findings/updates go to
`diegoami/neo-angband-tracker`, never `angband/angband`.

## Verified production checkpoint

* #290: stale panel offsets reset at equality with cave bounds.
* #287: actual UTF-8 output byte capacity, reserve NUL and deterministic
  invalid-encoding behavior. The initial byte-limited fix truncated Sméagol;
  the actual-capacity refinement restored all original baseline parity.
* #322: thrust targets require distance exactly one. Warrior2000's former
  `+5` abort at1,887,392 is corrected; all three runs finish5,000,001 turns.

At source28, ordinary C/C and Rust/C pass crash-fixes6/6, extended12/12 and
natural2/2; both builds match105 baseline and8 captured-Rogue references.
All extended original2M/5M limits are reached. Thirty fork-private engine
captures match metadata and complete C/C/Rust bytes. Saved current depth78
is distinct from source observed depth99; fresh natural replays reach84/41.
The audited deepest logged current depths for longWarrior2000/Warrior2007/
Blackguard3100 are52/99/59. The final max_depth summary fields44/99/51 can
reset with recall and are not lifetime peaks. Depth-audit inputs are hashed.
Release tests100 passed/one existing ignored, clippy and154 identical engine
object/actual link-input no-C-Borg checks pass at these unchanged inputs.

## Persistent native checks

Actual C/Rust calculator30k, spatial/flow/physical-melee4k, glyph1,600 and
lunal1,280 states compare the declared logical maps/queues/tracking and ordered
callbacks/notes/keys. Seven actual-source faults, including private-state-only
changes, are rejected. Non-recovering C UBSan, clippy and real dependency-cache
invalidation controls pass. Separate Rust body coverage: melee84/84 lines,
thrust52/52, spread73/73, glyph121/124, lunal154/157. Engine/environment adapters
are explicit; whole-Borg private state and all historical checks are not claimed.

## Real engine synthetic scenarios

`harness/scenarios/` relinks unchanged native C objects with a fixture-only save
constructor/inspector. It never replaces a Borg algorithm. Five canonical engine
saves were generated twice identically; actual replay uses verified normal C,
normal Rust and the frozen Rust coverage image. Six cases pass all three logs
and complete final engine saves. The finalsave flag makes missing/changed saves
fail diff; negative controls are retained.

Ready self-lunal starts depth20/max99/clevel50 with the option on:155 real thinker
entries and descent to21; option-off and max34 controls have zero entries. The
accepted case stops at its first descent. Earlier game-clock/later-depth pilots
timed out during zero-clock stair cycling; their FAILED receipts are excluded.

Glyph-ready starts at100 with20 known Rune scrolls and a safe preparation room:
4 builder entries, one engine glyph, scrolls20 to19. Supply9 gives0 entries/no
glyph. Prelaid24-glyph ring gives4 entries and25 engine glyphs/19 scrolls,
exposing existing#295. A read-only native C stop observer sees23 visible engine
glyphs but Borg map0/tracked1; all normal-C output/save bytes remain unchanged.
No TRAP bit or private Borg field is forced. Full rune formation/scumming/win
are not established; no tactical source correction is applied.

Canonical data/ref: `harness/saves/synthetic-tactics`,
`harness/reference/synthetic-tactics`; suite `harness/suites/synthetic-tactics`.
Reproduction is in `harness/scenarios/README.md`. Native finding#295 is separate
from reverse-flow lead#345 and counter/rollback#118. Source lead#357 concerns
X11 preferences only, not the repaired wrapper save-path issue.

## Full F measurement

All11 groups/139 successful oracle-checked executions use one frozen image:
logic29550/34610 lines (85.38%),1269/1380 LLVM function instances (91.96%).
Branch coverage is unmeasured; closures/monomorphizations count separately.
The original133 baseline (84.65%/91.59%) is retained under original-133;
source26's48-run pilot remains historical. Native checker profiles are separate.

The frozen image/source checkout is `/tmp/borg-rs-m4-coverage` at build revision
f987c07, production input bytes identical to28b8170/current work branch.
It uses instrument-coverage/link-dead-code, LLVM22.1.8 and an explicit profiler
runtime/_exit flush shim. Actual flags and runtime/shim/image hashes are retained.
Only harness/data files were added for the new scenarios; no image rebuild or
identity override was used. Original profiles plus six new profiles form139.
Matching tools remain under `/tmp/borg-llvm-tools`.

One original collector's shell footer failed after all groups passed because its
script was edited live. It did not get full DONE. Strict join independently
validated every accepted group, process, recipe/source/binary/profile hash and
created the assembled full DONE. Current coordinator runs an immutable startup
script snapshot; a deliberate disposable live-edit smoke control passes3/3.
Duplicate complete extended/natural collections passed and are excluded from the
merge. The original attempted interruption was rejected by automatic approval
review, so the collectors were left to finish; no required engine run was lost.

Full local data: `harness/out/coverage/{complete-133,complete-139,current-matrix,
current-natural,synthetic-collection,current-long}` in the isolated checkout.
Durable export retains all139 raw profiles, merged profdata, raw LLVM JSON,
process/config/summary ledgers and all three full-log hashes. Keep the local
image for re-exporting these exact profiles; fresh source can recollect them.

## Owner-requested interactive launcher

`./run-rust-borg.sh` builds ordinary X11/Rust in `build/play-rust` with harness
hooks off and verifies the binary's Rust identity. Main font is the C launcher's
28-pixel Courier (BORG_PLAY_FONT override); explicit one-window layout verified
1362x722. Ctrl-Z, confirm if needed, then z starts Borg; Ctrl-X saves/exits.

Default save: `/home/diego/projects/borg-rs/.angband-rust/save/RustBorg`.
Existing borg.txt is preserved; first stock copy has cheat death FALSE.
BORG_PLAY_DATA_DIR supports isolated profiles. Directory overrides must precede
-u because the engine freezes the filename at -u. This wrapper ordering bug
was fixed. A generated legacy lib/save symlink repaired the owner's already-
running session, and the retry created the57KB save. A separate two-profile
real GUI load/save/reload/save cycle passes with a synthetic level50 fixture;
the owner's save is never used in that test or committed.

## Next work

Present the verified bounded M4 scope for the owner to tag; do not move main or
tag automatically. If the owner asks for a tactical correction, use the linked
source findings and existing scenario controls, then rebuild/checkpoint affected
inputs and update references only after two corrected C passes agree. Do not
infer a win or correct C strategy from parity or a nonzero function entry.
