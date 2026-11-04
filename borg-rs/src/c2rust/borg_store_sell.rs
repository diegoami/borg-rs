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
    fn flag_is_empty(flags: *const bitflag, size: size_t) -> bool;
    fn flag_is_equal(flags1: *const bitflag, flags2: *const bitflag, size: size_t) -> bool;
    static mut k_info: *mut object_kind;
    static mut e_info: *mut ego_item;
    static mut player: *mut player;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut safe_items: *mut borg_item;
    fn obj_kind_can_browse(kind: *const object_kind) -> bool;
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    static mut num_mana: int16_t;
    fn borg_notice_home(in_item: *mut borg_item, no_items: bool);
    fn borg_power_home() -> int32_t;
    fn borg_item_worth_id(item: *const borg_item) -> bool;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    fn borg_ego_has_random_power(e_ptr: *mut ego_item) -> bool;
    fn borg_item_note_needs_id(item: *const borg_item) -> bool;
    static mut sv_potion_cure_critical: ::core::ffi::c_int;
    static mut sv_potion_cure_serious: ::core::ffi::c_int;
    static mut sv_potion_healing: ::core::ffi::c_int;
    static mut sv_potion_star_healing: ::core::ffi::c_int;
    static mut sv_potion_life: ::core::ffi::c_int;
    static mut sv_potion_restore_mana: ::core::ffi::c_int;
    static mut kv_potion_restore_mana: ::core::ffi::c_int;
    static mut sv_potion_speed: ::core::ffi::c_int;
    static mut sv_scroll_teleport: ::core::ffi::c_int;
    static mut sv_rod_mapping: ::core::ffi::c_int;
    static mut sv_rod_healing: ::core::ffi::c_int;
    static mut kv_rod_healing: ::core::ffi::c_int;
    static mut sv_staff_teleportation: ::core::ffi::c_int;
    static mut sv_wand_stinking_cloud: ::core::ffi::c_int;
    static mut sv_wand_magic_missile: ::core::ffi::c_int;
    static mut sv_wand_annihilation: ::core::ffi::c_int;
    static mut sv_wand_drain_life: ::core::ffi::c_int;
    fn borg_power() -> int32_t;
    static mut bought_item_tval: [::core::ffi::c_int; 10];
    static mut bought_item_sval: [::core::ffi::c_int; 10];
    static mut bought_item_pval: [::core::ffi::c_int; 10];
    static mut bought_item_store: [::core::ffi::c_int; 10];
    static mut bought_item_num: ::core::ffi::c_int;
    static mut borg_shops: *mut borg_shop;
    static mut borg_safe_shops: *mut borg_shop;
    static mut shop_num: int16_t;
    static mut weapon_swap: ::core::ffi::c_int;
    static mut armour_swap: ::core::ffi::c_int;
    fn borg_notice(notice_swap: bool);
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
pub const OF_MAX: C2RustUnnamed_0 = 39;
pub const OF_MULTIPLY_WEIGHT: C2RustUnnamed_0 = 38;
pub const OF_THROWING: C2RustUnnamed_0 = 37;
pub const OF_TRAP_IMMUNE: C2RustUnnamed_0 = 36;
pub const OF_EXPLODE: C2RustUnnamed_0 = 35;
pub const OF_DIG_3: C2RustUnnamed_0 = 34;
pub const OF_DIG_2: C2RustUnnamed_0 = 33;
pub const OF_DIG_1: C2RustUnnamed_0 = 32;
pub const OF_LIGHT_3: C2RustUnnamed_0 = 31;
pub const OF_LIGHT_2: C2RustUnnamed_0 = 30;
pub const OF_FRAGILE: C2RustUnnamed_0 = 29;
pub const OF_STICKY: C2RustUnnamed_0 = 28;
pub const OF_DRAIN_EXP: C2RustUnnamed_0 = 27;
pub const OF_AGGRAVATE: C2RustUnnamed_0 = 26;
pub const OF_NO_TELEPORT: C2RustUnnamed_0 = 25;
pub const OF_AFRAID: C2RustUnnamed_0 = 24;
pub const OF_IMPAIR_MANA: C2RustUnnamed_0 = 23;
pub const OF_IMPAIR_HP: C2RustUnnamed_0 = 22;
pub const OF_NO_FUEL: C2RustUnnamed_0 = 21;
pub const OF_TAKES_FUEL: C2RustUnnamed_0 = 20;
pub const OF_BURNS_OUT: C2RustUnnamed_0 = 19;
pub const OF_BLESSED: C2RustUnnamed_0 = 18;
pub const OF_IMPACT: C2RustUnnamed_0 = 17;
pub const OF_HOLD_LIFE: C2RustUnnamed_0 = 16;
pub const OF_FREE_ACT: C2RustUnnamed_0 = 15;
pub const OF_SEE_INVIS: C2RustUnnamed_0 = 14;
pub const OF_TELEPATHY: C2RustUnnamed_0 = 13;
pub const OF_REGEN: C2RustUnnamed_0 = 12;
pub const OF_FEATHER: C2RustUnnamed_0 = 11;
pub const OF_SLOW_DIGEST: C2RustUnnamed_0 = 10;
pub const OF_PROT_STUN: C2RustUnnamed_0 = 9;
pub const OF_PROT_CONF: C2RustUnnamed_0 = 8;
pub const OF_PROT_BLIND: C2RustUnnamed_0 = 7;
pub const OF_PROT_FEAR: C2RustUnnamed_0 = 6;
pub const OF_SUST_CON: C2RustUnnamed_0 = 5;
pub const OF_SUST_DEX: C2RustUnnamed_0 = 4;
pub const OF_SUST_WIS: C2RustUnnamed_0 = 3;
pub const OF_SUST_INT: C2RustUnnamed_0 = 2;
pub const OF_SUST_STR: C2RustUnnamed_0 = 1;
pub const OF_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const OBJ_MOD_MAX: C2RustUnnamed_1 = 16;
pub const OBJ_MOD_MOVES: C2RustUnnamed_1 = 15;
pub const OBJ_MOD_DAM_RED: C2RustUnnamed_1 = 14;
pub const OBJ_MOD_LIGHT: C2RustUnnamed_1 = 13;
pub const OBJ_MOD_MIGHT: C2RustUnnamed_1 = 12;
pub const OBJ_MOD_SHOTS: C2RustUnnamed_1 = 11;
pub const OBJ_MOD_BLOWS: C2RustUnnamed_1 = 10;
pub const OBJ_MOD_SPEED: C2RustUnnamed_1 = 9;
pub const OBJ_MOD_TUNNEL: C2RustUnnamed_1 = 8;
pub const OBJ_MOD_INFRA: C2RustUnnamed_1 = 7;
pub const OBJ_MOD_SEARCH: C2RustUnnamed_1 = 6;
pub const OBJ_MOD_STEALTH: C2RustUnnamed_1 = 5;
pub const OBJ_MOD_CON: C2RustUnnamed_1 = 4;
pub const OBJ_MOD_DEX: C2RustUnnamed_1 = 3;
pub const OBJ_MOD_WIS: C2RustUnnamed_1 = 2;
pub const OBJ_MOD_INT: C2RustUnnamed_1 = 1;
pub const OBJ_MOD_STR: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const ELEM_MAX: C2RustUnnamed_2 = 25;
pub const ELEM_ARROW: C2RustUnnamed_2 = 24;
pub const ELEM_HOLY_ORB: C2RustUnnamed_2 = 23;
pub const ELEM_MANA: C2RustUnnamed_2 = 22;
pub const ELEM_MISSILE: C2RustUnnamed_2 = 21;
pub const ELEM_METEOR: C2RustUnnamed_2 = 20;
pub const ELEM_PLASMA: C2RustUnnamed_2 = 19;
pub const ELEM_TIME: C2RustUnnamed_2 = 18;
pub const ELEM_FORCE: C2RustUnnamed_2 = 17;
pub const ELEM_INERTIA: C2RustUnnamed_2 = 16;
pub const ELEM_GRAVITY: C2RustUnnamed_2 = 15;
pub const ELEM_ICE: C2RustUnnamed_2 = 14;
pub const ELEM_WATER: C2RustUnnamed_2 = 13;
pub const ELEM_DISEN: C2RustUnnamed_2 = 12;
pub const ELEM_CHAOS: C2RustUnnamed_2 = 11;
pub const ELEM_NETHER: C2RustUnnamed_2 = 10;
pub const ELEM_NEXUS: C2RustUnnamed_2 = 9;
pub const ELEM_SHARD: C2RustUnnamed_2 = 8;
pub const ELEM_SOUND: C2RustUnnamed_2 = 7;
pub const ELEM_DARK: C2RustUnnamed_2 = 6;
pub const ELEM_LIGHT: C2RustUnnamed_2 = 5;
pub const ELEM_POIS: C2RustUnnamed_2 = 4;
pub const ELEM_COLD: C2RustUnnamed_2 = 3;
pub const ELEM_FIRE: C2RustUnnamed_2 = 2;
pub const ELEM_ELEC: C2RustUnnamed_2 = 1;
pub const ELEM_ACID: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const EL_INFO_RANDOM: C2RustUnnamed_3 = 4;
pub const EL_INFO_IGNORE: C2RustUnnamed_3 = 2;
pub const EL_INFO_HATES: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const OPT_MAX: C2RustUnnamed_4 = 46;
pub const OPT_birth_percent_damage: C2RustUnnamed_4 = 45;
pub const OPT_birth_levels_persist: C2RustUnnamed_4 = 44;
pub const OPT_birth_know_flavors: C2RustUnnamed_4 = 43;
pub const OPT_birth_know_runes: C2RustUnnamed_4 = 42;
pub const OPT_birth_ai_learn: C2RustUnnamed_4 = 41;
pub const OPT_birth_start_kit: C2RustUnnamed_4 = 40;
pub const OPT_birth_no_selling: C2RustUnnamed_4 = 39;
pub const OPT_birth_feelings: C2RustUnnamed_4 = 38;
pub const OPT_birth_lose_arts: C2RustUnnamed_4 = 37;
pub const OPT_birth_stacking: C2RustUnnamed_4 = 36;
pub const OPT_birth_no_artifacts: C2RustUnnamed_4 = 35;
pub const OPT_birth_no_recall: C2RustUnnamed_4 = 34;
pub const OPT_birth_force_descend: C2RustUnnamed_4 = 33;
pub const OPT_birth_connect_stairs: C2RustUnnamed_4 = 32;
pub const OPT_birth_randarts: C2RustUnnamed_4 = 31;
pub const OPT_score_live: C2RustUnnamed_4 = 30;
pub const OPT_cheat_live: C2RustUnnamed_4 = 29;
pub const OPT_score_xtra: C2RustUnnamed_4 = 28;
pub const OPT_cheat_xtra: C2RustUnnamed_4 = 27;
pub const OPT_score_room: C2RustUnnamed_4 = 26;
pub const OPT_cheat_room: C2RustUnnamed_4 = 25;
pub const OPT_score_hear: C2RustUnnamed_4 = 24;
pub const OPT_cheat_hear: C2RustUnnamed_4 = 23;
pub const OPT_effective_speed: C2RustUnnamed_4 = 22;
pub const OPT_notify_recharge: C2RustUnnamed_4 = 21;
pub const OPT_mouse_movement: C2RustUnnamed_4 = 20;
pub const OPT_hp_changes_color: C2RustUnnamed_4 = 19;
pub const OPT_auto_more: C2RustUnnamed_4 = 18;
pub const OPT_purple_uniques: C2RustUnnamed_4 = 17;
pub const OPT_center_player: C2RustUnnamed_4 = 16;
pub const OPT_animate_flicker: C2RustUnnamed_4 = 15;
pub const OPT_view_yellow_light: C2RustUnnamed_4 = 14;
pub const OPT_hybrid_walls: C2RustUnnamed_4 = 13;
pub const OPT_solid_walls: C2RustUnnamed_4 = 12;
pub const OPT_disturb_near: C2RustUnnamed_4 = 11;
pub const OPT_highlight_player: C2RustUnnamed_4 = 10;
pub const OPT_show_target: C2RustUnnamed_4 = 9;
pub const OPT_show_flavors: C2RustUnnamed_4 = 8;
pub const OPT_pickup_inven: C2RustUnnamed_4 = 7;
pub const OPT_pickup_always: C2RustUnnamed_4 = 6;
pub const OPT_use_old_target: C2RustUnnamed_4 = 5;
pub const OPT_show_damage: C2RustUnnamed_4 = 4;
pub const OPT_use_sound: C2RustUnnamed_4 = 3;
pub const OPT_autoexplore_commands: C2RustUnnamed_4 = 2;
pub const OPT_rogue_like_commands: C2RustUnnamed_4 = 1;
pub const OPT_none: C2RustUnnamed_4 = 0;
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
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const BORG_CURSE_MAX: C2RustUnnamed_5 = 28;
pub const BORG_CURSE_AIR_SWING: C2RustUnnamed_5 = 27;
pub const BORG_CURSE_STEELSKIN: C2RustUnnamed_5 = 26;
pub const BORG_CURSE_CHILLED_TO_THE_BONE: C2RustUnnamed_5 = 25;
pub const BORG_CURSE_BURNING_UP: C2RustUnnamed_5 = 24;
pub const BORG_CURSE_TREACHEROUS_WEAPON: C2RustUnnamed_5 = 23;
pub const BORG_CURSE_ANTI_TELEPORTATION: C2RustUnnamed_5 = 22;
pub const BORG_CURSE_STONE: C2RustUnnamed_5 = 21;
pub const BORG_CURSE_COWARDICE: C2RustUnnamed_5 = 20;
pub const BORG_CURSE_IMPAIR_HITPOINT_RECOVERY: C2RustUnnamed_5 = 19;
pub const BORG_CURSE_IMPAIR_MANA_RECOVERY: C2RustUnnamed_5 = 18;
pub const BORG_CURSE_UNDEAD_SUMMON: C2RustUnnamed_5 = 17;
pub const BORG_CURSE_DEMON_SUMMON: C2RustUnnamed_5 = 16;
pub const BORG_CURSE_DRAGON_SUMMON: C2RustUnnamed_5 = 15;
pub const BORG_CURSE_PARALYSIS: C2RustUnnamed_5 = 14;
pub const BORG_CURSE_HALLUCINATION: C2RustUnnamed_5 = 13;
pub const BORG_CURSE_SIREN: C2RustUnnamed_5 = 12;
pub const BORG_CURSE_POISON: C2RustUnnamed_5 = 11;
pub const BORG_CURSE_ANNOYANCE: C2RustUnnamed_5 = 10;
pub const BORG_CURSE_SLOWNESS: C2RustUnnamed_5 = 9;
pub const BORG_CURSE_CLUMSINESS: C2RustUnnamed_5 = 8;
pub const BORG_CURSE_WEAKNESS: C2RustUnnamed_5 = 7;
pub const BORG_CURSE_IRRITATION: C2RustUnnamed_5 = 6;
pub const BORG_CURSE_ENVELOPING: C2RustUnnamed_5 = 5;
pub const BORG_CURSE_SICKLINESS: C2RustUnnamed_5 = 4;
pub const BORG_CURSE_DULLNESS: C2RustUnnamed_5 = 3;
pub const BORG_CURSE_TELEPORTATION: C2RustUnnamed_5 = 2;
pub const BORG_CURSE_VULNERABILITY: C2RustUnnamed_5 = 1;
pub const BORG_CURSE_UNKNOWN: C2RustUnnamed_5 = 0;
pub const BI_GOLD: C2RustUnnamed_14 = 45;
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
pub struct borg_best {
    pub home: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
}
pub type borg_time = int32_t;
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
pub struct borg_shop {
    pub ware: *mut borg_item,
}
pub const BI_QUIVER_SLOTS: C2RustUnnamed_14 = 158;
pub const BI_MAXDEPTH: C2RustUnnamed_14 = 106;
pub const BI_MAXSP: C2RustUnnamed_14 = 31;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_15 = 24;
pub const BI_MAXCLEVEL: C2RustUnnamed_14 = 36;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_15 = 9;
pub const BI_CLEVEL: C2RustUnnamed_14 = 35;
pub const BI_CLASS: C2RustUnnamed_14 = 25;
pub const BI_AMISSILES: C2RustUnnamed_14 = 155;
pub const BI_AMMO_TVAL: C2RustUnnamed_14 = 152;
pub type keycode_t = uint32_t;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_14 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_14 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_14 = 263;
pub const BI_EMPTY: C2RustUnnamed_14 = 262;
pub const BI_CARRY: C2RustUnnamed_14 = 261;
pub const BI_WEIGHT: C2RustUnnamed_14 = 260;
pub const BI_DINV: C2RustUnnamed_14 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_14 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_14 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_14 = 256;
pub const BI_ADIGGER: C2RustUnnamed_14 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_14 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_14 = 253;
pub const BI_AROD2: C2RustUnnamed_14 = 252;
pub const BI_AROD1: C2RustUnnamed_14 = 251;
pub const BI_ASHROOM: C2RustUnnamed_14 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_14 = 249;
pub const BI_AHWORD: C2RustUnnamed_14 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_14 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_14 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_14 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_14 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_14 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_14 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_14 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_14 = 240;
pub const BI_ABRAND: C2RustUnnamed_14 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_14 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_14 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_14 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_14 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_14 = 234;
pub const BI_ACLW: C2RustUnnamed_14 = 233;
pub const BI_ACSW: C2RustUnnamed_14 = 232;
pub const BI_ACCW: C2RustUnnamed_14 = 231;
pub const BI_AGLYPH: C2RustUnnamed_14 = 230;
pub const BI_APFE: C2RustUnnamed_14 = 229;
pub const BI_ALITE: C2RustUnnamed_14 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_14 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_14 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_14 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_14 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_14 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_14 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_14 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_14 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_14 = 219;
pub const BI_ASPEED: C2RustUnnamed_14 = 218;
pub const BI_AID: C2RustUnnamed_14 = 217;
pub const BI_ALIFE: C2RustUnnamed_14 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_14 = 215;
pub const BI_AHEAL: C2RustUnnamed_14 = 214;
pub const BI_AFUEL: C2RustUnnamed_14 = 213;
pub const BI_AESCAPE: C2RustUnnamed_14 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_14 = 211;
pub const BI_APHASE: C2RustUnnamed_14 = 210;
pub const BI_WB_POIS: C2RustUnnamed_14 = 209;
pub const BI_WB_COLD: C2RustUnnamed_14 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_14 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_14 = 206;
pub const BI_WB_ACID: C2RustUnnamed_14 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_14 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_14 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_14 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_14 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_14 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_14 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_14 = 198;
pub const BI_WS_ORC: C2RustUnnamed_14 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_14 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_14 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_14 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_14 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_14 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_14 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_14 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_14 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_14 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_14 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_14 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_14 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_14 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_14 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_14 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_14 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_14 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_14 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_14 = 178;
pub const BI_CRSSICK: C2RustUnnamed_14 = 177;
pub const BI_CRSDULL: C2RustUnnamed_14 = 176;
pub const BI_CRSVULN: C2RustUnnamed_14 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_14 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_14 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_14 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_14 = 171;
pub const BI_CRSSUND: C2RustUnnamed_14 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_14 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_14 = 168;
pub const BI_CRSPARA: C2RustUnnamed_14 = 167;
pub const BI_CRSHALU: C2RustUnnamed_14 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_14 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_14 = 164;
pub const BI_CRSTELE: C2RustUnnamed_14 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_14 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_14 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_14 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_14 = 159;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_14 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_14 = 156;
pub const BI_AMMO_POWER: C2RustUnnamed_14 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_14 = 153;
pub const BI_AMMO_COUNT: C2RustUnnamed_14 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_14 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_14 = 149;
pub const BI_SHOTS: C2RustUnnamed_14 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_14 = 147;
pub const BI_BLOWS: C2RustUnnamed_14 = 146;
pub const BI_BART: C2RustUnnamed_14 = 145;
pub const BI_SLING: C2RustUnnamed_14 = 144;
pub const BI_BTODAM: C2RustUnnamed_14 = 143;
pub const BI_BTOHIT: C2RustUnnamed_14 = 142;
pub const BI_BID: C2RustUnnamed_14 = 141;
pub const BI_WDS: C2RustUnnamed_14 = 140;
pub const BI_WDD: C2RustUnnamed_14 = 139;
pub const BI_WID: C2RustUnnamed_14 = 138;
pub const BI_WTODAM: C2RustUnnamed_14 = 137;
pub const BI_WTOHIT: C2RustUnnamed_14 = 136;
pub const BI_TODAM: C2RustUnnamed_14 = 135;
pub const BI_TOHIT: C2RustUnnamed_14 = 134;
pub const BI_ARMOR: C2RustUnnamed_14 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_14 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_14 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_14 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_14 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_14 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_14 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_14 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_14 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_14 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_14 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_14 = 122;
pub const BI_ISFORGET: C2RustUnnamed_14 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_14 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_14 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_14 = 118;
pub const BI_ISSTUN: C2RustUnnamed_14 = 117;
pub const BI_ISCUT: C2RustUnnamed_14 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_14 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_14 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_14 = 113;
pub const BI_ISBLIND: C2RustUnnamed_14 = 112;
pub const BI_ISGORGED: C2RustUnnamed_14 = 111;
pub const BI_ISFULL: C2RustUnnamed_14 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_14 = 109;
pub const BI_ISWEAK: C2RustUnnamed_14 = 108;
pub const BI_KING: C2RustUnnamed_14 = 107;
pub const BI_CDEPTH: C2RustUnnamed_14 = 105;
pub const BI_SFRACT: C2RustUnnamed_14 = 104;
pub const BI_SHLIFE: C2RustUnnamed_14 = 103;
pub const BI_SRDIS: C2RustUnnamed_14 = 102;
pub const BI_SRKAOS: C2RustUnnamed_14 = 101;
pub const BI_SRNTHR: C2RustUnnamed_14 = 100;
pub const BI_SRNXUS: C2RustUnnamed_14 = 99;
pub const BI_SRSHRD: C2RustUnnamed_14 = 98;
pub const BI_SRSND: C2RustUnnamed_14 = 97;
pub const BI_SRCONF: C2RustUnnamed_14 = 96;
pub const BI_SRBLIND: C2RustUnnamed_14 = 95;
pub const BI_SRDARK: C2RustUnnamed_14 = 94;
pub const BI_SRLITE: C2RustUnnamed_14 = 93;
pub const BI_SRFEAR: C2RustUnnamed_14 = 92;
pub const BI_SRPOIS: C2RustUnnamed_14 = 91;
pub const BI_SRACID: C2RustUnnamed_14 = 90;
pub const BI_SRELEC: C2RustUnnamed_14 = 89;
pub const BI_SRCOLD: C2RustUnnamed_14 = 88;
pub const BI_SRFIRE: C2RustUnnamed_14 = 87;
pub const BI_FRACT: C2RustUnnamed_14 = 86;
pub const BI_HLIFE: C2RustUnnamed_14 = 85;
pub const BI_RDIS: C2RustUnnamed_14 = 84;
pub const BI_RKAOS: C2RustUnnamed_14 = 83;
pub const BI_RNTHR: C2RustUnnamed_14 = 82;
pub const BI_RNXUS: C2RustUnnamed_14 = 81;
pub const BI_RSHRD: C2RustUnnamed_14 = 80;
pub const BI_RSND: C2RustUnnamed_14 = 79;
pub const BI_RCONF: C2RustUnnamed_14 = 78;
pub const BI_RBLIND: C2RustUnnamed_14 = 77;
pub const BI_RDARK: C2RustUnnamed_14 = 76;
pub const BI_RLITE: C2RustUnnamed_14 = 75;
pub const BI_RFEAR: C2RustUnnamed_14 = 74;
pub const BI_RPOIS: C2RustUnnamed_14 = 73;
pub const BI_RACID: C2RustUnnamed_14 = 72;
pub const BI_RELEC: C2RustUnnamed_14 = 71;
pub const BI_RCOLD: C2RustUnnamed_14 = 70;
pub const BI_RFIRE: C2RustUnnamed_14 = 69;
pub const BI_IPOIS: C2RustUnnamed_14 = 68;
pub const BI_IELEC: C2RustUnnamed_14 = 67;
pub const BI_ICOLD: C2RustUnnamed_14 = 66;
pub const BI_IACID: C2RustUnnamed_14 = 65;
pub const BI_IFIRE: C2RustUnnamed_14 = 64;
pub const BI_DIG: C2RustUnnamed_14 = 63;
pub const BI_THT: C2RustUnnamed_14 = 62;
pub const BI_THB: C2RustUnnamed_14 = 61;
pub const BI_THN: C2RustUnnamed_14 = 60;
pub const BI_SRCH: C2RustUnnamed_14 = 59;
pub const BI_STL: C2RustUnnamed_14 = 58;
pub const BI_SAV: C2RustUnnamed_14 = 57;
pub const BI_DEV: C2RustUnnamed_14 = 56;
pub const BI_DISM: C2RustUnnamed_14 = 55;
pub const BI_DISP: C2RustUnnamed_14 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_14 = 53;
pub const BI_INFRA: C2RustUnnamed_14 = 52;
pub const BI_SINV: C2RustUnnamed_14 = 51;
pub const BI_REG: C2RustUnnamed_14 = 50;
pub const BI_FEATH: C2RustUnnamed_14 = 49;
pub const BI_SDIG: C2RustUnnamed_14 = 48;
pub const BI_DAM_RED: C2RustUnnamed_14 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_14 = 46;
pub const BI_SPEED: C2RustUnnamed_14 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_14 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_14 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_14 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_14 = 40;
pub const BI_FOOD: C2RustUnnamed_14 = 39;
pub const BI_RECALL: C2RustUnnamed_14 = 38;
pub const BI_ESP: C2RustUnnamed_14 = 37;
pub const BI_FAIL2: C2RustUnnamed_14 = 34;
pub const BI_FAIL1: C2RustUnnamed_14 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_14 = 32;
pub const BI_CURSP: C2RustUnnamed_14 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_14 = 29;
pub const BI_MAXHP: C2RustUnnamed_14 = 28;
pub const BI_CURHP: C2RustUnnamed_14 = 27;
pub const BI_LIGHT: C2RustUnnamed_14 = 26;
pub const BI_SCON: C2RustUnnamed_14 = 24;
pub const BI_SDEX: C2RustUnnamed_14 = 23;
pub const BI_SWIS: C2RustUnnamed_14 = 22;
pub const BI_SINT: C2RustUnnamed_14 = 21;
pub const BI_SSTR: C2RustUnnamed_14 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_14 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_14 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_14 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_14 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_14 = 15;
pub const BI_CCON: C2RustUnnamed_14 = 14;
pub const BI_CDEX: C2RustUnnamed_14 = 13;
pub const BI_CWIS: C2RustUnnamed_14 = 12;
pub const BI_CINT: C2RustUnnamed_14 = 11;
pub const BI_CSTR: C2RustUnnamed_14 = 10;
pub const BI_ACON: C2RustUnnamed_14 = 9;
pub const BI_ADEX: C2RustUnnamed_14 = 8;
pub const BI_AWIS: C2RustUnnamed_14 = 7;
pub const BI_AINT: C2RustUnnamed_14 = 6;
pub const BI_ASTR: C2RustUnnamed_14 = 5;
pub const BI_CON: C2RustUnnamed_14 = 4;
pub const BI_DEX: C2RustUnnamed_14 = 3;
pub const BI_WIS: C2RustUnnamed_14 = 2;
pub const BI_INT: C2RustUnnamed_14 = 1;
pub const BI_STR: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_15 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_15 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_15 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_15 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_15 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_15 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_15 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_15 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_15 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_15 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_15 = 25;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_15 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_15 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_15 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_15 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_15 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_15 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_15 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_15 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_15 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_15 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_15 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_15 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_15 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_15 = 10;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_15 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_15 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_15 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_15 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_15 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_15 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_15 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_15 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_15 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FLAG_WIDTH: usize = (::core::mem::size_of::<bitflag>() as usize).wrapping_mul(8 as usize);
#[no_mangle]
pub static mut sold_item_tval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_sval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_pval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_store: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut sold_item_num: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut sold_item_nxt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut test_item: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
#[no_mangle]
pub static mut best_item: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
unsafe extern "C" fn borg_object_similar(
    mut o_ptr: *mut borg_item,
    mut j_ptr: *mut borg_item,
) -> bool {
    let mut total: ::core::ffi::c_int =
        (*o_ptr).iqty as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if (*o_ptr).kind != (*j_ptr).kind {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !flag_is_equal(
        &raw mut (*o_ptr).flags as *mut bitflag,
        &raw mut (*j_ptr).flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add(FLAG_WIDTH)
            .wrapping_sub(1 as size_t)
            .wrapping_div(FLAG_WIDTH),
    ) {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ELEM_MAX as ::core::ffi::c_int {
        if (*o_ptr).el_info[i as usize].res_level as ::core::ffi::c_int
            != (*j_ptr).el_info[i as usize].res_level as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if (*o_ptr).el_info[i as usize].flags as ::core::ffi::c_int
            & (EL_INFO_HATES as ::core::ffi::c_int | EL_INFO_IGNORE as ::core::ffi::c_int)
            != (*j_ptr).el_info[i as usize].flags as ::core::ffi::c_int
                & (EL_INFO_HATES as ::core::ffi::c_int | EL_INFO_IGNORE as ::core::ffi::c_int)
        {
            return false_0 != 0;
        }
        i += 1;
    }
    let mut current_block_60: u64;
    match (*o_ptr).tval as ::core::ffi::c_int {
        1 => return 0 as ::core::ffi::c_int != 0,
        22 | 23 => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 2116367355679836638;
        }
        28 | 26 | 25 | 24 => {
            current_block_60 = 2116367355679836638;
        }
        5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 21 | 20 | 19 => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 15925075030174552612;
        }
        4 | 3 | 2 => {
            current_block_60 = 15925075030174552612;
        }
        _ => {
            if !(*o_ptr).aware || !(*j_ptr).aware {
                return 0 as ::core::ffi::c_int != 0;
            }
            current_block_60 = 2116367355679836638;
        }
    }
    match current_block_60 {
        15925075030174552612 => {
            if (*o_ptr).to_h as ::core::ffi::c_int != (*j_ptr).to_h as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).to_d as ::core::ffi::c_int != (*j_ptr).to_d as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).to_a as ::core::ffi::c_int != (*j_ptr).to_a as ::core::ffi::c_int {
                return false_0 != 0;
            }
            i = 0 as ::core::ffi::c_int;
            while i < OBJ_MOD_MAX as ::core::ffi::c_int {
                if (*o_ptr).modifiers[i as usize] as ::core::ffi::c_int
                    != (*j_ptr).modifiers[i as usize] as ::core::ffi::c_int
                {
                    return false_0 != 0;
                }
                i += 1;
            }
            if (*o_ptr).cursed as ::core::ffi::c_int != (*j_ptr).cursed as ::core::ffi::c_int
                || (*o_ptr).uncursable as ::core::ffi::c_int
                    != (*j_ptr).uncursable as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            if (*o_ptr).cursed {
                i = 0 as ::core::ffi::c_int;
                while i < BORG_CURSE_MAX as ::core::ffi::c_int {
                    if (*o_ptr).curses[i as usize] as ::core::ffi::c_int
                        != (*j_ptr).curses[i as usize] as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    i += 1;
                }
            }
            if (*o_ptr).art_idx as ::core::ffi::c_int != (*j_ptr).art_idx as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).ego_idx as ::core::ffi::c_int != (*j_ptr).ego_idx as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if !flag_is_empty(
                &raw mut (*o_ptr).flags as *mut bitflag,
                (OF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(FLAG_WIDTH)
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(FLAG_WIDTH),
            ) || !flag_is_empty(
                &raw mut (*j_ptr).flags as *mut bitflag,
                (OF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(FLAG_WIDTH)
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(FLAG_WIDTH),
            ) {
                return false_0 != 0;
            }
            if (*o_ptr).timeout as ::core::ffi::c_int != 0
                || (*j_ptr).timeout as ::core::ffi::c_int != 0
            {
                return false_0 != 0;
            }
            if (*o_ptr).ac as ::core::ffi::c_int != (*j_ptr).ac as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).dd as ::core::ffi::c_int != (*j_ptr).dd as ::core::ffi::c_int {
                return false_0 != 0;
            }
            if (*o_ptr).ds as ::core::ffi::c_int != (*j_ptr).ds as ::core::ffi::c_int {
                return false_0 != 0;
            }
        }
        _ => {}
    }
    if (*o_ptr).ident as ::core::ffi::c_int != (*j_ptr).ident as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !(*o_ptr).note.is_null() && (*j_ptr).note.is_null()
        || (*o_ptr).note.is_null() && !(*j_ptr).note.is_null()
    {
        return 0 as ::core::ffi::c_int != 0;
    }
    if !(*o_ptr).note.is_null() && !(*j_ptr).note.is_null() {
        if *(*o_ptr).note.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && *(*j_ptr).note.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcmp((*o_ptr).note, (*j_ptr).note) != 0
        {
            return 0 as ::core::ffi::c_int != 0;
        }
        if strcmp((*o_ptr).note, (*j_ptr).note) != 0 {
            return 0 as ::core::ffi::c_int != 0;
        }
    }
    if total >= (*(*k_info.offset((*o_ptr).kind as isize)).base).max_stack {
        return 0 as ::core::ffi::c_int != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_min_item_quantity(mut item: *mut borg_item) -> ::core::ffi::c_int {
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) < 250 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if (*item).value > 5 as int32_t {
        return 1 as ::core::ffi::c_int;
    }
    if !(*item).aware {
        return 1 as ::core::ffi::c_int;
    }
    match (*item).tval as ::core::ffi::c_int {
        2 | 3 | 4 => {
            if ((*item).iqty as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
                return (*item).iqty as ::core::ffi::c_int;
            }
            return 5 as ::core::ffi::c_int;
        }
        28 => {
            if ((*item).iqty as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                return (*item).iqty as ::core::ffi::c_int;
            }
            return 3 as ::core::ffi::c_int;
        }
        _ => return 1 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn borg_think_home_sell_bad(
    mut i: ::core::ffi::c_int,
    mut borg_empty_home_power: int32_t,
) -> bool {
    let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*item).iqty == 0 || !(*item).aware {
        return true_0 != 0;
    }
    if weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0
        && (*item).art_idx as ::core::ffi::c_int != 0
        && !(*item).ident
    {
        return true_0 != 0;
    }
    if (*item).value == 0 {
        return true_0 != 0;
    }
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < bought_item_num {
        if bought_item_tval[p as usize] == (*item).tval as ::core::ffi::c_int
            && bought_item_sval[p as usize] == (*item).sval as ::core::ffi::c_int
            && bought_item_pval[p as usize] == (*item).pval as ::core::ffi::c_int
            && bought_item_store[p as usize] == BORG_HOME
        {
            return true_0 != 0;
        }
        p += 1;
    }
    borg_notice_home(item, false_0 != 0);
    if borg_power_home() <= borg_empty_home_power {
        return true_0 != 0;
    }
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        charge_each = (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
    }
    (*item).iqty = (*item).iqty.wrapping_sub(1);
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = ((*item).pval as ::core::ffi::c_int - charge_each) as int16_t;
    }
    borg_notice(true_0 != 0);
    if borg_power() < borg.power {
        (*item).iqty = (*item).iqty.wrapping_add(1);
        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
        {
            (*item).pval = ((*item).pval as ::core::ffi::c_int + charge_each) as int16_t;
        }
        return true_0 != 0;
    }
    (*item).iqty = (*item).iqty.wrapping_add(1);
    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = ((*item).pval as ::core::ffi::c_int + charge_each) as int16_t;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_home_sell_best(mut best_home_power: *mut int32_t) {
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut item2: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut home_power: int32_t = 0;
    let mut borg_empty_home_power: int32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), true_0 != 0);
    borg_empty_home_power = borg_power_home();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    *best_home_power = borg_power_home();
    let mut first_empty: ::core::ffi::c_int = 0;
    first_empty = 0 as ::core::ffi::c_int;
    while first_empty < (*z_info).store_inven_max as ::core::ffi::c_int {
        if (*(*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(first_empty as isize))
        .iqty as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        first_empty += 1;
    }
    if first_empty < (*z_info).store_inven_max as ::core::ffi::c_int {
        first_empty += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < first_empty {
        item2 = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        let mut current_block_39: u64;
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).pack_size as ::core::ffi::c_int {
            item = borg_items.offset(i as isize) as *mut borg_item;
            if !borg_think_home_sell_bad(i, borg_empty_home_power) {
                if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                    || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                {
                    charge_each =
                        (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
                }
                if borg_object_similar(item2, item) {
                    (*item2).iqty = (*item2).iqty.wrapping_add(1);
                    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                    {
                        (*item2).pval =
                            ((*item2).pval as ::core::ffi::c_int + charge_each) as int16_t;
                    }
                    current_block_39 = 17281240262373992796;
                } else {
                    let mut found_match: bool = false_0 != 0;
                    k = 0 as ::core::ffi::c_int;
                    while k < (*z_info).store_inven_max as ::core::ffi::c_int {
                        if borg_object_similar(
                            (*borg_safe_shops.offset(BORG_HOME as isize))
                                .ware
                                .offset(k as isize) as *mut borg_item,
                            item,
                        ) {
                            found_match = true_0 != 0;
                            break;
                        } else {
                            k += 1;
                        }
                    }
                    if found_match {
                        current_block_39 = 10599921512955367680;
                    } else {
                        memcpy(
                            item2 as *mut ::core::ffi::c_void,
                            item as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        (*item2).iqty = 1 as uint8_t;
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            (*item2).pval = charge_each as int16_t;
                        }
                        current_block_39 = 17281240262373992796;
                    }
                }
                match current_block_39 {
                    10599921512955367680 => {}
                    _ => {
                        (*item).iqty = (*item).iqty.wrapping_sub(1);
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            (*item).pval =
                                ((*item).pval as ::core::ffi::c_int - charge_each) as int16_t;
                        }
                        *test_item.offset(n as isize) =
                            (i + (*z_info).store_inven_max as ::core::ffi::c_int) as uint8_t;
                        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
                        home_power = borg_power_home();
                        if home_power > *best_home_power {
                            k = 0 as ::core::ffi::c_int;
                            while k < (*z_info).store_inven_max as ::core::ffi::c_int {
                                *best_item.offset(k as isize) = *test_item.offset(k as isize);
                                k += 1;
                            }
                            *best_home_power = home_power;
                        }
                        memcpy(
                            item2 as *mut ::core::ffi::c_void,
                            (*borg_safe_shops.offset(BORG_HOME as isize))
                                .ware
                                .offset(n as isize) as *mut borg_item
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        (*item).iqty = (*item).iqty.wrapping_add(1);
                        *test_item.offset(n as isize) = n as uint8_t;
                    }
                }
            }
            i += 1;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_sell_useful(mut best_home_power: *mut int32_t) -> bool {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    memset(
        test_item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as size_t,
    );
    memset(
        best_item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as size_t,
    );
    if (*(*borg_shops.offset(BORG_HOME as isize)).ware.offset(
        ((*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty as ::core::ffi::c_int
        != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        let ref mut fresh0 = *test_item.offset(i as isize);
        *fresh0 = i as uint8_t;
        *best_item.offset(i as isize) = *fresh0;
        i += 1;
    }
    *best_home_power = -(1 as ::core::ffi::c_int) as int32_t;
    borg_think_home_sell_best(best_home_power);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        memcpy(
            (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            (*borg_safe_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        memcpy(
            borg_items.offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        i += 1;
    }
    borg_notice(true_0 != 0);
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        if *best_item.offset(i as isize) as ::core::ffi::c_int != i
            && *best_item.offset(i as isize) as ::core::ffi::c_int != 255 as ::core::ffi::c_int
        {
            let mut item: *mut borg_item = borg_items.offset(
                (*best_item.offset(i as isize) as ::core::ffi::c_int
                    - (*z_info).store_inven_max as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
            let mut item2: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item;
            if borg_object_similar(item2, item) {
                if !((*item2).iqty as ::core::ffi::c_int > 90 as ::core::ffi::c_int) {
                    borg.goal.shop = BORG_HOME as int16_t;
                    borg.goal.item = (*best_item.offset(i as isize) as ::core::ffi::c_int
                        - (*z_info).store_inven_max as ::core::ffi::c_int)
                        as int16_t;
                    return true_0 != 0;
                }
            }
        }
        i += 1;
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
    {
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).store_inven_max as ::core::ffi::c_int {
            if *best_item.offset(i as isize) as ::core::ffi::c_int != i
                && (*(*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(i as isize))
                .iqty as ::core::ffi::c_int
                    != 0
            {
                let mut item_0: *mut borg_item = borg_items.offset(
                    (*best_item.offset(i as isize) as ::core::ffi::c_int
                        - (*z_info).store_inven_max as ::core::ffi::c_int)
                        as isize,
                ) as *mut borg_item;
                let mut item2_0: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(i as isize)
                    as *mut borg_item;
                if !borg_object_similar(item_0, item2_0) {
                    p = 0 as ::core::ffi::c_int;
                    while p < sold_item_num {
                        if sold_item_tval[p as usize] == (*item2_0).tval as ::core::ffi::c_int
                            && sold_item_sval[p as usize] == (*item2_0).sval as ::core::ffi::c_int
                            && sold_item_store[p as usize] == BORG_HOME
                        {
                            return false_0 != 0;
                        }
                        p += 1;
                    }
                    borg.goal.shop = BORG_HOME as int16_t;
                    borg.goal.ware = i as int16_t;
                    return true_0 != 0;
                }
            }
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        if *best_item.offset(i as isize) as ::core::ffi::c_int != i
            && *best_item.offset(i as isize) as ::core::ffi::c_int != 255 as ::core::ffi::c_int
        {
            if (*borg_items.offset(
                (*best_item.offset(i as isize) as ::core::ffi::c_int
                    - (*z_info).store_inven_max as ::core::ffi::c_int) as isize,
            ))
            .iqty
                == 0
            {
                return false_0 != 0;
            }
            borg.goal.shop = BORG_HOME as int16_t;
            borg.goal.item = (*best_item.offset(i as isize) as ::core::ffi::c_int
                - (*z_info).store_inven_max as ::core::ffi::c_int)
                as int16_t;
            return true_0 != 0;
        }
        i += 1;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    return false_0 != 0;
}
unsafe extern "C" fn borg_has_mutiple(mut in_item: *mut borg_item) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if (*in_item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if !(*in_item).ident {
        match (*in_item).tval as ::core::ffi::c_int {
            10 | 11 | 12 | 13 | 14 | 16 | 17 | 2 | 4 | 3 | 5 | 6 | 7 | 8 | 9 => {
                return false_0 != 0;
            }
            _ => {}
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !(item == in_item) {
            if (*item).tval as ::core::ffi::c_int == (*in_item).tval as ::core::ffi::c_int
                && (*item).sval as ::core::ffi::c_int == (*in_item).sval as ::core::ffi::c_int
                && (*item).iqty as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                return true_0 != 0;
            }
        }
        i += 1;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item_0: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item_0).tval as ::core::ffi::c_int == (*in_item).tval as ::core::ffi::c_int
            && (*item_0).sval as ::core::ffi::c_int == (*in_item).sval as ::core::ffi::c_int
            && (*item_0).iqty as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_store_buys(
    mut item: *mut borg_item,
    mut who: ::core::ffi::c_int,
) -> bool {
    match who + 1 as ::core::ffi::c_int {
        1 => {
            match (*item).tval as ::core::ffi::c_int {
                28 | 29 | 27 | 2 | 4 | 3 | 6 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        2 => {
            match (*item).tval as ::core::ffi::c_int {
                10 | 11 | 12 | 13 | 14 | 16 | 17 | 18 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        3 => {
            match (*item).tval as ::core::ffi::c_int {
                2 | 4 | 3 | 5 | 6 | 7 | 8 | 9 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        4 => {
            match (*item).tval as ::core::ffi::c_int {
                31 | 30 | 32 | 33 | 34 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        5 => {
            match (*item).tval as ::core::ffi::c_int {
                25 | 26 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        6 => {
            match (*item).tval as ::core::ffi::c_int {
                20 | 21 | 22 | 23 | 24 | 30 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        7 => {
            if (*player).opts.opt[OPT_birth_no_selling as ::core::ffi::c_int as usize] {
                return true_0 != 0;
            }
            match (*item).tval as ::core::ffi::c_int {
                19 | 15 | 28 => return true_0 != 0,
                _ => {}
            }
            return false_0 != 0;
        }
        _ => {}
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_good_sell(mut item: *mut borg_item, mut who: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut multiple: bool = false_0 != 0;
    if (*item).value <= 0 as int32_t {
        if !(((*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
            || (*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int)
            && !(*item).ident)
        {
            return false_0 != 0;
        }
    }
    if !borg_store_buys(item, who) {
        return false_0 != 0;
    }
    if borg_item_note_needs_id(item) {
        multiple = borg_has_mutiple(item);
        if !multiple {
            return false_0 != 0;
        }
    }
    if !((*item).value > 0 as int32_t
        && (*borg_cfg.offset(BORG_WORSHIPS_GOLD as ::core::ffi::c_int as isize) != 0
            || *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < 10 as ::core::ffi::c_int
            || *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
                    != 0 as ::core::ffi::c_int))
    {
        match (*item).tval as ::core::ffi::c_int {
            26 | 25 => {
                if !(*item).ident {
                    return true_0 != 0;
                }
                if (*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && (*item).sval as ::core::ffi::c_int == sv_potion_restore_mana
                    && *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                        > 100 as ::core::ffi::c_int
                    && *borg.has.offset(kv_potion_restore_mana as isize)
                        + num_mana as ::core::ffi::c_int
                        > 99 as ::core::ffi::c_int
                {
                    return false_0 != 0;
                }
            }
            28 | 24 | 23 | 22 | 21 | 20 | 19 => {
                if borg_item_worth_id(item) as ::core::ffi::c_int != 0
                    && *borg
                        .trait_0
                        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                        < 35 as ::core::ffi::c_int
                    && !multiple
                {
                    return false_0 != 0;
                }
            }
            5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => {
                if borg_item_worth_id(item) as ::core::ffi::c_int != 0 && !multiple {
                    return false_0 != 0;
                }
            }
            _ => {}
        }
    }
    if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0
        && (*item).art_idx as ::core::ffi::c_int != 0
        && !(*item).ident
    {
        return false_0 != 0;
    }
    if !(*item).ident
        && (*item).ego_idx as ::core::ffi::c_int != 0
        && ((*item).iqty as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        if borg_ego_has_random_power(
            e_info.offset(
                (*borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ))
                .ego_idx as isize,
            ) as *mut ego_item,
        ) {
            return false_0 != 0;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < bought_item_num {
        if bought_item_tval[i as usize] == (*item).tval as ::core::ffi::c_int
            && bought_item_sval[i as usize] == (*item).sval as ::core::ffi::c_int
            && (bought_item_store[i as usize] == who || who != BORG_HOME)
        {
            return false_0 != 0;
        }
        i += 1;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_sell_useless() -> bool {
    let mut icky: ::core::ffi::c_int =
        (*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut c: int32_t = 0 as int32_t;
    let mut b_c: int32_t = 30001 as int32_t;
    let mut fix: bool = false_0 != 0;
    b_p = borg.power;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).store_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        if !((*(*borg_shops.offset(k as isize)).ware.offset(icky as isize)).iqty != 0) {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).pack_size as ::core::ffi::c_int {
                let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
                if !((*item).iqty == 0) {
                    if !((*item).tval as ::core::ffi::c_int
                        == *borg
                            .trait_0
                            .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
                        && *borg
                            .trait_0
                            .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
                            < 45 as ::core::ffi::c_int)
                    {
                        if !((*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int
                            && (*item).sval as ::core::ffi::c_int == sv_rod_healing
                            && *borg.has.offset(kv_rod_healing as isize) <= 3 as ::core::ffi::c_int)
                        {
                            if !(*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                                == CLASS_WARRIOR
                                && (*item).tval as ::core::ffi::c_int
                                    == TV_ROD as ::core::ffi::c_int
                                && (*item).sval as ::core::ffi::c_int == sv_rod_mapping
                                && (*item).iqty as ::core::ffi::c_int <= 2 as ::core::ffi::c_int)
                            {
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_WAND as ::core::ffi::c_int
                                    && *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        < 35 as ::core::ffi::c_int
                                    && ((*item).sval as ::core::ffi::c_int
                                        == sv_wand_magic_missile
                                        || (*item).sval as ::core::ffi::c_int
                                            == sv_wand_stinking_cloud
                                        || (*item).sval as ::core::ffi::c_int
                                            == sv_wand_annihilation)
                                    && (*item).pval as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int)
                                {
                                    if !(weapon_swap != 0
                                        && i == weapon_swap - 1 as ::core::ffi::c_int)
                                    {
                                        if !(armour_swap != 0
                                            && i == armour_swap - 1 as ::core::ffi::c_int)
                                        {
                                            if borg_good_sell(item, k) {
                                                memcpy(
                                                    (*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    safe_items.offset(i as isize) as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                qty = borg_min_item_quantity(item);
                                                (*(*borg_shops.offset(k as isize))
                                                    .ware
                                                    .offset(icky as isize))
                                                .iqty = qty as uint8_t;
                                                let ref mut fresh1 =
                                                    (*borg_items.offset(i as isize)).iqty;
                                                *fresh1 = (*fresh1 as ::core::ffi::c_int - qty)
                                                    as uint8_t;
                                                if (*item).tval as ::core::ffi::c_int
                                                    == TV_STAFF as ::core::ffi::c_int
                                                    || (*item).tval as ::core::ffi::c_int
                                                        == TV_WAND as ::core::ffi::c_int
                                                {
                                                    let mut charge_loss: ::core::ffi::c_int =
                                                        (*safe_items.offset(i as isize)).pval
                                                            as ::core::ffi::c_int
                                                            / (*safe_items.offset(i as isize)).iqty
                                                                as ::core::ffi::c_int;
                                                    (*(*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize))
                                                    .pval = (qty * charge_loss) as int16_t;
                                                    let ref mut fresh2 =
                                                        (*borg_items.offset(i as isize)).pval;
                                                    *fresh2 = (*fresh2 as ::core::ffi::c_int
                                                        - qty * charge_loss)
                                                        as int16_t;
                                                }
                                                fix = true_0 != 0;
                                                borg_notice(true_0 != 0);
                                                p = borg_power();
                                                memcpy(
                                                    borg_items.offset(i as isize) as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    safe_items.offset(i as isize) as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                memcpy(
                                                    (*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    (*borg_safe_shops.offset(k as isize))
                                                        .ware
                                                        .offset(icky as isize)
                                                        as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                if !(p < b_p) {
                                                    c = (if ((*item).value as ::core::ffi::c_long)
                                                        < 30000 as ::core::ffi::c_long
                                                    {
                                                        (*item).value as ::core::ffi::c_long
                                                    } else {
                                                        30000 as ::core::ffi::c_long
                                                    })
                                                        as int32_t;
                                                    if !(p == b_p && c >= b_c) {
                                                        b_k = k;
                                                        b_i = i;
                                                        b_p = p;
                                                        b_c = c;
                                                    }
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
        }
        k += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_k >= 0 as ::core::ffi::c_int && b_i >= 0 as ::core::ffi::c_int {
        borg.goal.shop = b_k as int16_t;
        borg.goal.item = b_i as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_sell() -> bool {
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if borg.goal.shop as ::core::ffi::c_int == shop_num as ::core::ffi::c_int
        && borg.goal.item as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = borg_items.offset(borg.goal.item as isize) as *mut borg_item;
        qty = borg_min_item_quantity(item);
        borg_note(format(
            b"# Selling %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('s' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset(borg.goal.item as isize) as keycode_t,
        );
        if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            || qty >= 2 as ::core::ffi::c_int
        {
            if qty == 5 as ::core::ffi::c_int {
                borg_keypress('5' as i32 as keycode_t);
            }
            if qty == 4 as ::core::ffi::c_int {
                borg_keypress('4' as i32 as keycode_t);
            }
            if qty == 3 as ::core::ffi::c_int {
                borg_keypress('3' as i32 as keycode_t);
            }
            if qty == 2 as ::core::ffi::c_int {
                borg_keypress('2' as i32 as keycode_t);
            }
            borg_keypress(KC_ENTER as keycode_t);
        }
        if borg.goal.shop as ::core::ffi::c_int != BORG_HOME {
            borg_keypress(KC_ENTER as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
            borg_keypress(ESCAPE as keycode_t);
        }
        if sold_item_nxt >= 9 as ::core::ffi::c_int {
            sold_item_nxt = 0 as ::core::ffi::c_int;
        }
        sold_item_pval[sold_item_nxt as usize] = (*item).pval as ::core::ffi::c_int;
        sold_item_tval[sold_item_nxt as usize] = (*item).tval as ::core::ffi::c_int;
        sold_item_sval[sold_item_nxt as usize] = (*item).sval as ::core::ffi::c_int;
        sold_item_store[sold_item_nxt as usize] = borg.goal.shop as ::core::ffi::c_int;
        sold_item_num = sold_item_nxt;
        sold_item_nxt += 1;
        borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
        borg.goal.ware = borg.goal.item;
        borg.goal.shop = borg.goal.ware;
        borg.antibounce_count += 1;
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.in_shop = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_count_sell() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut price: int32_t = 0;
    let mut greed: int32_t = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut sv_qty: ::core::ffi::c_int = 0;
    greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) as ::core::ffi::c_long
        / 100 as ::core::ffi::c_long
        + 100 as ::core::ffi::c_long) as int32_t;
    if (greed as ::core::ffi::c_long) < 1000 as ::core::ffi::c_long {
        greed = 1000 as int32_t;
    }
    if greed as ::core::ffi::c_long > 25000 as ::core::ffi::c_long {
        greed = 25000 as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 50 as ::core::ffi::c_int
    {
        greed = 75000 as ::core::ffi::c_int as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 25 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 20 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 35 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 20 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 13 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 10 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long
            + 5 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 5 as ::core::ffi::c_int
    {
        greed = (*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long) as int32_t;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).value <= 0 as int32_t) {
                if !(weapon_swap != 0 && i == weapon_swap - 1 as ::core::ffi::c_int) {
                    if !(armour_swap != 0 && i == armour_swap - 1 as ::core::ffi::c_int) {
                        if !((*item).tval as ::core::ffi::c_int
                            == *borg
                                .trait_0
                                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize))
                        {
                            if !obj_kind_can_browse(
                                k_info.offset((*item).kind as isize) as *mut object_kind
                            ) {
                                if borg_item_note_needs_id(item) {
                                    if !borg_has_mutiple(item) {
                                        return false_0;
                                    }
                                }
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_POTION as ::core::ffi::c_int
                                    && (*item).sval as ::core::ffi::c_int == sv_potion_cure_serious
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == sv_potion_cure_critical
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == sv_potion_healing
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == sv_potion_star_healing
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == sv_potion_life
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_POTION as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == sv_potion_speed
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_STAFF as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == sv_staff_teleportation
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == sv_wand_drain_life
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int
                                            == sv_wand_annihilation
                                    || (*item).tval as ::core::ffi::c_int
                                        == TV_SCROLL as ::core::ffi::c_int
                                        && (*item).sval as ::core::ffi::c_int == sv_scroll_teleport)
                                {
                                    price = (if ((*item).value as ::core::ffi::c_long)
                                        < 30000 as ::core::ffi::c_long
                                    {
                                        (*item).value as ::core::ffi::c_long
                                    } else {
                                        30000 as ::core::ffi::c_long
                                    }) as int32_t;
                                    if !((price * (*item).iqty as int32_t) < greed
                                        && !borg_item_note_needs_id(item))
                                    {
                                        sv_qty = (*item).iqty as ::core::ffi::c_int;
                                        (*item).iqty = 0 as uint8_t;
                                        borg_notice(true_0 != 0);
                                        p = borg_power() as ::core::ffi::c_int;
                                        (*item).iqty = sv_qty as uint8_t;
                                        if !((p as int32_t + 50 as int32_t) < borg.power) {
                                            k += 1;
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
    borg_notice(true_0 != 0);
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_store_sell() {
    borg_free_store_sell();
    test_item = mem_zalloc(
        ((*z_info).store_inven_max as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    best_item = mem_zalloc(
        ((*z_info).store_inven_max as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_store_sell() {
    if !test_item.is_null() {
        mem_free(test_item as *mut ::core::ffi::c_void);
        test_item = ::core::ptr::null_mut::<uint8_t>();
    }
    if !best_item.is_null() {
        mem_free(best_item as *mut ::core::ffi::c_void);
        best_item = ::core::ptr::null_mut::<uint8_t>();
    }
}
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
