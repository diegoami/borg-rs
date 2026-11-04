//! `/home/user/borg-rs/angband/src/borg/borg-cave.h`
pub use super::stdint_uintn_h::uint16_t;
pub use super::stdint_uintn_h::uint8_t;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_grid {
    pub feat: uint8_t,
    pub info: uint16_t,
    pub trap: bool,
    pub glyph: bool,
    pub web: bool,
    pub store: uint8_t,
    pub take: uint8_t,
    pub kill: uint8_t,
    pub xtra: uint8_t,
}
pub use crate::c2rust::borg_cave::borg_free_cave;
pub use crate::c2rust::borg_cave::borg_grids;
pub use crate::c2rust::borg_cave::borg_init_cave;
