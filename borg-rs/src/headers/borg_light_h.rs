//! `/home/user/borg-rs/angband/src/borg/borg-light.h`
//!
//! borg-light.c is converted (`crate::borg_light`); the C names below are
//! C-ABI shims in `bridge::light`.
pub use super::borg_item_use_h::borg_need;
#[allow(unused_imports)]
use super::*;
pub use crate::bridge::light::borg_check_light;
pub use crate::bridge::light::borg_check_light_only;
pub use crate::bridge::light::borg_light_beam;
pub use crate::bridge::light::borg_maintain_light;
