//! `/home/user/borg-rs/angband/src/borg/borg-update.h`
//!
//! borg-update.c is converted (`crate::borg_update`): its globals live in
//! `BorgState::update`, and `borg_update` is a C-ABI shim in
//! `crate::bridge::update` (borg_think).
#[allow(unused_imports)]
use super::*;
pub use crate::bridge::update::borg_update;
