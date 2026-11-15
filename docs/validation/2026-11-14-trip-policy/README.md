# Trip-policy merge validation

Candidate: `ee54f13558eb`, merging town branch `b3f4285` into the
published trip-policy branch at `e81add2`. The merge was conflict-free.
The trip policy remains off by default. No policy parameters were accepted
or changed by this validation.

## Completed checks

- Both engine implementations built with `harness/build.sh`.
- `cargo test --release`: 224 passed, 0 failed, 4 ignored.
- `cargo clippy --release --all-targets -- -D warnings`: passed.
- C/Rust smoke parity: 3/3 identical keys, messages and state.
- C/Rust full parity: 45/45 identical keys, messages and state.
- C/Rust deep parity: 9/9 identical keys, messages and state.
- C/Rust options parity: 24/24 identical keys, messages and state.
- C/Rust synthetic-tactics parity: 3/3 identical keys, messages and state,
  using the retained synthetic saves via `SAVE_ROOT` and `--save-root`.
  These are the remaining glyph cases; retired lunal cases are historical.
- Seed 101, Human Warrior, 20,000 turns: C, Rust, and Rust with
  `BORG_TRIP_RECORD` all produce key SHA-256
  `8d3029a7a4558e087ce1e6e51b51f2f2c777dbbb7ad7f8be65ee9c956108a6c5`.
- Rust family buying validator: 8 completed cases; all four
  direct/selected stream identities and Warrior/Mage purchase anchors pass.

Build identities and suite manifests name the candidate above and use no
identity overrides. Raw outputs remain outside Git under
`/tmp/borg-trip-validation/`; build trees are in
`/tmp/borg-rs-trip-policy/build/`. These are local paths, not portable
artifact storage.

## Outstanding checks and decisions

All five declared parity suites completed: 84/84 cases identical. This
does not establish benefit for the policy when enabled or replace the
missing owner-save replay.

The #59 save replay has not run: the issue's recorded bundle path
`harness/out/bug-reports/2026-11-14-blessing-full-pack/` does not exist in
the owner's main checkout on this machine, and no other copy was found.
The issue remains open; source removal alone is not a replay receipt.

The town merge already includes the owner's healing and Recall decisions;
see `docs/TOWN-INVENTORY-PLAN.md`. Shared `PlannedTrip` integration remains
unimplemented and belongs to the buying session (owner decision,
2026-11-14). In particular, the town model's two Recall uses must be
reconciled with the trip proposal's three-use round trip before integration.
Policy fitting, default activation, zoo migration and derived tags remain
outside this validation.
