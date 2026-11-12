# The town-visit dump

The town-visit dump is an observation-only record of every town visit the
Borg makes: the state it was in, the stock of the shops, and the decision
the current policy took. It exists so that the town inventory strategy can
be written against labelled examples instead of against a guess.

The dump changes nothing. It never presses a key, never changes a goal,
never changes a setting and never changes what the Borg decides, so the
keypress stream of a run with the dump enabled is the stream of a run
without it. The C/Rust parity suites stay valid.

## Running it

The dump is off unless the environment variable `BORG_TOWN_DUMP` names a
file:

```sh
BORG_TOWN_DUMP=OUT/town-dump.jsonl BORG_IMPL=rust \
    harness/run.sh OUT 42 1 5 2000
```

The file is created if it does not exist and appended to if it does. One
JSON object per line.

To read it:

```sh
tools/town_dump_report.py OUT/town-dump.jsonl --limit 20
tools/town_dump_report.py OUT/town-dump.jsonl --limit 20 --markdown
```

The renderer prints one block per visit and leaves a `label:` line for the
owner. `--full` prints every shop's stock rather than only the shop of the
decision, `--action buy` keeps only the buys, and `--distinct` keeps only
the first visit of each distinct decision: the Borg re-decides the same
thing every turn while it walks to the shop, so the repeats are the same
decision again and only the first visit carries its reason.

## The records

A `decision` record is one town visit:

| field | what it holds |
| --- | --- |
| `seq` | the record number over the run |
| `turn`, `depth`, `level`, `hp`, `gold` | where the Borg was |
| `race`, `class`, `name` | who the Borg was |
| `traits` | `light`, `food`, `cut`, `poisoned` |
| `pack`, `equipment` | the pack and the worn kit, with the cost of each |
| `home` | what the home held |
| `shops` | every shop, and every ware in it with its price |
| `decision` | `action`, the goal ware or item, the shop, and `reason` |

A `purchase` record is a transaction that was actually carried out: the
ware, the price, and the gold before it.

The `reason` is the Borg's own note, the same text that reaches the
message log. It is the closest thing the Borg has to a justification, so it
is kept verbatim rather than paraphrased.

## What it is for

A zoo sweep can find the best weights of a usefulness function of a given
form. It cannot tell whether the form is right: whether the need list is
complete, whether the decay has the right shape, whether the alternatives
are compared the right way round, or which needs the owner would never
trade. Those are the questions the labels answer.

So the label is not "buy the Phase Door" (a list of objects) but "buy the
Phase Door - the escape need" (a parameter). And the label covers the
non-purchases too: "I would not buy the armour here" is half the signal,
and it is what tells a decay apart from a fixed ranking. That is why the
record keeps the whole shop stock and not only the wares that were bought.

Two limits are worth keeping in mind:

* The owner's choices are taste, not survival. If "buy armour at level 3"
  is wrong because it costs a pack slot, the dataset teaches the wrong
  thing and nothing inside the dataset reveals it. The zoo stays the judge;
  the dataset is a prior on the form, never a substitute.
* The visits are the current policy's visits: the states where the current
  algorithm is not obviously wrong. Harvesting from several policies,
  including the `borg_town_no_stores` baseline, widens the sample.

## Where the code is

* `borg-rs/src/borg_town_dump.rs` - the dump.
* `borg-rs/src/borg_think_store.rs` - `borg_choose_shop` records the
  decision; `note` records the reason.
* `borg-rs/src/borg_store_buy.rs` - `borg_think_shop_buy` records the
  purchase.
* `tools/town_dump_report.py` - the renderer.
