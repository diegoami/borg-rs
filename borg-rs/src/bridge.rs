//! Transitional: the raw `borg_item.note` pointer. Goes away when `note`
//! becomes a safe offset into `desc`.

use core::ffi::c_char;

use crate::state::BorgState;

unsafe fn cbytes<'a>(p: *const c_char) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

/// A zeroed `borg_item` (what `mem_zalloc` gave the C arrays).
pub fn blank_borg_item() -> crate::headers::borg_item_h::borg_item {
    // SAFETY: borg_item is plain C data (integers, bools, arrays and one
    // nullable pointer); the all-zero bit pattern is valid for each field.
    unsafe { core::mem::zeroed() }
}

/// `borg_items[j].note[0]`. `note` points into the item's `desc` (set by
/// borg_item_analyze). C dereferences it without a check; a NULL note
/// would crash C, so it panics here.
pub fn item_note_first_byte(st: &mut BorgState, j: usize) -> u8 {
    let p = st.items.items[j].note;
    assert!(!p.is_null(), "borg_items[{}].note is NULL", j);
    // SAFETY: a non-NULL note points at a NUL-terminated suffix of a
    // `desc` buffer of a live item (C semantics).
    unsafe { *p as u8 }
}

/// The bytes of `borg_get_note(item)` (the inscription part of `desc`, or
/// "" when `note` is NULL). `note` points into an item's `desc` (set by
/// borg_item_analyze); for a copied item it still points into the
/// original's `desc`, as in C.
pub fn item_note(item: &crate::headers::borg_item_h::borg_item) -> Vec<u8> {
    if item.note.is_null() {
        return Vec::new();
    }
    // SAFETY: a non-NULL note points at a NUL-terminated suffix of a live
    // item's `desc` buffer (C semantics; C reads it the same way).
    unsafe { cbytes(item.note).to_vec() }
}

/// `item->note = item->desc + offset` (borg-item-analyze.c). `note` stays
/// a raw pointer into the item's own `desc` (C semantics: copies of the
/// item keep pointing at this item's `desc`).
pub fn set_item_note(item: &mut crate::headers::borg_item_h::borg_item, offset: usize) {
    assert!(offset < item.desc.len(), "note offset outside desc");
    // SAFETY: offset is inside the item's desc array (checked), so the
    // pointer stays in bounds of that allocation; nothing is dereferenced
    // here. It is dereferenced later by readers, as in C.
    item.note = unsafe { item.desc.as_mut_ptr().add(offset) };
}

/// `borg_get_note(&borg_items[i])` (borg-item.c), as bytes.
pub fn borg_get_note(st: &mut BorgState, i: usize) -> Vec<u8> {
    item_note(&st.items.items[i])
}
