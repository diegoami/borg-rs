pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};

use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_use_h::borg_equips_item;

pub use crate::headers::borg_magic_h::{
    borg_spell_legal, borg_spell_legal_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
    BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS,
    BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS,
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
pub use crate::headers::borg_store_h::{borg_shop, borg_shops};
pub use crate::headers::borg_trait_h::{
    borg_calc_blows, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
pub use crate::headers::list_kind_flags_h::{
    KF_EASY_KNOW, KF_GOOD, KF_INSTA_ART, KF_MAX, KF_NONE, KF_QUEST_ART, KF_RAND_BASE_RES,
    KF_RAND_HI_RES, KF_RAND_POWER, KF_RAND_RES_POWER, KF_RAND_SUSTAIN, KF_SHOOTS_ARROWS,
    KF_SHOOTS_BOLTS, KF_SHOOTS_SHOTS, KF_SHOW_DICE, KF_SHOW_MULT,
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
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
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
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
};
pub use crate::headers::obj_properties_h::{
    C2RustUnnamedH_20, C2RustUnnamedH_31, C2RustUnnamedH_36, C2RustUnnamedH_39, OBJ_MOD_MAX,
    OF_NONE, TV_MAX,
};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, k_info,
    object, object_base, object_kind, poss_item, C2RustUnnamedH_38, ELEM_MAX,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_flags, player_history,
    player_race, player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23,
    C2RustUnnamedH_34, PF_MAX, STAT_MAX,
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
pub static mut num_food: int16_t = 0;
#[no_mangle]
pub static mut num_fuel: int16_t = 0;
#[no_mangle]
pub static mut num_mold: int16_t = 0;
#[no_mangle]
pub static mut num_ident: int16_t = 0;
#[no_mangle]
pub static mut num_recall: int16_t = 0;
#[no_mangle]
pub static mut num_phase: int16_t = 0;
#[no_mangle]
pub static mut num_escape: int16_t = 0;
#[no_mangle]
pub static mut num_tele_staves: int16_t = 0;
#[no_mangle]
pub static mut num_teleport: int16_t = 0;
#[no_mangle]
pub static mut num_berserk: int16_t = 0;
#[no_mangle]
pub static mut num_teleport_level: int16_t = 0;
#[no_mangle]
pub static mut num_recharge: int16_t = 0;
#[no_mangle]
pub static mut num_cure_critical: int16_t = 0;
#[no_mangle]
pub static mut num_cure_serious: int16_t = 0;
#[no_mangle]
pub static mut num_pot_rheat: int16_t = 0;
#[no_mangle]
pub static mut num_pot_rcold: int16_t = 0;
#[no_mangle]
pub static mut num_missile: int16_t = 0;
#[no_mangle]
pub static mut num_book: [int16_t; 9] = [0; 9];
#[no_mangle]
pub static mut num_fix_stat: [int16_t; 5] = [0; 5];
#[no_mangle]
pub static mut home_stat_add: [int16_t; 5] = [0; 5];
#[no_mangle]
pub static mut num_fix_exp: int16_t = 0;
#[no_mangle]
pub static mut num_mana: int16_t = 0;
#[no_mangle]
pub static mut num_heal: int16_t = 0;
#[no_mangle]
pub static mut num_heal_true: int16_t = 0;
#[no_mangle]
pub static mut num_ezheal: int16_t = 0;
#[no_mangle]
pub static mut num_ezheal_true: int16_t = 0;
#[no_mangle]
pub static mut num_life: int16_t = 0;
#[no_mangle]
pub static mut num_life_true: int16_t = 0;
#[no_mangle]
pub static mut num_pfe: int16_t = 0;
#[no_mangle]
pub static mut num_glyph: int16_t = 0;
#[no_mangle]
pub static mut num_mass_genocide: int16_t = 0;
#[no_mangle]
pub static mut num_speed: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_a: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_d: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_h: int16_t = 0;
#[no_mangle]
pub static mut num_brand_weapon: int16_t = 0;
#[no_mangle]
pub static mut num_genocide: int16_t = 0;
#[no_mangle]
pub static mut num_artifact: int16_t = 0;
#[no_mangle]
pub static mut num_ego: int16_t = 0;
#[no_mangle]
pub static mut home_slot_free: int16_t = 0;
#[no_mangle]
pub static mut home_un_id: int16_t = 0;
#[no_mangle]
pub static mut home_damage: int16_t = 0;
#[no_mangle]
pub static mut num_duplicate_items: int16_t = 0;
#[no_mangle]
pub static mut num_slow_digest: int16_t = 0;
#[no_mangle]
pub static mut num_regenerate: int16_t = 0;
#[no_mangle]
pub static mut num_telepathy: int16_t = 0;
#[no_mangle]
pub static mut num_LIGHT: int16_t = 0;
#[no_mangle]
pub static mut num_see_inv: int16_t = 0;
#[no_mangle]
pub static mut num_invisible: int16_t = 0;
#[no_mangle]
pub static mut num_ffall: int16_t = 0;
#[no_mangle]
pub static mut num_free_act: int16_t = 0;
#[no_mangle]
pub static mut num_hold_life: int16_t = 0;
#[no_mangle]
pub static mut num_immune_acid: int16_t = 0;
#[no_mangle]
pub static mut num_immune_elec: int16_t = 0;
#[no_mangle]
pub static mut num_immune_fire: int16_t = 0;
#[no_mangle]
pub static mut num_immune_cold: int16_t = 0;
#[no_mangle]
pub static mut num_resist_acid: int16_t = 0;
#[no_mangle]
pub static mut num_resist_elec: int16_t = 0;
#[no_mangle]
pub static mut num_resist_fire: int16_t = 0;
#[no_mangle]
pub static mut num_resist_cold: int16_t = 0;
#[no_mangle]
pub static mut num_resist_pois: int16_t = 0;
#[no_mangle]
pub static mut num_resist_conf: int16_t = 0;
#[no_mangle]
pub static mut num_resist_sound: int16_t = 0;
#[no_mangle]
pub static mut num_resist_LIGHT: int16_t = 0;
#[no_mangle]
pub static mut num_resist_dark: int16_t = 0;
#[no_mangle]
pub static mut num_resist_chaos: int16_t = 0;
#[no_mangle]
pub static mut num_resist_disen: int16_t = 0;
#[no_mangle]
pub static mut num_resist_shard: int16_t = 0;
#[no_mangle]
pub static mut num_resist_nexus: int16_t = 0;
#[no_mangle]
pub static mut num_resist_blind: int16_t = 0;
#[no_mangle]
pub static mut num_resist_neth: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_str: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_int: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_wis: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_dex: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_con: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_all: int16_t = 0;
#[no_mangle]
pub static mut num_edged_weapon: int16_t = 0;
#[no_mangle]
pub static mut num_bad_gloves: int16_t = 0;
#[no_mangle]
pub static mut num_weapons: int16_t = 0;
#[no_mangle]
pub static mut num_bow: int16_t = 0;
#[no_mangle]
pub static mut num_rings: int16_t = 0;
#[no_mangle]
pub static mut num_neck: int16_t = 0;
#[no_mangle]
pub static mut num_armor: int16_t = 0;
#[no_mangle]
pub static mut num_cloaks: int16_t = 0;
#[no_mangle]
pub static mut num_shields: int16_t = 0;
#[no_mangle]
pub static mut num_hats: int16_t = 0;
#[no_mangle]
pub static mut num_gloves: int16_t = 0;
#[no_mangle]
pub static mut num_boots: int16_t = 0;
unsafe extern "C" fn borg_notice_home_clear(mut in_item: *mut borg_item, mut no_items: bool) {
    num_food = 0 as int16_t;
    num_fuel = 0 as int16_t;
    num_mold = 0 as int16_t;
    num_ident = 0 as int16_t;
    num_recall = 0 as int16_t;
    num_phase = 0 as int16_t;
    num_escape = 0 as int16_t;
    num_tele_staves = 0 as int16_t;
    num_teleport = 0 as int16_t;
    num_teleport_level = 0 as int16_t;
    num_recharge = 0 as int16_t;
    num_artifact = 0 as int16_t;
    num_ego = 0 as int16_t;
    num_invisible = 0 as int16_t;
    num_pfe = 0 as int16_t;
    num_glyph = 0 as int16_t;
    num_genocide = 0 as int16_t;
    num_mass_genocide = 0 as int16_t;
    num_berserk = 0 as int16_t;
    num_pot_rheat = 0 as int16_t;
    num_pot_rcold = 0 as int16_t;
    num_speed = 0 as int16_t;
    num_slow_digest = 0 as int16_t;
    num_regenerate = 0 as int16_t;
    num_telepathy = 0 as int16_t;
    num_see_inv = 0 as int16_t;
    num_ffall = 0 as int16_t;
    num_free_act = 0 as int16_t;
    num_hold_life = 0 as int16_t;
    num_immune_acid = 0 as int16_t;
    num_immune_elec = 0 as int16_t;
    num_immune_fire = 0 as int16_t;
    num_immune_cold = 0 as int16_t;
    num_resist_acid = 0 as int16_t;
    num_resist_elec = 0 as int16_t;
    num_resist_fire = 0 as int16_t;
    num_resist_cold = 0 as int16_t;
    num_resist_pois = 0 as int16_t;
    num_resist_conf = 0 as int16_t;
    num_resist_sound = 0 as int16_t;
    num_resist_LIGHT = 0 as int16_t;
    num_resist_dark = 0 as int16_t;
    num_resist_chaos = 0 as int16_t;
    num_resist_disen = 0 as int16_t;
    num_resist_shard = 0 as int16_t;
    num_resist_nexus = 0 as int16_t;
    num_resist_blind = 0 as int16_t;
    num_resist_neth = 0 as int16_t;
    num_sustain_str = 0 as int16_t;
    num_sustain_int = 0 as int16_t;
    num_sustain_wis = 0 as int16_t;
    num_sustain_dex = 0 as int16_t;
    num_sustain_con = 0 as int16_t;
    num_sustain_all = 0 as int16_t;
    home_stat_add[STAT_STR as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_INT as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_WIS as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_CON as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_weapons = 0 as int16_t;
    num_bow = 0 as int16_t;
    num_rings = 0 as int16_t;
    num_neck = 0 as int16_t;
    num_armor = 0 as int16_t;
    num_cloaks = 0 as int16_t;
    num_shields = 0 as int16_t;
    num_hats = 0 as int16_t;
    num_gloves = 0 as int16_t;
    num_boots = 0 as int16_t;
    num_LIGHT = 0 as int16_t;
    num_speed = 0 as int16_t;
    num_edged_weapon = 0 as int16_t;
    num_bad_gloves = 0 as int16_t;
    num_cure_critical = 0 as int16_t;
    num_cure_serious = 0 as int16_t;
    num_fix_exp = 0 as int16_t;
    num_mana = 0 as int16_t;
    num_heal = 0 as int16_t;
    num_ezheal = 0 as int16_t;
    num_life = 0 as int16_t;
    if in_item.is_null() && !no_items {
        num_ezheal_true = 0 as int16_t;
    }
    if in_item.is_null() && !no_items {
        num_heal_true = 0 as int16_t;
    }
    if in_item.is_null() && !no_items {
        num_life_true = 0 as int16_t;
    }
    num_missile = 0 as int16_t;
    num_book[0 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[1 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[2 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[3 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[4 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[5 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[6 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[7 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[8 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_enchant_to_a = 0 as int16_t;
    num_enchant_to_d = 0 as int16_t;
    num_enchant_to_h = 0 as int16_t;
    home_slot_free = 0 as int16_t;
    home_damage = 0 as int16_t;
    home_un_id = 0 as int16_t;
    num_duplicate_items = 0 as int16_t;
}
unsafe extern "C" fn borg_notice_home_dupe(
    mut item: *mut borg_item,
    mut check_sval: bool,
    mut i: ::core::ffi::c_int,
) {
    let mut dupe_count: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut item2: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut e_ptr: *mut ego_item = e_info.offset((*item).ego_idx as isize) as *mut ego_item;
    if borg_ego_has_random_power(e_ptr) {
        return;
    }
    if (*item).needs_ident {
        return;
    }
    dupe_count = (*item).iqty as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    x = 0 as ::core::ffi::c_int;
    while x < i {
        if x < (*z_info).store_inven_max as ::core::ffi::c_int {
            item2 = (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(x as isize) as *mut borg_item;
        } else {
            item2 = borg_items.offset(
                (x - (*z_info).store_inven_max as ::core::ffi::c_int
                    + (*z_info).pack_size as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
        }
        if !((*item2).iqty == 0 || !(*item2).aware) {
            if (*item).tval as ::core::ffi::c_int == (*item2).tval as ::core::ffi::c_int
                && (if check_sval as ::core::ffi::c_int != 0 {
                    ((*item).sval as ::core::ffi::c_int == (*item2).sval as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                } else {
                    true_0
                }) != 0
                && (*item).art_idx as ::core::ffi::c_int == (*item2).art_idx as ::core::ffi::c_int
                && (*item).ego_idx as ::core::ffi::c_int == (*item2).ego_idx as ::core::ffi::c_int
            {
                dupe_count += 1;
            }
        }
        x += 1;
    }
    if (*item).tval as ::core::ffi::c_int == TV_RING as ::core::ffi::c_int && dupe_count != 0 {
        dupe_count -= 1;
    }
    num_duplicate_items = (num_duplicate_items as ::core::ffi::c_int + dupe_count) as int16_t;
}
unsafe extern "C" fn borg_notice_home_aux(mut in_item: *mut borg_item, mut no_items: bool) {
    let mut i: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut home: *mut borg_shop = borg_shops.offset(BORG_HOME as isize) as *mut borg_shop;
    let mut f: [bitflag; 5] = [0; 5];
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).store_inven_max as ::core::ffi::c_int
            + ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                - (*z_info).pack_size as ::core::ffi::c_int)
    {
        if no_items {
            break;
        }
        if in_item.is_null() {
            if i < (*z_info).store_inven_max as ::core::ffi::c_int {
                item = (*home).ware.offset(i as isize) as *mut borg_item;
            } else {
                item = borg_items.offset(
                    (i - (*z_info).store_inven_max as ::core::ffi::c_int
                        + (*z_info).pack_size as ::core::ffi::c_int) as isize,
                ) as *mut borg_item;
            }
        } else {
            item = in_item;
        }
        if (*item).iqty == 0 {
            if i < (*z_info).store_inven_max as ::core::ffi::c_int {
                home_slot_free += 1;
            }
        } else if (*item).aware {
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
                OF_SLOW_DIGEST as ::core::ffi::c_int,
            ) {
                num_slow_digest = (num_slow_digest as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_REGEN as ::core::ffi::c_int,
            ) {
                num_regenerate = (num_regenerate as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                num_telepathy = (num_telepathy as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                num_see_inv = (num_see_inv as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
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
                OF_FEATHER as ::core::ffi::c_int,
            ) {
                num_ffall = (num_ffall as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                num_free_act = (num_free_act as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
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
                num_hold_life = (num_hold_life as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                num_resist_conf = (num_resist_conf as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                num_resist_blind = (num_resist_blind as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_fire = (num_immune_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_fire = (num_resist_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_acid = (num_immune_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_acid = (num_resist_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_cold = (num_immune_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_cold = (num_resist_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_elec = (num_immune_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_elec = (num_resist_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_acid = (num_resist_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_elec = (num_resist_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_fire = (num_resist_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_cold = (num_resist_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_pois = (num_resist_pois as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_sound = (num_resist_sound as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_LIGHT = (num_resist_LIGHT as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_dark = (num_resist_dark as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_chaos = (num_resist_chaos as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_disen = (num_resist_disen as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_shard = (num_resist_shard as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_nexus = (num_resist_nexus as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_neth = (num_resist_neth as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_STR as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_INT as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_WIS as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_DEX as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_CON as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_STR as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_INT as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_WIS as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_DEX as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_CON as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
            {
                num_sustain_all = (num_sustain_all as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_STR as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_INT as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_WIS as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_CON as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            num_speed = (num_speed as ::core::ffi::c_int
                + (*item).modifiers[OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*item).iqty as ::core::ffi::c_int) as int16_t;
            if (*item).art_idx != 0 {
                num_artifact = (num_artifact as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
            }
            if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
                as ::core::ffi::c_int
                != 0
                && (*item).needs_ident as ::core::ffi::c_int != 0
            {
                num_ego =
                    (num_ego as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int) as int16_t;
            }
            if (*item).needs_ident as ::core::ffi::c_int != 0
                && i < (*z_info).store_inven_max as ::core::ffi::c_int
            {
                home_un_id += 1;
            }
            match (*item).tval as ::core::ffi::c_int {
                16 | 17 => {
                    num_armor = (num_armor as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                18 => {
                    num_armor = (num_armor as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                15 => {
                    num_cloaks = (num_cloaks as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                14 => {
                    num_shields = (num_shields as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                12 | 13 => {
                    num_hats = (num_hats as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                11 => {
                    num_gloves = (num_gloves as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    home_damage = (home_damage as ::core::ffi::c_int
                        + (*item).to_d as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                27 => {
                    if (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .sval as ::core::ffi::c_int
                        == crate::bridge::st().item_val.sv_light_lantern
                    {
                        num_fuel = (num_fuel as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                19 => {
                    if (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .sval as ::core::ffi::c_int
                        == crate::bridge::st().item_val.sv_light_torch
                    {
                        num_fuel = (num_fuel as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                    if (*item).art_idx != 0 {
                        num_LIGHT = (num_LIGHT as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                10 => {
                    num_boots = (num_boots as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                9 | 8 | 7 => {
                    let mut num_blow: int16_t = 0;
                    num_weapons = (num_weapons as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    if flag_has(
                        &raw mut (*player).state.pflags as *mut bitflag,
                        (PF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        PF_BLESS_WEAPON as ::core::ffi::c_int,
                    ) {
                        if !((*item).tval as ::core::ffi::c_int == TV_HAFTED as ::core::ffi::c_int
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
                                OF_BLESSED as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int
                                != 0)
                        {
                            num_edged_weapon = (num_edged_weapon as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                    num_blow = borg_calc_blows(item) as int16_t;
                    if (*item).to_d as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                        || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                            < 15 as ::core::ffi::c_int
                    {
                        home_damage = (home_damage as ::core::ffi::c_int
                            + num_blow as ::core::ffi::c_int
                                * ((*item).dd as ::core::ffi::c_int
                                    * (*item).ds as ::core::ffi::c_int
                                    + (crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize]
                                        + (*item).to_d as ::core::ffi::c_int)))
                            as int16_t;
                    } else {
                        home_damage = (home_damage as ::core::ffi::c_int
                            + num_blow as ::core::ffi::c_int
                                * ((*item).dd as ::core::ffi::c_int
                                    * (*item).ds as ::core::ffi::c_int
                                    + (crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize]
                                        + 8 as ::core::ffi::c_int)))
                            as int16_t;
                    }
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                5 => {
                    num_bow = (num_bow as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                21 => {
                    num_rings = (num_rings as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                20 => {
                    num_neck = (num_neck as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                30 | 31 | 32 | 33 | 34 => {
                    if obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
                    {
                        if !flag_has(
                            &raw mut (*k_info.offset((*item).kind as isize)).kind_flags
                                as *mut bitflag,
                            (KF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            KF_GOOD as ::core::ffi::c_int,
                        ) {
                            num_book[(*item).sval as usize] = (num_book[(*item).sval as usize]
                                as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                }
                28 => {
                    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_food_ration {
                        num_food = (num_food as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_food_slime_mold {
                        num_mold = (num_mold as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_purging {
                        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_mush_restoring {
                        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                26 => {
                    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_critical {
                        num_cure_critical = (num_cure_critical as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_cure_serious {
                        num_cure_serious = (num_cure_serious as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_resist_heat {
                        num_pot_rheat = (num_pot_rheat as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_resist_cold {
                        num_pot_rcold = (num_pot_rcold as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_restore_life {
                        num_fix_exp = (num_fix_exp as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_restore_mana {
                        num_mana = (num_mana as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_healing {
                        num_heal = (num_heal as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_heal_true = (num_heal_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_star_healing {
                        num_ezheal = (num_ezheal as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_ezheal_true = (num_ezheal_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_life {
                        num_life = (num_life as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_life_true = (num_life_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_berserk {
                        num_berserk = (num_berserk as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_speed {
                        num_speed = (num_speed as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                25 => {
                    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_identify {
                        num_ident = (num_ident as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_phase_door {
                        num_phase = (num_phase as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_teleport {
                        num_teleport = (num_teleport as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_word_of_recall {
                        num_recall = (num_recall as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_armor {
                        num_enchant_to_a = (num_enchant_to_a as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_hit
                    {
                        num_enchant_to_h = (num_enchant_to_h as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_dam
                    {
                        num_enchant_to_d = (num_enchant_to_d as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_protection_from_evil {
                        num_pfe = (num_pfe as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_rune_of_protection {
                        num_glyph = (num_glyph as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_teleport_level {
                        num_teleport_level = (num_teleport_level as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_recharging {
                        num_recharge = (num_recharge as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_mass_banishment {
                        num_mass_genocide = (num_mass_genocide as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                24 => {
                    if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_recall {
                        num_recall = (num_recall as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                22 => {
                    if !((*item).pval as ::core::ffi::c_int <= 3 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                            > 30 as ::core::ffi::c_int)
                    {
                        if (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_staff_teleportation {
                            num_escape = (num_escape as ::core::ffi::c_int
                                + (*item).pval as ::core::ffi::c_int
                                    * (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                            num_tele_staves += 1;
                        }
                    }
                }
                2 | 3 | 4 => {
                    if !((*item).tval as ::core::ffi::c_int
                        != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
                    {
                        if !((*item).value <= 0 as int32_t) {
                            num_missile = (num_missile as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                }
                _ => {}
            }
            if !in_item.is_null() {
                break;
            }
        }
        i += 1;
    }
    if borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
        || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0
    {
        num_food = (num_food as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(IDENTIFY_RUNE) {
        num_ident = (num_ident as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) {
        num_enchant_to_h =
            (num_enchant_to_h as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
        num_enchant_to_d =
            (num_enchant_to_d as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(PROTECTION_FROM_EVIL) {
        num_pfe = (num_pfe as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(GLYPH_OF_WARDING) as ::core::ffi::c_int != 0
        || borg_equips_item(crate::bridge::st().act.glyph, false_0 != 0) as ::core::ffi::c_int != 0
    {
        num_glyph = (num_glyph as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(WORD_OF_RECALL) {
        num_recall = (num_recall as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(TELEPORT_LEVEL) {
        num_teleport_level =
            (num_teleport_level as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(RECHARGING) {
        num_recharge = (num_recharge as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] != 0 {
        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] != 0 {
        num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] != 0 {
        num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] != 0 {
        num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] != 0 {
        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    player_flags(player, &raw mut f as *mut bitflag);
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        num_slow_digest += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        num_ffall += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_LIGHT_2 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut f as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
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
        num_LIGHT += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        num_regenerate += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        num_telepathy += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        num_see_inv += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        num_free_act += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        num_hold_life += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        num_resist_conf += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        num_resist_blind += 1;
    }
    if (*(*player).race).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_fire += 1;
    }
    if (*(*player).race).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_acid += 1;
    }
    if (*(*player).race).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_cold += 1;
    }
    if (*(*player).race).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_elec += 1;
    }
    if (*(*player).race).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_acid += 1;
    }
    if (*(*player).race).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_elec += 1;
    }
    if (*(*player).race).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_fire += 1;
    }
    if (*(*player).race).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_cold += 1;
    }
    if (*(*player).race).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_pois += 1;
    }
    if (*(*player).race).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_LIGHT += 1;
    }
    if (*(*player).race).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_dark += 1;
    }
    if (*(*player).race).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_sound += 1;
    }
    if (*(*player).race).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_shard += 1;
    }
    if (*(*player).race).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_nexus += 1;
    }
    if (*(*player).race).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_neth += 1;
    }
    if (*(*player).race).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_chaos += 1;
    }
    if (*(*player).race).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_disen += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_STR as ::core::ffi::c_int,
    ) {
        num_sustain_str += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_INT as ::core::ffi::c_int,
    ) {
        num_sustain_int += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_WIS as ::core::ffi::c_int,
    ) {
        num_sustain_wis += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_DEX as ::core::ffi::c_int,
    ) {
        num_sustain_dex += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_CON as ::core::ffi::c_int,
    ) {
        num_sustain_con += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_home(mut in_item: *mut borg_item, mut no_items: bool) {
    borg_notice_home_clear(in_item, no_items);
    borg_notice_home_aux(in_item, no_items);
}
