pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::borg_keypress;

use crate::headers::borg_item_decurse_h::{
    borg_decurse_any, borg_decurse_armour, borg_decurse_weapon,
};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_use_h::{borg_activate_item, borg_read_scroll};

pub use crate::headers::borg_magic_h::{
    borg_spell_fail, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_trait_swap_h::{
    enchant_weapon_swap_to_d, enchant_weapon_swap_to_h, weapon_swap,
};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
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
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
unsafe extern "C" fn borg_pick_armor(mut i: ::core::ffi::c_int) {
    if i >= (*z_info).pack_size as ::core::ffi::c_int {
        let mut found: bool = false_0 != 0;
        let mut e: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while e < (*z_info).pack_size as ::core::ffi::c_int {
            if (*borg_items.offset(e as isize)).iqty as ::core::ffi::c_int != 0
                && ((*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                    == TV_BOOTS as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_GLOVES as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_CLOAK as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_CROWN as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_HELM as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_SHIELD as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_HELM as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_SOFT_ARMOR as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_HARD_ARMOR as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_DRAG_ARMOR as ::core::ffi::c_int)
            {
                found = true_0 != 0;
                break;
            } else {
                e += 1;
            }
        }
        if found {
            borg_keypress('/' as i32 as keycode_t);
        }
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                as keycode_t,
        );
    } else {
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
                as keycode_t,
        );
    };
}
unsafe extern "C" fn borg_pick_weapon(mut i: ::core::ffi::c_int) {
    if i < (*z_info).pack_size as ::core::ffi::c_int {
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
                as keycode_t,
        );
    } else {
        let mut found: bool = false_0 != 0;
        let mut e: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while e < (*z_info).pack_size as ::core::ffi::c_int {
            if (*borg_items.offset(e as isize)).iqty as ::core::ffi::c_int != 0
                && ((*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                    == TV_BOW as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_DIGGING as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_HAFTED as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_POLEARM as ::core::ffi::c_int
                    || (*borg_items.offset(e as isize)).tval as ::core::ffi::c_int
                        == TV_SWORD as ::core::ffi::c_int)
            {
                found = true_0 != 0;
                break;
            } else {
                e += 1;
            }
        }
        if i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
            if found {
                borg_keypress('/' as i32 as keycode_t);
            }
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
        } else {
            if found as ::core::ffi::c_int != 0
                || (*borg_items.offset((*z_info).pack_size as isize)).iqty as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                || (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .iqty as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                borg_keypress('|' as i32 as keycode_t);
            }
            borg_keypress(
                ('0' as i32
                    + (i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)))
                    as keycode_t,
            );
        }
    };
}
unsafe extern "C" fn borg_enchant_to_a() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut a: ::core::ffi::c_int = 0;
    let mut b_a: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_ARM as ::core::ffi::c_int) as usize]
        == 0
        && crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize]
            == 0
    {
        return false_0 != 0;
    }
    let mut current_block_7: u64;
    i = (*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                a = (*item).to_a as ::core::ffi::c_int;
                if borg_spell_okay_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    != 0
                    || crate::bridge::st().borg.trait_[(BI_AENCH_SARM as ::core::ffi::c_int) as usize]
                        >= 1 as ::core::ffi::c_int
                {
                    if a >= crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize] {
                        current_block_7 = 17179679302217393232;
                    } else {
                        current_block_7 = 1841672684692190573;
                    }
                } else if a >= 8 as ::core::ffi::c_int {
                    current_block_7 = 17179679302217393232;
                } else {
                    current_block_7 = 1841672684692190573;
                }
                match current_block_7 {
                    17179679302217393232 => {}
                    _ => {
                        if !(b_i >= 0 as ::core::ffi::c_int && b_a < a) {
                            b_i = i;
                            b_a = a;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_read_scroll(crate::bridge::st().item_val.sv_scroll_star_enchant_armor) as ::core::ffi::c_int != 0
        || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_enchant_armor) as ::core::ffi::c_int != 0
        || borg_spell_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_pick_armor(b_i);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_enchant_to_h() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut a: ::core::ffi::c_int = 0;
    let mut s_a: ::core::ffi::c_int = 0;
    let mut b_a: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int) as usize]
        == 0
        && enchant_weapon_swap_to_h == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_TOH as ::core::ffi::c_int) as usize]
        == 0
        && crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
            == 0
    {
        return false_0 != 0;
    }
    let mut current_block_7: u64;
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !((*item).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int) {
                    a = (*item).to_h as ::core::ffi::c_int;
                    if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                        || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                            >= 1 as ::core::ffi::c_int
                    {
                        if a >= crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                        {
                            current_block_7 = 17179679302217393232;
                        } else {
                            current_block_7 = 12800627514080957624;
                        }
                    } else if a >= 8 as ::core::ffi::c_int {
                        current_block_7 = 17179679302217393232;
                    } else {
                        current_block_7 = 12800627514080957624;
                    }
                    match current_block_7 {
                        17179679302217393232 => {}
                        _ => {
                            if !(i
                                == (*z_info).pack_size as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                && crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                                    < 3 as ::core::ffi::c_int
                                && ((*item).art_idx == 0 && (*item).ego_idx == 0))
                            {
                                if !(b_i >= 0 as ::core::ffi::c_int && b_a < a) {
                                    b_i = i;
                                    b_a = a;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if weapon_swap != 0 {
        let mut skip: bool = false_0 != 0;
        let mut item_0: *mut borg_item =
            borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize) as *mut borg_item;
        if (*item_0).ident as ::core::ffi::c_int != 0
            && (*item_0).tval as ::core::ffi::c_int != TV_DIGGING as ::core::ffi::c_int
        {
            s_a = (*item_0).to_h as ::core::ffi::c_int;
            if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
                || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                    >= 1 as ::core::ffi::c_int
            {
                if s_a >= crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize] {
                    skip = true_0 != 0;
                }
            } else if s_a >= 8 as ::core::ffi::c_int {
                skip = true_0 != 0;
            }
            if b_a > s_a && !skip {
                b_i = weapon_swap - 1 as ::core::ffi::c_int;
                b_a = s_a;
            }
        }
    }
    if b_i < 0 as ::core::ffi::c_int {
        let mut current_block_28: u64;
        i = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
        while i
            < (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int
        {
            let mut item_1: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
            if !(crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                < 3 as ::core::ffi::c_int
                || (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .art_idx
                    == 0
                    && (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .ego_idx
                        == 0)
            {
                if !(((*item_1).iqty as ::core::ffi::c_int) < 5 as ::core::ffi::c_int) {
                    if (*item_1).ident {
                        if !((*item_1).tval as ::core::ffi::c_int
                            != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
                        {
                            a = (*item_1).to_h as ::core::ffi::c_int;
                            if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                != 0
                                || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                                    >= 1 as ::core::ffi::c_int
                            {
                                if a >= 10 as ::core::ffi::c_int {
                                    current_block_28 = 11913429853522160501;
                                } else {
                                    current_block_28 = 2543120759711851213;
                                }
                            } else if a >= 8 as ::core::ffi::c_int {
                                current_block_28 = 11913429853522160501;
                            } else {
                                current_block_28 = 2543120759711851213;
                            }
                            match current_block_28 {
                                11913429853522160501 => {}
                                _ => {
                                    if !(b_i >= 0 as ::core::ffi::c_int && b_a < a) {
                                        b_i = i;
                                        b_a = a;
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
    if b_i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_read_scroll(crate::bridge::st().item_val.sv_scroll_star_enchant_weapon) as ::core::ffi::c_int != 0
        || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_hit) as ::core::ffi::c_int != 0
        || borg_spell_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_pick_weapon(b_i);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_enchant_to_d() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut a: ::core::ffi::c_int = 0;
    let mut s_a: ::core::ffi::c_int = 0;
    let mut b_a: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int) as usize]
        == 0
        && enchant_weapon_swap_to_d == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_AENCH_TOD as ::core::ffi::c_int) as usize]
        == 0
        && crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
            == 0
    {
        return false_0 != 0;
    }
    let mut current_block_7: u64;
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !((*item).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int) {
                    a = (*item).to_d as ::core::ffi::c_int;
                    if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                        || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                            >= 1 as ::core::ffi::c_int
                    {
                        if a >= crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize]
                        {
                            current_block_7 = 17179679302217393232;
                        } else {
                            current_block_7 = 12800627514080957624;
                        }
                    } else if a >= 8 as ::core::ffi::c_int {
                        current_block_7 = 17179679302217393232;
                    } else {
                        current_block_7 = 12800627514080957624;
                    }
                    match current_block_7 {
                        17179679302217393232 => {}
                        _ => {
                            if !(i
                                == (*z_info).pack_size as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                && crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                                    < 3 as ::core::ffi::c_int
                                && ((*item).art_idx == 0 && (*item).ego_idx == 0))
                            {
                                if !(b_i >= 0 as ::core::ffi::c_int && b_a < a) {
                                    b_i = i;
                                    b_a = a;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if weapon_swap != 0 {
        let mut skip: bool = false_0 != 0;
        let mut item_0: *mut borg_item =
            borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize) as *mut borg_item;
        if (*item_0).ident as ::core::ffi::c_int != 0
            && (*item_0).tval as ::core::ffi::c_int != TV_DIGGING as ::core::ffi::c_int
        {
            s_a = (*item_0).to_d as ::core::ffi::c_int;
            if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
                || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                    >= 1 as ::core::ffi::c_int
            {
                if s_a >= crate::bridge::st().core.cfg[(BORG_ENCHANT_LIMIT as ::core::ffi::c_int) as usize] {
                    skip = true_0 != 0;
                }
            } else if s_a >= 8 as ::core::ffi::c_int {
                skip = true_0 != 0;
            }
            if b_a > s_a && !skip {
                b_i = weapon_swap - 1 as ::core::ffi::c_int;
                b_a = s_a;
            }
        }
    }
    if b_i < 0 as ::core::ffi::c_int {
        let mut current_block_28: u64;
        i = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
        while i
            < (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int
        {
            let mut item_1: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
            if !(crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                < 3 as ::core::ffi::c_int
                || (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .art_idx
                    == 0
                    && (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .ego_idx
                        == 0)
            {
                if !(crate::bridge::st().borg.trait_[(BI_AMMO_POWER as ::core::ffi::c_int) as usize]
                    < 3 as ::core::ffi::c_int)
                {
                    if !(((*item_1).iqty as ::core::ffi::c_int) < 5 as ::core::ffi::c_int) {
                        if (*item_1).ident {
                            if !((*item_1).tval as ::core::ffi::c_int
                                != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
                            {
                                a = (*item_1).to_d as ::core::ffi::c_int;
                                if borg_spell_okay_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                                    != 0
                                    || crate::bridge::st().borg.trait_[(BI_AENCH_SWEP as ::core::ffi::c_int) as usize]
                                        >= 1 as ::core::ffi::c_int
                                {
                                    if a >= 10 as ::core::ffi::c_int {
                                        current_block_28 = 11913429853522160501;
                                    } else {
                                        current_block_28 = 3938820862080741272;
                                    }
                                } else if a >= 8 as ::core::ffi::c_int {
                                    current_block_28 = 11913429853522160501;
                                } else {
                                    current_block_28 = 3938820862080741272;
                                }
                                match current_block_28 {
                                    11913429853522160501 => {}
                                    _ => {
                                        if !(b_i >= 0 as ::core::ffi::c_int && b_a < a) {
                                            b_i = i;
                                            b_a = a;
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
    if b_i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_read_scroll(crate::bridge::st().item_val.sv_scroll_star_enchant_weapon) as ::core::ffi::c_int != 0
        || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_enchant_weapon_to_dam) as ::core::ffi::c_int != 0
        || borg_spell_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_pick_weapon(b_i);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_brand_weapon() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut a: ::core::ffi::c_int = 0;
    let mut b_a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_NEED_BRAND_WEAPON as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ABRAND as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !(((*item).iqty as ::core::ffi::c_int) < 20 as ::core::ffi::c_int) {
            if (*item).ident {
                if !((*item).tval as ::core::ffi::c_int
                    != crate::bridge::st().borg.trait_[(BI_AMMO_TVAL as ::core::ffi::c_int) as usize])
                {
                    a = (*item).to_h as ::core::ffi::c_int;
                    if !((*item).ego_idx != 0) {
                        if !(b_i >= 0 as ::core::ffi::c_int && b_a > a) {
                            b_i = i;
                            b_a = a;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if borg_activate_item(crate::bridge::st().act.firebrand) as ::core::ffi::c_int != 0
        || borg_spell_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_pick_weapon(b_i);
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_enchanting() -> bool {
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 5500 as borg_time
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
        || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 3501 as borg_time
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_decurse_armour() {
        return true_0 != 0;
    }
    if borg_decurse_weapon() {
        return true_0 != 0;
    }
    if borg_decurse_any() {
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if borg_brand_weapon() {
        return true_0 != 0;
    }
    if borg_enchant_to_h() {
        return true_0 != 0;
    }
    if borg_enchant_to_d() {
        return true_0 != 0;
    }
    if borg_enchant_to_a() {
        return true_0 != 0;
    }
    return false_0 != 0;
}
