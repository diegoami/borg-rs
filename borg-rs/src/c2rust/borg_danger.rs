pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


pub use crate::headers::borg_danger_h::{
    BORG_MONBLOW, MONBLOW_ACID, MONBLOW_BLACK_BREATH, MONBLOW_BLIND, MONBLOW_COLD, MONBLOW_CONFUSE,
    MONBLOW_DISENCHANT, MONBLOW_DRAIN_CHARGES, MONBLOW_EAT_FOOD, MONBLOW_EAT_GOLD,
    MONBLOW_EAT_ITEM, MONBLOW_EAT_LIGHT, MONBLOW_ELEC, MONBLOW_EXP_10, MONBLOW_EXP_20,
    MONBLOW_EXP_40, MONBLOW_EXP_80, MONBLOW_FIRE, MONBLOW_HALLU, MONBLOW_HURT, MONBLOW_LOSE_ALL,
    MONBLOW_LOSE_CON, MONBLOW_LOSE_DEX, MONBLOW_LOSE_INT, MONBLOW_LOSE_STR, MONBLOW_LOSE_WIS,
    MONBLOW_NONE, MONBLOW_PARALYZE, MONBLOW_POISON, MONBLOW_SHATTER, MONBLOW_TERRIFY,
    MONBLOW_UNDEFINED,
};
use crate::headers::borg_fight_attack_h::{
    borg_mon_blow_effect, borg_tp_other_index, borg_tp_other_n,
};
use crate::headers::borg_flow_glyph_h::track_glyph;
pub use crate::headers::borg_flow_h::borg_track;
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_kills_nxt};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
pub use crate::headers::borg_item_h::{borg_item};
pub use crate::headers::borg_magic_h::{
    borg_spell_legal, borg_spell_stat, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_projection_h::{borg_distance, borg_projectable, borg_projectable_pure};
pub use crate::headers::borg_trait_h::{
    borg_attacking, borg_confuse_spell, borg_create_door, borg_crush_spell, borg_fear_mon_spell,
    borg_on_glyph, borg_sleep_spell, borg_sleep_spell_ii, borg_slow_spell, C2RustUnnamedH_8,
    BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL,
    BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD,
    BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL, BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID,
    BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID, BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES,
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
    cave, chunk, connector, ddx_ddd, ddy_ddd, heatmap, monster_group, square, square_in_bounds,
    square_in_bounds_fully, square_isvault, trap, C2RustUnnamedH_18, FEAT_MAX,
};
use crate::headers::game_world_h::extract_energy;
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
pub use crate::headers::list_object_flags_h::{
    OF_AFRAID, OF_AGGRAVATE, OF_BLESSED, OF_BURNS_OUT, OF_DIG_1, OF_DIG_2, OF_DIG_3, OF_DRAIN_EXP,
    OF_EXPLODE, OF_FEATHER, OF_FRAGILE, OF_FREE_ACT, OF_HOLD_LIFE, OF_IMPACT, OF_IMPAIR_HP,
    OF_IMPAIR_MANA, OF_LIGHT_2, OF_LIGHT_3, OF_MAX, OF_MULTIPLY_WEIGHT, OF_NO_FUEL, OF_NO_TELEPORT,
    OF_PROT_BLIND, OF_PROT_CONF, OF_PROT_FEAR, OF_PROT_STUN, OF_REGEN, OF_SEE_INVIS,
    OF_SLOW_DIGEST, OF_STICKY, OF_SUST_CON, OF_SUST_DEX, OF_SUST_INT, OF_SUST_STR, OF_SUST_WIS,
    OF_TAKES_FUEL, OF_TELEPATHY, OF_THROWING, OF_TRAP_IMMUNE,
};
pub use crate::headers::list_stats_h::{STAT_CON, STAT_DEX, STAT_INT, STAT_STR, STAT_WIS};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
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
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_31, OF_NONE};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
use crate::headers::player_calcs_h::adj_dex_safe;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_23, STAT_MAX,
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
pub static mut borg_fear_region: [[uint16_t; 19]; 7] = [[0; 19]; 7];
#[no_mangle]
pub static mut borg_fear_monsters: [[uint16_t; 199]; 67] = [[0; 199]; 67];
#[no_mangle]
pub static mut borg_danger_wipe: bool = false_0 != 0;
unsafe extern "C" fn borg_danger_physical(
    mut i: ::core::ffi::c_int,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pfe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut power: ::core::ffi::c_int = 0;
    let mut chance: ::core::ffi::c_int = 0;
    let mut ac: int16_t = crate::bridge::st().borg.trait_[(BI_ARMOR as ::core::ffi::c_int) as usize] as int16_t;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if crate::bridge::st().borg.temp.shield {
        ac = (ac as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as int16_t;
    }
    if crate::bridge::st().borg.temp.prot_from_evil as ::core::ffi::c_int != 0
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
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= (*r_ptr).level
    {
        pfe = 1 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 1000 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).mon_blows_max as ::core::ffi::c_int {
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut effect: *mut blow_effect = (*(*r_ptr).blow.offset(k as isize)).effect;
        let mut method: *mut blow_method = (*(*r_ptr).blow.offset(k as isize)).method;
        let mut d_dice: ::core::ffi::c_int = (*(*r_ptr).blow.offset(k as isize)).dice.dice;
        let mut d_side: ::core::ffi::c_int = (*(*r_ptr).blow.offset(k as isize)).dice.sides;
        power = 0 as ::core::ffi::c_int;
        if method.is_null() {
            break;
        }
        match borg_mon_blow_effect((*effect).name) {
            1 => {
                z = d_dice * d_side;
                if d_side < 3 as ::core::ffi::c_int && z > d_dice * d_side {
                    n += 200 as ::core::ffi::c_int;
                }
                if d_side < 3 as ::core::ffi::c_int && d_dice > 5 as ::core::ffi::c_int {
                    n += 400 as ::core::ffi::c_int;
                }
                power = 60 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            2 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0) {
                    if !crate::bridge::st().borg.temp.res_pois {
                        z += 10 as ::core::ffi::c_int;
                        if pfe != 0 && !borg_attacking {
                            z /= 2 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            3 => {
                z = d_dice * d_side;
                power = 20 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0) {
                    z += 500 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            4 => {
                z = d_dice * d_side;
                z += 20 as ::core::ffi::c_int;
                power = 15 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            5 => {
                z = d_dice * d_side;
                if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 5 as ::core::ffi::c_int
                {
                    z += 50 as ::core::ffi::c_int;
                }
                power = 5 as ::core::ffi::c_int;
                if !(100 as ::core::ffi::c_int
                    <= adj_dex_safe[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
                        as usize]
                        + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize])
                {
                    if !(crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                        < 100 as ::core::ffi::c_int)
                    {
                        if !(crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                            > 100000 as ::core::ffi::c_int)
                        {
                            z += 5 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            6 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(100 as ::core::ffi::c_int
                    <= adj_dex_safe[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
                        as usize]
                        + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize])
                {
                    z += 5 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            7 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                    > 5 as ::core::ffi::c_int)
                {
                    z += 5 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            8 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !((crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)) as usize])
                .timeout
                    == 0
                    || flag_has(
                        &raw mut (crate::bridge::st().items.items[(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)) as usize])
                        .flags as *mut bitflag,
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
                        != 0)
                {
                    if !(crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize]
                        > 5 as ::core::ffi::c_int)
                    {
                        z += 5 as ::core::ffi::c_int;
                        if pfe != 0 && !borg_attacking {
                            z /= 2 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            9 => {
                if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                    z = d_dice * d_side;
                    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z += 200 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            10 => {
                if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            11 => {
                if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            12 => {
                if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            13 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    z += 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE {
                        z += 75 as ::core::ffi::c_int;
                    }
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            14 => {
                z = d_dice * d_side;
                power = 10 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0) {
                    z += 200 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE {
                        z += 200 as ::core::ffi::c_int;
                    }
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            15 => {
                z = d_dice * d_side;
                power = 10 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] != 0) {
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            16 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0) {
                    z += 200 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            17 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
                                        < 10 as ::core::ffi::c_int
                                    {
                                        z += 100 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            20 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                z += 150 as ::core::ffi::c_int;
                                if crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
                                    < 10 as ::core::ffi::c_int
                                {
                                    z += 100 as ::core::ffi::c_int;
                                }
                                if pfe != 0 && !borg_attacking {
                                    z /= 2 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
            21 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize]
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
                                        < 8 as ::core::ffi::c_int
                                    {
                                        z += 100 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            18 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize]
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if borg_spell_stat() == STAT_INT as ::core::ffi::c_int {
                                        z += 50 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            19 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize]
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                z += 150 as ::core::ffi::c_int;
                                if borg_spell_stat() == STAT_WIS as ::core::ffi::c_int {
                                    z += 50 as ::core::ffi::c_int;
                                }
                                if pfe != 0 && !borg_attacking {
                                    z /= 2 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
            22 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
            }
            23 => {
                z = d_dice * d_side;
                z -=
                    z * (if (ac as ::core::ffi::c_int) < 150 as ::core::ffi::c_int {
                        ac as ::core::ffi::c_int
                    } else {
                        150 as ::core::ffi::c_int
                    }) / 250 as ::core::ffi::c_int;
                power = 60 as ::core::ffi::c_int;
                z += 150 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            24 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        == 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 100 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            25 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 150 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            26 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 200 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            27 => {
                z = d_dice * d_side;
                if !(crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 250 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            28 => {
                z = d_dice * d_side;
                z += 250 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        z -= crate::bridge::st().borg.trait_[(BI_DAM_RED as ::core::ffi::c_int) as usize];
        if z < 0 as ::core::ffi::c_int {
            z = 0 as ::core::ffi::c_int;
        }
        if !full_damage {
            if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                || (*r_ptr).level + power > 0 as ::core::ffi::c_int
            {
                chance = 150 as ::core::ffi::c_int
                    - (ac as ::core::ffi::c_int * 300 as ::core::ffi::c_int
                        / 4 as ::core::ffi::c_int
                        / 1 as ::core::ffi::c_int
                        + ((*r_ptr).level + power) * 3 as ::core::ffi::c_int);
            } else {
                chance = -(1 as ::core::ffi::c_int);
            }
            if chance < 5 as ::core::ffi::c_int {
                chance = 5 as ::core::ffi::c_int;
            }
            z = z * chance / 100 as ::core::ffi::c_int;
        }
        n += z;
        k += 1;
    }
    return n;
}
unsafe extern "C" fn borg_danger_spell(
    mut i: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut average: bool,
) -> ::core::ffi::c_int {
    let mut q: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pfe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut glyph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut glyph_check: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spot_x: ::core::ffi::c_int = 0;
    let mut spot_y: ::core::ffi::c_int = 0;
    let mut spot_safe: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut hp: ::core::ffi::c_int = 0;
    let mut total_dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut av: ::core::ffi::c_int = 0;
    let mut bolt: bool = false_0 != 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if crate::bridge::st().borg.temp.prot_from_evil as ::core::ffi::c_int != 0
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
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= (*r_ptr).level
    {
        pfe = 1 as ::core::ffi::c_int;
    }
    if borg_on_glyph {
        glyph = 1 as ::core::ffi::c_int;
    } else if track_glyph.num != 0 {
        glyph_check = 0 as ::core::ffi::c_int;
        while glyph_check < track_glyph.num as ::core::ffi::c_int {
            if *track_glyph.y.offset(glyph_check as isize) == y
                && *track_glyph.x.offset(glyph_check as isize) == x
            {
                glyph = 1 as ::core::ffi::c_int;
            }
            glyph_check += 1;
        }
    }
    if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).ranged_attack == 0 {
        return 0 as ::core::ffi::c_int;
    }
    hp = (*kill).power as ::core::ffi::c_int;
    q = 0 as ::core::ffi::c_int;
    while q < (*kill).ranged_attack as ::core::ffi::c_int {
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        match (*kill).spell[q as usize] as ::core::ffi::c_int {
            1 => {
                p += 5 as ::core::ffi::c_int;
            }
            2 => {
                if d < 3 as ::core::ffi::c_int {
                    z = 100 as ::core::ffi::c_int;
                }
            }
            3 => {
                if d < 4 as ::core::ffi::c_int {
                    z = 100 as ::core::ffi::c_int;
                }
            }
            4 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 5 as ::core::ffi::c_int;
            }
            5 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 6 as ::core::ffi::c_int;
            }
            6 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 7 as ::core::ffi::c_int;
            }
            7 => {
                if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            8 => {
                if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            9 => {
                if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            10 => {
                if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            11 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 800 as ::core::ffi::c_int {
                    z = 800 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0 {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.temp.res_pois {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if !crate::bridge::st().borg.temp.res_pois {
                    if !(crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0) {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            12 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 600 as ::core::ffi::c_int {
                    z = 600 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                } else {
                    p += 125 as ::core::ffi::c_int;
                }
            }
            13 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
                } else if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            14 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            15 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 5 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                    p += 50 as ::core::ffi::c_int;
                }
            }
            16 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 600 as ::core::ffi::c_int {
                    z = 600 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0) {
                    p += 200 as ::core::ffi::c_int;
                }
            }
            17 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0) {
                    p += 500 as ::core::ffi::c_int;
                }
            }
            18 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 400 as ::core::ffi::c_int {
                    z = 400 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0) {
                    p += 100 as ::core::ffi::c_int;
                }
            }
            19 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 150 as ::core::ffi::c_int {
                    z = 150 as ::core::ffi::c_int;
                }
                p += 250 as ::core::ffi::c_int;
            }
            20 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
            }
            21 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            22 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0) {
                    p += 50 as ::core::ffi::c_int;
                }
            }
            23 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 150 as ::core::ffi::c_int {
                    z = 150 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    p += 100 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            24 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 100 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    p += 50 as ::core::ffi::c_int;
                }
            }
            25 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 1600 as ::core::ffi::c_int {
                    z = 1600 as ::core::ffi::c_int;
                }
            }
            26 => {
                z = (1 as ::core::ffi::c_int + (*r_ptr).spell_power / 7 as ::core::ffi::c_int)
                    * 12 as ::core::ffi::c_int;
                bolt = true_0 != 0;
            }
            28 => {
                if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int + 15 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            29 => {
                if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            30 => {
                if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                    z = (*r_ptr).spell_power * 7 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            31 => {
                if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            32 => {
                z = ((*r_ptr).spell_power / 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    * 4 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0 {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if crate::bridge::st().borg.temp.res_pois {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if !crate::bridge::st().borg.temp.res_pois {
                    if !(crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0) {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            33 => {
                z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0) {
                    p += 20 as ::core::ffi::c_int;
                }
            }
            34 => {
                z = (*r_ptr).spell_power * 4 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                    p += 250 as ::core::ffi::c_int;
                }
            }
            35 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    if !(crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0) {
                        p += 50 as ::core::ffi::c_int;
                        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            36 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                p += 50 as ::core::ffi::c_int;
            }
            37 => {
                z = 10 as ::core::ffi::c_int
                    + ((*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                p += 50 as ::core::ffi::c_int;
            }
            38 => {
                z = (*r_ptr).spell_power * 4 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            39 => {
                z = 10 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            40 => {
                z = 70 as ::core::ffi::c_int + (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0;
                }
            }
            41 => {
                if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] != 0 {
                    p += 100 as ::core::ffi::c_int;
                }
            }
            42 => {
                if crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    < 100 as ::core::ffi::c_int
                {
                    z = (*r_ptr).spell_power / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                }
            }
            43 => {
                z = 12 as ::core::ffi::c_int * (15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
                p += 200 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int
                        * crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize];
                if p < 0 as ::core::ffi::c_int {
                    p = 0 as ::core::ffi::c_int;
                }
            }
            44 => {
                if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    >= 100 as ::core::ffi::c_int)
                {
                    z = (*r_ptr).spell_power / 3 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * 5 as ::core::ffi::c_int;
                    z = z
                        * (120 as ::core::ffi::c_int
                            - crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize])
                        / 100 as ::core::ffi::c_int;
                }
            }
            45 => {
                bolt = true_0 != 0;
                if !(crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0) {
                    z = 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            46 => {
                if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                    bolt = true_0 != 0;
                    z = 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            47 => {
                if !(crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0) {
                    bolt = true_0 != 0;
                    z = 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            48 => {
                if !(crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0) {
                    bolt = true_0 != 0;
                    z = 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            49 => {
                if !(crate::bridge::st().borg.trait_[(BI_IPOIS as ::core::ffi::c_int) as usize] != 0) {
                    z = 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_pois {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    bolt = true_0 != 0;
                }
            }
            50 => {
                bolt = true_0 != 0;
                z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                    p += 200 as ::core::ffi::c_int;
                }
            }
            51 => {
                z = 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    if !(crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0) {
                        p += 20 as ::core::ffi::c_int;
                        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            52 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                bolt = true_0 != 0;
                p += 50 as ::core::ffi::c_int;
            }
            53 => {
                z = 10 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                    + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            54 => {
                z = 6 as ::core::ffi::c_int * 6 as ::core::ffi::c_int + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                p += 20 as ::core::ffi::c_int;
                if !(crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0) {
                    if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 50 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            55 => {
                z = 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                    + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                bolt = true_0 != 0;
            }
            56 => {
                if !(crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0) {
                    z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                        + (*r_ptr).spell_power * 2 as ::core::ffi::c_int
                        + 30 as ::core::ffi::c_int;
                    if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if crate::bridge::st().borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    bolt = true_0 != 0;
                }
            }
            57 => {
                bolt = true_0 != 0;
                z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 2 as ::core::ffi::c_int
                    + 30 as ::core::ffi::c_int;
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0) {
                    bolt = true_0 != 0;
                }
            }
            58 => {
                if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    >= 100 as ::core::ffi::c_int)
                {
                    p += 10 as ::core::ffi::c_int;
                }
            }
            59 => {
                if !(crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0)
                {
                    if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 10 as ::core::ffi::c_int;
                    }
                }
            }
            60 => {
                if !(crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 10 as ::core::ffi::c_int;
                    }
                }
            }
            61 => {
                if !(crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 5 as ::core::ffi::c_int;
                    }
                }
            }
            62 => {
                if !(crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0) {
                    if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 150 as ::core::ffi::c_int;
                    }
                }
            }
            63 => {
                p += 10 as ::core::ffi::c_int;
            }
            64 => {
                p += 10 as ::core::ffi::c_int;
            }
            67 => {
                p += 10 as ::core::ffi::c_int;
            }
            68 => {
                p += 20 as ::core::ffi::c_int;
            }
            69 => {
                p += 20 as ::core::ffi::c_int;
            }
            70 => {
                p += 10 as ::core::ffi::c_int;
            }
            71 => {
                if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    >= 100 as ::core::ffi::c_int)
                {
                    p += 50 as ::core::ffi::c_int;
                }
            }
            72 => {
                p += 5 as ::core::ffi::c_int;
            }
            73 => {
                p += 50 as ::core::ffi::c_int;
            }
            74 => {
                if !(crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    >= 100 as ::core::ffi::c_int)
                {
                    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                        < 15 as ::core::ffi::c_int
                    {
                        p += 500 as ::core::ffi::c_int;
                    } else {
                        p += 30 as ::core::ffi::c_int;
                    }
                }
            }
            75 => {
                p += 200 as ::core::ffi::c_int;
            }
            76 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            77 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            78 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
            }
            79 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            80 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            81 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            82 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            83 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 2 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            84 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            85 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            86 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            87 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            88 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            89 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            90 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            91 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = crate::bridge::grid(x, y) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if crate::borg_cave_util::borg_feature_protected(&*(ag)) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
                                    || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0 || borg_create_door as ::core::ffi::c_int != 0 {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
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
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            27 | 65 | 66 | _ => {}
        }
        if bolt as ::core::ffi::c_int == true_0
            && !borg_projectable_pure((*kill).pos.y, (*kill).pos.x, crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x)
        {
            z = 0 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 75 as ::core::ffi::c_int
        {
            p = 0 as ::core::ffi::c_int;
        }
        p += z;
        if p > n {
            n = p;
        }
        total_dam += p;
        q += 1;
    }
    total_dam -= crate::bridge::st().borg.trait_[(BI_DAM_RED as ::core::ffi::c_int) as usize];
    if total_dam < 0 as ::core::ffi::c_int {
        total_dam = 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        total_dam = total_dam * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    av = total_dam / (*kill).ranged_attack as ::core::ffi::c_int;
    if !average {
        return av;
    }
    if n >= av * 15 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || n > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            * 8 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
    {
        return n;
    } else {
        return av;
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_danger_one_kill(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut average: bool,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    let mut x9: ::core::ffi::c_int = (*kill).pos.x;
    let mut y9: ::core::ffi::c_int = (*kill).pos.y;
    let mut y_temp: ::core::ffi::c_int = 0;
    let mut x_temp: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut v1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_v2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_v1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fake_speed: ::core::ffi::c_int =
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize];
    let mut monster_speed: ::core::ffi::c_int = (*kill).speed as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut chance: ::core::ffi::c_int = 0;
    if (*kill).r_idx == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 100 as ::core::ffi::c_int;
    }
    if borg_tp_other_n != 0 {
        ii = 0 as ::core::ffi::c_int;
        while ii <= borg_tp_other_n as ::core::ffi::c_int {
            if i == borg_tp_other_index[ii as usize] {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
    }
    ax = if x9 > x { x9 - x } else { x - x9 };
    ay = if y9 > y { y9 - y } else { y - y9 };
    d = if ax < ay { ay } else { ax };
    if d < 1 as ::core::ffi::c_int {
        d = 1 as ::core::ffi::c_int;
    }
    if d > 20 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] >= 135 as ::core::ffi::c_int {
        fake_speed = if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
            120 as ::core::ffi::c_int
        } else {
            125 as ::core::ffi::c_int
        };
    }
    if crate::bridge::st().borg.temp.fast {
        fake_speed += 10 as ::core::ffi::c_int;
    }
    if borg_slow_spell {
        monster_speed -= 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] < 20 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        monster_speed += 3 as ::core::ffi::c_int;
    }
    e = extract_energy[fake_speed as usize] as ::core::ffi::c_int;
    t = (100 as ::core::ffi::c_int + (e - 1 as ::core::ffi::c_int)) / e;
    e = extract_energy[monster_speed as usize] as ::core::ffi::c_int;
    q = c * (t * e / 10 as ::core::ffi::c_int);
    if full_damage {
        q = (q + 9 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
    }
    if q <= 10 as ::core::ffi::c_int {
        q = 10 as ::core::ffi::c_int;
    }
    v1 = borg_danger_physical(i, full_damage);
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 1200 as ::core::ffi::c_int
        || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 25000 as borg_time
    {
        v1 = v1 / 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_BLOW as ::core::ffi::c_int,
    ) {
        v1 = 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && d > 1 as ::core::ffi::c_int
    {
        v1 = 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
    {
        v1 = v1 + v1 * 15 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
        {
            v1 = v1 + v1 * 18 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else {
            v1 = v1 + v1 * 13 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        }
    }
    if !(*kill).awake {
        let mut inc: ::core::ffi::c_int = (*r_ptr).sleep + 5 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= 25 as ::core::ffi::c_int
        {
            v1 = v1 / 2 as ::core::ffi::c_int;
        }
        v1 = v1 + v1 * inc / 100 as ::core::ffi::c_int;
    }
    if borg_sleep_spell_ii {
        if d == 1 as ::core::ffi::c_int
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
                RF_NO_SLEEP as ::core::ffi::c_int,
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
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    - 15 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 20 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / 3 as ::core::ffi::c_int;
            }
        }
    }
    if borg_sleep_spell {
        if (*kill).awake as ::core::ffi::c_int != 0
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
                RF_NO_SLEEP as ::core::ffi::c_int,
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
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    - 15 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 20 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / (d + 2 as ::core::ffi::c_int);
            }
        }
    }
    if borg_crush_spell {
        if ((*kill).power as ::core::ffi::c_int
            * (100 as ::core::ffi::c_int - (*kill).injury as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int)
            < crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 4 as ::core::ffi::c_int
        {
            let mut ag: *mut borg_grid = crate::bridge::grid(x9, y9) as *mut borg_grid;
            if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0
                && crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) as ::core::ffi::c_int != 0
            {
                v1 = 0 as ::core::ffi::c_int;
            }
        }
    }
    if (*kill).confused {
        v1 = v1 / 2 as ::core::ffi::c_int;
    }
    if (*kill).stunned {
        v1 = v1 * 10 as ::core::ffi::c_int / 13 as ::core::ffi::c_int;
    }
    if borg_confuse_spell {
        if (*kill).awake as ::core::ffi::c_int != 0
            && !(*kill).confused
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
                RF_NO_SLEEP as ::core::ffi::c_int,
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
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    - 15 as ::core::ffi::c_int
        {
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 20 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / (d + 2 as ::core::ffi::c_int);
            }
        }
    }
    if borg_fear_mon_spell {
        v1 = 0 as ::core::ffi::c_int;
    }
    if q > 10 as ::core::ffi::c_int
        && d != 1 as ::core::ffi::c_int
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
    {
        b_v1 = 0 as ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while ii < 8 as ::core::ffi::c_int {
            y_temp = y9 + ddy_ddd[ii as usize] as ::core::ffi::c_int;
            x_temp = x9 + ddx_ddd[ii as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x_temp, y_temp)) {
                if !((*crate::bridge::grid(x_temp, y_temp)).kill != 0) {
                    if !((*crate::bridge::grid(x_temp, y_temp)).feat
                        as ::core::ffi::c_int
                        == FEAT_CLOSED as ::core::ffi::c_int)
                    {
                        if !((*crate::bridge::grid(x_temp, y_temp)).feat
                            as ::core::ffi::c_int
                            == FEAT_PERM as ::core::ffi::c_int)
                        {
                            if (*crate::bridge::grid(x_temp, y_temp)).feat
                                as ::core::ffi::c_int
                                == FEAT_GRANITE as ::core::ffi::c_int
                                || ((*crate::bridge::grid(x_temp, y_temp)).feat
                                    as ::core::ffi::c_int
                                    == FEAT_MAGMA as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_MAGMA_K as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ_K as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_RUBBLE as ::core::ffi::c_int)
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
                                    RF_PASS_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_distance(y_temp, x_temp, y, x)
                                        == 1 as ::core::ffi::c_int
                                    {
                                        b_v1 = v1;
                                    }
                                }
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
                                    RF_KILL_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_distance(y_temp, x_temp, y, x)
                                        == 1 as ::core::ffi::c_int
                                    {
                                        b_v1 = v1;
                                    }
                                }
                            }
                            if !(borg_distance(y_temp, x_temp, y, x) > 1 as ::core::ffi::c_int) {
                                if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y_temp, x_temp) {
                                    b_v1 = v1 * (q / (d * 10 as ::core::ffi::c_int));
                                }
                            }
                        }
                    }
                }
            }
            ii += 1;
        }
        v1 = b_v1;
    }
    if q > 10 as ::core::ffi::c_int && d == 1 as ::core::ffi::c_int {
        v1 = v1 * q / 10 as ::core::ffi::c_int;
    }
    if q == 10 as ::core::ffi::c_int && d > 1 as ::core::ffi::c_int {
        v1 = 0 as ::core::ffi::c_int;
    }
    v2 = borg_danger_spell(i, y, x, d, average);
    if (*r_ptr).freq_innate == 0 && (*r_ptr).freq_spell == 0 {
        v2 = 0 as ::core::ffi::c_int;
    }
    if borg_distance(y9, x9, y, x) > (*z_info).max_range as ::core::ffi::c_int {
        v2 = 0 as ::core::ffi::c_int;
    }
    if q <= 10 as ::core::ffi::c_int
        && !borg_projectable(y9, x9, y, x)
        && !borg_projectable(y, x, y9, x9)
    {
        v2 = 0 as ::core::ffi::c_int;
    }
    if q >= 20 as ::core::ffi::c_int {
        let mut b_q: ::core::ffi::c_int = q;
        b_v2 = 0 as ::core::ffi::c_int;
        if q > 20 as ::core::ffi::c_int {
            q = 20 as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < 8 as ::core::ffi::c_int {
            y_temp = y9 + ddy_ddd[ii as usize] as ::core::ffi::c_int;
            x_temp = x9 + ddx_ddd[ii as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x_temp, y_temp)) {
                if !((*crate::bridge::grid(x_temp, y_temp)).kill != 0) {
                    if !((*crate::bridge::grid(x_temp, y_temp)).feat
                        as ::core::ffi::c_int
                        == FEAT_CLOSED as ::core::ffi::c_int)
                    {
                        if !((*crate::bridge::grid(x_temp, y_temp)).feat
                            as ::core::ffi::c_int
                            == FEAT_PERM as ::core::ffi::c_int)
                        {
                            if (*crate::bridge::grid(x_temp, y_temp)).feat
                                as ::core::ffi::c_int
                                >= FEAT_GRANITE as ::core::ffi::c_int
                                || ((*crate::bridge::grid(x_temp, y_temp)).feat
                                    as ::core::ffi::c_int
                                    == FEAT_MAGMA as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_MAGMA_K as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ_K as ::core::ffi::c_int
                                    || (*crate::bridge::grid(x_temp, y_temp)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_RUBBLE as ::core::ffi::c_int)
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
                                    RF_PASS_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_projectable(y_temp, x_temp, y, x) {
                                        b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                                    }
                                }
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
                                    RF_KILL_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_projectable(y_temp, x_temp, y, x) {
                                        b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                                    }
                                }
                            } else if borg_projectable(y_temp, x_temp, y, x) {
                                b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            ii += 1;
        }
        v2 = b_v2;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 1200 as ::core::ffi::c_int
        || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 25000 as borg_time
    {
        v2 = v2 / 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
    {
        v2 = v2 + v2 * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
    {
        v2 = v2 + v2 * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if !(*kill).awake {
        let mut inc_0: ::core::ffi::c_int = (*r_ptr).sleep + 5 as ::core::ffi::c_int;
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= 25 as ::core::ffi::c_int
        {
            v2 = v2 / 2 as ::core::ffi::c_int;
        }
        v2 = v2 + v2 * inc_0 / 100 as ::core::ffi::c_int;
    }
    if borg_sleep_spell_ii {
        if d == 1 as ::core::ffi::c_int
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
                RF_NO_SLEEP as ::core::ffi::c_int,
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
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= (if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 15 as ::core::ffi::c_int
                {
                    crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                } else {
                    (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                        - 10 as ::core::ffi::c_int)
                        / 4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int
                })
        {
            v2 = v2 / 3 as ::core::ffi::c_int;
        }
    }
    if borg_crush_spell {
        if ((*kill).power as ::core::ffi::c_int
            * (100 as ::core::ffi::c_int - (*kill).injury as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int)
            < crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 4 as ::core::ffi::c_int
        {
            let mut ag_0: *mut borg_grid = crate::bridge::grid(x9, y9) as *mut borg_grid;
            if (*ag_0).info as ::core::ffi::c_int & BORG_VIEW != 0
                && crate::borg_cave_util::borg_cave_floor_grid(&*(ag_0)) as ::core::ffi::c_int != 0
            {
                v1 = 0 as ::core::ffi::c_int;
            }
        }
    }
    if borg_sleep_spell {
        v2 = v2 / (d + 2 as ::core::ffi::c_int);
    }
    if (*kill).confused {
        v2 = v2 / 2 as ::core::ffi::c_int;
    }
    if (*kill).stunned {
        v2 = v2 * 10 as ::core::ffi::c_int / 13 as ::core::ffi::c_int;
    }
    if borg_confuse_spell {
        v2 = v2 / 6 as ::core::ffi::c_int;
    }
    if !full_damage {
        chance = ((*r_ptr).freq_innate + (*r_ptr).freq_spell) / 2 as ::core::ffi::c_int;
        if chance < 11 as ::core::ffi::c_int {
            v2 = v2 * 4 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else if chance < 26 as ::core::ffi::c_int {
            v2 = v2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else if chance < 51 as ::core::ffi::c_int {
            v2 = v2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        }
    }
    if v2 != 0 {
        r = q;
        v2 = v2 * r / 10 as ::core::ffi::c_int;
    }
    p = if v1 < v2 { v2 } else { v1 };
    if p > 2000 as ::core::ffi::c_int {
        p = 2000 as ::core::ffi::c_int;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn borg_danger(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut average: bool,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: loc = loc(x, y);
    if !square_in_bounds(cave, l) {
        return 2000 as ::core::ffi::c_int;
    }
    if !square_isvault(cave, l)
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            <= 80 as ::core::ffi::c_int
    {
        p += borg_fear_region[(y / 11 as ::core::ffi::c_int) as usize]
            [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            * c;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 100 as ::core::ffi::c_int
        && p >= 300 as ::core::ffi::c_int
    {
        p = 300 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int <= 200 as ::core::ffi::c_int
        && !square_isvault(cave, loc(x, y))
    {
        p += borg_fear_monsters[y as usize][x as usize] as ::core::ffi::c_int * c;
    }
    full_damage = true_0 != 0;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            p += borg_danger_one_kill(y, x, c, i, average, full_damage);
        }
        i += 1;
    }
    return if p > 2000 as ::core::ffi::c_int {
        2000 as ::core::ffi::c_int
    } else {
        p
    };
}
