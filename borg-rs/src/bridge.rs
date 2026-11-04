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
    borg_log_death => crate::c2rust::borg_log::borg_log_death;
    borg_log_death_data => crate::c2rust::borg_log::borg_log_death_data;
    borg_clear_reactions => crate::c2rust::borg_messages_react::borg_clear_reactions;
    borg_force_reincarnate => crate::c2rust::borg_reincarnate::borg_force_reincarnate;
    borg_notice_player => crate::c2rust::borg_trait::borg_notice_player;
    borg_reincarnate_end => crate::c2rust::borg_reincarnate::borg_reincarnate_end;
    borg_display_status => crate::c2rust::borg_log::borg_display_status;
}

/// `borg_write_map(ask)`
pub fn borg_write_map(_st: &mut BorgState, ask: bool) {
    // SAFETY: plain call into transitional c2rust code.
    unsafe { crate::c2rust::borg_log::borg_write_map(ask) }
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

/// `borg_los(y1, x1, y2, x2)` (still c2rust, borg-projection.c).
pub fn borg_los(_st: &mut BorgState, y1: i32, x1: i32, y2: i32, x2: i32) -> bool {
    // SAFETY: plain call into transitional c2rust code.
    unsafe { crate::c2rust::borg_projection::borg_los(y1, x1, y2, x2) }
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

/// `parse_value(value, full_line)` (still c2rust, borg-formulas.c). It
/// parses a writable copy of `value` in place (as C did with its copy);
/// the mem_alloc'd result is copied out and freed.
pub fn parse_value(
    _st: &mut BorgState,
    value: &[u8],
    full_line: &[u8],
) -> Option<crate::headers::borg_formulas_h::value_sec> {
    let mut buf = value.to_vec();
    buf.push(0);
    let full = std::ffi::CString::new(full_line).expect("no NUL");
    // SAFETY: buf is a NUL-terminated writable copy; parse_value returns
    // NULL or a mem_alloc'd value_sec that nothing else references.
    unsafe {
        let v = crate::c2rust::borg_formulas::parse_value(
            buf.as_mut_ptr() as *mut c_char,
            full.as_ptr(),
        );
        if v.is_null() {
            return None;
        }
        let r = *v;
        crate::headers::z_virt_h::mem_free(v as *mut core::ffi::c_void);
        Some(r)
    }
}

/// `calculate_from_value(value, range_index)` (still c2rust,
/// borg-formulas.c).
pub fn calculate_from_value(
    _st: &mut BorgState,
    value: &crate::headers::borg_formulas_h::value_sec,
    range_index: i32,
) -> i32 {
    let mut v = *value;
    // SAFETY: v is a valid value_sec; the callee only reads it.
    unsafe { crate::c2rust::borg_formulas::calculate_from_value(&mut v, range_index) }
}

/// `borg_formula_error(section, full_line, label, error)` (still c2rust,
/// borg-formulas.c).
pub fn borg_formula_error(
    _st: &mut BorgState,
    section: &[u8],
    full_line: &[u8],
    label: &str,
    error: Option<&str>,
) {
    let section = std::ffi::CString::new(section).expect("no NUL");
    let full = std::ffi::CString::new(full_line).expect("no NUL");
    let label = std::ffi::CString::new(label).expect("no NUL");
    let error = error.map(|e| std::ffi::CString::new(e).expect("no NUL"));
    let error_ptr = error.as_ref().map_or(core::ptr::null(), |e| e.as_ptr());
    // SAFETY: all strings are NUL-terminated and outlive the call.
    unsafe {
        crate::c2rust::borg_formulas::borg_formula_error(
            section.as_ptr(),
            full.as_ptr(),
            label.as_ptr(),
            error_ptr,
        )
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
