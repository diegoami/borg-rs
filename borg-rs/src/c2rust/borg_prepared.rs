pub const CLASS_WARRIOR: ::core::ffi::c_int = 0;
pub const CLASS_MAGE: ::core::ffi::c_int = 1;
pub const CLASS_DRUID: ::core::ffi::c_int = 2;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6;
pub const CLASS_RANGER: ::core::ffi::c_int = 7;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_formulas_h::{borg_prepared_dynamic, borg_restock_dynamic};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_home_notice_h::{
    borg_notice_home, num_ezheal, num_ezheal_true, num_life, num_life_true,
};
pub use crate::headers::borg_item_h::borg_item;

pub use crate::headers::borg_magic_h::{
    borg_spell_legal_fail, borg_spell_stat, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
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

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::strnfmt;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub const MAX_REASON: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut borg_prepared_buffer: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut borg_numb_live_unique: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut borg_first_living_unique: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut borg_depth_hunted_unique: ::core::ffi::c_int = 0;
unsafe extern "C" fn borg_prepared_aux(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if -(1 as ::core::ffi::c_int) == crate::bridge::st().borg.ready_morgoth {
        crate::bridge::st().borg.ready_morgoth = 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] != 0 {
        crate::bridge::st().borg.ready_morgoth = 1 as ::core::ffi::c_int;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if depth == 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int {
        return b"1 Lite\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] < 5 as ::core::ffi::c_int {
        return b"5 Food\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 1 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] < 5 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
    {
        return b"5 Fuel\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] < 30 as ::core::ffi::c_int
        {
            return b"30 hp\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if depth <= 2 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
            CLASS_WARRIOR | CLASS_BLACKGUARD => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 50 as ::core::ffi::c_int
                {
                    return b"50 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 4 as ::core::ffi::c_int
                {
                    return b"4 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_ROGUE => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 50 as ::core::ffi::c_int
                {
                    return b"50 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 8 as ::core::ffi::c_int
                {
                    return b"8 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PRIEST | CLASS_DRUID => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 40 as ::core::ffi::c_int
                {
                    return b"40 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 9 as ::core::ffi::c_int
                {
                    return b"9 level\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PALADIN => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 50 as ::core::ffi::c_int
                {
                    return b"50 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 4 as ::core::ffi::c_int
                {
                    return b"4 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_RANGER => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 50 as ::core::ffi::c_int
                {
                    return b"50 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 4 as ::core::ffi::c_int
                {
                    return b"4 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_MAGE | CLASS_NECROMANCER => {
                if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    < 60 as ::core::ffi::c_int
                {
                    return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < 11 as ::core::ffi::c_int
                {
                    return b"11 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            _ => {}
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            < 2 as ::core::ffi::c_int
    {
        return b"2 cure\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 4 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
        {
            match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
                CLASS_WARRIOR | CLASS_BLACKGUARD => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 60 as ::core::ffi::c_int
                    {
                        return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 6 as ::core::ffi::c_int
                    {
                        return b"6 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                CLASS_ROGUE => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 60 as ::core::ffi::c_int
                    {
                        return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 10 as ::core::ffi::c_int
                    {
                        return b"10 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                CLASS_PRIEST | CLASS_DRUID => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 60 as ::core::ffi::c_int
                    {
                        return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 15 as ::core::ffi::c_int
                    {
                        return b"15 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                CLASS_PALADIN => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 60 as ::core::ffi::c_int
                    {
                        return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 6 as ::core::ffi::c_int
                    {
                        return b"6 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                CLASS_RANGER => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 60 as ::core::ffi::c_int
                    {
                        return b"60 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 6 as ::core::ffi::c_int
                    {
                        return b"6 clevel\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                CLASS_MAGE | CLASS_NECROMANCER => {
                    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        < 80 as ::core::ffi::c_int
                    {
                        return b"80 hp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        < 15 as ::core::ffi::c_int
                    {
                        return b"15 level\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                _ => {}
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            < 2 as ::core::ffi::c_int
    {
        return b"2 cures (clw + csw + ccw)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
        < 1 as ::core::ffi::c_int
    {
        return b"1 recall\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 9 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_NECROMANCER
    {
        return b"2 light radius\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
        < 2 as ::core::ffi::c_int
    {
        return b"2 tele + teleport staffs\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
            CLASS_WARRIOR | CLASS_BLACKGUARD => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth - 4 as ::core::ffi::c_int
                    && depth <= 19 as ::core::ffi::c_int
                {
                    return b"dlevel - 4 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_ROGUE => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth
                    && depth <= 19 as ::core::ffi::c_int
                {
                    return b"dlevel >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PRIEST | CLASS_DRUID => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth
                    && depth <= 19 as ::core::ffi::c_int
                {
                    return b"dlevel >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PALADIN => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth
                    && depth <= 19 as ::core::ffi::c_int
                {
                    return b"dlevel >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_RANGER => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth
                    && depth <= 19 as ::core::ffi::c_int
                {
                    return b"dlevel >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_MAGE | CLASS_NECROMANCER => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 5 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 28 as ::core::ffi::c_int
                {
                    return b"dlevel + 5 > = clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            _ => {}
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] < 3 as ::core::ffi::c_int
    {
        return b"ccw < 3\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.trait_[(BI_DINV as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] == 0
    {
        return b"See Invis : ESP\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 19 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] == 0 {
        return b"free action\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 20 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRFIRE as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist fire\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let mut basics: ::core::ffi::c_int =
        crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize];
    if basics < 2 as ::core::ffi::c_int {
        return b"2 basic resists\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] < 7 as ::core::ffi::c_int {
        return b"low STR\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat != -(1 as ::core::ffi::c_int) {
        if crate::bridge::st().borg.trait_[((BI_STR as ::core::ffi::c_int + spell_stat)) as usize]
            < 7 as ::core::ffi::c_int
        {
            return b"low spell stat\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_DEX as ::core::ffi::c_int) as usize] < 7 as ::core::ffi::c_int {
        return b"low DEX\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_CON as ::core::ffi::c_int) as usize] < 7 as ::core::ffi::c_int {
        return b"low CON\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        match crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] {
            CLASS_WARRIOR | CLASS_BLACKGUARD => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 5 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 38 as ::core::ffi::c_int
                {
                    return b"dlevel + 5 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_ROGUE => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 10 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 43 as ::core::ffi::c_int
                {
                    return b"dlevel + 10 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PRIEST | CLASS_DRUID => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 13 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 46 as ::core::ffi::c_int
                {
                    return b"dlevel + 13 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_PALADIN => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 7 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 40 as ::core::ffi::c_int
                {
                    return b"dlevel + 7 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_RANGER => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 8 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 41 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        > 28 as ::core::ffi::c_int
                {
                    return b"dlevel + 8 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
            CLASS_MAGE | CLASS_NECROMANCER => {
                if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    < depth + 8 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 38 as ::core::ffi::c_int
                {
                    return b"dlevel + 8 >= clevel\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if ((crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                    - 38 as ::core::ffi::c_int)
                    * 2 as ::core::ffi::c_int
                    + 30 as ::core::ffi::c_int)
                    < depth
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        <= 44 as ::core::ffi::c_int
                    && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                        > 38 as ::core::ffi::c_int
                {
                    return b"(clevel-38)*2+30 < dlevel\0" as *const u8
                        as *const ::core::ffi::c_char;
                }
            }
            _ => {}
        }
    }
    if depth <= 25 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRCOLD as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist cold\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRELEC as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist elec\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRACID as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist acid\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
        < 6 as ::core::ffi::c_int
    {
        return b"6 tell + telep staffs\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            < 10 as ::core::ffi::c_int
    {
        return b"10 ccw + csw\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 33 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 40 as ::core::ffi::c_int
        && crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0
    {
        return b"level 40\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 39 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRPOIS as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist pois\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRCONF as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist conf\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize] < 16 as ::core::ffi::c_int {
        return b"STR < 16\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if spell_stat != -(1 as ::core::ffi::c_int) {
        if crate::bridge::st().borg.trait_[((BI_STR as ::core::ffi::c_int + spell_stat)) as usize]
            < 16 as ::core::ffi::c_int
        {
            return b"spell stat < 16\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_DEX as ::core::ffi::c_int) as usize] < 16 as ::core::ffi::c_int {
        return b"dex < 16\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_CON as ::core::ffi::c_int) as usize] < 16 as ::core::ffi::c_int {
        return b"con < 16\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 45 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 115 as ::core::ffi::c_int {
        return b"+5 speed\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            < 1 as ::core::ffi::c_int
    {
        return b"1 heal\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0 {
        if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] < 500 as ::core::ffi::c_int
        {
            return b"HP 500\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_STR as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 40 as ::core::ffi::c_int
    {
        return b"str < 18(40)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if spell_stat != -(1 as ::core::ffi::c_int) {
        if crate::bridge::st().borg.trait_[((BI_STR as ::core::ffi::c_int + spell_stat)) as usize]
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
        {
            return b"spell stat needs to be max\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_DEX as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 60 as ::core::ffi::c_int
    {
        return b"dex < 18 (60)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_CON as ::core::ffi::c_int) as usize]
        < 18 as ::core::ffi::c_int + 60 as ::core::ffi::c_int
    {
        return b"con < 18 (60)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SHLIFE as ::core::ffi::c_int) as usize]
        == 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 50 as ::core::ffi::c_int
    {
        return b"hold life\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 55 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            < 1 as ::core::ffi::c_int
    {
        return b"2 heal + *heal*\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRBLIND as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist blind\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] == 0 {
        return b"ESP\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 59 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 120 as ::core::ffi::c_int {
        return b"+10 speed\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRKAOS as ::core::ffi::c_int) as usize]
        == 0
    {
        return b"resist chaos\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SRDIS as ::core::ffi::c_int) as usize] == 0 {
        return b"resist disenchant\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 80 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize] < 130 as ::core::ffi::c_int {
        return b"+20 Speed\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 85 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if depth <= 99 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0 {
        if crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] > 100 as ::core::ffi::c_int
            && crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_restore_mana) as usize] < 15 as ::core::ffi::c_int
        {
            return b"10 restore mana\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_potion_healing) as usize] < 5 as ::core::ffi::c_int {
            return b"5 Heal\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
        {
            return b"15 *heal* or life\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if crate::bridge::st().borg.trait_[(BI_ASPEED as ::core::ffi::c_int) as usize]
            < 10 as ::core::ffi::c_int
        {
            return b"10 speed potions\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if depth <= 127 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    return NULL as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepared(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut reason: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if depth == 1 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().core.cfg[(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int) as usize] != 0 {
        reason = borg_restock(depth);
        if !reason.is_null() {
            return reason;
        }
        reason = borg_prepared_dynamic(depth);
        if !reason.is_null() {
            return reason;
        }
    } else {
        reason = borg_restock(depth);
        if !reason.is_null() {
            return reason;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < depth
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                < 50 as ::core::ffi::c_int
        {
            return b"Clevel < depth\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if depth <= 99 as ::core::ffi::c_int {
            reason = borg_prepared_aux(depth);
            if !reason.is_null() {
                return reason;
            }
        }
        if depth >= crate::bridge::st().core.cfg[(BORG_NO_DEEPER as ::core::ffi::c_int) as usize] {
            strnfmt(
                &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
                MAX_REASON as size_t,
                b"No deeper %d.\0" as *const u8 as *const ::core::ffi::c_char,
                crate::bridge::st().core.cfg[(BORG_NO_DEEPER as ::core::ffi::c_int) as usize],
            );
            return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] != 0 {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        return reason;
    }
    if depth >= 82 as ::core::ffi::c_int
        && ((num_ezheal as ::core::ffi::c_int + num_life as ::core::ffi::c_int)
            < 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
                + crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int)
    {
        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
        strnfmt(
            &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
            MAX_REASON as size_t,
            b"Scumming *Heal* potions (%d to go).\0" as *const u8 as *const ::core::ffi::c_char,
            10 as ::core::ffi::c_int - num_ezheal as ::core::ffi::c_int,
        );
        return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
    }
    if depth >= 82 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= 97 as ::core::ffi::c_int
    {
        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
        let mut heals: ::core::ffi::c_int = num_ezheal_true as ::core::ffi::c_int
            + crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            + num_life_true as ::core::ffi::c_int
            + crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize];
        if heals < 30 as ::core::ffi::c_int {
            strnfmt(
                &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
                MAX_REASON as size_t,
                b"Scumming *Heal* potions (%d to go).\0" as *const u8 as *const ::core::ffi::c_char,
                30 as ::core::ffi::c_int - heals,
            );
            return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
        }
        if crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize]
            < 30 as ::core::ffi::c_int
            && heals >= 30 as ::core::ffi::c_int
            && num_ezheal_true as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                >= 99 as ::core::ffi::c_int
        {
            strnfmt(
                &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
                MAX_REASON as size_t,
                b"Collect from house (%d potions).\0" as *const u8 as *const ::core::ffi::c_char,
                num_ezheal_true as ::core::ffi::c_int + num_life_true as ::core::ffi::c_int,
            );
            return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        <= 98 as ::core::ffi::c_int
    {
        let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        if crate::bridge::st().core.cfg[(BORG_KILLS_UNIQUES as ::core::ffi::c_int) as usize] == false_0 {
            return NULL as *mut ::core::ffi::c_char;
        }
        if borg_numb_live_unique < 3 as ::core::ffi::c_int {
            return NULL as *mut ::core::ffi::c_char;
        }
        if depth <= borg_depth_hunted_unique {
            return NULL as *mut ::core::ffi::c_char;
        }
        r_ptr = r_info.offset(borg_first_living_unique as isize) as *mut monster_race;
        strnfmt(
            &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
            MAX_REASON as size_t,
            b"Must kill %s.\0" as *const u8 as *const ::core::ffi::c_char,
            (*r_ptr).name,
        );
        return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
    } else if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 98 as ::core::ffi::c_int
        && depth >= 98 as ::core::ffi::c_int
    {
        let mut r_ptr_0: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        r_ptr_0 = r_info.offset(borg_first_living_unique as isize) as *mut monster_race;
        crate::bridge::st().borg.ready_morgoth = -(1 as ::core::ffi::c_int);
        if borg_numb_live_unique < 1 as ::core::ffi::c_int
            || borg_first_living_unique == crate::bridge::st().borg.id.morgoth
        {
            if depth >= 99 as ::core::ffi::c_int {
                crate::bridge::st().borg.ready_morgoth = 1 as ::core::ffi::c_int;
            }
            return NULL as *mut ::core::ffi::c_char;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            == 99 as ::core::ffi::c_int
            && depth <= 98 as ::core::ffi::c_int
            && (borg_spell_legal_fail(TELEPORT_LEVEL, 20 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
                || crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize]
                    >= 1 as ::core::ffi::c_int)
        {
            return NULL as *mut ::core::ffi::c_char;
        }
        strnfmt(
            &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char,
            MAX_REASON as size_t,
            b"%s still alive!\0" as *const u8 as *const ::core::ffi::c_char,
            (*r_ptr_0).name,
        );
        return &raw mut borg_prepared_buffer as *mut ::core::ffi::c_char;
    }
    return NULL as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn borg_restock(mut depth: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    if -(1 as ::core::ffi::c_int) == crate::bridge::st().borg.ready_morgoth {
        crate::bridge::st().borg.ready_morgoth = 0 as ::core::ffi::c_int;
    }
    if crate::bridge::st().core.cfg[(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int) as usize] != 0 {
        return borg_restock_dynamic(depth);
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int {
        return b"restock light radius < 1\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
    {
        return b"restock fuel\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 1 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
    {
        return b"restock fuel < 2\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] < 3 as ::core::ffi::c_int {
        return b"restock food < 3\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 3 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if depth <= 5 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
        < 1 as ::core::ffi::c_int
    {
        return b"restock phase door\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            < 1 as ::core::ffi::c_int
    {
        return b"restock clw+csw+ccw\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 9 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_NECROMANCER
    {
        return b"2 light radius\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACLW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            < 2 as ::core::ffi::c_int
    {
        return b"restock clw + csw + ccw \0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
        < 2 as ::core::ffi::c_int
    {
        return b"restock tele + tele staff < 2\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 19 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 30 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ACSW as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
            < 4 as ::core::ffi::c_int
    {
        return b"restock csw + ccw < 4\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
        < 4 as ::core::ffi::c_int
    {
        return b"restock 4 > teleport + teleport staff \0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if depth <= 35 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.trait_[(BI_ATELEPORTLVL as ::core::ffi::c_int) as usize]
        < 2 as ::core::ffi::c_int
    {
        return b"restock teleport + teleport level scrolls\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if depth <= 45 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if depth <= 64 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize]
        + crate::bridge::st().borg.has[(crate::bridge::st().item_val.kv_rod_healing) as usize]
        + crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
        < 1 as ::core::ffi::c_int
    {
        return b"restock heal\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if depth <= 99 as ::core::ffi::c_int {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level < 10 as borg_time
        && crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
    {
        return b"restock *heal*\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return NULL as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn borg_must_return_to_town() -> *const ::core::ffi::c_char {
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
    {
        return NULL as *mut ::core::ffi::c_char;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level < 100 as borg_time
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 100 as ::core::ffi::c_int
    {
        return NULL as *mut ::core::ffi::c_char;
    }
    return borg_restock(
        crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
    );
}
