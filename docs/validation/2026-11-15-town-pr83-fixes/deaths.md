# Natural deaths in the combined-candidate checks

All eight natural-death attempts in the 33-game matrix are listed. Replays and selected seeds are not independent lives. Exact source, build/input hashes and actual native before-death observations are in [runs](runs.json) and [observations](death-observations.json). Frames are sampled at native key boundaries; the shown last live frame is not relabeled as the fatal action. Full movies/logs and final saves remain in the raw validation root. No alternate-action rescue is claimed.

## pilot-necromancer-813430662-rs-kit-trip-default

Cause: **burning to a cinder in lava**, death at world turn 60088. Last recorded live frame: turn 60088, HP 20, mana 3, depth 0.

The last messages show town flight from a perceived threat and a backing-up step immediately followed by lethal lava damage. This is a terrain/retreat candidate, not evidence that another healing purchase would prevent the step.

Follow-up: Trace the public terrain and retreat-choice gate before proposing any tactical correction. No extra tactic or default is changed in this bundle.

```text
4038 60088 0 # Tracking a monster 'pitiful-looking beggar' at (7,39) from (6,38)
4038 60088 0 # Tracking a monster 'filthy street urchin' at (6,38) from (6,39)
4038 60088 0 # Scary guy on level.
4038 60088 0 # Loc:6,37 Dep:0 Lev:2 HP:20/21 SP:3/4 Danger:p=154
4038 60088 0 # Fleeing and leaving the level. (scaryguy)
4038 60088 0 # Backing up to 36,5 (154 > 0)
4040 60088 0 The lava burns you!
4040 60088 8 You die.
4041 60088 0 You have a Set of Leather Gloves of Free Action [1,+3] (o).
4041 60088 0 Score not registered for borgs.
4041 60088 0 # Aborting (death).
4041 60088 0 # Aborting (player died).
```

## pilot-necromancer-825451786-rs-kit-trip-control

Cause: **a metallic green centipede**, death at world turn 6908. Last recorded live frame: turn 6904, HP 1, mana 2, depth 1.

The native tail shows close combat, acid and stings. Healing stock at the last live frame does not prove an available healing action would survive the next attack. This is corrected control, not the live shopper.

Follow-up: If prioritized later, inspect healing-versus-attack/retreat gating on the retained original trajectory. No extra tactic or default is changed in this bundle.

```text
951 6899 0 # Performing attack type 1 with value 1011.
951 6899 0 # Facing metallic green centipede at (9,81) who has 2 Hit Points.
951 6899 0 # Attacking with weapon 'Main Gauche (1d5) (+0,+0)'
953 6899 2 You hit the metallic green centipede.
953 6904 44 The metallic green centipede crawls on you.
954 6904 0 You are covered in acid!
954 6904 40 The metallic green centipede stings you.
955 6908 44 The metallic green centipede crawls on you.
956 6908 8 You die.
957 6908 0 Score not registered for borgs.
957 6908 0 # Aborting (death).
957 6908 0 # Aborting (player died).
```

## pilot-necromancer-485772878-rs-kit-trip-control

Cause: **a small kobold**, death at world turn 14792. Last recorded live frame: turn 14790, HP 3, mana 2, depth 1.

The Borg is at 3/20 HP, fleeing toward up stairs at cost 13, immediately before the kobold hit. This demonstrates unsuccessful retreat, not a purchase execution failure.

Follow-up: Keep as a corrected-control regression outcome; investigate route exposure and available resources separately. No extra tactic or default is changed in this bundle.

```text
1293 14790 0 # HIT_BY:The small kobold (+)
1293 14790 0 # HIT_BY:The small kobold (3)
1293 14790 0 # Scary guy on level.
1293 14790 0 # Loc:17,48 Dep:1 Lev:2 HP:3/20 SP:2/4 Danger:p=5
1293 14790 0 # Fleeing and leaving the level. (scaryguy)
1293 14790 0 # Flowing toward up-stairs at cost 13
1293 14790 0 # Desperate for Stairs (three)
1294 14792 34 The small kobold hits you.
1294 14792 8 You die.
1295 14792 0 Score not registered for borgs.
1295 14792 0 # Aborting (death).
1295 14792 0 # Aborting (player died).
```

## pilot-ranger-285192323-rs-kit-trip-default

Cause: **poison**, death at world turn 20970. Last recorded live frame: turn 20959, HP 1, mana 0, depth 0.

The native tail shows continued stair flight before poison death. The packing repair does not solve every resource/poison/escape decision; selected-case rescue from the simpler quiver-only probe is not reproduced by this combined candidate.

Follow-up: Inspect acquisition and consumption of poison cures and Phase Door over the full poisoned interval before choosing a class parameter change. No extra tactic or default is changed in this bundle.

```text
3237 20959 0 # Scary guy on level.
3237 20959 0 # Fleeing and leaving the level. (scaryguy)
3237 20959 0 # Fleeing and leaving the level. (Looking for any stair)
3237 20959 0 # Flowing toward stairs at cost 8
3238 20969 0 # Scary guy on level.
3238 20969 0 # Fleeing and leaving the level. (scaryguy)
3238 20969 0 # Fleeing and leaving the level. (Looking for any stair)
3238 20969 0 # Flowing toward stairs at cost 7
3239 20970 8 You die.
3240 20970 0 Score not registered for borgs.
3240 20970 0 # Aborting (death).
3240 20970 0 # Aborting (player died).
```

## pilot-ranger-1770524781-rs-kit-trip-control

Cause: **a crow**, death at world turn 23425. Last recorded live frame: turn 23425, HP 2, mana 2, depth 2.

The native tail shows repeated missed melee attacks and crow bites. This is the inherited shopper/control path with the mirrored light correction.

Follow-up: Retain as a control death; any tactical explanation needs the actual danger/action gates. No extra tactic or default is changed in this bundle.

```text
3348 23417 0 # Performing attack type 1 with value 32.
3348 23417 0 # Facing crow at (40,49) who has 9 Hit Points.
3348 23417 0 # Attacking with weapon 'Main Gauche (1d5) (+0,+0)'
3350 23417 3 You miss the crow.
3350 23420 0 The crow misses you.
3350 23420 39 The crow bites you.
3350 23425 0 The crow misses you.
3351 23425 39 The crow bites you.
3351 23425 8 You die.
3352 23425 0 Score not registered for borgs.
3352 23425 0 # Aborting (death).
3352 23425 0 # Aborting (player died).
```

## pilot-ranger-1021027760-rs-kit-trip-default

Cause: **a cave spider**, death at world turn 16906. Last recorded live frame: turn 16901, HP 2, mana 0, depth 1.

The last action is melee against a cave spider reported with 2 HP, followed by a miss and bites. This seed survived under the original default, so the bundle has a selected new death. A zero-ammunition claim alone would not establish the cause.

Follow-up: Review allocation and finishing-attack/escape choices; do not promote from the rescue examples alone. No extra tactic or default is changed in this bundle.

```text
2845 16901 0 # Adjacent to monster on Scaryguy Level.
2845 16901 0 # Adjacent to monster on Scaryguy Level.
2845 16901 0 # Performing attack type 1 with value 57.
2845 16901 0 # Facing cave spider at (54,57) who has 2 Hit Points.
2845 16901 0 # Attacking with weapon 'Main Gauche (1d5) (+0,+0)'
2847 16901 3 You miss something.
2847 16902 39 Something bites you.
2847 16906 39 Something bites you.
2847 16906 8 You die.
2848 16906 0 Score not registered for borgs.
2848 16906 0 # Aborting (death).
2848 16906 0 # Aborting (player died).
```

## pilot-ranger-763715166-rs-kit-trip-control

Cause: **Bullroarer the Hobbit**, death at world turn 29789. Last recorded live frame: turn 29789, HP 5, mana 0, depth 1.

The native tail shows melee with Bullroarer and repeated incoming hits. A last live snapshot is not a proof of a safe alternative.

Follow-up: Retain the original build/input/native observations; prioritize any follow-up through a separate tactical assessment. No extra tactic or default is changed in this bundle.

```text
3873 29782 0 # Performing attack type 1 with value 194.
3873 29782 0 # Facing Bullroarer the Hobbit at (24,77) who has 54 Hit Points.
3873 29782 0 # Attacking with weapon 'Main Gauche (1d5) (+0,+0)'
3875 29782 2 You hit Bullroarer the Hobbit.
3875 29784 34 Bullroarer the Hobbit hits you.
3875 29784 34 Bullroarer the Hobbit hits you.
3876 29789 34 Bullroarer the Hobbit hits you.
3876 29789 34 Bullroarer the Hobbit hits you.
3877 29789 8 You die.
3878 29789 0 Score not registered for borgs.
3878 29789 0 # Aborting (death).
3878 29789 0 # Aborting (player died).
```

## fresh-class-2

Cause: **a giant white centipede**, death at world turn 11999. Last recorded live frame: turn 11999, HP 1, mana 2, depth 1.

The native tail shows unarmed melee against a giant white centipede, then cold and sting damage. The last live frame has healing stock. This reappears in the earlier nine-class validation; no causal shopper fix is established.

Follow-up: Inspect weapon disposition, healing and retreat gates if this case is selected for follow-up; do not invent a cure from this snapshot. No extra tactic or default is changed in this bundle.

```text
2176 11993 0 # Adjacent to monster on Scaryguy Level.
2176 11993 0 # Performing attack type 1 with value 19.
2176 11993 0 # Facing giant white centipede at (34,139) who has 10 Hit Points.
2176 11993 0 # Attacking with weapon ''
2178 11993 2 You punch the giant white centipede.
2178 11999 44 The giant white centipede crawls on you.
2179 11999 0 You are covered with frost!
2179 11999 40 The giant white centipede stings you.
2180 11999 8 You die.
2181 11999 0 Score not registered for borgs.
2181 11999 0 # Aborting (death).
2181 11999 0 # Aborting (player died).
```
