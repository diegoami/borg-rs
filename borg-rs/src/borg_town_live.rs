//! Opt-in execution of the same planner that produces v5 shadow records.
//! One purchase at a time; whole-stack dispositions. Replan on store arrival
//! and after every transaction, so ware/pack indices are never used stale.
use crate::borg_headers::borg_h::{BORG_TOWN_PLAN, BORG_TOWN_NO_STORES};
use crate::borg_io::{borg_keypress, borg_keypresses, borg_note_bytes, KC_ENTER};
use crate::borg_town_record::action;
use crate::borg_town_stock::{KindOffer, Stock};
use crate::state::BorgState;

#[derive(Default)]
pub struct LiveState {
    pub action: i64,
    pub quantity: i32,
    pub shop: i16,
    pub slot: i16,
    pub direct: bool,
    visit_clock: Option<i32>,
    last_sale: Option<(i16, u8, u8, i32)>,
    rejected_sales: Vec<(i16, u8, u8)>,
}

pub fn enabled(st: &BorgState) -> bool {
    st.cfg(BORG_TOWN_PLAN) == 1 && st.cfg(BORG_TOWN_NO_STORES) == 0
}

/// Bind only valid action shapes; the executor rechecks real quantities.
pub fn bind(st: &mut BorgState, code: i64, act: [i64; 4]) -> bool {
    st.borg.goal.shop = -1;
    st.borg.goal.ware = -1;
    st.borg.goal.item = -1;
    st.town_live.direct = false;
    st.town_live.quantity = 0;
    if code == action::LEAVE { return false; }
    if act[1] < 0 || act[1] > 255 || act[2] < 1 { return false; }
    let shop = match code {
        action::BUY | action::SELL if (0..7).contains(&act[0]) => act[0] as i16,
        action::STORE | action::TAKE_HOME | action::SELL_HOME => 7,
        action::DROP => -1,
        _ => return false,
    };
    st.town_live.action = code;
    st.town_live.shop = shop;
    st.town_live.slot = act[1] as i16;
    st.town_live.quantity = if code == action::BUY { 1 } else { act[2].min(255) as i32 };
    st.borg.goal.shop = shop;
    if matches!(code, action::BUY | action::TAKE_HOME | action::SELL_HOME) {
        st.borg.goal.ware = act[1] as i16;
    } else { st.borg.goal.item = act[1] as i16; }
    true
}

pub fn quantity(st: &BorgState, sale: bool) -> Option<i32> {
    if !enabled(st) || st.borg.goal.shop != st.town_live.shop { return None; }
    let slot = if sale { st.borg.goal.item } else { st.borg.goal.ware };
    if slot != st.town_live.slot || st.town_live.quantity <= 0 { return None; }
    Some(st.town_live.quantity)
}

/// Prevent trading a kind back to the shop it was sold to this visit.
pub fn exclude_sold(st: &BorgState, stock: &mut Stock) {
    let sold = &st.store_sell;
    let n = sold.sold_item_num.clamp(0, 10) as usize;
    stock.candidates.retain(|c| {
        let it = &st.store.shops[c.shop as usize].ware[c.slot as usize];
        !(0..n).any(|i| sold.sold_item_store[i] == i32::from(c.shop)
            && sold.sold_item_tval[i] == i32::from(it.tval) && sold.sold_item_sval[i] == i32::from(it.sval))
    });
    let mut kinds = std::collections::BTreeMap::<u32, Vec<usize>>::new();
    for (i, c) in stock.candidates.iter().enumerate() { kinds.entry(c.kind).or_default().push(i); }
    stock.offers = kinds.into_iter().map(|(kind, mut wares)| {
        wares.sort_by_key(|&i| { let c = &stock.candidates[i]; (c.price, c.shop, c.slot) });
        KindOffer { kind, wares }
    }).collect();
}

/// Free known Home cures precede cash purchases for current urgent needs.
pub fn home_urgent(kit: &[crate::borg_town_dispose::KitItem], required: u8) -> Option<(u8, u8)> {
    kit.iter().filter(|k| k.place == crate::borg_town_dispose::Place::Home && k.qty > 0 && !k.unknown_flavour)
        .filter_map(|k| { let served = k.serves & required; (served != 0).then(|| (k.slot, served.trailing_zeros() as u8)) })
        .min_by_key(|&(slot, _)| slot)
}

/// Refresh the quotes the shop UI actually uses. This is deliberately live
/// only; the current control retains its historical appraisal and key stream.
pub fn refresh_prices(st: &mut BorgState) {
    let BorgState { access, store, .. } = st;
    for shop in 0..7 {
        for (slot, object) in crate::engine::store_stock(access, shop).into_iter().enumerate() {
            store.shops[shop].ware[slot].cost = object.purchase_price(shop);
        }
    }
}

fn held(st: &BorgState, tval: u8, sval: u8) -> i32 {
    st.items.items.iter().filter(|i| i.tval == tval && i.sval == sval).map(|i| i32::from(i.iqty)).sum()
}

pub fn rejected_sale(st: &BorgState, it: &crate::borg_headers::borg_item_h::borg_item, shop: i16) -> bool {
    enabled(st) && st.town_live.rejected_sales.contains(&(shop, it.tval, it.sval))
}

/// Infer refusal only from the observed unchanged holdings after the queued
/// transaction, never from the diagnostic hidden value of an unknown kind.
pub fn remember_sale(st: &mut BorgState) {
    if quantity(st, true).is_none() { return; }
    let i = st.borg.goal.item as usize;
    if let Some(it) = st.items.items.get(i) {
        st.town_live.last_sale = Some((st.borg.goal.shop, it.tval, it.sval, held(st, it.tval, it.sval)));
    }
}

fn observe_sale(st: &mut BorgState) {
    if let Some((shop, tval, sval, before)) = st.town_live.last_sale.take() {
        if held(st, tval, sval) >= before {
            st.town_live.rejected_sales.push((shop, tval, sval));
            st.borg.goal.shop = -1; st.borg.goal.item = -1; st.borg.goal.ware = -1;
        }
    }
}

pub fn choose(st: &mut BorgState) -> bool {
    st.town_live.direct = false;
    if st.town_live.visit_clock != Some(st.borg.time.level) {
        st.town_live.visit_clock = Some(st.borg.time.level);
        st.town_live.last_sale = None; st.town_live.rejected_sales.clear();
        st.buy.bought_item_num = 0; st.buy.bought_item_nxt = 0;
        st.store_sell.sold_item_num = 0; st.store_sell.sold_item_nxt = 0;
    }
    observe_sale(st);
    if !st.borg.in_shop && (0..8).contains(&st.borg.goal.shop) { return true; }
    let Some(record) = crate::borg_town_shadow::plan_visit(st, 1) else { return false; };
    if record.action == action::DROP && st.borg.in_shop {
        // The store UI has its own 'd' command. Leave, then drop outside.
        return bind(st, action::LEAVE, [-1; 4]);
    }
    if !bind(st, record.action, record.act) { return false; }
    let note = format!("# Town plan live: action={} shop={} slot={} quantity={} need={}",
        record.action, st.town_live.shop, st.town_live.slot, st.town_live.quantity, record.for_need);
    borg_note_bytes(st, note.as_bytes());
    if record.action == action::DROP {
        let slot = st.borg.goal.item as usize;
        if slot >= crate::borg_item::inven_wield() as usize || st.items.items[slot].iqty == 0 { return false; }
        // Mark dropped junk so exploration does not pick it straight back up.
        borg_keypress(st, u32::from(b'{'));
        borg_keypress(st, crate::engine::all_letters_nohjkl(slot as i32) as u32);
        borg_keypresses(st, b"borg ignore"); borg_keypress(st, KC_ENTER);
        borg_keypress(st, u32::from(b'd'));
        borg_keypress(st, crate::engine::all_letters_nohjkl(slot as i32) as u32);
        if st.items.items[slot].iqty > 1 { borg_keypress(st, u32::from(b'*')); borg_keypress(st, KC_ENTER); }
        st.borg.goal.item = -1;
        st.town_live.direct = true;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    fn state() -> BorgState {
        let mut s = BorgState::default();
        s.core.cfg = crate::borg_init::BORG_SETTINGS.iter().map(|s| s.default_value).collect();
        s
    }
    #[test]
    fn control_and_negative_baseline_cannot_enable_live_actions() {
        let mut s = state(); assert!(!enabled(&s));
        s.core.cfg[BORG_TOWN_PLAN as usize] = 1; assert!(enabled(&s));
        s.core.cfg[BORG_TOWN_NO_STORES as usize] = 1; assert!(!enabled(&s));
    }
    #[test]
    fn binding_replaces_stale_goals_and_distinguishes_home_directions() {
        let mut s = state(); s.core.cfg[BORG_TOWN_PLAN as usize] = 1;
        assert!(bind(&mut s, action::STORE, [7, 2, 17, 0]));
        assert_eq!((s.borg.goal.shop, s.borg.goal.item, s.borg.goal.ware), (7, 2, -1));
        assert_eq!(quantity(&s, true), Some(17));
        assert!(bind(&mut s, action::TAKE_HOME, [7, 5, 4, 0]));
        assert_eq!((s.borg.goal.shop, s.borg.goal.item, s.borg.goal.ware), (7, -1, 5));
        assert_eq!(quantity(&s, false), Some(4));
        assert!(bind(&mut s, action::BUY, [4, 3, 8, 20]));
        assert_eq!(quantity(&s, false), Some(1));
        s.borg.goal.ware = 9; assert_eq!(quantity(&s, false), None);
        assert!(!bind(&mut s, action::LEAVE, [-1; 4]));
        assert_eq!((s.borg.goal.shop, s.borg.goal.item, s.borg.goal.ware), (-1, -1, -1));
    }
    #[test]
    fn free_known_home_cure_precedes_spending_for_an_urgent_condition() {
        use crate::borg_town_dispose::{KitItem, Place};
        let known = KitItem { place: Place::Home, slot: 3, kind: 1, qty: 2, weight: 4,
            need: 2, reduction: 20000, serves: (1 << 2) | (1 << 3), power: 0,
            proceeds: 0, sell_shop: 4, unknown: false, unknown_flavour: false, just_bought: false };
        assert_eq!(home_urgent(&[known], 1 << 3), Some((3, 3)));
        assert_eq!(home_urgent(&[KitItem { unknown_flavour: true, ..known }], 1 << 3), None);
        assert_eq!(home_urgent(&[known], 1 << 1), None);
    }
    #[test]
    fn sold_veto_retains_other_shops_and_reindexes_offers() {
        use crate::borg_town_stock::tests::{item, town, build};
        let shops = town([&[item(2, 3, 4)], &[], &[], &[], &[], &[], &[item(2, 3, 12)], &[]]);
        let mut stock = build(&shops, &[]);
        let mut s = state();
        s.store.shops = shops.into_iter().map(|ware| crate::borg_store::BorgShop { ware }).collect();
        s.store_sell.sold_item_num = 1; s.store_sell.sold_item_store[0] = 0;
        s.store_sell.sold_item_tval[0] = 2; s.store_sell.sold_item_sval[0] = 0;
        exclude_sold(&s, &mut stock);
        assert_eq!(stock.candidates.len(), 1);
        assert_eq!(stock.candidates[0].shop, 6);
        assert_eq!(stock.offers[0].wares, [0]);
    }
    #[test]
    fn an_observed_refusal_blocks_only_that_buyer_and_success_does_not() {
        let mut s = state(); s.core.cfg[BORG_TOWN_PLAN as usize] = 1;
        let mut it = crate::borg_item::blank_item(); it.tval = 25; it.sval = 3; it.iqty = 2;
        s.items.items = vec![it];
        bind(&mut s, action::SELL, [4, 0, 1, 0]); remember_sale(&mut s);
        s.items.items[0].iqty = 1; observe_sale(&mut s);
        assert!(!rejected_sale(&s, &it, 4));
        bind(&mut s, action::SELL, [4, 0, 1, 0]); remember_sale(&mut s); observe_sale(&mut s);
        assert!(rejected_sale(&s, &it, 4)); assert!(!rejected_sale(&s, &it, 6));
        assert_eq!(s.borg.goal.shop, -1);
    }
    #[test]
    fn invalid_routes_cannot_index_a_shop() {
        let mut s = state();
        for a in [[-1, 0, 1, 0], [8, 0, 1, 0], [4, -1, 1, 0], [4, 0, 0, 0]] {
            assert!(!bind(&mut s, action::BUY, a)); assert_eq!(s.borg.goal.shop, -1);
        }
    }
}
