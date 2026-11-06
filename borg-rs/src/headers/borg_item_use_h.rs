//! `/home/user/borg-rs/angband/src/borg/borg-item-use.h`
//!
//! borg-item-use.c is converted (`crate::borg_item_use`); the static
//! `when_last_quaff` lives in `BorgState::item_use`.
#[allow(unused_imports)]
use super::*;
pub const BORG_UNMET_NEED: borg_need = 2;
pub type borg_need = ::core::ffi::c_uint;
pub const BORG_MET_NEED: borg_need = 1;
pub const BORG_NO_NEED: borg_need = 0;
