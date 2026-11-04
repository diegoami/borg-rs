pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
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
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut borg_light_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_light_x: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_light_y: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_glow_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_glow_x: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_glow_y: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub unsafe extern "C" fn borg_update_light() {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut min_x: ::core::ffi::c_int = 0;
    let mut max_x: ::core::ffi::c_int = 0;
    let mut min_y: ::core::ffi::c_int = 0;
    let mut max_y: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_light_n as ::core::ffi::c_int {
        y = borg_light_y[i as usize] as ::core::ffi::c_int;
        x = borg_light_x[i as usize] as ::core::ffi::c_int;
        let ref mut fresh0 = (*crate::bridge::grid(x, y)).info;
        *fresh0 = (*fresh0 as ::core::ffi::c_int & !BORG_LIGHT) as uint16_t;
        i += 1;
    }
    borg_light_n = 0 as int16_t;
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
    .iqty as ::core::ffi::c_int
        <= 0 as ::core::ffi::c_int
    {
        return;
    }
    let ref mut fresh1 = (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).info;
    *fresh1 = (*fresh1 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
    borg_light_y[borg_light_n as usize] = crate::bridge::st().borg.c.y as uint8_t;
    borg_light_x[borg_light_n as usize] = crate::bridge::st().borg.c.x as uint8_t;
    borg_light_n += 1;
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] >= 1 as ::core::ffi::c_int {
        let ref mut fresh2 = (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int)))
        .info;
        *fresh2 = (*fresh2 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = crate::bridge::st().borg.c.x as uint8_t;
        borg_light_n += 1;
        let ref mut fresh3 = (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int)))
        .info;
        *fresh3 = (*fresh3 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = crate::bridge::st().borg.c.x as uint8_t;
        borg_light_n += 1;
        let ref mut fresh4 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int), crate::bridge::st().borg.c.y))
        .info;
        *fresh4 = (*fresh4 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = crate::bridge::st().borg.c.y as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
        let ref mut fresh5 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int), crate::bridge::st().borg.c.y))
        .info;
        *fresh5 = (*fresh5 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = crate::bridge::st().borg.c.y as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
        let ref mut fresh6 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int)))
        .info;
        *fresh6 = (*fresh6 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
        let ref mut fresh7 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int)))
        .info;
        *fresh7 = (*fresh7 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
        let ref mut fresh8 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int)))
        .info;
        *fresh8 = (*fresh8 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
        let ref mut fresh9 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int)))
        .info;
        *fresh9 = (*fresh9 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
        borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 1 as ::core::ffi::c_int) as uint8_t;
        borg_light_n += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] >= 2 as ::core::ffi::c_int
        && (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int) < AUTO_MAX_Y
        && crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        && (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) < AUTO_MAX_X
        && crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int, crate::bridge::st().borg.c.x) {
            let ref mut fresh10 = (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int)))
            .info;
            *fresh10 = (*fresh10 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = crate::bridge::st().borg.c.x as uint8_t;
            borg_light_n += 1;
            let ref mut fresh11 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int)))
            .info;
            *fresh11 = (*fresh11 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh12 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int)))
            .info;
            *fresh12 = (*fresh12 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int, crate::bridge::st().borg.c.x) {
            let ref mut fresh13 = (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int)))
            .info;
            *fresh13 = (*fresh13 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = crate::bridge::st().borg.c.x as uint8_t;
            borg_light_n += 1;
            let ref mut fresh14 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int)))
            .info;
            *fresh14 = (*fresh14 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh15 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int)))
            .info;
            *fresh15 = (*fresh15 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) {
            let ref mut fresh16 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int), crate::bridge::st().borg.c.y))
            .info;
            *fresh16 = (*fresh16 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = crate::bridge::st().borg.c.y as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh17 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int)))
            .info;
            *fresh17 = (*fresh17 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 1 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh18 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int)))
            .info;
            *fresh18 = (*fresh18 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 1 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) {
            let ref mut fresh19 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int), crate::bridge::st().borg.c.y))
            .info;
            *fresh19 = (*fresh19 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = crate::bridge::st().borg.c.y as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh20 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int)))
            .info;
            *fresh20 = (*fresh20 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
            let ref mut fresh21 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int)))
            .info;
            *fresh21 = (*fresh21 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] >= 3 as ::core::ffi::c_int
        && (crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int) < AUTO_MAX_Y
        && crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        && (crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int) < AUTO_MAX_X
        && crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut d: ::core::ffi::c_int = 0;
        let mut p: ::core::ffi::c_int = 0;
        p = crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize];
        if p > 5 as ::core::ffi::c_int {
            p = 5 as ::core::ffi::c_int;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int, crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int) {
            let ref mut fresh22 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int)))
            .info;
            *fresh22 = (*fresh22 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int, crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int) {
            let ref mut fresh23 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int)))
            .info;
            *fresh23 = (*fresh23 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y + 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int, crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int) {
            let ref mut fresh24 = (*crate::bridge::grid((crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int)))
            .info;
            *fresh24 = (*fresh24 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x + 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int, crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int) {
            let ref mut fresh25 = (*crate::bridge::grid((crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int), (crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int)))
            .info;
            *fresh25 = (*fresh25 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
            borg_light_y[borg_light_n as usize] = (crate::bridge::st().borg.c.y - 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_x[borg_light_n as usize] = (crate::bridge::st().borg.c.x - 3 as ::core::ffi::c_int) as uint8_t;
            borg_light_n += 1;
        }
        min_y = crate::bridge::st().borg.c.y - p;
        if min_y < 0 as ::core::ffi::c_int {
            min_y = 0 as ::core::ffi::c_int;
        }
        max_y = crate::bridge::st().borg.c.y + p;
        if max_y > AUTO_MAX_Y - 1 as ::core::ffi::c_int {
            max_y = AUTO_MAX_Y - 1 as ::core::ffi::c_int;
        }
        min_x = crate::bridge::st().borg.c.x - p;
        if min_x < 0 as ::core::ffi::c_int {
            min_x = 0 as ::core::ffi::c_int;
        }
        max_x = crate::bridge::st().borg.c.x + p;
        if max_x > AUTO_MAX_X - 1 as ::core::ffi::c_int {
            max_x = AUTO_MAX_X - 1 as ::core::ffi::c_int;
        }
        y = min_y;
        while y <= max_y {
            x = min_x;
            while x <= max_x {
                let mut dy: ::core::ffi::c_int = if crate::bridge::st().borg.c.y > y {
                    crate::bridge::st().borg.c.y - y
                } else {
                    y - crate::bridge::st().borg.c.y
                };
                let mut dx: ::core::ffi::c_int = if crate::bridge::st().borg.c.x > x {
                    crate::bridge::st().borg.c.x - x
                } else {
                    x - crate::bridge::st().borg.c.x
                };
                if !(dy <= 2 as ::core::ffi::c_int && dx <= 2 as ::core::ffi::c_int) {
                    d = if dy > dx {
                        dy + (dx >> 1 as ::core::ffi::c_int)
                    } else {
                        dx + (dy >> 1 as ::core::ffi::c_int)
                    };
                    if !(d > p) {
                        if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int
                            & BORG_VIEW
                            != 0
                        {
                            let ref mut fresh26 = (*crate::bridge::grid(x, y)).info;
                            *fresh26 = (*fresh26 as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
                            borg_light_y[borg_light_n as usize] = y as uint8_t;
                            borg_light_x[borg_light_n as usize] = x as uint8_t;
                            borg_light_n += 1;
                        }
                    }
                }
                x += 1;
            }
            y += 1;
        }
    }
}
