# Review prompt: combined town inventory and trip policy

Review vehicle: [draft PR #83](https://github.com/diegoami/borg-rs/pull/83).
This is an implementation review against a frozen experimental baseline, not a
merge into that baseline. PR #42 remains the never-merged architecture/history
vehicle. No runtime fix is included in this review-preparation package.

Copy the following prompt into the independent review session.

---

You are the independent reviewer of diegoami/borg-rs PR #83. Review correctness,
policy consistency, observability and the evidence boundary. You are not the
implementation author; do not treat author receipts as your own verification.

## Pins and scope

- Runtime/code candidate: `dd3eeac646e097bcdd7c289c092e91400896a713`.
  This includes merged PR #78 and plan schema 8, not just the earlier schema-7
  capability repair at `c91fc36` or the earlier town-only branch at `ac63c5e`.
- PR base: `experiment/joint-policy-m12`, frozen at
  `d9a56f48bac8675c91f551619492af36b698b4cf`.
- Wider town/trip branch context starts from M12 merge-base
  `e6d13fc9baff0ef7f06de6333fbda436c961e0bf`.
- Main at preparation time: `727ccff75cfc731f953a296c14ef3071a57c28fb`
  (M13 accepted). Neither this PR nor its tests establish integration with
  that later main or any later milestone carry.
- The PR head may add review documents after the code candidate. Record its
  exact SHA and verify that its runtime inputs are unchanged from dd3eeac.
  If code has advanced again, stop using these receipts as evidence for the
  new code and report the new head to the owner before changing review scope.

Use a new isolated worktree, never another session's worktree or playable build:

```sh
git fetch origin feature/town-inventory-strategy experiment/joint-policy-m12
git worktree add --detach /tmp/borg-town-pr83-review dd3eeac646e097bcdd7c289c092e91400896a713
cd /tmp/borg-town-pr83-review
git diff d9a56f48bac8675c91f551619492af36b698b4cf..HEAD -- borg-rs/src angband/src/borg harness .github tools
git diff e6d13fc9baff0ef7f06de6333fbda436c961e0bf..HEAD -- borg-rs/src/borg_town* borg-rs/src/borg_trip.rs
```

Use a different fresh path if that worktree already exists. Review the complete
planner/solver/executor context: the PR base already contains substantial
implementation, so reviewing added lines alone is insufficient.

## Read first

1. `CLAUDE.md` at the code candidate. Historical M4/M5 reviews in `docs/REVIEW.md`
   are background, not a request to reopen their tasks or comment on #1/#2.
2. `docs/TOWN-CAPABILITIES.md`, `docs/TRIP-POLICY.md`,
   `docs/TOWN-INVENTORY-PLAN.md` and the dated decisions linked from #36/#35.
   Check source when older documents describe superseded units or policies.
3. `docs/validation/2026-11-15-town-review-preparation/README.md` from the PR's
   documentation package. This is the author's preparation review, not an
   independent verdict. Its open findings are leads to verify, not a complete
   list or an instruction to approve everything else.
4. Capability receipt: `docs/validation/2026-11-15-town-capabilities/README.md`.
   Learning integration receipt:
   `docs/validation/2026-11-15-trip-inventory-integration/README.md`.
5. Open #79, #80 and #81. These are follow-ups, not implemented fixes or grounds
   to omit their defects from this review's verdict.

## Review questions

### 1. Does valued supply have an actual usable meaning?

Inspect `borg_town_capability.rs`, the tactical consumers, and the native effect
semantics. Check charges, rod readiness/cooldowns, nutrition, fuel, matching
launchers, owned/usable class books, mana and secondary cures. Distinguish:
physical mapping; implemented/enabled consumer; conditional future use; and
confirmed actual consumption. A positive mapping or a hypothetical known-item
audit is not evidence of use.

In particular, independently assess the Monster Confusion issue: there is no
known-scroll tactical consumer, and its 10000 arming timer is consumed by a
melee side effect rather than providing 10000 turns of benefit. Check other
buffs/statuses for the same error. Blessing/Holy Chant already have narrow
consumers; do not misreport them as wholly absent.

Identification is a separate first-copy benefit. Unknown-item and just-bought
protections must not manufacture combat value or let useless reserves grow
without a valid reason. Do not require an enemy to be visible in town as a
substitute for forecasting future usable encounters.

### 2. Do town valuation and dungeon execution share the same trip?

Trace the automatic trip target, `effective_params`, `Journey::of`, and the
throwing fuel guard. Verify the provided fuel-forecast counterexample rather
than assuming its author diagnosis is complete. Check shallow walking versus
Recall, ambition growth, class preparation and explicit duration overrides.
Fuel bottles cannot be both burnt and thrown. A guard must not release fuel
that the same plan reserved for the journey.

### 3. Are budget, capacity and optimisation claims true?

Read `borg_town_budget.rs`, `borg_town_solve.rs`, `borg_town_record.rs`,
`borg_town_utility.rs` and `borg_town_dispose.rs`, including code inherited
from the base. Check shared shortages, non-additive effects, cash opportunity
costs, group merges, pure/mixed options, bounds, scaling and every feasibility
flag. A certificate for generated options is not a proof of the globally best
kit or of survival utility.

Exercise full pack/Home, negative remaining capacity, no-selling, zero/low
purse, stack joins after dispositions, only urgent cures, unavailable stock,
free Home supplies, unknown flavours/runes and sale rejection. Home storage is
not identification. Current urgent purchases consider shop candidates before
ordinary Home withdrawals; do not import the historical town-only document's
claim of urgent Home-source priority into the current runtime.

### 4. Does learning-return state respect knowledge and progress?

Inspect `borg_trip.rs`, `borg_item_id.rs`, `engine::object_learning_key`,
`borg_think.rs` and their tests. Audit all inventory/equipment/quiver types,
matching identical public descriptions, stack movement/merging and changed
runes/resources. Verify that unchanged unresolved backlogs alone stop causing
return loops, while real new opportunities can reopen them.

Check actual public knowledge: no hidden kind/rune leakage or fresh dungeon-
time shop inspection; cache prices/stock are estimates. Test no buyer, no
shops, no gold, no ID resources, a newly legal spell, stale observations,
full Home, and save/restart behaviour. Process-local cache loss is documented;
assess whether its consequences are acceptable. Check default-off observation
noninterference and both live switches independently.

### 5. Are probes, FFI, records and live transactions sound?

Audit try-on/try-off restoration, every touched state field, RNG observation
and read-only effect/quote helpers. Inspect the new dice FFI declarations and
loaded-data lifetime assumptions. Test that simulation never queues input.

Read live slot/quantity binding and action refresh on store arrival and after
each transaction. Check native prices, stale inventory indices, rejected sales,
Home withdrawals intended for sale, urgent precedence, stay/leave guards and
loop termination. Confirm completed transactions in native messages and wallet
changes; queued commands and plans are not receipts.

Validate plan schemas 4–8, append-only capability IDs, replay validation and
trip records v2/v3. Do not silently reinterpret historical records using the
new model. Check that new effective inputs and flags are materialised for
engine-free replay.

## Verification

At the unmodified candidate, record commands, return codes and actual source /
compiler / binary identities:

```sh
cargo test --release --manifest-path borg-rs/Cargo.toml
cargo clippy --release --manifest-path borg-rs/Cargo.toml --all-targets -- -D warnings
python3 -m unittest discover -s tools/town-plan -p 'test_*.py'
python3 -m unittest discover -s tools -p 'test_trip_policy_report.py'
BORG_IMPL=rust harness/build.sh
bash harness/check-town-capabilities.sh /absolute/fresh/coverage-output
```

Never run cargo fmt. Always specify BORG_IMPL for builds and runs. Use a
separate CARGO_TARGET_DIR for another toolchain. Serialize heavy local runs
with `/tmp/angband-validation.lock`; use finite caps and fresh output paths.
Do not start a large hosted zoo study.

Rebuild controls separately and compare complete key/message/state streams.
Test logging on/off and disabled shops. Add bounded scenarios for the new
learning rules and the review findings, plus native cases that separate a
buying decision from successful consumption. Capture plan/trip records under
unique per-process paths and replay them:

```sh
PLAN_RECORD_FILE=/absolute/run/plan.jsonl cargo test --release --manifest-path borg-rs/Cargo.toml -- --ignored --nocapture a_run_s_records_all_replay
```

The provided fuel probe patch adds only a temporary test to a disposable copy.
It is expected to FAIL at dd3eeac: the throwing path gets (1200 turns, zero
reserved bottles), the effective town plan gets (12200 turns, one bottle).
It demonstrates a pure-policy mismatch, not a native survival counterfactual.
Restore the test file afterwards and keep that diagnostic separate from the
unmodified candidate's checks.

The author's /tmp/trip-integrated-validation source-8 raw files were not
available on the preparation host. Its committed receipts were inspected;
its claimed 570 source-8 records/native gameplay were not independently rerun
here. Report unavailable originals explicitly; create fresh qualified evidence
instead of treating a log of a prior test as your own execution.

## Evidence boundary and deliverable

- The capability comparison was nine Human starts per arm at c91fc36 versus
  d9a56f4: two deaths versus one, 530 receipt/wallet-verified retail purchases.
  It is not dd3eeac outcome evidence and does not establish improvement.
- The later integration receipt is targeted wiring evidence, chiefly Warrior
  starts. It is not a nine-class survival study or natural mushroom-sale proof.
- Native catalogue coverage means mapping, not complete tactical usability.
- Count failures and repeated attempts; do not pool repeats as extra lives.
  Assess every new natural death with original provenance and qualified traces.

Return severity-ordered findings with exact candidate paths/lines, reproducible
triggers, expected/actual behaviour and a minimal remedy or next discriminating
check. Separate confirmed defects, source leads and unsupported claims. State
which checks you executed, which receipts you inspected and what was unavailable.
Give an explicit verdict (pass / qualified pass / changes requested /
insufficient evidence) and distinguish experimental correctness, live promotion,
and main integration readiness.

Review only. No production fixes, rebases, merges, tags, issue closing or changes
to frozen zoo pins. Temporary diagnostic tests in your own disposable worktree
are allowed, with their modifications disclosed and restored. Preserve owner
saves, other sessions' worktrees and prior evidence. Return the report to the
owner; do not post GitHub comments without separate owner approval. Never post
to angband/angband. PR #42 and this frozen-baseline review PR are not to be merged.
