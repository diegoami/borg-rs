# Rules for work on borg-rs

Owner-approved current direction (2026-11-07): prepare the focused M5 review
at 28468aa, then follow verified better-borg-c bug-fixing milestones. Read
[docs/M5-PLAN.md](docs/M5-PLAN.md), [docs/C-CONSUMER-ADOPTION.md](docs/C-CONSUMER-ADOPTION.md)
and [review#2](https://github.com/diegoami/borg-rs/issues/2) first. M5 has an independent qualified pass at28468aa; author tooling follow-ups
await reviewer verification and owner acceptance; no C target is selected and no new tag is created.
After acceptance, exact accepted C milestone pins drive Rust adoption. Existing
neo-angband-tracker links stay source/history provenance; its queue does not
independently schedule post-M5 work. That adoption contract supersedes older
routing/fidelity wording for the new lane; preserve old reports and references.


Owner update (2026-11-06): targeted C/Rust corrections for crashes and
blatant play blockers are allowed; other conflicts are case by case.
See `docs/FORK-CORRECTIONS.md`, which supersedes earlier absolute fidelity
restrictions for those corrections. Broader divergence follows a milestone
and owner-created tag.
Read `docs/REVIEW.md` and `docs/STATUS.md`; `docs/M4-PLAN.md` records the earlier completed sequence. In short: this is a faithful Rust port of the
Angband C Borg. The Rust Borg must produce the same keypress stream as the C
Borg. Known C bugs are kept on purpose and listed in `docs/c-bugs.md`.

## Historical source-report routing (M1-M4)

The original lane filed every C bug as an issue in
[diegoami/neo-angband-tracker](https://github.com/diegoami/neo-angband-tracker),
following that repository's conventions, including code-reading leads.
The rules below preserve existing report provenance. Current M5 work coordinates
on issue#2; post-M5 findings/adoption follow C-CONSUMER-ADOPTION.md and the
C consumer's ownership/source links, rather than a second direct tracker queue.

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


## Death record workflow

Named GUI/headless launchers export completed natural deaths automatically.
Read docs/deaths/README.md and FEEDBACK.md when collecting or investigating
cases. Keep actual source/binary/input identity and curated original observations;
full raw bundles/saves/builds stay outside Git. Use tools/record-death.py and
aggregate-deaths.py for qualified manual exports; historical cases without
movies retain their weaker evidence. Keep assessment/actions/authorized feedback
durable, distinguish native facts from hypotheses, and never count reruns or
backup branches as independent lives. Source findings follow the active milestone policy above; existing canonical
links remain preserved. Logging alone never authorizes tactical changes
or automatic issue creation. Check linked owner feedback during active case
work; no unattended monitoring or deletion of unique evidence.
