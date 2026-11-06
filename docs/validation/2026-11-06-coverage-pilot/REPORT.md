# Rust coverage: pilot_subset

Source `26a96bee4570496dceee5f9f6407a91036c55891`; suites smoke, full; 48 successful oracle-checked runs.
Only the selected completed groups are merged. Interrupted or failed groups are excluded.

| scope | covered lines | covered LLVM functions |
|---|---:|---:|
| All project Rust | 22434/36184 (62.00%) | 1313/1709 (76.83%) |
| Borg logic (excluding FFI/generated/regression modules) | 21251/34610 (61.40%) | 1063/1380 (77.03%) |

LLVM function counts include monomorphizations and closures. Branch coverage is not measured.

| Rust source | covered lines |
|---|---:|
| borg.rs | 112/177 (63.28%) |
| borg_attack_munchkin.rs | 0/144 (0.00%) |
| borg_caution.rs | 928/1242 (74.72%) |
| borg_cave.rs | 20/26 (76.92%) |
| borg_cave_light.rs | 107/107 (100.00%) |
| borg_cave_util.rs | 40/46 (86.96%) |
| borg_cave_view.rs | 226/226 (100.00%) |
| borg_commands.rs | 28/807 (3.47%) |
| borg_danger.rs | 654/1623 (40.30%) |
| borg_escape.rs | 579/752 (76.99%) |
| borg_fight_attack.rs | 1449/2384 (60.78%) |
| borg_fight_defend.rs | 827/2062 (40.11%) |
| borg_fight_perm.rs | 10/393 (2.54%) |
| borg_flow.rs | 635/720 (88.19%) |
| borg_flow_dark.rs | 443/492 (90.04%) |
| borg_flow_glyph.rs | 6/160 (3.75%) |
| borg_flow_kill.rs | 1339/1805 (74.18%) |
| borg_flow_misc.rs | 694/826 (84.02%) |
| borg_flow_stairs.rs | 176/199 (88.44%) |
| borg_flow_take.rs | 308/483 (63.77%) |
| borg_formulas.rs | 83/951 (8.73%) |
| borg_formulas_calc.rs | 3/447 (0.67%) |
| borg_home_notice.rs | 525/648 (81.02%) |
| borg_home_power.rs | 268/292 (91.78%) |
| borg_init.rs | 241/412 (58.50%) |
| borg_inventory.rs | 123/126 (97.62%) |
| borg_io.rs | 149/242 (61.57%) |
| borg_item.rs | 103/126 (81.75%) |
| borg_item_activation.rs | 167/173 (96.53%) |
| borg_item_analyze.rs | 286/363 (78.79%) |
| borg_item_decurse.rs | 48/84 (57.14%) |
| borg_item_enchant.rs | 68/295 (23.05%) |
| borg_item_id.rs | 13/104 (12.50%) |
| borg_item_use.rs | 376/655 (57.40%) |
| borg_item_val.rs | 241/255 (94.51%) |
| borg_item_wear.rs | 518/835 (62.04%) |
| borg_junk.rs | 618/830 (74.46%) |
| borg_light.rs | 280/388 (72.16%) |
| borg_log.rs | 453/832 (54.45%) |
| borg_magic.rs | 304/367 (82.83%) |
| borg_magic_play.rs | 104/124 (83.87%) |
| borg_messages.rs | 649/878 (73.92%) |
| borg_messages_react.rs | 94/164 (57.32%) |
| borg_power.rs | 872/1358 (64.21%) |
| borg_prepared.rs | 205/585 (35.04%) |
| borg_projection.rs | 391/450 (86.89%) |
| borg_recover.rs | 218/315 (69.21%) |
| borg_reincarnate.rs | 0/76 (0.00%) |
| borg_store.rs | 94/114 (82.46%) |
| borg_store_buy.rs | 545/752 (72.47%) |
| borg_store_sell.rs | 567/721 (78.64%) |
| borg_think.rs | 59/104 (56.73%) |
| borg_think_dungeon.rs | 746/1276 (58.46%) |
| borg_think_dungeon_util.rs | 280/720 (38.89%) |
| borg_think_store.rs | 161/192 (83.85%) |
| borg_trait.rs | 1259/1551 (81.17%) |
| borg_trait_swap.rs | 297/460 (64.57%) |
| borg_update.rs | 1195/1604 (74.50%) |
| borg_util.rs | 52/52 (100.00%) |
| cctype.rs | 15/15 (100.00%) |
| engine.rs | 1183/1505 (78.60%) |
| regress.rs | 0/69 (0.00%) |

## Unentered logic functions (first 40 by source location)

The complete list and counts are in `functions.json`. A nonzero entry count alone does not prove a tactic completed.

- [borg.rs:321](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg.rs#L321): `borg_rs[c7a0125852e02980]::borg::check_for_deactivate_or_death::{closure#0}`
- [borg_attack_munchkin.rs:39](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_attack_munchkin.rs#L39): `borg_rs[c7a0125852e02980]::borg_attack_munchkin::borg_timer`
- [borg_attack_munchkin.rs:44](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_attack_munchkin.rs#L44): `borg_rs[c7a0125852e02980]::borg_attack_munchkin::on_stair`
- [borg_attack_munchkin.rs:51](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_attack_munchkin.rs#L51): `borg_rs[c7a0125852e02980]::borg_attack_munchkin::too_dangerous`
- [borg_attack_munchkin.rs:68](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_attack_munchkin.rs#L68): `borg_rs[c7a0125852e02980]::borg_attack_munchkin::borg_munchkin_mage`
- [borg_attack_munchkin.rs:216](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_attack_munchkin.rs#L216): `borg_rs[c7a0125852e02980]::borg_attack_munchkin::borg_munchkin_melee`
- [borg_caution.rs:93](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_caution.rs#L93): `borg_rs[c7a0125852e02980]::borg_caution::set_cursp`
- [borg_caution.rs:1935](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_caution.rs#L1935): `borg_rs[c7a0125852e02980]::borg_caution::note_with_reason`
- [borg_cave.rs:65](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_cave.rs#L65): `<borg_rs[c7a0125852e02980]::borg_cave::CaveState>::row_mut`
- [borg_cave_util.rs:63](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_cave_util.rs#L63): `borg_rs[c7a0125852e02980]::borg_cave_util::borg_feature_protected`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<alloc[12e961f32105355]::vec::Vec<u8>>`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<alloc[12e961f32105355]::string::String>`
- [borg_commands.rs:114](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L114): `borg_rs[c7a0125852e02980]::borg_commands::msg::<&str>`
- [borg_commands.rs:119](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L119): `borg_rs[c7a0125852e02980]::borg_commands::message_flush`
- [borg_commands.rs:124](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L124): `borg_rs[c7a0125852e02980]::borg_commands::get_com_text`
- [borg_commands.rs:130](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L130): `borg_rs[c7a0125852e02980]::borg_commands::c_chars`
- [borg_commands.rs:131](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L131): `borg_rs[c7a0125852e02980]::borg_commands::c_chars::{closure#0}`
- [borg_commands.rs:131](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L131): `borg_rs[c7a0125852e02980]::borg_commands::c_chars::{closure#1}`
- [borg_commands.rs:135](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L135): `borg_rs[c7a0125852e02980]::borg_commands::push_char`
- [borg_commands.rs:140](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L140): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_target`
- [borg_commands.rs:175](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L175): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_help`
- [borg_commands.rs:203](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L203): `borg_rs[c7a0125852e02980]::borg_commands::help_line`
- [borg_commands.rs:240](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L240): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_init_txt_file`
- [borg_commands.rs:246](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L246): `borg_rs[c7a0125852e02980]::borg_commands::avoidance_colour`
- [borg_commands.rs:270](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L270): `borg_rs[c7a0125852e02980]::borg_commands::panel_text`
- [borg_commands.rs:283](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L283): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_avoidances`
- [borg_commands.rs:304](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L304): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_step`
- [borg_commands.rs:334](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L334): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_update`
- [borg_commands.rs:357](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L357): `borg_rs[c7a0125852e02980]::borg_commands::label_attr`
- [borg_commands.rs:366](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L366): `borg_rs[c7a0125852e02980]::borg_commands::bool_text`
- [borg_commands.rs:377](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L377): `borg_rs[c7a0125852e02980]::borg_commands::get_cfg_display`
- [borg_commands.rs:396](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L396): `borg_rs[c7a0125852e02980]::borg_commands::borg_cfg_get_quantity`
- [borg_commands.rs:425](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L425): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_flags`
- [borg_commands.rs:438](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L438): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_flags::{closure#0}`
- [borg_commands.rs:471](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L471): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_cheat`
- [borg_commands.rs:478](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L478): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_nasties`
- [borg_commands.rs:504](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L504): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_search_string`
- [borg_commands.rs:539](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L539): `borg_rs[c7a0125852e02980]::borg_commands::get_feature_display`
- [borg_commands.rs:567](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L567): `borg_rs[c7a0125852e02980]::borg_commands::grid_feature_shown`
- [borg_commands.rs:593](https://github.com/diegoami/borg-rs/blob/26a96bee4570496dceee5f9f6407a91036c55891/borg-rs/src/borg_commands.rs#L593): `borg_rs[c7a0125852e02980]::borg_commands::borg_cmd_grid_feature`
