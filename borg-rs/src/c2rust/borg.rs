pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{
    borg_best, borg_struct, borg_time, inkey_hack, C2RustUnnamedH_10, C2RustUnnamedH_11,
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
use crate::headers::borg_init_h::borg_reset_settings;
use crate::headers::borg_io_h::{borg_flush, borg_inkey, borg_note, borg_save_keypress};
use crate::headers::borg_log_h::{
    borg_display_status, borg_log_death, borg_log_death_data, borg_write_map,
};
use crate::headers::borg_messages_h::{borg_get_messages, borg_parse};
use crate::headers::borg_messages_react_h::borg_clear_reactions;
use crate::headers::borg_reincarnate_h::{borg_force_reincarnate, borg_reincarnate_end};
use crate::headers::borg_think_h::{borg_oops, borg_think};
pub use crate::headers::borg_trait_h::{
    borg_notice_player, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW,
    BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM,
    BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL,
    BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID,
    BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL,
    BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE,
    BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED,
    BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS,
    BI_BART, BI_BID, BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX,
    BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY,
    BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING,
    BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP,
    BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN,
    BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO,
    BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP, BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX,
    BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY, BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1,
    BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD, BI_FOOD_CURE_BLIND,
    BI_FOOD_CURE_CONF, BI_FOOD_HI, BI_FOOD_LO, BI_FRACT, BI_GOLD, BI_GOOD_S_CHG, BI_GOOD_W_CHG,
    BI_HASFIXEXP, BI_HEAVYBOW, BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ, BI_IACID, BI_ICOLD, BI_IELEC,
    BI_IFIRE, BI_INFRA, BI_INT, BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID, BI_ISBLIND, BI_ISCONFUSED,
    BI_ISCUT, BI_ISENCUMB, BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX, BI_ISFIXEXP, BI_ISFIXINT,
    BI_ISFIXLEV, BI_ISFIXSTR, BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL, BI_ISGORGED, BI_ISHEAVYSTUN,
    BI_ISHUNGRY, BI_ISIMAGE, BI_ISPARALYZED, BI_ISPOISONED, BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK,
    BI_KING, BI_LIGHT, BI_MAX, BI_MAXCLEVEL, BI_MAXDEPTH, BI_MAXHP, BI_MAXSP, BI_MOD_MOVES,
    BI_MULTIPLE_BONUSES, BI_NEED_BRAND_WEAPON, BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D,
    BI_NEED_ENCHANT_TO_H, BI_PREP_BIG_FIGHT, BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD,
    BI_RCONF, BI_RDARK, BI_RDIS, BI_RECALL, BI_REG, BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS,
    BI_RLITE, BI_RNTHR, BI_RNXUS, BI_RPOIS, BI_RSHRD, BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON,
    BI_SDEX, BI_SDIG, BI_SFRACT, BI_SHLIFE, BI_SHOTS, BI_SINT, BI_SINV, BI_SLING, BI_SPEED,
    BI_SP_ADJ, BI_SRACID, BI_SRBLIND, BI_SRCH, BI_SRCOLD, BI_SRCONF, BI_SRDARK, BI_SRDIS,
    BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS, BI_SRLITE, BI_SRNTHR, BI_SRNXUS, BI_SRPOIS,
    BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR, BI_STR_INDEX, BI_SWIS, BI_THB, BI_THN, BI_THT,
    BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD, BI_WB_ELEC, BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS,
    BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS, BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON,
    BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL, BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL,
    BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD, BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
use crate::headers::cmd_core_h::cmd_get_nrepeats;
pub use crate::headers::game_event_h::{
    game_event_data, game_event_type, C2RustUnnamedH_0, C2RustUnnamedH_1, C2RustUnnamedH_2,
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
use crate::headers::string_h::strstr;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
use crate::headers::ui_input_h::flush;
use crate::headers::ui_term_h::{Term_fresh, Term_inkey, Term_locate};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_quick, Rand_value};
pub use crate::headers::z_type_h::loc;
#[no_mangle]
pub static mut borg: borg_struct = borg_struct {
    player: ::core::ptr::null::<player>() as *mut player,
    trait_0: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    has: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    activation: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    power: 0,
    c: loc { x: 0, y: 0 },
    panels: loc { x: 0, y: 0 },
    avoidance: 0,
    oldchp: 0,
    lunal_mode: false,
    munchkin_mode: false,
    stair_less: false,
    stair_more: false,
    in_shop: false,
    ready_morgoth: 0,
    temp: C2RustUnnamedH_17 {
        need_see_invis: 0,
        see_inv: 0,
        res_fire: false,
        res_cold: false,
        res_acid: false,
        res_elec: false,
        res_pois: false,
        prot_from_evil: false,
        fast: false,
        bless: false,
        hero: false,
        berserk: false,
        fastcast: false,
        regen: false,
        smite_evil: false,
        venom: false,
        shield: false,
    },
    status: C2RustUnnamedH_16 {
        active: false,
        cancel: false,
        save: false,
        old_depth: 0,
        respawning: 0,
        cheat_death: false,
        anti_summon: false,
        digging: false,
        need_alter: false,
        no_alter: false,
        redraw: false,
        vault: false,
        desperate: false,
    },
    time: C2RustUnnamedH_15 {
        now: 0,
        level: 0,
        town: 0,
        morgoth: 0,
        antisummon: 0,
        call_light: 0,
        wizard_light: 0,
        detect_traps: 0,
        detect_doors: 0,
        detect_walls: 0,
        detect_evil: 0,
        detect_obj: 0,
        last_kill_mult: 0,
    },
    need_see_invis: 0,
    see_inv: 0,
    need_shift_panel: false,
    when_shift_panel: 0,
    antibounce_count: 0,
    no_retreat: 0,
    resistance: 0,
    no_rest_prep: 0,
    times_twitch: 0,
    escapes: 0,
    trying_unknown: false,
    dont_react: false,
    targeting: false,
    goal: C2RustUnnamedH_14 {
        type_0: 0,
        g: loc { x: 0, y: 0 },
        rising: false,
        leaving: false,
        fleeing: false,
        fleeing_lunal: false,
        fleeing_munchkin: false,
        fleeing_to_town: false,
        ignoring: false,
        less: false,
        waiting: false,
        recalling: 0,
        descending: 0,
        shop: 0,
        ware: 0,
        item: 0,
        do_best: false,
        best_item: ::core::ptr::null::<borg_best>() as *mut borg_best,
    },
    init_save: C2RustUnnamedH_13 {
        key_mode: 0,
        kinfo_ignore: ::core::ptr::null::<uint8_t>() as *mut uint8_t,
        ignore_level: [0; 27],
        ego_ignore_types: ::core::ptr::null::<*mut bool>() as *mut *mut bool,
    },
    mon: C2RustUnnamedH_12 {
        breeders: 0,
        scary: false,
        morgoth_panel: loc { x: 0, y: 0 },
        unique: 0,
    },
    id: C2RustUnnamedH_11 {
        morgoth: 0,
        sauron: 0,
        tarrasque: 0,
        maggot: 0,
        bullroarer: 0,
        beorn: 0,
        grip: 0,
        fang: 0,
        osse: 0,
        squint_eyed_rogue: 0,
        small_kobold: 0,
        soldier: 0,
        cutpurse: 0,
        acolyte: 0,
        apprentice: 0,
        kobold: 0,
        shrieker_mushroom_patch: 0,
        filthy_street_urchin: 0,
        battle_scarred_veteran: 0,
        mean_looking_mercenary: 0,
        giant_white_mouse: 0,
        white_worm_mass: 0,
        green_worm_mass: 0,
        cave_spider: 0,
        red_naga: 0,
        giant_red_frog: 0,
        radiation_eye: 0,
        yellow_worm_mass: 0,
        gravity_hound: 0,
        inertia_hound: 0,
        dread: 0,
        dracolich: 0,
        dracolisk: 0,
        light_hound: 0,
        dark_hound: 0,
        pukelman: 0,
        night_mare: 0,
        drolem: 0,
    },
    near: C2RustUnnamedH_10 {
        morgoth: false,
        breeder: false,
        unique: 0,
        evil_unique: false,
        summoner: false,
        summoner_idx: 0,
    },
    morgoth_position: false,
    amt_book: [0; 9],
    book_idx: [0; 9],
    need_statgain: [false; 5],
    amt_statgain: [0; 5],
};
#[no_mangle]
pub static mut borg_cfg: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_rand_local: uint32_t = 0 as uint32_t;
#[no_mangle]
pub static mut borg_engine_date: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"Nov  4 2026\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut borg_step: uint16_t = 0 as uint16_t;
#[no_mangle]
pub static mut w_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut w_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_entry_point(mut flush_first: ::core::ffi::c_int) -> keypress {
    return borg_save_keypress(generate_keypress(flush_first));
}
#[no_mangle]
pub unsafe extern "C" fn borg_update_entrypoint(mut start: bool) {
    if start {
        inkey_hack = Some(borg_entry_point as unsafe extern "C" fn(::core::ffi::c_int) -> keypress)
            as Option<unsafe extern "C" fn(::core::ffi::c_int) -> keypress>;
    } else {
        inkey_hack = None;
    };
}
unsafe extern "C" fn check_for_deactivate_or_death(mut key: *mut keypress) -> bool {
    if !borg.status.active {
        borg_note(b"# Removing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
        inkey_hack = None;
        borg_flush();
        flush(
            EVENT_MAP,
            ::core::ptr::null_mut::<game_event_data>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        borg_reset_settings();
        (*key).code = ESCAPE as keycode_t;
        return true_0 != 0;
    }
    if (*player).is_dead as ::core::ffi::c_int != 0 && borg.status.respawning == 0 {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= *borg_cfg.offset(BORG_DUMP_LEVEL as ::core::ffi::c_int as isize)
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
        borg_parse(::core::ptr::null_mut::<::core::ffi::c_char>());
        borg_clear_reactions();
        borg_oops(b"player died\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) != 0 {
        if *borg_cfg.offset(BORG_STOP_KING as ::core::ffi::c_int as isize) != 0 {
            borg_write_map(false_0 != 0);
            borg_oops(b"retire\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if *borg_cfg.offset(BORG_RESPAWN_WINNERS as ::core::ffi::c_int as isize) != 0 {
            borg_write_map(false_0 != 0);
            borg_log_death();
            borg_log_death_data();
            borg_flush();
            borg_force_reincarnate();
            return false_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == *borg_cfg.offset(BORG_STOP_DLEVEL as ::core::ffi::c_int as isize)
    {
        borg_oops(b"Auto-stop for user DLevel.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        == *borg_cfg.offset(BORG_STOP_CLEVEL as ::core::ffi::c_int as isize)
    {
        borg_oops(b"Auto-stop for user CLevel.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        > *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg_cfg.offset(BORG_SELF_SCUM as ::core::ffi::c_int as isize) == 0
    {
        borg_oops(b"Money Scum complete.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn user_abort() -> bool {
    let mut ch_evt: ui_event = ui_event { type_0: EVT_NONE };
    Term_inkey(&raw mut ch_evt, false_0 != 0, true_0 != 0);
    if !borg.in_shop
        && (ch_evt.type_0 as ::core::ffi::c_uint
            & EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && ch_evt.key.code > 0 as keycode_t
            && ch_evt.key.code != 10 as keycode_t
            || ch_evt.type_0 as ::core::ffi::c_uint
                == EVT_DISCONNECT as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if ch_evt.type_0 as ::core::ffi::c_uint
            == EVT_DISCONNECT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            borg_oops(b"terminal disconnect abort\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            if ch_evt.key.code >= 32 as keycode_t && ch_evt.key.code <= 126 as keycode_t {
                borg_note(format(
                    b"# User key press <%lu><%c>\0" as *const u8 as *const ::core::ffi::c_char,
                    ch_evt.key.code as ::core::ffi::c_ulong,
                    ch_evt.key.code as ::core::ffi::c_char as ::core::ffi::c_int,
                ));
            } else {
                borg_note(format(
                    b"# User key press <%lu>\0" as *const u8 as *const ::core::ffi::c_char,
                    ch_evt.key.code as ::core::ffi::c_ulong,
                ));
            }
            borg_note(format(
                b"# Key type was <%d><%c>\0" as *const u8 as *const ::core::ffi::c_char,
                ch_evt.type_0 as ::core::ffi::c_uint,
                ch_evt.type_0 as ::core::ffi::c_uint,
            ));
            borg_oops(b"user abort\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn generate_keypress(mut flush_first: ::core::ffi::c_int) -> keypress {
    let mut borg_ch: keycode_t = 0;
    let mut key: keypress = keypress {
        type_0: EVT_KBRD,
        code: 0 as keycode_t,
        mods: 0 as uint8_t,
    };
    let mut cursor: loc = loc { x: 0, y: 0 };
    let mut rand_quick: bool = false;
    let mut rand_value: uint32_t = 0;
    Term_locate(&raw mut cursor.x, &raw mut cursor.y);
    Term_fresh();
    borg_notice_player();
    if check_for_deactivate_or_death(&raw mut key) {
        return key;
    }
    if flush_first != 0 {
        if borg_inkey(false_0 != 0) != 0 as keycode_t {
            borg_note(b"# Flushing keypress buffer\0" as *const u8 as *const ::core::ffi::c_char);
            borg_flush();
        }
    }
    if borg_get_messages(&raw mut key, cursor) {
        return key;
    }
    borg_parse(::core::ptr::null_mut::<::core::ffi::c_char>());
    borg.dont_react = false_0 != 0;
    borg_ch = borg_inkey(true_0 != 0);
    if borg_ch != 0 {
        key.code = borg_ch;
        return key;
    }
    if user_abort() {
        key.code = ESCAPE as keycode_t;
        return key;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 1 as ::core::ffi::c_int
    {
        borg.in_shop = false_0 != 0;
    }
    if (*(*player).upkeep).resting != 0 || cmd_get_nrepeats() > 0 as ::core::ffi::c_int {
        key.type_0 = EVT_NONE;
        return key;
    }
    if borg.status.respawning != 0 {
        if !(*player).is_dead {
            borg_reincarnate_end();
            borg.status.respawning = false_0 as int16_t;
        } else {
            key.code = ESCAPE as keycode_t;
            return key;
        }
    }
    borg.targeting = false_0 != 0;
    rand_quick = Rand_quick;
    rand_value = Rand_value;
    Rand_quick = true_0 != 0;
    Rand_value = borg_rand_local;
    while !borg_think() {}
    borg_display_status();
    borg_rand_local = Rand_value;
    Rand_quick = rand_quick;
    Rand_value = rand_value;
    if borg_step as ::core::ffi::c_int != 0 && {
        borg_step = borg_step.wrapping_sub(1);
        borg_step == 0
    } {
        borg.status.cancel = true_0 != 0;
    }
    borg_ch = borg_inkey(true_0 != 0);
    if borg_ch != 0 {
        key.code = borg_ch;
        return key;
    }
    borg_oops(b"normal abort\0" as *const u8 as *const ::core::ffi::c_char);
    key.code = ESCAPE as keycode_t;
    return key;
}
