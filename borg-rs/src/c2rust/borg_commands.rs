pub const TARGET_LOOK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLOUR_ORANGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const COLOUR_RED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COLOUR_GREEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLOUR_BLUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COLOUR_YELLOW: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const COLOUR_L_BLUE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const BORG_MARK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_TEMP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const BORG_XTRA: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


use crate::headers::borg_danger_h::{borg_danger};

pub use crate::headers::borg_flow_kill_h::{borg_kill};
pub use crate::headers::borg_flow_take_h::{borg_take};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_home_notice_h::borg_notice_home;
use crate::headers::borg_home_power_h::borg_power_home;
pub use crate::headers::borg_init_h::{
    borg_free, borg_init, borg_init_txt_file, borg_reinit_options,
};
use crate::headers::borg_inventory_h::borg_inventory;
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::borg_item_h::{borg_find_all_items, borg_item};
use crate::headers::borg_item_id_h::borg_object_fully_id;
use crate::headers::borg_item_wear_h::borg_clear_best;
use crate::headers::borg_log_h::{borg_display_item, borg_write_map};
pub use crate::headers::borg_magic_h::{borg_can_cast, borg_magic, borg_spell_fail_rate, borg_spell_legal, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW, HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL, ZONE_OF_UNMAGIC};
use crate::headers::borg_messages_h::borg_match;
use crate::headers::borg_power_h::borg_power;
use crate::headers::borg_prepared_h::borg_prepared;
use crate::headers::borg_projection_h::{borg_inc_motion, borg_los, borg_projectable_dark};
use crate::headers::borg_reincarnate_h::borg_force_reincarnate;
pub use crate::headers::borg_trait_h::{
    borg_notice, prefix_pref, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW,
    BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM,
    BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL,
    BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID,
    BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL,
    BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE,
    BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED,
    BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS,
    BI_BART, BI_BID, BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX,
    BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY,
    BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING,
    BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP,
    BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN,
    BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO,
    BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP, BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX,
    BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY, BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1,
    BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD, BI_FOOD_CURE_BLIND,
    BI_FOOD_CURE_CONF, BI_FOOD_HI, BI_FOOD_LO, BI_FRACT, BI_GOLD, BI_GOOD_S_CHG, BI_GOOD_W_CHG,
    BI_HASFIXEXP, BI_HEAVYBOW, BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ, BI_IACID, BI_ICOLD, BI_IELEC,
    BI_IFIRE, BI_INFRA, BI_INT, BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID, BI_ISBLIND, BI_ISCONFUSED,
    BI_ISCUT, BI_ISENCUMB, BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX, BI_ISFIXEXP, BI_ISFIXINT,
    BI_ISFIXLEV, BI_ISFIXSTR, BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL, BI_ISGORGED, BI_ISHEAVYSTUN,
    BI_ISHUNGRY, BI_ISIMAGE, BI_ISPARALYZED, BI_ISPOISONED, BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK,
    BI_KING, BI_LIGHT, BI_MAX, BI_MAXCLEVEL, BI_MAXDEPTH, BI_MAXHP, BI_MAXSP, BI_MOD_MOVES,
    BI_MULTIPLE_BONUSES, BI_NEED_BRAND_WEAPON, BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D,
    BI_NEED_ENCHANT_TO_H, BI_PREP_BIG_FIGHT, BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD,
    BI_RCONF, BI_RDARK, BI_RDIS, BI_RECALL, BI_REG, BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS,
    BI_RLITE, BI_RNTHR, BI_RNXUS, BI_RPOIS, BI_RSHRD, BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON,
    BI_SDEX, BI_SDIG, BI_SFRACT, BI_SHLIFE, BI_SHOTS, BI_SINT, BI_SINV, BI_SLING, BI_SPEED,
    BI_SP_ADJ, BI_SRACID, BI_SRBLIND, BI_SRCH, BI_SRCOLD, BI_SRCONF, BI_SRDARK, BI_SRDIS,
    BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS, BI_SRLITE, BI_SRNTHR, BI_SRNXUS, BI_SRPOIS,
    BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR, BI_STR_INDEX, BI_SWIS, BI_THB, BI_THN, BI_THT,
    BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD, BI_WB_ELEC, BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS,
    BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS, BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON,
    BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL, BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL,
    BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD, BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
use crate::headers::borg_trait_swap_h::{
    armour_swap, armour_swap_value, weapon_swap, weapon_swap_value,
};
use crate::headers::borg_update_h::borg_update;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, f_info, feature, heatmap, monster_group, square,
    square_in_bounds, trap, C2RustUnnamedH_18, C2RustUnnamedH_24, FEAT_MAX, SQUARE_MAX,
};
pub use crate::headers::cmd_core_h::{
    cmd_code, CMD_ACCEPT_CHARACTER, CMD_ACTIVATE, CMD_ALTER, CMD_AUTOINSCRIBE, CMD_AUTOPICKUP,
    CMD_BIRTH_INIT, CMD_BIRTH_RESET, CMD_BROWSE_SPELL, CMD_BUY, CMD_BUY_STAT, CMD_CAST,
    CMD_CHOOSE_CLASS, CMD_CHOOSE_RACE, CMD_CLOSE, CMD_COMMAND_MONSTER, CMD_DISARM, CMD_DROP,
    CMD_EAT, CMD_EXPLORE, CMD_FIRE, CMD_GO_DOWN, CMD_GO_UP, CMD_HELP, CMD_HISTORY_CHOICE, CMD_HOLD,
    CMD_IGNORE, CMD_INSCRIBE, CMD_JUMP, CMD_LOADFILE, CMD_NAME_CHOICE, CMD_NAVIGATE_DOWN,
    CMD_NAVIGATE_UP, CMD_NEWGAME, CMD_NULL, CMD_OPEN, CMD_PATHFIND, CMD_PICKUP, CMD_PREV_STATS,
    CMD_QUAFF, CMD_READ_SCROLL, CMD_REFILL, CMD_REFRESH_STATS, CMD_REPEAT, CMD_RESET_STATS,
    CMD_REST, CMD_RETIRE, CMD_RETRIEVE, CMD_ROLL_STATS, CMD_RUN, CMD_SELL, CMD_SELL_STAT,
    CMD_SLEEP, CMD_SPOIL_ARTIFACT, CMD_SPOIL_MON, CMD_SPOIL_MON_BRIEF, CMD_SPOIL_OBJ, CMD_STASH,
    CMD_STEAL, CMD_STUDY, CMD_TAKEOFF, CMD_THROW, CMD_TUNNEL, CMD_UNINSCRIBE, CMD_USE, CMD_USE_ROD,
    CMD_USE_STAFF, CMD_USE_WAND, CMD_WALK, CMD_WIELD, CMD_WIZ_ACQUIRE, CMD_WIZ_ADVANCE,
    CMD_WIZ_BANISH, CMD_WIZ_CHANGE_ITEM_QUANTITY, CMD_WIZ_COLLECT_DISCONNECT_STATS,
    CMD_WIZ_COLLECT_OBJ_MON_STATS, CMD_WIZ_COLLECT_PIT_STATS, CMD_WIZ_CREATE_ALL_ARTIFACT,
    CMD_WIZ_CREATE_ALL_ARTIFACT_FROM_TVAL, CMD_WIZ_CREATE_ALL_OBJ,
    CMD_WIZ_CREATE_ALL_OBJ_FROM_TVAL, CMD_WIZ_CREATE_ARTIFACT, CMD_WIZ_CREATE_OBJ,
    CMD_WIZ_CREATE_TRAP, CMD_WIZ_CURE_ALL, CMD_WIZ_CURSE_ITEM, CMD_WIZ_DETECT_ALL_LOCAL,
    CMD_WIZ_DETECT_ALL_MONSTERS, CMD_WIZ_DUMP_LEVEL_MAP, CMD_WIZ_EDIT_PLAYER_EXP,
    CMD_WIZ_EDIT_PLAYER_GOLD, CMD_WIZ_EDIT_PLAYER_START, CMD_WIZ_EDIT_PLAYER_STAT,
    CMD_WIZ_HIT_ALL_LOS, CMD_WIZ_INCREASE_EXP, CMD_WIZ_JUMP_LEVEL, CMD_WIZ_LEARN_OBJECT_KINDS,
    CMD_WIZ_MAGIC_MAP, CMD_WIZ_PEEK_NOISE_SCENT, CMD_WIZ_PERFORM_EFFECT, CMD_WIZ_PLAY_ITEM,
    CMD_WIZ_PUSH_OBJECT, CMD_WIZ_QUERY_FEATURE, CMD_WIZ_QUERY_SQUARE_FLAG, CMD_WIZ_QUIT_NO_SAVE,
    CMD_WIZ_RECALL_MONSTER, CMD_WIZ_RERATE, CMD_WIZ_REROLL_ITEM, CMD_WIZ_STAT_ITEM,
    CMD_WIZ_SUMMON_NAMED, CMD_WIZ_SUMMON_RANDOM, CMD_WIZ_TELEPORT_RANDOM, CMD_WIZ_TELEPORT_TO,
    CMD_WIZ_TWEAK_ITEM, CMD_WIZ_WIPE_RECALL, CMD_WIZ_WIZARD_LIGHT,
};
pub use crate::headers::game_event_h::{
    event_signal, game_event_type, EVENT_AC, EVENT_ANIMATE, EVENT_BELL, EVENT_BIRTHPOINTS,
    EVENT_BOLT, EVENT_CHEAT_DEATH, EVENT_CHECK_INTERRUPT, EVENT_COMMAND_REPEAT,
    EVENT_DETECTIONSTATUS, EVENT_DUNGEONLEVEL, EVENT_END, EVENT_ENTER_BIRTH, EVENT_ENTER_DEATH,
    EVENT_ENTER_GAME, EVENT_ENTER_INIT, EVENT_ENTER_STORE, EVENT_ENTER_WORLD, EVENT_EQUIPMENT,
    EVENT_EXPERIENCE, EVENT_EXPLOSION, EVENT_FEELING, EVENT_GEN_LEVEL_END, EVENT_GEN_LEVEL_START,
    EVENT_GEN_ROOM_CHOOSE_SIZE, EVENT_GEN_ROOM_CHOOSE_SUBTYPE, EVENT_GEN_ROOM_END,
    EVENT_GEN_ROOM_START, EVENT_GEN_TUNNEL_FINISHED, EVENT_GOLD, EVENT_HP, EVENT_INITSTATUS,
    EVENT_INPUT_FLUSH, EVENT_INVENTORY, EVENT_ITEMLIST, EVENT_LEAVE_BIRTH, EVENT_LEAVE_DEATH,
    EVENT_LEAVE_GAME, EVENT_LEAVE_INIT, EVENT_LEAVE_STORE, EVENT_LEAVE_WORLD, EVENT_LIGHT,
    EVENT_MANA, EVENT_MAP, EVENT_MESSAGE, EVENT_MESSAGE_FLUSH, EVENT_MISSILE, EVENT_MONSTERHEALTH,
    EVENT_MONSTERLIST, EVENT_MONSTERTARGET, EVENT_NEW_LEVEL_DISPLAY, EVENT_OBJECTTARGET,
    EVENT_PLAYERLEVEL, EVENT_PLAYERMOVED, EVENT_PLAYERSPEED, EVENT_PLAYERTITLE, EVENT_RACE_CLASS,
    EVENT_REFRESH, EVENT_SEEFLOOR, EVENT_SOUND, EVENT_STATE, EVENT_STATS, EVENT_STATUS,
    EVENT_STORECHANGED, EVENT_STUDYSTATUS, EVENT_USE_STORE,
};
use crate::headers::game_input_h::{get_com, get_quantity, get_string};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_elements_h::{
    PROJ_ACID, PROJ_ARROW, PROJ_CHAOS, PROJ_COLD, PROJ_DARK, PROJ_DISEN, PROJ_ELEC, PROJ_FIRE,
    PROJ_FORCE, PROJ_GRAVITY, PROJ_HOLY_ORB, PROJ_ICE, PROJ_INERTIA, PROJ_LIGHT, PROJ_MANA,
    PROJ_METEOR, PROJ_MISSILE, PROJ_NETHER, PROJ_NEXUS, PROJ_PLASMA, PROJ_POIS, PROJ_SHARD,
    PROJ_SOUND, PROJ_TIME, PROJ_WATER,
};
pub use crate::headers::list_projections_h::{
    PROJ_AWAY_ALL, PROJ_AWAY_EVIL, PROJ_AWAY_SPIRIT, PROJ_AWAY_UNDEAD, PROJ_DARK_WEAK,
    PROJ_DISP_ALL, PROJ_DISP_EVIL, PROJ_DISP_UNDEAD, PROJ_KILL_DOOR, PROJ_KILL_TRAP,
    PROJ_KILL_WALL, PROJ_LIGHT_WEAK, PROJ_MAKE_DOOR, PROJ_MAKE_TRAP, PROJ_MON_CLONE, PROJ_MON_CONF,
    PROJ_MON_CRUSH, PROJ_MON_DRAIN, PROJ_MON_HEAL, PROJ_MON_HOLD, PROJ_MON_POLY, PROJ_MON_SLOW,
    PROJ_MON_SPEED, PROJ_MON_STUN, PROJ_SLEEP_ALL, PROJ_SLEEP_EVIL, PROJ_SLEEP_UNDEAD,
    PROJ_TURN_ALL, PROJ_TURN_EVIL, PROJ_TURN_LIVING, PROJ_TURN_UNDEAD,
};
pub use crate::headers::list_square_flags_h::{
    SQUARE_CLOSE_PLAYER, SQUARE_DTRAP, SQUARE_FEEL, SQUARE_GLOW, SQUARE_INVIS, SQUARE_MARK,
    SQUARE_MON_RESTRICT, SQUARE_NONE, SQUARE_NO_ESP, SQUARE_NO_MAP, SQUARE_NO_STAIRS,
    SQUARE_NO_TELEPORT, SQUARE_PROJECT, SQUARE_ROOM, SQUARE_SEEN, SQUARE_TRAP, SQUARE_VAULT,
    SQUARE_VIEW, SQUARE_WALL_INNER, SQUARE_WALL_OUTER, SQUARE_WALL_SOLID, SQUARE_WASSEEN,
};
pub use crate::headers::list_stats_h::{STAT_CON, STAT_DEX, STAT_INT, STAT_STR, STAT_WIS};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
use crate::headers::message_h::msg;
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::object_h::{
    a_info, activation, artifact, chest_trap, curse_data, effect, ego_item, element_info, flavor,
    k_info, object, object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23, STAT_MAX,
};
pub use crate::headers::project_h::{
    project, C2RustUnnamedH_27, C2RustUnnamedH_28, PROJECT_ARC, PROJECT_AWARE, PROJECT_BEAM,
    PROJECT_GRID, PROJECT_HIDE, PROJECT_INFO, PROJECT_ITEM, PROJECT_JUMP, PROJECT_KILL,
    PROJECT_NONE, PROJECT_PLAY, PROJECT_ROCK, PROJECT_SAFE, PROJECT_SELF, PROJECT_SHORT,
    PROJECT_STOP, PROJECT_THRU, PROJ_MAX,
};
pub use crate::headers::source_h::{
    source, source_player, C2RustUnnamedH_25, C2RustUnnamedH_26, SRC_CHEST_TRAP, SRC_MONSTER,
    SRC_NONE, SRC_OBJECT, SRC_PLAYER, SRC_TRAP,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::stdio_h::snprintf;
use crate::headers::stdlib_h::atoi;
pub use crate::headers::target_h::{target, target_get, target_okay};
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_input_h::{cmd_info, get_com_ex};
use crate::headers::ui_map_h::{print_rel, prt_map};
pub use crate::headers::ui_menu_h::{
    menu, menu_iter, menu_layout, menu_new, menu_select, menu_setpriv, menu_skin, skin_id,
    MN_SKIN_COLUMNS, MN_SKIN_OBJECT, MN_SKIN_SCROLL,
};
pub use crate::headers::ui_output_h::{c_put_str, region, screen_load, screen_save, window_make};
use crate::headers::ui_target_h::target_set_interactive;
pub use crate::headers::ui_term_h::{
    col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width, Term,
    Term_clear, Term_erase, Term_flush, Term_fresh, Term_load, Term_putstr, Term_save,
};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};

pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::{format, strnfmt};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, my_strcpy};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_commands {
    pub selection: ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub function: Option<unsafe extern "C" fn() -> ()>,
}
static mut borg_commands: [borg_commands; 29] = unsafe {
    [
        borg_commands {
            selection: '?' as i32 as ::core::ffi::c_char,
            description: b"borg command help\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_help as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '$' as i32 as ::core::ffi::c_char,
            description: b"reload borg.txt file\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_init_txt_file as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'z' as i32 as ::core::ffi::c_char,
            description: b"activate borg\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_start as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'x' as i32 as ::core::ffi::c_char,
            description: b"step the borg\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_step as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'm' as i32 as ::core::ffi::c_char,
            description: b"show map information\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_map as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'G' as i32 as ::core::ffi::c_char,
            description: b"target grid features\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_feature as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'I' as i32 as ::core::ffi::c_char,
            description: b"target grid info\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_info as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'D' as i32 as ::core::ffi::c_char,
            description: b"target grid danger\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_danger as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '@' as i32 as ::core::ffi::c_char,
            description: b"line of sight map\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_line_of_sight as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'F' as i32 as ::core::ffi::c_char,
            description: b"map of fear levels\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_fear as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'f' as i32 as ::core::ffi::c_char,
            description: b"update config flags\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_flags as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'u' as i32 as ::core::ffi::c_char,
            description: b"refresh borg vars\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_update as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'c' as i32 as ::core::ffi::c_char,
            description: b"toggle cheat death\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_cheat as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 's' as i32 as ::core::ffi::c_char,
            description: b"set search string\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_search_string as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '^' as i32 as ::core::ffi::c_char,
            description: b"show the borg path\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_flow as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'y' as i32 as ::core::ffi::c_char,
            description: b"step history\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_steps as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 't' as i32 as ::core::ffi::c_char,
            description: b"current targeting\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_targeting as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'h' as i32 as ::core::ffi::c_char,
            description: b"things the borg has\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_has as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'p' as i32 as ::core::ffi::c_char,
            description: b"borg power\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_power as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'P' as i32 as ::core::ffi::c_char,
            description: b"preparation depth\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_prepare as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '!' as i32 as ::core::ffi::c_char,
            description: b"time tracking flags\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_time as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'C' as i32 as ::core::ffi::c_char,
            description: b"list count of 'nasties'\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_nasties as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '0' as i32 as ::core::ffi::c_char,
            description: b"borg stats (str/int etc)\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_stats as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'w' as i32 as ::core::ffi::c_char,
            description: b"swap weapons/armor\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_swaps as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'S' as i32 as ::core::ffi::c_char,
            description: b"spells\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_spells as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'l' as i32 as ::core::ffi::c_char,
            description: b"creates snapshot log file\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_dump as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'o' as i32 as ::core::ffi::c_char,
            description: b"object details\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_object_desc as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'R' as i32 as ::core::ffi::c_char,
            description: b"respawn\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_respawn as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'v' as i32 as ::core::ffi::c_char,
            description: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_version as unsafe extern "C" fn() -> ()),
        },
    ]
};
unsafe extern "C" fn borg_cmd_target() -> bool {
    let mut cmd: ::core::ffi::c_char = 'n' as i32 as ::core::ffi::c_char;
    Term_load();
    Term_flush();
    if target_okay() {
        if !get_com(
            b"Use current target? (y/n) \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut cmd,
        ) {
            borg_note(b"command aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
    }
    if cmd as ::core::ffi::c_int != 'y' as i32 && cmd as ::core::ffi::c_int != 'Y' as i32 {
        let mut ke: ui_event = ui_event { type_0: EVT_NONE };
        if !get_com_ex(
            b"Select grid location (t to target): \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ke,
        ) {
            borg_note(b"targeting aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
        if !target_set_interactive(TARGET_LOOK, crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y, true_0 != 0) {
            borg_note(b"targeting aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_cmd_help() {
    let mut i: ::core::ffi::c_int = 0;
    let mut line: ::core::ffi::c_int = 0;
    let command_count: ::core::ffi::c_int = (::core::mem::size_of::<[borg_commands; 29]>() as usize)
        .wrapping_div(::core::mem::size_of::<borg_commands>() as usize)
        as ::core::ffi::c_int;
    Term_save();
    Term_clear();
    i = 0 as ::core::ffi::c_int;
    line = 2 as ::core::ffi::c_int;
    while i < command_count {
        Term_putstr(
            2 as ::core::ffi::c_int,
            line,
            -(1 as ::core::ffi::c_int),
            COLOUR_WHITE,
            format(
                b"Command '%c' %s\0" as *const u8 as *const ::core::ffi::c_char,
                borg_commands[i as usize].selection as ::core::ffi::c_int,
                borg_commands[i as usize].description,
            ),
        );
        if (i + 1 as ::core::ffi::c_int) < command_count {
            Term_putstr(
                42 as ::core::ffi::c_int,
                line,
                -(1 as ::core::ffi::c_int),
                COLOUR_WHITE,
                format(
                    b"Command '%c' %s\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_commands[(i + 1 as ::core::ffi::c_int) as usize].selection
                        as ::core::ffi::c_int,
                    borg_commands[(i + 1 as ::core::ffi::c_int) as usize].description,
                ),
            );
        }
        i += 2 as ::core::ffi::c_int;
        line += 1;
    }
    do_cmd_borg();
    Term_load();
}
unsafe extern "C" fn borg_cmd_start() {
    borg_reinit_options();
    borg_clear_best();
    crate::bridge::st().borg.status.active = true_0 != 0;
    crate::bridge::st().borg.status.cancel = false_0 != 0;
    crate::bridge::st().core.step = 0 as uint16_t;
    if (*player).opts.lazymove_delay as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        borg_note(
            b"# Turning off lazy movement controls\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*player).opts.lazymove_delay = 0 as uint8_t;
    }
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    crate::borg::borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_init_txt_file() {
    borg_init_txt_file();
}
unsafe extern "C" fn borg_cmd_avoidances() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
            if !(p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int) {
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a = COLOUR_ORANGE as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
                    a = COLOUR_YELLOW as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 6 as ::core::ffi::c_int {
                    a = COLOUR_GREEN as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
                    a = COLOUR_BLUE as uint8_t;
                }
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Avoidance value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        (*Term).offset_y / crate::borg_cave_util::borg_panel_hgt(),
        (*Term).offset_x / crate::borg_cave_util::borg_panel_wid(),
        crate::bridge::st().borg.avoidance as ::core::ffi::c_int,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
}
unsafe extern "C" fn borg_cmd_step() {
    borg_reinit_options();
    borg_clear_best();
    crate::bridge::st().borg.status.active = true_0 != 0;
    crate::bridge::st().borg.status.cancel = false_0 != 0;
    crate::bridge::st().core.step = get_quantity(
        b"Step how many times? \0" as *const u8 as *const ::core::ffi::c_char,
        1000 as ::core::ffi::c_int,
    ) as uint16_t;
    if (crate::bridge::st().core.step as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        crate::bridge::st().core.step = 1 as uint16_t;
    }
    borg_notice(true_0 != 0);
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    borg_note(format(
        b"# Stepping Borg %d times\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.step as ::core::ffi::c_int,
    ));
    crate::borg::borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_update() {
    borg_reinit_options();
    crate::bridge::st().borg.status.active = true_0 != 0;
    crate::bridge::st().borg.status.cancel = true_0 != 0;
    crate::bridge::st().core.step = 0 as uint16_t;
    borg_notice(true_0 != 0);
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    crate::borg::borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn get_cfg_display(
    mut menu: *mut menu,
    mut oid: ::core::ffi::c_int,
    mut cursor: bool,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut value: [::core::ffi::c_char; 40] = [0; 40];
    let mut label_attr: ::core::ffi::c_int = if cursor as ::core::ffi::c_int != 0 {
        COLOUR_L_BLUE
    } else {
        COLOUR_WHITE
    };
    Term_erase(
        if (col - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            col - 1 as ::core::ffi::c_int
        },
        row,
        width,
    );
    c_put_str(
        label_attr as uint8_t,
        crate::borg_init::BORG_SETTINGS[oid as usize].name.as_ptr(),
        row,
        col,
    );
    if crate::borg_init::BORG_SETTINGS[oid as usize].kind
        as ::core::ffi::c_int
        == 'b' as i32
    {
        my_strcpy(
            &raw mut value as *mut ::core::ffi::c_char,
            if crate::bridge::st().core.cfg[(oid) as usize] != 0 {
                b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 40]>() as size_t,
        );
    } else {
        snprintf(
            &raw mut value as *mut ::core::ffi::c_char,
            30 as size_t,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            crate::bridge::st().core.cfg[(oid) as usize],
        );
    }
    c_put_str(
        label_attr as uint8_t,
        &raw mut value as *mut ::core::ffi::c_char,
        row,
        col + 30 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn borg_cfg_get_quantity(mut config: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut amt: ::core::ffi::c_int = crate::bridge::st().core.cfg[(config) as usize];
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    strnfmt(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        amt,
    );
    if !get_string(
        format(
            b"Enter new temp value for %s: \0" as *const u8 as *const ::core::ffi::c_char,
            crate::borg_init::BORG_SETTINGS[config as usize].name.as_ptr(),
        ),
        &raw mut buf as *mut ::core::ffi::c_char,
        7 as size_t,
    ) {
        return 0 as ::core::ffi::c_int;
    }
    amt = atoi(&raw mut buf as *mut ::core::ffi::c_char);
    if amt < -(1 as ::core::ffi::c_int) {
        amt = -(1 as ::core::ffi::c_int);
    }
    return amt;
}
unsafe extern "C" fn borg_cmd_flags() {
    static mut borg_cfg_menu_iter: menu_iter = unsafe {
        menu_iter {
            get_tag: None,
            valid_row: None,
            display_row: Some(
                get_cfg_display
                    as unsafe extern "C" fn(
                        *mut menu,
                        ::core::ffi::c_int,
                        bool,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            row_handler: None,
            resize: None,
        }
    };
    let mut area: region = region {
        col: 21 as ::core::ffi::c_int,
        row: 5 as ::core::ffi::c_int,
        width: 1 as ::core::ffi::c_int,
        page_rows: 10 as ::core::ffi::c_int,
    };
    let mut choice: *mut cmd_info = ::core::ptr::null_mut::<cmd_info>();
    let mut m: *mut menu = ::core::ptr::null_mut::<menu>();
    let mut result: ui_event = ui_event { type_0: EVT_NONE };
    msg(
        b"Pick a configuration setting to change (runtime only)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    m = menu_new(MN_SKIN_SCROLL, &raw mut borg_cfg_menu_iter);
    menu_setpriv(
        m,
        BORG_MAX_SETTINGS as ::core::ffi::c_int,
        &raw mut borg_cfg_menu_iter as *mut ::core::ffi::c_void,
    );
    menu_setpriv(
        m,
        BORG_MAX_SETTINGS as ::core::ffi::c_int,
        &raw mut choice as *mut ::core::ffi::c_void,
    );
    menu_layout(m, &raw mut area);
    screen_save();
    window_make(
        19 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    );
    result = menu_select(m, 0 as ::core::ffi::c_int, true_0 != 0);
    screen_load();
    if result.type_0 as ::core::ffi::c_uint
        == EVT_ESCAPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if result.type_0 as ::core::ffi::c_uint
        == EVT_SELECT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*m).cursor < 0 as ::core::ffi::c_int
            || (*m).cursor >= BORG_MAX_SETTINGS as ::core::ffi::c_int
        {
            borg_note(b"oops\0" as *const u8 as *const ::core::ffi::c_char);
            return;
        }
        if crate::borg_init::BORG_SETTINGS[(*m).cursor as usize].kind as ::core::ffi::c_int
            == 'b' as i32
        {
            crate::bridge::st().core.cfg[((*m).cursor) as usize] =
                (crate::bridge::st().core.cfg[((*m).cursor) as usize] == 0) as ::core::ffi::c_int;
            msg(format(
                b"new temp config for %s is %s\0" as *const u8 as *const ::core::ffi::c_char,
                crate::borg_init::BORG_SETTINGS[(*m).cursor as usize].name.as_ptr(),
                if crate::bridge::st().core.cfg[((*m).cursor) as usize] != 0 {
                    b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                },
            ));
        } else {
            let mut new_value: ::core::ffi::c_int = borg_cfg_get_quantity((*m).cursor);
            crate::bridge::st().core.cfg[((*m).cursor) as usize] = new_value;
            msg(format(
                b"new temp config for %s is %d\0" as *const u8 as *const ::core::ffi::c_char,
                crate::borg_init::BORG_SETTINGS[(*m).cursor as usize].name.as_ptr(),
                crate::bridge::st().core.cfg[((*m).cursor) as usize],
            ));
        }
    }
}
unsafe extern "C" fn borg_cmd_cheat() {
    crate::bridge::st().borg.status.cheat_death = !crate::bridge::st().borg.status.cheat_death;
    msg(
        b"Borg -- borg_cheat_death is now %d.\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.status.cheat_death as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn borg_cmd_nasties() {
    let mut i: ::core::ffi::c_int = 0;
    borg_note(b"Borg Nasties Count\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while i < crate::bridge::st().kills.nasties_num as ::core::ffi::c_int {
        borg_note(format(
            b"Nasty: [%c] Count: %d, limited: %d\0" as *const u8 as *const ::core::ffi::c_char,
            crate::bridge::st().kills.nasties[i as usize] as ::core::ffi::c_int,
            crate::bridge::st().kills.nasties_count[i as usize] as ::core::ffi::c_int,
            crate::bridge::st().kills.nasties_limit[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    msg(b"Borg Nasty Dump Complete.  Examine Log.\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn borg_cmd_search_string() {
    if !get_string(
        b"Borg Match String: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_match as *mut ::core::ffi::c_char,
        70 as size_t,
    ) {
        my_strcpy(
            &raw mut borg_match as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        );
        msg(b"Borg Match String de-activated.\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn get_feature_display(
    mut menu: *mut menu,
    mut oid: ::core::ffi::c_int,
    mut cursor: bool,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut label_attr: ::core::ffi::c_int = if cursor as ::core::ffi::c_int != 0 {
        COLOUR_L_BLUE
    } else {
        COLOUR_WHITE
    };
    Term_erase(
        if (col - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            col - 1 as ::core::ffi::c_int
        },
        row,
        width,
    );
    if oid < FEAT_MAX as ::core::ffi::c_int {
        c_put_str(
            label_attr as uint8_t,
            (*f_info.offset(oid as isize)).name,
            row,
            col,
        );
    } else {
        let mut i: ::core::ffi::c_int =
            oid - FEAT_MAX as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
        if i >= 0 as ::core::ffi::c_int {
            match i {
                0 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"MARK\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                1 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"GLOW\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                2 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"DARK\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                3 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"OKAY\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                4 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"LIGHT\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                5 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"VIEW\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                6 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"TEMP\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                7 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"XTRA\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                8 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"IGNORE_MAP\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                _ => {}
            }
        } else {
            if oid == FEAT_MAX as ::core::ffi::c_int {
                c_put_str(
                    label_attr as uint8_t,
                    b"Traps\0" as *const u8 as *const ::core::ffi::c_char,
                    row,
                    col,
                );
            }
            if oid == FEAT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                c_put_str(
                    label_attr as uint8_t,
                    b"Glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                    row,
                    col,
                );
            }
        }
    };
}
unsafe extern "C" fn borg_cmd_grid_feature() {
    static mut borg_feature_menu_iter: menu_iter = unsafe {
        menu_iter {
            get_tag: None,
            valid_row: None,
            display_row: Some(
                get_feature_display
                    as unsafe extern "C" fn(
                        *mut menu,
                        ::core::ffi::c_int,
                        bool,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            row_handler: None,
            resize: None,
        }
    };
    let mut area: region = region {
        col: 21 as ::core::ffi::c_int,
        row: 5 as ::core::ffi::c_int,
        width: 1 as ::core::ffi::c_int,
        page_rows: 10 as ::core::ffi::c_int,
    };
    let mut m: *mut menu = ::core::ptr::null_mut::<menu>();
    let mut result: ui_event = ui_event { type_0: EVT_NONE };
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    m = menu_new(MN_SKIN_SCROLL, &raw mut borg_feature_menu_iter);
    menu_setpriv(
        m,
        FEAT_MAX as ::core::ffi::c_int + 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
        &raw mut borg_feature_menu_iter as *mut ::core::ffi::c_void,
    );
    menu_layout(m, &raw mut area);
    screen_save();
    window_make(
        19 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    );
    result = menu_select(m, 0 as ::core::ffi::c_int, true_0 != 0);
    screen_load();
    if result.type_0 as ::core::ffi::c_uint
        == EVT_ESCAPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        let mut current_block_13: u64;
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            let mut ag: *mut borg_grid = crate::bridge::grid(x, y) as *mut borg_grid;
            if (*m).cursor < FEAT_MAX as ::core::ffi::c_int {
                if !((*ag).feat as ::core::ffi::c_int == (*m).cursor) {
                    current_block_13 = 11812396948646013369;
                } else {
                    current_block_13 = 7659304154607701039;
                }
            } else {
                let mut i: ::core::ffi::c_int =
                    (*m).cursor - FEAT_MAX as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
                if i >= 0 as ::core::ffi::c_int {
                    match i {
                        0 => {
                            current_block_13 = 12465488344595199675;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block_13 = 10562881441515713487;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block_13 = 3833396878894470733;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        3 => {
                            current_block_13 = 41435768513642067;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        4 => {
                            current_block_13 = 5412339429498267972;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        5 => {
                            current_block_13 = 13065097511781892099;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        6 => {
                            current_block_13 = 18315407756364132905;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        7 => {
                            current_block_13 = 12493991067027566107;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        8 => {
                            current_block_13 = 4533473061595253016;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        _ => {
                            current_block_13 = 7659304154607701039;
                        }
                    }
                } else {
                    if (*m).cursor == FEAT_MAX as ::core::ffi::c_int {
                        if !(*ag).trap {
                            current_block_13 = 11812396948646013369;
                        } else {
                            current_block_13 = 7333393191927787629;
                        }
                    } else {
                        current_block_13 = 7333393191927787629;
                    }
                    match current_block_13 {
                        11812396948646013369 => {}
                        _ => {
                            if (*m).cursor
                                == FEAT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            {
                                if !(*ag).glyph {
                                    current_block_13 = 11812396948646013369;
                                } else {
                                    current_block_13 = 7659304154607701039;
                                }
                            } else {
                                current_block_13 = 7659304154607701039;
                            }
                        }
                    }
                }
            }
            match current_block_13 {
                7659304154607701039 => {
                    if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y, x) {
                        a = COLOUR_YELLOW as uint8_t;
                    }
                    print_rel('*' as i32, a, y, x);
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Press any key.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_feature() {
    let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut mask: uint16_t = 0;
    if !borg_cmd_target() {
        return;
    }
    let mut l: loc = loc { x: 0, y: 0 };
    target_get(&raw mut l);
    y = l.y;
    x = l.x;
    mask = (*crate::bridge::grid(x, y)).feat as uint16_t;
    let mut feat: uint8_t = (*square(cave, crate::bridge::st().borg.c)).feat;
    borg_note(format(
        b"Borg's Feat for grid (%d, %d) is %d, game Feat is %d\0" as *const u8
            as *const ::core::ffi::c_char,
        y,
        x,
        mask as ::core::ffi::c_int,
        feat as ::core::ffi::c_int,
    ));
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_info() {
    let mut i: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut l: loc = loc { x: 0, y: 0 };
    if !borg_cmd_target() {
        return;
    }
    target_get(&raw mut l);
    y = l.y;
    x = l.x;
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_MARK != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is MARK\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_GLOW != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is GLOW\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_DARK != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is DARK\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_OKAY != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is OKAY\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_LIGHT != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is LITE\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_VIEW != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is VIEW\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_TEMP != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is TEMP\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*crate::bridge::grid(x, y)).info as ::core::ffi::c_int & BORG_XTRA != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is XTRA\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < SQUARE_MAX as ::core::ffi::c_int {
        if flag_has(
            (*square(cave, l)).info,
            (SQUARE_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            i,
        ) {
            msg(format(
                b"Sys Info for grid (%d, %d) is %d\0" as *const u8 as *const ::core::ffi::c_char,
                y,
                x,
                i,
            ));
        }
        i += 1;
    }
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_danger() {
    let mut n: ::core::ffi::c_int = 0;
    let mut l: loc = loc { x: 0, y: 0 };
    if !borg_cmd_target() {
        return;
    }
    target_get(&raw mut l);
    n = get_quantity(
        b"Time on square? \0" as *const u8 as *const ::core::ffi::c_char,
        1000 as ::core::ffi::c_int,
    );
    msg(
        b"Danger(%d,%d,%d) is %d\0" as *const u8 as *const ::core::ffi::c_char,
        l.x,
        l.y,
        n,
        borg_danger(l.y, l.x, n, true_0 != 0, false_0 != 0),
    );
    prt_map();
}
unsafe extern "C" fn borg_cmd_steps() {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: uint8_t = COLOUR_RED as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < crate::bridge::st().flow.track_step.num as ::core::ffi::c_int {
        print_rel(
            '*' as i32,
            a,
            crate::bridge::st().flow.track_step.y[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
            crate::bridge::st().flow.track_step.x[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
        );
        msg(
            b"(-%d) Steps noted %d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            crate::bridge::st().flow.track_step.y[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
            crate::bridge::st().flow.track_step.x[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        print_rel(
            '*' as i32,
            COLOUR_ORANGE as uint8_t,
            crate::bridge::st().flow.track_step.y[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
            crate::bridge::st().flow.track_step.x[((crate::bridge::st().flow.track_step.num as ::core::ffi::c_int - i)) as usize],
        );
        i += 1;
    }
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_monsters() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < crate::bridge::st().kills.nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = &raw mut crate::bridge::st().kills.kills[(i) as usize] as *mut borg_kill;
        if (*kill).r_idx != 0 {
            let mut x: ::core::ffi::c_int = (*kill).pos.x;
            let mut y: ::core::ffi::c_int = (*kill).pos.y;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            n += 1;
        }
        i += 1;
    }
    msg(
        b"There are %d known monsters.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_objects() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < crate::bridge::st().takes.nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = &raw mut crate::bridge::st().takes.takes[(i) as usize] as *mut borg_take;
        if (*take).kidx != 0 {
            let mut x: ::core::ffi::c_int = (*take).x as ::core::ffi::c_int;
            let mut y: ::core::ffi::c_int = (*take).y as ::core::ffi::c_int;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            n += 1;
        }
        i += 1;
    }
    msg(
        b"There are %d known objects.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_targeting() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut l: loc = loc { x: 0, y: 0 };
    let mut ch: ::core::ffi::c_char = 0;
    let mut svDelay: uint8_t = (*player).opts.delay_factor;
    (*player).opts.delay_factor = 200 as uint8_t;
    n_x = crate::bridge::st().borg.c.x;
    n_y = crate::bridge::st().borg.c.y;
    target_get(&raw mut l);
    x = l.x;
    y = l.y;
    loop {
        print_rel('*' as i32, COLOUR_RED as uint8_t, n_y, n_x);
        if n_x == x && n_y == y {
            break;
        }
        borg_inc_motion(&raw mut n_y, &raw mut n_x, crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x);
    }
    msg(b"Borg's Targeting Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    n_x = crate::bridge::st().borg.c.x;
    n_y = crate::bridge::st().borg.c.y;
    x = l.x;
    y = l.y;
    if !get_com(
        b"Borg command: Show Arc (Y/y): \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut ch,
    ) {
        return;
    }
    msg(b"Actual Targeting Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    if ch as ::core::ffi::c_int != 'Y' as i32 && ch as ::core::ffi::c_int != 'y' as i32 {
        project(
            source_player(),
            0 as ::core::ffi::c_int,
            loc(x, y),
            1 as ::core::ffi::c_int,
            PROJ_MISSILE as ::core::ffi::c_int,
            PROJECT_BEAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as uint8_t,
            ::core::ptr::null::<object>(),
        );
    } else {
        project(
            source_player(),
            10 as ::core::ffi::c_int,
            loc(x, y),
            50 as ::core::ffi::c_int,
            PROJ_MISSILE as ::core::ffi::c_int,
            PROJECT_ARC as ::core::ffi::c_int,
            60 as ::core::ffi::c_int,
            4 as uint8_t,
            ::core::ptr::null::<object>(),
        );
    }
    (*player).opts.delay_factor = svDelay;
    prt_map();
}
unsafe extern "C" fn borg_cmd_flow() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut o: ::core::ffi::c_int = 0;
    let mut false_y: ::core::ffi::c_int = 0;
    let mut false_x: ::core::ffi::c_int = 0;
    false_y = crate::bridge::st().borg.c.y;
    false_x = crate::bridge::st().borg.c.x;
    o = 0 as ::core::ffi::c_int;
    while o < 250 as ::core::ffi::c_int {
        let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0;
        let mut b_c: ::core::ffi::c_int = 0;
        b_c = crate::bridge::st().flow.data_flow[crate::bridge::st().borg.c.y as usize][crate::bridge::st().borg.c.x as usize] as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int;
        b_c = b_c - 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = false_x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = false_y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            c = crate::bridge::st().flow.data_flow[y as usize][x as usize] as ::core::ffi::c_int
                * 10 as ::core::ffi::c_int;
            if !(c > b_c) {
                if c < b_c {
                    b_n = 0 as ::core::ffi::c_int;
                }
                b_n += 1;
                if !(b_n >= 2 as ::core::ffi::c_int
                    && Rand_div(b_n as uint32_t) as int32_t != 0 as int32_t)
                {
                    b_i = i;
                    b_c = c;
                }
            }
            i += 1;
        }
        if b_i >= 0 as ::core::ffi::c_int {
            x = false_x + ddx_ddd[b_i as usize] as ::core::ffi::c_int;
            y = false_y + ddy_ddd[b_i as usize] as ::core::ffi::c_int;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            false_y = y;
            false_x = x;
        }
        o += 1;
    }
    print_rel(
        '*' as i32,
        COLOUR_YELLOW as uint8_t,
        crate::bridge::st().flow.flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        crate::bridge::st().flow.flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
    );
    msg(b"Probable Flow Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_fear() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            p = crate::bridge::st().danger.fear_region[(y / 11 as ::core::ffi::c_int) as usize]
                [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            if !(p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int) {
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a = COLOUR_ORANGE as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
                    a = COLOUR_YELLOW as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 6 as ::core::ffi::c_int {
                    a = COLOUR_GREEN as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
                    a = COLOUR_BLUE as uint8_t;
                }
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Regional Fear.\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        (*Term).offset_y / crate::borg_cave_util::borg_panel_hgt(),
        (*Term).offset_x / crate::borg_cave_util::borg_panel_wid(),
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X {
            let mut a_0: uint8_t = COLOUR_BLUE as uint8_t;
            p = crate::bridge::st().danger.fear_monsters[y as usize][x as usize] as ::core::ffi::c_int;
            if !(p <= 0 as ::core::ffi::c_int) {
                if p == 1 as ::core::ffi::c_int {
                    a_0 = COLOUR_L_BLUE as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
                    && p > 1 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_BLUE as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                    && p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_GREEN as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int
                    && p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_YELLOW as uint8_t;
                }
                if p < crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    && p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_ORANGE as uint8_t;
                }
                if p > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a_0 = COLOUR_RED as uint8_t;
                }
                print_rel('*' as i32, a_0, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Monster Fear.\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        (*Term).offset_y / crate::borg_cave_util::borg_panel_hgt(),
        (*Term).offset_x / crate::borg_cave_util::borg_panel_wid(),
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_time() {
    msg(
        b"time: (%ld) \0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.time.now as ::core::ffi::c_long,
    );
    msg(
        b"; from town (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town) as ::core::ffi::c_long,
    );
    msg(
        b"; bounciness (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.antibounce_count as ::core::ffi::c_long,
    );
    msg(
        b"; when inviso (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.need_see_invis as ::core::ffi::c_long,
    );
}
unsafe extern "C" fn borg_cmd_grid_line_of_sight() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = crate::bridge::st().core.w_y;
    while y < crate::bridge::st().core.w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = crate::bridge::st().core.w_x;
        while x < crate::bridge::st().core.w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            if borg_los(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) {
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has Projectable to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    y = crate::bridge::st().core.w_y;
    while y < crate::bridge::st().core.w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = crate::bridge::st().core.w_x;
        while x < crate::bridge::st().core.w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a_0: uint8_t = COLOUR_YELLOW as uint8_t;
            if square_in_bounds(cave, loc(x, y)) {
                if borg_projectable_dark(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) {
                    print_rel('*' as i32, a_0, y, x);
                }
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has Projectable Dark to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    y = crate::bridge::st().core.w_y;
    while y < crate::bridge::st().core.w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = crate::bridge::st().core.w_x;
        while x < crate::bridge::st().core.w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a_1: uint8_t = COLOUR_GREEN as uint8_t;
            if borg_los(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y, x) {
                print_rel('*' as i32, a_1, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has LOS to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_power() {
    let mut p: int32_t = 0;
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    p = borg_power();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    msg(
        b"Current Borg Power %ld\0" as *const u8 as *const ::core::ffi::c_char,
        p,
    );
    msg(
        b"Current Home Power %ld\0" as *const u8 as *const ::core::ffi::c_char,
        borg_power_home(),
    );
}
unsafe extern "C" fn borg_cmd_prepare() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    i = 1 as ::core::ffi::c_int;
    while i <= 101 as ::core::ffi::c_int {
        if !(borg_prepared(i) as *mut ::core::ffi::c_char).is_null() {
            break;
        }
        i += 1;
    }
    msg(
        b"Max Level: %d  Prep'd For: %d  Reason: %s\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize],
        i - 1 as ::core::ffi::c_int,
        borg_prepared(i),
    );
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        msg(b"Unable to check for big fight from town.\0" as *const u8
            as *const ::core::ffi::c_char);
    } else if crate::bridge::st().borg.ready_morgoth == 1 as ::core::ffi::c_int {
        msg(b"You are ready for the big fight!!\0" as *const u8 as *const ::core::ffi::c_char);
    } else if crate::bridge::st().borg.ready_morgoth == 0 as ::core::ffi::c_int {
        msg(b"You are NOT ready for the big fight!!\0" as *const u8 as *const ::core::ffi::c_char);
    } else if crate::bridge::st().borg.ready_morgoth == -(1 as ::core::ffi::c_int) {
        msg(b"No readiness check done.\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn borg_cmd_stats() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        borg_note(format(
            b"stat # %s, is: cur %d, used %d\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char)
                .offset((BI_STR as ::core::ffi::c_int + i) as isize),
            crate::bridge::st().borg.trait_[((BI_CSTR as ::core::ffi::c_int + i)) as usize],
            crate::bridge::st().borg.trait_[((BI_STR as ::core::ffi::c_int + i)) as usize],
        ));
        i += 1;
    }
}
unsafe extern "C" fn borg_cmd_swaps() {
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    borg_power();
    borg_update();
    if weapon_swap != 0 {
        item =
            &raw mut crate::bridge::st().items.items[((weapon_swap - 1 as ::core::ffi::c_int)) as usize] as *mut borg_item;
        msg(
            b"Swap Weapon:  %s, value= %d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            weapon_swap_value,
        );
    } else {
        msg(b"Swap Weapon:  NONE\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if armour_swap != 0 {
        item =
            &raw mut crate::bridge::st().items.items[((armour_swap - 1 as ::core::ffi::c_int)) as usize] as *mut borg_item;
        msg(
            b"Swap Armour:  %s, value= %d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            armour_swap_value,
        );
    } else {
        msg(b"Swap Armour:  NONE\0" as *const u8 as *const ::core::ffi::c_char);
    };
}
unsafe extern "C" fn borg_cmd_spells() {
    let mut ii: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut cmd: ::core::ffi::c_char = 0;
    Term_save();
    if borg_can_cast() {
        let mut i: ::core::ffi::c_int = 0;
        let mut book: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < (*(*player).class).magic.total_spells {
            let mut as_0: *mut borg_magic = &raw mut crate::bridge::st().magic.magics[(i) as usize] as *mut borg_magic;
            let mut failpercent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if ((*as_0).level as ::core::ffi::c_int) < 99 as ::core::ffi::c_int {
                if book != (*as_0).book {
                    if book != -(1 as ::core::ffi::c_int) {
                        get_com(
                            format(
                                b"Exam spell book %d.  Press any key for next book.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*as_0).book,
                            ),
                            &raw mut cmd,
                        );
                    }
                    ii = 2 as ::core::ffi::c_int;
                    Term_clear();
                    let fresh0 = ii;
                    ii = ii + 1;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        fresh0,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"[ Spells ].\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    book = (*as_0).book;
                }
                let mut legal: *const ::core::ffi::c_char =
                    if borg_spell_legal((*as_0).spell_enum) as ::core::ffi::c_int != 0 {
                        b"legal\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"Not Legal \0" as *const u8 as *const ::core::ffi::c_char
                    };
                failpercent = borg_spell_fail_rate((*as_0).spell_enum);
                let fresh1 = ii;
                ii = ii + 1;
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    fresh1,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%s, %s, attempted %ld times, fail rate:%d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*as_0).name.as_ptr(),
                        legal,
                        (*as_0).times as ::core::ffi::c_long,
                        failpercent,
                    ),
                );
            }
            i += 1;
        }
        get_com(
            format(
                b"Exam spell book %d.  Press any key for next book.\0" as *const u8
                    as *const ::core::ffi::c_char,
                book,
            ),
            &raw mut cmd,
        );
    }
    Term_load();
}
unsafe extern "C" fn borg_cmd_has() {
    let mut item: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut cmd: ::core::ffi::c_char = 0;
    if !get_com(
        b"Dynamic Borg Has What: ((a)ny/(i)nv/(w)orn/a(r)tifact/(s)kill) \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut cmd,
    ) {
        return;
    }
    match cmd as ::core::ffi::c_int {
        97 | 65 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).k_max as ::core::ffi::c_int;
        }
        105 | 73 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int;
        }
        119 | 87 => {
            item = (*z_info).pack_size as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int;
        }
        114 | 82 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int;
        }
        _ => {
            item = 0 as ::core::ffi::c_int;
            to = BI_MAX as ::core::ffi::c_int;
        }
    }
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    while item < to {
        match cmd as ::core::ffi::c_int {
            97 | 65 => {
                if crate::bridge::st().borg.has[(item) as usize] != 0 {
                    borg_note(format(
                        b"Item-Kind:%03d name=%s value= %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        (*k_info.offset(item as isize)).name,
                        crate::bridge::st().borg.has[(item) as usize],
                    ));
                }
            }
            105 | 73 => {
                if (crate::bridge::st().items.items[(item) as usize]).iqty != 0 {
                    borg_note(format(
                        b"Item-Invn:%03d desc= %s qty %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        &raw mut (crate::bridge::st().items.items[(item) as usize]).desc
                            as *mut ::core::ffi::c_char,
                        (crate::bridge::st().items.items[(item) as usize]).iqty as ::core::ffi::c_int,
                    ));
                }
            }
            119 | 87 => {
                if (crate::bridge::st().items.items[(item) as usize]).iqty != 0 {
                    borg_note(format(
                        b"Item-Worn:%03d desc= %s qty %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        &raw mut (crate::bridge::st().items.items[(item) as usize]).desc
                            as *mut ::core::ffi::c_char,
                        (crate::bridge::st().items.items[(item) as usize]).iqty as ::core::ffi::c_int,
                    ));
                }
            }
            114 | 82 => {
                if (crate::bridge::st().items.items[(item) as usize]).iqty as ::core::ffi::c_int != 0
                    && (crate::bridge::st().items.items[(item) as usize]).art_idx as ::core::ffi::c_int != 0
                {
                    borg_note(format(
                        b"Item-Arti:%03d name= %s.\0" as *const u8 as *const ::core::ffi::c_char,
                        item,
                        (*a_info.offset((crate::bridge::st().items.items[(item) as usize]).art_idx as isize)).name,
                    ));
                }
            }
            _ => {
                borg_note(format(
                    b"trait %d (%s) value= %d.\0" as *const u8 as *const ::core::ffi::c_char,
                    item,
                    *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char)
                        .offset(item as isize),
                    crate::bridge::st().borg.trait_[(item) as usize],
                ));
            }
        }
        item += 1;
    }
    msg(b"Borg_has[] dump complete.  Examine Log. \0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn borg_cmd_dump() {
    borg_inventory();
    borg_write_map(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_version() {
    msg(
        b"Borg Version: %s\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::ENGINE_DATE.as_ptr() as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn borg_cmd_object_desc() {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item2: *mut object = ::core::ptr::null_mut::<object>();
    n = get_quantity(
        b"Which item?\0" as *const u8 as *const ::core::ffi::c_char,
        (*z_info).pack_size as ::core::ffi::c_int,
    );
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    borg_power();
    borg_update();
    Term_save();
    item2 = *(*(*player).upkeep).inven.offset(n as isize);
    borg_display_item(item2, n);
    msg(b"Borg believes: \0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    Term_load();
}
unsafe extern "C" fn borg_cmd_respawn() {
    let mut cmd: ::core::ffi::c_char = 0;
    get_com(
        b"Are you sure you want to Respawn this borg? (y or n)? \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut cmd,
    );
    if cmd as ::core::ffi::c_int == 'y' as i32 || cmd as ::core::ffi::c_int == 'Y' as i32 {
        borg_force_reincarnate();
        borg_cmd_start();
    }
}
unsafe extern "C" fn borg_cmd_grid_glyphs() {
    let mut glyph_check: ::core::ffi::c_int = 0;
    let mut a: uint8_t = COLOUR_RED as uint8_t;
    glyph_check = 0 as ::core::ffi::c_int;
    while glyph_check < crate::bridge::st().glyph.track.num as ::core::ffi::c_int {
        print_rel(
            '*' as i32,
            a,
            crate::bridge::st().glyph.track.y[glyph_check as usize],
            crate::bridge::st().glyph.track.x[glyph_check as usize],
        );
        msg(
            b"Borg has Glyph (%d)noted.\0" as *const u8 as *const ::core::ffi::c_char,
            glyph_check,
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        glyph_check += 1;
    }
}
unsafe extern "C" fn borg_cmd_map() {
    let mut option: ::core::ffi::c_char = 0;
    if !get_com(
        b"What to show ((a)voidances, (f)eatures, (g)lyphs, (m)onsters, (o)bjects): \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut option,
    ) {
        return;
    }
    match option as ::core::ffi::c_int {
        97 => {
            borg_cmd_avoidances();
            return;
        }
        102 => {
            borg_cmd_grid_feature();
            return;
        }
        103 => {
            borg_cmd_grid_glyphs();
            return;
        }
        109 => {
            borg_cmd_grid_monsters();
            return;
        }
        111 => {
            borg_cmd_grid_objects();
            return;
        }
        _ => {
            msg(b"That is not a legal option.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_borg() {
    let mut i: ::core::ffi::c_int = 0;
    let mut cmd: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let command_count: ::core::ffi::c_int = (::core::mem::size_of::<[borg_commands; 29]>() as usize)
        .wrapping_div(::core::mem::size_of::<borg_commands>() as usize)
        as ::core::ffi::c_int;
    crate::bridge::st().borg.c = (*player).grid;
    if !get_com(
        b"Borg command (? for help): \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut cmd,
    ) {
        Term_load();
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < command_count {
        if borg_commands[i as usize].selection as ::core::ffi::c_int == cmd as ::core::ffi::c_int {
            if !crate::bridge::st().init.initialized || crate::bridge::st().init.game_closed as ::core::ffi::c_int != 0 {
                if crate::bridge::st().init.initialized {
                    borg_free();
                }
                borg_init();
                if crate::bridge::st().init.init_failure {
                    crate::bridge::st().init.initialized = false_0 != 0;
                    borg_free();
                    borg_note(
                        b"** startup failure borg cannot run ** \0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    Term_fresh();
                    return;
                }
            }
            borg_commands[i as usize]
                .function
                .expect("non-null function pointer")();
            return;
        }
        i += 1;
    }
    msg(b"That is not a legal Borg command.\0" as *const u8 as *const ::core::ffi::c_char);
}
