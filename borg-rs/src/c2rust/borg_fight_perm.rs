pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::borg_note;
use crate::headers::borg_item_activation_h::{act_rage_bless_resist, act_resist_all};
use crate::headers::borg_item_use_h::{borg_activate_item, borg_equips_item, borg_quaff_potion};
use crate::headers::borg_item_val_h::sv_potion_berserk;
pub use crate::headers::borg_magic_h::{
    borg_get_spell_power, borg_heroism_level, borg_spell, borg_spell_fail, borg_spell_okay_fail,
    borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM,
    BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT,
    CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH,
    CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE,
    DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT,
    DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING,
    ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL,
    FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE,
    GRONDS_BLOW, HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD,
    HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE,
    LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM,
    MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION,
    ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE,
    RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE,
    REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT,
    RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT,
    SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT,
    SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE,
    TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR,
    TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM,
    VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION,
    WORD_OF_RECALL, ZONE_OF_UNMAGIC,
};
pub use crate::headers::borg_trait_h::{
    borg_simulate, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
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
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub const BP_SEE_INV: C2RustUnnamed_9 = 14;
pub const BP_GLYPH: C2RustUnnamed_9 = 13;
pub const BP_REGEN: C2RustUnnamed_9 = 12;
pub const BP_VENOM: C2RustUnnamed_9 = 11;
pub const BP_SMITE_EVIL: C2RustUnnamed_9 = 10;
pub const BP_BERSERK_POTION: C2RustUnnamed_9 = 9;
pub const BP_BERSERK: C2RustUnnamed_9 = 8;
pub const BP_HERO: C2RustUnnamed_9 = 7;
pub const BP_FASTCAST: C2RustUnnamed_9 = 6;
pub const BP_BLESS: C2RustUnnamed_9 = 2;
pub const BP_RESIST_P: C2RustUnnamed_9 = 5;
pub const BP_RESIST_ALL_COLLUIN: C2RustUnnamed_9 = 4;
pub const BP_RESIST_ALL: C2RustUnnamed_9 = 3;
pub const BP_PROT_FROM_EVIL: C2RustUnnamed_9 = 1;
pub const BP_SPEED: C2RustUnnamed_9 = 0;
pub const BP_MAX: C2RustUnnamed_9 = 15;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
unsafe extern "C" fn borg_perma_aux_bless() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 20 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 25 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.bless {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BLESS, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(BLESS);
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 10 as ::core::ffi::c_int
        && cost
            >= (if crate::bridge::st().borg.mon.unique != 0 {
                crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                    / 7 as ::core::ffi::c_int
            } else {
                crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                    / 10 as ::core::ffi::c_int
            })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    borg_spell(BLESS);
    crate::bridge::st().borg.no_rest_prep = 10000 as int16_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.res_fire as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_acid as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_elec as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_cold as ::core::ffi::c_int
        >= 3 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RESISTANCE);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    borg_spell_fail(RESISTANCE, fail_allowed);
    crate::bridge::st().borg.no_rest_prep = 21000 as int16_t;
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist_colluin() -> ::core::ffi::c_int {
    if crate::bridge::st().borg.temp.res_fire as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_acid as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_pois as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_elec as ::core::ffi::c_int
        + crate::bridge::st().borg.temp.res_cold as ::core::ffi::c_int
        >= 3 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
        || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.no_rest_prep = 21000 as int16_t;
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist_p() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.res_pois as ::core::ffi::c_int != 0 || crate::bridge::st().borg.mon.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESIST_POISON, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RESIST_POISON);
    if cost
        >= crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 20 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    if borg_spell_fail(RESIST_POISON, fail_allowed) {
        crate::bridge::st().borg.no_rest_prep = 21000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_speed() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.fast {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(HASTE_SELF, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(HASTE_SELF);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 5 as ::core::ffi::c_int;
    }
    if borg_spell_fail(HASTE_SELF, fail_allowed) {
        crate::bridge::st().borg.no_rest_prep = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            * 1000 as ::core::ffi::c_int) as int16_t;
        return 5 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_prot_evil() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if crate::bridge::st().borg.temp.prot_from_evil {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(PROTECTION_FROM_EVIL);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        crate::bridge::st().borg.no_rest_prep = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            * 1000 as ::core::ffi::c_int) as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_fastcast() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.fastcast {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(MANA_CHANNEL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(MANA_CHANNEL);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 5 as ::core::ffi::c_int;
    }
    if borg_spell(MANA_CHANNEL) {
        crate::bridge::st().borg.no_rest_prep = 6000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_hero() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.hero {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        <= borg_heroism_level()
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(HEROISM, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(HEROISM);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    if borg_spell(HEROISM) {
        crate::bridge::st().borg.no_rest_prep = 3000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_regen() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.regen {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISFORGET as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] < 100 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RAPID_REGENERATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RAPID_REGENERATION);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell(RAPID_REGENERATION) {
        crate::bridge::st().borg.no_rest_prep = 6000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_smite_evil() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.smite_evil as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SMITE_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(SMITE_EVIL);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell(SMITE_EVIL) {
        crate::bridge::st().borg.no_rest_prep = 21000 as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_venom() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.venom as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_WB_POIS as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(VENOM, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(SMITE_EVIL);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell(VENOM) {
        crate::bridge::st().borg.no_rest_prep = 19000 as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_berserk() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.berserk {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BERSERK_STRENGTH, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(BERSERK_STRENGTH);
    if cost
        >= (if crate::bridge::st().borg.mon.unique != 0 {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 7 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_spell(BERSERK_STRENGTH) {
        crate::bridge::st().borg.no_rest_prep = 11000 as int16_t;
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_berserk_potion() -> ::core::ffi::c_int {
    if crate::bridge::st().borg.near.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.temp.hero as ::core::ffi::c_int != 0 || crate::bridge::st().borg.temp.berserk as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if -(1 as ::core::ffi::c_int) == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_berserk) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_quaff_potion(sv_potion_berserk) {
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_see_inv() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.see_inv as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SENSE_INVISIBLE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(SENSE_INVISIBLE, fail_allowed) {
        crate::bridge::st().borg.see_inv = 32000 as int16_t;
        crate::bridge::st().borg.no_rest_prep = 16000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux(mut what: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match what {
        0 => return borg_perma_aux_speed(),
        1 => return borg_perma_aux_prot_evil(),
        3 => return borg_perma_aux_resist(),
        4 => return borg_perma_aux_resist_colluin(),
        5 => return borg_perma_aux_resist_p(),
        2 => return borg_perma_aux_bless(),
        6 => return borg_perma_aux_fastcast(),
        7 => return borg_perma_aux_hero(),
        8 => return borg_perma_aux_berserk(),
        9 => return borg_perma_aux_berserk_potion(),
        10 => return borg_perma_aux_smite_evil(),
        11 => return borg_perma_aux_venom(),
        12 => return borg_perma_aux_regen(),
        13 => return 0 as ::core::ffi::c_int,
        14 => return borg_perma_aux_see_inv(),
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_perma_spell() -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    borg_simulate = true_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            / 3 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            < 7 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        <= 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] * 75 as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    g = 0 as ::core::ffi::c_int;
    while g < BP_MAX as ::core::ffi::c_int {
        n = borg_perma_aux(g);
        if !(n <= b_n) {
            b_g = g;
            b_n = n;
        }
        g += 1;
    }
    if b_n <= 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing perma-spell type %d with value %d\0" as *const u8
            as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_perma_aux(b_g);
    return true_0 != 0;
}
