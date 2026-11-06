# Second external review: borg-rs at 821cad310

(Temporary review document, kept until the owner removes it. The owner
pasted the review into the session; this is a condensed record of it: the
verdict tables, every finding with its locations, scenario and suggested
fix, and the reproduced results are kept, some prose and the detailed
"checked and found fine" section are shortened. The project's answer is
`docs/REVIEW-RESPONSE-821cad310.md`.)

Reviewed 6 November 2026. Exact commit: `821cad31070bab6637347fd891806146ace472c9`.

The primary checkout remained detached at this commit, with no tracked changes. Experiments and modified configurations were kept separately. No branch was modified, and no issue, pull request, or comment was posted.

## Verdicts

| Claim | Verdict | Reason |
|---|---|---|
| Equivalence within the stated seeds, stock configuration, and turn limits | **Holds for the reproduced evidence.** | Fresh C smoke and all four Rust suites match their references. No seeded C/Rust key divergence was found. This is a finite observation, without an endgame or whole-state guarantee. |
| All 59 modules converted; unsafe confined to the boundary | **Holds structurally.** | All counterparts exist, the bridge/c2rust code is gone, and the count is 269: engine 241, headers 28. The no-C-Borg check passes, including the reproducing relink. |
| The exported safe Rust API is sound | **Does not hold.** | All-safe callers still reach undefined behavior through public engine wrappers. Initialization/thread preconditions remain unenforced; two arithmetic wrappers also permit C signed overflow. |
| M3 "safe, idiomatic Rust" done | **Partially holds.** | Conversion and substantial refactoring are complete. Unconditional safety certification is premature; the boundary and evidence gaps below remain. |

| First-review fix | Verdict at this commit |
|---|---|
| 1. Note aliasing | **Holds for defined C executions with the stock live allocations.** The original scenario and expanded copy/origin cases agree. Dangling pointers after freeing/reallocation have no defined C result to reproduce. |
| 2. FFI preconditions | **Partially holds.** The flag and listed index repairs work; completeness of the safe facade does not. |
| 3. L4/L5/root | **Partially holds overall.** The borrow-bound object handles and mutable-table copies withstand the checks below. The public initialization/thread problem remains. An extracted root passes Miri under both borrow models, which supports but does not prove the whole boundary. |
| 4. Provenance | **Partially holds.** Ordinary wrong/stale binaries and recorded overrides are rejected. Compression bypasses the expected-implementation check. |
| 5. Untested domains/nesting | **Partially holds.** The committed regression exercises real reentry and completes. Endgame/state-oracle evidence is still missing. |
| 6. Clippy | **Holds.** Clean on both documented versions, 1.97.0 and 1.98.1, and additionally 1.99.1. |
| 7. Cited stale wear/store-buy comments | **Holds at those locations.** A separate stale item-state comment remains. |

## Findings, most severe first

### 1. High — unsound/UB: public safe wrappers lack an initialization/thread capability

**Locations:** `borg-rs/src/engine.rs:194` (`z_info`), `:453` (`player_ref`), `:549`–`:571` (global RNG/hook operations), `:182` (`check_access`); `borg-rs/src/lib.rs:86` exports the facade. C: `angband/src/init.c:78` defines the initially NULL `z_info`; `angband/src/player.c:36` defines the initially NULL `player`.

**Scenario, reproduced:** an external rlib client containing no unsafe code calls `println!("{}", borg_rs::engine::z_info().k_max);`. Linked against the actual `init.c` definition, before initialization, this exits with **SIGSEGV, status 139**. `z_info()` constructs a reference from the NULL global. It never calls `enter`, so `enter`'s readiness check does not protect it. Public callers of `player_ref` have the same architectural problem. By reading, safe clients can also call `rand_state`/`set_rand_state` concurrently. The wrappers have no capability, lock, atomic, or thread check. Even an object constructor using a fabricated `Access::default()` reads `ROOT` unsynchronized in `check_access` before rejecting the token. I did not execute a race reproducer.

**Why it matters:** privatizing `enter` closes one entrance, but does not make the exported facade sound. **Suggested fix:** provide operations through a non-Send/non-Sync engine access scope created only after initialization on the engine thread, and require it everywhere live engine state is touched. Alternatively make the convention-dependent facade crate-private, with any external unchecked entrance explicitly unsafe. Restricting only `enter` is insufficient. This is a **synthetic external-client counterexample**, not seeded-game UB.

### 2. Medium — unsound/UB: arithmetic wrappers accept inputs that overflow C

**Locations:** `borg-rs/src/engine.rs:211` (`distance`), `:3105` (`modify_stat_value`). C: `angband/src/cave-view.c:38`–`:45`, `angband/src/player-util.c:339`–`:355`.

Reproduced with UBSan through the real Rust wrappers and the original C sources: `distance(loc { x: i32::MIN, y: 0 }, loc { x: 1, y: 0 })` (`1 - INT_MIN` overflows); `modify_stat_value(i32::MAX, 1)` (`INT_MAX + 10`); `modify_stat_value(18, i32::MIN)` (negating `INT_MIN`). These need no initialized engine. **Suggested fix:** checked/widened arithmetic in Rust preserving results on C's defined domain, or assert a documented domain before crossing FFI (distance: subtraction, absolute value and final addition; stat changes: negation and cumulative positive adjustments). All three are **synthetic API cases**; no actual Borg caller supplying these extremes was found.

### 3. Medium — evidence gap: compression bypasses `--expect-impl rust`

**Locations:** `harness/diff:190`–`:203` (`check_done`), `:216`–`:244` (`check_manifest`). Reproduced: copy the fresh **C** smoke output, `xz` its three `keys.log` files, keep the C manifest, and `harness/diff --suite smoke --expect-impl rust /tmp/compressed-c harness/reference/smoke`: both operands are classified as references, the first prints `impl=c`, result **3/3 identical, exit 0**, with no Rust output involved. `check_manifest` identifies a reference solely by the absence of plain `keys.log`, and its compressed branch returns before checking `expect_impl`, overrides or build checks. **Suggested fix:** distinguish operand role from compression; keep output checks when a current suite manifest exists; recognise historical references by an explicit marker or location; with `--expect-impl`, require a verified output of that implementation. A **tooling counterexample**, not gameplay divergence.

### 4. Medium — acknowledged evidence gap: no replayable endgame/state oracle

Depth-100 Sea of Runes and Morgoth teleport scoring, depth-99 stockpiling, depth-50 swaps, self-lunal thinking are not covered; stored references stop at depth 26; the temporary cross-check tooling is not committed; keys do not observe every state mutation. The corrected scope statement is appropriate, so this does not refute the scoped claim. **Suggested fix:** endgame saves/state fixtures, native-C differential drivers, state/note assertions; keep narrow wording.

### 5. Low — inaccuracy/minor: stale current-summary wording

`docs/STATUS.md:9` presents 270 unsafe in the milestone summary (this commit: 269); `borg-rs/src/borg_item.rs:71` says a note is an offset into *its* description; `docs/PROGRESS.md:198` (and REVIEW's FFI discussion) point to SELF-REVIEW section 5 for the wrapper audit, which is section 6.

## Reproduced validation (summary)

Fresh C smoke 3/3; Rust smoke/full/deep/classes 3/3, 45/45, 9/9, 16/16; `check-no-c-borg.sh` pass (154/154 engine objects, 31 staticlib members); note differential 11 lines identical, an expanded one 17 lines identical; nested regression 7 entries; tests 96 passed on Rust 1.97.0/1.98.1/1.99.1, clippy clean on each; unsafe 269; native-C/Rust flag differential 2,278,000 comparisons identical; extracted root under Miri Stacked/Tree Borrows passes. Extra configuration evidence: `borg_munchkin_start = TRUE` and `borg_lunal_mode = TRUE` (Human Warrior seed 42, Half-Elf Mage 12345, Dunadan Blackguard 1080, menu letters, cheat death, 100k/100k/20k turns): C twice identical and Rust vs C identical, 3/3 each; the lunal flag does not by itself reach `borg_think_dungeon_lunal` (self-lunal gate at `borg_think_dungeon.rs:1438`).

## Panics where C has undefined behavior

No normal-play native-C out-of-bounds execution with a corresponding Rust panic was reproduced. Conditional, source-traced candidates: `borg_messages.rs:459` / `engine.rs:2940` (#198, fuzz-only); `borg_messages.rs:1102` (failed glyph at capacity/underflow); `borg_fight_attack.rs:2229`, `borg_flow_kill.rs:2029` (real missile evaluation with `b_k = -1`); `borg_danger.rs:2055`, `borg_fight_attack.rs:229` (teleport list at capacity); `borg_item_use.rs:93` (failure divisor zero). A safe port should not recreate C UB as UB; checked failure is a safe departure, and compatibility claims need that qualification.

## Documentation accuracy and limits

Current suite counts, coverage, 96 tests, nested entries, the note regression, 59 conversions and unsafe confinement check out; all 68 unsafe-history rows recomputed with zero mismatches; all 89 backticked hashes resolve. Not done: regenerating all C references, whole-engine sanitizer/memory-model runs, certifying every kept bug, an endgame/all-state oracle.
