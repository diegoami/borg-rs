//! The town-visit dump: an observation-only record of every town visit.
//!
//! Enabled with the environment variable `BORG_TOWN_DUMP=<file>`. When it is
//! unset the module does nothing at all: no file is opened, no decision is
//! recorded, and the run is the run without it.
//!
//! The dump is observation only. It never presses a key, never changes a
//! goal, never changes a setting and never changes what the Borg decides.
//! The keypress stream of a run with the dump enabled is therefore the same
//! as the stream of a run without it, so the C/Rust parity suites stay
//! valid.
//!
//! Two kinds of record are written, one JSON object per line:
//!
//! * `decision` - the town visit itself: the state (gold, depth, class,
//!   race, level, hp, the traits the town strategy reads, the pack, the
//!   equipment and the home), the stock of every shop with the price of
//!   every ware, and the decision the current policy took (the goal ware or
//!   item, the shop, and the Borg's own note saying why).
//! * `purchase` - a transaction that was actually carried out: the ware, the
//!   price and the gold before and after.
//!
//! The point of the dump is to be labelled: the owner reads a decision
//! record and says what should have been bought instead, and why. So the
//! record keeps the whole shop stock (the wares that were *not* bought are
//! the contrastive signal), and it keeps the non-purchases (a decision to
//! buy nothing is a decision).

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::borg_headers::borg_item_h::borg_item;
use crate::borg_headers::borg_trait_h::{
    BI_CDEPTH, BI_CLEVEL, BI_FOOD, BI_GOLD, BI_ISCUT, BI_ISPOISONED, BI_LIGHT,
};
use crate::borg_item::inven_wield;
use crate::borg_store::BORG_HOME;
use crate::engine;
use crate::state::BorgState;

/// The dump file, opened once per process.
fn dump_file() -> Option<&'static Mutex<File>> {
    static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    FILE.get_or_init(|| {
        let Ok(path) = std::env::var("BORG_TOWN_DUMP") else {
            return None;
        };
        if path.is_empty() {
            return None;
        }
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => Some(Mutex::new(file)),
            Err(err) => {
                eprintln!("borg: BORG_TOWN_DUMP {path}: {err}");
                None
            }
        }
    })
    .as_ref()
}

/// The record number, over the whole run.
static SEQ: AtomicU64 = AtomicU64::new(0);

thread_local! {
    /// The Borg's own note about the decision it just took.
    static LAST_REASON: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Remember the note the Borg made about its decision (`borg_think_store`'s
/// `note`). The dump reads it back when the decision is recorded.
pub fn record_reason(bytes: &[u8]) {
    LAST_REASON.with(|reason| *reason.borrow_mut() = bytes.to_vec());
}

/// Take the last note (and forget it).
fn take_reason() -> Vec<u8> {
    LAST_REASON.with(|reason| std::mem::take(&mut *reason.borrow_mut()))
}

/// Append one record.
fn write_record(out: &str) {
    let Some(file) = dump_file() else { return };
    let Ok(mut file) = file.lock() else { return };
    let _ = file.write_all(out.as_bytes());
    let _ = file.write_all(b"\n");
}

/// A hex digit, lower case.
fn hex_digit(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'a' + nibble - 10) as char,
    }
}

/// Append a JSON string. The bytes are the Borg's own bytes (a description
/// is a byte string, not necessarily UTF-8), so a byte outside ASCII is
/// written as its Latin-1 code point.
fn push_str(out: &mut String, bytes: &[u8]) {
    out.push('"');
    for &byte in bytes {
        match byte {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(byte as char),
            _ => {
                out.push_str("\\u00");
                out.push(hex_digit(byte >> 4));
                out.push(hex_digit(byte & 0x0f));
            }
        }
    }
    out.push('"');
}

/// Append a JSON number.
fn push_num(out: &mut String, value: impl std::fmt::Display) {
    out.push_str(&value.to_string());
}

/// An item's description, up to its NUL.
fn desc_bytes(item: &borg_item) -> Vec<u8> {
    item.desc
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect()
}

/// One item of the pack, the equipment, the home or a shop stock.
fn push_item(out: &mut String, item: &borg_item, slot: usize, price: bool) {
    out.push_str("{\"slot\":");
    push_num(out, slot);
    out.push_str(",\"desc\":");
    push_str(out, &desc_bytes(item));
    out.push_str(",\"qty\":");
    push_num(out, item.iqty);
    out.push_str(",\"tval\":");
    push_num(out, item.tval);
    out.push_str(",\"sval\":");
    push_num(out, item.sval);
    out.push_str(",\"value\":");
    push_num(out, item.value);
    if price {
        out.push_str(",\"price\":");
    } else {
        out.push_str(",\"cost\":");
    }
    push_num(out, item.cost);
    out.push_str(",\"aware\":");
    out.push_str(if item.aware { "true" } else { "false" });
    out.push_str(",\"needs_ident\":");
    out.push_str(if item.needs_ident { "true" } else { "false" });
    out.push('}');
}

/// The state the town strategy reads.
fn push_state(out: &mut String, st: &BorgState) {
    let notice = engine::player_notice();
    let summary = engine::player_summary();

    out.push_str("\"turn\":");
    push_num(out, st.borg.time.now);
    out.push_str(",\"depth\":");
    push_num(out, st.borg.tr(BI_CDEPTH));
    out.push_str(",\"level\":");
    push_num(out, st.borg.tr(BI_CLEVEL));
    out.push_str(",\"hp\":[");
    push_num(out, notice.chp);
    out.push(',');
    push_num(out, notice.mhp);
    out.push(']');
    out.push_str(",\"gold\":");
    push_num(out, st.borg.tr(BI_GOLD));
    out.push_str(",\"race\":");
    push_str(out, summary.race_name);
    out.push_str(",\"class\":");
    push_str(out, summary.class_name);
    out.push_str(",\"name\":");
    push_str(out, &summary.full_name);
    out.push_str(",\"traits\":{\"light\":");
    push_num(out, st.borg.tr(BI_LIGHT));
    out.push_str(",\"food\":");
    push_num(out, st.borg.tr(BI_FOOD));
    out.push_str(",\"cut\":");
    push_num(out, st.borg.tr(BI_ISCUT));
    out.push_str(",\"poisoned\":");
    push_num(out, st.borg.tr(BI_ISPOISONED));
    out.push('}');
}

/// Record one town visit: the state, the shop stock, and the decision the
/// current policy took with the Borg's own reason for it.
pub fn dump_town_visit(st: &mut BorgState) {
    if dump_file().is_none() {
        return;
    }
    let reason = take_reason();

    let mut out = String::with_capacity(4096);
    out.push_str("{\"type\":\"decision\",\"seq\":");
    push_num(&mut out, SEQ.fetch_add(1, Ordering::Relaxed));
    out.push(',');
    push_state(&mut out, st);

    // The pack (slots 0..pack_size) and the equipment (the next twelve).
    let pack_end = inven_wield() as usize;
    let mut first = true;
    out.push_str(",\"pack\":[");
    for slot in 0..pack_end {
        let item = &st.items.items[slot];
        if item.iqty == 0 {
            continue;
        }
        if !first {
            out.push(',');
        }
        first = false;
        push_item(&mut out, item, slot, false);
    }
    out.push_str("],\"equipment\":[");
    first = true;
    for slot in pack_end..pack_end + 12 {
        let Some(item) = st.items.items.get(slot) else {
            break;
        };
        if item.iqty == 0 {
            continue;
        }
        if !first {
            out.push(',');
        }
        first = false;
        push_item(&mut out, item, slot, false);
    }
    out.push(']');

    // The home.
    out.push_str(",\"home\":[");
    if let Some(home) = st.store.shops.get(BORG_HOME) {
        first = true;
        for slot in 0..home.ware.len() {
            let item = &home.ware[slot];
            if item.iqty == 0 {
                continue;
            }
            if !first {
                out.push(',');
            }
            first = false;
            push_item(&mut out, item, slot, false);
        }
    }
    out.push(']');

    // The stock of every shop, with the price of every ware. The wares that
    // were not bought are the contrastive signal, so the whole stock is
    // kept.
    out.push_str(",\"shops\":[");
    first = true;
    for shop in 0..st.store.shops.len() {
        if !first {
            out.push(',');
        }
        first = false;
        out.push_str("{\"shop\":");
        push_num(&mut out, shop);
        out.push_str(",\"name\":");
        push_str(&mut out, &engine::store_feat_name(shop));
        out.push_str(",\"wares\":[");
        let wares = &st.store.shops[shop].ware;
        let mut first_ware = true;
        for (slot, item) in wares.iter().enumerate() {
            if item.iqty == 0 {
                continue;
            }
            if !first_ware {
                out.push(',');
            }
            first_ware = false;
            push_item(&mut out, item, slot, true);
        }
        out.push_str("]}");
    }
    out.push(']');

    // The decision.
    let shop = st.borg.goal.shop;
    let ware = st.borg.goal.ware;
    let item = st.borg.goal.item;
    let action = if ware >= 0 {
        if shop == BORG_HOME as i16 {
            "take_home"
        } else {
            "buy"
        }
    } else if item >= 0 {
        if shop == BORG_HOME as i16 {
            "sell_home"
        } else {
            "sell"
        }
    } else {
        "visit"
    };

    out.push_str(",\"decision\":{\"action\":");
    push_str(&mut out, action.as_bytes());
    out.push_str(",\"shop\":");
    push_num(&mut out, shop);
    out.push_str(",\"shop_name\":");
    if shop >= 0 && usize::from(shop as u16) < st.store.shops.len() {
        push_str(&mut out, &engine::store_feat_name(shop as usize));
    } else {
        push_str(&mut out, b"");
    }
    out.push_str(",\"ware\":");
    push_num(&mut out, ware);
    out.push_str(",\"ware_desc\":");
    if ware >= 0 {
        if let Some(shops) = st.store.shops.get(shop as usize) {
            if let Some(w) = shops.ware.get(ware as usize) {
                push_str(&mut out, &desc_bytes(w));
            } else {
                push_str(&mut out, b"");
            }
        } else {
            push_str(&mut out, b"");
        }
    } else {
        push_str(&mut out, b"");
    }
    out.push_str(",\"item\":");
    push_num(&mut out, item);
    out.push_str(",\"item_desc\":");
    if item >= 0 {
        if let Some(it) = st.items.items.get(item as usize) {
            push_str(&mut out, &desc_bytes(it));
        } else {
            push_str(&mut out, b"");
        }
    } else {
        push_str(&mut out, b"");
    }
    out.push_str(",\"reason\":");
    push_str(&mut out, &reason);
    out.push_str("}}");

    write_record(&out);
}

/// Record a purchase that was actually carried out.
pub fn dump_purchase(st: &mut BorgState, shop: i32, item: &borg_item) {
    if dump_file().is_none() {
        return;
    }

    let mut out = String::with_capacity(512);
    out.push_str("{\"type\":\"purchase\",\"seq\":");
    push_num(&mut out, SEQ.fetch_add(1, Ordering::Relaxed));
    out.push_str(",\"turn\":");
    push_num(&mut out, st.borg.time.now);
    out.push_str(",\"shop\":");
    push_num(&mut out, shop);
    out.push_str(",\"shop_name\":");
    if shop >= 0 && usize::from(shop as u16) < st.store.shops.len() {
        push_str(&mut out, &engine::store_feat_name(shop as usize));
    } else {
        push_str(&mut out, b"");
    }
    out.push_str(",\"gold_before\":");
    push_num(&mut out, st.borg.tr(BI_GOLD));
    out.push_str(",\"ware\":");
    push_item(&mut out, item, 0, true);
    out.push('}');

    write_record(&out);
}
