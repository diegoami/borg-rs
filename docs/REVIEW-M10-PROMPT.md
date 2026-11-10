# Milestone 10 (C consumer) adoption — independent review prompt

Launch with any model you choose, e.g.:

    opencode -m <provider/model>

Paste the prompt below as the opening message.

---

You are the **independent reviewer** for the Rust side of diegoami/borg-rs. You
are reviewing already-merged work on `main`; you are **not** implementing.
Post your verdict on **diegoami/borg-rs#24** and nowhere else: no PRs, no
code changes, no tag placement, no issue closing, nothing posted to
angband/angband or to better-borg-c.

## Orientation (read first)

1. `CLAUDE.md` at the repo root, then `docs/C-CONSUMER-ADOPTION.md` (the
   adoption contract), `docs/STATUS.md`, `docs/FORK-CORRECTIONS.md`,
   `docs/REVIEW.md`.
2. `docs/LOCAL-SESSION-HANDOVER.md`, the top section, "C milestone10 adoption
   (current)" — the author's handover, including the two points it asks a
   reviewer to check first.
3. The adoption record: `docs/adoption/c-m10/` — `README.md` (the
   reconciliation), `series.json` (the two improvement units and the crash
   correction, with the consumer's merge commits and evidence),
   `ledger.json` (the Rust disposition per unit), `patches/` (the consumer's
   patch files), `removals/` (the five per-unit removal patches).
4. The validation receipt: `docs/validation/2026-11-10-c-m10/` — `README.md`,
   `matrix.json` (per-case digests), `native-comparison.json` (before/after
   fixture rows), `fault-controls.json`, `current-m10-c-summary.json`,
   `native-logs/`, and the two BUILD_IDs.
5. `docs/adoption/status.json` — the batch's machine-readable state
   (`active_batch` is `c-m10`, `state: author_validated_review_pending`).

## What is under review

C milestone 10 of the better-borg-c consumer: target
`6944d88e87d02e9d24e85438d25f0c6ef864c170`, tag `4.2.6-impact-v10` there.
The C side of the consumer was already reviewed upstream
(better-borg-c#420, "approve with corrections" M10-R-01..07, applied in the
snapshot commit, re-reviewed approve at `46057bc4a`). **This review covers
the Rust side of that carry**, not the consumer's own work.

Milestone 10 carries **no new bug ordinal**. Its units are:

| Unit | Nature | Consumer |
|---|---|---|
| **I-001** | owner-directed **local improvement unit** (never proposed upstream): a lone adjacent known monster that one blow would kill no longer justifies a strategic retreat | PR 394, issue #324 |
| **I-002** | owner-directed **local improvement unit** (never proposed upstream): `borg_caution()` tends wounds/poison/cuts while fleeing when the grid is survivable this round | PR 414, issue #408 |
| **0099 correction** | crash correction attached to the **already-carried** 0099 ledger entry | PR 398, fix `dbc2bed77` |

The Rust implementation commits are `4f7152d` (the carry), `8a135f1` (the
removal units and the adoption record), `ef8b063` (status), `2890410` (the
validation receipt), merged as `44a96af` "Impact v10", then `9674ca0` and
`b8cbbe1`. The comparison base is the owner-accepted milestone 9 commit
`8a7579d` (tag `4.2.6-impact-v9`).

Because I-001 and I-002 **change Borg behaviour** rather than fix a C bug,
the adoption contract's rule that an approved bug-fix milestone does not
authorize unrelated policy redesign needs a judgement from you: the author's
reasoning is that the consumer merged these units into its own product, so
not carrying them would leave the Rust Borg behind the accepted C target,
and the Rust Borg's contract is to produce the same keypress stream as the
consumer's C Borg. **Judge that reasoning on its merits.**

## What to verify (spot-check; distinguish what you ran from what you read)

1. **C/Rust fidelity of I-001.** Compare the consumer's
   `src/borg/borg-caution.c` at `6944d88e8` (new
   `borg_adjacent_kill_in_one_blow()` and the strategic-retreat guard) with
   `borg-rs/src/borg_caution.rs`, and the consumer's
   `src/borg/borg-fight-attack.{c,h}` with
   `borg-rs/src/borg_fight_attack.rs`. Check the loop bounds, the
   `r_idx`/`z_info->r_max - 1` comparison (note the C's signed `int`
   promotion versus the Rust's handling of a degenerate `r_max == 0`), the
   `borg_timer(kill->when) > 2` staleness test, the `distance(...) != 1`
   test, and the `borg_thrust_damage_one(i) >= kill->power` comparison.
2. **The `retreat` flag in `may_move_away()`.** This is the author's
   highest-risk translation decision. The C applies I-001's guard **only** to
   the strategic-retreat condition; the C back-away condition, which shares
   the other guard clauses, does **not** carry it. Rust factors both into one
   helper, `may_move_away()`, which now takes a `retreat: bool`. Verify
   from the source that the retreat call site passes `true`, the back-away
   call site passes `false`, and that neither can be reached with the wrong
   value. **If the flag were dropped or mis-set, the change would silently
   widen to the back-away path** — a divergence from the accepted C target.
3. **C/Rust fidelity of I-002.** The consumer's branch is
   `} else if (pos_danger < borg.trait[BI_CURHP]) {` placed after the
   existing `!borg.goal.fleeing` heal/defend block, calling the same
   `borg_heal(pos_danger)`. Check the Rust branch's position, its
   condition, and that `borg_heal()`'s own danger gates still apply (that
   is, that the new branch does not bypass them).
4. **The 0099 correction.** The claim is that borg-rs already carried the
   identical `square_in_bounds(cave, borg.c)` guard from milestone 9, so the
   C side needed a comment sync only and the Rust side needed no change.
   Verify against `docs/adoption/c-m9/ledger.json` (the `0099-guard` entry)
   and `docs/FORK-CORRECTIONS.md`, and confirm `borg-rs/src/borg_magic.rs`
   already guards the lit-square test. Confirm the C change really is
   comment-only in effect (the guard's behaviour is unchanged).
5. **C blob equality.** `borg-fight-attack.h` and `borg-magic.c` are claimed
   byte-identical to the consumer's blobs at `6944d88e8`; `borg-fight-attack.c`
   differs only by the retained local #322; `borg-caution.c` differs only
   outside the m10 hunks. Check with `git hash-object` against the consumer
   tree.
6. **The native fixtures.** `harness/adoption/m10/` imports the consumer's
   `necrolight-296-crash` and `fleeheal-408` kits at the milestone10 pin.
   Verify the imported bytes are the consumer's committed files, and that
   the I-002 fail-before/pass-after story in `native-comparison.json` is
   what the logs actually show (`fleeheal flee-low`: before
   `heal=0 rest=1`, after `heal=1 rest=0`; the three controls byte-identical
   across the builds). The receipt states the fixture kits' **baseline logs
   were not re-derived** here — judge whether that is adequate.
7. **The removal units.** Five units in `docs/adoption/c-m10/removals/`.
   Verify the three claims in `docs/adoption/c-m10/README.md`: each
   reverse-applies on its own on the final tree; applying all of them
   restores `8a7579d` exactly for `angband/src` and `borg-rs/src`; and the
   implementation diff's hunks are partitioned across the units with none in
   two units and none left out.
8. **The parity result.** `docs/validation/2026-11-10-c-m10/matrix.json`
   claims 131/131 cases with full C/Rust parity and `harness/diff` exit 0.
   Check the matrix for internal consistency (that the recorded digests
   really correspond per case), and **independently re-run a meaningful
   subset** rather than only reading the receipt. The plan is
   `harness/out/c-m2/adoption-suite-final`; `harness/adoption/m3/matrix.py`
   is the comparator. Heavy runs should take
   `flock /tmp/angband-validation.lock ...` and must not contend with the
   owner's other lanes.
9. **The fault controls.** `fault-controls.json` claims I-001 13/23 and
   I-002 14/23 cases differ from the corrected C run. Verify the method
   makes sense (one Rust removal unit reverse-applied in a separate
   worktree, rebuilt, then the 23-case fault suite run) and spot-check one
   unit by rebuilding and rerunning it yourself.
10. **I-001's missing native fixture.** The consumer declares the absence
    (its guard never fires in any of its fixtures; its own matrix shows zero
    row changes), and the receipt calls I-001 "the thinnest part of the
    batch". Judge whether the evidence actually offered for I-001 is
    adequate, and whether the record is honest about it.
11. **The behaviour-change claim.** 94/131 cases differ from the milestone 9
    run, and deaths go 13 → 14. Confirm the digests support that, and judge
    whether the receipt correctly frames this as an observation rather than a
    survival or progress claim.
12. **The disclosure in the receipt about the discarded first Rust suite
    pass** (the author rebuilt the Rust image while the suite was running;
    three cases were recorded `binary_changed` by the suite's own guard and
    the pass was rerun). Check that the rerun is genuinely independent of
    the discarded one and that the receipt says so plainly.

## Constraints

- **Read-only on the repo**: no commits, no pushes, no tag placement, no
  issue closing, no worktree removal, no branch deletion. Review scratch
  under `/tmp/opencode/` only.
- Do not weaken any assertion or re-run a gate to green. If a check you run
  contradicts the record, **that is a finding**, not something to fix.
- Keep the source comments, docs and the existing receipts untouched
  (upstream-review hygiene). Do not "helpfully" amend
  `docs/adoption/c-m10/` or the receipt.
- The `4.2.6-impact-v10` tag is the owner's decision. Do not place it, and
  do not propose placing it in the repo. (Precedent for v8/v9: after owner
  acceptance the tag goes on the **runtime snapshot**, not on the package
  commit.)
- Do not use the owner's `.angband-rust` save/settings, display or playable
  build. Large frozen bundles stay in local `harness/out`; Git exports are
  not standalone replay bundles.
- Record unavailable evidence honestly: if you could not run something,
  say so and say why, rather than taking the receipt's word for it.

## Deliverable

Post **one** comment on **diegoami/borg-rs#24** containing:

- **Verdict**: `pass` / `qualified pass` / `fail` / `insufficient evidence`,
  with **acceptance blockers** named explicitly.
- **Numbered findings** `M10-R-01`, `M10-R-02`, …, each with:
  - severity (`blocker` / `major` / `minor` / `note` / `observation`),
  - the **exact file and location** reviewed,
  - what is wrong or worth recording, and
  - the exact next action if a correction is needed.
- **What you spot-checked versus what you took on trust** — an explicit list.
- **Checks you executed** (with the commands) versus **receipts you merely
  inspected**.
- **Limits**: what remains unverified, and anything you could not run.

The author will respond to each finding on issue #24 and will explicitly
verify any claimed fix, or mark it unverified. Owner acceptance remains a
separate decision after the review.

**Nothing in this batch is claimed as accepted.** No tag is placed, and this
review does not authorize one.
