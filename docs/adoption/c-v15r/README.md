# better-borg-c v14 and v15 reconciliation

This candidate carries the runtime changes from the previously adopted
[better-borg-c v13](../c-v13r/README.md) (`9ccb2aaeebc56ad6aeb35b668f112f0f58b142f8`)
through [better-borg-c v15](https://github.com/diegoami/better-borg-c/releases/tag/4.2.6-impact-v15)
(`646ebfc2a3a80a0caf87f1ab72fe4bcee0d94343`). The v14 snapshot is
`cdfb6d477c477e948fef3f56aa610d682488b2e3`. The v15 runtime import is
`b684b95b450f89ba3d819482d41af2a6946d14cf`.
The [v14 review corrections](https://github.com/diegoami/better-borg-c/pull/543)
concern the donor's evidence and documentation. The [v15 review](https://github.com/diegoami/better-borg-c/issues/547)
was open when this candidate was prepared.

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
