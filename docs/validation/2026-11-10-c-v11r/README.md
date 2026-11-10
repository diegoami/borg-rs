# Carry better-borg-c milestone 11 as v11R

Owner direction (2026-11-10): carry the consumer's milestone 11
(`4.2.6-impact-v11`, runtime snapshot `39268ca8a470d1cbd44de27bf1fd1b3ee403bada`)
into borg-rs as **v11R**, the Rust counterpart of its 18 runtime units
0100-0117. Record: [adoption/c-v11r](../../adoption/c-v11r/README.md). This is
author validation, not independent review or acceptance.

Start: Rust main `399c907950b1fd474ddfc1c5a5d4f86063778aac`
(`4.2.6-impact-v10`). Implementation branch: `claude/v11r-carry`, commit
`0337f1fe1e410a176b14bcd9e8bb0b9d7724767a`.

The consumer's milestone 11 was independently reviewed: verdict approve with
corrections, all five findings applied in `981be4a1d` with no runtime change,
and the owner placed the annotated tag `4.2.6-impact-v11` on the reviewed
runtime snapshot (tag object `443ba5c4`, record `c1177a2df`). See
[better-borg-c#471](https://github.com/diegoami/better-borg-c/issues/471) and
[#474](https://github.com/diegoami/better-borg-c/pull/474).

## Scope

All 18 runtime units, 19 patch files (`0101a` corrects `0101`). The 11 C Borg
files this diff touches are byte-identical to the consumer's blobs at
`39268ca8a`; the Rust port carries the same corrections. No other gameplay
change is included.

## Identities

Clean builds of the final commit, from a fresh clone:

- [C-BUILD_ID](C-BUILD_ID): `impl=c`, `git_rev=0337f1f`, `git_dirty=no`,
  `inputs_dirty=no`, source `e1e65f643e599e66…`, binary `2104446058c08204…`.
- [RUST-BUILD_ID](RUST-BUILD_ID): `impl=rust`, `git_rev=0337f1f`,
  `git_dirty=no`, `inputs_dirty=no`, source `55d5b1e1361ef661…`, binary
  `1a313f3eff57d5f5…`.
- Toolchain: Ubuntu `cc` 15.2.0 and `rustc` 1.98.1.

The 45-case full suite below also ran on these clean builds; the C and Rust
source hashes are the same as in the smoke manifests.

## Checks

- `cargo test --release --manifest-path borg-rs/Cargo.toml`: **105 passed, one
  existing ignored** (including the no-rest timer no-wrap test added after the
  review).
- `cargo clippy --release --all-targets --manifest-path borg-rs/Cargo.toml --
  -D warnings`: clean.
- `harness/check-no-c-borg.sh`: OK; the Rust binary links no C Borg.
- Smoke suite: **3/3 runs identical** on keys, messages and state.
- Full suite: **45/45 runs identical** on keys, messages and state.
- C Borg sources: all 11 files byte-identical to the consumer's v11 blobs.

The 45-case full suite is the standard early-game matrix (100k turns per
case), not a new cohort and not a per-unit fixture run. Both roles used the
same pinned engine and data.

## Unit coverage

The adoption record maps every unit to its C patch and Rust site. The Rust
sites are:

- `borg_escape.rs` (0100), `borg_flow_take.rs` (0101/0101a, 0104),
  `borg_flow_kill.rs` (0102, 0110), `borg_danger.rs` (0103, 0107, 0108,
  0109), `borg_think_dungeon_util.rs` (0106), `borg_inventory.rs` (0111),
  `borg_item_use.rs` (0112), `borg_item_wear.rs` (0113),
  `borg_store_buy.rs` (0114, 0115), `borg_store_sell.rs` (0116, 0117).
- `no_rest_prep` (0105) spans `borg.rs`, `borg_fight_defend.rs`,
  `borg_fight_perm.rs`, `borg_flow_stairs.rs` and `borg_update.rs`.
- Rust unit tests were added for the nearest-stair scan and the
  granite-only wall test; `docs/c-bugs.md` is updated per finding.

## Limits

- This receipt does **not** rerun the consumer's per-unit native fixtures or
  its 95-group/290-case matrix. Those remain the consumer's native evidence
  for the target; this repository checks the Rust translation against the
  consumer's C blobs through the shared harness.
- No cohort, per-mode effect, survival or progress claim is made. The
  consumer's package records every unit's per-mode effect as not measured.
- The v11R milestone is the Rust counterpart only. No `v11R` tag is placed;
  acceptance and any tag decision remain with the owner.
