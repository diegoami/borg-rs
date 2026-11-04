//! `/home/user/borg-rs/angband/src/obj-ignore.h`
pub use super::stdint_uintn_h::uint8_t;
#[allow(unused_imports)]
use super::*;
pub const ITYPE_MAX: C2RustUnnamedH_40 = 27;
pub type C2RustUnnamedH_40 = ::core::ffi::c_uint;
extern "C" {
    pub static mut ego_ignore_types: *mut *mut bool;
}
extern "C" {
    pub fn ignore_birth_init();
}
extern "C" {
    pub static mut ignore_level: [uint8_t; 0];
}
