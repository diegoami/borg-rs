pub const PY_SPELL_LEARNED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PY_SPELL_WORKED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PY_SPELL_FORGOTTEN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_MAGIC_LOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BORG_MAGIC_HIGH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BORG_MAGIC_OKAY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BORG_MAGIC_TEST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BORG_MAGIC_KNOW: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1;
pub const CLASS_DRUID: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6;
pub const CLASS_RANGER: ::core::ffi::c_int = 7;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_init_h::borg_init_failure;
use crate::headers::borg_io_h::{borg_keypress, borg_note};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_magic_h::{
    borg_magic, borg_spell_rating, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL,
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, no_light, square};
pub use crate::headers::effects_h::{C2RustUnnamedH_45, EF_MAX, EF_NONE};
pub use crate::headers::h_basic_h::errr;
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
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND,
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
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_34, PF_MAX,
};
use crate::headers::player_spell_h::{spell_by_index, spell_okay_to_cast};
use crate::headers::source_h::trap;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::strcmp;
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
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_magics: *mut borg_magic = ::core::ptr::null::<borg_magic>() as *mut borg_magic;
static mut borg_spell_ratings: *mut borg_spell_rating =
    ::core::ptr::null::<borg_spell_rating>() as *mut borg_spell_rating;
static mut borg_spell_ratings_MAGE: [borg_spell_rating; 30] = [
    borg_spell_rating {
        name: b"Magic Missile\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MAGIC_MISSILE,
    },
    borg_spell_rating {
        name: b"Light Room\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: LIGHT_ROOM,
    },
    borg_spell_rating {
        name: b"Find Traps, Doors & Stairs\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: FIND_TRAPS_DOORS_STAIRS,
    },
    borg_spell_rating {
        name: b"Phase Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: PHASE_DOOR,
    },
    borg_spell_rating {
        name: b"Electric Arc\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ELECTRIC_ARC,
    },
    borg_spell_rating {
        name: b"Detect Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_MONSTERS,
    },
    borg_spell_rating {
        name: b"Fire Ball\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FIRE_BALL,
    },
    borg_spell_rating {
        name: b"Recharging\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: RECHARGING,
    },
    borg_spell_rating {
        name: b"Identify Rune\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: IDENTIFY_RUNE,
    },
    borg_spell_rating {
        name: b"Treasure Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: TREASURE_DETECTION,
    },
    borg_spell_rating {
        name: b"Frost Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FROST_BOLT,
    },
    borg_spell_rating {
        name: b"Reveal Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REVEAL_MONSTERS,
    },
    borg_spell_rating {
        name: b"Acid Spray\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ACID_SPRAY,
    },
    borg_spell_rating {
        name: b"Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DISABLE_TRAPS_DESTROY_DOORS,
    },
    borg_spell_rating {
        name: b"Teleport Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: TELEPORT_SELF,
    },
    borg_spell_rating {
        name: b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: TELEPORT_OTHER,
    },
    borg_spell_rating {
        name: b"Resistance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RESISTANCE,
    },
    borg_spell_rating {
        name: b"Tap Magical Energy\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: TAP_MAGICAL_ENERGY,
    },
    borg_spell_rating {
        name: b"Mana Channel\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MANA_CHANNEL,
    },
    borg_spell_rating {
        name: b"Door Creation\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DOOR_CREATION,
    },
    borg_spell_rating {
        name: b"Mana Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MANA_BOLT,
    },
    borg_spell_rating {
        name: b"Teleport Level\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: TELEPORT_LEVEL,
    },
    borg_spell_rating {
        name: b"Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DETECTION,
    },
    borg_spell_rating {
        name: b"Dimension Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DIMENSION_DOOR,
    },
    borg_spell_rating {
        name: b"Thrust Away\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: THRUST_AWAY,
    },
    borg_spell_rating {
        name: b"Shock Wave\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SHOCK_WAVE,
    },
    borg_spell_rating {
        name: b"Explosion\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: EXPLOSION,
    },
    borg_spell_rating {
        name: b"Banishment\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: BANISHMENT,
    },
    borg_spell_rating {
        name: b"Mass Banishment\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: MASS_BANISHMENT,
    },
    borg_spell_rating {
        name: b"Mana Storm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: MANA_STORM,
    },
];
static mut borg_spell_ratings_DRUID: [borg_spell_rating; 27] = [
    borg_spell_rating {
        name: b"Detect Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DETECT_LIFE,
    },
    borg_spell_rating {
        name: b"Fox Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: FOX_FORM,
    },
    borg_spell_rating {
        name: b"Remove Hunger\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REMOVE_HUNGER,
    },
    borg_spell_rating {
        name: b"Stinking Cloud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: STINKING_CLOUD,
    },
    borg_spell_rating {
        name: b"Confuse Monster\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: CONFUSE_MONSTER,
    },
    borg_spell_rating {
        name: b"Slow Monster\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: SLOW_MONSTER,
    },
    borg_spell_rating {
        name: b"Cure Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: CURE_POISON,
    },
    borg_spell_rating {
        name: b"Resist Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 60 as uint8_t,
        spell_enum: RESIST_POISON,
    },
    borg_spell_rating {
        name: b"Turn Stone to Mud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: TURN_STONE_TO_MUD,
    },
    borg_spell_rating {
        name: b"Sense Surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: SENSE_SURROUNDINGS,
    },
    borg_spell_rating {
        name: b"Lightning Strike\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: LIGHTNING_STRIKE,
    },
    borg_spell_rating {
        name: b"Earth Rising\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 70 as uint8_t,
        spell_enum: EARTH_RISING,
    },
    borg_spell_rating {
        name: b"Trance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: TRANCE,
    },
    borg_spell_rating {
        name: b"Mass Sleep\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: MASS_SLEEP,
    },
    borg_spell_rating {
        name: b"Become Pukel-man\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BECOME_PUKEL_MAN,
    },
    borg_spell_rating {
        name: b"Eagle's Flight\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: EAGLES_FLIGHT,
    },
    borg_spell_rating {
        name: b"Bear Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BEAR_FORM,
    },
    borg_spell_rating {
        name: b"Tremor\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: TREMOR,
    },
    borg_spell_rating {
        name: b"Haste Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: HASTE_SELF,
    },
    borg_spell_rating {
        name: b"Revitalize\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: REVITALIZE,
    },
    borg_spell_rating {
        name: b"Rapid Regeneration\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: RAPID_REGENERATION,
    },
    borg_spell_rating {
        name: b"Herbal Curing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: HERBAL_CURING,
    },
    borg_spell_rating {
        name: b"Meteor Swarm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: METEOR_SWARM,
    },
    borg_spell_rating {
        name: b"Rift\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RIFT,
    },
    borg_spell_rating {
        name: b"Ice Storm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ICE_STORM,
    },
    borg_spell_rating {
        name: b"Volcanic Eruption\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 60 as uint8_t,
        spell_enum: VOLCANIC_ERUPTION,
    },
    borg_spell_rating {
        name: b"River of Lightning\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RIVER_OF_LIGHTNING,
    },
];
static mut borg_spell_ratings_PRIEST: [borg_spell_rating; 28] = [
    borg_spell_rating {
        name: b"Call Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: CALL_LIGHT,
    },
    borg_spell_rating {
        name: b"Detect Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_EVIL,
    },
    borg_spell_rating {
        name: b"Minor Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: MINOR_HEALING,
    },
    borg_spell_rating {
        name: b"Bless\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: BLESS,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: HEROISM,
    },
    borg_spell_rating {
        name: b"Orb of Draining\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: ORB_OF_DRAINING,
    },
    borg_spell_rating {
        name: b"Spear of Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SPEAR_OF_LIGHT,
    },
    borg_spell_rating {
        name: b"Dispel Undead\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_UNDEAD,
    },
    borg_spell_rating {
        name: b"Dispel Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_EVIL,
    },
    borg_spell_rating {
        name: b"Protection from Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PROTECTION_FROM_EVIL,
    },
    borg_spell_rating {
        name: b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REMOVE_CURSE,
    },
    borg_spell_rating {
        name: b"Portal\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PORTAL,
    },
    borg_spell_rating {
        name: b"Remembrance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: REMEMBRANCE,
    },
    borg_spell_rating {
        name: b"Word of Recall\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: WORD_OF_RECALL,
    },
    borg_spell_rating {
        name: b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HEALING,
    },
    borg_spell_rating {
        name: b"Restoration\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: RESTORATION,
    },
    borg_spell_rating {
        name: b"Clairvoyance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CLAIRVOYANCE,
    },
    borg_spell_rating {
        name: b"Enchant Weapon\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_WEAPON,
    },
    borg_spell_rating {
        name: b"Enchant Armour\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_ARMOUR,
    },
    borg_spell_rating {
        name: b"Smite Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SMITE_EVIL,
    },
    borg_spell_rating {
        name: b"Glyph of Warding\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: GLYPH_OF_WARDING,
    },
    borg_spell_rating {
        name: b"Demon Bane\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DEMON_BANE,
    },
    borg_spell_rating {
        name: b"Banish Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: BANISH_EVIL,
    },
    borg_spell_rating {
        name: b"Word of Destruction\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: WORD_OF_DESTRUCTION,
    },
    borg_spell_rating {
        name: b"Holy Word\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: HOLY_WORD,
    },
    borg_spell_rating {
        name: b"Spear of Orom\xC3\xAB\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SPEAR_OF_OROME,
    },
    borg_spell_rating {
        name: b"Light of Varda\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: LIGHT_OF_VARDA,
    },
];
static mut borg_spell_ratings_NECROMANCER: [borg_spell_rating; 26] = [
    borg_spell_rating {
        name: b"Nether Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: NETHER_BOLT,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Create Darkness\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: CREATE_DARKNESS,
    },
    borg_spell_rating {
        name: b"Bat Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BAT_FORM,
    },
    borg_spell_rating {
        name: b"Read Minds\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: READ_MINDS,
    },
    borg_spell_rating {
        name: b"Tap Unlife\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TAP_UNLIFE,
    },
    borg_spell_rating {
        name: b"Crush\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: CRUSH,
    },
    borg_spell_rating {
        name: b"Sleep Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SLEEP_EVIL,
    },
    borg_spell_rating {
        name: b"Shadow Shift\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SHADOW_SHIFT,
    },
    borg_spell_rating {
        name: b"Disenchant\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 25 as uint8_t,
        spell_enum: DISENCHANT,
    },
    borg_spell_rating {
        name: b"Frighten\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: FRIGHTEN,
    },
    borg_spell_rating {
        name: b"Vampire Strike\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: VAMPIRE_STRIKE,
    },
    borg_spell_rating {
        name: b"Dispel Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_LIFE,
    },
    borg_spell_rating {
        name: b"Dark Spear\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DARK_SPEAR,
    },
    borg_spell_rating {
        name: b"Warg Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: WARG_FORM,
    },
    borg_spell_rating {
        name: b"Banish Spirits\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: BANISH_SPIRITS,
    },
    borg_spell_rating {
        name: b"Annihilate\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: ANNIHILATE,
    },
    borg_spell_rating {
        name: b"Grond's Blow\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: GRONDS_BLOW,
    },
    borg_spell_rating {
        name: b"Unleash Chaos\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: UNLEASH_CHAOS,
    },
    borg_spell_rating {
        name: b"Fume of Mordor\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FUME_OF_MORDOR,
    },
    borg_spell_rating {
        name: b"Storm of Darkness\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: STORM_OF_DARKNESS,
    },
    borg_spell_rating {
        name: b"Power Sacrifice\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: POWER_SACRIFICE,
    },
    borg_spell_rating {
        name: b"Zone of Unmagic\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: ZONE_OF_UNMAGIC,
    },
    borg_spell_rating {
        name: b"Vampire Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: VAMPIRE_FORM,
    },
    borg_spell_rating {
        name: b"Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: CURSE,
    },
    borg_spell_rating {
        name: b"Command\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: COMMAND,
    },
];
static mut borg_spell_ratings_PALADIN: [borg_spell_rating; 16] = [
    borg_spell_rating {
        name: b"Bless\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BLESS,
    },
    borg_spell_rating {
        name: b"Detect Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_EVIL,
    },
    borg_spell_rating {
        name: b"Call Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CALL_LIGHT,
    },
    borg_spell_rating {
        name: b"Minor Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MINOR_HEALING,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: HEROISM,
    },
    borg_spell_rating {
        name: b"Protection from Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PROTECTION_FROM_EVIL,
    },
    borg_spell_rating {
        name: b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: REMOVE_CURSE,
    },
    borg_spell_rating {
        name: b"Word of Recall\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: WORD_OF_RECALL,
    },
    borg_spell_rating {
        name: b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HEALING,
    },
    borg_spell_rating {
        name: b"Clairvoyance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CLAIRVOYANCE,
    },
    borg_spell_rating {
        name: b"Smite Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: SMITE_EVIL,
    },
    borg_spell_rating {
        name: b"Demon Bane\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: DEMON_BANE,
    },
    borg_spell_rating {
        name: b"Enchant Weapon\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_WEAPON,
    },
    borg_spell_rating {
        name: b"Enchant Armour\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ENCHANT_ARMOUR,
    },
    borg_spell_rating {
        name: b"Single Combat\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SINGLE_COMBAT,
    },
];
static mut borg_spell_ratings_ROGUE: [borg_spell_rating; 10] = [
    borg_spell_rating {
        name: b"Detect Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_MONSTERS,
    },
    borg_spell_rating {
        name: b"Phase Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: PHASE_DOOR,
    },
    borg_spell_rating {
        name: b"Object Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: OBJECT_DETECTION,
    },
    borg_spell_rating {
        name: b"Detect Stairs\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: DETECT_STAIRS,
    },
    borg_spell_rating {
        name: b"Recharging\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: RECHARGING,
    },
    borg_spell_rating {
        name: b"Reveal Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REVEAL_MONSTERS,
    },
    borg_spell_rating {
        name: b"Teleport Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: TELEPORT_SELF,
    },
    borg_spell_rating {
        name: b"Hit and Run\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 15 as uint8_t,
        spell_enum: HIT_AND_RUN,
    },
    borg_spell_rating {
        name: b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TELEPORT_OTHER,
    },
    borg_spell_rating {
        name: b"Teleport Level\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: TELEPORT_LEVEL,
    },
];
static mut borg_spell_ratings_RANGER: [borg_spell_rating; 11] = [
    borg_spell_rating {
        name: b"Remove Hunger\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: REMOVE_HUNGER,
    },
    borg_spell_rating {
        name: b"Detect Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_LIFE,
    },
    borg_spell_rating {
        name: b"Herbal Curing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HERBAL_CURING,
    },
    borg_spell_rating {
        name: b"Resist Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: RESIST_POISON,
    },
    borg_spell_rating {
        name: b"Turn Stone to Mud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TURN_STONE_TO_MUD,
    },
    borg_spell_rating {
        name: b"Sense Surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SENSE_SURROUNDINGS,
    },
    borg_spell_rating {
        name: b"Cover Tracks\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 25 as uint8_t,
        spell_enum: COVER_TRACKS,
    },
    borg_spell_rating {
        name: b"Create Arrows\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CREATE_ARROWS,
    },
    borg_spell_rating {
        name: b"Haste Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HASTE_SELF,
    },
    borg_spell_rating {
        name: b"Decoy\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: DECOY,
    },
    borg_spell_rating {
        name: b"Brand Ammunition\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BRAND_AMMUNITION,
    },
];
static mut borg_spell_ratings_BLACKGUARD: [borg_spell_rating; 15] = [
    borg_spell_rating {
        name: b"Seek Battle\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: SEEK_BATTLE,
    },
    borg_spell_rating {
        name: b"Berserk Strength\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BERSERK_STRENGTH,
    },
    borg_spell_rating {
        name: b"Whirlwind Attack\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: WHIRLWIND_ATTACK,
    },
    borg_spell_rating {
        name: b"Shatter Stone\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SHATTER_STONE,
    },
    borg_spell_rating {
        name: b"Leap into Battle\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: LEAP_INTO_BATTLE,
    },
    borg_spell_rating {
        name: b"Grim Purpose\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: GRIM_PURPOSE,
    },
    borg_spell_rating {
        name: b"Maim Foe\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: MAIM_FOE,
    },
    borg_spell_rating {
        name: b"Howl of the Damned\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: HOWL_OF_THE_DAMNED,
    },
    borg_spell_rating {
        name: b"Relentless Taunting\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: RELENTLESS_TAUNTING,
    },
    borg_spell_rating {
        name: b"Venom\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: VENOM,
    },
    borg_spell_rating {
        name: b"Werewolf Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: WEREWOLF_FORM,
    },
    borg_spell_rating {
        name: b"Bloodlust\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BLOODLUST,
    },
    borg_spell_rating {
        name: b"Unholy Reprieve\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: UNHOLY_REPRIEVE,
    },
    borg_spell_rating {
        name: b"Forceful Blow\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: FORCEFUL_BLOW,
    },
    borg_spell_rating {
        name: b"Quake\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: QUAKE,
    },
];
#[no_mangle]
pub unsafe extern "C" fn borg_spell_stat() -> ::core::ffi::c_int {
    if borg_can_cast() {
        let mut spell: *mut class_spell = (*(*(*player).class)
            .magic
            .books
            .offset(0 as ::core::ffi::c_int as isize))
        .spells
        .offset(0 as ::core::ffi::c_int as isize)
            as *mut class_spell;
        if !spell.is_null() {
            return (*(*spell).realm).stat;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_can_cast() -> bool {
    return (*(*player).class).magic.total_spells != 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_primarily_caster() -> bool {
    return (*(*player).class).magic.num_books > 3 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_heroism_level() -> ::core::ffi::c_int {
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PRIEST {
        return 20 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PALADIN {
        return 15 as ::core::ffi::c_int;
    }
    return 99 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_book_num(mut sval: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if !borg_can_cast() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut book_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book_num < (*(*player).class).magic.num_books {
        if (*(*(*player).class).magic.books.offset(book_num as isize)).sval == sval {
            return book_num;
        }
        book_num += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_is_dungeon_book(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> bool {
    match tval {
        31 | 30 | 32 | 33 | 34 => {}
        _ => return false_0 != 0,
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*player).class).magic.num_books {
        let mut book: class_book = *(*(*player).class).magic.books.offset(i as isize);
        if tval == book.tval && sval == book.sval && book.dungeon as ::core::ffi::c_int != 0 {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_spell_entry(
    mut book: ::core::ffi::c_int,
    mut entry: ::core::ffi::c_int,
) -> *mut borg_magic {
    let mut entry_in_book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < (*(*player).class).magic.total_spells {
        if (*borg_magics.offset(spell_num as isize)).book == book {
            if entry_in_book == entry {
                return borg_magics.offset(spell_num as isize) as *mut borg_magic;
            }
            entry_in_book += 1;
        }
        spell_num += 1;
    }
    return ::core::ptr::null_mut::<borg_magic>();
}
unsafe extern "C" fn borg_get_spell_number(spell: borg_spells) -> ::core::ffi::c_int {
    if borg_magics.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut total_spells: ::core::ffi::c_int = (*(*player).class).magic.total_spells;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < total_spells {
        if (*borg_magics.offset(spell_num as isize)).spell_enum as ::core::ffi::c_uint
            == spell as ::core::ffi::c_uint
        {
            return spell_num;
        }
        spell_num += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_spell_power(spell: borg_spells) -> ::core::ffi::c_int {
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    return (*as_0).power as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_legal(spell: borg_spells) -> bool {
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if (borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if ((*borg_magics.offset(spell_num as isize)).status as ::core::ffi::c_int) < BORG_MAGIC_TEST {
        return false_0 != 0;
    }
    if (*borg_magics.offset(spell_num as isize)).power as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_spell_has_effect(
    mut spell_num: ::core::ffi::c_int,
    mut effect: uint16_t,
) -> bool {
    let mut cspell: *const class_spell = spell_by_index(player, spell_num);
    let mut eff: *mut effect = (*cspell).effect;
    while !eff.is_null() {
        if (*eff).index as ::core::ffi::c_int == effect as ::core::ffi::c_int {
            return true_0 != 0;
        }
        eff = (*eff).next;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_okay(spell: borg_spells) -> bool {
    let mut reserve_mana: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if no_light(player) {
        return false_0 != 0;
    }
    match *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) {
        CLASS_MAGE => {
            reserve_mana = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            reserve_mana = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            reserve_mana = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            reserve_mana = 10 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            reserve_mana = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            reserve_mana = 20 as ::core::ffi::c_int;
        }
        CLASS_BLACKGUARD => {
            reserve_mana = 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 35 as ::core::ffi::c_int
    {
        reserve_mana = 0 as ::core::ffi::c_int;
    }
    if !borg_spell_legal(spell) {
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
    if (*as_0).power as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    if (*borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
        - (*as_0).power as ::core::ffi::c_int)
        < reserve_mana
    {
        if borg_spell_has_effect(spell_num, EF_NOURISH as ::core::ffi::c_int as uint16_t) {
            return true_0 != 0;
        }
        if borg_spell_has_effect(spell_num, EF_TELEPORT as ::core::ffi::c_int as uint16_t) {
            return true_0 != 0;
        }
        if MAGIC_MISSILE as ::core::ffi::c_int as ::core::ffi::c_uint
            == spell as ::core::ffi::c_uint
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                <= 35 as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_fail_rate(spell: borg_spells) -> ::core::ffi::c_int {
    let mut chance: ::core::ffi::c_int = 0;
    let mut minfail: ::core::ffi::c_int = 0;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return 100 as ::core::ffi::c_int;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    chance = (*as_0).sfail as ::core::ffi::c_int;
    chance -= 3 as ::core::ffi::c_int
        * (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - (*as_0).level as ::core::ffi::c_int);
    chance -= *borg.trait_0.offset(BI_FAIL1 as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 20 as ::core::ffi::c_int;
    }
    minfail = *borg.trait_0.offset(BI_FAIL2 as ::core::ffi::c_int as isize);
    if !flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_ZERO_FAIL as ::core::ffi::c_int,
    ) {
        if minfail < 5 as ::core::ffi::c_int {
            minfail = 5 as ::core::ffi::c_int;
        }
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
        && (*crate::bridge::grid(borg.c.x, borg.c.y)).info as ::core::ffi::c_int
            & BORG_LIGHT
            != 0
    {
        chance += 25 as ::core::ffi::c_int;
    }
    if chance < minfail {
        chance = minfail;
    }
    if chance > 50 as ::core::ffi::c_int {
        chance = 50 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 25 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 15 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
        != 0
    {
        chance *= 2 as ::core::ffi::c_int;
    }
    if chance > 95 as ::core::ffi::c_int {
        chance = 95 as ::core::ffi::c_int;
    }
    return chance;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_okay_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell_okay(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_legal_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell_legal(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell(spell: borg_spells) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if !borg_spell_okay(spell) {
        return false_0 != 0;
    }
    i = borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Casting %s (%d,%d).\0" as *const u8 as *const ::core::ffi::c_char,
        (*as_0).name,
        i,
        (*as_0).book_offset,
    ));
    borg_keypress('m' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
            .offset((*as_0).book_offset as isize) as keycode_t,
    );
    (*as_0).times += 1;
    return true_0 != 0;
}
unsafe extern "C" fn borg_cheat_spell(mut book_num: ::core::ffi::c_int) {
    let mut book: *mut class_book =
        (*(*player).class).magic.books.offset(book_num as isize) as *mut class_book;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < (*book).num_spells {
        let mut cspell: *mut class_spell =
            (*book).spells.offset(spell_num as isize) as *mut class_spell;
        let mut as_0: *mut borg_magic =
            borg_magics.offset((*cspell).sidx as isize) as *mut borg_magic;
        if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_FORGOTTEN
            != 0
        {
            (*as_0).status = BORG_MAGIC_LOST as uint8_t;
        } else if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < (*as_0).level as ::core::ffi::c_int
        {
            (*as_0).status = BORG_MAGIC_HIGH as uint8_t;
        } else if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_LEARNED
            == 0
        {
            (*as_0).status = BORG_MAGIC_OKAY as uint8_t;
        } else if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_WORKED
            == 0
        {
            (*as_0).status = BORG_MAGIC_TEST as uint8_t;
        } else {
            (*as_0).status = BORG_MAGIC_KNOW as uint8_t;
        }
        spell_num += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_cheat_spells() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        borg.book_idx[i as usize] = -(1 as ::core::ffi::c_int) as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut book_num: ::core::ffi::c_int = 0;
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        book_num = 0 as ::core::ffi::c_int;
        while book_num < (*(*player).class).magic.num_books {
            let mut book: class_book = *(*(*player).class).magic.books.offset(book_num as isize);
            if (*item).tval as ::core::ffi::c_int == book.tval
                && (*item).sval as ::core::ffi::c_int == book.sval
            {
                borg.book_idx[book_num as usize] = i as int16_t;
                break;
            } else {
                book_num += 1;
            }
        }
        i += 1;
    }
    if !borg_can_cast() {
        return;
    }
    let mut book_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book_idx < 8 as ::core::ffi::c_int {
        i = borg.book_idx[book_idx as usize] as ::core::ffi::c_int;
        if i >= 0 as ::core::ffi::c_int {
            borg_cheat_spell(book_idx);
        }
        book_idx += 1;
    }
}
unsafe extern "C" fn borg_get_book_offset(mut index: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut magic: *const class_magic = &raw const (*(*player).class).magic;
    if index < 0 as ::core::ffi::c_int || index >= (*magic).total_spells {
        return -(1 as ::core::ffi::c_int);
    }
    while (count + (*(*magic).books.offset(book as isize)).num_spells - 1 as ::core::ffi::c_int)
        < index
    {
        let fresh0 = book;
        book = book + 1;
        count += (*(*magic).books.offset(fresh0 as isize)).num_spells;
    }
    return index - count;
}
unsafe extern "C" fn borg_init_spell(
    mut spells: *mut borg_magic,
    mut spell_num: ::core::ffi::c_int,
) {
    let mut spell: *mut borg_magic = spells.offset(spell_num as isize) as *mut borg_magic;
    let mut cspell: *const class_spell = spell_by_index(player, spell_num);
    if strcmp(
        (*cspell).name,
        (*borg_spell_ratings.offset(spell_num as isize)).name,
    ) != 0
    {
        borg_note(format(
            b"**STARTUP FAILURE** spell definition mismatch. <%s> not the same as <%s>\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*cspell).name,
            (*borg_spell_ratings.offset(spell_num as isize)).name,
        ));
        borg_init_failure = true_0 != 0;
        return;
    }
    (*spell).rating = (*borg_spell_ratings.offset(spell_num as isize)).rating;
    (*spell).name = (*borg_spell_ratings.offset(spell_num as isize)).name;
    (*spell).spell_enum = (*borg_spell_ratings.offset(spell_num as isize)).spell_enum;
    (*spell).level = (*cspell).slevel as uint8_t;
    (*spell).book_offset = borg_get_book_offset((*cspell).sidx);
    (*spell).effect_index = (*(*cspell).effect).index;
    (*spell).power = (*cspell).smana as uint8_t;
    (*spell).sfail = (*cspell).sfail as uint8_t;
    (*spell).status = spell_okay_to_cast(player, spell_num) as uint8_t;
    (*spell).times = 0 as ::core::ffi::c_int as int32_t;
    (*spell).book = (*cspell).bidx;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepare_book_info() {
    match (*(*player).class).cidx {
        1 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_MAGE as *mut borg_spell_rating;
        }
        2 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_DRUID as *mut borg_spell_rating;
        }
        3 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_PRIEST as *mut borg_spell_rating;
        }
        4 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_NECROMANCER as *mut borg_spell_rating;
        }
        5 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_PALADIN as *mut borg_spell_rating;
        }
        6 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_ROGUE as *mut borg_spell_rating;
        }
        7 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_RANGER as *mut borg_spell_rating;
        }
        8 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_BLACKGUARD as *mut borg_spell_rating;
        }
        _ => {
            borg_spell_ratings = ::core::ptr::null_mut::<borg_spell_rating>();
            return;
        }
    }
    if !borg_magics.is_null() {
        mem_free(borg_magics as *mut ::core::ffi::c_void);
    }
    borg_magics = mem_zalloc(
        ((*(*player).class).magic.total_spells as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_magic>() as size_t),
    ) as *mut borg_magic;
    let mut spell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell < (*(*player).class).magic.total_spells {
        borg_init_spell(borg_magics, spell);
        spell += 1;
    }
}
