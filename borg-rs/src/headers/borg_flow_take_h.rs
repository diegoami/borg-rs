//! `/home/user/borg-rs/angband/src/borg/borg-flow-take.h`
pub use super::__stddef_wchar_t_h::wchar_t;
pub use super::borg_h::borg_time;
pub use super::borg_h::C2RustUnnamedH_10;
pub use super::borg_h::C2RustUnnamedH_11;
pub use super::borg_h::C2RustUnnamedH_12;
pub use super::borg_h::C2RustUnnamedH_13;
pub use super::borg_h::C2RustUnnamedH_14;
pub use super::borg_h::C2RustUnnamedH_15;
pub use super::borg_h::C2RustUnnamedH_16;
pub use super::borg_h::C2RustUnnamedH_17;
pub use super::cave_h::chunk;
pub use super::object_h::object;
pub use super::player_h::player;
pub use super::stdint_intn_h::int16_t;
pub use super::stdint_intn_h::int32_t;
pub use super::stdint_uintn_h::uint32_t;
pub use super::stdint_uintn_h::uint8_t;
pub use super::z_type_h::loc;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_take {
    pub kidx: uint32_t,
    pub known: bool,
    pub seen: bool,
    pub extra: bool,
    pub orbed: bool,
    pub x: uint8_t,
    pub y: uint8_t,
    pub when: borg_time,
    pub value: ::core::ffi::c_int,
    pub tval: ::core::ffi::c_int,
}
pub use crate::c2rust::borg_flow_take::borg_delete_take;
pub use crate::c2rust::borg_flow_take::borg_flow_take;
pub use crate::c2rust::borg_flow_take::borg_flow_take_lunal;
pub use crate::c2rust::borg_flow_take::borg_flow_take_scum;
pub use crate::c2rust::borg_flow_take::borg_follow_take;
pub use crate::c2rust::borg_flow_take::borg_free_flow_take;
pub use crate::c2rust::borg_flow_take::borg_get_top_object;
pub use crate::c2rust::borg_flow_take::borg_init_flow_take;
pub use crate::c2rust::borg_flow_take::borg_takes;
pub use crate::c2rust::borg_flow_take::borg_takes_cnt;
pub use crate::c2rust::borg_flow_take::borg_takes_nxt;
pub use crate::c2rust::borg_flow_take::observe_take_diff;
pub use crate::c2rust::borg_flow_take::observe_take_move;
