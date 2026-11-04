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
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    static mut player: *mut player;
    fn Term_xtra(n: ::core::ffi::c_int, v: ::core::ffi::c_int) -> errr;
    fn do_cmd_redraw();
    fn borg_munchkin_mage() -> bool;
    fn borg_munchkin_melee() -> bool;
    fn borg_caution() -> bool;
    static mut borg_grids: [*mut borg_grid; 66];
    static mut borg_danger_wipe: bool;
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    fn borg_recall() -> bool;
    fn borg_shadow_shift(allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_dimension_door(allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_allow_teleport() -> bool;
    fn borg_spell(spell: borg_spells) -> bool;
    fn borg_attack(boosted_bravery: bool) -> bool;
    fn borg_defend(p1: ::core::ffi::c_int) -> bool;
    fn borg_perma_spell() -> bool;
    static mut track_step: borg_track;
    static mut track_door: borg_track;
    static mut track_closed: borg_track;
    fn borg_flow_old(why: ::core::ffi::c_int) -> bool;
    fn borg_flow_dark(neer: bool) -> bool;
    static mut track_glyph: borg_track;
    fn borg_flow_glyph() -> bool;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut cave: *mut chunk;
    fn distance(grid1: loc, grid2: loc) -> ::core::ffi::c_int;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square(c: *mut chunk, grid: loc) -> *const square;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    fn borg_notice(notice_swap: bool);
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_kills_cnt: int16_t;
    static mut borg_kills_nxt: int16_t;
    fn borg_delete_kill(i: ::core::ffi::c_int);
    fn borg_flow_kill(viewable: bool, nearness: ::core::ffi::c_int) -> bool;
    fn borg_flow_kill_aim(viewable: bool) -> bool;
    fn borg_flow_kill_corridor() -> bool;
    fn borg_flow_kill_direct(twitchy: bool) -> bool;
    fn borg_near_monster_type(dist: ::core::ffi::c_int);
    static mut track_vein: borg_track;
    fn borg_check_rest(y: ::core::ffi::c_int, x: ::core::ffi::c_int) -> bool;
    fn borg_flow_recover(dist: ::core::ffi::c_int) -> bool;
    fn borg_flow_vein(viewable: bool, nearness: ::core::ffi::c_int) -> bool;
    fn borg_flow_spastic(bored: bool) -> bool;
    fn borg_flow_shop_entry(i: ::core::ffi::c_int) -> bool;
    fn borg_flow_vault(nearness: ::core::ffi::c_int) -> bool;
    fn borg_twitchy() -> bool;
    static mut track_less: borg_track;
    static mut track_more: borg_track;
    fn borg_flow_stair_both(why: ::core::ffi::c_int, sneak: bool) -> bool;
    fn borg_flow_stair_less(why: ::core::ffi::c_int, sneak: bool) -> bool;
    fn borg_flow_stair_more(why: ::core::ffi::c_int, sneak: bool, brave: bool) -> bool;
    static mut borg_takes_cnt: int16_t;
    static mut borg_takes_nxt: int16_t;
    fn borg_delete_take(i: ::core::ffi::c_int);
    fn borg_flow_take(viewable: bool, nearness: ::core::ffi::c_int) -> bool;
    fn borg_flow_take_lunal(viewable: bool, nearness: ::core::ffi::c_int) -> bool;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    static mut act_tele_level: ::core::ffi::c_int;
    static mut act_tele_phase: ::core::ffi::c_int;
    fn borg_enchanting() -> bool;
    fn borg_read_scroll(sval: ::core::ffi::c_int) -> bool;
    fn borg_use_staff(sval: ::core::ffi::c_int) -> bool;
    fn borg_activate_item(activation: ::core::ffi::c_int) -> bool;
    fn borg_use_things() -> bool;
    fn borg_recharging() -> bool;
    static mut sv_scroll_phase_door: ::core::ffi::c_int;
    static mut sv_scroll_teleport: ::core::ffi::c_int;
    static mut sv_scroll_teleport_level: ::core::ffi::c_int;
    static mut sv_staff_teleportation: ::core::ffi::c_int;
    fn borg_test_stuff() -> bool;
    fn borg_swap_rings() -> bool;
    fn borg_wear_rings() -> bool;
    fn borg_wear_stuff() -> bool;
    fn borg_wear_recharge() -> bool;
    fn borg_destroy_floor() -> bool;
    fn borg_drop_junk() -> bool;
    fn borg_drop_hole(desperate: bool) -> bool;
    fn borg_drop_slow() -> bool;
    fn borg_dump_quiver() -> bool;
    fn borg_remove_stuff() -> bool;
    fn borg_check_light() -> bool;
    fn borg_maintain_light() -> borg_need;
    fn borg_light_beam(simulation: bool) -> bool;
    fn borg_play_magic(bored: bool) -> bool;
    fn borg_prepared(depth: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_must_return_to_town() -> *const ::core::ffi::c_char;
    fn borg_projectable(
        y1: ::core::ffi::c_int,
        x1: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
    ) -> bool;
    fn borg_recover() -> bool;
    fn borg_count_sell() -> ::core::ffi::c_int;
    fn borg_money_scum() -> bool;
    fn borg_think_dungeon_light() -> bool;
    fn borg_think_stair_scum() -> bool;
    fn borg_leave_level(bored: bool) -> bool;
    fn borg_excavate_vault(range: ::core::ffi::c_int) -> bool;
    fn borg_choose_shop() -> bool;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PF_MAX: C2RustUnnamed_1 = 18;
pub const PF_COMBAT_REGEN: C2RustUnnamed_1 = 17;
pub const PF_EVIL: C2RustUnnamed_1 = 16;
pub const PF_SHIELD_BASH: C2RustUnnamed_1 = 15;
pub const PF_STEAL: C2RustUnnamed_1 = 14;
pub const PF_ROCK: C2RustUnnamed_1 = 13;
pub const PF_UNLIGHT: C2RustUnnamed_1 = 12;
pub const PF_CHARM: C2RustUnnamed_1 = 11;
pub const PF_NO_MANA: C2RustUnnamed_1 = 10;
pub const PF_SEE_ORE: C2RustUnnamed_1 = 9;
pub const PF_KNOW_ZAPPER: C2RustUnnamed_1 = 8;
pub const PF_KNOW_MUSHROOM: C2RustUnnamed_1 = 7;
pub const PF_CHOOSE_SPELLS: C2RustUnnamed_1 = 6;
pub const PF_BEAM: C2RustUnnamed_1 = 5;
pub const PF_ZERO_FAIL: C2RustUnnamed_1 = 4;
pub const PF_BLESS_WEAPON: C2RustUnnamed_1 = 3;
pub const PF_BRAVERY_30: C2RustUnnamed_1 = 2;
pub const PF_FAST_SHOT: C2RustUnnamed_1 = 1;
pub const PF_NONE: C2RustUnnamed_1 = 0;
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
pub struct C2RustUnnamed_2 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
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
    pub temp: C2RustUnnamed_9,
    pub status: C2RustUnnamed_8,
    pub time: C2RustUnnamed_7,
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
    pub goal: C2RustUnnamed_6,
    pub init_save: C2RustUnnamed_5,
    pub mon: C2RustUnnamed_4,
    pub id: C2RustUnnamed_3,
    pub near: C2RustUnnamed_2,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
pub const BI_CDEPTH: C2RustUnnamed_11 = 105;
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
pub const BI_CURHP: C2RustUnnamed_11 = 27;
pub type keycode_t = uint32_t;
pub const FEAT_MORE: C2RustUnnamed_10 = 6;
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
pub const BI_MAXHP: C2RustUnnamed_11 = 28;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_12 = 10;
pub const BI_LIGHT: C2RustUnnamed_11 = 26;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_12 = 24;
pub const BI_GOLD: C2RustUnnamed_11 = 45;
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
pub const BI_CLEVEL: C2RustUnnamed_11 = 35;
pub const BI_ISCONFUSED: C2RustUnnamed_11 = 114;
pub const BI_ISBLIND: C2RustUnnamed_11 = 112;
pub const BI_AGLYPH: C2RustUnnamed_11 = 230;
pub const BI_KING: C2RustUnnamed_11 = 107;
pub const BI_CURSP: C2RustUnnamed_11 = 30;
pub const BI_MAXSP: C2RustUnnamed_11 = 31;
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
pub const BI_QUIVER_SLOTS: C2RustUnnamed_11 = 158;
pub const BORG_MET_NEED: borg_need = 1;
pub type borg_need = ::core::ffi::c_uint;
pub const BORG_UNMET_NEED: borg_need = 2;
pub const BORG_NO_NEED: borg_need = 0;
pub const BI_FOOD: C2RustUnnamed_11 = 39;
pub const BI_ISPOISONED: C2RustUnnamed_11 = 115;
pub const BI_ISCUT: C2RustUnnamed_11 = 116;
pub const FEAT_LESS: C2RustUnnamed_10 = 5;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_12 = 3;
pub const BI_ISHUNGRY: C2RustUnnamed_11 = 109;
pub const BI_ISWEAK: C2RustUnnamed_11 = 108;
pub const BI_MAXCLEVEL: C2RustUnnamed_11 = 36;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_12 = 1;
pub const BI_MAXDEPTH: C2RustUnnamed_11 = 106;
pub const BORG_SELF_LUNAL: C2RustUnnamed_12 = 27;
pub const BI_RECALL: C2RustUnnamed_11 = 38;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_12 = 23;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_10 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_10 = 24;
pub const FEAT_LAVA: C2RustUnnamed_10 = 23;
pub const FEAT_PERM: C2RustUnnamed_10 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_10 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_10 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_10 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_10 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_10 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_10 = 16;
pub const FEAT_SECRET: C2RustUnnamed_10 = 15;
pub const FEAT_HOME: C2RustUnnamed_10 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_10 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_10 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_10 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_10 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_10 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_10 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_10 = 7;
pub const FEAT_BROKEN: C2RustUnnamed_10 = 4;
pub const FEAT_OPEN: C2RustUnnamed_10 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_10 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_10 = 1;
pub const FEAT_NONE: C2RustUnnamed_10 = 0;
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
pub const BI_CRSFEAR: C2RustUnnamed_11 = 186;
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
pub const BI_ISAFRAID: C2RustUnnamed_11 = 113;
pub const BI_ISGORGED: C2RustUnnamed_11 = 111;
pub const BI_ISFULL: C2RustUnnamed_11 = 110;
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
pub const BI_IFIRE: C2RustUnnamed_11 = 64;
pub const BI_DIG: C2RustUnnamed_11 = 63;
pub const BI_THT: C2RustUnnamed_11 = 62;
pub const BI_THB: C2RustUnnamed_11 = 61;
pub const BI_THN: C2RustUnnamed_11 = 60;
pub const BI_SRCH: C2RustUnnamed_11 = 59;
pub const BI_STL: C2RustUnnamed_11 = 58;
pub const BI_SAV: C2RustUnnamed_11 = 57;
pub const BI_DEV: C2RustUnnamed_11 = 56;
pub const BI_DISM: C2RustUnnamed_11 = 55;
pub const BI_DISP: C2RustUnnamed_11 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_11 = 53;
pub const BI_INFRA: C2RustUnnamed_11 = 52;
pub const BI_SINV: C2RustUnnamed_11 = 51;
pub const BI_REG: C2RustUnnamed_11 = 50;
pub const BI_FEATH: C2RustUnnamed_11 = 49;
pub const BI_SDIG: C2RustUnnamed_11 = 48;
pub const BI_DAM_RED: C2RustUnnamed_11 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_11 = 46;
pub const BI_SPEED: C2RustUnnamed_11 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_11 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_11 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_11 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_11 = 40;
pub const BI_ESP: C2RustUnnamed_11 = 37;
pub const BI_FAIL2: C2RustUnnamed_11 = 34;
pub const BI_FAIL1: C2RustUnnamed_11 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_11 = 32;
pub const BI_HP_ADJ: C2RustUnnamed_11 = 29;
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
pub const BORG_LUNAL_MODE: C2RustUnnamed_12 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_12 = 25;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_12 = 22;
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
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_12 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_12 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_12 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_12 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_12 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_12 = 4;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_12 = 2;
pub const BORG_VERBOSE: C2RustUnnamed_12 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TERM_XTRA_DELAY: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_feeling_danger: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_feeling_stuff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_think_dungeon_lunal() -> bool {
    let mut safe_place: bool = false_0 != 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut feat: uint8_t = (*square(cave, borg.c)).feat;
    let mut need: borg_need = BORG_NO_NEED;
    borg_notice(true_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        borg_note(
            b"# Leaving Lunal Mode. (Town or Weak)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_MORE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            if *track_more.x.offset(i as isize) == borg.c.x
                && *track_more.y.offset(i as isize) == borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_more.num as ::core::ffi::c_int && i < track_more.size as ::core::ffi::c_int {
            *track_more.x.offset(i as isize) = borg.c.x;
            *track_more.y.offset(i as isize) = borg.c.y;
            track_more.num += 1;
        }
        (*ag).feat = FEAT_MORE as ::core::ffi::c_int as uint8_t;
    }
    if feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_LESS as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) == borg.c.x
                && *track_less.y.offset(i as isize) == borg.c.y;
            i += 1;
        }
        if i == track_less.num as ::core::ffi::c_int && i < track_less.size as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) = borg.c.x;
            *track_less.y.offset(i as isize) = borg.c.y;
            track_less.num += 1;
        }
        (*ag).feat = FEAT_LESS as ::core::ffi::c_int as uint8_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && track_less.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg_note(
            b"# Leaving Lunal Mode. (No Stairs seen)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if *borg_cfg.offset(BORG_SELF_LUNAL as ::core::ffi::c_int as isize) != 0 {
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            <= *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 15 as ::core::ffi::c_int
            || !(borg_prepared(
                *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    - 5 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char)
                .is_null()
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 50 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
        {
            borg.lunal_mode = false_0 != 0;
            borg.goal.fleeing = false_0 != 0;
            borg.goal.fleeing_lunal = false_0 != 0;
            borg_note(
                b"# Self Lunal mode disengaged normally.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(b"# Lunal. (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
        .tval as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        borg_note(b"# No Light at all.\0" as *const u8 as *const ::core::ffi::c_char);
        return false_0 != 0;
    }
    safe_place = borg_check_rest(borg.c.y, borg.c.x);
    if safe_place as ::core::ffi::c_int != 0 && borg_recover() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_use_things() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        && borg_use_things() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_drop_junk() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_flow_old(GOAL_TAKE) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_flow_take_lunal(true_0 != 0, 4 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    borg_note(b"# Fleeing level. Lunal Mode\0" as *const u8 as *const ::core::ffi::c_char);
    borg.goal.fleeing_lunal = true_0 != 0;
    borg.goal.fleeing = true_0 != 0;
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 2 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut closeness: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
        let mut tmp_ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        {
            if track_less.num != 0 {
                closeness = 20 as ::core::ffi::c_int;
            }
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Lunal Mode.  Power Climb (needing to sell). \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Lunal Mode (needing to sell).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_more.num as ::core::ffi::c_int != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int)
    {
        let mut y_0: ::core::ffi::c_int = 0;
        let mut x_0: ::core::ffi::c_int = 0;
        if track_more.num as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            borg_note(
                b"# Lunal Mode: I know of a down stair.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            x_0 = *track_more.x.offset(i as isize);
            y_0 = *track_more.y.offset(i as isize);
            j = distance(borg.c, loc(x_0, y_0));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int && safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int
        {
            borg_note(
                b"# Lunal Mode.  Power Diving. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_more(GOAL_FLEE, true_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
            if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
                borg_keypress('>' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 1 as ::core::ffi::c_int
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut y_1: ::core::ffi::c_int = 0;
        let mut x_1: ::core::ffi::c_int = 0;
        let mut tmp_ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x_1 = *track_less.x.offset(i as isize);
            y_1 = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x_1, y_1));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(b"# Lunal Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Lunal Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
    {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        borg_note(b"# Lunal Mode.  Any Stair. \0" as *const u8 as *const ::core::ffi::c_char);
        if borg_flow_stair_both(GOAL_FLEE, true_0 != 0) {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 99 as ::core::ffi::c_int
    {
        borg_note(b"# Lunal Mode ended at depth.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        > 1 as ::core::ffi::c_int
    {
        borg_note(b"# Lunal Mode ended incorrectly.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    borg_note(
        b"Leaving Lunal Mode. (End of Lunal Mode)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg.lunal_mode = false_0 != 0;
    borg.goal.fleeing_lunal = false_0 != 0;
    borg.goal.fleeing = borg.goal.fleeing_lunal;
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_dungeon_munchkin() -> bool {
    let mut safe_place: bool = false_0 != 0;
    let mut bb_j: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut closeness: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut feat: uint8_t = (*square(cave, borg.c)).feat;
    let mut need: borg_need = BORG_NO_NEED;
    borg_notice(true_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        borg_note(
            b"# Leaving munchkin Mode. (Town or Weak)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_MORE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            if *track_more.x.offset(i as isize) == borg.c.x
                && *track_more.y.offset(i as isize) == borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_more.num as ::core::ffi::c_int && i < track_more.size as ::core::ffi::c_int {
            *track_more.x.offset(i as isize) = borg.c.x;
            *track_more.y.offset(i as isize) = borg.c.y;
            track_more.num += 1;
        }
        (*ag).feat = FEAT_MORE as ::core::ffi::c_int as uint8_t;
    }
    if feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_LESS as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) == borg.c.x
                && *track_less.y.offset(i as isize) == borg.c.y;
            i += 1;
        }
        if i == track_less.num as ::core::ffi::c_int && i < track_less.size as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) = borg.c.x;
            *track_less.y.offset(i as isize) = borg.c.y;
            track_less.num += 1;
        }
        (*ag).feat = FEAT_LESS as ::core::ffi::c_int as uint8_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    if track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && track_less.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg_note(
            b"# Leaving Munchkin Mode. (No Stairs seen)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(b"# Munchkin. (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
        borg_note(b"# No Light at all.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    safe_place = borg_check_rest(borg.c.y, borg.c.x);
    if borg_munchkin_mage() {
        return true_0 != 0;
    }
    if borg_munchkin_melee() {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_use_things() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        && borg_use_things() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_wear_stuff() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_remove_stuff() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_drop_junk() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_play_magic(true_0 != 0) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_flow_take_lunal(true_0 != 0, 5 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_recover() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    borg_note(b"# Fleeing level. Munchkin Mode\0" as *const u8 as *const ::core::ffi::c_char);
    borg.goal.fleeing_munchkin = true_0 != 0;
    borg.goal.fleeing = true_0 != 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        <= 10 as ::core::ffi::c_int
    {
        closeness += *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - 10 as ::core::ffi::c_int
            + (10 as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize));
    }
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 2 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int)
    {
        let mut tmp_ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        {
            if track_less.num != 0 {
                closeness = 20 as ::core::ffi::c_int;
            }
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb (needing to sell). \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode (needing to sell).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            > *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut tmp_ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        b_j = -(1 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                    if b_j < bb_j {
                        bb_j = b_j;
                    }
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_more.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int)
    {
        b_j = -(1 as ::core::ffi::c_int);
        if track_more.num as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            borg_note(
                b"# Munchkin Mode: I know of a down stair.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            x = *track_more.x.offset(i as isize);
            y = *track_more.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 1 as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Diving. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_more(GOAL_FLEE, true_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
            if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
                borg_keypress('>' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 1 as ::core::ffi::c_int
        && safe_place as ::core::ffi::c_int != 0
        || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
    {
        let mut tmp_ag_1: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_1).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_1).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
        || !safe_place
    {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        borg_note(b"# Munchkin Mode.  Any Stair. \0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            if square_in_bounds(cave, loc(x, y)) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                    .offset(x as isize) as *mut borg_grid;
                if (*ag).kill != 0 {
                    ii = 0 as ::core::ffi::c_int;
                    while ii < track_less.num as ::core::ffi::c_int {
                        x = *track_less.x.offset(ii as isize);
                        y = *track_less.y.offset(ii as isize);
                        j = distance(borg.c, loc(x, y));
                        if borg_projectable(y, x, borg.c.y, borg.c.x) {
                            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                                b_j = j;
                            }
                        }
                        ii += 1;
                    }
                    ii = 0 as ::core::ffi::c_int;
                    while ii < track_more.num as ::core::ffi::c_int {
                        x = *track_more.x.offset(ii as isize);
                        y = *track_more.y.offset(ii as isize);
                        j = distance(borg.c, loc(x, y));
                        if borg_projectable(y, x, borg.c.y, borg.c.x) {
                            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                                b_j = j;
                            }
                        }
                        ii += 1;
                    }
                    if b_j <= 3 as ::core::ffi::c_int {
                        if borg_flow_stair_both(GOAL_FLEE, false_0 != 0) {
                            return true_0 != 0;
                        }
                    } else if borg_attack(false_0 != 0) {
                        return true_0 != 0;
                    }
                }
            }
            i += 1;
        }
        if borg_flow_stair_both(GOAL_FLEE, false_0 != 0) {
            return true_0 != 0;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
            borg_keypress('>' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    borg_note(b"Leaving Munchkin Mode. (End of Mode)\0" as *const u8 as *const ::core::ffi::c_char);
    borg.munchkin_mode = false_0 != 0;
    borg.goal.fleeing_munchkin = false_0 != 0;
    borg.goal.fleeing = borg.goal.fleeing_munchkin;
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_dungeon_brave() -> bool {
    let mut p1: ::core::ffi::c_int = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && borg_defend(p1) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_attack(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_light_beam(false_0 != 0) {
        return true_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        == FEAT_MORE as ::core::ffi::c_int
    {
        borg_note(b"# Fleeing via stairs.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('>' as i32 as keycode_t);
        return true_0 != 0;
    }
    if borg.goal.less {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Goal_less, brave.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if borg.goal.fleeing as ::core::ffi::c_int != 0
        || borg.goal.leaving as ::core::ffi::c_int != 0
        || borg.mon.scary as ::core::ffi::c_int != 0
    {
        borg.stair_less = borg.goal.fleeing;
        if borg.ready_morgoth == 0 as ::core::ffi::c_int {
            borg.stair_less = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg.stair_less = false_0 != 0;
        }
        if borg.stair_less as ::core::ffi::c_int == true_0 {
            borg_note(
                b"# Fleeing and leaving the level. Brave Thinking.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        borg.stair_more = borg.goal.fleeing;
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        {
            borg.stair_more = true_0 != 0;
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.stair_less {
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Flee, brave.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        if borg.stair_more {
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, true_0 != 0) {
                return true_0 != 0;
            }
        }
    }
    if borg.status.vault {
        if borg_flow_old(GOAL_KILL) {
            return true_0 != 0;
        }
        if borg_flow_kill(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_old(GOAL_TAKE) {
            return true_0 != 0;
        }
        if borg_flow_take(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_vein(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_old(GOAL_VAULT) {
            return true_0 != 0;
        }
        if borg_flow_vault(35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_kill(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_MISC) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_DARK) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_XTRA) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_BORE) {
        return true_0 != 0;
    }
    if borg_flow_dark(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_dark(false_0 != 0) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_take(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_kill(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_leave_level(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_spastic(true_0 != 0) {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_check_bounciness() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && (borg.antibounce_count as ::core::ffi::c_int >= 300 as ::core::ffi::c_int
            && borg.antibounce_count as ::core::ffi::c_int <= 303 as ::core::ffi::c_int)
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && (borg.antibounce_count as ::core::ffi::c_int >= 500 as ::core::ffi::c_int
            && borg.antibounce_count as ::core::ffi::c_int <= 503 as ::core::ffi::c_int)
    {
        i = 1 as ::core::ffi::c_int;
        while i < borg_takes_nxt as ::core::ffi::c_int {
            borg_delete_take(i);
            i += 1;
        }
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            borg_delete_kill(i);
            i += 1;
        }
        borg_kills_cnt = 0 as int16_t;
        borg_kills_nxt = 1 as int16_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && borg.antibounce_count as ::core::ffi::c_int >= 700 as ::core::ffi::c_int
    {
        if !borg.goal.leaving {
            borg_note(b"# Leaving (bouncing-borg)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (bouncing-borg)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_check_breeders() -> bool {
    if borg.mon.breeders as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
        borg.near.breeder = true_0 != 0;
    }
    if borg.mon.breeders as ::core::ffi::c_int
        >= (if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int
            > 5 as ::core::ffi::c_int
        {
            5 as ::core::ffi::c_int
        } else {
            *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 2 as ::core::ffi::c_int
        })
        && (*borg
            .trait_0
            .offset(BI_RECALL as ::core::ffi::c_int as isize)
            <= 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 35 as ::core::ffi::c_int)
    {
        if !borg.goal.ignoring && borg.time.now - borg.time.level >= 2500 as borg_time {
            borg_note(
                b"# Ignoring breeders (no recall)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.ignoring = true_0 != 0;
        }
        if !borg.goal.leaving {
            borg_note(b"# Leaving (no recall)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (no recall)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_dungeon() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut msec: ::core::ffi::c_int = 0;
    if borg.time.now >= 12000 as borg_time && borg.time.now <= 12025 as borg_time
        || borg.time.now >= 25000 as borg_time && borg.time.now <= 25025 as borg_time
    {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.status.old_depth = 126 as int16_t;
        return true_0 != 0;
    }
    if borg.time.now >= BORG_TIME_MAX as borg_time - 3000 as borg_time {
        borg_oops(b"clock overflow\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    borg.trying_unknown = false_0 != 0;
    if borg_destroy_floor() {
        return true_0 != 0;
    }
    msec = (*player).opts.delay_factor as ::core::ffi::c_int
        * (*player).opts.delay_factor as ::core::ffi::c_int
        + *borg_cfg.offset(BORG_DELAY_FACTOR as ::core::ffi::c_int as isize)
            * *borg_cfg.offset(BORG_DELAY_FACTOR as ::core::ffi::c_int as isize);
    Term_xtra(TERM_XTRA_DELAY, msec);
    if borg.status.redraw {
        borg_note(format(
            b"#  Redrawing screen.\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        do_cmd_redraw();
        borg.status.redraw = false_0 != 0;
    }
    if borg.time.now - borg.time.level >= 10000 as borg_time {
        if !borg.goal.leaving {
            borg_note(b"# Leaving (boredom)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (boredom)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    borg_near_monster_type(
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
        {
            (*z_info).max_sight as ::core::ffi::c_int
        } else {
            12 as ::core::ffi::c_int
        },
    );
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && borg.goal.fleeing_to_town as ::core::ffi::c_int != 0
    {
        if borg_think_stair_scum() {
            return true_0 != 0;
        }
        if !borg.goal.leaving {
            borg_note(b"# Leaving (finish shopping)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (finish shopping)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    if borg_check_bounciness() {
        return true_0 != 0;
    }
    if borg_check_breeders() {
        return true_0 != 0;
    }
    if borg.avoidance as ::core::ffi::c_int
        != *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
    }
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
            < 30 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 15 as ::core::ffi::c_int)
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 5 as ::core::ffi::c_int
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if !borg.goal.less
            && b_j
                > *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 3 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
        {
            if !borg.goal.less {
                borg_note(format(
                    b"# Return to Stair (wandered too far.  Leash: %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        * 3 as ::core::ffi::c_int
                        + 14 as ::core::ffi::c_int,
                ));
                borg.goal.less = true_0 != 0;
            }
        } else if borg.goal.less as ::core::ffi::c_int != 0 && b_j < 3 as ::core::ffi::c_int {
            borg_note(b"# Close enough to Stair.\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.less = false_0 != 0;
            borg.goal.type_0 = 0 as int16_t;
        }
    }
    borg_notice(true_0 != 0);
    if *borg_cfg.offset(BORG_SELF_LUNAL as ::core::ffi::c_int as isize) != 0
        && *borg_cfg.offset(BORG_PLAYS_RISKY as ::core::ffi::c_int as isize) == 0
    {
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 15 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
            && *borg
                .trait_0
                .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                >= *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    + 15 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 1 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                > *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 3 as ::core::ffi::c_int
        {
            borg.lunal_mode = true_0 != 0;
            if borg.lunal_mode as ::core::ffi::c_int != 0
                && borg_think_dungeon_lunal() as ::core::ffi::c_int != 0
            {
                return true_0 != 0;
            }
        }
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 12 as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
        {
            borg.munchkin_mode = true_0 != 0;
            if borg_think_dungeon_munchkin() {
                return true_0 != 0;
            }
        }
        borg.munchkin_mode = false_0 != 0;
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 10 as ::core::ffi::c_int
        && !borg.goal.less
        && !(borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize),
        ) as *mut ::core::ffi::c_char)
            .is_null()
    {
        borg_note(
            b"# Needing to get back on correct depth\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.goal.less = true_0 != 0;
        if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    if borg_think_dungeon_light() {
        return true_0 != 0;
    }
    if borg.no_retreat as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        borg.no_retreat -= 1;
    }
    if borg.times_twitch as ::core::ffi::c_int > 20 as ::core::ffi::c_int {
        borg.goal.fleeing = true_0 != 0;
    }
    if borg_flow_old(GOAL_DIGGING) {
        return true_0 != 0;
    }
    if borg_caution() {
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            if borg_maintain_light() as ::core::ffi::c_uint
                == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return true_0 != 0;
            }
            if borg_wear_stuff() {
                return true_0 != 0;
            }
        }
        if borg_recover() {
            return true_0 != 0;
        }
        if (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
            && borg_drop_hole(false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_choose_shop() {
            if borg_flow_shop_entry(borg.goal.shop as ::core::ffi::c_int) {
                return true_0 != 0;
            }
        }
    }
    if !(borg_must_return_to_town() as *mut ::core::ffi::c_char).is_null() {
        if borg_leave_level(false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_play_magic(false_0 != 0) {
        return true_0 != 0;
    }
    if (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
        == TV_DIGGING as ::core::ffi::c_int
        && borg_wear_stuff() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if (*borg_items.offset((*z_info).pack_size as isize)).tval == 0
        && borg_wear_stuff() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_flow_kill_corridor() {
        return true_0 != 0;
    }
    if borg_attack(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_wear_stuff() {
        return true_0 != 0;
    }
    if borg_swap_rings() {
        return true_0 != 0;
    }
    if borg_wear_rings() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_wear_rings() {
        return true_0 != 0;
    }
    if borg_remove_stuff() {
        return true_0 != 0;
    }
    if borg_dump_quiver() {
        return true_0 != 0;
    }
    if borg_check_light() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_RECOVER) {
        return true_0 != 0;
    }
    if borg_recover() {
        return true_0 != 0;
    }
    if borg_flow_recover(50 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_perma_spell() {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && borg.no_rest_prep as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && !borg.temp.bless
        && !borg.temp.hero
        && !borg.temp.berserk
        && !borg.temp.fastcast
        && !flag_has(
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
        )
    {
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
        {
            let mut tmp_i: ::core::ffi::c_int = 0;
            let mut y_0: ::core::ffi::c_int = 0;
            let mut x_0: ::core::ffi::c_int = 0;
            tmp_i = 0 as ::core::ffi::c_int;
            while tmp_i < track_less.num as ::core::ffi::c_int {
                x_0 = *track_less.x.offset(tmp_i as isize);
                y_0 = *track_less.y.offset(tmp_i as isize);
                if !(borg.c.y != y_0 || borg.c.x != x_0) {
                    borg.goal.less = false_0 != 0;
                    if borg_danger(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ) == 0 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Resting on stair to gain Mana.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress(',' as i32 as keycode_t);
                        return true_0 != 0;
                    }
                }
                tmp_i += 1;
            }
        } else {
            let mut tmp_i_0: ::core::ffi::c_int = 0;
            let mut y_1: ::core::ffi::c_int = 0;
            let mut x_1: ::core::ffi::c_int = 0;
            tmp_i_0 = 0 as ::core::ffi::c_int;
            while tmp_i_0 < track_more.num as ::core::ffi::c_int {
                x_1 = *track_more.x.offset(tmp_i_0 as isize);
                y_1 = *track_more.y.offset(tmp_i_0 as isize);
                if !(borg.c.y != y_1 || borg.c.x != x_1) {
                    if borg_danger(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ) == 0 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Resting on town stair to gain Mana.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress(',' as i32 as keycode_t);
                        return true_0 != 0;
                    }
                }
                tmp_i_0 += 1;
            }
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
            borg_note(
                b"# Looking for stairs. Stair hugging.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 6 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) < 10 as ::core::ffi::c_int
        && borg_count_sell() < 5 as ::core::ffi::c_int
    {
        borg_note(
            b"# Nothing to sell in town (leaving).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.goal.leaving = true_0 != 0;
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg.goal.less {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Goal_less, Fleeing.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if borg.goal.fleeing as ::core::ffi::c_int != 0 && borg.goal.recalling == 0 {
        borg.stair_more = true_0 != 0;
        borg.stair_less = borg.stair_more;
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg.stair_less = false_0 != 0;
        }
        borg_note(
            b"# Fleeing and leaving the level. (Looking for any stair)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Fleeing.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) == 0
        && borg.near.morgoth as ::core::ffi::c_int != 0
        && !borg.morgoth_position
        && (*borg
            .trait_0
            .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
            >= 10 as ::core::ffi::c_int
            && (*borg
                .trait_0
                .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                == 0
                && *borg
                    .trait_0
                    .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                    == 0))
    {
        if borg_flow_old(GOAL_MISC) {
            return true_0 != 0;
        }
        if borg_flow_glyph() {
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_kill(true_0 != 0, 20 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 10 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 10 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_VAULT) {
        return true_0 != 0;
    }
    if borg_flow_kill_aim(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_use_things() {
        return true_0 != 0;
    }
    if borg_test_stuff() {
        return true_0 != 0;
    }
    if borg_enchanting() {
        return true_0 != 0;
    }
    if borg_recharging() {
        return true_0 != 0;
    }
    if borg_drop_junk() {
        return true_0 != 0;
    }
    if borg_drop_hole(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_drop_slow() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg.goal.leaving as ::core::ffi::c_int != 0
        && borg.goal.recalling == 0
        && borg.mon.unique == 0
        || *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 25 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                < 25000 as ::core::ffi::c_int
            && borg_count_sell() >= 13 as ::core::ffi::c_int
    {
        if borg.ready_morgoth == 0 as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_note(
                b"# Fleeing and leaving the level (Looking for Up Stair).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.stair_less = true_0 != 0;
        }
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        {
            borg.stair_more = true_0 != 0;
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.stair_less {
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Goal_Leaving.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 25 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                < 25000 as ::core::ffi::c_int
            && borg_count_sell() >= 13 as ::core::ffi::c_int
        {
            borg.stair_more = false_0 != 0;
        }
        if borg.stair_more {
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0 as ::core::ffi::c_int
        && (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 5 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        && !borg.stair_less
    {
        borg.stair_more = true_0 != 0;
        if borg_flow_old(GOAL_BORE) {
            return true_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 25 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                < 25000 as ::core::ffi::c_int
            && borg_count_sell() >= 13 as ::core::ffi::c_int
        {
            borg.stair_more = false_0 != 0;
        }
        if borg.stair_more as ::core::ffi::c_int != 0
            && borg_flow_stair_more(GOAL_BORE, true_0 != 0, false_0 != 0) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Powerdiving.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_MISC) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_DARK) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_XTRA) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_BORE) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_VAULT) {
        return true_0 != 0;
    }
    if borg.status.vault {
        if borg_flow_kill(
            false_0 != 0,
            (*z_info).max_range as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        ) {
            return true_0 != 0;
        }
        if borg_flow_take(false_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_vein(false_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_excavate_vault((*z_info).max_range as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        {
            return true_0 != 0;
        }
        if borg_flow_vault(35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_dark(true_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_flow_kill(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_dark(true_0 != 0) {
        return true_0 != 0;
    }
    if !(*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        < *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0)
    {
        if borg_leave_level(false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_flow_dark(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_choose_shop() {
        if borg_flow_shop_entry(borg.goal.shop as ::core::ffi::c_int) {
            return true_0 != 0;
        }
    }
    if !borg.goal.fleeing && borg_play_magic(true_0 != 0) as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_spastic(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_kill_direct(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_wear_recharge() {
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        < *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0
        && *borg_cfg.offset(BORG_PLAYS_RISKY as ::core::ffi::c_int as isize) == 0
    {
        if borg_money_scum() {
            return true_0 != 0;
        }
    } else if borg_leave_level(true_0 != 0) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_spastic(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_kill_direct(false_0 != 0) {
        return true_0 != 0;
    }
    if borg.goal.recalling != 0
        && borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        ) <= 0 as ::core::ffi::c_int
    {
        borg_note(b"# Waiting for Recall...\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('9' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && borg.time.now - borg.time.level > 800 as borg_time
        || borg.time.now - borg.time.level > 28000 as borg_time
    {
        borg.status.old_depth = 126 as int16_t;
    }
    borg.no_retreat = 5 as int16_t;
    if (borg.avoidance as ::core::ffi::c_int)
        < *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) * 2 as ::core::ffi::c_int
    {
        let mut done: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (1) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) * 2 as ::core::ffi::c_int,
        ));
        borg.avoidance = (*borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if borg_think_dungeon_brave() {
            done = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done {
            return true_0 != 0;
        }
    }
    borg.times_twitch += 1;
    if (borg.times_twitch as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        borg_note(b"# Considering Phase (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_spell(PHASE_DOOR) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_phase_door) as ::core::ffi::c_int != 0
                || borg_dimension_door(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(TELEPORT_SELF) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0
                || borg_shadow_shift(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0)
        {
            return true_0 != 0;
        }
    }
    borg.no_retreat = 10 as int16_t;
    if (borg.avoidance as ::core::ffi::c_int)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 4 as ::core::ffi::c_int
    {
        let mut done_0: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (2) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 4 as ::core::ffi::c_int,
        ));
        borg.avoidance = (*borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
            * 4 as ::core::ffi::c_int) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if borg_think_dungeon_brave() {
            done_0 = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done_0 {
            return true_0 != 0;
        }
    }
    if (borg.avoidance as ::core::ffi::c_int) < 30000 as ::core::ffi::c_int {
        let mut done_1: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (3) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            30000 as ::core::ffi::c_int,
        ));
        borg.avoidance = 30000 as int16_t;
        borg_danger_wipe = true_0 != 0;
        borg.mon.unique = 0 as ::core::ffi::c_uint;
        borg.mon.scary = false_0 != 0;
        borg.near.breeder = false_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        {
            borg.goal.rising = false_0 != 0;
        }
        borg.goal.ignoring = false_0 != 0;
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        track_glyph.num = 0 as int16_t;
        track_step.num = 0 as int16_t;
        track_door.num = 0 as int16_t;
        track_closed.num = 0 as int16_t;
        track_vein.num = 0 as int16_t;
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        if borg_think_dungeon_brave() {
            done_1 = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done_1 {
            return true_0 != 0;
        }
    }
    borg.times_twitch += 1;
    if (borg.times_twitch as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(TELEPORT_SELF) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0
                || borg_shadow_shift(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_use_staff(sv_staff_teleportation) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_teleport_level) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_level) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Teleport (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
    {
        if borg_recall() {
            borg_note(b"# Recalling (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    borg.mon.unique = 0 as ::core::ffi::c_uint;
    borg.mon.scary = false_0 != 0;
    borg.near.breeder = false_0 != 0;
    borg_takes_cnt = 0 as int16_t;
    borg_takes_nxt = 1 as int16_t;
    borg_kills_cnt = 0 as int16_t;
    borg_kills_nxt = 1 as int16_t;
    if borg_flow_kill_direct(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_twitchy() {
        return true_0 != 0;
    }
    return false_0 != 0;
}
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_MISC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const GOAL_DARK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const GOAL_XTRA: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GOAL_BORE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GOAL_FLEE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const GOAL_VAULT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GOAL_RECOVER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BORG_TIME_MAX: ::core::ffi::c_int = INT32_MAX;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
