//! Rust port of the Angband Borg.
//!
//! `c2rust` is the unsafe transpilation (one module per C file) and
//! `headers` the deduplicated C header declarations of `angband/src/borg/*.c`
//! (upstream copyright: Ben Harrison, James E. Wilson, Robert A. Koeneke,
//! Andi Sidwell, Chris Carr, Ed Graham, Erik Osheim and other Angband/Borg
//! contributors; GPLv2 / Angband licence).
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(static_mut_refs)]

extern crate libc;

#[allow(clippy::all)]
pub mod bridge;
#[allow(clippy::all)]
pub mod c2rust;
pub mod cctype;
#[allow(clippy::all)]
pub mod headers;

pub mod borg;
pub mod borg_cave;
pub mod borg_cave_view;
pub mod borg_cave_util;
pub mod borg_formulas_calc;
pub mod borg_io;
pub mod borg_item_activation;
pub mod borg_item_val;
pub mod borg_util;
pub mod engine;
pub mod state;
