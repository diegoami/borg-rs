# M4 checkpoint review on main

The owner resumed work after main integration and explicitly authorized
Codex to create and push the M4 milestone tag. The tag will identify this
review commit, before any subsequent glyph or other tactical correction.

Review findings corrected in this commit: REVIEW.md still described the
pre-M4 depth limit, missing tactic entry and old Claude branch as current;
the coverage tooling README still said full F measurement was open.
The current reviewer guide now distinguishes the completed finite corpus
from remaining strategy work and preserves the earlier review's limits.

Checks repeated on 2026-11-07:

* Main on GitHub and the local checkout both started at `47b1fbf`.
  Remote tags were m1-harness, m2-c2rust and m3; no M4 tag existed.
* Production inputs have no diff from `28b8170`. Current source hashes:
  C `3e9ac5743b149636107135dd323ca2f2f96799be73121abcf4812f31dccffa46`;
  Rust `8ac76b00ed9f96875777bb497655a7156ee1c08892bc52e37c8836594625ec8f`.
* All 874 artifact-hash entries checked successfully: coverage 514;
  synthetic tactics 285; native check ledgers 19 and 40; launcher 11;
  depth audit 5. The two native ledgers have different scopes; this is
  a count of validated entries, not distinct artifacts or executions.
* Strict coverage validation rechecked all 11 groups/139 profiles at
  `/tmp/borg-rs-m4-coverage/harness/out/coverage/complete-139`, including
  recipe/source/image identity, successful process/PID, exact profile sets
  and hashes. This validates retained runs, not newly executed games.
* Retained synthetic C/C, normal Rust/C and coverage Rust/C each pass
  6/6 with the current strict diff, including final engine-save bytes.
* The retained frozen-image crash-fixes group passes 6/6 against the
  corrected C reference, comparing keys, messages and state. The older
  ordinary melee receipt contains metadata and log hashes rather than
  complete raw logs; attempting diff directly there reports missing logs
  and is not accepted as a fresh parity pass. Its historical C/C result
  remains recorded in six-case-receipt.json.
* `git diff --check` and `bash -n run-rust-borg.sh` pass.

No production source changed for this review and no expensive game suite
was rerun. The M4 conclusion remains the declared A-F validation scope:
139 engine executions and the separately measured native checks. Complete
rune formation, useful unbounded self-lunal scumming and an unassisted win
remain outside that claim. The first agreed post-tag task is #295.
