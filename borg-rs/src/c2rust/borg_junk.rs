pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CLASS_RANGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};
use crate::headers::borg_danger_h::borg_danger;
pub use crate::headers::borg_flow_take_h::{borg_get_top_object, borg_take};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_inventory_h::{
    borg_first_empty_inventory_slot, borg_item_worth_id,
};
use crate::headers::borg_io_h::{borg_keypress, borg_keypresses, borg_note};
use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_item, borg_item_weight};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;
use crate::headers::borg_item_use_h::{borg_eat, borg_quaff_potion, borg_read_scroll};

use crate::headers::borg_power_h::borg_power;
pub use crate::headers::borg_trait_h::{
    borg_notice, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
    BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM,
    BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL, BI_AGLYPH,
    BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID, BI_AMAGICMAP,
    BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL, BI_AMMO_COUNT,
    BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE, BI_ARESCOLD,
    BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED, BI_ASTFDEST,
    BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS, BI_BART, BI_BID,
    BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX, BI_CINT, BI_CLASS,
    BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY, BI_CRSAVULN,
    BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING, BI_CRSEVULN, BI_CRSFEAR,
    BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP, BI_CRSNOTEL, BI_CRSPARA,
    BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN, BI_CRSSLOW, BI_CRSSTEELSKIN,
    BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO, BI_CRSVULN, BI_CRSWEAK, BI_CSTR,
    BI_CURHP, BI_CURSP, BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX, BI_DEX_INDEX, BI_DIG, BI_DINV,
    BI_DISM, BI_DISP, BI_EMPTY, BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1, BI_FAIL2, BI_FAST_SHOTS,
    BI_FEATH, BI_FIRST_CURSED, BI_FOOD, BI_FOOD_CURE_BLIND, BI_FOOD_CURE_CONF, BI_FOOD_HI,
    BI_FOOD_LO, BI_FRACT, BI_GOLD, BI_GOOD_S_CHG, BI_GOOD_W_CHG, BI_HASFIXEXP, BI_HEAVYBOW,
    BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ, BI_IACID, BI_ICOLD, BI_IELEC, BI_IFIRE, BI_INFRA, BI_INT,
    BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID, BI_ISBLIND, BI_ISCONFUSED, BI_ISCUT, BI_ISENCUMB,
    BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX, BI_ISFIXEXP, BI_ISFIXINT, BI_ISFIXLEV, BI_ISFIXSTR,
    BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL, BI_ISGORGED, BI_ISHEAVYSTUN, BI_ISHUNGRY, BI_ISIMAGE,
    BI_ISPARALYZED, BI_ISPOISONED, BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK, BI_KING, BI_LIGHT, BI_MAX,
    BI_MAXCLEVEL, BI_MAXDEPTH, BI_MAXHP, BI_MAXSP, BI_MOD_MOVES, BI_MULTIPLE_BONUSES,
    BI_NEED_BRAND_WEAPON, BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D, BI_NEED_ENCHANT_TO_H,
    BI_PREP_BIG_FIGHT, BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD, BI_RCONF, BI_RDARK, BI_RDIS,
    BI_RECALL, BI_REG, BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS, BI_RLITE, BI_RNTHR, BI_RNXUS,
    BI_RPOIS, BI_RSHRD, BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON, BI_SDEX, BI_SDIG, BI_SFRACT,
    BI_SHLIFE, BI_SHOTS, BI_SINT, BI_SINV, BI_SLING, BI_SPEED, BI_SP_ADJ, BI_SRACID, BI_SRBLIND,
    BI_SRCH, BI_SRCOLD, BI_SRCONF, BI_SRDARK, BI_SRDIS, BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS,
    BI_SRLITE, BI_SRNTHR, BI_SRNXUS, BI_SRPOIS, BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR,
    BI_STR_INDEX, BI_SWIS, BI_THB, BI_THN, BI_THT, BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD,
    BI_WB_ELEC, BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS, BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS,
    BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON, BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL,
    BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL, BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD,
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
use crate::headers::borg_trait_swap_h::{armour_swap, weapon_swap};
pub use crate::headers::cave_h::{
    cave, chunk, connector, heatmap, monster_group, square, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_object_modifiers_h::{
    OBJ_MOD_BLOWS, OBJ_MOD_DAM_RED, OBJ_MOD_INFRA, OBJ_MOD_LIGHT, OBJ_MOD_MIGHT, OBJ_MOD_MOVES,
    OBJ_MOD_SEARCH, OBJ_MOD_SHOTS, OBJ_MOD_SPEED, OBJ_MOD_STEALTH, OBJ_MOD_TUNNEL,
};
pub use crate::headers::list_options_h::{
    OPT_animate_flicker, OPT_auto_more, OPT_autoexplore_commands, OPT_birth_ai_learn,
    OPT_birth_connect_stairs, OPT_birth_feelings, OPT_birth_force_descend, OPT_birth_know_flavors,
    OPT_birth_know_runes, OPT_birth_levels_persist, OPT_birth_lose_arts, OPT_birth_no_artifacts,
    OPT_birth_no_recall, OPT_birth_no_selling, OPT_birth_percent_damage, OPT_birth_randarts,
    OPT_birth_stacking, OPT_birth_start_kit, OPT_center_player, OPT_cheat_hear, OPT_cheat_live,
    OPT_cheat_room, OPT_cheat_xtra, OPT_disturb_near, OPT_effective_speed, OPT_highlight_player,
    OPT_hp_changes_color, OPT_hybrid_walls, OPT_mouse_movement, OPT_none, OPT_notify_recharge,
    OPT_pickup_always, OPT_pickup_inven, OPT_purple_uniques, OPT_rogue_like_commands,
    OPT_score_hear, OPT_score_live, OPT_score_room, OPT_score_xtra, OPT_show_damage,
    OPT_show_flavors, OPT_show_target, OPT_solid_walls, OPT_use_old_target, OPT_use_sound,
    OPT_view_yellow_light,
};
pub use crate::headers::list_stats_h::{
    OBJ_MOD_CON, OBJ_MOD_DEX, OBJ_MOD_INT, OBJ_MOD_STR, OBJ_MOD_WIS,
};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::list_tvals_h::{
    TV_AMULET, TV_ARROW, TV_BOLT, TV_BOOTS, TV_BOW, TV_CHEST, TV_CLOAK, TV_CROWN, TV_DIGGING,
    TV_DRAG_ARMOR, TV_FLASK, TV_FOOD, TV_GLOVES, TV_GOLD, TV_HAFTED, TV_HARD_ARMOR, TV_HELM,
    TV_LIGHT, TV_MAGIC_BOOK, TV_MUSHROOM, TV_NATURE_BOOK, TV_NULL, TV_OTHER_BOOK, TV_POLEARM,
    TV_POTION, TV_PRAYER_BOOK, TV_RING, TV_ROD, TV_SCROLL, TV_SHADOW_BOOK, TV_SHIELD, TV_SHOT,
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_properties_h::{
    C2RustUnnamedH_20, C2RustUnnamedH_39, OBJ_MOD_MAX, TV_MAX,
};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, k_info,
    kb_info, object, object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memcpy;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_destroy_floor() -> bool {
    if (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).take == 0 {
        return false_0 != 0;
    }
    if borg_get_top_object(cave, crate::bridge::st().borg.c).is_null() {
        return false_0 != 0;
    }
    let mut take: *mut borg_take = &raw mut crate::bridge::st().takes.takes[((*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)) .take) as usize] as *mut borg_take;
    if (*take).value != -(10 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if (*take).kidx != 0 && !(*k_info.offset((*take).kidx as isize)).base.is_null() {
        borg_note(format(
            b"# Destroying floor item %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*(*k_info.offset((*take).kidx as isize)).base).name,
            (*k_info.offset((*take).kidx as isize)).name,
        ));
    } else {
        borg_note(b"# Destroying floor item.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    borg_keypress('k' as i32 as keycode_t);
    borg_keypress('-' as i32 as keycode_t);
    borg_keypress('a' as i32 as keycode_t);
    borg_keypress('a' as i32 as keycode_t);
    return true_0 != 0;
}
unsafe extern "C" fn borg_consume(mut i: ::core::ffi::c_int) -> bool {
    let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
    if !(*item).aware {
        return false_0 != 0;
    }
    match (*item).tval as ::core::ffi::c_int {
        26 => {
            if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_slime_mold
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_light
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_serious
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_critical
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_healing
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_star_healing
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_life
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_restore_life
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_restore_mana
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_heroism
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_berserk
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_resist_heat
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_resist_cold
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_infravision
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_detect_invis
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_poison
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_speed
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_exp
            {
                if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                    return true_0 != 0;
                }
            } else {
                if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_str2 {
                    if crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
                        >= 118 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_DRUID
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_NECROMANCER
                    {
                        return false_0 != 0;
                    }
                    if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                        return true_0 != 0;
                    }
                }
                if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_int2 {
                    if crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize]
                        >= 118 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_NECROMANCER
                    {
                        return false_0 != 0;
                    }
                    if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                        return true_0 != 0;
                    }
                } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_wis2 {
                    if crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize]
                        >= 118 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_PRIEST
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_DRUID
                    {
                        return false_0 != 0;
                    }
                    if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                        return true_0 != 0;
                    }
                } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_dex2 {
                    if crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
                        >= 118 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_PRIEST
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_DRUID
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_NECROMANCER
                    {
                        return false_0 != 0;
                    }
                    if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                        return true_0 != 0;
                    }
                } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_con2 {
                    if crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize]
                        >= 118 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                        return true_0 != 0;
                    }
                }
            }
        }
        25 => {
            if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_light
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_monster_confusion
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_trap_door_destruction
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_satisfy_hunger
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_dispel_undead
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_blessing
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_holy_chant
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_holy_prayer
            {
                if borg_read_scroll((*item).sval as ::core::ffi::c_int) {
                    return true_0 != 0;
                }
            }
        }
        28 => {
            if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_food_ration
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_food_slime_mold
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_food_waybread
            {
                if crate::bridge::st().borg.trait_[(BI_ISFULL as ::core::ffi::c_int) as usize]
                    == 0
                    && borg_eat(
                        (*item).tval as ::core::ffi::c_int,
                        (*item).sval as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    return true_0 != 0;
                }
            }
        }
        29 => {
            if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_second_sight
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_fast_recovery
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_cure_mind
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_restoring
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_emergency
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_terror
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_stoneskin
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_debility
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_sprinting
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_purging
            {
                if crate::bridge::st().borg.trait_[(BI_ISFULL as ::core::ffi::c_int) as usize]
                    == 0
                    && borg_eat(
                        (*item).tval as ::core::ffi::c_int,
                        (*item).sval as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    return true_0 != 0;
                }
            }
        }
        _ => {}
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_drop_junk() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut fix: bool = false_0 != 0;
    let mut p: int32_t = 0;
    let mut b_p: int32_t = crate::bridge::st().borg.power;
    let mut value: int32_t = 0 as int32_t;
    if !crate::bridge::st().inventory.do_crush_junk {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.goal.recalling != 0 {
        return false_0 != 0;
    }
    if borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    ) > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if !(weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int) {
                if !(armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int) {
                    if !obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
                    {
                        if !((*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                            && !(*item).aware)
                        {
                            if !((*item).tval as ::core::ffi::c_int
                                == TV_BOOTS as ::core::ffi::c_int
                                && !(*item).ident)
                            {
                                value = (*item).value;
                                if ((*item).tval as ::core::ffi::c_int
                                    == TV_STAFF as ::core::ffi::c_int
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int)
                                    && (*item).aware as ::core::ffi::c_int != 0
                                {
                                    if (*item).iqty as ::core::ffi::c_int
                                        > (*item).pval as ::core::ffi::c_int
                                    {
                                        value = 0 as int32_t;
                                    }
                                }
                                if (*item).tval as ::core::ffi::c_int
                                    >= TV_CHEST as ::core::ffi::c_int
                                {
                                    if !(!(*item).ident && value > 0 as int32_t) {
                                        if ((*item).modifiers
                                            [OBJ_MOD_STR as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                            || (*item).modifiers
                                                [OBJ_MOD_INT as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                > 0 as ::core::ffi::c_int
                                            || (*item).modifiers
                                                [OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                > 0 as ::core::ffi::c_int
                                            || (*item).modifiers
                                                [OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                > 0 as ::core::ffi::c_int
                                            || (*item).modifiers
                                                [OBJ_MOD_CON as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                > 0 as ::core::ffi::c_int)
                                            && value > 0 as int32_t
                                        {
                                            value = (value as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if (*item).tval as ::core::ffi::c_int
                                            == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                                            && value > 0 as int32_t
                                        {
                                            value = (value as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if (*item).aware as ::core::ffi::c_int != 0
                                            && ((*item).tval as ::core::ffi::c_int
                                                == TV_POTION as ::core::ffi::c_int
                                                && (*item).sval as ::core::ffi::c_int
                                                    == crate::bridge::st().item_val.sv_potion_restore_mana
                                                && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] >= 1 as ::core::ffi::c_int
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_POTION as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_potion_healing
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_POTION as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_potion_star_healing
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_POTION as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_potion_life
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_POTION as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_potion_speed
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_ROD as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_rod_drain_life
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_ROD as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_rod_healing
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_ROD as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_rod_mapping
                                                    && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_STAFF as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_staff_dispel_evil
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_STAFF as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_staff_power
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_STAFF as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_staff_holiness
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_WAND as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_wand_drain_life
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_WAND as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_wand_annihilation
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_WAND as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_wand_teleport_away
                                                    && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
                                                || (*item).ego_idx as ::core::ffi::c_int != 0
                                                    && borg_ego_has_random_power(
                                                        e_info.offset((*item).ego_idx as isize)
                                                            as *mut ego_item,
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_SCROLL as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_scroll_teleport_level
                                                    && crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize] < 1000 as ::core::ffi::c_int
                                                || (*item).tval as ::core::ffi::c_int
                                                    == TV_SCROLL as ::core::ffi::c_int
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_scroll_protection_from_evil)
                                        {
                                            value = (value as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if (*item).tval as ::core::ffi::c_int
                                            == TV_DIGGING as ::core::ffi::c_int
                                        {
                                            value = 0 as int32_t;
                                        }
                                        if (*item).tval as ::core::ffi::c_int
                                            == TV_SHOT as ::core::ffi::c_int
                                            || (*item).tval as ::core::ffi::c_int
                                                == TV_ARROW as ::core::ffi::c_int
                                            || (*item).tval as ::core::ffi::c_int
                                                == TV_BOLT as ::core::ffi::c_int
                                        {
                                            if (*item).tval as ::core::ffi::c_int
                                                != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                                            {
                                                value = 0 as int32_t;
                                            }
                                        }
                                        if !((*item).value > 0 as int32_t
                                            && (crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize] != 0
                                                || crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] < 10 as ::core::ffi::c_int
                                                || crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] < crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] && crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] != 0 as ::core::ffi::c_int)
                                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] <= 20 as ::core::ffi::c_int
                                            && !(*item).cursed)
                                        {
                                            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                < 5 as ::core::ffi::c_int
                                                && value > 0 as int32_t)
                                            {
                                                if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 10 as ::core::ffi::c_int
                                                    && value > 15 as int32_t)
                                                {
                                                    if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 15 as ::core::ffi::c_int
                                                        && value > 100 as int32_t)
                                                    {
                                                        if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 30 as ::core::ffi::c_int
                                                            && value > 500 as int32_t)
                                                        {
                                                            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 40 as ::core::ffi::c_int
                                                                && value > 1000 as int32_t)
                                                            {
                                                                if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 60 as ::core::ffi::c_int
                                                                    && value > 1200 as int32_t)
                                                                {
                                                                    if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] < 80
                                                                        as ::core::ffi::c_int
                                                                        && value > 1400 as int32_t)
                                                                    {
                                                                        if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                                            < 90 as ::core::ffi::c_int && value > 1600 as int32_t)
                                                                        {
                                                                            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                                                < 95 as ::core::ffi::c_int && value > 4800 as int32_t)
                                                                            {
                                                                                if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                                                    < 127 as ::core::ffi::c_int && value > 5600 as int32_t)
                                                                                {
                                                                                    if !borg_item_note_needs_id(item) {
                                                                                        if !((*player)
                                                                                            .opts
                                                                                            .opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
                                                                                            as ::core::ffi::c_int != 0
                                                                                            && (*item).art_idx as ::core::ffi::c_int != 0
                                                                                            && !(*item).ident)
                                                                                        {
                                                                                            let ref mut fresh0 = (crate::bridge::st().items.items[(i) as usize]).iqty;
                                                                                            *fresh0 = (*fresh0 as ::core::ffi::c_int
                                                                                                - 1 as ::core::ffi::c_int) as uint8_t;
                                                                                            fix = true_0 != 0;
                                                                                            borg_notice(true_0 != 0);
                                                                                            p = borg_power();
                                                                                            memcpy(
                                                                                                &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item
                                                                                                    as *mut ::core::ffi::c_void,
                                                                                                &raw mut crate::bridge::st().items.safe_items[(i) as usize] as *mut borg_item
                                                                                                    as *const ::core::ffi::c_void,
                                                                                                ::core::mem::size_of::<borg_item>() as size_t,
                                                                                            );
                                                                                            if p > b_p {
                                                                                                b_p = p;
                                                                                                b_i = i;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i != -(1 as ::core::ffi::c_int) {
        let mut item_0: *mut borg_item = &raw mut crate::bridge::st().items.items[(b_i) as usize] as *mut borg_item;
        borg_note(format(
            b"# Dropping junk %s (valued at %ld)\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item_0).desc as *mut ::core::ffi::c_char,
            value as ::core::ffi::c_long,
        ));
        borg_keypress('{' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        borg_keypresses(b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress(KC_ENTER as keycode_t);
        borg_keypress('d' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        if (*item_0).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            borg_keypress('1' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
        }
        return true_0 != 0;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    crate::bridge::st().inventory.do_crush_junk = false_0 != 0;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_drop_hole(mut desperate: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut w: int32_t = 0;
    let mut b_w: int32_t = 0 as int32_t;
    let mut value: int32_t = 0;
    let mut value_boost: int32_t = 0;
    let mut fix: bool = false_0 != 0;
    if (crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize] - 1 as ::core::ffi::c_int)) as usize])
    .iqty
        == 0
    {
        return false_0 != 0;
    }
    if (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        != 0
        && (borg_danger(
            crate::bridge::st().borg.c.y,
            crate::bridge::st().borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        ) > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            / 10 as ::core::ffi::c_int
            && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                != crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                || borg_danger(
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    1 as ::core::ffi::c_int,
                    true_0 != 0,
                    false_0 != 0,
                ) > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int))
    {
        return false_0 != 0;
    }
    let mut current_block_78: u64;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        value_boost = 0 as int32_t;
        if !((*item).iqty == 0) {
            if !((*item).art_idx != 0) {
                if !((*item).tval as ::core::ffi::c_int == TV_FOOD as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                        < 5 as ::core::ffi::c_int)
                {
                    if !(weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int) {
                        if !(armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int) {
                            if !obj_kind_can_browse(
                                k_info.offset((*item).kind as isize) as *mut object_kind
                            ) {
                                if (*item).tval as ::core::ffi::c_int
                                    == TV_BOOTS as ::core::ffi::c_int
                                    && !(*item).ident
                                {
                                    if desperate {
                                        value_boost = 10000 as ::core::ffi::c_int as int32_t;
                                        current_block_78 = 5948590327928692120;
                                    } else {
                                        current_block_78 = 11006700562992250127;
                                    }
                                } else {
                                    current_block_78 = 5948590327928692120;
                                }
                                match current_block_78 {
                                    11006700562992250127 => {}
                                    _ => {
                                        if (*item).tval as ::core::ffi::c_int
                                            >= TV_DIGGING as ::core::ffi::c_int
                                            && (*item).tval as ::core::ffi::c_int
                                                <= TV_SWORD as ::core::ffi::c_int
                                            && (crate::bridge::st().items.items[((*z_info).pack_size) as usize])
                                                .tval
                                                as ::core::ffi::c_int
                                                == TV_DIGGING as ::core::ffi::c_int
                                        {
                                            if desperate {
                                                value_boost = 10000 as int32_t;
                                                current_block_78 = 15089075282327824602;
                                            } else {
                                                current_block_78 = 11006700562992250127;
                                            }
                                        } else {
                                            current_block_78 = 15089075282327824602;
                                        }
                                        match current_block_78 {
                                            11006700562992250127 => {}
                                            _ => {
                                                if !((*item).art_idx as ::core::ffi::c_int != 0
                                                    && !(*item).ident)
                                                {
                                                    if borg_item_note_needs_id(item) {
                                                        if desperate {
                                                            value_boost = 5000 as int32_t;
                                                            current_block_78 = 18386322304582297246;
                                                        } else {
                                                            current_block_78 = 11006700562992250127;
                                                        }
                                                    } else {
                                                        current_block_78 = 18386322304582297246;
                                                    }
                                                    match current_block_78 {
                                                        11006700562992250127 => {}
                                                        _ => {
                                                            if !desperate {
                                                                if (*item).aware {
                                                                    if (*item).tval as ::core::ffi::c_int
                                                                        == TV_POTION as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_potion_restore_mana
                                                                        && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                                                                            >= 1 as ::core::ffi::c_int
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_POTION as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_healing
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_POTION as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_potion_star_healing
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_POTION as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_life
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_POTION as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_speed
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_SCROLL as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_scroll_protection_from_evil
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_SCROLL as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_scroll_rune_of_protection
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_SCROLL as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_scroll_teleport_level
                                                                        && crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize]
                                                                            < 1000 as ::core::ffi::c_int
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_ROD as ::core::ffi::c_int
                                                                        && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_healing
                                                                            || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_mapping
                                                                                && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                                                                                    == CLASS_WARRIOR)
                                                                        && (*item).iqty as ::core::ffi::c_int
                                                                            <= 5 as ::core::ffi::c_int
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_WAND as ::core::ffi::c_int
                                                                        && (*item).sval as ::core::ffi::c_int
                                                                            == crate::bridge::st().item_val.sv_wand_teleport_away
                                                                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                                                                            == CLASS_WARRIOR
                                                                        && crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize]
                                                                            <= 8 as ::core::ffi::c_int
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else if (*item).tval as ::core::ffi::c_int
                                                                        == TV_ROD as ::core::ffi::c_int
                                                                        && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_light
                                                                            && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                                                                                <= 0 as ::core::ffi::c_int)
                                                                    {
                                                                        current_block_78 = 11006700562992250127;
                                                                    } else {
                                                                        current_block_78 = 7226443171521532240;
                                                                    }
                                                                } else {
                                                                    current_block_78 =
                                                                        7226443171521532240;
                                                                }
                                                            } else {
                                                                value_boost = 7000
                                                                    as ::core::ffi::c_int
                                                                    as int32_t;
                                                                current_block_78 =
                                                                    7226443171521532240;
                                                            }
                                                            match current_block_78 {
                                                                11006700562992250127 => {}
                                                                _ => {
                                                                    if (*item).ego_idx
                                                                        as ::core::ffi::c_int
                                                                        != 0
                                                                        && borg_ego_has_random_power(
                                                                            e_info.offset(
                                                                                (*item).ego_idx
                                                                                    as isize,
                                                                            )
                                                                                as *mut ego_item,
                                                                        )
                                                                            as ::core::ffi::c_int
                                                                            != 0
                                                                        && !(*item).ident
                                                                    {
                                                                        if desperate {
                                                                            value_boost =
                                                                                10000 as int32_t;
                                                                            current_block_78 =
                                                                                790185930182612747;
                                                                        } else {
                                                                            current_block_78 = 11006700562992250127;
                                                                        }
                                                                    } else {
                                                                        current_block_78 =
                                                                            790185930182612747;
                                                                    }
                                                                    match current_block_78 {
                                                                        11006700562992250127 => {}
                                                                        _ => {
                                                                            value = (*item).value
                                                                                + value_boost;
                                                                            w = borg_item_weight(
                                                                                item,
                                                                            )
                                                                                as int32_t;
                                                                            (crate::bridge::st().items.items[(i) as usize])
                                                                            .iqty = 0 as uint8_t;
                                                                            fix = true_0 != 0;
                                                                            borg_notice(
                                                                                true_0 != 0,
                                                                            );
                                                                            p = borg_power();
                                                                            memcpy(
                                                                                &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item
                                                                                    as *mut ::core::ffi::c_void,
                                                                                &raw mut crate::bridge::st().items.safe_items[(i) as usize] as *mut borg_item
                                                                                    as *const ::core::ffi::c_void,
                                                                                ::core::mem::size_of::<borg_item>() as size_t,
                                                                            );
                                                                            if (*item)
                                                                                .modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                                                                                as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                                                                || (*item)
                                                                                    .modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                                                                                    as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                                                                || (*item)
                                                                                    .modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                                                                                    as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                                                                || (*item)
                                                                                    .modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                                                                                    as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                                                                || (*item)
                                                                                    .modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                                                                                    as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                                                            {
                                                                                value = (value as ::core::ffi::c_long
                                                                                    + 20000 as ::core::ffi::c_long) as int32_t;
                                                                            }
                                                                            if (*item).tval as ::core::ffi::c_int
                                                                                == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                                                                                && value > 0 as int32_t
                                                                                && crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                                                                                    <= 35 as ::core::ffi::c_int
                                                                                && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                                                    >= 1 as ::core::ffi::c_int
                                                                            {
                                                                                value = (value as ::core::ffi::c_long
                                                                                    + 5000 as ::core::ffi::c_long) as int32_t;
                                                                            }
                                                                            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                                                > 35 as ::core::ffi::c_int
                                                                                && (*item).aware as ::core::ffi::c_int != 0
                                                                            {
                                                                                match (*item).tval as ::core::ffi::c_int {
                                                                                    21 => {
                                                                                        value = (*item).iqty as int32_t * value * 10 as int32_t;
                                                                                    }
                                                                                    20 | 5 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 16 | 17
                                                                                    | 18 => {
                                                                                        value = (*item).iqty as int32_t * value * 5 as int32_t;
                                                                                    }
                                                                                    15 => {
                                                                                        if (*item).ego_idx as ::core::ffi::c_int != 0
                                                                                            && borg_ego_has_random_power(
                                                                                                e_info.offset((*item).ego_idx as isize) as *mut ego_item,
                                                                                            ) as ::core::ffi::c_int != 0
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 300000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * value;
                                                                                        }
                                                                                    }
                                                                                    24 => {
                                                                                        if (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_rod_drain_life
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_rod_acid_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_rod_elec_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_rod_fire_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_rod_cold_ball
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 300000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * value;
                                                                                        }
                                                                                    }
                                                                                    22 => {
                                                                                        if (*item).sval as ::core::ffi::c_int
                                                                                            != crate::bridge::st().item_val.sv_staff_dispel_evil
                                                                                            || ((*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_staff_power
                                                                                                || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_staff_holiness)
                                                                                                && crate::bridge::st().borg.trait_[(BI_GOOD_S_CHG as ::core::ffi::c_int) as usize]
                                                                                                    < 2 as ::core::ffi::c_int
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_staff_destruction
                                                                                                && crate::bridge::st().borg.trait_[(BI_ASTFDEST as ::core::ffi::c_int) as usize]
                                                                                                    < 2 as ::core::ffi::c_int
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 300000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * (value / 2 as int32_t);
                                                                                        }
                                                                                    }
                                                                                    23 => {
                                                                                        if (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_drain_life
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_wand_teleport_away
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_acid_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_elec_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_fire_ball
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_cold_ball
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_wand_annihilation
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_dragon_fire
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_wand_dragon_cold
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 300000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * (value / 2 as int32_t);
                                                                                        }
                                                                                    }
                                                                                    25 => {
                                                                                        if (*item).sval as ::core::ffi::c_int
                                                                                            != crate::bridge::st().item_val.sv_scroll_protection_from_evil
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_scroll_rune_of_protection
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 30000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * (value / 10 as int32_t);
                                                                                        }
                                                                                    }
                                                                                    26 => {
                                                                                        if (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_potion_healing
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_potion_star_healing
                                                                                            || (*item).sval as ::core::ffi::c_int != crate::bridge::st().item_val.sv_potion_life
                                                                                            || (*item).sval as ::core::ffi::c_int
                                                                                                != crate::bridge::st().item_val.sv_potion_restore_mana
                                                                                        {
                                                                                            value = ((*item).iqty as ::core::ffi::c_long
                                                                                                * 300000 as ::core::ffi::c_long) as int32_t;
                                                                                        } else {
                                                                                            value = (*item).iqty as int32_t * (value / 10 as int32_t);
                                                                                        }
                                                                                    }
                                                                                    _ => {
                                                                                        value = (*item).iqty as int32_t * (value / 3 as int32_t);
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                value = (*item).iqty as int32_t * value;
                                                                            }
                                                                            if !(*item).aware
                                                                                && value
                                                                                    > 0 as int32_t
                                                                            {
                                                                                match (*item).tval as ::core::ffi::c_int {
                                                                                    21 | 20 => {
                                                                                        value = (crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                                                            as ::core::ffi::c_long * 5000 as ::core::ffi::c_long)
                                                                                            as int32_t;
                                                                                    }
                                                                                    24 => {
                                                                                        value = (crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                                                            as ::core::ffi::c_long * 3000 as ::core::ffi::c_long)
                                                                                            as int32_t;
                                                                                    }
                                                                                    22 | 23 => {
                                                                                        value = (crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                                                            as ::core::ffi::c_long * 2000 as ::core::ffi::c_long)
                                                                                            as int32_t;
                                                                                    }
                                                                                    25 | 26 => {
                                                                                        value = (crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                                                            as ::core::ffi::c_long * 500 as ::core::ffi::c_long)
                                                                                            as int32_t;
                                                                                    }
                                                                                    28 => {
                                                                                        value = (crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                                                            as ::core::ffi::c_long * 10 as ::core::ffi::c_long)
                                                                                            as int32_t;
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                            if !(*item).ident && value > 0 as int32_t
                                                                                && borg_item_worth_id(item) as ::core::ffi::c_int != 0
                                                                            {
                                                                                match (*item).tval as ::core::ffi::c_int {
                                                                                    2 | 3 | 4 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 100 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    5 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 20000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    6 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 10 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    7 | 8 | 9 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 10000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    10 | 11 | 12 | 13 | 14 | 15 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 15000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    16 | 17 | 18 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 15000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    20 | 21 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 5000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    22 | 23 => {
                                                                                        value = (value as ::core::ffi::c_long
                                                                                            + 1000 as ::core::ffi::c_long) as int32_t;
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                            value = value
                                                                                / 100 as int32_t;
                                                                            if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                                                                == 0 as ::core::ffi::c_int
                                                                                && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                                                                                    == 0 as ::core::ffi::c_int
                                                                            {
                                                                                value = value / 500 as int32_t;
                                                                            }
                                                                            p -= value;
                                                                            if !(b_i >= 0 as ::core::ffi::c_int && p < b_p) {
                                                                                if !(b_p == p && w < b_w) {
                                                                                    b_i = i;
                                                                                    b_p = p;
                                                                                    b_w = w;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_i >= 0 as ::core::ffi::c_int {
        let mut item_0: *mut borg_item = &raw mut crate::bridge::st().items.items[(b_i) as usize] as *mut borg_item;
        borg_note(format(
            b"# Junking %ld gold (full)\0" as *const u8 as *const ::core::ffi::c_char,
            crate::bridge::st().borg.power as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                - b_p as ::core::ffi::c_long,
        ));
        if borg_consume(b_i) {
            return true_0 != 0;
        }
        borg_note(format(
            b"# Dropping %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item_0).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('{' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        borg_keypresses(b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress(KC_ENTER as keycode_t);
        borg_keypress('d' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        if (*item_0).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
        }
        return true_0 != 0;
    }
    if !desperate {
        return borg_drop_hole(true_0 != 0);
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_drop_slow() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut temp: int32_t = 0;
    let mut greed: int32_t = 0;
    let mut fix: bool = false_0 != 0;
    if borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    ) > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] / 20 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 110 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.munchkin_mode {
        return false_0 != 0;
    }
    greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
        / 100 as ::core::ffi::c_long
        + 100 as ::core::ffi::c_long) as int32_t;
    if (greed as ::core::ffi::c_long) < 500 as ::core::ffi::c_long
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            > 35 as ::core::ffi::c_int
    {
        greed = 500 as int32_t;
    }
    if greed as ::core::ffi::c_long > 25000 as ::core::ffi::c_long {
        greed = 25000 as int32_t;
    }
    greed = (greed as ::core::ffi::c_int
        - (110 as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize])
            * 500 as ::core::ffi::c_int) as int32_t;
    if greed <= 0 as int32_t {
        greed = 0 as int32_t;
    }
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if !(i >= (*z_info).pack_size as ::core::ffi::c_int
                && i <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
            {
                if !(weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int) {
                    if !(armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int) {
                        if !(!(*item).ident && borg_item_worth_id(item) as ::core::ffi::c_int != 0)
                        {
                            if !((*item).tval as ::core::ffi::c_int
                                == TV_BOOTS as ::core::ffi::c_int
                                && !(*item).ident)
                            {
                                if !((*player).opts.opt
                                    [OPT_birth_randarts as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int
                                    != 0
                                    && (*item).art_idx as ::core::ffi::c_int != 0
                                    && !(*item).ident)
                                {
                                    if !borg_item_note_needs_id(item) {
                                        if !((*item).tval as ::core::ffi::c_int
                                            >= TV_DIGGING as ::core::ffi::c_int
                                            && (*item).tval as ::core::ffi::c_int
                                                <= TV_SWORD as ::core::ffi::c_int
                                            && (crate::bridge::st().items.items[((*z_info).pack_size) as usize])
                                                .tval
                                                as ::core::ffi::c_int
                                                == TV_DIGGING as ::core::ffi::c_int)
                                        {
                                            if !((*item).tval as ::core::ffi::c_int
                                                == TV_ROD as ::core::ffi::c_int
                                                && (*item).aware as ::core::ffi::c_int != 0
                                                && ((*item).sval as ::core::ffi::c_int
                                                    == crate::bridge::st().item_val.sv_rod_light
                                                    && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] <= 0 as ::core::ffi::c_int))
                                            {
                                                if !((*item).tval as ::core::ffi::c_int
                                                    == TV_ROD as ::core::ffi::c_int
                                                    && (*item).aware as ::core::ffi::c_int != 0
                                                    && (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_rod_healing)
                                                {
                                                    let ref mut fresh1 =
                                                        (crate::bridge::st().items.items[(i) as usize]).iqty;
                                                    *fresh1 = (*fresh1).wrapping_sub(1);
                                                    fix = true_0 != 0;
                                                    borg_notice(true_0 != 0);
                                                    p = borg_power();
                                                    let ref mut fresh2 =
                                                        (crate::bridge::st().items.items[(i) as usize]).iqty;
                                                    *fresh2 = (*fresh2).wrapping_add(1);
                                                    temp = (if ((*item).value
                                                        as ::core::ffi::c_long)
                                                        < 30000 as ::core::ffi::c_long
                                                    {
                                                        (*item).value as ::core::ffi::c_long
                                                    } else {
                                                        30000 as ::core::ffi::c_long
                                                    })
                                                        as int32_t;
                                                    if temp < greed {
                                                        temp = 0 as int32_t;
                                                    }
                                                    p -= temp;
                                                    temp = (*item).weight as int32_t;
                                                    p = (p as ::core::ffi::c_int
                                                        + (temp * 100 as int32_t)
                                                            as ::core::ffi::c_int)
                                                        as int32_t;
                                                    if !(p < b_p) {
                                                        b_i = i;
                                                        b_p = p;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_i >= 0 as ::core::ffi::c_int && b_p >= crate::bridge::st().borg.power {
        let mut item_0: *mut borg_item = &raw mut crate::bridge::st().items.items[(b_i) as usize] as *mut borg_item;
        borg_note(format(
            b"# Junking %ld power (slow) value %ld\0" as *const u8 as *const ::core::ffi::c_char,
            b_p as ::core::ffi::c_long - crate::bridge::st().borg.power as ::core::ffi::c_long,
            (*item_0).value as ::core::ffi::c_long,
        ));
        if borg_consume(b_i) {
            return true_0 != 0;
        }
        borg_note(format(
            b"# Dropping %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item_0).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('{' as i32 as keycode_t);
        if b_i < (*z_info).pack_size as ::core::ffi::c_int {
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                    as keycode_t,
            );
        } else if b_i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
            borg_keypress('/' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((b_i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
        } else {
            borg_keypress('|' as i32 as keycode_t);
            borg_keypress(
                ('0' as i32
                    + (b_i
                        - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)))
                    as keycode_t,
            );
        }
        borg_keypresses(b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress(KC_ENTER as keycode_t);
        borg_keypress('d' as i32 as keycode_t);
        if b_i < (*z_info).pack_size as ::core::ffi::c_int {
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                    as keycode_t,
            );
        } else if b_i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
            borg_keypress('/' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((b_i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
        } else {
            borg_keypress('|' as i32 as keycode_t);
            borg_keypress(
                ('0' as i32
                    + (b_i
                        - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)))
                    as keycode_t,
            );
        }
        if (*item_0).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            borg_keypress('1' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_dump_quiver() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut quiver_capacity: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 150 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_RANGER
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
    {
        quiver_capacity = ((*kb_info.offset(TV_ARROW as ::core::ffi::c_int as isize)).max_stack
            - 1 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int;
    } else {
        quiver_capacity = (*kb_info.offset(TV_ARROW as ::core::ffi::c_int as isize)).max_stack
            - 1 as ::core::ffi::c_int;
    }
    quiver_capacity *= (*z_info).quiver_size as ::core::ffi::c_int;
    let mut current_block_14: u64;
    i = (*z_info).pack_size as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int
        + (*z_info).quiver_size as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int;
    while i >= (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if !(*item).cursed
                && (*item).tval as ::core::ffi::c_int
                    == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
            {
                if (*item).to_d as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (*item).to_h as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    current_block_14 = 13109137661213826276;
                } else if borg_item_note_needs_id(item) {
                    current_block_14 = 13109137661213826276;
                } else if crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                    <= quiver_capacity
                    && (*item).to_d as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                    && (*item).to_h as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                {
                    current_block_14 = 13109137661213826276;
                } else {
                    current_block_14 = 15652330335145281839;
                }
            } else {
                current_block_14 = 15652330335145281839;
            }
            match current_block_14 {
                13109137661213826276 => {}
                _ => {
                    b_i = i;
                }
            }
        }
        i -= 1;
    }
    if b_i >= 0 as ::core::ffi::c_int {
        item = &raw mut crate::bridge::st().items.items[(b_i) as usize] as *mut borg_item;
        borg_note(format(
            b"# Dumping %s.  Bad ammo in quiver.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('{' as i32 as keycode_t);
        borg_keypress('|' as i32 as keycode_t);
        borg_keypress(
            (b_i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                + '0' as i32) as keycode_t,
        );
        borg_keypresses(b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress(KC_ENTER as keycode_t);
        borg_keypress('d' as i32 as keycode_t);
        borg_keypress('|' as i32 as keycode_t);
        borg_keypress(
            (b_i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                + '0' as i32) as keycode_t,
        );
        if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
        }
        crate::bridge::st().borg.antibounce_count += 1;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_remove_stuff() -> bool {
    let mut hole: ::core::ffi::c_int = borg_first_empty_inventory_slot();
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut w_p: int32_t = 0 as int32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut fix: bool = false_0 != 0;
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 150 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    b_p = crate::bridge::st().borg.power;
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !(borg_item_note_needs_id(item) as ::core::ffi::c_int != 0 && !(*item).cursed) {
                    if !(*item).one_ring {
                        memcpy(
                            &raw mut crate::bridge::st().items.items[(hole) as usize] as *mut borg_item
                                as *mut ::core::ffi::c_void,
                            item as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        memcpy(
                            item as *mut ::core::ffi::c_void,
                            &raw mut crate::bridge::st().items.safe_items[(hole) as usize] as *mut borg_item
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        fix = true_0 != 0;
                        borg_notice(true_0 != 0);
                        p = borg_power();
                        memcpy(
                            item as *mut ::core::ffi::c_void,
                            &raw mut crate::bridge::st().items.safe_items[(i) as usize] as *mut borg_item
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        (crate::bridge::st().items.items[(hole) as usize]).iqty = 0 as uint8_t;
                        if p >= b_p {
                            b_i = i;
                            w_p = p;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_i >= 0 as ::core::ffi::c_int {
        item = &raw mut crate::bridge::st().items.items[(b_i) as usize] as *mut borg_item;
        borg_note(format(
            b"# Removing %s.  Power with: (%ld) Power w/o (%ld)\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            b_p as ::core::ffi::c_long,
            w_p as ::core::ffi::c_long,
        ));
        borg_keypress('t' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((b_i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                as keycode_t,
        );
        crate::bridge::st().borg.antibounce_count += 1;
        return true_0 != 0;
    }
    return false_0 != 0;
}
