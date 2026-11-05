//! `/home/user/borg-rs/angband/src/borg/borg-flow-kill.h`
//!
//! borg-flow-kill.c is converted (`crate::borg_flow_kill`): the struct is
//! `BorgKill`, the globals live in `BorgState::kills`, and the functions
//! are C-ABI shims in `crate::bridge::flow_kill`.
pub use crate::borg_flow_kill::BorgKill as borg_kill;
pub use crate::bridge::flow_kill::{
    borg_count_death, borg_create_kill, borg_delete_kill, borg_flow_kill, borg_flow_kill_aim,
    borg_flow_kill_corridor, borg_flow_kill_direct, borg_follow_kill, borg_free_flow_kill,
    borg_init_flow_kill, borg_locate_kill, borg_near_monster_type, borg_race_name,
    borg_shoot_scoot_safe, borg_sleep_kill, observe_kill_diff, observe_kill_move,
};
