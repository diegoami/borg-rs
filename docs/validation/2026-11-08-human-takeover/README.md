# Human takeover from a recorded turn

Owner authorized playing an independent copy from a selected recorded turn,
then delegated D0006 analysis. This receipt covers the tooling; the D0006
assessment and qualified private control-flow trace are separate under its
death dossier and investigation#5. No gameplay algorithm or milestone tag
changes here. This is author validation, not independent review.

Commands: `replay-borg.py RECORDING --play-from-turn N`, or
`replay_next_death.py --case D0006 --play-from-turn N`; `--prepare-only` creates
without X11, `--output NEW_DIR` chooses a new directory. The printed
`play-branch.py DIRECTORY` command resumes the separate human save.

`validation.json` binds code hashes, historical sources, complete control
receipts, capture boundaries/native saves and raw local evidence. Full builds,
replay bundles and private saves remain under `harness/out/human-takeover` and
`harness/out/death-workflow/human-builds`, outside Git.

## Results

- D5 source9881301 and D6 sourceaa4ad6c rebuilt with only declared capture/
  manual-start hooks, original data and version stamp. Entire original raw
  frames, keys, messages, player-state logs, final native save, all scheduled
  saves and archive bodies match. The existing declared historical headless
  first-line archive clock exception remains explicit in full proofs.
- D5 captures36004(town),36094, request36095->36104 pass exact prefix checks,
  required message/checkpoint completeness, fork-private/terminal save equality
  and historical native-reader player/complex RNG checks. D6 capture22794
  is the down stair(33,49), HP9/SP3. Actual queue command request22795->22804.
  The death-turn36289 request refuses a usable branch.
- Six complete human GUI sessions: D5 encounter load/save/reload, D5 town
  manual wait/save/reload, D6 refuge load/save/reload. All start alive with
  Borg off, retain the character lock, have zero Borg actions, and rerun
  exactly through their frozen GUI input recorder. Human sessions preserve
  their initial state/complex RNG and carry original provenance. No new
  death-registry directory is created in these tests.
- SIGTERM at a live D6 human input boundary stops the owned GUI before the
  lock is available; incomplete recording is rejected. An isolated copy and
  private Xvfb are used throughout, never owner saves/settings/display.
- The ordinary six-case GUI suite passes in the disposable checkout with the
  updated shared recording tools: load/save/reload/restore, actual Rust Borg
  intervention with resize/preference reload, loaded Rogue, fresh birth.
  Exact save hashes from prior launcher receipts remain reproducible.
- Python controls:62 run,59 pass,3 existing fixture-dependent skips;12 GUI
  negative controls pass. Coverage includes corrupt/omitted original prefix
  streams, truncated messages, missing checkpoint, player/RNG/Borg mismatch,
  unsupported GUI origin, conflicting CLI modes, existing output protection,
  frozen-input mutation and signal cleanup. Python compilation and diff
  whitespace checks pass. Logs are retained here.

Native save/load compacts/rebuilds engine structures. These are serialized
counterfactual sessions, not uninterrupted future replay, private Borg-memory
restoration, successful rescues or independent lives. Full raw captures and
original dossiers remain unchanged. GUI-origin takeover is currently refused;
ordinary viewing remains available. Ctrl-X saves; Escape closes the native
exit screen (Return opens the score list).
