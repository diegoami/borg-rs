//! `/home/user/borg-rs/angband/src/borg/borg-formulas.h`
pub use super::stdint_intn_h::int32_t;
pub use super::z_file_h::ang_file;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_array {
    pub max: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type value_type = ::core::ffi::c_int;
pub const VT_MAX: value_type = 41;
pub const VT_CLASS: value_type = 4;
pub const VT_ACTIVATION: value_type = 3;
pub const VT_CONFIG: value_type = 2;
pub const VT_TRAIT: value_type = 1;
pub const VT_RANGE_INDEX: value_type = 0;
pub const VT_NONE: value_type = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct value_sec {
    pub type_0: value_type,
    pub index: int32_t,
}
pub use crate::c2rust::borg_formulas::borg_array_add;
pub use crate::c2rust::borg_formulas::borg_formula_error;
pub use crate::c2rust::borg_formulas::borg_free_formulas;
pub use crate::c2rust::borg_formulas::borg_load_formulas;
pub use crate::c2rust::borg_formulas::borg_power_dynamic;
pub use crate::c2rust::borg_formulas::borg_prepared_dynamic;
pub use crate::c2rust::borg_formulas::borg_restock_dynamic;
pub use crate::c2rust::borg_formulas::calculate_from_value;
pub use crate::c2rust::borg_formulas::parse_value;
