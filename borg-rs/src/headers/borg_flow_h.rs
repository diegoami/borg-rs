//! `/home/user/borg-rs/angband/src/borg/borg-flow.h`
pub use super::stdint_intn_h::int16_t;
pub use super::stdint_uintn_h::uint8_t;
#[allow(unused_imports)]
use super::*;
pub use crate::c2rust::borg_flow::borg_temp_n;
pub use crate::c2rust::borg_flow::borg_temp_x;
pub use crate::c2rust::borg_flow::borg_temp_y;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub use crate::c2rust::borg_flow::track_step;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_data {
    pub data: [[uint8_t; 198]; 66],
}
pub use crate::c2rust::borg_flow::borg_can_dig;
pub use crate::c2rust::borg_flow::borg_data_cost;
pub use crate::c2rust::borg_flow::borg_data_flow;
pub use crate::c2rust::borg_flow::borg_data_hard;
pub use crate::c2rust::borg_flow::borg_data_icky;
pub use crate::c2rust::borg_flow::borg_data_know;
pub use crate::c2rust::borg_flow::borg_ddx_ddd;
pub use crate::c2rust::borg_flow::borg_ddy_ddd;
pub use crate::c2rust::borg_flow::borg_flow_clear;
pub use crate::c2rust::borg_flow::borg_flow_commit;
pub use crate::c2rust::borg_flow::borg_flow_enqueue_grid;
pub use crate::c2rust::borg_flow::borg_flow_old;
pub use crate::c2rust::borg_flow::borg_flow_spread;
pub use crate::c2rust::borg_flow::borg_flow_x;
pub use crate::c2rust::borg_flow::borg_flow_y;
pub use crate::c2rust::borg_flow::borg_free_flow;
pub use crate::c2rust::borg_flow::borg_free_track;
pub use crate::c2rust::borg_flow::borg_init_flow;
pub use crate::c2rust::borg_flow::borg_init_track;
pub use crate::c2rust::borg_flow::track_closed;
pub use crate::c2rust::borg_flow::track_door;
