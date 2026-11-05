//! `/home/user/borg-rs/angband/src/borg/borg-flow.h`
//! (the flow data, queue, `borg_temp_*` and `track_step/door/closed` live
//! in `BorgState::flow`.)
pub use super::stdint_intn_h::int16_t;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub use crate::bridge::flow::borg_can_dig;
pub use crate::bridge::flow::borg_flow_clear;
pub use crate::bridge::flow::borg_flow_commit;
pub use crate::bridge::flow::borg_flow_enqueue_grid;
pub use crate::bridge::flow::borg_flow_old;
pub use crate::bridge::flow::borg_flow_spread;
pub use crate::bridge::flow::borg_free_flow;
pub use crate::bridge::flow::borg_free_track;
pub use crate::bridge::flow::borg_init_flow;
pub use crate::bridge::flow::borg_init_track;
