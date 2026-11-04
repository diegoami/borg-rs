pub const PW_BORG_2: ::core::ffi::c_long = 0x8000 as ::core::ffi::c_long;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLOUR_SLATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COLOUR_GREEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLOUR_BLUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};
use crate::headers::borg_flow_glyph_h::borg_needs_new_sea;
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_kills_nxt, borg_race_name};
pub use crate::headers::borg_flow_take_h::{borg_take, borg_takes};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};

pub use crate::headers::borg_item_h::{borg_get_note, borg_item, borg_items};
pub use crate::headers::borg_magic_h::{borg_can_cast, borg_magic, borg_spell_fail_rate, borg_spell_legal, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW, HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL, ZONE_OF_UNMAGIC};
use crate::headers::borg_prepared_h::borg_prepared;

pub use crate::headers::borg_trait_h::{
    prefix_pref, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
use crate::headers::borg_trait_swap_h::{armour_swap, weapon_swap};
pub use crate::headers::cave_h::{
    cave, chunk, connector, f_info, feature, heatmap, monster_group, square, square_monster, trap,
};
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};
use crate::headers::game_input_h::get_string;
use crate::headers::game_world_h::turn;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{
    angband_constants, critical_level, o_critical_level, z_info, ANGBAND_DIR_ARCHIVE,
};
pub use crate::headers::list_object_flags_h::{
    OF_AFRAID, OF_AGGRAVATE, OF_BLESSED, OF_BURNS_OUT, OF_DIG_1, OF_DIG_2, OF_DIG_3, OF_DRAIN_EXP,
    OF_EXPLODE, OF_FEATHER, OF_FRAGILE, OF_FREE_ACT, OF_HOLD_LIFE, OF_IMPACT, OF_IMPAIR_HP,
    OF_IMPAIR_MANA, OF_LIGHT_2, OF_LIGHT_3, OF_MAX, OF_MULTIPLY_WEIGHT, OF_NO_FUEL, OF_NO_TELEPORT,
    OF_PROT_BLIND, OF_PROT_CONF, OF_PROT_FEAR, OF_PROT_STUN, OF_REGEN, OF_SEE_INVIS,
    OF_SLOW_DIGEST, OF_STICKY, OF_SUST_CON, OF_SUST_DEX, OF_SUST_INT, OF_SUST_STR, OF_SUST_WIS,
    OF_TAKES_FUEL, OF_TELEPATHY, OF_THROWING, OF_TRAP_IMMUNE,
};
pub use crate::headers::list_player_timed_h::{
    TMD_AFRAID, TMD_AMNESIA, TMD_ATT_ACID, TMD_ATT_COLD, TMD_ATT_CONF, TMD_ATT_DEMON, TMD_ATT_ELEC,
    TMD_ATT_EVIL, TMD_ATT_FIRE, TMD_ATT_POIS, TMD_ATT_RUN, TMD_ATT_VAMP, TMD_BLACKBREATH,
    TMD_BLESSED, TMD_BLIND, TMD_BLOODLUST, TMD_BOLD, TMD_COMMAND, TMD_CONFUSED, TMD_COVERTRACKS,
    TMD_CUT, TMD_FAST, TMD_FASTCAST, TMD_FOOD, TMD_FREE_ACT, TMD_HEAL, TMD_HERO, TMD_IMAGE,
    TMD_INVULN, TMD_OPP_ACID, TMD_OPP_COLD, TMD_OPP_CONF, TMD_OPP_ELEC, TMD_OPP_FIRE, TMD_OPP_POIS,
    TMD_PARALYZED, TMD_POISONED, TMD_POWERSHOT, TMD_PROTEVIL, TMD_SCRAMBLE, TMD_SHERO, TMD_SHIELD,
    TMD_SINFRA, TMD_SINVIS, TMD_SLOW, TMD_SPRINT, TMD_STEALTH, TMD_STONESKIN, TMD_STUN, TMD_TAUNT,
    TMD_TELEPATHY, TMD_TERROR, TMD_TRAPSAFE,
};
use crate::headers::message_h::{message_str, messages_num, msg};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_desc_h::{
    object_desc, C2RustUnnamedH_42, ODESC_ALTNUM, ODESC_BASE, ODESC_CAPITAL, ODESC_COMBAT,
    ODESC_EXTRA, ODESC_FULL, ODESC_NOEGO, ODESC_PLURAL, ODESC_PREFIX, ODESC_SINGULAR, ODESC_SPOIL,
    ODESC_STORE, ODESC_TERSE,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_31, OF_NONE};
use crate::headers::obj_util_h::object_flags;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};
pub use crate::headers::player_timed_h::{C2RustUnnamedH_22, TMD_MAX};
use crate::headers::player_util_h::player_of_has;
pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::stdlib_h::wcstombs;
pub use crate::headers::store_h::{object_buy, owner, store, store_stock_list, stores};
pub use crate::headers::struct_tm_h::tm;
pub use crate::headers::target_h::target;
use crate::headers::time_h::{localtime, strftime, time};
pub use crate::headers::time_t_h::time_t;
pub use crate::headers::types_h::{
    __int16_t, __int32_t, __time_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
use crate::headers::ui_menu_h::all_letters_nohjkl;
use crate::headers::ui_output_h::prt;
pub use crate::headers::ui_term_h::{
    angband_term, term, term_win, window_flag, Term, Term_activate, Term_clear, Term_fresh,
    Term_putch, Term_putstr,
};
use crate::headers::wchar_h::wcslen;
pub use crate::headers::z_bitflag_h::bitflag;

pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_file_h::{
    ang_file, file_close, file_mode, file_open, file_put, file_putf, file_type, path_build,
    FTYPE_HTML, FTYPE_RAW, FTYPE_SAVE, FTYPE_TEXT, MODE_APPEND, MODE_READ, MODE_WRITE,
};
use crate::headers::z_form_h::{format, quit_fmt};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, prefix};
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub unsafe extern "C" fn borg_log_death() {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut borg_log_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut death_time: time_t = 0;
    if crate::bridge::st().core.cfg[(BORG_SAVE_DEATH as ::core::ffi::c_int) as usize] == 0 {
        return;
    }
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        b"borg-log.txt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg_log_file = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_APPEND,
        FTYPE_TEXT,
    );
    if borg_log_file.is_null() {
        return;
    }
    time(&raw mut death_time);
    strftime(
        &raw mut buf as *mut ::core::ffi::c_char,
        80 as size_t,
        b"%Y/%m/%d %H:%M\n\0" as *const u8 as *const ::core::ffi::c_char,
        localtime(&raw mut death_time),
    );
    file_put(borg_log_file, &raw mut buf as *mut ::core::ffi::c_char);
    file_putf(
        borg_log_file,
        b"%s the %s %s, Level %d/%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*player).full_name as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).max_lev as ::core::ffi::c_int,
    );
    file_putf(
        borg_log_file,
        b"Exp: %lu  Gold: %lu  Turn: %lu\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).max_exp as ::core::ffi::c_long
            + (100 as ::core::ffi::c_int * (*player).max_depth as ::core::ffi::c_int)
                as ::core::ffi::c_long,
        (*player).au as ::core::ffi::c_long,
        turn as ::core::ffi::c_long,
    );
    file_putf(
        borg_log_file,
        b"Killed on level: %d (max. %d) by %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).depth as ::core::ffi::c_int,
        (*player).max_depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_putf(
        borg_log_file,
        b"Borg Compile Date: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::ENGINE_DATE.as_ptr() as *mut ::core::ffi::c_char,
    );
    file_put(
        borg_log_file,
        b"----------\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_close(borg_log_file);
}
#[no_mangle]
pub unsafe extern "C" fn borg_log_death_data() {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut borg_log_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut death_time: time_t = 0;
    if crate::bridge::st().core.cfg[(BORG_SAVE_DEATH as ::core::ffi::c_int) as usize] == 0 {
        return;
    }
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        b"borg.dat\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg_log_file = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_APPEND,
        FTYPE_TEXT,
    );
    if borg_log_file.is_null() {
        return;
    }
    time(&raw mut death_time);
    file_putf(
        borg_log_file,
        b"%s, %s, %s, %d, %d, %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::ENGINE_DATE.as_ptr() as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_close(borg_log_file);
}
unsafe extern "C" fn borg_index_to_label(mut i: ::core::ffi::c_int) -> ::core::ffi::c_char {
    if i < (*z_info).pack_size as ::core::ffi::c_int {
        return *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize);
    }
    return *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
        .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn borg_write_map(mut ask: bool) {
    let mut buf2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    let mut borg_map_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut line: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut ch_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut itemm: ::core::ffi::c_int = 0;
    let mut m_idx: int16_t = 0;
    let mut o_name: [::core::ffi::c_char; 80] = [0; 80];
    if crate::bridge::st().core.cfg[(BORG_SAVE_DEATH as ::core::ffi::c_int) as usize] == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while (*player).full_name[i as usize] != 0 {
        let mut c: ::core::ffi::c_char = (*player).full_name[i as usize];
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
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
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
            && *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
        {
            c = '_' as i32 as ::core::ffi::c_char;
        }
        buf[i as usize] = c;
        i += 1;
    }
    let fresh0 = i;
    i = i + 1;
    buf[fresh0 as usize] = '.' as i32 as ::core::ffi::c_char;
    let fresh1 = i;
    i = i + 1;
    buf[fresh1 as usize] = 'm' as i32 as ::core::ffi::c_char;
    let fresh2 = i;
    i = i + 1;
    buf[fresh2 as usize] = 'a' as i32 as ::core::ffi::c_char;
    let fresh3 = i;
    i = i + 1;
    buf[fresh3 as usize] = 'p' as i32 as ::core::ffi::c_char;
    let fresh4 = i;
    i = i + 1;
    buf[fresh4 as usize] = '\0' as i32 as ::core::ffi::c_char;
    path_build(
        &raw mut buf2 as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    if ask as ::core::ffi::c_int != 0
        && get_string(
            b"Borg map File: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf2 as *mut ::core::ffi::c_char,
            70 as size_t,
        ) as ::core::ffi::c_int
            != 0
    {
        borg_map_file = file_open(
            &raw mut buf2 as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
        if borg_map_file.is_null() {
            msg(b"Cannot open that file.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    } else if !ask {
        borg_map_file = file_open(
            &raw mut buf2 as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
    }
    file_putf(
        borg_map_file,
        b"%s the %s %s, Level %d/%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*player).full_name as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).max_lev as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"Exp: %lu  Gold: %lu  Turn: %lu\n\0" as *const u8 as *const ::core::ffi::c_char,
        ((*player).max_exp + 100 as int32_t * (*player).max_depth as int32_t)
            as ::core::ffi::c_long,
        (*player).au as ::core::ffi::c_long,
        turn as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Killed on level: %d (max. %d) by %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).depth as ::core::ffi::c_int,
        (*player).max_depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Borg Compile Date: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::ENGINE_DATE.as_ptr() as *mut ::core::ffi::c_char,
    );
    line = mem_zalloc(
        ((DUNGEON_WID + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<wchar_t>() as size_t),
    ) as *mut wchar_t;
    ch_line = mem_zalloc(
        ((DUNGEON_WID + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < DUNGEON_HGT {
        j = 0 as ::core::ffi::c_int;
        while j < DUNGEON_WID {
            let mut ch: wchar_t = 0;
            let mut ag: *mut borg_grid = crate::bridge::grid(j, i) as *mut borg_grid;
            if !square_monster(cave, loc(j, i)).is_null() {
                m_idx = (*square_monster(cave, loc(j, i))).midx as int16_t;
            } else {
                m_idx = 0 as int16_t;
            }
            ch = ' ' as i32 as wchar_t;
            if (*ag).feat != 0 {
                ch = (*f_info.offset((*ag).feat as isize)).d_char;
            }
            if (*ag).take != 0 {
                let mut take: *mut borg_take =
                    borg_takes.offset((*ag).take as isize) as *mut borg_take;
                if (*take).kidx != 0 {
                    let mut k_ptr: *mut object_kind =
                        k_info.offset((*take).kidx as isize) as *mut object_kind;
                    ch = (*k_ptr).d_char;
                }
            }
            if m_idx != 0 {
                ch = '&' as i32 as wchar_t;
            }
            if (*ag).kill != 0 {
                let mut kill: *mut borg_kill =
                    borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                let mut r_ptr: *mut monster_race =
                    r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                ch = (*r_ptr).d_char;
            }
            if i == crate::bridge::st().borg.c.y && j == crate::bridge::st().borg.c.x {
                ch = '@' as i32 as wchar_t;
            }
            *line.offset(j as isize) = ch;
            j += 1;
        }
        let fresh5 = j;
        j = j + 1;
        *line.offset(fresh5 as isize) = '\0' as i32 as wchar_t;
        wcstombs(
            ch_line,
            line,
            (wcslen(line) as size_t).wrapping_add(1 as size_t),
        );
        file_putf(
            borg_map_file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            ch_line,
        );
        i += 1;
    }
    mem_free(line as *mut ::core::ffi::c_void);
    line = ::core::ptr::null_mut::<wchar_t>();
    mem_free(ch_line as *mut ::core::ffi::c_void);
    ch_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_0).r_idx == 0) {
            file_putf(
                borg_map_file,
                b"monster '%s' (%d) at (%d,%d) speed:%d \n\0" as *const u8
                    as *const ::core::ffi::c_char,
                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                (*kill_0).r_idx as ::core::ffi::c_int,
                (*kill_0).pos.y,
                (*kill_0).pos.x,
                (*kill_0).speed as ::core::ffi::c_int,
            );
        }
        i += 1;
    }
    i = messages_num() as ::core::ffi::c_int;
    if i > 250 as ::core::ffi::c_int {
        i = 250 as ::core::ffi::c_int;
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Last Messages]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    loop {
        let fresh6 = i;
        i = i - 1;
        if !(fresh6 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut msg_0: *const ::core::ffi::c_char = message_str(i as int16_t as uint16_t);
        if prefix(
            msg_0,
            b"# Matched\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || prefix(
                msg_0,
                b"# There is\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# Tracking\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(msg_0, b"> \0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            continue;
        }
        file_putf(
            borg_map_file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            msg_0,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Character Equipment]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*player).body.count as ::core::ffi::c_int {
        let mut obj: *mut object = (*(*player).body.slots.offset(i as isize)).obj;
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            obj,
            ODESC_FULL as ::core::ffi::c_int as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            borg_index_to_label(i) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Character Quiver]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).quiver_size as ::core::ffi::c_int {
        let mut obj_0: *mut object = *(*(*player).upkeep).quiver.offset(i as isize);
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            obj_0,
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            borg_index_to_label(i) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Character Inventory]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item).iqty != 0 {
            if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                file_putf(
                    borg_map_file,
                    b"%c) %d %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_index_to_label(i) as ::core::ffi::c_int,
                    (*item).iqty as ::core::ffi::c_int,
                    &raw mut (*item).desc as *mut ::core::ffi::c_char,
                );
            } else {
                file_putf(
                    borg_map_file,
                    b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_index_to_label(i) as ::core::ffi::c_int,
                    &raw mut (*item).desc as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Home Inventory (page 1)]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut list: *mut *mut object = mem_zalloc(
        (::core::mem::size_of::<*mut object>() as size_t)
            .wrapping_mul((*z_info).store_inven_max as size_t),
    ) as *mut *mut object;
    store_stock_list(
        stores.offset(BORG_HOME as isize) as *mut store,
        list,
        (*z_info).store_inven_max as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            *list.offset(i as isize),
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i % 12 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Home Inventory (page 2)]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = (*z_info).store_inven_max as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            *list.offset(i as isize),
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i % 12 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    mem_free(list as *mut ::core::ffi::c_void);
    list = ::core::ptr::null_mut::<*mut object>();
    if crate::bridge::st().core.cfg[(BORG_USES_SWAPS as ::core::ffi::c_int) as usize] != 0 {
        file_putf(
            borg_map_file,
            b"  [Swap info]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if weapon_swap != 0 {
            item = borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize)
                as *mut borg_item;
            file_putf(
                borg_map_file,
                b"Swap Weapon:  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*item).desc as *mut ::core::ffi::c_char,
            );
        } else {
            file_put(
                borg_map_file,
                b"Swap Weapon:  NONE\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if armour_swap != 0 {
            item = borg_items.offset((armour_swap - 1 as ::core::ffi::c_int) as isize)
                as *mut borg_item;
            file_putf(
                borg_map_file,
                b"Swap Armour:  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*item).desc as *mut ::core::ffi::c_char,
            );
        } else {
            file_put(
                borg_map_file,
                b"Swap Armour:  NONE\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        file_putf(
            borg_map_file,
            b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"  [Player State at Death] \n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Current speed: %d. \n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize],
    );
    if *(*player)
        .timed
        .offset(TMD_BLIND as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You cannot see.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are confused.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_AFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are terrified.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CUT as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are bleeding.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are stunned.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_POISONED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are poisoned.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_IMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are hallucinating.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if player_of_has(player, OF_AGGRAVATE as ::core::ffi::c_int) {
        file_putf(
            borg_map_file,
            b"You aggravate monsters.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_BLESSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You feel righteous.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_HERO as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You feel heroic.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_SHERO as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are in a battle rage.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are protected from evil.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_SHIELD as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are protected by a mystic shield.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_INVULN as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are temporarily invulnerable.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"Your hands are glowing dull red.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*player).word_recall != 0 {
        file_putf(
            borg_map_file,
            b"You will soon be recalled.  (%d turns)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*player).word_recall as ::core::ffi::c_int,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_FIRE as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist fire exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist acid exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_ELEC as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist elec exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_COLD as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist cold exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_POIS as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist poison exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Anti-bounce counter: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"Time on this level: %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
        (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level) as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Time since left town: %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
        (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town) as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Borg_no_retreat: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.no_retreat as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"borg.near.breeder: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.near.breeder as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"borg.mon.unique: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.mon.unique,
    );
    if turn as ::core::ffi::c_long
        % (10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long)
        < 10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long
            / 2 as ::core::ffi::c_long
    {
        file_putf(
            borg_map_file,
            b"It is daytime in town.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        file_putf(
            borg_map_file,
            b"It is night-time in town.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"borg_uses_swaps: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_USES_SWAPS as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_damage: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_speed: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_SPEED as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_hp: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_HP as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_mana: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_MANA as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_ac: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_AC as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_worships_gold: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"borg_plays_risky: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"prepping for big fight: %d\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize],
    );
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if borg_can_cast() {
        file_putf(
            borg_map_file,
            b"\n\n   [ Spells ] \n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        file_putf(
            borg_map_file,
            b"Name                           Legal Times cast\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*(*player).class).magic.total_spells {
            let mut as_0: *mut borg_magic = &raw mut crate::bridge::st().magic.magics[(i) as usize] as *mut borg_magic;
            let mut failpercent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if ((*as_0).level as ::core::ffi::c_int) < 99 as ::core::ffi::c_int {
                let mut legal: *const ::core::ffi::c_char =
                    if borg_spell_legal((*as_0).spell_enum) as ::core::ffi::c_int != 0 {
                        b"Yes\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"No \0" as *const u8 as *const ::core::ffi::c_char
                    };
                failpercent = borg_spell_fail_rate((*as_0).spell_enum);
                file_putf(
                    borg_map_file,
                    b"%-30s   %s   %ld   fail:%d \n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*as_0).name.as_ptr(),
                    legal,
                    (*as_0).times as ::core::ffi::c_long,
                    failpercent,
                );
            }
            i += 1;
        }
        file_putf(
            borg_map_file,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    itemm = 0 as ::core::ffi::c_int;
    to = BI_MAX as ::core::ffi::c_int;
    while itemm < to {
        file_putf(
            borg_map_file,
            b"skill %d (%s) value= %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
            itemm,
            *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char).offset(itemm as isize),
            crate::bridge::st().borg.trait_[(itemm) as usize],
        );
        itemm += 1;
    }
    file_close(borg_map_file);
}
unsafe extern "C" fn borg_prt_binary(
    mut flags: uint32_t,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut bitmask: uint32_t = 0;
    bitmask = 1 as uint32_t;
    i = bitmask as ::core::ffi::c_int;
    while i <= 32 as ::core::ffi::c_int {
        if flags & bitmask != 0 {
            let fresh7 = col;
            col = col + 1;
            Term_putch(fresh7, row, COLOUR_BLUE, '*' as i32);
        } else {
            let fresh8 = col;
            col = col + 1;
            Term_putch(fresh8, row, COLOUR_WHITE, '-' as i32);
        }
        i += 1;
        bitmask = bitmask.wrapping_mul(2 as uint32_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_display_item(mut item2: *mut object, mut n: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: [bitflag; 5] = [0; 5];
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    item = borg_items.offset(n as isize) as *mut borg_item;
    object_flags(item2, &raw mut f as *mut bitflag);
    Term_clear();
    prt(
        &raw mut (*item).desc as *mut ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"kind = %-5lu  level = %-4d  tval = %-5d  sval = %-5d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).kind as ::core::ffi::c_ulong,
            (*item).level as ::core::ffi::c_int,
            (*item).tval as ::core::ffi::c_int,
            (*item).sval as ::core::ffi::c_int,
        ),
        4 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"number = %-3d  wgt = %-6d  ac = %-5d    damage = %dd%d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).iqty as ::core::ffi::c_int,
            (*item).weight as ::core::ffi::c_int,
            (*item).ac as ::core::ffi::c_int,
            (*item).dd as ::core::ffi::c_int,
            (*item).ds as ::core::ffi::c_int,
        ),
        5 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"pval = %-5d  toac = %-5d  tohit = %-4d  todam = %-4d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).pval as ::core::ffi::c_int,
            (*item).to_a as ::core::ffi::c_int,
            (*item).to_h as ::core::ffi::c_int,
            (*item).to_d as ::core::ffi::c_int,
        ),
        6 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"name1 = %-4d  name2 = %-4d  value = %ld   cursed = %d   can uncurse = %d\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*item).art_idx as ::core::ffi::c_int,
            (*item).ego_idx as ::core::ffi::c_int,
            (*item).value as ::core::ffi::c_long,
            (*item).cursed as ::core::ffi::c_int,
            (*item).uncursable as ::core::ffi::c_int,
        ),
        7 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"ident = %d      timeout = %-d\0" as *const u8 as *const ::core::ffi::c_char,
            (*item).ident as ::core::ffi::c_int,
            (*item).timeout as ::core::ffi::c_int,
        ),
        8 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"Inscription: %s, chance: %d\0" as *const u8 as *const ::core::ffi::c_char,
            borg_get_note(item),
            crate::bridge::st().borg.trait_[(BI_DEV as ::core::ffi::c_int) as usize]
                - (*item).level as ::core::ffi::c_int,
        ),
        9 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"+------------FLAGS1------------+\0" as *const u8 as *const ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"AFFECT..........SLAY.......BRAND\0" as *const u8 as *const ::core::ffi::c_char,
        11 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"                ae      xxxpaefc\0" as *const u8 as *const ::core::ffi::c_char,
        12 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"siwdcc  ssidsasmnvudotgddduoclio\0" as *const u8 as *const ::core::ffi::c_char,
        13 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"tnieoh  trnipthgiinmrrnrrmniierl\0" as *const u8 as *const ::core::ffi::c_char,
        14 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"rtsxna..lcfgdkttmldncltggndsdced\0" as *const u8 as *const ::core::ffi::c_char,
        15 as ::core::ffi::c_int,
        j,
    );
    if (*item).ident {
        borg_prt_binary(
            f[0 as ::core::ffi::c_int as usize] as uint32_t,
            16 as ::core::ffi::c_int,
            j,
        );
    }
    prt(
        b"+------------FLAGS2------------+\0" as *const u8 as *const ::core::ffi::c_char,
        17 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"SUST........IMM.RESIST.........\0" as *const u8 as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"            afecaefcpfldbc s n  \0" as *const u8 as *const ::core::ffi::c_char,
        19 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"siwdcc      cilocliooeialoshnecd\0" as *const u8 as *const ::core::ffi::c_char,
        20 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"tnieoh      irelierliatrnnnrethi\0" as *const u8 as *const ::core::ffi::c_char,
        21 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"rtsxna......decddcedsrekdfddxhss\0" as *const u8 as *const ::core::ffi::c_char,
        22 as ::core::ffi::c_int,
        j,
    );
    if (*item).ident {
        borg_prt_binary(
            f[1 as ::core::ffi::c_int as usize] as uint32_t,
            23 as ::core::ffi::c_int,
            j,
        );
    }
    prt(
        b"+------------FLAGS3------------+\0" as *const u8 as *const ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"s   ts h     tadiiii   aiehs  hp\0" as *const u8 as *const ::core::ffi::c_char,
        11 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"lf  eefo     egrgggg  bcnaih  vr\0" as *const u8 as *const ::core::ffi::c_char,
        12 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"we  lerln   ilgannnn  ltssdo  ym\0" as *const u8 as *const ::core::ffi::c_char,
        13 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"da reiedo   merirrrr  eityew ccc\0" as *const u8 as *const ::core::ffi::c_char,
        14 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"itlepnelf   ppanaefc  svaktm uuu\0" as *const u8 as *const ::core::ffi::c_char,
        15 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"ghigavaiu   aoveclio  saanyo rrr\0" as *const u8 as *const ::core::ffi::c_char,
        16 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"seteticfe   craxierl  etropd sss\0" as *const u8 as *const ::core::ffi::c_char,
        17 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"trenhstel   tttpdced  detwes eee\0" as *const u8 as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    if (*item).ident {
        borg_prt_binary(
            f[2 as ::core::ffi::c_int as usize] as uint32_t,
            19 as ::core::ffi::c_int,
            j + 32 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_display_status() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        let mut old: *mut term = Term;
        if !angband_term[j as usize].is_null() {
            if window_flag[j as usize] as ::core::ffi::c_long & 0x8000 as ::core::ffi::c_long != 0 {
                let mut attr: uint8_t = 0;
                Term_activate(angband_term[j as usize]);
                Term_putstr(
                    5 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"RESISTS\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if crate::bridge::st().borg.trait_[(BI_RACID as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.temp.res_acid {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_IACID as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Acid\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if crate::bridge::st().borg.trait_[(BI_RELEC as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.temp.res_elec {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_IELEC as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Elec\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if crate::bridge::st().borg.trait_[(BI_RFIRE as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.temp.res_fire {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fire\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if crate::bridge::st().borg.trait_[(BI_RCOLD as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.temp.res_cold {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_ICOLD as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cold\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if crate::bridge::st().borg.trait_[(BI_RPOIS as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.temp.res_pois {
                    attr = COLOUR_GREEN as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Pois\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RFEAR as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fear\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Lite\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Dark\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Blind\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Confu\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RSND as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Sound\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RSHRD as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Shard\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RNXUS as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Nexus\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RNTHR as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Nethr\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RKAOS as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Chaos\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_RDIS as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Disen\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SDIG as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"S.Dig\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_FEATH as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Feath\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"PLite\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Regen\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Telep\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SINV as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Invis\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"FrAct\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_HLIFE as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"HLife\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    5 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Weapon Slays:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_ANIMAL as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Animal\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_EVIL as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Evil\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_UNDEAD as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Undead\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_DEMON as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_WK_DEMON as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_GREEN as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Demon\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_ORC as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Orc\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_TROLL as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Troll\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_GIANT as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Giant\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WS_DRAGON as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if crate::bridge::st().borg.trait_[(BI_WK_DRAGON as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_GREEN as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Dragon\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WB_ACID as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Acid\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WB_COLD as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cold\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WB_ELEC as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Elec\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_WB_FIRE as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fire\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Concerns:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cursed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Weak\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Poison\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cut\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Stun\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Confused\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.goal.fleeing {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Goal Fleeing\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.no_rest_prep as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"No Resting\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    60 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Time:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"This Level         \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level) as ::core::ffi::c_long,
                    ),
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"Since Town         \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town) as ::core::ffi::c_long,
                    ),
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"Anti-bounce count  \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                        crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int,
                    ),
                );
                Term_putstr(
                    19 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Sustains\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SSTR as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"STR\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SINT as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"INT\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SWIS as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"WIS\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SDEX as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"DEX\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.trait_[(BI_SCON as ::core::ffi::c_int) as usize] != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"CON\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Temp Effects\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.prot_from_evil {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Prot. Evil\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.fastcast {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fastcast\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.hero {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.berserk {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Berserk\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.shield {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Shielded\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.bless {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Blessed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.temp.fast {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fast\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"See Inv\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Level Information\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.status.vault {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Vault on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.mon.unique != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Unique on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.mon.unique != 0 {
                    Term_putstr(
                        58 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"(%s)\0" as *const u8 as *const ::core::ffi::c_char,
                            (*r_info.offset(crate::bridge::st().borg.mon.unique as isize)).name,
                        ),
                    );
                } else {
                    Term_putstr(
                        58 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                                                \0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if crate::bridge::st().borg.mon.scary {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Scary Guy on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.near.breeder {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Breeder level (closing doors)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().borg.near.summoner_idx as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Summoner very close (AS-Corridor)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Reason for not diving:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_WHITE as uint8_t;
                Term_putstr(
                    64 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    format(
                        b"%s                              \0" as *const u8
                            as *const ::core::ffi::c_char,
                        borg_prepared(
                            crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                + 1 as ::core::ffi::c_int,
                        ),
                    ),
                );
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Scumming: not active                          \0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
                    != 0 as ::core::ffi::c_int
                {
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        42 as ::core::ffi::c_int,
                        7 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"Scumming: $%d                  \0" as *const u8
                                as *const ::core::ffi::c_char,
                            crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize],
                        ),
                    );
                }
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Maximal Depth:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_WHITE as uint8_t;
                Term_putstr(
                    56 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    format(
                        b"%d    \0" as *const u8 as *const ::core::ffi::c_char,
                        crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize],
                    ),
                );
                if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                    >= 50 as ::core::ffi::c_int
                {
                    Term_putstr(
                        5 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"Important Deep Events:\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home *Heal*:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        13 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_ezheal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home Heal:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_heal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home Life:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_life as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Res_Mana:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_mana as ::core::ffi::c_int,
                        ),
                    );
                    if crate::bridge::st().borg.near.morgoth {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        20 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"Morgoth on Level.  Last seen:%ld       \0" as *const u8
                                as *const ::core::ffi::c_char,
                            (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth) as ::core::ffi::c_long,
                        ),
                    );
                    if crate::bridge::st().borg.morgoth_position {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    if borg_needs_new_sea {
                        attr = COLOUR_WHITE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        21 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Sea of Runes.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if crate::bridge::st().borg.ready_morgoth != 0 {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        22 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Ready for Morgoth.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    Term_putstr(
                        5 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"                        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        10 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_ezheal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        10 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_life as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_heal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                   \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::bridge::st().home.num_mana as ::core::ffi::c_int,
                        ),
                    );
                }
                Term_fresh();
                Term_activate(old);
            }
        }
        j += 1;
    }
}
