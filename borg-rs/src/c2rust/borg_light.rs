pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};


pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::{borg_keypress, borg_note};

pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_aim_wand, borg_equips_rod, borg_need, borg_read_scroll,
    borg_use_staff, borg_zap_rod, BORG_MET_NEED, BORG_NO_NEED, BORG_UNMET_NEED,
};

pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_fail, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE,
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
use crate::headers::borg_messages_react_h::borg_react;
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
use crate::headers::borg_update_h::{
    borg_detect_door, borg_detect_evil, borg_detect_obj, borg_detect_trap, borg_detect_wall,
};
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, heatmap, monster_group, square,
    square_in_bounds_fully, square_isglow, trap, C2RustUnnamedH_18, FEAT_MAX,
};
use crate::headers::game_input_h::panel_contains;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_object_flags_h::{
    OF_AFRAID, OF_AGGRAVATE, OF_BLESSED, OF_BURNS_OUT, OF_DIG_1, OF_DIG_2, OF_DIG_3, OF_DRAIN_EXP,
    OF_EXPLODE, OF_FEATHER, OF_FRAGILE, OF_FREE_ACT, OF_HOLD_LIFE, OF_IMPACT, OF_IMPAIR_HP,
    OF_IMPAIR_MANA, OF_LIGHT_2, OF_LIGHT_3, OF_MAX, OF_MULTIPLY_WEIGHT, OF_NO_FUEL, OF_NO_TELEPORT,
    OF_PROT_BLIND, OF_PROT_CONF, OF_PROT_FEAR, OF_PROT_STUN, OF_REGEN, OF_SEE_INVIS,
    OF_SLOW_DIGEST, OF_STICKY, OF_SUST_CON, OF_SUST_DEX, OF_SUST_INT, OF_SUST_STR, OF_SUST_WIS,
    OF_TAKES_FUEL, OF_TELEPATHY, OF_THROWING, OF_TRAP_IMMUNE,
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
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, C2RustUnnamedH_31, OF_NONE, TV_MAX};
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
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
unsafe extern "C" fn borg_check_dark_only() -> bool {
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_NECROMANCER {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.call_light != 0 as borg_time
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.call_light < 7 as borg_time
    {
        return false_0 != 0;
    }
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut floors: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int;
    while y <= crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int {
        x = crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int;
        while x <= crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int {
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            if square_in_bounds_fully(cave, loc(x, y)) {
                ag = crate::bridge::grid(x, y) as *mut borg_grid;
                if crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) as ::core::ffi::c_int != 0
                    && square_isglow(cave, loc(x, y)) as ::core::ffi::c_int != 0
                {
                    floors += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }
    if floors < 11 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_spell_fail(CREATE_DARKNESS, 40 as ::core::ffi::c_int) {
        borg_note(
            b"# Calling Darkness in the dungeon\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.time.call_light = crate::bridge::st().borg.time.now;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_check_light_only() -> bool {
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.wizard_light == 0 || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.wizard_light >= 1000 as borg_time {
        if borg_activate_item(crate::bridge::st().act.clairvoyance) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.enlightenment) as ::core::ffi::c_int != 0
            || borg_spell_fail(FUME_OF_MORDOR, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(CLAIRVOYANCE, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            borg_note(
                b"# Wizard lighting the dungeon\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.wizard_light = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER {
        return borg_check_dark_only();
    }
    if crate::bridge::st().borg.time.call_light != 0 as borg_time
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.call_light < 7 as borg_time
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        let mut corners: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 4 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            let mut x: ::core::ffi::c_int = crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            let mut y: ::core::ffi::c_int = crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x, y)) {
                ag = crate::bridge::grid(x, y) as *mut borg_grid;
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    corners += 1;
                }
                if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
                    corners += 1;
                }
            }
            i += 1;
        }
        if corners > 2 as ::core::ffi::c_int {
            return false_0 != 0;
        }
    } else if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        > 1 as ::core::ffi::c_int
    {
        let mut x_0: ::core::ffi::c_int = 0;
        let mut y_0: ::core::ffi::c_int = 0;
        let mut floors: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        y_0 = crate::bridge::st().borg.c.y - 2 as ::core::ffi::c_int;
        while y_0 <= crate::bridge::st().borg.c.y + 2 as ::core::ffi::c_int {
            x_0 = crate::bridge::st().borg.c.x - 2 as ::core::ffi::c_int;
            while x_0 <= crate::bridge::st().borg.c.x + 2 as ::core::ffi::c_int {
                let mut ag_0: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
                if square_in_bounds_fully(cave, loc(x_0, y_0)) {
                    ag_0 = crate::bridge::grid(x_0, y_0) as *mut borg_grid;
                    if crate::borg_cave_util::borg_cave_floor_grid(&*(ag_0)) as ::core::ffi::c_int != 0
                        && (*ag_0).info as ::core::ffi::c_int & BORG_LIGHT != 0
                        && !square_isglow(cave, loc(x_0, y_0))
                    {
                        floors += 1;
                    }
                }
                x_0 += 1;
            }
            y_0 += 1;
        }
        if floors < 11 as ::core::ffi::c_int {
            return false_0 != 0;
        }
    }
    if borg_activate_item(crate::bridge::st().act.illumination) as ::core::ffi::c_int != 0
        || borg_activate_item(crate::bridge::st().act.light) as ::core::ffi::c_int != 0
        || borg_zap_rod(crate::bridge::st().item_val.sv_rod_illumination) as ::core::ffi::c_int != 0
        || borg_use_staff(crate::bridge::st().item_val.sv_staff_light) as ::core::ffi::c_int != 0
        || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_light) as ::core::ffi::c_int != 0
        || borg_spell_fail(LIGHT_ROOM, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_fail(CALL_LIGHT, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(b"# Illuminating the dungeon\0" as *const u8 as *const ::core::ffi::c_char);
        borg_react(
            b"SELF:lite\0" as *const u8 as *const ::core::ffi::c_char,
            b"SELF:lite\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.time.call_light = crate::bridge::st().borg.time.now;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_check_light() -> bool {
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut do_trap: bool = false;
    let mut do_door: bool = false;
    let mut do_wall: bool = false;
    let mut do_evil: bool = false;
    let mut do_obj: bool = false;
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        > 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    q_x = crate::bridge::st().core.w_x / crate::borg_cave_util::borg_panel_wid();
    q_y = crate::bridge::st().core.w_y / crate::borg_cave_util::borg_panel_hgt();
    do_trap = false_0 != 0;
    if !*(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_trap = true_0 != 0;
    }
    if !*(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_trap = true_0 != 0;
    }
    if !*(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_trap = true_0 != 0;
    }
    if !*(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_trap = true_0 != 0;
    }
    do_door = false_0 != 0;
    if !*(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_door = true_0 != 0;
    }
    if !*(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_door = true_0 != 0;
    }
    if !*(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_door = true_0 != 0;
    }
    if !*(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_door = true_0 != 0;
    }
    do_wall = false_0 != 0;
    if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_wall = true_0 != 0;
    }
    if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_wall = true_0 != 0;
    }
    if !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_wall = true_0 != 0;
    }
    if !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_wall = true_0 != 0;
    }
    do_evil = false_0 != 0;
    if !*(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_evil = true_0 != 0;
    }
    if !*(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_evil = true_0 != 0;
    }
    if !*(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_evil = true_0 != 0;
    }
    if !*(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_evil = true_0 != 0;
    }
    do_obj = false_0 != 0;
    if !*(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_obj = true_0 != 0;
    }
    if !*(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_obj = true_0 != 0;
    }
    if !*(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 0 as ::core::ffi::c_int) as isize)
    {
        do_obj = true_0 != 0;
    }
    if !*(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
        .offset((q_x + 1 as ::core::ffi::c_int) as isize)
    {
        do_obj = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
        do_evil = false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        do_trap = false_0 != 0;
        do_door = false_0 != 0;
        do_wall = false_0 != 0;
    }
    if (do_trap as ::core::ffi::c_int != 0
        || do_door as ::core::ffi::c_int != 0
        || do_evil as ::core::ffi::c_int != 0)
        && (crate::bridge::st().borg.time.detect_traps == 0
            || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_traps >= 5 as borg_time
            || (crate::bridge::st().borg.time.detect_evil == 0
                || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_evil >= 5 as borg_time)
            || (crate::bridge::st().borg.time.detect_doors == 0
                || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_doors >= 5 as borg_time))
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        if borg_activate_item(crate::bridge::st().act.detect_all) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.mapping) as ::core::ffi::c_int != 0
            || borg_zap_rod(crate::bridge::st().item_val.sv_rod_detection) as ::core::ffi::c_int != 0
            || borg_spell_fail(SENSE_SURROUNDINGS, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
        {
            borg_note(
                b"# Checking for traps, doors, and evil.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_react(
                b"SELF:TDE\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:TDE\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_traps = crate::bridge::st().borg.time.now;
            crate::bridge::st().borg.time.detect_doors = crate::bridge::st().borg.time.now;
            crate::bridge::st().borg.time.detect_evil = crate::bridge::st().borg.time.now;
            crate::bridge::st().borg.time.detect_obj = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if do_evil as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.time.detect_evil == 0 || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_evil >= 20 as borg_time)
    {
        if borg_spell_fail(DETECT_EVIL, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(DETECT_MONSTERS, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(READ_MINDS, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(SEEK_BATTLE, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Checking for monsters.\0" as *const u8 as *const ::core::ffi::c_char);
            borg_react(
                b"SELF:evil\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:evil\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_evil = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if (do_trap as ::core::ffi::c_int != 0 || do_door as ::core::ffi::c_int != 0)
        && (crate::bridge::st().borg.time.detect_traps == 0
            || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_traps >= 5 as borg_time
            || (crate::bridge::st().borg.time.detect_doors == 0
                || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_doors >= 5 as borg_time))
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        if borg_activate_item(crate::bridge::st().act.detect_all) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.mapping) as ::core::ffi::c_int != 0
            || borg_spell_fail(DETECTION, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(FIND_TRAPS_DOORS_STAIRS, 40 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
            || borg_spell_fail(DETECT_STAIRS, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            borg_note(
                b"# Checking for traps, doors & stairs.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_react(
                b"SELF:both\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:both\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_traps = crate::bridge::st().borg.time.now;
            crate::bridge::st().borg.time.detect_doors = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if do_trap as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.time.detect_traps == 0 || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_traps >= 7 as borg_time)
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        if borg_spell_fail(DETECTION, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(FIND_TRAPS_DOORS_STAIRS, 40 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
        {
            borg_note(b"# Checking for traps.\0" as *const u8 as *const ::core::ffi::c_char);
            borg_react(
                b"SELF:trap\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:trap\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_traps = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if do_door as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.time.detect_doors == 0 || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_doors >= 9 as borg_time)
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        if borg_activate_item(crate::bridge::st().act.detect_all) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.mapping) as ::core::ffi::c_int != 0
            || borg_spell_fail(DETECTION, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(FIND_TRAPS_DOORS_STAIRS, 40 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
        {
            borg_note(b"# Checking for doors.\0" as *const u8 as *const ::core::ffi::c_char);
            borg_react(
                b"SELF:door\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:door\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_doors = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    if do_wall as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.time.detect_walls == 0
            || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_walls >= 15 as borg_time)
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
    {
        if borg_activate_item(crate::bridge::st().act.mapping) as ::core::ffi::c_int != 0
            || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_mapping) as ::core::ffi::c_int != 0
            || borg_use_staff(crate::bridge::st().item_val.sv_staff_mapping) as ::core::ffi::c_int != 0
            || borg_zap_rod(crate::bridge::st().item_val.sv_rod_mapping) as ::core::ffi::c_int != 0
            || borg_spell(SENSE_SURROUNDINGS) as ::core::ffi::c_int != 0
        {
            let mut y: ::core::ffi::c_int = 0;
            let mut x: ::core::ffi::c_int = 0;
            borg_note(b"# Checking for walls.\0" as *const u8 as *const ::core::ffi::c_char);
            borg_react(
                b"SELF:wall\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:wall\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_walls = crate::bridge::st().borg.time.now;
            y = 0 as ::core::ffi::c_int;
            while y < AUTO_MAX_Y {
                x = 0 as ::core::ffi::c_int;
                while x < AUTO_MAX_X {
                    let ref mut fresh0 = (*crate::bridge::grid(x, y)).info;
                    *fresh0 = (*fresh0 as ::core::ffi::c_int & !BORG_IGNORE_MAP) as uint16_t;
                    x += 1;
                }
                y += 1;
            }
            return true_0 != 0;
        }
    }
    if do_obj as ::core::ffi::c_int != 0
        && (crate::bridge::st().borg.time.detect_obj == 0 || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.detect_obj >= 20 as borg_time)
    {
        if borg_activate_item(crate::bridge::st().act.detect_objects) as ::core::ffi::c_int != 0
            || borg_spell_fail(OBJECT_DETECTION, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
        {
            borg_note(b"# Checking for objects.\0" as *const u8 as *const ::core::ffi::c_char);
            borg_react(
                b"SELF:obj\0" as *const u8 as *const ::core::ffi::c_char,
                b"SELF:obj\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.time.detect_obj = crate::bridge::st().borg.time.now;
            return true_0 != 0;
        }
    }
    return borg_check_light_only();
}
unsafe extern "C" fn borg_refuel_lantern() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    i = borg_slot(TV_FLASK as ::core::ffi::c_int, crate::bridge::st().item_val.sv_flask_oil);
    if i < 0 as ::core::ffi::c_int {
        i = borg_slot(TV_LIGHT as ::core::ffi::c_int, crate::bridge::st().item_val.sv_light_lantern);
        if i >= 0 as ::core::ffi::c_int {
            while i < (*z_info).pack_size as ::core::ffi::c_int {
                if (*borg_items.offset(i as isize)).tval as ::core::ffi::c_int
                    != TV_LIGHT as ::core::ffi::c_int
                    || (*borg_items.offset(i as isize)).sval as ::core::ffi::c_int
                        != crate::bridge::st().item_val.sv_light_lantern
                {
                    i = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    if !flag_has(
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
                    ) {
                        if (*borg_items.offset(i as isize)).timeout as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    i += 1;
                }
            }
            if i >= (*z_info).pack_size as ::core::ffi::c_int {
                i = -(1 as ::core::ffi::c_int);
            }
        }
    }
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
    .sval as ::core::ffi::c_int
        != crate::bridge::st().item_val.sv_light_lantern
    {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Refueling with %s.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('F' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
    crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_maintain_light() -> borg_need {
    let mut i: ::core::ffi::c_int = 0;
    let mut current_light: *mut borg_item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if flag_has(
        &raw mut (*current_light).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_NO_FUEL as ::core::ffi::c_int,
    ) {
        return BORG_NO_NEED;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER {
        return BORG_NO_NEED;
    }
    if (*current_light).tval as ::core::ffi::c_int == TV_LIGHT as ::core::ffi::c_int {
        if (*current_light).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_light_torch {
            if (*current_light).timeout as ::core::ffi::c_int > 250 as ::core::ffi::c_int {
                return BORG_NO_NEED;
            } else {
                i = borg_slot(TV_LIGHT as ::core::ffi::c_int, crate::bridge::st().item_val.sv_light_torch);
                if i < 0 as ::core::ffi::c_int {
                    return BORG_UNMET_NEED;
                }
                return BORG_NO_NEED;
            }
        }
        if (*current_light).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_light_lantern {
            if ((*borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
            ))
            .timeout as ::core::ffi::c_int)
                < 1000 as ::core::ffi::c_int
            {
                if borg_refuel_lantern() {
                    return BORG_MET_NEED;
                }
                return BORG_UNMET_NEED;
            }
        }
        return BORG_NO_NEED;
    } else {
        i = borg_slot(TV_LIGHT as ::core::ffi::c_int, crate::bridge::st().item_val.sv_light_lantern);
        if i < 0 as ::core::ffi::c_int {
            i = borg_slot(TV_LIGHT as ::core::ffi::c_int, crate::bridge::st().item_val.sv_light_torch);
        }
        if i < 0 as ::core::ffi::c_int {
            return BORG_UNMET_NEED;
        } else {
            borg_keypress('w' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
                    as keycode_t,
            );
            return BORG_MET_NEED;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_light_beam(mut simulation: bool) -> bool {
    let mut dir: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut spell_ok: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut blocked: bool = false_0 != 0;
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if borg_spell_okay_fail(SPEAR_OF_LIGHT, 20 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || -(1 as ::core::ffi::c_int) != borg_slot(TV_WAND as ::core::ffi::c_int, crate::bridge::st().item_val.sv_wand_light)
            && (*borg_items
                .offset(borg_slot(TV_WAND as ::core::ffi::c_int, crate::bridge::st().item_val.sv_wand_light) as isize))
            .pval as ::core::ffi::c_int
                != 0
        || borg_equips_rod(crate::bridge::st().item_val.sv_rod_light) as ::core::ffi::c_int != 0
    {
        spell_ok = true_0 != 0;
    }
    if crate::bridge::st().borg.c.y
        - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        - 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        ag = crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y
                - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int)) as *mut borg_grid;
        if panel_contains(
            (crate::bridge::st().borg.c.y
                - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            crate::bridge::st().borg.c.x as ::core::ffi::c_uint,
        ) {
            i = 0 as ::core::ffi::c_int;
            while i <= crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] {
                if (crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y - i, crate::bridge::st().borg.c.x) as ::core::ffi::c_int != 0
                    || (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y - i))).feat
                        as ::core::ffi::c_int
                        == FEAT_NONE as ::core::ffi::c_int)
                    && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                    && blocked as ::core::ffi::c_int == false_0
                {
                    dir = 8 as ::core::ffi::c_int;
                    i += 1;
                } else {
                    dir = 5 as ::core::ffi::c_int;
                    blocked = true_0 != 0;
                    break;
                }
            }
        }
    }
    if (crate::bridge::st().borg.c.y
        + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        + 1 as ::core::ffi::c_int)
        < AUTO_MAX_Y
        && dir == 5 as ::core::ffi::c_int
    {
        blocked = false_0 != 0;
        ag = crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y
                + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int)) as *mut borg_grid;
        if panel_contains(
            (crate::bridge::st().borg.c.y
                + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            crate::bridge::st().borg.c.x as ::core::ffi::c_uint,
        ) {
            i = 0 as ::core::ffi::c_int;
            while i <= crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] {
                if (crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y + i, crate::bridge::st().borg.c.x) as ::core::ffi::c_int != 0
                    || (*crate::bridge::grid(crate::bridge::st().borg.c.x, (crate::bridge::st().borg.c.y + i))).feat
                        as ::core::ffi::c_int
                        == FEAT_NONE as ::core::ffi::c_int)
                    && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                    && blocked as ::core::ffi::c_int == false_0
                {
                    dir = 2 as ::core::ffi::c_int;
                    i += 1;
                } else {
                    dir = 5 as ::core::ffi::c_int;
                    blocked = true_0 != 0;
                    break;
                }
            }
        }
    }
    if (crate::bridge::st().borg.c.x
        + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        + 1 as ::core::ffi::c_int)
        < AUTO_MAX_X
        && dir == 5 as ::core::ffi::c_int
    {
        blocked = false_0 != 0;
        ag = crate::bridge::grid((crate::bridge::st().borg.c.x
                + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int), crate::bridge::st().borg.c.y) as *mut borg_grid;
        if panel_contains(
            crate::bridge::st().borg.c.y as ::core::ffi::c_uint,
            (crate::bridge::st().borg.c.x
                + crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        ) {
            i = 0 as ::core::ffi::c_int;
            while i <= crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] {
                if (crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x + i) as ::core::ffi::c_int != 0
                    || (*crate::bridge::grid((crate::bridge::st().borg.c.x + i), crate::bridge::st().borg.c.y)).feat
                        as ::core::ffi::c_int
                        == FEAT_NONE as ::core::ffi::c_int)
                    && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                    && blocked as ::core::ffi::c_int == false_0
                {
                    dir = 6 as ::core::ffi::c_int;
                    i += 1;
                } else {
                    dir = 5 as ::core::ffi::c_int;
                    blocked = true_0 != 0;
                    break;
                }
            }
        }
    }
    if crate::bridge::st().borg.c.x
        - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
        - 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
        && dir == 5 as ::core::ffi::c_int
    {
        blocked = false_0 != 0;
        ag = crate::bridge::grid((crate::bridge::st().borg.c.x
                - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int), crate::bridge::st().borg.c.y) as *mut borg_grid;
        if panel_contains(
            crate::bridge::st().borg.c.y as ::core::ffi::c_uint,
            (crate::bridge::st().borg.c.x
                - crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        ) {
            i = 1 as ::core::ffi::c_int;
            while i <= crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] {
                if (crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x - i) as ::core::ffi::c_int != 0
                    || (*crate::bridge::grid((crate::bridge::st().borg.c.x - i), crate::bridge::st().borg.c.y)).feat
                        as ::core::ffi::c_int
                        == FEAT_NONE as ::core::ffi::c_int)
                    && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                    && blocked as ::core::ffi::c_int == false_0
                {
                    dir = 4 as ::core::ffi::c_int;
                    i += 1;
                } else {
                    dir = 5 as ::core::ffi::c_int;
                    blocked = true_0 != 0;
                    break;
                }
            }
        }
    }
    if dir == 5 as ::core::ffi::c_int
        || spell_ok as ::core::ffi::c_int == false_0
        || blocked as ::core::ffi::c_int == true_0
        || dir == 2 as ::core::ffi::c_int && crate::bridge::st().borg.c.y > crate::borg_cave_util::borg_panel_hgt() - 2 as ::core::ffi::c_int
        || dir == 8 as ::core::ffi::c_int && crate::bridge::st().borg.c.y > crate::borg_cave_util::borg_panel_hgt() - 2 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if simulation {
        return true_0 != 0;
    }
    if borg_spell_fail(SPEAR_OF_LIGHT, 20 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_zap_rod(crate::bridge::st().item_val.sv_rod_light) as ::core::ffi::c_int != 0
        || borg_aim_wand(crate::bridge::st().item_val.sv_wand_light) as ::core::ffi::c_int != 0
    {
        borg_keypress((dir + '0' as i32) as keycode_t);
        borg_note(b"# Illuminating this hallway\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    return false_0 != 0;
}
