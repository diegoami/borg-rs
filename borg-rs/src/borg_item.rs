//! Port of `borg-item.c`: the Borg's view of its items.
//!
//! Copyright (c) 1997 Ben Harrison, James E. Wilson, Robert A. Koeneke
//! Copyright (c) 2007-9 Andi Sidwell, Chris Carr, Ed Graham, Erik Osheim
//!
//! This work is free software; you can redistribute it and/or modify it
//! under the terms of either:
//!
//! a) the GNU General Public License as published by the Free Software
//!    Foundation, version 2, or
//!
//! b) the "Angband License":
//!    This software may be copied and distributed for educational, research,
//!    and not for profit purposes provided that this copyright and statement
//!    are included in all such copies.  Other copyrights may also apply.

use crate::borg_io::borg_keypress;
use crate::headers::borg_item_h::borg_item;
use crate::headers::list_tvals_h::TV_FOOD;
use crate::state::BorgState;
use crate::{bridge, engine};

/// The items (`borg_items`) and the snapshot taken after each full read
/// (`safe_items`, the "safety inventory").
///
/// Slots: `0..pack_size` the pack, `pack_size..pack_size + 12` the
/// equipment (`INVEN_WIELD` = `pack_size`), then the quiver.
///
/// TRANSITIONAL: `borg_item` is still the C struct. Its `note` field points
/// into the item's own `desc` (set by borg_item_analyze, still c2rust);
/// safe code reads it only through `bridge::item_note_first_byte`.
#[derive(Default)]
pub struct ItemState {
    pub items: Vec<borg_item>,
    pub safe_items: Vec<borg_item>,
}

/// `INVEN_WIELD`: the first equipment slot.
pub fn inven_wield() -> i32 {
    i32::from(engine::z_info().pack_size)
}

/// `INVEN_FEET`: the last equipment slot that has a letter.
pub fn inven_feet() -> i32 {
    inven_wield() + 11
}

/// `QUIVER_START`
pub fn quiver_start() -> i32 {
    inven_wield() + 12
}

/// `QUIVER_END`: the number of item slots.
pub fn quiver_end() -> i32 {
    let z = engine::z_info();
    i32::from(z.pack_size) + 12 + i32::from(z.quiver_size)
}

/// A zeroed item (as `mem_zalloc` makes them).
pub fn blank_item() -> borg_item {
    bridge::blank_borg_item()
}

/// Does the note of slot `j` start with `{` (an inscription)?
fn note_starts_with_brace(st: &mut BorgState, j: i32) -> bool {
    bridge::item_note_first_byte(st, j as usize) == b'{'
}

/// Remove the inscription of slot `i` (`borg_deinscribe`).
pub fn borg_deinscribe(st: &mut BorgState, i: i32) {
    let it = &st.items.items[i as usize];
    if u32::from(it.tval) == TV_FOOD
        && i32::from(it.sval) == st.item_val.sv_food_slime_mold
    {
        return;
    }

    borg_keypress(st, u32::from(b'}'));

    let letter = |n: i32| engine::all_letters_nohjkl(n) as i8 as i32 as u32;
    if i < inven_wield() {
        borg_keypress(st, letter(i));
    } else if i < inven_feet() {
        for j in 0..inven_wield() {
            if st.items.items[j as usize].iqty != 0 && note_starts_with_brace(st, j) {
                borg_keypress(st, u32::from(b'/'));
                break;
            }
        }
        borg_keypress(st, letter(i - inven_wield()));
    } else {
        for j in 0..=inven_feet() {
            if st.items.items[j as usize].iqty != 0 && note_starts_with_brace(st, j) {
                borg_keypress(st, u32::from(b'|'));
                break;
            }
        }
        borg_keypress(st, (i32::from(b'0') + (i - quiver_start())) as u32);
    }
    borg_keypress(st, u32::from(b'y'));
    borg_keypress(st, u32::from(b'y'));
}

/// Weight of the whole stack, as C's `int16_t` (`borg_item_weight`).
pub fn borg_item_weight(item: &borg_item) -> i16 {
    (i32::from(item.iqty) * i32::from(item.weight)) as i16
}

/// Read the equipment, the pack and the store, then snapshot the items
/// (`borg_find_all_items`).
pub fn borg_find_all_items(st: &mut BorgState) {
    crate::borg_inventory::borg_equipment(st);
    crate::borg_inventory::borg_inventory(st);
    crate::borg_store::borg_notice_store(st);
    let n = quiver_end() as usize;
    let snapshot = st.items.items[..n].to_vec();
    st.items.safe_items[..n].copy_from_slice(&snapshot);
}

/// Allocate the item arrays (`borg_init_item`).
pub fn borg_init_item(st: &mut BorgState) {
    let n = quiver_end() as usize;
    st.items.items = vec![blank_item(); n];
    st.items.safe_items = vec![blank_item(); n];
}

/// Free the item arrays (`borg_free_item`).
pub fn borg_free_item(st: &mut BorgState) {
    st.items.safe_items = Vec::new();
    st.items.items = Vec::new();
}
