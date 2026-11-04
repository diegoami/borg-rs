#[repr(C)]
pub struct dice_s {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct history_info {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct monster_group {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct trap {
    _opaque: [u8; 0],
}

extern "C" {
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static extract_energy: [uint8_t; 200];
    static mut borg_grids: [*mut borg_grid; 66];
    static adj_dex_safe: [::core::ffi::c_int; 38];
    fn borg_cave_floor_bold(y: ::core::ffi::c_int, X: ::core::ffi::c_int) -> bool;
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_feature_protected(ag: *mut borg_grid) -> bool;
    fn borg_spell_stat() -> ::core::ffi::c_int;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    static mut borg_tp_other_n: int16_t;
    static mut borg_tp_other_index: [::core::ffi::c_int; 255];
    fn borg_mon_blow_effect(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut track_glyph: borg_track;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn square_isvault(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut borg_attacking: bool;
    static mut borg_on_glyph: bool;
    static mut borg_create_door: bool;
    static mut borg_sleep_spell: bool;
    static mut borg_sleep_spell_ii: bool;
    static mut borg_crush_spell: bool;
    static mut borg_slow_spell: bool;
    static mut borg_confuse_spell: bool;
    static mut borg_fear_mon_spell: bool;
    static mut borg: borg_struct;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    fn borg_projectable(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_projectable_pure(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_distance(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type wchar_t = ::libc::wchar_t;
pub type bitflag = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_rational {
    pub n: ::core::ffi::c_uint,
    pub d: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random {
    pub base: ::core::ffi::c_int,
    pub dice: ::core::ffi::c_int,
    pub sides: ::core::ffi::c_int,
    pub m_bonus: ::core::ffi::c_int,
}
pub type random_value = random;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loc {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object {
    pub kind: *mut object_kind,
    pub ego: *mut ego_item,
    pub artifact: *const artifact,
    pub prev: *mut object,
    pub next: *mut object,
    pub known: *mut object,
    pub oidx: uint16_t,
    pub grid: loc,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
    pub weight: int16_t,
    pub dd: uint8_t,
    pub ds: uint8_t,
    pub ac: int16_t,
    pub to_a: int16_t,
    pub to_h: int16_t,
    pub to_d: int16_t,
    pub flags: [bitflag; 5],
    pub modifiers: [int16_t; 16],
    pub el_info: [element_info; 25],
    pub brands: *mut bool,
    pub slays: *mut bool,
    pub curses: *mut curse_data,
    pub effect: *mut effect,
    pub effect_msg: *mut ::core::ffi::c_char,
    pub activation: *mut activation,
    pub time: random_value,
    pub timeout: int16_t,
    pub number: uint8_t,
    pub notice: bitflag,
    pub held_m_idx: int16_t,
    pub mimicking_m_idx: int16_t,
    pub origin: uint8_t,
    pub origin_depth: uint8_t,
    pub origin_race: *const monster_race,
    pub note: quark_t,
}
pub type quark_t = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_race {
    pub next: *mut monster_race,
    pub ridx: ::core::ffi::c_uint,
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub plural: *mut ::core::ffi::c_char,
    pub base: *mut monster_base,
    pub avg_hp: ::core::ffi::c_int,
    pub ac: ::core::ffi::c_int,
    pub sleep: ::core::ffi::c_int,
    pub hearing: ::core::ffi::c_int,
    pub smell: ::core::ffi::c_int,
    pub speed: ::core::ffi::c_int,
    pub light: ::core::ffi::c_int,
    pub mexp: ::core::ffi::c_int,
    pub freq_innate: ::core::ffi::c_int,
    pub freq_spell: ::core::ffi::c_int,
    pub spell_power: ::core::ffi::c_int,
    pub flags: [bitflag; 11],
    pub spell_flags: [bitflag; 12],
    pub blow: *mut monster_blow,
    pub level: ::core::ffi::c_int,
    pub rarity: ::core::ffi::c_int,
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
    pub max_num: uint8_t,
    pub cur_num: ::core::ffi::c_int,
    pub spell_msgs: *mut monster_altmsg,
    pub drops: *mut monster_drop,
    pub friends: *mut monster_friends,
    pub friends_base: *mut monster_friends_base,
    pub mimic_kinds: *mut monster_mimic,
    pub shapes: *mut monster_shape,
    pub num_shapes: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_shape {
    pub next: *mut monster_shape,
    pub name: *mut ::core::ffi::c_char,
    pub race: *mut monster_race,
    pub base: *mut monster_base,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_base {
    pub next: *mut monster_base,
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub flags: [bitflag; 11],
    pub d_char: wchar_t,
    pub pain: *mut monster_pain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_pain {
    pub messages: [*const ::core::ffi::c_char; 7],
    pub pain_idx: ::core::ffi::c_int,
    pub next: *mut monster_pain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_mimic {
    pub next: *mut monster_mimic,
    pub kind: *mut object_kind,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_kind {
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub base: *mut object_base,
    pub next: *mut object_kind,
    pub kidx: uint32_t,
    pub tval: ::core::ffi::c_int,
    pub sval: ::core::ffi::c_int,
    pub pval: random_value,
    pub to_h: random_value,
    pub to_d: random_value,
    pub to_a: random_value,
    pub ac: ::core::ffi::c_int,
    pub dd: ::core::ffi::c_int,
    pub ds: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
    pub cost: ::core::ffi::c_int,
    pub flags: [bitflag; 5],
    pub kind_flags: [bitflag; 2],
    pub modifiers: [random_value; 16],
    pub el_info: [element_info; 25],
    pub brands: *mut bool,
    pub slays: *mut bool,
    pub curses: *mut ::core::ffi::c_int,
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
    pub alloc_prob: ::core::ffi::c_int,
    pub alloc_min: ::core::ffi::c_int,
    pub alloc_max: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub activation: *mut activation,
    pub effect: *mut effect,
    pub power: ::core::ffi::c_int,
    pub effect_msg: *mut ::core::ffi::c_char,
    pub vis_msg: *mut ::core::ffi::c_char,
    pub time: random_value,
    pub charge: random_value,
    pub gen_mult_prob: ::core::ffi::c_int,
    pub stack_size: random_value,
    pub flavor: *mut flavor,
    pub note_aware: quark_t,
    pub note_unaware: quark_t,
    pub aware: bool,
    pub tried: bool,
    pub ignore: uint8_t,
    pub everseen: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flavor {
    pub text: *mut ::core::ffi::c_char,
    pub next: *mut flavor,
    pub fidx: ::core::ffi::c_uint,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct effect {
    pub next: *mut effect,
    pub index: uint16_t,
    pub dice: *mut dice_t,
    pub y: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub radius: ::core::ffi::c_int,
    pub other: ::core::ffi::c_int,
    pub msg: *mut ::core::ffi::c_char,
}
pub type dice_t = dice_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct activation {
    pub next: *mut activation,
    pub name: *mut ::core::ffi::c_char,
    pub index: ::core::ffi::c_int,
    pub aim: bool,
    pub level: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub effect: *mut effect,
    pub message: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element_info {
    pub res_level: int16_t,
    pub flags: bitflag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_base {
    pub name: *mut ::core::ffi::c_char,
    pub tval: ::core::ffi::c_int,
    pub next: *mut object_base,
    pub attr: ::core::ffi::c_int,
    pub flags: [bitflag; 5],
    pub kind_flags: [bitflag; 2],
    pub el_info: [element_info; 25],
    pub break_perc: ::core::ffi::c_int,
    pub max_stack: ::core::ffi::c_int,
    pub num_svals: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_friends_base {
    pub next: *mut monster_friends_base,
    pub base: *mut monster_base,
    pub role: monster_group_role,
    pub percent_chance: ::core::ffi::c_uint,
    pub number_dice: ::core::ffi::c_uint,
    pub number_side: ::core::ffi::c_uint,
}
pub type monster_group_role = ::core::ffi::c_uint;
pub const MON_GROUP_SUMMON: monster_group_role = 4;
pub const MON_GROUP_MEMBER: monster_group_role = 3;
pub const MON_GROUP_BODYGUARD: monster_group_role = 2;
pub const MON_GROUP_SERVANT: monster_group_role = 1;
pub const MON_GROUP_LEADER: monster_group_role = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_friends {
    pub next: *mut monster_friends,
    pub name: *mut ::core::ffi::c_char,
    pub race: *mut monster_race,
    pub role: monster_group_role,
    pub percent_chance: ::core::ffi::c_uint,
    pub number_dice: ::core::ffi::c_uint,
    pub number_side: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_drop {
    pub next: *mut monster_drop,
    pub kind: *mut object_kind,
    pub tval: ::core::ffi::c_uint,
    pub percent_chance: ::core::ffi::c_uint,
    pub min: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_altmsg {
    pub next: *mut monster_altmsg,
    pub message: *mut ::core::ffi::c_char,
    pub msg_type: monster_altmsg_type,
    pub index: uint16_t,
}
pub type monster_altmsg_type = ::core::ffi::c_uint;
pub const MON_ALTMSG_MISS: monster_altmsg_type = 2;
pub const MON_ALTMSG_UNSEEN: monster_altmsg_type = 1;
pub const MON_ALTMSG_SEEN: monster_altmsg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_blow {
    pub next: *mut monster_blow,
    pub method: *mut blow_method,
    pub effect: *mut blow_effect,
    pub dice: random_value,
    pub times_seen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blow_effect {
    pub name: *mut ::core::ffi::c_char,
    pub power: ::core::ffi::c_int,
    pub eval: ::core::ffi::c_int,
    pub desc: *mut ::core::ffi::c_char,
    pub lore_attr: uint8_t,
    pub lore_attr_resist: uint8_t,
    pub lore_attr_immune: uint8_t,
    pub effect_type: *mut ::core::ffi::c_char,
    pub resist: ::core::ffi::c_int,
    pub lash_type: ::core::ffi::c_int,
    pub next: *mut blow_effect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blow_method {
    pub name: *mut ::core::ffi::c_char,
    pub cut: bool,
    pub stun: bool,
    pub miss: bool,
    pub phys: bool,
    pub msgt: ::core::ffi::c_int,
    pub messages: *mut blow_message,
    pub num_messages: ::core::ffi::c_int,
    pub desc: *mut ::core::ffi::c_char,
    pub next: *mut blow_method,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blow_message {
    pub act_msg: *mut ::core::ffi::c_char,
    pub next: *mut blow_message,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curse_data {
    pub power: ::core::ffi::c_int,
    pub timeout: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct artifact {
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub aidx: uint32_t,
    pub next: *mut artifact,
    pub tval: ::core::ffi::c_int,
    pub sval: ::core::ffi::c_int,
    pub to_h: ::core::ffi::c_int,
    pub to_d: ::core::ffi::c_int,
    pub to_a: ::core::ffi::c_int,
    pub ac: ::core::ffi::c_int,
    pub dd: ::core::ffi::c_int,
    pub ds: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
    pub cost: ::core::ffi::c_int,
    pub flags: [bitflag; 5],
    pub modifiers: [::core::ffi::c_int; 16],
    pub el_info: [element_info; 25],
    pub brands: *mut bool,
    pub slays: *mut bool,
    pub curses: *mut ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub alloc_prob: ::core::ffi::c_int,
    pub alloc_min: ::core::ffi::c_int,
    pub alloc_max: ::core::ffi::c_int,
    pub activation: *mut activation,
    pub alt_msg: *mut ::core::ffi::c_char,
    pub time: random_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ego_item {
    pub next: *mut ego_item,
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub eidx: uint32_t,
    pub cost: ::core::ffi::c_int,
    pub flags: [bitflag; 5],
    pub flags_off: [bitflag; 5],
    pub kind_flags: [bitflag; 2],
    pub modifiers: [random_value; 16],
    pub min_modifiers: [::core::ffi::c_int; 16],
    pub el_info: [element_info; 25],
    pub brands: *mut bool,
    pub slays: *mut bool,
    pub curses: *mut ::core::ffi::c_int,
    pub rating: ::core::ffi::c_int,
    pub alloc_prob: ::core::ffi::c_int,
    pub alloc_min: ::core::ffi::c_int,
    pub alloc_max: ::core::ffi::c_int,
    pub poss_items: *mut poss_item,
    pub to_h: random_value,
    pub to_d: random_value,
    pub to_a: random_value,
    pub min_to_h: ::core::ffi::c_int,
    pub min_to_d: ::core::ffi::c_int,
    pub min_to_a: ::core::ffi::c_int,
    pub activation: *mut activation,
    pub time: random_value,
    pub everseen: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poss_item {
    pub kidx: uint32_t,
    pub next: *mut poss_item,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const OF_MAX: C2RustUnnamed = 39;
pub const OF_MULTIPLY_WEIGHT: C2RustUnnamed = 38;
pub const OF_THROWING: C2RustUnnamed = 37;
pub const OF_TRAP_IMMUNE: C2RustUnnamed = 36;
pub const OF_EXPLODE: C2RustUnnamed = 35;
pub const OF_DIG_3: C2RustUnnamed = 34;
pub const OF_DIG_2: C2RustUnnamed = 33;
pub const OF_DIG_1: C2RustUnnamed = 32;
pub const OF_LIGHT_3: C2RustUnnamed = 31;
pub const OF_LIGHT_2: C2RustUnnamed = 30;
pub const OF_FRAGILE: C2RustUnnamed = 29;
pub const OF_STICKY: C2RustUnnamed = 28;
pub const OF_DRAIN_EXP: C2RustUnnamed = 27;
pub const OF_AGGRAVATE: C2RustUnnamed = 26;
pub const OF_NO_TELEPORT: C2RustUnnamed = 25;
pub const OF_AFRAID: C2RustUnnamed = 24;
pub const OF_IMPAIR_MANA: C2RustUnnamed = 23;
pub const OF_IMPAIR_HP: C2RustUnnamed = 22;
pub const OF_NO_FUEL: C2RustUnnamed = 21;
pub const OF_TAKES_FUEL: C2RustUnnamed = 20;
pub const OF_BURNS_OUT: C2RustUnnamed = 19;
pub const OF_BLESSED: C2RustUnnamed = 18;
pub const OF_IMPACT: C2RustUnnamed = 17;
pub const OF_HOLD_LIFE: C2RustUnnamed = 16;
pub const OF_FREE_ACT: C2RustUnnamed = 15;
pub const OF_SEE_INVIS: C2RustUnnamed = 14;
pub const OF_TELEPATHY: C2RustUnnamed = 13;
pub const OF_REGEN: C2RustUnnamed = 12;
pub const OF_FEATHER: C2RustUnnamed = 11;
pub const OF_SLOW_DIGEST: C2RustUnnamed = 10;
pub const OF_PROT_STUN: C2RustUnnamed = 9;
pub const OF_PROT_CONF: C2RustUnnamed = 8;
pub const OF_PROT_BLIND: C2RustUnnamed = 7;
pub const OF_PROT_FEAR: C2RustUnnamed = 6;
pub const OF_SUST_CON: C2RustUnnamed = 5;
pub const OF_SUST_DEX: C2RustUnnamed = 4;
pub const OF_SUST_WIS: C2RustUnnamed = 3;
pub const OF_SUST_INT: C2RustUnnamed = 2;
pub const OF_SUST_STR: C2RustUnnamed = 1;
pub const OF_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const STAT_MAX: C2RustUnnamed_0 = 5;
pub const STAT_CON: C2RustUnnamed_0 = 4;
pub const STAT_DEX: C2RustUnnamed_0 = 3;
pub const STAT_WIS: C2RustUnnamed_0 = 2;
pub const STAT_INT: C2RustUnnamed_0 = 1;
pub const STAT_STR: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quest {
    pub next: *mut quest,
    pub index: uint8_t,
    pub name: *mut ::core::ffi::c_char,
    pub level: uint8_t,
    pub race: *mut monster_race,
    pub cur_num: ::core::ffi::c_int,
    pub max_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct equip_slot {
    pub next: *mut equip_slot,
    pub type_0: uint16_t,
    pub name: *mut ::core::ffi::c_char,
    pub obj: *mut object,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_body {
    pub next: *mut player_body,
    pub name: *mut ::core::ffi::c_char,
    pub count: uint16_t,
    pub slots: *mut equip_slot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race {
    pub next: *mut player_race,
    pub name: *const ::core::ffi::c_char,
    pub ridx: ::core::ffi::c_uint,
    pub r_mhp: ::core::ffi::c_int,
    pub r_exp: ::core::ffi::c_int,
    pub b_age: ::core::ffi::c_int,
    pub m_age: ::core::ffi::c_int,
    pub base_hgt: ::core::ffi::c_int,
    pub mod_hgt: ::core::ffi::c_int,
    pub base_wgt: ::core::ffi::c_int,
    pub mod_wgt: ::core::ffi::c_int,
    pub infra: ::core::ffi::c_int,
    pub body: ::core::ffi::c_int,
    pub r_adj: [::core::ffi::c_int; 5],
    pub r_skills: [::core::ffi::c_int; 10],
    pub flags: [bitflag; 5],
    pub pflags: [bitflag; 3],
    pub history: *mut history_chart,
    pub el_info: [element_info; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct history_chart {
    pub next: *mut history_chart,
    pub entries: *mut history_entry,
    pub idx: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct history_entry {
    pub next: *mut history_entry,
    pub succ: *mut history_chart,
    pub isucc: ::core::ffi::c_int,
    pub roll: ::core::ffi::c_int,
    pub text: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_blow {
    pub next: *mut player_blow,
    pub name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_shape {
    pub next: *mut player_shape,
    pub name: *const ::core::ffi::c_char,
    pub sidx: ::core::ffi::c_int,
    pub to_a: ::core::ffi::c_int,
    pub to_h: ::core::ffi::c_int,
    pub to_d: ::core::ffi::c_int,
    pub skills: [::core::ffi::c_int; 10],
    pub flags: [bitflag; 5],
    pub pflags: [bitflag; 3],
    pub modifiers: [::core::ffi::c_int; 16],
    pub el_info: [element_info; 25],
    pub effect: *mut effect,
    pub blows: *mut player_blow,
    pub num_blows: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct start_item {
    pub tval: ::core::ffi::c_int,
    pub sval: ::core::ffi::c_int,
    pub min: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub eopts: *mut ::core::ffi::c_int,
    pub next: *mut start_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct magic_realm {
    pub next: *mut magic_realm,
    pub code: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub stat: ::core::ffi::c_int,
    pub verb: *mut ::core::ffi::c_char,
    pub spell_noun: *mut ::core::ffi::c_char,
    pub book_noun: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct class_spell {
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub effect: *mut effect,
    pub realm: *const magic_realm,
    pub sidx: ::core::ffi::c_int,
    pub bidx: ::core::ffi::c_int,
    pub slevel: ::core::ffi::c_int,
    pub smana: ::core::ffi::c_int,
    pub sfail: ::core::ffi::c_int,
    pub sexp: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct class_book {
    pub tval: ::core::ffi::c_int,
    pub sval: ::core::ffi::c_int,
    pub dungeon: bool,
    pub num_spells: ::core::ffi::c_int,
    pub realm: *const magic_realm,
    pub spells: *mut class_spell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct class_magic {
    pub spell_first: ::core::ffi::c_int,
    pub spell_weight: ::core::ffi::c_int,
    pub num_books: ::core::ffi::c_int,
    pub books: *mut class_book,
    pub total_spells: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_class {
    pub next: *mut player_class,
    pub name: *const ::core::ffi::c_char,
    pub cidx: ::core::ffi::c_uint,
    pub title: [*const ::core::ffi::c_char; 10],
    pub c_adj: [::core::ffi::c_int; 5],
    pub c_skills: [::core::ffi::c_int; 10],
    pub x_skills: [::core::ffi::c_int; 10],
    pub c_mhp: ::core::ffi::c_int,
    pub c_exp: ::core::ffi::c_int,
    pub flags: [bitflag; 5],
    pub pflags: [bitflag; 3],
    pub max_attacks: ::core::ffi::c_int,
    pub min_weight: ::core::ffi::c_int,
    pub att_multiply: ::core::ffi::c_int,
    pub start_items: *mut start_item,
    pub magic: class_magic,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_history {
    pub entries: *mut history_info,
    pub next: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_state {
    pub stat_add: [::core::ffi::c_int; 5],
    pub stat_ind: [::core::ffi::c_int; 5],
    pub stat_use: [::core::ffi::c_int; 5],
    pub stat_top: [::core::ffi::c_int; 5],
    pub skills: [::core::ffi::c_int; 10],
    pub speed: ::core::ffi::c_int,
    pub num_blows: ::core::ffi::c_int,
    pub num_shots: ::core::ffi::c_int,
    pub num_moves: ::core::ffi::c_int,
    pub ammo_mult: ::core::ffi::c_int,
    pub ammo_tval: ::core::ffi::c_int,
    pub ac: ::core::ffi::c_int,
    pub dam_red: ::core::ffi::c_int,
    pub perc_dam_red: ::core::ffi::c_int,
    pub to_a: ::core::ffi::c_int,
    pub to_h: ::core::ffi::c_int,
    pub to_d: ::core::ffi::c_int,
    pub see_infra: ::core::ffi::c_int,
    pub cur_light: ::core::ffi::c_int,
    pub heavy_wield: bool,
    pub heavy_shoot: bool,
    pub bless_wield: bool,
    pub cumber_armor: bool,
    pub flags: [bitflag; 5],
    pub pflags: [bitflag; 3],
    pub el_info: [element_info; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_upkeep {
    pub playing: bool,
    pub autosave: bool,
    pub generate_level: bool,
    pub only_partial: bool,
    pub dropping: bool,
    pub energy_use: ::core::ffi::c_int,
    pub new_spells: ::core::ffi::c_int,
    pub health_who: *mut monster,
    pub monster_race: *mut monster_race,
    pub object: *mut object,
    pub object_kind: *mut object_kind,
    pub notice: uint32_t,
    pub update: uint32_t,
    pub redraw: uint32_t,
    pub command_wrk: ::core::ffi::c_int,
    pub create_up_stair: bool,
    pub create_down_stair: bool,
    pub light_level: bool,
    pub arena_level: bool,
    pub resting: ::core::ffi::c_int,
    pub running: ::core::ffi::c_int,
    pub running_firststep: bool,
    pub quiver: *mut *mut object,
    pub inven: *mut *mut object,
    pub total_weight: ::core::ffi::c_int,
    pub inven_cnt: ::core::ffi::c_int,
    pub equip_cnt: ::core::ffi::c_int,
    pub quiver_cnt: ::core::ffi::c_int,
    pub recharge_pow: ::core::ffi::c_int,
    pub step_count: ::core::ffi::c_int,
    pub steps: *mut int16_t,
    pub path_dest: loc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster {
    pub race: *mut monster_race,
    pub original_race: *mut monster_race,
    pub midx: ::core::ffi::c_int,
    pub grid: loc,
    pub hp: int16_t,
    pub maxhp: int16_t,
    pub m_timed: [int16_t; 10],
    pub mspeed: uint8_t,
    pub energy: uint8_t,
    pub cdis: uint8_t,
    pub mflag: [bitflag; 2],
    pub mimicked_obj: *mut object,
    pub held_obj: *mut object,
    pub attr: uint8_t,
    pub known_pstate: monster_learned_player,
    pub target: target,
    pub group_info: [monster_group_info; 2],
    pub heatmap: heatmap,
    pub min_range: uint8_t,
    pub best_range: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heatmap {
    pub grids: *mut *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_group_info {
    pub index: ::core::ffi::c_int,
    pub role: monster_group_role,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct target {
    pub grid: loc,
    pub midx: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_learned_player {
    pub flags: [bitflag; 5],
    pub pflags: [bitflag; 3],
    pub res_level: [int16_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player {
    pub race: *const player_race,
    pub class: *const player_class,
    pub grid: loc,
    pub old_grid: loc,
    pub hitdie: uint8_t,
    pub expfact: uint8_t,
    pub age: int16_t,
    pub ht: int16_t,
    pub wt: int16_t,
    pub au: int32_t,
    pub max_depth: int16_t,
    pub recall_depth: int16_t,
    pub depth: int16_t,
    pub max_lev: int16_t,
    pub lev: int16_t,
    pub max_exp: int32_t,
    pub exp: int32_t,
    pub exp_frac: uint16_t,
    pub mhp: int16_t,
    pub chp: int16_t,
    pub chp_frac: uint16_t,
    pub msp: int16_t,
    pub csp: int16_t,
    pub csp_frac: uint16_t,
    pub stat_max: [int16_t; 5],
    pub stat_cur: [int16_t; 5],
    pub stat_map: [int16_t; 5],
    pub timed: *mut int16_t,
    pub word_recall: int16_t,
    pub deep_descent: int16_t,
    pub energy: int16_t,
    pub total_energy: uint32_t,
    pub resting_turn: uint32_t,
    pub food: int16_t,
    pub unignoring: uint8_t,
    pub skip_cmd_coercion: uint8_t,
    pub noise_grid: loc,
    pub noise_falloff: uint16_t,
    pub spell_flags: *mut uint8_t,
    pub spell_order: *mut uint8_t,
    pub full_name: [::core::ffi::c_char; 32],
    pub died_from: [::core::ffi::c_char; 80],
    pub history: *mut ::core::ffi::c_char,
    pub quests: *mut quest,
    pub total_winner: uint16_t,
    pub noscore: uint16_t,
    pub is_dead: bool,
    pub wizard: bool,
    pub player_hp: [int16_t; 50],
    pub au_birth: int32_t,
    pub stat_birth: [int16_t; 5],
    pub ht_birth: int16_t,
    pub wt_birth: int16_t,
    pub opts: player_options,
    pub hist: player_history,
    pub body: player_body,
    pub shape: *mut player_shape,
    pub gear: *mut object,
    pub gear_k: *mut object,
    pub obj_k: *mut object,
    pub cave: *mut chunk,
    pub state: player_state,
    pub known_state: player_state,
    pub upkeep: *mut player_upkeep,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub name: *mut ::core::ffi::c_char,
    pub turn: int32_t,
    pub depth: ::core::ffi::c_int,
    pub feeling: uint8_t,
    pub obj_rating: uint32_t,
    pub mon_rating: uint32_t,
    pub good_item: bool,
    pub height: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub feeling_squares: uint16_t,
    pub feat_count: *mut ::core::ffi::c_int,
    pub squares: *mut *mut square,
    pub noise: heatmap,
    pub scent: heatmap,
    pub decoy: loc,
    pub objects: *mut *mut object,
    pub obj_max: uint16_t,
    pub monsters: *mut monster,
    pub mon_max: uint16_t,
    pub mon_cnt: uint16_t,
    pub mon_current: ::core::ffi::c_int,
    pub num_repro: ::core::ffi::c_int,
    pub monster_groups: *mut *mut monster_group,
    pub join: *mut connector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connector {
    pub grid: loc,
    pub feat: uint8_t,
    pub info: *mut bitflag,
    pub next: *mut connector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct square {
    pub feat: uint8_t,
    pub info: *mut bitflag,
    pub light: ::core::ffi::c_int,
    pub mon: int16_t,
    pub obj: *mut object,
    pub trap: *mut trap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_grid {
    pub feat: uint8_t,
    pub info: uint16_t,
    pub trap: bool,
    pub glyph: bool,
    pub web: bool,
    pub store: uint8_t,
    pub take: uint8_t,
    pub kill: uint8_t,
    pub xtra: uint8_t,
}
pub type BORG_MONBLOW = ::core::ffi::c_uint;
pub const MONBLOW_UNDEFINED: BORG_MONBLOW = 30;
pub const MONBLOW_BLACK_BREATH: BORG_MONBLOW = 29;
pub const MONBLOW_HALLU: BORG_MONBLOW = 28;
pub const MONBLOW_EXP_80: BORG_MONBLOW = 27;
pub const MONBLOW_EXP_40: BORG_MONBLOW = 26;
pub const MONBLOW_EXP_20: BORG_MONBLOW = 25;
pub const MONBLOW_EXP_10: BORG_MONBLOW = 24;
pub const MONBLOW_SHATTER: BORG_MONBLOW = 23;
pub const MONBLOW_LOSE_ALL: BORG_MONBLOW = 22;
pub const MONBLOW_LOSE_CON: BORG_MONBLOW = 21;
pub const MONBLOW_LOSE_DEX: BORG_MONBLOW = 20;
pub const MONBLOW_LOSE_WIS: BORG_MONBLOW = 19;
pub const MONBLOW_LOSE_INT: BORG_MONBLOW = 18;
pub const MONBLOW_LOSE_STR: BORG_MONBLOW = 17;
pub const MONBLOW_PARALYZE: BORG_MONBLOW = 16;
pub const MONBLOW_TERRIFY: BORG_MONBLOW = 15;
pub const MONBLOW_CONFUSE: BORG_MONBLOW = 14;
pub const MONBLOW_BLIND: BORG_MONBLOW = 13;
pub const MONBLOW_COLD: BORG_MONBLOW = 12;
pub const MONBLOW_FIRE: BORG_MONBLOW = 11;
pub const MONBLOW_ELEC: BORG_MONBLOW = 10;
pub const MONBLOW_ACID: BORG_MONBLOW = 9;
pub const MONBLOW_EAT_LIGHT: BORG_MONBLOW = 8;
pub const MONBLOW_EAT_FOOD: BORG_MONBLOW = 7;
pub const MONBLOW_EAT_ITEM: BORG_MONBLOW = 6;
pub const MONBLOW_EAT_GOLD: BORG_MONBLOW = 5;
pub const MONBLOW_DRAIN_CHARGES: BORG_MONBLOW = 4;
pub const MONBLOW_DISENCHANT: BORG_MONBLOW = 3;
pub const MONBLOW_POISON: BORG_MONBLOW = 2;
pub const MONBLOW_HURT: BORG_MONBLOW = 1;
pub const MONBLOW_NONE: BORG_MONBLOW = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_kill {
    pub r_idx: uint16_t,
    pub known: bool,
    pub awake: bool,
    pub confused: bool,
    pub afraid: bool,
    pub quiver: bool,
    pub stunned: bool,
    pub poisoned: bool,
    pub seen: bool,
    pub used: bool,
    pub pos: loc,
    pub ox: uint8_t,
    pub oy: uint8_t,
    pub speed: uint8_t,
    pub moves: uint8_t,
    pub ranged_attack: uint8_t,
    pub spell: [uint8_t; 92],
    pub power: int16_t,
    pub injury: int16_t,
    pub other: int16_t,
    pub level: int16_t,
    pub spell_flags: [uint32_t; 85],
    pub when: borg_time,
    pub m_idx: int16_t,
}
pub type borg_time = int32_t;
pub const BI_CLEVEL: C2RustUnnamed_12 = 35;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_struct {
    pub player: *mut player,
    pub trait_0: *mut ::core::ffi::c_int,
    pub has: *mut ::core::ffi::c_int,
    pub activation: *mut ::core::ffi::c_int,
    pub power: int32_t,
    pub c: loc,
    pub panels: loc,
    pub avoidance: int16_t,
    pub oldchp: int16_t,
    pub lunal_mode: bool,
    pub munchkin_mode: bool,
    pub stair_less: bool,
    pub stair_more: bool,
    pub in_shop: bool,
    pub ready_morgoth: ::core::ffi::c_int,
    pub temp: C2RustUnnamed_8,
    pub status: C2RustUnnamed_7,
    pub time: C2RustUnnamed_6,
    pub need_see_invis: borg_time,
    pub see_inv: int16_t,
    pub need_shift_panel: bool,
    pub when_shift_panel: borg_time,
    pub antibounce_count: int16_t,
    pub no_retreat: int16_t,
    pub resistance: int16_t,
    pub no_rest_prep: int16_t,
    pub times_twitch: int16_t,
    pub escapes: int16_t,
    pub trying_unknown: bool,
    pub dont_react: bool,
    pub targeting: bool,
    pub goal: C2RustUnnamed_5,
    pub init_save: C2RustUnnamed_4,
    pub mon: C2RustUnnamed_3,
    pub id: C2RustUnnamed_2,
    pub near: C2RustUnnamed_1,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub morgoth: ::core::ffi::c_uint,
    pub sauron: ::core::ffi::c_uint,
    pub tarrasque: ::core::ffi::c_uint,
    pub maggot: ::core::ffi::c_uint,
    pub bullroarer: ::core::ffi::c_uint,
    pub beorn: ::core::ffi::c_uint,
    pub grip: ::core::ffi::c_uint,
    pub fang: ::core::ffi::c_uint,
    pub osse: ::core::ffi::c_uint,
    pub squint_eyed_rogue: ::core::ffi::c_uint,
    pub small_kobold: ::core::ffi::c_uint,
    pub soldier: ::core::ffi::c_uint,
    pub cutpurse: ::core::ffi::c_uint,
    pub acolyte: ::core::ffi::c_uint,
    pub apprentice: ::core::ffi::c_uint,
    pub kobold: ::core::ffi::c_uint,
    pub shrieker_mushroom_patch: ::core::ffi::c_uint,
    pub filthy_street_urchin: ::core::ffi::c_uint,
    pub battle_scarred_veteran: ::core::ffi::c_uint,
    pub mean_looking_mercenary: ::core::ffi::c_uint,
    pub giant_white_mouse: ::core::ffi::c_uint,
    pub white_worm_mass: ::core::ffi::c_uint,
    pub green_worm_mass: ::core::ffi::c_uint,
    pub cave_spider: ::core::ffi::c_uint,
    pub red_naga: ::core::ffi::c_uint,
    pub giant_red_frog: ::core::ffi::c_uint,
    pub radiation_eye: ::core::ffi::c_uint,
    pub yellow_worm_mass: ::core::ffi::c_uint,
    pub gravity_hound: ::core::ffi::c_uint,
    pub inertia_hound: ::core::ffi::c_uint,
    pub dread: ::core::ffi::c_uint,
    pub dracolich: ::core::ffi::c_uint,
    pub dracolisk: ::core::ffi::c_uint,
    pub light_hound: ::core::ffi::c_uint,
    pub dark_hound: ::core::ffi::c_uint,
    pub pukelman: ::core::ffi::c_uint,
    pub night_mare: ::core::ffi::c_uint,
    pub drolem: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub type_0: int16_t,
    pub g: loc,
    pub rising: bool,
    pub leaving: bool,
    pub fleeing: bool,
    pub fleeing_lunal: bool,
    pub fleeing_munchkin: bool,
    pub fleeing_to_town: bool,
    pub ignoring: bool,
    pub less: bool,
    pub waiting: bool,
    pub recalling: ::core::ffi::c_int,
    pub descending: ::core::ffi::c_int,
    pub shop: int16_t,
    pub ware: int16_t,
    pub item: int16_t,
    pub do_best: bool,
    pub best_item: *mut borg_best,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_best {
    pub home: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub now: borg_time,
    pub level: borg_time,
    pub town: borg_time,
    pub morgoth: borg_time,
    pub antisummon: borg_time,
    pub call_light: borg_time,
    pub wizard_light: borg_time,
    pub detect_traps: borg_time,
    pub detect_doors: borg_time,
    pub detect_walls: borg_time,
    pub detect_evil: borg_time,
    pub detect_obj: borg_time,
    pub last_kill_mult: borg_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub active: bool,
    pub cancel: bool,
    pub save: bool,
    pub old_depth: int16_t,
    pub respawning: int16_t,
    pub cheat_death: bool,
    pub anti_summon: bool,
    pub digging: bool,
    pub need_alter: bool,
    pub no_alter: bool,
    pub redraw: bool,
    pub vault: bool,
    pub desperate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub need_see_invis: int16_t,
    pub see_inv: int16_t,
    pub res_fire: bool,
    pub res_cold: bool,
    pub res_acid: bool,
    pub res_elec: bool,
    pub res_pois: bool,
    pub prot_from_evil: bool,
    pub fast: bool,
    pub bless: bool,
    pub hero: bool,
    pub berserk: bool,
    pub fastcast: bool,
    pub regen: bool,
    pub smite_evil: bool,
    pub venom: bool,
    pub shield: bool,
}
pub const RF_UNIQUE: C2RustUnnamed_10 = 1;
pub const RF_MAX: C2RustUnnamed_10 = 85;
pub const RF_NO_SLEEP: C2RustUnnamed_10 = 82;
pub const RF_MULTIPLY: C2RustUnnamed_10 = 17;
pub const RF_KILL_WALL: C2RustUnnamed_10 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_10 = 46;
pub const FEAT_RUBBLE: C2RustUnnamed_9 = 16;
pub const FEAT_QUARTZ_K: C2RustUnnamed_9 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_9 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_9 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_9 = 17;
pub const FEAT_GRANITE: C2RustUnnamed_9 = 21;
pub const FEAT_PERM: C2RustUnnamed_9 = 22;
pub const FEAT_CLOSED: C2RustUnnamed_9 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct angband_constants {
    pub store_max: uint16_t,
    pub trap_max: uint16_t,
    pub k_max: uint16_t,
    pub a_max: uint16_t,
    pub e_max: uint16_t,
    pub r_max: uint16_t,
    pub mp_max: uint16_t,
    pub s_max: uint16_t,
    pub pit_max: uint16_t,
    pub act_max: uint16_t,
    pub curse_max: uint8_t,
    pub slay_max: uint8_t,
    pub brand_max: uint8_t,
    pub mon_blows_max: uint16_t,
    pub blow_methods_max: uint16_t,
    pub blow_effects_max: uint16_t,
    pub equip_slots_max: uint16_t,
    pub profile_max: uint16_t,
    pub quest_max: uint16_t,
    pub projection_max: uint16_t,
    pub calculation_max: uint16_t,
    pub property_max: uint16_t,
    pub ordinary_kind_max: uint16_t,
    pub shape_max: uint16_t,
    pub level_monster_max: uint16_t,
    pub alloc_monster_chance: uint16_t,
    pub level_monster_min: uint16_t,
    pub town_monsters_day: uint16_t,
    pub town_monsters_night: uint16_t,
    pub repro_monster_max: uint16_t,
    pub ood_monster_chance: uint16_t,
    pub ood_monster_amount: uint16_t,
    pub monster_group_max: uint16_t,
    pub monster_group_dist: uint16_t,
    pub glyph_hardness: uint16_t,
    pub repro_monster_rate: uint16_t,
    pub life_drain_percent: uint16_t,
    pub flee_range: uint16_t,
    pub turn_range: uint16_t,
    pub level_room_max: uint16_t,
    pub level_door_max: uint16_t,
    pub wall_pierce_max: uint16_t,
    pub tunn_grid_max: uint16_t,
    pub room_item_av: uint16_t,
    pub both_item_av: uint16_t,
    pub both_gold_av: uint16_t,
    pub level_pit_max: uint16_t,
    pub max_depth: uint16_t,
    pub day_length: uint16_t,
    pub dungeon_hgt: uint16_t,
    pub dungeon_wid: uint16_t,
    pub town_hgt: uint16_t,
    pub town_wid: uint16_t,
    pub feeling_total: uint16_t,
    pub feeling_need: uint16_t,
    pub stair_skip: uint16_t,
    pub move_energy: uint16_t,
    pub pack_size: uint16_t,
    pub quiver_size: uint16_t,
    pub quiver_slot_size: uint16_t,
    pub thrown_quiver_mult: uint16_t,
    pub floor_size: uint16_t,
    pub store_inven_max: uint16_t,
    pub store_turns: uint16_t,
    pub store_shuffle: uint16_t,
    pub store_magic_level: uint16_t,
    pub max_obj_depth: uint16_t,
    pub great_obj: uint16_t,
    pub great_ego: uint16_t,
    pub fuel_torch: uint16_t,
    pub fuel_lamp: uint16_t,
    pub default_lamp: uint16_t,
    pub max_sight: uint16_t,
    pub max_range: uint16_t,
    pub start_gold: uint16_t,
    pub food_value: uint16_t,
    pub m_crit_debuff_toh: ::core::ffi::c_int,
    pub m_crit_chance_weight_scl: ::core::ffi::c_int,
    pub m_crit_chance_toh_scl: ::core::ffi::c_int,
    pub m_crit_chance_level_scl: ::core::ffi::c_int,
    pub m_crit_chance_toh_skill_scl: ::core::ffi::c_int,
    pub m_crit_chance_offset: ::core::ffi::c_int,
    pub m_crit_chance_range: ::core::ffi::c_int,
    pub m_crit_power_weight_scl: ::core::ffi::c_int,
    pub m_crit_power_random: ::core::ffi::c_int,
    pub m_crit_level_head: *mut critical_level,
    pub r_crit_debuff_toh: ::core::ffi::c_int,
    pub r_crit_chance_weight_scl: ::core::ffi::c_int,
    pub r_crit_chance_toh_scl: ::core::ffi::c_int,
    pub r_crit_chance_level_scl: ::core::ffi::c_int,
    pub r_crit_chance_launched_toh_skill_scl: ::core::ffi::c_int,
    pub r_crit_chance_thrown_toh_skill_scl: ::core::ffi::c_int,
    pub r_crit_chance_offset: ::core::ffi::c_int,
    pub r_crit_chance_range: ::core::ffi::c_int,
    pub r_crit_power_weight_scl: ::core::ffi::c_int,
    pub r_crit_power_random: ::core::ffi::c_int,
    pub r_crit_level_head: *mut critical_level,
    pub o_m_crit_debuff_toh: ::core::ffi::c_int,
    pub o_m_crit_power_toh_scl_num: ::core::ffi::c_int,
    pub o_m_crit_power_toh_scl_den: ::core::ffi::c_int,
    pub o_m_crit_chance_power_scl_num: ::core::ffi::c_int,
    pub o_m_crit_chance_power_scl_den: ::core::ffi::c_int,
    pub o_m_crit_chance_add_den: ::core::ffi::c_int,
    pub o_m_crit_level_head: *mut o_critical_level,
    pub o_m_max_added: my_rational,
    pub o_r_crit_debuff_toh: ::core::ffi::c_int,
    pub o_r_crit_power_launched_toh_scl_num: ::core::ffi::c_int,
    pub o_r_crit_power_launched_toh_scl_den: ::core::ffi::c_int,
    pub o_r_crit_power_thrown_toh_scl_num: ::core::ffi::c_int,
    pub o_r_crit_power_thrown_toh_scl_den: ::core::ffi::c_int,
    pub o_r_crit_chance_power_scl_num: ::core::ffi::c_int,
    pub o_r_crit_chance_power_scl_den: ::core::ffi::c_int,
    pub o_r_crit_chance_add_den: ::core::ffi::c_int,
    pub o_r_crit_level_head: *mut o_critical_level,
    pub o_r_max_added: my_rational,
    pub max_hearing: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct o_critical_level {
    pub next: *mut o_critical_level,
    pub chance: ::core::ffi::c_uint,
    pub added_dice: ::core::ffi::c_uint,
    pub msgt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct critical_level {
    pub next: *mut critical_level,
    pub cutoff: ::core::ffi::c_int,
    pub mult: ::core::ffi::c_int,
    pub add: ::core::ffi::c_int,
    pub msgt: ::core::ffi::c_int,
}
pub const BI_CURHP: C2RustUnnamed_12 = 27;
pub const BI_DAM_RED: C2RustUnnamed_12 = 47;
pub const BI_MAXDEPTH: C2RustUnnamed_12 = 106;
pub const RSF_S_UNIQUE: C2RustUnnamed_11 = 91;
pub const RSF_S_WRAITH: C2RustUnnamed_11 = 90;
pub const RSF_S_HI_DRAGON: C2RustUnnamed_11 = 89;
pub const RSF_S_HI_UNDEAD: C2RustUnnamed_11 = 88;
pub const RSF_S_DRAGON: C2RustUnnamed_11 = 87;
pub const RSF_S_UNDEAD: C2RustUnnamed_11 = 86;
pub const RSF_S_DEMON: C2RustUnnamed_11 = 85;
pub const RSF_S_AINU: C2RustUnnamed_11 = 84;
pub const RSF_S_HYDRA: C2RustUnnamed_11 = 83;
pub const RSF_S_HOUND: C2RustUnnamed_11 = 82;
pub const RSF_S_SPIDER: C2RustUnnamed_11 = 81;
pub const RSF_S_ANIMAL: C2RustUnnamed_11 = 80;
pub const RSF_S_MONSTERS: C2RustUnnamed_11 = 79;
pub const RSF_S_MONSTER: C2RustUnnamed_11 = 78;
pub const RSF_S_HI_DEMON: C2RustUnnamed_11 = 77;
pub const RSF_S_KIN: C2RustUnnamed_11 = 76;
pub const RSF_SHAPECHANGE: C2RustUnnamed_11 = 75;
pub const BI_CURSP: C2RustUnnamed_12 = 30;
pub const BI_SAV: C2RustUnnamed_12 = 57;
pub const RSF_FORGET: C2RustUnnamed_11 = 74;
pub const RSF_TRAPS: C2RustUnnamed_11 = 73;
pub const RSF_DARKNESS: C2RustUnnamed_11 = 72;
pub const RSF_TELE_LEVEL: C2RustUnnamed_11 = 71;
pub const RSF_TELE_AWAY: C2RustUnnamed_11 = 70;
pub const RSF_TELE_SELF_TO: C2RustUnnamed_11 = 69;
pub const RSF_TELE_TO: C2RustUnnamed_11 = 68;
pub const RSF_TPORT: C2RustUnnamed_11 = 67;
pub const RSF_BLINK: C2RustUnnamed_11 = 66;
pub const RSF_HEAL_KIN: C2RustUnnamed_11 = 65;
pub const RSF_HEAL: C2RustUnnamed_11 = 64;
pub const RSF_HASTE: C2RustUnnamed_11 = 63;
pub const BI_FRACT: C2RustUnnamed_12 = 86;
pub const RSF_HOLD: C2RustUnnamed_11 = 62;
pub const RSF_SLOW: C2RustUnnamed_11 = 61;
pub const BI_RCONF: C2RustUnnamed_12 = 78;
pub const RSF_CONF: C2RustUnnamed_11 = 60;
pub const BI_RBLIND: C2RustUnnamed_12 = 77;
pub const RSF_BLIND: C2RustUnnamed_11 = 59;
pub const RSF_SCARE: C2RustUnnamed_11 = 58;
pub const BI_RNTHR: C2RustUnnamed_12 = 82;
pub const RSF_BE_NETH: C2RustUnnamed_11 = 57;
pub const BI_RELEC: C2RustUnnamed_12 = 71;
pub const BI_IELEC: C2RustUnnamed_12 = 67;
pub const RSF_BE_ELEC: C2RustUnnamed_11 = 56;
pub const RSF_MISSILE: C2RustUnnamed_11 = 55;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_12 = 118;
pub const BI_ISSTUN: C2RustUnnamed_12 = 117;
pub const BI_RSND: C2RustUnnamed_12 = 79;
pub const RSF_BO_ICE: C2RustUnnamed_11 = 54;
pub const RSF_BO_PLAS: C2RustUnnamed_11 = 53;
pub const RSF_BO_MANA: C2RustUnnamed_11 = 52;
pub const BI_CLASS: C2RustUnnamed_12 = 25;
pub const RSF_BO_WATE: C2RustUnnamed_11 = 51;
pub const RSF_BO_NETH: C2RustUnnamed_11 = 50;
pub const BI_RPOIS: C2RustUnnamed_12 = 73;
pub const BI_IPOIS: C2RustUnnamed_12 = 68;
pub const RSF_BO_POIS: C2RustUnnamed_11 = 49;
pub const BI_RCOLD: C2RustUnnamed_12 = 70;
pub const BI_ICOLD: C2RustUnnamed_12 = 66;
pub const RSF_BO_COLD: C2RustUnnamed_11 = 48;
pub const BI_RFIRE: C2RustUnnamed_12 = 69;
pub const BI_IFIRE: C2RustUnnamed_12 = 64;
pub const RSF_BO_FIRE: C2RustUnnamed_11 = 47;
pub const RSF_BO_ELEC: C2RustUnnamed_11 = 46;
pub const BI_RACID: C2RustUnnamed_12 = 72;
pub const BI_IACID: C2RustUnnamed_12 = 65;
pub const RSF_BO_ACID: C2RustUnnamed_11 = 45;
pub const RSF_WOUND: C2RustUnnamed_11 = 44;
pub const RSF_BRAIN_SMASH: C2RustUnnamed_11 = 43;
pub const RSF_MIND_BLAST: C2RustUnnamed_11 = 42;
pub const BI_MAXSP: C2RustUnnamed_12 = 31;
pub const RSF_DRAIN_MANA: C2RustUnnamed_11 = 41;
pub const RSF_STORM: C2RustUnnamed_11 = 40;
pub const BI_RLITE: C2RustUnnamed_12 = 75;
pub const RSF_BA_LIGHT: C2RustUnnamed_11 = 39;
pub const BI_RDARK: C2RustUnnamed_12 = 76;
pub const RSF_BA_DARK: C2RustUnnamed_11 = 38;
pub const RSF_BA_HOLY: C2RustUnnamed_11 = 37;
pub const RSF_BA_MANA: C2RustUnnamed_11 = 36;
pub const RSF_BA_WATE: C2RustUnnamed_11 = 35;
pub const RSF_BA_NETH: C2RustUnnamed_11 = 34;
pub const BI_RSHRD: C2RustUnnamed_12 = 80;
pub const RSF_BA_SHAR: C2RustUnnamed_11 = 33;
pub const RSF_BA_POIS: C2RustUnnamed_11 = 32;
pub const RSF_BA_COLD: C2RustUnnamed_11 = 31;
pub const RSF_BA_FIRE: C2RustUnnamed_11 = 30;
pub const RSF_BA_ELEC: C2RustUnnamed_11 = 29;
pub const RSF_BA_ACID: C2RustUnnamed_11 = 28;
pub const RSF_WEAVE: C2RustUnnamed_11 = 27;
pub const RSF_BOULDER: C2RustUnnamed_11 = 26;
pub const RSF_BR_MANA: C2RustUnnamed_11 = 25;
pub const RSF_BR_WALL: C2RustUnnamed_11 = 24;
pub const RSF_BR_PLAS: C2RustUnnamed_11 = 23;
pub const RSF_BR_SHAR: C2RustUnnamed_11 = 22;
pub const RSF_BR_GRAV: C2RustUnnamed_11 = 21;
pub const RSF_BR_INER: C2RustUnnamed_11 = 20;
pub const RSF_BR_TIME: C2RustUnnamed_11 = 19;
pub const BI_RNXUS: C2RustUnnamed_12 = 81;
pub const RSF_BR_NEXU: C2RustUnnamed_11 = 18;
pub const BI_RDIS: C2RustUnnamed_12 = 84;
pub const RSF_BR_DISE: C2RustUnnamed_11 = 17;
pub const BI_RKAOS: C2RustUnnamed_12 = 83;
pub const RSF_BR_CHAO: C2RustUnnamed_11 = 16;
pub const RSF_BR_SOUN: C2RustUnnamed_11 = 15;
pub const RSF_BR_DARK: C2RustUnnamed_11 = 14;
pub const RSF_BR_LIGHT: C2RustUnnamed_11 = 13;
pub const RSF_BR_NETH: C2RustUnnamed_11 = 12;
pub const RSF_BR_POIS: C2RustUnnamed_11 = 11;
pub const RSF_BR_COLD: C2RustUnnamed_11 = 10;
pub const RSF_BR_FIRE: C2RustUnnamed_11 = 9;
pub const RSF_BR_ELEC: C2RustUnnamed_11 = 8;
pub const RSF_BR_ACID: C2RustUnnamed_11 = 7;
pub const RSF_BOLT: C2RustUnnamed_11 = 6;
pub const RSF_ARROW: C2RustUnnamed_11 = 5;
pub const RSF_SHOT: C2RustUnnamed_11 = 4;
pub const RSF_SPIT: C2RustUnnamed_11 = 3;
pub const RSF_WHIP: C2RustUnnamed_11 = 2;
pub const RSF_SHRIEK: C2RustUnnamed_11 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub const RF_EVIL: C2RustUnnamed_10 = 61;
pub const RF_NEVER_MOVE: C2RustUnnamed_10 = 21;
pub const BI_MAXHP: C2RustUnnamed_12 = 28;
pub const RF_NEVER_BLOW: C2RustUnnamed_10 = 20;
pub const BI_ARMOR: C2RustUnnamed_12 = 133;
pub type borg_spells = ::core::ffi::c_uint;
pub const QUAKE: borg_spells = 132;
pub const FORCEFUL_BLOW: borg_spells = 131;
pub const UNHOLY_REPRIEVE: borg_spells = 130;
pub const BLOODLUST: borg_spells = 129;
pub const WEREWOLF_FORM: borg_spells = 128;
pub const VENOM: borg_spells = 127;
pub const RELENTLESS_TAUNTING: borg_spells = 126;
pub const HOWL_OF_THE_DAMNED: borg_spells = 125;
pub const MAIM_FOE: borg_spells = 124;
pub const GRIM_PURPOSE: borg_spells = 123;
pub const LEAP_INTO_BATTLE: borg_spells = 122;
pub const SHATTER_STONE: borg_spells = 121;
pub const WHIRLWIND_ATTACK: borg_spells = 120;
pub const BERSERK_STRENGTH: borg_spells = 119;
pub const SEEK_BATTLE: borg_spells = 118;
pub const BRAND_AMMUNITION: borg_spells = 117;
pub const DECOY: borg_spells = 116;
pub const CREATE_ARROWS: borg_spells = 115;
pub const COVER_TRACKS: borg_spells = 114;
pub const HIT_AND_RUN: borg_spells = 113;
pub const DETECT_STAIRS: borg_spells = 112;
pub const OBJECT_DETECTION: borg_spells = 111;
pub const SINGLE_COMBAT: borg_spells = 110;
pub const COMMAND: borg_spells = 109;
pub const CURSE: borg_spells = 108;
pub const VAMPIRE_FORM: borg_spells = 107;
pub const ZONE_OF_UNMAGIC: borg_spells = 106;
pub const POWER_SACRIFICE: borg_spells = 105;
pub const STORM_OF_DARKNESS: borg_spells = 104;
pub const FUME_OF_MORDOR: borg_spells = 103;
pub const UNLEASH_CHAOS: borg_spells = 102;
pub const GRONDS_BLOW: borg_spells = 101;
pub const ANNIHILATE: borg_spells = 100;
pub const BANISH_SPIRITS: borg_spells = 99;
pub const WARG_FORM: borg_spells = 98;
pub const DARK_SPEAR: borg_spells = 97;
pub const DISPEL_LIFE: borg_spells = 96;
pub const VAMPIRE_STRIKE: borg_spells = 95;
pub const FRIGHTEN: borg_spells = 94;
pub const DISENCHANT: borg_spells = 93;
pub const SHADOW_SHIFT: borg_spells = 92;
pub const SLEEP_EVIL: borg_spells = 91;
pub const CRUSH: borg_spells = 90;
pub const TAP_UNLIFE: borg_spells = 89;
pub const READ_MINDS: borg_spells = 88;
pub const BAT_FORM: borg_spells = 87;
pub const CREATE_DARKNESS: borg_spells = 86;
pub const NETHER_BOLT: borg_spells = 85;
pub const LIGHT_OF_VARDA: borg_spells = 84;
pub const SPEAR_OF_OROME: borg_spells = 83;
pub const HOLY_WORD: borg_spells = 82;
pub const WORD_OF_DESTRUCTION: borg_spells = 81;
pub const BANISH_EVIL: borg_spells = 80;
pub const DEMON_BANE: borg_spells = 79;
pub const GLYPH_OF_WARDING: borg_spells = 78;
pub const SMITE_EVIL: borg_spells = 77;
pub const ENCHANT_ARMOUR: borg_spells = 76;
pub const ENCHANT_WEAPON: borg_spells = 75;
pub const CLAIRVOYANCE: borg_spells = 74;
pub const RESTORATION: borg_spells = 73;
pub const HEALING: borg_spells = 72;
pub const WORD_OF_RECALL: borg_spells = 71;
pub const REMEMBRANCE: borg_spells = 70;
pub const PORTAL: borg_spells = 69;
pub const REMOVE_CURSE: borg_spells = 68;
pub const PROTECTION_FROM_EVIL: borg_spells = 67;
pub const DISPEL_EVIL: borg_spells = 66;
pub const DISPEL_UNDEAD: borg_spells = 65;
pub const SPEAR_OF_LIGHT: borg_spells = 64;
pub const ORB_OF_DRAINING: borg_spells = 63;
pub const HEROISM: borg_spells = 62;
pub const SENSE_INVISIBLE: borg_spells = 61;
pub const BLESS: borg_spells = 60;
pub const MINOR_HEALING: borg_spells = 59;
pub const DETECT_EVIL: borg_spells = 58;
pub const CALL_LIGHT: borg_spells = 57;
pub const RIVER_OF_LIGHTNING: borg_spells = 56;
pub const VOLCANIC_ERUPTION: borg_spells = 55;
pub const ICE_STORM: borg_spells = 54;
pub const RIFT: borg_spells = 53;
pub const METEOR_SWARM: borg_spells = 52;
pub const HERBAL_CURING: borg_spells = 51;
pub const RAPID_REGENERATION: borg_spells = 50;
pub const REVITALIZE: borg_spells = 49;
pub const HASTE_SELF: borg_spells = 48;
pub const TREMOR: borg_spells = 47;
pub const BEAR_FORM: borg_spells = 46;
pub const EAGLES_FLIGHT: borg_spells = 45;
pub const BECOME_PUKEL_MAN: borg_spells = 44;
pub const MASS_SLEEP: borg_spells = 43;
pub const TRANCE: borg_spells = 42;
pub const EARTH_RISING: borg_spells = 41;
pub const LIGHTNING_STRIKE: borg_spells = 40;
pub const SENSE_SURROUNDINGS: borg_spells = 39;
pub const TURN_STONE_TO_MUD: borg_spells = 38;
pub const RESIST_POISON: borg_spells = 37;
pub const CURE_POISON: borg_spells = 36;
pub const SLOW_MONSTER: borg_spells = 35;
pub const CONFUSE_MONSTER: borg_spells = 34;
pub const STINKING_CLOUD: borg_spells = 33;
pub const REMOVE_HUNGER: borg_spells = 32;
pub const FOX_FORM: borg_spells = 31;
pub const DETECT_LIFE: borg_spells = 30;
pub const MANA_STORM: borg_spells = 29;
pub const MASS_BANISHMENT: borg_spells = 28;
pub const BANISHMENT: borg_spells = 27;
pub const EXPLOSION: borg_spells = 26;
pub const SHOCK_WAVE: borg_spells = 25;
pub const THRUST_AWAY: borg_spells = 24;
pub const DIMENSION_DOOR: borg_spells = 23;
pub const DETECTION: borg_spells = 22;
pub const TELEPORT_LEVEL: borg_spells = 21;
pub const MANA_BOLT: borg_spells = 20;
pub const DOOR_CREATION: borg_spells = 19;
pub const MANA_CHANNEL: borg_spells = 18;
pub const TAP_MAGICAL_ENERGY: borg_spells = 17;
pub const RESISTANCE: borg_spells = 16;
pub const TELEPORT_OTHER: borg_spells = 15;
pub const TELEPORT_SELF: borg_spells = 14;
pub const DISABLE_TRAPS_DESTROY_DOORS: borg_spells = 13;
pub const ACID_SPRAY: borg_spells = 12;
pub const REVEAL_MONSTERS: borg_spells = 11;
pub const FROST_BOLT: borg_spells = 10;
pub const TREASURE_DETECTION: borg_spells = 9;
pub const IDENTIFY_RUNE: borg_spells = 8;
pub const RECHARGING: borg_spells = 7;
pub const FIRE_BALL: borg_spells = 6;
pub const DETECT_MONSTERS: borg_spells = 5;
pub const ELECTRIC_ARC: borg_spells = 4;
pub const PHASE_DOOR: borg_spells = 3;
pub const FIND_TRAPS_DOORS_STAIRS: borg_spells = 2;
pub const LIGHT_ROOM: borg_spells = 1;
pub const MAGIC_MISSILE: borg_spells = 0;
pub const BI_HLIFE: C2RustUnnamed_12 = 85;
pub const BI_CWIS: C2RustUnnamed_12 = 12;
pub const BI_SWIS: C2RustUnnamed_12 = 22;
pub const BI_CINT: C2RustUnnamed_12 = 11;
pub const BI_SINT: C2RustUnnamed_12 = 21;
pub const BI_CSTR: C2RustUnnamed_12 = 10;
pub const BI_CCON: C2RustUnnamed_12 = 14;
pub const BI_SCON: C2RustUnnamed_12 = 24;
pub const BI_CDEX: C2RustUnnamed_12 = 13;
pub const BI_SDEX: C2RustUnnamed_12 = 23;
pub const BI_SSTR: C2RustUnnamed_12 = 20;
pub const BI_RFEAR: C2RustUnnamed_12 = 74;
pub const BI_AFUEL: C2RustUnnamed_12 = 213;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_item {
    pub desc: [::core::ffi::c_char; 80],
    pub note: *mut ::core::ffi::c_char,
    pub kind: uint32_t,
    pub ident: bool,
    pub needs_ident: bool,
    pub aware: bool,
    pub xxxx: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
    pub iqty: uint8_t,
    pub weight: int16_t,
    pub art_idx: uint8_t,
    pub ego_idx: uint8_t,
    pub activ_idx: ::core::ffi::c_int,
    pub one_ring: bool,
    pub timeout: int16_t,
    pub to_h: int16_t,
    pub to_d: int16_t,
    pub to_a: int16_t,
    pub ac: int16_t,
    pub dd: uint8_t,
    pub ds: uint8_t,
    pub level: uint8_t,
    pub cost: int32_t,
    pub value: int32_t,
    pub cursed: bool,
    pub uncursable: bool,
    pub curses: [bool; 28],
    pub flags: [bitflag; 5],
    pub modifiers: [int16_t; 16],
    pub el_info: [element_info; 25],
    pub brands: [bool; 254],
    pub slays: [::core::ffi::c_int; 85],
}
pub const BI_FOOD: C2RustUnnamed_12 = 39;
pub const BI_DEX_INDEX: C2RustUnnamed_12 = 18;
pub const BI_GOLD: C2RustUnnamed_12 = 45;
pub const BI_SPEED: C2RustUnnamed_12 = 44;
pub const BI_CDEPTH: C2RustUnnamed_12 = 105;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_9 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_9 = 24;
pub const FEAT_LAVA: C2RustUnnamed_9 = 23;
pub const FEAT_SECRET: C2RustUnnamed_9 = 15;
pub const FEAT_HOME: C2RustUnnamed_9 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_9 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_9 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_9 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_9 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_9 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_9 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_9 = 7;
pub const FEAT_MORE: C2RustUnnamed_9 = 6;
pub const FEAT_LESS: C2RustUnnamed_9 = 5;
pub const FEAT_BROKEN: C2RustUnnamed_9 = 4;
pub const FEAT_OPEN: C2RustUnnamed_9 = 3;
pub const FEAT_FLOOR: C2RustUnnamed_9 = 1;
pub const FEAT_NONE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_10 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_10 = 83;
pub const RF_NO_CONF: C2RustUnnamed_10 = 81;
pub const RF_NO_STUN: C2RustUnnamed_10 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_10 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_10 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_10 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_10 = 76;
pub const RF_IM_WATER: C2RustUnnamed_10 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_10 = 74;
pub const RF_IM_POIS: C2RustUnnamed_10 = 73;
pub const RF_IM_COLD: C2RustUnnamed_10 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_10 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_10 = 70;
pub const RF_IM_ACID: C2RustUnnamed_10 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_10 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_10 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_10 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_10 = 65;
pub const RF_METAL: C2RustUnnamed_10 = 64;
pub const RF_NONLIVING: C2RustUnnamed_10 = 63;
pub const RF_UNDEAD: C2RustUnnamed_10 = 62;
pub const RF_ANIMAL: C2RustUnnamed_10 = 60;
pub const RF_DEMON: C2RustUnnamed_10 = 59;
pub const RF_DRAGON: C2RustUnnamed_10 = 58;
pub const RF_GIANT: C2RustUnnamed_10 = 57;
pub const RF_TROLL: C2RustUnnamed_10 = 56;
pub const RF_ORC: C2RustUnnamed_10 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_10 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_10 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_10 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_10 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_10 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_10 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_10 = 48;
pub const RF_BASH_DOOR: C2RustUnnamed_10 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_10 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_10 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_10 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_10 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_10 = 40;
pub const RF_DROP_20: C2RustUnnamed_10 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_10 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_10 = 37;
pub const RF_DROP_4: C2RustUnnamed_10 = 36;
pub const RF_DROP_3: C2RustUnnamed_10 = 35;
pub const RF_DROP_2: C2RustUnnamed_10 = 34;
pub const RF_DROP_1: C2RustUnnamed_10 = 33;
pub const RF_DROP_60: C2RustUnnamed_10 = 32;
pub const RF_DROP_40: C2RustUnnamed_10 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_10 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_10 = 29;
pub const RF_POWERFUL: C2RustUnnamed_10 = 28;
pub const RF_SPIRIT: C2RustUnnamed_10 = 27;
pub const RF_SMART: C2RustUnnamed_10 = 26;
pub const RF_STUPID: C2RustUnnamed_10 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_10 = 24;
pub const RF_RAND_50: C2RustUnnamed_10 = 23;
pub const RF_RAND_25: C2RustUnnamed_10 = 22;
pub const RF_FRIGHTENED: C2RustUnnamed_10 = 19;
pub const RF_REGENERATE: C2RustUnnamed_10 = 18;
pub const RF_UNAWARE: C2RustUnnamed_10 = 16;
pub const RF_SEASONAL: C2RustUnnamed_10 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_10 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_10 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_10 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_10 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_10 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_10 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_10 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_10 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_10 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_10 = 5;
pub const RF_FEMALE: C2RustUnnamed_10 = 4;
pub const RF_MALE: C2RustUnnamed_10 = 3;
pub const RF_QUESTOR: C2RustUnnamed_10 = 2;
pub const RF_NONE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const RSF_MAX: C2RustUnnamed_11 = 92;
pub const RSF_NONE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_12 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_12 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_12 = 263;
pub const BI_EMPTY: C2RustUnnamed_12 = 262;
pub const BI_CARRY: C2RustUnnamed_12 = 261;
pub const BI_WEIGHT: C2RustUnnamed_12 = 260;
pub const BI_DINV: C2RustUnnamed_12 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_12 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_12 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_12 = 256;
pub const BI_ADIGGER: C2RustUnnamed_12 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_12 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_12 = 253;
pub const BI_AROD2: C2RustUnnamed_12 = 252;
pub const BI_AROD1: C2RustUnnamed_12 = 251;
pub const BI_ASHROOM: C2RustUnnamed_12 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_12 = 249;
pub const BI_AHWORD: C2RustUnnamed_12 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_12 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_12 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_12 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_12 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_12 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_12 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_12 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_12 = 240;
pub const BI_ABRAND: C2RustUnnamed_12 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_12 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_12 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_12 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_12 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_12 = 234;
pub const BI_ACLW: C2RustUnnamed_12 = 233;
pub const BI_ACSW: C2RustUnnamed_12 = 232;
pub const BI_ACCW: C2RustUnnamed_12 = 231;
pub const BI_AGLYPH: C2RustUnnamed_12 = 230;
pub const BI_APFE: C2RustUnnamed_12 = 229;
pub const BI_ALITE: C2RustUnnamed_12 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_12 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_12 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_12 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_12 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_12 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_12 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_12 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_12 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_12 = 219;
pub const BI_ASPEED: C2RustUnnamed_12 = 218;
pub const BI_AID: C2RustUnnamed_12 = 217;
pub const BI_ALIFE: C2RustUnnamed_12 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_12 = 215;
pub const BI_AHEAL: C2RustUnnamed_12 = 214;
pub const BI_AESCAPE: C2RustUnnamed_12 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_12 = 211;
pub const BI_APHASE: C2RustUnnamed_12 = 210;
pub const BI_WB_POIS: C2RustUnnamed_12 = 209;
pub const BI_WB_COLD: C2RustUnnamed_12 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_12 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_12 = 206;
pub const BI_WB_ACID: C2RustUnnamed_12 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_12 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_12 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_12 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_12 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_12 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_12 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_12 = 198;
pub const BI_WS_ORC: C2RustUnnamed_12 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_12 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_12 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_12 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_12 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_12 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_12 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_12 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_12 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_12 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_12 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_12 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_12 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_12 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_12 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_12 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_12 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_12 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_12 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_12 = 178;
pub const BI_CRSSICK: C2RustUnnamed_12 = 177;
pub const BI_CRSDULL: C2RustUnnamed_12 = 176;
pub const BI_CRSVULN: C2RustUnnamed_12 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_12 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_12 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_12 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_12 = 171;
pub const BI_CRSSUND: C2RustUnnamed_12 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_12 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_12 = 168;
pub const BI_CRSPARA: C2RustUnnamed_12 = 167;
pub const BI_CRSHALU: C2RustUnnamed_12 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_12 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_12 = 164;
pub const BI_CRSTELE: C2RustUnnamed_12 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_12 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_12 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_12 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_12 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_12 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_12 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_12 = 156;
pub const BI_AMISSILES: C2RustUnnamed_12 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_12 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_12 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_12 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_12 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_12 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_12 = 149;
pub const BI_SHOTS: C2RustUnnamed_12 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_12 = 147;
pub const BI_BLOWS: C2RustUnnamed_12 = 146;
pub const BI_BART: C2RustUnnamed_12 = 145;
pub const BI_SLING: C2RustUnnamed_12 = 144;
pub const BI_BTODAM: C2RustUnnamed_12 = 143;
pub const BI_BTOHIT: C2RustUnnamed_12 = 142;
pub const BI_BID: C2RustUnnamed_12 = 141;
pub const BI_WDS: C2RustUnnamed_12 = 140;
pub const BI_WDD: C2RustUnnamed_12 = 139;
pub const BI_WID: C2RustUnnamed_12 = 138;
pub const BI_WTODAM: C2RustUnnamed_12 = 137;
pub const BI_WTOHIT: C2RustUnnamed_12 = 136;
pub const BI_TODAM: C2RustUnnamed_12 = 135;
pub const BI_TOHIT: C2RustUnnamed_12 = 134;
pub const BI_ISFIXALL: C2RustUnnamed_12 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_12 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_12 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_12 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_12 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_12 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_12 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_12 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_12 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_12 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_12 = 122;
pub const BI_ISFORGET: C2RustUnnamed_12 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_12 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_12 = 119;
pub const BI_ISCUT: C2RustUnnamed_12 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_12 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_12 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_12 = 113;
pub const BI_ISBLIND: C2RustUnnamed_12 = 112;
pub const BI_ISGORGED: C2RustUnnamed_12 = 111;
pub const BI_ISFULL: C2RustUnnamed_12 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_12 = 109;
pub const BI_ISWEAK: C2RustUnnamed_12 = 108;
pub const BI_KING: C2RustUnnamed_12 = 107;
pub const BI_SFRACT: C2RustUnnamed_12 = 104;
pub const BI_SHLIFE: C2RustUnnamed_12 = 103;
pub const BI_SRDIS: C2RustUnnamed_12 = 102;
pub const BI_SRKAOS: C2RustUnnamed_12 = 101;
pub const BI_SRNTHR: C2RustUnnamed_12 = 100;
pub const BI_SRNXUS: C2RustUnnamed_12 = 99;
pub const BI_SRSHRD: C2RustUnnamed_12 = 98;
pub const BI_SRSND: C2RustUnnamed_12 = 97;
pub const BI_SRCONF: C2RustUnnamed_12 = 96;
pub const BI_SRBLIND: C2RustUnnamed_12 = 95;
pub const BI_SRDARK: C2RustUnnamed_12 = 94;
pub const BI_SRLITE: C2RustUnnamed_12 = 93;
pub const BI_SRFEAR: C2RustUnnamed_12 = 92;
pub const BI_SRPOIS: C2RustUnnamed_12 = 91;
pub const BI_SRACID: C2RustUnnamed_12 = 90;
pub const BI_SRELEC: C2RustUnnamed_12 = 89;
pub const BI_SRCOLD: C2RustUnnamed_12 = 88;
pub const BI_SRFIRE: C2RustUnnamed_12 = 87;
pub const BI_DIG: C2RustUnnamed_12 = 63;
pub const BI_THT: C2RustUnnamed_12 = 62;
pub const BI_THB: C2RustUnnamed_12 = 61;
pub const BI_THN: C2RustUnnamed_12 = 60;
pub const BI_SRCH: C2RustUnnamed_12 = 59;
pub const BI_STL: C2RustUnnamed_12 = 58;
pub const BI_DEV: C2RustUnnamed_12 = 56;
pub const BI_DISM: C2RustUnnamed_12 = 55;
pub const BI_DISP: C2RustUnnamed_12 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_12 = 53;
pub const BI_INFRA: C2RustUnnamed_12 = 52;
pub const BI_SINV: C2RustUnnamed_12 = 51;
pub const BI_REG: C2RustUnnamed_12 = 50;
pub const BI_FEATH: C2RustUnnamed_12 = 49;
pub const BI_SDIG: C2RustUnnamed_12 = 48;
pub const BI_MOD_MOVES: C2RustUnnamed_12 = 46;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_12 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_12 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_12 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_12 = 40;
pub const BI_RECALL: C2RustUnnamed_12 = 38;
pub const BI_ESP: C2RustUnnamed_12 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_12 = 36;
pub const BI_FAIL2: C2RustUnnamed_12 = 34;
pub const BI_FAIL1: C2RustUnnamed_12 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_12 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_12 = 29;
pub const BI_LIGHT: C2RustUnnamed_12 = 26;
pub const BI_CON_INDEX: C2RustUnnamed_12 = 19;
pub const BI_WIS_INDEX: C2RustUnnamed_12 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_12 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_12 = 15;
pub const BI_ACON: C2RustUnnamed_12 = 9;
pub const BI_ADEX: C2RustUnnamed_12 = 8;
pub const BI_AWIS: C2RustUnnamed_12 = 7;
pub const BI_AINT: C2RustUnnamed_12 = 6;
pub const BI_ASTR: C2RustUnnamed_12 = 5;
pub const BI_CON: C2RustUnnamed_12 = 4;
pub const BI_DEX: C2RustUnnamed_12 = 3;
pub const BI_WIS: C2RustUnnamed_12 = 2;
pub const BI_INT: C2RustUnnamed_12 = 1;
pub const BI_STR: C2RustUnnamed_12 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_fear_region: [[uint16_t; 19]; 7] = [[0; 19]; 7];
#[no_mangle]
pub static mut borg_fear_monsters: [[uint16_t; 199]; 67] = [[0; 199]; 67];
#[no_mangle]
pub static mut borg_danger_wipe: bool = false_0 != 0;
unsafe extern "C" fn borg_danger_physical(
    mut i: ::core::ffi::c_int,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pfe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut power: ::core::ffi::c_int = 0;
    let mut chance: ::core::ffi::c_int = 0;
    let mut ac: int16_t = *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) as int16_t;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if borg.temp.shield {
        ac = (ac as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as int16_t;
    }
    if borg.temp.prot_from_evil as ::core::ffi::c_int != 0
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_EVIL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= (*r_ptr).level
    {
        pfe = 1 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 1000 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).mon_blows_max as ::core::ffi::c_int {
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut effect: *mut blow_effect = (*(*r_ptr).blow.offset(k as isize)).effect;
        let mut method: *mut blow_method = (*(*r_ptr).blow.offset(k as isize)).method;
        let mut d_dice: ::core::ffi::c_int = (*(*r_ptr).blow.offset(k as isize)).dice.dice;
        let mut d_side: ::core::ffi::c_int = (*(*r_ptr).blow.offset(k as isize)).dice.sides;
        power = 0 as ::core::ffi::c_int;
        if method.is_null() {
            break;
        }
        match borg_mon_blow_effect((*effect).name) {
            1 => {
                z = d_dice * d_side;
                if d_side < 3 as ::core::ffi::c_int && z > d_dice * d_side {
                    n += 200 as ::core::ffi::c_int;
                }
                if d_side < 3 as ::core::ffi::c_int && d_dice > 5 as ::core::ffi::c_int {
                    n += 400 as ::core::ffi::c_int;
                }
                power = 60 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            2 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0) {
                    if !borg.temp.res_pois {
                        z += 10 as ::core::ffi::c_int;
                        if pfe != 0 && !borg_attacking {
                            z /= 2 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            3 => {
                z = d_dice * d_side;
                power = 20 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0) {
                    z += 500 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            4 => {
                z = d_dice * d_side;
                z += 20 as ::core::ffi::c_int;
                power = 15 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            5 => {
                z = d_dice * d_side;
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 5 as ::core::ffi::c_int
                {
                    z += 50 as ::core::ffi::c_int;
                }
                power = 5 as ::core::ffi::c_int;
                if !(100 as ::core::ffi::c_int
                    <= adj_dex_safe[*borg
                        .trait_0
                        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
                        as usize]
                        + *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                {
                    if !(*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                        < 100 as ::core::ffi::c_int)
                    {
                        if !(*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                            > 100000 as ::core::ffi::c_int)
                        {
                            z += 5 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            6 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(100 as ::core::ffi::c_int
                    <= adj_dex_safe[*borg
                        .trait_0
                        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
                        as usize]
                        + *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                {
                    z += 5 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            7 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                    > 5 as ::core::ffi::c_int)
                {
                    z += 5 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            8 => {
                z = d_dice * d_side;
                power = 5 as ::core::ffi::c_int;
                if !((*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ))
                .timeout
                    == 0
                    || flag_has(
                        &raw mut (*borg_items.offset(
                            ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                                as isize,
                        ))
                        .flags as *mut bitflag,
                        (OF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        OF_NO_FUEL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0)
                {
                    if !(*borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize)
                        > 5 as ::core::ffi::c_int)
                    {
                        z += 5 as ::core::ffi::c_int;
                        if pfe != 0 && !borg_attacking {
                            z /= 2 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            9 => {
                if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                    z = d_dice * d_side;
                    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z += 200 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            10 => {
                if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            11 => {
                if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            12 => {
                if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                    z = d_dice * d_side;
                    power = 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            13 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    z += 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE {
                        z += 75 as ::core::ffi::c_int;
                    }
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            14 => {
                z = d_dice * d_side;
                power = 10 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0) {
                    z += 200 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE {
                        z += 200 as ::core::ffi::c_int;
                    }
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            15 => {
                z = d_dice * d_side;
                power = 10 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) != 0) {
                    z = z * 2 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            16 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0) {
                    z += 200 as ::core::ffi::c_int;
                    if pfe != 0 && !borg_attacking {
                        z /= 2 as ::core::ffi::c_int;
                    }
                }
            }
            17 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize)
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize)
                                        < 10 as ::core::ffi::c_int
                                    {
                                        z += 100 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            20 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize)
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                z += 150 as ::core::ffi::c_int;
                                if *borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize)
                                    < 10 as ::core::ffi::c_int
                                {
                                    z += 100 as ::core::ffi::c_int;
                                }
                                if pfe != 0 && !borg_attacking {
                                    z /= 2 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
            21 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_CCON as ::core::ffi::c_int as isize)
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize)
                                        < 8 as ::core::ffi::c_int
                                    {
                                        z += 100 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            18 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_CINT as ::core::ffi::c_int as isize)
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                if !borg_spell_legal(UNHOLY_REPRIEVE) {
                                    z += 150 as ::core::ffi::c_int;
                                    if borg_spell_stat() == STAT_INT as ::core::ffi::c_int {
                                        z += 50 as ::core::ffi::c_int;
                                    }
                                    if pfe != 0 && !borg_attacking {
                                        z /= 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            19 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_CWIS as ::core::ffi::c_int as isize)
                        <= 3 as ::core::ffi::c_int)
                    {
                        if !borg_spell_legal(RESTORATION) {
                            if !borg_spell_legal(REVITALIZE) {
                                z += 150 as ::core::ffi::c_int;
                                if borg_spell_stat() == STAT_WIS as ::core::ffi::c_int {
                                    z += 50 as ::core::ffi::c_int;
                                }
                                if pfe != 0 && !borg_attacking {
                                    z /= 2 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
            22 => {
                z = d_dice * d_side;
                power = 2 as ::core::ffi::c_int;
            }
            23 => {
                z = d_dice * d_side;
                z -=
                    z * (if (ac as ::core::ffi::c_int) < 150 as ::core::ffi::c_int {
                        ac as ::core::ffi::c_int
                    } else {
                        150 as ::core::ffi::c_int
                    }) / 250 as ::core::ffi::c_int;
                power = 60 as ::core::ffi::c_int;
                z += 150 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            24 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        == 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 100 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            25 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 150 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            26 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 200 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            27 => {
                z = d_dice * d_side;
                if !(*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        >= 50 as ::core::ffi::c_int)
                    {
                        if !(borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(UNHOLY_REPRIEVE) as ::core::ffi::c_int != 0
                            || borg_spell_legal(REVITALIZE) as ::core::ffi::c_int != 0)
                        {
                            z += 250 as ::core::ffi::c_int;
                            if pfe != 0 && !borg_attacking {
                                z /= 2 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            28 => {
                z = d_dice * d_side;
                z += 250 as ::core::ffi::c_int;
                if pfe != 0 && !borg_attacking {
                    z /= 2 as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        z -= *borg
            .trait_0
            .offset(BI_DAM_RED as ::core::ffi::c_int as isize);
        if z < 0 as ::core::ffi::c_int {
            z = 0 as ::core::ffi::c_int;
        }
        if !full_damage {
            if borg.near.unique as ::core::ffi::c_int != 0
                || (*r_ptr).level + power > 0 as ::core::ffi::c_int
            {
                chance = 150 as ::core::ffi::c_int
                    - (ac as ::core::ffi::c_int * 300 as ::core::ffi::c_int
                        / 4 as ::core::ffi::c_int
                        / 1 as ::core::ffi::c_int
                        + ((*r_ptr).level + power) * 3 as ::core::ffi::c_int);
            } else {
                chance = -(1 as ::core::ffi::c_int);
            }
            if chance < 5 as ::core::ffi::c_int {
                chance = 5 as ::core::ffi::c_int;
            }
            z = z * chance / 100 as ::core::ffi::c_int;
        }
        n += z;
        k += 1;
    }
    return n;
}
unsafe extern "C" fn borg_danger_spell(
    mut i: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut average: bool,
) -> ::core::ffi::c_int {
    let mut q: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pfe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut glyph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut glyph_check: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spot_x: ::core::ffi::c_int = 0;
    let mut spot_y: ::core::ffi::c_int = 0;
    let mut spot_safe: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut hp: ::core::ffi::c_int = 0;
    let mut total_dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut av: ::core::ffi::c_int = 0;
    let mut bolt: bool = false_0 != 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if borg.temp.prot_from_evil as ::core::ffi::c_int != 0
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_EVIL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= (*r_ptr).level
    {
        pfe = 1 as ::core::ffi::c_int;
    }
    if borg_on_glyph {
        glyph = 1 as ::core::ffi::c_int;
    } else if track_glyph.num != 0 {
        glyph_check = 0 as ::core::ffi::c_int;
        while glyph_check < track_glyph.num as ::core::ffi::c_int {
            if *track_glyph.y.offset(glyph_check as isize) == y
                && *track_glyph.x.offset(glyph_check as isize) == x
            {
                glyph = 1 as ::core::ffi::c_int;
            }
            glyph_check += 1;
        }
    }
    if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 1000 as ::core::ffi::c_int;
    }
    if (*kill).ranged_attack == 0 {
        return 0 as ::core::ffi::c_int;
    }
    hp = (*kill).power as ::core::ffi::c_int;
    q = 0 as ::core::ffi::c_int;
    while q < (*kill).ranged_attack as ::core::ffi::c_int {
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        match (*kill).spell[q as usize] as ::core::ffi::c_int {
            1 => {
                p += 5 as ::core::ffi::c_int;
            }
            2 => {
                if d < 3 as ::core::ffi::c_int {
                    z = 100 as ::core::ffi::c_int;
                }
            }
            3 => {
                if d < 4 as ::core::ffi::c_int {
                    z = 100 as ::core::ffi::c_int;
                }
            }
            4 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 5 as ::core::ffi::c_int;
            }
            5 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 6 as ::core::ffi::c_int;
            }
            6 => {
                z = ((*r_ptr).spell_power / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * 7 as ::core::ffi::c_int;
            }
            7 => {
                if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            8 => {
                if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            9 => {
                if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            10 => {
                if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                    z = hp / 3 as ::core::ffi::c_int;
                    if z > 1600 as ::core::ffi::c_int {
                        z = 1600 as ::core::ffi::c_int;
                    }
                    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            11 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 800 as ::core::ffi::c_int {
                    z = 800 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0 {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if borg.temp.res_pois {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if !borg.temp.res_pois {
                    if !(*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0) {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            12 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 600 as ::core::ffi::c_int {
                    z = 600 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                } else {
                    p += 125 as ::core::ffi::c_int;
                }
            }
            13 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0 {
                    z = z * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
                } else if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            14 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0 {
                    z = z * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            15 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0 {
                    z = z * 5 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                    p += 50 as ::core::ffi::c_int;
                }
            }
            16 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 600 as ::core::ffi::c_int {
                    z = 600 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0) {
                    p += 200 as ::core::ffi::c_int;
                }
            }
            17 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0) {
                    p += 500 as ::core::ffi::c_int;
                }
            }
            18 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 400 as ::core::ffi::c_int {
                    z = 400 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0) {
                    p += 100 as ::core::ffi::c_int;
                }
            }
            19 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 150 as ::core::ffi::c_int {
                    z = 150 as ::core::ffi::c_int;
                }
                p += 250 as ::core::ffi::c_int;
            }
            20 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
            }
            21 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                p += 100 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            22 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 500 as ::core::ffi::c_int {
                    z = 500 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0) {
                    p += 50 as ::core::ffi::c_int;
                }
            }
            23 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 150 as ::core::ffi::c_int {
                    z = 150 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    p += 100 as ::core::ffi::c_int;
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            24 => {
                z = hp / 6 as ::core::ffi::c_int;
                if z > 200 as ::core::ffi::c_int {
                    z = 200 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 100 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    p += 50 as ::core::ffi::c_int;
                }
            }
            25 => {
                z = hp / 3 as ::core::ffi::c_int;
                if z > 1600 as ::core::ffi::c_int {
                    z = 1600 as ::core::ffi::c_int;
                }
            }
            26 => {
                z = (1 as ::core::ffi::c_int + (*r_ptr).spell_power / 7 as ::core::ffi::c_int)
                    * 12 as ::core::ffi::c_int;
                bolt = true_0 != 0;
            }
            28 => {
                if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int + 15 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            29 => {
                if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            30 => {
                if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                    z = (*r_ptr).spell_power * 7 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            31 => {
                if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                    z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            32 => {
                z = ((*r_ptr).spell_power / 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    * 4 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0 {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if borg.temp.res_pois {
                    z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                }
                if !borg.temp.res_pois {
                    if !(*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0) {
                        p += 20 as ::core::ffi::c_int;
                    }
                }
            }
            33 => {
                z = (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0) {
                    p += 20 as ::core::ffi::c_int;
                }
            }
            34 => {
                z = (*r_ptr).spell_power * 4 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                    p += 250 as ::core::ffi::c_int;
                }
            }
            35 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    if !(*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0) {
                        p += 50 as ::core::ffi::c_int;
                        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            36 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                p += 50 as ::core::ffi::c_int;
            }
            37 => {
                z = 10 as ::core::ffi::c_int
                    + ((*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                p += 50 as ::core::ffi::c_int;
            }
            38 => {
                z = (*r_ptr).spell_power * 4 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            39 => {
                z = 10 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                        != 0)
                    {
                        p += 20 as ::core::ffi::c_int;
                        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            40 => {
                z = 70 as ::core::ffi::c_int + (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0;
                }
            }
            41 => {
                if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0 {
                    p += 100 as ::core::ffi::c_int;
                }
            }
            42 => {
                if *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                    < 100 as ::core::ffi::c_int
                {
                    z = (*r_ptr).spell_power / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                }
            }
            43 => {
                z = 12 as ::core::ffi::c_int * (15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
                p += 200 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int
                        * *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize);
                if p < 0 as ::core::ffi::c_int {
                    p = 0 as ::core::ffi::c_int;
                }
            }
            44 => {
                if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                    >= 100 as ::core::ffi::c_int)
                {
                    z = (*r_ptr).spell_power / 3 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * 5 as ::core::ffi::c_int;
                    z = z
                        * (120 as ::core::ffi::c_int
                            - *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize))
                        / 100 as ::core::ffi::c_int;
                }
            }
            45 => {
                bolt = true_0 != 0;
                if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                    z = 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_acid {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            46 => {
                if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                    bolt = true_0 != 0;
                    z = 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            47 => {
                if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                    bolt = true_0 != 0;
                    z = 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_fire {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 40 as ::core::ffi::c_int;
                }
            }
            48 => {
                if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                    bolt = true_0 != 0;
                    z = 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_cold {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    p += 20 as ::core::ffi::c_int;
                }
            }
            49 => {
                if !(*borg.trait_0.offset(BI_IPOIS as ::core::ffi::c_int as isize) != 0) {
                    z = 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_pois {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    bolt = true_0 != 0;
                }
            }
            50 => {
                bolt = true_0 != 0;
                z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                    p += 200 as ::core::ffi::c_int;
                }
            }
            51 => {
                z = 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        p += 1000 as ::core::ffi::c_int;
                    }
                    if !(*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0) {
                        p += 20 as ::core::ffi::c_int;
                        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                        {
                            p += 20 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            52 => {
                z = (*r_ptr).spell_power * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 50 as ::core::ffi::c_int;
                bolt = true_0 != 0;
                p += 50 as ::core::ffi::c_int;
            }
            53 => {
                z = 10 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                    + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 500 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            54 => {
                z = 6 as ::core::ffi::c_int * 6 as ::core::ffi::c_int + (*r_ptr).spell_power;
                bolt = true_0 != 0;
                p += 20 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                    if *borg
                        .trait_0
                        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 50 as ::core::ffi::c_int;
                    }
                    if *borg
                        .trait_0
                        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                        != 0
                    {
                        z += 1000 as ::core::ffi::c_int;
                    }
                }
            }
            55 => {
                z = 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                    + (*r_ptr).spell_power / 3 as ::core::ffi::c_int;
                bolt = true_0 != 0;
            }
            56 => {
                if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                    z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                        + (*r_ptr).spell_power * 2 as ::core::ffi::c_int
                        + 30 as ::core::ffi::c_int;
                    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    if borg.temp.res_elec {
                        z = (z + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    }
                    bolt = true_0 != 0;
                }
            }
            57 => {
                bolt = true_0 != 0;
                z = 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                    + (*r_ptr).spell_power * 2 as ::core::ffi::c_int
                    + 30 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
                    z = z * 6 as ::core::ffi::c_int / 8 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                    bolt = true_0 != 0;
                }
            }
            58 => {
                if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                    >= 100 as ::core::ffi::c_int)
                {
                    p += 10 as ::core::ffi::c_int;
                }
            }
            59 => {
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 10 as ::core::ffi::c_int;
                    }
                }
            }
            60 => {
                if !(*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 10 as ::core::ffi::c_int;
                    }
                }
            }
            61 => {
                if !(*borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 5 as ::core::ffi::c_int;
                    }
                }
            }
            62 => {
                if !(*borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                        >= 100 as ::core::ffi::c_int)
                    {
                        p += 150 as ::core::ffi::c_int;
                    }
                }
            }
            63 => {
                p += 10 as ::core::ffi::c_int;
            }
            64 => {
                p += 10 as ::core::ffi::c_int;
            }
            67 => {
                p += 10 as ::core::ffi::c_int;
            }
            68 => {
                p += 20 as ::core::ffi::c_int;
            }
            69 => {
                p += 20 as ::core::ffi::c_int;
            }
            70 => {
                p += 10 as ::core::ffi::c_int;
            }
            71 => {
                if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                    >= 100 as ::core::ffi::c_int)
                {
                    p += 50 as ::core::ffi::c_int;
                }
            }
            72 => {
                p += 5 as ::core::ffi::c_int;
            }
            73 => {
                p += 50 as ::core::ffi::c_int;
            }
            74 => {
                if !(*borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize)
                    >= 100 as ::core::ffi::c_int)
                {
                    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                        < 15 as ::core::ffi::c_int
                    {
                        p += 500 as ::core::ffi::c_int;
                    } else {
                        p += 30 as ::core::ffi::c_int;
                    }
                }
            }
            75 => {
                p += 200 as ::core::ffi::c_int;
            }
            76 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            77 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            78 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
            }
            79 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            80 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            81 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            82 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0
                    || glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += 0 as ::core::ffi::c_int;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            83 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 2 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 5 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            84 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 || borg.near.unique as ::core::ffi::c_int != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            85 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            86 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            87 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 7 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            88 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            89 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            90 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0
                    || borg_create_door as ::core::ffi::c_int != 0
                    || borg.near.unique as ::core::ffi::c_int != 0
                {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 12 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            91 => {
                spot_x = -(1 as ::core::ffi::c_int);
                while spot_x <= 1 as ::core::ffi::c_int {
                    spot_y = -(1 as ::core::ffi::c_int);
                    while spot_y <= 1 as ::core::ffi::c_int {
                        x = spot_x + (*kill).pos.x;
                        y = spot_y + (*kill).pos.y;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(x == (*kill).pos.x && y == (*kill).pos.y) {
                            if borg_feature_protected(ag) {
                                spot_safe += 1;
                                if spot_safe == 0 as ::core::ffi::c_int {
                                    spot_safe = 1 as ::core::ffi::c_int;
                                }
                                if spot_safe == 8 as ::core::ffi::c_int {
                                    spot_safe = 100 as ::core::ffi::c_int;
                                }
                                if borg.morgoth_position as ::core::ffi::c_int != 0
                                    || borg.status.anti_summon as ::core::ffi::c_int != 0
                                {
                                    spot_safe = 1000 as ::core::ffi::c_int;
                                }
                            }
                        }
                        spot_y += 1;
                    }
                    spot_x += 1;
                }
                if pfe != 0 {
                    p += (*r_ptr).spell_power;
                    p = p / spot_safe;
                } else if glyph != 0 || borg_create_door as ::core::ffi::c_int != 0 {
                    p += (*r_ptr).spell_power * 3 as ::core::ffi::c_int;
                    p = p / spot_safe;
                } else {
                    p += (*r_ptr).spell_power * 6 as ::core::ffi::c_int;
                    p = p / spot_safe;
                }
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    p = p * 75 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
                }
            }
            27 | 65 | 66 | _ => {}
        }
        if bolt as ::core::ffi::c_int == true_0
            && !borg_projectable_pure((*kill).pos.y, (*kill).pos.x, borg.c.y, borg.c.x)
        {
            z = 0 as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 75 as ::core::ffi::c_int
        {
            p = 0 as ::core::ffi::c_int;
        }
        p += z;
        if p > n {
            n = p;
        }
        total_dam += p;
        q += 1;
    }
    total_dam -= *borg
        .trait_0
        .offset(BI_DAM_RED as ::core::ffi::c_int as isize);
    if total_dam < 0 as ::core::ffi::c_int {
        total_dam = 0 as ::core::ffi::c_int;
    }
    if borg.morgoth_position as ::core::ffi::c_int != 0
        || borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        total_dam = total_dam * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    av = total_dam / (*kill).ranged_attack as ::core::ffi::c_int;
    if !average {
        return av;
    }
    if n >= av * 15 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || n > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            * 8 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
    {
        return n;
    } else {
        return av;
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_danger_one_kill(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut average: bool,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    let mut x9: ::core::ffi::c_int = (*kill).pos.x;
    let mut y9: ::core::ffi::c_int = (*kill).pos.y;
    let mut y_temp: ::core::ffi::c_int = 0;
    let mut x_temp: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut v1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_v2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_v1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fake_speed: ::core::ffi::c_int =
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize);
    let mut monster_speed: ::core::ffi::c_int = (*kill).speed as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut chance: ::core::ffi::c_int = 0;
    if (*kill).r_idx == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 100 as ::core::ffi::c_int;
    }
    if borg_tp_other_n != 0 {
        ii = 0 as ::core::ffi::c_int;
        while ii <= borg_tp_other_n as ::core::ffi::c_int {
            if i == borg_tp_other_index[ii as usize] {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
    }
    ax = if x9 > x { x9 - x } else { x - x9 };
    ay = if y9 > y { y9 - y } else { y - y9 };
    d = if ax < ay { ay } else { ax };
    if d < 1 as ::core::ffi::c_int {
        d = 1 as ::core::ffi::c_int;
    }
    if d > 20 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) >= 135 as ::core::ffi::c_int {
        fake_speed = if borg.near.unique as ::core::ffi::c_int != 0 {
            120 as ::core::ffi::c_int
        } else {
            125 as ::core::ffi::c_int
        };
    }
    if borg.temp.fast {
        fake_speed += 10 as ::core::ffi::c_int;
    }
    if borg_slow_spell {
        monster_speed -= 10 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) < 20 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
    {
        monster_speed += 3 as ::core::ffi::c_int;
    }
    e = extract_energy[fake_speed as usize] as ::core::ffi::c_int;
    t = (100 as ::core::ffi::c_int + (e - 1 as ::core::ffi::c_int)) / e;
    e = extract_energy[monster_speed as usize] as ::core::ffi::c_int;
    q = c * (t * e / 10 as ::core::ffi::c_int);
    if full_damage {
        q = (q + 9 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
    }
    if q <= 10 as ::core::ffi::c_int {
        q = 10 as ::core::ffi::c_int;
    }
    v1 = borg_danger_physical(i, full_damage);
    if borg.antibounce_count as ::core::ffi::c_int > 1200 as ::core::ffi::c_int
        || borg.time.now - borg.time.level > 25000 as borg_time
    {
        v1 = v1 / 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_BLOW as ::core::ffi::c_int,
    ) {
        v1 = 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && d > 1 as ::core::ffi::c_int
    {
        v1 = 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        v1 = v1 + v1 * 15 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
        {
            v1 = v1 + v1 * 18 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else {
            v1 = v1 + v1 * 13 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        }
    }
    if !(*kill).awake {
        let mut inc: ::core::ffi::c_int = (*r_ptr).sleep + 5 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= 25 as ::core::ffi::c_int
        {
            v1 = v1 / 2 as ::core::ffi::c_int;
        }
        v1 = v1 + v1 * inc / 100 as ::core::ffi::c_int;
    }
    if borg_sleep_spell_ii {
        if d == 1 as ::core::ffi::c_int
            && (*kill).awake as ::core::ffi::c_int != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_SLEEP as ::core::ffi::c_int,
            )
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    - 15 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 20 as ::core::ffi::c_int
                && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                    < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / 3 as ::core::ffi::c_int;
            }
        }
    }
    if borg_sleep_spell {
        if (*kill).awake as ::core::ffi::c_int != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_SLEEP as ::core::ffi::c_int,
            )
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    - 15 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 20 as ::core::ffi::c_int
                && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                    < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / (d + 2 as ::core::ffi::c_int);
            }
        }
    }
    if borg_crush_spell {
        if ((*kill).power as ::core::ffi::c_int
            * (100 as ::core::ffi::c_int - (*kill).injury as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int)
            < *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 4 as ::core::ffi::c_int
        {
            let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y9 as isize))
            .offset(x9 as isize) as *mut borg_grid;
            if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0
                && borg_cave_floor_grid(ag) as ::core::ffi::c_int != 0
            {
                v1 = 0 as ::core::ffi::c_int;
            }
        }
    }
    if (*kill).confused {
        v1 = v1 / 2 as ::core::ffi::c_int;
    }
    if (*kill).stunned {
        v1 = v1 * 10 as ::core::ffi::c_int / 13 as ::core::ffi::c_int;
    }
    if borg_confuse_spell {
        if (*kill).awake as ::core::ffi::c_int != 0
            && !(*kill).confused
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_SLEEP as ::core::ffi::c_int,
            )
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    - 15 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 20 as ::core::ffi::c_int
                && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                    < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int
            {
                v1 = 0 as ::core::ffi::c_int;
            } else {
                v1 = v1 / (d + 2 as ::core::ffi::c_int);
            }
        }
    }
    if borg_fear_mon_spell {
        v1 = 0 as ::core::ffi::c_int;
    }
    if q > 10 as ::core::ffi::c_int
        && d != 1 as ::core::ffi::c_int
        && !flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_NEVER_MOVE as ::core::ffi::c_int,
        )
    {
        b_v1 = 0 as ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while ii < 8 as ::core::ffi::c_int {
            y_temp = y9 + ddy_ddd[ii as usize] as ::core::ffi::c_int;
            x_temp = x9 + ddx_ddd[ii as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x_temp, y_temp)) {
                if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).kill != 0) {
                    if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                        as ::core::ffi::c_int
                        == FEAT_CLOSED as ::core::ffi::c_int)
                    {
                        if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                            as ::core::ffi::c_int
                            == FEAT_PERM as ::core::ffi::c_int)
                        {
                            if (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                as ::core::ffi::c_int
                                == FEAT_GRANITE as ::core::ffi::c_int
                                || ((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                    as ::core::ffi::c_int
                                    == FEAT_MAGMA as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_MAGMA_K as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ_K as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_RUBBLE as ::core::ffi::c_int)
                            {
                                if flag_has(
                                    &raw mut (*r_ptr).flags as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_PASS_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_distance(y_temp, x_temp, y, x)
                                        == 1 as ::core::ffi::c_int
                                    {
                                        b_v1 = v1;
                                    }
                                }
                                if flag_has(
                                    &raw mut (*r_ptr).flags as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_KILL_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_distance(y_temp, x_temp, y, x)
                                        == 1 as ::core::ffi::c_int
                                    {
                                        b_v1 = v1;
                                    }
                                }
                            }
                            if !(borg_distance(y_temp, x_temp, y, x) > 1 as ::core::ffi::c_int) {
                                if borg_cave_floor_bold(y_temp, x_temp) {
                                    b_v1 = v1 * (q / (d * 10 as ::core::ffi::c_int));
                                }
                            }
                        }
                    }
                }
            }
            ii += 1;
        }
        v1 = b_v1;
    }
    if q > 10 as ::core::ffi::c_int && d == 1 as ::core::ffi::c_int {
        v1 = v1 * q / 10 as ::core::ffi::c_int;
    }
    if q == 10 as ::core::ffi::c_int && d > 1 as ::core::ffi::c_int {
        v1 = 0 as ::core::ffi::c_int;
    }
    v2 = borg_danger_spell(i, y, x, d, average);
    if (*r_ptr).freq_innate == 0 && (*r_ptr).freq_spell == 0 {
        v2 = 0 as ::core::ffi::c_int;
    }
    if borg_distance(y9, x9, y, x) > (*z_info).max_range as ::core::ffi::c_int {
        v2 = 0 as ::core::ffi::c_int;
    }
    if q <= 10 as ::core::ffi::c_int
        && !borg_projectable(y9, x9, y, x)
        && !borg_projectable(y, x, y9, x9)
    {
        v2 = 0 as ::core::ffi::c_int;
    }
    if q >= 20 as ::core::ffi::c_int {
        let mut b_q: ::core::ffi::c_int = q;
        b_v2 = 0 as ::core::ffi::c_int;
        if q > 20 as ::core::ffi::c_int {
            q = 20 as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < 8 as ::core::ffi::c_int {
            y_temp = y9 + ddy_ddd[ii as usize] as ::core::ffi::c_int;
            x_temp = x9 + ddx_ddd[ii as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x_temp, y_temp)) {
                if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).kill != 0) {
                    if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                        as ::core::ffi::c_int
                        == FEAT_CLOSED as ::core::ffi::c_int)
                    {
                        if !((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                            as ::core::ffi::c_int
                            == FEAT_PERM as ::core::ffi::c_int)
                        {
                            if (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                as ::core::ffi::c_int
                                >= FEAT_GRANITE as ::core::ffi::c_int
                                || ((*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                    as ::core::ffi::c_int
                                    == FEAT_MAGMA as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_MAGMA_K as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_QUARTZ_K as ::core::ffi::c_int
                                    || (*borg_grids[y_temp as usize].offset(x_temp as isize)).feat
                                        as ::core::ffi::c_int
                                        == FEAT_RUBBLE as ::core::ffi::c_int)
                            {
                                if flag_has(
                                    &raw mut (*r_ptr).flags as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_PASS_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_projectable(y_temp, x_temp, y, x) {
                                        b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                                    }
                                }
                                if flag_has(
                                    &raw mut (*r_ptr).flags as *mut bitflag,
                                    (RF_MAX as ::core::ffi::c_int as size_t)
                                        .wrapping_add(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        )
                                        .wrapping_sub(1 as size_t)
                                        .wrapping_div(
                                            (::core::mem::size_of::<bitflag>() as size_t)
                                                .wrapping_mul(8 as size_t),
                                        ),
                                    RF_KILL_WALL as ::core::ffi::c_int,
                                ) {
                                    if borg_projectable(y_temp, x_temp, y, x) {
                                        b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                                    }
                                }
                            } else if borg_projectable(y_temp, x_temp, y, x) {
                                b_v2 = v2 * b_q / 10 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
            ii += 1;
        }
        v2 = b_v2;
    }
    if borg.antibounce_count as ::core::ffi::c_int > 1200 as ::core::ffi::c_int
        || borg.time.now - borg.time.level > 25000 as borg_time
    {
        v2 = v2 / 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        v2 = v2 + v2 * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        v2 = v2 + v2 * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if !(*kill).awake {
        let mut inc_0: ::core::ffi::c_int = (*r_ptr).sleep + 5 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= 25 as ::core::ffi::c_int
        {
            v2 = v2 / 2 as ::core::ffi::c_int;
        }
        v2 = v2 + v2 * inc_0 / 100 as ::core::ffi::c_int;
    }
    if borg_sleep_spell_ii {
        if d == 1 as ::core::ffi::c_int
            && (*kill).awake as ::core::ffi::c_int != 0
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_NO_SLEEP as ::core::ffi::c_int,
            )
            && !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            )
            && (*kill).level as ::core::ffi::c_int
                <= (if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 15 as ::core::ffi::c_int
                {
                    *borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                } else {
                    (*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        - 10 as ::core::ffi::c_int)
                        / 4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int
                })
        {
            v2 = v2 / 3 as ::core::ffi::c_int;
        }
    }
    if borg_crush_spell {
        if ((*kill).power as ::core::ffi::c_int
            * (100 as ::core::ffi::c_int - (*kill).injury as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int)
            < *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 4 as ::core::ffi::c_int
        {
            let mut ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y9 as isize))
            .offset(x9 as isize) as *mut borg_grid;
            if (*ag_0).info as ::core::ffi::c_int & BORG_VIEW != 0
                && borg_cave_floor_grid(ag_0) as ::core::ffi::c_int != 0
            {
                v1 = 0 as ::core::ffi::c_int;
            }
        }
    }
    if borg_sleep_spell {
        v2 = v2 / (d + 2 as ::core::ffi::c_int);
    }
    if (*kill).confused {
        v2 = v2 / 2 as ::core::ffi::c_int;
    }
    if (*kill).stunned {
        v2 = v2 * 10 as ::core::ffi::c_int / 13 as ::core::ffi::c_int;
    }
    if borg_confuse_spell {
        v2 = v2 / 6 as ::core::ffi::c_int;
    }
    if !full_damage {
        chance = ((*r_ptr).freq_innate + (*r_ptr).freq_spell) / 2 as ::core::ffi::c_int;
        if chance < 11 as ::core::ffi::c_int {
            v2 = v2 * 4 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else if chance < 26 as ::core::ffi::c_int {
            v2 = v2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        } else if chance < 51 as ::core::ffi::c_int {
            v2 = v2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        }
    }
    if v2 != 0 {
        r = q;
        v2 = v2 * r / 10 as ::core::ffi::c_int;
    }
    p = if v1 < v2 { v2 } else { v1 };
    if p > 2000 as ::core::ffi::c_int {
        p = 2000 as ::core::ffi::c_int;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn borg_danger(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut average: bool,
    mut full_damage: bool,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: loc = loc(x, y);
    if !square_in_bounds(cave, l) {
        return 2000 as ::core::ffi::c_int;
    }
    if !square_isvault(cave, l)
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            <= 80 as ::core::ffi::c_int
    {
        p += borg_fear_region[(y / 11 as ::core::ffi::c_int) as usize]
            [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            * c;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && p >= 300 as ::core::ffi::c_int
    {
        p = 300 as ::core::ffi::c_int;
    }
    if borg.antibounce_count as ::core::ffi::c_int <= 200 as ::core::ffi::c_int
        && !square_isvault(cave, loc(x, y))
    {
        p += borg_fear_monsters[y as usize][x as usize] as ::core::ffi::c_int * c;
    }
    full_damage = true_0 != 0;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            p += borg_danger_one_kill(y, x, c, i, average, full_damage);
        }
        i += 1;
    }
    return if p > 2000 as ::core::ffi::c_int {
        2000 as ::core::ffi::c_int
    } else {
        p
    };
}
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
