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
| `options` | `harness/suites/options` | **the M1/M4-era recording - not refreshed** | 25 runs; still the old engine (see below) |

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

**`options` was not refreshed.** On the imported engine the Borg acts in
about 80,000 of the suite's 100,000 turns instead of about 14,000, and each
active run writes gigabytes to the workspace's network filesystem, so a
single C pass of this suite costs about four hours here (the same suite took
four minutes for the M1-era build). Two passes, the method the other
oracles were made with, were not affordable in the session that refreshed
them. The `options` oracle is therefore still the M1-era recording: it no
longer describes the imported Borg and must not be read as if it did. To
refresh it:

    BORG_IMPL=c JOBS=2 harness/suite.sh options harness/out/c-1/options
    BORG_IMPL=c JOBS=2 harness/suite.sh options harness/out/c-2/options
    # compare the two passes, then store one of them as the new oracle

The receipt names the measurement behind this exception.

## The frozen investigation corpora

These are records of one past investigation, not live oracles. They are
kept as they are: rewriting them would destroy the evidence they are.
They were made on the `e473720ed` engine and describe the C Borg and engine
of their time.

`captured-rogue`, `crash-fixes` (+ `-26a96be`, `-bd184cc`), `endgame`,
`extended` (+ `-26a96be`), `glyph-construction`, `glyph-recognition`,
`lunal-investigation`, `natural`, `normal-live`, `synthetic-tactics` (+ `-345`,
`-glyph-295`), `better-borg-c/m2-2675d198e4c3fcb1c69ee63ef25a2b6ac27ab96f`
(the milestone 2 corpus), and `deep` (the 1,000,000-turn suite oracle).

`deep` is the one standing suite oracle that was **not** refreshed: its
nine 1,000,000-turn runs cost about two hours per C pass here, and it was
not re-run for the v12R carry. It is listed with the frozen corpora for
that reason.

## Using an oracle

    BORG_IMPL=rust harness/suite.sh full
    harness/diff --suite full --expect-impl rust harness/out/rust/full harness/reference/full

A reference dir with no build identity in its manifest is compared by
content only. A reference that keeps the manifest of the C suite run that
made it (as the six refreshed oracles do) is treated as a compressed C
suite output: the diff prints that C build's identity and checks it
(`BORG_IMPL=c`, no override, `impl_check=ok`, `build_check=ok`, the binary
unchanged during the run), and the `DONE` marker is required.
