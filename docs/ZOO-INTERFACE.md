# The zoo interface: the defaults, and the contract

## What this file is

The one place that lists every setting whose default is not the original C
Borg's default, says which of them changes what a Borg does, and states the
contract that the zoo and the tools follow.

It is the prose twin of the declaration table in
`borg-rs/src/borg_init.rs` (`UPSTREAM_SETTINGS` and `NEW_SETTINGS`), which the
test module checks against the C parser table and the two `borg.txt` copies.
If the two ever disagree, the tests fail.

## The upstream pin

The original C Borg is Angband's own, at
`e473720ed88a0f0c7ca66815cef3000eb4a38bc3` - `src/borg/` in the
`angband/angband` repository. borg-rs's own C Borg, the oracle under
`angband/src/borg/`, is the consumer's C Borg (better-borg-c milestone 11)
plus this repository's additions; the audit compared the *upstream's* table
and config against both the Rust table and the two `borg.txt` copies here.

The upstream has **35 settings**. Every one of them keeps its upstream
default in borg-rs, in the built-in table and in both `borg.txt` files - the
test `upstream_defaults_are_unchanged` is what keeps that true. **No setting
the upstream had has a different default here**, in either the table or the
configuration.

Nine settings are not in the upstream. They are listed below.

## The settings the upstream did not have

The rule for a setting the upstream never had:

1. **Its built-in default is the original C Borg's behaviour.** That is what
   the zoo's RULES 1 asks of a new parameter, and it is what an unconfigured
   Borg must do.
2. **The product's decision goes in the two `borg.txt` files**, which name the
   setting. A decision that belongs to the product is visible in the
   configuration, not buried in a default.
3. **It is declared.** The declaration table in `borg-rs/src/borg_init.rs`
   (`NEW_SETTINGS`) names the setting, its kind, its built-in default and the
   value the configuration gives it, and `every_setting_is_declared` fails if
   a setting is added without that. Adding a setting is a deliberate act.

All nine keep rule 1: their built-in default does what the original C Borg
did, so an unconfigured Borg is the original C Borg.

| setting | kind | built-in default | the original C Borg's behaviour | the two `borg.txt` files | changes an unconfigured run? | added by |
|---|---|---|---|---|---|---|
| `borg_munchkin_by_class` | bool | **FALSE** | the class rule is off, as it was | **TRUE** | **yes, through the config**: a Borg that reads the delivered `borg.txt` plays Warrior, Druid and Ranger with the stair-scum start; a Borg that reads no `borg.txt` does not | M8 - better-borg-c PR302 (`fd683ced`), carried by owner decision 2026-11-09 |
| `borg_munchkin_class_depth` | int | 6 | inert: the rule is off, so the class depth never applies | 6 | no | M8, with the rule |
| `borg_buy_strategy_default` | int | 0 | 0 is the original buying | 0 | no | borg-rs PR #26/#28 |
| `borg_buy_strategy_munchkin` | int | -1 | -1 is every class, which is the original buying | -1 | no | borg-rs PR #26/#28 |
| `borg_buy_strategy_current_classes` | int | 0 | no class switches | 0 | no | borg-rs PR #26/#28 |
| `borg_buy_strategy_reserve410_classes` | int | 0 | no class switches | 0 | no | borg-rs PR #26/#28 |
| `borg_buy_strategy_current_munchkin_classes` | int | 0 | no class switches | 0 | no | borg-rs PR #26/#28 |
| `borg_buy_strategy_reserve410_munchkin_classes` | int | 0 | no class switches | 0 | no | borg-rs PR #26/#28 |
| `borg_buy_identify` | int | 0 | 0 is off, and the original C Borg never bought to identify | 0 | no | borg-rs PR #28 |

**One departure, and it is deliberate.** `borg_munchkin_by_class` is the only
setting whose built-in default and configured value differ. The built-in
default is FALSE - what the original C Borg did - and the product's milestone 8
decision is carried by the two `borg.txt` files, which say TRUE. So:

- a Borg that reads **no** `borg.txt` plays the **original C Borg's**
  behaviour;
- a Borg that reads the **delivered** `borg.txt` plays the **product's**
  behaviour;
- every tool that runs a Borg **names the setting**, so the difference never
  reaches a measurement by accident.

The owner decided this on 2026-11-12, after the defaults audit.

The C parser registers `borg_munchkin_by_class` and the C Borg reads it, so
the two backends agree on both sides of that line.

## The zoo contract

1. **Every profile names every mode flag.** A profile that names none - the
   zoo's `default` - plays the product's defaults, which since M8 are not the
   original C Borg's behaviour. This is the zoo's own rule (borg-zoo#54) and
   it is why the `normal` and `munchkin-class` profiles exist.
2. **The guard that enforces rule 1 does not yet reach borg-rs.** `zoo plan`
   refuses the `default` profile for a post-cutoff better-borg-c specimen but
   accepts it for a borg-rs one, because `catalogue.mode_side` only knows
   better-borg-c. Routed as borg-zoo#82. Until that is fixed, a borg-rs
   experiment must name its modes by hand.
3. **A specimen is pinned by commit.** A change to a default lands as a new
   commit and a new tag; it never moves an existing specimen's ref, so every
   recorded result stays valid (borg-zoo RULES 4).
4. **A new parameter enters the zoo as a profile** once this repository
   exposes it with a default that reproduces the original behaviour. The
   table above is the list of those that do not, and why.

## The tests that enforce it

In `borg-rs/src/borg_init.rs`:

- `upstream_defaults_are_unchanged` - every one of the 35 upstream settings
  keeps its upstream default;
- `every_setting_is_declared` - a setting is either an upstream setting or a
  declared new one, with its kind and default, so **adding a setting is a
  deliberate act**;
- `the_c_table_matches_the_rust_table` - the C parser table and the Rust
  table carry the same settings, in the same order, with the same kinds and
  defaults;
- `the_borg_txt_copies_agree_and_name_every_setting` - the two `borg.txt`
  copies carry the same settings, and between them they name every setting
  in the table;
- `the_borg_txt_values_follow_the_declaration` - an upstream setting keeps
  its upstream default in the config, and a new setting carries the value
  the declaration names.

## Re-running the audit

```
git -C . fetch --depth 1 https://github.com/angband/angband.git e473720ed88a0f0c7ca66815cef3000eb4a38bc3
git show e473720ed88a0f0c7ca66815cef3000eb4a38bc3:src/borg/borg-init.c   # the original table
git show e473720ed88a0f0c7ca66815cef3000eb4a38bc3:src/borg/borg.txt     # the original config
```

Compare them with `angband/src/borg/borg-init.c`, `borg-rs/src/borg_init.rs`,
`harness/borg.txt` and `angband/src/borg/borg.txt`. The audit was done on
2026-11-12 and the result is the table above.
