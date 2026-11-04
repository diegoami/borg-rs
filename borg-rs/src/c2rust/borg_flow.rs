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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut k_info: *mut object_kind;
    static mut player: *mut player;
    static mut borg_danger_wipe: bool;
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    fn borg_init_flow_glyph();
    fn borg_free_flow_glyph();
    static mut borg_grids: [*mut borg_grid; 66];
    static ddx: [int16_t; 10];
    static ddy: [int16_t; 10];
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn feat_is_shop(feat: ::core::ffi::c_int) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut weapon_swap: ::core::ffi::c_int;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_kills: *mut borg_kill;
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static mut track_vein: borg_track;
    fn borg_goto_dir(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn borg_init_flow_misc();
    fn borg_free_flow_misc();
    static mut track_less: borg_track;
    fn borg_init_flow_stairs();
    fn borg_free_flow_stairs();
    static mut borg_takes: *mut borg_take;
    fn borg_get_top_object(c: *mut chunk, grid: loc) -> *mut object;
    fn borg_delete_take(i: ::core::ffi::c_int);
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    fn borg_queue_direction(k: keycode_t);
    static mut act_disable_traps: ::core::ffi::c_int;
    static mut act_stone_to_mud: ::core::ffi::c_int;
    static mut act_destroy_doors: ::core::ffi::c_int;
    fn borg_equips_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    static mut sv_ring_digging: ::core::ffi::c_int;
    fn borg_spell_okay(spell: borg_spells) -> bool;
    fn borg_spell(spell: borg_spells) -> bool;
    fn borg_spell_okay_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_target(target: loc, require_monster: bool) -> bool;
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
pub const OPT_MAX: C2RustUnnamed_0 = 46;
pub const OPT_birth_percent_damage: C2RustUnnamed_0 = 45;
pub const OPT_birth_levels_persist: C2RustUnnamed_0 = 44;
pub const OPT_birth_know_flavors: C2RustUnnamed_0 = 43;
pub const OPT_birth_know_runes: C2RustUnnamed_0 = 42;
pub const OPT_birth_ai_learn: C2RustUnnamed_0 = 41;
pub const OPT_birth_start_kit: C2RustUnnamed_0 = 40;
pub const OPT_birth_no_selling: C2RustUnnamed_0 = 39;
pub const OPT_birth_feelings: C2RustUnnamed_0 = 38;
pub const OPT_birth_lose_arts: C2RustUnnamed_0 = 37;
pub const OPT_birth_stacking: C2RustUnnamed_0 = 36;
pub const OPT_birth_no_artifacts: C2RustUnnamed_0 = 35;
pub const OPT_birth_no_recall: C2RustUnnamed_0 = 34;
pub const OPT_birth_force_descend: C2RustUnnamed_0 = 33;
pub const OPT_birth_connect_stairs: C2RustUnnamed_0 = 32;
pub const OPT_birth_randarts: C2RustUnnamed_0 = 31;
pub const OPT_score_live: C2RustUnnamed_0 = 30;
pub const OPT_cheat_live: C2RustUnnamed_0 = 29;
pub const OPT_score_xtra: C2RustUnnamed_0 = 28;
pub const OPT_cheat_xtra: C2RustUnnamed_0 = 27;
pub const OPT_score_room: C2RustUnnamed_0 = 26;
pub const OPT_cheat_room: C2RustUnnamed_0 = 25;
pub const OPT_score_hear: C2RustUnnamed_0 = 24;
pub const OPT_cheat_hear: C2RustUnnamed_0 = 23;
pub const OPT_effective_speed: C2RustUnnamed_0 = 22;
pub const OPT_notify_recharge: C2RustUnnamed_0 = 21;
pub const OPT_mouse_movement: C2RustUnnamed_0 = 20;
pub const OPT_hp_changes_color: C2RustUnnamed_0 = 19;
pub const OPT_auto_more: C2RustUnnamed_0 = 18;
pub const OPT_purple_uniques: C2RustUnnamed_0 = 17;
pub const OPT_center_player: C2RustUnnamed_0 = 16;
pub const OPT_animate_flicker: C2RustUnnamed_0 = 15;
pub const OPT_view_yellow_light: C2RustUnnamed_0 = 14;
pub const OPT_hybrid_walls: C2RustUnnamed_0 = 13;
pub const OPT_solid_walls: C2RustUnnamed_0 = 12;
pub const OPT_disturb_near: C2RustUnnamed_0 = 11;
pub const OPT_highlight_player: C2RustUnnamed_0 = 10;
pub const OPT_show_target: C2RustUnnamed_0 = 9;
pub const OPT_show_flavors: C2RustUnnamed_0 = 8;
pub const OPT_pickup_inven: C2RustUnnamed_0 = 7;
pub const OPT_pickup_always: C2RustUnnamed_0 = 6;
pub const OPT_use_old_target: C2RustUnnamed_0 = 5;
pub const OPT_show_damage: C2RustUnnamed_0 = 4;
pub const OPT_use_sound: C2RustUnnamed_0 = 3;
pub const OPT_autoexplore_commands: C2RustUnnamed_0 = 2;
pub const OPT_rogue_like_commands: C2RustUnnamed_0 = 1;
pub const OPT_none: C2RustUnnamed_0 = 0;
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
pub struct borg_data {
    pub data: [[uint8_t; 198]; 66],
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
pub const BI_ISWEAK: C2RustUnnamed_11 = 108;
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
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_9 = 24;
pub const FEAT_RUBBLE: C2RustUnnamed_9 = 16;
pub const BI_DIG: C2RustUnnamed_11 = 63;
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
pub const FEAT_QUARTZ_K: C2RustUnnamed_9 = 20;
pub const FEAT_QUARTZ: C2RustUnnamed_9 = 18;
pub const FEAT_SECRET: C2RustUnnamed_9 = 15;
pub const FEAT_CLOSED: C2RustUnnamed_9 = 2;
pub const FEAT_GRANITE: C2RustUnnamed_9 = 21;
pub const FEAT_LAVA: C2RustUnnamed_9 = 23;
pub const FEAT_PERM: C2RustUnnamed_9 = 22;
pub const BI_ISHUNGRY: C2RustUnnamed_11 = 109;
pub const BI_CLEVEL: C2RustUnnamed_11 = 35;
pub const BI_FOOD: C2RustUnnamed_11 = 39;
pub const BI_MAXCLEVEL: C2RustUnnamed_11 = 36;
pub const BI_DISM: C2RustUnnamed_11 = 55;
pub const BI_DISP: C2RustUnnamed_11 = 54;
pub const BI_CURHP: C2RustUnnamed_11 = 27;
pub const BI_ISAFRAID: C2RustUnnamed_11 = 113;
pub const FEAT_NONE: C2RustUnnamed_9 = 0;
pub const BI_IFIRE: C2RustUnnamed_11 = 64;
pub type ui_event_type = ::core::ffi::c_uint;
pub const EVT_DISCONNECT: ui_event_type = 256;
pub const EVT_SWITCH: ui_event_type = 128;
pub const EVT_SELECT: ui_event_type = 64;
pub const EVT_MOVE: ui_event_type = 32;
pub const EVT_ESCAPE: ui_event_type = 16;
pub const EVT_BUTTON: ui_event_type = 8;
pub const EVT_RESIZE: ui_event_type = 4;
pub const EVT_MOUSE: ui_event_type = 2;
pub const EVT_KBRD: ui_event_type = 1;
pub const EVT_NONE: ui_event_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui_event {
    pub type_0: ui_event_type,
    pub mouse: mouseclick,
    pub key: keypress,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypress {
    pub type_0: ui_event_type,
    pub code: keycode_t,
    pub mods: uint8_t,
}
pub type keycode_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouseclick {
    pub type_0: ui_event_type,
    pub x: uint8_t,
    pub y: uint8_t,
    pub button: uint8_t,
    pub mods: uint8_t,
}
pub const BI_MAXSP: C2RustUnnamed_11 = 31;
pub const BI_CURSP: C2RustUnnamed_11 = 30;
pub const BI_ISCONFUSED: C2RustUnnamed_11 = 114;
pub const BI_ISBLIND: C2RustUnnamed_11 = 112;
pub const BI_LIGHT: C2RustUnnamed_11 = 26;
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
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_12 = 22;
pub const BI_DEV: C2RustUnnamed_11 = 56;
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
pub const BI_CDEPTH: C2RustUnnamed_11 = 105;
pub const RF_UNIQUE: C2RustUnnamed_10 = 1;
pub const RF_MAX: C2RustUnnamed_10 = 85;
pub const BI_CRSFEAR: C2RustUnnamed_11 = 186;
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
pub const FEAT_LESS: C2RustUnnamed_9 = 5;
pub const FEAT_OPEN: C2RustUnnamed_9 = 3;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_9 = 25;
pub const FEAT_MAGMA_K: C2RustUnnamed_9 = 19;
pub const FEAT_MAGMA: C2RustUnnamed_9 = 17;
pub const FEAT_HOME: C2RustUnnamed_9 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_9 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_9 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_9 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_9 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_9 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_9 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_9 = 7;
pub const FEAT_MORE: C2RustUnnamed_9 = 6;
pub const FEAT_BROKEN: C2RustUnnamed_9 = 4;
pub const FEAT_FLOOR: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_10 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_10 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_10 = 82;
pub const RF_NO_CONF: C2RustUnnamed_10 = 81;
pub const RF_NO_STUN: C2RustUnnamed_10 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_10 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_10 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_10 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_10 = 76;
pub const RF_IM_WATER: C2RustUnnamed_10 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_10 = 74;
pub const RF_IM_POIS: C2RustUnnamed_10 = 73;
pub const RF_IM_COLD: C2RustUnnamed_10 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_10 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_10 = 70;
pub const RF_IM_ACID: C2RustUnnamed_10 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_10 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_10 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_10 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_10 = 65;
pub const RF_METAL: C2RustUnnamed_10 = 64;
pub const RF_NONLIVING: C2RustUnnamed_10 = 63;
pub const RF_UNDEAD: C2RustUnnamed_10 = 62;
pub const RF_EVIL: C2RustUnnamed_10 = 61;
pub const RF_ANIMAL: C2RustUnnamed_10 = 60;
pub const RF_DEMON: C2RustUnnamed_10 = 59;
pub const RF_DRAGON: C2RustUnnamed_10 = 58;
pub const RF_GIANT: C2RustUnnamed_10 = 57;
pub const RF_TROLL: C2RustUnnamed_10 = 56;
pub const RF_ORC: C2RustUnnamed_10 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_10 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_10 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_10 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_10 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_10 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_10 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_10 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_10 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_10 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_10 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_10 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_10 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_10 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_10 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_10 = 40;
pub const RF_DROP_20: C2RustUnnamed_10 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_10 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_10 = 37;
pub const RF_DROP_4: C2RustUnnamed_10 = 36;
pub const RF_DROP_3: C2RustUnnamed_10 = 35;
pub const RF_DROP_2: C2RustUnnamed_10 = 34;
pub const RF_DROP_1: C2RustUnnamed_10 = 33;
pub const RF_DROP_60: C2RustUnnamed_10 = 32;
pub const RF_DROP_40: C2RustUnnamed_10 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_10 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_10 = 29;
pub const RF_POWERFUL: C2RustUnnamed_10 = 28;
pub const RF_SPIRIT: C2RustUnnamed_10 = 27;
pub const RF_SMART: C2RustUnnamed_10 = 26;
pub const RF_STUPID: C2RustUnnamed_10 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_10 = 24;
pub const RF_RAND_50: C2RustUnnamed_10 = 23;
pub const RF_RAND_25: C2RustUnnamed_10 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_10 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_10 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_10 = 19;
pub const RF_REGENERATE: C2RustUnnamed_10 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_10 = 17;
pub const RF_UNAWARE: C2RustUnnamed_10 = 16;
pub const RF_SEASONAL: C2RustUnnamed_10 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_10 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_10 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_10 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_10 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_10 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_10 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_10 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_10 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_10 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_10 = 5;
pub const RF_FEMALE: C2RustUnnamed_10 = 4;
pub const RF_MALE: C2RustUnnamed_10 = 3;
pub const RF_QUESTOR: C2RustUnnamed_10 = 2;
pub const RF_NONE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_11 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_11 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_11 = 263;
pub const BI_EMPTY: C2RustUnnamed_11 = 262;
pub const BI_CARRY: C2RustUnnamed_11 = 261;
pub const BI_WEIGHT: C2RustUnnamed_11 = 260;
pub const BI_DINV: C2RustUnnamed_11 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_11 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_11 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_11 = 256;
pub const BI_ADIGGER: C2RustUnnamed_11 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_11 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_11 = 253;
pub const BI_AROD2: C2RustUnnamed_11 = 252;
pub const BI_AROD1: C2RustUnnamed_11 = 251;
pub const BI_ASHROOM: C2RustUnnamed_11 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_11 = 249;
pub const BI_AHWORD: C2RustUnnamed_11 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_11 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_11 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_11 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_11 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_11 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_11 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_11 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_11 = 240;
pub const BI_ABRAND: C2RustUnnamed_11 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_11 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_11 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_11 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_11 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_11 = 234;
pub const BI_ACLW: C2RustUnnamed_11 = 233;
pub const BI_ACSW: C2RustUnnamed_11 = 232;
pub const BI_ACCW: C2RustUnnamed_11 = 231;
pub const BI_AGLYPH: C2RustUnnamed_11 = 230;
pub const BI_APFE: C2RustUnnamed_11 = 229;
pub const BI_ALITE: C2RustUnnamed_11 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_11 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_11 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_11 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_11 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_11 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_11 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_11 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_11 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_11 = 219;
pub const BI_ASPEED: C2RustUnnamed_11 = 218;
pub const BI_AID: C2RustUnnamed_11 = 217;
pub const BI_ALIFE: C2RustUnnamed_11 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_11 = 215;
pub const BI_AHEAL: C2RustUnnamed_11 = 214;
pub const BI_AFUEL: C2RustUnnamed_11 = 213;
pub const BI_AESCAPE: C2RustUnnamed_11 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_11 = 211;
pub const BI_APHASE: C2RustUnnamed_11 = 210;
pub const BI_WB_POIS: C2RustUnnamed_11 = 209;
pub const BI_WB_COLD: C2RustUnnamed_11 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_11 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_11 = 206;
pub const BI_WB_ACID: C2RustUnnamed_11 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_11 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_11 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_11 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_11 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_11 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_11 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_11 = 198;
pub const BI_WS_ORC: C2RustUnnamed_11 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_11 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_11 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_11 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_11 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_11 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_11 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_11 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_11 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_11 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_11 = 187;
pub const BI_CRSAIRSWING: C2RustUnnamed_11 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_11 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_11 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_11 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_11 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_11 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_11 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_11 = 178;
pub const BI_CRSSICK: C2RustUnnamed_11 = 177;
pub const BI_CRSDULL: C2RustUnnamed_11 = 176;
pub const BI_CRSVULN: C2RustUnnamed_11 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_11 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_11 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_11 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_11 = 171;
pub const BI_CRSSUND: C2RustUnnamed_11 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_11 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_11 = 168;
pub const BI_CRSPARA: C2RustUnnamed_11 = 167;
pub const BI_CRSHALU: C2RustUnnamed_11 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_11 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_11 = 164;
pub const BI_CRSTELE: C2RustUnnamed_11 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_11 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_11 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_11 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_11 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_11 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_11 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_11 = 156;
pub const BI_AMISSILES: C2RustUnnamed_11 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_11 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_11 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_11 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_11 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_11 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_11 = 149;
pub const BI_SHOTS: C2RustUnnamed_11 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_11 = 147;
pub const BI_BLOWS: C2RustUnnamed_11 = 146;
pub const BI_BART: C2RustUnnamed_11 = 145;
pub const BI_SLING: C2RustUnnamed_11 = 144;
pub const BI_BTODAM: C2RustUnnamed_11 = 143;
pub const BI_BTOHIT: C2RustUnnamed_11 = 142;
pub const BI_BID: C2RustUnnamed_11 = 141;
pub const BI_WDS: C2RustUnnamed_11 = 140;
pub const BI_WDD: C2RustUnnamed_11 = 139;
pub const BI_WID: C2RustUnnamed_11 = 138;
pub const BI_WTODAM: C2RustUnnamed_11 = 137;
pub const BI_WTOHIT: C2RustUnnamed_11 = 136;
pub const BI_TODAM: C2RustUnnamed_11 = 135;
pub const BI_TOHIT: C2RustUnnamed_11 = 134;
pub const BI_ARMOR: C2RustUnnamed_11 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_11 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_11 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_11 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_11 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_11 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_11 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_11 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_11 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_11 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_11 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_11 = 122;
pub const BI_ISFORGET: C2RustUnnamed_11 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_11 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_11 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_11 = 118;
pub const BI_ISSTUN: C2RustUnnamed_11 = 117;
pub const BI_ISCUT: C2RustUnnamed_11 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_11 = 115;
pub const BI_ISGORGED: C2RustUnnamed_11 = 111;
pub const BI_ISFULL: C2RustUnnamed_11 = 110;
pub const BI_KING: C2RustUnnamed_11 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_11 = 106;
pub const BI_SFRACT: C2RustUnnamed_11 = 104;
pub const BI_SHLIFE: C2RustUnnamed_11 = 103;
pub const BI_SRDIS: C2RustUnnamed_11 = 102;
pub const BI_SRKAOS: C2RustUnnamed_11 = 101;
pub const BI_SRNTHR: C2RustUnnamed_11 = 100;
pub const BI_SRNXUS: C2RustUnnamed_11 = 99;
pub const BI_SRSHRD: C2RustUnnamed_11 = 98;
pub const BI_SRSND: C2RustUnnamed_11 = 97;
pub const BI_SRCONF: C2RustUnnamed_11 = 96;
pub const BI_SRBLIND: C2RustUnnamed_11 = 95;
pub const BI_SRDARK: C2RustUnnamed_11 = 94;
pub const BI_SRLITE: C2RustUnnamed_11 = 93;
pub const BI_SRFEAR: C2RustUnnamed_11 = 92;
pub const BI_SRPOIS: C2RustUnnamed_11 = 91;
pub const BI_SRACID: C2RustUnnamed_11 = 90;
pub const BI_SRELEC: C2RustUnnamed_11 = 89;
pub const BI_SRCOLD: C2RustUnnamed_11 = 88;
pub const BI_SRFIRE: C2RustUnnamed_11 = 87;
pub const BI_FRACT: C2RustUnnamed_11 = 86;
pub const BI_HLIFE: C2RustUnnamed_11 = 85;
pub const BI_RDIS: C2RustUnnamed_11 = 84;
pub const BI_RKAOS: C2RustUnnamed_11 = 83;
pub const BI_RNTHR: C2RustUnnamed_11 = 82;
pub const BI_RNXUS: C2RustUnnamed_11 = 81;
pub const BI_RSHRD: C2RustUnnamed_11 = 80;
pub const BI_RSND: C2RustUnnamed_11 = 79;
pub const BI_RCONF: C2RustUnnamed_11 = 78;
pub const BI_RBLIND: C2RustUnnamed_11 = 77;
pub const BI_RDARK: C2RustUnnamed_11 = 76;
pub const BI_RLITE: C2RustUnnamed_11 = 75;
pub const BI_RFEAR: C2RustUnnamed_11 = 74;
pub const BI_RPOIS: C2RustUnnamed_11 = 73;
pub const BI_RACID: C2RustUnnamed_11 = 72;
pub const BI_RELEC: C2RustUnnamed_11 = 71;
pub const BI_RCOLD: C2RustUnnamed_11 = 70;
pub const BI_RFIRE: C2RustUnnamed_11 = 69;
pub const BI_IPOIS: C2RustUnnamed_11 = 68;
pub const BI_IELEC: C2RustUnnamed_11 = 67;
pub const BI_ICOLD: C2RustUnnamed_11 = 66;
pub const BI_IACID: C2RustUnnamed_11 = 65;
pub const BI_THT: C2RustUnnamed_11 = 62;
pub const BI_THB: C2RustUnnamed_11 = 61;
pub const BI_THN: C2RustUnnamed_11 = 60;
pub const BI_SRCH: C2RustUnnamed_11 = 59;
pub const BI_STL: C2RustUnnamed_11 = 58;
pub const BI_SAV: C2RustUnnamed_11 = 57;
pub const BI_FAST_SHOTS: C2RustUnnamed_11 = 53;
pub const BI_INFRA: C2RustUnnamed_11 = 52;
pub const BI_SINV: C2RustUnnamed_11 = 51;
pub const BI_REG: C2RustUnnamed_11 = 50;
pub const BI_FEATH: C2RustUnnamed_11 = 49;
pub const BI_SDIG: C2RustUnnamed_11 = 48;
pub const BI_DAM_RED: C2RustUnnamed_11 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_11 = 46;
pub const BI_GOLD: C2RustUnnamed_11 = 45;
pub const BI_SPEED: C2RustUnnamed_11 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_11 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_11 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_11 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_11 = 40;
pub const BI_RECALL: C2RustUnnamed_11 = 38;
pub const BI_ESP: C2RustUnnamed_11 = 37;
pub const BI_FAIL2: C2RustUnnamed_11 = 34;
pub const BI_FAIL1: C2RustUnnamed_11 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_11 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_11 = 29;
pub const BI_MAXHP: C2RustUnnamed_11 = 28;
pub const BI_CLASS: C2RustUnnamed_11 = 25;
pub const BI_SCON: C2RustUnnamed_11 = 24;
pub const BI_SDEX: C2RustUnnamed_11 = 23;
pub const BI_SWIS: C2RustUnnamed_11 = 22;
pub const BI_SINT: C2RustUnnamed_11 = 21;
pub const BI_SSTR: C2RustUnnamed_11 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_11 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_11 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_11 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_11 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_11 = 15;
pub const BI_CCON: C2RustUnnamed_11 = 14;
pub const BI_CDEX: C2RustUnnamed_11 = 13;
pub const BI_CWIS: C2RustUnnamed_11 = 12;
pub const BI_CINT: C2RustUnnamed_11 = 11;
pub const BI_CSTR: C2RustUnnamed_11 = 10;
pub const BI_ACON: C2RustUnnamed_11 = 9;
pub const BI_ADEX: C2RustUnnamed_11 = 8;
pub const BI_AWIS: C2RustUnnamed_11 = 7;
pub const BI_AINT: C2RustUnnamed_11 = 6;
pub const BI_ASTR: C2RustUnnamed_11 = 5;
pub const BI_CON: C2RustUnnamed_11 = 4;
pub const BI_DEX: C2RustUnnamed_11 = 3;
pub const BI_WIS: C2RustUnnamed_11 = 2;
pub const BI_INT: C2RustUnnamed_11 = 1;
pub const BI_STR: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_12 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_12 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_12 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_12 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_12 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_12 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_12 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_12 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_12 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_12 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_12 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_12 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_12 = 23;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_12 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_12 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_12 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_12 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_12 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_12 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_12 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_12 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_12 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_12 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_12 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_12 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_12 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_12 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_12 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_12 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_12 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_12 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_12 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_12 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_12 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_12 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_DIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BORG_DIG_MOD: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const BORG_DIG_HARD: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const AUTO_FLOW_MAX: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
#[no_mangle]
pub static mut flow_head: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut flow_tail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_flow_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_flow_x: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_flow_y: [uint8_t; 1536] = [0; 1536];
#[no_mangle]
pub static mut borg_data_flow: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_cost: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_hard: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_know: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_data_icky: *mut borg_data = ::core::ptr::null::<borg_data>() as *mut borg_data;
#[no_mangle]
pub static mut borg_temp_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_temp_x: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut borg_temp_y: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut track_step: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut track_door: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut track_closed: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut borg_ddx_ddd: [int16_t; 24] = [
    0 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
];
#[no_mangle]
pub static mut borg_ddy_ddd: [int16_t; 24] = [
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(1 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    1 as ::core::ffi::c_int as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    -(2 as ::core::ffi::c_int) as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
    2 as ::core::ffi::c_int as int16_t,
];
#[no_mangle]
pub unsafe extern "C" fn borg_can_dig(mut check_fail: bool, mut feat: uint8_t) -> bool {
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        || (feat as ::core::ffi::c_int) < FEAT_SECRET as ::core::ffi::c_int
            && feat as ::core::ffi::c_int != FEAT_CLOSED as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    let mut dig_check: ::core::ffi::c_int = 0;
    if feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_SECRET as ::core::ffi::c_int
    {
        dig_check = BORG_DIG_HARD;
    } else if feat as ::core::ffi::c_int == FEAT_QUARTZ as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_QUARTZ_K as ::core::ffi::c_int
    {
        dig_check = BORG_DIG_MOD;
    } else {
        dig_check = BORG_DIG;
    }
    if borg.times_twitch as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
        dig_check -= if borg.times_twitch as ::core::ffi::c_int - 10 as ::core::ffi::c_int
            > 19 as ::core::ffi::c_int
        {
            19 as ::core::ffi::c_int
        } else {
            borg.times_twitch as ::core::ffi::c_int - 10 as ::core::ffi::c_int
        };
    }
    if weapon_swap != 0
        && *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) >= dig_check
        && (*borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize)).tval
            as ::core::ffi::c_int
            == TV_DIGGING as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize)
            >= dig_check + 20 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if (feat as ::core::ffi::c_int == FEAT_RUBBLE as ::core::ffi::c_int
        || feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int)
        && *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            == 0
    {
        return true_0 != 0;
    }
    if check_fail {
        if borg_spell_okay_fail(TURN_STONE_TO_MUD, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0
            || borg_spell_okay_fail(SHATTER_STONE, 40 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
            || borg_equips_item(act_stone_to_mud, true_0 != 0) as ::core::ffi::c_int != 0
            || borg_equips_ring(sv_ring_digging) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    } else if borg_spell_okay(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
        || borg_spell_okay(SHATTER_STONE) as ::core::ffi::c_int != 0
        || borg_equips_item(act_stone_to_mud, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_ring(sv_ring_digging) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_clear() {
    memcpy(
        borg_data_cost as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    if borg_danger_wipe {
        memset(
            borg_data_know as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_data>() as size_t,
        );
        memset(
            borg_data_icky as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<borg_data>() as size_t,
        );
        borg_danger_wipe = false_0 != 0;
    }
    flow_head = 0 as ::core::ffi::c_int;
    flow_tail = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_spread(
    mut depth: ::core::ffi::c_int,
    mut optimize: bool,
    mut avoid: bool,
    mut tunneling: bool,
    mut stair_idx: ::core::ffi::c_int,
    mut sneak: bool,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut o: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut fear: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int = 0;
    let mut yy: ::core::ffi::c_int = 0;
    let mut xx: ::core::ffi::c_int = 0;
    let mut bad_sneak: bool = false_0 != 0;
    let mut origin_y: ::core::ffi::c_int = 0;
    let mut origin_x: ::core::ffi::c_int = 0;
    let mut twitchy: bool = false_0 != 0;
    origin_y = borg.c.y;
    origin_x = borg.c.x;
    if borg.avoidance as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        twitchy = true_0 != 0;
    }
    if stair_idx >= 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
    {
        origin_y = *track_less.y.offset(stair_idx as isize);
        origin_x = *track_less.x.offset(stair_idx as isize);
        optimize = false_0 != 0;
    }
    while flow_head != flow_tail {
        x1 = borg_flow_x[flow_tail as usize] as ::core::ffi::c_int;
        y1 = borg_flow_y[flow_tail as usize] as ::core::ffi::c_int;
        flow_tail += 1;
        if flow_tail == AUTO_FLOW_MAX {
            flow_tail = 0 as ::core::ffi::c_int;
        }
        n = (*borg_data_cost).data[y1 as usize][x1 as usize] as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        if n > o {
            if optimize as ::core::ffi::c_int != 0
                && n > (*borg_data_cost).data[origin_y as usize][origin_x as usize]
                    as ::core::ffi::c_int
            {
                break;
            }
            if n > depth {
                break;
            }
            o = n;
        }
        let mut current_block_55: u64;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut old_head: ::core::ffi::c_int = 0;
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            bad_sneak = false_0 != 0;
            x = x1 + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = y1 + ddy_ddd[i as usize] as ::core::ffi::c_int;
            if square_in_bounds_fully(cave, loc(x, y)) {
                if !((*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int <= n) {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                        .offset(x as isize) as *mut borg_grid;
                    if sneak as ::core::ffi::c_int != 0 && !borg.status.desperate && !twitchy {
                        ii = 0 as ::core::ffi::c_int;
                        while ii < 8 as ::core::ffi::c_int {
                            xx = x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                            yy = y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(xx, yy)) {
                                if (*borg_grids[yy as usize].offset(xx as isize)).kill != 0 {
                                    bad_sneak = true_0 != 0;
                                    break;
                                }
                            }
                            ii += 1;
                        }
                    }
                    if !bad_sneak {
                        if !(!tunneling
                            && ((*ag).feat as ::core::ffi::c_int
                                >= FEAT_SECRET as ::core::ffi::c_int
                                && (*ag).feat as ::core::ffi::c_int
                                    != FEAT_PASS_RUBBLE as ::core::ffi::c_int
                                && (*ag).feat as ::core::ffi::c_int
                                    != FEAT_LAVA as ::core::ffi::c_int))
                        {
                            if !((*ag).feat as ::core::ffi::c_int
                                == FEAT_PERM as ::core::ffi::c_int)
                            {
                                if !((*ag).feat as ::core::ffi::c_int
                                    == FEAT_LAVA as ::core::ffi::c_int
                                    && *borg
                                        .trait_0
                                        .offset(BI_IFIRE as ::core::ffi::c_int as isize)
                                        == 0)
                                {
                                    if !((avoid as ::core::ffi::c_int != 0
                                        || borg.status.desperate as ::core::ffi::c_int != 0)
                                        && (*ag).feat as ::core::ffi::c_int
                                            == FEAT_NONE as ::core::ffi::c_int
                                        && !twitchy)
                                    {
                                        if (*ag).kill != 0 {
                                            if borg.status.desperate as ::core::ffi::c_int != 0
                                                || borg.lunal_mode as ::core::ffi::c_int != 0
                                                || borg.munchkin_mode as ::core::ffi::c_int != 0
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else if *borg
                                                .trait_0
                                                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                                                != 0
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else if !twitchy
                                                && *borg
                                                    .trait_0
                                                    .offset(BI_FOOD as ::core::ffi::c_int as isize)
                                                    >= 2 as ::core::ffi::c_int
                                                && *borg.trait_0.offset(
                                                    BI_MAXCLEVEL as ::core::ffi::c_int as isize,
                                                ) < 5 as ::core::ffi::c_int
                                            {
                                                current_block_55 = 13242334135786603907;
                                            } else {
                                                current_block_55 = 790185930182612747;
                                            }
                                        } else {
                                            current_block_55 = 790185930182612747;
                                        }
                                        match current_block_55 {
                                            13242334135786603907 => {}
                                            _ => {
                                                if !(borg.goal.shop as ::core::ffi::c_int
                                                    >= 0 as ::core::ffi::c_int
                                                    && feat_is_shop(
                                                        (*ag).feat as ::core::ffi::c_int,
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0
                                                    && (*ag).store as ::core::ffi::c_int
                                                        != borg.goal.shop as ::core::ffi::c_int
                                                    && y != borg.c.y
                                                    && x != borg.c.x)
                                                {
                                                    if (*ag).trap as ::core::ffi::c_int != 0
                                                        && !(*ag).glyph
                                                        && !twitchy
                                                    {
                                                        if *borg.trait_0.offset(
                                                            BI_CURHP as ::core::ffi::c_int as isize,
                                                        ) < 60 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if *borg.trait_0.offset(
                                                            BI_DISP as ::core::ffi::c_int as isize,
                                                        ) < 30 as ::core::ffi::c_int
                                                            && *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) < 20 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if *borg.trait_0.offset(
                                                            BI_DISP as ::core::ffi::c_int as isize,
                                                        ) < 45 as ::core::ffi::c_int
                                                            && *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) < 10 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if *borg.trait_0.offset(
                                                            BI_DISM as ::core::ffi::c_int as isize,
                                                        ) < 30 as ::core::ffi::c_int
                                                            && *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) < 20 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else if *borg.trait_0.offset(
                                                            BI_DISM as ::core::ffi::c_int as isize,
                                                        ) < 45 as ::core::ffi::c_int
                                                            && *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) < 10 as ::core::ffi::c_int
                                                        {
                                                            current_block_55 = 13242334135786603907;
                                                        } else {
                                                            current_block_55 = 5141539773904409130;
                                                        }
                                                    } else {
                                                        current_block_55 = 5141539773904409130;
                                                    }
                                                    match current_block_55 {
                                                        13242334135786603907 => {}
                                                        _ => {
                                                            if !((*borg_data_icky).data[y as usize]
                                                                [x as usize]
                                                                != 0)
                                                            {
                                                                if (*borg_data_know).data
                                                                    [y as usize]
                                                                    [x as usize]
                                                                    == 0
                                                                {
                                                                    let mut p: ::core::ffi::c_int =
                                                                        0;
                                                                    (*borg_data_know).data
                                                                        [y as usize]
                                                                        [x as usize] =
                                                                        true_0 as uint8_t;
                                                                    if !borg.status.desperate
                                                                        && !borg.lunal_mode
                                                                        && !borg.munchkin_mode
                                                                        && !borg.status.digging
                                                                    {
                                                                        p = borg_danger(
                                                                            y,
                                                                            x,
                                                                            1 as ::core::ffi::c_int,
                                                                            true_0 != 0,
                                                                            false_0 != 0,
                                                                        );
                                                                        if *borg
                                                                            .trait_0
                                                                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                                                            == 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if *borg
                                                                            .trait_0
                                                                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                                                            != 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if borg.mon.scary {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 2 as ::core::ffi::c_int;
                                                                        }
                                                                        if borg.mon.unique != 0
                                                                            && borg.status.vault as ::core::ffi::c_int != 0
                                                                            && *borg
                                                                                .trait_0
                                                                                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                                                                == 50 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int;
                                                                        }
                                                                        if borg.mon.scary as ::core::ffi::c_int != 0
                                                                            && *borg
                                                                                .trait_0
                                                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                                                <= 5 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int;
                                                                        }
                                                                        if borg.goal.ignoring {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 5 as ::core::ffi::c_int;
                                                                        }
                                                                        if borg.time.now
                                                                            - borg.time.level
                                                                            > 5000 as borg_time
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 25 as ::core::ffi::c_int;
                                                                        }
                                                                        if *borg
                                                                            .trait_0
                                                                            .offset(BI_FOOD as ::core::ffi::c_int as isize)
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 100 as ::core::ffi::c_int;
                                                                        }
                                                                        if *borg
                                                                            .trait_0
                                                                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            fear = borg.avoidance as ::core::ffi::c_int
                                                                                * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
                                                                        }
                                                                        if p > fear {
                                                                            (*borg_data_icky)
                                                                                .data
                                                                                [y as usize]
                                                                                [x as usize] =
                                                                                true_0 as uint8_t;
                                                                            current_block_55 = 13242334135786603907;
                                                                        } else {
                                                                            current_block_55 = 16029476503615101993;
                                                                        }
                                                                    } else {
                                                                        current_block_55 =
                                                                            16029476503615101993;
                                                                    }
                                                                } else {
                                                                    current_block_55 =
                                                                        16029476503615101993;
                                                                }
                                                                match current_block_55 {
                                                                    13242334135786603907 => {}
                                                                    _ => {
                                                                        (*borg_data_cost).data
                                                                            [y as usize]
                                                                            [x as usize] =
                                                                            n as uint8_t;
                                                                        borg_flow_x
                                                                            [flow_head as usize] =
                                                                            x as uint8_t;
                                                                        borg_flow_y
                                                                            [flow_head as usize] =
                                                                            y as uint8_t;
                                                                        old_head = flow_head;
                                                                        flow_head += 1;
                                                                        if flow_head
                                                                            == AUTO_FLOW_MAX
                                                                        {
                                                                            flow_head = 0 as ::core::ffi::c_int;
                                                                        }
                                                                        if flow_head == flow_tail {
                                                                            flow_head = old_head;
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
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
    flow_tail = 0 as ::core::ffi::c_int;
    flow_head = flow_tail;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_enqueue_grid(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) {
    let mut old_head: ::core::ffi::c_int = 0;
    let mut fear: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0;
    if (*borg_data_icky).data[y as usize][x as usize] != 0 {
        return;
    }
    if (*borg_data_know).data[y as usize][x as usize] == 0 {
        (*borg_data_know).data[y as usize][x as usize] = true_0 as uint8_t;
        p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            == 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            != 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if borg.mon.scary {
            fear = borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
        }
        if borg.mon.unique != 0
            && borg.status.vault as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                == 50 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 5 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
        }
        if borg.goal.ignoring {
            fear = borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
        }
        if borg.time.now - borg.time.level > 5000 as borg_time {
            fear = borg.avoidance as ::core::ffi::c_int * 25 as ::core::ffi::c_int;
        }
        if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
            fear = borg.avoidance as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
        {
            fear = borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int;
        }
        if p > fear
            && !borg.status.desperate
            && !borg.lunal_mode
            && !borg.munchkin_mode
            && !borg.status.digging
        {
            (*borg_data_icky).data[y as usize][x as usize] = true_0 as uint8_t;
            return;
        }
    }
    if (*borg_data_cost).data[y as usize][x as usize] == 0 {
        return;
    }
    (*borg_data_cost).data[y as usize][x as usize] = 0 as uint8_t;
    borg_flow_y[flow_head as usize] = y as uint8_t;
    borg_flow_x[flow_head as usize] = x as uint8_t;
    old_head = flow_head;
    flow_head += 1;
    if flow_head == AUTO_FLOW_MAX {
        flow_head = 0 as ::core::ffi::c_int;
    }
    if flow_head == flow_tail {
        flow_head = old_head;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_commit(
    mut who: *const ::core::ffi::c_char,
    mut why: ::core::ffi::c_int,
) -> bool {
    let mut cost: ::core::ffi::c_int = 0;
    cost = (*borg_data_cost).data[borg.c.y as usize][borg.c.x as usize] as ::core::ffi::c_int;
    if cost >= 250 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if !who.is_null() {
        borg_note(format(
            b"# Flowing toward %s at cost %d\0" as *const u8 as *const ::core::ffi::c_char,
            who,
            cost,
        ));
    }
    memcpy(
        borg_data_flow as *mut ::core::ffi::c_void,
        borg_data_cost as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    borg.goal.type_0 = why as int16_t;
    return true_0 != 0;
}
unsafe extern "C" fn borg_play_step(
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut ag2: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut ch_evt: ui_event = EVENT_EMPTY;
    let mut dir: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if borg.near.breeder {
        ox = -(1 as ::core::ffi::c_int);
        while ox <= 1 as ::core::ffi::c_int {
            oy = -(1 as ::core::ffi::c_int);
            while oy <= 1 as ::core::ffi::c_int {
                if !(oy + borg.c.y == borg.c.y && ox + borg.c.x == borg.c.x) {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                        .offset((oy + borg.c.y) as isize))
                    .offset((ox + borg.c.x) as isize) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int != FEAT_OPEN as ::core::ffi::c_int) {
                        if !((*ag).kill != 0) {
                            if !(track_door.num as ::core::ffi::c_int >= 255 as ::core::ffi::c_int)
                            {
                                if !(oy + borg.c.y == y2 && ox + borg.c.x == x2) {
                                    o_y = oy;
                                    o_x = ox;
                                    door_found += 1;
                                }
                            }
                        }
                    }
                }
                oy += 1;
            }
            ox += 1;
        }
        if door_found != 0 {
            dir = borg_goto_dir(borg.c.y, borg.c.x, borg.c.y + o_y, borg.c.x + o_x);
            x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
            y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
            borg.goal.g.x = x;
            borg.goal.g.y = y;
            borg_note(b"# Closing a door\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress('c' as i32 as keycode_t);
            borg_queue_direction((dir + '0' as i32) as keycode_t);
            i = 0 as ::core::ffi::c_int;
            while i < track_door.num as ::core::ffi::c_int {
                if *track_door.x.offset(i as isize) == x && *track_door.y.offset(i as isize) == y {
                    return true_0 != 0;
                }
                i += 1;
            }
            if i == track_door.num as ::core::ffi::c_int
                && i < track_door.size as ::core::ffi::c_int
            {
                borg_note(
                    b"# Noting the closing of a door.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                track_door.num += 1;
                *track_door.x.offset(i as isize) = x;
                *track_door.y.offset(i as isize) = y;
            }
            return true_0 != 0;
        }
    }
    if borg.goal.less {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.c.y as isize))
            .offset(borg.c.x as isize) as *mut borg_grid;
        if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg.goal.less = false_0 != 0;
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    dir = borg_goto_dir(borg.c.y, borg.c.x, y2, x2);
    if dir == 5 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
    y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    borg.goal.g.x = x;
    borg.goal.g.y = y;
    if (*ag).kill != 0 {
        let mut kill: *mut borg_kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        if (*kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
            != 0
            || *borg
                .trait_0
                .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
                != 0
        {
            return false_0 != 0;
        }
        if flag_has(
            &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_UNIQUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 5 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        borg_note(format(
            b"# Walking into a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).pos.y,
            (*kill).pos.x,
        ));
        if borg.status.no_alter {
            borg_keypress(';' as i32 as keycode_t);
            borg.status.no_alter = false_0 != 0;
        } else {
            borg_keypress('+' as i32 as keycode_t);
        }
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if (*ag).take as ::core::ffi::c_int != 0 && (*borg_takes.offset((*ag).take as isize)).kidx != 0
    {
        let mut take: *mut borg_take = borg_takes.offset((*ag).take as isize) as *mut borg_take;
        if !strstr(
            (*k_info.offset((*take).kidx as isize)).name,
            b"chest\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            && strstr(
                (*k_info.offset((*take).kidx as isize)).name,
                b"Ruined\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            let mut o_ptr: *mut object = borg_get_top_object(cave, loc(x2, y2));
            if o_ptr.is_null() {
                borg_delete_take((*ag).take as ::core::ffi::c_int);
                return false_0 != 0;
            }
            if (*o_ptr).pval as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                && !(*o_ptr).known.is_null()
                && *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize)
                    - (*o_ptr).pval as ::core::ffi::c_int
                    >= *borg_cfg.offset(BORG_CHEST_FAIL_TOLERANCE as ::core::ffi::c_int as isize)
            {
                borg_note(format(
                    b"# Disarming a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_keypress('D' as i32 as keycode_t);
                borg_queue_direction((dir + '0' as i32) as keycode_t);
                return true_0 != 0;
            }
            if ((*o_ptr).pval as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (*o_ptr).known.is_null()
            {
                borg_note(format(
                    b"# Opening a '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_keypress('o' as i32 as keycode_t);
                borg_queue_direction((dir + '0' as i32) as keycode_t);
                return true_0 != 0;
            }
        }
        if (*take).orbed as ::core::ffi::c_int == false_0
            && ((*take).tval >= TV_SHOT as ::core::ffi::c_int
                && (*take).tval < TV_STAFF as ::core::ffi::c_int)
        {
            if distance(
                loc(
                    (*take).x as ::core::ffi::c_int,
                    (*take).y as ::core::ffi::c_int,
                ),
                borg.c,
            ) == 1 as ::core::ffi::c_int
            {
                if borg_spell_okay_fail(ORB_OF_DRAINING, 25 as ::core::ffi::c_int) {
                    borg_target(
                        loc(
                            (*take).x as ::core::ffi::c_int,
                            (*take).y as ::core::ffi::c_int,
                        ),
                        false_0 != 0,
                    );
                    borg_spell(ORB_OF_DRAINING);
                    borg_note(
                        b"# Orbing an object to check for cursed item.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    borg_keypress('5' as i32 as keycode_t);
                    (*take).orbed = true_0 != 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < 24 as ::core::ffi::c_int {
                        let mut xx: ::core::ffi::c_int = (*take).x as ::core::ffi::c_int
                            + borg_ddx_ddd[i as usize] as ::core::ffi::c_int;
                        let mut yy: ::core::ffi::c_int = (*take).y as ::core::ffi::c_int
                            + borg_ddy_ddd[i as usize] as ::core::ffi::c_int;
                        if square_in_bounds_fully(cave, loc(xx, yy)) {
                            ag2 = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                .offset(yy as isize))
                            .offset(xx as isize)
                                as *mut borg_grid;
                            if (*ag2).take != 0 {
                                (*borg_takes.offset(
                                    (*borg_grids[yy as usize].offset(xx as isize)).take as isize,
                                ))
                                .orbed = true_0 != 0;
                            }
                        }
                        i += 1;
                    }
                    return true_0 != 0;
                }
            }
        }
        borg_note(format(
            b"# Walking onto and deleting a '%s' at (%d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            (*k_info.offset((*take).kidx as isize)).name,
            (*take).y as ::core::ffi::c_int,
            (*take).x as ::core::ffi::c_int,
        ));
        borg_delete_take((*ag).take as ::core::ffi::c_int);
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if (*ag).glyph {
        borg_note(format(
            b"# Walking onto a glyph of warding.\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
        && !borg.mon.scary
        && (*ag).trap as ::core::ffi::c_int != 0
    {
        if borg_activate_item(act_disable_traps) {
            borg_note(
                b"# Activation to Disable Traps, Destroy Doors\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*ag).trap = 0 as ::core::ffi::c_int != 0;
            borg.no_rest_prep = 3000 as int16_t;
            borg_target(borg.goal.g, false_0 != 0);
            return true_0 != 0;
        }
        if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                * 4 as ::core::ffi::c_int
                / 5 as ::core::ffi::c_int
        {
            if borg_spell(DISABLE_TRAPS_DESTROY_DOORS) {
                borg_note(
                    b"# Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*ag).trap = 0 as ::core::ffi::c_int != 0;
                borg.no_rest_prep = 3000 as int16_t;
                return true_0 != 0;
            }
        }
        borg_note(b"# Disarming a trap\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('D' as i32 as keycode_t);
        borg_queue_direction((dir + '0' as i32) as keycode_t);
        (*ag).trap = 0 as ::core::ffi::c_int != 0;
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int {
        if Rand_div(100 as uint32_t) as int32_t == 0 {
            return false_0 != 0;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            ag2 = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if (*ag2).kill as ::core::ffi::c_int != 0
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 15 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                    == 0
            {
                return false_0 != 0;
            }
            i += 1;
        }
        if Rand_div(100 as uint32_t) as int32_t == 0
            || borg.antibounce_count as ::core::ffi::c_int >= 500 as ::core::ffi::c_int
        {
            if borg_spell(DISABLE_TRAPS_DESTROY_DOORS) as ::core::ffi::c_int != 0
                || borg_activate_item(act_destroy_doors) as ::core::ffi::c_int != 0
            {
                borg_note(
                    b"# Disable Traps, Destroy Doors\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if borg_spell(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
                || borg_spell(SHATTER_STONE) as ::core::ffi::c_int != 0
                || borg_activate_ring(sv_ring_digging) as ::core::ffi::c_int != 0
                || borg_activate_item(act_stone_to_mud) as ::core::ffi::c_int != 0
            {
                borg_note(b"# Melting a door\0" as *const u8 as *const ::core::ffi::c_char);
                borg_keypress((dir + '0' as i32) as keycode_t);
                if track_closed.num != 0 {
                    track_closed.num = 0 as int16_t;
                }
                return true_0 != 0;
            }
        }
        if borg.status.need_alter {
            borg_keypress('+' as i32 as keycode_t);
            borg.status.need_alter = false_0 != 0;
        } else {
            borg_note(b"# Opening a door\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress('o' as i32 as keycode_t);
        }
        borg_queue_direction((dir + '0' as i32) as keycode_t);
        if track_closed.num != 0 {
            track_closed.num = 0 as int16_t;
        }
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int <= FEAT_GRANITE as ::core::ffi::c_int
    {
        if (*ag).feat as ::core::ffi::c_int != FEAT_RUBBLE as ::core::ffi::c_int
            && borg.goal.type_0 as ::core::ffi::c_int == GOAL_DARK
        {
            return false_0 != 0;
        }
        if !borg_can_dig(false_0 != 0, (*ag).feat) {
            borg.goal.type_0 = 0 as int16_t;
            return false_0 != 0;
        }
        if borg_spell(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
            || borg_spell(SHATTER_STONE) as ::core::ffi::c_int != 0
            || borg_activate_ring(sv_ring_digging) as ::core::ffi::c_int != 0
            || borg_activate_item(act_stone_to_mud) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Melting a wall/etc\0" as *const u8 as *const ::core::ffi::c_char);
            borg_keypress((dir + '0' as i32) as keycode_t);
            track_vein.num = 0 as int16_t;
            return true_0 != 0;
        }
        if Rand_div(500 as uint32_t) as int32_t <= 5 as int32_t && !borg.status.vault {
            return false_0 != 0;
        }
        borg_note(b"# Digging through wall/etc\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('T' as i32 as keycode_t);
        borg_keypress((dir + '0' as i32) as keycode_t);
        track_vein.num = 0 as int16_t;
        return true_0 != 0;
    }
    if feat_is_shop((*ag).feat as ::core::ffi::c_int) {
        borg_note(format(
            b"# Entering a '%d' shop\0" as *const u8 as *const ::core::ffi::c_char,
            (*ag).store as ::core::ffi::c_int,
        ));
        borg_keypress((dir + '0' as i32) as keycode_t);
        return true_0 != 0;
    }
    if borg.status.need_alter {
        borg_keypress('+' as i32 as keycode_t);
        borg.status.need_alter = false_0 != 0;
    }
    borg_keypress((dir + '0' as i32) as keycode_t);
    borg.in_shop = false_0 != 0;
    if ch_evt.type_0 as ::core::ffi::c_uint
        & EVT_SELECT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        ch_evt.type_0 = EVT_KBRD;
    }
    if ch_evt.type_0 as ::core::ffi::c_uint & EVT_MOVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        ch_evt.type_0 = EVT_KBRD;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_old(mut why: ::core::ffi::c_int) -> bool {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if borg.goal.type_0 as ::core::ffi::c_int == why {
        let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0;
        let mut b_c: ::core::ffi::c_int = 0;
        b_c = (*borg_data_flow).data[borg.c.y as usize][borg.c.x as usize] as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int;
        b_c = b_c - 5 as ::core::ffi::c_int;
        let mut current_block_9: u64;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            c = (*borg_data_flow).data[y as usize][x as usize] as ::core::ffi::c_int
                * 10 as ::core::ffi::c_int;
            if !(c > b_c) {
                if !(x > AUTO_MAX_X - 1 as ::core::ffi::c_int
                    || x < 1 as ::core::ffi::c_int
                    || y > AUTO_MAX_Y - 1 as ::core::ffi::c_int
                    || y < 1 as ::core::ffi::c_int)
                {
                    if c < b_c {
                        b_n = 0 as ::core::ffi::c_int;
                    }
                    if !(*borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                        == 0 as ::core::ffi::c_int
                        && {
                            b_n += 1;
                            b_n >= 2 as ::core::ffi::c_int
                        }
                        && Rand_div(b_n as uint32_t) as int32_t != 0 as int32_t)
                    {
                        if !(*borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            >= 1 as ::core::ffi::c_int
                            && {
                                b_n += 1;
                                b_n >= 2 as ::core::ffi::c_int
                            })
                        {
                            if borg.goal.type_0 as ::core::ffi::c_int == GOAL_DIGGING
                                && (ddx_ddd[i as usize] as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    || ddy_ddd[i as usize] as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                            {
                                if distance(
                                    borg.c,
                                    loc(
                                        borg_flow_x[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                        borg_flow_y[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                    ),
                                ) <= 2 as ::core::ffi::c_int
                                {
                                    current_block_9 = 17778012151635330486;
                                } else {
                                    current_block_9 = 17860125682698302841;
                                }
                            } else {
                                current_block_9 = 17860125682698302841;
                            }
                            match current_block_9 {
                                17778012151635330486 => {}
                                _ => {
                                    b_i = i;
                                    b_c = c;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if b_i >= 0 as ::core::ffi::c_int {
            x = borg.c.x + ddx_ddd[b_i as usize] as ::core::ffi::c_int;
            y = borg.c.y + ddy_ddd[b_i as usize] as ::core::ffi::c_int;
            if borg_play_step(y, x) {
                return true_0 != 0;
            }
        }
        if borg.goal.type_0 as ::core::ffi::c_int == GOAL_DIGGING
            && borg.c.y == borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            && borg.c.x == borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        {
            borg.time.antisummon = borg.time.now;
        }
        borg.goal.type_0 = 0 as int16_t;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_track(mut track: *mut borg_track, mut size: ::core::ffi::c_int) {
    (*track).num = 0 as int16_t;
    (*track).size = size as int16_t;
    (*track).x = mem_zalloc(
        (size as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    (*track).y = mem_zalloc(
        (size as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_track(mut track: *mut borg_track) {
    (*track).num = 0 as int16_t;
    (*track).size = 0 as int16_t;
    if !(*track).x.is_null() {
        mem_free((*track).x as *mut ::core::ffi::c_void);
        (*track).x = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !(*track).y.is_null() {
        mem_free((*track).y as *mut ::core::ffi::c_void);
        (*track).y = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    borg_free_flow();
    borg_data_flow = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_cost = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_hard = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_know = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    borg_data_icky = mem_zalloc(::core::mem::size_of::<borg_data>() as size_t) as *mut borg_data;
    y = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        x = 0 as ::core::ffi::c_int;
        while x < AUTO_MAX_X {
            (*borg_data_hard).data[y as usize][x as usize] = 255 as uint8_t;
            x += 1;
        }
        y += 1;
    }
    borg_init_track(&raw mut track_step, 100 as ::core::ffi::c_int);
    borg_init_track(&raw mut track_door, 100 as ::core::ffi::c_int);
    borg_init_track(&raw mut track_closed, 100 as ::core::ffi::c_int);
    borg_init_flow_stairs();
    borg_init_flow_glyph();
    borg_init_flow_misc();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow() {
    borg_free_flow_misc();
    borg_free_flow_glyph();
    borg_free_flow_stairs();
    borg_free_track(&raw mut track_closed);
    borg_free_track(&raw mut track_door);
    borg_free_track(&raw mut track_step);
    if !borg_data_flow.is_null() {
        mem_free(borg_data_flow as *mut ::core::ffi::c_void);
        borg_data_flow = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_icky.is_null() {
        mem_free(borg_data_icky as *mut ::core::ffi::c_void);
        borg_data_icky = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_know.is_null() {
        mem_free(borg_data_know as *mut ::core::ffi::c_void);
        borg_data_know = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_hard.is_null() {
        mem_free(borg_data_hard as *mut ::core::ffi::c_void);
        borg_data_hard = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_cost.is_null() {
        mem_free(borg_data_cost as *mut ::core::ffi::c_void);
        borg_data_cost = ::core::ptr::null_mut::<borg_data>();
    }
    if !borg_data_flow.is_null() {
        mem_free(borg_data_flow as *mut ::core::ffi::c_void);
        borg_data_flow = ::core::ptr::null_mut::<borg_data>();
    }
}
static mut EVENT_EMPTY: ui_event = ui_event { type_0: EVT_NONE };
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const GOAL_DARK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
