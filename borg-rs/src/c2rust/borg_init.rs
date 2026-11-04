pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PR_MAP: ::core::ffi::c_long = 0x10000 as ::core::ffi::c_long;
pub const MAX_RACES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MAX_CLASSES: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_cave_h::{borg_free_cave, borg_init_cave};
use crate::headers::borg_flow_h::{borg_free_flow, borg_init_flow};
use crate::headers::borg_flow_kill_h::{borg_free_flow_kill, borg_init_flow_kill};
use crate::headers::borg_flow_take_h::{borg_free_flow_take, borg_init_flow_take};
use crate::headers::borg_formulas_h::{borg_free_formulas, borg_load_formulas};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_cfg, borg_rand_local, borg_struct, borg_time, C2RustUnnamedH_10,
    C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15,
    C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE,
    BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL,
    BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS,
    BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START,
    BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS,
    BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM,
    BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS,
    BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD,
    BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED,
};
pub use crate::headers::borg_init_h::borg_setting;
use crate::headers::borg_io_h::{borg_free_io, borg_init_io, borg_note, borg_warning};
use crate::headers::borg_item_activation_h::borg_init_item_activation;
use crate::headers::borg_item_h::{borg_free_item, borg_init_item};
use crate::headers::borg_item_val_h::borg_init_item_val;
use crate::headers::borg_item_wear_h::{borg_free_item_wear, borg_init_item_wear};
use crate::headers::borg_magic_h::borg_prepare_book_info;
use crate::headers::borg_messages_h::{borg_free_messages, borg_init_messages};
use crate::headers::borg_store_h::{borg_free_store, borg_init_store};
pub use crate::headers::borg_trait_h::{borg_trait_free, borg_trait_init};
use crate::headers::borg_update_h::{borg_free_detection, borg_free_update, borg_init_update};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};
pub use crate::headers::game_event_h::{
    event_add_handler, event_remove_handler, event_signal, game_event_data, game_event_handler,
    game_event_type, C2RustUnnamedH_0, C2RustUnnamedH_1, C2RustUnnamedH_2, C2RustUnnamedH_3,
    C2RustUnnamedH_4, C2RustUnnamedH_5, C2RustUnnamedH_6, C2RustUnnamedH_7, EVENT_AC,
    EVENT_ANIMATE, EVENT_BELL, EVENT_BIRTHPOINTS, EVENT_BOLT, EVENT_CHEAT_DEATH,
    EVENT_CHECK_INTERRUPT, EVENT_COMMAND_REPEAT, EVENT_DETECTIONSTATUS, EVENT_DUNGEONLEVEL,
    EVENT_END, EVENT_ENTER_BIRTH, EVENT_ENTER_DEATH, EVENT_ENTER_GAME, EVENT_ENTER_INIT,
    EVENT_ENTER_STORE, EVENT_ENTER_WORLD, EVENT_EQUIPMENT, EVENT_EXPERIENCE, EVENT_EXPLOSION,
    EVENT_FEELING, EVENT_GEN_LEVEL_END, EVENT_GEN_LEVEL_START, EVENT_GEN_ROOM_CHOOSE_SIZE,
    EVENT_GEN_ROOM_CHOOSE_SUBTYPE, EVENT_GEN_ROOM_END, EVENT_GEN_ROOM_START,
    EVENT_GEN_TUNNEL_FINISHED, EVENT_GOLD, EVENT_HP, EVENT_INITSTATUS, EVENT_INPUT_FLUSH,
    EVENT_INVENTORY, EVENT_ITEMLIST, EVENT_LEAVE_BIRTH, EVENT_LEAVE_DEATH, EVENT_LEAVE_GAME,
    EVENT_LEAVE_INIT, EVENT_LEAVE_STORE, EVENT_LEAVE_WORLD, EVENT_LIGHT, EVENT_MANA, EVENT_MAP,
    EVENT_MESSAGE, EVENT_MESSAGE_FLUSH, EVENT_MISSILE, EVENT_MONSTERHEALTH, EVENT_MONSTERLIST,
    EVENT_MONSTERTARGET, EVENT_NEW_LEVEL_DISPLAY, EVENT_OBJECTTARGET, EVENT_PLAYERLEVEL,
    EVENT_PLAYERMOVED, EVENT_PLAYERSPEED, EVENT_PLAYERTITLE, EVENT_RACE_CLASS, EVENT_REFRESH,
    EVENT_SEEFLOOR, EVENT_SOUND, EVENT_STATE, EVENT_STATS, EVENT_STATUS, EVENT_STORECHANGED,
    EVENT_STUDYSTATUS, EVENT_USE_STORE,
};
pub use crate::headers::guid_h::guid;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{
    angband_constants, critical_level, o_critical_level, z_info, ANGBAND_DIR_USER,
};
pub use crate::headers::list_ignore_types_h::{
    ITYPE_AMULET, ITYPE_ARROW, ITYPE_BALANCE_DRAGON_ARMOR, ITYPE_BASIC_DRAGON_ARMOR, ITYPE_BLUNT,
    ITYPE_BODY_ARMOR, ITYPE_BOLT, ITYPE_BOW, ITYPE_CLOAK, ITYPE_CROSSBOW, ITYPE_DIGGER,
    ITYPE_ELVEN_CLOAK, ITYPE_FEET, ITYPE_GREAT, ITYPE_HANDGEAR, ITYPE_HEADGEAR,
    ITYPE_HIGH_DRAGON_ARMOR, ITYPE_LIGHT, ITYPE_MULTI_DRAGON_ARMOR, ITYPE_NONE,
    ITYPE_POWER_DRAGON_ARMOR, ITYPE_RING, ITYPE_ROBE, ITYPE_SHARP, ITYPE_SHIELD, ITYPE_SHOT,
    ITYPE_SLING,
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
use crate::headers::message_h::msg;
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_ignore_h::{
    ego_ignore_types, ignore_birth_init, ignore_level, C2RustUnnamedH_40, ITYPE_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::{
    option_set, player_options, screensaver, C2RustUnnamedH_21, OPT_MAX,
};

pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_id2class,
    player_race, player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::stdlib_h::atoi;
use crate::headers::string_h::{strcmp, strlen};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
use crate::headers::ui_command_h::do_cmd_redraw;
pub use crate::headers::ui_keymap_h::{
    C2RustUnnamedH_41, KEYMAP_MODE_MAX, KEYMAP_MODE_ORIG, KEYMAP_MODE_ROGUE,
};
use crate::headers::ui_term_h::{tile_height, tile_width, Term_fresh};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_file_h::{
    ang_file, file_close, file_getl, file_mode, file_open, file_putf, file_type, path_build,
    FTYPE_HTML, FTYPE_RAW, FTYPE_SAVE, FTYPE_TEXT, MODE_APPEND, MODE_READ, MODE_WRITE,
};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, prefix_i};
use crate::headers::z_virt_h::{mem_alloc, mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_initialized: bool = false;
#[no_mangle]
pub static mut game_closed: bool = false;
#[no_mangle]
pub static mut borg_init_failure: bool = false_0 != 0;
#[no_mangle]
pub static mut borg_settings: [borg_setting; 36] = [
    borg_setting {
        setting_string: b"borg_verbose\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_munchkin_start\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_munchkin_level\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 12 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_munchkin_depth\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 16 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_worships_damage\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_speed\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_hp\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_mana\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_ac\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_gold\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_plays_risky\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_kills_uniques\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_uses_swaps\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_uses_dynamic_calcs\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_stop_dlevel\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 128 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_clevel\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 51 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_no_deeper\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 127 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_king\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_cheat_death\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_respawn_winners\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_respawn_class\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: -(1 as ::core::ffi::c_int),
    },
    borg_setting {
        setting_string: b"borg_respawn_race\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: -(1 as ::core::ffi::c_int),
    },
    borg_setting {
        setting_string: b"borg_chest_fail_tolerance\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 7 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_delay_factor\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_money_scum_amount\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_self_scum\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_lunal_mode\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_self_lunal\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_enchant_limit\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 12 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_dump_level\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 1 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_save_death\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 1 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_on_bell\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_allow_strange_opts\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_autosave\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_restore_ignore_settings\0" as *const u8
            as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: ::core::ptr::null::<::core::ffi::c_char>(),
        setting_type: 0 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn borg_proc_setting(
    mut setting: ::core::ffi::c_int,
    mut setting_string: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
) -> bool {
    let mut negative: bool = false_0 != 0;
    while *line as ::core::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    if !prefix_i(line, setting_string) {
        return false_0 != 0;
    }
    line = line.offset(strlen(setting_string) as isize);
    while *line != 0 {
        let mut ch: ::core::ffi::c_char = *line;
        if ch as ::core::ffi::c_int == ' ' as i32 || ch as ::core::ffi::c_int == '=' as i32 {
            line = line.offset(1);
        } else if type_0 as ::core::ffi::c_int == 'b' as i32 {
            if ch as ::core::ffi::c_int == 'T' as i32
                || ch as ::core::ffi::c_int == 't' as i32
                || ch as ::core::ffi::c_int == '1' as i32
                || ch as ::core::ffi::c_int == 'Y' as i32
                || ch as ::core::ffi::c_int == 'y' as i32
            {
                *borg_cfg.offset(setting as isize) = true_0;
            } else {
                *borg_cfg.offset(setting as isize) = false_0;
            }
            break;
        } else {
            if !(type_0 as ::core::ffi::c_int == 'i' as i32) {
                continue;
            }
            if ch as ::core::ffi::c_int == '-' as i32 {
                negative = true_0 != 0;
                line = line.offset(1);
            } else {
                if !(*(*__ctype_b_loc())
                    .offset(ch as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0)
                {
                    continue;
                }
                *borg_cfg.offset(setting as isize) = atoi(line);
                break;
            }
        }
    }
    if negative {
        *borg_cfg.offset(setting as isize) *= -(1 as ::core::ffi::c_int);
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_free_txt_file() {
    borg_trait_free();
    borg_free_formulas();
    if !borg_cfg.is_null() {
        mem_free(borg_cfg as *mut ::core::ffi::c_void);
    }
    borg_cfg = ::core::ptr::null_mut::<::core::ffi::c_int>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_txt_file() -> bool {
    let mut fp: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut formulas_found: bool = false_0 != 0;
    let mut old_formulas_found: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut warning: bool = false_0 != 0;
    borg_free_txt_file();
    borg_trait_init();
    if strcmp(
        borg_settings[BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as usize].setting_string,
        b"borg_munchkin_level\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        || strcmp(
            borg_settings[BORG_RESPAWN_RACE as ::core::ffi::c_int as usize].setting_string,
            b"borg_respawn_race\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        msg(
            b"borg settings structures not correct.  aborting. \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_USER,
        b"borg.txt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fp = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_READ,
        4294967295 as file_type,
    );
    if fp.is_null() {
        msg(
            b"*****WARNING***** You do not have a proper BORG.TXT file!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg(
            b"writing borg.txt to the \\user\\ subdirectory with default values\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg(
            b"Which is: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        fp = file_open(
            &raw mut buf as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
        if fp.is_null() {
            borg_warning(
                b"*****WARNING***** unable to write default BORG.TXT file!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        file_putf(
            fp,
            b"# BORG.txt default settings \n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# A more descriptive version of this file is delivered. \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# Check your original install for borg.txt and \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# replace this one with the delivered version. \n\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
            if borg_settings[i as usize].setting_type as ::core::ffi::c_int == 'b' as i32 {
                file_putf(
                    fp,
                    b"%s = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_settings[i as usize].setting_string,
                    if borg_settings[i as usize].default_value != 0 {
                        b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
            }
            if borg_settings[i as usize].setting_type as ::core::ffi::c_int == 'i' as i32 {
                file_putf(
                    fp,
                    b"%s = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_settings[i as usize].setting_string,
                    borg_settings[i as usize].default_value,
                );
            }
            i += 1;
        }
        file_close(fp);
        fp = file_open(
            &raw mut buf as *mut ::core::ffi::c_char,
            MODE_READ,
            4294967295 as file_type,
        );
    }
    borg_cfg = mem_alloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(BORG_MAX_SETTINGS as ::core::ffi::c_int as size_t),
    ) as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
        *borg_cfg.offset(i as isize) = borg_settings[i as usize].default_value;
        i += 1;
    }
    while file_getl(
        fp,
        &raw mut buf as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t).wrapping_sub(1 as size_t),
    ) {
        if buf[0 as ::core::ffi::c_int as usize] == 0
            || buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        buf[(::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize)
            .wrapping_sub(1 as usize) as usize] = '\0' as i32 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
            if borg_proc_setting(
                i,
                borg_settings[i as usize].setting_string,
                borg_settings[i as usize].setting_type,
                &raw mut buf as *mut ::core::ffi::c_char,
            ) {
                break;
            }
            i += 1;
        }
        if i != BORG_MAX_SETTINGS as ::core::ffi::c_int {
            continue;
        }
        if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"REQ\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"FORMULA\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"CND\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"POWER\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"[BEGIN FORMULA SECTION]\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            formulas_found = true_0 != 0;
            borg_load_formulas(fp);
        }
    }
    file_close(fp);
    if *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) != 0 {
        warning = true_0 != 0;
        if old_formulas_found {
            borg_warning(
                b"** Borg's dynamic calculations enabled but old formulas found in borg.txt.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            borg_warning(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
            *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
        } else if !formulas_found {
            borg_warning(
                b"** Borg's dynamic calculations enabled but no formulas found in borg.txt.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            borg_warning(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
            *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
        } else {
            borg_warning(
                b"Borg's dynamic calculations enabled.  You may see some performance loss (~20 percent).\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        Term_fresh();
    }
    borg.lunal_mode = *borg_cfg.offset(BORG_LUNAL_MODE as ::core::ffi::c_int as isize) != 0;
    if *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize)
        <= 1 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize) =
            1 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize)
        >= 50 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize) =
            50 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        <= 1 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize) =
            8 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        >= 100 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize) =
            100 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
        <= 8 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize) =
            8 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
        >= 15 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize) =
            15 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize) >= MAX_RACES
        || *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
            < -(1 as ::core::ffi::c_int)
    {
        *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize) >= MAX_CLASSES
        || *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
            < -(1 as ::core::ffi::c_int)
    {
        *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_WINNERS as ::core::ffi::c_int as isize) != 0 {
        *borg_cfg.offset(BORG_STOP_KING as ::core::ffi::c_int as isize) = false_0;
    }
    return warning;
}
#[no_mangle]
pub unsafe extern "C" fn borg_reset_ignore() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if *borg_cfg.offset(BORG_RESTORE_IGNORE_SETTINGS as ::core::ffi::c_int as isize) != 0 {
        if !borg.init_save.kinfo_ignore.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).k_max as ::core::ffi::c_int {
                (*k_info.offset(i as isize)).ignore =
                    *borg.init_save.kinfo_ignore.offset(i as isize);
                i += 1;
            }
        }
        i = ITYPE_NONE as ::core::ffi::c_int;
        while i < ITYPE_MAX as ::core::ffi::c_int {
            *(&raw mut ignore_level as *mut uint8_t).offset(i as isize) =
                borg.init_save.ignore_level[i as usize];
            i += 1;
        }
        if !borg.init_save.ego_ignore_types.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).e_max as ::core::ffi::c_int {
                j = ITYPE_NONE as ::core::ffi::c_int;
                while j < ITYPE_MAX as ::core::ffi::c_int {
                    *(*ego_ignore_types.offset(i as isize)).offset(j as isize) =
                        *(*borg.init_save.ego_ignore_types.offset(i as isize)).offset(j as isize);
                    j += 1;
                }
                i += 1;
            }
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        mem_free(*borg.init_save.ego_ignore_types.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    mem_free(borg.init_save.ego_ignore_types as *mut ::core::ffi::c_void);
    borg.init_save.ego_ignore_types = ::core::ptr::null_mut::<*mut bool>();
    mem_free(borg.init_save.kinfo_ignore as *mut ::core::ffi::c_void);
    borg.init_save.kinfo_ignore = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn borg_init_ignore() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    borg.init_save.kinfo_ignore = mem_alloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*z_info).k_max as size_t),
    ) as *mut uint8_t;
    borg.init_save.ego_ignore_types = mem_zalloc(
        ((*z_info).e_max as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        let ref mut fresh0 = *borg.init_save.ego_ignore_types.offset(i as isize);
        *fresh0 = mem_zalloc(
            (ITYPE_MAX as ::core::ffi::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).k_max as ::core::ffi::c_int {
        *borg.init_save.kinfo_ignore.offset(i as isize) = (*k_info.offset(i as isize)).ignore;
        i += 1;
    }
    i = ITYPE_NONE as ::core::ffi::c_int;
    while i < ITYPE_MAX as ::core::ffi::c_int {
        borg.init_save.ignore_level[i as usize] =
            *(&raw mut ignore_level as *mut uint8_t).offset(i as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        j = ITYPE_NONE as ::core::ffi::c_int;
        while j < ITYPE_MAX as ::core::ffi::c_int {
            *(*borg.init_save.ego_ignore_types.offset(i as isize)).offset(j as isize) =
                *(*ego_ignore_types.offset(i as isize)).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    ignore_birth_init();
}
#[no_mangle]
pub unsafe extern "C" fn borg_reinit_options() {
    borg.init_save.key_mode = if (*player).opts.opt
        [OPT_rogue_like_commands as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        != 0
    {
        KEYMAP_MODE_ROGUE as ::core::ffi::c_int
    } else {
        KEYMAP_MODE_ORIG as ::core::ffi::c_int
    };
    borg_init_ignore();
    option_set(
        b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"auto_more\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"pickup_always\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    option_set(
        b"pickup_detail\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"use_old_target\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"pickup_inven\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    option_set(
        b"show_piles\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"show_labels\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"show_flavors\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"easy_open\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"easy_alter\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    (*player).opts.hitpoint_warn = 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_reset_settings() {
    if borg.init_save.key_mode == KEYMAP_MODE_ROGUE as ::core::ffi::c_int {
        option_set(
            b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
            true_0,
        );
    } else if borg.init_save.key_mode == KEYMAP_MODE_ORIG as ::core::ffi::c_int {
        option_set(
            b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
            false_0,
        );
    }
    borg_reset_ignore();
    borg_free_detection();
}
unsafe extern "C" fn borg_leave_game(
    mut ev_type: game_event_type,
    mut ev_data: *mut game_event_data,
    mut user: *mut ::core::ffi::c_void,
) {
    game_closed = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepare_race_class_info() {
    borg_prepare_book_info();
    game_closed = false_0 != 0;
    event_add_handler(
        EVENT_LEAVE_GAME,
        Some(
            borg_leave_game
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_init() {
    let mut memory_test: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut warning_given: bool = false;
    do_cmd_redraw();
    borg_note(b"Initializing the Borg... (memory)\0" as *const u8 as *const ::core::ffi::c_char);
    borg_init_failure = false_0 != 0;
    Term_fresh();
    memory_test = mem_zalloc(
        ((400 as ::core::ffi::c_long * 1024 as ::core::ffi::c_long) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    mem_free(memory_test as *mut ::core::ffi::c_void);
    memory_test = ::core::ptr::null_mut::<uint8_t>();
    if borg_rand_local == 0 {
        borg_rand_local = (Rand_div(0x10000000 as uint32_t) as int32_t + 1 as int32_t) as uint32_t;
    }
    borg.player = player;
    borg.status.old_depth = 128 as int16_t;
    borg_note(b"Initializing the Borg... (borg.txt)\0" as *const u8 as *const ::core::ffi::c_char);
    warning_given = borg_init_txt_file();
    borg_note(b"Initializing the Borg... (options)\0" as *const u8 as *const ::core::ffi::c_char);
    Term_fresh();
    if screensaver as ::core::ffi::c_int != 0
        || *borg_cfg.offset(BORG_CHEAT_DEATH as ::core::ffi::c_int as isize) != 0
    {
        option_set(
            b"cheat_live\0" as *const u8 as *const ::core::ffi::c_char,
            true_0,
        );
    }
    (*(*player).upkeep).redraw = ((*(*player).upkeep).redraw as ::core::ffi::c_long
        | 0x10000 as ::core::ffi::c_long) as uint32_t;
    borg_note(b"Initializing the Borg... (various)\0" as *const u8 as *const ::core::ffi::c_char);
    Term_fresh();
    borg.status.cheat_death = false_0 != 0;
    if (*player).opts.opt[OPT_cheat_live as ::core::ffi::c_int as usize] {
        borg.status.cheat_death = true_0 != 0;
    }
    borg_init_messages();
    borg_init_io();
    borg_init_item_val();
    borg_init_item_activation();
    borg_init_cave();
    borg_init_flow();
    borg_init_item_wear();
    borg_init_flow_take();
    borg_init_flow_kill();
    borg_init_item();
    borg_init_store();
    borg_init_update();
    borg_prepare_race_class_info();
    borg_note(b"Initializing the Borg... done.\0" as *const u8 as *const ::core::ffi::c_char);
    borg.time.now = 10 as ::core::ffi::c_int as borg_time;
    if strcmp(
        (*player_id2class(0 as guid)).name,
        b"Warrior\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        || strcmp(
            (*player_id2class(1 as guid)).name,
            b"Mage\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(2 as guid)).name,
            b"Druid\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(3 as guid)).name,
            b"Priest\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(4 as guid)).name,
            b"Necromancer\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(5 as guid)).name,
            b"Paladin\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(6 as guid)).name,
            b"Rogue\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(7 as guid)).name,
            b"Ranger\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(8 as guid)).name,
            b"Blackguard\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        borg_note(
            b"**STARTUP FAILURE** classes do not match\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
    if *borg_cfg.offset(BORG_ALLOW_STRANGE_OPTS as ::core::ffi::c_int as isize) == 0 {
        if tile_width as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            || tile_height as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        {
            borg_note(
                b"**STARTUP FAILURE** Scaled tiles not allowed\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must allow up stairs\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if !(*player).opts.opt[OPT_birth_connect_stairs as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must connect stairs\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_no_recall as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must allow recall\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** strange damage calculation\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
    }
    if !borg_init_failure && !warning_given {
        borg_note(b"# Ready...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    borg_initialized = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free() {
    event_remove_handler(
        EVENT_LEAVE_GAME,
        Some(
            borg_leave_game
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    borg_free_update();
    borg_free_item();
    borg_free_store();
    borg_free_flow_kill();
    borg_free_flow_take();
    borg_free_item_wear();
    borg_free_flow();
    borg_free_cave();
    borg_free_io();
    borg_free_messages();
    borg_free_txt_file();
}
