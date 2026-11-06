# Historical extended parity before the melee correction

At source `26a96bee4570496dceee5f9f6407a91036c55891`, all three unchanged
12-case extended passes finish with `DONE`. Strict C/C and Rust/C each
report **12/12 identical** in keys, messages and player state; all 36
processes exit successfully with unchanged executable hashes and no overrides.
The compressed C1 reference is `harness/reference/extended-26a96be`,
rechecked against C2 after compression. Exact summaries, process outcomes,
log hashes and comparisons are retained here; uncompressed local outputs
are `harness/out/local/extended-{c1,c2,rust}`.

This is historical parity evidence, **not acceptance of the later melee
correction or a claim that every scheduled 5M-turn scenario reached its
limit**. Warrior2000 stops at1,887,392 turns because a stale same-square
monster is selected for melee, queues `+5` and triggers the unexpected
request-for-direction abort. The stop is identical in both C passes and
Rust. Its message tail and summary are retained, and the defect is tracker
[#322](https://github.com/diegoami/neo-angband-tracker/issues/322).
Owner-authorized correction `28b8170` requires distance exactly one in
C/Rust; the native real-body fixture fails before and passes after.
Fresh full post-correction validation is running separately.

The other two long scenarios demonstrate natural deep play:
Warrior2007 finishes at5,000,192 turns, clevel50, maximum depth99;
Dunadan Blackguard3100 finishes at5,000,002 turns, clevel43, maximum depth51.
Other completed maxima include Ranger36, Warrior34 and Paladin31.
A reached depth alone does not prove Sea of Runes, self-lunal or a win.
The original declared suite remains unchanged. Main/tags are unchanged.
