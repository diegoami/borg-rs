pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};
use crate::headers::borg_cave_util_h::{borg_cave_floor_bold, borg_cave_floor_grid};
use crate::headers::borg_danger_h::borg_fear_region;
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_race_name};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
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
    cave, chunk, connector, distance, heatmap, monster_group, square, square_in_bounds_fully, trap,
    C2RustUnnamedH_18, FEAT_MAX,
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
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut borg_target_loc: loc = loc { x: 0, y: 0 };
#[no_mangle]
pub unsafe extern "C" fn borg_los(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut qx: ::core::ffi::c_int = 0;
    let mut qy: ::core::ffi::c_int = 0;
    let mut tx: ::core::ffi::c_int = 0;
    let mut ty: ::core::ffi::c_int = 0;
    let mut f1: ::core::ffi::c_int = 0;
    let mut f2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    dy = y2 - y1;
    dx = x2 - x1;
    ay = if dy < 0 as ::core::ffi::c_int {
        -dy
    } else {
        dy
    };
    ax = if dx < 0 as ::core::ffi::c_int {
        -dx
    } else {
        dx
    };
    if ax < 2 as ::core::ffi::c_int && ay < 2 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if !square_in_bounds_fully(cave, loc(x1, y1)) {
        return false_0 != 0;
    }
    if dx == 0 {
        if dy > 0 as ::core::ffi::c_int {
            ty = y1 + 1 as ::core::ffi::c_int;
            while ty < y2 {
                if !borg_cave_floor_bold(ty, x1) {
                    return false_0 != 0;
                }
                ty += 1;
            }
        } else {
            ty = y1 - 1 as ::core::ffi::c_int;
            while ty > y2 {
                if !borg_cave_floor_bold(ty, x1) {
                    return false_0 != 0;
                }
                ty -= 1;
            }
        }
        return true_0 != 0;
    }
    if dy == 0 {
        if dx > 0 as ::core::ffi::c_int {
            tx = x1 + 1 as ::core::ffi::c_int;
            while tx < x2 {
                if !borg_cave_floor_bold(y1, tx) {
                    return false_0 != 0;
                }
                tx += 1;
            }
        } else {
            tx = x1 - 1 as ::core::ffi::c_int;
            while tx > x2 {
                if !borg_cave_floor_bold(y1, tx) {
                    return false_0 != 0;
                }
                tx -= 1;
            }
        }
        return true_0 != 0;
    }
    sx = if dx < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    sy = if dy < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    if ax == 1 as ::core::ffi::c_int {
        if ay == 2 as ::core::ffi::c_int {
            if borg_cave_floor_bold(y1 + sy, x1) {
                return true_0 != 0;
            }
        }
    } else if ay == 1 as ::core::ffi::c_int {
        if ax == 2 as ::core::ffi::c_int {
            if borg_cave_floor_bold(y1, x1 + sx) {
                return true_0 != 0;
            }
        }
    }
    f2 = ax * ay;
    f1 = f2 << 1 as ::core::ffi::c_int;
    if ax >= ay {
        qy = ay * ay;
        m = qy << 1 as ::core::ffi::c_int;
        tx = x1 + sx;
        if qy == f2 {
            ty = y1 + sy;
            qy -= f1;
        } else {
            ty = y1;
        }
        while x2 - tx != 0 {
            if !borg_cave_floor_bold(ty, tx) {
                return false_0 != 0;
            }
            qy += m;
            if qy < f2 {
                tx += sx;
            } else if qy > f2 {
                ty += sy;
                if !borg_cave_floor_bold(ty, tx) {
                    return false_0 != 0;
                }
                qy -= f1;
                tx += sx;
            } else {
                ty += sy;
                qy -= f1;
                tx += sx;
            }
        }
    } else {
        qx = ax * ax;
        m = qx << 1 as ::core::ffi::c_int;
        ty = y1 + sy;
        if qx == f2 {
            tx = x1 + sx;
            qx -= f1;
        } else {
            tx = x1;
        }
        while y2 - ty != 0 {
            if !borg_cave_floor_bold(ty, tx) {
                return false_0 != 0;
            }
            qx += m;
            if qx < f2 {
                ty += sy;
            } else if qx > f2 {
                tx += sx;
                if !borg_cave_floor_bold(ty, tx) {
                    return false_0 != 0;
                }
                qx -= f1;
                ty += sy;
            } else {
                tx += sx;
                qx -= f1;
                ty += sy;
            }
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
            || borg.morgoth_position as ::core::ffi::c_int != 0
            || borg.mon.scary as ::core::ffi::c_int != 0
        {
            if dist > 20 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int
        {
            if dist > 10 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            >= borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
        {
            if dist > (*z_info).max_range as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if dist > 2 as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
        {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_offset_projectable(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            break;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable_pure(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        if (*ag).kill != 0 {
            break;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable_dark(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut unknown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            unknown += 1;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 && unknown >= 1 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        if (*ag).kill != 0 {
            break;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inc_motion(
    mut py: *mut ::core::ffi::c_int,
    mut px: *mut ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) {
    let mut dy: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut full: ::core::ffi::c_int = 0;
    let mut half: ::core::ffi::c_int = 0;
    let mut frac: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if y2 < y1 {
        dy = y1 - y2;
        sy = -(1 as ::core::ffi::c_int);
    } else {
        dy = y2 - y1;
        sy = 1 as ::core::ffi::c_int;
    }
    if x2 < x1 {
        dx = x1 - x2;
        sx = -(1 as ::core::ffi::c_int);
    } else {
        dx = x2 - x1;
        sx = 1 as ::core::ffi::c_int;
    }
    if dy == 0 && dx == 0 {
        return;
    }
    half = dy * dx;
    full = half << 1 as ::core::ffi::c_int;
    if *px == x1 && *py == y1 {
        if dy > dx {
            *py += sy;
            return;
        } else if dx > dy {
            *px += sx;
            return;
        } else {
            *px += sx;
            *py += sy;
            return;
        }
    }
    if dy > dx {
        k = dy;
        frac = dx * dx;
        m = frac << 1 as ::core::ffi::c_int;
        y = y1 + sy;
        x = x1;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            if m != 0 {
                frac += m;
                if frac >= half {
                    x += sx;
                    frac -= full;
                }
            }
            y += sy;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    } else if dx > dy {
        frac = dy * dy;
        m = frac << 1 as ::core::ffi::c_int;
        y = y1;
        x = x1 + sx;
        k = dx;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            if m != 0 {
                frac += m;
                if frac >= half {
                    y += sy;
                    frac -= full;
                }
            }
            x += sx;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    } else {
        k = dy;
        y = y1 + sy;
        x = x1 + sx;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            y += sy;
            x += sx;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_distance(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return distance(loc(x, y), loc(x2, y2));
}
#[no_mangle]
pub unsafe extern "C" fn borg_target(mut t: loc, mut require_monster: bool) -> bool {
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    ag = crate::bridge::grid(t.x, t.y)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if (*ag).kill != 0 {
        borg_note(format(
            b"# Targeting %s who has %d Hit Points (%d,%d).\0" as *const u8
                as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).power as ::core::ffi::c_int,
            t.y,
            t.x,
        ));
    } else {
        borg_note(format(
            b"# Targeting location (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            t.y,
            t.x,
        ));
    }
    borg_keypress('*' as i32 as keycode_t);
    borg_keypress('p' as i32 as keycode_t);
    x1 = borg.c.x;
    y1 = borg.c.y;
    x2 = t.x;
    y2 = t.y;
    while y1 < y2 && x1 < x2 {
        borg_keypress('3' as i32 as keycode_t);
        y1 += 1;
        x1 += 1;
    }
    while y1 < y2 && x1 > x2 {
        borg_keypress('1' as i32 as keycode_t);
        y1 += 1;
        x1 -= 1;
    }
    while y1 > y2 && x1 < x2 {
        borg_keypress('9' as i32 as keycode_t);
        y1 -= 1;
        x1 += 1;
    }
    while y1 > y2 && x1 > x2 {
        borg_keypress('7' as i32 as keycode_t);
        y1 -= 1;
        x1 -= 1;
    }
    while y1 < y2 {
        borg_keypress('2' as i32 as keycode_t);
        y1 += 1;
    }
    while y1 > y2 {
        borg_keypress('8' as i32 as keycode_t);
        y1 -= 1;
    }
    while x1 < x2 {
        borg_keypress('6' as i32 as keycode_t);
        x1 += 1;
    }
    while x1 > x2 {
        borg_keypress('4' as i32 as keycode_t);
        x1 -= 1;
    }
    if require_monster {
        borg_keypress('m' as i32 as keycode_t);
    }
    borg_keypress('5' as i32 as keycode_t);
    borg_target_loc.y = t.y;
    borg_target_loc.x = t.x;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_target_unknown_wall(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut found: bool = false_0 != 0;
    let mut y_hall: bool = false_0 != 0;
    let mut x_hall: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    borg_note(format(
        b"# Perhaps wall near targetted location (%d,%d)\0" as *const u8
            as *const ::core::ffi::c_char,
        y,
        x,
    ));
    n_x = borg.c.x;
    n_y = borg.c.y;
    if (*crate::bridge::grid(borg.c.x, (borg.c.y + 1 as ::core::ffi::c_int))).feat
        as ::core::ffi::c_int
        == FEAT_FLOOR as ::core::ffi::c_int
        && (*crate::bridge::grid(borg.c.x, (borg.c.y - 1 as ::core::ffi::c_int)))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
        && ((*crate::bridge::grid(borg.c.x, (borg.c.y + 2 as ::core::ffi::c_int)))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
            || (*crate::bridge::grid(borg.c.x, (borg.c.y - 2 as ::core::ffi::c_int)))
            .feat as ::core::ffi::c_int
                == FEAT_FLOOR as ::core::ffi::c_int)
        && ((*crate::bridge::grid((borg.c.x + 1 as ::core::ffi::c_int), borg.c.y))
            .feat as ::core::ffi::c_int
            != FEAT_FLOOR as ::core::ffi::c_int
            && (*crate::bridge::grid((borg.c.x - 1 as ::core::ffi::c_int), borg.c.y))
            .feat as ::core::ffi::c_int
                != FEAT_FLOOR as ::core::ffi::c_int)
    {
        x_hall = true_0 != 0;
    }
    if (*crate::bridge::grid((borg.c.x + 1 as ::core::ffi::c_int), borg.c.y)).feat
        as ::core::ffi::c_int
        == FEAT_FLOOR as ::core::ffi::c_int
        && (*crate::bridge::grid((borg.c.x - 1 as ::core::ffi::c_int), borg.c.y))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
        && ((*crate::bridge::grid((borg.c.x + 2 as ::core::ffi::c_int), borg.c.y))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
            || (*crate::bridge::grid((borg.c.x - 2 as ::core::ffi::c_int), borg.c.y))
            .feat as ::core::ffi::c_int
                == FEAT_FLOOR as ::core::ffi::c_int)
        && ((*crate::bridge::grid(borg.c.x, (borg.c.y + 1 as ::core::ffi::c_int)))
            .feat as ::core::ffi::c_int
            != FEAT_FLOOR as ::core::ffi::c_int
            && (*crate::bridge::grid(borg.c.x, (borg.c.y - 1 as ::core::ffi::c_int)))
            .feat as ::core::ffi::c_int
                != FEAT_FLOOR as ::core::ffi::c_int)
    {
        y_hall = true_0 != 0;
    }
    loop {
        ag = crate::bridge::grid(n_x, n_y) as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
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
            RF_PASS_WALL as ::core::ffi::c_int,
        ) {
            borg_note(format(
                b"# Guessing wall (%d,%d) under ghostly target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                n_y,
                n_x,
            ));
            (*crate::bridge::grid(n_x, n_y)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        if (*crate::bridge::grid(n_x, n_y)).feat as ::core::ffi::c_int
            == FEAT_NONE as ::core::ffi::c_int
            && (n_y != borg.c.y || !y_hall)
            && (n_x != borg.c.x || !x_hall)
        {
            borg_note(format(
                b"# Guessing wall (%d,%d) near target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                y,
                x,
            ));
            (*crate::bridge::grid(n_x, n_y)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        if n_x == x && n_y == y {
            borg_inc_motion(&raw mut n_y, &raw mut n_x, y, x, borg.c.y, borg.c.x);
            borg_note(format(
                b"# Guessing wall (%d,%d) near target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                y,
                x,
            ));
            (*crate::bridge::grid(n_x, n_y)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        borg_inc_motion(&raw mut n_y, &raw mut n_x, borg.c.y, borg.c.x, y, x);
    }
}
