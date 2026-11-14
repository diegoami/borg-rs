//! Execute the town plan through the existing store UI, refreshing after
//! every transaction. Default-off: the historical chooser remains control.
use crate::borg_headers::borg_h::{BORG_TOWN_NO_STORES, BORG_TOWN_PLAN};
use crate::borg_headers::borg_trait_h::BI_CDEPTH;
use crate::borg_io::{borg_keypress, borg_keypresses, borg_note, KC_ENTER};
use crate::borg_town_record::action;
use crate::borg_town_shadow::TownAction;
use crate::state::BorgState;

#[derive(Clone, PartialEq, Eq)]
struct SaleStamp {
    shop: i16,
    slot: usize,
    desc: [core::ffi::c_char; 80],
    qty: u8,
    value: i32,
}

impl SaleStamp {
    fn of(shop: i16, slot: usize, item: &crate::borg_headers::borg_item_h::borg_item) -> Self {
        Self { shop, slot, desc: item.desc, qty: item.iqty, value: item.value }
    }
    fn matches(&self, item: &crate::borg_headers::borg_item_h::borg_item) -> bool {
        self.desc == item.desc && self.qty == item.iqty && self.value == item.value
    }
}

#[derive(Default)]
pub struct LiveState {
    clock: Option<i32>,
    pending_sale: Option<SaleStamp>,
    rejected_sales: Vec<SaleStamp>,
}

/// The native store may reject an otherwise plausible offer (for example,
/// its stock is full). A queued command is not proof of a transaction.
fn settle_sale(st: &mut BorgState) {
    if st.think.town_live.clock != Some(st.borg.time.level) {
        st.think.town_live = LiveState { clock: Some(st.borg.time.level), ..LiveState::default() };
    }
    if let Some(attempt) = st.think.town_live.pending_sale.take() {
        if st.items.items.get(attempt.slot).is_some_and(|it| attempt.matches(it)) {
            st.think.town_live.rejected_sales.push(attempt);
            borg_note(st, "# Town plan: sale made no inventory change; exclude that offer this visit");
        }
    }
}

pub fn sale_rejected(st: &BorgState, shop: i16, item: &crate::borg_headers::borg_item_h::borg_item) -> bool {
    st.think.town_live.rejected_sales.iter().any(|s| s.shop == shop && s.matches(item))
}

pub fn active(st: &BorgState) -> bool { st.cfg(BORG_TOWN_PLAN) == 1 }

fn clear(st: &mut BorgState) {
    st.borg.goal.shop = -1;
    st.borg.goal.ware = -1;
    st.borg.goal.item = -1;
    st.think.town_action = None;
}

/// Map model actions to shop goals. A Home sale first withdraws from Home;
/// the next solve then sees an ordinary carried stack eligible for sale.
fn goal(a: TownAction) -> Option<(i16, i16, i16)> {
    let slot = i16::try_from(a.at[1]).ok()?;
    if slot < 0 || a.at[2] <= 0 { return None; }
    match a.code {
        action::BUY if (0..7).contains(&a.at[0]) => Some((a.at[0] as i16, slot, -1)),
        action::SELL if (0..7).contains(&a.at[0]) => Some((a.at[0] as i16, -1, slot)),
        action::STORE => Some((7, -1, slot)),
        action::TAKE_HOME | action::SELL_HOME => Some((7, slot, -1)),
        action::DROP => Some((-1, -1, slot)),
        _ => None,
    }
}

fn visit_gate(st: &mut BorgState) -> Option<bool> {
    if st.borg.tr(BI_CDEPTH) != 0 { return Some(false); }
    if st.cfg(BORG_TOWN_NO_STORES) != 0 { clear(st); return Some(false); }
    if let Some(shop) = st.borg.shop_override {
        clear(st);
        st.borg.goal.shop = shop;
        return Some(true);
    }
    None
}

pub fn choose(st: &mut BorgState) -> bool {
    settle_sale(st);
    if let Some(result) = visit_gate(st) { return result; }
    if st.borg.time.now.wrapping_sub(st.borg.time.level) > 2000
        || st.borg.antibounce_count > 1350 { clear(st); return false; }
    // While walking, retain only the destination. On entering a store the
    // plan is recomputed against freshly read inventory and stock indices.
    if !st.borg.in_shop && st.borg.goal.shop >= 0 && st.think.town_action.is_some() {
        return true;
    }
    let a = crate::borg_town_shadow::live_action(st);
    let Some((shop, ware, item)) = goal(a) else { clear(st); return false; };
    st.borg.goal.shop = shop;
    st.borg.goal.ware = ware;
    st.borg.goal.item = item;
    st.think.town_action = Some(a);
    true
}

pub fn execute_store(st: &mut BorgState) -> bool {
    let Some(a) = st.think.town_action else { return false; };
    if st.borg.goal.shop != st.think.shop_num || st.think.shop_num < 0 { return false; }
    // Reject a transient Home withdrawal without carrying room. Replanning
    // may request it after a pack disposition; never send an invalid slot.
    if matches!(a.code, action::TAKE_HOME | action::SELL_HOME) {
        let Some(item) = st.store.shops[7].ware.get(a.at[1] as usize).copied() else { clear(st); return false; };
        if item.iqty == 0 { clear(st); return false; }
        if crate::borg_inventory::borg_first_empty_inventory_slot(st) < 0
            && !st.items.items.iter().take(crate::borg_item::inven_wield() as usize)
                .any(|held| held.iqty > 0 && crate::borg_store_sell::borg_object_similar(st, held, &item)) {
            borg_note(st, "# Town plan: Home withdrawal blocked by pack space");
            clear(st);
            return false;
        }
    }
    borg_note(st, &format!("# Town plan executing action {} at shop {} slot {}", a.code, st.think.shop_num, a.at[1]));
    let executed = match a.code {
        action::BUY | action::TAKE_HOME => crate::borg_store_buy::borg_think_shop_buy(st),
        action::SELL_HOME => crate::borg_store_buy::take_selected_for_sale(st),
        action::SELL | action::STORE => {
            if a.code == action::SELL {
                st.think.town_live.pending_sale = Some(SaleStamp::of(st.think.shop_num, a.at[1] as usize, &st.items.items[a.at[1] as usize]));
            }
            crate::borg_store_sell::sell_one_selected(st)
        }
        _ => false,
    };
    if executed { st.think.town_action = None; }
    executed
}

/// A drop is an outside action, not a request to route to shop -1.
pub fn execute_outside(st: &mut BorgState) -> bool {
    if !active(st) || st.borg.in_shop { return false; }
    let Some(a) = st.think.town_action else { return false; };
    if a.code != action::DROP { return false; }
    let slot = a.at[1] as usize;
    let Some(item) = st.items.items.get(slot) else { clear(st); return false; };
    let qty = item.iqty;
    if qty == 0 || slot >= crate::borg_item::inven_wield() as usize { clear(st); return false; }
    let letter = crate::engine::all_letters_nohjkl(slot as i32) as i8 as u32;
    borg_note(st, "# Town plan dropping worthless stack");
    borg_keypress(st, u32::from(b'{'));
    borg_keypress(st, letter);
    borg_keypresses(st, b"borg ignore");
    borg_keypress(st, KC_ENTER);
    borg_keypress(st, u32::from(b'd'));
    borg_keypress(st, letter);
    if qty > 1 {
        borg_keypresses(st, qty.to_string().as_bytes());
        borg_keypress(st, KC_ENTER);
    }
    clear(st);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_queued_sale_requires_observed_inventory_progress() {
        let mut it = crate::borg_item::blank_item();
        it.iqty = 3;
        it.value = 20;
        let stamp = SaleStamp::of(4, 0, &it);
        assert!(stamp.matches(&it), "unchanged item is not a completed sale");
        it.iqty = 2;
        assert!(!stamp.matches(&it), "one copy really left");
        it.iqty = 3;
        it.desc[0] = b'K' as core::ffi::c_char;
        assert!(!stamp.matches(&it), "a different item in the slot must not be rejected");
    }
    fn state() -> BorgState {
        let mut st = BorgState::default();
        st.core.cfg = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
        st.borg.trait_ = vec![0; crate::borg_headers::borg_trait_h::BI_MAX as usize];
        st.core.cfg[BORG_TOWN_PLAN as usize] = 1;
        st
    }
    #[test]
    fn disabled_stores_cancel_a_live_destination() {
        let mut st = state();
        st.core.cfg[BORG_TOWN_NO_STORES as usize] = 1;
        st.borg.goal.shop = 4;
        st.think.town_action = Some(TownAction { code: action::BUY, at: [4, 2, 1, 10] });
        assert_eq!(visit_gate(&mut st), Some(false));
        assert_eq!(st.borg.goal.shop, -1);
        assert!(st.think.town_action.is_none());
    }
    #[test]
    fn manual_shop_override_cancels_cached_actions() {
        let mut st = state();
        st.borg.shop_override = Some(7);
        st.think.town_action = Some(TownAction { code: action::BUY, at: [4, 2, 1, 10] });
        assert_eq!(visit_gate(&mut st), Some(true));
        assert_eq!((st.borg.goal.shop, st.borg.goal.ware, st.borg.goal.item), (7, -1, -1));
        assert!(st.think.town_action.is_none());
    }
    #[test]
    fn actions_route_pack_and_home_indices_to_the_correct_ui() {
        let a = |code, shop, slot| TownAction { code, at: [shop, slot, 1, 0] };
        assert_eq!(goal(a(action::BUY, 4, 2)), Some((4, 2, -1)));
        assert_eq!(goal(a(action::SELL, 1, 3)), Some((1, -1, 3)));
        assert_eq!(goal(a(action::STORE, 7, 3)), Some((7, -1, 3)));
        assert_eq!(goal(a(action::TAKE_HOME, 7, 5)), Some((7, 5, -1)));
        assert_eq!(goal(a(action::SELL_HOME, 4, 5)), Some((7, 5, -1)));
        assert_eq!(goal(a(action::DROP, -1, 3)), Some((-1, -1, 3)));
        assert_eq!(goal(a(action::LEAVE, -1, -1)), None);
        assert_eq!(goal(a(action::BUY, 99, 2)), None);
        assert_eq!(goal(a(action::SELL, 1, -1)), None);
    }
}
