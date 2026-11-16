# What the Borg actually bought in town

Confirmed transactions from the corrected live full suite: 45 starts per arm, 100000 turns, nine classes. Source runtime ac63c5e. Every retail purchase has a native “You bought … for … gold” receipt and a matching per-key purse decrease. These are executed purchases, not plans.

| Arm | Confirmed purchases | Units | Gold spent | Home deposits¹ | Home withdrawals¹ | Market sales |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| current | 1714 | 1714 | 38339 | 176 | 7 | 974 |
| no-shops | 0 | 0 | 0 | 0 | 0 | 0 |
| utility-reference | 1373 | 1373 | 35458 | 490 | 3 | 675 |
| utility-cash50 | 1367 | 1367 | 33661 | 505 | 4 | 689 |

## Concrete example: Human Warrior, seed 1000

These are the first confirmed transactions, including the first seven purchases. Short dungeon detours occur between some town actions; this is not labelled as one uninterrupted visit.

| Game turn | Action | Item | Qty | Gold paid | Purse before → after |
| ---: | --- | --- | ---: | ---: | --- |
| 61 | sell | Potion of Berserk Strength | 1 | 0 | 146 → 146 |
| 191 | home_store | Scroll of Word of Recall | 1 | 0 | 146 → 146 |
| 201 | home_store | Wooden Torch (5000 turns) | 1 | 0 | 146 → 146 |
| 409 | buy | Potion of Cure Light Wounds | 1 | 20 | 146 → 126 |
| 649 | buy | Scroll of Phase Door | 1 | 15 | 126 → 111 |
| 659 | buy | Potion of Cure Light Wounds | 1 | 20 | 111 → 91 |
| 659 | buy | Potion of Boldness | 1 | 8 | 91 → 83 |
| 779 | home_store | Potion of Boldness | 1 | 0 | 83 → 83 |
| 909 | buy | Scroll of Detect Invisible | 1 | 15 | 83 → 68 |
| 1039 | home_store | Scroll of Detect Invisible | 1 | 0 | 68 → 68 |
| 1169 | buy | Scroll of Blessing | 1 | 15 | 68 → 53 |
| 1299 | home_store | Scroll of Blessing | 1 | 0 | 53 → 53 |
| 1429 | buy | Scroll of Phase Door | 1 | 15 | 53 → 38 |
| 4237 | buy | Potion of Cure Light Wounds | 1 | 20 | 48 → 28 |
| 4547 | buy | Ration of Food | 1 | 3 | 28 → 25 |
| 4730 | home_store | Ration of Food | 2 | 0 | 25 → 25 |
| 4910 | buy | Ration of Food | 1 | 3 | 25 → 22 |
| 4920 | buy | Ration of Food | 1 | 3 | 22 → 19 |

The trace shows Boldness, Detect Invisible and Blessing bought and then deposited. It also deposits two rations and shortly buys rations again. These behaviours are visible in the original receipts; the report does not hide them or claim that the planner is already efficient.


## Open a class’s actual shopping ledger

Each page has item totals and all confirmed retail purchases and the normal trade-executor sales/Home transfers in order, separated by seed.

| Class | Current | Utility reference | Cash priority 50 |
| --- | --- | --- | --- |
| Warrior | [current](current/warrior.md) | [utility-reference](utility-reference/warrior.md) | [utility-cash50](utility-cash50/warrior.md) |
| Mage | [current](current/mage.md) | [utility-reference](utility-reference/mage.md) | [utility-cash50](utility-cash50/mage.md) |
| Druid | [current](current/druid.md) | [utility-reference](utility-reference/druid.md) | [utility-cash50](utility-cash50/druid.md) |
| Priest | [current](current/priest.md) | [utility-reference](utility-reference/priest.md) | [utility-cash50](utility-cash50/priest.md) |
| Necromancer | [current](current/necromancer.md) | [utility-reference](utility-reference/necromancer.md) | [utility-cash50](utility-cash50/necromancer.md) |
| Paladin | [current](current/paladin.md) | [utility-reference](utility-reference/paladin.md) | [utility-cash50](utility-cash50/paladin.md) |
| Rogue | [current](current/rogue.md) | [utility-reference](utility-reference/rogue.md) | [utility-cash50](utility-cash50/rogue.md) |
| Ranger | [current](current/ranger.md) | [utility-reference](utility-reference/ranger.md) | [utility-cash50](utility-cash50/ranger.md) |
| Blackguard | [current](current/blackguard.md) | [utility-reference](utility-reference/blackguard.md) | [utility-cash50](utility-cash50/blackguard.md) |

## Most gold spent: current

| Item / variant | Units | Gold | Purchases |
| --- | ---: | ---: | ---: |
| Scroll of Phase Door | 261 | 3915 | 261 |
| Potion of Cure Light Wounds | 105 | 2100 | 105 |
| Ring of Searching | 19 | 2048 | 19 |
| Wooden Torch (5000 turns) | 328 | 1968 | 328 |
| Studded Leather Armour (-1) [12,+0] | 12 | 1800 | 12 |
| Amulet of Slow Digestion | 25 | 1650 | 25 |
| Iron Helm [7,+0] | 15 | 1560 | 15 |
| Ring of Feather Falling | 24 | 1200 | 24 |
| Scroll of Recharging | 6 | 1200 | 6 |
| Dagger of Slay Animal (1d5) (+4,+6) | 1 | 1184 | 1 |
| Leather Shield [8,+0] | 17 | 1122 | 17 |
| Amulet of Resist Acid | 5 | 1020 | 5 |
| Flask of oil | 312 | 936 | 312 |
| Ration of Food | 301 | 903 | 301 |
| Soft Leather Armour [8,+0] | 9 | 756 | 9 |
| Short Sword of Slay Orc (1d7) (+2,+5) | 1 | 750 | 1 |
| Wand of Magic Missile (12 charges) | 2 | 640 | 2 |
| Scroll of Word of Recall | 5 | 625 | 5 |
| Scroll of Teleportation | 5 | 600 | 5 |
| Ring of Protection [+10] | 2 | 532 | 2 |
| Scroll of Remove Curse | 5 | 500 | 5 |
| Studded Leather Armour (-1) [12,+5] | 1 | 500 | 1 |
| Metal Cap [3,+0] | 18 | 432 | 18 |
| Hard Leather Cap of Intelligence [2,+4] | 1 | 414 | 1 |
| Set of Gauntlets [3,+0] | 17 | 408 | 17 |

## Most gold spent: utility-reference

| Item / variant | Units | Gold | Purchases |
| --- | ---: | ---: | ---: |
| Scroll of Phase Door | 321 | 4815 | 321 |
| Potion of Cure Light Wounds | 209 | 4180 | 209 |
| Scroll of Remove Curse | 19 | 1900 | 19 |
| Amulet of Slow Digestion | 27 | 1782 | 27 |
| Ring of Searching | 19 | 1766 | 19 |
| Ring of Feather Falling | 32 | 1600 | 32 |
| Scroll of Holy Chant | 36 | 1440 | 36 |
| Scroll of Magic Mapping | 21 | 1365 | 21 |
| Potion of Cure Serious Wounds | 18 | 1260 | 18 |
| Potion of Resist Cold | 37 | 1110 | 37 |
| Potion of Resist Heat | 35 | 1050 | 35 |
| Scroll of Monster Confusion | 35 | 1050 | 35 |
| Potion of Berserk Strength | 13 | 1040 | 13 |
| Amulet of Resist Acid | 5 | 1020 | 5 |
| Mushroom of Stoneskin | 15 | 900 | 15 |
| Mushroom of Purging | 14 | 840 | 14 |
| Potion of Cure Critical Wounds | 3 | 600 | 3 |
| Scroll of Detect Invisible | 37 | 555 | 37 |
| Ration of Food | 174 | 522 | 174 |
| Scroll of Blessing | 32 | 480 | 32 |
| Metal Cap of Wisdom [3,+3] | 1 | 414 | 1 |
| Scroll of Recharging | 2 | 400 | 2 |
| Wooden Torch (5000 turns) | 64 | 384 | 64 |
| Scroll of Remove Hunger | 38 | 380 | 38 |
| Potion of Neutralize Poison | 5 | 375 | 5 |

## Most gold spent: utility-cash50

| Item / variant | Units | Gold | Purchases |
| --- | ---: | ---: | ---: |
| Scroll of Phase Door | 323 | 4845 | 323 |
| Potion of Cure Light Wounds | 200 | 4000 | 200 |
| Amulet of Slow Digestion | 31 | 2046 | 31 |
| Scroll of Remove Curse | 19 | 1900 | 19 |
| Scroll of Magic Mapping | 26 | 1690 | 26 |
| Ring of Feather Falling | 32 | 1600 | 32 |
| Scroll of Holy Chant | 36 | 1440 | 36 |
| Mushroom of Stoneskin | 23 | 1380 | 23 |
| Potion of Cure Serious Wounds | 17 | 1190 | 17 |
| Ring of Searching | 15 | 1188 | 15 |
| Mushroom of Purging | 19 | 1140 | 19 |
| Potion of Resist Heat | 36 | 1080 | 36 |
| Scroll of Monster Confusion | 36 | 1080 | 36 |
| Amulet of Resist Acid | 5 | 1020 | 5 |
| Potion of Resist Cold | 32 | 960 | 32 |
| Potion of Berserk Strength | 11 | 880 | 11 |
| Potion of Cure Critical Wounds | 3 | 600 | 3 |
| Scroll of Detect Invisible | 35 | 525 | 35 |
| Scroll of Blessing | 34 | 510 | 34 |
| Ration of Food | 165 | 495 | 165 |
| Scroll of Remove Hunger | 40 | 440 | 40 |
| Wooden Torch (5000 turns) | 69 | 414 | 69 |
| Scroll of Recharging | 2 | 400 | 2 |
| Potion of Neutralize Poison | 5 | 375 | 5 |
| Potion of Boldness | 36 | 288 | 36 |

¹ Home columns cover confirmed transfers through the normal trade executors. The separate equipment optimiser can transfer worn gear directly without these commands; those auto-swaps are not claimed complete here. Other inventory messages are not automatically labelled as Home transfers.


## Evidence and limits

[Complete machine-readable transactions](transactions.csv) · [audit and original log checksums](audit.json).
The engine prints a numeric quantity for every stack larger than one; an unprefixed armour/mass-noun receipt therefore means one physical item. Post-purchase inventory descriptions supply identified names/modifiers when available. Their resulting stack counts are never reused as purchase quantities.
The ledger uses original messages/state logs from the corrected batch, not newly simulated visits. Inventory descriptions in Borg shop notes may be plural because they describe shop stock; quantities come from native receipts. Unknown flavour names in receipts are paired with the Borg’s contemporaneous shop description.
Market “You had …” receipts mean zero selling proceeds under birth_no_selling. Home deposits require native “You drop …” confirmation after a deposit command; they are not sales. Home withdrawal quantities use observed Home stock deltas for live arms, or the explicitly labelled legacy default of one; an unverified quantity is left unknown. A Home stock delta is not inferred from a resulting pack total.
Unconfirmed commands retained separately: 0. They are excluded from completed transaction totals. Source line numbers and exact confirmation text remain in the CSV. Gear variants retain their modifiers rather than being merged into an invented average item.
