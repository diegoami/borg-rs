pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::borg_cave_h::borg_grid;
use crate::headers::borg_init_h::borg_init_failure;
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};

pub use crate::headers::stdint_uintn_h::{uint16_t, uint8_t};
pub use crate::headers::types_h::{__uint16_t, __uint8_t};
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut borg_grids: [*mut borg_grid; 66] =
    [::core::ptr::null::<borg_grid>() as *mut borg_grid; 66];
#[no_mangle]
pub unsafe extern "C" fn borg_init_cave() {
    borg_free_cave();
    if DUNGEON_WID != (*z_info).dungeon_wid as ::core::ffi::c_int
        || DUNGEON_HGT != (*z_info).dungeon_hgt as ::core::ffi::c_int
    {
        borg_note(
            b"**STARTUP FAILURE** dungeon size miss match\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        borg_grids[y as usize] = mem_zalloc(
            (AUTO_MAX_X as size_t).wrapping_mul(::core::mem::size_of::<borg_grid>() as size_t),
        ) as *mut borg_grid;
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_cave() {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        if !borg_grids[y as usize].is_null() {
            mem_free(borg_grids[y as usize] as *mut ::core::ffi::c_void);
        }
        borg_grids[y as usize] = ::core::ptr::null_mut::<borg_grid>();
        y += 1;
    }
}
