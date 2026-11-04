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
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    fn borg_calculate_attack_effectiveness(attack_type: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    static mut z_info: *mut angband_constants;
    static mut borg_simulate: bool;
    static mut borg_attacking: bool;
    static mut borg: borg_struct;
    static mut borg_kills_cnt: int16_t;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    fn borg_note(what: *const ::core::ffi::c_char);
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
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
pub const BF_MAX: C2RustUnnamed_7 = 170;
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
pub const BI_SPEED: C2RustUnnamed_9 = 44;
pub const BI_CDEPTH: C2RustUnnamed_9 = 105;
pub const BI_ISCONFUSED: C2RustUnnamed_9 = 114;
pub const BI_MAXHP: C2RustUnnamed_9 = 28;
pub const BI_CURHP: C2RustUnnamed_9 = 27;
pub const FEAT_LESS: C2RustUnnamed_8 = 5;
pub const FEAT_MORE: C2RustUnnamed_8 = 6;
pub const BF_THRUST: C2RustUnnamed_7 = 1;
pub const BI_CLASS: C2RustUnnamed_9 = 25;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const BF_ACT_DRAGON_POWER: C2RustUnnamed_7 = 169;
pub const BF_ACT_DRAGON_SHINING: C2RustUnnamed_7 = 168;
pub const BF_ACT_DRAGON_BALANCE: C2RustUnnamed_7 = 167;
pub const BF_ACT_DRAGON_LAW: C2RustUnnamed_7 = 166;
pub const BF_ACT_DRAGON_CHAOS: C2RustUnnamed_7 = 165;
pub const BF_ACT_DRAGON_GOLD: C2RustUnnamed_7 = 164;
pub const BF_ACT_DRAGON_MULTIHUED: C2RustUnnamed_7 = 163;
pub const BF_ACT_DRAGON_RED: C2RustUnnamed_7 = 162;
pub const BF_ACT_DRAGON_GREEN: C2RustUnnamed_7 = 161;
pub const BF_ACT_DRAGON_BLUE: C2RustUnnamed_7 = 160;
pub const BF_ACT_RING_LIGHTNING: C2RustUnnamed_7 = 159;
pub const BF_ACT_RING_ICE: C2RustUnnamed_7 = 158;
pub const BF_ACT_RING_FIRE: C2RustUnnamed_7 = 157;
pub const BF_ACT_RING_ACID: C2RustUnnamed_7 = 156;
pub const BF_ACT_STAFF_HOLY: C2RustUnnamed_7 = 155;
pub const BF_ACT_WONDER: C2RustUnnamed_7 = 154;
pub const BF_ACT_LOSCONF: C2RustUnnamed_7 = 153;
pub const BF_ACT_LOSSLEEP: C2RustUnnamed_7 = 152;
pub const BF_ACT_LOSSLOW: C2RustUnnamed_7 = 151;
pub const BF_ACT_DISPEL_ALL: C2RustUnnamed_7 = 150;
pub const BF_ACT_DISPEL_UNDEAD: C2RustUnnamed_7 = 149;
pub const BF_ACT_DISPEL_EVIL60: C2RustUnnamed_7 = 148;
pub const BF_ACT_FIRE_BOLT3: C2RustUnnamed_7 = 147;
pub const BF_ACT_FIRE_BOLT2: C2RustUnnamed_7 = 146;
pub const BF_ACT_FIRE_BALL200: C2RustUnnamed_7 = 145;
pub const BF_ACT_FIRE_BALL2: C2RustUnnamed_7 = 144;
pub const BF_ACT_COLD_BALL2: C2RustUnnamed_7 = 143;
pub const BF_ACT_COLD_BALL160: C2RustUnnamed_7 = 142;
pub const BF_ACT_ACID_BALL: C2RustUnnamed_7 = 141;
pub const BF_ACT_ACID_BOLT3: C2RustUnnamed_7 = 140;
pub const BF_ACT_ACID_BOLT2: C2RustUnnamed_7 = 139;
pub const BF_ACT_ELEC_BALL2: C2RustUnnamed_7 = 138;
pub const BF_ACT_ELEC_BALL: C2RustUnnamed_7 = 137;
pub const BF_ACT_DRAIN_LIFE4: C2RustUnnamed_7 = 136;
pub const BF_ACT_DRAIN_LIFE3: C2RustUnnamed_7 = 135;
pub const BF_ACT_LIGHT_BEAM: C2RustUnnamed_7 = 134;
pub const BF_ACT_FEAR_MONSTER: C2RustUnnamed_7 = 133;
pub const BF_ACT_SLEEP_ALL: C2RustUnnamed_7 = 132;
pub const BF_ACT_MON_CONFUSE: C2RustUnnamed_7 = 131;
pub const BF_ACT_MON_SLOW: C2RustUnnamed_7 = 130;
pub const BF_ACT_STARLIGHT: C2RustUnnamed_7 = 129;
pub const BF_ACT_STARLIGHT2: C2RustUnnamed_7 = 128;
pub const BF_ACT_STAR_BALL: C2RustUnnamed_7 = 127;
pub const BF_ACT_MANA_BOLT: C2RustUnnamed_7 = 126;
pub const BF_ACT_DISPEL_EVIL: C2RustUnnamed_7 = 125;
pub const BF_ACT_ACID_BOLT: C2RustUnnamed_7 = 124;
pub const BF_ACT_ELEC_BOLT: C2RustUnnamed_7 = 123;
pub const BF_ACT_SLEEPII: C2RustUnnamed_7 = 122;
pub const BF_ACT_MISSILE: C2RustUnnamed_7 = 121;
pub const BF_ACT_ARROW: C2RustUnnamed_7 = 120;
pub const BF_ACT_CONFUSE2: C2RustUnnamed_7 = 119;
pub const BF_ACT_STINKING_CLOUD: C2RustUnnamed_7 = 118;
pub const BF_ACT_DRAIN_LIFE2: C2RustUnnamed_7 = 117;
pub const BF_ACT_DRAIN_LIFE1: C2RustUnnamed_7 = 116;
pub const BF_ACT_COLD_BOLT2: C2RustUnnamed_7 = 115;
pub const BF_ACT_COLD_BALL100: C2RustUnnamed_7 = 114;
pub const BF_ACT_COLD_BALL50: C2RustUnnamed_7 = 113;
pub const BF_ACT_COLD_BOLT: C2RustUnnamed_7 = 112;
pub const BF_ACT_FIRE_BALL: C2RustUnnamed_7 = 111;
pub const BF_ACT_FIRE_BALL72: C2RustUnnamed_7 = 110;
pub const BF_ACT_FIRE_BOLT: C2RustUnnamed_7 = 109;
pub const BF_DRAGON_POWER: C2RustUnnamed_7 = 108;
pub const BF_DRAGON_SHINING: C2RustUnnamed_7 = 107;
pub const BF_DRAGON_BALANCE: C2RustUnnamed_7 = 106;
pub const BF_DRAGON_LAW: C2RustUnnamed_7 = 105;
pub const BF_DRAGON_CHAOS: C2RustUnnamed_7 = 104;
pub const BF_DRAGON_GOLD: C2RustUnnamed_7 = 103;
pub const BF_DRAGON_MULTIHUED: C2RustUnnamed_7 = 102;
pub const BF_DRAGON_RED: C2RustUnnamed_7 = 101;
pub const BF_DRAGON_GREEN: C2RustUnnamed_7 = 100;
pub const BF_DRAGON_BLACK: C2RustUnnamed_7 = 99;
pub const BF_DRAGON_WHITE: C2RustUnnamed_7 = 98;
pub const BF_DRAGON_BLUE: C2RustUnnamed_7 = 97;
pub const BF_RING_LIGHTNING: C2RustUnnamed_7 = 96;
pub const BF_RING_ICE: C2RustUnnamed_7 = 95;
pub const BF_RING_FIRE: C2RustUnnamed_7 = 94;
pub const BF_RING_ACID: C2RustUnnamed_7 = 93;
pub const BF_WAND_DRAGON_FIRE: C2RustUnnamed_7 = 92;
pub const BF_WAND_DRAGON_COLD: C2RustUnnamed_7 = 91;
pub const BF_WAND_WONDER: C2RustUnnamed_7 = 90;
pub const BF_WAND_FIRE_BALL: C2RustUnnamed_7 = 89;
pub const BF_WAND_ACID_BALL: C2RustUnnamed_7 = 88;
pub const BF_WAND_COLD_BALL: C2RustUnnamed_7 = 87;
pub const BF_WAND_ELEC_BALL: C2RustUnnamed_7 = 86;
pub const BF_WAND_STINKING_CLOUD: C2RustUnnamed_7 = 85;
pub const BF_WAND_LIGHT_BEAM: C2RustUnnamed_7 = 84;
pub const BF_WAND_DRAIN_LIFE: C2RustUnnamed_7 = 83;
pub const BF_WAND_ANNIHILATION: C2RustUnnamed_7 = 82;
pub const BF_WAND_FEAR_MONSTER: C2RustUnnamed_7 = 81;
pub const BF_WAND_CONFUSE_MONSTER: C2RustUnnamed_7 = 80;
pub const BF_WAND_HOLD_MONSTER: C2RustUnnamed_7 = 79;
pub const BF_WAND_SLOW_MONSTER: C2RustUnnamed_7 = 78;
pub const BF_WAND_FIRE_BOLT: C2RustUnnamed_7 = 77;
pub const BF_WAND_ACID_BOLT: C2RustUnnamed_7 = 76;
pub const BF_WAND_COLD_BOLT: C2RustUnnamed_7 = 75;
pub const BF_WAND_ELEC_BOLT: C2RustUnnamed_7 = 74;
pub const BF_WAND_MAGIC_MISSILE: C2RustUnnamed_7 = 73;
pub const BF_WAND_UNKNOWN: C2RustUnnamed_7 = 72;
pub const BF_STAFF_HOLINESS: C2RustUnnamed_7 = 71;
pub const BF_STAFF_POWER: C2RustUnnamed_7 = 70;
pub const BF_STAFF_DISPEL_EVIL: C2RustUnnamed_7 = 69;
pub const BF_STAFF_SLOW_MONSTERS: C2RustUnnamed_7 = 68;
pub const BF_STAFF_SLEEP_MONSTERS: C2RustUnnamed_7 = 67;
pub const BF_ROD_UNKNOWN: C2RustUnnamed_7 = 66;
pub const BF_ROD_SLEEP_MONSTER: C2RustUnnamed_7 = 65;
pub const BF_ROD_SLOW_MONSTER: C2RustUnnamed_7 = 64;
pub const BF_ROD_FIRE_BALL: C2RustUnnamed_7 = 63;
pub const BF_ROD_ACID_BALL: C2RustUnnamed_7 = 62;
pub const BF_ROD_COLD_BALL: C2RustUnnamed_7 = 61;
pub const BF_ROD_ELEC_BALL: C2RustUnnamed_7 = 60;
pub const BF_ROD_DRAIN_LIFE: C2RustUnnamed_7 = 59;
pub const BF_ROD_LIGHT_BEAM: C2RustUnnamed_7 = 58;
pub const BF_ROD_FIRE_BOLT: C2RustUnnamed_7 = 57;
pub const BF_ROD_ACID_BOLT: C2RustUnnamed_7 = 56;
pub const BF_ROD_COLD_BOLT: C2RustUnnamed_7 = 55;
pub const BF_ROD_ELEC_BOLT: C2RustUnnamed_7 = 54;
pub const BF_SPELL_HOWL_OF_THE_DAMNED: C2RustUnnamed_7 = 53;
pub const BF_SPELL_MAIM_FOE: C2RustUnnamed_7 = 52;
pub const BF_SPELL_LEAP_INTO_BATTLE: C2RustUnnamed_7 = 51;
pub const BF_SPELL_WHIRLWIND_ATTACK: C2RustUnnamed_7 = 50;
pub const BF_SPELL_CURSE: C2RustUnnamed_7 = 49;
pub const BF_SPELL_STORM_OF_DARKNESS: C2RustUnnamed_7 = 48;
pub const BF_SPELL_UNLEASH_CHAOS: C2RustUnnamed_7 = 47;
pub const BF_SPELL_DARK_SPEAR: C2RustUnnamed_7 = 46;
pub const BF_PRAYER_DISPEL_LIFE: C2RustUnnamed_7 = 45;
pub const BF_SPELL_VAMPIRE_STRIKE: C2RustUnnamed_7 = 44;
pub const BF_SPELL_FRIGHTEN: C2RustUnnamed_7 = 43;
pub const BF_SPELL_DISENCHANT: C2RustUnnamed_7 = 42;
pub const BF_SPELL_SLEEP_EVIL: C2RustUnnamed_7 = 41;
pub const BF_SPELL_CRUSH: C2RustUnnamed_7 = 40;
pub const BF_SPELL_TAP_UNLIFE: C2RustUnnamed_7 = 39;
pub const BF_SPELL_NETHER_BOLT: C2RustUnnamed_7 = 38;
pub const BF_SPELL_LIGHT_OF_VARDA: C2RustUnnamed_7 = 37;
pub const BF_SPELL_SPEAR_OF_OROME: C2RustUnnamed_7 = 36;
pub const BF_SPELL_RIVER_OF_LIGHTNING: C2RustUnnamed_7 = 35;
pub const BF_SPELL_VOLCANIC_ERUPTION: C2RustUnnamed_7 = 34;
pub const BF_SPELL_EARTH_RISING: C2RustUnnamed_7 = 33;
pub const BF_SPELL_LIGHTNING_STRIKE: C2RustUnnamed_7 = 32;
pub const BF_SPELL_THRUST_AWAY: C2RustUnnamed_7 = 31;
pub const BF_SPELL_MANA_BOLT: C2RustUnnamed_7 = 30;
pub const BF_SPELL_ACID_SPRAY: C2RustUnnamed_7 = 29;
pub const BF_SPELL_ELECTRIC_ARC: C2RustUnnamed_7 = 28;
pub const BF_SPELL_ANNIHILATE: C2RustUnnamed_7 = 27;
pub const BF_PRAYER_HOLY_WORD: C2RustUnnamed_7 = 26;
pub const BF_PRAYER_DISP_SPIRITS: C2RustUnnamed_7 = 25;
pub const BF_PRAYER_DISP_EVIL: C2RustUnnamed_7 = 24;
pub const BF_PRAYER_DISP_UNDEAD: C2RustUnnamed_7 = 23;
pub const BF_PRAYER_HOLY_ORB_BALL: C2RustUnnamed_7 = 22;
pub const BF_SPELL_TRANCE: C2RustUnnamed_7 = 21;
pub const BF_SPELL_BLIND_CREATURE: C2RustUnnamed_7 = 20;
pub const BF_SPELL_MANA_STORM: C2RustUnnamed_7 = 19;
pub const BF_SPELL_RIFT: C2RustUnnamed_7 = 18;
pub const BF_SPELL_METEOR_SWARM: C2RustUnnamed_7 = 17;
pub const BF_SPELL_COLD_STORM: C2RustUnnamed_7 = 16;
pub const BF_SPELL_CONFUSE_MONSTER: C2RustUnnamed_7 = 15;
pub const BF_SPELL_EXPLOSION: C2RustUnnamed_7 = 14;
pub const BF_SPELL_SHOCK_WAVE: C2RustUnnamed_7 = 13;
pub const BF_SPELL_FIRE_BALL: C2RustUnnamed_7 = 12;
pub const BF_SPELL_SLEEP_III: C2RustUnnamed_7 = 11;
pub const BF_SPELL_SLOW_MONSTER: C2RustUnnamed_7 = 10;
pub const BF_SPELL_STONE_TO_MUD: C2RustUnnamed_7 = 9;
pub const BF_SPELL_COLD_BOLT: C2RustUnnamed_7 = 8;
pub const BF_SPELL_LIGHT_BEAM: C2RustUnnamed_7 = 7;
pub const BF_SPELL_STINK_CLOUD: C2RustUnnamed_7 = 6;
pub const BF_SPELL_MAGIC_MISSILE_RESERVE: C2RustUnnamed_7 = 5;
pub const BF_SPELL_MAGIC_MISSILE: C2RustUnnamed_7 = 4;
pub const BF_LAUNCH: C2RustUnnamed_7 = 3;
pub const BF_OBJECT: C2RustUnnamed_7 = 2;
pub const BF_REST: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_8 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_8 = 24;
pub const FEAT_LAVA: C2RustUnnamed_8 = 23;
pub const FEAT_PERM: C2RustUnnamed_8 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_8 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_8 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_8 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_8 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_8 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_8 = 16;
pub const FEAT_SECRET: C2RustUnnamed_8 = 15;
pub const FEAT_HOME: C2RustUnnamed_8 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_8 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_8 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_8 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_8 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_8 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_8 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_8 = 7;
pub const FEAT_BROKEN: C2RustUnnamed_8 = 4;
pub const FEAT_OPEN: C2RustUnnamed_8 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_8 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_8 = 1;
pub const FEAT_NONE: C2RustUnnamed_8 = 0;
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
pub const BI_ISAFRAID: C2RustUnnamed_9 = 113;
pub const BI_ISBLIND: C2RustUnnamed_9 = 112;
pub const BI_ISGORGED: C2RustUnnamed_9 = 111;
pub const BI_ISFULL: C2RustUnnamed_9 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_9 = 109;
pub const BI_ISWEAK: C2RustUnnamed_9 = 108;
pub const BI_KING: C2RustUnnamed_9 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_9 = 106;
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
pub unsafe extern "C" fn borg_munchkin_mage() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: loc = loc { x: 0, y: 0 };
    let mut c: loc = loc { x: 0, y: 0 };
    let mut b_dam: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        != FEAT_MORE as ::core::ffi::c_int
        && (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            != FEAT_LESS as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    ) > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                a.x = (*kill).pos.x;
                a.y = (*kill).pos.y;
                if !(*borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
                {
                    if (*kill).speed as ::core::ffi::c_int
                        > *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                        && distance(borg.c, a) <= 2 as ::core::ffi::c_int
                        || distance(borg.c, a) <= 1 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if borg.mon.scary {
                        return false_0 != 0;
                    }
                    c.x = (*kill).pos.x;
                    c.y = (*kill).pos.y;
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(c.y as isize))
                        .offset(c.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                            if !(distance(borg.c, c) > (*z_info).max_range as ::core::ffi::c_int) {
                                borg_temp_x[borg_temp_n as usize] = c.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = c.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < BF_MAX as ::core::ffi::c_int {
        if !(i <= 1 as ::core::ffi::c_int) {
            dam = borg_calculate_attack_effectiveness(i);
            if dam >= b_dam && dam > 0 as ::core::ffi::c_int {
                b_dam = dam;
                b_n = i;
            }
        }
        i += 1;
    }
    if b_n < 0 as ::core::ffi::c_int || b_dam <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing munchkin attack with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        b_dam,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(b_n);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_munchkin_melee() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
    {
        return false_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        != FEAT_MORE as ::core::ffi::c_int
        && (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            != FEAT_LESS as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    ) > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                if !(*borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
                {
                    if borg.mon.scary {
                        return false_0 != 0;
                    }
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                        .offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                            if !(distance(borg.c, (*kill).pos) != 1 as ::core::ffi::c_int) {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    n = borg_calculate_attack_effectiveness(BF_THRUST as ::core::ffi::c_int);
    if n <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing munchkin attack with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(BF_THRUST as ::core::ffi::c_int);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
