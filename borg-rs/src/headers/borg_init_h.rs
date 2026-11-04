//! `/home/user/borg-rs/angband/src/borg/borg-init.h`
#[allow(unused_imports)]
use super::*;
pub use crate::c2rust::borg_init::borg_init_failure;
pub use crate::c2rust::borg_init::borg_reset_settings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_setting {
    pub setting_string: *const ::core::ffi::c_char,
    pub setting_type: ::core::ffi::c_char,
    pub default_value: ::core::ffi::c_int,
}
pub use crate::c2rust::borg_init::borg_free;
pub use crate::c2rust::borg_init::borg_init;
pub use crate::c2rust::borg_init::borg_init_txt_file;
pub use crate::c2rust::borg_init::borg_initialized;
pub use crate::c2rust::borg_init::borg_prepare_race_class_info;
pub use crate::c2rust::borg_init::borg_reinit_options;
pub use crate::c2rust::borg_init::borg_settings;
pub use crate::c2rust::borg_init::game_closed;
