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
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    static mut k_info: *mut object_kind;
    static mut e_info: *mut ego_item;
    static mut player: *mut player;
    fn player_flags(p: *mut player, f: *mut bitflag);
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    fn obj_kind_can_browse(kind: *const object_kind) -> bool;
    fn borg_calc_blows(item: *mut borg_item) -> ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut act_glyph: ::core::ffi::c_int;
    fn borg_ego_has_random_power(e_ptr: *mut ego_item) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    static mut sv_food_ration: ::core::ffi::c_int;
    static mut sv_food_slime_mold: ::core::ffi::c_int;
    static mut sv_mush_restoring: ::core::ffi::c_int;
    static mut sv_mush_purging: ::core::ffi::c_int;
    static mut sv_light_lantern: ::core::ffi::c_int;
    static mut sv_light_torch: ::core::ffi::c_int;
    static mut sv_potion_cure_critical: ::core::ffi::c_int;
    static mut sv_potion_cure_serious: ::core::ffi::c_int;
    static mut sv_potion_healing: ::core::ffi::c_int;
    static mut sv_potion_star_healing: ::core::ffi::c_int;
    static mut sv_potion_life: ::core::ffi::c_int;
    static mut sv_potion_restore_mana: ::core::ffi::c_int;
    static mut sv_potion_resist_heat: ::core::ffi::c_int;
    static mut sv_potion_resist_cold: ::core::ffi::c_int;
    static mut sv_potion_restore_life: ::core::ffi::c_int;
    static mut sv_potion_speed: ::core::ffi::c_int;
    static mut sv_potion_berserk: ::core::ffi::c_int;
    static mut sv_scroll_identify: ::core::ffi::c_int;
    static mut sv_scroll_phase_door: ::core::ffi::c_int;
    static mut sv_scroll_teleport: ::core::ffi::c_int;
    static mut sv_scroll_word_of_recall: ::core::ffi::c_int;
    static mut sv_scroll_enchant_armor: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_hit: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_dam: ::core::ffi::c_int;
    static mut sv_scroll_protection_from_evil: ::core::ffi::c_int;
    static mut sv_scroll_rune_of_protection: ::core::ffi::c_int;
    static mut sv_scroll_teleport_level: ::core::ffi::c_int;
    static mut sv_scroll_recharging: ::core::ffi::c_int;
    static mut sv_scroll_mass_banishment: ::core::ffi::c_int;
    static mut sv_rod_recall: ::core::ffi::c_int;
    static mut sv_staff_teleportation: ::core::ffi::c_int;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    fn borg_spell_legal_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    static mut borg_shops: *mut borg_shop;
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
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const STAT_MAX: C2RustUnnamed_4 = 5;
pub const STAT_CON: C2RustUnnamed_4 = 4;
pub const STAT_DEX: C2RustUnnamed_4 = 3;
pub const STAT_WIS: C2RustUnnamed_4 = 2;
pub const STAT_INT: C2RustUnnamed_4 = 1;
pub const STAT_STR: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const PF_MAX: C2RustUnnamed_5 = 18;
pub const PF_COMBAT_REGEN: C2RustUnnamed_5 = 17;
pub const PF_EVIL: C2RustUnnamed_5 = 16;
pub const PF_SHIELD_BASH: C2RustUnnamed_5 = 15;
pub const PF_STEAL: C2RustUnnamed_5 = 14;
pub const PF_ROCK: C2RustUnnamed_5 = 13;
pub const PF_UNLIGHT: C2RustUnnamed_5 = 12;
pub const PF_CHARM: C2RustUnnamed_5 = 11;
pub const PF_NO_MANA: C2RustUnnamed_5 = 10;
pub const PF_SEE_ORE: C2RustUnnamed_5 = 9;
pub const PF_KNOW_ZAPPER: C2RustUnnamed_5 = 8;
pub const PF_KNOW_MUSHROOM: C2RustUnnamed_5 = 7;
pub const PF_CHOOSE_SPELLS: C2RustUnnamed_5 = 6;
pub const PF_BEAM: C2RustUnnamed_5 = 5;
pub const PF_ZERO_FAIL: C2RustUnnamed_5 = 4;
pub const PF_BLESS_WEAPON: C2RustUnnamed_5 = 3;
pub const PF_BRAVERY_30: C2RustUnnamed_5 = 2;
pub const PF_FAST_SHOT: C2RustUnnamed_5 = 1;
pub const PF_NONE: C2RustUnnamed_5 = 0;
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
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const RF_MAX: C2RustUnnamed_6 = 85;
pub const RF_NO_SLOW: C2RustUnnamed_6 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_6 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_6 = 82;
pub const RF_NO_CONF: C2RustUnnamed_6 = 81;
pub const RF_NO_STUN: C2RustUnnamed_6 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_6 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_6 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_6 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_6 = 76;
pub const RF_IM_WATER: C2RustUnnamed_6 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_6 = 74;
pub const RF_IM_POIS: C2RustUnnamed_6 = 73;
pub const RF_IM_COLD: C2RustUnnamed_6 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_6 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_6 = 70;
pub const RF_IM_ACID: C2RustUnnamed_6 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_6 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_6 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_6 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_6 = 65;
pub const RF_METAL: C2RustUnnamed_6 = 64;
pub const RF_NONLIVING: C2RustUnnamed_6 = 63;
pub const RF_UNDEAD: C2RustUnnamed_6 = 62;
pub const RF_EVIL: C2RustUnnamed_6 = 61;
pub const RF_ANIMAL: C2RustUnnamed_6 = 60;
pub const RF_DEMON: C2RustUnnamed_6 = 59;
pub const RF_DRAGON: C2RustUnnamed_6 = 58;
pub const RF_GIANT: C2RustUnnamed_6 = 57;
pub const RF_TROLL: C2RustUnnamed_6 = 56;
pub const RF_ORC: C2RustUnnamed_6 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_6 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_6 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_6 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_6 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_6 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_6 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_6 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_6 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_6 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_6 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_6 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_6 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_6 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_6 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_6 = 40;
pub const RF_DROP_20: C2RustUnnamed_6 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_6 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_6 = 37;
pub const RF_DROP_4: C2RustUnnamed_6 = 36;
pub const RF_DROP_3: C2RustUnnamed_6 = 35;
pub const RF_DROP_2: C2RustUnnamed_6 = 34;
pub const RF_DROP_1: C2RustUnnamed_6 = 33;
pub const RF_DROP_60: C2RustUnnamed_6 = 32;
pub const RF_DROP_40: C2RustUnnamed_6 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_6 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_6 = 29;
pub const RF_POWERFUL: C2RustUnnamed_6 = 28;
pub const RF_SPIRIT: C2RustUnnamed_6 = 27;
pub const RF_SMART: C2RustUnnamed_6 = 26;
pub const RF_STUPID: C2RustUnnamed_6 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_6 = 24;
pub const RF_RAND_50: C2RustUnnamed_6 = 23;
pub const RF_RAND_25: C2RustUnnamed_6 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_6 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_6 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_6 = 19;
pub const RF_REGENERATE: C2RustUnnamed_6 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_6 = 17;
pub const RF_UNAWARE: C2RustUnnamed_6 = 16;
pub const RF_SEASONAL: C2RustUnnamed_6 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_6 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_6 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_6 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_6 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_6 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_6 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_6 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_6 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_6 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_6 = 5;
pub const RF_FEMALE: C2RustUnnamed_6 = 4;
pub const RF_MALE: C2RustUnnamed_6 = 3;
pub const RF_QUESTOR: C2RustUnnamed_6 = 2;
pub const RF_UNIQUE: C2RustUnnamed_6 = 1;
pub const RF_NONE: C2RustUnnamed_6 = 0;
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
pub const BI_SCON: C2RustUnnamed_15 = 24;
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
    pub temp: C2RustUnnamed_14,
    pub status: C2RustUnnamed_13,
    pub time: C2RustUnnamed_12,
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
    pub goal: C2RustUnnamed_11,
    pub init_save: C2RustUnnamed_10,
    pub mon: C2RustUnnamed_9,
    pub id: C2RustUnnamed_8,
    pub near: C2RustUnnamed_7,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub struct C2RustUnnamed_12 {
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
pub struct C2RustUnnamed_13 {
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
pub struct C2RustUnnamed_14 {
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
pub const BI_SDEX: C2RustUnnamed_15 = 23;
pub const BI_SWIS: C2RustUnnamed_15 = 22;
pub const BI_SINT: C2RustUnnamed_15 = 21;
pub const BI_SSTR: C2RustUnnamed_15 = 20;
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
pub const BI_AMMO_TVAL: C2RustUnnamed_15 = 152;
pub const BI_CLEVEL: C2RustUnnamed_15 = 35;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_shop {
    pub ware: *mut borg_item,
}
pub const BI_TODAM: C2RustUnnamed_15 = 135;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_15 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_15 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_15 = 263;
pub const BI_EMPTY: C2RustUnnamed_15 = 262;
pub const BI_CARRY: C2RustUnnamed_15 = 261;
pub const BI_WEIGHT: C2RustUnnamed_15 = 260;
pub const BI_DINV: C2RustUnnamed_15 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_15 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_15 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_15 = 256;
pub const BI_ADIGGER: C2RustUnnamed_15 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_15 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_15 = 253;
pub const BI_AROD2: C2RustUnnamed_15 = 252;
pub const BI_AROD1: C2RustUnnamed_15 = 251;
pub const BI_ASHROOM: C2RustUnnamed_15 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_15 = 249;
pub const BI_AHWORD: C2RustUnnamed_15 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_15 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_15 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_15 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_15 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_15 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_15 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_15 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_15 = 240;
pub const BI_ABRAND: C2RustUnnamed_15 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_15 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_15 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_15 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_15 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_15 = 234;
pub const BI_ACLW: C2RustUnnamed_15 = 233;
pub const BI_ACSW: C2RustUnnamed_15 = 232;
pub const BI_ACCW: C2RustUnnamed_15 = 231;
pub const BI_AGLYPH: C2RustUnnamed_15 = 230;
pub const BI_APFE: C2RustUnnamed_15 = 229;
pub const BI_ALITE: C2RustUnnamed_15 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_15 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_15 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_15 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_15 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_15 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_15 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_15 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_15 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_15 = 219;
pub const BI_ASPEED: C2RustUnnamed_15 = 218;
pub const BI_AID: C2RustUnnamed_15 = 217;
pub const BI_ALIFE: C2RustUnnamed_15 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_15 = 215;
pub const BI_AHEAL: C2RustUnnamed_15 = 214;
pub const BI_AFUEL: C2RustUnnamed_15 = 213;
pub const BI_AESCAPE: C2RustUnnamed_15 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_15 = 211;
pub const BI_APHASE: C2RustUnnamed_15 = 210;
pub const BI_WB_POIS: C2RustUnnamed_15 = 209;
pub const BI_WB_COLD: C2RustUnnamed_15 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_15 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_15 = 206;
pub const BI_WB_ACID: C2RustUnnamed_15 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_15 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_15 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_15 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_15 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_15 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_15 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_15 = 198;
pub const BI_WS_ORC: C2RustUnnamed_15 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_15 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_15 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_15 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_15 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_15 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_15 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_15 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_15 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_15 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_15 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_15 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_15 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_15 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_15 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_15 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_15 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_15 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_15 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_15 = 178;
pub const BI_CRSSICK: C2RustUnnamed_15 = 177;
pub const BI_CRSDULL: C2RustUnnamed_15 = 176;
pub const BI_CRSVULN: C2RustUnnamed_15 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_15 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_15 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_15 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_15 = 171;
pub const BI_CRSSUND: C2RustUnnamed_15 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_15 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_15 = 168;
pub const BI_CRSPARA: C2RustUnnamed_15 = 167;
pub const BI_CRSHALU: C2RustUnnamed_15 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_15 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_15 = 164;
pub const BI_CRSTELE: C2RustUnnamed_15 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_15 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_15 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_15 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_15 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_15 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_15 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_15 = 156;
pub const BI_AMISSILES: C2RustUnnamed_15 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_15 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_15 = 153;
pub const BI_AMMO_COUNT: C2RustUnnamed_15 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_15 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_15 = 149;
pub const BI_SHOTS: C2RustUnnamed_15 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_15 = 147;
pub const BI_BLOWS: C2RustUnnamed_15 = 146;
pub const BI_BART: C2RustUnnamed_15 = 145;
pub const BI_SLING: C2RustUnnamed_15 = 144;
pub const BI_BTODAM: C2RustUnnamed_15 = 143;
pub const BI_BTOHIT: C2RustUnnamed_15 = 142;
pub const BI_BID: C2RustUnnamed_15 = 141;
pub const BI_WDS: C2RustUnnamed_15 = 140;
pub const BI_WDD: C2RustUnnamed_15 = 139;
pub const BI_WID: C2RustUnnamed_15 = 138;
pub const BI_WTODAM: C2RustUnnamed_15 = 137;
pub const BI_WTOHIT: C2RustUnnamed_15 = 136;
pub const BI_TOHIT: C2RustUnnamed_15 = 134;
pub const BI_ARMOR: C2RustUnnamed_15 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_15 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_15 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_15 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_15 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_15 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_15 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_15 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_15 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_15 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_15 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_15 = 122;
pub const BI_ISFORGET: C2RustUnnamed_15 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_15 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_15 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_15 = 118;
pub const BI_ISSTUN: C2RustUnnamed_15 = 117;
pub const BI_ISCUT: C2RustUnnamed_15 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_15 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_15 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_15 = 113;
pub const BI_ISBLIND: C2RustUnnamed_15 = 112;
pub const BI_ISGORGED: C2RustUnnamed_15 = 111;
pub const BI_ISFULL: C2RustUnnamed_15 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_15 = 109;
pub const BI_ISWEAK: C2RustUnnamed_15 = 108;
pub const BI_KING: C2RustUnnamed_15 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_15 = 106;
pub const BI_CDEPTH: C2RustUnnamed_15 = 105;
pub const BI_SFRACT: C2RustUnnamed_15 = 104;
pub const BI_SHLIFE: C2RustUnnamed_15 = 103;
pub const BI_SRDIS: C2RustUnnamed_15 = 102;
pub const BI_SRKAOS: C2RustUnnamed_15 = 101;
pub const BI_SRNTHR: C2RustUnnamed_15 = 100;
pub const BI_SRNXUS: C2RustUnnamed_15 = 99;
pub const BI_SRSHRD: C2RustUnnamed_15 = 98;
pub const BI_SRSND: C2RustUnnamed_15 = 97;
pub const BI_SRCONF: C2RustUnnamed_15 = 96;
pub const BI_SRBLIND: C2RustUnnamed_15 = 95;
pub const BI_SRDARK: C2RustUnnamed_15 = 94;
pub const BI_SRLITE: C2RustUnnamed_15 = 93;
pub const BI_SRFEAR: C2RustUnnamed_15 = 92;
pub const BI_SRPOIS: C2RustUnnamed_15 = 91;
pub const BI_SRACID: C2RustUnnamed_15 = 90;
pub const BI_SRELEC: C2RustUnnamed_15 = 89;
pub const BI_SRCOLD: C2RustUnnamed_15 = 88;
pub const BI_SRFIRE: C2RustUnnamed_15 = 87;
pub const BI_FRACT: C2RustUnnamed_15 = 86;
pub const BI_HLIFE: C2RustUnnamed_15 = 85;
pub const BI_RDIS: C2RustUnnamed_15 = 84;
pub const BI_RKAOS: C2RustUnnamed_15 = 83;
pub const BI_RNTHR: C2RustUnnamed_15 = 82;
pub const BI_RNXUS: C2RustUnnamed_15 = 81;
pub const BI_RSHRD: C2RustUnnamed_15 = 80;
pub const BI_RSND: C2RustUnnamed_15 = 79;
pub const BI_RCONF: C2RustUnnamed_15 = 78;
pub const BI_RBLIND: C2RustUnnamed_15 = 77;
pub const BI_RDARK: C2RustUnnamed_15 = 76;
pub const BI_RLITE: C2RustUnnamed_15 = 75;
pub const BI_RFEAR: C2RustUnnamed_15 = 74;
pub const BI_RPOIS: C2RustUnnamed_15 = 73;
pub const BI_RACID: C2RustUnnamed_15 = 72;
pub const BI_RELEC: C2RustUnnamed_15 = 71;
pub const BI_RCOLD: C2RustUnnamed_15 = 70;
pub const BI_RFIRE: C2RustUnnamed_15 = 69;
pub const BI_IPOIS: C2RustUnnamed_15 = 68;
pub const BI_IELEC: C2RustUnnamed_15 = 67;
pub const BI_ICOLD: C2RustUnnamed_15 = 66;
pub const BI_IACID: C2RustUnnamed_15 = 65;
pub const BI_IFIRE: C2RustUnnamed_15 = 64;
pub const BI_DIG: C2RustUnnamed_15 = 63;
pub const BI_THT: C2RustUnnamed_15 = 62;
pub const BI_THB: C2RustUnnamed_15 = 61;
pub const BI_THN: C2RustUnnamed_15 = 60;
pub const BI_SRCH: C2RustUnnamed_15 = 59;
pub const BI_STL: C2RustUnnamed_15 = 58;
pub const BI_SAV: C2RustUnnamed_15 = 57;
pub const BI_DEV: C2RustUnnamed_15 = 56;
pub const BI_DISM: C2RustUnnamed_15 = 55;
pub const BI_DISP: C2RustUnnamed_15 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_15 = 53;
pub const BI_INFRA: C2RustUnnamed_15 = 52;
pub const BI_SINV: C2RustUnnamed_15 = 51;
pub const BI_REG: C2RustUnnamed_15 = 50;
pub const BI_FEATH: C2RustUnnamed_15 = 49;
pub const BI_SDIG: C2RustUnnamed_15 = 48;
pub const BI_DAM_RED: C2RustUnnamed_15 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_15 = 46;
pub const BI_GOLD: C2RustUnnamed_15 = 45;
pub const BI_SPEED: C2RustUnnamed_15 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_15 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_15 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_15 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_15 = 40;
pub const BI_FOOD: C2RustUnnamed_15 = 39;
pub const BI_RECALL: C2RustUnnamed_15 = 38;
pub const BI_ESP: C2RustUnnamed_15 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_15 = 36;
pub const BI_FAIL2: C2RustUnnamed_15 = 34;
pub const BI_FAIL1: C2RustUnnamed_15 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_15 = 32;
pub const BI_MAXSP: C2RustUnnamed_15 = 31;
pub const BI_CURSP: C2RustUnnamed_15 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_15 = 29;
pub const BI_MAXHP: C2RustUnnamed_15 = 28;
pub const BI_CURHP: C2RustUnnamed_15 = 27;
pub const BI_LIGHT: C2RustUnnamed_15 = 26;
pub const BI_CLASS: C2RustUnnamed_15 = 25;
pub const BI_CON_INDEX: C2RustUnnamed_15 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_15 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_15 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_15 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_15 = 15;
pub const BI_CCON: C2RustUnnamed_15 = 14;
pub const BI_CDEX: C2RustUnnamed_15 = 13;
pub const BI_CWIS: C2RustUnnamed_15 = 12;
pub const BI_CINT: C2RustUnnamed_15 = 11;
pub const BI_CSTR: C2RustUnnamed_15 = 10;
pub const BI_ACON: C2RustUnnamed_15 = 9;
pub const BI_ADEX: C2RustUnnamed_15 = 8;
pub const BI_AWIS: C2RustUnnamed_15 = 7;
pub const BI_AINT: C2RustUnnamed_15 = 6;
pub const BI_ASTR: C2RustUnnamed_15 = 5;
pub const BI_CON: C2RustUnnamed_15 = 4;
pub const BI_DEX: C2RustUnnamed_15 = 3;
pub const BI_WIS: C2RustUnnamed_15 = 2;
pub const BI_INT: C2RustUnnamed_15 = 1;
pub const BI_STR: C2RustUnnamed_15 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut num_food: int16_t = 0;
#[no_mangle]
pub static mut num_fuel: int16_t = 0;
#[no_mangle]
pub static mut num_mold: int16_t = 0;
#[no_mangle]
pub static mut num_ident: int16_t = 0;
#[no_mangle]
pub static mut num_recall: int16_t = 0;
#[no_mangle]
pub static mut num_phase: int16_t = 0;
#[no_mangle]
pub static mut num_escape: int16_t = 0;
#[no_mangle]
pub static mut num_tele_staves: int16_t = 0;
#[no_mangle]
pub static mut num_teleport: int16_t = 0;
#[no_mangle]
pub static mut num_berserk: int16_t = 0;
#[no_mangle]
pub static mut num_teleport_level: int16_t = 0;
#[no_mangle]
pub static mut num_recharge: int16_t = 0;
#[no_mangle]
pub static mut num_cure_critical: int16_t = 0;
#[no_mangle]
pub static mut num_cure_serious: int16_t = 0;
#[no_mangle]
pub static mut num_pot_rheat: int16_t = 0;
#[no_mangle]
pub static mut num_pot_rcold: int16_t = 0;
#[no_mangle]
pub static mut num_missile: int16_t = 0;
#[no_mangle]
pub static mut num_book: [int16_t; 9] = [0; 9];
#[no_mangle]
pub static mut num_fix_stat: [int16_t; 5] = [0; 5];
#[no_mangle]
pub static mut home_stat_add: [int16_t; 5] = [0; 5];
#[no_mangle]
pub static mut num_fix_exp: int16_t = 0;
#[no_mangle]
pub static mut num_mana: int16_t = 0;
#[no_mangle]
pub static mut num_heal: int16_t = 0;
#[no_mangle]
pub static mut num_heal_true: int16_t = 0;
#[no_mangle]
pub static mut num_ezheal: int16_t = 0;
#[no_mangle]
pub static mut num_ezheal_true: int16_t = 0;
#[no_mangle]
pub static mut num_life: int16_t = 0;
#[no_mangle]
pub static mut num_life_true: int16_t = 0;
#[no_mangle]
pub static mut num_pfe: int16_t = 0;
#[no_mangle]
pub static mut num_glyph: int16_t = 0;
#[no_mangle]
pub static mut num_mass_genocide: int16_t = 0;
#[no_mangle]
pub static mut num_speed: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_a: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_d: int16_t = 0;
#[no_mangle]
pub static mut num_enchant_to_h: int16_t = 0;
#[no_mangle]
pub static mut num_brand_weapon: int16_t = 0;
#[no_mangle]
pub static mut num_genocide: int16_t = 0;
#[no_mangle]
pub static mut num_artifact: int16_t = 0;
#[no_mangle]
pub static mut num_ego: int16_t = 0;
#[no_mangle]
pub static mut home_slot_free: int16_t = 0;
#[no_mangle]
pub static mut home_un_id: int16_t = 0;
#[no_mangle]
pub static mut home_damage: int16_t = 0;
#[no_mangle]
pub static mut num_duplicate_items: int16_t = 0;
#[no_mangle]
pub static mut num_slow_digest: int16_t = 0;
#[no_mangle]
pub static mut num_regenerate: int16_t = 0;
#[no_mangle]
pub static mut num_telepathy: int16_t = 0;
#[no_mangle]
pub static mut num_LIGHT: int16_t = 0;
#[no_mangle]
pub static mut num_see_inv: int16_t = 0;
#[no_mangle]
pub static mut num_invisible: int16_t = 0;
#[no_mangle]
pub static mut num_ffall: int16_t = 0;
#[no_mangle]
pub static mut num_free_act: int16_t = 0;
#[no_mangle]
pub static mut num_hold_life: int16_t = 0;
#[no_mangle]
pub static mut num_immune_acid: int16_t = 0;
#[no_mangle]
pub static mut num_immune_elec: int16_t = 0;
#[no_mangle]
pub static mut num_immune_fire: int16_t = 0;
#[no_mangle]
pub static mut num_immune_cold: int16_t = 0;
#[no_mangle]
pub static mut num_resist_acid: int16_t = 0;
#[no_mangle]
pub static mut num_resist_elec: int16_t = 0;
#[no_mangle]
pub static mut num_resist_fire: int16_t = 0;
#[no_mangle]
pub static mut num_resist_cold: int16_t = 0;
#[no_mangle]
pub static mut num_resist_pois: int16_t = 0;
#[no_mangle]
pub static mut num_resist_conf: int16_t = 0;
#[no_mangle]
pub static mut num_resist_sound: int16_t = 0;
#[no_mangle]
pub static mut num_resist_LIGHT: int16_t = 0;
#[no_mangle]
pub static mut num_resist_dark: int16_t = 0;
#[no_mangle]
pub static mut num_resist_chaos: int16_t = 0;
#[no_mangle]
pub static mut num_resist_disen: int16_t = 0;
#[no_mangle]
pub static mut num_resist_shard: int16_t = 0;
#[no_mangle]
pub static mut num_resist_nexus: int16_t = 0;
#[no_mangle]
pub static mut num_resist_blind: int16_t = 0;
#[no_mangle]
pub static mut num_resist_neth: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_str: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_int: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_wis: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_dex: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_con: int16_t = 0;
#[no_mangle]
pub static mut num_sustain_all: int16_t = 0;
#[no_mangle]
pub static mut num_edged_weapon: int16_t = 0;
#[no_mangle]
pub static mut num_bad_gloves: int16_t = 0;
#[no_mangle]
pub static mut num_weapons: int16_t = 0;
#[no_mangle]
pub static mut num_bow: int16_t = 0;
#[no_mangle]
pub static mut num_rings: int16_t = 0;
#[no_mangle]
pub static mut num_neck: int16_t = 0;
#[no_mangle]
pub static mut num_armor: int16_t = 0;
#[no_mangle]
pub static mut num_cloaks: int16_t = 0;
#[no_mangle]
pub static mut num_shields: int16_t = 0;
#[no_mangle]
pub static mut num_hats: int16_t = 0;
#[no_mangle]
pub static mut num_gloves: int16_t = 0;
#[no_mangle]
pub static mut num_boots: int16_t = 0;
unsafe extern "C" fn borg_notice_home_clear(mut in_item: *mut borg_item, mut no_items: bool) {
    num_food = 0 as int16_t;
    num_fuel = 0 as int16_t;
    num_mold = 0 as int16_t;
    num_ident = 0 as int16_t;
    num_recall = 0 as int16_t;
    num_phase = 0 as int16_t;
    num_escape = 0 as int16_t;
    num_tele_staves = 0 as int16_t;
    num_teleport = 0 as int16_t;
    num_teleport_level = 0 as int16_t;
    num_recharge = 0 as int16_t;
    num_artifact = 0 as int16_t;
    num_ego = 0 as int16_t;
    num_invisible = 0 as int16_t;
    num_pfe = 0 as int16_t;
    num_glyph = 0 as int16_t;
    num_genocide = 0 as int16_t;
    num_mass_genocide = 0 as int16_t;
    num_berserk = 0 as int16_t;
    num_pot_rheat = 0 as int16_t;
    num_pot_rcold = 0 as int16_t;
    num_speed = 0 as int16_t;
    num_slow_digest = 0 as int16_t;
    num_regenerate = 0 as int16_t;
    num_telepathy = 0 as int16_t;
    num_see_inv = 0 as int16_t;
    num_ffall = 0 as int16_t;
    num_free_act = 0 as int16_t;
    num_hold_life = 0 as int16_t;
    num_immune_acid = 0 as int16_t;
    num_immune_elec = 0 as int16_t;
    num_immune_fire = 0 as int16_t;
    num_immune_cold = 0 as int16_t;
    num_resist_acid = 0 as int16_t;
    num_resist_elec = 0 as int16_t;
    num_resist_fire = 0 as int16_t;
    num_resist_cold = 0 as int16_t;
    num_resist_pois = 0 as int16_t;
    num_resist_conf = 0 as int16_t;
    num_resist_sound = 0 as int16_t;
    num_resist_LIGHT = 0 as int16_t;
    num_resist_dark = 0 as int16_t;
    num_resist_chaos = 0 as int16_t;
    num_resist_disen = 0 as int16_t;
    num_resist_shard = 0 as int16_t;
    num_resist_nexus = 0 as int16_t;
    num_resist_blind = 0 as int16_t;
    num_resist_neth = 0 as int16_t;
    num_sustain_str = 0 as int16_t;
    num_sustain_int = 0 as int16_t;
    num_sustain_wis = 0 as int16_t;
    num_sustain_dex = 0 as int16_t;
    num_sustain_con = 0 as int16_t;
    num_sustain_all = 0 as int16_t;
    home_stat_add[STAT_STR as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_INT as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_WIS as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] = 0 as int16_t;
    home_stat_add[STAT_CON as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_weapons = 0 as int16_t;
    num_bow = 0 as int16_t;
    num_rings = 0 as int16_t;
    num_neck = 0 as int16_t;
    num_armor = 0 as int16_t;
    num_cloaks = 0 as int16_t;
    num_shields = 0 as int16_t;
    num_hats = 0 as int16_t;
    num_gloves = 0 as int16_t;
    num_boots = 0 as int16_t;
    num_LIGHT = 0 as int16_t;
    num_speed = 0 as int16_t;
    num_edged_weapon = 0 as int16_t;
    num_bad_gloves = 0 as int16_t;
    num_cure_critical = 0 as int16_t;
    num_cure_serious = 0 as int16_t;
    num_fix_exp = 0 as int16_t;
    num_mana = 0 as int16_t;
    num_heal = 0 as int16_t;
    num_ezheal = 0 as int16_t;
    num_life = 0 as int16_t;
    if in_item.is_null() && !no_items {
        num_ezheal_true = 0 as int16_t;
    }
    if in_item.is_null() && !no_items {
        num_heal_true = 0 as int16_t;
    }
    if in_item.is_null() && !no_items {
        num_life_true = 0 as int16_t;
    }
    num_missile = 0 as int16_t;
    num_book[0 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[1 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[2 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[3 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[4 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[5 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[6 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[7 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_book[8 as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = 0 as int16_t;
    num_enchant_to_a = 0 as int16_t;
    num_enchant_to_d = 0 as int16_t;
    num_enchant_to_h = 0 as int16_t;
    home_slot_free = 0 as int16_t;
    home_damage = 0 as int16_t;
    home_un_id = 0 as int16_t;
    num_duplicate_items = 0 as int16_t;
}
unsafe extern "C" fn borg_notice_home_dupe(
    mut item: *mut borg_item,
    mut check_sval: bool,
    mut i: ::core::ffi::c_int,
) {
    let mut dupe_count: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut item2: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut e_ptr: *mut ego_item = e_info.offset((*item).ego_idx as isize) as *mut ego_item;
    if borg_ego_has_random_power(e_ptr) {
        return;
    }
    if (*item).needs_ident {
        return;
    }
    dupe_count = (*item).iqty as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    x = 0 as ::core::ffi::c_int;
    while x < i {
        if x < (*z_info).store_inven_max as ::core::ffi::c_int {
            item2 = (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(x as isize) as *mut borg_item;
        } else {
            item2 = borg_items.offset(
                (x - (*z_info).store_inven_max as ::core::ffi::c_int
                    + (*z_info).pack_size as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
        }
        if !((*item2).iqty == 0 || !(*item2).aware) {
            if (*item).tval as ::core::ffi::c_int == (*item2).tval as ::core::ffi::c_int
                && (if check_sval as ::core::ffi::c_int != 0 {
                    ((*item).sval as ::core::ffi::c_int == (*item2).sval as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                } else {
                    true_0
                }) != 0
                && (*item).art_idx as ::core::ffi::c_int == (*item2).art_idx as ::core::ffi::c_int
                && (*item).ego_idx as ::core::ffi::c_int == (*item2).ego_idx as ::core::ffi::c_int
            {
                dupe_count += 1;
            }
        }
        x += 1;
    }
    if (*item).tval as ::core::ffi::c_int == TV_RING as ::core::ffi::c_int && dupe_count != 0 {
        dupe_count -= 1;
    }
    num_duplicate_items = (num_duplicate_items as ::core::ffi::c_int + dupe_count) as int16_t;
}
unsafe extern "C" fn borg_notice_home_aux(mut in_item: *mut borg_item, mut no_items: bool) {
    let mut i: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut home: *mut borg_shop = borg_shops.offset(BORG_HOME as isize) as *mut borg_shop;
    let mut f: [bitflag; 5] = [0; 5];
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).store_inven_max as ::core::ffi::c_int
            + ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                - (*z_info).pack_size as ::core::ffi::c_int)
    {
        if no_items {
            break;
        }
        if in_item.is_null() {
            if i < (*z_info).store_inven_max as ::core::ffi::c_int {
                item = (*home).ware.offset(i as isize) as *mut borg_item;
            } else {
                item = borg_items.offset(
                    (i - (*z_info).store_inven_max as ::core::ffi::c_int
                        + (*z_info).pack_size as ::core::ffi::c_int) as isize,
                ) as *mut borg_item;
            }
        } else {
            item = in_item;
        }
        if (*item).iqty == 0 {
            if i < (*z_info).store_inven_max as ::core::ffi::c_int {
                home_slot_free += 1;
            }
        } else if (*item).aware {
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
                OF_SLOW_DIGEST as ::core::ffi::c_int,
            ) {
                num_slow_digest = (num_slow_digest as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_REGEN as ::core::ffi::c_int,
            ) {
                num_regenerate = (num_regenerate as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_TELEPATHY as ::core::ffi::c_int,
            ) {
                num_telepathy = (num_telepathy as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SEE_INVIS as ::core::ffi::c_int,
            ) {
                num_see_inv = (num_see_inv as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
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
                OF_FEATHER as ::core::ffi::c_int,
            ) {
                num_ffall = (num_ffall as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_FREE_ACT as ::core::ffi::c_int,
            ) {
                num_free_act = (num_free_act as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
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
                OF_HOLD_LIFE as ::core::ffi::c_int,
            ) {
                num_hold_life = (num_hold_life as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_PROT_CONF as ::core::ffi::c_int,
            ) {
                num_resist_conf = (num_resist_conf as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_PROT_BLIND as ::core::ffi::c_int,
            ) {
                num_resist_blind = (num_resist_blind as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_fire = (num_immune_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_fire = (num_resist_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_acid = (num_immune_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_acid = (num_resist_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_cold = (num_immune_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_cold = (num_resist_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                num_immune_elec = (num_immune_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
                num_resist_elec = (num_resist_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_acid = (num_resist_acid as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_elec = (num_resist_elec as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_fire = (num_resist_fire as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_cold = (num_resist_cold as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_pois = (num_resist_pois as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_sound = (num_resist_sound as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_LIGHT = (num_resist_LIGHT as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_dark = (num_resist_dark as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_chaos = (num_resist_chaos as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_disen = (num_resist_disen as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_shard = (num_resist_shard as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_nexus = (num_resist_nexus as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                num_resist_neth = (num_resist_neth as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_STR as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_INT as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_WIS as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_DEX as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_CON as ::core::ffi::c_int,
            ) {
                num_sustain_str = (num_sustain_str as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
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
                OF_SUST_STR as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_INT as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_WIS as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_DEX as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && flag_has(
                    &raw mut (*item).flags as *mut bitflag,
                    (OF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    OF_SUST_CON as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
            {
                num_sustain_all = (num_sustain_all as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int)
                    as int16_t;
            }
            if (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_STR as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_INT as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_WIS as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_DEX as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            if (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize] != 0 {
                if (*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int
                    || (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 3 as ::core::ffi::c_int
                {
                    home_stat_add[STAT_CON as ::core::ffi::c_int as usize] = (home_stat_add
                        [STAT_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + (*item).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                }
            }
            num_speed = (num_speed as ::core::ffi::c_int
                + (*item).modifiers[OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*item).iqty as ::core::ffi::c_int) as int16_t;
            if (*item).art_idx != 0 {
                num_artifact = (num_artifact as ::core::ffi::c_int
                    + (*item).iqty as ::core::ffi::c_int) as int16_t;
            }
            if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
                as ::core::ffi::c_int
                != 0
                && (*item).needs_ident as ::core::ffi::c_int != 0
            {
                num_ego =
                    (num_ego as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int) as int16_t;
            }
            if (*item).needs_ident as ::core::ffi::c_int != 0
                && i < (*z_info).store_inven_max as ::core::ffi::c_int
            {
                home_un_id += 1;
            }
            match (*item).tval as ::core::ffi::c_int {
                16 | 17 => {
                    num_armor = (num_armor as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                18 => {
                    num_armor = (num_armor as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                15 => {
                    num_cloaks = (num_cloaks as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                14 => {
                    num_shields = (num_shields as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                12 | 13 => {
                    num_hats = (num_hats as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                11 => {
                    num_gloves = (num_gloves as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    home_damage = (home_damage as ::core::ffi::c_int
                        + (*item).to_d as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                27 => {
                    if (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .sval as ::core::ffi::c_int
                        == sv_light_lantern
                    {
                        num_fuel = (num_fuel as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                19 => {
                    if (*borg_items.offset(
                        ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .sval as ::core::ffi::c_int
                        == sv_light_torch
                    {
                        num_fuel = (num_fuel as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                    if (*item).art_idx != 0 {
                        num_LIGHT = (num_LIGHT as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                10 => {
                    num_boots = (num_boots as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                9 | 8 | 7 => {
                    let mut num_blow: int16_t = 0;
                    num_weapons = (num_weapons as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    if flag_has(
                        &raw mut (*player).state.pflags as *mut bitflag,
                        (PF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        PF_BLESS_WEAPON as ::core::ffi::c_int,
                    ) {
                        if !((*item).tval as ::core::ffi::c_int == TV_HAFTED as ::core::ffi::c_int
                            || flag_has(
                                &raw mut (*item).flags as *mut bitflag,
                                (OF_MAX as ::core::ffi::c_int as size_t)
                                    .wrapping_add(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    )
                                    .wrapping_sub(1 as size_t)
                                    .wrapping_div(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    ),
                                OF_BLESSED as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int
                                != 0)
                        {
                            num_edged_weapon = (num_edged_weapon as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                    num_blow = borg_calc_blows(item) as int16_t;
                    if (*item).to_d as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                        || *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                            < 15 as ::core::ffi::c_int
                    {
                        home_damage = (home_damage as ::core::ffi::c_int
                            + num_blow as ::core::ffi::c_int
                                * ((*item).dd as ::core::ffi::c_int
                                    * (*item).ds as ::core::ffi::c_int
                                    + (*borg
                                        .trait_0
                                        .offset(BI_TODAM as ::core::ffi::c_int as isize)
                                        + (*item).to_d as ::core::ffi::c_int)))
                            as int16_t;
                    } else {
                        home_damage = (home_damage as ::core::ffi::c_int
                            + num_blow as ::core::ffi::c_int
                                * ((*item).dd as ::core::ffi::c_int
                                    * (*item).ds as ::core::ffi::c_int
                                    + (*borg
                                        .trait_0
                                        .offset(BI_TODAM as ::core::ffi::c_int as isize)
                                        + 8 as ::core::ffi::c_int)))
                            as int16_t;
                    }
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                5 => {
                    num_bow = (num_bow as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, false_0 != 0, i);
                }
                21 => {
                    num_rings = (num_rings as ::core::ffi::c_int
                        + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                20 => {
                    num_neck = (num_neck as ::core::ffi::c_int + (*item).iqty as ::core::ffi::c_int)
                        as int16_t;
                    borg_notice_home_dupe(item, true_0 != 0, i);
                }
                30 | 31 | 32 | 33 | 34 => {
                    if obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
                    {
                        if !flag_has(
                            &raw mut (*k_info.offset((*item).kind as isize)).kind_flags
                                as *mut bitflag,
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
                            KF_GOOD as ::core::ffi::c_int,
                        ) {
                            num_book[(*item).sval as usize] = (num_book[(*item).sval as usize]
                                as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                }
                28 => {
                    if (*item).sval as ::core::ffi::c_int == sv_food_ration {
                        num_food = (num_food as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_food_slime_mold {
                        num_mold = (num_mold as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_mush_purging {
                        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_mush_restoring {
                        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] = (num_fix_stat
                            [STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                26 => {
                    if (*item).sval as ::core::ffi::c_int == sv_potion_cure_critical {
                        num_cure_critical = (num_cure_critical as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_serious {
                        num_cure_serious = (num_cure_serious as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_heat {
                        num_pot_rheat = (num_pot_rheat as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_cold {
                        num_pot_rcold = (num_pot_rcold as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_restore_life {
                        num_fix_exp = (num_fix_exp as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_restore_mana {
                        num_mana = (num_mana as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_healing {
                        num_heal = (num_heal as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_heal_true = (num_heal_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_star_healing {
                        num_ezheal = (num_ezheal as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_ezheal_true = (num_ezheal_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_life {
                        num_life = (num_life as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                        if in_item.is_null() && !no_items {
                            num_life_true = (num_life_true as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_berserk {
                        num_berserk = (num_berserk as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_potion_speed {
                        num_speed = (num_speed as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                25 => {
                    if (*item).sval as ::core::ffi::c_int == sv_scroll_identify {
                        num_ident = (num_ident as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_phase_door {
                        num_phase = (num_phase as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport {
                        num_teleport = (num_teleport as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_word_of_recall {
                        num_recall = (num_recall as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_armor {
                        num_enchant_to_a = (num_enchant_to_a as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_hit
                    {
                        num_enchant_to_h = (num_enchant_to_h as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_dam
                    {
                        num_enchant_to_d = (num_enchant_to_d as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_protection_from_evil {
                        num_pfe = (num_pfe as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_rune_of_protection {
                        num_glyph = (num_glyph as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport_level {
                        num_teleport_level = (num_teleport_level as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_recharging {
                        num_recharge = (num_recharge as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    } else if (*item).sval as ::core::ffi::c_int == sv_scroll_mass_banishment {
                        num_mass_genocide = (num_mass_genocide as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                24 => {
                    if (*item).sval as ::core::ffi::c_int == sv_rod_recall {
                        num_recall = (num_recall as ::core::ffi::c_int
                            + (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                            as int16_t;
                    }
                }
                22 => {
                    if !((*item).pval as ::core::ffi::c_int <= 3 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                            > 30 as ::core::ffi::c_int)
                    {
                        if (*item).sval as ::core::ffi::c_int == sv_staff_teleportation {
                            num_escape = (num_escape as ::core::ffi::c_int
                                + (*item).pval as ::core::ffi::c_int
                                    * (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                            num_tele_staves += 1;
                        }
                    }
                }
                2 | 3 | 4 => {
                    if !((*item).tval as ::core::ffi::c_int
                        != *borg
                            .trait_0
                            .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize))
                    {
                        if !((*item).value <= 0 as int32_t) {
                            num_missile = (num_missile as ::core::ffi::c_int
                                + (*item).iqty as ::core::ffi::c_int)
                                as int16_t;
                        }
                    }
                }
                _ => {}
            }
            if !in_item.is_null() {
                break;
            }
        }
        i += 1;
    }
    if borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
        || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0
    {
        num_food = (num_food as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(IDENTIFY_RUNE) {
        num_ident = (num_ident as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) {
        num_enchant_to_h =
            (num_enchant_to_h as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
        num_enchant_to_d =
            (num_enchant_to_d as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(PROTECTION_FROM_EVIL) {
        num_pfe = (num_pfe as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(GLYPH_OF_WARDING) as ::core::ffi::c_int != 0
        || borg_equips_item(act_glyph, false_0 != 0) as ::core::ffi::c_int != 0
    {
        num_glyph = (num_glyph as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(WORD_OF_RECALL) {
        num_recall = (num_recall as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(TELEPORT_LEVEL) {
        num_teleport_level =
            (num_teleport_level as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if borg_spell_legal(RECHARGING) {
        num_recharge = (num_recharge as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) != 0 {
        num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) != 0 {
        num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_INT as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) != 0 {
        num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_WIS as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) != 0 {
        num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    if *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) != 0 {
        num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] =
            (num_fix_stat[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1000 as ::core::ffi::c_int) as int16_t;
    }
    player_flags(player, &raw mut f as *mut bitflag);
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        num_slow_digest += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        num_ffall += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_LIGHT_2 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut f as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            OF_LIGHT_3 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        num_LIGHT += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        num_regenerate += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        num_telepathy += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        num_see_inv += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        num_free_act += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        num_hold_life += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        num_resist_conf += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        num_resist_blind += 1;
    }
    if (*(*player).race).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_fire += 1;
    }
    if (*(*player).race).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_acid += 1;
    }
    if (*(*player).race).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_cold += 1;
    }
    if (*(*player).race).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        num_immune_elec += 1;
    }
    if (*(*player).race).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_acid += 1;
    }
    if (*(*player).race).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_elec += 1;
    }
    if (*(*player).race).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_fire += 1;
    }
    if (*(*player).race).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_cold += 1;
    }
    if (*(*player).race).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_pois += 1;
    }
    if (*(*player).race).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_LIGHT += 1;
    }
    if (*(*player).race).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_dark += 1;
    }
    if (*(*player).race).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_sound += 1;
    }
    if (*(*player).race).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_shard += 1;
    }
    if (*(*player).race).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_nexus += 1;
    }
    if (*(*player).race).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_neth += 1;
    }
    if (*(*player).race).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_chaos += 1;
    }
    if (*(*player).race).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
        as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        num_resist_disen += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_STR as ::core::ffi::c_int,
    ) {
        num_sustain_str += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_INT as ::core::ffi::c_int,
    ) {
        num_sustain_int += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_WIS as ::core::ffi::c_int,
    ) {
        num_sustain_wis += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_DEX as ::core::ffi::c_int,
    ) {
        num_sustain_dex += 1;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_CON as ::core::ffi::c_int,
    ) {
        num_sustain_con += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_home(mut in_item: *mut borg_item, mut no_items: bool) {
    borg_notice_home_clear(in_item, no_items);
    borg_notice_home_aux(in_item, no_items);
}
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
