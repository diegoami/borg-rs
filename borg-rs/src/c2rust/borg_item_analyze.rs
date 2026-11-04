pub const FLAG_WIDTH: usize = (::core::mem::size_of::<bitflag>() as usize).wrapping_mul(8 as usize);
pub const FLAG_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLAG_END: ::core::ffi::c_int = FLAG_START - 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_BLOWS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;

pub use crate::headers::borg_item_h::{
    borg_item, C2RustUnnamedH_44, BORG_CURSE_AIR_SWING, BORG_CURSE_ANNOYANCE,
    BORG_CURSE_ANTI_TELEPORTATION, BORG_CURSE_BURNING_UP, BORG_CURSE_CHILLED_TO_THE_BONE,
    BORG_CURSE_CLUMSINESS, BORG_CURSE_COWARDICE, BORG_CURSE_DEMON_SUMMON, BORG_CURSE_DRAGON_SUMMON,
    BORG_CURSE_DULLNESS, BORG_CURSE_ENVELOPING, BORG_CURSE_HALLUCINATION,
    BORG_CURSE_IMPAIR_HITPOINT_RECOVERY, BORG_CURSE_IMPAIR_MANA_RECOVERY, BORG_CURSE_IRRITATION,
    BORG_CURSE_MAX, BORG_CURSE_PARALYSIS, BORG_CURSE_POISON, BORG_CURSE_SICKLINESS,
    BORG_CURSE_SIREN, BORG_CURSE_SLOWNESS, BORG_CURSE_STEELSKIN, BORG_CURSE_STONE,
    BORG_CURSE_TELEPORTATION, BORG_CURSE_TREACHEROUS_WEAPON, BORG_CURSE_UNDEAD_SUMMON,
    BORG_CURSE_UNKNOWN, BORG_CURSE_VULNERABILITY, BORG_CURSE_WEAKNESS,
};
use crate::headers::borg_item_val_h::{
    sv_dragon_balance, sv_dragon_blue, sv_dragon_chaos, sv_dragon_gold, sv_dragon_green,
    sv_dragon_law, sv_dragon_multihued, sv_dragon_power, sv_dragon_red, sv_dragon_shining,
    sv_ring_acid, sv_ring_dog, sv_ring_flames, sv_ring_ice, sv_ring_lightning,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_elements_h::{
    ELEM_ACID, ELEM_ARROW, ELEM_CHAOS, ELEM_COLD, ELEM_DARK, ELEM_DISEN, ELEM_ELEC, ELEM_FIRE,
    ELEM_FORCE, ELEM_GRAVITY, ELEM_HOLY_ORB, ELEM_ICE, ELEM_INERTIA, ELEM_LIGHT, ELEM_MANA,
    ELEM_METEOR, ELEM_MISSILE, ELEM_NETHER, ELEM_NEXUS, ELEM_PLASMA, ELEM_POIS, ELEM_SHARD,
    ELEM_SOUND, ELEM_TIME, ELEM_WATER,
};
pub use crate::headers::list_kind_flags_h::{
    KF_EASY_KNOW, KF_GOOD, KF_INSTA_ART, KF_MAX, KF_NONE, KF_QUEST_ART, KF_RAND_BASE_RES,
    KF_RAND_HI_RES, KF_RAND_POWER, KF_RAND_RES_POWER, KF_RAND_SUSTAIN, KF_SHOOTS_ARROWS,
    KF_SHOOTS_BOLTS, KF_SHOOTS_SHOTS, KF_SHOW_DICE, KF_SHOW_MULT,
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
pub use crate::headers::list_stats_h::{
    OBJ_MOD_CON, OBJ_MOD_DEX, OBJ_MOD_INT, OBJ_MOD_STR, OBJ_MOD_WIS, STAT_CON, STAT_DEX, STAT_INT,
    STAT_STR, STAT_WIS,
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
    monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop, monster_friends,
    monster_friends_base, monster_group_role, monster_mimic, monster_pain, monster_race,
    monster_shape, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
use crate::headers::obj_curse_h::curses;
use crate::headers::obj_knowledge_h::{
    object_flavor_is_aware, object_fully_known, object_runes_known,
};

pub use crate::headers::obj_properties_h::{
    lookup_obj_property, obj_property, obj_property_type, C2RustUnnamedH_20, C2RustUnnamedH_31,
    C2RustUnnamedH_36, C2RustUnnamedH_39, OBJ_MOD_MAX, OBJ_PROPERTY_FLAG, OBJ_PROPERTY_IGNORE,
    OBJ_PROPERTY_IMM, OBJ_PROPERTY_MAX, OBJ_PROPERTY_MOD, OBJ_PROPERTY_NONE, OBJ_PROPERTY_RESIST,
    OBJ_PROPERTY_STAT, OBJ_PROPERTY_VULN, OF_NONE, TV_MAX,
};
use crate::headers::obj_slays_h::slays;
use crate::headers::obj_util_h::{object_flags_known, object_weight_one};
pub use crate::headers::object_h::{
    a_info, activation, artifact, curse, curse_data, e_info, effect, ego_item, element_info,
    flavor, k_info, object, object_base, object_kind, poss_item, slay, C2RustUnnamedH_38, ELEM_MAX,
};
pub use crate::headers::player_h::{C2RustUnnamedH_23, STAT_MAX};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memset, strcmp};
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has, flag_next};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{
    aspect, randcalc, random, random_value, AVERAGE, EXTREMIFY, MAXIMISE, MINIMISE, RANDOMISE,
};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, my_strcpy};
unsafe extern "C" fn borg_object_value_known(mut item: *mut borg_item) -> int32_t {
    let mut value: int32_t = 0;
    let mut k_ptr: *mut object_kind = k_info.offset((*item).kind as isize) as *mut object_kind;
    if (*k_ptr).cost == 0 {
        return 0 as int32_t;
    }
    value = (*k_ptr).cost as int32_t;
    if (*item).art_idx != 0 {
        let mut a_ptr: *mut artifact = a_info.offset((*item).art_idx as isize) as *mut artifact;
        if (*a_ptr).cost == 0 {
            return 0 as int32_t;
        }
        value = (*a_ptr).cost as int32_t;
    }
    if (*item).ego_idx != 0 {
        let mut e_ptr: *mut ego_item = e_info.offset((*item).ego_idx as isize) as *mut ego_item;
        if (*e_ptr).cost == 0 {
            return 0 as int32_t;
        }
        value = (value as ::core::ffi::c_int + (*e_ptr).cost) as int32_t;
    }
    match (*item).tval as ::core::ffi::c_int {
        23 | 22 => {
            value = (value as ::core::ffi::c_int
                + (value / 20 as int32_t * (*item).pval as int32_t) as ::core::ffi::c_int)
                as int32_t;
        }
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20
        | 21 => {
            if ((*item).pval as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if !((*item).pval == 0) {
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_STR as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_INT as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_WIS as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_DEX as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_CON as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_STEALTH as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SEARCH as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 50 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_TUNNEL as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 50 as ::core::ffi::c_long) as int32_t;
                if (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize] != 0 {
                    if (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > MAX_BLOWS
                    {
                        value = (value as ::core::ffi::c_long
                            + MAX_BLOWS as ::core::ffi::c_long * 2000 as ::core::ffi::c_long)
                            as int32_t;
                    } else {
                        value = (value as ::core::ffi::c_long
                            + (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                * 2000 as ::core::ffi::c_long)
                            as int32_t;
                    }
                }
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SHOTS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 2000 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 30000 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_MIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_MOVES as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_DAM_RED as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 200 as ::core::ffi::c_long) as int32_t;
            }
        }
        _ => {}
    }
    let mut current_block_69: u64;
    match (*item).tval as ::core::ffi::c_int {
        21 => {
            if (*item).sval as ::core::ffi::c_int == sv_ring_dog {
                return 0 as int32_t;
            }
            current_block_69 = 790185930182612747;
        }
        20 => {
            current_block_69 = 790185930182612747;
        }
        10 | 11 | 15 | 13 | 12 | 14 | 16 | 17 | 18 => {
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && (*item).to_h as ::core::ffi::c_int > -(5 as ::core::ffi::c_int)
            {
                value = (value as ::core::ffi::c_long
                    + ((*item).to_d as ::core::ffi::c_int + (*item).to_a as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((*item).to_h as ::core::ffi::c_int
                        + (*item).to_d as ::core::ffi::c_int
                        + (*item).to_a as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        5 | 6 | 7 | 9 | 8 => {
            if ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int
                    + (*item).to_d as ::core::ffi::c_int
                    + (*item).to_a as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long) as int32_t;
            if (*item).dd as ::core::ffi::c_int > (*k_ptr).dd
                && (*item).ds as ::core::ffi::c_int == (*k_ptr).ds
            {
                value = (value as ::core::ffi::c_long
                    + (((*item).dd as ::core::ffi::c_int - (*k_ptr).dd)
                        * (*item).ds as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 200 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        2 | 3 | 4 => {
            if ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 5 as ::core::ffi::c_long) as int32_t;
            if (*item).dd as ::core::ffi::c_int > (*k_ptr).dd
                && (*item).ds as ::core::ffi::c_int == (*k_ptr).ds
            {
                value = (value as ::core::ffi::c_long
                    + (((*item).dd as ::core::ffi::c_int - (*k_ptr).dd)
                        * (*item).ds as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 5 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        _ => {
            current_block_69 = 5028470053297453708;
        }
    }
    match current_block_69 {
        790185930182612747 => {
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int
                    + (*item).to_d as ::core::ffi::c_int
                    + (*item).to_a as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long) as int32_t;
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn borg_object_value_guess(mut item: *mut borg_item) -> int32_t {
    let mut value: int32_t = 0;
    match (*item).tval as ::core::ffi::c_int {
        28 => {
            value = 5 as int32_t;
        }
        26 => {
            value = 20 as int32_t;
        }
        25 => {
            value = 20 as int32_t;
        }
        22 => {
            value = 70 as int32_t;
        }
        23 => {
            value = 50 as int32_t;
        }
        24 => {
            value = 90 as int32_t;
        }
        21 | 20 => {
            value = 45 as int32_t;
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as ::core::ffi::c_int as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < OBJ_MOD_MAX as ::core::ffi::c_int && value != 0 {
                if ((*item).modifiers[i as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    value = 0 as int32_t;
                }
                i += 1;
            }
        }
        _ => {
            value = 20 as int32_t;
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as ::core::ffi::c_int as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
        }
    }
    return value;
}
unsafe extern "C" fn borg_set_slays(mut item: *mut borg_item, mut o: *const object) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).slay_max as ::core::ffi::c_int {
        if *(*o).slays.offset(i as isize) {
            (*item).slays[(*slays.offset(i as isize)).race_flag as usize] =
                (*slays.offset(i as isize)).multiplier;
        }
        i += 1;
    }
}
unsafe extern "C" fn borg_set_curses(mut item: *mut borg_item, mut o: *const object) {
    let mut i: ::core::ffi::c_int = 0;
    let mut item_curses: *mut bool = &raw mut (*item).curses as *mut bool;
    (*item).uncursable = false_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).curse_max as ::core::ffi::c_int {
        let mut c: *mut curse = curses.offset(i as isize) as *mut curse;
        if (*(*o).curses.offset(i as isize)).power > 0 as ::core::ffi::c_int {
            (*item).cursed = true_0 != 0;
            if (*(*o).curses.offset(i as isize)).power < 100 as ::core::ffi::c_int {
                (*item).uncursable = true_0 != 0;
            }
            if strcmp(
                (*c).name,
                b"vulnerability\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_VULNERABILITY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"teleportation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_TELEPORTATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"dullness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DULLNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"sickliness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SICKLINESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"enveloping\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ENVELOPING as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"irritation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_IRRITATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"weakness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_WEAKNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"clumsiness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_CLUMSINESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"slowness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SLOWNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"annoyance\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ANNOYANCE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"poison\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_POISON as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"siren\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SIREN as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"hallucination\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_HALLUCINATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"paralysis\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_PARALYSIS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"dragon summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DRAGON_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"demon summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DEMON_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"undead summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_UNDEAD_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"impair mana recovery\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_IMPAIR_MANA_RECOVERY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"impair hitpoint recovery\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_IMPAIR_HITPOINT_RECOVERY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"cowardice\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_COWARDICE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"stone\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_STONE as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"anti-teleportation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ANTI_TELEPORTATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"treacherous weapon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_TREACHEROUS_WEAPON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"burning up\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_BURNING_UP as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"chilled to the bone\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_CHILLED_TO_THE_BONE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"steelskin\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_STEELSKIN as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"air swing\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_AIR_SWING as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else {
                *item_curses.offset(BORG_CURSE_UNKNOWN as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn borg_item_has_negative_flag(mut item: *const borg_item) -> bool {
    let mut i: size_t = 0;
    i = flag_next(
        &raw const (*item).flags as *const bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add(FLAG_WIDTH)
            .wrapping_sub(1 as size_t)
            .wrapping_div(FLAG_WIDTH),
        1 as ::core::ffi::c_int,
    ) as size_t;
    while i != FLAG_END as size_t {
        let mut flag: *mut obj_property = lookup_obj_property(
            OBJ_PROPERTY_FLAG as ::core::ffi::c_int,
            i as ::core::ffi::c_int,
        );
        if !flag.is_null() && (*flag).power < 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        i = flag_next(
            &raw const (*item).flags as *const bitflag,
            (OF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(FLAG_WIDTH)
                .wrapping_sub(1 as size_t)
                .wrapping_div(FLAG_WIDTH),
            i.wrapping_add(1 as size_t) as ::core::ffi::c_int,
        ) as size_t;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_analyze(
    mut item: *mut borg_item,
    mut real_item: *const object,
    mut desc: *mut ::core::ffi::c_char,
    mut in_store: bool,
) {
    let mut scan: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut o: *const object = ::core::ptr::null::<object>();
    memset(
        item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_item>() as size_t,
    );
    if (*real_item).kind.is_null() || (*real_item).number == 0 {
        return;
    }
    (*item).ident = object_fully_known(real_item);
    if (*item).ident {
        o = real_item;
    } else if (*real_item).known.is_null() {
        o = real_item;
    } else {
        o = (*real_item).known;
    }
    (*item).needs_ident = !object_runes_known(real_item);
    object_flags_known(real_item, &raw mut (*item).flags as *mut bitflag);
    my_strcpy(
        &raw mut (*item).desc as *mut ::core::ffi::c_char,
        desc,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
    );
    scan = &raw mut (*item).desc as *mut ::core::ffi::c_char;
    while *scan as ::core::ffi::c_int != 0 && *scan as ::core::ffi::c_int != '{' as i32 {
        scan = scan.offset(1);
    }
    (*item).note = scan;
    (*item).tval = (*real_item).tval;
    (*item).sval = (*real_item).sval;
    (*item).iqty = (*real_item).number;
    (*item).weight = object_weight_one(real_item);
    (*item).timeout = (*real_item).timeout;
    (*item).level = (*(*real_item).kind).level as uint8_t;
    (*item).aware = in_store as ::core::ffi::c_int != 0
        || object_flavor_is_aware(real_item) as ::core::ffi::c_int != 0;
    (*item).ac = (*o).ac;
    (*item).dd = (*o).dd;
    (*item).ds = (*o).ds;
    (*item).to_h = (*o).to_h;
    (*item).to_d = (*o).to_d;
    (*item).to_a = (*o).to_a;
    i = 0 as ::core::ffi::c_int;
    while i < OBJ_MOD_MAX as ::core::ffi::c_int {
        (*item).modifiers[i as usize] = (*o).modifiers[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ELEM_MAX as ::core::ffi::c_int {
        (*item).el_info[i as usize].res_level = (*o).el_info[i as usize].res_level;
        (*item).el_info[i as usize].flags = (*o).el_info[i as usize].flags;
        i += 1;
    }
    if !(*o).curses.is_null() {
        borg_set_curses(item, o);
    }
    if borg_item_has_negative_flag(item) {
        (*item).cursed = true_0 != 0;
    }
    if !(*o).slays.is_null() {
        borg_set_slays(item, o);
    }
    if !(*o).brands.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).brand_max as ::core::ffi::c_int {
            (*item).brands[i as usize] = *(*o).brands.offset(i as isize);
            i += 1;
        }
    }
    if !(*o).activation.is_null() {
        if (*(*o).activation).index == crate::bridge::st().act.bizarre {
            (*item).one_ring = true_0 != 0;
        }
        (*item).activ_idx = (*(*o).activation).index;
    } else {
        if (*item).tval as ::core::ffi::c_int == TV_RING as ::core::ffi::c_int {
            if (*item).sval as ::core::ffi::c_int == sv_ring_flames {
                (*item).activ_idx = crate::bridge::st().act.ring_flames;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_acid {
                (*item).activ_idx = crate::bridge::st().act.ring_acid;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_ice {
                (*item).activ_idx = crate::bridge::st().act.ring_ice;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_lightning {
                (*item).activ_idx = crate::bridge::st().act.ring_lightning;
            }
        }
        if (*item).tval as ::core::ffi::c_int == TV_DRAG_ARMOR as ::core::ffi::c_int {
            if (*item).sval as ::core::ffi::c_int == sv_dragon_blue {
                (*item).activ_idx = crate::bridge::st().act.dragon_blue;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_red {
                (*item).activ_idx = crate::bridge::st().act.dragon_red;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_green {
                (*item).activ_idx = crate::bridge::st().act.dragon_green;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_multihued {
                (*item).activ_idx = crate::bridge::st().act.dragon_multihued;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_shining {
                (*item).activ_idx = crate::bridge::st().act.dragon_shining;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_law {
                (*item).activ_idx = crate::bridge::st().act.dragon_law;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_gold {
                (*item).activ_idx = crate::bridge::st().act.dragon_gold;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_chaos {
                (*item).activ_idx = crate::bridge::st().act.dragon_chaos;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_balance {
                (*item).activ_idx = crate::bridge::st().act.dragon_balance;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_power {
                (*item).activ_idx = crate::bridge::st().act.dragon_power;
            }
        }
    }
    if (*item).ident {
        (*item).pval = (*o).pval;
    }
    if (*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int {
        if (*item).iqty as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (*item).pval = (if (*real_item).timeout as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as int16_t;
        } else {
            let mut power: ::core::ffi::c_int = 0;
            let mut time_base: ::core::ffi::c_int =
                randcalc((*(*real_item).kind).time, 0 as ::core::ffi::c_int, MINIMISE);
            if time_base == 0 {
                time_base = 1 as ::core::ffi::c_int;
            }
            power = ((*real_item).timeout as ::core::ffi::c_int
                + (time_base - 1 as ::core::ffi::c_int))
                / time_base;
            (*item).pval = (if power < (*item).iqty as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as int16_t;
        }
    } else if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = 1 as int16_t;
        if (*item).ident {
            (*item).pval = (*real_item).pval;
        }
        if !(*item).aware && (*o).pval == 0 {
            (*item).pval = 0 as int16_t;
        }
    }
    if (*item).aware {
        (*item).kind = (*(*o).kind).kidx;
    }
    if !(*o).artifact.is_null() {
        (*item).art_idx = (*(*o).artifact).aidx as uint8_t;
    }
    if !(*o).ego.is_null() {
        (*item).ego_idx = (*(*o).ego).eidx as uint8_t;
    }
    if (*item).ident {
        (*item).value = borg_object_value_known(item);
    } else if (*item).aware {
        (*item).value = (*(*o).kind).cost as int32_t;
    } else {
        (*item).value = borg_object_value_guess(item);
    }
    if (*item).cursed as ::core::ffi::c_int != 0 && !(*item).one_ring {
        (*item).value = 0 as int32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_obj_has_effect(
    mut kind: uint32_t,
    mut index: ::core::ffi::c_int,
    mut subtype: ::core::ffi::c_int,
) -> bool {
    let mut ke: *mut effect = (*k_info.offset(kind as isize)).effect;
    while !ke.is_null() {
        if (*ke).index as ::core::ffi::c_int == index
            && ((*ke).subtype == subtype || subtype == -(1 as ::core::ffi::c_int))
        {
            return true_0 != 0;
        }
        ke = (*ke).next;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_ego_has_random_power(mut e_ptr: *mut ego_item) -> bool {
    if flag_has(
        &raw mut (*e_ptr).kind_flags as *mut bitflag,
        (KF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        KF_RAND_POWER as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_SUSTAIN as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_BASE_RES as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_HI_RES as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_RES_POWER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
