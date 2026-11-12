# The interactive session: the Borg plays the dungeon, the player plays the town

The baseline of inventory choices needs a second kind of evidence: the
*player's* choices, in the *real* game, at the *real* town states. The
headless corpus gives the control's choices; this gives a human's.

The shape of it: the player starts the game, activates the Borg (CTRL-Z,
then `y`, then `z` at the Borg command prompt), and buys things in town. The
Borg keeps playing the dungeon. Every time the character is in town, the
controls are the player's; every time it is in the dungeon, they are the
Borg's. When the character dies, the player starts a new character and the
same thing happens again.

## The handover

The engine asks for a key through `inkey_ex` (`angband/src/ui-input.c`).
If the Borg's input hook is installed, the hook's key is used; if the hook
returns no key at all (`EVT_NONE`), the engine falls through to the real
keyboard. That is the whole mechanism:

```c
if (inkey_hack)
{
    ke.key = (*inkey_hack)(inkey_xtra);
    if (ke.key.type != EVT_NONE)
    {
        ...
        return (ke);
    }
}
```

`generate_keypress` (`borg-rs/src/borg.rs`) returns `EVT_NONE` when the
character is in town and `BORG_TOWN_PLAYER=1`. The engine then reads the
keyboard, and the player has the controls.

The handover is *not* the null arm. The null arm (`borg_town_no_stores`)
stops the Borg's shop decisions but leaves the rest of its town behaviour
alone (it still recalls, still equips, still dives). The handover stops the
Borg from doing *anything* in town, which is what makes it safe for the two
of them to share the keyboard: the Borg's keys and the player's keys never
mix, because in town the Borg produces none.

The handover is off unless the environment variable says so, so the headless
arms (the null arm among them) are untouched. The keypress stream of the
headless runs is byte-identical with and without it.

## What is recorded

Per character, in `OUTDIR/char-N`:

- `state.jsonl` - the states the character was in, in town, one record per
  distinct state. The record is the integer projection's shape (schema
  version 3: the state on its own, with no decision in it): the traits, the
  pack, the equipment, the home, and every shop's stock. The clock is the
  game's turn.
- `native/` - the native recording (the C side's `SUPPORT_BORG_RECORDING`,
  which is an opt-in build: `BORG_RECORDING=1 harness/build.sh`). The
  inputs, the frames and the messages. The player's purchases are in the
  messages; the frames carry the character's state at every frame.
- `dump.jsonl` - the Borg's own town decisions. With the handover there are
  none: the Borg makes no decision in town.

## Running it

```
BORG_RECORDING=1 BORG_IMPL=rust harness/build.sh
harness/play.sh /tmp/session 1
```

The game needs a display (the interactive front end this build has is X11;
there is no terminal front end in the cmake build). In the game: CTRL-Z,
then `y`, then `z`. The player buys what they want in town and dives; the
Borg plays the dungeon and recalls when it wants to trade.

`harness/play.sh OUTDIR CHARACTERS` plays several characters, one directory
each: the player quits the game when a character is done, and the next one
starts.

## What was checked

- The headless arms are unaffected: `cargo test --release` and the smoke
  suite's parity (3/3 identical between the C and the Rust build) with the
  handover code in place, and the handover off (the default).
- The state dump in a headless run: 138 records for seed 201 (Human
  Warrior, 8000 turns), every one of them in town (depth 0), the clock the
  game's turn.
- The interactive session, driven under a virtual X server: the game starts,
  the Borg activates, the character stays in town with the controls handed
  back (the state dump fills), and the player's `>` takes the character into
  the dungeon, where the Borg plays on its own.

## What was not checked

- The native recording in the interactive session: the build used for the
  check above has the recording *off* (the cmake default), so the inputs
  were not logged in that run. The recording is the C side's own, already
  used by the replay tooling; the interactive session's use of it is not
  verified here.
- The pairing of the player's purchases with the states they happened in:
  that needs the recording (see below), and the clock for it is still open.
  Store commands do not consume game turns, so several purchases in one
  visit share a turn, and the native recorder's frame counter (the clock
  its messages carry) is not readable from the Rust side today.
- The death and the next character: the harness plays several characters in
  several directories, but the loop was not driven through a death here.
