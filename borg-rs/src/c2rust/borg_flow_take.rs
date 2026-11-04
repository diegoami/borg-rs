pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

pub use crate::headers::borg_flow_h::{
    borg_flow_clear, borg_flow_commit, borg_flow_enqueue_grid, borg_flow_old, borg_flow_spread,
    borg_temp_n, borg_temp_x, borg_temp_y, borg_track,
};
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills};
use crate::headers::borg_flow_misc_h::{borg_flow_far_from_stairs, borg_get_leash};
use crate::headers::borg_flow_stairs_h::{borg_flow_cost_stair, track_less};
pub use crate::headers::borg_flow_take_h::borg_take;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_val_h::{sv_potion_detect_invis, sv_potion_inc_str};
use crate::headers::borg_item_wear_h::borg_is_ammo;
use crate::headers::borg_light_h::borg_check_light;
use crate::headers::borg_projection_h::borg_distance;
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
pub use crate::headers::cave_h::{
    cave, chunk, connector, distance, heatmap, monster_group, square, square_object, trap,
    C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_kind_flags_h::{
    KF_EASY_KNOW, KF_GOOD, KF_INSTA_ART, KF_MAX, KF_NONE, KF_QUEST_ART, KF_RAND_BASE_RES,
    KF_RAND_HI_RES, KF_RAND_POWER, KF_RAND_RES_POWER, KF_RAND_SUSTAIN, KF_SHOOTS_ARROWS,
    KF_SHOOTS_BOLTS, KF_SHOOTS_SHOTS, KF_SHOW_DICE, KF_SHOW_MULT,
};
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
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
};
use crate::headers::obj_knowledge_h::object_fully_known;
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, C2RustUnnamedH_36, TV_MAX};
use crate::headers::obj_tval_h::tval_is_book_k;
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item, C2RustUnnamedH_35, OBJ_NOTICE_ASSESSED, OBJ_NOTICE_IGNORE,
    OBJ_NOTICE_IMAGINED, OBJ_NOTICE_WORN,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memset;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::{quark_str, quark_t};
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, prefix};
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_takes_cnt: int16_t = 0;
#[no_mangle]
pub static mut borg_takes_nxt: int16_t = 0;
#[no_mangle]
pub static mut borg_takes: *mut borg_take = ::core::ptr::null::<borg_take>() as *mut borg_take;
#[no_mangle]
pub unsafe extern "C" fn borg_get_top_object(mut c: *mut chunk, mut grid: loc) -> *mut object {
    let mut o_ptr: *mut object = ::core::ptr::null_mut::<object>();
    o_ptr = square_object(c, grid);
    while !o_ptr.is_null() {
        if !(*o_ptr).known.is_null()
            && (*(*o_ptr).known).notice as ::core::ffi::c_int
                & OBJ_NOTICE_IGNORE as ::core::ffi::c_int
                != 0
        {
            o_ptr = (*o_ptr).next;
        } else {
            if !((*(*o_ptr).kind).ignore != 0) {
                break;
            }
            o_ptr = (*o_ptr).next;
        }
    }
    return o_ptr;
}
unsafe extern "C" fn borg_guess_kind(
    mut a: uint8_t,
    mut c: wchar_t,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> *mut object_kind {
    let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x, y));
    if o_ptr.is_null() {
        return ::core::ptr::null_mut::<object_kind>();
    }
    return (*o_ptr).kind;
}
#[no_mangle]
pub unsafe extern "C" fn borg_delete_take(mut i: ::core::ffi::c_int) {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
    if (*take).kidx == 0 {
        return;
    }
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        if (*take).kidx >= (*z_info).k_max as uint32_t {
            borg_note(format(
                b"# Forgetting an object at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                (*take).y as ::core::ffi::c_int,
                (*take).x as ::core::ffi::c_int,
            ));
        } else {
            borg_note(format(
                b"# Forgetting an object '%s' at (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*k_info.offset((*take).kidx as isize)).name,
                (*take).y as ::core::ffi::c_int,
                (*take).x as ::core::ffi::c_int,
            ));
        }
    }
    ag = crate::bridge::grid((*take).x, (*take).y) as *mut borg_grid;
    (*ag).take = 0 as uint8_t;
    memset(
        take as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_take>() as size_t,
    );
    borg_takes_cnt -= 1;
    if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
}
unsafe extern "C" fn borg_follow_take_aux(mut i: ::core::ffi::c_int) -> bool {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    ag = crate::bridge::grid((*borg_takes.offset(i as isize)).x, (*borg_takes.offset(i as isize)).y) as *mut borg_grid;
    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_follow_take(mut i: ::core::ffi::c_int) {
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
    let mut ag: *mut borg_grid = crate::bridge::grid((*take).x, (*take).y) as *mut borg_grid;
    let mut kill: *mut borg_kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    let mut old_kind: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    if (*take).kidx == 0 {
        return;
    }
    ox = (*take).x as ::core::ffi::c_int;
    oy = (*take).y as ::core::ffi::c_int;
    old_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
    if (*take).y as ::core::ffi::c_int == crate::bridge::st().borg.c.y && (*take).x as ::core::ffi::c_int == crate::bridge::st().borg.c.x {
        borg_delete_take(i);
    }
    if !borg_follow_take_aux(i) {
        return;
    }
    if (*ag).kill as ::core::ffi::c_int != 0
        && !flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_TAKE_ITEM as ::core::ffi::c_int,
        )
        && !flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_KILL_ITEM as ::core::ffi::c_int,
        )
    {
        return;
    }
    borg_note(format(
        b"# There was an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        (*old_kind).name,
        ox,
        oy,
    ));
    borg_delete_take(i);
}
unsafe extern "C" fn borg_new_take(
    mut kind: *mut object_kind,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    let mut ag: *mut borg_grid = crate::bridge::grid(x, y) as *mut borg_grid;
    let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x, y));
    i = 1 as ::core::ffi::c_int;
    while n < 0 as ::core::ffi::c_int && i < borg_takes_nxt as ::core::ffi::c_int {
        if (*borg_takes.offset(i as isize)).kidx == 0 {
            n = i;
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int
        && (borg_takes_nxt as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
    {
        let fresh0 = borg_takes_nxt;
        borg_takes_nxt = borg_takes_nxt + 1;
        n = fresh0 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        borg_note(b"# Too many objects\0" as *const u8 as *const ::core::ffi::c_char);
        n = (Rand_div((borg_takes_nxt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t)
            as int32_t
            + 1 as int32_t) as ::core::ffi::c_int;
        borg_delete_take(n);
    }
    borg_takes_cnt += 1;
    take = borg_takes.offset(n as isize) as *mut borg_take;
    (*take).kidx = (*kind).kidx;
    (*take).tval = (*kind).tval;
    (*take).x = x as uint8_t;
    (*take).y = y as uint8_t;
    (*ag).take = n as uint8_t;
    (*take).when = crate::bridge::st().borg.time.now;
    (*take).orbed = false_0 != 0;
    if (*kind).aware {
        (*take).value = (*kind).cost;
        if (*take).tval == TV_GOLD as ::core::ffi::c_int {
            (*take).value = 30 as ::core::ffi::c_int;
        }
    } else {
        (*take).value = 1 as ::core::ffi::c_int;
    }
    if object_fully_known(o_ptr) as ::core::ffi::c_int != 0
        && (((*o_ptr).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || ((*o_ptr).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || ((*o_ptr).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
    {
        (*take).value = -(10 as ::core::ffi::c_int);
    }
    if (*o_ptr).note != 0
        && prefix(
            quark_str((*o_ptr).note),
            b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        (*take).value = -(10 as ::core::ffi::c_int);
    }
    borg_note(format(
        b"# Creating an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        (*k_info.offset((*take).kidx as isize)).name,
        (*take).x as ::core::ffi::c_int,
        (*take).y as ::core::ffi::c_int,
    ));
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0 {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn observe_take_diff(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut kind: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    kind = borg_guess_kind(a, c, y, x);
    if kind.is_null() {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    i = borg_new_take(kind, y, x);
    take = borg_takes.offset(i as isize) as *mut borg_take;
    (*take).when = crate::bridge::st().borg.time.now;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn observe_take_move(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut k_ptr: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut current_block_17: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            if !(*take).seen {
                ox = (*take).x as ::core::ffi::c_int;
                oy = (*take).y as ::core::ffi::c_int;
                z = borg_distance(oy, ox, y, x);
                if !(z > d) {
                    k_ptr = k_info.offset((*take).kidx as isize) as *mut object_kind;
                    if !(crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                        == 0
                        && c != (*k_ptr).d_char)
                    {
                        if crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                            == 0
                            && a as ::core::ffi::c_int != (*k_ptr).d_attr as ::core::ffi::c_int
                            && ((*k_ptr).d_attr as ::core::ffi::c_int != 11 as ::core::ffi::c_int
                                && (*k_ptr).d_char == '!' as i32)
                        {
                            if (*k_ptr).flavor.is_null() {
                                current_block_17 = 16668937799742929182;
                            } else {
                                current_block_17 = 2968425633554183086;
                            }
                        } else {
                            current_block_17 = 2968425633554183086;
                        }
                        match current_block_17 {
                            16668937799742929182 => {}
                            _ => {
                                if z != 0 {
                                    (*crate::bridge::grid((*take).x, (*take).y))
                                        .take = 0 as uint8_t;
                                    (*take).x = x as uint8_t;
                                    (*take).y = y as uint8_t;
                                    (*crate::bridge::grid((*take).x, (*take).y))
                                        .take = i as uint8_t;
                                    borg_note(format(
                                        b"# Tracking an object '%s' at (%d,%d) from (%d,%d)\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        (*k_ptr).name,
                                        (*take).y as ::core::ffi::c_int,
                                        (*take).x as ::core::ffi::c_int,
                                        ox,
                                        oy,
                                    ));
                                    if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE {
                                        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
                                    }
                                }
                                (*take).when = crate::bridge::st().borg.time.now;
                                (*take).seen = true_0 != 0;
                                (*crate::bridge::grid((*take).x, (*take).y)).feat =
                                    FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
                                return true_0 != 0;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut leash: ::core::ffi::c_int = borg_get_leash(true_0 != 0);
    let mut full_quiver: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if crate::bridge::st().borg.trait_[(BI_FAST_SHOTS as ::core::ffi::c_int) as usize]
        != 0
    {
        full_quiver = ((*z_info).quiver_slot_size as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int;
    } else {
        full_quiver = (*z_info).quiver_slot_size as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.mon.scary {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.morgoth_position {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    let mut current_block_29: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            if b_stair != -(1 as ::core::ffi::c_int)
                && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 10 as ::core::ffi::c_int
            {
                j = borg_distance(
                    *track_less.y.offset(b_stair as isize),
                    *track_less.x.offset(b_stair as isize),
                    y,
                    x,
                );
                if j != 255 as ::core::ffi::c_int && (b_j <= leash && j >= leash) {
                    current_block_29 = 7175849428784450219;
                } else {
                    current_block_29 = 6669252993407410313;
                }
            } else {
                current_block_29 = 6669252993407410313;
            }
            match current_block_29 {
                7175849428784450219 => {}
                _ => {
                    if !((*take).value <= 0 as ::core::ffi::c_int) {
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(viewable as ::core::ffi::c_int != 0
                            && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                        {
                            if !((*take).tval
                                == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                                && crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                                    >= full_quiver)
                            {
                                if crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                                    >= 500000 as ::core::ffi::c_int
                                    && crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
                                {
                                    if (*take).tval == TV_GOLD as ::core::ffi::c_int {
                                        current_block_29 = 7175849428784450219;
                                    } else if tval_is_book_k(
                                        k_info.offset((*take).kidx as isize) as *mut object_kind
                                    )
                                        as ::core::ffi::c_int
                                        != 0
                                        && !obj_kind_can_browse(
                                            k_info.offset((*take).kidx as isize)
                                                as *mut object_kind,
                                        )
                                    {
                                        current_block_29 = 7175849428784450219;
                                    } else if ((*take).tval == TV_SHOT as ::core::ffi::c_int
                                        || (*take).tval == TV_ARROW as ::core::ffi::c_int
                                        || (*take).tval == TV_BOLT as ::core::ffi::c_int)
                                        && (*take).tval
                                            != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                                    {
                                        current_block_29 = 7175849428784450219;
                                    } else {
                                        current_block_29 = 14434620278749266018;
                                    }
                                } else {
                                    current_block_29 = 14434620278749266018;
                                }
                                match current_block_29 {
                                    7175849428784450219 => {}
                                    _ => {
                                        borg_flow_clear();
                                        if !(nearness > 5 as ::core::ffi::c_int
                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                < 20 as ::core::ffi::c_int
                                            && borg_flow_cost_stair(y, x, b_stair) > leash)
                                        {
                                            borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                            borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                            borg_temp_n += 1;
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
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take_scum(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if !((*take).value <= 0 as ::core::ffi::c_int) {
                if !(viewable as ::core::ffi::c_int != 0
                    && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                {
                    if !borg_flow_far_from_stairs(x, y, b_stair) {
                        borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                        borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                        borg_temp_n += 1;
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        true_0 != 0,
    );
    if !borg_flow_commit(
        b"Scum item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take_lunal(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        let mut k_ptr: *mut object_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
        let mut item_bad: bool = false;
        if !k_ptr.is_null() {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            item_bad = true_0 != 0;
            if (*take).tval == TV_GOLD as ::core::ffi::c_int {
                borg_note(format(
                    b"# Lunal Item %s, at %d,%d\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    y,
                    x,
                ));
                item_bad = false_0 != 0;
            }
            if item_bad as ::core::ffi::c_int != 0 && (*take).value > 0 as ::core::ffi::c_int {
                if borg_is_ammo((*take).tval) {
                    ii = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
                    while ii
                        < (*z_info).pack_size as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int
                            + (*z_info).quiver_size as ::core::ffi::c_int
                    {
                        if !((*borg_items.offset(ii as isize)).iqty == 0) {
                            if !((*borg_items.offset(ii as isize)).iqty as ::core::ffi::c_int
                                == (*z_info).quiver_slot_size as ::core::ffi::c_int)
                            {
                                if !((*take).kidx != (*borg_items.offset(ii as isize)).kind) {
                                    if (*k_ptr).sval
                                        == (*borg_items.offset(ii as isize)).sval
                                            as ::core::ffi::c_int
                                        && (*k_ptr).tval
                                            == (*borg_items.offset(ii as isize)).tval
                                                as ::core::ffi::c_int
                                    {
                                        item_bad = false_0 != 0;
                                    }
                                }
                            }
                        }
                        ii += 1;
                    }
                } else if (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int
                        - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .iqty
                    != 0
                {
                    ii = 0 as ::core::ffi::c_int;
                    while ii
                        < (*z_info).pack_size as ::core::ffi::c_int
                            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                    {
                        if !((*borg_items.offset(ii as isize)).iqty == 0) {
                            if !((*take).kidx != (*borg_items.offset(ii as isize)).kind) {
                                if (*k_ptr).sval
                                    == (*borg_items.offset(ii as isize)).sval as ::core::ffi::c_int
                                    && (*k_ptr).tval
                                        == (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                    && ((*borg_items.offset(ii as isize)).tval
                                        as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        || (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                            == TV_SCROLL as ::core::ffi::c_int
                                        || (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                            == TV_ROD as ::core::ffi::c_int)
                                {
                                    item_bad = false_0 != 0;
                                }
                            }
                        }
                        ii += 1;
                    }
                }
            }
            if (*borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int
                    - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                    - 1 as ::core::ffi::c_int) as isize,
            ))
            .iqty
                == 0
                && item_bad as ::core::ffi::c_int == true_0
            {
                if !borg_is_ammo((*take).tval)
                    || (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int
                            + (*z_info).quiver_size as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                    .iqty as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    if (*take).tval == TV_POTION as ::core::ffi::c_int
                        && (*k_info.offset((*take).kidx as isize)).sval >= sv_potion_inc_str
                        && (*k_info.offset((*take).kidx as isize)).sval <= sv_potion_detect_invis
                    {
                        borg_note(format(
                            b"# Lunal Item %s, at %d,%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*k_info.offset((*take).kidx as isize)).name,
                            y,
                            x,
                        ));
                        item_bad = false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*k_info.offset((*take).kidx as isize)).kind_flags as *mut bitflag,
                        (KF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        KF_INSTA_ART as ::core::ffi::c_int,
                    ) {
                        borg_note(format(
                            b"# Lunal Item %s, at %d,%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*k_info.offset((*take).kidx as isize)).name,
                            y,
                            x,
                        ));
                        item_bad = false_0 != 0;
                    }
                    if crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0 {
                        if (*take).value >= 1 as ::core::ffi::c_int {
                            item_bad = false_0 != 0;
                        }
                    }
                }
            }
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if !(viewable as ::core::ffi::c_int != 0
                && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
            {
                if !borg_flow_far_from_stairs(x, y, b_stair) {
                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                    borg_temp_n += 1;
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        false_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        true_0 != 0,
    );
    if !borg_flow_commit(
        b"munchkin item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if borg_check_light() {
        return true_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_take() {
    borg_free_flow_take();
    borg_takes_cnt = 0 as int16_t;
    borg_takes_nxt = 1 as int16_t;
    borg_takes =
        mem_zalloc((256 as size_t).wrapping_mul(::core::mem::size_of::<borg_take>() as size_t))
            as *mut borg_take;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_take() {
    if !borg_takes.is_null() {
        mem_free(borg_takes as *mut ::core::ffi::c_void);
        borg_takes = ::core::ptr::null_mut::<borg_take>();
    }
}
