pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const SHRT_MAX: ::core::ffi::c_int = __SHRT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


use crate::headers::borg_danger_h::{
    borg_danger, borg_danger_one_kill, borg_danger_wipe, borg_fear_region,
};
pub use crate::headers::borg_fight_attack_h::{
    borg_attack_aux_launch, borg_attack_aux_spell_bolt, C2RustUnnamedH_32, BORG_ATTACK_ACID,
    BORG_ATTACK_ARROW, BORG_ATTACK_AWAY_ALL, BORG_ATTACK_AWAY_ALL_MORGOTH, BORG_ATTACK_AWAY_EVIL,
    BORG_ATTACK_AWAY_UNDEAD, BORG_ATTACK_CHAOS, BORG_ATTACK_COLD, BORG_ATTACK_CONFU,
    BORG_ATTACK_CURSE, BORG_ATTACK_DARK, BORG_ATTACK_DARK_WEAK, BORG_ATTACK_DISEN,
    BORG_ATTACK_DISP_ALL, BORG_ATTACK_DISP_EVIL, BORG_ATTACK_DISP_SPIRITS, BORG_ATTACK_DISP_UNDEAD,
    BORG_ATTACK_DRAIN_LIFE, BORG_ATTACK_ELEC, BORG_ATTACK_ELEC_STRIKE, BORG_ATTACK_FIRE,
    BORG_ATTACK_FORCE, BORG_ATTACK_GRAVITY, BORG_ATTACK_HOLY_ORB, BORG_ATTACK_HOLY_WORD,
    BORG_ATTACK_ICE, BORG_ATTACK_INERTIA, BORG_ATTACK_KILL_DOOR, BORG_ATTACK_KILL_TRAP,
    BORG_ATTACK_KILL_WALL, BORG_ATTACK_LIGHT, BORG_ATTACK_LIGHT_WEAK, BORG_ATTACK_MAKE_DOOR,
    BORG_ATTACK_MAKE_TRAP, BORG_ATTACK_MAKE_WALL, BORG_ATTACK_MANA, BORG_ATTACK_METEOR,
    BORG_ATTACK_MISSILE, BORG_ATTACK_NETHER, BORG_ATTACK_NEXUS, BORG_ATTACK_OLD_CLONE,
    BORG_ATTACK_OLD_CONF, BORG_ATTACK_OLD_DRAIN, BORG_ATTACK_OLD_HEAL, BORG_ATTACK_OLD_POLY,
    BORG_ATTACK_OLD_SLEEP, BORG_ATTACK_OLD_SLOW, BORG_ATTACK_OLD_SPEED, BORG_ATTACK_PLASMA,
    BORG_ATTACK_POIS, BORG_ATTACK_SHARD, BORG_ATTACK_SLEEP_EVIL, BORG_ATTACK_SOUND,
    BORG_ATTACK_TAP_UNLIFE, BORG_ATTACK_TIME, BORG_ATTACK_TURN_ALL, BORG_ATTACK_TURN_EVIL,
    BORG_ATTACK_TURN_UNDEAD, BORG_ATTACK_WATER,
};
pub use crate::headers::borg_flow_h::{
    borg_can_dig, borg_flow_clear, borg_flow_commit, borg_flow_enqueue_grid, borg_flow_old,
    borg_flow_spread, borg_flow_x, borg_flow_y, borg_temp_n, borg_temp_x, borg_temp_y, borg_track,
};
pub use crate::headers::borg_flow_kill_h::borg_kill;
use crate::headers::borg_flow_misc_h::borg_flow_far_from_stairs;
use crate::headers::borg_flow_stairs_h::track_less;
pub use crate::headers::borg_flow_take_h::{
    borg_delete_take, borg_take, borg_takes, borg_takes_nxt,
};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_massage_special_chars, borg_note};

use crate::headers::borg_item_use_h::{borg_equips_item, borg_equips_ring};
use crate::headers::borg_item_val_h::sv_ring_digging;
pub use crate::headers::borg_magic_h::{
    borg_primarily_caster, borg_spell_okay, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_projection_h::{borg_distance, borg_los, borg_projectable};
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
use crate::headers::borg_update_h::borg_detect_wall;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, distance, heatmap, monster_group, square,
    square_in_bounds_fully, square_isvault, square_monster, trap, C2RustUnnamedH_18, FEAT_MAX,
};
use crate::headers::game_world_h::extract_energy;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::internal::__SHRT_MAX__;

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
pub use crate::headers::list_mon_timed_h::{
    MON_TMD_CHANGED, MON_TMD_COMMAND, MON_TMD_CONF, MON_TMD_DISEN, MON_TMD_FAST, MON_TMD_FEAR,
    MON_TMD_HOLD, MON_TMD_MAX, MON_TMD_SLEEP, MON_TMD_SLOW, MON_TMD_STUN,
};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::mon_timed_h::C2RustUnnamedH_33;
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, C2RustUnnamedH_30, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN,
    MON_GROUP_BODYGUARD, MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
    RF_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memset, strcmp, strlen};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width, Term,
};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{
    my_rational, my_strcpy, my_stristr, my_strnicmp, prefix, prefix_i, suffix,
};
use crate::headers::z_virt_h::{mem_free, mem_zalloc, string_free};
#[no_mangle]
pub static mut borg_kills_cnt: int16_t = 0;
#[no_mangle]
pub static mut borg_kills_nxt: int16_t = 0;
#[no_mangle]
pub static mut borg_kills: *mut borg_kill = ::core::ptr::null::<borg_kill>() as *mut borg_kill;
#[no_mangle]
pub static mut borg_race_count: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
#[no_mangle]
pub static mut borg_race_death: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
static mut borg_unique_size: ::core::ffi::c_int = 0;
static mut borg_unique_what: *mut ::core::ffi::c_uint =
    ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint;
static mut borg_unique_text: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
static mut borg_normal_size: ::core::ffi::c_int = 0;
static mut borg_normal_what: *mut ::core::ffi::c_uint =
    ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint;
static mut borg_normal_text: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_nasties_num: uint8_t = 7 as uint8_t;
#[no_mangle]
pub static mut borg_nasties_count: [uint8_t; 7] = [0; 7];
#[no_mangle]
pub static mut borg_nasties: [::core::ffi::c_char; 7] = [
    'Z' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut borg_nasties_limit: [uint8_t; 7] = [
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn borg_race_name(
    mut r_idx: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if r_idx == 0 {
        return b"dead monster\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if r_idx >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        return b"player ghost\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return (*r_info.offset(r_idx as isize)).name;
}
unsafe extern "C" fn borg_update_kill_new(mut i: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pct: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut m_ptr: *mut monster = (*cave).monsters.offset((*kill).m_idx as isize) as *mut monster;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    (*kill).speed = (*m_ptr).mspeed;
    if (*m_ptr).maxhp != 0 {
        pct = (100 as ::core::ffi::c_long * (*m_ptr).hp as ::core::ffi::c_long
            / (if (*m_ptr).maxhp as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*m_ptr).maxhp as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    } else {
        pct = 100 as ::core::ffi::c_int;
    }
    (*kill).power =
        ((*m_ptr).maxhp as ::core::ffi::c_int * pct / 100 as ::core::ffi::c_int) as int16_t;
    (*kill).injury = (100 as ::core::ffi::c_int - pct) as int16_t;
    (*kill).level = (*r_ptr).level as int16_t;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) {
        (*kill).awake = true_0 != 0;
    }
    (*kill).m_idx = (*square_monster(cave, loc((*kill).pos.x, (*kill).pos.y))).midx as int16_t;
    if (*m_ptr).m_timed[MON_TMD_SLEEP as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).awake = true_0 != 0;
    } else {
        (*kill).awake = false_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_FEAR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).afraid = false_0 != 0;
    } else {
        (*kill).afraid = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_CONF as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).confused = false_0 != 0;
    } else {
        (*kill).confused = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_STUN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).stunned = false_0 != 0;
    } else {
        (*kill).stunned = true_0 != 0;
    }
    (*kill).spell_flags[0 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[0 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[1 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[1 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[2 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[2 as ::core::ffi::c_int as usize] as uint32_t;
    k = RSF_NONE as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while k < RSF_MAX as ::core::ffi::c_int {
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
            k,
        ) {
            let fresh2 = num;
            num = num + 1;
            (*kill).spell[fresh2 as usize] = k as uint8_t;
        }
        k += 1;
    }
    (*kill).ranged_attack = num as uint8_t;
    if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
        j = ((*kill).pos.y - crate::borg_cave_util::borg_panel_hgt() / 2 as ::core::ffi::c_int) / crate::borg_cave_util::borg_panel_hgt()
            * crate::borg_cave_util::borg_panel_hgt();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_HGT
            - ((*Term).hgt
                - row_top_map[(*Term).sidebar_mode as usize]
                - row_bottom_map[(*Term).sidebar_mode as usize])
                / tile_height as ::core::ffi::c_int
        {
            j = DUNGEON_HGT
                - ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.mon.morgoth_panel.y = j;
        j = ((*kill).pos.x - crate::borg_cave_util::borg_panel_wid() / 2 as ::core::ffi::c_int) / crate::borg_cave_util::borg_panel_wid()
            * crate::borg_cave_util::borg_panel_wid();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_WID
            - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            j = DUNGEON_WID
                - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                    / tile_width as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.mon.morgoth_panel.x = j;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat =
            FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat
        as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
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
            RF_PASS_WALL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat =
            FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
}
unsafe extern "C" fn borg_update_kill_old(mut i: ::core::ffi::c_int) {
    let mut t: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pct: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut m_ptr: *mut monster = square_monster(cave, loc((*kill).pos.x, (*kill).pos.y));
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*m_ptr).maxhp != 0 {
        pct = (100 as ::core::ffi::c_long * (*m_ptr).hp as ::core::ffi::c_long
            / (if (*m_ptr).maxhp as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*m_ptr).maxhp as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    } else {
        pct = 100 as ::core::ffi::c_int;
    }
    (*kill).power =
        ((*m_ptr).maxhp as ::core::ffi::c_int * pct / 100 as ::core::ffi::c_int) as int16_t;
    (*kill).injury = (100 as ::core::ffi::c_int - pct) as int16_t;
    if (*m_ptr).m_timed[MON_TMD_SLEEP as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).awake = true_0 != 0;
    } else {
        (*kill).awake = false_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_FEAR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).afraid = false_0 != 0;
    } else {
        (*kill).afraid = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_CONF as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).confused = false_0 != 0;
    } else {
        (*kill).confused = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_STUN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).stunned = false_0 != 0;
    } else {
        (*kill).stunned = true_0 != 0;
    }
    (*kill).m_idx = (*square_monster(cave, loc((*kill).pos.x, (*kill).pos.y))).midx as int16_t;
    (*kill).speed = (*m_ptr).mspeed;
    e = extract_energy[crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] as usize]
        as ::core::ffi::c_int;
    t = (100 as ::core::ffi::c_int + (e - 1 as ::core::ffi::c_int)) / e;
    e = extract_energy[(*kill).speed as usize] as ::core::ffi::c_int;
    (*kill).moves = (t * e / 10 as ::core::ffi::c_int) as uint8_t;
    (*kill).spell_flags[0 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[0 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[1 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[1 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[2 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[2 as ::core::ffi::c_int as usize] as uint32_t;
    k = RSF_NONE as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while k < RSF_MAX as ::core::ffi::c_int {
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
            k,
        ) {
            let fresh1 = num;
            num = num + 1;
            (*kill).spell[fresh1 as usize] = k as uint8_t;
        }
        k += 1;
    }
    (*kill).ranged_attack = num as uint8_t;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        *borg_race_death.offset(i as isize) = 1 as int16_t;
    }
    if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
        j = ((*kill).pos.y - crate::borg_cave_util::borg_panel_hgt() / 2 as ::core::ffi::c_int) / crate::borg_cave_util::borg_panel_hgt()
            * crate::borg_cave_util::borg_panel_hgt();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_HGT
            - ((*Term).hgt
                - row_top_map[(*Term).sidebar_mode as usize]
                - row_bottom_map[(*Term).sidebar_mode as usize])
                / tile_height as ::core::ffi::c_int
        {
            j = DUNGEON_HGT
                - ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.mon.morgoth_panel.y = j;
        j = ((*kill).pos.x - crate::borg_cave_util::borg_panel_wid() / 2 as ::core::ffi::c_int) / crate::borg_cave_util::borg_panel_wid()
            * crate::borg_cave_util::borg_panel_wid();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_WID
            - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            j = DUNGEON_WID
                - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                    / tile_width as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.mon.morgoth_panel.x = j;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat =
            FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat
        as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
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
            RF_PASS_WALL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).feat =
            FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_delete_kill(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    borg_note(format(
        b"# Forgetting a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
    ));
    if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
        && borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.y
        && borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.x
    {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).kill = 0 as uint8_t;
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.time.last_kill_mult = crate::bridge::st().borg.time.now;
    }
    memset(
        kill as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_kill>() as size_t,
    );
    borg_kills_cnt -= 1;
    borg_danger_wipe = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_sleep_kill(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    borg_note(format(
        b"# Noting sleep on a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
    ));
    (*kill).awake = false_0 != 0;
    crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    borg_danger_wipe = true_0 != 0;
}
unsafe extern "C" fn borg_follow_kill_aux(
    mut i: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut d: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    d = distance(crate::bridge::st().borg.c, loc(x, y));
    if d > (*z_info).max_sight as ::core::ffi::c_int {
        return false_0 != 0;
    }
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
        return false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0 {
        if (*ag).info as ::core::ffi::c_int & (BORG_LIGHT | BORG_GLOW) != 0 {
            if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0
                || crate::bridge::st().borg.see_inv as ::core::ffi::c_int != 0
            {
                return true_0 != 0;
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
                RF_INVISIBLE as ::core::ffi::c_int,
            ) {
                return true_0 != 0;
            }
        }
        if d <= crate::bridge::st().borg.trait_[(BI_INFRA as ::core::ffi::c_int) as usize] {
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
                RF_COLD_BLOOD as ::core::ffi::c_int,
            ) {
                return true_0 != 0;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
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
            RF_EMPTY_MIND as ::core::ffi::c_int,
        ) {
            return false_0 != 0;
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
            RF_WEIRD_MIND as ::core::ffi::c_int,
        ) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_follow_kill(mut i: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut b_dx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dy: ::core::ffi::c_int = 0;
    let mut b_dy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    ox = (*kill).pos.x;
    oy = (*kill).pos.y;
    if !borg_follow_kill_aux(i, oy, ox) {
        return;
    }
    borg_note(format(
        b"# There was a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        oy,
        ox,
    ));
    if !crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), oy, ox) {
        borg_delete_kill(i);
        return;
    }
    if (Rand_div(100 as uint32_t) as int32_t + 1 as int32_t) < 1 as int32_t {
        borg_delete_kill(i);
        return;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || (*kill).awake as ::core::ffi::c_int == false_0
    {
        if (*kill).pos.y == crate::bridge::st().borg.c.y && (*kill).pos.x == crate::bridge::st().borg.c.x {
            borg_delete_kill(i);
        }
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        dx = ddx_ddd[j as usize] as ::core::ffi::c_int;
        dy = ddy_ddd[j as usize] as ::core::ffi::c_int;
        x = ox + dx;
        y = oy + dy;
        if square_in_bounds_fully(cave, loc(x, y)) {
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
                if !((*ag).kill != 0) {
                    if !borg_follow_kill_aux(i, y, x) {
                        b_dx += dx;
                        b_dy += dy;
                    }
                }
            }
        }
        j += 1;
    }
    if b_dx < -(1 as ::core::ffi::c_int) {
        b_dx = -(1 as ::core::ffi::c_int);
    } else if b_dx > 1 as ::core::ffi::c_int {
        b_dx = 1 as ::core::ffi::c_int;
    }
    if b_dy < -(1 as ::core::ffi::c_int) {
        b_dy = -(1 as ::core::ffi::c_int);
    } else if b_dy > 1 as ::core::ffi::c_int {
        b_dy = 1 as ::core::ffi::c_int;
    }
    x = ox + b_dx;
    y = oy + b_dy;
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
        borg_delete_kill(i);
        return;
    }
    if (*ag).kill as ::core::ffi::c_int != i {
        borg_delete_kill(i);
        return;
    }
    if (*kill).pos.y == oy && (*kill).pos.x == ox {
        borg_delete_kill(i);
        return;
    }
    (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).kill = 0 as uint8_t;
    (*kill).ox = ox as uint8_t;
    (*kill).oy = oy as uint8_t;
    (*kill).pos.x = ox + b_dx;
    (*kill).pos.y = oy + b_dy;
    (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).kill = i as uint8_t;
    borg_note(format(
        b"# Following a monster '%s' to (%d,%d) from (%d,%d)\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
        oy,
        ox,
    ));
    borg_danger_wipe = true_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
        && (borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.y
            && borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.x)
        || crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE
            && crate::bridge::st().borg.munchkin_mode as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
}
unsafe extern "C" fn borg_new_kill(
    mut r_idx: ::core::ffi::c_uint,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut m_ptr: *mut monster = ::core::ptr::null_mut::<monster>();
    i = 1 as ::core::ffi::c_int;
    while n < 0 as ::core::ffi::c_int && i < borg_kills_nxt as ::core::ffi::c_int {
        if (*borg_kills.offset(i as isize)).r_idx == 0 {
            n = i;
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int
        && (borg_kills_nxt as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
    {
        let fresh0 = borg_kills_nxt;
        borg_kills_nxt = borg_kills_nxt + 1;
        n = fresh0 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        borg_note(b"# Too many monsters\0" as *const u8 as *const ::core::ffi::c_char);
        n = (Rand_div((borg_kills_nxt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t)
            as int32_t
            + 1 as int32_t
            + 1 as int32_t) as ::core::ffi::c_int;
        borg_delete_kill(n);
    }
    m_ptr = square_monster(cave, loc(x, y));
    if m_ptr.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    borg_kills_cnt += 1;
    kill = borg_kills.offset(n as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    (*kill).r_idx = r_idx as uint16_t;
    (*kill).pos.x = x;
    (*kill).ox = (*kill).pos.x as uint8_t;
    (*kill).pos.y = y;
    (*kill).oy = (*kill).pos.y as uint8_t;
    (*kill).m_idx = (*m_ptr).midx as int16_t;
    (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y)).kill = n as uint8_t;
    (*kill).when = crate::bridge::st().borg.time.now;
    if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
        crate::bridge::st().borg.time.morgoth = crate::bridge::st().borg.time.now;
    }
    borg_update_kill_new(n);
    borg_update_kill_old(n);
    borg_note(format(
        b"# Creating a monster '%s' at (%d,%d), HP: %d, Index: %d\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
        (*kill).power as ::core::ffi::c_int,
        (*kill).r_idx as ::core::ffi::c_int,
    ));
    borg_danger_wipe = true_0 != 0;
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.need_see_invis > 45 as borg_time {
        let mut y0: ::core::ffi::c_int = 0;
        let mut x0: ::core::ffi::c_int = 0;
        let mut y1: ::core::ffi::c_int = 0;
        let mut x1: ::core::ffi::c_int = 0;
        let mut y2: ::core::ffi::c_int = 0;
        let mut x2: ::core::ffi::c_int = 0;
        y0 = crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int;
        x0 = crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int;
        y1 = if y0 > 0 as ::core::ffi::c_int {
            y0 - 1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        x1 = if x0 > 0 as ::core::ffi::c_int {
            x0 - 1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y2 = if y0 < 5 as ::core::ffi::c_int {
            y0 + 1 as ::core::ffi::c_int
        } else {
            5 as ::core::ffi::c_int
        };
        x2 = if x0 < 17 as ::core::ffi::c_int {
            x0 + 1 as ::core::ffi::c_int
        } else {
            17 as ::core::ffi::c_int
        };
        borg_fear_region[y0 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y0 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y0 as usize][x2 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x2 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x2 as usize] = 0 as uint16_t;
        borg_note(format(
            b"# Removing Regional Fear (%d,%d) because of a LOS %s\0" as *const u8
                as *const ::core::ffi::c_char,
            y,
            x,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        ));
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0
        && borg_los((*kill).pos.y, (*kill).pos.x, crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*ag).feat = FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_nasties_num as ::core::ffi::c_int {
        if (*r_info.offset((*kill).r_idx as isize)).d_char == borg_nasties[i as usize] as wchar_t {
            borg_nasties_count[i as usize] = borg_nasties_count[i as usize].wrapping_add(1);
        }
        i += 1;
    }
    return n;
}
unsafe extern "C" fn borg_guess_race(
    mut a: uint8_t,
    mut c: wchar_t,
    mut multi: bool,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut m_ptr: *mut monster = ::core::ptr::null_mut::<monster>();
    m_ptr = square_monster(cave, loc(x, y));
    if m_ptr.is_null() {
        return 0 as ::core::ffi::c_uint;
    }
    return (*(*m_ptr).race).ridx;
}
#[no_mangle]
pub unsafe extern "C" fn observe_kill_diff(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    r_idx = borg_guess_race(a, c, false_0 != 0, y, x);
    if r_idx == 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    i = borg_new_kill(r_idx, y, x);
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    (*kill).when = crate::bridge::st().borg.time.now;
    if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
        crate::bridge::st().borg.time.morgoth = crate::bridge::st().borg.time.now;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn observe_kill_move(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
    mut flag: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut flicker: bool = false_0 != 0;
    let mut current_block_35: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(*kill).seen {
                ox = (*kill).pos.x;
                oy = (*kill).pos.y;
                z = borg_distance(oy, ox, y, x);
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
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
                    d = d * 2 as ::core::ffi::c_int;
                }
                if !(z > d) {
                    if !((*kill).m_idx as ::core::ffi::c_int
                        != (*square_monster(cave, loc(x, y))).midx)
                    {
                        if !(!flag
                            && z > (*kill).moves as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                        {
                            if !(crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                                == 0
                                && c != (*r_ptr).d_char)
                            {
                                if a as ::core::ffi::c_int != (*r_ptr).d_attr as ::core::ffi::c_int
                                    || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
                                        != 0
                                {
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
                                        RF_ATTR_MULTI as ::core::ffi::c_int,
                                    ) && !flag_has(
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
                                        RF_ATTR_CLEAR as ::core::ffi::c_int,
                                    ) {
                                        if !flag {
                                            current_block_35 = 14916268686031723178;
                                        } else if (*kill).known {
                                            current_block_35 = 14916268686031723178;
                                        } else {
                                            r_idx = borg_guess_race(a, c, true_0 != 0, y, x);
                                            if r_idx == 0 {
                                                current_block_35 = 14916268686031723178;
                                            } else {
                                                borg_note(format(
                                                    b"# Flickering monster '%s' at (%d,%d)\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    (*r_info.offset(r_idx as isize)).name,
                                                    y,
                                                    x,
                                                ));
                                                borg_note(format(
                                                    b"# Converting a monster '%s' at (%d,%d)\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    borg_race_name(
                                                        (*kill).r_idx as ::core::ffi::c_int,
                                                    ),
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                ));
                                                (*kill).r_idx = r_idx as uint16_t;
                                                flicker = true_0 != 0;
                                                borg_danger_wipe = true_0 != 0;
                                                crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
                                                current_block_35 = 10043043949733653460;
                                            }
                                        }
                                    } else {
                                        current_block_35 = 10043043949733653460;
                                    }
                                } else {
                                    current_block_35 = 10043043949733653460;
                                }
                                match current_block_35 {
                                    14916268686031723178 => {}
                                    _ => {
                                        if z != 0 {
                                            (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y))
                                            .kill = 0 as uint8_t;
                                            (*kill).ox = (*kill).pos.x as uint8_t;
                                            (*kill).oy = (*kill).pos.y as uint8_t;
                                            (*kill).pos.x = x;
                                            (*kill).pos.y = y;
                                            (*crate::bridge::grid((*kill).pos.x, (*kill).pos.y))
                                            .kill = i as uint8_t;
                                            borg_note(
                                                format(
                                                    b"# Tracking a monster '%s' at (%d,%d) from (%d,%d)\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                    oy,
                                                    ox,
                                                ),
                                            );
                                            borg_danger_wipe = true_0 != 0;
                                            if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize]
                                                == 0
                                                && crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int
                                                    == GOAL_KILL
                                                && (borg_flow_y[0 as ::core::ffi::c_int as usize]
                                                    as ::core::ffi::c_int
                                                    == (*kill).pos.y
                                                    && borg_flow_x[0 as ::core::ffi::c_int as usize]
                                                        as ::core::ffi::c_int
                                                        == (*kill).pos.x)
                                                || crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int
                                                    == GOAL_TAKE
                                                    && crate::bridge::st().borg.munchkin_mode as ::core::ffi::c_int != 0
                                            {
                                                crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
                                            }
                                        }
                                        (*kill).when = crate::bridge::st().borg.time.now;
                                        if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
                                            crate::bridge::st().borg.time.morgoth = crate::bridge::st().borg.time.now;
                                        }
                                        if flicker {
                                            borg_update_kill_new(i);
                                        }
                                        borg_update_kill_old(i);
                                        (*kill).seen = true_0 != 0;
                                        return true_0 != 0;
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
    return false_0 != 0;
}
unsafe extern "C" fn borg_guess_race_name(
    mut who: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_int = 0;
    let mut b_s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut partial: [::core::ffi::c_char; 160] = [0; 160];
    let mut len: ::core::ffi::c_int = strlen(who) as ::core::ffi::c_int;
    m = 0 as ::core::ffi::c_int;
    n = borg_unique_size;
    while m < n - 1 as ::core::ffi::c_int {
        i = (m + n) / 2 as ::core::ffi::c_int;
        if strcmp(*borg_unique_text.offset(i as isize), who) <= 0 as ::core::ffi::c_int {
            m = i;
        } else {
            n = i;
        }
    }
    if strcmp(who, *borg_unique_text.offset(m as isize)) == 0 {
        return *borg_unique_what.offset(m as isize);
    }
    if suffix(
        who,
        b" (offscreen)\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        my_strcpy(
            &raw mut partial as *mut ::core::ffi::c_char,
            who,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>() as size_t,
        );
        partial[(len - 12 as ::core::ffi::c_int) as usize] = '\0' as i32 as ::core::ffi::c_char;
        who = &raw mut partial as *mut ::core::ffi::c_char;
        borg_note(format(
            b"# Handling offscreen monster (%s)\0" as *const u8 as *const ::core::ffi::c_char,
            who,
        ));
    }
    if !prefix(who, b"The \0" as *const u8 as *const ::core::ffi::c_char)
        && !prefix(who, b"the \0" as *const u8 as *const ::core::ffi::c_char)
    {
        borg_note(format(
            b"# Assuming player ghost (%s) (a)\0" as *const u8 as *const ::core::ffi::c_char,
            who,
        ));
        return ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    }
    who = who.offset(4 as ::core::ffi::c_int as isize);
    m = 0 as ::core::ffi::c_int;
    n = borg_normal_size;
    while m < n - 1 as ::core::ffi::c_int {
        i = (m + n) / 2 as ::core::ffi::c_int;
        if strcmp(*borg_normal_text.offset(i as isize), who) <= 0 as ::core::ffi::c_int {
            m = i;
        } else {
            n = i;
        }
    }
    k = m;
    while k >= 0 as ::core::ffi::c_int {
        if strcmp(who, *borg_normal_text.offset(k as isize)) != 0 {
            break;
        }
        i = *borg_normal_what.offset(k as isize) as ::core::ffi::c_int;
        r_ptr = r_info.offset(i as isize) as *mut monster_race;
        s = 1000 as ::core::ffi::c_int;
        s = s
            - (if (*r_ptr).level
                - crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < 0 as ::core::ffi::c_int
            {
                -((*r_ptr).level
                    - crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize])
            } else {
                (*r_ptr).level
                    - crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            });
        if !(b_i != 0 && s < b_s) {
            b_i = i;
            b_s = s;
        }
        k -= 1;
    }
    if b_i != 0 {
        return b_i as ::core::ffi::c_uint;
    }
    borg_note(format(
        b"# Assuming player ghost (%s)(b)\0" as *const u8 as *const ::core::ffi::c_char,
        who,
    ));
    return ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn borg_locate_kill(
    mut who: *mut ::core::ffi::c_char,
    mut c: loc,
    mut r: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut b_i: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = 0;
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut k_ptr: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if my_strnicmp(
        who,
        b"Something\0" as *const u8 as *const ::core::ffi::c_char,
        9 as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
        || my_strnicmp(
            who,
            b"It\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
    {
        borg_note(b"# Invisible monster nearby.\0" as *const u8 as *const ::core::ffi::c_char);
        if crate::bridge::st().borg.time.now - crate::bridge::st().borg.need_see_invis > 50 as borg_time {
            crate::bridge::st().borg.need_see_invis = crate::bridge::st().borg.time.now;
        }
        return 0 as ::core::ffi::c_int;
    }
    if suffix(
        who,
        b" (offscreen)\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_note(b"# Offscreen monster nearby\0" as *const u8 as *const ::core::ffi::c_char);
        crate::bridge::st().borg.need_shift_panel = true_0 != 0;
        return 0 as ::core::ffi::c_int;
    }
    r_idx = borg_guess_race_name(who);
    r_ptr = r_info.offset(r_idx as isize) as *mut monster_race;
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        borg_note(format(
            b"# There is a monster '%s' within %d grids of %d,%d\0" as *const u8
                as *const ::core::ffi::c_char,
            borg_race_name(r_idx as ::core::ffi::c_int),
            r,
            c.y,
            c.x,
        ));
    }
    if (*borg_race_count.offset(r_idx as isize) as ::core::ffi::c_int) < SHRT_MAX {
        let ref mut fresh3 = *borg_race_count.offset(r_idx as isize);
        *fresh3 += 1;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_CHAR_CLEAR as ::core::ffi::c_int,
    ) {
        borg_note(b"# Bizarre monster nearby\0" as *const u8 as *const ::core::ffi::c_char);
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    let mut current_block_29: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            k_ptr = k_info.offset((*take).kidx as isize) as *mut object_kind;
            if !((*k_ptr).d_char != (*r_ptr).d_char) {
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
                    RF_ATTR_MULTI as ::core::ffi::c_int,
                ) && !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_ATTR_CLEAR as ::core::ffi::c_int,
                ) {
                    if (*k_ptr).flavor.is_null() {
                        if (*k_ptr).d_attr as ::core::ffi::c_int
                            != (*r_ptr).d_attr as ::core::ffi::c_int
                        {
                            current_block_29 = 26972500619410423;
                        } else {
                            current_block_29 = 3437258052017859086;
                        }
                    } else {
                        current_block_29 = 3437258052017859086;
                    }
                } else {
                    current_block_29 = 3437258052017859086;
                }
                match current_block_29 {
                    26972500619410423 => {}
                    _ => {
                        d = distance(
                            loc(
                                (*take).x as ::core::ffi::c_int,
                                (*take).y as ::core::ffi::c_int,
                            ),
                            c,
                        );
                        if !(d > r) {
                            if !(d > b_d) {
                                b_i = i;
                                b_d = d;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i >= 0 as ::core::ffi::c_int {
        take = borg_takes.offset(b_i as isize) as *mut borg_take;
        borg_note(format(
            b"# Converting an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            (*k_info.offset((*take).kidx as isize)).name,
            (*take).y as ::core::ffi::c_int,
            (*take).x as ::core::ffi::c_int,
        ));
        c.x = (*take).x as ::core::ffi::c_int;
        c.y = (*take).y as ::core::ffi::c_int;
        borg_delete_take(b_i);
        b_i = borg_new_kill(r_idx, c.y, c.x);
        if b_i < 0 as ::core::ffi::c_int {
            return b_i;
        }
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        (*kill).when = crate::bridge::st().borg.time.now;
        if (*kill).r_idx as ::core::ffi::c_uint == crate::bridge::st().borg.id.morgoth {
            crate::bridge::st().borg.time.morgoth = crate::bridge::st().borg.time.now;
        }
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        return b_i;
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    let mut current_block_54: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint == r_idx) {
                let mut m_ptr: *mut monster =
                    (*cave).monsters.offset((*kill).m_idx as isize) as *mut monster;
                if !((*m_ptr).grid.x != (*kill).pos.x || (*m_ptr).grid.y != (*kill).pos.y) {
                    if !((*r_info.offset((*kill).r_idx as isize)).d_char != (*r_ptr).d_char) {
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
                            RF_ATTR_MULTI as ::core::ffi::c_int,
                        ) && !flag_has(
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
                            RF_ATTR_CLEAR as ::core::ffi::c_int,
                        ) {
                            if (*r_info.offset((*kill).r_idx as isize)).d_attr as ::core::ffi::c_int
                                != (*r_ptr).d_attr as ::core::ffi::c_int
                            {
                                current_block_54 = 1345366029464561491;
                            } else {
                                current_block_54 = 9353995356876505083;
                            }
                        } else {
                            current_block_54 = 9353995356876505083;
                        }
                        match current_block_54 {
                            1345366029464561491 => {}
                            _ => {
                                d = distance((*kill).pos, c);
                                if !(d > r) {
                                    if !(d > b_d) {
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
    if b_i >= 0 as ::core::ffi::c_int {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        borg_note(format(
            b"# Converting a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).pos.y,
            (*kill).pos.x,
        ));
        (*kill).r_idx = r_idx as uint16_t;
        borg_update_kill_new(b_i);
        borg_update_kill_old(b_i);
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        borg_danger_wipe = true_0 != 0;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return b_i;
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint != r_idx) {
                d = distance((*kill).pos, c);
                if !(d > r + 3 as ::core::ffi::c_int) {
                    if borg_projectable((*kill).pos.y, (*kill).pos.x, c.y, c.x) {
                        if !(d > b_d) {
                            b_i = i;
                            b_d = d;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        b_i = -(1 as ::core::ffi::c_int);
        b_d = 999 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
                if !((*kill).r_idx as ::core::ffi::c_uint != r_idx) {
                    d = distance((*kill).pos, c);
                    if !(d > r + 20 as ::core::ffi::c_int) {
                        if !(d > b_d) {
                            b_i = i;
                            b_d = d;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if b_i >= 0 as ::core::ffi::c_int {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(format(
                b"# Matched a monster '%s' at (%d,%d) for the parsed msg.\0" as *const u8
                    as *const ::core::ffi::c_char,
                borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                (*kill).pos.y,
                (*kill).pos.x,
            ));
        }
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        return b_i;
    }
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        borg_note(format(
            b"# Unable to locate monster '%s' near (%d,%d), which generated the msg (%s).\0"
                as *const u8 as *const ::core::ffi::c_char,
            borg_race_name(r_idx as ::core::ffi::c_int),
            c.y,
            c.x,
            who,
        ));
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_count_death(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx != 0 {
        if (*borg_race_death.offset((*kill).r_idx as isize) as ::core::ffi::c_int) < SHRT_MAX {
            let ref mut fresh4 = *borg_race_death.offset((*kill).r_idx as isize);
            *fresh4 += 1;
        }
        if flag_has(
            &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
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
            crate::bridge::st().borg.mon.unique = 0 as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut borg_in_hall: bool = false_0 != 0;
    let mut hall_y: ::core::ffi::c_int = 0;
    let mut hall_x: ::core::ffi::c_int = 0;
    let mut hall_walls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skip_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
        && crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] == 0
    {
        return false_0 != 0;
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < (if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                != 0
            {
                35 as ::core::ffi::c_int
            } else {
                25 as ::core::ffi::c_int
            })
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.morgoth_position {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    hall_x = -(1 as ::core::ffi::c_int);
    while hall_x <= 1 as ::core::ffi::c_int {
        hall_y = -(1 as ::core::ffi::c_int);
        while hall_y <= 1 as ::core::ffi::c_int {
            x = hall_x + crate::bridge::st().borg.c.x;
            y = hall_y + crate::bridge::st().borg.c.y;
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if (*ag).glyph as ::core::ffi::c_int != 0
                || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                    && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
            {
                hall_walls += 1;
            }
            if hall_walls >= 5 as ::core::ffi::c_int {
                borg_in_hall = true_0 != 0;
            }
            hall_y += 1;
        }
        hall_x += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(crate::bridge::st().borg.c, loc(x, y));
        if !(b_j < j && b_j != -(1 as ::core::ffi::c_int)) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    let mut current_block_60: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        if !((*kill).r_idx == 0) {
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
            if !(d == 1 as ::core::ffi::c_int
                && (crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                    != 0
                    || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
                        != 0))
            {
                if !(crate::bridge::st().borg.goal.ignoring as ::core::ffi::c_int != 0
                    && crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                        == 0
                    && flag_has(
                        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
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
                    if !(crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 10 as ::core::ffi::c_int
                        && flag_has(
                            &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
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
                        ) as ::core::ffi::c_int
                            != 0)
                    {
                        if !crate::bridge::st().borg.mon.scary {
                            if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                < 10 as ::core::ffi::c_int
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
                                    != 0)
                            {
                                if !(flag_has(
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
                                    RF_UNIQUE as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int
                                    != 0
                                    && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                                        == 0 as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                        < 5 as ::core::ffi::c_int)
                                {
                                    x = (*kill).pos.x;
                                    y = (*kill).pos.y;
                                    ag = crate::bridge::grid(x, y)
                                        as *mut borg_grid;
                                    if !(viewable as ::core::ffi::c_int != 0
                                        && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                                    {
                                        p = borg_danger(
                                            y,
                                            x,
                                            1 as ::core::ffi::c_int,
                                            true_0 != 0,
                                            false_0 != 0,
                                        );
                                        if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                            > 25 as ::core::ffi::c_int
                                            && !flag_has(
                                                &raw mut (*r_info.offset((*kill).r_idx as isize))
                                                    .flags
                                                    as *mut bitflag,
                                                (RF_MAX as ::core::ffi::c_int as size_t)
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
                                                RF_UNIQUE as ::core::ffi::c_int,
                                            )
                                            && p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                / 2 as ::core::ffi::c_int)
                                        {
                                            if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                <= 15 as ::core::ffi::c_int
                                                && p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                    / 3 as ::core::ffi::c_int)
                                            {
                                                if b_stair != -(1 as ::core::ffi::c_int)
                                                    && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] < 10 as ::core::ffi::c_int
                                                {
                                                    j = borg_distance(
                                                        *track_less.y.offset(b_stair as isize),
                                                        *track_less.x.offset(b_stair as isize),
                                                        y,
                                                        x,
                                                    );
                                                    if b_j
                                                        <= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] * 5 as ::core::ffi::c_int
                                                            + 9 as ::core::ffi::c_int
                                                        && j >= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] * 5 as ::core::ffi::c_int
                                                            + 9 as ::core::ffi::c_int
                                                    {
                                                        current_block_60 = 4775909272756257391;
                                                    } else {
                                                        current_block_60 = 16738040538446813684;
                                                    }
                                                } else {
                                                    current_block_60 = 16738040538446813684;
                                                }
                                                match current_block_60 {
                                                    4775909272756257391 => {}
                                                    _ => {
                                                        if borg_in_hall as ::core::ffi::c_int != 0
                                                            && flag_has(
                                                                &raw mut (*r_info
                                                                    .offset((*kill).r_idx as isize))
                                                                .flags
                                                                    as *mut bitflag,
                                                                (RF_MAX as ::core::ffi::c_int
                                                                    as size_t)
                                                                    .wrapping_add(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    )
                                                                    .wrapping_sub(1 as size_t)
                                                                    .wrapping_div(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    ),
                                                                RF_GROUP_AI as ::core::ffi::c_int,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            hall_x = -(1 as ::core::ffi::c_int);
                                                            while hall_x <= 1 as ::core::ffi::c_int
                                                            {
                                                                hall_y = -(1 as ::core::ffi::c_int);
                                                                while hall_y
                                                                    <= 1 as ::core::ffi::c_int
                                                                {
                                                                    if square_in_bounds_fully(
                                                                        cave,
                                                                        loc(hall_x + x, hall_y + y),
                                                                    ) {
                                                                        ag = crate::bridge::grid((hall_x + x), (hall_y + y))
                                                                            as *mut borg_grid;
                                                                        if (*ag).glyph as ::core::ffi::c_int != 0
                                                                            || (*ag).feat as ::core::ffi::c_int
                                                                                >= FEAT_MAGMA as ::core::ffi::c_int
                                                                                && (*ag).feat as ::core::ffi::c_int
                                                                                    <= FEAT_PERM as ::core::ffi::c_int
                                                                        {
                                                                            hall_walls += 1;
                                                                        }
                                                                        if hall_walls < 4 as ::core::ffi::c_int {
                                                                            skip_monster = true_0 != 0;
                                                                        }
                                                                    }
                                                                    hall_y += 1;
                                                                }
                                                                hall_x += 1;
                                                            }
                                                        }
                                                        if d == 2 as ::core::ffi::c_int
                                                            && (*kill).ranged_attack == 0
                                                            && !flag_has(
                                                                &raw mut (*r_info
                                                                    .offset((*kill).r_idx as isize))
                                                                .flags
                                                                    as *mut bitflag,
                                                                (RF_MAX as ::core::ffi::c_int
                                                                    as size_t)
                                                                    .wrapping_add(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    )
                                                                    .wrapping_sub(1 as size_t)
                                                                    .wrapping_div(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    ),
                                                                RF_NEVER_MOVE as ::core::ffi::c_int,
                                                            )
                                                        {
                                                            skip_monster = true_0 != 0;
                                                        }
                                                        if !skip_monster {
                                                            if !borg_flow_far_from_stairs(
                                                                x, y, b_stair,
                                                            ) {
                                                                borg_temp_x[borg_temp_n as usize] =
                                                                    x as uint8_t;
                                                                borg_temp_y[borg_temp_n as usize] =
                                                                    y as uint8_t;
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
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"kill\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_KILL,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_KILL) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_has_distance_attack() -> bool {
    if borg_attack_aux_spell_bolt(
        MAGIC_MISSILE,
        0 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        BORG_ATTACK_MISSILE as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        false_0 != 0,
    ) > 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if borg_attack_aux_spell_bolt(
        NETHER_BOLT,
        0 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        BORG_ATTACK_NETHER as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        false_0 != 0,
    ) > 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if borg_attack_aux_launch() > 0 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_aim(mut viewable: bool) -> bool {
    let mut o_y: ::core::ffi::c_int = 0;
    let mut o_x: ::core::ffi::c_int = 0;
    let mut s_c_y: ::core::ffi::c_int = crate::bridge::st().borg.c.y;
    let mut s_c_x: ::core::ffi::c_int = crate::bridge::st().borg.c.x;
    let mut i: ::core::ffi::c_int = 0;
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_has_distance_attack() {
        return false_0 != 0;
    }
    o_x = -(2 as ::core::ffi::c_int);
    while o_x <= 2 as ::core::ffi::c_int {
        o_y = -(2 as ::core::ffi::c_int);
        while o_y <= 2 as ::core::ffi::c_int {
            if !(o_x == 0 as ::core::ffi::c_int && o_y == 0 as ::core::ffi::c_int) {
                crate::bridge::st().borg.c.x = s_c_x + o_x;
                crate::bridge::st().borg.c.y = s_c_y + o_y;
                if !(crate::bridge::st().borg.c.x > AUTO_MAX_X - 2 as ::core::ffi::c_int
                    || crate::bridge::st().borg.c.x < 2 as ::core::ffi::c_int
                    || crate::bridge::st().borg.c.y > AUTO_MAX_Y - 2 as ::core::ffi::c_int
                    || crate::bridge::st().borg.c.y < 2 as ::core::ffi::c_int)
                {
                    i = 0 as ::core::ffi::c_int;
                    while i < borg_kills_nxt as ::core::ffi::c_int {
                        if distance(crate::bridge::st().borg.c, (*borg_kills.offset(i as isize)).pos)
                            == 1 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    if !(i != borg_kills_nxt as ::core::ffi::c_int) {
                        if borg_has_distance_attack() {
                            borg_flow_clear();
                            borg_flow_enqueue_grid(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x);
                            crate::bridge::st().borg.c.x = s_c_x;
                            crate::bridge::st().borg.c.y = s_c_y;
                            borg_flow_spread(
                                5 as ::core::ffi::c_int,
                                true_0 != 0,
                                !viewable,
                                false_0 != 0,
                                -(1 as ::core::ffi::c_int),
                                false_0 != 0,
                            );
                            if !borg_flow_commit(
                                b"targetable position\0" as *const u8 as *const ::core::ffi::c_char,
                                GOAL_KILL,
                            ) {
                                return false_0 != 0;
                            }
                            if !borg_flow_old(GOAL_KILL) {
                                return false_0 != 0;
                            }
                            return true_0 != 0;
                        }
                    }
                }
            }
            o_y += 1;
        }
        o_x += 1;
    }
    crate::bridge::st().borg.c.x = s_c_x;
    crate::bridge::st().borg.c.y = s_c_y;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_corridor() -> bool {
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_distance: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_n: bool = false_0 != 0;
    let mut b_s: bool = false_0 != 0;
    let mut b_e: bool = false_0 != 0;
    let mut b_w: bool = false_0 != 0;
    let mut n_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut ny: [::core::ffi::c_int; 25] = [
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut nx: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut s_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut sy: [::core::ffi::c_int; 25] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    ];
    let mut sx: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut e_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut ey: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut ex: [::core::ffi::c_int; 25] = [
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    ];
    let mut w_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut wy: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut wx: [::core::ffi::c_int; 25] = [
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
    ];
    let mut wall_north: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_south: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_east: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_west: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    crate::bridge::st().borg.status.digging = false_0 != 0;
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.near.summoner_idx as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.morgoth_position {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.status.anti_summon {
        return false_0 != 0;
    }
    kill = borg_kills.offset(crate::bridge::st().borg.near.summoner_idx as isize) as *mut borg_kill;
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_KILL_WALL as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if !(*kill).awake {
        return false_0 != 0;
    }
    if !borg_spell_okay(TURN_STONE_TO_MUD)
        && !borg_spell_okay(SHATTER_STONE)
        && !borg_equips_ring(sv_ring_digging)
        && !borg_equips_item(crate::bridge::st().act.stone_to_mud, true_0 != 0)
    {
        return false_0 != 0;
    }
    if !borg_los((*kill).pos.y, (*kill).pos.x, crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x) {
        q_x = crate::bridge::st().core.w_x / crate::borg_cave_util::borg_panel_wid();
        q_y = crate::bridge::st().core.w_y / crate::borg_cave_util::borg_panel_hgt();
        if *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == true_0
            && *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
        {
            borg_flow_clear();
            crate::bridge::st().borg.status.digging = true_0 != 0;
            borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
            borg_flow_spread(
                10 as ::core::ffi::c_int,
                true_0 != 0,
                false_0 != 0,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
                false_0 != 0,
            );
            if !borg_flow_commit(
                b"Monster Path\0" as *const u8 as *const ::core::ffi::c_char,
                GOAL_KILL,
            ) {
                return false_0 != 0;
            }
        } else {
            borg_flow_clear();
            crate::bridge::st().borg.status.digging = true_0 != 0;
            borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
            borg_flow_spread(
                10 as ::core::ffi::c_int,
                true_0 != 0,
                true_0 != 0,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
                false_0 != 0,
            );
            if !borg_flow_commit(
                b"Monster Path\0" as *const u8 as *const ::core::ffi::c_char,
                GOAL_KILL,
            ) {
                return false_0 != 0;
            }
        }
    }
    o_y = -(2 as ::core::ffi::c_int);
    while o_y < 1 as ::core::ffi::c_int {
        wall_north = 0 as ::core::ffi::c_int;
        o_x = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = crate::bridge::st().borg.c.y + o_y + ny[i as usize];
            m_x = crate::bridge::st().borg.c.x + o_x + nx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag = crate::bridge::grid(m_x, m_y) as *mut borg_grid;
                if n_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_north += 1;
                }
                if n_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_north += 1;
                }
            }
            i += 1;
        }
        if wall_north == 25 as ::core::ffi::c_int {
            if distance(
                crate::bridge::st().borg.c,
                loc(
                    crate::bridge::st().borg.c.x + o_x + nx[7 as ::core::ffi::c_int as usize],
                    crate::bridge::st().borg.c.y + o_y + ny[7 as ::core::ffi::c_int as usize],
                ),
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_n = true_0 != 0;
                b_distance = distance(
                    crate::bridge::st().borg.c,
                    loc(
                        crate::bridge::st().borg.c.x + o_x + nx[7 as ::core::ffi::c_int as usize],
                        crate::bridge::st().borg.c.y + o_y + ny[7 as ::core::ffi::c_int as usize],
                    ),
                );
            }
        }
        o_y += 1;
    }
    o_y = -(1 as ::core::ffi::c_int);
    while o_y < 2 as ::core::ffi::c_int {
        wall_south = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_0: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            o_x = 0 as ::core::ffi::c_int;
            m_y = crate::bridge::st().borg.c.y + o_y + sy[i as usize];
            m_x = crate::bridge::st().borg.c.x + o_x + sx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_0 = crate::bridge::grid(m_x, m_y) as *mut borg_grid;
                if s_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_0).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_0).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_south += 1;
                }
                if s_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_0).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_0).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_south += 1;
                }
            }
            i += 1;
        }
        if wall_south == 25 as ::core::ffi::c_int {
            if borg_distance(
                crate::bridge::st().borg.c.y,
                crate::bridge::st().borg.c.x,
                crate::bridge::st().borg.c.y + o_y + sy[17 as ::core::ffi::c_int as usize],
                crate::bridge::st().borg.c.x + o_x + sx[17 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_s = true_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    crate::bridge::st().borg.c.y + b_y + sy[17 as ::core::ffi::c_int as usize],
                    crate::bridge::st().borg.c.x + b_x + sx[17 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_y += 1;
    }
    o_x = -(1 as ::core::ffi::c_int);
    while o_x < 2 as ::core::ffi::c_int {
        wall_east = 0 as ::core::ffi::c_int;
        o_y = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_1: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = crate::bridge::st().borg.c.y + o_y + ey[i as usize];
            m_x = crate::bridge::st().borg.c.x + o_x + ex[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_1 = crate::bridge::grid(m_x, m_y) as *mut borg_grid;
                if e_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_1).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_1).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_east += 1;
                }
                if e_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_1).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_1).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_east += 1;
                }
            }
            i += 1;
        }
        if wall_east == 25 as ::core::ffi::c_int {
            if borg_distance(
                crate::bridge::st().borg.c.y,
                crate::bridge::st().borg.c.x,
                crate::bridge::st().borg.c.y + o_y + ey[13 as ::core::ffi::c_int as usize],
                crate::bridge::st().borg.c.x + o_x + ex[13 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_e = true_0 != 0;
                b_s = false_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    crate::bridge::st().borg.c.y + b_y + ey[13 as ::core::ffi::c_int as usize],
                    crate::bridge::st().borg.c.x + b_x + ex[13 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_x += 1;
    }
    o_x = -(2 as ::core::ffi::c_int);
    while o_x < 1 as ::core::ffi::c_int {
        wall_west = 0 as ::core::ffi::c_int;
        o_y = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_2: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = crate::bridge::st().borg.c.y + o_y + wy[i as usize];
            m_x = crate::bridge::st().borg.c.x + o_x + wx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_2 = crate::bridge::grid(m_x, m_y) as *mut borg_grid;
                if w_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_2).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_2).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_west += 1;
                }
                if w_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_2).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_2).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_west += 1;
                }
            }
            i += 1;
        }
        if wall_west == 25 as ::core::ffi::c_int {
            if borg_distance(
                crate::bridge::st().borg.c.y,
                crate::bridge::st().borg.c.x,
                crate::bridge::st().borg.c.y + o_y + wy[11 as ::core::ffi::c_int as usize],
                crate::bridge::st().borg.c.x + o_x + wx[11 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_w = true_0 != 0;
                b_e = false_0 != 0;
                b_s = false_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    crate::bridge::st().borg.c.y,
                    crate::bridge::st().borg.c.x,
                    crate::bridge::st().borg.c.y + o_y + wy[11 as ::core::ffi::c_int as usize],
                    crate::bridge::st().borg.c.x + o_x + wx[11 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_x += 1;
    }
    if b_n as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        crate::bridge::st().borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            crate::bridge::st().borg.c.y + b_y + ny[7 as ::core::ffi::c_int as usize],
            crate::bridge::st().borg.c.x + b_x + nx[7 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor north type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_s as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        crate::bridge::st().borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            crate::bridge::st().borg.c.y + b_y + sy[17 as ::core::ffi::c_int as usize],
            crate::bridge::st().borg.c.x + b_x + sx[17 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            6 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor south type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_e as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        crate::bridge::st().borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            crate::bridge::st().borg.c.y + b_y + ey[13 as ::core::ffi::c_int as usize],
            crate::bridge::st().borg.c.x + b_x + ex[13 as ::core::ffi::c_int as usize],
        );
        crate::bridge::st().borg.status.digging = true_0 != 0;
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor east type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_w as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        crate::bridge::st().borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            crate::bridge::st().borg.c.y + b_y + wy[11 as ::core::ffi::c_int as usize],
            crate::bridge::st().borg.c.x + b_x + wx[11 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor west type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_direct(mut twitchy: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_sight as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_can_dig(false_0 != 0, FEAT_GRANITE as ::core::ffi::c_int as uint8_t) {
        return false_0 != 0;
    }
    if !twitchy
        && (crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
            != 0
            || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
                != 0
            || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int)
    {
        return false_0 != 0;
    }
    if !twitchy
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level < 3000 as borg_time
        && (crate::bridge::st().borg.times_twitch as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_kills_cnt != 0 {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
                d = distance((*kill).pos, crate::bridge::st().borg.c);
                if !(d > b_d) {
                    b_i = i;
                    b_d = d;
                }
            }
            i += 1;
        }
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        borg_flow_clear();
        borg_flow_enqueue_grid(
            AUTO_MAX_Y / 2 as ::core::ffi::c_int,
            AUTO_MAX_X / 2 as ::core::ffi::c_int,
        );
        borg_flow_spread(
            150 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"center direct\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_i != 0 {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        borg_flow_clear();
        borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
        borg_flow_spread(
            15 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"kill direct\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_near_monster_type(mut dist: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut x9: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut breeder_count: ::core::ffi::c_int = crate::bridge::st().borg.mon.breeders as ::core::ffi::c_int;
    crate::bridge::st().borg.near.summoner = false_0 != 0;
    crate::bridge::st().borg.near.unique = 0 as int16_t;
    crate::bridge::st().borg.near.evil_unique = false_0 != 0;
    crate::bridge::st().borg.near.summoner_idx = -(1 as ::core::ffi::c_int) as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 5 as ::core::ffi::c_int
                    && (*r_ptr).ridx == crate::bridge::st().borg.id.squint_eyed_rogue
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 6 as ::core::ffi::c_int
                    && (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
                        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_PRIEST)
                    && (*r_ptr).ridx == crate::bridge::st().borg.id.squint_eyed_rogue
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 5 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.grip
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.fang
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.small_kobold)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 8 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.soldier
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.cutpurse
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.acolyte
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.apprentice
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.kobold
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.shrieker_mushroom_patch
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.maggot
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.filthy_street_urchin
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.battle_scarred_veteran
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.mean_looking_mercenary)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 15 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.bullroarer
                        || ((*r_ptr).ridx == crate::bridge::st().borg.id.giant_white_mouse
                            || (*r_ptr).ridx == crate::bridge::st().borg.id.white_worm_mass
                            || (*r_ptr).ridx == crate::bridge::st().borg.id.green_worm_mass)
                            && breeder_count
                                >= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize])
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 20 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.cave_spider
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.red_naga
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.giant_red_frog
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.radiation_eye
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.yellow_worm_mass
                            && breeder_count
                                >= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize])
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 45 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.gravity_hound
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.inertia_hound
                        || prefix_i(
                            (*(*r_ptr).base).name,
                            b"ancient dragon\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as ::core::ffi::c_int
                            != 0
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.beorn
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.dread)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_SRNTHR as ::core::ffi::c_int) as usize]
                    == 0
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.osse
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.dracolich
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.dracolisk)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_SRBLIND as ::core::ffi::c_int) as usize]
                    == 0
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.light_hound
                        && crate::bridge::st().borg.trait_[(BI_SRLITE as ::core::ffi::c_int) as usize]
                            == 0
                        || (*r_ptr).ridx == crate::bridge::st().borg.id.dark_hound
                            && crate::bridge::st().borg.trait_[(BI_SRDARK as ::core::ffi::c_int) as usize]
                                == 0)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_SRKAOS as ::core::ffi::c_int) as usize]
                    == 0
                    && crate::bridge::st().borg.trait_[(BI_SRCONF as ::core::ffi::c_int) as usize]
                        == 0
                    && flag_has(
                        &raw mut (*r_ptr).spell_flags as *mut bitflag,
                        (RSF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RSF_BR_CHAO as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_SRCONF as ::core::ffi::c_int) as usize]
                    == 0
                    && ((*r_ptr).ridx == crate::bridge::st().borg.id.pukelman || (*r_ptr).ridx == crate::bridge::st().borg.id.night_mare)
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
                if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] == 0
                    && (*r_ptr).ridx == crate::bridge::st().borg.id.drolem
                {
                    crate::bridge::st().borg.mon.scary = true_0 != 0;
                }
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
                if !(d > dist
                    && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        != 0)
                {
                    if flag_has(
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
                        crate::bridge::st().borg.mon.unique = (*kill).r_idx as ::core::ffi::c_uint;
                        if flag_has(
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
                            RF_QUESTOR as ::core::ffi::c_int,
                        ) {
                            crate::bridge::st().borg.near.unique = (crate::bridge::st().borg.near.unique as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                as int16_t;
                        }
                        crate::bridge::st().borg.near.unique += 1;
                        if flag_has(
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
                            RF_EVIL as ::core::ffi::c_int,
                        ) {
                            crate::bridge::st().borg.near.evil_unique = true_0 != 0;
                        }
                    }
                    if flag_has(
                        &raw mut (*r_ptr).spell_flags as *mut bitflag,
                        (RSF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RSF_S_KIN as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_DEMON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_MONSTER as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_MONSTERS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_ANIMAL as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_SPIDER as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HOUND as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HYDRA as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_AINU as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_DEMON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_UNDEAD as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_DRAGON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_DRAGON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_WRAITH as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_UNIQUE as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                    {
                        crate::bridge::st().borg.near.summoner = true_0 != 0;
                        if d < 8 as ::core::ffi::c_int {
                            crate::bridge::st().borg.near.summoner_idx = i as int16_t;
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_shoot_scoot_safe(
    mut emergency: ::core::ffi::c_int,
    mut turns: ::core::ffi::c_int,
    mut b_p: ::core::ffi::c_int,
) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut dis: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = dis / 2 as ::core::ffi::c_int;
    let mut adjacent_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 8 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 {
        return false_0 != 0;
    }
    if square_isvault(cave, crate::bridge::st().borg.c) {
        return false_0 != 0;
    }
    if borg_primarily_caster() {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= 45 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                < 15 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 45 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                < 5 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
    } else if crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
        < 5 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= 45 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        x = crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
        y = crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
        ag = crate::bridge::grid(x, y)
            as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if (*ag).kill as ::core::ffi::c_int != 0
            && (*kill).awake as ::core::ffi::c_int != 0
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
                RF_NEVER_MOVE as ::core::ffi::c_int,
            )
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
                RF_PASS_WALL as ::core::ffi::c_int,
            )
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
                RF_KILL_WALL as ::core::ffi::c_int,
            )
            && (*kill).power as ::core::ffi::c_int
                >= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        {
            if borg_spell_okay(MAGIC_MISSILE) {
                adjacent_monster = true_0 != 0;
            } else if borg_spell_okay(ORB_OF_DRAINING) {
                adjacent_monster = true_0 != 0;
            } else if borg_spell_okay(NETHER_BOLT) {
                adjacent_monster = true_0 != 0;
            } else if borg_danger_one_kill(
                (*kill).pos.y,
                (*kill).pos.x,
                1 as ::core::ffi::c_int,
                i,
                true_0 != 0,
                false_0 != 0,
            ) > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
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
                || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 5 as ::core::ffi::c_int
            {
                adjacent_monster = true_0 != 0;
            }
        }
        i += 1;
    }
    if adjacent_monster as ::core::ffi::c_int == false_0 {
        return false_0 != 0;
    }
    k = 0 as ::core::ffi::c_int;
    n = k;
    while k < 100 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 100 as ::core::ffi::c_int {
            loop {
                y = (crate::bridge::st().borg.c.y as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                x = (crate::bridge::st().borg.c.x as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                d = distance(crate::bridge::st().borg.c, loc(x, y));
                if d >= min && d <= dis {
                    break;
                }
            }
            if !(y <= 0 as ::core::ffi::c_int || y >= AUTO_MAX_Y - 2 as ::core::ffi::c_int) {
                if !(x <= 0 as ::core::ffi::c_int || x >= AUTO_MAX_X - 2 as ::core::ffi::c_int) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y, x) {
                            if !((*ag).kill != 0) {
                                break;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if i >= 100 as ::core::ffi::c_int {
            n += 1;
        } else {
            p = borg_danger(y, x, turns, true_0 != 0, false_0 != 0);
            if p > b_p {
                n += 1;
            } else {
                u = 0 as ::core::ffi::c_int;
                while u < 8 as ::core::ffi::c_int {
                    ag = crate::bridge::grid((x + *(&raw const ddx_ddd as *const int16_t).offset(u as isize)
                            as ::core::ffi::c_int), (y + *(&raw const ddy_ddd as *const int16_t).offset(u as isize)
                            as ::core::ffi::c_int)) as *mut borg_grid;
                    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                    if (*ag).kill as ::core::ffi::c_int != 0
                        && (*kill).awake as ::core::ffi::c_int != 0
                    {
                        n += 1;
                    }
                    u += 1;
                }
            }
        }
        k += 1;
    }
    if n > emergency {
        borg_note(format(
            b"# No Shoot'N'Scoot. scary squares: %d/100\0" as *const u8
                as *const ::core::ffi::c_char,
            n,
        ));
        return false_0 != 0;
    } else {
        borg_note(format(
            b"# Safe to Shoot'N'Scoot. scary squares: %d/100\0" as *const u8
                as *const ::core::ffi::c_char,
            n,
        ));
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_create_kill(
    mut who: *mut ::core::ffi::c_char,
    mut c: loc,
) -> ::core::ffi::c_int {
    return borg_new_kill(borg_guess_race_name(who), c.y, c.x);
}
unsafe extern "C" fn borg_init_monster_names() {
    let mut i: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut r_max: uint16_t =
        ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
    let mut what: [int16_t; 1024] = [0; 1024];
    let mut text: [*mut ::core::ffi::c_char; 1024] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 1024];
    size = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < r_max as ::core::ffi::c_int {
        let mut r_ptr: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr).name.is_null() {
            if strcmp(
                (*r_ptr).name,
                b"Morgoth, Lord of Darkness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.morgoth = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Sauron, the Sorcerer\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.sauron = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.tarrasque = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"squint-eyed rogue\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.squint_eyed_rogue = (*r_ptr).ridx;
            }
            if prefix(
                (*r_ptr).name,
                b"Grip\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                crate::bridge::st().borg.id.grip = (*r_ptr).ridx;
            }
            if prefix(
                (*r_ptr).name,
                b"Fang\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                crate::bridge::st().borg.id.fang = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"small kobold\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.small_kobold = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"soldier\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.soldier = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"cutpurse\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.cutpurse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"acolyte\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.acolyte = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"apprentice\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.apprentice = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"kobold\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.kobold = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"shrieker mushroom patch\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.shrieker_mushroom_patch = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Farmer Maggot\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.maggot = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"filthy street urchin\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.filthy_street_urchin = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"battle-scarred veteran\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.battle_scarred_veteran = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"mean-looking mercenary\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.mean_looking_mercenary = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Bullroarer the Hobbit\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.bullroarer = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"giant white mouse\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.giant_white_mouse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"white worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.white_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"green worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.green_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"cave spider\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.cave_spider = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"red naga\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.red_naga = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"giant red frog\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.giant_red_frog = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"radiation eye\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.radiation_eye = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"yellow worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.yellow_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"gravity hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.gravity_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"inertia hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.inertia_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dread\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.dread = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Beorn, the Shape-Changer\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.beorn = (*r_ptr).ridx;
            }
            if !my_stristr(
                (*r_ptr).name,
                b"Herald of Ulmo\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            {
                crate::bridge::st().borg.id.osse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dracolich\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.dracolich = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dracolisk\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.dracolisk = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"light hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.light_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dark hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.dark_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"night mare\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.night_mare = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"pukelman\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.pukelman = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"drolem\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                crate::bridge::st().borg.id.drolem = (*r_ptr).ridx;
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
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                text[size as usize] = borg_massage_special_chars((*r_ptr).name);
                what[size as usize] = i as int16_t;
                size += 1;
            }
        }
        i += 1;
    }
    sort_text_what(&mut text, &mut what, size);
    borg_unique_size = size;
    borg_unique_text = mem_zalloc(
        (borg_unique_size as size_t)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    borg_unique_what = mem_zalloc(
        (borg_unique_size as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        let ref mut fresh5 = *borg_unique_text.offset(i as isize);
        *fresh5 = text[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < size {
        *borg_unique_what.offset(i as isize) = what[i as usize] as ::core::ffi::c_uint;
        i += 1;
    }
    size = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < r_max as ::core::ffi::c_int {
        let mut r_ptr_0: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr_0).name.is_null() {
            if !flag_has(
                &raw mut (*r_ptr_0).flags as *mut bitflag,
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
                text[size as usize] = borg_massage_special_chars((*r_ptr_0).name);
                what[size as usize] = i as int16_t;
                size += 1;
                if !(*r_ptr_0).plural.is_null() {
                    text[size as usize] = borg_massage_special_chars((*r_ptr_0).plural);
                    what[size as usize] = i as int16_t;
                    size += 1;
                }
            }
        }
        i += 1;
    }
    sort_text_what(&mut text, &mut what, size);
    borg_normal_size = size;
    borg_normal_text = mem_zalloc(
        (borg_normal_size as size_t)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    borg_normal_what = mem_zalloc(
        (borg_normal_size as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        let ref mut fresh6 = *borg_normal_text.offset(i as isize);
        *fresh6 = text[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < size {
        *borg_normal_what.offset(i as isize) = what[i as usize] as ::core::ffi::c_uint;
        i += 1;
    }
}
unsafe extern "C" fn borg_free_monster_names() {
    let mut i: ::core::ffi::c_int = 0;
    if !borg_normal_what.is_null() {
        mem_free(borg_normal_what as *mut ::core::ffi::c_void);
        borg_normal_what = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    }
    if !borg_normal_text.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < borg_normal_size {
            string_free(*borg_normal_text.offset(i as isize) as *mut ::core::ffi::c_char);
            i += 1;
        }
        mem_free(borg_normal_text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        borg_normal_text = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    borg_normal_size = 0 as ::core::ffi::c_int;
    if !borg_unique_what.is_null() {
        mem_free(borg_unique_what as *mut ::core::ffi::c_void);
        borg_unique_what = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    }
    if !borg_unique_text.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < borg_unique_size {
            string_free(*borg_unique_text.offset(i as isize) as *mut ::core::ffi::c_char);
            i += 1;
        }
        mem_free(borg_unique_text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        borg_unique_text = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    borg_unique_size = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_kill() {
    borg_free_flow_kill();
    borg_kills_cnt = 0 as int16_t;
    borg_kills_nxt = 1 as int16_t;
    borg_kills =
        mem_zalloc((256 as size_t).wrapping_mul(::core::mem::size_of::<borg_kill>() as size_t))
            as *mut borg_kill;
    borg_race_count = mem_zalloc(
        ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    borg_race_death = mem_zalloc(
        ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        let mut r_ptr: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr).name.is_null() {
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
                if (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *borg_race_death.offset(i as isize) = 1 as int16_t;
                }
            }
        }
        i += 1;
    }
    borg_init_monster_names();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_kill() {
    borg_free_monster_names();
    if !borg_race_death.is_null() {
        mem_free(borg_race_death as *mut ::core::ffi::c_void);
        borg_race_death = ::core::ptr::null_mut::<int16_t>();
    }
    if !borg_race_count.is_null() {
        mem_free(borg_race_count as *mut ::core::ffi::c_void);
        borg_race_count = ::core::ptr::null_mut::<int16_t>();
    }
    if !borg_kills.is_null() {
        mem_free(borg_kills as *mut ::core::ffi::c_void);
        borg_kills = ::core::ptr::null_mut::<borg_kill>();
    }
}

/// Bridge to the safe `borg_sort` for the C string arrays built here
/// (borg_sort_comp_hook / borg_sort_swap_hook).
unsafe fn sort_text_what(
    text: &mut [*mut ::core::ffi::c_char],
    what: &mut [int16_t],
    size: ::core::ffi::c_int,
) {
    let n = size as usize;
    let mut t: Vec<&::core::ffi::CStr> =
        text[..n].iter().map(|&p| ::core::ffi::CStr::from_ptr(p)).collect();
    crate::borg_util::borg_sort(
        &mut crate::borg_util::TextWhat { text: &mut t, what: &mut what[..n] },
        size,
    );
    for (dst, src) in text[..n].iter_mut().zip(t) {
        *dst = src.as_ptr() as *mut ::core::ffi::c_char;
    }
}
