//! Complete logical map/flow snapshots beside original native C functions.
#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    unused_imports
)]
use std::{cell::RefCell, ffi::CStr};
include!(concat!(env!("OUT_DIR"), "/spatial_items.rs"));
use cave::{BorgGrid, CaveState};
use terrain::*;
use traits::*;
#[derive(Clone, Copy, Default)]
pub struct loc {
    pub x: i32,
    pub y: i32,
}
#[derive(Default)]
struct Status {
    desperate: bool,
    vault: bool,
    digging: bool,
}
#[derive(Default)]
struct Mon {
    scary: bool,
    unique: i16,
}
#[derive(Default)]
struct Goal {
    shop: i16,
    ignoring: bool,
    type_0: i16,
    g: loc,
    less: bool,
    fleeing: bool,
    fleeing_lunal: bool,
}
#[derive(Default)]
struct Time {
    now: i32,
    level: i32,
}
struct Borg {
    c: loc,
    traits: [i32; 265],
    avoidance: i16,
    status: Status,
    mon: Mon,
    goal: Goal,
    time: Time,
    lunal_mode: bool,
    munchkin_mode: bool,
    morgoth_position: bool,
}
impl Default for Borg {
    fn default() -> Self {
        Self {
            c: loc::default(),
            traits: [0; 265],
            avoidance: 0,
            status: Status::default(),
            mon: Mon::default(),
            goal: Goal::default(),
            time: Time::default(),
            lunal_mode: false,
            munchkin_mode: false,
            morgoth_position: false,
        }
    }
}
impl Borg {
    fn tr(&self, k: u32) -> i32 {
        self.traits[k as usize]
    }
}
#[derive(Default)]
struct Danger {
    wipe: bool,
    fear_region: [[u16; 19]; 7],
}
#[derive(Clone, Copy, Default)]
struct Kill {
    r_idx: u16,
    power: i16,
    awake: bool,
}
struct Kills {
    kills: Vec<Kill>,
}
struct Stairs {
    more: flow::StairTrack,
    less: flow::StairTrack,
}
struct Traits {
    simulate: bool,
}
struct Items {
    items: Vec<borg_item_h::borg_item>,
}
use borg_flow_kill::borg_race_name;
fn inven_wield() -> i32 {
    0
}
struct ItemValues {
    sv_scroll_rune_of_protection: i32,
}
struct Activations {
    glyph: i32,
}
pub struct BorgState {
    lunal_params: [i32; 19],
    lunal_trace: Vec<[i32; 5]>,
    glyph: glyph::GlyphState,
    glyph_flags: i32,
    glyph_scenario: i32,
    glyph_trace: Vec<[i32; 4]>,
    item_val: ItemValues,
    act: Activations,
    physical_mode: bool,
    traits: Traits,
    items: Items,
    damage_scores: [i32; 10],
    combat_trace: RefCell<Vec<[i32; 7]>>,
    borg: Borg,
    cave: CaveState,
    flow: flow::FlowState,
    danger: Danger,
    projection: projection::ProjectionState,
    kills: Kills,
    stairs: Stairs,
    keys: Vec<u32>,
    notes: String,
    dangers: Vec<u16>,
    danger_trace: Vec<[i32; 5]>,
}
#[derive(Clone)]
struct Race {
    ac: i32,
    flags: Vec<bool>,
    spell_flags: Vec<bool>,
}
struct Env {
    pack_size: u16,
    observed: u8,
    forced: bool,
    races: Vec<Race>,
    max_range: i16,
    pass_walls: u32,
    width: i32,
    height: i32,
}
thread_local! {static ENGINE:RefCell<Env>=const {RefCell::new(Env{pack_size:0,observed:0,forced:false,races:vec![],max_range:18,pass_walls:0,width:198,height:66})};}
mod engine {
    use super::*;
    pub struct Z {
        pub max_range: i16,
        pub r_max: i16,
        pub pack_size: u16,
    }
    pub fn z_info() -> Z {
        ENGINE.with(|e| Z {
            max_range: e.borrow().max_range,
            r_max: 10,
            pack_size: e.borrow().pack_size,
        })
    }
    pub fn square_feat(_: i32, _: i32) -> u8 {
        ENGINE.with(|e| e.borrow().observed)
    }
    pub fn player_opt(_: u32) -> bool {
        ENGINE.with(|e| e.borrow().forced)
    }
    pub fn square_in_bounds_fully(x: i32, y: i32) -> bool {
        ENGINE.with(|e| {
            let e = e.borrow();
            x > 0 && x < e.width - 1 && y > 0 && y < e.height - 1
        })
    }
    pub fn ddx(i: usize) -> i16 {
        [0, -1, 0, 1, -1, 0, 1, -1, 0, 1][i]
    }
    pub fn ddy(i: usize) -> i16 {
        [0, 1, 1, 1, 0, 0, 0, -1, -1, -1][i]
    }
    pub fn ddx_ddd(i: usize) -> i16 {
        [0, 0, 1, -1, 1, -1, 1, -1][i]
    }
    pub fn ddy_ddd(i: usize) -> i16 {
        [1, -1, 0, 0, 1, 1, -1, -1][i]
    }
    pub fn feat_is_shop(f: i32) -> bool {
        (7..=14).contains(&f)
    }
    include!(concat!(env!("OUT_DIR"), "/distance.rs"));
    pub fn race_flag_has(i: usize, flag: u32) -> bool {
        ENGINE.with(|e| e.borrow().races[i].flags[flag as usize])
    }
    pub fn race(i: usize) -> Race {
        ENGINE.with(|e| e.borrow().races[i].clone())
    }
    pub fn flag_has(flags: &[bool], flag: i32) -> bool {
        flags[flag as usize]
    }
}
mod borg_flow_kill {
    use super::*;
    pub fn borg_race_name(_: i32) -> &'static CStr {
        c"fixture"
    }
}

fn borg_keypress(st: &mut BorgState, k: u32) {
    st.keys.push(k)
}
fn borg_note(st: &mut BorgState, s: &str) {
    st.notes.push_str(s);
    st.notes.push('\n')
}
fn borg_note_bytes(st: &mut BorgState, s: &[u8]) {
    borg_note(st, std::str::from_utf8(s).unwrap())
}
fn borg_thrust_damage_one(st: &BorgState, k: i32) -> i32 {
    st.combat_trace.borrow_mut().push([
        0,
        k,
        if st.physical_mode {
            combat::physical_damage_one(st, k)
        } else {
            st.damage_scores[k as usize]
        },
        st.physical_mode as i32,
        0,
        0,
        0,
    ]);
    if st.physical_mode {
        combat::physical_damage_one(st, k)
    } else {
        st.damage_scores[k as usize]
    }
}
fn borg_danger_one_kill(
    st: &mut BorgState,
    y: i32,
    x: i32,
    n: i32,
    k: i32,
    a: bool,
    b: bool,
) -> i32 {
    st.combat_trace
        .borrow_mut()
        .push([1, y, x, n, k, a as i32, b as i32]);
    st.dangers[(y * 198 + x) as usize] as i32 + k * 17 + n * 3
}
fn borg_danger(st: &mut BorgState, y: i32, x: i32, n: i32, a: bool, b: bool) -> i32 {
    st.danger_trace.push([y, x, n, a as i32, b as i32]);
    st.dangers[(y * 198 + x) as usize] as i32
}
struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn u8(&mut self) -> u8 {
        let n = self.b[self.at];
        self.at += 1;
        n
    }
    fn i32(&mut self) -> i32 {
        let n = i32::from_le_bytes(self.b[self.at..self.at + 4].try_into().unwrap());
        self.at += 4;
        n
    }
    fn u16(&mut self) -> u16 {
        u16::from_le_bytes([self.u8(), self.u8()])
    }
}
fn put32(b: &mut Vec<u8>, n: i32) {
    b.extend(n.to_le_bytes())
}
fn load(packet: &[u8]) -> (BorgState, [i32; 32], Vec<(i32, i32)>) {
    let mut r = Reader { b: packet, at: 0 };
    let f = std::array::from_fn(|_| r.i32());
    let d: [i32; 11] = std::array::from_fn(|_| r.i32());
    let mut races = Vec::new();
    let rf = [
        race_flags::RF_ANIMAL,
        race_flags::RF_EVIL,
        race_flags::RF_UNDEAD,
        race_flags::RF_DEMON,
        race_flags::RF_ORC,
        race_flags::RF_TROLL,
        race_flags::RF_GIANT,
        race_flags::RF_DRAGON,
        race_flags::RF_IM_ACID,
        race_flags::RF_IM_FIRE,
        race_flags::RF_IM_COLD,
        race_flags::RF_IM_POIS,
        race_flags::RF_IM_ELEC,
        race_flags::RF_UNIQUE,
        race_flags::RF_MULTIPLY,
        race_flags::RF_QUESTOR,
    ];
    let sf = [
        spell_flags::RSF_S_KIN,
        spell_flags::RSF_S_HI_DEMON,
        spell_flags::RSF_S_MONSTER,
        spell_flags::RSF_S_MONSTERS,
        spell_flags::RSF_S_ANIMAL,
        spell_flags::RSF_S_SPIDER,
        spell_flags::RSF_S_HOUND,
        spell_flags::RSF_S_HYDRA,
        spell_flags::RSF_S_AINU,
        spell_flags::RSF_S_DEMON,
        spell_flags::RSF_S_UNDEAD,
        spell_flags::RSF_S_DRAGON,
        spell_flags::RSF_S_HI_UNDEAD,
        spell_flags::RSF_S_WRAITH,
        spell_flags::RSF_S_UNIQUE,
        spell_flags::RSF_S_HI_DRAGON,
    ];
    for k in 0..10 {
        let ac = r.i32();
        let flags = r.i32();
        let spells = r.i32();
        let mut race = Race {
            ac,
            flags: vec![false; 128],
            spell_flags: vec![false; 128],
        };
        for i in 0..16 {
            race.flags[rf[i] as usize] = flags & (1 << i) != 0;
            race.spell_flags[sf[i] as usize] = spells & (1 << i) != 0;
        }
        race.flags[RF_PASS_WALL as usize] = k != 0 && f[31] & (1 << k) != 0;
        races.push(race);
    }
    let mut st = BorgState {
        lunal_params: [0; 19],
        lunal_trace: vec![],
        glyph: glyph::GlyphState::default(),
        glyph_flags: f[29] & 15,
        glyph_scenario: f[28],
        glyph_trace: vec![],
        item_val: ItemValues {
            sv_scroll_rune_of_protection: 77,
        },
        act: Activations { glyph: 88 },
        physical_mode: false,
        traits: Traits { simulate: false },
        items: Items {
            items: vec![borg_item_h::borg_item {
                desc: b"fixture weapon\0".iter().map(|b| *b as i8).collect(),
                dd: d[0] as u8,
                ds: d[1] as u8,
                to_d: d[2] as i16,
                to_h: d[3] as i16,
                tval: 0,
                iqty: 0,
            }],
        },
        damage_scores: std::array::from_fn(|i| {
            if i == 0 {
                0
            } else if i == 1 {
                1000
            } else if (i as i32 + f[7]) % 5 == 0 {
                0
            } else {
                10 + f[7] * 3 + i as i32 * 7
            }
        }),
        combat_trace: RefCell::new(vec![]),
        borg: Borg::default(),
        cave: cave::allocated(),
        flow: flow::FlowState::default(),
        danger: Danger::default(),
        projection: projection::ProjectionState::default(),
        kills: Kills {
            kills: (0..10)
                .map(|i| Kill {
                    r_idx: i,
                    power: if i == 0 { 0 } else { 100 + i as i16 * 10 },
                    awake: f[29] & (1 << (8 + i)) != 0,
                })
                .collect(),
        },
        stairs: Stairs {
            more: flow::StairTrack {
                num: 0,
                size: 8,
                x: vec![0; 8],
                y: vec![0; 8],
            },
            less: flow::StairTrack {
                num: 1,
                size: 1,
                x: vec![f[24]],
                y: vec![f[25]],
            },
        },
        keys: vec![],
        notes: String::new(),
        dangers: vec![0; 13068],
        danger_trace: vec![],
    };
    st.borg.c = loc { x: f[0], y: f[1] };
    for (k, i) in [
        (BI_CURHP, 2),
        (BI_MAXHP, 3),
        (BI_MAXCLEVEL, 4),
        (BI_CLEVEL, 5),
        (BI_FOOD, 6),
        (BI_DISP, 7),
        (BI_DISM, 8),
        (BI_IFIRE, 9),
        (BI_ISAFRAID, 10),
    ] {
        st.borg.traits[k as usize] = f[i];
    }
    st.borg.traits[BI_CRSFEAR as usize] = f[29] & 1;
    st.borg.traits[BI_CDEPTH as usize] = if f[28] == 6 { 0 } else { 50 };
    for (k, v) in [
        (BI_TODAM, d[4]),
        (BI_BLOWS, d[5]),
        (BI_THN, d[6]),
        (BI_TOHIT, d[7]),
        (BI_CLASS, d[8]),
        (BI_CURSP, d[9]),
    ] {
        st.borg.traits[k as usize] = v;
    }
    for (i, k) in [
        BI_WS_ANIMAL,
        BI_WS_EVIL,
        BI_WS_UNDEAD,
        BI_WS_DEMON,
        BI_WS_ORC,
        BI_WS_TROLL,
        BI_WS_GIANT,
        BI_WS_DRAGON,
        BI_WB_ACID,
        BI_WB_FIRE,
        BI_WB_COLD,
        BI_WB_POIS,
        BI_WB_ELEC,
        BI_WK_UNDEAD,
        BI_WK_DEMON,
        BI_WK_DRAGON,
    ]
    .iter()
    .enumerate()
    {
        st.borg.traits[*k as usize] = ((d[10] >> i) & 1) as i32;
    }
    st.borg.avoidance = f[11] as i16;
    let mask = f[12];
    st.borg.status = Status {
        desperate: mask & 1 != 0,
        vault: mask & 2 != 0,
        digging: mask & 4 != 0,
    };
    st.borg.mon = Mon {
        scary: mask & 8 != 0,
        unique: f[13] as i16,
    };
    st.borg.goal = Goal {
        shop: f[16] as i16,
        ignoring: mask & 16 != 0,
        type_0: 0,
        g: loc::default(),
        less: false,
        fleeing: false,
        fleeing_lunal: false,
    };
    st.borg.time = Time {
        now: f[14],
        level: f[15],
    };
    st.borg.lunal_mode = mask & 32 != 0;
    st.borg.munchkin_mode = mask & 64 != 0;
    st.borg.morgoth_position = mask & 128 != 0;
    st.danger.wipe = mask & 256 != 0;
    st.danger.fear_region = [[f[17] as u16; 19]; 7];
    ENGINE.with(|e| {
        *e.borrow_mut() = Env {
            pack_size: 0,
            observed: 0,
            forced: false,
            races,
            max_range: f[18] as i16,
            pass_walls: f[31] as u32,
            width: if f[28] == 6 && f[30] != 2 { 66 } else { 198 },
            height: if f[28] == 6 && f[30] != 2 { 22 } else { 66 },
        }
    });
    st.flow.data_flow = vec![[0; 198]; 66];
    st.flow.data_cost = st.flow.data_flow.clone();
    st.flow.data_hard = st.flow.data_flow.clone();
    st.flow.data_know = st.flow.data_flow.clone();
    st.flow.data_icky = st.flow.data_flow.clone();
    st.glyph.track = flow::StairTrack {
        num: 0,
        size: 200,
        x: vec![0; 200],
        y: vec![0; 200],
    };
    for y in 0..66 {
        for x in 0..198 {
            *st.cave.grid_mut(x, y) = BorgGrid {
                feat: r.u8(),
                info: r.u16(),
                trap: r.u8() != 0,
                glyph: r.u8() != 0,
                web: r.u8() != 0,
                store: r.u8(),
                take: r.u8(),
                kill: r.u8(),
                xtra: r.u8(),
            };
            st.flow.data_flow[y as usize][x as usize] = r.u8();
            st.flow.data_cost[y as usize][x as usize] = r.u8();
            st.flow.data_hard[y as usize][x as usize] = r.u8();
            st.flow.data_know[y as usize][x as usize] = r.u8();
            st.flow.data_icky[y as usize][x as usize] = r.u8();
            st.dangers[(y * 198 + x) as usize] = r.u16();
        }
    }
    for i in 0..1536 {
        st.flow.flow_x[i] = r.u8();
        st.flow.flow_y[i] = r.u8();
    }
    let n = r.i32();
    let origins = (0..n).map(|_| (r.u8() as i32, r.u8() as i32)).collect();
    if r.at < packet.len() {
        st.lunal_params = std::array::from_fn(|_| r.i32());
    }
    assert_eq!(r.at, packet.len());
    (st, f, origins)
}
fn bindings() -> Vec<u32> {
    const RF_BINDINGS: [u32; 16] = [
        race_flags::RF_ANIMAL,
        race_flags::RF_EVIL,
        race_flags::RF_UNDEAD,
        race_flags::RF_DEMON,
        race_flags::RF_ORC,
        race_flags::RF_TROLL,
        race_flags::RF_GIANT,
        race_flags::RF_DRAGON,
        race_flags::RF_IM_ACID,
        race_flags::RF_IM_FIRE,
        race_flags::RF_IM_COLD,
        race_flags::RF_IM_POIS,
        race_flags::RF_IM_ELEC,
        race_flags::RF_UNIQUE,
        race_flags::RF_MULTIPLY,
        race_flags::RF_QUESTOR,
    ];
    const SF_BINDINGS: [u32; 16] = [
        spell_flags::RSF_S_KIN,
        spell_flags::RSF_S_HI_DEMON,
        spell_flags::RSF_S_MONSTER,
        spell_flags::RSF_S_MONSTERS,
        spell_flags::RSF_S_ANIMAL,
        spell_flags::RSF_S_SPIDER,
        spell_flags::RSF_S_HOUND,
        spell_flags::RSF_S_HYDRA,
        spell_flags::RSF_S_AINU,
        spell_flags::RSF_S_DEMON,
        spell_flags::RSF_S_UNDEAD,
        spell_flags::RSF_S_DRAGON,
        spell_flags::RSF_S_HI_UNDEAD,
        spell_flags::RSF_S_WRAITH,
        spell_flags::RSF_S_UNIQUE,
        spell_flags::RSF_S_HI_DRAGON,
    ];

    RF_BINDINGS
        .into_iter()
        .chain([RF_PASS_WALL])
        .chain(SF_BINDINGS)
        .collect()
}

fn snapshot(packet: &[u8]) -> Vec<u8> {
    let (mut st, f, origins) = load(packet);
    let mut out = vec![];
    for n in bindings() {
        put32(&mut out, n as i32);
    }
    let (px, py, tx, ty) = (f[0], f[1], f[19], f[20]);
    put32(&mut out, projection::borg_los(&st, 0, 0, py, px) as i32);
    put32(&mut out, projection::borg_los(&st, py, px, ty, tx) as i32);
    put32(
        &mut out,
        projection::borg_projectable(&mut st, py, px, ty, tx) as i32,
    );
    put32(
        &mut out,
        projection::borg_offset_projectable(&st, py, px, ty, tx) as i32,
    );
    put32(
        &mut out,
        projection::borg_projectable_pure(&st, py, px, ty, tx) as i32,
    );
    put32(
        &mut out,
        projection::borg_projectable_dark(&st, py, px, ty, tx) as i32,
    );
    let (mut x, mut y) = (px, py);
    for _ in 0..12 {
        projection::borg_inc_motion(&mut y, &mut x, py, px, ty, tx);
        put32(&mut out, x);
        put32(&mut out, y);
    }
    put32(
        &mut out,
        projection::borg_target(&mut st, loc { x: tx, y: ty }, f[30] != 0) as i32,
    );
    put32(
        &mut out,
        if (px, py) != (tx, ty) {
            projection::borg_target_unknown_wall(&mut st, ty, tx) as i32
        } else {
            -1
        },
    );
    put32(&mut out, directions::borg_extract_dir(py, px, ty, tx));
    put32(&mut out, directions::borg_goto_dir(&st, py, px, ty, tx));
    st.flow.temp_n = 0;
    if st.cave.grid(px, py).kill != 0 {
        st.flow.temp_push(py, px);
    }
    for dy in -1..=1 {
        for dx in -1..=1 {
            if (dx != 0 || dy != 0) && st.cave.grid(px + dx, py + dy).kill != 0 {
                st.flow.temp_push(py + dy, px + dx);
            }
        }
    }
    if st.cave.grid(px + 2, py).kill != 0 {
        st.flow.temp_push(py, px + 2);
    }
    for k in 1..10 {
        put32(&mut out, combat::physical_damage_one(&st, k));
    }
    st.physical_mode = false;
    st.traits.simulate = true;
    put32(&mut out, combat::borg_attack_aux_thrust(&mut st));
    put32(&mut out, st.borg.goal.g.x);
    put32(&mut out, st.borg.goal.g.y);
    st.traits.simulate = false;
    put32(&mut out, combat::borg_attack_aux_thrust(&mut st));
    put32(&mut out, st.borg.goal.g.x);
    put32(&mut out, st.borg.goal.g.y);
    st.physical_mode = true;
    st.traits.simulate = true;
    put32(&mut out, combat::borg_attack_aux_thrust(&mut st));
    put32(&mut out, st.borg.goal.g.x);
    put32(&mut out, st.borg.goal.g.y);
    st.traits.simulate = false;
    put32(&mut out, combat::borg_attack_aux_thrust(&mut st));
    put32(&mut out, st.borg.goal.g.x);
    put32(&mut out, st.borg.goal.g.y);
    st.flow.head = f[27];
    st.flow.tail = (f[27] + 1) % 1536;
    flow::borg_flow_clear(&mut st);
    for n in [st.flow.head, st.flow.tail, st.danger.wipe as i32] {
        put32(&mut out, n);
    }
    for y in 0..66 {
        for x in 0..198 {
            for m in [
                &st.flow.data_cost,
                &st.flow.data_hard,
                &st.flow.data_know,
                &st.flow.data_icky,
            ] {
                out.push(m[y][x]);
            }
        }
    }
    st.flow.head = f[27];
    st.flow.tail = f[27];
    for (x, y) in origins {
        flow::borg_flow_enqueue_grid(&mut st, y, x);
    }
    for n in [st.flow.head, st.flow.tail] {
        put32(&mut out, n);
    }
    for y in 0..66 {
        for x in 0..198 {
            for m in [
                &st.flow.data_cost,
                &st.flow.data_hard,
                &st.flow.data_know,
                &st.flow.data_icky,
            ] {
                out.push(m[y][x]);
            }
        }
    }
    for i in 0..1536 {
        out.extend([st.flow.flow_x[i], st.flow.flow_y[i]]);
    }
    flow::borg_flow_spread(
        &mut st,
        f[21],
        f[22] & 1 != 0,
        f[22] & 2 != 0,
        f[22] & 4 != 0,
        f[23],
        f[22] & 8 != 0,
    );
    put32(
        &mut out,
        flow::flow_commit(&mut st, Some(b"fixture"), f[26]) as i32,
    );
    serialize(st, out)
}
fn serialize(st: BorgState, mut out: Vec<u8>) -> Vec<u8> {
    for n in [
        st.borg.lunal_mode as i32,
        st.borg.goal.fleeing as i32,
        st.borg.goal.fleeing_lunal as i32,
        st.borg.goal.less as i32,
    ] {
        put32(&mut out, n);
    }
    for t in [&st.stairs.less, &st.stairs.more] {
        put32(&mut out, t.num as i32);
        put32(&mut out, t.size as i32);
        for i in 0..t.size as usize {
            put32(&mut out, t.x[i]);
            put32(&mut out, t.y[i]);
        }
    }
    put32(&mut out, st.lunal_trace.len() as i32);
    for row in st.lunal_trace {
        for n in row {
            put32(&mut out, n);
        }
    }

    for n in [
        st.glyph.needs_new_sea as i32,
        st.glyph.glyph_x as i32,
        st.glyph.glyph_y as i32,
        st.glyph.center_x as i32,
        st.glyph.center_y as i32,
        st.glyph.track.num as i32,
        st.glyph.track.size as i32,
    ] {
        put32(&mut out, n);
    }
    for i in 0..200 {
        put32(&mut out, st.glyph.track.x[i]);
        put32(&mut out, st.glyph.track.y[i]);
    }
    put32(&mut out, st.glyph_trace.len() as i32);
    for trace in st.glyph_trace {
        for n in trace {
            put32(&mut out, n);
        }
    }
    for n in [
        st.flow.head,
        st.flow.tail,
        st.borg.goal.type_0 as i32,
        st.danger.wipe as i32,
        st.projection.target_loc.x,
        st.projection.target_loc.y,
    ] {
        put32(&mut out, n);
    }
    for y in 0..66 {
        for x in 0..198 {
            let g = st.cave.grid(x, y);
            out.push(g.feat);
            out.extend(g.info.to_le_bytes());
            out.extend([
                g.trap as u8,
                g.glyph as u8,
                g.web as u8,
                g.store,
                g.take,
                g.kill,
                g.xtra,
            ]);
            for m in [
                &st.flow.data_flow,
                &st.flow.data_cost,
                &st.flow.data_hard,
                &st.flow.data_know,
                &st.flow.data_icky,
            ] {
                out.push(m[y as usize][x as usize]);
            }
        }
    }
    for i in 0..1536 {
        out.extend([st.flow.flow_x[i], st.flow.flow_y[i]]);
    }
    let combat_trace = st.combat_trace.into_inner();
    put32(&mut out, combat_trace.len() as i32);
    for t in combat_trace {
        for n in t {
            put32(&mut out, n);
        }
    }
    put32(&mut out, st.danger_trace.len() as i32);
    for t in st.danger_trace {
        for n in t {
            put32(&mut out, n);
        }
    }
    put32(&mut out, st.keys.len() as i32);
    for k in st.keys {
        put32(&mut out, k as i32);
    }
    put32(&mut out, st.notes.len() as i32);
    out.extend(st.notes.as_bytes());
    out
}
fn glyph_callback(st: &mut BorgState, kind: i32, a: i32, b: i32) -> bool {
    let result = st.glyph_flags & (1 << (kind - 1)) != 0;
    st.glyph_trace.push([kind, a, b, result as i32]);
    result
}
mod glyph_environment {
    use super::*;
    pub fn borg_flow_reverse(
        st: &mut BorgState,
        depth: i32,
        optimize: bool,
        avoid: bool,
        tunneling: bool,
        stair_idx: i32,
        sneak: bool,
    ) {
        directions::borg_flow_reverse(st, depth, optimize, avoid, tunneling, stair_idx, sneak);
        // Candidate-cost adapter: #345 otherwise leaves only the origin reachable.
        // Keep real reverse flow in all other cases; do not alter its algorithm.
        if (19..=22).contains(&st.glyph_scenario) {
            st.flow.data_cost[33][99] = 1;
            st.flow.data_cost[33][101] = if st.glyph_scenario >= 21 { 2 } else { 1 };
        }
        if (23..=25).contains(&st.glyph_scenario) {
            st.flow.data_cost[33][101] = (74 + st.glyph_scenario - 23) as u8;
        }
    }
}
mod borg_io {
    use super::*;
    pub fn borg_note(st: &mut BorgState, s: &str) {
        super::borg_note(st, s);
    }
}
mod borg_magic {
    use super::*;
    pub fn borg_spell_fail(st: &mut BorgState, spell: u32, fail: i32) -> bool {
        glyph_callback(st, 1, spell as i32, fail)
    }
}
mod borg_item_use {
    use super::*;
    pub fn borg_read_scroll(st: &mut BorgState, sval: i32) -> bool {
        glyph_callback(st, 2, sval, 0)
    }
    pub fn borg_activate_item(st: &mut BorgState, act: i32) -> bool {
        glyph_callback(st, 3, act, 0)
    }
}
fn glyph_result(packet: &[u8]) -> Vec<u8> {
    let (mut st, f, _) = load(packet);
    let (px, py) = (f[0], f[1]);
    let scenario = f[28];
    st.glyph.needs_new_sea = matches!(scenario, 0 | 2 | 5 | 6 | 7 | 11 | 12 | 13 | 15);
    if scenario <= 1
        || scenario == 6
        || scenario == 7
        || scenario == 13
        || (16..=18).contains(&scenario)
    {
        st.glyph.glyph_x = px as u8;
        st.glyph.glyph_y = py as u8;
    }
    if matches!(scenario, 1 | 3 | 4 | 9 | 10 | 14) {
        st.glyph.center_x = (px
            + match scenario {
                9 | 10 => -1,
                14 => 10,
                _ => 0,
            }) as u8;
        st.glyph.center_y = py as u8;
        st.glyph.track.x[0] = i32::from(st.glyph.center_x);
        st.glyph.track.y[0] = py;
        st.glyph.track.num = 1;
    }
    if scenario == 4 || scenario == 14 {
        for i in 0..24 {
            let (dx, dy) = flow::borg_ddxy_ddd(i);
            st.glyph.track.x[i + 1] = i32::from(st.glyph.center_x) + dx;
            st.glyph.track.y[i + 1] = py + dy;
        }
        st.glyph.track.num = 25;
    }
    if scenario == 6 || scenario == 7 {
        st.glyph.track.num = if scenario == 6 { 200 } else { 199 };
        for i in 0..st.glyph.track.num as usize {
            st.glyph.track.x[i] = 1 + i as i32 % 196;
            st.glyph.track.y[i] = 1 + i as i32 / 196;
        }
    }
    if scenario == 8 {
        st.glyph.center_x = 10;
        st.glyph.center_y = 10;
        st.glyph.needs_new_sea = false;
    }
    if (16..=18).contains(&scenario) {
        st.glyph.center_x = (px - (49 + scenario - 16)) as u8;
        st.glyph.center_y = py as u8;
        st.glyph.track.x[0] = i32::from(st.glyph.center_x);
        st.glyph.track.y[0] = py;
        st.glyph.track.num = 1;
    }
    if scenario == 20 || scenario == 22 || (23..=25).contains(&scenario) {
        st.glyph.center_x = px as u8;
        st.glyph.center_y = py as u8;
        st.glyph.track.x[0] = px;
        st.glyph.track.y[0] = py;
        st.glyph.track.num = 1;
    }
    let mut out = vec![];
    for n in bindings() {
        put32(&mut out, n as i32);
    }
    put32(&mut out, glyph::borg_flow_glyph(&mut st) as i32);
    serialize(st, out)
}
const GLYPH_SCENARIOS: usize = 26;
fn glyph_packet(rng: &mut Rng, i: usize) -> Vec<u8> {
    glyph_packet_for(rng, i, i % GLYPH_SCENARIOS)
}
fn glyph_packet_for(rng: &mut Rng, i: usize, scenario: usize) -> Vec<u8> {
    let mut b = packet(rng, i);
    let mut f: [i32; 32] =
        std::array::from_fn(|k| i32::from_le_bytes(b[k * 4..k * 4 + 4].try_into().unwrap()));
    f[0] = 100;
    f[1] = 33;
    f[2] = f[3];
    f[4] = 50;
    f[5] = 50;
    f[10] = 0;
    f[11] = 1000;
    f[12] = 257;
    f[16] = -1;
    f[19] = 101;
    f[20] = 33;
    f[24] = 100;
    f[25] = 33;
    f[28] = scenario as i32;
    f[30] = 2;
    f[29] = match scenario {
        0 => 9,
        1 => 10,
        2 => 8,
        3 => 8,
        4 => 8,
        6 => 9,
        7 => 12,
        16..=18 => 9,
        19..=25 => 8,
        8 | 9 | 10 | 11 | 14 | 15 => 8,
        _ => 0,
    };
    for (k, n) in f.into_iter().enumerate() {
        b[k * 4..k * 4 + 4].copy_from_slice(&n.to_le_bytes());
    }
    let start = 32 * 4 + 11 * 4 + 10 * 12;
    for y in 0i32..66 {
        for x in 0i32..198 {
            let at = start + (y as usize * 198 + x as usize) * 17;
            let border = x == 0 || x == 197 || y == 0 || y == 65;
            let room = (85..=115).contains(&x) && (20..=46).contains(&y);
            b[at] = if border {
                22
            } else if room && scenario != 5 {
                1
            } else {
                21
            };
            b[at + 3] = 0;
            b[at + 4] =
                ((matches!(scenario, 1 | 3 | 4 | 10 | 11 | 19..=25)) && x == 100 && y == 33) as u8;
            if (scenario == 4 || scenario == 12) && ((x - 100).abs() <= 2 && (y - 33).abs() <= 2) {
                b[at + 4] = 1;
            }
            if (scenario == 9 || scenario == 10) && x == 99 && y == 33 {
                b[at + 4] = 1;
            }
            if scenario == 14 {
                if x == 105 {
                    b[at] = 21;
                }
                if (x - 110).abs() <= 2 && (y - 33).abs() <= 2 && (x != 110 || y != 33) {
                    b[at + 4] = 1;
                }
            }
            if scenario == 15 && x == 50 && y == 15 {
                b[at + 4] = 1;
            }
            b[at + 8] = 0;
            for off in [10, 11, 13, 14, 15, 16] {
                b[at + off] = 0;
            }
            b[at + 12] = 255;
        }
    }
    b
}

impl BorgState {
    fn cfg(&self, _: u32) -> i32 {
        self.lunal_params[9]
    }
}
mod borg_flow_stairs {
    pub use crate::flow::StairTrack;
}
fn lcall(st: &mut BorgState, kind: i32, a: i32, b: i32, c: i32, result: i32) -> i32 {
    st.lunal_trace.push([kind, a, b, c, result]);
    result
}
fn lbool(st: &mut BorgState, kind: i32, a: i32, b: i32, c: i32) -> bool {
    let value = (st.lunal_params[18] & (1 << kind) != 0) as i32;
    lcall(st, kind, a, b, c, value) != 0
}
fn borg_notice(st: &mut BorgState, swap: bool) {
    lcall(st, 0, swap as i32, 0, 0, 0);
}
mod borg_prepared {
    use super::*;
    pub fn borg_prepared_buf(st: &mut BorgState, d: i32) -> Option<()> {
        let ready = st.lunal_params[10];
        if lcall(st, 1, d, 0, 0, ready) != 0 {
            None
        } else {
            Some(())
        }
    }
}
fn borg_maintain_light(st: &mut BorgState) -> lunal::BorgNeed {
    let n = st.lunal_params[11];
    match lcall(st, 2, 0, 0, 0, n) {
        0 => lunal::BorgNeed::NoNeed,
        1 => lunal::BorgNeed::MetNeed,
        2 => lunal::BorgNeed::UnmetNeed,
        _ => panic!("need"),
    }
}
fn borg_check_rest(st: &mut BorgState, y: i32, x: i32) -> bool {
    let safe = st.lunal_params[12];
    lcall(st, 3, y, x, 0, safe) != 0
}
fn borg_recover(st: &mut BorgState) -> bool {
    lbool(st, 4, 0, 0, 0)
}
fn borg_use_things(st: &mut BorgState) -> bool {
    lbool(st, 5, 0, 0, 0)
}
fn borg_drop_junk(st: &mut BorgState) -> bool {
    lbool(st, 6, 0, 0, 0)
}
fn borg_flow_old(st: &mut BorgState, why: i32) -> bool {
    let bit = if why == 2 { 7 } else { 12 };
    let result = (st.lunal_params[18] & (1 << bit) != 0) as i32;
    lcall(st, 7, why, 0, 0, result) != 0
}
fn borg_flow_take_lunal(st: &mut BorgState, view: bool, d: i32) -> bool {
    lbool(st, 8, view as i32, d, 0)
}
fn borg_flow_stair_less(st: &mut BorgState, why: i32, sneak: bool) -> bool {
    lbool(st, 9, why, sneak as i32, 0)
}
fn borg_flow_stair_more(st: &mut BorgState, why: i32, sneak: bool, brave: bool) -> bool {
    lbool(st, 10, why, sneak as i32, brave as i32)
}
fn borg_flow_stair_both(st: &mut BorgState, why: i32, sneak: bool) -> bool {
    lbool(st, 11, why, sneak as i32, 0)
}
fn key(c: u8) -> u32 {
    c as u32
}
fn lunal_result(packet: &[u8]) -> Vec<u8> {
    let (mut st, _, _) = load(packet);
    let p = st.lunal_params;
    let c = st.borg.c;
    st.borg.lunal_mode = true;
    st.borg.goal.fleeing = true;
    st.borg.goal.fleeing_lunal = true;
    for (k, v) in [
        (BI_CDEPTH, p[0]),
        (BI_MAXDEPTH, p[1]),
        (BI_ISWEAK, p[2]),
        (BI_ISHUNGRY, p[3]),
        (BI_LIGHT, p[4]),
    ] {
        st.borg.traits[k as usize] = v;
    }
    st.items.items = (0..32)
        .map(|_| borg_item_h::borg_item {
            desc: vec![0],
            dd: 1,
            ds: 1,
            to_d: 0,
            to_h: 0,
            tval: 0,
            iqty: 0,
        })
        .collect();
    st.items.items[25].tval = p[5] as u8;
    st.items.items[18].iqty = p[6] as u8;
    st.cave.grid_mut(c.x, c.y).feat = p[8] as u8;
    st.stairs.less = flow::StairTrack {
        num: p[13] as i16,
        size: 8,
        x: (0..8).map(|i| c.x + p[15] + i).collect(),
        y: vec![c.y; 8],
    };
    st.stairs.more = flow::StairTrack {
        num: p[14] as i16,
        size: 8,
        x: (0..8).map(|i| c.x + p[16] + i).collect(),
        y: vec![c.y; 8],
    };
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.pack_size = 20;
        e.observed = p[7] as u8;
        e.forced = p[17] != 0;
    });
    let mut out = vec![];
    for n in bindings() {
        put32(&mut out, n as i32);
    }
    put32(&mut out, lunal::borg_think_dungeon_lunal(&mut st) as i32);
    serialize(st, out)
}
fn lunal_packet(r: &mut Rng, i: usize) -> Vec<u8> {
    let mut b = glyph_packet_for(r, i, i % 16);
    let mut p = [20, 99, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 2, 3, 0, 0];
    let scenario = i % 40;
    match scenario {
        0 => p[0] = 0,
        1 => p[2] = 1,
        2 => {
            p[0] = 1;
            p[13] = 0;
            p[14] = 0;
            p[7] = 6;
            p[11] = 1;
        }
        3 => {
            p[0] = 1;
            p[14] = 0;
        }
        4 => {
            p[13] = 0;
            p[14] = 0;
        }
        5 => {
            p[9] = 1;
            p[1] = p[0] + 15;
        }
        6 => {
            p[9] = 1;
            p[10] = 0;
        }
        7 => {
            p[9] = 1;
            p[0] = 50;
        }
        8 => {
            p[9] = 1;
            p[11] = 1;
        }
        9 => {
            p[11] = 2;
            p[4] = 0;
            p[5] = 0;
        }
        10 => p[18] = 1 << 4,
        11 => p[18] = 1 << 5,
        12 => {
            p[12] = 0;
            p[3] = 1;
            p[18] = 1 << 5;
        }
        13 => p[18] = 1 << 6,
        14 => p[18] = 1 << 7,
        15 => p[18] = 1 << 8,
        16 => {
            p[6] = 1;
            p[18] = 1 << 7;
        }
        17 => {
            p[6] = 1;
            p[18] = 1 << 9;
        }
        18 => {
            p[6] = 1;
            p[7] = 5;
            p[8] = 5;
        }
        19 => {
            p[6] = 1;
            p[7] = 5;
            p[8] = 5;
            p[17] = 1;
        }
        20 => {
            p[14] = 2;
            p[18] = 1 << 7;
        }
        21 => p[18] = 1 << 10,
        22 => {
            p[7] = 6;
            p[8] = 6;
        }
        23 => {
            p[14] = 0;
            p[18] = 1 << 9;
        }
        24 => {
            p[0] = 40;
            p[12] = 0;
            p[15] = 20;
            p[16] = 20;
            p[18] = 1 << 11;
        }
        25 => {
            p[0] = 99;
            p[12] = 0;
            p[15] = 20;
            p[16] = 20;
        }
        26 => {
            p[7] = 5;
            p[15] = 0;
        }
        27 => {
            p[7] = 6;
            p[16] = 0;
        }
        28 => {
            p[7] = 5;
            p[13] = 8;
        }
        29 => {
            p[7] = 6;
            p[14] = 8;
        }
        30 => {
            p[6] = 1;
            p[15] = 0;
            p[16] = 20;
        }
        31 => {
            p[0] = 35;
            p[9] = 1;
            p[18] = 1 << 11;
        }
        32 => {
            p[6] = 1;
            p[18] = 1 << 12;
        }
        33 => {
            p[13] = 0;
            p[18] = 1 << 12;
        }
        34 => {
            p[14] = 0;
            p[18] = 1 << 12;
        }
        35 => {
            p[0] = 40;
            p[12] = 0;
            p[18] = 1 << 12;
        }
        36 => {
            p[0] = 1;
            p[6] = 1;
        }
        37 => {
            p[12] = 0;
            p[7] = 6;
            p[8] = 6;
        }
        38 => {
            p[12] = 0;
        }
        _ => {
            p[0] = 1;
            p[6] = 1;
            p[18] = 1 << 12;
        }
    }
    for n in p {
        put32(&mut b, n);
    }
    b
}
struct Rng(u64);
impl Rng {
    fn n(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
}
fn packet(r: &mut Rng, i: usize) -> Vec<u8> {
    let mut f = [0i32; 32];
    f[0] = 3 + r.n(192) as i32;
    f[1] = 3 + r.n(60) as i32;
    f[3] = 30 + r.n(1800) as i32;
    f[2] = [0, f[3] / 3, f[3] / 2, f[3]][i % 4];
    f[4] = [4, 15, 35, 50][i % 4];
    f[5] = [1, 4, 5, 9, 10, 14, 15, 19, 20, 35, 50][(i / 8) % 11];
    f[4] = f[4].max(f[5]);
    f[6] = r.n(5) as i32;
    f[7] = r.n(60) as i32;
    f[8] = r.n(60) as i32;
    f[9] = r.n(2) as i32;
    f[10] = r.n(2) as i32;
    f[11] = 20 + r.n(3000) as i32;
    f[12] = r.n(512) as i32;
    f[13] = r.n(3) as i32;
    f[14] = r.n(10001) as i32;
    f[15] = 0;
    f[16] = r.n(4) as i32 - 1;
    f[17] = [0, 1, 15, 150, 1000][i % 5];
    f[18] = 30;
    f[19] = 3 + r.n(192) as i32;
    f[20] = 3 + r.n(60) as i32;
    if i.is_multiple_of(8) {
        f[19] = f[0] + 1;
        f[20] = f[1];
    }
    f[21] = [1, 2, 10, 50, 250][i % 5];
    f[22] = r.n(16) as i32;
    f[23] = if i.is_multiple_of(3) { 0 } else { -1 };
    f[24] = 1 + r.n(196) as i32;
    f[25] = 1 + r.n(64) as i32;
    f[26] = r.n(11) as i32;
    f[27] = [0, 1, 1534, 1535][i % 4];
    f[28] = (i % 8) as i32;
    f[30] = r.n(2) as i32;
    f[31] = r.n(1024) as i32;
    f[29] = ((r.n(1024) as i32) << 8) | (r.n(4) == 0) as i32;
    if i.is_multiple_of(8) {
        f[12] = 257;
        f[10] = 0;
        f[6] = 3;
        f[16] = -1;
        f[21] = 250;
        f[22] = 0;
        f[29] = 0x3fe00;
    }
    let width = if i % 8 == 6 { 66 } else { 198 };
    let height = if i % 8 == 6 { 22 } else { 66 };
    if i % 8 == 6 {
        for k in [0, 19] {
            f[k] = 3 + (f[k] - 3) % 60;
        }
        for k in [1, 20] {
            f[k] = 3 + (f[k] - 3) % 16;
        }
        f[24] = 1 + (f[24] - 1) % 64;
        f[25] = 1 + (f[25] - 1) % 20;
    }
    let mut b = vec![];
    for n in f {
        put32(&mut b, n);
    }
    let damage = [
        1 + r.n(8) as i32,
        1 + r.n(16) as i32,
        r.n(41) as i32 - 10,
        r.n(41) as i32 - 10,
        r.n(61) as i32 - 10,
        1 + r.n(6) as i32,
        r.n(201) as i32,
        r.n(61) as i32 - 20,
        (i % 9) as i32,
        r.n(25) as i32,
        if i % 20 < 16 {
            1 << (i % 16)
        } else if i % 20 == 16 {
            0
        } else {
            r.n(65536) as i32
        },
    ];
    for n in damage {
        put32(&mut b, n);
    }
    for k in 0..10 {
        put32(&mut b, if k == 0 { 0 } else { r.n(251) as i32 });
        let flags = if k == 0 {
            0
        } else {
            let mut mask = 0;
            for bit in 0..16 {
                if r.n(8) == 0 {
                    mask |= 1 << bit;
                }
            }
            mask
        };
        put32(&mut b, flags);
        put32(
            &mut b,
            if k == 0 {
                0
            } else {
                match i % 18 {
                    0..=15 => 1 << (i % 16),
                    16 => 0,
                    _ => r.n(65536) as i32,
                }
            },
        );
    }
    let features = [
        0u8, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 14, 15, 16, 21, 22, 23, 24,
    ];
    for y in 0..66 {
        for x in 0..198 {
            let border = x == 0 || x >= width - 1 || y == 0 || y >= height - 1;
            let mode = i % 8;
            let feat = if border {
                22
            } else if mode == 0 || mode == 4 || mode == 5 {
                1
            } else if mode == 2 {
                if x == f[0] {
                    1
                } else {
                    21
                }
            } else if mode == 3 {
                if y == f[1] {
                    1
                } else {
                    21
                }
            } else {
                features[r.n(features.len() as u64) as usize]
            };
            b.push(feat);
            b.extend((r.n(65536) as u16).to_le_bytes());
            let trap = r.n(20) == 0;
            let glyph = r.n(30) == 0;
            let mut kill = if mode == 0 {
                0
            } else if mode == 4 || mode == 5 {
                if r.n(30) == 0 {
                    1 + r.n(7) as u8
                } else {
                    0
                }
            } else {
                r.n(8) as u8
            };
            if i.is_multiple_of(4) {
                if x == f[0] && y == f[1] {
                    kill = 1;
                } else if (x - f[0]).abs() <= 1 && (y - f[1]).abs() <= 1 {
                    let at = (y - f[1] + 1) * 3 + (x - f[0] + 1);
                    kill = if at < 4 { at as u8 + 2 } else { at as u8 + 1 };
                }
            }
            b.extend([
                if mode == 0 { 0 } else { trap as u8 },
                glyph as u8,
                (r.n(20) == 0) as u8,
                r.n(9) as u8,
                r.n(8) as u8,
                kill,
                r.n(255) as u8,
            ]);
            b.extend([
                r.n(255) as u8,
                r.n(255) as u8,
                255,
                r.n(2) as u8,
                r.n(2) as u8,
            ]);
            b.extend((if mode == 0 { 0 } else { r.n(3000) as u16 }).to_le_bytes());
        }
    }
    for _ in 0..1536 {
        b.extend([r.n(198) as u8, r.n(66) as u8]);
    }
    let n = if i.is_multiple_of(8) {
        1600
    } else {
        1 + r.n(12) as i32
    };
    put32(&mut b, n);
    for j in 0..n {
        let (x, y) = if i.is_multiple_of(8) {
            (1 + j % 196, 1 + j / 196)
        } else {
            (
                1 + r.n((width - 2) as u64) as i32,
                1 + r.n((height - 2) as u64) as i32,
            )
        };
        b.extend([x as u8, y as u8]);
    }
    b
}
unsafe extern "C" {
    fn lunal_snapshot(input: *const u8, len: usize, out: *mut u8, capacity: usize) -> usize;
    fn glyph_snapshot(input: *const u8, len: usize, out: *mut u8, capacity: usize) -> usize;
    fn spatial_snapshot(input: *const u8, len: usize, out: *mut u8, capacity: usize) -> usize;
}
fn glyph_state_offset(bytes: &[u8]) -> usize {
    let read = |at: usize| i32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let mut at = bindings().len() * 4 + 4 + 4 * 4;
    for _ in 0..2 {
        let size = read(at + 4) as usize;
        at += 8 + size * 8;
    }
    let callbacks = read(at) as usize;
    at += 4 + callbacks * 5 * 4;
    at
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let count = args
        .get(1)
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(2000);
    let control = args.iter().any(|s| s == "--fault-control");
    if env!("SPATIAL_OVERRIDE") == "1" && !control {
        eprintln!("source override requires --fault-control");
        std::process::exit(2);
    }
    let seed = args
        .get(2)
        .filter(|s| !s.starts_with("--"))
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(0x6d34_2026);
    let mut rng = Rng(seed);
    let mut output = vec![0u8; 1048576];
    let lunal_mode = args.iter().any(|s| s == "--lunal");
    let glyph_mode = args.iter().any(|s| s == "--glyph");
    for i in 0..count {
        let data = if lunal_mode {
            lunal_packet(&mut rng, i)
        } else if glyph_mode {
            glyph_packet(&mut rng, i)
        } else {
            packet(&mut rng, i)
        };
        let rust = if lunal_mode {
            lunal_result(&data)
        } else if glyph_mode {
            glyph_result(&data)
        } else {
            snapshot(&data)
        };
        let n = unsafe {
            if lunal_mode {
                lunal_snapshot(data.as_ptr(), data.len(), output.as_mut_ptr(), output.len())
            } else if glyph_mode {
                glyph_snapshot(data.as_ptr(), data.len(), output.as_mut_ptr(), output.len())
            } else {
                spatial_snapshot(data.as_ptr(), data.len(), output.as_mut_ptr(), output.len())
            }
        };
        let native = &output[..n];
        if glyph_mode && i < GLYPH_SCENARIOS {
            let at = bindings().len() * 4;
            let result = i32::from_le_bytes(native[at..at + 4].try_into().unwrap());
            let offset = glyph_state_offset(native);
            let field = |n: usize| {
                i32::from_le_bytes(
                    native[offset + n * 4..offset + n * 4 + 4]
                        .try_into()
                        .unwrap(),
                )
            };
            let scenario = i % GLYPH_SCENARIOS;
            if (16..=18).contains(&scenario) {
                assert_eq!(
                    field(3),
                    if scenario == 16 { 51 } else { 100 },
                    "native distance boundary"
                );
            }
            if (19..=22).contains(&scenario) {
                assert_eq!(
                    field(1),
                    if scenario >= 21 { 99 } else { 101 },
                    "native rank/tie choice"
                );
            }
            if (23..=25).contains(&scenario) {
                assert_eq!(
                    result,
                    i32::from(scenario == 23),
                    "native cost74/75/76 cutoff"
                );
                assert_eq!(
                    field(1),
                    if scenario == 23 { 101 } else { 0 },
                    "native cutoff target"
                );
            }
            println!(
                "glyph scenario={} native_result={} tracked={} target_x={} center_x={}",
                scenario,
                result,
                field(5),
                field(1),
                field(3)
            );
        }
        if native != rust {
            let at = native
                .iter()
                .zip(&rust)
                .position(|(a, b)| a != b)
                .unwrap_or(native.len().min(rust.len()));
            eprintln!("MISMATCH spatial case={i} byte={at} native_len={} rust_len={} native={:?} rust={:?}",native.len(),rust.len(),&native[at..native.len().min(at+16)],&rust[at..rust.len().min(at+16)]);
            std::process::exit(1);
        }
    }
    if env!("SPATIAL_OVERRIDE") == "1" {
        eprintln!("fault source unexpectedly matched");
        std::process::exit(2);
    }
    if lunal_mode {
        println!("PASS {count} native C/Rust lunal thinker states: exits, recovery gates, stair tracking/choice, flags, callbacks, notes and keys");
    } else if glyph_mode {
        let scenarios = count.min(GLYPH_SCENARIOS);
        println!("PASS {count} native C/Rust glyph-builder executions over {scenarios}/{GLYPH_SCENARIOS} hand-built scenarios: distance boundaries, controlled ranking ties, pattern/flow/creation/capacity, callbacks and complete declared maps/queues");
    } else {
        println!("PASS {count} native C/Rust spatial states seed={seed}: projection/motion/directions, physical melee damage and combat scores/goals, complete staged map/flow/queue state, callbacks, notes and keys");
    }
}
