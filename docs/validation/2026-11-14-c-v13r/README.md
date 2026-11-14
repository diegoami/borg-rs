# Carry better-borg-c milestone 13 as v13R

Owner direction (2026-11-14, "you can do v13r"): carry the consumer's
milestone 13 (`4.2.6-impact-v13`, runtime snapshot
`9ccb2aaeebc56ad6aeb35b668f112f0f58b142f8`) into borg-rs as **v13R**, the
Rust counterpart of its nineteen carry units (patches 0119-0137). Record:
[adoption/c-v13r](../../adoption/c-v13r/README.md). This is author
validation, not independent review or acceptance.

Start: Rust main `a4739fa8c81e6183f17723733e899c0ed8e19dfb` (after the
v12R acceptance records, the CI workflow, and the munchkin-off change).
Implementation branch `claude/v13r-carry`.

The consumer's own milestone-13 review
([better-borg-c#521](https://github.com/diegoami/better-borg-c/issues/521))
was **open at carry time** (reviewer assigned the same day, no verdict).
The v12 cycle had the same shape; the consumer's package, review map,
per-unit verifiers, kits and native gates
(`docs/measurement/IMPACT-MILESTONE13.md`, `docs/review/MILESTONE13.md`)
are the review input, and borg-rs's own checks below are this
repository's evidence.

## Scope

Nineteen units, exactly the consumer's tagged milestone. The consumer's
runtime surface is 16 Borg files, +155/-124; this tree's C side carries
the same change with the known local deltas preserved (the
`borg-caution.c` include order, the `borg-update.c` 0098 guard, the
`borg-fight-attack.c` stale-target comment, and the family-lane /
munchkin-off settings in `borg-init.c`, `borg.h`, `borg.txt`). The Rust
side carries the same nineteen units; the unit map is in the adoption
record and `series.json`.

Not taken: the consumer's post-tag carries 0138-0140 (sources 154, 590,
591; milestone-14 scope there), source616 (the consumer proved it a
byte-identical duplicate of carried 0004), sources 13/249/250 (no
released fix), sources 595/615 (awaiting the consumer's owner decision).

## Rust and link checks

- `cargo test --release --manifest-path borg-rs/Cargo.toml`: **123
  passed, 0 failed, 1 ignored**. The new test
  `borg_trait_swap::tests::prot_fear_marks_the_swap_resist_fear` pins the
  carried 0132 rule (OF_PROT_FEAR writes the swap flag; clear() resets
  it), and the pre-existing `clear_keeps_resist_fear_only` pin of the old
  C bug 45 behaviour was replaced by `clear_resets_every_flag` (also
  asserting the 0132 reset).
- `cargo clippy --release --all-targets --manifest-path
  borg-rs/Cargo.toml -- -D warnings`: **clean** on this machine's rustc
  1.98.1 (the seven findings recorded in the v12R receipt appear only on
  the newer 1.100.0 toolchain).
- `harness/check-no-c-borg.sh`: **OK**; the Rust binary links no C Borg
  (154/154 engine objects byte-identical between the C and the Rust
  build). The first run of this check failed on a stale orphan object:
  `build/rust` still carried a `native-replay.c.o` from an earlier
  `SUPPORT_BORG_RECORDING=ON` configure (the death-workflow builds), and
  CMake does not delete orphaned objects when the source leaves the
  target. A clean reconfigure of `build/rust` fixed it; the failure was
  environmental, not a carry defect.

## The trait-enum shift (0135/0137)

`borg-trait.h` gains five traits (`BI_PSTUN`; `BI_VFIRE`, `BI_VELEC`,
`BI_VCOLD`, `BI_VACID`). The Rust mirror
(`borg_headers/borg_trait_h.rs`) was regenerated from the carried header
- every trait index after the insertion points shifted (+1, then +4) -
and the `prefix_pref` mirror in `borg_trait.rs` received the same two
insertions at the same positions. The consumer's review map names this
exact hazard for 0135/0137. `borg_danger`'s physical cold case keeps the
upstream asymmetry: it gains only the vulnerability penalty, not the
`&& !BI_VCOLD` condition the other eleven element sites get.

## C/Rust parity

Both roles are clean builds of this branch's head (`git_dirty=no`,
`inputs_dirty=no`) on the same pinned engine and data. The standing
oracle was re-made from the C build with the documented two-pass method
(each suite run twice, the passes identical, pass 1 stored as `.xz` logs
plus `summary.json`, `config.txt`, `timing.txt`, the manifest and the
`DONE` marker):

| suite | runs | the two C passes | pass 2 vs the oracle | the Rust role vs the oracle |
|---|---|---|---|---|
| smoke | 3 | identical | **3/3** | **3/3** |
| full | 45 | identical | **45/45** | **45/45** |
| classes | 16 | identical | **16/16** | **16/16** |
| config | 7 | identical | **7/7** | **7/7** |
| config-six | 6 | identical | **6/6** | **6/6** |
| options | 25 | identical | **25/25** | **25/25** |
| deep | 9 | identical | **9/9** | **9/9** |

**111 of 111 runs identical on keys, messages and state** in all three
columns. The digests of every stored file are in
[oracle-digests.json](oracle-digests.json) (576 files); the raw runs stay
under `harness/out/` outside Git.

### What the milestone changed in the corpus

Against the previous (munchkin-off) oracle, **7 of the 111 runs changed**;
the other 104 are byte-identical, so the v13 branches never fire there:

- `config`: `cfg-lunal-deep-warrior` (first divergence at game turn
  373,119 - the heal-amount, cure-list and danger changes reach the
  million-turn run);
- `options`: `opt-chest-war` (turn 159,893), `opt-wmana-rogue`,
  `opt-respawn-random-druid` (**first divergence at turn 1, key ~3,930** -
  patch 0123's respawn birth-menu letters, exactly where a reincarnating
  run re-enters the menu);
- `deep`: `deep-0-warrior`, `deep-5-paladin`, `deep-6-rogue`,
  `deep-7-ranger` (first divergences at turns 159,893 to 270,312).

`smoke`, `full`, `classes` and `config-six` did not change at all; their
re-recorded oracles are byte-identical to the previous ones, and they were
re-made anyway so the whole standing corpus carries one build identity.

**Provenance.** The C build of this branch
(`checkout_source_sha256=41879551b0ff7eccbd3c3d38fcba7547ff310e9dc2e2762f31c18e4254878378`
- a new digest, as expected: the carry changes the C Borg sources;
`git_dirty=no`, `inputs_dirty=no`) produced both passes; the Rust build is
the same tree with `BORG_IMPL=rust`. The investigation corpora under
`harness/reference/` stay untouched. The previous (v12R, munchkin-off)
recordings are preserved in the repository's history.

## Limits

- This receipt does **not** rerun the consumer's per-unit verifiers, kits
  or native gates; they remain the consumer's per-unit evidence. This
  repository's check is whole-program C/Rust parity on the shared harness
  plus the Rust unit tests (including the new 0132 pin).
- The consumer's milestone-13 review (#521) had no verdict at carry time;
  the carry rests on the consumer's package claims and borg-rs's own
  validation. borg-rs's independent review happens on the PR.
- No cohort, per-mode effect, survival or progress claim is made. The
  consumer's package records every unit's per-mode effect as not
  measured; this carry inherits that limit.
- Per-unit dispositions, the not-carried list and the preserved local
  deltas are in [the adoption record](../../adoption/c-v13r/README.md).
