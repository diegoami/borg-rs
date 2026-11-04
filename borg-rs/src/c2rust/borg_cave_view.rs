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
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut z_info: *mut angband_constants;
    static mut cave: *mut chunk;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut borg: borg_struct;
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_los(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_view_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_view_x: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut borg_view_y: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub unsafe extern "C" fn borg_forget_view() {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_view_n == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_view_n as ::core::ffi::c_int {
        let mut y: ::core::ffi::c_int = borg_view_y[i as usize] as ::core::ffi::c_int;
        let mut x: ::core::ffi::c_int = borg_view_x[i as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_VIEW) as uint16_t;
        i += 1;
    }
    borg_view_n = 0 as int16_t;
}
unsafe extern "C" fn borg_update_view_aux(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut f1: bool = false;
    let mut f2: bool = false;
    let mut v1: bool = false;
    let mut v2: bool = false;
    let mut vis1: bool = false;
    let mut vis2: bool = false;
    let mut wall: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g1_ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g2_ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    g1_ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y1 as isize)).offset(x1 as isize)
        as *mut borg_grid;
    g2_ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize)).offset(x2 as isize)
        as *mut borg_grid;
    f1 = borg_cave_floor_grid(g1_ag);
    f2 = borg_cave_floor_grid(g2_ag);
    if !f1 && !f2 {
        return true_0 != 0;
    }
    v1 = f1 as ::core::ffi::c_int != 0 && (*g1_ag).info as ::core::ffi::c_int & BORG_VIEW != 0;
    v2 = f2 as ::core::ffi::c_int != 0 && (*g2_ag).info as ::core::ffi::c_int & BORG_VIEW != 0;
    if !v1 && !v2 {
        return true_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    wall = !borg_cave_floor_grid(ag);
    vis1 = v1 as ::core::ffi::c_int != 0 && (*g1_ag).info as ::core::ffi::c_int & BORG_XTRA != 0;
    vis2 = v2 as ::core::ffi::c_int != 0 && (*g2_ag).info as ::core::ffi::c_int & BORG_XTRA != 0;
    if vis1 as ::core::ffi::c_int != 0 && vis2 as ::core::ffi::c_int != 0 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if vis1 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if v1 as ::core::ffi::c_int != 0 && v2 as ::core::ffi::c_int != 0 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if wall {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if borg_los(borg.c.y, borg.c.x, y, x) {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_update_view() {
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut se: ::core::ffi::c_int = 0;
    let mut sw: ::core::ffi::c_int = 0;
    let mut ne: ::core::ffi::c_int = 0;
    let mut nw: ::core::ffi::c_int = 0;
    let mut es: ::core::ffi::c_int = 0;
    let mut en: ::core::ffi::c_int = 0;
    let mut ws: ::core::ffi::c_int = 0;
    let mut wn: ::core::ffi::c_int = 0;
    let mut full: ::core::ffi::c_int = 0;
    let mut over: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    full = (*z_info).max_sight as ::core::ffi::c_int;
    over = (*z_info).max_sight as ::core::ffi::c_int * 3 as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    while n < borg_view_n as ::core::ffi::c_int {
        y = borg_view_y[n as usize] as ::core::ffi::c_int;
        x = borg_view_x[n as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !(0x20 as ::core::ffi::c_int)) as uint16_t;
        n += 1;
    }
    borg_view_n = 0 as int16_t;
    y = borg.c.y;
    x = borg.c.x;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
    borg_view_y[borg_view_n as usize] = y as uint8_t;
    borg_view_x[borg_view_n as usize] = x as uint8_t;
    borg_view_n += 1;
    z = full * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x + d, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x - d, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x + d, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x - d, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset(x as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = x as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    sw = d;
    se = sw;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset(x as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = x as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    nw = d;
    ne = nw;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x + d, y)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = y as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    en = d;
    es = en;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x - d, y)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = y as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    wn = d;
    ws = wn;
    n = 1 as ::core::ffi::c_int;
    while n <= over / 2 as ::core::ffi::c_int {
        let mut ypn: ::core::ffi::c_int = 0;
        let mut ymn: ::core::ffi::c_int = 0;
        let mut xpn: ::core::ffi::c_int = 0;
        let mut xmn: ::core::ffi::c_int = 0;
        z = over - n - n;
        if z > full - n {
            z = full - n;
        }
        while z + n + (n >> 1 as ::core::ffi::c_int) > full {
            z -= 1;
        }
        ypn = y + n;
        ymn = y - n;
        xpn = x + n;
        xmn = x - n;
        if ypn < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
            m = if z > 66 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - ypn {
                66 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - ypn
            } else {
                z
            };
            if xpn <= AUTO_MAX_X - 1 as ::core::ffi::c_int && n < se {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn + d,
                        xpn,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xpn - 1 as ::core::ffi::c_int,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xpn,
                    ) {
                        if n + d >= se {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                se = k + 1 as ::core::ffi::c_int;
            }
            if xmn >= 0 as ::core::ffi::c_int && n < sw {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn + d,
                        xmn,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xmn + 1 as ::core::ffi::c_int,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xmn,
                    ) {
                        if n + d >= sw {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                sw = k + 1 as ::core::ffi::c_int;
            }
        }
        if ymn > 0 as ::core::ffi::c_int {
            m = if z > ymn { ymn } else { z };
            if xpn <= AUTO_MAX_X - 1 as ::core::ffi::c_int && n < ne {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn - d,
                        xpn,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xpn - 1 as ::core::ffi::c_int,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xpn,
                    ) {
                        if n + d >= ne {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                ne = k + 1 as ::core::ffi::c_int;
            }
            if xmn >= 0 as ::core::ffi::c_int && n < nw {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn - d,
                        xmn,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xmn + 1 as ::core::ffi::c_int,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xmn,
                    ) {
                        if n + d >= nw {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                nw = k + 1 as ::core::ffi::c_int;
            }
        }
        if xpn < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            m = if z > 198 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - xpn {
                198 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - xpn
            } else {
                z
            };
            if ypn <= AUTO_MAX_Y - 1 as ::core::ffi::c_int && n < es {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn,
                        xpn + d,
                        ypn - 1 as ::core::ffi::c_int,
                        xpn + d - 1 as ::core::ffi::c_int,
                        ypn,
                        xpn + d - 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= es {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                es = k + 1 as ::core::ffi::c_int;
            }
            if ymn >= 0 as ::core::ffi::c_int && n < en {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn,
                        xpn + d,
                        ymn + 1 as ::core::ffi::c_int,
                        xpn + d - 1 as ::core::ffi::c_int,
                        ymn,
                        xpn + d - 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= en {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                en = k + 1 as ::core::ffi::c_int;
            }
        }
        if xmn > 0 as ::core::ffi::c_int {
            m = if z > xmn { xmn } else { z };
            if ypn <= AUTO_MAX_Y - 1 as ::core::ffi::c_int && n < ws {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn,
                        xmn - d,
                        ypn - 1 as ::core::ffi::c_int,
                        xmn - d + 1 as ::core::ffi::c_int,
                        ypn,
                        xmn - d + 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= ws {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                ws = k + 1 as ::core::ffi::c_int;
            }
            if ymn >= 0 as ::core::ffi::c_int && n < wn {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn,
                        xmn - d,
                        ymn + 1 as ::core::ffi::c_int,
                        xmn - d + 1 as ::core::ffi::c_int,
                        ymn,
                        xmn - d + 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= wn {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                wn = k + 1 as ::core::ffi::c_int;
            }
        }
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < borg_view_n as ::core::ffi::c_int {
        y = borg_view_y[n as usize] as ::core::ffi::c_int;
        x = borg_view_x[n as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_XTRA) as uint16_t;
        n += 1;
    }
}
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_XTRA: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
