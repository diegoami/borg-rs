# Rules for work on borg-rs

Owner-approved current direction (2026-11-07): M5 is owner-accepted and
annotated tag `m5` points to `0203227`, after independent pass with no open
findings for candidate `28468aa` plus that follow-up. Review#2 is closed.
Read [docs/M5-PLAN.md](docs/M5-PLAN.md),
[docs/C-CONSUMER-ADOPTION.md](docs/C-CONSUMER-ADOPTION.md) and
[review#2](https://github.com/diegoami/borg-rs/issues/2) first.

The owner selected better-borg-c milestone2 on2026-11-08 and requested
adoption issue [borg-rs#6](https://github.com/diegoami/borg-rs/issues/6).
Target2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f contains seven fixes,
independently approved in C review38/comment6705894080. Read
[adoption/c-m2](docs/adoption/c-m2/README.md) and adoption/status.json for
actual implementation/validation state. That M2 batch included no later C milestone.
Owner then directed C milestone3 (2026-11-08): target702f091f6, tracking
[borg-rs#8](https://github.com/diegoami/borg-rs/issues/8), record
[adoption/c-m3](docs/adoption/c-m3/README.md). It retires the local #290
equality reset in favour of the consumer's patch0018 guard; #287/#322 cover
patches0025/0027. Independent qualified pass (issue8); on the owner's
instruction ("if accepted, create the milestone and the tag") it is recorded
as impact milestone 3 in docs/STATUS.md with annotated tag `4.2.6-impact-v3`,
following `4.2.6-impact-v2` for the milestone2 carry.
Owner then directed C milestone4 (target 70993393a, `4.2.6-impact-v4`),
tracking [borg-rs#11](https://github.com/diegoami/borg-rs/issues/11), record
[adoption/c-m4](docs/adoption/c-m4/README.md). The engine Remove Curse crash
it exposed (C and Rust alike) is root-caused to the wizard cure-all and fixed
narrowly in the engine (FORK-CORRECTIONS; neo-angband-tracker#474).
Independent review: qualified pass (issue11). Owner accepted it as impact
milestone 4 (tag `4.2.6-impact-v4`) after harness/diff was allowed to accept
a mid-command Borg self-stop.
Owner then directed C milestone5 (target 5384f12ed, `4.2.6-impact-v5`),
tracking [borg-rs#13](https://github.com/diegoami/borg-rs/issues/13), record
[adoption/c-m5](docs/adoption/c-m5/README.md). Independent qualified pass
(issue13); owner accepted it as impact milestone 5 (tag `4.2.6-impact-v5`).
Owner then directed C milestone6 (target b3c825c97, `4.2.6-impact-v6`),
tracking [borg-rs#15](https://github.com/diegoami/borg-rs/issues/15), record
[adoption/c-m6](docs/adoption/c-m6/README.md). Independent qualified pass
(issue15); owner accepted it as impact milestone 6 (tag `4.2.6-impact-v6`).
Exact accepted C milestone pins now
drive Rust adoption. Existing neo-angband-tracker links stay source/history
provenance; its queue does not independently schedule post-M5 work. The
adoption contract supersedes older routing/fidelity wording for the new lane;
preserve old reports and references.


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
The rules below preserve existing report provenance. M5 review#2 is complete; post-M5 findings/adoption follow
C-CONSUMER-ADOPTION.md and the
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


Updated death-analysis workflow: shared owner feedback hub is borg-rs#3;
D0005 focused investigation is#4. Use docs/deaths/REVIEW-TEMPLATE.md for the
whole trajectory, native/player-known resources, competing explanations and
ranked next evidence. Missing private gates or rescue evidence stays provisional;
source reasoning is not runtime branch proof. Player-known inventory/spells
and diagnostic hidden fields must remain distinct. Historical replay extensions
add only declared read-only observations after all original channels/saves match;
never rewrite original capture/identity or silently use newer game algorithms.
