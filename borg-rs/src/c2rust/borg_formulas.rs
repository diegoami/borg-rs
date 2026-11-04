#[repr(C)]
pub struct dice_s {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ang_file {
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
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn atol(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_alloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn mem_realloc(p: *mut ::core::ffi::c_void, len: size_t) -> *mut ::core::ffi::c_void;
    fn string_make(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn string_free(str: *mut ::core::ffi::c_char);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn my_stricmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn prefix_i(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn file_getl(f: *mut ang_file, buf: *mut ::core::ffi::c_char, n: size_t) -> bool;
    static mut e_info: *mut ego_item;
    static mut classes: *mut player_class;
    static mut player: *mut player;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    fn borg_item_weight(item: *mut borg_item) -> int16_t;
    static mut prefix_pref: [*const ::core::ffi::c_char; 0];
    fn spell_chance(spell_index: ::core::ffi::c_int) -> int16_t;
    fn parse_calculation_line(
        line: *mut ::core::ffi::c_char,
        full_line: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn borg_calculate_dynamic(
        formula: ::core::ffi::c_int,
        range_index: ::core::ffi::c_int,
    ) -> int32_t;
    fn calculations_free();
    static mut borg_settings: [borg_setting; 0];
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_findact(act_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn borg_ego_has_random_power(e_ptr: *mut ego_item) -> bool;
    static mut sv_potion_inc_str: ::core::ffi::c_int;
    static mut sv_potion_inc_int: ::core::ffi::c_int;
    static mut sv_potion_inc_wis: ::core::ffi::c_int;
    static mut sv_potion_inc_dex: ::core::ffi::c_int;
    static mut sv_potion_inc_con: ::core::ffi::c_int;
    static mut sv_potion_inc_all: ::core::ffi::c_int;
    static mut sv_scroll_enchant_armor: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_hit: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_dam: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_weapon: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_armor: ::core::ffi::c_int;
    fn borg_lookup_kind(tval: ::core::ffi::c_int, sval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn borg_can_cast() -> bool;
    fn borg_spell_stat() -> ::core::ffi::c_int;
    fn borg_get_spell_entry(book: ::core::ffi::c_int, what: ::core::ffi::c_int) -> *mut borg_magic;
    fn borg_trim(line: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    fn lookup_sval(
        tval: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct grouper {
    pub tval: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TV_MAX: C2RustUnnamed_0 = 36;
pub const TV_GOLD: C2RustUnnamed_0 = 35;
pub const TV_OTHER_BOOK: C2RustUnnamed_0 = 34;
pub const TV_SHADOW_BOOK: C2RustUnnamed_0 = 33;
pub const TV_NATURE_BOOK: C2RustUnnamed_0 = 32;
pub const TV_PRAYER_BOOK: C2RustUnnamed_0 = 31;
pub const TV_MAGIC_BOOK: C2RustUnnamed_0 = 30;
pub const TV_MUSHROOM: C2RustUnnamed_0 = 29;
pub const TV_FOOD: C2RustUnnamed_0 = 28;
pub const TV_FLASK: C2RustUnnamed_0 = 27;
pub const TV_POTION: C2RustUnnamed_0 = 26;
pub const TV_SCROLL: C2RustUnnamed_0 = 25;
pub const TV_ROD: C2RustUnnamed_0 = 24;
pub const TV_WAND: C2RustUnnamed_0 = 23;
pub const TV_STAFF: C2RustUnnamed_0 = 22;
pub const TV_RING: C2RustUnnamed_0 = 21;
pub const TV_AMULET: C2RustUnnamed_0 = 20;
pub const TV_LIGHT: C2RustUnnamed_0 = 19;
pub const TV_DRAG_ARMOR: C2RustUnnamed_0 = 18;
pub const TV_HARD_ARMOR: C2RustUnnamed_0 = 17;
pub const TV_SOFT_ARMOR: C2RustUnnamed_0 = 16;
pub const TV_CLOAK: C2RustUnnamed_0 = 15;
pub const TV_SHIELD: C2RustUnnamed_0 = 14;
pub const TV_CROWN: C2RustUnnamed_0 = 13;
pub const TV_HELM: C2RustUnnamed_0 = 12;
pub const TV_GLOVES: C2RustUnnamed_0 = 11;
pub const TV_BOOTS: C2RustUnnamed_0 = 10;
pub const TV_SWORD: C2RustUnnamed_0 = 9;
pub const TV_POLEARM: C2RustUnnamed_0 = 8;
pub const TV_HAFTED: C2RustUnnamed_0 = 7;
pub const TV_DIGGING: C2RustUnnamed_0 = 6;
pub const TV_BOW: C2RustUnnamed_0 = 5;
pub const TV_BOLT: C2RustUnnamed_0 = 4;
pub const TV_ARROW: C2RustUnnamed_0 = 3;
pub const TV_SHOT: C2RustUnnamed_0 = 2;
pub const TV_CHEST: C2RustUnnamed_0 = 1;
pub const TV_NULL: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PF_MAX: C2RustUnnamed_1 = 18;
pub const PF_COMBAT_REGEN: C2RustUnnamed_1 = 17;
pub const PF_EVIL: C2RustUnnamed_1 = 16;
pub const PF_SHIELD_BASH: C2RustUnnamed_1 = 15;
pub const PF_STEAL: C2RustUnnamed_1 = 14;
pub const PF_ROCK: C2RustUnnamed_1 = 13;
pub const PF_UNLIGHT: C2RustUnnamed_1 = 12;
pub const PF_CHARM: C2RustUnnamed_1 = 11;
pub const PF_NO_MANA: C2RustUnnamed_1 = 10;
pub const PF_SEE_ORE: C2RustUnnamed_1 = 9;
pub const PF_KNOW_ZAPPER: C2RustUnnamed_1 = 8;
pub const PF_KNOW_MUSHROOM: C2RustUnnamed_1 = 7;
pub const PF_CHOOSE_SPELLS: C2RustUnnamed_1 = 6;
pub const PF_BEAM: C2RustUnnamed_1 = 5;
pub const PF_ZERO_FAIL: C2RustUnnamed_1 = 4;
pub const PF_BLESS_WEAPON: C2RustUnnamed_1 = 3;
pub const PF_BRAVERY_30: C2RustUnnamed_1 = 2;
pub const PF_FAST_SHOT: C2RustUnnamed_1 = 1;
pub const PF_NONE: C2RustUnnamed_1 = 0;
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
pub struct critical_level {
    pub next: *mut critical_level,
    pub cutoff: ::core::ffi::c_int,
    pub mult: ::core::ffi::c_int,
    pub add: ::core::ffi::c_int,
    pub msgt: ::core::ffi::c_int,
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_2 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_2 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_2 = 263;
pub const BI_EMPTY: C2RustUnnamed_2 = 262;
pub const BI_CARRY: C2RustUnnamed_2 = 261;
pub const BI_WEIGHT: C2RustUnnamed_2 = 260;
pub const BI_DINV: C2RustUnnamed_2 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_2 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_2 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_2 = 256;
pub const BI_ADIGGER: C2RustUnnamed_2 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_2 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_2 = 253;
pub const BI_AROD2: C2RustUnnamed_2 = 252;
pub const BI_AROD1: C2RustUnnamed_2 = 251;
pub const BI_ASHROOM: C2RustUnnamed_2 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_2 = 249;
pub const BI_AHWORD: C2RustUnnamed_2 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_2 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_2 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_2 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_2 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_2 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_2 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_2 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_2 = 240;
pub const BI_ABRAND: C2RustUnnamed_2 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_2 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_2 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_2 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_2 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_2 = 234;
pub const BI_ACLW: C2RustUnnamed_2 = 233;
pub const BI_ACSW: C2RustUnnamed_2 = 232;
pub const BI_ACCW: C2RustUnnamed_2 = 231;
pub const BI_AGLYPH: C2RustUnnamed_2 = 230;
pub const BI_APFE: C2RustUnnamed_2 = 229;
pub const BI_ALITE: C2RustUnnamed_2 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_2 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_2 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_2 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_2 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_2 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_2 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_2 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_2 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_2 = 219;
pub const BI_ASPEED: C2RustUnnamed_2 = 218;
pub const BI_AID: C2RustUnnamed_2 = 217;
pub const BI_ALIFE: C2RustUnnamed_2 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_2 = 215;
pub const BI_AHEAL: C2RustUnnamed_2 = 214;
pub const BI_AFUEL: C2RustUnnamed_2 = 213;
pub const BI_AESCAPE: C2RustUnnamed_2 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_2 = 211;
pub const BI_APHASE: C2RustUnnamed_2 = 210;
pub const BI_WB_POIS: C2RustUnnamed_2 = 209;
pub const BI_WB_COLD: C2RustUnnamed_2 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_2 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_2 = 206;
pub const BI_WB_ACID: C2RustUnnamed_2 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_2 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_2 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_2 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_2 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_2 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_2 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_2 = 198;
pub const BI_WS_ORC: C2RustUnnamed_2 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_2 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_2 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_2 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_2 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_2 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_2 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_2 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_2 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_2 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_2 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_2 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_2 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_2 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_2 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_2 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_2 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_2 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_2 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_2 = 178;
pub const BI_CRSSICK: C2RustUnnamed_2 = 177;
pub const BI_CRSDULL: C2RustUnnamed_2 = 176;
pub const BI_CRSVULN: C2RustUnnamed_2 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_2 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_2 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_2 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_2 = 171;
pub const BI_CRSSUND: C2RustUnnamed_2 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_2 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_2 = 168;
pub const BI_CRSPARA: C2RustUnnamed_2 = 167;
pub const BI_CRSHALU: C2RustUnnamed_2 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_2 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_2 = 164;
pub const BI_CRSTELE: C2RustUnnamed_2 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_2 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_2 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_2 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_2 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_2 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_2 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_2 = 156;
pub const BI_AMISSILES: C2RustUnnamed_2 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_2 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_2 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_2 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_2 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_2 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_2 = 149;
pub const BI_SHOTS: C2RustUnnamed_2 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_2 = 147;
pub const BI_BLOWS: C2RustUnnamed_2 = 146;
pub const BI_BART: C2RustUnnamed_2 = 145;
pub const BI_SLING: C2RustUnnamed_2 = 144;
pub const BI_BTODAM: C2RustUnnamed_2 = 143;
pub const BI_BTOHIT: C2RustUnnamed_2 = 142;
pub const BI_BID: C2RustUnnamed_2 = 141;
pub const BI_WDS: C2RustUnnamed_2 = 140;
pub const BI_WDD: C2RustUnnamed_2 = 139;
pub const BI_WID: C2RustUnnamed_2 = 138;
pub const BI_WTODAM: C2RustUnnamed_2 = 137;
pub const BI_WTOHIT: C2RustUnnamed_2 = 136;
pub const BI_TODAM: C2RustUnnamed_2 = 135;
pub const BI_TOHIT: C2RustUnnamed_2 = 134;
pub const BI_ARMOR: C2RustUnnamed_2 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_2 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_2 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_2 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_2 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_2 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_2 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_2 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_2 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_2 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_2 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_2 = 122;
pub const BI_ISFORGET: C2RustUnnamed_2 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_2 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_2 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_2 = 118;
pub const BI_ISSTUN: C2RustUnnamed_2 = 117;
pub const BI_ISCUT: C2RustUnnamed_2 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_2 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_2 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_2 = 113;
pub const BI_ISBLIND: C2RustUnnamed_2 = 112;
pub const BI_ISGORGED: C2RustUnnamed_2 = 111;
pub const BI_ISFULL: C2RustUnnamed_2 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_2 = 109;
pub const BI_ISWEAK: C2RustUnnamed_2 = 108;
pub const BI_KING: C2RustUnnamed_2 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_2 = 106;
pub const BI_CDEPTH: C2RustUnnamed_2 = 105;
pub const BI_SFRACT: C2RustUnnamed_2 = 104;
pub const BI_SHLIFE: C2RustUnnamed_2 = 103;
pub const BI_SRDIS: C2RustUnnamed_2 = 102;
pub const BI_SRKAOS: C2RustUnnamed_2 = 101;
pub const BI_SRNTHR: C2RustUnnamed_2 = 100;
pub const BI_SRNXUS: C2RustUnnamed_2 = 99;
pub const BI_SRSHRD: C2RustUnnamed_2 = 98;
pub const BI_SRSND: C2RustUnnamed_2 = 97;
pub const BI_SRCONF: C2RustUnnamed_2 = 96;
pub const BI_SRBLIND: C2RustUnnamed_2 = 95;
pub const BI_SRDARK: C2RustUnnamed_2 = 94;
pub const BI_SRLITE: C2RustUnnamed_2 = 93;
pub const BI_SRFEAR: C2RustUnnamed_2 = 92;
pub const BI_SRPOIS: C2RustUnnamed_2 = 91;
pub const BI_SRACID: C2RustUnnamed_2 = 90;
pub const BI_SRELEC: C2RustUnnamed_2 = 89;
pub const BI_SRCOLD: C2RustUnnamed_2 = 88;
pub const BI_SRFIRE: C2RustUnnamed_2 = 87;
pub const BI_FRACT: C2RustUnnamed_2 = 86;
pub const BI_HLIFE: C2RustUnnamed_2 = 85;
pub const BI_RDIS: C2RustUnnamed_2 = 84;
pub const BI_RKAOS: C2RustUnnamed_2 = 83;
pub const BI_RNTHR: C2RustUnnamed_2 = 82;
pub const BI_RNXUS: C2RustUnnamed_2 = 81;
pub const BI_RSHRD: C2RustUnnamed_2 = 80;
pub const BI_RSND: C2RustUnnamed_2 = 79;
pub const BI_RCONF: C2RustUnnamed_2 = 78;
pub const BI_RBLIND: C2RustUnnamed_2 = 77;
pub const BI_RDARK: C2RustUnnamed_2 = 76;
pub const BI_RLITE: C2RustUnnamed_2 = 75;
pub const BI_RFEAR: C2RustUnnamed_2 = 74;
pub const BI_RPOIS: C2RustUnnamed_2 = 73;
pub const BI_RACID: C2RustUnnamed_2 = 72;
pub const BI_RELEC: C2RustUnnamed_2 = 71;
pub const BI_RCOLD: C2RustUnnamed_2 = 70;
pub const BI_RFIRE: C2RustUnnamed_2 = 69;
pub const BI_IPOIS: C2RustUnnamed_2 = 68;
pub const BI_IELEC: C2RustUnnamed_2 = 67;
pub const BI_ICOLD: C2RustUnnamed_2 = 66;
pub const BI_IACID: C2RustUnnamed_2 = 65;
pub const BI_IFIRE: C2RustUnnamed_2 = 64;
pub const BI_DIG: C2RustUnnamed_2 = 63;
pub const BI_THT: C2RustUnnamed_2 = 62;
pub const BI_THB: C2RustUnnamed_2 = 61;
pub const BI_THN: C2RustUnnamed_2 = 60;
pub const BI_SRCH: C2RustUnnamed_2 = 59;
pub const BI_STL: C2RustUnnamed_2 = 58;
pub const BI_SAV: C2RustUnnamed_2 = 57;
pub const BI_DEV: C2RustUnnamed_2 = 56;
pub const BI_DISM: C2RustUnnamed_2 = 55;
pub const BI_DISP: C2RustUnnamed_2 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_2 = 53;
pub const BI_INFRA: C2RustUnnamed_2 = 52;
pub const BI_SINV: C2RustUnnamed_2 = 51;
pub const BI_REG: C2RustUnnamed_2 = 50;
pub const BI_FEATH: C2RustUnnamed_2 = 49;
pub const BI_SDIG: C2RustUnnamed_2 = 48;
pub const BI_DAM_RED: C2RustUnnamed_2 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_2 = 46;
pub const BI_GOLD: C2RustUnnamed_2 = 45;
pub const BI_SPEED: C2RustUnnamed_2 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_2 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_2 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_2 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_2 = 40;
pub const BI_FOOD: C2RustUnnamed_2 = 39;
pub const BI_RECALL: C2RustUnnamed_2 = 38;
pub const BI_ESP: C2RustUnnamed_2 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_2 = 36;
pub const BI_CLEVEL: C2RustUnnamed_2 = 35;
pub const BI_FAIL2: C2RustUnnamed_2 = 34;
pub const BI_FAIL1: C2RustUnnamed_2 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_2 = 32;
pub const BI_MAXSP: C2RustUnnamed_2 = 31;
pub const BI_CURSP: C2RustUnnamed_2 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_2 = 29;
pub const BI_MAXHP: C2RustUnnamed_2 = 28;
pub const BI_CURHP: C2RustUnnamed_2 = 27;
pub const BI_LIGHT: C2RustUnnamed_2 = 26;
pub const BI_CLASS: C2RustUnnamed_2 = 25;
pub const BI_SCON: C2RustUnnamed_2 = 24;
pub const BI_SDEX: C2RustUnnamed_2 = 23;
pub const BI_SWIS: C2RustUnnamed_2 = 22;
pub const BI_SINT: C2RustUnnamed_2 = 21;
pub const BI_SSTR: C2RustUnnamed_2 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_2 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_2 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_2 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_2 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_2 = 15;
pub const BI_CCON: C2RustUnnamed_2 = 14;
pub const BI_CDEX: C2RustUnnamed_2 = 13;
pub const BI_CWIS: C2RustUnnamed_2 = 12;
pub const BI_CINT: C2RustUnnamed_2 = 11;
pub const BI_CSTR: C2RustUnnamed_2 = 10;
pub const BI_ACON: C2RustUnnamed_2 = 9;
pub const BI_ADEX: C2RustUnnamed_2 = 8;
pub const BI_AWIS: C2RustUnnamed_2 = 7;
pub const BI_AINT: C2RustUnnamed_2 = 6;
pub const BI_ASTR: C2RustUnnamed_2 = 5;
pub const BI_CON: C2RustUnnamed_2 = 4;
pub const BI_DEX: C2RustUnnamed_2 = 3;
pub const BI_WIS: C2RustUnnamed_2 = 2;
pub const BI_INT: C2RustUnnamed_2 = 1;
pub const BI_STR: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_array {
    pub max: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type value_type = ::core::ffi::c_int;
pub const VT_MAX: value_type = 41;
pub const VT_GOLD: value_type = 40;
pub const VT_OTHER_BOOK: value_type = 39;
pub const VT_SHADOW_BOOK: value_type = 38;
pub const VT_NATURE_BOOK: value_type = 37;
pub const VT_PRAYER_BOOK: value_type = 36;
pub const VT_MAGIC_BOOK: value_type = 35;
pub const VT_MUSHROOM: value_type = 34;
pub const VT_FOOD: value_type = 33;
pub const VT_FLASK: value_type = 32;
pub const VT_POTION: value_type = 31;
pub const VT_SCROLL: value_type = 30;
pub const VT_ROD: value_type = 29;
pub const VT_WAND: value_type = 28;
pub const VT_STAFF: value_type = 27;
pub const VT_RING: value_type = 26;
pub const VT_AMULET: value_type = 25;
pub const VT_LIGHT: value_type = 24;
pub const VT_DRAG_ARMOR: value_type = 23;
pub const VT_HARD_ARMOR: value_type = 22;
pub const VT_SOFT_ARMOR: value_type = 21;
pub const VT_CLOAK: value_type = 20;
pub const VT_SHIELD: value_type = 19;
pub const VT_CROWN: value_type = 18;
pub const VT_HELM: value_type = 17;
pub const VT_GLOVES: value_type = 16;
pub const VT_BOOTS: value_type = 15;
pub const VT_SWORD: value_type = 14;
pub const VT_POLEARM: value_type = 13;
pub const VT_HAFTED: value_type = 12;
pub const VT_DIGGING: value_type = 11;
pub const VT_BOW: value_type = 10;
pub const VT_BOLT: value_type = 9;
pub const VT_ARROW: value_type = 8;
pub const VT_SHOT: value_type = 7;
pub const VT_CHEST: value_type = 6;
pub const VT_NULL: value_type = 5;
pub const VT_CLASS: value_type = 4;
pub const VT_ACTIVATION: value_type = 3;
pub const VT_CONFIG: value_type = 2;
pub const VT_TRAIT: value_type = 1;
pub const VT_RANGE_INDEX: value_type = 0;
pub const VT_NONE: value_type = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct value_sec {
    pub type_0: value_type,
    pub index: int32_t,
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
    pub temp: C2RustUnnamed_10,
    pub status: C2RustUnnamed_9,
    pub time: C2RustUnnamed_8,
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
    pub goal: C2RustUnnamed_7,
    pub init_save: C2RustUnnamed_6,
    pub mon: C2RustUnnamed_5,
    pub id: C2RustUnnamed_4,
    pub near: C2RustUnnamed_3,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
pub struct borg_setting {
    pub setting_string: *const ::core::ffi::c_char,
    pub setting_type: ::core::ffi::c_char,
    pub default_value: ::core::ffi::c_int,
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
pub struct borg_power_line {
    pub range: *mut range_sec,
    pub reward: *mut math_section,
    pub value: *mut value_sec,
    pub condition: *mut math_section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct math_section {
    pub is_calculation: bool,
    pub calculation_index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_sec {
    pub from: int32_t,
    pub to: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_formula_arrays {
    pub power: borg_array,
    pub depth: borg_array,
    pub restock: borg_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_depth_line {
    pub dlevel: int32_t,
    pub reason: *mut ::core::ffi::c_char,
    pub condition: *mut math_section,
}
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_11 = 13;
pub const SEC_NONE: e_section = 0;
pub type e_section = ::core::ffi::c_uint;
pub const SEC_RESTOCK: e_section = 3;
pub const SEC_DEPTH: e_section = 2;
pub const SEC_POWER: e_section = 1;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_11 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_11 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_11 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_11 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_11 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_11 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_11 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_11 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_11 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_11 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_11 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_11 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_11 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_11 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_11 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_11 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_11 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_11 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_11 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_11 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_11 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_11 = 14;
pub const BORG_USES_SWAPS: C2RustUnnamed_11 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_11 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_11 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_11 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_11 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_11 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_11 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_11 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_11 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_11 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_11 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_11 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_11 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut value_type_names: [grouper; 42] = [
    grouper {
        tval: VT_NONE as ::core::ffi::c_int,
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_RANGE_INDEX as ::core::ffi::c_int,
        name: b"range\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_TRAIT as ::core::ffi::c_int,
        name: b"trait\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CONFIG as ::core::ffi::c_int,
        name: b"config\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ACTIVATION as ::core::ffi::c_int,
        name: b"activation\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CLASS as ::core::ffi::c_int,
        name: b"class\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_NULL as ::core::ffi::c_int,
        name: b"none\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CHEST as ::core::ffi::c_int,
        name: b"chest\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHOT as ::core::ffi::c_int,
        name: b"shot\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ARROW as ::core::ffi::c_int,
        name: b"arrow\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOLT as ::core::ffi::c_int,
        name: b"bolt\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOW as ::core::ffi::c_int,
        name: b"bow\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_DIGGING as ::core::ffi::c_int,
        name: b"digger\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HAFTED as ::core::ffi::c_int,
        name: b"hafted\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_POLEARM as ::core::ffi::c_int,
        name: b"polearm\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SWORD as ::core::ffi::c_int,
        name: b"sword\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_BOOTS as ::core::ffi::c_int,
        name: b"boots\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_GLOVES as ::core::ffi::c_int,
        name: b"gloves\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HELM as ::core::ffi::c_int,
        name: b"helm\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CROWN as ::core::ffi::c_int,
        name: b"crown\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHIELD as ::core::ffi::c_int,
        name: b"shield\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_CLOAK as ::core::ffi::c_int,
        name: b"cloak\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SOFT_ARMOR as ::core::ffi::c_int,
        name: b"soft armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_HARD_ARMOR as ::core::ffi::c_int,
        name: b"hard armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_DRAG_ARMOR as ::core::ffi::c_int,
        name: b"dragon armor\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_LIGHT as ::core::ffi::c_int,
        name: b"light\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_AMULET as ::core::ffi::c_int,
        name: b"amulet\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_RING as ::core::ffi::c_int,
        name: b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_STAFF as ::core::ffi::c_int,
        name: b"staff\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_WAND as ::core::ffi::c_int,
        name: b"wand\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_ROD as ::core::ffi::c_int,
        name: b"rod\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SCROLL as ::core::ffi::c_int,
        name: b"scroll\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_POTION as ::core::ffi::c_int,
        name: b"potion\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_FLASK as ::core::ffi::c_int,
        name: b"flask\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_FOOD as ::core::ffi::c_int,
        name: b"food\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_MUSHROOM as ::core::ffi::c_int,
        name: b"mushroom\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_MAGIC_BOOK as ::core::ffi::c_int,
        name: b"magic book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_PRAYER_BOOK as ::core::ffi::c_int,
        name: b"prayer book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_NATURE_BOOK as ::core::ffi::c_int,
        name: b"nature book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_SHADOW_BOOK as ::core::ffi::c_int,
        name: b"shadow book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_OTHER_BOOK as ::core::ffi::c_int,
        name: b"other book\0" as *const u8 as *const ::core::ffi::c_char,
    },
    grouper {
        tval: VT_GOLD as ::core::ffi::c_int,
        name: b"gold\0" as *const u8 as *const ::core::ffi::c_char,
    },
];
#[no_mangle]
pub static mut borg_formulas: borg_formula_arrays = borg_formula_arrays {
    power: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
    depth: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
    restock: borg_array {
        max: 0,
        count: 0,
        items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    },
};
#[no_mangle]
pub unsafe extern "C" fn borg_array_add(
    mut a: *mut borg_array,
    mut item: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*a).count == 0 {
        (*a).max = 10 as ::core::ffi::c_int;
        (*a).items = mem_alloc(
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul((*a).max as size_t),
        ) as *mut *mut ::core::ffi::c_void;
    }
    if (*a).max < (*a).count + 1 as ::core::ffi::c_int {
        (*a).max += 10 as ::core::ffi::c_int;
        (*a).items = mem_realloc(
            (*a).items as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul((*a).max as size_t),
        ) as *mut *mut ::core::ffi::c_void;
    }
    let ref mut fresh0 = *(*a).items.offset((*a).count as isize);
    *fresh0 = item;
    let fresh1 = (*a).count;
    (*a).count = (*a).count + 1;
    return fresh1;
}
unsafe extern "C" fn borg_get_class(
    mut class_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: *mut player_class = ::core::ptr::null_mut::<player_class>();
    c = classes;
    while !c.is_null() {
        if my_stricmp((*c).name, class_name) == 0 {
            return (*c).cidx as ::core::ffi::c_int;
        }
        c = (*c).next;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_formula_error(
    mut section: *const ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
    mut section_label: *const ::core::ffi::c_char,
    mut error: *const ::core::ffi::c_char,
) {
    borg_note(b"** borg formula failure ** \0" as *const u8 as *const ::core::ffi::c_char);
    borg_note(format(
        b"** error on line '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        full_line,
    ));
    borg_note(format(
        b"** error for section %s '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        section_label,
        section,
    ));
    if !error.is_null() {
        borg_note(error);
    }
    borg_note(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn parse_depth(mut line: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    line = strchr(line, '(' as i32);
    if line.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_long = strtol(line, &raw mut end, 10 as ::core::ffi::c_int);
    if end == line {
        return -(1 as ::core::ffi::c_int);
    }
    while *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        end = end.offset(1);
    }
    if *end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ')' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    return l as ::core::ffi::c_int;
}
unsafe extern "C" fn is_all_number(mut line: *mut ::core::ffi::c_char) -> bool {
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    if *line != 0 {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn parse_formula_section(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut formula: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    if is_all_number(line) {
        let mut c: *mut math_section =
            mem_alloc(::core::mem::size_of::<math_section>() as size_t) as *mut math_section;
        (*c).calculation_index = atol(line) as int32_t;
        (*c).is_calculation = false_0 != 0;
        return c;
    }
    formula = parse_calculation_line(line, full_line);
    if formula == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<math_section>();
    }
    let mut c_0: *mut math_section =
        mem_alloc(::core::mem::size_of::<math_section>() as size_t) as *mut math_section;
    (*c_0).calculation_index = formula as int32_t;
    (*c_0).is_calculation = true_0 != 0;
    return c_0;
}
unsafe extern "C" fn parse_condition(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut start: *mut ::core::ffi::c_char = line;
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    line = line.offset(1);
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    return parse_formula_section(line, full_line);
}
unsafe extern "C" fn parse_reward(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut math_section {
    let mut start: *mut ::core::ffi::c_char = line;
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no starting '('\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    line = line.offset(1);
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<math_section>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    return parse_formula_section(line, full_line);
}
unsafe extern "C" fn parse_range(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut range_sec {
    let mut from: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut start: *mut ::core::ffi::c_char = line;
    let mut val1str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val2str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no starting '('\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val1str = line;
    line = strchr(line, ',' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** range needs two numbers separated by a ','\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    *line = 0 as ::core::ffi::c_char;
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val2str = line;
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** first value is not a number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    if !is_all_number(val1str) {
        return ::core::ptr::null_mut::<range_sec>();
    }
    if !is_all_number(val2str) {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value is not a number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    from = atol(val1str) as ::core::ffi::c_int;
    to = atol(val2str) as ::core::ffi::c_int;
    if from < 1 as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** first value less than one\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    if to < 0 as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value less than one\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    if to < from {
        borg_formula_error(
            start,
            full_line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            b"** second value less first value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<range_sec>();
    }
    let mut r: *mut range_sec =
        mem_alloc(::core::mem::size_of::<range_sec>() as size_t) as *mut range_sec;
    (*r).from = from as int32_t;
    (*r).to = to as int32_t;
    return r;
}
unsafe extern "C" fn get_value_type(mut value_type_name: *mut ::core::ffi::c_char) -> value_type {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[grouper; 42]>() as usize)
            .wrapping_div(::core::mem::size_of::<grouper>() as usize)
            as ::core::ffi::c_int
    {
        if my_stricmp(value_type_name, value_type_names[i as usize].name) == 0 {
            return value_type_names[i as usize].tval as value_type;
        }
        i += 1;
    }
    return VT_NONE;
}
unsafe extern "C" fn get_value_index(
    mut t: value_type,
    mut name: *const ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    match t as ::core::ffi::c_int {
        3 => {
            i = borg_findact(name);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** unable to find activation\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return i;
        }
        1 => {
            i = 0 as ::core::ffi::c_int;
            while i < BI_MAX as ::core::ffi::c_int {
                if my_stricmp(
                    *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char).offset(i as isize),
                    name,
                ) == 0
                {
                    return i;
                }
                i += 1;
            }
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** unable to find trait\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        2 => {
            i = 0 as ::core::ffi::c_int;
            while !(*(&raw mut borg_settings as *mut borg_setting).offset(i as isize))
                .setting_string
                .is_null()
            {
                if my_stricmp(
                    (*(&raw mut borg_settings as *mut borg_setting).offset(i as isize))
                        .setting_string
                        .offset(5 as ::core::ffi::c_int as isize),
                    name,
                ) == 0
                {
                    return i;
                }
                i += 1;
            }
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** unable to find config\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        0 => return 1 as ::core::ffi::c_int,
        4 => {
            i = borg_get_class(name);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** invalid class\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return i;
        }
        -1 | 5 | 41 => {
            borg_formula_error(
                name,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** invalid value type\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            let mut tval: ::core::ffi::c_int =
                t as ::core::ffi::c_int - VT_NULL as ::core::ffi::c_int;
            let mut sval: ::core::ffi::c_int = lookup_sval(tval, name);
            if sval == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    format(
                        b"** value error for object type '%s' subtype '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        value_type_names[t as usize].name,
                        name,
                    ),
                );
                return -(1 as ::core::ffi::c_int);
            }
            i = borg_lookup_kind(tval, sval);
            if i == -(1 as ::core::ffi::c_int) {
                borg_formula_error(
                    name,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    format(
                        b"** value error for object type '%s' subtype '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        value_type_names[t as usize].name,
                        name,
                    ),
                );
            }
            return i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn calculate_from_value(
    mut value: *mut value_sec,
    mut range_index: ::core::ffi::c_int,
) -> int32_t {
    match (*value).type_0 as ::core::ffi::c_int {
        3 => return *borg.activation.offset((*value).index as isize) as int32_t,
        1 => return *borg.trait_0.offset((*value).index as isize) as int32_t,
        2 => return *borg_cfg.offset((*value).index as isize) as int32_t,
        4 => {
            return (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) as int32_t
                == (*value).index) as ::core::ffi::c_int;
        }
        0 => return range_index as int32_t,
        _ => return *borg.has.offset((*value).index as isize) as int32_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_value(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> *mut value_sec {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut val1str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val2str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line = strchr(line, '(' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val1str = line;
    line = strchr(line, ',' as i32);
    if line.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** value needs two values separated by a ',' \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    *line = 0 as ::core::ffi::c_char;
    line = line.offset(1);
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    val2str = line;
    if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** no ending ')' \0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    let mut t: value_type = get_value_type(val1str);
    if t as ::core::ffi::c_int == VT_NONE as ::core::ffi::c_int {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            format(
                b"** value type (first parameter '%s') not found\0" as *const u8
                    as *const ::core::ffi::c_char,
                val1str,
            ),
        );
        return ::core::ptr::null_mut::<value_sec>();
    }
    let mut index: ::core::ffi::c_int = get_value_index(t, val2str, full_line);
    if index == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<value_sec>();
    }
    let mut v: *mut value_sec =
        mem_alloc(::core::mem::size_of::<value_sec>() as size_t) as *mut value_sec;
    (*v).type_0 = t;
    (*v).index = index as int32_t;
    return v;
}
unsafe extern "C" fn get_section(
    mut line: *mut ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    if prefix_i(line, name) {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        if !(*value).is_null() {
            borg_formula_error(
                start,
                full_line,
                b"value\0" as *const u8 as *const ::core::ffi::c_char,
                b"** section repeated\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        let mut len: ::core::ffi::c_int = 0;
        let mut pos: *mut ::core::ffi::c_char = line;
        line = strchr(line, ':' as i32);
        if !line.is_null() {
            len = line.offset_from(pos) as ::core::ffi::c_long as ::core::ffi::c_int;
        } else {
            len = strlen(pos) as ::core::ffi::c_int;
        }
        if *pos.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ';' as i32
        {
            len -= 1;
        }
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, pos, len as size_t);
        buf[len as usize] = 0 as ::core::ffi::c_char;
        *value = string_make(&raw mut buf as *mut ::core::ffi::c_char);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn depth_free(mut depth: *mut borg_depth_line) {
    if !depth.is_null() {
        if !(*depth).condition.is_null() {
            mem_free((*depth).condition as *mut ::core::ffi::c_void);
            (*depth).condition = ::core::ptr::null_mut::<math_section>();
        }
        if !(*depth).reason.is_null() {
            string_free((*depth).reason);
            (*depth).reason = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !depth.is_null() {
            mem_free(depth as *mut ::core::ffi::c_void);
        }
        depth = ::core::ptr::null_mut::<borg_depth_line>();
    }
}
unsafe extern "C" fn parse_depth_line(
    mut restock: bool,
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut fail: bool = false_0 != 0;
    let mut s_depth: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_condition: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *line != 0 {
        if get_section(
            line,
            b"depth\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_depth,
            full_line,
        ) {
            line = line.offset(strlen(s_depth) as isize);
        }
        if get_section(
            line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_condition,
            full_line,
        ) {
            line = line.offset(strlen(s_condition) as isize);
        }
        if *line as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ':' as i32
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ';' as i32
        {
            let mut err: *mut ::core::ffi::c_char = string_make(format(
                b"** unparsed information '%s' in depth line\0" as *const u8
                    as *const ::core::ffi::c_char,
                line,
            ));
            borg_formula_error(
                start,
                full_line,
                b"depth\0" as *const u8 as *const ::core::ffi::c_char,
                err,
            );
            string_free(err);
            fail = true_0 != 0;
            break;
        } else {
            line = line.offset(1);
        }
    }
    if !fail && s_depth.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** depth section missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    if !fail && s_condition.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            b"** condition section missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    let mut d: *mut borg_depth_line = ::core::ptr::null_mut::<borg_depth_line>();
    if !fail {
        d = mem_zalloc(::core::mem::size_of::<borg_depth_line>() as size_t) as *mut borg_depth_line;
    }
    if !s_depth.is_null() {
        if !fail {
            (*d).dlevel = parse_depth(s_depth) as int32_t;
            if (*d).dlevel < 0 as int32_t {
                borg_formula_error(
                    start,
                    full_line,
                    b"value\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** depth section can't be parsed\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fail = true_0 != 0;
            }
        }
        string_free(s_depth);
    }
    if !s_condition.is_null() {
        if !fail {
            (*d).condition = parse_condition(s_condition, full_line);
            (*d).reason = s_condition;
            if (*d).condition.is_null() {
                fail = true_0 != 0;
            }
        }
    }
    if !fail {
        if restock {
            borg_array_add(
                &raw mut borg_formulas.restock,
                d as *mut ::core::ffi::c_void,
            );
        } else {
            borg_array_add(&raw mut borg_formulas.depth, d as *mut ::core::ffi::c_void);
        }
    }
    if fail {
        depth_free(d);
    }
    return fail;
}
unsafe extern "C" fn parse_power_line(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut start: *mut ::core::ffi::c_char = line;
    let mut fail: bool = false_0 != 0;
    let mut s_range: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_reward: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s_condition: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *line != 0 {
        if get_section(
            line,
            b"range\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_range,
            full_line,
        ) {
            line = line.offset(strlen(s_range) as isize);
        }
        if get_section(
            line,
            b"reward\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_reward,
            full_line,
        ) {
            line = line.offset(strlen(s_reward) as isize);
        }
        if get_section(
            line,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_value,
            full_line,
        ) {
            line = line.offset(strlen(s_value) as isize);
        }
        if get_section(
            line,
            b"condition\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s_condition,
            full_line,
        ) {
            line = line.offset(strlen(s_condition) as isize);
        }
        if *line as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ':' as i32
            && *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ';' as i32
        {
            let mut err: *mut ::core::ffi::c_char = string_make(format(
                b"** unparsed information '%s' in power line\0" as *const u8
                    as *const ::core::ffi::c_char,
                line,
            ));
            borg_formula_error(
                start,
                full_line,
                b"power\0" as *const u8 as *const ::core::ffi::c_char,
                err,
            );
            string_free(err);
            fail = true_0 != 0;
            break;
        } else {
            line = line.offset(1);
        }
    }
    if !fail && !s_range.is_null() && s_value.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"power\0" as *const u8 as *const ::core::ffi::c_char,
            b"** if range() is specified value() must also be specified\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    if !fail && s_reward.is_null() {
        borg_formula_error(
            start,
            full_line,
            b"power\0" as *const u8 as *const ::core::ffi::c_char,
            b"** reward() must be specified specified\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    let mut p: *mut borg_power_line = ::core::ptr::null_mut::<borg_power_line>();
    if !fail {
        p = mem_zalloc(::core::mem::size_of::<borg_power_line>() as size_t) as *mut borg_power_line;
    }
    if !s_range.is_null() {
        if !fail {
            (*p).range = parse_range(s_range, full_line);
            if (*p).range.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_range);
    }
    if !s_reward.is_null() {
        if !fail {
            (*p).reward = parse_reward(s_reward, full_line);
            if (*p).reward.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_reward);
    }
    if !s_value.is_null() {
        if !fail {
            (*p).value = parse_value(s_value, full_line);
            if (*p).value.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_value);
    }
    if !s_condition.is_null() {
        if !fail {
            (*p).condition = parse_condition(s_condition, full_line);
            if (*p).condition.is_null() {
                fail = true_0 != 0;
            }
        }
        string_free(s_condition);
    }
    if !fail {
        borg_array_add(&raw mut borg_formulas.power, p as *mut ::core::ffi::c_void);
    }
    if fail as ::core::ffi::c_int != 0 && !p.is_null() {
        mem_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<borg_power_line>();
    }
    return fail;
}
unsafe extern "C" fn check_condition(
    mut condition: *mut math_section,
    mut range: ::core::ffi::c_int,
) -> bool {
    if !(*condition).is_calculation {
        return (*condition).calculation_index != 0 as int32_t;
    }
    return borg_calculate_dynamic((*condition).calculation_index as ::core::ffi::c_int, range)
        != 0;
}
unsafe extern "C" fn calc_power(mut p: *mut borg_power_line) -> int32_t {
    let mut total: int32_t = 0 as int32_t;
    if !(*p).condition.is_null() {
        if !check_condition((*p).condition, 0 as ::core::ffi::c_int) {
            return 0 as int32_t;
        }
    }
    if !(*p).range.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut end: ::core::ffi::c_int =
            if calculate_from_value((*p).value, 0 as ::core::ffi::c_int) > (*(*p).range).to {
                (*(*p).range).to as ::core::ffi::c_int
            } else {
                calculate_from_value((*p).value, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            };
        i = (*(*p).range).from as ::core::ffi::c_int;
        while i <= end {
            if (*(*p).reward).is_calculation {
                total += borg_calculate_dynamic(
                    (*(*p).reward).calculation_index as ::core::ffi::c_int,
                    i - 1 as ::core::ffi::c_int,
                );
            } else {
                total += (*(*p).reward).calculation_index;
            }
            i += 1;
        }
        return total;
    }
    if (*(*p).reward).is_calculation {
        total += borg_calculate_dynamic(
            (*(*p).reward).calculation_index as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    } else {
        total += (*(*p).reward).calculation_index;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn borg_power_dynamic() -> int32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut total: int32_t = 0 as int32_t;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.power.count {
        total += calc_power(*borg_formulas.power.items.offset(i as isize) as *mut borg_power_line);
        i += 1;
    }
    if borg_spell_stat() > 0 as ::core::ffi::c_int {
        total = (total as ::core::ffi::c_int
            + (100 as ::core::ffi::c_int
                - spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int) as int32_t;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_ZERO_FAIL as ::core::ffi::c_int,
    ) {
        if (spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize))
    .iqty
        != 0
    {
        let mut item: *mut borg_item = borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item;
        if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
            as ::core::ffi::c_int
            != 0
            && !(*item).ident
        {
            total = (total as ::core::ffi::c_long + 999999 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 200 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int
                - (borg_item_weight(borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) as *mut borg_item) as ::core::ffi::c_int
                    - 200 as ::core::ffi::c_int)
                    * 15 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 30 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 50 as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
    }
    let mut cur_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    let mut max_wgt: ::core::ffi::c_int = (*(*player).class).magic.spell_weight;
    if borg_can_cast() as ::core::ffi::c_int != 0
        && (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut max_sp: ::core::ffi::c_int = *borg
            .trait_0
            .offset(BI_SP_ADJ as ::core::ffi::c_int as isize)
            / 100 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        max_sp -= (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int;
        if max_sp >= 300 as ::core::ffi::c_int && max_sp <= 350 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 400 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 200 as ::core::ffi::c_int && max_sp <= 299 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 800 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 100 as ::core::ffi::c_int && max_sp <= 199 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1600 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp <= 99 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 3200 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        <= 40 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 25 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            < 100000 as ::core::ffi::c_int
        && (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
            != TV_DIGGING as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_ADIGGER as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
    {
        total = (total as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    let mut book: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while book < 9 as ::core::ffi::c_int {
        if !(borg.amt_book[book as usize] == 0) {
            if (*(*(*player).class).magic.books.offset(book as isize)).dungeon {
                let mut what: ::core::ffi::c_int = 0;
                what = 0 as ::core::ffi::c_int;
                while what < 9 as ::core::ffi::c_int {
                    let mut as_0: *mut borg_magic = borg_get_spell_entry(book, what);
                    if as_0.is_null() {
                        break;
                    }
                    if !((*as_0).level as ::core::ffi::c_int
                        > *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize))
                    {
                        if !((*as_0).power as ::core::ffi::c_int
                            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize))
                        {
                            total = (total as ::core::ffi::c_long + 15000 as ::core::ffi::c_long)
                                as int32_t;
                        }
                    }
                    what += 1;
                }
            } else {
                let mut what_0: ::core::ffi::c_int = 0;
                let mut when: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
                what_0 = 0 as ::core::ffi::c_int;
                while what_0 < 9 as ::core::ffi::c_int {
                    let mut as_1: *mut borg_magic = borg_get_spell_entry(book, what_0);
                    if as_1.is_null() {
                        break;
                    }
                    if ((*as_1).level as ::core::ffi::c_int) < when {
                        when = (*as_1).level as ::core::ffi::c_int;
                    }
                    what_0 += 1;
                }
                if !(when > 5 as ::core::ffi::c_int
                    && when
                        >= *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                            + 2 as ::core::ffi::c_int)
                {
                    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while k < 1 as ::core::ffi::c_int
                        && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                    {
                        total = (total as ::core::ffi::c_long + 500000 as ::core::ffi::c_long)
                            as int32_t;
                        k += 1;
                    }
                    if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize)
                        > 5 as ::core::ffi::c_int
                    {
                        while k < 2 as ::core::ffi::c_int
                            && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                        {
                            total = (total as ::core::ffi::c_long + 10000 as ::core::ffi::c_long)
                                as int32_t;
                            k += 1;
                        }
                    }
                }
            }
        }
        book += 1;
    }
    if *borg
        .trait_0
        .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        let mut item_0: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
        i = (*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize);
        while i >= 0 as ::core::ffi::c_int {
            if (*borg_items.offset(i as isize)).iqty != 0 {
                item_0 = borg_items.offset(i as isize) as *mut borg_item;
                break;
            } else {
                i -= 1;
            }
        }
        if !(!item_0.is_null()
            && (*item_0).iqty as ::core::ffi::c_int != 0
            && (*item_0).aware as ::core::ffi::c_int != 0
            && ((*item_0).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                && ((*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_armor
                    && *borg
                        .trait_0
                        .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize)
                        < 1000 as ::core::ffi::c_int
                    && *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize)
                        != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_hit
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize)
                            != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_dam
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize)
                            != 0
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_star_enchant_weapon
                    || (*item_0).sval as ::core::ffi::c_int == sv_scroll_star_enchant_armor)
                || (*item_0).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && ((*item_0).sval as ::core::ffi::c_int == sv_potion_inc_str
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_int
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_wis
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_dex
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_con
                        || (*item_0).sval as ::core::ffi::c_int == sv_potion_inc_all)))
        {
            total = (total as ::core::ffi::c_long
                - ((*borg
                    .trait_0
                    .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
                    - *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int)
                    / (*borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 10 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long) as int32_t;
        }
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepared_dynamic(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.depth.count {
        let mut d: *mut borg_depth_line =
            *borg_formulas.depth.items.offset(i as isize) as *mut borg_depth_line;
        if (*d).dlevel > depth as int32_t {
            break;
        }
        if check_condition((*d).condition, depth) {
            return (*d).reason;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_restock_dynamic(
    mut depth: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.restock.count {
        let mut d: *mut borg_depth_line =
            *borg_formulas.restock.items.offset(i as isize) as *mut borg_depth_line;
        if (*d).dlevel > depth as int32_t {
            break;
        }
        if check_condition((*d).condition, depth) {
            return (*d).reason;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_load_formulas(mut fp: *mut ang_file) -> bool {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut formulas_off: bool = false_0 != 0;
    let mut section: e_section = SEC_NONE;
    let mut skip_open: bool = true_0 != 0;
    borg_formulas.restock.count = 0 as ::core::ffi::c_int;
    borg_formulas.depth.count = 0 as ::core::ffi::c_int;
    borg_formulas.power.count = 0 as ::core::ffi::c_int;
    if *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) == 0 {
        formulas_off = true_0 != 0;
    }
    while file_getl(
        fp,
        &raw mut buf as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t).wrapping_sub(1 as size_t),
    ) {
        let mut line: *mut ::core::ffi::c_char =
            borg_trim(&raw mut buf as *mut ::core::ffi::c_char);
        if *line.offset(0 as ::core::ffi::c_int as isize) == 0
            || *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if prefix_i(
            line,
            b"[END FORMULA SECTION]\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            break;
        }
        if formulas_off {
            continue;
        }
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            if !skip_open {
                formulas_off = true_0 != 0;
            }
            skip_open = false_0 != 0;
        } else if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ']' as i32
        {
            if section as ::core::ffi::c_uint
                == SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return formulas_off;
            }
            section = SEC_NONE;
        } else {
            if formulas_off {
                continue;
            }
            if prefix_i(line, b"Power\0" as *const u8 as *const ::core::ffi::c_char) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_POWER;
                skip_open = true_0 != 0;
            } else if prefix_i(
                line,
                b"Depth Requirement\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_DEPTH;
                skip_open = true_0 != 0;
            } else if prefix_i(
                line,
                b"Restock Requirement\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if section as ::core::ffi::c_uint
                    != SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = true_0 != 0;
                }
                section = SEC_RESTOCK;
                skip_open = true_0 != 0;
            } else if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                != ';' as i32
            {
                if section as ::core::ffi::c_uint
                    == SEC_DEPTH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"depth\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if section as ::core::ffi::c_uint
                    == SEC_POWER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"power\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if section as ::core::ffi::c_uint
                    == SEC_RESTOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"restock\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"base\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must end in a ;\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                formulas_off = true_0 != 0;
            } else {
                if section as ::core::ffi::c_uint
                    == SEC_DEPTH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_depth_line(
                        false_0 != 0,
                        line,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                }
                if section as ::core::ffi::c_uint
                    == SEC_RESTOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_depth_line(
                        true_0 != 0,
                        line,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                }
                if section as ::core::ffi::c_uint
                    == SEC_POWER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    formulas_off = parse_power_line(line, &raw mut buf as *mut ::core::ffi::c_char);
                }
                if section as ::core::ffi::c_uint
                    == SEC_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    borg_formula_error(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"base\0" as *const u8 as *const ::core::ffi::c_char,
                        b"** line must be part of a Power or Depth section\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    formulas_off = true_0 != 0;
                }
            }
        }
    }
    if formulas_off as ::core::ffi::c_int != 0
        && *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) != 0
    {
        *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
    }
    return formulas_off;
}
unsafe extern "C" fn power_free(mut power: *mut borg_power_line) {
    if !power.is_null() {
        if !(*power).condition.is_null() {
            mem_free((*power).condition as *mut ::core::ffi::c_void);
            (*power).condition = ::core::ptr::null_mut::<math_section>();
        }
        if !(*power).range.is_null() {
            mem_free((*power).range as *mut ::core::ffi::c_void);
            (*power).range = ::core::ptr::null_mut::<range_sec>();
        }
        if !(*power).reward.is_null() {
            mem_free((*power).reward as *mut ::core::ffi::c_void);
            (*power).reward = ::core::ptr::null_mut::<math_section>();
        }
        if !(*power).value.is_null() {
            mem_free((*power).value as *mut ::core::ffi::c_void);
            (*power).value = ::core::ptr::null_mut::<value_sec>();
        }
        if !power.is_null() {
            mem_free(power as *mut ::core::ffi::c_void);
        }
        power = ::core::ptr::null_mut::<borg_power_line>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_formulas() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.restock.count {
        depth_free(*borg_formulas.restock.items.offset(i as isize) as *mut borg_depth_line);
        let ref mut fresh2 = *borg_formulas.restock.items.offset(i as isize);
        *fresh2 = NULL;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.depth.count {
        depth_free(*borg_formulas.depth.items.offset(i as isize) as *mut borg_depth_line);
        let ref mut fresh3 = *borg_formulas.depth.items.offset(i as isize);
        *fresh3 = NULL;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_formulas.power.count {
        power_free(*borg_formulas.power.items.offset(i as isize) as *mut borg_power_line);
        let ref mut fresh4 = *borg_formulas.power.items.offset(i as isize);
        *fresh4 = NULL;
        i += 1;
    }
    borg_formulas.restock.count = 0 as ::core::ffi::c_int;
    borg_formulas.depth.count = 0 as ::core::ffi::c_int;
    borg_formulas.power.count = 0 as ::core::ffi::c_int;
    calculations_free();
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
