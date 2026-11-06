//! Transitional adapters between converted (safe) modules and the
//! not-yet-converted c2rust code. Everything here goes away when the last
//! c2rust module is converted.

use core::ffi::c_char;

use crate::state::BorgState;

static mut STATE: *mut BorgState = core::ptr::null_mut();

/// The one Borg state, as a raw pointer. Allocated on first use and never
/// freed: the C globals lived for the whole process too.
pub(crate) fn state_ptr() -> *mut BorgState {
    // SAFETY: single-threaded (the engine thread); the pointer is created
    // once and never freed or moved.
    unsafe {
        if STATE.is_null() {
            STATE = Box::into_raw(Box::default());
        }
        STATE
    }
}

/// The one Borg state as `&mut`, for code that is still c2rust and for
/// the C-ABI shims. Transitional: c2rust code and converted code share the
/// state the way they shared the C globals, and this goes away with
/// `bridge.rs`.
///
/// # Safety
///
/// The result aliases every other `&mut BorgState` handed out (by earlier
/// `st()` calls, and the `st` parameter of converted code). Callers must
/// not use another reference into the state while the returned one is in
/// use, and code that holds one across a call that may reach `st()` (any
/// c2rust function or C-ABI shim) must pass it through `escape` before and
/// after that call, so the compiler does not keep state in registers
/// across it. Under Rust's aliasing model this is still undefined
/// behaviour; `escape` only keeps today's optimiser from exploiting it
/// (see docs/REVIEW.md). Engine thread only.
pub(crate) unsafe fn st() -> &'static mut BorgState {
    &mut *state_ptr()
}

static mut ENTRY_DEPTH: u32 = 0;

/// Run an engine callback into the Borg (the `inkey_hack` hook, an event
/// handler) with the state. This is where a borrow of the state starts:
/// nothing else holds one when the engine calls in.
///
/// Except in one case, which C allows too: a Borg call into the engine
/// that waits for a key (a long `msg()` with `auto_more` off goes through
/// `msg_flush` -> `anykey` -> `inkey_ex` -> the hook) runs a whole nested
/// Borg turn while the outer one is still running. The nested turn is kept,
/// as in C, but it aliases the outer `&mut` (see `st`). It is reported on
/// stderr, which the harness keeps per run (`stderr.log`), so the suites
/// show whether it ever happens.
pub(crate) fn with_entry_state<R>(f: impl FnOnce(&mut BorgState) -> R) -> R {
    // SAFETY: engine thread only; the counter is only touched here.
    let depth = unsafe {
        ENTRY_DEPTH += 1;
        ENTRY_DEPTH
    };
    if depth > 1 {
        eprintln!("borg-rs: nested Borg entry (depth {depth})");
    }
    // SAFETY: at depth 1 no other reference into the state is live (the
    // engine called in from outside the Borg). At depth > 1 see above.
    let r = f(unsafe { st() });
    // SAFETY: as above.
    unsafe { ENTRY_DEPTH -= 1 };
    r
}


unsafe fn cbytes<'a>(p: *const c_char) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

// ---------------------------------------------------------------------------
// Calls from converted code into still-c2rust functions. Each takes the
// caller's `&mut BorgState` so the borrow visibly escapes into the call (the
// c2rust code mutates the same state through `st()`).






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

/// `msg(text)` from Borg code that holds the state. The engine may wait
/// for a key here and so run a nested Borg turn (see `with_entry_state`),
/// so the state escapes around the call like around a c2rust call.
pub fn msg(st: &mut BorgState, text: &[u8]) {
    escape(st);
    crate::engine::msg(text);
    escape(st);
}

/// `get_string(prompt, buf, len)` from Borg code that holds the state
/// (waits for keys, so as `msg`).
pub fn get_string(st: &mut BorgState, prompt: &std::ffi::CStr, buf: &mut [u8], len: usize) -> bool {
    escape(st);
    let r = crate::engine::get_string(prompt, buf, len);
    escape(st);
    r
}

/// `event_signal(ev)` from Borg code that holds the state (a message flush
/// can wait for a key, so as `msg`).
pub fn event_signal(st: &mut BorgState, ev: crate::headers::game_event_h::game_event_type) {
    escape(st);
    crate::engine::event_signal(ev);
    escape(st);
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


/// `borg_get_note(&borg_items[i])` (borg-item.c), as bytes. The note
/// pointer is still raw (see borg_item.rs); copied out at once.
pub fn borg_get_note(st: &mut BorgState, i: usize) -> Vec<u8> {
    item_note(&st.items.items[i])
}









// ---------------------------------------------------------------------------
// borg-item-wear.c (converted): calls into still-c2rust code.


// ---------------------------------------------------------------------------
// borg-item-decurse.c (converted): calls into still-c2rust code.

// ---------------------------------------------------------------------------
// borg-flow-dark.c (converted): calls into still-c2rust flow code.






// ---------------------------------------------------------------------------
// borg-store-buy.c (converted): calls into still-c2rust code.





// ---------------------------------------------------------------------------
// borg-caution.c (converted): calls into still-c2rust code.





// ---------------------------------------------------------------------------
// borg-commands.c (converted): the engine's entry into the ^Z commands,
// engine calls that wait for keys, and calls into still-c2rust code.

/// The engine's call into the Borg commands (`do_cmd_borg`, called by
/// `do_cmd_try_borg` in cmd-misc.c for `^Z`).
pub mod commands {
    #[no_mangle]
    pub extern "C" fn do_cmd_borg() {
        super::with_entry_state(crate::borg_commands::do_cmd_borg)
    }
}

/// `get_com(prompt, &cmd)` from Borg code that holds the state (waits for
/// a key, so as `msg`).
pub fn get_com(st: &mut BorgState, prompt: &std::ffi::CStr) -> (bool, u8) {
    escape(st);
    let r = crate::engine::get_com(prompt);
    escape(st);
    r
}

/// `get_com_ex(prompt, &ke)` from Borg code that holds the state (waits
/// for a key, so as `msg`).
pub fn get_com_ex(st: &mut BorgState, prompt: &std::ffi::CStr) -> bool {
    escape(st);
    let r = crate::engine::get_com_ex(prompt);
    escape(st);
    r
}

/// `get_quantity(prompt, max)` from Borg code that holds the state (waits
/// for keys, so as `msg`).
pub fn get_quantity(st: &mut BorgState, prompt: &std::ffi::CStr, max: i32) -> i32 {
    escape(st);
    let r = crate::engine::get_quantity(prompt, max);
    escape(st);
    r
}

/// `target_set_interactive(mode, x, y, allow)` from Borg code that holds
/// the state (waits for keys, so as `msg`).
pub fn target_set_interactive(st: &mut BorgState, mode: i32, x: i32, y: i32, allow: bool) -> bool {
    escape(st);
    let r = crate::engine::target_set_interactive(mode, x, y, allow);
    escape(st);
    r
}

/// `screen_save()` from Borg code that holds the state (flushes messages,
/// so as `msg`).
pub fn screen_save(st: &mut BorgState) {
    escape(st);
    crate::engine::screen_save();
    escape(st);
}

/// `screen_load()` from Borg code that holds the state (flushes messages,
/// so as `msg`).
pub fn screen_load(st: &mut BorgState) {
    escape(st);
    crate::engine::screen_load();
    escape(st);
}

/// `menu_select(m, notify, popup)` from Borg code that holds the state
/// (waits for keys, so as `msg`). `display` must not use the state.
pub fn menu_select(
    st: &mut BorgState,
    m: &mut crate::engine::Menu,
    notify: i32,
    popup: bool,
    display: &mut dyn FnMut(crate::engine::MenuRow),
) -> crate::headers::ui_event_h::ui_event_type {
    escape(st);
    let r = m.select(notify, popup, display);
    escape(st);
    r
}

/// `project(source_player(), ...)` from Borg code that holds the state
/// (signals display events, so as `msg`).
#[allow(clippy::too_many_arguments)]
pub fn project_from_player(
    st: &mut BorgState,
    rad: i32,
    x: i32,
    y: i32,
    dam: i32,
    typ: i32,
    flg: i32,
    arc: i32,
    diameter: u8,
) -> bool {
    escape(st);
    let r = crate::engine::project_from_player(rad, x, y, dam, typ, flg, arc, diameter);
    escape(st);
    r
}

