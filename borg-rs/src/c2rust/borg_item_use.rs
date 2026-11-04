pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{
    act_food_waybread, act_recharge, act_restore_all, act_restore_con, act_restore_dex,
    act_restore_exp, act_restore_int, act_restore_life, act_restore_st_lev, act_restore_str,
    act_restore_wis, act_shroom_purging,
};
use crate::headers::borg_item_analyze_h::borg_obj_has_effect;
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_val_h::{
    sv_food_apple, sv_food_draught, sv_food_handful, sv_food_honey_cake, sv_food_pint,
    sv_food_ration, sv_food_slice, sv_food_slime_mold, sv_food_waybread, sv_mush_purging,
    sv_mush_restoring, sv_potion_blindness, sv_potion_confusion, sv_potion_cure_critical,
    sv_potion_cure_light, sv_potion_cure_serious, sv_potion_enlightenment, sv_potion_healing,
    sv_potion_inc_all, sv_potion_inc_con, sv_potion_inc_dex, sv_potion_inc_int, sv_potion_inc_str,
    sv_potion_inc_wis, sv_potion_restore_life, sv_potion_sleep, sv_potion_slime_mold,
    sv_potion_slowness, sv_rod_recall, sv_scroll_acquirement, sv_scroll_mapping,
    sv_scroll_recharging, sv_scroll_star_acquirement, sv_staff_destruction, sv_staff_power,
    sv_staff_teleportation,
};
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_fail, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE,
    BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN,
    BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND,
    CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE,
    DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS,
    DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL,
    DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC,
    ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW,
    FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW,
    HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED,
    ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM,
    MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP,
    METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR,
    PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS,
    RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE,
    RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE,
    SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT,
    SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD,
    STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER,
    TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD,
    UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION,
    WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL,
    ZONE_OF_UNMAGIC,
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, no_light, square};
pub use crate::headers::effects_h::{C2RustUnnamedH_45, EF_MAX, EF_NONE};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_effects_h::{
    EF_ACQUIRE, EF_ALTER, EF_ALTER_REALITY, EF_ARC, EF_BALL, EF_BANISH, EF_BEAM, EF_BIZARRE,
    EF_BOLT, EF_BOLT_AWARE, EF_BOLT_OR_BEAM, EF_BOLT_STATUS, EF_BOLT_STATUS_DAM, EF_BRAND_AMMO,
    EF_BRAND_BOLTS, EF_BRAND_WEAPON, EF_BREATH, EF_CLEAR_VALUE, EF_COMMAND, EF_CREATE_ARROWS,
    EF_CREATE_STAIRS, EF_CRUNCH, EF_CURE, EF_CURSE, EF_CURSE_ARMOR, EF_CURSE_WEAPON, EF_DAMAGE,
    EF_DARKEN_AREA, EF_DARKEN_LEVEL, EF_DEEP_DESCENT, EF_DESTRUCTION, EF_DETECT_DOORS,
    EF_DETECT_EVIL, EF_DETECT_FEARFUL_MONSTERS, EF_DETECT_GOLD, EF_DETECT_INVISIBLE_MONSTERS,
    EF_DETECT_LIVING_MONSTERS, EF_DETECT_OBJECTS, EF_DETECT_ORE, EF_DETECT_SOUL, EF_DETECT_STAIRS,
    EF_DETECT_TRAPS, EF_DETECT_VISIBLE_MONSTERS, EF_DISENCHANT, EF_DRAIN_LIGHT, EF_DRAIN_MANA,
    EF_DRAIN_STAT, EF_EARTHQUAKE, EF_ENCHANT, EF_GAIN_EXP, EF_GAIN_STAT, EF_GLYPH, EF_GRANITE,
    EF_HEAL_HP, EF_IDENTIFY, EF_JUMP_AND_BITE, EF_LASH, EF_LIGHT_AREA, EF_LIGHT_LEVEL, EF_LINE,
    EF_LOSE_RANDOM_STAT, EF_MAP_AREA, EF_MASS_BANISH, EF_MELEE_BLOWS, EF_MON_HEAL_HP,
    EF_MON_HEAL_KIN, EF_MON_TIMED_INC, EF_MOVE_ATTACK, EF_NOURISH, EF_PROBE, EF_PROJECT_LOS,
    EF_PROJECT_LOS_AWARE, EF_RANDOM, EF_READ_MINDS, EF_RECALL, EF_RECHARGE, EF_REMOVE_CURSE,
    EF_RESTORE_EXP, EF_RESTORE_MANA, EF_RESTORE_STAT, EF_RUBBLE, EF_SCRAMBLE_STATS, EF_SELECT,
    EF_SENSE_GOLD, EF_SENSE_OBJECTS, EF_SET_VALUE, EF_SHAPECHANGE, EF_SHORT_BEAM, EF_SINGLE_COMBAT,
    EF_SPHERE, EF_SPOT, EF_STAR, EF_STAR_BALL, EF_STRIKE, EF_SUMMON, EF_SWARM, EF_SWEEP,
    EF_TAP_DEVICE, EF_TAP_UNLIFE, EF_TELEPORT, EF_TELEPORT_LEVEL, EF_TELEPORT_TO, EF_TIMED_DEC,
    EF_TIMED_INC, EF_TIMED_INC_NO_RES, EF_TIMED_SET, EF_TOUCH, EF_TOUCH_AWARE, EF_UNSCRAMBLE_STATS,
    EF_WAKE, EF_WEB, EF_WONDER,
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
use crate::headers::source_h::trap;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_quaff_crit(mut no_check: bool) -> bool {
    static mut when_last_quaff: borg_time = 0 as borg_time;
    if no_check {
        if borg_quaff_potion(sv_potion_cure_critical) {
            when_last_quaff = borg.time.now;
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    if borg.time.now - when_last_quaff > 4 as borg_time
        && borg.time.now - when_last_quaff > 0 as borg_time
        && (Rand_div(100 as uint32_t) as int32_t + 1 as int32_t) < 75 as int32_t
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize) < 2 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_quaff_potion(sv_potion_cure_critical) {
        when_last_quaff = borg.time.now;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_quaff_potion(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(TV_POTION as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Quaffing %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('q' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_quaff_unknown() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).tval as ::core::ffi::c_int != TV_POTION as ::core::ffi::c_int) {
                if !(*item).aware {
                    n = i;
                }
            }
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Quaffing unknown potion %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(n as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('q' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(n as isize)
            as keycode_t,
    );
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    borg.trying_unknown = true_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_read_scroll(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if no_light(player) {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    i = borg_slot(TV_SCROLL as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Reading %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress('r' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_read_unknown() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).tval as ::core::ffi::c_int != TV_SCROLL as ::core::ffi::c_int) {
                if !(*item).aware {
                    n = i;
                }
            }
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if no_light(player) {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Reading unknown scroll %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(n as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('r' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(n as isize)
            as keycode_t,
    );
    borg_keypress(ESCAPE as keycode_t);
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    borg.trying_unknown = true_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_eat(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(tval, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Eating %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('E' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_eat_unknown() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).tval as ::core::ffi::c_int != TV_FOOD as ::core::ffi::c_int
                && (*item).tval as ::core::ffi::c_int != TV_MUSHROOM as ::core::ffi::c_int)
            {
                if !(*item).aware {
                    n = i;
                }
            }
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Eating unknown mushroom %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(n as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('E' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(n as isize)
            as keycode_t,
    );
    borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    borg.goal.ware = borg.goal.item;
    borg.goal.shop = borg.goal.ware;
    borg.trying_unknown = true_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_eat_food_any() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !((*item).tval as ::core::ffi::c_int != TV_FOOD as ::core::ffi::c_int) {
                    if borg_eat(
                        (*item).tval as ::core::ffi::c_int,
                        (*item).sval as ::core::ffi::c_int,
                    ) {
                        return true_0 != 0;
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item_0: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item_0).iqty == 0) {
            if (*item_0).aware {
                if !((*item_0).tval as ::core::ffi::c_int != TV_FOOD as ::core::ffi::c_int
                    && (*item_0).tval as ::core::ffi::c_int != TV_MUSHROOM as ::core::ffi::c_int)
                {
                    if !(!borg_obj_has_effect(
                        (*item_0).kind,
                        EF_NOURISH as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ) && !borg_obj_has_effect(
                        (*item_0).kind,
                        EF_NOURISH as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    ) && !borg_obj_has_effect(
                        (*item_0).kind,
                        EF_NOURISH as ::core::ffi::c_int,
                        3 as ::core::ffi::c_int,
                    )) {
                        if borg_eat(
                            (*item_0).tval as ::core::ffi::c_int,
                            (*item_0).sval as ::core::ffi::c_int,
                        ) {
                            return true_0 != 0;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_quaff_potion(sv_potion_slime_mold) {
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0
        && (borg_quaff_potion(sv_potion_sleep) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_slowness) as ::core::ffi::c_int != 0)
        || *borg
            .trait_0
            .offset(BI_RBLIND as ::core::ffi::c_int as isize)
            != 0
            && borg_quaff_potion(sv_potion_blindness) as ::core::ffi::c_int != 0
        || *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0
            && borg_quaff_potion(sv_potion_confusion) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if (*borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 4 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            <= *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize))
        && (borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_critical) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0)
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_equips_rod(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    let mut lev: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    i = borg_slot(TV_ROD as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    fail = 100 as ::core::ffi::c_int
        * (skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
    if fail > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_zap_rod(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut lev: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    i = borg_slot(TV_ROD as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    fail = 100 as ::core::ffi::c_int
        * (skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
    if sval != sv_rod_recall {
        if fail > 500 as ::core::ffi::c_int {
            return false_0 != 0;
        }
    }
    borg_note(format(
        b"# Zapping %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('z' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_use_staff(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(TV_STAFF as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Using %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('u' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_use_unknown() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).tval as ::core::ffi::c_int != TV_STAFF as ::core::ffi::c_int) {
                if !(*item).aware {
                    n = i;
                }
            }
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Using unknown Staff %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(n as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('u' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(n as isize)
            as keycode_t,
    );
    borg_keypress(ESCAPE as keycode_t);
    borg.trying_unknown = true_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_use_staff_fail(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut lev: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    i = borg_slot(TV_STAFF as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    fail = 100 as ::core::ffi::c_int
        * (skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
    if fail > 500 as ::core::ffi::c_int {
        if sval != sv_staff_teleportation {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
            && *borg
                .trait_0
                .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                == 0
        {
            if fail > 500 as ::core::ffi::c_int {
                return false_0 != 0;
            }
        }
    }
    borg_note(format(
        b"# Using %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('u' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_equips_staff_fail(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut lev: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    i = borg_slot(TV_STAFF as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    fail = 100 as ::core::ffi::c_int
        * (skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
    if sval == sv_staff_destruction {
        return true_0 != 0;
    }
    if fail > 500 as ::core::ffi::c_int {
        if sval != sv_staff_teleportation {
            return false_0 != 0;
        }
        if sval == sv_staff_teleportation
            && *borg
                .trait_0
                .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                == 0
        {
            if fail < 650 as ::core::ffi::c_int {
                return false_0 != 0;
            }
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_aim_wand(mut sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(TV_WAND as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Aiming %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('a' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_equips_ring(mut ring_sval: ::core::ffi::c_int) -> bool {
    let mut lev: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    i = (*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0 || !(*item).aware) {
            if !((*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int) {
                if !((*item).sval as ::core::ffi::c_int != ring_sval) {
                    if !((*item).timeout != 0) {
                        if (*item).ident {
                            lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
                            skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
                            if *borg
                                .trait_0
                                .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                                != 0
                            {
                                skill =
                                    skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                            }
                            fail = 100 as ::core::ffi::c_int
                                * (skill
                                    - lev
                                    - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                                / (lev
                                    - skill
                                    - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
                            if !(fail > 500 as ::core::ffi::c_int) {
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
pub unsafe extern "C" fn borg_activate_ring(mut ring_sval: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = (*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item).aware {
            if !((*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int) {
                if !((*item).sval as ::core::ffi::c_int != ring_sval) {
                    if !((*item).timeout != 0) {
                        if (*item).ident {
                            borg_note(format(
                                b"# Activating ring %s.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &raw mut (*item).desc as *mut ::core::ffi::c_char,
                            ));
                            borg_keypress('A' as i32 as keycode_t);
                            borg_keypress(
                                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                                    .offset(
                                        (i - (*z_info).pack_size as ::core::ffi::c_int) as isize,
                                    ) as keycode_t,
                            );
                            return true_0 != 0;
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
pub unsafe extern "C" fn borg_equips_dragon(mut drag_sval: ::core::ffi::c_int) -> bool {
    let mut lev: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    let mut numerator: ::core::ffi::c_int = 0;
    let mut denominator: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if (*item).iqty == 0 || !(*item).aware {
        return false_0 != 0;
    }
    if (*item).tval as ::core::ffi::c_int != TV_DRAG_ARMOR as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*item).sval as ::core::ffi::c_int != drag_sval {
        return false_0 != 0;
    }
    if (*item).timeout != 0 {
        return false_0 != 0;
    }
    if !(*item).ident {
        return false_0 != 0;
    }
    lev = (*item).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    numerator = skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    denominator = lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int);
    if denominator == 0 as ::core::ffi::c_int {
        denominator = if numerator > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    }
    fail = 100 as ::core::ffi::c_int * numerator / denominator;
    if fail > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_activate_dragon(mut drag_sval: ::core::ffi::c_int) -> bool {
    let mut item: *mut borg_item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if (*item).iqty == 0 || !(*item).aware {
        return false_0 != 0;
    }
    if (*item).tval as ::core::ffi::c_int != TV_DRAG_ARMOR as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*item).sval as ::core::ffi::c_int != drag_sval {
        return false_0 != 0;
    }
    if (*item).timeout != 0 {
        return false_0 != 0;
    }
    if !(*item).ident {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Activating dragon scale %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*item).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('A' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(
            ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int
                - (*z_info).pack_size as ::core::ffi::c_int) as isize,
        ) as keycode_t,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_activate_item(mut activation: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if *borg.activation.offset(activation as isize) == 0 {
        return false_0 != 0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).activ_idx != activation) {
                if !((*item).timeout != 0) {
                    borg_note(format(
                        b"# Activating item %s.\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut (*item).desc as *mut ::core::ffi::c_char,
                    ));
                    borg_keypress('A' as i32 as keycode_t);
                    borg_keypress(
                        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                            .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                            as keycode_t,
                    );
                    return true_0 != 0;
                }
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_equips_item(
    mut activation: ::core::ffi::c_int,
    mut check_charge: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if *borg.activation.offset(activation as isize) == 0 {
        return false_0 != 0;
    } else if !check_charge {
        return true_0 != 0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).activ_idx != activation) {
                if !(check_charge as ::core::ffi::c_int != 0
                    && (*item).timeout as ::core::ffi::c_int >= 1 as ::core::ffi::c_int)
                {
                    return true_0 != 0;
                }
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_activate_failure(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lev: ::core::ffi::c_int = 0;
    let mut skill: ::core::ffi::c_int = 0;
    let mut fail: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(tval, sval);
    if i < 0 as ::core::ffi::c_int {
        return 100 as ::core::ffi::c_int;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return 100 as ::core::ffi::c_int;
    }
    if (*borg_items.offset(i as isize)).activ_idx == 0 {
        return 100 as ::core::ffi::c_int;
    }
    lev = (*borg_items.offset(i as isize)).level as ::core::ffi::c_int;
    skill = *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        skill = skill * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    }
    fail = 100 as ::core::ffi::c_int
        * (skill - lev - (141 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (lev - skill - (100 as ::core::ffi::c_int - 10 as ::core::ffi::c_int));
    return fail;
}
#[no_mangle]
pub unsafe extern "C" fn borg_use_things() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if *borg
        .trait_0
        .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize)
        != 0
        && (borg_spell(REVITALIZE) as ::core::ffi::c_int != 0
            || borg_spell(REMEMBRANCE) as ::core::ffi::c_int != 0
            || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                > 90 as ::core::ffi::c_int
                && borg_spell(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_exp) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_st_lev) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_life) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_restore_life) as ::core::ffi::c_int != 0)
    {
        return true_0 != 0;
    }
    if borg_quaff_potion(sv_potion_inc_str) as ::core::ffi::c_int != 0
        || borg_quaff_potion(sv_potion_inc_int) as ::core::ffi::c_int != 0
        || borg_quaff_potion(sv_potion_inc_wis) as ::core::ffi::c_int != 0
        || borg_quaff_potion(sv_potion_inc_dex) as ::core::ffi::c_int != 0
        || borg_quaff_potion(sv_potion_inc_con) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISFIXSTR as ::core::ffi::c_int as isize)
        != 0
        && (borg_quaff_potion(sv_potion_inc_str) as ::core::ffi::c_int != 0
            || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_purging) as ::core::ffi::c_int
                != 0
            || borg_activate_item(act_shroom_purging) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_str) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_all) as ::core::ffi::c_int != 0
            || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring) as ::core::ffi::c_int
                != 0)
        || *borg
            .trait_0
            .offset(BI_ISFIXINT as ::core::ffi::c_int as isize)
            != 0
            && (borg_quaff_potion(sv_potion_inc_int) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_int) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_all) as ::core::ffi::c_int != 0
                || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring)
                    as ::core::ffi::c_int
                    != 0)
        || *borg
            .trait_0
            .offset(BI_ISFIXWIS as ::core::ffi::c_int as isize)
            != 0
            && (borg_quaff_potion(sv_potion_inc_wis) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_wis) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_all) as ::core::ffi::c_int != 0
                || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring)
                    as ::core::ffi::c_int
                    != 0)
        || *borg
            .trait_0
            .offset(BI_ISFIXDEX as ::core::ffi::c_int as isize)
            != 0
            && (borg_quaff_potion(sv_potion_inc_dex) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_dex) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_all) as ::core::ffi::c_int != 0
                || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring)
                    as ::core::ffi::c_int
                    != 0)
        || *borg
            .trait_0
            .offset(BI_ISFIXCON as ::core::ffi::c_int as isize)
            != 0
            && (borg_quaff_potion(sv_potion_inc_con) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_con) as ::core::ffi::c_int != 0
                || borg_activate_item(act_restore_all) as ::core::ffi::c_int != 0
                || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_purging)
                    as ::core::ffi::c_int
                    != 0
                || borg_activate_item(act_shroom_purging) as ::core::ffi::c_int != 0
                || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring)
                    as ::core::ffi::c_int
                    != 0)
    {
        return true_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0 || !(*item).aware) {
            match (*item).tval as ::core::ffi::c_int {
                26 => {
                    if (*item).sval as ::core::ffi::c_int == sv_potion_enlightenment {
                        *borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            == 0;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_all {
                        if borg_quaff_potion((*item).sval as ::core::ffi::c_int) {
                            return true_0 != 0;
                        }
                    }
                }
                25 => {
                    if !(*borg
                        .trait_0
                        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                        != 0
                        || *borg
                            .trait_0
                            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                            != 0)
                    {
                        if (*item).sval as ::core::ffi::c_int == sv_scroll_mapping
                            || (*item).sval as ::core::ffi::c_int == sv_scroll_acquirement
                            || (*item).sval as ::core::ffi::c_int == sv_scroll_star_acquirement
                        {
                            if !(*borg
                                .trait_0
                                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                == 0)
                            {
                                if borg_read_scroll((*item).sval as ::core::ffi::c_int) {
                                    return true_0 != 0;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
    {
        if borg_spell(REMOVE_HUNGER) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_slime_mold) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_slime_mold) as ::core::ffi::c_int
                != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_slice) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_apple) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_pint) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_handful) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_honey_cake) as ::core::ffi::c_int
                != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_ration) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_waybread) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_draught) as ::core::ffi::c_int != 0
            || borg_activate_item(act_food_waybread) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_recharging() -> bool {
    let mut i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut charge: bool = false_0 != 0;
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !(!(*item).ident || !(*item).aware) {
                charge = false_0 != 0;
                if (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                    && (*item).pval as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
                {
                    charge = true_0 != 0;
                }
                if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int {
                    if ((*item).pval as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                        && borg_spell_okay_fail(RECHARGING, 96 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            != 0
                        && ((*item).sval as ::core::ffi::c_int) < sv_staff_power
                    {
                        charge = true_0 != 0;
                    }
                    if (*item).pval as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                        charge = true_0 != 0;
                    }
                    if (*item).sval as ::core::ffi::c_int == sv_staff_teleportation
                        && ((*item).pval as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            == 0
                    {
                        charge = true_0 != 0;
                    }
                    if (*item).iqty as ::core::ffi::c_int + (*item).pval as ::core::ffi::c_int
                        >= 4 as ::core::ffi::c_int
                        && (*item).pval as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                    {
                        charge = false_0 != 0;
                    }
                }
                if charge {
                    if !(borg_read_scroll(sv_scroll_recharging) as ::core::ffi::c_int != 0
                        || borg_spell_fail(RECHARGING, 96 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            != 0
                        || borg_activate_item(act_recharge) as ::core::ffi::c_int != 0)
                    {
                        break;
                    }
                    borg_note(format(
                        b"Recharging %s with current charge of %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut (*item).desc as *mut ::core::ffi::c_char,
                        (*item).pval as ::core::ffi::c_int,
                    ));
                    borg_keypress(
                        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                            .offset(i as isize) as keycode_t,
                    );
                    return true_0 != 0;
                }
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
