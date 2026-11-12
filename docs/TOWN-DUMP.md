# The town-visit dump

The town-visit dump is an observation-only record of every town visit the
Borg makes: the state it was in, the stock of the shops, and the decision
the current policy took.

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

The renderer prints one block per visit. `--full` prints every shop's stock
rather than only the shop of the decision, `--action buy` keeps only the
buys, and `--distinct` keeps only the first visit of each distinct
decision: the Borg re-decides the same thing every turn while it walks to
the shop, so the repeats are the same decision again and only the first
visit carries its reason.

In `--markdown` the state and the stock are folded into a `<details>`
block and the decision and the reason stay visible, so a batch of visits
can be scanned on one screen.

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

## The integer projection

`BORG_TOWN_DUMP_INTEGERS=<file>` writes a second file: the same records
with **integers only**, a fixed field order and a version tag.

```sh
BORG_TOWN_DUMP=OUT/town-dump.jsonl \
BORG_TOWN_DUMP_INTEGERS=OUT/town-dump.i32 \
    harness/run.sh OUT 42 1 5 2000
```

The descriptive records carry names, and a name is not comparable between
two runs; an integer is. The projection is what the record schema (P13)
asks for - a versioned, machine-readable line, integers only, a stable
field order, a total ordering - so it is here to be specified against, and
to make the two sides of a shadow-mode comparison (today's action and the
plan's) diffable as numbers.

It is not the record schema: P13 wants the line to carry the plan's
decision as well, and to be replayable from its own logged inputs. This
one carries today's side only.

| field | what it holds |
| --- | --- |
| `v` | the schema version |
| `seq` | the record number over the run; the records are totally ordered by it |
| `turn`, `depth`, `level`, `chp`, `mhp`, `gold` | where the Borg was |
| `race`, `class` | the game's indexes, not the names; `-1` if the class is not set |
| `light`, `food`, `cut`, `poison` | the traits |
| `action` | 0 visit, 1 buy, 2 take_home, 3 sell, 4 sell_home |
| `shop`, `ware`, `item`, `price` | the goal, and the price of the goal ware |
| `pack`, `equip`, `home` | `[slot, kind, subkind, quantity, value]` each |
| `shops` | `[shop, [[slot, kind, subkind, quantity, price], ...]]` each |

The file holds two record shapes. A **decision** record is the table above; a
**purchase** record is the transaction that was carried out: `v`, `seq`,
`turn`, `shop`, `price`, `gold_before`, and the `ware` as
`[slot, kind, subkind, quantity, price]`. A decision record carries an
`action`; a purchase record carries the gold before it. The renderer tells
the two apart on that.

An absent value is `-1`, everywhere: no goal ware, no goal item, no price,
no class.

To read it:

```sh
tools/town_dump_report.py --integers OUT/town-dump.i32
```

## What the dump is for

The owner's judgement in this plan enters at exactly one place: the
**usefulness table of phase 1** (#41) - the table of states and their
expected usefulness that phase 1's suite asserts. That table is the door
the owner reads, and it is the one to read first. The dump is not a
second door.

It is what that table is built from:

- **the states are real.** The dump carries the town visits of headless
  runs, each with the state and the stock: the inputs a usefulness
  function needs. Phase 1's table can be seeded from the discriminating
  visits rather than from invented states.
- **the diff is automatic.** Once the usefulness function exists, the
  dump says what the current policy bought at every visit and the new
  function says what it would buy. The disagreements come out on their
  own; the owner reads those, not every visit.
- **it is the debugging view for phases 1 to 4**, which is where a
  usefulness function needs one.

The labelling route - the owner naming the need for every visit - was
dropped on 2026-11-12. The need list (#35), the decay (P4), the
alternatives (P5) and the floors were all decided in the epic's log that
day, and the weights are the free parameters the zoo sweeps in phase 4;
so the labels re-derived what was already settled, at eighteen times the
effort.

Two limits are worth keeping in mind:

- The dump is a **control description, not a judgement**. It shows what
  the current policy decides; it cannot say whether those decisions are
  right, because the escape deficit and the trip model do not exist yet.
- The visits are the **current policy's visits**: the states where the
  current algorithm is not obviously wrong. Harvesting from several
  policies, including the `borg_town_no_stores` baseline, widens the
  sample.

## Where the code is

- `borg-rs/src/borg_town_dump.rs` - the dump and its integer projection.
- `borg-rs/src/borg_think_store.rs` - `borg_choose_shop` records the
  decision, `note` records the reason, and `town_gate_closed` is the null
  arm's gate.
- `borg-rs/src/borg_store_buy.rs` - `borg_think_shop_buy` records the
  purchase.
- `borg-rs/src/borg_io.rs` - `queued_keys`, the test's window on the key
  queue.
- `tools/town_dump_report.py` - the renderer.
