//! `/home/user/borg-rs/angband/src/borg/borg-trait-swap.h`
//!
//! borg-trait-swap.c is converted (`crate::borg_trait_swap`); the C names
//! below are C-ABI shims in `bridge::trait_swap`. (The swap globals live in
//! `BorgState::trait_swap`.)
#[allow(unused_imports)]
use super::*;
pub use crate::bridge::trait_swap::{borg_notice_armour_swap, borg_notice_weapon_swap};
