pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::{borg_keypress, borg_keypresses};
use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_get_note, borg_item, borg_items};
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
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, object,
    object_base, object_kind, poss_item, C2RustUnnamedH_38, ELEM_MAX,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{strcmp, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_object_fully_id() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        let mut e_ptr: *mut ego_item = e_info.offset((*item).ego_idx as isize) as *mut ego_item;
        let mut note: *const ::core::ffi::c_char = borg_get_note(item);
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
            && (*item).tval as ::core::ffi::c_int >= TV_BOW as ::core::ffi::c_int
            && (*item).tval as ::core::ffi::c_int <= TV_RING as ::core::ffi::c_int
            && (borg_ego_has_random_power(e_ptr) as ::core::ffi::c_int != 0
                || (*item).art_idx as ::core::ffi::c_int != 0)
            && (strcmp(note, b"{ }\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || strcmp(note, b"\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || !strstr(
                    note,
                    b"uncursed\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
        {
            borg_keypress('{' as i32 as keycode_t);
            if i >= (*z_info).pack_size as ::core::ffi::c_int {
                borg_keypress('/' as i32 as keycode_t);
                borg_keypress(
                    *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                        .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                        as keycode_t,
                );
            } else {
                borg_keypress(
                    *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                        .offset(i as isize) as keycode_t,
                );
            }
            if (*item).modifiers[OBJ_MOD_SPEED as ::core::ffi::c_int as usize] != 0 {
                borg_keypresses(b"Spd\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Poisn\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                borg_keypresses(b"IFir\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                borg_keypresses(b"ICld\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                borg_keypresses(b"IAcd\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                borg_keypresses(b"IElc\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Lite\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Dark\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_PROT_BLIND as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"Blnd\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_PROT_CONF as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"Conf\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Sound\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Shrd\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Nthr\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Nxs\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Chaos\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                borg_keypresses(b"Disn\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if (*item).activ_idx != 0 {
                borg_keypresses(b"Actv\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_TELEPATHY as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"ESP\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_HOLD_LIFE as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"HL\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_FREE_ACT as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"FA\0" as *const u8 as *const ::core::ffi::c_char);
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
                OF_SEE_INVIS as ::core::ffi::c_int,
            ) {
                borg_keypresses(b"SInv\0" as *const u8 as *const ::core::ffi::c_char);
            }
            borg_keypress(KC_ENTER as keycode_t);
        }
        i += 1;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_note_needs_id(mut item: *const borg_item) -> bool {
    if (*item).ident {
        return false_0 != 0;
    }
    if (*item).needs_ident {
        return true_0 != 0;
    }
    return !strstr(
        borg_get_note(item),
        b"{??}\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null();
}
