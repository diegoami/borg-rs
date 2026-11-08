# Borgh measurements for the Borg zoo

Owner requested harness-only measurement work on 2026-11-08, tracked on
[issue #7](https://github.com/diegoami/borg-rs/issues/7). Both backends implement
`-states FILE`, `-stall-decisions N`, `-exact-turns` and `-random-birth`, plus
live effective settings and the requested stop statistics/RNG state. The
[interface](../../harness.md#borg-zoo-measurements-2026-11-08) defines every
field, energy unit, stop boundary and birth-policy exception.

**Result: 165 cases per backend retain exact key/message/per-key-state parity
against the appropriate reference or pre-change image on this machine/load.**
The 131-case source-qualified M2 corpus uses a 900-second test wall; the
additional five suites use retained successful 900-second cases and fresh
7,200-second cases. These are game-turn/keypress oracle checks, not a portable
wall-time guarantee or an independent survival cohort. All 60 smoke/config
option executions pass; new C/Rust summaries and state streams agree.

## Source and evidence boundaries

The full game/option corpus ran the frozen implementation
**033209a0ff56cc6c9ddaa604c1f47f74a1b9e02a**. Its BUILD_ID stamps truthfully
name the precommit dirty checkout at 0bc7a09; their input hashes match the
committed 033209a tree. The pre-change Rust image is bound by both binary
hash and exact input-tree hash to **0bc7a09fb41a01cfbc4f5bb86b89002b074f3712**.
This baseline already carries the separate M2 adoption. Historical M4/M5
references, original deaths, Borg sources and borg-zoo remain untouched.

For 131 cases, fresh strict `harness/diff` comparisons pass against the
existing source-qualified M2 C oracle and between C/Rust, including all 131
full native saves. For the 34 cases outside that corpus, the five original
suite recipes run against the preserved pre-harness image and both new
backends. Their explicit attributed-join manifests verify implementation,
source/binary/config/input identity and every successful process. Strict
comparisons pass without `--allow-unverified`, `--allow-incomplete` or stale
build overrides. No older gameplay reference is rewritten to accommodate
pre-existing M2 changes.

The game executable follow-up **7ee493b** only supplies observation sinks to
unit-test executables. After rebuilding both backends, every production
object is byte-identical to the measured image except `buildid.c.o`; the
Rust static library is byte-identical. The sinks are not linked into the
game. [Object equivalence](object-equivalence.json) records the final build
identities. New smoke runs reproduce all three logs and full saves against
the M2 reference and C/Rust, and the focused option tests pass again.
Later review disposition **9db8833** changes evidence wording/status and an
audit diagnostic; it makes no game-code change. M2's independent qualified
pass at 0bc7a09 remains distinct from this author-run harness validation.

## Executed checks

- Both 131-case default sweeps finish `DONE`, with zero failed child
  processes. C/reference, Rust/reference and C/Rust each pass 131/131 strict
  comparisons. Every added summary field is also compared between backends.
- All 34 additional cases per backend match the preserved source-equivalent
  baseline in keys/messages/player state and existing summary fields; native
  final saves match wherever the recipes request them. C/Rust also match
  every new measurement field. This covers all 16 existing suite groups.
- Smoke/config: 60 executions (10 cases × 2 backends × states/nontriggering
  stall/exact options). Observation and nontriggering stall preserve all
  three existing streams. Exact caps preserve the key prefix and end at the
  requested game turn. All 30 C/Rust pairs match all three streams and full
  summaries; STATE files match byte-for-byte.
- Focused controls cover native STATE format/cadence, HP/max-level/energy/RNG
  values, live profile settings, exact 20,000 versus legacy 20,003, invalid
  limits and a `/dev/full` stream failure. A production-body frozen-clock
  fixture checks rising energy, turn changes and disabled stall stopping.
- The real lunal fixture stops at frozen turn 371 after decision delta 100
  and energy delta 10,000. C/Rust key logs are identical prefixes of the
  uncapped key-budget run. No Borg loop or tactic is fixed.
- Three random births match frozen native-observer results on both
  backends: 2624955987 Elf Druid, 2624955988 Gnome Mage, 3167239341 Half-Troll
  Ranger. Random birth changes a fixed character's keys; exact/stall stops
  can shorten the old log. Those are declared input/stop changes, not changed
  Borg choices for the same input prefix.
- The native runner's unchanged `parse_states()` accepts the new records.
  [Contract pins](observer-contract.json) identify the inspected C observer
  and parser. The fixed non-holiday calendar remains unchanged.
- Non-harness `game-world.c` compiles byte-identically to the pre-change
  source with BORG_HARNESS disabled; see [check](nonharness-check.json).
- Actual CMake unit targets link and pass 11/11 parser tests per backend.
  All 49 original native C scenarios still pass against the final core and
  unit helpers. These C-private-global scenarios are not Rust-native tests.
- Final no-C-Borg/link audit passes: 154 identical production engine link
  objects; 158 matching stored engine/test objects; zero C Borg objects in
  Rust; the traced relink reproduces the executable exactly. The asymmetric
  unit-object control still fails, now with a useful hint; no criterion is
  relaxed. Python compilation and whitespace checks pass.

## Retained attempts and reproduction

Raw runs, logs, images and source/object hashes remain under
`harness/out/measurement-validation`; focused runs use separate
`harness/out/measurement-*` directories. No whole worktree is archived.
The default sweeps started at two workers per backend, then used four;
the scheduling receipt records that change. Three initial extended jobs were
interrupted when replacing the inadequate 900-second verification wall with
7,200 seconds. Their partial records remain excluded. Completed cases were
requalified and reused; later scheduler pauses let active games finish.
Original incomplete coordinators never receive fabricated completion markers.
The first additional-suite comparison was refused for missing manifests;
the collector now writes explicit source-verified case-join manifests from
the actual receipts before running strict comparisons.

The focused negative RNG control uses a disposable copy and preserves its
original run. Missing measurement fields/calendar are rejected; legacy
summaries remain valid. [validation.json](validation.json) binds source,
image, stream/summary trees, comparison logs, selected cases and scheduling.
[Final checks](final-checks.json) bind the rebuilt checks. The receipts retain
precommit build metadata instead of relabelling it as a later clean build.

Build with `harness/build.sh` and `BORG_IMPL=rust harness/build.sh`.
Run `python3 harness/regress/measurement-test.py` for focused controls.
`measurement-suites.py` runs the smoke/config matrix; `measurement-remaining.py`
collects the five extra suites, supports qualified reuse and `--verify-only`,
and binds preserved images using `--baseline-ref`/`--measurement-ref`.
`measurement-receipt.py` reruns the complete strict comparisons before export.
Its current paths select the retained M2 plan, fixtures and measured outputs.
These checks do not establish whole-private-Borg equivalence, new coverage
percentages, survival improvement, milestone acceptance or a tag decision.
