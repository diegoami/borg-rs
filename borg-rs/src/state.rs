//! All Borg state in one place (the C globals of `angband/src/borg/*.c`).
//!
//! Each converted module adds its own sub-struct here. During the
//! conversion, code that is still c2rust reaches the single instance
//! through `crate::bridge::st()`. Converted code receives `&mut BorgState`
//! explicitly.

use crate::borg::{Borg, CoreState};
use crate::borg_cave::CaveState;
use crate::borg_io::IoState;
use crate::borg_item_activation::ActivationIds;

#[derive(Default)]
pub struct BorgState {
    /// borg.c: `borg` (what the Borg knows about itself)
    pub borg: Borg,
    /// borg.c: the other globals (`borg_cfg`, `borg_rand_local`, ...)
    pub core: CoreState,
    /// borg-cave.c
    pub cave: CaveState,
    /// borg-io.c
    pub io: IoState,
    /// borg-item-activation.c: `act_*`
    pub act: ActivationIds,
}
