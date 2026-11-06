# Review of borg-rs at 4c06cec08

Reviewed 2026-11-06, full commit `4c06cec087c11ef03696c8e62fdbc7b854b84d4f`.
Project code and tracked reference files were not modified during the review. The report and supporting evidence were initially local only. At the owner’s subsequent request, they are published as temporary documentation on branch `review/m3-4c06cec08`; no issues, PRs or comments were created.

## Verdict

| Claim | Verdict |
|---|---|
| Exact C/Rust key-stream equivalence | Strong finite-suite evidence, not a general guarantee. A reproduced item-copy semantic divergence invalidates the argument supporting the offset representation; its effect on an actual game's keys remains unproven. |
| Safe Rust with unsafe confined to the FFI boundary | Unsafe confinement and the 270-token count check out. Soundness does not: a safe flag wrapper permits invalid memory reads. L4/L5 also remain unsound API contracts, even if today's callers usually observe the conventions. |
| M3 done, safe and idiomatic | Conversion/bridge removal is complete structurally. The unconditional safety and faithfulness interpretation of “done” is premature. Much of the Rust is meaningfully refactored, rather than merely transpiled, but the boundary and item-copy findings need resolution. |

Findings below are ordered by severity. “Reproduced” distinguishes a standalone counterexample from a seeded gameplay failure; no seeded key-stream divergence has been demonstrated by this review.

## Findings

### 1. High — divergence: item-note offsets lose observable pointer aliasing during wear trials

**Locations:** `borg-rs/src/borg_item.rs:155`, `borg-rs/src/borg_item_wear.rs:795`, `borg-rs/src/borg_item_id.rs:157`, `borg-rs/src/borg_trait.rs:2157`; C: `angband/src/borg/borg-item-analyze.c:465`, `borg-item-wear.c:857`, `borg-item-id.c:145`, `borg-trait.c:2222`. The contrary proof is in `docs/PROGRESS.md:192`, repeated in `docs/SELF-REVIEW.md:44` and `docs/REVIEW.md:110`.

**Scenario:** a normal wear trial copies `safe_items[slot]` to the empty pack hole, then copies `safe_items[i]` over the live equipment slot, then calls `borg_notice(true)` before restoring the equipment. The hole is occupied by the old equipment. In C its `note` still points into the live equipment slot, whose `desc` now belongs to the candidate. Rust's hole reads its own old `desc`.

This contradicts row 60's assertion that a modified origin is only read in its listed ring-hole and home-sell cases. The ordinary wear trial is another such case. Restoring the equipment *after* `borg_notice` does not repair the intervening read.

**Reproduction:** the attached `borg-review-note.c` compiles the original C `borg-item.c` and `borg-item-id.c`, uses the real `borg_item` layout, snapshots items and performs those two copies. `borg-review-note.rs` performs the same copies using the actual Rust type and helpers. With old description `Old`, old note at offset 3, `ident=false`, `needs_ident=false`, and candidate description `New longer {??}`:

```text
C:    hole note=[ longer {??}] needs_id=1
Rust: hole note=[] needs_id=false
```

Both descriptions and offsets are in bounds; this counterexample does not depend on C UB. These short descriptions are synthetic. For gameplay relevance, `ident=false, needs_ident=false` is not intrinsically impossible: all runes may be known while an activation's effect is unknown (`angband/src/obj-knowledge.c:734`, `:754`, `:540`). An eligible, longer candidate containing `{??}` can put that substring beyond the old description's end offset. A stock artifact's unknown effect and a candidate with unknown runes are a plausible instance; no seed/save demonstrating the complete instance was produced here.

**Consequence:** the hole can add one to C's `BI_ALL_NEED_ID` but not Rust's. `borg_power.rs:1227` rewards available identification up to that count by 6000 per item. With enough ID supply to avoid saturation (e.g. at least two when the candidate itself also needs ID), trial power can differ, potentially selecting different wear keys. This is a reproduced helper/state divergence and a source-traced path to scoring, **not** a reproduced whole-game key divergence.

**Fix:** retain origin identity plus offset, resolving notes against the current original item, or explicitly emulate that aliasing at every trial read. Do not simply make C use the copy's bytes: that would fix/change the C Borg. Commit the differential regression, including hole/equipment aliasing and known-runes/unknown-effect items, and correct the row-60 proof. Any alternative invariant excluding the example needs a checked engine-level proof.

### 2. High — unsound/UB: safe flag wrappers trust assertions removed from the C build

**Locations:** `borg-rs/src/engine.rs:946`, `:1984`, also `:658`, `:808`, `:793`; C: `angband/src/z-bitflag.c:29`, `:74`, macros in `angband/src/z-bitflag.h:54`, `angband/src/player-spell.c:337`.

`flag_has`'s SAFETY comment says C asserts the offset is in range. That assertion is absent with `-DNDEBUG`. The public safe Rust call `borg_rs::engine::flag_has(&[], 9)` reaches a read at address `0x2`, not a Rust panic. A one-byte allocation with flag 9 instead reads the byte immediately after that allocation. Negative inputs to `flag_next` likewise permit reads before the slice and invalid shift counts; its comment that the C loop stays below the supplied size does not establish a lower bound.

**Reproduced:** the attached all-safe `borg-review-flag.rs` was linked against this release rlib and the actual `z-bitflag.c`, compiled with `-DNDEBUG -fsanitize=address`. ASan reports SEGV in `flag_has`; the attached C allocation reproducer reports heap-buffer-overflow. This wrapper does not require engine initialization, so its failure is independent of the engine-lifetime convention. I did not find a stock gameplay caller supplying an invalid flag.

**Related unchecked APIs found by reading:** `race_flag_has` and `player_has` call the C flag routine directly with unchecked flag IDs. `spell_okay_to_cast(index)` passes an unchecked signed index to C, which directly reads `p->spell_flags[spell]`; for example `-1` reads before the allocated array in an initialized caster. Unlike `player_spell_flags` immediately below it, it has no Rust range assertion. These variants were not separately executed under ASan. The range audit should include them rather than fixing only the demonstrated wrapper.

**Fix:** check the flag domain in Rust before entering C (preserving flag 0's special false result for `flag_has`), including negative inputs and the slice's bit capacity. Audit `flag_next`'s starting flag separately. Add boundary tests. Moving unsafe into one file is insufficient when safe wrappers fail to enforce its preconditions.

### 3. Medium — unsound/UB: L4/L5 are unenforced memory-safety preconditions

**Locations:** `borg-rs/src/engine.rs:483`, `:547`, `:1678`, `:1694`, `:2029`, `:2162`; documented at `docs/SELF-REVIEW.md:47`–48.

**Concrete safe-code scenario in an initialized engine:** retain `let kinds = engine::k_info();`, call `engine::set_kind_ignore(i, new_value)`, then read `kinds[i].ignore`. A live shared Rust reference covers storage written through the raw C pointer, without `UnsafeCell`. No caller-side unsafe is required. Similarly, a copied `GameObject`/`FloorObject` can survive the engine action that frees its object and subsequently dereference it. “It was live when the handle was created” is not a lifetime proof.

**Found by reading; no current-game UB execution demonstrated.** The self-review candidly documents these defects. I found no engine object handle fields in `BorgState`, and the inspected callers generally use handles transiently. That makes the immediate gameplay risk lower, but “within one function” is not sufficient: engine calls and nested entries can occur within that function. These are tolerable as explicit transitional constraints, not as a sound safe abstraction promised by a finished safety milestone.

**Fix:** return owned views/copies for mutable engine tables, and bind object views to an engine access scope that cannot coexist with mutating calls; alternatively keep the unchecked operations unsafe behind an audited internal interface. Use an initialization/thread capability too: public `enter` at `engine.rs:61` mutates a `static mut` root based solely on a single-thread assumption. Safe callers must not be able to invoke it concurrently or access uninitialized engine globals.

### 4. Medium — evidence gap: a Rust-labelled suite passes using the C binary

**Locations:** `harness/suite.sh:25`, `:57`–70, `:90`, `harness/diff:194` (comparison never reads the manifest).

**Reproduced:**

```sh
BORG_IMPL=rust ANGBAND_BIN="$PWD/build/c/game/angband" JOBS=2 \
  harness/suite.sh smoke /tmp/borg-review-wrong-binary
harness/diff --suite smoke /tmp/borg-review-wrong-binary harness/reference/smoke
```

Result: `3/3 runs identical (expected 3)`, exit 0. The manifest says both `borg_impl_env=rust` and `binary_cmake_borg_impl=c`; the suite writes `DONE` anyway. This uses the documented binary override, not forged logs. The same missing enforcement leaves stale executable/source mismatches to human inspection: recording the current checkout revision does not prove the executable was built from it.

**Fix:** reject known implementation mismatches before running; require explicit handling of an unknown implementation for external binaries. Record a build-generated source identity in the executable and validate it for equivalence evidence. Let diff/reporting surface implementation/build identities. The ordinary default-path reruns in this review used the rebuilt Rust executable and are not the wrong-binary experiment.

### 5. Medium — evidence gap: the claimed untested-domain equivalence has no replayable oracle

**Locations:** `docs/REVIEW.md:67`–72 and `:90`–100, `docs/PROGRESS.md:192`; `harness/run.sh:47`–55 selects the single stock config; `harness/suites/deep:1`.

**Concrete missing scenarios:** Morgoth/depth 100 combat; swaps requiring max depth 50; munchkin/lunal mode's state-leaking early returns; every debug command except startup `z`; and note aliasing in finding 1. The stored references top out at depth 26. No amount of replaying these suites verifies a change confined to a depth-50 swap branch or a stock-disabled config branch. Notes and immediate engine-state mutations can differ without changing a key within the finite limit.

The docs acknowledge these gaps, but the positive cross-check numbers cannot be independently rerun: the comparison programs, mutations, seeds and assertions are not committed. A comparison against old c2rust code also assumes that code's fidelity to native C. Finding 1 shows why an informal invariant plus transient instrumentation is insufficient for a representation change.

**Found by reading and inspecting reference summaries. Fix:** preserve native-C differential fixtures/tools, targeted engine-backed state snapshots, config suites and note/direct-state observations. At minimum commit the note-alias regression and a nested-entry test. Qualify equivalence to tested seeds/configuration/limits until broader evidence exists.

### 6. Low — inaccuracy/reproducibility: the advertised clippy command fails on the installed stable toolchain

**Locations:** `borg-rs/src/borg_projection.rs:491`, `:500`; clean claim in `docs/STATUS.md:189`, `docs/PROGRESS.md:194`.

**Reproduced:** `cargo clippy --release -- -D warnings` fails with two `clippy::needless_late_init` errors for the paired `dy/sy` and `dx/sx` initializations. Installed toolchain: rustc 1.98.1, clippy 0.1.98. The historical machine section records Rust 1.97; this does not refute a clean run on that version. There is no pinned toolchain ensuring the reproduction command uses it.

**Fix:** use tuple initializations without changing evaluation order, or document/pin the validated toolchain and test an explicit supported version range. This is a reproducibility problem, not a behavioral defect.

### 7. Low — minor: several current module comments still describe the removed pointer representation

**Locations:** `borg-rs/src/borg_item_wear.rs:19`, `borg-rs/src/borg_store_buy.rs:20`.

**Found by reading:** the module headers say a copied item's note keeps pointing into the original's description. The current type is an offset and actually reads the copy. This matters directly when investigating wear/store what-if mutations, as finding 1 illustrates.

**Fix:** describe the actual representation and its precise equivalence constraints, with a link to a corrected, tested argument.

## Validation results

Fresh validation at the reviewed commit, using Rust 1.98.1 and `JOBS=2` for suites:

| Check | Result |
|---|---|
| `harness/build.sh` and `BORG_IMPL=rust harness/build.sh` | Both succeeded |
| Rust smoke vs C references | 3/3 byte-identical |
| Rust full vs C references | 45/45 byte-identical |
| Rust deep vs C references | 9/9 byte-identical |
| Rust classes vs C references | 16/16 byte-identical |
| Fresh C smoke vs C references | 3/3 byte-identical |
| `harness/check-no-c-borg.sh` | Passed; 154/154 engine objects identical |
| `cargo test --release` | 85 passed, 0 failed, 1 ignored (optional parser fixture dumper) |
| `cargo clippy --release -- -D warnings` | Failed: two needless-late-init errors; finding 6 |
| `borg-rs/tools/count-unsafe.sh` | 270: engine.rs 242, headers 28; no c2rust modules |
| Nested entry reports across all 73 Rust runs | 0 |

The four Rust diffs used `harness/diff --suite NAME`, without `--allow-incomplete`. Suite manifests, timing summaries, diff outputs, unsafe counts and other receipts are preserved alongside the report. Smoke/full/deep started before review artifacts were written; classes records a dirty checkout because the new report/evidence files existed. `git diff --exit-code` at validation time confirmed no tracked source/reference changes; the later publication adds only review documentation and evidence. All four manifests identify the intended Rust binary and exact reviewed checkout; the deliberately wrong-binary experiment has its own separate receipt.

The no-C-Borg check found zero C Borg compilation entries/objects in the Rust build, passed its archive and symbol checks, and found **154/154** engine objects byte-identical between builds on this machine. That is useful positive evidence. It is not a general link-provenance proof: `harness/check-no-c-borg.sh:126` compares build-tree objects rather than reconstructing the executable's actual link inputs; the name-based/archive checks also have the explicitly documented renamed-code limitation. I found no evidence that C Borg code is secretly linked into this freshly built Rust binary. A linker map tied to a clean build would make that claim stronger.

All **118 files** in vendored `angband/src/borg/` match the blob hashes returned for the [pinned upstream directory](https://github.com/angband/angband/tree/e473720ed88a0f0c7ca66815cef3000eb4a38bc3/src/borg), including all 59 C modules. The local checkout does not contain the upstream commit object, so this comparison used the read-only GitHub connector. The engine is harness-instrumented, not literally unmodified upstream: the seed/date/input hooks are documented in `docs/harness.md`. Both builds use the same instrumented engine. Fresh C smoke matches the stored oracle, but I did not regenerate all C full/deep/classes references or independently establish their historical binary hashes.

## Review coverage and checks that held up

- Read the requested review/status/self-review/harness guidance, progress records and C-bug catalogue; inspected scripts and corresponding source around findings. This was not a complete line-by-line audit of 64k C lines or all 270 unsafe occurrences.
- Compared both munchkin attack routines against their complete C module: scan/early-return order, later-type tie preference, and leaked `attacking`/`simulate` flags are preserved (`borg_attack_munchkin.rs:185`, `:309`; C bug section 30). I found no additional difference there.
- Compared the depth-50 weapon-swap scan and final selected-item flag reconstruction against C: option-disabled early return, troll multiplier slip, missing fear writes, final acid-from-electricity slip and retained digger state are deliberately preserved (`borg_trait_swap.rs:385`, `:599`, `:692`; section 45).
- Inspected preparedness static-buffer aliasing: `PreparedC::Buffer` and deferred reads in `borg_think_dungeon_util.rs:750` preserve the overwritten reason rather than accidentally freezing it. This is the right approach for C bug section 47; it contrasts with the item-note representation problem.
- Spot-checked preserved C bugs in danger (cold/acid resistance and CON/STR), `borg_magic_play`'s equality test, and the `swap_letter(-1)` zero-key emulation. These are spot checks, **not certification of every bullet in all 51 sections**. No deliberate silent C-bug fix was established by this review; finding 1 is an unintended representation change.
- State ownership is much clearer than c2rust: explicit `&mut BorgState`, per-module substructures, helpers, enums and owned vectors. `lib.rs:18` denies unsafe outside engine/headers. The bridge/global `st()` scheme is gone. C names, shared whole-state parameters and broad public visibility remain; these are reasonable fidelity tradeoffs but reduce isolation.
- Inspected `enter`/`lend` and the lending paths for messages, event dispatch, prompts, menus, projection, redraw and cure-all. Reborrowing the caller-derived pointer is a sensible mechanism; the closure API prevents normal callers from using the lent state concurrently. I found no specific additional non-lending engine call that demonstrably reaches the Borg hook. No Miri/Tree-Borrows proof or forced-nesting experiment was performed here.
- The diff's equality is raw-byte equality, not normalized text equality. It verifies summary counts/final turns, compares summaries except wall time and requires the exact suite run set. Negative experiments rejected a space-to-tab change (exit 1), a missing smoke run (exit 1), and a plain-log output without `DONE` (exit 2). Fresh-output directories address the old stale/missing-run defects. These checks do not enforce provenance (finding 4).
- Reference-summary coverage agrees with the corrected docs: full has 45 runs, 8 classes/8 races, max clevel 10/depth 5; deep has 9 runs, 8 classes/5 races, max clevel 30/depth 26; classes has 16 runs including 8 actual Blackguards, max clevel 26/depth 22. The ordinary birth-letter bug remains and the classes suite supplies the missing class/races.

## Stale-position glyph panic (#198): not established as a normal-play failure

Locations: `borg-rs/src/borg_messages.rs:459`, `:1089`, `:1252`, `borg.rs:421`, `borg_update.rs:363`, `engine.rs:2699`; C `borg-messages.c:625`, `:1188`, `game-world.c:952`, `:1094`, `:1124`, `mon-make.c:1177`.

Parsing precedes the update of `borg.c`, and the Rust bounds assertion would abort for an out-of-bounds old position. That part of the lead is real. But one must also deliver a hit with tracked glyphs, or a rune-break message, during that stale window. A position alone from an unconstrained fuzz state does not establish that. Level departure/arrival flushes messages; `ui-input.c:397` clears the line. For fresh generated levels, monsters begin with energy `randint0(50)` while the player starts with at least one move's energy; the pre-player monster pass requires more energy than the player has. That obstructs the simple “new level, monster immediately hits, parser aborts” story.

Persistent/restored levels, queued message fragments and nested input deserve targeted testing; this review did not establish their reachability or rule them out. The [tracker issue](https://github.com/diegoami/neo-angband-tracker/issues/198) itself reports only a fuzz lead, not a native-C game reproduction. None of my seeded runs demonstrated this crash.

A faithful **safe** port should not reproduce an out-of-bounds C access as UB. Retain the checked failure until the valid-input domain and desired compatibility policy are explicit. Replacing the old position with the player's new one silently changes C behavior and should not be presented as a faithful translation. On an execution that actually reaches C UB there is no language-defined reference behavior to reproduce; scope the guarantee or document a deliberate safe deviation. A native-C save/seed reproducer is needed before declaring this a normal-play Rust regression.

## Remaining limitations

No endgame save, full engine-state oracle, whole-program sanitizer run, all-config replay, exhaustive kept-bug audit or complete engine-call-graph proof was produced. Temporary conversion cross-checks cannot be recreated from the committed repository. Historical checkpoints and unsafe-count history were not all rerun. The local checkout has tags `m1-harness` and `m2-c2rust`, but no local `m3` ref; I reviewed the requested exact commit and did not treat local tag absence as evidence about the remote repository.

Supporting source reproducers and logs are in [review-evidence-4c06cec08](review-evidence-4c06cec08/). They are review artifacts, not project fixes.
