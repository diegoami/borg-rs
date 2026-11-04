pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const PW_BORG_1: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEY_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_cfg, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH,
    BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT,
    BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT,
    BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER,
    BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS,
    BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM,
    BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS,
    BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD,
    BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED,
};

use crate::headers::borg_think_h::borg_oops;
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};
pub use crate::headers::game_event_h::{
    event_add_handler, event_remove_handler, event_signal_message, game_event_data,
    game_event_handler, game_event_type, C2RustUnnamedH_0, C2RustUnnamedH_1, C2RustUnnamedH_2,
    C2RustUnnamedH_3, C2RustUnnamedH_4, C2RustUnnamedH_5, C2RustUnnamedH_6, C2RustUnnamedH_7,
    EVENT_AC, EVENT_ANIMATE, EVENT_BELL, EVENT_BIRTHPOINTS, EVENT_BOLT, EVENT_CHEAT_DEATH,
    EVENT_CHECK_INTERRUPT, EVENT_COMMAND_REPEAT, EVENT_DETECTIONSTATUS, EVENT_DUNGEONLEVEL,
    EVENT_END, EVENT_ENTER_BIRTH, EVENT_ENTER_DEATH, EVENT_ENTER_GAME, EVENT_ENTER_INIT,
    EVENT_ENTER_STORE, EVENT_ENTER_WORLD, EVENT_EQUIPMENT, EVENT_EXPERIENCE, EVENT_EXPLOSION,
    EVENT_FEELING, EVENT_GEN_LEVEL_END, EVENT_GEN_LEVEL_START, EVENT_GEN_ROOM_CHOOSE_SIZE,
    EVENT_GEN_ROOM_CHOOSE_SUBTYPE, EVENT_GEN_ROOM_END, EVENT_GEN_ROOM_START,
    EVENT_GEN_TUNNEL_FINISHED, EVENT_GOLD, EVENT_HP, EVENT_INITSTATUS, EVENT_INPUT_FLUSH,
    EVENT_INVENTORY, EVENT_ITEMLIST, EVENT_LEAVE_BIRTH, EVENT_LEAVE_DEATH, EVENT_LEAVE_GAME,
    EVENT_LEAVE_INIT, EVENT_LEAVE_STORE, EVENT_LEAVE_WORLD, EVENT_LIGHT, EVENT_MANA, EVENT_MAP,
    EVENT_MESSAGE, EVENT_MESSAGE_FLUSH, EVENT_MISSILE, EVENT_MONSTERHEALTH, EVENT_MONSTERLIST,
    EVENT_MONSTERTARGET, EVENT_NEW_LEVEL_DISPLAY, EVENT_OBJECTTARGET, EVENT_PLAYERLEVEL,
    EVENT_PLAYERMOVED, EVENT_PLAYERSPEED, EVENT_PLAYERTITLE, EVENT_RACE_CLASS, EVENT_REFRESH,
    EVENT_SEEFLOOR, EVENT_SOUND, EVENT_STATE, EVENT_STATS, EVENT_STATUS, EVENT_STORECHANGED,
    EVENT_STUDYSTATUS, EVENT_USE_STORE,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::list_message_h::{
    MSG_ACT_ARTIFACT, MSG_AFRAID, MSG_AMBIENT_DAY, MSG_AMBIENT_DNG1, MSG_AMBIENT_DNG2,
    MSG_AMBIENT_DNG3, MSG_AMBIENT_DNG4, MSG_AMBIENT_DNG5, MSG_AMBIENT_NITE, MSG_BELL, MSG_BERSERK,
    MSG_BIRTH, MSG_BLESSED, MSG_BLIND, MSG_BOLD, MSG_BR_ACID, MSG_BR_CHAOS, MSG_BR_DARK,
    MSG_BR_DISEN, MSG_BR_ELEC, MSG_BR_ELEMENTS, MSG_BR_FIRE, MSG_BR_FORCE, MSG_BR_FROST,
    MSG_BR_GAS, MSG_BR_GRAVITY, MSG_BR_INERTIA, MSG_BR_LIGHT, MSG_BR_NETHER, MSG_BR_NEXUS,
    MSG_BR_PLASMA, MSG_BR_SHARDS, MSG_BR_SOUND, MSG_BR_TIME, MSG_CAST_FEAR, MSG_CONFUSED,
    MSG_CREATE_TRAP, MSG_CURSED, MSG_CUT, MSG_DEATH, MSG_DESTROY, MSG_DIG, MSG_DISARM,
    MSG_DRAIN_STAT, MSG_DROP, MSG_DRUGGED, MSG_EAT, MSG_FLEE, MSG_GENERIC, MSG_HERO, MSG_HIT,
    MSG_HITPOINT_WARN, MSG_HITWALL, MSG_HIT_GOOD, MSG_HIT_GREAT, MSG_HIT_HI_GREAT,
    MSG_HIT_HI_SUPERB, MSG_HIT_SUPERB, MSG_HUNGRY, MSG_IDENT_ART, MSG_IDENT_BAD, MSG_IDENT_EGO,
    MSG_INFRARED, MSG_INVULN, MSG_KILL, MSG_KILL_KING, MSG_KILL_UNIQUE, MSG_LEVEL, MSG_LOCKPICK,
    MSG_LOCKPICK_FAIL, MSG_MAX, MSG_MISS, MSG_MONEY1, MSG_MONEY2, MSG_MONEY3, MSG_MON_BEG,
    MSG_MON_BITE, MSG_MON_BUTT, MSG_MON_CLAW, MSG_MON_CRAWL, MSG_MON_CRUSH, MSG_MON_DROOL,
    MSG_MON_ENGULF, MSG_MON_GAZE, MSG_MON_HIT, MSG_MON_INSULT, MSG_MON_KICK, MSG_MON_MOAN,
    MSG_MON_PUNCH, MSG_MON_SPIT, MSG_MON_SPORE, MSG_MON_STING, MSG_MON_TOUCH, MSG_MON_WAIL,
    MSG_MULTIPLY, MSG_NOTHING_TO_OPEN, MSG_NOTICE, MSG_OPENDOOR, MSG_PARALYZED, MSG_POISONED,
    MSG_PRAYER, MSG_PROT_EVIL, MSG_QUAFF, MSG_QUIVER, MSG_RECOVER, MSG_RES_ACID, MSG_RES_COLD,
    MSG_RES_ELEC, MSG_RES_FIRE, MSG_RES_POIS, MSG_RUNE, MSG_SCRAMBLE, MSG_SEE_INVIS, MSG_SHIELD,
    MSG_SHOOT, MSG_SHOOT_HIT, MSG_SHRIEK, MSG_SHUTDOOR, MSG_SLOW, MSG_SPEED, MSG_SPELL,
    MSG_STAIRS_DOWN, MSG_STAIRS_UP, MSG_STORE1, MSG_STORE2, MSG_STORE3, MSG_STORE4, MSG_STORE5,
    MSG_STORE_ENTER, MSG_STORE_HOME, MSG_STORE_LEAVE, MSG_STUDY, MSG_STUN, MSG_SUM_AINU,
    MSG_SUM_ANIMAL, MSG_SUM_DEMON, MSG_SUM_DRAGON, MSG_SUM_HI_DEMON, MSG_SUM_HI_DRAGON,
    MSG_SUM_HI_UNDEAD, MSG_SUM_HOUND, MSG_SUM_HYDRA, MSG_SUM_MONSTER, MSG_SUM_SPIDER,
    MSG_SUM_UNDEAD, MSG_SUM_UNIQUE, MSG_SUM_WRAITH, MSG_TELEPORT, MSG_TPLEVEL, MSG_TPOTHER,
    MSG_USE_STAFF, MSG_WALK, MSG_WIELD, MSG_ZAP_ROD,
};
pub use crate::headers::message_h::{message_add, msg, C2RustUnnamedH_43, SOUND_MAX};
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

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::stdlib_h::wcstombs;
use crate::headers::string_h::{memcpy, strlen, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    angband_term, term, term_win, window_flag, Term, Term_activate, Term_addstr, Term_erase,
    Term_fresh, Term_get_size, Term_locate,
};
pub use crate::headers::z_bitflag_h::bitflag;

pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
use crate::headers::z_util_h::text_mbstowcs;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_confirm_target: bool = false;
#[no_mangle]
pub unsafe extern "C" fn borg_what_text(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut a: *mut uint8_t,
    mut s: *mut ::core::ffi::c_char,
) -> errr {
    let mut i: ::core::ffi::c_int = 0;
    let mut screen_str: [wchar_t; 1024] = [0; 1024];
    let mut t_a: ::core::ffi::c_int = 0;
    let mut t_c: wchar_t = 0;
    let mut aa: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut cc: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut d_a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m: ::core::ffi::c_int = if n < 0 as ::core::ffi::c_int { -n } else { n };
    Term_get_size(&raw mut w, &raw mut h);
    if x + m > w {
        m = w - x;
    }
    aa = (*(*(*Term).scr).a.offset(y as isize)).offset(x as isize) as *mut ::core::ffi::c_int;
    cc = (*(*(*Term).scr).c.offset(y as isize)).offset(x as isize) as *mut wchar_t;
    i = 0 as ::core::ffi::c_int;
    while i < m {
        let fresh0 = aa;
        aa = aa.offset(1);
        t_a = *fresh0;
        let fresh1 = cc;
        cc = cc.offset(1);
        t_c = *fresh1;
        if t_c == ' ' as i32 || t_a == 0 {
            screen_str[i as usize] = ' ' as i32 as wchar_t;
        } else {
            if d_a != 0 {
                if t_a != d_a {
                    break;
                }
            } else {
                d_a = t_a;
            }
            screen_str[i as usize] = t_c;
        }
        i += 1;
    }
    screen_str[i as usize] = '\0' as i32 as wchar_t;
    *a = d_a as uint8_t;
    wcstombs(
        s,
        &raw mut screen_str as *mut wchar_t,
        ((if n < 0 as ::core::ffi::c_int { -n } else { n }) + 1 as ::core::ffi::c_int) as size_t,
    );
    if n > 0 as ::core::ffi::c_int && i != n {
        return 1 as errr;
    }
    return 0 as errr;
}
unsafe extern "C" fn borg_info(mut what: *const ::core::ffi::c_char) {}
unsafe extern "C" fn borg_note_internal(mut warning: bool, mut what: *const ::core::ffi::c_char) {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut old: *mut term = Term;
    if warning {
        msg(b"%s\0" as *const u8 as *const ::core::ffi::c_char, what);
    } else {
        message_add(what, MSG_GENERIC as ::core::ffi::c_int as uint16_t);
        event_signal_message(
            EVENT_MESSAGE,
            MSG_GENERIC as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    borg_info(what);
    if !strstr(
        what,
        b"Best Combo\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            what,
            b"Taking off \0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        borg.antibounce_count =
            (borg.antibounce_count as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as int16_t;
        borg_note(format(
            b"# Anti-loop variable tick (%d).\0" as *const u8 as *const ::core::ffi::c_char,
            borg.antibounce_count as ::core::ffi::c_int,
        ));
    }
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        if !angband_term[j as usize].is_null() {
            if !(window_flag[j as usize] as ::core::ffi::c_long & PW_BORG_1 == 0) {
                Term_activate(angband_term[j as usize]);
                Term_get_size(&raw mut w, &raw mut h);
                Term_locate(&raw mut x, &raw mut y);
                Term_erase(0 as ::core::ffi::c_int, y, w);
                n = strlen(what) as ::core::ffi::c_int;
                if n > w - 2 as ::core::ffi::c_int {
                    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
                    while n > w - 2 as ::core::ffi::c_int {
                        k = w - 2 as ::core::ffi::c_int;
                        i = w / 2 as ::core::ffi::c_int;
                        while i < w - 2 as ::core::ffi::c_int {
                            if *(*__ctype_b_loc())
                                .offset(*what.offset(i as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    as isize) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0
                            {
                                k = i;
                            }
                            i += 1;
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < k {
                            buf[i as usize] = *what.offset(i as isize);
                            i += 1;
                        }
                        let fresh2 = i;
                        i = i + 1;
                        buf[fresh2 as usize] = '\\' as i32 as ::core::ffi::c_char;
                        buf[i as usize] = '\0' as i32 as ::core::ffi::c_char;
                        Term_addstr(
                            -(1 as ::core::ffi::c_int),
                            COLOUR_WHITE,
                            &raw mut buf as *mut ::core::ffi::c_char,
                        );
                        y += 1;
                        if y >= h {
                            y = 0 as ::core::ffi::c_int;
                        }
                        Term_erase(0 as ::core::ffi::c_int, y, w);
                        what = what.offset(k as isize);
                        n -= k;
                    }
                    Term_addstr(-(1 as ::core::ffi::c_int), COLOUR_WHITE, what);
                    y += 1;
                    if y >= h {
                        y = 0 as ::core::ffi::c_int;
                    }
                    Term_erase(0 as ::core::ffi::c_int, y, w);
                } else {
                    Term_addstr(-(1 as ::core::ffi::c_int), COLOUR_WHITE, what);
                    y += 1;
                    if y >= h {
                        y = 0 as ::core::ffi::c_int;
                    }
                    Term_erase(0 as ::core::ffi::c_int, y, w);
                }
                Term_fresh();
                Term_activate(old);
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_warning(mut what: *const ::core::ffi::c_char) {
    borg_note_internal(true_0 != 0, what);
}
#[no_mangle]
pub unsafe extern "C" fn borg_note(mut what: *const ::core::ffi::c_char) {
    borg_note_internal(false_0 != 0, what);
}
static mut borg_key_queue: *mut keycode_t = ::core::ptr::null::<keycode_t>() as *mut keycode_t;
static mut borg_key_head: int16_t = 0;
static mut borg_key_tail: int16_t = 0;
static mut borg_key_history: *mut keypress = ::core::ptr::null::<keypress>() as *mut keypress;
static mut borg_key_history_head: int16_t = 0;
static mut borg_queued_direction: keycode_t = 0 as keycode_t;
#[no_mangle]
pub unsafe extern "C" fn borg_keypress(mut k: keycode_t) -> errr {
    if k == 0 {
        borg_note(b" & Key * *BAD KEY * *\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as errr);
    }
    let fresh3 = borg_key_head;
    borg_key_head = borg_key_head + 1;
    *borg_key_queue.offset(fresh3 as isize) = k;
    if borg_key_head as ::core::ffi::c_int == KEY_SIZE {
        borg_key_head = 0 as int16_t;
    }
    if borg_key_head as ::core::ffi::c_int == borg_key_tail as ::core::ffi::c_int {
        borg_oops(b"overflow\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if borg_key_tail as ::core::ffi::c_int == KEY_SIZE {
        borg_key_tail = 0 as int16_t;
    }
    return 0 as errr;
}
#[no_mangle]
pub unsafe extern "C" fn borg_save_keypress(mut kp: keypress) -> keypress {
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        if kp.type_0 as ::core::ffi::c_uint == EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: keycode_t = kp.code;
            if k >= 32 as keycode_t && k <= 126 as keycode_t {
                borg_note(format(
                    b"& Key <%c> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_char as ::core::ffi::c_int,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == KC_ENTER as keycode_t {
                borg_note(format(
                    b"& Key <Enter> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == ESCAPE as keycode_t {
                borg_note(format(
                    b"& Key <Esc> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else {
                borg_note(format(
                    b"& Key <0x%02lX>\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            }
        } else {
            borg_note(format(
                b"& non-Keyboard <0x%02X>\0" as *const u8 as *const ::core::ffi::c_char,
                kp.type_0 as ::core::ffi::c_uint,
            ));
        }
    }
    (*borg_key_history.offset(borg_key_history_head as isize)).code = kp.code;
    let fresh4 = borg_key_history_head;
    borg_key_history_head = borg_key_history_head + 1;
    (*borg_key_history.offset(fresh4 as isize)).type_0 = kp.type_0;
    if borg_key_history_head as ::core::ffi::c_int == KEY_SIZE {
        memcpy(
            borg_key_history as *mut ::core::ffi::c_void,
            borg_key_history.offset((KEY_SIZE - 101 as ::core::ffi::c_int) as isize)
                as *mut keypress as *const ::core::ffi::c_void,
            (::core::mem::size_of::<keypress>() as size_t).wrapping_mul(100 as size_t),
        );
        borg_key_history_head = 100 as int16_t;
    }
    return kp;
}
#[no_mangle]
pub unsafe extern "C" fn borg_keypresses(mut str: *const ::core::ffi::c_char) -> errr {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = str;
    while *s != 0 {
        borg_keypress(*s as keycode_t);
        s = s.offset(1);
    }
    return 0 as errr;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inkey(mut take: bool) -> keycode_t {
    let mut i: ::core::ffi::c_int = 0;
    if borg_key_head as ::core::ffi::c_int == borg_key_tail as ::core::ffi::c_int {
        return 0 as keycode_t;
    }
    i = *borg_key_queue.offset(borg_key_tail as isize) as ::core::ffi::c_int;
    if !take {
        return i as keycode_t;
    }
    borg_key_tail += 1;
    if borg_key_tail as ::core::ffi::c_int == KEY_SIZE {
        borg_key_tail = 0 as int16_t;
    }
    return i as keycode_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flush() {
    borg_key_tail = borg_key_head;
    borg_queued_direction = 0 as keycode_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_queue_direction(mut k: keycode_t) {
    borg_queued_direction = k;
    borg_confirm_target = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_queued_direction() -> keycode_t {
    let mut k: keycode_t = borg_queued_direction;
    borg_queued_direction = 0 as keycode_t;
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn borg_massage_special_chars(
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut wide_name: [wchar_t; 1024] = [0; 1024];
    let mut memory: *mut ::core::ffi::c_char = mem_zalloc(
        strlen(name)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    text_mbstowcs(
        &raw mut wide_name as *mut wchar_t,
        name,
        strlen(name).wrapping_add(1 as size_t) as ::core::ffi::c_int,
    );
    wcstombs(
        memory,
        &raw mut wide_name as *mut wchar_t,
        strlen(name).wrapping_add(1 as size_t),
    );
    return memory;
}
#[no_mangle]
pub unsafe extern "C" fn borg_dump_recent_keys(mut num: ::core::ffi::c_int) {
    let mut end: ::core::ffi::c_int = borg_key_history_head as ::core::ffi::c_int;
    let mut start: ::core::ffi::c_int = if (borg_key_history_head as ::core::ffi::c_int) < num {
        0 as ::core::ffi::c_int
    } else {
        borg_key_history_head as ::core::ffi::c_int - num
    };
    while start < end {
        let mut kp: *mut keypress = borg_key_history.offset(start as isize) as *mut keypress;
        if (*kp).type_0 as ::core::ffi::c_uint
            == EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: keycode_t = (*kp).code;
            if k >= 32 as keycode_t && k <= 126 as keycode_t {
                borg_note(format(
                    b"& Key history <%c> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_char as ::core::ffi::c_int,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == KC_ENTER as keycode_t {
                borg_note(format(
                    b"& Key history <Enter> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == ESCAPE as keycode_t {
                borg_note(format(
                    b"& Key history <Esc> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else {
                borg_note(format(
                    b"& Key history <0x%02lX>\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            }
        } else {
            borg_note(format(
                b"& non-Keyboard <0x%02X>\0" as *const u8 as *const ::core::ffi::c_char,
                (*kp).type_0 as ::core::ffi::c_uint,
            ));
        }
        start += 1;
    }
}
unsafe extern "C" fn borg_bell(
    mut unused: game_event_type,
    mut data: *mut game_event_data,
    mut user: *mut ::core::ffi::c_void,
) {
    borg_note(
        b"** BELL SOUNDED Dumping keypress history ***\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    borg_dump_recent_keys(20 as ::core::ffi::c_int);
    if *borg_cfg.offset(BORG_STOP_ON_BELL as ::core::ffi::c_int as isize) != 0 {
        borg_oops(b"** BELL SOUNDED***\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_io() {
    borg_free_io();
    borg_key_queue = mem_zalloc(
        (KEY_SIZE as size_t).wrapping_mul(::core::mem::size_of::<keycode_t>() as size_t),
    ) as *mut keycode_t;
    borg_key_history =
        mem_zalloc((KEY_SIZE as size_t).wrapping_mul(::core::mem::size_of::<keypress>() as size_t))
            as *mut keypress;
    event_add_handler(
        EVENT_BELL,
        Some(
            borg_bell
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_io() {
    event_remove_handler(
        EVENT_BELL,
        Some(
            borg_bell
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    if !borg_key_history.is_null() {
        mem_free(borg_key_history as *mut ::core::ffi::c_void);
    }
    borg_key_history = ::core::ptr::null_mut::<keypress>();
    if !borg_key_queue.is_null() {
        mem_free(borg_key_queue as *mut ::core::ffi::c_void);
    }
    borg_key_queue = ::core::ptr::null_mut::<keycode_t>();
}
