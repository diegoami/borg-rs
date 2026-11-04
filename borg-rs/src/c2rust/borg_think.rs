pub const BORG_TIME_MAX: ::core::ffi::c_int = INT32_MAX;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{
    borg_flush, borg_keypress, borg_note, borg_warning, borg_what_text,
};
use crate::headers::borg_item_h::borg_find_all_items;
use crate::headers::borg_magic_h::borg_cheat_spells;
use crate::headers::borg_power_h::borg_power;
use crate::headers::borg_think_dungeon_h::borg_think_dungeon;
use crate::headers::borg_think_store_h::borg_think_store;
pub use crate::headers::borg_trait_h::{
    borg_notice, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
use crate::headers::borg_update_h::borg_update;
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};
pub use crate::headers::h_basic_h::errr;
use crate::headers::init_h::ANGBAND_DIR_ARCHIVE;
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
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
use crate::headers::player_util_h::player_is_shapechanged;

pub use crate::headers::stdint_h::INT32_MAX;
pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::{memcpy, strcmp, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_game_h::savefile;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_file_h::path_build;
use crate::headers::z_form_h::{format, quit_fmt};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
#[no_mangle]
pub static mut shop_num: int16_t = -(1 as ::core::ffi::c_int) as int16_t;
#[no_mangle]
pub static mut borg_do_spell: bool = true_0 != 0;
#[no_mangle]
pub unsafe extern "C" fn borg_oops(mut what: *const ::core::ffi::c_char) {
    crate::bridge::st().borg.status.active = false_0 != 0;
    borg_note(format(
        b"# Aborting (%s).\0" as *const u8 as *const ::core::ffi::c_char,
        what,
    ));
    borg_flush();
}
unsafe extern "C" fn borg_save_game() -> bool {
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress('^' as i32 as keycode_t);
    borg_keypress('S' as i32 as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    return true_0 != 0;
}
unsafe extern "C" fn borg_check_save() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    static mut svSavefile: [::core::ffi::c_char; 1024] = [0; 1024];
    static mut svSavefile2: [::core::ffi::c_char; 1024] = [0; 1024];
    static mut justSaved: bool = false_0 != 0;
    if crate::bridge::st().borg.status.save as ::core::ffi::c_int != 0 && borg_save_game() as ::core::ffi::c_int != 0 {
        borg_note(b"# Auto Save!\0" as *const u8 as *const ::core::ffi::c_char);
        crate::bridge::st().borg.status.save = false_0 != 0;
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            >= crate::bridge::st().core.cfg[(BORG_DUMP_LEVEL as ::core::ffi::c_int) as usize]
            || !strstr(
                &raw mut (*player).died_from as *mut ::core::ffi::c_char,
                b"starvation\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            memcpy(
                &raw mut svSavefile as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                &raw mut savefile as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while (*player).full_name[i as usize] != 0 {
                let mut c: ::core::ffi::c_char = (*player).full_name[i as usize];
                if *(*__ctype_b_loc())
                    .offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _IScntrl as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                {
                    quit_fmt(
                        b"Illegal control char (0x%02X) in player name\0" as *const u8
                            as *const ::core::ffi::c_char,
                        c as ::core::ffi::c_int,
                    );
                }
                if *(*__ctype_b_loc())
                    .offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    == 0
                    && *(*__ctype_b_loc())
                        .offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        == 0
                {
                    c = '_' as i32 as ::core::ffi::c_char;
                }
                svSavefile2[i as usize] = c;
                i += 1;
            }
            svSavefile2[i as usize] = 0 as ::core::ffi::c_char;
            path_build(
                &raw mut savefile as *mut ::core::ffi::c_char,
                1024 as size_t,
                ANGBAND_DIR_ARCHIVE,
                &raw mut svSavefile2 as *mut ::core::ffi::c_char,
            );
            justSaved = true_0 != 0;
        }
        return true_0 != 0;
    }
    if justSaved {
        memcpy(
            &raw mut savefile as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut svSavefile as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        );
        borg_save_game();
        justSaved = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think() -> bool {
    let mut t_a: uint8_t = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    borg_find_all_items();
    if borg_check_save() {
        return true_0 != 0;
    }
    if borg_do_spell {
        borg_cheat_spells();
        borg_do_spell = false_0 != 0;
    }
    if player_is_shapechanged(player) {
        borg_keypress('v' as i32 as keycode_t);
        borg_keypress('r' as i32 as keycode_t);
        return true_0 != 0;
    }
    borg_notice(true_0 != 0);
    borg_update();
    crate::bridge::st().borg.power = borg_power();
    if 0 as ::core::ffi::c_int
        == borg_what_text(
            1 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw mut t_a,
            &raw mut buf as *mut ::core::ffi::c_char,
        )
        && (strcmp(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"Stor\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"Home\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0)
    {
        return borg_think_store();
    }
    borg_do_spell = true_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
    {
        crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] = crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize];
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        > crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
    {
        crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize] = crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize];
    }
    crate::bridge::st().borg.time.now += 1;
    if crate::bridge::st().borg.time.now > BORG_TIME_MAX as borg_time - 100 as borg_time {
        crate::bridge::st().borg.time.now = 1 as ::core::ffi::c_int as borg_time;
        borg_warning(
            b"*****WARNING***** Borg Clock Overflow\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    crate::bridge::st().borg.antibounce_count += 1;
    if crate::bridge::st().borg.status.cancel {
        return true_0 != 0;
    }
    return borg_think_dungeon();
}
