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

/// `borg_note(what)` (borg-io.c, converted).
pub fn borg_note(st: &mut BorgState, what: &str) {
    crate::borg_io::borg_note(st, what)
}

unsafe fn cbytes<'a>(p: *const c_char) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

// ---------------------------------------------------------------------------
// Calls from converted code into still-c2rust functions. Each takes the
// caller's `&mut BorgState` so the borrow visibly escapes into the call (the
// c2rust code mutates the same state through `st()`).

/// A copy of `borg_items[i]` (borg-item.c, converted).
pub fn borg_item(st: &mut BorgState, i: i32) -> crate::headers::borg_item_h::borg_item {
    st.items.items[i as usize]
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

/// `borg_fear_region[y][x]` (borg-danger.c, converted).
pub fn borg_fear_region(st: &mut BorgState, y: i32, x: i32) -> u16 {
    st.danger.fear_region[y as usize][x as usize]
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

/// `borg_equips_ring(ring_sval)` (borg-item-use.c, converted).
pub fn borg_equips_ring(st: &mut BorgState, ring_sval: i32) -> bool {
    crate::borg_item_use::borg_equips_ring(st, ring_sval)
}

/// `borg_items[i]` (still c2rust, borg-item.c), copied out. Transitional:
/// goes away when borg_item converts. The copy's `note` field is a raw C
/// string pointer that safe code must not dereference.
pub fn item(st: &mut BorgState, i: usize) -> crate::headers::borg_item_h::borg_item {
    st.items.items[i]
}

/// `(borg_items[i].tval, borg_items[i].sval)` (still c2rust, borg-item.c).
pub fn borg_item_tval_sval(st: &mut BorgState, i: i32) -> (u8, u8) {
    assert!(i >= 0 && i < i32::from(crate::engine::z_info().pack_size));
    let it = &st.items.items[i as usize];
    (it.tval, it.sval)
}

/// A copy of `borg_shops[shop].ware[i]` (borg-store.c, converted). The
/// copy's `note` field is a raw C string pointer that safe code must not
/// dereference.
pub fn shop_item(st: &mut BorgState, shop: usize, i: usize) -> crate::headers::borg_item_h::borg_item {
    st.store.shops[shop].ware[i]
}

/// `borg_equips_item(activation, check_charge)`: forwards to the safe
/// `borg_item_use::borg_equips_item` (kept for callers not yet switched).
pub fn borg_equips_item(st: &mut BorgState, activation: i32, check_charge: bool) -> bool {
    crate::borg_item_use::borg_equips_item(st, activation, check_charge)
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
                write_buffer(&s);
                (&raw const PREPARED_BUFFER).cast()
            }
            Some(Reason::Depth(i)) => st().formulas.depth[i].reason.as_ptr(),
            Some(Reason::Restock(i)) => st().formulas.restock[i].reason.as_ptr(),
        }
    }

    /// `strnfmt(borg_prepared_buffer, MAX_REASON, ...)` of a formatted
    /// reason.
    fn write_buffer(s: &[u8]) {
        // SAFETY: engine thread only; no reference to the buffer is live.
        let buf = unsafe { &mut *(&raw mut PREPARED_BUFFER) };
        let n = s.len().min(MAX_REASON - 1);
        for (d, &b) in buf.iter_mut().zip(&s[..n]) {
            *d = b as c_char;
        }
        buf[n] = 0;
    }

    /// The text in `borg_prepared_buffer` now (up to its NUL).
    pub(crate) fn buffer_text() -> Vec<u8> {
        // SAFETY: engine thread only; a plain read of the static.
        let buf = unsafe { &*(&raw const PREPARED_BUFFER) };
        buf.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect()
    }

    /// `borg_prepared(depth)` for converted code, keeping C's pointer:
    /// a formatted reason is written into `borg_prepared_buffer` (as C
    /// does) and returned as `PreparedC::Buffer`.
    pub(crate) fn borg_prepared_buf(
        st: &mut crate::state::BorgState,
        depth: c_int,
    ) -> Option<super::PreparedC> {
        let r = p::borg_prepared(st, depth)?;
        Some(match r {
            Reason::Formatted(s) => {
                write_buffer(&s);
                super::PreparedC::Buffer
            }
            r => super::PreparedC::Text(r.text(st).to_vec()),
        })
    }

    pub unsafe extern "C" fn borg_prepared(depth: c_int) -> *const c_char {
        c_reason(p::borg_prepared(st(), depth))
    }
    pub unsafe extern "C" fn borg_restock(depth: c_int) -> *const c_char {
        c_reason(p::borg_restock(st(), depth))
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

/// `borg_get_note(&borg_items[i])` (borg-item.c), as bytes. The note
/// pointer is still raw (see borg_item.rs); copied out at once.
pub fn borg_get_note(st: &mut BorgState, i: usize) -> Vec<u8> {
    item_note(&st.items.items[i])
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

/// `borg_first_empty_inventory_slot()` (still c2rust, borg-inventory.c),
/// for borg-store.c.
pub fn store_first_empty_inventory_slot(st: &mut BorgState) -> i32 {
    crate::borg_inventory::borg_first_empty_inventory_slot(st)
}

/// `borg_init_store_sell()` (borg-store-sell.c, converted).
pub fn borg_init_store_sell(st: &mut BorgState) {
    crate::borg_store_sell::borg_init_store_sell(st)
}

/// `borg_free_store_sell()` (borg-store-sell.c, converted).
pub fn borg_free_store_sell(st: &mut BorgState) {
    crate::borg_store_sell::borg_free_store_sell(st)
}

/// One entry of the "just bought" list of borg-store-buy.c (still
/// c2rust): `bought_item_tval/sval/pval/store[i]`.
#[derive(Clone, Copy, Debug)]
pub struct BoughtItem {
    pub tval: i32,
    pub sval: i32,
    pub pval: i32,
    pub store: i32,
}

/// `bought_item_num` (borg-store-buy.c, converted).
pub fn bought_item_num(st: &mut BorgState) -> i32 {
    st.buy.bought_item_num
}

/// `bought_item_*[i]` (still c2rust, borg-store-buy.c); panics past the
/// 10 entries.
pub fn bought_item(st: &mut BorgState, i: usize) -> BoughtItem {
    let b = &st.buy;
    BoughtItem {
        tval: b.bought_item_tval[i],
        sval: b.bought_item_sval[i],
        pval: b.bought_item_pval[i],
        store: b.bought_item_store[i],
    }
}

/// `borg_item_analyze(&borg_shops[shop].ware[slot], obj, desc, in_store)`
/// for borg_notice_store (c2rust). Calls the converted analyzer directly,
/// with the ware and the two tables it reads borrowed as separate fields.
pub fn borg_item_analyze_ware(
    st: &mut BorgState,
    shop: usize,
    slot: usize,
    obj: crate::engine::StoreObject,
    desc: &[u8],
    in_store: bool,
) {
    let BorgState { store, act, item_val, .. } = st;
    crate::borg_item_analyze::borg_item_analyze(
        &mut store.shops[shop].ware[slot],
        obj.into(),
        desc,
        in_store,
        act,
        item_val,
    );
}

// ---------------------------------------------------------------------------
// borg-item-wear.c (converted): calls into still-c2rust code.

/// `borg_read_scroll(sval)` (borg-item-use.c, converted).
pub fn borg_read_scroll(st: &mut BorgState, sval: i32) -> bool {
    crate::borg_item_use::borg_read_scroll(st, sval)
}

// ---------------------------------------------------------------------------
// borg-item-decurse.c (converted): calls into still-c2rust code.

// ---------------------------------------------------------------------------
// borg-flow-dark.c (converted): calls into still-c2rust flow code.

/// One of the flow arrays of borg-flow.c (converted, `st.flow`).
#[derive(Clone, Copy)]
pub enum FlowData {
    /// `borg_data_cost`
    Cost,
    /// `borg_data_know`
    Know,
    /// `borg_data_icky`
    Icky,
}

/// `borg_data_<which>->data[y][x]` (borg-flow.c, converted).
pub fn borg_data_at(st: &mut BorgState, which: FlowData, y: i32, x: i32) -> u8 {
    let f = &st.flow;
    let d = match which {
        FlowData::Cost => &f.data_cost,
        FlowData::Know => &f.data_know,
        FlowData::Icky => &f.data_icky,
    };
    assert!(!d.is_empty(), "flow array not allocated");
    d[y as usize][x as usize]
}

/// `borg_data_<which>->data[y][x] = v` (borg-flow.c, converted).
pub fn set_borg_data(st: &mut BorgState, which: FlowData, y: i32, x: i32, v: u8) {
    let f = &mut st.flow;
    let d = match which {
        FlowData::Cost => &mut f.data_cost,
        FlowData::Know => &mut f.data_know,
        FlowData::Icky => &mut f.data_icky,
    };
    assert!(!d.is_empty(), "flow array not allocated");
    d[y as usize][x as usize] = v;
}

/// `track_door.num` (borg-flow.c, converted).
pub fn track_door_num(st: &mut BorgState) -> i16 {
    st.flow.track_door.num
}

/// `(track_door.y[i], track_door.x[i])` (borg-flow.c, converted), for
/// `i < track_door.size`.
pub fn track_door_at(st: &mut BorgState, i: usize) -> (i32, i32) {
    let t = &st.flow.track_door;
    assert!(i < t.size.max(0) as usize, "track_door index past the list");
    (t.y[i], t.x[i])
}

// ---------------------------------------------------------------------------
// borg-store-buy.c (converted): calls into still-c2rust code.

/// `sold_item_num` (still c2rust, borg-store-sell.c): the index of the
/// last item sold (-1 at start).
pub fn sold_item_num(st: &mut BorgState) -> i32 {
    st.store_sell.sold_item_num
}

/// `(sold_item_tval[p], sold_item_sval[p], sold_item_store[p])` (still
/// c2rust, borg-store-sell.c); panics past the 10 entries.
pub fn sold_item(st: &mut BorgState, p: usize) -> (i32, i32, i32) {
    let s = &st.store_sell;
    (s.sold_item_tval[p], s.sold_item_sval[p], s.sold_item_store[p])
}

/// `borg_min_item_quantity(item)` (still c2rust, borg-store-sell.c), on a
/// copy of the item (it only reads it).
pub fn borg_min_item_quantity(
    st: &mut BorgState,
    item: &crate::headers::borg_item_h::borg_item,
) -> i32 {
    crate::borg_store_sell::borg_min_item_quantity(st, item)
}

/// `borg_think_home_sell_useful(&best_home_power)` (still c2rust,
/// borg-store-sell.c).
pub fn borg_think_home_sell_useful(st: &mut BorgState, best_home_power: &mut i32) -> bool {
    crate::borg_store_sell::borg_think_home_sell_useful(st, best_home_power)
}

// ---------------------------------------------------------------------------
// borg-caution.c (converted): calls into still-c2rust code.

/// What C's `borg_prepared()` returned (when not NULL): a text that stays
/// as it is, or the static `borg_prepared_buffer`, whose text the next
/// formatted reason overwrites (read it with `prepared_buffer`).
pub enum PreparedC {
    /// A fixed text (literal or formula reason)
    Text(Vec<u8>),
    /// `borg_prepared_buffer`
    Buffer,
}

/// `borg_prepared(depth)` (borg-prepared.c, converted), keeping C's
/// pointer semantics (see `PreparedC`). Writes `borg_prepared_buffer` as C
/// does.
pub fn borg_prepared_buf(st: &mut BorgState, depth: i32) -> Option<PreparedC> {
    prepared::borg_prepared_buf(st, depth)
}

/// The text `borg_prepared_buffer` holds now.
pub fn prepared_buffer(_st: &mut BorgState) -> Vec<u8> {
    prepared::buffer_text()
}

/// `borg_restock(depth)` through the C-ABI shim, so the static
/// `borg_prepared_buffer` is written as in C (as `borg_prepared_c`).
/// `None` for NULL.
pub fn borg_restock_c(st: &mut BorgState, depth: i32) -> Option<Vec<u8>> {
    escape(st);
    // SAFETY: the shim returns NULL or a NUL-terminated string that stays
    // valid until the next call; copied out at once.
    let r = unsafe {
        let p = prepared::borg_restock(depth);
        (!p.is_null()).then(|| cbytes(p).to_vec())
    };
    escape(st);
    r
}

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

