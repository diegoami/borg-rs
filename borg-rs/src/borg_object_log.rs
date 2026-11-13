//! The session's object log: the objects the Borg uses, buys and picks up.
//!
//! Enabled with the environment variable `BORG_OBJECT_LOG=<file>`. When it is
//! unset the module does nothing at all: no file is opened, no line is
//! printed, and the run is the run without it. The log never presses a key
//! and never changes a decision, so the keypress stream of a run with the log
//! is the stream of a run without it.
//!
//! Every event is printed on the console (the game's stdout) and appended to
//! the file, numbered from 1, so the end-of-session feedback can point at a
//! line:
//!
//! ```text
//! 0007 turn 142 depth 23 quaff a Potion of Healing
//! 0008 turn 150 depth 0 buy a Scroll of Light at the General Store for 25 gold
//! 0009 turn 150 depth 0 acquire 2 x a Potion of Healing
//! ```
//!
//! Three kinds of event:
//!
//! * `quaff`, `read`, `use`, `zap` - the Borg used an object it carried.
//! * `buy` - the Borg bought an object, with the store and the price.
//! * `acquire` - something appeared in the pack that was not there before.
//!   This is the player's side of the session: with the handover the player
//!   shops in town, and the pack is the only place that shows it. It cannot
//!   tell a purchase from a pickup, so it says `acquire` and not `buy`.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::borg_headers::borg_item_h::borg_item;
use crate::borg_headers::borg_trait_h::BI_CDEPTH;
use crate::state::BorgState;

/// The number of events logged so far.
static COUNT: AtomicU64 = AtomicU64::new(0);

/// The log file: `BORG_OBJECT_LOG`.
fn log_file() -> Option<&'static Mutex<File>> {
    static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    FILE.get_or_init(|| {
        let path_var = "BORG_OBJECT_LOG";
        let Ok(path) = std::env::var(path_var) else {
            return None;
        };
        if path.is_empty() {
            return None;
        }
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => Some(Mutex::new(file)),
            Err(err) => {
                eprintln!("borg: {path_var} {path}: {err}");
                None
            }
        }
    })
    .as_ref()
}

/// The item's description up to its NUL (`item->desc`).
fn item_desc(item: &borg_item) -> String {
    let bytes: Vec<u8> = item
        .desc
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect();
    String::from_utf8_lossy(&bytes).trim().to_string()
}

/// Write one event: on the console and in the file.
fn emit(st: &BorgState, action: &str, details: &str) {
    let Some(file) = log_file() else {
        return;
    };
    let n = COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let turn = st.borg.time.now;
    let depth = st.borg.tr(BI_CDEPTH);
    let line = format!("{n:04} turn {turn} depth {depth} {action} {details}\n");

    // The console: the owner watches the run and sees the objects go by.
    print!("{line}");

    if let Ok(mut file) = file.lock() {
        let _ = file.write_all(line.as_bytes());
        let _ = file.flush();
    }
}

/// The Borg used an object it carried (`quaff`, `read`, `use`, `zap`).
pub fn log_use(st: &BorgState, action: &str, item: &borg_item) {
    emit(st, action, &item_desc(item));
}

/// The Borg bought an object: the store and the price with it.
pub fn log_buy(st: &BorgState, item: &borg_item, shop: &str, price: i32) {
    emit(
        st,
        "buy",
        &format!("{} at the {} for {price} gold", item_desc(item), shop),
    );
}

/// Something new appeared in the pack.
fn log_acquire(st: &BorgState, desc: &str, count: u32) {
    if count == 1 {
        emit(st, "acquire", desc);
    } else {
        emit(st, "acquire", &format!("{count} x {desc}"));
    }
}

/// One pack slot, as the snapshot keeps it.
type Slot = (u8, u8, i16, u8, String);

/// The pack as it was the last time we looked.
fn previous_pack() -> &'static Mutex<Option<Vec<Slot>>> {
    static PACK: OnceLock<Mutex<Option<Vec<Slot>>>> = OnceLock::new();
    PACK.get_or_init(|| Mutex::new(None))
}

/// The pack now: the slots that hold something.
fn current_pack(st: &BorgState) -> Vec<Slot> {
    let mut out = Vec::new();
    let n = i32::from(crate::engine::z_info().pack_size);
    for i in 0..n {
        let item = &st.items.items[i as usize];
        if item.iqty == 0 {
            continue;
        }
        out.push((
            item.tval,
            item.sval,
            item.pval,
            item.iqty,
            item_desc(item),
        ));
    }
    out
}

/// Report what appeared in the pack since the last look.
///
/// The first look only takes the baseline: the starting kit is not an
/// acquisition. After that, every kind that grew is reported with the number
/// it grew by.
pub fn log_acquisitions(st: &BorgState) {
    let now = current_pack(st);
    let mut previous = previous_pack().lock().unwrap_or_else(|err| err.into_inner());
    let Some(old) = previous.as_ref() else {
        *previous = Some(now);
        return;
    };

    // Count every kind in the old pack and in the new one.
    let mut old_counts: HashMap<(u8, u8, i16), u32> = HashMap::new();
    for (tval, sval, pval, qty, _) in old {
        *old_counts.entry((*tval, *sval, *pval)).or_insert(0) += u32::from(*qty);
    }
    let mut new_counts: HashMap<(u8, u8, i16), u32> = HashMap::new();
    for (tval, sval, pval, qty, _) in &now {
        *new_counts.entry((*tval, *sval, *pval)).or_insert(0) += u32::from(*qty);
    }

    for ((tval, sval, pval), count) in &new_counts {
        let before = old_counts.get(&(*tval, *sval, *pval)).copied().unwrap_or(0);
        if *count <= before {
            continue;
        }
        let grown = count - before;
        // The description of the kind: the first slot that holds it.
        if let Some((_, _, _, _, desc)) = now
            .iter()
            .find(|(t, s, p, _, _)| (*t, *s, *p) == (*tval, *sval, *pval))
        {
            log_acquire(st, desc, grown);
        }
    }

    *previous = Some(now);
}
