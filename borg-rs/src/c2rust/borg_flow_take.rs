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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn prefix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    fn quark_str(q: quark_t) -> *const ::core::ffi::c_char;
    static mut k_info: *mut object_kind;
    static mut z_info: *mut angband_constants;
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_object(c: *mut chunk, grid: loc) -> *mut object;
    static mut borg_items: *mut borg_item;
    static mut r_info: *mut monster_race;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
    fn borg_flow_clear();
    fn borg_flow_spread(
        depth: ::core::ffi::c_int,
        optimize: bool,
        avoid: bool,
        tunneling: bool,
        stair_idx: ::core::ffi::c_int,
        sneak: bool,
    );
    fn borg_flow_enqueue_grid(y: ::core::ffi::c_int, x: ::core::ffi::c_int);
    fn borg_flow_commit(who: *const ::core::ffi::c_char, why: ::core::ffi::c_int) -> bool;
    fn borg_flow_old(why: ::core::ffi::c_int) -> bool;
    fn tval_is_book_k(kind: *const object_kind) -> bool;
    fn obj_kind_can_browse(kind: *const object_kind) -> bool;
    static mut borg_kills: *mut borg_kill;
    fn borg_get_leash(pick_up: bool) -> ::core::ffi::c_int;
    fn borg_flow_far_from_stairs(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        b_stair: ::core::ffi::c_int,
    ) -> bool;
    static mut track_less: borg_track;
    fn borg_flow_cost_stair(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        b_stair: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn borg_note(what: *const ::core::ffi::c_char);
    static mut borg_grids: [*mut borg_grid; 66];
    static mut sv_potion_inc_str: ::core::ffi::c_int;
    static mut sv_potion_detect_invis: ::core::ffi::c_int;
    fn borg_is_ammo(tval: ::core::ffi::c_int) -> bool;
    fn borg_check_light() -> bool;
    fn borg_distance(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn object_fully_known(obj: *const object) -> bool;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const KF_MAX: C2RustUnnamed_0 = 15;
pub const KF_RAND_RES_POWER: C2RustUnnamed_0 = 14;
pub const KF_RAND_BASE_RES: C2RustUnnamed_0 = 13;
pub const KF_SHOOTS_BOLTS: C2RustUnnamed_0 = 12;
pub const KF_SHOOTS_ARROWS: C2RustUnnamed_0 = 11;
pub const KF_SHOOTS_SHOTS: C2RustUnnamed_0 = 10;
pub const KF_SHOW_MULT: C2RustUnnamed_0 = 9;
pub const KF_SHOW_DICE: C2RustUnnamed_0 = 8;
pub const KF_GOOD: C2RustUnnamed_0 = 7;
pub const KF_EASY_KNOW: C2RustUnnamed_0 = 6;
pub const KF_QUEST_ART: C2RustUnnamed_0 = 5;
pub const KF_INSTA_ART: C2RustUnnamed_0 = 4;
pub const KF_RAND_POWER: C2RustUnnamed_0 = 3;
pub const KF_RAND_SUSTAIN: C2RustUnnamed_0 = 2;
pub const KF_RAND_HI_RES: C2RustUnnamed_0 = 1;
pub const KF_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const OBJ_NOTICE_IMAGINED: C2RustUnnamed_1 = 8;
pub const OBJ_NOTICE_IGNORE: C2RustUnnamed_1 = 4;
pub const OBJ_NOTICE_ASSESSED: C2RustUnnamed_1 = 2;
pub const OBJ_NOTICE_WORN: C2RustUnnamed_1 = 1;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_2 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_2 = 24;
pub const FEAT_LAVA: C2RustUnnamed_2 = 23;
pub const FEAT_PERM: C2RustUnnamed_2 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_2 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_2 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_2 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_2 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_2 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_2 = 16;
pub const FEAT_SECRET: C2RustUnnamed_2 = 15;
pub const FEAT_HOME: C2RustUnnamed_2 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_2 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_2 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_2 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_2 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_2 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_2 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_2 = 7;
pub const FEAT_MORE: C2RustUnnamed_2 = 6;
pub const FEAT_LESS: C2RustUnnamed_2 = 5;
pub const FEAT_BROKEN: C2RustUnnamed_2 = 4;
pub const FEAT_OPEN: C2RustUnnamed_2 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_2 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_2 = 1;
pub const FEAT_NONE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const RF_MAX: C2RustUnnamed_3 = 85;
pub const RF_NO_SLOW: C2RustUnnamed_3 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_3 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_3 = 82;
pub const RF_NO_CONF: C2RustUnnamed_3 = 81;
pub const RF_NO_STUN: C2RustUnnamed_3 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_3 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_3 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_3 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_3 = 76;
pub const RF_IM_WATER: C2RustUnnamed_3 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_3 = 74;
pub const RF_IM_POIS: C2RustUnnamed_3 = 73;
pub const RF_IM_COLD: C2RustUnnamed_3 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_3 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_3 = 70;
pub const RF_IM_ACID: C2RustUnnamed_3 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_3 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_3 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_3 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_3 = 65;
pub const RF_METAL: C2RustUnnamed_3 = 64;
pub const RF_NONLIVING: C2RustUnnamed_3 = 63;
pub const RF_UNDEAD: C2RustUnnamed_3 = 62;
pub const RF_EVIL: C2RustUnnamed_3 = 61;
pub const RF_ANIMAL: C2RustUnnamed_3 = 60;
pub const RF_DEMON: C2RustUnnamed_3 = 59;
pub const RF_DRAGON: C2RustUnnamed_3 = 58;
pub const RF_GIANT: C2RustUnnamed_3 = 57;
pub const RF_TROLL: C2RustUnnamed_3 = 56;
pub const RF_ORC: C2RustUnnamed_3 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_3 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_3 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_3 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_3 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_3 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_3 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_3 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_3 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_3 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_3 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_3 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_3 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_3 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_3 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_3 = 40;
pub const RF_DROP_20: C2RustUnnamed_3 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_3 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_3 = 37;
pub const RF_DROP_4: C2RustUnnamed_3 = 36;
pub const RF_DROP_3: C2RustUnnamed_3 = 35;
pub const RF_DROP_2: C2RustUnnamed_3 = 34;
pub const RF_DROP_1: C2RustUnnamed_3 = 33;
pub const RF_DROP_60: C2RustUnnamed_3 = 32;
pub const RF_DROP_40: C2RustUnnamed_3 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_3 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_3 = 29;
pub const RF_POWERFUL: C2RustUnnamed_3 = 28;
pub const RF_SPIRIT: C2RustUnnamed_3 = 27;
pub const RF_SMART: C2RustUnnamed_3 = 26;
pub const RF_STUPID: C2RustUnnamed_3 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_3 = 24;
pub const RF_RAND_50: C2RustUnnamed_3 = 23;
pub const RF_RAND_25: C2RustUnnamed_3 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_3 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_3 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_3 = 19;
pub const RF_REGENERATE: C2RustUnnamed_3 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_3 = 17;
pub const RF_UNAWARE: C2RustUnnamed_3 = 16;
pub const RF_SEASONAL: C2RustUnnamed_3 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_3 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_3 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_3 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_3 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_3 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_3 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_3 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_3 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_3 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_3 = 5;
pub const RF_FEMALE: C2RustUnnamed_3 = 4;
pub const RF_MALE: C2RustUnnamed_3 = 3;
pub const RF_QUESTOR: C2RustUnnamed_3 = 2;
pub const RF_UNIQUE: C2RustUnnamed_3 = 1;
pub const RF_NONE: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_4 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_4 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_4 = 263;
pub const BI_EMPTY: C2RustUnnamed_4 = 262;
pub const BI_CARRY: C2RustUnnamed_4 = 261;
pub const BI_WEIGHT: C2RustUnnamed_4 = 260;
pub const BI_DINV: C2RustUnnamed_4 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_4 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_4 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_4 = 256;
pub const BI_ADIGGER: C2RustUnnamed_4 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_4 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_4 = 253;
pub const BI_AROD2: C2RustUnnamed_4 = 252;
pub const BI_AROD1: C2RustUnnamed_4 = 251;
pub const BI_ASHROOM: C2RustUnnamed_4 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_4 = 249;
pub const BI_AHWORD: C2RustUnnamed_4 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_4 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_4 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_4 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_4 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_4 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_4 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_4 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_4 = 240;
pub const BI_ABRAND: C2RustUnnamed_4 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_4 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_4 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_4 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_4 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_4 = 234;
pub const BI_ACLW: C2RustUnnamed_4 = 233;
pub const BI_ACSW: C2RustUnnamed_4 = 232;
pub const BI_ACCW: C2RustUnnamed_4 = 231;
pub const BI_AGLYPH: C2RustUnnamed_4 = 230;
pub const BI_APFE: C2RustUnnamed_4 = 229;
pub const BI_ALITE: C2RustUnnamed_4 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_4 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_4 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_4 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_4 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_4 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_4 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_4 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_4 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_4 = 219;
pub const BI_ASPEED: C2RustUnnamed_4 = 218;
pub const BI_AID: C2RustUnnamed_4 = 217;
pub const BI_ALIFE: C2RustUnnamed_4 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_4 = 215;
pub const BI_AHEAL: C2RustUnnamed_4 = 214;
pub const BI_AFUEL: C2RustUnnamed_4 = 213;
pub const BI_AESCAPE: C2RustUnnamed_4 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_4 = 211;
pub const BI_APHASE: C2RustUnnamed_4 = 210;
pub const BI_WB_POIS: C2RustUnnamed_4 = 209;
pub const BI_WB_COLD: C2RustUnnamed_4 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_4 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_4 = 206;
pub const BI_WB_ACID: C2RustUnnamed_4 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_4 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_4 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_4 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_4 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_4 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_4 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_4 = 198;
pub const BI_WS_ORC: C2RustUnnamed_4 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_4 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_4 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_4 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_4 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_4 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_4 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_4 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_4 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_4 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_4 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_4 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_4 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_4 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_4 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_4 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_4 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_4 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_4 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_4 = 178;
pub const BI_CRSSICK: C2RustUnnamed_4 = 177;
pub const BI_CRSDULL: C2RustUnnamed_4 = 176;
pub const BI_CRSVULN: C2RustUnnamed_4 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_4 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_4 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_4 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_4 = 171;
pub const BI_CRSSUND: C2RustUnnamed_4 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_4 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_4 = 168;
pub const BI_CRSPARA: C2RustUnnamed_4 = 167;
pub const BI_CRSHALU: C2RustUnnamed_4 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_4 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_4 = 164;
pub const BI_CRSTELE: C2RustUnnamed_4 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_4 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_4 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_4 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_4 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_4 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_4 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_4 = 156;
pub const BI_AMISSILES: C2RustUnnamed_4 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_4 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_4 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_4 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_4 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_4 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_4 = 149;
pub const BI_SHOTS: C2RustUnnamed_4 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_4 = 147;
pub const BI_BLOWS: C2RustUnnamed_4 = 146;
pub const BI_BART: C2RustUnnamed_4 = 145;
pub const BI_SLING: C2RustUnnamed_4 = 144;
pub const BI_BTODAM: C2RustUnnamed_4 = 143;
pub const BI_BTOHIT: C2RustUnnamed_4 = 142;
pub const BI_BID: C2RustUnnamed_4 = 141;
pub const BI_WDS: C2RustUnnamed_4 = 140;
pub const BI_WDD: C2RustUnnamed_4 = 139;
pub const BI_WID: C2RustUnnamed_4 = 138;
pub const BI_WTODAM: C2RustUnnamed_4 = 137;
pub const BI_WTOHIT: C2RustUnnamed_4 = 136;
pub const BI_TODAM: C2RustUnnamed_4 = 135;
pub const BI_TOHIT: C2RustUnnamed_4 = 134;
pub const BI_ARMOR: C2RustUnnamed_4 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_4 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_4 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_4 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_4 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_4 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_4 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_4 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_4 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_4 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_4 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_4 = 122;
pub const BI_ISFORGET: C2RustUnnamed_4 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_4 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_4 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_4 = 118;
pub const BI_ISSTUN: C2RustUnnamed_4 = 117;
pub const BI_ISCUT: C2RustUnnamed_4 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_4 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_4 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_4 = 113;
pub const BI_ISBLIND: C2RustUnnamed_4 = 112;
pub const BI_ISGORGED: C2RustUnnamed_4 = 111;
pub const BI_ISFULL: C2RustUnnamed_4 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_4 = 109;
pub const BI_ISWEAK: C2RustUnnamed_4 = 108;
pub const BI_KING: C2RustUnnamed_4 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_4 = 106;
pub const BI_CDEPTH: C2RustUnnamed_4 = 105;
pub const BI_SFRACT: C2RustUnnamed_4 = 104;
pub const BI_SHLIFE: C2RustUnnamed_4 = 103;
pub const BI_SRDIS: C2RustUnnamed_4 = 102;
pub const BI_SRKAOS: C2RustUnnamed_4 = 101;
pub const BI_SRNTHR: C2RustUnnamed_4 = 100;
pub const BI_SRNXUS: C2RustUnnamed_4 = 99;
pub const BI_SRSHRD: C2RustUnnamed_4 = 98;
pub const BI_SRSND: C2RustUnnamed_4 = 97;
pub const BI_SRCONF: C2RustUnnamed_4 = 96;
pub const BI_SRBLIND: C2RustUnnamed_4 = 95;
pub const BI_SRDARK: C2RustUnnamed_4 = 94;
pub const BI_SRLITE: C2RustUnnamed_4 = 93;
pub const BI_SRFEAR: C2RustUnnamed_4 = 92;
pub const BI_SRPOIS: C2RustUnnamed_4 = 91;
pub const BI_SRACID: C2RustUnnamed_4 = 90;
pub const BI_SRELEC: C2RustUnnamed_4 = 89;
pub const BI_SRCOLD: C2RustUnnamed_4 = 88;
pub const BI_SRFIRE: C2RustUnnamed_4 = 87;
pub const BI_FRACT: C2RustUnnamed_4 = 86;
pub const BI_HLIFE: C2RustUnnamed_4 = 85;
pub const BI_RDIS: C2RustUnnamed_4 = 84;
pub const BI_RKAOS: C2RustUnnamed_4 = 83;
pub const BI_RNTHR: C2RustUnnamed_4 = 82;
pub const BI_RNXUS: C2RustUnnamed_4 = 81;
pub const BI_RSHRD: C2RustUnnamed_4 = 80;
pub const BI_RSND: C2RustUnnamed_4 = 79;
pub const BI_RCONF: C2RustUnnamed_4 = 78;
pub const BI_RBLIND: C2RustUnnamed_4 = 77;
pub const BI_RDARK: C2RustUnnamed_4 = 76;
pub const BI_RLITE: C2RustUnnamed_4 = 75;
pub const BI_RFEAR: C2RustUnnamed_4 = 74;
pub const BI_RPOIS: C2RustUnnamed_4 = 73;
pub const BI_RACID: C2RustUnnamed_4 = 72;
pub const BI_RELEC: C2RustUnnamed_4 = 71;
pub const BI_RCOLD: C2RustUnnamed_4 = 70;
pub const BI_RFIRE: C2RustUnnamed_4 = 69;
pub const BI_IPOIS: C2RustUnnamed_4 = 68;
pub const BI_IELEC: C2RustUnnamed_4 = 67;
pub const BI_ICOLD: C2RustUnnamed_4 = 66;
pub const BI_IACID: C2RustUnnamed_4 = 65;
pub const BI_IFIRE: C2RustUnnamed_4 = 64;
pub const BI_DIG: C2RustUnnamed_4 = 63;
pub const BI_THT: C2RustUnnamed_4 = 62;
pub const BI_THB: C2RustUnnamed_4 = 61;
pub const BI_THN: C2RustUnnamed_4 = 60;
pub const BI_SRCH: C2RustUnnamed_4 = 59;
pub const BI_STL: C2RustUnnamed_4 = 58;
pub const BI_SAV: C2RustUnnamed_4 = 57;
pub const BI_DEV: C2RustUnnamed_4 = 56;
pub const BI_DISM: C2RustUnnamed_4 = 55;
pub const BI_DISP: C2RustUnnamed_4 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_4 = 53;
pub const BI_INFRA: C2RustUnnamed_4 = 52;
pub const BI_SINV: C2RustUnnamed_4 = 51;
pub const BI_REG: C2RustUnnamed_4 = 50;
pub const BI_FEATH: C2RustUnnamed_4 = 49;
pub const BI_SDIG: C2RustUnnamed_4 = 48;
pub const BI_DAM_RED: C2RustUnnamed_4 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_4 = 46;
pub const BI_GOLD: C2RustUnnamed_4 = 45;
pub const BI_SPEED: C2RustUnnamed_4 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_4 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_4 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_4 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_4 = 40;
pub const BI_FOOD: C2RustUnnamed_4 = 39;
pub const BI_RECALL: C2RustUnnamed_4 = 38;
pub const BI_ESP: C2RustUnnamed_4 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_4 = 36;
pub const BI_CLEVEL: C2RustUnnamed_4 = 35;
pub const BI_FAIL2: C2RustUnnamed_4 = 34;
pub const BI_FAIL1: C2RustUnnamed_4 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_4 = 32;
pub const BI_MAXSP: C2RustUnnamed_4 = 31;
pub const BI_CURSP: C2RustUnnamed_4 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_4 = 29;
pub const BI_MAXHP: C2RustUnnamed_4 = 28;
pub const BI_CURHP: C2RustUnnamed_4 = 27;
pub const BI_LIGHT: C2RustUnnamed_4 = 26;
pub const BI_CLASS: C2RustUnnamed_4 = 25;
pub const BI_SCON: C2RustUnnamed_4 = 24;
pub const BI_SDEX: C2RustUnnamed_4 = 23;
pub const BI_SWIS: C2RustUnnamed_4 = 22;
pub const BI_SINT: C2RustUnnamed_4 = 21;
pub const BI_SSTR: C2RustUnnamed_4 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_4 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_4 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_4 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_4 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_4 = 15;
pub const BI_CCON: C2RustUnnamed_4 = 14;
pub const BI_CDEX: C2RustUnnamed_4 = 13;
pub const BI_CWIS: C2RustUnnamed_4 = 12;
pub const BI_CINT: C2RustUnnamed_4 = 11;
pub const BI_CSTR: C2RustUnnamed_4 = 10;
pub const BI_ACON: C2RustUnnamed_4 = 9;
pub const BI_ADEX: C2RustUnnamed_4 = 8;
pub const BI_AWIS: C2RustUnnamed_4 = 7;
pub const BI_AINT: C2RustUnnamed_4 = 6;
pub const BI_ASTR: C2RustUnnamed_4 = 5;
pub const BI_CON: C2RustUnnamed_4 = 4;
pub const BI_DEX: C2RustUnnamed_4 = 3;
pub const BI_WIS: C2RustUnnamed_4 = 2;
pub const BI_INT: C2RustUnnamed_4 = 1;
pub const BI_STR: C2RustUnnamed_4 = 0;
pub type borg_time = int32_t;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_5 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_5 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_5 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_5 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_5 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_5 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_5 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_5 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_5 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_5 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_5 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_5 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_5 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_5 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_5 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_5 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_5 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_5 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_5 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_5 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_5 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_5 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_5 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_5 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_5 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_5 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_5 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_5 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_5 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_5 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_5 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_5 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_5 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_5 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_5 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_5 = 0;
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
    pub temp: C2RustUnnamed_13,
    pub status: C2RustUnnamed_12,
    pub time: C2RustUnnamed_11,
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
    pub goal: C2RustUnnamed_10,
    pub init_save: C2RustUnnamed_9,
    pub mon: C2RustUnnamed_8,
    pub id: C2RustUnnamed_7,
    pub near: C2RustUnnamed_6,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
pub struct C2RustUnnamed_12 {
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
pub struct C2RustUnnamed_13 {
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
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_take {
    pub kidx: uint32_t,
    pub known: bool,
    pub seen: bool,
    pub extra: bool,
    pub orbed: bool,
    pub x: uint8_t,
    pub y: uint8_t,
    pub when: borg_time,
    pub value: ::core::ffi::c_int,
    pub tval: ::core::ffi::c_int,
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_takes_cnt: int16_t = 0;
#[no_mangle]
pub static mut borg_takes_nxt: int16_t = 0;
#[no_mangle]
pub static mut borg_takes: *mut borg_take = ::core::ptr::null::<borg_take>() as *mut borg_take;
#[no_mangle]
pub unsafe extern "C" fn borg_get_top_object(mut c: *mut chunk, mut grid: loc) -> *mut object {
    let mut o_ptr: *mut object = ::core::ptr::null_mut::<object>();
    o_ptr = square_object(c, grid);
    while !o_ptr.is_null() {
        if !(*o_ptr).known.is_null()
            && (*(*o_ptr).known).notice as ::core::ffi::c_int
                & OBJ_NOTICE_IGNORE as ::core::ffi::c_int
                != 0
        {
            o_ptr = (*o_ptr).next;
        } else {
            if !((*(*o_ptr).kind).ignore != 0) {
                break;
            }
            o_ptr = (*o_ptr).next;
        }
    }
    return o_ptr;
}
unsafe extern "C" fn borg_guess_kind(
    mut a: uint8_t,
    mut c: wchar_t,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> *mut object_kind {
    let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x, y));
    if o_ptr.is_null() {
        return ::core::ptr::null_mut::<object_kind>();
    }
    return (*o_ptr).kind;
}
#[no_mangle]
pub unsafe extern "C" fn borg_delete_take(mut i: ::core::ffi::c_int) {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
    if (*take).kidx == 0 {
        return;
    }
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        if (*take).kidx >= (*z_info).k_max as uint32_t {
            borg_note(format(
                b"# Forgetting an object at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                (*take).y as ::core::ffi::c_int,
                (*take).x as ::core::ffi::c_int,
            ));
        } else {
            borg_note(format(
                b"# Forgetting an object '%s' at (%d,%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*k_info.offset((*take).kidx as isize)).name,
                (*take).y as ::core::ffi::c_int,
                (*take).x as ::core::ffi::c_int,
            ));
        }
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*take).y as isize))
        .offset((*take).x as isize) as *mut borg_grid;
    (*ag).take = 0 as uint8_t;
    memset(
        take as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_take>() as size_t,
    );
    borg_takes_cnt -= 1;
    if borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE {
        borg.goal.type_0 = 0 as int16_t;
    }
}
unsafe extern "C" fn borg_follow_take_aux(mut i: ::core::ffi::c_int) -> bool {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset((*borg_takes.offset(i as isize)).y as isize))
    .offset((*borg_takes.offset(i as isize)).x as isize) as *mut borg_grid;
    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_follow_take(mut i: ::core::ffi::c_int) {
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset((*take).y as isize))
    .offset((*take).x as isize) as *mut borg_grid;
    let mut kill: *mut borg_kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    let mut old_kind: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    if (*take).kidx == 0 {
        return;
    }
    ox = (*take).x as ::core::ffi::c_int;
    oy = (*take).y as ::core::ffi::c_int;
    old_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
    if (*take).y as ::core::ffi::c_int == borg.c.y && (*take).x as ::core::ffi::c_int == borg.c.x {
        borg_delete_take(i);
    }
    if !borg_follow_take_aux(i) {
        return;
    }
    if (*ag).kill as ::core::ffi::c_int != 0
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
            RF_TAKE_ITEM as ::core::ffi::c_int,
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
            RF_KILL_ITEM as ::core::ffi::c_int,
        )
    {
        return;
    }
    borg_note(format(
        b"# There was an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        (*old_kind).name,
        ox,
        oy,
    ));
    borg_delete_take(i);
}
unsafe extern "C" fn borg_new_take(
    mut kind: *mut object_kind,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
        .offset(x as isize) as *mut borg_grid;
    let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x, y));
    i = 1 as ::core::ffi::c_int;
    while n < 0 as ::core::ffi::c_int && i < borg_takes_nxt as ::core::ffi::c_int {
        if (*borg_takes.offset(i as isize)).kidx == 0 {
            n = i;
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int
        && (borg_takes_nxt as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
    {
        let fresh0 = borg_takes_nxt;
        borg_takes_nxt = borg_takes_nxt + 1;
        n = fresh0 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        borg_note(b"# Too many objects\0" as *const u8 as *const ::core::ffi::c_char);
        n = (Rand_div((borg_takes_nxt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t)
            as int32_t
            + 1 as int32_t) as ::core::ffi::c_int;
        borg_delete_take(n);
    }
    borg_takes_cnt += 1;
    take = borg_takes.offset(n as isize) as *mut borg_take;
    (*take).kidx = (*kind).kidx;
    (*take).tval = (*kind).tval;
    (*take).x = x as uint8_t;
    (*take).y = y as uint8_t;
    (*ag).take = n as uint8_t;
    (*take).when = borg.time.now;
    (*take).orbed = false_0 != 0;
    if (*kind).aware {
        (*take).value = (*kind).cost;
        if (*take).tval == TV_GOLD as ::core::ffi::c_int {
            (*take).value = 30 as ::core::ffi::c_int;
        }
    } else {
        (*take).value = 1 as ::core::ffi::c_int;
    }
    if object_fully_known(o_ptr) as ::core::ffi::c_int != 0
        && (((*o_ptr).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || ((*o_ptr).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || ((*o_ptr).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
    {
        (*take).value = -(10 as ::core::ffi::c_int);
    }
    if (*o_ptr).note != 0
        && prefix(
            quark_str((*o_ptr).note),
            b"borg ignore\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        (*take).value = -(10 as ::core::ffi::c_int);
    }
    borg_note(format(
        b"# Creating an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        (*k_info.offset((*take).kidx as isize)).name,
        (*take).x as ::core::ffi::c_int,
        (*take).y as ::core::ffi::c_int,
    ));
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0 {
        borg.goal.type_0 = 0 as int16_t;
    }
    (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn observe_take_diff(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut kind: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    kind = borg_guess_kind(a, c, y, x);
    if kind.is_null() {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    i = borg_new_take(kind, y, x);
    take = borg_takes.offset(i as isize) as *mut borg_take;
    (*take).when = borg.time.now;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn observe_take_move(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut k_ptr: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut current_block_17: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            if !(*take).seen {
                ox = (*take).x as ::core::ffi::c_int;
                oy = (*take).y as ::core::ffi::c_int;
                z = borg_distance(oy, ox, y, x);
                if !(z > d) {
                    k_ptr = k_info.offset((*take).kidx as isize) as *mut object_kind;
                    if !(*borg
                        .trait_0
                        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                        == 0
                        && c != (*k_ptr).d_char)
                    {
                        if *borg
                            .trait_0
                            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                            == 0
                            && a as ::core::ffi::c_int != (*k_ptr).d_attr as ::core::ffi::c_int
                            && ((*k_ptr).d_attr as ::core::ffi::c_int != 11 as ::core::ffi::c_int
                                && (*k_ptr).d_char == '!' as i32)
                        {
                            if (*k_ptr).flavor.is_null() {
                                current_block_17 = 16668937799742929182;
                            } else {
                                current_block_17 = 2968425633554183086;
                            }
                        } else {
                            current_block_17 = 2968425633554183086;
                        }
                        match current_block_17 {
                            16668937799742929182 => {}
                            _ => {
                                if z != 0 {
                                    (*borg_grids[(*take).y as usize].offset((*take).x as isize))
                                        .take = 0 as uint8_t;
                                    (*take).x = x as uint8_t;
                                    (*take).y = y as uint8_t;
                                    (*borg_grids[(*take).y as usize].offset((*take).x as isize))
                                        .take = i as uint8_t;
                                    borg_note(format(
                                        b"# Tracking an object '%s' at (%d,%d) from (%d,%d)\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        (*k_ptr).name,
                                        (*take).y as ::core::ffi::c_int,
                                        (*take).x as ::core::ffi::c_int,
                                        ox,
                                        oy,
                                    ));
                                    if borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE {
                                        borg.goal.type_0 = 0 as int16_t;
                                    }
                                }
                                (*take).when = borg.time.now;
                                (*take).seen = true_0 != 0;
                                (*borg_grids[(*take).y as usize].offset((*take).x as isize)).feat =
                                    FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
                                return true_0 != 0;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut leash: ::core::ffi::c_int = borg_get_leash(true_0 != 0);
    let mut full_quiver: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg
        .trait_0
        .offset(BI_FAST_SHOTS as ::core::ffi::c_int as isize)
        != 0
    {
        full_quiver = ((*z_info).quiver_slot_size as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int;
    } else {
        full_quiver = (*z_info).quiver_slot_size as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    if borg.mon.scary {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 {
        return false_0 != 0;
    }
    if borg.morgoth_position {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    let mut current_block_29: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            if b_stair != -(1 as ::core::ffi::c_int)
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 10 as ::core::ffi::c_int
            {
                j = borg_distance(
                    *track_less.y.offset(b_stair as isize),
                    *track_less.x.offset(b_stair as isize),
                    y,
                    x,
                );
                if j != 255 as ::core::ffi::c_int && (b_j <= leash && j >= leash) {
                    current_block_29 = 7175849428784450219;
                } else {
                    current_block_29 = 6669252993407410313;
                }
            } else {
                current_block_29 = 6669252993407410313;
            }
            match current_block_29 {
                7175849428784450219 => {}
                _ => {
                    if !((*take).value <= 0 as ::core::ffi::c_int) {
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !(viewable as ::core::ffi::c_int != 0
                            && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                        {
                            if !((*take).tval
                                == *borg
                                    .trait_0
                                    .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
                                && *borg
                                    .trait_0
                                    .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
                                    >= full_quiver)
                            {
                                if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                                    >= 500000 as ::core::ffi::c_int
                                    && *borg_cfg.offset(
                                        BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    if (*take).tval == TV_GOLD as ::core::ffi::c_int {
                                        current_block_29 = 7175849428784450219;
                                    } else if tval_is_book_k(
                                        k_info.offset((*take).kidx as isize) as *mut object_kind
                                    )
                                        as ::core::ffi::c_int
                                        != 0
                                        && !obj_kind_can_browse(
                                            k_info.offset((*take).kidx as isize)
                                                as *mut object_kind,
                                        )
                                    {
                                        current_block_29 = 7175849428784450219;
                                    } else if ((*take).tval == TV_SHOT as ::core::ffi::c_int
                                        || (*take).tval == TV_ARROW as ::core::ffi::c_int
                                        || (*take).tval == TV_BOLT as ::core::ffi::c_int)
                                        && (*take).tval
                                            != *borg
                                                .trait_0
                                                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
                                    {
                                        current_block_29 = 7175849428784450219;
                                    } else {
                                        current_block_29 = 14434620278749266018;
                                    }
                                } else {
                                    current_block_29 = 14434620278749266018;
                                }
                                match current_block_29 {
                                    7175849428784450219 => {}
                                    _ => {
                                        borg_flow_clear();
                                        if !(nearness > 5 as ::core::ffi::c_int
                                            && *borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                < 20 as ::core::ffi::c_int
                                            && borg_flow_cost_stair(y, x, b_stair) > leash)
                                        {
                                            borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                            borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                            borg_temp_n += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take_scum(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !((*take).value <= 0 as ::core::ffi::c_int) {
                if !(viewable as ::core::ffi::c_int != 0
                    && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                {
                    if !borg_flow_far_from_stairs(x, y, b_stair) {
                        borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                        borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                        borg_temp_n += 1;
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        true_0 != 0,
    );
    if !borg_flow_commit(
        b"Scum item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_take_lunal(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_takes_cnt == 0 {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        let mut k_ptr: *mut object_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
        let mut item_bad: bool = false;
        if !k_ptr.is_null() {
            x = (*take).x as ::core::ffi::c_int;
            y = (*take).y as ::core::ffi::c_int;
            item_bad = true_0 != 0;
            if (*take).tval == TV_GOLD as ::core::ffi::c_int {
                borg_note(format(
                    b"# Lunal Item %s, at %d,%d\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    y,
                    x,
                ));
                item_bad = false_0 != 0;
            }
            if item_bad as ::core::ffi::c_int != 0 && (*take).value > 0 as ::core::ffi::c_int {
                if borg_is_ammo((*take).tval) {
                    ii = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
                    while ii
                        < (*z_info).pack_size as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int
                            + (*z_info).quiver_size as ::core::ffi::c_int
                    {
                        if !((*borg_items.offset(ii as isize)).iqty == 0) {
                            if !((*borg_items.offset(ii as isize)).iqty as ::core::ffi::c_int
                                == (*z_info).quiver_slot_size as ::core::ffi::c_int)
                            {
                                if !((*take).kidx != (*borg_items.offset(ii as isize)).kind) {
                                    if (*k_ptr).sval
                                        == (*borg_items.offset(ii as isize)).sval
                                            as ::core::ffi::c_int
                                        && (*k_ptr).tval
                                            == (*borg_items.offset(ii as isize)).tval
                                                as ::core::ffi::c_int
                                    {
                                        item_bad = false_0 != 0;
                                    }
                                }
                            }
                        }
                        ii += 1;
                    }
                } else if (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int
                        - *borg
                            .trait_0
                            .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .iqty
                    != 0
                {
                    ii = 0 as ::core::ffi::c_int;
                    while ii
                        < (*z_info).pack_size as ::core::ffi::c_int
                            - *borg
                                .trait_0
                                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                    {
                        if !((*borg_items.offset(ii as isize)).iqty == 0) {
                            if !((*take).kidx != (*borg_items.offset(ii as isize)).kind) {
                                if (*k_ptr).sval
                                    == (*borg_items.offset(ii as isize)).sval as ::core::ffi::c_int
                                    && (*k_ptr).tval
                                        == (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                    && ((*borg_items.offset(ii as isize)).tval
                                        as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        || (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                            == TV_SCROLL as ::core::ffi::c_int
                                        || (*borg_items.offset(ii as isize)).tval
                                            as ::core::ffi::c_int
                                            == TV_ROD as ::core::ffi::c_int)
                                {
                                    item_bad = false_0 != 0;
                                }
                            }
                        }
                        ii += 1;
                    }
                }
            }
            if (*borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int
                    - *borg
                        .trait_0
                        .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                    - 1 as ::core::ffi::c_int) as isize,
            ))
            .iqty
                == 0
                && item_bad as ::core::ffi::c_int == true_0
            {
                if !borg_is_ammo((*take).tval)
                    || (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int
                            + (*z_info).quiver_size as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                    .iqty as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    if (*take).tval == TV_POTION as ::core::ffi::c_int
                        && (*k_info.offset((*take).kidx as isize)).sval >= sv_potion_inc_str
                        && (*k_info.offset((*take).kidx as isize)).sval <= sv_potion_detect_invis
                    {
                        borg_note(format(
                            b"# Lunal Item %s, at %d,%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*k_info.offset((*take).kidx as isize)).name,
                            y,
                            x,
                        ));
                        item_bad = false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*k_info.offset((*take).kidx as isize)).kind_flags as *mut bitflag,
                        (KF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        KF_INSTA_ART as ::core::ffi::c_int,
                    ) {
                        borg_note(format(
                            b"# Lunal Item %s, at %d,%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*k_info.offset((*take).kidx as isize)).name,
                            y,
                            x,
                        ));
                        item_bad = false_0 != 0;
                    }
                    if *borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0 {
                        if (*take).value >= 1 as ::core::ffi::c_int {
                            item_bad = false_0 != 0;
                        }
                    }
                }
            }
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !(viewable as ::core::ffi::c_int != 0
                && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
            {
                if !borg_flow_far_from_stairs(x, y, b_stair) {
                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                    borg_temp_n += 1;
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        false_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        true_0 != 0,
    );
    if !borg_flow_commit(
        b"munchkin item\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if borg_check_light() {
        return true_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_take() {
    borg_free_flow_take();
    borg_takes_cnt = 0 as int16_t;
    borg_takes_nxt = 1 as int16_t;
    borg_takes =
        mem_zalloc((256 as size_t).wrapping_mul(::core::mem::size_of::<borg_take>() as size_t))
            as *mut borg_take;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_take() {
    if !borg_takes.is_null() {
        mem_free(borg_takes as *mut ::core::ffi::c_void);
        borg_takes = ::core::ptr::null_mut::<borg_take>();
    }
}
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
