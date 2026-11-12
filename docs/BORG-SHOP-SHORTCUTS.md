# The Borg's shop shortcuts

The Borg has no way to be told which shop to visit. The strategy picks one
(`borg_choose_shop` in `borg-rs/src/borg_think_store.rs`), the player walks to
one, and nothing else exists. This document is the analysis of how to add a
shortcut, which key to use, and what was built.

## What the game already gives us

**The Borg's own machinery.** `st.borg.goal.shop` is the store the Borg is
heading for, and `borg_flow_shop_entry` (`borg-rs/src/borg_flow_misc.rs`)
flows the Borg to that store's entrance and presses `5` to open it. "Switch
to shop N" is therefore a matter of setting that one field.

**The store order.** `square_shopnum()` returns the store index (0-based) and
`stores[i].feat` its feature, so the indexes are the terrain order:

| Borg's number | store index | store |
|---|---|---|
| 1 | 0 | General Store |
| 2 | 1 | Armoury |
| 3 | 2 | Weapon Smiths |
| 4 | 3 | Bookseller |
| 5 | 4 | Alchemy Shop |
| 6 | 5 | Magic Shop |
| 7 | 6 | Black Market |
| 8 | 7 | Home |

The Borg already speaks this numbering in its own log ("Currently in
preferred store 'N'" prints `goal.shop + 1`, `borg_think_store.rs:425`), so
the shortcut uses it rather than inventing a second one. The shops are not in
a row in the town (`town_gen_layout` in `angband/src/gen-cave.c` puts them in
lots around the crossroads), so "the next shop" is by store index, never by
visual order.

## The key question

The owner's shape was `/1`, `/2`, `/3`, ..., `/+`, `/-`, `/>`: a prefix and a
selector. Two things decide the prefix.

**ESC cannot be the prefix.** It is the cancel key of every prompt in the
game and of the Borg's own menu (`get_com` returns "not ok" on ESC). A
prefix that swallows ESC would break the cancel convention everywhere.

**The four proposed keys are all taken in the game.** The game's command
lookup is a single-key table (`cmd_lookup` over `converted_list[mode][key]`
in `angband/src/ui-game.c`); there is no general prefix mechanism, only the
three built-in prefixes in `textui_get_command` (`angband/src/ui-input.c`):
`0` (a count), `\` (keymap bypass) and `^` (a control character).

| key | in the game | in the Borg's command menu |
|---|---|---|
| `/` | "Identify symbol" (`ui-game.c:190`) | **free** |
| `+` | "Alter a grid" (`ui-game.c:222`) | **free** |
| `-` | "Walk into a trap" (`ui-game.c:160`) | **free** |
| `>` | "Go down staircase" (`ui-game.c:155`) | **free** |

So the owner's exact keys work as a prefix **in the Borg's own command
menu** (reached with `^Z`; its keys are `?$zxmGID@Ffucs^ythpP!C0wSloRv`, so
`/`, `+`, `-`, `>` and every digit but `0` are unused) and would each collide
with an existing command **in the game**.

At the game level the free printable ASCII is `! # $ % & ' ( 0-9 B J N Y ^ \``
and `y`; the digits are used by the store and inventory interfaces, so the
practical game-level prefixes are `#`, `%`, `&`, `'` and `(`.

## What was built, and why there

The shortcut lives in **the Borg's command menu**, with `/` as the prefix:

* the prefix is free there, so no existing command is unbound;
* the menu is a `get_com` prompt, so a prefix costs one extra key read;
* the Borg's mechanism for it already exists (`goal.shop` +
  `borg_flow_shop_entry`), so the change is small and Rust-only;
* it needs no change to the C engine, so it is not a fork decision.

The player-side variant ("the *player* walks to shop N") is a different
thing and is **not** in this change: it needs a new command in the C engine
(a new game feature in the fork, which is the owner's call), and the pathing
it would use lives in `angband/src/player-path.c` (`find_path`,
`pathfind_direction_to`). If the owner wants that too, the prefix there
would have to be one of the free game keys above, not `/`.

### The commands

In the Borg's command menu (`^Z`, then `/`):

| keys | effect |
|---|---|
| `/1` .. `/8` | visit that store, by the Borg's own numbering |
| `/+` | the next store (wraps around) |
| `/-` | the previous store (wraps around) |
| `/>` | drop the choice: the strategy picks the store again |
| `ESC` | cancel, the choice is left as it was |

The choice is an override (`borg.shop_override`): it outranks the strategy's
own pick until the Borg has been to that store, and then it is dropped and
the strategy decides again. It is ignored while the negative-baseline arm
(`borg_town_no_stores`) is on, because that arm's contract is that the Borg
never walks into a shop.

### In the interactive session

With the handover on (`BORG_TOWN_PLAYER=1`) the player has the keyboard in
town, so a store the player asks for is walked by the Borg: the handover
hands the Borg one step at a time toward the store, opens the store when it
arrives, and gives the keyboard back to the player there. The player's keys
and the Borg's keys still never mix: the Borg only presses keys while it is
walking to a store a human asked for.

The walk is the Borg's own flow, so it needs the Borg to know the way: a
store it has never seen gives "The Borg does not know where the General
Store is.", and a store it cannot path to gives "The Borg cannot find a way
to the General Store." In both cases the choice is dropped and the player
can ask again. The Borg learns the town from the screen, as the player
walks it: the handover updates the Borg's map on every key the game asks
for, so the store doors the player has seen are the ones the Borg can walk
to.

## What was checked

* `cargo test --release`: 139 passed, 0 failed, 1 ignored (6 new tests for
  the selector).
* `cargo clippy --release --all-targets -- -D warnings`: clean.
* `harness/diff --suite smoke --expect-impl rust`: 3/3 identical (the menu
  is only entered by hand, so the keypress stream cannot change).
* **Driven in a live session** (Xvfb, the Rust build, the handover on):
  * `^Z` then `/` shows the prompt `Shop (1-9, +/- to step, > to clear):`;
  * `1` gives "The Borg will visit the General Store.", the Borg walks
    across the town and the **General Store opens** (the store screen with
    its wares);
  * `+` steps to the next store: the Borg walks there and the **Armoury
    opens**;
  * `-` steps back: the **General Store opens** again;
  * `>` gives "The Borg chooses the shop itself again." and the choice is
    dropped;
  * a store the Borg has never seen gives "The Borg does not know where
    the General Store is." (seen live before the Borg had seen the town).

## What was not checked

* The C build's command menu has no such command: the shortcut is Rust-only
  and the C build's `^Z` menu is unchanged. The C and Rust keypress streams
  are identical in the harness, but a human opening the menu in the C build
  would not find it.
* The store the Borg opens is the one the player asked for; what the Borg
  *does* in the store (the Borg's own shopping) was not exercised here,
  because the handover leaves the keyboard with the player in town.
* The interactive walk was driven with `xdotool` against an Xvfb display;
  the key delivery there is unreliable, so the session was driven step by
  step with screenshots rather than scripted end to end.

