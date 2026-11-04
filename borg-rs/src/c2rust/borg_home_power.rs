pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_home_notice_h::{
    home_damage, home_stat_add, home_un_id, num_LIGHT, num_armor, num_artifact, num_bad_gloves,
    num_book, num_boots, num_bow, num_cloaks, num_cure_critical, num_cure_serious,
    num_duplicate_items, num_edged_weapon, num_ego, num_enchant_to_a, num_enchant_to_d,
    num_enchant_to_h, num_escape, num_ezheal, num_ffall, num_fix_exp, num_food, num_free_act,
    num_genocide, num_gloves, num_glyph, num_hats, num_heal, num_hold_life, num_ident,
    num_immune_acid, num_immune_cold, num_immune_elec, num_immune_fire, num_life, num_mana,
    num_mass_genocide, num_neck, num_pfe, num_phase, num_pot_rcold, num_pot_rheat, num_recall,
    num_recharge, num_regenerate, num_resist_LIGHT, num_resist_acid, num_resist_blind,
    num_resist_chaos, num_resist_cold, num_resist_conf, num_resist_dark, num_resist_disen,
    num_resist_elec, num_resist_fire, num_resist_neth, num_resist_nexus, num_resist_pois,
    num_resist_shard, num_resist_sound, num_rings, num_see_inv, num_shields, num_slow_digest,
    num_sustain_all, num_sustain_con, num_sustain_dex, num_sustain_int, num_sustain_str,
    num_sustain_wis, num_tele_staves, num_telepathy, num_teleport, num_weapons,
};
use crate::headers::borg_magic_h::borg_spell_stat;
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
pub use crate::headers::list_stats_h::{STAT_CON, STAT_DEX, STAT_INT, STAT_STR, STAT_WIS};
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
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, TV_MAX};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, kb_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23, STAT_MAX,
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
unsafe extern "C" fn borg_power_home_aux1() -> int32_t {
    let mut value: int32_t = 0 as int32_t;
    if num_LIGHT as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 150 as ::core::ffi::c_long) as int32_t;
    } else if num_LIGHT as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 170 as ::core::ffi::c_long) as int32_t;
    } else if num_LIGHT as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (170 as ::core::ffi::c_long
                + (num_LIGHT as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 5 as ::core::ffi::c_long)) as int32_t;
    }
    if num_slow_digest as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    } else if num_slow_digest as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 70 as ::core::ffi::c_long) as int32_t;
    } else if num_slow_digest as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (70 as ::core::ffi::c_long
                + (num_slow_digest as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 5 as ::core::ffi::c_long)) as int32_t;
    }
    if num_regenerate as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 75 as ::core::ffi::c_long) as int32_t;
    } else if num_regenerate as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    } else if num_regenerate as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (100 as ::core::ffi::c_long
                + (num_regenerate as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_telepathy as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_telepathy as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_telepathy as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_telepathy as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_see_inv as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 800 as ::core::ffi::c_long) as int32_t;
    } else if num_see_inv as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1200 as ::core::ffi::c_long) as int32_t;
    } else if num_see_inv as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1200 as ::core::ffi::c_long
                + (num_see_inv as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_ffall as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as int32_t;
    } else if num_ffall as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 15 as ::core::ffi::c_long) as int32_t;
    } else if num_ffall as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (15 as ::core::ffi::c_long
                + (num_ffall as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_free_act as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_free_act as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_free_act as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_free_act as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_hold_life as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_hold_life as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_hold_life as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_hold_life as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_acid as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_acid as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_acid as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_resist_acid as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_immune_acid as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_acid as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_acid as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (5000 as ::core::ffi::c_long
                + (num_immune_acid as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 30 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_elec as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_elec as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_elec as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_resist_elec as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_immune_elec as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_elec as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_elec as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (5000 as ::core::ffi::c_long
                + (num_immune_elec as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 30 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_fire as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_fire as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_fire as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_resist_fire as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_immune_fire as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_fire as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_fire as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (5000 as ::core::ffi::c_long
                + (num_immune_fire as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 30 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_cold as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_cold as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_cold as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_resist_cold as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_immune_cold as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_cold as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_immune_cold as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (5000 as ::core::ffi::c_long
                + (num_immune_cold as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 30 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_pois as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_pois as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 9000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_pois as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (9000 as ::core::ffi::c_long
                + (num_resist_pois as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 40 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_conf as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_conf as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_conf as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (8000 as ::core::ffi::c_long
                + (num_resist_conf as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 45 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_sound as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_sound as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 700 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_sound as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (700 as ::core::ffi::c_long
                + (num_resist_sound as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 30 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_LIGHT as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_LIGHT as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 150 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_LIGHT as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (150 as ::core::ffi::c_long
                + (num_resist_LIGHT as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_dark as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_dark as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 150 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_dark as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (150 as ::core::ffi::c_long
                + (num_resist_dark as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_chaos as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_chaos as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_chaos as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_resist_chaos as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 10 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_disen as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_disen as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 7000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_disen as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (7000 as ::core::ffi::c_long
                + (num_resist_disen as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 35 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_shard as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_shard as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 150 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_shard as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (150 as ::core::ffi::c_long
                + (num_resist_shard as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_nexus as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_nexus as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 300 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_nexus as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (300 as ::core::ffi::c_long
                + (num_resist_nexus as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 2 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_blind as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_blind as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_blind as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1000 as ::core::ffi::c_long
                + (num_resist_blind as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 5 as ::core::ffi::c_long)) as int32_t;
    }
    if num_resist_neth as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_neth as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
    } else if num_resist_neth as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (4000 as ::core::ffi::c_long
                + (num_resist_neth as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 45 as ::core::ffi::c_long)) as int32_t;
    }
    if (home_stat_add[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + home_stat_add[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                * 300 as ::core::ffi::c_long) as int32_t;
    } else if (home_stat_add[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 15 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (9 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + (home_stat_add[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 9 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 200 as ::core::ffi::c_long)) as int32_t;
    } else {
        value = (value as ::core::ffi::c_long
            + (9 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + 6 as ::core::ffi::c_long * 200 as ::core::ffi::c_long
                + (home_stat_add[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 15 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if (home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                * 300 as ::core::ffi::c_long) as int32_t;
    } else if (home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 15 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (9 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + (home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 9 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 200 as ::core::ffi::c_long)) as int32_t;
    } else {
        value = (value as ::core::ffi::c_long
            + (9 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + 6 as ::core::ffi::c_long * 200 as ::core::ffi::c_long
                + (home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 15 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if (home_stat_add[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 15 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + home_stat_add[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                * 300 as ::core::ffi::c_long) as int32_t;
    } else if (home_stat_add[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        < 21 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (15 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + (home_stat_add[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 15 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 200 as ::core::ffi::c_long)) as int32_t;
    } else {
        value = (value as ::core::ffi::c_long
            + (15 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                + 6 as ::core::ffi::c_long * 200 as ::core::ffi::c_long
                + (home_stat_add[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 21 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        if (home_stat_add[spell_stat as usize] as ::core::ffi::c_int) < 20 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                + home_stat_add[spell_stat as usize] as ::core::ffi::c_long
                    * 400 as ::core::ffi::c_long) as int32_t;
        } else if (home_stat_add[spell_stat as usize] as ::core::ffi::c_int)
            < 26 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (20 as ::core::ffi::c_long * 400 as ::core::ffi::c_long
                    + (home_stat_add[spell_stat as usize] as ::core::ffi::c_int
                        - 20 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 300 as ::core::ffi::c_long)) as int32_t;
        } else {
            value = (value as ::core::ffi::c_long
                + (20 as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                    + 6 as ::core::ffi::c_long * 300 as ::core::ffi::c_long
                    + (home_stat_add[spell_stat as usize] as ::core::ffi::c_int
                        - 26 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 5 as ::core::ffi::c_long)) as int32_t;
        }
    }
    if num_sustain_str as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_str as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_str as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (250 as ::core::ffi::c_long
                + (num_sustain_str as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_sustain_int as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_int as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_int as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (250 as ::core::ffi::c_long
                + (num_sustain_int as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_sustain_wis as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_wis as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_wis as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (250 as ::core::ffi::c_long
                + (num_sustain_wis as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_sustain_con as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_con as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_con as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (250 as ::core::ffi::c_long
                + (num_sustain_con as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_sustain_dex as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_dex as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_dex as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (250 as ::core::ffi::c_long
                + (num_sustain_dex as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_sustain_all as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_all as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_sustain_all as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long
                + (num_sustain_all as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 1 as ::core::ffi::c_long)) as int32_t;
    }
    if num_weapons as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_weapons as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 2000 as ::core::ffi::c_long) as int32_t;
    } else if num_weapons as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_weapons as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 100 as ::core::ffi::c_long) as int32_t;
    }
    if num_bow as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_bow as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_rings as ::core::ffi::c_int > 6 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_rings as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 4000 as ::core::ffi::c_long) as int32_t;
    } else if num_rings as ::core::ffi::c_int > 4 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_rings as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 2000 as ::core::ffi::c_long) as int32_t;
    }
    if num_neck as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_neck as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1500 as ::core::ffi::c_long) as int32_t;
    } else if num_neck as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_neck as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 700 as ::core::ffi::c_long) as int32_t;
    }
    if num_armor as ::core::ffi::c_int > 6 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_armor as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_cloaks as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_cloaks as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_shields as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_shields as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_hats as ::core::ffi::c_int > 4 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_hats as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_gloves as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_gloves as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    if num_boots as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            - (num_boots as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as int32_t;
    }
    value = (value as ::core::ffi::c_int + home_damage as ::core::ffi::c_int) as int32_t;
    value = (value as ::core::ffi::c_long
        - num_edged_weapon as ::core::ffi::c_long * 50 as ::core::ffi::c_long)
        as int32_t;
    value = (value as ::core::ffi::c_long
        - num_bad_gloves as ::core::ffi::c_long * 3000 as ::core::ffi::c_long)
        as int32_t;
    value = (value as ::core::ffi::c_long
        - num_duplicate_items as ::core::ffi::c_long * 50000 as ::core::ffi::c_long)
        as int32_t;
    return value;
}
unsafe extern "C" fn borg_power_home_aux2() -> int32_t {
    let mut k: ::core::ffi::c_int = 0;
    let mut book: ::core::ffi::c_int = 0;
    let mut value: int32_t = 0 as int32_t;
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_FOOD as ::core::ffi::c_int as isize)).max_stack
            && k < num_food as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (8000 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
        && k < num_ident as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (2000 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 45 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            && k < num_enchant_to_a as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (500 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 45 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            && k < num_enchant_to_h as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (500 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 45 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            && k < num_enchant_to_d as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (500 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
        && k < num_pfe as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (500 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
        && k < num_glyph as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (500 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k
        < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            * 2 as ::core::ffi::c_int
        && k < num_genocide as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (500 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k
        < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            * 2 as ::core::ffi::c_int
        && k < num_mass_genocide as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
        && k < num_recharge as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (500 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            > 20 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 80 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_POTION as ::core::ffi::c_int as isize)).max_stack
            && k < num_pot_rheat as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (100 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
        while k < (*kb_info.offset(TV_POTION as ::core::ffi::c_int as isize)).max_stack
            && k < num_pot_rcold as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (100 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 5 as ::core::ffi::c_int && k < num_recall as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 85 as ::core::ffi::c_int && k < num_escape as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + (2000 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    k = (*kb_info.offset(TV_STAFF as ::core::ffi::c_int as isize)).max_stack;
    while k < num_tele_staves as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long - 50000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 85 as ::core::ffi::c_int && k < num_teleport as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_SCROLL as ::core::ffi::c_int as isize)).max_stack
            && k < num_phase as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 1 as ::core::ffi::c_int {
        k = 0 as ::core::ffi::c_int;
        while k < (*kb_info.offset(TV_POTION as ::core::ffi::c_int as isize)).max_stack
            && k < num_mana as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (6000 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 8 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        == 1 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int && k < num_heal as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long - 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*kb_info.offset(TV_POTION as ::core::ffi::c_int as isize)).max_stack
        && k < num_cure_critical as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (1500 as ::core::ffi::c_long - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
            as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 90 as ::core::ffi::c_int && k < num_heal as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 198 as ::core::ffi::c_int && k < num_ezheal as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 198 as ::core::ffi::c_int && k < num_life as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long + 9000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 35 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 90 as ::core::ffi::c_int && k < num_cure_serious as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - (1500 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        == 50 as ::core::ffi::c_int
        && num_fix_exp as ::core::ffi::c_int != 0
    {
        value = (value as ::core::ffi::c_long - 7500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 35 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            <= 49 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 70 as ::core::ffi::c_int && k < num_fix_exp as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                + (1000 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    } else if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        <= 35 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int && k < num_fix_exp as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                + (1000 as ::core::ffi::c_long
                    - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    book = 0 as ::core::ffi::c_int;
    while book < 9 as ::core::ffi::c_int {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
        {
            k = 0 as ::core::ffi::c_int;
            while k < 5 as ::core::ffi::c_int && k < num_book[book as usize] as ::core::ffi::c_int {
                if num_book[book as usize] != 0 {
                    value = (value as ::core::ffi::c_long
                        + (5000 as ::core::ffi::c_long
                            - k as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
                        as int32_t;
                }
                k += 1;
            }
        }
        book += 1;
    }
    value = (value as ::core::ffi::c_long
        + num_artifact as ::core::ffi::c_long * 500 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + num_ego as ::core::ffi::c_long * 5000 as ::core::ffi::c_long) as int32_t;
    if home_un_id != 0 {
        value = (value as ::core::ffi::c_long
            + (home_un_id as ::core::ffi::c_int
                - crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize])
                as ::core::ffi::c_long
                * 1005 as ::core::ffi::c_long) as int32_t;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn borg_power_home() -> int32_t {
    let mut value: int32_t = 0 as int32_t;
    value += borg_power_home_aux1();
    value += borg_power_home_aux2();
    return value;
}
