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
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    static mut player: *mut player;
    static mut Term: *mut term;
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
    fn borg_caution_teleport(emergency: ::core::ffi::c_int, turns: ::core::ffi::c_int) -> bool;
    fn borg_surrounded() -> bool;
    fn borg_heroism_level() -> ::core::ffi::c_int;
    fn borg_spell(spell: borg_spells) -> bool;
    fn borg_spell_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_spell_okay_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    static mut successful_target: ::core::ffi::c_int;
    static mut borg_tp_other_n: int16_t;
    static mut borg_tp_other_x: [uint8_t; 255];
    static mut borg_tp_other_y: [uint8_t; 255];
    static mut borg_tp_other_index: [::core::ffi::c_int; 255];
    fn borg_launch_bolt(
        rad: ::core::ffi::c_int,
        dam: ::core::ffi::c_int,
        typ: ::core::ffi::c_int,
        max: ::core::ffi::c_int,
        ammo_location: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut borg_temp_n: int16_t;
    static mut borg_temp_y: [uint8_t; 9000];
    static mut borg_temp_x: [uint8_t; 9000];
    static mut track_step: borg_track;
    static mut track_glyph: borg_track;
    static mut borg_needs_new_sea: bool;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    static mut r_info: *mut monster_race;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut borg_simulate: bool;
    static mut borg_attacking: bool;
    static mut borg_on_glyph: bool;
    static mut borg_create_door: bool;
    static mut borg_game_ratio: int16_t;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    static mut borg_kills_cnt: int16_t;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    static mut borg_nasties_num: uint8_t;
    static mut borg_nasties_count: [uint8_t; 7];
    static mut borg_nasties: [::core::ffi::c_char; 7];
    static mut borg_nasties_limit: [uint8_t; 7];
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_delete_kill(i: ::core::ffi::c_int);
    fn borg_slot(tval: ::core::ffi::c_int, sval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    static mut act_ring_lightning: ::core::ffi::c_int;
    static mut act_ring_ice: ::core::ffi::c_int;
    static mut act_ring_flames: ::core::ffi::c_int;
    static mut act_ring_acid: ::core::ffi::c_int;
    static mut act_shroom_stone: ::core::ffi::c_int;
    static mut act_berserker: ::core::ffi::c_int;
    static mut act_tele_other: ::core::ffi::c_int;
    static mut act_rage_bless_resist: ::core::ffi::c_int;
    static mut act_haste2: ::core::ffi::c_int;
    static mut act_haste1: ::core::ffi::c_int;
    static mut act_haste: ::core::ffi::c_int;
    static mut act_loskill: ::core::ffi::c_int;
    static mut act_destruction2: ::core::ffi::c_int;
    static mut act_earthquakes: ::core::ffi::c_int;
    static mut act_blessing3: ::core::ffi::c_int;
    static mut act_blessing2: ::core::ffi::c_int;
    static mut act_blessing: ::core::ffi::c_int;
    static mut act_protevil: ::core::ffi::c_int;
    static mut act_banishment: ::core::ffi::c_int;
    static mut act_glyph: ::core::ffi::c_int;
    static mut act_detect_evil: ::core::ffi::c_int;
    static mut act_detect_invis: ::core::ffi::c_int;
    static mut act_resist_all: ::core::ffi::c_int;
    static mut act_resist_pois: ::core::ffi::c_int;
    static mut act_resist_cold: ::core::ffi::c_int;
    static mut act_resist_fire: ::core::ffi::c_int;
    static mut act_resist_elec: ::core::ffi::c_int;
    static mut act_resist_acid: ::core::ffi::c_int;
    static mut act_shero: ::core::ffi::c_int;
    static mut act_tmd_esp: ::core::ffi::c_int;
    static mut act_tmd_sinvis: ::core::ffi::c_int;
    fn borg_quaff_potion(sval: ::core::ffi::c_int) -> bool;
    fn borg_read_scroll(sval: ::core::ffi::c_int) -> bool;
    fn borg_eat(tval: ::core::ffi::c_int, sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_rod(sval: ::core::ffi::c_int) -> bool;
    fn borg_zap_rod(sval: ::core::ffi::c_int) -> bool;
    fn borg_use_staff(sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_staff_fail(sval: ::core::ffi::c_int) -> bool;
    fn borg_aim_wand(sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    static mut sv_mush_stoneskin: ::core::ffi::c_int;
    static mut kv_mush_stoneskin: ::core::ffi::c_int;
    static mut sv_potion_resist_heat: ::core::ffi::c_int;
    static mut sv_potion_resist_cold: ::core::ffi::c_int;
    static mut sv_potion_resist_pois: ::core::ffi::c_int;
    static mut sv_potion_speed: ::core::ffi::c_int;
    static mut sv_potion_berserk: ::core::ffi::c_int;
    static mut sv_potion_heroism: ::core::ffi::c_int;
    static mut sv_potion_detect_invis: ::core::ffi::c_int;
    static mut sv_scroll_protection_from_evil: ::core::ffi::c_int;
    static mut sv_scroll_rune_of_protection: ::core::ffi::c_int;
    static mut sv_scroll_banishment: ::core::ffi::c_int;
    static mut sv_scroll_mass_banishment: ::core::ffi::c_int;
    static mut sv_scroll_blessing: ::core::ffi::c_int;
    static mut sv_scroll_holy_chant: ::core::ffi::c_int;
    static mut sv_scroll_holy_prayer: ::core::ffi::c_int;
    static mut sv_scroll_detect_invis: ::core::ffi::c_int;
    static mut sv_ring_flames: ::core::ffi::c_int;
    static mut sv_ring_ice: ::core::ffi::c_int;
    static mut sv_ring_acid: ::core::ffi::c_int;
    static mut sv_ring_lightning: ::core::ffi::c_int;
    static mut sv_rod_speed: ::core::ffi::c_int;
    static mut sv_staff_destruction: ::core::ffi::c_int;
    static mut sv_staff_speed: ::core::ffi::c_int;
    static mut sv_staff_detect_invis: ::core::ffi::c_int;
    static mut sv_staff_detect_evil: ::core::ffi::c_int;
    static mut sv_staff_banishment: ::core::ffi::c_int;
    static mut sv_wand_teleport_away: ::core::ffi::c_int;
    fn borg_light_beam(simulation: bool) -> bool;
    static mut borg_target_loc: loc;
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
    fn borg_projectable_pure(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_projectable_dark(
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
pub type keycode_t = uint32_t;
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
pub type borg_time = int32_t;
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
pub const BI_ISFORGET: C2RustUnnamed_12 = 121;
pub const BI_ISCONFUSED: C2RustUnnamed_12 = 114;
pub const BI_ISBLIND: C2RustUnnamed_12 = 112;
pub const BD_LIGHT_MORGOTH: C2RustUnnamed_14 = 30;
pub const RF_UNIQUE: C2RustUnnamed_11 = 1;
pub const RF_MAX: C2RustUnnamed_11 = 85;
pub const RF_EVIL: C2RustUnnamed_11 = 61;
pub const BD_BANISHMENT_MORGOTH: C2RustUnnamed_14 = 29;
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
pub const BORG_ATTACK_AWAY_ALL_MORGOTH: C2RustUnnamed_9 = 46;
pub const BI_AGLYPH: C2RustUnnamed_12 = 230;
pub const BI_CDEPTH: C2RustUnnamed_12 = 105;
pub const BD_TELE_AWAY_MORGOTH: C2RustUnnamed_14 = 28;
pub const BI_CURHP: C2RustUnnamed_12 = 27;
pub const BD_REST: C2RustUnnamed_14 = 27;
pub const BD_LIGHT_BEAM: C2RustUnnamed_14 = 26;
pub const BI_LIGHT: C2RustUnnamed_12 = 26;
pub const BD_DETECT_INVISO: C2RustUnnamed_14 = 25;
pub const BD_BANISHMENT: C2RustUnnamed_14 = 24;
pub const BI_AESCAPE: C2RustUnnamed_12 = 212;
pub const BI_ATELEPORTLVL: C2RustUnnamed_12 = 247;
pub const BI_ATELEPORT: C2RustUnnamed_12 = 211;
pub const BD_DESTRUCTION: C2RustUnnamed_14 = 22;
pub const BD_TPORTLEVEL: C2RustUnnamed_14 = 23;
pub const BD_EARTHQUAKE: C2RustUnnamed_14 = 21;
pub const BI_MAXHP: C2RustUnnamed_12 = 28;
pub const BD_GENOCIDE_NASTIES: C2RustUnnamed_14 = 20;
pub const BD_GENOCIDE: C2RustUnnamed_14 = 19;
pub const BI_AMASSBAN: C2RustUnnamed_12 = 249;
pub const BD_MASS_GENOCIDE: C2RustUnnamed_14 = 18;
pub const FEAT_BROKEN: C2RustUnnamed_10 = 4;
pub const FEAT_OPEN: C2RustUnnamed_10 = 3;
pub const FEAT_MORE: C2RustUnnamed_10 = 6;
pub const FEAT_LESS: C2RustUnnamed_10 = 5;
pub const FEAT_CLOSED: C2RustUnnamed_10 = 2;
pub const FEAT_PERM: C2RustUnnamed_10 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_10 = 21;
pub const BD_CREATE_DOOR: C2RustUnnamed_14 = 17;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
pub const BI_ISIMAGE: C2RustUnnamed_12 = 120;
pub const BD_GLYPH: C2RustUnnamed_14 = 16;
pub const BORG_ATTACK_AWAY_ALL: C2RustUnnamed_9 = 45;
pub const FEAT_RUBBLE: C2RustUnnamed_10 = 16;
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
pub const BI_CURSP: C2RustUnnamed_12 = 30;
pub const BD_TELE_AWAY: C2RustUnnamed_14 = 11;
pub const BD_SHIELD: C2RustUnnamed_14 = 10;
pub const BD_REGEN: C2RustUnnamed_14 = 15;
pub const BI_WS_EVIL: C2RustUnnamed_12 = 194;
pub const BD_SMITE_EVIL: C2RustUnnamed_14 = 14;
pub const BD_BERSERK: C2RustUnnamed_14 = 13;
pub const BI_CLEVEL: C2RustUnnamed_12 = 35;
pub const BD_HERO: C2RustUnnamed_14 = 12;
pub const BD_BLESS: C2RustUnnamed_14 = 0;
pub const BD_RESIST_P: C2RustUnnamed_14 = 8;
pub const BD_RESIST_E: C2RustUnnamed_14 = 7;
pub const BD_RESIST_A: C2RustUnnamed_14 = 6;
pub const BD_RESIST_C: C2RustUnnamed_14 = 5;
pub const BD_RESIST_F: C2RustUnnamed_14 = 4;
pub const BD_RESIST_FECAP: C2RustUnnamed_14 = 3;
pub const BI_FRACT: C2RustUnnamed_12 = 86;
pub const BI_RCONF: C2RustUnnamed_12 = 78;
pub const BD_GRIM_PURPOSE: C2RustUnnamed_14 = 2;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_13 = 24;
pub const BD_PROT_FROM_EVIL: C2RustUnnamed_14 = 9;
pub const BD_SPEED: C2RustUnnamed_14 = 1;
pub const BD_MAX: C2RustUnnamed_14 = 31;
pub const TMD_OPP_ACID: C2RustUnnamed_8 = 19;
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
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const TMD_MAX: C2RustUnnamed_8 = 53;
pub const TMD_FREE_ACT: C2RustUnnamed_8 = 52;
pub const TMD_STEALTH: C2RustUnnamed_8 = 51;
pub const TMD_BLACKBREATH: C2RustUnnamed_8 = 50;
pub const TMD_BLOODLUST: C2RustUnnamed_8 = 49;
pub const TMD_TAUNT: C2RustUnnamed_8 = 48;
pub const TMD_POWERSHOT: C2RustUnnamed_8 = 47;
pub const TMD_COVERTRACKS: C2RustUnnamed_8 = 46;
pub const TMD_ATT_RUN: C2RustUnnamed_8 = 45;
pub const TMD_COMMAND: C2RustUnnamed_8 = 44;
pub const TMD_HEAL: C2RustUnnamed_8 = 43;
pub const TMD_ATT_VAMP: C2RustUnnamed_8 = 42;
pub const TMD_ATT_DEMON: C2RustUnnamed_8 = 41;
pub const TMD_ATT_EVIL: C2RustUnnamed_8 = 40;
pub const TMD_ATT_CONF: C2RustUnnamed_8 = 39;
pub const TMD_ATT_POIS: C2RustUnnamed_8 = 38;
pub const TMD_ATT_COLD: C2RustUnnamed_8 = 37;
pub const TMD_ATT_FIRE: C2RustUnnamed_8 = 36;
pub const TMD_ATT_ELEC: C2RustUnnamed_8 = 35;
pub const TMD_ATT_ACID: C2RustUnnamed_8 = 34;
pub const TMD_FASTCAST: C2RustUnnamed_8 = 33;
pub const TMD_TRAPSAFE: C2RustUnnamed_8 = 32;
pub const TMD_SCRAMBLE: C2RustUnnamed_8 = 31;
pub const TMD_BOLD: C2RustUnnamed_8 = 30;
pub const TMD_SPRINT: C2RustUnnamed_8 = 29;
pub const TMD_TERROR: C2RustUnnamed_8 = 28;
pub const TMD_STONESKIN: C2RustUnnamed_8 = 27;
pub const TMD_TELEPATHY: C2RustUnnamed_8 = 26;
pub const TMD_AMNESIA: C2RustUnnamed_8 = 25;
pub const TMD_OPP_CONF: C2RustUnnamed_8 = 24;
pub const TMD_OPP_POIS: C2RustUnnamed_8 = 23;
pub const TMD_OPP_COLD: C2RustUnnamed_8 = 22;
pub const TMD_OPP_FIRE: C2RustUnnamed_8 = 21;
pub const TMD_OPP_ELEC: C2RustUnnamed_8 = 20;
pub const TMD_SINFRA: C2RustUnnamed_8 = 18;
pub const TMD_SINVIS: C2RustUnnamed_8 = 17;
pub const TMD_BLESSED: C2RustUnnamed_8 = 16;
pub const TMD_SHIELD: C2RustUnnamed_8 = 15;
pub const TMD_SHERO: C2RustUnnamed_8 = 14;
pub const TMD_HERO: C2RustUnnamed_8 = 13;
pub const TMD_INVULN: C2RustUnnamed_8 = 12;
pub const TMD_PROTEVIL: C2RustUnnamed_8 = 11;
pub const TMD_FOOD: C2RustUnnamed_8 = 10;
pub const TMD_STUN: C2RustUnnamed_8 = 9;
pub const TMD_CUT: C2RustUnnamed_8 = 8;
pub const TMD_POISONED: C2RustUnnamed_8 = 7;
pub const TMD_IMAGE: C2RustUnnamed_8 = 6;
pub const TMD_AFRAID: C2RustUnnamed_8 = 5;
pub const TMD_CONFUSED: C2RustUnnamed_8 = 4;
pub const TMD_PARALYZED: C2RustUnnamed_8 = 3;
pub const TMD_BLIND: C2RustUnnamed_8 = 2;
pub const TMD_SLOW: C2RustUnnamed_8 = 1;
pub const TMD_FAST: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const BORG_ATTACK_CURSE: C2RustUnnamed_9 = 58;
pub const BORG_ATTACK_ELEC_STRIKE: C2RustUnnamed_9 = 57;
pub const BORG_ATTACK_DRAIN_LIFE: C2RustUnnamed_9 = 56;
pub const BORG_ATTACK_TAP_UNLIFE: C2RustUnnamed_9 = 55;
pub const BORG_ATTACK_AWAY_EVIL: C2RustUnnamed_9 = 54;
pub const BORG_ATTACK_TURN_UNDEAD: C2RustUnnamed_9 = 53;
pub const BORG_ATTACK_OLD_POLY: C2RustUnnamed_9 = 52;
pub const BORG_ATTACK_OLD_SLEEP: C2RustUnnamed_9 = 51;
pub const BORG_ATTACK_OLD_SLOW: C2RustUnnamed_9 = 50;
pub const BORG_ATTACK_TURN_ALL: C2RustUnnamed_9 = 49;
pub const BORG_ATTACK_OLD_CONF: C2RustUnnamed_9 = 48;
pub const BORG_ATTACK_DISP_ALL: C2RustUnnamed_9 = 47;
pub const BORG_ATTACK_TURN_EVIL: C2RustUnnamed_9 = 44;
pub const BORG_ATTACK_AWAY_UNDEAD: C2RustUnnamed_9 = 43;
pub const BORG_ATTACK_MAKE_TRAP: C2RustUnnamed_9 = 42;
pub const BORG_ATTACK_MAKE_DOOR: C2RustUnnamed_9 = 41;
pub const BORG_ATTACK_MAKE_WALL: C2RustUnnamed_9 = 40;
pub const BORG_ATTACK_KILL_TRAP: C2RustUnnamed_9 = 39;
pub const BORG_ATTACK_KILL_DOOR: C2RustUnnamed_9 = 38;
pub const BORG_ATTACK_DARK_WEAK: C2RustUnnamed_9 = 37;
pub const BORG_ATTACK_OLD_SPEED: C2RustUnnamed_9 = 36;
pub const BORG_ATTACK_OLD_CLONE: C2RustUnnamed_9 = 35;
pub const BORG_ATTACK_OLD_HEAL: C2RustUnnamed_9 = 34;
pub const BORG_ATTACK_WATER: C2RustUnnamed_9 = 33;
pub const BORG_ATTACK_DARK: C2RustUnnamed_9 = 32;
pub const BORG_ATTACK_LIGHT: C2RustUnnamed_9 = 31;
pub const BORG_ATTACK_TIME: C2RustUnnamed_9 = 30;
pub const BORG_ATTACK_INERTIA: C2RustUnnamed_9 = 29;
pub const BORG_ATTACK_FORCE: C2RustUnnamed_9 = 28;
pub const BORG_ATTACK_NEXUS: C2RustUnnamed_9 = 27;
pub const BORG_ATTACK_DISEN: C2RustUnnamed_9 = 26;
pub const BORG_ATTACK_CONFU: C2RustUnnamed_9 = 25;
pub const BORG_ATTACK_PLASMA: C2RustUnnamed_9 = 24;
pub const BORG_ATTACK_SOUND: C2RustUnnamed_9 = 23;
pub const BORG_ATTACK_SHARD: C2RustUnnamed_9 = 22;
pub const BORG_ATTACK_GRAVITY: C2RustUnnamed_9 = 21;
pub const BORG_ATTACK_CHAOS: C2RustUnnamed_9 = 20;
pub const BORG_ATTACK_NETHER: C2RustUnnamed_9 = 19;
pub const BORG_ATTACK_KILL_WALL: C2RustUnnamed_9 = 18;
pub const BORG_ATTACK_OLD_DRAIN: C2RustUnnamed_9 = 17;
pub const BORG_ATTACK_LIGHT_WEAK: C2RustUnnamed_9 = 16;
pub const BORG_ATTACK_HOLY_WORD: C2RustUnnamed_9 = 15;
pub const BORG_ATTACK_SLEEP_EVIL: C2RustUnnamed_9 = 14;
pub const BORG_ATTACK_DISP_SPIRITS: C2RustUnnamed_9 = 13;
pub const BORG_ATTACK_DISP_EVIL: C2RustUnnamed_9 = 12;
pub const BORG_ATTACK_DISP_UNDEAD: C2RustUnnamed_9 = 11;
pub const BORG_ATTACK_HOLY_ORB: C2RustUnnamed_9 = 10;
pub const BORG_ATTACK_ICE: C2RustUnnamed_9 = 9;
pub const BORG_ATTACK_POIS: C2RustUnnamed_9 = 8;
pub const BORG_ATTACK_COLD: C2RustUnnamed_9 = 7;
pub const BORG_ATTACK_FIRE: C2RustUnnamed_9 = 6;
pub const BORG_ATTACK_ELEC: C2RustUnnamed_9 = 5;
pub const BORG_ATTACK_ACID: C2RustUnnamed_9 = 4;
pub const BORG_ATTACK_METEOR: C2RustUnnamed_9 = 3;
pub const BORG_ATTACK_MANA: C2RustUnnamed_9 = 2;
pub const BORG_ATTACK_ARROW: C2RustUnnamed_9 = 1;
pub const BORG_ATTACK_MISSILE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_10 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_10 = 24;
pub const FEAT_LAVA: C2RustUnnamed_10 = 23;
pub const FEAT_QUARTZ_K: C2RustUnnamed_10 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_10 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_10 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_10 = 17;
pub const FEAT_SECRET: C2RustUnnamed_10 = 15;
pub const FEAT_HOME: C2RustUnnamed_10 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_10 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_10 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_10 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_10 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_10 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_10 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_10 = 7;
pub const FEAT_FLOOR: C2RustUnnamed_10 = 1;
pub const FEAT_NONE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const RF_NO_SLOW: C2RustUnnamed_11 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_11 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_11 = 82;
pub const RF_NO_CONF: C2RustUnnamed_11 = 81;
pub const RF_NO_STUN: C2RustUnnamed_11 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_11 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_11 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_11 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_11 = 76;
pub const RF_IM_WATER: C2RustUnnamed_11 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_11 = 74;
pub const RF_IM_POIS: C2RustUnnamed_11 = 73;
pub const RF_IM_COLD: C2RustUnnamed_11 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_11 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_11 = 70;
pub const RF_IM_ACID: C2RustUnnamed_11 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_11 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_11 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_11 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_11 = 65;
pub const RF_METAL: C2RustUnnamed_11 = 64;
pub const RF_NONLIVING: C2RustUnnamed_11 = 63;
pub const RF_UNDEAD: C2RustUnnamed_11 = 62;
pub const RF_ANIMAL: C2RustUnnamed_11 = 60;
pub const RF_DEMON: C2RustUnnamed_11 = 59;
pub const RF_DRAGON: C2RustUnnamed_11 = 58;
pub const RF_GIANT: C2RustUnnamed_11 = 57;
pub const RF_TROLL: C2RustUnnamed_11 = 56;
pub const RF_ORC: C2RustUnnamed_11 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_11 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_11 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_11 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_11 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_11 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_11 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_11 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_11 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_11 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_11 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_11 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_11 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_11 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_11 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_11 = 40;
pub const RF_DROP_20: C2RustUnnamed_11 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_11 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_11 = 37;
pub const RF_DROP_4: C2RustUnnamed_11 = 36;
pub const RF_DROP_3: C2RustUnnamed_11 = 35;
pub const RF_DROP_2: C2RustUnnamed_11 = 34;
pub const RF_DROP_1: C2RustUnnamed_11 = 33;
pub const RF_DROP_60: C2RustUnnamed_11 = 32;
pub const RF_DROP_40: C2RustUnnamed_11 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_11 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_11 = 29;
pub const RF_POWERFUL: C2RustUnnamed_11 = 28;
pub const RF_SPIRIT: C2RustUnnamed_11 = 27;
pub const RF_SMART: C2RustUnnamed_11 = 26;
pub const RF_STUPID: C2RustUnnamed_11 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_11 = 24;
pub const RF_RAND_50: C2RustUnnamed_11 = 23;
pub const RF_RAND_25: C2RustUnnamed_11 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_11 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_11 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_11 = 19;
pub const RF_REGENERATE: C2RustUnnamed_11 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_11 = 17;
pub const RF_UNAWARE: C2RustUnnamed_11 = 16;
pub const RF_SEASONAL: C2RustUnnamed_11 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_11 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_11 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_11 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_11 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_11 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_11 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_11 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_11 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_11 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_11 = 5;
pub const RF_FEMALE: C2RustUnnamed_11 = 4;
pub const RF_MALE: C2RustUnnamed_11 = 3;
pub const RF_QUESTOR: C2RustUnnamed_11 = 2;
pub const RF_NONE: C2RustUnnamed_11 = 0;
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
pub const BI_AHWORD: C2RustUnnamed_12 = 248;
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
pub const BI_ISPARALYZED: C2RustUnnamed_12 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_12 = 118;
pub const BI_ISSTUN: C2RustUnnamed_12 = 117;
pub const BI_ISCUT: C2RustUnnamed_12 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_12 = 115;
pub const BI_ISAFRAID: C2RustUnnamed_12 = 113;
pub const BI_ISGORGED: C2RustUnnamed_12 = 111;
pub const BI_ISFULL: C2RustUnnamed_12 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_12 = 109;
pub const BI_ISWEAK: C2RustUnnamed_12 = 108;
pub const BI_KING: C2RustUnnamed_12 = 107;
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
pub const BI_HLIFE: C2RustUnnamed_12 = 85;
pub const BI_RDIS: C2RustUnnamed_12 = 84;
pub const BI_RKAOS: C2RustUnnamed_12 = 83;
pub const BI_RNTHR: C2RustUnnamed_12 = 82;
pub const BI_RNXUS: C2RustUnnamed_12 = 81;
pub const BI_RSHRD: C2RustUnnamed_12 = 80;
pub const BI_RSND: C2RustUnnamed_12 = 79;
pub const BI_RBLIND: C2RustUnnamed_12 = 77;
pub const BI_RDARK: C2RustUnnamed_12 = 76;
pub const BI_RLITE: C2RustUnnamed_12 = 75;
pub const BI_RFEAR: C2RustUnnamed_12 = 74;
pub const BI_RPOIS: C2RustUnnamed_12 = 73;
pub const BI_RACID: C2RustUnnamed_12 = 72;
pub const BI_RELEC: C2RustUnnamed_12 = 71;
pub const BI_RCOLD: C2RustUnnamed_12 = 70;
pub const BI_RFIRE: C2RustUnnamed_12 = 69;
pub const BI_IPOIS: C2RustUnnamed_12 = 68;
pub const BI_IELEC: C2RustUnnamed_12 = 67;
pub const BI_ICOLD: C2RustUnnamed_12 = 66;
pub const BI_IACID: C2RustUnnamed_12 = 65;
pub const BI_IFIRE: C2RustUnnamed_12 = 64;
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
pub const BI_MAXSP: C2RustUnnamed_12 = 31;
pub const BI_HP_ADJ: C2RustUnnamed_12 = 29;
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
pub const BORG_AUTOSAVE: C2RustUnnamed_13 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_13 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_13 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_13 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_13 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_13 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_13 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_13 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_13 = 25;
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
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_attempting_refresh_resist: bool = false_0 != 0;
unsafe extern "C" fn borg_log_spellpath() {
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    y = borg_target_loc.y;
    x = borg_target_loc.x;
    n_x = borg.c.x;
    n_y = borg.c.y;
    loop {
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(n_y as isize))
            .offset(n_x as isize) as *mut borg_grid;
        kill = borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
        if !borg_cave_floor_grid(ag) {
            borg_note(format(
                b"# Logging Spell pathway (%d,%d): Wall grid.\0" as *const u8
                    as *const ::core::ffi::c_char,
                n_y,
                n_x,
            ));
            break;
        } else {
            if (*ag).kill != 0 {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d): %s, danger %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                    borg_race_name((*kill).r_idx as ::core::ffi::c_int),
                    borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        (*ag).kill as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ),
                ));
            } else if n_y == borg.c.y && n_x == borg.c.x {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d): My grid.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                ));
            } else {
                borg_note(format(
                    b"# Logging Spell pathway (%d,%d).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    n_y,
                    n_x,
                ));
            }
            if n_x == x && n_y == y {
                break;
            }
            dist += 1;
            if dist >= (*z_info).max_range as ::core::ffi::c_int {
                break;
            }
            borg_inc_motion(&raw mut n_y, &raw mut n_x, borg.c.y, borg.c.x, y, x);
        }
    }
}
unsafe extern "C" fn borg_defend_aux_bless(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut i: ::core::ffi::c_int = 0;
    let mut borg_near_kill: bool = false_0 != 0;
    if borg.temp.bless {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BLESS, fail_allowed)
        && !borg_equips_item(act_blessing, true_0 != 0)
        && !borg_equips_item(act_blessing2, true_0 != 0)
        && !borg_equips_item(act_blessing3, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_blessing)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_holy_chant)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_holy_prayer)
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 5 as borg_time) {
                if !(distance(borg.c, (*kill).pos) > 3 as ::core::ffi::c_int) {
                    borg_near_kill = true_0 != 0;
                }
            }
        }
        i += 1;
    }
    if (p1 > borg.avoidance as ::core::ffi::c_int / 12 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            <= 15 as ::core::ffi::c_int)
        && p1 > 0 as ::core::ffi::c_int
        && borg_near_kill as ::core::ffi::c_int != 0
        && p1 < borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast Bless\0" as *const u8 as *const ::core::ffi::c_char);
        borg.no_rest_prep = 11000 as int16_t;
        if borg_spell(BLESS) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing2) as ::core::ffi::c_int != 0
            || borg_activate_item(act_blessing3) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_blessing) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_holy_chant) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_holy_prayer) as ::core::ffi::c_int != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_speed(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut good_speed: bool = false_0 != 0;
    let mut speed_spell: bool = false_0 != 0;
    let mut speed_staff: bool = false_0 != 0;
    let mut speed_rod: bool = false_0 != 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    if borg.temp.fast {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(HASTE_SELF, fail_allowed) {
        speed_spell = true_0 != 0;
    }
    if borg_equips_staff_fail(sv_staff_speed) {
        speed_staff = true_0 != 0;
    }
    if borg_equips_rod(sv_rod_speed) {
        speed_rod = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int > borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_speed)
        && !speed_staff
        && !speed_rod
        && !speed_spell
        && !borg_equips_item(act_haste, true_0 != 0)
        && !borg_equips_item(act_haste1, true_0 != 0)
        && !borg_equips_item(act_haste2, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if speed_rod as ::core::ffi::c_int != 0
        || speed_spell as ::core::ffi::c_int != 0
        || speed_staff as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste1, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste2, true_0 != 0) as ::core::ffi::c_int != 0
    {
        good_speed = true_0 != 0;
    }
    borg.temp.fast = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg.temp.fast = false_0 != 0;
    if borg.mon.scary {
        p2 = p2 * 3 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if good_speed as ::core::ffi::c_int != 0 && borg.near.unique as ::core::ffi::c_int != 0 {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if borg.near.summoner as ::core::ffi::c_int != 0 && borg.near.unique as ::core::ffi::c_int != 0
    {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 99 as ::core::ffi::c_int
        && borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int != 0
        && (strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"Bullroarer the Hobbit\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Mughash the Kobold Lord\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Wormtongue, Agent of Saruman\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Lagduf, the Snaga\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Brodda, the Easterling\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                (*r_info.offset(borg.mon.unique as isize)).name,
                b"Orfax, Son of Boldor\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0)
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
    {
        p2 = p2 * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 97 as ::core::ffi::c_int
        && borg.near.unique == 0
        && !good_speed
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int
        && good_speed as ::core::ffi::c_int != 0
        || p1 > p2
            && p2
                <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                    borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 3 as ::core::ffi::c_int
                } else {
                    borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                })
            && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast Speed\0" as *const u8 as *const ::core::ffi::c_char);
        borg.no_rest_prep = (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 1000 as ::core::ffi::c_int) as int16_t;
        if borg_zap_rod(sv_rod_speed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste1) as ::core::ffi::c_int != 0
            || borg_activate_item(act_haste2) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_speed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_speed) as ::core::ffi::c_int != 0
        {
            return p1 - p2;
        }
        if borg_spell_fail(HASTE_SELF, fail_allowed) {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_grim_purpose(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_conf: bool = *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0;
    let mut save_fa: bool = *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0;
    if save_conf as ::core::ffi::c_int != 0 && save_fa as ::core::ffi::c_int != 0 {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(GRIM_PURPOSE, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = true_0;
    *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = true_0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = save_conf as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = save_fa as ::core::ffi::c_int;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2 + 2 as ::core::ffi::c_int;
        }
        borg_note(
            b"# Attempting to cast Grim Purpose\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if borg_spell(GRIM_PURPOSE) {
            borg.no_rest_prep = 13000 as int16_t;
        }
        return p1 - p2 + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_fecap(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save_fire: bool = false_0 != 0;
    let mut save_acid: bool = false_0 != 0;
    let mut save_poison: bool = false_0 != 0;
    let mut save_elec: bool = false_0 != 0;
    let mut save_cold: bool = false_0 != 0;
    if borg.temp.res_fire as ::core::ffi::c_int != 0
        && borg.temp.res_acid as ::core::ffi::c_int != 0
        && borg.temp.res_pois as ::core::ffi::c_int != 0
        && borg.temp.res_elec as ::core::ffi::c_int != 0
        && borg.temp.res_cold as ::core::ffi::c_int != 0
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_fire = borg.temp.res_fire;
    save_elec = borg.temp.res_elec;
    save_cold = borg.temp.res_cold;
    save_acid = borg.temp.res_acid;
    save_poison = borg.temp.res_pois;
    borg.temp.res_fire = true_0 != 0;
    borg.temp.res_elec = true_0 != 0;
    borg.temp.res_cold = true_0 != 0;
    borg.temp.res_acid = true_0 != 0;
    borg.temp.res_pois = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = save_fire;
    borg.temp.res_elec = save_elec;
    borg.temp.res_cold = save_cold;
    borg.temp.res_acid = save_acid;
    borg.temp.res_pois = save_poison;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 45 as ::core::ffi::c_int
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2 + 2 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast FECAP\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2 + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_f(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_fire: bool = false_0 != 0;
    save_fire = borg.temp.res_fire;
    if borg.temp.res_fire {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_resist_fire, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_flames)
        && !borg_equips_item(act_ring_flames, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_heat)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_fire = save_fire;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RFire\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_ring(sv_ring_flames) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_flames) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_fire) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_spell_fail(RESISTANCE, fail_allowed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_heat) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_c(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_cold: bool = false_0 != 0;
    if borg.temp.res_cold {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_item(act_resist_cold, true_0 != 0)
        && !borg_equips_ring(sv_ring_ice)
        && !borg_equips_item(act_ring_ice, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_cold)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_cold = borg.temp.res_cold;
    borg.temp.res_cold = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_cold = save_cold;
    if borg.near.unique as ::core::ffi::c_int != 0
        && strcmp(
            (*r_info.offset(borg.mon.unique as isize)).name,
            b"The Tarrasque\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        p2 = p2 * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RCold\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_activate_ring(sv_ring_ice) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_ice) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_cold) as ::core::ffi::c_int != 0
            || borg_spell_fail(RESISTANCE, fail_allowed) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_cold) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_a(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_acid: bool = false_0 != 0;
    if borg.temp.res_acid {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_acid, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_acid)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_acid = borg.temp.res_acid;
    borg.temp.res_acid = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_acid = save_acid;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RAcid\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell(RESISTANCE) {
            return p1 - p2;
        }
        if borg_activate_ring(sv_ring_acid) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_acid) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_acid) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_e(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_elec: bool = false_0 != 0;
    if borg.temp.res_elec {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESISTANCE, fail_allowed)
        && !borg_equips_item(act_resist_elec, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_ring(sv_ring_lightning)
        && !borg_equips_item(act_ring_lightning, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_elec = borg.temp.res_elec;
    borg.temp.res_elec = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_elec = save_elec;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RAcid\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell(RESISTANCE) {
            return p1 - p2;
        }
        if borg_activate_ring(sv_ring_lightning) as ::core::ffi::c_int != 0
            || borg_activate_item(act_ring_lightning) as ::core::ffi::c_int != 0
        {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
            return p1 - p2;
        }
        if borg_activate_item(act_resist_elec) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_resist_p(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut save_poison: bool = false_0 != 0;
    if borg.temp.res_pois {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RESIST_POISON, fail_allowed)
        && !borg_equips_item(act_resist_pois, true_0 != 0)
        && !borg_equips_item(act_resist_all, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_resist_pois)
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    save_poison = borg.temp.res_pois;
    borg.temp.res_pois = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.res_pois = save_poison;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast RPois\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell_fail(RESIST_POISON, fail_allowed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_pois) as ::core::ffi::c_int != 0
            || borg_activate_item(act_resist_all) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_resist_pois) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 21000 as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_prot_evil(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut pfe_spell: bool = false_0 != 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    if borg.temp.prot_from_evil {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(PROTECTION_FROM_EVIL, fail_allowed) {
        pfe_spell = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int
        <= borg_slot(
            TV_SCROLL as ::core::ffi::c_int,
            sv_scroll_protection_from_evil,
        )
    {
        pfe_spell = true_0 != 0;
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
        pfe_spell = false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        pfe_spell = false_0 != 0;
    }
    if borg_equips_item(act_protevil, true_0 != 0) {
        pfe_spell = true_0 != 0;
    }
    if pfe_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    p1 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.prot_from_evil = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
    );
    borg.temp.prot_from_evil = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
        || *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to cast PFE\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_spell_fail(PROTECTION_FROM_EVIL, fail_allowed) as ::core::ffi::c_int != 0
            || borg_activate_item(act_protevil) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_protection_from_evil) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                * 1000 as ::core::ffi::c_int) as int16_t;
        }
        return p1 - p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_shield(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if borg.temp.shield {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.has.offset(kv_mush_stoneskin as isize) <= 0 as ::core::ffi::c_int
        && !borg_equips_item(act_shroom_stone, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    borg.temp.shield = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg.temp.shield = false_0 != 0;
    if borg.near.unique != 0 {
        p2 = p2 * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        borg_note(b"# Attempting to eat a stone skin\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_stoneskin) as ::core::ffi::c_int != 0
            || borg_activate_item(act_shroom_stone) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 2000 as int16_t;
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_tele_away(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = p1;
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut spell_ok: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.unique != 0 {
        if p1
            < borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                > 30 as ::core::ffi::c_int
            && borg_simulate as ::core::ffi::c_int != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if p1
        < borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    spell_ok = false_0 != 0;
    if p1 > borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
    {
        fail_allowed += 5 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(TELEPORT_OTHER, fail_allowed) as ::core::ffi::c_int != 0
        || borg_equips_item(act_tele_other, true_0 != 0) as ::core::ffi::c_int != 0
        || -(1 as ::core::ffi::c_int)
            != borg_slot(TV_WAND as ::core::ffi::c_int, sv_wand_teleport_away)
            && (*borg_items
                .offset(borg_slot(TV_WAND as ::core::ffi::c_int, sv_wand_teleport_away) as isize))
            .pval as ::core::ffi::c_int
                != 0
    {
        spell_ok = true_0 != 0;
    }
    if !spell_ok {
        return 0 as ::core::ffi::c_int;
    }
    if borg_surrounded() as ::core::ffi::c_int == true_0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                    if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                        if !((*ag).feat as ::core::ffi::c_int >= FEAT_RUBBLE as ::core::ffi::c_int
                            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int)
                        {
                            if !(distance(borg.c, (*kill).pos)
                                > (*z_info).max_range as ::core::ffi::c_int)
                            {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        p1,
        BORG_ATTACK_AWAY_ALL as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if p2 <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        borg_temp_n = 0 as int16_t;
        borg_tp_other_n = 0 as int16_t;
        if p2 != 0 && p2 > borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
            return p2;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    }
    borg_log_spellpath();
    i = 0 as ::core::ffi::c_int;
    while i < borg_tp_other_n as ::core::ffi::c_int {
        borg_note(format(
            b"# T.O. %d, index %d (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_tp_other_n as ::core::ffi::c_int,
            borg_tp_other_index[i as usize],
            borg_tp_other_y[i as usize] as ::core::ffi::c_int,
            borg_tp_other_x[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    if borg_spell(TELEPORT_OTHER) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tele_other) as ::core::ffi::c_int != 0
        || borg_aim_wand(sv_wand_teleport_away) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
        return p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_hero(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut potion: bool = false;
    let mut spell: bool = false;
    if borg.temp.hero {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    spell = borg_spell_okay_fail(HEROISM, fail_allowed) as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= borg_heroism_level();
    potion =
        -(1 as ::core::ffi::c_int) != borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_heroism);
    if !potion && !spell {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        borg_note(b"# Attempting to cast Hero\0" as *const u8 as *const ::core::ffi::c_char);
        if spell as ::core::ffi::c_int != 0 && borg_spell(HEROISM) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_heroism) as ::core::ffi::c_int != 0
        {
            borg.no_rest_prep = 10000 as int16_t;
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_regen(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.regen {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) < 100 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(RAPID_REGENERATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 1 as ::core::ffi::c_int;
        }
        if borg_spell(RAPID_REGENERATION) {
            borg.no_rest_prep = 10000 as int16_t;
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_berserk(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.berserk {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(BERSERK_STRENGTH, fail_allowed)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_berserk)
        && !borg_equips_item(act_berserker, true_0 != 0)
        && !borg_equips_item(act_rage_bless_resist, true_0 != 0)
        && !borg_equips_item(act_shero, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 5 as ::core::ffi::c_int;
        }
        if borg_spell(BERSERK_STRENGTH) as ::core::ffi::c_int != 0
            || borg_activate_item(act_berserker) as ::core::ffi::c_int != 0
            || borg_activate_item(act_rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_activate_item(act_shero) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_berserk) as ::core::ffi::c_int != 0
        {
            return 5 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn near_evil() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                        .offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !(distance(borg.c, (*kill).pos) > 3 as ::core::ffi::c_int) {
                            if flag_has(
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
                                RF_EVIL as ::core::ffi::c_int,
                            ) {
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
unsafe extern "C" fn borg_defend_aux_smite_evil(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if borg.temp.smite_evil as ::core::ffi::c_int != 0
        || *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            != 0
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SMITE_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if !near_evil() {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && p1
            < borg.avoidance as ::core::ffi::c_int
                * (if borg.near.unique as ::core::ffi::c_int != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                })
                / 10 as ::core::ffi::c_int
    {
        if borg_simulate {
            return 5 as ::core::ffi::c_int;
        }
        if borg_spell(SMITE_EVIL) {
            return 5 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_glyph(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut glyph_spell: bool = false_0 != 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).take as ::core::ffi::c_int != 0
        || (*ag).trap as ::core::ffi::c_int != 0
        || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 20 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(GLYPH_OF_WARDING, fail_allowed) {
        glyph_spell = true_0 != 0;
    }
    if 0 as ::core::ffi::c_int
        <= borg_slot(
            TV_SCROLL as ::core::ffi::c_int,
            sv_scroll_rune_of_protection,
        )
    {
        glyph_spell = true_0 != 0;
    }
    if borg_equips_item(act_glyph, true_0 != 0) {
        glyph_spell = true_0 != 0;
    }
    if (*borg
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
            != 0)
        && glyph_spell as ::core::ffi::c_int != 0
    {
        glyph_spell = false_0 != 0;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        glyph_spell = false_0 != 0;
    }
    if !glyph_spell {
        return 0 as ::core::ffi::c_int;
    }
    borg_on_glyph = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_on_glyph = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_spell_fail(GLYPH_OF_WARDING, fail_allowed) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_rune_of_protection) as ::core::ffi::c_int != 0
            || borg_activate_item(act_glyph) as ::core::ffi::c_int != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < track_glyph.num as ::core::ffi::c_int {
                if *track_glyph.x.offset(i as isize) == borg.c.x
                    && *track_glyph.y.offset(i as isize) == borg.c.y
                {
                    return p1 - p2;
                }
                i += 1;
            }
            if (track_glyph.num as ::core::ffi::c_int) < track_glyph.size as ::core::ffi::c_int {
                borg_note(
                    b"# Noting the creation of a glyph.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                *track_glyph.x.offset(track_glyph.num as isize) = borg.c.x;
                *track_glyph.y.offset(track_glyph.num as isize) = borg.c.y;
                track_glyph.num += 1;
            }
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_create_door(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut door_bad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut door_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.summoner {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 5 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 20 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(DOOR_CREATION, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    door_x = -(1 as ::core::ffi::c_int);
    while door_x <= 1 as ::core::ffi::c_int {
        door_y = -(1 as ::core::ffi::c_int);
        while door_y <= 1 as ::core::ffi::c_int {
            x = door_x + borg.c.x;
            y = door_y + borg.c.y;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if (*ag).glyph as ::core::ffi::c_int != 0
                || (*ag).kill as ::core::ffi::c_int != 0
                || ((*ag).feat as ::core::ffi::c_int == FEAT_GRANITE as ::core::ffi::c_int
                    || (*ag).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                    || (*ag).feat as ::core::ffi::c_int == FEAT_CLOSED as ::core::ffi::c_int)
            {
                door_bad += 1;
            }
            if (*ag).take as ::core::ffi::c_int != 0
                || (*ag).trap as ::core::ffi::c_int != 0
                || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
                || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
                || (*ag).kill as ::core::ffi::c_int != 0
            {
                door_bad += 1;
            }
            door_y += 1;
        }
        door_x += 1;
    }
    if door_bad >= 6 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    borg_create_door = true_0 != 0;
    p2 = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    borg_create_door = false_0 != 0;
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 7 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_spell_fail(DOOR_CREATION, fail_allowed) {
            borg.near.breeder = true_0 != 0;
            borg_needs_new_sea = true_0 != 0;
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_mass_genocide(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = 0;
    let mut b_p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(MASS_BANISHMENT, 40 as ::core::ffi::c_int)
        && !borg_equips_item(act_banishment, true_0 != 0)
        && *borg
            .trait_0
            .offset(BI_AMASSBAN as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && borg_simulate as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if !(distance(borg.c, (*kill).pos) > 20 as ::core::ffi::c_int) {
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
                        p = borg_danger_one_kill(
                            borg.c.y,
                            borg.c.x,
                            1 as ::core::ffi::c_int,
                            i,
                            true_0 != 0,
                            true_0 != 0,
                        );
                        b_p = b_p + p;
                        hit = hit + 3 as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    p2 = p1 - b_p;
    if p2 < 0 as ::core::ffi::c_int {
        p2 = 0 as ::core::ffi::c_int;
    }
    if hit * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        return 0 as ::core::ffi::c_int;
    }
    p2 = p2 + hit;
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && hit / 3 as ::core::ffi::c_int > 8 as ::core::ffi::c_int
    {
        p2 = p2 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
    {
        if borg_simulate {
            return p1 - p2;
        }
        if borg_read_scroll(sv_scroll_mass_banishment) as ::core::ffi::c_int != 0
            || borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
            || borg_spell(MASS_BANISHMENT) as ::core::ffi::c_int != 0
        {
            i = 1 as ::core::ffi::c_int;
            while i < borg_kills_nxt as ::core::ffi::c_int {
                let mut tmp_kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
                let mut tmp_r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
                tmp_kill = borg_kills.offset(i as isize) as *mut borg_kill;
                if !((*tmp_kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                    if !((*tmp_kill).r_idx as ::core::ffi::c_int
                        >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    {
                        tmp_r_ptr = r_info.offset((*tmp_kill).r_idx as isize) as *mut monster_race;
                        if !flag_has(
                            &raw mut (*tmp_r_ptr).flags as *mut bitflag,
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
                            borg_delete_kill(i);
                        }
                    }
                }
                i += 1;
            }
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_genocide(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p_threat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p_without_kill: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_threat: ::core::ffi::c_int = 0;
    let mut kill_char: ::core::ffi::c_uchar = 0;
    let mut b_kill_count: [::core::ffi::c_int; 255] = [0; 255];
    let mut b_danger: [::core::ffi::c_int; 255] = [0; 255];
    let mut b_threat: [::core::ffi::c_int; 255] = [0; 255];
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut total_danger_to_me: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp_genocide_target: ::core::ffi::c_char =
        0 as ::core::ffi::c_int as ::core::ffi::c_char;
    let mut biggest_threat: ::core::ffi::c_uchar = 0;
    let mut biggest_danger: ::core::ffi::c_uchar = 0;
    let mut genocide_spell: bool = false_0 != 0;
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    if p1 > borg.avoidance as ::core::ffi::c_int {
        fail_allowed -= 19 as ::core::ffi::c_int;
    } else if p1
        > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
    {
        fail_allowed -= 10 as ::core::ffi::c_int;
    } else if p1 < borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
        fail_allowed += 10 as ::core::ffi::c_int;
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
        [(borg.c.x / 11 as ::core::ffi::c_int) as usize]
        != 0
    {
        p1 -= borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    }
    if borg_fear_monsters[borg.c.y as usize][borg.c.x as usize] != 0 {
        p1 -= borg_fear_monsters[borg.c.y as usize][borg.c.x as usize] as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(BANISHMENT, fail_allowed) as ::core::ffi::c_int != 0
        || borg_equips_item(act_banishment, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_staff_fail(sv_staff_banishment) as ::core::ffi::c_int != 0
        || -(1 as ::core::ffi::c_int)
            != borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_banishment)
    {
        genocide_spell = true_0 != 0;
    }
    if genocide_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) <= 75 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    biggest_danger = 0 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_uchar;
    biggest_threat = 0 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_uchar;
    i = 0 as ::core::ffi::c_int;
    while i < UCHAR_MAX {
        b_danger[i as usize] = 0 as ::core::ffi::c_int;
        b_kill_count[i as usize] = 0 as ::core::ffi::c_int;
        b_threat[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    max_threat = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
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
                    p = borg_danger_one_kill(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    p_threat = borg_danger_one_kill(
                        (*kill).pos.y,
                        (*kill).pos.x,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    kill_char = (*r_ptr).d_char as ::core::ffi::c_uchar;
                    total_danger_to_me += p;
                    b_danger[kill_char as usize] = b_danger[kill_char as usize] + p;
                    b_threat[kill_char as usize] = b_threat[kill_char as usize] + p_threat;
                    b_kill_count[kill_char as usize] += 1;
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < UCHAR_MAX {
        if !(b_kill_count[i as usize] == 0 as ::core::ffi::c_int) {
            if b_danger[i as usize] > max_threat {
                max_threat = b_danger[i as usize];
                biggest_danger = i as ::core::ffi::c_uchar;
                p_without_kill = total_danger_to_me - b_danger[biggest_danger as usize];
            }
            if b_threat[i as usize] > max_threat {
                max_threat = b_threat[i as usize];
                biggest_threat = i as ::core::ffi::c_uchar;
            }
            if !borg_simulate {
                borg_note(format(
                    b"# Race '%c' is a threat with total danger %d from %d individuals.\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    i,
                    b_threat[i as usize],
                    b_kill_count[i as usize],
                ));
            }
        }
        i += 1;
    }
    if biggest_threat != 0 {
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 375 as ::core::ffi::c_int
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_threat[biggest_threat as usize]
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                * 3 as ::core::ffi::c_int
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_kill_count[biggest_threat as usize]
            * 4 as ::core::ffi::c_int
            * 12 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
            >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if b_kill_count[biggest_threat as usize] >= 75 as ::core::ffi::c_int {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int {
            biggest_threat = 0 as ::core::ffi::c_uchar;
        }
        if biggest_threat != 0 {
            borg_note(format(
                b"# Race '%c' is a real threat with total danger %d from %d individuals.\0"
                    as *const u8 as *const ::core::ffi::c_char,
                biggest_threat as ::core::ffi::c_int,
                b_threat[biggest_threat as usize],
                b_kill_count[biggest_threat as usize],
            ));
        }
        tmp_genocide_target = biggest_threat as ::core::ffi::c_char;
    }
    if biggest_danger != 0 {
        if b_kill_count[biggest_danger as usize]
            * 4 as ::core::ffi::c_int
            * 12 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
            >= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if p1
            < borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                > 75 as ::core::ffi::c_int
                && p1
                    < borg.avoidance as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int
        {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if p_without_kill <= borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
            biggest_danger = 0 as ::core::ffi::c_uchar;
        }
        if biggest_danger != 0 {
            tmp_genocide_target = biggest_danger as ::core::ffi::c_char;
        }
    }
    if tmp_genocide_target != 0 {
        if borg_simulate {
            if biggest_danger != 0 {
                return p1 - p_without_kill;
            }
            if biggest_threat != 0 {
                return b_threat[biggest_threat as usize];
            }
        }
        if biggest_danger != 0 {
            borg_note(format(
                b"# Banishing race '%c' (qty:%d).  Danger after spell:%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp_genocide_target as ::core::ffi::c_int,
                b_kill_count[biggest_danger as usize],
                p_without_kill,
            ));
        }
        if biggest_threat != 0 {
            borg_note(format(
                b"# Banishing race '%c' (qty:%d).  Danger from them:%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp_genocide_target as ::core::ffi::c_int,
                b_kill_count[biggest_threat as usize],
                b_threat[biggest_threat as usize],
            ));
        }
        if borg_read_scroll(sv_scroll_banishment) as ::core::ffi::c_int != 0
            || borg_spell(BANISHMENT) as ::core::ffi::c_int != 0
            || borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_banishment) as ::core::ffi::c_int != 0
        {
            borg_keypress(tmp_genocide_target as keycode_t);
            i = 1 as ::core::ffi::c_int;
            while i < borg_kills_nxt as ::core::ffi::c_int {
                kill = borg_kills.offset(i as isize) as *mut borg_kill;
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                if !((*r_ptr).d_char != tmp_genocide_target as wchar_t) {
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
                        borg_delete_kill(i);
                    }
                }
                i += 1;
            }
        }
        if biggest_danger != 0 {
            return p1 - p_without_kill;
        }
        if biggest_threat != 0 {
            return b_threat[biggest_threat as usize];
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_genocide_nasties(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut genocide_spell: bool = false_0 != 0;
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 7 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) < 250 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        < 25 as ::core::ffi::c_int
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(BANISHMENT, 35 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_banishment, true_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_staff_fail(sv_staff_banishment) as ::core::ffi::c_int != 0
    {
        genocide_spell = true_0 != 0;
    }
    if genocide_spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_nasties_num as ::core::ffi::c_int {
        if borg_nasties_count[i as usize] as ::core::ffi::c_int
            >= borg_nasties_limit[i as usize] as ::core::ffi::c_int
        {
            b_i = i;
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Banishing nasties '%c' (qty:%d).\0" as *const u8 as *const ::core::ffi::c_char,
        borg_nasties[b_i as usize] as ::core::ffi::c_int,
        borg_nasties_count[b_i as usize] as ::core::ffi::c_int,
    ));
    if borg_activate_item(act_banishment) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_banishment) as ::core::ffi::c_int != 0
        || borg_spell(BANISHMENT) as ::core::ffi::c_int != 0
    {
        borg_keypress(borg_nasties[b_i as usize] as keycode_t);
        borg_nasties_count[b_i as usize] = 0 as uint8_t;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
            let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
            kill = borg_kills.offset(i as isize) as *mut borg_kill;
            r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
            if !((*r_ptr).d_char != borg_nasties[b_i as usize] as wchar_t) {
                borg_delete_kill(i);
            }
            i += 1;
        }
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_earthquake(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 9999 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut threat_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg_simulate {
        borg_needs_new_sea = true_0 != 0;
        if borg_spell(TREMOR) {
            borg_keypress('*' as i32 as keycode_t);
            borg_keypress('5' as i32 as keycode_t);
        } else if borg_spell(QUAKE) as ::core::ffi::c_int != 0
            || borg_spell(GRONDS_BLOW) as ::core::ffi::c_int != 0
            || borg_activate_item(act_earthquakes) as ::core::ffi::c_int != 0
        {
            return p2;
        }
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(TREMOR, 35 as ::core::ffi::c_int)
        && !borg_spell_okay_fail(QUAKE, 35 as ::core::ffi::c_int)
        && !borg_spell_okay_fail(GRONDS_BLOW, 35 as ::core::ffi::c_int)
        && !borg_equips_item(act_earthquakes, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1
        < borg.avoidance as ::core::ffi::c_int * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && !borg.near.summoner
    {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if borg_los(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) as ::core::ffi::c_int != 0
            && (*kill).ranged_attack as ::core::ffi::c_int != 0
            && distance((*kill).pos, borg.c) >= 2 as ::core::ffi::c_int
        {
            threat_count += 1;
        }
        i += 1;
    }
    if threat_count >= 4 as ::core::ffi::c_int
        && p1
            > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        p2 = p1 / 3 as ::core::ffi::c_int;
    }
    if threat_count == 3 as ::core::ffi::c_int
        && p1
            > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        p2 = p1 * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
        && p1 > borg.avoidance as ::core::ffi::c_int / 5 as ::core::ffi::c_int
    {
        if borg_simulate {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_destruction(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut spell: bool = false_0 != 0;
    let mut real_danger: bool = false_0 != 0;
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_simulate {
        if borg_spell(WORD_OF_DESTRUCTION) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_destruction) as ::core::ffi::c_int != 0
            || borg_activate_item(act_destruction2) as ::core::ffi::c_int != 0
        {
            borg_needs_new_sea = true_0 != 0;
        }
        return 500 as ::core::ffi::c_int;
    }
    if borg.morgoth_position {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int {
        real_danger = true_0 != 0;
    }
    if p1
        > borg.avoidance as ::core::ffi::c_int * 8 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 90 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            <= 300 as ::core::ffi::c_int
    {
        real_danger = true_0 != 0;
    }
    if real_danger as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg
        .trait_0
        .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize)
            != 0)
        && *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
        && borg.near.unique as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= 275 as ::core::ffi::c_int
    {
        if borg_caution_teleport(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *borg
        .trait_0
        .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= 275 as ::core::ffi::c_int
    {
        if borg_caution_teleport(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if borg_spell_okay_fail(WORD_OF_DESTRUCTION, 55 as ::core::ffi::c_int) as ::core::ffi::c_int
        != 0
        || borg_equips_staff_fail(sv_staff_destruction) as ::core::ffi::c_int != 0
        || borg_equips_item(act_destruction2, true_0 != 0) as ::core::ffi::c_int != 0
    {
        spell = true_0 != 0;
    }
    if (p1 > borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int
        || p1 > borg.avoidance as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                <= 150 as ::core::ffi::c_int)
        && borg_equips_staff_fail(sv_staff_destruction) as ::core::ffi::c_int != 0
    {
        spell = true_0 != 0;
    }
    if spell as ::core::ffi::c_int == false_0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = 0 as ::core::ffi::c_int;
    d = p1 - p2;
    if borg.near.unique as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
        && p1 < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        d = 0 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        d = 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return d;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_teleportlevel(
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !borg_simulate {
        if borg_spell(TELEPORT_LEVEL) {
            borg_needs_new_sea = true_0 != 0;
            return 500 as ::core::ffi::c_int;
        }
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 < borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*borg
        .trait_0
        .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize)
            != 0)
        && *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
    {
        if borg_caution_teleport(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *borg
        .trait_0
        .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
    {
        if borg_caution_teleport(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    if !borg_spell_okay_fail(TELEPORT_LEVEL, 55 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if borg.near.morgoth as ::core::ffi::c_int != 0
        || borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && borg.status.anti_summon as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return p1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_banishment(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut banished_monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut using_artifact: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if p1
        < borg.avoidance as ::core::ffi::c_int * 1 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int {
        fail_allowed -= 10 as ::core::ffi::c_int;
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    using_artifact = borg_equips_item(act_loskill, true_0 != 0) as ::core::ffi::c_int != 0
        && *borg.has.offset(BI_CURHP as ::core::ffi::c_int as isize) > 100 as ::core::ffi::c_int;
    if !using_artifact && !borg_spell_okay_fail(BANISH_EVIL, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    p1 = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
            .offset((*kill).pos.x as isize) as *mut borg_grid;
        if !((*kill).r_idx == 0) {
            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                p1 += borg_danger_one_kill(
                    borg.c.y,
                    borg.c.x,
                    1 as ::core::ffi::c_int,
                    i,
                    true_0 != 0,
                    true_0 != 0,
                );
            }
        }
        i += 1;
    }
    p2 = p1;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
        kill_0 = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill_0).r_idx as isize) as *mut monster_race;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill_0).pos.y as isize))
            .offset((*kill_0).pos.x as isize) as *mut borg_grid;
        if !((*kill_0).r_idx == 0) {
            if !((*kill_0).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                if borg_projectable(borg.c.y, borg.c.x, (*kill_0).pos.y, (*kill_0).pos.x) {
                    if !borg_simulate {
                        borg_note(format(
                            b"# Banishing Evil: (%d,%d): %s, danger %d. is considered.\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            (*kill_0).pos.y,
                            (*kill_0).pos.x,
                            borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                            borg_danger_one_kill(
                                borg.c.y,
                                borg.c.x,
                                1 as ::core::ffi::c_int,
                                (*ag).kill as ::core::ffi::c_int,
                                true_0 != 0,
                                false_0 != 0,
                            ),
                        ));
                    }
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
                        RF_EVIL as ::core::ffi::c_int,
                    ) {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Stays (not evil).\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    false_0 != 0,
                                ),
                            ));
                        }
                    } else if flag_has(
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
                    ) as ::core::ffi::c_int
                        != 0
                        && (*kill_0).injury as ::core::ffi::c_int > 60 as ::core::ffi::c_int
                    {
                        if !borg_simulate {
                            borg_note(
                                format(
                                    b"# Banishing Evil: (%d,%d): %s, danger %d. Unique not considered: Injury %d.\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    (*kill_0).pos.y,
                                    (*kill_0).pos.x,
                                    borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                    borg_danger_one_kill(
                                        borg.c.y,
                                        borg.c.x,
                                        1 as ::core::ffi::c_int,
                                        (*ag).kill as ::core::ffi::c_int,
                                        true_0 != 0,
                                        false_0 != 0,
                                    ),
                                    (*kill_0).injury as ::core::ffi::c_int,
                                ),
                            );
                        }
                    } else if !borg_cave_floor_bold((*kill_0).pos.y, (*kill_0).pos.x) {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Stays (in wall).\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                ),
                            ));
                        }
                    } else {
                        if !borg_simulate {
                            borg_note(format(
                                b"# Banishing Evil: (%d,%d): %s, danger %d. Booted.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*kill_0).pos.y,
                                (*kill_0).pos.x,
                                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                                borg_danger_one_kill(
                                    borg.c.y,
                                    borg.c.x,
                                    1 as ::core::ffi::c_int,
                                    (*ag).kill as ::core::ffi::c_int,
                                    true_0 != 0,
                                    true_0 != 0,
                                ),
                            ));
                            borg_delete_kill(i);
                        }
                        banished_monsters += 1;
                        p2 -= borg_danger_one_kill(
                            borg.c.y,
                            borg.c.x,
                            1 as ::core::ffi::c_int,
                            i,
                            true_0 != 0,
                            true_0 != 0,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    if !borg_simulate {
        if using_artifact {
            if borg_activate_item(act_loskill) {
                return p1 - p2;
            }
        }
        if borg_spell(BANISH_EVIL) {
            return p1 - p2;
        }
    }
    if p2 <= 0 as ::core::ffi::c_int {
        p2 = 0 as ::core::ffi::c_int;
    }
    if banished_monsters == 0 as ::core::ffi::c_int {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) > 250 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 99 as ::core::ffi::c_int
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) > 350 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 100 as ::core::ffi::c_int
    {
        p2 = 9999 as ::core::ffi::c_int;
    }
    if p1 > p2
        && p2
            <= (if borg.near.unique as ::core::ffi::c_int != 0 {
                borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            } else {
                borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            })
    {
        if borg_simulate {
            return p1 - p2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_inviso(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    if *borg
        .trait_0
        .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
            != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
        || borg.see_inv as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.time.now - borg.need_see_invis > 45 as borg_time {
        return 0 as ::core::ffi::c_int;
    }
    if p1 > borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if -(1 as ::core::ffi::c_int)
        == borg_slot(TV_POTION as ::core::ffi::c_int, sv_potion_detect_invis)
        && -(1 as ::core::ffi::c_int)
            == borg_slot(TV_SCROLL as ::core::ffi::c_int, sv_scroll_detect_invis)
        && !borg_equips_staff_fail(sv_staff_detect_invis)
        && !borg_equips_staff_fail(sv_staff_detect_evil)
        && !borg_spell_okay_fail(SENSE_INVISIBLE, fail_allowed)
        && !borg_spell_okay_fail(DETECTION, fail_allowed)
        && !borg_spell_okay_fail(REVEAL_MONSTERS, fail_allowed)
        && !borg_equips_item(act_detect_invis, true_0 != 0)
        && !borg_equips_item(act_tmd_sinvis, true_0 != 0)
        && !borg_equips_item(act_tmd_esp, true_0 != 0)
        && !borg_equips_item(act_detect_evil, true_0 != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(REVEAL_MONSTERS, fail_allowed) as ::core::ffi::c_int != 0
        || borg_spell_fail(DETECTION, fail_allowed) as ::core::ffi::c_int != 0
        || borg_read_scroll(sv_scroll_detect_invis) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_detect_invis) as ::core::ffi::c_int != 0
        || borg_use_staff(sv_staff_detect_evil) as ::core::ffi::c_int != 0
        || borg_activate_item(act_detect_invis) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tmd_sinvis) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tmd_esp) as ::core::ffi::c_int != 0
        || borg_activate_item(act_detect_evil) as ::core::ffi::c_int != 0
    {
        borg.see_inv = 3000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    if borg_quaff_potion(sv_potion_detect_invis) {
        borg.see_inv = 18000 as int16_t;
        borg.no_rest_prep = 18000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    if borg_spell_fail(SENSE_INVISIBLE, fail_allowed) {
        borg.see_inv = 30000 as int16_t;
        borg.no_rest_prep = 16000 as int16_t;
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_lbeam(mut p1: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut hallway: bool = false_0 != 0;
    let mut x: ::core::ffi::c_int = borg.c.x;
    let mut y: ::core::ffi::c_int = borg.c.y;
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg.time.now - borg.need_see_invis < 48 as borg_time {
        return 0 as ::core::ffi::c_int;
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
        hallway = true_0 != 0;
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
        hallway = true_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
    {
        hallway = true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
    {
        hallway = true_0 != 0;
    }
    if !hallway {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate as ::core::ffi::c_int != 0
        && p1
            > borg.avoidance as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_light_beam(true_0 != 0) {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 10 as ::core::ffi::c_int;
    }
    if borg_light_beam(false_0 != 0) {
        return 10 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_panel_shift() -> bool {
    let mut dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut panel_num_x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut panel_num_y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut pos_on_panel_x: ::core::ffi::c_int = borg.c.x - (*Term).offset_x;
    let mut pos_on_panel_y: ::core::ffi::c_int = borg.c.y - (*Term).offset_y;
    if !borg.need_shift_panel
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 70 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && w_y == borg.mon.morgoth_panel.y
        && w_x == borg.mon.morgoth_panel.x
    {
        return false_0 != 0;
    }
    if (*Term).offset_x > borg_panel_wid() {
        panel_num_x =
            (*Term).offset_x * 2 as ::core::ffi::c_int / borg_panel_wid() + 1 as ::core::ffi::c_int;
    }
    if (*Term).offset_y > borg_panel_hgt() {
        panel_num_y =
            (*Term).offset_y * 2 as ::core::ffi::c_int / borg_panel_hgt() + 1 as ::core::ffi::c_int;
    }
    if borg_panel_wid() - pos_on_panel_x < 5 as ::core::ffi::c_int && panel_num_x < borg.panels.x {
        dir = 6 as ::core::ffi::c_int;
    }
    if pos_on_panel_x < 5 as ::core::ffi::c_int && panel_num_x > 1 as ::core::ffi::c_int {
        dir = 4 as ::core::ffi::c_int;
    }
    if borg_panel_hgt() - pos_on_panel_y < 5 as ::core::ffi::c_int && panel_num_y < borg.panels.y {
        dir = 2 as ::core::ffi::c_int;
    }
    if pos_on_panel_y < 5 as ::core::ffi::c_int && panel_num_y > 1 as ::core::ffi::c_int {
        dir = 8 as ::core::ffi::c_int;
    }
    if dir == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if borg.need_shift_panel as ::core::ffi::c_int == true_0 {
        borg_keypress('L' as i32 as keycode_t);
        borg_keypress((dir + '0' as i32) as keycode_t);
        borg_note(
            b"# Shifted panel to locate offscreen monster.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.need_shift_panel = false_0 != 0;
        borg_keypress(ESCAPE as keycode_t);
        return true_0 != 0;
    } else if !(borg.when_shift_panel != 0
        && (borg.time.now - borg.when_shift_panel <= 10 as borg_time
            || borg.time.now - borg.time.morgoth <= 10 as borg_time))
    {
        if track_step.num != 0 {
            if dir == 8 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y - 2 as ::core::ffi::c_int,
                    borg.c.x,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.y.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.y - 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 2 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y + 2 as ::core::ffi::c_int,
                    borg.c.x,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.y.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.y + 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 4 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y,
                    borg.c.x - 2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.x.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.x - 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            } else if dir == 6 as ::core::ffi::c_int
                && borg_projectable_pure(
                    borg.c.y,
                    borg.c.x,
                    borg.c.y,
                    borg.c.x + 2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
                && *track_step.x.offset(
                    (track_step.num as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) != borg.c.x + 1 as ::core::ffi::c_int
            {
                borg_keypress('L' as i32 as keycode_t);
                borg_keypress((dir + '0' as i32) as keycode_t);
                borg_note(
                    b"# Shifted panel as a precaution.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg.when_shift_panel = borg.time.now;
                borg_keypress(ESCAPE as keycode_t);
                return true_0 != 0;
            }
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_defend_aux_rest() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !borg.morgoth_position
        && (!borg.status.anti_summon || borg.time.now - borg.time.antisummon >= 50 as borg_time)
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth
        && (!borg.status.anti_summon || borg.time.now - borg.time.antisummon >= 50 as borg_time)
    {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_spell_okay_fail(TELEPORT_OTHER, 30 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
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
            if !(d > (*z_info).max_range as ::core::ffi::c_int) {
                if borg_los(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) as ::core::ffi::c_int
                    != 0
                    && ((*kill).r_idx as ::core::ffi::c_uint == borg.id.morgoth
                        || (*kill).ranged_attack as ::core::ffi::c_int != 0)
                    && borg.avoidance as ::core::ffi::c_int
                        <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                {
                    borg_note(
                        b"# Not resting. I can see Morgoth or a shooter.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                borg.avoidance as ::core::ffi::c_int
                    > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize);
            }
        }
        i += 1;
    }
    if borg_simulate {
        return 200 as ::core::ffi::c_int;
    }
    borg_keypress(',' as i32 as keycode_t);
    borg_note(format(
        b"# Resting on grid (%d, %d), waiting for Morgoth.\0" as *const u8
            as *const ::core::ffi::c_char,
        borg.c.y,
        borg.c.x,
    ));
    return 200 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_tele_away_morgoth() -> ::core::ffi::c_int {
    let mut p2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_allowed: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if !(*borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(TELEPORT_OTHER, fail_allowed) {
        return 0 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_surrounded() as ::core::ffi::c_int == true_0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                    if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                        if !(distance(borg.c, (*kill).pos)
                            > (*z_info).max_range as ::core::ffi::c_int)
                        {
                            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    p2 = borg_launch_bolt(
        0 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        BORG_ATTACK_AWAY_ALL_MORGOTH as ::core::ffi::c_int,
        (*z_info).max_range as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if p2 > 1000 as ::core::ffi::c_int {
        p2 = 1000 as ::core::ffi::c_int;
    }
    borg_temp_n = 0 as int16_t;
    borg_tp_other_n = 0 as int16_t;
    if borg_simulate {
        return p2;
    }
    borg_log_spellpath();
    i = 0 as ::core::ffi::c_int;
    while i < borg_tp_other_n as ::core::ffi::c_int {
        borg_note(format(
            b"# %d, index %d (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg_tp_other_n as ::core::ffi::c_int,
            borg_tp_other_index[i as usize],
            borg_tp_other_y[i as usize] as ::core::ffi::c_int,
            borg_tp_other_x[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    borg_note(
        b"# Attempting to cast T.O. for depth 100.\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if borg_spell(TELEPORT_OTHER) as ::core::ffi::c_int != 0
        || borg_activate_item(act_tele_other) as ::core::ffi::c_int != 0
        || borg_aim_wand(sv_wand_teleport_away) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        successful_target = -(1 as ::core::ffi::c_int);
        return p2;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_banishment_morgoth() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut banish_evil: bool = false;
    let mut banishment: bool = false;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    if !borg.near.morgoth {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    banish_evil = borg_spell_okay_fail(BANISH_EVIL, fail_allowed);
    banishment = borg_spell_okay_fail(MASS_BANISHMENT, fail_allowed);
    if !banishment && !banish_evil {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    count = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    if !(banish_evil as ::core::ffi::c_int != 0
                        && !banishment
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
                            RF_EVIL as ::core::ffi::c_int,
                        ))
                    {
                        if !(distance(borg.c, (*kill).pos)
                            > (*z_info).max_range as ::core::ffi::c_int)
                        {
                            if borg_projectable(borg.c.y, borg.c.x, (*kill).pos.y, (*kill).pos.x) {
                                if distance(borg.c, (*kill).pos) <= 7 as ::core::ffi::c_int {
                                    count += 1;
                                }
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if count <= 7 as ::core::ffi::c_int && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 1500 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Attempting to cast Banishment for depth 100.  %d monsters \0" as *const u8
            as *const ::core::ffi::c_char,
        count,
    ));
    if borg_spell(MASS_BANISHMENT) as ::core::ffi::c_int != 0
        || borg_spell(BANISH_EVIL) as ::core::ffi::c_int != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut tmp_kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
            let mut tmp_r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
            tmp_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*tmp_kill).r_idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*tmp_kill).r_idx as ::core::ffi::c_int
                    >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                tmp_r_ptr = r_info.offset((*tmp_kill).r_idx as isize) as *mut monster_race;
                if !flag_has(
                    &raw mut (*tmp_r_ptr).flags as *mut bitflag,
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
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        return 1000 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_defend_aux_light_morgoth() -> ::core::ffi::c_int {
    let mut fail_allowed: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut best: loc = loc { x: 0, y: 0 };
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    if !borg.morgoth_position {
        return 0 as ::core::ffi::c_int;
    }
    if !borg.near.morgoth {
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
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !borg_spell_okay_fail(SPEAR_OF_LIGHT, fail_allowed)
        && !borg_spell_okay_fail(CLAIRVOYANCE, fail_allowed)
        && !borg_spell_okay_fail(FUME_OF_MORDOR, fail_allowed)
    {
        return 0 as ::core::ffi::c_int;
    }
    if borg_kills_cnt == 0 && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_uint != borg.id.morgoth) {
                if !(borg.time.now - (*kill).when > 2 as borg_time) {
                    x = (*kill).pos.x;
                    y = (*kill).pos.y;
                    if !(distance(borg.c, (*kill).pos) > (*z_info).max_range as ::core::ffi::c_int)
                    {
                        if !(distance(borg.c, (*kill).pos) <= 5 as ::core::ffi::c_int) {
                            if borg_projectable_dark(borg.c.y, borg.c.x, y, x) {
                                count += 1;
                                best.y = y;
                                best.x = x;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if count <= 0 as ::core::ffi::c_int && borg_simulate as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if borg_simulate {
        return 500 as ::core::ffi::c_int;
    }
    borg_note(format(
        b"# Attempting to Illuminate a Pathway to (%d, %d)\0" as *const u8
            as *const ::core::ffi::c_char,
        best.y,
        best.x,
    ));
    borg_target(best, false_0 != 0);
    if borg_spell(SPEAR_OF_LIGHT) as ::core::ffi::c_int != 0
        || borg_spell(CLAIRVOYANCE) as ::core::ffi::c_int != 0
        || borg_spell(FUME_OF_MORDOR) as ::core::ffi::c_int != 0
    {
        borg_keypress('5' as i32 as keycode_t);
        return 200 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_calculate_defense_effectiveness(
    mut what: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match what {
        1 => return borg_defend_aux_speed(p1),
        9 => return borg_defend_aux_prot_evil(p1),
        2 => return borg_defend_aux_grim_purpose(p1),
        3 => return borg_defend_aux_resist_fecap(p1),
        4 => return borg_defend_aux_resist_f(p1),
        5 => return borg_defend_aux_resist_c(p1),
        6 => return borg_defend_aux_resist_a(p1),
        7 => return borg_defend_aux_resist_e(p1),
        8 => return borg_defend_aux_resist_p(p1),
        0 => return borg_defend_aux_bless(p1),
        12 => return borg_defend_aux_hero(p1),
        13 => return borg_defend_aux_berserk(p1),
        14 => return borg_defend_aux_smite_evil(p1),
        15 => return borg_defend_aux_regen(p1),
        10 => return borg_defend_aux_shield(p1),
        11 => return borg_defend_aux_tele_away(p1),
        16 => return borg_defend_aux_glyph(p1),
        17 => return borg_defend_aux_create_door(p1),
        18 => return borg_defend_aux_mass_genocide(p1),
        19 => return borg_defend_aux_genocide(p1),
        20 => return borg_defend_aux_genocide_nasties(p1),
        21 => return borg_defend_aux_earthquake(p1),
        23 => return borg_defend_aux_teleportlevel(p1),
        22 => return borg_defend_aux_destruction(p1),
        24 => return borg_defend_aux_banishment(p1),
        25 => return borg_defend_aux_inviso(p1),
        26 => return borg_defend_aux_lbeam(p1),
        27 => return borg_defend_aux_rest(),
        28 => return borg_defend_aux_tele_away_morgoth(),
        29 => return borg_defend_aux_banishment_morgoth(),
        30 => return borg_defend_aux_light_morgoth(),
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_defend(mut p1: ::core::ffi::c_int) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut g: ::core::ffi::c_int = 0;
    let mut b_g: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if borg_defend_panel_shift() {
        return true_0 != 0;
    }
    borg_simulate = true_0 != 0;
    if borg.resistance as ::core::ffi::c_int != 0
        && (borg.resistance as ::core::ffi::c_int)
            < borg_game_ratio as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        let mut p: ::core::ffi::c_int = 0;
        borg_attacking = true_0 != 0;
        p = borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            false_0 != 0,
            false_0 != 0,
        );
        borg_attacking = false_0 != 0;
        if p > borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            || borg.near.unique as ::core::ffi::c_int != 0
        {
            if borg_spell(RESISTANCE) {
                borg_note(format(
                    b"# Refreshing Resistance.  borg.resistance=%d, player->=%d, (ratio=%d)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    borg.resistance as ::core::ffi::c_int,
                    *(*player)
                        .timed
                        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    borg_game_ratio as ::core::ffi::c_int,
                ));
                borg_attempting_refresh_resist = true_0 != 0;
                borg.resistance = 25000 as int16_t;
                return true_0 != 0;
            }
        }
    }
    g = 0 as ::core::ffi::c_int;
    while g < BD_MAX as ::core::ffi::c_int {
        n = borg_calculate_defense_effectiveness(g, p1);
        if !(n <= b_n) {
            b_g = g;
            b_n = n;
        }
        g += 1;
    }
    if b_n <= 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing defense type %d with value %d\0" as *const u8 as *const ::core::ffi::c_char,
        b_g,
        b_n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_defense_effectiveness(b_g, p1);
    return true_0 != 0;
}
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
