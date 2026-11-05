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
// Converted modules contain no `unsafe`: only the FFI boundary (`engine`,
// `headers`) and the transitional code (`bridge`, `c2rust`) may.
#![deny(unsafe_code)]

extern crate libc;

#[allow(clippy::all)]
#[allow(unsafe_code)]
pub mod bridge;
#[allow(clippy::all)]
#[allow(unsafe_code)]
pub mod c2rust;
pub mod cctype;
#[allow(clippy::all)]
#[allow(unsafe_code)]
pub mod headers;

pub mod borg;
pub mod borg_attack_munchkin;
pub mod borg_cave;
pub mod borg_cave_light;
pub mod borg_cave_view;
pub mod borg_cave_util;
pub mod borg_danger;
pub mod borg_escape;
pub mod borg_fight_perm;
pub mod borg_flow;
pub mod borg_flow_glyph;
pub mod borg_flow_dark;
pub mod borg_flow_take;
pub mod borg_flow_kill;
pub mod borg_flow_stairs;
pub mod borg_formulas;
pub mod borg_formulas_calc;
pub mod borg_home_notice;
pub mod borg_home_power;
pub mod borg_inventory;
pub mod borg_io;
pub mod borg_item;
pub mod borg_item_activation;
pub mod borg_item_analyze;
pub mod borg_item_decurse;
pub mod borg_item_enchant;
pub mod borg_item_id;
pub mod borg_item_use;
pub mod borg_item_val;
pub mod borg_item_wear;
pub mod borg_junk;
pub mod borg_light;
pub mod borg_log;
pub mod borg_magic;
pub mod borg_magic_play;
pub mod borg_messages_react;
pub mod borg_prepared;
pub mod borg_projection;
pub mod borg_recover;
pub mod borg_reincarnate;
pub mod borg_store;
pub mod borg_store_sell;
pub mod borg_store_buy;
pub mod borg_think_store;
pub mod borg_util;
#[allow(unsafe_code)]
pub mod engine;
pub mod state;
