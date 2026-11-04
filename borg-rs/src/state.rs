//! All Borg state in one place (the C globals of `angband/src/borg/*.c`).
//!
//! Each converted module adds its own sub-struct here. During the
//! conversion, code that is still c2rust reaches the single instance
//! through `crate::bridge::st()`. Converted code receives `&mut BorgState`
//! explicitly.

use crate::borg::{Borg, CoreState};
use crate::borg_cave::CaveState;
use crate::borg_cave_light::LightState;
use crate::borg_cave_view::ViewState;
use crate::borg_formulas::FormulasState;
use crate::borg_formulas_calc::CalcState;
use crate::borg_home_notice::HomeState;
use crate::borg_inventory::InventoryState;
use crate::borg_io::IoState;
use crate::borg_item::ItemState;
use crate::borg_item_activation::ActivationIds;
use crate::borg_item_val::ItemValState;
use crate::borg_magic::MagicState;
use crate::borg_messages_react::MsgState;
use crate::borg_prepared::PreparedState;
use crate::borg_projection::ProjectionState;

#[derive(Default)]
pub struct BorgState {
    /// borg.c: `borg` (what the Borg knows about itself)
    pub borg: Borg,
    /// borg.c: the other globals (`borg_cfg`, `borg_rand_local`, ...)
    pub core: CoreState,
    /// borg-cave.c
    pub cave: CaveState,
    /// borg-cave-light.c: `borg_light_*`, `borg_glow_*`
    pub light: LightState,
    /// borg-cave-view.c: `borg_view_n`, `borg_view_x`, `borg_view_y`
    pub view: ViewState,
    /// borg-formulas.c: `borg_formulas`
    pub formulas: FormulasState,
    /// borg-formulas-calc.c: `calculations`
    pub formulas_calc: CalcState,
    /// borg-home-notice.c: the home counters (`num_*`, `home_*`)
    pub home: HomeState,
    /// borg-inventory.c: `borg_do_crush_junk`
    pub inventory: InventoryState,
    /// borg-io.c
    pub io: IoState,
    /// borg-item.c: `borg_items`, `safe_items`
    pub items: ItemState,
    /// borg-item-activation.c: `act_*`
    pub act: ActivationIds,
    /// borg-item-val.c: `sv_*`, `kv_*`
    pub item_val: ItemValState,
    /// borg-magic.c: `borg_magics`, `borg_spell_ratings`
    pub magic: MagicState,
    /// borg-messages.c / borg-messages-react.c: the message queue
    /// (`borg_msg_*`) and `borg_dont_react`
    pub msgs: MsgState,
    /// borg-prepared.c: `borg_numb_live_unique`, `borg_first_living_unique`,
    /// `borg_depth_hunted_unique`
    pub prepared: PreparedState,
    /// borg-projection.c: `borg_target_loc`
    pub projection: ProjectionState,
}
