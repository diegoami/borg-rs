# M5 independent review: author tooling follow-up

Reviewed candidate28468aa remains frozen. Independent review:
https://github.com/diegoami/borg-rs/issues/2#issuecomment-6711741153
Verdict: qualified pass, no acceptance blockers. Author dispositions:
https://github.com/diegoami/borg-rs/issues/2#issuecomment-6711790956

This separate follow-up changes Python operational tooling/tests and current
status/qualifications. It changes no production C/Rust, engine/data, references,
historical dossier identities, older receipts or tags. Independent verification
of these author fixes and owner acceptance remain pending. M4 stays accepted;
no C-consumer gameplay milestone is selected by this work.

## Findings and dispositions

### M1 — agree; effective continuation identity ignores the requested seed

The engine restores saved RNG under keep-load-rng. Requested --seed stays
in the original recording/profile/summary for exact rerun provenance, but it
is not a gameplay input for this continuation. Effective death identity.seed
is now null and the family is derived from actual origin/race/class/config.
Attempts retain requested_seed and seed_role, so two executions remain two
process attempts. Fresh-birth seeds still identify their starting conditions;
GUI identities continue using actual recorded GUI inputs.

Legacy continuation identities are not rewritten: comparison and aggregation
derive the same effective family, and new repeats append only to the matching
case. Old dossiers retain original identity/evidence bytes. The four historical
fresh-birth dossiers and their committed aggregate still reproduce byte-for-byte.

The actual reproducer copies the ordinary living Druid5002 checkpoint, runs
seed1, restores the same origin from backup2 and runs seed2. Both die at10668;
origin and complete keys/messages/state/frames/final.save bytes match. Before:
2cases/2starting conditions/2process attempts. After:1case/1starting condition/
2process attempts, with requested seeds1/2 preserved. Aggregating a copy of
the two older dossiers gives2historical cases/1starting condition/2attempts
without changing their record.json bytes. This is diagnostic continuation
replay, not two independent lives or a survival improvement.

### L1 — agree; validate the selected frozen backup before restoring

Build availability is checked before restore. The selected backup is copied
privately, deserialized for living/race/class metadata and validated. Restore
then uses those exact validated bytes; only successful preflight may preserve/
replace the current name and rotate backups. GUI restore's existing default
helper path stays compatible.

The real wrong-race continuation refusal changed the named file/backup set
before this fix. The same request now leaves every named/backup byte unchanged.
Further actual native metadata checks reject wrong class, dead and corrupt
backups without replacement/rotation; an injected unavailable-build failure
also leaves data unchanged. Existing ordinary valid restore still passes.

### L2 — agree; test expectations come from the supplied original bundle

Death cause/turn assertions now read the native summary. CLI turn/frame
expectations are selected from the actual recorded JSON frames. Exact/missing
window coverage is checked without assuming a particular game's gaps.

Before the fix, our independently generated Druid8402/cutpurse5590 bundle
produces the same two literal failures reported by the reviewer. After, the
same19 death/CLI controls pass on both that Druid and the older Rogue8406/
centipede5600 bundle. The nineteenth check is the narrowed timestamp control;
no test depends on the author's original cause/turn literals.

### I1 — agree with the declared exception; restricted to the first line

Only line1 of headless borg-log.txt can differ as a recognized timestamp.
Every later byte compares exactly, including date-shaped text. A real-rerun
mutation control gives line2 different timestamp-shaped values in original
and rerun output and is rejected. Original first-header bytes stay hashed and
their difference remains reported; they are not authenticated as historical
truth by replay. GUI archive bytes still compare exactly. No clock/RNG or
native gameplay change is made.

### I2 — agree; trusted local cache is explicit

The matching local verification.json cache can skip a fresh execution; it is
not a signature or independent attestation. tools/README.md now says this
explicitly and gives --output NEW_DIRECTORY / --timeout SECONDS to force an
actual rerun. Input/runtime hashes are checked before cache acceptance.

### I3 — agree; portable source-pin check added separately

The historical native/SHA256SUMS retains its original absolute paths. They
refer to old checkout bytes and should not be checked against today's edited
worktree. SOURCE_AT_C2B52D9.sha256 and historical-pin.json preserve all19
original hashes with repo-relative paths. verify-historical-source.py verifies
the immutable git blobs at c2b52d9d190c2238a9d18b4d97d37ab16f83f29c; all19 match.
The original ledger/receipts are unchanged, not repaired to newer source.

### Q1 — agree; recognition observer is the discriminating channel

The prelaid-ring case discriminates #295 through gameplay logs/full save;
the other five recognition cases match before/after gameplay output. The
known-Rune/decoy/hidden/trap evidence is the native observer's map/tracking
state, with independently verified observation noninterference. Current fork
policy docs now say this explicitly; original receipts remain source-qualified.

## Checks and reproduction

- Same-origin continuation reproduction before and after, with complete
  channel/save hashes, registry counts and rejected-restore byte inventories.
- Four M5 controls pass (including four native metadata refusal subcases):
  build failure, invalid backup validation, real seed-variant grouping, and
  preservation/grouping of old continuation identities.
- Nineteen death/CLI tests pass independently on each Rogue/Druid recording.
- Ten existing headless/save and ten GUI recording controls pass.
- GUI death export metadata remains GUI-input-scoped; its seed is not a birth
  seed. Historical fresh-birth aggregate/index/README reproduce unchanged.
- Nineteen historical source blobs match their c2b52d9 pin. Python compilation,
  source whitespace checks and unchanged production source checks pass.

No gameplay corpus/native checker/coverage rerun is claimed for these Python
fixes. The reviewer's independently executed C/C/Rust/native/GUI checks at the
frozen candidate remain their own evidence. Real validation used private
profiles/registries and actual living/dead saves, never owner game data.
Failed negative-control replay outputs stay in their retained private paths.

```
python3 reproduce.py /tmp/m5-review-after
BORG_M5_CONTINUATION_RESULT=/tmp/m5-review-after/result.json BORG_M5_LEGACY_REGISTRY=/path/to/before/deaths python3 tools/test_m5_followup.py
BORG_DEATH_TEST_RECORDING=/path/to/a/complete/natural/headless/death python3 tools/test_death_records.py
BORG_DEATH_TEST_RECORDING=/path/to/a/complete/natural/headless/death python3 tools/test_replay_cli.py
python3 docs/validation/2026-11-07-m5-review-followup/verify-historical-source.py --repo /path/to/borg-rs
```

reproduce.py expects a new output directory and uses private profile/registry
paths. Its pinned living fixture is diagnostic; it does not create an artificial
kill. The before/follow-up outputs and full frozen runtime bundles remain under
harness/out/m5-review-followup. This export retains compact results, manifests,
curated counts/evidence and logs; it is not a complete executable replay bundle.
