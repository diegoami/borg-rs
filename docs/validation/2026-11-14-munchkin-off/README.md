# The munchkin-by-class start turned off, and the standing oracle re-made

Owner direction (2026-11-14): "I want to remove it, put it all to false in
main" - the implicit munchkin start that Warriors, Druids and Rangers took
under the better-borg-c PR302 feature. This is author validation on the
owner's machine, not independent review.

## The change

One line of configuration, no source change: `harness/borg.txt` sets
`borg_munchkin_by_class = FALSE` (commit on top of the v12R acceptance
records). The global `borg_munchkin_start` was already FALSE, so with this
change **no class takes the munchkin start any more**: every character
plays the plain game from turn one. The C and Rust code keeps the PR302
feature and the per-run `munchkin` flag still forces the mode, so the
`cfg-munchkin-*` runs of `config`/`config-six` and `opt-munchkin-limits-war`
keep exercising it deliberately. The binaries are unchanged: `borg.txt` is
runtime configuration both roles read, and the C build identity
(`checkout_source_sha256=28974d08…`) still matches the previous corpus's
build inputs.

Effect on the streams, measured on the new recordings: the munchkin
flavour collapsed where it used to apply (smoke warrior 9,681 -> 2,616
keys; `opt-respawn-random-druid` 54,502 -> 15,586; `opt-wdam-war`
80,635 -> 14,278, clevel 11 -> 7 - the worshippers-damage run stair-scummed
heavily before), while runs that never munchkinned kept their exact key
streams and differ only in the `effective_settings` metadata (which now
records `borg_munchkin_by_class: 0`).

**A labelling quirk found while measuring this.** The `deep` suite's run
names predate a class-list change: `deep-7-ranger` actually births a
**Warrior** and `deep-8-blackguard` a **Ranger** (the old summary.json
records the class names), which is why four of its nine runs - not the two
the names suggest - were munchkin-flavoured and changed. The suite files
are kept byte-identical; their names are historical labels.

## The oracle re-made

The standing suite oracle was re-made from the same v12R C build with the
method `docs/harness.md` prescribes, on the owner's machine: each of the
seven suites ran **twice**, the two passes were compared on `keys.log`,
`messages.log`, `state.log` and `summary.json` (apart from
`wall_seconds`) and were identical, and pass 1 was stored as `.xz` logs
plus `summary.json`, `config.txt`, `timing.txt`, the suite manifest and
the `DONE` marker. Pass 2 was then re-checked against the stored oracle,
and the Rust role ran every suite against it.

| suite | runs | the two C passes | pass 2 vs the oracle | the Rust role vs the oracle |
|---|---|---|---|---|
| `smoke` | 3 | identical | **3/3** | **3/3** |
| `full` | 45 | identical | **45/45** | **45/45** |
| `classes` | 16 | identical | **16/16** | **16/16** |
| `config` | 7 | identical | **7/7** | **7/7** |
| `config-six` | 6 | identical | **6/6** | **6/6** |
| `options` | 25 | identical | **25/25** | **25/25** |
| `deep` | 9 | identical | **9/9** | **9/9** |

**111 of 111 runs identical on keys, messages and state** in all three
columns.

**Provenance.** The C build is the v12R C build of main at the munchkin
commit (the vendored engine `42cd156ac` plus the documented fork
corrections and harness hooks; `git_dirty=no`, `inputs_dirty=no`,
`checkout_source_sha256=28974d08729bda49cd70affad36d1a4f27cf61d7672d81af23e887aeee19a231`,
the same build-input digest the v12R oracles record). The Rust build is
the same tree with `BORG_IMPL=rust`. The `Rust vs oracle` column is the
C/Rust parity check of the munchkin-off behaviour: the Rust Borg tracks
the C Borg exactly with the new configuration.

**Scope.** Only the seven standing suite oracles were re-made. The
frozen investigation corpora under `harness/reference/` are records of
past investigations on the `e473720ed` engine and stay unchanged.
`harness/reference/README.md` maps every corpus. The previous (munchkin
on) recordings are preserved in the repository's history.

The digests of every stored file are in
[oracle-digests.json](oracle-digests.json) (576 files); the raw runs stay
under `harness/out/` outside Git.

## Limits

- This receipt records a configuration change and its oracle: it makes no
  survival, progress or benefit claim for either setting. The by-class
  feature was carried at the owner's direction in milestone 8
  (better-borg-c PR302) and is now off by the owner's direction; the code
  stays for upstream parity.
- The `Rust vs oracle` checks ran on the owner's machine, not in CI; CI
  (`.github/workflows/rust-test.yml`) covers the Rust unit tests only.
- The suite-label quirk above is recorded, not fixed: renaming the `deep`
  runs would change the suite files' bytes and with them every recorded
  `suite_sha256`.
