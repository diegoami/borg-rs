pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const TERM_XTRA_NOISE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_DARK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

use crate::headers::borg_fight_attack_h::{successful_target, target_closest};
use crate::headers::borg_fight_defend_h::borg_attempting_refresh_resist;
use crate::headers::borg_flow_glyph_h::{borg_needs_new_sea, track_glyph};
pub use crate::headers::borg_flow_h::borg_track;
pub use crate::headers::borg_flow_kill_h::{
    borg_delete_kill, borg_kill, borg_kills, borg_kills_nxt,
};
use crate::headers::borg_flow_stairs_h::{track_less, track_more};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_flush, borg_keypress, borg_note, borg_what_text};
pub use crate::headers::borg_item_h::{borg_item};
use crate::headers::borg_messages_react_h::{borg_react, borg_react_prompted};
use crate::headers::borg_think_h::borg_oops;
pub use crate::headers::borg_trait_h::{
    C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS, BI_ADETDOOR,
    BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM, BI_AENCH_SWEP,
    BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL, BI_AGLYPH, BI_AHEAL, BI_AHWORD,
    BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID, BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES,
    BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL, BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES,
    BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE, BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS,
    BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED, BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR,
    BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS, BI_BART, BI_BID, BI_BLOWS, BI_BTODAM,
    BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX, BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON,
    BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY, BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN,
    BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING, BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU,
    BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP, BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM,
    BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN, BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND,
    BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO, BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP,
    BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX, BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY,
    BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1, BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD,
    BI_FOOD_CURE_BLIND, BI_FOOD_CURE_CONF, BI_FOOD_HI, BI_FOOD_LO, BI_FRACT, BI_GOLD,
    BI_GOOD_S_CHG, BI_GOOD_W_CHG, BI_HASFIXEXP, BI_HEAVYBOW, BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ,
    BI_IACID, BI_ICOLD, BI_IELEC, BI_IFIRE, BI_INFRA, BI_INT, BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID,
    BI_ISBLIND, BI_ISCONFUSED, BI_ISCUT, BI_ISENCUMB, BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX,
    BI_ISFIXEXP, BI_ISFIXINT, BI_ISFIXLEV, BI_ISFIXSTR, BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL,
    BI_ISGORGED, BI_ISHEAVYSTUN, BI_ISHUNGRY, BI_ISIMAGE, BI_ISPARALYZED, BI_ISPOISONED,
    BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK, BI_KING, BI_LIGHT, BI_MAX, BI_MAXCLEVEL, BI_MAXDEPTH,
    BI_MAXHP, BI_MAXSP, BI_MOD_MOVES, BI_MULTIPLE_BONUSES, BI_NEED_BRAND_WEAPON,
    BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D, BI_NEED_ENCHANT_TO_H, BI_PREP_BIG_FIGHT,
    BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD, BI_RCONF, BI_RDARK, BI_RDIS, BI_RECALL, BI_REG,
    BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS, BI_RLITE, BI_RNTHR, BI_RNXUS, BI_RPOIS, BI_RSHRD,
    BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON, BI_SDEX, BI_SDIG, BI_SFRACT, BI_SHLIFE, BI_SHOTS,
    BI_SINT, BI_SINV, BI_SLING, BI_SPEED, BI_SP_ADJ, BI_SRACID, BI_SRBLIND, BI_SRCH, BI_SRCOLD,
    BI_SRCONF, BI_SRDARK, BI_SRDIS, BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS, BI_SRLITE,
    BI_SRNTHR, BI_SRNXUS, BI_SRPOIS, BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR, BI_STR_INDEX,
    BI_SWIS, BI_THB, BI_THN, BI_THT, BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD, BI_WB_ELEC,
    BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS, BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS,
    BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON, BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL,
    BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL, BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD,
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
use crate::headers::borg_update_h::borg_failure;
pub use crate::headers::cave_h::{
    cave, chunk, connector, heatmap, monster_group, square, square_iswarded, trap,
    C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_message_h::{
    MSG_ACT_ARTIFACT, MSG_AFRAID, MSG_AMBIENT_DAY, MSG_AMBIENT_DNG1, MSG_AMBIENT_DNG2,
    MSG_AMBIENT_DNG3, MSG_AMBIENT_DNG4, MSG_AMBIENT_DNG5, MSG_AMBIENT_NITE, MSG_BELL, MSG_BERSERK,
    MSG_BIRTH, MSG_BLESSED, MSG_BLIND, MSG_BOLD, MSG_BR_ACID, MSG_BR_CHAOS, MSG_BR_DARK,
    MSG_BR_DISEN, MSG_BR_ELEC, MSG_BR_ELEMENTS, MSG_BR_FIRE, MSG_BR_FORCE, MSG_BR_FROST,
    MSG_BR_GAS, MSG_BR_GRAVITY, MSG_BR_INERTIA, MSG_BR_LIGHT, MSG_BR_NETHER, MSG_BR_NEXUS,
    MSG_BR_PLASMA, MSG_BR_SHARDS, MSG_BR_SOUND, MSG_BR_TIME, MSG_CAST_FEAR, MSG_CONFUSED,
    MSG_CREATE_TRAP, MSG_CURSED, MSG_CUT, MSG_DEATH, MSG_DESTROY, MSG_DIG, MSG_DISARM,
    MSG_DRAIN_STAT, MSG_DROP, MSG_DRUGGED, MSG_EAT, MSG_FLEE, MSG_GENERIC, MSG_HERO, MSG_HIT,
    MSG_HITPOINT_WARN, MSG_HITWALL, MSG_HIT_GOOD, MSG_HIT_GREAT, MSG_HIT_HI_GREAT,
    MSG_HIT_HI_SUPERB, MSG_HIT_SUPERB, MSG_HUNGRY, MSG_IDENT_ART, MSG_IDENT_BAD, MSG_IDENT_EGO,
    MSG_INFRARED, MSG_INVULN, MSG_KILL, MSG_KILL_KING, MSG_KILL_UNIQUE, MSG_LEVEL, MSG_LOCKPICK,
    MSG_LOCKPICK_FAIL, MSG_MAX, MSG_MISS, MSG_MONEY1, MSG_MONEY2, MSG_MONEY3, MSG_MON_BEG,
    MSG_MON_BITE, MSG_MON_BUTT, MSG_MON_CLAW, MSG_MON_CRAWL, MSG_MON_CRUSH, MSG_MON_DROOL,
    MSG_MON_ENGULF, MSG_MON_GAZE, MSG_MON_HIT, MSG_MON_INSULT, MSG_MON_KICK, MSG_MON_MOAN,
    MSG_MON_PUNCH, MSG_MON_SPIT, MSG_MON_SPORE, MSG_MON_STING, MSG_MON_TOUCH, MSG_MON_WAIL,
    MSG_MULTIPLY, MSG_NOTHING_TO_OPEN, MSG_NOTICE, MSG_OPENDOOR, MSG_PARALYZED, MSG_POISONED,
    MSG_PRAYER, MSG_PROT_EVIL, MSG_QUAFF, MSG_QUIVER, MSG_RECOVER, MSG_RES_ACID, MSG_RES_COLD,
    MSG_RES_ELEC, MSG_RES_FIRE, MSG_RES_POIS, MSG_RUNE, MSG_SCRAMBLE, MSG_SEE_INVIS, MSG_SHIELD,
    MSG_SHOOT, MSG_SHOOT_HIT, MSG_SHRIEK, MSG_SHUTDOOR, MSG_SLOW, MSG_SPEED, MSG_SPELL,
    MSG_STAIRS_DOWN, MSG_STAIRS_UP, MSG_STORE1, MSG_STORE2, MSG_STORE3, MSG_STORE4, MSG_STORE5,
    MSG_STORE_ENTER, MSG_STORE_HOME, MSG_STORE_LEAVE, MSG_STUDY, MSG_STUN, MSG_SUM_AINU,
    MSG_SUM_ANIMAL, MSG_SUM_DEMON, MSG_SUM_DRAGON, MSG_SUM_HI_DEMON, MSG_SUM_HI_DRAGON,
    MSG_SUM_HI_UNDEAD, MSG_SUM_HOUND, MSG_SUM_HYDRA, MSG_SUM_MONSTER, MSG_SUM_SPIDER,
    MSG_SUM_UNDEAD, MSG_SUM_UNIQUE, MSG_SUM_WRAITH, MSG_TELEPORT, MSG_TPLEVEL, MSG_TPOTHER,
    MSG_USE_STAFF, MSG_WALK, MSG_WIELD, MSG_ZAP_ROD,
};
pub use crate::headers::list_mon_message_h::{
    MON_MSG_0, MON_MSG_10, MON_MSG_20, MON_MSG_35, MON_MSG_50, MON_MSG_75, MON_MSG_95,
    MON_MSG_APPEAR, MON_MSG_BADLY_FROZEN, MON_MSG_BRIEF_PUZZLE, MON_MSG_CATCH_FIRE, MON_MSG_CHANGE,
    MON_MSG_COMMAND, MON_MSG_CONFUSED, MON_MSG_CRINGE_LIGHT, MON_MSG_DAZED, MON_MSG_DESTROYED,
    MON_MSG_DIE, MON_MSG_DISAPPEAR, MON_MSG_DISEN, MON_MSG_DISINTEGRATES, MON_MSG_DISSOLVE,
    MON_MSG_FALL_ASLEEP, MON_MSG_FLEE_IN_TERROR, MON_MSG_FREEZE_SHATTER, MON_MSG_HASTED,
    MON_MSG_HEALTHIER, MON_MSG_HELD, MON_MSG_HIT_AND_RUN, MON_MSG_HIT_HARD, MON_MSG_IMMUNE,
    MON_MSG_LOSE_SKIN, MON_MSG_MAINTAIN_SHAPE, MON_MSG_MANA_DRAIN, MON_MSG_MAX,
    MON_MSG_MORE_AFRAID, MON_MSG_MORE_CONFUSED, MON_MSG_MORE_DAZED, MON_MSG_MORE_HASTED,
    MON_MSG_MORE_SLOWED, MON_MSG_MORIA_DEATH, MON_MSG_NONE, MON_MSG_NOT_AFRAID,
    MON_MSG_NOT_COMMAND, MON_MSG_NOT_CONFUSED, MON_MSG_NOT_DAZED, MON_MSG_NOT_DISEN,
    MON_MSG_NOT_HASTED, MON_MSG_NOT_HELD, MON_MSG_NOT_SLOWED, MON_MSG_QUAKE_DEATH,
    MON_MSG_QUAKE_HURT, MON_MSG_RESIST, MON_MSG_RESIST_A_LOT, MON_MSG_RESIST_SOMEWHAT,
    MON_MSG_SHAPE_FAIL, MON_MSG_SHRIVEL_LIGHT, MON_MSG_SHUDDER, MON_MSG_SLOWED, MON_MSG_SPAWN,
    MON_MSG_UNAFFECTED, MON_MSG_UNHARMED, MON_MSG_WAKES_UP,
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
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::message_h::{C2RustUnnamedH_43, SOUND_MAX};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method, blow_methods};
pub use crate::headers::mon_msg_h::mon_messages;
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    monster_spell, monster_spell_level, monster_spells, pain_messages, MON_ALTMSG_MISS,
    MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER, MON_GROUP_MEMBER,
    MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memcpy, strchr, strcmp, strlen, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
use crate::headers::ui_input_h::inkey_flag;
pub use crate::headers::ui_term_h::{term, term_win, tile_width, Term, Term_xtra};
pub use crate::headers::z_bitflag_h::bitflag;

pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::{format, strnfmt};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, prefix, suffix};
use crate::headers::z_virt_h::{
    mem_alloc, mem_free, mem_realloc, mem_zalloc, string_free, string_make,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_read_messages {
    pub count: ::core::ffi::c_int,
    pub allocated: ::core::ffi::c_int,
    pub messages: *mut borg_read_message,
    pub index: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_read_message {
    pub message_p1: *mut ::core::ffi::c_char,
    pub message_p2: *mut ::core::ffi::c_char,
    pub message_p3: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub msg: *const ::core::ffi::c_char,
    pub omit_subject: bool,
    pub type_0: ::core::ffi::c_int,
}
static mut suffix_pain: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_match: [::core::ffi::c_char; 128] = unsafe {
    ::core::mem::transmute::<
        [u8; 128],
        [::core::ffi::c_char; 128],
    >(
        *b"plain gold ring\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut prefix_kill: [*const ::core::ffi::c_char; 4] = [
    b"You have killed \0" as *const u8 as *const ::core::ffi::c_char,
    b"You have slain \0" as *const u8 as *const ::core::ffi::c_char,
    b"You have destroyed \0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_died: [*const ::core::ffi::c_char; 18] = [
    b" die.\0" as *const u8 as *const ::core::ffi::c_char,
    b" dies.\0" as *const u8 as *const ::core::ffi::c_char,
    b" is destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
    b" are destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
    b" is destroyed!\0" as *const u8 as *const ::core::ffi::c_char,
    b" are destroyed!\0" as *const u8 as *const ::core::ffi::c_char,
    b" shrivel away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
    b" shrivels away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
    b" dissolve!\0" as *const u8 as *const ::core::ffi::c_char,
    b" dissolves!\0" as *const u8 as *const ::core::ffi::c_char,
    b" scream of agony!\0" as *const u8 as *const ::core::ffi::c_char,
    b" screams of agony!\0" as *const u8 as *const ::core::ffi::c_char,
    b" disintegrate!\0" as *const u8 as *const ::core::ffi::c_char,
    b" disintegrates!\0" as *const u8 as *const ::core::ffi::c_char,
    b" freeze and shatter!\0" as *const u8 as *const ::core::ffi::c_char,
    b" freezes and shatters!\0" as *const u8 as *const ::core::ffi::c_char,
    b" is drained dry!\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_blink: [*const ::core::ffi::c_char; 6] = [
    b" disappears!\0" as *const u8 as *const ::core::ffi::c_char,
    b" intones strange words.\0" as *const u8 as *const ::core::ffi::c_char,
    b" teleports away.\0" as *const u8 as *const ::core::ffi::c_char,
    b" blinks.\0" as *const u8 as *const ::core::ffi::c_char,
    b" makes a soft 'pop'.\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_hit_by: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
static mut spell_msgs: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
static mut spell_invis_msgs: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
unsafe extern "C" fn borg_message_contains(
    mut value: *const ::core::ffi::c_char,
    mut message: *mut borg_read_message,
) -> bool {
    if !strstr(value, (*message).message_p1).is_null()
        && ((*message).message_p2.is_null() || !strstr(value, (*message).message_p2).is_null())
        && ((*message).message_p3.is_null() || !strstr(value, (*message).message_p3).is_null())
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
static mut prefix_feeling_danger: [*const ::core::ffi::c_char; 11] = [
    b"You are still uncertain about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omens of death haunt this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems murderous\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems terribly dangerous\0" as *const u8 as *const ::core::ffi::c_char,
    b"You feel anxious about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"You feel nervous about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place does not seem too risky\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems reasonably safe\0" as *const u8 as *const ::core::ffi::c_char,
    b"This seems a tame, sheltered place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This seems a quiet, peaceful place\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_feeling_stuff: [*const ::core::ffi::c_char; 12] = [
    b"Looks like any other level.\0" as *const u8 as *const ::core::ffi::c_char,
    b"you sense an item of wondrous power!\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are superb treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are excellent treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are very good treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are good treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there may be something worthwhile here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there may not be much interesting here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there aren't many treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are only scraps of junk here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there is naught but cobwebs here.\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn borg_get_messages(mut key: *mut keypress, mut cursor: loc) -> bool {
    let mut borg_prompt: bool = false_0 != 0;
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: *mut ::core::ffi::c_char = &raw mut buffer as *mut ::core::ffi::c_char;
    let mut t_a: uint8_t = 0;
    buf = &raw mut buffer as *mut ::core::ffi::c_char;
    borg_what_text(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        ((*Term).wid - 1 as ::core::ffi::c_int) / tile_width as ::core::ffi::c_int,
        &raw mut t_a,
        &raw mut buffer as *mut ::core::ffi::c_char,
    );
    buf = crate::bridge::trim_c_buf(&raw mut buffer);
    if t_a as ::core::ffi::c_int != COLOUR_DARK
        && (*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32)
    {
        borg_prompt = true_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0
        && prefix(buf, b"Type\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        borg_prompt = false_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0 && !inkey_flag && strlen(buf) != 0 {
        if borg_react_prompted(buf, key, cursor) {
            return true_0 != 0;
        }
    }
    buf = &raw mut buffer as *mut ::core::ffi::c_char;
    if borg_prompt as ::core::ffi::c_int != 0
        && !inkey_flag
        && cursor.y == 0 as ::core::ffi::c_int
        && cursor.x >= 7 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == borg_what_text(
                cursor.x - 7 as ::core::ffi::c_int,
                cursor.y,
                7 as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        && suffix(buf, b" -more-\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(b"# message with -more-\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if 0 as ::core::ffi::c_int
            == borg_what_text(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                cursor.x - 7 as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        {
            borg_parse(buf);
        }
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(b"clearing -more-\0" as *const u8 as *const ::core::ffi::c_char);
        }
        (*key).code = ' ' as i32 as keycode_t;
        return true_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0 && inkey_flag as ::core::ffi::c_int != 0 {
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(b"# parse normal message\0" as *const u8 as *const ::core::ffi::c_char);
        }
        buf = &raw mut buffer as *mut ::core::ffi::c_char;
        if 0 as ::core::ffi::c_int
            == borg_what_text(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                ((*Term).wid - 1 as ::core::ffi::c_int) / tile_width as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        {
            let mut k: ::core::ffi::c_int = strlen(buf) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int
                && *buf.offset((k - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == ' ' as i32
            {
                k -= 1;
            }
            *buf.offset(k as isize) = '\0' as i32 as ::core::ffi::c_char;
            borg_parse(buf);
        }
        (*key).code = ' ' as i32 as keycode_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_parse_aux(
    mut msg: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut x9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut who: [::core::ffi::c_char; 256] = [0; 256];
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.goal.g.x, crate::bridge::st().borg.goal.g.y) as *mut borg_grid;
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        borg_note(format(
            b"# Parse Msg bite <%s>\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
    }
    if prefix(
        msg,
        b"You die.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if !((*player).wizard as ::core::ffi::c_int != 0
            || (*player).opts.opt[OPT_cheat_live as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                != 0
            || crate::bridge::st().borg.status.cheat_death as ::core::ffi::c_int != 0)
        {
            borg_oops(b"death\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.status.respawning == 0 {
                crate::bridge::st().borg.status.active = false_0 != 0;
            }
            Term_xtra(TERM_XTRA_NOISE, 1 as ::core::ffi::c_int);
        }
        return;
    }
    if prefix(
        msg,
        b"You failed \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_note(b"# Normal failure.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_failure = true_0 != 0;
        borg_flush();
        successful_target = 0 as ::core::ffi::c_int;
        crate::bridge::st().io.confirm_target = false_0 != 0;
        if borg_attempting_refresh_resist {
            if crate::bridge::st().borg.resistance as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                crate::bridge::st().borg.resistance = (crate::bridge::st().borg.resistance as ::core::ffi::c_int
                    - 25000 as ::core::ffi::c_int) as int16_t;
            }
            borg_attempting_refresh_resist = false_0 != 0;
        }
        return;
    }
    if borg_match[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && !strstr(msg, &raw mut borg_match as *mut ::core::ffi::c_char).is_null()
    {
        crate::bridge::st().borg.status.cancel = true_0 != 0;
    }
    if prefix(
        msg,
        b"You hit a teleport\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"An arrow \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"A small dart \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"The cave \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_react(
            msg,
            b"QUAKE:Somebody\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg_needs_new_sea = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp = strlen(b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"AFRAID:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You feel your memories fade.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.trait_[(BI_ISFORGET as ::core::ffi::c_int) as usize] = true_0;
    }
    if strcmp(
        msg,
        b"Your memories come flooding back.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::bridge::st().borg.trait_[(BI_ISFORGET as ::core::ffi::c_int) as usize] = false_0;
    }
    if strcmp(
        msg,
        b"You have been knocked out.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(b"Ignoring Messages While KO'd\0" as *const u8 as *const ::core::ffi::c_char);
        crate::bridge::st().msgs.dont_react = true_0 != 0;
    }
    if strcmp(
        msg,
        b"You are paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"Ignoring Messages While Paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().msgs.dont_react = true_0 != 0;
    }
    if strcmp(
        msg,
        b"You feel drugged!\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"# Hallucinating.  Special control of wanks.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize] = true_0;
    }
    if strcmp(
        msg,
        b"The draining fails.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_react(
            msg,
            b"MISS_BY:something\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if strcmp(
        msg,
        b"You can see clearly again.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"# Hallucination ended.  Normal control of wanks.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize] = false_0;
    }
    if prefix(
        msg,
        b"You hit \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You hit \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"HIT:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You bite \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You bite \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"HIT:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        target_closest = 1 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"You draw power from\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = 1 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"No Available Target.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"This spell must target a monster.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"Not enough room next to \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"You miss \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You miss \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"MISS:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp = strlen(b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"MISS:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(msg, b"Your \0" as *const u8 as *const ::core::ffi::c_char) {
        if suffix(
            msg,
            b" is unaffected!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            return;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_pain.offset(i as isize)).is_null() {
            if suffix(msg, *suffix_pain.offset(i as isize)) {
                tmp = strlen(*suffix_pain.offset(i as isize)) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"PAIN:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !prefix_kill[i as usize].is_null() {
            if prefix(msg, prefix_kill[i as usize]) {
                tmp = strlen(prefix_kill[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg.offset(tmp as isize),
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"KILL:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !suffix_died[i as usize].is_null() {
            if suffix(msg, suffix_died[i as usize]) {
                tmp = strlen(suffix_died[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"DIED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !suffix_blink[i as usize].is_null() {
            if suffix(msg, suffix_blink[i as usize]) {
                tmp = strlen(suffix_blink[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"BLINK:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        if suffix(
            msg,
            b" misses you.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" misses you.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"MISS_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" is repelled.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" is repelled.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"MISS_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_hit_by.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if borg_message_contains(
                msg,
                suffix_hit_by.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                let mut start: *mut ::core::ffi::c_char =
                    strstr(msg, (*suffix_hit_by.messages.offset(i as isize)).message_p1);
                if !start.is_null() {
                    strnfmt(
                        &raw mut who as *mut ::core::ffi::c_char,
                        start.offset_from(msg) as ::core::ffi::c_long as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        msg,
                    );
                    strnfmt(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        256 as size_t,
                        b"HIT_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut who as *mut ::core::ffi::c_char,
                    );
                    borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                    if track_glyph.num != 0 {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_glyph.num as ::core::ffi::c_int {
                            *track_glyph.x.offset(i as isize) = 0 as ::core::ffi::c_int;
                            *track_glyph.y.offset(i as isize) = 0 as ::core::ffi::c_int;
                            i += 1;
                        }
                        track_glyph.num = 0 as int16_t;
                        if square_iswarded(cave, crate::bridge::st().borg.c) {
                            *track_glyph.x.offset(track_glyph.num as isize) = crate::bridge::st().borg.c.x;
                            *track_glyph.y.offset(track_glyph.num as isize) = crate::bridge::st().borg.c.y;
                            track_glyph.num += 1;
                        }
                    }
                    return;
                }
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*spell_invis_msgs.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if !prefix(
                msg,
                b"Something \0" as *const u8 as *const ::core::ffi::c_char,
            ) && !prefix(msg, b"You \0" as *const u8 as *const ::core::ffi::c_char)
            {
                break;
            }
            if borg_message_contains(
                msg,
                spell_invis_msgs.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"SPELL_%03d:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    *spell_invis_msgs.index.offset(i as isize),
                    b"Something\0" as *const u8 as *const ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*spell_msgs.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if borg_message_contains(
                msg,
                spell_msgs.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                let mut start_0: *mut ::core::ffi::c_char =
                    strstr(msg, (*spell_msgs.messages.offset(i as isize)).message_p1);
                if !start_0.is_null() {
                    strnfmt(
                        &raw mut who as *mut ::core::ffi::c_char,
                        start_0.offset_from(msg) as ::core::ffi::c_long as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        msg,
                    );
                    strnfmt(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        256 as size_t,
                        b"SPELL_%03d:%s\0" as *const u8 as *const ::core::ffi::c_char,
                        *spell_msgs.index.offset(i as isize),
                        &raw mut who as *mut ::core::ffi::c_char,
                    );
                    borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                    return;
                }
            }
            i += 1;
        }
        if suffix(
            msg,
            b" falls asleep!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" falls asleep!\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_SLEEP:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" looks confused.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" looks confused.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_CONFUSED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" looks more confused.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" looks more confused.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_CONFUSED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" wakes up.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" wakes up.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_AWAKE:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" flees in terror!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" flees in terror!\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__FEAR:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers his courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers his courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers her courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers her courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers its courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers its courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
    }
    if strcmp(
        msg,
        b"The door appears to be broken.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int {
            (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"This seems to be permanent rock.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int >= FEAT_GRANITE as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            (*ag).feat = FEAT_PERM as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the granite wall.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::bridge::st().borg.antibounce_count = 1 as int16_t;
        if (*ag).feat as ::core::ffi::c_int >= FEAT_GRANITE as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            (*ag).feat = FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the quartz vein.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA_K as ::core::ffi::c_int {
            (*ag).feat = FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        } else if (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA as ::core::ffi::c_int {
            (*ag).feat = FEAT_QUARTZ as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the magma vein.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ_K as ::core::ffi::c_int {
            (*ag).feat = FEAT_MAGMA_K as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        } else if (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ as ::core::ffi::c_int {
            (*ag).feat = FEAT_MAGMA as ::core::ffi::c_int as uint8_t;
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if prefix(
        msg,
        b"You chip away futilely \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
            && (*ag).kill as ::core::ffi::c_int != 0
        {
            borg_delete_kill((*ag).kill as ::core::ffi::c_int);
        }
        return;
    }
    if prefix(
        msg,
        b"The air about you becomes \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.goal.recalling = 15000 as ::core::ffi::c_int + 5000 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The air around you starts \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.goal.descending = 3000 as ::core::ffi::c_int + 2000 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself yanked \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_flush();
        crate::bridge::st().borg.goal.recalling = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The floor opens beneath you!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_flush();
        crate::bridge::st().borg.goal.descending = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"A tension leaves \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.goal.recalling = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The air around you stops \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.goal.descending = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"Oops!  It feels deathly cold!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"You feel safe from evil!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.prot_from_evil = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel safe from evil.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.prot_from_evil = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself moving faster!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.fast = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself slow down.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.fast = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel righteous\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.bless = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"The prayer has expired.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.bless = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel your mind accelerate.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.fastcast = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel your mind slow again.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.fastcast = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel like a hero!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.hero = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel heroic.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.hero = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel like a killing machine!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.berserk = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel berserk.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.berserk = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"Your eyes feel very sensitive!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.see_inv = 30000 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"Your eyes no longer feel so sensitive.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.see_inv = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a wall \0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
    {
        crate::bridge::st().borg.status.redraw = true_0 != 0;
        crate::bridge::st().borg.status.need_alter = true_0 != 0;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a closed door blocking your way.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && (crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
            && crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                == 0)
    {
        crate::bridge::st().borg.status.redraw = true_0 != 0;
        crate::bridge::st().borg.status.need_alter = true_0 != 0;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"You spin around.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
    {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
                x9 = (*kill).pos.x;
                y9 = (*kill).pos.y;
                ax = if x9 > crate::bridge::st().borg.c.x {
                    x9 - crate::bridge::st().borg.c.x
                } else {
                    crate::bridge::st().borg.c.x - x9
                };
                ay = if y9 > crate::bridge::st().borg.c.y {
                    y9 - crate::bridge::st().borg.c.y
                } else {
                    crate::bridge::st().borg.c.y - y9
                };
                d = if ax < ay { ay } else { ax };
                if d < 4 as ::core::ffi::c_int {
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        crate::bridge::st().borg.status.no_alter = true_0 != 0;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"No known path to \0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"Something is here.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat =
            FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        return;
    }
    if prefix(
        msg,
        b"You see nothing there \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        crate::bridge::st().borg.status.no_alter = true_0 != 0;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"Illegal \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        crate::bridge::st().borg.antibounce_count =
            (crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
        return;
    }
    if prefix(
        msg,
        b"You have nothing to identify\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        crate::bridge::st().borg.antibounce_count =
            (crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
        i = (*z_info).pack_size as ::core::ffi::c_int;
        while i <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
            let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
            if !((*item).iqty == 0) {
                (*item).ident = true_0 != 0;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i <= (*z_info).pack_size as ::core::ffi::c_int {
            let mut item_0: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
            if !((*item_0).iqty == 0) {
                (*item_0).ident = true_0 != 0;
            }
            i += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"Identifying The Phial\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        let mut item_1: *mut borg_item = &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)) as usize]
            as *mut borg_item;
        (*item_1).ident = true_0 != 0;
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        crate::bridge::st().borg.antibounce_count =
            (crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
    }
    if prefix(
        msg,
        b"You feel resistant to acid!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_acid = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to acid.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_acid = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to electricity!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_elec = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to electricity.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_elec = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to fire!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_fire = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to fire.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_fire = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to cold!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_cold = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to cold.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_cold = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to poison!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_pois = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to poison.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.temp.res_pois = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"A mystic shield forms around your body!\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"Your skin turns to stone.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.temp.shield = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"Your mystic shield crumbles away.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"A fleshy shade returns to your skin.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.temp.shield = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You inscribe a mystic symbol on the ground!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 0 as ::core::ffi::c_int;
        while i < track_glyph.num as ::core::ffi::c_int {
            if *track_glyph.x.offset(i as isize) == crate::bridge::st().borg.c.x
                && *track_glyph.y.offset(i as isize) == crate::bridge::st().borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_glyph.num as ::core::ffi::c_int && i < track_glyph.size as ::core::ffi::c_int
        {
            borg_note(
                b"# Noting the creation of a glyph.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            *track_glyph.x.offset(i as isize) = crate::bridge::st().borg.c.x;
            *track_glyph.y.offset(i as isize) = crate::bridge::st().borg.c.y;
            track_glyph.num += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The rune of protection is broken!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 0 as ::core::ffi::c_int;
        while i < track_glyph.num as ::core::ffi::c_int {
            *track_glyph.x.offset(i as isize) = 0 as ::core::ffi::c_int;
            *track_glyph.y.offset(i as isize) = 0 as ::core::ffi::c_int;
            i += 1;
        }
        track_glyph.num = 0 as int16_t;
        if square_iswarded(cave, crate::bridge::st().borg.c) {
            *track_glyph.x.offset(track_glyph.num as isize) = crate::bridge::st().borg.c.x;
            *track_glyph.y.offset(track_glyph.num as isize) = crate::bridge::st().borg.c.y;
            track_glyph.num += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The object resists the spell\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"There is no clear floor\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        *track_glyph.x.offset(track_glyph.num as isize) = 0 as ::core::ffi::c_int;
        *track_glyph.y.offset(track_glyph.num as isize) = 0 as ::core::ffi::c_int;
        track_glyph.num -= 1;
        borg_note(
            b"# Removing the Glyph under me, placing with broken door.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        return;
    }
    if prefix(
        msg,
        b"You have removed the \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        let mut x: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_int = 0;
        y = crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int;
        while y < crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int {
            x = crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int;
            while x < crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int {
                if !(crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0) {
                    if (*ag).feat as ::core::ffi::c_int == FEAT_RUBBLE as ::core::ffi::c_int {
                        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
                    }
                }
                x += 1;
            }
            y += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The enchantment failed\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        crate::bridge::st().borg.antibounce_count = 1 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a searing blast of light!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill_0: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            x9 = (*kill_0).pos.x;
            y9 = (*kill_0).pos.y;
            if !((*kill_0).r_idx == 0) {
                ax = if x9 > crate::bridge::st().borg.c.x {
                    x9 - crate::bridge::st().borg.c.x
                } else {
                    crate::bridge::st().borg.c.x - x9
                };
                ay = if y9 > crate::bridge::st().borg.c.y {
                    y9 - crate::bridge::st().borg.c.y
                } else {
                    crate::bridge::st().borg.c.y - y9
                };
                d = if ax < ay { ay } else { ax };
                if !(d > 12 as ::core::ffi::c_int) {
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        crate::bridge::st().danger.fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize] = 0 as uint16_t;
        return;
    }
    if prefix(
        msg,
        b"You hear a door burst open!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            <= 3 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 5 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.mon.scary = true_0 != 0;
        }
    }
    if prefix(
        msg,
        b"commands you to return.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"teleports you away.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || prefix(
            msg,
            b"gestures at your feet.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.lunal_mode = false_0 != 0;
        borg_note(
            b"# Disconnecting Lunal Mode due to monster spell.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while !prefix_feeling_danger[i as usize].is_null() {
        if prefix(msg, prefix_feeling_danger[i as usize]) {
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"FEELING_DANGER:%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while !suffix_feeling_stuff[i as usize].is_null() {
        if suffix(msg, suffix_feeling_stuff[i as usize]) {
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"FEELING_STUFF:%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_parse(mut msg: *mut ::core::ffi::c_char) {
    static mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 && !msg.is_null() {
        borg_note(format(
            b"# Parsing msg <%s>\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
    }
    if len != 0
        && (msg.is_null()
            || *msg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        j = i;
        while i < len - 1 as ::core::ffi::c_int {
            if buf[i as usize] as ::core::ffi::c_int == '.' as i32
                || buf[i as usize] as ::core::ffi::c_int == '!' as i32
                || buf[i as usize] as ::core::ffi::c_int == '?' as i32
                || buf[i as usize] as ::core::ffi::c_int == '"' as i32
            {
                if buf[(i + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int == ' ' as i32 {
                    buf[(i + 1 as ::core::ffi::c_int) as usize] =
                        '\0' as i32 as ::core::ffi::c_char;
                    borg_parse_aux(
                        (&raw mut buf as *mut ::core::ffi::c_char).offset(j as isize),
                        i + 1 as ::core::ffi::c_int - j,
                    );
                    buf[(i + 1 as ::core::ffi::c_int) as usize] = ' ' as i32 as ::core::ffi::c_char;
                    j = i + 2 as ::core::ffi::c_int;
                    while buf[j as usize] as ::core::ffi::c_int == ' ' as i32 {
                        j += 1;
                    }
                }
            }
            i += 1;
        }
        borg_parse_aux(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(j as isize),
            len - j,
        );
        len = 0 as ::core::ffi::c_int;
    }
    if msg.is_null() {
        len = 0 as ::core::ffi::c_int;
    } else if *msg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        len = (len as size_t).wrapping_add(strnfmt(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
            (1024 as ::core::ffi::c_int - len) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        )) as ::core::ffi::c_int as ::core::ffi::c_int;
    } else {
        len = strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            1024 as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn borg_read_message_equal(
    mut msg1: *mut borg_read_message,
    mut msg2: *mut borg_read_message,
) -> bool {
    if (!(*msg1).message_p1.is_null()
        && !(*msg2).message_p1.is_null()
        && strcmp((*msg1).message_p1, (*msg2).message_p1) == 0
        || (*msg1).message_p1.is_null() && (*msg2).message_p1.is_null())
        && (!(*msg1).message_p2.is_null()
            && !(*msg2).message_p2.is_null()
            && strcmp((*msg1).message_p2, (*msg2).message_p2) == 0
            || (*msg1).message_p2.is_null() && (*msg2).message_p2.is_null())
        && (!(*msg1).message_p3.is_null()
            && !(*msg2).message_p3.is_null()
            && strcmp((*msg1).message_p3, (*msg2).message_p3) == 0
            || (*msg1).message_p3.is_null() && (*msg2).message_p3.is_null())
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn insert_msg(
    mut msgs: *mut borg_read_messages,
    mut msg: *mut borg_read_message,
    mut spell_number: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut found_dup: bool = false_0 != 0;
    if (*msgs).messages.is_null() {
        (*msgs).allocated = 10 as ::core::ffi::c_int;
        (*msgs).messages = mem_alloc(
            (::core::mem::size_of::<borg_read_message>() as size_t)
                .wrapping_mul((*msgs).allocated as size_t),
        ) as *mut borg_read_message;
        (*msgs).index = mem_alloc(
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul((*msgs).allocated as size_t),
        ) as *mut ::core::ffi::c_int;
    }
    if msg.is_null() {
        let ref mut fresh0 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p1;
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh1 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p2;
        *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh2 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p3;
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*msgs).count += 1;
        (*msgs).messages = mem_realloc(
            (*msgs).messages as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<borg_read_message>() as size_t)
                .wrapping_mul((*msgs).count as size_t),
        ) as *mut borg_read_message;
        (*msgs).index = mem_realloc(
            (*msgs).index as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul((*msgs).count as size_t),
        ) as *mut ::core::ffi::c_int;
        (*msgs).allocated = (*msgs).count;
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*msgs).count {
        if borg_read_message_equal(
            (*msgs).messages.offset(i as isize) as *mut borg_read_message,
            msg,
        ) {
            found_dup = true_0 != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found_dup {
        memcpy(
            (*msgs).messages.offset((*msgs).count as isize) as *mut borg_read_message
                as *mut ::core::ffi::c_void,
            msg as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_read_message>() as size_t,
        );
        *(*msgs).index.offset((*msgs).count as isize) = spell_number;
        (*msgs).count += 1;
        if (*msgs).count == (*msgs).allocated {
            (*msgs).allocated += 10 as ::core::ffi::c_int;
            (*msgs).messages = mem_realloc(
                (*msgs).messages as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<borg_read_message>() as size_t)
                    .wrapping_mul((*msgs).allocated as size_t),
            ) as *mut borg_read_message;
            (*msgs).index = mem_realloc(
                (*msgs).index as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                    .wrapping_mul((*msgs).allocated as size_t),
            ) as *mut ::core::ffi::c_int;
        }
    } else {
        string_free((*msg).message_p1);
        string_free((*msg).message_p2);
        string_free((*msg).message_p3);
    };
}
unsafe extern "C" fn clean_msgs(mut msgs: *mut borg_read_messages) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*msgs).count {
        string_free((*(*msgs).messages.offset(i as isize)).message_p1);
        string_free((*(*msgs).messages.offset(i as isize)).message_p2);
        string_free((*(*msgs).messages.offset(i as isize)).message_p3);
        i += 1;
    }
    if !(*msgs).messages.is_null() {
        mem_free((*msgs).messages as *mut ::core::ffi::c_void);
    }
    (*msgs).messages = ::core::ptr::null_mut::<borg_read_message>();
    if !(*msgs).index.is_null() {
        mem_free((*msgs).index as *mut ::core::ffi::c_void);
    }
    (*msgs).index = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*msgs).count = 0 as ::core::ffi::c_int;
    (*msgs).allocated = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_trim_lead_space(
    mut orig: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if orig.is_null()
        || *orig.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
    {
        return orig;
    }
    return orig.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn borg_load_read_message(
    mut message: *mut ::core::ffi::c_char,
    mut read_message: *mut borg_read_message,
) {
    (*read_message).message_p1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*read_message).message_p2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*read_message).message_p3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix_0: *mut ::core::ffi::c_char = strchr(message, '}' as i32);
    if suffix_0.is_null() {
        (*read_message).message_p1 = string_make(borg_trim_lead_space(message));
        return;
    }
    if *message.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '{' as i32 {
        suffix_0 = suffix_0.offset(1);
    } else {
        suffix_0 = message;
    }
    let mut var: *mut ::core::ffi::c_char = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        (*read_message).message_p1 = string_make(borg_trim_lead_space(suffix_0));
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    let mut part_len: ::core::ffi::c_int =
        strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    (*read_message).message_p1 = string_make(format(
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        part_len,
        suffix_0,
    ));
    suffix_0 = suffix_0.offset(part_len as isize);
    suffix_0 = strchr(var, '}' as i32);
    if suffix_0.is_null() {
        return;
    }
    suffix_0 = suffix_0.offset(1);
    var = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            suffix_0 = suffix_0.offset(1);
        }
        if strlen(suffix_0) != 0
            && strcmp(suffix_0, b".\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            (*read_message).message_p2 = string_make(suffix_0);
        }
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    part_len = strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    if part_len != 0 {
        (*read_message).message_p2 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    }
    suffix_0 = suffix_0.offset(part_len as isize);
    suffix_0 = strchr(var, '}' as i32);
    if suffix_0.is_null() {
        return;
    }
    suffix_0 = suffix_0.offset(1);
    var = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            suffix_0 = suffix_0.offset(1);
        }
        if strlen(suffix_0) != 0
            && strcmp(suffix_0, b".\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            if !(*read_message).message_p2.is_null() {
                (*read_message).message_p3 = string_make(borg_trim_lead_space(suffix_0));
            } else {
                (*read_message).message_p2 = string_make(borg_trim_lead_space(suffix_0));
            }
        }
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    part_len = strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    if !(*read_message).message_p2.is_null() {
        (*read_message).message_p3 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    } else {
        (*read_message).message_p2 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    };
}
unsafe extern "C" fn borg_init_spell_messages() {
    let mut spell: *const monster_spell = monster_spells;
    let mut spell_level: *const monster_spell_level = ::core::ptr::null::<monster_spell_level>();
    let mut read_message: borg_read_message = borg_read_message {
        message_p1: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p2: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p3: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    while !spell.is_null() {
        spell_level = (*spell).level;
        while !spell_level.is_null() {
            if !(*spell_level).blind_message.is_null() {
                borg_load_read_message((*spell_level).blind_message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_invis_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            if !(*spell_level).message.is_null() {
                borg_load_read_message((*spell_level).message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            if !(*spell_level).miss_message.is_null() {
                borg_load_read_message((*spell_level).miss_message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            spell_level = (*spell_level).next;
        }
        spell = (*spell).next;
    }
    insert_msg(
        &raw mut spell_invis_msgs,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
    insert_msg(
        &raw mut spell_msgs,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
}
pub const MSG_PARSE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSG_PARSE_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MSG_PARSE_PLURAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn borg_get_parsed_pain(
    mut pain: *const ::core::ffi::c_char,
    mut do_plural: bool,
) -> *mut ::core::ffi::c_char {
    let mut buflen: size_t = strlen(pain).wrapping_add(1 as size_t);
    let mut buf: *mut ::core::ffi::c_char = mem_zalloc(buflen) as *mut ::core::ffi::c_char;
    let mut state: ::core::ffi::c_int = MSG_PARSE_NORMAL;
    let mut maxlen: size_t = strlen(pain);
    let mut pos: size_t = 1 as size_t;
    *buf.offset(0 as ::core::ffi::c_int as isize) = ' ' as i32 as ::core::ffi::c_char;
    let mut i: size_t = 0 as size_t;
    while i < maxlen && pos < buflen.wrapping_sub(1 as size_t) {
        let mut cur: ::core::ffi::c_char = *pain.offset(i as isize);
        if state == MSG_PARSE_NORMAL && cur as ::core::ffi::c_int == '[' as i32 {
            state = MSG_PARSE_SINGLE;
        } else if state == MSG_PARSE_SINGLE && cur as ::core::ffi::c_int == '|' as i32 {
            state = MSG_PARSE_PLURAL;
        } else if state != MSG_PARSE_NORMAL && cur as ::core::ffi::c_int == ']' as i32 {
            state = MSG_PARSE_NORMAL;
        } else if state == MSG_PARSE_NORMAL
            || state == MSG_PARSE_SINGLE && do_plural as ::core::ffi::c_int == false_0
            || state == MSG_PARSE_PLURAL && do_plural as ::core::ffi::c_int == true_0
        {
            let fresh8 = pos;
            pos = pos.wrapping_add(1);
            *buf.offset(fresh8 as isize) = cur;
        }
        i = i.wrapping_add(1);
    }
    return buf;
}
unsafe extern "C" fn borg_insert_pain(
    mut pain: *const ::core::ffi::c_char,
    mut capacity: *mut ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) {
    let mut new_message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *capacity <= *count + 2 as ::core::ffi::c_int {
        *capacity += 14 as ::core::ffi::c_int;
        suffix_pain = mem_realloc(
            suffix_pain as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(*capacity as size_t),
        ) as *mut *mut ::core::ffi::c_char;
    }
    new_message = borg_get_parsed_pain(pain, false_0 != 0);
    let fresh4 = *count;
    *count = *count + 1;
    let ref mut fresh5 = *suffix_pain.offset(fresh4 as isize);
    *fresh5 = new_message;
    new_message = borg_get_parsed_pain(pain, true_0 != 0);
    let fresh6 = *count;
    *count = *count + 1;
    let ref mut fresh7 = *suffix_pain.offset(fresh6 as isize);
    *fresh7 = new_message;
}
static mut borg_msg_repository: [C2RustUnnamed_9; 63] = [
    C2RustUnnamed_9 {
        msg: b"[is|are] hurt.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"die[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s] a lot.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] hit hard.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] immune.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s] somewhat.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] unaffected!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"spawn[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] healthier.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"fall[s] asleep!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"wake[s] up.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"cringe[s] from the light!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shrivel[s] away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"lose[s] some skin!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"dissolve[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"catch[es] fire!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] badly frozen.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shudder[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"change[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"disappear[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] even more stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more slowed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] slowed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"speed[s] up.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] even faster!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"start[s] moving faster.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"slow[s] down.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more terrified!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"flee[s] in terror!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_FLEE as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer afraid.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] frozen to the spot!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"can move again.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"seem[s] less magical!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"seem[s] magical again.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"falls under your spell!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"is no longer under your control.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shimmers for a moment.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"You hear [a|several] scream[|s] of agony!\0" as *const u8
            as *const ::core::ffi::c_char,
        omit_subject: 1 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"disintegrate[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"freeze[s] and shatter[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"lose[s] some mana!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] briefly puzzled.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"maintain[s] the same shape.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] unharmed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"appear[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"There is a puff of smoke!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 1 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] embedded in rock!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"wail[s] out in pain!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn borg_init_pain_messages() {
    let mut capacity: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut pain: *mut monster_pain = ::core::ptr::null_mut::<monster_pain>();
    suffix_pain = mem_alloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul(capacity as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    idx = 0 as ::core::ffi::c_int;
    while idx < (*z_info).mp_max as ::core::ffi::c_int {
        pain = pain_messages.offset(idx as isize) as *mut monster_pain;
        i = 0 as ::core::ffi::c_int;
        while i < 7 as ::core::ffi::c_int {
            if pain.is_null() || (*pain).messages[i as usize].is_null() {
                break;
            }
            borg_insert_pain(
                (*pain).messages[i as usize],
                &raw mut capacity,
                &raw mut count,
            );
            i += 1;
        }
        idx += 1;
    }
    idx = 0 as ::core::ffi::c_int;
    while idx < MON_MSG_MAX as ::core::ffi::c_int {
        if !(borg_msg_repository[idx as usize].type_0 == MSG_KILL as ::core::ffi::c_int) {
            let mut std_pain: *const ::core::ffi::c_char = borg_msg_repository[idx as usize].msg;
            match idx {
                21 | 55 | 56 | 57 | 58 | 59 | 60 | 61 => {}
                _ => {
                    if !std_pain.is_null() {
                        borg_insert_pain(std_pain, &raw mut capacity, &raw mut count);
                    }
                }
            }
        }
        idx += 1;
    }
    if count + 1 as ::core::ffi::c_int != capacity {
        suffix_pain = mem_realloc(
            suffix_pain as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul((count + 1 as ::core::ffi::c_int) as size_t),
        ) as *mut *mut ::core::ffi::c_char;
    }
    let ref mut fresh3 = *suffix_pain.offset(count as isize);
    *fresh3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn borg_init_hit_by_messages() {
    let mut read_message: borg_read_message = borg_read_message {
        message_p1: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p2: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p3: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).blow_methods_max as ::core::ffi::c_int {
        let mut messages: *mut blow_message = (*blow_methods.offset(i as isize)).messages;
        while !messages.is_null() {
            if !(*messages).act_msg.is_null() {
                borg_load_read_message((*messages).act_msg, &raw mut read_message);
                insert_msg(
                    &raw mut suffix_hit_by,
                    &raw mut read_message,
                    (*blow_methods.offset(i as isize)).msgt,
                );
            }
            messages = (*messages).next;
        }
        i += 1;
    }
    insert_msg(
        &raw mut suffix_hit_by,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_messages() {
    borg_free_messages();
    borg_init_spell_messages();
    borg_init_pain_messages();
    borg_init_hit_by_messages();
    let m = &mut crate::bridge::st().msgs;
    m.len = 0 as int16_t;
    m.siz = 4096 as int16_t;
    m.buf = vec![0; m.siz as usize];
    m.num = 0 as int16_t;
    m.max = 256 as int16_t;
    m.pos = vec![0; m.max as usize];
    m.use_ = vec![0; m.max as usize];
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_messages() {
    let mut i: ::core::ffi::c_int = 0;
    {
        // (borg_msg_len and borg_msg_max are left as they are, as in C)
        let m = &mut crate::bridge::st().msgs;
        m.buf = Vec::new();
        m.use_ = Vec::new();
        m.pos = Vec::new();
        m.num = 0 as int16_t;
        m.siz = 0 as int16_t;
    }
    if !suffix_pain.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_pain.offset(i as isize)).is_null() {
            mem_free(*suffix_pain.offset(i as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh9 = *suffix_pain.offset(i as isize);
            *fresh9 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            i += 1;
        }
        mem_free(suffix_pain as *mut ::core::ffi::c_void);
        suffix_pain = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    clean_msgs(&raw mut suffix_hit_by);
    clean_msgs(&raw mut spell_invis_msgs);
    clean_msgs(&raw mut spell_msgs);
}
