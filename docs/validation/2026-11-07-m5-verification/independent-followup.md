## Reviewer verification of follow-up `0203227`

I verified `020322749026418a666da3209c169f48ec9d608c` in a fresh detached worktree, using a private Rust build, profiles and registries.

**Scope confirmed:**
- `git diff 28468aa 0203227 -- angband borg-rs Cargo.toml Cargo.lock harness/run.sh harness/supervise.py` is empty.
- The rebuilt Rust harness has the same `source_sha256` as the reviewed candidate (`d247f691…`). Only the embedded build path/revision changes the binary hash.
- The changes are limited to Python tooling, tests and docs.

| finding | my independent check at 0203227 | status |
|---|---|---|
| **M1** | Reran my exact repro: `live-druid.save`, `--seed 1`, then `--from-backup 2 --seed 2`. Both die at 10668; the second registers as "Death already recorded". Index: **1 case / 1 starting condition / 2 attempts**. Attempts keep `requested_seed` 1 and 2 with the role "ignored on load; saved engine RNG". My *pre-fix* registry (D0001 seed 1, D0002 seed 2 same origin, D0003 different origin) re-aggregates to 3 cases / **2** starting conditions / 3 attempts, with all `record.json` bytes unchanged. The committed 4-dossier index regenerates unchanged. Code: seed removed from family/identity for loaded origins; `canonical_identity()` is used for dedupe and aggregation. | **verified fixed** |
| **L1** | A wrong-race `--from-backup 2 --race 3` refusal leaves the named save **and** the whole backup inventory byte-identical. A dead backup candidate (`--from-backup 5`) and a dead named save are also refused with no changes. `build()` and the snapshot inspection now run before any restore; the restore uses the inspected snapshot. | **verified fixed** |
| **L2** | `test_death_records.py` and `test_replay_cli.py` both pass on **two deaths I produced independently**: Druid 8402 / cutpurse / 5590, and the continuation Druid / centipede / 10668. `test_borg_tools.py` passes. In `test_m5_followup.py`, 1 test ran and passed; 3 tests skip without the author's result/registry files, so I covered those behaviors with the direct repros above. | **verified fixed** |
| **I1** | Line 1 rewritten to `1999/01/01 00:00` still VERIFIES and is reported, as declared. The same text on **line 2 is now rejected** ("native death log body diverges"). | **verified narrowed** |
| **I2** | `tools/README.md` documents the cache as trusted local data, not attestation, and explains how to force a rerun. | **verified (docs)** |
| **I3** | `verify-historical-source.py` reports "19 source blobs match c2b52d9…". The original ledger is untouched. | **verified** |
| **Q1** | `FORK-CORRECTIONS.md:150-152` and the follow-up README state that the prelaid ring is the gameplay-output discriminator and the observer covers the other cases. | **verified (docs)** |

**Not rerun at 0203227:** game, kernel and coverage suites. Production inputs are unchanged, so my candidate-source results at 28468aa still stand. I also did not rerun the GUI recording controls; `borg_gui.py` is unchanged.

**Updated verdict: pass for the M5 delta at `28468aa` + tooling follow-up `0203227`, with no open findings.** The standing limits from my final review still apply: no win, survival benefit, whole-Borg memory equality or new coverage is claimed; Rune construction is a prepared-room result; the headless death-log line-1 clock exception is declared. Owner acceptance, any tag, issue closure and the choice of C milestone remain the owner's decisions.
