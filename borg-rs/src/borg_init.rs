//! Port of `borg-init.c`: full initialization of all borg variables.
//!
//! Copyright (c) 1997 Ben Harrison, James E. Wilson, Robert A. Koeneke
//! Copyright (c) 2007-9 Andi Sidwell, Chris Carr, Ed Graham, Erik Osheim
//!
//! This work is free software; you can redistribute it and/or modify it
//! under the terms of either:
//!
//! a) the GNU General Public License as published by the Free Software
//!    Foundation, version 2, or
//!
//! b) the "Angband License":
//!    This software may be copied and distributed for educational, research,
//!    and not for profit purposes provided that this copyright and statement
//!    are included in all such copies.  Other copyrights may also apply.

use std::ffi::CStr;

use crate::borg_io::{borg_note, borg_warning};
use crate::cctype::{c_isdigit, c_toupper};
use crate::borg_headers::borg_h::{
    BORG_ALLOW_STRANGE_OPTS, BORG_CHEAT_DEATH, BORG_ENCHANT_LIMIT, BORG_LUNAL_MODE,
    BORG_MAX_SETTINGS, BORG_MUNCHKIN_BY_CLASS, BORG_MUNCHKIN_CLASS_DEPTH, BORG_MUNCHKIN_DEPTH,
    BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_RESPAWN_CLASS,
    BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_STOP_KING,
    BORG_USES_DYNAMIC_CALCS,
};
use crate::headers::game_event_h::{
    game_event_data, game_event_type, EVENT_LEAVE_GAME, EVENT_MESSAGE_FLUSH,
};
use crate::headers::list_options_h::{
    OPT_birth_connect_stairs, OPT_birth_force_descend, OPT_birth_no_recall,
    OPT_birth_percent_damage, OPT_cheat_live, OPT_rogue_like_commands,
};
use crate::headers::ui_keymap_h::{KEYMAP_MODE_ORIG, KEYMAP_MODE_ROGUE};
use crate::state::BorgState;
use crate::engine;

/// `MAX_RACES` (borg-trait.h).
const MAX_RACES: i32 = 11;
/// `MAX_CLASSES` (borg-trait.h).
const MAX_CLASSES: i32 = 9;
/// `PR_MAP` (player-calcs.h).
const PR_MAP: u32 = 0x10000;
/// The size of the line buffer of `borg_init_txt_file()` (`char buf[1024]`).
const TXT_BUF: usize = 1024;

/// borg-init.c's globals.
#[derive(Default)]
pub struct InitState {
    /// `borg_initialized`
    pub initialized: bool,
    /// `game_closed`: has the game been closed since the borg was
    /// initialized
    pub game_closed: bool,
    /// `borg_init_failure`
    pub init_failure: bool,
    /// `borg_munchkin_start_setting`: the munchkin start as borg.txt gave
    /// it, before the class rule
    pub munchkin_start_setting: i32,
    /// `borg_munchkin_depth_setting`
    pub munchkin_depth_setting: i32,
}

/// One borg.txt setting (`struct borg_setting`).
pub struct BorgSetting {
    /// `setting_string`
    pub name: &'static CStr,
    /// `setting_type`: `b'b'` (bool) or `b'i'` (int)
    pub kind: u8,
    /// `default_value`
    pub default_value: i32,
}

const fn b(name: &'static CStr, default_value: bool) -> BorgSetting {
    BorgSetting {
        name,
        kind: b'b',
        default_value: default_value as i32,
    }
}

const fn i(name: &'static CStr, default_value: i32) -> BorgSetting {
    BorgSetting {
        name,
        kind: b'i',
        default_value,
    }
}

/// Borg settings information (`borg_settings`, without the `{0, 0, 0}`
/// entry that ends the C table). Must match the `BORG_*` enum of borg.h.
pub const BORG_SETTINGS: [BorgSetting; BORG_MAX_SETTINGS as usize] = [
    b(c"borg_verbose", false),
    b(c"borg_munchkin_start", false),
    i(c"borg_munchkin_level", 12),
    i(c"borg_munchkin_depth", 16),
    b(c"borg_worships_damage", false),
    b(c"borg_worships_speed", false),
    b(c"borg_worships_hp", false),
    b(c"borg_worships_mana", false),
    b(c"borg_worships_ac", false),
    b(c"borg_worships_gold", false),
    b(c"borg_plays_risky", false),
    b(c"borg_kills_uniques", false),
    b(c"borg_uses_swaps", true),
    b(c"borg_uses_dynamic_calcs", false),
    i(c"borg_stop_dlevel", 128),
    i(c"borg_stop_clevel", 51),
    i(c"borg_no_deeper", 127),
    b(c"borg_stop_king", true),
    b(c"borg_cheat_death", false),
    b(c"borg_respawn_winners", false),
    i(c"borg_respawn_class", -1),
    i(c"borg_respawn_race", -1),
    i(c"borg_chest_fail_tolerance", 7),
    i(c"borg_delay_factor", 0),
    i(c"borg_money_scum_amount", 0),
    b(c"borg_self_scum", true),
    b(c"borg_lunal_mode", false),
    b(c"borg_self_lunal", false),
    i(c"borg_enchant_limit", 12),
    i(c"borg_dump_level", 1),
    i(c"borg_save_death", 1),
    b(c"borg_stop_on_bell", false),
    b(c"borg_allow_strange_opts", false),
    b(c"borg_autosave", false),
    b(c"borg_restore_ignore_settings", false),
    b(c"borg_munchkin_by_class", false),
    i(c"borg_munchkin_class_depth", 6),
    i(c"borg_buy_strategy_default", 0),
    i(c"borg_buy_strategy_munchkin", -1),
    i(c"borg_buy_strategy_current_classes", 0),
    i(c"borg_buy_strategy_reserve410_classes", 0),
    i(c"borg_buy_strategy_current_munchkin_classes", 0),
    i(c"borg_buy_strategy_reserve410_munchkin_classes", 0),
    i(c"borg_buy_identify", 0),
    b(c"borg_town_no_stores", false),
    i(c"borg_town_plan", 0),
    i(c"borg_town_w_light", 100),
    i(c"borg_town_w_food", 100),
    i(c"borg_town_w_bleeding", 100),
    i(c"borg_town_w_poison", 100),
    i(c"borg_town_w_escape", 50),
    i(c"borg_town_w_knowledge", 25),
    i(c"borg_town_w_power", 50),
    i(c"borg_town_w_trip", 50),
    i(c"borg_town_share", 10),
    i(c"borg_town_light_depth", 15),
    i(c"borg_town_food_trip", 10),
];

/// Class indexes of borg-trait.h used by the munchkin class rule.
const CLASS_WARRIOR: u32 = 0;
const CLASS_DRUID: u32 = 2;
const CLASS_RANGER: u32 = 7;

/// `borg_munchkin_class_settings()`: Warriors, Druids and Rangers survive
/// their first levels more often with a munchkin start. When borg.txt
/// leaves the munchkin start off and `borg_munchkin_by_class` is on, these
/// classes use it anyway, stair-scumming at `borg_munchkin_class_depth`.
/// Every other class plays as borg.txt says. This runs again after a
/// reincarnation, since the class may change. `cidx` is
/// `player->class->cidx` (`None` while there is no player class).
/// (better-borg-c milestone8, owner change PR302.)
pub fn borg_munchkin_class_settings(st: &mut BorgState, cidx: Option<u32>) {
    let Some(cidx) = cidx else {
        return;
    };
    if st.core.cfg.is_empty() {
        return;
    }

    let (start, depth) = (st.init.munchkin_start_setting, st.init.munchkin_depth_setting);
    let cfg = &mut st.core.cfg;
    cfg[BORG_MUNCHKIN_START as usize] = start;
    cfg[BORG_MUNCHKIN_DEPTH as usize] = depth;

    if start != 0 || cfg[BORG_MUNCHKIN_BY_CLASS as usize] == 0 {
        return;
    }

    if cidx == CLASS_WARRIOR || cidx == CLASS_DRUID || cidx == CLASS_RANGER {
        cfg[BORG_MUNCHKIN_START as usize] = 1;
        cfg[BORG_MUNCHKIN_DEPTH as usize] = cfg[BORG_MUNCHKIN_CLASS_DEPTH as usize];
    }
}

/// `prefix_i(s, t)` (z-util.c): does `s` start with `t`, ignoring case?
/// `s` is a C string without its NUL (past its end C reads the NUL, which
/// matches no byte of `t`).
fn prefix_i(s: &[u8], t: &[u8]) -> bool {
    t.iter()
        .enumerate()
        .all(|(k, &c)| c_toupper(c) == c_toupper(s.get(k).copied().unwrap_or(0)))
}

/// `atoi(s)` (glibc: `(int) strtol(s, NULL, 10)`).
fn atoi(s: &[u8]) -> i32 {
    crate::borg_formulas::c_strtol(s).0 as i32
}

/// Read a line from the config file and parse the setting value, if there
/// is one (`borg_proc_setting`). `line` is the C string without its NUL.
/// Returns whether the line names the setting.
///
/// For an int setting, a byte after the name that is not ` `, `=`, `-` or
/// a digit ends the parse and keeps the default, and a `-` only negates a
/// number that follows it (milestone6 patches 0057/0058; before them the C
/// loop, and this one, spun forever there; c-bugs.md section 38).
fn borg_proc_setting(
    cfg: &mut [i32],
    setting: usize,
    setting_string: &[u8],
    kind: u8,
    line: &[u8],
) -> bool {
    let at = |p: usize| line.get(p).copied().unwrap_or(0);
    let mut negative = false;
    let mut p = 0;

    // skip leading space
    while at(p) == b' ' {
        p += 1;
    }
    if !prefix_i(&line[p..], setting_string) {
        return false;
    }

    p += setting_string.len();

    // accept either
    // value true or
    // value=true or
    // value=1 or
    // value y
    while at(p) != 0 {
        let ch = at(p);
        if ch == b' ' || ch == b'=' {
            p += 1;
            continue;
        }

        if kind == b'b' {
            cfg[setting] = i32::from(matches!(ch, b'T' | b't' | b'1' | b'Y' | b'y'));
            break;
        }
        if kind == b'i' {
            if ch == b'-' {
                negative = true;
                p += 1;
                continue;
            }
            if c_isdigit(ch) {
                cfg[setting] = atoi(&line[p..]);
                if negative {
                    // C `*= -1` (int overflow on INT_MIN; wraps)
                    cfg[setting] = cfg[setting].wrapping_mul(-1);
                }
                break;
            }
        }

        // anything else is not a value: keep the default
        break;
    }

    true
}

/// Release resources allocated by `borg_init_txt_file()`
/// (`borg_free_txt_file`).
fn borg_free_txt_file(st: &mut BorgState) {
    crate::borg_trait::borg_trait_free(st);
    crate::borg_formulas::borg_free_formulas(st);

    st.core.cfg = Vec::new();
}

/// What the borg.txt parser reads from and writes to: the open file, the
/// formula loader and the outputs. The game uses [`TxtFile`]; the unit
/// tests use a fake.
pub(crate) trait TxtEnv {
    fn st(&mut self) -> &mut BorgState;
    /// `file_getl(fp, buf, sizeof(buf) - 1)` with a `TXT_BUF` buffer.
    fn getl(&mut self, buf: &mut [u8; TXT_BUF]) -> bool;
    /// `borg_load_formulas(fp)`
    fn load_formulas(&mut self);
    /// `borg_warning(what)`
    fn warning(&mut self, what: &str);
    /// `Term_fresh()`
    fn term_fresh(&mut self);
    /// `player->class->cidx` (`None` while there is no player class)
    fn class_cidx(&mut self) -> Option<u32>;
}

/// The borg.txt of the game (`None` once closed).
struct TxtFile<'a> {
    st: &'a mut BorgState,
    fp: Option<engine::ReadFile>,
}

impl TxtEnv for TxtFile<'_> {
    fn st(&mut self) -> &mut BorgState {
        self.st
    }
    fn getl(&mut self, buf: &mut [u8; TXT_BUF]) -> bool {
        self.fp
            .as_mut()
            .expect("borg.txt is open")
            .getl(buf, TXT_BUF - 1)
    }
    fn load_formulas(&mut self) {
        let fp = self.fp.as_mut().expect("borg.txt is open");
        crate::borg_formulas::borg_load_formulas_from(self.st, |buf, len| fp.getl(buf, len));
    }
    fn warning(&mut self, what: &str) {
        borg_warning(self.st, what);
    }
    fn term_fresh(&mut self) {
        engine::term_fresh();
    }
    fn class_cidx(&mut self) -> Option<u32> {
        engine::class_cidx_if_set()
    }
}

/// The default borg.txt that `borg_init_txt_file()` writes when there is
/// none (each piece is one `file_putf()`).
pub(crate) fn default_txt_file() -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = vec![
        b"# BORG.txt default settings \n".to_vec(),
        b"# A more descriptive version of this file is delivered. \n".to_vec(),
        b"# Check your original install for borg.txt and \n".to_vec(),
        b"# replace this one with the delivered version. \n\n".to_vec(),
    ];
    for s in &BORG_SETTINGS {
        let name = s.name.to_str().expect("ASCII setting name");
        if s.kind == b'b' {
            let v = if s.default_value != 0 {
                "TRUE"
            } else {
                "FALSE"
            };
            out.push(format!("{name} = {v}\n").into_bytes());
        }
        if s.kind == b'i' {
            out.push(format!("{name} = {}\n", s.default_value).into_bytes());
        }
    }
    out
}

/// The parse loop of `borg_init_txt_file()`: start `borg_cfg` with the
/// defaults and read the settings. Returns `(formulas_found,
/// old_formulas_found)`.
pub(crate) fn read_txt_settings<E: TxtEnv>(env: &mut E) -> (bool, bool) {
    let mut formulas_found = false;
    let mut old_formulas_found = false;
    let n = BORG_SETTINGS.len();

    // allocate the config data and start it with the default values
    env.st().core.cfg = BORG_SETTINGS.iter().map(|s| s.default_value).collect();

    let mut buf = [0u8; TXT_BUF];

    // Parse the file
    while env.getl(&mut buf) {
        // Skip comments and blank lines
        if buf[0] == 0 || buf[0] == b'#' {
            continue;
        }

        // Chop the buffer
        buf[TXT_BUF - 1] = 0;
        let len = buf.iter().position(|&c| c == 0).expect("NUL-terminated");
        let line = &buf[..len];

        let mut i = 0;
        while i < n {
            let s = &BORG_SETTINGS[i];
            if borg_proc_setting(&mut env.st().core.cfg, i, s.name.to_bytes(), s.kind, line) {
                break;
            }
            i += 1;
        }
        if i != n {
            continue;
        }

        // other settings
        if prefix_i(line, b"REQ") {
            old_formulas_found = true;
            continue;
        }
        if prefix_i(line, b"FORMULA") {
            old_formulas_found = true;
            continue;
        }
        if prefix_i(line, b"CND") {
            old_formulas_found = true;
            continue;
        }
        if prefix_i(line, b"POWER") {
            old_formulas_found = true;
            continue;
        }

        if prefix_i(line, b"[BEGIN FORMULA SECTION]") {
            formulas_found = true;
            env.load_formulas();
        }
    }

    (formulas_found, old_formulas_found)
}

/// The end of `borg_init_txt_file()`, after the file is closed: the
/// dynamic calculation warnings and the range checks. Returns whether a
/// warning was given.
pub(crate) fn finish_txt_settings<E: TxtEnv>(
    env: &mut E,
    formulas_found: bool,
    old_formulas_found: bool,
) -> bool {
    let mut warning = false;
    let dyn_calcs = BORG_USES_DYNAMIC_CALCS as usize;

    if env.st().core.cfg[dyn_calcs] != 0 {
        warning = true;
        if old_formulas_found {
            env.warning(
                "** Borg's dynamic calculations enabled but old formulas found in borg.txt.",
            );
            env.warning("** formulas disabled ** ");
            env.st().core.cfg[dyn_calcs] = 0;
        } else if !formulas_found {
            env.warning(
                "** Borg's dynamic calculations enabled but no formulas found in borg.txt.",
            );
            env.warning("** formulas disabled ** ");
            env.st().core.cfg[dyn_calcs] = 0;
        } else {
            env.warning(
                "Borg's dynamic calculations enabled.  You may see some performance loss (~20 percent).",
            );
        }

        // Flush it
        env.term_fresh();
    }

    let st = env.st();
    let cfg = &mut st.core.cfg;

    // lunal mode is a default rather than a setting
    st.borg.lunal_mode = cfg[BORG_LUNAL_MODE as usize] != 0;

    // a few sanity range checks (C: `<= lo` then `>= hi` tests)
    let level = BORG_MUNCHKIN_LEVEL as usize;
    cfg[level] = cfg[level].clamp(1, 50);

    let depth = BORG_MUNCHKIN_DEPTH as usize;
    if cfg[depth] <= 1 {
        cfg[depth] = 8;
    }
    if cfg[depth] >= 100 {
        cfg[depth] = 100;
    }

    let class_depth = BORG_MUNCHKIN_CLASS_DEPTH as usize;
    if cfg[class_depth] <= 1 {
        cfg[class_depth] = 8;
    }
    if cfg[class_depth] >= 100 {
        cfg[class_depth] = 100;
    }

    st.init.munchkin_start_setting = cfg[BORG_MUNCHKIN_START as usize];
    st.init.munchkin_depth_setting = cfg[depth];
    let cidx = env.class_cidx();
    let st = env.st();
    borg_munchkin_class_settings(st, cidx);
    let cfg = &mut st.core.cfg;

    let enchant = BORG_ENCHANT_LIMIT as usize;
    cfg[enchant] = cfg[enchant].clamp(8, 15);

    let race = BORG_RESPAWN_RACE as usize;
    if cfg[race] >= MAX_RACES || cfg[race] < -1 {
        cfg[race] = 0;
    }

    let class = BORG_RESPAWN_CLASS as usize;
    if cfg[class] >= MAX_CLASSES || cfg[class] < -1 {
        cfg[class] = 0;
    }

    // make sure it continues to run if reset
    if cfg[BORG_RESPAWN_WINNERS as usize] != 0 {
        cfg[BORG_STOP_KING as usize] = 0;
    }

    // Post-M10 family settings are Rust-only behaviour. The C parser carries
    // them (the C Borg reads the same borg.txt and must know the names) but
    // no C code reads their values: the C oracle plays the original buying
    // behaviour, and this port's byte-identical parity with it is pinned by
    // the harness. Their consistency is checked in Rust (below) and the
    // C side deliberately does not duplicate it.
    let current = cfg[BORG_BUY_STRATEGY_CURRENT_CLASSES as usize];
    let reserve = cfg[BORG_BUY_STRATEGY_RESERVE410_CLASSES as usize];
    if (current != 0 && reserve != 0) || current > 1 || reserve > 2 {
        borg_note(
            st,
            "**STARTUP FAILURE** borg buy-strategy masks conflict (current/reserve410)",
        );
        st.init.init_failure = true;
    }
    let current_m = cfg[BORG_BUY_STRATEGY_CURRENT_MUNCHKIN_CLASSES as usize];
    let reserve_m = cfg[BORG_BUY_STRATEGY_RESERVE410_MUNCHKIN_CLASSES as usize];
    if (current_m != 0 && reserve_m != 0) || current_m > 1 || reserve_m > 2 {
        borg_note(
            st,
            "**STARTUP FAILURE** borg buy-strategy masks conflict (munchkin current/reserve410)",
        );
        st.init.init_failure = true;
    }

    // Success
    warning
}

/// Initialize borg.txt (`borg_init_txt_file`). Returns whether a warning
/// was given.
pub fn borg_init_txt_file(st: &mut BorgState) -> bool {
    borg_free_txt_file(st);
    crate::borg_trait::borg_trait_init(st);

    // a couple of spot checks on settings definitions
    if BORG_SETTINGS[BORG_MUNCHKIN_LEVEL as usize].name.to_bytes() != b"borg_munchkin_level"
        || BORG_SETTINGS[BORG_RESPAWN_RACE as usize].name.to_bytes() != b"borg_respawn_race"
    {
        borg_note(st, "borg settings structures not correct.  aborting. ");
        st.init.init_failure = true;
    }

    let mut buf = [0u8; TXT_BUF];
    engine::path_build(&mut buf, engine::ANGBAND_DIR_USER, "borg.txt");
    let path = engine::buf_to_string(&buf);

    // Open the file
    let fp = engine::file_open_read(&path);

    // No file, use defaults
    let Some(fp) = fp else {
        // Complain
        borg_note(st, "*****WARNING***** You do not have a proper BORG.TXT file!");
        borg_note(
            st,
            "writing borg.txt to the \\user\\ subdirectory with default values",
        );
        borg_note(st, &format!("Which is: {path}"));
        engine::event_signal(EVENT_MESSAGE_FLUSH);

        let lines = default_txt_file();
        if !engine::file_write_text(&path, &lines.concat()) {
            borg_warning(st, "*****WARNING***** unable to write default BORG.TXT file!");
            return true;
        }
        let Some(fp) = engine::file_open_read(&path) else {
            borg_warning(st, "*****WARNING***** unable to read default BORG.TXT file!");
            return true;
        };
        let mut env = TxtFile {
            st,
            fp: Some(fp),
        };
        let (formulas_found, old_formulas_found) = read_txt_settings(&mut env);
        drop(env.fp.take());
        return finish_txt_settings(&mut env, formulas_found, old_formulas_found);
    };

    let mut env = TxtFile {
        st,
        fp: Some(fp),
    };
    let (formulas_found, old_formulas_found) = read_txt_settings(&mut env);
    drop(env.fp.take());
    finish_txt_settings(&mut env, formulas_found, old_formulas_found)
}

/// Reset the required options when returning from user control
/// (`borg_reinit_options`). The ignore-bit save and restore is handled by
/// the Rust ignore module; the options live on the engine's player.
pub fn borg_reinit_options(st: &mut BorgState) {
    // Save current key mode
    st.init_save.key_mode = if engine::player_option(OPT_rogue_like_commands) {
        KEYMAP_MODE_ROGUE
    } else {
        KEYMAP_MODE_ORIG
    };

    engine::borg_init_ignore(st);

    // The Borg uses the original keypress codes
    engine::option_set("rogue_like_commands", false);

    // No auto_more
    engine::option_set("auto_more", false);

    // We pick up items when we step on them
    engine::option_set("pickup_always", true);

    // We do not want verbose messages
    engine::option_set("pickup_detail", false);

    // We specify targets by hand
    engine::option_set("use_old_target", false);

    // We must pick items up without verification
    engine::option_set("pickup_inven", true);

    // Pile symbol '&' confuse the borg
    engine::option_set("show_piles", false);

    // We need space
    engine::option_set("show_labels", false);

    // flavors are confusing
    engine::option_set("show_flavors", false);

    // The "easy" options confuse the Borg
    engine::option_set("easy_open", false);
    engine::option_set("easy_alter", false);

    // Efficiency
    engine::set_hitpoint_warn(0);
}

/// `borg_reset_settings()`.
pub fn borg_reset_settings(st: &mut BorgState) {
    // Restore user key mode
    if st.init_save.key_mode == KEYMAP_MODE_ROGUE {
        engine::option_set("rogue_like_commands", true);
    } else if st.init_save.key_mode == KEYMAP_MODE_ORIG {
        engine::option_set("rogue_like_commands", false);
    }

    engine::borg_reset_ignore(st);
    crate::borg_update::borg_free_detection(st);
}

/// Tell the borg that the game was closed (`borg_leave_game`).
extern "C" fn borg_leave_game(
    ev_type: game_event_type,
    ev_data: *mut game_event_data,
    user: *mut core::ffi::c_void,
) {
    assert!(
        ev_type == EVENT_LEAVE_GAME && ev_data.is_null() && user.is_null()
    );
    engine::with_state(|st| st.init.game_closed = true);
}

/// Prepare some stuff based on the player race and class
/// (`borg_prepare_race_class_info`).
pub fn borg_prepare_race_class_info(st: &mut BorgState) {
    // The munchkin start may depend on the class
    let cidx = engine::class_cidx_if_set();
    borg_munchkin_class_settings(st, cidx);

    // Initialize the various spell arrays by book
    crate::borg_magic::borg_prepare_book_info(st);

    // Notice if the game is closed so a reinitialization can be done if the
    // game is restarted without exiting.
    // this needs to be done here because the game clears all events when
    // the dungeon is recreated
    st.init.game_closed = false;
    engine::event_add_handler(EVENT_LEAVE_GAME, borg_leave_game);
}

/// Initialize the Borg (`borg_init`).
pub fn borg_init(st: &mut BorgState) {
    // Verify system
    // Redraw everything
    engine::do_cmd_redraw(st);

    // Message
    borg_note(st, "Initializing the Borg... (memory)");
    st.init.init_failure = false;

    // Flush it
    engine::term_fresh();

    // Mega-Hack -- verify memory (C allocates and frees 400k zero bytes)

    // Prapare a local random number seed (randint1)
    if st.core.rand_local == 0 {
        st.core.rand_local = (engine::randint0(0x1000_0000) + 1) as u32;
    }

    // C: `borg.player = player;` (an MSVC work-around). The Rust `Borg`
    // has no such field: the player is read through `engine`.

    // old depth starts at deepest level
    st.borg.status.old_depth = 128;

    // Initialize borg.ini options

    // Message
    borg_note(st, "Initializing the Borg... (borg.txt)");
    let warning_given = borg_init_txt_file(st);

    // Initialize game options

    // Message
    borg_note(st, "Initializing the Borg... (options)");

    // Flush it
    engine::term_fresh();

    // Make sure it rolls up a new guy at death
    if engine::screensaver() || st.core.cfg[BORG_CHEAT_DEATH as usize] != 0 {
        // We need the borg to keep playing after he dies
        engine::option_set("cheat_live", true);
    }

    // Redraw
    // Redraw map
    engine::player_redraw(PR_MAP);

    // Various

    // Message
    borg_note(st, "Initializing the Borg... (various)");

    // Flush it
    engine::term_fresh();

    // Cheat / Panic

    // more cheating
    st.borg.status.cheat_death = false;

    // set the continuous play mode if the game cheat death is on
    if engine::player_option(OPT_cheat_live) {
        st.borg.status.cheat_death = true;
    }

    // Initialize

    crate::borg_messages::borg_init_messages(st);

    // Initialize
    crate::borg_io::borg_init_io(st);
    crate::borg_item_val::borg_init_item_val(st);
    crate::borg_item_activation::borg_init_item_activation(st);

    // Grids
    crate::borg_cave::borg_init_cave(st);
    crate::borg_flow::borg_init_flow(st);

    crate::borg_item_wear::borg_init_item_wear(st);
    crate::borg_flow_take::borg_init_flow_take(st);
    crate::borg_flow_kill::borg_init_flow_kill(st);

    crate::borg_item::borg_init_item(st);
    crate::borg_store::borg_init_store(st);
    // Object/Monster tracking
    crate::borg_update::borg_init_update(st);

    // React to race and class

    // Notice the new race and class
    borg_prepare_race_class_info(st);

    // All done

    // Done initialization
    borg_note(st, "Initializing the Borg... done.");

    // Reset the clock
    st.borg.time.now = 10;

    // note: I would check if player_id2class returns null but it
    // never does, even on a bad class
    let names_ok = engine::class_name_is(CLASS_WARRIOR, "Warrior")
        && engine::class_name_is(1, "Mage")
        && engine::class_name_is(CLASS_DRUID, "Druid")
        && engine::class_name_is(3, "Priest")
        && engine::class_name_is(4, "Necromancer")
        && engine::class_name_is(5, "Paladin")
        && engine::class_name_is(6, "Rogue")
        && engine::class_name_is(CLASS_RANGER, "Ranger")
        && engine::class_name_is(8, "Blackguard");
    if !names_ok {
        borg_note(st, "**STARTUP FAILURE** classes do not match");
        st.init.init_failure = true;
    }

    // Don't allow the user to do stupid things unless they ask to
    if st.core.cfg[BORG_ALLOW_STRANGE_OPTS as usize] == 0 {
        if engine::tile_width() > 1 || engine::tile_height() > 1 {
            borg_note(st, "**STARTUP FAILURE** Scaled tiles not allowed");
            borg_note(st, "** can be bypassed with borg_allow_strange_opts **");
            st.init.init_failure = true;
        }

        if engine::player_option(OPT_birth_force_descend) {
            borg_note(st, "**STARTUP FAILURE** must allow up stairs");
            borg_note(st, "** birth option failure **");
            borg_note(st, "** can be bypassed with borg_allow_strange_opts **");
            st.init.init_failure = true;
        }

        if !engine::player_option(OPT_birth_connect_stairs) {
            borg_note(st, "**STARTUP FAILURE** must connect stairs");
            borg_note(st, "** birth option failure **");
            borg_note(st, "** can be bypassed with borg_allow_strange_opts **");
            st.init.init_failure = true;
        }

        if engine::player_option(OPT_birth_no_recall) {
            borg_note(st, "**STARTUP FAILURE** must allow recall");
            borg_note(st, "** birth option failure **");
            borg_note(st, "** can be bypassed with borg_allow_strange_opts **");
            st.init.init_failure = true;
        }

        if engine::player_option(OPT_birth_percent_damage) {
            borg_note(st, "**STARTUP FAILURE** strange damage calculation");
            borg_note(st, "** birth option failure **");
            borg_note(st, "** can be bypassed with borg_allow_strange_opts **");
            st.init.init_failure = true;
        }
    }

    // Official message
    if !st.init.init_failure && !warning_given {
        borg_note(st, "# Ready...");
    }

    // Now it is ready
    st.init.initialized = true;
}

/// Clean up resources allocated for the borg (`borg_free`).
pub fn borg_free(st: &mut BorgState) {
    // Undo the allocations in reverse order from what borg_init() does.
    engine::event_remove_handler(EVENT_LEAVE_GAME, borg_leave_game);
    crate::borg_update::borg_free_update(st);
    crate::borg_item::borg_free_item(st);
    crate::borg_store::borg_free_store(st);

    crate::borg_flow_kill::borg_free_flow_kill(st);
    crate::borg_flow_take::borg_free_flow_take(st);
    crate::borg_item_wear::borg_free_item_wear(st);

    crate::borg_flow::borg_free_flow(st);
    crate::borg_cave::borg_free_cave(st);
    crate::borg_io::borg_free_io(st);

    crate::borg_messages::borg_free_messages(st);
    borg_free_txt_file(st);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borg_headers::borg_h::{
        BORG_MUNCHKIN_CLASS_DEPTH, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_START,
        BORG_USES_DYNAMIC_CALCS,
    };

    /// A `TxtEnv` over an in-memory borg.txt, with the outputs recorded.
    struct FakeTxt {
        st: BorgState,
        lines: Vec<u8>,
        at: usize,
        out: Vec<String>,
        formulas: Vec<u8>,
        class: Option<u32>,
    }

    impl FakeTxt {
        fn new(text: &[u8]) -> Self {
            FakeTxt {
                st: BorgState::default(),
                lines: text.to_vec(),
                at: 0,
                out: Vec::new(),
                formulas: Vec::new(),
                class: None,
            }
        }

        /// Run the parse and the finish; return the warnings.
        fn run(text: &[u8]) -> Vec<String> {
            let mut env = FakeTxt::new(text);
            let (found, old) = read_txt_settings(&mut env);
            finish_txt_settings(&mut env, found, old);
            env.out
        }
    }

    impl TxtEnv for FakeTxt {
        fn st(&mut self) -> &mut BorgState {
            &mut self.st
        }
        fn getl(&mut self, buf: &mut [u8; TXT_BUF]) -> bool {
            // `file_getl`: one line, LF gone, NUL-terminated; false at EOF
            if self.at >= self.lines.len() {
                return false;
            }
            let rest = &self.lines[self.at..];
            let eol = rest.iter().position(|&c| c == b'\n').unwrap_or(rest.len());
            let line = &rest[..eol];
            let n = line.len().min(TXT_BUF - 2);
            buf[..n].copy_from_slice(&line[..n]);
            buf[n] = 0;
            self.at += eol + 1;
            true
        }
        fn load_formulas(&mut self) {
            // enough for the tests: the formulas are the rest of the file
            self.formulas = self.lines[self.at..].to_vec();
            self.at = self.lines.len();
        }
        fn warning(&mut self, what: &str) {
            self.out.push(what.to_string());
        }
        fn term_fresh(&mut self) {}
        fn class_cidx(&mut self) -> Option<u32> {
            self.class
        }
    }

    /// Parse `text` and return the resulting `borg_cfg` and warnings.
    fn cfg_of(text: &[u8]) -> (Vec<i32>, Vec<String>) {
        let mut env = FakeTxt::new(text);
        let (found, old) = read_txt_settings(&mut env);
        finish_txt_settings(&mut env, found, old);
        (env.st.core.cfg.clone(), env.out)
    }

    #[test]
    fn defaults_match_the_table() {
        let (cfg, out) = cfg_of(b"");
        assert_eq!(cfg.len(), BORG_SETTINGS.len());
        for (i, s) in BORG_SETTINGS.iter().enumerate() {
            assert_eq!(cfg[i], s.default_value, "{:?}", s.name);
        }
        assert!(out.is_empty());
    }

    #[test]
    fn settings_parse_like_c() {
        let (cfg, _) = cfg_of(
            b"borg_verbose = TRUE\nborg_stop_dlevel = 100\n  borg_stop_clevel 42\n\
              borg_delay_factor=-3\nborg_enchant_limit = 9xy\nborg_save_death = yes\n",
        );
        assert_eq!(cfg[0], 1, "borg_verbose");
        assert_eq!(cfg[14], 100, "borg_stop_dlevel");
        assert_eq!(cfg[15], 42, "borg_stop_clevel");
        assert_eq!(cfg[23], -3, "borg_delay_factor");
        assert_eq!(cfg[28], 9, "borg_enchant_limit: atoi stops at x");
        assert_eq!(cfg[30], 1, "borg_save_death");
        // unknown lines, comments and blank lines are skipped
        let (cfg, _) = cfg_of(b"# comment\n\nnot a setting\nborg_verbose = FALSE\n");
        assert_eq!(cfg[0], 0);
    }

    #[test]
    fn prefix_is_case_insensitive() {
        let (cfg, _) = cfg_of(b"BORG_VERBOSE = TRUE\nBoRg_StOp_DlEvEl = 7\n");
        assert_eq!(cfg[0], 1);
        assert_eq!(cfg[14], 7);
    }

    #[test]
    fn dynamic_calcs_warnings() {
        // off: no warning, no output
        let (cfg, out) = cfg_of(b"borg_uses_dynamic_calcs = FALSE\n");
        assert_eq!(cfg[BORG_USES_DYNAMIC_CALCS as usize], 0);
        assert!(out.is_empty());
        // on with the formula section: one informational warning
        let (cfg, out) = cfg_of(
            b"borg_uses_dynamic_calcs = TRUE\n[BEGIN FORMULA SECTION]\n[END FORMULA SECTION]\n",
        );
        assert_eq!(cfg[BORG_USES_DYNAMIC_CALCS as usize], 1);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("dynamic calculations enabled"));
        // on with old formulas: disabled again
        let (cfg, out) = cfg_of(b"borg_uses_dynamic_calcs = TRUE\nREQ str 10\n");
        assert_eq!(cfg[BORG_USES_DYNAMIC_CALCS as usize], 0);
        assert!(out.iter().any(|w| w.contains("old formulas")));
        // on with no formulas at all: disabled again
        let (cfg, out) = cfg_of(b"borg_uses_dynamic_calcs = TRUE\n");
        assert_eq!(cfg[BORG_USES_DYNAMIC_CALCS as usize], 0);
        assert!(out.iter().any(|w| w.contains("no formulas")));
    }

    #[test]
    fn range_checks_clamp_like_c() {
        let (cfg, _) = cfg_of(b"borg_munchkin_level = 0\nborg_munchkin_depth = 200\n\
              borg_enchant_limit = 20\nborg_respawn_race = -2\nborg_respawn_class = 9\n\
              borg_respawn_winners = TRUE\n");
        assert_eq!(cfg[2], 1, "munchkin_level <= 1 becomes 1");
        assert_eq!(cfg[3], 100, "munchkin_depth >= 100 becomes 100");
        assert_eq!(cfg[28], 15, "enchant_limit >= 15 becomes 15");
        assert_eq!(cfg[20], 0, "respawn_race out of range becomes 0");
        assert_eq!(cfg[21], 0, "respawn_class out of range becomes 0");
        assert_eq!(cfg[17], 0, "respawn_winners clears stop_king");
        let (cfg, _) = cfg_of(b"borg_munchkin_depth = 0\n");
        assert_eq!(cfg[3], 8, "munchkin_depth <= 1 becomes 8");
    }

    #[test]
    fn munchkin_class_rule() {
        let run = |text: &[u8], class: Option<u32>| {
            let mut env = FakeTxt::new(text);
            env.class = class;
            let (found, old) = read_txt_settings(&mut env);
            finish_txt_settings(&mut env, found, old);
            let c = &env.st.core.cfg;
            (c[BORG_MUNCHKIN_START as usize], c[BORG_MUNCHKIN_DEPTH as usize])
        };
        // with no borg.txt the rule is off: the original C Borg's behaviour
        for class in [0, 1, 2, 3, 4, 5, 6, 7, 8] {
            assert_eq!(run(b"", Some(class)), (0, 16), "class {class}");
        }
        assert_eq!(run(b"", None), (0, 16));
        // the rule on: the three classes take the munchkin start at depth 6
        for class in [0, 2, 7] {
            assert_eq!(
                run(b"borg_munchkin_by_class = TRUE\n", Some(class)),
                (1, 6),
                "class {class}"
            );
        }
        // every other class, and no player class yet
        for class in [1, 3, 4, 5, 6, 8] {
            assert_eq!(
                run(b"borg_munchkin_by_class = TRUE\n", Some(class)),
                (0, 16),
                "class {class}"
            );
        }
        assert_eq!(run(b"borg_munchkin_by_class = TRUE\n", None), (0, 16));
        // the rule off, a munchkin start from borg.txt, the depth setting
        assert_eq!(run(b"borg_munchkin_by_class = FALSE\n", Some(0)), (0, 16));
        assert_eq!(run(b"borg_munchkin_start = TRUE\n", Some(0)), (1, 16));
        assert_eq!(
            run(b"borg_munchkin_by_class = TRUE\nborg_munchkin_class_depth = 9\n", Some(2)),
            (1, 9)
        );
        assert_eq!(
            run(b"borg_munchkin_by_class = TRUE\nborg_munchkin_class_depth = 0\n", Some(7)),
            (1, 8)
        );

        // a reincarnation into another class restores the borg.txt values
        let mut env = FakeTxt::new(b"borg_munchkin_by_class = TRUE\n");
        env.class = Some(0);
        let (found, old) = read_txt_settings(&mut env);
        finish_txt_settings(&mut env, found, old);
        borg_munchkin_class_settings(&mut env.st, Some(1));
        let c = &env.st.core.cfg;
        assert_eq!((c[BORG_MUNCHKIN_START as usize], c[BORG_MUNCHKIN_DEPTH as usize]), (0, 16));
    }

    #[test]
    fn buying_strategy_settings_parse_and_validate() {
        use crate::borg_headers::borg_h::*;
        let (cfg, _) = cfg_of(
            b"borg_buy_strategy_default = 1\nborg_buy_strategy_munchkin = 0\n\
              borg_buy_strategy_current_classes = 1\nborg_buy_strategy_reserve410_classes = 2\n\
              borg_buy_strategy_current_munchkin_classes = 4\n\
              borg_buy_strategy_reserve410_munchkin_classes = 8\n\
              borg_buy_identify = 1\n",
        );
        assert_eq!(cfg[BORG_BUY_STRATEGY_DEFAULT as usize], 1);
        assert_eq!(cfg[BORG_BUY_STRATEGY_MUNCHKIN as usize], 0);
        assert_eq!(cfg[BORG_BUY_STRATEGY_CURRENT_CLASSES as usize], 1);
        assert_eq!(cfg[BORG_BUY_STRATEGY_RESERVE410_CLASSES as usize], 2);
        assert_eq!(cfg[BORG_BUY_STRATEGY_CURRENT_MUNCHKIN_CLASSES as usize], 4);
        assert_eq!(cfg[BORG_BUY_STRATEGY_RESERVE410_MUNCHKIN_CLASSES as usize], 8);
        assert_eq!(cfg[BORG_BUY_IDENTIFY as usize], 1);

        let mut env = FakeTxt::new(
            b"borg_buy_strategy_current_classes = 1\nborg_buy_strategy_reserve410_classes = 1\n",
        );
        let (found, old) = read_txt_settings(&mut env);
        finish_txt_settings(&mut env, found, old);
        assert!(env.st.init.init_failure);
        assert!(env.out.iter().any(|line| line.contains("masks conflict")));
    }

    #[test]
    fn default_file_matches_c() {
        let text: Vec<u8> = default_txt_file().concat();
        let text = String::from_utf8(text).unwrap();
        assert!(text.starts_with("# BORG.txt default settings \n"));
        assert!(text.contains("\n\nborg_verbose = FALSE\nborg_munchkin_start = FALSE\n"));
        assert!(text.contains("\nborg_respawn_class = -1\n"));
        assert!(text.ends_with(
            "borg_buy_strategy_default = 0\nborg_buy_strategy_munchkin = -1\n\
             borg_buy_strategy_current_classes = 0\nborg_buy_strategy_reserve410_classes = 0\n\
             borg_buy_strategy_current_munchkin_classes = 0\n\
             borg_buy_strategy_reserve410_munchkin_classes = 0\n\
             borg_buy_identify = 0\nborg_town_no_stores = FALSE\n\
             borg_town_plan = 0\nborg_town_w_light = 100\nborg_town_w_food = 100\n\
             borg_town_w_bleeding = 100\nborg_town_w_poison = 100\nborg_town_w_escape = 50\n\
             borg_town_w_knowledge = 25\nborg_town_w_power = 50\nborg_town_w_trip = 50\n\
             borg_town_share = 10\nborg_town_light_depth = 15\nborg_town_food_trip = 10\n"
        ));
        assert!(text.contains("\nborg_munchkin_by_class = FALSE\n"));
    }

    /// The town inventory redesign's parameter vector (borg-rs#36): the
    /// twelve settings parse like any other int setting.
    #[test]
    fn town_settings_parse() {
        use crate::borg_headers::borg_h::*;
        let (cfg, _) = cfg_of(
            b"borg_town_plan = 1\nborg_town_w_light = 90\nborg_town_w_food = 80\n\
              borg_town_w_bleeding = 70\nborg_town_w_poison = 60\nborg_town_w_escape = 55\n\
              borg_town_w_knowledge = 45\nborg_town_w_power = 35\nborg_town_w_trip = 25\n\
              borg_town_share = 4\nborg_town_light_depth = 20\nborg_town_food_trip = 12\n",
        );
        assert_eq!(cfg[BORG_TOWN_PLAN as usize], 1);
        assert_eq!(cfg[BORG_TOWN_W_LIGHT as usize], 90);
        assert_eq!(cfg[BORG_TOWN_W_FOOD as usize], 80);
        assert_eq!(cfg[BORG_TOWN_W_BLEEDING as usize], 70);
        assert_eq!(cfg[BORG_TOWN_W_POISON as usize], 60);
        assert_eq!(cfg[BORG_TOWN_W_ESCAPE as usize], 55);
        assert_eq!(cfg[BORG_TOWN_W_KNOWLEDGE as usize], 45);
        assert_eq!(cfg[BORG_TOWN_W_POWER as usize], 35);
        assert_eq!(cfg[BORG_TOWN_W_TRIP as usize], 25);
        assert_eq!(cfg[BORG_TOWN_SHARE as usize], 4);
        assert_eq!(cfg[BORG_TOWN_LIGHT_DEPTH as usize], 20);
        assert_eq!(cfg[BORG_TOWN_FOOD_TRIP as usize], 12);
    }

    /// Differential check against the C parser: for every file in
    /// `$BORG_TXT_DIFF_DIR` (not `*.out`), write what the C driver prints
    /// to `<file>.rust.out`. Run with `--ignored`.
    #[test]
    #[ignore]
    fn dump_txt_dir() {
        let Ok(dir) = std::env::var("BORG_TXT_DIFF_DIR") else { return };
        for e in std::fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "out") {
                continue;
            }
            let data = std::fs::read(&p).unwrap();
            let mut text = FakeTxt::run(&data).join("\n");
            text.push('\n');
            std::fs::write(format!("{}.rust.out", p.display()), text).unwrap();
        }
        let text: Vec<u8> = default_txt_file().concat();
        std::fs::write(format!("{dir}/default.rust.out"), text).unwrap();
    }

    /// The original C Borg's settings, with the defaults Angband's own C Borg
    /// had at the upstream pin `e473720ed88a0f0c7ca66815cef3000eb4a38bc3`
    /// (the tree vendored under `angband/`). Every one of them keeps that
    /// default in borg-rs; a setting the upstream did not have is declared in
    /// `NEW_SETTINGS` instead.
    const UPSTREAM_SETTINGS: &[(&str, u8, i32)] = &[
        ("borg_verbose", b'b', 0),
        ("borg_munchkin_start", b'b', 0),
        ("borg_munchkin_level", b'i', 12),
        ("borg_munchkin_depth", b'i', 16),
        ("borg_worships_damage", b'b', 0),
        ("borg_worships_speed", b'b', 0),
        ("borg_worships_hp", b'b', 0),
        ("borg_worships_mana", b'b', 0),
        ("borg_worships_ac", b'b', 0),
        ("borg_worships_gold", b'b', 0),
        ("borg_plays_risky", b'b', 0),
        ("borg_kills_uniques", b'b', 0),
        ("borg_uses_swaps", b'b', 1),
        ("borg_uses_dynamic_calcs", b'b', 0),
        ("borg_stop_dlevel", b'i', 128),
        ("borg_stop_clevel", b'i', 51),
        ("borg_no_deeper", b'i', 127),
        ("borg_stop_king", b'b', 1),
        ("borg_cheat_death", b'b', 0),
        ("borg_respawn_winners", b'b', 0),
        ("borg_respawn_class", b'i', -1),
        ("borg_respawn_race", b'i', -1),
        ("borg_chest_fail_tolerance", b'i', 7),
        ("borg_delay_factor", b'i', 0),
        ("borg_money_scum_amount", b'i', 0),
        ("borg_self_scum", b'b', 1),
        ("borg_lunal_mode", b'b', 0),
        ("borg_self_lunal", b'b', 0),
        ("borg_enchant_limit", b'i', 12),
        ("borg_dump_level", b'i', 1),
        ("borg_save_death", b'i', 1),
        ("borg_stop_on_bell", b'b', 0),
        ("borg_allow_strange_opts", b'b', 0),
        ("borg_autosave", b'b', 0),
        ("borg_restore_ignore_settings", b'b', 0),
    ];

    /// The settings the upstream C Borg did not have: name, kind, the built-in
    /// default, and the value the two borg.txt files give them. A setting
    /// whose two values differ is a product choice, named here; every other
    /// new setting reproduces the behaviour the original C Borg had.
    ///
    /// `borg_munchkin_by_class` is the one that differs: the built-in default
    /// is FALSE, the original C Borg's behaviour, and the product's milestone
    /// 8 decision (better-borg-c PR302, `fd683ced`) is carried by the two
    /// borg.txt files instead. `docs/ZOO-INTERFACE.md` is the prose version
    /// of this table, and the audit behind it.
    const NEW_SETTINGS: &[(&str, u8, i32, &str)] = &[
        ("borg_munchkin_by_class", b'b', 0, "TRUE"),
        ("borg_munchkin_class_depth", b'i', 6, "6"),
        ("borg_buy_strategy_default", b'i', 0, "0"),
        ("borg_buy_strategy_munchkin", b'i', -1, "-1"),
        ("borg_buy_strategy_current_classes", b'i', 0, "0"),
        ("borg_buy_strategy_reserve410_classes", b'i', 0, "0"),
        ("borg_buy_strategy_current_munchkin_classes", b'i', 0, "0"),
        ("borg_buy_strategy_reserve410_munchkin_classes", b'i', 0, "0"),
        ("borg_buy_identify", b'i', 0, "0"),
        ("borg_town_no_stores", b'b', 0, "FALSE"),
        ("borg_town_plan", b'i', 0, "0"),
        ("borg_town_w_light", b'i', 100, "100"),
        ("borg_town_w_food", b'i', 100, "100"),
        ("borg_town_w_bleeding", b'i', 100, "100"),
        ("borg_town_w_poison", b'i', 100, "100"),
        ("borg_town_w_escape", b'i', 50, "50"),
        ("borg_town_w_knowledge", b'i', 25, "25"),
        ("borg_town_w_power", b'i', 50, "50"),
        ("borg_town_w_trip", b'i', 50, "50"),
        ("borg_town_share", b'i', 10, "10"),
        ("borg_town_light_depth", b'i', 15, "15"),
        ("borg_town_food_trip", b'i', 10, "10"),
    ];

    /// The setting lines of a borg.txt: the name and the value, in order.
    /// Comment lines and the formula section carry no setting lines.
    fn borg_txt_settings(text: &str) -> Vec<(String, String)> {
        text.lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .filter_map(|l| {
                let (name, value) = l.split_once('=')?;
                let name = name.trim();
                if !name.starts_with("borg_") {
                    return None;
                }
                Some((name.to_string(), value.trim().to_string()))
            })
            .collect()
    }

    /// The table entry for a setting name.
    fn setting(name: &str) -> Option<&'static BorgSetting> {
        BORG_SETTINGS.iter().find(|s| s.name.to_str().ok() == Some(name))
    }

    /// The upstream C Borg's defaults are the ones borg-rs keeps (RULES 1 of
    /// the zoo: a parameter enters it with a default that reproduces the
    /// original behaviour).
    #[test]
    fn upstream_defaults_are_unchanged() {
        for (name, kind, default) in UPSTREAM_SETTINGS {
            let s = setting(name).unwrap_or_else(|| panic!("{name} is missing from the table"));
            assert_eq!(s.kind, *kind, "{name}");
            assert_eq!(s.default_value, *default, "{name}");
        }
    }

    /// Every setting is either an upstream setting or a declared new one, and
    /// a declared one says what it is: adding a setting is a deliberate act.
    #[test]
    fn every_setting_is_declared() {
        for s in BORG_SETTINGS.iter() {
            let name = s.name.to_str().unwrap();
            let upstream = UPSTREAM_SETTINGS.iter().any(|(n, ..)| *n == name);
            let new = NEW_SETTINGS.iter().find(|(n, ..)| *n == name);
            assert!(
                upstream || new.is_some(),
                "{name} is neither an upstream setting nor a declared new one"
            );
            if let Some((_, kind, default, _)) = new {
                assert_eq!(s.kind, *kind, "{name}");
                assert_eq!(s.default_value, *default, "{name}");
            }
        }
        for (name, ..) in NEW_SETTINGS {
            assert!(setting(name).is_some(), "{name} is declared but not in the table");
        }
    }

    /// The C parser table and the Rust table carry the same settings, in the
    /// same order, with the same kinds and defaults: the C Borg reads them
    /// (and ignores the Rust-only ones), so a drift between the two would
    /// make the two backends read different configurations.
    #[test]
    fn the_c_table_matches_the_rust_table() {
        let c = include_str!("../../angband/src/borg/borg-init.c");
        let mut entries: Vec<(&str, u8, i32)> = Vec::new();
        for line in c.lines() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("{ \"") else {
                continue;
            };
            let Some((name, rest)) = rest.split_once("\", '") else {
                continue;
            };
            let Some((kind, rest)) = rest.split_once('\'') else {
                continue;
            };
            let default = rest
                .trim()
                .trim_start_matches(',')
                .trim()
                .trim_end_matches(',')
                .trim()
                .trim_end_matches('}')
                .trim();
            let value = match default {
                "true" => 1,
                "false" => 0,
                v => v.parse().unwrap(),
            };
            entries.push((name, kind.as_bytes()[0], value));
        }
        assert_eq!(entries.len(), BORG_SETTINGS.len());
        for (i, (name, kind, value)) in entries.iter().enumerate() {
            let s = &BORG_SETTINGS[i];
            assert_eq!(s.name.to_str().unwrap(), *name);
            assert_eq!(s.kind, *kind);
            assert_eq!(s.default_value, *value);
        }
    }

    /// The two borg.txt copies carry the same settings, and between them they
    /// name every setting in the table.
    #[test]
    fn the_borg_txt_copies_agree_and_name_every_setting() {
        let harness = borg_txt_settings(include_str!("../../harness/borg.txt"));
        let shipped = borg_txt_settings(include_str!("../../angband/src/borg/borg.txt"));
        assert_eq!(harness, shipped, "the two borg.txt copies disagree");
        for s in BORG_SETTINGS.iter() {
            let name = s.name.to_str().unwrap();
            assert!(
                harness.iter().any(|(n, _)| n == name),
                "{name} is missing from the borg.txt files"
            );
        }
    }

    /// The configured values follow the declaration: an upstream setting keeps
    /// its upstream default in the config, and a new setting carries the value
    /// `NEW_SETTINGS` names. A configured value that differs from the built-in
    /// default is a product choice, and it is listed.
    #[test]
    fn the_borg_txt_values_follow_the_declaration() {
        let harness = borg_txt_settings(include_str!("../../harness/borg.txt"));
        // the configured values are compared case-insensitively: one of the
        // borg.txt lines says "false" in lower case
        let value_of = |name: &str| -> String {
            harness
                .iter()
                .find(|(n, _)| n == name)
                .unwrap_or_else(|| panic!("{name} is missing from borg.txt"))
                .1
                .to_ascii_uppercase()
        };
        for (name, _, default) in UPSTREAM_SETTINGS {
            let want = if setting(name).unwrap().kind == b'b' {
                if *default != 0 { "TRUE" } else { "FALSE" }.to_string()
            } else {
                default.to_string()
            };
            assert_eq!(value_of(name), want, "{name}");
        }
        for (name, _, _, configured) in NEW_SETTINGS {
            assert_eq!(value_of(name), configured.to_ascii_uppercase(), "{name}");
        }
    }

}
