pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_cave_util_h::{
    borg_cave_floor_bold, borg_cave_floor_grid, borg_panel_hgt, borg_panel_wid,
};

use crate::headers::borg_danger_h::{
    borg_danger, borg_danger_one_kill, borg_fear_monsters, borg_fear_region,
};
use crate::headers::borg_escape_h::{borg_caution_teleport, borg_surrounded};
pub use crate::headers::borg_fight_attack_h::{
    borg_launch_bolt, borg_tp_other_index, borg_tp_other_n, borg_tp_other_x, borg_tp_other_y,
    successful_target, C2RustUnnamedH_32, BORG_ATTACK_ACID, BORG_ATTACK_ARROW,
    BORG_ATTACK_AWAY_ALL, BORG_ATTACK_AWAY_ALL_MORGOTH, BORG_ATTACK_AWAY_EVIL,
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
use crate::headers::borg_flow_glyph_h::{borg_needs_new_sea, track_glyph};
pub use crate::headers::borg_flow_h::{
    borg_temp_n, borg_temp_x, borg_temp_y, borg_track, track_step,
};
pub use crate::headers::borg_flow_kill_h::{
    borg_delete_kill, borg_kill, borg_kills, borg_kills_cnt, borg_kills_nxt, borg_nasties,
    borg_nasties_count, borg_nasties_limit, borg_nasties_num, borg_race_name,
};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_cfg, borg_struct, borg_time, w_x, w_y, C2RustUnnamedH_10,
    C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15,
    C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE,
    BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL,
    BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS,
    BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START,
    BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS,
    BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM,
    BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS,
    BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD,
    BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED,
};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{
    act_banishment, act_berserker, act_blessing, act_blessing2, act_blessing3, act_destruction2,
    act_detect_evil, act_detect_invis, act_earthquakes, act_glyph, act_haste, act_haste1,
    act_haste2, act_loskill, act_protevil, act_rage_bless_resist, act_resist_acid, act_resist_all,
    act_resist_cold, act_resist_elec, act_resist_fire, act_resist_pois, act_ring_acid,
    act_ring_flames, act_ring_ice, act_ring_lightning, act_shero, act_shroom_stone, act_tele_other,
    act_tmd_esp, act_tmd_sinvis,
};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_activate_ring, borg_aim_wand, borg_eat, borg_equips_item,
    borg_equips_ring, borg_equips_rod, borg_equips_staff_fail, borg_quaff_potion, borg_read_scroll,
    borg_use_staff, borg_zap_rod,
};
use crate::headers::borg_item_val_h::{
    kv_mush_stoneskin, sv_mush_stoneskin, sv_potion_berserk, sv_potion_detect_invis,
    sv_potion_heroism, sv_potion_resist_cold, sv_potion_resist_heat, sv_potion_resist_pois,
    sv_potion_speed, sv_ring_acid, sv_ring_flames, sv_ring_ice, sv_ring_lightning, sv_rod_speed,
    sv_scroll_banishment, sv_scroll_blessing, sv_scroll_detect_invis, sv_scroll_holy_chant,
    sv_scroll_holy_prayer, sv_scroll_mass_banishment, sv_scroll_protection_from_evil,
    sv_scroll_rune_of_protection, sv_staff_banishment, sv_staff_destruction, sv_staff_detect_evil,
    sv_staff_detect_invis, sv_staff_speed, sv_wand_teleport_away,
};
use crate::headers::borg_light_h::borg_light_beam;
pub use crate::headers::borg_magic_h::{
    borg_heroism_level, borg_spell, borg_spell_fail, borg_spell_okay_fail, borg_spells, ACID_SPRAY,
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
use crate::headers::borg_projection_h::{
    borg_inc_motion, borg_los, borg_projectable, borg_projectable_dark, borg_projectable_pure,
    borg_target, borg_target_loc,
};
pub use crate::headers::borg_trait_h::{
    borg_attacking, borg_create_door, borg_game_ratio, borg_on_glyph, borg_simulate,
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
    chunk, connector, distance, heatmap, monster_group, square, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::internal::__SCHAR_MAX__;

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
pub use crate::headers::list_player_timed_h::{
    TMD_AFRAID, TMD_AMNESIA, TMD_ATT_ACID, TMD_ATT_COLD, TMD_ATT_CONF, TMD_ATT_DEMON, TMD_ATT_ELEC,
    TMD_ATT_EVIL, TMD_ATT_FIRE, TMD_ATT_POIS, TMD_ATT_RUN, TMD_ATT_VAMP, TMD_BLACKBREATH,
    TMD_BLESSED, TMD_BLIND, TMD_BLOODLUST, TMD_BOLD, TMD_COMMAND, TMD_CONFUSED, TMD_COVERTRACKS,
    TMD_CUT, TMD_FAST, TMD_FASTCAST, TMD_FOOD, TMD_FREE_ACT, TMD_HEAL, TMD_HERO, TMD_IMAGE,
    TMD_INVULN, TMD_OPP_ACID, TMD_OPP_COLD, TMD_OPP_CONF, TMD_OPP_ELEC, TMD_OPP_FIRE, TMD_OPP_POIS,
    TMD_PARALYZED, TMD_POISONED, TMD_POWERSHOT, TMD_PROTEVIL, TMD_SCRAMBLE, TMD_SHERO, TMD_SHIELD,
    TMD_SINFRA, TMD_SINVIS, TMD_SLOW, TMD_SPRINT, TMD_STEALTH, TMD_STONESKIN, TMD_STUN, TMD_TAUNT,
    TMD_TELEPATHY, TMD_TERROR, TMD_TRAPSAFE,
};
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
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
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
pub use crate::headers::player_timed_h::{C2RustUnnamedH_22, TMD_MAX};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::strcmp;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{term, term_win, Term};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
pub const BD_LIGHT_MORGOTH: C2RustUnnamed_14 = 30;
pub const BD_BANISHMENT_MORGOTH: C2RustUnnamed_14 = 29;
pub const BD_TELE_AWAY_MORGOTH: C2RustUnnamed_14 = 28;
pub const BD_REST: C2RustUnnamed_14 = 27;
pub const BD_LIGHT_BEAM: C2RustUnnamed_14 = 26;
pub const BD_DETECT_INVISO: C2RustUnnamed_14 = 25;
pub const BD_BANISHMENT: C2RustUnnamed_14 = 24;
pub const BD_DESTRUCTION: C2RustUnnamed_14 = 22;
pub const BD_TPORTLEVEL: C2RustUnnamed_14 = 23;
pub const BD_EARTHQUAKE: C2RustUnnamed_14 = 21;
pub const BD_GENOCIDE_NASTIES: C2RustUnnamed_14 = 20;
pub const BD_GENOCIDE: C2RustUnnamed_14 = 19;
pub const BD_MASS_GENOCIDE: C2RustUnnamed_14 = 18;
pub const BD_CREATE_DOOR: C2RustUnnamed_14 = 17;
pub const BD_GLYPH: C2RustUnnamed_14 = 16;
pub const BD_TELE_AWAY: C2RustUnnamed_14 = 11;
pub const BD_SHIELD: C2RustUnnamed_14 = 10;
pub const BD_REGEN: C2RustUnnamed_14 = 15;
pub const BD_SMITE_EVIL: C2RustUnnamed_14 = 14;
pub const BD_BERSERK: C2RustUnnamed_14 = 13;
pub const BD_HERO: C2RustUnnamed_14 = 12;
pub const BD_BLESS: C2RustUnnamed_14 = 0;
pub const BD_RESIST_P: C2RustUnnamed_14 = 8;
pub const BD_RESIST_E: C2RustUnnamed_14 = 7;
pub const BD_RESIST_A: C2RustUnnamed_14 = 6;
pub const BD_RESIST_C: C2RustUnnamed_14 = 5;
pub const BD_RESIST_F: C2RustUnnamed_14 = 4;
pub const BD_RESIST_FECAP: C2RustUnnamed_14 = 3;
pub const BD_GRIM_PURPOSE: C2RustUnnamed_14 = 2;
pub const BD_PROT_FROM_EVIL: C2RustUnnamed_14 = 9;
pub const BD_SPEED: C2RustUnnamed_14 = 1;
pub const BD_MAX: C2RustUnnamed_14 = 31;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
#[no_mangle]
pub static mut borg_attempting_refresh_resist: bool = false_0 != 0;
unsafe extern "C" fn borg_log_spellpath() {
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    y = borg_target_loc.y;
    x = borg_target_loc.x;
    n_x = borg.c.x;
    n_y = borg.c.y;
    loop {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(n_y as isize))
            .offset(n_x as isize) as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        if !borg_cave_floor_grid(ag) {
            borg_note(format(
                b"# Logging Spell pathway (%d,%d): Wall grid.\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
            ));
            break;
        } else {
            if (*ag).kill != 0 {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d): %s, danger %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                    borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                    borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        (*ag).kill as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ),
                ));
            } else if n_y == borg.c.y && n_x == borg.c.x {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d): My grid.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                ));
            } else {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                ));
            }
            if n_x == x && n_y == y {
                break;
            }
            dist += 1;
            if dist >= (*z_info).max_range as ::core::ffi::c_int {
                break;
            }
            borg_inc_motion(&raw mut n_y, &raw mut n_x, borg.c.y, borg.c.x, y, x);
        }
    }
}
unsafe extern "C" fn borg_defend_aux_bless(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut i: ::core::ffi::c_int = 0;
    let mut borg_near_kill: bool = false_0 != 0;
    if borg.temp.bless {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BLESS, fail_allowed)
        && !borg_equips_item(act_blessing, true_0 != 0)
        && !borg_equips_item(act_blessing2, true_0 != 0)
        && !borg_equips_item(act_blessing3, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_blessing)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_holy_chant)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_holy_prayer)
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 5 as borg_time) {
                if !(distance(borg.c, (*kill).pos) > 3 as ::core::ffi::c_int) {
                    borg_near_kill = true_0 != 0;
                }
            }
        }
        i += 1;
    }
    if (p1 > borg.avoidance as ::core::ffi::c_int / 12 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            <= 15 as ::core::ffi::c_int)
        && p1 > 0 as ::core::ffi::c_int
        && borg_near_kill as ::core::ffi::c_int != 0
        && p1 < borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast Bless\0" as *const u8 as *const ::core::ffi::c_char);
        borg.no_rest_prep = 11000 as int16_t;
        if borg_spell(BLESS) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing2) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing3) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_blessing) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_holy_chant) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_holy_prayer) as ::core::ffi::c_int != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_speed(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut good_speed: bool = false_0 != 0;
    let mut speed_spell: bool = false_0 != 0;
    let mut speed_staff: bool = false_0 != 0;
    let mut speed_rod: bool = false_0 != 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    if borg.temp.fast {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(HASTE_SELF, fail_allowed) {
        speed_spell = true_0 != 0;
    }
    if borg_equips_staff_fail(sv_staff_speed) {
        speed_staff = true_0 != 0;
    }
    if borg_equips_rod(sv_rod_speed) {
        speed_rod = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int > borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_speed)
        && !speed_staff
        && !speed_rod
        && !speed_spell
        && !borg_equips_item(act_haste, true_0 != 0)
        && !borg_equips_item(act_haste1, true_0 != 0)
        && !borg_equips_item(act_haste2, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if speed_rod as ::core::ffi::c_int != 0
        || speed_spell as ::core::ffi::c_int != 0
        || speed_staff as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste1, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste2, true_0 != 0) as ::core::ffi::c_int != 0
    {
        good_speed = true_0 != 0;
    }
    borg.temp.fast = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg.temp.fast = false_0 != 0;
    if borg.mon.scary {
        p2 = p2 * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if good_speed as ::core::ffi::c_int != 0 && borg.near.unique as ::core::ffi::c_int != 0 {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if borg.near.summoner as ::core::ffi::c_int != 0 && borg.near.unique as ::core::ffi::c_int != 0
    {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 99 as ::core::ffi::c_int
        && borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int != 0
        && (strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"Bullroarer the Hobbit\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Mughash the Kobold Lord\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Wormtongue, Agent of Saruman\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Lagduf, the Snaga\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Brodda, the Easterling\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Orfax, Son of Boldor\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0)
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
    {
        p2 = p2 * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 97 as ::core::ffi::c_int
        && borg.near.unique == 0
        && !good_speed
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int
        && good_speed as ::core::ffi::c_int != 0
        || p1 > p2
            && p2
                <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                    borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 3 as ::core::ffi::c_int
                } else {
                    borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                })
            && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast Speed\0" as *const u8 as *const ::core::ffi::c_char);
        borg.no_rest_prep = (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 1000 as ::core::ffi::c_int) as int16_t;
        if borg_zap_rod(sv_rod_speed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste2) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_speed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_speed) as ::core::ffi::c_int != 0
        {
            return p1 - p2;
        }
        if borg_spell_fail(HASTE_SELF, fail_allowed) {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_grim_purpose(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_conf: bool = *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0;
    let mut save_fa: bool = *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0;
    if save_conf as ::core::ffi::c_int != 0 && save_fa as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(GRIM_PURPOSE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = true_0;
    *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = true_0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = save_conf as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = save_fa as ::core::ffi::c_int;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2 + 2 as ::core::ffi::c_int;
        }
        borg_note(
            b"# Attempting to cast Grim Purpose\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if borg_spell(GRIM_PURPOSE) {
            borg.no_rest_prep = 13000 as int16_t;
        }
        return p1 - p2 + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_fecap(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save_fire: bool = false_0 != 0;
    let mut save_acid: bool = false_0 != 0;
    let mut save_poison: bool = false_0 != 0;
    let mut save_elec: bool = false_0 != 0;
    let mut save_cold: bool = false_0 != 0;
    if borg.temp.res_fire as ::core::ffi::c_int != 0
        && borg.temp.res_acid as ::core::ffi::c_int != 0
        && borg.temp.res_pois as ::core::ffi::c_int != 0
        && borg.temp.res_elec as ::core::ffi::c_int != 0
        && borg.temp.res_cold as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_fire = borg.temp.res_fire;
    save_elec = borg.temp.res_elec;
    save_cold = borg.temp.res_cold;
    save_acid = borg.temp.res_acid;
    save_poison = borg.temp.res_pois;
    borg.temp.res_fire = true_0 != 0;
    borg.temp.res_elec = true_0 != 0;
    borg.temp.res_cold = true_0 != 0;
    borg.temp.res_acid = true_0 != 0;
    borg.temp.res_pois = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = save_fire;
    borg.temp.res_elec = save_elec;
    borg.temp.res_cold = save_cold;
    borg.temp.res_acid = save_acid;
    borg.temp.res_pois = save_poison;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 45 as ::core::ffi::c_int
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2 + 2 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast FECAP\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2 + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_f(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_fire: bool = false_0 != 0;
    save_fire = borg.temp.res_fire;
    if borg.temp.res_fire {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_resist_fire, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_flames)
        && !borg_equips_item(act_ring_flames, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_heat)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = save_fire;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RFire\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_ring(sv_ring_flames) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_flames) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_fire) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_spell_fail(RESISTANCE, fail_allowed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_heat) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_c(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_cold: bool = false_0 != 0;
    if borg.temp.res_cold {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_item(act_resist_cold, true_0 != 0)
        && !borg_equips_ring(sv_ring_ice)
        && !borg_equips_item(act_ring_ice, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_cold)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_cold = borg.temp.res_cold;
    borg.temp.res_cold = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_cold = save_cold;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RCold\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_ring(sv_ring_ice) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_ice) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_cold) as ::core::ffi::c_int != 0
            || borg_spell_fail(RESISTANCE, fail_allowed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_cold) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_a(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_acid: bool = false_0 != 0;
    if borg.temp.res_acid {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_acid, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_acid)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_acid = borg.temp.res_acid;
    borg.temp.res_acid = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_acid = save_acid;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RAcid\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell(RESISTANCE) {
            return p1 - p2;
        }
        if borg_activate_ring(sv_ring_acid) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_acid) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_acid) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_e(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_elec: bool = false_0 != 0;
    if borg.temp.res_elec {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_elec, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_lightning)
        && !borg_equips_item(act_ring_lightning, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_elec = borg.temp.res_elec;
    borg.temp.res_elec = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_elec = save_elec;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RAcid\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell(RESISTANCE) {
            return p1 - p2;
        }
        if borg_activate_ring(sv_ring_lightning) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_lightning) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_elec) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_p(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_poison: bool = false_0 != 0;
    if borg.temp.res_pois {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESIST_POISON, fail_allowed)
        && !borg_equips_item(act_resist_pois, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_pois)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_poison = borg.temp.res_pois;
    borg.temp.res_pois = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_pois = save_poison;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RPois\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell_fail(RESIST_POISON, fail_allowed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_pois) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_pois) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_prot_evil(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut pfe_spell: bool = false_0 != 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    if borg.temp.prot_from_evil {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        pfe_spell = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int
        <= borg_slot(
            TV_SCROLL as ::core::ffi::c_int,
            sv_scroll_protection_from_evil,
        )
    {
        pfe_spell = true_0 != 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        pfe_spell = false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        pfe_spell = false_0 != 0;
    }
    if borg_equips_item(act_protevil, true_0 != 0) {
        pfe_spell = true_0 != 0;
    }
    if pfe_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.prot_from_evil = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.prot_from_evil = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
        || *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast PFE\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell_fail(PROTECTION_FROM_EVIL, fail_allowed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_protevil) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_protection_from_evil) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 1000 as ::core::ffi::c_int) as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_shield(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if borg.temp.shield {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if *borg.has.offset(kv_mush_stoneskin as isize) <= 0 as ::core::ffi::c_int
        && !borg_equips_item(act_shroom_stone, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    borg.temp.shield = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg.temp.shield = false_0 != 0;
    if borg.near.unique != 0 {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to eat a stone skin\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_stoneskin) as ::core::ffi::c_int != 0
            || borg_activate_item(act_shroom_stone) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 2000 as int16_t;
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_tele_away(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = p1;
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut spell_ok: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
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
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        if p1
            < borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                > 30 as ::core::ffi::c_int
            && borg_simulate as ::core::ffi::c_int != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if p1
        < borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    spell_ok = false_0 != 0;
    if p1 > borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
    {
        fail_allowed += 5 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(TELEPORT_OTHER, fail_allowed) as ::core::ffi::c_int != 0
        || borg_equips_item(act_tele_other, true_0 != 0) as ::core::ffi::c_int != 0
        || -(1 as ::core::ffi::c_int)
            != borg_slot(TV_WAND as ::core::ffi::c_int, sv_wand_teleport_away)
            && (*borg_items
                .offset(borg_slot(TV_WAND as ::core::ffi::c_int, sv_wand_teleport_away) as isize))
            .pval as ::core::ffi::c_int
                != 0
    {
        spell_ok = true_0 != 0;
    }
    if !spell_ok {
        return 0 as ::core::ffi::c_int;
    }
    if borg_surrounded() as ::core::ffi::c_int == true_0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                    if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                        if !((*ag).feat as ::core::ffi::c_int >= FEAT_RUBBLE as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int)
                        {
                            if !(distance(borg.c, (*kill).pos)
                                > (*z_info).max_range as ::core::ffi::c_int)
                            {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        p1,
        BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if p2 <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        borg_temp_n = 0 as int16_t;
        borg_tp_other_n = 0 as int16_t;
        if p2 != 0 && p2 > borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
            return p2;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    }
    borg_log_spellpath();
    i = 0 as ::core::ffi::c_int;
    while i < borg_tp_other_n as ::core::ffi::c_int {
        borg_note(format(
            b"# T.O. %d, index %d (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_tp_other_n as ::core::ffi::c_int,
            borg_tp_other_index[i as usize],
            borg_tp_other_y[i as usize] as ::core::ffi::c_int,
            borg_tp_other_x[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    if borg_spell(TELEPORT_OTHER) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tele_other) as ::core::ffi::c_int != 0
        || borg_aim_wand(sv_wand_teleport_away) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
        return p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_hero(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut potion: bool = false;
    let mut spell: bool = false;
    if borg.temp.hero {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    spell = borg_spell_okay_fail(HEROISM, fail_allowed) as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= borg_heroism_level();
    potion =
        -(1 as ::core::ffi::c_int) != borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_heroism);
    if !potion && !spell {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast Hero\0" as *const u8 as *const ::core::ffi::c_char);
        if spell as ::core::ffi::c_int != 0 && borg_spell(HEROISM) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_heroism) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 10000 as int16_t;
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_regen(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.regen {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) < 100 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RAPID_REGENERATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        if borg_spell(RAPID_REGENERATION) {
            borg.no_rest_prep = 10000 as int16_t;
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_berserk(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.berserk {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BERSERK_STRENGTH, fail_allowed)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_berserk)
        && !borg_equips_item(act_berserker, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_item(act_shero, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 5 as ::core::ffi::c_int;
        }
        if borg_spell(BERSERK_STRENGTH) as ::core::ffi::c_int != 0
            || borg_activate_item(act_berserker) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_activate_item(act_shero) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_berserk) as ::core::ffi::c_int != 0
        {
            return 5 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn near_evil() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                        .offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !(distance(borg.c, (*kill).pos) > 3 as ::core::ffi::c_int) {
                            if flag_has(
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
                                RF_EVIL as ::core::ffi::c_int,
                            ) {
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
unsafe extern "C" fn borg_defend_aux_smite_evil(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.smite_evil as ::core::ffi::c_int != 0
        || *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SMITE_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if !near_evil() {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 5 as ::core::ffi::c_int;
        }
        if borg_spell(SMITE_EVIL) {
            return 5 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_glyph(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut glyph_spell: bool = false_0 != 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
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
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).take as ::core::ffi::c_int != 0
        || (*ag).trap as ::core::ffi::c_int != 0
        || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 20 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(GLYPH_OF_WARDING, fail_allowed) {
        glyph_spell = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int
        <= borg_slot(
            TV_SCROLL as ::core::ffi::c_int,
            sv_scroll_rune_of_protection,
        )
    {
        glyph_spell = true_0 != 0;
    }
    if borg_equips_item(act_glyph, true_0 != 0) {
        glyph_spell = true_0 != 0;
    }
    if (*borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0)
        && glyph_spell as ::core::ffi::c_int != 0
    {
        glyph_spell = false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        glyph_spell = false_0 != 0;
    }
    if !glyph_spell {
        return 0 as ::core::ffi::c_int;
    }
    borg_on_glyph = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_on_glyph = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_spell_fail(GLYPH_OF_WARDING, fail_allowed) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_rune_of_protection) as ::core::ffi::c_int != 0
            || borg_activate_item(act_glyph) as ::core::ffi::c_int != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < track_glyph.num as ::core::ffi::c_int {
                if *track_glyph.x.offset(i as isize) == borg.c.x
                    && *track_glyph.y.offset(i as isize) == borg.c.y
                {
                    return p1 - p2;
                }
                i += 1;
            }
            if (track_glyph.num as ::core::ffi::c_int) < track_glyph.size as ::core::ffi::c_int {
                borg_note(
                    b"# Noting the creation of a glyph.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                *track_glyph.x.offset(track_glyph.num as isize) = borg.c.x;
                *track_glyph.y.offset(track_glyph.num as isize) = borg.c.y;
                track_glyph.num += 1;
            }
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_create_door(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut door_bad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.summoner {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 20 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(DOOR_CREATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    door_x = -(1 as ::core::ffi::c_int);
    while door_x <= 1 as ::core::ffi::c_int {
        door_y = -(1 as ::core::ffi::c_int);
        while door_y <= 1 as ::core::ffi::c_int {
            x = door_x + borg.c.x;
            y = door_y + borg.c.y;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if (*ag).glyph as ::core::ffi::c_int != 0
                || (*ag).kill as ::core::ffi::c_int != 0
                || ((*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int
                    || (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                    || (*ag).feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int)
            {
                door_bad += 1;
            }
            if (*ag).take as ::core::ffi::c_int != 0
                || (*ag).trap as ::core::ffi::c_int != 0
                || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
                || (*ag).kill as ::core::ffi::c_int != 0
            {
                door_bad += 1;
            }
            door_y += 1;
        }
        door_x += 1;
    }
    if door_bad >= 6 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    borg_create_door = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_create_door = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_spell_fail(DOOR_CREATION, fail_allowed) {
            borg.near.breeder = true_0 != 0;
            borg_needs_new_sea = true_0 != 0;
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_mass_genocide(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0;
    let mut b_p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(MASS_BANISHMENT, 40 as ::core::ffi::c_int)
        && !borg_equips_item(act_banishment, true_0 != 0)
        && *borg
            .trait_0
            .offset(BI_AMASSBAN as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(distance(borg.c, (*kill).pos) > 20 as ::core::ffi::c_int) {
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
                        p = borg_danger_one_kill(
                            borg.c.y,
                            borg.c.x,
                            1 as ::core::ffi::c_int,
                            i,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        b_p = b_p + p;
                        hit = hit + 3 as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    p2 = p1 - b_p;
    if p2 < 0 as ::core::ffi::c_int {
        p2 = 0 as ::core::ffi::c_int;
    }
    if hit * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        return 0 as ::core::ffi::c_int;
    }
    p2 = p2 + hit;
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && hit / 3 as ::core::ffi::c_int > 8 as ::core::ffi::c_int
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_read_scroll(sv_scroll_mass_banishment) as ::core::ffi::c_int != 0
            || borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
            || borg_spell(MASS_BANISHMENT) as ::core::ffi::c_int != 0
        {
            i = 1 as ::core::ffi::c_int;
            while i < borg_kills_nxt as ::core::ffi::c_int {
                let mut tmp_kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
                let mut tmp_r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
                tmp_kill = borg_kills.offset(i as isize) as *mut borg_kill;
                if !((*tmp_kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                    if !((*tmp_kill).r_idx as ::core::ffi::c_int
                        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    {
                        tmp_r_ptr = r_info.offset((*tmp_kill).r_idx as isize) as *mut monster_race;
                        if !flag_has(
                            &raw mut (*tmp_r_ptr).flags as *mut bitflag,
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
                            borg_delete_kill(i);
                        }
                    }
                }
                i += 1;
            }
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_genocide(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p_threat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p_without_kill: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_threat: ::core::ffi::c_int = 0;
    let mut kill_char: ::core::ffi::c_uchar = 0;
    let mut b_kill_count: [::core::ffi::c_int; 255] = [0; 255];
    let mut b_danger: [::core::ffi::c_int; 255] = [0; 255];
    let mut b_threat: [::core::ffi::c_int; 255] = [0; 255];
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut total_danger_to_me: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp_genocide_target: ::core::ffi::c_char =
        0 as ::core::ffi::c_int as ::core::ffi::c_char;
    let mut biggest_threat: ::core::ffi::c_uchar = 0;
    let mut biggest_danger: ::core::ffi::c_uchar = 0;
    let mut genocide_spell: bool = false_0 != 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
        [(borg.c.x / 11 as ::core::ffi::c_int) as usize]
        != 0
    {
        p1 -= borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    }
    if borg_fear_monsters[borg.c.y as usize][borg.c.x as usize] != 0 {
        p1 -= borg_fear_monsters[borg.c.y as usize][borg.c.x as usize] as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(BANISHMENT, fail_allowed) as ::core::ffi::c_int != 0
        || borg_equips_item(act_banishment, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_staff_fail(sv_staff_banishment) as ::core::ffi::c_int != 0
        || -(1 as ::core::ffi::c_int)
            != borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_banishment)
    {
        genocide_spell = true_0 != 0;
    }
    if genocide_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) <= 75 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    biggest_danger = 0 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_uchar;
    biggest_threat = 0 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_uchar;
    i = 0 as ::core::ffi::c_int;
    while i < UCHAR_MAX {
        b_danger[i as usize] = 0 as ::core::ffi::c_int;
        b_kill_count[i as usize] = 0 as ::core::ffi::c_int;
        b_threat[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    max_threat = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
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
                    p = borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    p_threat = borg_danger_one_kill(
                        (*kill).pos.y,
                        (*kill).pos.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    kill_char = (*r_ptr).d_char as ::core::ffi::c_uchar;
                    total_danger_to_me += p;
                    b_danger[kill_char as usize] = b_danger[kill_char as usize] + p;
                    b_threat[kill_char as usize] = b_threat[kill_char as usize] + p_threat;
                    b_kill_count[kill_char as usize] += 1;
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < UCHAR_MAX {
        if !(b_kill_count[i as usize] == 0 as ::core::ffi::c_int) {
            if b_danger[i as usize] > max_threat {
                max_threat = b_danger[i as usize];
                biggest_danger = i as ::core::ffi::c_uchar;
                p_without_kill = total_danger_to_me - b_danger[biggest_danger as usize];
            }
            if b_threat[i as usize] > max_threat {
                max_threat = b_threat[i as usize];
                biggest_threat = i as ::core::ffi::c_uchar;
            }
            if !borg_simulate {
                borg_note(format(
                    b"# Race '%c' is a threat with total danger %d from %d individuals.\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    i,
                    b_threat[i as usize],
                    b_kill_count[i as usize],
                ));
            }
        }
        i += 1;
    }
    if biggest_threat != 0 {
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 375 as ::core::ffi::c_int
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_threat[biggest_threat as usize]
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_kill_count[biggest_threat as usize]
            * 4 as ::core::ffi::c_int
            * 12 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
            >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_kill_count[biggest_threat as usize] >= 75 as ::core::ffi::c_int {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if biggest_threat != 0 {
            borg_note(format(
                b"# Race '%c' is a real threat with total danger %d from %d individuals.\0"
                    as *const u8 as *const ::core::ffi::c_char,
                biggest_threat as ::core::ffi::c_int,
                b_threat[biggest_threat as usize],
                b_kill_count[biggest_threat as usize],
            ));
        }
        tmp_genocide_target = biggest_threat as ::core::ffi::c_char;
    }
    if biggest_danger != 0 {
        if b_kill_count[biggest_danger as usize]
            * 4 as ::core::ffi::c_int
            * 12 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
            >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if p1
            < borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                > 75 as ::core::ffi::c_int
                && p1
                    < borg.avoidance as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int
        {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if p_without_kill <= borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if biggest_danger != 0 {
            tmp_genocide_target = biggest_danger as ::core::ffi::c_char;
        }
    }
    if tmp_genocide_target != 0 {
        if borg_simulate {
            if biggest_danger != 0 {
                return p1 - p_without_kill;
            }
            if biggest_threat != 0 {
                return b_threat[biggest_threat as usize];
            }
        }
        if biggest_danger != 0 {
            borg_note(format(
                b"# Banishing race '%c' (qty:%d).  Danger after spell:%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp_genocide_target as ::core::ffi::c_int,
                b_kill_count[biggest_danger as usize],
                p_without_kill,
            ));
        }
        if biggest_threat != 0 {
            borg_note(format(
                b"# Banishing race '%c' (qty:%d).  Danger from them:%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp_genocide_target as ::core::ffi::c_int,
                b_kill_count[biggest_threat as usize],
                b_threat[biggest_threat as usize],
            ));
        }
        if borg_read_scroll(sv_scroll_banishment) as ::core::ffi::c_int != 0
            || borg_spell(BANISHMENT) as ::core::ffi::c_int != 0
            || borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_banishment) as ::core::ffi::c_int != 0
        {
            borg_keypress(tmp_genocide_target as keycode_t);
            i = 1 as ::core::ffi::c_int;
            while i < borg_kills_nxt as ::core::ffi::c_int {
                kill = borg_kills.offset(i as isize) as *mut borg_kill;
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                if !((*r_ptr).d_char != tmp_genocide_target as wchar_t) {
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
                        borg_delete_kill(i);
                    }
                }
                i += 1;
            }
        }
        if biggest_danger != 0 {
            return p1 - p_without_kill;
        }
        if biggest_threat != 0 {
            return b_threat[biggest_threat as usize];
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_genocide_nasties(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut genocide_spell: bool = false_0 != 0;
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 7 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 250 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        < 25 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(BANISHMENT, 35 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_banishment, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_staff_fail(sv_staff_banishment) as ::core::ffi::c_int != 0
    {
        genocide_spell = true_0 != 0;
    }
    if genocide_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_nasties_num as ::core::ffi::c_int {
        if borg_nasties_count[i as usize] as ::core::ffi::c_int
            >= borg_nasties_limit[i as usize] as ::core::ffi::c_int
        {
            b_i = i;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Banishing nasties '%c' (qty:%d).\0" as *const u8 as *const ::core::ffi::c_char,
        borg_nasties[b_i as usize] as ::core::ffi::c_int,
        borg_nasties_count[b_i as usize] as ::core::ffi::c_int,
    ));
    if borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_banishment) as ::core::ffi::c_int != 0
        || borg_spell(BANISHMENT) as ::core::ffi::c_int != 0
    {
        borg_keypress(borg_nasties[b_i as usize] as keycode_t);
        borg_nasties_count[b_i as usize] = 0 as uint8_t;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
            let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
            if !((*r_ptr).d_char != borg_nasties[b_i as usize] as wchar_t) {
                borg_delete_kill(i);
            }
            i += 1;
        }
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_earthquake(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 9999 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut threat_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_simulate {
        borg_needs_new_sea = true_0 != 0;
        if borg_spell(TREMOR) {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
        } else if borg_spell(QUAKE) as ::core::ffi::c_int != 0
            || borg_spell(GRONDS_BLOW) as ::core::ffi::c_int != 0
            || borg_activate_item(act_earthquakes) as ::core::ffi::c_int != 0
        {
            return p2;
        }
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(TREMOR, 35 as ::core::ffi::c_int)
        && !borg_spell_okay_fail(QUAKE, 35 as ::core::ffi::c_int)
        && !borg_spell_okay_fail(GRONDS_BLOW, 35 as ::core::ffi::c_int)
        && !borg_equips_item(act_earthquakes, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && !borg.near.summoner
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if borg_los(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) as ::core::ffi::c_int != 0
            && (*kill).ranged_attack as ::core::ffi::c_int != 0
            && distance((*kill).pos, borg.c) >= 2 as ::core::ffi::c_int
        {
            threat_count += 1;
        }
        i += 1;
    }
    if threat_count >= 4 as ::core::ffi::c_int
        && p1
            > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        p2 = p1 / 3 as ::core::ffi::c_int;
    }
    if threat_count == 3 as ::core::ffi::c_int
        && p1
            > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        p2 = p1 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_destruction(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell: bool = false_0 != 0;
    let mut real_danger: bool = false_0 != 0;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_simulate {
        if borg_spell(WORD_OF_DESTRUCTION) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_destruction) as ::core::ffi::c_int != 0
            || borg_activate_item(act_destruction2) as ::core::ffi::c_int != 0
        {
            borg_needs_new_sea = true_0 != 0;
        }
        return 500 as ::core::ffi::c_int;
    }
    if borg.morgoth_position {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        real_danger = true_0 != 0;
    }
    if p1
        > borg.avoidance as ::core::ffi::c_int * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 90 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            <= 300 as ::core::ffi::c_int
    {
        real_danger = true_0 != 0;
    }
    if real_danger as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg
        .trait_0
        .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize)
            != 0)
        && *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
        && borg.near.unique as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= 275 as ::core::ffi::c_int
    {
        if borg_caution_teleport(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *borg
        .trait_0
        .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= 275 as ::core::ffi::c_int
    {
        if borg_caution_teleport(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if borg_spell_okay_fail(WORD_OF_DESTRUCTION, 55 as ::core::ffi::c_int) as ::core::ffi::c_int
        != 0
        || borg_equips_staff_fail(sv_staff_destruction) as ::core::ffi::c_int != 0
        || borg_equips_item(act_destruction2, true_0 != 0) as ::core::ffi::c_int != 0
    {
        spell = true_0 != 0;
    }
    if (p1 > borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int
        || p1 > borg.avoidance as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                <= 150 as ::core::ffi::c_int)
        && borg_equips_staff_fail(sv_staff_destruction) as ::core::ffi::c_int != 0
    {
        spell = true_0 != 0;
    }
    if spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = 0 as ::core::ffi::c_int;
    d = p1 - p2;
    if borg.near.unique as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
        && p1 < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        d = 0 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        d = 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_teleportlevel(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !borg_simulate {
        if borg_spell(TELEPORT_LEVEL) {
            borg_needs_new_sea = true_0 != 0;
            return 500 as ::core::ffi::c_int;
        }
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
        return 0 as ::core::ffi::c_int;
    }
    if p1 < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg
        .trait_0
        .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize)
            != 0)
        && *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
    {
        if borg_caution_teleport(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *borg
        .trait_0
        .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
    {
        if borg_caution_teleport(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if !borg_spell_okay_fail(TELEPORT_LEVEL, 55 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.morgoth as ::core::ffi::c_int != 0
        || borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return p1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_banishment(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut banished_monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut using_artifact: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if p1
        < borg.avoidance as ::core::ffi::c_int * 1 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int {
        fail_allowed -= 10 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    using_artifact = borg_equips_item(act_loskill, true_0 != 0) as ::core::ffi::c_int != 0
        && *borg.has.offset(BI_CURHP as ::core::ffi::c_int as isize) > 100 as ::core::ffi::c_int;
    if !using_artifact && !borg_spell_okay_fail(BANISH_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    p1 = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
            .offset((*kill).pos.x as isize) as *mut borg_grid;
        if !((*kill).r_idx == 0) {
            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                p1 += borg_danger_one_kill(
                    borg.c.y,
                    borg.c.x,
                    1 as ::core::ffi::c_int,
                    i,
                    true_0 != 0,
                    true_0 != 0,
                );
            }
        }
        i += 1;
    }
    p2 = p1;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        kill_0 = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill_0).r_idx as isize) as *mut monster_race;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill_0).pos.y as isize))
            .offset((*kill_0).pos.x as isize) as *mut borg_grid;
        if !((*kill_0).r_idx == 0) {
            if !((*kill_0).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if borg_projectable(borg.c.y, borg.c.x, (*kill_0).pos.y, (*kill_0).pos.x) {
                    if !borg_simulate {
                        borg_note(format(
                            b"# Banishing Evil: (%d,%d): %s, danger %d. is considered.\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            (*kill_0).pos.y,
                            (*kill_0).pos.x,
                            borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                            borg_danger_one_kill(
                                borg.c.y,
                                borg.c.x,
                                1 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                false_0 != 0,
                            ),
                        ));
                    }
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
                        RF_EVIL as ::core::ffi::c_int,
                    ) {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Stays (not evil).\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    false_0 != 0,
                                ),
                            ));
                        }
                    } else if flag_has(
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
                    ) as ::core::ffi::c_int
                        != 0
                        && (*kill_0).injury as ::core::ffi::c_int > 60 as ::core::ffi::c_int
                    {
                        if !borg_simulate {
                            borg_note(
                                format(
                                    b"# Banishing Evil: (%d,%d): %s, danger %d. Unique not considered: Injury %d.\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    (*kill_0).pos.y,
                                    (*kill_0).pos.x,
                                    borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                    borg_danger_one_kill(
                                        borg.c.y,
                                        borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        (*ag).kill as ::core::ffi::c_int,
                                        true_0 != 0,
                                        false_0 != 0,
                                    ),
                                    (*kill_0).injury as ::core::ffi::c_int,
                                ),
                            );
                        }
                    } else if !borg_cave_floor_bold((*kill_0).pos.y, (*kill_0).pos.x) {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Stays (in wall).\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                ),
                            ));
                        }
                    } else {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Booted.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                ),
                            ));
                            borg_delete_kill(i);
                        }
                        banished_monsters += 1;
                        p2 -= borg_danger_one_kill(
                            borg.c.y,
                            borg.c.x,
                            1 as ::core::ffi::c_int,
                            i,
                            true_0 != 0,
                            true_0 != 0,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    if !borg_simulate {
        if using_artifact {
            if borg_activate_item(act_loskill) {
                return p1 - p2;
            }
        }
        if borg_spell(BANISH_EVIL) {
            return p1 - p2;
        }
    }
    if p2 <= 0 as ::core::ffi::c_int {
        p2 = 0 as ::core::ffi::c_int;
    }
    if banished_monsters == 0 as ::core::ffi::c_int {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) > 250 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 99 as ::core::ffi::c_int
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) > 350 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 100 as ::core::ffi::c_int
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
    {
        if borg_simulate {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_inviso(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    if *borg
        .trait_0
        .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || borg.see_inv as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.time.now - borg.need_see_invis > 45 as borg_time {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if -(1 as ::core::ffi::c_int)
        == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_detect_invis)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_detect_invis)
        && !borg_equips_staff_fail(sv_staff_detect_invis)
        && !borg_equips_staff_fail(sv_staff_detect_evil)
        && !borg_spell_okay_fail(SENSE_INVISIBLE, fail_allowed)
        && !borg_spell_okay_fail(DETECTION, fail_allowed)
        && !borg_spell_okay_fail(REVEAL_MONSTERS, fail_allowed)
        && !borg_equips_item(act_detect_invis, true_0 != 0)
        && !borg_equips_item(act_tmd_sinvis, true_0 != 0)
        && !borg_equips_item(act_tmd_esp, true_0 != 0)
        && !borg_equips_item(act_detect_evil, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(REVEAL_MONSTERS, fail_allowed) as ::core::ffi::c_int != 0
        || borg_spell_fail(DETECTION, fail_allowed) as ::core::ffi::c_int != 0
        || borg_read_scroll(sv_scroll_detect_invis) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_detect_invis) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_detect_evil) as ::core::ffi::c_int != 0
        || borg_activate_item(act_detect_invis) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tmd_sinvis) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tmd_esp) as ::core::ffi::c_int != 0
        || borg_activate_item(act_detect_evil) as ::core::ffi::c_int != 0
    {
        borg.see_inv = 3000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    if borg_quaff_potion(sv_potion_detect_invis) {
        borg.see_inv = 18000 as int16_t;
        borg.no_rest_prep = 18000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(SENSE_INVISIBLE, fail_allowed) {
        borg.see_inv = 30000 as int16_t;
        borg.no_rest_prep = 16000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_lbeam(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut hallway: bool = false_0 != 0;
    let mut x: ::core::ffi::c_int = borg.c.x;
    let mut y: ::core::ffi::c_int = borg.c.y;
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
        return 0 as ::core::ffi::c_int;
    }
    if borg.time.now - borg.need_see_invis < 48 as borg_time {
        return 0 as ::core::ffi::c_int;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        hallway = true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        hallway = true_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
    {
        hallway = true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
    {
        hallway = true_0 != 0;
    }
    if !hallway {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && p1
            > borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_light_beam(true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_light_beam(false_0 != 0) {
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_panel_shift() -> bool {
    let mut dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut panel_num_x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut panel_num_y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut pos_on_panel_x: ::core::ffi::c_int = borg.c.x - (*Term).offset_x;
    let mut pos_on_panel_y: ::core::ffi::c_int = borg.c.y - (*Term).offset_y;
    if !borg.need_shift_panel
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 70 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && w_y == borg.mon.morgoth_panel.y
        && w_x == borg.mon.morgoth_panel.x
    {
        return false_0 != 0;
    }
    if (*Term).offset_x > borg_panel_wid() {
        panel_num_x =
            (*Term).offset_x * 2 as ::core::ffi::c_int / borg_panel_wid() + 1 as ::core::ffi::c_int;
    }
    if (*Term).offset_y > borg_panel_hgt() {
        panel_num_y =
            (*Term).offset_y * 2 as ::core::ffi::c_int / borg_panel_hgt() + 1 as ::core::ffi::c_int;
    }
    if borg_panel_wid() - pos_on_panel_x < 5 as ::core::ffi::c_int && panel_num_x < borg.panels.x {
        dir = 6 as ::core::ffi::c_int;
    }
    if pos_on_panel_x < 5 as ::core::ffi::c_int && panel_num_x > 1 as ::core::ffi::c_int {
        dir = 4 as ::core::ffi::c_int;
    }
    if borg_panel_hgt() - pos_on_panel_y < 5 as ::core::ffi::c_int && panel_num_y < borg.panels.y {
        dir = 2 as ::core::ffi::c_int;
    }
    if pos_on_panel_y < 5 as ::core::ffi::c_int && panel_num_y > 1 as ::core::ffi::c_int {
        dir = 8 as ::core::ffi::c_int;
    }
    if dir == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg.need_shift_panel as ::core::ffi::c_int == true_0 {
        borg_keypress('L' as i32 as keycode_t);
        borg_keypress((dir + '0' as i32) as keycode_t);
        borg_note(
            b"# Shifted panel to locate offscreen monster.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.need_shift_panel = false_0 != 0;
        borg_keypress(ESCAPE as keycode_t);
        return true_0 != 0;
    } else if !(borg.when_shift_panel != 0
        && (borg.time.now - borg.when_shift_panel <= 10 as borg_time
            || borg.time.now - borg.time.morgoth <= 10 as borg_time))
    {
        if track_step.num != 0 {
            if dir == 8 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y - 2 as ::core::ffi::c_int,
                    borg.c.x,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.y.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.y - 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 2 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y + 2 as ::core::ffi::c_int,
                    borg.c.x,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.y.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.y + 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 4 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y,
                    borg.c.x - 2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.x.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.x - 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 6 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y,
                    borg.c.x + 2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.x.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.x + 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            }
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_defend_aux_rest() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !borg.morgoth_position
        && (!borg.status.anti_summon || borg.time.now - borg.time.antisummon >= 50 as borg_time)
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth
        && (!borg.status.anti_summon || borg.time.now - borg.time.antisummon >= 50 as borg_time)
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(TELEPORT_OTHER, 30 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        if !((*kill).r_idx == 0) {
            ax = if x9 > borg.c.x {
                x9 - borg.c.x
            } else {
                borg.c.x - x9
            };
            ay = if y9 > borg.c.y {
                y9 - borg.c.y
            } else {
                borg.c.y - y9
            };
            d = if ax < ay { ay } else { ax };
            if !(d > (*z_info).max_range as ::core::ffi::c_int) {
                if borg_los(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) as ::core::ffi::c_int
                    != 0
                    && ((*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth
                        || (*kill).ranged_attack as ::core::ffi::c_int != 0)
                    && borg.avoidance as ::core::ffi::c_int
                        <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                {
                    borg_note(
                        b"# Not resting. I can see Morgoth or a shooter.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                borg.avoidance as ::core::ffi::c_int
                    > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize);
            }
        }
        i += 1;
    }
    if borg_simulate {
        return 200 as ::core::ffi::c_int;
    }
    borg_keypress(',' as i32 as keycode_t);
    borg_note(format(
        b"# Resting on grid (%d, %d), waiting for Morgoth.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg.c.y,
        borg.c.x,
    ));
    return 200 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_tele_away_morgoth() -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if !(*borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(TELEPORT_OTHER, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_surrounded() as ::core::ffi::c_int == true_0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                    if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                        if !(distance(borg.c, (*kill).pos)
                            > (*z_info).max_range as ::core::ffi::c_int)
                        {
                            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if p2 > 1000 as ::core::ffi::c_int {
        p2 = 1000 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    if borg_simulate {
        return p2;
    }
    borg_log_spellpath();
    i = 0 as ::core::ffi::c_int;
    while i < borg_tp_other_n as ::core::ffi::c_int {
        borg_note(format(
            b"# %d, index %d (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_tp_other_n as ::core::ffi::c_int,
            borg_tp_other_index[i as usize],
            borg_tp_other_y[i as usize] as ::core::ffi::c_int,
            borg_tp_other_x[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    borg_note(
        b"# Attempting to cast T.O. for depth 100.\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if borg_spell(TELEPORT_OTHER) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tele_other) as ::core::ffi::c_int != 0
        || borg_aim_wand(sv_wand_teleport_away) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
        return p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_banishment_morgoth() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut banish_evil: bool = false;
    let mut banishment: bool = false;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if !borg.near.morgoth {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    banish_evil = borg_spell_okay_fail(BANISH_EVIL, fail_allowed);
    banishment = borg_spell_okay_fail(MASS_BANISHMENT, fail_allowed);
    if !banishment && !banish_evil {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    count = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    if !(banish_evil as ::core::ffi::c_int != 0
                        && !banishment
                        && !flag_has(
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
                        ))
                    {
                        if !(distance(borg.c, (*kill).pos)
                            > (*z_info).max_range as ::core::ffi::c_int)
                        {
                            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                                if distance(borg.c, (*kill).pos) <= 7 as ::core::ffi::c_int {
                                    count += 1;
                                }
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if count <= 7 as ::core::ffi::c_int && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1500 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Attempting to cast Banishment for depth 100.  %d monsters \0" as *const u8
            as *const ::core::ffi::c_char,
        count,
    ));
    if borg_spell(MASS_BANISHMENT) as ::core::ffi::c_int != 0
        || borg_spell(BANISH_EVIL) as ::core::ffi::c_int != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut tmp_kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
            let mut tmp_r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
            tmp_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*tmp_kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*tmp_kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                tmp_r_ptr = r_info.offset((*tmp_kill).r_idx as isize) as *mut monster_race;
                if !flag_has(
                    &raw mut (*tmp_r_ptr).flags as *mut bitflag,
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
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        return 1000 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_light_morgoth() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut best: loc = loc { x: 0, y: 0 };
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg.morgoth_position {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SPEAR_OF_LIGHT, fail_allowed)
        && !borg_spell_okay_fail(CLAIRVOYANCE, fail_allowed)
        && !borg_spell_okay_fail(FUME_OF_MORDOR, fail_allowed)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint != borg.id.morgoth) {
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    x = (*kill).pos.x;
                    y = (*kill).pos.y;
                    if !(distance(borg.c, (*kill).pos) > (*z_info).max_range as ::core::ffi::c_int)
                    {
                        if !(distance(borg.c, (*kill).pos) <= 5 as ::core::ffi::c_int) {
                            if borg_projectable_dark(borg.c.y, borg.c.x, y, x) {
                                count += 1;
                                best.y = y;
                                best.x = x;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if count <= 0 as ::core::ffi::c_int && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 500 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Attempting to Illuminate a Pathway to (%d, %d)\0" as *const u8
            as *const ::core::ffi::c_char,
        best.y,
        best.x,
    ));
    borg_target(best, false_0 != 0);
    if borg_spell(SPEAR_OF_LIGHT) as ::core::ffi::c_int != 0
        || borg_spell(CLAIRVOYANCE) as ::core::ffi::c_int != 0
        || borg_spell(FUME_OF_MORDOR) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        return 200 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_calculate_defense_effectiveness(
    mut what: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match what {
        1 => return borg_defend_aux_speed(p1),
        9 => return borg_defend_aux_prot_evil(p1),
        2 => return borg_defend_aux_grim_purpose(p1),
        3 => return borg_defend_aux_resist_fecap(p1),
        4 => return borg_defend_aux_resist_f(p1),
        5 => return borg_defend_aux_resist_c(p1),
        6 => return borg_defend_aux_resist_a(p1),
        7 => return borg_defend_aux_resist_e(p1),
        8 => return borg_defend_aux_resist_p(p1),
        0 => return borg_defend_aux_bless(p1),
        12 => return borg_defend_aux_hero(p1),
        13 => return borg_defend_aux_berserk(p1),
        14 => return borg_defend_aux_smite_evil(p1),
        15 => return borg_defend_aux_regen(p1),
        10 => return borg_defend_aux_shield(p1),
        11 => return borg_defend_aux_tele_away(p1),
        16 => return borg_defend_aux_glyph(p1),
        17 => return borg_defend_aux_create_door(p1),
        18 => return borg_defend_aux_mass_genocide(p1),
        19 => return borg_defend_aux_genocide(p1),
        20 => return borg_defend_aux_genocide_nasties(p1),
        21 => return borg_defend_aux_earthquake(p1),
        23 => return borg_defend_aux_teleportlevel(p1),
        22 => return borg_defend_aux_destruction(p1),
        24 => return borg_defend_aux_banishment(p1),
        25 => return borg_defend_aux_inviso(p1),
        26 => return borg_defend_aux_lbeam(p1),
        27 => return borg_defend_aux_rest(),
        28 => return borg_defend_aux_tele_away_morgoth(),
        29 => return borg_defend_aux_banishment_morgoth(),
        30 => return borg_defend_aux_light_morgoth(),
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_defend(mut p1: ::core::ffi::c_int) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if borg_defend_panel_shift() {
        return true_0 != 0;
    }
    borg_simulate = true_0 != 0;
    if borg.resistance as ::core::ffi::c_int != 0
        && (borg.resistance as ::core::ffi::c_int)
            < borg_game_ratio as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        let mut p: ::core::ffi::c_int = 0;
        borg_attacking = true_0 != 0;
        p = borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            false_0 != 0,
            false_0 != 0,
        );
        borg_attacking = false_0 != 0;
        if p > borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            || borg.near.unique as ::core::ffi::c_int != 0
        {
            if borg_spell(RESISTANCE) {
                borg_note(format(
                    b"# Refreshing Resistance.  borg.resistance=%d, player->=%d, (ratio=%d)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    borg.resistance as ::core::ffi::c_int,
                    *(*player)
                        .timed
                        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    borg_game_ratio as ::core::ffi::c_int,
                ));
                borg_attempting_refresh_resist = true_0 != 0;
                borg.resistance = 25000 as int16_t;
                return true_0 != 0;
            }
        }
    }
    g = 0 as ::core::ffi::c_int;
    while g < BD_MAX as ::core::ffi::c_int {
        n = borg_calculate_defense_effectiveness(g, p1);
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
        b"# Performing defense type %d with value %d\0" as *const u8 as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_defense_effectiveness(b_g, p1);
    return true_0 != 0;
}
