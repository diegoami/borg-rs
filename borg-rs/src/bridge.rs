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
pub fn borg_oops(st: &mut BorgState, what: &str) {
    let c = std::ffi::CString::new(what).expect("no NUL");
    escape(st);
    // SAFETY: borg_oops reads the string and copies it into a note.
    unsafe { crate::c2rust::borg_think::borg_oops(c.as_ptr()) }
    escape(st);
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
pub fn set_borg_init_failure(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain store to a C-style global on the engine thread.
    unsafe { crate::c2rust::borg_init::borg_init_failure = true }
    escape(st);
}

// ---------------------------------------------------------------------------
// Calls from converted code into still-c2rust functions. Each takes the
// caller's `&mut BorgState` so the borrow visibly escapes into the call (the
// c2rust code mutates the same state through `st()`).

macro_rules! c2r_void {
    ($($name:ident => $path:path;)*) => {$(
        pub fn $name(st: &mut BorgState) {
            escape(st);
            // SAFETY: plain call into transitional c2rust code.
            unsafe { $path() }
            escape(st);
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
pub fn borg_parse_null(st: &mut BorgState) {
    escape(st);
    // SAFETY: NULL is the documented "flush" argument.
    unsafe { crate::c2rust::borg_messages::borg_parse(core::ptr::null_mut()) }
    escape(st);
}

/// `borg_get_messages(&key, cursor)`
pub fn borg_get_messages(
    st: &mut BorgState,
    key: &mut crate::headers::ui_event_h::keypress,
    cursor: crate::headers::z_type_h::loc,
) -> bool {
    escape(st);
    // SAFETY: key is a valid out-pointer for the call.
    let r = unsafe { crate::c2rust::borg_messages::borg_get_messages(key, cursor) };
    escape(st);
    r
}

/// `borg_think()`
pub fn borg_think(st: &mut BorgState) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_think::borg_think() };
    escape(st);
    r
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
pub fn setting_string(st: &mut BorgState, i: usize) -> Option<&'static [u8]> {
    escape(st);
    // SAFETY: borg_settings is a static table of string literals ended by
    // a NULL entry (callers stop there); nothing writes the strings.
    let r = unsafe {
        let p = crate::c2rust::borg_init::borg_settings[i].setting_string;
        (!p.is_null()).then(|| cbytes(p))
    };
    escape(st);
    r
}

/// A copy of `borg_items[i]` (borg-item.c, converted).
pub fn borg_item(st: &mut BorgState, i: i32) -> crate::headers::borg_item_h::borg_item {
    st.items.items[i as usize]
}

/// `borg_item_weight(&borg_items[i])` (borg-item.c, converted).
pub fn borg_item_weight(st: &mut BorgState, i: i32) -> i16 {
    crate::borg_item::borg_item_weight(&st.items.items[i as usize])
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

/// `borg_fear_region[y][x]` (borg-danger.c, converted).
pub fn borg_fear_region(st: &mut BorgState, y: i32, x: i32) -> u16 {
    st.danger.fear_region[y as usize][x as usize]
}

macro_rules! c2r_bool_global {
    ($($name:ident => $path:path;)*) => {$(
        #[doc = concat!("`", stringify!($name), "` (still c2rust).")]
        pub fn $name(_st: &mut BorgState) -> bool {
            // SAFETY: plain read of a bool global; Borg thread.
            unsafe { $path }
        }
    )*};
}
c2r_bool_global! {
    borg_attacking => crate::c2rust::borg_trait::borg_attacking;
    borg_on_glyph => crate::c2rust::borg_trait::borg_on_glyph;
    borg_create_door => crate::c2rust::borg_trait::borg_create_door;
    borg_sleep_spell => crate::c2rust::borg_trait::borg_sleep_spell;
    borg_sleep_spell_ii => crate::c2rust::borg_trait::borg_sleep_spell_ii;
    borg_crush_spell => crate::c2rust::borg_trait::borg_crush_spell;
    borg_slow_spell => crate::c2rust::borg_trait::borg_slow_spell;
    borg_confuse_spell => crate::c2rust::borg_trait::borg_confuse_spell;
    borg_fear_mon_spell => crate::c2rust::borg_trait::borg_fear_mon_spell;
    borg_simulate => crate::c2rust::borg_trait::borg_simulate;
}

/// `borg_simulate = v` (global of borg-trait.c, still c2rust).
pub fn set_borg_simulate(_st: &mut BorgState, v: bool) {
    // SAFETY: plain write of a bool global; Borg thread.
    unsafe { crate::c2rust::borg_trait::borg_simulate = v };
}

/// `borg_tp_other_n` (still c2rust, borg-fight-attack.c).
pub fn borg_tp_other_n(st: &mut BorgState) -> i16 {
    escape(st);
    // SAFETY: plain read of an int16_t global; Borg thread.
    let r = unsafe { crate::c2rust::borg_fight_attack::borg_tp_other_n };
    escape(st);
    r
}

/// `borg_tp_other_index[i]` (still c2rust, borg-fight-attack.c); panics
/// past the 255 entries.
pub fn borg_tp_other_index(st: &mut BorgState, i: usize) -> i32 {
    escape(st);
    // SAFETY: plain read of a fixed-size int array global; Borg thread.
    let r = unsafe { crate::c2rust::borg_fight_attack::borg_tp_other_index[i] };
    escape(st);
    r
}

/// `borg_mon_blow_effect(name)` (still c2rust, borg-fight-attack.c).
pub fn borg_mon_blow_effect(st: &mut BorgState, name: &std::ffi::CStr) -> i32 {
    escape(st);
    // SAFETY: the callee only compares the C string with its table.
    let r = unsafe { crate::c2rust::borg_fight_attack::borg_mon_blow_effect(name.as_ptr()) };
    escape(st);
    r
}

/// C-ABI shims with the borg-danger.c names, for c2rust callers.
pub mod danger {
    use super::st;
    use crate::borg_danger as d;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_danger_one_kill(
        y: c_int,
        x: c_int,
        c: c_int,
        i: c_int,
        average: bool,
        full_damage: bool,
    ) -> c_int {
        d::borg_danger_one_kill(st(), y, x, c, i, average, full_damage)
    }
    pub unsafe extern "C" fn borg_danger(
        y: c_int,
        x: c_int,
        c: c_int,
        average: bool,
        full_damage: bool,
    ) -> c_int {
        d::borg_danger(st(), y, x, c, average, full_damage)
    }
}

// ---------------------------------------------------------------------------
// borg-flow-kill.c (converted): calls into still-c2rust code and C-ABI shims.

/// Let the caller's `&mut BorgState` escape before a call into c2rust code.
///
/// An unused `_st` parameter is not enough once the wrapper is inlined: LLVM
/// then sees no use of the `noalias` borrow around the call and may keep
/// the caller's state in registers or drop stores the callee reads through
/// `st()` (borg_flow_kill_aim moves `borg.c` around such calls).
/// `black_box` makes the pointer escape into an opaque use, so the stores
/// happen before the call and the values are reloaded after it.
#[inline(always)]
fn escape(st: &mut BorgState) {
    std::hint::black_box(st as *mut BorgState);
}

/// `(borg_flow_y[0], borg_flow_x[0])` (still c2rust, borg-flow.c).
pub fn borg_flow_first(st: &mut BorgState) -> (i32, i32) {
    escape(st);
    // SAFETY: plain read of fixed-size u8 array globals; Borg thread.
    let r = unsafe {
        (
            i32::from(crate::c2rust::borg_flow::borg_flow_y[0]),
            i32::from(crate::c2rust::borg_flow::borg_flow_x[0]),
        )
    };
    escape(st);
    r
}

/// `borg_temp_n = 0` (still c2rust, borg-flow.c).
pub fn borg_temp_clear(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain store to an int16_t global; Borg thread.
    unsafe { crate::c2rust::borg_flow::borg_temp_n = 0 }
    escape(st);
}

/// `borg_temp_n` (still c2rust, borg-flow.c).
pub fn borg_temp_n(st: &mut BorgState) -> i16 {
    escape(st);
    // SAFETY: plain read of an int16_t global; Borg thread.
    let r = unsafe { crate::c2rust::borg_flow::borg_temp_n };
    escape(st);
    r
}

/// `borg_temp_x[borg_temp_n] = x; borg_temp_y[borg_temp_n] = y;
/// borg_temp_n++;` (still c2rust, borg-flow.c); panics past the 9000
/// entries.
pub fn borg_temp_push(st: &mut BorgState, y: i32, x: i32) {
    escape(st);
    // SAFETY: plain stores to fixed-size array globals (indexing is
    // bounds checked); Borg thread.
    unsafe {
        use crate::c2rust::borg_flow as f;
        let n = f::borg_temp_n as usize;
        (*(&raw mut f::borg_temp_x))[n] = x as u8;
        (*(&raw mut f::borg_temp_y))[n] = y as u8;
        f::borg_temp_n += 1;
    }
    escape(st);
}

/// `(borg_temp_y[i], borg_temp_x[i])` (still c2rust, borg-flow.c).
pub fn borg_temp_at(st: &mut BorgState, i: usize) -> (i32, i32) {
    escape(st);
    // SAFETY: plain reads of fixed-size array globals (bounds checked).
    let r = unsafe {
        use crate::c2rust::borg_flow as f;
        (i32::from((*(&raw const f::borg_temp_y))[i]), i32::from((*(&raw const f::borg_temp_x))[i]))
    };
    escape(st);
    r
}

/// `borg_flow_clear()` (still c2rust, borg-flow.c).
pub fn borg_flow_clear(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_flow::borg_flow_clear() };
    escape(st);
    r
}

/// `borg_flow_enqueue_grid(y, x)` (still c2rust, borg-flow.c).
pub fn borg_flow_enqueue_grid(st: &mut BorgState, y: i32, x: i32) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_flow::borg_flow_enqueue_grid(y, x) };
    escape(st);
    r
}

/// `borg_flow_spread(...)` (still c2rust, borg-flow.c).
pub fn borg_flow_spread(
    st: &mut BorgState,
    depth: i32,
    optimize: bool,
    avoid: bool,
    tunneling: bool,
    stair_idx: i32,
    sneak: bool,
) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe {
        crate::c2rust::borg_flow::borg_flow_spread(depth, optimize, avoid, tunneling, stair_idx, sneak)
    };
    escape(st);
    r
}

/// `borg_flow_commit(who, why)` (still c2rust, borg-flow.c).
pub fn borg_flow_commit(st: &mut BorgState, who: &std::ffi::CStr, why: i32) -> bool {
    escape(st);
    // SAFETY: the callee only reads the C string (for a note).
    let r = unsafe { crate::c2rust::borg_flow::borg_flow_commit(who.as_ptr(), why) };
    escape(st);
    r
}

/// `borg_flow_old(why)` (still c2rust, borg-flow.c).
pub fn borg_flow_old(st: &mut BorgState, why: i32) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_flow::borg_flow_old(why) };
    escape(st);
    r
}

/// `borg_can_dig(check_fail, feat)` (still c2rust, borg-flow.c).
pub fn borg_can_dig(st: &mut BorgState, check_fail: bool, feat: u8) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_flow::borg_can_dig(check_fail, feat) };
    escape(st);
    r
}

/// `borg_flow_far_from_stairs(x, y, b_stair)` (still c2rust,
/// borg-flow-misc.c).
pub fn borg_flow_far_from_stairs(st: &mut BorgState, x: i32, y: i32, b_stair: i32) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_flow_misc::borg_flow_far_from_stairs(x, y, b_stair) };
    escape(st);
    r
}

/// `borg_flow_reverse(...)` (still c2rust, borg-flow-misc.c).
pub fn borg_flow_reverse(
    st: &mut BorgState,
    depth: i32,
    optimize: bool,
    avoid: bool,
    tunneling: bool,
    stair_idx: i32,
    sneak: bool,
) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    unsafe {
        crate::c2rust::borg_flow_misc::borg_flow_reverse(
            depth, optimize, avoid, tunneling, stair_idx, sneak,
        )
    };
    escape(st);
}

/// `(borg_ddx_ddd[i], borg_ddy_ddd[i])` (still c2rust, borg-flow.c): the
/// 24 offsets around a grid; panics past them.
pub fn borg_ddxy_ddd(i: usize) -> (i32, i32) {
    // SAFETY: plain read of fixed-size int16_t tables that are never
    // written (bounds checked); Borg thread.
    unsafe {
        use crate::c2rust::borg_flow as f;
        (i32::from((*(&raw const f::borg_ddx_ddd))[i]), i32::from((*(&raw const f::borg_ddy_ddd))[i]))
    }
}

/// `borg_data_cost->data[y][x]` (still c2rust, borg-flow.c).
pub fn borg_data_cost_at(st: &mut BorgState, y: i32, x: i32) -> u8 {
    escape(st);
    // SAFETY: borg_init_flow() allocates borg_data_cost (checked non-NULL);
    // the fixed-size array indexes are bounds checked. Borg thread.
    let r = unsafe {
        let p = crate::c2rust::borg_flow::borg_data_cost;
        assert!(!p.is_null());
        (*p).data[y as usize][x as usize]
    };
    escape(st);
    r
}

/// `borg_count_sell()` (still c2rust, borg-store-sell.c).
pub fn borg_count_sell(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_store_sell::borg_count_sell() };
    escape(st);
    r
}

/// `borg_takes_nxt` (borg-flow-take.c, converted).
pub fn borg_takes_nxt(st: &mut BorgState) -> i16 {
    st.takes.nxt
}

/// A copy of `borg_takes[i]` (borg-flow-take.c, converted).
pub fn borg_take(st: &mut BorgState, i: usize) -> crate::headers::borg_flow_take_h::borg_take {
    assert!(i < 256 && !st.takes.takes.is_empty());
    st.takes.takes[i]
}

/// `borg_delete_take(i)` (borg-flow-take.c, converted).
pub fn borg_delete_take(st: &mut BorgState, i: i32) {
    crate::borg_flow_take::borg_delete_take(st, i)
}

/// `borg_attack_aux_spell_bolt(...)` (still c2rust, borg-fight-attack.c).
pub fn borg_attack_aux_spell_bolt(
    st: &mut BorgState,
    spell: crate::headers::borg_magic_h::borg_spells,
    rad: i32,
    dam: i32,
    typ: i32,
    max_range: i32,
    is_arc: bool,
) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe {
        crate::c2rust::borg_fight_attack::borg_attack_aux_spell_bolt(
            spell, rad, dam, typ, max_range, is_arc,
        )
    };
    escape(st);
    r
}

/// `borg_attack_aux_launch()` (still c2rust, borg-fight-attack.c).
pub fn borg_attack_aux_launch(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_fight_attack::borg_attack_aux_launch() };
    escape(st);
    r
}

/// `borg_equips_ring(ring_sval)` (borg-item-use.c, converted).
pub fn borg_equips_ring(st: &mut BorgState, ring_sval: i32) -> bool {
    crate::borg_item_use::borg_equips_ring(st, ring_sval)
}

/// `borg_detect_wall[y][x]` (still c2rust, borg-update.c). The map is
/// `(borg.panels.y + 1) x (borg.panels.x + 1)`.
pub fn borg_detect_wall(st: &mut BorgState, y: i32, x: i32) -> bool {
    assert!((0..=st.borg.panels.y).contains(&y) && (0..=st.borg.panels.x).contains(&x));
    // SAFETY: borg_alloc_detection() allocates panels.y + 1 rows of
    // panels.x + 1 bools (pointer checked non-NULL, indexes checked above).
    unsafe {
        let p = crate::c2rust::borg_update::borg_detect_wall;
        assert!(!p.is_null());
        let row = *p.add(y as usize);
        assert!(!row.is_null());
        *row.add(x as usize)
    }
}

/// C-ABI shims with the borg-flow-kill.c names, for c2rust callers.
pub mod flow_kill {
    use super::{cbytes, st};
    use crate::borg_flow_kill as fk;
    use crate::headers::__stddef_wchar_t_h::wchar_t;
    use crate::headers::z_type_h::loc;
    use core::ffi::{c_char, c_int};

    pub unsafe extern "C" fn borg_race_name(r_idx: c_int) -> *const c_char {
        fk::borg_race_name(r_idx).as_ptr()
    }
    pub unsafe extern "C" fn borg_delete_kill(i: c_int) {
        fk::borg_delete_kill(st(), i)
    }
    pub unsafe extern "C" fn borg_sleep_kill(i: c_int) {
        fk::borg_sleep_kill(st(), i)
    }
    pub unsafe extern "C" fn borg_follow_kill(i: c_int) {
        fk::borg_follow_kill(st(), i)
    }
    /// `a` and `c` (the screen attr/char) are not used by the C code.
    pub unsafe extern "C" fn observe_kill_diff(y: c_int, x: c_int, _a: u8, _c: wchar_t) -> bool {
        fk::observe_kill_diff(st(), y, x)
    }
    pub unsafe extern "C" fn observe_kill_move(
        y: c_int,
        x: c_int,
        d: c_int,
        a: u8,
        c: wchar_t,
        flag: bool,
    ) -> bool {
        fk::observe_kill_move(st(), y, x, d, a, c, flag)
    }
    pub unsafe extern "C" fn borg_locate_kill(who: *mut c_char, c: loc, r: c_int) -> c_int {
        fk::borg_locate_kill(st(), cbytes(who), c, r)
    }
    pub unsafe extern "C" fn borg_count_death(i: c_int) {
        fk::borg_count_death(st(), i)
    }
    pub unsafe extern "C" fn borg_flow_kill(viewable: bool, nearness: c_int) -> bool {
        fk::borg_flow_kill(st(), viewable, nearness)
    }
    pub unsafe extern "C" fn borg_flow_kill_aim(viewable: bool) -> bool {
        fk::borg_flow_kill_aim(st(), viewable)
    }
    pub unsafe extern "C" fn borg_flow_kill_corridor() -> bool {
        fk::borg_flow_kill_corridor(st())
    }
    pub unsafe extern "C" fn borg_flow_kill_direct(twitchy: bool) -> bool {
        fk::borg_flow_kill_direct(st(), twitchy)
    }
    pub unsafe extern "C" fn borg_near_monster_type(dist: c_int) {
        fk::borg_near_monster_type(st(), dist)
    }
    pub unsafe extern "C" fn borg_shoot_scoot_safe(emergency: c_int, turns: c_int, b_p: c_int) -> bool {
        fk::borg_shoot_scoot_safe(st(), emergency, turns, b_p)
    }
    pub unsafe extern "C" fn borg_create_kill(who: *mut c_char, c: loc) -> c_int {
        fk::borg_create_kill(st(), cbytes(who), c)
    }
    pub unsafe extern "C" fn borg_init_flow_kill() {
        fk::borg_init_flow_kill(st())
    }
    pub unsafe extern "C" fn borg_free_flow_kill() {
        fk::borg_free_flow_kill(st())
    }
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

/// A copy of `borg_shops[shop].ware[i]` (borg-store.c, converted). The
/// copy's `note` field is a raw C string pointer that safe code must not
/// dereference.
pub fn shop_item(st: &mut BorgState, shop: usize, i: usize) -> crate::headers::borg_item_h::borg_item {
    st.store.shops[shop].ware[i]
}

/// `borg_calc_blows(item)` (still c2rust, borg-trait.c).
pub fn borg_calc_blows(st: &mut BorgState, item: &crate::headers::borg_item_h::borg_item) -> i32 {
    let mut copy = *item;
    escape(st);
    // SAFETY: the callee only reads the item (a local copy), the player
    // and the Borg's traits.
    let r = unsafe { crate::c2rust::borg_trait::borg_calc_blows(&mut copy) };
    escape(st);
    r
}

/// `borg_equips_item(activation, check_charge)`: forwards to the safe
/// `borg_item_use::borg_equips_item` (kept for callers not yet switched).
pub fn borg_equips_item(st: &mut BorgState, activation: i32, check_charge: bool) -> bool {
    crate::borg_item_use::borg_equips_item(st, activation, check_charge)
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

/// The bytes of `borg_get_note(item)` (the inscription part of `desc`, or
/// "" when `note` is NULL). Transitional: `note` points into an item's
/// `desc` (set by borg_item_analyze, still c2rust); for a copied item it
/// still points into the original's `desc`, as in C.
pub fn item_note(item: &crate::headers::borg_item_h::borg_item) -> Vec<u8> {
    if item.note.is_null() {
        return Vec::new();
    }
    // SAFETY: a non-NULL note points at a NUL-terminated suffix of a live
    // item's `desc` buffer (C semantics; C reads it the same way).
    unsafe { cbytes(item.note).to_vec() }
}

/// `borg_item_analyze(&borg_items[i], obj, desc, in_store)` (borg-item-analyze.c,
/// converted). Kept for callers in conversions in flight; borg_inventory
/// calls the safe function directly.
pub fn borg_item_analyze(
    st: &mut BorgState,
    i: usize,
    obj: crate::engine::CarriedObject,
    desc: &[u8],
    in_store: bool,
) {
    crate::borg_item_analyze::borg_item_analyze_slot(st, i, obj.into(), desc, in_store)
}

/// `item->note = item->desc + offset` (borg-item-analyze.c). Transitional:
/// `note` stays a raw pointer into the item's own `desc` (C semantics:
/// copies of the item keep pointing at this item's `desc`).
pub fn set_item_note(item: &mut crate::headers::borg_item_h::borg_item, offset: usize) {
    assert!(offset < item.desc.len(), "note offset outside desc");
    // SAFETY: offset is inside the item's desc array (checked), so the
    // pointer stays in bounds of that allocation; nothing is dereferenced
    // here. It is dereferenced later by readers, as in C.
    item.note = unsafe { item.desc.as_mut_ptr().add(offset) };
}

/// C-ABI shims with the borg-item-analyze.c names, for c2rust callers.
pub mod item_analyze {
    use super::{cbytes, st};
    use crate::borg_item_analyze as ia;
    use crate::headers::borg_item_h::borg_item;
    use crate::headers::object_h::{ego_item, object};
    use core::ffi::{c_char, c_int};

    /// `item` is written in place (its `note` points into its own
    /// `desc`); `real_item` is a live object with or without a kind (no
    /// kind or no number: the item is just wiped); `desc` is a C string
    /// that is only read.
    pub unsafe extern "C" fn borg_item_analyze(
        item: *mut borg_item,
        real_item: *const object,
        desc: *mut c_char,
        in_store: bool,
    ) {
        let s = st();
        let desc = cbytes(desc).to_vec();
        ia::borg_item_analyze(
            &mut *item,
            crate::engine::GameObject::from_raw(real_item),
            &desc,
            in_store,
            &s.act,
            &s.item_val,
        )
    }
    pub unsafe extern "C" fn borg_obj_has_effect(kind: u32, index: c_int, subtype: c_int) -> bool {
        ia::borg_obj_has_effect(kind, index, subtype)
    }
    /// `e_ptr` points at an ego of `e_info` (only its kind flags are read).
    pub unsafe extern "C" fn borg_ego_has_random_power(e_ptr: *mut ego_item) -> bool {
        ia::ego_kind_flags_have_random_power(&(*e_ptr).kind_flags)
    }
}

/// `borg_item_note_needs_id(item)` (borg-item-id.c, converted). The
/// item's `note` must be NULL or point into a live item's `desc` (C
/// semantics; callers pass items of the Borg's arrays or copies of them
/// whose originals are still live).
pub fn borg_item_note_needs_id(
    _st: &mut BorgState,
    item: &crate::headers::borg_item_h::borg_item,
) -> bool {
    crate::borg_item_id::borg_item_note_needs_id(item)
}

/// C-ABI shims with the borg-inventory.c names.
pub mod inventory {
    use super::st;
    use crate::borg_inventory as inv;
    use crate::headers::borg_item_h::borg_item;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_wield_slot(item: *const borg_item) -> c_int {
        inv::borg_wield_slot(&*item)
    }
    pub unsafe extern "C" fn borg_slot(tval: c_int, sval: c_int) -> c_int {
        inv::borg_slot(st(), tval, sval)
    }
    pub unsafe extern "C" fn borg_equipment() {
        inv::borg_equipment(st())
    }
    pub unsafe extern "C" fn borg_inventory() {
        inv::borg_inventory(st())
    }
    pub unsafe extern "C" fn borg_first_empty_inventory_slot() -> c_int {
        inv::borg_first_empty_inventory_slot(st())
    }
    /// The item is copied first (it may be one of `st().items`); the
    /// copy's note still points into the live original.
    pub unsafe extern "C" fn borg_item_worth_id(item: *const borg_item) -> bool {
        let copy = *item;
        inv::borg_item_worth_id(st(), &copy)
    }
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

/// `weapon_swap` (still c2rust, borg-trait-swap.c): the swap weapon's
/// slot + 1, or 0.
pub fn weapon_swap(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain read of an int global; Borg thread.
    let r = unsafe { crate::c2rust::borg_trait_swap::weapon_swap };
    escape(st);
    r
}

/// `armour_swap` (still c2rust, borg-trait-swap.c): the swap armour's
/// slot + 1, or 0.
pub fn armour_swap(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain read of an int global; Borg thread.
    let r = unsafe { crate::c2rust::borg_trait_swap::armour_swap };
    escape(st);
    r
}

/// `enchant_weapon_swap_to_h` (still c2rust, borg-trait-swap.c): the
/// swap weapon's to-hit enchantment need.
pub fn enchant_weapon_swap_to_h(_st: &mut BorgState) -> i32 {
    // SAFETY: plain read of an int global; Borg thread.
    unsafe { crate::c2rust::borg_trait_swap::enchant_weapon_swap_to_h }
}

/// `enchant_weapon_swap_to_d` (still c2rust, borg-trait-swap.c): the
/// swap weapon's to-dam enchantment need.
pub fn enchant_weapon_swap_to_d(_st: &mut BorgState) -> i32 {
    // SAFETY: plain read of an int global; Borg thread.
    unsafe { crate::c2rust::borg_trait_swap::enchant_weapon_swap_to_d }
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
pub fn borg_prepared_c(st: &mut BorgState, depth: i32) -> Option<Vec<u8>> {
    escape(st);
    // SAFETY: the shim returns NULL or a NUL-terminated string that stays
    // valid until the next call; copied out at once.
    let r = unsafe {
        let p = prepared::borg_prepared(depth);
        (!p.is_null()).then(|| cbytes(p).to_vec())
    };
    escape(st);
    r
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
/// C-ABI shims with the borg-item-id.c names, for c2rust callers.
pub mod item_id {
    use super::st;
    use crate::borg_item_id as id;
    use crate::headers::borg_item_h::borg_item;

    pub unsafe extern "C" fn borg_object_fully_id() -> bool {
        id::borg_object_fully_id(st())
    }
    /// `item` is only read.
    pub unsafe extern "C" fn borg_item_note_needs_id(item: *const borg_item) -> bool {
        id::borg_item_note_needs_id(&*item)
    }
}


/// `borg_first_empty_inventory_slot()` (still c2rust, borg-inventory.c),
/// for borg-store.c.
pub fn store_first_empty_inventory_slot(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional code (it reads the items and
    // traits).
    let r = unsafe { crate::headers::borg_inventory_h::borg_first_empty_inventory_slot() };
    escape(st);
    r
}

/// `borg_init_store_sell()` (still c2rust, borg-store-sell.c).
pub fn borg_init_store_sell(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    unsafe { crate::headers::borg_store_sell_h::borg_init_store_sell() }
    escape(st);
}

/// `borg_free_store_sell()` (still c2rust, borg-store-sell.c).
pub fn borg_free_store_sell(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    unsafe { crate::headers::borg_store_sell_h::borg_free_store_sell() }
    escape(st);
}

/// `borg_item_analyze(&borg_shops[shop].ware[slot], obj, desc, in_store)`
/// (still c2rust, borg-item-analyze.c), for borg_notice_store.
pub fn borg_item_analyze_ware(
    st: &mut BorgState,
    shop: usize,
    slot: usize,
    obj: crate::engine::StoreObject,
    desc: &[u8],
    in_store: bool,
) {
    let mut buf = desc.to_vec();
    buf.push(0);
    let item: *mut crate::headers::borg_item_h::borg_item = &mut st.store.shops[shop].ware[slot];
    // SAFETY: item points at a live ware (the callee does not reallocate
    // the shops); obj is a live store object (engine::StoreObject); buf is
    // NUL-terminated and outlives the call, which copies it.
    unsafe {
        crate::headers::borg_item_analyze_h::borg_item_analyze(
            item,
            obj.as_ptr(),
            buf.as_mut_ptr() as *mut c_char,
            in_store,
        )
    }
}

// ---------------------------------------------------------------------------
// borg-flow-take.c (converted): calls into still-c2rust flow code.

/// `borg_get_leash(pick_up)` (still c2rust, borg-flow-misc.c).
pub fn borg_get_leash(st: &mut BorgState, pick_up: bool) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional c2rust code (reads traits).
    let r = unsafe { crate::c2rust::borg_flow_misc::borg_get_leash(pick_up) };
    escape(st);
    r
}

c2r_void! {
}

/// The panel detection arrays of borg-update.c (still c2rust):
/// `borg_detect_wall/trap/door/evil/obj`.
#[derive(Clone, Copy)]
pub enum Detect {
    Wall,
    Trap,
    Door,
    Evil,
    Obj,
}

/// `borg_detect_<what>[y][x]` (still c2rust, borg-update.c). Panics when
/// the arrays are not allocated or the panel is past them (C would read
/// through NULL or out of bounds).
pub fn borg_detect(_st: &mut BorgState, what: Detect, y: i32, x: i32) -> bool {
    use crate::c2rust::borg_update as u;
    // SAFETY: borg_alloc_detection() allocates `old_panels.y + 1` rows of
    // `old_panels.x + 1` bools for each array and records the sizes in
    // `old_panels`; borg_free_detection() nulls the arrays (checked). The
    // indexes are checked against those sizes. Borg thread.
    unsafe {
        let arr = match what {
            Detect::Wall => u::borg_detect_wall,
            Detect::Trap => u::borg_detect_trap,
            Detect::Door => u::borg_detect_door,
            Detect::Evil => u::borg_detect_evil,
            Detect::Obj => u::borg_detect_obj,
        };
        let size = u::old_panels;
        assert!(!arr.is_null(), "borg_detect: arrays not allocated");
        assert!((0..=size.y).contains(&y) && (0..=size.x).contains(&x), "borg_detect: out of bounds");
        let row = *arr.add(y as usize);
        assert!(!row.is_null());
        *row.add(x as usize)
    }
}

/// `borg_temp_n = 0` (still c2rust, borg-flow.c).
pub fn borg_temp_reset(st: &mut BorgState) {
    escape(st);
    // SAFETY: plain store to an int16_t global; Borg thread.
    unsafe { crate::c2rust::borg_flow::borg_temp_n = 0 }
    escape(st);
}

/// C-ABI shims with the borg-store.c names.
pub mod store {
    use super::st;
    use crate::borg_store as s;

    pub unsafe extern "C" fn borg_home_full() -> bool {
        s::borg_home_full(st())
    }
    pub unsafe extern "C" fn borg_inventory_full() -> bool {
        s::borg_inventory_full(st())
    }
    pub unsafe extern "C" fn borg_notice_store() {
        s::borg_notice_store(st())
    }
    pub unsafe extern "C" fn borg_init_store() {
        s::borg_init_store(st())
    }
    pub unsafe extern "C" fn borg_free_store() {
        s::borg_free_store(st())
    }
}
/// C-ABI shims with the borg-flow-take.c names, for c2rust callers.
pub mod flow_take {
    use super::st;
    use crate::borg_flow_take as t;
    use crate::headers::__stddef_wchar_t_h::wchar_t;
    use crate::headers::cave_h::chunk;
    use crate::headers::object_h::object;
    use crate::headers::z_type_h::loc;
    use core::ffi::c_int;

    /// Only called with the current level (`cave`), as in C.
    pub unsafe extern "C" fn borg_get_top_object(c: *mut chunk, grid: loc) -> *mut object {
        assert!(c == crate::engine::cave_ptr(), "borg_get_top_object: not the current level");
        crate::engine::top_floor_object(grid.x, grid.y)
            .map_or(core::ptr::null_mut(), |o| o.as_ptr())
    }
    pub unsafe extern "C" fn borg_delete_take(i: c_int) {
        t::borg_delete_take(st(), i)
    }
    pub unsafe extern "C" fn borg_follow_take(i: c_int) {
        t::borg_follow_take(st(), i)
    }
    pub unsafe extern "C" fn observe_take_diff(y: c_int, x: c_int, a: u8, c: wchar_t) -> bool {
        t::observe_take_diff(st(), y, x, a, c)
    }
    pub unsafe extern "C" fn observe_take_move(
        y: c_int,
        x: c_int,
        d: c_int,
        a: u8,
        c: wchar_t,
    ) -> bool {
        t::observe_take_move(st(), y, x, d, a, c)
    }
    pub unsafe extern "C" fn borg_flow_take(viewable: bool, nearness: c_int) -> bool {
        t::borg_flow_take(st(), viewable, nearness)
    }
    pub unsafe extern "C" fn borg_flow_take_scum(viewable: bool, nearness: c_int) -> bool {
        t::borg_flow_take_scum(st(), viewable, nearness)
    }
    pub unsafe extern "C" fn borg_flow_take_lunal(viewable: bool, nearness: c_int) -> bool {
        t::borg_flow_take_lunal(st(), viewable, nearness)
    }
    pub unsafe extern "C" fn borg_init_flow_take() {
        t::borg_init_flow_take(st())
    }
    pub unsafe extern "C" fn borg_free_flow_take() {
        t::borg_free_flow_take(st())
    }
}


/// C-ABI shims with the borg-light.c names, for c2rust callers.
pub mod light {
    use super::st;
    use crate::borg_light as l;
    use crate::headers::borg_item_use_h::borg_need;

    pub unsafe extern "C" fn borg_check_light_only() -> bool {
        l::borg_check_light_only(st())
    }
    pub unsafe extern "C" fn borg_check_light() -> bool {
        l::borg_check_light(st())
    }
    pub unsafe extern "C" fn borg_maintain_light() -> borg_need {
        l::borg_maintain_light(st()) as borg_need
    }
    pub unsafe extern "C" fn borg_light_beam(simulation: bool) -> bool {
        l::borg_light_beam(st(), simulation)
    }
}

/// C-ABI shims with the borg-item-use.c names, for c2rust callers.
pub mod item_use {
    use super::st;
    use crate::borg_item_use as u;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_quaff_crit(no_check: bool) -> bool {
        u::borg_quaff_crit(st(), no_check)
    }
    pub unsafe extern "C" fn borg_quaff_potion(sval: c_int) -> bool {
        u::borg_quaff_potion(st(), sval)
    }
    pub unsafe extern "C" fn borg_quaff_unknown() -> bool {
        u::borg_quaff_unknown(st())
    }
    pub unsafe extern "C" fn borg_read_scroll(sval: c_int) -> bool {
        u::borg_read_scroll(st(), sval)
    }
    pub unsafe extern "C" fn borg_read_unknown() -> bool {
        u::borg_read_unknown(st())
    }
    pub unsafe extern "C" fn borg_eat(tval: c_int, sval: c_int) -> bool {
        u::borg_eat(st(), tval, sval)
    }
    pub unsafe extern "C" fn borg_eat_unknown() -> bool {
        u::borg_eat_unknown(st())
    }
    pub unsafe extern "C" fn borg_eat_food_any() -> bool {
        u::borg_eat_food_any(st())
    }
    pub unsafe extern "C" fn borg_equips_rod(sval: c_int) -> bool {
        u::borg_equips_rod(st(), sval)
    }
    pub unsafe extern "C" fn borg_zap_rod(sval: c_int) -> bool {
        u::borg_zap_rod(st(), sval)
    }
    pub unsafe extern "C" fn borg_use_staff(sval: c_int) -> bool {
        u::borg_use_staff(st(), sval)
    }
    pub unsafe extern "C" fn borg_use_unknown() -> bool {
        u::borg_use_unknown(st())
    }
    pub unsafe extern "C" fn borg_use_staff_fail(sval: c_int) -> bool {
        u::borg_use_staff_fail(st(), sval)
    }
    pub unsafe extern "C" fn borg_equips_staff_fail(sval: c_int) -> bool {
        u::borg_equips_staff_fail(st(), sval)
    }
    pub unsafe extern "C" fn borg_aim_wand(sval: c_int) -> bool {
        u::borg_aim_wand(st(), sval)
    }
    pub unsafe extern "C" fn borg_equips_ring(ring_sval: c_int) -> bool {
        u::borg_equips_ring(st(), ring_sval)
    }
    pub unsafe extern "C" fn borg_activate_ring(ring_sval: c_int) -> bool {
        u::borg_activate_ring(st(), ring_sval)
    }
    pub unsafe extern "C" fn borg_equips_dragon(drag_sval: c_int) -> bool {
        u::borg_equips_dragon(st(), drag_sval)
    }
    pub unsafe extern "C" fn borg_activate_dragon(drag_sval: c_int) -> bool {
        u::borg_activate_dragon(st(), drag_sval)
    }
    pub unsafe extern "C" fn borg_activate_item(activation: c_int) -> bool {
        u::borg_activate_item(st(), activation)
    }
    pub unsafe extern "C" fn borg_equips_item(activation: c_int, check_charge: bool) -> bool {
        u::borg_equips_item(st(), activation, check_charge)
    }
    pub unsafe extern "C" fn borg_activate_failure(tval: c_int, sval: c_int) -> c_int {
        u::borg_activate_failure(st(), tval, sval)
    }
    pub unsafe extern "C" fn borg_use_things() -> bool {
        u::borg_use_things(st())
    }
    pub unsafe extern "C" fn borg_recharging() -> bool {
        u::borg_recharging(st())
    }
}

// ---------------------------------------------------------------------------
// borg-item-wear.c (converted): calls into still-c2rust code.

/// `borg_notice(notice_swap)` (still c2rust, borg-trait.c).
pub fn borg_notice(st: &mut BorgState, notice_swap: bool) {
    escape(st);
    // SAFETY: plain call into transitional c2rust code (reads the items,
    // writes the traits).
    unsafe { crate::c2rust::borg_trait::borg_notice(notice_swap) }
    escape(st);
}

/// `borg_power()` (still c2rust, borg-power.c).
pub fn borg_power(st: &mut BorgState) -> i32 {
    escape(st);
    // SAFETY: plain call into transitional c2rust code.
    let r = unsafe { crate::c2rust::borg_power::borg_power() };
    escape(st);
    r
}

/// `borg_check_rest(y, x)` (still c2rust, borg-flow-misc.c).
pub fn borg_check_rest(st: &mut BorgState, y: i32, x: i32) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code; (y, x) is the
    // Borg's grid.
    let r = unsafe { crate::c2rust::borg_flow_misc::borg_check_rest(y, x) };
    escape(st);
    r
}

/// `borg_read_scroll(sval)` (borg-item-use.c, converted).
pub fn borg_read_scroll(st: &mut BorgState, sval: i32) -> bool {
    crate::borg_item_use::borg_read_scroll(st, sval)
}

/// `shop_num` (still c2rust, borg-think.c): the store the Borg is in, or -1.
pub fn shop_num(st: &mut BorgState) -> i16 {
    escape(st);
    // SAFETY: plain read of an int16_t global; Borg thread.
    let r = unsafe { crate::c2rust::borg_think::shop_num };
    escape(st);
    r
}

/// C-ABI shims with the borg-item-wear.c names, for c2rust callers
/// (borg_think_dungeon, borg_think_dungeon_util, borg_think_store,
/// borg_caution, borg_trait, borg_update, borg_commands, borg_init).
pub mod item_wear {
    use super::st;
    use crate::borg_item_wear as w;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_test_stuff() -> bool {
        w::borg_test_stuff(st())
    }
    pub unsafe extern "C" fn borg_swap_rings() -> bool {
        w::borg_swap_rings(st())
    }
    pub unsafe extern "C" fn borg_wear_rings() -> bool {
        w::borg_wear_rings(st())
    }
    pub unsafe extern "C" fn borg_backup_swap(p: c_int) -> bool {
        w::borg_backup_swap(st(), p)
    }
    pub unsafe extern "C" fn borg_wear_stuff() -> bool {
        w::borg_wear_stuff(st())
    }
    pub unsafe extern "C" fn borg_best_stuff() -> bool {
        w::borg_best_stuff(st())
    }
    pub unsafe extern "C" fn borg_clear_best() {
        w::borg_clear_best(st())
    }
    pub unsafe extern "C" fn borg_wear_recharge() -> bool {
        w::borg_wear_recharge(st())
    }
    pub unsafe extern "C" fn borg_is_ammo(tval: c_int) -> bool {
        w::borg_is_ammo(tval)
    }
    pub unsafe extern "C" fn borg_init_item_wear() {
        w::borg_init_item_wear(st())
    }
    pub unsafe extern "C" fn borg_free_item_wear() {
        w::borg_free_item_wear(st())
    }
}

// ---------------------------------------------------------------------------
// borg-item-decurse.c (converted): calls into still-c2rust code.

/// `borg_uses_swaps()` (still c2rust, borg-trait.c).
pub fn borg_uses_swaps(st: &mut BorgState) -> bool {
    escape(st);
    // SAFETY: plain call into transitional c2rust code (reads the config
    // and traits).
    let r = unsafe { crate::c2rust::borg_trait::borg_uses_swaps() };
    escape(st);
    r
}

/// `decurse_armour_swap` (still c2rust, borg-trait-swap.c): the swap
/// armour is good except for its curse.
pub fn decurse_armour_swap(st: &mut BorgState) -> bool {
    escape(st);
    // SAFETY: plain read of a bool global; Borg thread.
    let r = unsafe { crate::c2rust::borg_trait_swap::decurse_armour_swap };
    escape(st);
    r
}

/// `decurse_weapon_swap` (still c2rust, borg-trait-swap.c): the swap
/// weapon is good except for its curse.
pub fn decurse_weapon_swap(st: &mut BorgState) -> bool {
    escape(st);
    // SAFETY: plain read of a bool global; Borg thread.
    let r = unsafe { crate::c2rust::borg_trait_swap::decurse_weapon_swap };
    escape(st);
    r
}

/// C-ABI shims with the borg-item-enchant.c names, for c2rust callers
/// (borg_think_dungeon).
pub mod item_enchant {
    use super::st;
    use crate::borg_item_enchant as e;

    pub unsafe extern "C" fn borg_enchanting() -> bool {
        e::borg_enchanting(st())
    }
}

/// C-ABI shims with the borg-recover.c names, for c2rust callers
/// (borg_think_dungeon).
pub mod recover {
    use super::st;
    use crate::borg_recover as r;

    pub unsafe extern "C" fn borg_recover() -> bool {
        r::borg_recover(st())
    }
}

/// C-ABI shims with the borg-flow-stairs.c names, for c2rust callers.
pub mod flow_stairs {
    use super::st;
    use crate::borg_flow_stairs as fs;
    use core::ffi::c_int;

    pub unsafe extern "C" fn borg_flow_cost_stair(y: c_int, x: c_int, b_stair: c_int) -> c_int {
        fs::borg_flow_cost_stair(st(), y, x, b_stair)
    }
    pub unsafe extern "C" fn borg_flow_stair_both(why: c_int, sneak: bool) -> bool {
        fs::borg_flow_stair_both(st(), why, sneak)
    }
    pub unsafe extern "C" fn borg_flow_stair_less(why: c_int, sneak: bool) -> bool {
        fs::borg_flow_stair_less(st(), why, sneak)
    }
    pub unsafe extern "C" fn borg_flow_stair_more(why: c_int, sneak: bool, brave: bool) -> bool {
        fs::borg_flow_stair_more(st(), why, sneak, brave)
    }
    pub unsafe extern "C" fn borg_prep_leave_level_spells() -> bool {
        fs::borg_prep_leave_level_spells(st())
    }
    pub unsafe extern "C" fn borg_init_flow_stairs() {
        fs::borg_init_flow_stairs(st())
    }
    pub unsafe extern "C" fn borg_free_flow_stairs() {
        fs::borg_free_flow_stairs(st())
    }
}

/// C-ABI shims with the borg-fight-perm.c names, for c2rust callers
/// (borg_think_dungeon).
pub mod fight_perm {
    use super::st;
    use crate::borg_fight_perm as p;

    pub unsafe extern "C" fn borg_perma_spell() -> bool {
        p::borg_perma_spell(st())
    }
}

/// C-ABI shims with the borg-flow-glyph.c names, for c2rust callers.
pub mod flow_glyph {
    use super::st;
    use crate::borg_flow_glyph as fg;

    pub unsafe extern "C" fn borg_flow_glyph() -> bool {
        fg::borg_flow_glyph(st())
    }
    pub unsafe extern "C" fn borg_init_flow_glyph() {
        fg::borg_init_flow_glyph(st())
    }
    pub unsafe extern "C" fn borg_free_flow_glyph() {
        fg::borg_free_flow_glyph(st())
    }
}
