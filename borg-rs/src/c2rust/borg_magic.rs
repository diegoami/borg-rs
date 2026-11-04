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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    static mut player: *mut player;
    fn spell_by_index(p: *const player, index: ::core::ffi::c_int) -> *const class_spell;
    fn spell_okay_to_cast(p: *const player, spell_index: ::core::ffi::c_int) -> bool;
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    static mut z_info: *mut angband_constants;
    fn no_light(p: *const player) -> bool;
    static mut borg_items: *mut borg_item;
    static mut borg: borg_struct;
    static mut borg_grids: [*mut borg_grid; 66];
    static mut borg_init_failure: bool;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PF_MAX: C2RustUnnamed_0 = 18;
pub const PF_COMBAT_REGEN: C2RustUnnamed_0 = 17;
pub const PF_EVIL: C2RustUnnamed_0 = 16;
pub const PF_SHIELD_BASH: C2RustUnnamed_0 = 15;
pub const PF_STEAL: C2RustUnnamed_0 = 14;
pub const PF_ROCK: C2RustUnnamed_0 = 13;
pub const PF_UNLIGHT: C2RustUnnamed_0 = 12;
pub const PF_CHARM: C2RustUnnamed_0 = 11;
pub const PF_NO_MANA: C2RustUnnamed_0 = 10;
pub const PF_SEE_ORE: C2RustUnnamed_0 = 9;
pub const PF_KNOW_ZAPPER: C2RustUnnamed_0 = 8;
pub const PF_KNOW_MUSHROOM: C2RustUnnamed_0 = 7;
pub const PF_CHOOSE_SPELLS: C2RustUnnamed_0 = 6;
pub const PF_BEAM: C2RustUnnamed_0 = 5;
pub const PF_ZERO_FAIL: C2RustUnnamed_0 = 4;
pub const PF_BLESS_WEAPON: C2RustUnnamed_0 = 3;
pub const PF_BRAVERY_30: C2RustUnnamed_0 = 2;
pub const PF_FAST_SHOT: C2RustUnnamed_0 = 1;
pub const PF_NONE: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_spell_rating {
    pub name: *const ::core::ffi::c_char,
    pub rating: uint8_t,
    pub spell_enum: borg_spells,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_magic {
    pub name: *const ::core::ffi::c_char,
    pub status: uint8_t,
    pub effect_index: uint16_t,
    pub rating: uint8_t,
    pub level: uint8_t,
    pub power: uint8_t,
    pub sfail: uint8_t,
    pub book_offset: ::core::ffi::c_int,
    pub book: ::core::ffi::c_int,
    pub times: int32_t,
    pub spell_enum: borg_spells,
}
pub const BI_CLASS: C2RustUnnamed_10 = 25;
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
pub type borg_time = int32_t;
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
pub const BI_MAXSP: C2RustUnnamed_10 = 31;
pub const BI_CDEPTH: C2RustUnnamed_10 = 105;
pub const EF_TELEPORT: C2RustUnnamed_9 = 58;
pub const EF_NOURISH: C2RustUnnamed_9 = 6;
pub const BI_CURSP: C2RustUnnamed_10 = 30;
pub const BI_ISCONFUSED: C2RustUnnamed_10 = 114;
pub const BI_ISBLIND: C2RustUnnamed_10 = 112;
pub const BI_CLEVEL: C2RustUnnamed_10 = 35;
pub type keycode_t = uint32_t;
pub const BI_ISFORGET: C2RustUnnamed_10 = 121;
pub const BI_ISSTUN: C2RustUnnamed_10 = 117;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_10 = 118;
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
pub const BI_FAIL2: C2RustUnnamed_10 = 34;
pub const BI_ISAFRAID: C2RustUnnamed_10 = 113;
pub const BI_FAIL1: C2RustUnnamed_10 = 33;
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
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const EF_MAX: C2RustUnnamed_9 = 113;
pub const EF_UNSCRAMBLE_STATS: C2RustUnnamed_9 = 112;
pub const EF_SCRAMBLE_STATS: C2RustUnnamed_9 = 111;
pub const EF_CLEAR_VALUE: C2RustUnnamed_9 = 110;
pub const EF_SET_VALUE: C2RustUnnamed_9 = 109;
pub const EF_SELECT: C2RustUnnamed_9 = 108;
pub const EF_WONDER: C2RustUnnamed_9 = 107;
pub const EF_BIZARRE: C2RustUnnamed_9 = 106;
pub const EF_SWEEP: C2RustUnnamed_9 = 105;
pub const EF_MELEE_BLOWS: C2RustUnnamed_9 = 104;
pub const EF_SINGLE_COMBAT: C2RustUnnamed_9 = 103;
pub const EF_MOVE_ATTACK: C2RustUnnamed_9 = 102;
pub const EF_JUMP_AND_BITE: C2RustUnnamed_9 = 101;
pub const EF_COMMAND: C2RustUnnamed_9 = 100;
pub const EF_CURSE: C2RustUnnamed_9 = 99;
pub const EF_SHAPECHANGE: C2RustUnnamed_9 = 98;
pub const EF_TAP_UNLIFE: C2RustUnnamed_9 = 97;
pub const EF_TAP_DEVICE: C2RustUnnamed_9 = 96;
pub const EF_CREATE_ARROWS: C2RustUnnamed_9 = 95;
pub const EF_BRAND_BOLTS: C2RustUnnamed_9 = 94;
pub const EF_BRAND_AMMO: C2RustUnnamed_9 = 93;
pub const EF_BRAND_WEAPON: C2RustUnnamed_9 = 92;
pub const EF_CURSE_WEAPON: C2RustUnnamed_9 = 91;
pub const EF_CURSE_ARMOR: C2RustUnnamed_9 = 90;
pub const EF_TOUCH_AWARE: C2RustUnnamed_9 = 89;
pub const EF_TOUCH: C2RustUnnamed_9 = 88;
pub const EF_BOLT_AWARE: C2RustUnnamed_9 = 87;
pub const EF_BOLT_STATUS_DAM: C2RustUnnamed_9 = 86;
pub const EF_BOLT_STATUS: C2RustUnnamed_9 = 85;
pub const EF_ALTER: C2RustUnnamed_9 = 84;
pub const EF_LINE: C2RustUnnamed_9 = 83;
pub const EF_BOLT_OR_BEAM: C2RustUnnamed_9 = 82;
pub const EF_BEAM: C2RustUnnamed_9 = 81;
pub const EF_BOLT: C2RustUnnamed_9 = 80;
pub const EF_STAR_BALL: C2RustUnnamed_9 = 79;
pub const EF_STAR: C2RustUnnamed_9 = 78;
pub const EF_STRIKE: C2RustUnnamed_9 = 77;
pub const EF_SWARM: C2RustUnnamed_9 = 76;
pub const EF_LASH: C2RustUnnamed_9 = 75;
pub const EF_SHORT_BEAM: C2RustUnnamed_9 = 74;
pub const EF_ARC: C2RustUnnamed_9 = 73;
pub const EF_BREATH: C2RustUnnamed_9 = 72;
pub const EF_BALL: C2RustUnnamed_9 = 71;
pub const EF_SPHERE: C2RustUnnamed_9 = 70;
pub const EF_SPOT: C2RustUnnamed_9 = 69;
pub const EF_DARKEN_AREA: C2RustUnnamed_9 = 68;
pub const EF_LIGHT_AREA: C2RustUnnamed_9 = 67;
pub const EF_DARKEN_LEVEL: C2RustUnnamed_9 = 66;
pub const EF_LIGHT_LEVEL: C2RustUnnamed_9 = 65;
pub const EF_EARTHQUAKE: C2RustUnnamed_9 = 64;
pub const EF_DESTRUCTION: C2RustUnnamed_9 = 63;
pub const EF_GRANITE: C2RustUnnamed_9 = 62;
pub const EF_RUBBLE: C2RustUnnamed_9 = 61;
pub const EF_TELEPORT_LEVEL: C2RustUnnamed_9 = 60;
pub const EF_TELEPORT_TO: C2RustUnnamed_9 = 59;
pub const EF_PROBE: C2RustUnnamed_9 = 57;
pub const EF_MASS_BANISH: C2RustUnnamed_9 = 56;
pub const EF_BANISH: C2RustUnnamed_9 = 55;
pub const EF_SUMMON: C2RustUnnamed_9 = 54;
pub const EF_WAKE: C2RustUnnamed_9 = 53;
pub const EF_ACQUIRE: C2RustUnnamed_9 = 52;
pub const EF_PROJECT_LOS_AWARE: C2RustUnnamed_9 = 51;
pub const EF_PROJECT_LOS: C2RustUnnamed_9 = 50;
pub const EF_RECHARGE: C2RustUnnamed_9 = 49;
pub const EF_ENCHANT: C2RustUnnamed_9 = 48;
pub const EF_DISENCHANT: C2RustUnnamed_9 = 47;
pub const EF_CREATE_STAIRS: C2RustUnnamed_9 = 46;
pub const EF_DETECT_SOUL: C2RustUnnamed_9 = 45;
pub const EF_DETECT_EVIL: C2RustUnnamed_9 = 44;
pub const EF_IDENTIFY: C2RustUnnamed_9 = 43;
pub const EF_DETECT_FEARFUL_MONSTERS: C2RustUnnamed_9 = 42;
pub const EF_DETECT_INVISIBLE_MONSTERS: C2RustUnnamed_9 = 41;
pub const EF_DETECT_VISIBLE_MONSTERS: C2RustUnnamed_9 = 40;
pub const EF_DETECT_LIVING_MONSTERS: C2RustUnnamed_9 = 39;
pub const EF_DETECT_OBJECTS: C2RustUnnamed_9 = 38;
pub const EF_SENSE_OBJECTS: C2RustUnnamed_9 = 37;
pub const EF_DETECT_GOLD: C2RustUnnamed_9 = 36;
pub const EF_SENSE_GOLD: C2RustUnnamed_9 = 35;
pub const EF_DETECT_ORE: C2RustUnnamed_9 = 34;
pub const EF_DETECT_STAIRS: C2RustUnnamed_9 = 33;
pub const EF_DETECT_DOORS: C2RustUnnamed_9 = 32;
pub const EF_DETECT_TRAPS: C2RustUnnamed_9 = 31;
pub const EF_READ_MINDS: C2RustUnnamed_9 = 30;
pub const EF_MAP_AREA: C2RustUnnamed_9 = 29;
pub const EF_ALTER_REALITY: C2RustUnnamed_9 = 28;
pub const EF_DEEP_DESCENT: C2RustUnnamed_9 = 27;
pub const EF_RECALL: C2RustUnnamed_9 = 26;
pub const EF_REMOVE_CURSE: C2RustUnnamed_9 = 25;
pub const EF_RESTORE_MANA: C2RustUnnamed_9 = 24;
pub const EF_DRAIN_MANA: C2RustUnnamed_9 = 23;
pub const EF_DRAIN_LIGHT: C2RustUnnamed_9 = 22;
pub const EF_GAIN_EXP: C2RustUnnamed_9 = 21;
pub const EF_RESTORE_EXP: C2RustUnnamed_9 = 20;
pub const EF_GAIN_STAT: C2RustUnnamed_9 = 19;
pub const EF_LOSE_RANDOM_STAT: C2RustUnnamed_9 = 18;
pub const EF_DRAIN_STAT: C2RustUnnamed_9 = 17;
pub const EF_RESTORE_STAT: C2RustUnnamed_9 = 16;
pub const EF_WEB: C2RustUnnamed_9 = 15;
pub const EF_GLYPH: C2RustUnnamed_9 = 14;
pub const EF_TIMED_DEC: C2RustUnnamed_9 = 13;
pub const EF_MON_TIMED_INC: C2RustUnnamed_9 = 12;
pub const EF_TIMED_INC_NO_RES: C2RustUnnamed_9 = 11;
pub const EF_TIMED_INC: C2RustUnnamed_9 = 10;
pub const EF_TIMED_SET: C2RustUnnamed_9 = 9;
pub const EF_CURE: C2RustUnnamed_9 = 8;
pub const EF_CRUNCH: C2RustUnnamed_9 = 7;
pub const EF_MON_HEAL_KIN: C2RustUnnamed_9 = 5;
pub const EF_MON_HEAL_HP: C2RustUnnamed_9 = 4;
pub const EF_HEAL_HP: C2RustUnnamed_9 = 3;
pub const EF_DAMAGE: C2RustUnnamed_9 = 2;
pub const EF_RANDOM: C2RustUnnamed_9 = 1;
pub const EF_NONE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_10 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_10 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_10 = 263;
pub const BI_EMPTY: C2RustUnnamed_10 = 262;
pub const BI_CARRY: C2RustUnnamed_10 = 261;
pub const BI_WEIGHT: C2RustUnnamed_10 = 260;
pub const BI_DINV: C2RustUnnamed_10 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_10 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_10 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_10 = 256;
pub const BI_ADIGGER: C2RustUnnamed_10 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_10 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_10 = 253;
pub const BI_AROD2: C2RustUnnamed_10 = 252;
pub const BI_AROD1: C2RustUnnamed_10 = 251;
pub const BI_ASHROOM: C2RustUnnamed_10 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_10 = 249;
pub const BI_AHWORD: C2RustUnnamed_10 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_10 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_10 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_10 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_10 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_10 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_10 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_10 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_10 = 240;
pub const BI_ABRAND: C2RustUnnamed_10 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_10 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_10 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_10 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_10 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_10 = 234;
pub const BI_ACLW: C2RustUnnamed_10 = 233;
pub const BI_ACSW: C2RustUnnamed_10 = 232;
pub const BI_ACCW: C2RustUnnamed_10 = 231;
pub const BI_AGLYPH: C2RustUnnamed_10 = 230;
pub const BI_APFE: C2RustUnnamed_10 = 229;
pub const BI_ALITE: C2RustUnnamed_10 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_10 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_10 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_10 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_10 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_10 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_10 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_10 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_10 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_10 = 219;
pub const BI_ASPEED: C2RustUnnamed_10 = 218;
pub const BI_AID: C2RustUnnamed_10 = 217;
pub const BI_ALIFE: C2RustUnnamed_10 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_10 = 215;
pub const BI_AHEAL: C2RustUnnamed_10 = 214;
pub const BI_AFUEL: C2RustUnnamed_10 = 213;
pub const BI_AESCAPE: C2RustUnnamed_10 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_10 = 211;
pub const BI_APHASE: C2RustUnnamed_10 = 210;
pub const BI_WB_POIS: C2RustUnnamed_10 = 209;
pub const BI_WB_COLD: C2RustUnnamed_10 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_10 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_10 = 206;
pub const BI_WB_ACID: C2RustUnnamed_10 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_10 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_10 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_10 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_10 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_10 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_10 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_10 = 198;
pub const BI_WS_ORC: C2RustUnnamed_10 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_10 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_10 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_10 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_10 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_10 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_10 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_10 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_10 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_10 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_10 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_10 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_10 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_10 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_10 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_10 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_10 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_10 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_10 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_10 = 178;
pub const BI_CRSSICK: C2RustUnnamed_10 = 177;
pub const BI_CRSDULL: C2RustUnnamed_10 = 176;
pub const BI_CRSVULN: C2RustUnnamed_10 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_10 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_10 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_10 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_10 = 171;
pub const BI_CRSSUND: C2RustUnnamed_10 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_10 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_10 = 168;
pub const BI_CRSPARA: C2RustUnnamed_10 = 167;
pub const BI_CRSHALU: C2RustUnnamed_10 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_10 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_10 = 164;
pub const BI_CRSTELE: C2RustUnnamed_10 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_10 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_10 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_10 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_10 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_10 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_10 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_10 = 156;
pub const BI_AMISSILES: C2RustUnnamed_10 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_10 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_10 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_10 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_10 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_10 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_10 = 149;
pub const BI_SHOTS: C2RustUnnamed_10 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_10 = 147;
pub const BI_BLOWS: C2RustUnnamed_10 = 146;
pub const BI_BART: C2RustUnnamed_10 = 145;
pub const BI_SLING: C2RustUnnamed_10 = 144;
pub const BI_BTODAM: C2RustUnnamed_10 = 143;
pub const BI_BTOHIT: C2RustUnnamed_10 = 142;
pub const BI_BID: C2RustUnnamed_10 = 141;
pub const BI_WDS: C2RustUnnamed_10 = 140;
pub const BI_WDD: C2RustUnnamed_10 = 139;
pub const BI_WID: C2RustUnnamed_10 = 138;
pub const BI_WTODAM: C2RustUnnamed_10 = 137;
pub const BI_WTOHIT: C2RustUnnamed_10 = 136;
pub const BI_TODAM: C2RustUnnamed_10 = 135;
pub const BI_TOHIT: C2RustUnnamed_10 = 134;
pub const BI_ARMOR: C2RustUnnamed_10 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_10 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_10 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_10 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_10 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_10 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_10 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_10 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_10 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_10 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_10 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_10 = 122;
pub const BI_ISIMAGE: C2RustUnnamed_10 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_10 = 119;
pub const BI_ISCUT: C2RustUnnamed_10 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_10 = 115;
pub const BI_ISGORGED: C2RustUnnamed_10 = 111;
pub const BI_ISFULL: C2RustUnnamed_10 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_10 = 109;
pub const BI_ISWEAK: C2RustUnnamed_10 = 108;
pub const BI_KING: C2RustUnnamed_10 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_10 = 106;
pub const BI_SFRACT: C2RustUnnamed_10 = 104;
pub const BI_SHLIFE: C2RustUnnamed_10 = 103;
pub const BI_SRDIS: C2RustUnnamed_10 = 102;
pub const BI_SRKAOS: C2RustUnnamed_10 = 101;
pub const BI_SRNTHR: C2RustUnnamed_10 = 100;
pub const BI_SRNXUS: C2RustUnnamed_10 = 99;
pub const BI_SRSHRD: C2RustUnnamed_10 = 98;
pub const BI_SRSND: C2RustUnnamed_10 = 97;
pub const BI_SRCONF: C2RustUnnamed_10 = 96;
pub const BI_SRBLIND: C2RustUnnamed_10 = 95;
pub const BI_SRDARK: C2RustUnnamed_10 = 94;
pub const BI_SRLITE: C2RustUnnamed_10 = 93;
pub const BI_SRFEAR: C2RustUnnamed_10 = 92;
pub const BI_SRPOIS: C2RustUnnamed_10 = 91;
pub const BI_SRACID: C2RustUnnamed_10 = 90;
pub const BI_SRELEC: C2RustUnnamed_10 = 89;
pub const BI_SRCOLD: C2RustUnnamed_10 = 88;
pub const BI_SRFIRE: C2RustUnnamed_10 = 87;
pub const BI_FRACT: C2RustUnnamed_10 = 86;
pub const BI_HLIFE: C2RustUnnamed_10 = 85;
pub const BI_RDIS: C2RustUnnamed_10 = 84;
pub const BI_RKAOS: C2RustUnnamed_10 = 83;
pub const BI_RNTHR: C2RustUnnamed_10 = 82;
pub const BI_RNXUS: C2RustUnnamed_10 = 81;
pub const BI_RSHRD: C2RustUnnamed_10 = 80;
pub const BI_RSND: C2RustUnnamed_10 = 79;
pub const BI_RCONF: C2RustUnnamed_10 = 78;
pub const BI_RBLIND: C2RustUnnamed_10 = 77;
pub const BI_RDARK: C2RustUnnamed_10 = 76;
pub const BI_RLITE: C2RustUnnamed_10 = 75;
pub const BI_RFEAR: C2RustUnnamed_10 = 74;
pub const BI_RPOIS: C2RustUnnamed_10 = 73;
pub const BI_RACID: C2RustUnnamed_10 = 72;
pub const BI_RELEC: C2RustUnnamed_10 = 71;
pub const BI_RCOLD: C2RustUnnamed_10 = 70;
pub const BI_RFIRE: C2RustUnnamed_10 = 69;
pub const BI_IPOIS: C2RustUnnamed_10 = 68;
pub const BI_IELEC: C2RustUnnamed_10 = 67;
pub const BI_ICOLD: C2RustUnnamed_10 = 66;
pub const BI_IACID: C2RustUnnamed_10 = 65;
pub const BI_IFIRE: C2RustUnnamed_10 = 64;
pub const BI_DIG: C2RustUnnamed_10 = 63;
pub const BI_THT: C2RustUnnamed_10 = 62;
pub const BI_THB: C2RustUnnamed_10 = 61;
pub const BI_THN: C2RustUnnamed_10 = 60;
pub const BI_SRCH: C2RustUnnamed_10 = 59;
pub const BI_STL: C2RustUnnamed_10 = 58;
pub const BI_SAV: C2RustUnnamed_10 = 57;
pub const BI_DEV: C2RustUnnamed_10 = 56;
pub const BI_DISM: C2RustUnnamed_10 = 55;
pub const BI_DISP: C2RustUnnamed_10 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_10 = 53;
pub const BI_INFRA: C2RustUnnamed_10 = 52;
pub const BI_SINV: C2RustUnnamed_10 = 51;
pub const BI_REG: C2RustUnnamed_10 = 50;
pub const BI_FEATH: C2RustUnnamed_10 = 49;
pub const BI_SDIG: C2RustUnnamed_10 = 48;
pub const BI_DAM_RED: C2RustUnnamed_10 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_10 = 46;
pub const BI_GOLD: C2RustUnnamed_10 = 45;
pub const BI_SPEED: C2RustUnnamed_10 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_10 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_10 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_10 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_10 = 40;
pub const BI_FOOD: C2RustUnnamed_10 = 39;
pub const BI_RECALL: C2RustUnnamed_10 = 38;
pub const BI_ESP: C2RustUnnamed_10 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_10 = 36;
pub const BI_SP_ADJ: C2RustUnnamed_10 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_10 = 29;
pub const BI_MAXHP: C2RustUnnamed_10 = 28;
pub const BI_CURHP: C2RustUnnamed_10 = 27;
pub const BI_LIGHT: C2RustUnnamed_10 = 26;
pub const BI_SCON: C2RustUnnamed_10 = 24;
pub const BI_SDEX: C2RustUnnamed_10 = 23;
pub const BI_SWIS: C2RustUnnamed_10 = 22;
pub const BI_SINT: C2RustUnnamed_10 = 21;
pub const BI_SSTR: C2RustUnnamed_10 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_10 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_10 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_10 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_10 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_10 = 15;
pub const BI_CCON: C2RustUnnamed_10 = 14;
pub const BI_CDEX: C2RustUnnamed_10 = 13;
pub const BI_CWIS: C2RustUnnamed_10 = 12;
pub const BI_CINT: C2RustUnnamed_10 = 11;
pub const BI_CSTR: C2RustUnnamed_10 = 10;
pub const BI_ACON: C2RustUnnamed_10 = 9;
pub const BI_ADEX: C2RustUnnamed_10 = 8;
pub const BI_AWIS: C2RustUnnamed_10 = 7;
pub const BI_AINT: C2RustUnnamed_10 = 6;
pub const BI_ASTR: C2RustUnnamed_10 = 5;
pub const BI_CON: C2RustUnnamed_10 = 4;
pub const BI_DEX: C2RustUnnamed_10 = 3;
pub const BI_WIS: C2RustUnnamed_10 = 2;
pub const BI_INT: C2RustUnnamed_10 = 1;
pub const BI_STR: C2RustUnnamed_10 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PY_SPELL_LEARNED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PY_SPELL_WORKED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PY_SPELL_FORGOTTEN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_magics: *mut borg_magic = ::core::ptr::null::<borg_magic>() as *mut borg_magic;
static mut borg_spell_ratings: *mut borg_spell_rating =
    ::core::ptr::null::<borg_spell_rating>() as *mut borg_spell_rating;
static mut borg_spell_ratings_MAGE: [borg_spell_rating; 30] = [
    borg_spell_rating {
        name: b"Magic Missile\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MAGIC_MISSILE,
    },
    borg_spell_rating {
        name: b"Light Room\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: LIGHT_ROOM,
    },
    borg_spell_rating {
        name: b"Find Traps, Doors & Stairs\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: FIND_TRAPS_DOORS_STAIRS,
    },
    borg_spell_rating {
        name: b"Phase Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: PHASE_DOOR,
    },
    borg_spell_rating {
        name: b"Electric Arc\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ELECTRIC_ARC,
    },
    borg_spell_rating {
        name: b"Detect Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_MONSTERS,
    },
    borg_spell_rating {
        name: b"Fire Ball\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FIRE_BALL,
    },
    borg_spell_rating {
        name: b"Recharging\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: RECHARGING,
    },
    borg_spell_rating {
        name: b"Identify Rune\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: IDENTIFY_RUNE,
    },
    borg_spell_rating {
        name: b"Treasure Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: TREASURE_DETECTION,
    },
    borg_spell_rating {
        name: b"Frost Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FROST_BOLT,
    },
    borg_spell_rating {
        name: b"Reveal Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REVEAL_MONSTERS,
    },
    borg_spell_rating {
        name: b"Acid Spray\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ACID_SPRAY,
    },
    borg_spell_rating {
        name: b"Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DISABLE_TRAPS_DESTROY_DOORS,
    },
    borg_spell_rating {
        name: b"Teleport Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: TELEPORT_SELF,
    },
    borg_spell_rating {
        name: b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: TELEPORT_OTHER,
    },
    borg_spell_rating {
        name: b"Resistance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RESISTANCE,
    },
    borg_spell_rating {
        name: b"Tap Magical Energy\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: TAP_MAGICAL_ENERGY,
    },
    borg_spell_rating {
        name: b"Mana Channel\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MANA_CHANNEL,
    },
    borg_spell_rating {
        name: b"Door Creation\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DOOR_CREATION,
    },
    borg_spell_rating {
        name: b"Mana Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MANA_BOLT,
    },
    borg_spell_rating {
        name: b"Teleport Level\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: TELEPORT_LEVEL,
    },
    borg_spell_rating {
        name: b"Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DETECTION,
    },
    borg_spell_rating {
        name: b"Dimension Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DIMENSION_DOOR,
    },
    borg_spell_rating {
        name: b"Thrust Away\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: THRUST_AWAY,
    },
    borg_spell_rating {
        name: b"Shock Wave\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SHOCK_WAVE,
    },
    borg_spell_rating {
        name: b"Explosion\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: EXPLOSION,
    },
    borg_spell_rating {
        name: b"Banishment\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: BANISHMENT,
    },
    borg_spell_rating {
        name: b"Mass Banishment\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: MASS_BANISHMENT,
    },
    borg_spell_rating {
        name: b"Mana Storm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: MANA_STORM,
    },
];
static mut borg_spell_ratings_DRUID: [borg_spell_rating; 27] = [
    borg_spell_rating {
        name: b"Detect Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: DETECT_LIFE,
    },
    borg_spell_rating {
        name: b"Fox Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: FOX_FORM,
    },
    borg_spell_rating {
        name: b"Remove Hunger\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REMOVE_HUNGER,
    },
    borg_spell_rating {
        name: b"Stinking Cloud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: STINKING_CLOUD,
    },
    borg_spell_rating {
        name: b"Confuse Monster\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: CONFUSE_MONSTER,
    },
    borg_spell_rating {
        name: b"Slow Monster\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: SLOW_MONSTER,
    },
    borg_spell_rating {
        name: b"Cure Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: CURE_POISON,
    },
    borg_spell_rating {
        name: b"Resist Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 60 as uint8_t,
        spell_enum: RESIST_POISON,
    },
    borg_spell_rating {
        name: b"Turn Stone to Mud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: TURN_STONE_TO_MUD,
    },
    borg_spell_rating {
        name: b"Sense Surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: SENSE_SURROUNDINGS,
    },
    borg_spell_rating {
        name: b"Lightning Strike\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: LIGHTNING_STRIKE,
    },
    borg_spell_rating {
        name: b"Earth Rising\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 70 as uint8_t,
        spell_enum: EARTH_RISING,
    },
    borg_spell_rating {
        name: b"Trance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: TRANCE,
    },
    borg_spell_rating {
        name: b"Mass Sleep\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: MASS_SLEEP,
    },
    borg_spell_rating {
        name: b"Become Pukel-man\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BECOME_PUKEL_MAN,
    },
    borg_spell_rating {
        name: b"Eagle's Flight\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: EAGLES_FLIGHT,
    },
    borg_spell_rating {
        name: b"Bear Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BEAR_FORM,
    },
    borg_spell_rating {
        name: b"Tremor\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 80 as uint8_t,
        spell_enum: TREMOR,
    },
    borg_spell_rating {
        name: b"Haste Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: HASTE_SELF,
    },
    borg_spell_rating {
        name: b"Revitalize\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: REVITALIZE,
    },
    borg_spell_rating {
        name: b"Rapid Regeneration\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: RAPID_REGENERATION,
    },
    borg_spell_rating {
        name: b"Herbal Curing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: HERBAL_CURING,
    },
    borg_spell_rating {
        name: b"Meteor Swarm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: METEOR_SWARM,
    },
    borg_spell_rating {
        name: b"Rift\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RIFT,
    },
    borg_spell_rating {
        name: b"Ice Storm\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ICE_STORM,
    },
    borg_spell_rating {
        name: b"Volcanic Eruption\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 60 as uint8_t,
        spell_enum: VOLCANIC_ERUPTION,
    },
    borg_spell_rating {
        name: b"River of Lightning\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 90 as uint8_t,
        spell_enum: RIVER_OF_LIGHTNING,
    },
];
static mut borg_spell_ratings_PRIEST: [borg_spell_rating; 28] = [
    borg_spell_rating {
        name: b"Call Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: CALL_LIGHT,
    },
    borg_spell_rating {
        name: b"Detect Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_EVIL,
    },
    borg_spell_rating {
        name: b"Minor Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: MINOR_HEALING,
    },
    borg_spell_rating {
        name: b"Bless\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: BLESS,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: HEROISM,
    },
    borg_spell_rating {
        name: b"Orb of Draining\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: ORB_OF_DRAINING,
    },
    borg_spell_rating {
        name: b"Spear of Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SPEAR_OF_LIGHT,
    },
    borg_spell_rating {
        name: b"Dispel Undead\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_UNDEAD,
    },
    borg_spell_rating {
        name: b"Dispel Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_EVIL,
    },
    borg_spell_rating {
        name: b"Protection from Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PROTECTION_FROM_EVIL,
    },
    borg_spell_rating {
        name: b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REMOVE_CURSE,
    },
    borg_spell_rating {
        name: b"Portal\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PORTAL,
    },
    borg_spell_rating {
        name: b"Remembrance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: REMEMBRANCE,
    },
    borg_spell_rating {
        name: b"Word of Recall\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: WORD_OF_RECALL,
    },
    borg_spell_rating {
        name: b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HEALING,
    },
    borg_spell_rating {
        name: b"Restoration\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: RESTORATION,
    },
    borg_spell_rating {
        name: b"Clairvoyance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CLAIRVOYANCE,
    },
    borg_spell_rating {
        name: b"Enchant Weapon\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_WEAPON,
    },
    borg_spell_rating {
        name: b"Enchant Armour\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_ARMOUR,
    },
    borg_spell_rating {
        name: b"Smite Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SMITE_EVIL,
    },
    borg_spell_rating {
        name: b"Glyph of Warding\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: GLYPH_OF_WARDING,
    },
    borg_spell_rating {
        name: b"Demon Bane\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DEMON_BANE,
    },
    borg_spell_rating {
        name: b"Banish Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: BANISH_EVIL,
    },
    borg_spell_rating {
        name: b"Word of Destruction\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: WORD_OF_DESTRUCTION,
    },
    borg_spell_rating {
        name: b"Holy Word\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: HOLY_WORD,
    },
    borg_spell_rating {
        name: b"Spear of Orom\xC3\xAB\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SPEAR_OF_OROME,
    },
    borg_spell_rating {
        name: b"Light of Varda\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: LIGHT_OF_VARDA,
    },
];
static mut borg_spell_ratings_NECROMANCER: [borg_spell_rating; 26] = [
    borg_spell_rating {
        name: b"Nether Bolt\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: NETHER_BOLT,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Create Darkness\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: CREATE_DARKNESS,
    },
    borg_spell_rating {
        name: b"Bat Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BAT_FORM,
    },
    borg_spell_rating {
        name: b"Read Minds\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: READ_MINDS,
    },
    borg_spell_rating {
        name: b"Tap Unlife\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TAP_UNLIFE,
    },
    borg_spell_rating {
        name: b"Crush\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: CRUSH,
    },
    borg_spell_rating {
        name: b"Sleep Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: SLEEP_EVIL,
    },
    borg_spell_rating {
        name: b"Shadow Shift\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SHADOW_SHIFT,
    },
    borg_spell_rating {
        name: b"Disenchant\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 25 as uint8_t,
        spell_enum: DISENCHANT,
    },
    borg_spell_rating {
        name: b"Frighten\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: FRIGHTEN,
    },
    borg_spell_rating {
        name: b"Vampire Strike\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: VAMPIRE_STRIKE,
    },
    borg_spell_rating {
        name: b"Dispel Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DISPEL_LIFE,
    },
    borg_spell_rating {
        name: b"Dark Spear\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: DARK_SPEAR,
    },
    borg_spell_rating {
        name: b"Warg Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: WARG_FORM,
    },
    borg_spell_rating {
        name: b"Banish Spirits\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: BANISH_SPIRITS,
    },
    borg_spell_rating {
        name: b"Annihilate\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: ANNIHILATE,
    },
    borg_spell_rating {
        name: b"Grond's Blow\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: GRONDS_BLOW,
    },
    borg_spell_rating {
        name: b"Unleash Chaos\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: UNLEASH_CHAOS,
    },
    borg_spell_rating {
        name: b"Fume of Mordor\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: FUME_OF_MORDOR,
    },
    borg_spell_rating {
        name: b"Storm of Darkness\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: STORM_OF_DARKNESS,
    },
    borg_spell_rating {
        name: b"Power Sacrifice\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: POWER_SACRIFICE,
    },
    borg_spell_rating {
        name: b"Zone of Unmagic\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: ZONE_OF_UNMAGIC,
    },
    borg_spell_rating {
        name: b"Vampire Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: VAMPIRE_FORM,
    },
    borg_spell_rating {
        name: b"Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: CURSE,
    },
    borg_spell_rating {
        name: b"Command\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: COMMAND,
    },
];
static mut borg_spell_ratings_PALADIN: [borg_spell_rating; 16] = [
    borg_spell_rating {
        name: b"Bless\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BLESS,
    },
    borg_spell_rating {
        name: b"Detect Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_EVIL,
    },
    borg_spell_rating {
        name: b"Call Light\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CALL_LIGHT,
    },
    borg_spell_rating {
        name: b"Minor Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: MINOR_HEALING,
    },
    borg_spell_rating {
        name: b"Sense Invisible\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: SENSE_INVISIBLE,
    },
    borg_spell_rating {
        name: b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: HEROISM,
    },
    borg_spell_rating {
        name: b"Protection from Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: PROTECTION_FROM_EVIL,
    },
    borg_spell_rating {
        name: b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: REMOVE_CURSE,
    },
    borg_spell_rating {
        name: b"Word of Recall\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: WORD_OF_RECALL,
    },
    borg_spell_rating {
        name: b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HEALING,
    },
    borg_spell_rating {
        name: b"Clairvoyance\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CLAIRVOYANCE,
    },
    borg_spell_rating {
        name: b"Smite Evil\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: SMITE_EVIL,
    },
    borg_spell_rating {
        name: b"Demon Bane\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: DEMON_BANE,
    },
    borg_spell_rating {
        name: b"Enchant Weapon\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: ENCHANT_WEAPON,
    },
    borg_spell_rating {
        name: b"Enchant Armour\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: ENCHANT_ARMOUR,
    },
    borg_spell_rating {
        name: b"Single Combat\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SINGLE_COMBAT,
    },
];
static mut borg_spell_ratings_ROGUE: [borg_spell_rating; 10] = [
    borg_spell_rating {
        name: b"Detect Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_MONSTERS,
    },
    borg_spell_rating {
        name: b"Phase Door\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: PHASE_DOOR,
    },
    borg_spell_rating {
        name: b"Object Detection\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: OBJECT_DETECTION,
    },
    borg_spell_rating {
        name: b"Detect Stairs\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: DETECT_STAIRS,
    },
    borg_spell_rating {
        name: b"Recharging\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: RECHARGING,
    },
    borg_spell_rating {
        name: b"Reveal Monsters\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: REVEAL_MONSTERS,
    },
    borg_spell_rating {
        name: b"Teleport Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: TELEPORT_SELF,
    },
    borg_spell_rating {
        name: b"Hit and Run\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 15 as uint8_t,
        spell_enum: HIT_AND_RUN,
    },
    borg_spell_rating {
        name: b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TELEPORT_OTHER,
    },
    borg_spell_rating {
        name: b"Teleport Level\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: TELEPORT_LEVEL,
    },
];
static mut borg_spell_ratings_RANGER: [borg_spell_rating; 11] = [
    borg_spell_rating {
        name: b"Remove Hunger\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: REMOVE_HUNGER,
    },
    borg_spell_rating {
        name: b"Detect Life\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: DETECT_LIFE,
    },
    borg_spell_rating {
        name: b"Herbal Curing\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HERBAL_CURING,
    },
    borg_spell_rating {
        name: b"Resist Poison\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: RESIST_POISON,
    },
    borg_spell_rating {
        name: b"Turn Stone to Mud\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: TURN_STONE_TO_MUD,
    },
    borg_spell_rating {
        name: b"Sense Surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: SENSE_SURROUNDINGS,
    },
    borg_spell_rating {
        name: b"Cover Tracks\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 25 as uint8_t,
        spell_enum: COVER_TRACKS,
    },
    borg_spell_rating {
        name: b"Create Arrows\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: CREATE_ARROWS,
    },
    borg_spell_rating {
        name: b"Haste Self\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: HASTE_SELF,
    },
    borg_spell_rating {
        name: b"Decoy\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: DECOY,
    },
    borg_spell_rating {
        name: b"Brand Ammunition\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BRAND_AMMUNITION,
    },
];
static mut borg_spell_ratings_BLACKGUARD: [borg_spell_rating; 15] = [
    borg_spell_rating {
        name: b"Seek Battle\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: SEEK_BATTLE,
    },
    borg_spell_rating {
        name: b"Berserk Strength\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: BERSERK_STRENGTH,
    },
    borg_spell_rating {
        name: b"Whirlwind Attack\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 85 as uint8_t,
        spell_enum: WHIRLWIND_ATTACK,
    },
    borg_spell_rating {
        name: b"Shatter Stone\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: SHATTER_STONE,
    },
    borg_spell_rating {
        name: b"Leap into Battle\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: LEAP_INTO_BATTLE,
    },
    borg_spell_rating {
        name: b"Grim Purpose\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 65 as uint8_t,
        spell_enum: GRIM_PURPOSE,
    },
    borg_spell_rating {
        name: b"Maim Foe\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 75 as uint8_t,
        spell_enum: MAIM_FOE,
    },
    borg_spell_rating {
        name: b"Howl of the Damned\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: HOWL_OF_THE_DAMNED,
    },
    borg_spell_rating {
        name: b"Relentless Taunting\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: RELENTLESS_TAUNTING,
    },
    borg_spell_rating {
        name: b"Venom\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 55 as uint8_t,
        spell_enum: VENOM,
    },
    borg_spell_rating {
        name: b"Werewolf Form\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: WEREWOLF_FORM,
    },
    borg_spell_rating {
        name: b"Bloodlust\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: BLOODLUST,
    },
    borg_spell_rating {
        name: b"Unholy Reprieve\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: UNHOLY_REPRIEVE,
    },
    borg_spell_rating {
        name: b"Forceful Blow\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 5 as uint8_t,
        spell_enum: FORCEFUL_BLOW,
    },
    borg_spell_rating {
        name: b"Quake\0" as *const u8 as *const ::core::ffi::c_char,
        rating: 95 as uint8_t,
        spell_enum: QUAKE,
    },
];
#[no_mangle]
pub unsafe extern "C" fn borg_spell_stat() -> ::core::ffi::c_int {
    if borg_can_cast() {
        let mut spell: *mut class_spell = (*(*(*player).class)
            .magic
            .books
            .offset(0 as ::core::ffi::c_int as isize))
        .spells
        .offset(0 as ::core::ffi::c_int as isize)
            as *mut class_spell;
        if !spell.is_null() {
            return (*(*spell).realm).stat;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_can_cast() -> bool {
    return (*(*player).class).magic.total_spells != 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_primarily_caster() -> bool {
    return (*(*player).class).magic.num_books > 3 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_heroism_level() -> ::core::ffi::c_int {
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PRIEST {
        return 20 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PALADIN {
        return 15 as ::core::ffi::c_int;
    }
    return 99 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_book_num(mut sval: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if !borg_can_cast() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut book_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book_num < (*(*player).class).magic.num_books {
        if (*(*(*player).class).magic.books.offset(book_num as isize)).sval == sval {
            return book_num;
        }
        book_num += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_is_dungeon_book(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> bool {
    match tval {
        31 | 30 | 32 | 33 | 34 => {}
        _ => return false_0 != 0,
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*player).class).magic.num_books {
        let mut book: class_book = *(*(*player).class).magic.books.offset(i as isize);
        if tval == book.tval && sval == book.sval && book.dungeon as ::core::ffi::c_int != 0 {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_spell_entry(
    mut book: ::core::ffi::c_int,
    mut entry: ::core::ffi::c_int,
) -> *mut borg_magic {
    let mut entry_in_book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < (*(*player).class).magic.total_spells {
        if (*borg_magics.offset(spell_num as isize)).book == book {
            if entry_in_book == entry {
                return borg_magics.offset(spell_num as isize) as *mut borg_magic;
            }
            entry_in_book += 1;
        }
        spell_num += 1;
    }
    return ::core::ptr::null_mut::<borg_magic>();
}
unsafe extern "C" fn borg_get_spell_number(spell: borg_spells) -> ::core::ffi::c_int {
    if borg_magics.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut total_spells: ::core::ffi::c_int = (*(*player).class).magic.total_spells;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < total_spells {
        if (*borg_magics.offset(spell_num as isize)).spell_enum as ::core::ffi::c_uint
            == spell as ::core::ffi::c_uint
        {
            return spell_num;
        }
        spell_num += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_spell_power(spell: borg_spells) -> ::core::ffi::c_int {
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    return (*as_0).power as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_legal(spell: borg_spells) -> bool {
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if (borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if ((*borg_magics.offset(spell_num as isize)).status as ::core::ffi::c_int) < BORG_MAGIC_TEST {
        return false_0 != 0;
    }
    if (*borg_magics.offset(spell_num as isize)).power as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_spell_has_effect(
    mut spell_num: ::core::ffi::c_int,
    mut effect: uint16_t,
) -> bool {
    let mut cspell: *const class_spell = spell_by_index(player, spell_num);
    let mut eff: *mut effect = (*cspell).effect;
    while !eff.is_null() {
        if (*eff).index as ::core::ffi::c_int == effect as ::core::ffi::c_int {
            return true_0 != 0;
        }
        eff = (*eff).next;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_okay(spell: borg_spells) -> bool {
    let mut reserve_mana: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if no_light(player) {
        return false_0 != 0;
    }
    match *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) {
        CLASS_MAGE => {
            reserve_mana = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            reserve_mana = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            reserve_mana = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            reserve_mana = 10 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            reserve_mana = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            reserve_mana = 20 as ::core::ffi::c_int;
        }
        CLASS_BLACKGUARD => {
            reserve_mana = 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 35 as ::core::ffi::c_int
    {
        reserve_mana = 0 as ::core::ffi::c_int;
    }
    if !borg_spell_legal(spell) {
        return false_0 != 0;
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
        return false_0 != 0;
    }
    if (*as_0).power as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    if (*borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
        - (*as_0).power as ::core::ffi::c_int)
        < reserve_mana
    {
        if borg_spell_has_effect(spell_num, EF_NOURISH as ::core::ffi::c_int as uint16_t) {
            return true_0 != 0;
        }
        if borg_spell_has_effect(spell_num, EF_TELEPORT as ::core::ffi::c_int as uint16_t) {
            return true_0 != 0;
        }
        if MAGIC_MISSILE as ::core::ffi::c_int as ::core::ffi::c_uint
            == spell as ::core::ffi::c_uint
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                <= 35 as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_fail_rate(spell: borg_spells) -> ::core::ffi::c_int {
    let mut chance: ::core::ffi::c_int = 0;
    let mut minfail: ::core::ffi::c_int = 0;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return 100 as ::core::ffi::c_int;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    chance = (*as_0).sfail as ::core::ffi::c_int;
    chance -= 3 as ::core::ffi::c_int
        * (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - (*as_0).level as ::core::ffi::c_int);
    chance -= *borg.trait_0.offset(BI_FAIL1 as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 20 as ::core::ffi::c_int;
    }
    minfail = *borg.trait_0.offset(BI_FAIL2 as ::core::ffi::c_int as isize);
    if !flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_ZERO_FAIL as ::core::ffi::c_int,
    ) {
        if minfail < 5 as ::core::ffi::c_int {
            minfail = 5 as ::core::ffi::c_int;
        }
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
        && (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).info as ::core::ffi::c_int
            & BORG_LIGHT
            != 0
    {
        chance += 25 as ::core::ffi::c_int;
    }
    if chance < minfail {
        chance = minfail;
    }
    if chance > 50 as ::core::ffi::c_int {
        chance = 50 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 25 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
        != 0
    {
        chance += 15 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
        != 0
    {
        chance *= 2 as ::core::ffi::c_int;
    }
    if chance > 95 as ::core::ffi::c_int {
        chance = 95 as ::core::ffi::c_int;
    }
    return chance;
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_okay_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell_okay(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell_legal_fail(
    spell: borg_spells,
    mut allow_fail: ::core::ffi::c_int,
) -> bool {
    if borg_spell_fail_rate(spell) > allow_fail {
        return false_0 != 0;
    }
    return borg_spell_legal(spell);
}
#[no_mangle]
pub unsafe extern "C" fn borg_spell(spell: borg_spells) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut spell_num: ::core::ffi::c_int = borg_get_spell_number(spell);
    if spell_num < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
    if !borg_spell_okay(spell) {
        return false_0 != 0;
    }
    i = borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Casting %s (%d,%d).\0" as *const u8 as *const ::core::ffi::c_char,
        (*as_0).name,
        i,
        (*as_0).book_offset,
    ));
    borg_keypress('m' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
            as keycode_t,
    );
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
            .offset((*as_0).book_offset as isize) as keycode_t,
    );
    (*as_0).times += 1;
    return true_0 != 0;
}
unsafe extern "C" fn borg_cheat_spell(mut book_num: ::core::ffi::c_int) {
    let mut book: *mut class_book =
        (*(*player).class).magic.books.offset(book_num as isize) as *mut class_book;
    let mut spell_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell_num < (*book).num_spells {
        let mut cspell: *mut class_spell =
            (*book).spells.offset(spell_num as isize) as *mut class_spell;
        let mut as_0: *mut borg_magic =
            borg_magics.offset((*cspell).sidx as isize) as *mut borg_magic;
        if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_FORGOTTEN
            != 0
        {
            (*as_0).status = BORG_MAGIC_LOST as uint8_t;
        } else if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < (*as_0).level as ::core::ffi::c_int
        {
            (*as_0).status = BORG_MAGIC_HIGH as uint8_t;
        } else if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_LEARNED
            == 0
        {
            (*as_0).status = BORG_MAGIC_OKAY as uint8_t;
        } else if *(*player).spell_flags.offset((*cspell).sidx as isize) as ::core::ffi::c_int
            & PY_SPELL_WORKED
            == 0
        {
            (*as_0).status = BORG_MAGIC_TEST as uint8_t;
        } else {
            (*as_0).status = BORG_MAGIC_KNOW as uint8_t;
        }
        spell_num += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_cheat_spells() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        borg.book_idx[i as usize] = -(1 as ::core::ffi::c_int) as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut book_num: ::core::ffi::c_int = 0;
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        book_num = 0 as ::core::ffi::c_int;
        while book_num < (*(*player).class).magic.num_books {
            let mut book: class_book = *(*(*player).class).magic.books.offset(book_num as isize);
            if (*item).tval as ::core::ffi::c_int == book.tval
                && (*item).sval as ::core::ffi::c_int == book.sval
            {
                borg.book_idx[book_num as usize] = i as int16_t;
                break;
            } else {
                book_num += 1;
            }
        }
        i += 1;
    }
    if !borg_can_cast() {
        return;
    }
    let mut book_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book_idx < 8 as ::core::ffi::c_int {
        i = borg.book_idx[book_idx as usize] as ::core::ffi::c_int;
        if i >= 0 as ::core::ffi::c_int {
            borg_cheat_spell(book_idx);
        }
        book_idx += 1;
    }
}
unsafe extern "C" fn borg_get_book_offset(mut index: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut magic: *const class_magic = &raw const (*(*player).class).magic;
    if index < 0 as ::core::ffi::c_int || index >= (*magic).total_spells {
        return -(1 as ::core::ffi::c_int);
    }
    while (count + (*(*magic).books.offset(book as isize)).num_spells - 1 as ::core::ffi::c_int)
        < index
    {
        let fresh0 = book;
        book = book + 1;
        count += (*(*magic).books.offset(fresh0 as isize)).num_spells;
    }
    return index - count;
}
unsafe extern "C" fn borg_init_spell(
    mut spells: *mut borg_magic,
    mut spell_num: ::core::ffi::c_int,
) {
    let mut spell: *mut borg_magic = spells.offset(spell_num as isize) as *mut borg_magic;
    let mut cspell: *const class_spell = spell_by_index(player, spell_num);
    if strcmp(
        (*cspell).name,
        (*borg_spell_ratings.offset(spell_num as isize)).name,
    ) != 0
    {
        borg_note(format(
            b"**STARTUP FAILURE** spell definition mismatch. <%s> not the same as <%s>\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*cspell).name,
            (*borg_spell_ratings.offset(spell_num as isize)).name,
        ));
        borg_init_failure = true_0 != 0;
        return;
    }
    (*spell).rating = (*borg_spell_ratings.offset(spell_num as isize)).rating;
    (*spell).name = (*borg_spell_ratings.offset(spell_num as isize)).name;
    (*spell).spell_enum = (*borg_spell_ratings.offset(spell_num as isize)).spell_enum;
    (*spell).level = (*cspell).slevel as uint8_t;
    (*spell).book_offset = borg_get_book_offset((*cspell).sidx);
    (*spell).effect_index = (*(*cspell).effect).index;
    (*spell).power = (*cspell).smana as uint8_t;
    (*spell).sfail = (*cspell).sfail as uint8_t;
    (*spell).status = spell_okay_to_cast(player, spell_num) as uint8_t;
    (*spell).times = 0 as ::core::ffi::c_int as int32_t;
    (*spell).book = (*cspell).bidx;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepare_book_info() {
    match (*(*player).class).cidx {
        1 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_MAGE as *mut borg_spell_rating;
        }
        2 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_DRUID as *mut borg_spell_rating;
        }
        3 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_PRIEST as *mut borg_spell_rating;
        }
        4 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_NECROMANCER as *mut borg_spell_rating;
        }
        5 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_PALADIN as *mut borg_spell_rating;
        }
        6 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_ROGUE as *mut borg_spell_rating;
        }
        7 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_RANGER as *mut borg_spell_rating;
        }
        8 => {
            borg_spell_ratings = &raw mut borg_spell_ratings_BLACKGUARD as *mut borg_spell_rating;
        }
        _ => {
            borg_spell_ratings = ::core::ptr::null_mut::<borg_spell_rating>();
            return;
        }
    }
    if !borg_magics.is_null() {
        mem_free(borg_magics as *mut ::core::ffi::c_void);
    }
    borg_magics = mem_zalloc(
        ((*(*player).class).magic.total_spells as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_magic>() as size_t),
    ) as *mut borg_magic;
    let mut spell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while spell < (*(*player).class).magic.total_spells {
        borg_init_spell(borg_magics, spell);
        spell += 1;
    }
}
pub const BORG_MAGIC_LOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BORG_MAGIC_HIGH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BORG_MAGIC_OKAY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BORG_MAGIC_TEST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BORG_MAGIC_KNOW: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1;
pub const CLASS_DRUID: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6;
pub const CLASS_RANGER: ::core::ffi::c_int = 7;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
