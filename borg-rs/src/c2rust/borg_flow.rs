pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_DIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BORG_DIG_MOD: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const BORG_DIG_HARD: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const AUTO_FLOW_MAX: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const GOAL_DARK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};
use crate::headers::borg_danger_h::{borg_danger, borg_danger_wipe};
use crate::headers::borg_flow_glyph_h::{borg_free_flow_glyph, borg_init_flow_glyph};
pub use crate::headers::borg_flow_h::{borg_data, borg_track};
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_race_name};
use crate::headers::borg_flow_misc_h::{
    borg_free_flow_misc, borg_goto_dir, borg_init_flow_misc, track_vein,
};
use crate::headers::borg_flow_stairs_h::{
    borg_free_flow_stairs, borg_init_flow_stairs, track_less,
};
pub use crate::headers::borg_flow_take_h::{
    borg_delete_take, borg_get_top_object, borg_take, borg_takes,
};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_keypress, borg_note, borg_queue_direction};

pub use crate::headers::borg_item_h::{borg_item};
use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_activate_ring, borg_equips_item, borg_equips_ring,
};

pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_okay, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE,
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
use crate::headers::borg_projection_h::borg_target;
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
use crate::headers::borg_trait_swap_h::weapon_swap;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx, ddx_ddd, ddy, ddy_ddd, distance, feat_is_shop, heatmap,
    monster_group, square, square_in_bounds_fully, trap, C2RustUnnamedH_18, FEAT_MAX,
};
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
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memcpy, memset, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVENT_EMPTY, EVT_BUTTON,
    EVT_DISCONNECT, EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT,
    EVT_SWITCH,
};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut flow_head: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut flow_tail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_flow_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_flow_x: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_flow_y: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_data_flow: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_cost: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_hard: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_know: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_icky: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_temp_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_temp_x: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut borg_temp_y: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut track_step: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut track_door: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut track_closed: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut borg_ddx_ddd: [int16_t; 24] = [
    0 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
];
#[no_mangle]
pub static mut borg_ddy_ddd: [int16_t; 24] = [
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
];
#[no_mangle]
pub unsafe extern "C" fn borg_can_dig(mut check_fail: bool, mut feat: uint8_t) -> bool {
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        || (feat as ::core::ffi::c_int) < FEAT_SECRET as ::core::ffi::c_int
            && feat as ::core::ffi::c_int != FEAT_CLOSED as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    let mut dig_check: ::core::ffi::c_int = 0;
    if feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_SECRET as ::core::ffi::c_int
    {
        dig_check = BORG_DIG_HARD;
    } else if feat as ::core::ffi::c_int == FEAT_QUARTZ as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_QUARTZ_K as ::core::ffi::c_int
    {
        dig_check = BORG_DIG_MOD;
    } else {
        dig_check = BORG_DIG;
    }
    if crate::bridge::st().borg.times_twitch as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
        dig_check -= if crate::bridge::st().borg.times_twitch as ::core::ffi::c_int - 10 as ::core::ffi::c_int
            > 19 as ::core::ffi::c_int
        {
            19 as ::core::ffi::c_int
        } else {
            crate::bridge::st().borg.times_twitch as ::core::ffi::c_int - 10 as ::core::ffi::c_int
        };
    }
    if weapon_swap != 0
        && crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize] >= dig_check
        && (crate::bridge::st().items.items[((weapon_swap - 1 as ::core::ffi::c_int)) as usize]).tval
            as ::core::ffi::c_int
            == TV_DIGGING as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_DIG as ::core::ffi::c_int) as usize]
            >= dig_check + 20 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if (feat as ::core::ffi::c_int == FEAT_RUBBLE as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int)
        && crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            == 0
    {
        return true_0 != 0;
    }
    if check_fail {
        if borg_spell_okay_fail(TURN_STONE_TO_MUD, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0
            || borg_spell_okay_fail(SHATTER_STONE, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
            || borg_equips_item(crate::bridge::st().act.stone_to_mud, true_0 != 0) as ::core::ffi::c_int != 0
            || borg_equips_ring(crate::bridge::st().item_val.sv_ring_digging) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    } else if borg_spell_okay(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
        || borg_spell_okay(SHATTER_STONE) as ::core::ffi::c_int != 0
        || borg_equips_item(crate::bridge::st().act.stone_to_mud, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_ring(crate::bridge::st().item_val.sv_ring_digging) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_clear() {
    memcpy(
        borg_data_cost as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    if borg_danger_wipe {
        memset(
            borg_data_know as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_data>() as size_t,
        );
        memset(
            borg_data_icky as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_data>() as size_t,
        );
        borg_danger_wipe = false_0 != 0;
    }
    flow_head = 0 as ::core::ffi::c_int;
    flow_tail = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_spread(
    mut depth: ::core::ffi::c_int,
    mut optimize: bool,
    mut avoid: bool,
    mut tunneling: bool,
    mut stair_idx: ::core::ffi::c_int,
    mut sneak: bool,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut o: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut fear: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int = 0;
    let mut yy: ::core::ffi::c_int = 0;
    let mut xx: ::core::ffi::c_int = 0;
    let mut bad_sneak: bool = false_0 != 0;
    let mut origin_y: ::core::ffi::c_int = 0;
    let mut origin_x: ::core::ffi::c_int = 0;
    let mut twitchy: bool = false_0 != 0;
    origin_y = crate::bridge::st().borg.c.y;
    origin_x = crate::bridge::st().borg.c.x;
    if crate::bridge::st().borg.avoidance as ::core::ffi::c_int
        > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
    {
        twitchy = true_0 != 0;
    }
    if stair_idx >= 0 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
    {
        origin_y = *track_less.y.offset(stair_idx as isize);
        origin_x = *track_less.x.offset(stair_idx as isize);
        optimize = false_0 != 0;
    }
    while flow_head != flow_tail {
        x1 = borg_flow_x[flow_tail as usize] as ::core::ffi::c_int;
        y1 = borg_flow_y[flow_tail as usize] as ::core::ffi::c_int;
        flow_tail += 1;
        if flow_tail == AUTO_FLOW_MAX {
            flow_tail = 0 as ::core::ffi::c_int;
        }
        n = (*borg_data_cost).data[y1 as usize][x1 as usize] as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        if n > o {
            if optimize as ::core::ffi::c_int != 0
                && n > (*borg_data_cost).data[origin_y as usize][origin_x as usize]
                    as ::core::ffi::c_int
            {
                break;
            }
            if n > depth {
                break;
            }
            o = n;
        }
        let mut current_block_55: u64;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut old_head: ::core::ffi::c_int = 0;
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            bad_sneak = false_0 != 0;
            x = x1 + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = y1 + ddy_ddd[i as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x, y)) {
                if !((*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int <= n) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if sneak as ::core::ffi::c_int != 0 && !crate::bridge::st().borg.status.desperate && !twitchy {
                        ii = 0 as ::core::ffi::c_int;
                        while ii < 8 as ::core::ffi::c_int {
                            xx = x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                            yy = y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(xx, yy)) {
                                if (*crate::bridge::grid(xx, yy)).kill != 0 {
                                    bad_sneak = true_0 != 0;
                                    break;
                                }
                            }
                            ii += 1;
                        }
                    }
                    if !bad_sneak {
                        if !(!tunneling
                            && ((*ag).feat as ::core::ffi::c_int
                                >= FEAT_SECRET as ::core::ffi::c_int
                                && (*ag).feat as ::core::ffi::c_int
                                    != FEAT_PASS_RUBBLE as ::core::ffi::c_int
                                && (*ag).feat as ::core::ffi::c_int
                                    != FEAT_LAVA as ::core::ffi::c_int))
                        {
                            if !((*ag).feat as ::core::ffi::c_int
                                == FEAT_PERM as ::core::ffi::c_int)
                            {
                                if !((*ag).feat as ::core::ffi::c_int
                                    == FEAT_LAVA as ::core::ffi::c_int
                                    && crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                        == 0)
                                {
                                    if !((avoid as ::core::ffi::c_int != 0
                                        || crate::bridge::st().borg.status.desperate as ::core::ffi::c_int != 0)
                                        && (*ag).feat as ::core::ffi::c_int
                                            == FEAT_NONE as ::core::ffi::c_int
                                        && !twitchy)
                                    {
                                        if (*ag).kill != 0 {
                                            if crate::bridge::st().borg.status.desperate as ::core::ffi::c_int != 0
                                                || crate::bridge::st().borg.lunal_mode as ::core::ffi::c_int != 0
                                                || crate::bridge::st().borg.munchkin_mode as ::core::ffi::c_int != 0
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                                                != 0
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else if !twitchy
                                                && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                                                    >= 2 as ::core::ffi::c_int
                                                && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] < 5 as ::core::ffi::c_int
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else {
                                                current_block_55 = 790185930182612747;
                                            }
                                        } else {
                                            current_block_55 = 790185930182612747;
                                        }
                                        match current_block_55 {
                                            13242334135786603907 => {}
                                            _ => {
                                                if !(crate::bridge::st().borg.goal.shop as ::core::ffi::c_int
                                                    >= 0 as ::core::ffi::c_int
                                                    && feat_is_shop(
                                                        (*ag).feat as ::core::ffi::c_int,
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0
                                                    && (*ag).store as ::core::ffi::c_int
                                                        != crate::bridge::st().borg.goal.shop as ::core::ffi::c_int
                                                    && y != crate::bridge::st().borg.c.y
                                                    && x != crate::bridge::st().borg.c.x)
                                                {
                                                    if (*ag).trap as ::core::ffi::c_int != 0
                                                        && !(*ag).glyph
                                                        && !twitchy
                                                    {
                                                        if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 60 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] < 30 as ::core::ffi::c_int
                                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] < 20 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if crate::bridge::st().borg.trait_[(BI_DISP as ::core::ffi::c_int) as usize] < 45 as ::core::ffi::c_int
                                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] < 10 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] < 30 as ::core::ffi::c_int
                                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] < 20 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if crate::bridge::st().borg.trait_[(BI_DISM as ::core::ffi::c_int) as usize] < 45 as ::core::ffi::c_int
                                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] < 10 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else {
                                                            current_block_55 = 5141539773904409130;
                                                        }
                                                    } else {
                                                        current_block_55 = 5141539773904409130;
                                                    }
                                                    match current_block_55 {
                                                        13242334135786603907 => {}
                                                        _ => {
                                                            if !((*borg_data_icky).data[y as usize]
                                                                [x as usize]
                                                                != 0)
                                                            {
                                                                if (*borg_data_know).data
                                                                    [y as usize]
                                                                    [x as usize]
                                                                    == 0
                                                                {
                                                                    let mut p: ::core::ffi::c_int =
                                                                        0;
                                                                    (*borg_data_know).data
                                                                        [y as usize]
                                                                        [x as usize] =
                                                                        true_0 as uint8_t;
                                                                    if !crate::bridge::st().borg.status.desperate
                                                                        && !crate::bridge::st().borg.lunal_mode
                                                                        && !crate::bridge::st().borg.munchkin_mode
                                                                        && !crate::bridge::st().borg.status.digging
                                                                    {
                                                                        p = borg_danger(
                                                                            y,
                                                                            x,
                                                                            1 as ::core::ffi::c_int,
                                                                            true_0 != 0,
                                                                            false_0 != 0,
                                                                        );
                                                                        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                                            == 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                                            != 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.mon.scary {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 2 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.mon.unique != 0
                                                                            && crate::bridge::st().borg.status.vault as ::core::ffi::c_int != 0
                                                                            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                                                                == 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
                                                                            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                                                <= 5 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.goal.ignoring {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 5 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.time.now
                                                                            - crate::bridge::st().borg.time.level
                                                                            > 5000 as borg_time
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 25 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 100 as ::core::ffi::c_int;
                                                                        }
                                                                        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if p > fear {
                                                                            (*borg_data_icky)
                                                                                .data
                                                                                [y as usize]
                                                                                [x as usize] =
                                                                                true_0 as uint8_t;
                                                                            current_block_55 = 13242334135786603907;
                                                                        } else {
                                                                            current_block_55 = 16029476503615101993;
                                                                        }
                                                                    } else {
                                                                        current_block_55 =
                                                                            16029476503615101993;
                                                                    }
                                                                } else {
                                                                    current_block_55 =
                                                                        16029476503615101993;
                                                                }
                                                                match current_block_55 {
                                                                    13242334135786603907 => {}
                                                                    _ => {
                                                                        (*borg_data_cost).data
                                                                            [y as usize]
                                                                            [x as usize] =
                                                                            n as uint8_t;
                                                                        borg_flow_x
                                                                            [flow_head as usize] =
                                                                            x as uint8_t;
                                                                        borg_flow_y
                                                                            [flow_head as usize] =
                                                                            y as uint8_t;
                                                                        old_head = flow_head;
                                                                        flow_head += 1;
                                                                        if flow_head
                                                                            == AUTO_FLOW_MAX
                                                                        {
                                                                            flow_head = 0 as ::core::ffi::c_int;
                                                                        }
                                                                        if flow_head == flow_tail {
                                                                            flow_head = old_head;
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
                }
            }
            i += 1;
        }
    }
    flow_tail = 0 as ::core::ffi::c_int;
    flow_head = flow_tail;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_enqueue_grid(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) {
    let mut old_head: ::core::ffi::c_int = 0;
    let mut fear: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0;
    if (*borg_data_icky).data[y as usize][x as usize] != 0 {
        return;
    }
    if (*borg_data_know).data[y as usize][x as usize] == 0 {
        (*borg_data_know).data[y as usize][x as usize] = true_0 as uint8_t;
        p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            == 50 as ::core::ffi::c_int
        {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            != 50 as ::core::ffi::c_int
        {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.mon.scary {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.mon.unique != 0
            && crate::bridge::st().borg.status.vault as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                == 50 as ::core::ffi::c_int
        {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 5 as ::core::ffi::c_int
        {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.goal.ignoring {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 5000 as borg_time {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 25 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
        {
            fear = crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if p > fear
            && !crate::bridge::st().borg.status.desperate
            && !crate::bridge::st().borg.lunal_mode
            && !crate::bridge::st().borg.munchkin_mode
            && !crate::bridge::st().borg.status.digging
        {
            (*borg_data_icky).data[y as usize][x as usize] = true_0 as uint8_t;
            return;
        }
    }
    if (*borg_data_cost).data[y as usize][x as usize] == 0 {
        return;
    }
    (*borg_data_cost).data[y as usize][x as usize] = 0 as uint8_t;
    borg_flow_y[flow_head as usize] = y as uint8_t;
    borg_flow_x[flow_head as usize] = x as uint8_t;
    old_head = flow_head;
    flow_head += 1;
    if flow_head == AUTO_FLOW_MAX {
        flow_head = 0 as ::core::ffi::c_int;
    }
    if flow_head == flow_tail {
        flow_head = old_head;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_commit(
    mut who: *const ::core::ffi::c_char,
    mut why: ::core::ffi::c_int,
) -> bool {
    let mut cost: ::core::ffi::c_int = 0;
    cost = (*borg_data_cost).data[crate::bridge::st().borg.c.y as usize][crate::bridge::st().borg.c.x as usize] as ::core::ffi::c_int;
    if cost >= 250 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if !who.is_null() {
        borg_note(format(
            b"# Flowing toward %s at cost %d\0" as *const u8 as *const ::core::ffi::c_char,
            who,
            cost,
        ));
    }
    memcpy(
        borg_data_flow as *mut ::core::ffi::c_void,
        borg_data_cost as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    crate::bridge::st().borg.goal.type_0 = why as int16_t;
    return true_0 != 0;
}
unsafe extern "C" fn borg_play_step(
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut ag2: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut ch_evt: ui_event = EVENT_EMPTY;
    let mut dir: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.near.breeder {
        ox = -(1 as ::core::ffi::c_int);
        while ox <= 1 as ::core::ffi::c_int {
            oy = -(1 as ::core::ffi::c_int);
            while oy <= 1 as ::core::ffi::c_int {
                if !(oy + crate::bridge::st().borg.c.y == crate::bridge::st().borg.c.y && ox + crate::bridge::st().borg.c.x == crate::bridge::st().borg.c.x) {
                    ag = crate::bridge::grid((ox + crate::bridge::st().borg.c.x), (oy + crate::bridge::st().borg.c.y)) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int != FEAT_OPEN as ::core::ffi::c_int) {
                        if !((*ag).kill != 0) {
                            if !(track_door.num as ::core::ffi::c_int >= 255 as ::core::ffi::c_int)
                            {
                                if !(oy + crate::bridge::st().borg.c.y == y2 && ox + crate::bridge::st().borg.c.x == x2) {
                                    o_y = oy;
                                    o_x = ox;
                                    door_found += 1;
                                }
                            }
                        }
                    }
                }
                oy += 1;
            }
            ox += 1;
        }
        if door_found != 0 {
            dir = borg_goto_dir(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y + o_y, crate::bridge::st().borg.c.x + o_x);
            x = crate::bridge::st().borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
            y = crate::bridge::st().borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
            crate::bridge::st().borg.goal.g.x = x;
            crate::bridge::st().borg.goal.g.y = y;
            borg_note(b"# Closing a door\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress('c' as i32 as keycode_t);
            borg_queue_direction((dir + '0' as i32) as keycode_t);
            i = 0 as ::core::ffi::c_int;
            while i < track_door.num as ::core::ffi::c_int {
                if *track_door.x.offset(i as isize) == x && *track_door.y.offset(i as isize) == y {
                    return true_0 != 0;
                }
                i += 1;
            }
            if i == track_door.num as ::core::ffi::c_int
                && i < track_door.size as ::core::ffi::c_int
            {
                borg_note(
                    b"# Noting the closing of a door.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                track_door.num += 1;
                *track_door.x.offset(i as isize) = x;
                *track_door.y.offset(i as isize) = y;
            }
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.goal.less {
        ag = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
        if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            crate::bridge::st().borg.goal.less = false_0 != 0;
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    dir = borg_goto_dir(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y2, x2);
    if dir == 5 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    x = crate::bridge::st().borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
    y = crate::bridge::st().borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
    ag = crate::bridge::grid(x, y)
        as *mut borg_grid;
    crate::bridge::st().borg.goal.g.x = x;
    crate::bridge::st().borg.goal.g.y = y;
    if (*ag).kill != 0 {
        let mut kill: *mut borg_kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
            != 0
            || crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
                != 0
        {
            return false_0 != 0;
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
        ) as ::core::ffi::c_int
            != 0
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 5 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        borg_note(format(
            b"# Walking into a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).pos.y,
            (*kill).pos.x,
        ));
        if crate::bridge::st().borg.status.no_alter {
            borg_keypress(';' as i32 as keycode_t);
            crate::bridge::st().borg.status.no_alter = false_0 != 0;
        } else {
            borg_keypress('+' as i32 as keycode_t);
        }
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if (*ag).take as ::core::ffi::c_int != 0 && (*borg_takes.offset((*ag).take as isize)).kidx != 0
    {
        let mut take: *mut borg_take = borg_takes.offset((*ag).take as isize) as *mut borg_take;
        if !strstr(
            (*k_info.offset((*take).kidx as isize)).name,
            b"chest\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            && strstr(
                (*k_info.offset((*take).kidx as isize)).name,
                b"Ruined\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x2, y2));
            if o_ptr.is_null() {
                borg_delete_take((*ag).take as ::core::ffi::c_int);
                return false_0 != 0;
            }
            if (*o_ptr).pval as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                && !(*o_ptr).known.is_null()
                && crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize]
                    - (*o_ptr).pval as ::core::ffi::c_int
                    >= crate::bridge::st().core.cfg[(BORG_CHEST_FAIL_TOLERANCE as ::core::ffi::c_int) as usize]
            {
                borg_note(format(
                    b"# Disarming a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_keypress('D' as i32 as keycode_t);
                borg_queue_direction((dir + '0' as i32) as keycode_t);
                return true_0 != 0;
            }
            if ((*o_ptr).pval as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (*o_ptr).known.is_null()
            {
                borg_note(format(
                    b"# Opening a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_keypress('o' as i32 as keycode_t);
                borg_queue_direction((dir + '0' as i32) as keycode_t);
                return true_0 != 0;
            }
        }
        if (*take).orbed as ::core::ffi::c_int == false_0
            && ((*take).tval >= TV_SHOT as ::core::ffi::c_int
                && (*take).tval < TV_STAFF as ::core::ffi::c_int)
        {
            if distance(
                loc(
                    (*take).x as ::core::ffi::c_int,
                    (*take).y as ::core::ffi::c_int,
                ),
                crate::bridge::st().borg.c,
            ) == 1 as ::core::ffi::c_int
            {
                if borg_spell_okay_fail(ORB_OF_DRAINING, 25 as ::core::ffi::c_int) {
                    borg_target(
                        loc(
                            (*take).x as ::core::ffi::c_int,
                            (*take).y as ::core::ffi::c_int,
                        ),
                        false_0 != 0,
                    );
                    borg_spell(ORB_OF_DRAINING);
                    borg_note(
                        b"# Orbing an object to check for cursed item.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    borg_keypress('5' as i32 as keycode_t);
                    (*take).orbed = true_0 != 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < 24 as ::core::ffi::c_int {
                        let mut xx: ::core::ffi::c_int = (*take).x as ::core::ffi::c_int
                            + borg_ddx_ddd[i as usize] as ::core::ffi::c_int;
                        let mut yy: ::core::ffi::c_int = (*take).y as ::core::ffi::c_int
                            + borg_ddy_ddd[i as usize] as ::core::ffi::c_int;
                        if square_in_bounds_fully(cave, loc(xx, yy)) {
                            ag2 = crate::bridge::grid(xx, yy)
                                as *mut borg_grid;
                            if (*ag2).take != 0 {
                                (*borg_takes.offset(
                                    (*crate::bridge::grid(xx, yy)).take as isize,
                                ))
                                .orbed = true_0 != 0;
                            }
                        }
                        i += 1;
                    }
                    return true_0 != 0;
                }
            }
        }
        borg_note(format(
            b"# Walking onto and deleting a '%s' at (%d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            (*k_info.offset((*take).kidx as isize)).name,
            (*take).y as ::core::ffi::c_int,
            (*take).x as ::core::ffi::c_int,
        ));
        borg_delete_take((*ag).take as ::core::ffi::c_int);
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if (*ag).glyph {
        borg_note(format(
            b"# Walking onto a glyph of warding.\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
            == 0
        && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
        && !crate::bridge::st().borg.mon.scary
        && (*ag).trap as ::core::ffi::c_int != 0
    {
        if borg_activate_item(crate::bridge::st().act.disable_traps) {
            borg_note(
                b"# Activation to Disable Traps, Destroy Doors\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*ag).trap = 0 as ::core::ffi::c_int != 0;
            crate::bridge::st().borg.no_rest_prep = 3000 as int16_t;
            borg_target(crate::bridge::st().borg.goal.g, false_0 != 0);
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
            > crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                * 4 as ::core::ffi::c_int
                / 5 as ::core::ffi::c_int
        {
            if borg_spell(DISABLE_TRAPS_DESTROY_DOORS) {
                borg_note(
                    b"# Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*ag).trap = 0 as ::core::ffi::c_int != 0;
                crate::bridge::st().borg.no_rest_prep = 3000 as int16_t;
                return true_0 != 0;
            }
        }
        borg_note(b"# Disarming a trap\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('D' as i32 as keycode_t);
        borg_queue_direction((dir + '0' as i32) as keycode_t);
        (*ag).trap = 0 as ::core::ffi::c_int != 0;
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int {
        if Rand_div(100 as uint32_t) as int32_t == 0 {
            return false_0 != 0;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            ag2 = crate::bridge::grid(x, y) as *mut borg_grid;
            if (*ag2).kill as ::core::ffi::c_int != 0
                && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 15 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                    == 0
            {
                return false_0 != 0;
            }
            i += 1;
        }
        if Rand_div(100 as uint32_t) as int32_t == 0
            || crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int >= 500 as ::core::ffi::c_int
        {
            if borg_spell(DISABLE_TRAPS_DESTROY_DOORS) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.destroy_doors) as ::core::ffi::c_int != 0
            {
                borg_note(
                    b"# Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if borg_spell(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
                || borg_spell(SHATTER_STONE) as ::core::ffi::c_int != 0
                || borg_activate_ring(crate::bridge::st().item_val.sv_ring_digging) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.stone_to_mud) as ::core::ffi::c_int != 0
            {
                borg_note(b"# Melting a door\0" as *const u8 as *const ::core::ffi::c_char);
                borg_keypress((dir + '0' as i32) as keycode_t);
                if track_closed.num != 0 {
                    track_closed.num = 0 as int16_t;
                }
                return true_0 != 0;
            }
        }
        if crate::bridge::st().borg.status.need_alter {
            borg_keypress('+' as i32 as keycode_t);
            crate::bridge::st().borg.status.need_alter = false_0 != 0;
        } else {
            borg_note(b"# Opening a door\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress('o' as i32 as keycode_t);
        }
        borg_queue_direction((dir + '0' as i32) as keycode_t);
        if track_closed.num != 0 {
            track_closed.num = 0 as int16_t;
        }
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] == 0
    {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int <= FEAT_GRANITE as ::core::ffi::c_int
    {
        if (*ag).feat as ::core::ffi::c_int != FEAT_RUBBLE as ::core::ffi::c_int
            && crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_DARK
        {
            return false_0 != 0;
        }
        if !borg_can_dig(false_0 != 0, (*ag).feat) {
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
            return false_0 != 0;
        }
        if borg_spell(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
            || borg_spell(SHATTER_STONE) as ::core::ffi::c_int != 0
            || borg_activate_ring(crate::bridge::st().item_val.sv_ring_digging) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.stone_to_mud) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Melting a wall/etc\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress((dir + '0' as i32) as keycode_t);
            track_vein.num = 0 as int16_t;
            return true_0 != 0;
        }
        if Rand_div(500 as uint32_t) as int32_t <= 5 as int32_t && !crate::bridge::st().borg.status.vault {
            return false_0 != 0;
        }
        borg_note(b"# Digging through wall/etc\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('T' as i32 as keycode_t);
        borg_keypress((dir + '0' as i32) as keycode_t);
        track_vein.num = 0 as int16_t;
        return true_0 != 0;
    }
    if feat_is_shop((*ag).feat as ::core::ffi::c_int) {
        borg_note(format(
            b"# Entering a '%d' shop\0" as *const u8 as *const ::core::ffi::c_char,
            (*ag).store as ::core::ffi::c_int,
        ));
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.status.need_alter {
        borg_keypress('+' as i32 as keycode_t);
        crate::bridge::st().borg.status.need_alter = false_0 != 0;
    }
    borg_keypress((dir + '0' as i32) as keycode_t);
    crate::bridge::st().borg.in_shop = false_0 != 0;
    if ch_evt.type_0 as ::core::ffi::c_uint
        & EVT_SELECT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        ch_evt.type_0 = EVT_KBRD;
    }
    if ch_evt.type_0 as ::core::ffi::c_uint & EVT_MOVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        ch_evt.type_0 = EVT_KBRD;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_old(mut why: ::core::ffi::c_int) -> bool {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == why {
        let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0;
        let mut b_c: ::core::ffi::c_int = 0;
        b_c = (*borg_data_flow).data[crate::bridge::st().borg.c.y as usize][crate::bridge::st().borg.c.x as usize] as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int;
        b_c = b_c - 5 as ::core::ffi::c_int;
        let mut current_block_9: u64;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            c = (*borg_data_flow).data[y as usize][x as usize] as ::core::ffi::c_int
                * 10 as ::core::ffi::c_int;
            if !(c > b_c) {
                if !(x > AUTO_MAX_X - 1 as ::core::ffi::c_int
                    || x < 1 as ::core::ffi::c_int
                    || y > AUTO_MAX_Y - 1 as ::core::ffi::c_int
                    || y < 1 as ::core::ffi::c_int)
                {
                    if c < b_c {
                        b_n = 0 as ::core::ffi::c_int;
                    }
                    if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        == 0 as ::core::ffi::c_int
                        && {
                            b_n += 1;
                            b_n >= 2 as ::core::ffi::c_int
                        }
                        && Rand_div(b_n as uint32_t) as int32_t != 0 as int32_t)
                    {
                        if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            >= 1 as ::core::ffi::c_int
                            && {
                                b_n += 1;
                                b_n >= 2 as ::core::ffi::c_int
                            })
                        {
                            if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_DIGGING
                                && (ddx_ddd[i as usize] as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    || ddy_ddd[i as usize] as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                            {
                                if distance(
                                    crate::bridge::st().borg.c,
                                    loc(
                                        borg_flow_x[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                        borg_flow_y[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                    ),
                                ) <= 2 as ::core::ffi::c_int
                                {
                                    current_block_9 = 17778012151635330486;
                                } else {
                                    current_block_9 = 17860125682698302841;
                                }
                            } else {
                                current_block_9 = 17860125682698302841;
                            }
                            match current_block_9 {
                                17778012151635330486 => {}
                                _ => {
                                    b_i = i;
                                    b_c = c;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if b_i >= 0 as ::core::ffi::c_int {
            x = crate::bridge::st().borg.c.x + ddx_ddd[b_i as usize] as ::core::ffi::c_int;
            y = crate::bridge::st().borg.c.y + ddy_ddd[b_i as usize] as ::core::ffi::c_int;
            if borg_play_step(y, x) {
                return true_0 != 0;
            }
        }
        if crate::bridge::st().borg.goal.type_0 as ::core::ffi::c_int == GOAL_DIGGING
            && crate::bridge::st().borg.c.y == borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            && crate::bridge::st().borg.c.x == borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        {
            crate::bridge::st().borg.time.antisummon = crate::bridge::st().borg.time.now;
        }
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_track(mut track: *mut borg_track, mut size: ::core::ffi::c_int) {
    (*track).num = 0 as int16_t;
    (*track).size = size as int16_t;
    (*track).x = mem_zalloc(
        (size as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    (*track).y = mem_zalloc(
        (size as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_track(mut track: *mut borg_track) {
    (*track).num = 0 as int16_t;
    (*track).size = 0 as int16_t;
    if !(*track).x.is_null() {
        mem_free((*track).x as *mut ::core::ffi::c_void);
        (*track).x = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(*track).y.is_null() {
        mem_free((*track).y as *mut ::core::ffi::c_void);
        (*track).y = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    borg_free_flow();
    borg_data_flow = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_cost = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_hard = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_know = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_icky = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    y = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        x = 0 as ::core::ffi::c_int;
        while x < AUTO_MAX_X {
            (*borg_data_hard).data[y as usize][x as usize] = 255 as uint8_t;
            x += 1;
        }
        y += 1;
    }
    borg_init_track(&raw mut track_step, 100 as ::core::ffi::c_int);
    borg_init_track(&raw mut track_door, 100 as ::core::ffi::c_int);
    borg_init_track(&raw mut track_closed, 100 as ::core::ffi::c_int);
    borg_init_flow_stairs();
    borg_init_flow_glyph();
    borg_init_flow_misc();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow() {
    borg_free_flow_misc();
    borg_free_flow_glyph();
    borg_free_flow_stairs();
    borg_free_track(&raw mut track_closed);
    borg_free_track(&raw mut track_door);
    borg_free_track(&raw mut track_step);
    if !borg_data_flow.is_null() {
        mem_free(borg_data_flow as *mut ::core::ffi::c_void);
        borg_data_flow = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_icky.is_null() {
        mem_free(borg_data_icky as *mut ::core::ffi::c_void);
        borg_data_icky = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_know.is_null() {
        mem_free(borg_data_know as *mut ::core::ffi::c_void);
        borg_data_know = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_hard.is_null() {
        mem_free(borg_data_hard as *mut ::core::ffi::c_void);
        borg_data_hard = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_cost.is_null() {
        mem_free(borg_data_cost as *mut ::core::ffi::c_void);
        borg_data_cost = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_flow.is_null() {
        mem_free(borg_data_flow as *mut ::core::ffi::c_void);
        borg_data_flow = ::core::ptr::null_mut::<borg_data>();
    }
}
