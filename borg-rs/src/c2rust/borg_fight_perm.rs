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
    static mut borg_simulate: bool;
    static mut borg: borg_struct;
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_slot(tval: ::core::ffi::c_int, sval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn borg_note(what: *const ::core::ffi::c_char);
    static mut act_rage_bless_resist: ::core::ffi::c_int;
    static mut act_resist_all: ::core::ffi::c_int;
    fn borg_quaff_potion(sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    static mut sv_potion_berserk: ::core::ffi::c_int;
    fn borg_heroism_level() -> ::core::ffi::c_int;
    fn borg_get_spell_power(spell: borg_spells) -> ::core::ffi::c_int;
    fn borg_spell(spell: borg_spells) -> bool;
    fn borg_spell_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_spell_okay_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
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
pub const TV_MAX: C2RustUnnamed = 36;
pub const TV_GOLD: C2RustUnnamed = 35;
pub const TV_OTHER_BOOK: C2RustUnnamed = 34;
pub const TV_SHADOW_BOOK: C2RustUnnamed = 33;
pub const TV_NATURE_BOOK: C2RustUnnamed = 32;
pub const TV_PRAYER_BOOK: C2RustUnnamed = 31;
pub const TV_MAGIC_BOOK: C2RustUnnamed = 30;
pub const TV_MUSHROOM: C2RustUnnamed = 29;
pub const TV_FOOD: C2RustUnnamed = 28;
pub const TV_FLASK: C2RustUnnamed = 27;
pub const TV_POTION: C2RustUnnamed = 26;
pub const TV_SCROLL: C2RustUnnamed = 25;
pub const TV_ROD: C2RustUnnamed = 24;
pub const TV_WAND: C2RustUnnamed = 23;
pub const TV_STAFF: C2RustUnnamed = 22;
pub const TV_RING: C2RustUnnamed = 21;
pub const TV_AMULET: C2RustUnnamed = 20;
pub const TV_LIGHT: C2RustUnnamed = 19;
pub const TV_DRAG_ARMOR: C2RustUnnamed = 18;
pub const TV_HARD_ARMOR: C2RustUnnamed = 17;
pub const TV_SOFT_ARMOR: C2RustUnnamed = 16;
pub const TV_CLOAK: C2RustUnnamed = 15;
pub const TV_SHIELD: C2RustUnnamed = 14;
pub const TV_CROWN: C2RustUnnamed = 13;
pub const TV_HELM: C2RustUnnamed = 12;
pub const TV_GLOVES: C2RustUnnamed = 11;
pub const TV_BOOTS: C2RustUnnamed = 10;
pub const TV_SWORD: C2RustUnnamed = 9;
pub const TV_POLEARM: C2RustUnnamed = 8;
pub const TV_HAFTED: C2RustUnnamed = 7;
pub const TV_DIGGING: C2RustUnnamed = 6;
pub const TV_BOW: C2RustUnnamed = 5;
pub const TV_BOLT: C2RustUnnamed = 4;
pub const TV_ARROW: C2RustUnnamed = 3;
pub const TV_SHOT: C2RustUnnamed = 2;
pub const TV_CHEST: C2RustUnnamed = 1;
pub const TV_NULL: C2RustUnnamed = 0;
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
    pub temp: C2RustUnnamed_7,
    pub status: C2RustUnnamed_6,
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
pub struct C2RustUnnamed_7 {
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
pub const BI_LIGHT: C2RustUnnamed_8 = 26;
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
pub const BI_SINV: C2RustUnnamed_8 = 51;
pub const BI_ISCONFUSED: C2RustUnnamed_8 = 114;
pub const BI_ISBLIND: C2RustUnnamed_8 = 112;
pub const BP_SEE_INV: C2RustUnnamed_9 = 14;
pub const BP_GLYPH: C2RustUnnamed_9 = 13;
pub const BI_CURSP: C2RustUnnamed_8 = 30;
pub const BI_MAXHP: C2RustUnnamed_8 = 28;
pub const BI_ISFORGET: C2RustUnnamed_8 = 121;
pub const BP_REGEN: C2RustUnnamed_9 = 12;
pub const BI_WB_POIS: C2RustUnnamed_8 = 209;
pub const BP_VENOM: C2RustUnnamed_9 = 11;
pub const BI_WS_EVIL: C2RustUnnamed_8 = 194;
pub const BP_SMITE_EVIL: C2RustUnnamed_9 = 10;
pub const BP_BERSERK_POTION: C2RustUnnamed_9 = 9;
pub const BP_BERSERK: C2RustUnnamed_9 = 8;
pub const BI_CLEVEL: C2RustUnnamed_8 = 35;
pub const BP_HERO: C2RustUnnamed_9 = 7;
pub const BP_FASTCAST: C2RustUnnamed_9 = 6;
pub const BP_BLESS: C2RustUnnamed_9 = 2;
pub const BP_RESIST_P: C2RustUnnamed_9 = 5;
pub const BP_RESIST_ALL_COLLUIN: C2RustUnnamed_9 = 4;
pub const BP_RESIST_ALL: C2RustUnnamed_9 = 3;
pub const BP_PROT_FROM_EVIL: C2RustUnnamed_9 = 1;
pub const BP_SPEED: C2RustUnnamed_9 = 0;
pub const BP_MAX: C2RustUnnamed_9 = 15;
pub const BI_MAXSP: C2RustUnnamed_8 = 31;
pub const BI_CDEPTH: C2RustUnnamed_8 = 105;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_8 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_8 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_8 = 263;
pub const BI_EMPTY: C2RustUnnamed_8 = 262;
pub const BI_CARRY: C2RustUnnamed_8 = 261;
pub const BI_WEIGHT: C2RustUnnamed_8 = 260;
pub const BI_DINV: C2RustUnnamed_8 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_8 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_8 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_8 = 256;
pub const BI_ADIGGER: C2RustUnnamed_8 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_8 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_8 = 253;
pub const BI_AROD2: C2RustUnnamed_8 = 252;
pub const BI_AROD1: C2RustUnnamed_8 = 251;
pub const BI_ASHROOM: C2RustUnnamed_8 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_8 = 249;
pub const BI_AHWORD: C2RustUnnamed_8 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_8 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_8 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_8 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_8 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_8 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_8 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_8 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_8 = 240;
pub const BI_ABRAND: C2RustUnnamed_8 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_8 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_8 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_8 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_8 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_8 = 234;
pub const BI_ACLW: C2RustUnnamed_8 = 233;
pub const BI_ACSW: C2RustUnnamed_8 = 232;
pub const BI_ACCW: C2RustUnnamed_8 = 231;
pub const BI_AGLYPH: C2RustUnnamed_8 = 230;
pub const BI_APFE: C2RustUnnamed_8 = 229;
pub const BI_ALITE: C2RustUnnamed_8 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_8 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_8 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_8 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_8 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_8 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_8 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_8 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_8 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_8 = 219;
pub const BI_ASPEED: C2RustUnnamed_8 = 218;
pub const BI_AID: C2RustUnnamed_8 = 217;
pub const BI_ALIFE: C2RustUnnamed_8 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_8 = 215;
pub const BI_AHEAL: C2RustUnnamed_8 = 214;
pub const BI_AFUEL: C2RustUnnamed_8 = 213;
pub const BI_AESCAPE: C2RustUnnamed_8 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_8 = 211;
pub const BI_APHASE: C2RustUnnamed_8 = 210;
pub const BI_WB_COLD: C2RustUnnamed_8 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_8 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_8 = 206;
pub const BI_WB_ACID: C2RustUnnamed_8 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_8 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_8 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_8 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_8 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_8 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_8 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_8 = 198;
pub const BI_WS_ORC: C2RustUnnamed_8 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_8 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_8 = 195;
pub const BI_WS_ANIMAL: C2RustUnnamed_8 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_8 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_8 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_8 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_8 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_8 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_8 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_8 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_8 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_8 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_8 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_8 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_8 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_8 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_8 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_8 = 178;
pub const BI_CRSSICK: C2RustUnnamed_8 = 177;
pub const BI_CRSDULL: C2RustUnnamed_8 = 176;
pub const BI_CRSVULN: C2RustUnnamed_8 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_8 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_8 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_8 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_8 = 171;
pub const BI_CRSSUND: C2RustUnnamed_8 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_8 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_8 = 168;
pub const BI_CRSPARA: C2RustUnnamed_8 = 167;
pub const BI_CRSHALU: C2RustUnnamed_8 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_8 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_8 = 164;
pub const BI_CRSTELE: C2RustUnnamed_8 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_8 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_8 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_8 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_8 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_8 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_8 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_8 = 156;
pub const BI_AMISSILES: C2RustUnnamed_8 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_8 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_8 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_8 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_8 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_8 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_8 = 149;
pub const BI_SHOTS: C2RustUnnamed_8 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_8 = 147;
pub const BI_BLOWS: C2RustUnnamed_8 = 146;
pub const BI_BART: C2RustUnnamed_8 = 145;
pub const BI_SLING: C2RustUnnamed_8 = 144;
pub const BI_BTODAM: C2RustUnnamed_8 = 143;
pub const BI_BTOHIT: C2RustUnnamed_8 = 142;
pub const BI_BID: C2RustUnnamed_8 = 141;
pub const BI_WDS: C2RustUnnamed_8 = 140;
pub const BI_WDD: C2RustUnnamed_8 = 139;
pub const BI_WID: C2RustUnnamed_8 = 138;
pub const BI_WTODAM: C2RustUnnamed_8 = 137;
pub const BI_WTOHIT: C2RustUnnamed_8 = 136;
pub const BI_TODAM: C2RustUnnamed_8 = 135;
pub const BI_TOHIT: C2RustUnnamed_8 = 134;
pub const BI_ARMOR: C2RustUnnamed_8 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_8 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_8 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_8 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_8 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_8 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_8 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_8 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_8 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_8 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_8 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_8 = 122;
pub const BI_ISIMAGE: C2RustUnnamed_8 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_8 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_8 = 118;
pub const BI_ISSTUN: C2RustUnnamed_8 = 117;
pub const BI_ISCUT: C2RustUnnamed_8 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_8 = 115;
pub const BI_ISAFRAID: C2RustUnnamed_8 = 113;
pub const BI_ISGORGED: C2RustUnnamed_8 = 111;
pub const BI_ISFULL: C2RustUnnamed_8 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_8 = 109;
pub const BI_ISWEAK: C2RustUnnamed_8 = 108;
pub const BI_KING: C2RustUnnamed_8 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_8 = 106;
pub const BI_SFRACT: C2RustUnnamed_8 = 104;
pub const BI_SHLIFE: C2RustUnnamed_8 = 103;
pub const BI_SRDIS: C2RustUnnamed_8 = 102;
pub const BI_SRKAOS: C2RustUnnamed_8 = 101;
pub const BI_SRNTHR: C2RustUnnamed_8 = 100;
pub const BI_SRNXUS: C2RustUnnamed_8 = 99;
pub const BI_SRSHRD: C2RustUnnamed_8 = 98;
pub const BI_SRSND: C2RustUnnamed_8 = 97;
pub const BI_SRCONF: C2RustUnnamed_8 = 96;
pub const BI_SRBLIND: C2RustUnnamed_8 = 95;
pub const BI_SRDARK: C2RustUnnamed_8 = 94;
pub const BI_SRLITE: C2RustUnnamed_8 = 93;
pub const BI_SRFEAR: C2RustUnnamed_8 = 92;
pub const BI_SRPOIS: C2RustUnnamed_8 = 91;
pub const BI_SRACID: C2RustUnnamed_8 = 90;
pub const BI_SRELEC: C2RustUnnamed_8 = 89;
pub const BI_SRCOLD: C2RustUnnamed_8 = 88;
pub const BI_SRFIRE: C2RustUnnamed_8 = 87;
pub const BI_FRACT: C2RustUnnamed_8 = 86;
pub const BI_HLIFE: C2RustUnnamed_8 = 85;
pub const BI_RDIS: C2RustUnnamed_8 = 84;
pub const BI_RKAOS: C2RustUnnamed_8 = 83;
pub const BI_RNTHR: C2RustUnnamed_8 = 82;
pub const BI_RNXUS: C2RustUnnamed_8 = 81;
pub const BI_RSHRD: C2RustUnnamed_8 = 80;
pub const BI_RSND: C2RustUnnamed_8 = 79;
pub const BI_RCONF: C2RustUnnamed_8 = 78;
pub const BI_RBLIND: C2RustUnnamed_8 = 77;
pub const BI_RDARK: C2RustUnnamed_8 = 76;
pub const BI_RLITE: C2RustUnnamed_8 = 75;
pub const BI_RFEAR: C2RustUnnamed_8 = 74;
pub const BI_RPOIS: C2RustUnnamed_8 = 73;
pub const BI_RACID: C2RustUnnamed_8 = 72;
pub const BI_RELEC: C2RustUnnamed_8 = 71;
pub const BI_RCOLD: C2RustUnnamed_8 = 70;
pub const BI_RFIRE: C2RustUnnamed_8 = 69;
pub const BI_IPOIS: C2RustUnnamed_8 = 68;
pub const BI_IELEC: C2RustUnnamed_8 = 67;
pub const BI_ICOLD: C2RustUnnamed_8 = 66;
pub const BI_IACID: C2RustUnnamed_8 = 65;
pub const BI_IFIRE: C2RustUnnamed_8 = 64;
pub const BI_DIG: C2RustUnnamed_8 = 63;
pub const BI_THT: C2RustUnnamed_8 = 62;
pub const BI_THB: C2RustUnnamed_8 = 61;
pub const BI_THN: C2RustUnnamed_8 = 60;
pub const BI_SRCH: C2RustUnnamed_8 = 59;
pub const BI_STL: C2RustUnnamed_8 = 58;
pub const BI_SAV: C2RustUnnamed_8 = 57;
pub const BI_DEV: C2RustUnnamed_8 = 56;
pub const BI_DISM: C2RustUnnamed_8 = 55;
pub const BI_DISP: C2RustUnnamed_8 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_8 = 53;
pub const BI_INFRA: C2RustUnnamed_8 = 52;
pub const BI_REG: C2RustUnnamed_8 = 50;
pub const BI_FEATH: C2RustUnnamed_8 = 49;
pub const BI_SDIG: C2RustUnnamed_8 = 48;
pub const BI_DAM_RED: C2RustUnnamed_8 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_8 = 46;
pub const BI_GOLD: C2RustUnnamed_8 = 45;
pub const BI_SPEED: C2RustUnnamed_8 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_8 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_8 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_8 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_8 = 40;
pub const BI_FOOD: C2RustUnnamed_8 = 39;
pub const BI_RECALL: C2RustUnnamed_8 = 38;
pub const BI_ESP: C2RustUnnamed_8 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_8 = 36;
pub const BI_FAIL2: C2RustUnnamed_8 = 34;
pub const BI_FAIL1: C2RustUnnamed_8 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_8 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_8 = 29;
pub const BI_CURHP: C2RustUnnamed_8 = 27;
pub const BI_CLASS: C2RustUnnamed_8 = 25;
pub const BI_SCON: C2RustUnnamed_8 = 24;
pub const BI_SDEX: C2RustUnnamed_8 = 23;
pub const BI_SWIS: C2RustUnnamed_8 = 22;
pub const BI_SINT: C2RustUnnamed_8 = 21;
pub const BI_SSTR: C2RustUnnamed_8 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_8 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_8 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_8 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_8 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_8 = 15;
pub const BI_CCON: C2RustUnnamed_8 = 14;
pub const BI_CDEX: C2RustUnnamed_8 = 13;
pub const BI_CWIS: C2RustUnnamed_8 = 12;
pub const BI_CINT: C2RustUnnamed_8 = 11;
pub const BI_CSTR: C2RustUnnamed_8 = 10;
pub const BI_ACON: C2RustUnnamed_8 = 9;
pub const BI_ADEX: C2RustUnnamed_8 = 8;
pub const BI_AWIS: C2RustUnnamed_8 = 7;
pub const BI_AINT: C2RustUnnamed_8 = 6;
pub const BI_ASTR: C2RustUnnamed_8 = 5;
pub const BI_CON: C2RustUnnamed_8 = 4;
pub const BI_DEX: C2RustUnnamed_8 = 3;
pub const BI_WIS: C2RustUnnamed_8 = 2;
pub const BI_INT: C2RustUnnamed_8 = 1;
pub const BI_STR: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_perma_aux_bless() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 20 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 25 as ::core::ffi::c_int;
    }
    if borg.temp.bless {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BLESS, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(BLESS);
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        > 10 as ::core::ffi::c_int
        && cost
            >= (if borg.mon.unique != 0 {
                *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                    / 7 as ::core::ffi::c_int
            } else {
                *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                    / 10 as ::core::ffi::c_int
            })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    borg_spell(BLESS);
    borg.no_rest_prep = 10000 as int16_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.res_fire as ::core::ffi::c_int
        + borg.temp.res_acid as ::core::ffi::c_int
        + borg.temp.res_elec as ::core::ffi::c_int
        + borg.temp.res_cold as ::core::ffi::c_int
        >= 3 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RESISTANCE);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    borg_spell_fail(RESISTANCE, fail_allowed);
    borg.no_rest_prep = 21000 as int16_t;
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist_colluin() -> ::core::ffi::c_int {
    if borg.temp.res_fire as ::core::ffi::c_int
        + borg.temp.res_acid as ::core::ffi::c_int
        + borg.temp.res_pois as ::core::ffi::c_int
        + borg.temp.res_elec as ::core::ffi::c_int
        + borg.temp.res_cold as ::core::ffi::c_int
        >= 3 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
        || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
    {
        borg.no_rest_prep = 21000 as int16_t;
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_resist_p() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.res_pois as ::core::ffi::c_int != 0 || borg.mon.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESIST_POISON, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RESIST_POISON);
    if cost
        >= *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 20 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    if borg_spell_fail(RESIST_POISON, fail_allowed) {
        borg.no_rest_prep = 21000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_speed() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.fast {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(HASTE_SELF, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(HASTE_SELF);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 5 as ::core::ffi::c_int;
    }
    if borg_spell_fail(HASTE_SELF, fail_allowed) {
        borg.no_rest_prep = (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 1000 as ::core::ffi::c_int) as int16_t;
        return 5 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_prot_evil() -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if borg.temp.prot_from_evil {
        return 0 as ::core::ffi::c_int;
    }
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(PROTECTION_FROM_EVIL);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        borg.no_rest_prep = (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 1000 as ::core::ffi::c_int) as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_fastcast() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.fastcast {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(MANA_CHANNEL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(MANA_CHANNEL);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 5 as ::core::ffi::c_int;
    }
    if borg_spell(MANA_CHANNEL) {
        borg.no_rest_prep = 6000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_hero() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.hero {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= borg_heroism_level()
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(HEROISM, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(HEROISM);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    if borg_spell(HEROISM) {
        borg.no_rest_prep = 3000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_regen() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.regen {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) < 100 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RAPID_REGENERATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(RAPID_REGENERATION);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell(RAPID_REGENERATION) {
        borg.no_rest_prep = 6000 as int16_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_smite_evil() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.smite_evil as ::core::ffi::c_int != 0
        || *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SMITE_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(SMITE_EVIL);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell(SMITE_EVIL) {
        borg.no_rest_prep = 21000 as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_venom() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.venom as ::core::ffi::c_int != 0
        || *borg
            .trait_0
            .offset(BI_WB_POIS as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(VENOM, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(SMITE_EVIL);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 3 as ::core::ffi::c_int;
    }
    if borg_spell(VENOM) {
        borg.no_rest_prep = 19000 as int16_t;
        return 3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_berserk() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0;
    if borg.mon.unique != 0 {
        fail_allowed = 10 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        fail_allowed = 15 as ::core::ffi::c_int;
    }
    if borg.temp.berserk {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BERSERK_STRENGTH, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    cost = borg_get_spell_power(BERSERK_STRENGTH);
    if cost
        >= (if borg.mon.unique != 0 {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 7 as ::core::ffi::c_int
        } else {
            *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) / 10 as ::core::ffi::c_int
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_spell(BERSERK_STRENGTH) {
        borg.no_rest_prep = 11000 as int16_t;
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_berserk_potion() -> ::core::ffi::c_int {
    if borg.near.unique == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg.temp.hero as ::core::ffi::c_int != 0 || borg.temp.berserk as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if -(1 as ::core::ffi::c_int) == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_berserk) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 2 as ::core::ffi::c_int;
    }
    if borg_quaff_potion(sv_potion_berserk) {
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux_see_inv() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0
        || borg.see_inv as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SENSE_INVISIBLE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(SENSE_INVISIBLE, fail_allowed) {
        borg.see_inv = 32000 as int16_t;
        borg.no_rest_prep = 16000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_perma_aux(mut what: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match what {
        0 => return borg_perma_aux_speed(),
        1 => return borg_perma_aux_prot_evil(),
        3 => return borg_perma_aux_resist(),
        4 => return borg_perma_aux_resist_colluin(),
        5 => return borg_perma_aux_resist_p(),
        2 => return borg_perma_aux_bless(),
        6 => return borg_perma_aux_fastcast(),
        7 => return borg_perma_aux_hero(),
        8 => return borg_perma_aux_berserk(),
        9 => return borg_perma_aux_berserk_potion(),
        10 => return borg_perma_aux_smite_evil(),
        11 => return borg_perma_aux_venom(),
        12 => return borg_perma_aux_regen(),
        13 => return 0 as ::core::ffi::c_int,
        14 => return borg_perma_aux_see_inv(),
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_perma_spell() -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    borg_simulate = true_0 != 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        < *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            / 3 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 7 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) * 75 as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    g = 0 as ::core::ffi::c_int;
    while g < BP_MAX as ::core::ffi::c_int {
        n = borg_perma_aux(g);
        if !(n <= b_n) {
            b_g = g;
            b_n = n;
        }
        g += 1;
    }
    if b_n <= 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing perma-spell type %d with value %d\0" as *const u8
            as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_perma_aux(b_g);
    return true_0 != 0;
}
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
