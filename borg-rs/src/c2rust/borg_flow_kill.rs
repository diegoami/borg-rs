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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn string_free(str: *mut ::core::ffi::c_char);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn my_strnicmp(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn my_stristr(
        string: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn my_strcpy(
        buf: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        bufsize: size_t,
    ) -> size_t;
    fn prefix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn prefix_i(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn suffix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut k_info: *mut object_kind;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_isvault(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    fn square_monster(c: *mut chunk, grid: loc) -> *mut monster;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    static mut borg_flow_y: [uint8_t; 1536];
    static mut borg_flow_x: [uint8_t; 1536];
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
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
    static mut row_top_map: [::core::ffi::c_int; 3];
    static mut row_bottom_map: [::core::ffi::c_int; 3];
    static mut col_map: [::core::ffi::c_int; 3];
    static mut Term: *mut term;
    static mut tile_width: uint8_t;
    static mut tile_height: uint8_t;
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_bold(y: ::core::ffi::c_int, X: ::core::ffi::c_int) -> bool;
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_panel_hgt() -> ::core::ffi::c_int;
    fn borg_panel_wid() -> ::core::ffi::c_int;
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut borg_danger_wipe: bool;
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
    fn borg_primarily_caster() -> bool;
    fn borg_spell_okay(spell: borg_spells) -> bool;
    fn borg_attack_aux_launch() -> ::core::ffi::c_int;
    fn borg_attack_aux_spell_bolt(
        spell: borg_spells,
        rad: ::core::ffi::c_int,
        dam: ::core::ffi::c_int,
        typ: ::core::ffi::c_int,
        max_range: ::core::ffi::c_int,
        is_arc: bool,
    ) -> ::core::ffi::c_int;
    fn borg_flow_far_from_stairs(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        b_stair: ::core::ffi::c_int,
    ) -> bool;
    static mut track_less: borg_track;
    static mut borg_takes_nxt: int16_t;
    static mut borg_takes: *mut borg_take;
    fn borg_delete_take(i: ::core::ffi::c_int);
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_massage_special_chars(name: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut act_stone_to_mud: ::core::ffi::c_int;
    fn borg_equips_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    static mut sv_ring_digging: ::core::ffi::c_int;
    fn borg_los(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_projectable(
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
    static mut borg_detect_wall: *mut *mut bool;
    fn borg_sort(u: *mut ::core::ffi::c_void, v: *mut ::core::ffi::c_void, n: ::core::ffi::c_int);
    fn borg_sort_comp_hook(
        u: *mut ::core::ffi::c_void,
        v: *mut ::core::ffi::c_void,
        a: ::core::ffi::c_int,
        b: ::core::ffi::c_int,
    ) -> bool;
    fn borg_sort_swap_hook(
        u: *mut ::core::ffi::c_void,
        v: *mut ::core::ffi::c_void,
        a: ::core::ffi::c_int,
        b: ::core::ffi::c_int,
    );
    static mut borg_sort_comp: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> bool,
    >;
    static mut borg_sort_swap: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    static extract_energy: [uint8_t; 200];
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed = 24;
pub const FEAT_LAVA: C2RustUnnamed = 23;
pub const FEAT_PERM: C2RustUnnamed = 22;
pub const FEAT_GRANITE: C2RustUnnamed = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed = 19;
pub const FEAT_QUARTZ: C2RustUnnamed = 18;
pub const FEAT_MAGMA: C2RustUnnamed = 17;
pub const FEAT_RUBBLE: C2RustUnnamed = 16;
pub const FEAT_SECRET: C2RustUnnamed = 15;
pub const FEAT_HOME: C2RustUnnamed = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed = 7;
pub const FEAT_MORE: C2RustUnnamed = 6;
pub const FEAT_LESS: C2RustUnnamed = 5;
pub const FEAT_BROKEN: C2RustUnnamed = 4;
pub const FEAT_OPEN: C2RustUnnamed = 3;
pub const FEAT_CLOSED: C2RustUnnamed = 2;
pub const FEAT_FLOOR: C2RustUnnamed = 1;
pub const FEAT_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MON_TMD_MAX: C2RustUnnamed_0 = 10;
pub const MON_TMD_CHANGED: C2RustUnnamed_0 = 9;
pub const MON_TMD_COMMAND: C2RustUnnamed_0 = 8;
pub const MON_TMD_DISEN: C2RustUnnamed_0 = 7;
pub const MON_TMD_HOLD: C2RustUnnamed_0 = 6;
pub const MON_TMD_FAST: C2RustUnnamed_0 = 5;
pub const MON_TMD_SLOW: C2RustUnnamed_0 = 4;
pub const MON_TMD_FEAR: C2RustUnnamed_0 = 3;
pub const MON_TMD_CONF: C2RustUnnamed_0 = 2;
pub const MON_TMD_STUN: C2RustUnnamed_0 = 1;
pub const MON_TMD_SLEEP: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const RF_MAX: C2RustUnnamed_1 = 85;
pub const RF_NO_SLOW: C2RustUnnamed_1 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_1 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_1 = 82;
pub const RF_NO_CONF: C2RustUnnamed_1 = 81;
pub const RF_NO_STUN: C2RustUnnamed_1 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_1 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_1 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_1 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_1 = 76;
pub const RF_IM_WATER: C2RustUnnamed_1 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_1 = 74;
pub const RF_IM_POIS: C2RustUnnamed_1 = 73;
pub const RF_IM_COLD: C2RustUnnamed_1 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_1 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_1 = 70;
pub const RF_IM_ACID: C2RustUnnamed_1 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_1 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_1 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_1 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_1 = 65;
pub const RF_METAL: C2RustUnnamed_1 = 64;
pub const RF_NONLIVING: C2RustUnnamed_1 = 63;
pub const RF_UNDEAD: C2RustUnnamed_1 = 62;
pub const RF_EVIL: C2RustUnnamed_1 = 61;
pub const RF_ANIMAL: C2RustUnnamed_1 = 60;
pub const RF_DEMON: C2RustUnnamed_1 = 59;
pub const RF_DRAGON: C2RustUnnamed_1 = 58;
pub const RF_GIANT: C2RustUnnamed_1 = 57;
pub const RF_TROLL: C2RustUnnamed_1 = 56;
pub const RF_ORC: C2RustUnnamed_1 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_1 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_1 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_1 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_1 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_1 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_1 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_1 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_1 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_1 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_1 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_1 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_1 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_1 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_1 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_1 = 40;
pub const RF_DROP_20: C2RustUnnamed_1 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_1 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_1 = 37;
pub const RF_DROP_4: C2RustUnnamed_1 = 36;
pub const RF_DROP_3: C2RustUnnamed_1 = 35;
pub const RF_DROP_2: C2RustUnnamed_1 = 34;
pub const RF_DROP_1: C2RustUnnamed_1 = 33;
pub const RF_DROP_60: C2RustUnnamed_1 = 32;
pub const RF_DROP_40: C2RustUnnamed_1 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_1 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_1 = 29;
pub const RF_POWERFUL: C2RustUnnamed_1 = 28;
pub const RF_SPIRIT: C2RustUnnamed_1 = 27;
pub const RF_SMART: C2RustUnnamed_1 = 26;
pub const RF_STUPID: C2RustUnnamed_1 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_1 = 24;
pub const RF_RAND_50: C2RustUnnamed_1 = 23;
pub const RF_RAND_25: C2RustUnnamed_1 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_1 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_1 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_1 = 19;
pub const RF_REGENERATE: C2RustUnnamed_1 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_1 = 17;
pub const RF_UNAWARE: C2RustUnnamed_1 = 16;
pub const RF_SEASONAL: C2RustUnnamed_1 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_1 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_1 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_1 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_1 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_1 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_1 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_1 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_1 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_1 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_1 = 5;
pub const RF_FEMALE: C2RustUnnamed_1 = 4;
pub const RF_MALE: C2RustUnnamed_1 = 3;
pub const RF_QUESTOR: C2RustUnnamed_1 = 2;
pub const RF_UNIQUE: C2RustUnnamed_1 = 1;
pub const RF_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const RSF_MAX: C2RustUnnamed_2 = 92;
pub const RSF_S_UNIQUE: C2RustUnnamed_2 = 91;
pub const RSF_S_WRAITH: C2RustUnnamed_2 = 90;
pub const RSF_S_HI_DRAGON: C2RustUnnamed_2 = 89;
pub const RSF_S_HI_UNDEAD: C2RustUnnamed_2 = 88;
pub const RSF_S_DRAGON: C2RustUnnamed_2 = 87;
pub const RSF_S_UNDEAD: C2RustUnnamed_2 = 86;
pub const RSF_S_DEMON: C2RustUnnamed_2 = 85;
pub const RSF_S_AINU: C2RustUnnamed_2 = 84;
pub const RSF_S_HYDRA: C2RustUnnamed_2 = 83;
pub const RSF_S_HOUND: C2RustUnnamed_2 = 82;
pub const RSF_S_SPIDER: C2RustUnnamed_2 = 81;
pub const RSF_S_ANIMAL: C2RustUnnamed_2 = 80;
pub const RSF_S_MONSTERS: C2RustUnnamed_2 = 79;
pub const RSF_S_MONSTER: C2RustUnnamed_2 = 78;
pub const RSF_S_HI_DEMON: C2RustUnnamed_2 = 77;
pub const RSF_S_KIN: C2RustUnnamed_2 = 76;
pub const RSF_SHAPECHANGE: C2RustUnnamed_2 = 75;
pub const RSF_FORGET: C2RustUnnamed_2 = 74;
pub const RSF_TRAPS: C2RustUnnamed_2 = 73;
pub const RSF_DARKNESS: C2RustUnnamed_2 = 72;
pub const RSF_TELE_LEVEL: C2RustUnnamed_2 = 71;
pub const RSF_TELE_AWAY: C2RustUnnamed_2 = 70;
pub const RSF_TELE_SELF_TO: C2RustUnnamed_2 = 69;
pub const RSF_TELE_TO: C2RustUnnamed_2 = 68;
pub const RSF_TPORT: C2RustUnnamed_2 = 67;
pub const RSF_BLINK: C2RustUnnamed_2 = 66;
pub const RSF_HEAL_KIN: C2RustUnnamed_2 = 65;
pub const RSF_HEAL: C2RustUnnamed_2 = 64;
pub const RSF_HASTE: C2RustUnnamed_2 = 63;
pub const RSF_HOLD: C2RustUnnamed_2 = 62;
pub const RSF_SLOW: C2RustUnnamed_2 = 61;
pub const RSF_CONF: C2RustUnnamed_2 = 60;
pub const RSF_BLIND: C2RustUnnamed_2 = 59;
pub const RSF_SCARE: C2RustUnnamed_2 = 58;
pub const RSF_BE_NETH: C2RustUnnamed_2 = 57;
pub const RSF_BE_ELEC: C2RustUnnamed_2 = 56;
pub const RSF_MISSILE: C2RustUnnamed_2 = 55;
pub const RSF_BO_ICE: C2RustUnnamed_2 = 54;
pub const RSF_BO_PLAS: C2RustUnnamed_2 = 53;
pub const RSF_BO_MANA: C2RustUnnamed_2 = 52;
pub const RSF_BO_WATE: C2RustUnnamed_2 = 51;
pub const RSF_BO_NETH: C2RustUnnamed_2 = 50;
pub const RSF_BO_POIS: C2RustUnnamed_2 = 49;
pub const RSF_BO_COLD: C2RustUnnamed_2 = 48;
pub const RSF_BO_FIRE: C2RustUnnamed_2 = 47;
pub const RSF_BO_ELEC: C2RustUnnamed_2 = 46;
pub const RSF_BO_ACID: C2RustUnnamed_2 = 45;
pub const RSF_WOUND: C2RustUnnamed_2 = 44;
pub const RSF_BRAIN_SMASH: C2RustUnnamed_2 = 43;
pub const RSF_MIND_BLAST: C2RustUnnamed_2 = 42;
pub const RSF_DRAIN_MANA: C2RustUnnamed_2 = 41;
pub const RSF_STORM: C2RustUnnamed_2 = 40;
pub const RSF_BA_LIGHT: C2RustUnnamed_2 = 39;
pub const RSF_BA_DARK: C2RustUnnamed_2 = 38;
pub const RSF_BA_HOLY: C2RustUnnamed_2 = 37;
pub const RSF_BA_MANA: C2RustUnnamed_2 = 36;
pub const RSF_BA_WATE: C2RustUnnamed_2 = 35;
pub const RSF_BA_NETH: C2RustUnnamed_2 = 34;
pub const RSF_BA_SHAR: C2RustUnnamed_2 = 33;
pub const RSF_BA_POIS: C2RustUnnamed_2 = 32;
pub const RSF_BA_COLD: C2RustUnnamed_2 = 31;
pub const RSF_BA_FIRE: C2RustUnnamed_2 = 30;
pub const RSF_BA_ELEC: C2RustUnnamed_2 = 29;
pub const RSF_BA_ACID: C2RustUnnamed_2 = 28;
pub const RSF_WEAVE: C2RustUnnamed_2 = 27;
pub const RSF_BOULDER: C2RustUnnamed_2 = 26;
pub const RSF_BR_MANA: C2RustUnnamed_2 = 25;
pub const RSF_BR_WALL: C2RustUnnamed_2 = 24;
pub const RSF_BR_PLAS: C2RustUnnamed_2 = 23;
pub const RSF_BR_SHAR: C2RustUnnamed_2 = 22;
pub const RSF_BR_GRAV: C2RustUnnamed_2 = 21;
pub const RSF_BR_INER: C2RustUnnamed_2 = 20;
pub const RSF_BR_TIME: C2RustUnnamed_2 = 19;
pub const RSF_BR_NEXU: C2RustUnnamed_2 = 18;
pub const RSF_BR_DISE: C2RustUnnamed_2 = 17;
pub const RSF_BR_CHAO: C2RustUnnamed_2 = 16;
pub const RSF_BR_SOUN: C2RustUnnamed_2 = 15;
pub const RSF_BR_DARK: C2RustUnnamed_2 = 14;
pub const RSF_BR_LIGHT: C2RustUnnamed_2 = 13;
pub const RSF_BR_NETH: C2RustUnnamed_2 = 12;
pub const RSF_BR_POIS: C2RustUnnamed_2 = 11;
pub const RSF_BR_COLD: C2RustUnnamed_2 = 10;
pub const RSF_BR_FIRE: C2RustUnnamed_2 = 9;
pub const RSF_BR_ELEC: C2RustUnnamed_2 = 8;
pub const RSF_BR_ACID: C2RustUnnamed_2 = 7;
pub const RSF_BOLT: C2RustUnnamed_2 = 6;
pub const RSF_ARROW: C2RustUnnamed_2 = 5;
pub const RSF_SHOT: C2RustUnnamed_2 = 4;
pub const RSF_SPIT: C2RustUnnamed_2 = 3;
pub const RSF_WHIP: C2RustUnnamed_2 = 2;
pub const RSF_SHRIEK: C2RustUnnamed_2 = 1;
pub const RSF_NONE: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_3 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_3 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_3 = 263;
pub const BI_EMPTY: C2RustUnnamed_3 = 262;
pub const BI_CARRY: C2RustUnnamed_3 = 261;
pub const BI_WEIGHT: C2RustUnnamed_3 = 260;
pub const BI_DINV: C2RustUnnamed_3 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_3 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_3 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_3 = 256;
pub const BI_ADIGGER: C2RustUnnamed_3 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_3 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_3 = 253;
pub const BI_AROD2: C2RustUnnamed_3 = 252;
pub const BI_AROD1: C2RustUnnamed_3 = 251;
pub const BI_ASHROOM: C2RustUnnamed_3 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_3 = 249;
pub const BI_AHWORD: C2RustUnnamed_3 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_3 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_3 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_3 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_3 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_3 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_3 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_3 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_3 = 240;
pub const BI_ABRAND: C2RustUnnamed_3 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_3 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_3 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_3 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_3 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_3 = 234;
pub const BI_ACLW: C2RustUnnamed_3 = 233;
pub const BI_ACSW: C2RustUnnamed_3 = 232;
pub const BI_ACCW: C2RustUnnamed_3 = 231;
pub const BI_AGLYPH: C2RustUnnamed_3 = 230;
pub const BI_APFE: C2RustUnnamed_3 = 229;
pub const BI_ALITE: C2RustUnnamed_3 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_3 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_3 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_3 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_3 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_3 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_3 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_3 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_3 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_3 = 219;
pub const BI_ASPEED: C2RustUnnamed_3 = 218;
pub const BI_AID: C2RustUnnamed_3 = 217;
pub const BI_ALIFE: C2RustUnnamed_3 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_3 = 215;
pub const BI_AHEAL: C2RustUnnamed_3 = 214;
pub const BI_AFUEL: C2RustUnnamed_3 = 213;
pub const BI_AESCAPE: C2RustUnnamed_3 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_3 = 211;
pub const BI_APHASE: C2RustUnnamed_3 = 210;
pub const BI_WB_POIS: C2RustUnnamed_3 = 209;
pub const BI_WB_COLD: C2RustUnnamed_3 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_3 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_3 = 206;
pub const BI_WB_ACID: C2RustUnnamed_3 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_3 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_3 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_3 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_3 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_3 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_3 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_3 = 198;
pub const BI_WS_ORC: C2RustUnnamed_3 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_3 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_3 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_3 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_3 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_3 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_3 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_3 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_3 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_3 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_3 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_3 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_3 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_3 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_3 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_3 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_3 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_3 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_3 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_3 = 178;
pub const BI_CRSSICK: C2RustUnnamed_3 = 177;
pub const BI_CRSDULL: C2RustUnnamed_3 = 176;
pub const BI_CRSVULN: C2RustUnnamed_3 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_3 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_3 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_3 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_3 = 171;
pub const BI_CRSSUND: C2RustUnnamed_3 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_3 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_3 = 168;
pub const BI_CRSPARA: C2RustUnnamed_3 = 167;
pub const BI_CRSHALU: C2RustUnnamed_3 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_3 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_3 = 164;
pub const BI_CRSTELE: C2RustUnnamed_3 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_3 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_3 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_3 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_3 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_3 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_3 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_3 = 156;
pub const BI_AMISSILES: C2RustUnnamed_3 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_3 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_3 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_3 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_3 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_3 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_3 = 149;
pub const BI_SHOTS: C2RustUnnamed_3 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_3 = 147;
pub const BI_BLOWS: C2RustUnnamed_3 = 146;
pub const BI_BART: C2RustUnnamed_3 = 145;
pub const BI_SLING: C2RustUnnamed_3 = 144;
pub const BI_BTODAM: C2RustUnnamed_3 = 143;
pub const BI_BTOHIT: C2RustUnnamed_3 = 142;
pub const BI_BID: C2RustUnnamed_3 = 141;
pub const BI_WDS: C2RustUnnamed_3 = 140;
pub const BI_WDD: C2RustUnnamed_3 = 139;
pub const BI_WID: C2RustUnnamed_3 = 138;
pub const BI_WTODAM: C2RustUnnamed_3 = 137;
pub const BI_WTOHIT: C2RustUnnamed_3 = 136;
pub const BI_TODAM: C2RustUnnamed_3 = 135;
pub const BI_TOHIT: C2RustUnnamed_3 = 134;
pub const BI_ARMOR: C2RustUnnamed_3 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_3 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_3 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_3 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_3 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_3 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_3 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_3 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_3 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_3 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_3 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_3 = 122;
pub const BI_ISFORGET: C2RustUnnamed_3 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_3 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_3 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_3 = 118;
pub const BI_ISSTUN: C2RustUnnamed_3 = 117;
pub const BI_ISCUT: C2RustUnnamed_3 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_3 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_3 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_3 = 113;
pub const BI_ISBLIND: C2RustUnnamed_3 = 112;
pub const BI_ISGORGED: C2RustUnnamed_3 = 111;
pub const BI_ISFULL: C2RustUnnamed_3 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_3 = 109;
pub const BI_ISWEAK: C2RustUnnamed_3 = 108;
pub const BI_KING: C2RustUnnamed_3 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_3 = 106;
pub const BI_CDEPTH: C2RustUnnamed_3 = 105;
pub const BI_SFRACT: C2RustUnnamed_3 = 104;
pub const BI_SHLIFE: C2RustUnnamed_3 = 103;
pub const BI_SRDIS: C2RustUnnamed_3 = 102;
pub const BI_SRKAOS: C2RustUnnamed_3 = 101;
pub const BI_SRNTHR: C2RustUnnamed_3 = 100;
pub const BI_SRNXUS: C2RustUnnamed_3 = 99;
pub const BI_SRSHRD: C2RustUnnamed_3 = 98;
pub const BI_SRSND: C2RustUnnamed_3 = 97;
pub const BI_SRCONF: C2RustUnnamed_3 = 96;
pub const BI_SRBLIND: C2RustUnnamed_3 = 95;
pub const BI_SRDARK: C2RustUnnamed_3 = 94;
pub const BI_SRLITE: C2RustUnnamed_3 = 93;
pub const BI_SRFEAR: C2RustUnnamed_3 = 92;
pub const BI_SRPOIS: C2RustUnnamed_3 = 91;
pub const BI_SRACID: C2RustUnnamed_3 = 90;
pub const BI_SRELEC: C2RustUnnamed_3 = 89;
pub const BI_SRCOLD: C2RustUnnamed_3 = 88;
pub const BI_SRFIRE: C2RustUnnamed_3 = 87;
pub const BI_FRACT: C2RustUnnamed_3 = 86;
pub const BI_HLIFE: C2RustUnnamed_3 = 85;
pub const BI_RDIS: C2RustUnnamed_3 = 84;
pub const BI_RKAOS: C2RustUnnamed_3 = 83;
pub const BI_RNTHR: C2RustUnnamed_3 = 82;
pub const BI_RNXUS: C2RustUnnamed_3 = 81;
pub const BI_RSHRD: C2RustUnnamed_3 = 80;
pub const BI_RSND: C2RustUnnamed_3 = 79;
pub const BI_RCONF: C2RustUnnamed_3 = 78;
pub const BI_RBLIND: C2RustUnnamed_3 = 77;
pub const BI_RDARK: C2RustUnnamed_3 = 76;
pub const BI_RLITE: C2RustUnnamed_3 = 75;
pub const BI_RFEAR: C2RustUnnamed_3 = 74;
pub const BI_RPOIS: C2RustUnnamed_3 = 73;
pub const BI_RACID: C2RustUnnamed_3 = 72;
pub const BI_RELEC: C2RustUnnamed_3 = 71;
pub const BI_RCOLD: C2RustUnnamed_3 = 70;
pub const BI_RFIRE: C2RustUnnamed_3 = 69;
pub const BI_IPOIS: C2RustUnnamed_3 = 68;
pub const BI_IELEC: C2RustUnnamed_3 = 67;
pub const BI_ICOLD: C2RustUnnamed_3 = 66;
pub const BI_IACID: C2RustUnnamed_3 = 65;
pub const BI_IFIRE: C2RustUnnamed_3 = 64;
pub const BI_DIG: C2RustUnnamed_3 = 63;
pub const BI_THT: C2RustUnnamed_3 = 62;
pub const BI_THB: C2RustUnnamed_3 = 61;
pub const BI_THN: C2RustUnnamed_3 = 60;
pub const BI_SRCH: C2RustUnnamed_3 = 59;
pub const BI_STL: C2RustUnnamed_3 = 58;
pub const BI_SAV: C2RustUnnamed_3 = 57;
pub const BI_DEV: C2RustUnnamed_3 = 56;
pub const BI_DISM: C2RustUnnamed_3 = 55;
pub const BI_DISP: C2RustUnnamed_3 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_3 = 53;
pub const BI_INFRA: C2RustUnnamed_3 = 52;
pub const BI_SINV: C2RustUnnamed_3 = 51;
pub const BI_REG: C2RustUnnamed_3 = 50;
pub const BI_FEATH: C2RustUnnamed_3 = 49;
pub const BI_SDIG: C2RustUnnamed_3 = 48;
pub const BI_DAM_RED: C2RustUnnamed_3 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_3 = 46;
pub const BI_GOLD: C2RustUnnamed_3 = 45;
pub const BI_SPEED: C2RustUnnamed_3 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_3 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_3 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_3 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_3 = 40;
pub const BI_FOOD: C2RustUnnamed_3 = 39;
pub const BI_RECALL: C2RustUnnamed_3 = 38;
pub const BI_ESP: C2RustUnnamed_3 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_3 = 36;
pub const BI_CLEVEL: C2RustUnnamed_3 = 35;
pub const BI_FAIL2: C2RustUnnamed_3 = 34;
pub const BI_FAIL1: C2RustUnnamed_3 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_3 = 32;
pub const BI_MAXSP: C2RustUnnamed_3 = 31;
pub const BI_CURSP: C2RustUnnamed_3 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_3 = 29;
pub const BI_MAXHP: C2RustUnnamed_3 = 28;
pub const BI_CURHP: C2RustUnnamed_3 = 27;
pub const BI_LIGHT: C2RustUnnamed_3 = 26;
pub const BI_CLASS: C2RustUnnamed_3 = 25;
pub const BI_SCON: C2RustUnnamed_3 = 24;
pub const BI_SDEX: C2RustUnnamed_3 = 23;
pub const BI_SWIS: C2RustUnnamed_3 = 22;
pub const BI_SINT: C2RustUnnamed_3 = 21;
pub const BI_SSTR: C2RustUnnamed_3 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_3 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_3 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_3 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_3 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_3 = 15;
pub const BI_CCON: C2RustUnnamed_3 = 14;
pub const BI_CDEX: C2RustUnnamed_3 = 13;
pub const BI_CWIS: C2RustUnnamed_3 = 12;
pub const BI_CINT: C2RustUnnamed_3 = 11;
pub const BI_CSTR: C2RustUnnamed_3 = 10;
pub const BI_ACON: C2RustUnnamed_3 = 9;
pub const BI_ADEX: C2RustUnnamed_3 = 8;
pub const BI_AWIS: C2RustUnnamed_3 = 7;
pub const BI_AINT: C2RustUnnamed_3 = 6;
pub const BI_ASTR: C2RustUnnamed_3 = 5;
pub const BI_CON: C2RustUnnamed_3 = 4;
pub const BI_DEX: C2RustUnnamed_3 = 3;
pub const BI_WIS: C2RustUnnamed_3 = 2;
pub const BI_INT: C2RustUnnamed_3 = 1;
pub const BI_STR: C2RustUnnamed_3 = 0;
pub type borg_time = int32_t;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_4 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_4 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_4 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_4 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_4 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_4 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_4 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_4 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_4 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_4 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_4 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_4 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_4 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_4 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_4 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_4 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_4 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_4 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_4 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_4 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_4 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_4 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_4 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_4 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_4 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_4 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_4 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_4 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_4 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_4 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_4 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_4 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_4 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_4 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_4 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_4 = 0;
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
    pub temp: C2RustUnnamed_12,
    pub status: C2RustUnnamed_11,
    pub time: C2RustUnnamed_10,
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
    pub goal: C2RustUnnamed_9,
    pub init_save: C2RustUnnamed_8,
    pub mon: C2RustUnnamed_7,
    pub id: C2RustUnnamed_6,
    pub near: C2RustUnnamed_5,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
pub struct C2RustUnnamed_12 {
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
pub struct keypress {
    pub type_0: ui_event_type,
    pub code: keycode_t,
    pub mods: uint8_t,
}
pub type keycode_t = uint32_t;
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
pub struct mouseclick {
    pub type_0: ui_event_type,
    pub x: uint8_t,
    pub y: uint8_t,
    pub button: uint8_t,
    pub mods: uint8_t,
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
pub const BORG_ATTACK_NETHER: C2RustUnnamed_13 = 19;
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
pub const BORG_ATTACK_MISSILE: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const BORG_ATTACK_CURSE: C2RustUnnamed_13 = 58;
pub const BORG_ATTACK_ELEC_STRIKE: C2RustUnnamed_13 = 57;
pub const BORG_ATTACK_DRAIN_LIFE: C2RustUnnamed_13 = 56;
pub const BORG_ATTACK_TAP_UNLIFE: C2RustUnnamed_13 = 55;
pub const BORG_ATTACK_AWAY_EVIL: C2RustUnnamed_13 = 54;
pub const BORG_ATTACK_TURN_UNDEAD: C2RustUnnamed_13 = 53;
pub const BORG_ATTACK_OLD_POLY: C2RustUnnamed_13 = 52;
pub const BORG_ATTACK_OLD_SLEEP: C2RustUnnamed_13 = 51;
pub const BORG_ATTACK_OLD_SLOW: C2RustUnnamed_13 = 50;
pub const BORG_ATTACK_TURN_ALL: C2RustUnnamed_13 = 49;
pub const BORG_ATTACK_OLD_CONF: C2RustUnnamed_13 = 48;
pub const BORG_ATTACK_DISP_ALL: C2RustUnnamed_13 = 47;
pub const BORG_ATTACK_AWAY_ALL_MORGOTH: C2RustUnnamed_13 = 46;
pub const BORG_ATTACK_AWAY_ALL: C2RustUnnamed_13 = 45;
pub const BORG_ATTACK_TURN_EVIL: C2RustUnnamed_13 = 44;
pub const BORG_ATTACK_AWAY_UNDEAD: C2RustUnnamed_13 = 43;
pub const BORG_ATTACK_MAKE_TRAP: C2RustUnnamed_13 = 42;
pub const BORG_ATTACK_MAKE_DOOR: C2RustUnnamed_13 = 41;
pub const BORG_ATTACK_MAKE_WALL: C2RustUnnamed_13 = 40;
pub const BORG_ATTACK_KILL_TRAP: C2RustUnnamed_13 = 39;
pub const BORG_ATTACK_KILL_DOOR: C2RustUnnamed_13 = 38;
pub const BORG_ATTACK_DARK_WEAK: C2RustUnnamed_13 = 37;
pub const BORG_ATTACK_OLD_SPEED: C2RustUnnamed_13 = 36;
pub const BORG_ATTACK_OLD_CLONE: C2RustUnnamed_13 = 35;
pub const BORG_ATTACK_OLD_HEAL: C2RustUnnamed_13 = 34;
pub const BORG_ATTACK_WATER: C2RustUnnamed_13 = 33;
pub const BORG_ATTACK_DARK: C2RustUnnamed_13 = 32;
pub const BORG_ATTACK_LIGHT: C2RustUnnamed_13 = 31;
pub const BORG_ATTACK_TIME: C2RustUnnamed_13 = 30;
pub const BORG_ATTACK_INERTIA: C2RustUnnamed_13 = 29;
pub const BORG_ATTACK_FORCE: C2RustUnnamed_13 = 28;
pub const BORG_ATTACK_NEXUS: C2RustUnnamed_13 = 27;
pub const BORG_ATTACK_DISEN: C2RustUnnamed_13 = 26;
pub const BORG_ATTACK_CONFU: C2RustUnnamed_13 = 25;
pub const BORG_ATTACK_PLASMA: C2RustUnnamed_13 = 24;
pub const BORG_ATTACK_SOUND: C2RustUnnamed_13 = 23;
pub const BORG_ATTACK_SHARD: C2RustUnnamed_13 = 22;
pub const BORG_ATTACK_GRAVITY: C2RustUnnamed_13 = 21;
pub const BORG_ATTACK_CHAOS: C2RustUnnamed_13 = 20;
pub const BORG_ATTACK_KILL_WALL: C2RustUnnamed_13 = 18;
pub const BORG_ATTACK_OLD_DRAIN: C2RustUnnamed_13 = 17;
pub const BORG_ATTACK_LIGHT_WEAK: C2RustUnnamed_13 = 16;
pub const BORG_ATTACK_HOLY_WORD: C2RustUnnamed_13 = 15;
pub const BORG_ATTACK_SLEEP_EVIL: C2RustUnnamed_13 = 14;
pub const BORG_ATTACK_DISP_SPIRITS: C2RustUnnamed_13 = 13;
pub const BORG_ATTACK_DISP_EVIL: C2RustUnnamed_13 = 12;
pub const BORG_ATTACK_DISP_UNDEAD: C2RustUnnamed_13 = 11;
pub const BORG_ATTACK_HOLY_ORB: C2RustUnnamed_13 = 10;
pub const BORG_ATTACK_ICE: C2RustUnnamed_13 = 9;
pub const BORG_ATTACK_POIS: C2RustUnnamed_13 = 8;
pub const BORG_ATTACK_COLD: C2RustUnnamed_13 = 7;
pub const BORG_ATTACK_FIRE: C2RustUnnamed_13 = 6;
pub const BORG_ATTACK_ELEC: C2RustUnnamed_13 = 5;
pub const BORG_ATTACK_ACID: C2RustUnnamed_13 = 4;
pub const BORG_ATTACK_METEOR: C2RustUnnamed_13 = 3;
pub const BORG_ATTACK_MANA: C2RustUnnamed_13 = 2;
pub const BORG_ATTACK_ARROW: C2RustUnnamed_13 = 1;
pub const SHRT_MAX: ::core::ffi::c_int = __SHRT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_kills_cnt: int16_t = 0;
#[no_mangle]
pub static mut borg_kills_nxt: int16_t = 0;
#[no_mangle]
pub static mut borg_kills: *mut borg_kill = ::core::ptr::null::<borg_kill>() as *mut borg_kill;
#[no_mangle]
pub static mut borg_race_count: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
#[no_mangle]
pub static mut borg_race_death: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
static mut borg_unique_size: ::core::ffi::c_int = 0;
static mut borg_unique_what: *mut ::core::ffi::c_uint =
    ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint;
static mut borg_unique_text: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
static mut borg_normal_size: ::core::ffi::c_int = 0;
static mut borg_normal_what: *mut ::core::ffi::c_uint =
    ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint;
static mut borg_normal_text: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_nasties_num: uint8_t = 7 as uint8_t;
#[no_mangle]
pub static mut borg_nasties_count: [uint8_t; 7] = [0; 7];
#[no_mangle]
pub static mut borg_nasties: [::core::ffi::c_char; 7] = [
    'Z' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut borg_nasties_limit: [uint8_t; 7] = [
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn borg_race_name(
    mut r_idx: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if r_idx == 0 {
        return b"dead monster\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if r_idx >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        return b"player ghost\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return (*r_info.offset(r_idx as isize)).name;
}
unsafe extern "C" fn borg_update_kill_new(mut i: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pct: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut m_ptr: *mut monster = (*cave).monsters.offset((*kill).m_idx as isize) as *mut monster;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    (*kill).speed = (*m_ptr).mspeed;
    if (*m_ptr).maxhp != 0 {
        pct = (100 as ::core::ffi::c_long * (*m_ptr).hp as ::core::ffi::c_long
            / (if (*m_ptr).maxhp as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*m_ptr).maxhp as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    } else {
        pct = 100 as ::core::ffi::c_int;
    }
    (*kill).power =
        ((*m_ptr).maxhp as ::core::ffi::c_int * pct / 100 as ::core::ffi::c_int) as int16_t;
    (*kill).injury = (100 as ::core::ffi::c_int - pct) as int16_t;
    (*kill).level = (*r_ptr).level as int16_t;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) {
        (*kill).awake = true_0 != 0;
    }
    (*kill).m_idx = (*square_monster(cave, loc((*kill).pos.x, (*kill).pos.y))).midx as int16_t;
    if (*m_ptr).m_timed[MON_TMD_SLEEP as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).awake = true_0 != 0;
    } else {
        (*kill).awake = false_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_FEAR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).afraid = false_0 != 0;
    } else {
        (*kill).afraid = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_CONF as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).confused = false_0 != 0;
    } else {
        (*kill).confused = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_STUN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).stunned = false_0 != 0;
    } else {
        (*kill).stunned = true_0 != 0;
    }
    (*kill).spell_flags[0 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[0 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[1 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[1 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[2 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[2 as ::core::ffi::c_int as usize] as uint32_t;
    k = RSF_NONE as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while k < RSF_MAX as ::core::ffi::c_int {
        if flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            k,
        ) {
            let fresh2 = num;
            num = num + 1;
            (*kill).spell[fresh2 as usize] = k as uint8_t;
        }
        k += 1;
    }
    (*kill).ranged_attack = num as uint8_t;
    if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
        j = ((*kill).pos.y - borg_panel_hgt() / 2 as ::core::ffi::c_int) / borg_panel_hgt()
            * borg_panel_hgt();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_HGT
            - ((*Term).hgt
                - row_top_map[(*Term).sidebar_mode as usize]
                - row_bottom_map[(*Term).sidebar_mode as usize])
                / tile_height as ::core::ffi::c_int
        {
            j = DUNGEON_HGT
                - ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int;
        }
        borg.mon.morgoth_panel.y = j;
        j = ((*kill).pos.x - borg_panel_wid() / 2 as ::core::ffi::c_int) / borg_panel_wid()
            * borg_panel_wid();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_WID
            - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            j = DUNGEON_WID
                - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                    / tile_width as ::core::ffi::c_int;
        }
        borg.mon.morgoth_panel.x = j;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat =
            FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat
        as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_PASS_WALL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat =
            FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
}
unsafe extern "C" fn borg_update_kill_old(mut i: ::core::ffi::c_int) {
    let mut t: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pct: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut m_ptr: *mut monster = square_monster(cave, loc((*kill).pos.x, (*kill).pos.y));
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*m_ptr).maxhp != 0 {
        pct = (100 as ::core::ffi::c_long * (*m_ptr).hp as ::core::ffi::c_long
            / (if (*m_ptr).maxhp as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*m_ptr).maxhp as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    } else {
        pct = 100 as ::core::ffi::c_int;
    }
    (*kill).power =
        ((*m_ptr).maxhp as ::core::ffi::c_int * pct / 100 as ::core::ffi::c_int) as int16_t;
    (*kill).injury = (100 as ::core::ffi::c_int - pct) as int16_t;
    if (*m_ptr).m_timed[MON_TMD_SLEEP as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).awake = true_0 != 0;
    } else {
        (*kill).awake = false_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_FEAR as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).afraid = false_0 != 0;
    } else {
        (*kill).afraid = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_CONF as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).confused = false_0 != 0;
    } else {
        (*kill).confused = true_0 != 0;
    }
    if (*m_ptr).m_timed[MON_TMD_STUN as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*kill).stunned = false_0 != 0;
    } else {
        (*kill).stunned = true_0 != 0;
    }
    (*kill).m_idx = (*square_monster(cave, loc((*kill).pos.x, (*kill).pos.y))).midx as int16_t;
    (*kill).speed = (*m_ptr).mspeed;
    e = extract_energy[*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) as usize]
        as ::core::ffi::c_int;
    t = (100 as ::core::ffi::c_int + (e - 1 as ::core::ffi::c_int)) / e;
    e = extract_energy[(*kill).speed as usize] as ::core::ffi::c_int;
    (*kill).moves = (t * e / 10 as ::core::ffi::c_int) as uint8_t;
    (*kill).spell_flags[0 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[0 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[1 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[1 as ::core::ffi::c_int as usize] as uint32_t;
    (*kill).spell_flags[2 as ::core::ffi::c_int as usize] =
        (*r_ptr).spell_flags[2 as ::core::ffi::c_int as usize] as uint32_t;
    k = RSF_NONE as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while k < RSF_MAX as ::core::ffi::c_int {
        if flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            k,
        ) {
            let fresh1 = num;
            num = num + 1;
            (*kill).spell[fresh1 as usize] = k as uint8_t;
        }
        k += 1;
    }
    (*kill).ranged_attack = num as uint8_t;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        *borg_race_death.offset(i as isize) = 1 as int16_t;
    }
    if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
        j = ((*kill).pos.y - borg_panel_hgt() / 2 as ::core::ffi::c_int) / borg_panel_hgt()
            * borg_panel_hgt();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_HGT
            - ((*Term).hgt
                - row_top_map[(*Term).sidebar_mode as usize]
                - row_bottom_map[(*Term).sidebar_mode as usize])
                / tile_height as ::core::ffi::c_int
        {
            j = DUNGEON_HGT
                - ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int;
        }
        borg.mon.morgoth_panel.y = j;
        j = ((*kill).pos.x - borg_panel_wid() / 2 as ::core::ffi::c_int) / borg_panel_wid()
            * borg_panel_wid();
        if j < 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
        }
        if j > DUNGEON_WID
            - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            j = DUNGEON_WID
                - ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                    / tile_width as ::core::ffi::c_int;
        }
        borg.mon.morgoth_panel.x = j;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat =
            FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat
        as ::core::ffi::c_int
        != FEAT_FLOOR as ::core::ffi::c_int
        && flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_PASS_WALL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).feat =
            FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_delete_kill(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    borg_note(format(
        b"# Forgetting a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
    ));
    if borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
        && borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.y
        && borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.x
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).kill = 0 as uint8_t;
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        borg.time.last_kill_mult = borg.time.now;
    }
    memset(
        kill as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<borg_kill>() as size_t,
    );
    borg_kills_cnt -= 1;
    borg_danger_wipe = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_sleep_kill(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    borg_note(format(
        b"# Noting sleep on a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
    ));
    (*kill).awake = false_0 != 0;
    borg.goal.type_0 = 0 as int16_t;
    borg_danger_wipe = true_0 != 0;
}
unsafe extern "C" fn borg_follow_kill_aux(
    mut i: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut d: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    d = distance(borg.c, loc(x, y));
    if d > (*z_info).max_sight as ::core::ffi::c_int {
        return false_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
        return false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0 {
        if (*ag).info as ::core::ffi::c_int & (BORG_LIGHT | BORG_GLOW) != 0 {
            if *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0
                || borg.see_inv as ::core::ffi::c_int != 0
            {
                return true_0 != 0;
            }
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_INVISIBLE as ::core::ffi::c_int,
            ) {
                return true_0 != 0;
            }
        }
        if d <= *borg.trait_0.offset(BI_INFRA as ::core::ffi::c_int as isize) {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_COLD_BLOOD as ::core::ffi::c_int,
            ) {
                return true_0 != 0;
            }
        }
    }
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0 {
        if flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_EMPTY_MIND as ::core::ffi::c_int,
        ) {
            return false_0 != 0;
        }
        if flag_has(
            &raw mut (*r_ptr).flags as *mut bitflag,
            (RF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RF_WEIRD_MIND as ::core::ffi::c_int,
        ) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_follow_kill(mut i: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut b_dx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dy: ::core::ffi::c_int = 0;
    let mut b_dy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx == 0 {
        return;
    }
    ox = (*kill).pos.x;
    oy = (*kill).pos.y;
    if !borg_follow_kill_aux(i, oy, ox) {
        return;
    }
    borg_note(format(
        b"# There was a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        oy,
        ox,
    ));
    if !borg_cave_floor_bold(oy, ox) {
        borg_delete_kill(i);
        return;
    }
    if (Rand_div(100 as uint32_t) as int32_t + 1 as int32_t) < 1 as int32_t {
        borg_delete_kill(i);
        return;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || (*kill).awake as ::core::ffi::c_int == false_0
    {
        if (*kill).pos.y == borg.c.y && (*kill).pos.x == borg.c.x {
            borg_delete_kill(i);
        }
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        dx = ddx_ddd[j as usize] as ::core::ffi::c_int;
        dy = ddy_ddd[j as usize] as ::core::ffi::c_int;
        x = ox + dx;
        y = oy + dy;
        if square_in_bounds_fully(cave, loc(x, y)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if borg_cave_floor_grid(ag) {
                if !((*ag).kill != 0) {
                    if !borg_follow_kill_aux(i, y, x) {
                        b_dx += dx;
                        b_dy += dy;
                    }
                }
            }
        }
        j += 1;
    }
    if b_dx < -(1 as ::core::ffi::c_int) {
        b_dx = -(1 as ::core::ffi::c_int);
    } else if b_dx > 1 as ::core::ffi::c_int {
        b_dx = 1 as ::core::ffi::c_int;
    }
    if b_dy < -(1 as ::core::ffi::c_int) {
        b_dy = -(1 as ::core::ffi::c_int);
    } else if b_dy > 1 as ::core::ffi::c_int {
        b_dy = 1 as ::core::ffi::c_int;
    }
    x = ox + b_dx;
    y = oy + b_dy;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    if !borg_cave_floor_grid(ag) {
        borg_delete_kill(i);
        return;
    }
    if (*ag).kill as ::core::ffi::c_int != i {
        borg_delete_kill(i);
        return;
    }
    if (*kill).pos.y == oy && (*kill).pos.x == ox {
        borg_delete_kill(i);
        return;
    }
    (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).kill = 0 as uint8_t;
    (*kill).ox = ox as uint8_t;
    (*kill).oy = oy as uint8_t;
    (*kill).pos.x = ox + b_dx;
    (*kill).pos.y = oy + b_dy;
    (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).kill = i as uint8_t;
    borg_note(format(
        b"# Following a monster '%s' to (%d,%d) from (%d,%d)\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
        oy,
        ox,
    ));
    borg_danger_wipe = true_0 != 0;
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) == 0
        && borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
        && (borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.y
            && borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == (*kill).pos.x)
        || borg.goal.type_0 as ::core::ffi::c_int == GOAL_TAKE
            && borg.munchkin_mode as ::core::ffi::c_int != 0
    {
        borg.goal.type_0 = 0 as int16_t;
    }
}
unsafe extern "C" fn borg_new_kill(
    mut r_idx: ::core::ffi::c_uint,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut m_ptr: *mut monster = ::core::ptr::null_mut::<monster>();
    i = 1 as ::core::ffi::c_int;
    while n < 0 as ::core::ffi::c_int && i < borg_kills_nxt as ::core::ffi::c_int {
        if (*borg_kills.offset(i as isize)).r_idx == 0 {
            n = i;
        }
        i += 1;
    }
    if n < 0 as ::core::ffi::c_int
        && (borg_kills_nxt as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
    {
        let fresh0 = borg_kills_nxt;
        borg_kills_nxt = borg_kills_nxt + 1;
        n = fresh0 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        borg_note(b"# Too many monsters\0" as *const u8 as *const ::core::ffi::c_char);
        n = (Rand_div((borg_kills_nxt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t)
            as int32_t
            + 1 as int32_t
            + 1 as int32_t) as ::core::ffi::c_int;
        borg_delete_kill(n);
    }
    m_ptr = square_monster(cave, loc(x, y));
    if m_ptr.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    borg_kills_cnt += 1;
    kill = borg_kills.offset(n as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    (*kill).r_idx = r_idx as uint16_t;
    (*kill).pos.x = x;
    (*kill).ox = (*kill).pos.x as uint8_t;
    (*kill).pos.y = y;
    (*kill).oy = (*kill).pos.y as uint8_t;
    (*kill).m_idx = (*m_ptr).midx as int16_t;
    (*borg_grids[(*kill).pos.y as usize].offset((*kill).pos.x as isize)).kill = n as uint8_t;
    (*kill).when = borg.time.now;
    if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
        borg.time.morgoth = borg.time.now;
    }
    borg_update_kill_new(n);
    borg_update_kill_old(n);
    borg_note(format(
        b"# Creating a monster '%s' at (%d,%d), HP: %d, Index: %d\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        (*kill).pos.y,
        (*kill).pos.x,
        (*kill).power as ::core::ffi::c_int,
        (*kill).r_idx as ::core::ffi::c_int,
    ));
    borg_danger_wipe = true_0 != 0;
    if borg.time.now - borg.need_see_invis > 45 as borg_time {
        let mut y0: ::core::ffi::c_int = 0;
        let mut x0: ::core::ffi::c_int = 0;
        let mut y1: ::core::ffi::c_int = 0;
        let mut x1: ::core::ffi::c_int = 0;
        let mut y2: ::core::ffi::c_int = 0;
        let mut x2: ::core::ffi::c_int = 0;
        y0 = borg.c.y / 11 as ::core::ffi::c_int;
        x0 = borg.c.x / 11 as ::core::ffi::c_int;
        y1 = if y0 > 0 as ::core::ffi::c_int {
            y0 - 1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        x1 = if x0 > 0 as ::core::ffi::c_int {
            x0 - 1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y2 = if y0 < 5 as ::core::ffi::c_int {
            y0 + 1 as ::core::ffi::c_int
        } else {
            5 as ::core::ffi::c_int
        };
        x2 = if x0 < 17 as ::core::ffi::c_int {
            x0 + 1 as ::core::ffi::c_int
        } else {
            17 as ::core::ffi::c_int
        };
        borg_fear_region[y0 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y0 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y0 as usize][x2 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x0 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y1 as usize][x2 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x1 as usize] = 0 as uint16_t;
        borg_fear_region[y2 as usize][x2 as usize] = 0 as uint16_t;
        borg_note(format(
            b"# Removing Regional Fear (%d,%d) because of a LOS %s\0" as *const u8
                as *const ::core::ffi::c_char,
            y,
            x,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        ));
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0
        && borg_los((*kill).pos.y, (*kill).pos.x, borg.c.y, borg.c.x) as ::core::ffi::c_int != 0
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    if !flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        (*ag).feat = FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_nasties_num as ::core::ffi::c_int {
        if (*r_info.offset((*kill).r_idx as isize)).d_char == borg_nasties[i as usize] as wchar_t {
            borg_nasties_count[i as usize] = borg_nasties_count[i as usize].wrapping_add(1);
        }
        i += 1;
    }
    return n;
}
unsafe extern "C" fn borg_guess_race(
    mut a: uint8_t,
    mut c: wchar_t,
    mut multi: bool,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut m_ptr: *mut monster = ::core::ptr::null_mut::<monster>();
    m_ptr = square_monster(cave, loc(x, y));
    if m_ptr.is_null() {
        return 0 as ::core::ffi::c_uint;
    }
    return (*(*m_ptr).race).ridx;
}
#[no_mangle]
pub unsafe extern "C" fn observe_kill_diff(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    r_idx = borg_guess_race(a, c, false_0 != 0, y, x);
    if r_idx == 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    i = borg_new_kill(r_idx, y, x);
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    (*kill).when = borg.time.now;
    if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
        borg.time.morgoth = borg.time.now;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn observe_kill_move(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut a: uint8_t,
    mut c: wchar_t,
    mut flag: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut ox: ::core::ffi::c_int = 0;
    let mut oy: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut flicker: bool = false_0 != 0;
    let mut current_block_35: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(*kill).seen {
                ox = (*kill).pos.x;
                oy = (*kill).pos.y;
                z = borg_distance(oy, ox, y, x);
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                if flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_UNIQUE as ::core::ffi::c_int,
                ) {
                    d = d * 2 as ::core::ffi::c_int;
                }
                if !(z > d) {
                    if !((*kill).m_idx as ::core::ffi::c_int
                        != (*square_monster(cave, loc(x, y))).midx)
                    {
                        if !(!flag
                            && z > (*kill).moves as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                        {
                            if !(*borg
                                .trait_0
                                .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                                == 0
                                && c != (*r_ptr).d_char)
                            {
                                if a as ::core::ffi::c_int != (*r_ptr).d_attr as ::core::ffi::c_int
                                    || *borg
                                        .trait_0
                                        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                                        != 0
                                {
                                    if !flag_has(
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
                                        RF_ATTR_MULTI as ::core::ffi::c_int,
                                    ) && !flag_has(
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
                                        RF_ATTR_CLEAR as ::core::ffi::c_int,
                                    ) {
                                        if !flag {
                                            current_block_35 = 14916268686031723178;
                                        } else if (*kill).known {
                                            current_block_35 = 14916268686031723178;
                                        } else {
                                            r_idx = borg_guess_race(a, c, true_0 != 0, y, x);
                                            if r_idx == 0 {
                                                current_block_35 = 14916268686031723178;
                                            } else {
                                                borg_note(format(
                                                    b"# Flickering monster '%s' at (%d,%d)\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    (*r_info.offset(r_idx as isize)).name,
                                                    y,
                                                    x,
                                                ));
                                                borg_note(format(
                                                    b"# Converting a monster '%s' at (%d,%d)\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    borg_race_name(
                                                        (*kill).r_idx as ::core::ffi::c_int,
                                                    ),
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                ));
                                                (*kill).r_idx = r_idx as uint16_t;
                                                flicker = true_0 != 0;
                                                borg_danger_wipe = true_0 != 0;
                                                borg.goal.type_0 = 0 as int16_t;
                                                current_block_35 = 10043043949733653460;
                                            }
                                        }
                                    } else {
                                        current_block_35 = 10043043949733653460;
                                    }
                                } else {
                                    current_block_35 = 10043043949733653460;
                                }
                                match current_block_35 {
                                    14916268686031723178 => {}
                                    _ => {
                                        if z != 0 {
                                            (*borg_grids[(*kill).pos.y as usize]
                                                .offset((*kill).pos.x as isize))
                                            .kill = 0 as uint8_t;
                                            (*kill).ox = (*kill).pos.x as uint8_t;
                                            (*kill).oy = (*kill).pos.y as uint8_t;
                                            (*kill).pos.x = x;
                                            (*kill).pos.y = y;
                                            (*borg_grids[(*kill).pos.y as usize]
                                                .offset((*kill).pos.x as isize))
                                            .kill = i as uint8_t;
                                            borg_note(
                                                format(
                                                    b"# Tracking a monster '%s' at (%d,%d) from (%d,%d)\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                    oy,
                                                    ox,
                                                ),
                                            );
                                            borg_danger_wipe = true_0 != 0;
                                            if *borg
                                                .trait_0
                                                .offset(BI_ESP as ::core::ffi::c_int as isize)
                                                == 0
                                                && borg.goal.type_0 as ::core::ffi::c_int
                                                    == GOAL_KILL
                                                && (borg_flow_y[0 as ::core::ffi::c_int as usize]
                                                    as ::core::ffi::c_int
                                                    == (*kill).pos.y
                                                    && borg_flow_x[0 as ::core::ffi::c_int as usize]
                                                        as ::core::ffi::c_int
                                                        == (*kill).pos.x)
                                                || borg.goal.type_0 as ::core::ffi::c_int
                                                    == GOAL_TAKE
                                                    && borg.munchkin_mode as ::core::ffi::c_int != 0
                                            {
                                                borg.goal.type_0 = 0 as int16_t;
                                            }
                                        }
                                        (*kill).when = borg.time.now;
                                        if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
                                            borg.time.morgoth = borg.time.now;
                                        }
                                        if flicker {
                                            borg_update_kill_new(i);
                                        }
                                        borg_update_kill_old(i);
                                        (*kill).seen = true_0 != 0;
                                        return true_0 != 0;
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
    return false_0 != 0;
}
unsafe extern "C" fn borg_guess_race_name(
    mut who: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_int = 0;
    let mut b_s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut partial: [::core::ffi::c_char; 160] = [0; 160];
    let mut len: ::core::ffi::c_int = strlen(who) as ::core::ffi::c_int;
    m = 0 as ::core::ffi::c_int;
    n = borg_unique_size;
    while m < n - 1 as ::core::ffi::c_int {
        i = (m + n) / 2 as ::core::ffi::c_int;
        if strcmp(*borg_unique_text.offset(i as isize), who) <= 0 as ::core::ffi::c_int {
            m = i;
        } else {
            n = i;
        }
    }
    if strcmp(who, *borg_unique_text.offset(m as isize)) == 0 {
        return *borg_unique_what.offset(m as isize);
    }
    if suffix(
        who,
        b" (offscreen)\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        my_strcpy(
            &raw mut partial as *mut ::core::ffi::c_char,
            who,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>() as size_t,
        );
        partial[(len - 12 as ::core::ffi::c_int) as usize] = '\0' as i32 as ::core::ffi::c_char;
        who = &raw mut partial as *mut ::core::ffi::c_char;
        borg_note(format(
            b"# Handling offscreen monster (%s)\0" as *const u8 as *const ::core::ffi::c_char,
            who,
        ));
    }
    if !prefix(who, b"The \0" as *const u8 as *const ::core::ffi::c_char)
        && !prefix(who, b"the \0" as *const u8 as *const ::core::ffi::c_char)
    {
        borg_note(format(
            b"# Assuming player ghost (%s) (a)\0" as *const u8 as *const ::core::ffi::c_char,
            who,
        ));
        return ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    }
    who = who.offset(4 as ::core::ffi::c_int as isize);
    m = 0 as ::core::ffi::c_int;
    n = borg_normal_size;
    while m < n - 1 as ::core::ffi::c_int {
        i = (m + n) / 2 as ::core::ffi::c_int;
        if strcmp(*borg_normal_text.offset(i as isize), who) <= 0 as ::core::ffi::c_int {
            m = i;
        } else {
            n = i;
        }
    }
    k = m;
    while k >= 0 as ::core::ffi::c_int {
        if strcmp(who, *borg_normal_text.offset(k as isize)) != 0 {
            break;
        }
        i = *borg_normal_what.offset(k as isize) as ::core::ffi::c_int;
        r_ptr = r_info.offset(i as isize) as *mut monster_race;
        s = 1000 as ::core::ffi::c_int;
        s = s
            - (if (*r_ptr).level
                - *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                < 0 as ::core::ffi::c_int
            {
                -((*r_ptr).level
                    - *borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize))
            } else {
                (*r_ptr).level
                    - *borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            });
        if !(b_i != 0 && s < b_s) {
            b_i = i;
            b_s = s;
        }
        k -= 1;
    }
    if b_i != 0 {
        return b_i as ::core::ffi::c_uint;
    }
    borg_note(format(
        b"# Assuming player ghost (%s)(b)\0" as *const u8 as *const ::core::ffi::c_char,
        who,
    ));
    return ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn borg_locate_kill(
    mut who: *mut ::core::ffi::c_char,
    mut c: loc,
    mut r: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut r_idx: ::core::ffi::c_uint = 0;
    let mut b_i: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = 0;
    let mut take: *mut borg_take = ::core::ptr::null_mut::<borg_take>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut k_ptr: *mut object_kind = ::core::ptr::null_mut::<object_kind>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if my_strnicmp(
        who,
        b"Something\0" as *const u8 as *const ::core::ffi::c_char,
        9 as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
        || my_strnicmp(
            who,
            b"It\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
    {
        borg_note(b"# Invisible monster nearby.\0" as *const u8 as *const ::core::ffi::c_char);
        if borg.time.now - borg.need_see_invis > 50 as borg_time {
            borg.need_see_invis = borg.time.now;
        }
        return 0 as ::core::ffi::c_int;
    }
    if suffix(
        who,
        b" (offscreen)\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_note(b"# Offscreen monster nearby\0" as *const u8 as *const ::core::ffi::c_char);
        borg.need_shift_panel = true_0 != 0;
        return 0 as ::core::ffi::c_int;
    }
    r_idx = borg_guess_race_name(who);
    r_ptr = r_info.offset(r_idx as isize) as *mut monster_race;
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        borg_note(format(
            b"# There is a monster '%s' within %d grids of %d,%d\0" as *const u8
                as *const ::core::ffi::c_char,
            borg_race_name(r_idx as ::core::ffi::c_int),
            r,
            c.y,
            c.x,
        ));
    }
    if (*borg_race_count.offset(r_idx as isize) as ::core::ffi::c_int) < SHRT_MAX {
        let ref mut fresh3 = *borg_race_count.offset(r_idx as isize);
        *fresh3 += 1;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_CHAR_CLEAR as ::core::ffi::c_int,
    ) {
        borg_note(b"# Bizarre monster nearby\0" as *const u8 as *const ::core::ffi::c_char);
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    let mut current_block_29: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            k_ptr = k_info.offset((*take).kidx as isize) as *mut object_kind;
            if !((*k_ptr).d_char != (*r_ptr).d_char) {
                if !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_ATTR_MULTI as ::core::ffi::c_int,
                ) && !flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_ATTR_CLEAR as ::core::ffi::c_int,
                ) {
                    if (*k_ptr).flavor.is_null() {
                        if (*k_ptr).d_attr as ::core::ffi::c_int
                            != (*r_ptr).d_attr as ::core::ffi::c_int
                        {
                            current_block_29 = 26972500619410423;
                        } else {
                            current_block_29 = 3437258052017859086;
                        }
                    } else {
                        current_block_29 = 3437258052017859086;
                    }
                } else {
                    current_block_29 = 3437258052017859086;
                }
                match current_block_29 {
                    26972500619410423 => {}
                    _ => {
                        d = distance(
                            loc(
                                (*take).x as ::core::ffi::c_int,
                                (*take).y as ::core::ffi::c_int,
                            ),
                            c,
                        );
                        if !(d > r) {
                            if !(d > b_d) {
                                b_i = i;
                                b_d = d;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i >= 0 as ::core::ffi::c_int {
        take = borg_takes.offset(b_i as isize) as *mut borg_take;
        borg_note(format(
            b"# Converting an object '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            (*k_info.offset((*take).kidx as isize)).name,
            (*take).y as ::core::ffi::c_int,
            (*take).x as ::core::ffi::c_int,
        ));
        c.x = (*take).x as ::core::ffi::c_int;
        c.y = (*take).y as ::core::ffi::c_int;
        borg_delete_take(b_i);
        b_i = borg_new_kill(r_idx, c.y, c.x);
        if b_i < 0 as ::core::ffi::c_int {
            return b_i;
        }
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        (*kill).when = borg.time.now;
        if (*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth {
            borg.time.morgoth = borg.time.now;
        }
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        return b_i;
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    let mut current_block_54: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint == r_idx) {
                let mut m_ptr: *mut monster =
                    (*cave).monsters.offset((*kill).m_idx as isize) as *mut monster;
                if !((*m_ptr).grid.x != (*kill).pos.x || (*m_ptr).grid.y != (*kill).pos.y) {
                    if !((*r_info.offset((*kill).r_idx as isize)).d_char != (*r_ptr).d_char) {
                        if !flag_has(
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
                            RF_ATTR_MULTI as ::core::ffi::c_int,
                        ) && !flag_has(
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
                            RF_ATTR_CLEAR as ::core::ffi::c_int,
                        ) {
                            if (*r_info.offset((*kill).r_idx as isize)).d_attr as ::core::ffi::c_int
                                != (*r_ptr).d_attr as ::core::ffi::c_int
                            {
                                current_block_54 = 1345366029464561491;
                            } else {
                                current_block_54 = 9353995356876505083;
                            }
                        } else {
                            current_block_54 = 9353995356876505083;
                        }
                        match current_block_54 {
                            1345366029464561491 => {}
                            _ => {
                                d = distance((*kill).pos, c);
                                if !(d > r) {
                                    if !(d > b_d) {
                                        b_i = i;
                                        b_d = d;
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
    if b_i >= 0 as ::core::ffi::c_int {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        borg_note(format(
            b"# Converting a monster '%s' at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_race_name((*kill).r_idx as ::core::ffi::c_int),
            (*kill).pos.y,
            (*kill).pos.x,
        ));
        (*kill).r_idx = r_idx as uint16_t;
        borg_update_kill_new(b_i);
        borg_update_kill_old(b_i);
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        borg_danger_wipe = true_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        return b_i;
    }
    b_i = -(1 as ::core::ffi::c_int);
    b_d = 999 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint != r_idx) {
                d = distance((*kill).pos, c);
                if !(d > r + 3 as ::core::ffi::c_int) {
                    if borg_projectable((*kill).pos.y, (*kill).pos.x, c.y, c.x) {
                        if !(d > b_d) {
                            b_i = i;
                            b_d = d;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        b_i = -(1 as ::core::ffi::c_int);
        b_d = 999 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
                if !((*kill).r_idx as ::core::ffi::c_uint != r_idx) {
                    d = distance((*kill).pos, c);
                    if !(d > r + 20 as ::core::ffi::c_int) {
                        if !(d > b_d) {
                            b_i = i;
                            b_d = d;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if b_i >= 0 as ::core::ffi::c_int {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
            borg_note(format(
                b"# Matched a monster '%s' at (%d,%d) for the parsed msg.\0" as *const u8
                    as *const ::core::ffi::c_char,
                borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                (*kill).pos.y,
                (*kill).pos.x,
            ));
        }
        if r == 0 {
            (*kill).known = true_0 != 0;
        }
        return b_i;
    }
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        borg_note(format(
            b"# Unable to locate monster '%s' near (%d,%d), which generated the msg (%s).\0"
                as *const u8 as *const ::core::ffi::c_char,
            borg_race_name(r_idx as ::core::ffi::c_int),
            c.y,
            c.x,
            who,
        ));
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_count_death(mut i: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx != 0 {
        if (*borg_race_death.offset((*kill).r_idx as isize) as ::core::ffi::c_int) < SHRT_MAX {
            let ref mut fresh4 = *borg_race_death.offset((*kill).r_idx as isize);
            *fresh4 += 1;
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
        ) {
            borg.mon.unique = 0 as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut borg_in_hall: bool = false_0 != 0;
    let mut hall_y: ::core::ffi::c_int = 0;
    let mut hall_x: ::core::ffi::c_int = 0;
    let mut hall_walls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skip_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER)
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < (if *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 0
            {
                35 as ::core::ffi::c_int
            } else {
                25 as ::core::ffi::c_int
            })
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg.morgoth_position {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    hall_x = -(1 as ::core::ffi::c_int);
    while hall_x <= 1 as ::core::ffi::c_int {
        hall_y = -(1 as ::core::ffi::c_int);
        while hall_y <= 1 as ::core::ffi::c_int {
            x = hall_x + borg.c.x;
            y = hall_y + borg.c.y;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if (*ag).glyph as ::core::ffi::c_int != 0
                || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                    && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
            {
                hall_walls += 1;
            }
            if hall_walls >= 5 as ::core::ffi::c_int {
                borg_in_hall = true_0 != 0;
            }
            hall_y += 1;
        }
        hall_x += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j < j && b_j != -(1 as ::core::ffi::c_int)) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    let mut current_block_60: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        if !((*kill).r_idx == 0) {
            ax = if x9 > borg.c.x {
                x9 - borg.c.x
            } else {
                borg.c.x - x9
            };
            ay = if y9 > borg.c.y {
                y9 - borg.c.y
            } else {
                borg.c.y - y9
            };
            d = if ax < ay { ay } else { ax };
            if !(d == 1 as ::core::ffi::c_int
                && (*borg
                    .trait_0
                    .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                    != 0
                    || *borg
                        .trait_0
                        .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
                        != 0))
            {
                if !(borg.goal.ignoring as ::core::ffi::c_int != 0
                    && *borg
                        .trait_0
                        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                        == 0
                    && flag_has(
                        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
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
                        != 0)
                {
                    if !(*borg
                        .trait_0
                        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                        < 10 as ::core::ffi::c_int
                        && flag_has(
                            &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
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
                        ) as ::core::ffi::c_int
                            != 0)
                    {
                        if !borg.mon.scary {
                            if !(*borg
                                .trait_0
                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                < 10 as ::core::ffi::c_int
                                && flag_has(
                                    &raw mut (*r_info.offset((*kill).r_idx as isize)).flags
                                        as *mut bitflag,
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
                                    != 0)
                            {
                                if !(flag_has(
                                    &raw mut (*r_info.offset((*kill).r_idx as isize)).flags
                                        as *mut bitflag,
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
                                        < 5 as ::core::ffi::c_int)
                                {
                                    x = (*kill).pos.x;
                                    y = (*kill).pos.y;
                                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                        .offset(y as isize))
                                    .offset(x as isize)
                                        as *mut borg_grid;
                                    if !(viewable as ::core::ffi::c_int != 0
                                        && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
                                    {
                                        p = borg_danger(
                                            y,
                                            x,
                                            1 as ::core::ffi::c_int,
                                            true_0 != 0,
                                            false_0 != 0,
                                        );
                                        if !(*borg
                                            .trait_0
                                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                            > 25 as ::core::ffi::c_int
                                            && !flag_has(
                                                &raw mut (*r_info.offset((*kill).r_idx as isize))
                                                    .flags
                                                    as *mut bitflag,
                                                (RF_MAX as ::core::ffi::c_int as size_t)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    )
                                                    .wrapping_sub(1 as size_t)
                                                    .wrapping_div(
                                                        (::core::mem::size_of::<bitflag>()
                                                            as size_t)
                                                            .wrapping_mul(8 as size_t),
                                                    ),
                                                RF_UNIQUE as ::core::ffi::c_int,
                                            )
                                            && p > borg.avoidance as ::core::ffi::c_int
                                                / 2 as ::core::ffi::c_int)
                                        {
                                            if !(*borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                <= 15 as ::core::ffi::c_int
                                                && p > borg.avoidance as ::core::ffi::c_int
                                                    / 3 as ::core::ffi::c_int)
                                            {
                                                if b_stair != -(1 as ::core::ffi::c_int)
                                                    && *borg.trait_0.offset(
                                                        BI_CLEVEL as ::core::ffi::c_int as isize,
                                                    ) < 10 as ::core::ffi::c_int
                                                {
                                                    j = borg_distance(
                                                        *track_less.y.offset(b_stair as isize),
                                                        *track_less.x.offset(b_stair as isize),
                                                        y,
                                                        x,
                                                    );
                                                    if b_j
                                                        <= *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) * 5 as ::core::ffi::c_int
                                                            + 9 as ::core::ffi::c_int
                                                        && j >= *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) * 5 as ::core::ffi::c_int
                                                            + 9 as ::core::ffi::c_int
                                                    {
                                                        current_block_60 = 4775909272756257391;
                                                    } else {
                                                        current_block_60 = 16738040538446813684;
                                                    }
                                                } else {
                                                    current_block_60 = 16738040538446813684;
                                                }
                                                match current_block_60 {
                                                    4775909272756257391 => {}
                                                    _ => {
                                                        if borg_in_hall as ::core::ffi::c_int != 0
                                                            && flag_has(
                                                                &raw mut (*r_info
                                                                    .offset((*kill).r_idx as isize))
                                                                .flags
                                                                    as *mut bitflag,
                                                                (RF_MAX as ::core::ffi::c_int
                                                                    as size_t)
                                                                    .wrapping_add(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    )
                                                                    .wrapping_sub(1 as size_t)
                                                                    .wrapping_div(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    ),
                                                                RF_GROUP_AI as ::core::ffi::c_int,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            hall_x = -(1 as ::core::ffi::c_int);
                                                            while hall_x <= 1 as ::core::ffi::c_int
                                                            {
                                                                hall_y = -(1 as ::core::ffi::c_int);
                                                                while hall_y
                                                                    <= 1 as ::core::ffi::c_int
                                                                {
                                                                    if square_in_bounds_fully(
                                                                        cave,
                                                                        loc(hall_x + x, hall_y + y),
                                                                    ) {
                                                                        ag = (*(&raw mut borg_grids
                                                                            as *mut *mut borg_grid)
                                                                            .offset(
                                                                                (hall_y + y)
                                                                                    as isize,
                                                                            ))
                                                                        .offset(
                                                                            (hall_x + x) as isize,
                                                                        )
                                                                            as *mut borg_grid;
                                                                        if (*ag).glyph as ::core::ffi::c_int != 0
                                                                            || (*ag).feat as ::core::ffi::c_int
                                                                                >= FEAT_MAGMA as ::core::ffi::c_int
                                                                                && (*ag).feat as ::core::ffi::c_int
                                                                                    <= FEAT_PERM as ::core::ffi::c_int
                                                                        {
                                                                            hall_walls += 1;
                                                                        }
                                                                        if hall_walls < 4 as ::core::ffi::c_int {
                                                                            skip_monster = true_0 != 0;
                                                                        }
                                                                    }
                                                                    hall_y += 1;
                                                                }
                                                                hall_x += 1;
                                                            }
                                                        }
                                                        if d == 2 as ::core::ffi::c_int
                                                            && (*kill).ranged_attack == 0
                                                            && !flag_has(
                                                                &raw mut (*r_info
                                                                    .offset((*kill).r_idx as isize))
                                                                .flags
                                                                    as *mut bitflag,
                                                                (RF_MAX as ::core::ffi::c_int
                                                                    as size_t)
                                                                    .wrapping_add(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    )
                                                                    .wrapping_sub(1 as size_t)
                                                                    .wrapping_div(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    ),
                                                                RF_NEVER_MOVE as ::core::ffi::c_int,
                                                            )
                                                        {
                                                            skip_monster = true_0 != 0;
                                                        }
                                                        if !skip_monster {
                                                            if !borg_flow_far_from_stairs(
                                                                x, y, b_stair,
                                                            ) {
                                                                borg_temp_x[borg_temp_n as usize] =
                                                                    x as uint8_t;
                                                                borg_temp_y[borg_temp_n as usize] =
                                                                    y as uint8_t;
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
        b"kill\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_KILL,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_KILL) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_has_distance_attack() -> bool {
    if borg_attack_aux_spell_bolt(
        MAGIC_MISSILE,
        0 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        BORG_ATTACK_MISSILE as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        false_0 != 0,
    ) > 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if borg_attack_aux_spell_bolt(
        NETHER_BOLT,
        0 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        BORG_ATTACK_NETHER as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        false_0 != 0,
    ) > 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if borg_attack_aux_launch() > 0 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_aim(mut viewable: bool) -> bool {
    let mut o_y: ::core::ffi::c_int = 0;
    let mut o_x: ::core::ffi::c_int = 0;
    let mut s_c_y: ::core::ffi::c_int = borg.c.y;
    let mut s_c_x: ::core::ffi::c_int = borg.c.x;
    let mut i: ::core::ffi::c_int = 0;
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_has_distance_attack() {
        return false_0 != 0;
    }
    o_x = -(2 as ::core::ffi::c_int);
    while o_x <= 2 as ::core::ffi::c_int {
        o_y = -(2 as ::core::ffi::c_int);
        while o_y <= 2 as ::core::ffi::c_int {
            if !(o_x == 0 as ::core::ffi::c_int && o_y == 0 as ::core::ffi::c_int) {
                borg.c.x = s_c_x + o_x;
                borg.c.y = s_c_y + o_y;
                if !(borg.c.x > AUTO_MAX_X - 2 as ::core::ffi::c_int
                    || borg.c.x < 2 as ::core::ffi::c_int
                    || borg.c.y > AUTO_MAX_Y - 2 as ::core::ffi::c_int
                    || borg.c.y < 2 as ::core::ffi::c_int)
                {
                    i = 0 as ::core::ffi::c_int;
                    while i < borg_kills_nxt as ::core::ffi::c_int {
                        if distance(borg.c, (*borg_kills.offset(i as isize)).pos)
                            == 1 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    if !(i != borg_kills_nxt as ::core::ffi::c_int) {
                        if borg_has_distance_attack() {
                            borg_flow_clear();
                            borg_flow_enqueue_grid(borg.c.y, borg.c.x);
                            borg.c.x = s_c_x;
                            borg.c.y = s_c_y;
                            borg_flow_spread(
                                5 as ::core::ffi::c_int,
                                true_0 != 0,
                                !viewable,
                                false_0 != 0,
                                -(1 as ::core::ffi::c_int),
                                false_0 != 0,
                            );
                            if !borg_flow_commit(
                                b"targetable position\0" as *const u8 as *const ::core::ffi::c_char,
                                GOAL_KILL,
                            ) {
                                return false_0 != 0;
                            }
                            if !borg_flow_old(GOAL_KILL) {
                                return false_0 != 0;
                            }
                            return true_0 != 0;
                        }
                    }
                }
            }
            o_y += 1;
        }
        o_x += 1;
    }
    borg.c.x = s_c_x;
    borg.c.y = s_c_y;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_corridor() -> bool {
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_distance: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_n: bool = false_0 != 0;
    let mut b_s: bool = false_0 != 0;
    let mut b_e: bool = false_0 != 0;
    let mut b_w: bool = false_0 != 0;
    let mut n_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut ny: [::core::ffi::c_int; 25] = [
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut nx: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut s_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut sy: [::core::ffi::c_int; 25] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    ];
    let mut sx: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut e_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut ey: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut ex: [::core::ffi::c_int; 25] = [
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    ];
    let mut w_array: [::core::ffi::c_int; 25] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut wy: [::core::ffi::c_int; 25] = [
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ];
    let mut wx: [::core::ffi::c_int; 25] = [
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        -(4 as ::core::ffi::c_int),
        -(3 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
    ];
    let mut wall_north: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_south: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_east: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wall_west: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    borg.status.digging = false_0 != 0;
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if borg.near.summoner_idx as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    if borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg.morgoth_position {
        return false_0 != 0;
    }
    if borg.status.anti_summon {
        return false_0 != 0;
    }
    kill = borg_kills.offset(borg.near.summoner_idx as isize) as *mut borg_kill;
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NEVER_MOVE as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if flag_has(
        &raw mut (*r_info.offset((*kill).r_idx as isize)).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_KILL_WALL as ::core::ffi::c_int,
    ) {
        return false_0 != 0;
    }
    if !(*kill).awake {
        return false_0 != 0;
    }
    if !borg_spell_okay(TURN_STONE_TO_MUD)
        && !borg_spell_okay(SHATTER_STONE)
        && !borg_equips_ring(sv_ring_digging)
        && !borg_equips_item(act_stone_to_mud, true_0 != 0)
    {
        return false_0 != 0;
    }
    if !borg_los((*kill).pos.y, (*kill).pos.x, borg.c.y, borg.c.x) {
        q_x = w_x / borg_panel_wid();
        q_y = w_y / borg_panel_hgt();
        if *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == true_0
            && *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == true_0
        {
            borg_flow_clear();
            borg.status.digging = true_0 != 0;
            borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
            borg_flow_spread(
                10 as ::core::ffi::c_int,
                true_0 != 0,
                false_0 != 0,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
                false_0 != 0,
            );
            if !borg_flow_commit(
                b"Monster Path\0" as *const u8 as *const ::core::ffi::c_char,
                GOAL_KILL,
            ) {
                return false_0 != 0;
            }
        } else {
            borg_flow_clear();
            borg.status.digging = true_0 != 0;
            borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
            borg_flow_spread(
                10 as ::core::ffi::c_int,
                true_0 != 0,
                true_0 != 0,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
                false_0 != 0,
            );
            if !borg_flow_commit(
                b"Monster Path\0" as *const u8 as *const ::core::ffi::c_char,
                GOAL_KILL,
            ) {
                return false_0 != 0;
            }
        }
    }
    o_y = -(2 as ::core::ffi::c_int);
    while o_y < 1 as ::core::ffi::c_int {
        wall_north = 0 as ::core::ffi::c_int;
        o_x = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = borg.c.y + o_y + ny[i as usize];
            m_x = borg.c.x + o_x + nx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(m_y as isize))
                    .offset(m_x as isize) as *mut borg_grid;
                if n_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_north += 1;
                }
                if n_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_north += 1;
                }
            }
            i += 1;
        }
        if wall_north == 25 as ::core::ffi::c_int {
            if distance(
                borg.c,
                loc(
                    borg.c.x + o_x + nx[7 as ::core::ffi::c_int as usize],
                    borg.c.y + o_y + ny[7 as ::core::ffi::c_int as usize],
                ),
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_n = true_0 != 0;
                b_distance = distance(
                    borg.c,
                    loc(
                        borg.c.x + o_x + nx[7 as ::core::ffi::c_int as usize],
                        borg.c.y + o_y + ny[7 as ::core::ffi::c_int as usize],
                    ),
                );
            }
        }
        o_y += 1;
    }
    o_y = -(1 as ::core::ffi::c_int);
    while o_y < 2 as ::core::ffi::c_int {
        wall_south = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_0: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            o_x = 0 as ::core::ffi::c_int;
            m_y = borg.c.y + o_y + sy[i as usize];
            m_x = borg.c.x + o_x + sx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_0 = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(m_y as isize))
                    .offset(m_x as isize) as *mut borg_grid;
                if s_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_0).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_0).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_south += 1;
                }
                if s_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_0).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_0).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_0).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_south += 1;
                }
            }
            i += 1;
        }
        if wall_south == 25 as ::core::ffi::c_int {
            if borg_distance(
                borg.c.y,
                borg.c.x,
                borg.c.y + o_y + sy[17 as ::core::ffi::c_int as usize],
                borg.c.x + o_x + sx[17 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_s = true_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y + b_y + sy[17 as ::core::ffi::c_int as usize],
                    borg.c.x + b_x + sx[17 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_y += 1;
    }
    o_x = -(1 as ::core::ffi::c_int);
    while o_x < 2 as ::core::ffi::c_int {
        wall_east = 0 as ::core::ffi::c_int;
        o_y = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_1: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = borg.c.y + o_y + ey[i as usize];
            m_x = borg.c.x + o_x + ex[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_1 = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(m_y as isize))
                    .offset(m_x as isize) as *mut borg_grid;
                if e_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_1).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_1).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_east += 1;
                }
                if e_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_1).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_1).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_1).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_east += 1;
                }
            }
            i += 1;
        }
        if wall_east == 25 as ::core::ffi::c_int {
            if borg_distance(
                borg.c.y,
                borg.c.x,
                borg.c.y + o_y + ey[13 as ::core::ffi::c_int as usize],
                borg.c.x + o_x + ex[13 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_e = true_0 != 0;
                b_s = false_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y + b_y + ey[13 as ::core::ffi::c_int as usize],
                    borg.c.x + b_x + ex[13 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_x += 1;
    }
    o_x = -(2 as ::core::ffi::c_int);
    while o_x < 1 as ::core::ffi::c_int {
        wall_west = 0 as ::core::ffi::c_int;
        o_y = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 25 as ::core::ffi::c_int {
            let mut ag_2: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
            m_y = borg.c.y + o_y + wy[i as usize];
            m_x = borg.c.x + o_x + wx[i as usize];
            if square_in_bounds_fully(cave, loc(m_x, m_y)) {
                ag_2 = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(m_y as isize))
                    .offset(m_x as isize) as *mut borg_grid;
                if w_array[i as usize] == 0 as ::core::ffi::c_int
                    && ((*ag_2).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_2).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_west += 1;
                }
                if w_array[i as usize] == 1 as ::core::ffi::c_int
                    && ((*ag_2).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                            && (*ag_2).feat as ::core::ffi::c_int
                                <= FEAT_QUARTZ_K as ::core::ffi::c_int
                        || (*ag_2).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int)
                {
                    wall_west += 1;
                }
            }
            i += 1;
        }
        if wall_west == 25 as ::core::ffi::c_int {
            if borg_distance(
                borg.c.y,
                borg.c.x,
                borg.c.y + o_y + wy[11 as ::core::ffi::c_int as usize],
                borg.c.x + o_x + wx[11 as ::core::ffi::c_int as usize],
            ) < b_distance
            {
                b_y = o_y;
                b_x = o_x;
                b_w = true_0 != 0;
                b_e = false_0 != 0;
                b_s = false_0 != 0;
                b_n = false_0 != 0;
                b_distance = borg_distance(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y + o_y + wy[11 as ::core::ffi::c_int as usize],
                    borg.c.x + o_x + wx[11 as ::core::ffi::c_int as usize],
                );
            }
        }
        o_x += 1;
    }
    if b_n as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            borg.c.y + b_y + ny[7 as ::core::ffi::c_int as usize],
            borg.c.x + b_x + nx[7 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor north type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_s as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            borg.c.y + b_y + sy[17 as ::core::ffi::c_int as usize],
            borg.c.x + b_x + sx[17 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            6 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor south type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_e as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            borg.c.y + b_y + ey[13 as ::core::ffi::c_int as usize],
            borg.c.x + b_x + ex[13 as ::core::ffi::c_int as usize],
        );
        borg.status.digging = true_0 != 0;
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor east type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_w as ::core::ffi::c_int == true_0 {
        borg_flow_clear();
        borg.status.digging = true_0 != 0;
        borg_flow_enqueue_grid(
            borg.c.y + b_y + wy[11 as ::core::ffi::c_int as usize],
            borg.c.x + b_x + wx[11 as ::core::ffi::c_int as usize],
        );
        borg_flow_spread(
            5 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"anti-summon corridor west type 1\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_kill_direct(mut twitchy: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_sight as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_can_dig(false_0 != 0, FEAT_GRANITE as ::core::ffi::c_int as uint8_t) {
        return false_0 != 0;
    }
    if !twitchy
        && (*borg
            .trait_0
            .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
            != 0
            || *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
            || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int)
    {
        return false_0 != 0;
    }
    if !twitchy
        && borg.time.now - borg.time.level < 3000 as borg_time
        && (borg.times_twitch as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg_kills_cnt != 0 {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
                d = distance((*kill).pos, borg.c);
                if !(d > b_d) {
                    b_i = i;
                    b_d = d;
                }
            }
            i += 1;
        }
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        borg_flow_clear();
        borg_flow_enqueue_grid(
            AUTO_MAX_Y / 2 as ::core::ffi::c_int,
            AUTO_MAX_X / 2 as ::core::ffi::c_int,
        );
        borg_flow_spread(
            150 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"center direct\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    if b_i != 0 {
        kill = borg_kills.offset(b_i as isize) as *mut borg_kill;
        borg_flow_clear();
        borg_flow_enqueue_grid((*kill).pos.y, (*kill).pos.x);
        borg_flow_spread(
            15 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            true_0 != 0,
            -(1 as ::core::ffi::c_int),
            false_0 != 0,
        );
        if !borg_flow_commit(
            b"kill direct\0" as *const u8 as *const ::core::ffi::c_char,
            GOAL_DIGGING,
        ) {
            return false_0 != 0;
        }
        if !borg_flow_old(GOAL_DIGGING) {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_near_monster_type(mut dist: ::core::ffi::c_int) {
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut x9: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut breeder_count: ::core::ffi::c_int = borg.mon.breeders as ::core::ffi::c_int;
    borg.near.summoner = false_0 != 0;
    borg.near.unique = 0 as int16_t;
    borg.near.evil_unique = false_0 != 0;
    borg.near.summoner_idx = -(1 as ::core::ffi::c_int) as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 5 as ::core::ffi::c_int
                    && (*r_ptr).ridx == borg.id.squint_eyed_rogue
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 6 as ::core::ffi::c_int
                    && (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
                        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_PRIEST)
                    && (*r_ptr).ridx == borg.id.squint_eyed_rogue
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 5 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == borg.id.grip
                        || (*r_ptr).ridx == borg.id.fang
                        || (*r_ptr).ridx == borg.id.small_kobold)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 8 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == borg.id.soldier
                        || (*r_ptr).ridx == borg.id.cutpurse
                        || (*r_ptr).ridx == borg.id.acolyte
                        || (*r_ptr).ridx == borg.id.apprentice
                        || (*r_ptr).ridx == borg.id.kobold
                        || (*r_ptr).ridx == borg.id.shrieker_mushroom_patch
                        || (*r_ptr).ridx == borg.id.maggot
                        || (*r_ptr).ridx == borg.id.filthy_street_urchin
                        || (*r_ptr).ridx == borg.id.battle_scarred_veteran
                        || (*r_ptr).ridx == borg.id.mean_looking_mercenary)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 15 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == borg.id.bullroarer
                        || ((*r_ptr).ridx == borg.id.giant_white_mouse
                            || (*r_ptr).ridx == borg.id.white_worm_mass
                            || (*r_ptr).ridx == borg.id.green_worm_mass)
                            && breeder_count
                                >= *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 20 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == borg.id.cave_spider
                        || (*r_ptr).ridx == borg.id.red_naga
                        || (*r_ptr).ridx == borg.id.giant_red_frog
                        || (*r_ptr).ridx == borg.id.radiation_eye
                        || (*r_ptr).ridx == borg.id.yellow_worm_mass
                            && breeder_count
                                >= *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 45 as ::core::ffi::c_int
                    && ((*r_ptr).ridx == borg.id.gravity_hound
                        || (*r_ptr).ridx == borg.id.inertia_hound
                        || prefix_i(
                            (*(*r_ptr).base).name,
                            b"ancient dragon\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as ::core::ffi::c_int
                            != 0
                        || (*r_ptr).ridx == borg.id.beorn
                        || (*r_ptr).ridx == borg.id.dread)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_SRNTHR as ::core::ffi::c_int as isize)
                    == 0
                    && ((*r_ptr).ridx == borg.id.osse
                        || (*r_ptr).ridx == borg.id.dracolich
                        || (*r_ptr).ridx == borg.id.dracolisk)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_SRBLIND as ::core::ffi::c_int as isize)
                    == 0
                    && ((*r_ptr).ridx == borg.id.light_hound
                        && *borg
                            .trait_0
                            .offset(BI_SRLITE as ::core::ffi::c_int as isize)
                            == 0
                        || (*r_ptr).ridx == borg.id.dark_hound
                            && *borg
                                .trait_0
                                .offset(BI_SRDARK as ::core::ffi::c_int as isize)
                                == 0)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_SRKAOS as ::core::ffi::c_int as isize)
                    == 0
                    && *borg
                        .trait_0
                        .offset(BI_SRCONF as ::core::ffi::c_int as isize)
                        == 0
                    && flag_has(
                        &raw mut (*r_ptr).spell_flags as *mut bitflag,
                        (RSF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RSF_BR_CHAO as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg
                    .trait_0
                    .offset(BI_SRCONF as ::core::ffi::c_int as isize)
                    == 0
                    && ((*r_ptr).ridx == borg.id.pukelman || (*r_ptr).ridx == borg.id.night_mare)
                {
                    borg.mon.scary = true_0 != 0;
                }
                if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) == 0
                    && (*r_ptr).ridx == borg.id.drolem
                {
                    borg.mon.scary = true_0 != 0;
                }
                x9 = (*kill).pos.x;
                y9 = (*kill).pos.y;
                ax = if x9 > borg.c.x {
                    x9 - borg.c.x
                } else {
                    borg.c.x - x9
                };
                ay = if y9 > borg.c.y {
                    y9 - borg.c.y
                } else {
                    borg.c.y - y9
                };
                d = if ax < ay { ay } else { ax };
                if !(d > dist
                    && *borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                        != 0)
                {
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
                        RF_UNIQUE as ::core::ffi::c_int,
                    ) {
                        borg.mon.unique = (*kill).r_idx as ::core::ffi::c_uint;
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
                            RF_QUESTOR as ::core::ffi::c_int,
                        ) {
                            borg.near.unique = (borg.near.unique as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                as int16_t;
                        }
                        borg.near.unique += 1;
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
                            RF_EVIL as ::core::ffi::c_int,
                        ) {
                            borg.near.evil_unique = true_0 != 0;
                        }
                    }
                    if flag_has(
                        &raw mut (*r_ptr).spell_flags as *mut bitflag,
                        (RSF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RSF_S_KIN as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_DEMON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_MONSTER as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_MONSTERS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_ANIMAL as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_SPIDER as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HOUND as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HYDRA as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_AINU as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_DEMON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_UNDEAD as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_DRAGON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_DRAGON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_WRAITH as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        || flag_has(
                            &raw mut (*r_ptr).spell_flags as *mut bitflag,
                            (RSF_MAX as ::core::ffi::c_int as size_t)
                                .wrapping_add(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                )
                                .wrapping_sub(1 as size_t)
                                .wrapping_div(
                                    (::core::mem::size_of::<bitflag>() as size_t)
                                        .wrapping_mul(8 as size_t),
                                ),
                            RSF_S_UNIQUE as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                    {
                        borg.near.summoner = true_0 != 0;
                        if d < 8 as ::core::ffi::c_int {
                            borg.near.summoner_idx = i as int16_t;
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_shoot_scoot_safe(
    mut emergency: ::core::ffi::c_int,
    mut turns: ::core::ffi::c_int,
    mut b_p: ::core::ffi::c_int,
) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut dis: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = dis / 2 as ::core::ffi::c_int;
    let mut adjacent_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 8 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_APHASE as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 {
        return false_0 != 0;
    }
    if square_isvault(cave, borg.c) {
        return false_0 != 0;
    }
    if borg_primarily_caster() {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= 45 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                < 15 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 45 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                < 5 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
    } else if *borg
        .trait_0
        .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
        < 5 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= 45 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg.morgoth_position as ::core::ffi::c_int != 0
        || borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
        y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if (*ag).kill as ::core::ffi::c_int != 0
            && (*kill).awake as ::core::ffi::c_int != 0
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
                RF_NEVER_MOVE as ::core::ffi::c_int,
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
                RF_PASS_WALL as ::core::ffi::c_int,
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
                RF_KILL_WALL as ::core::ffi::c_int,
            )
            && (*kill).power as ::core::ffi::c_int
                >= *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        {
            if borg_spell_okay(MAGIC_MISSILE) {
                adjacent_monster = true_0 != 0;
            } else if borg_spell_okay(ORB_OF_DRAINING) {
                adjacent_monster = true_0 != 0;
            } else if borg_spell_okay(NETHER_BOLT) {
                adjacent_monster = true_0 != 0;
            } else if borg_danger_one_kill(
                (*kill).pos.y,
                (*kill).pos.x,
                1 as ::core::ffi::c_int,
                i,
                true_0 != 0,
                false_0 != 0,
            ) > borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
                || (!(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null())
                    && (*kill).level as ::core::ffi::c_int
                        >= *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                            - 5 as ::core::ffi::c_int
                || (*kill).ranged_attack as ::core::ffi::c_int != 0
                || flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
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
                || flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_MULTIPLY as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                || *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    <= 5 as ::core::ffi::c_int
            {
                adjacent_monster = true_0 != 0;
            }
        }
        i += 1;
    }
    if adjacent_monster as ::core::ffi::c_int == false_0 {
        return false_0 != 0;
    }
    k = 0 as ::core::ffi::c_int;
    n = k;
    while k < 100 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 100 as ::core::ffi::c_int {
            loop {
                y = (borg.c.y as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                x = (borg.c.x as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                d = distance(borg.c, loc(x, y));
                if d >= min && d <= dis {
                    break;
                }
            }
            if !(y <= 0 as ::core::ffi::c_int || y >= AUTO_MAX_Y - 2 as ::core::ffi::c_int) {
                if !(x <= 0 as ::core::ffi::c_int || x >= AUTO_MAX_X - 2 as ::core::ffi::c_int) {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                        .offset(x as isize) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                        if borg_cave_floor_bold(y, x) {
                            if !((*ag).kill != 0) {
                                break;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if i >= 100 as ::core::ffi::c_int {
            n += 1;
        } else {
            p = borg_danger(y, x, turns, true_0 != 0, false_0 != 0);
            if p > b_p {
                n += 1;
            } else {
                u = 0 as ::core::ffi::c_int;
                while u < 8 as ::core::ffi::c_int {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(
                        (y + *(&raw const ddy_ddd as *const int16_t).offset(u as isize)
                            as ::core::ffi::c_int) as isize,
                    ))
                    .offset(
                        (x + *(&raw const ddx_ddd as *const int16_t).offset(u as isize)
                            as ::core::ffi::c_int) as isize,
                    ) as *mut borg_grid;
                    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                    if (*ag).kill as ::core::ffi::c_int != 0
                        && (*kill).awake as ::core::ffi::c_int != 0
                    {
                        n += 1;
                    }
                    u += 1;
                }
            }
        }
        k += 1;
    }
    if n > emergency {
        borg_note(format(
            b"# No Shoot'N'Scoot. scary squares: %d/100\0" as *const u8
                as *const ::core::ffi::c_char,
            n,
        ));
        return false_0 != 0;
    } else {
        borg_note(format(
            b"# Safe to Shoot'N'Scoot. scary squares: %d/100\0" as *const u8
                as *const ::core::ffi::c_char,
            n,
        ));
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_create_kill(
    mut who: *mut ::core::ffi::c_char,
    mut c: loc,
) -> ::core::ffi::c_int {
    return borg_new_kill(borg_guess_race_name(who), c.y, c.x);
}
unsafe extern "C" fn borg_init_monster_names() {
    let mut i: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut r_max: uint16_t =
        ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
    let mut what: [int16_t; 1024] = [0; 1024];
    let mut text: [*mut ::core::ffi::c_char; 1024] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 1024];
    size = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < r_max as ::core::ffi::c_int {
        let mut r_ptr: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr).name.is_null() {
            if strcmp(
                (*r_ptr).name,
                b"Morgoth, Lord of Darkness\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.morgoth = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Sauron, the Sorcerer\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.sauron = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.tarrasque = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"squint-eyed rogue\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.squint_eyed_rogue = (*r_ptr).ridx;
            }
            if prefix(
                (*r_ptr).name,
                b"Grip\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg.id.grip = (*r_ptr).ridx;
            }
            if prefix(
                (*r_ptr).name,
                b"Fang\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg.id.fang = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"small kobold\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.small_kobold = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"soldier\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.soldier = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"cutpurse\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.cutpurse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"acolyte\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.acolyte = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"apprentice\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.apprentice = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"kobold\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.kobold = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"shrieker mushroom patch\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.shrieker_mushroom_patch = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Farmer Maggot\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.maggot = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"filthy street urchin\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.filthy_street_urchin = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"battle-scarred veteran\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.battle_scarred_veteran = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"mean-looking mercenary\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.mean_looking_mercenary = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Bullroarer the Hobbit\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.bullroarer = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"giant white mouse\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.giant_white_mouse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"white worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.white_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"green worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.green_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"cave spider\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.cave_spider = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"red naga\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.red_naga = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"giant red frog\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.giant_red_frog = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"radiation eye\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.radiation_eye = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"yellow worm mass\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.yellow_worm_mass = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"gravity hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.gravity_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"inertia hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.inertia_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dread\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.dread = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"Beorn, the Shape-Changer\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.beorn = (*r_ptr).ridx;
            }
            if !my_stristr(
                (*r_ptr).name,
                b"Herald of Ulmo\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            {
                borg.id.osse = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dracolich\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.dracolich = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dracolisk\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.dracolisk = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"light hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.light_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"dark hound\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.dark_hound = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"night mare\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.night_mare = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"pukelman\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.pukelman = (*r_ptr).ridx;
            }
            if strcmp(
                (*r_ptr).name,
                b"drolem\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                borg.id.drolem = (*r_ptr).ridx;
            }
            if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                text[size as usize] = borg_massage_special_chars((*r_ptr).name);
                what[size as usize] = i as int16_t;
                size += 1;
            }
        }
        i += 1;
    }
    borg_sort_comp = Some(
        borg_sort_comp_hook
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> bool,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> bool,
        >;
    borg_sort_swap = Some(
        borg_sort_swap_hook
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    borg_sort(
        &raw mut text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut what as *mut int16_t as *mut ::core::ffi::c_void,
        size,
    );
    borg_unique_size = size;
    borg_unique_text = mem_zalloc(
        (borg_unique_size as size_t)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    borg_unique_what = mem_zalloc(
        (borg_unique_size as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        let ref mut fresh5 = *borg_unique_text.offset(i as isize);
        *fresh5 = text[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < size {
        *borg_unique_what.offset(i as isize) = what[i as usize] as ::core::ffi::c_uint;
        i += 1;
    }
    size = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < r_max as ::core::ffi::c_int {
        let mut r_ptr_0: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr_0).name.is_null() {
            if !flag_has(
                &raw mut (*r_ptr_0).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                text[size as usize] = borg_massage_special_chars((*r_ptr_0).name);
                what[size as usize] = i as int16_t;
                size += 1;
                if !(*r_ptr_0).plural.is_null() {
                    text[size as usize] = borg_massage_special_chars((*r_ptr_0).plural);
                    what[size as usize] = i as int16_t;
                    size += 1;
                }
            }
        }
        i += 1;
    }
    borg_sort_comp = Some(
        borg_sort_comp_hook
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> bool,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> bool,
        >;
    borg_sort_swap = Some(
        borg_sort_swap_hook
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    borg_sort(
        &raw mut text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut what as *mut int16_t as *mut ::core::ffi::c_void,
        size,
    );
    borg_normal_size = size;
    borg_normal_text = mem_zalloc(
        (borg_normal_size as size_t)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    borg_normal_what = mem_zalloc(
        (borg_normal_size as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        let ref mut fresh6 = *borg_normal_text.offset(i as isize);
        *fresh6 = text[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < size {
        *borg_normal_what.offset(i as isize) = what[i as usize] as ::core::ffi::c_uint;
        i += 1;
    }
}
unsafe extern "C" fn borg_free_monster_names() {
    let mut i: ::core::ffi::c_int = 0;
    if !borg_normal_what.is_null() {
        mem_free(borg_normal_what as *mut ::core::ffi::c_void);
        borg_normal_what = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    }
    if !borg_normal_text.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < borg_normal_size {
            string_free(*borg_normal_text.offset(i as isize) as *mut ::core::ffi::c_char);
            i += 1;
        }
        mem_free(borg_normal_text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        borg_normal_text = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    borg_normal_size = 0 as ::core::ffi::c_int;
    if !borg_unique_what.is_null() {
        mem_free(borg_unique_what as *mut ::core::ffi::c_void);
        borg_unique_what = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    }
    if !borg_unique_text.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < borg_unique_size {
            string_free(*borg_unique_text.offset(i as isize) as *mut ::core::ffi::c_char);
            i += 1;
        }
        mem_free(borg_unique_text as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        borg_unique_text = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    borg_unique_size = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_kill() {
    borg_free_flow_kill();
    borg_kills_cnt = 0 as int16_t;
    borg_kills_nxt = 1 as int16_t;
    borg_kills =
        mem_zalloc((256 as size_t).wrapping_mul(::core::mem::size_of::<borg_kill>() as size_t))
            as *mut borg_kill;
    borg_race_count = mem_zalloc(
        ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    borg_race_death = mem_zalloc(
        ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        let mut r_ptr: *mut monster_race = r_info.offset(i as isize) as *mut monster_race;
        if !(*r_ptr).name.is_null() {
            if !flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                if (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *borg_race_death.offset(i as isize) = 1 as int16_t;
                }
            }
        }
        i += 1;
    }
    borg_init_monster_names();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_kill() {
    borg_free_monster_names();
    if !borg_race_death.is_null() {
        mem_free(borg_race_death as *mut ::core::ffi::c_void);
        borg_race_death = ::core::ptr::null_mut::<int16_t>();
    }
    if !borg_race_count.is_null() {
        mem_free(borg_race_count as *mut ::core::ffi::c_void);
        borg_race_count = ::core::ptr::null_mut::<int16_t>();
    }
    if !borg_kills.is_null() {
        mem_free(borg_kills as *mut ::core::ffi::c_void);
        borg_kills = ::core::ptr::null_mut::<borg_kill>();
    }
}
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const __SHRT_MAX__: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
