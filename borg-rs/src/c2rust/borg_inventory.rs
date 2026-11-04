pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_item_analyze_h::borg_item_analyze;
pub use crate::headers::borg_item_h::{borg_deinscribe, borg_item};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;
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
pub use crate::headers::obj_desc_h::{
    object_desc, C2RustUnnamedH_42, ODESC_ALTNUM, ODESC_BASE, ODESC_CAPITAL, ODESC_COMBAT,
    ODESC_EXTRA, ODESC_FULL, ODESC_NOEGO, ODESC_PLURAL, ODESC_PREFIX, ODESC_SINGULAR, ODESC_SPOIL,
    ODESC_STORE, ODESC_TERSE,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, TV_MAX};
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
use crate::headers::string_h::{memset, strcmp, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut borg_do_crush_junk: bool = false_0 != 0;
#[no_mangle]
pub unsafe extern "C" fn borg_wield_slot(mut item: *const borg_item) -> ::core::ffi::c_int {
    match (*item).tval as ::core::ffi::c_int {
        9 | 8 | 7 | 6 => return (*z_info).pack_size as ::core::ffi::c_int,
        18 | 17 | 16 => {
            return (*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
        }
        14 => return (*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
        13 | 12 => {
            return (*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int;
        }
        5 => return (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        21 => return (*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
        20 => return (*z_info).pack_size as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
        19 => return (*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
        15 => return (*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int,
        11 => return (*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
        10 => return (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int,
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_slot(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !((*item).tval as ::core::ffi::c_int != tval) {
                    if !((*item).sval as ::core::ffi::c_int != sval) {
                        if !(n >= 0 as ::core::ffi::c_int
                            && (*item).iqty as ::core::ffi::c_int
                                > (crate::bridge::st().items.items[(n) as usize]).iqty as ::core::ffi::c_int)
                        {
                            if !(n >= 0 as ::core::ffi::c_int
                                && ((*item).pval as ::core::ffi::c_int)
                                    < (crate::bridge::st().items.items[(n) as usize]).pval as ::core::ffi::c_int
                                && (*item).iqty as ::core::ffi::c_int
                                    > (crate::bridge::st().items.items[(n) as usize]).iqty as ::core::ffi::c_int)
                            {
                                n = i;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return n;
}
unsafe extern "C" fn borg_cheat_quiver() {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int =
        (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while j < (*z_info).quiver_size as ::core::ffi::c_int {
        memset(
            &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        let mut obj: *mut object = *(*(*player).upkeep).quiver.offset(j as isize);
        if !obj.is_null() {
            buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            if !(*obj).kind.is_null() {
                buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                if !(*obj).kind.is_null() {
                    object_desc(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                        obj,
                        ODESC_FULL as ::core::ffi::c_int as uint32_t,
                        player,
                    );
                    borg_item_analyze(
                        &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item,
                        obj,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        false_0 != 0,
                    );
                    if !(crate::bridge::st().items.items[(i) as usize]).note.is_null()
                        && (!strstr(
                            (crate::bridge::st().items.items[(i) as usize]).note,
                            b"!\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            || !strstr(
                                (crate::bridge::st().items.items[(i) as usize]).note,
                                b"borg\0" as *const u8 as *const ::core::ffi::c_char,
                            )
                            .is_null())
                    {
                        borg_deinscribe(i);
                    }
                }
            }
        }
        i += 1;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_equipment() {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut count: ::core::ffi::c_int =
        (*player).body.count as ::core::ffi::c_int + (*z_info).pack_size as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = (*z_info).pack_size as ::core::ffi::c_int;
    while i < count {
        memset(
            &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        let mut obj: *mut object = (*(*player).body.slots.offset(j as isize)).obj;
        if !obj.is_null() {
            buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            if !(*obj).kind.is_null() {
                buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                if !(*obj).kind.is_null() {
                    object_desc(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                        obj,
                        ODESC_FULL as ::core::ffi::c_int as uint32_t,
                        player,
                    );
                    borg_item_analyze(
                        &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item,
                        obj,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        false_0 != 0,
                    );
                    if !(crate::bridge::st().items.items[(i) as usize]).note.is_null()
                        && (!strstr(
                            (crate::bridge::st().items.items[(i) as usize]).note,
                            b"!\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            || !strstr(
                                (crate::bridge::st().items.items[(i) as usize]).note,
                                b"borg\0" as *const u8 as *const ::core::ffi::c_char,
                            )
                            .is_null())
                    {
                        borg_deinscribe(i);
                    }
                }
            }
        }
        i += 1;
        j += 1;
    }
    borg_cheat_quiver();
}
#[no_mangle]
pub unsafe extern "C" fn borg_inventory() {
    let mut i: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut obj: *mut object = *(*(*player).upkeep).inven.offset(i as isize);
        memset(
            &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        if !(obj.is_null() || (*obj).kind.is_null()) {
            buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            object_desc(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                obj,
                ODESC_FULL as ::core::ffi::c_int as uint32_t,
                player,
            );
            if !(strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"(nothing)\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0)
            {
                borg_item_analyze(
                    &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item,
                    obj,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    false_0 != 0,
                );
                borg_do_crush_junk = true_0 != 0;
                if !(crate::bridge::st().items.items[(i) as usize]).note.is_null()
                    && (!strstr(
                        (crate::bridge::st().items.items[(i) as usize]).note,
                        b"!\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null()
                        || !strstr(
                            (crate::bridge::st().items.items[(i) as usize]).note,
                            b"borg\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null())
                {
                    borg_deinscribe(i);
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_first_empty_inventory_slot() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (*z_info).pack_size as ::core::ffi::c_int
        - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
        - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if (crate::bridge::st().items.items[(i) as usize]).iqty != 0 {
            if (i + 1 as ::core::ffi::c_int)
                < (*z_info).pack_size as ::core::ffi::c_int
                    - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
            {
                return i + 1 as ::core::ffi::c_int;
            }
            break;
        } else {
            i -= 1;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_worth_id(mut item: *const borg_item) -> bool {
    let mut slot: ::core::ffi::c_int = 0;
    if !borg_item_note_needs_id(item) {
        return false_0 != 0;
    }
    slot = borg_wield_slot(item);
    if slot < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
