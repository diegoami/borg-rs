# Carry better-borg-c milestone2

Owner selected this target on2026-11-08 and requested
[Rust adoption issue6](https://github.com/diegoami/borg-rs/issues/6).
The runtime pin is **2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f**;
the separate reviewed documentation/package pin is
**9929eb4eb16412ef2415a5711a3eecab92fa1288**.
[C review38 approval](https://github.com/diegoami/better-borg-c/issues/38#issuecomment-6705894080)
has observations only and no required M2 fixes. Source baseline remains
e473720ed88a0f0c7ca66815cef3000eb4a38bc3. C approval is target qualification,
not Rust signoff or upstream incorporation. Current state is in
[../status.json](../status.json).

Rust starts from main379728289dbce6c2ba2d017073d5d639bcbbd65c, after accepted
m5 at020322749026418a666da3209c169f48ec9d608c plus operational tooling/death
analysis. The five existing local corrections287/290/322/295/345 remain.
No later C milestone, new strategy, historical reference/tag or original death
identity is changed by this batch.

## Reconciliation and implementation

All seven were absent in pre-adoption local C and Rust. They touch separate
sites from the five retained corrections. The six changed C files match the
target consumer blobs exactly; other local code retains its prior ownership.
Data and engine interfaces remain the shared local baseline. The original
[series.json](series.json), [patch bundles](patches) and per-fix
[ledger.json](ledger.json) retain source/consumer/applied/evidence pins, original
credit and independent removal units. Rust translates those exact semantics.

| Patch / source / C consumer | Rust module | Change |
|---|---|---|
|0001 / neo82 / C4|borg_item_wear.rs|Choose lower danger among available emergency swaps; retain weapon ties|
|0002 / neo14 / C7|borg.rs|Reset MsgState's actual reaction-suppression flag after parsing|
|0003 / neo110 / C15|borg_fight_perm.rs|Return the regeneration simulation score without queuing a real spell|
|0004 / neo26 / C17|borg_danger.rs|Read temporary cold protection for cold blows|
|0005 / neo78 / C20|borg_item_use.rs|Include the left ring in both readiness and activation scans|
|0006 / neo77 / C31|borg_item_use.rs|Conserve CCW only after a real prior drink1-4 internal turns ago; preserve same-clock/bypass/reserve rules|
|0007 / neo102 / C36|borg_flow_kill.rs|Match exact case-insensitive unseen "It", retaining C NUL-termination semantics|

The unused C struct dont_react field remains explicitly unused; no engine FFI
layout is replaced. Potion eligibility changes RNG consumption and can produce
different future trajectories: the donor's unfavorable seed47 measurement
remains a documented limitation. A known recent-drink conservation tradeoff is
retained; these fixes are not a demonstrated population survival improvement.

## Checks and evidence

Reproduction and precise boundary qualifications are in
[harness/adoption/m2](../../../harness/adoption/m2/README.md).
Initial author validation establishes49/49 original native scenarios after
versus32/49 before, with17 changed cases and32 unchanged controls; two fixed-C
native passes match after aliasing only the isolated PROFILE path literal.
The native49 fixtures exercise C private globals. Separately,1,785 selected
production-body/seam C/Rust comparisons pass with C UBSan; nine actual Rust
fault controls are rejected. These are bounded shared-environment checks,
not whole-private-Borg equivalence. An initial exact-name length translation
failed the C NUL-boundary control and was corrected before final-source use.

The author's131-case real-game matrix passes corrected C twice and final-source
Rust on the recorded hardware/load under its300s per-case wall: every key/message/state byte and all131 full terminal native saves match.
It preserves smoke/full/deep/classes/config/options/crash/Rune/natural/ordinary
game inputs and budgets. The ordinary9 profiles' living-only capture was
uniformly repeated with terminal capture:27 three-log/summary comparisons and
existing living-save bytes are unchanged; three deaths now retain saves too.
No outcome-based seed/budget selection or policy change is made.

Initial concurrent passes retain3wall timeouts and11save-directory preflight
failures per C/C/initial-Rust sweep. Exact same-input300s-budget C repeats
finish Ranger1M, Blackguard munchkin and Rogue2M; final-source Rust finishes
all ordinary executions under that budget. The original failed suites stay
failed. The attributed case join reruns strict diffs, validates source/binary/
config/input hashes/budgets and compares final and mutable autosaves. The
reference export is read back/decompressed, not accepted from saved PASS text.
The initial guard variant and failed aggregation attempts remain diagnostics.

See [validation receipt](../../validation/2026-11-08-c-m2/README.md),
[target-specific oracles](../../../harness/reference/better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f/README.md)
and [issue6](https://github.com/diegoami/borg-rs/issues/6) for exact identities,
failure/repeat accounting and review handoff. Raw evidence is harness/out/c-m2.

M4/M5 references remain immutable. New target references and any valid joined
case matrix must identify exact build/source/config and retained failure/repeat
origins, and must rerun actual comparisons. No coverage percentage is inherited.
Accumulated review is recorded on issue6; implementation does not imply a new
Rust milestone acceptance or tag.


[Independent review](https://github.com/diegoami/borg-rs/issues/6#issuecomment-6716468916):
**qualified pass; no acceptance blockers**. The reviewer reproduced131/131 C1
cases,125/131 complete across all three roles under the same300s wall, and
125/125 strict joined comparisons. All completed channels and native saves
match; timed-out prefixes match. The131-case author count is hardware/load
specific. Native49, controlled body/seam checks and whole-private-Borg
equivalence retain their distinct scopes. Owner acceptance and a tag remain
pending; later harness measurements are not independent M2-review evidence.
