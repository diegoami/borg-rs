# Rules for work on borg-rs

Read `docs/REVIEW.md` first. In short: this is a faithful Rust port of the
Angband C Borg. The Rust Borg must produce the same keypress stream as the C
Borg. Known C bugs are kept on purpose and listed in `docs/c-bugs.md`.

## C bug leads are filed in the tracker

Every C bug found here is also filed as an issue in
[diegoami/neo-angband-tracker](https://github.com/diegoami/neo-angband-tracker),
following that repository's conventions. This applies even to bugs that are
only leads, found by reading the code and never seen in a game.

* **When:** after the module commit that adds the entry to `docs/c-bugs.md`
  is merged and pushed to the work branch, so the discovery link points at a
  published commit. Conversion agents record bugs in `docs/c-bugs.md` only;
  whoever merges files the issues.
* **One issue per distinct slip**, not one per `c-bugs.md` section. This
  matches the tracker's one-finding-per-defect granularity.
* **No duplicates.** First check the tracker's findings
  (`docs/c-bugs/findings/*.json`) and its open and closed issues with the
  `upstream-c` label for the same C function and lines. If the slip is
  already filed, comment on that issue instead of opening a new one.
* **Form:** the tracker's `c-finding` template.
  * Labels: `upstream-c`, `finding`, `candidate`, `evidence:reported-lead`,
    `source:borg-rs`. Priority labels are the tracker's triage; leave them off.
  * Title: the human consequence in one line. Do not invent a `CBUG-NNNN`
    ID: the tracker's catalogue tool assigns them.
  * Body, in this order:
    1. `**Threads**: none yet`
    2. `**Finding**:` "not yet assigned (borg-rs lead)"
    3. `**Evidence level**: reported-lead`, with what was and was not checked
       (code reading, cross-check, observed in a suite run)
    4. a one-line summary
    5. the `c-bugs.md` text quoted
    6. `C context:` a link to the C lines at the pinned upstream commit
       `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
    7. `Discovery:` a link to the `docs/c-bugs.md` lines at the pushed
       borg-rs commit
    8. the consumer note: the C source state belongs on the issue; borg-rs
       keeps the bug faithfully and does not translate C fixes during M3
* **Record the link back.** Add a `Tracker:` line with the issue reference(s)
  (`diegoami/neo-angband-tracker#N`) to the `c-bugs.md` section.
* **Never** open issues, PRs or comments on `angband/angband`.
