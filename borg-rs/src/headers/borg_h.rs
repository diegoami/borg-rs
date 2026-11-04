//! `/home/user/borg-rs/angband/src/borg/borg.h`
pub use super::player_h::player;
pub use super::stdint_h::INT32_MAX;
pub use super::stdint_intn_h::int16_t;
pub use super::stdint_intn_h::int32_t;
pub use super::stdint_uintn_h::uint16_t;
pub use super::stdint_uintn_h::uint32_t;
pub use super::stdint_uintn_h::uint8_t;
pub use super::ui_event_h::keypress;
pub use super::z_type_h::loc;
#[allow(unused_imports)]
use super::*;
pub type borg_time = int32_t;
pub type C2RustUnnamedH_9 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamedH_9 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamedH_9 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamedH_9 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamedH_9 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamedH_9 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamedH_9 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamedH_9 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamedH_9 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamedH_9 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamedH_9 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamedH_9 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamedH_9 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamedH_9 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamedH_9 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamedH_9 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamedH_9 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamedH_9 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamedH_9 = 18;
pub const BORG_STOP_KING: C2RustUnnamedH_9 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamedH_9 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamedH_9 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamedH_9 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamedH_9 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamedH_9 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamedH_9 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamedH_9 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamedH_9 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamedH_9 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamedH_9 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamedH_9 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamedH_9 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamedH_9 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamedH_9 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamedH_9 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamedH_9 = 1;
pub const BORG_VERBOSE: C2RustUnnamedH_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_best {
    pub home: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_struct {
    pub player: *mut player,
    pub trait_0: *mut ::core::ffi::c_int,
    pub has: *mut ::core::ffi::c_int,
    pub activation: *mut ::core::ffi::c_int,
    pub power: int32_t,
    pub c: loc,
    pub panels: loc,
    pub avoidance: int16_t,
    pub oldchp: int16_t,
    pub lunal_mode: bool,
    pub munchkin_mode: bool,
    pub stair_less: bool,
    pub stair_more: bool,
    pub in_shop: bool,
    pub ready_morgoth: ::core::ffi::c_int,
    pub temp: C2RustUnnamedH_17,
    pub status: C2RustUnnamedH_16,
    pub time: C2RustUnnamedH_15,
    pub need_see_invis: borg_time,
    pub see_inv: int16_t,
    pub need_shift_panel: bool,
    pub when_shift_panel: borg_time,
    pub antibounce_count: int16_t,
    pub no_retreat: int16_t,
    pub resistance: int16_t,
    pub no_rest_prep: int16_t,
    pub times_twitch: int16_t,
    pub escapes: int16_t,
    pub trying_unknown: bool,
    pub dont_react: bool,
    pub targeting: bool,
    pub goal: C2RustUnnamedH_14,
    pub init_save: C2RustUnnamedH_13,
    pub mon: C2RustUnnamedH_12,
    pub id: C2RustUnnamedH_11,
    pub near: C2RustUnnamedH_10,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_10 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_11 {
    pub morgoth: ::core::ffi::c_uint,
    pub sauron: ::core::ffi::c_uint,
    pub tarrasque: ::core::ffi::c_uint,
    pub maggot: ::core::ffi::c_uint,
    pub bullroarer: ::core::ffi::c_uint,
    pub beorn: ::core::ffi::c_uint,
    pub grip: ::core::ffi::c_uint,
    pub fang: ::core::ffi::c_uint,
    pub osse: ::core::ffi::c_uint,
    pub squint_eyed_rogue: ::core::ffi::c_uint,
    pub small_kobold: ::core::ffi::c_uint,
    pub soldier: ::core::ffi::c_uint,
    pub cutpurse: ::core::ffi::c_uint,
    pub acolyte: ::core::ffi::c_uint,
    pub apprentice: ::core::ffi::c_uint,
    pub kobold: ::core::ffi::c_uint,
    pub shrieker_mushroom_patch: ::core::ffi::c_uint,
    pub filthy_street_urchin: ::core::ffi::c_uint,
    pub battle_scarred_veteran: ::core::ffi::c_uint,
    pub mean_looking_mercenary: ::core::ffi::c_uint,
    pub giant_white_mouse: ::core::ffi::c_uint,
    pub white_worm_mass: ::core::ffi::c_uint,
    pub green_worm_mass: ::core::ffi::c_uint,
    pub cave_spider: ::core::ffi::c_uint,
    pub red_naga: ::core::ffi::c_uint,
    pub giant_red_frog: ::core::ffi::c_uint,
    pub radiation_eye: ::core::ffi::c_uint,
    pub yellow_worm_mass: ::core::ffi::c_uint,
    pub gravity_hound: ::core::ffi::c_uint,
    pub inertia_hound: ::core::ffi::c_uint,
    pub dread: ::core::ffi::c_uint,
    pub dracolich: ::core::ffi::c_uint,
    pub dracolisk: ::core::ffi::c_uint,
    pub light_hound: ::core::ffi::c_uint,
    pub dark_hound: ::core::ffi::c_uint,
    pub pukelman: ::core::ffi::c_uint,
    pub night_mare: ::core::ffi::c_uint,
    pub drolem: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_12 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_13 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_14 {
    pub type_0: int16_t,
    pub g: loc,
    pub rising: bool,
    pub leaving: bool,
    pub fleeing: bool,
    pub fleeing_lunal: bool,
    pub fleeing_munchkin: bool,
    pub fleeing_to_town: bool,
    pub ignoring: bool,
    pub less: bool,
    pub waiting: bool,
    pub recalling: ::core::ffi::c_int,
    pub descending: ::core::ffi::c_int,
    pub shop: int16_t,
    pub ware: int16_t,
    pub item: int16_t,
    pub do_best: bool,
    pub best_item: *mut borg_best,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_15 {
    pub now: borg_time,
    pub level: borg_time,
    pub town: borg_time,
    pub morgoth: borg_time,
    pub antisummon: borg_time,
    pub call_light: borg_time,
    pub wizard_light: borg_time,
    pub detect_traps: borg_time,
    pub detect_doors: borg_time,
    pub detect_walls: borg_time,
    pub detect_evil: borg_time,
    pub detect_obj: borg_time,
    pub last_kill_mult: borg_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_16 {
    pub active: bool,
    pub cancel: bool,
    pub save: bool,
    pub old_depth: int16_t,
    pub respawning: int16_t,
    pub cheat_death: bool,
    pub anti_summon: bool,
    pub digging: bool,
    pub need_alter: bool,
    pub no_alter: bool,
    pub redraw: bool,
    pub vault: bool,
    pub desperate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamedH_17 {
    pub need_see_invis: int16_t,
    pub see_inv: int16_t,
    pub res_fire: bool,
    pub res_cold: bool,
    pub res_acid: bool,
    pub res_elec: bool,
    pub res_pois: bool,
    pub prot_from_evil: bool,
    pub fast: bool,
    pub bless: bool,
    pub hero: bool,
    pub berserk: bool,
    pub fastcast: bool,
    pub regen: bool,
    pub smite_evil: bool,
    pub venom: bool,
    pub shield: bool,
}
extern "C" {
    pub static mut inkey_hack: Option<unsafe extern "C" fn(::core::ffi::c_int) -> keypress>;
}
pub use crate::c2rust::borg::borg;
pub use crate::c2rust::borg::borg_cfg;
pub use crate::c2rust::borg::borg_engine_date;
pub use crate::c2rust::borg::borg_rand_local;
pub use crate::c2rust::borg::borg_step;
pub use crate::c2rust::borg::borg_update_entrypoint;
pub use crate::c2rust::borg::w_x;
pub use crate::c2rust::borg::w_y;
