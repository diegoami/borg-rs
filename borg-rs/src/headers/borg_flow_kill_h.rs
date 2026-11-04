//! `/home/user/borg-rs/angband/src/borg/borg-flow-kill.h`
pub use super::__stddef_wchar_t_h::wchar_t;
pub use super::borg_h::borg_time;
pub use super::borg_h::C2RustUnnamedH_10;
pub use super::borg_h::C2RustUnnamedH_11;
pub use super::borg_h::C2RustUnnamedH_12;
pub use super::borg_h::C2RustUnnamedH_13;
pub use super::borg_h::C2RustUnnamedH_14;
pub use super::borg_h::C2RustUnnamedH_15;
pub use super::borg_h::C2RustUnnamedH_16;
pub use super::borg_h::C2RustUnnamedH_17;
pub use super::player_h::player;
pub use super::stdint_intn_h::int16_t;
pub use super::stdint_intn_h::int32_t;
pub use super::stdint_uintn_h::uint16_t;
pub use super::stdint_uintn_h::uint32_t;
pub use super::stdint_uintn_h::uint8_t;
pub use super::z_type_h::loc;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_kill {
    pub r_idx: uint16_t,
    pub known: bool,
    pub awake: bool,
    pub confused: bool,
    pub afraid: bool,
    pub quiver: bool,
    pub stunned: bool,
    pub poisoned: bool,
    pub seen: bool,
    pub used: bool,
    pub pos: loc,
    pub ox: uint8_t,
    pub oy: uint8_t,
    pub speed: uint8_t,
    pub moves: uint8_t,
    pub ranged_attack: uint8_t,
    pub spell: [uint8_t; 92],
    pub power: int16_t,
    pub injury: int16_t,
    pub other: int16_t,
    pub level: int16_t,
    pub spell_flags: [uint32_t; 85],
    pub when: borg_time,
    pub m_idx: int16_t,
}
pub use crate::c2rust::borg_flow_kill::borg_count_death;
pub use crate::c2rust::borg_flow_kill::borg_create_kill;
pub use crate::c2rust::borg_flow_kill::borg_delete_kill;
pub use crate::c2rust::borg_flow_kill::borg_flow_kill;
pub use crate::c2rust::borg_flow_kill::borg_flow_kill_aim;
pub use crate::c2rust::borg_flow_kill::borg_flow_kill_corridor;
pub use crate::c2rust::borg_flow_kill::borg_flow_kill_direct;
pub use crate::c2rust::borg_flow_kill::borg_follow_kill;
pub use crate::c2rust::borg_flow_kill::borg_free_flow_kill;
pub use crate::c2rust::borg_flow_kill::borg_init_flow_kill;
pub use crate::c2rust::borg_flow_kill::borg_kills;
pub use crate::c2rust::borg_flow_kill::borg_kills_cnt;
pub use crate::c2rust::borg_flow_kill::borg_kills_nxt;
pub use crate::c2rust::borg_flow_kill::borg_locate_kill;
pub use crate::c2rust::borg_flow_kill::borg_nasties;
pub use crate::c2rust::borg_flow_kill::borg_nasties_count;
pub use crate::c2rust::borg_flow_kill::borg_nasties_limit;
pub use crate::c2rust::borg_flow_kill::borg_nasties_num;
pub use crate::c2rust::borg_flow_kill::borg_near_monster_type;
pub use crate::c2rust::borg_flow_kill::borg_race_count;
pub use crate::c2rust::borg_flow_kill::borg_race_death;
pub use crate::c2rust::borg_flow_kill::borg_race_name;
pub use crate::c2rust::borg_flow_kill::borg_shoot_scoot_safe;
pub use crate::c2rust::borg_flow_kill::borg_sleep_kill;
pub use crate::c2rust::borg_flow_kill::observe_kill_diff;
pub use crate::c2rust::borg_flow_kill::observe_kill_move;
