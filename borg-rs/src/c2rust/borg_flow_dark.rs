pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const AUTO_TEMP_MAX: ::core::ffi::c_int = 9000 as ::core::ffi::c_int;
pub const GOAL_DARK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_cave_light_h::{borg_light_n, borg_light_x, borg_light_y};
use crate::headers::borg_cave_util_h::borg_cave_floor_grid;

use crate::headers::borg_danger_h::{borg_danger, borg_danger_wipe};
pub use crate::headers::borg_flow_h::{
    borg_can_dig, borg_data, borg_data_cost, borg_data_icky, borg_data_know, borg_flow_clear,
    borg_flow_commit, borg_flow_enqueue_grid, borg_flow_old, borg_flow_spread, borg_temp_n,
    borg_temp_x, borg_temp_y, borg_track, track_door,
};
use crate::headers::borg_flow_misc_h::{
    borg_flow_far_from_stairs, borg_flow_far_from_stairs_dist, borg_get_leash,
};
use crate::headers::borg_flow_stairs_h::track_less;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
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
    chunk, connector, ddx_ddd, ddy_ddd, distance, feat_is_trap_holding, heatmap, monster_group,
    square, trap, C2RustUnnamedH_18, FEAT_MAX,
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
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
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
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
unsafe extern "C" fn borg_flow_dark_interesting(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut oy: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if ((*ag).feat as ::core::ffi::c_int) < FEAT_SECRET as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_CLOSED as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA_K as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ_K as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            >= 100000 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if !borg_can_dig(false_0 != 0, (*ag).feat) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        {
            return false_0 != 0;
        }
        if !borg.status.vault {
            return false_0 != 0;
        }
        if !borg_can_dig(false_0 != 0, (*ag).feat) {
            return false_0 != 0;
        }
        if x < AUTO_MAX_X - 1 as ::core::ffi::c_int
            && y < AUTO_MAX_Y - 1 as ::core::ffi::c_int
            && x > 1 as ::core::ffi::c_int
            && y > 1 as ::core::ffi::c_int
        {
            ox = -(1 as ::core::ffi::c_int);
            while ox <= 1 as ::core::ffi::c_int {
                oy = -(1 as ::core::ffi::c_int);
                while oy <= 1 as ::core::ffi::c_int {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((oy + y) as isize))
                        .offset((ox + x) as isize) as *mut borg_grid;
                    if (*ag).feat as ::core::ffi::c_int != FEAT_PERM as ::core::ffi::c_int {
                        oy += 1;
                    } else {
                        return true_0 != 0;
                    }
                }
                ox += 1;
            }
        }
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_RUBBLE as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            == 0
    {
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int {
        if borg.near.breeder {
            i = 0 as ::core::ffi::c_int;
            while i < track_door.num as ::core::ffi::c_int {
                if *track_door.x.offset(i as isize) == x && *track_door.y.offset(i as isize) == y {
                    return false_0 != 0;
                }
                i += 1;
            }
        }
        return true_0 != 0;
    }
    if feat_is_trap_holding((*ag).feat as ::core::ffi::c_int) {
        if *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            != 0
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 99 as ::core::ffi::c_int
            && (*ag).trap as ::core::ffi::c_int != 0
            && !(*ag).glyph
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 60 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) < 30 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 20 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) < 45 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 10 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) < 30 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 20 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) < 45 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 10 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if borg.mon.scary {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_flow_dark_reachable(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut j: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        let mut y2: ::core::ffi::c_int = y + ddy_ddd[j as usize] as ::core::ffi::c_int;
        let mut x2: ::core::ffi::c_int = x + ddx_ddd[j as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize)).offset(x2 as isize)
            as *mut borg_grid;
        if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
            if borg_cave_floor_grid(ag) {
                return true_0 != 0;
            }
        }
        j += 1;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_flow_direct(mut y: ::core::ffi::c_int, mut x: ::core::ffi::c_int) {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut fear: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if (*borg_data_icky).data[y as usize][x as usize] != 0 {
        return;
    }
    if (*borg_data_know).data[y as usize][x as usize] == 0 {
        (*borg_data_know).data[y as usize][x as usize] = true_0 as uint8_t;
        p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            == 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            != 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if borg.mon.scary {
            fear = borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
        }
        if borg.mon.unique != 0
            && borg.status.vault as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                == 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 5 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if borg.goal.ignoring {
            fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
        }
        if borg.time.now - borg.time.level > 5000 as borg_time {
            fear = borg.avoidance as ::core::ffi::c_int * 25 as ::core::ffi::c_int;
        }
        if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
            fear = borg.avoidance as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if p > fear {
            (*borg_data_icky).data[y as usize][x as usize] = true_0 as uint8_t;
            return;
        }
    }
    (*borg_data_cost).data[y as usize][x as usize] = 0 as uint8_t;
    y1 = y;
    x1 = x;
    y2 = borg.c.y;
    x2 = borg.c.x;
    ay = if y2 < y1 { y1 - y2 } else { y2 - y1 };
    ax = if x2 < x1 { x1 - x2 } else { x2 - x1 };
    loop {
        if x == x2 && y == y2 {
            return;
        }
        n += 1;
        if ay > ax {
            shift = (n * ax + (ay - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int) / ay;
            x = if x2 < x1 { x1 - shift } else { x1 + shift };
            y = if y2 < y1 { y1 - n } else { y1 + n };
        } else {
            shift = (n * ay + (ax - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int) / ax;
            y = if y2 < y1 { y1 - shift } else { y1 + shift };
            x = if x2 < x1 { x1 - n } else { x1 + n };
        }
        if !(x >= 0 as ::core::ffi::c_int
            && y >= 0 as ::core::ffi::c_int
            && x < AUTO_MAX_X
            && y < AUTO_MAX_Y)
        {
            return;
        }
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if !borg_cave_floor_grid(ag)
            || (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
                && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
        {
            return;
        }
        if (*ag).trap as ::core::ffi::c_int != 0
            && borg.avoidance as ::core::ffi::c_int
                <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            && !borg.mon.scary
        {
            if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < 60 as ::core::ffi::c_int
            {
                return;
            }
            if *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 20 as ::core::ffi::c_int
            {
                return;
            }
            if *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize)
                < 45 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 10 as ::core::ffi::c_int
            {
                return;
            }
            if *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 20 as ::core::ffi::c_int
            {
                return;
            }
            if *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize)
                < 45 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 10 as ::core::ffi::c_int
            {
                return;
            }
        }
        if (*borg_data_icky).data[y as usize][x as usize] != 0 {
            return;
        }
        if (*borg_data_know).data[y as usize][x as usize] == 0 {
            (*borg_data_know).data[y as usize][x as usize] = true_0 as uint8_t;
            p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
            if *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                == 50 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int;
            }
            if *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                != 50 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int;
            }
            if borg.mon.scary {
                fear = borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            }
            if borg.mon.unique != 0
                && borg.status.vault as ::core::ffi::c_int != 0
                && *borg
                    .trait_0
                    .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                    == 50 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
            }
            if borg.mon.scary as ::core::ffi::c_int != 0
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 5 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
            }
            if borg.goal.ignoring {
                fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
            }
            if borg.time.now - borg.time.level > 5000 as borg_time {
                fear = borg.avoidance as ::core::ffi::c_int * 25 as ::core::ffi::c_int;
            }
            if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
            }
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            {
                fear = borg.avoidance as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int;
            }
            if p > fear {
                (*borg_data_icky).data[y as usize][x as usize] = true_0 as uint8_t;
                return;
            }
        }
        if (*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int <= n {
            break;
        }
        (*borg_data_cost).data[y as usize][x as usize] = n as uint8_t;
    }
}
unsafe extern "C" fn borg_flow_border(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
    mut stop: bool,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = y1;
    while y <= y2 {
        (*borg_data_know).data[y as usize][x1 as usize] = stop as uint8_t;
        (*borg_data_icky).data[y as usize][x1 as usize] = stop as uint8_t;
        (*borg_data_know).data[y as usize][x2 as usize] = stop as uint8_t;
        (*borg_data_icky).data[y as usize][x2 as usize] = stop as uint8_t;
        y += 1;
    }
    x = x1;
    while x <= x2 {
        (*borg_data_know).data[y1 as usize][x as usize] = stop as uint8_t;
        (*borg_data_icky).data[y1 as usize][x as usize] = stop as uint8_t;
        (*borg_data_know).data[y2 as usize][x as usize] = stop as uint8_t;
        (*borg_data_icky).data[y2 as usize][x as usize] = stop as uint8_t;
        x += 1;
    }
}
unsafe extern "C" fn borg_flow_dark_1(mut b_stair: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < borg_light_n as ::core::ffi::c_int {
        y = borg_light_y[i as usize] as ::core::ffi::c_int;
        x = borg_light_x[i as usize] as ::core::ffi::c_int;
        if borg_flow_dark_interesting(y, x) {
            if !borg_flow_far_from_stairs(x, y, b_stair) {
                borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                borg_temp_n += 1;
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    if borg.goal.ignoring as ::core::ffi::c_int != 0 || borg.mon.scary as ::core::ffi::c_int != 0 {
        borg_danger_wipe = true_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        borg_flow_direct(y, x);
        i += 1;
    }
    if !borg_flow_commit(
        b"dark-1\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_DARK,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_DARK) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_flow_dark_2(mut b_stair: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    r = *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int * r;
        x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int * r;
        if !(y < 1 as ::core::ffi::c_int) {
            if !(x < 1 as ::core::ffi::c_int) {
                if !(y > AUTO_MAX_Y - 2 as ::core::ffi::c_int) {
                    if !(x > AUTO_MAX_X - 2 as ::core::ffi::c_int) {
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !((*ag).feat as ::core::ffi::c_int != FEAT_NONE as ::core::ffi::c_int) {
                            if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                                if !borg_flow_far_from_stairs(x, y, b_stair) {
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
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    if borg.goal.ignoring as ::core::ffi::c_int != 0 || borg.mon.scary as ::core::ffi::c_int != 0 {
        borg_danger_wipe = true_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        borg_flow_direct(y, x);
        i += 1;
    }
    if !borg_flow_commit(
        b"dark-2\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_DARK,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_DARK) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_flow_dark_3(mut b_stair: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    y1 = borg.c.y - 4 as ::core::ffi::c_int;
    x1 = borg.c.x - 4 as ::core::ffi::c_int;
    y2 = borg.c.y + 4 as ::core::ffi::c_int;
    x2 = borg.c.x + 4 as ::core::ffi::c_int;
    if y1 < 1 as ::core::ffi::c_int {
        y1 = 1 as ::core::ffi::c_int;
    }
    if x1 < 1 as ::core::ffi::c_int {
        x1 = 1 as ::core::ffi::c_int;
    }
    if y2 > AUTO_MAX_Y - 2 as ::core::ffi::c_int {
        y2 = AUTO_MAX_Y - 2 as ::core::ffi::c_int;
    }
    if x2 > AUTO_MAX_X - 2 as ::core::ffi::c_int {
        x2 = AUTO_MAX_X - 2 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            if borg_flow_dark_interesting(y, x) {
                if borg_flow_dark_reachable(y, x) {
                    if !borg_flow_far_from_stairs(x, y, b_stair) {
                        borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                        borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                        borg_temp_n += 1;
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    if borg.goal.ignoring as ::core::ffi::c_int != 0 || borg.mon.scary as ::core::ffi::c_int != 0 {
        borg_danger_wipe = true_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        borg_flow_enqueue_grid(y, x);
        i += 1;
    }
    borg_flow_spread(
        5 as ::core::ffi::c_int,
        false_0 != 0,
        true_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"dark-3\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_DARK,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_DARK) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_flow_dark_4(mut b_stair: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut leash: ::core::ffi::c_int = borg_get_leash(false_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    if borg.status.vault {
        return false_0 != 0;
    }
    y1 = borg.c.y - 11 as ::core::ffi::c_int;
    x1 = borg.c.x - 11 as ::core::ffi::c_int;
    y2 = borg.c.y + 11 as ::core::ffi::c_int;
    x2 = borg.c.x + 11 as ::core::ffi::c_int;
    if y1 < 1 as ::core::ffi::c_int {
        y1 = 1 as ::core::ffi::c_int;
    }
    if x1 < 1 as ::core::ffi::c_int {
        x1 = 1 as ::core::ffi::c_int;
    }
    if y2 > AUTO_MAX_Y - 2 as ::core::ffi::c_int {
        y2 = AUTO_MAX_Y - 2 as ::core::ffi::c_int;
    }
    if x2 > AUTO_MAX_X - 2 as ::core::ffi::c_int {
        x2 = AUTO_MAX_X - 2 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            if borg_flow_dark_interesting(y, x) {
                if borg_flow_dark_reachable(y, x) {
                    if !borg_flow_far_from_stairs_dist(x, y, b_stair, leash) {
                        borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                        borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                        borg_temp_n += 1;
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    if borg.goal.ignoring as ::core::ffi::c_int != 0 || borg.mon.scary as ::core::ffi::c_int != 0 {
        borg_danger_wipe = true_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        borg_flow_enqueue_grid(y, x);
        i += 1;
    }
    y1 -= 1;
    x1 -= 1;
    y2 += 1;
    x2 += 1;
    borg_flow_border(y1, x1, y2, x2, true_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        borg_flow_spread(
            leash,
            true_0 != 0,
            true_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
    } else {
        borg_flow_spread(
            250 as ::core::ffi::c_int,
            true_0 != 0,
            true_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
    }
    borg_flow_border(y1, x1, y2, x2, false_0 != 0);
    if !borg_flow_commit(
        b"dark-4\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_DARK,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_DARK) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_flow_dark_5(mut b_stair: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut leash: ::core::ffi::c_int = borg_get_leash(false_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    y = 1 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            if borg_flow_dark_interesting(y, x) {
                if borg_flow_dark_reachable(y, x) {
                    if !borg_flow_far_from_stairs_dist(x, y, b_stair, leash) {
                        borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                        borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                        borg_temp_n += 1;
                        if borg_temp_n as ::core::ffi::c_int == AUTO_TEMP_MAX {
                            y = AUTO_MAX_Y;
                            x = AUTO_MAX_X;
                            break;
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    if borg.goal.ignoring as ::core::ffi::c_int != 0 || borg.mon.scary as ::core::ffi::c_int != 0 {
        borg_danger_wipe = true_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        borg_flow_enqueue_grid(y, x);
        i += 1;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 5 as ::core::ffi::c_int
        && borg.avoidance as ::core::ffi::c_int
            <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        borg_flow_spread(
            leash,
            true_0 != 0,
            true_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
    } else if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 30 as ::core::ffi::c_int
        && borg.avoidance as ::core::ffi::c_int
            <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        borg_flow_spread(
            leash,
            true_0 != 0,
            true_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
    } else {
        borg_flow_spread(
            250 as ::core::ffi::c_int,
            true_0 != 0,
            true_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
    }
    if !borg_flow_commit(
        b"dark-5\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_DARK,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_DARK) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_dark(mut neer: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if borg.morgoth_position as ::core::ffi::c_int != 0
        && borg.near.morgoth as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    if borg_flow_dark_interesting(borg.c.y, borg.c.x) {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    if neer {
        if borg_flow_dark_1(b_stair) {
            return true_0 != 0;
        }
        if borg_flow_dark_2(b_stair) {
            return true_0 != 0;
        }
        if borg_flow_dark_3(b_stair) {
            return true_0 != 0;
        }
    } else {
        if borg_flow_dark_4(b_stair) {
            return true_0 != 0;
        }
        if borg_flow_dark_5(b_stair) {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
