//! All Borg state in one place (the C globals of `angband/src/borg/*.c`).
//!
//! Each converted module adds its own sub-struct here. During the
//! conversion, code that is still c2rust reaches the single instance
//! through `crate::bridge::st()`. Converted code receives `&mut BorgState`
//! explicitly.

use crate::borg_cave::CaveState;

#[derive(Default)]
pub struct BorgState {
    /// borg-cave.c
    pub cave: CaveState,
}
