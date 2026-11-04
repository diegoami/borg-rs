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
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut player: *mut player;
    static mut stores: *mut store;
    static mut row_top_map: [::core::ffi::c_int; 3];
    static mut row_bottom_map: [::core::ffi::c_int; 3];
    static mut col_map: [::core::ffi::c_int; 3];
    static mut Term: *mut term;
    static mut tile_width: uint8_t;
    static mut tile_height: uint8_t;
    static mut borg_glow_n: int16_t;
    static mut borg_glow_y: [uint8_t; 1536];
    static mut borg_glow_x: [uint8_t; 1536];
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_bold(y: ::core::ffi::c_int, X: ::core::ffi::c_int) -> bool;
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_panel_hgt() -> ::core::ffi::c_int;
    fn borg_panel_wid() -> ::core::ffi::c_int;
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut borg_fear_monsters: [[uint16_t; 199]; 67];
    fn borg_danger_one_kill(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        i: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    fn borg_caution_phase(emergency: ::core::ffi::c_int, turns: ::core::ffi::c_int) -> bool;
    fn borg_shadow_shift(allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_allow_teleport() -> bool;
    static ddx: [int16_t; 10];
    static ddy: [int16_t; 10];
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut f_info: *mut feature;
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg: borg_struct;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    static mut track_less: borg_track;
    fn borg_flow_cost_stair(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        b_stair: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    static mut act_tele_long: ::core::ffi::c_int;
    static mut act_tele_phase: ::core::ffi::c_int;
    fn borg_read_scroll(sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    static mut sv_scroll_phase_door: ::core::ffi::c_int;
    fn borg_primarily_caster() -> bool;
    fn borg_spell_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_los(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_distance(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut borg_detect_door: *mut *mut bool;
    static mut borg_data_cost: *mut borg_data;
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
    static mut track_step: borg_track;
    fn borg_can_dig(check_fail: bool, feat: uint8_t) -> bool;
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
    fn borg_init_track(track: *mut borg_track, size: ::core::ffi::c_int);
    fn borg_free_track(track: *mut borg_track);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PF_MAX: C2RustUnnamed = 18;
pub const PF_COMBAT_REGEN: C2RustUnnamed = 17;
pub const PF_EVIL: C2RustUnnamed = 16;
pub const PF_SHIELD_BASH: C2RustUnnamed = 15;
pub const PF_STEAL: C2RustUnnamed = 14;
pub const PF_ROCK: C2RustUnnamed = 13;
pub const PF_UNLIGHT: C2RustUnnamed = 12;
pub const PF_CHARM: C2RustUnnamed = 11;
pub const PF_NO_MANA: C2RustUnnamed = 10;
pub const PF_SEE_ORE: C2RustUnnamed = 9;
pub const PF_KNOW_ZAPPER: C2RustUnnamed = 8;
pub const PF_KNOW_MUSHROOM: C2RustUnnamed = 7;
pub const PF_CHOOSE_SPELLS: C2RustUnnamed = 6;
pub const PF_BEAM: C2RustUnnamed = 5;
pub const PF_ZERO_FAIL: C2RustUnnamed = 4;
pub const PF_BLESS_WEAPON: C2RustUnnamed = 3;
pub const PF_BRAVERY_30: C2RustUnnamed = 2;
pub const PF_FAST_SHOT: C2RustUnnamed = 1;
pub const PF_NONE: C2RustUnnamed = 0;
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
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub const RF_KILL_WALL: C2RustUnnamed_9 = 47;
pub const RF_MAX: C2RustUnnamed_9 = 85;
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
pub type borg_time = int32_t;
pub const RF_PASS_WALL: C2RustUnnamed_9 = 46;
pub const RF_NEVER_MOVE: C2RustUnnamed_9 = 21;
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
pub const RF_MULTIPLY: C2RustUnnamed_9 = 17;
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
pub const BI_FOOD: C2RustUnnamed_10 = 39;
pub const BI_ISWEAK: C2RustUnnamed_10 = 108;
pub const BI_LIGHT: C2RustUnnamed_10 = 26;
pub const BI_CDEPTH: C2RustUnnamed_10 = 105;
pub const BI_CURHP: C2RustUnnamed_10 = 27;
pub const BI_IFIRE: C2RustUnnamed_10 = 64;
pub const FEAT_LAVA: C2RustUnnamed_8 = 23;
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
pub const BI_MAXSP: C2RustUnnamed_10 = 31;
pub const BI_CURSP: C2RustUnnamed_10 = 30;
pub const FEAT_PERM: C2RustUnnamed_8 = 22;
pub const BI_MAXHP: C2RustUnnamed_10 = 28;
pub const BI_CLEVEL: C2RustUnnamed_10 = 35;
pub const FEAT_MORE: C2RustUnnamed_8 = 6;
pub const FEAT_LESS: C2RustUnnamed_8 = 5;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_8 = 24;
pub const FEAT_SECRET: C2RustUnnamed_8 = 15;
pub const BI_ISHUNGRY: C2RustUnnamed_10 = 109;
pub const BI_ISAFRAID: C2RustUnnamed_10 = 113;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_10 = 118;
pub const BI_ISSTUN: C2RustUnnamed_10 = 117;
pub const BI_ISCUT: C2RustUnnamed_10 = 116;
pub const FEAT_MAGMA_K: C2RustUnnamed_8 = 19;
pub const FEAT_QUARTZ_K: C2RustUnnamed_8 = 20;
pub const BI_GOLD: C2RustUnnamed_10 = 45;
pub const FEAT_RUBBLE: C2RustUnnamed_8 = 16;
pub const FEAT_CLOSED: C2RustUnnamed_8 = 2;
pub const FEAT_BROKEN: C2RustUnnamed_8 = 4;
pub const FEAT_OPEN: C2RustUnnamed_8 = 3;
pub const FEAT_GRANITE: C2RustUnnamed_8 = 21;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_data {
    pub data: [[uint8_t; 198]; 66],
}
pub const FEAT_NONE: C2RustUnnamed_8 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owner {
    pub oidx: ::core::ffi::c_uint,
    pub next: *mut owner,
    pub name: *mut ::core::ffi::c_char,
    pub max_cost: int32_t,
}
pub type keycode_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub user: *mut ::core::ffi::c_void,
    pub data: *mut ::core::ffi::c_void,
    pub user_flag: bool,
    pub data_flag: bool,
    pub active_flag: bool,
    pub mapped_flag: bool,
    pub total_erase: bool,
    pub fixed_shape: bool,
    pub icky_corner: bool,
    pub soft_cursor: bool,
    pub always_pict: bool,
    pub higher_pict: bool,
    pub always_text: bool,
    pub unused_flag: bool,
    pub never_bored: bool,
    pub never_frosh: bool,
    pub sidebar_mode: ::core::ffi::c_int,
    pub complex_input: bool,
    pub key_queue: *mut ui_event,
    pub key_head: uint16_t,
    pub key_tail: uint16_t,
    pub key_xtra: uint16_t,
    pub key_size: uint16_t,
    pub wid: ::core::ffi::c_int,
    pub hgt: ::core::ffi::c_int,
    pub y1: ::core::ffi::c_int,
    pub y2: ::core::ffi::c_int,
    pub x1: *mut ::core::ffi::c_int,
    pub x2: *mut ::core::ffi::c_int,
    pub offset_x: ::core::ffi::c_int,
    pub offset_y: ::core::ffi::c_int,
    pub old: *mut term_win,
    pub scr: *mut term_win,
    pub tmp: *mut term_win,
    pub mem: *mut term_win,
    pub saved: uint8_t,
    pub init_hook: Option<unsafe extern "C" fn(*mut term) -> ()>,
    pub nuke_hook: Option<unsafe extern "C" fn(*mut term) -> ()>,
    pub xtra_hook: Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> errr>,
    pub curs_hook: Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> errr>,
    pub bigcurs_hook: Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> errr>,
    pub wipe_hook: Option<
        unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> errr,
    >,
    pub text_hook: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const wchar_t,
        ) -> errr,
    >,
    pub pict_hook: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const wchar_t,
            *const ::core::ffi::c_int,
            *const wchar_t,
        ) -> errr,
    >,
    pub view_map_hook: Option<unsafe extern "C" fn(*mut term) -> ()>,
    pub dblh_hook: Option<unsafe extern "C" fn(::core::ffi::c_int, wchar_t) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_win {
    pub cu: bool,
    pub cv: bool,
    pub cx: ::core::ffi::c_int,
    pub cy: ::core::ffi::c_int,
    pub cnx: ::core::ffi::c_int,
    pub cny: ::core::ffi::c_int,
    pub a: *mut *mut ::core::ffi::c_int,
    pub c: *mut *mut wchar_t,
    pub va: *mut ::core::ffi::c_int,
    pub vc: *mut wchar_t,
    pub ta: *mut *mut ::core::ffi::c_int,
    pub tc: *mut *mut wchar_t,
    pub vta: *mut ::core::ffi::c_int,
    pub vtc: *mut wchar_t,
    pub next: *mut term_win,
}
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
pub struct mouseclick {
    pub type_0: ui_event_type,
    pub x: uint8_t,
    pub y: uint8_t,
    pub button: uint8_t,
    pub mods: uint8_t,
}
pub const FEAT_MAGMA: C2RustUnnamed_8 = 17;
pub const FEAT_QUARTZ: C2RustUnnamed_8 = 18;
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
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_8 = 25;
pub const FEAT_HOME: C2RustUnnamed_8 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_8 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_8 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_8 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_8 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_8 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_8 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_8 = 7;
pub const FEAT_FLOOR: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_9 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_9 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_9 = 82;
pub const RF_NO_CONF: C2RustUnnamed_9 = 81;
pub const RF_NO_STUN: C2RustUnnamed_9 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_9 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_9 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_9 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_9 = 76;
pub const RF_IM_WATER: C2RustUnnamed_9 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_9 = 74;
pub const RF_IM_POIS: C2RustUnnamed_9 = 73;
pub const RF_IM_COLD: C2RustUnnamed_9 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_9 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_9 = 70;
pub const RF_IM_ACID: C2RustUnnamed_9 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_9 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_9 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_9 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_9 = 65;
pub const RF_METAL: C2RustUnnamed_9 = 64;
pub const RF_NONLIVING: C2RustUnnamed_9 = 63;
pub const RF_UNDEAD: C2RustUnnamed_9 = 62;
pub const RF_EVIL: C2RustUnnamed_9 = 61;
pub const RF_ANIMAL: C2RustUnnamed_9 = 60;
pub const RF_DEMON: C2RustUnnamed_9 = 59;
pub const RF_DRAGON: C2RustUnnamed_9 = 58;
pub const RF_GIANT: C2RustUnnamed_9 = 57;
pub const RF_TROLL: C2RustUnnamed_9 = 56;
pub const RF_ORC: C2RustUnnamed_9 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_9 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_9 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_9 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_9 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_9 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_9 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_9 = 48;
pub const RF_BASH_DOOR: C2RustUnnamed_9 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_9 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_9 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_9 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_9 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_9 = 40;
pub const RF_DROP_20: C2RustUnnamed_9 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_9 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_9 = 37;
pub const RF_DROP_4: C2RustUnnamed_9 = 36;
pub const RF_DROP_3: C2RustUnnamed_9 = 35;
pub const RF_DROP_2: C2RustUnnamed_9 = 34;
pub const RF_DROP_1: C2RustUnnamed_9 = 33;
pub const RF_DROP_60: C2RustUnnamed_9 = 32;
pub const RF_DROP_40: C2RustUnnamed_9 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_9 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_9 = 29;
pub const RF_POWERFUL: C2RustUnnamed_9 = 28;
pub const RF_SPIRIT: C2RustUnnamed_9 = 27;
pub const RF_SMART: C2RustUnnamed_9 = 26;
pub const RF_STUPID: C2RustUnnamed_9 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_9 = 24;
pub const RF_RAND_50: C2RustUnnamed_9 = 23;
pub const RF_RAND_25: C2RustUnnamed_9 = 22;
pub const RF_NEVER_BLOW: C2RustUnnamed_9 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_9 = 19;
pub const RF_REGENERATE: C2RustUnnamed_9 = 18;
pub const RF_UNAWARE: C2RustUnnamed_9 = 16;
pub const RF_SEASONAL: C2RustUnnamed_9 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_9 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_9 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_9 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_9 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_9 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_9 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_9 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_9 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_9 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_9 = 5;
pub const RF_FEMALE: C2RustUnnamed_9 = 4;
pub const RF_MALE: C2RustUnnamed_9 = 3;
pub const RF_QUESTOR: C2RustUnnamed_9 = 2;
pub const RF_UNIQUE: C2RustUnnamed_9 = 1;
pub const RF_NONE: C2RustUnnamed_9 = 0;
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
pub const BI_ISFORGET: C2RustUnnamed_10 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_10 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_10 = 119;
pub const BI_ISPOISONED: C2RustUnnamed_10 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_10 = 114;
pub const BI_ISBLIND: C2RustUnnamed_10 = 112;
pub const BI_ISGORGED: C2RustUnnamed_10 = 111;
pub const BI_ISFULL: C2RustUnnamed_10 = 110;
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
pub const BI_SPEED: C2RustUnnamed_10 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_10 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_10 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_10 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_10 = 40;
pub const BI_RECALL: C2RustUnnamed_10 = 38;
pub const BI_ESP: C2RustUnnamed_10 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_10 = 36;
pub const BI_FAIL2: C2RustUnnamed_10 = 34;
pub const BI_FAIL1: C2RustUnnamed_10 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_10 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_10 = 29;
pub const BI_CLASS: C2RustUnnamed_10 = 25;
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
#[no_mangle]
pub static mut track_shop_x: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut track_shop_y: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut track_vein: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub unsafe extern "C" fn borg_flow_reverse(
    mut depth: ::core::ffi::c_int,
    mut optimize: bool,
    mut avoid: bool,
    mut tunneling: bool,
    mut stair_idx: ::core::ffi::c_int,
    mut sneak: bool,
) {
    borg_flow_clear();
    borg_flow_enqueue_grid(borg.c.y, borg.c.x);
    borg_flow_spread(depth, optimize, avoid, tunneling, stair_idx, sneak);
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_leash(mut pick_up: bool) -> ::core::ffi::c_int {
    let mut leash: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
    if pick_up as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        leash = *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 3 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int;
    }
    if !pick_up
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 5 as ::core::ffi::c_int
    {
        leash = *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 3 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int;
    }
    if borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
        leash += borg.times_twitch as ::core::ffi::c_int;
    }
    return leash;
}
#[no_mangle]
pub unsafe extern "C" fn borg_happy_grid_bold(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
        .offset(x as isize) as *mut borg_grid;
    if y >= DUNGEON_HGT - 2 as ::core::ffi::c_int
        || y <= 2 as ::core::ffi::c_int
        || x >= DUNGEON_WID - 2 as ::core::ffi::c_int
        || x <= 2 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*ag).glyph {
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
        || *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.level >= 2000 as borg_time {
        return false_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y - 2 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 2 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y, x + 2 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y, x - 2 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_step.num as ::core::ffi::c_int {
        if *track_step.y.offset(i as isize) == y && *track_step.x.offset(i as isize) == x {
            if i < 25 as ::core::ffi::c_int {
                return true_0 != 0;
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_recover(mut dist: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 5 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_primarily_caster() {
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
            && (*borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                    / 4 as ::core::ffi::c_int
                || *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
            && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
            && *borg
                .trait_0
                .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                == 0
        {
            return false_0 != 0;
        }
    } else if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 3 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
        && *borg
            .trait_0
            .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
            == 0
    {
        return false_0 != 0;
    }
    if borg.goal.fleeing {
        return false_0 != 0;
    }
    if borg.lunal_mode as ::core::ffi::c_int != 0 || borg.munchkin_mode as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    y = borg.c.y - 25 as ::core::ffi::c_int;
    while y < borg.c.y + 25 as ::core::ffi::c_int {
        x = borg.c.x - 25 as ::core::ffi::c_int;
        while x < borg.c.x + 25 as ::core::ffi::c_int {
            if square_in_bounds(cave, loc(x, y)) {
                if !(y == borg.c.y && x == borg.c.x) {
                    if !(distance(borg.c, loc(x, y)) < 7 as ::core::ffi::c_int) {
                        if borg_happy_grid_bold(y, x) {
                            if !((*borg_grids[y as usize].offset(x as isize)).feat
                                as ::core::ffi::c_int
                                >= FEAT_SECRET as ::core::ffi::c_int
                                && (*borg_grids[y as usize].offset(x as isize)).feat
                                    as ::core::ffi::c_int
                                    != FEAT_PASS_RUBBLE as ::core::ffi::c_int)
                            {
                                if borg_check_rest(y, x) {
                                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                    borg_temp_n += 1;
                                }
                            }
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
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
        dist,
        false_0 != 0,
        true_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"Recover Grid\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_RECOVER,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_RECOVER) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_vein(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut leash: ::core::ffi::c_int = borg_get_leash(true_0 != 0);
    let mut min_feat: uint8_t = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if track_vein.num == 0 {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) >= 100000 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    min_feat = FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t;
    if borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
        min_feat = FEAT_MAGMA_K as ::core::ffi::c_int as uint8_t;
    }
    if !borg_can_dig(true_0 != 0, min_feat) {
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
    let mut current_block_25: u64;
    i = 0 as ::core::ffi::c_int;
    while i < track_vein.num as ::core::ffi::c_int {
        x = *track_vein.x.offset(i as isize);
        y = *track_vein.y.offset(i as isize);
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if !(viewable as ::core::ffi::c_int != 0
            && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
        {
            borg_flow_clear();
            if nearness > 5 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 20 as ::core::ffi::c_int
            {
                cost = borg_flow_cost_stair(y, x, b_stair);
                if cost > leash {
                    current_block_25 = 5143058163439228106;
                } else {
                    current_block_25 = 10043043949733653460;
                }
            } else {
                current_block_25 = 10043043949733653460;
            }
            match current_block_25 {
                5143058163439228106 => {}
                _ => {
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
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"vein\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
static mut spastic_x: uint8_t = 0;
static mut spastic_y: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn borg_flow_spastic(mut bored: bool) -> bool {
    let mut cost: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut b_x: ::core::ffi::c_int = borg.c.x;
    let mut b_y: ::core::ffi::c_int = borg.c.y;
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.level > 3000 as borg_time
        && borg.avoidance as ::core::ffi::c_int
            <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    if !bored {
        let mut p: ::core::ffi::c_int = borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        );
        if p > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
            return false_0 != 0;
        }
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
    if spastic_x as ::core::ffi::c_int == borg.c.x && spastic_y as ::core::ffi::c_int == borg.c.y {
        spastic_x = 0 as uint8_t;
        spastic_y = 0 as uint8_t;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.c.y as isize))
            .offset(borg.c.x as isize) as *mut borg_grid;
        borg_note(format(
            b"# Spastic Searching at (%d,%d)...value:%d\0" as *const u8
                as *const ::core::ffi::c_char,
            borg.c.x,
            borg.c.y,
            (*ag).xtra as ::core::ffi::c_int,
        ));
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            let mut xx: ::core::ffi::c_int = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            let mut yy: ::core::ffi::c_int = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(yy as isize))
                .offset(xx as isize) as *mut borg_grid;
            if ((*ag).xtra as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
                (*ag).xtra =
                    ((*ag).xtra as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t;
            }
            i += 1;
        }
        return false_0 != 0;
    }
    borg_flow_reverse(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    y = 1 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        let mut current_block_69: u64;
        x = 1 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut ag_ptr: [*mut borg_grid; 8] = [::core::ptr::null_mut::<borg_grid>(); 8];
            let mut wall: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut supp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut diag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                if !(*ag).trap {
                    if borg_cave_floor_grid(ag) {
                        cost = (*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int;
                        if !(cost >= 250 as ::core::ffi::c_int) {
                            if !(cost >= 25 as ::core::ffi::c_int
                                && *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    < 30 as ::core::ffi::c_int)
                            {
                                if !(cost >= 50 as ::core::ffi::c_int) {
                                    if !((*ag).xtra as ::core::ffi::c_int
                                        >= 50 as ::core::ffi::c_int)
                                    {
                                        if !((*ag).xtra as ::core::ffi::c_int
                                            >= *borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                                        {
                                            if !(!bored
                                                && (*ag).xtra as ::core::ffi::c_int
                                                    > 5 as ::core::ffi::c_int)
                                            {
                                                if !*(*borg_detect_door
                                                    .offset((y / borg_panel_hgt()) as isize))
                                                .offset((x / borg_panel_wid()) as isize)
                                                {
                                                    if b_stair != -(1 as ::core::ffi::c_int)
                                                        && *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) < 15 as ::core::ffi::c_int
                                                        && borg.avoidance as ::core::ffi::c_int
                                                            <= *borg.trait_0.offset(
                                                                BI_CURHP as ::core::ffi::c_int
                                                                    as isize,
                                                            )
                                                    {
                                                        j = borg_distance(
                                                            *track_less.y.offset(b_stair as isize),
                                                            *track_less.x.offset(b_stair as isize),
                                                            y,
                                                            x,
                                                        );
                                                        b_j = borg_distance(
                                                            borg.c.y,
                                                            borg.c.x,
                                                            *track_less.y.offset(b_stair as isize),
                                                            *track_less.x.offset(b_stair as isize),
                                                        );
                                                        if b_j
                                                            <= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) * 3 as ::core::ffi::c_int
                                                                + 9 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) * 3 as ::core::ffi::c_int
                                                                + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 3 as ::core::ffi::c_int
                                                            && b_j
                                                                <= *borg.trait_0.offset(
                                                                    BI_CLEVEL as ::core::ffi::c_int
                                                                        as isize,
                                                                ) + 9 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 3 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 5 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 10 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else {
                                                            current_block_69 = 12556861819962772176;
                                                        }
                                                    } else {
                                                        current_block_69 = 12556861819962772176;
                                                    }
                                                    match current_block_69 {
                                                        2569451025026770673 => {}
                                                        _ => {
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < 8 as ::core::ffi::c_int {
                                                                let mut xx_0: ::core::ffi::c_int = x
                                                                    + ddx_ddd[i as usize]
                                                                        as ::core::ffi::c_int;
                                                                let mut yy_0: ::core::ffi::c_int = y
                                                                    + ddy_ddd[i as usize]
                                                                        as ::core::ffi::c_int;
                                                                ag_ptr[i as usize] =
                                                                    (*(&raw mut borg_grids
                                                                        as *mut *mut borg_grid)
                                                                        .offset(yy_0 as isize))
                                                                    .offset(xx_0 as isize)
                                                                        as *mut borg_grid;
                                                                i += 1;
                                                            }
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < 4 as ::core::ffi::c_int {
                                                                ag = ag_ptr[i as usize];
                                                                if (*ag).feat as ::core::ffi::c_int
                                                                    >= FEAT_GRANITE
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    wall += 1;
                                                                }
                                                                i += 1;
                                                            }
                                                            if !(wall < 1 as ::core::ffi::c_int) {
                                                                i = 0 as ::core::ffi::c_int;
                                                                while i < 4 as ::core::ffi::c_int {
                                                                    ag = ag_ptr[i as usize];
                                                                    if !((*ag).feat
                                                                        as ::core::ffi::c_int
                                                                        == FEAT_RUBBLE
                                                                            as ::core::ffi::c_int)
                                                                    {
                                                                        if (*ag).feat as ::core::ffi::c_int
                                                                            >= FEAT_SECRET as ::core::ffi::c_int
                                                                            && (*ag).feat as ::core::ffi::c_int
                                                                                <= FEAT_GRANITE as ::core::ffi::c_int
                                                                            || ((*ag).feat as ::core::ffi::c_int
                                                                                == FEAT_OPEN as ::core::ffi::c_int
                                                                                || (*ag).feat as ::core::ffi::c_int
                                                                                    == FEAT_BROKEN as ::core::ffi::c_int)
                                                                            || (*ag).feat as ::core::ffi::c_int
                                                                                == FEAT_CLOSED as ::core::ffi::c_int
                                                                        {
                                                                            supp += 1;
                                                                        }
                                                                    }
                                                                    i += 1;
                                                                }
                                                                i = 4 as ::core::ffi::c_int;
                                                                while i < 8 as ::core::ffi::c_int {
                                                                    ag = ag_ptr[i as usize];
                                                                    if !((*ag).feat
                                                                        as ::core::ffi::c_int
                                                                        == FEAT_RUBBLE
                                                                            as ::core::ffi::c_int)
                                                                    {
                                                                        if (*ag).feat as ::core::ffi::c_int
                                                                            >= FEAT_SECRET as ::core::ffi::c_int
                                                                        {
                                                                            diag += 1;
                                                                        }
                                                                    }
                                                                    i += 1;
                                                                }
                                                                if !(diag < 2 as ::core::ffi::c_int)
                                                                {
                                                                    i = 0 as ::core::ffi::c_int;
                                                                    while i < 8
                                                                        as ::core::ffi::c_int
                                                                    {
                                                                        ag = ag_ptr[i as usize];
                                                                        if (*ag).kill != 0 {
                                                                            monsters += 1;
                                                                        }
                                                                        i += 1;
                                                                    }
                                                                    if !(monsters
                                                                        >= 1 as ::core::ffi::c_int)
                                                                    {
                                                                        v = (supp as borg_time
                                                                            * 500 as borg_time
                                                                            + diag as borg_time
                                                                                * 100 as borg_time
                                                                            - (*ag).xtra
                                                                                as borg_time
                                                                                * 40 as borg_time
                                                                            - cost as borg_time
                                                                                * 2 as borg_time
                                                                            - (borg.time.now
                                                                                - borg.time.level))
                                                                            as ::core::ffi::c_int;
                                                                        v
                                                                            -= (50 as ::core::ffi::c_int
                                                                                - *borg
                                                                                    .trait_0
                                                                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                                                                                * 5 as ::core::ffi::c_int;
                                                                        if !(v <= 0
                                                                            as ::core::ffi::c_int)
                                                                        {
                                                                            if !(!bored && v < 1500 as ::core::ffi::c_int) {
                                                                                if !(b_v >= 0 as ::core::ffi::c_int && v < b_v) {
                                                                                    b_v = v;
                                                                                    b_x = x;
                                                                                    b_y = y;
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
                }
            }
            x += 1;
        }
        y += 1;
    }
    borg_flow_clear();
    if b_v < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(b_y as isize)).offset(b_x as isize)
        as *mut borg_grid;
    spastic_x = b_x as uint8_t;
    spastic_y = b_y as uint8_t;
    borg_flow_enqueue_grid(b_y, b_x);
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"spastic\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_XTRA,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_XTRA) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_shop_entry(mut i: ::core::ffi::c_int) -> bool {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut name: *const ::core::ffi::c_char =
        (*f_info.offset((*stores.offset(i as isize)).feat as isize)).name;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    x = *track_shop_x.offset(i as isize);
    y = *track_shop_y.offset(i as isize);
    if x == 0 || y == 0 {
        return false_0 != 0;
    }
    if x == borg.c.x && y == borg.c.y {
        borg_note(b"# Re-entering a shop\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('5' as i32 as keycode_t);
        return true_0 != 0;
    }
    borg_flow_clear();
    borg_flow_enqueue_grid(y, x);
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(name, GOAL_MISC) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_MISC) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_light(mut why: ::core::ffi::c_int) -> bool {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    borg_glow_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y as isize))
            .offset(x as isize) as *mut borg_grid;
            if !((*ag).info as ::core::ffi::c_int & BORG_GLOW == 0) {
                borg_glow_y[borg_glow_n as usize] = y as uint8_t;
                borg_glow_x[borg_glow_n as usize] = x as uint8_t;
                borg_glow_n += 1;
            }
            x += 1;
        }
        y += 1;
    }
    if borg_glow_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_glow_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_glow_y[i as usize] as ::core::ffi::c_int,
            borg_glow_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"a lighted area\0" as *const u8 as *const ::core::ffi::c_char,
        why,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(why) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_vault(mut nearness: ::core::ffi::c_int) -> bool {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut b_x: ::core::ffi::c_int = 0;
    let mut can_dig_hard: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    if !borg.status.vault {
        return false_0 != 0;
    }
    if !borg_can_dig(false_0 != 0, FEAT_QUARTZ as ::core::ffi::c_int as uint8_t) {
        return false_0 != 0;
    }
    can_dig_hard = borg_can_dig(false_0 != 0, FEAT_GRANITE as ::core::ffi::c_int as uint8_t);
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        let mut current_block_16: u64;
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            if !(distance(borg.c, loc(x, y)) > nearness) {
                let mut feat: uint8_t = (*borg_grids[y as usize].offset(x as isize)).feat;
                if feat as ::core::ffi::c_int != FEAT_RUBBLE as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_QUARTZ as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_MAGMA as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_QUARTZ_K as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_MAGMA_K as ::core::ffi::c_int
                {
                    if !can_dig_hard
                        || feat as ::core::ffi::c_int != FEAT_GRANITE as ::core::ffi::c_int
                    {
                        current_block_16 = 8515828400728868193;
                    } else {
                        current_block_16 = 12800627514080957624;
                    }
                } else {
                    current_block_16 = 12800627514080957624;
                }
                match current_block_16 {
                    8515828400728868193 => {}
                    _ => {
                        i = 0 as ::core::ffi::c_int;
                        while i < 8 as ::core::ffi::c_int {
                            b_x = x + ddx_ddd[i as usize] as ::core::ffi::c_int;
                            b_y = y + ddy_ddd[i as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(b_x, b_y)) {
                                ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                    .offset(b_y as isize))
                                .offset(b_x as isize)
                                    as *mut borg_grid;
                                if !((*ag).feat as ::core::ffi::c_int
                                    != FEAT_PERM as ::core::ffi::c_int)
                                {
                                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                    borg_temp_n += 1;
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"vault excavation\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_VAULT,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_VAULT) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_twitchy() -> bool {
    let mut dir: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0;
    let mut all_walls: bool = true_0 != 0;
    let mut grid: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut l: loc = loc { x: 0, y: 0 };
    borg_note(b"# Twitchy!\0" as *const u8 as *const ::core::ffi::c_char);
    if borg_allow_teleport() {
        if borg_caution_phase(15 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
            && (borg_spell_fail(PHASE_DOOR, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_shadow_shift(40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_phase) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            return true_0 != 0;
        }
    }
    count = 20 as ::core::ffi::c_int;
    loop {
        dir = Rand_div(10 as uint32_t) as int32_t as ::core::ffi::c_int;
        if dir == 5 as ::core::ffi::c_int || dir == 0 as ::core::ffi::c_int {
            continue;
        }
        if count == 0 {
            break;
        }
        count -= 1;
        borg.goal.g.x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
        borg.goal.g.y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
        if !square_in_bounds_fully(cave, borg.goal.g) {
            continue;
        }
        grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.goal.g.y as isize))
            .offset(borg.goal.g.x as isize) as *mut borg_grid;
        if (*grid).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
            && (*grid).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            continue;
        }
        if !((*grid).kill as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                != 0)
        {
            break;
        }
    }
    if count == 0 {
        let mut current_block_17: u64;
        dir = 1 as ::core::ffi::c_int;
        while dir < 10 as ::core::ffi::c_int {
            if !(dir == 5 as ::core::ffi::c_int) {
                l.x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
                l.y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
                if square_in_bounds_fully(cave, l) {
                    grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(l.y as isize))
                        .offset(l.x as isize) as *mut borg_grid;
                    if (*grid).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
                        && (*grid).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
                    {
                        if *borg
                            .trait_0
                            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                            == 0
                            || (*grid).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                        {
                            current_block_17 = 6009453772311597924;
                        } else {
                            current_block_17 = 9828876828309294594;
                        }
                    } else {
                        current_block_17 = 9828876828309294594;
                    }
                    match current_block_17 {
                        6009453772311597924 => {}
                        _ => {
                            if !((*grid).kill as ::core::ffi::c_int != 0
                                && *borg
                                    .trait_0
                                    .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                                    != 0)
                            {
                                all_walls = false_0 != 0;
                                break;
                            }
                        }
                    }
                }
            }
            dir += 1;
        }
        if all_walls {
            borg_keypress('R' as i32 as keycode_t);
            borg_keypress('1' as i32 as keycode_t);
            borg_keypress('0' as i32 as keycode_t);
            borg_keypress('0' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        borg_keypress('+' as i32 as keycode_t);
    }
    borg_keypress((dir + '0' as i32) as keycode_t);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_extract_dir(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if y1 == y2 && x1 == x2 {
        return 5 as ::core::ffi::c_int;
    }
    if x1 == x2 {
        return if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
    }
    if y1 == y2 {
        return if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
    }
    if y1 < y2 {
        return if x1 < x2 {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
    if y1 > y2 {
        return if x1 < x2 {
            9 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
    }
    return 5 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_goto_dir(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    let mut ax: ::core::ffi::c_int = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    e = borg_extract_dir(y1, x1, y2, x2);
    if ax <= 1 as ::core::ffi::c_int && ay <= 1 as ::core::ffi::c_int {
        return e;
    }
    if ay > ax {
        d = if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay < ax {
        d = if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    d = borg_extract_dir(y1, x1, y2, x2);
    if borg_cave_floor_bold(
        y1 + ddy[d as usize] as ::core::ffi::c_int,
        x1 + ddx[d as usize] as ::core::ffi::c_int,
    ) {
        return d;
    }
    if ay <= ax {
        d = if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay >= ax {
        d = if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay == 0 {
        d = if x1 < x2 {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
        d = if x1 < x2 {
            9 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ax == 0 {
        d = if y1 < y2 {
            3 as ::core::ffi::c_int
        } else {
            9 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
        d = if y1 < y2 {
            1 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn borg_check_rest(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut borg_in_vault: bool = false_0 != 0;
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        == *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
        && flag_has(
            &raw mut (*player).state.pflags as *mut bitflag,
            (PF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            PF_COMBAT_REGEN as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.last_kill_mult < 4 as borg_time
        && borg.time.now - borg.time.last_kill_mult > 0 as borg_time
    {
        return false_0 != 0;
    }
    if (borg.temp.bless as ::core::ffi::c_int != 0
        || borg.temp.hero as ::core::ffi::c_int != 0
        || borg.temp.berserk as ::core::ffi::c_int != 0
        || borg.temp.fastcast as ::core::ffi::c_int != 0
        || borg.temp.regen as ::core::ffi::c_int != 0
        || borg.temp.smite_evil as ::core::ffi::c_int != 0)
        && !borg.munchkin_mode
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                * 8 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            >= *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    borg.time.last_kill_mult = 0 as ::core::ffi::c_int as borg_time;
    if borg.status.vault {
        i = -(1 as ::core::ffi::c_int);
        while i < 1 as ::core::ffi::c_int {
            ii = -(1 as ::core::ffi::c_int);
            while ii < 1 as ::core::ffi::c_int {
                if square_in_bounds_fully(cave, loc(borg.c.x + ii, borg.c.y + i)) {
                    if (*borg_grids[(borg.c.y + i) as usize].offset((borg.c.x + ii) as isize)).feat
                        as ::core::ffi::c_int
                        == FEAT_PERM as ::core::ffi::c_int
                    {
                        borg_in_vault = true_0 != 0;
                    }
                }
                ii += 1;
            }
            i += 1;
        }
    }
    if borg.no_rest_prep as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
        && !borg.munchkin_mode
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 85 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*borg_grids[y as usize].offset(x as isize)).feat as ::core::ffi::c_int
        == FEAT_LAVA as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if !borg_in_vault {
        if borg_fear_region[(y / 11 as ::core::ffi::c_int) as usize]
            [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 20 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 100 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if borg_fear_monsters[y as usize][x as usize] as ::core::ffi::c_int
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 10 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 100 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0)
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 40 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 85 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if (*borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
            || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                < 2 as ::core::ffi::c_int)
            && !borg.munchkin_mode
        {
            return false_0 != 0;
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut r_ptr: *mut monster_race =
            r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !((*kill).r_idx == 0) {
            ax = if x9 > x { x9 - x } else { x - x9 };
            ay = if y9 > y { y9 - y } else { y - y9 };
            d = if ax < ay { ay } else { ax };
            if !(d > (*z_info).max_range as ::core::ffi::c_int) {
                if d < 2 as ::core::ffi::c_int
                    && !flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_NEVER_MOVE as ::core::ffi::c_int,
                    )
                {
                    return false_0 != 0;
                }
                if d == 1 as ::core::ffi::c_int {
                    return false_0 != 0;
                }
                if d < 10 as ::core::ffi::c_int
                    && flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_MULTIPLY as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    return false_0 != 0;
                }
                if !(!(*kill).awake && d > 8 as ::core::ffi::c_int && !borg.munchkin_mode) {
                    p = borg_danger_one_kill(
                        y9,
                        x9,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if !borg_in_vault {
                        if d < 5 as ::core::ffi::c_int
                            && p > borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                            && !borg.munchkin_mode
                        {
                            return false_0 != 0;
                        }
                    }
                    if borg_los(y9, x9, y, x) as ::core::ffi::c_int != 0
                        && (*kill).ranged_attack as ::core::ffi::c_int != 0
                    {
                        return false_0 != 0;
                    }
                    if borg.munchkin_mode as ::core::ffi::c_int != 0
                        && borg_los(y9, x9, y, x) as ::core::ffi::c_int != 0
                        && ((*kill).awake as ::core::ffi::c_int != 0
                            && !flag_has(
                                &raw mut (*r_ptr).flags as *mut bitflag,
                                (RF_MAX as ::core::ffi::c_int as size_t)
                                    .wrapping_add(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    )
                                    .wrapping_sub(1 as size_t)
                                    .wrapping_div(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    ),
                                RF_NEVER_MOVE as ::core::ffi::c_int,
                            ))
                    {
                        return false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_PASS_WALL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && !borg_in_vault
                    {
                        return false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_KILL_WALL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && !borg_in_vault
                    {
                        return false_0 != 0;
                    }
                }
            }
        }
        i += 1;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_far_from_stairs(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut b_stair: ::core::ffi::c_int,
) -> bool {
    return borg_flow_far_from_stairs_dist(x, y, b_stair, borg_get_leash(false_0 != 0));
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_far_from_stairs_dist(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut b_stair: ::core::ffi::c_int,
    mut distance_0: ::core::ffi::c_int,
) -> bool {
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - 5 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        let mut cost: ::core::ffi::c_int = borg_flow_cost_stair(y, x, b_stair);
        if cost > distance_0 {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_misc() {
    track_shop_x = mem_zalloc(
        (9 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    track_shop_y = mem_zalloc(
        (9 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    borg_init_track(&raw mut track_vein, 100 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_misc() {
    borg_free_track(&raw mut track_vein);
    if !track_shop_y.is_null() {
        mem_free(track_shop_y as *mut ::core::ffi::c_void);
        track_shop_y = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !track_shop_x.is_null() {
        mem_free(track_shop_x as *mut ::core::ffi::c_void);
        track_shop_x = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_MISC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const GOAL_XTRA: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GOAL_VAULT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GOAL_RECOVER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
