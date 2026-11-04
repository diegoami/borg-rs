pub const BTH_PLUS_ADJ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6;
pub const CLASS_RANGER: ::core::ffi::c_int = 7;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


pub use crate::headers::borg_danger_h::{
    borg_danger, borg_danger_one_kill, borg_fear_region, BORG_MONBLOW, MONBLOW_ACID,
    MONBLOW_BLACK_BREATH, MONBLOW_BLIND, MONBLOW_COLD, MONBLOW_CONFUSE, MONBLOW_DISENCHANT,
    MONBLOW_DRAIN_CHARGES, MONBLOW_EAT_FOOD, MONBLOW_EAT_GOLD, MONBLOW_EAT_ITEM, MONBLOW_EAT_LIGHT,
    MONBLOW_ELEC, MONBLOW_EXP_10, MONBLOW_EXP_20, MONBLOW_EXP_40, MONBLOW_EXP_80, MONBLOW_FIRE,
    MONBLOW_HALLU, MONBLOW_HURT, MONBLOW_LOSE_ALL, MONBLOW_LOSE_CON, MONBLOW_LOSE_DEX,
    MONBLOW_LOSE_INT, MONBLOW_LOSE_STR, MONBLOW_LOSE_WIS, MONBLOW_NONE, MONBLOW_PARALYZE,
    MONBLOW_POISON, MONBLOW_SHATTER, MONBLOW_TERRIFY, MONBLOW_UNDEFINED,
};
pub use crate::headers::borg_fight_attack_h::{
    C2RustUnnamedH_19, C2RustUnnamedH_32, BF_ACT_ACID_BALL, BF_ACT_ACID_BOLT, BF_ACT_ACID_BOLT2,
    BF_ACT_ACID_BOLT3, BF_ACT_ARROW, BF_ACT_COLD_BALL100, BF_ACT_COLD_BALL160, BF_ACT_COLD_BALL2,
    BF_ACT_COLD_BALL50, BF_ACT_COLD_BOLT, BF_ACT_COLD_BOLT2, BF_ACT_CONFUSE2, BF_ACT_DISPEL_ALL,
    BF_ACT_DISPEL_EVIL, BF_ACT_DISPEL_EVIL60, BF_ACT_DISPEL_UNDEAD, BF_ACT_DRAGON_BALANCE,
    BF_ACT_DRAGON_BLUE, BF_ACT_DRAGON_CHAOS, BF_ACT_DRAGON_GOLD, BF_ACT_DRAGON_GREEN,
    BF_ACT_DRAGON_LAW, BF_ACT_DRAGON_MULTIHUED, BF_ACT_DRAGON_POWER, BF_ACT_DRAGON_RED,
    BF_ACT_DRAGON_SHINING, BF_ACT_DRAIN_LIFE1, BF_ACT_DRAIN_LIFE2, BF_ACT_DRAIN_LIFE3,
    BF_ACT_DRAIN_LIFE4, BF_ACT_ELEC_BALL, BF_ACT_ELEC_BALL2, BF_ACT_ELEC_BOLT, BF_ACT_FEAR_MONSTER,
    BF_ACT_FIRE_BALL, BF_ACT_FIRE_BALL2, BF_ACT_FIRE_BALL200, BF_ACT_FIRE_BALL72, BF_ACT_FIRE_BOLT,
    BF_ACT_FIRE_BOLT2, BF_ACT_FIRE_BOLT3, BF_ACT_LIGHT_BEAM, BF_ACT_LOSCONF, BF_ACT_LOSSLEEP,
    BF_ACT_LOSSLOW, BF_ACT_MANA_BOLT, BF_ACT_MISSILE, BF_ACT_MON_CONFUSE, BF_ACT_MON_SLOW,
    BF_ACT_RING_ACID, BF_ACT_RING_FIRE, BF_ACT_RING_ICE, BF_ACT_RING_LIGHTNING, BF_ACT_SLEEPII,
    BF_ACT_SLEEP_ALL, BF_ACT_STAFF_HOLY, BF_ACT_STARLIGHT, BF_ACT_STARLIGHT2, BF_ACT_STAR_BALL,
    BF_ACT_STINKING_CLOUD, BF_ACT_WONDER, BF_DRAGON_BALANCE, BF_DRAGON_BLACK, BF_DRAGON_BLUE,
    BF_DRAGON_CHAOS, BF_DRAGON_GOLD, BF_DRAGON_GREEN, BF_DRAGON_LAW, BF_DRAGON_MULTIHUED,
    BF_DRAGON_POWER, BF_DRAGON_RED, BF_DRAGON_SHINING, BF_DRAGON_WHITE, BF_LAUNCH, BF_MAX,
    BF_OBJECT, BF_PRAYER_DISPEL_LIFE, BF_PRAYER_DISP_EVIL, BF_PRAYER_DISP_SPIRITS,
    BF_PRAYER_DISP_UNDEAD, BF_PRAYER_HOLY_ORB_BALL, BF_PRAYER_HOLY_WORD, BF_REST, BF_RING_ACID,
    BF_RING_FIRE, BF_RING_ICE, BF_RING_LIGHTNING, BF_ROD_ACID_BALL, BF_ROD_ACID_BOLT,
    BF_ROD_COLD_BALL, BF_ROD_COLD_BOLT, BF_ROD_DRAIN_LIFE, BF_ROD_ELEC_BALL, BF_ROD_ELEC_BOLT,
    BF_ROD_FIRE_BALL, BF_ROD_FIRE_BOLT, BF_ROD_LIGHT_BEAM, BF_ROD_SLEEP_MONSTER,
    BF_ROD_SLOW_MONSTER, BF_ROD_UNKNOWN, BF_SPELL_ACID_SPRAY, BF_SPELL_ANNIHILATE,
    BF_SPELL_BLIND_CREATURE, BF_SPELL_COLD_BOLT, BF_SPELL_COLD_STORM, BF_SPELL_CONFUSE_MONSTER,
    BF_SPELL_CRUSH, BF_SPELL_CURSE, BF_SPELL_DARK_SPEAR, BF_SPELL_DISENCHANT,
    BF_SPELL_EARTH_RISING, BF_SPELL_ELECTRIC_ARC, BF_SPELL_EXPLOSION, BF_SPELL_FIRE_BALL,
    BF_SPELL_FRIGHTEN, BF_SPELL_HOWL_OF_THE_DAMNED, BF_SPELL_LEAP_INTO_BATTLE,
    BF_SPELL_LIGHTNING_STRIKE, BF_SPELL_LIGHT_BEAM, BF_SPELL_LIGHT_OF_VARDA,
    BF_SPELL_MAGIC_MISSILE, BF_SPELL_MAGIC_MISSILE_RESERVE, BF_SPELL_MAIM_FOE, BF_SPELL_MANA_BOLT,
    BF_SPELL_MANA_STORM, BF_SPELL_METEOR_SWARM, BF_SPELL_NETHER_BOLT, BF_SPELL_RIFT,
    BF_SPELL_RIVER_OF_LIGHTNING, BF_SPELL_SHOCK_WAVE, BF_SPELL_SLEEP_EVIL, BF_SPELL_SLEEP_III,
    BF_SPELL_SLOW_MONSTER, BF_SPELL_SPEAR_OF_OROME, BF_SPELL_STINK_CLOUD, BF_SPELL_STONE_TO_MUD,
    BF_SPELL_STORM_OF_DARKNESS, BF_SPELL_TAP_UNLIFE, BF_SPELL_THRUST_AWAY, BF_SPELL_TRANCE,
    BF_SPELL_UNLEASH_CHAOS, BF_SPELL_VAMPIRE_STRIKE, BF_SPELL_VOLCANIC_ERUPTION,
    BF_SPELL_WHIRLWIND_ATTACK, BF_STAFF_DISPEL_EVIL, BF_STAFF_HOLINESS, BF_STAFF_POWER,
    BF_STAFF_SLEEP_MONSTERS, BF_STAFF_SLOW_MONSTERS, BF_THRUST, BF_WAND_ACID_BALL,
    BF_WAND_ACID_BOLT, BF_WAND_ANNIHILATION, BF_WAND_COLD_BALL, BF_WAND_COLD_BOLT,
    BF_WAND_CONFUSE_MONSTER, BF_WAND_DRAGON_COLD, BF_WAND_DRAGON_FIRE, BF_WAND_DRAIN_LIFE,
    BF_WAND_ELEC_BALL, BF_WAND_ELEC_BOLT, BF_WAND_FEAR_MONSTER, BF_WAND_FIRE_BALL,
    BF_WAND_FIRE_BOLT, BF_WAND_HOLD_MONSTER, BF_WAND_LIGHT_BEAM, BF_WAND_MAGIC_MISSILE,
    BF_WAND_SLOW_MONSTER, BF_WAND_STINKING_CLOUD, BF_WAND_UNKNOWN, BF_WAND_WONDER,
    BORG_ATTACK_ACID, BORG_ATTACK_ARROW, BORG_ATTACK_AWAY_ALL, BORG_ATTACK_AWAY_ALL_MORGOTH,
    BORG_ATTACK_AWAY_EVIL, BORG_ATTACK_AWAY_UNDEAD, BORG_ATTACK_CHAOS, BORG_ATTACK_COLD,
    BORG_ATTACK_CONFU, BORG_ATTACK_CURSE, BORG_ATTACK_DARK, BORG_ATTACK_DARK_WEAK,
    BORG_ATTACK_DISEN, BORG_ATTACK_DISP_ALL, BORG_ATTACK_DISP_EVIL, BORG_ATTACK_DISP_SPIRITS,
    BORG_ATTACK_DISP_UNDEAD, BORG_ATTACK_DRAIN_LIFE, BORG_ATTACK_ELEC, BORG_ATTACK_ELEC_STRIKE,
    BORG_ATTACK_FIRE, BORG_ATTACK_FORCE, BORG_ATTACK_GRAVITY, BORG_ATTACK_HOLY_ORB,
    BORG_ATTACK_HOLY_WORD, BORG_ATTACK_ICE, BORG_ATTACK_INERTIA, BORG_ATTACK_KILL_DOOR,
    BORG_ATTACK_KILL_TRAP, BORG_ATTACK_KILL_WALL, BORG_ATTACK_LIGHT, BORG_ATTACK_LIGHT_WEAK,
    BORG_ATTACK_MAKE_DOOR, BORG_ATTACK_MAKE_TRAP, BORG_ATTACK_MAKE_WALL, BORG_ATTACK_MANA,
    BORG_ATTACK_METEOR, BORG_ATTACK_MISSILE, BORG_ATTACK_NETHER, BORG_ATTACK_NEXUS,
    BORG_ATTACK_OLD_CLONE, BORG_ATTACK_OLD_CONF, BORG_ATTACK_OLD_DRAIN, BORG_ATTACK_OLD_HEAL,
    BORG_ATTACK_OLD_POLY, BORG_ATTACK_OLD_SLEEP, BORG_ATTACK_OLD_SLOW, BORG_ATTACK_OLD_SPEED,
    BORG_ATTACK_PLASMA, BORG_ATTACK_POIS, BORG_ATTACK_SHARD, BORG_ATTACK_SLEEP_EVIL,
    BORG_ATTACK_SOUND, BORG_ATTACK_TAP_UNLIFE, BORG_ATTACK_TIME, BORG_ATTACK_TURN_ALL,
    BORG_ATTACK_TURN_EVIL, BORG_ATTACK_TURN_UNDEAD, BORG_ATTACK_WATER,
};
use crate::headers::borg_flow_h::{borg_flow_y, borg_temp_n, borg_temp_x, borg_temp_y};
pub use crate::headers::borg_flow_kill_h::{
    borg_kill, borg_kills, borg_kills_cnt, borg_kills_nxt, borg_race_name,
};
use crate::headers::borg_flow_misc_h::borg_extract_dir;
pub use crate::headers::borg_flow_take_h::{borg_take, borg_takes};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_inventory_h::{borg_slot, borg_wield_slot};
use crate::headers::borg_io_h::{borg_keypress, borg_note, borg_queue_direction};

pub use crate::headers::borg_item_h::{borg_item};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;
use crate::headers::borg_item_use_h::{
    borg_activate_dragon, borg_activate_failure, borg_activate_item, borg_activate_ring,
    borg_aim_wand, borg_equips_dragon, borg_equips_item, borg_equips_ring, borg_equips_rod,
    borg_equips_staff_fail, borg_use_staff, borg_zap_rod,
};

pub use crate::headers::borg_magic_h::{
    borg_get_spell_power, borg_spell, borg_spell_fail, borg_spell_legal, borg_spell_legal_fail,
    borg_spell_okay, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_projection_h::{
    borg_distance, borg_inc_motion, borg_los, borg_offset_projectable, borg_projectable_pure,
    borg_target,
};
pub use crate::headers::borg_trait_h::{
    borg_attacking, borg_confuse_spell, borg_crush_spell, borg_fear_mon_spell, borg_simulate,
    borg_sleep_spell, borg_sleep_spell_ii, borg_slow_spell, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW,
    BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX,
    BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE,
    BI_AEZHEAL, BI_AFUEL, BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE,
    BI_ALL_NEED_ID, BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED,
    BI_AMISSILES_SPECIAL, BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE,
    BI_APHASE, BI_ARECHARGE, BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2,
    BI_ASHROOM, BI_ASPEED, BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL,
    BI_ATPORTOTHER, BI_AWIS, BI_BART, BI_BID, BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON,
    BI_CDEPTH, BI_CDEX, BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV,
    BI_CRSAIRSWING, BI_CRSANNOY, BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL,
    BI_CRSENVELOPING, BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP,
    BI_CRSIRRITATION, BI_CRSMPIMP, BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA,
    BI_CRSSICK, BI_CRSSIREN, BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE,
    BI_CRSTWEP, BI_CRSUNKNO, BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP, BI_CWIS,
    BI_DAM_RED, BI_DEV, BI_DEX, BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY, BI_ESP,
    BI_EXTRA_BLOWS, BI_FAIL1, BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD,
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
use crate::headers::borg_update_h::borg_detect_wall;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, distance, heatmap, monster_group, square,
    square_in_bounds, square_in_bounds_fully, trap, C2RustUnnamedH_18, FEAT_MAX,
};
use crate::headers::generate_h::get_angle_to_grid;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
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
pub use crate::headers::list_mon_spells_h::{
    RSF_ARROW, RSF_BA_ACID, RSF_BA_COLD, RSF_BA_DARK, RSF_BA_ELEC, RSF_BA_FIRE, RSF_BA_HOLY,
    RSF_BA_LIGHT, RSF_BA_MANA, RSF_BA_NETH, RSF_BA_POIS, RSF_BA_SHAR, RSF_BA_WATE, RSF_BE_ELEC,
    RSF_BE_NETH, RSF_BLIND, RSF_BLINK, RSF_BOLT, RSF_BOULDER, RSF_BO_ACID, RSF_BO_COLD,
    RSF_BO_ELEC, RSF_BO_FIRE, RSF_BO_ICE, RSF_BO_MANA, RSF_BO_NETH, RSF_BO_PLAS, RSF_BO_POIS,
    RSF_BO_WATE, RSF_BRAIN_SMASH, RSF_BR_ACID, RSF_BR_CHAO, RSF_BR_COLD, RSF_BR_DARK, RSF_BR_DISE,
    RSF_BR_ELEC, RSF_BR_FIRE, RSF_BR_GRAV, RSF_BR_INER, RSF_BR_LIGHT, RSF_BR_MANA, RSF_BR_NETH,
    RSF_BR_NEXU, RSF_BR_PLAS, RSF_BR_POIS, RSF_BR_SHAR, RSF_BR_SOUN, RSF_BR_TIME, RSF_BR_WALL,
    RSF_CONF, RSF_DARKNESS, RSF_DRAIN_MANA, RSF_FORGET, RSF_HASTE, RSF_HEAL, RSF_HEAL_KIN,
    RSF_HOLD, RSF_MAX, RSF_MIND_BLAST, RSF_MISSILE, RSF_NONE, RSF_SCARE, RSF_SHAPECHANGE, RSF_SHOT,
    RSF_SHRIEK, RSF_SLOW, RSF_SPIT, RSF_STORM, RSF_S_AINU, RSF_S_ANIMAL, RSF_S_DEMON, RSF_S_DRAGON,
    RSF_S_HI_DEMON, RSF_S_HI_DRAGON, RSF_S_HI_UNDEAD, RSF_S_HOUND, RSF_S_HYDRA, RSF_S_KIN,
    RSF_S_MONSTER, RSF_S_MONSTERS, RSF_S_SPIDER, RSF_S_UNDEAD, RSF_S_UNIQUE, RSF_S_WRAITH,
    RSF_TELE_AWAY, RSF_TELE_LEVEL, RSF_TELE_SELF_TO, RSF_TELE_TO, RSF_TPORT, RSF_TRAPS, RSF_WEAVE,
    RSF_WHIP, RSF_WOUND,
};
pub use crate::headers::list_options_h::{
    OPT_animate_flicker, OPT_auto_more, OPT_autoexplore_commands, OPT_birth_ai_learn,
    OPT_birth_connect_stairs, OPT_birth_feelings, OPT_birth_force_descend, OPT_birth_know_flavors,
    OPT_birth_know_runes, OPT_birth_levels_persist, OPT_birth_lose_arts, OPT_birth_no_artifacts,
    OPT_birth_no_recall, OPT_birth_no_selling, OPT_birth_percent_damage, OPT_birth_randarts,
    OPT_birth_stacking, OPT_birth_start_kit, OPT_center_player, OPT_cheat_hear, OPT_cheat_live,
    OPT_cheat_room, OPT_cheat_xtra, OPT_disturb_near, OPT_effective_speed, OPT_highlight_player,
    OPT_hp_changes_color, OPT_hybrid_walls, OPT_mouse_movement, OPT_none, OPT_notify_recharge,
    OPT_pickup_always, OPT_pickup_inven, OPT_purple_uniques, OPT_rogue_like_commands,
    OPT_score_hear, OPT_score_live, OPT_score_room, OPT_score_xtra, OPT_show_damage,
    OPT_show_flavors, OPT_show_target, OPT_solid_walls, OPT_use_old_target, OPT_use_sound,
    OPT_view_yellow_light,
};
pub use crate::headers::list_stats_h::{STAT_CON, STAT_DEX, STAT_INT, STAT_STR, STAT_WIS};
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
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, C2RustUnnamedH_30, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN,
    MON_GROUP_BODYGUARD, MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
    RF_MAX,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, TV_MAX};
use crate::headers::obj_slays_h::{
    brands, player_has_temporary_brand, player_has_temporary_slay, slays,
};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, brand, curse_data, effect, ego_item, element_info, flavor, k_info,
    object, object_base, object_kind, poss_item, slay,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
use crate::headers::player_calcs_h::adj_str_blow;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23, STAT_MAX,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{strchr, strcmp, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub name: *const ::core::ffi::c_char,
    pub val: BORG_MONBLOW,
}
#[no_mangle]
pub static mut successful_target: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut target_closest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_tp_other_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_tp_other_x: [uint8_t; 255] = [0; 255];
#[no_mangle]
pub static mut borg_tp_other_y: [uint8_t; 255] = [0; 255];
#[no_mangle]
pub static mut borg_tp_other_index: [::core::ffi::c_int; 255] = [0; 255];
#[no_mangle]
pub unsafe extern "C" fn borg_mon_blow_effect(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut monblow: [C2RustUnnamed_4; 31] = [
        C2RustUnnamed_4 {
            name: b"NONE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_NONE,
        },
        C2RustUnnamed_4 {
            name: b"HURT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_HURT,
        },
        C2RustUnnamed_4 {
            name: b"POISON\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_POISON,
        },
        C2RustUnnamed_4 {
            name: b"DISENCHANT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_DISENCHANT,
        },
        C2RustUnnamed_4 {
            name: b"DRAIN_CHARGES\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_DRAIN_CHARGES,
        },
        C2RustUnnamed_4 {
            name: b"EAT_GOLD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_GOLD,
        },
        C2RustUnnamed_4 {
            name: b"EAT_ITEM\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_ITEM,
        },
        C2RustUnnamed_4 {
            name: b"EAT_FOOD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_FOOD,
        },
        C2RustUnnamed_4 {
            name: b"EAT_LIGHT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_LIGHT,
        },
        C2RustUnnamed_4 {
            name: b"ACID\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_ACID,
        },
        C2RustUnnamed_4 {
            name: b"ELEC\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_ELEC,
        },
        C2RustUnnamed_4 {
            name: b"FIRE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_FIRE,
        },
        C2RustUnnamed_4 {
            name: b"COLD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_COLD,
        },
        C2RustUnnamed_4 {
            name: b"BLIND\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_BLIND,
        },
        C2RustUnnamed_4 {
            name: b"CONFUSE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_CONFUSE,
        },
        C2RustUnnamed_4 {
            name: b"TERRIFY\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_TERRIFY,
        },
        C2RustUnnamed_4 {
            name: b"PARALYZE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_PARALYZE,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_STR\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_STR,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_INT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_INT,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_WIS\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_WIS,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_DEX\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_DEX,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_CON\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_CON,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_ALL\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_ALL,
        },
        C2RustUnnamed_4 {
            name: b"SHATTER\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_SHATTER,
        },
        C2RustUnnamed_4 {
            name: b"EXP_10\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_10,
        },
        C2RustUnnamed_4 {
            name: b"EXP_20\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_20,
        },
        C2RustUnnamed_4 {
            name: b"EXP_40\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_40,
        },
        C2RustUnnamed_4 {
            name: b"EXP_80\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_80,
        },
        C2RustUnnamed_4 {
            name: b"HALLU\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_HALLU,
        },
        C2RustUnnamed_4 {
            name: b"BLACK_BREATH\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_BLACK_BREATH,
        },
        C2RustUnnamed_4 {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            val: MONBLOW_NONE,
        },
    ];
    let mut i: ::core::ffi::c_ulong = 0;
    i = 0 as ::core::ffi::c_ulong;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_4; 31]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as usize)
    {
        if strcmp(name, monblow[i as usize].name) == 0 {
            return monblow[i as usize].val as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return MONBLOW_NONE as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_thrust_damage_one(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut dam: ::core::ffi::c_int = 0;
    let mut mult: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut chance: ::core::ffi::c_int = 0;
    item = &raw mut crate::bridge::st().items.items[((*z_info).pack_size) as usize] as *mut borg_item;
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    dam = (*item).dd as ::core::ffi::c_int
        * ((*item).ds as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int;
    mult = 1 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize]
        != 0
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
    {
        mult = 2 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize]
        != 0
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_DEMON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_ORC as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_TROLL as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_GIANT as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_DRAGON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize]
            != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_ACID as ::core::ffi::c_int,
            )
        || crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize]
            != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_FIRE as ::core::ffi::c_int,
            )
        || crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize]
            != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_COLD as ::core::ffi::c_int,
            )
        || crate::bridge::st().borg.trait_[(BI_WB_POIS as ::core::ffi::c_int) as usize]
            != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_POIS as ::core::ffi::c_int,
            )
        || crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
            != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_ELEC as ::core::ffi::c_int,
            )
    {
        mult = 3 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_WK_UNDEAD as ::core::ffi::c_int) as usize]
        != 0
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || crate::bridge::st().borg.trait_[(BI_WK_DEMON as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_DEMON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
            != 0
            && flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_DRAGON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
    {
        mult = 5 as ::core::ffi::c_int;
    }
    dam *= mult;
    dam += (*item).to_d as ::core::ffi::c_int;
    dam += crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize];
    dam *= crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize];
    chance = crate::bridge::st().borg.trait_[(BI_THN as ::core::ffi::c_int) as usize]
        + (crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
            + (*item).to_h as ::core::ffi::c_int)
            * 3 as ::core::ffi::c_int;
    if chance
        < (*r_ptr).ac * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
    {
        dam = 0 as ::core::ffi::c_int;
    }
    if chance > 95 as ::core::ffi::c_int {
        chance = 95 as ::core::ffi::c_int;
    }
    if chance < 5 as ::core::ffi::c_int {
        chance = 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 15 as ::core::ffi::c_int
    {
        chance += 10 as ::core::ffi::c_int;
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
        && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] > 1 as ::core::ffi::c_int
    {
        chance -= 10 as ::core::ffi::c_int;
    }
    dam = dam * chance / 100 as ::core::ffi::c_int;
    if dam <= 0 as ::core::ffi::c_int {
        dam = 1 as ::core::ffi::c_int;
    }
    if dam > (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        && !flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNIQUE as ::core::ffi::c_int,
        )
    {
        dam = (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 40 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] > 1 as ::core::ffi::c_int
    {
        dam = dam * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
    {
        dam += dam * 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
    {
        dam = dam * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 5 as ::core::ffi::c_int
        {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        dam = dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).spell_flags as *mut bitflag,
        (RSF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RSF_S_KIN as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTERS as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_SPIDER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HOUND as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HYDRA as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_AINU as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_WRAITH as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNIQUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        dam += dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_QUESTOR as ::core::ffi::c_int,
    ) {
        dam += dam * 5 as ::core::ffi::c_int;
    }
    return dam;
}
unsafe extern "C" fn borg_attack_aux_thrust() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut current_block_12: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(borg_distance(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) > 1 as ::core::ffi::c_int) {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
            if !(d <= 0 as ::core::ffi::c_int) {
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !((*kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    if !(*kill).awake
                        && d <= (*kill).power as ::core::ffi::c_int
                        && !crate::bridge::st().borg.munchkin_mode
                    {
                        p = borg_danger_one_kill(
                            y,
                            x,
                            1 as ::core::ffi::c_int,
                            (*ag).kill as ::core::ffi::c_int,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                            current_block_12 = 17778012151635330486;
                        } else {
                            current_block_12 = 10599921512955367680;
                        }
                    } else {
                        current_block_12 = 10599921512955367680;
                    }
                    match current_block_12 {
                        17778012151635330486 => {}
                        _ => {
                            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                == 0
                                && !(*kill).awake)
                            {
                                p = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    2 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if d <= (*kill).power as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                        > 15 as ::core::ffi::c_int
                                {
                                    p = p / 10 as ::core::ffi::c_int;
                                }
                                d += p;
                                if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
                                    b_i = i;
                                    b_d = d;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    crate::bridge::st().borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    crate::bridge::st().borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = crate::bridge::grid(crate::bridge::st().borg.goal.g.x, crate::bridge::st().borg.goal.g.y) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    borg_note(format(
        b"# Facing %s at (%d,%d) who has %d Hit Points.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        crate::bridge::st().borg.goal.g.y,
        crate::bridge::st().borg.goal.g.x,
        (*kill).power as ::core::ffi::c_int,
    ));
    borg_note(format(
        b"# Attacking with weapon '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[((*z_info).pack_size) as usize]).desc
            as *mut ::core::ffi::c_char,
    ));
    dir = borg_extract_dir(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, crate::bridge::st().borg.goal.g.y, crate::bridge::st().borg.goal.g.x);
    borg_keypress('+' as i32 as keycode_t);
    borg_keypress((dir + '0' as i32) as keycode_t);
    return b_d;
}
unsafe extern "C" fn borg_best_mult(
    mut obj: *mut borg_item,
    mut r_ptr: *mut monster_race,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut max_mult: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut current_block_6: u64;
    i = 1 as ::core::ffi::c_int;
    while i < (*z_info).brand_max as ::core::ffi::c_int {
        let mut brand: *mut brand = brands.offset(i as isize) as *mut brand;
        if !obj.is_null() && (*obj).iqty as ::core::ffi::c_int != 0 {
            if !(*obj).brands[i as usize] {
                current_block_6 = 16668937799742929182;
            } else {
                current_block_6 = 10879442775620481940;
            }
        } else if !player_has_temporary_brand(player, i) {
            current_block_6 = 16668937799742929182;
        } else {
            current_block_6 = 10879442775620481940;
        }
        match current_block_6 {
            10879442775620481940 => {
                if !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    (*brand).resist_flag,
                ) {
                    let mut mult: ::core::ffi::c_int = (*brand).multiplier;
                    if (*brand).vuln_flag != 0
                        && flag_has(
                            &raw mut (*r_ptr).flags as *mut bitflag,
                            (RF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            (*brand).vuln_flag,
                        ) as ::core::ffi::c_int
                            != 0
                    {
                        mult *= 2 as ::core::ffi::c_int;
                    }
                    max_mult = if mult < max_mult { max_mult } else { mult };
                }
            }
            _ => {}
        }
        i += 1;
    }
    let mut current_block_11: u64;
    i = 1 as ::core::ffi::c_int;
    while i < (*z_info).slay_max as ::core::ffi::c_int {
        let mut slay: *mut slay = slays.offset(i as isize) as *mut slay;
        if !obj.is_null() && (*obj).iqty as ::core::ffi::c_int != 0 {
            if (*obj).slays[i as usize] == 0 {
                current_block_11 = 3512920355445576850;
            } else {
                current_block_11 = 12124785117276362961;
            }
        } else if !player_has_temporary_slay(player, i) {
            current_block_11 = 3512920355445576850;
        } else {
            current_block_11 = 12124785117276362961;
        }
        match current_block_11 {
            12124785117276362961 => {
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    (*slay).race_flag,
                ) {
                    max_mult = if (*slay).multiplier < max_mult {
                        max_mult
                    } else {
                        (*slay).multiplier
                    };
                }
            }
            _ => {}
        }
        i += 1;
    }
    return max_mult;
}
unsafe extern "C" fn borg_launch_damage_one(
    mut i: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut borg_use_missile: bool = false_0 != 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut vault_grids: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut gold_eater: bool = false_0 != 0;
    let mut chance: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bonus: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur_dis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut armor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    cur_dis = distance(crate::bridge::st().borg.c, (*kill).pos);
    bonus = crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize]
        + (crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as usize])
        .to_h as ::core::ffi::c_int
        + (crate::bridge::st().items.items[(ammo_location) as usize]).to_h as ::core::ffi::c_int;
    chance = crate::bridge::st().borg.trait_[(BI_THB as ::core::ffi::c_int) as usize] + bonus * BTH_PLUS_ADJ;
    armor = (*r_ptr).ac + cur_dis;
    k = 0 as ::core::ffi::c_int;
    while k < 4 as ::core::ffi::c_int {
        if !(*(*r_ptr).blow.offset(k as isize)).effect.is_null()
            && borg_mon_blow_effect((*(*(*r_ptr).blow.offset(k as isize)).effect).name)
                == MONBLOW_EAT_GOLD as ::core::ffi::c_int
        {
            gold_eater = true_0 != 0;
        }
        k += 1;
    }
    match typ {
        1 => {
            let mut bow: *mut borg_item = &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as usize] as *mut borg_item;
            let mut ammo: *mut borg_item =
                &raw mut crate::bridge::st().items.items[(ammo_location) as usize] as *mut borg_item;
            let mut mult: ::core::ffi::c_int = borg_best_mult(bow, r_ptr);
            mult = if mult < borg_best_mult(ammo, r_ptr) {
                borg_best_mult(ammo, r_ptr)
            } else {
                mult
            };
            dam *= mult;
            if cur_dis == 1 as ::core::ffi::c_int
                && !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                )
            {
                dam /= 5 as ::core::ffi::c_int;
            }
            if chance < armor * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        2 => {
            if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                && crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_restore_mana) as usize] > 3 as ::core::ffi::c_int
            {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        4 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_ACID as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        5 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_ELEC as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        6 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_FIRE as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_HURT_FIRE as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        7 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_COLD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_HURT_COLD as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        8 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_POIS as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        9 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_COLD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        10 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        11 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        13 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_SPIRIT as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        12 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        56 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NONLIVING as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        15 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        16 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_HURT_LIGHT as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        17 => {
            if distance(crate::bridge::st().borg.c, (*kill).pos) == 1 as ::core::ffi::c_int {
                dam /= 5 as ::core::ffi::c_int;
            }
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
                || flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_DEMON as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                || !strchr(
                    b"Egv\0" as *const u8 as *const ::core::ffi::c_char,
                    (*r_ptr).d_char as ::core::ffi::c_int,
                )
                .is_null()
            {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        18 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_HURT_ROCK as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        19 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_NETH as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            } else if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam /= 2 as ::core::ffi::c_int;
            }
        }
        20 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_CHAO as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                dam = -(999 as ::core::ffi::c_int);
            }
        }
        21 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_GRAV as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        22 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_SHAR as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        23 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_SOUN as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        24 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_PLAS as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        25 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_CONF as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        26 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_DISE as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        27 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_NEXU as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        28 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_WALL as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        29 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_INER as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        30 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_TIME as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        31 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_LIGHT as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        32 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_DARK as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        33 => {
            if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BA_WATE as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
            dam /= 2 as ::core::ffi::c_int;
        }
        34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 => {
            dam = 0 as ::core::ffi::c_int;
        }
        45 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                if (*kill).injury as ::core::ffi::c_int >= 60 as ::core::ffi::c_int {
                    dam = -(9999 as ::core::ffi::c_int);
                } else if crate::bridge::st().borg.status.anti_summon {
                    dam = -(9999 as ::core::ffi::c_int);
                } else if dam
                    > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 13 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        <= 98 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                    && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                    == CLASS_MAGE
                    && distance(crate::bridge::st().borg.c, (*kill).pos) <= 2 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if crate::bridge::st().borg.status.vault {
                    ii = 0 as ::core::ffi::c_int;
                    while ii < 8 as ::core::ffi::c_int {
                        x = (*kill).pos.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                        y = (*kill).pos.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                            if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int {
                                vault_grids += 1;
                            }
                        }
                        ii += 1;
                    }
                    if vault_grids >= 2 as ::core::ffi::c_int {
                        borg_tp_other_index[borg_tp_other_n as usize] = i;
                        borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                        borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                        borg_tp_other_n += 1;
                    }
                } else {
                    dam = -(999 as ::core::ffi::c_int);
                }
            } else {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
            }
        }
        46 => {
            dam = 0 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < 8 as ::core::ffi::c_int {
                let mut y2: ::core::ffi::c_int =
                    (*kill).pos.y + ddy_ddd[j as usize] as ::core::ffi::c_int;
                let mut x2: ::core::ffi::c_int =
                    (*kill).pos.x + ddx_ddd[j as usize] as ::core::ffi::c_int;
                ag = crate::bridge::grid(x2, y2) as *mut borg_grid;
                if (*ag).glyph {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                    dam = 300 as ::core::ffi::c_int;
                }
                j += 1;
            }
            if crate::bridge::st().borg.near.morgoth as ::core::ffi::c_int != 0 && !crate::bridge::st().borg.morgoth_position {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
                dam = 100 as ::core::ffi::c_int;
            }
            if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                <= 35 as ::core::ffi::c_int
            {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
                dam = 150 as ::core::ffi::c_int;
            }
        }
        47 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else {
                dam = borg_danger_one_kill(
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    1 as ::core::ffi::c_int,
                    i,
                    true_0 != 0,
                    true_0 != 0,
                );
            }
        }
        48 => {
            dam = 0 as ::core::ffi::c_int;
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_CONF as ::core::ffi::c_int,
            ) {
                if !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_MULTIPLY as ::core::ffi::c_int,
                ) {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if !(*kill).confused {
                            if (*kill).awake {
                                if !((*kill).level as ::core::ffi::c_int
                                    > (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        < 13 as ::core::ffi::c_int
                                    {
                                        10 as ::core::ffi::c_int
                                    } else {
                                        (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                            - 10 as ::core::ffi::c_int)
                                            / 4 as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int
                                    }))
                                {
                                    dam = -(999 as ::core::ffi::c_int);
                                    if !flag_has(
                                        &raw mut (*r_ptr).flags as *mut bitflag,
                                        (RF_MAX as ::core::ffi::c_int as size_t)
                                            .wrapping_add(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            )
                                            .wrapping_sub(1 as size_t)
                                            .wrapping_div(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            ),
                                        RF_UNIQUE as ::core::ffi::c_int,
                                    ) {
                                        borg_confuse_spell = false_0 != 0;
                                        p1 = borg_danger_one_kill(
                                            crate::bridge::st().borg.c.y,
                                            crate::bridge::st().borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        if (*kill).afraid as ::core::ffi::c_int != 0
                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                <= 10 as ::core::ffi::c_int
                                        {
                                            p1 = p1 + 20 as ::core::ffi::c_int;
                                        }
                                        borg_confuse_spell = true_0 != 0;
                                        p2 = borg_danger_one_kill(
                                            crate::bridge::st().borg.c.y,
                                            crate::bridge::st().borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        borg_confuse_spell = false_0 != 0;
                                        dam = p1 - p2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        49 => {
            dam = 0 as ::core::ffi::c_int;
            if !(((*kill).speed as ::core::ffi::c_int) < (*r_ptr).speed - 5 as ::core::ffi::c_int) {
                if !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_NO_FEAR as ::core::ffi::c_int,
                ) {
                    if !(*kill).confused {
                        if (*kill).awake {
                            if !((*kill).level as ::core::ffi::c_int
                                > (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                    < 13 as ::core::ffi::c_int
                                {
                                    10 as ::core::ffi::c_int
                                } else {
                                    (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        - 10 as ::core::ffi::c_int)
                                        / 4 as ::core::ffi::c_int
                                        * 3 as ::core::ffi::c_int
                                        + 10 as ::core::ffi::c_int
                                }))
                            {
                                dam = -(999 as ::core::ffi::c_int);
                                if !flag_has(
                                    &raw mut (*r_ptr).flags as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_UNIQUE as ::core::ffi::c_int,
                                ) {
                                    borg_fear_mon_spell = false_0 != 0;
                                    p1 = borg_danger_one_kill(
                                        crate::bridge::st().borg.c.y,
                                        crate::bridge::st().borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        i,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    if (*kill).afraid as ::core::ffi::c_int != 0
                                        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                            <= 10 as ::core::ffi::c_int
                                    {
                                        p1 = p1 + 20 as ::core::ffi::c_int;
                                    }
                                    borg_fear_mon_spell = true_0 != 0;
                                    p2 = borg_danger_one_kill(
                                        crate::bridge::st().borg.c.y,
                                        crate::bridge::st().borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        i,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    borg_fear_mon_spell = false_0 != 0;
                                    dam = p1 - p2;
                                }
                            }
                        }
                    }
                }
            }
        }
        50 => {
            dam = 0 as ::core::ffi::c_int;
            if !(((*kill).speed as ::core::ffi::c_int) < (*r_ptr).speed - 5 as ::core::ffi::c_int) {
                if !(*kill).confused {
                    if (*kill).awake {
                        if !((*kill).level as ::core::ffi::c_int
                            > (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                < 13 as ::core::ffi::c_int
                            {
                                10 as ::core::ffi::c_int
                            } else {
                                (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                    - 10 as ::core::ffi::c_int)
                                    / 4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    + 10 as ::core::ffi::c_int
                            }))
                        {
                            dam = -(999 as ::core::ffi::c_int);
                            if !flag_has(
                                &raw mut (*r_ptr).flags as *mut bitflag,
                                (RF_MAX as ::core::ffi::c_int as size_t)
                                    .wrapping_add(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    )
                                    .wrapping_sub(1 as size_t)
                                    .wrapping_div(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    ),
                                RF_UNIQUE as ::core::ffi::c_int,
                            ) {
                                borg_slow_spell = false_0 != 0;
                                p1 = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if (*kill).afraid as ::core::ffi::c_int != 0
                                    && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        <= 10 as ::core::ffi::c_int
                                {
                                    p1 = p1 + 20 as ::core::ffi::c_int;
                                }
                                borg_slow_spell = true_0 != 0;
                                p2 = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_slow_spell = false_0 != 0;
                                dam = p1 - p2;
                            }
                        }
                    }
                }
            }
        }
        51 | 14 => {
            dam = 0 as ::core::ffi::c_int;
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_SLEEP as ::core::ffi::c_int,
            ) {
                if !(!flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_EVIL as ::core::ffi::c_int,
                ) && typ == BORG_ATTACK_SLEEP_EVIL as ::core::ffi::c_int)
                {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if !(*kill).confused {
                            if (*kill).awake {
                                if !((*kill).level as ::core::ffi::c_int
                                    > (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        < 13 as ::core::ffi::c_int
                                    {
                                        10 as ::core::ffi::c_int
                                    } else {
                                        (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                            - 10 as ::core::ffi::c_int)
                                            / 4 as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int
                                    }))
                                {
                                    dam = -(999 as ::core::ffi::c_int);
                                    if !flag_has(
                                        &raw mut (*r_ptr).flags as *mut bitflag,
                                        (RF_MAX as ::core::ffi::c_int as size_t)
                                            .wrapping_add(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            )
                                            .wrapping_sub(1 as size_t)
                                            .wrapping_div(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            ),
                                        RF_UNIQUE as ::core::ffi::c_int,
                                    ) {
                                        borg_sleep_spell = false_0 != 0;
                                        p1 = borg_danger_one_kill(
                                            crate::bridge::st().borg.c.y,
                                            crate::bridge::st().borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        if (*kill).afraid as ::core::ffi::c_int != 0
                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                <= 10 as ::core::ffi::c_int
                                        {
                                            p1 = p1 + 20 as ::core::ffi::c_int;
                                        }
                                        borg_sleep_spell = true_0 != 0;
                                        p2 = borg_danger_one_kill(
                                            crate::bridge::st().borg.c.y,
                                            crate::bridge::st().borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        borg_sleep_spell = false_0 != 0;
                                        dam = p1 - p2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        52 => {
            dam = 0 as ::core::ffi::c_int;
            if !((*kill).level as ::core::ffi::c_int
                > (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 13 as ::core::ffi::c_int
                {
                    10 as ::core::ffi::c_int
                } else {
                    (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        - 10 as ::core::ffi::c_int)
                        / 4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int
                }))
            {
                dam = -(999 as ::core::ffi::c_int);
                if !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    dam = borg_danger_one_kill(
                        crate::bridge::st().borg.c.y,
                        crate::bridge::st().borg.c.x,
                        2 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if dam < crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        && !(*kill).afraid
                    {
                        dam = 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        53 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
                if !(*kill).confused {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if (*kill).awake {
                            if !((*kill).level as ::core::ffi::c_int
                                > crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                    - 5 as ::core::ffi::c_int)
                            {
                                borg_fear_mon_spell = false_0 != 0;
                                p1 = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_fear_mon_spell = true_0 != 0;
                                p2 = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_fear_mon_spell = false_0 != 0;
                                dam = p1 - p2;
                            }
                        }
                    }
                }
            } else {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        54 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    if !(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null() {
                        dam = 0 as ::core::ffi::c_int;
                    } else {
                        dam = -(500 as ::core::ffi::c_int);
                    }
                } else {
                    dam = borg_danger_one_kill(
                        crate::bridge::st().borg.c.y,
                        crate::bridge::st().borg.c.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                }
            } else {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        55 => {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else {
                let mut sp_drain: ::core::ffi::c_int =
                    crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                        - crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize];
                if sp_drain < (*kill).power as ::core::ffi::c_int {
                    dam = (*kill).power as ::core::ffi::c_int - sp_drain;
                }
            }
        }
        58 => {
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 12 as ::core::ffi::c_int
                * (50 as ::core::ffi::c_int
                    + (*kill).injury as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
        }
        57 => {
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_IM_ELEC as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else if !borg_projectable_pure(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        0 | 3 | _ => {}
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        >= 1 as ::core::ffi::c_int
        && (borg_danger_one_kill(
            (*kill).pos.y,
            (*kill).pos.x,
            1 as ::core::ffi::c_int,
            i,
            true_0 != 0,
            true_0 != 0,
        ) > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
            || (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
                && (*kill).level as ::core::ffi::c_int
                    >= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        - 5 as ::core::ffi::c_int
            || (*kill).ranged_attack as ::core::ffi::c_int != 0
            || flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_MULTIPLY as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || gold_eater as ::core::ffi::c_int != 0
            || flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NEVER_MOVE as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 20 as ::core::ffi::c_int)
    {
        borg_use_missile = true_0 != 0;
    }
    if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_EVIL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int
    {
        return dam;
    }
    if dam > (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        && !flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNIQUE as ::core::ffi::c_int,
        )
    {
        dam = (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
    {
        dam = dam * 3 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
    {
        dam = dam * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 5 as ::core::ffi::c_int
        {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        dam = dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).spell_flags as *mut bitflag,
        (RSF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RSF_S_KIN as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTERS as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_SPIDER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HOUND as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HYDRA as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_AINU as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_WRAITH as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNIQUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        dam += dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_QUESTOR as ::core::ffi::c_int,
    ) {
        dam += dam * 9 as ::core::ffi::c_int;
    }
    if typ == BORG_ATTACK_ARROW as ::core::ffi::c_int {
        if !borg_use_missile {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    return dam;
}
unsafe extern "C" fn borg_launch_bolt_aux_hack(
    mut i: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut p1: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut walls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unknown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.time.now - (*kill).when > 2 as borg_time {
        return 0 as ::core::ffi::c_int;
    }
    x = (*kill).pos.x;
    y = (*kill).pos.y;
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
        return 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && ((*ag).feat as ::core::ffi::c_int != FEAT_FLOOR as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int != FEAT_OPEN as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int != FEAT_BROKEN as ::core::ffi::c_int
            && !(*ag).trap)
    {
        return 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        x = (*kill).pos.x;
        y = (*kill).pos.y;
        o_x = -(1 as ::core::ffi::c_int);
        while o_x <= 1 as ::core::ffi::c_int {
            o_y = -(1 as ::core::ffi::c_int);
            while o_y <= 1 as ::core::ffi::c_int {
                x = (*kill).pos.x + o_x;
                y = (*kill).pos.y + o_y;
                ag = crate::bridge::grid(x, y) as *mut borg_grid;
                if (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                    && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
                {
                    walls += 1;
                }
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    unknown += 1;
                }
                o_y += 1;
            }
            o_x += 1;
        }
        if walls >= 2 as ::core::ffi::c_int && unknown >= 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    d = borg_launch_damage_one(i, dam, typ, ammo_location);
    if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int
    {
        return d;
    }
    if typ == BORG_ATTACK_AWAY_EVIL as ::core::ffi::c_int {
        return d;
    }
    if d <= 0 as ::core::ffi::c_int {
        return d;
    }
    p2 = borg_danger_one_kill(y, x, 1 as ::core::ffi::c_int, i, true_0 != 0, false_0 != 0);
    if !(*kill).awake
        && p2 > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        && d < (*kill).power as ::core::ffi::c_int
        && !crate::bridge::st().borg.munchkin_mode
    {
        return -(999 as ::core::ffi::c_int);
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
        && !(*kill).awake
    {
        return 0 as ::core::ffi::c_int;
    }
    crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 3 as ::core::ffi::c_int;
    p1 = borg_danger_one_kill(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        i,
        true_0 != 0,
        false_0 != 0,
    );
    if d >= (*kill).power as ::core::ffi::c_int {
        d = 2 as ::core::ffi::c_int * d;
    }
    d = d + p1;
    return d;
}
unsafe extern "C" fn borg_launch_destroy_stuff(
    mut ag: *mut borg_grid,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut take: *mut borg_take = borg_takes.offset((*ag).take as isize) as *mut borg_take;
    let mut k_ptr: *mut object_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
    match typ {
        4 => {
            if (*k_ptr).tval == TV_BOOTS as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
        }
        5 => {
            if (*k_ptr).tval == TV_RING as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
            if (*k_ptr).tval == TV_RING as ::core::ffi::c_int && (*k_ptr).sval == crate::bridge::st().item_val.sv_ring_speed {
                return 2000 as ::core::ffi::c_int;
            }
        }
        6 => {
            if (*k_ptr).tval == TV_BOOTS as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
        }
        7 => {
            if (*k_ptr).tval == TV_POTION as ::core::ffi::c_int {
                if !(*k_ptr).aware
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_healing
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_star_healing
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_life
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_inc_str
                        && crate::bridge::st().borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_inc_int
                        && crate::bridge::st().borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_inc_wis
                        && crate::bridge::st().borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_inc_dex
                        && crate::bridge::st().borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == crate::bridge::st().item_val.sv_potion_inc_con
                        && crate::bridge::st().borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                {
                    return 2000 as ::core::ffi::c_int;
                }
                return 20 as ::core::ffi::c_int;
            }
        }
        2 | _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_launch_bolt_at_location(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ry: ::core::ffi::c_int = 0;
    let mut rx: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    q_x = crate::bridge::st().core.w_x / crate::borg_cave_util::borg_panel_wid();
    q_y = crate::bridge::st().core.w_y / crate::borg_cave_util::borg_panel_hgt();
    n = 0 as ::core::ffi::c_int;
    x1 = crate::bridge::st().borg.c.x;
    y1 = crate::bridge::st().borg.c.y;
    x2 = x;
    y2 = y;
    if !square_in_bounds_fully(cave, loc(x, y)) {
        return 0 as ::core::ffi::c_int;
    }
    x = x1;
    y = y1;
    ag = crate::bridge::grid(x2, y2)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    dist = 1 as ::core::ffi::c_int;
    while dist < max {
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        if !square_in_bounds_fully(cave, loc(x, y)) {
            break;
        }
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag))
            || (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int
        {
            if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            } else {
                return n;
            }
        }
        if rad <= 0 as ::core::ffi::c_int || rad == 10 as ::core::ffi::c_int {
            n += borg_launch_bolt_aux_hack(
                (*ag).kill as ::core::ffi::c_int,
                dam,
                typ,
                ammo_location,
            );
        }
        if rad != -(1 as ::core::ffi::c_int)
            && rad != 10 as ::core::ffi::c_int
            && (x == x2 && y == y2)
        {
            break;
        }
        if rad == 0 && (*ag).kill as ::core::ffi::c_int != 0 {
            return n;
        }
        if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] == 0 {
            if crate::bridge::st().borg.trait_[(BI_INFRA as ::core::ffi::c_int) as usize]
                <= 0 as ::core::ffi::c_int
                && !((*r_ptr).light > 0 as ::core::ffi::c_int)
            {
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                } else {
                    return n;
                }
            }
        } else {
            if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && (borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
                    [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
            {
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    if successful_target <= -(12 as ::core::ffi::c_int) {
                        successful_target = 0 as ::core::ffi::c_int;
                    }
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                } else {
                    return n;
                }
            }
        }
        dist += 1;
    }
    if rad <= 0 as ::core::ffi::c_int {
        return n;
    }
    if dist >= max {
        return 0 as ::core::ffi::c_int;
    }
    ry = y2 - rad;
    while ry < y2 + rad {
        rx = x2 - rad;
        while rx < x2 + rad {
            if square_in_bounds(cave, loc(rx, ry)) {
                ag = crate::bridge::grid(rx, ry) as *mut borg_grid;
                r = borg_distance(y2, x2, ry, rx);
                if !(r > rad) {
                    if borg_los(y2, x2, ry, rx) {
                        if rad == 10 as ::core::ffi::c_int {
                            r = 0 as ::core::ffi::c_int;
                        }
                        n += borg_launch_bolt_aux_hack(
                            (*ag).kill as ::core::ffi::c_int,
                            dam / (r + 1 as ::core::ffi::c_int),
                            typ,
                            ammo_location,
                        );
                        if (*ag).take as ::core::ffi::c_int != 0
                            && (*borg_takes.offset((*ag).take as isize)).kidx != 0
                        {
                            n -= borg_launch_destroy_stuff(ag, typ);
                        }
                    }
                }
            }
            rx += 1;
        }
        ry += 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_launch_bolt(
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut require_monster: bool = false_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        o_x = -(1 as ::core::ffi::c_int);
        while o_x <= 1 as ::core::ffi::c_int {
            o_y = -(1 as ::core::ffi::c_int);
            while o_y <= 1 as ::core::ffi::c_int {
                x = borg_temp_x[i as usize] as ::core::ffi::c_int + o_x;
                y = borg_temp_y[i as usize] as ::core::ffi::c_int + o_y;
                borg_tp_other_n = 0 as int16_t;
                n = 0 as ::core::ffi::c_int;
                if square_in_bounds(cave, loc(x, y)) {
                    d = distance(
                        crate::bridge::st().borg.c,
                        loc(
                            borg_temp_x[i as usize] as ::core::ffi::c_int,
                            borg_temp_y[i as usize] as ::core::ffi::c_int,
                        ),
                    );
                    if !((x != borg_temp_x[i as usize] as ::core::ffi::c_int
                        || y != borg_temp_y[i as usize] as ::core::ffi::c_int)
                        && typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int)
                    {
                        if !(distance(crate::bridge::st().borg.c, loc(x, y)) > max) {
                            if rad >= 2 as ::core::ffi::c_int
                                && (*crate::bridge::grid(x, y)).feat
                                    as ::core::ffi::c_int
                                    != FEAT_NONE as ::core::ffi::c_int
                                || y == borg_temp_y[i as usize] as ::core::ffi::c_int
                                    && x == borg_temp_x[i as usize] as ::core::ffi::c_int
                            {
                                n = borg_launch_bolt_at_location(
                                    y,
                                    x,
                                    rad,
                                    dam,
                                    typ,
                                    max,
                                    ammo_location,
                                );
                            }
                            if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
                                && n > 0 as ::core::ffi::c_int
                            {
                                n = borg_danger(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    true_0 != 0,
                                    false_0 != 0,
                                );
                                n = dam - n;
                            }
                            borg_tp_other_n = 0 as int16_t;
                            if !(n <= 0 as ::core::ffi::c_int) {
                                if !(x == 0 as ::core::ffi::c_int
                                    || y == 0 as ::core::ffi::c_int
                                    || x == DUNGEON_WID - 1 as ::core::ffi::c_int
                                    || y == DUNGEON_HGT - 1 as ::core::ffi::c_int)
                                {
                                    if !(b_i >= 0 as ::core::ffi::c_int && n < b_n) {
                                        if !(n == b_n && d > b_d) {
                                            b_i = i;
                                            b_n = n;
                                            b_o_y = o_y;
                                            b_o_x = o_x;
                                            b_d = d;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                o_y += 1;
            }
            o_x += 1;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return b_n;
    }
    borg_tp_other_n = 0 as int16_t;
    if borg_simulate {
        return b_n;
    }
    crate::bridge::st().borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int + b_o_x;
    crate::bridge::st().borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int + b_o_y;
    if typ == BORG_ATTACK_CURSE as ::core::ffi::c_int {
        require_monster = true_0 != 0;
    }
    borg_target(crate::bridge::st().borg.goal.g, require_monster);
    return b_n;
}
unsafe extern "C" fn borg_launch_arc_at_location(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut degrees: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ry: ::core::ffi::c_int = 0;
    let mut rx: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut path_grids: [loc; 256] = [loc { x: 0, y: 0 }; 256];
    q_x = crate::bridge::st().core.w_x / crate::borg_cave_util::borg_panel_wid();
    q_y = crate::bridge::st().core.w_y / crate::borg_cave_util::borg_panel_hgt();
    n = 0 as ::core::ffi::c_int;
    x1 = crate::bridge::st().borg.c.x;
    y1 = crate::bridge::st().borg.c.y;
    x2 = x;
    y2 = y;
    if !square_in_bounds_fully(cave, loc(x, y)) {
        return 0 as ::core::ffi::c_int;
    }
    x = x1;
    y = y1;
    ag = crate::bridge::grid(x2, y2)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    path_grids[0 as ::core::ffi::c_int as usize].x = x;
    path_grids[0 as ::core::ffi::c_int as usize].y = y;
    dist = 1 as ::core::ffi::c_int;
    while dist < max {
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        if !square_in_bounds_fully(cave, loc(x, y)) {
            break;
        }
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag))
            || (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int
        {
            break;
        }
        path_grids[dist as usize].x = x;
        path_grids[dist as usize].y = y;
        if x == x2 && y == y2 {
            break;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
                if crate::bridge::st().borg.trait_[(BI_INFRA as ::core::ffi::c_int) as usize]
                    <= 0 as ::core::ffi::c_int
                    && !((*r_ptr).light > 0 as ::core::ffi::c_int)
                {
                    break;
                }
            } else if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && (borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
                    [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
            {
                break;
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                break;
            }
        }
        dist += 1;
    }
    if dist < 21 as ::core::ffi::c_int {
        dist = dist - 1 as ::core::ffi::c_int;
    } else {
        dist = 20 as ::core::ffi::c_int;
    }
    let mut n1x: ::core::ffi::c_int = 0;
    let mut n1y: ::core::ffi::c_int = 0;
    let mut n2y: ::core::ffi::c_int = 0;
    let mut n2x: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut rotate: ::core::ffi::c_int = 0;
    let mut diff: ::core::ffi::c_int = 0;
    n1y = path_grids[dist as usize].y - y1 + 20 as ::core::ffi::c_int;
    n1x = path_grids[dist as usize].x - x1 + 20 as ::core::ffi::c_int;
    ry = y1 - max;
    while ry < y1 + max {
        rx = x1 - max;
        while rx < x1 + max {
            if square_in_bounds(cave, loc(rx, ry)) {
                ag = crate::bridge::grid(rx, ry) as *mut borg_grid;
                r = borg_distance(y1, x1, ry, rx);
                if !(r > max) {
                    if borg_los(y1, x1, ry, rx) {
                        n2y = y - y1 + 20 as ::core::ffi::c_int;
                        n2x = x - x1 + 20 as ::core::ffi::c_int;
                        rotate = 90 as ::core::ffi::c_int
                            - get_angle_to_grid[n1y as usize][n1x as usize] as ::core::ffi::c_int;
                        tmp = (if get_angle_to_grid[n2y as usize][n2x as usize]
                            as ::core::ffi::c_int
                            + rotate
                            < 0 as ::core::ffi::c_int
                        {
                            -(get_angle_to_grid[n2y as usize][n2x as usize] as ::core::ffi::c_int
                                + rotate)
                        } else {
                            get_angle_to_grid[n2y as usize][n2x as usize] as ::core::ffi::c_int
                                + rotate
                        }) % 180 as ::core::ffi::c_int;
                        diff = if 90 as ::core::ffi::c_int - tmp < 0 as ::core::ffi::c_int {
                            -(90 as ::core::ffi::c_int - tmp)
                        } else {
                            90 as ::core::ffi::c_int - tmp
                        };
                        if !(diff >= (degrees + 6 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int)
                        {
                            if (*ag).kill != 0 {
                                n += borg_launch_bolt_aux_hack(
                                    (*ag).kill as ::core::ffi::c_int,
                                    dam / (r + 1 as ::core::ffi::c_int),
                                    typ,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            if (*ag).take as ::core::ffi::c_int != 0
                                && (*borg_takes.offset((*ag).take as isize)).kidx != 0
                            {
                                n -= borg_launch_destroy_stuff(ag, typ);
                            }
                        }
                    }
                }
            }
            rx += 1;
        }
        ry += 1;
    }
    return n;
}
unsafe extern "C" fn borg_launch_arc(
    mut degrees: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    if max > 20 as ::core::ffi::c_int {
        max = 20 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        n = 0 as ::core::ffi::c_int;
        d = distance(
            crate::bridge::st().borg.c,
            loc(
                borg_temp_x[i as usize] as ::core::ffi::c_int,
                borg_temp_y[i as usize] as ::core::ffi::c_int,
            ),
        );
        if !(d > max) {
            n = borg_launch_arc_at_location(y, x, degrees, dam, typ, max);
            if !(n <= 0 as ::core::ffi::c_int) {
                if !(b_i >= 0 as ::core::ffi::c_int && n < b_n) {
                    if !(n == b_n && d > b_d) {
                        b_i = i;
                        b_n = n;
                        b_o_y = o_y;
                        b_o_x = o_x;
                        b_d = d;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return b_n;
    }
    if borg_simulate {
        return b_n;
    }
    crate::bridge::st().borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int + b_o_x;
    crate::bridge::st().borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int + b_o_y;
    borg_target(crate::bridge::st().borg.goal.g, false_0 != 0);
    return b_n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack_aux_launch() -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut v: ::core::ffi::c_int = 0;
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut bow: *mut borg_item = &raw mut crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as usize]
        as *mut borg_item;
    if (*bow).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    k = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while k
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(k) as usize] as *mut borg_item;
        if (*item).iqty == 0 {
            break;
        }
        if !((*item).tval as ::core::ffi::c_int
            != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
        {
            if !((*item).value <= 0 as int32_t) {
                d = (*item).dd as ::core::ffi::c_int
                    * ((*item).ds as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
                d = d + (*item).to_d as ::core::ffi::c_int + (*bow).to_d as ::core::ffi::c_int;
                d = d
                    * crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                    * crate::bridge::st().borg.trait_[(BI_SHOTS as ::core::ffi::c_int) as usize];
                v = (*item).value as ::core::ffi::c_int;
                if borg_item_note_needs_id(item) {
                    d = d * 99 as ::core::ffi::c_int;
                }
                if !(d <= 0 as ::core::ffi::c_int) {
                    n = borg_launch_bolt(
                        0 as ::core::ffi::c_int,
                        d,
                        BORG_ATTACK_ARROW as ::core::ffi::c_int,
                        6 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize],
                        k,
                    );
                    if !(n == b_n && v >= b_v) {
                        if n >= b_n {
                            b_n = n;
                            b_v = v;
                            b_k = k;
                        }
                    }
                }
            }
        }
        k += 1;
    }
    if b_n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Firing missile '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[(b_k) as usize]).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('f' as i32 as keycode_t);
    borg_keypress(
        (b_k - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int) + '0' as i32)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(2 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rest() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut my_danger: ::core::ffi::c_int = 0;
    if borg_simulate as ::core::ffi::c_int != 0 && crate::bridge::st().borg.goal.waiting as ::core::ffi::c_int != 0 {
        crate::bridge::st().borg.goal.waiting = false_0 != 0;
        return 0 as ::core::ffi::c_int;
    }
    my_danger = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                ax = if x9 > crate::bridge::st().borg.c.x {
                    x9 - crate::bridge::st().borg.c.x
                } else {
                    crate::bridge::st().borg.c.x - x9
                };
                ay = if y9 > crate::bridge::st().borg.c.y {
                    y9 - crate::bridge::st().borg.c.y
                } else {
                    crate::bridge::st().borg.c.y - y9
                };
                d = if ax < ay { ay } else { ax };
                if !(d != 2 as ::core::ffi::c_int) {
                    if !((*kill).ranged_attack != 0) {
                        if (*kill).awake {
                            if !(crate::bridge::st().borg.time.now - (*kill).when > 10 as borg_time) {
                                if !flag_has(
                                    &raw mut (*r_info.offset((*kill).r_idx as isize)).flags
                                        as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_NEVER_MOVE as ::core::ffi::c_int,
                                ) {
                                    if !((*kill).speed as ::core::ffi::c_int
                                        - crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                                        >= 5 as ::core::ffi::c_int)
                                    {
                                        if !((*kill).afraid as ::core::ffi::c_int != 0
                                            || (*kill).confused as ::core::ffi::c_int != 0
                                            || (*kill).stunned as ::core::ffi::c_int != 0)
                                        {
                                            if !(crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int
                                                != GOAL_KILL
                                                || borg_flow_y[0 as ::core::ffi::c_int as usize]
                                                    as ::core::ffi::c_int
                                                    != (*kill).pos.y)
                                            {
                                                if borg_los(
                                                    crate::bridge::st().borg.c.y,
                                                    crate::bridge::st().borg.c.x,
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                ) {
                                                    if !(my_danger
                                                        > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
                                                    {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if i >= borg_kills_nxt as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    borg_keypress(',' as i32 as keycode_t);
    borg_note(format(
        b"# Waiting for monster '%s' to approach.\0" as *const u8 as *const ::core::ffi::c_char,
        (*r_info.offset((*borg_kills.offset(i as isize)).r_idx as isize)).name,
    ));
    crate::bridge::st().borg.goal.waiting = true_0 != 0;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_object() -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut b_r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut div: ::core::ffi::c_int = 0;
    let mut mul: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = &raw mut crate::bridge::st().items.items[(k) as usize] as *mut borg_item;
        if !((*item).iqty == 0) {
            if !obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind) {
                if !(borg_wield_slot(item) >= 0 as ::core::ffi::c_int) {
                    d = (*k_info.offset((*item).kind as isize)).dd
                        * ((*k_info.offset((*item).kind as isize)).ds + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                    if !((*item).value > 100 as int32_t && d < 5 as ::core::ffi::c_int) {
                        if !(d <= 0 as ::core::ffi::c_int) {
                            if !((*item).tval as ::core::ffi::c_int
                                == TV_POTION as ::core::ffi::c_int)
                            {
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_FLASK as ::core::ffi::c_int
                                    && (crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize]
                                        <= 1 as ::core::ffi::c_int
                                        && crate::bridge::st().borg.near.unique == 0))
                                {
                                    if !((*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        || (*item).tval as ::core::ffi::c_int
                                            == TV_ROD as ::core::ffi::c_int)
                                    {
                                        if !(b_k >= 0 as ::core::ffi::c_int && d <= b_d) {
                                            b_k = k;
                                            b_d = d;
                                            mul = 10 as ::core::ffi::c_int;
                                            div = if (*item).weight as ::core::ffi::c_int
                                                > 10 as ::core::ffi::c_int
                                            {
                                                (*item).weight as ::core::ffi::c_int
                                            } else {
                                                10 as ::core::ffi::c_int
                                            };
                                            b_r = (adj_str_blow[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
                                                as usize]
                                                + 20 as ::core::ffi::c_int)
                                                * mul
                                                / div;
                                            if b_r > 10 as ::core::ffi::c_int {
                                                b_r = 10 as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        k += 1;
    }
    if b_k < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        b_d,
        BORG_ATTACK_ARROW as ::core::ffi::c_int,
        6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize],
        b_k,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Throwing painful object '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[(b_k) as usize]).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('v' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_k as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(2 as ::core::ffi::c_int);
    return b_n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack_aux_spell_bolt(
    spell: borg_spells,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max_range: ::core::ffi::c_int,
    mut is_arc: bool,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut penalty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_MAGE
            && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_NECROMANCER
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 2 as ::core::ffi::c_int)
        && (Rand_div(100 as uint32_t) as int32_t) < 1 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
            && (borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
                || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0))
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(
        spell,
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    if is_arc {
        b_n = borg_launch_arc(rad, dam, typ, max_range);
    } else {
        b_n = borg_launch_bolt(rad, dam, typ, max_range, 0 as ::core::ffi::c_int);
    }
    let mut primary_spell_for_class: borg_spells = MAGIC_MISSILE;
    match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
        CLASS_MAGE => {
            primary_spell_for_class = MAGIC_MISSILE;
        }
        CLASS_DRUID => {
            primary_spell_for_class = STINKING_CLOUD;
        }
        CLASS_PRIEST => {
            primary_spell_for_class = ORB_OF_DRAINING;
        }
        CLASS_NECROMANCER => {
            primary_spell_for_class = NETHER_BOLT;
        }
        CLASS_PALADIN | CLASS_ROGUE | CLASS_RANGER | CLASS_BLACKGUARD | _ => {}
    }
    if spell as ::core::ffi::c_uint == primary_spell_for_class as ::core::ffi::c_uint
        && (!borg_spell_legal_fail(TELEPORT_SELF, 15 as ::core::ffi::c_int)
            || crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                <= 30 as ::core::ffi::c_int)
    {
        if borg_simulate {
            return b_n;
        }
    }
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(spell);
    if spell as ::core::ffi::c_uint != primary_spell_for_class as ::core::ffi::c_uint {
        b_n = b_n - spell_power;
        if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] < 50 as ::core::ffi::c_int
            && spell_power > b_n
        {
            b_n = b_n - spell_power;
        }
        if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
            < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
        {
            b_n = b_n - spell_power * 3 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
            < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                / 3 as ::core::ffi::c_int
        {
            b_n = b_n - spell_power * 5 as ::core::ffi::c_int;
        }
    }
    match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
        CLASS_MAGE => {
            penalty = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            penalty = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            penalty = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            penalty = 10 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power < penalty
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_spell(spell);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_spell_bolt_reserve(
    spell: borg_spells,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max_range: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut x9: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut near_monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sv_mana: ::core::ffi::c_int =
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 15 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
            && borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    ) < crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if (*kill).r_idx == 0 {
            i += 1;
        } else {
            x9 = (*kill).pos.x;
            y9 = (*kill).pos.y;
            ax = if x9 > crate::bridge::st().borg.c.x {
                x9 - crate::bridge::st().borg.c.x
            } else {
                crate::bridge::st().borg.c.x - x9
            };
            ay = if y9 > crate::bridge::st().borg.c.y {
                y9 - crate::bridge::st().borg.c.y
            } else {
                crate::bridge::st().borg.c.y - y9
            };
            d = if ax < ay { ay } else { ax };
            if d < 7 as ::core::ffi::c_int {
                near_monsters += 1;
            }
            if (*kill).power as ::core::ffi::c_int > dam + 4 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            break;
        }
    }
    if near_monsters > 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
        crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize];
    if !borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = sv_mana;
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(rad, dam, typ, max_range, 0 as ::core::ffi::c_int);
    if borg_simulate {
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = sv_mana;
        return b_n;
    }
    crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
        crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize];
    if borg_spell_fail(spell, 25 as ::core::ffi::c_int) {
        borg_note(
            b"# Emergency use of an Attack Spell.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    borg_queue_direction('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_int;
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_spell_dispel(
    spell: borg_spells,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut penalty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
            && (borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
                || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0))
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        10 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(spell);
    b_n = b_n - spell_power;
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 3 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 3 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 5 as ::core::ffi::c_int;
    }
    match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
        CLASS_MAGE => {
            penalty = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            penalty = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            penalty = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            penalty = 10 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power < penalty
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
            < 6 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if target_closest < 0 as ::core::ffi::c_int
        && BORG_ATTACK_TAP_UNLIFE as ::core::ffi::c_int == typ
        && b_n > 0 as ::core::ffi::c_int
    {
        target_closest = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_spell(spell);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_staff_dispel(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_staff_fail(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    b_n = b_n - 50 as ::core::ffi::c_int;
    if borg_simulate {
        return b_n;
    }
    borg_use_staff(sval);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rod_bolt(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if (500 as ::core::ffi::c_int) < borg_activate_failure(TV_ROD as ::core::ffi::c_int, sval) {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_rod(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_zap_rod(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_wand_bolt(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = borg_slot(TV_WAND as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (crate::bridge::st().items.items[(i) as usize]).pval == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (500 as ::core::ffi::c_int) < borg_activate_failure(TV_WAND as ::core::ffi::c_int, sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 5 as ::core::ffi::c_int
    {
        b_n = b_n - 5 as ::core::ffi::c_int;
    }
    if sval == crate::bridge::st().item_val.sv_wand_wonder && !crate::bridge::st().borg.munchkin_mode {
        if b_n > 0 as ::core::ffi::c_int
            && borg_danger(
                crate::bridge::st().borg.c.y,
                crate::bridge::st().borg.c.x,
                1 as ::core::ffi::c_int,
                true_0 != 0,
                false_0 != 0,
            ) >= crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        {
            b_n = 999 as ::core::ffi::c_int;
            borg_note(format(
                b"# Emergency use of a Wand of Wonder.\0" as *const u8
                    as *const ::core::ffi::c_char,
            ));
        } else {
            b_n = 0 as ::core::ffi::c_int;
        }
    }
    if borg_simulate {
        return b_n;
    }
    borg_aim_wand(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_wand_bolt_unknown(
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 5 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        if !((crate::bridge::st().items.items[(i) as usize]).iqty == 0) {
            if !((crate::bridge::st().items.items[(i) as usize]).tval as ::core::ffi::c_int
                != TV_WAND as ::core::ffi::c_int)
            {
                if !(crate::bridge::st().items.items[(i) as usize]).aware {
                    if !((crate::bridge::st().items.items[(i) as usize]).pval == 0) {
                        b_i = i;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Aiming unknown wand '%s.'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[(b_i) as usize]).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('a' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    crate::bridge::st().borg.trying_unknown = true_0 != 0;
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rod_bolt_unknown(
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 5 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        if !((crate::bridge::st().items.items[(i) as usize]).iqty == 0) {
            if !((crate::bridge::st().items.items[(i) as usize]).tval as ::core::ffi::c_int
                != TV_ROD as ::core::ffi::c_int)
            {
                if !(crate::bridge::st().items.items[(i) as usize]).aware {
                    if !((crate::bridge::st().items.items[(i) as usize]).pval == 0) {
                        if strstr(
                            &raw mut (crate::bridge::st().items.items[(i) as usize]).desc
                                as *mut ::core::ffi::c_char,
                            b"tried\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        {
                            b_i = i;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Aiming unknown rod '%s.'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[(b_i) as usize]).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('z' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    crate::bridge::st().borg.trying_unknown = true_0 != 0;
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_activation(
    mut activation: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut aim: bool,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(activation, true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_activate_item(activation);
    if aim {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
    }
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_ring(
    mut ring_name: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_ring(ring_name) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_activate_ring(ring_name);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_dragon(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if (crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int)) as usize])
    .art_idx
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_dragon(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_arc(rad, dam, typ, (*z_info).max_range as ::core::ffi::c_int);
    if borg_simulate {
        return b_n;
    }
    borg_activate_dragon(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_whirlwind_attack() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut total_d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        WHIRLWIND_ATTACK,
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    let mut blows: ::core::ffi::c_int = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        + 10 as ::core::ffi::c_int)
        / 15 as ::core::ffi::c_int;
    let mut current_block_12: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(distance(crate::bridge::st().borg.c, loc(x, y)) > 1 as ::core::ffi::c_int) {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
            if !(d <= 0 as ::core::ffi::c_int) {
                d = d * blows;
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !(*kill).awake && d <= (*kill).power as ::core::ffi::c_int && !crate::bridge::st().borg.munchkin_mode
                {
                    p = borg_danger_one_kill(
                        y,
                        x,
                        1 as ::core::ffi::c_int,
                        (*ag).kill as ::core::ffi::c_int,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                        current_block_12 = 7502529970979898288;
                    } else {
                        current_block_12 = 3512920355445576850;
                    }
                } else {
                    current_block_12 = 3512920355445576850;
                }
                match current_block_12 {
                    7502529970979898288 => {}
                    _ => {
                        if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            == 0
                            && !(*kill).awake)
                        {
                            p = borg_danger_one_kill(
                                crate::bridge::st().borg.c.y,
                                crate::bridge::st().borg.c.x,
                                2 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                true_0 != 0,
                            );
                            if d <= (*kill).power as ::core::ffi::c_int
                                && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                    > 15 as ::core::ffi::c_int
                            {
                                p = p / 10 as ::core::ffi::c_int;
                            }
                            d += p;
                            total_d += d;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if total_d < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return total_d;
    }
    if borg_spell(WHIRLWIND_ATTACK) {
        return total_d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_leap_into_battle() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        LEAP_INTO_BATTLE,
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if target_closest < 10 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut current_block_19: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut blows: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        let mut m_dist: ::core::ffi::c_int = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(m_dist > 4 as ::core::ffi::c_int) {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if !((*ag).kill == 0) {
                d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
                blows = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    + 5 as ::core::ffi::c_int)
                    / 15 as ::core::ffi::c_int;
                blows = (blows * m_dist + 2 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int;
                d *= blows;
                if !(d <= 0 as ::core::ffi::c_int) {
                    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                    if !((*kill).r_idx as ::core::ffi::c_int
                        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    {
                        if !(*kill).awake
                            && d <= (*kill).power as ::core::ffi::c_int
                            && !crate::bridge::st().borg.munchkin_mode
                        {
                            p = borg_danger_one_kill(
                                y,
                                x,
                                1 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                true_0 != 0,
                            );
                            if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                                current_block_19 = 4906268039856690917;
                            } else {
                                current_block_19 = 13797916685926291137;
                            }
                        } else {
                            current_block_19 = 13797916685926291137;
                        }
                        match current_block_19 {
                            4906268039856690917 => {}
                            _ => {
                                if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                    == 0
                                    && !(*kill).awake)
                                {
                                    p = borg_danger_one_kill(
                                        crate::bridge::st().borg.c.y,
                                        crate::bridge::st().borg.c.x,
                                        2 as ::core::ffi::c_int,
                                        (*ag).kill as ::core::ffi::c_int,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    if d <= (*kill).power as ::core::ffi::c_int
                                        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                            > 15 as ::core::ffi::c_int
                                    {
                                        p = p / 10 as ::core::ffi::c_int;
                                    }
                                    d += p;
                                    if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
                                        b_i = i;
                                        b_d = d;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    crate::bridge::st().borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    crate::bridge::st().borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = crate::bridge::grid(crate::bridge::st().borg.goal.g.x, crate::bridge::st().borg.goal.g.y) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    borg_note(format(
        b"# Leaping at %s at (%d,%d dist %d) who has %d Hit Points.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        crate::bridge::st().borg.goal.g.y,
        crate::bridge::st().borg.goal.g.x,
        distance(crate::bridge::st().borg.c, crate::bridge::st().borg.goal.g),
        (*kill).power as ::core::ffi::c_int,
    ));
    borg_note(format(
        b"# Attacking with weapon '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (crate::bridge::st().items.items[((*z_info).pack_size) as usize]).desc
            as *mut ::core::ffi::c_char,
    ));
    borg_target(crate::bridge::st().borg.goal.g, true_0 != 0);
    borg_spell(LEAP_INTO_BATTLE);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_d;
}
unsafe extern "C" fn borg_attack_aux_maim_foe() -> ::core::ffi::c_int {
    let mut blows: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(
        MAIM_FOE,
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    blows = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        / 15 as ::core::ffi::c_int;
    let mut current_block_15: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(distance(crate::bridge::st().borg.c, loc(x, y)) > 1 as ::core::ffi::c_int) {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int) * blows;
            if !(d <= 0 as ::core::ffi::c_int) {
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !((*kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    if !(*kill).awake
                        && d <= (*kill).power as ::core::ffi::c_int
                        && !crate::bridge::st().borg.munchkin_mode
                    {
                        p = borg_danger_one_kill(
                            y,
                            x,
                            1 as ::core::ffi::c_int,
                            (*ag).kill as ::core::ffi::c_int,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                            current_block_15 = 6483416627284290920;
                        } else {
                            current_block_15 = 1054647088692577877;
                        }
                    } else {
                        current_block_15 = 1054647088692577877;
                    }
                    match current_block_15 {
                        6483416627284290920 => {}
                        _ => {
                            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                == 0
                                && !(*kill).awake)
                            {
                                p = borg_danger_one_kill(
                                    crate::bridge::st().borg.c.y,
                                    crate::bridge::st().borg.c.x,
                                    2 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if d <= (*kill).power as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                        > 15 as ::core::ffi::c_int
                                {
                                    p = p / 10 as ::core::ffi::c_int;
                                }
                                d += p;
                                if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
                                    b_i = i;
                                    b_d = d;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    crate::bridge::st().borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    crate::bridge::st().borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = crate::bridge::grid(crate::bridge::st().borg.goal.g.x, crate::bridge::st().borg.goal.g.y) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    dir = borg_extract_dir(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, crate::bridge::st().borg.goal.g.y, crate::bridge::st().borg.goal.g.x);
    if borg_simulate {
        return d;
    }
    borg_spell(MAIM_FOE);
    borg_keypress((dir + '0' as i32) as keycode_t);
    return d;
}
unsafe extern "C" fn borg_attack_aux_vampire_strike() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut best_dist: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        VAMPIRE_STRIKE,
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        dist = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(dist > best_dist) {
            best_dist = dist;
            b_i = i;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if dist >= 20 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    let mut found: bool = false_0 != 0;
    o_x = -(1 as ::core::ffi::c_int);
    while o_x <= 1 as ::core::ffi::c_int && !found {
        o_y = -(1 as ::core::ffi::c_int);
        while o_y <= 1 as ::core::ffi::c_int && !found {
            if !(o_x == 0 && o_y == 0) {
                x2 = borg_temp_x[b_i as usize] as ::core::ffi::c_int + o_x;
                y2 = borg_temp_y[b_i as usize] as ::core::ffi::c_int + o_y;
                ag = crate::bridge::grid(x2, y2) as *mut borg_grid;
                if (*ag).kill == 0
                    && (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
                    && !(*ag).web
                    && !(*ag).glyph
                    && (y2 != crate::bridge::st().borg.c.y || x2 != crate::bridge::st().borg.c.x)
                {
                    found = true_0 != 0;
                }
            }
            o_y += 1;
        }
        o_x += 1;
    }
    if !found {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_offset_projectable(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) {
        return 0 as ::core::ffi::c_int;
    }
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    d = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        * 2 as ::core::ffi::c_int;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NONLIVING as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*kill).awake && d <= (*kill).power as ::core::ffi::c_int && !crate::bridge::st().borg.munchkin_mode {
        p = borg_danger_one_kill(
            y,
            x,
            1 as ::core::ffi::c_int,
            (*ag).kill as ::core::ffi::c_int,
            true_0 != 0,
            true_0 != 0,
        );
        if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    p = borg_danger_one_kill(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        2 as ::core::ffi::c_int,
        (*ag).kill as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    );
    if d <= (*kill).power as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            > 15 as ::core::ffi::c_int
    {
        p = p / 10 as ::core::ffi::c_int;
    }
    d += p;
    if target_closest < 0 as ::core::ffi::c_int && d > 0 as ::core::ffi::c_int {
        target_closest = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    borg_spell(VAMPIRE_STRIKE);
    return d;
}
unsafe extern "C" fn borg_attack_aux_crush() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_spell_okay(CRUSH) {
        return 0 as ::core::ffi::c_int;
    }
    if (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + 10 as ::core::ffi::c_int)
        < crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            * 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    borg_crush_spell = false_0 != 0;
    p1 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_crush_spell = true_0 != 0;
    p2 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_crush_spell = false_0 != 0;
    d = p1 - p2;
    let mut new_hp: ::core::ffi::c_int =
        crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int;
    if borg_simulate as ::core::ffi::c_int != 0
        && (p2 >= new_hp * 2 as ::core::ffi::c_int || new_hp <= 50 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(CRUSH);
    d = d - spell_power;
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        d = d - spell_power * 10 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    if borg_spell(CRUSH) {
        return d;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn borg_attack_aux_trance() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_spell_okay(TRANCE) {
        return 0 as ::core::ffi::c_int;
    }
    borg_sleep_spell_ii = false_0 != 0;
    p1 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = true_0 != 0;
    p2 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = false_0 != 0;
    d = p1 - p2;
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(TRANCE);
    d = d - spell_power;
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] - spell_power
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        d = d - spell_power * 10 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    if borg_spell(TRANCE) {
        return d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_artifact_holcolleth() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_equips_item(crate::bridge::st().act.sleepii, true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    borg_sleep_spell_ii = false_0 != 0;
    p1 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = true_0 != 0;
    p2 = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = false_0 != 0;
    d = p1 - p2;
    if borg_simulate {
        return d;
    }
    if borg_activate_item(crate::bridge::st().act.sleepii) {
        return d;
    } else {
        borg_note(
            b"# Failed to properly activate the artifact\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_calculate_attack_effectiveness(
    mut attack_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match attack_type {
        0 => return borg_attack_aux_rest(),
        1 => return borg_attack_aux_thrust(),
        3 => return borg_attack_aux_launch(),
        2 => return borg_attack_aux_object(),
        10 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SLOW_MONSTER,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        15 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                CONFUSE_MONSTER,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        11 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                MASS_SLEEP,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        4 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MAGIC_MISSILE,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        5 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
            return borg_attack_aux_spell_bolt_reserve(
                MAGIC_MISSILE,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
            );
        }
        8 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - 5 as ::core::ffi::c_int)
                / 3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FROST_BOLT,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        9 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int + 30 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                TURN_STONE_TO_MUD,
                rad,
                dam,
                BORG_ATTACK_KILL_WALL as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        7 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SPEAR_OF_LIGHT,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        6 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                STINKING_CLOUD,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        12 => {
            rad = 2 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FIRE_BALL,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        16 => {
            dam = 3 as ::core::ffi::c_int
                * (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                ICE_STORM,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
            );
        }
        17 => {
            rad = 1 as ::core::ffi::c_int;
            dam = 30 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 20 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                METEOR_SWARM,
                rad,
                dam,
                BORG_ATTACK_METEOR as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        18 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 3 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                RIFT,
                rad,
                dam,
                BORG_ATTACK_GRAVITY as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        19 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MANA_STORM,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        13 => {
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int;
            rad = 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SHOCK_WAVE,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        14 => {
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 5 as ::core::ffi::c_int;
            rad = 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                EXPLOSION,
                rad,
                dam,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        22 => {
            rad = if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                >= 30 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 3 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ORB_OF_DRAINING,
                rad,
                dam,
                BORG_ATTACK_HOLY_ORB as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        20 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FRIGHTEN,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        21 => return borg_attack_aux_trance(),
        23 => {
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                DISPEL_UNDEAD,
                dam,
                BORG_ATTACK_DISP_UNDEAD as ::core::ffi::c_int,
            );
        }
        24 => {
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                DISPEL_EVIL,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        25 => {
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                BANISH_SPIRITS,
                dam,
                BORG_ATTACK_DISP_SPIRITS as ::core::ffi::c_int,
            );
        }
        26 => {
            if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                >= 300 as ::core::ffi::c_int
            {
                dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 10 as ::core::ffi::c_int;
                return borg_attack_aux_spell_dispel(
                    HOLY_WORD,
                    dam,
                    BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                );
            } else {
                dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 3 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int
                    - 50 as ::core::ffi::c_int;
                return borg_attack_aux_spell_dispel(
                    DISPEL_EVIL,
                    dam,
                    BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                );
            }
        }
        27 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 4 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ANNIHILATE,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        28 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ELECTRIC_ARC,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize],
                false_0 != 0,
            );
        }
        29 => {
            rad = 60 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ACID_SPRAY,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                10 as ::core::ffi::c_int,
                true_0 != 0,
            );
        }
        30 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - 10 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MANA_BOLT,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        31 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                THRUST_AWAY,
                rad,
                dam,
                BORG_ATTACK_FORCE as ::core::ffi::c_int,
                crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 10 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        32 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 4 as ::core::ffi::c_int
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                + 5 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                LIGHTNING_STRIKE,
                rad,
                dam,
                BORG_ATTACK_ELEC_STRIKE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        33 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                + 5 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                EARTH_RISING,
                rad,
                dam,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
                crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    / 5 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        34 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 3 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
                * (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                VOLCANIC_ERUPTION,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        35 => {
            rad = 20 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                + 10 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                RIVER_OF_LIGHTNING,
                rad,
                dam,
                BORG_ATTACK_PLASMA as ::core::ffi::c_int,
                20 as ::core::ffi::c_int,
                true_0 != 0,
            );
        }
        36 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
                + (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SPEAR_OF_OROME,
                rad,
                dam,
                BORG_ATTACK_HOLY_ORB as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        37 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 5 as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                LIGHT_OF_VARDA,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        38 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                NETHER_BOLT,
                rad,
                dam,
                BORG_ATTACK_NETHER as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        39 => {
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                / 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                TAP_UNLIFE,
                dam,
                BORG_ATTACK_TAP_UNLIFE as ::core::ffi::c_int,
            );
        }
        40 => return borg_attack_aux_crush(),
        41 => {
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 10 as ::core::ffi::c_int
                + 500 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                SLEEP_EVIL,
                dam,
                BORG_ATTACK_SLEEP_EVIL as ::core::ffi::c_int,
            );
        }
        42 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DISENCHANT,
                rad,
                dam,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        43 => {
            rad = 0 as ::core::ffi::c_int;
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize];
            return borg_attack_aux_spell_bolt(
                FRIGHTEN,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        44 => return borg_attack_aux_vampire_strike(),
        45 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DISPEL_LIFE,
                rad,
                dam,
                BORG_ATTACK_DRAIN_LIFE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        46 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DARK_SPEAR,
                rad,
                dam,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        47 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 8 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                UNLEASH_CHAOS,
                rad,
                dam,
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        48 => {
            rad = 4 as ::core::ffi::c_int;
            dam = (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 4 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                STORM_OF_DARKNESS,
                rad,
                dam,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        49 => {
            if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < 120 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            rad = 0 as ::core::ffi::c_int;
            dam = -(1 as ::core::ffi::c_int);
            return borg_attack_aux_spell_bolt(
                CURSE,
                rad,
                dam,
                BORG_ATTACK_CURSE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        50 => return borg_attack_aux_whirlwind_attack(),
        51 => return borg_attack_aux_leap_into_battle(),
        52 => return borg_attack_aux_maim_foe(),
        53 => {
            dam = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize];
            return borg_attack_aux_spell_dispel(
                HOWL_OF_THE_DAMNED,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
            );
        }
        64 => {
            dam = 10 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_slow_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
            );
        }
        65 => {
            dam = 10 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_sleep_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        54 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        55 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
            );
        }
        56 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        57 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        58 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_light,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
            );
        }
        59 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_drain_life,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
            );
        }
        60 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        61 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_cold_ball,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
            );
        }
        62 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        63 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                crate::bridge::st().item_val.sv_rod_fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        66 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt_unknown(
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
            );
        }
        72 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt_unknown(
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
            );
        }
        73 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 3 as ::core::ffi::c_int * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_magic_missile,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        78 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_slow_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        79 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_hold_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        81 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_fear_monster,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        80 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_confuse_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        74 => {
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            rad = -(1 as ::core::ffi::c_int);
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        75 => {
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        76 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        77 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        84 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_light,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        85 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_stinking_cloud,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        86 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        87 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_cold_ball,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        88 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        89 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        91 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 160 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_dragon_cold,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        92 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_dragon_fire,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        82 => {
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_annihilation,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        83 => {
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_drain_life,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        90 => {
            dam = 35 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                crate::bridge::st().item_val.sv_wand_wonder,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        67 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                crate::bridge::st().item_val.sv_staff_sleep_monsters,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        68 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                crate::bridge::st().item_val.sv_staff_slow_monsters,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
            );
        }
        69 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                crate::bridge::st().item_val.sv_staff_dispel_evil,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        70 => {
            dam = 120 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                crate::bridge::st().item_val.sv_staff_power,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
            );
        }
        71 => {
            if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int
            {
                dam = 500 as ::core::ffi::c_int;
            } else {
                dam = 120 as ::core::ffi::c_int;
            }
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                crate::bridge::st().item_val.sv_staff_holiness,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        109 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 9 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        110 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 72 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_ball72,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        111 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        112 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        113 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 50 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_ball50,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        115 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_bolt2,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        114 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_ball100,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        123 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        118 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.stinking_cloud,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        117 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.drain_life2,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        119 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.confuse2,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        122 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_artifact_holcolleth();
        }
        116 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 90 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.drain_life1,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        120 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.arrow,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        121 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 3 as ::core::ffi::c_int * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.missile,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        124 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        125 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 5 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dispel_evil,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        126 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.mana_bolt,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        127 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.star_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        128 => {
            rad = 7 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.starlight2,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        129 => {
            rad = 7 as ::core::ffi::c_int;
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.starlight,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        130 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.mon_slow,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        131 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.mon_confuse,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        132 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.sleep_all,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        133 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.mon_scare,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        134 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.light_line,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        135 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.drain_life3,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        136 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.drain_life4,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        137 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        138 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.elec_ball2,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        139 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.acid_bolt2,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        140 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.acid_bolt2,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        141 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        142 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 160 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_ball160,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        143 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.cold_ball2,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        144 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_ball2,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        145 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_ball200,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        146 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_bolt2,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        147 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 16 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.fire_bolt3,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        148 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dispel_evil60,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        149 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dispel_undead,
                rad,
                dam,
                BORG_ATTACK_DISP_UNDEAD as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        150 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dispel_undead,
                rad,
                dam,
                BORG_ATTACK_DISP_ALL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        151 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.losslow,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        152 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.lossleep,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        153 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int
                + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.losconf,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        154 => {
            dam = 5 as ::core::ffi::c_int
                + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.wonder,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        155 => {
            if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 2 as ::core::ffi::c_int
            {
                dam = 500 as ::core::ffi::c_int;
            } else {
                dam = 120 as ::core::ffi::c_int;
            }
            return borg_attack_aux_activation(
                crate::bridge::st().act.staff_holy,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        156 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 70 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.ring_acid,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        157 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 80 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.ring_flames,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        158 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.ring_ice,
                rad,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        159 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 85 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.ring_lightning,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        160 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dragon_blue,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        161 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dragon_green,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        162 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dragon_red,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        163 => {
            let mut value: [::core::ffi::c_int; 5] = [0; 5];
            let mut type_0: [::core::ffi::c_int; 5] = [
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            ];
            let mut biggest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 5 as ::core::ffi::c_int {
                value[x as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_multihued,
                    rad,
                    dam,
                    type_0[x as usize],
                    true_0 != 0,
                    x,
                );
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_0 < 5 as ::core::ffi::c_int {
                if value[x_0 as usize] > value[biggest as usize] {
                    biggest = x_0;
                }
                x_0 += 1;
            }
            borg_simulate = tmp_simulate;
            if !borg_simulate {
                value[biggest as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_multihued,
                    rad,
                    dam,
                    type_0[biggest as usize],
                    true_0 != 0,
                    biggest,
                );
            }
            return value[biggest as usize];
        }
        164 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dragon_gold,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        165 => {
            let mut value_0: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_1: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
            ];
            let mut biggest_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_0: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_1 < 2 as ::core::ffi::c_int {
                value_0[x_1 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_chaos,
                    rad,
                    dam,
                    type_1[x_1 as usize],
                    true_0 != 0,
                    x_1,
                );
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_2 < 2 as ::core::ffi::c_int {
                if value_0[x_2 as usize] > value_0[biggest_0 as usize] {
                    biggest_0 = x_2;
                }
                x_2 += 1;
            }
            borg_simulate = tmp_simulate_0;
            if !borg_simulate {
                value_0[biggest_0 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_chaos,
                    rad,
                    dam,
                    type_1[biggest_0 as usize],
                    true_0 != 0,
                    biggest_0,
                );
            }
            return value_0[biggest_0 as usize];
        }
        166 => {
            let mut value_1: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_2: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_1: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_3 < 2 as ::core::ffi::c_int {
                value_1[x_3 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_law,
                    rad,
                    dam,
                    type_2[x_3 as usize],
                    true_0 != 0,
                    x_3,
                );
                x_3 += 1;
            }
            let mut x_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_4 < 2 as ::core::ffi::c_int {
                if value_1[x_4 as usize] > value_1[biggest_1 as usize] {
                    biggest_1 = x_4;
                }
                x_4 += 1;
            }
            borg_simulate = tmp_simulate_1;
            if !borg_simulate {
                value_1[biggest_1 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_law,
                    rad,
                    dam,
                    type_2[biggest_1 as usize],
                    true_0 != 0,
                    biggest_1,
                );
            }
            return value_1[biggest_1 as usize];
        }
        167 => {
            let mut value_2: [::core::ffi::c_int; 4] = [0; 4];
            let mut type_3: [::core::ffi::c_int; 4] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_2: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_5 < 4 as ::core::ffi::c_int {
                value_2[x_5 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_balance,
                    rad,
                    dam,
                    type_3[x_5 as usize],
                    true_0 != 0,
                    x_5,
                );
                x_5 += 1;
            }
            let mut x_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_6 < 4 as ::core::ffi::c_int {
                if value_2[x_6 as usize] > value_2[biggest_2 as usize] {
                    biggest_2 = x_6;
                }
                x_6 += 1;
            }
            borg_simulate = tmp_simulate_2;
            if !borg_simulate {
                value_2[biggest_2 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_balance,
                    rad,
                    dam,
                    type_3[biggest_2 as usize],
                    true_0 != 0,
                    biggest_2,
                );
            }
            return value_2[biggest_2 as usize];
        }
        168 => {
            let mut value_3: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_4: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
            ];
            let mut biggest_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_3: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_7 < 2 as ::core::ffi::c_int {
                value_3[x_7 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_shining,
                    rad,
                    dam,
                    type_4[x_7 as usize],
                    true_0 != 0,
                    x_7,
                );
                x_7 += 1;
            }
            let mut x_8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_8 < 2 as ::core::ffi::c_int {
                if value_3[x_8 as usize] > value_3[biggest_3 as usize] {
                    biggest_3 = x_8;
                }
                x_8 += 1;
            }
            borg_simulate = tmp_simulate_3;
            if !borg_simulate {
                value_3[biggest_3 as usize] = borg_attack_aux_activation(
                    crate::bridge::st().act.dragon_shining,
                    rad,
                    dam,
                    type_4[biggest_3 as usize],
                    true_0 != 0,
                    biggest_3,
                );
            }
            return value_3[biggest_3 as usize];
        }
        169 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                crate::bridge::st().act.dragon_power,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        93 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 70 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                crate::bridge::st().item_val.sv_ring_acid,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        94 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 80 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                crate::bridge::st().item_val.sv_ring_flames,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        95 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                crate::bridge::st().item_val.sv_ring_ice,
                rad,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
            );
        }
        96 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 85 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                crate::bridge::st().item_val.sv_ring_lightning,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        97 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_blue,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        98 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_white,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        99 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_black,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        100 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_green,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        101 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_red,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        102 => {
            let mut value_4: [::core::ffi::c_int; 5] = [0; 5];
            let mut type_5: [::core::ffi::c_int; 5] = [
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            ];
            let mut biggest_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_4: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_9 < 5 as ::core::ffi::c_int {
                value_4[x_9 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_multihued,
                    rad,
                    dam,
                    type_5[x_9 as usize],
                    x_9,
                );
                x_9 += 1;
            }
            let mut x_10: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_10 < 5 as ::core::ffi::c_int {
                if value_4[x_10 as usize] > value_4[biggest_4 as usize] {
                    biggest_4 = x_10;
                }
                x_10 += 1;
            }
            borg_simulate = tmp_simulate_4;
            if !borg_simulate {
                value_4[biggest_4 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_multihued,
                    rad,
                    dam,
                    type_5[biggest_4 as usize],
                    biggest_4,
                );
            }
            return value_4[biggest_4 as usize];
        }
        103 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_gold,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        104 => {
            let mut value_5: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_6: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
            ];
            let mut biggest_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_5: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_11 < 2 as ::core::ffi::c_int {
                value_5[x_11 as usize] =
                    borg_attack_aux_dragon(crate::bridge::st().item_val.sv_dragon_chaos, rad, dam, type_6[x_11 as usize], x_11);
                x_11 += 1;
            }
            let mut x_12: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_12 < 2 as ::core::ffi::c_int {
                if value_5[x_12 as usize] > value_5[biggest_5 as usize] {
                    biggest_5 = x_12;
                }
                x_12 += 1;
            }
            borg_simulate = tmp_simulate_5;
            if !borg_simulate {
                value_5[biggest_5 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_chaos,
                    rad,
                    dam,
                    type_6[biggest_5 as usize],
                    biggest_5,
                );
            }
            return value_5[biggest_5 as usize];
        }
        105 => {
            let mut value_6: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_7: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_6: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_13: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_13 < 2 as ::core::ffi::c_int {
                value_6[x_13 as usize] =
                    borg_attack_aux_dragon(crate::bridge::st().item_val.sv_dragon_law, rad, dam, type_7[x_13 as usize], x_13);
                x_13 += 1;
            }
            let mut x_14: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_14 < 2 as ::core::ffi::c_int {
                if value_6[x_14 as usize] > value_6[biggest_6 as usize] {
                    biggest_6 = x_14;
                }
                x_14 += 1;
            }
            borg_simulate = tmp_simulate_6;
            if !borg_simulate {
                value_6[biggest_6 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_law,
                    rad,
                    dam,
                    type_7[biggest_6 as usize],
                    biggest_6,
                );
            }
            return value_6[biggest_6 as usize];
        }
        106 => {
            let mut value_7: [::core::ffi::c_int; 4] = [0; 4];
            let mut type_8: [::core::ffi::c_int; 4] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_7: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_15: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_15 < 4 as ::core::ffi::c_int {
                value_7[x_15 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_balance,
                    rad,
                    dam,
                    type_8[x_15 as usize],
                    x_15,
                );
                x_15 += 1;
            }
            let mut x_16: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_16 < 4 as ::core::ffi::c_int {
                if value_7[x_16 as usize] > value_7[biggest_7 as usize] {
                    biggest_7 = x_16;
                }
                x_16 += 1;
            }
            borg_simulate = tmp_simulate_7;
            if !borg_simulate {
                value_7[biggest_7 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_balance,
                    rad,
                    dam,
                    type_8[biggest_7 as usize],
                    biggest_7,
                );
            }
            return value_7[biggest_7 as usize];
        }
        107 => {
            let mut value_8: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_9: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
            ];
            let mut biggest_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_8: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_17: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_17 < 2 as ::core::ffi::c_int {
                value_8[x_17 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_shining,
                    rad,
                    dam,
                    type_9[x_17 as usize],
                    x_17,
                );
                x_17 += 1;
            }
            let mut x_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_18 < 2 as ::core::ffi::c_int {
                if value_8[x_18 as usize] > value_8[biggest_8 as usize] {
                    biggest_8 = x_18;
                }
                x_18 += 1;
            }
            borg_simulate = tmp_simulate_8;
            if !borg_simulate {
                value_8[biggest_8 as usize] = borg_attack_aux_dragon(
                    crate::bridge::st().item_val.sv_dragon_shining,
                    rad,
                    dam,
                    type_9[biggest_8 as usize],
                    biggest_8,
                );
            }
            return value_8[biggest_8 as usize];
        }
        108 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                crate::bridge::st().item_val.sv_dragon_power,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack(mut boosted_bravery: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut adjacent_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    let mut current_block_23: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(crate::bridge::st().borg.time.now - (*kill).when > 2 as borg_time) {
                    if !(crate::bridge::st().borg.goal.ignoring as ::core::ffi::c_int != 0
                        && crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                            == 0
                        && flag_has(
                            &raw mut (*r_ptr).flags as *mut bitflag,
                            (RF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RF_MULTIPLY as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0)
                    {
                        if !(crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                < 10 as ::core::ffi::c_int
                            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                == 0 as ::core::ffi::c_int
                            && !strstr(
                                (*r_ptr).name,
                                b"Farmer\0" as *const u8 as *const ::core::ffi::c_char,
                            )
                            .is_null())
                        {
                            if (*kill).speed as ::core::ffi::c_int
                                > crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                                && distance(crate::bridge::st().borg.c, (*kill).pos) <= 2 as ::core::ffi::c_int
                                || distance(crate::bridge::st().borg.c, (*kill).pos) <= 1 as ::core::ffi::c_int
                            {
                                adjacent_monster = true_0 != 0;
                            }
                            if crate::bridge::st().borg.mon.scary {
                                if !strstr(
                                    (*r_ptr).name,
                                    b"Grip\0" as *const u8 as *const ::core::ffi::c_char,
                                )
                                .is_null()
                                    || !strstr(
                                        (*r_ptr).name,
                                        b"Fang\0" as *const u8 as *const ::core::ffi::c_char,
                                    )
                                    .is_null()
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                    <= 5 as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                        != 0 as ::core::ffi::c_int
                                    && flag_has(
                                        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags
                                            as *mut bitflag,
                                        (RF_MAX as ::core::ffi::c_int as size_t)
                                            .wrapping_add(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            )
                                            .wrapping_sub(1 as size_t)
                                            .wrapping_div(
                                                (::core::mem::size_of::<bitflag>() as size_t)
                                                    .wrapping_mul(8 as size_t),
                                            ),
                                        RF_MULTIPLY as ::core::ffi::c_int,
                                    ) as ::core::ffi::c_int
                                        != 0
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level >= 2000 as borg_time
                                    || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town >= 3000 as borg_time
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if boosted_bravery as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.no_retreat as ::core::ffi::c_int
                                        >= 1 as ::core::ffi::c_int
                                    || crate::bridge::st().borg.goal.recalling != 0
                                    || crate::bridge::st().borg.goal.descending != 0
                                {
                                    borg_note(
                                        b"# Bored, or recalling and fighting a monster on Scaryguy Level.\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                    * 4 as ::core::ffi::c_int
                                    <= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                    && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        > 10 as ::core::ffi::c_int
                                {
                                    borg_note(
                                        b"# High clevel fighting monster on Scaryguy Level.\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else if adjacent_monster {
                                    borg_note(
                                        b"# Adjacent to monster on Scaryguy Level.\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else {
                                    current_block_23 = 15427931788582360902;
                                }
                            } else {
                                current_block_23 = 4761528863920922185;
                            }
                            match current_block_23 {
                                15427931788582360902 => {}
                                _ => {
                                    ag = crate::bridge::grid((*kill).pos.x, (*kill).pos.y)
                                        as *mut borg_grid;
                                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                                            if !(distance(crate::bridge::st().borg.c, (*kill).pos)
                                                > (*z_info).max_range as ::core::ffi::c_int)
                                            {
                                                if (*ag).kill == 0 {
                                                    (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y))
                                                    .kill = i as uint8_t;
                                                }
                                                borg_temp_x[borg_temp_n as usize] =
                                                    (*kill).pos.x as uint8_t;
                                                borg_temp_y[borg_temp_n as usize] =
                                                    (*kill).pos.y as uint8_t;
                                                borg_temp_n += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    let mut max_attacks: ::core::ffi::c_int = BF_MAX as ::core::ffi::c_int;
    if !(*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] {
        max_attacks = BF_ACT_STARLIGHT as ::core::ffi::c_int;
    }
    g = 0 as ::core::ffi::c_int;
    while g < max_attacks {
        n = borg_calculate_attack_effectiveness(g);
        if !(n <= b_n) {
            b_g = g;
            b_n = n;
        }
        g += 1;
    }
    if b_n <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing attack type %d with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(b_g);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
