pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_formulas_calc_h::{
    borg_calculate_dynamic, calculations_free, parse_calculation_line,
};
pub use crate::headers::borg_formulas_h::{
    borg_array, value_sec, value_type, VT_ACTIVATION, VT_CLASS, VT_CONFIG, VT_MAX, VT_NONE,
    VT_RANGE_INDEX, VT_TRAIT,
};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_cfg, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH,
    BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT,
    BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT,
    BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER,
    BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS,
    BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM,
    BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS,
    BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD,
    BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED,
};
pub use crate::headers::borg_init_h::{borg_setting, borg_settings};
use crate::headers::borg_io_h::borg_note;
use crate::headers::borg_item_activation_h::borg_findact;
use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_item, borg_item_weight, borg_items};
use crate::headers::borg_item_val_h::{
    borg_lookup_kind, sv_potion_inc_all, sv_potion_inc_con, sv_potion_inc_dex, sv_potion_inc_int,
    sv_potion_inc_str, sv_potion_inc_wis, sv_scroll_enchant_armor, sv_scroll_enchant_weapon_to_dam,
    sv_scroll_enchant_weapon_to_hit, sv_scroll_star_enchant_armor, sv_scroll_star_enchant_weapon,
};
pub use crate::headers::borg_magic_h::{
    borg_can_cast, borg_get_spell_entry, borg_magic, borg_spell_stat, borg_spells, ACID_SPRAY,
    ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN,
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
    prefix_pref, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
use crate::headers::borg_util_h::borg_trim;
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
};
pub use crate::headers::list_tvals_h::{
    TV_AMULET, TV_ARROW, TV_BOLT, TV_BOOTS, TV_BOW, TV_CHEST, TV_CLOAK, TV_CROWN, TV_DIGGING,
    TV_DRAG_ARMOR, TV_FLASK, TV_FOOD, TV_GLOVES, TV_GOLD, TV_HAFTED, TV_HARD_ARMOR, TV_HELM,
    TV_LIGHT, TV_MAGIC_BOOK, TV_MUSHROOM, TV_NATURE_BOOK, TV_NULL, TV_OTHER_BOOK, TV_POLEARM,
    TV_POTION, TV_PRAYER_BOOK, TV_RING, TV_ROD, TV_SCROLL, TV_SHADOW_BOOK, TV_SHIELD, TV_SHOT,
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND, VT_AMULET, VT_ARROW, VT_BOLT, VT_BOOTS, VT_BOW,
    VT_CHEST, VT_CLOAK, VT_CROWN, VT_DIGGING, VT_DRAG_ARMOR, VT_FLASK, VT_FOOD, VT_GLOVES, VT_GOLD,
    VT_HAFTED, VT_HARD_ARMOR, VT_HELM, VT_LIGHT, VT_MAGIC_BOOK, VT_MUSHROOM, VT_NATURE_BOOK,
    VT_NULL, VT_OTHER_BOOK, VT_POLEARM, VT_POTION, VT_PRAYER_BOOK, VT_RING, VT_ROD, VT_SCROLL,
    VT_SHADOW_BOOK, VT_SHIELD, VT_SHOT, VT_SOFT_ARMOR, VT_STAFF, VT_SWORD, VT_WAND,
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
use crate::headers::obj_util_h::lookup_sval;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, classes, equip_slot, history_chart, history_entry,
    history_info, magic_realm, player, player_blow, player_body, player_class, player_history,
    player_race, player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_34,
    PF_MAX,
};
use crate::headers::player_spell_h::spell_chance;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::stdlib_h::{atol, strtol};
use crate::headers::string_h::{strchr, strlen, strncpy};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_file_h::{ang_file, file_getl};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::{grouper, loc};
pub use crate::headers::z_util_h::{my_rational, my_stricmp, prefix_i};
use crate::headers::z_virt_h::{
    mem_alloc, mem_free, mem_realloc, mem_zalloc, string_free, string_make,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_power_line {
    pub range: *mut range_sec,
    pub reward: *mut math_section,
    pub value: *mut value_sec,
    pub condition: *mut math_section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct math_section {
    pub is_calculation: bool,
    pub calculation_index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_sec {
    pub from: int32_t,
    pub to: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_formula_arrays {
    pub power: borg_array,
    pub depth: borg_array,
    pub restock: borg_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_depth_line {
    pub dlevel: int32_t,
    pub reason: *mut ::core::ffi::c_char,
    pub condition: *mut math_section,
}
pub const SEC_NONE: e_section = 0;
pub type e_section = ::core::ffi::c_uint;
pub const SEC_RESTOCK: e_section = 3;
pub const SEC_DEPTH: e_section = 2;
pub const SEC_POWER: e_section = 1;
static mut value_type_names: [grouper; 42] = [
    grouper {
        tval: VT_NONE as ::core::ffi::c_int,
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_RANGE_INDEX as ::core::ffi::c_int,
        name: b"range\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_TRAIT as ::core::ffi::c_int,
        name: b"trait\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CONFIG as ::core::ffi::c_int,
        name: b"config\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ACTIVATION as ::core::ffi::c_int,
        name: b"activation\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CLASS as ::core::ffi::c_int,
        name: b"class\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_NULL as ::core::ffi::c_int,
        name: b"none\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CHEST as ::core::ffi::c_int,
        name: b"chest\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHOT as ::core::ffi::c_int,
        name: b"shot\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ARROW as ::core::ffi::c_int,
        name: b"arrow\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOLT as ::core::ffi::c_int,
        name: b"bolt\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOW as ::core::ffi::c_int,
        name: b"bow\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_DIGGING as ::core::ffi::c_int,
        name: b"digger\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HAFTED as ::core::ffi::c_int,
        name: b"hafted\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_POLEARM as ::core::ffi::c_int,
        name: b"polearm\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SWORD as ::core::ffi::c_int,
        name: b"sword\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOOTS as ::core::ffi::c_int,
        name: b"boots\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_GLOVES as ::core::ffi::c_int,
        name: b"gloves\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HELM as ::core::ffi::c_int,
        name: b"helm\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CROWN as ::core::ffi::c_int,
        name: b"crown\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHIELD as ::core::ffi::c_int,
        name: b"shield\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CLOAK as ::core::ffi::c_int,
        name: b"cloak\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SOFT_ARMOR as ::core::ffi::c_int,
        name: b"soft armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HARD_ARMOR as ::core::ffi::c_int,
        name: b"hard armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_DRAG_ARMOR as ::core::ffi::c_int,
        name: b"dragon armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_LIGHT as ::core::ffi::c_int,
        name: b"light\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_AMULET as ::core::ffi::c_int,
        name: b"amulet\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_RING as ::core::ffi::c_int,
        name: b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_STAFF as ::core::ffi::c_int,
        name: b"staff\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_WAND as ::core::ffi::c_int,
        name: b"wand\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ROD as ::core::ffi::c_int,
        name: b"rod\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SCROLL as ::core::ffi::c_int,
        name: b"scroll\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_POTION as ::core::ffi::c_int,
        name: b"potion\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_FLASK as ::core::ffi::c_int,
        name: b"flask\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_FOOD as ::core::ffi::c_int,
        name: b"food\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_MUSHROOM as ::core::ffi::c_int,
        name: b"mushroom\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_MAGIC_BOOK as ::core::ffi::c_int,
        name: b"magic book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_PRAYER_BOOK as ::core::ffi::c_int,
        name: b"prayer book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_NATURE_BOOK as ::core::ffi::c_int,
        name: b"nature book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHADOW_BOOK as ::core::ffi::c_int,
        name: b"shadow book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_OTHER_BOOK as ::core::ffi::c_int,
        name: b"other book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_GOLD as ::core::ffi::c_int,
        name: b"gold\0" as *const u8 as *const ::core::ffi::c_char,
    },
];
#[no_mangle]
pub static mut borg_formulas: borg_formula_arrays = borg_formula_arrays {
    power: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
    depth: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
    restock: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
};
#[no_mangle]
pub unsafe extern "C" fn borg_array_add(
    mut a: *mut borg_array,
    mut item: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*a).count == 0 {
        (*a).max = 10 as ::core::ffi::c_int;
        (*a).items = mem_alloc(
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul((*a).max as size_t),
        ) as *mut *mut ::core::ffi::c_void;
    }
    if (*a).max < (*a).count + 1 as ::core::ffi::c_int {
        (*a).max += 10 as ::core::ffi::c_int;
        (*a).items = mem_realloc(
            (*a).items as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul((*a).max as size_t),
        ) as *mut *mut ::core::ffi::c_void;
    }
    let ref mut fresh0 = *(*a).items.offset((*a).count as isize);
    *fresh0 = item;
    let fresh1 = (*a).count;
    (*a).count = (*a).count + 1;
    return fresh1;
}
unsafe extern "C" fn borg_get_class(
    mut class_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: *mut player_class = ::core::ptr::null_mut::<player_class>();
    c = classes;
    while !c.is_null() {
        if my_stricmp((*c).name, class_name) == 0 {
            return (*c).cidx as ::core::ffi::c_int;
        }
        c = (*c).next;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_formula_error(
    mut section: *const ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
    mut section_label: *const ::core::ffi::c_char,
    mut error: *const ::core::ffi::c_char,
) {
    borg_note(b"** borg formula failure ** \0" as *const u8 as *const ::core::ffi::c_char);
    borg_note(format(
        b"** error on line '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        full_line,
    ));
    borg_note(format(
        b"** error for section %s '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        section_label,
        section,
    ));
    if !error.is_null() {
        borg_note(error);
    }
    borg_note(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn parse_depth(mut line: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    line = strchr(line, '(' as i32);
    if line.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_long = strtol(line, &raw mut end, 10 as ::core::ffi::c_int);
    if end == line {
        return -(1 as ::core::ffi::c_int);
    }
    while *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        end = end.offset(1);
    }
    if *end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ')' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    return l as ::core::ffi::c_int;
}
unsafe extern "C" fn is_all_number(mut line: *mut ::core::ffi::c_char) -> bool {
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    if *line != 0 {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn parse_formula_section(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut formula: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    if is_all_number(line) {
        let mut c: *mut math_section =
            mem_alloc(::core::mem::size_of::<math_section>() as size_t) as *mut math_section;
        (*c).calculation_index = atol(line) as int32_t;
        (*c).is_calculation = false_0 != 0;
        return c;
    }
    formula = parse_calculation_line(line, full_line);
    if formula == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<math_section>();
    }
    let mut c_0: *mut math_section =
        mem_alloc(::core::mem::size_of::<math_section>() as size_t) as *mut math_section;
    (*c_0).calculation_index = formula as int32_t;
    (*c_0).is_calculation = true_0 != 0;
    return c_0;
}
unsafe extern "C" fn parse_condition(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut start: *mut ::core::ffi::c_char = line;
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    line = line.offset(1);
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    return parse_formula_section(line, full_line);
}
unsafe extern "C" fn parse_reward(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut start: *mut ::core::ffi::c_char = line;
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no starting '('\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    line = line.offset(1);
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    return parse_formula_section(line, full_line);
}
unsafe extern "C" fn parse_range(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut range_sec {
    let mut from: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut start: *mut ::core::ffi::c_char = line;
    let mut val1str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val2str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no starting '('\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val1str = line;
    line = strchr(line, ',' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** range needs two numbers separated by a ','\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    *line = 0 as ::core::ffi::c_char;
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val2str = line;
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** first value is not a number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    if !is_all_number(val1str) {
        return ::core::ptr::null_mut::<range_sec>();
    }
    if !is_all_number(val2str) {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value is not a number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    from = atol(val1str) as ::core::ffi::c_int;
    to = atol(val2str) as ::core::ffi::c_int;
    if from < 1 as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** first value less than one\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    if to < 0 as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value less than one\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    if to < from {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value less first value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    let mut r: *mut range_sec =
        mem_alloc(::core::mem::size_of::<range_sec>() as size_t) as *mut range_sec;
    (*r).from = from as int32_t;
    (*r).to = to as int32_t;
    return r;
}
unsafe extern "C" fn get_value_type(mut value_type_name: *mut ::core::ffi::c_char) -> value_type {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[grouper; 42]>() as usize)
            .wrapping_div(::core::mem::size_of::<grouper>() as usize)
            as ::core::ffi::c_int
    {
        if my_stricmp(value_type_name, value_type_names[i as usize].name) == 0 {
            return value_type_names[i as usize].tval as value_type;
        }
        i += 1;
    }
    return VT_NONE;
}
unsafe extern "C" fn get_value_index(
    mut t: value_type,
    mut name: *const ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    match t as ::core::ffi::c_int {
        3 => {
            i = borg_findact(name);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** unable to find activation\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return i;
        }
        1 => {
            i = 0 as ::core::ffi::c_int;
            while i < BI_MAX as ::core::ffi::c_int {
                if my_stricmp(
                    *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char).offset(i as isize),
                    name,
                ) == 0
                {
                    return i;
                }
                i += 1;
            }
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** unable to find trait\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        2 => {
            i = 0 as ::core::ffi::c_int;
            while !(*(&raw mut borg_settings as *mut borg_setting).offset(i as isize))
                .setting_string
                .is_null()
            {
                if my_stricmp(
                    (*(&raw mut borg_settings as *mut borg_setting).offset(i as isize))
                        .setting_string
                        .offset(5 as ::core::ffi::c_int as isize),
                    name,
                ) == 0
                {
                    return i;
                }
                i += 1;
            }
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** unable to find config\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        0 => return 1 as ::core::ffi::c_int,
        4 => {
            i = borg_get_class(name);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** invalid class\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return i;
        }
        -1 | 5 | 41 => {
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** invalid value type\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            let mut tval: ::core::ffi::c_int =
                t as ::core::ffi::c_int - VT_NULL as ::core::ffi::c_int;
            let mut sval: ::core::ffi::c_int = lookup_sval(tval, name);
            if sval == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    format(
                        b"** value error for object type '%s' subtype '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        value_type_names[t as usize].name,
                        name,
                    ),
                );
                return -(1 as ::core::ffi::c_int);
            }
            i = borg_lookup_kind(tval, sval);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    format(
                        b"** value error for object type '%s' subtype '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        value_type_names[t as usize].name,
                        name,
                    ),
                );
            }
            return i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn calculate_from_value(
    mut value: *mut value_sec,
    mut range_index: ::core::ffi::c_int,
) -> int32_t {
    match (*value).type_0 as ::core::ffi::c_int {
        3 => return *borg.activation.offset((*value).index as isize) as int32_t,
        1 => return *borg.trait_0.offset((*value).index as isize) as int32_t,
        2 => return *borg_cfg.offset((*value).index as isize) as int32_t,
        4 => {
            return (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) as int32_t
                == (*value).index) as ::core::ffi::c_int;
        }
        0 => return range_index as int32_t,
        _ => return *borg.has.offset((*value).index as isize) as int32_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_value(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut value_sec {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut val1str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val2str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val1str = line;
    line = strchr(line, ',' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** value needs two values separated by a ',' \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    *line = 0 as ::core::ffi::c_char;
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val2str = line;
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')' \0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    let mut t: value_type = get_value_type(val1str);
    if t as ::core::ffi::c_int == VT_NONE as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            format(
                b"** value type (first parameter '%s') not found\0" as *const u8
                    as *const ::core::ffi::c_char,
                val1str,
            ),
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    let mut index: ::core::ffi::c_int = get_value_index(t, val2str, full_line);
    if index == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<value_sec>();
    }
    let mut v: *mut value_sec =
        mem_alloc(::core::mem::size_of::<value_sec>() as size_t) as *mut value_sec;
    (*v).type_0 = t;
    (*v).index = index as int32_t;
    return v;
}
unsafe extern "C" fn get_section(
    mut line: *mut ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    if prefix_i(line, name) {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        if !(*value).is_null() {
            borg_formula_error(
                start,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** section repeated\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        let mut len: ::core::ffi::c_int = 0;
        let mut pos: *mut ::core::ffi::c_char = line;
        line = strchr(line, ':' as i32);
        if !line.is_null() {
            len = line.offset_from(pos) as ::core::ffi::c_long as ::core::ffi::c_int;
        } else {
            len = strlen(pos) as ::core::ffi::c_int;
        }
        if *pos.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ';' as i32
        {
            len -= 1;
        }
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, pos, len as size_t);
        buf[len as usize] = 0 as ::core::ffi::c_char;
        *value = string_make(&raw mut buf as *mut ::core::ffi::c_char);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn depth_free(mut depth: *mut borg_depth_line) {
    if !depth.is_null() {
        if !(*depth).condition.is_null() {
            mem_free((*depth).condition as *mut ::core::ffi::c_void);
            (*depth).condition = ::core::ptr::null_mut::<math_section>();
        }
        if !(*depth).reason.is_null() {
            string_free((*depth).reason);
            (*depth).reason = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !depth.is_null() {
            mem_free(depth as *mut ::core::ffi::c_void);
        }
        depth = ::core::ptr::null_mut::<borg_depth_line>();
    }
}
unsafe extern "C" fn parse_depth_line(
    mut restock: bool,
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut fail: bool = false_0 != 0;
    let mut s_depth: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_condition: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *line != 0 {
        if get_section(
            line,
            b"depth\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_depth,
            full_line,
        ) {
            line = line.offset(strlen(s_depth) as isize);
        }
        if get_section(
            line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_condition,
            full_line,
        ) {
            line = line.offset(strlen(s_condition) as isize);
        }
        if *line as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ':' as i32
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ';' as i32
        {
            let mut err: *mut ::core::ffi::c_char = string_make(format(
                b"** unparsed information '%s' in depth line\0" as *const u8
                    as *const ::core::ffi::c_char,
                line,
            ));
            borg_formula_error(
                start,
                full_line,
                b"depth\0" as *const u8 as *const ::core::ffi::c_char,
                err,
            );
            string_free(err);
            fail = true_0 != 0;
            break;
        } else {
            line = line.offset(1);
        }
    }
    if !fail && s_depth.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** depth section missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    if !fail && s_condition.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** condition section missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    let mut d: *mut borg_depth_line = ::core::ptr::null_mut::<borg_depth_line>();
    if !fail {
        d = mem_zalloc(::core::mem::size_of::<borg_depth_line>() as size_t) as *mut borg_depth_line;
    }
    if !s_depth.is_null() {
        if !fail {
            (*d).dlevel = parse_depth(s_depth) as int32_t;
            if (*d).dlevel < 0 as int32_t {
                borg_formula_error(
                    start,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** depth section can't be parsed\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fail = true_0 != 0;
            }
        }
        string_free(s_depth);
    }
    if !s_condition.is_null() {
        if !fail {
            (*d).condition = parse_condition(s_condition, full_line);
            (*d).reason = s_condition;
            if (*d).condition.is_null() {
                fail = true_0 != 0;
            }
        }
    }
    if !fail {
        if restock {
            borg_array_add(
                &raw mut borg_formulas.restock,
                d as *mut ::core::ffi::c_void,
            );
        } else {
            borg_array_add(&raw mut borg_formulas.depth, d as *mut ::core::ffi::c_void);
        }
    }
    if fail {
        depth_free(d);
    }
    return fail;
}
unsafe extern "C" fn parse_power_line(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut fail: bool = false_0 != 0;
    let mut s_range: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_reward: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_condition: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *line != 0 {
        if get_section(
            line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_range,
            full_line,
        ) {
            line = line.offset(strlen(s_range) as isize);
        }
        if get_section(
            line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_reward,
            full_line,
        ) {
            line = line.offset(strlen(s_reward) as isize);
        }
        if get_section(
            line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_value,
            full_line,
        ) {
            line = line.offset(strlen(s_value) as isize);
        }
        if get_section(
            line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_condition,
            full_line,
        ) {
            line = line.offset(strlen(s_condition) as isize);
        }
        if *line as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ':' as i32
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ';' as i32
        {
            let mut err: *mut ::core::ffi::c_char = string_make(format(
                b"** unparsed information '%s' in power line\0" as *const u8
                    as *const ::core::ffi::c_char,
                line,
            ));
            borg_formula_error(
                start,
                full_line,
                b"power\0" as *const u8 as *const ::core::ffi::c_char,
                err,
            );
            string_free(err);
            fail = true_0 != 0;
            break;
        } else {
            line = line.offset(1);
        }
    }
    if !fail && !s_range.is_null() && s_value.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"power\0" as *const u8 as *const ::core::ffi::c_char,
            b"** if range() is specified value() must also be specified\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    if !fail && s_reward.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"power\0" as *const u8 as *const ::core::ffi::c_char,
            b"** reward() must be specified specified\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    let mut p: *mut borg_power_line = ::core::ptr::null_mut::<borg_power_line>();
    if !fail {
        p = mem_zalloc(::core::mem::size_of::<borg_power_line>() as size_t) as *mut borg_power_line;
    }
    if !s_range.is_null() {
        if !fail {
            (*p).range = parse_range(s_range, full_line);
            if (*p).range.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_range);
    }
    if !s_reward.is_null() {
        if !fail {
            (*p).reward = parse_reward(s_reward, full_line);
            if (*p).reward.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_reward);
    }
    if !s_value.is_null() {
        if !fail {
            (*p).value = parse_value(s_value, full_line);
            if (*p).value.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_value);
    }
    if !s_condition.is_null() {
        if !fail {
            (*p).condition = parse_condition(s_condition, full_line);
            if (*p).condition.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_condition);
    }
    if !fail {
        borg_array_add(&raw mut borg_formulas.power, p as *mut ::core::ffi::c_void);
    }
    if fail as ::core::ffi::c_int != 0 && !p.is_null() {
        mem_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<borg_power_line>();
    }
    return fail;
}
unsafe extern "C" fn check_condition(
    mut condition: *mut math_section,
    mut range: ::core::ffi::c_int,
) -> bool {
    if !(*condition).is_calculation {
        return (*condition).calculation_index != 0 as int32_t;
    }
    return borg_calculate_dynamic((*condition).calculation_index as ::core::ffi::c_int, range)
        != 0;
}
unsafe extern "C" fn calc_power(mut p: *mut borg_power_line) -> int32_t {
    let mut total: int32_t = 0 as int32_t;
    if !(*p).condition.is_null() {
        if !check_condition((*p).condition, 0 as ::core::ffi::c_int) {
            return 0 as int32_t;
        }
    }
    if !(*p).range.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut end: ::core::ffi::c_int =
            if calculate_from_value((*p).value, 0 as ::core::ffi::c_int) > (*(*p).range).to {
                (*(*p).range).to as ::core::ffi::c_int
            } else {
                calculate_from_value((*p).value, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            };
        i = (*(*p).range).from as ::core::ffi::c_int;
        while i <= end {
            if (*(*p).reward).is_calculation {
                total += borg_calculate_dynamic(
                    (*(*p).reward).calculation_index as ::core::ffi::c_int,
                    i - 1 as ::core::ffi::c_int,
                );
            } else {
                total += (*(*p).reward).calculation_index;
            }
            i += 1;
        }
        return total;
    }
    if (*(*p).reward).is_calculation {
        total += borg_calculate_dynamic(
            (*(*p).reward).calculation_index as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    } else {
        total += (*(*p).reward).calculation_index;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn borg_power_dynamic() -> int32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut total: int32_t = 0 as int32_t;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.power.count {
        total += calc_power(*borg_formulas.power.items.offset(i as isize) as *mut borg_power_line);
        i += 1;
    }
    if borg_spell_stat() > 0 as ::core::ffi::c_int {
        total = (total as ::core::ffi::c_int
            + (100 as ::core::ffi::c_int
                - spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int) as int32_t;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_ZERO_FAIL as ::core::ffi::c_int,
    ) {
        if (spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize))
    .iqty
        != 0
    {
        let mut item: *mut borg_item = borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item;
        if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
            as ::core::ffi::c_int
            != 0
            && !(*item).ident
        {
            total = (total as ::core::ffi::c_long + 999999 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 200 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int
                - (borg_item_weight(borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) as *mut borg_item) as ::core::ffi::c_int
                    - 200 as ::core::ffi::c_int)
                    * 15 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 30 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 50 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
    }
    let mut cur_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    let mut max_wgt: ::core::ffi::c_int = (*(*player).class).magic.spell_weight;
    if borg_can_cast() as ::core::ffi::c_int != 0
        && (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut max_sp: ::core::ffi::c_int = *borg
            .trait_0
            .offset(BI_SP_ADJ as ::core::ffi::c_int as isize)
            / 100 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        max_sp -= (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int;
        if max_sp >= 300 as ::core::ffi::c_int && max_sp <= 350 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 400 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 200 as ::core::ffi::c_int && max_sp <= 299 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 800 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 100 as ::core::ffi::c_int && max_sp <= 199 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1600 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp <= 99 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 3200 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        <= 40 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 25 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            < 100000 as ::core::ffi::c_int
        && (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
            != TV_DIGGING as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_ADIGGER as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
    {
        total = (total as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    let mut book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book < 9 as ::core::ffi::c_int {
        if !(borg.amt_book[book as usize] == 0) {
            if (*(*(*player).class).magic.books.offset(book as isize)).dungeon {
                let mut what: ::core::ffi::c_int = 0;
                what = 0 as ::core::ffi::c_int;
                while what < 9 as ::core::ffi::c_int {
                    let mut as_0: *mut borg_magic = borg_get_spell_entry(book, what);
                    if as_0.is_null() {
                        break;
                    }
                    if !((*as_0).level as ::core::ffi::c_int
                        > *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize))
                    {
                        if !((*as_0).power as ::core::ffi::c_int
                            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize))
                        {
                            total = (total as ::core::ffi::c_long + 15000 as ::core::ffi::c_long)
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
                        >= *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                            + 2 as ::core::ffi::c_int)
                {
                    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while k < 1 as ::core::ffi::c_int
                        && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                    {
                        total = (total as ::core::ffi::c_long + 500000 as ::core::ffi::c_long)
                            as int32_t;
                        k += 1;
                    }
                    if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize)
                        > 5 as ::core::ffi::c_int
                    {
                        while k < 2 as ::core::ffi::c_int
                            && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                        {
                            total = (total as ::core::ffi::c_long + 10000 as ::core::ffi::c_long)
                                as int32_t;
                            k += 1;
                        }
                    }
                }
            }
        }
        book += 1;
    }
    if *borg
        .trait_0
        .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        let mut item_0: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
        i = (*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize);
        while i >= 0 as ::core::ffi::c_int {
            if (*borg_items.offset(i as isize)).iqty != 0 {
                item_0 = borg_items.offset(i as isize) as *mut borg_item;
                break;
            } else {
                i -= 1;
            }
        }
        if !(!item_0.is_null()
            && (*item_0).iqty as ::core::ffi::c_int != 0
            && (*item_0).aware as ::core::ffi::c_int != 0
            && ((*item_0).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                && ((*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_armor
                    && *borg
                        .trait_0
                        .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize)
                        < 1000 as ::core::ffi::c_int
                    && *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize)
                        != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_hit
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize)
                            != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_dam
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize)
                            != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_star_enchant_weapon
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_star_enchant_armor)
                || (*item_0).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && ((*item_0).sval as ::core::ffi::c_int == sv_potion_inc_str
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_int
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_wis
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_dex
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_con
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_all)))
        {
            total = (total as ::core::ffi::c_long
                - ((*borg
                    .trait_0
                    .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
                    - *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int)
                    / (*borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 10 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long) as int32_t;
        }
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepared_dynamic(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.depth.count {
        let mut d: *mut borg_depth_line =
            *borg_formulas.depth.items.offset(i as isize) as *mut borg_depth_line;
        if (*d).dlevel > depth as int32_t {
            break;
        }
        if check_condition((*d).condition, depth) {
            return (*d).reason;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_restock_dynamic(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.restock.count {
        let mut d: *mut borg_depth_line =
            *borg_formulas.restock.items.offset(i as isize) as *mut borg_depth_line;
        if (*d).dlevel > depth as int32_t {
            break;
        }
        if check_condition((*d).condition, depth) {
            return (*d).reason;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_load_formulas(mut fp: *mut ang_file) -> bool {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut formulas_off: bool = false_0 != 0;
    let mut section: e_section = SEC_NONE;
    let mut skip_open: bool = true_0 != 0;
    borg_formulas.restock.count = 0 as ::core::ffi::c_int;
    borg_formulas.depth.count = 0 as ::core::ffi::c_int;
    borg_formulas.power.count = 0 as ::core::ffi::c_int;
    if *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) == 0 {
        formulas_off = true_0 != 0;
    }
    while file_getl(
        fp,
        &raw mut buf as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t).wrapping_sub(1 as size_t),
    ) {
        let mut line: *mut ::core::ffi::c_char =
            borg_trim(&raw mut buf as *mut ::core::ffi::c_char);
        if *line.offset(0 as ::core::ffi::c_int as isize) == 0
            || *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if prefix_i(
            line,
            b"[END FORMULA SECTION]\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            break;
        }
        if formulas_off {
            continue;
        }
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            if !skip_open {
                formulas_off = true_0 != 0;
            }
            skip_open = false_0 != 0;
        } else if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ']' as i32
        {
            if section as ::core::ffi::c_uint
                == SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return formulas_off;
            }
            section = SEC_NONE;
        } else {
            if formulas_off {
                continue;
            }
            if prefix_i(line, b"Power\0" as *const u8 as *const ::core::ffi::c_char) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_POWER;
                skip_open = true_0 != 0;
            } else if prefix_i(
                line,
                b"Depth Requirement\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_DEPTH;
                skip_open = true_0 != 0;
            } else if prefix_i(
                line,
                b"Restock Requirement\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_RESTOCK;
                skip_open = true_0 != 0;
            } else if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                != ';' as i32
            {
                if section as ::core::ffi::c_uint
                    == SEC_DEPTH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"depth\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if section as ::core::ffi::c_uint
                    == SEC_POWER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"power\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if section as ::core::ffi::c_uint
                    == SEC_RESTOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"restock\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"base\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                formulas_off = true_0 != 0;
            } else {
                if section as ::core::ffi::c_uint
                    == SEC_DEPTH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_depth_line(
                        false_0 != 0,
                        line,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                }
                if section as ::core::ffi::c_uint
                    == SEC_RESTOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_depth_line(
                        true_0 != 0,
                        line,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                }
                if section as ::core::ffi::c_uint
                    == SEC_POWER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_power_line(line, &raw mut buf as *mut ::core::ffi::c_char);
                }
                if section as ::core::ffi::c_uint
                    == SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"base\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must be part of a Power or Depth section\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    formulas_off = true_0 != 0;
                }
            }
        }
    }
    if formulas_off as ::core::ffi::c_int != 0
        && *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) != 0
    {
        *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
    }
    return formulas_off;
}
unsafe extern "C" fn power_free(mut power: *mut borg_power_line) {
    if !power.is_null() {
        if !(*power).condition.is_null() {
            mem_free((*power).condition as *mut ::core::ffi::c_void);
            (*power).condition = ::core::ptr::null_mut::<math_section>();
        }
        if !(*power).range.is_null() {
            mem_free((*power).range as *mut ::core::ffi::c_void);
            (*power).range = ::core::ptr::null_mut::<range_sec>();
        }
        if !(*power).reward.is_null() {
            mem_free((*power).reward as *mut ::core::ffi::c_void);
            (*power).reward = ::core::ptr::null_mut::<math_section>();
        }
        if !(*power).value.is_null() {
            mem_free((*power).value as *mut ::core::ffi::c_void);
            (*power).value = ::core::ptr::null_mut::<value_sec>();
        }
        if !power.is_null() {
            mem_free(power as *mut ::core::ffi::c_void);
        }
        power = ::core::ptr::null_mut::<borg_power_line>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_formulas() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.restock.count {
        depth_free(*borg_formulas.restock.items.offset(i as isize) as *mut borg_depth_line);
        let ref mut fresh2 = *borg_formulas.restock.items.offset(i as isize);
        *fresh2 = NULL;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.depth.count {
        depth_free(*borg_formulas.depth.items.offset(i as isize) as *mut borg_depth_line);
        let ref mut fresh3 = *borg_formulas.depth.items.offset(i as isize);
        *fresh3 = NULL;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.power.count {
        power_free(*borg_formulas.power.items.offset(i as isize) as *mut borg_power_line);
        let ref mut fresh4 = *borg_formulas.power.items.offset(i as isize);
        *fresh4 = NULL;
        i += 1;
    }
    borg_formulas.restock.count = 0 as ::core::ffi::c_int;
    borg_formulas.depth.count = 0 as ::core::ffi::c_int;
    borg_formulas.power.count = 0 as ::core::ffi::c_int;
    calculations_free();
}
