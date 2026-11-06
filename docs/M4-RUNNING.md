# M4 C/D working record, 2026-11-06

This is WIP evidence, not a completed milestone or a replacement for the
checkpoint rules in HANDOVER. Main stays at `af523ebd715b`; no tags made.

## C: extended seeds and turns

`harness/suites/extended` is unchanged from the run plan. It has all nine
actual classes on seeds 4000-4008 at 2M turns each, plus Human Warrior
2000/2007 and Dunadan Blackguard 3100 at 5M turns each, all `menu,cheat`.
Three isolated passes run with two jobs each, using the original A+B
binaries. No references have been accepted for this suite.

The Warrior's completed scenario reaches max depth 34 and Paladin 31.
Seed 4006 (`ext-rogue`) fails in **both** C passes with exit 139 and in
Rust with exit 134, so no `DONE`/passing suite is possible. The runners
continue collecting the other scenarios and will mark the suite `FAILED`.
Do not use `--allow-incomplete` to accept it. A separate corrected-Rust
replay confirms the detection-array bounds panic and backtrace, documented
in C bug 53. No native C crash stack has been captured.

In this environment the frozen checkout and outputs are at
`/workspace/scratch/a48f4cbdbac3/borg-rs`. The active branch is checked out
separately at `/tmp/borg-d-savefiles`, so source changes cannot invalidate
the binaries currently driving the extended passes.

Once the full scenario set can complete under the fidelity rules, require
**all** three outputs to have `DONE`, then from the frozen checkout:

```sh
harness/diff --suite extended --expect-impl c harness/out/c/extended-1 harness/out/c/extended-2
harness/diff --suite extended --expect-impl rust harness/out/rust/extended harness/out/c/extended-1
```

Logs beside these outputs: `harness/out/extended-c1.log`,
`extended-c2.log`, `extended-rust.log`. Do not trust a `.partial.*` output.
The initial Rust pass predates the NULL-note correction: if it differs,
retain it as fail-before evidence and rerun Rust with the correction;
never substitute matching fragments or alter scenarios.

## D: saved-character endgame replay

The five synthetic saves and recipes are in `harness/saves/`; fixture
generation was done twice with C, matching the entire saved bytes, logs
and summaries (apart from elapsed time). Generation identity and file
hashes are committed. The native C Borg is untouched.

Both C endgame passes are complete and `harness/diff --suite endgame
--expect-impl c` reports 5/5 identical in keys, messages and state. One
pass was compressed into the new `harness/reference/endgame`, which
rechecks 5/5 against the second C pass. Existing references are unchanged.
Initial Rust keys and state match 5/5; messages only 2/5. Two differ by
`(null)` versus empty text, fixed in Rust to follow `z-form.c`'s formatter.
The remaining case is C bug 52, reported as
`diegoami/neo-angband-tracker#287`: C parses stale stack bytes after a
multibyte screen conversion without NUL termination. No C fix, stack-byte
emulation, message filtering or seed/fixture change is applied.

The corrected Rust replay is `harness/out/rust/endgame-fixed` in the active
worktree, with `endgame-fixed.log` and `endgame-fixed.diff` alongside it.
It completed: 4/5 runs match all three logs; 5/5 match keys and player
state. Only depth98 messages differ. The diff correctly exits 1.
An endgame diff must still **fail** while bug 52 differs. Do not call it a
completed checkpoint merely because keys are equal.

The initial depth50 replay reached max depth 69; the other fixtures start
at 98-100. Near-Morgoth logs include attacks received and melee hits on
Morgoth. They also report "Not ready for Morgoth now" and leaving: these
fixtures do not prove Sea of Runes or a winning strategy. No self-lunal
fixture or additional class/equipment set has been tested yet.

## Checks and next actions

* Before starting C/D, both builds and both smoke suites passed 3/3 in
  all three logs; `check-no-c-borg.sh` passed.
* Saved-character regression: C/C and C/Rust 2/2 identical in all logs;
  missing-file, wrong-race and wrong-hash controls passed.
* Native multibyte-conversion probe passes, showing an ASCII control and
  the missing NUL for UTF-8 without making undefined reads itself.
* Rust tests after the NULL-note correction: 100 passed, 1 ignored
  (existing ignored test). Clippy with `-D warnings` passes.
* Post-correction smoke: 3/3 identical in all logs. The saved-character
  regression passes again, including all three rejection controls.
* The pre-correction D build passed smoke 3/3 and full 45/45 against
  existing references. Its next suite correctly refused the now-stale
  build after the Rust source correction; no override was used. This
  is **not** the post-correction full checkpoint.
* Preserve both strict failures. Collect the remaining extended runs,
  retain `FAILED`, and continue C-side investigation through the tracker
  without correcting the C oracle or inventing undefined bytes in Rust.
  A fresh final-source checkpoint is required before either milestone
  can be declared done. M4 C/D remain incomplete; main must not move.

The CLI checkout has no push credential. WIP commits were published to
the requested branch through the connected GitHub API; no upstream
Angband issue, PR, comment or tag was created.
