pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_DIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;

use crate::headers::borg_flow_kill_h::borg_race_death;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_home_notice_h::{
    borg_notice_home, num_ezheal_true, num_heal_true, num_life_true, num_speed,
};
use crate::headers::borg_item_activation_h::{
    act_clairvoyance, act_cure_critical, act_cure_full, act_cure_full2, act_cure_light,
    act_cure_nonorlybig, act_cure_serious, act_detect_all, act_enchant_armor, act_enchant_armor2,
    act_enchant_todam, act_enchant_tohit, act_enchant_weapon, act_enlightenment, act_firebrand,
    act_glyph, act_haste, act_haste1, act_haste2, act_heal1, act_heal2, act_heal3,
    act_illumination, act_light, act_mapping, act_protevil, act_recall, act_recharge,
    act_restore_exp, act_restore_life, act_restore_st_lev, act_staff_holy, act_staff_magi,
    act_stone_to_mud, act_tele_long, act_tele_phase,
};
use crate::headers::borg_item_analyze_h::borg_obj_has_effect;
pub use crate::headers::borg_item_h::{
    borg_item, borg_item_weight, borg_items, C2RustUnnamedH_44, BORG_CURSE_AIR_SWING,
    BORG_CURSE_ANNOYANCE, BORG_CURSE_ANTI_TELEPORTATION, BORG_CURSE_BURNING_UP,
    BORG_CURSE_CHILLED_TO_THE_BONE, BORG_CURSE_CLUMSINESS, BORG_CURSE_COWARDICE,
    BORG_CURSE_DEMON_SUMMON, BORG_CURSE_DRAGON_SUMMON, BORG_CURSE_DULLNESS, BORG_CURSE_ENVELOPING,
    BORG_CURSE_HALLUCINATION, BORG_CURSE_IMPAIR_HITPOINT_RECOVERY, BORG_CURSE_IMPAIR_MANA_RECOVERY,
    BORG_CURSE_IRRITATION, BORG_CURSE_MAX, BORG_CURSE_PARALYSIS, BORG_CURSE_POISON,
    BORG_CURSE_SICKLINESS, BORG_CURSE_SIREN, BORG_CURSE_SLOWNESS, BORG_CURSE_STEELSKIN,
    BORG_CURSE_STONE, BORG_CURSE_TELEPORTATION, BORG_CURSE_TREACHEROUS_WEAPON,
    BORG_CURSE_UNDEAD_SUMMON, BORG_CURSE_UNKNOWN, BORG_CURSE_VULNERABILITY, BORG_CURSE_WEAKNESS,
};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;
use crate::headers::borg_item_use_h::{borg_activate_failure, borg_equips_item, borg_equips_ring};
use crate::headers::borg_item_val_h::{
    kv_flask_oil, kv_potion_healing, kv_staff_holiness, sv_food_apple, sv_food_draught,
    sv_food_handful, sv_food_honey_cake, sv_food_pint, sv_food_ration, sv_food_sip, sv_food_slice,
    sv_food_slime_mold, sv_food_waybread, sv_heavy_xbow, sv_light_lantern, sv_light_torch,
    sv_light_xbow, sv_long_bow, sv_mush_cure_mind, sv_mush_debility, sv_mush_emergency,
    sv_mush_purging, sv_mush_restoring, sv_mush_second_sight, sv_mush_sprinting, sv_mush_stoneskin,
    sv_mush_terror, sv_potion_cure_critical, sv_potion_cure_light, sv_potion_cure_poison,
    sv_potion_cure_serious, sv_potion_healing, sv_potion_inc_all, sv_potion_inc_con,
    sv_potion_inc_dex, sv_potion_inc_int, sv_potion_inc_str, sv_potion_inc_wis, sv_potion_life,
    sv_potion_resist_cold, sv_potion_resist_heat, sv_potion_resist_pois, sv_potion_restore_life,
    sv_potion_speed, sv_potion_star_healing, sv_ring_digging, sv_rod_acid_ball, sv_rod_acid_bolt,
    sv_rod_cold_ball, sv_rod_cold_bolt, sv_rod_detection, sv_rod_drain_life, sv_rod_elec_ball,
    sv_rod_elec_bolt, sv_rod_fire_ball, sv_rod_fire_bolt, sv_rod_healing, sv_rod_illumination,
    sv_rod_light, sv_rod_mapping, sv_rod_recall, sv_rod_speed, sv_scroll_enchant_armor,
    sv_scroll_enchant_weapon_to_dam, sv_scroll_enchant_weapon_to_hit, sv_scroll_identify,
    sv_scroll_mass_banishment, sv_scroll_phase_door, sv_scroll_protection_from_evil,
    sv_scroll_recharging, sv_scroll_rune_of_protection, sv_scroll_star_enchant_armor,
    sv_scroll_star_enchant_weapon, sv_scroll_teleport, sv_scroll_teleport_level,
    sv_scroll_word_of_recall, sv_short_bow, sv_sling, sv_staff_destruction, sv_staff_healing,
    sv_staff_holiness, sv_staff_power, sv_staff_speed, sv_staff_teleportation, sv_staff_the_magi,
    sv_wand_annihilation, sv_wand_magic_missile, sv_wand_stinking_cloud, sv_wand_teleport_away,
};
use crate::headers::borg_item_wear_h::borg_is_ammo;
pub use crate::headers::borg_magic_h::{
    borg_cheat_spells, borg_get_book_num, borg_spell_legal, borg_spell_legal_fail, borg_spell_stat,
    borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM,
    BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT,
    CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH,
    CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE,
    DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT,
    DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING,
    ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL,
    FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE,
    GRONDS_BLOW, HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD,
    HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE,
    LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM,
    MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION,
    ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE,
    RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE,
    REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT,
    RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT,
    SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT,
    SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE,
    TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR,
    TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM,
    VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION,
    WORD_OF_RECALL, ZONE_OF_UNMAGIC,
};
use crate::headers::borg_think_h::borg_do_spell;
pub use crate::headers::borg_trait_h::{
    borg_item_pos, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT, BORG_EQUIP, BORG_INVEN, BORG_QUILL,
};
use crate::headers::borg_trait_swap_h::{
    armour_swap_free_act, armour_swap_hold_life, armour_swap_resist_acid, armour_swap_resist_blind,
    armour_swap_resist_chaos, armour_swap_resist_cold, armour_swap_resist_conf,
    armour_swap_resist_dark, armour_swap_resist_disen, armour_swap_resist_elec,
    armour_swap_resist_fear, armour_swap_resist_fire, armour_swap_resist_light,
    armour_swap_resist_neth, armour_swap_resist_nexus, armour_swap_resist_pois,
    armour_swap_resist_shard, armour_swap_resist_sound, borg_notice_armour_swap,
    borg_notice_weapon_swap, weapon_swap_free_act, weapon_swap_hold_life, weapon_swap_resist_acid,
    weapon_swap_resist_blind, weapon_swap_resist_chaos, weapon_swap_resist_cold,
    weapon_swap_resist_conf, weapon_swap_resist_dark, weapon_swap_resist_disen,
    weapon_swap_resist_elec, weapon_swap_resist_fear, weapon_swap_resist_fire,
    weapon_swap_resist_light, weapon_swap_resist_neth, weapon_swap_resist_nexus,
    weapon_swap_resist_pois, weapon_swap_resist_shard, weapon_swap_resist_sound,
};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::effects_h::{C2RustUnnamedH_45, EF_MAX, EF_NONE};
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
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
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
    C2RustUnnamedH_20, C2RustUnnamedH_31, C2RustUnnamedH_39, OBJ_MOD_MAX, OF_NONE, TV_MAX,
};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item, C2RustUnnamedH_38, ELEM_MAX,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
use crate::headers::player_calcs_h::{adj_str_blow, adj_str_hold};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_flags, player_history,
    player_race, player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23,
    C2RustUnnamedH_34, C2RustUnnamedH_47, PF_MAX, SKILL_DEVICE, SKILL_DIGGING, SKILL_DISARM_MAGIC,
    SKILL_DISARM_PHYS, SKILL_MAX, SKILL_SAVE, SKILL_SEARCH, SKILL_STEALTH, SKILL_TO_HIT_BOW,
    SKILL_TO_HIT_MELEE, SKILL_TO_HIT_THROW, STAT_MAX,
};
pub use crate::headers::player_timed_h::{
    C2RustUnnamedH_22, PY_FOOD_FULL, PY_FOOD_HUNGRY, PY_FOOD_MAX, PY_FOOD_WEAK, TMD_MAX,
};
use crate::headers::player_util_h::modify_stat_value;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memset;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
static mut borg_adj_mag_mana: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    60 as ::core::ffi::c_int,
    70 as ::core::ffi::c_int,
    80 as ::core::ffi::c_int,
    90 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    110 as ::core::ffi::c_int,
    120 as ::core::ffi::c_int,
    130 as ::core::ffi::c_int,
    140 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    160 as ::core::ffi::c_int,
    170 as ::core::ffi::c_int,
    180 as ::core::ffi::c_int,
    190 as ::core::ffi::c_int,
    200 as ::core::ffi::c_int,
    225 as ::core::ffi::c_int,
    250 as ::core::ffi::c_int,
    300 as ::core::ffi::c_int,
    350 as ::core::ffi::c_int,
    400 as ::core::ffi::c_int,
    450 as ::core::ffi::c_int,
    500 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    650 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    750 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
];
static mut borg_adj_dex_ta: [::core::ffi::c_int; 38] = [
    -(4 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_adj_str_td: [::core::ffi::c_int; 38] = [
    -(2 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_adj_dex_th: [::core::ffi::c_int; 38] = [
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
static mut borg_adj_str_th: [::core::ffi::c_int; 38] = [
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
static mut borg_adj_dex_blow: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
];
static mut borg_blows_table: [[::core::ffi::c_int; 12]; 12] = [
    [
        100 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
    ],
    [
        100 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
    ],
    [
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
    ],
    [
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
    ],
    [
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
    ],
    [
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        35 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        34 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        33 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    ],
];
static mut borg_adj_dex_dis: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_int_dis: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_int_dev: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
];
static mut borg_adj_str_dig: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    25 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    35 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    45 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    55 as ::core::ffi::c_int,
    60 as ::core::ffi::c_int,
    65 as ::core::ffi::c_int,
    70 as ::core::ffi::c_int,
    75 as ::core::ffi::c_int,
    80 as ::core::ffi::c_int,
    85 as ::core::ffi::c_int,
    90 as ::core::ffi::c_int,
    95 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
];
static mut borg_adj_wis_sav: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_str_wgt: [::core::ffi::c_int; 38] = [
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    22 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    26 as ::core::ffi::c_int,
    28 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
];
static mut borg_adj_con_mhp: [::core::ffi::c_int; 38] = [
    -(250 as ::core::ffi::c_int),
    -(150 as ::core::ffi::c_int),
    -(100 as ::core::ffi::c_int),
    -(75 as ::core::ffi::c_int),
    -(50 as ::core::ffi::c_int),
    -(25 as ::core::ffi::c_int),
    -(10 as ::core::ffi::c_int),
    -(5 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    25 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    75 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    175 as ::core::ffi::c_int,
    200 as ::core::ffi::c_int,
    225 as ::core::ffi::c_int,
    250 as ::core::ffi::c_int,
    275 as ::core::ffi::c_int,
    300 as ::core::ffi::c_int,
    350 as ::core::ffi::c_int,
    400 as ::core::ffi::c_int,
    450 as ::core::ffi::c_int,
    500 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    650 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    750 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    900 as ::core::ffi::c_int,
    1000 as ::core::ffi::c_int,
    1100 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
];
static mut borg_adj_mag_fail: [::core::ffi::c_int; 38] = [
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
];
static mut borg_adj_mag_stat: [::core::ffi::c_int; 38] = [
    -(5 as ::core::ffi::c_int),
    -(4 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    21 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    27 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    33 as ::core::ffi::c_int,
    36 as ::core::ffi::c_int,
    39 as ::core::ffi::c_int,
    42 as ::core::ffi::c_int,
    45 as ::core::ffi::c_int,
    48 as ::core::ffi::c_int,
    51 as ::core::ffi::c_int,
    54 as ::core::ffi::c_int,
    57 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_simulate: bool = false;
#[no_mangle]
pub static mut borg_attacking: bool = false;
#[no_mangle]
pub static mut borg_on_glyph: bool = false;
#[no_mangle]
pub static mut borg_create_door: bool = false;
#[no_mangle]
pub static mut borg_sleep_spell: bool = false;
#[no_mangle]
pub static mut borg_sleep_spell_ii: bool = false;
#[no_mangle]
pub static mut borg_crush_spell: bool = false;
#[no_mangle]
pub static mut borg_slow_spell: bool = false;
#[no_mangle]
pub static mut borg_confuse_spell: bool = false;
#[no_mangle]
pub static mut borg_fear_mon_spell: bool = false;
#[no_mangle]
pub static mut borg_game_ratio: int16_t = 0;
#[no_mangle]
pub static mut prefix_pref: [*const ::core::ffi::c_char; 266] = [
    b"str\0" as *const u8 as *const ::core::ffi::c_char,
    b"int\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"con\0" as *const u8 as *const ::core::ffi::c_char,
    b"str adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"int adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"con adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur str\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur int\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur con\0" as *const u8 as *const ::core::ffi::c_char,
    b"str index\0" as *const u8 as *const ::core::ffi::c_char,
    b"int index\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis index\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex index\0" as *const u8 as *const ::core::ffi::c_char,
    b"con index\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust str\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust int\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust con\0" as *const u8 as *const ::core::ffi::c_char,
    b"class\0" as *const u8 as *const ::core::ffi::c_char,
    b"light\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"max hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"hp adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur sp\0" as *const u8 as *const ::core::ffi::c_char,
    b"max sp\0" as *const u8 as *const ::core::ffi::c_char,
    b"sp adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFAIL1\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFAIL2\0" as *const u8 as *const ::core::ffi::c_char,
    b"clevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"max clevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"esp\0" as *const u8 as *const ::core::ffi::c_char,
    b"recall\0" as *const u8 as *const ::core::ffi::c_char,
    b"food\0" as *const u8 as *const ::core::ffi::c_char,
    b"food high\0" as *const u8 as *const ::core::ffi::c_char,
    b"food low\0" as *const u8 as *const ::core::ffi::c_char,
    b"food cure conf\0" as *const u8 as *const ::core::ffi::c_char,
    b"food cure blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"speed\0" as *const u8 as *const ::core::ffi::c_char,
    b"gold\0" as *const u8 as *const ::core::ffi::c_char,
    b"extra moves\0" as *const u8 as *const ::core::ffi::c_char,
    b"damage reduction\0" as *const u8 as *const ::core::ffi::c_char,
    b"slow dig\0" as *const u8 as *const ::core::ffi::c_char,
    b"feather fall\0" as *const u8 as *const ::core::ffi::c_char,
    b"regen\0" as *const u8 as *const ::core::ffi::c_char,
    b"see inv\0" as *const u8 as *const ::core::ffi::c_char,
    b"infravision\0" as *const u8 as *const ::core::ffi::c_char,
    b"fast shots\0" as *const u8 as *const ::core::ffi::c_char,
    b"disarm ph\0" as *const u8 as *const ::core::ffi::c_char,
    b"disarm mag\0" as *const u8 as *const ::core::ffi::c_char,
    b"use device\0" as *const u8 as *const ::core::ffi::c_char,
    b"save\0" as *const u8 as *const ::core::ffi::c_char,
    b"stealth\0" as *const u8 as *const ::core::ffi::c_char,
    b"search\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit normal\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit bow\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit throw\0" as *const u8 as *const ::core::ffi::c_char,
    b"dig\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fear\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist lite\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dark\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist conf\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist sound\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist shards\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nexus\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nether\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist chaos\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dis\0" as *const u8 as *const ::core::ffi::c_char,
    b"hold life\0" as *const u8 as *const ::core::ffi::c_char,
    b"free action\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fire with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist cold with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist elec with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist acid with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist poison with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fear with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist lite with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dark with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist blind with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist conf with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist sound with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist shards with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nexus with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nether with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist chaos with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dis with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"hold life with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"free action with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"max depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"king\0" as *const u8 as *const ::core::ffi::c_char,
    b"is weak\0" as *const u8 as *const ::core::ffi::c_char,
    b"is hungry\0" as *const u8 as *const ::core::ffi::c_char,
    b"is full\0" as *const u8 as *const ::core::ffi::c_char,
    b"is gorged\0" as *const u8 as *const ::core::ffi::c_char,
    b"is blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"is afraid\0" as *const u8 as *const ::core::ffi::c_char,
    b"is confused\0" as *const u8 as *const ::core::ffi::c_char,
    b"is poisoned\0" as *const u8 as *const ::core::ffi::c_char,
    b"is cut\0" as *const u8 as *const ::core::ffi::c_char,
    b"is stun\0" as *const u8 as *const ::core::ffi::c_char,
    b"is heavystun\0" as *const u8 as *const ::core::ffi::c_char,
    b"is paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
    b"is image\0" as *const u8 as *const ::core::ffi::c_char,
    b"is forget\0" as *const u8 as *const ::core::ffi::c_char,
    b"is encumb\0" as *const u8 as *const ::core::ffi::c_char,
    b"is study\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixlev\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixexp\0" as *const u8 as *const ::core::ffi::c_char,
    b"has fixexp\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixstr\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixint\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixwis\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixdex\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixcon\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixall\0" as *const u8 as *const ::core::ffi::c_char,
    b"armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep id\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep damage dice\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep damage sides\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow id\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow is sling\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow artifact\0" as *const u8 as *const ::core::ffi::c_char,
    b"blows\0" as *const u8 as *const ::core::ffi::c_char,
    b"EXTRA_BLOWS\0" as *const u8 as *const ::core::ffi::c_char,
    b"shots\0" as *const u8 as *const ::core::ffi::c_char,
    b"heavy weapon\0" as *const u8 as *const ::core::ffi::c_char,
    b"heavy bow\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo count\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo tval\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo sides\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo power\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ego missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cursed missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"quiver slots\0" as *const u8 as *const ::core::ffi::c_char,
    b"first cursed\0" as *const u8 as *const ::core::ffi::c_char,
    b"where cursed\0" as *const u8 as *const ::core::ffi::c_char,
    b"enveloping\0" as *const u8 as *const ::core::ffi::c_char,
    b"irritation\0" as *const u8 as *const ::core::ffi::c_char,
    b"teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"siren\0" as *const u8 as *const ::core::ffi::c_char,
    b"hallucinate\0" as *const u8 as *const ::core::ffi::c_char,
    b"paralysis\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse stone\0" as *const u8 as *const ::core::ffi::c_char,
    b"no teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"treach wep\0" as *const u8 as *const ::core::ffi::c_char,
    b"aggravate\0" as *const u8 as *const ::core::ffi::c_char,
    b"vulnerable\0" as *const u8 as *const ::core::ffi::c_char,
    b"dullness\0" as *const u8 as *const ::core::ffi::c_char,
    b"sickness\0" as *const u8 as *const ::core::ffi::c_char,
    b"weakness\0" as *const u8 as *const ::core::ffi::c_char,
    b"clumsiness\0" as *const u8 as *const ::core::ffi::c_char,
    b"slowness\0" as *const u8 as *const ::core::ffi::c_char,
    b"annoyance\0" as *const u8 as *const ::core::ffi::c_char,
    b"impair hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"CRSMPIMP\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse steel\0" as *const u8 as *const ::core::ffi::c_char,
    b"air swing\0" as *const u8 as *const ::core::ffi::c_char,
    b"fear\0" as *const u8 as *const ::core::ffi::c_char,
    b"drain xp\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown curse\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay animal\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay orc\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay troll\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay giant\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep impact\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt phase\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt escape\0" as *const u8 as *const ::core::ffi::c_char,
    b"fuel\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt heal\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ezheal\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt life\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt speed\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt staff magi\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt staff destruction\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport other\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cure poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect traps\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect door\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt magic map\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt recharge\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt call lite\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt prot evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt glyph\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion ccw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion csw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion clw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to dam\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt *ench to wep*\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt *ench to armor*\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt brand\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to dam\0" as *const u8 as *const ::core::ffi::c_char,
    b"need brand\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist heat\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport level\0" as *const u8 as *const ::core::ffi::c_char,
    b"holy word\0" as *const u8 as *const ::core::ffi::c_char,
    b"mass banishment\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cool shroom\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt attack rods1\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt attack rods2\0" as *const u8 as *const ::core::ffi::c_char,
    b"worn need id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt need id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt diggers\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt good staff chg\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt good wand chg\0" as *const u8 as *const ::core::ffi::c_char,
    b"multi bonus\0" as *const u8 as *const ::core::ffi::c_char,
    b"detect inv\0" as *const u8 as *const ::core::ffi::c_char,
    b"weight\0" as *const u8 as *const ::core::ffi::c_char,
    b"carry\0" as *const u8 as *const ::core::ffi::c_char,
    b"empty slots\0" as *const u8 as *const ::core::ffi::c_char,
    b"sauron dead\0" as *const u8 as *const ::core::ffi::c_char,
    b"prep big fight\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn borg_uses_swaps() -> bool {
    return crate::bridge::st().core.cfg[(BORG_USES_SWAPS as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 90 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_calc_blows(mut item: *mut borg_item) -> ::core::ffi::c_int {
    let mut blows: ::core::ffi::c_int = 0;
    let mut str_index: ::core::ffi::c_int = 0;
    let mut dex_index: ::core::ffi::c_int = 0;
    let mut div: ::core::ffi::c_int = 0;
    let mut blow_energy: ::core::ffi::c_int = 0;
    let mut weight: ::core::ffi::c_int =
        (*item).weight as ::core::ffi::c_int * (*item).iqty as ::core::ffi::c_int;
    let mut min_weight: ::core::ffi::c_int = (*(*player).class).min_weight;
    div = if weight < min_weight {
        min_weight
    } else {
        weight
    };
    str_index = adj_str_blow[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize] as usize]
        * (*(*player).class).att_multiply
        / div;
    if str_index > 11 as ::core::ffi::c_int {
        str_index = 11 as ::core::ffi::c_int;
    }
    dex_index = if borg_adj_dex_blow[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
        as usize]
        > 11 as ::core::ffi::c_int
    {
        11 as ::core::ffi::c_int
    } else {
        borg_adj_dex_blow[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize] as usize]
    };
    blow_energy = borg_blows_table[str_index as usize][dex_index as usize];
    blows = if 10000 as ::core::ffi::c_int / blow_energy
        > 100 as ::core::ffi::c_int * (*(*player).class).max_attacks
    {
        100 as ::core::ffi::c_int * (*(*player).class).max_attacks
    } else {
        10000 as ::core::ffi::c_int / blow_energy
    };
    return (if blows
        + 100 as ::core::ffi::c_int
            * ((*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_EXTRA_BLOWS as ::core::ffi::c_int) as usize])
        < (if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        {
            200 as ::core::ffi::c_int
        } else {
            100 as ::core::ffi::c_int
        }) {
        (if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        {
            200 as ::core::ffi::c_int
        } else {
            100 as ::core::ffi::c_int
        })
    } else {
        blows
            + 100 as ::core::ffi::c_int
                * ((*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + crate::bridge::st().borg.trait_[(BI_EXTRA_BLOWS as ::core::ffi::c_int) as usize])
    }) / 100 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_notice_ammo(mut slot: ::core::ffi::c_int) {
    let mut item: *mut borg_item = borg_items.offset(slot as isize) as *mut borg_item;
    if (*item).iqty == 0 {
        return;
    }
    if slot >= (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let ref mut fresh1 = crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize];
        *fresh1 += 1;
    }
    crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize] +=
        borg_item_weight(item) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_AMMO_COUNT as ::core::ffi::c_int) as usize] += (*item).iqty as ::core::ffi::c_int;
    if (*item).tval as ::core::ffi::c_int
        != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
    {
        return;
    }
    crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize] += (*item).iqty as ::core::ffi::c_int;
    if (*item).uncursable {
        crate::bridge::st().borg.trait_[(BI_WHERE_CURSED as ::core::ffi::c_int) as usize] |=
            BORG_QUILL as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
            == 0
        {
            crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize] =
                slot + 1 as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.trait_[(BI_AMISSILES_CURSED as ::core::ffi::c_int) as usize] +=
            (*item).iqty as ::core::ffi::c_int;
        return;
    }
    if (*item).ego_idx != 0 {
        crate::bridge::st().borg.trait_[(BI_AMISSILES_SPECIAL as ::core::ffi::c_int) as usize] +=
            (*item).iqty as ::core::ffi::c_int;
    }
    if (*item).value <= 0 as int32_t {
        return;
    }
    if crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
        >= 3 as ::core::ffi::c_int
    {
        if (borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            && (*item).ego_idx == 0
            && (*item).art_idx == 0
            && (*item).ident as ::core::ffi::c_int != 0
            && (*item).tval as ::core::ffi::c_int
                == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
        {
            let ref mut fresh2 = crate::bridge::st().borg.trait_[(BI_NEED_BRAND_WEAPON as ::core::ffi::c_int) as usize];
            *fresh2 =
                (*fresh2 as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            > 35 as ::core::ffi::c_int
        {
            if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
                && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            {
                if ((*item).to_h as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize] +=
                        10 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                }
                if ((*item).to_d as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize] +=
                        10 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                }
            } else {
                if ((*item).to_h as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize] +=
                        8 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                }
                if ((*item).to_d as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize] +=
                        8 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                }
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
        < 3 as ::core::ffi::c_int
    {
        return;
    }
    if (borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0)
        && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
        && (*item).art_idx == 0
        && (*item).ego_idx == 0
        && (*item).ident as ::core::ffi::c_int != 0
        && (*item).tval as ::core::ffi::c_int
            == crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
    {
        let ref mut fresh3 = crate::bridge::st().borg.trait_[(BI_NEED_BRAND_WEAPON as ::core::ffi::c_int) as usize];
        *fresh3 =
            (*fresh3 as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn cursed_nonartifact(mut item: *mut borg_item) -> bool {
    if item.is_null() || (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*item).art_idx != 0 {
        return false_0 != 0;
    }
    if (*item).uncursable {
        return false_0 != 0;
    }
    return (*item).cursed;
}
unsafe extern "C" fn borg_notice_equipment() {
    let mut i: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_int = 0;
    let mut rb_ptr: *const player_race = (*player).race;
    let mut cb_ptr: *const player_class = (*player).class;
    let mut extra_shots: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut extra_might: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut my_num_fire: ::core::ffi::c_int = 0;
    let mut f: [bitflag; 5] = [0; 5];
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut stat_cur: [int16_t; 5] = [0; 5];
    my_num_fire = 1 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_INFRA as ::core::ffi::c_int) as usize] = (*rb_ptr).infra;
    crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_DISARM_PHYS as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DISARM_PHYS as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_DEVICE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DEVICE as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_SAVE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_SAVE as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_STEALTH as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_STEALTH as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_SRCH as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_SEARCH as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_SEARCH as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_THN as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_THB as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_THT as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize];
    crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] = (*rb_ptr).r_skills
        [SKILL_DIGGING as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DIGGING as ::core::ffi::c_int as usize];
    player_flags(player, &raw mut f as *mut bitflag);
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_IMPACT as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_W_IMPACT as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AGGRAVATE as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AFRAID as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_DRAIN_EXP as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_CRSDRAIN_XP as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        crate::bridge::st().borg.trait_[(BI_CRSFVULN as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        crate::bridge::st().borg.trait_[(BI_CRSAVULN as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        crate::bridge::st().borg.trait_[(BI_CRSCVULN as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        crate::bridge::st().borg.trait_[(BI_CRSEVULN as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*rb_ptr).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_FEAR as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_STR as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_INT as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_WIS as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_DEX as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_CON as ::core::ffi::c_int,
    ) {
        crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_FAST as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0
        || *(*player)
            .timed
            .offset(TMD_SPRINT as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] += 10 as ::core::ffi::c_int;
    } else if *(*player)
        .timed
        .offset(TMD_TERROR as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] += 5 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        if (*borg_items.offset(i as isize)).activ_idx != 0 {
            crate::bridge::st().borg.activation[((*borg_items.offset(i as isize)).activ_idx) as usize] +=
                1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if crate::bridge::st().borg.activation[(act_staff_magi) as usize] != 0 {
        crate::bridge::st().borg.trait_[(BI_ASTFMAGI as ::core::ffi::c_int) as usize] += 10 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).uncursable {
                crate::bridge::st().borg.trait_[(BI_WHERE_CURSED as ::core::ffi::c_int) as usize] |=
                    BORG_EQUIP as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
                    == 0
                {
                    crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize] =
                        i + 1 as ::core::ffi::c_int;
                }
            }
            if !cursed_nonartifact(item) {
                crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize] +=
                    borg_item_weight(item) as ::core::ffi::c_int;
                if borg_item_note_needs_id(item) {
                    crate::bridge::st().borg.trait_[(BI_ALL_NEED_ID as ::core::ffi::c_int) as usize] +=
                        1 as ::core::ffi::c_int;
                    crate::bridge::st().borg.trait_[(BI_WORN_NEED_ID as ::core::ffi::c_int) as usize] +=
                        1 as ::core::ffi::c_int;
                }
                crate::bridge::st().borg.trait_[(BI_ASTR as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_STR as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_AINT as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_INT as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_AWIS as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_ADEX as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_ACON as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_CON as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_ANIMAL as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_EVIL as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_UNDEAD as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_DEMON as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_ORC as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_TROLL as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_GIANT as ::core::ffi::c_int as usize];
                crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize] =
                    (*item).slays[RF_DRAGON as ::core::ffi::c_int as usize];
                if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_WB_POIS as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_IMPACT as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_W_IMPACT as ::core::ffi::c_int) as usize] = true_0;
                }
                crate::bridge::st().borg.trait_[(BI_INFRA as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_STEALTH as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_SRCH as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_SEARCH as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * 5 as ::core::ffi::c_int;
                let mut dig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if (*item).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int {
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
                    ) {
                        dig = 1 as ::core::ffi::c_int;
                    } else if flag_has(
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
                    ) {
                        dig = 2 as ::core::ffi::c_int;
                    } else if flag_has(
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
                    ) {
                        dig = 3 as ::core::ffi::c_int;
                    }
                }
                dig += (*item).modifiers[OBJ_MOD_TUNNEL as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] +=
                    dig * 20 as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                if i != (*z_info).pack_size as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_EXTRA_BLOWS as ::core::ffi::c_int) as usize] += (*item).modifiers
                        [OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                }
                extra_shots += (*item).modifiers[OBJ_MOD_SHOTS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                extra_might += (*item).modifiers[OBJ_MOD_MIGHT as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                if i != (*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                    || flag_has(
                        &raw mut (*borg_items.offset(i as isize)).flags as *mut bitflag,
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
                        OF_NO_FUEL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                    || (*item).timeout as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] += (*item)
                        .modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
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
                    ) {
                        crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] +=
                            2 as ::core::ffi::c_int;
                    } else if flag_has(
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
                        OF_LIGHT_3 as ::core::ffi::c_int,
                    ) {
                        crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] +=
                            3 as ::core::ffi::c_int;
                    }
                    if (*item).modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_NECROMANCER
                    {
                        let ref mut fresh4 =
                            crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize];
                        *fresh4 -= 1;
                    }
                    crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] += (*item)
                        .modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                }
                crate::bridge::st().borg.trait_[(BI_MOD_MOVES as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_MOVES as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_DAM_RED as ::core::ffi::c_int) as usize] += (*item).modifiers
                    [OBJ_MOD_DAM_RED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
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
                    crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_AGGRAVATE as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_IMPAIR_HP as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_CRSHPIMP as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_IMPAIR_MANA as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_CRSMPIMP as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_AFRAID as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize] = true_0;
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
                    OF_DRAIN_EXP as ::core::ffi::c_int,
                ) {
                    crate::bridge::st().borg.trait_[(BI_CRSDRAIN_XP as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_VULNERABILITY as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSVULN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_TELEPORTATION as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSTELE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_DULLNESS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSDULL as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_SICKLINESS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSICK as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_ENVELOPING as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSENVELOPING as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_IRRITATION as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_CRSIRRITATION as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_WEAKNESS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSWEAK as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_CLUMSINESS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSCLUM as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_SLOWNESS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSLOW as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_ANNOYANCE as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSANNOY as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_POISON as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSPOIS as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_SIREN as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSIREN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_HALLUCINATION as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSHALU as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_PARALYSIS as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSPARA as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_DEMON_SUMMON as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSDEM as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_DRAGON_SUMMON as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSDRA as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_UNDEAD_SUMMON as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSUND as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_IMPAIR_MANA_RECOVERY as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSMPIMP as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses
                    [BORG_CURSE_IMPAIR_HITPOINT_RECOVERY as ::core::ffi::c_int as usize]
                {
                    crate::bridge::st().borg.trait_[(BI_CRSHPIMP as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_COWARDICE as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_STONE as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSTONE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_ANTI_TELEPORTATION as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSNOTEL as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_TREACHEROUS_WEAPON as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSTWEP as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_BURNING_UP as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSFVULN as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_CHILLED_TO_THE_BONE as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSCVULN as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_STEELSKIN as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSSTEELSKIN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_AIR_SWING as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSAIRSWING as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).curses[BORG_CURSE_UNKNOWN as ::core::ffi::c_int as usize] {
                    crate::bridge::st().borg.trait_[(BI_CRSUNKNO as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    crate::bridge::st().borg.trait_[(BI_CRSFVULN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    crate::bridge::st().borg.trait_[(BI_CRSAVULN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    crate::bridge::st().borg.trait_[(BI_CRSCVULN as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    crate::bridge::st().borg.trait_[(BI_CRSEVULN as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.temp.res_fire = true_0 != 0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.temp.res_acid = true_0 != 0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.temp.res_cold = true_0 != 0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] = true_0;
                    crate::bridge::st().borg.temp.res_elec = true_0 != 0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] = true_0;
                }
                if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] = true_0;
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
                    crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] = true_0;
                }
                let mut bonuses: ::core::ffi::c_int = ((*item).el_info
                    [ELEM_POIS as ::core::ffi::c_int as usize]
                    .res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + (flag_has(
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
                        OF_SUST_STR as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && flag_has(
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
                            OF_SUST_INT as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_WIS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_DEX as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_CON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
                if bonuses > 2 as ::core::ffi::c_int {
                    crate::bridge::st().borg.trait_[(BI_MULTIPLE_BONUSES as ::core::ffi::c_int) as usize] += bonuses;
                }
                if (*item).art_idx == 0
                    && (*item).ego_idx == 0
                    && (*item).ac as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                    && (*item).to_a as ::core::ffi::c_int + (*item).ac as ::core::ffi::c_int
                        <= 0 as ::core::ffi::c_int
                {
                    (*item).to_a = -(20 as ::core::ffi::c_int) as int16_t;
                }
                crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] +=
                    (*item).ac as ::core::ffi::c_int;
                crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] +=
                    (*item).to_a as ::core::ffi::c_int;
                if !(i == (*z_info).pack_size as ::core::ffi::c_int) {
                    if !(i == (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) {
                        crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] +=
                            (*item).to_h as ::core::ffi::c_int;
                        crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize] +=
                            (*item).to_d as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] <= 0 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] = 1 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSVULN as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize] = true_0;
        crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] -= 50 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSDULL as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSICK as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSWEAK as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize] -= 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSCLUM as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize] -= 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSSLOW as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] -= 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSANNOY as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] -= 10 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] -= 10 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_CRSAGRV as ::core::ffi::c_int) as usize] = true_0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        stat_cur[i as usize] = (*player).stat_cur[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        let mut add: ::core::ffi::c_int = 0;
        let mut use_0: ::core::ffi::c_int = 0;
        let mut ind: ::core::ffi::c_int = 0;
        add = crate::bridge::st().borg.trait_[((BI_ASTR as ::core::ffi::c_int + i)) as usize];
        add += (*(*player).race).r_adj[i as usize] + (*(*player).class).c_adj[i as usize];
        use_0 = modify_stat_value(stat_cur[i as usize] as ::core::ffi::c_int, add)
            as ::core::ffi::c_int;
        if use_0 <= 18 as ::core::ffi::c_int {
            ind = use_0 - 3 as ::core::ffi::c_int;
        } else if use_0 <= 18 as ::core::ffi::c_int + 219 as ::core::ffi::c_int {
            ind = 15 as ::core::ffi::c_int
                + (use_0 - 18 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int;
        } else {
            ind = 37 as ::core::ffi::c_int;
        }
        if ind > 37 as ::core::ffi::c_int {
            crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + i)) as usize] =
                37 as ::core::ffi::c_int;
        } else {
            crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + i)) as usize] = ind;
        }
        crate::bridge::st().borg.trait_[((BI_STR as ::core::ffi::c_int + i)) as usize] = use_0;
        crate::bridge::st().borg.trait_[((BI_CSTR as ::core::ffi::c_int + i)) as usize] =
            stat_cur[i as usize] as ::core::ffi::c_int;
        i += 1;
    }
    crate::bridge::st().borg.trait_[(BI_HP_ADJ as ::core::ffi::c_int) as usize] = (*player).player_hp
        [((*player).lev as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int
        + borg_adj_con_mhp[crate::bridge::st().borg.trait_[(BI_CON_INDEX as ::core::ffi::c_int) as usize]
            as usize]
            * crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            / 100 as ::core::ffi::c_int;
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_SP_ADJ as ::core::ffi::c_int) as usize] = borg_adj_mag_mana[crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + spell_stat)) as usize]
            as usize]
            * (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                - (*(*player).class).magic.spell_first
                + 1 as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_FAIL1 as ::core::ffi::c_int) as usize] = borg_adj_mag_stat[crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + spell_stat)) as usize]
            as usize];
        crate::bridge::st().borg.trait_[(BI_FAIL2 as ::core::ffi::c_int) as usize] = borg_adj_mag_fail[crate::bridge::st().borg.trait_[((BI_STR_INDEX as ::core::ffi::c_int + spell_stat)) as usize]
            as usize];
    }
    if crate::bridge::st().borg.trait_[(BI_ISGORGED as ::core::ffi::c_int) as usize]
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] -= 10 as ::core::ffi::c_int;
    }
    crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] += borg_adj_dex_ta[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize] += borg_adj_str_td[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] += borg_adj_dex_th[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] += borg_adj_str_th[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    hold = adj_str_hold[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize] as usize];
    crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] += borg_adj_str_dig[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || cursed_nonartifact(item) as ::core::ffi::c_int != 0
    {
        (*item).ds = 0 as uint8_t;
        (*item).dd = 0 as uint8_t;
        (*item).to_d = 0 as int16_t;
        (*item).to_h = 0 as int16_t;
        (*item).weight = 0 as int16_t;
        (*item).ident = true_0 != 0;
        (*item).sval = 0 as uint8_t;
    }
    crate::bridge::st().borg.trait_[(BI_BTOHIT as ::core::ffi::c_int) as usize] = (*item).to_h as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_BTODAM as ::core::ffi::c_int) as usize] = (*item).to_d as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_BID as ::core::ffi::c_int) as usize] =
        (*item).ident as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SLING as ::core::ffi::c_int) as usize] =
        ((*item).sval as ::core::ffi::c_int == sv_sling) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_BART as ::core::ffi::c_int) as usize] =
        (*item).art_idx as ::core::ffi::c_int;
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_HEAVYBOW as ::core::ffi::c_int) as usize] = true_0;
        crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] += 2 as ::core::ffi::c_int
            * (hold - (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int);
    }
    if (*item).iqty as ::core::ffi::c_int != 0
        && hold >= (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        if (*item).sval as ::core::ffi::c_int == sv_sling {
            crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] =
                TV_SHOT as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 3 as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] = 2 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_short_bow {
            crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] =
                TV_ARROW as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 4 as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] = 2 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_long_bow {
            crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] =
                TV_ARROW as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 4 as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] = 3 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_light_xbow {
            crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] =
                TV_BOLT as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 5 as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] = 3 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_heavy_xbow {
            crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] =
                TV_BOLT as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 5 as ::core::ffi::c_int;
            crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] = 4 as ::core::ffi::c_int;
        }
        crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize] += extra_might;
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
            PF_FAST_SHOT as ::core::ffi::c_int,
        ) {
            if crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize]
                == TV_ARROW as ::core::ffi::c_int
            {
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    >= 20 as ::core::ffi::c_int
                {
                    my_num_fire += 1;
                }
            }
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                >= 40 as ::core::ffi::c_int
            {
                my_num_fire += 1;
            }
            crate::bridge::st().borg.trait_[(BI_FAST_SHOTS as ::core::ffi::c_int) as usize] = true_0;
        }
        my_num_fire += extra_shots;
        if my_num_fire < 1 as ::core::ffi::c_int {
            my_num_fire = 1 as ::core::ffi::c_int;
        }
    }
    crate::bridge::st().borg.trait_[(BI_SHOTS as ::core::ffi::c_int) as usize] = my_num_fire;
    item = borg_items.offset((*z_info).pack_size as isize) as *mut borg_item;
    if (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || cursed_nonartifact(item) as ::core::ffi::c_int != 0
    {
        (*item).ds = 0 as uint8_t;
        (*item).dd = 0 as uint8_t;
        (*item).to_d = 0 as int16_t;
        (*item).to_h = 0 as int16_t;
        (*item).weight = 0 as int16_t;
        (*item).ident = true_0 != 0;
    }
    crate::bridge::st().borg.trait_[(BI_WTOHIT as ::core::ffi::c_int) as usize] = (*item).to_h as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_WTODAM as ::core::ffi::c_int) as usize] = (*item).to_d as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_WID as ::core::ffi::c_int) as usize] =
        (*item).ident as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_WDD as ::core::ffi::c_int) as usize] = (*item).dd as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_WDS as ::core::ffi::c_int) as usize] = (*item).ds as ::core::ffi::c_int;
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_HEAVYWEPON as ::core::ffi::c_int) as usize] = true_0;
        crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] += 2 as ::core::ffi::c_int
            * (hold - (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int);
    }
    if (*item).iqty as ::core::ffi::c_int != 0
        && hold >= (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize] = borg_calc_blows(item);
        crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] +=
            (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_BRAVERY_30 as ::core::ffi::c_int,
    ) {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= 30 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] = true_0;
        }
    }
    crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] += borg_adj_dex_dis[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] += borg_adj_int_dis[crate::bridge::st().borg.trait_[(BI_INT_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] += borg_adj_int_dev[crate::bridge::st().borg.trait_[(BI_INT_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] += borg_adj_wis_sav[crate::bridge::st().borg.trait_[(BI_WIS_INDEX as ::core::ffi::c_int) as usize]
        as usize];
    crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_DISARM_PHYS as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_DEVICE as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_SAVE as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_STEALTH as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRCH as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_SEARCH as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_THN as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_THB as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_THT as ::core::ffi::c_int) as usize] += (*cb_ptr).x_skills
        [SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize]
        * crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        / 10 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] > 30 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] = 30 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] < 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_STL as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] = 1 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
            != 0
    {
        crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] -= 20 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] += 8 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] =
            crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize] * 95 as ::core::ffi::c_int
                / 100 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_BLESS_WEAPON as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && ((*item).tval as ::core::ffi::c_int == TV_HAFTED as ::core::ffi::c_int
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
                OF_BLESSED as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0)
    {
        crate::bridge::st().borg.trait_[(BI_TOHIT as ::core::ffi::c_int) as usize] += 2 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_TODAM as ::core::ffi::c_int) as usize] += 2 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !cursed_nonartifact(item) {
                    if !(i == (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                            < 3 as ::core::ffi::c_int
                        && ((*item).art_idx == 0 && (*item).ego_idx == 0))
                    {
                        if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            != 0
                            || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                                >= 1 as ::core::ffi::c_int
                        {
                            if ((*item).to_h as ::core::ffi::c_int)
                                < crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                            {
                                crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize] +=
                                    crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                                        - (*item).to_h as ::core::ffi::c_int;
                            }
                            if ((*item).to_d as ::core::ffi::c_int)
                                < crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                            {
                                crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize] +=
                                    crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                                        - (*item).to_d as ::core::ffi::c_int;
                            }
                        } else {
                            if ((*item).to_h as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                                crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize] +=
                                    8 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                            }
                            if ((*item).to_d as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                                crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize] +=
                                    8 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !cursed_nonartifact(item) {
                    if borg_spell_legal_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                        || crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize]
                            >= 1 as ::core::ffi::c_int
                    {
                        if ((*item).to_a as ::core::ffi::c_int)
                            < crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                        {
                            crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int) as usize] +=
                                crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                                    - (*item).to_a as ::core::ffi::c_int;
                        }
                    } else if ((*item).to_a as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                        crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int) as usize] +=
                            8 as ::core::ffi::c_int - (*item).to_a as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    if crate::bridge::st().borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] = true_0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
        && borg_spell_legal(SENSE_INVISIBLE) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] = true_0;
    }
    if crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] >= 100 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] = true_0;
    }
    if crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize] >= 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0
    {
        crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize] = true_0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        borg_notice_ammo(i);
        i += 1;
    }
}
unsafe extern "C" fn borg_notice_inventory() {
    let mut i: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        crate::bridge::st().borg.need_statgain[i as usize] = false_0 != 0;
        crate::bridge::st().borg.amt_statgain[i as usize] = 0 as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        crate::bridge::st().borg.amt_book[i as usize] = 0 as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
    {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item).iqty == 0 {
            let ref mut fresh0 = crate::bridge::st().borg.trait_[(BI_EMPTY as ::core::ffi::c_int) as usize];
            *fresh0 += 1;
        } else if borg_is_ammo((*item).tval as ::core::ffi::c_int) {
            borg_notice_ammo(i);
        } else {
            crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize] +=
                borg_item_weight(item) as ::core::ffi::c_int;
            if borg_item_note_needs_id(item) {
                crate::bridge::st().borg.trait_[(BI_ALL_NEED_ID as ::core::ffi::c_int) as usize] +=
                    1 as ::core::ffi::c_int;
            }
            if (*item).uncursable {
                crate::bridge::st().borg.trait_[(BI_WHERE_CURSED as ::core::ffi::c_int) as usize] |=
                    BORG_INVEN as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
                    == 0
                {
                    crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize] =
                        i + 1 as ::core::ffi::c_int;
                }
            }
            if (*item).aware {
                crate::bridge::st().borg.has[((*item).kind) as usize] += (*item).iqty as ::core::ffi::c_int;
                if !cursed_nonartifact(item) {
                    let mut current_block_187: u64;
                    match (*item).tval as ::core::ffi::c_int {
                        30 | 31 | 32 | 33 | 34 => {
                            if !obj_kind_can_browse(
                                k_info.offset((*item).kind as isize) as *mut object_kind
                            ) {
                                current_block_187 = 2167674248514145403;
                            } else {
                                crate::bridge::st().borg.amt_book[borg_get_book_num((*item).sval as ::core::ffi::c_int)
                                    as usize] = (crate::bridge::st().borg.amt_book
                                    [borg_get_book_num((*item).sval as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_int
                                    + (*item).iqty as ::core::ffi::c_int)
                                    as int16_t;
                                current_block_187 = 2167674248514145403;
                            }
                        }
                        29 => {
                            if (*item).sval as ::core::ffi::c_int == sv_mush_purging
                                || (*item).sval as ::core::ffi::c_int == sv_mush_restoring
                                || (*item).sval as ::core::ffi::c_int == sv_mush_cure_mind
                            {
                                if crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize]
                                    != 0
                                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                        < crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize]
                                {
                                    current_block_187 = 2167674248514145403;
                                } else {
                                    current_block_187 = 8693738493027456495;
                                }
                            } else {
                                current_block_187 = 8693738493027456495;
                            }
                            match current_block_187 {
                                2167674248514145403 => {}
                                _ => {
                                    if (*item).sval as ::core::ffi::c_int == sv_mush_second_sight
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_emergency
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_terror
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_stoneskin
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_debility
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_sprinting
                                    {
                                        if crate::bridge::st().core.cfg[(BORG_MUNCHKIN_START as ::core::ffi::c_int) as usize] != 0
                                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                >= crate::bridge::st().core.cfg[(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int) as usize]
                                        {
                                            crate::bridge::st().borg.trait_[(BI_ASHROOM as ::core::ffi::c_int) as usize] += (*item).iqty as ::core::ffi::c_int;
                                        }
                                    }
                                    current_block_187 = 18153031941552419006;
                                }
                            }
                        }
                        28 => {
                            current_block_187 = 18153031941552419006;
                        }
                        26 => {
                            if (*item).sval as ::core::ffi::c_int == sv_potion_healing {
                                crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_star_healing {
                                crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_life {
                                crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_critical
                            {
                                crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_serious {
                                crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_light {
                                crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_poison {
                                crate::bridge::st().borg.trait_[(BI_ACUREPOIS as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_heat {
                                crate::bridge::st().borg.trait_[(BI_ARESHEAT as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_cold {
                                crate::bridge::st().borg.trait_[(BI_ARESCOLD as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_pois {
                                crate::bridge::st().borg.trait_[(BI_ARESPOIS as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_str {
                                crate::bridge::st().borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_int {
                                crate::bridge::st().borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_wis {
                                crate::bridge::st().borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_dex {
                                crate::bridge::st().borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_con {
                                crate::bridge::st().borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_all {
                                crate::bridge::st().borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                crate::bridge::st().borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                crate::bridge::st().borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                crate::bridge::st().borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                crate::bridge::st().borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] =
                                    (crate::bridge::st().borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_restore_life {
                                crate::bridge::st().borg.trait_[(BI_HASFIXEXP as ::core::ffi::c_int) as usize] = true_0;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_speed {
                                crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        25 => {
                            if (*item).sval as ::core::ffi::c_int == sv_scroll_identify {
                                crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_recharging {
                                crate::bridge::st().borg.trait_[(BI_ARECHARGE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_phase_door {
                                crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport {
                                crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_word_of_recall
                            {
                                crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_armor
                            {
                                crate::bridge::st().borg.trait_[(BI_AENCH_ARM as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_star_enchant_armor
                            {
                                crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_enchant_weapon_to_hit
                            {
                                crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_enchant_weapon_to_dam
                            {
                                crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_star_enchant_weapon
                            {
                                crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_protection_from_evil
                            {
                                crate::bridge::st().borg.trait_[(BI_APFE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_rune_of_protection
                            {
                                crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport_level
                            {
                                crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                                crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize] +=
                                    1 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_mass_banishment
                            {
                                crate::bridge::st().borg.trait_[(BI_AMASSBAN as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        24 => {
                            if (*item).sval as ::core::ffi::c_int == sv_rod_recall {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 100 as ::core::ffi::c_int;
                                } else {
                                    crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_detection {
                                crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                                crate::bridge::st().borg.trait_[(BI_ADETDOOR as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                                crate::bridge::st().borg.trait_[(BI_ADETEVIL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_illumination {
                                crate::bridge::st().borg.trait_[(BI_ALITE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_speed {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 100 as ::core::ffi::c_int;
                                } else {
                                    crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_mapping {
                                crate::bridge::st().borg.trait_[(BI_AMAGICMAP as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_healing {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int;
                                } else {
                                    crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_light
                                || (*item).sval as ::core::ffi::c_int == sv_rod_fire_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_elec_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_cold_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_acid_bolt
                            {
                                crate::bridge::st().borg.trait_[(BI_AROD1 as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_drain_life
                                || (*item).sval as ::core::ffi::c_int == sv_rod_fire_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_elec_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_cold_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_acid_ball
                            {
                                crate::bridge::st().borg.trait_[(BI_AROD2 as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        23 => {
                            if (*item).sval as ::core::ffi::c_int == sv_wand_teleport_away {
                                crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_stinking_cloud
                                && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    < 30 as ::core::ffi::c_int
                            {
                                crate::bridge::st().borg.trait_[(BI_GOOD_W_CHG as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_magic_missile
                                && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    < 30 as ::core::ffi::c_int
                            {
                                crate::bridge::st().borg.trait_[(BI_GOOD_W_CHG as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_annihilation {
                                crate::bridge::st().borg.trait_[(BI_GOOD_W_CHG as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        22 => {
                            if (*item).sval as ::core::ffi::c_int == sv_staff_teleportation {
                                crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize] +=
                                        (*item).pval as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_speed {
                                crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_healing {
                                crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_the_magi {
                                crate::bridge::st().borg.trait_[(BI_ASTFMAGI as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_destruction {
                                crate::bridge::st().borg.trait_[(BI_ASTFDEST as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_power {
                                crate::bridge::st().borg.trait_[(BI_GOOD_S_CHG as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_holiness {
                                crate::bridge::st().borg.trait_[(BI_GOOD_S_CHG as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                                crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        27 => {
                            if (*borg_items.offset(
                                ((*z_info).pack_size as ::core::ffi::c_int
                                    + 5 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .sval as ::core::ffi::c_int
                                == sv_light_lantern
                            {
                                crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        19 => {
                            if (*item).sval as ::core::ffi::c_int == sv_light_torch
                                && (*item).timeout as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                                && (*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 5 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .sval as ::core::ffi::c_int
                                    == sv_light_torch
                                && (*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 5 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .iqty as ::core::ffi::c_int
                                    != 0
                            {
                                crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        6 => {
                            if (*item).value <= 0 as int32_t {
                                current_block_187 = 2167674248514145403;
                            } else if (*item).cursed {
                                current_block_187 = 2167674248514145403;
                            } else if crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize]
                                < BORG_DIG
                            {
                                current_block_187 = 2167674248514145403;
                            } else {
                                crate::bridge::st().borg.trait_[(BI_ADIGGER as ::core::ffi::c_int) as usize] +=
                                    (*item).iqty as ::core::ffi::c_int;
                                current_block_187 = 2167674248514145403;
                            }
                        }
                        7 | 8 | 9 | _ => {
                            current_block_187 = 2167674248514145403;
                        }
                    }
                    match current_block_187 {
                        18153031941552419006 => {
                            if !(borg_obj_has_effect(
                                (*item).kind,
                                EF_CRUNCH as ::core::ffi::c_int,
                                -(1 as ::core::ffi::c_int),
                            ) as ::core::ffi::c_int
                                != 0
                                || borg_obj_has_effect(
                                    (*item).kind,
                                    EF_TIMED_INC as ::core::ffi::c_int,
                                    TMD_CONFUSED as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int
                                    != 0)
                            {
                                if (*item).tval as ::core::ffi::c_int
                                    == TV_MUSHROOM as ::core::ffi::c_int
                                {
                                    if borg_obj_has_effect(
                                        (*item).kind,
                                        EF_NOURISH as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                    ) {
                                        crate::bridge::st().borg.trait_[(BI_FOOD_LO as ::core::ffi::c_int) as usize] +=
                                            (*item).iqty as ::core::ffi::c_int;
                                    }
                                } else if (*item).sval as ::core::ffi::c_int == sv_food_apple
                                    || (*item).sval as ::core::ffi::c_int == sv_food_handful
                                    || (*item).sval as ::core::ffi::c_int == sv_food_slime_mold
                                    || (*item).sval as ::core::ffi::c_int == sv_food_pint
                                    || (*item).sval as ::core::ffi::c_int == sv_food_sip
                                {
                                    crate::bridge::st().borg.trait_[(BI_FOOD_LO as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int;
                                } else if (*item).sval as ::core::ffi::c_int == sv_food_ration
                                    || (*item).sval as ::core::ffi::c_int == sv_food_slice
                                    || (*item).sval as ::core::ffi::c_int == sv_food_honey_cake
                                    || (*item).sval as ::core::ffi::c_int == sv_food_waybread
                                    || (*item).sval as ::core::ffi::c_int == sv_food_draught
                                {
                                    crate::bridge::st().borg.trait_[(BI_FOOD_HI as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_POISONED as ::core::ffi::c_int,
                                ) {
                                    crate::bridge::st().borg.trait_[(BI_ACUREPOIS as ::core::ffi::c_int) as usize] +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_CONFUSED as ::core::ffi::c_int,
                                ) {
                                    crate::bridge::st().borg.trait_[(BI_FOOD_CURE_CONF as ::core::ffi::c_int) as usize] += (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_BLIND as ::core::ffi::c_int,
                                ) {
                                    crate::bridge::st().borg.trait_[(BI_FOOD_CURE_BLIND as ::core::ffi::c_int) as usize] += (*item).iqty as ::core::ffi::c_int;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1;
    }
    if crate::bridge::st().borg.has[(kv_flask_oil) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.has[(kv_flask_oil) as usize] < 15 as ::core::ffi::c_int {
            crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize] +=
                crate::bridge::st().borg.has[(kv_flask_oil) as usize];
        } else {
            crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize] += 15 as ::core::ffi::c_int;
        }
    }
    if borg_spell_legal_fail(REMOVE_HUNGER, 80 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(HERBAL_CURING, 80 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(IDENTIFY_RUNE) {
        crate::bridge::st().borg.trait_[(BI_AID as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(FIND_TRAPS_DOORS_STAIRS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECTION) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(REVEAL_MONSTERS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_LIFE) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_EVIL) as ::core::ffi::c_int != 0
        || borg_spell_legal(READ_MINDS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_MONSTERS) as ::core::ffi::c_int != 0
        || borg_spell_legal(SEEK_BATTLE) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ADETEVIL as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(DETECTION) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enlightenment, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_clairvoyance, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ADETDOOR as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ADETEVIL as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(SENSE_INVISIBLE) {
        crate::bridge::st().borg.trait_[(BI_DINV as ::core::ffi::c_int) as usize] = true_0;
    }
    if borg_spell_legal(SENSE_SURROUNDINGS) as ::core::ffi::c_int != 0
        || borg_equips_item(act_detect_all, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_mapping, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ADETDOOR as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_AMAGICMAP as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(LIGHT_ROOM) as ::core::ffi::c_int != 0
        || borg_equips_item(act_light, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_illumination, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(CALL_LIGHT) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ALITE as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(PROTECTION_FROM_EVIL) as ::core::ffi::c_int != 0
        || borg_equips_item(act_protevil, false_0 != 0) as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.has[(kv_staff_holiness) as usize] != 0
        || borg_equips_item(act_staff_holy, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_APFE as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(GLYPH_OF_WARDING) as ::core::ffi::c_int != 0
        || borg_equips_item(act_glyph, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_AGLYPH as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(FIND_TRAPS_DOORS_STAIRS) {
        crate::bridge::st().borg.trait_[(BI_ADETDOOR as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ADETTRAP as ::core::ffi::c_int) as usize] = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_weapon, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_enchant_tohit, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_enchant_todam, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.trait_[(BI_ABRAND as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_armor, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_armor2, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_AENCH_ARM as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TURN_STONE_TO_MUD, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_stone_to_mud, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_ring(sv_ring_digging) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ADIGGER as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(WORD_OF_RECALL, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 100 as ::core::ffi::c_int
            && borg_spell_legal(WORD_OF_RECALL) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_recall, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_LEVEL, 20 as ::core::ffi::c_int) {
        crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(PHASE_DOOR, 3 as ::core::ffi::c_int) {
        crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_tele_phase, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_SELF, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(PORTAL, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(SHADOW_SHIFT, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(DIMENSION_DOOR, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_tele_long, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize] += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_OTHER, 40 as ::core::ffi::c_int) {
        crate::bridge::st().borg.trait_[(BI_ATPORTOTHER as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(HOLY_WORD) {
        crate::bridge::st().borg.trait_[(BI_AHWORD as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(HASTE_SELF) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste1, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste2, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_light, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_serious, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_critical, false_0 != 0) {
        crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_full, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_full2, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal1, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal2, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal3, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(HEALING) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_exp, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_st_lev, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_life, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_HASFIXEXP as ::core::ffi::c_int) as usize] = true_0;
    }
    if borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_exp, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_st_lev, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_life, false_0 != 0) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] = true_0;
    }
    if borg_equips_item(act_recharge, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(RECHARGING) as ::core::ffi::c_int != 0
    {
        crate::bridge::st().borg.trait_[(BI_ARECHARGE as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
        .flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_NO_FUEL as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER
    {
        crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] += 1000 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize]
        == 0
    {
        crate::bridge::st().borg.trait_[(BI_HASFIXEXP as ::core::ffi::c_int) as usize] = true_0;
    }
    crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] += crate::bridge::st().borg.trait_[(BI_FOOD_HI as ::core::ffi::c_int) as usize];
    crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] += crate::bridge::st().borg.trait_[(BI_FOOD_LO as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
            >= 1000 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] -= 1000 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice(mut notice_swap: bool) {
    { let v = &mut crate::bridge::st().borg.has; v.iter_mut().for_each(|x| *x = 0); }
    { let v = &mut crate::bridge::st().borg.trait_; v.iter_mut().for_each(|x| *x = 0); }
    { let v = &mut crate::bridge::st().borg.activation; v.iter_mut().for_each(|x| *x = 0); }
    crate::bridge::st().borg.trait_[(BI_BLOWS as ::core::ffi::c_int) as usize] = 1 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] = 110 as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize] = -(1 as ::core::ffi::c_int);
    crate::bridge::st().borg.trait_[(BI_AMMO_SIDES as ::core::ffi::c_int) as usize] = 4 as ::core::ffi::c_int;
    borg_notice_player();
    if borg_do_spell {
        borg_cheat_spells();
        borg_do_spell = false_0 != 0;
    }
    borg_notice_equipment();
    borg_notice_inventory();
    if notice_swap {
        borg_notice_weapon_swap();
        borg_notice_armour_swap();
    }
    crate::bridge::st().borg.trait_[(BI_SRACID as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_acid as ::core::ffi::c_int != 0
            || weapon_swap_resist_acid as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRELEC as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_elec as ::core::ffi::c_int != 0
            || weapon_swap_resist_elec as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRFIRE as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_fire as ::core::ffi::c_int != 0
            || weapon_swap_resist_fire as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRCOLD as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_cold as ::core::ffi::c_int != 0
            || weapon_swap_resist_cold as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRPOIS as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_pois as ::core::ffi::c_int != 0
            || weapon_swap_resist_pois as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESIST_POISON, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRFEAR as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_fear as ::core::ffi::c_int != 0
            || weapon_swap_resist_fear as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRLITE as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_light as ::core::ffi::c_int != 0
            || weapon_swap_resist_light as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRDARK as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_dark as ::core::ffi::c_int != 0
            || weapon_swap_resist_dark as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRBLIND as ::core::ffi::c_int) as usize] = (crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
        != 0
        || armour_swap_resist_blind as ::core::ffi::c_int != 0
        || weapon_swap_resist_blind as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRCONF as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_conf as ::core::ffi::c_int != 0
            || weapon_swap_resist_conf as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRSND as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_sound as ::core::ffi::c_int != 0
            || weapon_swap_resist_sound as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRSHRD as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_shard as ::core::ffi::c_int != 0
            || weapon_swap_resist_shard as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRNXUS as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_nexus as ::core::ffi::c_int != 0
            || weapon_swap_resist_nexus as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRNTHR as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_neth as ::core::ffi::c_int != 0
            || weapon_swap_resist_neth as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRKAOS as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_chaos as ::core::ffi::c_int != 0
            || weapon_swap_resist_chaos as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SRDIS as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0
            || armour_swap_resist_disen as ::core::ffi::c_int != 0
            || weapon_swap_resist_disen as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SHLIFE as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0
            || armour_swap_hold_life as ::core::ffi::c_int != 0
            || weapon_swap_hold_life as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_SFRACT as ::core::ffi::c_int) as usize] =
        (crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0
            || armour_swap_free_act as ::core::ffi::c_int != 0
            || weapon_swap_free_act as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize] = borg_adj_str_wgt[crate::bridge::st().borg.trait_[(BI_STR_INDEX as ::core::ffi::c_int) as usize]
        as usize]
        * 100 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize]
        > crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize] / 2 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] -= (crate::bridge::st().borg.trait_[(BI_WEIGHT as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int)
            / (crate::bridge::st().borg.trait_[(BI_CARRY as ::core::ffi::c_int) as usize]
                / 10 as ::core::ffi::c_int);
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] > 199 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] = 199 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] > 110 as ::core::ffi::c_int {
        borg_game_ratio = (100000 as ::core::ffi::c_int
            / ((crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                - 110 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int)) as int16_t;
    } else {
        borg_game_ratio = 1000 as int16_t;
    }
    crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize] = false_0;
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 99 as ::core::ffi::c_int
    {
        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
        let mut total_big_heal: ::core::ffi::c_int = crate::bridge::st().borg.has[(kv_potion_healing) as usize];
        total_big_heal += crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize];
        total_big_heal += crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize];
        total_big_heal += num_heal_true as ::core::ffi::c_int;
        total_big_heal += num_ezheal_true as ::core::ffi::c_int;
        total_big_heal += num_life_true as ::core::ffi::c_int;
        if total_big_heal < 30 as ::core::ffi::c_int
            || num_speed as ::core::ffi::c_int
                + crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize]
                < 15 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize] = true_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_player() {
    let mut i: ::core::ffi::c_int = 0;
    crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] =
        (*(*player).class).cidx as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_ISFIXLEV as ::core::ffi::c_int) as usize] = false_0;
    if ((*player).lev as ::core::ffi::c_int) < (*player).max_lev as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[(BI_ISFIXLEV as ::core::ffi::c_int) as usize] = true_0;
    }
    crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] = (*player).lev as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] =
        (*player).max_lev as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] =
        (*player).total_winner as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize] = false_0;
    crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] = (*player).depth as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize] =
        (*player).max_depth as ::core::ffi::c_int;
    if (*player).exp < (*player).max_exp {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            == 50 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize] = true_0;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            == 50 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                >= 1 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize] = false_0;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            != 50 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize] = true_0;
        }
    }
    crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] =
        (*player).au as ::core::ffi::c_int;
    if crate::bridge::st().borg.goal.recalling == 0 && (*player).word_recall as ::core::ffi::c_int != 0 {
        crate::bridge::st().borg.goal.recalling =
            (*player).word_recall as ::core::ffi::c_int * 1000 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.goal.recalling != 0 && (*player).word_recall == 0 {
        crate::bridge::st().borg.goal.recalling = 0 as ::core::ffi::c_int;
    }
    if !crate::bridge::st().borg.temp.prot_from_evil
        && *(*player)
            .timed
            .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.temp.prot_from_evil = if *(*player)
            .timed
            .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != 0
        {
            true_0
        } else {
            false_0
        } != 0;
    }
    if !crate::bridge::st().borg.temp.fast
        && (*(*player)
            .timed
            .offset(TMD_FAST as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
            || *(*player)
                .timed
                .offset(TMD_SPRINT as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
            || *(*player)
                .timed
                .offset(TMD_TERROR as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0)
    {
        crate::bridge::st().borg.temp.fast = if *(*player)
            .timed
            .offset(TMD_FAST as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != 0
            || *(*player)
                .timed
                .offset(TMD_SPRINT as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
            || *(*player)
                .timed
                .offset(TMD_TERROR as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
        {
            true_0
        } else {
            false_0
        } != 0;
    }
    crate::bridge::st().borg.temp.res_acid = if *(*player)
        .timed
        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.res_elec = if *(*player)
        .timed
        .offset(TMD_OPP_ELEC as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.res_fire = if *(*player)
        .timed
        .offset(TMD_OPP_FIRE as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.res_cold = if *(*player)
        .timed
        .offset(TMD_OPP_COLD as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.res_pois = if *(*player)
        .timed
        .offset(TMD_OPP_POIS as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.bless = if *(*player)
        .timed
        .offset(TMD_BLESSED as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.shield = if *(*player)
        .timed
        .offset(TMD_SHIELD as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
        || *(*player)
            .timed
            .offset(TMD_STONESKIN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.fastcast = if *(*player)
        .timed
        .offset(TMD_FASTCAST as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.hero = if *(*player)
        .timed
        .offset(TMD_HERO as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.berserk = if *(*player)
        .timed
        .offset(TMD_SHERO as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.regen = if *(*player)
        .timed
        .offset(TMD_HEAL as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.venom = if *(*player)
        .timed
        .offset(TMD_ATT_POIS as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    crate::bridge::st().borg.temp.smite_evil = if *(*player)
        .timed
        .offset(TMD_ATT_EVIL as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    if crate::bridge::st().borg.see_inv == 0
        && *(*player)
            .timed
            .offset(TMD_SINVIS as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        crate::bridge::st().borg.see_inv = 1000 as int16_t;
    }
    crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] =
        (*player).chp as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] =
        (*player).mhp as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
        (*player).csp as ::core::ffi::c_int;
    crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] =
        (*player).msp as ::core::ffi::c_int;
    let ref mut fresh5 = crate::bridge::st().borg.trait_[(BI_ISGORGED as ::core::ffi::c_int) as usize];
    *fresh5 = false_0;
    let ref mut fresh6 = crate::bridge::st().borg.trait_[(BI_ISFULL as ::core::ffi::c_int) as usize];
    *fresh6 = *fresh5;
    let ref mut fresh7 = crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize];
    *fresh7 = *fresh6;
    crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize] = *fresh7;
    let ref mut fresh8 = crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize];
    *fresh8 = false_0;
    let ref mut fresh9 = crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize];
    *fresh9 = *fresh8;
    let ref mut fresh10 = crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize];
    *fresh10 = *fresh9;
    crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize] = *fresh10;
    let ref mut fresh11 = crate::bridge::st().borg.trait_[(BI_ISSTUDY as ::core::ffi::c_int) as usize];
    *fresh11 = false_0;
    let ref mut fresh12 = crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize];
    *fresh12 = *fresh11;
    let ref mut fresh13 = crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize];
    *fresh13 = *fresh12;
    let ref mut fresh14 = crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize];
    *fresh14 = *fresh13;
    crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] = *fresh14;
    crate::bridge::st().borg.trait_[(BI_ISPARALYZED as ::core::ffi::c_int) as usize] = false_0;
    crate::bridge::st().borg.trait_[(BI_ISFORGET as ::core::ffi::c_int) as usize] = false_0;
    if (*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_WEAK
    {
        let ref mut fresh15 = crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize];
        *fresh15 = true_0;
        crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize] = *fresh15;
    } else if (*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_HUNGRY
    {
        crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize] = true_0;
    } else if !((*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_FULL)
    {
        if (*(*player)
            .timed
            .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < PY_FOOD_MAX
        {
            crate::bridge::st().borg.trait_[(BI_ISFULL as ::core::ffi::c_int) as usize] = true_0;
        } else {
            let ref mut fresh16 = crate::bridge::st().borg.trait_[(BI_ISFULL as ::core::ffi::c_int) as usize];
            *fresh16 = true_0;
            crate::bridge::st().borg.trait_[(BI_ISGORGED as ::core::ffi::c_int) as usize] = *fresh16;
        }
    }
    if *(*player)
        .timed
        .offset(TMD_BLIND as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_AFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_POISONED as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_CUT as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0
        && *(*player)
            .timed
            .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            <= 50 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        > 50 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_PARALYZED as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        > 50 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.trait_[(BI_ISPARALYZED as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_IMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize] = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_AMNESIA as ::core::ffi::c_int as isize)
        != 0
    {
        crate::bridge::st().borg.trait_[(BI_ISFORGET as ::core::ffi::c_int) as usize] = true_0;
    }
    if (*(*player).upkeep).new_spells != 0 {
        crate::bridge::st().borg.trait_[(BI_ISSTUDY as ::core::ffi::c_int) as usize] = true_0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        crate::bridge::st().borg.trait_[((BI_ISFIXSTR as ::core::ffi::c_int + i)) as usize] =
            (((*player).stat_cur[(STAT_STR as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int)
                < (*player).stat_max[(STAT_STR as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int) as ::core::ffi::c_int;
        crate::bridge::st().borg.trait_[((BI_CSTR as ::core::ffi::c_int + i)) as usize] =
            (*player).stat_cur[(STAT_STR as ::core::ffi::c_int + i) as usize] as ::core::ffi::c_int;
        i += 1;
    }
    crate::bridge::st().borg.trait_[(BI_SAURON_DEAD as ::core::ffi::c_int) as usize] =
        *borg_race_death.offset(crate::bridge::st().borg.id.sauron as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_trait_init() {
    borg_trait_free();
    crate::bridge::st().borg.has = vec![0; (*z_info).k_max as usize];
    crate::bridge::st().borg.trait_ = vec![0; BI_MAX as usize];
    crate::bridge::st().borg.activation = vec![0; (*z_info).act_max as usize];
}
#[no_mangle]
pub unsafe extern "C" fn borg_trait_free() {
    crate::bridge::st().borg.has = Vec::new();
    crate::bridge::st().borg.trait_ = Vec::new();
    crate::bridge::st().borg.activation = Vec::new();
}
