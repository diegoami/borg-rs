pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLASS_RANGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_formulas_h::borg_power_dynamic;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};

use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_item, borg_item_weight};

pub use crate::headers::borg_magic_h::{
    borg_get_spell_entry, borg_magic, borg_spell_stat, borg_spells, ACID_SPRAY, ANNIHILATE,
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
use crate::headers::borg_prepared_h::borg_prepared;
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
use crate::headers::borg_trait_swap_h::{armour_swap_value, weapon_swap_value};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
};
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
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
use crate::headers::player_calcs_h::adj_str_hold;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23,
    C2RustUnnamedH_34, PF_MAX, STAT_MAX,
};
use crate::headers::player_spell_h::spell_chance;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
unsafe extern "C" fn borg_power_equipment() -> int32_t {
    let mut hold: ::core::ffi::c_int = 0;
    let mut damage: ::core::ffi::c_int = 0;
    let mut dam: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cur_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: int32_t = 0 as int32_t;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    hold = adj_str_hold[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize] as usize];
    item = &raw mut crate::bridge::st().items.items[((*z_info).pack_size) as usize] as *mut borg_item;
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
        && (*item).iqty as ::core::ffi::c_int != 0
        && (*item).ident as ::core::ffi::c_int != true_0
    {
        value = (value as ::core::ffi::c_int + 1000000 as ::core::ffi::c_int) as int32_t;
    }
    damage = (((*item).dd as ::core::ffi::c_int * (*item).ds as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * 20 as ::core::ffi::c_long) as ::core::ffi::c_int;
    value = (value as ::core::ffi::c_int
        + damage
            * (crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int)) as int32_t;
    value = (value as ::core::ffi::c_long
        + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
            + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + (crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize]
            + (*item).to_d as ::core::ffi::c_int) as ::core::ffi::c_long
            * 30 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 15 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 75 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 15 as ::core::ffi::c_long) as int32_t;
        value = (value as ::core::ffi::c_long
            + ((*item).dd as ::core::ffi::c_int * (*item).ds as ::core::ffi::c_int)
                as ::core::ffi::c_long
                * 20 as ::core::ffi::c_long
                * 2 as ::core::ffi::c_long
                * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long) as int32_t;
    }
    dam = damage
        * 2 as ::core::ffi::c_int
        * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 7 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    dam = damage
        * 3 as ::core::ffi::c_int
        * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_WK_UNDEAD as ::core::ffi::c_int) as usize]
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_WK_DEMON as ::core::ffi::c_int) as usize]
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 6 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 4 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 4 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    dam = damage
        * 5 as ::core::ffi::c_int
        * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_WK_UNDEAD as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WK_DEMON as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_W_IMPACT as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_HEAVYWEPON as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 500000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        <= 10 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
                * 45000 as ::core::ffi::c_long) as int32_t;
    }
    item = &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as usize]
        as *mut borg_item;
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
        && (*item).iqty as ::core::ffi::c_int != 0
        && (*item).ident as ::core::ffi::c_int != true_0
    {
        value = (value as ::core::ffi::c_int + 6000000 as ::core::ffi::c_int) as int32_t;
    }
    if (*item).to_d as ::core::ffi::c_int > 8 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 25 as ::core::ffi::c_int
    {
        damage = (crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize]
            + (*item).to_d as ::core::ffi::c_int)
            * crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize];
    } else {
        damage = (crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize]
            + 8 as ::core::ffi::c_int)
            * crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize];
    }
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_SHOTS as ::core::ffi::c_int) as usize] * damage)
                as ::core::ffi::c_long
                * 11 as ::core::ffi::c_long) as int32_t;
    } else {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_SHOTS as ::core::ffi::c_int) as usize] * damage)
                as ::core::ffi::c_long
                * 9 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 15 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_SHOTS as ::core::ffi::c_int) as usize] * damage)
                as ::core::ffi::c_long
                * 200 as ::core::ffi::c_long) as int32_t;
    }
    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_sling
        && (*item).art_idx == 0
        && crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] < 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 5000 as ::core::ffi::c_long) as int32_t;
    }
    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_sling
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            == 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] >= 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 8000 as ::core::ffi::c_int) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
            + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long
            + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 25 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_FAST_SHOTS as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
            == TV_ARROW as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
    }
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long - 500000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER {
        value = (value as ::core::ffi::c_long
            - (crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 10000 as ::core::ffi::c_long) as int32_t;
    } else if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        <= 3 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
                * 10000 as ::core::ffi::c_long) as int32_t;
    } else if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        > 3 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (30000 as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                    * 1000 as ::core::ffi::c_int) as ::core::ffi::c_long))
            as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_MOD_MOVES as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 3000 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_DAM_RED as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 10000 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_SPEED as ::core::ffi::c_int) as usize] != 0 {
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 150 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 185000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 145 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 149 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 180000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 140 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 144 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 135 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 139 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 130 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 134 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 160000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 125 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 129 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 135000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 120 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 124 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 110000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 115 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 119 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 85000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 110 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 114 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 65000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 110 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
    } else {
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 140 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 135 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 139 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 165000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 130 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 134 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 150000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 125 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 129 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 125000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 120 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 124 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 100000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 115 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 119 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 75000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
            >= 110 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                <= 114 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 55000 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 110 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
    }
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + spell_stat)) as usize]
                as ::core::ffi::c_long
                * 500 as ::core::ffi::c_long) as int32_t;
        if crate::bridge::st().core.cfg[(BORG_WORSHIPS_MANA as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_SP_ADJ as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 255 as ::core::ffi::c_long) as int32_t;
        } else {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_SP_ADJ as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 155 as ::core::ffi::c_long) as int32_t;
        }
        value = (value as ::core::ffi::c_int
            + (100 as ::core::ffi::c_int
                - spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int) as int32_t;
        if flag_has(
            &raw mut (*player).state.pflags as *mut bitflag,
            (PF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            PF_ZERO_FAIL as ::core::ffi::c_int,
        ) {
            if (spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                < 1 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
        <= 37 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_long
                * 120 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CON_INDEX as ::core::ffi::c_int) as usize]
        <= 37 as ::core::ffi::c_int
    {
        if crate::bridge::st().core.cfg[(BORG_WORSHIPS_HP as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long
                + crate::bridge::st().borg.trait_[(BI_CON_INDEX as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 250 as ::core::ffi::c_long) as int32_t;
            if crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                < 800 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long
                    + crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_long
                        * 450 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                        - 800 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long
                        + 350 as ::core::ffi::c_long * 500 as ::core::ffi::c_long))
                    as int32_t;
            }
        } else {
            value = (value as ::core::ffi::c_long
                + crate::bridge::st().borg.trait_[(BI_CON_INDEX as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 150 as ::core::ffi::c_long) as int32_t;
            if crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                < 500 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long
                    + crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_long
                        * 350 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize]
                        - 500 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long
                        + 350 as ::core::ffi::c_long * 500 as ::core::ffi::c_long))
                    as int32_t;
            }
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_int
            + crate::bridge::st().borg.trait_[((BI_ASTR as ::core::ffi::c_int + i)) as usize]) as int32_t;
        i += 1;
    }
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 25 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 25 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] > 99 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_int + 10000 as ::core::ffi::c_int) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_SRCH as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 1 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_THN as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 5 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_THB as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 35 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_THT as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 750 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 7500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 7500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        < 20 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        }
    } else if crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
        if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_DINV as ::core::ffi::c_int) as usize] == 0 {
        if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 50 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0 {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        }
    } else if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 65000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 40000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 3500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 800 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 800 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        && crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 5500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    }
    if (crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0)
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 10 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 20 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 25 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 115 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 55000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 55 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 55 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 55000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 104000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 90000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 120 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 130 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 15000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 25000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 140 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().core.cfg[(BORG_WORSHIPS_AC as ::core::ffi::c_int) as usize] != 0 {
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] < 15 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                < 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 2000 as ::core::ffi::c_long
                    + 28250 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] >= 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 73750 as ::core::ffi::c_long)) as int32_t;
        }
    } else {
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] < 15 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 2000 as ::core::ffi::c_long) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                < 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 500 as ::core::ffi::c_long
                    + 28350 as ::core::ffi::c_long)) as int32_t;
        }
        if crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] >= 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long
                    + 73750 as ::core::ffi::c_long)) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSHPIMP as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int - 35000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_WARRIOR
        && crate::bridge::st().borg.trait_[(BI_CRSMPIMP as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_int - 15000 as ::core::ffi::c_int) as int32_t;
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_DRUID
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
        && crate::bridge::st().borg.trait_[(BI_CRSMPIMP as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_int - 15000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 400000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE
    {
        value = (value as ::core::ffi::c_long - 200000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSDRAIN_XP as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 400000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSFVULN as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSEVULN as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSCVULN as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSAVULN as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSTELE as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSENVELOPING as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 50000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSIRRITATION as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 20000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSPOIS as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSIREN as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSHALU as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSPARA as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSDEM as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSDRA as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSUND as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSTONE as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 140 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSTEELSKIN as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 140 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSNOTEL as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 700000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSTWEP as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSAIRSWING as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSUNKNO as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long - 9999999 as ::core::ffi::c_long) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + 3000 as ::core::ffi::c_long
            * crate::bridge::st().borg.trait_[(BI_MULTIPLE_BONUSES as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + 10000 as ::core::ffi::c_long
            * crate::bridge::st().borg.trait_[(BI_WORN_NEED_ID as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_long) as int32_t;
    let mut activation_bonus: ::core::ffi::c_int = 0;
    let mut act: ::core::ffi::c_int = 0;
    act = 1 as ::core::ffi::c_int;
    while act < (*z_info).act_max as ::core::ffi::c_int {
        if !(crate::bridge::st().borg.activation[(act) as usize] == 0) {
            activation_bonus = 0 as ::core::ffi::c_int;
            if crate::bridge::st().act.illumination == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.mapping == act {
                activation_bonus += 550 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.clairvoyance == act {
                activation_bonus += 600 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 9 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.elec_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.acid_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.mana_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.stinking_cloud == act {
                activation_bonus += 500 as ::core::ffi::c_int + 24 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_ball50 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 96 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_ball100 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_ball72 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 72 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 144 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dispel_evil == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + (10 as ::core::ffi::c_int
                        + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                            * 5 as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int);
            } else if crate::bridge::st().act.confuse2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.haste == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.haste1 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.haste2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.detect_objects == act {
                activation_bonus += 10 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.probing == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.stone_to_mud == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tele_other == act {
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE {
                    activation_bonus += 500 as ::core::ffi::c_int;
                } else {
                    activation_bonus += 500 as ::core::ffi::c_int + 500 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.drain_life1 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 90 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.drain_life2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.berserker == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_light == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_serious == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_critical == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_full2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.loskill == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.recall == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.arrow == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.rem_fear_pois == act {
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
                    || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
                    || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_DRUID
                {
                    activation_bonus += 500 as ::core::ffi::c_int;
                } else {
                    activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.tele_phase == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.detect_all == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_full == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.heal1 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.heal2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.heal3 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_nonorlybig == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.protevil == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.destroy_doors == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.banishment == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_all == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE {
                    activation_bonus += 25000 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.sleepii == act {
                activation_bonus += 500 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_DRUID
                    && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                        != CLASS_NECROMANCER
                {
                    activation_bonus += 200 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.recharge == act {
                activation_bonus += 500 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE
                    && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_ROGUE
                {
                    activation_bonus += 100 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.tele_long == act {
                activation_bonus += 300 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.missile == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_temp == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.starlight2 == act {
                activation_bonus += 100 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.bizarre == act {
                activation_bonus += 999999 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.star_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 300 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.rage_bless_resist == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE {
                    activation_bonus += 25000 as ::core::ffi::c_int;
                }
            } else if crate::bridge::st().act.polymorph == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.starlight == act {
                activation_bonus += 100 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.light == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.firebrand == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_life == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_exp == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_st_lev == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enlightenment == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.hero == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shero == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_paranoia == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_mind == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_body == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.mon_slow == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.mon_confuse == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.sleep_all == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.mon_scare == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.light_line == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.disable_traps == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.drain_life3 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.drain_life4 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 250 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.elec_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 64 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.elec_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 250 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.acid_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.acid_bolt3 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.acid_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_ball160 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 160 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cold_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_ball200 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.fire_bolt3 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dispel_evil60 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dispel_undead == act {
                activation_bonus += 500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dispel_all == act {
                activation_bonus +=
                    500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.deep_descent == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.earthquakes == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.destruction2 == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.losslow == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.lossleep == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.losconf == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.satisfy == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.blessing == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.blessing2 == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.blessing3 == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.glyph == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tele_level == act {
                activation_bonus = (activation_bonus as ::core::ffi::c_long
                    + 5000 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
            } else if crate::bridge::st().act.confusing == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enchant_tohit == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enchant_todam == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enchant_weapon == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enchant_armor == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.enchant_armor2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.remove_curse == act {
                activation_bonus += 9000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.remove_curse2 == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.detect_treasure == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.detect_invis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.detect_evil == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_mana == act {
                activation_bonus += 5000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.brawn == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.intellect == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.contemplation == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.toughness == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.nimbleness == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_str == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_int == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_wis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_dex == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_con == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.restore_all == act {
                activation_bonus += 150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tmd_free_act == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tmd_infra == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tmd_sinvis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.tmd_esp == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_acid == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_elec == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_fire == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_cold == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.resist_pois == act {
                activation_bonus += 150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.cure_confusion == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.wonder == act {
                activation_bonus += 300 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.wand_breath == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.staff_magi == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.staff_holy == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.drink_breath == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.food_waybread == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_emergency == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_terror == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_stone == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_debility == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_sprinting == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.shroom_purging == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.ring_acid == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.ring_flames == act {
                activation_bonus += 25000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.ring_ice == act {
                activation_bonus += 15000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.ring_lightning == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_blue == act {
                activation_bonus += 1100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_green == act {
                activation_bonus += 2750 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_red == act {
                activation_bonus += 1100 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_multihued == act {
                activation_bonus += 3250 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_gold == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_chaos == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_law == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_balance == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_shining == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if crate::bridge::st().act.dragon_power == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            }
            value = (value as ::core::ffi::c_int
                + activation_bonus * crate::bridge::st().borg.activation[(act) as usize])
                as int32_t;
        }
        act += 1;
    }
    if (crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int)) as usize])
    .iqty
        != 0
    {
        item = &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int)) as usize]
            as *mut borg_item;
        if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
            as ::core::ffi::c_int
            != 0
            && !(*item).ident
        {
            value = (value as ::core::ffi::c_long + 999999 as ::core::ffi::c_long) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
        < 15 as ::core::ffi::c_int
    {
        if borg_item_weight(
            &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int)) as usize] as *mut borg_item,
        ) as ::core::ffi::c_int
            > 200 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int
                - (borg_item_weight(&raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int)) as usize] as *mut borg_item) as ::core::ffi::c_int
                    - 200 as ::core::ffi::c_int)
                    * 15 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int)) as usize] as *mut borg_item,
        ) as ::core::ffi::c_int
            > 30 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int)) as usize] as *mut borg_item,
        ) as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int)) as usize] as *mut borg_item,
        ) as ::core::ffi::c_int
            > 50 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
    }
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int)) as usize]
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    max_wgt = (*(*player).class).magic.spell_weight;
    if (*(*player).class).magic.total_spells != 0
        && (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut max_sp: ::core::ffi::c_int = crate::bridge::st().borg.trait_[(BI_SP_ADJ as ::core::ffi::c_int) as usize]
            / 100 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        max_sp -= (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int;
        if max_sp >= 300 as ::core::ffi::c_int && max_sp <= 350 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 400 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 200 as ::core::ffi::c_int && max_sp <= 299 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 800 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 100 as ::core::ffi::c_int && max_sp <= 199 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1600 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp <= 99 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 3200 as ::core::ffi::c_long) as int32_t;
        }
    }
    return value;
}
unsafe extern "C" fn borg_power_inventory() -> int32_t {
    let mut k: ::core::ffi::c_int = 0;
    let mut book: ::core::ffi::c_int = 0;
    let mut value: int32_t = 0 as int32_t;
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 60000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int {
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long
                + (6000 as ::core::ffi::c_long
                    - (k * 100 as ::core::ffi::c_int) as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0)
        && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_int + 100000 as ::core::ffi::c_int) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 7 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int {
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            <= 15 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 15 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 700 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 7 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_FOOD_HI as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 52 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 15 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_FOOD_LO as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if (crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0)
        && crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_int + 100000 as ::core::ffi::c_int) as int32_t;
    }
    if (crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0)
        && crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if (crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0)
        && crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize] != 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 25000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_ACUREPOIS as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_int + 15000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_IPOIS as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.trait_[(BI_ACUREPOIS as ::core::ffi::c_int) as usize]
            <= 20 as ::core::ffi::c_int
    {
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
        {
            while k < 4 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_ARESPOIS as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 300 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            > 20 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
        {
            if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_ARESHEAT as ::core::ffi::c_int) as usize]
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            k = 0 as ::core::ffi::c_int;
            if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_ARESCOLD as ::core::ffi::c_int) as usize]
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            if crate::bridge::st().borg.trait_[(BI_IPOIS as ::core::ffi::c_int) as usize] == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_ARESPOIS as ::core::ffi::c_int) as usize]
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 10 as ::core::ffi::c_int
    {
        while k < 5 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 600 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ALL_NEED_ID as ::core::ffi::c_int) as usize]
        != 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < crate::bridge::st().borg.trait_[(BI_ALL_NEED_ID as ::core::ffi::c_int) as usize]
            && k < crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
    {
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_APFE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 25 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_APFE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 10 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    while k < 25 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 75 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AMASSBAN as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 7 as ::core::ffi::c_int
    {
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
        {
            while k < 3 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
            if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize]
                >= 15 as ::core::ffi::c_int
            {
                while k < 7 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
                {
                    value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                >= 50 as ::core::ffi::c_int
                && crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_rod_recall) as usize] != 0
            {
                value = (value as ::core::ffi::c_int + 12000 as ::core::ffi::c_int) as int32_t;
            }
        }
    }
    k = 1 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
    {
        while k < 8 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] >= 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
    {
        while k < 2 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            > 70 as ::core::ffi::c_int
        {
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 3 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
            != 0
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 7 as ::core::ffi::c_int
    {
        while k < 3 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 30 as ::core::ffi::c_int
    {
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 15 as ::core::ffi::c_int
    {
        while k < 5 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_ROGUE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_BLACKGUARD
    {
        k = 0 as ::core::ffi::c_int;
        while k < 15 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 46 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize]
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 6 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_rod_healing) as usize] {
            value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_RANGER
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PALADIN
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 46 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize]
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PALADIN {
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_healing) as usize] {
                value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 4 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_rod_healing) as usize] {
            value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_DRUID
    {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            == 1 as ::core::ffi::c_int
        {
            k = 0 as ::core::ffi::c_int;
            while k < 10 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_healing) as usize] {
                value = (value as ::core::ffi::c_long - 2000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_healing) as usize] {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 46 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize]
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 99 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize]
            == 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_healing) as usize] {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_WARRIOR {
            while k < 99 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_restore_mana) as usize]
            {
                value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 40 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_mush_stoneskin) as usize] {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_SAURON_DEAD as ::core::ffi::c_int) as usize]
            != 0
        {
            k = 0 as ::core::ffi::c_int;
            while k < 99 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AMASSBAN as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 100 as ::core::ffi::c_int {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_restore_mana) as usize]
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 100 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ASTFMAGI as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 35 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            > 10 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 35 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] > 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] < 5 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            > 10 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 35 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] == 0)
    {
        k = 0 as ::core::ffi::c_int;
        while k < 7 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] > 15 as ::core::ffi::c_int {
            while k < 10 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 5 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
        < 5 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 8 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 550 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] == 0 {
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int)
        {
            k = 0 as ::core::ffi::c_int;
            while k < 10 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_FOOD_CURE_CONF as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 400 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
        == 0
    {
        k = 0 as ::core::ffi::c_int;
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
        {
            while k < 5 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_FOOD_CURE_BLIND as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 300 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] == 0 {
        k = 0 as ::core::ffi::c_int;
        if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
        {
            while k < 5 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_ACUREPOIS as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_ADETDOOR as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] == 0 {
        k = 0 as ::core::ffi::c_int;
        while k < 1 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ADETEVIL as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AMAGICMAP as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_NECROMANCER {
        k = 0 as ::core::ffi::c_int;
        while k < 1 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ALITE as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_scroll_mass_banishment) as usize]
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 25 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_scroll_mass_banishment) as usize]
        {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize])
    {
        while k < 20 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ARECHARGE as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 99 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_RANGER
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
    {
        k = 0 as ::core::ffi::c_int;
        while k < 40 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] > 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize]
                <= 18 as ::core::ffi::c_int
        {
            while k < 80 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] > 18 as ::core::ffi::c_int {
            while k < 180 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 8 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 4 as ::core::ffi::c_int;
        while k < crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else {
        k = 0 as ::core::ffi::c_int;
        while k < 20 as ::core::ffi::c_int
            && k < crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] > 15 as ::core::ffi::c_int {
            while k < 50 as ::core::ffi::c_int
                && k < crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
            {
                value = (value as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] <= 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                > 20 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long - 1000 as ::core::ffi::c_long) as int32_t;
        }
        k = 2 as ::core::ffi::c_int;
        while k < crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
        {
            value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    value = (value as ::core::ffi::c_long
        - 1000 as ::core::ffi::c_long
            * crate::bridge::st().borg.trait_[(BI_AMISSILES_CURSED as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + 100 as ::core::ffi::c_long
            * crate::bridge::st().borg.trait_[(BI_AMISSILES_SPECIAL as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().borg.trait_[(BI_ASTFDEST as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 9 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_ASTFDEST as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        && crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 15 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if (crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_magic_missile) as usize] != 0
        || crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_stinking_cloud) as usize] != 0)
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 30 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_annihilation) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            < 30 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            <= 20 as ::core::ffi::c_int)
        && (crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_magic_missile) as usize] != 0
            || crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_annihilation) as usize] != 0
            || crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_wand_stinking_cloud) as usize] != 0)
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + crate::bridge::st().borg.trait_[(BI_GOOD_W_CHG as ::core::ffi::c_int) as usize] as ::core::ffi::c_long
            * 50 as ::core::ffi::c_long) as int32_t;
    if crate::bridge::st().borg.trait_[(BI_GOOD_S_CHG as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 3 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_GOOD_S_CHG as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AROD1 as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_int + 8000 as ::core::ffi::c_int) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_AROD2 as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_int + 12000 as ::core::ffi::c_int) as int32_t;
        k += 1;
    }
    if !crate::bridge::st().borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !crate::bridge::st().borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 20000 as ::core::ffi::c_int) as int32_t;
    }
    if !crate::bridge::st().borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 20000 as ::core::ffi::c_int) as int32_t;
    }
    if !crate::bridge::st().borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !crate::bridge::st().borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        if !crate::bridge::st().borg.need_statgain[spell_stat as usize] {
            value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if crate::bridge::st().borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 520000 as ::core::ffi::c_int) as int32_t;
    }
    if spell_stat >= 0 as ::core::ffi::c_int {
        if crate::bridge::st().borg.amt_statgain[spell_stat as usize] as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[((BI_CSTR as ::core::ffi::c_int + spell_stat)) as usize]
                < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int + 575000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if crate::bridge::st().borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 520000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
        != 0
    {
        if crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_scroll_star_remove_curse) as usize] != 0 {
            value = (value as ::core::ffi::c_int + 90000 as ::core::ffi::c_int) as int32_t;
        }
        if crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_scroll_remove_curse) as usize] != 0 {
            value = (value as ::core::ffi::c_int + 90000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_HASFIXEXP as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_ARM as ::core::ffi::c_int) as usize]
        < 1000 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 540 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize]
        < 1000 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 540 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize]
        < 1000 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize]
            != 0
    {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize]
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    k = 1 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < crate::bridge::st().borg.trait_[(BI_EMPTY as ::core::ffi::c_int) as usize]
    {
        value = (value as ::core::ffi::c_long + 40 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_EMPTY as ::core::ffi::c_int) as usize] != 0 {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        <= 40 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 25 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            < 100000 as ::core::ffi::c_int
        && (crate::bridge::st().items.items[((*z_info).pack_size) as usize]).tval as ::core::ffi::c_int
            != TV_DIGGING as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ADIGGER as ::core::ffi::c_int) as usize]
            == 1 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    book = 0 as ::core::ffi::c_int;
    while book < 9 as ::core::ffi::c_int {
        if !(crate::bridge::st().borg.amt_book[book as usize] == 0) {
            if (*(*(*player).class).magic.books.offset(book as isize)).dungeon {
                let mut what: ::core::ffi::c_int = 0;
                what = 0 as ::core::ffi::c_int;
                while what < 9 as ::core::ffi::c_int {
                    let mut as_0: *mut borg_magic = borg_get_spell_entry(book, what);
                    if as_0.is_null() {
                        break;
                    }
                    if !((*as_0).level as ::core::ffi::c_int
                        > crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize])
                    {
                        if !((*as_0).power as ::core::ffi::c_int
                            > crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize])
                        {
                            value = (value as ::core::ffi::c_long + 15000 as ::core::ffi::c_long)
                                as int32_t;
                        }
                    }
                    what += 1;
                }
            } else {
                let mut what_0: ::core::ffi::c_int = 0;
                let mut when: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
                what_0 = 0 as ::core::ffi::c_int;
                while what_0 < 9 as ::core::ffi::c_int {
                    let mut as_1: *mut borg_magic = borg_get_spell_entry(book, what_0);
                    if as_1.is_null() {
                        break;
                    }
                    if ((*as_1).level as ::core::ffi::c_int) < when {
                        when = (*as_1).level as ::core::ffi::c_int;
                    }
                    what_0 += 1;
                }
                if !(when > 5 as ::core::ffi::c_int
                    && when
                        >= crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                            + 2 as ::core::ffi::c_int)
                {
                    k = 0 as ::core::ffi::c_int;
                    while k < 1 as ::core::ffi::c_int
                        && k < crate::bridge::st().borg.amt_book[book as usize] as ::core::ffi::c_int
                    {
                        value = (value as ::core::ffi::c_long + 500000 as ::core::ffi::c_long)
                            as int32_t;
                        k += 1;
                    }
                    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize]
                        > 5 as ::core::ffi::c_int
                    {
                        while k < 2 as ::core::ffi::c_int
                            && k < crate::bridge::st().borg.amt_book[book as usize] as ::core::ffi::c_int
                        {
                            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long)
                                as int32_t;
                            k += 1;
                        }
                    }
                }
            }
        }
        book += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize]
        > crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
        let mut i: ::core::ffi::c_int = (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize];
        while i >= 0 as ::core::ffi::c_int {
            if (crate::bridge::st().items.items[(i) as usize]).iqty != 0 {
                item = &raw mut crate::bridge::st().items.items[(i) as usize] as *mut borg_item;
                break;
            } else {
                i -= 1;
            }
        }
        if !(!item.is_null()
            && (*item).iqty as ::core::ffi::c_int != 0
            && (*item).aware as ::core::ffi::c_int != 0
            && ((*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_armor
                    && crate::bridge::st().borg.trait_[(BI_AENCH_ARM as ::core::ffi::c_int) as usize]
                        < 1000 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int) as usize]
                        != 0
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_hit
                        && crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize]
                            < 1000 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize]
                            != 0
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_dam
                        && crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize]
                            < 1000 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize]
                            != 0
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_star_enchant_weapon
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_star_enchant_armor)
                || (*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_str
                        || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_int
                        || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_wis
                        || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_dex
                        || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_con
                        || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_all)))
        {
            value = (value as ::core::ffi::c_long
                - ((crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize]
                    - crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize]
                        / 2 as ::core::ffi::c_int)
                    / (crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize]
                        / 10 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long) as int32_t;
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn borg_power() -> int32_t {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut value: int32_t = 0 as int32_t;
    if crate::bridge::st().core.cfg[(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int) as usize] != 0 {
        value += borg_power_dynamic();
    } else {
        value += borg_power_equipment();
        value += borg_power_inventory();
    }
    i = 1 as ::core::ffi::c_int;
    while i
        <= crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            + 50 as ::core::ffi::c_int
    {
        if !(borg_prepared(i) as *mut ::core::ffi::c_char).is_null() {
            break;
        }
        i += 1;
    }
    value = (value as ::core::ffi::c_long
        + (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_long * 40000 as ::core::ffi::c_long)
        as int32_t;
    value += weapon_swap_value;
    value += armour_swap_value;
    return value;
}
