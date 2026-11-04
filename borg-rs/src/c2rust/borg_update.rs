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

extern "C" {
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn prefix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut k_info: *mut object_kind;
    static mut trap_info: *mut trap_kind;
    static mut row_top_map: [::core::ffi::c_int; 3];
    static mut row_bottom_map: [::core::ffi::c_int; 3];
    static mut col_map: [::core::ffi::c_int; 3];
    static mut Term: *mut term;
    static mut tile_width: uint8_t;
    static mut tile_height: uint8_t;
    static mut borg_light_n: int16_t;
    static mut borg_light_y: [uint8_t; 1536];
    static mut borg_light_x: [uint8_t; 1536];
    fn borg_update_light();
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_panel_hgt() -> ::core::ffi::c_int;
    fn borg_panel_wid() -> ::core::ffi::c_int;
    fn borg_forget_view();
    fn borg_update_view();
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut borg_fear_monsters: [[uint16_t; 199]; 67];
    static mut borg_danger_wipe: bool;
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    static mut successful_target: ::core::ffi::c_int;
    static mut borg_data_flow: *mut borg_data;
    static mut borg_data_cost: *mut borg_data;
    static mut borg_data_hard: *mut borg_data;
    static mut borg_data_know: *mut borg_data;
    static mut borg_data_icky: *mut borg_data;
    static mut track_step: borg_track;
    static mut track_door: borg_track;
    static mut track_closed: borg_track;
    static borg_ddx_ddd: [int16_t; 24];
    static borg_ddy_ddd: [int16_t; 24];
    fn borg_can_dig(check_fail: bool, feat: uint8_t) -> bool;
    static mut track_glyph: borg_track;
    static mut borg_needs_new_sea: bool;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn map_info(grid: loc, g: *mut grid_data);
    fn feat_is_shop(feat: ::core::ffi::c_int) -> bool;
    fn square_isvault(c: *mut chunk, grid: loc) -> bool;
    fn square_iswebbed(c: *mut chunk, grid: loc) -> bool;
    fn square_isdisarmabletrap(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    fn square(c: *mut chunk, grid: loc) -> *const square;
    fn square_shopnum(c: *mut chunk, grid: loc) -> ::core::ffi::c_int;
    fn cave_monster(c: *mut chunk, idx: ::core::ffi::c_int) -> *mut monster;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg_game_ratio: int16_t;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    static mut borg_kills_cnt: int16_t;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    static mut borg_race_count: *mut int16_t;
    static mut borg_race_death: *mut int16_t;
    static mut borg_nasties_num: uint8_t;
    static mut borg_nasties_count: [uint8_t; 7];
    static mut borg_nasties: [::core::ffi::c_char; 7];
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_delete_kill(i: ::core::ffi::c_int);
    fn borg_sleep_kill(i: ::core::ffi::c_int);
    fn borg_follow_kill(i: ::core::ffi::c_int);
    fn observe_kill_diff(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        a: uint8_t,
        c: wchar_t,
    ) -> bool;
    fn observe_kill_move(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        d: ::core::ffi::c_int,
        a: uint8_t,
        c: wchar_t,
        flag: bool,
    ) -> bool;
    fn borg_locate_kill(
        who: *mut ::core::ffi::c_char,
        c: loc,
        r: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn borg_count_death(i: ::core::ffi::c_int);
    fn borg_create_kill(who: *mut ::core::ffi::c_char, c: loc) -> ::core::ffi::c_int;
    static mut track_shop_x: *mut ::core::ffi::c_int;
    static mut track_shop_y: *mut ::core::ffi::c_int;
    static mut track_vein: borg_track;
    static mut track_less: borg_track;
    static mut track_more: borg_track;
    static mut borg_takes_cnt: int16_t;
    static mut borg_takes_nxt: int16_t;
    static mut borg_takes: *mut borg_take;
    fn borg_delete_take(i: ::core::ffi::c_int);
    fn borg_follow_take(i: ::core::ffi::c_int);
    fn observe_take_diff(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        a: uint8_t,
        c: wchar_t,
    ) -> bool;
    fn observe_take_move(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        d: ::core::ffi::c_int,
        a: uint8_t,
        c: wchar_t,
    ) -> bool;
    static mut borg_do_crush_junk: bool;
    fn borg_note(what: *const ::core::ffi::c_char);
    static mut track_worn_num: int16_t;
    fn borg_clear_best();
    static mut borg_msg_len: int16_t;
    static mut borg_msg_buf: *mut ::core::ffi::c_char;
    static mut borg_msg_num: int16_t;
    static mut borg_msg_pos: *mut int16_t;
    static mut borg_msg_use: *mut int16_t;
    static mut borg_numb_live_unique: ::core::ffi::c_int;
    static mut borg_first_living_unique: ::core::ffi::c_uint;
    static mut borg_depth_hunted_unique: ::core::ffi::c_int;
    fn borg_los(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_target_unknown_wall(y: ::core::ffi::c_int, x: ::core::ffi::c_int) -> bool;
    static mut bought_item_tval: [::core::ffi::c_int; 10];
    static mut bought_item_sval: [::core::ffi::c_int; 10];
    static mut bought_item_pval: [::core::ffi::c_int; 10];
    static mut bought_item_store: [::core::ffi::c_int; 10];
    static mut bought_item_num: ::core::ffi::c_int;
    static mut bought_item_nxt: ::core::ffi::c_int;
    static mut sold_item_tval: [::core::ffi::c_int; 10];
    static mut sold_item_sval: [::core::ffi::c_int; 10];
    static mut sold_item_pval: [::core::ffi::c_int; 10];
    static mut sold_item_store: [::core::ffi::c_int; 10];
    static mut sold_item_num: ::core::ffi::c_int;
    static mut sold_item_nxt: ::core::ffi::c_int;
    static mut borg_feeling_danger: ::core::ffi::c_int;
    static mut borg_feeling_stuff: ::core::ffi::c_int;
    static mut borg_do_spell: bool;
    fn borg_oops(what: *const ::core::ffi::c_char);
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trap {
    pub t_idx: uint8_t,
    pub kind: *mut trap_kind,
    pub next: *mut trap,
    pub grid: loc,
    pub power: uint8_t,
    pub timeout: uint8_t,
    pub flags: [bitflag; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trap_kind {
    pub name: *mut ::core::ffi::c_char,
    pub text: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
    pub msg: *mut ::core::ffi::c_char,
    pub msg_good: *mut ::core::ffi::c_char,
    pub msg_bad: *mut ::core::ffi::c_char,
    pub msg_xtra: *mut ::core::ffi::c_char,
    pub next: *mut trap_kind,
    pub tidx: ::core::ffi::c_int,
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
    pub rarity: ::core::ffi::c_int,
    pub min_depth: ::core::ffi::c_int,
    pub max_num: ::core::ffi::c_int,
    pub power: random_value,
    pub flags: [bitflag; 2],
    pub save_flags: [bitflag; 5],
    pub effect: *mut effect,
    pub effect_xtra: *mut effect,
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
    pub temp: C2RustUnnamed_6,
    pub status: C2RustUnnamed_5,
    pub time: C2RustUnnamed_4,
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
    pub goal: C2RustUnnamed_3,
    pub init_save: C2RustUnnamed_2,
    pub mon: C2RustUnnamed_1,
    pub id: C2RustUnnamed_0,
    pub near: C2RustUnnamed,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
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
pub const BI_CURHP: C2RustUnnamed_12 = 27;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_12 = 118;
pub const BI_ISSTUN: C2RustUnnamed_12 = 117;
pub const BI_ISCUT: C2RustUnnamed_12 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_12 = 115;
pub const BI_ISWEAK: C2RustUnnamed_12 = 108;
pub const BI_ISIMAGE: C2RustUnnamed_12 = 120;
pub const BI_ISAFRAID: C2RustUnnamed_12 = 113;
pub const BI_ISCONFUSED: C2RustUnnamed_12 = 114;
pub const BI_ISBLIND: C2RustUnnamed_12 = 112;
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
pub const BI_CLEVEL: C2RustUnnamed_12 = 35;
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
pub const BI_CDEPTH: C2RustUnnamed_12 = 105;
pub const FEAT_PERM: C2RustUnnamed_7 = 22;
pub const FEAT_NONE: C2RustUnnamed_7 = 0;
pub const RF_NEVER_MOVE: C2RustUnnamed_10 = 21;
pub const RF_MAX: C2RustUnnamed_10 = 85;
pub const RF_MULTIPLY: C2RustUnnamed_10 = 17;
pub const RSF_S_UNIQUE: C2RustUnnamed_11 = 91;
pub const RSF_S_WRAITH: C2RustUnnamed_11 = 90;
pub const RSF_S_HI_DRAGON: C2RustUnnamed_11 = 89;
pub const RSF_S_HI_UNDEAD: C2RustUnnamed_11 = 88;
pub const RSF_S_DRAGON: C2RustUnnamed_11 = 87;
pub const RSF_S_UNDEAD: C2RustUnnamed_11 = 86;
pub const RSF_S_DEMON: C2RustUnnamed_11 = 85;
pub const RSF_S_AINU: C2RustUnnamed_11 = 84;
pub const RSF_S_HYDRA: C2RustUnnamed_11 = 83;
pub const RSF_S_HOUND: C2RustUnnamed_11 = 82;
pub const RSF_S_SPIDER: C2RustUnnamed_11 = 81;
pub const RSF_S_ANIMAL: C2RustUnnamed_11 = 80;
pub const RSF_S_MONSTERS: C2RustUnnamed_11 = 79;
pub const RSF_S_MONSTER: C2RustUnnamed_11 = 78;
pub const RSF_S_HI_DEMON: C2RustUnnamed_11 = 77;
pub const RSF_S_KIN: C2RustUnnamed_11 = 76;
pub const RSF_SHAPECHANGE: C2RustUnnamed_11 = 75;
pub const BI_CURSP: C2RustUnnamed_12 = 30;
pub const RSF_FORGET: C2RustUnnamed_11 = 74;
pub const RSF_TRAPS: C2RustUnnamed_11 = 73;
pub const RSF_DARKNESS: C2RustUnnamed_11 = 72;
pub const RSF_TELE_LEVEL: C2RustUnnamed_11 = 71;
pub const RSF_TELE_AWAY: C2RustUnnamed_11 = 70;
pub const RSF_TELE_SELF_TO: C2RustUnnamed_11 = 69;
pub const RSF_TELE_TO: C2RustUnnamed_11 = 68;
pub const RSF_TPORT: C2RustUnnamed_11 = 67;
pub const RSF_BLINK: C2RustUnnamed_11 = 66;
pub const RSF_HEAL_KIN: C2RustUnnamed_11 = 65;
pub const RSF_HEAL: C2RustUnnamed_11 = 64;
pub const RSF_HASTE: C2RustUnnamed_11 = 63;
pub const RSF_HOLD: C2RustUnnamed_11 = 62;
pub const RSF_SLOW: C2RustUnnamed_11 = 61;
pub const RSF_CONF: C2RustUnnamed_11 = 60;
pub const RSF_BLIND: C2RustUnnamed_11 = 59;
pub const RSF_SCARE: C2RustUnnamed_11 = 58;
pub const RSF_BE_NETH: C2RustUnnamed_11 = 57;
pub const RSF_BE_ELEC: C2RustUnnamed_11 = 56;
pub const RSF_MISSILE: C2RustUnnamed_11 = 55;
pub const RSF_BO_ICE: C2RustUnnamed_11 = 54;
pub const RSF_BO_PLAS: C2RustUnnamed_11 = 53;
pub const RSF_BO_MANA: C2RustUnnamed_11 = 52;
pub const RSF_BO_WATE: C2RustUnnamed_11 = 51;
pub const BI_RNTHR: C2RustUnnamed_12 = 82;
pub const RSF_BO_NETH: C2RustUnnamed_11 = 50;
pub const BI_IPOIS: C2RustUnnamed_12 = 68;
pub const RSF_BO_POIS: C2RustUnnamed_11 = 49;
pub const BI_ICOLD: C2RustUnnamed_12 = 66;
pub const RSF_BO_COLD: C2RustUnnamed_11 = 48;
pub const BI_IFIRE: C2RustUnnamed_12 = 64;
pub const RSF_BO_FIRE: C2RustUnnamed_11 = 47;
pub const BI_IELEC: C2RustUnnamed_12 = 67;
pub const RSF_BO_ELEC: C2RustUnnamed_11 = 46;
pub const BI_IACID: C2RustUnnamed_12 = 65;
pub const RSF_BO_ACID: C2RustUnnamed_11 = 45;
pub const RSF_WOUND: C2RustUnnamed_11 = 44;
pub const RSF_BRAIN_SMASH: C2RustUnnamed_11 = 43;
pub const RSF_MIND_BLAST: C2RustUnnamed_11 = 42;
pub const BI_MAXSP: C2RustUnnamed_12 = 31;
pub const RSF_DRAIN_MANA: C2RustUnnamed_11 = 41;
pub const BI_RCONF: C2RustUnnamed_12 = 78;
pub const BI_RSND: C2RustUnnamed_12 = 79;
pub const RSF_STORM: C2RustUnnamed_11 = 40;
pub const BI_RBLIND: C2RustUnnamed_12 = 77;
pub const BI_RLITE: C2RustUnnamed_12 = 75;
pub const RSF_BA_LIGHT: C2RustUnnamed_11 = 39;
pub const BI_RDARK: C2RustUnnamed_12 = 76;
pub const RSF_BA_DARK: C2RustUnnamed_11 = 38;
pub const RSF_BA_HOLY: C2RustUnnamed_11 = 37;
pub const RSF_BA_MANA: C2RustUnnamed_11 = 36;
pub const RSF_BA_WATE: C2RustUnnamed_11 = 35;
pub const RSF_BA_NETH: C2RustUnnamed_11 = 34;
pub const BI_RSHRD: C2RustUnnamed_12 = 80;
pub const RSF_BA_SHAR: C2RustUnnamed_11 = 33;
pub const BI_RPOIS: C2RustUnnamed_12 = 73;
pub const RSF_BA_POIS: C2RustUnnamed_11 = 32;
pub const RSF_BA_COLD: C2RustUnnamed_11 = 31;
pub const RSF_BA_FIRE: C2RustUnnamed_11 = 30;
pub const RSF_BA_ELEC: C2RustUnnamed_11 = 29;
pub const RSF_BA_ACID: C2RustUnnamed_11 = 28;
pub const RSF_WEAVE: C2RustUnnamed_11 = 27;
pub const RSF_BOULDER: C2RustUnnamed_11 = 26;
pub const RSF_BR_MANA: C2RustUnnamed_11 = 25;
pub const RSF_BR_WALL: C2RustUnnamed_11 = 24;
pub const RSF_BR_PLAS: C2RustUnnamed_11 = 23;
pub const RSF_BR_SHAR: C2RustUnnamed_11 = 22;
pub const RSF_BR_GRAV: C2RustUnnamed_11 = 21;
pub const RSF_BR_INER: C2RustUnnamed_11 = 20;
pub const RSF_BR_TIME: C2RustUnnamed_11 = 19;
pub const BI_RNXUS: C2RustUnnamed_12 = 81;
pub const RSF_BR_NEXU: C2RustUnnamed_11 = 18;
pub const BI_RDIS: C2RustUnnamed_12 = 84;
pub const RSF_BR_DISE: C2RustUnnamed_11 = 17;
pub const BI_HLIFE: C2RustUnnamed_12 = 85;
pub const BI_RKAOS: C2RustUnnamed_12 = 83;
pub const RSF_BR_CHAO: C2RustUnnamed_11 = 16;
pub const RSF_BR_SOUN: C2RustUnnamed_11 = 15;
pub const RSF_BR_DARK: C2RustUnnamed_11 = 14;
pub const RSF_BR_LIGHT: C2RustUnnamed_11 = 13;
pub const RSF_BR_NETH: C2RustUnnamed_11 = 12;
pub const RSF_BR_POIS: C2RustUnnamed_11 = 11;
pub const RSF_BR_COLD: C2RustUnnamed_11 = 10;
pub const RSF_BR_FIRE: C2RustUnnamed_11 = 9;
pub const RSF_BR_ELEC: C2RustUnnamed_11 = 8;
pub const RSF_BR_ACID: C2RustUnnamed_11 = 7;
pub const RSF_BOLT: C2RustUnnamed_11 = 6;
pub const RSF_ARROW: C2RustUnnamed_11 = 5;
pub const RSF_SHOT: C2RustUnnamed_11 = 4;
pub const RSF_SPIT: C2RustUnnamed_11 = 3;
pub const RSF_WHIP: C2RustUnnamed_11 = 2;
pub const RSF_SHRIEK: C2RustUnnamed_11 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_wank {
    pub x: uint8_t,
    pub y: uint8_t,
    pub t_a: uint8_t,
    pub t_c: wchar_t,
    pub is_take: bool,
    pub is_kill: bool,
}
pub const FEAT_BROKEN: C2RustUnnamed_7 = 4;
pub const FEAT_OPEN: C2RustUnnamed_7 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub const FEAT_MORE: C2RustUnnamed_7 = 6;
pub const BI_KING: C2RustUnnamed_12 = 107;
pub const FEAT_FLOOR: C2RustUnnamed_7 = 1;
pub const RF_PASS_WALL: C2RustUnnamed_10 = 46;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_data {
    pub data: [[uint8_t; 198]; 66],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_data {
    pub m_idx: uint32_t,
    pub f_idx: uint32_t,
    pub first_kind: *mut object_kind,
    pub trap: *mut trap,
    pub multiple_objects: bool,
    pub unseen_object: bool,
    pub unseen_money: bool,
    pub lighting: grid_light_level,
    pub in_view: bool,
    pub is_player: bool,
    pub hallucinate: bool,
}
pub type grid_light_level = ::core::ffi::c_uint;
pub const LIGHTING_MAX: grid_light_level = 4;
pub const LIGHTING_DARK: grid_light_level = 3;
pub const LIGHTING_LIT: grid_light_level = 2;
pub const LIGHTING_TORCH: grid_light_level = 1;
pub const LIGHTING_LOS: grid_light_level = 0;
pub const MON_TMD_SLEEP: C2RustUnnamed_9 = 0;
pub const FEAT_LESS: C2RustUnnamed_7 = 5;
pub const FEAT_CLOSED: C2RustUnnamed_7 = 2;
pub const FEAT_QUARTZ_K: C2RustUnnamed_7 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_7 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_7 = 18;
pub const FEAT_RUBBLE: C2RustUnnamed_7 = 16;
pub const FEAT_MAGMA: C2RustUnnamed_7 = 17;
pub const FEAT_LAVA: C2RustUnnamed_7 = 23;
pub const FEAT_GRANITE: C2RustUnnamed_7 = 21;
pub const TRF_GLYPH: C2RustUnnamed_8 = 1;
pub const TRF_MAX: C2RustUnnamed_8 = 15;
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
pub struct mouseclick {
    pub type_0: ui_event_type,
    pub x: uint8_t,
    pub y: uint8_t,
    pub button: uint8_t,
    pub mods: uint8_t,
}
pub const BORG_AUTOSAVE: C2RustUnnamed_13 = 33;
pub const RF_SEASONAL: C2RustUnnamed_10 = 15;
pub const RF_QUESTOR: C2RustUnnamed_10 = 2;
pub const RF_UNIQUE: C2RustUnnamed_10 = 1;
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
pub const BI_LIGHT: C2RustUnnamed_12 = 26;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_7 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_7 = 24;
pub const FEAT_SECRET: C2RustUnnamed_7 = 15;
pub const FEAT_HOME: C2RustUnnamed_7 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_7 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_7 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_7 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_7 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_7 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_7 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_7 = 7;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const TRF_WEB: C2RustUnnamed_8 = 14;
pub const TRF_DELAY: C2RustUnnamed_8 = 13;
pub const TRF_LOCK: C2RustUnnamed_8 = 12;
pub const TRF_SAVE_ARMOR: C2RustUnnamed_8 = 11;
pub const TRF_SAVE_THROW: C2RustUnnamed_8 = 10;
pub const TRF_MAGICAL: C2RustUnnamed_8 = 9;
pub const TRF_ONETIME: C2RustUnnamed_8 = 8;
pub const TRF_PIT: C2RustUnnamed_8 = 7;
pub const TRF_DOWN: C2RustUnnamed_8 = 6;
pub const TRF_FLOOR: C2RustUnnamed_8 = 5;
pub const TRF_INVISIBLE: C2RustUnnamed_8 = 4;
pub const TRF_VISIBLE: C2RustUnnamed_8 = 3;
pub const TRF_TRAP: C2RustUnnamed_8 = 2;
pub const TRF_NONE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const MON_TMD_MAX: C2RustUnnamed_9 = 10;
pub const MON_TMD_CHANGED: C2RustUnnamed_9 = 9;
pub const MON_TMD_COMMAND: C2RustUnnamed_9 = 8;
pub const MON_TMD_DISEN: C2RustUnnamed_9 = 7;
pub const MON_TMD_HOLD: C2RustUnnamed_9 = 6;
pub const MON_TMD_FAST: C2RustUnnamed_9 = 5;
pub const MON_TMD_SLOW: C2RustUnnamed_9 = 4;
pub const MON_TMD_FEAR: C2RustUnnamed_9 = 3;
pub const MON_TMD_CONF: C2RustUnnamed_9 = 2;
pub const MON_TMD_STUN: C2RustUnnamed_9 = 1;
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
pub const RF_NEVER_BLOW: C2RustUnnamed_10 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_10 = 19;
pub const RF_REGENERATE: C2RustUnnamed_10 = 18;
pub const RF_UNAWARE: C2RustUnnamed_10 = 16;
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
pub const RF_NONE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const RSF_MAX: C2RustUnnamed_11 = 92;
pub const RSF_NONE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_12 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_12 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_12 = 263;
pub const BI_EMPTY: C2RustUnnamed_12 = 262;
pub const BI_CARRY: C2RustUnnamed_12 = 261;
pub const BI_WEIGHT: C2RustUnnamed_12 = 260;
pub const BI_DINV: C2RustUnnamed_12 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_12 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_12 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_12 = 256;
pub const BI_ADIGGER: C2RustUnnamed_12 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_12 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_12 = 253;
pub const BI_AROD2: C2RustUnnamed_12 = 252;
pub const BI_AROD1: C2RustUnnamed_12 = 251;
pub const BI_ASHROOM: C2RustUnnamed_12 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_12 = 249;
pub const BI_AHWORD: C2RustUnnamed_12 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_12 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_12 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_12 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_12 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_12 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_12 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_12 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_12 = 240;
pub const BI_ABRAND: C2RustUnnamed_12 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_12 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_12 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_12 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_12 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_12 = 234;
pub const BI_ACLW: C2RustUnnamed_12 = 233;
pub const BI_ACSW: C2RustUnnamed_12 = 232;
pub const BI_ACCW: C2RustUnnamed_12 = 231;
pub const BI_AGLYPH: C2RustUnnamed_12 = 230;
pub const BI_APFE: C2RustUnnamed_12 = 229;
pub const BI_ALITE: C2RustUnnamed_12 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_12 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_12 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_12 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_12 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_12 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_12 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_12 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_12 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_12 = 219;
pub const BI_ASPEED: C2RustUnnamed_12 = 218;
pub const BI_AID: C2RustUnnamed_12 = 217;
pub const BI_ALIFE: C2RustUnnamed_12 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_12 = 215;
pub const BI_AHEAL: C2RustUnnamed_12 = 214;
pub const BI_AFUEL: C2RustUnnamed_12 = 213;
pub const BI_AESCAPE: C2RustUnnamed_12 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_12 = 211;
pub const BI_APHASE: C2RustUnnamed_12 = 210;
pub const BI_WB_POIS: C2RustUnnamed_12 = 209;
pub const BI_WB_COLD: C2RustUnnamed_12 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_12 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_12 = 206;
pub const BI_WB_ACID: C2RustUnnamed_12 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_12 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_12 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_12 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_12 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_12 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_12 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_12 = 198;
pub const BI_WS_ORC: C2RustUnnamed_12 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_12 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_12 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_12 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_12 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_12 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_12 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_12 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_12 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_12 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_12 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_12 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_12 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_12 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_12 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_12 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_12 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_12 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_12 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_12 = 178;
pub const BI_CRSSICK: C2RustUnnamed_12 = 177;
pub const BI_CRSDULL: C2RustUnnamed_12 = 176;
pub const BI_CRSVULN: C2RustUnnamed_12 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_12 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_12 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_12 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_12 = 171;
pub const BI_CRSSUND: C2RustUnnamed_12 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_12 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_12 = 168;
pub const BI_CRSPARA: C2RustUnnamed_12 = 167;
pub const BI_CRSHALU: C2RustUnnamed_12 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_12 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_12 = 164;
pub const BI_CRSTELE: C2RustUnnamed_12 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_12 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_12 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_12 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_12 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_12 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_12 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_12 = 156;
pub const BI_AMISSILES: C2RustUnnamed_12 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_12 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_12 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_12 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_12 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_12 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_12 = 149;
pub const BI_SHOTS: C2RustUnnamed_12 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_12 = 147;
pub const BI_BLOWS: C2RustUnnamed_12 = 146;
pub const BI_BART: C2RustUnnamed_12 = 145;
pub const BI_SLING: C2RustUnnamed_12 = 144;
pub const BI_BTODAM: C2RustUnnamed_12 = 143;
pub const BI_BTOHIT: C2RustUnnamed_12 = 142;
pub const BI_BID: C2RustUnnamed_12 = 141;
pub const BI_WDS: C2RustUnnamed_12 = 140;
pub const BI_WDD: C2RustUnnamed_12 = 139;
pub const BI_WID: C2RustUnnamed_12 = 138;
pub const BI_WTODAM: C2RustUnnamed_12 = 137;
pub const BI_WTOHIT: C2RustUnnamed_12 = 136;
pub const BI_TODAM: C2RustUnnamed_12 = 135;
pub const BI_TOHIT: C2RustUnnamed_12 = 134;
pub const BI_ARMOR: C2RustUnnamed_12 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_12 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_12 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_12 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_12 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_12 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_12 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_12 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_12 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_12 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_12 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_12 = 122;
pub const BI_ISFORGET: C2RustUnnamed_12 = 121;
pub const BI_ISPARALYZED: C2RustUnnamed_12 = 119;
pub const BI_ISGORGED: C2RustUnnamed_12 = 111;
pub const BI_ISFULL: C2RustUnnamed_12 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_12 = 109;
pub const BI_MAXDEPTH: C2RustUnnamed_12 = 106;
pub const BI_SFRACT: C2RustUnnamed_12 = 104;
pub const BI_SHLIFE: C2RustUnnamed_12 = 103;
pub const BI_SRDIS: C2RustUnnamed_12 = 102;
pub const BI_SRKAOS: C2RustUnnamed_12 = 101;
pub const BI_SRNTHR: C2RustUnnamed_12 = 100;
pub const BI_SRNXUS: C2RustUnnamed_12 = 99;
pub const BI_SRSHRD: C2RustUnnamed_12 = 98;
pub const BI_SRSND: C2RustUnnamed_12 = 97;
pub const BI_SRCONF: C2RustUnnamed_12 = 96;
pub const BI_SRBLIND: C2RustUnnamed_12 = 95;
pub const BI_SRDARK: C2RustUnnamed_12 = 94;
pub const BI_SRLITE: C2RustUnnamed_12 = 93;
pub const BI_SRFEAR: C2RustUnnamed_12 = 92;
pub const BI_SRPOIS: C2RustUnnamed_12 = 91;
pub const BI_SRACID: C2RustUnnamed_12 = 90;
pub const BI_SRELEC: C2RustUnnamed_12 = 89;
pub const BI_SRCOLD: C2RustUnnamed_12 = 88;
pub const BI_SRFIRE: C2RustUnnamed_12 = 87;
pub const BI_FRACT: C2RustUnnamed_12 = 86;
pub const BI_RFEAR: C2RustUnnamed_12 = 74;
pub const BI_RACID: C2RustUnnamed_12 = 72;
pub const BI_RELEC: C2RustUnnamed_12 = 71;
pub const BI_RCOLD: C2RustUnnamed_12 = 70;
pub const BI_RFIRE: C2RustUnnamed_12 = 69;
pub const BI_DIG: C2RustUnnamed_12 = 63;
pub const BI_THT: C2RustUnnamed_12 = 62;
pub const BI_THB: C2RustUnnamed_12 = 61;
pub const BI_THN: C2RustUnnamed_12 = 60;
pub const BI_SRCH: C2RustUnnamed_12 = 59;
pub const BI_STL: C2RustUnnamed_12 = 58;
pub const BI_SAV: C2RustUnnamed_12 = 57;
pub const BI_DEV: C2RustUnnamed_12 = 56;
pub const BI_DISM: C2RustUnnamed_12 = 55;
pub const BI_DISP: C2RustUnnamed_12 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_12 = 53;
pub const BI_INFRA: C2RustUnnamed_12 = 52;
pub const BI_SINV: C2RustUnnamed_12 = 51;
pub const BI_REG: C2RustUnnamed_12 = 50;
pub const BI_FEATH: C2RustUnnamed_12 = 49;
pub const BI_SDIG: C2RustUnnamed_12 = 48;
pub const BI_DAM_RED: C2RustUnnamed_12 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_12 = 46;
pub const BI_GOLD: C2RustUnnamed_12 = 45;
pub const BI_SPEED: C2RustUnnamed_12 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_12 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_12 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_12 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_12 = 40;
pub const BI_FOOD: C2RustUnnamed_12 = 39;
pub const BI_RECALL: C2RustUnnamed_12 = 38;
pub const BI_ESP: C2RustUnnamed_12 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_12 = 36;
pub const BI_FAIL2: C2RustUnnamed_12 = 34;
pub const BI_FAIL1: C2RustUnnamed_12 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_12 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_12 = 29;
pub const BI_MAXHP: C2RustUnnamed_12 = 28;
pub const BI_CLASS: C2RustUnnamed_12 = 25;
pub const BI_SCON: C2RustUnnamed_12 = 24;
pub const BI_SDEX: C2RustUnnamed_12 = 23;
pub const BI_SWIS: C2RustUnnamed_12 = 22;
pub const BI_SINT: C2RustUnnamed_12 = 21;
pub const BI_SSTR: C2RustUnnamed_12 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_12 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_12 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_12 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_12 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_12 = 15;
pub const BI_CCON: C2RustUnnamed_12 = 14;
pub const BI_CDEX: C2RustUnnamed_12 = 13;
pub const BI_CWIS: C2RustUnnamed_12 = 12;
pub const BI_CINT: C2RustUnnamed_12 = 11;
pub const BI_CSTR: C2RustUnnamed_12 = 10;
pub const BI_ACON: C2RustUnnamed_12 = 9;
pub const BI_ADEX: C2RustUnnamed_12 = 8;
pub const BI_AWIS: C2RustUnnamed_12 = 7;
pub const BI_AINT: C2RustUnnamed_12 = 6;
pub const BI_ASTR: C2RustUnnamed_12 = 5;
pub const BI_CON: C2RustUnnamed_12 = 4;
pub const BI_DEX: C2RustUnnamed_12 = 3;
pub const BI_WIS: C2RustUnnamed_12 = 2;
pub const BI_INT: C2RustUnnamed_12 = 1;
pub const BI_STR: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_13 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_13 = 34;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_13 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_13 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_13 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_13 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_13 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_13 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_13 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_13 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_13 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_13 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_13 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_13 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_13 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_13 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_13 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_13 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_13 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_13 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_13 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_13 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_13 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_13 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_13 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_13 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_13 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_13 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_13 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_13 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_13 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_13 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_13 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_13 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_13 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut borg_wank_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut borg_wanks: *mut borg_wank = ::core::ptr::null::<borg_wank>() as *mut borg_wank;
#[no_mangle]
pub static mut borg_failure: bool = false;
#[no_mangle]
pub static mut borg_detect_wall: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_trap: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_door: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_evil: *mut *mut bool =
    ::core::ptr::null::<*mut bool>() as *mut *mut bool;
#[no_mangle]
pub static mut borg_detect_obj: *mut *mut bool = ::core::ptr::null::<*mut bool>() as *mut *mut bool;
static mut old_panels: loc = loc {
    x: -(1 as ::core::ffi::c_int),
    y: -(1 as ::core::ffi::c_int),
};
#[no_mangle]
pub static mut borg_do_update_view: bool = false_0 != 0;
#[no_mangle]
pub static mut borg_do_update_lite: bool = false_0 != 0;
static mut o_w_x: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut o_w_y: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut old_c: loc = loc {
    x: -(1 as ::core::ffi::c_int),
    y: -(1 as ::core::ffi::c_int),
};
unsafe extern "C" fn borg_forget_map() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    y = 0 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y {
        x = 0 as ::core::ffi::c_int;
        while x < AUTO_MAX_X {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            memset(
                ag as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<borg_grid>() as size_t,
            );
            (*ag).feat = FEAT_PERM as ::core::ffi::c_int as uint8_t;
            x += 1;
        }
        y += 1;
    }
    y = 1 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            (*ag).feat = FEAT_NONE as ::core::ffi::c_int as uint8_t;
            if *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0
            {
                (*ag).feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
            }
            x += 1;
        }
        y += 1;
    }
    memcpy(
        borg_data_cost as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
    memcpy(
        borg_data_flow as *mut ::core::ffi::c_void,
        borg_data_hard as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_data>() as size_t,
    );
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
    borg_forget_view();
}
unsafe extern "C" fn borg_update_map() {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g: grid_data = grid_data {
        m_idx: 0,
        f_idx: 0,
        first_kind: ::core::ptr::null_mut::<object_kind>(),
        trap: ::core::ptr::null_mut::<trap>(),
        multiple_objects: false,
        unseen_object: false,
        unseen_money: false,
        lighting: LIGHTING_LOS,
        in_view: false,
        is_player: false,
        hallucinate: false,
    };
    dy = 0 as ::core::ffi::c_int;
    while dy
        < ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        let mut current_block_150: u64;
        dx = 0 as ::core::ffi::c_int;
        while dx
            < ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut old_wall: bool = false;
            let mut new_wall: bool = false;
            x = w_x + dx;
            y = w_y + dy;
            let mut l: loc = loc(x, y);
            if square_in_bounds(cave, l) {
                map_info(l, &raw mut g);
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                    .offset(x as isize) as *mut borg_grid;
                (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_OKAY) as uint16_t;
                if g.f_idx != FEAT_NONE as ::core::ffi::c_int as uint32_t
                    && (g.in_view as ::core::ffi::c_int != 0
                        || (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0)
                {
                    if g.in_view {
                        (*ag).info =
                            ((*ag).info as ::core::ffi::c_int & !BORG_IGNORE_MAP) as uint16_t;
                    }
                    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_MARK) as uint16_t;
                    (*ag).feat = g.f_idx as uint8_t;
                }
                (*ag).store = BORG_HOME as uint8_t;
                if g.is_player {
                    borg.c.x = x;
                    borg.c.y = y;
                }
                old_wall = !borg_cave_floor_grid(ag);
                if feat_is_shop(g.f_idx as ::core::ffi::c_int) {
                    (*ag).feat = g.f_idx as uint8_t;
                    i = square_shopnum(cave, l);
                    (*ag).store = i as uint8_t;
                    *track_shop_x.offset(i as isize) = x;
                    *track_shop_y.offset(i as isize) = y;
                    current_block_150 = 18002345992382212654;
                } else if square_isdisarmabletrap(cave, l) {
                    (*ag).trap = true_0 != 0;
                    let mut t_idx: uint8_t = (*(*square(cave, l)).trap).t_idx;
                    if flag_has(
                        &raw mut (*trap_info.offset(t_idx as isize)).flags as *mut bitflag,
                        (TRF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        TRF_GLYPH as ::core::ffi::c_int,
                    ) {
                        (*ag).glyph = true_0 != 0;
                        i = 0 as ::core::ffi::c_int;
                        while i < track_glyph.num as ::core::ffi::c_int {
                            if *track_glyph.x.offset(i as isize) == x
                                && *track_glyph.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_glyph.num as ::core::ffi::c_int
                            && i < track_glyph.size as ::core::ffi::c_int
                        {
                            *track_glyph.x.offset(i as isize) = x;
                            *track_glyph.y.offset(i as isize) = y;
                            track_glyph.num += 1;
                        }
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_NONE as ::core::ffi::c_int as uint32_t {
                    (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_GLOW) as uint16_t;
                    if (*ag).feat as ::core::ffi::c_int != FEAT_NONE as ::core::ffi::c_int {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_DARK) as uint16_t;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_NONE as ::core::ffi::c_int as uint32_t {
                    *borg
                        .trait_0
                        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                        != 0;
                    if g.lighting as ::core::ffi::c_uint
                        == LIGHTING_DARK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_DARK) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_GLOW) as uint16_t;
                    } else if g.lighting as ::core::ffi::c_uint
                        == LIGHTING_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    } else {
                        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_LIGHT) as uint16_t;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_OPEN as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_BROKEN as ::core::ffi::c_int as uint32_t
                {
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_GRANITE as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_PERM as ::core::ffi::c_int as uint32_t
                {
                    if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            != 0
                        && x != 0
                        && y != 0
                        && x != (*cave).width - 1 as ::core::ffi::c_int
                        && y != (*cave).height - 1 as ::core::ffi::c_int
                    {
                        borg.status.vault = true_0 != 0;
                    }
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_LAVA as ::core::ffi::c_int as uint32_t {
                    current_block_150 = 18002345992382212654;
                } else if g.f_idx == FEAT_MAGMA as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_QUARTZ as ::core::ffi::c_int as uint32_t
                    || g.f_idx == FEAT_RUBBLE as ::core::ffi::c_int as uint32_t
                {
                    if borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
                        if !borg_can_dig(
                            true_0 != 0,
                            FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t,
                        ) && g.f_idx == FEAT_QUARTZ as ::core::ffi::c_int as uint32_t
                        {
                            current_block_150 = 10680521327981672866;
                        } else {
                            i = 0 as ::core::ffi::c_int;
                            while i < track_vein.num as ::core::ffi::c_int {
                                if *track_vein.x.offset(i as isize) == x
                                    && *track_vein.y.offset(i as isize) == y
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == track_vein.num as ::core::ffi::c_int
                                && i < track_vein.size as ::core::ffi::c_int
                            {
                                *track_vein.x.offset(i as isize) = x;
                                *track_vein.y.offset(i as isize) = y;
                                track_vein.num += 1;
                                if track_vein.num as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
                                    track_vein.num = 99 as int16_t;
                                }
                            }
                            current_block_150 = 18002345992382212654;
                        }
                    } else {
                        current_block_150 = 18002345992382212654;
                    }
                } else {
                    if g.f_idx == FEAT_MAGMA_K as ::core::ffi::c_int as uint32_t
                        || g.f_idx == FEAT_QUARTZ_K as ::core::ffi::c_int as uint32_t
                    {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_vein.num as ::core::ffi::c_int {
                            if *track_vein.x.offset(i as isize) == x
                                && *track_vein.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_vein.num as ::core::ffi::c_int
                            && i < track_vein.size as ::core::ffi::c_int
                        {
                            *track_vein.x.offset(i as isize) = x;
                            *track_vein.y.offset(i as isize) = y;
                            track_vein.num += 1;
                            if track_vein.num as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
                                track_vein.num = 99 as int16_t;
                            }
                        }
                    } else if g.f_idx == FEAT_CLOSED as ::core::ffi::c_int as uint32_t {
                        if *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                            <= 5 as ::core::ffi::c_int
                        {
                            i = 0 as ::core::ffi::c_int;
                            while i < track_closed.num as ::core::ffi::c_int {
                                if *track_closed.x.offset(i as isize) == x
                                    && *track_closed.y.offset(i as isize) == y
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == track_closed.num as ::core::ffi::c_int
                                && i < track_closed.size as ::core::ffi::c_int
                            {
                                *track_closed.x.offset(i as isize) = x;
                                *track_closed.y.offset(i as isize) = y;
                                track_closed.num += 1;
                                if track_closed.num as ::core::ffi::c_int
                                    > 254 as ::core::ffi::c_int
                                {
                                    track_closed.num = 254 as int16_t;
                                }
                            }
                        }
                    } else if g.f_idx == FEAT_LESS as ::core::ffi::c_int as uint32_t {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_less.num as ::core::ffi::c_int {
                            if *track_less.x.offset(i as isize) == x
                                && *track_less.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_less.num as ::core::ffi::c_int
                            && i < track_less.size as ::core::ffi::c_int
                        {
                            *track_less.x.offset(i as isize) = x;
                            *track_less.y.offset(i as isize) = y;
                            track_less.num += 1;
                        }
                    } else if g.f_idx == FEAT_MORE as ::core::ffi::c_int as uint32_t {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_more.num as ::core::ffi::c_int {
                            if *track_more.x.offset(i as isize) == x
                                && *track_more.y.offset(i as isize) == y
                            {
                                break;
                            }
                            i += 1;
                        }
                        if i == track_more.num as ::core::ffi::c_int
                            && i < track_more.size as ::core::ffi::c_int
                        {
                            *track_more.x.offset(i as isize) = x;
                            *track_more.y.offset(i as isize) = y;
                            track_more.num += 1;
                        }
                    }
                    current_block_150 = 18002345992382212654;
                }
                match current_block_150 {
                    10680521327981672866 => {}
                    _ => {
                        if (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
                            && square_iswebbed(cave, l) as ::core::ffi::c_int != 0
                        {
                            (*ag).web = true_0 != 0;
                        } else {
                            (*ag).web = false_0 != 0;
                        }
                        if (!g.first_kind.is_null() || g.m_idx != 0)
                            && *borg
                                .trait_0
                                .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                                == 0
                        {
                            let mut wank: *mut borg_wank = ::core::ptr::null_mut::<borg_wank>();
                            let mut is_mimic: bool = false_0 != 0;
                            if borg_wank_num == AUTO_VIEW_MAX {
                                borg_note(format(
                                    b"# Wank problem at grid (%d,%d) m:%lu o:%lu, borg at (%d,%d)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    y,
                                    x,
                                    g.m_idx as ::core::ffi::c_ulong,
                                    (if !g.first_kind.is_null() {
                                        (*g.first_kind).kidx
                                    } else {
                                        0 as uint32_t
                                    }) as ::core::ffi::c_ulong,
                                    borg.c.y,
                                    borg.c.x,
                                ));
                                borg_oops(
                                    b"too many objects...\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            let fresh0 = borg_wank_num;
                            borg_wank_num = borg_wank_num + 1;
                            wank = borg_wanks.offset(fresh0 as isize) as *mut borg_wank;
                            (*wank).x = x as uint8_t;
                            (*wank).y = y as uint8_t;
                            if g.m_idx != 0 {
                                let mut m_ptr: *mut monster =
                                    cave_monster(cave, g.m_idx as ::core::ffi::c_int);
                                (*wank).t_a = (*m_ptr).attr;
                                (*wank).t_c =
                                    (*r_info.offset((*(*m_ptr).race).ridx as isize)).d_char;
                                if !(*m_ptr).mimicked_obj.is_null()
                                    && (*m_ptr).m_timed
                                        [MON_TMD_SLEEP as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        != 0
                                {
                                    is_mimic = true_0 != 0;
                                }
                            } else {
                                (*wank).t_a = (*g.first_kind).d_attr;
                                (*wank).t_c = (*g.first_kind).d_char;
                            }
                            (*wank).is_take =
                                !g.first_kind.is_null() || is_mimic as ::core::ffi::c_int != 0;
                            (*wank).is_kill = g.m_idx != 0 as uint32_t && !is_mimic;
                        }
                        new_wall = !borg_cave_floor_grid(ag);
                        if old_wall as ::core::ffi::c_int != new_wall as ::core::ffi::c_int {
                            if new_wall {
                                (*borg_data_flow).data[y as usize][x as usize] = 255 as uint8_t;
                            }
                            (*borg_data_know).data[y as usize][x as usize] = false_0 as uint8_t;
                            (*borg_data_icky).data[y as usize][x as usize] = false_0 as uint8_t;
                            if (*ag).info as ::core::ffi::c_int & BORG_VIEW != 0 {
                                borg_do_update_view = true_0 != 0;
                            }
                            if (*ag).info as ::core::ffi::c_int & BORG_LIGHT != 0 {
                                borg_do_update_lite = true_0 != 0;
                            }
                            borg.time.wizard_light = 0 as ::core::ffi::c_int as borg_time;
                            borg.time.detect_walls = 0 as ::core::ffi::c_int as borg_time;
                        }
                    }
                }
            }
            dx += 1;
        }
        dy += 1;
    }
}
unsafe extern "C" fn borg_fear_grid(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) {
    let mut x1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    if borg.morgoth_position as ::core::ffi::c_int != 0
        || borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return;
    }
    if square_isvault(cave, loc(x, y)) {
        return;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        == 50 as ::core::ffi::c_int
    {
        k = k * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    x1 = -(6 as ::core::ffi::c_int);
    while x1 <= 6 as ::core::ffi::c_int {
        y1 = -(6 as ::core::ffi::c_int);
        while y1 <= 6 as ::core::ffi::c_int {
            if !(x + x1 <= 0 as ::core::ffi::c_int || x1 + x >= AUTO_MAX_X) {
                if !(y + y1 <= 0 as ::core::ffi::c_int || y1 + y >= AUTO_MAX_Y) {
                    if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                        borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                            (borg_fear_monsters[(y + y1) as usize][(x + x1) as usize]
                                as ::core::ffi::c_int
                                + k / 8 as ::core::ffi::c_int)
                                as uint16_t;
                    }
                    if !(x1 <= -(5 as ::core::ffi::c_int) || x1 >= 5 as ::core::ffi::c_int) {
                        if !(y1 <= -(5 as ::core::ffi::c_int) || y1 >= 5 as ::core::ffi::c_int) {
                            if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                                borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                                    (borg_fear_monsters[(y + y1) as usize][(x + x1) as usize]
                                        as ::core::ffi::c_int
                                        + k / 5 as ::core::ffi::c_int)
                                        as uint16_t;
                            }
                            if !(x1 <= -(3 as ::core::ffi::c_int) || x1 >= 3 as ::core::ffi::c_int)
                            {
                                if !(y1 <= -(3 as ::core::ffi::c_int)
                                    || y1 >= 3 as ::core::ffi::c_int)
                                {
                                    if borg_los((*kill).pos.y, (*kill).pos.x, y + y1, x + x1) {
                                        borg_fear_monsters[(y + y1) as usize][(x + x1) as usize] =
                                            (borg_fear_monsters[(y + y1) as usize]
                                                [(x + x1) as usize]
                                                as ::core::ffi::c_int
                                                + k / 3 as ::core::ffi::c_int)
                                                as uint16_t;
                                    }
                                    if !(x1 <= -(2 as ::core::ffi::c_int)
                                        || x1 >= 2 as ::core::ffi::c_int)
                                    {
                                        if !(y1 <= -(2 as ::core::ffi::c_int)
                                            || y1 >= 2 as ::core::ffi::c_int)
                                        {
                                            if borg_los(
                                                (*kill).pos.y,
                                                (*kill).pos.x,
                                                y + y1,
                                                x + x1,
                                            ) {
                                                borg_fear_monsters[(y + y1) as usize]
                                                    [(x + x1) as usize] = (borg_fear_monsters
                                                    [(y + y1) as usize]
                                                    [(x + x1) as usize]
                                                    as ::core::ffi::c_int
                                                    + k / 2 as ::core::ffi::c_int)
                                                    as uint16_t;
                                            }
                                            if !(x1 <= -(1 as ::core::ffi::c_int)
                                                || x1 >= 1 as ::core::ffi::c_int)
                                            {
                                                if !(y1 <= -(1 as ::core::ffi::c_int)
                                                    || y1 >= 1 as ::core::ffi::c_int)
                                                {
                                                    if borg_los(
                                                        (*kill).pos.y,
                                                        (*kill).pos.x,
                                                        y + y1,
                                                        x + x1,
                                                    ) {
                                                        borg_fear_monsters[(y + y1) as usize]
                                                            [(x + x1) as usize] =
                                                            (borg_fear_monsters[(y + y1) as usize]
                                                                [(x + x1) as usize]
                                                                as ::core::ffi::c_int
                                                                + k)
                                                                as uint16_t;
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
            y1 += 1;
        }
        x1 += 1;
    }
}
unsafe extern "C" fn borg_fear_regional(
    mut who: *const ::core::ffi::c_char,
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut seen_guy: bool,
) {
    let mut x0: ::core::ffi::c_int = 0;
    let mut y0: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    if square_isvault(cave, loc(x, y)) {
        return;
    }
    if seen_guy {
        borg_note(format(
            b"# Fearing region value %d.\0" as *const u8 as *const ::core::ffi::c_char,
            k,
        ));
    } else {
        borg_note(format(
            b"# Fearing region (%d,%d) value %d because of a non-LOS %s\0" as *const u8
                as *const ::core::ffi::c_char,
            y,
            x,
            k,
            who,
        ));
        borg.need_shift_panel = true_0 != 0;
    }
    y0 = y / 11 as ::core::ffi::c_int;
    x0 = x / 11 as ::core::ffi::c_int;
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
    borg_fear_region[y0 as usize][x0 as usize] =
        (borg_fear_region[y0 as usize][x0 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y0 as usize][x1 as usize] =
        (borg_fear_region[y0 as usize][x1 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y0 as usize][x2 as usize] =
        (borg_fear_region[y0 as usize][x2 as usize] as ::core::ffi::c_int + k) as uint16_t;
    borg_fear_region[y1 as usize][x0 as usize] = (borg_fear_region[y1 as usize][x0 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x0 as usize] = (borg_fear_region[y2 as usize][x0 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y1 as usize][x1 as usize] = (borg_fear_region[y1 as usize][x1 as usize]
        as ::core::ffi::c_int
        + k / 2 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y1 as usize][x2 as usize] = (borg_fear_region[y1 as usize][x2 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x1 as usize] = (borg_fear_region[y2 as usize][x1 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
    borg_fear_region[y2 as usize][x2 as usize] = (borg_fear_region[y2 as usize][x2 as usize]
        as ::core::ffi::c_int
        + k / 3 as ::core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn borg_fear_spell(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ouch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if borg.oldchp as ::core::ffi::c_int
        > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        ouch = (borg.oldchp as ::core::ffi::c_int
            - *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize))
            * 2 as ::core::ffi::c_int;
    }
    match i {
        1 => {
            p += 10 as ::core::ffi::c_int;
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 5 as ::core::ffi::c_int
            {
                borg.goal.leaving = true_0 != 0;
                borg.goal.fleeing = borg.goal.leaving;
            }
        }
        2 => {
            p = 100 as ::core::ffi::c_int;
        }
        4 => {
            z = ouch;
        }
        5 => {
            z = ouch;
        }
        6 => {
            z = ouch;
        }
        7 => {
            if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        8 => {
            if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        9 => {
            if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        10 => {
            if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        11 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0) {
                if !borg.temp.res_pois {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        12 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
                if !(*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0) {
                    if !(*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        >= 50 as ::core::ffi::c_int)
                    {
                        p += 150 as ::core::ffi::c_int;
                    }
                }
            }
        }
        13 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0) {
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        14 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0) {
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        15 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        16 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0) {
                p += 200 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) == 0 {
                    p += 50 as ::core::ffi::c_int;
                }
                if !(*borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    == 50 as ::core::ffi::c_int)
                {
                    p += 100 as ::core::ffi::c_int;
                }
            }
        }
        17 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0) {
                p += 500 as ::core::ffi::c_int;
            }
        }
        18 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0) {
                p += 100 as ::core::ffi::c_int;
            }
        }
        19 => {
            z = ouch;
            p += 200 as ::core::ffi::c_int;
        }
        20 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        21 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
            if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        22 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        23 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        24 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                p += 50 as ::core::ffi::c_int;
            }
        }
        25 => {
            p += 50 as ::core::ffi::c_int;
        }
        26 => {
            z = ouch;
            p += 40 as ::core::ffi::c_int;
        }
        28 => {
            if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        29 => {
            if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        30 => {
            if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        31 => {
            if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        32 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0) {
                p += 20 as ::core::ffi::c_int;
            }
        }
        33 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0) {
                p += 20 as ::core::ffi::c_int;
            }
        }
        34 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                p += 300 as ::core::ffi::c_int;
            }
        }
        35 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        36 => {
            z = ouch;
        }
        37 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        38 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0) {
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        39 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0) {
                if !(*borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0)
                {
                    p += 20 as ::core::ffi::c_int;
                }
            }
        }
        40 => {
            z = ouch;
            if !(*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0) {
                if *borg
                    .trait_0
                    .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                    != 0
                {
                    p += 50 as ::core::ffi::c_int;
                }
                if *borg
                    .trait_0
                    .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                    != 0
                {
                    p += 100 as ::core::ffi::c_int;
                }
                if !(*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0) {
                    p += 100 as ::core::ffi::c_int;
                }
            }
        }
        41 => {
            if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0 {
                p += 10 as ::core::ffi::c_int;
            }
        }
        42 => {
            z = 20 as ::core::ffi::c_int;
        }
        43 => {
            z = 12 as ::core::ffi::c_int * 15 as ::core::ffi::c_int;
            p += 100 as ::core::ffi::c_int;
        }
        44 => {
            z = 12 as ::core::ffi::c_int * 15 as ::core::ffi::c_int;
            p += 100 as ::core::ffi::c_int;
        }
        45 => {
            if !(*borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        46 => {
            if !(*borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        47 => {
            if !(*borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 40 as ::core::ffi::c_int;
            }
        }
        48 => {
            if !(*borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        49 => {
            if !(*borg.trait_0.offset(BI_IPOIS as ::core::ffi::c_int as isize) != 0) {
                z = ouch;
                p += 20 as ::core::ffi::c_int;
            }
        }
        50 => {
            z = ouch + 100 as ::core::ffi::c_int;
            if !(*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0) {
                p += 200 as ::core::ffi::c_int;
            }
        }
        51 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        52 => {
            z = ouch;
        }
        53 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        54 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        55 => {
            z = ouch;
        }
        56 => {
            z = ouch;
            p += 20 as ::core::ffi::c_int;
        }
        57 => {
            z = ouch;
            p += 50 as ::core::ffi::c_int;
        }
        58 => {
            p += 10 as ::core::ffi::c_int;
        }
        59 => {
            p += 10 as ::core::ffi::c_int;
        }
        60 => {
            p += 10 as ::core::ffi::c_int;
        }
        61 => {
            p += 5 as ::core::ffi::c_int;
        }
        62 => {
            p += 20 as ::core::ffi::c_int;
        }
        63 => {
            p += 10 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize);
        }
        64 => {
            p += 10 as ::core::ffi::c_int;
        }
        67 => {
            p += 10 as ::core::ffi::c_int;
        }
        68 => {
            p += 20 as ::core::ffi::c_int;
        }
        69 => {
            p += 20 as ::core::ffi::c_int;
        }
        70 => {
            p += 10 as ::core::ffi::c_int;
        }
        71 => {
            p += 25 as ::core::ffi::c_int;
        }
        3 | 27 | 65 | 66 | 72 => {}
        73 => {
            p += 50 as ::core::ffi::c_int;
        }
        74 => {
            if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                > 10 as ::core::ffi::c_int
            {
                p += 500 as ::core::ffi::c_int;
            } else {
                p += 30 as ::core::ffi::c_int;
            }
        }
        75 => {
            p += 200 as ::core::ffi::c_int;
        }
        76 => {
            p += 55 as ::core::ffi::c_int;
        }
        77 => {
            p += 95 as ::core::ffi::c_int;
        }
        78 => {
            p += 55 as ::core::ffi::c_int;
        }
        79 => {
            p += 30 as ::core::ffi::c_int;
        }
        80 => {
            p += 15 as ::core::ffi::c_int;
        }
        81 => {
            p += 25 as ::core::ffi::c_int;
        }
        82 => {
            p += 45 as ::core::ffi::c_int;
        }
        83 => {
            p += 70 as ::core::ffi::c_int;
        }
        84 => {
            p += 80 as ::core::ffi::c_int;
        }
        85 => {
            p += 80 as ::core::ffi::c_int;
        }
        86 => {
            p += 80 as ::core::ffi::c_int;
        }
        87 => {
            p += 80 as ::core::ffi::c_int;
        }
        88 => {
            p += 95 as ::core::ffi::c_int;
        }
        89 => {
            p += 95 as ::core::ffi::c_int;
        }
        90 => {
            p += 95 as ::core::ffi::c_int;
        }
        91 => {
            p += 50 as ::core::ffi::c_int;
        }
        _ => {
            p += 1000 as ::core::ffi::c_int;
            borg_note(b"# Mystery attack.  BUG.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    if ouch
        >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        ouch = ouch * 2 as ::core::ffi::c_int;
    }
    return p
        + z
        + *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_handle_self(mut str: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    q_x = o_w_x / borg_panel_wid();
    q_y = o_w_y / borg_panel_hgt();
    if borg_failure {
        borg_note(b"# Something failed\0" as *const u8 as *const ::core::ffi::c_char);
    } else if prefix(str, b"lite\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Called lite at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            old_c.y,
            old_c.x,
        ));
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 {
            y = borg.c.y - 1 as ::core::ffi::c_int;
            while y <= borg.c.y + 1 as ::core::ffi::c_int {
                x = borg.c.x - 1 as ::core::ffi::c_int;
                while x <= borg.c.x + 1 as ::core::ffi::c_int {
                    let mut ag: *mut borg_grid =
                        (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                    (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
                    x += 1;
                }
                y += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < borg_light_n as ::core::ffi::c_int {
            x = borg_light_x[i as usize] as ::core::ffi::c_int;
            y = borg_light_y[i as usize] as ::core::ffi::c_int;
            let mut ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y as isize))
            .offset(x as isize) as *mut borg_grid;
            (*ag_0).info = ((*ag_0).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
            (*ag_0).info = ((*ag_0).info as ::core::ffi::c_int & !BORG_DARK) as uint16_t;
            i += 1;
        }
    } else if prefix(str, b"wall\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected walls (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"trap\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"door\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected doors (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"both\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps and doors (%d,%d to %d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"TDE\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected traps, doors & evil (%d,%d to %d,%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_trap.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_door.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"evil\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected evil (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_evil.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    } else if prefix(str, b"obj\0" as *const u8 as *const ::core::ffi::c_char) {
        borg_note(format(
            b"# Detected objects (%d,%d to %d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            q_y,
            q_x,
            q_y + 1 as ::core::ffi::c_int,
            q_x + 1 as ::core::ffi::c_int,
        ));
        *(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 0 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 0 as ::core::ffi::c_int) as isize) = true_0 != 0;
        *(*borg_detect_obj.offset((q_y + 1 as ::core::ffi::c_int) as isize))
            .offset((q_x + 1 as ::core::ffi::c_int) as isize) = true_0 != 0;
    }
}
unsafe extern "C" fn intcomp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return *(a as *mut ::core::ffi::c_int) - *(b as *mut ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_update() {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut max_x: ::core::ffi::c_int = 0;
    let mut max_y: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut u_i: ::core::ffi::c_uint = 0;
    let mut hit_dist: ::core::ffi::c_int = 0;
    let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut what: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut reset: bool = false_0 != 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut floor_grid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut floor_glyphed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut monster_in_vault: bool = false_0 != 0;
    let mut created_traps: bool = false_0 != 0;
    w_y = (*Term).offset_y;
    w_x = (*Term).offset_x;
    borg.panels.x = 1 as ::core::ffi::c_int;
    if (*cave).width > borg_panel_wid() {
        borg.panels.x = ((*cave).width - borg_panel_wid()) * 2 as ::core::ffi::c_int
            / borg_panel_wid()
            + 1 as ::core::ffi::c_int;
    }
    borg.panels.y = 1 as ::core::ffi::c_int;
    if (*cave).height > borg_panel_hgt() {
        borg.panels.y = ((*cave).height - borg_panel_hgt()) * 2 as ::core::ffi::c_int
            / borg_panel_hgt()
            + 1 as ::core::ffi::c_int;
    }
    if o_w_x > (*cave).width || o_w_y > (*cave).height {
        o_w_y = 0 as ::core::ffi::c_int;
        o_w_x = o_w_y;
    }
    borg_alloc_detection();
    let mut current_block_16: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            (*kill).seen = false_0 != 0;
            (*kill).used = false_0 != 0;
            if borg.time.now - (*kill).when < 2000 as borg_time {
                if !(*borg
                    .trait_0
                    .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                    != 0
                    && *borg
                        .trait_0
                        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                        != 0)
                {
                    current_block_16 = 3512920355445576850;
                } else {
                    current_block_16 = 13797916685926291137;
                }
            } else {
                current_block_16 = 13797916685926291137;
            }
            match current_block_16 {
                3512920355445576850 => {}
                _ => {
                    borg_note(format(
                        b"# Expiring a monster '%s' (%d) at (%d,%d)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                        (*kill).r_idx as ::core::ffi::c_int,
                        (*kill).pos.y,
                        (*kill).pos.x,
                    ));
                    borg_delete_kill(i);
                }
            }
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take).kidx == 0) {
            (*take).seen = false_0 != 0;
            if (*take).y as ::core::ffi::c_int == borg.c.y
                && (*take).x as ::core::ffi::c_int == borg.c.x
                || (*take).y as ::core::ffi::c_int == old_c.y
                    && (*take).x as ::core::ffi::c_int == old_c.x
                || *borg
                    .trait_0
                    .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                    != 0
            {
                borg_delete_take(i);
            } else if !(borg.time.now - (*take).when < 2000 as borg_time) {
                borg_note(format(
                    b"# Expiring an object '%s' (%lu) at (%d,%d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*k_info.offset((*take).kidx as isize)).name,
                    (*k_info.offset((*take).kidx as isize)).kidx as ::core::ffi::c_ulong,
                    (*take).y as ::core::ffi::c_int,
                    (*take).x as ::core::ffi::c_int,
                ));
                borg_delete_take(i);
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        msg = borg_msg_buf.offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
        borg_note(format(
            b"# %s (+)\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"SELF:\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_handle_self(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            } else if prefix(
                msg,
                b"FEELING_DANGER:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_feeling_danger = atoi(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            } else if prefix(
                msg,
                b"FEELING_STUFF:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_feeling_stuff = atoi(what);
                *borg_msg_use.offset(i as isize) = 1 as int16_t;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(msg, b"MISS:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(msg, b"AFRAID:\0" as *const u8 as *const ::core::ffi::c_char) {
                if (*borg_grids[borg.goal.g.y as usize].offset(borg.goal.g.x as isize)).kill
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                } else {
                    borg_create_kill(what, borg.goal.g);
                }
            } else if prefix(msg, b"KILL:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"BLINK:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 3 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 3 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.goal.g, 0 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 2 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(
                msg,
                b"STATE_CONFUSED:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            }
        }
        i += 1;
    }
    hit_dist = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            created_traps = false_0 != 0;
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                let mut spell: ::core::ffi::c_int =
                    atoi(msg.offset(6 as ::core::ffi::c_int as isize));
                if spell == RSF_TELE_AWAY as ::core::ffi::c_int
                    || spell == RSF_TELE_TO as ::core::ffi::c_int
                {
                    hit_dist = 100 as ::core::ffi::c_int;
                }
                break;
            } else if prefix(msg, b"QUAKE:\0" as *const u8 as *const ::core::ffi::c_char) {
                hit_dist = 3 as ::core::ffi::c_int;
                break;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"MISS:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"KILL:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"BLINK:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.goal.g, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target == -(1 as ::core::ffi::c_int) {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                    borg.antibounce_count = 1 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    successful_target = 2 as ::core::ffi::c_int;
                }
            } else if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 1 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_sleep_kill(k);
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_AWAKE:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, old_c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 3 as int16_t;
                }
                let mut spell_0: ::core::ffi::c_int =
                    atoi(msg.offset(6 as ::core::ffi::c_int as isize));
                if spell_0 == RSF_TRAPS as ::core::ffi::c_int {
                    created_traps = true_0 != 0;
                }
            }
            if created_traps {
                *(*borg_detect_trap
                    .offset((o_w_y / borg_panel_hgt() + 0 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / borg_panel_wid() + 0 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / borg_panel_hgt() + 0 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / borg_panel_wid() + 1 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / borg_panel_hgt() + 1 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / borg_panel_wid() + 0 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                *(*borg_detect_trap
                    .offset((o_w_y / borg_panel_hgt() + 1 as ::core::ffi::c_int) as isize))
                .offset((o_w_x / borg_panel_wid() + 1 as ::core::ffi::c_int) as isize) =
                    false_0 != 0;
                borg_note(
                    b"# Logging need for use of Detect Traps.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ii = 0 as ::core::ffi::c_int;
                while ii < 8 as ::core::ffi::c_int {
                    x = borg.c.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                    y = borg.c.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                        .offset(x as isize) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                        if borg_cave_floor_grid(ag) {
                            (*ag).trap = true_0 != 0;
                            borg_note(format(
                                b"# Assuming a Traps at (%d,%d).\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                y,
                                x,
                            ));
                        }
                    }
                    ii += 1;
                }
            }
        }
        i += 1;
    }
    if successful_target < 0 as ::core::ffi::c_int {
        if successful_target > -(10 as ::core::ffi::c_int) {
            successful_target -= 10 as ::core::ffi::c_int;
            if borg_target_unknown_wall(borg.goal.g.y, borg.goal.g.x) {
                successful_target = 2 as ::core::ffi::c_int;
            }
        }
    }
    if borg.status.old_depth as ::core::ffi::c_int
        != *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
    {
        borg.time.level = borg.time.now;
        borg.time.morgoth = 0 as ::core::ffi::c_int as borg_time;
        borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
        if borg.status.old_depth == 0 {
            borg.time.town = borg.time.now;
        }
        borg.antibounce_count = 1 as int16_t;
        borg.status.vault = false_0 != 0;
        borg.mon.unique = 0 as ::core::ffi::c_uint;
        borg.mon.scary = false_0 != 0;
        borg.near.breeder = false_0 != 0;
        borg.need_see_invis = 0 as ::core::ffi::c_int as borg_time;
        successful_target = 0 as ::core::ffi::c_int;
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        sold_item_num = 0 as ::core::ffi::c_int;
        sold_item_nxt = 1 as ::core::ffi::c_int;
        bought_item_nxt = 1 as ::core::ffi::c_int;
        bought_item_num = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            sold_item_tval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_sval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_pval[i as usize] = 0 as ::core::ffi::c_int;
            sold_item_store[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_tval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_sval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_pval[i as usize] = 0 as ::core::ffi::c_int;
            bought_item_store[i as usize] = 0 as ::core::ffi::c_int;
            i += 1;
        }
        borg_do_update_view = true_0 != 0;
        borg_do_update_lite = true_0 != 0;
        borg_do_spell = true_0 != 0;
        borg_do_crush_junk = true_0 != 0;
        borg.time.call_light = 0 as ::core::ffi::c_int as borg_time;
        borg.time.wizard_light = 0 as ::core::ffi::c_int as borg_time;
        borg.time.detect_traps = 0 as ::core::ffi::c_int as borg_time;
        borg.time.detect_doors = 0 as ::core::ffi::c_int as borg_time;
        borg.time.detect_walls = 0 as ::core::ffi::c_int as borg_time;
        borg.time.detect_evil = 0 as ::core::ffi::c_int as borg_time;
        borg.time.detect_obj = 0 as ::core::ffi::c_int as borg_time;
        y = 0 as ::core::ffi::c_int;
        while y < borg.panels.y {
            x = 0 as ::core::ffi::c_int;
            while x < borg.panels.x {
                *(*borg_detect_wall.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_trap.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_door.offset(y as isize)).offset(x as isize) = false_0 != 0;
                *(*borg_detect_evil.offset(y as isize)).offset(x as isize) = false_0 != 0;
                x += 1;
            }
            y += 1;
        }
        max_x = AUTO_MAX_X / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        max_y = AUTO_MAX_Y / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        y = 0 as ::core::ffi::c_int;
        while y < max_y {
            x = 0 as ::core::ffi::c_int;
            while x < max_x {
                borg_fear_region[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        y = 0 as ::core::ffi::c_int;
        while y < AUTO_MAX_Y {
            x = 0 as ::core::ffi::c_int;
            while x < AUTO_MAX_X {
                borg_fear_monsters[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        borg.goal.type_0 = 0 as int16_t;
        borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
        borg.goal.ware = borg.goal.item;
        borg.goal.shop = borg.goal.ware;
        borg.stair_more = false_0 != 0;
        borg.stair_less = borg.stair_more;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        {
            borg.goal.rising = false_0 != 0;
        }
        borg.goal.leaving = false_0 != 0;
        borg.goal.fleeing = false_0 != 0;
        borg.goal.fleeing_lunal = false_0 != 0;
        borg.goal.fleeing_munchkin = false_0 != 0;
        borg.goal.ignoring = false_0 != 0;
        borg_feeling_stuff = 0 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        {
            borg_clear_best();
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        {
            borg.goal.fleeing_to_town = false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 2 as ::core::ffi::c_int
        {
            borg.goal.fleeing_to_town = false_0 != 0;
        }
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        track_glyph.num = 0 as int16_t;
        track_step.num = 0 as int16_t;
        track_door.num = 0 as int16_t;
        track_closed.num = 0 as int16_t;
        track_vein.num = 0 as int16_t;
        track_worn_num = 0 as int16_t;
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        memset(
            borg_takes as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<borg_take>() as size_t),
        );
        borg_kills_cnt = 0 as int16_t;
        borg_kills_nxt = 1 as int16_t;
        borg.near.morgoth = false_0 != 0;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 100 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) == 0
            || borg.mon.unique == borg.id.morgoth
        {
            borg.near.morgoth = true_0 != 0;
            borg_needs_new_sea = true_0 != 0;
        }
        i = 0 as ::core::ffi::c_int;
        while i < borg_nasties_num as ::core::ffi::c_int {
            borg_nasties_count[i as usize] = 0 as uint8_t;
            if borg_nasties[i as usize] as ::core::ffi::c_int == 'Z' as i32 {
                borg_nasties_count[i as usize] = 25 as uint8_t;
            }
            i += 1;
        }
        memset(
            borg_kills as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<borg_kill>() as size_t),
        );
        memset(
            borg_race_count as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*z_info).r_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
        borg_numb_live_unique = 0 as ::core::ffi::c_int;
        borg_first_living_unique = 0 as ::core::ffi::c_uint;
        borg_depth_hunted_unique = 0 as ::core::ffi::c_int;
        let mut unique_depths: [::core::ffi::c_int; 4] = [
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ];
        u_i = 1 as ::core::ffi::c_uint;
        while u_i
            < ((*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            let mut r_ptr: *mut monster_race = r_info.offset(u_i as isize) as *mut monster_race;
            if !(*r_ptr).name.is_null() {
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
                    if (*r_ptr).max_num as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *borg_race_death.offset(u_i as isize) = 1 as int16_t;
                    }
                    if !(*borg_race_death.offset(u_i as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int)
                    {
                        if !(flag_has(
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
                        ) as ::core::ffi::c_int
                            != 0
                            || flag_has(
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
                                RF_SEASONAL as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int
                                != 0)
                        {
                            if !((*r_ptr).rarity == 0) {
                                if borg_numb_live_unique < 4 as ::core::ffi::c_int {
                                    unique_depths[borg_numb_live_unique as usize] = (*r_ptr).level;
                                    borg_numb_live_unique += 1;
                                    if borg_numb_live_unique == 1 as ::core::ffi::c_int {
                                        borg_first_living_unique = u_i;
                                    }
                                    if borg_numb_live_unique == 3 as ::core::ffi::c_int {
                                        qsort(
                                            &raw mut unique_depths as *mut ::core::ffi::c_int
                                                as *mut ::core::ffi::c_void,
                                            3 as size_t,
                                            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                            Some(
                                                intcomp
                                                    as unsafe extern "C" fn(
                                                        *const ::core::ffi::c_void,
                                                        *const ::core::ffi::c_void,
                                                    )
                                                        -> ::core::ffi::c_int,
                                            ),
                                        );
                                    }
                                } else {
                                    borg_numb_live_unique += 1;
                                    if (*r_ptr).level
                                        < unique_depths[2 as ::core::ffi::c_int as usize]
                                    {
                                        unique_depths[3 as ::core::ffi::c_int as usize] =
                                            (*r_ptr).level;
                                        qsort(
                                            &raw mut unique_depths as *mut ::core::ffi::c_int
                                                as *mut ::core::ffi::c_void,
                                            4 as size_t,
                                            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                            Some(
                                                intcomp
                                                    as unsafe extern "C" fn(
                                                        *const ::core::ffi::c_void,
                                                        *const ::core::ffi::c_void,
                                                    )
                                                        -> ::core::ffi::c_int,
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            u_i = u_i.wrapping_add(1);
        }
        if borg_numb_live_unique >= 3 as ::core::ffi::c_int {
            borg_depth_hunted_unique = unique_depths[2 as ::core::ffi::c_int as usize];
        } else if borg_numb_live_unique != 0 as ::core::ffi::c_int {
            borg_depth_hunted_unique =
                unique_depths[(borg_numb_live_unique - 1 as ::core::ffi::c_int) as usize];
        } else {
            borg_depth_hunted_unique = 127 as ::core::ffi::c_int;
        }
        borg_forget_map();
        reset = true_0 != 0;
        if *borg_cfg.offset(BORG_AUTOSAVE as ::core::ffi::c_int as isize) != 0
            && !borg.lunal_mode
            && !borg.munchkin_mode
        {
            borg.status.save = true_0 != 0;
        }
        borg.status.old_depth = *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            as int16_t;
        borg.times_twitch = 0 as int16_t;
        borg.escapes = 0 as int16_t;
    } else {
        if borg.resistance as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            borg.resistance = (borg.resistance as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        if borg.no_rest_prep as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            borg.no_rest_prep = (borg.no_rest_prep as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        if borg.goal.recalling >= 1 as ::core::ffi::c_int {
            borg.goal.recalling -= borg_game_ratio as ::core::ffi::c_int;
            if borg.goal.recalling <= 0 as ::core::ffi::c_int {
                borg.goal.recalling = 1 as ::core::ffi::c_int;
            }
        }
        if borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            borg.see_inv = (borg.see_inv as ::core::ffi::c_int
                - borg_game_ratio as ::core::ffi::c_int) as int16_t;
        }
        borg.near.morgoth = false_0 != 0;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 100 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) == 0
            || borg.mon.unique == borg.id.morgoth
        {
            borg.near.morgoth = true_0 != 0;
        }
        if borg.near.morgoth as ::core::ffi::c_int != 0
            && borg.time.now - borg.time.level >= 500 as borg_time
        {
            if borg.mon.unique != borg.id.morgoth {
                borg.near.morgoth = false_0 != 0;
            }
            if borg.mon.unique == borg.id.morgoth
                && borg.time.now - borg.time.morgoth > 500 as borg_time
            {
                borg_note(format(
                    b"# Morgoth has not been seen in %ld turns.  Going to hunt him.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (borg.time.now - borg.time.morgoth) as ::core::ffi::c_long,
                ));
                borg.near.morgoth = false_0 != 0;
            }
            if borg.time.now - borg.time.morgoth > 2500 as borg_time {
                borg_note(format(
                    b"# Morgoth has not been seen in %ld turns.  No show.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (borg.time.now - borg.time.morgoth) as ::core::ffi::c_long,
                ));
                borg.mon.unique = 0 as ::core::ffi::c_uint;
            }
        }
        if borg.resistance as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            && borg.temp.res_fire as ::core::ffi::c_int != 0
            && borg.temp.res_elec as ::core::ffi::c_int != 0
            && borg.temp.res_cold as ::core::ffi::c_int != 0
            && borg.temp.res_acid as ::core::ffi::c_int != 0
            && borg.temp.res_pois as ::core::ffi::c_int != 0
        {
            borg.resistance = 20000 as int16_t;
        }
        if borg.resistance as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && borg.temp.res_fire as ::core::ffi::c_int
                + borg.temp.res_elec as ::core::ffi::c_int
                + borg.temp.res_cold as ::core::ffi::c_int
                + borg.temp.res_acid as ::core::ffi::c_int
                + borg.temp.res_pois as ::core::ffi::c_int
                != 5 as ::core::ffi::c_int
        {
            borg.resistance = 0 as int16_t;
        }
        if borg.time.now % 10 as borg_time == 0 {
            max_x = AUTO_MAX_X / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            max_y = AUTO_MAX_Y / 11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            y = 0 as ::core::ffi::c_int;
            while y < max_y {
                x = 0 as ::core::ffi::c_int;
                while x < max_x {
                    if borg_fear_region[y as usize][x as usize] != 0 {
                        borg_fear_region[y as usize][x as usize] =
                            borg_fear_region[y as usize][x as usize].wrapping_sub(1);
                    }
                    x += 1;
                }
                y += 1;
            }
        }
        y = 0 as ::core::ffi::c_int;
        while y < AUTO_MAX_Y {
            x = 0 as ::core::ffi::c_int;
            while x < AUTO_MAX_X {
                borg_fear_monsters[y as usize][x as usize] = 0 as uint16_t;
                x += 1;
            }
            y += 1;
        }
        if o_w_x != w_x || o_w_y != w_y {
            dy = 0 as ::core::ffi::c_int;
            while dy
                < ((*Term).hgt
                    - row_top_map[(*Term).sidebar_mode as usize]
                    - row_bottom_map[(*Term).sidebar_mode as usize])
                    / tile_height as ::core::ffi::c_int
            {
                dx = 0 as ::core::ffi::c_int;
                while dx
                    < ((*Term).wid
                        - col_map[(*Term).sidebar_mode as usize]
                        - 1 as ::core::ffi::c_int)
                        / tile_width as ::core::ffi::c_int
                {
                    x = o_w_x + dx;
                    y = o_w_y + dy;
                    if !(y >= AUTO_MAX_Y || x >= AUTO_MAX_X) {
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_OKAY) as uint16_t;
                    }
                    dx += 1;
                }
                dy += 1;
            }
            borg.antibounce_count = 1 as int16_t;
        }
    }
    borg_wank_num = 0 as ::core::ffi::c_int;
    borg_update_map();
    *track_step.x.offset(track_step.num as isize) = borg.c.x;
    *track_step.y.offset(track_step.num as isize) = borg.c.y;
    track_step.num += 1;
    if track_step.num as ::core::ffi::c_int >= 75 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= 75 as ::core::ffi::c_int {
            *track_step.x.offset(i as isize) =
                *track_step.x.offset((i + 1 as ::core::ffi::c_int) as isize);
            *track_step.y.offset(i as isize) =
                *track_step.y.offset((i + 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        track_step.num = 75 as int16_t;
    }
    if reset {
        o_w_x = w_x;
        o_w_y = w_y;
        old_c = borg.c;
        borg.goal.g = borg.c;
    }
    if old_c.x != borg.c.x || old_c.y != borg.c.y {
        borg_do_update_view = true_0 != 0;
        borg_do_update_lite = true_0 != 0;
        successful_target = 0 as ::core::ffi::c_int;
    }
    if borg_do_update_view {
        borg_update_view();
        borg_do_update_view = false_0 != 0;
    }
    if borg_do_update_lite {
        borg_update_light();
        borg_do_update_lite = false_0 != 0;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill_0 = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_0).r_idx == 0) {
            if !flag_has(
                &raw mut (*r_info.offset((*kill_0).r_idx as isize)).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_PASS_WALL as ::core::ffi::c_int,
            ) {
                (*borg_grids[(*kill_0).pos.y as usize].offset((*kill_0).pos.x as isize)).feat =
                    FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
            }
        }
        i += 1;
    }
    borg.morgoth_position = false_0 != 0;
    if *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) == 0
        && borg.near.morgoth as ::core::ffi::c_int != 0
    {
        if borg.c.y >= 15 as ::core::ffi::c_int
            && borg.c.y <= AUTO_MAX_Y - 15 as ::core::ffi::c_int
            && borg.c.x >= 50 as ::core::ffi::c_int
            && borg.c.x <= AUTO_MAX_X - 50 as ::core::ffi::c_int
        {
            j = 0 as ::core::ffi::c_int;
            while j < 24 as ::core::ffi::c_int {
                y = borg.c.y + borg_ddy_ddd[j as usize] as ::core::ffi::c_int;
                x = borg.c.x + borg_ddx_ddd[j as usize] as ::core::ffi::c_int;
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                    .offset(x as isize) as *mut borg_grid;
                if (*ag).glyph {
                    floor_glyphed += 1;
                }
                j += 1;
            }
            if floor_glyphed == 24 as ::core::ffi::c_int {
                borg.morgoth_position = true_0 != 0;
            }
        }
    }
    j = 0 as ::core::ffi::c_int;
    while j < 24 as ::core::ffi::c_int {
        y = borg.c.y + borg_ddy_ddd[j as usize] as ::core::ffi::c_int;
        x = borg.c.x + borg_ddx_ddd[j as usize] as ::core::ffi::c_int;
        if !square_in_bounds(cave, loc(x, y)) {
            floor_grid += 1;
        } else {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if j <= 7 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
            {
                floor_grid += 1;
            }
            if j >= 8 as ::core::ffi::c_int
                && (*ag).feat as ::core::ffi::c_int <= FEAT_MORE as ::core::ffi::c_int
                && borg_los(borg.c.y, borg.c.x, y, x) as ::core::ffi::c_int != 0
            {
                floor_grid += 1;
            }
        }
        j += 1;
    }
    if floor_grid == 1 as ::core::ffi::c_int {
        borg.status.anti_summon = true_0 != 0;
    } else {
        borg.status.anti_summon = false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 5 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
        && track_closed.num as ::core::ffi::c_int != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_closed.num as ::core::ffi::c_int {
            x = *track_closed.x.offset(i as isize);
            y = *track_closed.y.offset(i as isize);
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
            {
                borg_note(format(
                    b"# Monster opened door at %d,%d.  That's scary.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    y,
                    x,
                ));
                borg.mon.scary = true_0 != 0;
            }
            i += 1;
        }
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank).y as ::core::ffi::c_int,
                (*wank).x as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*wank).t_a,
                (*wank).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank).is_kill = false_0 != 0;
            if !(*wank).is_take && !(*wank).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_0: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_0).is_take as ::core::ffi::c_int != 0
            && observe_take_move(
                (*wank_0).y as ::core::ffi::c_int,
                (*wank_0).x as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*wank_0).t_a,
                (*wank_0).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_0).is_take = false_0 != 0;
            if !(*wank_0).is_take && !(*wank_0).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_1: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_1).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_1).y as ::core::ffi::c_int,
                (*wank_1).x as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                (*wank_1).t_a,
                (*wank_1).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_1).is_kill = false_0 != 0;
            if !(*wank_1).is_take && !(*wank_1).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_2: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_2).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_2).y as ::core::ffi::c_int,
                (*wank_2).x as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                (*wank_2).t_a,
                (*wank_2).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_2).is_kill = false_0 != 0;
            if !(*wank_2).is_take && !(*wank_2).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_3: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_3).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_3).y as ::core::ffi::c_int,
                (*wank_3).x as ::core::ffi::c_int,
                3 as ::core::ffi::c_int,
                (*wank_3).t_a,
                (*wank_3).t_c,
                false_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_3).is_kill = false_0 != 0;
            if !(*wank_3).is_take && !(*wank_3).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_4: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_4).is_kill as ::core::ffi::c_int != 0
            && observe_kill_move(
                (*wank_4).y as ::core::ffi::c_int,
                (*wank_4).x as ::core::ffi::c_int,
                7 as ::core::ffi::c_int,
                (*wank_4).t_a,
                (*wank_4).t_c,
                true_0 != 0,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_4).is_kill = false_0 != 0;
            if !(*wank_4).is_take && !(*wank_4).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_5: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_5).is_take as ::core::ffi::c_int != 0
            && observe_take_diff(
                (*wank_5).y as ::core::ffi::c_int,
                (*wank_5).x as ::core::ffi::c_int,
                (*wank_5).t_a,
                (*wank_5).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_5).is_take = false_0 != 0;
            if !(*wank_5).is_take && !(*wank_5).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = borg_wank_num - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wank_6: *mut borg_wank = borg_wanks.offset(i as isize) as *mut borg_wank;
        if (*wank_6).is_kill as ::core::ffi::c_int != 0
            && observe_kill_diff(
                (*wank_6).y as ::core::ffi::c_int,
                (*wank_6).x as ::core::ffi::c_int,
                (*wank_6).t_a,
                (*wank_6).t_c,
            ) as ::core::ffi::c_int
                != 0
        {
            (*wank_6).is_kill = false_0 != 0;
            if !(*wank_6).is_take && !(*wank_6).is_kill {
                borg_wank_num -= 1;
                *borg_wanks.offset(i as isize) = *borg_wanks.offset(borg_wank_num as isize);
            }
        }
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"DIED:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_count_death(k);
                    borg_delete_kill(k);
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"PAIN:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.c, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.c, hit_dist);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_SLEEP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    borg_sleep_kill(k);
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE_AWAKE:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__FEAR:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(
                msg,
                b"STATE__BOLD:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                k = borg_locate_kill(what, borg.c, 20 as ::core::ffi::c_int);
                if k > 0 as ::core::ffi::c_int {
                    *borg_msg_use.offset(i as isize) = 4 as int16_t;
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        if !(*borg_msg_use.offset(i as isize) != 0) {
            msg = borg_msg_buf
                .offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
            what = strchr(msg, ':' as i32).offset(1 as ::core::ffi::c_int as isize);
            if prefix(msg, b"HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_fear_regional(
                    what,
                    borg.c.y,
                    borg.c.x,
                    4 as ::core::ffi::c_int
                        * (*borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            / 5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            } else if prefix(
                msg,
                b"MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                borg_fear_regional(
                    what,
                    borg.c.y,
                    borg.c.x,
                    2 as ::core::ffi::c_int
                        * (*borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            / 5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            } else if prefix(msg, b"SPELL_\0" as *const u8 as *const ::core::ffi::c_char) {
                borg_fear_regional(
                    what,
                    borg.c.y,
                    borg.c.x,
                    borg_fear_spell(atoi(msg.offset(6 as ::core::ffi::c_int as isize))),
                    false_0 != 0,
                );
                *borg_msg_use.offset(i as isize) = 5 as int16_t;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_msg_num as ::core::ffi::c_int {
        msg = borg_msg_buf.offset(*borg_msg_pos.offset(i as isize) as ::core::ffi::c_int as isize);
        borg_note(format(
            b"# %s (%d)\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
            *borg_msg_use.offset(i as isize) as ::core::ffi::c_int,
        ));
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_1: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_1).r_idx == 0) {
            if !((*kill_1).when == borg.time.now) {
                if !(*kill_1).seen {
                    if !(*borg
                        .trait_0
                        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                        != 0
                        || *borg
                            .trait_0
                            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                            != 0)
                    {
                        borg_follow_kill(i);
                    }
                }
            }
        }
        i += 1;
    }
    borg.mon.breeders = 0 as uint16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut p: ::core::ffi::c_int = 0;
        let mut kill_2: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut r_ptr_0: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        monster_in_vault = false_0 != 0;
        if !((*kill_2).r_idx == 0) {
            r_ptr_0 = r_info.offset((*kill_2).r_idx as isize) as *mut monster_race;
            if flag_has(
                &raw mut (*r_ptr_0).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_MULTIPLY as ::core::ffi::c_int,
            ) {
                borg.mon.breeders = borg.mon.breeders.wrapping_add(1);
            }
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
                RF_NEVER_MOVE as ::core::ffi::c_int,
            ) {
                if !(distance((*kill_2).pos, borg.c) >= 20 as ::core::ffi::c_int) {
                    if borg.status.vault {
                        ii = 0 as ::core::ffi::c_int;
                        while ii < 8 as ::core::ffi::c_int {
                            x = (*kill_2).pos.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                            y = (*kill_2).pos.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(x, y)) {
                                ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                    .offset(y as isize))
                                .offset(x as isize)
                                    as *mut borg_grid;
                                if !((*ag).feat as ::core::ffi::c_int
                                    == FEAT_NONE as ::core::ffi::c_int)
                                {
                                    if (*ag).feat as ::core::ffi::c_int
                                        == FEAT_PERM as ::core::ffi::c_int
                                    {
                                        monster_in_vault = true_0 != 0;
                                    }
                                }
                            }
                            ii += 1;
                        }
                    }
                    if !(monster_in_vault as ::core::ffi::c_int == true_0) {
                        p = borg_danger(
                            (*kill_2).pos.y,
                            (*kill_2).pos.x,
                            1 as ::core::ffi::c_int,
                            false_0 != 0,
                            false_0 != 0,
                        ) / 10 as ::core::ffi::c_int;
                        borg_fear_grid((*kill_2).pos.y, (*kill_2).pos.x, p);
                    }
                }
            }
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take_0: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if !((*take_0).kidx == 0) {
            if !(borg.time.now - (*take_0).when <= 2 as borg_time) {
                if !(*borg
                    .trait_0
                    .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                    != 0
                    || *borg
                        .trait_0
                        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                        != 0)
                {
                    borg_follow_take(i);
                }
            }
        }
        i += 1;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
            != 0
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    borg.oldchp = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
    borg_failure = false_0 != 0;
    borg_msg_len = 0 as int16_t;
    borg_msg_num = 0 as int16_t;
    old_c = borg.c;
    o_w_x = w_x;
    o_w_y = w_y;
    borg.goal.g = borg.c;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_detection() {
    let mut i: ::core::ffi::c_int = 0;
    let mut array_size_y: ::core::ffi::c_int = old_panels.y + 1 as ::core::ffi::c_int;
    if borg_detect_wall.is_null()
        && borg_detect_trap.is_null()
        && borg_detect_door.is_null()
        && borg_detect_evil.is_null()
        && borg_detect_obj.is_null()
    {
        return;
    }
    if !borg_detect_wall.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_wall.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_wall as *mut ::core::ffi::c_void);
        borg_detect_wall = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_trap.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_trap.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_trap as *mut ::core::ffi::c_void);
        borg_detect_trap = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_door.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_door.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_door as *mut ::core::ffi::c_void);
        borg_detect_door = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_evil.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_evil.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_evil as *mut ::core::ffi::c_void);
        borg_detect_evil = ::core::ptr::null_mut::<*mut bool>();
    }
    if !borg_detect_obj.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < array_size_y {
            mem_free(*borg_detect_obj.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        mem_free(borg_detect_obj as *mut ::core::ffi::c_void);
        borg_detect_obj = ::core::ptr::null_mut::<*mut bool>();
    }
    old_panels.x = -(1 as ::core::ffi::c_int);
    old_panels.y = -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_alloc_detection() {
    let mut i: ::core::ffi::c_int = 0;
    let mut array_size_y: ::core::ffi::c_int = borg.panels.y + 1 as ::core::ffi::c_int;
    let mut array_size_x: ::core::ffi::c_int = borg.panels.x + 1 as ::core::ffi::c_int;
    if old_panels.x == borg.panels.x && old_panels.y == borg.panels.y {
        return;
    }
    borg_free_detection();
    borg_detect_wall = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh1 = *borg_detect_wall.offset(i as isize);
        *fresh1 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_trap = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh2 = *borg_detect_trap.offset(i as isize);
        *fresh2 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_door = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh3 = *borg_detect_door.offset(i as isize);
        *fresh3 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_evil = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh4 = *borg_detect_evil.offset(i as isize);
        *fresh4 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    borg_detect_obj = mem_zalloc(
        (array_size_y as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < array_size_y {
        let ref mut fresh5 = *borg_detect_obj.offset(i as isize);
        *fresh5 = mem_zalloc(
            (array_size_x as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    old_panels = borg.panels;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_update() {
    borg_free_update();
    borg_wanks = mem_zalloc(
        (AUTO_VIEW_MAX as size_t).wrapping_mul(::core::mem::size_of::<borg_wank>() as size_t),
    ) as *mut borg_wank;
    borg_forget_map();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_update() {
    if !borg_wanks.is_null() {
        mem_free(borg_wanks as *mut ::core::ffi::c_void);
        borg_wanks = ::core::ptr::null_mut::<borg_wank>();
    }
}
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const AUTO_VIEW_MAX: ::core::ffi::c_int = 9000 as ::core::ffi::c_int;
pub const BORG_MARK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
