# Engine captures from the corrected extended matrix

Thirty fixed-cadence engine saves are retained from source28b8170's
unchanged extended suite. Every scenario reaches its original limit;
strict C/C and Rust/C are12/12 identical in keys/messages/player state.
All capture metadata, save sizes, SHA-256 hashes and complete saved bytes
also match across both C passes and Rust. `SOURCES.json` retains all three
source manifests, capture coordinates/levels/turns/keys and file hashes;
`SHA256SUMS` verifies these installed copies. Full source validation receipts
are in `docs/validation/2026-11-06-melee/extended/`.

These characters were born and played by the Borg with `menu,cheat`,
including cheat death. They were not wizard-equipped. Saving occurs in a
forked private engine copy; its monster compaction cannot mutate the live
source. The source run reaches natural maximum depth99 (Warrior2007)
and51 (Blackguard3100); the deepest actual saved starting point is the
Warrior at78. A source maximum is not necessarily its saved current depth.

Loading starts a fresh Borg, reseeds at the pre-Borg boundary and restores
engine state only. Private Borg memory is not restored; these are independent
scenarios, not pieces of an uninterrupted trace. Saved-byte equality covers
the serialized engine state, not every private/runtime state field or every
possible tactic.

`harness/suites/natural` declares the100k-turn fresh replays of the depth78
Warrior (clevel50) and depth40 Dunadan Blackguard (clevel35). Both inputs
have prior C/C/Rust helper replay matrices marked MATCHED. Run the named
suite with `SAVE_ROOT=harness/saves/captured-extended`, and compare with
`harness/diff --suite natural --save-root harness/saves/captured-extended`.
The `--turns` limits are absolute engine turns after loading, not extra
turns counted from zero. No self-lunal, Sea of Runes or win is inferred
from a depth or from the character's maximum depth alone.
