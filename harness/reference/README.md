# The harness reference corpora

`harness/reference/` holds the C oracle for the harness suites and the
frozen evidence of past investigations. It is **not** one uniform set: the
suites' oracles are refreshed when the C build changes, while the
investigation corpora are records of one past run and are never rewritten.

## The standing suite oracles

These are the corpora that `harness/diff` compares a suite run against.

| dir | suite | C build that made it | notes |
|---|---|---|---|
| `smoke` | `harness/suites/smoke` | the v12R C build (see below) | 3 runs |
| `full` | `harness/suites/full` | the v12R C build | 45 runs |
| `classes` | `harness/suites/classes` | the v12R C build | 16 runs |
| `config` | `harness/suites/config` | the v12R C build | 7 runs |
| `config-six` | `harness/suites/config-six` | the v12R C build | the six review runs of `config`, without the 1,000,000-turn case |
| `options` | `harness/suites/options` | the v12R C build | 25 runs |
| `deep` | `harness/suites/deep` | the v12R C build | 9 runs of 1,000,000 turns |

Each of the five refreshed dirs holds, per run, `keys.log.xz`,
`messages.log.xz`, `state.log.xz`, `summary.json` and `config.txt`, and at
the top the manifest of the C suite run that made it, `timing.txt` and the
`DONE` marker.

**The v12R C build** is the C Borg of the consumer's milestone 12
(`better-borg-c` `4e1b912cbf6d2b8a0243b19ebddfb2d26e7509f2`, tag
`4.2.6-impact-v12`) on the imported engine `42cd156ac`, with borg-rs's own
fork corrections and harness hooks. The five refreshed oracles above were
made from that build with the documented method (`docs/harness.md`, "Suites
and reference logs"): the C build ran the suite **twice**, the two passes were
identical on `keys.log`, `messages.log`, `state.log` and `summary.json`
(apart from `wall_seconds`), and one pass was stored as `.xz` logs plus
`summary.json`, `config.txt`, `timing.txt`, the manifest and the `DONE`
marker. The record is
[docs/validation/2026-11-13-c-v12r](../../docs/validation/2026-11-13-c-v12r/README.md),
and the digests of every stored file are in its `oracle-digests.json`.

Before this refresh the suite oracles were the M1/M4-era recordings, made
on the `e473720ed` engine with the M1 Borg. They stopped describing the C
Borg after the milestone 2 carry and stopped being reproducible after the
milestone 12 import, which changed engine behaviour: the new C build
matched 0/45 of the old `full` oracle. The previous recordings are kept in
the repository's history (they were not deleted, only superseded).

**`options` and `deep` were refreshed last (2026-11-14, on the owner's
machine).** They were the last two M1/M4-era recordings after the first
refresh: on the imported engine the Borg acts in about 80,000 of an
`options` run's 100,000 turns instead of about 14,000, and in the sandbox
where the other five oracles were made each active run writes gigabytes to
a network filesystem, so a two-pass refresh was not affordable there. On
the owner's machine, with local disk and more cores, each C pass cost
about four minutes (`options`, 25 runs, JOBS=13) and about six minutes
(`deep`, 9 runs, JOBS=9), and the same method was applied: the two passes
were identical in both suites, one pass was stored, pass 2 was re-checked
against the stored oracle, and the Rust role matches both oracles -
**25/25** and **9/9** identical on keys, messages and state. That is also
the first C/Rust parity check of these two suites on the imported engine.
The refresh used a fresh build of the same branch head whose
`checkout_source_sha256` equals the five earlier oracles' build inputs, so
the whole standing corpus now describes one C source of truth.

## The frozen investigation corpora

These are records of one past investigation, not live oracles. They are
kept as they are: rewriting them would destroy the evidence they are.
They were made on the `e473720ed` engine and describe the C Borg and engine
of their time.

`captured-rogue`, `crash-fixes` (+ `-26a96be`, `-bd184cc`), `endgame`,
`extended` (+ `-26a96be`), `glyph-construction`, `glyph-recognition`,
`lunal-investigation`, `natural`, `normal-live`, `synthetic-tactics` (+ `-345`,
`-glyph-295`), and `better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f`
(the milestone 2 corpus).

All seven standing suite oracles are now the v12R C build, made with the
two-pass method; none is left on the old engine.

## Using an oracle

    BORG_IMPL=rust harness/suite.sh full
    harness/diff --suite full --expect-impl rust harness/out/rust/full harness/reference/full

A reference dir with no build identity in its manifest is compared by
content only. A reference that keeps the manifest of the C suite run that
made it (as the refreshed oracles do) is treated as a compressed C
suite output: the diff prints that C build's identity and checks it
(`BORG_IMPL=c`, no override, `impl_check=ok`, `build_check=ok`, the binary
unchanged during the run), and the `DONE` marker is required.
