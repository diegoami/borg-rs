pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_flow_kill_h::borg_race_death;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_inventory_h::borg_wield_slot;
pub use crate::headers::borg_item_h::{
    borg_item, borg_items, C2RustUnnamedH_44, BORG_CURSE_AIR_SWING, BORG_CURSE_ANNOYANCE,
    BORG_CURSE_ANTI_TELEPORTATION, BORG_CURSE_BURNING_UP, BORG_CURSE_CHILLED_TO_THE_BONE,
    BORG_CURSE_CLUMSINESS, BORG_CURSE_COWARDICE, BORG_CURSE_DEMON_SUMMON, BORG_CURSE_DRAGON_SUMMON,
    BORG_CURSE_DULLNESS, BORG_CURSE_ENVELOPING, BORG_CURSE_HALLUCINATION,
    BORG_CURSE_IMPAIR_HITPOINT_RECOVERY, BORG_CURSE_IMPAIR_MANA_RECOVERY, BORG_CURSE_IRRITATION,
    BORG_CURSE_MAX, BORG_CURSE_PARALYSIS, BORG_CURSE_POISON, BORG_CURSE_SICKLINESS,
    BORG_CURSE_SIREN, BORG_CURSE_SLOWNESS, BORG_CURSE_STEELSKIN, BORG_CURSE_STONE,
    BORG_CURSE_TELEPORTATION, BORG_CURSE_TREACHEROUS_WEAPON, BORG_CURSE_UNDEAD_SUMMON,
    BORG_CURSE_UNKNOWN, BORG_CURSE_VULNERABILITY, BORG_CURSE_WEAKNESS,
};
pub use crate::headers::borg_magic_h::{
    borg_spell_legal_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL,
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
    borg_uses_swaps, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_elements_h::{
    ELEM_ACID, ELEM_ARROW, ELEM_CHAOS, ELEM_COLD, ELEM_DARK, ELEM_DISEN, ELEM_ELEC, ELEM_FIRE,
    ELEM_FORCE, ELEM_GRAVITY, ELEM_HOLY_ORB, ELEM_ICE, ELEM_INERTIA, ELEM_LIGHT, ELEM_MANA,
    ELEM_METEOR, ELEM_MISSILE, ELEM_NETHER, ELEM_NEXUS, ELEM_PLASMA, ELEM_POIS, ELEM_SHARD,
    ELEM_SOUND, ELEM_TIME, ELEM_WATER,
};
pub use crate::headers::list_mon_race_flags_h::{
    RF_ANIMAL, RF_ATTR_CLEAR, RF_ATTR_FLICKER, RF_ATTR_MULTI, RF_ATTR_RAND, RF_BASH_DOOR,
    RF_CHAR_CLEAR, RF_CLEAR_WEB, RF_COLD_BLOOD, RF_DEMON, RF_DRAGON, RF_DROP_1, RF_DROP_2,
    RF_DROP_20, RF_DROP_3, RF_DROP_4, RF_DROP_40, RF_DROP_60, RF_DROP_GOOD, RF_DROP_GREAT,
    RF_EMPTY_MIND, RF_EVIL, RF_FEMALE, RF_FORCE_DEPTH, RF_FORCE_EXTRA, RF_FORCE_SLEEP,
    RF_FRIGHTENED, RF_GIANT, RF_GROUP_AI, RF_HURT_COLD, RF_HURT_FIRE, RF_HURT_LIGHT, RF_HURT_ROCK,
    RF_IM_ACID, RF_IM_COLD, RF_IM_DISEN, RF_IM_ELEC, RF_IM_FIRE, RF_IM_NETHER, RF_IM_NEXUS,
    RF_IM_PLASMA, RF_IM_POIS, RF_IM_WATER, RF_INVISIBLE, RF_KILL_BODY, RF_KILL_ITEM, RF_KILL_WALL,
    RF_MALE, RF_METAL, RF_MIMIC_INV, RF_MOVE_BODY, RF_MULTIPLY, RF_NAME_COMMA, RF_NEVER_BLOW,
    RF_NEVER_MOVE, RF_NONE, RF_NONLIVING, RF_NO_CONF, RF_NO_FEAR, RF_NO_HOLD, RF_NO_SLEEP,
    RF_NO_SLOW, RF_NO_STUN, RF_ONLY_GOLD, RF_ONLY_ITEM, RF_OPEN_DOOR, RF_ORC, RF_PASS_WALL,
    RF_PASS_WEB, RF_POWERFUL, RF_QUESTOR, RF_RAND_25, RF_RAND_50, RF_REGENERATE, RF_SEASONAL,
    RF_SMART, RF_SMASH_WALL, RF_SPIRIT, RF_STUPID, RF_TAKE_ITEM, RF_TROLL, RF_UNAWARE, RF_UNDEAD,
    RF_UNIQUE, RF_WEIRD_MIND,
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
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
};
pub use crate::headers::obj_properties_h::{
    C2RustUnnamedH_20, C2RustUnnamedH_31, C2RustUnnamedH_39, OBJ_MOD_MAX, OF_NONE, TV_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item, C2RustUnnamedH_38, ELEM_MAX,
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
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut weapon_swap: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut armour_swap: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut weapon_swap_value: int32_t = 0;
#[no_mangle]
pub static mut armour_swap_value: int32_t = 0;
#[no_mangle]
pub static mut decurse_weapon_swap: bool = false;
#[no_mangle]
pub static mut enchant_weapon_swap_to_h: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut enchant_weapon_swap_to_d: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut decurse_armour_swap: bool = false;
#[no_mangle]
pub static mut enchant_armour_swap_to_a: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut weapon_swap_digger: int16_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_animal: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_evil: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_undead: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_demon: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_orc: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_troll: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_giant: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slay_dragon: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_impact: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_brand_acid: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_brand_elec: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_brand_fire: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_brand_cold: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_brand_pois: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_see_infra: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_slow_digest: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_aggravate: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_bad_curse: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_regenerate: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_telepathy: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_light: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_see_invis: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_ffall: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_free_act: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_hold_life: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_immune_fire: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_immune_acid: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_immune_cold: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_immune_elec: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_acid: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_elec: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_fire: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_cold: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_pois: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_conf: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_sound: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_light: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_dark: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_chaos: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_disen: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_shard: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_nexus: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_blind: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_neth: uint8_t = 0;
#[no_mangle]
pub static mut weapon_swap_resist_fear: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_animal: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_evil: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_undead: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_demon: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_orc: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_troll: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_giant: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slay_dragon: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_impact: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_brand_acid: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_brand_elec: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_brand_fire: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_brand_cold: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_brand_pois: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_see_infra: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_slow_digest: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_aggravate: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_bad_curse: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_regenerate: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_telepathy: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_light: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_see_invis: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_ffall: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_free_act: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_hold_life: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_immune_fire: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_immune_acid: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_immune_cold: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_immune_elec: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_acid: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_elec: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_fire: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_cold: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_pois: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_conf: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_sound: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_light: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_dark: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_chaos: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_disen: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_shard: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_nexus: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_blind: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_neth: uint8_t = 0;
#[no_mangle]
pub static mut armour_swap_resist_fear: uint8_t = 0;
unsafe extern "C" fn borg_has_bad_curse(mut item: *mut borg_item) -> bool {
    if (*item).curses[BORG_CURSE_TELEPORTATION as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0
        || (*item).curses[BORG_CURSE_POISON as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_SIREN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_HALLUCINATION as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_PARALYSIS as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_DEMON_SUMMON as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_DRAGON_SUMMON as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_UNDEAD_SUMMON as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_STONE as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_ANTI_TELEPORTATION as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_TREACHEROUS_WEAPON as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        || (*item).curses[BORG_CURSE_UNKNOWN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_weapon_swap() {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut v: int32_t = -(1 as int32_t);
    let mut b_v: int32_t = -(1 as int32_t);
    let mut dam: ::core::ffi::c_int = 0;
    let mut damage: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    weapon_swap = 0 as ::core::ffi::c_int;
    weapon_swap_value = -(1 as ::core::ffi::c_int) as int32_t;
    let mut current_block_256: u64;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        v = -(1 as ::core::ffi::c_long) as int32_t;
        dam = 0 as ::core::ffi::c_int;
        damage = 0 as ::core::ffi::c_int;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !(borg_wield_slot(item) == -(1 as ::core::ffi::c_int)) {
                    if !(crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                        < 50 as ::core::ffi::c_int
                        && (*item).tval as ::core::ffi::c_int != TV_DIGGING as ::core::ffi::c_int)
                    {
                        if !borg_uses_swaps()
                            && (*item).tval as ::core::ffi::c_int
                                != TV_DIGGING as ::core::ffi::c_int
                        {
                            return;
                        }
                        weapon_swap_digger = 0 as int16_t;
                        weapon_swap_slay_animal = 0 as uint8_t;
                        weapon_swap_slay_evil = 0 as uint8_t;
                        weapon_swap_slay_undead = 0 as uint8_t;
                        weapon_swap_slay_demon = 0 as uint8_t;
                        weapon_swap_slay_orc = 0 as uint8_t;
                        weapon_swap_slay_troll = 0 as uint8_t;
                        weapon_swap_slay_giant = 0 as uint8_t;
                        weapon_swap_slay_dragon = 0 as uint8_t;
                        weapon_swap_impact = false_0 as uint8_t;
                        weapon_swap_brand_acid = false_0 as uint8_t;
                        weapon_swap_brand_elec = false_0 as uint8_t;
                        weapon_swap_brand_fire = false_0 as uint8_t;
                        weapon_swap_brand_cold = false_0 as uint8_t;
                        weapon_swap_brand_pois = false_0 as uint8_t;
                        weapon_swap_see_infra = false_0 as uint8_t;
                        weapon_swap_slow_digest = false_0 as uint8_t;
                        weapon_swap_aggravate = false_0 as uint8_t;
                        weapon_swap_bad_curse = false_0 as uint8_t;
                        weapon_swap_regenerate = false_0 as uint8_t;
                        weapon_swap_telepathy = false_0 as uint8_t;
                        weapon_swap_light = false_0 as uint8_t;
                        weapon_swap_see_invis = false_0 as uint8_t;
                        weapon_swap_ffall = false_0 as uint8_t;
                        weapon_swap_free_act = false_0 as uint8_t;
                        weapon_swap_hold_life = false_0 as uint8_t;
                        weapon_swap_immune_fire = false_0 as uint8_t;
                        weapon_swap_immune_acid = false_0 as uint8_t;
                        weapon_swap_immune_cold = false_0 as uint8_t;
                        weapon_swap_immune_elec = false_0 as uint8_t;
                        weapon_swap_resist_acid = false_0 as uint8_t;
                        weapon_swap_resist_elec = false_0 as uint8_t;
                        weapon_swap_resist_fire = false_0 as uint8_t;
                        weapon_swap_resist_cold = false_0 as uint8_t;
                        weapon_swap_resist_pois = false_0 as uint8_t;
                        weapon_swap_resist_conf = false_0 as uint8_t;
                        weapon_swap_resist_sound = false_0 as uint8_t;
                        weapon_swap_resist_light = false_0 as uint8_t;
                        weapon_swap_resist_dark = false_0 as uint8_t;
                        weapon_swap_resist_chaos = false_0 as uint8_t;
                        weapon_swap_resist_disen = false_0 as uint8_t;
                        weapon_swap_resist_shard = false_0 as uint8_t;
                        weapon_swap_resist_nexus = false_0 as uint8_t;
                        weapon_swap_resist_blind = false_0 as uint8_t;
                        weapon_swap_resist_neth = false_0 as uint8_t;
                        decurse_weapon_swap = false_0 != 0;
                        match (*item).tval as ::core::ffi::c_int {
                            7 | 8 | 9 | 6 => {
                                if flag_has(
                                    &raw mut (*item).flags as *mut bitflag,
                                    (OF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    OF_DIG_1 as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int
                                    != 0
                                    || flag_has(
                                        &raw mut (*item).flags as *mut bitflag,
                                        (OF_MAX as ::core::ffi::c_int as size_t)
                                            .wrapping_add(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            )
                                            .wrapping_sub(1 as size_t)
                                            .wrapping_div(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            ),
                                        OF_DIG_2 as ::core::ffi::c_int,
                                    ) as ::core::ffi::c_int
                                        != 0
                                    || flag_has(
                                        &raw mut (*item).flags as *mut bitflag,
                                        (OF_MAX as ::core::ffi::c_int as size_t)
                                            .wrapping_add(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            )
                                            .wrapping_sub(1 as size_t)
                                            .wrapping_div(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            ),
                                        OF_DIG_3 as ::core::ffi::c_int,
                                    ) as ::core::ffi::c_int
                                        != 0
                                {
                                    if (*item).value <= 0 as int32_t {
                                        current_block_256 = 17179679302217393232;
                                    } else if (*item).cursed {
                                        current_block_256 = 17179679302217393232;
                                    } else {
                                        if !borg_spell_legal_fail(
                                            TURN_STONE_TO_MUD,
                                            40 as ::core::ffi::c_int,
                                        ) && !flag_has(
                                            &raw mut (*borg_items
                                                .offset((*z_info).pack_size as isize))
                                            .flags
                                                as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_DIG_1 as ::core::ffi::c_int,
                                        ) && !flag_has(
                                            &raw mut (*borg_items
                                                .offset((*z_info).pack_size as isize))
                                            .flags
                                                as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_DIG_2 as ::core::ffi::c_int,
                                        ) && !flag_has(
                                            &raw mut (*borg_items
                                                .offset((*z_info).pack_size as isize))
                                            .flags
                                                as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_DIG_3 as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_digger = (*item).pval;
                                        }
                                        current_block_256 = 1434579379687443766;
                                    }
                                } else {
                                    current_block_256 = 1434579379687443766;
                                }
                                match current_block_256 {
                                    17179679302217393232 => {}
                                    _ => {
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_IMPACT as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_impact = true_0 as uint8_t;
                                        }
                                        weapon_swap_slay_animal = (*item).slays
                                            [RF_ANIMAL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_evil = (*item).slays
                                            [RF_EVIL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_undead = (*item).slays
                                            [RF_UNDEAD as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_demon = (*item).slays
                                            [RF_DEMON as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_orc = (*item).slays
                                            [RF_ORC as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_troll = (*item).slays
                                            [RF_TROLL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_giant = (*item).slays
                                            [RF_GIANT as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        weapon_swap_slay_dragon = (*item).slays
                                            [RF_DRAGON as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize]
                                        {
                                            weapon_swap_brand_acid = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize]
                                        {
                                            weapon_swap_brand_elec = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize]
                                        {
                                            weapon_swap_brand_fire = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize]
                                        {
                                            weapon_swap_brand_cold = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize]
                                        {
                                            weapon_swap_brand_pois = true_0 as uint8_t;
                                        }
                                        weapon_swap_see_infra = (weapon_swap_see_infra
                                            as ::core::ffi::c_int
                                            + (*item).modifiers
                                                [OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int)
                                            as uint8_t;
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_SLOW_DIGEST as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_slow_digest = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_AGGRAVATE as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_aggravate = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_REGEN as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_regenerate = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_TELEPATHY as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_telepathy = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_LIGHT_2 as ::core::ffi::c_int,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                            || flag_has(
                                                &raw mut (*item).flags as *mut bitflag,
                                                (OF_MAX as ::core::ffi::c_int as size_t)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    )
                                                    .wrapping_sub(1 as size_t)
                                                    .wrapping_div(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    ),
                                                OF_LIGHT_3 as ::core::ffi::c_int,
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            weapon_swap_light = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_SEE_INVIS as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_see_invis = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_FEATHER as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_ffall = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_FREE_ACT as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_free_act = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_HOLD_LIFE as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_hold_life = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_PROT_CONF as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_resist_conf = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_PROT_BLIND as ::core::ffi::c_int,
                                        ) {
                                            weapon_swap_resist_blind = true_0 as uint8_t;
                                        }
                                        if borg_has_bad_curse(item) {
                                            weapon_swap_bad_curse = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            weapon_swap_immune_fire = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            weapon_swap_immune_acid = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            weapon_swap_immune_cold = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            weapon_swap_immune_elec = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_acid = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_elec = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_fire = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_cold = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_pois = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_SOUND as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_sound = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_LIGHT as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_light = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_dark = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_CHAOS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_chaos = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_DISEN as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_disen = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_SHARD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_shard = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_NEXUS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_nexus = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_NETHER as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            weapon_swap_resist_neth = true_0 as uint8_t;
                                        }
                                        if (*item).uncursable {
                                            decurse_weapon_swap = true_0 != 0;
                                        }
                                        damage = (((*item).dd as ::core::ffi::c_int
                                            * (*item).ds as ::core::ffi::c_int)
                                            as ::core::ffi::c_long
                                            * 25 as ::core::ffi::c_long)
                                            as ::core::ffi::c_int;
                                        v = (v as ::core::ffi::c_int
                                            + damage
                                                * (crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize] + 1 as ::core::ffi::c_int))
                                            as int32_t;
                                        v = (v as ::core::ffi::c_long
                                            + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
                                                + (*item).to_h as ::core::ffi::c_int)
                                                as ::core::ffi::c_long
                                                * 100 as ::core::ffi::c_long)
                                            as int32_t;
                                        v = (v as ::core::ffi::c_long
                                            + (crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize]
                                                + (*item).to_d as ::core::ffi::c_int)
                                                as ::core::ffi::c_long
                                                * 75 as ::core::ffi::c_long)
                                            as int32_t;
                                        dam = damage
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        dam = damage
                                            * 2 as ::core::ffi::c_int
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        if crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
                                                == 0
                                            && weapon_swap_slay_animal as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_animal as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_evil as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * weapon_swap_slay_evil as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        dam = damage
                                            * 3 as ::core::ffi::c_int
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_orc as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * weapon_swap_slay_orc as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_troll as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 2 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_undead as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_undead as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_demon as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_demon as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slay_giant as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_giant as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
                                                == 0
                                            && weapon_swap_slay_dragon as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_dragon as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_brand_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 4 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_brand_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 5 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_brand_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_brand_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_POIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_brand_pois as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                                == 0
                                            && weapon_swap_slay_orc as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * weapon_swap_slay_orc as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                                == 0
                                            && weapon_swap_slay_troll as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * weapon_swap_slay_troll as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                            <= 40 as ::core::ffi::c_int
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                >= 25 as ::core::ffi::c_int
                                            && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                                                < 100000 as ::core::ffi::c_int
                                            && weapon_swap_digger as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + (weapon_swap_digger as ::core::ffi::c_long
                                                    * 3500 as ::core::ffi::c_long
                                                    + 1000 as ::core::ffi::c_long))
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_slow_digest as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if weapon_swap_aggravate != 0 {
                                            v = (v as ::core::ffi::c_long
                                                - 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if weapon_swap_bad_curse != 0 {
                                            v = (v as ::core::ffi::c_long
                                                - 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if decurse_weapon_swap {
                                            v = (v as ::core::ffi::c_long
                                                - 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_regenerate as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_telepathy as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_light as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_see_invis as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 50000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_ffall as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_free_act as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                < 50 as ::core::ffi::c_int
                                            && weapon_swap_hold_life as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_immune_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 70000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_immune_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 30000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_immune_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 50000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_immune_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 25000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 6000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 4000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 3000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if weapon_swap_resist_fire as ::core::ffi::c_int != 0
                                            && weapon_swap_resist_acid as ::core::ffi::c_int != 0
                                            && weapon_swap_resist_elec as ::core::ffi::c_int != 0
                                            && weapon_swap_resist_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_pois as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 20000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_conf as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_sound as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_light as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 800 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_dark as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 800 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_chaos as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_disen as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_shard as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_nexus as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_blind as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_neth as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5500 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize]
                                            == 0
                                            && weapon_swap_resist_fear as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5500 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                            >= 75 as ::core::ffi::c_int
                                            && (crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                                == 0
                                                && weapon_swap_immune_cold as ::core::ffi::c_int
                                                    != 0
                                                || crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] == 0
                                                    && weapon_swap_immune_fire
                                                        as ::core::ffi::c_int
                                                        != 0)
                                        {
                                            if *borg_race_death.offset(crate::bridge::st().borg.id.tarrasque as isize)
                                                as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                                    == 0
                                                    && weapon_swap_immune_cold as ::core::ffi::c_int
                                                        != 0
                                                {
                                                    v = (v as ::core::ffi::c_long
                                                        + 90000 as ::core::ffi::c_long)
                                                        as int32_t;
                                                }
                                                if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                                    == 0
                                                    && weapon_swap_immune_fire as ::core::ffi::c_int
                                                        != 0
                                                {
                                                    v = (v as ::core::ffi::c_long
                                                        + 90000 as ::core::ffi::c_long)
                                                        as int32_t;
                                                }
                                            }
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 55 as ::core::ffi::c_int
                                            && weapon_swap_resist_neth as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 60 as ::core::ffi::c_int
                                            && weapon_swap_resist_chaos as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 60 as ::core::ffi::c_int
                                            && weapon_swap_resist_disen as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if !(v <= 1000 as int32_t) {
                                            if !(v < b_v) {
                                                b_i = i;
                                                b_v = v;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        i += 1;
    }
    weapon_swap_slay_animal = 0 as uint8_t;
    weapon_swap_slay_evil = 0 as uint8_t;
    weapon_swap_slay_undead = 0 as uint8_t;
    weapon_swap_slay_demon = 0 as uint8_t;
    weapon_swap_slay_orc = 0 as uint8_t;
    weapon_swap_slay_troll = 0 as uint8_t;
    weapon_swap_slay_giant = 0 as uint8_t;
    weapon_swap_slay_dragon = 0 as uint8_t;
    weapon_swap_impact = false_0 as uint8_t;
    weapon_swap_brand_acid = false_0 as uint8_t;
    weapon_swap_brand_elec = false_0 as uint8_t;
    weapon_swap_brand_fire = false_0 as uint8_t;
    weapon_swap_brand_cold = false_0 as uint8_t;
    weapon_swap_brand_pois = false_0 as uint8_t;
    weapon_swap_see_infra = false_0 as uint8_t;
    weapon_swap_slow_digest = false_0 as uint8_t;
    weapon_swap_aggravate = false_0 as uint8_t;
    weapon_swap_bad_curse = false_0 as uint8_t;
    weapon_swap_regenerate = false_0 as uint8_t;
    weapon_swap_telepathy = false_0 as uint8_t;
    weapon_swap_light = false_0 as uint8_t;
    weapon_swap_see_invis = false_0 as uint8_t;
    weapon_swap_ffall = false_0 as uint8_t;
    weapon_swap_free_act = false_0 as uint8_t;
    weapon_swap_hold_life = false_0 as uint8_t;
    weapon_swap_immune_fire = false_0 as uint8_t;
    weapon_swap_immune_acid = false_0 as uint8_t;
    weapon_swap_immune_cold = false_0 as uint8_t;
    weapon_swap_immune_elec = false_0 as uint8_t;
    weapon_swap_resist_acid = false_0 as uint8_t;
    weapon_swap_resist_elec = false_0 as uint8_t;
    weapon_swap_resist_fire = false_0 as uint8_t;
    weapon_swap_resist_cold = false_0 as uint8_t;
    weapon_swap_resist_pois = false_0 as uint8_t;
    weapon_swap_resist_conf = false_0 as uint8_t;
    weapon_swap_resist_sound = false_0 as uint8_t;
    weapon_swap_resist_light = false_0 as uint8_t;
    weapon_swap_resist_dark = false_0 as uint8_t;
    weapon_swap_resist_chaos = false_0 as uint8_t;
    weapon_swap_resist_disen = false_0 as uint8_t;
    weapon_swap_resist_shard = false_0 as uint8_t;
    weapon_swap_resist_nexus = false_0 as uint8_t;
    weapon_swap_resist_blind = false_0 as uint8_t;
    weapon_swap_resist_neth = false_0 as uint8_t;
    decurse_weapon_swap = false_0 != 0;
    enchant_weapon_swap_to_h = 0 as ::core::ffi::c_int;
    enchant_weapon_swap_to_d = 0 as ::core::ffi::c_int;
    if b_i == -(1 as ::core::ffi::c_int) {
        return;
    }
    weapon_swap_value = b_v;
    weapon_swap = b_i + 1 as ::core::ffi::c_int;
    item = borg_items.offset(b_i as isize) as *mut borg_item;
    if (borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int)
        && (*item).tval as ::core::ffi::c_int != TV_DIGGING as ::core::ffi::c_int
    {
        if ((*item).to_h as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            enchant_weapon_swap_to_h +=
                10 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
        }
        if ((*item).to_d as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            enchant_weapon_swap_to_d +=
                10 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
        }
    } else if (*item).tval as ::core::ffi::c_int != TV_DIGGING as ::core::ffi::c_int {
        if ((*item).to_h as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            enchant_weapon_swap_to_h +=
                8 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
        }
        if ((*item).to_d as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            enchant_weapon_swap_to_d +=
                8 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
        }
    }
    weapon_swap_slay_animal = (*item).slays[RF_ANIMAL as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_evil = (*item).slays[RF_EVIL as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_undead = (*item).slays[RF_UNDEAD as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_demon = (*item).slays[RF_DEMON as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_orc = (*item).slays[RF_ORC as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_troll = (*item).slays[RF_TROLL as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_giant = (*item).slays[RF_GIANT as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_dragon = (*item).slays[RF_DRAGON as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_undead = (*item).slays[RF_UNDEAD as ::core::ffi::c_int as usize] as uint8_t;
    weapon_swap_slay_demon = (*item).slays[RF_DEMON as ::core::ffi::c_int as usize] as uint8_t;
    if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize] {
        weapon_swap_brand_acid = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize] {
        weapon_swap_brand_elec = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize] {
        weapon_swap_brand_fire = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize] {
        weapon_swap_brand_cold = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize] {
        weapon_swap_brand_pois = true_0 as uint8_t;
    }
    weapon_swap_see_infra = (weapon_swap_see_infra as ::core::ffi::c_int
        + (*item).modifiers[OBJ_MOD_INFRA as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        as uint8_t;
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_IMPACT as ::core::ffi::c_int,
    ) {
        weapon_swap_impact = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        weapon_swap_slow_digest = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AGGRAVATE as ::core::ffi::c_int,
    ) {
        weapon_swap_aggravate = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        weapon_swap_regenerate = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        weapon_swap_telepathy = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_LIGHT_2 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*item).flags as *mut bitflag,
            (OF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            OF_LIGHT_3 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        weapon_swap_light = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        weapon_swap_see_invis = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        weapon_swap_ffall = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        weapon_swap_free_act = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        weapon_swap_hold_life = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        weapon_swap_resist_conf = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        weapon_swap_resist_blind = true_0 as uint8_t;
    }
    if borg_has_bad_curse(item) {
        weapon_swap_bad_curse = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        weapon_swap_immune_fire = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        weapon_swap_immune_acid = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        weapon_swap_immune_cold = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        weapon_swap_immune_elec = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_acid = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_elec = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_fire = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_cold = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_pois = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_sound = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_light = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_dark = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_chaos = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_disen = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_shard = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_nexus = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        weapon_swap_resist_neth = true_0 as uint8_t;
    }
    if (*item).uncursable {
        decurse_weapon_swap = true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_armour_swap() {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut v: int32_t = -(1 as ::core::ffi::c_long) as int32_t;
    let mut b_v: int32_t = 0 as int32_t;
    let mut dam: ::core::ffi::c_int = 0;
    let mut damage: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    armour_swap = 0 as ::core::ffi::c_int;
    armour_swap_value = -(1 as ::core::ffi::c_int) as int32_t;
    if !borg_uses_swaps() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        v = -(1 as ::core::ffi::c_long) as int32_t;
        dam = 0 as ::core::ffi::c_int;
        damage = 0 as ::core::ffi::c_int;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !(borg_wield_slot(item) == -(1 as ::core::ffi::c_int)) {
                    if !(crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                        < 50 as ::core::ffi::c_int)
                    {
                        if !((*item).cursed as ::core::ffi::c_int != 0 && !(*item).uncursable) {
                            if !(*item).one_ring {
                                armour_swap_slay_animal = 0 as uint8_t;
                                armour_swap_slay_evil = 0 as uint8_t;
                                armour_swap_slay_undead = 0 as uint8_t;
                                armour_swap_slay_demon = 0 as uint8_t;
                                armour_swap_slay_orc = 0 as uint8_t;
                                armour_swap_slay_troll = 0 as uint8_t;
                                armour_swap_slay_giant = 0 as uint8_t;
                                armour_swap_slay_dragon = 0 as uint8_t;
                                armour_swap_impact = false_0 as uint8_t;
                                armour_swap_brand_acid = false_0 as uint8_t;
                                armour_swap_brand_elec = false_0 as uint8_t;
                                armour_swap_brand_fire = false_0 as uint8_t;
                                armour_swap_brand_cold = false_0 as uint8_t;
                                armour_swap_brand_pois = false_0 as uint8_t;
                                armour_swap_see_infra = false_0 as uint8_t;
                                armour_swap_slow_digest = false_0 as uint8_t;
                                armour_swap_aggravate = false_0 as uint8_t;
                                armour_swap_bad_curse = false_0 as uint8_t;
                                armour_swap_regenerate = false_0 as uint8_t;
                                armour_swap_telepathy = false_0 as uint8_t;
                                armour_swap_light = false_0 as uint8_t;
                                armour_swap_see_invis = false_0 as uint8_t;
                                armour_swap_ffall = false_0 as uint8_t;
                                armour_swap_free_act = false_0 as uint8_t;
                                armour_swap_hold_life = false_0 as uint8_t;
                                armour_swap_immune_fire = false_0 as uint8_t;
                                armour_swap_immune_acid = false_0 as uint8_t;
                                armour_swap_immune_cold = false_0 as uint8_t;
                                armour_swap_immune_elec = false_0 as uint8_t;
                                armour_swap_resist_acid = false_0 as uint8_t;
                                armour_swap_resist_elec = false_0 as uint8_t;
                                armour_swap_resist_fire = false_0 as uint8_t;
                                armour_swap_resist_cold = false_0 as uint8_t;
                                armour_swap_resist_pois = false_0 as uint8_t;
                                armour_swap_resist_conf = false_0 as uint8_t;
                                armour_swap_resist_sound = false_0 as uint8_t;
                                armour_swap_resist_light = false_0 as uint8_t;
                                armour_swap_resist_dark = false_0 as uint8_t;
                                armour_swap_resist_chaos = false_0 as uint8_t;
                                armour_swap_resist_disen = false_0 as uint8_t;
                                armour_swap_resist_shard = false_0 as uint8_t;
                                armour_swap_resist_nexus = false_0 as uint8_t;
                                armour_swap_resist_blind = false_0 as uint8_t;
                                armour_swap_resist_neth = false_0 as uint8_t;
                                decurse_armour_swap = false_0 != 0;
                                match (*item).tval as ::core::ffi::c_int {
                                    21 | 20 | 10 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => {
                                        armour_swap_slay_animal = (*item).slays
                                            [RF_ANIMAL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_evil = (*item).slays
                                            [RF_EVIL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_undead = (*item).slays
                                            [RF_UNDEAD as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_demon = (*item).slays
                                            [RF_DEMON as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_orc = (*item).slays
                                            [RF_ORC as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_troll = (*item).slays
                                            [RF_TROLL as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_giant = (*item).slays
                                            [RF_GIANT as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        armour_swap_slay_dragon = (*item).slays
                                            [RF_DRAGON as ::core::ffi::c_int as usize]
                                            as uint8_t;
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_IMPACT as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_impact = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize]
                                        {
                                            armour_swap_brand_acid = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize]
                                        {
                                            armour_swap_brand_elec = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize]
                                        {
                                            armour_swap_brand_fire = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize]
                                        {
                                            armour_swap_brand_cold = true_0 as uint8_t;
                                        }
                                        if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize]
                                        {
                                            armour_swap_brand_pois = true_0 as uint8_t;
                                        }
                                        armour_swap_see_infra = (armour_swap_see_infra
                                            as ::core::ffi::c_int
                                            + (*item).modifiers
                                                [OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int)
                                            as uint8_t;
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_SLOW_DIGEST as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_slow_digest = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_AGGRAVATE as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_aggravate = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_REGEN as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_regenerate = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_TELEPATHY as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_telepathy = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_LIGHT_2 as ::core::ffi::c_int,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                            || flag_has(
                                                &raw mut (*item).flags as *mut bitflag,
                                                (OF_MAX as ::core::ffi::c_int as size_t)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    )
                                                    .wrapping_sub(1 as size_t)
                                                    .wrapping_div(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    ),
                                                OF_LIGHT_3 as ::core::ffi::c_int,
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            armour_swap_light = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_SEE_INVIS as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_see_invis = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_FEATHER as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_ffall = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_FREE_ACT as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_free_act = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_HOLD_LIFE as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_hold_life = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_PROT_CONF as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_resist_conf = true_0 as uint8_t;
                                        }
                                        if flag_has(
                                            &raw mut (*item).flags as *mut bitflag,
                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                )
                                                .wrapping_sub(1 as size_t)
                                                .wrapping_div(
                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                        .wrapping_mul(8 as size_t),
                                                ),
                                            OF_PROT_BLIND as ::core::ffi::c_int,
                                        ) {
                                            armour_swap_resist_blind = true_0 as uint8_t;
                                        }
                                        if borg_has_bad_curse(item) {
                                            armour_swap_bad_curse = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            armour_swap_immune_fire = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            armour_swap_immune_acid = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            armour_swap_immune_cold = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            == 3 as ::core::ffi::c_int
                                        {
                                            armour_swap_immune_elec = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_acid = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_elec = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_fire = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_cold = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_pois = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_SOUND as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_sound = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_LIGHT as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_light = true_0 as uint8_t;
                                        }
                                        if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_dark = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_CHAOS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_chaos = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_DISEN as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_disen = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_SHARD as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_shard = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_NEXUS as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_nexus = true_0 as uint8_t;
                                        }
                                        if (*item).el_info
                                            [ELEM_NETHER as ::core::ffi::c_int as usize]
                                            .res_level
                                            as ::core::ffi::c_int
                                            > 0 as ::core::ffi::c_int
                                        {
                                            armour_swap_resist_neth = true_0 as uint8_t;
                                        }
                                        if (*item).uncursable {
                                            decurse_armour_swap = true_0 != 0;
                                        }
                                        damage = (((*item).dd as ::core::ffi::c_int
                                            * (*item).ds as ::core::ffi::c_int)
                                            as ::core::ffi::c_long
                                            * 35 as ::core::ffi::c_long)
                                            as ::core::ffi::c_int;
                                        v = (v as ::core::ffi::c_int
                                            + damage
                                                * (crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize] + 1 as ::core::ffi::c_int))
                                            as int32_t;
                                        v = (v as ::core::ffi::c_long
                                            + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
                                                + (*item).to_h as ::core::ffi::c_int)
                                                as ::core::ffi::c_long
                                                * 100 as ::core::ffi::c_long)
                                            as int32_t;
                                        v = (v as ::core::ffi::c_long
                                            + (crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize]
                                                + (*item).to_d as ::core::ffi::c_int)
                                                as ::core::ffi::c_long
                                                * 35 as ::core::ffi::c_long)
                                            as int32_t;
                                        dam = damage
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        dam = damage
                                            * 2 as ::core::ffi::c_int
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        if crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
                                                == 0
                                            && armour_swap_slay_animal as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_animal as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_evil as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * armour_swap_slay_evil as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        dam = damage
                                            * 3 as ::core::ffi::c_int
                                            * crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
                                        if crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_undead as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_undead as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_demon as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_demon as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_giant as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_giant as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
                                                == 0
                                            && armour_swap_slay_dragon as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_dragon as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_brand_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 4 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_brand_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 5 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_brand_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_brand_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WB_POIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_brand_pois as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * 3 as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_orc as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * armour_swap_slay_orc as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slay_troll as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_troll as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                                == 0
                                            && armour_swap_slay_orc as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam * armour_swap_slay_orc as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                                                == 0
                                            && armour_swap_slay_troll as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_int
                                                + dam
                                                    * armour_swap_slay_troll as ::core::ffi::c_int
                                                    / 2 as ::core::ffi::c_int)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_slow_digest as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if armour_swap_aggravate != 0 {
                                            v = (v as ::core::ffi::c_long
                                                - 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if armour_swap_bad_curse != 0 {
                                            v = (v as ::core::ffi::c_long
                                                - 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if decurse_armour_swap {
                                            v = (v as ::core::ffi::c_long
                                                - 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_regenerate as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_telepathy as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_light as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_see_invis as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 50000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_ffall as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_free_act as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                < 50 as ::core::ffi::c_int
                                            && armour_swap_hold_life as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_immune_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 70000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_immune_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 30000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_immune_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 50000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_immune_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 25000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_fire as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_acid as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 6000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 4000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_elec as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 3000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if armour_swap_resist_fire as ::core::ffi::c_int != 0
                                            && armour_swap_resist_acid as ::core::ffi::c_int != 0
                                            && armour_swap_resist_elec as ::core::ffi::c_int != 0
                                            && armour_swap_resist_cold as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 10000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_pois as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 20000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_conf as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_sound as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 2000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_light as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 800 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_dark as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 800 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_chaos as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 8000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_disen as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_shard as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_nexus as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_blind as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize]
                                            == 0
                                            && armour_swap_resist_neth as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 5500 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                            >= 75 as ::core::ffi::c_int
                                            && (crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                                == 0
                                                && armour_swap_immune_cold as ::core::ffi::c_int
                                                    != 0
                                                || crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] == 0
                                                    && armour_swap_immune_fire
                                                        as ::core::ffi::c_int
                                                        != 0)
                                        {
                                            if *borg_race_death.offset(crate::bridge::st().borg.id.tarrasque as isize)
                                                as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize]
                                                    == 0
                                                    && armour_swap_immune_cold as ::core::ffi::c_int
                                                        != 0
                                                {
                                                    v = (v as ::core::ffi::c_long
                                                        + 90000 as ::core::ffi::c_long)
                                                        as int32_t;
                                                }
                                                if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                                    == 0
                                                    && armour_swap_immune_fire as ::core::ffi::c_int
                                                        != 0
                                                {
                                                    v = (v as ::core::ffi::c_long
                                                        + 90000 as ::core::ffi::c_long)
                                                        as int32_t;
                                                }
                                            }
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 55 as ::core::ffi::c_int
                                            && weapon_swap_resist_neth == 0
                                            && armour_swap_resist_neth as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 105000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 60 as ::core::ffi::c_int
                                            && weapon_swap_resist_chaos == 0
                                            && armour_swap_resist_chaos as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 104000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize]
                                            == 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                                + 1 as ::core::ffi::c_int
                                                >= 60 as ::core::ffi::c_int
                                            && weapon_swap_resist_disen == 0
                                            && armour_swap_resist_disen as ::core::ffi::c_int != 0
                                        {
                                            v = (v as ::core::ffi::c_long
                                                + 100000 as ::core::ffi::c_long)
                                                as int32_t;
                                        }
                                        if !(v <= 1000 as int32_t) {
                                            if !(b_i >= 0 as ::core::ffi::c_int && v < b_v) {
                                                b_i = i;
                                                b_v = v;
                                                armour_swap_value = v;
                                                armour_swap = i + 1 as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    armour_swap_slay_animal = 0 as uint8_t;
    armour_swap_slay_evil = 0 as uint8_t;
    armour_swap_slay_undead = 0 as uint8_t;
    armour_swap_slay_demon = 0 as uint8_t;
    armour_swap_slay_orc = 0 as uint8_t;
    armour_swap_slay_troll = 0 as uint8_t;
    armour_swap_slay_giant = 0 as uint8_t;
    armour_swap_slay_dragon = 0 as uint8_t;
    armour_swap_impact = false_0 as uint8_t;
    armour_swap_brand_acid = false_0 as uint8_t;
    armour_swap_brand_elec = false_0 as uint8_t;
    armour_swap_brand_fire = false_0 as uint8_t;
    armour_swap_brand_cold = false_0 as uint8_t;
    armour_swap_brand_pois = false_0 as uint8_t;
    armour_swap_see_infra = false_0 as uint8_t;
    armour_swap_slow_digest = false_0 as uint8_t;
    armour_swap_aggravate = false_0 as uint8_t;
    armour_swap_bad_curse = false_0 as uint8_t;
    armour_swap_regenerate = false_0 as uint8_t;
    armour_swap_telepathy = false_0 as uint8_t;
    armour_swap_light = false_0 as uint8_t;
    armour_swap_see_invis = false_0 as uint8_t;
    armour_swap_ffall = false_0 as uint8_t;
    armour_swap_free_act = false_0 as uint8_t;
    armour_swap_hold_life = false_0 as uint8_t;
    armour_swap_immune_fire = false_0 as uint8_t;
    armour_swap_immune_acid = false_0 as uint8_t;
    armour_swap_immune_cold = false_0 as uint8_t;
    armour_swap_immune_elec = false_0 as uint8_t;
    armour_swap_resist_acid = false_0 as uint8_t;
    armour_swap_resist_elec = false_0 as uint8_t;
    armour_swap_resist_fire = false_0 as uint8_t;
    armour_swap_resist_cold = false_0 as uint8_t;
    armour_swap_resist_pois = false_0 as uint8_t;
    armour_swap_resist_conf = false_0 as uint8_t;
    armour_swap_resist_sound = false_0 as uint8_t;
    armour_swap_resist_light = false_0 as uint8_t;
    armour_swap_resist_dark = false_0 as uint8_t;
    armour_swap_resist_chaos = false_0 as uint8_t;
    armour_swap_resist_disen = false_0 as uint8_t;
    armour_swap_resist_shard = false_0 as uint8_t;
    armour_swap_resist_nexus = false_0 as uint8_t;
    armour_swap_resist_blind = false_0 as uint8_t;
    armour_swap_resist_neth = false_0 as uint8_t;
    decurse_armour_swap = false_0 != 0;
    if b_i == -(1 as ::core::ffi::c_int) {
        return;
    }
    item = borg_items.offset(b_i as isize) as *mut borg_item;
    armour_swap_slay_animal = (*item).slays[RF_ANIMAL as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_evil = (*item).slays[RF_EVIL as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_undead = (*item).slays[RF_UNDEAD as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_demon = (*item).slays[RF_DEMON as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_orc = (*item).slays[RF_ORC as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_troll = (*item).slays[RF_TROLL as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_giant = (*item).slays[RF_GIANT as ::core::ffi::c_int as usize] as uint8_t;
    armour_swap_slay_dragon = (*item).slays[RF_DRAGON as ::core::ffi::c_int as usize] as uint8_t;
    if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize] {
        armour_swap_brand_acid = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize] {
        armour_swap_brand_elec = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize] {
        armour_swap_brand_fire = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize] {
        armour_swap_brand_cold = true_0 as uint8_t;
    }
    if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize] {
        armour_swap_brand_pois = true_0 as uint8_t;
    }
    armour_swap_see_infra = (armour_swap_see_infra as ::core::ffi::c_int
        + (*item).modifiers[OBJ_MOD_INFRA as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        as uint8_t;
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_IMPACT as ::core::ffi::c_int,
    ) {
        armour_swap_impact = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        armour_swap_slow_digest = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AGGRAVATE as ::core::ffi::c_int,
    ) {
        armour_swap_aggravate = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        armour_swap_regenerate = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        armour_swap_telepathy = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_LIGHT_2 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*item).flags as *mut bitflag,
            (OF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            OF_LIGHT_3 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        armour_swap_light = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        armour_swap_see_invis = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        armour_swap_ffall = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        armour_swap_free_act = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        armour_swap_hold_life = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        armour_swap_resist_conf = true_0 as uint8_t;
    }
    if flag_has(
        &raw mut (*item).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        armour_swap_resist_blind = true_0 as uint8_t;
    }
    if borg_has_bad_curse(item) {
        armour_swap_bad_curse = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        armour_swap_immune_fire = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        armour_swap_immune_acid = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        armour_swap_immune_cold = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        armour_swap_immune_elec = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_acid = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_elec = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_fire = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_cold = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_pois = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_sound = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_light = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_dark = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_chaos = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_disen = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_shard = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_nexus = true_0 as uint8_t;
    }
    if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        armour_swap_resist_neth = true_0 as uint8_t;
    }
    if (*item).uncursable {
        decurse_armour_swap = true_0 != 0;
    }
    enchant_armour_swap_to_a = 0 as ::core::ffi::c_int;
    if (*item).tval as ::core::ffi::c_int >= TV_LIGHT as ::core::ffi::c_int {
        return;
    }
    if borg_spell_legal_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
    {
        if ((*item).to_a as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            enchant_armour_swap_to_a +=
                10 as ::core::ffi::c_int - (*item).to_a as ::core::ffi::c_int;
        }
    } else if ((*item).to_a as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        enchant_armour_swap_to_a += 8 as ::core::ffi::c_int - (*item).to_a as ::core::ffi::c_int;
    }
}
