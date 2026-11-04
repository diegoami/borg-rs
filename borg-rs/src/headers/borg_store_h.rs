//! `/home/user/borg-rs/angband/src/borg/borg-store.h`
pub use super::borg_item_h::borg_item;
#[allow(unused_imports)]
use super::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_shop {
    pub ware: *mut borg_item,
}
pub use crate::c2rust::borg_store::borg_free_store;
pub use crate::c2rust::borg_store::borg_home_full;
pub use crate::c2rust::borg_store::borg_init_store;
pub use crate::c2rust::borg_store::borg_inventory_full;
pub use crate::c2rust::borg_store::borg_notice_store;
pub use crate::c2rust::borg_store::borg_safe_shops;
pub use crate::c2rust::borg_store::borg_shops;
pub use crate::c2rust::borg_store::SHOP_MENU_ITEMS;
