pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const GOAL_RECOVER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_danger_h::{borg_danger, borg_fear_region};
use crate::headers::borg_flow_misc_h::borg_check_rest;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{
    act_cure_body, act_cure_confusion, act_cure_critical, act_cure_full, act_cure_full2,
    act_cure_light, act_cure_serious, act_cure_temp, act_food_waybread, act_heal1, act_heal2,
    act_heal3, act_rem_fear_pois, act_restore_exp, act_restore_life, act_restore_st_lev,
    act_satisfy,
};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_eat, borg_need, borg_quaff_crit, borg_quaff_potion, borg_read_scroll,
    borg_use_staff_fail, borg_zap_rod, BORG_MET_NEED, BORG_NO_NEED, BORG_UNMET_NEED,
};
use crate::headers::borg_item_val_h::{
    kv_rod_healing, kv_rod_recall, sv_food_waybread, sv_mush_cure_mind, sv_mush_fast_recovery,
    sv_potion_berserk, sv_potion_boldness, sv_potion_cure_light, sv_potion_cure_poison,
    sv_potion_cure_serious, sv_potion_heroism, sv_rod_curing, sv_rod_healing, sv_rod_recall,
    sv_scroll_satisfy_hunger, sv_staff_curing,
};
use crate::headers::borg_light_h::{borg_check_light_only, borg_maintain_light};
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_okay, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL,
    BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST,
    BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS,
    CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE,
    DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR,
    DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD,
    DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON,
    EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT,
    FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW, HASTE_SELF, HEALING,
    HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE,
    LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE,
    MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING,
    NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE,
    PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING,
    REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION,
    REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE,
    SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL,
    SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS,
    TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY,
    TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS,
    VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM,
    WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL, ZONE_OF_UNMAGIC,
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
    chunk, connector, heatmap, monster_group, square, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
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
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
#[no_mangle]
pub unsafe extern "C" fn borg_recover() -> bool {
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut q: ::core::ffi::c_int = 0;
    let mut need: borg_need = BORG_NO_NEED;
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(format(
            b"# Need to refuel but cant!\0" as *const u8 as *const ::core::ffi::c_char,
        ));
    }
    p = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    if p > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    q = Rand_div(100 as uint32_t) as int32_t as ::core::ffi::c_int;
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        q = q - 10 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 4 as ::core::ffi::c_int
    {
        q = q - 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
        != 0
        && q < 75 as ::core::ffi::c_int
    {
        if borg_activate_item(act_cure_body) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_critical) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_temp) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal3) as ::core::ffi::c_int != 0
            || borg_spell(MINOR_HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Cure Stun - danger %d\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            ));
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
        != 0
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery) as ::core::ffi::c_int
            != 0
            || borg_activate_item(act_cure_body) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_critical) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_temp) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal3) as ::core::ffi::c_int != 0
            || borg_spell(MINOR_HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Cure Heavy Stun - danger %d\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            ));
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        && q < 75 as ::core::ffi::c_int
    {
        if borg_activate_item(act_cure_light) as ::core::ffi::c_int != 0
            || borg_spell(MINOR_HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Cure Cuts - danger %d\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            ));
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
        != 0
        && q < 75 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery) as ::core::ffi::c_int
            != 0
            || borg_activate_item(act_rem_fear_pois) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_spell(CURE_POISON) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Cure poison - danger %d\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            ));
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
            == 0
        && q < 75 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rem_fear_pois) as ::core::ffi::c_int != 0
            || borg_spell(HEROISM) as ::core::ffi::c_int != 0
            || borg_spell(BERSERK_STRENGTH) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Cure fear - danger %d\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            ));
            return true_0 != 0;
        }
    }
    if (*borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0)
        && q < 75 as ::core::ffi::c_int
    {
        if borg_spell(REMOVE_HUNGER) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
        != 0
        && q < 75 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) {
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
        && q < 75 as ::core::ffi::c_int
        && p == 0 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
    {
        if borg_activate_item(act_heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal2) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal3) as ::core::ffi::c_int != 0
            || borg_spell(HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
            || borg_spell(MINOR_HEALING) as ::core::ffi::c_int != 0
            || borg_spell(HEROISM) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# heal damage (recovering)\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize)
        != 0
        && (borg_activate_item(act_restore_exp) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_st_lev) as ::core::ffi::c_int != 0
            || borg_activate_item(act_restore_life) as ::core::ffi::c_int != 0
            || borg_spell(REVITALIZE) as ::core::ffi::c_int != 0
            || borg_spell(REMEMBRANCE) as ::core::ffi::c_int != 0
            || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                > 90 as ::core::ffi::c_int
                && borg_spell(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0)
    {
        return true_0 != 0;
    }
    if (*borg
        .trait_0
        .offset(BI_ISFIXSTR as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXINT as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXWIS as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXDEX as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXCON as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXALL as ::core::ffi::c_int as isize)
            != 0)
        && (borg_spell(RESTORATION) as ::core::ffi::c_int != 0
            || borg_spell(REVITALIZE) as ::core::ffi::c_int != 0)
    {
        return true_0 != 0;
    }
    if (*borg
        .trait_0
        .offset(BI_ISFIXSTR as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXINT as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFIXCON as ::core::ffi::c_int as isize)
            != 0)
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) > 90 as ::core::ffi::c_int
        && borg_spell(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
        != 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal2) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
        != 0
        && q < 95 as ::core::ffi::c_int
    {
        if borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal2) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_heal2) as ::core::ffi::c_int != 0
            || borg_quaff_crit(
                *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                    < 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
        != 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery) as ::core::ffi::c_int
            != 0
            || borg_quaff_potion(sv_potion_cure_poison) as ::core::ffi::c_int != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_waybread) as ::core::ffi::c_int != 0
            || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery)
                as ::core::ffi::c_int
                != 0
            || borg_quaff_crit(
                *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                    < 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rem_fear_pois) as ::core::ffi::c_int != 0
            || borg_activate_item(act_food_waybread) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery) as ::core::ffi::c_int
            != 0
            || borg_eat(TV_FOOD as ::core::ffi::c_int, sv_food_waybread) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_food_waybread) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_confusion) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_curing) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
            == 0
        && q < 25 as ::core::ffi::c_int
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_boldness) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_heroism) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_berserk) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rem_fear_pois) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if (*borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0)
        && q < 25 as ::core::ffi::c_int
    {
        if borg_read_scroll(sv_scroll_satisfy_hunger) as ::core::ffi::c_int != 0
            || borg_activate_item(act_satisfy) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
        && q < 25 as ::core::ffi::c_int
    {
        if borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
            || borg_activate_item(act_cure_serious) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if *borg.has.offset(kv_rod_recall as isize) != 0
        || *borg.has.offset(kv_rod_healing as isize) != 0
    {
        if *borg.has.offset(kv_rod_healing as isize) != 0
            && (*borg_items
                .offset(borg_slot(TV_ROD as ::core::ffi::c_int, sv_rod_healing) as isize))
            .pval
                == 0
            || *borg.has.offset(kv_rod_recall as isize) != 0
                && (*borg_items
                    .offset(borg_slot(TV_ROD as ::core::ffi::c_int, sv_rod_recall) as isize))
                .pval
                    == 0
        {
            if *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                == 0
                && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
                && *borg
                    .trait_0
                    .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
                    == 0
                && *borg
                    .trait_0
                    .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
                    == 0
                && borg_check_rest(borg.c.y, borg.c.x) as ::core::ffi::c_int != 0
                && !borg_spell_okay(RECHARGING)
            {
                borg_note(
                    b"# Resting to recharge a rod...\0" as *const u8 as *const ::core::ffi::c_char,
                );
                borg.antibounce_count = 0 as int16_t;
                borg_keypress('R' as i32 as keycode_t);
                borg_keypress('1' as i32 as keycode_t);
                borg_keypress('0' as i32 as keycode_t);
                borg_keypress('0' as i32 as keycode_t);
                borg_keypress(KC_ENTER as keycode_t);
                borg.in_shop = false_0 != 0;
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        == 0
        && *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            == 0
        && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
        && *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
            == 0
        && (*borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
            || *borg
                .trait_0
                .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                != 0
            || *borg
                .trait_0
                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                != 0
            || *borg
                .trait_0
                .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                != 0
            || *borg
                .trait_0
                .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                != 0
            || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
            || *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                    * (if *borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                        > 85 as ::core::ffi::c_int
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    })
                    / 10 as ::core::ffi::c_int)
    {
        if borg_check_rest(borg.c.y, borg.c.x) as ::core::ffi::c_int != 0
            && !borg.mon.scary
            && p <= borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
                [(borg.c.x / 11 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
            && borg.goal.type_0 as ::core::ffi::c_int != GOAL_RECOVER
        {
            if !borg_check_light_only() {
                borg_note(format(
                    b"# Resting to recover HP/SP...\0" as *const u8 as *const ::core::ffi::c_char,
                ));
                borg_keypress('R' as i32 as keycode_t);
                borg_keypress('&' as i32 as keycode_t);
                borg_keypress(KC_ENTER as keycode_t);
                borg.antibounce_count = 0 as int16_t;
                borg.need_see_invis = borg.time.now;
                return true_0 != 0;
            } else {
                borg_note(format(
                    b"# Lighted the darkened room instead of resting.\0" as *const u8
                        as *const ::core::ffi::c_char,
                ));
                return true_0 != 0;
            }
        }
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0
        && (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            <= 40 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 85 as ::core::ffi::c_int)
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                * 8 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        && p < borg.avoidance as ::core::ffi::c_int * 1 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
        && borg_check_rest(borg.c.y, borg.c.x) as ::core::ffi::c_int != 0
    {
        if *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            == 0
            && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
            && *borg
                .trait_0
                .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
                == 0
            && *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                > 2 as ::core::ffi::c_int
            && !borg.munchkin_mode
        {
            borg_note(format(
                b"# Resting to gain Mana. (danger %d)...\0" as *const u8
                    as *const ::core::ffi::c_char,
                p,
            ));
            borg_keypress('R' as i32 as keycode_t);
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
            borg.in_shop = false_0 != 0;
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0
        && borg.munchkin_mode as ::core::ffi::c_int == true_0
        && (*borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
            || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize))
        && borg_check_rest(borg.c.y, borg.c.x) as ::core::ffi::c_int != 0
    {
        if *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            == 0
            && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
            && *borg
                .trait_0
                .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
                == 0
            && *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                > 2 as ::core::ffi::c_int
            && ((*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat
                as ::core::ffi::c_int
                == FEAT_MORE as ::core::ffi::c_int
                || (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat
                    as ::core::ffi::c_int
                    == FEAT_LESS as ::core::ffi::c_int)
        {
            borg_note(format(
                b"# Resting to gain munchkin HP/mana. (danger %d)...\0" as *const u8
                    as *const ::core::ffi::c_char,
                p,
            ));
            borg_keypress('R' as i32 as keycode_t);
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
            borg.in_shop = false_0 != 0;
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        && borg.munchkin_mode as ::core::ffi::c_int == true_0
    {
        borg_note(
            b"# Resting to cure problem. (danger %d)...\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('*' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        borg.in_shop = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
