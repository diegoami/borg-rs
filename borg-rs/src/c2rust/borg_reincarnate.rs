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
    fn my_strcpy(
        buf: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        bufsize: size_t,
    ) -> size_t;
    fn my_strcat(
        buf: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        bufsize: size_t,
    ) -> size_t;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn option_set(opt: *const ::core::ffi::c_char, val: ::core::ffi::c_int) -> bool;
    static mut player: *mut player;
    static all_letters: [::core::ffi::c_char; 0];
    fn borg_reinit_options();
    fn borg_prepare_race_class_info();
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    fn borg_keypresses(str: *const ::core::ffi::c_char) -> errr;
    fn borg_clear_reactions();
    fn borg_notice_player();
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
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
pub type keycode_t = uint32_t;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_7 = 20;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_7 = 21;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub status: C2RustUnnamed,
    pub time: C2RustUnnamed_5,
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
    pub goal: C2RustUnnamed_4,
    pub init_save: C2RustUnnamed_3,
    pub mon: C2RustUnnamed_2,
    pub id: C2RustUnnamed_1,
    pub near: C2RustUnnamed_0,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_7 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_7 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_7 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_7 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_7 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_7 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_7 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_7 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_7 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_7 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_7 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_7 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_7 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_7 = 22;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_7 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_7 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_7 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_7 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_7 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_7 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_7 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_7 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_7 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_7 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_7 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_7 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_7 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_7 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_7 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_7 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_7 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_7 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_7 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_7 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NOSCORE_BORG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
static mut dwarf_syllable1: [*const ::core::ffi::c_char; 15] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Th\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut dwarf_syllable2: [*const ::core::ffi::c_char; 6] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut dwarf_syllable3: [*const ::core::ffi::c_char; 16] = [
    b"bur\0" as *const u8 as *const ::core::ffi::c_char,
    b"fur\0" as *const u8 as *const ::core::ffi::c_char,
    b"gan\0" as *const u8 as *const ::core::ffi::c_char,
    b"gnus\0" as *const u8 as *const ::core::ffi::c_char,
    b"gnar\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"lin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lir\0" as *const u8 as *const ::core::ffi::c_char,
    b"mli\0" as *const u8 as *const ::core::ffi::c_char,
    b"nar\0" as *const u8 as *const ::core::ffi::c_char,
    b"nus\0" as *const u8 as *const ::core::ffi::c_char,
    b"rin\0" as *const u8 as *const ::core::ffi::c_char,
    b"ran\0" as *const u8 as *const ::core::ffi::c_char,
    b"sin\0" as *const u8 as *const ::core::ffi::c_char,
    b"sil\0" as *const u8 as *const ::core::ffi::c_char,
    b"sur\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable1: [*const ::core::ffi::c_char; 34] = [
    b"Al\0" as *const u8 as *const ::core::ffi::c_char,
    b"An\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cel\0" as *const u8 as *const ::core::ffi::c_char,
    b"El\0" as *const u8 as *const ::core::ffi::c_char,
    b"Elr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Elv\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ear\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fin\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"Is\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lan\0" as *const u8 as *const ::core::ffi::c_char,
    b"Leg\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lom\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nel\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sel\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tin\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable2: [*const ::core::ffi::c_char; 17] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"adrie\0" as *const u8 as *const ::core::ffi::c_char,
    b"ara\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ebri\0" as *const u8 as *const ::core::ffi::c_char,
    b"ele\0" as *const u8 as *const ::core::ffi::c_char,
    b"ere\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"io\0" as *const u8 as *const ::core::ffi::c_char,
    b"ithra\0" as *const u8 as *const ::core::ffi::c_char,
    b"ilma\0" as *const u8 as *const ::core::ffi::c_char,
    b"il-Ga\0" as *const u8 as *const ::core::ffi::c_char,
    b"ili\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"orfi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable3: [*const ::core::ffi::c_char; 26] = [
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"las\0" as *const u8 as *const ::core::ffi::c_char,
    b"lad\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldor\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldur\0" as *const u8 as *const ::core::ffi::c_char,
    b"linde\0" as *const u8 as *const ::core::ffi::c_char,
    b"lith\0" as *const u8 as *const ::core::ffi::c_char,
    b"mir\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"nd\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndel\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndil\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndir\0" as *const u8 as *const ::core::ffi::c_char,
    b"nduil\0" as *const u8 as *const ::core::ffi::c_char,
    b"ng\0" as *const u8 as *const ::core::ffi::c_char,
    b"mbor\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"rith\0" as *const u8 as *const ::core::ffi::c_char,
    b"ril\0" as *const u8 as *const ::core::ffi::c_char,
    b"riand\0" as *const u8 as *const ::core::ffi::c_char,
    b"rion\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"thien\0" as *const u8 as *const ::core::ffi::c_char,
    b"viel\0" as *const u8 as *const ::core::ffi::c_char,
    b"wen\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyn\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable1: [*const ::core::ffi::c_char; 33] = [
    b"Aar\0" as *const u8 as *const ::core::ffi::c_char,
    b"An\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ar\0" as *const u8 as *const ::core::ffi::c_char,
    b"As\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"Han\0" as *const u8 as *const ::core::ffi::c_char,
    b"Har\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iir\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jan\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jar\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mar\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nik\0" as *const u8 as *const ::core::ffi::c_char,
    b"Os\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sam\0" as *const u8 as *const ::core::ffi::c_char,
    b"San\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ter\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tom\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ul\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable2: [*const ::core::ffi::c_char; 10] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"aa\0" as *const u8 as *const ::core::ffi::c_char,
    b"ai\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ei\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"uo\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"uu\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable3: [*const ::core::ffi::c_char; 32] = [
    b"ron\0" as *const u8 as *const ::core::ffi::c_char,
    b"re\0" as *const u8 as *const ::core::ffi::c_char,
    b"la\0" as *const u8 as *const ::core::ffi::c_char,
    b"ki\0" as *const u8 as *const ::core::ffi::c_char,
    b"kseli\0" as *const u8 as *const ::core::ffi::c_char,
    b"ksi\0" as *const u8 as *const ::core::ffi::c_char,
    b"ku\0" as *const u8 as *const ::core::ffi::c_char,
    b"ja\0" as *const u8 as *const ::core::ffi::c_char,
    b"ta\0" as *const u8 as *const ::core::ffi::c_char,
    b"na\0" as *const u8 as *const ::core::ffi::c_char,
    b"namari\0" as *const u8 as *const ::core::ffi::c_char,
    b"neli\0" as *const u8 as *const ::core::ffi::c_char,
    b"nika\0" as *const u8 as *const ::core::ffi::c_char,
    b"nikki\0" as *const u8 as *const ::core::ffi::c_char,
    b"nu\0" as *const u8 as *const ::core::ffi::c_char,
    b"nukka\0" as *const u8 as *const ::core::ffi::c_char,
    b"ka\0" as *const u8 as *const ::core::ffi::c_char,
    b"ko\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"kki\0" as *const u8 as *const ::core::ffi::c_char,
    b"rik\0" as *const u8 as *const ::core::ffi::c_char,
    b"po\0" as *const u8 as *const ::core::ffi::c_char,
    b"to\0" as *const u8 as *const ::core::ffi::c_char,
    b"pekka\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjaana\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjatta\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjukka\0" as *const u8 as *const ::core::ffi::c_char,
    b"la\0" as *const u8 as *const ::core::ffi::c_char,
    b"lla\0" as *const u8 as *const ::core::ffi::c_char,
    b"lli\0" as *const u8 as *const ::core::ffi::c_char,
    b"mo\0" as *const u8 as *const ::core::ffi::c_char,
    b"nni\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable1: [*const ::core::ffi::c_char; 22] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ber\0" as *const u8 as *const ::core::ffi::c_char,
    b"Br\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"Der\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fr\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ler\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mer\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Per\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable2: [*const ::core::ffi::c_char; 7] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"ia\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable3: [*const ::core::ffi::c_char; 25] = [
    b"bo\0" as *const u8 as *const ::core::ffi::c_char,
    b"ck\0" as *const u8 as *const ::core::ffi::c_char,
    b"decan\0" as *const u8 as *const ::core::ffi::c_char,
    b"degar\0" as *const u8 as *const ::core::ffi::c_char,
    b"do\0" as *const u8 as *const ::core::ffi::c_char,
    b"doc\0" as *const u8 as *const ::core::ffi::c_char,
    b"go\0" as *const u8 as *const ::core::ffi::c_char,
    b"grin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lba\0" as *const u8 as *const ::core::ffi::c_char,
    b"lbo\0" as *const u8 as *const ::core::ffi::c_char,
    b"lda\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldo\0" as *const u8 as *const ::core::ffi::c_char,
    b"lla\0" as *const u8 as *const ::core::ffi::c_char,
    b"ll\0" as *const u8 as *const ::core::ffi::c_char,
    b"lo\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"mwise\0" as *const u8 as *const ::core::ffi::c_char,
    b"nac\0" as *const u8 as *const ::core::ffi::c_char,
    b"noc\0" as *const u8 as *const ::core::ffi::c_char,
    b"nwise\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"ppin\0" as *const u8 as *const ::core::ffi::c_char,
    b"pper\0" as *const u8 as *const ::core::ffi::c_char,
    b"tho\0" as *const u8 as *const ::core::ffi::c_char,
    b"to\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable1: [*const ::core::ffi::c_char; 65] = [
    b"Ab\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ac\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ad\0" as *const u8 as *const ::core::ffi::c_char,
    b"Af\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ast\0" as *const u8 as *const ::core::ffi::c_char,
    b"As\0" as *const u8 as *const ::core::ffi::c_char,
    b"Al\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ar\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Br\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ch\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cad\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ed\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Et\0" as *const u8 as *const ::core::ffi::c_char,
    b"Er\0" as *const u8 as *const ::core::ffi::c_char,
    b"El\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eow\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fr\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ha\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ib\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jer\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ka\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ked\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"Loth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Leg\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mir\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nyd\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oc\0" as *const u8 as *const ::core::ffi::c_char,
    b"On\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rh\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sev\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Th\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"Wic\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable2: [*const ::core::ffi::c_char; 34] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"au\0" as *const u8 as *const ::core::ffi::c_char,
    b"ao\0" as *const u8 as *const ::core::ffi::c_char,
    b"are\0" as *const u8 as *const ::core::ffi::c_char,
    b"ale\0" as *const u8 as *const ::core::ffi::c_char,
    b"ali\0" as *const u8 as *const ::core::ffi::c_char,
    b"ay\0" as *const u8 as *const ::core::ffi::c_char,
    b"ardo\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ei\0" as *const u8 as *const ::core::ffi::c_char,
    b"ea\0" as *const u8 as *const ::core::ffi::c_char,
    b"eri\0" as *const u8 as *const ::core::ffi::c_char,
    b"era\0" as *const u8 as *const ::core::ffi::c_char,
    b"ela\0" as *const u8 as *const ::core::ffi::c_char,
    b"eli\0" as *const u8 as *const ::core::ffi::c_char,
    b"enda\0" as *const u8 as *const ::core::ffi::c_char,
    b"erra\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"ia\0" as *const u8 as *const ::core::ffi::c_char,
    b"ie\0" as *const u8 as *const ::core::ffi::c_char,
    b"ire\0" as *const u8 as *const ::core::ffi::c_char,
    b"ira\0" as *const u8 as *const ::core::ffi::c_char,
    b"ila\0" as *const u8 as *const ::core::ffi::c_char,
    b"ili\0" as *const u8 as *const ::core::ffi::c_char,
    b"ira\0" as *const u8 as *const ::core::ffi::c_char,
    b"igo\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oa\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"ore\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable3: [*const ::core::ffi::c_char; 70] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"and\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"bwyn\0" as *const u8 as *const ::core::ffi::c_char,
    b"baen\0" as *const u8 as *const ::core::ffi::c_char,
    b"bard\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"ctred\0" as *const u8 as *const ::core::ffi::c_char,
    b"cred\0" as *const u8 as *const ::core::ffi::c_char,
    b"ch\0" as *const u8 as *const ::core::ffi::c_char,
    b"can\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"dan\0" as *const u8 as *const ::core::ffi::c_char,
    b"don\0" as *const u8 as *const ::core::ffi::c_char,
    b"der\0" as *const u8 as *const ::core::ffi::c_char,
    b"dric\0" as *const u8 as *const ::core::ffi::c_char,
    b"dfrid\0" as *const u8 as *const ::core::ffi::c_char,
    b"dus\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"gord\0" as *const u8 as *const ::core::ffi::c_char,
    b"gan\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"lgrin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lith\0" as *const u8 as *const ::core::ffi::c_char,
    b"lath\0" as *const u8 as *const ::core::ffi::c_char,
    b"loth\0" as *const u8 as *const ::core::ffi::c_char,
    b"ld\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldric\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldan\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"mas\0" as *const u8 as *const ::core::ffi::c_char,
    b"mos\0" as *const u8 as *const ::core::ffi::c_char,
    b"mar\0" as *const u8 as *const ::core::ffi::c_char,
    b"mond\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"nydd\0" as *const u8 as *const ::core::ffi::c_char,
    b"nidd\0" as *const u8 as *const ::core::ffi::c_char,
    b"nnon\0" as *const u8 as *const ::core::ffi::c_char,
    b"nwan\0" as *const u8 as *const ::core::ffi::c_char,
    b"nyth\0" as *const u8 as *const ::core::ffi::c_char,
    b"nad\0" as *const u8 as *const ::core::ffi::c_char,
    b"nn\0" as *const u8 as *const ::core::ffi::c_char,
    b"nnor\0" as *const u8 as *const ::core::ffi::c_char,
    b"nd\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"ron\0" as *const u8 as *const ::core::ffi::c_char,
    b"rd\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"sh\0" as *const u8 as *const ::core::ffi::c_char,
    b"seth\0" as *const u8 as *const ::core::ffi::c_char,
    b"sean\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"th\0" as *const u8 as *const ::core::ffi::c_char,
    b"tha\0" as *const u8 as *const ::core::ffi::c_char,
    b"tlan\0" as *const u8 as *const ::core::ffi::c_char,
    b"trem\0" as *const u8 as *const ::core::ffi::c_char,
    b"tram\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"vudd\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"wan\0" as *const u8 as *const ::core::ffi::c_char,
    b"win\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyn\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyr\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyr\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyth\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable1: [*const ::core::ffi::c_char; 14] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Er\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gr\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vr\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tr\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable2: [*const ::core::ffi::c_char; 6] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oo\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"ui\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable3: [*const ::core::ffi::c_char; 24] = [
    b"dash\0" as *const u8 as *const ::core::ffi::c_char,
    b"dish\0" as *const u8 as *const ::core::ffi::c_char,
    b"dush\0" as *const u8 as *const ::core::ffi::c_char,
    b"gar\0" as *const u8 as *const ::core::ffi::c_char,
    b"gor\0" as *const u8 as *const ::core::ffi::c_char,
    b"gdush\0" as *const u8 as *const ::core::ffi::c_char,
    b"lo\0" as *const u8 as *const ::core::ffi::c_char,
    b"gdish\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"lg\0" as *const u8 as *const ::core::ffi::c_char,
    b"nak\0" as *const u8 as *const ::core::ffi::c_char,
    b"rag\0" as *const u8 as *const ::core::ffi::c_char,
    b"rbag\0" as *const u8 as *const ::core::ffi::c_char,
    b"rg\0" as *const u8 as *const ::core::ffi::c_char,
    b"rk\0" as *const u8 as *const ::core::ffi::c_char,
    b"ng\0" as *const u8 as *const ::core::ffi::c_char,
    b"nk\0" as *const u8 as *const ::core::ffi::c_char,
    b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    b"ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"urk\0" as *const u8 as *const ::core::ffi::c_char,
    b"shnak\0" as *const u8 as *const ::core::ffi::c_char,
    b"mog\0" as *const u8 as *const ::core::ffi::c_char,
    b"mak\0" as *const u8 as *const ::core::ffi::c_char,
    b"rak\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn create_random_name(
    mut race: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
    mut name_len: size_t,
) {
    if name.is_null() {
        return;
    }
    match race {
        RACE_DWARF => {
            my_strcpy(
                name,
                dwarf_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 15]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                dwarf_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                dwarf_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_ELF | RACE_HALF_ELF | RACE_HIGH_ELF => {
            my_strcpy(
                name,
                elf_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 34]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                elf_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 17]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                elf_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 26]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_GNOME => {
            my_strcpy(
                name,
                gnome_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 33]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                gnome_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                gnome_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 32]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HOBBIT => {
            my_strcpy(
                name,
                hobbit_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 22]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                hobbit_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                hobbit_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 25]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HUMAN | RACE_DUNADAN => {
            my_strcpy(
                name,
                human_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 65]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                human_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 34]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                human_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 70]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HALF_ORC | RACE_HALF_TROLL | RACE_KOBOLD => {
            my_strcpy(
                name,
                orc_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 14]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                orc_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                orc_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 24]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        _ => {
            *name.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_force_reincarnate() {
    borg_keypress('Q' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('@' as i32 as keycode_t);
    borg_reincarnate_start();
}
#[no_mangle]
pub unsafe extern "C" fn borg_reincarnate_end() {
    borg.player = player;
    borg_prepare_race_class_info();
    borg_notice_player();
    borg_clear_reactions();
    borg.status.old_depth = 128 as int16_t;
    if (*player).noscore as ::core::ffi::c_int & NOSCORE_BORG == 0 {
        (*player).noscore = ((*player).noscore as ::core::ffi::c_int | NOSCORE_BORG) as uint16_t;
    }
    borg_reinit_options();
    option_set(
        b"cheat_live\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    borg_note(b"# Done respawning\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn borg_reincarnate_start() {
    let mut full_name: [::core::ffi::c_char; 32] = [0; 32];
    let mut race: ::core::ffi::c_uint = 0;
    let mut class: ::core::ffi::c_uint = 0;
    borg.status.respawning = true_0 as int16_t;
    borg_clear_reactions();
    borg_note(b"# Respawning\0" as *const u8 as *const ::core::ffi::c_char);
    borg_keypress('n' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('n' as i32 as keycode_t);
    if *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
        != -(1 as ::core::ffi::c_int)
    {
        race = *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uint;
    } else {
        race = Rand_div(11 as uint32_t) as int32_t as ::core::ffi::c_uint;
    }
    if *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
        != -(1 as ::core::ffi::c_int)
    {
        class = *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uint;
    } else {
        class = Rand_div(9 as uint32_t) as int32_t as ::core::ffi::c_uint;
    }
    if race != (*(*player).race).ridx {
        borg_keypress(
            *(&raw const all_letters as *const ::core::ffi::c_char).offset(race as isize)
                as keycode_t,
        );
    }
    borg_keypress(KC_ENTER as keycode_t);
    if class != (*(*player).class).cidx {
        borg_keypress(
            *(&raw const all_letters as *const ::core::ffi::c_char).offset(class as isize)
                as keycode_t,
        );
    }
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    create_random_name(
        race as ::core::ffi::c_int,
        &raw mut full_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    borg_keypresses(&raw mut full_name as *mut ::core::ffi::c_char);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
}
pub const RACE_HUMAN: ::core::ffi::c_int = 0;
pub const RACE_HALF_ELF: ::core::ffi::c_int = 1;
pub const RACE_ELF: ::core::ffi::c_int = 2;
pub const RACE_HOBBIT: ::core::ffi::c_int = 3;
pub const RACE_GNOME: ::core::ffi::c_int = 4;
pub const RACE_DWARF: ::core::ffi::c_int = 5;
pub const RACE_HALF_ORC: ::core::ffi::c_int = 6;
pub const RACE_HALF_TROLL: ::core::ffi::c_int = 7;
pub const RACE_DUNADAN: ::core::ffi::c_int = 8;
pub const RACE_HIGH_ELF: ::core::ffi::c_int = 9;
pub const RACE_KOBOLD: ::core::ffi::c_int = 10;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
