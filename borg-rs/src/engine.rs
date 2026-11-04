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
