# Town-visit labels - batch 1

The labelling sheet for the town-visit dump (see [TOWN-DUMP.md](TOWN-DUMP.md)).
Edit this file on the branch and commit it; the labels are read back from
here and become the golden cases of phase 1's usefulness-function suite.

**Convention: a blank `label:` means the Borg's decision is agreed.**
Anything else means "here is what I would have done, and here is the need".

    label: buy the Phase Door - escape
    label: buy nothing - the armour costs a pack slot I want for artifacts
    label: agree

Rules of thumb for the label itself:

- name the **need**, not the object: "escape", "light", "food", "armour",
  "knowledge", "trip" - that is the parameter the usefulness function needs;
- the **non-purchases count**. "I would not buy the armour here" is half the
  signal, and it is what tells a decay apart from a fixed ranking;
- a wrong choice and a wrong reason are different labels. If the Borg bought
  the wrong thing, say what was right instead; if it bought the right thing
  for the wrong reason, say the reason.

The `alternatives` line holds the wares of the shop the decision was made in,
with their prices: the wares that were not bought are the contrastive
signal. The full stock of every shop, and the raw records, are in the dump
(kept out of Git) and in the issue thread on
[#35](https://github.com/diegoami/borg-rs/issues/35).

Three headless runs, 20000 turns each: seed 101 Human Warrior, seed 102 Elf
Mage, seed 103 Dwarf Priest. 195 town visits, 98 buys, 47 purchases carried
out, 18 distinct buy decisions - the Borg re-decides the same ware every turn
while it walks to the shop, so only the first visit of each decision is here.

These are the **current policy's visits**: the states where the current
algorithm is not obviously wrong. The next harvest is the
`borg_town_no_stores` baseline, whose "never improves" states are the
divergence points.


---

## warrior-101 - Human Warrior

### 1. warrior-101 | turn 61 | level 1 | gold 137 | light 2 food 2 cut 0 poison 0
borg_bought: Wooden Torches (5000 turns) @ General Store (price 6)
borg_reason: # Buying 'Wooden Torches (5000 turns)'(j) at 'General Store' (for player 'b')
alternatives: Rations of Food (3); Flasks of Whisky (1); Pints of Fine Wine (2); Flasks of oil (3); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 2. warrior-101 | turn 76 | level 1 | gold 113 | light 2 food 2 cut 0 poison 0
borg_bought: Rations of Food @ General Store (price 3)
borg_reason: # Buying 'Rations of Food'(a) at 'General Store' (for player 'b')
alternatives: Flasks of Whisky (1); Pints of Fine Wine (2); Flasks of oil (3); Wooden Torches (5000 turns) (6); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 3. warrior-101 | turn 81 | level 1 | gold 98 | light 2 food 7 cut 0 poison 0
borg_bought: Scrolls of Phase Door {unseen} @ Alchemy Shop (price 15)
borg_reason: # Buying 'Scrolls of Phase Door {unseen}'(n) at 'Alchemy Shop' (for player 'b')
alternatives: Potions of Cure Light Wounds {unseen} (20); Potion of Cure Serious Wounds {unseen} (70); Potions of Neutralize Poison {unseen} (75); Potion of Berserk Strength (80); Potions of Boldness {unseen} (8); Potions of Resist Cold {unseen} (30); Scrolls of Detect Invisible {unseen} (15); Scrolls of Remove Curse {unseen} (100); Scrolls of Word of Recall (125); Scrolls of Blessing {unseen} (15); Scrolls of Holy Chant {unseen} (40); Scrolls of Monster Confusion {unseen} (30)
label:

### 4. warrior-101 | turn 93 | level 1 | gold 83 | light 2 food 7 cut 0 poison 0
borg_bought: Small Metal Shields [5,+0] @ Armoury (price 36)
borg_reason: # Buying 'Small Metal Shields [5,+0]'(h) at 'Armoury' (for player 'b')
alternatives: Robe [2,+8] (336); Robes [2,+0] (14); Soft Leather Armours [8,+0] (84); Hard Leather Caps [2,+2] (66); Hard Leather Caps [2,+0] (14); Sets of Leather Gloves [1,+0] (6); Sets of Gauntlets [3,+0] (24); Pairs of Leather Boots [2,+0] (14)
label:

### 5. warrior-101 | turn 117 | level 1 | gold 23 | light 2 food 7 cut 0 poison 0
borg_bought: Hard Leather Caps [2,+0] @ Armoury (price 14)
borg_reason: # Buying 'Hard Leather Caps [2,+0]'(m) at 'Armoury' (for player 'b')
alternatives: Robe [2,+8] (336); Robes [2,+0] (14); Soft Leather Armours [8,+0] (84); Small Metal Shields [5,+0] (36); Hard Leather Caps [2,+2] (66); Sets of Leather Gloves [1,+0] (6); Sets of Gauntlets [3,+0] (24); Pairs of Leather Boots [2,+0] (14)
label:


---

## mage-102 - Elf Mage

### 6. mage-102 | turn 25 | level 1 | gold 325 | light 2 food 3 cut 0 poison 0
borg_bought: Rations of Food @ General Store (price 3)
borg_reason: # Buying 'Rations of Food'(c) at 'General Store' (for player 'b')
alternatives: Mushroom of Stoneskin {unseen} (60); Pints of Fine Wine (2); Flasks of oil (3); Wooden Torches (5000 turns) (6); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 7. mage-102 | turn 44 | level 1 | gold 316 | light 2 food 6 cut 0 poison 0
borg_bought: Wooden Torches (5000 turns) @ General Store (price 6)
borg_reason: # Buying 'Wooden Torches (5000 turns)'(j) at 'General Store' (for player 'b')
alternatives: Mushroom of Stoneskin {unseen} (60); Rations of Food (3); Pints of Fine Wine (2); Flasks of oil (3); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 8. mage-102 | turn 50 | level 1 | gold 283 | light 2 food 7 cut 0 poison 0
borg_bought: Scrolls of Phase Door {unseen} @ Alchemy Shop (price 15)
borg_reason: # Buying 'Scrolls of Phase Door {unseen}'(n) at 'Alchemy Shop' (for player 'b')
alternatives: Potions of Cure Light Wounds {unseen} (20); Potions of Cure Serious Wounds {unseen} (70); Potions of Cure Critical Wounds {unseen} (200); Potions of Neutralize Poison {unseen} (75); Potions of Berserk Strength {unseen} (80); Potions of Resist Cold {unseen} (30); Scrolls of Magic Mapping {unseen} (65); Scrolls of Remove Curse {unseen} (100); Scrolls of Word of Recall (125); Scrolls of Recharging {unseen} (200); Scrolls of Blessing {unseen} (15); Scrolls of Holy Chant {unseen} (40); Scrolls of Monster Confusion {unseen} (30)
label:

### 9. mage-102 | turn 56 | level 1 | gold 268 | light 2 food 7 cut 0 poison 0
borg_bought: Sets of Gauntlets [3,+4] @ Armoury (price 176)
borg_reason: # Buying 'Sets of Gauntlets [3,+4]'(o) at 'Armoury' (for player 'b')
alternatives: Chain Mails (-2) [32,+0] (1184); Robes [2,+2] (66); Hard Leather Armour of Resist Fire (-1) [16,+4] {??} (1254); Hard Leather Armour (-1) [16,+0] (336); Leather Scale Mail (-1) [20,+4] (986); Leather Scale Mail (-1) [20,+0] (546); Iron Helms [7,+0] (104); Pairs of Leather Sandals [1,+0] (6); Pairs of Leather Boots [2,+2] (66); Pairs of Leather Boots [2,+0] (14); Pair of Iron Shod Boots [4,+3] (234)
label:

### 10. mage-102 | turn 109 | level 1 | gold 116 | light 2 food 7 cut 0 poison 0
borg_bought: Iron Helms [7,+0] @ Armoury (price 104)
borg_reason: # Buying 'Iron Helms [7,+0]'(n) at 'Armoury' (for player 'b')
alternatives: Chain Mails (-2) [32,+0] (1184); Robes [2,+2] (66); Hard Leather Armour of Resist Fire (-1) [16,+4] {??} (1254); Hard Leather Armour (-1) [16,+0] (336); Leather Scale Mail (-1) [20,+4] (986); Leather Scale Mail (-1) [20,+0] (546); Set of Gauntlets [3,+4] (176); Pairs of Leather Sandals [1,+0] (6); Pairs of Leather Boots [2,+2] (66); Pairs of Leather Boots [2,+0] (14); Pair of Iron Shod Boots [4,+3] (234)
label:

### 11. mage-102 | turn 111 | level 1 | gold 12 | light 2 food 7 cut 0 poison 0
borg_bought: Cloaks [1,+0] @ General Store (price 6)
borg_reason: # Buying 'Cloaks [1,+0]'(m) at 'General Store' (for player 'b')
alternatives: Mushroom of Stoneskin {unseen} (60); Rations of Food (3); Pints of Fine Wine (2); Flasks of oil (3); Wooden Torches (5000 turns) (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:


---

## priest-103 - Dwarf Priest

### 12. priest-103 | turn 95 | level 1 | gold 97 | light 2 food 1 cut 0 poison 0
borg_bought: Wooden Torches (5000 turns) @ General Store (price 6)
borg_reason: # Buying 'Wooden Torches (5000 turns)'(h) at 'General Store' (for player 'b')
alternatives: Rations of Food (3); Pints of Fine Wine (2); Flasks of oil (3); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 13. priest-103 | turn 121 | level 1 | gold 61 | light 2 food 1 cut 0 poison 0
borg_bought: Rations of Food @ General Store (price 3)
borg_reason: # Buying 'Rations of Food'(a) at 'General Store' (for player 'b')
alternatives: Pints of Fine Wine (2); Flasks of oil (3); Wooden Torches (5000 turns) (6); Cloaks [1,+0] (6); Shovels (1d2) (+0,+0) (66); Picks (1d3) (+0,+0) (176); Bolts (1d5) (+0,+0) (1); Arrows (1d4) (+0,+0) (1); Iron Shots (1d4) (+0,+0) (3)
label:

### 14. priest-103 | turn 127 | level 1 | gold 43 | light 2 food 7 cut 0 poison 0
borg_bought: Scrolls of Phase Door {unseen} @ Alchemy Shop (price 15)
borg_reason: # Buying 'Scrolls of Phase Door {unseen}'(o) at 'Alchemy Shop' (for player 'b')
alternatives: Potions of Cure Light Wounds {unseen} (20); Potions of Cure Serious Wounds (70); Potions of Neutralize Poison {unseen} (75); Potion of Restore Life Levels {unseen} (400); Potions of Berserk Strength {unseen} (80); Potions of Boldness {unseen} (8); Potions of Resist Cold {unseen} (30); Scrolls of Magic Mapping {unseen} (65); Scrolls of Detect Invisible {unseen} (15); Scrolls of Remove Curse {unseen} (100); Scrolls of Word of Recall (125); Scrolls of Blessing {unseen} (15); Scrolls of Holy Chant {unseen} (40)
label:

### 15. priest-103 | turn 149 | level 1 | gold 28 | light 2 food 7 cut 0 poison 0
borg_bought: Metal Caps [3,+0] @ Armoury (price 24)
borg_reason: # Buying 'Metal Caps [3,+0]'(r) at 'Armoury' (for player 'b')
alternatives: Metal Scale Mail (-2) [38,+5] (2964); Metal Scale Mail (-2) [38,+2] (2250); Metal Scale Mail (-2) [38,+0] (1886); Soft Leather Armour of Resist Cold [8,+6] {??} (696); Leather Scale Mail (-1) [20,+3] (864); Leather Scale Mail (-1) [20,+0] (546); Leather Shields [8,+0] (66); Hard Leather Cap [2,+6] (234); Hard Leather Caps [2,+2] (66); Iron Helm [7,+0] (104); Pairs of Leather Sandals [1,+0] (6)
label:

### 16. priest-103 | turn 838 | level 2 | gold 144 | light 2 food 10 cut 0 poison 0
borg_bought: Scrolls of Phase Door @ Alchemy Shop (price 15)
borg_reason: # Buying 'Scrolls of Phase Door'(q) at 'Alchemy Shop' (for player 'b')
alternatives: Potions of Cure Light Wounds {unseen} (20); Potions of Cure Serious Wounds (70); Potions of Neutralize Poison (75); Potion of Restore Life Levels {unseen} (400); Potion of Heroism (25); Potions of Berserk Strength {unseen} (80); Potions of Boldness {unseen} (8); Potions of Resist Cold {unseen} (30); Scrolls of Magic Mapping {unseen} (65); Scroll of Treasure Detection (70); Scrolls of Detect Invisible {unseen} (15); Scrolls of Remove Curse {unseen} (100); Scrolls of Word of Recall (125); Scrolls of Blessing (15); Scrolls of Holy Chant {unseen} (40)
label:

### 17. priest-103 | turn 839 | level 2 | gold 129 | light 2 food 10 cut 0 poison 0
borg_bought: Leather Shields [8,+0] @ Armoury (price 66)
borg_reason: # Buying 'Leather Shields [8,+0]'(n) at 'Armoury' (for player 'b')
alternatives: Metal Scale Mail (-2) [38,+5] (2964); Metal Scale Mail (-2) [38,+2] (2250); Metal Scale Mail (-2) [38,+0] (1886); Soft Leather Armour of Resist Cold [8,+6] {??} (696); Leather Scale Mail (-1) [20,+3] (864); Leather Scale Mail (-1) [20,+0] (546); Hard Leather Cap [2,+6] (234); Hard Leather Caps [2,+2] (66); Metal Caps [3,+0] (24); Iron Helm [7,+0] (104); Pairs of Leather Sandals [1,+0] (6)
label:

### 18. priest-103 | turn 845 | level 2 | gold 63 | light 2 food 10 cut 0 poison 0
borg_bought: Ring of Feather Falling {unseen} @ Magic Shop (price 50)
borg_reason: # Buying 'Ring of Feather Falling {unseen}'(v) at 'Magic Shop' (for player 'b')
alternatives: Wands of Magic Missile (41 charges) {unseen} (340); Wands of Stinking Cloud (69 charges) {unseen} (640); Wands of Slow Monster (20 charges) {unseen} (675); Wands of Confuse Monster (20 charges) {unseen} (675); Wands of Stone to Mud (67 charges) {unseen} (510); Staves of Detect Invisible (67 charges) {unseen} (370); Staff of Detect Evil (14 charges) {unseen} (595); Staves of Mapping (33 charges) {unseen} (870); Staff of Light (19 charges) {unseen} (487); Rings of Searching {unseen} (150); Ring of Open Wounds {unseen} (546); Amulets of Resist Acid {unseen} (204); Amulets of Slow Digestion {unseen} (66)
label:

