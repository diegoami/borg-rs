//! `/home/user/borg-rs/angband/src/borg/borg-item.h`
pub use super::object_h::element_info;
pub use super::stdint_intn_h::int16_t;
pub use super::stdint_intn_h::int32_t;
pub use super::stdint_uintn_h::uint32_t;
pub use super::stdint_uintn_h::uint8_t;
pub use super::z_bitflag_h::bitflag;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_item {
    pub desc: [::core::ffi::c_char; 80],
    pub note: *mut ::core::ffi::c_char,
    pub kind: uint32_t,
    pub ident: bool,
    pub needs_ident: bool,
    pub aware: bool,
    pub xxxx: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
    pub iqty: uint8_t,
    pub weight: int16_t,
    pub art_idx: uint8_t,
    pub ego_idx: uint8_t,
    pub activ_idx: ::core::ffi::c_int,
    pub one_ring: bool,
    pub timeout: int16_t,
    pub to_h: int16_t,
    pub to_d: int16_t,
    pub to_a: int16_t,
    pub ac: int16_t,
    pub dd: uint8_t,
    pub ds: uint8_t,
    pub level: uint8_t,
    pub cost: int32_t,
    pub value: int32_t,
    pub cursed: bool,
    pub uncursable: bool,
    pub curses: [bool; 28],
    pub flags: [bitflag; 5],
    pub modifiers: [int16_t; 16],
    pub el_info: [element_info; 25],
    pub brands: [bool; 254],
    pub slays: [::core::ffi::c_int; 85],
}
pub use crate::c2rust::borg_item::borg_deinscribe;
pub use crate::c2rust::borg_item::borg_find_all_items;
pub use crate::c2rust::borg_item::borg_free_item;
pub use crate::c2rust::borg_item::borg_init_item;
pub use crate::c2rust::borg_item::borg_item_weight;
pub use crate::c2rust::borg_item::borg_items;
pub type C2RustUnnamedH_44 = ::core::ffi::c_uint;
pub const BORG_CURSE_MAX: C2RustUnnamedH_44 = 28;
pub const BORG_CURSE_AIR_SWING: C2RustUnnamedH_44 = 27;
pub const BORG_CURSE_STEELSKIN: C2RustUnnamedH_44 = 26;
pub const BORG_CURSE_CHILLED_TO_THE_BONE: C2RustUnnamedH_44 = 25;
pub const BORG_CURSE_BURNING_UP: C2RustUnnamedH_44 = 24;
pub const BORG_CURSE_TREACHEROUS_WEAPON: C2RustUnnamedH_44 = 23;
pub const BORG_CURSE_ANTI_TELEPORTATION: C2RustUnnamedH_44 = 22;
pub const BORG_CURSE_STONE: C2RustUnnamedH_44 = 21;
pub const BORG_CURSE_COWARDICE: C2RustUnnamedH_44 = 20;
pub const BORG_CURSE_IMPAIR_HITPOINT_RECOVERY: C2RustUnnamedH_44 = 19;
pub const BORG_CURSE_IMPAIR_MANA_RECOVERY: C2RustUnnamedH_44 = 18;
pub const BORG_CURSE_UNDEAD_SUMMON: C2RustUnnamedH_44 = 17;
pub const BORG_CURSE_DEMON_SUMMON: C2RustUnnamedH_44 = 16;
pub const BORG_CURSE_DRAGON_SUMMON: C2RustUnnamedH_44 = 15;
pub const BORG_CURSE_PARALYSIS: C2RustUnnamedH_44 = 14;
pub const BORG_CURSE_HALLUCINATION: C2RustUnnamedH_44 = 13;
pub const BORG_CURSE_SIREN: C2RustUnnamedH_44 = 12;
pub const BORG_CURSE_POISON: C2RustUnnamedH_44 = 11;
pub const BORG_CURSE_ANNOYANCE: C2RustUnnamedH_44 = 10;
pub const BORG_CURSE_SLOWNESS: C2RustUnnamedH_44 = 9;
pub const BORG_CURSE_CLUMSINESS: C2RustUnnamedH_44 = 8;
pub const BORG_CURSE_WEAKNESS: C2RustUnnamedH_44 = 7;
pub const BORG_CURSE_IRRITATION: C2RustUnnamedH_44 = 6;
pub const BORG_CURSE_ENVELOPING: C2RustUnnamedH_44 = 5;
pub const BORG_CURSE_SICKLINESS: C2RustUnnamedH_44 = 4;
pub const BORG_CURSE_DULLNESS: C2RustUnnamedH_44 = 3;
pub const BORG_CURSE_TELEPORTATION: C2RustUnnamedH_44 = 2;
pub const BORG_CURSE_VULNERABILITY: C2RustUnnamedH_44 = 1;
pub const BORG_CURSE_UNKNOWN: C2RustUnnamedH_44 = 0;
pub use crate::c2rust::borg_item::borg_get_note;
pub use crate::c2rust::borg_item::safe_items;
