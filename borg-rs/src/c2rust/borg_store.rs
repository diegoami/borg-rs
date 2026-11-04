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
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn msg(fmt: *const ::core::ffi::c_char, ...);
    static mut player: *mut player;
    static mut z_info: *mut angband_constants;
    static mut f_info: *mut feature;
    fn object_desc(
        buf: *mut ::core::ffi::c_char,
        max: size_t,
        obj: *const object,
        mode: uint32_t,
        p: *const player,
    ) -> size_t;
    fn object_value_real(obj: *const object, qty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn object_value(obj: *const object, qty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tval_can_have_charges(obj: *const object) -> bool;
    static mut stores: *mut store;
    fn store_stock_list(store: *mut store, list: *mut *mut object, n: ::core::ffi::c_int);
    static mut borg_init_failure: bool;
    fn borg_first_empty_inventory_slot() -> ::core::ffi::c_int;
    fn borg_item_analyze(
        item: *mut borg_item,
        real_item: *const object,
        desc: *mut ::core::ffi::c_char,
        in_store: bool,
    );
    fn borg_init_store_sell();
    fn borg_free_store_sell();
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
pub const OPT_MAX: C2RustUnnamed = 46;
pub const OPT_birth_percent_damage: C2RustUnnamed = 45;
pub const OPT_birth_levels_persist: C2RustUnnamed = 44;
pub const OPT_birth_know_flavors: C2RustUnnamed = 43;
pub const OPT_birth_know_runes: C2RustUnnamed = 42;
pub const OPT_birth_ai_learn: C2RustUnnamed = 41;
pub const OPT_birth_start_kit: C2RustUnnamed = 40;
pub const OPT_birth_no_selling: C2RustUnnamed = 39;
pub const OPT_birth_feelings: C2RustUnnamed = 38;
pub const OPT_birth_lose_arts: C2RustUnnamed = 37;
pub const OPT_birth_stacking: C2RustUnnamed = 36;
pub const OPT_birth_no_artifacts: C2RustUnnamed = 35;
pub const OPT_birth_no_recall: C2RustUnnamed = 34;
pub const OPT_birth_force_descend: C2RustUnnamed = 33;
pub const OPT_birth_connect_stairs: C2RustUnnamed = 32;
pub const OPT_birth_randarts: C2RustUnnamed = 31;
pub const OPT_score_live: C2RustUnnamed = 30;
pub const OPT_cheat_live: C2RustUnnamed = 29;
pub const OPT_score_xtra: C2RustUnnamed = 28;
pub const OPT_cheat_xtra: C2RustUnnamed = 27;
pub const OPT_score_room: C2RustUnnamed = 26;
pub const OPT_cheat_room: C2RustUnnamed = 25;
pub const OPT_score_hear: C2RustUnnamed = 24;
pub const OPT_cheat_hear: C2RustUnnamed = 23;
pub const OPT_effective_speed: C2RustUnnamed = 22;
pub const OPT_notify_recharge: C2RustUnnamed = 21;
pub const OPT_mouse_movement: C2RustUnnamed = 20;
pub const OPT_hp_changes_color: C2RustUnnamed = 19;
pub const OPT_auto_more: C2RustUnnamed = 18;
pub const OPT_purple_uniques: C2RustUnnamed = 17;
pub const OPT_center_player: C2RustUnnamed = 16;
pub const OPT_animate_flicker: C2RustUnnamed = 15;
pub const OPT_view_yellow_light: C2RustUnnamed = 14;
pub const OPT_hybrid_walls: C2RustUnnamed = 13;
pub const OPT_solid_walls: C2RustUnnamed = 12;
pub const OPT_disturb_near: C2RustUnnamed = 11;
pub const OPT_highlight_player: C2RustUnnamed = 10;
pub const OPT_show_target: C2RustUnnamed = 9;
pub const OPT_show_flavors: C2RustUnnamed = 8;
pub const OPT_pickup_inven: C2RustUnnamed = 7;
pub const OPT_pickup_always: C2RustUnnamed = 6;
pub const OPT_use_old_target: C2RustUnnamed = 5;
pub const OPT_show_damage: C2RustUnnamed = 4;
pub const OPT_use_sound: C2RustUnnamed = 3;
pub const OPT_autoexplore_commands: C2RustUnnamed = 2;
pub const OPT_rogue_like_commands: C2RustUnnamed = 1;
pub const OPT_none: C2RustUnnamed = 0;
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
pub struct feature {
    pub name: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
    pub fidx: ::core::ffi::c_int,
    pub mimic: *mut feature,
    pub priority: uint8_t,
    pub shopnum: uint8_t,
    pub dig: uint8_t,
    pub flags: [bitflag; 4],
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
    pub walk_msg: *mut ::core::ffi::c_char,
    pub run_msg: *mut ::core::ffi::c_char,
    pub hurt_msg: *mut ::core::ffi::c_char,
    pub die_msg: *mut ::core::ffi::c_char,
    pub confused_msg: *mut ::core::ffi::c_char,
    pub look_prefix: *mut ::core::ffi::c_char,
    pub look_in_preposition: *mut ::core::ffi::c_char,
    pub resist_flag: ::core::ffi::c_int,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_0 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_0 = 24;
pub const FEAT_LAVA: C2RustUnnamed_0 = 23;
pub const FEAT_PERM: C2RustUnnamed_0 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_0 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_0 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_0 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_0 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_0 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_0 = 16;
pub const FEAT_SECRET: C2RustUnnamed_0 = 15;
pub const FEAT_HOME: C2RustUnnamed_0 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_0 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_0 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_0 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_0 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_0 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_0 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_0 = 7;
pub const FEAT_MORE: C2RustUnnamed_0 = 6;
pub const FEAT_LESS: C2RustUnnamed_0 = 5;
pub const FEAT_BROKEN: C2RustUnnamed_0 = 4;
pub const FEAT_OPEN: C2RustUnnamed_0 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_0 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_0 = 1;
pub const FEAT_NONE: C2RustUnnamed_0 = 0;
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
pub struct borg_shop {
    pub ware: *mut borg_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owner {
    pub oidx: ::core::ffi::c_uint,
    pub next: *mut owner,
    pub name: *mut ::core::ffi::c_char,
    pub max_cost: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store {
    pub owners: *mut owner,
    pub owner: *mut owner,
    pub feat: ::core::ffi::c_int,
    pub stock_num: uint8_t,
    pub stock_size: int16_t,
    pub stock: *mut object,
    pub stock_k: *mut object,
    pub always_size: size_t,
    pub always_num: size_t,
    pub always_table: *mut *mut object_kind,
    pub normal_size: size_t,
    pub normal_num: size_t,
    pub normal_table: *mut *mut object_kind,
    pub buy: *mut object_buy,
    pub turnover: ::core::ffi::c_int,
    pub normal_stock_min: ::core::ffi::c_int,
    pub normal_stock_max: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_buy {
    pub next: *mut object_buy,
    pub tval: size_t,
    pub flag: size_t,
}
pub const ODESC_STORE: C2RustUnnamed_1 = 4;
pub const ODESC_FULL: C2RustUnnamed_1 = 3;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const ODESC_ALTNUM: C2RustUnnamed_1 = 1024;
pub const ODESC_NOEGO: C2RustUnnamed_1 = 512;
pub const ODESC_TERSE: C2RustUnnamed_1 = 256;
pub const ODESC_CAPITAL: C2RustUnnamed_1 = 128;
pub const ODESC_PREFIX: C2RustUnnamed_1 = 64;
pub const ODESC_SPOIL: C2RustUnnamed_1 = 32;
pub const ODESC_SINGULAR: C2RustUnnamed_1 = 16;
pub const ODESC_PLURAL: C2RustUnnamed_1 = 8;
pub const ODESC_EXTRA: C2RustUnnamed_1 = 2;
pub const ODESC_COMBAT: C2RustUnnamed_1 = 1;
pub const ODESC_BASE: C2RustUnnamed_1 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut SHOP_MENU_ITEMS: *const ::core::ffi::c_char =
    b"acfhjmnoqruvyzABDFGHJKLMNOPQRSTUVWXYZ\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_shops: *mut borg_shop = ::core::ptr::null::<borg_shop>() as *mut borg_shop;
#[no_mangle]
pub static mut borg_safe_shops: *mut borg_shop = ::core::ptr::null::<borg_shop>() as *mut borg_shop;
#[no_mangle]
pub unsafe extern "C" fn borg_home_full() -> bool {
    return (*(*borg_shops.offset(BORG_HOME as isize)).ware.offset(
        ((*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inventory_full() -> bool {
    return borg_first_empty_inventory_slot() == -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn borg_price_item(
    mut obj: *const object,
    mut store_buying: bool,
    mut qty: ::core::ffi::c_int,
    mut this_store: ::core::ffi::c_int,
) -> int32_t {
    let mut adjust: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    let mut price: ::core::ffi::c_int = 0;
    let mut proprietor: *mut owner = ::core::ptr::null_mut::<owner>();
    if this_store
        == (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        return 0 as int32_t;
    }
    proprietor = (*stores.offset(this_store as isize)).owner;
    if tval_can_have_charges(obj) {
        price = if object_value_real(obj, qty) > object_value(obj, qty) {
            object_value(obj, qty)
        } else {
            object_value_real(obj, qty)
        };
    } else {
        price = if object_value_real(obj, 1 as ::core::ffi::c_int)
            > object_value(obj, 1 as ::core::ffi::c_int)
        {
            object_value(obj, 1 as ::core::ffi::c_int)
        } else {
            object_value_real(obj, 1 as ::core::ffi::c_int)
        };
    }
    if price <= 0 as ::core::ffi::c_int {
        return 0 as int32_t;
    }
    if this_store
        == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        adjust = 150 as ::core::ffi::c_int;
    }
    if store_buying {
        adjust = 100 as ::core::ffi::c_int + (100 as ::core::ffi::c_int - adjust);
        if adjust > 100 as ::core::ffi::c_int {
            adjust = 100 as ::core::ffi::c_int;
        }
        price = price * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if this_store
            == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            price = price / 2 as ::core::ffi::c_int;
        }
        if (*player).opts.opt[OPT_birth_no_selling as ::core::ffi::c_int as usize] {
            return 0 as int32_t;
        }
    } else {
        if tval_can_have_charges(obj) {
            price = object_value_real(obj, qty);
        } else {
            price = object_value_real(obj, 1 as ::core::ffi::c_int);
        }
        if this_store
            == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            price = price * 2 as ::core::ffi::c_int;
        }
    }
    price = (((price * adjust) as ::core::ffi::c_long + 50 as ::core::ffi::c_long)
        / 100 as ::core::ffi::c_long) as ::core::ffi::c_int;
    if !tval_can_have_charges(obj) {
        price *= qty;
    }
    if store_buying as ::core::ffi::c_int != 0
        && price as int32_t > (*proprietor).max_cost * qty as int32_t
    {
        price = ((*proprietor).max_cost * qty as int32_t) as ::core::ffi::c_int;
    }
    if price <= 0 as ::core::ffi::c_int {
        return qty as int32_t;
    }
    return price as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_store() {
    let mut slot: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut store_num: ::core::ffi::c_int = 0;
    let mut o_ptr: *mut object = ::core::ptr::null_mut::<object>();
    let mut list: *mut *mut object = mem_zalloc(
        (::core::mem::size_of::<*mut object>() as size_t)
            .wrapping_mul((*z_info).store_inven_max as size_t),
    ) as *mut *mut object;
    store_num = 0 as ::core::ffi::c_int;
    while store_num < (*z_info).store_max as ::core::ffi::c_int {
        let mut st_ptr: *mut store = stores.offset(store_num as isize) as *mut store;
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).store_inven_max as ::core::ffi::c_int {
            memset(
                (*borg_shops.offset(store_num as isize))
                    .ware
                    .offset(i as isize) as *mut borg_item
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<borg_item>() as size_t,
            );
            i += 1;
        }
        store_stock_list(
            st_ptr,
            list,
            (*z_info).store_inven_max as ::core::ffi::c_int,
        );
        slot = 0 as ::core::ffi::c_int;
        while slot < (*z_info).store_inven_max as ::core::ffi::c_int
            && !(*list.offset(slot as isize)).is_null()
        {
            o_ptr = *list.offset(slot as isize);
            let mut b_item: *mut borg_item = (*borg_shops.offset(store_num as isize))
                .ware
                .offset(slot as isize)
                as *mut borg_item;
            let mut buf: [::core::ffi::c_char; 120] = [0; 120];
            object_desc(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 120]>() as size_t,
                o_ptr,
                (ODESC_FULL as ::core::ffi::c_int | ODESC_STORE as ::core::ffi::c_int) as uint32_t,
                player,
            );
            if strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"(nothing)\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            borg_item_analyze(
                b_item,
                o_ptr,
                &raw mut buf as *mut ::core::ffi::c_char,
                if store_num == BORG_HOME {
                    false_0
                } else {
                    true_0
                } != 0,
            );
            (*b_item).cost =
                borg_price_item(o_ptr, false_0 != 0, 1 as ::core::ffi::c_int, store_num);
            slot += 1;
        }
        memcpy(
            (*borg_safe_shops.offset(store_num as isize)).ware as *mut ::core::ffi::c_void,
            (*borg_shops.offset(store_num as isize)).ware as *const ::core::ffi::c_void,
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        );
        store_num += 1;
    }
    mem_free(list as *mut ::core::ffi::c_void);
    list = ::core::ptr::null_mut::<*mut object>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_store() {
    borg_free_store();
    if (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int
        != BORG_HOME
    {
        msg(format(
            b"borg thinks home is %d, game thinks home is %d.  aborting. \0" as *const u8
                as *const ::core::ffi::c_char,
            (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            BORG_HOME,
        ));
        borg_init_failure = true_0 != 0;
    }
    borg_shops = mem_zalloc(
        ((*z_info).store_max as size_t).wrapping_mul(::core::mem::size_of::<borg_shop>() as size_t),
    ) as *mut borg_shop;
    borg_safe_shops = mem_zalloc(
        ((*z_info).store_max as size_t).wrapping_mul(::core::mem::size_of::<borg_shop>() as size_t),
    ) as *mut borg_shop;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_max as ::core::ffi::c_int {
        let ref mut fresh0 = (*borg_shops.offset(i as isize)).ware;
        *fresh0 = mem_zalloc(
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        ) as *mut borg_item;
        let ref mut fresh1 = (*borg_safe_shops.offset(i as isize)).ware;
        *fresh1 = mem_zalloc(
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        ) as *mut borg_item;
        i += 1;
    }
    borg_init_store_sell();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_store() {
    borg_free_store_sell();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_max as ::core::ffi::c_int {
        if !borg_shops.is_null() && !(*borg_shops.offset(i as isize)).ware.is_null() {
            mem_free((*borg_shops.offset(i as isize)).ware as *mut ::core::ffi::c_void);
            let ref mut fresh2 = (*borg_shops.offset(i as isize)).ware;
            *fresh2 = ::core::ptr::null_mut::<borg_item>();
        }
        if !borg_safe_shops.is_null() && !(*borg_safe_shops.offset(i as isize)).ware.is_null() {
            mem_free((*borg_safe_shops.offset(i as isize)).ware as *mut ::core::ffi::c_void);
            let ref mut fresh3 = (*borg_safe_shops.offset(i as isize)).ware;
            *fresh3 = ::core::ptr::null_mut::<borg_item>();
        }
        i += 1;
    }
    if !borg_safe_shops.is_null() {
        mem_free(borg_safe_shops as *mut ::core::ffi::c_void);
        borg_safe_shops = ::core::ptr::null_mut::<borg_shop>();
    }
    if !borg_shops.is_null() {
        mem_free(borg_shops as *mut ::core::ffi::c_void);
        borg_shops = ::core::ptr::null_mut::<borg_shop>();
    }
}
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
