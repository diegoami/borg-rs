pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_dump_recent_keys, borg_flush, borg_get_queued_direction, borg_inkey, borg_note};
use crate::headers::borg_log_h::{borg_log_death, borg_log_death_data, borg_write_map};
use crate::headers::borg_messages_h::{
    borg_msg_buf, borg_msg_len, borg_msg_max, borg_msg_num, borg_msg_pos, borg_msg_siz,
    borg_msg_use, borg_parse,
};
use crate::headers::borg_reincarnate_h::borg_reincarnate_start;
use crate::headers::borg_think_h::borg_oops;
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
use crate::headers::cmds_h::do_cmd_wiz_cure_all;
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
use crate::headers::string_h::{strcmp, strlen, strncmp, strstr};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, ui_event_type, EVT_BUTTON, EVT_DISCONNECT, EVT_ESCAPE, EVT_KBRD,
    EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
use crate::headers::z_util_h::{my_strcpy, prefix};
#[no_mangle]
pub static mut borg_dont_react: bool = false_0 != 0;
#[no_mangle]
pub unsafe extern "C" fn borg_react(
    mut msg: *const ::core::ffi::c_char,
    mut buf: *const ::core::ffi::c_char,
) {
    let mut len: ::core::ffi::c_int = 0;
    if borg_dont_react as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_ISPARALYZED as ::core::ffi::c_int) as usize]
            != 0
    {
        borg_note(b"# Ignoring messages.\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        borg_note(format(
            b"> Reacting Msg (%s)\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
    }
    len = strlen(buf) as ::core::ffi::c_int;
    if len != 0
        && *buf.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ',' as i32
    {
        let mut tmp: *mut ::core::ffi::c_char = buf as *mut ::core::ffi::c_char;
        len -= 1;
        *tmp.offset(len as isize) = 0 as ::core::ffi::c_char;
    }
    if borg_msg_num as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        > borg_msg_max as ::core::ffi::c_int
    {
        borg_note(b"too many messages\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if borg_msg_len as ::core::ffi::c_int + len + 1 as ::core::ffi::c_int
        > borg_msg_siz as ::core::ffi::c_int
    {
        borg_note(b"too much messages\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    *borg_msg_use.offset(borg_msg_num as isize) = 0 as int16_t;
    *borg_msg_pos.offset(borg_msg_num as isize) = borg_msg_len;
    my_strcpy(
        borg_msg_buf.offset(borg_msg_len as ::core::ffi::c_int as isize),
        buf,
        (borg_msg_siz as ::core::ffi::c_int - borg_msg_len as ::core::ffi::c_int) as size_t,
    );
    borg_msg_len =
        (borg_msg_len as ::core::ffi::c_int + (len + 1 as ::core::ffi::c_int)) as int16_t;
    borg_msg_num += 1;
}
#[no_mangle]
pub unsafe extern "C" fn borg_react_prompted(
    mut buf: *const ::core::ffi::c_char,
    mut key: *mut keypress,
    mut cursor: loc,
) -> bool {
    if cursor.y == 0 as ::core::ffi::c_int {
        if prefix(buf, b"Die?\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
            && crate::bridge::st().borg.status.cheat_death as ::core::ffi::c_int != 0
        {
            borg_parse(::core::ptr::null_mut::<::core::ffi::c_char>());
            borg_flush();
            borg_note(b"# Cheating death...\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                >= crate::bridge::st().core.cfg[(BORG_DUMP_LEVEL as ::core::ffi::c_int) as usize]
                || !strstr(
                    &raw mut (*player).died_from as *mut ::core::ffi::c_char,
                    b"starvation\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
            {
                borg_write_map(false_0 != 0);
            }
            borg_log_death();
            borg_log_death_data();
            borg_flush();
            if crate::bridge::st().core.cfg[(BORG_CHEAT_DEATH as ::core::ffi::c_int) as usize] == 0 {
                borg_reincarnate_start();
                (*key).code = 'y' as i32 as keycode_t;
                return true_0 != 0;
            } else {
                do_cmd_wiz_cure_all(::core::ptr::null_mut::<command>());
                crate::bridge::st().borg.time.now = 100 as ::core::ffi::c_int as borg_time;
                (*key).code = 'n' as i32 as keycode_t;
                return true_0 != 0;
            }
        }
        if prefix(buf, b"Atte\0" as *const u8 as *const ::core::ffi::c_char) {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(
                    b"# Confirming use of Spell/Prayer when low on mana.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*key).code = 'y' as i32 as keycode_t;
            return true_0 != 0;
        }
        if prefix(
            buf,
            b"(Equip: c-d,\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            (*key).code = 'c' as i32 as keycode_t;
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(
                    b"# Putting ring on the left hand.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            return true_0 != 0;
        }
        if borg_inkey(false_0 != 0) == 0
            && strncmp(
                buf,
                b"Direction\0" as *const u8 as *const ::core::ffi::c_char,
                9 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            if crate::bridge::st().io.confirm_target {
                if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                    borg_note(
                        b"# Expected request for Direction.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                crate::bridge::st().io.confirm_target = false_0 != 0;
                (*key).code = borg_get_queued_direction();
                return !key.is_null();
            } else {
                borg_note(
                    b"** UNEXPECTED REQUEST FOR DIRECTION Dumping keypress history ***\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                borg_note(format(
                    b"** line starting <%s> ***\0" as *const u8 as *const ::core::ffi::c_char,
                    buf,
                ));
                borg_dump_recent_keys(20 as ::core::ffi::c_int);
                borg_oops(
                    b"unexpected request for direction\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*key).code = ESCAPE as keycode_t;
                return true_0 != 0;
            }
        }
        if prefix(
            buf,
            b"You have no room\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(
                    b"# 'You have no room' is a no-op\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*key).code = 0 as keycode_t;
            return true_0 != 0;
        }
        if borg_inkey(false_0 != 0) == 0
            && prefix(
                buf,
                b"Choose a monster race (by symbol) to banish\0" as *const u8
                    as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(
                    b"# Use of Wonder selected BANISHMENT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*key).code = ESCAPE as keycode_t;
            return !key.is_null();
        }
        if prefix(
            buf,
            b"The lava will\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || prefix(
                buf,
                b"Lava blocks y\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(b"# ignoring Lava warning\0" as *const u8 as *const ::core::ffi::c_char);
            }
            (*key).code = 'y' as i32 as keycode_t;
            return true_0 != 0;
        }
        if borg_inkey(false_0 != 0) == 0
            && prefix(
                buf,
                b"Set recall depth\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(
                    b"# Use of unknown object with recall\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*key).code = 'n' as i32 as keycode_t;
            return true_0 != 0;
        }
        if prefix(
            buf,
            b"Target out of range\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            (*key).code = 'y' as i32 as keycode_t;
            return true_0 != 0;
        }
        if prefix(
            buf,
            b"Word of Recall is already\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            if !crate::bridge::st().borg.trying_unknown {
                borg_oops(
                    b"unexpected double Word of Recall\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*key).code = 'n' as i32 as keycode_t;
            return true_0 != 0;
        }
        if strcmp(
            buf,
            b"(Equipment) \0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*key).code = ESCAPE as keycode_t;
            return true_0 != 0;
        }
        if strcmp(
            buf,
            b"(Inventory) \0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*key).code = ESCAPE as keycode_t;
            return true_0 != 0;
        }
        if strcmp(buf, b"Wear o\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*key).code = ESCAPE as keycode_t;
            return true_0 != 0;
        }
        if strcmp(
            buf,
            b"Lv Mana Fail\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*key).code = ESCAPE as keycode_t;
            return true_0 != 0;
        }
    }
    if prefix(
        buf,
        b"Throw which item? (Throw\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(
                b"# switching to inventory for throws\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*key).code = '/' as i32 as keycode_t;
        return true_0 != 0;
    }
    if cursor.y == 1 as ::core::ffi::c_int
        && borg_inkey(false_0 != 0) == 0
        && prefix(
            buf,
            b"Which effect?\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
            borg_note(
                b"# Use of unknown object with multiple effects\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*key).code = 'a' as i32 as keycode_t;
        return !key.is_null();
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_clear_reactions() {
    borg_msg_len = 0 as int16_t;
    borg_msg_num = 0 as int16_t;
}
