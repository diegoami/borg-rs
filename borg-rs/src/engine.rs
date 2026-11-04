//! Safe access to the C game engine: the permanent FFI boundary.
//!
//! The Borg runs inside the engine's single thread, called from
//! `inkey_hack` (or the `^Z` command). The engine globals read here are
//! stable for the duration of such a call.

use crate::headers;

/// The game's constants (`z_info`), set up by the engine before the Borg
/// can be started.
pub fn z_info() -> &'static headers::init_h::angband_constants {
    // SAFETY: z_info is allocated by init_angband() before any Borg code
    // runs and is never freed while the game runs.
    unsafe { &*headers::init_h::z_info }
}

use crate::headers::z_type_h::loc;

/// `square_in_bounds_fully(cave, loc(x, y))` on the current level.
pub fn square_in_bounds_fully(x: i32, y: i32) -> bool {
    // SAFETY: `cave` is the current level, valid while the game runs.
    unsafe { headers::cave_h::square_in_bounds_fully(headers::cave_h::cave, loc { x, y }) }
}

/// Panel geometry of the current terminal (`Term`), as read by
/// `borg_panel_hgt()` / `borg_panel_wid()`.
#[derive(Clone, Copy, Debug)]
pub struct TermGeometry {
    /// `Term == angband_term[0]`
    pub is_main: bool,
    pub wid: i32,
    pub hgt: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub row_top_map: i32,
    pub row_bottom_map: i32,
    pub col_map: i32,
    pub tile_width: i32,
    pub tile_height: i32,
}

/// Read the current terminal's geometry.
pub fn term_geometry() -> TermGeometry {
    use crate::headers::ui_term_h as t;
    // SAFETY: Term and the map/tile globals are set up by the frontend
    // before the Borg runs; we only read plain values.
    unsafe {
        let term = &*t::Term;
        let mode = term.sidebar_mode as usize;
        TermGeometry {
            is_main: t::Term == t::angband_term[0],
            wid: term.wid,
            hgt: term.hgt,
            offset_x: term.offset_x,
            offset_y: term.offset_y,
            row_top_map: t::row_top_map[mode],
            row_bottom_map: t::row_bottom_map[mode],
            col_map: t::col_map[mode],
            tile_width: i32::from(t::tile_width),
            tile_height: i32::from(t::tile_height),
        }
    }
}

// ---------------------------------------------------------------------------
// Terminal

use crate::headers::game_event_h::{game_event_handler, game_event_type};
use crate::headers::__stddef_wchar_t_h::wchar_t;
use core::ffi::c_char;
use std::ffi::CString;

/// An opaque handle to an engine terminal (to restore the active one).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TermHandle(*mut crate::headers::ui_term_h::term);

/// The active terminal (`Term`).
pub fn term_current() -> TermHandle {
    // SAFETY: reads a pointer-sized global.
    unsafe { TermHandle(crate::headers::ui_term_h::Term) }
}

/// `angband_term[j]`, if that window exists.
pub fn angband_term(j: usize) -> Option<TermHandle> {
    // SAFETY: reads a pointer from a fixed-size global array.
    let t = unsafe { crate::headers::ui_term_h::angband_term[j] };
    (!t.is_null()).then_some(TermHandle(t))
}

/// `window_flag[j]`.
pub fn window_flag(j: usize) -> u32 {
    // SAFETY: reads a fixed-size global array.
    unsafe { crate::headers::ui_term_h::window_flag[j] }
}

/// `Term_activate(t)`.
pub fn term_activate(t: TermHandle) {
    // SAFETY: t came from term_current()/angband_term(), a live term.
    unsafe {
        crate::headers::ui_term_h::Term_activate(t.0);
    }
}

/// `Term_get_size(&w, &h)`.
pub fn term_get_size() -> (i32, i32) {
    let (mut w, mut h) = (0, 0);
    // SAFETY: out-pointers to locals.
    unsafe {
        crate::headers::ui_term_h::Term_get_size(&mut w, &mut h);
    }
    (w, h)
}

/// `Term_locate(&x, &y)`: the cursor position.
pub fn term_locate() -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    // SAFETY: out-pointers to locals.
    unsafe {
        crate::headers::ui_term_h::Term_locate(&mut x, &mut y);
    }
    (x, y)
}

/// `Term_erase(x, y, n)`.
pub fn term_erase(x: i32, y: i32, n: i32) {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_erase(x, y, n);
    }
}

/// `Term_addstr(n, a, s)` with a byte string (no interior NUL).
pub fn term_addstr(n: i32, a: i32, s: &[u8]) {
    let c = CString::new(s).expect("no NUL in term text");
    // SAFETY: c is a valid NUL-terminated string for the call.
    unsafe {
        crate::headers::ui_term_h::Term_addstr(n, a, c.as_ptr());
    }
}

/// `Term_fresh()`.
pub fn term_fresh() {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_fresh();
    }
}

/// The attribute and character on screen at (x, y) of the active term
/// (`Term->scr->a[y][x]`, `Term->scr->c[y][x]`).
pub fn screen_at(x: i32, y: i32) -> (i32, wchar_t) {
    // SAFETY: callers keep (x, y) on the screen (checked against
    // term_get_size()); scr rows are allocated for the term size.
    unsafe {
        let scr = &*(*crate::headers::ui_term_h::Term).scr;
        let a = *(*scr.a.offset(y as isize)).offset(x as isize);
        let c = *(*scr.c.offset(y as isize)).offset(x as isize);
        (a, c)
    }
}

// ---------------------------------------------------------------------------
// Text conversion (locale dependent, like the C code)

/// `wcstombs(dst, src, dst.len())`: convert the NUL-terminated wide
/// string `src` into `dst`. Returns the number of bytes written (not
/// counting the NUL), or `usize::MAX` on a conversion error.
pub fn wcstombs(dst: &mut [u8], src: &[wchar_t]) -> usize {
    assert!(src.contains(&0), "wide string must be NUL-terminated");
    // SAFETY: src is NUL-terminated; wcstombs writes at most dst.len().
    unsafe {
        crate::headers::stdlib_h::wcstombs(dst.as_mut_ptr() as *mut c_char, src.as_ptr(), dst.len())
    }
}

/// `text_mbstowcs(dst, src, n)`: the game's multibyte-to-wide conversion.
/// `src` must be NUL-terminated.
pub fn text_mbstowcs(dst: &mut [wchar_t], src: &[u8], n: i32) -> usize {
    assert!(src.contains(&0), "string must be NUL-terminated");
    assert!(n >= 0 && n as usize <= dst.len());
    // SAFETY: src is NUL-terminated; at most n wide chars are written.
    unsafe {
        crate::headers::z_util_h::text_mbstowcs(dst.as_mut_ptr(), src.as_ptr() as *const c_char, n)
    }
}

// ---------------------------------------------------------------------------
// Messages and events

/// `msg("%s", text)`.
pub fn msg(text: &[u8]) {
    let c = CString::new(text).expect("no NUL in message");
    // SAFETY: format string and argument are valid C strings.
    unsafe { crate::headers::message_h::msg(c"%s".as_ptr(), c.as_ptr()) }
}

/// `message_add(text, type)`.
pub fn message_add(text: &[u8], kind: u16) {
    let c = CString::new(text).expect("no NUL in message");
    // SAFETY: c is valid for the call; message_add copies it.
    unsafe { crate::headers::message_h::message_add(c.as_ptr(), kind) }
}

/// `event_signal_message(type, t, NULL)`.
pub fn event_signal_message_null(kind: game_event_type, t: i32) {
    // SAFETY: NULL message text is allowed by the engine.
    unsafe { crate::headers::game_event_h::event_signal_message(kind, t, core::ptr::null()) }
}

/// `event_add_handler(type, fn, NULL)`.
pub fn event_add_handler(kind: game_event_type, f: game_event_handler) {
    // SAFETY: registers a function pointer with a NULL user pointer.
    unsafe { crate::headers::game_event_h::event_add_handler(kind, Some(f), core::ptr::null_mut()) }
}

/// `event_remove_handler(type, fn, NULL)`.
pub fn event_remove_handler(kind: game_event_type, f: game_event_handler) {
    // SAFETY: unregisters a function pointer registered above.
    unsafe {
        crate::headers::game_event_h::event_remove_handler(kind, Some(f), core::ptr::null_mut())
    }
}

// ---------------------------------------------------------------------------
// Player, input, RNG

use crate::headers::ui_event_h::{keypress, ui_event_type};

fn player_ref() -> &'static crate::headers::player_h::player {
    // SAFETY: `player` is allocated at startup and lives for the game.
    unsafe { &*crate::headers::player_h::player }
}

/// `player->is_dead`
pub fn player_is_dead() -> bool {
    player_ref().is_dead
}

/// `player->died_from` (bytes up to the NUL).
pub fn player_died_from() -> Vec<u8> {
    let d = &player_ref().died_from;
    d.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect()
}

/// `player->upkeep->resting`
pub fn player_resting() -> i32 {
    // SAFETY: upkeep is allocated with the player.
    unsafe { (*player_ref().upkeep).resting }
}

/// `cmd_get_nrepeats()`
pub fn cmd_get_nrepeats() -> i32 {
    // SAFETY: plain call.
    unsafe { crate::headers::cmd_core_h::cmd_get_nrepeats() }
}

/// `Term_inkey(&ev, wait, take)`: returns the event type and its key.
pub fn term_inkey(wait: bool, take: bool) -> (ui_event_type, keypress) {
    // SAFETY: ui_event is a C union of plain-data structs that all start
    // with the type; Term_inkey zeroes it first, so reading the key
    // variant is always defined.
    unsafe {
        let mut ev: crate::headers::ui_event_h::ui_event = core::mem::zeroed();
        crate::headers::ui_term_h::Term_inkey(&mut ev, wait, take);
        (ev.type_0, ev.key)
    }
}

/// `flush(0, 0, 0)` (ui-input.c): discard pending input.
pub fn flush_input() {
    // SAFETY: the engine's flush ignores its arguments.
    unsafe { crate::headers::ui_input_h::flush(0, core::ptr::null_mut(), core::ptr::null_mut()) }
}

/// The game RNG's simple-mode state (`Rand_quick`, `Rand_value`).
pub fn rand_state() -> (bool, u32) {
    // SAFETY: plain globals.
    unsafe { (crate::headers::z_rand_h::Rand_quick, crate::headers::z_rand_h::Rand_value) }
}

/// Set `Rand_quick` and `Rand_value`.
pub fn set_rand_state(quick: bool, value: u32) {
    // SAFETY: plain globals, engine thread.
    unsafe {
        crate::headers::z_rand_h::Rand_quick = quick;
        crate::headers::z_rand_h::Rand_value = value;
    }
}

/// The `inkey_hack` callback type.
pub type InkeyHack = unsafe extern "C" fn(core::ffi::c_int) -> keypress;

/// Set or clear `inkey_hack` (ui-input.c).
pub fn set_inkey_hack(f: Option<InkeyHack>) {
    // SAFETY: plain global, engine thread.
    unsafe { crate::headers::borg_h::inkey_hack = f }
}

// ---------------------------------------------------------------------------
// Game data lookups

/// Walk the activation list from `activations[1]` and return the index of
/// the first entry whose name equals `name` ignoring case (`my_stricmp`).
pub fn find_activation(name: &str) -> Option<i32> {
    let cname = CString::new(name).expect("no NUL in name");
    // SAFETY: `activations` is loaded at startup; the list is linked
    // through `next` and terminated by NULL; names are C strings.
    unsafe {
        let mut act = crate::headers::object_h::activations.add(1);
        while !act.is_null() {
            if crate::headers::z_util_h::my_stricmp((*act).name, cname.as_ptr()) == 0 {
                return Some((*act).index);
            }
            act = (*act).next;
        }
    }
    None
}
