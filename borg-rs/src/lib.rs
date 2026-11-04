//! Rust port of the Angband Borg.
//!
//! Stage M2: `c2rust` is the unsafe transpilation of `angband/src/borg/*.c`
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
#![allow(clippy::all)]

extern crate libc;

pub mod c2rust;
