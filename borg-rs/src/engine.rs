//! Safe access to the C game engine: the permanent FFI boundary.
//!
//! The Borg runs inside the engine's single thread, called from
//! `inkey_hack` (or the `^Z` command). The engine globals read here are
//! stable for the duration of such a call.

use crate::headers;

// ---------------------------------------------------------------------------
// The Borg state and the engine's entries into the Borg
//
// All Borg state is one `BorgState` (the C globals of borg/*.c). It is
// allocated on the first entry and never freed (the C globals lived for
// the whole process too), and it is reached only here: the engine calls
// into the Borg through four callbacks (the `inkey_hack` hook in borg.rs,
// the `EVENT_BELL` handler in borg_io.rs, the `EVENT_LEAVE_GAME` handler in
// borg_init.rs, and `do_cmd_borg` below), each of which goes through
// `enter`. `enter` hands the callback a `&mut BorgState`, and Borg code
// passes it down explicitly; no Borg code holds the state any other way.
//
// Nested entry (docs/SELF-REVIEW.md U1). A Borg call into the engine that
// waits for a key (a long `msg()` with `auto_more` off goes through
// `msg_flush` -> `anykey` -> `inkey_ex` -> the hook), rings the bell, or
// shows a message can run the Borg again while the outer Borg call is
// still running and holds its `&mut BorgState`. C has the same nesting (no
// guard in `inkey_ex`), so it is kept. To make it sound, every engine
// wrapper that can get there takes the caller's `&mut BorgState` and
// *lends* it for the duration of the engine call (`lend`): the nested
// entry then works on a reborrow of the caller's reference, which the
// caller cannot use until the engine call returns. A nested entry is
// reported on stderr (the harness keeps `stderr.log` per run). A nested
// entry from an engine call that does not lend the state would alias the
// outer reference; it cannot be made sound, so it panics (abort) after the
// report. The lending wrappers are: `msg`, `event_signal`,
// `event_signal_message_null`, `get_string`, `get_com`, `get_com_ex`,
// `get_quantity`, `target_set_interactive`, `screen_save`, `screen_load`,
// `Menu::select`, `project_from_player`, `do_cmd_wiz_cure_all` and
// `do_cmd_redraw`.

use crate::state::BorgState;

/// The root of the Borg state and the entry bookkeeping.
struct Root {
    /// The one `BorgState` (NULL until the first entry).
    state: *mut BorgState,
    /// How many engine callbacks into the Borg are running.
    depth: u32,
    /// The state lent to the engine call running now (NULL if none).
    lent: *mut BorgState,
}

/// Engine thread only (the Borg never runs on another thread).
static mut ROOT: Root = Root { state: core::ptr::null_mut(), depth: 0, lent: core::ptr::null_mut() };

/// The root, as a raw pointer (no reference to the static is kept).
fn root() -> *mut Root {
    &raw mut ROOT
}

/// Run an engine callback into the Borg with the state (see above).
pub fn enter<R>(f: impl FnOnce(&mut BorgState) -> R) -> R {
    // SAFETY: engine thread only; ROOT is only touched in this function
    // and in `lend`, and no reference to it is kept.
    let (depth, p, saved_lent) = unsafe {
        let root = &mut *root();
        if root.state.is_null() {
            root.state = Box::into_raw(Box::default());
        }
        root.depth += 1;
        let p = if root.depth == 1 { root.state } else { root.lent };
        (root.depth, p, core::mem::replace(&mut root.lent, core::ptr::null_mut()))
    };
    if depth > 1 {
        eprintln!("borg-rs: nested Borg entry (depth {depth})");
        assert!(!p.is_null(), "borg-rs: nested Borg entry from an engine call that does not lend the state");
    }
    // SAFETY: at depth 1 no Borg code is running, so no other reference
    // into the state exists. At depth > 1 `p` is the reference the running
    // Borg code lent to the engine call that led here (`lend`); that code
    // cannot use it until the call returns, which is after `f` returns.
    let r = f(unsafe { &mut *p });
    // SAFETY: as above.
    unsafe {
        let root = &mut *root();
        root.depth -= 1;
        root.lent = saved_lent;
    }
    r
}

/// Run an engine call that may enter the Borg again (see above) with the
/// caller's state lent to that entry.
fn lend<R>(st: &mut BorgState, f: impl FnOnce() -> R) -> R {
    let p: *mut BorgState = st;
    // SAFETY: engine thread only; see `enter`.
    let saved = unsafe { core::mem::replace(&mut (*root()).lent, p) };
    let r = f();
    // SAFETY: as above.
    unsafe { (*root()).lent = saved };
    r
}

/// The engine's call into the Borg commands (`do_cmd_borg`, called by
/// `do_cmd_try_borg` in cmd-misc.c for `^Z`).
#[no_mangle]
pub extern "C" fn do_cmd_borg() {
    enter(crate::borg_commands::do_cmd_borg)
}

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

/// `distance(grid1, grid2)` (cave.c): the game's approximate distance.
pub fn distance(grid1: loc, grid2: loc) -> i32 {
    // SAFETY: pure arithmetic on two values.
    unsafe { headers::cave_h::distance(grid1, grid2) }
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
/// Panics if (x, y) is off the term (C would read outside the screen).
pub fn screen_at(x: i32, y: i32) -> (i32, wchar_t) {
    // SAFETY: `Term` is the active term while the game runs; (x, y) is
    // inside it (checked below), and `scr` rows are allocated for its size.
    unsafe {
        let t = &*crate::headers::ui_term_h::Term;
        assert!((0..t.wid).contains(&x) && (0..t.hgt).contains(&y), "screen position off the term");
        let scr = &*t.scr;
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

/// `msg("%s", text)`. May wait for a key (lends the state).
pub fn msg(st: &mut BorgState, text: &[u8]) {
    let c = CString::new(text).expect("no NUL in message");
    // SAFETY: format string and argument are valid C strings.
    lend(st, || unsafe { crate::headers::message_h::msg(c"%s".as_ptr(), c.as_ptr()) })
}

/// `message_add(text, type)`.
pub fn message_add(text: &[u8], kind: u16) {
    let c = CString::new(text).expect("no NUL in message");
    // SAFETY: c is valid for the call; message_add copies it.
    unsafe { crate::headers::message_h::message_add(c.as_ptr(), kind) }
}

/// `event_signal_message(type, t, NULL)` (runs event handlers: lends the
/// state).
pub fn event_signal_message_null(st: &mut BorgState, kind: game_event_type, t: i32) {
    // SAFETY: NULL message text is allowed by the engine.
    lend(st, || unsafe { crate::headers::game_event_h::event_signal_message(kind, t, core::ptr::null()) })
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

/// `player->full_name` (bytes up to the NUL).
pub fn player_full_name() -> Vec<u8> {
    c_array_bytes(&player_ref().full_name)
}

/// `player_is_shapechanged(player)`.
pub fn player_is_shapechanged() -> bool {
    let p: *const crate::headers::player_h::player = player_ref();
    // SAFETY: p is the live player; the call only reads `p->shape`.
    unsafe { crate::headers::player_util_h::player_is_shapechanged(p) }
}

/// A copy of the game's `savefile` buffer (ui-game.c, `char savefile[1024]`).
pub fn savefile() -> [u8; 1024] {
    // SAFETY: plain copy of a char array global; engine thread.
    let s = unsafe { *core::ptr::addr_of!(crate::headers::ui_game_h::savefile) };
    s.map(|c| c as u8)
}

/// `memcpy(savefile, v, sizeof(savefile))`.
pub fn set_savefile(v: &[u8; 1024]) {
    // SAFETY: plain write of a char array global; engine thread.
    unsafe { *core::ptr::addr_of_mut!(crate::headers::ui_game_h::savefile) = v.map(|c| c as c_char) };
}

/// `path_build(savefile, sizeof(savefile), ANGBAND_DIR_ARCHIVE, leaf)`:
/// builds the path in place (bytes after its NUL are left as they were).
pub fn savefile_path_build_archive(leaf: &[u8]) {
    let leaf = CString::new(leaf).expect("no NUL in file name");
    // SAFETY: savefile has 1024 bytes and path_build writes at most that
    // many; ANGBAND_DIR_ARCHIVE is a C string set up at startup (or NULL,
    // which path_build accepts); leaf is a C string.
    unsafe {
        crate::headers::z_file_h::path_build(
            core::ptr::addr_of_mut!(crate::headers::ui_game_h::savefile) as *mut c_char,
            1024,
            crate::headers::init_h::ANGBAND_DIR_ARCHIVE,
            leaf.as_ptr(),
        );
    }
}

/// `player->upkeep->resting`
pub fn player_resting() -> i32 {
    // SAFETY: upkeep is allocated with the player.
    unsafe { (*player_ref().upkeep).resting }
}

/// `player->upkeep->arena_level`
pub fn player_arena_level() -> bool {
    // SAFETY: upkeep is allocated with the player.
    unsafe { (*player_ref().upkeep).arena_level }
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

/// An object kind (`struct object_kind`).
pub type ObjectKind = crate::headers::object_h::object_kind;

/// `k_info[0 .. z_info->k_max]`, the object kinds.
pub fn k_info() -> &'static [ObjectKind] {
    let n = usize::from(z_info().k_max);
    // SAFETY: k_info is allocated with z_info->k_max entries when the game
    // data is loaded, before any Borg code runs, and lives for the game.
    unsafe { std::slice::from_raw_parts(crate::headers::object_h::k_info, n) }
}

/// `k_info[kind].base->max_stack`: how many objects of this kind fit in
/// one stack (panics past `k_max` or for a kind without a base, where C
/// would read out of bounds or through NULL).
pub fn kind_max_stack(kind: usize) -> i32 {
    let base = k_info()[kind].base;
    assert!(!base.is_null(), "k_info[{}] has no base", kind);
    // SAFETY: a kind's base points into kb_info, which is allocated with
    // the game data before any Borg code runs and lives for the game
    // (checked non-NULL above).
    unsafe { (*base).max_stack }
}

/// An object base (`struct object_base`).
pub type ObjectBase = crate::headers::object_h::object_base;

/// `kb_info[0 .. TV_MAX]`, the object bases (indexed by tval).
pub fn kb_info() -> &'static [ObjectBase] {
    let n = crate::headers::obj_properties_h::TV_MAX as usize;
    // SAFETY: kb_info is allocated with TV_MAX entries (obj-init.c) when
    // the game data is loaded, before any Borg code runs, and lives for the
    // game.
    unsafe { std::slice::from_raw_parts(crate::headers::object_h::kb_info, n) }
}

/// `tval_find_idx(name)`: the tval with this name, or -1.
pub fn tval_find_idx(name: &str) -> i32 {
    let c = CString::new(name).expect("no NUL in name");
    // SAFETY: c is a valid C string for the call.
    unsafe { crate::headers::obj_tval_h::tval_find_idx(c.as_ptr()) }
}

/// `tval_find_name(tval)`: the tval's name ("unknown" if there is none).
pub fn tval_find_name(tval: i32) -> &'static [u8] {
    // SAFETY: the result points into the static tval name table (or is a
    // string literal), never NULL.
    unsafe {
        std::ffi::CStr::from_ptr(crate::headers::obj_tval_h::tval_find_name(tval)).to_bytes()
    }
}

/// `lookup_sval(tval, name)`: the sval of the kind with this name, or -1.
pub fn lookup_sval(tval: i32, name: &str) -> i32 {
    lookup_sval_bytes(tval, name.as_bytes())
}

/// `lookup_sval(tval, name)` for a name that may not be UTF-8 (borg.txt
/// bytes).
pub fn lookup_sval_bytes(tval: i32, name: &[u8]) -> i32 {
    let c = CString::new(name).expect("no NUL in name");
    // SAFETY: c is a valid C string; lookup_sval only reads it and k_info.
    unsafe { crate::headers::obj_util_h::lookup_sval(tval, c.as_ptr()) }
}

/// A monster race (`struct monster_race`).
pub type MonsterRace = crate::headers::monster_h::monster_race;

/// `r_info[0 .. z_info->r_max]`, the monster races.
pub fn r_info() -> &'static [MonsterRace] {
    let n = usize::from(z_info().r_max);
    // SAFETY: r_info is allocated with z_info->r_max entries when the game
    // data is loaded, before any Borg code runs, and lives for the game.
    unsafe { std::slice::from_raw_parts(crate::headers::monster_h::r_info, n) }
}

/// `r_info[r_idx].name` (bytes up to the NUL).
pub fn race_name(r_idx: usize) -> &'static [u8] {
    race_name_cstr(r_idx).to_bytes()
}

/// `r_info[r_idx].name` as a C string.
pub fn race_name_cstr(r_idx: usize) -> &'static CStr {
    let name = r_info()[r_idx].name;
    assert!(!name.is_null(), "race {r_idx} has no name");
    // SAFETY: a race's name is a C string allocated when the game data is
    // loaded and kept for the game; checked non-null.
    unsafe { CStr::from_ptr(name) }
}

/// `r_info[r_idx].plural` (bytes up to the NUL), `None` for NULL.
pub fn race_plural(r_idx: usize) -> Option<&'static [u8]> {
    let p = r_info()[r_idx].plural;
    // SAFETY: a non-NULL plural is a C string allocated when the game data
    // is loaded and kept for the game.
    (!p.is_null()).then(|| unsafe { CStr::from_ptr(p).to_bytes() })
}

/// `r_info[r_idx].base->name` (bytes up to the NUL).
pub fn race_base_name(r_idx: usize) -> &'static [u8] {
    let base = r_info()[r_idx].base;
    assert!(!base.is_null(), "race {r_idx} has no base");
    // SAFETY: a race's base points into rb_info, loaded with the game data
    // and kept for the game (checked non-null); its name is checked too.
    unsafe {
        let name = (*base).name;
        assert!(!name.is_null(), "race {r_idx} base has no name");
        CStr::from_ptr(name).to_bytes()
    }
}

/// `k_info[kidx].name` (bytes up to the NUL).
pub fn kind_name(kidx: usize) -> &'static [u8] {
    let name = k_info()[kidx].name;
    assert!(!name.is_null(), "kind {kidx} has no name");
    // SAFETY: a kind's name is a C string allocated when the game data is
    // loaded and kept for the game; checked non-null.
    unsafe { CStr::from_ptr(name).to_bytes() }
}

/// The fields of a game monster (`struct monster`) the Borg cheats.
#[derive(Clone, Copy, Debug)]
pub struct MonsterView {
    pub midx: i32,
    pub grid: loc,
    pub hp: i16,
    pub maxhp: i16,
    pub m_timed: [i16; crate::headers::list_mon_timed_h::MON_TMD_MAX as usize],
    pub mspeed: u8,
    /// `race->ridx`, `None` for a NULL race (an unused slot).
    pub ridx: Option<u32>,
    /// `attr`: the colour the monster is drawn in
    pub attr: u8,
    /// `mimicked_obj != NULL`: the monster mimics an object
    pub mimics: bool,
}

/// Copy the fields of a monster.
///
/// SAFETY (caller): `m` points at a `struct monster` of the current level.
unsafe fn monster_view(m: *const crate::headers::monster_h::monster) -> MonsterView {
    let m = &*m;
    MonsterView {
        midx: m.midx,
        grid: m.grid,
        hp: m.hp,
        maxhp: m.maxhp,
        m_timed: m.m_timed,
        mspeed: m.mspeed,
        ridx: (!m.race.is_null()).then(|| (*m.race).ridx),
        attr: m.attr,
        mimics: !m.mimicked_obj.is_null(),
    }
}

/// `square_monster(cave, loc(x, y))`, `None` for no monster. The grid must
/// be in bounds (the engine asserts it, as for C callers).
pub fn square_monster(x: i32, y: i32) -> Option<MonsterView> {
    // SAFETY: `cave` is the current level; the result is NULL or a live
    // monster of it, whose race (if any) points into r_info.
    unsafe {
        let m = crate::headers::cave_h::square_monster(crate::headers::cave_h::cave, loc { x, y });
        (!m.is_null()).then(|| monster_view(m))
    }
}

/// `cave->monsters[m_idx]` (any slot, live or not).
pub fn cave_monster(m_idx: i32) -> MonsterView {
    let max = i32::from(z_info().level_monster_max);
    assert!((0..max).contains(&m_idx), "monster index {m_idx} out of range");
    // SAFETY: `cave->monsters` has z_info->level_monster_max entries
    // (cave_new zeroes them); the index is checked; a slot's race is NULL
    // or a pointer into r_info.
    unsafe {
        let c = &*crate::headers::cave_h::cave;
        monster_view(c.monsters.add(m_idx as usize))
    }
}

/// `rf_has(r_info[r_idx].flags, flag)`.
pub fn race_flag_has(r_idx: usize, flag: u32) -> bool {
    let flags = &r_info()[r_idx].flags;
    // SAFETY: flags is the race's RF_SIZE-byte flag array; flag_has only
    // reads within the size given.
    unsafe { crate::headers::z_bitflag_h::flag_has(flags.as_ptr(), flags.len(), flag as i32) }
}

// ---------------------------------------------------------------------------
// Player class magic (books and spells)

use core::ffi::CStr;

fn class_magic_ref() -> &'static crate::headers::player_h::class_magic {
    // SAFETY: `player->class` points into the class list loaded at startup
    // (it is set at birth and lives for the game).
    unsafe { &(*player_ref().class).magic }
}

/// `player->class->cidx`
pub fn class_cidx() -> u32 {
    // SAFETY: as in class_magic_ref().
    unsafe { (*player_ref().class).cidx }
}

/// `player->class->magic.num_books`
pub fn class_num_books() -> i32 {
    class_magic_ref().num_books
}

/// `player->class->magic.total_spells`
pub fn class_total_spells() -> i32 {
    class_magic_ref().total_spells
}

/// The plain data of a `struct class_book`.
#[derive(Clone, Copy, Debug)]
pub struct ClassBook {
    pub tval: i32,
    pub sval: i32,
    pub dungeon: bool,
    pub num_spells: i32,
}

fn class_book_ref(book: i32) -> &'static crate::headers::player_h::class_book {
    let m = class_magic_ref();
    assert!(book >= 0 && book < m.num_books, "class book index out of range");
    // SAFETY: `books` has `num_books` entries (checked above).
    unsafe { &*m.books.add(book as usize) }
}

/// `player->class->magic.books[book]` (0 <= book < num_books).
pub fn class_book(book: i32) -> ClassBook {
    let b = class_book_ref(book);
    ClassBook { tval: b.tval, sval: b.sval, dungeon: b.dungeon, num_spells: b.num_spells }
}

/// The plain data of a `struct class_spell`.
#[derive(Clone, Copy, Debug)]
pub struct ClassSpell {
    /// `name` (bytes up to the NUL)
    pub name: &'static [u8],
    pub sidx: i32,
    pub bidx: i32,
    pub slevel: i32,
    pub smana: i32,
    pub sfail: i32,
}

fn class_spell_data(s: &'static crate::headers::player_h::class_spell) -> ClassSpell {
    // SAFETY: spell names are C strings loaded from class.txt that live
    // for the game.
    let name = unsafe { CStr::from_ptr(s.name).to_bytes() };
    ClassSpell {
        name,
        sidx: s.sidx,
        bidx: s.bidx,
        slevel: s.slevel,
        smana: s.smana,
        sfail: s.sfail,
    }
}

fn class_book_spell_ref(book: i32, n: i32) -> &'static crate::headers::player_h::class_spell {
    let b = class_book_ref(book);
    assert!(n >= 0 && n < b.num_spells, "book spell index out of range");
    // SAFETY: `spells` has `num_spells` entries (checked above).
    unsafe { &*b.spells.add(n as usize) }
}

/// `player->class->magic.books[book].spells[n]`.
pub fn class_book_spell(book: i32, n: i32) -> ClassSpell {
    class_spell_data(class_book_spell_ref(book, n))
}

/// `player->class->magic.books[book].spells[n].realm->stat`.
pub fn class_book_spell_realm_stat(book: i32, n: i32) -> i32 {
    let s = class_book_spell_ref(book, n);
    assert!(!s.realm.is_null(), "class spell without a realm");
    // SAFETY: realm points into the static realm list (checked non-NULL).
    unsafe { (*s.realm).stat }
}

fn spell_by_index_ref(index: i32) -> &'static crate::headers::player_h::class_spell {
    // SAFETY: spell_by_index only reads the player's class data and
    // returns NULL or a pointer into it.
    let p = unsafe {
        crate::headers::player_spell_h::spell_by_index(crate::headers::player_h::player, index)
    };
    assert!(!p.is_null(), "spell_by_index: no such spell");
    // SAFETY: non-NULL pointer into the class data, which lives for the game.
    unsafe { &*p }
}

/// `spell_by_index(player, index)` (panics where C would get NULL).
pub fn spell_by_index(index: i32) -> ClassSpell {
    class_spell_data(spell_by_index_ref(index))
}

/// The effect indices of `spell_by_index(player, index)->effect`, in list
/// order (following `next` until NULL).
pub fn spell_effect_indices(index: i32) -> Vec<u16> {
    let s = spell_by_index_ref(index);
    let mut out = Vec::new();
    let mut eff = s.effect;
    while !eff.is_null() {
        // SAFETY: effects form a NULL-terminated list loaded at startup.
        unsafe {
            out.push((*eff).index);
            eff = (*eff).next;
        }
    }
    out
}

/// `spell_okay_to_cast(player, index)`.
pub fn spell_okay_to_cast(index: i32) -> bool {
    // SAFETY: reads the player's spell flags and class data only.
    unsafe {
        crate::headers::player_spell_h::spell_okay_to_cast(crate::headers::player_h::player, index)
    }
}

/// `player->spell_flags[sidx]`.
pub fn player_spell_flags(sidx: i32) -> u8 {
    assert!(sidx >= 0 && sidx < class_total_spells(), "spell index out of range");
    // SAFETY: spell_flags is allocated with total_spells entries at birth.
    unsafe { *player_ref().spell_flags.add(sidx as usize) }
}

/// `player_has(player, flag)`: a player flag in `player->state.pflags`.
pub fn player_has(flag: u32) -> bool {
    let f = &player_ref().state.pflags;
    // SAFETY: flag_has reads within the size given (the array length).
    unsafe { crate::headers::z_bitflag_h::flag_has(f.as_ptr(), f.len(), flag as i32) }
}

/// `no_light(player)`: the player's grid is not lit.
pub fn no_light() -> bool {
    // SAFETY: reads the player and the current level.
    unsafe { crate::headers::cave_h::no_light(crate::headers::player_h::player) }
}

/// `all_letters_nohjkl[i]` (ui-menu.c): the menu letters without h/j/k/l
/// (49 bytes with the NUL; panics past it, where C would read out of bounds).
pub fn all_letters_nohjkl(i: i32) -> u8 {
    // SAFETY: a constant array defined in ui-menu.c, declared with its C
    // length (`[c_char; 49]`); nothing writes it.
    unsafe { crate::headers::ui_menu_h::all_letters_nohjkl[i as usize] as u8 }
}

/// `all_letters[i]` (ui-menu.c): the menu letters a-z, A-Z (53 bytes with
/// the NUL; panics past it, where C would read out of bounds).
pub fn all_letters(i: usize) -> u8 {
    // SAFETY: a constant array defined in ui-menu.c, declared with its C
    // length (`[c_char; 53]`); nothing writes it.
    unsafe { crate::headers::ui_menu_h::all_letters[i] as u8 }
}

/// `randint0(m)` (z-rand.h): `(int32_t) Rand_div(m)` from the game RNG
/// (the Borg's own RNG while it thinks, see `borg::generate_keypress`).
pub fn randint0(m: u32) -> i32 {
    // SAFETY: plain call; the RNG state is engine-thread global.
    unsafe { crate::headers::z_rand_h::Rand_div(m) as i32 }
}

/// `player->race->ridx`
pub fn race_ridx() -> u32 {
    // SAFETY: `player->race` points into the race list loaded at startup
    // (set at birth, lives for the game).
    unsafe { (*player_ref().race).ridx }
}

/// `player->noscore`
pub fn player_noscore() -> u16 {
    player_ref().noscore
}

/// `player->noscore = v`
pub fn set_player_noscore(v: u16) {
    // SAFETY: `player` is allocated at startup; plain field store on the
    // engine thread, no reference to it is held across the call.
    unsafe { (*crate::headers::player_h::player).noscore = v }
}

/// `do_cmd_wiz_cure_all(NULL)` (cmd-wizard.c): heal the player fully
/// (shows messages: lends the state).
pub fn do_cmd_wiz_cure_all(st: &mut BorgState) {
    // SAFETY: the command argument is unused by this wizard command (the
    // C Borg passes NULL); it only changes the player on the engine thread.
    lend(st, || unsafe { crate::headers::cmds_h::do_cmd_wiz_cure_all(core::ptr::null_mut()) })
}

/// `option_set(name, val)` (option.c): returns whether the option exists.
pub fn option_set(name: &CStr, val: bool) -> bool {
    // SAFETY: option_set only reads the NUL-terminated name.
    unsafe { crate::headers::option_h::option_set(name.as_ptr(), i32::from(val)) }
}

// ---------------------------------------------------------------------------
// Player class

fn player_class_ref() -> &'static crate::headers::player_h::player_class {
    // SAFETY: the player's class is set at birth, before the Borg runs,
    // and points into the static class list.
    unsafe { &*player_ref().class }
}

/// The `cidx` of the first class in the `classes` list whose name equals
/// `name` ignoring case (`my_stricmp`), or -1.
pub fn find_class(name: &[u8]) -> i32 {
    let cname = CString::new(name).expect("no NUL in name");
    // SAFETY: `classes` is loaded at startup, linked through `next` and
    // terminated by NULL; names are C strings.
    unsafe {
        let mut c = crate::headers::player_h::classes;
        while !c.is_null() {
            if crate::headers::z_util_h::my_stricmp((*c).name, cname.as_ptr()) == 0 {
                return (*c).cidx as i32;
            }
            c = (*c).next;
        }
    }
    -1
}

/// `player->class->magic.spell_weight`
pub fn class_spell_weight() -> i32 {
    player_class_ref().magic.spell_weight
}

/// `player->class->magic.books[book].dungeon`
pub fn class_book_dungeon(book: i32) -> bool {
    // Panics on a book the class does not have (C does not check; callers
    // only pass books the Borg holds, `borg.amt_book[book] != 0`).
    class_book_ref(book).dungeon
}

/// `spell_chance(spell_index)`: the player's fail rate for a spell.
pub fn spell_chance(spell_index: i32) -> i16 {
    // SAFETY: plain call; the engine checks the index itself.
    unsafe { crate::headers::player_spell_h::spell_chance(spell_index) }
}

// ---------------------------------------------------------------------------
// Files

/// An engine file (`ang_file *`) opened by someone else.
#[derive(Clone, Copy)]
pub struct AngFile(*mut crate::headers::z_file_h::ang_file);

impl AngFile {
}

/// `file_getl(f, buf, len)`: read a line into `buf` (at most `len - 1`
/// bytes and a NUL; `\r`, `\n` and `\r\n` end a line, tabs become
/// spaces). Returns false at the end of the file.
pub fn file_getl(f: AngFile, buf: &mut [u8], len: usize) -> bool {
    assert!(len >= 1 && len <= buf.len());
    // SAFETY: f is open (AngFile::from_raw); file_getl writes at most
    // `len` bytes into buf.
    unsafe { crate::headers::z_file_h::file_getl(f.0, buf.as_mut_ptr() as *mut c_char, len) }
}

// ---------------------------------------------------------------------------
// Flags, player flags and race resistances

/// `flag_has(flags, size, flag)` (z-bitflag.c) on a whole flag array; the
/// function behind `of_has`, `kf_has`, `rf_has` and friends.
pub fn flag_has(flags: &[u8], flag: i32) -> bool {
    // SAFETY: flag_has reads one byte below the size given (it asserts the
    // flag's offset is in range), and the size is the slice length.
    unsafe { crate::headers::z_bitflag_h::flag_has(flags.as_ptr(), flags.len(), flag) }
}

/// `OF_SIZE`: bytes in an object flag set.
pub const OF_SIZE: usize = 5;

/// `player_flags(player, f)`: the player's race and class object flags.
pub fn player_flags() -> [u8; OF_SIZE] {
    let mut f = [0u8; OF_SIZE];
    // SAFETY: `player` lives for the game; player_flags writes exactly
    // OF_SIZE bytes into f (a copy of the race's flags, then unions).
    unsafe {
        crate::headers::player_h::player_flags(crate::headers::player_h::player, f.as_mut_ptr())
    };
    f
}

/// `player->race->el_info[elem].res_level`.
pub fn player_race_res_level(elem: u32) -> i16 {
    // SAFETY: the race is set at birth and points into the race table,
    // which lives for the game.
    let race = unsafe { &*player_ref().race };
    race.el_info[elem as usize].res_level
}

/// `obj_kind_can_browse(&k_info[kind])`: the player's class can browse
/// books of this kind.
pub fn obj_kind_can_browse(kind: usize) -> bool {
    let k: *const ObjectKind = &k_info()[kind];
    // SAFETY: k points into k_info (index checked above); the callee only
    // reads the kind and the player's class.
    unsafe { crate::headers::obj_util_h::obj_kind_can_browse(k) }
}

// ---------------------------------------------------------------------------
// Writable files, paths and the clock (borg-log.c)

/// How `file_open()` opens a file for writing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WriteMode {
    /// `MODE_WRITE`: create or truncate.
    Write,
    /// `MODE_APPEND`: create or append.
    Append,
}

/// An engine file opened for writing by `file_open()` (`ang_file *`). Not
/// `Copy`: `close()` consumes it. Dropping it without closing leaks it.
pub struct OpenFile(*mut crate::headers::z_file_h::ang_file);

/// `file_open(path, mode, FTYPE_TEXT)`; `None` where C gets NULL.
pub fn file_open(path: &[u8], mode: WriteMode) -> Option<OpenFile> {
    use crate::headers::z_file_h as z;
    let c = CString::new(path).expect("no NUL in path");
    let m = match mode {
        WriteMode::Write => z::MODE_WRITE,
        WriteMode::Append => z::MODE_APPEND,
    };
    // SAFETY: c is a valid C string for the call; the result is NULL or a
    // file the engine allocated.
    let f = unsafe { z::file_open(c.as_ptr(), m, z::FTYPE_TEXT) };
    (!f.is_null()).then_some(OpenFile(f))
}

impl OpenFile {
    /// `file_put(f, s)` (no NUL in `s`).
    pub fn put(&mut self, s: &[u8]) -> bool {
        let c = CString::new(s).expect("no NUL in file text");
        // SAFETY: self.0 is open (only close() ends it, by value); c is a
        // valid C string.
        unsafe { crate::headers::z_file_h::file_put(self.0, c.as_ptr()) }
    }

    /// `file_close(f)`.
    pub fn close(self) -> bool {
        // SAFETY: self.0 is open and is not used again (self is consumed).
        unsafe { crate::headers::z_file_h::file_close(self.0) }
    }
}

/// `path_build(buf, len, ANGBAND_DIR_ARCHIVE, leaf)`: the path (bytes up
/// to the NUL) as built in a buffer of `len` bytes.
pub fn path_build_archive(len: usize, leaf: &[u8]) -> Vec<u8> {
    // SAFETY: plain read of a pointer global set up at startup.
    path_build_in(unsafe { crate::headers::init_h::ANGBAND_DIR_ARCHIVE }, len, leaf)
}

/// `path_build(buf, len, ANGBAND_DIR_USER, leaf)`: the path (bytes up to
/// the NUL) as built in a buffer of `len` bytes.
pub fn path_build_user(len: usize, leaf: &[u8]) -> Vec<u8> {
    // SAFETY: plain read of a pointer global set up at startup.
    path_build_in(unsafe { crate::headers::init_h::ANGBAND_DIR_USER }, len, leaf)
}

/// `path_build(buf, len, dir, leaf)` for one of the `ANGBAND_DIR_*` globals.
fn path_build_in(dir: *mut c_char, len: usize, leaf: &[u8]) -> Vec<u8> {
    assert!(len >= 1);
    let leaf = CString::new(leaf).expect("no NUL in file name");
    let mut buf = vec![0u8; len];
    // SAFETY: buf has len bytes; dir is an `ANGBAND_DIR_*` C string set up
    // at startup (or NULL, which path_build accepts); leaf is a C string.
    unsafe {
        crate::headers::z_file_h::path_build(buf.as_mut_ptr() as *mut c_char, len, dir, leaf.as_ptr());
    }
    let n = buf.iter().position(|&b| b == 0).unwrap_or(len);
    buf.truncate(n);
    buf
}

/// An engine file opened for reading by `file_open(path, MODE_READ, -1)`
/// (`ang_file *`). Not `Copy`: `close()` consumes it. Dropping it without
/// closing leaks it.
pub struct ReadFile(*mut crate::headers::z_file_h::ang_file);

/// `file_open(path, MODE_READ, -1)`; `None` where C gets NULL.
pub fn file_open_read(path: &[u8]) -> Option<ReadFile> {
    use crate::headers::z_file_h as z;
    let c = CString::new(path).expect("no NUL in path");
    // SAFETY: c is a valid C string for the call; the file type is not
    // used for reading (C passes -1); the result is NULL or a file the
    // engine allocated.
    let f = unsafe { z::file_open(c.as_ptr(), z::MODE_READ, u32::MAX) };
    (!f.is_null()).then_some(ReadFile(f))
}

impl ReadFile {
    /// `file_getl(f, buf, len)`, as [`file_getl`].
    pub fn getl(&mut self, buf: &mut [u8], len: usize) -> bool {
        // AngFile contract: self.0 is open (only close() ends it, by value)
        // and the handle does not outlive this call.
        file_getl(AngFile(self.0), buf, len)
    }

    /// `file_close(f)`.
    pub fn close(self) -> bool {
        // SAFETY: self.0 is open and is not used again (self is consumed).
        unsafe { crate::headers::z_file_h::file_close(self.0) }
    }
}

/// `time(NULL)`.
pub fn time_now() -> i64 {
    // SAFETY: time() accepts a NULL out-pointer.
    unsafe { crate::headers::time_h::time(core::ptr::null_mut()) }
}

/// `strftime(buf, max, fmt, localtime(&t))`: the bytes written (empty
/// where strftime returns 0).
pub fn strftime_local(t: i64, max: usize, fmt: &CStr) -> Vec<u8> {
    let mut buf = vec![0u8; max];
    // SAFETY: localtime reads t and returns its static struct (or NULL,
    // checked); strftime writes at most max bytes into buf.
    let n = unsafe {
        let tm = crate::headers::time_h::localtime(&t);
        assert!(!tm.is_null(), "localtime failed");
        crate::headers::time_h::strftime(buf.as_mut_ptr() as *mut c_char, max, fmt.as_ptr(), tm)
    };
    buf.truncate(n);
    buf
}

/// `quit_fmt("%s", text)`: end the game with a message. Never returns.
pub fn quit(text: &[u8]) -> ! {
    let c = CString::new(text).expect("no NUL in message");
    // SAFETY: format string and argument are valid C strings.
    unsafe { crate::headers::z_form_h::quit_fmt(c"%s".as_ptr(), c.as_ptr()) }
    unreachable!("quit() returned")
}

/// `get_string(prompt, buf, len)`: let the player edit the NUL-terminated
/// text in `buf` (at most `len` bytes, NUL included). False on escape.
/// Waits for keys (lends the state).
pub fn get_string(st: &mut BorgState, prompt: &CStr, buf: &mut [u8], len: usize) -> bool {
    assert!(len >= 1 && len <= buf.len() && buf.contains(&0));
    // SAFETY: buf is NUL-terminated and has at least len bytes; get_string
    // writes at most len bytes.
    lend(st, || unsafe {
        crate::headers::game_input_h::get_string(
            prompt.as_ptr(),
            buf.as_mut_ptr() as *mut c_char,
            len,
        )
    })
}

// ---------------------------------------------------------------------------
// Player summary, timed effects, objects (borg-log.c)

/// The bytes of a C string from the game data ("" for NULL, as `strnfmt`
/// prints it).
fn c_bytes(p: *const c_char) -> &'static [u8] {
    if p.is_null() {
        return b"";
    }
    // SAFETY: non-NULL C strings passed here are game data (names) that
    // live for the game.
    unsafe { CStr::from_ptr(p).to_bytes() }
}

/// The bytes of a C char array up to its NUL.
fn c_array_bytes(a: &[c_char]) -> Vec<u8> {
    a.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect()
}

/// The player fields that the death log and map dump print.
#[derive(Clone, Debug)]
pub struct PlayerSummary {
    /// `player->full_name`
    pub full_name: Vec<u8>,
    /// `player->race->name` (NULL reads as "", as in `strnfmt`)
    pub race_name: &'static [u8],
    /// `player->class->name`
    pub class_name: &'static [u8],
    pub lev: i16,
    pub max_lev: i16,
    pub max_exp: i32,
    pub max_depth: i16,
    pub au: i32,
    pub depth: i16,
    /// `player->died_from`
    pub died_from: Vec<u8>,
    pub word_recall: i16,
}

/// Read the player's summary fields.
pub fn player_summary() -> PlayerSummary {
    let p = player_ref();
    // SAFETY: race and class point into the static race/class lists (set
    // at birth, live for the game).
    let (race, class) = unsafe { ((*p.race).name, (*p.class).name) };
    PlayerSummary {
        full_name: c_array_bytes(&p.full_name),
        race_name: c_bytes(race),
        class_name: c_bytes(class),
        lev: p.lev,
        max_lev: p.max_lev,
        max_exp: p.max_exp,
        max_depth: p.max_depth,
        au: p.au,
        depth: p.depth,
        died_from: c_array_bytes(&p.died_from),
        word_recall: p.word_recall,
    }
}

/// `player->timed[idx]` (idx < TMD_MAX).
pub fn player_timed(idx: u32) -> i16 {
    assert!(idx < crate::headers::player_timed_h::TMD_MAX, "timed index out of range");
    // SAFETY: timed is allocated with TMD_MAX entries with the player.
    unsafe { *player_ref().timed.add(idx as usize) }
}

/// `player_of_has(player, flag)`: an object flag on the player.
pub fn player_of_has(flag: u32) -> bool {
    // SAFETY: reads the player's state only.
    unsafe {
        crate::headers::player_util_h::player_of_has(crate::headers::player_h::player, flag as i32)
    }
}

/// The game turn (`turn`).
pub fn game_turn() -> i32 {
    // SAFETY: plain global, engine thread.
    unsafe { crate::headers::game_world_h::turn }
}

/// `object_desc(buf, 80, obj, mode, player)` (bytes up to the NUL).
fn object_desc80(obj: *const crate::headers::object_h::object, mode: u32) -> Vec<u8> {
    let mut buf = [0u8; 80];
    // SAFETY: obj is NULL or a live object of the player or a store
    // (callers); object_desc writes at most 80 bytes, NUL included.
    unsafe {
        crate::headers::obj_desc_h::object_desc(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            obj,
            mode,
            crate::headers::player_h::player,
        );
    }
    let n = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    buf[..n].to_vec()
}

/// `player->body.count`: the number of equipment slots.
pub fn body_count() -> u16 {
    player_ref().body.count
}

/// `object_desc()` of `player->body.slots[i].obj` (NULL: "(nothing)").
pub fn equip_desc(i: usize, mode: u32) -> Vec<u8> {
    let body = &player_ref().body;
    assert!(i < usize::from(body.count), "equipment slot out of range");
    // SAFETY: slots has body.count entries (checked).
    let obj = unsafe { (*body.slots.add(i)).obj };
    object_desc80(obj, mode)
}

/// `object_desc()` of `player->upkeep->quiver[i]` (i < quiver_size).
pub fn quiver_desc(i: usize, mode: u32) -> Vec<u8> {
    assert!(i < usize::from(z_info().quiver_size), "quiver slot out of range");
    // SAFETY: the quiver is allocated with z_info->quiver_size entries
    // with the player's upkeep.
    let obj = unsafe { *(*player_ref().upkeep).quiver.add(i) };
    object_desc80(obj, mode)
}

/// `store_stock_list(&stores[store], list, z_info->store_inven_max)`, then
/// `object_desc()` of each list entry, in order.
pub fn store_stock_descs(store: usize, mode: u32) -> Vec<Vec<u8>> {
    let n = usize::from(z_info().store_inven_max);
    assert!(store < usize::from(z_info().store_max), "store index out of range");
    let mut list: Vec<*mut crate::headers::object_h::object> = vec![core::ptr::null_mut(); n];
    // SAFETY: stores has store_max entries (checked); list has room for
    // the n entries store_stock_list fills.
    unsafe {
        crate::headers::store_h::store_stock_list(
            crate::headers::store_h::stores.add(store),
            list.as_mut_ptr(),
            n as i32,
        );
    }
    list.iter().map(|&o| object_desc80(o, mode)).collect()
}

// ---------------------------------------------------------------------------
// Store stock (borg-store.c)

/// An object in a store's stock (an entry of `store_stock_list`).
///
/// Only valid while the store's stock does not change, i.e. within the
/// Borg call that got it; callers do not keep it.
#[derive(Clone, Copy)]
pub struct StoreObject(*const crate::headers::object_h::object);

/// `store_stock_list(&stores[store], list, z_info->store_inven_max)`: the
/// entries before the first NULL, in order.
pub fn store_stock(store: usize) -> Vec<StoreObject> {
    let n = usize::from(z_info().store_inven_max);
    assert!(store < usize::from(z_info().store_max), "store index out of range");
    let mut list: Vec<*mut crate::headers::object_h::object> = vec![core::ptr::null_mut(); n];
    // SAFETY: stores has store_max entries (checked); list has room for
    // the n entries store_stock_list fills.
    unsafe {
        crate::headers::store_h::store_stock_list(
            crate::headers::store_h::stores.add(store),
            list.as_mut_ptr(),
            n as i32,
        );
    }
    list.iter().take_while(|o| !o.is_null()).map(|&o| StoreObject(o)).collect()
}

impl StoreObject {

    /// `object_desc(buf, 120, obj, mode, player)` (bytes up to the NUL).
    pub fn desc120(self, mode: u32) -> Vec<u8> {
        let mut buf = [0u8; 120];
        // SAFETY: self is a live store object (see the type); object_desc
        // writes at most 120 bytes, NUL included.
        unsafe {
            crate::headers::obj_desc_h::object_desc(
                buf.as_mut_ptr() as *mut c_char,
                buf.len(),
                self.0,
                mode,
                crate::headers::player_h::player,
            );
        }
        let n = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        buf[..n].to_vec()
    }

    /// `tval_can_have_charges(obj)`
    pub fn can_have_charges(self) -> bool {
        // SAFETY: self is a live store object; the call only reads it.
        unsafe { crate::headers::obj_tval_h::tval_can_have_charges(self.0) }
    }

    /// `object_value_real(obj, qty)`
    pub fn value_real(self, qty: i32) -> i32 {
        // SAFETY: self is a live store object; the call only reads it.
        unsafe { crate::headers::obj_power_h::object_value_real(self.0, qty) }
    }

    /// `object_value(obj, qty)`
    pub fn value(self, qty: i32) -> i32 {
        // SAFETY: self is a live store object; the call only reads it.
        unsafe { crate::headers::obj_power_h::object_value(self.0, qty) }
    }
}

/// `stores[store].owner->max_cost`
pub fn store_owner_max_cost(store: usize) -> i32 {
    assert!(store < usize::from(z_info().store_max), "store index out of range");
    // SAFETY: stores has store_max entries (checked), each with an owner
    // set when the stores are created.
    unsafe {
        let owner = (*crate::headers::store_h::stores.add(store)).owner;
        assert!(!owner.is_null(), "store without owner");
        (*owner).max_cost
    }
}

/// `f_info[feat].shopnum`
pub fn feat_shopnum(feat: usize) -> u8 {
    assert!(feat < crate::headers::cave_h::FEAT_MAX as usize, "feature out of range");
    // SAFETY: f_info is allocated with FEAT_MAX entries when the game data
    // is loaded (index checked).
    unsafe { (*crate::headers::cave_h::f_info.add(feat)).shopnum }
}

/// `f_info[stores[store].feat].name` (bytes up to the NUL).
pub fn store_feat_name(store: usize) -> Vec<u8> {
    assert!(store < usize::from(z_info().store_max), "store index out of range");
    // SAFETY: stores has store_max entries (checked); each store's feature
    // is a valid f_info index whose name is a NUL-terminated string set
    // when the game data is loaded.
    unsafe {
        let feat = (*crate::headers::store_h::stores.add(store)).feat;
        assert!(
            (0..crate::headers::cave_h::FEAT_MAX as i32).contains(&feat),
            "feature out of range"
        );
        let name = (*crate::headers::cave_h::f_info.add(feat as usize)).name;
        assert!(!name.is_null(), "feature without a name");
        std::ffi::CStr::from_ptr(name).to_bytes().to_vec()
    }
}

/// `square_shopnum(cave, player->grid)`: the store at the player's grid,
/// or -1.
pub fn player_square_shopnum() -> i32 {
    // SAFETY: `cave` is the current level and the player's grid is inside
    // it; the call only reads the square.
    unsafe { crate::headers::cave_h::square_shopnum(headers::cave_h::cave, player_ref().grid) }
}

/// `player->opts.opt[opt]` (the `OPT(player, ...)` macro).
pub fn player_opt(opt: u32) -> bool {
    player_ref().opts.opt[opt as usize]
}

// ---------------------------------------------------------------------------
// Map and messages (borg-log.c)

/// `square_monster(cave, loc(x, y))->midx`, or `None` for no monster.
pub fn square_monster_midx(x: i32, y: i32) -> Option<i32> {
    // SAFETY: `cave` is the current level; callers stay inside the
    // dungeon bounds; the result is NULL or a live monster.
    unsafe {
        let m = crate::headers::cave_h::square_monster(crate::headers::cave_h::cave, loc { x, y });
        (!m.is_null()).then(|| (*m).midx)
    }
}

/// `f_info[feat].d_char`.
pub fn feat_d_char(feat: usize) -> wchar_t {
    assert!(feat < crate::headers::cave_h::FEAT_MAX as usize, "feature index out of range");
    // SAFETY: f_info is allocated with FEAT_MAX entries when the game data
    // is loaded and lives for the game (index checked).
    unsafe { (*crate::headers::cave_h::f_info.add(feat)).d_char }
}

/// `messages_num()`
pub fn messages_num() -> u16 {
    // SAFETY: plain call.
    unsafe { crate::headers::message_h::messages_num() }
}

/// `message_str(age)` (bytes up to the NUL; "" for no such message).
pub fn message_str(age: u16) -> Vec<u8> {
    // SAFETY: message_str returns a string owned by the message list (or
    // a literal); it is copied out at once.
    unsafe { c_bytes(crate::headers::message_h::message_str(age)).to_vec() }
}

// ---------------------------------------------------------------------------
// More terminal output (borg-log.c)

/// `Term_putstr(x, y, n, a, s)` (no NUL in `s`).
pub fn term_putstr(x: i32, y: i32, n: i32, a: i32, s: &[u8]) {
    let c = CString::new(s).expect("no NUL in term text");
    // SAFETY: c is a valid C string for the call.
    unsafe {
        crate::headers::ui_term_h::Term_putstr(x, y, n, a, c.as_ptr());
    }
}

/// `Term_putch(x, y, a, c)`.
pub fn term_putch(x: i32, y: i32, a: i32, c: wchar_t) {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_putch(x, y, a, c);
    }
}

/// `Term_clear()`.
pub fn term_clear() {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_clear();
    }
}

/// `prt(s, row, col)` (no NUL in `s`).
pub fn prt(s: &[u8], row: i32, col: i32) {
    let c = CString::new(s).expect("no NUL in term text");
    // SAFETY: c is a valid C string for the call.
    unsafe { crate::headers::ui_output_h::prt(c.as_ptr(), row, col) }
}

// ---------------------------------------------------------------------------
// Carried objects (borg-inventory.c)

/// A game object the player carries (pack, equipment or quiver) that has
/// a kind. Opaque: the pointer stays on the FFI side (`engine`).
#[derive(Clone, Copy)]
pub struct CarriedObject(*const crate::headers::object_h::object);

impl CarriedObject {

    /// `obj && obj->kind`
    fn with_kind(obj: *const crate::headers::object_h::object) -> Option<Self> {
        if obj.is_null() {
            return None;
        }
        // SAFETY: a non-NULL slot of the player's pack, body or quiver
        // points at a live object.
        let kind = unsafe { (*obj).kind };
        (!kind.is_null()).then_some(CarriedObject(obj))
    }
}

/// `player->upkeep->inven[i]` (i < pack_size), if it is an object with a
/// kind.
pub fn inven_object(i: usize) -> Option<CarriedObject> {
    assert!(i < usize::from(z_info().pack_size), "pack slot out of range");
    // SAFETY: inven is allocated with pack_size + 1 entries with the
    // player's upkeep (index checked).
    CarriedObject::with_kind(unsafe { *(*player_ref().upkeep).inven.add(i) })
}

/// `player->body.slots[i].obj` (i < body.count), if it is an object with a
/// kind.
pub fn equip_object(i: usize) -> Option<CarriedObject> {
    let body = &player_ref().body;
    assert!(i < usize::from(body.count), "equipment slot out of range");
    // SAFETY: slots has body.count entries (checked).
    CarriedObject::with_kind(unsafe { (*body.slots.add(i)).obj })
}

/// `player->upkeep->quiver[i]` (i < quiver_size), if it is an object with
/// a kind.
pub fn quiver_object(i: usize) -> Option<CarriedObject> {
    assert!(i < usize::from(z_info().quiver_size), "quiver slot out of range");
    // SAFETY: the quiver is allocated with z_info->quiver_size entries
    // with the player's upkeep (index checked).
    CarriedObject::with_kind(unsafe { *(*player_ref().upkeep).quiver.add(i) })
}

/// `object_desc(buf, 256, obj, ODESC_FULL, player)` (bytes up to the NUL).
pub fn object_desc_full(obj: CarriedObject) -> Vec<u8> {
    let mut buf = [0u8; 256];
    // SAFETY: obj is a live carried object; object_desc writes at most
    // 256 bytes, NUL included.
    unsafe {
        crate::headers::obj_desc_h::object_desc(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            obj.0,
            crate::headers::obj_desc_h::ODESC_FULL,
            crate::headers::player_h::player,
        );
    }
    let n = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    buf[..n].to_vec()
}
// Danger (borg-danger.c): cave squares, game tables and monster blows

/// `square_in_bounds(cave, loc(x, y))` on the current level.
pub fn square_in_bounds(x: i32, y: i32) -> bool {
    // SAFETY: `cave` is the current level, valid while the game runs; the
    // function only compares the grid with the level's size.
    unsafe { headers::cave_h::square_in_bounds(headers::cave_h::cave, loc { x, y }) }
}

/// `square_isvault(cave, loc(x, y))` on the current level. Panics if the
/// grid is out of bounds (the engine's own `assert` is compiled out in the
/// release build, so C would read outside the level).
pub fn square_isvault(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level, valid while the game runs; the
    // grid is in bounds (checked above).
    unsafe { headers::cave_h::square_isvault(headers::cave_h::cave, loc { x, y }) }
}

/// `square_isno_teleport(cave, loc(x, y))` on the current level: the
/// grid forbids teleporting (borg-escape.c). Panics if the grid is out of
/// bounds (the engine would read outside the level).
pub fn square_isno_teleport(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level, valid while the game runs; the
    // grid is in bounds (checked above).
    unsafe { headers::cave_h::square_isno_teleport(headers::cave_h::cave, loc { x, y }) }
}

/// `adj_dex_safe[i]` (player-calcs.c); panics out of range.
pub fn adj_dex_safe(i: usize) -> i32 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::player_calcs_h::adj_dex_safe[i] }
}

/// `adj_str_hold[i]` (player-calcs.c); panics out of range.
pub fn adj_str_hold(i: usize) -> i32 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::player_calcs_h::adj_str_hold[i] }
}

/// `extract_energy[speed]` (game-world.c); panics out of range.
pub fn extract_energy(speed: usize) -> u8 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::game_world_h::extract_energy[speed] }
}

/// `square_isglow(cave, loc(x, y))` on the current level. Panics if the
/// grid is out of bounds (as `square_isvault`).
pub fn square_isglow(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level, valid while the game runs; the
    // grid is in bounds (checked above).
    unsafe { headers::cave_h::square_isglow(headers::cave_h::cave, loc { x, y }) }
}

/// `panel_contains(y, x)` (game-input.c, `textui_panel_contains`): is the
/// grid on the current map panel? The arguments are C `unsigned int`s (a
/// negative value wraps, as in C).
pub fn panel_contains(y: i32, x: i32) -> bool {
    // SAFETY: plain call; the hook reads the main term's offsets and size.
    unsafe { headers::game_input_h::panel_contains(y as u32, x as u32) }
}

/// `feat_is_trap_holding(feat)` (cave.c): can the terrain feature hold a
/// trap?
pub fn feat_is_trap_holding(feat: i32) -> bool {
    assert!((0..crate::headers::cave_h::FEAT_MAX as i32).contains(&feat), "feature out of range");
    // SAFETY: plain call; reads `f_info[feat]` (index checked above).
    unsafe { headers::cave_h::feat_is_trap_holding(feat) }
}

/// `ddx_ddd[i]` (cave.c): the x offsets of the 8 directions (and 0).
pub fn ddx_ddd(i: usize) -> i16 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::cave_h::ddx_ddd[i] }
}

/// `ddy_ddd[i]` (cave.c): the y offsets of the 8 directions (and 0).
pub fn ddy_ddd(i: usize) -> i16 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::cave_h::ddy_ddd[i] }
}

/// `ddd[i]` (cave.c): the keypad directions in `ddx_ddd` order (and 5).
pub fn ddd(i: usize) -> i16 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::cave_h::ddd[i] }
}

/// `ddx[dir]` (cave.c): the x offset of keypad direction `dir` (0..9).
pub fn ddx(dir: usize) -> i16 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::cave_h::ddx[dir] }
}

/// `ddy[dir]` (cave.c): the y offset of keypad direction `dir` (0..9).
pub fn ddy(dir: usize) -> i16 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::cave_h::ddy[dir] }
}

/// `feat_is_shop(feat)` (cave-square.c): the terrain is a shop entrance.
pub fn feat_is_shop(feat: i32) -> bool {
    assert!((0..crate::headers::cave_h::FEAT_MAX as i32).contains(&feat), "feature out of range");
    // SAFETY: plain call; reads `f_info[feat]` (index checked above).
    unsafe { headers::cave_h::feat_is_shop(feat) }
}

/// One blow of a monster race (`r_info[r_idx].blow[k]`).
#[derive(Clone, Copy)]
pub struct RaceBlow {
    /// `effect->name`
    pub effect_name: &'static CStr,
    /// `dice.dice`
    pub dice: i32,
    /// `dice.sides`
    pub sides: i32,
}

/// `r_info[r_idx].blow[k]` for `k < z_info->mon_blows_max`, or `None` when
/// the blow has no method (the end of the race's blows).
pub fn race_blow(r_idx: usize, k: usize) -> Option<RaceBlow> {
    assert!(k < usize::from(z_info().mon_blows_max));
    let blows = r_info()[r_idx].blow;
    assert!(!blows.is_null(), "race {r_idx} has no blow array");
    // SAFETY: a race's blow array has z_info->mon_blows_max entries
    // (checked above), allocated when the game data is loaded and kept
    // for the game. A blow with a method has an effect with a name (the
    // C code dereferences both without a check; asserted here).
    unsafe {
        let b = &*blows.add(k);
        if b.method.is_null() {
            return None;
        }
        assert!(!b.effect.is_null(), "race {r_idx} blow {k} has no effect");
        let name = (*b.effect).name;
        assert!(!name.is_null(), "race {r_idx} blow {k} effect has no name");
        Some(RaceBlow { effect_name: CStr::from_ptr(name), dice: b.dice.dice, sides: b.dice.sides })
    }
}

// ---------------------------------------------------------------------------
// Game objects and object tables (borg-item-analyze.c)

use crate::headers::object_h::{element_info, object, object_kind};

/// A live game object (`const struct object *`): a carried object, a
/// store ware, or the player's knowledge of one (`obj->known`). Opaque:
/// the pointer stays on the FFI side.
#[derive(Clone, Copy)]
pub struct GameObject(*const object);

impl From<StoreObject> for GameObject {
    fn from(o: StoreObject) -> Self {
        GameObject(o.0)
    }
}

impl From<CarriedObject> for GameObject {
    fn from(o: CarriedObject) -> Self {
        GameObject(o.0)
    }
}

impl GameObject {

    fn obj(&self) -> &object {
        // SAFETY: the handle was made from a live object (CarriedObject or
        // from_raw), which the engine does not change or free while the
        // Borg thinks.
        unsafe { &*self.0 }
    }

    /// The object's kind (panics where C would dereference NULL).
    fn kind(&self) -> &object_kind {
        let k = self.obj().kind;
        assert!(!k.is_null(), "object without a kind");
        // SAFETY: non-NULL kinds point into k_info, which lives for the game.
        unsafe { &*k }
    }

    /// `obj->kind != NULL`
    pub fn has_kind(self) -> bool {
        !self.obj().kind.is_null()
    }
    /// `obj->number`
    pub fn number(self) -> u8 {
        self.obj().number
    }
    /// `obj->tval`
    pub fn tval(self) -> u8 {
        self.obj().tval
    }
    /// `obj->sval`
    pub fn sval(self) -> u8 {
        self.obj().sval
    }
    /// `obj->pval`
    pub fn pval(self) -> i16 {
        self.obj().pval
    }
    /// `obj->timeout`
    pub fn timeout(self) -> i16 {
        self.obj().timeout
    }
    /// `obj->ac`
    pub fn ac(self) -> i16 {
        self.obj().ac
    }
    /// `obj->dd`
    pub fn dd(self) -> u8 {
        self.obj().dd
    }
    /// `obj->ds`
    pub fn ds(self) -> u8 {
        self.obj().ds
    }
    /// `obj->to_h`
    pub fn to_h(self) -> i16 {
        self.obj().to_h
    }
    /// `obj->to_d`
    pub fn to_d(self) -> i16 {
        self.obj().to_d
    }
    /// `obj->to_a`
    pub fn to_a(self) -> i16 {
        self.obj().to_a
    }
    /// `obj->modifiers`
    pub fn modifiers(self) -> [i16; 16] {
        self.obj().modifiers
    }
    /// `obj->el_info`
    pub fn el_info(self) -> [element_info; 25] {
        self.obj().el_info
    }

    /// `obj->known`, if not NULL.
    pub fn known(self) -> Option<GameObject> {
        let k = self.obj().known;
        (!k.is_null()).then_some(GameObject(k))
    }

    /// `object_fully_known(obj)`
    pub fn fully_known(self) -> bool {
        // SAFETY: reads the live object (and its known version) only.
        unsafe { crate::headers::obj_knowledge_h::object_fully_known(self.0) }
    }
    /// `object_runes_known(obj)`
    pub fn runes_known(self) -> bool {
        // SAFETY: reads the live object (and its known version) only.
        unsafe { crate::headers::obj_knowledge_h::object_runes_known(self.0) }
    }
    /// `object_flavor_is_aware(obj)` (the object must have a kind).
    pub fn flavor_is_aware(self) -> bool {
        // SAFETY: reads the object's kind only.
        unsafe { crate::headers::obj_knowledge_h::object_flavor_is_aware(self.0) }
    }
    /// `object_flags_known(obj, flags)`
    pub fn flags_known(self) -> [u8; OF_SIZE] {
        let mut f = [0u8; OF_SIZE];
        // SAFETY: reads the live object (C dereferences `obj->known`, as
        // here); writes exactly OF_SIZE bytes into f.
        unsafe { crate::headers::obj_util_h::object_flags_known(self.0, f.as_mut_ptr()) };
        f
    }
    /// `object_weight_one(obj)`
    pub fn weight_one(self) -> i16 {
        // SAFETY: reads the live object and the curse table only.
        unsafe { crate::headers::obj_util_h::object_weight_one(self.0) }
    }

    /// `obj->kind->kidx`
    pub fn kind_kidx(self) -> u32 {
        self.kind().kidx
    }
    /// `obj->kind->level`
    pub fn kind_level(self) -> i32 {
        self.kind().level
    }
    /// `obj->kind->cost`
    pub fn kind_cost(self) -> i32 {
        self.kind().cost
    }
    /// `randcalc(obj->kind->time, 0, MINIMISE)` (draws no random numbers).
    pub fn kind_time_min(self) -> i32 {
        let t = self.kind().time;
        // SAFETY: pure arithmetic on a copied value; MINIMISE uses no RNG.
        unsafe { crate::headers::z_rand_h::randcalc(t, 0, crate::headers::z_rand_h::MINIMISE) }
    }

    /// `obj->curses[0 .. curse_max].power`, or `None` for NULL `curses`.
    pub fn curse_powers(self) -> Option<Vec<i32>> {
        let p = self.obj().curses;
        if p.is_null() {
            return None;
        }
        let n = usize::from(z_info().curse_max);
        // SAFETY: a non-NULL curses array has z_info->curse_max entries.
        let c = unsafe { std::slice::from_raw_parts(p, n) };
        Some(c.iter().map(|c| c.power).collect())
    }
    /// `obj->slays[0 .. slay_max]`, or `None` for NULL `slays`.
    pub fn slays(self) -> Option<Vec<bool>> {
        let p = self.obj().slays;
        if p.is_null() {
            return None;
        }
        let n = usize::from(z_info().slay_max);
        // SAFETY: a non-NULL slays array has z_info->slay_max entries.
        Some(unsafe { std::slice::from_raw_parts(p, n) }.to_vec())
    }
    /// `obj->brands[0 .. brand_max]`, or `None` for NULL `brands`.
    pub fn brands(self) -> Option<Vec<bool>> {
        let p = self.obj().brands;
        if p.is_null() {
            return None;
        }
        let n = usize::from(z_info().brand_max);
        // SAFETY: a non-NULL brands array has z_info->brand_max entries.
        Some(unsafe { std::slice::from_raw_parts(p, n) }.to_vec())
    }
    /// `obj->activation->index`, or `None` for no activation.
    pub fn activation_index(self) -> Option<i32> {
        let a = self.obj().activation;
        // SAFETY: a non-NULL activation points into the activation list,
        // which lives for the game.
        (!a.is_null()).then(|| unsafe { (*a).index })
    }
    /// `obj->artifact->aidx`, or `None` for no artifact.
    pub fn artifact_aidx(self) -> Option<u32> {
        let a = self.obj().artifact;
        // SAFETY: a non-NULL artifact points into a_info (lives for the game).
        (!a.is_null()).then(|| unsafe { (*a).aidx })
    }
    /// `obj->ego->eidx`, or `None` for no ego.
    pub fn ego_eidx(self) -> Option<u32> {
        let e = self.obj().ego;
        // SAFETY: a non-NULL ego points into e_info (lives for the game).
        (!e.is_null()).then(|| unsafe { (*e).eidx })
    }
}

/// An artifact (`struct artifact`).
pub type Artifact = crate::headers::object_h::artifact;

/// `a_info[0 .. z_info->a_max]`, the artifacts.
pub fn a_info() -> &'static [Artifact] {
    let n = usize::from(z_info().a_max);
    // SAFETY: a_info is allocated with z_info->a_max entries when the game
    // data is loaded, before any Borg code runs, and lives for the game.
    unsafe { std::slice::from_raw_parts(crate::headers::object_h::a_info, n) }
}

/// An ego item type (`struct ego_item`).
pub type EgoItem = crate::headers::object_h::ego_item;

/// `e_info[0 .. z_info->e_max]`, the ego item types.
pub fn e_info() -> &'static [EgoItem] {
    let n = usize::from(z_info().e_max);
    // SAFETY: e_info is allocated with z_info->e_max entries when the game
    // data is loaded, before any Borg code runs, and lives for the game.
    unsafe { std::slice::from_raw_parts(crate::headers::object_h::e_info, n) }
}

/// `curses[i].name` (i < curse_max; panics where C would read NULL).
pub fn curse_name(i: usize) -> &'static [u8] {
    assert!(i < usize::from(z_info().curse_max), "curse index out of range");
    // SAFETY: curses has z_info->curse_max entries (index checked), loaded
    // at startup and kept for the game.
    let name = unsafe { (*crate::headers::obj_curse_h::curses.add(i)).name };
    assert!(!name.is_null(), "curse {i} has no name");
    // SAFETY: a non-NULL curse name is a C string that lives for the game.
    unsafe { CStr::from_ptr(name).to_bytes() }
}

/// `(slays[i].race_flag, slays[i].multiplier)` (i < slay_max).
pub fn slay_race_flag_multiplier(i: usize) -> (i32, i32) {
    assert!(i < usize::from(z_info().slay_max), "slay index out of range");
    // SAFETY: slays has z_info->slay_max entries (index checked), loaded at
    // startup and kept for the game.
    let s = unsafe { &*crate::headers::obj_slays_h::slays.add(i) };
    (s.race_flag, s.multiplier)
}

/// `r_info[r_idx].blow[k].effect->name` (bytes), or `None` when the blow
/// has no effect (`effect == NULL`), for `k < z_info->mon_blows_max`. Unlike
/// `race_blow` this does not look at the method (borg-fight-attack.c tests
/// only the effect).
pub fn race_blow_effect_name(r_idx: usize, k: usize) -> Option<&'static [u8]> {
    assert!(k < usize::from(z_info().mon_blows_max));
    let blows = r_info()[r_idx].blow;
    assert!(!blows.is_null(), "race {r_idx} has no blow array");
    // SAFETY: a race's blow array has z_info->mon_blows_max entries
    // (checked above), allocated when the game data is loaded and kept for
    // the game; a non-NULL effect points into the blow effect list, whose
    // names are C strings kept for the game (checked non-NULL: C would
    // pass NULL to strcmp).
    unsafe {
        let b = &*blows.add(k);
        if b.effect.is_null() {
            return None;
        }
        let name = (*b.effect).name;
        assert!(!name.is_null(), "race {r_idx} blow {k} effect has no name");
        Some(CStr::from_ptr(name).to_bytes())
    }
}

/// `(brands[i].resist_flag, brands[i].vuln_flag, brands[i].multiplier)`
/// (i < brand_max).
pub fn brand_resist_vuln_multiplier(i: usize) -> (i32, i32, i32) {
    assert!(i < usize::from(z_info().brand_max), "brand index out of range");
    // SAFETY: brands has z_info->brand_max entries (index checked), loaded
    // at startup and kept for the game.
    let b = unsafe { &*crate::headers::obj_slays_h::brands.add(i) };
    (b.resist_flag, b.vuln_flag, b.multiplier)
}

/// `player_has_temporary_brand(player, idx)` (obj-slays.c).
pub fn player_has_temporary_brand(idx: i32) -> bool {
    // SAFETY: `player` lives for the game; the callee only reads the
    // player's timed effects and the brand list.
    unsafe { crate::headers::obj_slays_h::player_has_temporary_brand(crate::headers::player_h::player, idx) }
}

/// `player_has_temporary_slay(player, idx)` (obj-slays.c).
pub fn player_has_temporary_slay(idx: i32) -> bool {
    // SAFETY: as player_has_temporary_brand.
    unsafe { crate::headers::obj_slays_h::player_has_temporary_slay(crate::headers::player_h::player, idx) }
}

/// `get_angle_to_grid[y][x]` (generate.c, 41x41); panics out of range.
pub fn get_angle_to_grid(y: usize, x: usize) -> u8 {
    // SAFETY: a constant table of the engine (declared with its C size);
    // nothing writes it.
    unsafe { headers::generate_h::get_angle_to_grid[y][x] }
}

/// `lookup_obj_property(OBJ_PROPERTY_FLAG, flag)->power`, or `None` where
/// the lookup gives NULL.
pub fn obj_flag_power(flag: i32) -> Option<i32> {
    // SAFETY: the property list is loaded at startup; the result is NULL
    // or an entry of it.
    unsafe {
        let p = crate::headers::obj_properties_h::lookup_obj_property(
            crate::headers::obj_properties_h::OBJ_PROPERTY_FLAG as i32,
            flag,
        );
        (!p.is_null()).then(|| (*p).power)
    }
}

/// `flag_next(flags, size, flag)` (z-bitflag.c) on a whole flag array: the
/// first set flag from `flag` on, or `FLAG_END` (0).
pub fn flag_next(flags: &[u8], flag: i32) -> i32 {
    // SAFETY: flag_next reads only below the size given (the slice length).
    unsafe { crate::headers::z_bitflag_h::flag_next(flags.as_ptr(), flags.len(), flag) }
}

/// The effects of `k_info[kind]` as `(index, subtype)`, in list order
/// (following `next` until NULL).
pub fn kind_effects(kind: usize) -> Vec<(u16, i32)> {
    let mut out = Vec::new();
    let mut e = k_info()[kind].effect;
    while !e.is_null() {
        // SAFETY: effects form a NULL-terminated list loaded at startup.
        unsafe {
            out.push(((*e).index, (*e).subtype));
            e = (*e).next;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Floor objects and object kinds (borg-flow-take.c)

/// `k_info[kidx].base->name` (bytes up to the NUL), or `None` when the
/// kind has no base (`base` is NULL). Panics for `kidx >= z_info->k_max`.
pub fn kind_base_name(kidx: usize) -> Option<&'static [u8]> {
    let base = k_info()[kidx].base;
    // SAFETY: a non-NULL kind base points into kb_info, which (with its
    // names) lives for the game.
    (!base.is_null()).then(|| c_bytes(unsafe { (*base).name }))
}

/// `tval_is_book_k(&k_info[kidx])`.
pub fn tval_is_book_k(kidx: usize) -> bool {
    let k: *const ObjectKind = &k_info()[kidx];
    // SAFETY: k points into k_info (index checked above); the callee only
    // reads the kind's tval.
    unsafe { crate::headers::obj_tval_h::tval_is_book_k(k) }
}

/// An object lying on the floor of the current level (an entry of a
/// grid's object pile). Opaque: the pointer stays on the FFI side
/// (`engine`). Only valid until the game changes the pile, so
/// it is used within one Borg call.
#[derive(Clone, Copy)]
pub struct FloorObject(*mut crate::headers::object_h::object);

impl FloorObject {
    /// `*obj->kind` (a floor object always has a kind; it points into
    /// `k_info`).
    pub fn kind(self) -> &'static ObjectKind {
        // SAFETY: a live floor object; its kind points into k_info, which
        // lives for the game (checked non-NULL).
        unsafe {
            let k = (*self.0).kind;
            assert!(!k.is_null(), "floor object without a kind");
            &*k
        }
    }

    /// `object_fully_known(obj)`.
    pub fn fully_known(self) -> bool {
        // SAFETY: a live floor object; the callee only reads it.
        unsafe { crate::headers::obj_knowledge_h::object_fully_known(self.0) }
    }

    /// `(obj->to_a, obj->to_h, obj->to_d)`.
    pub fn to_ahd(self) -> (i16, i16, i16) {
        // SAFETY: a live floor object (plain field reads).
        unsafe { ((*self.0).to_a, (*self.0).to_h, (*self.0).to_d) }
    }

    /// `quark_str(obj->note)` when the object has a note (`obj->note`
    /// non-zero), as bytes.
    pub fn note(self) -> Option<&'static [u8]> {
        // SAFETY: a live floor object; a non-zero quark is a valid index
        // into the quark table, whose strings live for the game.
        unsafe {
            let q = (*self.0).note;
            (q != 0).then(|| c_bytes(crate::headers::z_quark_h::quark_str(q)))
        }
    }

    /// `obj->pval`.
    pub fn pval(self) -> i16 {
        // SAFETY: a live floor object (plain field read).
        unsafe { (*self.0).pval }
    }

    /// `obj->known != NULL`.
    pub fn has_known(self) -> bool {
        // SAFETY: a live floor object (plain field read).
        unsafe { !(*self.0).known.is_null() }
    }
}

/// `borg_get_top_object(cave, loc(x, y))` (borg-flow-take.c): the first
/// object of the grid's pile that is neither ignored by the player
/// (`known->notice & OBJ_NOTICE_IGNORE`) nor of an ignored kind, or `None`.
/// The grid must be in bounds (`square_object` asserts it).
pub fn top_floor_object(x: i32, y: i32) -> Option<FloorObject> {
    use crate::headers::object_h::OBJ_NOTICE_IGNORE;
    // SAFETY: `cave` is the current level, valid while the game runs;
    // square_object asserts the grid is in bounds and returns the head of
    // its pile (or NULL). Each object of a pile is live and has a kind,
    // and its `next` and `known` pointers are NULL or live objects.
    unsafe {
        let mut o = headers::cave_h::square_object(headers::cave_h::cave, loc { x, y });
        while !o.is_null() {
            let ignored_known = !(*o).known.is_null()
                && i32::from((*(*o).known).notice) & OBJ_NOTICE_IGNORE as i32 != 0;
            if !(ignored_known || (*(*o).kind).ignore != 0) {
                break;
            }
            o = (*o).next;
        }
        (!o.is_null()).then_some(FloorObject(o))
    }
}


// ---------------------------------------------------------------------------
// Borg start-up (borg-init.c)

/// `do_cmd_redraw()`: redraw the whole screen (signals events: lends the
/// state).
pub fn do_cmd_redraw(st: &mut BorgState) {
    // SAFETY: plain call on the engine thread.
    lend(st, || unsafe { crate::headers::ui_command_h::do_cmd_redraw() })
}

/// `event_signal(type)` (a message flush can wait for a key: lends the
/// state).
pub fn event_signal(st: &mut BorgState, kind: game_event_type) {
    // SAFETY: plain call; handlers get no data, as in C.
    lend(st, || unsafe { crate::headers::game_event_h::event_signal(kind) })
}

/// `screensaver` (option.c).
pub fn screensaver() -> bool {
    // SAFETY: plain read of a bool global on the engine thread.
    unsafe { crate::headers::option_h::screensaver }
}

/// `(tile_width, tile_height)` (ui-term.c).
pub fn tile_size() -> (u8, u8) {
    // SAFETY: plain reads of two byte globals on the engine thread.
    unsafe { (crate::headers::ui_term_h::tile_width, crate::headers::ui_term_h::tile_height) }
}

/// `player->upkeep->redraw |= flags`.
pub fn player_redraw_add(flags: u32) {
    // SAFETY: `player` and its upkeep are allocated at startup and live for
    // the game; single-threaded write.
    unsafe { (*(*crate::headers::player_h::player).upkeep).redraw |= flags }
}

/// `player->opts.hitpoint_warn = v`.
pub fn set_player_hitpoint_warn(v: u8) {
    // SAFETY: `player` is allocated at startup; single-threaded write.
    unsafe { (*crate::headers::player_h::player).opts.hitpoint_warn = v }
}

/// `player_id2class(id)->name` (panics where C would read through NULL).
pub fn class_name_by_id(id: u32) -> &'static [u8] {
    // SAFETY: player_id2class returns NULL or an entry of the static class
    // list, whose name is a C string that lives for the game.
    unsafe {
        let c = crate::headers::player_h::player_id2class(id);
        assert!(!c.is_null(), "player_id2class returned NULL");
        CStr::from_ptr((*c).name).to_bytes()
    }
}

/// `ITYPE_MAX` (obj-ignore.h): the number of ignore types.
pub const ITYPE_MAX: usize = crate::headers::obj_ignore_h::ITYPE_MAX as usize;

/// `k_info[kind].ignore = v`.
pub fn set_kind_ignore(kind: usize, v: u8) {
    assert!(kind < usize::from(z_info().k_max));
    // SAFETY: k_info has z_info->k_max entries (checked); single-threaded
    // write of a byte field.
    unsafe { (*crate::headers::object_h::k_info.add(kind)).ignore = v }
}

/// `ignore_level[0 .. ITYPE_MAX]` (obj-ignore.c).
pub fn ignore_level() -> [u8; ITYPE_MAX] {
    // SAFETY: ignore_level is a static array of ITYPE_MAX bytes (declared
    // with length 0 in the header).
    unsafe {
        let p = core::ptr::addr_of!(crate::headers::obj_ignore_h::ignore_level) as *const u8;
        *(p as *const [u8; ITYPE_MAX])
    }
}

/// `ignore_level[i] = levels[i]` for all ignore types.
pub fn set_ignore_level(levels: &[u8; ITYPE_MAX]) {
    // SAFETY: as ignore_level(); single-threaded write.
    unsafe {
        let p = core::ptr::addr_of_mut!(crate::headers::obj_ignore_h::ignore_level) as *mut u8;
        *(p as *mut [u8; ITYPE_MAX]) = *levels;
    }
}

/// `ego_ignore_types[ego][0 .. ITYPE_MAX]` (obj-ignore.c).
pub fn ego_ignore_types(ego: usize) -> [bool; ITYPE_MAX] {
    assert!(ego < usize::from(z_info().e_max));
    // SAFETY: ego_ignore_types has z_info->e_max rows (checked) of
    // ITYPE_MAX bools each, allocated by ignore_init() at startup.
    unsafe {
        *(*crate::headers::obj_ignore_h::ego_ignore_types.add(ego) as *const [bool; ITYPE_MAX])
    }
}

/// `ego_ignore_types[ego][i] = types[i]` for all ignore types.
pub fn set_ego_ignore_types(ego: usize, types: &[bool; ITYPE_MAX]) {
    assert!(ego < usize::from(z_info().e_max));
    // SAFETY: as ego_ignore_types(); single-threaded write.
    unsafe {
        *(*crate::headers::obj_ignore_h::ego_ignore_types.add(ego) as *mut [bool; ITYPE_MAX]) =
            *types;
    }
}

/// `ignore_birth_init()` (obj-ignore.c): reset the ignore settings.
pub fn ignore_birth_init() {
    // SAFETY: plain call on the engine thread.
    unsafe { crate::headers::obj_ignore_h::ignore_birth_init() }
}

// ---------------------------------------------------------------------------
// The Borg's ^Z command interface (borg-commands.c). The calls that can
// wait for a key (`get_com`, `get_com_ex`, `get_quantity`, `menu_select`,
// `target_set_interactive`, `screen_save`/`screen_load` (message flush),
// `project` (display events)) lend the caller's state (see `enter`).

/// `player->grid`
pub fn player_grid() -> loc {
    player_ref().grid
}

/// `player->opts.lazymove_delay`
pub fn player_lazymove_delay() -> u8 {
    player_ref().opts.lazymove_delay
}

/// `player->opts.lazymove_delay = v`
pub fn set_player_lazymove_delay(v: u8) {
    // SAFETY: `player` is allocated at startup; single-threaded write.
    unsafe { (*crate::headers::player_h::player).opts.lazymove_delay = v }
}

/// `player->opts.delay_factor`
pub fn player_delay_factor() -> u8 {
    player_ref().opts.delay_factor
}

/// `player->opts.delay_factor = v`
pub fn set_player_delay_factor(v: u8) {
    // SAFETY: `player` is allocated at startup; single-threaded write.
    unsafe { (*crate::headers::player_h::player).opts.delay_factor = v }
}

/// `Term_save()`
pub fn term_save() {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_save();
    }
}

/// `Term_load()`
pub fn term_load() {
    // SAFETY: plain call on the active term (with no saved screen it
    // loads a blank one).
    unsafe {
        crate::headers::ui_term_h::Term_load();
    }
}

/// `Term_flush()`: discard pending keypresses.
pub fn term_flush() {
    // SAFETY: plain call on the active term.
    unsafe {
        crate::headers::ui_term_h::Term_flush();
    }
}

/// `target_okay()`
pub fn target_okay() -> bool {
    // SAFETY: plain call; reads the target and the current level.
    unsafe { crate::headers::target_h::target_okay() }
}

/// `target_get(&grid)`
pub fn target_get() -> loc {
    let mut l = loc { x: 0, y: 0 };
    // SAFETY: out-pointer to a local.
    unsafe { crate::headers::target_h::target_get(&mut l) };
    l
}

/// `get_com(prompt, &cmd)`: the result and the key as the C `char`
/// (byte). Waits for a key (lends the state).
pub fn get_com(st: &mut BorgState, prompt: &CStr) -> (bool, u8) {
    let mut cmd: c_char = 0;
    // SAFETY: valid prompt and out-pointer to a local.
    let r = lend(st, || unsafe { crate::headers::game_input_h::get_com(prompt.as_ptr(), &mut cmd) });
    (r, cmd as u8)
}

/// `get_com_ex(prompt, &ke)`; the event itself is dropped. Waits for a
/// key (lends the state).
pub fn get_com_ex(st: &mut BorgState, prompt: &CStr) -> bool {
    // SAFETY: ui_event is plain data (zeroed = EVT_NONE); valid prompt and
    // out-pointer to a local.
    lend(st, || unsafe {
        let mut ke: crate::headers::ui_event_h::ui_event = core::mem::zeroed();
        crate::headers::ui_input_h::get_com_ex(prompt.as_ptr(), &mut ke)
    })
}

/// `get_quantity(prompt, max)`. Waits for keys (lends the state).
pub fn get_quantity(st: &mut BorgState, prompt: &CStr, max: i32) -> i32 {
    // SAFETY: valid prompt.
    lend(st, || unsafe { crate::headers::game_input_h::get_quantity(prompt.as_ptr(), max) })
}

/// `target_set_interactive(mode, x, y, allow_pathfinding)`. Waits for
/// keys (lends the state).
pub fn target_set_interactive(st: &mut BorgState, mode: i32, x: i32, y: i32, allow_pathfinding: bool) -> bool {
    // SAFETY: plain call on the engine thread.
    lend(st, || unsafe { crate::headers::ui_target_h::target_set_interactive(mode, x, y, allow_pathfinding) })
}

/// `print_rel(c, a, y, x)`: draw a character at a map grid (if it is on
/// the panel).
pub fn print_rel(c: u8, a: u8, y: i32, x: i32) {
    // SAFETY: plain call; the function only draws grids on the panel.
    unsafe { crate::headers::ui_map_h::print_rel(wchar_t::from(c), a, y, x) }
}

/// `prt_map()`: redraw the map.
pub fn prt_map() {
    // SAFETY: plain call on the engine thread.
    unsafe { crate::headers::ui_map_h::prt_map() }
}

/// `c_put_str(attr, s, row, col)` (no NUL in `s`).
pub fn c_put_str(attr: u8, s: &[u8], row: i32, col: i32) {
    let c = CString::new(s).expect("no NUL in term text");
    // SAFETY: c is a valid C string for the call.
    unsafe { crate::headers::ui_output_h::c_put_str(attr, c.as_ptr(), row, col) }
}

/// `screen_save()` (flushes messages, which can wait for a key: lends
/// the state).
pub fn screen_save(st: &mut BorgState) {
    // SAFETY: plain call on the engine thread.
    lend(st, || unsafe { crate::headers::ui_output_h::screen_save() })
}

/// `screen_load()` (flushes messages, which can wait for a key: lends
/// the state).
pub fn screen_load(st: &mut BorgState) {
    // SAFETY: plain call on the engine thread.
    lend(st, || unsafe { crate::headers::ui_output_h::screen_load() })
}

/// `window_make(origin_x, origin_y, end_x, end_y)`
pub fn window_make(origin_x: i32, origin_y: i32, end_x: i32, end_y: i32) {
    // SAFETY: plain call; draws on the active term.
    unsafe { crate::headers::ui_output_h::window_make(origin_x, origin_y, end_x, end_y) }
}

/// One `display_row` call of a menu.
#[derive(Clone, Copy, Debug)]
pub struct MenuRow {
    pub oid: i32,
    pub cursor: bool,
    pub row: i32,
    pub col: i32,
    pub width: i32,
}

/// The row display of the menu shown by `Menu::select` (lifetime erased;
/// only set while `select` runs).
static mut MENU_DISPLAY: Option<*mut dyn FnMut(MenuRow)> = None;

unsafe extern "C" fn menu_display_row(
    _menu: *mut crate::headers::ui_menu_h::menu,
    oid: core::ffi::c_int,
    cursor: bool,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    width: core::ffi::c_int,
) {
    let f = MENU_DISPLAY.expect("menu row drawn outside Menu::select");
    (*f)(MenuRow { oid, cursor, row, col, width });
}

/// The row functions of a Borg menu: only `display_row`, as the
/// `menu_iter`s of borg-commands.c.
static MENU_ITER: crate::headers::ui_menu_h::menu_iter = crate::headers::ui_menu_h::menu_iter {
    get_tag: None,
    valid_row: None,
    display_row: Some(menu_display_row),
    row_handler: None,
    resize: None,
};

/// A scrolling menu (`menu_new(MN_SKIN_SCROLL, iter)`). As in C it is
/// never freed.
pub struct Menu(*mut crate::headers::ui_menu_h::menu);

impl Menu {
    /// `menu_new(MN_SKIN_SCROLL, &iter)`
    pub fn new_scroll() -> Menu {
        // SAFETY: MENU_ITER is a static that outlives the menu.
        let m = unsafe {
            crate::headers::ui_menu_h::menu_new(crate::headers::ui_menu_h::MN_SKIN_SCROLL, &MENU_ITER)
        };
        assert!(!m.is_null());
        Menu(m)
    }

    /// `menu_setpriv(m, count, data)`. The private data is never read
    /// (the row display does not use it); a pointer to the row functions
    /// is stored.
    pub fn setpriv(&mut self, count: i32) {
        // SAFETY: live menu; the data pointer is only stored.
        unsafe {
            crate::headers::ui_menu_h::menu_setpriv(
                self.0,
                count,
                &MENU_ITER as *const _ as *mut core::ffi::c_void,
            )
        }
    }

    /// `menu_layout(m, &area)`
    pub fn layout(&mut self, area: crate::headers::ui_output_h::region) -> bool {
        // SAFETY: live menu; the region is read during the call.
        unsafe { crate::headers::ui_menu_h::menu_layout(self.0, &area) }
    }

    /// `menu_select(m, notify, popup)`: the type of the returned event.
    /// `display` draws a row (the `display_row` of the C iterator).
    /// Waits for keys (lends the state; `display` cannot use it).
    pub fn select(
        &mut self,
        st: &mut BorgState,
        notify: i32,
        popup: bool,
        display: &mut dyn FnMut(MenuRow),
    ) -> ui_event_type {
        let p: *mut (dyn FnMut(MenuRow) + '_) = display;
        // SAFETY: the pointer is only used by menu_display_row while this
        // call runs (restored before returning), so the erased lifetime is
        // never outlived; engine thread only.
        lend(st, || unsafe {
            let saved = MENU_DISPLAY;
            MENU_DISPLAY = Some(core::mem::transmute::<
                *mut (dyn FnMut(MenuRow) + '_),
                *mut (dyn FnMut(MenuRow) + 'static),
            >(p));
            let ev = crate::headers::ui_menu_h::menu_select(self.0, notify, popup);
            MENU_DISPLAY = saved;
            ev.type_0
        })
    }

    /// `m->cursor`
    pub fn cursor(&self) -> i32 {
        // SAFETY: live menu.
        unsafe { (*self.0).cursor }
    }
}

/// `project(source_player(), rad, loc(x, y), dam, typ, flg, arc,
/// diameter, NULL)`. Signals display events (lends the state).
#[allow(clippy::too_many_arguments)]
pub fn project_from_player(
    st: &mut BorgState,
    rad: i32,
    x: i32,
    y: i32,
    dam: i32,
    typ: i32,
    flg: i32,
    degrees_of_arc: i32,
    diameter_of_source: u8,
) -> bool {
    // SAFETY: plain call; a NULL object is allowed.
    lend(st, || unsafe {
        crate::headers::project_h::project(
            crate::headers::source_h::source_player(),
            rad,
            loc { x, y },
            dam,
            typ,
            flg,
            degrees_of_arc,
            diameter_of_source,
            core::ptr::null(),
        )
    })
}

/// `SQUARE_SIZE`: bytes in a square's info flags.
pub const SQUARE_SIZE: usize = (crate::headers::cave_h::SQUARE_MAX as usize).div_ceil(8);

/// `square(cave, loc(x, y))->feat`; panics out of bounds (the engine's
/// assert is compiled out).
pub fn square_feat(x: i32, y: i32) -> u8 {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds.
    unsafe { (*headers::cave_h::square(headers::cave_h::cave, loc { x, y })).feat }
}

/// `square(cave, loc(x, y))->info` (`SQUARE_SIZE` bytes); panics out of
/// bounds.
pub fn square_info(x: i32, y: i32) -> [u8; SQUARE_SIZE] {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds and its
    // info points at SQUARE_SIZE bytes.
    unsafe {
        let sq = headers::cave_h::square(headers::cave_h::cave, loc { x, y });
        *((*sq).info as *const [u8; SQUARE_SIZE])
    }
}

/// `f_info[feat].name` (feat < FEAT_MAX; panics on a NULL name, where C
/// would crash in `c_put_str`).
pub fn feat_name(feat: i32) -> &'static [u8] {
    assert!((0..crate::headers::cave_h::FEAT_MAX as i32).contains(&feat), "feature out of range");
    // SAFETY: f_info has FEAT_MAX entries (index checked), loaded at
    // startup and kept for the game.
    let name = unsafe { (*crate::headers::cave_h::f_info.add(feat as usize)).name };
    assert!(!name.is_null(), "feature without a name");
    // SAFETY: a non-NULL game data string.
    unsafe { CStr::from_ptr(name).to_bytes() }
}

/// `k_info[kidx].name` as `%s` prints it (NULL prints as "").
pub fn kind_name_or_empty(kidx: usize) -> &'static [u8] {
    c_bytes(k_info()[kidx].name)
}

/// `a_info[aidx].name` as `%s` prints it (NULL prints as "").
pub fn artifact_name_or_empty(aidx: usize) -> &'static [u8] {
    c_bytes(a_info()[aidx].name)
}

/// `object_flags(player->upkeep->inven[n], f)` (all zero for an empty
/// slot); `0 <= n <= pack_size` (the array has a NULL entry at
/// `pack_size`).
pub fn inven_object_flags(n: i32) -> [u8; OF_SIZE] {
    assert!((0..=i32::from(z_info().pack_size)).contains(&n), "pack slot out of range");
    let mut f = [0u8; OF_SIZE];
    // SAFETY: inven has pack_size + 1 entries (index checked); object_flags
    // accepts NULL and writes OF_SIZE bytes.
    unsafe {
        let obj = *(*player_ref().upkeep).inven.add(n as usize);
        crate::headers::obj_util_h::object_flags(obj, f.as_mut_ptr());
    }
    f
}

// ---------------------------------------------------------------------------
// Map updates (borg-update.c)

/// `cave->width`, `cave->height` of the current level.
pub fn cave_size() -> (i32, i32) {
    // SAFETY: `cave` is the current level, valid while the game runs.
    unsafe {
        let c = &*headers::cave_h::cave;
        (c.width, c.height)
    }
}

/// What `map_info()` reports about a grid (the fields the Borg reads).
#[derive(Clone, Copy, Debug)]
pub struct MapGrid {
    /// `m_idx`: the visible monster (0 = none)
    pub m_idx: u32,
    /// `f_idx`: the remembered feature (after mimics)
    pub f_idx: u32,
    /// `first_kind`: `(kidx, d_attr, d_char)` of the first visible object
    /// kind, `None` for NULL
    pub first_kind: Option<(u32, u8, wchar_t)>,
    /// `lighting` (`LIGHTING_*`)
    pub lighting: u32,
    pub in_view: bool,
    pub is_player: bool,
}

/// `map_info(loc(x, y), &g)` (cave-map.c). It also memorizes the grid
/// when the player sees it, as in C. Panics out of bounds (the engine's
/// asserts are compiled out).
pub fn map_info(x: i32, y: i32) -> MapGrid {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    let mut g = headers::cave_h::grid_data {
        m_idx: 0,
        f_idx: 0,
        first_kind: core::ptr::null_mut(),
        trap: core::ptr::null_mut(),
        multiple_objects: false,
        unseen_object: false,
        unseen_money: false,
        lighting: 0,
        in_view: false,
        is_player: false,
        hallucinate: false,
    };
    // SAFETY: the grid is in bounds (checked); map_info fills every field
    // the Borg reads; first_kind is NULL or an object kind of the game
    // data, which lives for the game.
    unsafe {
        headers::cave_h::map_info(loc { x, y }, &mut g);
        MapGrid {
            m_idx: g.m_idx,
            f_idx: g.f_idx,
            first_kind: (!g.first_kind.is_null()).then(|| {
                let k = &*g.first_kind;
                (k.kidx, k.d_attr, k.d_char)
            }),
            lighting: g.lighting,
            in_view: g.in_view,
            is_player: g.is_player,
        }
    }
}

/// `square_shopnum(cave, loc(x, y))`: the store of a shop entrance (-1
/// otherwise). Panics out of bounds.
pub fn square_shopnum(x: i32, y: i32) -> i32 {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds.
    unsafe { headers::cave_h::square_shopnum(headers::cave_h::cave, loc { x, y }) }
}

/// `square_isdisarmabletrap(cave, loc(x, y))`. Panics out of bounds.
pub fn square_isdisarmabletrap(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds.
    unsafe { headers::cave_h::square_isdisarmabletrap(headers::cave_h::cave, loc { x, y }) }
}

/// `trf_has(trap_info[square(cave, loc(x, y))->trap->t_idx].flags,
/// TRF_GLYPH)`: the (first) trap on the grid is a glyph. Panics out of
/// bounds, without a trap (C reads through NULL) or for a bad trap index.
pub fn square_trap_is_glyph(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds; the trap
    // pointer is checked; trap_info has z_info->trap_max entries (index
    // checked); flag_has reads within the flag array's size.
    unsafe {
        let sq = headers::cave_h::square(headers::cave_h::cave, loc { x, y });
        let trap = (*sq).trap;
        assert!(!trap.is_null(), "no trap on the grid");
        let t_idx = usize::from((*trap).t_idx);
        assert!(t_idx < usize::from(z_info().trap_max), "trap index out of range");
        let flags = &(*headers::trap_h::trap_info.add(t_idx)).flags;
        headers::z_bitflag_h::flag_has(
            flags.as_ptr(),
            flags.len(),
            headers::list_trap_flags_h::TRF_GLYPH as i32,
        )
    }
}

/// `square_iswebbed(cave, loc(x, y))`. Panics out of bounds.
pub fn square_iswebbed(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level; the grid is in bounds.
    unsafe { headers::cave_h::square_iswebbed(headers::cave_h::cave, loc { x, y }) }
}

// ---------------------------------------------------------------------------
// Messages (borg-messages.c)

/// A C string of game data as owned bytes, `None` for NULL.
fn opt_text(p: *const c_char) -> Option<Vec<u8>> {
    // SAFETY: non-NULL pointers passed here are game data strings that live
    // for the game.
    (!p.is_null()).then(|| unsafe { CStr::from_ptr(p).to_bytes().to_vec() })
}

/// `inkey_flag` (ui-input.c): the game is waiting for a command.
pub fn inkey_flag() -> bool {
    // SAFETY: plain read of a bool global; engine thread.
    unsafe { crate::headers::ui_input_h::inkey_flag }
}

/// `player->wizard`
pub fn player_wizard() -> bool {
    player_ref().wizard
}

/// `TERM_XTRA_NOISE` (ui-term.h): make a noise.
pub const TERM_XTRA_NOISE: i32 = 7;

/// `Term_xtra(n, v)` on the current terminal (the Borg only uses
/// `TERM_XTRA_NOISE` and `TERM_XTRA_DELAY`; the headless frontend ignores
/// the noise and does not sleep).
pub fn term_xtra(n: i32, v: i32) -> i32 {
    // SAFETY: the active terminal is set up by the frontend; a noise or
    // delay request never waits for a key.
    unsafe { crate::headers::ui_term_h::Term_xtra(n, v) }
}

/// `square_iswarded(cave, loc(x, y))` on the current level: a glyph of
/// warding (or another protective rune) on the grid. Panics if the grid
/// is out of bounds (as `square_isvault`).
pub fn square_iswarded(x: i32, y: i32) -> bool {
    assert!(square_in_bounds(x, y), "grid out of bounds");
    // SAFETY: `cave` is the current level, valid while the game runs; the
    // grid is in bounds (checked above).
    unsafe { headers::cave_h::square_iswarded(headers::cave_h::cave, loc { x, y }) }
}

/// The texts of one level of a monster spell (`struct
/// monster_spell_level`), `None` for NULL.
pub struct SpellLevelTexts {
    pub blind_message: Option<Vec<u8>>,
    pub message: Option<Vec<u8>>,
    pub miss_message: Option<Vec<u8>>,
}

/// The monster spells (`monster_spells`, in list order): each spell's
/// `index` and the texts of its levels (in list order).
pub fn monster_spell_texts() -> Vec<(u16, Vec<SpellLevelTexts>)> {
    use crate::headers::monster_h::{monster_spell, monster_spell_level};
    let mut out = Vec::new();
    // SAFETY: `monster_spells` is parsed game data (NULL-ended lists,
    // never freed while the game runs); the nodes are only read.
    unsafe {
        let mut spell = crate::headers::monster_h::monster_spells as *const monster_spell;
        while !spell.is_null() {
            let mut levels = Vec::new();
            let mut lv = (*spell).level as *const monster_spell_level;
            while !lv.is_null() {
                levels.push(SpellLevelTexts {
                    blind_message: opt_text((*lv).blind_message),
                    message: opt_text((*lv).message),
                    miss_message: opt_text((*lv).miss_message),
                });
                lv = (*lv).next;
            }
            out.push(((*spell).index, levels));
            spell = (*spell).next;
        }
    }
    out
}

/// `pain_messages[idx].messages` (`None` for NULL); panics unless
/// `idx < z_info->mp_max`.
pub fn pain_texts(idx: usize) -> [Option<Vec<u8>>; 7] {
    assert!(idx < usize::from(z_info().mp_max), "pain index out of range");
    // SAFETY: plain read of a pointer global set up by the game's init.
    let p = unsafe { crate::headers::monster_h::pain_messages };
    assert!(!p.is_null(), "no pain messages");
    // SAFETY: `pain_messages` has `mp_max` entries (index checked) of game
    // data that live for the game.
    let pain = unsafe { &*p.add(idx) };
    core::array::from_fn(|i| opt_text(pain.messages[i]))
}

/// The blow methods (`blow_methods[0 .. z_info->blow_methods_max]`): each
/// method's `msgt` and the `act_msg` of its messages (in list order,
/// `None` for NULL).
pub fn blow_method_texts() -> Vec<(i32, Vec<Option<Vec<u8>>>)> {
    use crate::headers::mon_blows_h::blow_message;
    let n = usize::from(z_info().blow_methods_max);
    // SAFETY: plain read of a pointer global set up by the game's init.
    let base = unsafe { crate::headers::mon_blows_h::blow_methods };
    assert!(n == 0 || !base.is_null(), "no blow methods");
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        // SAFETY: `blow_methods` has `blow_methods_max` entries of parsed
        // game data (NULL-ended message lists) that live for the game.
        unsafe {
            let m = &*base.add(i);
            let mut texts = Vec::new();
            let mut msg = m.messages as *const blow_message;
            while !msg.is_null() {
                texts.push(opt_text((*msg).act_msg));
                msg = (*msg).next;
            }
            out.push((m.msgt, texts));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The player as borg-trait.c reads it

/// `player->race`: the player's race (race list entry, set at birth).
pub fn player_race_info() -> &'static crate::headers::player_h::player_race {
    let r = player_ref().race;
    assert!(!r.is_null(), "player has no race");
    // SAFETY: `player->race` points into the race list loaded at startup
    // (set at birth, lives for the game); non-NULL (checked above).
    unsafe { &*r }
}

/// `player->class`: the player's class (class list entry, set at birth).
pub fn player_class_info() -> &'static crate::headers::player_h::player_class {
    let c = player_ref().class;
    assert!(!c.is_null(), "player has no class");
    // SAFETY: `player->class` points into the class list loaded at
    // startup (set at birth, lives for the game); non-NULL (checked above).
    unsafe { &*c }
}

/// The player fields that borg-trait.c reads (`borg_notice_player`,
/// `borg_notice_equipment`), copied.
#[derive(Clone, Copy, Debug)]
pub struct PlayerNotice {
    pub lev: i16,
    pub max_lev: i16,
    pub total_winner: u16,
    pub exp: i32,
    pub max_exp: i32,
    pub depth: i16,
    pub max_depth: i16,
    pub au: i32,
    pub word_recall: i16,
    pub chp: i16,
    pub mhp: i16,
    pub csp: i16,
    pub msp: i16,
    pub stat_cur: [i16; 5],
    pub stat_max: [i16; 5],
    /// `player->player_hp` (hit points per level)
    pub player_hp: [i16; 50],
    /// `player->upkeep->new_spells`
    pub new_spells: i32,
    /// `player->obj_k->modifiers` (the modifier runes the player knows)
    pub obj_k_modifiers: [i16; 16],
}

/// Read the player fields of `PlayerNotice`.
pub fn player_notice() -> PlayerNotice {
    let p = player_ref();
    assert!(!p.upkeep.is_null() && !p.obj_k.is_null(), "player not set up");
    // SAFETY: `upkeep` and `obj_k` are allocated with the player (checked
    // non-NULL above) and live for the game.
    let (new_spells, obj_k_modifiers) = unsafe { ((*p.upkeep).new_spells, (*p.obj_k).modifiers) };
    PlayerNotice {
        lev: p.lev,
        max_lev: p.max_lev,
        total_winner: p.total_winner,
        exp: p.exp,
        max_exp: p.max_exp,
        depth: p.depth,
        max_depth: p.max_depth,
        au: p.au,
        word_recall: p.word_recall,
        chp: p.chp,
        mhp: p.mhp,
        csp: p.csp,
        msp: p.msp,
        stat_cur: p.stat_cur,
        stat_max: p.stat_max,
        player_hp: p.player_hp,
        new_spells,
        obj_k_modifiers,
    }
}

/// `modify_stat_value(value, amount)` (player-util.c): a stat value
/// raised or lowered by `amount` points.
pub fn modify_stat_value(value: i32, amount: i32) -> i16 {
    // SAFETY: pure arithmetic on two values.
    unsafe { crate::headers::player_util_h::modify_stat_value(value, amount) }
}

/// `adj_str_blow[i]` (player-calcs.c); panics out of range.
pub fn adj_str_blow(i: usize) -> i32 {
    // SAFETY: a constant table of the engine; nothing writes it.
    unsafe { headers::player_calcs_h::adj_str_blow[i] }
}

/// The food levels of player-timed.c: (`PY_FOOD_WEAK`, `PY_FOOD_HUNGRY`,
/// `PY_FOOD_FULL`, `PY_FOOD_MAX`).
pub fn py_food_levels() -> (i32, i32, i32, i32) {
    use crate::headers::player_timed_h as t;
    // SAFETY: plain reads of int globals the game sets at init and never
    // changes afterwards.
    unsafe { (t::PY_FOOD_WEAK, t::PY_FOOD_HUNGRY, t::PY_FOOD_FULL, t::PY_FOOD_MAX) }
}
