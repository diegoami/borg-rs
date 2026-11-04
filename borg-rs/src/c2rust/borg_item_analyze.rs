#[repr(C)]
pub struct dice_s {
    _opaque: [u8; 0],
}

extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn flag_next(
        flags: *const bitflag,
        size: size_t,
        flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn my_strcpy(
        buf: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        bufsize: size_t,
    ) -> size_t;
    fn randcalc(
        v: random_value,
        level: ::core::ffi::c_int,
        rand_aspect: aspect,
    ) -> ::core::ffi::c_int;
    fn lookup_obj_property(
        type_0: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
    ) -> *mut obj_property;
    static mut k_info: *mut object_kind;
    static mut a_info: *mut artifact;
    static mut e_info: *mut ego_item;
    static mut z_info: *mut angband_constants;
    static mut curses: *mut curse;
    fn object_runes_known(obj: *const object) -> bool;
    fn object_fully_known(obj: *const object) -> bool;
    fn object_flavor_is_aware(obj: *const object) -> bool;
    static mut slays: *mut slay;
    fn object_weight_one(obj: *const object) -> int16_t;
    fn object_flags_known(obj: *const object, flags: *mut bitflag);
    static mut act_dragon_power: ::core::ffi::c_int;
    static mut act_dragon_shining: ::core::ffi::c_int;
    static mut act_dragon_balance: ::core::ffi::c_int;
    static mut act_dragon_law: ::core::ffi::c_int;
    static mut act_dragon_chaos: ::core::ffi::c_int;
    static mut act_dragon_gold: ::core::ffi::c_int;
    static mut act_dragon_multihued: ::core::ffi::c_int;
    static mut act_dragon_red: ::core::ffi::c_int;
    static mut act_dragon_green: ::core::ffi::c_int;
    static mut act_dragon_blue: ::core::ffi::c_int;
    static mut act_ring_lightning: ::core::ffi::c_int;
    static mut act_ring_ice: ::core::ffi::c_int;
    static mut act_ring_flames: ::core::ffi::c_int;
    static mut act_ring_acid: ::core::ffi::c_int;
    static mut act_bizarre: ::core::ffi::c_int;
    static mut sv_ring_flames: ::core::ffi::c_int;
    static mut sv_ring_ice: ::core::ffi::c_int;
    static mut sv_ring_acid: ::core::ffi::c_int;
    static mut sv_ring_lightning: ::core::ffi::c_int;
    static mut sv_ring_dog: ::core::ffi::c_int;
    static mut sv_dragon_blue: ::core::ffi::c_int;
    static mut sv_dragon_red: ::core::ffi::c_int;
    static mut sv_dragon_green: ::core::ffi::c_int;
    static mut sv_dragon_multihued: ::core::ffi::c_int;
    static mut sv_dragon_shining: ::core::ffi::c_int;
    static mut sv_dragon_law: ::core::ffi::c_int;
    static mut sv_dragon_gold: ::core::ffi::c_int;
    static mut sv_dragon_chaos: ::core::ffi::c_int;
    static mut sv_dragon_balance: ::core::ffi::c_int;
    static mut sv_dragon_power: ::core::ffi::c_int;
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
pub type aspect = ::core::ffi::c_uint;
pub const RANDOMISE: aspect = 4;
pub const EXTREMIFY: aspect = 3;
pub const MAXIMISE: aspect = 2;
pub const AVERAGE: aspect = 1;
pub const MINIMISE: aspect = 0;
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
pub const KF_MAX: C2RustUnnamed_1 = 15;
pub const KF_RAND_RES_POWER: C2RustUnnamed_1 = 14;
pub const KF_RAND_BASE_RES: C2RustUnnamed_1 = 13;
pub const KF_SHOOTS_BOLTS: C2RustUnnamed_1 = 12;
pub const KF_SHOOTS_ARROWS: C2RustUnnamed_1 = 11;
pub const KF_SHOOTS_SHOTS: C2RustUnnamed_1 = 10;
pub const KF_SHOW_MULT: C2RustUnnamed_1 = 9;
pub const KF_SHOW_DICE: C2RustUnnamed_1 = 8;
pub const KF_GOOD: C2RustUnnamed_1 = 7;
pub const KF_EASY_KNOW: C2RustUnnamed_1 = 6;
pub const KF_QUEST_ART: C2RustUnnamed_1 = 5;
pub const KF_INSTA_ART: C2RustUnnamed_1 = 4;
pub const KF_RAND_POWER: C2RustUnnamed_1 = 3;
pub const KF_RAND_SUSTAIN: C2RustUnnamed_1 = 2;
pub const KF_RAND_HI_RES: C2RustUnnamed_1 = 1;
pub const KF_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const OBJ_MOD_MAX: C2RustUnnamed_2 = 16;
pub const OBJ_MOD_MOVES: C2RustUnnamed_2 = 15;
pub const OBJ_MOD_DAM_RED: C2RustUnnamed_2 = 14;
pub const OBJ_MOD_LIGHT: C2RustUnnamed_2 = 13;
pub const OBJ_MOD_MIGHT: C2RustUnnamed_2 = 12;
pub const OBJ_MOD_SHOTS: C2RustUnnamed_2 = 11;
pub const OBJ_MOD_BLOWS: C2RustUnnamed_2 = 10;
pub const OBJ_MOD_SPEED: C2RustUnnamed_2 = 9;
pub const OBJ_MOD_TUNNEL: C2RustUnnamed_2 = 8;
pub const OBJ_MOD_INFRA: C2RustUnnamed_2 = 7;
pub const OBJ_MOD_SEARCH: C2RustUnnamed_2 = 6;
pub const OBJ_MOD_STEALTH: C2RustUnnamed_2 = 5;
pub const OBJ_MOD_CON: C2RustUnnamed_2 = 4;
pub const OBJ_MOD_DEX: C2RustUnnamed_2 = 3;
pub const OBJ_MOD_WIS: C2RustUnnamed_2 = 2;
pub const OBJ_MOD_INT: C2RustUnnamed_2 = 1;
pub const OBJ_MOD_STR: C2RustUnnamed_2 = 0;
pub type obj_property_type = ::core::ffi::c_uint;
pub const OBJ_PROPERTY_MAX: obj_property_type = 8;
pub const OBJ_PROPERTY_IMM: obj_property_type = 7;
pub const OBJ_PROPERTY_VULN: obj_property_type = 6;
pub const OBJ_PROPERTY_RESIST: obj_property_type = 5;
pub const OBJ_PROPERTY_IGNORE: obj_property_type = 4;
pub const OBJ_PROPERTY_FLAG: obj_property_type = 3;
pub const OBJ_PROPERTY_MOD: obj_property_type = 2;
pub const OBJ_PROPERTY_STAT: obj_property_type = 1;
pub const OBJ_PROPERTY_NONE: obj_property_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_property {
    pub next: *mut obj_property,
    pub type_0: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub id_type: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub mult: ::core::ffi::c_int,
    pub type_mult: [::core::ffi::c_int; 36],
    pub name: *mut ::core::ffi::c_char,
    pub adjective: *mut ::core::ffi::c_char,
    pub neg_adj: *mut ::core::ffi::c_char,
    pub msg: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const ELEM_MAX: C2RustUnnamed_3 = 25;
pub const ELEM_ARROW: C2RustUnnamed_3 = 24;
pub const ELEM_HOLY_ORB: C2RustUnnamed_3 = 23;
pub const ELEM_MANA: C2RustUnnamed_3 = 22;
pub const ELEM_MISSILE: C2RustUnnamed_3 = 21;
pub const ELEM_METEOR: C2RustUnnamed_3 = 20;
pub const ELEM_PLASMA: C2RustUnnamed_3 = 19;
pub const ELEM_TIME: C2RustUnnamed_3 = 18;
pub const ELEM_FORCE: C2RustUnnamed_3 = 17;
pub const ELEM_INERTIA: C2RustUnnamed_3 = 16;
pub const ELEM_GRAVITY: C2RustUnnamed_3 = 15;
pub const ELEM_ICE: C2RustUnnamed_3 = 14;
pub const ELEM_WATER: C2RustUnnamed_3 = 13;
pub const ELEM_DISEN: C2RustUnnamed_3 = 12;
pub const ELEM_CHAOS: C2RustUnnamed_3 = 11;
pub const ELEM_NETHER: C2RustUnnamed_3 = 10;
pub const ELEM_NEXUS: C2RustUnnamed_3 = 9;
pub const ELEM_SHARD: C2RustUnnamed_3 = 8;
pub const ELEM_SOUND: C2RustUnnamed_3 = 7;
pub const ELEM_DARK: C2RustUnnamed_3 = 6;
pub const ELEM_LIGHT: C2RustUnnamed_3 = 5;
pub const ELEM_POIS: C2RustUnnamed_3 = 4;
pub const ELEM_COLD: C2RustUnnamed_3 = 3;
pub const ELEM_FIRE: C2RustUnnamed_3 = 2;
pub const ELEM_ELEC: C2RustUnnamed_3 = 1;
pub const ELEM_ACID: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slay {
    pub code: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub base: *mut ::core::ffi::c_char,
    pub melee_verb: *mut ::core::ffi::c_char,
    pub range_verb: *mut ::core::ffi::c_char,
    pub race_flag: ::core::ffi::c_int,
    pub multiplier: ::core::ffi::c_int,
    pub o_multiplier: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub next: *mut slay,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curse {
    pub next: *mut curse,
    pub name: *mut ::core::ffi::c_char,
    pub poss: *mut bool,
    pub obj: *mut object,
    pub conflict: *mut ::core::ffi::c_char,
    pub conflict_flags: [bitflag; 5],
    pub desc: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const STAT_MAX: C2RustUnnamed_4 = 5;
pub const STAT_CON: C2RustUnnamed_4 = 4;
pub const STAT_DEX: C2RustUnnamed_4 = 3;
pub const STAT_WIS: C2RustUnnamed_4 = 2;
pub const STAT_INT: C2RustUnnamed_4 = 1;
pub const STAT_STR: C2RustUnnamed_4 = 0;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FLAG_WIDTH: usize = (::core::mem::size_of::<bitflag>() as usize).wrapping_mul(8 as usize);
pub const FLAG_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLAG_END: ::core::ffi::c_int = FLAG_START - 1 as ::core::ffi::c_int;
unsafe extern "C" fn borg_object_value_known(mut item: *mut borg_item) -> int32_t {
    let mut value: int32_t = 0;
    let mut k_ptr: *mut object_kind = k_info.offset((*item).kind as isize) as *mut object_kind;
    if (*k_ptr).cost == 0 {
        return 0 as int32_t;
    }
    value = (*k_ptr).cost as int32_t;
    if (*item).art_idx != 0 {
        let mut a_ptr: *mut artifact = a_info.offset((*item).art_idx as isize) as *mut artifact;
        if (*a_ptr).cost == 0 {
            return 0 as int32_t;
        }
        value = (*a_ptr).cost as int32_t;
    }
    if (*item).ego_idx != 0 {
        let mut e_ptr: *mut ego_item = e_info.offset((*item).ego_idx as isize) as *mut ego_item;
        if (*e_ptr).cost == 0 {
            return 0 as int32_t;
        }
        value = (value as ::core::ffi::c_int + (*e_ptr).cost) as int32_t;
    }
    match (*item).tval as ::core::ffi::c_int {
        23 | 22 => {
            value = (value as ::core::ffi::c_int
                + (value / 20 as int32_t * (*item).pval as int32_t) as ::core::ffi::c_int)
                as int32_t;
        }
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20
        | 21 => {
            if ((*item).pval as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if !((*item).pval == 0) {
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_STR as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_INT as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_WIS as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_DEX as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                if flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    STAT_CON as ::core::ffi::c_int,
                ) {
                    value = (value as ::core::ffi::c_long
                        + (*item).pval as ::core::ffi::c_long * 200 as ::core::ffi::c_long)
                        as int32_t;
                }
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_STEALTH as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SEARCH as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 50 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_TUNNEL as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 50 as ::core::ffi::c_long) as int32_t;
                if (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize] != 0 {
                    if (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > MAX_BLOWS
                    {
                        value = (value as ::core::ffi::c_long
                            + MAX_BLOWS as ::core::ffi::c_long * 2000 as ::core::ffi::c_long)
                            as int32_t;
                    } else {
                        value = (value as ::core::ffi::c_long
                            + (*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                * 2000 as ::core::ffi::c_long)
                            as int32_t;
                    }
                }
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SHOTS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 2000 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 30000 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_MIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_MOVES as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
                value = (value as ::core::ffi::c_long
                    + (*item).modifiers[OBJ_MOD_DAM_RED as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_long
                        * 200 as ::core::ffi::c_long) as int32_t;
            }
        }
        _ => {}
    }
    let mut current_block_69: u64;
    match (*item).tval as ::core::ffi::c_int {
        21 => {
            if (*item).sval as ::core::ffi::c_int == sv_ring_dog {
                return 0 as int32_t;
            }
            current_block_69 = 790185930182612747;
        }
        20 => {
            current_block_69 = 790185930182612747;
        }
        10 | 11 | 15 | 13 | 12 | 14 | 16 | 17 | 18 => {
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && (*item).to_h as ::core::ffi::c_int > -(5 as ::core::ffi::c_int)
            {
                value = (value as ::core::ffi::c_long
                    + ((*item).to_d as ::core::ffi::c_int + (*item).to_a as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((*item).to_h as ::core::ffi::c_int
                        + (*item).to_d as ::core::ffi::c_int
                        + (*item).to_a as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        5 | 6 | 7 | 9 | 8 => {
            if ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int
                    + (*item).to_d as ::core::ffi::c_int
                    + (*item).to_a as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long) as int32_t;
            if (*item).dd as ::core::ffi::c_int > (*k_ptr).dd
                && (*item).ds as ::core::ffi::c_int == (*k_ptr).ds
            {
                value = (value as ::core::ffi::c_long
                    + (((*item).dd as ::core::ffi::c_int - (*k_ptr).dd)
                        * (*item).ds as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 200 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        2 | 3 | 4 => {
            if ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int + (*item).to_d as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * 5 as ::core::ffi::c_long) as int32_t;
            if (*item).dd as ::core::ffi::c_int > (*k_ptr).dd
                && (*item).ds as ::core::ffi::c_int == (*k_ptr).ds
            {
                value = (value as ::core::ffi::c_long
                    + (((*item).dd as ::core::ffi::c_int - (*k_ptr).dd)
                        * (*item).ds as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 5 as ::core::ffi::c_long) as int32_t;
            }
            current_block_69 = 5028470053297453708;
        }
        _ => {
            current_block_69 = 5028470053297453708;
        }
    }
    match current_block_69 {
        790185930182612747 => {
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return 0 as int32_t;
            }
            value = (value as ::core::ffi::c_long
                + ((*item).to_h as ::core::ffi::c_int
                    + (*item).to_d as ::core::ffi::c_int
                    + (*item).to_a as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long) as int32_t;
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn borg_object_value_guess(mut item: *mut borg_item) -> int32_t {
    let mut value: int32_t = 0;
    match (*item).tval as ::core::ffi::c_int {
        28 => {
            value = 5 as int32_t;
        }
        26 => {
            value = 20 as int32_t;
        }
        25 => {
            value = 20 as int32_t;
        }
        22 => {
            value = 70 as int32_t;
        }
        23 => {
            value = 50 as int32_t;
        }
        24 => {
            value = 90 as int32_t;
        }
        21 | 20 => {
            value = 45 as int32_t;
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as ::core::ffi::c_int as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < OBJ_MOD_MAX as ::core::ffi::c_int && value != 0 {
                if ((*item).modifiers[i as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    value = 0 as int32_t;
                }
                i += 1;
            }
        }
        _ => {
            value = 20 as int32_t;
            if ((*item).to_a as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as ::core::ffi::c_int as int32_t;
            }
            if ((*item).to_h as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
            if ((*item).to_d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                value = 0 as int32_t;
            }
        }
    }
    return value;
}
unsafe extern "C" fn borg_set_slays(mut item: *mut borg_item, mut o: *const object) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).slay_max as ::core::ffi::c_int {
        if *(*o).slays.offset(i as isize) {
            (*item).slays[(*slays.offset(i as isize)).race_flag as usize] =
                (*slays.offset(i as isize)).multiplier;
        }
        i += 1;
    }
}
unsafe extern "C" fn borg_set_curses(mut item: *mut borg_item, mut o: *const object) {
    let mut i: ::core::ffi::c_int = 0;
    let mut item_curses: *mut bool = &raw mut (*item).curses as *mut bool;
    (*item).uncursable = false_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).curse_max as ::core::ffi::c_int {
        let mut c: *mut curse = curses.offset(i as isize) as *mut curse;
        if (*(*o).curses.offset(i as isize)).power > 0 as ::core::ffi::c_int {
            (*item).cursed = true_0 != 0;
            if (*(*o).curses.offset(i as isize)).power < 100 as ::core::ffi::c_int {
                (*item).uncursable = true_0 != 0;
            }
            if strcmp(
                (*c).name,
                b"vulnerability\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_VULNERABILITY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"teleportation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_TELEPORTATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"dullness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DULLNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"sickliness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SICKLINESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"enveloping\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ENVELOPING as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"irritation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_IRRITATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"weakness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_WEAKNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"clumsiness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_CLUMSINESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"slowness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SLOWNESS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"annoyance\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ANNOYANCE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"poison\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_POISON as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"siren\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_SIREN as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"hallucination\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_HALLUCINATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"paralysis\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_PARALYSIS as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"dragon summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DRAGON_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"demon summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_DEMON_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"undead summon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_UNDEAD_SUMMON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"impair mana recovery\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_IMPAIR_MANA_RECOVERY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"impair hitpoint recovery\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_IMPAIR_HITPOINT_RECOVERY as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"cowardice\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_COWARDICE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"stone\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_STONE as ::core::ffi::c_int as isize) = true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"anti-teleportation\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_ANTI_TELEPORTATION as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"treacherous weapon\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_TREACHEROUS_WEAPON as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"burning up\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_BURNING_UP as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"chilled to the bone\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses
                    .offset(BORG_CURSE_CHILLED_TO_THE_BONE as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"steelskin\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_STEELSKIN as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else if strcmp(
                (*c).name,
                b"air swing\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                *item_curses.offset(BORG_CURSE_AIR_SWING as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            } else {
                *item_curses.offset(BORG_CURSE_UNKNOWN as ::core::ffi::c_int as isize) =
                    true_0 != 0;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn borg_item_has_negative_flag(mut item: *const borg_item) -> bool {
    let mut i: size_t = 0;
    i = flag_next(
        &raw const (*item).flags as *const bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add(FLAG_WIDTH)
            .wrapping_sub(1 as size_t)
            .wrapping_div(FLAG_WIDTH),
        1 as ::core::ffi::c_int,
    ) as size_t;
    while i != FLAG_END as size_t {
        let mut flag: *mut obj_property = lookup_obj_property(
            OBJ_PROPERTY_FLAG as ::core::ffi::c_int,
            i as ::core::ffi::c_int,
        );
        if !flag.is_null() && (*flag).power < 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        i = flag_next(
            &raw const (*item).flags as *const bitflag,
            (OF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(FLAG_WIDTH)
                .wrapping_sub(1 as size_t)
                .wrapping_div(FLAG_WIDTH),
            i.wrapping_add(1 as size_t) as ::core::ffi::c_int,
        ) as size_t;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_analyze(
    mut item: *mut borg_item,
    mut real_item: *const object,
    mut desc: *mut ::core::ffi::c_char,
    mut in_store: bool,
) {
    let mut scan: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut o: *const object = ::core::ptr::null::<object>();
    memset(
        item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_item>() as size_t,
    );
    if (*real_item).kind.is_null() || (*real_item).number == 0 {
        return;
    }
    (*item).ident = object_fully_known(real_item);
    if (*item).ident {
        o = real_item;
    } else if (*real_item).known.is_null() {
        o = real_item;
    } else {
        o = (*real_item).known;
    }
    (*item).needs_ident = !object_runes_known(real_item);
    object_flags_known(real_item, &raw mut (*item).flags as *mut bitflag);
    my_strcpy(
        &raw mut (*item).desc as *mut ::core::ffi::c_char,
        desc,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
    );
    scan = &raw mut (*item).desc as *mut ::core::ffi::c_char;
    while *scan as ::core::ffi::c_int != 0 && *scan as ::core::ffi::c_int != '{' as i32 {
        scan = scan.offset(1);
    }
    (*item).note = scan;
    (*item).tval = (*real_item).tval;
    (*item).sval = (*real_item).sval;
    (*item).iqty = (*real_item).number;
    (*item).weight = object_weight_one(real_item);
    (*item).timeout = (*real_item).timeout;
    (*item).level = (*(*real_item).kind).level as uint8_t;
    (*item).aware = in_store as ::core::ffi::c_int != 0
        || object_flavor_is_aware(real_item) as ::core::ffi::c_int != 0;
    (*item).ac = (*o).ac;
    (*item).dd = (*o).dd;
    (*item).ds = (*o).ds;
    (*item).to_h = (*o).to_h;
    (*item).to_d = (*o).to_d;
    (*item).to_a = (*o).to_a;
    i = 0 as ::core::ffi::c_int;
    while i < OBJ_MOD_MAX as ::core::ffi::c_int {
        (*item).modifiers[i as usize] = (*o).modifiers[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ELEM_MAX as ::core::ffi::c_int {
        (*item).el_info[i as usize].res_level = (*o).el_info[i as usize].res_level;
        (*item).el_info[i as usize].flags = (*o).el_info[i as usize].flags;
        i += 1;
    }
    if !(*o).curses.is_null() {
        borg_set_curses(item, o);
    }
    if borg_item_has_negative_flag(item) {
        (*item).cursed = true_0 != 0;
    }
    if !(*o).slays.is_null() {
        borg_set_slays(item, o);
    }
    if !(*o).brands.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).brand_max as ::core::ffi::c_int {
            (*item).brands[i as usize] = *(*o).brands.offset(i as isize);
            i += 1;
        }
    }
    if !(*o).activation.is_null() {
        if (*(*o).activation).index == act_bizarre {
            (*item).one_ring = true_0 != 0;
        }
        (*item).activ_idx = (*(*o).activation).index;
    } else {
        if (*item).tval as ::core::ffi::c_int == TV_RING as ::core::ffi::c_int {
            if (*item).sval as ::core::ffi::c_int == sv_ring_flames {
                (*item).activ_idx = act_ring_flames;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_acid {
                (*item).activ_idx = act_ring_acid;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_ice {
                (*item).activ_idx = act_ring_ice;
            }
            if (*item).sval as ::core::ffi::c_int == sv_ring_lightning {
                (*item).activ_idx = act_ring_lightning;
            }
        }
        if (*item).tval as ::core::ffi::c_int == TV_DRAG_ARMOR as ::core::ffi::c_int {
            if (*item).sval as ::core::ffi::c_int == sv_dragon_blue {
                (*item).activ_idx = act_dragon_blue;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_red {
                (*item).activ_idx = act_dragon_red;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_green {
                (*item).activ_idx = act_dragon_green;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_multihued {
                (*item).activ_idx = act_dragon_multihued;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_shining {
                (*item).activ_idx = act_dragon_shining;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_law {
                (*item).activ_idx = act_dragon_law;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_gold {
                (*item).activ_idx = act_dragon_gold;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_chaos {
                (*item).activ_idx = act_dragon_chaos;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_balance {
                (*item).activ_idx = act_dragon_balance;
            }
            if (*item).sval as ::core::ffi::c_int == sv_dragon_power {
                (*item).activ_idx = act_dragon_power;
            }
        }
    }
    if (*item).ident {
        (*item).pval = (*o).pval;
    }
    if (*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int {
        if (*item).iqty as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (*item).pval = (if (*real_item).timeout as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as int16_t;
        } else {
            let mut power: ::core::ffi::c_int = 0;
            let mut time_base: ::core::ffi::c_int =
                randcalc((*(*real_item).kind).time, 0 as ::core::ffi::c_int, MINIMISE);
            if time_base == 0 {
                time_base = 1 as ::core::ffi::c_int;
            }
            power = ((*real_item).timeout as ::core::ffi::c_int
                + (time_base - 1 as ::core::ffi::c_int))
                / time_base;
            (*item).pval = (if power < (*item).iqty as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as int16_t;
        }
    } else if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
    {
        (*item).pval = 1 as int16_t;
        if (*item).ident {
            (*item).pval = (*real_item).pval;
        }
        if !(*item).aware && (*o).pval == 0 {
            (*item).pval = 0 as int16_t;
        }
    }
    if (*item).aware {
        (*item).kind = (*(*o).kind).kidx;
    }
    if !(*o).artifact.is_null() {
        (*item).art_idx = (*(*o).artifact).aidx as uint8_t;
    }
    if !(*o).ego.is_null() {
        (*item).ego_idx = (*(*o).ego).eidx as uint8_t;
    }
    if (*item).ident {
        (*item).value = borg_object_value_known(item);
    } else if (*item).aware {
        (*item).value = (*(*o).kind).cost as int32_t;
    } else {
        (*item).value = borg_object_value_guess(item);
    }
    if (*item).cursed as ::core::ffi::c_int != 0 && !(*item).one_ring {
        (*item).value = 0 as int32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_obj_has_effect(
    mut kind: uint32_t,
    mut index: ::core::ffi::c_int,
    mut subtype: ::core::ffi::c_int,
) -> bool {
    let mut ke: *mut effect = (*k_info.offset(kind as isize)).effect;
    while !ke.is_null() {
        if (*ke).index as ::core::ffi::c_int == index
            && ((*ke).subtype == subtype || subtype == -(1 as ::core::ffi::c_int))
        {
            return true_0 != 0;
        }
        ke = (*ke).next;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_ego_has_random_power(mut e_ptr: *mut ego_item) -> bool {
    if flag_has(
        &raw mut (*e_ptr).kind_flags as *mut bitflag,
        (KF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        KF_RAND_POWER as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_SUSTAIN as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_BASE_RES as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_HI_RES as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*e_ptr).kind_flags as *mut bitflag,
            (KF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            KF_RAND_RES_POWER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
pub const MAX_BLOWS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
