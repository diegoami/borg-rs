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
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut z_info: *mut angband_constants;
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut r_info: *mut monster_race;
    static mut borg: borg_struct;
    fn borg_cave_floor_bold(y: ::core::ffi::c_int, X: ::core::ffi::c_int) -> bool;
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut borg_kills: *mut borg_kill;
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    static mut borg_grids: [*mut borg_grid; 66];
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
pub type errr = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
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
pub const FEAT_NONE: C2RustUnnamed_7 = 0;
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
    pub temp: C2RustUnnamed_6,
    pub status: C2RustUnnamed_5,
    pub time: C2RustUnnamed_4,
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
    pub goal: C2RustUnnamed_3,
    pub init_save: C2RustUnnamed_2,
    pub mon: C2RustUnnamed_1,
    pub id: C2RustUnnamed_0,
    pub near: C2RustUnnamed,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub type borg_time = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
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
pub const BI_MAXHP: C2RustUnnamed_9 = 28;
pub const BI_CURHP: C2RustUnnamed_9 = 27;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_7 = 24;
pub type keycode_t = uint32_t;
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
pub const FEAT_GRANITE: C2RustUnnamed_7 = 21;
pub const RF_PASS_WALL: C2RustUnnamed_8 = 46;
pub const RF_MAX: C2RustUnnamed_8 = 85;
pub const FEAT_FLOOR: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_7 = 25;
pub const FEAT_LAVA: C2RustUnnamed_7 = 23;
pub const FEAT_PERM: C2RustUnnamed_7 = 22;
pub const FEAT_QUARTZ_K: C2RustUnnamed_7 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_7 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_7 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_7 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_7 = 16;
pub const FEAT_SECRET: C2RustUnnamed_7 = 15;
pub const FEAT_HOME: C2RustUnnamed_7 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_7 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_7 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_7 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_7 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_7 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_7 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_7 = 7;
pub const FEAT_MORE: C2RustUnnamed_7 = 6;
pub const FEAT_LESS: C2RustUnnamed_7 = 5;
pub const FEAT_BROKEN: C2RustUnnamed_7 = 4;
pub const FEAT_OPEN: C2RustUnnamed_7 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_7 = 2;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_8 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_8 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_8 = 82;
pub const RF_NO_CONF: C2RustUnnamed_8 = 81;
pub const RF_NO_STUN: C2RustUnnamed_8 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_8 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_8 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_8 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_8 = 76;
pub const RF_IM_WATER: C2RustUnnamed_8 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_8 = 74;
pub const RF_IM_POIS: C2RustUnnamed_8 = 73;
pub const RF_IM_COLD: C2RustUnnamed_8 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_8 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_8 = 70;
pub const RF_IM_ACID: C2RustUnnamed_8 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_8 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_8 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_8 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_8 = 65;
pub const RF_METAL: C2RustUnnamed_8 = 64;
pub const RF_NONLIVING: C2RustUnnamed_8 = 63;
pub const RF_UNDEAD: C2RustUnnamed_8 = 62;
pub const RF_EVIL: C2RustUnnamed_8 = 61;
pub const RF_ANIMAL: C2RustUnnamed_8 = 60;
pub const RF_DEMON: C2RustUnnamed_8 = 59;
pub const RF_DRAGON: C2RustUnnamed_8 = 58;
pub const RF_GIANT: C2RustUnnamed_8 = 57;
pub const RF_TROLL: C2RustUnnamed_8 = 56;
pub const RF_ORC: C2RustUnnamed_8 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_8 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_8 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_8 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_8 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_8 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_8 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_8 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_8 = 47;
pub const RF_BASH_DOOR: C2RustUnnamed_8 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_8 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_8 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_8 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_8 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_8 = 40;
pub const RF_DROP_20: C2RustUnnamed_8 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_8 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_8 = 37;
pub const RF_DROP_4: C2RustUnnamed_8 = 36;
pub const RF_DROP_3: C2RustUnnamed_8 = 35;
pub const RF_DROP_2: C2RustUnnamed_8 = 34;
pub const RF_DROP_1: C2RustUnnamed_8 = 33;
pub const RF_DROP_60: C2RustUnnamed_8 = 32;
pub const RF_DROP_40: C2RustUnnamed_8 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_8 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_8 = 29;
pub const RF_POWERFUL: C2RustUnnamed_8 = 28;
pub const RF_SPIRIT: C2RustUnnamed_8 = 27;
pub const RF_SMART: C2RustUnnamed_8 = 26;
pub const RF_STUPID: C2RustUnnamed_8 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_8 = 24;
pub const RF_RAND_50: C2RustUnnamed_8 = 23;
pub const RF_RAND_25: C2RustUnnamed_8 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_8 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_8 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_8 = 19;
pub const RF_REGENERATE: C2RustUnnamed_8 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_8 = 17;
pub const RF_UNAWARE: C2RustUnnamed_8 = 16;
pub const RF_SEASONAL: C2RustUnnamed_8 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_8 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_8 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_8 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_8 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_8 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_8 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_8 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_8 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_8 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_8 = 5;
pub const RF_FEMALE: C2RustUnnamed_8 = 4;
pub const RF_MALE: C2RustUnnamed_8 = 3;
pub const RF_QUESTOR: C2RustUnnamed_8 = 2;
pub const RF_UNIQUE: C2RustUnnamed_8 = 1;
pub const RF_NONE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_9 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_9 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_9 = 263;
pub const BI_EMPTY: C2RustUnnamed_9 = 262;
pub const BI_CARRY: C2RustUnnamed_9 = 261;
pub const BI_WEIGHT: C2RustUnnamed_9 = 260;
pub const BI_DINV: C2RustUnnamed_9 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_9 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_9 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_9 = 256;
pub const BI_ADIGGER: C2RustUnnamed_9 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_9 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_9 = 253;
pub const BI_AROD2: C2RustUnnamed_9 = 252;
pub const BI_AROD1: C2RustUnnamed_9 = 251;
pub const BI_ASHROOM: C2RustUnnamed_9 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_9 = 249;
pub const BI_AHWORD: C2RustUnnamed_9 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_9 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_9 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_9 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_9 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_9 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_9 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_9 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_9 = 240;
pub const BI_ABRAND: C2RustUnnamed_9 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_9 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_9 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_9 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_9 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_9 = 234;
pub const BI_ACLW: C2RustUnnamed_9 = 233;
pub const BI_ACSW: C2RustUnnamed_9 = 232;
pub const BI_ACCW: C2RustUnnamed_9 = 231;
pub const BI_AGLYPH: C2RustUnnamed_9 = 230;
pub const BI_APFE: C2RustUnnamed_9 = 229;
pub const BI_ALITE: C2RustUnnamed_9 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_9 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_9 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_9 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_9 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_9 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_9 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_9 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_9 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_9 = 219;
pub const BI_ASPEED: C2RustUnnamed_9 = 218;
pub const BI_AID: C2RustUnnamed_9 = 217;
pub const BI_ALIFE: C2RustUnnamed_9 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_9 = 215;
pub const BI_AHEAL: C2RustUnnamed_9 = 214;
pub const BI_AFUEL: C2RustUnnamed_9 = 213;
pub const BI_AESCAPE: C2RustUnnamed_9 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_9 = 211;
pub const BI_APHASE: C2RustUnnamed_9 = 210;
pub const BI_WB_POIS: C2RustUnnamed_9 = 209;
pub const BI_WB_COLD: C2RustUnnamed_9 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_9 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_9 = 206;
pub const BI_WB_ACID: C2RustUnnamed_9 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_9 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_9 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_9 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_9 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_9 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_9 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_9 = 198;
pub const BI_WS_ORC: C2RustUnnamed_9 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_9 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_9 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_9 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_9 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_9 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_9 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_9 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_9 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_9 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_9 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_9 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_9 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_9 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_9 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_9 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_9 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_9 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_9 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_9 = 178;
pub const BI_CRSSICK: C2RustUnnamed_9 = 177;
pub const BI_CRSDULL: C2RustUnnamed_9 = 176;
pub const BI_CRSVULN: C2RustUnnamed_9 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_9 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_9 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_9 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_9 = 171;
pub const BI_CRSSUND: C2RustUnnamed_9 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_9 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_9 = 168;
pub const BI_CRSPARA: C2RustUnnamed_9 = 167;
pub const BI_CRSHALU: C2RustUnnamed_9 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_9 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_9 = 164;
pub const BI_CRSTELE: C2RustUnnamed_9 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_9 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_9 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_9 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_9 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_9 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_9 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_9 = 156;
pub const BI_AMISSILES: C2RustUnnamed_9 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_9 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_9 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_9 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_9 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_9 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_9 = 149;
pub const BI_SHOTS: C2RustUnnamed_9 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_9 = 147;
pub const BI_BLOWS: C2RustUnnamed_9 = 146;
pub const BI_BART: C2RustUnnamed_9 = 145;
pub const BI_SLING: C2RustUnnamed_9 = 144;
pub const BI_BTODAM: C2RustUnnamed_9 = 143;
pub const BI_BTOHIT: C2RustUnnamed_9 = 142;
pub const BI_BID: C2RustUnnamed_9 = 141;
pub const BI_WDS: C2RustUnnamed_9 = 140;
pub const BI_WDD: C2RustUnnamed_9 = 139;
pub const BI_WID: C2RustUnnamed_9 = 138;
pub const BI_WTODAM: C2RustUnnamed_9 = 137;
pub const BI_WTOHIT: C2RustUnnamed_9 = 136;
pub const BI_TODAM: C2RustUnnamed_9 = 135;
pub const BI_TOHIT: C2RustUnnamed_9 = 134;
pub const BI_ARMOR: C2RustUnnamed_9 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_9 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_9 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_9 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_9 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_9 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_9 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_9 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_9 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_9 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_9 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_9 = 122;
pub const BI_ISFORGET: C2RustUnnamed_9 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_9 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_9 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_9 = 118;
pub const BI_ISSTUN: C2RustUnnamed_9 = 117;
pub const BI_ISCUT: C2RustUnnamed_9 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_9 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_9 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_9 = 113;
pub const BI_ISBLIND: C2RustUnnamed_9 = 112;
pub const BI_ISGORGED: C2RustUnnamed_9 = 111;
pub const BI_ISFULL: C2RustUnnamed_9 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_9 = 109;
pub const BI_ISWEAK: C2RustUnnamed_9 = 108;
pub const BI_KING: C2RustUnnamed_9 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_9 = 106;
pub const BI_CDEPTH: C2RustUnnamed_9 = 105;
pub const BI_SFRACT: C2RustUnnamed_9 = 104;
pub const BI_SHLIFE: C2RustUnnamed_9 = 103;
pub const BI_SRDIS: C2RustUnnamed_9 = 102;
pub const BI_SRKAOS: C2RustUnnamed_9 = 101;
pub const BI_SRNTHR: C2RustUnnamed_9 = 100;
pub const BI_SRNXUS: C2RustUnnamed_9 = 99;
pub const BI_SRSHRD: C2RustUnnamed_9 = 98;
pub const BI_SRSND: C2RustUnnamed_9 = 97;
pub const BI_SRCONF: C2RustUnnamed_9 = 96;
pub const BI_SRBLIND: C2RustUnnamed_9 = 95;
pub const BI_SRDARK: C2RustUnnamed_9 = 94;
pub const BI_SRLITE: C2RustUnnamed_9 = 93;
pub const BI_SRFEAR: C2RustUnnamed_9 = 92;
pub const BI_SRPOIS: C2RustUnnamed_9 = 91;
pub const BI_SRACID: C2RustUnnamed_9 = 90;
pub const BI_SRELEC: C2RustUnnamed_9 = 89;
pub const BI_SRCOLD: C2RustUnnamed_9 = 88;
pub const BI_SRFIRE: C2RustUnnamed_9 = 87;
pub const BI_FRACT: C2RustUnnamed_9 = 86;
pub const BI_HLIFE: C2RustUnnamed_9 = 85;
pub const BI_RDIS: C2RustUnnamed_9 = 84;
pub const BI_RKAOS: C2RustUnnamed_9 = 83;
pub const BI_RNTHR: C2RustUnnamed_9 = 82;
pub const BI_RNXUS: C2RustUnnamed_9 = 81;
pub const BI_RSHRD: C2RustUnnamed_9 = 80;
pub const BI_RSND: C2RustUnnamed_9 = 79;
pub const BI_RCONF: C2RustUnnamed_9 = 78;
pub const BI_RBLIND: C2RustUnnamed_9 = 77;
pub const BI_RDARK: C2RustUnnamed_9 = 76;
pub const BI_RLITE: C2RustUnnamed_9 = 75;
pub const BI_RFEAR: C2RustUnnamed_9 = 74;
pub const BI_RPOIS: C2RustUnnamed_9 = 73;
pub const BI_RACID: C2RustUnnamed_9 = 72;
pub const BI_RELEC: C2RustUnnamed_9 = 71;
pub const BI_RCOLD: C2RustUnnamed_9 = 70;
pub const BI_RFIRE: C2RustUnnamed_9 = 69;
pub const BI_IPOIS: C2RustUnnamed_9 = 68;
pub const BI_IELEC: C2RustUnnamed_9 = 67;
pub const BI_ICOLD: C2RustUnnamed_9 = 66;
pub const BI_IACID: C2RustUnnamed_9 = 65;
pub const BI_IFIRE: C2RustUnnamed_9 = 64;
pub const BI_DIG: C2RustUnnamed_9 = 63;
pub const BI_THT: C2RustUnnamed_9 = 62;
pub const BI_THB: C2RustUnnamed_9 = 61;
pub const BI_THN: C2RustUnnamed_9 = 60;
pub const BI_SRCH: C2RustUnnamed_9 = 59;
pub const BI_STL: C2RustUnnamed_9 = 58;
pub const BI_SAV: C2RustUnnamed_9 = 57;
pub const BI_DEV: C2RustUnnamed_9 = 56;
pub const BI_DISM: C2RustUnnamed_9 = 55;
pub const BI_DISP: C2RustUnnamed_9 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_9 = 53;
pub const BI_INFRA: C2RustUnnamed_9 = 52;
pub const BI_SINV: C2RustUnnamed_9 = 51;
pub const BI_REG: C2RustUnnamed_9 = 50;
pub const BI_FEATH: C2RustUnnamed_9 = 49;
pub const BI_SDIG: C2RustUnnamed_9 = 48;
pub const BI_DAM_RED: C2RustUnnamed_9 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_9 = 46;
pub const BI_GOLD: C2RustUnnamed_9 = 45;
pub const BI_SPEED: C2RustUnnamed_9 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_9 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_9 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_9 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_9 = 40;
pub const BI_FOOD: C2RustUnnamed_9 = 39;
pub const BI_RECALL: C2RustUnnamed_9 = 38;
pub const BI_ESP: C2RustUnnamed_9 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_9 = 36;
pub const BI_CLEVEL: C2RustUnnamed_9 = 35;
pub const BI_FAIL2: C2RustUnnamed_9 = 34;
pub const BI_FAIL1: C2RustUnnamed_9 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_9 = 32;
pub const BI_MAXSP: C2RustUnnamed_9 = 31;
pub const BI_CURSP: C2RustUnnamed_9 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_9 = 29;
pub const BI_LIGHT: C2RustUnnamed_9 = 26;
pub const BI_CLASS: C2RustUnnamed_9 = 25;
pub const BI_SCON: C2RustUnnamed_9 = 24;
pub const BI_SDEX: C2RustUnnamed_9 = 23;
pub const BI_SWIS: C2RustUnnamed_9 = 22;
pub const BI_SINT: C2RustUnnamed_9 = 21;
pub const BI_SSTR: C2RustUnnamed_9 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_9 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_9 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_9 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_9 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_9 = 15;
pub const BI_CCON: C2RustUnnamed_9 = 14;
pub const BI_CDEX: C2RustUnnamed_9 = 13;
pub const BI_CWIS: C2RustUnnamed_9 = 12;
pub const BI_CINT: C2RustUnnamed_9 = 11;
pub const BI_CSTR: C2RustUnnamed_9 = 10;
pub const BI_ACON: C2RustUnnamed_9 = 9;
pub const BI_ADEX: C2RustUnnamed_9 = 8;
pub const BI_AWIS: C2RustUnnamed_9 = 7;
pub const BI_AINT: C2RustUnnamed_9 = 6;
pub const BI_ASTR: C2RustUnnamed_9 = 5;
pub const BI_CON: C2RustUnnamed_9 = 4;
pub const BI_DEX: C2RustUnnamed_9 = 3;
pub const BI_WIS: C2RustUnnamed_9 = 2;
pub const BI_INT: C2RustUnnamed_9 = 1;
pub const BI_STR: C2RustUnnamed_9 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_target_loc: loc = loc { x: 0, y: 0 };
#[no_mangle]
pub unsafe extern "C" fn borg_los(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut qx: ::core::ffi::c_int = 0;
    let mut qy: ::core::ffi::c_int = 0;
    let mut tx: ::core::ffi::c_int = 0;
    let mut ty: ::core::ffi::c_int = 0;
    let mut f1: ::core::ffi::c_int = 0;
    let mut f2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    dy = y2 - y1;
    dx = x2 - x1;
    ay = if dy < 0 as ::core::ffi::c_int {
        -dy
    } else {
        dy
    };
    ax = if dx < 0 as ::core::ffi::c_int {
        -dx
    } else {
        dx
    };
    if ax < 2 as ::core::ffi::c_int && ay < 2 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if !square_in_bounds_fully(cave, loc(x1, y1)) {
        return false_0 != 0;
    }
    if dx == 0 {
        if dy > 0 as ::core::ffi::c_int {
            ty = y1 + 1 as ::core::ffi::c_int;
            while ty < y2 {
                if !borg_cave_floor_bold(ty, x1) {
                    return false_0 != 0;
                }
                ty += 1;
            }
        } else {
            ty = y1 - 1 as ::core::ffi::c_int;
            while ty > y2 {
                if !borg_cave_floor_bold(ty, x1) {
                    return false_0 != 0;
                }
                ty -= 1;
            }
        }
        return true_0 != 0;
    }
    if dy == 0 {
        if dx > 0 as ::core::ffi::c_int {
            tx = x1 + 1 as ::core::ffi::c_int;
            while tx < x2 {
                if !borg_cave_floor_bold(y1, tx) {
                    return false_0 != 0;
                }
                tx += 1;
            }
        } else {
            tx = x1 - 1 as ::core::ffi::c_int;
            while tx > x2 {
                if !borg_cave_floor_bold(y1, tx) {
                    return false_0 != 0;
                }
                tx -= 1;
            }
        }
        return true_0 != 0;
    }
    sx = if dx < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    sy = if dy < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    if ax == 1 as ::core::ffi::c_int {
        if ay == 2 as ::core::ffi::c_int {
            if borg_cave_floor_bold(y1 + sy, x1) {
                return true_0 != 0;
            }
        }
    } else if ay == 1 as ::core::ffi::c_int {
        if ax == 2 as ::core::ffi::c_int {
            if borg_cave_floor_bold(y1, x1 + sx) {
                return true_0 != 0;
            }
        }
    }
    f2 = ax * ay;
    f1 = f2 << 1 as ::core::ffi::c_int;
    if ax >= ay {
        qy = ay * ay;
        m = qy << 1 as ::core::ffi::c_int;
        tx = x1 + sx;
        if qy == f2 {
            ty = y1 + sy;
            qy -= f1;
        } else {
            ty = y1;
        }
        while x2 - tx != 0 {
            if !borg_cave_floor_bold(ty, tx) {
                return false_0 != 0;
            }
            qy += m;
            if qy < f2 {
                tx += sx;
            } else if qy > f2 {
                ty += sy;
                if !borg_cave_floor_bold(ty, tx) {
                    return false_0 != 0;
                }
                qy -= f1;
                tx += sx;
            } else {
                ty += sy;
                qy -= f1;
                tx += sx;
            }
        }
    } else {
        qx = ax * ax;
        m = qx << 1 as ::core::ffi::c_int;
        ty = y1 + sy;
        if qx == f2 {
            tx = x1 + sx;
            qx -= f1;
        } else {
            tx = x1;
        }
        while y2 - ty != 0 {
            if !borg_cave_floor_bold(ty, tx) {
                return false_0 != 0;
            }
            qx += m;
            if qx < f2 {
                ty += sy;
            } else if qx > f2 {
                tx += sx;
                if !borg_cave_floor_bold(ty, tx) {
                    return false_0 != 0;
                }
                qx -= f1;
                ty += sy;
            } else {
                tx += sx;
                qx -= f1;
                ty += sy;
            }
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
            || borg.morgoth_position as ::core::ffi::c_int != 0
            || borg.mon.scary as ::core::ffi::c_int != 0
        {
            if dist > 20 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int
        {
            if dist > 10 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            >= borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
        {
            if dist > (*z_info).max_range as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            {
                break;
            }
        } else if dist > 2 as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
        {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_offset_projectable(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            break;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable_pure(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            break;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 {
            return true_0 != 0;
        }
        if (*ag).kill != 0 {
            break;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_projectable_dark(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut dist: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut unknown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = y1;
    x = x1;
    dist = 0 as ::core::ffi::c_int;
    while dist <= (*z_info).max_range as ::core::ffi::c_int {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if dist != 0 && (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            unknown += 1;
        }
        if dist != 0 && !borg_cave_floor_grid(ag) {
            break;
        }
        if x == x2 && y == y2 && unknown >= 1 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        if (*ag).kill != 0 {
            break;
        }
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        dist += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inc_motion(
    mut py: *mut ::core::ffi::c_int,
    mut px: *mut ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) {
    let mut dy: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut full: ::core::ffi::c_int = 0;
    let mut half: ::core::ffi::c_int = 0;
    let mut frac: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if y2 < y1 {
        dy = y1 - y2;
        sy = -(1 as ::core::ffi::c_int);
    } else {
        dy = y2 - y1;
        sy = 1 as ::core::ffi::c_int;
    }
    if x2 < x1 {
        dx = x1 - x2;
        sx = -(1 as ::core::ffi::c_int);
    } else {
        dx = x2 - x1;
        sx = 1 as ::core::ffi::c_int;
    }
    if dy == 0 && dx == 0 {
        return;
    }
    half = dy * dx;
    full = half << 1 as ::core::ffi::c_int;
    if *px == x1 && *py == y1 {
        if dy > dx {
            *py += sy;
            return;
        } else if dx > dy {
            *px += sx;
            return;
        } else {
            *px += sx;
            *py += sy;
            return;
        }
    }
    if dy > dx {
        k = dy;
        frac = dx * dx;
        m = frac << 1 as ::core::ffi::c_int;
        y = y1 + sy;
        x = x1;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            if m != 0 {
                frac += m;
                if frac >= half {
                    x += sx;
                    frac -= full;
                }
            }
            y += sy;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    } else if dx > dy {
        frac = dy * dy;
        m = frac << 1 as ::core::ffi::c_int;
        y = y1;
        x = x1 + sx;
        k = dx;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            if m != 0 {
                frac += m;
                if frac >= half {
                    y += sy;
                    frac -= full;
                }
            }
            x += sx;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    } else {
        k = dy;
        y = y1 + sy;
        x = x1 + sx;
        loop {
            if x == *px && y == *py {
                k = 1 as ::core::ffi::c_int;
            }
            y += sy;
            x += sx;
            k -= 1;
            if k == 0 {
                *px = x;
                *py = y;
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_distance(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return distance(loc(x, y), loc(x2, y2));
}
#[no_mangle]
pub unsafe extern "C" fn borg_target(mut t: loc, mut require_monster: bool) -> bool {
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(t.y as isize)).offset(t.x as isize)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if (*ag).kill != 0 {
        borg_note(format(
            b"# Targeting %s who has %d Hit Points (%d,%d).\0" as *const u8
                as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).power as ::core::ffi::c_int,
            t.y,
            t.x,
        ));
    } else {
        borg_note(format(
            b"# Targeting location (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            t.y,
            t.x,
        ));
    }
    borg_keypress('*' as i32 as keycode_t);
    borg_keypress('p' as i32 as keycode_t);
    x1 = borg.c.x;
    y1 = borg.c.y;
    x2 = t.x;
    y2 = t.y;
    while y1 < y2 && x1 < x2 {
        borg_keypress('3' as i32 as keycode_t);
        y1 += 1;
        x1 += 1;
    }
    while y1 < y2 && x1 > x2 {
        borg_keypress('1' as i32 as keycode_t);
        y1 += 1;
        x1 -= 1;
    }
    while y1 > y2 && x1 < x2 {
        borg_keypress('9' as i32 as keycode_t);
        y1 -= 1;
        x1 += 1;
    }
    while y1 > y2 && x1 > x2 {
        borg_keypress('7' as i32 as keycode_t);
        y1 -= 1;
        x1 -= 1;
    }
    while y1 < y2 {
        borg_keypress('2' as i32 as keycode_t);
        y1 += 1;
    }
    while y1 > y2 {
        borg_keypress('8' as i32 as keycode_t);
        y1 -= 1;
    }
    while x1 < x2 {
        borg_keypress('6' as i32 as keycode_t);
        x1 += 1;
    }
    while x1 > x2 {
        borg_keypress('4' as i32 as keycode_t);
        x1 -= 1;
    }
    if require_monster {
        borg_keypress('m' as i32 as keycode_t);
    }
    borg_keypress('5' as i32 as keycode_t);
    borg_target_loc.y = t.y;
    borg_target_loc.x = t.x;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_target_unknown_wall(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut found: bool = false_0 != 0;
    let mut y_hall: bool = false_0 != 0;
    let mut x_hall: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    borg_note(format(
        b"# Perhaps wall near targetted location (%d,%d)\0" as *const u8
            as *const ::core::ffi::c_char,
        y,
        x,
    ));
    n_x = borg.c.x;
    n_y = borg.c.y;
    if (*borg_grids[(borg.c.y + 1 as ::core::ffi::c_int) as usize].offset(borg.c.x as isize)).feat
        as ::core::ffi::c_int
        == FEAT_FLOOR as ::core::ffi::c_int
        && (*borg_grids[(borg.c.y - 1 as ::core::ffi::c_int) as usize].offset(borg.c.x as isize))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
        && ((*borg_grids[(borg.c.y + 2 as ::core::ffi::c_int) as usize].offset(borg.c.x as isize))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
            || (*borg_grids[(borg.c.y - 2 as ::core::ffi::c_int) as usize]
                .offset(borg.c.x as isize))
            .feat as ::core::ffi::c_int
                == FEAT_FLOOR as ::core::ffi::c_int)
        && ((*borg_grids[borg.c.y as usize].offset((borg.c.x + 1 as ::core::ffi::c_int) as isize))
            .feat as ::core::ffi::c_int
            != FEAT_FLOOR as ::core::ffi::c_int
            && (*borg_grids[borg.c.y as usize]
                .offset((borg.c.x - 1 as ::core::ffi::c_int) as isize))
            .feat as ::core::ffi::c_int
                != FEAT_FLOOR as ::core::ffi::c_int)
    {
        x_hall = true_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset((borg.c.x + 1 as ::core::ffi::c_int) as isize)).feat
        as ::core::ffi::c_int
        == FEAT_FLOOR as ::core::ffi::c_int
        && (*borg_grids[borg.c.y as usize].offset((borg.c.x - 1 as ::core::ffi::c_int) as isize))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
        && ((*borg_grids[borg.c.y as usize].offset((borg.c.x + 2 as ::core::ffi::c_int) as isize))
            .feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
            || (*borg_grids[borg.c.y as usize]
                .offset((borg.c.x - 2 as ::core::ffi::c_int) as isize))
            .feat as ::core::ffi::c_int
                == FEAT_FLOOR as ::core::ffi::c_int)
        && ((*borg_grids[(borg.c.y + 1 as ::core::ffi::c_int) as usize].offset(borg.c.x as isize))
            .feat as ::core::ffi::c_int
            != FEAT_FLOOR as ::core::ffi::c_int
            && (*borg_grids[(borg.c.y - 1 as ::core::ffi::c_int) as usize]
                .offset(borg.c.x as isize))
            .feat as ::core::ffi::c_int
                != FEAT_FLOOR as ::core::ffi::c_int)
    {
        y_hall = true_0 != 0;
    }
    loop {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(n_y as isize))
            .offset(n_x as isize) as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
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
            RF_PASS_WALL as ::core::ffi::c_int,
        ) {
            borg_note(format(
                b"# Guessing wall (%d,%d) under ghostly target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                n_y,
                n_x,
            ));
            (*borg_grids[n_y as usize].offset(n_x as isize)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        if (*borg_grids[n_y as usize].offset(n_x as isize)).feat as ::core::ffi::c_int
            == FEAT_NONE as ::core::ffi::c_int
            && (n_y != borg.c.y || !y_hall)
            && (n_x != borg.c.x || !x_hall)
        {
            borg_note(format(
                b"# Guessing wall (%d,%d) near target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                y,
                x,
            ));
            (*borg_grids[n_y as usize].offset(n_x as isize)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        if n_x == x && n_y == y {
            borg_inc_motion(&raw mut n_y, &raw mut n_x, y, x, borg.c.y, borg.c.x);
            borg_note(format(
                b"# Guessing wall (%d,%d) near target (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
                y,
                x,
            ));
            (*borg_grids[n_y as usize].offset(n_x as isize)).feat =
                FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            found = true_0 != 0;
            return found;
        }
        borg_inc_motion(&raw mut n_y, &raw mut n_x, borg.c.y, borg.c.x, y, x);
    }
}
