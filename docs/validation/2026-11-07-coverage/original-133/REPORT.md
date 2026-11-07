# Rust coverage: complete_matrix

Source `f987c077c77360a537c5d6c45fab02b8ad651d91`; suites smoke, full, deep, classes, config, options, captured-rogue, crash-fixes, extended, natural; 133 successful oracle-checked runs.
Only the selected completed groups are merged. Interrupted or failed groups are excluded.

| scope | covered lines | covered LLVM functions |
|---|---:|---:|
| All project Rust | 30565/36184 (84.47%) | 1538/1709 (89.99%) |
| Borg logic (excluding FFI/generated/regression modules) | 29296/34610 (84.65%) | 1264/1380 (91.59%) |

LLVM function counts include monomorphizations and closures. Branch coverage is not measured.

| Rust source | covered lines |
|---|---:|
| borg.rs | 134/177 (75.71%) |
| borg_attack_munchkin.rs | 137/144 (95.14%) |
| borg_caution.rs | 1107/1242 (89.13%) |
| borg_cave.rs | 24/26 (92.31%) |
| borg_cave_light.rs | 107/107 (100.00%) |
| borg_cave_util.rs | 44/46 (95.65%) |
| borg_cave_view.rs | 226/226 (100.00%) |
| borg_commands.rs | 28/807 (3.47%) |
| borg_danger.rs | 1566/1623 (96.49%) |
| borg_escape.rs | 662/752 (88.03%) |
| borg_fight_attack.rs | 2016/2384 (84.56%) |
| borg_fight_defend.rs | 1555/2062 (75.41%) |
| borg_fight_perm.rs | 284/393 (72.26%) |
| borg_flow.rs | 698/720 (96.94%) |
| borg_flow_dark.rs | 460/492 (93.50%) |
| borg_flow_glyph.rs | 6/160 (3.75%) |
| borg_flow_kill.rs | 1709/1805 (94.68%) |
| borg_flow_misc.rs | 767/826 (92.86%) |
| borg_flow_stairs.rs | 184/199 (92.46%) |
| borg_flow_take.rs | 452/483 (93.58%) |
| borg_formulas.rs | 648/951 (68.14%) |
| borg_formulas_calc.rs | 385/447 (86.13%) |
| borg_home_notice.rs | 620/648 (95.68%) |
| borg_home_power.rs | 289/292 (98.97%) |
| borg_init.rs | 289/412 (70.15%) |
| borg_inventory.rs | 124/126 (98.41%) |
| borg_io.rs | 168/242 (69.42%) |
| borg_item.rs | 115/126 (91.27%) |
| borg_item_activation.rs | 167/173 (96.53%) |
| borg_item_analyze.rs | 342/363 (94.21%) |
| borg_item_decurse.rs | 52/84 (61.90%) |
| borg_item_enchant.rs | 237/295 (80.34%) |
| borg_item_id.rs | 13/104 (12.50%) |
| borg_item_use.rs | 559/655 (85.34%) |
| borg_item_val.rs | 241/255 (94.51%) |
| borg_item_wear.rs | 770/835 (92.22%) |
| borg_junk.rs | 742/830 (89.40%) |
| borg_light.rs | 338/388 (87.11%) |
| borg_log.rs | 474/832 (56.97%) |
| borg_magic.rs | 334/367 (91.01%) |
| borg_magic_play.rs | 114/124 (91.94%) |
| borg_messages.rs | 712/878 (81.09%) |
| borg_messages_react.rs | 125/164 (76.22%) |
| borg_power.rs | 1307/1358 (96.24%) |
| borg_prepared.rs | 495/585 (84.62%) |
| borg_projection.rs | 400/450 (88.89%) |
| borg_recover.rs | 280/315 (88.89%) |
| borg_reincarnate.rs | 64/76 (84.21%) |
| borg_store.rs | 94/114 (82.46%) |
| borg_store_buy.rs | 725/752 (96.41%) |
| borg_store_sell.rs | 679/721 (94.17%) |
| borg_think.rs | 88/104 (84.62%) |
| borg_think_dungeon.rs | 994/1276 (77.90%) |
| borg_think_dungeon_util.rs | 511/720 (70.97%) |
| borg_think_store.rs | 184/192 (95.83%) |
| borg_trait.rs | 1462/1551 (94.26%) |
| borg_trait_swap.rs | 444/460 (96.52%) |
| borg_update.rs | 1478/1604 (92.14%) |
| borg_util.rs | 52/52 (100.00%) |
| cctype.rs | 15/15 (100.00%) |
| engine.rs | 1269/1505 (84.32%) |
| regress.rs | 0/69 (0.00%) |

## Unentered logic functions (first 40 by source location)

The complete list and counts are in `functions.json`. A nonzero entry count alone does not prove a tactic completed.

- [borg.rs:321](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg.rs#L321): `borg_rs[c7a0125852e02980]::borg::check_for_deactivate_or_death::{closure#0}`
- [borg_caution.rs:93](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_caution.rs#L93): `borg_rs[c7a0125852e02980]::borg_caution::set_cursp`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<alloc[12e961f32105355]::vec::Vec<u8>>`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<alloc[12e961f32105355]::string::String>`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<&str>`
- [borg_commands.rs:119](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L119): `borg_rs[c7a0125852e02980]::borg_commands::message_flush`
- [borg_commands.rs:124](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L124): `borg_rs[c7a0125852e02980]::borg_commands::get_com_text`
- [borg_commands.rs:130](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L130): `borg_rs[c7a0125852e02980]::borg_commands::c_chars`
- [borg_commands.rs:131](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L131): `borg_rs[c7a0125852e02980]::borg_commands::c_chars::{closure#0}`
- [borg_commands.rs:131](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L131): `borg_rs[c7a0125852e02980]::borg_commands::c_chars::{closure#1}`
- [borg_commands.rs:135](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L135): `borg_rs[c7a0125852e02980]::borg_commands::push_char`
- [borg_commands.rs:140](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L140): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_target`
- [borg_commands.rs:175](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L175): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_help`
- [borg_commands.rs:203](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L203): `borg_rs[c7a0125852e02980]::borg_commands::help_line`
- [borg_commands.rs:240](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L240): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_init_txt_file`
- [borg_commands.rs:246](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L246): `borg_rs[c7a0125852e02980]::borg_commands::avoidance_colour`
- [borg_commands.rs:270](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L270): `borg_rs[c7a0125852e02980]::borg_commands::panel_text`
- [borg_commands.rs:283](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L283): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_avoidances`
- [borg_commands.rs:304](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L304): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_step`
- [borg_commands.rs:334](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L334): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_update`
- [borg_commands.rs:357](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L357): `borg_rs[c7a0125852e02980]::borg_commands::label_attr`
- [borg_commands.rs:366](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L366): `borg_rs[c7a0125852e02980]::borg_commands::bool_text`
- [borg_commands.rs:377](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L377): `borg_rs[c7a0125852e02980]::borg_commands::get_cfg_display`
- [borg_commands.rs:396](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L396): `borg_rs[c7a0125852e02980]::borg_commands::borg_cfg_get_quantity`
- [borg_commands.rs:425](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L425): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_flags`
- [borg_commands.rs:438](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L438): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_flags::{closure#0}`
- [borg_commands.rs:471](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L471): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_cheat`
- [borg_commands.rs:478](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L478): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_nasties`
- [borg_commands.rs:504](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L504): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_search_string`
- [borg_commands.rs:539](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L539): `borg_rs[c7a0125852e02980]::borg_commands::get_feature_display`
- [borg_commands.rs:567](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L567): `borg_rs[c7a0125852e02980]::borg_commands::grid_feature_shown`
- [borg_commands.rs:593](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L593): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_feature`
- [borg_commands.rs:640](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L640): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_target_feature`
- [borg_commands.rs:659](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L659): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_target_info`
- [borg_commands.rs:686](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L686): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_target_danger`
- [borg_commands.rs:703](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L703): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_steps`
- [borg_commands.rs:728](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L728): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_monsters`
- [borg_commands.rs:755](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L755): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_objects`
- [borg_commands.rs:781](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L781): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_targeting`
- [borg_commands.rs:833](https://github.com/diegoami/borg-rs/blob/f987c077c77360a537c5d6c45fab02b8ad651d91/borg-rs/src/borg_commands.rs#L833): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_flow`
