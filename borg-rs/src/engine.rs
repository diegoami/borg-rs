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
