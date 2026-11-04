pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const AUTO_VIEW_MAX: ::core::ffi::c_int = 9000 as ::core::ffi::c_int;
pub const BORG_MARK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};
use crate::headers::borg_cave_light_h::borg_update_light;

pub use crate::headers::borg_cave_view_h::{borg_forget_view, borg_update_view};
use crate::headers::borg_danger_h::{
    borg_danger, borg_danger_wipe, borg_fear_monsters, borg_fear_region,
};
use crate::headers::borg_fight_attack_h::successful_target;
use crate::headers::borg_flow_glyph_h::{borg_needs_new_sea, track_glyph};
pub use crate::headers::borg_flow_h::{
    borg_can_dig, borg_data, borg_data_cost, borg_data_flow, borg_data_hard, borg_data_icky,
    borg_data_know, borg_ddx_ddd, borg_ddy_ddd, borg_track, track_closed, track_door, track_step,
};
pub use crate::headers::borg_flow_kill_h::{
    borg_count_death, borg_create_kill, borg_delete_kill, borg_follow_kill, borg_kill, borg_kills,
    borg_kills_cnt, borg_kills_nxt, borg_locate_kill, borg_nasties, borg_nasties_count,
    borg_nasties_num, borg_race_count, borg_race_death, borg_race_name, borg_sleep_kill,
    observe_kill_diff, observe_kill_move,
};
use crate::headers::borg_flow_misc_h::{track_shop_x, track_shop_y, track_vein};
use crate::headers::borg_flow_stairs_h::{track_less, track_more};
pub use crate::headers::borg_flow_take_h::{
    borg_delete_take, borg_follow_take, borg_take, borg_takes, borg_takes_cnt, borg_takes_nxt,
    observe_take_diff, observe_take_move,
};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_inventory_h::borg_do_crush_junk;
use crate::headers::borg_io_h::borg_note;
use crate::headers::borg_item_wear_h::{borg_clear_best, track_worn_num};
use crate::headers::borg_messages_h::{
    borg_msg_buf, borg_msg_len, borg_msg_num, borg_msg_pos, borg_msg_use,
};
use crate::headers::borg_projection_h::{borg_los, borg_target_unknown_wall};
use crate::headers::borg_store_buy_h::{
    bought_item_num, bought_item_nxt, bought_item_pval, bought_item_store, bought_item_sval,
    bought_item_tval,
};

use crate::headers::borg_store_sell_h::{
    sold_item_num, sold_item_nxt, sold_item_pval, sold_item_store, sold_item_sval, sold_item_tval,
};
use crate::headers::borg_think_dungeon_h::{borg_feeling_danger, borg_feeling_stuff};
use crate::headers::borg_think_h::{borg_do_spell, borg_oops};
pub use crate::headers::borg_trait_h::{
    borg_game_ratio, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
pub use crate::headers::cave_h::{
    cave, cave_monster, chunk, connector, ddx_ddd, ddy_ddd, distance, feat_is_shop, grid_data,
    grid_light_level, heatmap, map_info, monster_group, square, square_in_bounds,
    square_in_bounds_fully, square_isdisarmabletrap, square_isvault, square_iswebbed,
    square_shopnum, C2RustUnnamedH_18, FEAT_MAX, LIGHTING_DARK, LIGHTING_LIT, LIGHTING_LOS,
    LIGHTING_MAX, LIGHTING_TORCH,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_mon_race_flags_h::{
    RF_ANIMAL, RF_ATTR_CLEAR, RF_ATTR_FLICKER, RF_ATTR_MULTI, RF_ATTR_RAND, RF_BASH_DOOR,
    RF_CHAR_CLEAR, RF_CLEAR_WEB, RF_COLD_BLOOD, RF_DEMON, RF_DRAGON, RF_DROP_1, RF_DROP_2,
    RF_DROP_20, RF_DROP_3, RF_DROP_4, RF_DROP_40, RF_DROP_60, RF_DROP_GOOD, RF_DROP_GREAT,
    RF_EMPTY_MIND, RF_EVIL, RF_FEMALE, RF_FORCE_DEPTH, RF_FORCE_EXTRA, RF_FORCE_SLEEP,
    RF_FRIGHTENED, RF_GIANT, RF_GROUP_AI, RF_HURT_COLD, RF_HURT_FIRE, RF_HURT_LIGHT, RF_HURT_ROCK,
    RF_IM_ACID, RF_IM_COLD, RF_IM_DISEN, RF_IM_ELEC, RF_IM_FIRE, RF_IM_NETHER, RF_IM_NEXUS,
    RF_IM_PLASMA, RF_IM_POIS, RF_IM_WATER, RF_INVISIBLE, RF_KILL_BODY, RF_KILL_ITEM, RF_KILL_WALL,
    RF_MALE, RF_METAL, RF_MIMIC_INV, RF_MOVE_BODY, RF_MULTIPLY, RF_NAME_COMMA, RF_NEVER_BLOW,
    RF_NEVER_MOVE, RF_NONE, RF_NONLIVING, RF_NO_CONF, RF_NO_FEAR, RF_NO_HOLD, RF_NO_SLEEP,
    RF_NO_SLOW, RF_NO_STUN, RF_ONLY_GOLD, RF_ONLY_ITEM, RF_OPEN_DOOR, RF_ORC, RF_PASS_WALL,
    RF_PASS_WEB, RF_POWERFUL, RF_QUESTOR, RF_RAND_25, RF_RAND_50, RF_REGENERATE, RF_SEASONAL,
    RF_SMART, RF_SMASH_WALL, RF_SPIRIT, RF_STUPID, RF_TAKE_ITEM, RF_TROLL, RF_UNAWARE, RF_UNDEAD,
    RF_UNIQUE, RF_WEIRD_MIND,
};
pub use crate::headers::list_mon_spells_h::{
    RSF_ARROW, RSF_BA_ACID, RSF_BA_COLD, RSF_BA_DARK, RSF_BA_ELEC, RSF_BA_FIRE, RSF_BA_HOLY,
    RSF_BA_LIGHT, RSF_BA_MANA, RSF_BA_NETH, RSF_BA_POIS, RSF_BA_SHAR, RSF_BA_WATE, RSF_BE_ELEC,
    RSF_BE_NETH, RSF_BLIND, RSF_BLINK, RSF_BOLT, RSF_BOULDER, RSF_BO_ACID, RSF_BO_COLD,
    RSF_BO_ELEC, RSF_BO_FIRE, RSF_BO_ICE, RSF_BO_MANA, RSF_BO_NETH, RSF_BO_PLAS, RSF_BO_POIS,
    RSF_BO_WATE, RSF_BRAIN_SMASH, RSF_BR_ACID, RSF_BR_CHAO, RSF_BR_COLD, RSF_BR_DARK, RSF_BR_DISE,
    RSF_BR_ELEC, RSF_BR_FIRE, RSF_BR_GRAV, RSF_BR_INER, RSF_BR_LIGHT, RSF_BR_MANA, RSF_BR_NETH,
    RSF_BR_NEXU, RSF_BR_PLAS, RSF_BR_POIS, RSF_BR_SHAR, RSF_BR_SOUN, RSF_BR_TIME, RSF_BR_WALL,
    RSF_CONF, RSF_DARKNESS, RSF_DRAIN_MANA, RSF_FORGET, RSF_HASTE, RSF_HEAL, RSF_HEAL_KIN,
    RSF_HOLD, RSF_MAX, RSF_MIND_BLAST, RSF_MISSILE, RSF_NONE, RSF_SCARE, RSF_SHAPECHANGE, RSF_SHOT,
    RSF_SHRIEK, RSF_SLOW, RSF_SPIT, RSF_STORM, RSF_S_AINU, RSF_S_ANIMAL, RSF_S_DEMON, RSF_S_DRAGON,
    RSF_S_HI_DEMON, RSF_S_HI_DRAGON, RSF_S_HI_UNDEAD, RSF_S_HOUND, RSF_S_HYDRA, RSF_S_KIN,
    RSF_S_MONSTER, RSF_S_MONSTERS, RSF_S_SPIDER, RSF_S_UNDEAD, RSF_S_UNIQUE, RSF_S_WRAITH,
    RSF_TELE_AWAY, RSF_TELE_LEVEL, RSF_TELE_SELF_TO, RSF_TELE_TO, RSF_TPORT, RSF_TRAPS, RSF_WEAVE,
    RSF_WHIP, RSF_WOUND,
};
pub use crate::headers::list_mon_timed_h::{
    MON_TMD_CHANGED, MON_TMD_COMMAND, MON_TMD_CONF, MON_TMD_DISEN, MON_TMD_FAST, MON_TMD_FEAR,
    MON_TMD_HOLD, MON_TMD_MAX, MON_TMD_SLEEP, MON_TMD_SLOW, MON_TMD_STUN,
};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::list_trap_flags_h::{
    TRF_DELAY, TRF_DOWN, TRF_FLOOR, TRF_GLYPH, TRF_INVISIBLE, TRF_LOCK, TRF_MAGICAL, TRF_NONE,
    TRF_ONETIME, TRF_PIT, TRF_SAVE_ARMOR, TRF_SAVE_THROW, TRF_TRAP, TRF_VISIBLE, TRF_WEB,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::mon_timed_h::C2RustUnnamedH_33;
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, C2RustUnnamedH_30, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN,
    MON_GROUP_BODYGUARD, MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
    RF_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::stdlib_h::{__compar_fn_t, atoi, qsort};
use crate::headers::string_h::{memcpy, memset, strchr};
pub use crate::headers::target_h::target;
pub use crate::headers::trap_h::{trap, trap_info, trap_kind, C2RustUnnamedH_48, TRF_MAX};
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width, Term,
};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, prefix};
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_wank {
    pub x: uint8_t,
    pub y: uint8_t,
    pub t_a: uint8_t,
    pub t_c: wchar_t,
    pub is_take: bool,
    pub is_kill: bool,
}
static mut borg_wank_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut borg_wanks: *mut borg_wank = ::core::ptr::null::<borg_wank>() as *mut borg_wank;
#[no_mangle]
pub static mut borg_failure: bool = false;
#[no_mangle]
pub static mut borg_detect_wall: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_trap: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_door: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_evil: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_obj: *mut *mut bool = ::core::ptr::null::<*mut bool>() as *mut *mut bool;
static mut old_panels: loc = loc {
    x: -(1 as ::core::ffi::c_int),
    y: -(1 as ::core::ffi::c_int),
};
#[no_mangle]
pub static mut borg_do_update_view: bool = false_0 != 0;
#[no_mangle]
pub static mut borg_do_update_lite: bool = false_0 != 0;
static mut o_w_x: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut o_w_y: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut old_c: loc = loc {
    x: -(1 as ::core::ffi::c_int),
    y: -(1 as ::core::ffi::c_int),
};
unsafe extern "C" fn borg_forget_map() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        x = 0 as ::core::ffi::c_int;
        while x < AUTO_MAX_X {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            memset(
                ag as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<borg_grid>() as size_t,
            );
            (*ag).feat = FEAT_PERM as ::core::ffi::c_int as uint8_t;
            x += 1;
        }
        y += 1;
    }
    y = 1 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            (*ag).feat = FEAT_NONE as ::core::ffi::c_int as uint8_t;
            if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 0
            {
                (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
            }
            x += 1;
        }
        y += 1;
    }
    memcpy(
        borg_data_cost as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    memcpy(
        borg_data_flow as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    memset(
        borg_data_know as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    memset(
        borg_data_icky as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    borg_forget_view();
}
unsafe extern "C" fn borg_update_map() {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g: grid_data = grid_data {
        m_idx: 0,
        f_idx: 0,
        first_kind: ::core::ptr::null_mut::<object_kind>(),
        trap: ::core::ptr::null_mut::<trap>(),
        multiple_objects: false,
        unseen_object: false,
        unseen_money: false,
        lighting: LIGHTING_LOS,
        in_view: false,
        is_player: false,
        hallucinate: false,
    };
    dy = 0 as ::core::ffi::c_int;
    while dy
        < ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        let mut current_block_150: u64;
        dx = 0 as ::core::ffi::c_int;
        while dx
            < ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut old_wall: bool = false;
            let mut new_wall: bool = false;
            x = crate::bridge::st().core.w_x + dx;
            y = crate::bridge::st().core.w_y + dy;
            let mut l: loc = loc(x, y);
            if square_in_bounds(cave, l) {
                map_info(l, &raw mut g);
                ag = crate::bridge::grid(x, y) as *mut borg_grid;
                (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_OKAY) as uint16_t;
                if g.f_idx != FEAT_NONE as ::core::ffi::c_int as uint32_t
                    && (g.in_view as ::core::ffi::c_int != 0
                        || (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0)
                {
                    if g.in_view {
                        (*ag).info =
                            ((*ag).info as ::core::ffi::c_int & !BORG_IGNORE_MAP) as uint16_t;
                    }
                    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_MARK) as uint16_t;
                    (*ag).feat = g.f_idx as uint8_t;
                }
                (*ag).store = BORG_HOME as uint8_t;
                if g.is_player {
                    crate::bridge::st().borg.c.x = x;
                    crate::bridge::st().borg.c.y = y;
                }
                old_wall = !crate::borg_cave_util::borg_cave_floor_grid(&*(ag));
                if feat_is_shop(g.f_idx as ::core::ffi::c_int) {
                    (*ag).feat = g.f_idx as uint8_t;
                    i = square_shopnum(cave, l);
                    (*ag).store = i as uint8_t;
                    *track_shop_x.offset(i as isize) = x;
                    *track_shop_y.offset(i as isize) = y;
                    current_block_150 = 18002345992382212654;
                } else if square_isdisarmabletrap(cave, l) {
                    (*ag).trap = true_0 != 0;
                    let mut t_idx: uint8_t = (*(*square(cave, l)).trap).t_idx;
                    if flag_has(
                        &raw mut (*trap_info.offset(t_idx as isize)).flags as *mut bitflag,
                        (TRF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        TRF_GLYPH as ::core::ffi::c_int,
                    ) {
                        (*ag).glyph = true_0 != 0;
                        i = 0 as ::core::ffi::c_int;
                        while i < track_glyph.num as ::core::ffi::c_int {
                            if *track_glyph.x.offset(i as isize) == x
                                && *track_glyph.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_glyph.num as ::core::ffi::c_int
                            && i < track_glyph.size as ::core::ffi::c_int
                        {
                            *track_glyph.x.offset(i as isize) = x;
                            *track_glyph.y.offset(i as isize) = y;
                            track_glyph.num += 1;
                        }
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_NONE as ::core::ffi::c_int as uint32_t {
                    (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_GLOW) as uint16_t;
                    if (*ag).feat as ::core::ffi::c_int != FEAT_NONE as ::core::ffi::c_int {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_DARK) as uint16_t;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_NONE as ::core::ffi::c_int as uint32_t {
                    crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
                        != 0;
                    if g.lighting as ::core::ffi::c_uint
                        == LIGHTING_DARK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_DARK) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_GLOW) as uint16_t;
                    } else if g.lighting as ::core::ffi::c_uint
                        == LIGHTING_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    } else {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_OPEN as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_BROKEN as ::core::ffi::c_int as uint32_t
                {
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_GRANITE as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_PERM as ::core::ffi::c_int as uint32_t
                {
                    if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            != 0
                        && x != 0
                        && y != 0
                        && x != (*cave).width - 1 as ::core::ffi::c_int
                        && y != (*cave).height - 1 as ::core::ffi::c_int
                    {
                        crate::bridge::st().borg.status.vault = true_0 != 0;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_LAVA as ::core::ffi::c_int as uint32_t {
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_MAGMA as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_QUARTZ as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_RUBBLE as ::core::ffi::c_int as uint32_t
                {
                    if crate::bridge::st().borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
                        if !borg_can_dig(
                            true_0 != 0,
                            FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t,
                        ) && g.f_idx == FEAT_QUARTZ as ::core::ffi::c_int as uint32_t
                        {
                            current_block_150 = 10680521327981672866;
                        } else {
                            i = 0 as ::core::ffi::c_int;
                            while i < track_vein.num as ::core::ffi::c_int {
                                if *track_vein.x.offset(i as isize) == x
                                    && *track_vein.y.offset(i as isize) == y
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == track_vein.num as ::core::ffi::c_int
                                && i < track_vein.size as ::core::ffi::c_int
                            {
                                *track_vein.x.offset(i as isize) = x;
                                *track_vein.y.offset(i as isize) = y;
                                track_vein.num += 1;
                                if track_vein.num as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
                                    track_vein.num = 99 as int16_t;
                                }
                            }
                            current_block_150 = 18002345992382212654;
                        }
                    } else {
                        current_block_150 = 18002345992382212654;
                    }
                } else {
                    if g.f_idx == FEAT_MAGMA_K as ::core::ffi::c_int as uint32_t
                        || g.f_idx == FEAT_QUARTZ_K as ::core::ffi::c_int as uint32_t
                    {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_vein.num as ::core::ffi::c_int {
                            if *track_vein.x.offset(i as isize) == x
                                && *track_vein.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_vein.num as ::core::ffi::c_int
                            && i < track_vein.size as ::core::ffi::c_int
                        {
                            *track_vein.x.offset(i as isize) = x;
                            *track_vein.y.offset(i as isize) = y;
                            track_vein.num += 1;
                            if track_vein.num as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
                                track_vein.num = 99 as int16_t;
                            }
                        }
                    } else if g.f_idx == FEAT_CLOSED as ::core::ffi::c_int as uint32_t {
                        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                            <= 5 as ::core::ffi::c_int
                        {
                            i = 0 as ::core::ffi::c_int;
                            while i < track_closed.num as ::core::ffi::c_int {
                                if *track_closed.x.offset(i as isize) == x
                                    && *track_closed.y.offset(i as isize) == y
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == track_closed.num as ::core::ffi::c_int
                                && i < track_closed.size as ::core::ffi::c_int
                            {
                                *track_closed.x.offset(i as isize) = x;
                                *track_closed.y.offset(i as isize) = y;
                                track_closed.num += 1;
                                if track_closed.num as ::core::ffi::c_int
                                    > 254 as ::core::ffi::c_int
                                {
                                    track_closed.num = 254 as int16_t;
                                }
                            }
                        }
                    } else if g.f_idx == FEAT_LESS as ::core::ffi::c_int as uint32_t {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_less.num as ::core::ffi::c_int {
                            if *track_less.x.offset(i as isize) == x
                                && *track_less.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_less.num as ::core::ffi::c_int
                            && i < track_less.size as ::core::ffi::c_int
                        {
                            *track_less.x.offset(i as isize) = x;
                            *track_less.y.offset(i as isize) = y;
                            track_less.num += 1;
                        }
                    } else if g.f_idx == FEAT_MORE as ::core::ffi::c_int as uint32_t {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_more.num as ::core::ffi::c_int {
                            if *track_more.x.offset(i as isize) == x
                                && *track_more.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_more.num as ::core::ffi::c_int
                            && i < track_more.size as ::core::ffi::c_int
                        {
                            *track_more.x.offset(i as isize) = x;
                            *track_more.y.offset(i as isize) = y;
                            track_more.num += 1;
                        }
                    }
                    current_block_150 = 18002345992382212654;
                }
                match current_block_150 {
                    10680521327981672866 => {}
                    _ => {
                        if (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
                            && square_iswebbed(cave, l) as ::core::ffi::c_int != 0
                        {
                            (*ag).web = true_0 != 0;
                        } else {
                            (*ag).web = false_0 != 0;
                        }
                        if (!g.first_kind.is_null() || g.m_idx != 0)
                            && crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                                == 0
                        {
                            let mut wank: *mut borg_wank = ::core::ptr::null_mut::<borg_wank>();
                            let mut is_mimic: bool = false_0 != 0;
                            if borg_wank_num == AUTO_VIEW_MAX {
                                borg_note(format(
                                    b"# Wank problem at grid (%d,%d) m:%lu o:%lu, borg at (%d,%d)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    y,
                                    x,
                                    g.m_idx as ::core::ffi::c_ulong,
                                    (if !g.first_kind.is_null() {
                                        (*g.first_kind).kidx
                                    } else {
                                        0 as uint32_t
                                    }) as ::core::ffi::c_ulong,
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                ));
                                borg_oops(
                                    b"too many objects...\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            let fresh0 = borg_wank_num;
                            borg_wank_num = borg_wank_num + 1;
                            wank = borg_wanks.offset(fresh0 as isize) as *mut borg_wank;
                            (*wank).x = x as uint8_t;
                            (*wank).y = y as uint8_t;
                            if g.m_idx != 0 {
                                let mut m_ptr: *mut monster =
                                    cave_monster(cave, g.m_idx as ::core::ffi::c_int);
                                (*wank).t_a = (*m_ptr).attr;
                                (*wank).t_c =
                                    (*r_info.offset((*(*m_ptr).race).ridx as isize)).d_char;
                                if !(*m_ptr).mimicked_obj.is_null()
                                    && (*m_ptr).m_timed
                                        [MON_TMD_SLEEP as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        != 0
                                {
                                    is_mimic = true_0 != 0;
                                }
                            } else {
                                (*wank).t_a = (*g.first_kind).d_attr;
                                (*wank).t_c = (*g.first_kind).d_char;
                            }
                            (*wank).is_take =
                                !g.first_kind.is_null() || is_mimic as ::core::ffi::c_int != 0;
                            (*wank).is_kill = g.m_idx != 0 as uint32_t && !is_mimic;
                        }
                        new_wall = !crate::borg_cave_util::borg_cave_floor_grid(&*(ag));
                        if old_wall as ::core::ffi::c_int != new_wall as ::core::ffi::c_int {
                            if new_wall {
                                (*borg_data_flow).data[y as usize][x as usize] = 255 as uint8_t;
                            }
                            (*borg_data_know).data[y as usize][x as usize] = false_0 as uint8_t;
                            (*borg_data_icky).data[y as usize][x as usize] = false_0 as uint8_t;
                            if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0 {
                                borg_do_update_view = true_0 != 0;
                            }
                            if (*ag).info as ::core::ffi::c_int & BORG_LIGHT != 0 {
                                borg_do_update_lite = true_0 != 0;
                            }
                            crate::bridge::st().borg.time.wizard_light = 0 as ::core::ffi::c_int as borg_time;
                            crate::bridge::st().borg.time.detect_walls = 0 as ::core::ffi::c_int as borg_time;
                        }
                    }
                }
            }
            dx += 1;
        }
        dy += 1;
    }
}
unsafe extern "C" fn borg_fear_grid(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) {
    let mut x1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return;
    }
    if square_isvault(cave, loc(x, y)) {
        return;
    }
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        == 50 as ::core::ffi::c_int
    {
        k = k * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    x1 = -(6 as ::core::ffi::c_int);
    while x1 <= 6 as ::core::ffi::c_int {
        y1 = -(6 as ::core::ffi::c_int);
        while y1 <= 6 as ::core::ffi::c_int {
            if !(x + x1 <= 0 as ::core::ffi::c_int || x1 + x >= AUTO_MAX_X) {
                if !(y + y1 <= 0 as ::core::ffi::c_int || y1 + y >= AUTO_MAX_Y) {
                    if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                        borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                            (borg_fear_monsters[(y + y1) as usize][(x + x1) as usize]
                                as ::core::ffi::c_int
                                + k / 8 as ::core::ffi::c_int)
                                as uint16_t;
                    }
                    if !(x1 <= -(5 as ::core::ffi::c_int) || x1 >= 5 as ::core::ffi::c_int) {
                        if !(y1 <= -(5 as ::core::ffi::c_int) || y1 >= 5 as ::core::ffi::c_int) {
                            if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                                borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                                    (borg_fear_monsters[(y + y1) as usize][(x + x1) as usize]
                                        as ::core::ffi::c_int
                                        + k / 5 as ::core::ffi::c_int)
                                        as uint16_t;
                            }
                            if !(x1 <= -(3 as ::core::ffi::c_int) || x1 >= 3 as ::core::ffi::c_int)
                            {
                                if !(y1 <= -(3 as ::core::ffi::c_int)
                                    || y1 >= 3 as ::core::ffi::c_int)
                                {
                                    if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                                        borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                                            (borg_fear_monsters[(y + y1) as usize]
                                                [(x + x1) as usize]
                                                as ::core::ffi::c_int
                                                + k / 3 as ::core::ffi::c_int)
                                                as uint16_t;
                                    }
                                    if !(x1 <= -(2 as ::core::ffi::c_int)
                                        || x1 >= 2 as ::core::ffi::c_int)
                                    {
                                        if !(y1 <= -(2 as ::core::ffi::c_int)
                                            || y1 >= 2 as ::core::ffi::c_int)
                                        {
                                            if borg_los(
                                                (*kill).pos.y,
                                                (*kill).pos.x,
                                                y + y1,
                                                x + x1,
                                            ) {
                                                borg_fear_monsters[(y + y1) as usize]
                                                    [(x + x1) as usize] = (borg_fear_monsters
                                                    [(y + y1) as usize]
                                                    [(x + x1) as usize]
                                                    as ::core::ffi::c_int
                                                    + k / 2 as ::core::ffi::c_int)
                                                    as uint16_t;
                                            }
                                            if !(x1 <= -(1 as ::core::ffi::c_int)
                                                || x1 >= 1 as ::core::ffi::c_int)
                                            {
                                                if !(y1 <= -(1 as ::core::ffi::c_int)
                                                    || y1 >= 1 as ::core::ffi::c_int)
                                                {
                                                    if borg_los(
                                                        (*kill).pos.y,
                                                        (*kill).pos.x,
                                                        y + y1,
                                                        x + x1,
                                                    ) {
                                                        borg_fear_monsters[(y + y1) as usize]
                                                            [(x + x1) as usize] =
                                                            (borg_fear_monsters[(y + y1) as usize]
                                                                [(x + x1) as usize]
                                                                as ::core::ffi::c_int
                                                                + k)
                                                                as uint16_t;
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
            y1 += 1;
        }
        x1 += 1;
    }
}
unsafe extern "C" fn borg_fear_regional(
    mut who: *const ::core::ffi::c_char,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut seen_guy: bool,
) {
    let mut x0: ::core::ffi::c_int = 0;
    let mut y0: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    if square_isvault(cave, loc(x, y)) {
        return;
    }
    if seen_guy {
        borg_note(format(
            b"# Fearing region value %d.\0" as *const u8 as *const ::core::ffi::c_char,
            k,
        ));
    } else {
        borg_note(format(
            b"# Fearing region (%d,%d) value %d because of a non-LOS %s\0" as *const u8
                as *const ::core::ffi::c_char,
            y,
            x,
            k,
            who,
        ));
        crate::bridge::st().borg.need_shift_panel = true_0 != 0;
    }
    y0 = y / 11 as ::core::ffi::c_int;
    x0 = x / 11 as ::core::ffi::c_int;
    y1 = if y0 > 0 as ::core::ffi::c_int {
        y0 - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    x1 = if x0 > 0 as ::core::ffi::c_int {
        x0 - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    y2 = if y0 < 5 as ::core::ffi::c_int {
        y0 + 1 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    };
    x2 = if x0 < 17 as ::core::ffi::c_int {
        x0 + 1 as ::core::ffi::c_int
    } else {
        17 as ::core::ffi::c_int
    };
    borg_fear_region[y0 as usize][x0 as usize] =
        (borg_fear_region[y0 as usize][x0 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y0 as usize][x1 as usize] =
        (borg_fear_region[y0 as usize][x1 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y0 as usize][x2 as usize] =
        (borg_fear_region[y0 as usize][x2 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y1 as usize][x0 as usize] = (borg_fear_region[y1 as usize][x0 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x0 as usize] = (borg_fear_region[y2 as usize][x0 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y1 as usize][x1 as usize] = (borg_fear_region[y1 as usize][x1 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y1 as usize][x2 as usize] = (borg_fear_region[y1 as usize][x2 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x1 as usize] = (borg_fear_region[y2 as usize][x1 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x2 as usize] = (borg_fear_region[y2 as usize][x2 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn borg_fear_spell(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ouch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.oldchp as ::core::ffi::c_int
        > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
    {
        ouch = (crate::bridge::st().borg.oldchp as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
            * 2 as ::core::ffi::c_int;
    }
    match i {
        1 => {
            p += 10 as ::core::ffi::c_int;
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 5 as ::core::ffi::c_int
            {
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
                crate::bridge::st().borg.goal.fleeing = crate::bridge::st().borg.goal.leaving;
            }
        }
        2 => {
            p = 100 as ::core::ffi::c_int;
        }
        4 => {
            z = ouch;
        }
        5 => {
            z = ouch;
        }
        6 => {
            z = ouch;
        }
        7 => {
            if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        8 => {
            if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        9 => {
            if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        10 => {
            if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        11 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0) {
                if !crate::bridge::st().borg.temp.res_pois {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        12 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        >= 50 as ::core::ffi::c_int)
                    {
                        p += 150 as ::core::ffi::c_int;
                    }
                }
            }
        }
        13 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0) {
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        14 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0) {
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        15 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        16 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0) {
                p += 200 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    == 50 as ::core::ffi::c_int)
                {
                    p += 100 as ::core::ffi::c_int;
                }
            }
        }
        17 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0) {
                p += 500 as ::core::ffi::c_int;
            }
        }
        18 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0) {
                p += 100 as ::core::ffi::c_int;
            }
        }
        19 => {
            z = ouch;
            p += 200 as ::core::ffi::c_int;
        }
        20 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        21 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
            if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        22 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        23 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        24 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        25 => {
            p += 50 as ::core::ffi::c_int;
        }
        26 => {
            z = ouch;
            p += 40 as ::core::ffi::c_int;
        }
        28 => {
            if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        29 => {
            if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        30 => {
            if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        31 => {
            if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        32 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0) {
                p += 20 as ::core::ffi::c_int;
            }
        }
        33 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0) {
                p += 20 as ::core::ffi::c_int;
            }
        }
        34 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                p += 300 as ::core::ffi::c_int;
            }
        }
        35 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        36 => {
            z = ouch;
        }
        37 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        38 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0) {
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        39 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0) {
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        40 => {
            z = ouch;
            if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                    != 0
                {
                    p += 50 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                    != 0
                {
                    p += 100 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0) {
                    p += 100 as ::core::ffi::c_int;
                }
            }
        }
        41 => {
            if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] != 0 {
                p += 10 as ::core::ffi::c_int;
            }
        }
        42 => {
            z = 20 as ::core::ffi::c_int;
        }
        43 => {
            z = 12 as ::core::ffi::c_int * 15 as ::core::ffi::c_int;
            p += 100 as ::core::ffi::c_int;
        }
        44 => {
            z = 12 as ::core::ffi::c_int * 15 as ::core::ffi::c_int;
            p += 100 as ::core::ffi::c_int;
        }
        45 => {
            if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        46 => {
            if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        47 => {
            if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        48 => {
            if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        49 => {
            if !(crate::bridge::st().borg.trait_[(BI_IPOIS as ::core::ffi::c_int) as usize] != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        50 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                p += 200 as ::core::ffi::c_int;
            }
        }
        51 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        52 => {
            z = ouch;
        }
        53 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        54 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        55 => {
            z = ouch;
        }
        56 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        57 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        58 => {
            p += 10 as ::core::ffi::c_int;
        }
        59 => {
            p += 10 as ::core::ffi::c_int;
        }
        60 => {
            p += 10 as ::core::ffi::c_int;
        }
        61 => {
            p += 5 as ::core::ffi::c_int;
        }
        62 => {
            p += 20 as ::core::ffi::c_int;
        }
        63 => {
            p += 10 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize];
        }
        64 => {
            p += 10 as ::core::ffi::c_int;
        }
        67 => {
            p += 10 as ::core::ffi::c_int;
        }
        68 => {
            p += 20 as ::core::ffi::c_int;
        }
        69 => {
            p += 20 as ::core::ffi::c_int;
        }
        70 => {
            p += 10 as ::core::ffi::c_int;
        }
        71 => {
            p += 25 as ::core::ffi::c_int;
        }
        3 | 27 | 65 | 66 | 72 => {}
        73 => {
            p += 50 as ::core::ffi::c_int;
        }
        74 => {
            if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                > 10 as ::core::ffi::c_int
            {
                p += 500 as ::core::ffi::c_int;
            } else {
                p += 30 as ::core::ffi::c_int;
            }
        }
        75 => {
            p += 200 as ::core::ffi::c_int;
        }
        76 => {
            p += 55 as ::core::ffi::c_int;
        }
        77 => {
            p += 95 as ::core::ffi::c_int;
        }
        78 => {
            p += 55 as ::core::ffi::c_int;
        }
        79 => {
            p += 30 as ::core::ffi::c_int;
        }
        80 => {
            p += 15 as ::core::ffi::c_int;
        }
        81 => {
            p += 25 as ::core::ffi::c_int;
        }
        82 => {
            p += 45 as ::core::ffi::c_int;
        }
        83 => {
            p += 70 as ::core::ffi::c_int;
        }
        84 => {
            p += 80 as ::core::ffi::c_int;
        }
        85 => {
            p += 80 as ::core::ffi::c_int;
        }
        86 => {
            p += 80 as ::core::ffi::c_int;
        }
        87 => {
            p += 80 as ::core::ffi::c_int;
        }
        88 => {
            p += 95 as ::core::ffi::c_int;
        }
        89 => {
            p += 95 as ::core::ffi::c_int;
        }
        90 => {
            p += 95 as ::core::ffi::c_int;
        }
        91 => {
            p += 50 as ::core::ffi::c_int;
        }
        _ => {
            p += 1000 as ::core::ffi::c_int;
            borg_note(b"# Mystery attack.  BUG.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    if ouch
        >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        ouch = ouch * 2 as ::core::ffi::c_int;
    }
    return p
        + z
        + crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            * 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_handle_self(mut str: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    q_x = o_w_x / crate::borg_cave_util::borg_panel_wid();
    q_y = o_w_y / crate::borg_cave_util::borg_panel_hgt();
    if borg_failure {
        borg_note(b"# Something failed\0" as *const u8 as *const ::core::ffi::c_char);
    } else if prefix(str, b"lite\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Called lite at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            old_c.y,
            old_c.x,
        ));
        if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 {
            y = crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int;
            while y <= crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int {
                x = crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int;
                while x <= crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int {
                    let mut ag: *mut borg_grid =
                        crate::bridge::grid(x, y) as *mut borg_grid;
                    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                    (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    x += 1;
                }
                y += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < crate::bridge::st().light.light_n as ::core::ffi::c_int {
            x = crate::bridge::st().light.light_x[i as usize] as ::core::ffi::c_int;
            y = crate::bridge::st().light.light_y[i as usize] as ::core::ffi::c_int;
            let mut ag_0: *mut borg_grid = crate::bridge::grid(x, y) as *mut borg_grid;
            (*ag_0).info = ((*ag_0).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
            (*ag_0).info = ((*ag_0).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
            i += 1;
        }
    } else if prefix(str, b"wall\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected walls (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"trap\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"door\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected doors (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"both\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps and doors (%d,%d to %d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"TDE\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps, doors & evil (%d,%d to %d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"evil\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected evil (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"obj\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected objects (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    }
}
unsafe extern "C" fn intcomp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return *(a as *mut ::core::ffi::c_int) - *(b as *mut ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_update() {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut max_x: ::core::ffi::c_int = 0;
    let mut max_y: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut u_i: ::core::ffi::c_uint = 0;
    let mut hit_dist: ::core::ffi::c_int = 0;
    let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut what: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut reset: bool = false_0 != 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut floor_grid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut floor_glyphed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut monster_in_vault: bool = false_0 != 0;
    let mut created_traps: bool = false_0 != 0;
    crate::bridge::st().core.w_y = (*Term).offset_y;
    crate::bridge::st().core.w_x = (*Term).offset_x;
    crate::bridge::st().borg.panels.x = 1 as ::core::ffi::c_int;
    if (*cave).width > crate::borg_cave_util::borg_panel_wid() {
        crate::bridge::st().borg.panels.x = ((*cave).width - crate::borg_cave_util::borg_panel_wid()) * 2 as ::core::ffi::c_int
            / crate::borg_cave_util::borg_panel_wid()
            + 1 as ::core::ffi::c_int;
    }
    crate::bridge::st().borg.panels.y = 1 as ::core::ffi::c_int;
    if (*cave).height > crate::borg_cave_util::borg_panel_hgt() {
        crate::bridge::st().borg.panels.y = ((*cave).height - crate::borg_cave_util::borg_panel_hgt()) * 2 as ::core::ffi::c_int
            / crate::borg_cave_util::borg_panel_hgt()
            + 1 as ::core::ffi::c_int;
    }
    if o_w_x > (*cave).width || o_w_y > (*cave).height {
        o_w_y = 0 as ::core::ffi::c_int;
        o_w_x = o_w_y;
    }
    borg_alloc_detection();
    let mut current_block_16: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            (*kill).seen = false_0 != 0;
            (*kill).used = false_0 != 0;
            if crate::bridge::st().borg.time.now - (*kill).when < 2000 as borg_time {
                if !(crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                    != 0
                    && crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                        != 0)
                {
                    current_block_16 = 3512920355445576850;
                } else {
                    current_block_16 = 13797916685926291137;
                }
            } else {
                current_block_16 = 13797916685926291137;
            }
            match current_block_16 {
                3512920355445576850 => {}
                _ => {
                    borg_note(format(
                        b"# Expiring a monster '%s' (%d) at (%d,%d)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                        (*kill).r_idx as ::core::ffi::c_int,
                        (*kill).pos.y,
                        (*kill).pos.x,
                    ));
                    borg_delete_kill(i);
                }
            }
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            (*take).seen = false_0 != 0;
            if (*take).y as ::core::ffi::c_int == crate::bridge::st().borg.c.y
                && (*take).x as ::core::ffi::c_int == crate::bridge::st().borg.c.x
                || (*take).y as ::core::ffi::c_int == old_c.y
                    && (*take).x as ::core::ffi::c_int == old_c.x
                || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                    != 0
            {
                borg_delete_take(i);
            } else if !(crate::bridge::st().borg.time.now - (*take).when < 2000 as borg_time) {
                borg_note(format(
                    b"# Expiring an object '%s' (%lu) at (%d,%d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*k_info.offset((*take).kidx as isize)).kidx as ::core::ffi::c_ulong,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_delete_take(i);
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        msg = borg_msg_buf.offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
        borg_note(format(
            b"# %s (+)\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"SELF:\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_handle_self(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            } else if prefix(
                msg,
                b"FEELING_DANGER:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_feeling_danger = atoi(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            } else if prefix(
                msg,
                b"FEELING_STUFF:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_feeling_stuff = atoi(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(msg, b"MISS:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(msg, b"AFRAID:\0" as *const u8 as *const ::core::ffi::c_char) {
                if (*crate::bridge::grid(crate::bridge::st().borg.goal.g.x, crate::bridge::st().borg.goal.g.y)).kill
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                } else {
                    borg_create_kill(what, crate::bridge::st().borg.goal.g);
                }
            } else if prefix(msg, b"KILL:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"BLINK:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 3 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 3 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE_CONFUSED:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            }
        }
        i += 1;
    }
    hit_dist = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            created_traps = false_0 != 0;
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                let mut spell: ::core::ffi::c_int =
                    atoi(msg.offset(6 as ::core::ffi::c_int as isize));
                if spell == RSF_TELE_AWAY as ::core::ffi::c_int
                    || spell == RSF_TELE_TO as ::core::ffi::c_int
                {
                    hit_dist = 100 as ::core::ffi::c_int;
                }
                break;
            } else if prefix(msg, b"QUAKE:\0" as *const u8 as *const ::core::ffi::c_char) {
                hit_dist = 3 as ::core::ffi::c_int;
                break;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"MISS:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"KILL:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"BLINK:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.goal.g, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target == -(1 as ::core::ffi::c_int) {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_sleep_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_AWAKE:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
                let mut spell_0: ::core::ffi::c_int =
                    atoi(msg.offset(6 as ::core::ffi::c_int as isize));
                if spell_0 == RSF_TRAPS as ::core::ffi::c_int {
                    created_traps = true_0 != 0;
                }
            }
            if created_traps {
                *(*borg_detect_trap
                    .offset((o_w_y / crate::borg_cave_util::borg_panel_hgt() + 0 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / crate::borg_cave_util::borg_panel_wid() + 0 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / crate::borg_cave_util::borg_panel_hgt() + 0 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / crate::borg_cave_util::borg_panel_wid() + 1 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / crate::borg_cave_util::borg_panel_hgt() + 1 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / crate::borg_cave_util::borg_panel_wid() + 0 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / crate::borg_cave_util::borg_panel_hgt() + 1 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / crate::borg_cave_util::borg_panel_wid() + 1 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                borg_note(
                    b"# Logging need for use of Detect Traps.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ii = 0 as ::core::ffi::c_int;
                while ii < 8 as ::core::ffi::c_int {
                    x = crate::bridge::st().borg.c.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                    y = crate::bridge::st().borg.c.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                        if crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
                            (*ag).trap = true_0 != 0;
                            borg_note(format(
                                b"# Assuming a Traps at (%d,%d).\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                y,
                                x,
                            ));
                        }
                    }
                    ii += 1;
                }
            }
        }
        i += 1;
    }
    if successful_target < 0 as ::core::ffi::c_int {
        if successful_target > -(10 as ::core::ffi::c_int) {
            successful_target -= 10 as ::core::ffi::c_int;
            if borg_target_unknown_wall(crate::bridge::st().borg.goal.g.y, crate::bridge::st().borg.goal.g.x) {
                successful_target = 2 as ::core::ffi::c_int;
            }
        }
    }
    if crate::bridge::st().borg.status.old_depth as ::core::ffi::c_int
        != crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
    {
        crate::bridge::st().borg.time.level = crate::bridge::st().borg.time.now;
        crate::bridge::st().borg.time.morgoth = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
        if crate::bridge::st().borg.status.old_depth == 0 {
            crate::bridge::st().borg.time.town = crate::bridge::st().borg.time.now;
        }
        crate::bridge::st().borg.antibounce_count = 1 as int16_t;
        crate::bridge::st().borg.status.vault = false_0 != 0;
        crate::bridge::st().borg.mon.unique = 0 as ::core::ffi::c_uint;
        crate::bridge::st().borg.mon.scary = false_0 != 0;
        crate::bridge::st().borg.near.breeder = false_0 != 0;
        crate::bridge::st().borg.need_see_invis = 0 as ::core::ffi::c_int as borg_time;
        successful_target = 0 as ::core::ffi::c_int;
        crate::bridge::st().borg.avoidance = crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] as int16_t;
        borg_danger_wipe = true_0 != 0;
        sold_item_num = 0 as ::core::ffi::c_int;
        sold_item_nxt = 1 as ::core::ffi::c_int;
        bought_item_nxt = 1 as ::core::ffi::c_int;
        bought_item_num = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            sold_item_tval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_sval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_pval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_store[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_tval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_sval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_pval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_store[i as usize] = 0 as ::core::ffi::c_int;
            i += 1;
        }
        borg_do_update_view = true_0 != 0;
        borg_do_update_lite = true_0 != 0;
        borg_do_spell = true_0 != 0;
        borg_do_crush_junk = true_0 != 0;
        crate::bridge::st().borg.time.call_light = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.wizard_light = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.detect_traps = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.detect_doors = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.detect_walls = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.detect_evil = 0 as ::core::ffi::c_int as borg_time;
        crate::bridge::st().borg.time.detect_obj = 0 as ::core::ffi::c_int as borg_time;
        y = 0 as ::core::ffi::c_int;
        while y < crate::bridge::st().borg.panels.y {
            x = 0 as ::core::ffi::c_int;
            while x < crate::bridge::st().borg.panels.x {
                *(*borg_detect_wall.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_trap.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_door.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_evil.offset(y as isize)).offset(x as isize) = false_0 != 0;
                x += 1;
            }
            y += 1;
        }
        max_x = AUTO_MAX_X / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        max_y = AUTO_MAX_Y / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        y = 0 as ::core::ffi::c_int;
        while y < max_y {
            x = 0 as ::core::ffi::c_int;
            while x < max_x {
                borg_fear_region[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        y = 0 as ::core::ffi::c_int;
        while y < AUTO_MAX_Y {
            x = 0 as ::core::ffi::c_int;
            while x < AUTO_MAX_X {
                borg_fear_monsters[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
        crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
        crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
        crate::bridge::st().borg.stair_more = false_0 != 0;
        crate::bridge::st().borg.stair_less = crate::bridge::st().borg.stair_more;
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
        {
            crate::bridge::st().borg.goal.rising = false_0 != 0;
        }
        crate::bridge::st().borg.goal.leaving = false_0 != 0;
        crate::bridge::st().borg.goal.fleeing = false_0 != 0;
        crate::bridge::st().borg.goal.fleeing_lunal = false_0 != 0;
        crate::bridge::st().borg.goal.fleeing_munchkin = false_0 != 0;
        crate::bridge::st().borg.goal.ignoring = false_0 != 0;
        borg_feeling_stuff = 0 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
        {
            borg_clear_best();
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
        {
            crate::bridge::st().borg.goal.fleeing_to_town = false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 2 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.goal.fleeing_to_town = false_0 != 0;
        }
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        track_glyph.num = 0 as int16_t;
        track_step.num = 0 as int16_t;
        track_door.num = 0 as int16_t;
        track_closed.num = 0 as int16_t;
        track_vein.num = 0 as int16_t;
        track_worn_num = 0 as int16_t;
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        memset(
            borg_takes as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<borg_take>() as size_t),
        );
        borg_kills_cnt = 0 as int16_t;
        borg_kills_nxt = 1 as int16_t;
        crate::bridge::st().borg.near.morgoth = false_0 != 0;
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 100 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
            || crate::bridge::st().borg.mon.unique == crate::bridge::st().borg.id.morgoth
        {
            crate::bridge::st().borg.near.morgoth = true_0 != 0;
            borg_needs_new_sea = true_0 != 0;
        }
        i = 0 as ::core::ffi::c_int;
        while i < borg_nasties_num as ::core::ffi::c_int {
            borg_nasties_count[i as usize] = 0 as uint8_t;
            if borg_nasties[i as usize] as ::core::ffi::c_int == 'Z' as i32 {
                borg_nasties_count[i as usize] = 25 as uint8_t;
            }
            i += 1;
        }
        memset(
            borg_kills as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<borg_kill>() as size_t),
        );
        memset(
            borg_race_count as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
        crate::bridge::st().prepared.numb_live_unique = 0 as ::core::ffi::c_int;
        crate::bridge::st().prepared.first_living_unique = 0 as ::core::ffi::c_uint;
        crate::bridge::st().prepared.depth_hunted_unique = 0 as ::core::ffi::c_int;
        let mut unique_depths: [::core::ffi::c_int; 4] = [
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ];
        u_i = 1 as ::core::ffi::c_uint;
        while u_i
            < ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            let mut r_ptr: *mut monster_race = r_info.offset(u_i as isize) as *mut monster_race;
            if !(*r_ptr).name.is_null() {
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    if (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *borg_race_death.offset(u_i as isize) = 1 as int16_t;
                    }
                    if !(*borg_race_death.offset(u_i as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int)
                    {
                        if !(flag_has(
                            &raw mut (*r_ptr).flags as *mut bitflag,
                            (RF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RF_QUESTOR as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                            || flag_has(
                                &raw mut (*r_ptr).flags as *mut bitflag,
                                (RF_MAX as ::core::ffi::c_int as size_t)
                                    .wrapping_add(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    )
                                    .wrapping_sub(1 as size_t)
                                    .wrapping_div(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    ),
                                RF_SEASONAL as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int
                                != 0)
                        {
                            if !((*r_ptr).rarity == 0) {
                                if crate::bridge::st().prepared.numb_live_unique < 4 as ::core::ffi::c_int {
                                    unique_depths[crate::bridge::st().prepared.numb_live_unique as usize] = (*r_ptr).level;
                                    crate::bridge::st().prepared.numb_live_unique += 1;
                                    if crate::bridge::st().prepared.numb_live_unique == 1 as ::core::ffi::c_int {
                                        crate::bridge::st().prepared.first_living_unique = u_i;
                                    }
                                    if crate::bridge::st().prepared.numb_live_unique == 3 as ::core::ffi::c_int {
                                        qsort(
                                            &raw mut unique_depths as *mut ::core::ffi::c_int
                                                as *mut ::core::ffi::c_void,
                                            3 as size_t,
                                            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                            Some(
                                                intcomp
                                                    as unsafe extern "C" fn(
                                                        *const ::core::ffi::c_void,
                                                        *const ::core::ffi::c_void,
                                                    )
                                                        -> ::core::ffi::c_int,
                                            ),
                                        );
                                    }
                                } else {
                                    crate::bridge::st().prepared.numb_live_unique += 1;
                                    if (*r_ptr).level
                                        < unique_depths[2 as ::core::ffi::c_int as usize]
                                    {
                                        unique_depths[3 as ::core::ffi::c_int as usize] =
                                            (*r_ptr).level;
                                        qsort(
                                            &raw mut unique_depths as *mut ::core::ffi::c_int
                                                as *mut ::core::ffi::c_void,
                                            4 as size_t,
                                            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                            Some(
                                                intcomp
                                                    as unsafe extern "C" fn(
                                                        *const ::core::ffi::c_void,
                                                        *const ::core::ffi::c_void,
                                                    )
                                                        -> ::core::ffi::c_int,
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            u_i = u_i.wrapping_add(1);
        }
        if crate::bridge::st().prepared.numb_live_unique >= 3 as ::core::ffi::c_int {
            crate::bridge::st().prepared.depth_hunted_unique = unique_depths[2 as ::core::ffi::c_int as usize];
        } else if crate::bridge::st().prepared.numb_live_unique != 0 as ::core::ffi::c_int {
            crate::bridge::st().prepared.depth_hunted_unique =
                unique_depths[(crate::bridge::st().prepared.numb_live_unique - 1 as ::core::ffi::c_int) as usize];
        } else {
            crate::bridge::st().prepared.depth_hunted_unique = 127 as ::core::ffi::c_int;
        }
        borg_forget_map();
        reset = true_0 != 0;
        if crate::bridge::st().core.cfg[(BORG_AUTOSAVE as ::core::ffi::c_int) as usize] != 0
            && !crate::bridge::st().borg.lunal_mode
            && !crate::bridge::st().borg.munchkin_mode
        {
            crate::bridge::st().borg.status.save = true_0 != 0;
        }
        crate::bridge::st().borg.status.old_depth = crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            as int16_t;
        crate::bridge::st().borg.times_twitch = 0 as int16_t;
        crate::bridge::st().borg.escapes = 0 as int16_t;
    } else {
        if crate::bridge::st().borg.resistance as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            crate::bridge::st().borg.resistance = (crate::bridge::st().borg.resistance as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        if crate::bridge::st().borg.no_rest_prep as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            crate::bridge::st().borg.no_rest_prep = (crate::bridge::st().borg.no_rest_prep as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        if crate::bridge::st().borg.goal.recalling >= 1 as ::core::ffi::c_int {
            crate::bridge::st().borg.goal.recalling -= borg_game_ratio as ::core::ffi::c_int;
            if crate::bridge::st().borg.goal.recalling <= 0 as ::core::ffi::c_int {
                crate::bridge::st().borg.goal.recalling = 1 as ::core::ffi::c_int;
            }
        }
        if crate::bridge::st().borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            crate::bridge::st().borg.see_inv = (crate::bridge::st().borg.see_inv as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        crate::bridge::st().borg.near.morgoth = false_0 != 0;
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 100 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
            || crate::bridge::st().borg.mon.unique == crate::bridge::st().borg.id.morgoth
        {
            crate::bridge::st().borg.near.morgoth = true_0 != 0;
        }
        if crate::bridge::st().borg.near.morgoth as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level >= 500 as borg_time
        {
            if crate::bridge::st().borg.mon.unique != crate::bridge::st().borg.id.morgoth {
                crate::bridge::st().borg.near.morgoth = false_0 != 0;
            }
            if crate::bridge::st().borg.mon.unique == crate::bridge::st().borg.id.morgoth
                && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth > 500 as borg_time
            {
                borg_note(format(
                    b"# Morgoth has not been seen in %ld turns.  Going to hunt him.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth) as ::core::ffi::c_long,
                ));
                crate::bridge::st().borg.near.morgoth = false_0 != 0;
            }
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth > 2500 as borg_time {
                borg_note(format(
                    b"# Morgoth has not been seen in %ld turns.  No show.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth) as ::core::ffi::c_long,
                ));
                crate::bridge::st().borg.mon.unique = 0 as ::core::ffi::c_uint;
            }
        }
        if crate::bridge::st().borg.resistance as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            && crate::bridge::st().borg.temp.res_fire as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.temp.res_elec as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.temp.res_cold as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.temp.res_acid as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.temp.res_pois as ::core::ffi::c_int != 0
        {
            crate::bridge::st().borg.resistance = 20000 as int16_t;
        }
        if crate::bridge::st().borg.resistance as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.temp.res_fire as ::core::ffi::c_int
                + crate::bridge::st().borg.temp.res_elec as ::core::ffi::c_int
                + crate::bridge::st().borg.temp.res_cold as ::core::ffi::c_int
                + crate::bridge::st().borg.temp.res_acid as ::core::ffi::c_int
                + crate::bridge::st().borg.temp.res_pois as ::core::ffi::c_int
                != 5 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.resistance = 0 as int16_t;
        }
        if crate::bridge::st().borg.time.now % 10 as borg_time == 0 {
            max_x = AUTO_MAX_X / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            max_y = AUTO_MAX_Y / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            y = 0 as ::core::ffi::c_int;
            while y < max_y {
                x = 0 as ::core::ffi::c_int;
                while x < max_x {
                    if borg_fear_region[y as usize][x as usize] != 0 {
                        borg_fear_region[y as usize][x as usize] =
                            borg_fear_region[y as usize][x as usize].wrapping_sub(1);
                    }
                    x += 1;
                }
                y += 1;
            }
        }
        y = 0 as ::core::ffi::c_int;
        while y < AUTO_MAX_Y {
            x = 0 as ::core::ffi::c_int;
            while x < AUTO_MAX_X {
                borg_fear_monsters[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        if o_w_x != crate::bridge::st().core.w_x || o_w_y != crate::bridge::st().core.w_y {
            dy = 0 as ::core::ffi::c_int;
            while dy
                < ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int
            {
                dx = 0 as ::core::ffi::c_int;
                while dx
                    < ((*Term).wid
                        - col_map[(*Term).sidebar_mode as usize]
                        - 1 as ::core::ffi::c_int)
                        / tile_width as ::core::ffi::c_int
                {
                    x = o_w_x + dx;
                    y = o_w_y + dy;
                    if !(y >= AUTO_MAX_Y || x >= AUTO_MAX_X) {
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_OKAY) as uint16_t;
                    }
                    dx += 1;
                }
                dy += 1;
            }
            crate::bridge::st().borg.antibounce_count = 1 as int16_t;
        }
    }
    borg_wank_num = 0 as ::core::ffi::c_int;
    borg_update_map();
    *track_step.x.offset(track_step.num as isize) = crate::bridge::st().borg.c.x;
    *track_step.y.offset(track_step.num as isize) = crate::bridge::st().borg.c.y;
    track_step.num += 1;
    if track_step.num as ::core::ffi::c_int >= 75 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= 75 as ::core::ffi::c_int {
            *track_step.x.offset(i as isize) =
                *track_step.x.offset((i + 1 as ::core::ffi::c_int) as isize);
            *track_step.y.offset(i as isize) =
                *track_step.y.offset((i + 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        track_step.num = 75 as int16_t;
    }
    if reset {
        o_w_x = crate::bridge::st().core.w_x;
        o_w_y = crate::bridge::st().core.w_y;
        old_c = crate::bridge::st().borg.c;
        crate::bridge::st().borg.goal.g = crate::bridge::st().borg.c;
    }
    if old_c.x != crate::bridge::st().borg.c.x || old_c.y != crate::bridge::st().borg.c.y {
        borg_do_update_view = true_0 != 0;
        borg_do_update_lite = true_0 != 0;
        successful_target = 0 as ::core::ffi::c_int;
    }
    if borg_do_update_view {
        borg_update_view();
        borg_do_update_view = false_0 != 0;
    }
    if borg_do_update_lite {
        borg_update_light();
        borg_do_update_lite = false_0 != 0;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill_0 = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_0).r_idx == 0) {
            if !flag_has(
                &raw mut (*r_info.offset((*kill_0).r_idx as isize)).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_PASS_WALL as ::core::ffi::c_int,
            ) {
                (*crate::bridge::grid((*kill_0).pos.x, (*kill_0).pos.y)).feat =
                    FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
            }
        }
        i += 1;
    }
    crate::bridge::st().borg.morgoth_position = false_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.near.morgoth as ::core::ffi::c_int != 0
    {
        if crate::bridge::st().borg.c.y >= 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.c.y <= AUTO_MAX_Y - 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.c.x >= 50 as ::core::ffi::c_int
            && crate::bridge::st().borg.c.x <= AUTO_MAX_X - 50 as ::core::ffi::c_int
        {
            j = 0 as ::core::ffi::c_int;
            while j < 24 as ::core::ffi::c_int {
                y = crate::bridge::st().borg.c.y + borg_ddy_ddd[j as usize] as ::core::ffi::c_int;
                x = crate::bridge::st().borg.c.x + borg_ddx_ddd[j as usize] as ::core::ffi::c_int;
                ag = crate::bridge::grid(x, y) as *mut borg_grid;
                if (*ag).glyph {
                    floor_glyphed += 1;
                }
                j += 1;
            }
            if floor_glyphed == 24 as ::core::ffi::c_int {
                crate::bridge::st().borg.morgoth_position = true_0 != 0;
            }
        }
    }
    j = 0 as ::core::ffi::c_int;
    while j < 24 as ::core::ffi::c_int {
        y = crate::bridge::st().borg.c.y + borg_ddy_ddd[j as usize] as ::core::ffi::c_int;
        x = crate::bridge::st().borg.c.x + borg_ddx_ddd[j as usize] as ::core::ffi::c_int;
        if !square_in_bounds(cave, loc(x, y)) {
            floor_grid += 1;
        } else {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if j <= 7 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
            {
                floor_grid += 1;
            }
            if j >= 8 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                && borg_los(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) as ::core::ffi::c_int != 0
            {
                floor_grid += 1;
            }
        }
        j += 1;
    }
    if floor_grid == 1 as ::core::ffi::c_int {
        crate::bridge::st().borg.status.anti_summon = true_0 != 0;
    } else {
        crate::bridge::st().borg.status.anti_summon = false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        <= 5 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
        && track_closed.num as ::core::ffi::c_int != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_closed.num as ::core::ffi::c_int {
            x = *track_closed.x.offset(i as isize);
            y = *track_closed.y.offset(i as isize);
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
            {
                borg_note(format(
                    b"# Monster opened door at %d,%d.  That's scary.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    y,
                    x,
                ));
                crate::bridge::st().borg.mon.scary = true_0 != 0;
            }
            i += 1;
        }
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank).y as ::core::ffi::c_int,
                (*wank).x as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*wank).t_a,
                (*wank).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank).is_kill = false_0 != 0;
            if !(*wank).is_take && !(*wank).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_0: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_0).is_take as ::core::ffi::c_int != 0
            && observe_take_move(
                (*wank_0).y as ::core::ffi::c_int,
                (*wank_0).x as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*wank_0).t_a,
                (*wank_0).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_0).is_take = false_0 != 0;
            if !(*wank_0).is_take && !(*wank_0).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_1: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_1).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_1).y as ::core::ffi::c_int,
                (*wank_1).x as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                (*wank_1).t_a,
                (*wank_1).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_1).is_kill = false_0 != 0;
            if !(*wank_1).is_take && !(*wank_1).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_2: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_2).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_2).y as ::core::ffi::c_int,
                (*wank_2).x as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                (*wank_2).t_a,
                (*wank_2).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_2).is_kill = false_0 != 0;
            if !(*wank_2).is_take && !(*wank_2).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_3: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_3).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_3).y as ::core::ffi::c_int,
                (*wank_3).x as ::core::ffi::c_int,
                3 as ::core::ffi::c_int,
                (*wank_3).t_a,
                (*wank_3).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_3).is_kill = false_0 != 0;
            if !(*wank_3).is_take && !(*wank_3).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_4: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_4).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_4).y as ::core::ffi::c_int,
                (*wank_4).x as ::core::ffi::c_int,
                7 as ::core::ffi::c_int,
                (*wank_4).t_a,
                (*wank_4).t_c,
                true_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_4).is_kill = false_0 != 0;
            if !(*wank_4).is_take && !(*wank_4).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_5: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_5).is_take as ::core::ffi::c_int != 0
            && observe_take_diff(
                (*wank_5).y as ::core::ffi::c_int,
                (*wank_5).x as ::core::ffi::c_int,
                (*wank_5).t_a,
                (*wank_5).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_5).is_take = false_0 != 0;
            if !(*wank_5).is_take && !(*wank_5).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_6: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_6).is_kill as ::core::ffi::c_int != 0
            && observe_kill_diff(
                (*wank_6).y as ::core::ffi::c_int,
                (*wank_6).x as ::core::ffi::c_int,
                (*wank_6).t_a,
                (*wank_6).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_6).is_kill = false_0 != 0;
            if !(*wank_6).is_take && !(*wank_6).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_sleep_kill(k);
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_AWAKE:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, crate::bridge::st().borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_fear_regional(
                    what,
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    4 as ::core::ffi::c_int
                        * (crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            / 5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_fear_regional(
                    what,
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    2 as ::core::ffi::c_int
                        * (crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            / 5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_fear_regional(
                    what,
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    borg_fear_spell(atoi(msg.offset(6 as ::core::ffi::c_int as isize))),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        msg = borg_msg_buf.offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
        borg_note(format(
            b"# %s (%d)\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
            *borg_msg_use.offset(i as isize) as ::core::ffi::c_int,
        ));
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_1: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_1).r_idx == 0) {
            if !((*kill_1).when == crate::bridge::st().borg.time.now) {
                if !(*kill_1).seen {
                    if !(crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
                        != 0
                        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                            != 0)
                    {
                        borg_follow_kill(i);
                    }
                }
            }
        }
        i += 1;
    }
    crate::bridge::st().borg.mon.breeders = 0 as uint16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut p: ::core::ffi::c_int = 0;
        let mut kill_2: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut r_ptr_0: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        monster_in_vault = false_0 != 0;
        if !((*kill_2).r_idx == 0) {
            r_ptr_0 = r_info.offset((*kill_2).r_idx as isize) as *mut monster_race;
            if flag_has(
                &raw mut (*r_ptr_0).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_MULTIPLY as ::core::ffi::c_int,
            ) {
                crate::bridge::st().borg.mon.breeders = crate::bridge::st().borg.mon.breeders.wrapping_add(1);
            }
            if !flag_has(
                &raw mut (*r_ptr_0).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NEVER_MOVE as ::core::ffi::c_int,
            ) {
                if !(distance((*kill_2).pos, crate::bridge::st().borg.c) >= 20 as ::core::ffi::c_int) {
                    if crate::bridge::st().borg.status.vault {
                        ii = 0 as ::core::ffi::c_int;
                        while ii < 8 as ::core::ffi::c_int {
                            x = (*kill_2).pos.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                            y = (*kill_2).pos.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(x, y)) {
                                ag = crate::bridge::grid(x, y)
                                    as *mut borg_grid;
                                if !((*ag).feat as ::core::ffi::c_int
                                    == FEAT_NONE as ::core::ffi::c_int)
                                {
                                    if (*ag).feat as ::core::ffi::c_int
                                        == FEAT_PERM as ::core::ffi::c_int
                                    {
                                        monster_in_vault = true_0 != 0;
                                    }
                                }
                            }
                            ii += 1;
                        }
                    }
                    if !(monster_in_vault as ::core::ffi::c_int == true_0) {
                        p = borg_danger(
                            (*kill_2).pos.y,
                            (*kill_2).pos.x,
                            1 as ::core::ffi::c_int,
                            false_0 != 0,
                            false_0 != 0,
                        ) / 10 as ::core::ffi::c_int;
                        borg_fear_grid((*kill_2).pos.y, (*kill_2).pos.x, p);
                    }
                }
            }
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take_0: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take_0).kidx == 0) {
            if !(crate::bridge::st().borg.time.now - (*take_0).when <= 2 as borg_time) {
                if !(crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
                    != 0
                    || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                        != 0)
                {
                    borg_follow_take(i);
                }
            }
        }
        i += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
            != 0
    {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    crate::bridge::st().borg.oldchp = crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] as int16_t;
    borg_failure = false_0 != 0;
    borg_msg_len = 0 as int16_t;
    borg_msg_num = 0 as int16_t;
    old_c = crate::bridge::st().borg.c;
    o_w_x = crate::bridge::st().core.w_x;
    o_w_y = crate::bridge::st().core.w_y;
    crate::bridge::st().borg.goal.g = crate::bridge::st().borg.c;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_detection() {
    let mut i: ::core::ffi::c_int = 0;
    let mut array_size_y: ::core::ffi::c_int = old_panels.y + 1 as ::core::ffi::c_int;
    if borg_detect_wall.is_null()
        && borg_detect_trap.is_null()
        && borg_detect_door.is_null()
        && borg_detect_evil.is_null()
        && borg_detect_obj.is_null()
    {
        return;
    }
    if !borg_detect_wall.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_wall.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_wall as *mut ::core::ffi::c_void);
        borg_detect_wall = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_trap.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_trap.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_trap as *mut ::core::ffi::c_void);
        borg_detect_trap = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_door.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_door.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_door as *mut ::core::ffi::c_void);
        borg_detect_door = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_evil.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_evil.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_evil as *mut ::core::ffi::c_void);
        borg_detect_evil = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_obj.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_obj.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_obj as *mut ::core::ffi::c_void);
        borg_detect_obj = ::core::ptr::null_mut::<*mut bool>();
    }
    old_panels.x = -(1 as ::core::ffi::c_int);
    old_panels.y = -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_alloc_detection() {
    let mut i: ::core::ffi::c_int = 0;
    let mut array_size_y: ::core::ffi::c_int = crate::bridge::st().borg.panels.y + 1 as ::core::ffi::c_int;
    let mut array_size_x: ::core::ffi::c_int = crate::bridge::st().borg.panels.x + 1 as ::core::ffi::c_int;
    if old_panels.x == crate::bridge::st().borg.panels.x && old_panels.y == crate::bridge::st().borg.panels.y {
        return;
    }
    borg_free_detection();
    borg_detect_wall = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh1 = *borg_detect_wall.offset(i as isize);
        *fresh1 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_trap = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh2 = *borg_detect_trap.offset(i as isize);
        *fresh2 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_door = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh3 = *borg_detect_door.offset(i as isize);
        *fresh3 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_evil = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh4 = *borg_detect_evil.offset(i as isize);
        *fresh4 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_obj = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh5 = *borg_detect_obj.offset(i as isize);
        *fresh5 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    old_panels = crate::bridge::st().borg.panels;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_update() {
    borg_free_update();
    borg_wanks = mem_zalloc(
        (AUTO_VIEW_MAX as size_t).wrapping_mul(::core::mem::size_of::<borg_wank>() as size_t),
    ) as *mut borg_wank;
    borg_forget_map();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_update() {
    if !borg_wanks.is_null() {
        mem_free(borg_wanks as *mut ::core::ffi::c_void);
        borg_wanks = ::core::ptr::null_mut::<borg_wank>();
    }
}
