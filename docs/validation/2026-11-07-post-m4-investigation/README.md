# Accepted M4: post-milestone investigations

The owner accepted the qualified M4 verdict and explicitly requested closure
of borg-rs#1 and resumption of the technical queue. The issue is closed as
completed; tag m4 remains3923a21 and its original finite-scope qualifications
and receipts are unchanged. This work is later source, not a relabelling of
M4's139 executions or coverage percentages.

## Rune construction — FORK #345

Only the glyph builder's reverse-flow call changes optimize=true to false,
mirrored in C/Rust. The helper itself and its other callers are unchanged.
The former optimized walk stopped at the origin, leaving candidates
unreachable. The before C control reverts that caller on the same input save.

The new controlled input is a wizard-equipped level50 Human Warrior with40
known Rune scrolls and no glyphs, in an open room at(60,33). Awake Morgoth is
placed at(190,55), and other uniques are dead to isolate preparation. No
private Borg field is seeded. This permits construction before combat reaches
the room; no enemy is put to sleep or changed after play starts.

Corrected C twice and Rust autonomously create25 engine Runes, use25 scrolls,
return to their chosen center(69,44), and finish at game turn177 with24 Runes
surrounding the player. Native C map/tracking count25 and Morgoth position
true are observed at stop; the four normal C key/message/state/save byte
streams remain unchanged by the separate observer. Rust has identical full
engine saves and public traces; a separate private Rust-state probe is not
claimed. The reverted C caller creates no Rune and fails to complete within
500 turns on this input. The prelaid control completes at turn1 without
spending scrolls; supply9 creates no Rune and reaches its finite turn cap.

The harness-only rune-pattern stop tests actual engine warding glyphs around
the player after an observed key. It only stops/saves; it never supplies
decisions or changes world/Borg state. All three cases pass strict C/C and
Rust/C comparisons including final native-save bytes. verify_construction.py
checks actual before/after outcomes and observer noninterference. Inputs and
separate C2 references are committed. This establishes autonomous formation
in a constructed preparation situation, not a natural career, combat benefit
or a win. An earlier closer-room pilot produced25 Runes but Morgoth disturbed
them before the old clock-only stop; it is not counted as completed formation.

## Self-lunal cycling

The original world-turn bound did not bound stair scumming. With self-lunal
enabled, the real C and Rust thinkers alternate immediately between known
arrival stairs; the fallback climb runs when no useful down stair is known.
Stair commands charge player energy, but on_new_level grants minimum energy
for the next level, allowing repeated transitions without a world-clock tick.
The source explicitly describes this mode as stair-scumming exploitation.

At3,000 observed key events, the ready case makes455 ascents/456 descents,
remains on depths20/21 and stops at world turn371. Non-persistent connected
levels and many distinct arrival positions are observed: this is active
level rerolling, not a frozen process or identical persistent cave. The
option-off control reaches turn4196/depth24 and shallow-history control
turn4680/depth26 within the same input budget. C twice and Rust match all
logs and final saves in all three cases. Private C flags at the final pause
are not interpreted as a decision-time snapshot; the engine may be between
level arrival and the next Borg update.

New key-limit flag / -keys CLI independently bounds this path, with the
existing wall timeout retained. No new lunal strategy change is made: this
run does not establish useful scumming beyond first descent, and the loop
is an inherited deliberate policy rather than a demonstrated Rust divergence.
Stock normal-live runs keep self-lunal off. The retained lunal-analysis.json
uses per-key engine states for depths, not sparse ordinary-thinker notes.

## Ordinary games and natural deaths

Nine fresh ordinary births, all nine actual classes, seeds8400–8408; no
wizard/load/cheat-death/cheat-live flags. Each is capped at100k world turns,
50k key events and180 wall seconds. C twice and Rust match9/9 on all public
logs and final living saves. All540 scheduled live-engine capture sets match
complete C1/C2/Rust bytes. Repeated C/Rust runs are verification, not additional
independent deaths: the cohort is nine distinct inputs, four deaths and five
alive at the cap. None is a win or statistical survival-improvement claim.

| case | actual race/class | result |
|---|---|---|
|8400|Human Warrior|dies to pseudo-dragon light breath, clevel8/current depth13, turn37680|
|8401|Kobold Mage|alive at100096, clevel4|
|8402|Half-Elf Druid|dies to cutpurse, clevel1/depth1, turn5590|
|8403|Half-Orc Priest|alive at100006, clevel2|
|8404|Half-Troll Necromancer|alive at100065, clevel3|
|8405|Dunadan Paladin|alive at100007, clevel5|
|8406|Hobbit Rogue|dies to giant yellow centipede, clevel2/depth1, turn5600|
|8407|Half-Elf Ranger|alive at100007, clevel6|
|8408|Gnome Blackguard|dies to wild cat, clevel1/depth1, turn6231|

All four have separate assessments in docs/deaths with curated prior engine
checkpoints and actual diagnostic inventories. Spoiler kind names are labelled
as engine diagnostics, never additional Borg input. Unknown Speed/Deep Descent
are not treated as known resources. Last-checkpoint stock is not assumed to
remain unchanged until death. No new tactical cause/fix is inferred solely
from mortality, and no early-game survival effect is attributed to #345.

## Validation and failed attempts

Both builds match the existing smoke3/full45/deep9 references (57 each).
Release tests100/one existing ignored, clippy and154 identical engine objects /
actual-link-input no-C-Borg checks pass. The current native checker passes
calculator30k, spatial2×2k, glyph2,600/lunal1,280, twelve real-source fault
controls and nonrecovering C UBSan. Its ranking/cutoff adapter now explicitly
isolates the cost plane after executing real reverse flow; other scenarios
exercise the corrected real reachability. The #345 revert is a new control.

New living-save semantics skip saving only an actually dead character; failed
living saves remain errors. Mandatory finalsave still rejects death. An initial
normal collection used the mandatory path accidentally and failed before its
death summaries; those runs are excluded, not new independent deaths or AI
crashes. Both negative cases have engine rc3 / wrapper nonzero. Finite-budget
death files are not fabricated. Raw local failed/pilot data remain under
harness/out/tactics; accepted exported cases carry DONE and actual processes.

The fixture constructor snapshot and object/binary hashes are retained.
Later gear-inspection output exits after native load before any Borg/world
action and does not write the original snapshot. No owner save or reviewer
worktree is used or removed. Full raw captures remain local; metadata/hashes
and four curated pre-death saves are durable.

Recheck with normal builds, suite glyph-construction and its save root,
lunal-investigation with synthetic-tactics saves, and normal-live from fresh
birth. Use the paired strict diffs and verify_construction.py; never overwrite
old references or enable stale/unknown implementation overrides.
