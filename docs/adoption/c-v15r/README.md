# better-borg-c v14 and v15 reconciliation

This candidate carries the runtime changes from the previously adopted
[better-borg-c v13](../c-v13r/README.md) (`9ccb2aaeebc56ad6aeb35b668f112f0f58b142f8`)
through [better-borg-c v15](https://github.com/diegoami/better-borg-c/releases/tag/4.2.6-impact-v15)
(`646ebfc2a3a80a0caf87f1ab72fe4bcee0d94343`). The v14 snapshot is
`cdfb6d477c477e948fef3f56aa610d682488b2e3`. The v15 runtime import is
`b684b95b450f89ba3d819482d41af2a6946d14cf`.
The [v14 review corrections](https://github.com/diegoami/better-borg-c/pull/543)
concern the donor's evidence and documentation. The [v15 review](https://github.com/diegoami/better-borg-c/issues/547)
returned approve with corrections. The [follow-up](https://github.com/diegoami/better-borg-c/pull/550)
fixed the donor retirement verifier, release patch base check, and
documentation. These corrections leave the carried runtime unchanged.

| Carry | Source | Disposition in borg-rs |
| --- | --- | --- |
| 0017 | [#239](https://github.com/diegoami/neo-angband-tracker/issues/239) | C and Rust take the maximum slay multiplier across equipment slots, matching upstream `007bea95c`. |
| 0092 | [#364](https://github.com/diegoami/neo-angband-tracker/issues/364) | The healing-chance signs already match upstream `b5747369b` in both implementations. |
| 0138 | [#154](https://github.com/diegoami/neo-angband-tracker/issues/154) | A faster adjacent monster rules out a candidate retreat route at its first step. |
| 0139 | [#590](https://github.com/diegoami/neo-angband-tracker/issues/590) | Only the starving-on-depth-one escape may use delayed Deep Descent. |
| 0140 | [#591](https://github.com/diegoami/neo-angband-tracker/issues/591) | Mind Blast danger accounts for 8d8 damage and unresisted confusion. |
| 0142 | [#192](https://github.com/diegoami/neo-angband-tracker/issues/192) | The removal message clears remembered rubble throughout the in-bounds 3×3 area in darkness. Both ports check map bounds before access. |
| 0143 | [#174](https://github.com/diegoami/neo-angband-tracker/issues/174) | Electricity resistance emits the RElec note. |
| 0144 | [#593](https://github.com/diegoami/neo-angband-tracker/issues/593) | Either teleport capability can enable the landing safety check, matching upstream `95387b06f`. |

The [official engine import](https://github.com/diegoami/better-borg-c/pull/546)
moves the vendored source from `42cd156ac9f95ddf4311e88dd7ad462292d6d1d6`
to `574b618c3bfec595f3503a58f8e488b1c4ae3a46`. The engine source delta is
applied under `angband/src/` and the Borg deltas are reconciled in C and Rust.
Donor CI workflows, release tooling, blocked carries 0141 and 0145, and unrelated
donor recorder tooling are outside this batch. The recorder's own vendored
baseline and the historical harness references retain their original identities.

## Validation

- Rust release tests passed (298 passed, 4 ignored), as did release Clippy
  with warnings denied. The root Python tooling suite completed with 127
  tests, including 26 skips.
- The native C gate passed 966 unit checks and 5 test-frontend checks.
  The pre-carry C gate passed 965 unit checks and 5 frontend checks.
- The link audit found zero C Borg objects in the Rust build. All 155
  production engine objects matched byte for byte. The audited relink
  reproduced the Rust binary exactly.
- C and Rust matched on keypresses, messages, and engine state in all
  3 smoke scenarios and all 45 full-suite scenarios. Both builds used
  runtime commit `5e6cd07565d145d8d5044d4c31536318ba38f785` and the
  repository harness configuration. The suite reported no overrides.
- Two candidate runs of the eight native C fixture kits passed all 18 modes
  and produced identical logs. The six v14 regression modes failed on
  the pre-carry build as expected. The pre-carry and candidate control
  logs match except for Deep Descent, which adds one teleport diagnostic
  line with the same decision. The heal and slay fixture logs also match
  across the two builds. The fixtures use C Borg internals. Rust parity
  is measured through the harness binaries.

The [native fixture runner](../../../harness/adoption/v15r/check-native.py)
reads the sources and profiles by commit from a better-borg-c Git checkout.
For a shallow clone, fetch the donor release tag and the two pinned tracker
fixture objects before running it.

```sh
git -C ../better-borg-c fetch origin tag 4.2.6-impact-v15
git -C ../better-borg-c fetch --depth=1 https://github.com/diegoami/neo-angband-tracker.git \
  f88fee1e3ace2b5f34f6fc8a1b4fbc30d3f94da1 \
  25b0118631f108edc926856c8f4b3c35d3611bf9
cmake -S angband -B build/native-v15 -DBORG_IMPL=c -DSUPPORT_TEST_FRONTEND=ON
cmake --build build/native-v15 --target alltests --parallel 4
python3 harness/adoption/v15r/check-native.py --source ../better-borg-c \
  --build build/native-v15 --output build/v15-fixtures
```

For the Rust repository's pre-carry C build at `8c5d000a58e9ddca1f1954ab9552e505856b313f`,
run the same fixture checker with `--before`. It expects the six v14
regression modes to fail and both already-carried behaviors to pass.

## Independent review

The [PR review](https://github.com/diegoami/borg-rs/pull/109#pullrequestreview-5984695685)
approved `d1b74dbc` and independently reproduced the native gate and fixtures.
The [fork corrections](../../FORK-CORRECTIONS.md#rubble-map-bounds-in-the-v15-carry)
and [validation receipt](../../validation/2026-11-19-c-v15r/README.md) record
the two documentation observations. Owner acceptance is pending.
