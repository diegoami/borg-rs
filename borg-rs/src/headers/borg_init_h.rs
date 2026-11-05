//! `/home/user/borg-rs/angband/src/borg/borg-init.h`
#[allow(unused_imports)]
use super::*;
// borg-init.c is converted (crate::borg_init); the globals live in
// `BorgState::init` and the settings table is `borg_init::BORG_SETTINGS`.
pub use crate::bridge::init::borg_free;
pub use crate::bridge::init::borg_init;
pub use crate::bridge::init::borg_init_txt_file;
pub use crate::bridge::init::borg_reinit_options;
