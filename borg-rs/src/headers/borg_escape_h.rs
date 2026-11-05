//! `/home/user/borg-rs/angband/src/borg/borg-escape.h`
//!
//! borg-escape.c is converted (`crate::borg_escape`); the C names below are
//! C-ABI shims in `bridge::escape_fns`.
#[allow(unused_imports)]
use super::*;
pub use crate::bridge::escape_fns::borg_allow_teleport;
pub use crate::bridge::escape_fns::borg_caution_phase;
pub use crate::bridge::escape_fns::borg_caution_teleport;
pub use crate::bridge::escape_fns::borg_dimension_door;
pub use crate::bridge::escape_fns::borg_recall;
pub use crate::bridge::escape_fns::borg_shadow_shift;
pub use crate::bridge::escape_fns::borg_surrounded;
