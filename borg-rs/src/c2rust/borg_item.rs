pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
use crate::headers::borg_inventory_h::{borg_equipment, borg_inventory};
use crate::headers::borg_io_h::borg_keypress;
pub use crate::headers::borg_item_h::borg_item;
use crate::headers::borg_item_val_h::sv_food_slime_mold;
use crate::headers::borg_store_h::borg_notice_store;
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_tvals_h::{
    TV_AMULET, TV_ARROW, TV_BOLT, TV_BOOTS, TV_BOW, TV_CHEST, TV_CLOAK, TV_CROWN, TV_DIGGING,
    TV_DRAG_ARMOR, TV_FLASK, TV_FOOD, TV_GLOVES, TV_GOLD, TV_HAFTED, TV_HARD_ARMOR, TV_HELM,
    TV_LIGHT, TV_MAGIC_BOOK, TV_MUSHROOM, TV_NATURE_BOOK, TV_NULL, TV_OTHER_BOOK, TV_POLEARM,
    TV_POTION, TV_PRAYER_BOOK, TV_RING, TV_ROD, TV_SCROLL, TV_SHADOW_BOOK, TV_SHIELD, TV_SHOT,
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, TV_MAX};
pub use crate::headers::object_h::element_info;
pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memcpy;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_items: *mut borg_item = ::core::ptr::null::<borg_item>() as *mut borg_item;
#[no_mangle]
pub static mut safe_items: *mut borg_item = ::core::ptr::null::<borg_item>() as *mut borg_item;
#[no_mangle]
pub unsafe extern "C" fn borg_get_note(mut item: *const borg_item) -> *const ::core::ffi::c_char {
    if !(*item).note.is_null() {
        return (*item).note;
    }
    return b"\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn borg_deinscribe(mut i: ::core::ffi::c_int) {
    if (*borg_items.offset(i as isize)).tval as ::core::ffi::c_int == TV_FOOD as ::core::ffi::c_int
        && (*borg_items.offset(i as isize)).sval as ::core::ffi::c_int == sv_food_slime_mold
    {
        return;
    }
    borg_keypress('}' as i32 as keycode_t);
    if i < (*z_info).pack_size as ::core::ffi::c_int {
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
                as keycode_t,
        );
    } else if i < (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*z_info).pack_size as ::core::ffi::c_int {
            if (*borg_items.offset(j as isize)).iqty as ::core::ffi::c_int != 0
                && *(*borg_items.offset(j as isize))
                    .note
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '{' as i32
            {
                borg_keypress('/' as i32 as keycode_t);
                break;
            } else {
                j += 1;
            }
        }
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                as keycode_t,
        );
    } else {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
            if (*borg_items.offset(j_0 as isize)).iqty as ::core::ffi::c_int != 0
                && *(*borg_items.offset(j_0 as isize))
                    .note
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '{' as i32
            {
                borg_keypress('|' as i32 as keycode_t);
                break;
            } else {
                j_0 += 1;
            }
        }
        borg_keypress(
            ('0' as i32
                + (i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)))
                as keycode_t,
        );
    }
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_weight(mut item: *mut borg_item) -> int16_t {
    return ((*item).iqty as ::core::ffi::c_int * (*item).weight as ::core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_find_all_items() {
    borg_equipment();
    borg_inventory();
    borg_notice_store();
    memcpy(
        safe_items as *mut ::core::ffi::c_void,
        borg_items as *const ::core::ffi::c_void,
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_item() {
    borg_items = mem_zalloc(
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    ) as *mut borg_item;
    safe_items = mem_zalloc(
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    ) as *mut borg_item;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_item() {
    if !safe_items.is_null() {
        mem_free(safe_items as *mut ::core::ffi::c_void);
        safe_items = ::core::ptr::null_mut::<borg_item>();
    }
    if !borg_items.is_null() {
        mem_free(borg_items as *mut ::core::ffi::c_void);
        borg_items = ::core::ptr::null_mut::<borg_item>();
    }
}
