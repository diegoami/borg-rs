# Author preparation review for PR #83

Code candidate: `dd3eeac646e097bcdd7c289c092e91400896a713`.
Review base: frozen combined M12 `d9a56f48bac8675c91f551619492af36b698b4cf`.
The review-package changes after dd3eeac are documentation and diagnostic
artifacts only; they do not fix the findings below.

This is an **author preparation review**, not independent signoff. The owner
requested a branch review, a PR and a prompt. [Draft PR #83](https://github.com/diegoami/borg-rs/pull/83)
provides the implementation delta; #42 remains the never-merged architecture
vehicle. Neither is authorised for merging into the frozen experiment.

## Findings

### R1 — P2: combat value is assigned without a tactical consumer

Status: open, tracked by #81, with related tactical work in #79/#80.

`borg_town_capability.rs:152` maps `TMD_ATT_CONF` to combat buff supply;
`:165` uses the effect magnitude as units. The Monster Confusion kind has an
arming timer of 10000, while `angband/src/player-attack.c:669` clears the armed
status when applying the melee side effect. It is not 10000 turns of effective
confusion. The known-scroll tactical consumer is absent.

Local original c91fc36 traces showed one purchase per character across nine
classes, no Home deposits or logged reads, and one scroll in each last town
pack snapshot. Last snapshots are not terminal-save inventory assertions.
The current branch retains this mapping and lacks the consumer. These traces
are earlier-runtime observations, not new dd3eeac gameplay.

This can make known unused stock look worth carrying/storing, independently
of its legitimate first-copy identification bonus. The catalogue mapping gate
cannot establish tactical usability. Merely filing #81 does not resolve it.
The independent reviewer should assess its blocking status for the requested
review scope; it blocks a claim of complete usable combat valuation.

### R2 — P2: throwing and town allocation can use different trip forecasts

Status: reproduced by a temporary pure-policy test; no production fix applied.

Town evaluation materialises the automatic trip policy target through
`borg_town_shadow.rs:336` and builds its Journey from `forecast_state` at
`:430`. The throwing guard instead calls `Journey::of(st)` on the raw config
at `borg_fight_attack.rs:2431`. With an automatic town target, that raw Journey
uses maximum depth plus the town depth-gain setting, not the trip-policy target.

Counterexample using the real `plan_trip`, `effective_params` and `Journey::of`
functions: maximum depth 1, prepared target 12, default exploration/stair rates,
automatic target/duration, both policies enabled, 5000 lantern fuel and a
7500-fuel oil flask. The town forecast is 12200 actions and reserves one bottle;
the throwing forecast is 1200 and reserves none. The guard can therefore release
oil that the town model reserved for fuel.

[Probe patch](fuel-forecast-probe.patch) · [observed failing test](fuel-forecast-probe.log).
The patch adds only a test in a disposable checkout; it was removed before
committing the review package. This is a policy-function counterexample, not a
native survival demonstration. The minimal remedy is to use one shared effective
forecast for valuation and the fuel reserve, preserving off-policy controls and
historical replay. Check both deeper and shallower trip targets, explicit
configuration, remaining light, multiple bottles and the unique-fight exception.

### R3 — P2: historical executor documentation looked current

Status: clarified in this documentation package; code gap remains disclosed.

`docs/TOWN-LIVE.md` described the earlier town-only ac63c5e/schema-6 executor:
urgent Home cures before paid cures, an unchanged dungeon depth policy and a
v11R base. The current code is the combined M12 policy. `Stock::build` excludes
Home from urgent shop candidates and `first_action_with_room` returns urgent
shop BUY before ordinary Home dispositions. The old Home-source claim must not
be used as evidence for this implementation.

A historical banner now identifies the source and points to current docs.
No urgent Home-source support or new runtime behaviour was added by that edit.

## Checks actually rerun on the unmodified code candidate

- [271 Rust release tests pass](rust-tests.log), four opt-in tests ignored.
- [24 town-analysis tests pass](town-python-tests.log).
- [Six trip-report tests pass](trip-python-tests.log).
- [Clippy passes](clippy.log) on the local toolchain.
- [9791 available historical records replay](historical-replay.log), plus
  [1522 earlier schema-7 records](schema7-replay.log). These are historical
  inputs, not newly generated schema-8 traces.
- The additional R2 probe **fails**, as expected for the finding. It is separate
  from the clean candidate suite and is not counted as a passing check.

No native games were run during this preparation review. The source inputs
under angband, borg-rs, Cargo manifests and harness/measurement are identical
between tested integration source 1651f808219e and dd3eeac; later commits in
that range are documentation. That comparison supports using its author
receipt as a claimed receipt for the code, not as independent verification.

## Evidence limitations and readiness

The latest integration receipt claims 20 native attempts, 570 schema-8 records
and 275 historical schema-5 records, with raw data at
`/tmp/trip-integrated-validation`. That path was **not available on this
preparation host**. Those originals, native execution and current-schema
replay were not rerun here. The independent reviewer must report this limitation
or rebuild fresh qualified scenarios, including the unknown-food/mushroom rules
that the author receipt covers only through regressions.

The older capability cohort reports two deaths versus one baseline death on
nine Human starts per arm. Its 530 purchases are receipt/wallet verified and
its corrected oil total is 70. It establishes neither improved survival nor
outcomes for the newer learning-return code.

Author readiness: **changes requested before claiming complete usable live
valuation or promoting the policy**. Independent review remains pending. Green
unit/replay/coverage results do not resolve R1 or R2. This PR is a review vehicle,
not a main-integration or experiment-replacement approval. Main at preparation
was `727ccff75cfc731f953a296c14ef3071a57c28fb`; later carry work is separate.
