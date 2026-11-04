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

/// An object kind (`struct object_kind`).
pub type ObjectKind = crate::headers::object_h::object_kind;

/// `k_info[0 .. z_info->k_max]`, the object kinds.
pub fn k_info() -> &'static [ObjectKind] {
    let n = usize::from(z_info().k_max);
    // SAFETY: k_info is allocated with z_info->k_max entries when the game
    // data is loaded, before any Borg code runs, and lives for the game.
    unsafe { std::slice::from_raw_parts(crate::headers::object_h::k_info, n) }
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
    let name = r_info()[r_idx].name;
    assert!(!name.is_null(), "race {r_idx} has no name");
    // SAFETY: a race's name is a C string allocated when the game data is
    // loaded and kept for the game; checked non-null.
    unsafe { CStr::from_ptr(name).to_bytes() }
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

/// `all_letters_nohjkl[i]` (ui-menu.c): the menu letters without h/j/k/l.
pub fn all_letters_nohjkl(i: i32) -> u8 {
    // SAFETY: a static NUL-terminated string defined in ui-menu.c.
    let s = unsafe {
        CStr::from_ptr(core::ptr::addr_of!(crate::headers::ui_menu_h::all_letters_nohjkl)
            as *const c_char)
    };
    s.to_bytes_with_nul()[i as usize]
}

/// `all_letters[i]` (ui-menu.c): the menu letters a-z, A-Z (panics past
/// the NUL, where C would read out of bounds).
pub fn all_letters(i: usize) -> u8 {
    // SAFETY: a static NUL-terminated string defined in ui-menu.c.
    let s = unsafe {
        CStr::from_ptr(core::ptr::addr_of!(crate::headers::ui_menu_h::all_letters) as *const c_char)
    };
    s.to_bytes_with_nul()[i]
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
    let magic = &player_class_ref().magic;
    debug_assert!(book >= 0 && book < magic.num_books);
    // SAFETY: as in C, `book` is not checked here: callers only pass books
    // the Borg holds copies of (`borg.amt_book[book] != 0`), and those
    // exist for the class.
    unsafe { (*magic.books.offset(book as isize)).dungeon }
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
    /// Wrap an open file.
    ///
    /// # Safety
    /// `f` must be an open `ang_file` that stays open while the handle is
    /// used.
    pub unsafe fn from_raw(f: *mut crate::headers::z_file_h::ang_file) -> Self {
        AngFile(f)
    }
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
