pub const FLAG_WIDTH: usize = (::core::mem::size_of::<bitflag>() as usize).wrapping_mul(8 as usize);
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_home_notice_h::{borg_notice_home, num_mana};
use crate::headers::borg_home_power_h::borg_power_home;
use crate::headers::borg_inventory_h::borg_item_worth_id;
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{
    borg_item, borg_items, safe_items, C2RustUnnamedH_44, BORG_CURSE_AIR_SWING,
    BORG_CURSE_ANNOYANCE, BORG_CURSE_ANTI_TELEPORTATION, BORG_CURSE_BURNING_UP,
    BORG_CURSE_CHILLED_TO_THE_BONE, BORG_CURSE_CLUMSINESS, BORG_CURSE_COWARDICE,
    BORG_CURSE_DEMON_SUMMON, BORG_CURSE_DRAGON_SUMMON, BORG_CURSE_DULLNESS, BORG_CURSE_ENVELOPING,
    BORG_CURSE_HALLUCINATION, BORG_CURSE_IMPAIR_HITPOINT_RECOVERY, BORG_CURSE_IMPAIR_MANA_RECOVERY,
    BORG_CURSE_IRRITATION, BORG_CURSE_MAX, BORG_CURSE_PARALYSIS, BORG_CURSE_POISON,
    BORG_CURSE_SICKLINESS, BORG_CURSE_SIREN, BORG_CURSE_SLOWNESS, BORG_CURSE_STEELSKIN,
    BORG_CURSE_STONE, BORG_CURSE_TELEPORTATION, BORG_CURSE_TREACHEROUS_WEAPON,
    BORG_CURSE_UNDEAD_SUMMON, BORG_CURSE_UNKNOWN, BORG_CURSE_VULNERABILITY, BORG_CURSE_WEAKNESS,
};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;

use crate::headers::borg_power_h::borg_power;
use crate::headers::borg_store_buy_h::{
    bought_item_num, bought_item_pval, bought_item_store, bought_item_sval, bought_item_tval,
};
pub use crate::headers::borg_store_h::{borg_safe_shops, borg_shop, borg_shops};
use crate::headers::borg_think_h::shop_num;
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_elements_h::{
    ELEM_ACID, ELEM_ARROW, ELEM_CHAOS, ELEM_COLD, ELEM_DARK, ELEM_DISEN, ELEM_ELEC, ELEM_FIRE,
    ELEM_FORCE, ELEM_GRAVITY, ELEM_HOLY_ORB, ELEM_ICE, ELEM_INERTIA, ELEM_LIGHT, ELEM_MANA,
    ELEM_METEOR, ELEM_MISSILE, ELEM_NETHER, ELEM_NEXUS, ELEM_PLASMA, ELEM_POIS, ELEM_SHARD,
    ELEM_SOUND, ELEM_TIME, ELEM_WATER,
};
pub use crate::headers::list_object_flags_h::{
    OF_AFRAID, OF_AGGRAVATE, OF_BLESSED, OF_BURNS_OUT, OF_DIG_1, OF_DIG_2, OF_DIG_3, OF_DRAIN_EXP,
    OF_EXPLODE, OF_FEATHER, OF_FRAGILE, OF_FREE_ACT, OF_HOLD_LIFE, OF_IMPACT, OF_IMPAIR_HP,
    OF_IMPAIR_MANA, OF_LIGHT_2, OF_LIGHT_3, OF_MAX, OF_MULTIPLY_WEIGHT, OF_NO_FUEL, OF_NO_TELEPORT,
    OF_PROT_BLIND, OF_PROT_CONF, OF_PROT_FEAR, OF_PROT_STUN, OF_REGEN, OF_SEE_INVIS,
    OF_SLOW_DIGEST, OF_STICKY, OF_SUST_CON, OF_SUST_DEX, OF_SUST_INT, OF_SUST_STR, OF_SUST_WIS,
    OF_TAKES_FUEL, OF_TELEPATHY, OF_THROWING, OF_TRAP_IMMUNE,
};
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
    C2RustUnnamedH_20, C2RustUnnamedH_31, C2RustUnnamedH_39, OBJ_MOD_MAX, OF_NONE, TV_MAX,
};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, k_info,
    object, object_base, object_kind, poss_item, C2RustUnnamedH_38, C2RustUnnamedH_46, ELEM_MAX,
    EL_INFO_HATES, EL_INFO_IGNORE, EL_INFO_RANDOM,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memcpy, memset, strcmp};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::{bitflag, flag_is_empty, flag_is_equal};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut sold_item_tval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_sval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_pval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_store: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_num: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut sold_item_nxt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut test_item: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
#[no_mangle]
pub static mut best_item: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
unsafe extern "C" fn borg_object_similar(
    mut o_ptr: *mut borg_item,
    mut j_ptr: *mut borg_item,
) -> bool {
    let mut total: ::core::ffi::c_int =
        (*o_ptr).iqty as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if (*o_ptr).kind != (*j_ptr).kind {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !flag_is_equal(
        &raw mut (*o_ptr).flags as *mut bitflag,
        &raw mut (*j_ptr).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add(FLAG_WIDTH)
            .wrapping_sub(1 as size_t)
            .wrapping_div(FLAG_WIDTH),
    ) {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ELEM_MAX as ::core::ffi::c_int {
        if (*o_ptr).el_info[i as usize].res_level as ::core::ffi::c_int
            != (*j_ptr).el_info[i as usize].res_level as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if (*o_ptr).el_info[i as usize].flags as ::core::ffi::c_int
            & (EL_INFO_HATES as ::core::ffi::c_int | EL_INFO_IGNORE as ::core::ffi::c_int)
            != (*j_ptr).el_info[i as usize].flags as ::core::ffi::c_int
                & (EL_INFO_HATES as ::core::ffi::c_int | EL_INFO_IGNORE as ::core::ffi::c_int)
        {
            return false_0 != 0;
        }
        i += 1;
    }
    let mut current_block_60: u64;
    match (*o_ptr).tval as ::core::ffi::c_int {
        1 => return 0 as ::core::ffi::c_int != 0,
        22 | 23 => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 2116367355679836638;
        }
        28 | 26 | 25 | 24 => {
            current_block_60 = 2116367355679836638;
        }
        5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 21 | 20 | 19 => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 15925075030174552612;
        }
        4 | 3 | 2 => {
            current_block_60 = 15925075030174552612;
        }
        _ => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 2116367355679836638;
        }
    }
    match current_block_60 {
        15925075030174552612 => {
            if (*o_ptr).to_h as ::core::ffi::c_int != (*j_ptr).to_h as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).to_d as ::core::ffi::c_int != (*j_ptr).to_d as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).to_a as ::core::ffi::c_int != (*j_ptr).to_a as ::core::ffi::c_int {
                return false_0 != 0;
            }
            i = 0 as ::core::ffi::c_int;
            while i < OBJ_MOD_MAX as ::core::ffi::c_int {
                if (*o_ptr).modifiers[i as usize] as ::core::ffi::c_int
                    != (*j_ptr).modifiers[i as usize] as ::core::ffi::c_int
                {
                    return false_0 != 0;
                }
                i += 1;
            }
            if (*o_ptr).cursed as ::core::ffi::c_int != (*j_ptr).cursed as ::core::ffi::c_int
                || (*o_ptr).uncursable as ::core::ffi::c_int
                    != (*j_ptr).uncursable as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            if (*o_ptr).cursed {
                i = 0 as ::core::ffi::c_int;
                while i < BORG_CURSE_MAX as ::core::ffi::c_int {
                    if (*o_ptr).curses[i as usize] as ::core::ffi::c_int
                        != (*j_ptr).curses[i as usize] as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    i += 1;
                }
            }
            if (*o_ptr).art_idx as ::core::ffi::c_int != (*j_ptr).art_idx as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).ego_idx as ::core::ffi::c_int != (*j_ptr).ego_idx as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if !flag_is_empty(
                &raw mut (*o_ptr).flags as *mut bitflag,
                (OF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(FLAG_WIDTH)
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(FLAG_WIDTH),
            ) || !flag_is_empty(
                &raw mut (*j_ptr).flags as *mut bitflag,
                (OF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(FLAG_WIDTH)
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(FLAG_WIDTH),
            ) {
                return false_0 != 0;
            }
            if (*o_ptr).timeout as ::core::ffi::c_int != 0
                || (*j_ptr).timeout as ::core::ffi::c_int != 0
            {
                return false_0 != 0;
            }
            if (*o_ptr).ac as ::core::ffi::c_int != (*j_ptr).ac as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).dd as ::core::ffi::c_int != (*j_ptr).dd as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).ds as ::core::ffi::c_int != (*j_ptr).ds as ::core::ffi::c_int {
                return false_0 != 0;
            }
        }
        _ => {}
    }
    if (*o_ptr).ident as ::core::ffi::c_int != (*j_ptr).ident as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !(*o_ptr).note.is_null() && (*j_ptr).note.is_null()
        || (*o_ptr).note.is_null() && !(*j_ptr).note.is_null()
    {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !(*o_ptr).note.is_null() && !(*j_ptr).note.is_null() {
        if *(*o_ptr).note.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && *(*j_ptr).note.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcmp((*o_ptr).note, (*j_ptr).note) != 0
        {
            return 0 as ::core::ffi::c_int != 0;
        }
        if strcmp((*o_ptr).note, (*j_ptr).note) != 0 {
            return 0 as ::core::ffi::c_int != 0;
        }
    }
    if total >= (*(*k_info.offset((*o_ptr).kind as isize)).base).max_stack {
        return 0 as ::core::ffi::c_int != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_min_item_quantity(mut item: *mut borg_item) -> ::core::ffi::c_int {
    if crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] < 250 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if (*item).value > 5 as int32_t {
        return 1 as ::core::ffi::c_int;
    }
    if !(*item).aware {
        return 1 as ::core::ffi::c_int;
    }
    match (*item).tval as ::core::ffi::c_int {
        2 | 3 | 4 => {
            if ((*item).iqty as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
                return (*item).iqty as ::core::ffi::c_int;
            }
            return 5 as ::core::ffi::c_int;
        }
        28 => {
            if ((*item).iqty as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                return (*item).iqty as ::core::ffi::c_int;
            }
            return 3 as ::core::ffi::c_int;
        }
        _ => return 1 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn borg_think_home_sell_bad(
    mut i: ::core::ffi::c_int,
    mut borg_empty_home_power: int32_t,
) -> bool {
    let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*item).iqty == 0 || !(*item).aware {
        return true_0 != 0;
    }
    if weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0
        && (*item).art_idx as ::core::ffi::c_int != 0
        && !(*item).ident
    {
        return true_0 != 0;
    }
    if (*item).value == 0 {
        return true_0 != 0;
    }
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < bought_item_num {
        if bought_item_tval[p as usize] == (*item).tval as ::core::ffi::c_int
            && bought_item_sval[p as usize] == (*item).sval as ::core::ffi::c_int
            && bought_item_pval[p as usize] == (*item).pval as ::core::ffi::c_int
            && bought_item_store[p as usize] == BORG_HOME
        {
            return true_0 != 0;
        }
        p += 1;
    }
    borg_notice_home(item, false_0 != 0);
    if borg_power_home() <= borg_empty_home_power {
        return true_0 != 0;
    }
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        charge_each = (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
    }
    (*item).iqty = (*item).iqty.wrapping_sub(1);
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = ((*item).pval as ::core::ffi::c_int - charge_each) as int16_t;
    }
    borg_notice(true_0 != 0);
    if borg_power() < crate::bridge::st().borg.power {
        (*item).iqty = (*item).iqty.wrapping_add(1);
        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
        {
            (*item).pval = ((*item).pval as ::core::ffi::c_int + charge_each) as int16_t;
        }
        return true_0 != 0;
    }
    (*item).iqty = (*item).iqty.wrapping_add(1);
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = ((*item).pval as ::core::ffi::c_int + charge_each) as int16_t;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_home_sell_best(mut best_home_power: *mut int32_t) {
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut item2: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut home_power: int32_t = 0;
    let mut borg_empty_home_power: int32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), true_0 != 0);
    borg_empty_home_power = borg_power_home();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    *best_home_power = borg_power_home();
    let mut first_empty: ::core::ffi::c_int = 0;
    first_empty = 0 as ::core::ffi::c_int;
    while first_empty < (*z_info).store_inven_max as ::core::ffi::c_int {
        if (*(*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(first_empty as isize))
        .iqty as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        first_empty += 1;
    }
    if first_empty < (*z_info).store_inven_max as ::core::ffi::c_int {
        first_empty += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < first_empty {
        item2 = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        let mut current_block_39: u64;
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).pack_size as ::core::ffi::c_int {
            item = borg_items.offset(i as isize) as *mut borg_item;
            if !borg_think_home_sell_bad(i, borg_empty_home_power) {
                if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                    || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                {
                    charge_each =
                        (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
                }
                if borg_object_similar(item2, item) {
                    (*item2).iqty = (*item2).iqty.wrapping_add(1);
                    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                    {
                        (*item2).pval =
                            ((*item2).pval as ::core::ffi::c_int + charge_each) as int16_t;
                    }
                    current_block_39 = 17281240262373992796;
                } else {
                    let mut found_match: bool = false_0 != 0;
                    k = 0 as ::core::ffi::c_int;
                    while k < (*z_info).store_inven_max as ::core::ffi::c_int {
                        if borg_object_similar(
                            (*borg_safe_shops.offset(BORG_HOME as isize))
                                .ware
                                .offset(k as isize) as *mut borg_item,
                            item,
                        ) {
                            found_match = true_0 != 0;
                            break;
                        } else {
                            k += 1;
                        }
                    }
                    if found_match {
                        current_block_39 = 10599921512955367680;
                    } else {
                        memcpy(
                            item2 as *mut ::core::ffi::c_void,
                            item as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        (*item2).iqty = 1 as uint8_t;
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            (*item2).pval = charge_each as int16_t;
                        }
                        current_block_39 = 17281240262373992796;
                    }
                }
                match current_block_39 {
                    10599921512955367680 => {}
                    _ => {
                        (*item).iqty = (*item).iqty.wrapping_sub(1);
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            (*item).pval =
                                ((*item).pval as ::core::ffi::c_int - charge_each) as int16_t;
                        }
                        *test_item.offset(n as isize) =
                            (i + (*z_info).store_inven_max as ::core::ffi::c_int) as uint8_t;
                        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
                        home_power = borg_power_home();
                        if home_power > *best_home_power {
                            k = 0 as ::core::ffi::c_int;
                            while k < (*z_info).store_inven_max as ::core::ffi::c_int {
                                *best_item.offset(k as isize) = *test_item.offset(k as isize);
                                k += 1;
                            }
                            *best_home_power = home_power;
                        }
                        memcpy(
                            item2 as *mut ::core::ffi::c_void,
                            (*borg_safe_shops.offset(BORG_HOME as isize))
                                .ware
                                .offset(n as isize) as *mut borg_item
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        (*item).iqty = (*item).iqty.wrapping_add(1);
                        *test_item.offset(n as isize) = n as uint8_t;
                    }
                }
            }
            i += 1;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_sell_useful(mut best_home_power: *mut int32_t) -> bool {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    memset(
        test_item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as size_t,
    );
    memset(
        best_item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as size_t,
    );
    if (*(*borg_shops.offset(BORG_HOME as isize)).ware.offset(
        ((*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty as ::core::ffi::c_int
        != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        let ref mut fresh0 = *test_item.offset(i as isize);
        *fresh0 = i as uint8_t;
        *best_item.offset(i as isize) = *fresh0;
        i += 1;
    }
    *best_home_power = -(1 as ::core::ffi::c_int) as int32_t;
    borg_think_home_sell_best(best_home_power);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        memcpy(
            (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            (*borg_safe_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        memcpy(
            borg_items.offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        i += 1;
    }
    borg_notice(true_0 != 0);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        if *best_item.offset(i as isize) as ::core::ffi::c_int != i
            && *best_item.offset(i as isize) as ::core::ffi::c_int != 255 as ::core::ffi::c_int
        {
            let mut item: *mut borg_item = borg_items.offset(
                (*best_item.offset(i as isize) as ::core::ffi::c_int
                    - (*z_info).store_inven_max as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
            let mut item2: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item;
            if borg_object_similar(item2, item) {
                if !((*item2).iqty as ::core::ffi::c_int > 90 as ::core::ffi::c_int) {
                    crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
                    crate::bridge::st().borg.goal.item = (*best_item.offset(i as isize) as ::core::ffi::c_int
                        - (*z_info).store_inven_max as ::core::ffi::c_int)
                        as int16_t;
                    return true_0 != 0;
                }
            }
        }
        i += 1;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        == 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).store_inven_max as ::core::ffi::c_int {
            if *best_item.offset(i as isize) as ::core::ffi::c_int != i
                && (*(*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(i as isize))
                .iqty as ::core::ffi::c_int
                    != 0
            {
                let mut item_0: *mut borg_item = borg_items.offset(
                    (*best_item.offset(i as isize) as ::core::ffi::c_int
                        - (*z_info).store_inven_max as ::core::ffi::c_int)
                        as isize,
                ) as *mut borg_item;
                let mut item2_0: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(i as isize)
                    as *mut borg_item;
                if !borg_object_similar(item_0, item2_0) {
                    p = 0 as ::core::ffi::c_int;
                    while p < sold_item_num {
                        if sold_item_tval[p as usize] == (*item2_0).tval as ::core::ffi::c_int
                            && sold_item_sval[p as usize] == (*item2_0).sval as ::core::ffi::c_int
                            && sold_item_store[p as usize] == BORG_HOME
                        {
                            return false_0 != 0;
                        }
                        p += 1;
                    }
                    crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
                    crate::bridge::st().borg.goal.ware = i as int16_t;
                    return true_0 != 0;
                }
            }
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        if *best_item.offset(i as isize) as ::core::ffi::c_int != i
            && *best_item.offset(i as isize) as ::core::ffi::c_int != 255 as ::core::ffi::c_int
        {
            if (*borg_items.offset(
                (*best_item.offset(i as isize) as ::core::ffi::c_int
                    - (*z_info).store_inven_max as ::core::ffi::c_int) as isize,
            ))
            .iqty
                == 0
            {
                return false_0 != 0;
            }
            crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
            crate::bridge::st().borg.goal.item = (*best_item.offset(i as isize) as ::core::ffi::c_int
                - (*z_info).store_inven_max as ::core::ffi::c_int)
                as int16_t;
            return true_0 != 0;
        }
        i += 1;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    return false_0 != 0;
}
unsafe extern "C" fn borg_has_mutiple(mut in_item: *mut borg_item) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if (*in_item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if !(*in_item).ident {
        match (*in_item).tval as ::core::ffi::c_int {
            10 | 11 | 12 | 13 | 14 | 16 | 17 | 2 | 4 | 3 | 5 | 6 | 7 | 8 | 9 => {
                return false_0 != 0;
            }
            _ => {}
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !(item == in_item) {
            if (*item).tval as ::core::ffi::c_int == (*in_item).tval as ::core::ffi::c_int
                && (*item).sval as ::core::ffi::c_int == (*in_item).sval as ::core::ffi::c_int
                && (*item).iqty as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                return true_0 != 0;
            }
        }
        i += 1;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item_0: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item_0).tval as ::core::ffi::c_int == (*in_item).tval as ::core::ffi::c_int
            && (*item_0).sval as ::core::ffi::c_int == (*in_item).sval as ::core::ffi::c_int
            && (*item_0).iqty as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_store_buys(
    mut item: *mut borg_item,
    mut who: ::core::ffi::c_int,
) -> bool {
    match who + 1 as ::core::ffi::c_int {
        1 => {
            match (*item).tval as ::core::ffi::c_int {
                28 | 29 | 27 | 2 | 4 | 3 | 6 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        2 => {
            match (*item).tval as ::core::ffi::c_int {
                10 | 11 | 12 | 13 | 14 | 16 | 17 | 18 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        3 => {
            match (*item).tval as ::core::ffi::c_int {
                2 | 4 | 3 | 5 | 6 | 7 | 8 | 9 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        4 => {
            match (*item).tval as ::core::ffi::c_int {
                31 | 30 | 32 | 33 | 34 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        5 => {
            match (*item).tval as ::core::ffi::c_int {
                25 | 26 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        6 => {
            match (*item).tval as ::core::ffi::c_int {
                20 | 21 | 22 | 23 | 24 | 30 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        7 => {
            if (*player).opts.opt[OPT_birth_no_selling as ::core::ffi::c_int as usize] {
                return true_0 != 0;
            }
            match (*item).tval as ::core::ffi::c_int {
                19 | 15 | 28 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        _ => {}
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_good_sell(mut item: *mut borg_item, mut who: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut multiple: bool = false_0 != 0;
    if (*item).value <= 0 as int32_t {
        if !(((*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
            || (*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int)
            && !(*item).ident)
        {
            return false_0 != 0;
        }
    }
    if !borg_store_buys(item, who) {
        return false_0 != 0;
    }
    if borg_item_note_needs_id(item) {
        multiple = borg_has_mutiple(item);
        if !multiple {
            return false_0 != 0;
        }
    }
    if !((*item).value > 0 as int32_t
        && (crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize] != 0
            || crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int
            || crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                && crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
                    != 0 as ::core::ffi::c_int))
    {
        match (*item).tval as ::core::ffi::c_int {
            26 | 25 => {
                if !(*item).ident {
                    return true_0 != 0;
                }
                if (*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_restore_mana
                    && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                        > 100 as ::core::ffi::c_int
                    && crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_restore_mana) as usize]
                        + num_mana as ::core::ffi::c_int
                        > 99 as ::core::ffi::c_int
                {
                    return false_0 != 0;
                }
            }
            28 | 24 | 23 | 22 | 21 | 20 | 19 => {
                if borg_item_worth_id(item) as ::core::ffi::c_int != 0
                    && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                        < 35 as ::core::ffi::c_int
                    && !multiple
                {
                    return false_0 != 0;
                }
            }
            5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => {
                if borg_item_worth_id(item) as ::core::ffi::c_int != 0 && !multiple {
                    return false_0 != 0;
                }
            }
            _ => {}
        }
    }
    if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0
        && (*item).art_idx as ::core::ffi::c_int != 0
        && !(*item).ident
    {
        return false_0 != 0;
    }
    if !(*item).ident
        && (*item).ego_idx as ::core::ffi::c_int != 0
        && ((*item).iqty as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        if borg_ego_has_random_power(
            e_info.offset(
                (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ))
                .ego_idx as isize,
            ) as *mut ego_item,
        ) {
            return false_0 != 0;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < bought_item_num {
        if bought_item_tval[i as usize] == (*item).tval as ::core::ffi::c_int
            && bought_item_sval[i as usize] == (*item).sval as ::core::ffi::c_int
            && (bought_item_store[i as usize] == who || who != BORG_HOME)
        {
            return false_0 != 0;
        }
        i += 1;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_sell_useless() -> bool {
    let mut icky: ::core::ffi::c_int =
        (*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut c: int32_t = 0 as int32_t;
    let mut b_c: int32_t = 30001 as int32_t;
    let mut fix: bool = false_0 != 0;
    b_p = crate::bridge::st().borg.power;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).store_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        if !((*(*borg_shops.offset(k as isize)).ware.offset(icky as isize)).iqty != 0) {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).pack_size as ::core::ffi::c_int {
                let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
                if !((*item).iqty == 0) {
                    if !((*item).tval as ::core::ffi::c_int
                        == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                        && crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                            < 45 as ::core::ffi::c_int)
                    {
                        if !((*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int
                            && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_healing
                            && crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_rod_healing) as usize] <= 3 as ::core::ffi::c_int)
                        {
                            if !(crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                                == CLASS_WARRIOR
                                && (*item).tval as ::core::ffi::c_int
                                    == TV_ROD as ::core::ffi::c_int
                                && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_mapping
                                && (*item).iqty as ::core::ffi::c_int <= 2 as ::core::ffi::c_int)
                            {
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_WAND as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        < 35 as ::core::ffi::c_int
                                    && ((*item).sval as ::core::ffi::c_int
                                        == crate::bridge::st().item_val.sv_wand_magic_missile
                                        || (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_wand_stinking_cloud
                                        || (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_wand_annihilation)
                                    && (*item).pval as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int)
                                {
                                    if !(weapon_swap != 0
                                        && i == weapon_swap - 1 as ::core::ffi::c_int)
                                    {
                                        if !(armour_swap != 0
                                            && i == armour_swap - 1 as ::core::ffi::c_int)
                                        {
                                            if borg_good_sell(item, k) {
                                                memcpy(
                                                    (*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    safe_items.offset(i as isize) as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                qty = borg_min_item_quantity(item);
                                                (*(*borg_shops.offset(k as isize))
                                                    .ware
                                                    .offset(icky as isize))
                                                .iqty = qty as uint8_t;
                                                let ref mut fresh1 =
                                                    (*borg_items.offset(i as isize)).iqty;
                                                *fresh1 = (*fresh1 as ::core::ffi::c_int - qty)
                                                    as uint8_t;
                                                if (*item).tval as ::core::ffi::c_int
                                                    == TV_STAFF as ::core::ffi::c_int
                                                    || (*item).tval as ::core::ffi::c_int
                                                        == TV_WAND as ::core::ffi::c_int
                                                {
                                                    let mut charge_loss: ::core::ffi::c_int =
                                                        (*safe_items.offset(i as isize)).pval
                                                            as ::core::ffi::c_int
                                                            / (*safe_items.offset(i as isize)).iqty
                                                                as ::core::ffi::c_int;
                                                    (*(*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize))
                                                    .pval = (qty * charge_loss) as int16_t;
                                                    let ref mut fresh2 =
                                                        (*borg_items.offset(i as isize)).pval;
                                                    *fresh2 = (*fresh2 as ::core::ffi::c_int
                                                        - qty * charge_loss)
                                                        as int16_t;
                                                }
                                                fix = true_0 != 0;
                                                borg_notice(true_0 != 0);
                                                p = borg_power();
                                                memcpy(
                                                    borg_items.offset(i as isize) as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    safe_items.offset(i as isize) as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                memcpy(
                                                    (*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    (*borg_safe_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                if !(p < b_p) {
                                                    c = (if ((*item).value as ::core::ffi::c_long)
                                                        < 30000 as ::core::ffi::c_long
                                                    {
                                                        (*item).value as ::core::ffi::c_long
                                                    } else {
                                                        30000 as ::core::ffi::c_long
                                                    })
                                                        as int32_t;
                                                    if !(p == b_p && c >= b_c) {
                                                        b_k = k;
                                                        b_i = i;
                                                        b_p = p;
                                                        b_c = c;
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
        }
        k += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_k >= 0 as ::core::ffi::c_int && b_i >= 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.goal.shop = b_k as int16_t;
        crate::bridge::st().borg.goal.item = b_i as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_sell() -> bool {
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if crate::bridge::st().borg.goal.shop as ::core::ffi::c_int == shop_num as ::core::ffi::c_int
        && crate::bridge::st().borg.goal.item as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = borg_items.offset(crate::bridge::st().borg.goal.item as isize) as *mut borg_item;
        qty = borg_min_item_quantity(item);
        borg_note(format(
            b"# Selling %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('s' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset(crate::bridge::st().borg.goal.item as isize) as keycode_t,
        );
        if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            || qty >= 2 as ::core::ffi::c_int
        {
            if qty == 5 as ::core::ffi::c_int {
                borg_keypress('5' as i32 as keycode_t);
            }
            if qty == 4 as ::core::ffi::c_int {
                borg_keypress('4' as i32 as keycode_t);
            }
            if qty == 3 as ::core::ffi::c_int {
                borg_keypress('3' as i32 as keycode_t);
            }
            if qty == 2 as ::core::ffi::c_int {
                borg_keypress('2' as i32 as keycode_t);
            }
            borg_keypress(KC_ENTER as keycode_t);
        }
        if crate::bridge::st().borg.goal.shop as ::core::ffi::c_int != BORG_HOME {
            borg_keypress(KC_ENTER as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
        }
        if sold_item_nxt >= 9 as ::core::ffi::c_int {
            sold_item_nxt = 0 as ::core::ffi::c_int;
        }
        sold_item_pval[sold_item_nxt as usize] = (*item).pval as ::core::ffi::c_int;
        sold_item_tval[sold_item_nxt as usize] = (*item).tval as ::core::ffi::c_int;
        sold_item_sval[sold_item_nxt as usize] = (*item).sval as ::core::ffi::c_int;
        sold_item_store[sold_item_nxt as usize] = crate::bridge::st().borg.goal.shop as ::core::ffi::c_int;
        sold_item_num = sold_item_nxt;
        sold_item_nxt += 1;
        crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
        crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
        crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
        crate::bridge::st().borg.antibounce_count += 1;
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        crate::bridge::st().borg.in_shop = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_count_sell() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut price: int32_t = 0;
    let mut greed: int32_t = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut sv_qty: ::core::ffi::c_int = 0;
    greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
        / 100 as ::core::ffi::c_long
        + 100 as ::core::ffi::c_long) as int32_t;
    if (greed as ::core::ffi::c_long) < 1000 as ::core::ffi::c_long {
        greed = 1000 as int32_t;
    }
    if greed as ::core::ffi::c_long > 25000 as ::core::ffi::c_long {
        greed = 25000 as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 50 as ::core::ffi::c_int
    {
        greed = 75000 as ::core::ffi::c_int as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 25 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 20 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 35 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 15 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 20 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 13 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 10 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 5 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 5 as ::core::ffi::c_int
    {
        greed = (crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long) as int32_t;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).value <= 0 as int32_t) {
                if !(weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int) {
                    if !(armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int) {
                        if !((*item).tval as ::core::ffi::c_int
                            == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
                        {
                            if !obj_kind_can_browse(
                                k_info.offset((*item).kind as isize) as *mut object_kind
                            ) {
                                if borg_item_note_needs_id(item) {
                                    if !borg_has_mutiple(item) {
                                        return false_0;
                                    }
                                }
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_POTION as ::core::ffi::c_int
                                    && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_serious
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_potion_cure_critical
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_healing
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_potion_star_healing
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_life
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_speed
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_STAFF as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_staff_teleportation
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_wand_drain_life
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == crate::bridge::st().item_val.sv_wand_annihilation
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_SCROLL as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_teleport)
                                {
                                    price = (if ((*item).value as ::core::ffi::c_long)
                                        < 30000 as ::core::ffi::c_long
                                    {
                                        (*item).value as ::core::ffi::c_long
                                    } else {
                                        30000 as ::core::ffi::c_long
                                    }) as int32_t;
                                    if !((price * (*item).iqty as int32_t) < greed
                                        && !borg_item_note_needs_id(item))
                                    {
                                        sv_qty = (*item).iqty as ::core::ffi::c_int;
                                        (*item).iqty = 0 as uint8_t;
                                        borg_notice(true_0 != 0);
                                        p = borg_power() as ::core::ffi::c_int;
                                        (*item).iqty = sv_qty as uint8_t;
                                        if !((p as int32_t + 50 as int32_t) < crate::bridge::st().borg.power) {
                                            k += 1;
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
    borg_notice(true_0 != 0);
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_store_sell() {
    borg_free_store_sell();
    test_item = mem_zalloc(
        ((*z_info).store_inven_max as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    best_item = mem_zalloc(
        ((*z_info).store_inven_max as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_store_sell() {
    if !test_item.is_null() {
        mem_free(test_item as *mut ::core::ffi::c_void);
        test_item = ::core::ptr::null_mut::<uint8_t>();
    }
    if !best_item.is_null() {
        mem_free(best_item as *mut ::core::ffi::c_void);
        best_item = ::core::ptr::null_mut::<uint8_t>();
    }
}
