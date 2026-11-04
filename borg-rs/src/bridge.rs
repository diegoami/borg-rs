//! Transitional adapters between converted (safe) modules and the
//! not-yet-converted c2rust code. Everything here goes away when the last
//! c2rust module is converted.

use core::ffi::c_char;

/// `borg_trim()` on a C buffer of `N` bytes: returns the trimmed start.
pub unsafe fn trim_c_buf<const N: usize>(buf: *mut [c_char; N]) -> *mut c_char {
    let bytes = &mut *(buf as *mut [u8; N]);
    let off = crate::borg_util::borg_trim(bytes);
    (buf as *mut c_char).add(off)
}

use crate::state::BorgState;

static mut STATE: *mut BorgState = core::ptr::null_mut();

/// The one Borg state, for code that is still c2rust. Allocated on first
/// use and never freed: the C globals lived for the whole process too.
pub fn st() -> &'static mut BorgState {
    // SAFETY: single-threaded (the engine thread); the pointer is
    // created once and never freed. Transitional: c2rust code and
    // converted code share it the way they shared the C globals.
    unsafe {
        if STATE.is_null() {
            STATE = Box::into_raw(Box::default());
        }
        &mut *STATE
    }
}

/// Integer conversions the way C does them when indexing (`as`).
pub trait CInt {
    fn ci(self) -> i32;
}
macro_rules! cint {
    ($($t:ty),*) => { $(impl CInt for $t { fn ci(self) -> i32 { self as i32 } })* };
}
cint!(u8, i8, u16, i16, u32, i32, i64, u64, isize, usize);

/// `&borg_grids[y][x]` for c2rust code.
pub unsafe fn grid<X: CInt, Y: CInt>(x: X, y: Y) -> *mut crate::borg_cave::BorgGrid {
    st().cave.grid_mut(x.ci(), y.ci()) as *mut _
}

/// `borg_note(what)` (borg-io.c, converted).
pub fn borg_note(st: &mut BorgState, what: &str) {
    crate::borg_io::borg_note(st, what)
}

/// `borg_engine_date` as a C string (for c2rust format() calls).
pub static ENGINE_DATE: &std::ffi::CStr = c"Nov  4 2026";

/// `borg_oops(what)` (still c2rust, borg-think.c).
pub fn borg_oops(_st: &mut BorgState, what: &str) {
    let c = std::ffi::CString::new(what).expect("no NUL");
    // SAFETY: borg_oops reads the string and copies it into a note.
    unsafe { crate::c2rust::borg_think::borg_oops(c.as_ptr()) }
}

unsafe fn cbytes<'a>(p: *const c_char) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

/// C-ABI shims with the borg-io.c names, for c2rust callers.
pub mod io {
    use super::{cbytes, st};
    use crate::borg_io;
    use crate::headers::ui_event_h::{keycode_t, keypress};
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn borg_what_text(
        x: c_int,
        y: c_int,
        n: c_int,
        a: *mut u8,
        s: *mut c_char,
    ) -> c_int {
        let len = (n.abs() + 1) as usize;
        let out = core::slice::from_raw_parts_mut(s as *mut u8, len);
        borg_io::borg_what_text(x, y, n, &mut *a, out)
    }
    pub unsafe extern "C" fn borg_note(what: *const c_char) {
        borg_io::borg_note_bytes(st(), cbytes(what))
    }
    pub unsafe extern "C" fn borg_warning(what: *const c_char) {
        borg_io::borg_warning_bytes(st(), cbytes(what))
    }
    pub unsafe extern "C" fn borg_keypress(k: keycode_t) -> c_int {
        borg_io::borg_keypress(st(), k)
    }
    pub unsafe extern "C" fn borg_save_keypress(kp: keypress) -> keypress {
        borg_io::borg_save_keypress(st(), kp)
    }
    pub unsafe extern "C" fn borg_keypresses(s: *const c_char) -> c_int {
        borg_io::borg_keypresses(st(), cbytes(s))
    }
    pub unsafe extern "C" fn borg_inkey(take: bool) -> keycode_t {
        borg_io::borg_inkey(st(), take)
    }
    pub unsafe extern "C" fn borg_flush() {
        borg_io::borg_flush(st())
    }
    pub unsafe extern "C" fn borg_queue_direction(k: keycode_t) {
        borg_io::borg_queue_direction(st(), k)
    }
    pub unsafe extern "C" fn borg_get_queued_direction() -> keycode_t {
        borg_io::borg_get_queued_direction(st())
    }
    /// Returns memory from mem_zalloc (callers mem_free it, as in C).
    pub unsafe extern "C" fn borg_massage_special_chars(name: *mut c_char) -> *mut c_char {
        let v = borg_io::borg_massage_special_chars(cbytes(name));
        let p = crate::headers::z_virt_h::mem_zalloc(v.len()) as *mut u8;
        core::ptr::copy_nonoverlapping(v.as_ptr(), p, v.len());
        p as *mut c_char
    }
    pub unsafe extern "C" fn borg_dump_recent_keys(num: c_int) {
        borg_io::borg_dump_recent_keys(st(), num)
    }
    pub unsafe extern "C" fn borg_init_io() {
        borg_io::borg_init_io(st())
    }
    pub unsafe extern "C" fn borg_free_io() {
        borg_io::borg_free_io(st())
    }
}

/// `borg_init_failure = true` (still c2rust, borg-init.c).
pub fn set_borg_init_failure(_st: &mut BorgState) {
    // SAFETY: plain store to a C-style global on the engine thread.
    unsafe { crate::c2rust::borg_init::borg_init_failure = true }
}

// ---------------------------------------------------------------------------
// Calls from converted code into still-c2rust functions. Each takes the
// caller's `&mut BorgState` so the borrow visibly escapes into the call (the
// c2rust code mutates the same state through `st()`).

macro_rules! c2r_void {
    ($($name:ident => $path:path;)*) => {$(
        pub fn $name(_st: &mut BorgState) {
            // SAFETY: plain call into transitional c2rust code.
            unsafe { $path() }
        }
    )*};
}
c2r_void! {
    borg_reset_settings => crate::c2rust::borg_init::borg_reset_settings;
    borg_notice_player => crate::c2rust::borg_trait::borg_notice_player;
    borg_prepare_race_class_info => crate::c2rust::borg_init::borg_prepare_race_class_info;
    borg_reinit_options => crate::c2rust::borg_init::borg_reinit_options;
}

/// `borg_parse(NULL)`
pub fn borg_parse_null(_st: &mut BorgState) {
    // SAFETY: NULL is the documented "flush" argument.
    unsafe { crate::c2rust::borg_messages::borg_parse(core::ptr::null_mut()) }
}

/// `borg_get_messages(&key, cursor)`
pub fn borg_get_messages(
    _st: &mut BorgState,
    key: &mut crate::headers::ui_event_h::keypress,
    cursor: crate::headers::z_type_h::loc,
) -> bool {
    // SAFETY: key is a valid out-pointer for the call.
    unsafe { crate::c2rust::borg_messages::borg_get_messages(key, cursor) }
}

/// `borg_think()`
pub fn borg_think(_st: &mut BorgState) -> bool {
    // SAFETY: plain call into transitional c2rust code.
    unsafe { crate::c2rust::borg_think::borg_think() }
}

/// C-ABI shims with the borg-item-activation.c names.
pub mod item_activation {
    use super::{cbytes, st};
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn borg_findact(name: *const c_char) -> c_int {
        let name = std::str::from_utf8(cbytes(name)).expect("ASCII activation name");
        crate::borg_item_activation::borg_findact(st(), name)
    }
    pub unsafe extern "C" fn borg_init_item_activation() {
        crate::borg_item_activation::borg_init_item_activation(st())
    }
}

/// C-ABI shims with the borg-cave-view.c names.
pub mod cave_view {
    use super::st;

    pub unsafe extern "C" fn borg_forget_view() {
        crate::borg_cave_view::borg_forget_view(st())
    }
    pub unsafe extern "C" fn borg_update_view() {
        crate::borg_cave_view::borg_update_view(st())
    }
}

/// C-ABI shims with the borg-item-val.c names.
pub mod item_val {
    use super::st;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_init_item_val() {
        crate::borg_item_val::borg_init_item_val(st())
    }
    pub unsafe extern "C" fn borg_lookup_kind(tval: c_int, sval: c_int) -> c_int {
        crate::borg_item_val::borg_lookup_kind(tval, sval)
    }
}

/// The name of trait `i` (`prefix_pref[i]`, still c2rust, borg-trait.c),
/// for `i < BI_MAX`.
pub fn trait_name(_st: &mut BorgState, i: usize) -> &'static [u8] {
    assert!(i < crate::headers::borg_trait_h::BI_MAX as usize);
    // SAFETY: prefix_pref is a static table of string literals with a
    // non-NULL entry for every trait below BI_MAX; nothing writes it.
    unsafe { cbytes(crate::c2rust::borg_trait::prefix_pref[i]) }
}

/// `borg_settings[i].setting_string` (still c2rust, borg-init.c); `None`
/// for the NULL entry that ends the table.
pub fn setting_string(_st: &mut BorgState, i: usize) -> Option<&'static [u8]> {
    // SAFETY: borg_settings is a static table of string literals ended by
    // a NULL entry (callers stop there); nothing writes the strings.
    unsafe {
        let p = crate::c2rust::borg_init::borg_settings[i].setting_string;
        (!p.is_null()).then(|| cbytes(p))
    }
}

/// A copy of `borg_items[i]` (borg-item.c, converted).
pub fn borg_item(st: &mut BorgState, i: i32) -> crate::headers::borg_item_h::borg_item {
    st.items.items[i as usize]
}

/// `borg_item_weight(&borg_items[i])` (borg-item.c, converted).
pub fn borg_item_weight(st: &mut BorgState, i: i32) -> i16 {
    crate::borg_item::borg_item_weight(&st.items.items[i as usize])
}

/// `borg_ego_has_random_power(&e_info[ego_idx])` (still c2rust,
/// borg-item-analyze.c).
pub fn borg_ego_has_random_power(_st: &mut BorgState, ego_idx: u8) -> bool {
    // SAFETY: e_info has z_info->e_max entries and item ego indexes come
    // from it; the callee only reads the ego.
    unsafe {
        crate::c2rust::borg_item_analyze::borg_ego_has_random_power(
            crate::headers::object_h::e_info.offset(isize::from(ego_idx)),
        )
    }
}

/// `borg_spell_stat()` (borg-magic.c, converted).
pub fn borg_spell_stat(_st: &mut BorgState) -> i32 {
    crate::borg_magic::borg_spell_stat()
}

/// `borg_can_cast()` (borg-magic.c, converted).
pub fn borg_can_cast(_st: &mut BorgState) -> bool {
    crate::borg_magic::borg_can_cast()
}

/// `borg_get_spell_entry(book, entry)` (borg-magic.c, converted): the
/// spell's `(level, power)`, or `None` for NULL.
pub fn borg_get_spell_entry(st: &mut BorgState, book: i32, entry: i32) -> Option<(u8, u8)> {
    crate::borg_magic::borg_get_spell_entry(st, book, entry).map(|i| {
        let m = &st.magic.magics[i];
        (m.level, m.power)
    })
}

/// C-ABI shims with the borg-formulas.c names.
pub mod formulas {
    use super::st;
    use crate::borg_formulas;
    use crate::headers::z_file_h::ang_file;
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn borg_load_formulas(fp: *mut ang_file) -> bool {
        borg_formulas::borg_load_formulas(st(), crate::engine::AngFile::from_raw(fp))
    }
    pub unsafe extern "C" fn borg_free_formulas() {
        borg_formulas::borg_free_formulas(st())
    }
    pub unsafe extern "C" fn borg_power_dynamic() -> i32 {
        borg_formulas::borg_power_dynamic(st())
    }
    /// The reason points into the Borg state; it stays valid until the
    /// formulas are loaded again or freed.
    pub unsafe extern "C" fn borg_prepared_dynamic(depth: c_int) -> *const c_char {
        borg_formulas::borg_prepared_dynamic(st(), depth).map_or(core::ptr::null(), |r| r.as_ptr())
    }
    /// As borg_prepared_dynamic().
    pub unsafe extern "C" fn borg_restock_dynamic(depth: c_int) -> *const c_char {
        borg_formulas::borg_restock_dynamic(st(), depth).map_or(core::ptr::null(), |r| r.as_ptr())
    }
}

/// C-ABI shims with the borg-formulas-calc.c names.
pub mod formulas_calc {
    use super::{cbytes, st};
    use crate::borg_formulas_calc;
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn parse_calculation_line(
        line: *mut c_char,
        full_line: *const c_char,
    ) -> c_int {
        borg_formulas_calc::parse_calculation_line(st(), cbytes(line), cbytes(full_line))
    }
    pub unsafe extern "C" fn borg_calculate_dynamic(formula: c_int, range_index: c_int) -> i32 {
        borg_formulas_calc::borg_calculate_dynamic(st(), formula, range_index)
    }
    pub unsafe extern "C" fn calculations_free() {
        borg_formulas_calc::calculations_free(st())
    }
}

/// `borg_fear_region[y][x]` (still c2rust, borg-danger.c).
pub fn borg_fear_region(_st: &mut BorgState, y: i32, x: i32) -> u16 {
    // SAFETY: plain read of a fixed-size C-style global; out-of-range
    // indices panic.
    unsafe { crate::c2rust::borg_danger::borg_fear_region[y as usize][x as usize] }
}

/// A copy of `borg_kills[i]` (still c2rust, borg-flow-kill.c).
pub fn borg_kill(_st: &mut BorgState, i: usize) -> crate::headers::borg_flow_kill_h::borg_kill {
    // SAFETY: borg_kills is allocated with 256 entries by
    // borg_init_flow_kill() before the Borg runs; the index is a u8 grid
    // field (`borg_grid.kill`).
    unsafe {
        assert!(!crate::c2rust::borg_flow_kill::borg_kills.is_null() && i < 256);
        *crate::c2rust::borg_flow_kill::borg_kills.add(i)
    }
}

/// `borg_race_name(r_idx)` (still c2rust, borg-flow-kill.c), as bytes.
pub fn borg_race_name(_st: &mut BorgState, r_idx: i32) -> Vec<u8> {
    // SAFETY: borg_race_name returns a static C string or a race name
    // from r_info, never NULL.
    unsafe { cbytes(crate::c2rust::borg_flow_kill::borg_race_name(r_idx)).to_vec() }
}

/// C-ABI shims with the borg-projection.c names.
pub mod projection {
    use super::st;
    use crate::borg_projection as p;
    use crate::headers::z_type_h::loc;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_los(y1: c_int, x1: c_int, y2: c_int, x2: c_int) -> bool {
        p::borg_los(st(), y1, x1, y2, x2)
    }
    pub unsafe extern "C" fn borg_projectable(y1: c_int, x1: c_int, y2: c_int, x2: c_int) -> bool {
        p::borg_projectable(st(), y1, x1, y2, x2)
    }
    pub unsafe extern "C" fn borg_offset_projectable(
        y1: c_int,
        x1: c_int,
        y2: c_int,
        x2: c_int,
    ) -> bool {
        p::borg_offset_projectable(st(), y1, x1, y2, x2)
    }
    pub unsafe extern "C" fn borg_projectable_pure(
        y1: c_int,
        x1: c_int,
        y2: c_int,
        x2: c_int,
    ) -> bool {
        p::borg_projectable_pure(st(), y1, x1, y2, x2)
    }
    pub unsafe extern "C" fn borg_projectable_dark(
        y1: c_int,
        x1: c_int,
        y2: c_int,
        x2: c_int,
    ) -> bool {
        p::borg_projectable_dark(st(), y1, x1, y2, x2)
    }
    /// `py`/`px` point to the caller's position variables.
    pub unsafe extern "C" fn borg_inc_motion(
        py: *mut c_int,
        px: *mut c_int,
        y1: c_int,
        x1: c_int,
        y2: c_int,
        x2: c_int,
    ) {
        p::borg_inc_motion(&mut *py, &mut *px, y1, x1, y2, x2)
    }
    pub unsafe extern "C" fn borg_distance(y: c_int, x: c_int, y2: c_int, x2: c_int) -> c_int {
        p::borg_distance(y, x, y2, x2)
    }
    pub unsafe extern "C" fn borg_target(t: loc, require_monster: bool) -> bool {
        p::borg_target(st(), t, require_monster)
    }
    pub unsafe extern "C" fn borg_target_unknown_wall(y: c_int, x: c_int) -> bool {
        p::borg_target_unknown_wall(st(), y, x)
    }
}

/// `borg_items[i]` (still c2rust, borg-item.c), copied out. Transitional:
/// goes away when borg_item converts. The copy's `note` field is a raw C
/// string pointer that safe code must not dereference.
pub fn item(st: &mut BorgState, i: usize) -> crate::headers::borg_item_h::borg_item {
    st.items.items[i]
}

/// C-ABI shims with the borg-cave-light.c names.
pub mod cave_light {
    use super::st;

    pub unsafe extern "C" fn borg_update_light() {
        crate::borg_cave_light::borg_update_light(st())
    }
}

/// `(borg_items[i].tval, borg_items[i].sval)` (still c2rust, borg-item.c).
pub fn borg_item_tval_sval(st: &mut BorgState, i: i32) -> (u8, u8) {
    assert!(i >= 0 && i < i32::from(crate::engine::z_info().pack_size));
    let it = &st.items.items[i as usize];
    (it.tval, it.sval)
}

/// C-ABI shims with the borg-magic.c names.
pub mod magic {
    use super::st;
    use crate::borg_magic as m;
    use crate::headers::borg_magic_h::{borg_magic, borg_spells};
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_spell_stat() -> c_int {
        m::borg_spell_stat()
    }
    pub unsafe extern "C" fn borg_can_cast() -> bool {
        m::borg_can_cast()
    }
    pub unsafe extern "C" fn borg_primarily_caster() -> bool {
        m::borg_primarily_caster()
    }
    pub unsafe extern "C" fn borg_heroism_level() -> c_int {
        m::borg_heroism_level(st())
    }
    pub unsafe extern "C" fn borg_get_book_num(sval: c_int) -> c_int {
        m::borg_get_book_num(sval)
    }
    pub unsafe extern "C" fn borg_is_dungeon_book(tval: c_int, sval: c_int) -> bool {
        m::borg_is_dungeon_book(tval, sval)
    }
    /// A pointer into `st().magic.magics` (stable until the next
    /// `borg_prepare_book_info`), or NULL.
    pub unsafe extern "C" fn borg_get_spell_entry(book: c_int, entry: c_int) -> *mut borg_magic {
        match m::borg_get_spell_entry(st(), book, entry) {
            Some(i) => &raw mut st().magic.magics[i],
            None => core::ptr::null_mut(),
        }
    }
    pub unsafe extern "C" fn borg_get_spell_power(spell: borg_spells) -> c_int {
        m::borg_get_spell_power(st(), spell)
    }
    pub unsafe extern "C" fn borg_spell_legal(spell: borg_spells) -> bool {
        m::borg_spell_legal(st(), spell)
    }
    pub unsafe extern "C" fn borg_spell_okay(spell: borg_spells) -> bool {
        m::borg_spell_okay(st(), spell)
    }
    pub unsafe extern "C" fn borg_spell_fail_rate(spell: borg_spells) -> c_int {
        m::borg_spell_fail_rate(st(), spell)
    }
    pub unsafe extern "C" fn borg_spell_okay_fail(spell: borg_spells, allow_fail: c_int) -> bool {
        m::borg_spell_okay_fail(st(), spell, allow_fail)
    }
    pub unsafe extern "C" fn borg_spell_fail(spell: borg_spells, allow_fail: c_int) -> bool {
        m::borg_spell_fail(st(), spell, allow_fail)
    }
    pub unsafe extern "C" fn borg_spell_legal_fail(spell: borg_spells, allow_fail: c_int) -> bool {
        m::borg_spell_legal_fail(st(), spell, allow_fail)
    }
    pub unsafe extern "C" fn borg_spell(spell: borg_spells) -> bool {
        m::borg_spell(st(), spell)
    }
    pub unsafe extern "C" fn borg_cheat_spells() {
        m::borg_cheat_spells(st())
    }
    pub unsafe extern "C" fn borg_prepare_book_info() {
        m::borg_prepare_book_info(st())
    }
}

/// A copy of `borg_shops[shop].ware[i]` (still c2rust, borg-store.c).
/// Transitional, like `item()`; the copy's `note` field is a raw C string
/// pointer that safe code must not dereference.
pub fn shop_item(
    _st: &mut BorgState,
    shop: usize,
    i: usize,
) -> crate::headers::borg_item_h::borg_item {
    let z = crate::engine::z_info();
    assert!(shop < usize::from(z.store_max) && i < usize::from(z.store_inven_max));
    // SAFETY: borg_init_store() allocates z_info->store_max shops, each
    // with z_info->store_inven_max wares, before the Borg runs (checked
    // non-null); the indices are checked above.
    unsafe {
        let shops = crate::c2rust::borg_store::borg_shops;
        assert!(!shops.is_null(), "borg_shops not allocated");
        let ware = (*shops.add(shop)).ware;
        assert!(!ware.is_null(), "borg_shops ware not allocated");
        *ware.add(i)
    }
}

/// `borg_calc_blows(item)` (still c2rust, borg-trait.c).
pub fn borg_calc_blows(_st: &mut BorgState, item: &crate::headers::borg_item_h::borg_item) -> i32 {
    let mut copy = *item;
    // SAFETY: the callee only reads the item (a local copy), the player
    // and the Borg's traits.
    unsafe { crate::c2rust::borg_trait::borg_calc_blows(&mut copy) }
}

/// `borg_equips_item(activation, check_charge)` (still c2rust,
/// borg-item-use.c).
pub fn borg_equips_item(_st: &mut BorgState, activation: i32, check_charge: bool) -> bool {
    // SAFETY: plain call into transitional c2rust code (it reads
    // borg.activation and the equipment).
    unsafe { crate::c2rust::borg_item_use::borg_equips_item(activation, check_charge) }
}

/// C-ABI shims with the borg-home-notice.c names.
pub mod home_notice {
    use super::st;
    use crate::headers::borg_item_h::borg_item;

    /// `in_item` may be NULL; it is only read (copied first).
    pub unsafe extern "C" fn borg_notice_home(in_item: *mut borg_item, no_items: bool) {
        let copy = if in_item.is_null() { None } else { Some(*in_item) };
        crate::borg_home_notice::borg_notice_home(st(), copy.as_ref(), no_items)
    }
}

/// C-ABI shims with the borg-home-power.c names.
pub mod home_power {
    use super::st;

    pub unsafe extern "C" fn borg_power_home() -> i32 {
        crate::borg_home_power::borg_power_home(st())
    }
}

/// C-ABI shim for c2rust callers of borg-magic-play.c (borg_think_dungeon).
pub mod magic_play {
    use super::st;

    pub unsafe extern "C" fn borg_play_magic(bored: bool) -> bool {
        crate::borg_magic_play::borg_play_magic(st(), bored)
    }
}

/// C-ABI shims with the borg-prepared.c names.
///
/// As in C, the returned string is a string literal (static), a formula
/// line's reason (valid until the formulas are reloaded or freed), or the
/// one static `borg_prepared_buffer`, which the next formatted reason
/// overwrites (callers that keep a reason across calls see that).
pub mod prepared {
    use super::st;
    use crate::borg_prepared::{self as p, Reason};
    use core::ffi::{c_char, c_int};
    use std::collections::HashMap;
    use std::ffi::CString;
    use std::sync::Mutex;

    /// `MAX_REASON`
    const MAX_REASON: usize = 1024;
    /// `borg_prepared_buffer`
    static mut PREPARED_BUFFER: [c_char; MAX_REASON] = [0; MAX_REASON];
    /// NUL-terminated copies of the fixed reasons (never freed, so the
    /// pointers stay valid like C string literals).
    static TEXTS: Mutex<Option<HashMap<&'static str, CString>>> = Mutex::new(None);

    unsafe fn c_reason(r: Option<Reason>) -> *const c_char {
        match r {
            None => core::ptr::null(),
            Some(Reason::Text(s)) => {
                let mut texts = TEXTS.lock().expect("not poisoned");
                let map = texts.get_or_insert_with(HashMap::new);
                map.entry(s).or_insert_with(|| CString::new(s).expect("no NUL")).as_ptr()
            }
            Some(Reason::Formatted(s)) => {
                // strnfmt(borg_prepared_buffer, MAX_REASON, ...)
                let buf = &mut *(&raw mut PREPARED_BUFFER);
                let bytes = s.as_bytes();
                let n = bytes.len().min(MAX_REASON - 1);
                for (d, &b) in buf.iter_mut().zip(&bytes[..n]) {
                    *d = b as c_char;
                }
                buf[n] = 0;
                buf.as_ptr()
            }
            Some(Reason::Depth(i)) => st().formulas.depth[i].reason.as_ptr(),
            Some(Reason::Restock(i)) => st().formulas.restock[i].reason.as_ptr(),
        }
    }

    pub unsafe extern "C" fn borg_prepared(depth: c_int) -> *const c_char {
        c_reason(p::borg_prepared(st(), depth))
    }
    pub unsafe extern "C" fn borg_restock(depth: c_int) -> *const c_char {
        c_reason(p::borg_restock(st(), depth))
    }
    pub unsafe extern "C" fn borg_must_return_to_town() -> *const c_char {
        c_reason(p::borg_must_return_to_town(st()))
    }
}

/// C-ABI shims with the borg-reincarnate.c names (callers in
/// borg_commands and borg_messages_react).
pub mod reincarnate {
    use super::st;

    pub unsafe extern "C" fn borg_force_reincarnate() {
        crate::borg_reincarnate::borg_force_reincarnate(st())
    }
    pub unsafe extern "C" fn borg_reincarnate_end() {
        crate::borg_reincarnate::borg_reincarnate_end(st())
    }
    pub unsafe extern "C" fn borg_reincarnate_start() {
        crate::borg_reincarnate::borg_reincarnate_start(st())
    }
}

// ---------------------------------------------------------------------------
// borg-item.c (converted); its `note` field is still a raw pointer.

/// A zeroed `borg_item` (what `mem_zalloc` gave the C arrays).
pub fn blank_borg_item() -> crate::headers::borg_item_h::borg_item {
    // SAFETY: borg_item is plain C data (integers, bools, arrays and one
    // nullable pointer); the all-zero bit pattern is valid for each field.
    unsafe { core::mem::zeroed() }
}

/// `borg_items[j].note[0]`. Transitional: `note` points into the item's
/// `desc` (set by borg_item_analyze, still c2rust). C dereferences it
/// without a check; a NULL note would crash C, so it panics here.
pub fn item_note_first_byte(st: &mut BorgState, j: usize) -> u8 {
    let p = st.items.items[j].note;
    assert!(!p.is_null(), "borg_items[{}].note is NULL", j);
    // SAFETY: a non-NULL note points at a NUL-terminated suffix of a
    // `desc` buffer of a live item (C semantics).
    unsafe { *p as u8 }
}

c2r_void! {
    borg_equipment => crate::c2rust::borg_inventory::borg_equipment;
    borg_inventory => crate::c2rust::borg_inventory::borg_inventory;
    borg_notice_store => crate::c2rust::borg_store::borg_notice_store;
}

/// C-ABI shims with the borg-item.c names.
pub mod item_fns {
    use super::st;
    use crate::borg_item;
    use crate::headers::borg_item_h::borg_item as BorgItemC;
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn borg_get_note(item: *const BorgItemC) -> *const c_char {
        if !(*item).note.is_null() {
            return (*item).note;
        }
        c"".as_ptr()
    }
    pub unsafe extern "C" fn borg_deinscribe(i: c_int) {
        borg_item::borg_deinscribe(st(), i)
    }
    pub unsafe extern "C" fn borg_item_weight(item: *mut BorgItemC) -> i16 {
        borg_item::borg_item_weight(&*item)
    }
    pub unsafe extern "C" fn borg_find_all_items() {
        borg_item::borg_find_all_items(st())
    }
    pub unsafe extern "C" fn borg_init_item() {
        borg_item::borg_init_item(st())
    }
    pub unsafe extern "C" fn borg_free_item() {
        borg_item::borg_free_item(st())
    }
}

/// `borg_kills_nxt` (still c2rust, borg-flow-kill.c).
pub fn borg_kills_nxt(_st: &mut BorgState) -> i16 {
    // SAFETY: plain read of an int16_t global; Borg thread.
    unsafe { crate::c2rust::borg_flow_kill::borg_kills_nxt }
}

/// `borg_takes[i].kidx` (still c2rust, borg-flow-take.c).
pub fn borg_take_kidx(_st: &mut BorgState, i: usize) -> u32 {
    // SAFETY: borg_takes is allocated with 256 entries by
    // borg_init_flow_take() before the Borg runs (checked); the index is a
    // u8 grid field (`borg_grid.take`).
    unsafe {
        let p = crate::c2rust::borg_flow_take::borg_takes;
        assert!(!p.is_null() && i < 256);
        (*p.add(i)).kidx
    }
}

/// `weapon_swap` (still c2rust, borg-trait-swap.c): the swap weapon's
/// slot + 1, or 0.
pub fn weapon_swap(_st: &mut BorgState) -> i32 {
    // SAFETY: plain read of an int global; Borg thread.
    unsafe { crate::c2rust::borg_trait_swap::weapon_swap }
}

/// `armour_swap` (still c2rust, borg-trait-swap.c): the swap armour's
/// slot + 1, or 0.
pub fn armour_swap(_st: &mut BorgState) -> i32 {
    // SAFETY: plain read of an int global; Borg thread.
    unsafe { crate::c2rust::borg_trait_swap::armour_swap }
}

/// `borg_needs_new_sea` (still c2rust, borg-flow-glyph.c).
pub fn borg_needs_new_sea(_st: &mut BorgState) -> bool {
    // SAFETY: plain read of a bool global; Borg thread.
    unsafe { crate::c2rust::borg_flow_glyph::borg_needs_new_sea }
}

/// `borg_get_note(&borg_items[i])` (borg-item.c), as bytes. The note
/// pointer is still raw (see borg_item.rs); copied out at once.
pub fn borg_get_note(st: &mut BorgState, i: usize) -> Vec<u8> {
    let it: *const crate::headers::borg_item_h::borg_item = &st.items.items[i];
    // SAFETY: `it` points at a live item; the note is NULL (-> "") or a
    // NUL-terminated suffix of its desc buffer.
    unsafe { cbytes(item_fns::borg_get_note(it)).to_vec() }
}

/// `borg_prepared(depth)` through the C-ABI shim, so the static
/// `borg_prepared_buffer` is written as in C (c2rust callers may hold a
/// pointer to it). `None` for NULL.
pub fn borg_prepared_c(_st: &mut BorgState, depth: i32) -> Option<Vec<u8>> {
    // SAFETY: the shim returns NULL or a NUL-terminated string that stays
    // valid until the next call; copied out at once.
    unsafe {
        let p = prepared::borg_prepared(depth);
        (!p.is_null()).then(|| cbytes(p).to_vec())
    }
}

/// C-ABI shims with the borg-log.c names (callers in borg_commands and
/// borg_messages_react).
pub mod log {
    use super::st;
    use crate::borg_log as l;
    use crate::headers::object_h::object;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_log_death() {
        l::borg_log_death(st())
    }
    pub unsafe extern "C" fn borg_log_death_data() {
        l::borg_log_death_data(st())
    }
    pub unsafe extern "C" fn borg_write_map(ask: bool) {
        l::borg_write_map(st(), ask)
    }
    /// `item2` is the game object of inventory slot `n` (borg_commands).
    pub unsafe extern "C" fn borg_display_item(item2: *mut object, n: c_int) {
        // object_flags(item2, f): a pure read, done before the safe code
        // (which clears the screen first) as in C.
        let mut f = [0u8; 5];
        crate::headers::obj_util_h::object_flags(item2, f.as_mut_ptr());
        l::borg_display_item(st(), &f, n)
    }
    pub unsafe extern "C" fn borg_display_status() {
        l::borg_display_status(st())
    }
}

/// C-ABI shims with the borg-messages-react.c names (callers in
/// borg_messages, borg_light and borg_think_dungeon_util).
pub mod messages_react {
    use super::{cbytes, st};
    use crate::borg_messages_react as r;
    use crate::headers::ui_event_h::keypress;
    use crate::headers::z_type_h::loc;
    use core::ffi::c_char;

    /// As in C, a trailing ',' of `buf` is cut in the caller's buffer
    /// (C casts away the `const`; the callers that pass string literals
    /// pass none ending in ',').
    pub unsafe extern "C" fn borg_react(msg: *const c_char, buf: *const c_char) {
        let n = cbytes(buf).len();
        if r::borg_react(st(), cbytes(msg), cbytes(buf)) {
            *(buf as *mut c_char).add(n - 1) = 0;
        }
    }
    pub unsafe extern "C" fn borg_react_prompted(
        buf: *const c_char,
        key: *mut keypress,
        cursor: loc,
    ) -> bool {
        match r::borg_react_prompted(st(), cbytes(buf), cursor) {
            Some(code) => {
                (*key).code = code;
                true
            }
            None => false,
        }
    }
    pub unsafe extern "C" fn borg_clear_reactions() {
        r::borg_clear_reactions(st())
    }
}

/// `borg_msg_buf` as a C pointer, for c2rust readers of the message queue
/// (`borg_msg_buf + borg_msg_pos[i]`). `borg_init_messages` allocates the
/// buffer once and it never grows, so the pointer stays valid until
/// `borg_free_messages`.
pub fn msg_buf() -> *mut c_char {
    st().msgs.buf.as_mut_ptr() as *mut c_char
}
