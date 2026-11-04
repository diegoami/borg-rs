pub const BORG_MAGIC_OKAY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BORG_MAGIC_TEST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};

pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
pub use crate::headers::borg_magic_h::{
    borg_magic, borg_magics, borg_spell, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square};
pub use crate::headers::cmd_core_h::{
    arg_CHOICE, arg_DIRECTION, arg_ITEM, arg_NONE, arg_NUMBER, arg_POINT, arg_STRING, arg_TARGET,
    cmd_arg, cmd_arg_data, cmd_arg_type, cmd_code, cmd_context, command, CMD_ACCEPT_CHARACTER,
    CMD_ACTIVATE, CMD_ALTER, CMD_AUTOINSCRIBE, CMD_AUTOPICKUP, CMD_BIRTH_INIT, CMD_BIRTH_RESET,
    CMD_BROWSE_SPELL, CMD_BUY, CMD_BUY_STAT, CMD_CAST, CMD_CHOOSE_CLASS, CMD_CHOOSE_RACE,
    CMD_CLOSE, CMD_COMMAND_MONSTER, CMD_DISARM, CMD_DROP, CMD_EAT, CMD_EXPLORE, CMD_FIRE,
    CMD_GO_DOWN, CMD_GO_UP, CMD_HELP, CMD_HISTORY_CHOICE, CMD_HOLD, CMD_IGNORE, CMD_INSCRIBE,
    CMD_JUMP, CMD_LOADFILE, CMD_NAME_CHOICE, CMD_NAVIGATE_DOWN, CMD_NAVIGATE_UP, CMD_NEWGAME,
    CMD_NULL, CMD_OPEN, CMD_PATHFIND, CMD_PICKUP, CMD_PREV_STATS, CMD_QUAFF, CMD_READ_SCROLL,
    CMD_REFILL, CMD_REFRESH_STATS, CMD_REPEAT, CMD_RESET_STATS, CMD_REST, CMD_RETIRE, CMD_RETRIEVE,
    CMD_ROLL_STATS, CMD_RUN, CMD_SELL, CMD_SELL_STAT, CMD_SLEEP, CMD_SPOIL_ARTIFACT, CMD_SPOIL_MON,
    CMD_SPOIL_MON_BRIEF, CMD_SPOIL_OBJ, CMD_STASH, CMD_STEAL, CMD_STUDY, CMD_TAKEOFF, CMD_THROW,
    CMD_TUNNEL, CMD_UNINSCRIBE, CMD_USE, CMD_USE_ROD, CMD_USE_STAFF, CMD_USE_WAND, CMD_WALK,
    CMD_WIELD, CMD_WIZ_ACQUIRE, CMD_WIZ_ADVANCE, CMD_WIZ_BANISH, CMD_WIZ_CHANGE_ITEM_QUANTITY,
    CMD_WIZ_COLLECT_DISCONNECT_STATS, CMD_WIZ_COLLECT_OBJ_MON_STATS, CMD_WIZ_COLLECT_PIT_STATS,
    CMD_WIZ_CREATE_ALL_ARTIFACT, CMD_WIZ_CREATE_ALL_ARTIFACT_FROM_TVAL, CMD_WIZ_CREATE_ALL_OBJ,
    CMD_WIZ_CREATE_ALL_OBJ_FROM_TVAL, CMD_WIZ_CREATE_ARTIFACT, CMD_WIZ_CREATE_OBJ,
    CMD_WIZ_CREATE_TRAP, CMD_WIZ_CURE_ALL, CMD_WIZ_CURSE_ITEM, CMD_WIZ_DETECT_ALL_LOCAL,
    CMD_WIZ_DETECT_ALL_MONSTERS, CMD_WIZ_DUMP_LEVEL_MAP, CMD_WIZ_EDIT_PLAYER_EXP,
    CMD_WIZ_EDIT_PLAYER_GOLD, CMD_WIZ_EDIT_PLAYER_START, CMD_WIZ_EDIT_PLAYER_STAT,
    CMD_WIZ_HIT_ALL_LOS, CMD_WIZ_INCREASE_EXP, CMD_WIZ_JUMP_LEVEL, CMD_WIZ_LEARN_OBJECT_KINDS,
    CMD_WIZ_MAGIC_MAP, CMD_WIZ_PEEK_NOISE_SCENT, CMD_WIZ_PERFORM_EFFECT, CMD_WIZ_PLAY_ITEM,
    CMD_WIZ_PUSH_OBJECT, CMD_WIZ_QUERY_FEATURE, CMD_WIZ_QUERY_SQUARE_FLAG, CMD_WIZ_QUIT_NO_SAVE,
    CMD_WIZ_RECALL_MONSTER, CMD_WIZ_RERATE, CMD_WIZ_REROLL_ITEM, CMD_WIZ_STAT_ITEM,
    CMD_WIZ_SUMMON_NAMED, CMD_WIZ_SUMMON_RANDOM, CMD_WIZ_TELEPORT_RANDOM, CMD_WIZ_TELEPORT_TO,
    CMD_WIZ_TWEAK_ITEM, CMD_WIZ_WIPE_RECALL, CMD_WIZ_WIZARD_LIGHT, CTX_BIRTH, CTX_DEATH, CTX_GAME,
    CTX_INIT, CTX_STORE,
};
pub use crate::headers::effect_handler_h::{
    effect_handler_context_s, effect_handler_context_t, effect_handler_f, effect_kind,
};
pub use crate::headers::effects_h::{effect_index, EF_MAX, EF_NONE};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::list_effects_h::{
    effect_handler_ACQUIRE, effect_handler_ALTER, effect_handler_ALTER_REALITY, effect_handler_ARC,
    effect_handler_BALL, effect_handler_BANISH, effect_handler_BEAM, effect_handler_BIZARRE,
    effect_handler_BOLT, effect_handler_BOLT_AWARE, effect_handler_BOLT_OR_BEAM,
    effect_handler_BOLT_STATUS, effect_handler_BOLT_STATUS_DAM, effect_handler_BRAND_AMMO,
    effect_handler_BRAND_BOLTS, effect_handler_BRAND_WEAPON, effect_handler_BREATH,
    effect_handler_CLEAR_VALUE, effect_handler_COMMAND, effect_handler_CREATE_ARROWS,
    effect_handler_CREATE_STAIRS, effect_handler_CRUNCH, effect_handler_CURE, effect_handler_CURSE,
    effect_handler_CURSE_ARMOR, effect_handler_CURSE_WEAPON, effect_handler_DAMAGE,
    effect_handler_DARKEN_AREA, effect_handler_DARKEN_LEVEL, effect_handler_DEEP_DESCENT,
    effect_handler_DESTRUCTION, effect_handler_DETECT_DOORS, effect_handler_DETECT_EVIL,
    effect_handler_DETECT_FEARFUL_MONSTERS, effect_handler_DETECT_GOLD,
    effect_handler_DETECT_INVISIBLE_MONSTERS, effect_handler_DETECT_LIVING_MONSTERS,
    effect_handler_DETECT_OBJECTS, effect_handler_DETECT_ORE, effect_handler_DETECT_SOUL,
    effect_handler_DETECT_STAIRS, effect_handler_DETECT_TRAPS,
    effect_handler_DETECT_VISIBLE_MONSTERS, effect_handler_DISENCHANT, effect_handler_DRAIN_LIGHT,
    effect_handler_DRAIN_MANA, effect_handler_DRAIN_STAT, effect_handler_EARTHQUAKE,
    effect_handler_ENCHANT, effect_handler_GAIN_EXP, effect_handler_GAIN_STAT,
    effect_handler_GLYPH, effect_handler_GRANITE, effect_handler_HEAL_HP, effect_handler_IDENTIFY,
    effect_handler_JUMP_AND_BITE, effect_handler_LASH, effect_handler_LIGHT_AREA,
    effect_handler_LIGHT_LEVEL, effect_handler_LINE, effect_handler_LOSE_RANDOM_STAT,
    effect_handler_MAP_AREA, effect_handler_MASS_BANISH, effect_handler_MELEE_BLOWS,
    effect_handler_MON_HEAL_HP, effect_handler_MON_HEAL_KIN, effect_handler_MON_TIMED_INC,
    effect_handler_MOVE_ATTACK, effect_handler_NOURISH, effect_handler_PROBE,
    effect_handler_PROJECT_LOS, effect_handler_PROJECT_LOS_AWARE, effect_handler_RANDOM,
    effect_handler_READ_MINDS, effect_handler_RECALL, effect_handler_RECHARGE,
    effect_handler_REMOVE_CURSE, effect_handler_RESTORE_EXP, effect_handler_RESTORE_MANA,
    effect_handler_RESTORE_STAT, effect_handler_RUBBLE, effect_handler_SCRAMBLE_STATS,
    effect_handler_SELECT, effect_handler_SENSE_GOLD, effect_handler_SENSE_OBJECTS,
    effect_handler_SET_VALUE, effect_handler_SHAPECHANGE, effect_handler_SHORT_BEAM,
    effect_handler_SINGLE_COMBAT, effect_handler_SPHERE, effect_handler_SPOT, effect_handler_STAR,
    effect_handler_STAR_BALL, effect_handler_STRIKE, effect_handler_SUMMON, effect_handler_SWARM,
    effect_handler_SWEEP, effect_handler_TAP_DEVICE, effect_handler_TAP_UNLIFE,
    effect_handler_TELEPORT, effect_handler_TELEPORT_LEVEL, effect_handler_TELEPORT_TO,
    effect_handler_TIMED_DEC, effect_handler_TIMED_INC, effect_handler_TIMED_INC_NO_RES,
    effect_handler_TIMED_SET, effect_handler_TOUCH, effect_handler_TOUCH_AWARE,
    effect_handler_UNSCRAMBLE_STATS, effect_handler_WAKE, effect_handler_WEB,
    effect_handler_WONDER, EF_ACQUIRE, EF_ALTER, EF_ALTER_REALITY, EF_ARC, EF_BALL, EF_BANISH,
    EF_BEAM, EF_BIZARRE, EF_BOLT, EF_BOLT_AWARE, EF_BOLT_OR_BEAM, EF_BOLT_STATUS,
    EF_BOLT_STATUS_DAM, EF_BRAND_AMMO, EF_BRAND_BOLTS, EF_BRAND_WEAPON, EF_BREATH, EF_CLEAR_VALUE,
    EF_COMMAND, EF_CREATE_ARROWS, EF_CREATE_STAIRS, EF_CRUNCH, EF_CURE, EF_CURSE, EF_CURSE_ARMOR,
    EF_CURSE_WEAPON, EF_DAMAGE, EF_DARKEN_AREA, EF_DARKEN_LEVEL, EF_DEEP_DESCENT, EF_DESTRUCTION,
    EF_DETECT_DOORS, EF_DETECT_EVIL, EF_DETECT_FEARFUL_MONSTERS, EF_DETECT_GOLD,
    EF_DETECT_INVISIBLE_MONSTERS, EF_DETECT_LIVING_MONSTERS, EF_DETECT_OBJECTS, EF_DETECT_ORE,
    EF_DETECT_SOUL, EF_DETECT_STAIRS, EF_DETECT_TRAPS, EF_DETECT_VISIBLE_MONSTERS, EF_DISENCHANT,
    EF_DRAIN_LIGHT, EF_DRAIN_MANA, EF_DRAIN_STAT, EF_EARTHQUAKE, EF_ENCHANT, EF_GAIN_EXP,
    EF_GAIN_STAT, EF_GLYPH, EF_GRANITE, EF_HEAL_HP, EF_IDENTIFY, EF_JUMP_AND_BITE, EF_LASH,
    EF_LIGHT_AREA, EF_LIGHT_LEVEL, EF_LINE, EF_LOSE_RANDOM_STAT, EF_MAP_AREA, EF_MASS_BANISH,
    EF_MELEE_BLOWS, EF_MON_HEAL_HP, EF_MON_HEAL_KIN, EF_MON_TIMED_INC, EF_MOVE_ATTACK, EF_NOURISH,
    EF_PROBE, EF_PROJECT_LOS, EF_PROJECT_LOS_AWARE, EF_RANDOM, EF_READ_MINDS, EF_RECALL,
    EF_RECHARGE, EF_REMOVE_CURSE, EF_RESTORE_EXP, EF_RESTORE_MANA, EF_RESTORE_STAT, EF_RUBBLE,
    EF_SCRAMBLE_STATS, EF_SELECT, EF_SENSE_GOLD, EF_SENSE_OBJECTS, EF_SET_VALUE, EF_SHAPECHANGE,
    EF_SHORT_BEAM, EF_SINGLE_COMBAT, EF_SPHERE, EF_SPOT, EF_STAR, EF_STAR_BALL, EF_STRIKE,
    EF_SUMMON, EF_SWARM, EF_SWEEP, EF_TAP_DEVICE, EF_TAP_UNLIFE, EF_TELEPORT, EF_TELEPORT_LEVEL,
    EF_TELEPORT_TO, EF_TIMED_DEC, EF_TIMED_INC, EF_TIMED_INC_NO_RES, EF_TIMED_SET, EF_TOUCH,
    EF_TOUCH_AWARE, EF_UNSCRAMBLE_STATS, EF_WAKE, EF_WEB, EF_WONDER,
};
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::object_h::{
    activation, artifact, chest_trap, curse_data, effect, ego_item, element_info, flavor, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_34, PF_MAX,
};
pub use crate::headers::source_h::{
    source, trap, C2RustUnnamedH_25, C2RustUnnamedH_26, SRC_CHEST_TRAP, SRC_MONSTER, SRC_NONE,
    SRC_OBJECT, SRC_PLAYER, SRC_TRAP,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
static mut effects: [effect_kind; 114] = unsafe {
    [
        effect_kind {
            index: EF_NONE as ::core::ffi::c_int as uint16_t,
            aim: false_0 != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: None,
            desc: ::core::ptr::null::<::core::ffi::c_char>(),
            menu_name: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        effect_kind {
            index: EF_RANDOM as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RANDOM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"randomly \0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DAMAGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"hurt\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_DAMAGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"does %s damage to the player\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_HEAL_HP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"heal\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_HEAL_HP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals %s hitpoints%s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"heal self\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_HEAL_HP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_HEAL_HP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals monster hitpoints\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_HEAL_KIN as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_HEAL_KIN
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals fellow monster hitpoints\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_NOURISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_NOURISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s for %s turns (%s percent)\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CRUNCH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CRUNCH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"crunches\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"cures %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"cure %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_SET as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TIMED_SET
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"administers %s for %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"administer %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_INC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dur\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TIMED_INC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"extends %s for %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"extend %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_INC_NO_RES as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dur\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TIMED_INC_NO_RES
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"extends %s for %s turns (unresistable)\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"extend %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_TIMED_INC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_TIMED_INC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"increases monster %s by %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_DEC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TIMED_DEC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces length of %s by %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"reduce %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GLYPH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GLYPH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"inscribes a glyph beneath you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"inscribe a glyph\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WEB as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WEB
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a web\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"create a web\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"restore %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LOSE_RANDOM_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LOSE_RANDOM_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces a stat other than %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GAIN_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GAIN_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"increases your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_EXP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_EXP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores your experience\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"restore experience\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GAIN_EXP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GAIN_EXP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"grants %d experience points\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_LIGHT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_LIGHT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains your light source\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_MANA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_MANA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains some mana\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_MANA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_MANA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores some mana\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"restore some mana\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_REMOVE_CURSE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_REMOVE_CURSE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"attempts power %s removal of a single curse on an object\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"remove curse\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RECALL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RECALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"returns you from the dungeon or takes you to the dungeon after a short delay\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"recall\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DEEP_DESCENT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DEEP_DESCENT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports you up to five dungeon levels lower than the lowest point you have reached so far\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"descend to the depths\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ALTER_REALITY as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ALTER_REALITY
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a new dungeon level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"alter reality\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MAP_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MAP_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"maps the area around you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"map surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_READ_MINDS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_READ_MINDS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"maps the area around recently detected monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"read minds\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_TRAPS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_TRAPS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects traps nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect traps\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_DOORS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_DOORS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects doors nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect doors\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_STAIRS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_STAIRS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects stairs nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect stairs\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_ORE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_ORE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects veins nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect veins\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SENSE_GOLD as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SENSE_GOLD
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"senses gold nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"sense gold\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_GOLD as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_GOLD
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects gold nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect gold\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SENSE_OBJECTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SENSE_OBJECTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"senses objects nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"sense objects\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_OBJECTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_OBJECTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects objects nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect objects\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_LIVING_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_LIVING_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects living creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect living\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_VISIBLE_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_VISIBLE_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects visible creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect visible\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_INVISIBLE_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_INVISIBLE_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects invisible creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect invisible\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_FEARFUL_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_FEARFUL_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects creatures nearby which are susceptible to fear\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect fearful\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_IDENTIFY as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_IDENTIFY
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"identifies a single unknown rune on a selected item\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"identify\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_EVIL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_EVIL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects evil creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect evil\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_SOUL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_SOUL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects creatures with a spirit nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect souls\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CREATE_STAIRS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CREATE_STAIRS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a staircase beneath your feet\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create stairs\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DISENCHANT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DISENCHANT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"disenchants one of your wielded items\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"disenchant item\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ENCHANT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ENCHANT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"attempts to magically enhance an item\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"enchant item\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RECHARGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_RECHARGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"tries to recharge a wand or staff, destroying the wand or staff on failure\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"recharge\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROJECT_LOS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_PROJECT_LOS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s which are in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROJECT_LOS_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_PROJECT_LOS_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s which are in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ACQUIRE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ACQUIRE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates good items nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create good items\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WAKE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WAKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"awakens all nearby sleeping monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"awaken all\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SUMMON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SUMMON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"summons %s at the current dungeon level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"summon %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BANISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BANISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"removes all of a given creature type from the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"banish\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MASS_BANISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MASS_BANISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"removes all nearby creatures\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"banish all\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROBE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_PROBE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"gives you information on the health and abilities of monsters you can see\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"probe\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"range\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TELEPORT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports %s randomly %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT_TO as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TELEPORT_TO
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports toward a target\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport to target\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TELEPORT_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports you one level up or down\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RUBBLE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RUBBLE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes rubble to fall around you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GRANITE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GRANITE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes a granite wall to fall behind you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DESTRUCTION as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DESTRUCTION
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"destroys an area around you in the shape of a circle radius %d, and blinds you for 1d10+10 turns\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"destroy area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_EARTHQUAKE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_EARTHQUAKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes an earthquake around you of radius %d\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cause earthquake\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LIGHT_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LIGHT_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"completely lights up and magically maps the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"light level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DARKEN_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DARKEN_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"completely darkens up and magically maps the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"darken level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LIGHT_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LIGHT_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"lights up the surrounding area\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"light area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DARKEN_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DARKEN_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"darkens the surrounding area\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"darken area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SPOT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SPOT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, centred on and hitting the player, with full intensity to radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"engulf with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SPHERE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SPHERE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, centred on the player, with full intensity to radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"project %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BALL as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a ball of %s with radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a ball of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BREATH as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BREATH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"breathes a cone of %s with width %d degrees, dealing %s damage at the source\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"breathe a cone of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ARC as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_ARC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"produces a cone of %s with width %d degrees, dealing %s damage at the source\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"produce a cone of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SHORT_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SHORT_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"produces a beam of %s with length %d, dealing %s damage\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"produce a beam of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LASH as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LASH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a beam of %s length %d, dealing damage determined by blows\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"lash with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SWARM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SWARM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a series of %s balls of radius %d, dealing %s damage at the centre of each\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a swarm of %s balls\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STRIKE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STRIKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"strike with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STAR as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STAR
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a line of %s in all directions, each dealing %s damage\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a line of %s in all directions\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STAR_BALL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STAR_BALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires balls of %s with radius %d in all directions, dealing %s damage at the centre of each\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire balls of %s in all directions\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a beam of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a beam of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_OR_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT_OR_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt or beam of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt or beam of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LINE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_LINE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a line of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a line of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ALTER as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ALTER
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a line which %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a line which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_STATUS as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BOLT_STATUS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt which %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_STATUS_DAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT_STATUS_DAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt which %s, dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BOLT_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TOUCH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TOUCH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s on all adjacent squares\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s all adjacent\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TOUCH_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TOUCH_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s on all adjacent squares\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s all adjacent\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE_ARMOR as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURSE_ARMOR
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"curses your worn armor\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"curse armor\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE_WEAPON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURSE_WEAPON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"curses your wielded melee weapon\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"curse weapon\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_WEAPON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_WEAPON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands your wielded melee weapon\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand weapon\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_AMMO as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_AMMO
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands a stack of ammunition\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand ammunition\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_BOLTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_BOLTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands bolts with fire, in an unbalanced fashion\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand bolts\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CREATE_ARROWS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CREATE_ARROWS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"uses a staff to create a stack of arrows\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"use staff for arrows\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TAP_DEVICE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TAP_DEVICE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains magical energy from a staff or wand\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"tap device\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TAP_UNLIFE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TAP_UNLIFE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains %s mana from the closest undead monster, damaging it\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"tap unlife\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SHAPECHANGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SHAPECHANGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"changes the player's shape\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"change shape\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_CURSE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"damages a monster directly\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"curse\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_COMMAND as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_COMMAND
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"takes control of a monster\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"command\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_JUMP_AND_BITE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_JUMP_AND_BITE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"jumps the player to the closest living monster and bites it\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"jump and bite\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MOVE_ATTACK as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_MOVE_ATTACK
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"moves the player up to 4 spaces and executes up to %d melee blows\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"move and attack\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SINGLE_COMBAT as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SINGLE_COMBAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"engages a monster in single combat\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"engage in single combat\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MELEE_BLOWS as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_MELEE_BLOWS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"strikes %d blows against an adjacent monster\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"pummel\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SWEEP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SWEEP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"strikes %d blows against all adjacent monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"sweep\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BIZARRE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BIZARRE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"does bizarre things\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"do bizarre things\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WONDER as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WONDER
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates random and unpredictable effects\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create random effects\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SELECT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SELECT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"selects one of \0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SET_VALUE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SET_VALUE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CLEAR_VALUE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CLEAR_VALUE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SCRAMBLE_STATS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SCRAMBLE_STATS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_UNSCRAMBLE_STATS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_UNSCRAMBLE_STATS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MAX as ::core::ffi::c_int as uint16_t,
            aim: false_0 != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: None,
            desc: ::core::ptr::null::<::core::ffi::c_char>(),
            menu_name: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ]
};
unsafe extern "C" fn borg_can_play_spell(mut as_0: *mut borg_magic) -> bool {
    match (*as_0).spell_enum as ::core::ffi::c_uint {
        40 | 89 | 17 | 68 | 47 | 81 | 101 | 116 | 78 | 110 | 95 | 109 | 108 | 131 | 124 => {
            return false_0 != 0
        }
        _ => {}
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_BRAND_BOLTS as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_CREATE_ARROWS as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_BRAND_AMMO as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_ENCHANT as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_IDENTIFY as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_RECHARGE as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_play_magic(mut bored: bool) -> bool {
    let mut r: ::core::ffi::c_int = 0;
    let mut b_r: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut spell_num: ::core::ffi::c_int = 0;
    let mut b_spell_num: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*(*player).class).magic.total_spells == 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 {
        return false_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).info as ::core::ffi::c_int
        == BORG_DARK
    {
        return false_0 != 0;
    }
    spell_num = (*(*player).class).magic.total_spells - 1 as ::core::ffi::c_int;
    while spell_num >= 0 as ::core::ffi::c_int {
        let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
        if !((borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
        {
            if !((*as_0).status as ::core::ffi::c_int != BORG_MAGIC_OKAY) {
                r = (*as_0).rating as ::core::ffi::c_int;
                if !(!bored && r <= 50 as ::core::ffi::c_int) {
                    if !(r <= 0 as ::core::ffi::c_int) {
                        if !(r <= b_r) {
                            b_r = r;
                            b_spell_num = spell_num;
                        }
                    }
                }
            }
        }
        spell_num -= 1;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUDY as ::core::ffi::c_int as isize)
        != 0
        && b_r > 0 as ::core::ffi::c_int
        && b_spell_num >= 0 as ::core::ffi::c_int
    {
        let mut as_1: *mut borg_magic = borg_magics.offset(b_spell_num as isize) as *mut borg_magic;
        borg_note(format(
            b"# Studying spell/prayer %s.\0" as *const u8 as *const ::core::ffi::c_char,
            (*as_1).name,
        ));
        borg_keypress('G' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset(borg.book_idx[(*as_1).book as usize] as isize) as keycode_t,
        );
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
            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
        ) {
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((*as_1).book_offset as isize) as keycode_t,
            );
        }
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && !borg.munchkin_mode
    {
        return false_0 != 0;
    }
    if !bored {
        return false_0 != 0;
    }
    let mut current_block_37: u64;
    spell_num = (*(*player).class).magic.total_spells - 1 as ::core::ffi::c_int;
    while spell_num >= 0 as ::core::ffi::c_int {
        let mut as_2: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
        if !((borg.book_idx[(*as_2).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
        {
            if !((*as_2).status as ::core::ffi::c_int != BORG_MAGIC_TEST) {
                if borg_can_play_spell(as_2) {
                    if borg.munchkin_mode {
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
                            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                            && (*as_2).spell_enum as ::core::ffi::c_uint
                                == MAGIC_MISSILE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            current_block_37 = 4068382217303356765;
                        } else if !flag_has(
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
                            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
                        ) && (*as_2).spell_enum as ::core::ffi::c_uint
                            == BLESS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            current_block_37 = 4068382217303356765;
                        } else {
                            current_block_37 = 18153031941552419006;
                        }
                    } else {
                        current_block_37 = 18153031941552419006;
                    }
                    match current_block_37 {
                        4068382217303356765 => {}
                        _ => {
                            borg_note(
                                b"# Testing untried spell/prayer\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            if borg_spell((*as_2).spell_enum) {
                                if effects[(*as_2).effect_index as usize].aim as ::core::ffi::c_int
                                    != 0
                                    || (*as_2).effect_index as ::core::ffi::c_int
                                        == EF_RANDOM as ::core::ffi::c_int
                                    || (*as_2).effect_index as ::core::ffi::c_int
                                        == EF_TELEPORT_TO as ::core::ffi::c_int
                                {
                                    borg_keypress('*' as i32 as keycode_t);
                                    borg_keypress('t' as i32 as keycode_t);
                                }
                                if (*as_2).effect_index as ::core::ffi::c_int
                                    == EF_BANISH as ::core::ffi::c_int
                                {
                                    borg_keypress('h' as i32 as keycode_t);
                                }
                                return true_0 != 0;
                            }
                        }
                    }
                }
            }
        }
        spell_num -= 1;
    }
    return false_0 != 0;
}
