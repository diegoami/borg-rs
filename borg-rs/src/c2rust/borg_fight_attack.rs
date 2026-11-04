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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut k_info: *mut object_kind;
    static mut player: *mut player;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    fn borg_spell_okay(spell: borg_spells) -> bool;
    fn borg_get_spell_power(spell: borg_spells) -> ::core::ffi::c_int;
    fn borg_spell(spell: borg_spells) -> bool;
    fn borg_spell_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_spell_okay_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_spell_legal_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square_in_bounds_fully(c: *mut chunk, grid: loc) -> bool;
    static mut get_angle_to_grid: [[uint8_t; 41]; 41];
    static mut r_info: *mut monster_race;
    static mut slays: *mut slay;
    static mut brands: *mut brand;
    fn player_has_temporary_brand(p: *const player, idx: ::core::ffi::c_int) -> bool;
    fn player_has_temporary_slay(p: *const player, idx: ::core::ffi::c_int) -> bool;
    fn obj_kind_can_browse(kind: *const object_kind) -> bool;
    static adj_str_blow: [::core::ffi::c_int; 38];
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_grid(ag: *mut borg_grid) -> bool;
    fn borg_panel_hgt() -> ::core::ffi::c_int;
    fn borg_panel_wid() -> ::core::ffi::c_int;
    static mut borg_fear_region: [[uint16_t; 19]; 7];
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
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut borg_simulate: bool;
    static mut borg_attacking: bool;
    static mut borg_sleep_spell: bool;
    static mut borg_sleep_spell_ii: bool;
    static mut borg_crush_spell: bool;
    static mut borg_slow_spell: bool;
    static mut borg_confuse_spell: bool;
    static mut borg_fear_mon_spell: bool;
    static mut borg: borg_struct;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    static mut borg_flow_y: [uint8_t; 1536];
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
    static mut borg_kills_cnt: int16_t;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_extract_dir(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut borg_takes: *mut borg_take;
    fn borg_wield_slot(item: *const borg_item) -> ::core::ffi::c_int;
    fn borg_slot(tval: ::core::ffi::c_int, sval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    fn borg_queue_direction(k: keycode_t);
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
    static mut act_staff_holy: ::core::ffi::c_int;
    static mut act_wonder: ::core::ffi::c_int;
    static mut act_starlight2: ::core::ffi::c_int;
    static mut act_starlight: ::core::ffi::c_int;
    static mut act_light_line: ::core::ffi::c_int;
    static mut act_mon_scare: ::core::ffi::c_int;
    static mut act_sleep_all: ::core::ffi::c_int;
    static mut act_mon_confuse: ::core::ffi::c_int;
    static mut act_mon_slow: ::core::ffi::c_int;
    static mut act_confuse2: ::core::ffi::c_int;
    static mut act_stinking_cloud: ::core::ffi::c_int;
    static mut act_arrow: ::core::ffi::c_int;
    static mut act_mana_bolt: ::core::ffi::c_int;
    static mut act_missile: ::core::ffi::c_int;
    static mut act_drain_life4: ::core::ffi::c_int;
    static mut act_drain_life3: ::core::ffi::c_int;
    static mut act_drain_life2: ::core::ffi::c_int;
    static mut act_drain_life1: ::core::ffi::c_int;
    static mut act_elec_ball2: ::core::ffi::c_int;
    static mut act_elec_ball: ::core::ffi::c_int;
    static mut act_elec_bolt: ::core::ffi::c_int;
    static mut act_acid_ball: ::core::ffi::c_int;
    static mut act_acid_bolt2: ::core::ffi::c_int;
    static mut act_acid_bolt: ::core::ffi::c_int;
    static mut act_cold_ball160: ::core::ffi::c_int;
    static mut act_cold_ball100: ::core::ffi::c_int;
    static mut act_cold_ball50: ::core::ffi::c_int;
    static mut act_cold_ball2: ::core::ffi::c_int;
    static mut act_cold_bolt2: ::core::ffi::c_int;
    static mut act_cold_bolt: ::core::ffi::c_int;
    static mut act_fire_ball200: ::core::ffi::c_int;
    static mut act_fire_ball2: ::core::ffi::c_int;
    static mut act_fire_ball: ::core::ffi::c_int;
    static mut act_fire_ball72: ::core::ffi::c_int;
    static mut act_fire_bolt3: ::core::ffi::c_int;
    static mut act_fire_bolt2: ::core::ffi::c_int;
    static mut act_fire_bolt: ::core::ffi::c_int;
    static mut act_star_ball: ::core::ffi::c_int;
    static mut act_sleepii: ::core::ffi::c_int;
    static mut act_dispel_undead: ::core::ffi::c_int;
    static mut act_dispel_evil60: ::core::ffi::c_int;
    static mut act_dispel_evil: ::core::ffi::c_int;
    static mut act_losconf: ::core::ffi::c_int;
    static mut act_lossleep: ::core::ffi::c_int;
    static mut act_losslow: ::core::ffi::c_int;
    fn borg_item_note_needs_id(item: *const borg_item) -> bool;
    fn borg_equips_rod(sval: ::core::ffi::c_int) -> bool;
    fn borg_zap_rod(sval: ::core::ffi::c_int) -> bool;
    fn borg_use_staff(sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_staff_fail(sval: ::core::ffi::c_int) -> bool;
    fn borg_aim_wand(sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_dragon(drag_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_dragon(drag_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    fn borg_activate_failure(
        tval: ::core::ffi::c_int,
        sval: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut sv_potion_healing: ::core::ffi::c_int;
    static mut sv_potion_star_healing: ::core::ffi::c_int;
    static mut sv_potion_life: ::core::ffi::c_int;
    static mut kv_potion_restore_mana: ::core::ffi::c_int;
    static mut sv_potion_inc_str: ::core::ffi::c_int;
    static mut sv_potion_inc_int: ::core::ffi::c_int;
    static mut sv_potion_inc_wis: ::core::ffi::c_int;
    static mut sv_potion_inc_dex: ::core::ffi::c_int;
    static mut sv_potion_inc_con: ::core::ffi::c_int;
    static mut sv_ring_flames: ::core::ffi::c_int;
    static mut sv_ring_ice: ::core::ffi::c_int;
    static mut sv_ring_acid: ::core::ffi::c_int;
    static mut sv_ring_lightning: ::core::ffi::c_int;
    static mut sv_ring_speed: ::core::ffi::c_int;
    static mut sv_rod_light: ::core::ffi::c_int;
    static mut sv_rod_fire_bolt: ::core::ffi::c_int;
    static mut sv_rod_elec_bolt: ::core::ffi::c_int;
    static mut sv_rod_cold_bolt: ::core::ffi::c_int;
    static mut sv_rod_acid_bolt: ::core::ffi::c_int;
    static mut sv_rod_drain_life: ::core::ffi::c_int;
    static mut sv_rod_fire_ball: ::core::ffi::c_int;
    static mut sv_rod_elec_ball: ::core::ffi::c_int;
    static mut sv_rod_cold_ball: ::core::ffi::c_int;
    static mut sv_rod_acid_ball: ::core::ffi::c_int;
    static mut sv_rod_slow_monster: ::core::ffi::c_int;
    static mut sv_rod_sleep_monster: ::core::ffi::c_int;
    static mut sv_staff_power: ::core::ffi::c_int;
    static mut sv_staff_holiness: ::core::ffi::c_int;
    static mut sv_staff_sleep_monsters: ::core::ffi::c_int;
    static mut sv_staff_slow_monsters: ::core::ffi::c_int;
    static mut sv_staff_dispel_evil: ::core::ffi::c_int;
    static mut sv_wand_light: ::core::ffi::c_int;
    static mut sv_wand_stinking_cloud: ::core::ffi::c_int;
    static mut sv_wand_magic_missile: ::core::ffi::c_int;
    static mut sv_wand_annihilation: ::core::ffi::c_int;
    static mut sv_wand_wonder: ::core::ffi::c_int;
    static mut sv_wand_hold_monster: ::core::ffi::c_int;
    static mut sv_wand_slow_monster: ::core::ffi::c_int;
    static mut sv_wand_fear_monster: ::core::ffi::c_int;
    static mut sv_wand_confuse_monster: ::core::ffi::c_int;
    static mut sv_wand_fire_bolt: ::core::ffi::c_int;
    static mut sv_wand_cold_bolt: ::core::ffi::c_int;
    static mut sv_wand_acid_bolt: ::core::ffi::c_int;
    static mut sv_wand_elec_bolt: ::core::ffi::c_int;
    static mut sv_wand_fire_ball: ::core::ffi::c_int;
    static mut sv_wand_cold_ball: ::core::ffi::c_int;
    static mut sv_wand_acid_ball: ::core::ffi::c_int;
    static mut sv_wand_elec_ball: ::core::ffi::c_int;
    static mut sv_wand_dragon_cold: ::core::ffi::c_int;
    static mut sv_wand_dragon_fire: ::core::ffi::c_int;
    static mut sv_wand_drain_life: ::core::ffi::c_int;
    static mut sv_dragon_blue: ::core::ffi::c_int;
    static mut sv_dragon_black: ::core::ffi::c_int;
    static mut sv_dragon_white: ::core::ffi::c_int;
    static mut sv_dragon_red: ::core::ffi::c_int;
    static mut sv_dragon_green: ::core::ffi::c_int;
    static mut sv_dragon_multihued: ::core::ffi::c_int;
    static mut sv_dragon_shining: ::core::ffi::c_int;
    static mut sv_dragon_law: ::core::ffi::c_int;
    static mut sv_dragon_gold: ::core::ffi::c_int;
    static mut sv_dragon_chaos: ::core::ffi::c_int;
    static mut sv_dragon_balance: ::core::ffi::c_int;
    static mut sv_dragon_power: ::core::ffi::c_int;
    fn borg_los(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_offset_projectable(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_projectable_pure(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_inc_motion(
        y: *mut ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    );
    fn borg_distance(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn borg_target(target: loc, require_monster: bool) -> bool;
    static mut borg_detect_wall: *mut *mut bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brand {
    pub code: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub verb: *mut ::core::ffi::c_char,
    pub resist_flag: ::core::ffi::c_int,
    pub vuln_flag: ::core::ffi::c_int,
    pub multiplier: ::core::ffi::c_int,
    pub o_multiplier: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub next: *mut brand,
}
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const STAT_MAX: C2RustUnnamed_1 = 5;
pub const STAT_CON: C2RustUnnamed_1 = 4;
pub const STAT_DEX: C2RustUnnamed_1 = 3;
pub const STAT_WIS: C2RustUnnamed_1 = 2;
pub const STAT_INT: C2RustUnnamed_1 = 1;
pub const STAT_STR: C2RustUnnamed_1 = 0;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const BORG_ATTACK_CURSE: C2RustUnnamed_2 = 58;
pub const BORG_ATTACK_ELEC_STRIKE: C2RustUnnamed_2 = 57;
pub const BORG_ATTACK_DRAIN_LIFE: C2RustUnnamed_2 = 56;
pub const BORG_ATTACK_TAP_UNLIFE: C2RustUnnamed_2 = 55;
pub const BORG_ATTACK_AWAY_EVIL: C2RustUnnamed_2 = 54;
pub const BORG_ATTACK_TURN_UNDEAD: C2RustUnnamed_2 = 53;
pub const BORG_ATTACK_OLD_POLY: C2RustUnnamed_2 = 52;
pub const BORG_ATTACK_OLD_SLEEP: C2RustUnnamed_2 = 51;
pub const BORG_ATTACK_OLD_SLOW: C2RustUnnamed_2 = 50;
pub const BORG_ATTACK_TURN_ALL: C2RustUnnamed_2 = 49;
pub const BORG_ATTACK_OLD_CONF: C2RustUnnamed_2 = 48;
pub const BORG_ATTACK_DISP_ALL: C2RustUnnamed_2 = 47;
pub const BORG_ATTACK_AWAY_ALL_MORGOTH: C2RustUnnamed_2 = 46;
pub const BORG_ATTACK_AWAY_ALL: C2RustUnnamed_2 = 45;
pub const BORG_ATTACK_TURN_EVIL: C2RustUnnamed_2 = 44;
pub const BORG_ATTACK_AWAY_UNDEAD: C2RustUnnamed_2 = 43;
pub const BORG_ATTACK_MAKE_TRAP: C2RustUnnamed_2 = 42;
pub const BORG_ATTACK_MAKE_DOOR: C2RustUnnamed_2 = 41;
pub const BORG_ATTACK_MAKE_WALL: C2RustUnnamed_2 = 40;
pub const BORG_ATTACK_KILL_TRAP: C2RustUnnamed_2 = 39;
pub const BORG_ATTACK_KILL_DOOR: C2RustUnnamed_2 = 38;
pub const BORG_ATTACK_DARK_WEAK: C2RustUnnamed_2 = 37;
pub const BORG_ATTACK_OLD_SPEED: C2RustUnnamed_2 = 36;
pub const BORG_ATTACK_OLD_CLONE: C2RustUnnamed_2 = 35;
pub const BORG_ATTACK_OLD_HEAL: C2RustUnnamed_2 = 34;
pub const BORG_ATTACK_WATER: C2RustUnnamed_2 = 33;
pub const BORG_ATTACK_DARK: C2RustUnnamed_2 = 32;
pub const BORG_ATTACK_LIGHT: C2RustUnnamed_2 = 31;
pub const BORG_ATTACK_TIME: C2RustUnnamed_2 = 30;
pub const BORG_ATTACK_INERTIA: C2RustUnnamed_2 = 29;
pub const BORG_ATTACK_FORCE: C2RustUnnamed_2 = 28;
pub const BORG_ATTACK_NEXUS: C2RustUnnamed_2 = 27;
pub const BORG_ATTACK_DISEN: C2RustUnnamed_2 = 26;
pub const BORG_ATTACK_CONFU: C2RustUnnamed_2 = 25;
pub const BORG_ATTACK_PLASMA: C2RustUnnamed_2 = 24;
pub const BORG_ATTACK_SOUND: C2RustUnnamed_2 = 23;
pub const BORG_ATTACK_SHARD: C2RustUnnamed_2 = 22;
pub const BORG_ATTACK_GRAVITY: C2RustUnnamed_2 = 21;
pub const BORG_ATTACK_CHAOS: C2RustUnnamed_2 = 20;
pub const BORG_ATTACK_NETHER: C2RustUnnamed_2 = 19;
pub const BORG_ATTACK_KILL_WALL: C2RustUnnamed_2 = 18;
pub const BORG_ATTACK_OLD_DRAIN: C2RustUnnamed_2 = 17;
pub const BORG_ATTACK_LIGHT_WEAK: C2RustUnnamed_2 = 16;
pub const BORG_ATTACK_HOLY_WORD: C2RustUnnamed_2 = 15;
pub const BORG_ATTACK_SLEEP_EVIL: C2RustUnnamed_2 = 14;
pub const BORG_ATTACK_DISP_SPIRITS: C2RustUnnamed_2 = 13;
pub const BORG_ATTACK_DISP_EVIL: C2RustUnnamed_2 = 12;
pub const BORG_ATTACK_DISP_UNDEAD: C2RustUnnamed_2 = 11;
pub const BORG_ATTACK_HOLY_ORB: C2RustUnnamed_2 = 10;
pub const BORG_ATTACK_ICE: C2RustUnnamed_2 = 9;
pub const BORG_ATTACK_POIS: C2RustUnnamed_2 = 8;
pub const BORG_ATTACK_COLD: C2RustUnnamed_2 = 7;
pub const BORG_ATTACK_FIRE: C2RustUnnamed_2 = 6;
pub const BORG_ATTACK_ELEC: C2RustUnnamed_2 = 5;
pub const BORG_ATTACK_ACID: C2RustUnnamed_2 = 4;
pub const BORG_ATTACK_METEOR: C2RustUnnamed_2 = 3;
pub const BORG_ATTACK_MANA: C2RustUnnamed_2 = 2;
pub const BORG_ATTACK_ARROW: C2RustUnnamed_2 = 1;
pub const BORG_ATTACK_MISSILE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const BF_MAX: C2RustUnnamed_3 = 170;
pub const BF_ACT_DRAGON_POWER: C2RustUnnamed_3 = 169;
pub const BF_ACT_DRAGON_SHINING: C2RustUnnamed_3 = 168;
pub const BF_ACT_DRAGON_BALANCE: C2RustUnnamed_3 = 167;
pub const BF_ACT_DRAGON_LAW: C2RustUnnamed_3 = 166;
pub const BF_ACT_DRAGON_CHAOS: C2RustUnnamed_3 = 165;
pub const BF_ACT_DRAGON_GOLD: C2RustUnnamed_3 = 164;
pub const BF_ACT_DRAGON_MULTIHUED: C2RustUnnamed_3 = 163;
pub const BF_ACT_DRAGON_RED: C2RustUnnamed_3 = 162;
pub const BF_ACT_DRAGON_GREEN: C2RustUnnamed_3 = 161;
pub const BF_ACT_DRAGON_BLUE: C2RustUnnamed_3 = 160;
pub const BF_ACT_RING_LIGHTNING: C2RustUnnamed_3 = 159;
pub const BF_ACT_RING_ICE: C2RustUnnamed_3 = 158;
pub const BF_ACT_RING_FIRE: C2RustUnnamed_3 = 157;
pub const BF_ACT_RING_ACID: C2RustUnnamed_3 = 156;
pub const BF_ACT_STAFF_HOLY: C2RustUnnamed_3 = 155;
pub const BF_ACT_WONDER: C2RustUnnamed_3 = 154;
pub const BF_ACT_LOSCONF: C2RustUnnamed_3 = 153;
pub const BF_ACT_LOSSLEEP: C2RustUnnamed_3 = 152;
pub const BF_ACT_LOSSLOW: C2RustUnnamed_3 = 151;
pub const BF_ACT_DISPEL_ALL: C2RustUnnamed_3 = 150;
pub const BF_ACT_DISPEL_UNDEAD: C2RustUnnamed_3 = 149;
pub const BF_ACT_DISPEL_EVIL60: C2RustUnnamed_3 = 148;
pub const BF_ACT_FIRE_BOLT3: C2RustUnnamed_3 = 147;
pub const BF_ACT_FIRE_BOLT2: C2RustUnnamed_3 = 146;
pub const BF_ACT_FIRE_BALL200: C2RustUnnamed_3 = 145;
pub const BF_ACT_FIRE_BALL2: C2RustUnnamed_3 = 144;
pub const BF_ACT_COLD_BALL2: C2RustUnnamed_3 = 143;
pub const BF_ACT_COLD_BALL160: C2RustUnnamed_3 = 142;
pub const BF_ACT_ACID_BALL: C2RustUnnamed_3 = 141;
pub const BF_ACT_ACID_BOLT3: C2RustUnnamed_3 = 140;
pub const BF_ACT_ACID_BOLT2: C2RustUnnamed_3 = 139;
pub const BF_ACT_ELEC_BALL2: C2RustUnnamed_3 = 138;
pub const BF_ACT_ELEC_BALL: C2RustUnnamed_3 = 137;
pub const BF_ACT_DRAIN_LIFE4: C2RustUnnamed_3 = 136;
pub const BF_ACT_DRAIN_LIFE3: C2RustUnnamed_3 = 135;
pub const BF_ACT_LIGHT_BEAM: C2RustUnnamed_3 = 134;
pub const BF_ACT_FEAR_MONSTER: C2RustUnnamed_3 = 133;
pub const BF_ACT_SLEEP_ALL: C2RustUnnamed_3 = 132;
pub const BF_ACT_MON_CONFUSE: C2RustUnnamed_3 = 131;
pub const BF_ACT_MON_SLOW: C2RustUnnamed_3 = 130;
pub const BF_ACT_STARLIGHT: C2RustUnnamed_3 = 129;
pub const BF_ACT_STARLIGHT2: C2RustUnnamed_3 = 128;
pub const BF_ACT_STAR_BALL: C2RustUnnamed_3 = 127;
pub const BF_ACT_MANA_BOLT: C2RustUnnamed_3 = 126;
pub const BF_ACT_DISPEL_EVIL: C2RustUnnamed_3 = 125;
pub const BF_ACT_ACID_BOLT: C2RustUnnamed_3 = 124;
pub const BF_ACT_ELEC_BOLT: C2RustUnnamed_3 = 123;
pub const BF_ACT_SLEEPII: C2RustUnnamed_3 = 122;
pub const BF_ACT_MISSILE: C2RustUnnamed_3 = 121;
pub const BF_ACT_ARROW: C2RustUnnamed_3 = 120;
pub const BF_ACT_CONFUSE2: C2RustUnnamed_3 = 119;
pub const BF_ACT_STINKING_CLOUD: C2RustUnnamed_3 = 118;
pub const BF_ACT_DRAIN_LIFE2: C2RustUnnamed_3 = 117;
pub const BF_ACT_DRAIN_LIFE1: C2RustUnnamed_3 = 116;
pub const BF_ACT_COLD_BOLT2: C2RustUnnamed_3 = 115;
pub const BF_ACT_COLD_BALL100: C2RustUnnamed_3 = 114;
pub const BF_ACT_COLD_BALL50: C2RustUnnamed_3 = 113;
pub const BF_ACT_COLD_BOLT: C2RustUnnamed_3 = 112;
pub const BF_ACT_FIRE_BALL: C2RustUnnamed_3 = 111;
pub const BF_ACT_FIRE_BALL72: C2RustUnnamed_3 = 110;
pub const BF_ACT_FIRE_BOLT: C2RustUnnamed_3 = 109;
pub const BF_DRAGON_POWER: C2RustUnnamed_3 = 108;
pub const BF_DRAGON_SHINING: C2RustUnnamed_3 = 107;
pub const BF_DRAGON_BALANCE: C2RustUnnamed_3 = 106;
pub const BF_DRAGON_LAW: C2RustUnnamed_3 = 105;
pub const BF_DRAGON_CHAOS: C2RustUnnamed_3 = 104;
pub const BF_DRAGON_GOLD: C2RustUnnamed_3 = 103;
pub const BF_DRAGON_MULTIHUED: C2RustUnnamed_3 = 102;
pub const BF_DRAGON_RED: C2RustUnnamed_3 = 101;
pub const BF_DRAGON_GREEN: C2RustUnnamed_3 = 100;
pub const BF_DRAGON_BLACK: C2RustUnnamed_3 = 99;
pub const BF_DRAGON_WHITE: C2RustUnnamed_3 = 98;
pub const BF_DRAGON_BLUE: C2RustUnnamed_3 = 97;
pub const BF_RING_LIGHTNING: C2RustUnnamed_3 = 96;
pub const BF_RING_ICE: C2RustUnnamed_3 = 95;
pub const BF_RING_FIRE: C2RustUnnamed_3 = 94;
pub const BF_RING_ACID: C2RustUnnamed_3 = 93;
pub const BF_WAND_DRAGON_FIRE: C2RustUnnamed_3 = 92;
pub const BF_WAND_DRAGON_COLD: C2RustUnnamed_3 = 91;
pub const BF_WAND_WONDER: C2RustUnnamed_3 = 90;
pub const BF_WAND_FIRE_BALL: C2RustUnnamed_3 = 89;
pub const BF_WAND_ACID_BALL: C2RustUnnamed_3 = 88;
pub const BF_WAND_COLD_BALL: C2RustUnnamed_3 = 87;
pub const BF_WAND_ELEC_BALL: C2RustUnnamed_3 = 86;
pub const BF_WAND_STINKING_CLOUD: C2RustUnnamed_3 = 85;
pub const BF_WAND_LIGHT_BEAM: C2RustUnnamed_3 = 84;
pub const BF_WAND_DRAIN_LIFE: C2RustUnnamed_3 = 83;
pub const BF_WAND_ANNIHILATION: C2RustUnnamed_3 = 82;
pub const BF_WAND_FEAR_MONSTER: C2RustUnnamed_3 = 81;
pub const BF_WAND_CONFUSE_MONSTER: C2RustUnnamed_3 = 80;
pub const BF_WAND_HOLD_MONSTER: C2RustUnnamed_3 = 79;
pub const BF_WAND_SLOW_MONSTER: C2RustUnnamed_3 = 78;
pub const BF_WAND_FIRE_BOLT: C2RustUnnamed_3 = 77;
pub const BF_WAND_ACID_BOLT: C2RustUnnamed_3 = 76;
pub const BF_WAND_COLD_BOLT: C2RustUnnamed_3 = 75;
pub const BF_WAND_ELEC_BOLT: C2RustUnnamed_3 = 74;
pub const BF_WAND_MAGIC_MISSILE: C2RustUnnamed_3 = 73;
pub const BF_WAND_UNKNOWN: C2RustUnnamed_3 = 72;
pub const BF_STAFF_HOLINESS: C2RustUnnamed_3 = 71;
pub const BF_STAFF_POWER: C2RustUnnamed_3 = 70;
pub const BF_STAFF_DISPEL_EVIL: C2RustUnnamed_3 = 69;
pub const BF_STAFF_SLOW_MONSTERS: C2RustUnnamed_3 = 68;
pub const BF_STAFF_SLEEP_MONSTERS: C2RustUnnamed_3 = 67;
pub const BF_ROD_UNKNOWN: C2RustUnnamed_3 = 66;
pub const BF_ROD_SLEEP_MONSTER: C2RustUnnamed_3 = 65;
pub const BF_ROD_SLOW_MONSTER: C2RustUnnamed_3 = 64;
pub const BF_ROD_FIRE_BALL: C2RustUnnamed_3 = 63;
pub const BF_ROD_ACID_BALL: C2RustUnnamed_3 = 62;
pub const BF_ROD_COLD_BALL: C2RustUnnamed_3 = 61;
pub const BF_ROD_ELEC_BALL: C2RustUnnamed_3 = 60;
pub const BF_ROD_DRAIN_LIFE: C2RustUnnamed_3 = 59;
pub const BF_ROD_LIGHT_BEAM: C2RustUnnamed_3 = 58;
pub const BF_ROD_FIRE_BOLT: C2RustUnnamed_3 = 57;
pub const BF_ROD_ACID_BOLT: C2RustUnnamed_3 = 56;
pub const BF_ROD_COLD_BOLT: C2RustUnnamed_3 = 55;
pub const BF_ROD_ELEC_BOLT: C2RustUnnamed_3 = 54;
pub const BF_SPELL_HOWL_OF_THE_DAMNED: C2RustUnnamed_3 = 53;
pub const BF_SPELL_MAIM_FOE: C2RustUnnamed_3 = 52;
pub const BF_SPELL_LEAP_INTO_BATTLE: C2RustUnnamed_3 = 51;
pub const BF_SPELL_WHIRLWIND_ATTACK: C2RustUnnamed_3 = 50;
pub const BF_SPELL_CURSE: C2RustUnnamed_3 = 49;
pub const BF_SPELL_STORM_OF_DARKNESS: C2RustUnnamed_3 = 48;
pub const BF_SPELL_UNLEASH_CHAOS: C2RustUnnamed_3 = 47;
pub const BF_SPELL_DARK_SPEAR: C2RustUnnamed_3 = 46;
pub const BF_PRAYER_DISPEL_LIFE: C2RustUnnamed_3 = 45;
pub const BF_SPELL_VAMPIRE_STRIKE: C2RustUnnamed_3 = 44;
pub const BF_SPELL_FRIGHTEN: C2RustUnnamed_3 = 43;
pub const BF_SPELL_DISENCHANT: C2RustUnnamed_3 = 42;
pub const BF_SPELL_SLEEP_EVIL: C2RustUnnamed_3 = 41;
pub const BF_SPELL_CRUSH: C2RustUnnamed_3 = 40;
pub const BF_SPELL_TAP_UNLIFE: C2RustUnnamed_3 = 39;
pub const BF_SPELL_NETHER_BOLT: C2RustUnnamed_3 = 38;
pub const BF_SPELL_LIGHT_OF_VARDA: C2RustUnnamed_3 = 37;
pub const BF_SPELL_SPEAR_OF_OROME: C2RustUnnamed_3 = 36;
pub const BF_SPELL_RIVER_OF_LIGHTNING: C2RustUnnamed_3 = 35;
pub const BF_SPELL_VOLCANIC_ERUPTION: C2RustUnnamed_3 = 34;
pub const BF_SPELL_EARTH_RISING: C2RustUnnamed_3 = 33;
pub const BF_SPELL_LIGHTNING_STRIKE: C2RustUnnamed_3 = 32;
pub const BF_SPELL_THRUST_AWAY: C2RustUnnamed_3 = 31;
pub const BF_SPELL_MANA_BOLT: C2RustUnnamed_3 = 30;
pub const BF_SPELL_ACID_SPRAY: C2RustUnnamed_3 = 29;
pub const BF_SPELL_ELECTRIC_ARC: C2RustUnnamed_3 = 28;
pub const BF_SPELL_ANNIHILATE: C2RustUnnamed_3 = 27;
pub const BF_PRAYER_HOLY_WORD: C2RustUnnamed_3 = 26;
pub const BF_PRAYER_DISP_SPIRITS: C2RustUnnamed_3 = 25;
pub const BF_PRAYER_DISP_EVIL: C2RustUnnamed_3 = 24;
pub const BF_PRAYER_DISP_UNDEAD: C2RustUnnamed_3 = 23;
pub const BF_PRAYER_HOLY_ORB_BALL: C2RustUnnamed_3 = 22;
pub const BF_SPELL_TRANCE: C2RustUnnamed_3 = 21;
pub const BF_SPELL_BLIND_CREATURE: C2RustUnnamed_3 = 20;
pub const BF_SPELL_MANA_STORM: C2RustUnnamed_3 = 19;
pub const BF_SPELL_RIFT: C2RustUnnamed_3 = 18;
pub const BF_SPELL_METEOR_SWARM: C2RustUnnamed_3 = 17;
pub const BF_SPELL_COLD_STORM: C2RustUnnamed_3 = 16;
pub const BF_SPELL_CONFUSE_MONSTER: C2RustUnnamed_3 = 15;
pub const BF_SPELL_EXPLOSION: C2RustUnnamed_3 = 14;
pub const BF_SPELL_SHOCK_WAVE: C2RustUnnamed_3 = 13;
pub const BF_SPELL_FIRE_BALL: C2RustUnnamed_3 = 12;
pub const BF_SPELL_SLEEP_III: C2RustUnnamed_3 = 11;
pub const BF_SPELL_SLOW_MONSTER: C2RustUnnamed_3 = 10;
pub const BF_SPELL_STONE_TO_MUD: C2RustUnnamed_3 = 9;
pub const BF_SPELL_COLD_BOLT: C2RustUnnamed_3 = 8;
pub const BF_SPELL_LIGHT_BEAM: C2RustUnnamed_3 = 7;
pub const BF_SPELL_STINK_CLOUD: C2RustUnnamed_3 = 6;
pub const BF_SPELL_MAGIC_MISSILE_RESERVE: C2RustUnnamed_3 = 5;
pub const BF_SPELL_MAGIC_MISSILE: C2RustUnnamed_3 = 4;
pub const BF_LAUNCH: C2RustUnnamed_3 = 3;
pub const BF_OBJECT: C2RustUnnamed_3 = 2;
pub const BF_THRUST: C2RustUnnamed_3 = 1;
pub const BF_REST: C2RustUnnamed_3 = 0;
pub const MONBLOW_NONE: BORG_MONBLOW = 0;
pub type BORG_MONBLOW = ::core::ffi::c_uint;
pub const MONBLOW_UNDEFINED: BORG_MONBLOW = 30;
pub const MONBLOW_BLACK_BREATH: BORG_MONBLOW = 29;
pub const MONBLOW_HALLU: BORG_MONBLOW = 28;
pub const MONBLOW_EXP_80: BORG_MONBLOW = 27;
pub const MONBLOW_EXP_40: BORG_MONBLOW = 26;
pub const MONBLOW_EXP_20: BORG_MONBLOW = 25;
pub const MONBLOW_EXP_10: BORG_MONBLOW = 24;
pub const MONBLOW_SHATTER: BORG_MONBLOW = 23;
pub const MONBLOW_LOSE_ALL: BORG_MONBLOW = 22;
pub const MONBLOW_LOSE_CON: BORG_MONBLOW = 21;
pub const MONBLOW_LOSE_DEX: BORG_MONBLOW = 20;
pub const MONBLOW_LOSE_WIS: BORG_MONBLOW = 19;
pub const MONBLOW_LOSE_INT: BORG_MONBLOW = 18;
pub const MONBLOW_LOSE_STR: BORG_MONBLOW = 17;
pub const MONBLOW_PARALYZE: BORG_MONBLOW = 16;
pub const MONBLOW_TERRIFY: BORG_MONBLOW = 15;
pub const MONBLOW_CONFUSE: BORG_MONBLOW = 14;
pub const MONBLOW_BLIND: BORG_MONBLOW = 13;
pub const MONBLOW_COLD: BORG_MONBLOW = 12;
pub const MONBLOW_FIRE: BORG_MONBLOW = 11;
pub const MONBLOW_ELEC: BORG_MONBLOW = 10;
pub const MONBLOW_ACID: BORG_MONBLOW = 9;
pub const MONBLOW_EAT_LIGHT: BORG_MONBLOW = 8;
pub const MONBLOW_EAT_FOOD: BORG_MONBLOW = 7;
pub const MONBLOW_EAT_ITEM: BORG_MONBLOW = 6;
pub const MONBLOW_EAT_GOLD: BORG_MONBLOW = 5;
pub const MONBLOW_DRAIN_CHARGES: BORG_MONBLOW = 4;
pub const MONBLOW_DISENCHANT: BORG_MONBLOW = 3;
pub const MONBLOW_POISON: BORG_MONBLOW = 2;
pub const MONBLOW_HURT: BORG_MONBLOW = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub name: *const ::core::ffi::c_char,
    pub val: BORG_MONBLOW,
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
    pub goal: C2RustUnnamed_5,
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
pub type borg_time = int32_t;
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
pub const BI_CLEVEL: C2RustUnnamed_16 = 35;
pub const BI_CDEPTH: C2RustUnnamed_16 = 105;
pub const RF_QUESTOR: C2RustUnnamed_14 = 2;
pub const RF_MAX: C2RustUnnamed_14 = 85;
pub const RSF_S_UNIQUE: C2RustUnnamed_15 = 91;
pub const RSF_MAX: C2RustUnnamed_15 = 92;
pub const RSF_S_WRAITH: C2RustUnnamed_15 = 90;
pub const RSF_S_HI_UNDEAD: C2RustUnnamed_15 = 88;
pub const RSF_S_HI_DRAGON: C2RustUnnamed_15 = 89;
pub const RSF_S_DRAGON: C2RustUnnamed_15 = 87;
pub const RSF_S_UNDEAD: C2RustUnnamed_15 = 86;
pub const RSF_S_DEMON: C2RustUnnamed_15 = 85;
pub const RSF_S_AINU: C2RustUnnamed_15 = 84;
pub const RSF_S_HYDRA: C2RustUnnamed_15 = 83;
pub const RSF_S_HOUND: C2RustUnnamed_15 = 82;
pub const RSF_S_SPIDER: C2RustUnnamed_15 = 81;
pub const RSF_S_ANIMAL: C2RustUnnamed_15 = 80;
pub const RSF_S_MONSTERS: C2RustUnnamed_15 = 79;
pub const RSF_S_MONSTER: C2RustUnnamed_15 = 78;
pub const RSF_S_HI_DEMON: C2RustUnnamed_15 = 77;
pub const RSF_S_KIN: C2RustUnnamed_15 = 76;
pub const RF_MULTIPLY: C2RustUnnamed_14 = 17;
pub const RF_UNIQUE: C2RustUnnamed_14 = 1;
pub const RF_NEVER_MOVE: C2RustUnnamed_14 = 21;
pub const RF_IM_ELEC: C2RustUnnamed_14 = 70;
pub const BI_CURSP: C2RustUnnamed_16 = 30;
pub const RF_UNDEAD: C2RustUnnamed_14 = 62;
pub const RF_EVIL: C2RustUnnamed_14 = 61;
pub const RF_NO_SLEEP: C2RustUnnamed_14 = 82;
pub const RF_NO_FEAR: C2RustUnnamed_14 = 79;
pub const RF_NO_CONF: C2RustUnnamed_14 = 81;
pub const FEAT_PERM: C2RustUnnamed_13 = 22;
pub const FEAT_NONE: C2RustUnnamed_13 = 0;
pub const BI_CLASS: C2RustUnnamed_16 = 25;
pub const RSF_BA_WATE: C2RustUnnamed_15 = 35;
pub const RSF_BR_DARK: C2RustUnnamed_15 = 14;
pub const RSF_BR_LIGHT: C2RustUnnamed_15 = 13;
pub const RSF_BR_TIME: C2RustUnnamed_15 = 19;
pub const RSF_BR_INER: C2RustUnnamed_15 = 20;
pub const RSF_BR_WALL: C2RustUnnamed_15 = 24;
pub const RSF_BR_NEXU: C2RustUnnamed_15 = 18;
pub const RSF_BR_DISE: C2RustUnnamed_15 = 17;
pub const RSF_BR_PLAS: C2RustUnnamed_15 = 23;
pub const RSF_BR_SOUN: C2RustUnnamed_15 = 15;
pub const RSF_BR_SHAR: C2RustUnnamed_15 = 22;
pub const RSF_BR_GRAV: C2RustUnnamed_15 = 21;
pub const RSF_BR_CHAO: C2RustUnnamed_15 = 16;
pub const RSF_BR_NETH: C2RustUnnamed_15 = 12;
pub const RF_HURT_ROCK: C2RustUnnamed_14 = 66;
pub const RF_DEMON: C2RustUnnamed_14 = 59;
pub const RF_HURT_LIGHT: C2RustUnnamed_14 = 65;
pub const RF_NONLIVING: C2RustUnnamed_14 = 63;
pub const RF_SPIRIT: C2RustUnnamed_14 = 27;
pub const RF_IM_COLD: C2RustUnnamed_14 = 72;
pub const RF_IM_POIS: C2RustUnnamed_14 = 73;
pub const RF_HURT_COLD: C2RustUnnamed_14 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_14 = 67;
pub const RF_IM_FIRE: C2RustUnnamed_14 = 71;
pub const RF_IM_ACID: C2RustUnnamed_14 = 69;
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
pub const BI_THB: C2RustUnnamed_16 = 61;
pub const BI_TOHIT: C2RustUnnamed_16 = 134;
pub const FEAT_MAGMA: C2RustUnnamed_13 = 17;
pub const RF_PASS_WALL: C2RustUnnamed_14 = 46;
pub const FEAT_BROKEN: C2RustUnnamed_13 = 4;
pub const FEAT_OPEN: C2RustUnnamed_13 = 3;
pub const FEAT_FLOOR: C2RustUnnamed_13 = 1;
pub const BI_INFRA: C2RustUnnamed_16 = 52;
pub const BI_ESP: C2RustUnnamed_16 = 37;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_13 = 24;
pub type keycode_t = uint32_t;
pub const BI_AMMO_POWER: C2RustUnnamed_16 = 154;
pub const BI_SHOTS: C2RustUnnamed_16 = 148;
pub const BI_AMMO_TVAL: C2RustUnnamed_16 = 152;
pub const BI_ISIMAGE: C2RustUnnamed_16 = 120;
pub const BI_ISCONFUSED: C2RustUnnamed_16 = 114;
pub const BI_ISBLIND: C2RustUnnamed_16 = 112;
pub const BI_MAXSP: C2RustUnnamed_16 = 31;
pub const BI_MAXCLEVEL: C2RustUnnamed_16 = 36;
pub const BI_ISWEAK: C2RustUnnamed_16 = 108;
pub const BI_FOOD: C2RustUnnamed_16 = 39;
pub const BI_MAXHP: C2RustUnnamed_16 = 28;
pub const BI_CURHP: C2RustUnnamed_16 = 27;
pub const BI_THN: C2RustUnnamed_16 = 60;
pub const BI_BLOWS: C2RustUnnamed_16 = 146;
pub const BI_TODAM: C2RustUnnamed_16 = 135;
pub const RF_DRAGON: C2RustUnnamed_14 = 58;
pub const BI_WK_DRAGON: C2RustUnnamed_16 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_16 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_16 = 201;
pub const BI_WB_ELEC: C2RustUnnamed_16 = 206;
pub const BI_WB_POIS: C2RustUnnamed_16 = 209;
pub const BI_WB_COLD: C2RustUnnamed_16 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_16 = 207;
pub const BI_WB_ACID: C2RustUnnamed_16 = 205;
pub const BI_WS_DRAGON: C2RustUnnamed_16 = 200;
pub const RF_GIANT: C2RustUnnamed_14 = 57;
pub const BI_WS_GIANT: C2RustUnnamed_16 = 199;
pub const RF_TROLL: C2RustUnnamed_14 = 56;
pub const BI_WS_TROLL: C2RustUnnamed_16 = 198;
pub const RF_ORC: C2RustUnnamed_14 = 55;
pub const BI_WS_ORC: C2RustUnnamed_16 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_16 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_16 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_16 = 194;
pub const RF_ANIMAL: C2RustUnnamed_14 = 60;
pub const BI_WS_ANIMAL: C2RustUnnamed_16 = 193;
pub const BI_CRSFEAR: C2RustUnnamed_16 = 186;
pub const BI_ISAFRAID: C2RustUnnamed_16 = 113;
pub const BI_STR_INDEX: C2RustUnnamed_16 = 15;
pub const BI_AFUEL: C2RustUnnamed_16 = 213;
pub const BI_SPEED: C2RustUnnamed_16 = 44;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_13 = 25;
pub const FEAT_LAVA: C2RustUnnamed_13 = 23;
pub const FEAT_GRANITE: C2RustUnnamed_13 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_13 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_13 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_13 = 18;
pub const FEAT_RUBBLE: C2RustUnnamed_13 = 16;
pub const FEAT_SECRET: C2RustUnnamed_13 = 15;
pub const FEAT_HOME: C2RustUnnamed_13 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_13 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_13 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_13 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_13 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_13 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_13 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_13 = 7;
pub const FEAT_MORE: C2RustUnnamed_13 = 6;
pub const FEAT_LESS: C2RustUnnamed_13 = 5;
pub const FEAT_CLOSED: C2RustUnnamed_13 = 2;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_14 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_14 = 83;
pub const RF_NO_STUN: C2RustUnnamed_14 = 80;
pub const RF_IM_DISEN: C2RustUnnamed_14 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_14 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_14 = 76;
pub const RF_IM_WATER: C2RustUnnamed_14 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_14 = 74;
pub const RF_METAL: C2RustUnnamed_14 = 64;
pub const RF_PASS_WEB: C2RustUnnamed_14 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_14 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_14 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_14 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_14 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_14 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_14 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_14 = 47;
pub const RF_BASH_DOOR: C2RustUnnamed_14 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_14 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_14 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_14 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_14 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_14 = 40;
pub const RF_DROP_20: C2RustUnnamed_14 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_14 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_14 = 37;
pub const RF_DROP_4: C2RustUnnamed_14 = 36;
pub const RF_DROP_3: C2RustUnnamed_14 = 35;
pub const RF_DROP_2: C2RustUnnamed_14 = 34;
pub const RF_DROP_1: C2RustUnnamed_14 = 33;
pub const RF_DROP_60: C2RustUnnamed_14 = 32;
pub const RF_DROP_40: C2RustUnnamed_14 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_14 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_14 = 29;
pub const RF_POWERFUL: C2RustUnnamed_14 = 28;
pub const RF_SMART: C2RustUnnamed_14 = 26;
pub const RF_STUPID: C2RustUnnamed_14 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_14 = 24;
pub const RF_RAND_50: C2RustUnnamed_14 = 23;
pub const RF_RAND_25: C2RustUnnamed_14 = 22;
pub const RF_NEVER_BLOW: C2RustUnnamed_14 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_14 = 19;
pub const RF_REGENERATE: C2RustUnnamed_14 = 18;
pub const RF_UNAWARE: C2RustUnnamed_14 = 16;
pub const RF_SEASONAL: C2RustUnnamed_14 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_14 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_14 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_14 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_14 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_14 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_14 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_14 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_14 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_14 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_14 = 5;
pub const RF_FEMALE: C2RustUnnamed_14 = 4;
pub const RF_MALE: C2RustUnnamed_14 = 3;
pub const RF_NONE: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const RSF_SHAPECHANGE: C2RustUnnamed_15 = 75;
pub const RSF_FORGET: C2RustUnnamed_15 = 74;
pub const RSF_TRAPS: C2RustUnnamed_15 = 73;
pub const RSF_DARKNESS: C2RustUnnamed_15 = 72;
pub const RSF_TELE_LEVEL: C2RustUnnamed_15 = 71;
pub const RSF_TELE_AWAY: C2RustUnnamed_15 = 70;
pub const RSF_TELE_SELF_TO: C2RustUnnamed_15 = 69;
pub const RSF_TELE_TO: C2RustUnnamed_15 = 68;
pub const RSF_TPORT: C2RustUnnamed_15 = 67;
pub const RSF_BLINK: C2RustUnnamed_15 = 66;
pub const RSF_HEAL_KIN: C2RustUnnamed_15 = 65;
pub const RSF_HEAL: C2RustUnnamed_15 = 64;
pub const RSF_HASTE: C2RustUnnamed_15 = 63;
pub const RSF_HOLD: C2RustUnnamed_15 = 62;
pub const RSF_SLOW: C2RustUnnamed_15 = 61;
pub const RSF_CONF: C2RustUnnamed_15 = 60;
pub const RSF_BLIND: C2RustUnnamed_15 = 59;
pub const RSF_SCARE: C2RustUnnamed_15 = 58;
pub const RSF_BE_NETH: C2RustUnnamed_15 = 57;
pub const RSF_BE_ELEC: C2RustUnnamed_15 = 56;
pub const RSF_MISSILE: C2RustUnnamed_15 = 55;
pub const RSF_BO_ICE: C2RustUnnamed_15 = 54;
pub const RSF_BO_PLAS: C2RustUnnamed_15 = 53;
pub const RSF_BO_MANA: C2RustUnnamed_15 = 52;
pub const RSF_BO_WATE: C2RustUnnamed_15 = 51;
pub const RSF_BO_NETH: C2RustUnnamed_15 = 50;
pub const RSF_BO_POIS: C2RustUnnamed_15 = 49;
pub const RSF_BO_COLD: C2RustUnnamed_15 = 48;
pub const RSF_BO_FIRE: C2RustUnnamed_15 = 47;
pub const RSF_BO_ELEC: C2RustUnnamed_15 = 46;
pub const RSF_BO_ACID: C2RustUnnamed_15 = 45;
pub const RSF_WOUND: C2RustUnnamed_15 = 44;
pub const RSF_BRAIN_SMASH: C2RustUnnamed_15 = 43;
pub const RSF_MIND_BLAST: C2RustUnnamed_15 = 42;
pub const RSF_DRAIN_MANA: C2RustUnnamed_15 = 41;
pub const RSF_STORM: C2RustUnnamed_15 = 40;
pub const RSF_BA_LIGHT: C2RustUnnamed_15 = 39;
pub const RSF_BA_DARK: C2RustUnnamed_15 = 38;
pub const RSF_BA_HOLY: C2RustUnnamed_15 = 37;
pub const RSF_BA_MANA: C2RustUnnamed_15 = 36;
pub const RSF_BA_NETH: C2RustUnnamed_15 = 34;
pub const RSF_BA_SHAR: C2RustUnnamed_15 = 33;
pub const RSF_BA_POIS: C2RustUnnamed_15 = 32;
pub const RSF_BA_COLD: C2RustUnnamed_15 = 31;
pub const RSF_BA_FIRE: C2RustUnnamed_15 = 30;
pub const RSF_BA_ELEC: C2RustUnnamed_15 = 29;
pub const RSF_BA_ACID: C2RustUnnamed_15 = 28;
pub const RSF_WEAVE: C2RustUnnamed_15 = 27;
pub const RSF_BOULDER: C2RustUnnamed_15 = 26;
pub const RSF_BR_MANA: C2RustUnnamed_15 = 25;
pub const RSF_BR_POIS: C2RustUnnamed_15 = 11;
pub const RSF_BR_COLD: C2RustUnnamed_15 = 10;
pub const RSF_BR_FIRE: C2RustUnnamed_15 = 9;
pub const RSF_BR_ELEC: C2RustUnnamed_15 = 8;
pub const RSF_BR_ACID: C2RustUnnamed_15 = 7;
pub const RSF_BOLT: C2RustUnnamed_15 = 6;
pub const RSF_ARROW: C2RustUnnamed_15 = 5;
pub const RSF_SHOT: C2RustUnnamed_15 = 4;
pub const RSF_SPIT: C2RustUnnamed_15 = 3;
pub const RSF_WHIP: C2RustUnnamed_15 = 2;
pub const RSF_SHRIEK: C2RustUnnamed_15 = 1;
pub const RSF_NONE: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_16 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_16 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_16 = 263;
pub const BI_EMPTY: C2RustUnnamed_16 = 262;
pub const BI_CARRY: C2RustUnnamed_16 = 261;
pub const BI_WEIGHT: C2RustUnnamed_16 = 260;
pub const BI_DINV: C2RustUnnamed_16 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_16 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_16 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_16 = 256;
pub const BI_ADIGGER: C2RustUnnamed_16 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_16 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_16 = 253;
pub const BI_AROD2: C2RustUnnamed_16 = 252;
pub const BI_AROD1: C2RustUnnamed_16 = 251;
pub const BI_ASHROOM: C2RustUnnamed_16 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_16 = 249;
pub const BI_AHWORD: C2RustUnnamed_16 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_16 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_16 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_16 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_16 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_16 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_16 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_16 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_16 = 240;
pub const BI_ABRAND: C2RustUnnamed_16 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_16 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_16 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_16 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_16 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_16 = 234;
pub const BI_ACLW: C2RustUnnamed_16 = 233;
pub const BI_ACSW: C2RustUnnamed_16 = 232;
pub const BI_ACCW: C2RustUnnamed_16 = 231;
pub const BI_AGLYPH: C2RustUnnamed_16 = 230;
pub const BI_APFE: C2RustUnnamed_16 = 229;
pub const BI_ALITE: C2RustUnnamed_16 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_16 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_16 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_16 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_16 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_16 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_16 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_16 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_16 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_16 = 219;
pub const BI_ASPEED: C2RustUnnamed_16 = 218;
pub const BI_AID: C2RustUnnamed_16 = 217;
pub const BI_ALIFE: C2RustUnnamed_16 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_16 = 215;
pub const BI_AHEAL: C2RustUnnamed_16 = 214;
pub const BI_AESCAPE: C2RustUnnamed_16 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_16 = 211;
pub const BI_APHASE: C2RustUnnamed_16 = 210;
pub const BI_W_IMPACT: C2RustUnnamed_16 = 204;
pub const BI_CRSUNKNO: C2RustUnnamed_16 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_16 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_16 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_16 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_16 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_16 = 187;
pub const BI_CRSAIRSWING: C2RustUnnamed_16 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_16 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_16 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_16 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_16 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_16 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_16 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_16 = 178;
pub const BI_CRSSICK: C2RustUnnamed_16 = 177;
pub const BI_CRSDULL: C2RustUnnamed_16 = 176;
pub const BI_CRSVULN: C2RustUnnamed_16 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_16 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_16 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_16 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_16 = 171;
pub const BI_CRSSUND: C2RustUnnamed_16 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_16 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_16 = 168;
pub const BI_CRSPARA: C2RustUnnamed_16 = 167;
pub const BI_CRSHALU: C2RustUnnamed_16 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_16 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_16 = 164;
pub const BI_CRSTELE: C2RustUnnamed_16 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_16 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_16 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_16 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_16 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_16 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_16 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_16 = 156;
pub const BI_AMISSILES: C2RustUnnamed_16 = 155;
pub const BI_AMMO_SIDES: C2RustUnnamed_16 = 153;
pub const BI_AMMO_COUNT: C2RustUnnamed_16 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_16 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_16 = 149;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_16 = 147;
pub const BI_BART: C2RustUnnamed_16 = 145;
pub const BI_SLING: C2RustUnnamed_16 = 144;
pub const BI_BTODAM: C2RustUnnamed_16 = 143;
pub const BI_BTOHIT: C2RustUnnamed_16 = 142;
pub const BI_BID: C2RustUnnamed_16 = 141;
pub const BI_WDS: C2RustUnnamed_16 = 140;
pub const BI_WDD: C2RustUnnamed_16 = 139;
pub const BI_WID: C2RustUnnamed_16 = 138;
pub const BI_WTODAM: C2RustUnnamed_16 = 137;
pub const BI_WTOHIT: C2RustUnnamed_16 = 136;
pub const BI_ARMOR: C2RustUnnamed_16 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_16 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_16 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_16 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_16 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_16 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_16 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_16 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_16 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_16 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_16 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_16 = 122;
pub const BI_ISFORGET: C2RustUnnamed_16 = 121;
pub const BI_ISPARALYZED: C2RustUnnamed_16 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_16 = 118;
pub const BI_ISSTUN: C2RustUnnamed_16 = 117;
pub const BI_ISCUT: C2RustUnnamed_16 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_16 = 115;
pub const BI_ISGORGED: C2RustUnnamed_16 = 111;
pub const BI_ISFULL: C2RustUnnamed_16 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_16 = 109;
pub const BI_KING: C2RustUnnamed_16 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_16 = 106;
pub const BI_SFRACT: C2RustUnnamed_16 = 104;
pub const BI_SHLIFE: C2RustUnnamed_16 = 103;
pub const BI_SRDIS: C2RustUnnamed_16 = 102;
pub const BI_SRKAOS: C2RustUnnamed_16 = 101;
pub const BI_SRNTHR: C2RustUnnamed_16 = 100;
pub const BI_SRNXUS: C2RustUnnamed_16 = 99;
pub const BI_SRSHRD: C2RustUnnamed_16 = 98;
pub const BI_SRSND: C2RustUnnamed_16 = 97;
pub const BI_SRCONF: C2RustUnnamed_16 = 96;
pub const BI_SRBLIND: C2RustUnnamed_16 = 95;
pub const BI_SRDARK: C2RustUnnamed_16 = 94;
pub const BI_SRLITE: C2RustUnnamed_16 = 93;
pub const BI_SRFEAR: C2RustUnnamed_16 = 92;
pub const BI_SRPOIS: C2RustUnnamed_16 = 91;
pub const BI_SRACID: C2RustUnnamed_16 = 90;
pub const BI_SRELEC: C2RustUnnamed_16 = 89;
pub const BI_SRCOLD: C2RustUnnamed_16 = 88;
pub const BI_SRFIRE: C2RustUnnamed_16 = 87;
pub const BI_FRACT: C2RustUnnamed_16 = 86;
pub const BI_HLIFE: C2RustUnnamed_16 = 85;
pub const BI_RDIS: C2RustUnnamed_16 = 84;
pub const BI_RKAOS: C2RustUnnamed_16 = 83;
pub const BI_RNTHR: C2RustUnnamed_16 = 82;
pub const BI_RNXUS: C2RustUnnamed_16 = 81;
pub const BI_RSHRD: C2RustUnnamed_16 = 80;
pub const BI_RSND: C2RustUnnamed_16 = 79;
pub const BI_RCONF: C2RustUnnamed_16 = 78;
pub const BI_RBLIND: C2RustUnnamed_16 = 77;
pub const BI_RDARK: C2RustUnnamed_16 = 76;
pub const BI_RLITE: C2RustUnnamed_16 = 75;
pub const BI_RFEAR: C2RustUnnamed_16 = 74;
pub const BI_RPOIS: C2RustUnnamed_16 = 73;
pub const BI_RACID: C2RustUnnamed_16 = 72;
pub const BI_RELEC: C2RustUnnamed_16 = 71;
pub const BI_RCOLD: C2RustUnnamed_16 = 70;
pub const BI_RFIRE: C2RustUnnamed_16 = 69;
pub const BI_IPOIS: C2RustUnnamed_16 = 68;
pub const BI_IELEC: C2RustUnnamed_16 = 67;
pub const BI_ICOLD: C2RustUnnamed_16 = 66;
pub const BI_IACID: C2RustUnnamed_16 = 65;
pub const BI_IFIRE: C2RustUnnamed_16 = 64;
pub const BI_DIG: C2RustUnnamed_16 = 63;
pub const BI_THT: C2RustUnnamed_16 = 62;
pub const BI_SRCH: C2RustUnnamed_16 = 59;
pub const BI_STL: C2RustUnnamed_16 = 58;
pub const BI_SAV: C2RustUnnamed_16 = 57;
pub const BI_DEV: C2RustUnnamed_16 = 56;
pub const BI_DISM: C2RustUnnamed_16 = 55;
pub const BI_DISP: C2RustUnnamed_16 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_16 = 53;
pub const BI_SINV: C2RustUnnamed_16 = 51;
pub const BI_REG: C2RustUnnamed_16 = 50;
pub const BI_FEATH: C2RustUnnamed_16 = 49;
pub const BI_SDIG: C2RustUnnamed_16 = 48;
pub const BI_DAM_RED: C2RustUnnamed_16 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_16 = 46;
pub const BI_GOLD: C2RustUnnamed_16 = 45;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_16 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_16 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_16 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_16 = 40;
pub const BI_RECALL: C2RustUnnamed_16 = 38;
pub const BI_FAIL2: C2RustUnnamed_16 = 34;
pub const BI_FAIL1: C2RustUnnamed_16 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_16 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_16 = 29;
pub const BI_LIGHT: C2RustUnnamed_16 = 26;
pub const BI_SCON: C2RustUnnamed_16 = 24;
pub const BI_SDEX: C2RustUnnamed_16 = 23;
pub const BI_SWIS: C2RustUnnamed_16 = 22;
pub const BI_SINT: C2RustUnnamed_16 = 21;
pub const BI_SSTR: C2RustUnnamed_16 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_16 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_16 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_16 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_16 = 16;
pub const BI_CCON: C2RustUnnamed_16 = 14;
pub const BI_CDEX: C2RustUnnamed_16 = 13;
pub const BI_CWIS: C2RustUnnamed_16 = 12;
pub const BI_CINT: C2RustUnnamed_16 = 11;
pub const BI_CSTR: C2RustUnnamed_16 = 10;
pub const BI_ACON: C2RustUnnamed_16 = 9;
pub const BI_ADEX: C2RustUnnamed_16 = 8;
pub const BI_AWIS: C2RustUnnamed_16 = 7;
pub const BI_AINT: C2RustUnnamed_16 = 6;
pub const BI_ASTR: C2RustUnnamed_16 = 5;
pub const BI_CON: C2RustUnnamed_16 = 4;
pub const BI_DEX: C2RustUnnamed_16 = 3;
pub const BI_WIS: C2RustUnnamed_16 = 2;
pub const BI_INT: C2RustUnnamed_16 = 1;
pub const BI_STR: C2RustUnnamed_16 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BTH_PLUS_ADJ: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut successful_target: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut target_closest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_tp_other_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_tp_other_x: [uint8_t; 255] = [0; 255];
#[no_mangle]
pub static mut borg_tp_other_y: [uint8_t; 255] = [0; 255];
#[no_mangle]
pub static mut borg_tp_other_index: [::core::ffi::c_int; 255] = [0; 255];
#[no_mangle]
pub unsafe extern "C" fn borg_mon_blow_effect(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut monblow: [C2RustUnnamed_4; 31] = [
        C2RustUnnamed_4 {
            name: b"NONE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_NONE,
        },
        C2RustUnnamed_4 {
            name: b"HURT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_HURT,
        },
        C2RustUnnamed_4 {
            name: b"POISON\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_POISON,
        },
        C2RustUnnamed_4 {
            name: b"DISENCHANT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_DISENCHANT,
        },
        C2RustUnnamed_4 {
            name: b"DRAIN_CHARGES\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_DRAIN_CHARGES,
        },
        C2RustUnnamed_4 {
            name: b"EAT_GOLD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_GOLD,
        },
        C2RustUnnamed_4 {
            name: b"EAT_ITEM\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_ITEM,
        },
        C2RustUnnamed_4 {
            name: b"EAT_FOOD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_FOOD,
        },
        C2RustUnnamed_4 {
            name: b"EAT_LIGHT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EAT_LIGHT,
        },
        C2RustUnnamed_4 {
            name: b"ACID\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_ACID,
        },
        C2RustUnnamed_4 {
            name: b"ELEC\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_ELEC,
        },
        C2RustUnnamed_4 {
            name: b"FIRE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_FIRE,
        },
        C2RustUnnamed_4 {
            name: b"COLD\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_COLD,
        },
        C2RustUnnamed_4 {
            name: b"BLIND\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_BLIND,
        },
        C2RustUnnamed_4 {
            name: b"CONFUSE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_CONFUSE,
        },
        C2RustUnnamed_4 {
            name: b"TERRIFY\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_TERRIFY,
        },
        C2RustUnnamed_4 {
            name: b"PARALYZE\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_PARALYZE,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_STR\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_STR,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_INT\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_INT,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_WIS\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_WIS,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_DEX\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_DEX,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_CON\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_CON,
        },
        C2RustUnnamed_4 {
            name: b"LOSE_ALL\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_LOSE_ALL,
        },
        C2RustUnnamed_4 {
            name: b"SHATTER\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_SHATTER,
        },
        C2RustUnnamed_4 {
            name: b"EXP_10\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_10,
        },
        C2RustUnnamed_4 {
            name: b"EXP_20\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_20,
        },
        C2RustUnnamed_4 {
            name: b"EXP_40\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_40,
        },
        C2RustUnnamed_4 {
            name: b"EXP_80\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_EXP_80,
        },
        C2RustUnnamed_4 {
            name: b"HALLU\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_HALLU,
        },
        C2RustUnnamed_4 {
            name: b"BLACK_BREATH\0" as *const u8 as *const ::core::ffi::c_char,
            val: MONBLOW_BLACK_BREATH,
        },
        C2RustUnnamed_4 {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            val: MONBLOW_NONE,
        },
    ];
    let mut i: ::core::ffi::c_ulong = 0;
    i = 0 as ::core::ffi::c_ulong;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_4; 31]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as usize)
    {
        if strcmp(name, monblow[i as usize].name) == 0 {
            return monblow[i as usize].val as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return MONBLOW_NONE as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_thrust_damage_one(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut dam: ::core::ffi::c_int = 0;
    let mut mult: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut chance: ::core::ffi::c_int = 0;
    item = borg_items.offset((*z_info).pack_size as isize) as *mut borg_item;
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    dam = (*item).dd as ::core::ffi::c_int
        * ((*item).ds as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int;
    mult = 1 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_WS_ANIMAL as ::core::ffi::c_int as isize)
        != 0
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
            RF_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            != 0
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
                RF_EVIL as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
    {
        mult = 2 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_WS_UNDEAD as ::core::ffi::c_int as isize)
        != 0
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
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || *borg
            .trait_0
            .offset(BI_WS_DEMON as ::core::ffi::c_int as isize)
            != 0
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
                RF_DEMON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WS_ORC as ::core::ffi::c_int as isize)
            != 0
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
                RF_ORC as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WS_TROLL as ::core::ffi::c_int as isize)
            != 0
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
                RF_TROLL as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WS_GIANT as ::core::ffi::c_int as isize)
            != 0
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
                RF_GIANT as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WS_DRAGON as ::core::ffi::c_int as isize)
            != 0
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
                RF_DRAGON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WB_ACID as ::core::ffi::c_int as isize)
            != 0
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
                RF_IM_ACID as ::core::ffi::c_int,
            )
        || *borg
            .trait_0
            .offset(BI_WB_FIRE as ::core::ffi::c_int as isize)
            != 0
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
                RF_IM_FIRE as ::core::ffi::c_int,
            )
        || *borg
            .trait_0
            .offset(BI_WB_COLD as ::core::ffi::c_int as isize)
            != 0
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
                RF_IM_COLD as ::core::ffi::c_int,
            )
        || *borg
            .trait_0
            .offset(BI_WB_POIS as ::core::ffi::c_int as isize)
            != 0
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
                RF_IM_POIS as ::core::ffi::c_int,
            )
        || *borg
            .trait_0
            .offset(BI_WB_ELEC as ::core::ffi::c_int as isize)
            != 0
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
                RF_IM_ELEC as ::core::ffi::c_int,
            )
    {
        mult = 3 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_WK_UNDEAD as ::core::ffi::c_int as isize)
        != 0
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
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || *borg
            .trait_0
            .offset(BI_WK_DEMON as ::core::ffi::c_int as isize)
            != 0
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
                RF_DEMON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
        || *borg
            .trait_0
            .offset(BI_WK_DRAGON as ::core::ffi::c_int as isize)
            != 0
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
                RF_DRAGON as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
    {
        mult = 5 as ::core::ffi::c_int;
    }
    dam *= mult;
    dam += (*item).to_d as ::core::ffi::c_int;
    dam += *borg.trait_0.offset(BI_TODAM as ::core::ffi::c_int as isize);
    dam *= *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize);
    chance = *borg.trait_0.offset(BI_THN as ::core::ffi::c_int as isize)
        + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
            + (*item).to_h as ::core::ffi::c_int)
            * 3 as ::core::ffi::c_int;
    if chance
        < (*r_ptr).ac * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
    {
        dam = 0 as ::core::ffi::c_int;
    }
    if chance > 95 as ::core::ffi::c_int {
        chance = 95 as ::core::ffi::c_int;
    }
    if chance < 5 as ::core::ffi::c_int {
        chance = 5 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        > 15 as ::core::ffi::c_int
    {
        chance += 10 as ::core::ffi::c_int;
    }
    if (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER)
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) > 1 as ::core::ffi::c_int
    {
        chance -= 10 as ::core::ffi::c_int;
    }
    dam = dam * chance / 100 as ::core::ffi::c_int;
    if dam <= 0 as ::core::ffi::c_int {
        dam = 1 as ::core::ffi::c_int;
    }
    if dam > (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int
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
            RF_UNIQUE as ::core::ffi::c_int,
        )
    {
        dam = (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    }
    if (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER)
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 40 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) > 1 as ::core::ffi::c_int
    {
        dam = dam * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
    {
        dam += dam * 5 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
    {
        dam = dam * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 5 as ::core::ffi::c_int
        {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        dam = dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).spell_flags as *mut bitflag,
        (RSF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RSF_S_KIN as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTERS as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_SPIDER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HOUND as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HYDRA as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_AINU as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_WRAITH as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNIQUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        dam += dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_QUESTOR as ::core::ffi::c_int,
    ) {
        dam += dam * 5 as ::core::ffi::c_int;
    }
    return dam;
}
unsafe extern "C" fn borg_attack_aux_thrust() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut current_block_12: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(borg_distance(borg.c.y, borg.c.x, y, x) > 1 as ::core::ffi::c_int) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
            if !(d <= 0 as ::core::ffi::c_int) {
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !((*kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    if !(*kill).awake
                        && d <= (*kill).power as ::core::ffi::c_int
                        && !borg.munchkin_mode
                    {
                        p = borg_danger_one_kill(
                            y,
                            x,
                            1 as ::core::ffi::c_int,
                            (*ag).kill as ::core::ffi::c_int,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        if p > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                            current_block_12 = 17778012151635330486;
                        } else {
                            current_block_12 = 10599921512955367680;
                        }
                    } else {
                        current_block_12 = 10599921512955367680;
                    }
                    match current_block_12 {
                        17778012151635330486 => {}
                        _ => {
                            if !(*borg
                                .trait_0
                                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                == 0
                                && !(*kill).awake)
                            {
                                p = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    2 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if d <= (*kill).power as ::core::ffi::c_int
                                    && *borg
                                        .trait_0
                                        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                        > 15 as ::core::ffi::c_int
                                {
                                    p = p / 10 as ::core::ffi::c_int;
                                }
                                d += p;
                                if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
                                    b_i = i;
                                    b_d = d;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.goal.g.y as isize))
        .offset(borg.goal.g.x as isize) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    borg_note(format(
        b"# Facing %s at (%d,%d) who has %d Hit Points.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        borg.goal.g.y,
        borg.goal.g.x,
        (*kill).power as ::core::ffi::c_int,
    ));
    borg_note(format(
        b"# Attacking with weapon '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset((*z_info).pack_size as isize)).desc
            as *mut ::core::ffi::c_char,
    ));
    dir = borg_extract_dir(borg.c.y, borg.c.x, borg.goal.g.y, borg.goal.g.x);
    borg_keypress('+' as i32 as keycode_t);
    borg_keypress((dir + '0' as i32) as keycode_t);
    return b_d;
}
unsafe extern "C" fn borg_best_mult(
    mut obj: *mut borg_item,
    mut r_ptr: *mut monster_race,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut max_mult: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut current_block_6: u64;
    i = 1 as ::core::ffi::c_int;
    while i < (*z_info).brand_max as ::core::ffi::c_int {
        let mut brand: *mut brand = brands.offset(i as isize) as *mut brand;
        if !obj.is_null() && (*obj).iqty as ::core::ffi::c_int != 0 {
            if !(*obj).brands[i as usize] {
                current_block_6 = 16668937799742929182;
            } else {
                current_block_6 = 10879442775620481940;
            }
        } else if !player_has_temporary_brand(player, i) {
            current_block_6 = 16668937799742929182;
        } else {
            current_block_6 = 10879442775620481940;
        }
        match current_block_6 {
            10879442775620481940 => {
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
                    (*brand).resist_flag,
                ) {
                    let mut mult: ::core::ffi::c_int = (*brand).multiplier;
                    if (*brand).vuln_flag != 0
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
                            (*brand).vuln_flag,
                        ) as ::core::ffi::c_int
                            != 0
                    {
                        mult *= 2 as ::core::ffi::c_int;
                    }
                    max_mult = if mult < max_mult { max_mult } else { mult };
                }
            }
            _ => {}
        }
        i += 1;
    }
    let mut current_block_11: u64;
    i = 1 as ::core::ffi::c_int;
    while i < (*z_info).slay_max as ::core::ffi::c_int {
        let mut slay: *mut slay = slays.offset(i as isize) as *mut slay;
        if !obj.is_null() && (*obj).iqty as ::core::ffi::c_int != 0 {
            if (*obj).slays[i as usize] == 0 {
                current_block_11 = 3512920355445576850;
            } else {
                current_block_11 = 12124785117276362961;
            }
        } else if !player_has_temporary_slay(player, i) {
            current_block_11 = 3512920355445576850;
        } else {
            current_block_11 = 12124785117276362961;
        }
        match current_block_11 {
            12124785117276362961 => {
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
                    (*slay).race_flag,
                ) {
                    max_mult = if (*slay).multiplier < max_mult {
                        max_mult
                    } else {
                        (*slay).multiplier
                    };
                }
            }
            _ => {}
        }
        i += 1;
    }
    return max_mult;
}
unsafe extern "C" fn borg_launch_damage_one(
    mut i: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut borg_use_missile: bool = false_0 != 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut vault_grids: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut gold_eater: bool = false_0 != 0;
    let mut chance: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bonus: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur_dis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut armor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    cur_dis = distance(borg.c, (*kill).pos);
    bonus = *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
        + (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .to_h as ::core::ffi::c_int
        + (*borg_items.offset(ammo_location as isize)).to_h as ::core::ffi::c_int;
    chance = *borg.trait_0.offset(BI_THB as ::core::ffi::c_int as isize) + bonus * BTH_PLUS_ADJ;
    armor = (*r_ptr).ac + cur_dis;
    k = 0 as ::core::ffi::c_int;
    while k < 4 as ::core::ffi::c_int {
        if !(*(*r_ptr).blow.offset(k as isize)).effect.is_null()
            && borg_mon_blow_effect((*(*(*r_ptr).blow.offset(k as isize)).effect).name)
                == MONBLOW_EAT_GOLD as ::core::ffi::c_int
        {
            gold_eater = true_0 != 0;
        }
        k += 1;
    }
    match typ {
        1 => {
            let mut bow: *mut borg_item = borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
            let mut ammo: *mut borg_item =
                borg_items.offset(ammo_location as isize) as *mut borg_item;
            let mut mult: ::core::ffi::c_int = borg_best_mult(bow, r_ptr);
            mult = if mult < borg_best_mult(ammo, r_ptr) {
                borg_best_mult(ammo, r_ptr)
            } else {
                mult
            };
            dam *= mult;
            if cur_dis == 1 as ::core::ffi::c_int
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
                    RF_UNIQUE as ::core::ffi::c_int,
                )
            {
                dam /= 5 as ::core::ffi::c_int;
            }
            if chance < armor * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        2 => {
            if borg.near.unique as ::core::ffi::c_int != 0
                && *borg.has.offset(kv_potion_restore_mana as isize) > 3 as ::core::ffi::c_int
            {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        4 => {
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
                RF_IM_ACID as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        5 => {
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
                RF_IM_ELEC as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        6 => {
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
                RF_IM_FIRE as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
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
                RF_HURT_FIRE as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        7 => {
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
                RF_IM_COLD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
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
                RF_HURT_COLD as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        8 => {
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
                RF_IM_POIS as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        9 => {
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
                RF_IM_COLD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        10 => {
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
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam *= 2 as ::core::ffi::c_int;
            }
        }
        11 => {
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
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        13 => {
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
                RF_SPIRIT as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        12 => {
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
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        56 => {
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
                RF_NONLIVING as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
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
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        15 => {
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
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        16 => {
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
                RF_HURT_LIGHT as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        17 => {
            if distance(borg.c, (*kill).pos) == 1 as ::core::ffi::c_int {
                dam /= 5 as ::core::ffi::c_int;
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
                RF_UNDEAD as ::core::ffi::c_int,
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
                    RF_DEMON as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                || !strchr(
                    b"Egv\0" as *const u8 as *const ::core::ffi::c_char,
                    (*r_ptr).d_char as ::core::ffi::c_int,
                )
                .is_null()
            {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        18 => {
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
                RF_HURT_ROCK as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        19 => {
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
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else if flag_has(
                &raw mut (*r_ptr).spell_flags as *mut bitflag,
                (RSF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RSF_BR_NETH as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            } else if flag_has(
                &raw mut (*r_ptr).flags as *mut bitflag,
                (RF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                RF_EVIL as ::core::ffi::c_int,
            ) {
                dam /= 2 as ::core::ffi::c_int;
            }
        }
        20 => {
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
                RSF_BR_CHAO as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
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
                RF_UNIQUE as ::core::ffi::c_int,
            ) {
                dam = -(999 as ::core::ffi::c_int);
            }
        }
        21 => {
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
                RSF_BR_GRAV as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        22 => {
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
                RSF_BR_SHAR as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        23 => {
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
                RSF_BR_SOUN as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        24 => {
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
                RSF_BR_PLAS as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        25 => {
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
                RF_NO_CONF as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        26 => {
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
                RSF_BR_DISE as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        27 => {
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
                RSF_BR_NEXU as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        28 => {
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
                RSF_BR_WALL as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        29 => {
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
                RSF_BR_INER as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        30 => {
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
                RSF_BR_TIME as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        31 => {
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
                RSF_BR_LIGHT as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        32 => {
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
                RSF_BR_DARK as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
        }
        33 => {
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
                RSF_BA_WATE as ::core::ffi::c_int,
            ) {
                dam *= 3 as ::core::ffi::c_int;
                dam /= 9 as ::core::ffi::c_int;
            }
            dam /= 2 as ::core::ffi::c_int;
        }
        34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 => {
            dam = 0 as ::core::ffi::c_int;
        }
        45 => {
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
                if (*kill).injury as ::core::ffi::c_int >= 60 as ::core::ffi::c_int {
                    dam = -(9999 as ::core::ffi::c_int);
                } else if borg.status.anti_summon {
                    dam = -(9999 as ::core::ffi::c_int);
                } else if dam
                    > borg.avoidance as ::core::ffi::c_int * 13 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int
                    && *borg
                        .trait_0
                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                        <= 98 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if borg.near.unique as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                    && borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                    == CLASS_MAGE
                    && distance(borg.c, (*kill).pos) <= 2 as ::core::ffi::c_int
                {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                } else if borg.status.vault {
                    ii = 0 as ::core::ffi::c_int;
                    while ii < 8 as ::core::ffi::c_int {
                        x = (*kill).pos.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                        y = (*kill).pos.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                            .offset(x as isize) as *mut borg_grid;
                        if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                            if (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int {
                                vault_grids += 1;
                            }
                        }
                        ii += 1;
                    }
                    if vault_grids >= 2 as ::core::ffi::c_int {
                        borg_tp_other_index[borg_tp_other_n as usize] = i;
                        borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                        borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                        borg_tp_other_n += 1;
                    }
                } else {
                    dam = -(999 as ::core::ffi::c_int);
                }
            } else {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
            }
        }
        46 => {
            dam = 0 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < 8 as ::core::ffi::c_int {
                let mut y2: ::core::ffi::c_int =
                    (*kill).pos.y + ddy_ddd[j as usize] as ::core::ffi::c_int;
                let mut x2: ::core::ffi::c_int =
                    (*kill).pos.x + ddx_ddd[j as usize] as ::core::ffi::c_int;
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize))
                    .offset(x2 as isize) as *mut borg_grid;
                if (*ag).glyph {
                    borg_tp_other_index[borg_tp_other_n as usize] = i;
                    borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                    borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                    borg_tp_other_n += 1;
                    dam = 300 as ::core::ffi::c_int;
                }
                j += 1;
            }
            if borg.near.morgoth as ::core::ffi::c_int != 0 && !borg.morgoth_position {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
                dam = 100 as ::core::ffi::c_int;
            }
            if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                <= 35 as ::core::ffi::c_int
            {
                borg_tp_other_index[borg_tp_other_n as usize] = i;
                borg_tp_other_y[borg_tp_other_n as usize] = (*kill).pos.y as uint8_t;
                borg_tp_other_x[borg_tp_other_n as usize] = (*kill).pos.x as uint8_t;
                borg_tp_other_n += 1;
                dam = 150 as ::core::ffi::c_int;
            }
        }
        47 => {
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
                dam = 0 as ::core::ffi::c_int;
            } else {
                dam = borg_danger_one_kill(
                    borg.c.y,
                    borg.c.x,
                    1 as ::core::ffi::c_int,
                    i,
                    true_0 != 0,
                    true_0 != 0,
                );
            }
        }
        48 => {
            dam = 0 as ::core::ffi::c_int;
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
                RF_NO_CONF as ::core::ffi::c_int,
            ) {
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
                    RF_MULTIPLY as ::core::ffi::c_int,
                ) {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if !(*kill).confused {
                            if (*kill).awake {
                                if !((*kill).level as ::core::ffi::c_int
                                    > (if *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        < 13 as ::core::ffi::c_int
                                    {
                                        10 as ::core::ffi::c_int
                                    } else {
                                        (*borg
                                            .trait_0
                                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                            - 10 as ::core::ffi::c_int)
                                            / 4 as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int
                                    }))
                                {
                                    dam = -(999 as ::core::ffi::c_int);
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
                                        RF_UNIQUE as ::core::ffi::c_int,
                                    ) {
                                        borg_confuse_spell = false_0 != 0;
                                        p1 = borg_danger_one_kill(
                                            borg.c.y,
                                            borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        if (*kill).afraid as ::core::ffi::c_int != 0
                                            && *borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                <= 10 as ::core::ffi::c_int
                                        {
                                            p1 = p1 + 20 as ::core::ffi::c_int;
                                        }
                                        borg_confuse_spell = true_0 != 0;
                                        p2 = borg_danger_one_kill(
                                            borg.c.y,
                                            borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        borg_confuse_spell = false_0 != 0;
                                        dam = p1 - p2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        49 => {
            dam = 0 as ::core::ffi::c_int;
            if !(((*kill).speed as ::core::ffi::c_int) < (*r_ptr).speed - 5 as ::core::ffi::c_int) {
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
                    RF_NO_FEAR as ::core::ffi::c_int,
                ) {
                    if !(*kill).confused {
                        if (*kill).awake {
                            if !((*kill).level as ::core::ffi::c_int
                                > (if *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    < 13 as ::core::ffi::c_int
                                {
                                    10 as ::core::ffi::c_int
                                } else {
                                    (*borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        - 10 as ::core::ffi::c_int)
                                        / 4 as ::core::ffi::c_int
                                        * 3 as ::core::ffi::c_int
                                        + 10 as ::core::ffi::c_int
                                }))
                            {
                                dam = -(999 as ::core::ffi::c_int);
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
                                    RF_UNIQUE as ::core::ffi::c_int,
                                ) {
                                    borg_fear_mon_spell = false_0 != 0;
                                    p1 = borg_danger_one_kill(
                                        borg.c.y,
                                        borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        i,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    if (*kill).afraid as ::core::ffi::c_int != 0
                                        && *borg
                                            .trait_0
                                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                            <= 10 as ::core::ffi::c_int
                                    {
                                        p1 = p1 + 20 as ::core::ffi::c_int;
                                    }
                                    borg_fear_mon_spell = true_0 != 0;
                                    p2 = borg_danger_one_kill(
                                        borg.c.y,
                                        borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        i,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    borg_fear_mon_spell = false_0 != 0;
                                    dam = p1 - p2;
                                }
                            }
                        }
                    }
                }
            }
        }
        50 => {
            dam = 0 as ::core::ffi::c_int;
            if !(((*kill).speed as ::core::ffi::c_int) < (*r_ptr).speed - 5 as ::core::ffi::c_int) {
                if !(*kill).confused {
                    if (*kill).awake {
                        if !((*kill).level as ::core::ffi::c_int
                            > (if *borg
                                .trait_0
                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                < 13 as ::core::ffi::c_int
                            {
                                10 as ::core::ffi::c_int
                            } else {
                                (*borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    - 10 as ::core::ffi::c_int)
                                    / 4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    + 10 as ::core::ffi::c_int
                            }))
                        {
                            dam = -(999 as ::core::ffi::c_int);
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
                                RF_UNIQUE as ::core::ffi::c_int,
                            ) {
                                borg_slow_spell = false_0 != 0;
                                p1 = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if (*kill).afraid as ::core::ffi::c_int != 0
                                    && *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        <= 10 as ::core::ffi::c_int
                                {
                                    p1 = p1 + 20 as ::core::ffi::c_int;
                                }
                                borg_slow_spell = true_0 != 0;
                                p2 = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_slow_spell = false_0 != 0;
                                dam = p1 - p2;
                            }
                        }
                    }
                }
            }
        }
        51 | 14 => {
            dam = 0 as ::core::ffi::c_int;
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
                RF_NO_SLEEP as ::core::ffi::c_int,
            ) {
                if !(!flag_has(
                    &raw mut (*r_ptr).flags as *mut bitflag,
                    (RF_MAX as ::core::ffi::c_int as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        )
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(
                            (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                        ),
                    RF_EVIL as ::core::ffi::c_int,
                ) && typ == BORG_ATTACK_SLEEP_EVIL as ::core::ffi::c_int)
                {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if !(*kill).confused {
                            if (*kill).awake {
                                if !((*kill).level as ::core::ffi::c_int
                                    > (if *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        < 13 as ::core::ffi::c_int
                                    {
                                        10 as ::core::ffi::c_int
                                    } else {
                                        (*borg
                                            .trait_0
                                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                            - 10 as ::core::ffi::c_int)
                                            / 4 as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int
                                    }))
                                {
                                    dam = -(999 as ::core::ffi::c_int);
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
                                        RF_UNIQUE as ::core::ffi::c_int,
                                    ) {
                                        borg_sleep_spell = false_0 != 0;
                                        p1 = borg_danger_one_kill(
                                            borg.c.y,
                                            borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        if (*kill).afraid as ::core::ffi::c_int != 0
                                            && *borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                                <= 10 as ::core::ffi::c_int
                                        {
                                            p1 = p1 + 20 as ::core::ffi::c_int;
                                        }
                                        borg_sleep_spell = true_0 != 0;
                                        p2 = borg_danger_one_kill(
                                            borg.c.y,
                                            borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            i,
                                            true_0 != 0,
                                            true_0 != 0,
                                        );
                                        borg_sleep_spell = false_0 != 0;
                                        dam = p1 - p2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        52 => {
            dam = 0 as ::core::ffi::c_int;
            if !((*kill).level as ::core::ffi::c_int
                > (if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 13 as ::core::ffi::c_int
                {
                    10 as ::core::ffi::c_int
                } else {
                    (*borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        - 10 as ::core::ffi::c_int)
                        / 4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int
                }))
            {
                dam = -(999 as ::core::ffi::c_int);
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
                    dam = borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        2 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if dam < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        && !(*kill).afraid
                    {
                        dam = 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        53 => {
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
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
                if !(*kill).confused {
                    if !(((*kill).speed as ::core::ffi::c_int)
                        < (*r_ptr).speed - 5 as ::core::ffi::c_int)
                    {
                        if (*kill).awake {
                            if !((*kill).level as ::core::ffi::c_int
                                > *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    - 5 as ::core::ffi::c_int)
                            {
                                borg_fear_mon_spell = false_0 != 0;
                                p1 = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_fear_mon_spell = true_0 != 0;
                                p2 = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    i,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                borg_fear_mon_spell = false_0 != 0;
                                dam = p1 - p2;
                            }
                        }
                    }
                }
            } else {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        54 => {
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
                RF_EVIL as ::core::ffi::c_int,
            ) {
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
                    if !(*r_ptr).friends.is_null() || !(*r_ptr).friends_base.is_null() {
                        dam = 0 as ::core::ffi::c_int;
                    } else {
                        dam = -(500 as ::core::ffi::c_int);
                    }
                } else {
                    dam = borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                }
            } else {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        55 => {
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
                RF_UNDEAD as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else {
                let mut sp_drain: ::core::ffi::c_int =
                    *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                        - *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize);
                if sp_drain < (*kill).power as ::core::ffi::c_int {
                    dam = (*kill).power as ::core::ffi::c_int - sp_drain;
                }
            }
        }
        58 => {
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 12 as ::core::ffi::c_int
                * (50 as ::core::ffi::c_int
                    + (*kill).injury as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
        }
        57 => {
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
                RF_IM_ELEC as ::core::ffi::c_int,
            ) {
                dam = 0 as ::core::ffi::c_int;
            } else if !borg_projectable_pure(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                dam = 0 as ::core::ffi::c_int;
            }
        }
        0 | 3 | _ => {}
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 1 as ::core::ffi::c_int
        && (borg_danger_one_kill(
            (*kill).pos.y,
            (*kill).pos.x,
            1 as ::core::ffi::c_int,
            i,
            true_0 != 0,
            true_0 != 0,
        ) > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
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
            || gold_eater as ::core::ffi::c_int != 0
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
                RF_NEVER_MOVE as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 20 as ::core::ffi::c_int)
    {
        borg_use_missile = true_0 != 0;
    }
    if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_EVIL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int
    {
        return dam;
    }
    if dam > (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int
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
            RF_UNIQUE as ::core::ffi::c_int,
        )
    {
        dam = (*kill).power as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
    {
        dam = dam * 3 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_UNIQUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
    {
        dam = dam * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 5 as ::core::ffi::c_int
        {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_MULTIPLY as ::core::ffi::c_int,
    ) {
        dam = dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).spell_flags as *mut bitflag,
        (RSF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RSF_S_KIN as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_MONSTERS as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_ANIMAL as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_SPIDER as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HOUND as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HYDRA as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_AINU as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DEMON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_DRAGON as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_HI_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_WRAITH as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
        || flag_has(
            &raw mut (*r_ptr).spell_flags as *mut bitflag,
            (RSF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            RSF_S_UNIQUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        dam += dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_QUESTOR as ::core::ffi::c_int,
    ) {
        dam += dam * 9 as ::core::ffi::c_int;
    }
    if typ == BORG_ATTACK_ARROW as ::core::ffi::c_int {
        if !borg_use_missile {
            dam = 0 as ::core::ffi::c_int;
        }
    }
    return dam;
}
unsafe extern "C" fn borg_launch_bolt_aux_hack(
    mut i: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut p1: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut walls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unknown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    kill = borg_kills.offset(i as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.time.now - (*kill).when > 2 as borg_time {
        return 0 as ::core::ffi::c_int;
    }
    x = (*kill).pos.x;
    y = (*kill).pos.y;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    if !borg_cave_floor_grid(ag) {
        return 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && ((*ag).feat as ::core::ffi::c_int != FEAT_FLOOR as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int != FEAT_OPEN as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int != FEAT_BROKEN as ::core::ffi::c_int
            && !(*ag).trap)
    {
        return 0 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_PASS_WALL as ::core::ffi::c_int,
    ) {
        x = (*kill).pos.x;
        y = (*kill).pos.y;
        o_x = -(1 as ::core::ffi::c_int);
        while o_x <= 1 as ::core::ffi::c_int {
            o_y = -(1 as ::core::ffi::c_int);
            while o_y <= 1 as ::core::ffi::c_int {
                x = (*kill).pos.x + o_x;
                y = (*kill).pos.y + o_y;
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                    .offset(x as isize) as *mut borg_grid;
                if (*ag).feat as ::core::ffi::c_int >= FEAT_MAGMA as ::core::ffi::c_int
                    && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
                {
                    walls += 1;
                }
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    unknown += 1;
                }
                o_y += 1;
            }
            o_x += 1;
        }
        if walls >= 2 as ::core::ffi::c_int && unknown >= 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    d = borg_launch_damage_one(i, dam, typ, ammo_location);
    if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
        || typ == BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int
    {
        return d;
    }
    if typ == BORG_ATTACK_AWAY_EVIL as ::core::ffi::c_int {
        return d;
    }
    if d <= 0 as ::core::ffi::c_int {
        return d;
    }
    p2 = borg_danger_one_kill(y, x, 1 as ::core::ffi::c_int, i, true_0 != 0, false_0 != 0);
    if !(*kill).awake
        && p2 > borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        && d < (*kill).power as ::core::ffi::c_int
        && !borg.munchkin_mode
    {
        return -(999 as ::core::ffi::c_int);
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
        && !(*kill).awake
    {
        return 0 as ::core::ffi::c_int;
    }
    *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 3 as ::core::ffi::c_int;
    p1 = borg_danger_one_kill(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        i,
        true_0 != 0,
        false_0 != 0,
    );
    if d >= (*kill).power as ::core::ffi::c_int {
        d = 2 as ::core::ffi::c_int * d;
    }
    d = d + p1;
    return d;
}
unsafe extern "C" fn borg_launch_destroy_stuff(
    mut ag: *mut borg_grid,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut take: *mut borg_take = borg_takes.offset((*ag).take as isize) as *mut borg_take;
    let mut k_ptr: *mut object_kind = k_info.offset((*take).kidx as isize) as *mut object_kind;
    match typ {
        4 => {
            if (*k_ptr).tval == TV_BOOTS as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
        }
        5 => {
            if (*k_ptr).tval == TV_RING as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
            if (*k_ptr).tval == TV_RING as ::core::ffi::c_int && (*k_ptr).sval == sv_ring_speed {
                return 2000 as ::core::ffi::c_int;
            }
        }
        6 => {
            if (*k_ptr).tval == TV_BOOTS as ::core::ffi::c_int && !(*k_ptr).aware {
                return 20 as ::core::ffi::c_int;
            }
        }
        7 => {
            if (*k_ptr).tval == TV_POTION as ::core::ffi::c_int {
                if !(*k_ptr).aware
                    || (*k_ptr).sval == sv_potion_healing
                    || (*k_ptr).sval == sv_potion_star_healing
                    || (*k_ptr).sval == sv_potion_life
                    || (*k_ptr).sval == sv_potion_inc_str
                        && borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == sv_potion_inc_int
                        && borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == sv_potion_inc_wis
                        && borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == sv_potion_inc_dex
                        && borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                    || (*k_ptr).sval == sv_potion_inc_con
                        && borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                {
                    return 2000 as ::core::ffi::c_int;
                }
                return 20 as ::core::ffi::c_int;
            }
        }
        2 | _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_launch_bolt_at_location(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ry: ::core::ffi::c_int = 0;
    let mut rx: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    q_x = w_x / borg_panel_wid();
    q_y = w_y / borg_panel_hgt();
    n = 0 as ::core::ffi::c_int;
    x1 = borg.c.x;
    y1 = borg.c.y;
    x2 = x;
    y2 = y;
    if !square_in_bounds_fully(cave, loc(x, y)) {
        return 0 as ::core::ffi::c_int;
    }
    x = x1;
    y = y1;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize)).offset(x2 as isize)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    dist = 1 as ::core::ffi::c_int;
    while dist < max {
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        if !square_in_bounds_fully(cave, loc(x, y)) {
            break;
        }
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if !borg_cave_floor_grid(ag)
            || (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int
        {
            if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            } else {
                return n;
            }
        }
        if rad <= 0 as ::core::ffi::c_int || rad == 10 as ::core::ffi::c_int {
            n += borg_launch_bolt_aux_hack(
                (*ag).kill as ::core::ffi::c_int,
                dam,
                typ,
                ammo_location,
            );
        }
        if rad != -(1 as ::core::ffi::c_int)
            && rad != 10 as ::core::ffi::c_int
            && (x == x2 && y == y2)
        {
            break;
        }
        if rad == 0 && (*ag).kill as ::core::ffi::c_int != 0 {
            return n;
        }
        if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) == 0 {
            if *borg.trait_0.offset(BI_INFRA as ::core::ffi::c_int as isize)
                <= 0 as ::core::ffi::c_int
                && !((*r_ptr).light > 0 as ::core::ffi::c_int)
            {
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                } else {
                    return n;
                }
            }
        } else {
            if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && (borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
                    [(borg.c.x / 11 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    < borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
            {
                if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
                if successful_target < 0 as ::core::ffi::c_int {
                    if successful_target <= -(12 as ::core::ffi::c_int) {
                        successful_target = 0 as ::core::ffi::c_int;
                    }
                    if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return n;
                    }
                }
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                if rad != -(1 as ::core::ffi::c_int) && rad != 10 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                } else {
                    return n;
                }
            }
        }
        dist += 1;
    }
    if rad <= 0 as ::core::ffi::c_int {
        return n;
    }
    if dist >= max {
        return 0 as ::core::ffi::c_int;
    }
    ry = y2 - rad;
    while ry < y2 + rad {
        rx = x2 - rad;
        while rx < x2 + rad {
            if square_in_bounds(cave, loc(rx, ry)) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(ry as isize))
                    .offset(rx as isize) as *mut borg_grid;
                r = borg_distance(y2, x2, ry, rx);
                if !(r > rad) {
                    if borg_los(y2, x2, ry, rx) {
                        if rad == 10 as ::core::ffi::c_int {
                            r = 0 as ::core::ffi::c_int;
                        }
                        n += borg_launch_bolt_aux_hack(
                            (*ag).kill as ::core::ffi::c_int,
                            dam / (r + 1 as ::core::ffi::c_int),
                            typ,
                            ammo_location,
                        );
                        if (*ag).take as ::core::ffi::c_int != 0
                            && (*borg_takes.offset((*ag).take as isize)).kidx != 0
                        {
                            n -= borg_launch_destroy_stuff(ag, typ);
                        }
                    }
                }
            }
            rx += 1;
        }
        ry += 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_launch_bolt(
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut ammo_location: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut require_monster: bool = false_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        o_x = -(1 as ::core::ffi::c_int);
        while o_x <= 1 as ::core::ffi::c_int {
            o_y = -(1 as ::core::ffi::c_int);
            while o_y <= 1 as ::core::ffi::c_int {
                x = borg_temp_x[i as usize] as ::core::ffi::c_int + o_x;
                y = borg_temp_y[i as usize] as ::core::ffi::c_int + o_y;
                borg_tp_other_n = 0 as int16_t;
                n = 0 as ::core::ffi::c_int;
                if square_in_bounds(cave, loc(x, y)) {
                    d = distance(
                        borg.c,
                        loc(
                            borg_temp_x[i as usize] as ::core::ffi::c_int,
                            borg_temp_y[i as usize] as ::core::ffi::c_int,
                        ),
                    );
                    if !((x != borg_temp_x[i as usize] as ::core::ffi::c_int
                        || y != borg_temp_y[i as usize] as ::core::ffi::c_int)
                        && typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int)
                    {
                        if !(distance(borg.c, loc(x, y)) > max) {
                            if rad >= 2 as ::core::ffi::c_int
                                && (*borg_grids[y as usize].offset(x as isize)).feat
                                    as ::core::ffi::c_int
                                    != FEAT_NONE as ::core::ffi::c_int
                                || y == borg_temp_y[i as usize] as ::core::ffi::c_int
                                    && x == borg_temp_x[i as usize] as ::core::ffi::c_int
                            {
                                n = borg_launch_bolt_at_location(
                                    y,
                                    x,
                                    rad,
                                    dam,
                                    typ,
                                    max,
                                    ammo_location,
                                );
                            }
                            if typ == BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int
                                && n > 0 as ::core::ffi::c_int
                            {
                                n = borg_danger(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    true_0 != 0,
                                    false_0 != 0,
                                );
                                n = dam - n;
                            }
                            borg_tp_other_n = 0 as int16_t;
                            if !(n <= 0 as ::core::ffi::c_int) {
                                if !(x == 0 as ::core::ffi::c_int
                                    || y == 0 as ::core::ffi::c_int
                                    || x == DUNGEON_WID - 1 as ::core::ffi::c_int
                                    || y == DUNGEON_HGT - 1 as ::core::ffi::c_int)
                                {
                                    if !(b_i >= 0 as ::core::ffi::c_int && n < b_n) {
                                        if !(n == b_n && d > b_d) {
                                            b_i = i;
                                            b_n = n;
                                            b_o_y = o_y;
                                            b_o_x = o_x;
                                            b_d = d;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                o_y += 1;
            }
            o_x += 1;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return b_n;
    }
    borg_tp_other_n = 0 as int16_t;
    if borg_simulate {
        return b_n;
    }
    borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int + b_o_x;
    borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int + b_o_y;
    if typ == BORG_ATTACK_CURSE as ::core::ffi::c_int {
        require_monster = true_0 != 0;
    }
    borg_target(borg.goal.g, require_monster);
    return b_n;
}
unsafe extern "C" fn borg_launch_arc_at_location(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut degrees: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ry: ::core::ffi::c_int = 0;
    let mut rx: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut path_grids: [loc; 256] = [loc { x: 0, y: 0 }; 256];
    q_x = w_x / borg_panel_wid();
    q_y = w_y / borg_panel_hgt();
    n = 0 as ::core::ffi::c_int;
    x1 = borg.c.x;
    y1 = borg.c.y;
    x2 = x;
    y2 = y;
    if !square_in_bounds_fully(cave, loc(x, y)) {
        return 0 as ::core::ffi::c_int;
    }
    x = x1;
    y = y1;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize)).offset(x2 as isize)
        as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    path_grids[0 as ::core::ffi::c_int as usize].x = x;
    path_grids[0 as ::core::ffi::c_int as usize].y = y;
    dist = 1 as ::core::ffi::c_int;
    while dist < max {
        borg_inc_motion(&raw mut y, &raw mut x, y1, x1, y2, x2);
        if !square_in_bounds_fully(cave, loc(x, y)) {
            break;
        }
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if !borg_cave_floor_grid(ag)
            || (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int
        {
            break;
        }
        path_grids[dist as usize].x = x;
        path_grids[dist as usize].y = y;
        if x == x2 && y == y2 {
            break;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
            if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0 {
                if *borg.trait_0.offset(BI_INFRA as ::core::ffi::c_int as isize)
                    <= 0 as ::core::ffi::c_int
                    && !((*r_ptr).light > 0 as ::core::ffi::c_int)
                {
                    break;
                }
            } else if !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                && !*(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                    .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                && (borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
                    [(borg.c.x / 11 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    < borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
            {
                break;
            }
            if successful_target < 0 as ::core::ffi::c_int {
                if successful_target <= -(12 as ::core::ffi::c_int) {
                    successful_target = 0 as ::core::ffi::c_int;
                }
                break;
            }
        }
        dist += 1;
    }
    if dist < 21 as ::core::ffi::c_int {
        dist = dist - 1 as ::core::ffi::c_int;
    } else {
        dist = 20 as ::core::ffi::c_int;
    }
    let mut n1x: ::core::ffi::c_int = 0;
    let mut n1y: ::core::ffi::c_int = 0;
    let mut n2y: ::core::ffi::c_int = 0;
    let mut n2x: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut rotate: ::core::ffi::c_int = 0;
    let mut diff: ::core::ffi::c_int = 0;
    n1y = path_grids[dist as usize].y - y1 + 20 as ::core::ffi::c_int;
    n1x = path_grids[dist as usize].x - x1 + 20 as ::core::ffi::c_int;
    ry = y1 - max;
    while ry < y1 + max {
        rx = x1 - max;
        while rx < x1 + max {
            if square_in_bounds(cave, loc(rx, ry)) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(ry as isize))
                    .offset(rx as isize) as *mut borg_grid;
                r = borg_distance(y1, x1, ry, rx);
                if !(r > max) {
                    if borg_los(y1, x1, ry, rx) {
                        n2y = y - y1 + 20 as ::core::ffi::c_int;
                        n2x = x - x1 + 20 as ::core::ffi::c_int;
                        rotate = 90 as ::core::ffi::c_int
                            - get_angle_to_grid[n1y as usize][n1x as usize] as ::core::ffi::c_int;
                        tmp = (if get_angle_to_grid[n2y as usize][n2x as usize]
                            as ::core::ffi::c_int
                            + rotate
                            < 0 as ::core::ffi::c_int
                        {
                            -(get_angle_to_grid[n2y as usize][n2x as usize] as ::core::ffi::c_int
                                + rotate)
                        } else {
                            get_angle_to_grid[n2y as usize][n2x as usize] as ::core::ffi::c_int
                                + rotate
                        }) % 180 as ::core::ffi::c_int;
                        diff = if 90 as ::core::ffi::c_int - tmp < 0 as ::core::ffi::c_int {
                            -(90 as ::core::ffi::c_int - tmp)
                        } else {
                            90 as ::core::ffi::c_int - tmp
                        };
                        if !(diff >= (degrees + 6 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int)
                        {
                            if (*ag).kill != 0 {
                                n += borg_launch_bolt_aux_hack(
                                    (*ag).kill as ::core::ffi::c_int,
                                    dam / (r + 1 as ::core::ffi::c_int),
                                    typ,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            if (*ag).take as ::core::ffi::c_int != 0
                                && (*borg_takes.offset((*ag).take as isize)).kidx != 0
                            {
                                n -= borg_launch_destroy_stuff(ag, typ);
                            }
                        }
                    }
                }
            }
            rx += 1;
        }
        ry += 1;
    }
    return n;
}
unsafe extern "C" fn borg_launch_arc(
    mut degrees: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    if max > 20 as ::core::ffi::c_int {
        max = 20 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        n = 0 as ::core::ffi::c_int;
        d = distance(
            borg.c,
            loc(
                borg_temp_x[i as usize] as ::core::ffi::c_int,
                borg_temp_y[i as usize] as ::core::ffi::c_int,
            ),
        );
        if !(d > max) {
            n = borg_launch_arc_at_location(y, x, degrees, dam, typ, max);
            if !(n <= 0 as ::core::ffi::c_int) {
                if !(b_i >= 0 as ::core::ffi::c_int && n < b_n) {
                    if !(n == b_n && d > b_d) {
                        b_i = i;
                        b_n = n;
                        b_o_y = o_y;
                        b_o_x = o_x;
                        b_d = d;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return b_n;
    }
    if borg_simulate {
        return b_n;
    }
    borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int + b_o_x;
    borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int + b_o_y;
    borg_target(borg.goal.g, false_0 != 0);
    return b_n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack_aux_launch() -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut v: ::core::ffi::c_int = 0;
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut bow: *mut borg_item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if (*bow).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    k = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while k
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = borg_items.offset(k as isize) as *mut borg_item;
        if (*item).iqty == 0 {
            break;
        }
        if !((*item).tval as ::core::ffi::c_int
            != *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize))
        {
            if !((*item).value <= 0 as int32_t) {
                d = (*item).dd as ::core::ffi::c_int
                    * ((*item).ds as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
                d = d + (*item).to_d as ::core::ffi::c_int + (*bow).to_d as ::core::ffi::c_int;
                d = d
                    * *borg
                        .trait_0
                        .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize)
                    * *borg.trait_0.offset(BI_SHOTS as ::core::ffi::c_int as isize);
                v = (*item).value as ::core::ffi::c_int;
                if borg_item_note_needs_id(item) {
                    d = d * 99 as ::core::ffi::c_int;
                }
                if !(d <= 0 as ::core::ffi::c_int) {
                    n = borg_launch_bolt(
                        0 as ::core::ffi::c_int,
                        d,
                        BORG_ATTACK_ARROW as ::core::ffi::c_int,
                        6 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *borg
                                    .trait_0
                                    .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize),
                        k,
                    );
                    if !(n == b_n && v >= b_v) {
                        if n >= b_n {
                            b_n = n;
                            b_v = v;
                            b_k = k;
                        }
                    }
                }
            }
        }
        k += 1;
    }
    if b_n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Firing missile '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(b_k as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('f' as i32 as keycode_t);
    borg_keypress(
        (b_k - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int) + '0' as i32)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(2 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rest() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut my_danger: ::core::ffi::c_int = 0;
    if borg_simulate as ::core::ffi::c_int != 0 && borg.goal.waiting as ::core::ffi::c_int != 0 {
        borg.goal.waiting = false_0 != 0;
        return 0 as ::core::ffi::c_int;
    }
    my_danger = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
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
                if !(d != 2 as ::core::ffi::c_int) {
                    if !((*kill).ranged_attack != 0) {
                        if (*kill).awake {
                            if !(borg.time.now - (*kill).when > 10 as borg_time) {
                                if !flag_has(
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
                                    RF_NEVER_MOVE as ::core::ffi::c_int,
                                ) {
                                    if !((*kill).speed as ::core::ffi::c_int
                                        - *borg
                                            .trait_0
                                            .offset(BI_SPEED as ::core::ffi::c_int as isize)
                                        >= 5 as ::core::ffi::c_int)
                                    {
                                        if !((*kill).afraid as ::core::ffi::c_int != 0
                                            || (*kill).confused as ::core::ffi::c_int != 0
                                            || (*kill).stunned as ::core::ffi::c_int != 0)
                                        {
                                            if !(borg.goal.type_0 as ::core::ffi::c_int
                                                != GOAL_KILL
                                                || borg_flow_y[0 as ::core::ffi::c_int as usize]
                                                    as ::core::ffi::c_int
                                                    != (*kill).pos.y)
                                            {
                                                if borg_los(
                                                    borg.c.y,
                                                    borg.c.x,
                                                    (*kill).pos.y,
                                                    (*kill).pos.x,
                                                ) {
                                                    if !(my_danger
                                                        > *borg.trait_0.offset(
                                                            BI_CURHP as ::core::ffi::c_int as isize,
                                                        ))
                                                    {
                                                        break;
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
    if i >= borg_kills_nxt as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1 as ::core::ffi::c_int;
    }
    borg_keypress(',' as i32 as keycode_t);
    borg_note(format(
        b"# Waiting for monster '%s' to approach.\0" as *const u8 as *const ::core::ffi::c_char,
        (*r_info.offset((*borg_kills.offset(i as isize)).r_idx as isize)).name,
    ));
    borg.goal.waiting = true_0 != 0;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_object() -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut b_r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut div: ::core::ffi::c_int = 0;
    let mut mul: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).pack_size as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(k as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind) {
                if !(borg_wield_slot(item) >= 0 as ::core::ffi::c_int) {
                    d = (*k_info.offset((*item).kind as isize)).dd
                        * ((*k_info.offset((*item).kind as isize)).ds + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                    if !((*item).value > 100 as int32_t && d < 5 as ::core::ffi::c_int) {
                        if !(d <= 0 as ::core::ffi::c_int) {
                            if !((*item).tval as ::core::ffi::c_int
                                == TV_POTION as ::core::ffi::c_int)
                            {
                                if !((*item).tval as ::core::ffi::c_int
                                    == TV_FLASK as ::core::ffi::c_int
                                    && (*borg
                                        .trait_0
                                        .offset(BI_AFUEL as ::core::ffi::c_int as isize)
                                        <= 1 as ::core::ffi::c_int
                                        && borg.near.unique == 0))
                                {
                                    if !((*item).tval as ::core::ffi::c_int
                                        == TV_WAND as ::core::ffi::c_int
                                        || (*item).tval as ::core::ffi::c_int
                                            == TV_ROD as ::core::ffi::c_int)
                                    {
                                        if !(b_k >= 0 as ::core::ffi::c_int && d <= b_d) {
                                            b_k = k;
                                            b_d = d;
                                            mul = 10 as ::core::ffi::c_int;
                                            div = if (*item).weight as ::core::ffi::c_int
                                                > 10 as ::core::ffi::c_int
                                            {
                                                (*item).weight as ::core::ffi::c_int
                                            } else {
                                                10 as ::core::ffi::c_int
                                            };
                                            b_r = (adj_str_blow[*borg
                                                .trait_0
                                                .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
                                                as usize]
                                                + 20 as ::core::ffi::c_int)
                                                * mul
                                                / div;
                                            if b_r > 10 as ::core::ffi::c_int {
                                                b_r = 10 as ::core::ffi::c_int;
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
        k += 1;
    }
    if b_k < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        b_d,
        BORG_ATTACK_ARROW as ::core::ffi::c_int,
        6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *borg
                    .trait_0
                    .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize),
        b_k,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Throwing painful object '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(b_k as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('v' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_k as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(2 as ::core::ffi::c_int);
    return b_n;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack_aux_spell_bolt(
    spell: borg_spells,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max_range: ::core::ffi::c_int,
    mut is_arc: bool,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut penalty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_MAGE
            && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_NECROMANCER
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 2 as ::core::ffi::c_int)
        && (Rand_div(100 as uint32_t) as int32_t) < 1 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && (*borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
            && (borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
                || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0))
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(
        spell,
        if borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    if is_arc {
        b_n = borg_launch_arc(rad, dam, typ, max_range);
    } else {
        b_n = borg_launch_bolt(rad, dam, typ, max_range, 0 as ::core::ffi::c_int);
    }
    let mut primary_spell_for_class: borg_spells = MAGIC_MISSILE;
    match *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) {
        CLASS_MAGE => {
            primary_spell_for_class = MAGIC_MISSILE;
        }
        CLASS_DRUID => {
            primary_spell_for_class = STINKING_CLOUD;
        }
        CLASS_PRIEST => {
            primary_spell_for_class = ORB_OF_DRAINING;
        }
        CLASS_NECROMANCER => {
            primary_spell_for_class = NETHER_BOLT;
        }
        CLASS_PALADIN | CLASS_ROGUE | CLASS_RANGER | CLASS_BLACKGUARD | _ => {}
    }
    if spell as ::core::ffi::c_uint == primary_spell_for_class as ::core::ffi::c_uint
        && (!borg_spell_legal_fail(TELEPORT_SELF, 15 as ::core::ffi::c_int)
            || *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                <= 30 as ::core::ffi::c_int)
    {
        if borg_simulate {
            return b_n;
        }
    }
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(spell);
    if spell as ::core::ffi::c_uint != primary_spell_for_class as ::core::ffi::c_uint {
        b_n = b_n - spell_power;
        if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) < 50 as ::core::ffi::c_int
            && spell_power > b_n
        {
            b_n = b_n - spell_power;
        }
        if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
            < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int
        {
            b_n = b_n - spell_power * 3 as ::core::ffi::c_int;
        }
        if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
            < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
        {
            b_n = b_n - spell_power * 5 as ::core::ffi::c_int;
        }
    }
    match *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) {
        CLASS_MAGE => {
            penalty = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            penalty = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            penalty = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            penalty = 10 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power < penalty
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_spell(spell);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_spell_bolt_reserve(
    spell: borg_spells,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut max_range: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut x9: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut near_monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sv_mana: ::core::ffi::c_int =
        *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 15 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && (*borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
            && borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    ) < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if (*kill).r_idx == 0 {
            i += 1;
        } else {
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
            if d < 7 as ::core::ffi::c_int {
                near_monsters += 1;
            }
            if (*kill).power as ::core::ffi::c_int > dam + 4 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            break;
        }
    }
    if near_monsters > 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) =
        *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize);
    if !borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) = sv_mana;
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(rad, dam, typ, max_range, 0 as ::core::ffi::c_int);
    if borg_simulate {
        *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) = sv_mana;
        return b_n;
    }
    *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) =
        *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize);
    if borg_spell_fail(spell, 25 as ::core::ffi::c_int) {
        borg_note(
            b"# Emergency use of an Attack Spell.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    borg_queue_direction('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_spell_dispel(
    spell: borg_spells,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
    let mut penalty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && (*borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
            && (borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
                || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0))
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(spell, 25 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        10 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(spell);
    b_n = b_n - spell_power;
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 3 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) / 3 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 5 as ::core::ffi::c_int;
    }
    match *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) {
        CLASS_MAGE => {
            penalty = 6 as ::core::ffi::c_int;
        }
        CLASS_RANGER => {
            penalty = 22 as ::core::ffi::c_int;
        }
        CLASS_ROGUE => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_PRIEST => {
            penalty = 8 as ::core::ffi::c_int;
        }
        CLASS_PALADIN => {
            penalty = 20 as ::core::ffi::c_int;
        }
        CLASS_NECROMANCER => {
            penalty = 10 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power < penalty
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
            < 6 as ::core::ffi::c_int
    {
        b_n = b_n - spell_power * 750 as ::core::ffi::c_int;
    }
    if target_closest < 0 as ::core::ffi::c_int
        && BORG_ATTACK_TAP_UNLIFE as ::core::ffi::c_int == typ
        && b_n > 0 as ::core::ffi::c_int
    {
        target_closest = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_n;
    }
    borg_spell(spell);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_staff_dispel(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_staff_fail(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    b_n = b_n - 50 as ::core::ffi::c_int;
    if borg_simulate {
        return b_n;
    }
    borg_use_staff(sval);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rod_bolt(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if (500 as ::core::ffi::c_int) < borg_activate_failure(TV_ROD as ::core::ffi::c_int, sval) {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_rod(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_zap_rod(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_wand_bolt(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = borg_slot(TV_WAND as ::core::ffi::c_int, sval);
    if i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg_items.offset(i as isize)).pval == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (500 as ::core::ffi::c_int) < borg_activate_failure(TV_WAND as ::core::ffi::c_int, sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        > 5 as ::core::ffi::c_int
    {
        b_n = b_n - 5 as ::core::ffi::c_int;
    }
    if sval == sv_wand_wonder && !borg.munchkin_mode {
        if b_n > 0 as ::core::ffi::c_int
            && borg_danger(
                borg.c.y,
                borg.c.x,
                1 as ::core::ffi::c_int,
                true_0 != 0,
                false_0 != 0,
            ) >= borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        {
            b_n = 999 as ::core::ffi::c_int;
            borg_note(format(
                b"# Emergency use of a Wand of Wonder.\0" as *const u8
                    as *const ::core::ffi::c_char,
            ));
        } else {
            b_n = 0 as ::core::ffi::c_int;
        }
    }
    if borg_simulate {
        return b_n;
    }
    borg_aim_wand(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_wand_bolt_unknown(
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 5 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        if !((*borg_items.offset(i as isize)).iqty == 0) {
            if !((*borg_items.offset(i as isize)).tval as ::core::ffi::c_int
                != TV_WAND as ::core::ffi::c_int)
            {
                if !(*borg_items.offset(i as isize)).aware {
                    if !((*borg_items.offset(i as isize)).pval == 0) {
                        b_i = i;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Aiming unknown wand '%s.'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(b_i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('a' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    borg.trying_unknown = true_0 != 0;
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_rod_bolt_unknown(
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 5 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        if !((*borg_items.offset(i as isize)).iqty == 0) {
            if !((*borg_items.offset(i as isize)).tval as ::core::ffi::c_int
                != TV_ROD as ::core::ffi::c_int)
            {
                if !(*borg_items.offset(i as isize)).aware {
                    if !((*borg_items.offset(i as isize)).pval == 0) {
                        if strstr(
                            &raw mut (*borg_items.offset(i as isize)).desc
                                as *mut ::core::ffi::c_char,
                            b"tried\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        {
                            b_i = i;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_note(format(
        b"# Aiming unknown rod '%s.'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset(b_i as isize)).desc as *mut ::core::ffi::c_char,
    ));
    borg_keypress('z' as i32 as keycode_t);
    borg_keypress(
        *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
            as keycode_t,
    );
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    borg.trying_unknown = true_0 != 0;
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_activation(
    mut activation: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut aim: bool,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(activation, true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_activate_item(activation);
    if aim {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
    }
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_ring(
    mut ring_name: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_ring(ring_name) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_bolt(
        rad,
        dam,
        typ,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if borg_simulate {
        return b_n;
    }
    borg_activate_ring(ring_name);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_dragon(
    mut sval: ::core::ffi::c_int,
    mut rad: ::core::ffi::c_int,
    mut dam: ::core::ffi::c_int,
    mut typ: ::core::ffi::c_int,
    mut selection: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_n: ::core::ffi::c_int = 0;
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
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 2 as int32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize))
    .art_idx
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_dragon(sval) {
        return 0 as ::core::ffi::c_int;
    }
    b_n = borg_launch_arc(rad, dam, typ, (*z_info).max_range as ::core::ffi::c_int);
    if borg_simulate {
        return b_n;
    }
    borg_activate_dragon(sval);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    if selection != -(1 as ::core::ffi::c_int) {
        borg_keypress(('b' as i32 + selection) as keycode_t);
    }
    return b_n;
}
unsafe extern "C" fn borg_attack_aux_whirlwind_attack() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut total_d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        WHIRLWIND_ATTACK,
        if borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    let mut blows: ::core::ffi::c_int = (*borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        + 10 as ::core::ffi::c_int)
        / 15 as ::core::ffi::c_int;
    let mut current_block_12: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(distance(borg.c, loc(x, y)) > 1 as ::core::ffi::c_int) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
            if !(d <= 0 as ::core::ffi::c_int) {
                d = d * blows;
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !(*kill).awake && d <= (*kill).power as ::core::ffi::c_int && !borg.munchkin_mode
                {
                    p = borg_danger_one_kill(
                        y,
                        x,
                        1 as ::core::ffi::c_int,
                        (*ag).kill as ::core::ffi::c_int,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if p > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                        current_block_12 = 7502529970979898288;
                    } else {
                        current_block_12 = 3512920355445576850;
                    }
                } else {
                    current_block_12 = 3512920355445576850;
                }
                match current_block_12 {
                    7502529970979898288 => {}
                    _ => {
                        if !(*borg
                            .trait_0
                            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                            == 0
                            && !(*kill).awake)
                        {
                            p = borg_danger_one_kill(
                                borg.c.y,
                                borg.c.x,
                                2 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                true_0 != 0,
                            );
                            if d <= (*kill).power as ::core::ffi::c_int
                                && *borg
                                    .trait_0
                                    .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                    > 15 as ::core::ffi::c_int
                            {
                                p = p / 10 as ::core::ffi::c_int;
                            }
                            d += p;
                            total_d += d;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if total_d < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return total_d;
    }
    if borg_spell(WHIRLWIND_ATTACK) {
        return total_d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_leap_into_battle() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        LEAP_INTO_BATTLE,
        if borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
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
        return 0 as ::core::ffi::c_int;
    }
    if target_closest < 10 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut current_block_19: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut blows: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        let mut m_dist: ::core::ffi::c_int = distance(borg.c, loc(x, y));
        if !(m_dist > 4 as ::core::ffi::c_int) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !((*ag).kill == 0) {
                d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int);
                blows = (*borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    + 5 as ::core::ffi::c_int)
                    / 15 as ::core::ffi::c_int;
                blows = (blows * m_dist + 2 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int;
                d *= blows;
                if !(d <= 0 as ::core::ffi::c_int) {
                    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                    if !((*kill).r_idx as ::core::ffi::c_int
                        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    {
                        if !(*kill).awake
                            && d <= (*kill).power as ::core::ffi::c_int
                            && !borg.munchkin_mode
                        {
                            p = borg_danger_one_kill(
                                y,
                                x,
                                1 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                true_0 != 0,
                            );
                            if p > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                                current_block_19 = 4906268039856690917;
                            } else {
                                current_block_19 = 13797916685926291137;
                            }
                        } else {
                            current_block_19 = 13797916685926291137;
                        }
                        match current_block_19 {
                            4906268039856690917 => {}
                            _ => {
                                if !(*borg
                                    .trait_0
                                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                    == 0
                                    && !(*kill).awake)
                                {
                                    p = borg_danger_one_kill(
                                        borg.c.y,
                                        borg.c.x,
                                        2 as ::core::ffi::c_int,
                                        (*ag).kill as ::core::ffi::c_int,
                                        true_0 != 0,
                                        true_0 != 0,
                                    );
                                    if d <= (*kill).power as ::core::ffi::c_int
                                        && *borg
                                            .trait_0
                                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                            > 15 as ::core::ffi::c_int
                                    {
                                        p = p / 10 as ::core::ffi::c_int;
                                    }
                                    d += p;
                                    if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
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
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.goal.g.y as isize))
        .offset(borg.goal.g.x as isize) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    borg_note(format(
        b"# Leaping at %s at (%d,%d dist %d) who has %d Hit Points.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg_race_name((*kill).r_idx as ::core::ffi::c_int),
        borg.goal.g.y,
        borg.goal.g.x,
        distance(borg.c, borg.goal.g),
        (*kill).power as ::core::ffi::c_int,
    ));
    borg_note(format(
        b"# Attacking with weapon '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*borg_items.offset((*z_info).pack_size as isize)).desc
            as *mut ::core::ffi::c_char,
    ));
    borg_target(borg.goal.g, true_0 != 0);
    borg_spell(LEAP_INTO_BATTLE);
    borg_keypress('5' as i32 as keycode_t);
    successful_target = -(1 as ::core::ffi::c_int);
    return b_d;
}
unsafe extern "C" fn borg_attack_aux_maim_foe() -> ::core::ffi::c_int {
    let mut blows: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(
        MAIM_FOE,
        if borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    blows = *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        / 15 as ::core::ffi::c_int;
    let mut current_block_15: u64;
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = borg_temp_x[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = borg_temp_y[i as usize] as ::core::ffi::c_int;
        if !(distance(borg.c, loc(x, y)) > 1 as ::core::ffi::c_int) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            d = borg_thrust_damage_one((*ag).kill as ::core::ffi::c_int) * blows;
            if !(d <= 0 as ::core::ffi::c_int) {
                kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                if !((*kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    if !(*kill).awake
                        && d <= (*kill).power as ::core::ffi::c_int
                        && !borg.munchkin_mode
                    {
                        p = borg_danger_one_kill(
                            y,
                            x,
                            1 as ::core::ffi::c_int,
                            (*ag).kill as ::core::ffi::c_int,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        if p > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                            current_block_15 = 6483416627284290920;
                        } else {
                            current_block_15 = 1054647088692577877;
                        }
                    } else {
                        current_block_15 = 1054647088692577877;
                    }
                    match current_block_15 {
                        6483416627284290920 => {}
                        _ => {
                            if !(*borg
                                .trait_0
                                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                == 0
                                && !(*kill).awake)
                            {
                                p = borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    2 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                );
                                if d <= (*kill).power as ::core::ffi::c_int
                                    && *borg
                                        .trait_0
                                        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                        > 15 as ::core::ffi::c_int
                                {
                                    p = p / 10 as ::core::ffi::c_int;
                                }
                                d += p;
                                if !(b_i >= 0 as ::core::ffi::c_int && d < b_d) {
                                    b_i = i;
                                    b_d = d;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if b_i < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return b_d;
    }
    borg.goal.g.x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    borg.goal.g.y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.goal.g.y as isize))
        .offset(borg.goal.g.x as isize) as *mut borg_grid;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    dir = borg_extract_dir(borg.c.y, borg.c.x, borg.goal.g.y, borg.goal.g.x);
    if borg_simulate {
        return d;
    }
    borg_spell(MAIM_FOE);
    borg_keypress((dir + '0' as i32) as keycode_t);
    return d;
}
unsafe extern "C" fn borg_attack_aux_vampire_strike() -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut best_dist: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut o_x: ::core::ffi::c_int = 0;
    let mut o_y: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_spell_okay_fail(
        VAMPIRE_STRIKE,
        if borg.near.unique as ::core::ffi::c_int != 0 {
            40 as ::core::ffi::c_int
        } else {
            25 as ::core::ffi::c_int
        },
    ) {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        x = borg_temp_x[i as usize] as ::core::ffi::c_int;
        y = borg_temp_y[i as usize] as ::core::ffi::c_int;
        dist = distance(borg.c, loc(x, y));
        if !(dist > best_dist) {
            best_dist = dist;
            b_i = i;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if dist >= 20 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    x = borg_temp_x[b_i as usize] as ::core::ffi::c_int;
    y = borg_temp_y[b_i as usize] as ::core::ffi::c_int;
    let mut found: bool = false_0 != 0;
    o_x = -(1 as ::core::ffi::c_int);
    while o_x <= 1 as ::core::ffi::c_int && !found {
        o_y = -(1 as ::core::ffi::c_int);
        while o_y <= 1 as ::core::ffi::c_int && !found {
            if !(o_x == 0 && o_y == 0) {
                x2 = borg_temp_x[b_i as usize] as ::core::ffi::c_int + o_x;
                y2 = borg_temp_y[b_i as usize] as ::core::ffi::c_int + o_y;
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize))
                    .offset(x2 as isize) as *mut borg_grid;
                if (*ag).kill == 0
                    && (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
                    && !(*ag).web
                    && !(*ag).glyph
                    && (y2 != borg.c.y || x2 != borg.c.x)
                {
                    found = true_0 != 0;
                }
            }
            o_y += 1;
        }
        o_x += 1;
    }
    if !found {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_offset_projectable(borg.c.y, borg.c.x, y, x) {
        return 0 as ::core::ffi::c_int;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    d = *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        * 2 as ::core::ffi::c_int;
    kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
    if (*kill).r_idx as ::core::ffi::c_int
        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut r_ptr: *mut monster_race = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
    if flag_has(
        &raw mut (*r_ptr).flags as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        RF_NONLIVING as ::core::ffi::c_int,
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
            RF_UNDEAD as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*kill).awake && d <= (*kill).power as ::core::ffi::c_int && !borg.munchkin_mode {
        p = borg_danger_one_kill(
            y,
            x,
            1 as ::core::ffi::c_int,
            (*ag).kill as ::core::ffi::c_int,
            true_0 != 0,
            true_0 != 0,
        );
        if p > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    p = borg_danger_one_kill(
        borg.c.y,
        borg.c.x,
        2 as ::core::ffi::c_int,
        (*ag).kill as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    );
    if d <= (*kill).power as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            > 15 as ::core::ffi::c_int
    {
        p = p / 10 as ::core::ffi::c_int;
    }
    d += p;
    if target_closest < 0 as ::core::ffi::c_int && d > 0 as ::core::ffi::c_int {
        target_closest = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    borg_spell(VAMPIRE_STRIKE);
    return d;
}
unsafe extern "C" fn borg_attack_aux_crush() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_spell_okay(CRUSH) {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) + 10 as ::core::ffi::c_int)
        < *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    borg_crush_spell = false_0 != 0;
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_crush_spell = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_crush_spell = false_0 != 0;
    d = p1 - p2;
    let mut new_hp: ::core::ffi::c_int =
        *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            - *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int;
    if borg_simulate as ::core::ffi::c_int != 0
        && (p2 >= new_hp * 2 as ::core::ffi::c_int || new_hp <= 50 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(CRUSH);
    d = d - spell_power;
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        d = d - spell_power * 10 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    if borg_spell(CRUSH) {
        return d;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn borg_attack_aux_trance() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_spell_okay(TRANCE) {
        return 0 as ::core::ffi::c_int;
    }
    borg_sleep_spell_ii = false_0 != 0;
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = false_0 != 0;
    d = p1 - p2;
    let mut spell_power: ::core::ffi::c_int = borg_get_spell_power(TRANCE);
    d = d - spell_power;
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) - spell_power
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        d = d - spell_power * 10 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    if borg_spell(TRANCE) {
        return d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_attack_aux_artifact_holcolleth() -> ::core::ffi::c_int {
    let mut p1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !borg_equips_item(act_sleepii, true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    borg_sleep_spell_ii = false_0 != 0;
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        4 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_sleep_spell_ii = false_0 != 0;
    d = p1 - p2;
    if borg_simulate {
        return d;
    }
    if borg_activate_item(act_sleepii) {
        return d;
    } else {
        borg_note(
            b"# Failed to properly activate the artifact\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_calculate_attack_effectiveness(
    mut attack_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match attack_type {
        0 => return borg_attack_aux_rest(),
        1 => return borg_attack_aux_thrust(),
        3 => return borg_attack_aux_launch(),
        2 => return borg_attack_aux_object(),
        10 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SLOW_MONSTER,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        15 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                CONFUSE_MONSTER,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        11 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                MASS_SLEEP,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        4 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MAGIC_MISSILE,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        5 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
            return borg_attack_aux_spell_bolt_reserve(
                MAGIC_MISSILE,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
            );
        }
        8 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 5 as ::core::ffi::c_int)
                / 3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FROST_BOLT,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        9 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int + 30 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                TURN_STONE_TO_MUD,
                rad,
                dam,
                BORG_ATTACK_KILL_WALL as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        7 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SPEAR_OF_LIGHT,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        6 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                STINKING_CLOUD,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        12 => {
            rad = 2 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FIRE_BALL,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        16 => {
            dam = 3 as ::core::ffi::c_int
                * (*borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                ICE_STORM,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
            );
        }
        17 => {
            rad = 1 as ::core::ffi::c_int;
            dam = 30 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 20 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                METEOR_SWARM,
                rad,
                dam,
                BORG_ATTACK_METEOR as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        18 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                RIFT,
                rad,
                dam,
                BORG_ATTACK_GRAVITY as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        19 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MANA_STORM,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        13 => {
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int;
            rad = 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SHOCK_WAVE,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        14 => {
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 5 as ::core::ffi::c_int;
            rad = 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                EXPLOSION,
                rad,
                dam,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        22 => {
            rad = if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                >= 30 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ORB_OF_DRAINING,
                rad,
                dam,
                BORG_ATTACK_HOLY_ORB as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        20 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                FRIGHTEN,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        21 => return borg_attack_aux_trance(),
        23 => {
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                DISPEL_UNDEAD,
                dam,
                BORG_ATTACK_DISP_UNDEAD as ::core::ffi::c_int,
            );
        }
        24 => {
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                DISPEL_EVIL,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        25 => {
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                BANISH_SPIRITS,
                dam,
                BORG_ATTACK_DISP_SPIRITS as ::core::ffi::c_int,
            );
        }
        26 => {
            if *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                - *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                >= 300 as ::core::ffi::c_int
            {
                dam = *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 10 as ::core::ffi::c_int;
                return borg_attack_aux_spell_dispel(
                    HOLY_WORD,
                    dam,
                    BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                );
            } else {
                dam = *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 3 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int
                    - 50 as ::core::ffi::c_int;
                return borg_attack_aux_spell_dispel(
                    DISPEL_EVIL,
                    dam,
                    BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                );
            }
        }
        27 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 4 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ANNIHILATE,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        28 => {
            rad = 0 as ::core::ffi::c_int;
            dam = ((*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ELECTRIC_ARC,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize),
                false_0 != 0,
            );
        }
        29 => {
            rad = 60 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                ACID_SPRAY,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                10 as ::core::ffi::c_int,
                true_0 != 0,
            );
        }
        30 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 10 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                MANA_BOLT,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        31 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                THRUST_AWAY,
                rad,
                dam,
                BORG_ATTACK_FORCE as ::core::ffi::c_int,
                *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 10 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        32 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 5 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                LIGHTNING_STRIKE,
                rad,
                dam,
                BORG_ATTACK_ELEC_STRIKE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        33 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int)
                * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 5 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                EARTH_RISING,
                rad,
                dam,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
                *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 5 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        34 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
                * (*borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                VOLCANIC_ERUPTION,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        35 => {
            rad = 20 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 10 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                RIVER_OF_LIGHTNING,
                rad,
                dam,
                BORG_ATTACK_PLASMA as ::core::ffi::c_int,
                20 as ::core::ffi::c_int,
                true_0 != 0,
            );
        }
        36 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int
                + (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                SPEAR_OF_OROME,
                rad,
                dam,
                BORG_ATTACK_HOLY_ORB as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        37 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 5 as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                LIGHT_OF_VARDA,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        38 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                NETHER_BOLT,
                rad,
                dam,
                BORG_ATTACK_NETHER as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        39 => {
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int)
                * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                TAP_UNLIFE,
                dam,
                BORG_ATTACK_TAP_UNLIFE as ::core::ffi::c_int,
            );
        }
        40 => return borg_attack_aux_crush(),
        41 => {
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 10 as ::core::ffi::c_int
                + 500 as ::core::ffi::c_int;
            return borg_attack_aux_spell_dispel(
                SLEEP_EVIL,
                dam,
                BORG_ATTACK_SLEEP_EVIL as ::core::ffi::c_int,
            );
        }
        42 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DISENCHANT,
                rad,
                dam,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        43 => {
            rad = 0 as ::core::ffi::c_int;
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize);
            return borg_attack_aux_spell_bolt(
                FRIGHTEN,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        44 => return borg_attack_aux_vampire_strike(),
        45 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DISPEL_LIFE,
                rad,
                dam,
                BORG_ATTACK_DRAIN_LIFE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        46 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                DARK_SPEAR,
                rad,
                dam,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        47 => {
            rad = 0 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 8 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                UNLEASH_CHAOS,
                rad,
                dam,
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        48 => {
            rad = 4 as ::core::ffi::c_int;
            dam = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                * 4 as ::core::ffi::c_int;
            return borg_attack_aux_spell_bolt(
                STORM_OF_DARKNESS,
                rad,
                dam,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        49 => {
            if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < 120 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            rad = 0 as ::core::ffi::c_int;
            dam = -(1 as ::core::ffi::c_int);
            return borg_attack_aux_spell_bolt(
                CURSE,
                rad,
                dam,
                BORG_ATTACK_CURSE as ::core::ffi::c_int,
                (*z_info).max_range as ::core::ffi::c_int,
                false_0 != 0,
            );
        }
        50 => return borg_attack_aux_whirlwind_attack(),
        51 => return borg_attack_aux_leap_into_battle(),
        52 => return borg_attack_aux_maim_foe(),
        53 => {
            dam = *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize);
            return borg_attack_aux_spell_dispel(
                HOWL_OF_THE_DAMNED,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
            );
        }
        64 => {
            dam = 10 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_slow_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
            );
        }
        65 => {
            dam = 10 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_sleep_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        54 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        55 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
            );
        }
        56 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        57 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        58 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_light,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
            );
        }
        59 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_drain_life,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
            );
        }
        60 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        61 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_cold_ball,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
            );
        }
        62 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        63 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt(
                sv_rod_fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        66 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_rod_bolt_unknown(
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
            );
        }
        72 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt_unknown(
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
            );
        }
        73 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 3 as ::core::ffi::c_int * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_magic_missile,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        78 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_slow_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        79 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_hold_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        81 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_fear_monster,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        80 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_confuse_monster,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        74 => {
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            rad = -(1 as ::core::ffi::c_int);
            return borg_attack_aux_wand_bolt(
                sv_wand_elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        75 => {
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            rad = 0 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        76 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        77 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        84 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_light,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        85 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_stinking_cloud,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        86 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        87 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_cold_ball,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        88 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        89 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        91 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 160 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_dragon_cold,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        92 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_dragon_fire,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        82 => {
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_annihilation,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        83 => {
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_drain_life,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        90 => {
            dam = 35 as ::core::ffi::c_int;
            return borg_attack_aux_wand_bolt(
                sv_wand_wonder,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        67 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                sv_staff_sleep_monsters,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
            );
        }
        68 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                sv_staff_slow_monsters,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
            );
        }
        69 => {
            dam = 60 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                sv_staff_dispel_evil,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        70 => {
            dam = 120 as ::core::ffi::c_int;
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                sv_staff_power,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
            );
        }
        71 => {
            if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int
            {
                dam = 500 as ::core::ffi::c_int;
            } else {
                dam = 120 as ::core::ffi::c_int;
            }
            rad = 10 as ::core::ffi::c_int;
            return borg_attack_aux_staff_dispel(
                sv_staff_holiness,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
            );
        }
        109 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 9 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_bolt,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        110 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 72 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_ball72,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        111 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 144 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_ball,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        112 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_bolt,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        113 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 50 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_ball50,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        115 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_bolt2,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        114 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_ball100,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        123 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_elec_bolt,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        118 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_stinking_cloud,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        117 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_drain_life2,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        119 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_confuse2,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        122 => {
            dam = 10 as ::core::ffi::c_int;
            return borg_attack_aux_artifact_holcolleth();
        }
        116 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 90 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_drain_life1,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        120 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_arrow,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        121 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 3 as ::core::ffi::c_int * (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_missile,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        124 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_acid_bolt,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        125 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 5 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dispel_evil,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        126 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_mana_bolt,
                rad,
                dam,
                BORG_ATTACK_MANA as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        127 => {
            rad = 3 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_star_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        128 => {
            rad = 7 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_starlight2,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        129 => {
            rad = 7 as ::core::ffi::c_int;
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_starlight,
                rad,
                dam,
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        130 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_mon_slow,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        131 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_mon_confuse,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        132 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_sleep_all,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        133 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_mon_scare,
                rad,
                dam,
                BORG_ATTACK_TURN_ALL as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        134 => {
            rad = -(1 as ::core::ffi::c_int);
            dam = 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_light_line,
                rad,
                dam,
                BORG_ATTACK_LIGHT_WEAK as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        135 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_drain_life3,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        136 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_drain_life4,
                rad,
                dam,
                BORG_ATTACK_OLD_DRAIN as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        137 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 64 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_elec_ball,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        138 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_elec_ball2,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        139 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 10 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_acid_bolt2,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        140 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_acid_bolt2,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        141 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_acid_ball,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        142 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 160 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_ball160,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        143 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_cold_ball2,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        144 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_ball2,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        145 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_ball200,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        146 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 12 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_bolt2,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        147 => {
            rad = 0 as ::core::ffi::c_int;
            dam = 16 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_fire_bolt3,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        148 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dispel_evil60,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        149 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dispel_undead,
                rad,
                dam,
                BORG_ATTACK_DISP_UNDEAD as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        150 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 60 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dispel_undead,
                rad,
                dam,
                BORG_ATTACK_DISP_ALL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        151 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_losslow,
                rad,
                dam,
                BORG_ATTACK_OLD_SLOW as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        152 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 20 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_lossleep,
                rad,
                dam,
                BORG_ATTACK_OLD_SLEEP as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        153 => {
            rad = 10 as ::core::ffi::c_int;
            dam = 5 as ::core::ffi::c_int
                + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_losconf,
                rad,
                dam,
                BORG_ATTACK_OLD_CONF as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        154 => {
            dam = 5 as ::core::ffi::c_int
                + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_wonder,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        155 => {
            if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int
            {
                dam = 500 as ::core::ffi::c_int;
            } else {
                dam = 120 as ::core::ffi::c_int;
            }
            return borg_attack_aux_activation(
                act_staff_holy,
                rad,
                dam,
                BORG_ATTACK_DISP_EVIL as ::core::ffi::c_int,
                false_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        156 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 70 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_ring_acid,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        157 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 80 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_ring_flames,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        158 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_ring_ice,
                rad,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        159 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 85 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_ring_lightning,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        160 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dragon_blue,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        161 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dragon_green,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        162 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dragon_red,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        163 => {
            let mut value: [::core::ffi::c_int; 5] = [0; 5];
            let mut type_0: [::core::ffi::c_int; 5] = [
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            ];
            let mut biggest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 5 as ::core::ffi::c_int {
                value[x as usize] = borg_attack_aux_activation(
                    act_dragon_multihued,
                    rad,
                    dam,
                    type_0[x as usize],
                    true_0 != 0,
                    x,
                );
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_0 < 5 as ::core::ffi::c_int {
                if value[x_0 as usize] > value[biggest as usize] {
                    biggest = x_0;
                }
                x_0 += 1;
            }
            borg_simulate = tmp_simulate;
            if !borg_simulate {
                value[biggest as usize] = borg_attack_aux_activation(
                    act_dragon_multihued,
                    rad,
                    dam,
                    type_0[biggest as usize],
                    true_0 != 0,
                    biggest,
                );
            }
            return value[biggest as usize];
        }
        164 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dragon_gold,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        165 => {
            let mut value_0: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_1: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
            ];
            let mut biggest_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_0: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_1 < 2 as ::core::ffi::c_int {
                value_0[x_1 as usize] = borg_attack_aux_activation(
                    act_dragon_chaos,
                    rad,
                    dam,
                    type_1[x_1 as usize],
                    true_0 != 0,
                    x_1,
                );
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_2 < 2 as ::core::ffi::c_int {
                if value_0[x_2 as usize] > value_0[biggest_0 as usize] {
                    biggest_0 = x_2;
                }
                x_2 += 1;
            }
            borg_simulate = tmp_simulate_0;
            if !borg_simulate {
                value_0[biggest_0 as usize] = borg_attack_aux_activation(
                    act_dragon_chaos,
                    rad,
                    dam,
                    type_1[biggest_0 as usize],
                    true_0 != 0,
                    biggest_0,
                );
            }
            return value_0[biggest_0 as usize];
        }
        166 => {
            let mut value_1: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_2: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_1: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_3 < 2 as ::core::ffi::c_int {
                value_1[x_3 as usize] = borg_attack_aux_activation(
                    act_dragon_law,
                    rad,
                    dam,
                    type_2[x_3 as usize],
                    true_0 != 0,
                    x_3,
                );
                x_3 += 1;
            }
            let mut x_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_4 < 2 as ::core::ffi::c_int {
                if value_1[x_4 as usize] > value_1[biggest_1 as usize] {
                    biggest_1 = x_4;
                }
                x_4 += 1;
            }
            borg_simulate = tmp_simulate_1;
            if !borg_simulate {
                value_1[biggest_1 as usize] = borg_attack_aux_activation(
                    act_dragon_law,
                    rad,
                    dam,
                    type_2[biggest_1 as usize],
                    true_0 != 0,
                    biggest_1,
                );
            }
            return value_1[biggest_1 as usize];
        }
        167 => {
            let mut value_2: [::core::ffi::c_int; 4] = [0; 4];
            let mut type_3: [::core::ffi::c_int; 4] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_2: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_5 < 4 as ::core::ffi::c_int {
                value_2[x_5 as usize] = borg_attack_aux_activation(
                    act_dragon_balance,
                    rad,
                    dam,
                    type_3[x_5 as usize],
                    true_0 != 0,
                    x_5,
                );
                x_5 += 1;
            }
            let mut x_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_6 < 4 as ::core::ffi::c_int {
                if value_2[x_6 as usize] > value_2[biggest_2 as usize] {
                    biggest_2 = x_6;
                }
                x_6 += 1;
            }
            borg_simulate = tmp_simulate_2;
            if !borg_simulate {
                value_2[biggest_2 as usize] = borg_attack_aux_activation(
                    act_dragon_balance,
                    rad,
                    dam,
                    type_3[biggest_2 as usize],
                    true_0 != 0,
                    biggest_2,
                );
            }
            return value_2[biggest_2 as usize];
        }
        168 => {
            let mut value_3: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_4: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
            ];
            let mut biggest_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_3: bool = borg_simulate;
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_7 < 2 as ::core::ffi::c_int {
                value_3[x_7 as usize] = borg_attack_aux_activation(
                    act_dragon_shining,
                    rad,
                    dam,
                    type_4[x_7 as usize],
                    true_0 != 0,
                    x_7,
                );
                x_7 += 1;
            }
            let mut x_8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_8 < 2 as ::core::ffi::c_int {
                if value_3[x_8 as usize] > value_3[biggest_3 as usize] {
                    biggest_3 = x_8;
                }
                x_8 += 1;
            }
            borg_simulate = tmp_simulate_3;
            if !borg_simulate {
                value_3[biggest_3 as usize] = borg_attack_aux_activation(
                    act_dragon_shining,
                    rad,
                    dam,
                    type_4[biggest_3 as usize],
                    true_0 != 0,
                    biggest_3,
                );
            }
            return value_3[biggest_3 as usize];
        }
        169 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int;
            return borg_attack_aux_activation(
                act_dragon_power,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                true_0 != 0,
                -(1 as ::core::ffi::c_int),
            );
        }
        93 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 70 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                sv_ring_acid,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
            );
        }
        94 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 80 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                sv_ring_flames,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            );
        }
        95 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 75 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                sv_ring_ice,
                rad,
                dam,
                BORG_ATTACK_ICE as ::core::ffi::c_int,
            );
        }
        96 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 85 as ::core::ffi::c_int;
            return borg_attack_aux_ring(
                sv_ring_lightning,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
            );
        }
        97 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_blue,
                rad,
                dam,
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        98 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 100 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_white,
                rad,
                dam,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        99 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 120 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_black,
                rad,
                dam,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        100 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_green,
                rad,
                dam,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        101 => {
            rad = 2 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_red,
                rad,
                dam,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        102 => {
            let mut value_4: [::core::ffi::c_int; 5] = [0; 5];
            let mut type_5: [::core::ffi::c_int; 5] = [
                BORG_ATTACK_ELEC as ::core::ffi::c_int,
                BORG_ATTACK_COLD as ::core::ffi::c_int,
                BORG_ATTACK_ACID as ::core::ffi::c_int,
                BORG_ATTACK_POIS as ::core::ffi::c_int,
                BORG_ATTACK_FIRE as ::core::ffi::c_int,
            ];
            let mut biggest_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_4: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_9 < 5 as ::core::ffi::c_int {
                value_4[x_9 as usize] = borg_attack_aux_dragon(
                    sv_dragon_multihued,
                    rad,
                    dam,
                    type_5[x_9 as usize],
                    x_9,
                );
                x_9 += 1;
            }
            let mut x_10: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_10 < 5 as ::core::ffi::c_int {
                if value_4[x_10 as usize] > value_4[biggest_4 as usize] {
                    biggest_4 = x_10;
                }
                x_10 += 1;
            }
            borg_simulate = tmp_simulate_4;
            if !borg_simulate {
                value_4[biggest_4 as usize] = borg_attack_aux_dragon(
                    sv_dragon_multihued,
                    rad,
                    dam,
                    type_5[biggest_4 as usize],
                    biggest_4,
                );
            }
            return value_4[biggest_4 as usize];
        }
        103 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 150 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_gold,
                rad,
                dam,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        104 => {
            let mut value_5: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_6: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
            ];
            let mut biggest_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_5: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_11 < 2 as ::core::ffi::c_int {
                value_5[x_11 as usize] =
                    borg_attack_aux_dragon(sv_dragon_chaos, rad, dam, type_6[x_11 as usize], x_11);
                x_11 += 1;
            }
            let mut x_12: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_12 < 2 as ::core::ffi::c_int {
                if value_5[x_12 as usize] > value_5[biggest_5 as usize] {
                    biggest_5 = x_12;
                }
                x_12 += 1;
            }
            borg_simulate = tmp_simulate_5;
            if !borg_simulate {
                value_5[biggest_5 as usize] = borg_attack_aux_dragon(
                    sv_dragon_chaos,
                    rad,
                    dam,
                    type_6[biggest_5 as usize],
                    biggest_5,
                );
            }
            return value_5[biggest_5 as usize];
        }
        105 => {
            let mut value_6: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_7: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_6: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 220 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_13: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_13 < 2 as ::core::ffi::c_int {
                value_6[x_13 as usize] =
                    borg_attack_aux_dragon(sv_dragon_law, rad, dam, type_7[x_13 as usize], x_13);
                x_13 += 1;
            }
            let mut x_14: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_14 < 2 as ::core::ffi::c_int {
                if value_6[x_14 as usize] > value_6[biggest_6 as usize] {
                    biggest_6 = x_14;
                }
                x_14 += 1;
            }
            borg_simulate = tmp_simulate_6;
            if !borg_simulate {
                value_6[biggest_6 as usize] = borg_attack_aux_dragon(
                    sv_dragon_law,
                    rad,
                    dam,
                    type_7[biggest_6 as usize],
                    biggest_6,
                );
            }
            return value_6[biggest_6 as usize];
        }
        106 => {
            let mut value_7: [::core::ffi::c_int; 4] = [0; 4];
            let mut type_8: [::core::ffi::c_int; 4] = [
                BORG_ATTACK_CHAOS as ::core::ffi::c_int,
                BORG_ATTACK_DISEN as ::core::ffi::c_int,
                BORG_ATTACK_SOUND as ::core::ffi::c_int,
                BORG_ATTACK_SHARD as ::core::ffi::c_int,
            ];
            let mut biggest_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_7: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 250 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_15: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_15 < 4 as ::core::ffi::c_int {
                value_7[x_15 as usize] = borg_attack_aux_dragon(
                    sv_dragon_balance,
                    rad,
                    dam,
                    type_8[x_15 as usize],
                    x_15,
                );
                x_15 += 1;
            }
            let mut x_16: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_16 < 4 as ::core::ffi::c_int {
                if value_7[x_16 as usize] > value_7[biggest_7 as usize] {
                    biggest_7 = x_16;
                }
                x_16 += 1;
            }
            borg_simulate = tmp_simulate_7;
            if !borg_simulate {
                value_7[biggest_7 as usize] = borg_attack_aux_dragon(
                    sv_dragon_balance,
                    rad,
                    dam,
                    type_8[biggest_7 as usize],
                    biggest_7,
                );
            }
            return value_7[biggest_7 as usize];
        }
        107 => {
            let mut value_8: [::core::ffi::c_int; 2] = [0; 2];
            let mut type_9: [::core::ffi::c_int; 2] = [
                BORG_ATTACK_LIGHT as ::core::ffi::c_int,
                BORG_ATTACK_DARK as ::core::ffi::c_int,
            ];
            let mut biggest_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_simulate_8: bool = borg_simulate;
            rad = 20 as ::core::ffi::c_int;
            dam = 200 as ::core::ffi::c_int;
            if !borg_simulate {
                borg_simulate = true_0 != 0;
            }
            let mut x_17: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_17 < 2 as ::core::ffi::c_int {
                value_8[x_17 as usize] = borg_attack_aux_dragon(
                    sv_dragon_shining,
                    rad,
                    dam,
                    type_9[x_17 as usize],
                    x_17,
                );
                x_17 += 1;
            }
            let mut x_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while x_18 < 2 as ::core::ffi::c_int {
                if value_8[x_18 as usize] > value_8[biggest_8 as usize] {
                    biggest_8 = x_18;
                }
                x_18 += 1;
            }
            borg_simulate = tmp_simulate_8;
            if !borg_simulate {
                value_8[biggest_8 as usize] = borg_attack_aux_dragon(
                    sv_dragon_shining,
                    rad,
                    dam,
                    type_9[biggest_8 as usize],
                    biggest_8,
                );
            }
            return value_8[biggest_8 as usize];
        }
        108 => {
            rad = 20 as ::core::ffi::c_int;
            dam = 300 as ::core::ffi::c_int;
            return borg_attack_aux_dragon(
                sv_dragon_power,
                rad,
                dam,
                BORG_ATTACK_MISSILE as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_attack(mut boosted_bravery: bool) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut adjacent_monster: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    let mut current_block_23: u64;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    if !(borg.goal.ignoring as ::core::ffi::c_int != 0
                        && *borg
                            .trait_0
                            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                            == 0
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
                            != 0)
                    {
                        if !(*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_MAGE
                            && *borg
                                .trait_0
                                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                < 10 as ::core::ffi::c_int
                            && *borg
                                .trait_0
                                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                == 0 as ::core::ffi::c_int
                            && !strstr(
                                (*r_ptr).name,
                                b"Farmer\0" as *const u8 as *const ::core::ffi::c_char,
                            )
                            .is_null())
                        {
                            if (*kill).speed as ::core::ffi::c_int
                                > *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                                && distance(borg.c, (*kill).pos) <= 2 as ::core::ffi::c_int
                                || distance(borg.c, (*kill).pos) <= 1 as ::core::ffi::c_int
                            {
                                adjacent_monster = true_0 != 0;
                            }
                            if borg.mon.scary {
                                if !strstr(
                                    (*r_ptr).name,
                                    b"Grip\0" as *const u8 as *const ::core::ffi::c_char,
                                )
                                .is_null()
                                    || !strstr(
                                        (*r_ptr).name,
                                        b"Fang\0" as *const u8 as *const ::core::ffi::c_char,
                                    )
                                    .is_null()
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if *borg
                                    .trait_0
                                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                    <= 5 as ::core::ffi::c_int
                                    && *borg
                                        .trait_0
                                        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                        != 0 as ::core::ffi::c_int
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
                                        != 0
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if borg.time.now - borg.time.level >= 2000 as borg_time
                                    || borg.time.now - borg.time.town >= 3000 as borg_time
                                {
                                    current_block_23 = 4761528863920922185;
                                } else if boosted_bravery as ::core::ffi::c_int != 0
                                    || borg.no_retreat as ::core::ffi::c_int
                                        >= 1 as ::core::ffi::c_int
                                    || borg.goal.recalling != 0
                                    || borg.goal.descending != 0
                                {
                                    borg_note(
                                        b"# Bored, or recalling and fighting a monster on Scaryguy Level.\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else if *borg
                                    .trait_0
                                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                                    * 4 as ::core::ffi::c_int
                                    <= *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    && *borg
                                        .trait_0
                                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                        > 10 as ::core::ffi::c_int
                                {
                                    borg_note(
                                        b"# High clevel fighting monster on Scaryguy Level.\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else if adjacent_monster {
                                    borg_note(
                                        b"# Adjacent to monster on Scaryguy Level.\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block_23 = 4761528863920922185;
                                } else {
                                    current_block_23 = 15427931788582360902;
                                }
                            } else {
                                current_block_23 = 4761528863920922185;
                            }
                            match current_block_23 {
                                15427931788582360902 => {}
                                _ => {
                                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                        .offset((*kill).pos.y as isize))
                                    .offset((*kill).pos.x as isize)
                                        as *mut borg_grid;
                                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                                            if !(distance(borg.c, (*kill).pos)
                                                > (*z_info).max_range as ::core::ffi::c_int)
                                            {
                                                if (*ag).kill == 0 {
                                                    (*borg_grids[(*kill).pos.y as usize]
                                                        .offset((*kill).pos.x as isize))
                                                    .kill = i as uint8_t;
                                                }
                                                borg_temp_x[borg_temp_n as usize] =
                                                    (*kill).pos.x as uint8_t;
                                                borg_temp_y[borg_temp_n as usize] =
                                                    (*kill).pos.y as uint8_t;
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
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    let mut max_attacks: ::core::ffi::c_int = BF_MAX as ::core::ffi::c_int;
    if !(*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize] {
        max_attacks = BF_ACT_STARLIGHT as ::core::ffi::c_int;
    }
    g = 0 as ::core::ffi::c_int;
    while g < max_attacks {
        n = borg_calculate_attack_effectiveness(g);
        if !(n <= b_n) {
            b_g = g;
            b_n = n;
        }
        g += 1;
    }
    if b_n <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing attack type %d with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(b_g);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6;
pub const CLASS_RANGER: ::core::ffi::c_int = 7;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8;
