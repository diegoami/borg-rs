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
pub use crate::bridge::item_use::borg_quaff_crit;
pub use crate::bridge::item_use::borg_quaff_potion;
pub use crate::bridge::item_use::borg_quaff_unknown;
pub use crate::bridge::item_use::borg_read_scroll;
pub use crate::bridge::item_use::borg_read_unknown;
pub use crate::bridge::item_use::borg_eat;
pub use crate::bridge::item_use::borg_eat_unknown;
pub use crate::bridge::item_use::borg_eat_food_any;
pub use crate::bridge::item_use::borg_equips_rod;
pub use crate::bridge::item_use::borg_zap_rod;
pub use crate::bridge::item_use::borg_use_staff;
pub use crate::bridge::item_use::borg_use_unknown;
pub use crate::bridge::item_use::borg_use_staff_fail;
pub use crate::bridge::item_use::borg_equips_staff_fail;
pub use crate::bridge::item_use::borg_aim_wand;
pub use crate::bridge::item_use::borg_equips_ring;
pub use crate::bridge::item_use::borg_activate_ring;
pub use crate::bridge::item_use::borg_equips_dragon;
pub use crate::bridge::item_use::borg_activate_dragon;
pub use crate::bridge::item_use::borg_activate_item;
pub use crate::bridge::item_use::borg_equips_item;
pub use crate::bridge::item_use::borg_activate_failure;
pub use crate::bridge::item_use::borg_use_things;
pub use crate::bridge::item_use::borg_recharging;
