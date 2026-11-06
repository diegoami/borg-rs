# Response to the third review (10c7f00ba)

The review is `docs/REVIEW-10c7f00ba.md` (a condensed record). It found no
C/Rust divergence and no new undefined behaviour, confirmed the second
review's fixes, and reported one harness gap and stale guidance.

| # | finding | verdict | change |
|---|---------|---------|--------|
| 1 | `harness/diff --suite` checks run names, not the scenarios | **confirmed** (reproduced) | fixed |
| 2 | endgame and self-lunal gameplay unobserved | **confirmed** (acknowledged gap) | none yet; plan below |
| 3 | stale reviewer guidance | **confirmed** | fixed |

## 1. `--suite` checks the scenarios — fixed

**Reproduced:** a copy of `harness/suites/smoke` with +1000 on every seed
and turn limit, same run names, against the untouched Rust output and
reference: 3/3 identical, exit 0.

**Fix.** With `--suite`, `harness/diff` now checks, besides the run names:
- a suite output's manifest records this suite file (`suite_sha256` equals
  the file's sha256), which pins every parameter of every run;
- on both operands (references included), every run's `summary.json`
  shows the suite's seed, race and class indices and menu-letter flag; a
  run that stopped on turns stopped at or after the suite's limit, an
  early stop (death) before it;
- a config run's `config.txt` records the suite's flags; a suite run with
  config flags must have one.

Checked: the reviewer's scenario now fails (exit 1) on both sides (the
output's suite hash; seeds and turn limits on output and reference); a
changed config flag and a removed `menu` flag are caught, the latter even
comparing a reference with itself; all five suites still pass, C smoke
too. All 80 stored reference runs match their suite files.

Limits (in `harness/diff`'s help and `docs/harness.md`): the stock
references record neither the cheat-death flag nor an exact upper turn
bound (a run stops at the first key at or past the limit, up to 1,191 game
turns later in the stored references), so for references those two
parameters are bounded, not pinned; suite outputs are pinned by the hash.

## 3. Stale guidance — fixed

`docs/REVIEW.md`: depth 27 and the self-lunal thinker in the coverage
note; nesting happens in real play (two config runs) instead of "never";
268 = 240 + 28; the crate-private modules instead of "an external crate
could call them". `docs/harness.md`: which references keep a manifest.
`harness/diff`'s help: roles by manifest, `--expect-impl`, `--suite`.

## 2. Endgame and self-lunal — acknowledged; plan

Agreed. The reviewer's native-C fixtures (preparedness, formulas, respawn)
and runs (money scumming, dynamic formulas, debug commands) all found C
and Rust equal, which strengthens the evidence, but no endgame state or
self-lunal play has been executed. The next steps, in order of cost:
1. a notes/state log beside the key log in both builds (catches state
   differences that do not change keys);
2. more seeds and longer cheat-death runs to reach deeper levels;
3. a suite per remaining configuration (money scum, dynamic calcs with a
   start-up acknowledgement, respawn, worships, risky);
4. engine-backed endgame saves (depth 50+, 99, 100, Morgoth) loaded by both
   builds with the same seed;
5. a committed native-C differential checker (the original C Borg linked
   with renamed symbols), to replace the uncommitted conversion
   cross-checks.
