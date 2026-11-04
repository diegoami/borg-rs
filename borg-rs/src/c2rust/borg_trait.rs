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
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    static mut k_info: *mut object_kind;
    static mut player: *mut player;
    fn player_flags(p: *mut player, f: *mut bitflag);
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    fn borg_item_weight(item: *mut borg_item) -> int16_t;
    static mut weapon_swap_free_act: uint8_t;
    static mut weapon_swap_resist_acid: uint8_t;
    static mut weapon_swap_resist_elec: uint8_t;
    static mut weapon_swap_resist_fire: uint8_t;
    static mut weapon_swap_resist_cold: uint8_t;
    static mut weapon_swap_resist_pois: uint8_t;
    static mut weapon_swap_resist_conf: uint8_t;
    static mut weapon_swap_resist_sound: uint8_t;
    static mut weapon_swap_resist_light: uint8_t;
    static mut weapon_swap_resist_dark: uint8_t;
    static mut weapon_swap_resist_chaos: uint8_t;
    static mut weapon_swap_resist_disen: uint8_t;
    static mut weapon_swap_resist_shard: uint8_t;
    static mut weapon_swap_resist_nexus: uint8_t;
    static mut weapon_swap_resist_blind: uint8_t;
    static mut weapon_swap_resist_neth: uint8_t;
    static mut weapon_swap_resist_fear: uint8_t;
    static mut weapon_swap_hold_life: uint8_t;
    static mut armour_swap_free_act: uint8_t;
    static mut armour_swap_resist_acid: uint8_t;
    static mut armour_swap_resist_elec: uint8_t;
    static mut armour_swap_resist_fire: uint8_t;
    static mut armour_swap_resist_cold: uint8_t;
    static mut armour_swap_resist_pois: uint8_t;
    static mut armour_swap_resist_conf: uint8_t;
    static mut armour_swap_resist_sound: uint8_t;
    static mut armour_swap_resist_light: uint8_t;
    static mut armour_swap_resist_dark: uint8_t;
    static mut armour_swap_resist_chaos: uint8_t;
    static mut armour_swap_resist_disen: uint8_t;
    static mut armour_swap_resist_shard: uint8_t;
    static mut armour_swap_resist_nexus: uint8_t;
    static mut armour_swap_resist_blind: uint8_t;
    static mut armour_swap_resist_neth: uint8_t;
    static mut armour_swap_resist_fear: uint8_t;
    static mut armour_swap_hold_life: uint8_t;
    fn borg_notice_weapon_swap();
    fn borg_notice_armour_swap();
    fn obj_kind_can_browse(kind: *const object_kind) -> bool;
    static adj_str_blow: [::core::ffi::c_int; 38];
    static adj_str_hold: [::core::ffi::c_int; 38];
    static mut PY_FOOD_MAX: ::core::ffi::c_int;
    static mut PY_FOOD_FULL: ::core::ffi::c_int;
    static mut PY_FOOD_HUNGRY: ::core::ffi::c_int;
    static mut PY_FOOD_WEAK: ::core::ffi::c_int;
    fn modify_stat_value(value: ::core::ffi::c_int, amount: ::core::ffi::c_int) -> int16_t;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_race_death: *mut int16_t;
    static mut act_staff_holy: ::core::ffi::c_int;
    static mut act_staff_magi: ::core::ffi::c_int;
    static mut act_stone_to_mud: ::core::ffi::c_int;
    static mut act_firebrand: ::core::ffi::c_int;
    static mut act_restore_life: ::core::ffi::c_int;
    static mut act_haste2: ::core::ffi::c_int;
    static mut act_haste1: ::core::ffi::c_int;
    static mut act_haste: ::core::ffi::c_int;
    static mut act_clairvoyance: ::core::ffi::c_int;
    static mut act_illumination: ::core::ffi::c_int;
    static mut act_recall: ::core::ffi::c_int;
    static mut act_protevil: ::core::ffi::c_int;
    static mut act_recharge: ::core::ffi::c_int;
    static mut act_glyph: ::core::ffi::c_int;
    static mut act_mapping: ::core::ffi::c_int;
    static mut act_tele_long: ::core::ffi::c_int;
    static mut act_tele_phase: ::core::ffi::c_int;
    static mut act_light: ::core::ffi::c_int;
    static mut act_enchant_armor2: ::core::ffi::c_int;
    static mut act_enchant_armor: ::core::ffi::c_int;
    static mut act_enchant_weapon: ::core::ffi::c_int;
    static mut act_enchant_todam: ::core::ffi::c_int;
    static mut act_enchant_tohit: ::core::ffi::c_int;
    static mut act_detect_all: ::core::ffi::c_int;
    static mut act_enlightenment: ::core::ffi::c_int;
    static mut act_restore_st_lev: ::core::ffi::c_int;
    static mut act_restore_exp: ::core::ffi::c_int;
    static mut act_heal3: ::core::ffi::c_int;
    static mut act_heal2: ::core::ffi::c_int;
    static mut act_heal1: ::core::ffi::c_int;
    static mut act_cure_nonorlybig: ::core::ffi::c_int;
    static mut act_cure_full2: ::core::ffi::c_int;
    static mut act_cure_full: ::core::ffi::c_int;
    static mut act_cure_critical: ::core::ffi::c_int;
    static mut act_cure_serious: ::core::ffi::c_int;
    static mut act_cure_light: ::core::ffi::c_int;
    fn borg_obj_has_effect(
        kind: uint32_t,
        index: ::core::ffi::c_int,
        subtype: ::core::ffi::c_int,
    ) -> bool;
    fn borg_item_note_needs_id(item: *const borg_item) -> bool;
    fn borg_equips_ring(ring_sval: ::core::ffi::c_int) -> bool;
    fn borg_equips_item(activation: ::core::ffi::c_int, check_charge: bool) -> bool;
    fn borg_activate_failure(
        tval: ::core::ffi::c_int,
        sval: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut sv_food_apple: ::core::ffi::c_int;
    static mut sv_food_ration: ::core::ffi::c_int;
    static mut sv_food_slime_mold: ::core::ffi::c_int;
    static mut sv_food_draught: ::core::ffi::c_int;
    static mut sv_food_pint: ::core::ffi::c_int;
    static mut sv_food_sip: ::core::ffi::c_int;
    static mut sv_food_waybread: ::core::ffi::c_int;
    static mut sv_food_honey_cake: ::core::ffi::c_int;
    static mut sv_food_slice: ::core::ffi::c_int;
    static mut sv_food_handful: ::core::ffi::c_int;
    static mut sv_mush_second_sight: ::core::ffi::c_int;
    static mut sv_mush_restoring: ::core::ffi::c_int;
    static mut sv_mush_emergency: ::core::ffi::c_int;
    static mut sv_mush_terror: ::core::ffi::c_int;
    static mut sv_mush_stoneskin: ::core::ffi::c_int;
    static mut sv_mush_debility: ::core::ffi::c_int;
    static mut sv_mush_sprinting: ::core::ffi::c_int;
    static mut sv_mush_cure_mind: ::core::ffi::c_int;
    static mut sv_mush_purging: ::core::ffi::c_int;
    static mut sv_light_lantern: ::core::ffi::c_int;
    static mut sv_light_torch: ::core::ffi::c_int;
    static mut kv_flask_oil: ::core::ffi::c_int;
    static mut sv_potion_cure_critical: ::core::ffi::c_int;
    static mut sv_potion_cure_serious: ::core::ffi::c_int;
    static mut sv_potion_cure_light: ::core::ffi::c_int;
    static mut sv_potion_healing: ::core::ffi::c_int;
    static mut kv_potion_healing: ::core::ffi::c_int;
    static mut sv_potion_star_healing: ::core::ffi::c_int;
    static mut sv_potion_life: ::core::ffi::c_int;
    static mut sv_potion_cure_poison: ::core::ffi::c_int;
    static mut sv_potion_resist_heat: ::core::ffi::c_int;
    static mut sv_potion_resist_cold: ::core::ffi::c_int;
    static mut sv_potion_resist_pois: ::core::ffi::c_int;
    static mut sv_potion_inc_str: ::core::ffi::c_int;
    static mut sv_potion_inc_int: ::core::ffi::c_int;
    static mut sv_potion_inc_wis: ::core::ffi::c_int;
    static mut sv_potion_inc_dex: ::core::ffi::c_int;
    static mut sv_potion_inc_con: ::core::ffi::c_int;
    static mut sv_potion_inc_all: ::core::ffi::c_int;
    static mut sv_potion_restore_life: ::core::ffi::c_int;
    static mut sv_potion_speed: ::core::ffi::c_int;
    static mut sv_scroll_identify: ::core::ffi::c_int;
    static mut sv_scroll_phase_door: ::core::ffi::c_int;
    static mut sv_scroll_teleport: ::core::ffi::c_int;
    static mut sv_scroll_word_of_recall: ::core::ffi::c_int;
    static mut sv_scroll_enchant_armor: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_hit: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_dam: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_weapon: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_armor: ::core::ffi::c_int;
    static mut sv_scroll_protection_from_evil: ::core::ffi::c_int;
    static mut sv_scroll_rune_of_protection: ::core::ffi::c_int;
    static mut sv_scroll_teleport_level: ::core::ffi::c_int;
    static mut sv_scroll_recharging: ::core::ffi::c_int;
    static mut sv_scroll_mass_banishment: ::core::ffi::c_int;
    static mut sv_ring_digging: ::core::ffi::c_int;
    static mut sv_rod_recall: ::core::ffi::c_int;
    static mut sv_rod_detection: ::core::ffi::c_int;
    static mut sv_rod_illumination: ::core::ffi::c_int;
    static mut sv_rod_speed: ::core::ffi::c_int;
    static mut sv_rod_mapping: ::core::ffi::c_int;
    static mut sv_rod_healing: ::core::ffi::c_int;
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
    static mut sv_staff_teleportation: ::core::ffi::c_int;
    static mut sv_staff_destruction: ::core::ffi::c_int;
    static mut sv_staff_speed: ::core::ffi::c_int;
    static mut sv_staff_healing: ::core::ffi::c_int;
    static mut sv_staff_the_magi: ::core::ffi::c_int;
    static mut sv_staff_power: ::core::ffi::c_int;
    static mut sv_staff_holiness: ::core::ffi::c_int;
    static mut kv_staff_holiness: ::core::ffi::c_int;
    static mut sv_wand_teleport_away: ::core::ffi::c_int;
    static mut sv_wand_stinking_cloud: ::core::ffi::c_int;
    static mut sv_wand_magic_missile: ::core::ffi::c_int;
    static mut sv_wand_annihilation: ::core::ffi::c_int;
    static mut sv_sling: ::core::ffi::c_int;
    static mut sv_short_bow: ::core::ffi::c_int;
    static mut sv_long_bow: ::core::ffi::c_int;
    static mut sv_light_xbow: ::core::ffi::c_int;
    static mut sv_heavy_xbow: ::core::ffi::c_int;
    fn borg_is_ammo(tval: ::core::ffi::c_int) -> bool;
    fn borg_spell_stat() -> ::core::ffi::c_int;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    fn borg_get_book_num(sval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn borg_spell_legal_fail(spell: borg_spells, allow_fail: ::core::ffi::c_int) -> bool;
    fn borg_cheat_spells();
    static mut borg_do_spell: bool;
    static mut num_heal_true: int16_t;
    static mut num_ezheal_true: int16_t;
    static mut num_life_true: int16_t;
    static mut num_speed: int16_t;
    fn borg_notice_home(in_item: *mut borg_item, no_items: bool);
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
pub const OPT_MAX: C2RustUnnamed_3 = 46;
pub const OPT_birth_percent_damage: C2RustUnnamed_3 = 45;
pub const OPT_birth_levels_persist: C2RustUnnamed_3 = 44;
pub const OPT_birth_know_flavors: C2RustUnnamed_3 = 43;
pub const OPT_birth_know_runes: C2RustUnnamed_3 = 42;
pub const OPT_birth_ai_learn: C2RustUnnamed_3 = 41;
pub const OPT_birth_start_kit: C2RustUnnamed_3 = 40;
pub const OPT_birth_no_selling: C2RustUnnamed_3 = 39;
pub const OPT_birth_feelings: C2RustUnnamed_3 = 38;
pub const OPT_birth_lose_arts: C2RustUnnamed_3 = 37;
pub const OPT_birth_stacking: C2RustUnnamed_3 = 36;
pub const OPT_birth_no_artifacts: C2RustUnnamed_3 = 35;
pub const OPT_birth_no_recall: C2RustUnnamed_3 = 34;
pub const OPT_birth_force_descend: C2RustUnnamed_3 = 33;
pub const OPT_birth_connect_stairs: C2RustUnnamed_3 = 32;
pub const OPT_birth_randarts: C2RustUnnamed_3 = 31;
pub const OPT_score_live: C2RustUnnamed_3 = 30;
pub const OPT_cheat_live: C2RustUnnamed_3 = 29;
pub const OPT_score_xtra: C2RustUnnamed_3 = 28;
pub const OPT_cheat_xtra: C2RustUnnamed_3 = 27;
pub const OPT_score_room: C2RustUnnamed_3 = 26;
pub const OPT_cheat_room: C2RustUnnamed_3 = 25;
pub const OPT_score_hear: C2RustUnnamed_3 = 24;
pub const OPT_cheat_hear: C2RustUnnamed_3 = 23;
pub const OPT_effective_speed: C2RustUnnamed_3 = 22;
pub const OPT_notify_recharge: C2RustUnnamed_3 = 21;
pub const OPT_mouse_movement: C2RustUnnamed_3 = 20;
pub const OPT_hp_changes_color: C2RustUnnamed_3 = 19;
pub const OPT_auto_more: C2RustUnnamed_3 = 18;
pub const OPT_purple_uniques: C2RustUnnamed_3 = 17;
pub const OPT_center_player: C2RustUnnamed_3 = 16;
pub const OPT_animate_flicker: C2RustUnnamed_3 = 15;
pub const OPT_view_yellow_light: C2RustUnnamed_3 = 14;
pub const OPT_hybrid_walls: C2RustUnnamed_3 = 13;
pub const OPT_solid_walls: C2RustUnnamed_3 = 12;
pub const OPT_disturb_near: C2RustUnnamed_3 = 11;
pub const OPT_highlight_player: C2RustUnnamed_3 = 10;
pub const OPT_show_target: C2RustUnnamed_3 = 9;
pub const OPT_show_flavors: C2RustUnnamed_3 = 8;
pub const OPT_pickup_inven: C2RustUnnamed_3 = 7;
pub const OPT_pickup_always: C2RustUnnamed_3 = 6;
pub const OPT_use_old_target: C2RustUnnamed_3 = 5;
pub const OPT_show_damage: C2RustUnnamed_3 = 4;
pub const OPT_use_sound: C2RustUnnamed_3 = 3;
pub const OPT_autoexplore_commands: C2RustUnnamed_3 = 2;
pub const OPT_rogue_like_commands: C2RustUnnamed_3 = 1;
pub const OPT_none: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const SKILL_MAX: C2RustUnnamed_6 = 10;
pub const SKILL_DIGGING: C2RustUnnamed_6 = 9;
pub const SKILL_TO_HIT_THROW: C2RustUnnamed_6 = 8;
pub const SKILL_TO_HIT_BOW: C2RustUnnamed_6 = 7;
pub const SKILL_TO_HIT_MELEE: C2RustUnnamed_6 = 6;
pub const SKILL_STEALTH: C2RustUnnamed_6 = 5;
pub const SKILL_SEARCH: C2RustUnnamed_6 = 4;
pub const SKILL_SAVE: C2RustUnnamed_6 = 3;
pub const SKILL_DEVICE: C2RustUnnamed_6 = 2;
pub const SKILL_DISARM_MAGIC: C2RustUnnamed_6 = 1;
pub const SKILL_DISARM_PHYS: C2RustUnnamed_6 = 0;
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const RF_MAX: C2RustUnnamed_7 = 85;
pub const RF_NO_SLOW: C2RustUnnamed_7 = 84;
pub const RF_NO_HOLD: C2RustUnnamed_7 = 83;
pub const RF_NO_SLEEP: C2RustUnnamed_7 = 82;
pub const RF_NO_CONF: C2RustUnnamed_7 = 81;
pub const RF_NO_STUN: C2RustUnnamed_7 = 80;
pub const RF_NO_FEAR: C2RustUnnamed_7 = 79;
pub const RF_IM_DISEN: C2RustUnnamed_7 = 78;
pub const RF_IM_NEXUS: C2RustUnnamed_7 = 77;
pub const RF_IM_PLASMA: C2RustUnnamed_7 = 76;
pub const RF_IM_WATER: C2RustUnnamed_7 = 75;
pub const RF_IM_NETHER: C2RustUnnamed_7 = 74;
pub const RF_IM_POIS: C2RustUnnamed_7 = 73;
pub const RF_IM_COLD: C2RustUnnamed_7 = 72;
pub const RF_IM_FIRE: C2RustUnnamed_7 = 71;
pub const RF_IM_ELEC: C2RustUnnamed_7 = 70;
pub const RF_IM_ACID: C2RustUnnamed_7 = 69;
pub const RF_HURT_COLD: C2RustUnnamed_7 = 68;
pub const RF_HURT_FIRE: C2RustUnnamed_7 = 67;
pub const RF_HURT_ROCK: C2RustUnnamed_7 = 66;
pub const RF_HURT_LIGHT: C2RustUnnamed_7 = 65;
pub const RF_METAL: C2RustUnnamed_7 = 64;
pub const RF_NONLIVING: C2RustUnnamed_7 = 63;
pub const RF_UNDEAD: C2RustUnnamed_7 = 62;
pub const RF_EVIL: C2RustUnnamed_7 = 61;
pub const RF_ANIMAL: C2RustUnnamed_7 = 60;
pub const RF_DEMON: C2RustUnnamed_7 = 59;
pub const RF_DRAGON: C2RustUnnamed_7 = 58;
pub const RF_GIANT: C2RustUnnamed_7 = 57;
pub const RF_TROLL: C2RustUnnamed_7 = 56;
pub const RF_ORC: C2RustUnnamed_7 = 55;
pub const RF_PASS_WEB: C2RustUnnamed_7 = 54;
pub const RF_CLEAR_WEB: C2RustUnnamed_7 = 53;
pub const RF_KILL_ITEM: C2RustUnnamed_7 = 52;
pub const RF_TAKE_ITEM: C2RustUnnamed_7 = 51;
pub const RF_KILL_BODY: C2RustUnnamed_7 = 50;
pub const RF_MOVE_BODY: C2RustUnnamed_7 = 49;
pub const RF_SMASH_WALL: C2RustUnnamed_7 = 48;
pub const RF_KILL_WALL: C2RustUnnamed_7 = 47;
pub const RF_PASS_WALL: C2RustUnnamed_7 = 46;
pub const RF_BASH_DOOR: C2RustUnnamed_7 = 45;
pub const RF_OPEN_DOOR: C2RustUnnamed_7 = 44;
pub const RF_WEIRD_MIND: C2RustUnnamed_7 = 43;
pub const RF_EMPTY_MIND: C2RustUnnamed_7 = 42;
pub const RF_COLD_BLOOD: C2RustUnnamed_7 = 41;
pub const RF_INVISIBLE: C2RustUnnamed_7 = 40;
pub const RF_DROP_20: C2RustUnnamed_7 = 39;
pub const RF_DROP_GREAT: C2RustUnnamed_7 = 38;
pub const RF_DROP_GOOD: C2RustUnnamed_7 = 37;
pub const RF_DROP_4: C2RustUnnamed_7 = 36;
pub const RF_DROP_3: C2RustUnnamed_7 = 35;
pub const RF_DROP_2: C2RustUnnamed_7 = 34;
pub const RF_DROP_1: C2RustUnnamed_7 = 33;
pub const RF_DROP_60: C2RustUnnamed_7 = 32;
pub const RF_DROP_40: C2RustUnnamed_7 = 31;
pub const RF_ONLY_ITEM: C2RustUnnamed_7 = 30;
pub const RF_ONLY_GOLD: C2RustUnnamed_7 = 29;
pub const RF_POWERFUL: C2RustUnnamed_7 = 28;
pub const RF_SPIRIT: C2RustUnnamed_7 = 27;
pub const RF_SMART: C2RustUnnamed_7 = 26;
pub const RF_STUPID: C2RustUnnamed_7 = 25;
pub const RF_MIMIC_INV: C2RustUnnamed_7 = 24;
pub const RF_RAND_50: C2RustUnnamed_7 = 23;
pub const RF_RAND_25: C2RustUnnamed_7 = 22;
pub const RF_NEVER_MOVE: C2RustUnnamed_7 = 21;
pub const RF_NEVER_BLOW: C2RustUnnamed_7 = 20;
pub const RF_FRIGHTENED: C2RustUnnamed_7 = 19;
pub const RF_REGENERATE: C2RustUnnamed_7 = 18;
pub const RF_MULTIPLY: C2RustUnnamed_7 = 17;
pub const RF_UNAWARE: C2RustUnnamed_7 = 16;
pub const RF_SEASONAL: C2RustUnnamed_7 = 15;
pub const RF_FORCE_EXTRA: C2RustUnnamed_7 = 14;
pub const RF_FORCE_SLEEP: C2RustUnnamed_7 = 13;
pub const RF_FORCE_DEPTH: C2RustUnnamed_7 = 12;
pub const RF_ATTR_FLICKER: C2RustUnnamed_7 = 11;
pub const RF_ATTR_MULTI: C2RustUnnamed_7 = 10;
pub const RF_ATTR_CLEAR: C2RustUnnamed_7 = 9;
pub const RF_ATTR_RAND: C2RustUnnamed_7 = 8;
pub const RF_CHAR_CLEAR: C2RustUnnamed_7 = 7;
pub const RF_NAME_COMMA: C2RustUnnamed_7 = 6;
pub const RF_GROUP_AI: C2RustUnnamed_7 = 5;
pub const RF_FEMALE: C2RustUnnamed_7 = 4;
pub const RF_MALE: C2RustUnnamed_7 = 3;
pub const RF_QUESTOR: C2RustUnnamed_7 = 2;
pub const RF_UNIQUE: C2RustUnnamed_7 = 1;
pub const RF_NONE: C2RustUnnamed_7 = 0;
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
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const BORG_CURSE_MAX: C2RustUnnamed_8 = 28;
pub const BORG_CURSE_AIR_SWING: C2RustUnnamed_8 = 27;
pub const BORG_CURSE_STEELSKIN: C2RustUnnamed_8 = 26;
pub const BORG_CURSE_CHILLED_TO_THE_BONE: C2RustUnnamed_8 = 25;
pub const BORG_CURSE_BURNING_UP: C2RustUnnamed_8 = 24;
pub const BORG_CURSE_TREACHEROUS_WEAPON: C2RustUnnamed_8 = 23;
pub const BORG_CURSE_ANTI_TELEPORTATION: C2RustUnnamed_8 = 22;
pub const BORG_CURSE_STONE: C2RustUnnamed_8 = 21;
pub const BORG_CURSE_COWARDICE: C2RustUnnamed_8 = 20;
pub const BORG_CURSE_IMPAIR_HITPOINT_RECOVERY: C2RustUnnamed_8 = 19;
pub const BORG_CURSE_IMPAIR_MANA_RECOVERY: C2RustUnnamed_8 = 18;
pub const BORG_CURSE_UNDEAD_SUMMON: C2RustUnnamed_8 = 17;
pub const BORG_CURSE_DEMON_SUMMON: C2RustUnnamed_8 = 16;
pub const BORG_CURSE_DRAGON_SUMMON: C2RustUnnamed_8 = 15;
pub const BORG_CURSE_PARALYSIS: C2RustUnnamed_8 = 14;
pub const BORG_CURSE_HALLUCINATION: C2RustUnnamed_8 = 13;
pub const BORG_CURSE_SIREN: C2RustUnnamed_8 = 12;
pub const BORG_CURSE_POISON: C2RustUnnamed_8 = 11;
pub const BORG_CURSE_ANNOYANCE: C2RustUnnamed_8 = 10;
pub const BORG_CURSE_SLOWNESS: C2RustUnnamed_8 = 9;
pub const BORG_CURSE_CLUMSINESS: C2RustUnnamed_8 = 8;
pub const BORG_CURSE_WEAKNESS: C2RustUnnamed_8 = 7;
pub const BORG_CURSE_IRRITATION: C2RustUnnamed_8 = 6;
pub const BORG_CURSE_ENVELOPING: C2RustUnnamed_8 = 5;
pub const BORG_CURSE_SICKLINESS: C2RustUnnamed_8 = 4;
pub const BORG_CURSE_DULLNESS: C2RustUnnamed_8 = 3;
pub const BORG_CURSE_TELEPORTATION: C2RustUnnamed_8 = 2;
pub const BORG_CURSE_VULNERABILITY: C2RustUnnamed_8 = 1;
pub const BORG_CURSE_UNKNOWN: C2RustUnnamed_8 = 0;
pub type borg_item_pos = ::core::ffi::c_uint;
pub const BORG_QUILL: borg_item_pos = 4;
pub const BORG_EQUIP: borg_item_pos = 2;
pub const BORG_INVEN: borg_item_pos = 1;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_9 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_9 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_9 = 263;
pub const BI_EMPTY: C2RustUnnamed_9 = 262;
pub const BI_CARRY: C2RustUnnamed_9 = 261;
pub const BI_WEIGHT: C2RustUnnamed_9 = 260;
pub const BI_DINV: C2RustUnnamed_9 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_9 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_9 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_9 = 256;
pub const BI_ADIGGER: C2RustUnnamed_9 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_9 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_9 = 253;
pub const BI_AROD2: C2RustUnnamed_9 = 252;
pub const BI_AROD1: C2RustUnnamed_9 = 251;
pub const BI_ASHROOM: C2RustUnnamed_9 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_9 = 249;
pub const BI_AHWORD: C2RustUnnamed_9 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_9 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_9 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_9 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_9 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_9 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_9 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_9 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_9 = 240;
pub const BI_ABRAND: C2RustUnnamed_9 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_9 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_9 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_9 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_9 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_9 = 234;
pub const BI_ACLW: C2RustUnnamed_9 = 233;
pub const BI_ACSW: C2RustUnnamed_9 = 232;
pub const BI_ACCW: C2RustUnnamed_9 = 231;
pub const BI_AGLYPH: C2RustUnnamed_9 = 230;
pub const BI_APFE: C2RustUnnamed_9 = 229;
pub const BI_ALITE: C2RustUnnamed_9 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_9 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_9 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_9 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_9 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_9 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_9 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_9 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_9 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_9 = 219;
pub const BI_ASPEED: C2RustUnnamed_9 = 218;
pub const BI_AID: C2RustUnnamed_9 = 217;
pub const BI_ALIFE: C2RustUnnamed_9 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_9 = 215;
pub const BI_AHEAL: C2RustUnnamed_9 = 214;
pub const BI_AFUEL: C2RustUnnamed_9 = 213;
pub const BI_AESCAPE: C2RustUnnamed_9 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_9 = 211;
pub const BI_APHASE: C2RustUnnamed_9 = 210;
pub const BI_WB_POIS: C2RustUnnamed_9 = 209;
pub const BI_WB_COLD: C2RustUnnamed_9 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_9 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_9 = 206;
pub const BI_WB_ACID: C2RustUnnamed_9 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_9 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_9 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_9 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_9 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_9 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_9 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_9 = 198;
pub const BI_WS_ORC: C2RustUnnamed_9 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_9 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_9 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_9 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_9 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_9 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_9 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_9 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_9 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_9 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_9 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_9 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_9 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_9 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_9 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_9 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_9 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_9 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_9 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_9 = 178;
pub const BI_CRSSICK: C2RustUnnamed_9 = 177;
pub const BI_CRSDULL: C2RustUnnamed_9 = 176;
pub const BI_CRSVULN: C2RustUnnamed_9 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_9 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_9 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_9 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_9 = 171;
pub const BI_CRSSUND: C2RustUnnamed_9 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_9 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_9 = 168;
pub const BI_CRSPARA: C2RustUnnamed_9 = 167;
pub const BI_CRSHALU: C2RustUnnamed_9 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_9 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_9 = 164;
pub const BI_CRSTELE: C2RustUnnamed_9 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_9 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_9 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_9 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_9 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_9 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_9 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_9 = 156;
pub const BI_AMISSILES: C2RustUnnamed_9 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_9 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_9 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_9 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_9 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_9 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_9 = 149;
pub const BI_SHOTS: C2RustUnnamed_9 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_9 = 147;
pub const BI_BLOWS: C2RustUnnamed_9 = 146;
pub const BI_BART: C2RustUnnamed_9 = 145;
pub const BI_SLING: C2RustUnnamed_9 = 144;
pub const BI_BTODAM: C2RustUnnamed_9 = 143;
pub const BI_BTOHIT: C2RustUnnamed_9 = 142;
pub const BI_BID: C2RustUnnamed_9 = 141;
pub const BI_WDS: C2RustUnnamed_9 = 140;
pub const BI_WDD: C2RustUnnamed_9 = 139;
pub const BI_WID: C2RustUnnamed_9 = 138;
pub const BI_WTODAM: C2RustUnnamed_9 = 137;
pub const BI_WTOHIT: C2RustUnnamed_9 = 136;
pub const BI_TODAM: C2RustUnnamed_9 = 135;
pub const BI_TOHIT: C2RustUnnamed_9 = 134;
pub const BI_ARMOR: C2RustUnnamed_9 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_9 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_9 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_9 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_9 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_9 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_9 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_9 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_9 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_9 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_9 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_9 = 122;
pub const BI_ISFORGET: C2RustUnnamed_9 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_9 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_9 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_9 = 118;
pub const BI_ISSTUN: C2RustUnnamed_9 = 117;
pub const BI_ISCUT: C2RustUnnamed_9 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_9 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_9 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_9 = 113;
pub const BI_ISBLIND: C2RustUnnamed_9 = 112;
pub const BI_ISGORGED: C2RustUnnamed_9 = 111;
pub const BI_ISFULL: C2RustUnnamed_9 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_9 = 109;
pub const BI_ISWEAK: C2RustUnnamed_9 = 108;
pub const BI_KING: C2RustUnnamed_9 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_9 = 106;
pub const BI_CDEPTH: C2RustUnnamed_9 = 105;
pub const BI_SFRACT: C2RustUnnamed_9 = 104;
pub const BI_SHLIFE: C2RustUnnamed_9 = 103;
pub const BI_SRDIS: C2RustUnnamed_9 = 102;
pub const BI_SRKAOS: C2RustUnnamed_9 = 101;
pub const BI_SRNTHR: C2RustUnnamed_9 = 100;
pub const BI_SRNXUS: C2RustUnnamed_9 = 99;
pub const BI_SRSHRD: C2RustUnnamed_9 = 98;
pub const BI_SRSND: C2RustUnnamed_9 = 97;
pub const BI_SRCONF: C2RustUnnamed_9 = 96;
pub const BI_SRBLIND: C2RustUnnamed_9 = 95;
pub const BI_SRDARK: C2RustUnnamed_9 = 94;
pub const BI_SRLITE: C2RustUnnamed_9 = 93;
pub const BI_SRFEAR: C2RustUnnamed_9 = 92;
pub const BI_SRPOIS: C2RustUnnamed_9 = 91;
pub const BI_SRACID: C2RustUnnamed_9 = 90;
pub const BI_SRELEC: C2RustUnnamed_9 = 89;
pub const BI_SRCOLD: C2RustUnnamed_9 = 88;
pub const BI_SRFIRE: C2RustUnnamed_9 = 87;
pub const BI_FRACT: C2RustUnnamed_9 = 86;
pub const BI_HLIFE: C2RustUnnamed_9 = 85;
pub const BI_RDIS: C2RustUnnamed_9 = 84;
pub const BI_RKAOS: C2RustUnnamed_9 = 83;
pub const BI_RNTHR: C2RustUnnamed_9 = 82;
pub const BI_RNXUS: C2RustUnnamed_9 = 81;
pub const BI_RSHRD: C2RustUnnamed_9 = 80;
pub const BI_RSND: C2RustUnnamed_9 = 79;
pub const BI_RCONF: C2RustUnnamed_9 = 78;
pub const BI_RBLIND: C2RustUnnamed_9 = 77;
pub const BI_RDARK: C2RustUnnamed_9 = 76;
pub const BI_RLITE: C2RustUnnamed_9 = 75;
pub const BI_RFEAR: C2RustUnnamed_9 = 74;
pub const BI_RPOIS: C2RustUnnamed_9 = 73;
pub const BI_RACID: C2RustUnnamed_9 = 72;
pub const BI_RELEC: C2RustUnnamed_9 = 71;
pub const BI_RCOLD: C2RustUnnamed_9 = 70;
pub const BI_RFIRE: C2RustUnnamed_9 = 69;
pub const BI_IPOIS: C2RustUnnamed_9 = 68;
pub const BI_IELEC: C2RustUnnamed_9 = 67;
pub const BI_ICOLD: C2RustUnnamed_9 = 66;
pub const BI_IACID: C2RustUnnamed_9 = 65;
pub const BI_IFIRE: C2RustUnnamed_9 = 64;
pub const BI_DIG: C2RustUnnamed_9 = 63;
pub const BI_THT: C2RustUnnamed_9 = 62;
pub const BI_THB: C2RustUnnamed_9 = 61;
pub const BI_THN: C2RustUnnamed_9 = 60;
pub const BI_SRCH: C2RustUnnamed_9 = 59;
pub const BI_STL: C2RustUnnamed_9 = 58;
pub const BI_SAV: C2RustUnnamed_9 = 57;
pub const BI_DEV: C2RustUnnamed_9 = 56;
pub const BI_DISM: C2RustUnnamed_9 = 55;
pub const BI_DISP: C2RustUnnamed_9 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_9 = 53;
pub const BI_INFRA: C2RustUnnamed_9 = 52;
pub const BI_SINV: C2RustUnnamed_9 = 51;
pub const BI_REG: C2RustUnnamed_9 = 50;
pub const BI_FEATH: C2RustUnnamed_9 = 49;
pub const BI_SDIG: C2RustUnnamed_9 = 48;
pub const BI_DAM_RED: C2RustUnnamed_9 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_9 = 46;
pub const BI_GOLD: C2RustUnnamed_9 = 45;
pub const BI_SPEED: C2RustUnnamed_9 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_9 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_9 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_9 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_9 = 40;
pub const BI_FOOD: C2RustUnnamed_9 = 39;
pub const BI_RECALL: C2RustUnnamed_9 = 38;
pub const BI_ESP: C2RustUnnamed_9 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_9 = 36;
pub const BI_CLEVEL: C2RustUnnamed_9 = 35;
pub const BI_FAIL2: C2RustUnnamed_9 = 34;
pub const BI_FAIL1: C2RustUnnamed_9 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_9 = 32;
pub const BI_MAXSP: C2RustUnnamed_9 = 31;
pub const BI_CURSP: C2RustUnnamed_9 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_9 = 29;
pub const BI_MAXHP: C2RustUnnamed_9 = 28;
pub const BI_CURHP: C2RustUnnamed_9 = 27;
pub const BI_LIGHT: C2RustUnnamed_9 = 26;
pub const BI_CLASS: C2RustUnnamed_9 = 25;
pub const BI_SCON: C2RustUnnamed_9 = 24;
pub const BI_SDEX: C2RustUnnamed_9 = 23;
pub const BI_SWIS: C2RustUnnamed_9 = 22;
pub const BI_SINT: C2RustUnnamed_9 = 21;
pub const BI_SSTR: C2RustUnnamed_9 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_9 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_9 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_9 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_9 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_9 = 15;
pub const BI_CCON: C2RustUnnamed_9 = 14;
pub const BI_CDEX: C2RustUnnamed_9 = 13;
pub const BI_CWIS: C2RustUnnamed_9 = 12;
pub const BI_CINT: C2RustUnnamed_9 = 11;
pub const BI_CSTR: C2RustUnnamed_9 = 10;
pub const BI_ACON: C2RustUnnamed_9 = 9;
pub const BI_ADEX: C2RustUnnamed_9 = 8;
pub const BI_AWIS: C2RustUnnamed_9 = 7;
pub const BI_AINT: C2RustUnnamed_9 = 6;
pub const BI_ASTR: C2RustUnnamed_9 = 5;
pub const BI_CON: C2RustUnnamed_9 = 4;
pub const BI_DEX: C2RustUnnamed_9 = 3;
pub const BI_WIS: C2RustUnnamed_9 = 2;
pub const BI_INT: C2RustUnnamed_9 = 1;
pub const BI_STR: C2RustUnnamed_9 = 0;
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
    pub temp: C2RustUnnamed_17,
    pub status: C2RustUnnamed_16,
    pub time: C2RustUnnamed_15,
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
    pub goal: C2RustUnnamed_14,
    pub init_save: C2RustUnnamed_13,
    pub mon: C2RustUnnamed_12,
    pub id: C2RustUnnamed_11,
    pub near: C2RustUnnamed_10,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub struct C2RustUnnamed_12 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub struct C2RustUnnamed_16 {
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
pub struct C2RustUnnamed_17 {
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
pub const BORG_USES_SWAPS: C2RustUnnamed_20 = 12;
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
pub const TMD_BLIND: C2RustUnnamed_19 = 2;
pub const EF_CURE: C2RustUnnamed_18 = 8;
pub const TMD_CONFUSED: C2RustUnnamed_19 = 4;
pub const TMD_POISONED: C2RustUnnamed_19 = 7;
pub const EF_NOURISH: C2RustUnnamed_18 = 6;
pub const EF_TIMED_INC: C2RustUnnamed_18 = 10;
pub const EF_CRUNCH: C2RustUnnamed_18 = 7;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_20 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_20 = 1;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_20 = 28;
pub const TMD_TERROR: C2RustUnnamed_19 = 28;
pub const TMD_SPRINT: C2RustUnnamed_19 = 29;
pub const TMD_FAST: C2RustUnnamed_19 = 0;
pub const TMD_AMNESIA: C2RustUnnamed_19 = 25;
pub const TMD_IMAGE: C2RustUnnamed_19 = 6;
pub const TMD_PARALYZED: C2RustUnnamed_19 = 3;
pub const TMD_STUN: C2RustUnnamed_19 = 9;
pub const TMD_CUT: C2RustUnnamed_19 = 8;
pub const TMD_AFRAID: C2RustUnnamed_19 = 5;
pub const TMD_FOOD: C2RustUnnamed_19 = 10;
pub const TMD_SINVIS: C2RustUnnamed_19 = 17;
pub const TMD_ATT_EVIL: C2RustUnnamed_19 = 40;
pub const TMD_ATT_POIS: C2RustUnnamed_19 = 38;
pub const TMD_HEAL: C2RustUnnamed_19 = 43;
pub const TMD_SHERO: C2RustUnnamed_19 = 14;
pub const TMD_HERO: C2RustUnnamed_19 = 13;
pub const TMD_FASTCAST: C2RustUnnamed_19 = 33;
pub const TMD_STONESKIN: C2RustUnnamed_19 = 27;
pub const TMD_SHIELD: C2RustUnnamed_19 = 15;
pub const TMD_BLESSED: C2RustUnnamed_19 = 16;
pub const TMD_OPP_POIS: C2RustUnnamed_19 = 23;
pub const TMD_OPP_COLD: C2RustUnnamed_19 = 22;
pub const TMD_OPP_FIRE: C2RustUnnamed_19 = 21;
pub const TMD_OPP_ELEC: C2RustUnnamed_19 = 20;
pub const TMD_OPP_ACID: C2RustUnnamed_19 = 19;
pub const TMD_PROTEVIL: C2RustUnnamed_19 = 11;
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const EF_MAX: C2RustUnnamed_18 = 113;
pub const EF_UNSCRAMBLE_STATS: C2RustUnnamed_18 = 112;
pub const EF_SCRAMBLE_STATS: C2RustUnnamed_18 = 111;
pub const EF_CLEAR_VALUE: C2RustUnnamed_18 = 110;
pub const EF_SET_VALUE: C2RustUnnamed_18 = 109;
pub const EF_SELECT: C2RustUnnamed_18 = 108;
pub const EF_WONDER: C2RustUnnamed_18 = 107;
pub const EF_BIZARRE: C2RustUnnamed_18 = 106;
pub const EF_SWEEP: C2RustUnnamed_18 = 105;
pub const EF_MELEE_BLOWS: C2RustUnnamed_18 = 104;
pub const EF_SINGLE_COMBAT: C2RustUnnamed_18 = 103;
pub const EF_MOVE_ATTACK: C2RustUnnamed_18 = 102;
pub const EF_JUMP_AND_BITE: C2RustUnnamed_18 = 101;
pub const EF_COMMAND: C2RustUnnamed_18 = 100;
pub const EF_CURSE: C2RustUnnamed_18 = 99;
pub const EF_SHAPECHANGE: C2RustUnnamed_18 = 98;
pub const EF_TAP_UNLIFE: C2RustUnnamed_18 = 97;
pub const EF_TAP_DEVICE: C2RustUnnamed_18 = 96;
pub const EF_CREATE_ARROWS: C2RustUnnamed_18 = 95;
pub const EF_BRAND_BOLTS: C2RustUnnamed_18 = 94;
pub const EF_BRAND_AMMO: C2RustUnnamed_18 = 93;
pub const EF_BRAND_WEAPON: C2RustUnnamed_18 = 92;
pub const EF_CURSE_WEAPON: C2RustUnnamed_18 = 91;
pub const EF_CURSE_ARMOR: C2RustUnnamed_18 = 90;
pub const EF_TOUCH_AWARE: C2RustUnnamed_18 = 89;
pub const EF_TOUCH: C2RustUnnamed_18 = 88;
pub const EF_BOLT_AWARE: C2RustUnnamed_18 = 87;
pub const EF_BOLT_STATUS_DAM: C2RustUnnamed_18 = 86;
pub const EF_BOLT_STATUS: C2RustUnnamed_18 = 85;
pub const EF_ALTER: C2RustUnnamed_18 = 84;
pub const EF_LINE: C2RustUnnamed_18 = 83;
pub const EF_BOLT_OR_BEAM: C2RustUnnamed_18 = 82;
pub const EF_BEAM: C2RustUnnamed_18 = 81;
pub const EF_BOLT: C2RustUnnamed_18 = 80;
pub const EF_STAR_BALL: C2RustUnnamed_18 = 79;
pub const EF_STAR: C2RustUnnamed_18 = 78;
pub const EF_STRIKE: C2RustUnnamed_18 = 77;
pub const EF_SWARM: C2RustUnnamed_18 = 76;
pub const EF_LASH: C2RustUnnamed_18 = 75;
pub const EF_SHORT_BEAM: C2RustUnnamed_18 = 74;
pub const EF_ARC: C2RustUnnamed_18 = 73;
pub const EF_BREATH: C2RustUnnamed_18 = 72;
pub const EF_BALL: C2RustUnnamed_18 = 71;
pub const EF_SPHERE: C2RustUnnamed_18 = 70;
pub const EF_SPOT: C2RustUnnamed_18 = 69;
pub const EF_DARKEN_AREA: C2RustUnnamed_18 = 68;
pub const EF_LIGHT_AREA: C2RustUnnamed_18 = 67;
pub const EF_DARKEN_LEVEL: C2RustUnnamed_18 = 66;
pub const EF_LIGHT_LEVEL: C2RustUnnamed_18 = 65;
pub const EF_EARTHQUAKE: C2RustUnnamed_18 = 64;
pub const EF_DESTRUCTION: C2RustUnnamed_18 = 63;
pub const EF_GRANITE: C2RustUnnamed_18 = 62;
pub const EF_RUBBLE: C2RustUnnamed_18 = 61;
pub const EF_TELEPORT_LEVEL: C2RustUnnamed_18 = 60;
pub const EF_TELEPORT_TO: C2RustUnnamed_18 = 59;
pub const EF_TELEPORT: C2RustUnnamed_18 = 58;
pub const EF_PROBE: C2RustUnnamed_18 = 57;
pub const EF_MASS_BANISH: C2RustUnnamed_18 = 56;
pub const EF_BANISH: C2RustUnnamed_18 = 55;
pub const EF_SUMMON: C2RustUnnamed_18 = 54;
pub const EF_WAKE: C2RustUnnamed_18 = 53;
pub const EF_ACQUIRE: C2RustUnnamed_18 = 52;
pub const EF_PROJECT_LOS_AWARE: C2RustUnnamed_18 = 51;
pub const EF_PROJECT_LOS: C2RustUnnamed_18 = 50;
pub const EF_RECHARGE: C2RustUnnamed_18 = 49;
pub const EF_ENCHANT: C2RustUnnamed_18 = 48;
pub const EF_DISENCHANT: C2RustUnnamed_18 = 47;
pub const EF_CREATE_STAIRS: C2RustUnnamed_18 = 46;
pub const EF_DETECT_SOUL: C2RustUnnamed_18 = 45;
pub const EF_DETECT_EVIL: C2RustUnnamed_18 = 44;
pub const EF_IDENTIFY: C2RustUnnamed_18 = 43;
pub const EF_DETECT_FEARFUL_MONSTERS: C2RustUnnamed_18 = 42;
pub const EF_DETECT_INVISIBLE_MONSTERS: C2RustUnnamed_18 = 41;
pub const EF_DETECT_VISIBLE_MONSTERS: C2RustUnnamed_18 = 40;
pub const EF_DETECT_LIVING_MONSTERS: C2RustUnnamed_18 = 39;
pub const EF_DETECT_OBJECTS: C2RustUnnamed_18 = 38;
pub const EF_SENSE_OBJECTS: C2RustUnnamed_18 = 37;
pub const EF_DETECT_GOLD: C2RustUnnamed_18 = 36;
pub const EF_SENSE_GOLD: C2RustUnnamed_18 = 35;
pub const EF_DETECT_ORE: C2RustUnnamed_18 = 34;
pub const EF_DETECT_STAIRS: C2RustUnnamed_18 = 33;
pub const EF_DETECT_DOORS: C2RustUnnamed_18 = 32;
pub const EF_DETECT_TRAPS: C2RustUnnamed_18 = 31;
pub const EF_READ_MINDS: C2RustUnnamed_18 = 30;
pub const EF_MAP_AREA: C2RustUnnamed_18 = 29;
pub const EF_ALTER_REALITY: C2RustUnnamed_18 = 28;
pub const EF_DEEP_DESCENT: C2RustUnnamed_18 = 27;
pub const EF_RECALL: C2RustUnnamed_18 = 26;
pub const EF_REMOVE_CURSE: C2RustUnnamed_18 = 25;
pub const EF_RESTORE_MANA: C2RustUnnamed_18 = 24;
pub const EF_DRAIN_MANA: C2RustUnnamed_18 = 23;
pub const EF_DRAIN_LIGHT: C2RustUnnamed_18 = 22;
pub const EF_GAIN_EXP: C2RustUnnamed_18 = 21;
pub const EF_RESTORE_EXP: C2RustUnnamed_18 = 20;
pub const EF_GAIN_STAT: C2RustUnnamed_18 = 19;
pub const EF_LOSE_RANDOM_STAT: C2RustUnnamed_18 = 18;
pub const EF_DRAIN_STAT: C2RustUnnamed_18 = 17;
pub const EF_RESTORE_STAT: C2RustUnnamed_18 = 16;
pub const EF_WEB: C2RustUnnamed_18 = 15;
pub const EF_GLYPH: C2RustUnnamed_18 = 14;
pub const EF_TIMED_DEC: C2RustUnnamed_18 = 13;
pub const EF_MON_TIMED_INC: C2RustUnnamed_18 = 12;
pub const EF_TIMED_INC_NO_RES: C2RustUnnamed_18 = 11;
pub const EF_TIMED_SET: C2RustUnnamed_18 = 9;
pub const EF_MON_HEAL_KIN: C2RustUnnamed_18 = 5;
pub const EF_MON_HEAL_HP: C2RustUnnamed_18 = 4;
pub const EF_HEAL_HP: C2RustUnnamed_18 = 3;
pub const EF_DAMAGE: C2RustUnnamed_18 = 2;
pub const EF_RANDOM: C2RustUnnamed_18 = 1;
pub const EF_NONE: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const TMD_MAX: C2RustUnnamed_19 = 53;
pub const TMD_FREE_ACT: C2RustUnnamed_19 = 52;
pub const TMD_STEALTH: C2RustUnnamed_19 = 51;
pub const TMD_BLACKBREATH: C2RustUnnamed_19 = 50;
pub const TMD_BLOODLUST: C2RustUnnamed_19 = 49;
pub const TMD_TAUNT: C2RustUnnamed_19 = 48;
pub const TMD_POWERSHOT: C2RustUnnamed_19 = 47;
pub const TMD_COVERTRACKS: C2RustUnnamed_19 = 46;
pub const TMD_ATT_RUN: C2RustUnnamed_19 = 45;
pub const TMD_COMMAND: C2RustUnnamed_19 = 44;
pub const TMD_ATT_VAMP: C2RustUnnamed_19 = 42;
pub const TMD_ATT_DEMON: C2RustUnnamed_19 = 41;
pub const TMD_ATT_CONF: C2RustUnnamed_19 = 39;
pub const TMD_ATT_COLD: C2RustUnnamed_19 = 37;
pub const TMD_ATT_FIRE: C2RustUnnamed_19 = 36;
pub const TMD_ATT_ELEC: C2RustUnnamed_19 = 35;
pub const TMD_ATT_ACID: C2RustUnnamed_19 = 34;
pub const TMD_TRAPSAFE: C2RustUnnamed_19 = 32;
pub const TMD_SCRAMBLE: C2RustUnnamed_19 = 31;
pub const TMD_BOLD: C2RustUnnamed_19 = 30;
pub const TMD_TELEPATHY: C2RustUnnamed_19 = 26;
pub const TMD_OPP_CONF: C2RustUnnamed_19 = 24;
pub const TMD_SINFRA: C2RustUnnamed_19 = 18;
pub const TMD_INVULN: C2RustUnnamed_19 = 12;
pub const TMD_SLOW: C2RustUnnamed_19 = 1;
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_20 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_20 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_20 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_20 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_20 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_20 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_20 = 29;
pub const BORG_SELF_LUNAL: C2RustUnnamed_20 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_20 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_20 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_20 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_20 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_20 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_20 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_20 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_20 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_20 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_20 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_20 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_20 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_20 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_20 = 13;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_20 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_20 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_20 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_20 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_20 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_20 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_20 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_20 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_20 = 3;
pub const BORG_VERBOSE: C2RustUnnamed_20 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut borg_adj_mag_mana: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    60 as ::core::ffi::c_int,
    70 as ::core::ffi::c_int,
    80 as ::core::ffi::c_int,
    90 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    110 as ::core::ffi::c_int,
    120 as ::core::ffi::c_int,
    130 as ::core::ffi::c_int,
    140 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    160 as ::core::ffi::c_int,
    170 as ::core::ffi::c_int,
    180 as ::core::ffi::c_int,
    190 as ::core::ffi::c_int,
    200 as ::core::ffi::c_int,
    225 as ::core::ffi::c_int,
    250 as ::core::ffi::c_int,
    300 as ::core::ffi::c_int,
    350 as ::core::ffi::c_int,
    400 as ::core::ffi::c_int,
    450 as ::core::ffi::c_int,
    500 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    650 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    750 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
];
static mut borg_adj_dex_ta: [::core::ffi::c_int; 38] = [
    -(4 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_adj_str_td: [::core::ffi::c_int; 38] = [
    -(2 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_adj_dex_th: [::core::ffi::c_int; 38] = [
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
static mut borg_adj_str_th: [::core::ffi::c_int; 38] = [
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
];
static mut borg_adj_dex_blow: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
];
static mut borg_blows_table: [[::core::ffi::c_int; 12]; 12] = [
    [
        100 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
    ],
    [
        100 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
    ],
    [
        95 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
    ],
    [
        85 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
    ],
    [
        75 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
    ],
    [
        60 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        50 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        42 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
    ],
    [
        36 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        35 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        34 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        33 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    ],
];
static mut borg_adj_dex_dis: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_int_dis: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_int_dev: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
];
static mut borg_adj_str_dig: [::core::ffi::c_int; 38] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    25 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    35 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    45 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    55 as ::core::ffi::c_int,
    60 as ::core::ffi::c_int,
    65 as ::core::ffi::c_int,
    70 as ::core::ffi::c_int,
    75 as ::core::ffi::c_int,
    80 as ::core::ffi::c_int,
    85 as ::core::ffi::c_int,
    90 as ::core::ffi::c_int,
    95 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
];
static mut borg_adj_wis_sav: [::core::ffi::c_int; 38] = [
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
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
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
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
];
static mut borg_adj_str_wgt: [::core::ffi::c_int; 38] = [
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    17 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    19 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    22 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    26 as ::core::ffi::c_int,
    28 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
];
static mut borg_adj_con_mhp: [::core::ffi::c_int; 38] = [
    -(250 as ::core::ffi::c_int),
    -(150 as ::core::ffi::c_int),
    -(100 as ::core::ffi::c_int),
    -(75 as ::core::ffi::c_int),
    -(50 as ::core::ffi::c_int),
    -(25 as ::core::ffi::c_int),
    -(10 as ::core::ffi::c_int),
    -(5 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    25 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    75 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    175 as ::core::ffi::c_int,
    200 as ::core::ffi::c_int,
    225 as ::core::ffi::c_int,
    250 as ::core::ffi::c_int,
    275 as ::core::ffi::c_int,
    300 as ::core::ffi::c_int,
    350 as ::core::ffi::c_int,
    400 as ::core::ffi::c_int,
    450 as ::core::ffi::c_int,
    500 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    650 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    750 as ::core::ffi::c_int,
    800 as ::core::ffi::c_int,
    900 as ::core::ffi::c_int,
    1000 as ::core::ffi::c_int,
    1100 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
    1250 as ::core::ffi::c_int,
];
static mut borg_adj_mag_fail: [::core::ffi::c_int; 38] = [
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
];
static mut borg_adj_mag_stat: [::core::ffi::c_int; 38] = [
    -(5 as ::core::ffi::c_int),
    -(4 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(3 as ::core::ffi::c_int),
    -(2 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    21 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    27 as ::core::ffi::c_int,
    30 as ::core::ffi::c_int,
    33 as ::core::ffi::c_int,
    36 as ::core::ffi::c_int,
    39 as ::core::ffi::c_int,
    42 as ::core::ffi::c_int,
    45 as ::core::ffi::c_int,
    48 as ::core::ffi::c_int,
    51 as ::core::ffi::c_int,
    54 as ::core::ffi::c_int,
    57 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut borg_simulate: bool = false;
#[no_mangle]
pub static mut borg_attacking: bool = false;
#[no_mangle]
pub static mut borg_on_glyph: bool = false;
#[no_mangle]
pub static mut borg_create_door: bool = false;
#[no_mangle]
pub static mut borg_sleep_spell: bool = false;
#[no_mangle]
pub static mut borg_sleep_spell_ii: bool = false;
#[no_mangle]
pub static mut borg_crush_spell: bool = false;
#[no_mangle]
pub static mut borg_slow_spell: bool = false;
#[no_mangle]
pub static mut borg_confuse_spell: bool = false;
#[no_mangle]
pub static mut borg_fear_mon_spell: bool = false;
#[no_mangle]
pub static mut borg_game_ratio: int16_t = 0;
#[no_mangle]
pub static mut prefix_pref: [*const ::core::ffi::c_char; 266] = [
    b"str\0" as *const u8 as *const ::core::ffi::c_char,
    b"int\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"con\0" as *const u8 as *const ::core::ffi::c_char,
    b"str adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"int adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"con adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur str\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur int\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur con\0" as *const u8 as *const ::core::ffi::c_char,
    b"str index\0" as *const u8 as *const ::core::ffi::c_char,
    b"int index\0" as *const u8 as *const ::core::ffi::c_char,
    b"wis index\0" as *const u8 as *const ::core::ffi::c_char,
    b"dex index\0" as *const u8 as *const ::core::ffi::c_char,
    b"con index\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust str\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust int\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust wis\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust dex\0" as *const u8 as *const ::core::ffi::c_char,
    b"sust con\0" as *const u8 as *const ::core::ffi::c_char,
    b"class\0" as *const u8 as *const ::core::ffi::c_char,
    b"light\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"max hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"hp adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"cur sp\0" as *const u8 as *const ::core::ffi::c_char,
    b"max sp\0" as *const u8 as *const ::core::ffi::c_char,
    b"sp adj\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFAIL1\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFAIL2\0" as *const u8 as *const ::core::ffi::c_char,
    b"clevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"max clevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"esp\0" as *const u8 as *const ::core::ffi::c_char,
    b"recall\0" as *const u8 as *const ::core::ffi::c_char,
    b"food\0" as *const u8 as *const ::core::ffi::c_char,
    b"food high\0" as *const u8 as *const ::core::ffi::c_char,
    b"food low\0" as *const u8 as *const ::core::ffi::c_char,
    b"food cure conf\0" as *const u8 as *const ::core::ffi::c_char,
    b"food cure blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"speed\0" as *const u8 as *const ::core::ffi::c_char,
    b"gold\0" as *const u8 as *const ::core::ffi::c_char,
    b"extra moves\0" as *const u8 as *const ::core::ffi::c_char,
    b"damage reduction\0" as *const u8 as *const ::core::ffi::c_char,
    b"slow dig\0" as *const u8 as *const ::core::ffi::c_char,
    b"feather fall\0" as *const u8 as *const ::core::ffi::c_char,
    b"regen\0" as *const u8 as *const ::core::ffi::c_char,
    b"see inv\0" as *const u8 as *const ::core::ffi::c_char,
    b"infravision\0" as *const u8 as *const ::core::ffi::c_char,
    b"fast shots\0" as *const u8 as *const ::core::ffi::c_char,
    b"disarm ph\0" as *const u8 as *const ::core::ffi::c_char,
    b"disarm mag\0" as *const u8 as *const ::core::ffi::c_char,
    b"use device\0" as *const u8 as *const ::core::ffi::c_char,
    b"save\0" as *const u8 as *const ::core::ffi::c_char,
    b"stealth\0" as *const u8 as *const ::core::ffi::c_char,
    b"search\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit normal\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit bow\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit throw\0" as *const u8 as *const ::core::ffi::c_char,
    b"dig\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"immune poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fear\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist lite\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dark\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist conf\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist sound\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist shards\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nexus\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nether\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist chaos\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dis\0" as *const u8 as *const ::core::ffi::c_char,
    b"hold life\0" as *const u8 as *const ::core::ffi::c_char,
    b"free action\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fire with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist cold with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist elec with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist acid with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist poison with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist fear with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist lite with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dark with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist blind with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist conf with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist sound with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist shards with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nexus with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist nether with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist chaos with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"resist dis with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"hold life with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"free action with swap\0" as *const u8 as *const ::core::ffi::c_char,
    b"depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"max depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"king\0" as *const u8 as *const ::core::ffi::c_char,
    b"is weak\0" as *const u8 as *const ::core::ffi::c_char,
    b"is hungry\0" as *const u8 as *const ::core::ffi::c_char,
    b"is full\0" as *const u8 as *const ::core::ffi::c_char,
    b"is gorged\0" as *const u8 as *const ::core::ffi::c_char,
    b"is blind\0" as *const u8 as *const ::core::ffi::c_char,
    b"is afraid\0" as *const u8 as *const ::core::ffi::c_char,
    b"is confused\0" as *const u8 as *const ::core::ffi::c_char,
    b"is poisoned\0" as *const u8 as *const ::core::ffi::c_char,
    b"is cut\0" as *const u8 as *const ::core::ffi::c_char,
    b"is stun\0" as *const u8 as *const ::core::ffi::c_char,
    b"is heavystun\0" as *const u8 as *const ::core::ffi::c_char,
    b"is paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
    b"is image\0" as *const u8 as *const ::core::ffi::c_char,
    b"is forget\0" as *const u8 as *const ::core::ffi::c_char,
    b"is encumb\0" as *const u8 as *const ::core::ffi::c_char,
    b"is study\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixlev\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixexp\0" as *const u8 as *const ::core::ffi::c_char,
    b"has fixexp\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixstr\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixint\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixwis\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixdex\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixcon\0" as *const u8 as *const ::core::ffi::c_char,
    b"is fixall\0" as *const u8 as *const ::core::ffi::c_char,
    b"armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep id\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep damage dice\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep damage sides\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow id\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow to damage\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow is sling\0" as *const u8 as *const ::core::ffi::c_char,
    b"bow artifact\0" as *const u8 as *const ::core::ffi::c_char,
    b"blows\0" as *const u8 as *const ::core::ffi::c_char,
    b"EXTRA_BLOWS\0" as *const u8 as *const ::core::ffi::c_char,
    b"shots\0" as *const u8 as *const ::core::ffi::c_char,
    b"heavy weapon\0" as *const u8 as *const ::core::ffi::c_char,
    b"heavy bow\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo count\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo tval\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo sides\0" as *const u8 as *const ::core::ffi::c_char,
    b"ammo power\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ego missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cursed missiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"quiver slots\0" as *const u8 as *const ::core::ffi::c_char,
    b"first cursed\0" as *const u8 as *const ::core::ffi::c_char,
    b"where cursed\0" as *const u8 as *const ::core::ffi::c_char,
    b"enveloping\0" as *const u8 as *const ::core::ffi::c_char,
    b"irritation\0" as *const u8 as *const ::core::ffi::c_char,
    b"teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"siren\0" as *const u8 as *const ::core::ffi::c_char,
    b"hallucinate\0" as *const u8 as *const ::core::ffi::c_char,
    b"paralysis\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"summon undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse stone\0" as *const u8 as *const ::core::ffi::c_char,
    b"no teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"treach wep\0" as *const u8 as *const ::core::ffi::c_char,
    b"aggravate\0" as *const u8 as *const ::core::ffi::c_char,
    b"vulnerable\0" as *const u8 as *const ::core::ffi::c_char,
    b"dullness\0" as *const u8 as *const ::core::ffi::c_char,
    b"sickness\0" as *const u8 as *const ::core::ffi::c_char,
    b"weakness\0" as *const u8 as *const ::core::ffi::c_char,
    b"clumsiness\0" as *const u8 as *const ::core::ffi::c_char,
    b"slowness\0" as *const u8 as *const ::core::ffi::c_char,
    b"annoyance\0" as *const u8 as *const ::core::ffi::c_char,
    b"impair hp\0" as *const u8 as *const ::core::ffi::c_char,
    b"CRSMPIMP\0" as *const u8 as *const ::core::ffi::c_char,
    b"curse steel\0" as *const u8 as *const ::core::ffi::c_char,
    b"air swing\0" as *const u8 as *const ::core::ffi::c_char,
    b"fear\0" as *const u8 as *const ::core::ffi::c_char,
    b"drain xp\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"vuln acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown curse\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay animal\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay orc\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay troll\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay giant\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep slay dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill undead\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill demon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep kill dragon\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep impact\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand acid\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand elec\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand fire\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"wep brand poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt phase\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt escape\0" as *const u8 as *const ::core::ffi::c_char,
    b"fuel\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt heal\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ezheal\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt life\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt speed\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt staff magi\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt staff destruction\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport other\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cure poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect traps\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect door\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt detect evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt magic map\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt recharge\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt call lite\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt prot evil\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt glyph\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion ccw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion csw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt potion clw\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to dam\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt *ench to wep*\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt ench to armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt *ench to armor*\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt brand\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to armor\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to hit\0" as *const u8 as *const ::core::ffi::c_char,
    b"need ench to dam\0" as *const u8 as *const ::core::ffi::c_char,
    b"need brand\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist heat\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist cold\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt resist poison\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt teleport level\0" as *const u8 as *const ::core::ffi::c_char,
    b"holy word\0" as *const u8 as *const ::core::ffi::c_char,
    b"mass banishment\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt cool shroom\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt attack rods1\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt attack rods2\0" as *const u8 as *const ::core::ffi::c_char,
    b"worn need id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt need id\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt diggers\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt good staff chg\0" as *const u8 as *const ::core::ffi::c_char,
    b"amt good wand chg\0" as *const u8 as *const ::core::ffi::c_char,
    b"multi bonus\0" as *const u8 as *const ::core::ffi::c_char,
    b"detect inv\0" as *const u8 as *const ::core::ffi::c_char,
    b"weight\0" as *const u8 as *const ::core::ffi::c_char,
    b"carry\0" as *const u8 as *const ::core::ffi::c_char,
    b"empty slots\0" as *const u8 as *const ::core::ffi::c_char,
    b"sauron dead\0" as *const u8 as *const ::core::ffi::c_char,
    b"prep big fight\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn borg_uses_swaps() -> bool {
    return *borg_cfg.offset(BORG_USES_SWAPS as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            < 90 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_calc_blows(mut item: *mut borg_item) -> ::core::ffi::c_int {
    let mut blows: ::core::ffi::c_int = 0;
    let mut str_index: ::core::ffi::c_int = 0;
    let mut dex_index: ::core::ffi::c_int = 0;
    let mut div: ::core::ffi::c_int = 0;
    let mut blow_energy: ::core::ffi::c_int = 0;
    let mut weight: ::core::ffi::c_int =
        (*item).weight as ::core::ffi::c_int * (*item).iqty as ::core::ffi::c_int;
    let mut min_weight: ::core::ffi::c_int = (*(*player).class).min_weight;
    div = if weight < min_weight {
        min_weight
    } else {
        weight
    };
    str_index = adj_str_blow[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize) as usize]
        * (*(*player).class).att_multiply
        / div;
    if str_index > 11 as ::core::ffi::c_int {
        str_index = 11 as ::core::ffi::c_int;
    }
    dex_index = if borg_adj_dex_blow[*borg
        .trait_0
        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
        as usize]
        > 11 as ::core::ffi::c_int
    {
        11 as ::core::ffi::c_int
    } else {
        borg_adj_dex_blow[*borg
            .trait_0
            .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize) as usize]
    };
    blow_energy = borg_blows_table[str_index as usize][dex_index as usize];
    blows = if 10000 as ::core::ffi::c_int / blow_energy
        > 100 as ::core::ffi::c_int * (*(*player).class).max_attacks
    {
        100 as ::core::ffi::c_int * (*(*player).class).max_attacks
    } else {
        10000 as ::core::ffi::c_int / blow_energy
    };
    return (if blows
        + 100 as ::core::ffi::c_int
            * ((*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_EXTRA_BLOWS as ::core::ffi::c_int as isize))
        < (if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        {
            200 as ::core::ffi::c_int
        } else {
            100 as ::core::ffi::c_int
        }) {
        (if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
        {
            200 as ::core::ffi::c_int
        } else {
            100 as ::core::ffi::c_int
        })
    } else {
        blows
            + 100 as ::core::ffi::c_int
                * ((*item).modifiers[OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + *borg
                        .trait_0
                        .offset(BI_EXTRA_BLOWS as ::core::ffi::c_int as isize))
    }) / 100 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_notice_ammo(mut slot: ::core::ffi::c_int) {
    let mut item: *mut borg_item = borg_items.offset(slot as isize) as *mut borg_item;
    if (*item).iqty == 0 {
        return;
    }
    if slot >= (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let ref mut fresh1 = *borg
            .trait_0
            .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize);
        *fresh1 += 1;
    }
    *borg
        .trait_0
        .offset(BI_WEIGHT as ::core::ffi::c_int as isize) +=
        borg_item_weight(item) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_AMMO_COUNT as ::core::ffi::c_int as isize) += (*item).iqty as ::core::ffi::c_int;
    if (*item).tval as ::core::ffi::c_int
        != *borg
            .trait_0
            .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
    {
        return;
    }
    *borg
        .trait_0
        .offset(BI_AMISSILES as ::core::ffi::c_int as isize) += (*item).iqty as ::core::ffi::c_int;
    if (*item).uncursable {
        *borg
            .trait_0
            .offset(BI_WHERE_CURSED as ::core::ffi::c_int as isize) |=
            BORG_QUILL as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize)
            == 0
        {
            *borg
                .trait_0
                .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize) =
                slot + 1 as ::core::ffi::c_int;
        }
        *borg
            .trait_0
            .offset(BI_AMISSILES_CURSED as ::core::ffi::c_int as isize) +=
            (*item).iqty as ::core::ffi::c_int;
        return;
    }
    if (*item).ego_idx != 0 {
        *borg
            .trait_0
            .offset(BI_AMISSILES_SPECIAL as ::core::ffi::c_int as isize) +=
            (*item).iqty as ::core::ffi::c_int;
    }
    if (*item).value <= 0 as int32_t {
        return;
    }
    if *borg
        .trait_0
        .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize)
        >= 3 as ::core::ffi::c_int
    {
        if (borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            && (*item).ego_idx == 0
            && (*item).art_idx == 0
            && (*item).ident as ::core::ffi::c_int != 0
            && (*item).tval as ::core::ffi::c_int
                == *borg
                    .trait_0
                    .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
        {
            let ref mut fresh2 = *borg
                .trait_0
                .offset(BI_NEED_BRAND_WEAPON as ::core::ffi::c_int as isize);
            *fresh2 =
                (*fresh2 as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            > 35 as ::core::ffi::c_int
        {
            if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
                && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            {
                if ((*item).to_h as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize) +=
                        10 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                }
                if ((*item).to_d as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize) +=
                        10 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                }
            } else {
                if ((*item).to_h as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize) +=
                        8 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                }
                if ((*item).to_d as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize) +=
                        8 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                }
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize)
        < 3 as ::core::ffi::c_int
    {
        return;
    }
    if (borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0)
        && (*item).iqty as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
        && (*item).art_idx == 0
        && (*item).ego_idx == 0
        && (*item).ident as ::core::ffi::c_int != 0
        && (*item).tval as ::core::ffi::c_int
            == *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
    {
        let ref mut fresh3 = *borg
            .trait_0
            .offset(BI_NEED_BRAND_WEAPON as ::core::ffi::c_int as isize);
        *fresh3 =
            (*fresh3 as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn cursed_nonartifact(mut item: *mut borg_item) -> bool {
    if item.is_null() || (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*item).art_idx != 0 {
        return false_0 != 0;
    }
    if (*item).uncursable {
        return false_0 != 0;
    }
    return (*item).cursed;
}
unsafe extern "C" fn borg_notice_equipment() {
    let mut i: ::core::ffi::c_int = 0;
    let mut hold: ::core::ffi::c_int = 0;
    let mut rb_ptr: *const player_race = (*player).race;
    let mut cb_ptr: *const player_class = (*player).class;
    let mut extra_shots: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut extra_might: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut my_num_fire: ::core::ffi::c_int = 0;
    let mut f: [bitflag; 5] = [0; 5];
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut stat_cur: [int16_t; 5] = [0; 5];
    my_num_fire = 1 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_INFRA as ::core::ffi::c_int as isize) = (*rb_ptr).infra;
    *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_DISARM_PHYS as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DISARM_PHYS as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_DEVICE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DEVICE as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_SAVE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_SAVE as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_STEALTH as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_STEALTH as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_SRCH as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_SEARCH as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_SEARCH as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_THN as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_THB as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_THT as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize];
    *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) = (*rb_ptr).r_skills
        [SKILL_DIGGING as ::core::ffi::c_int as usize]
        + (*cb_ptr).c_skills[SKILL_DIGGING as ::core::ffi::c_int as usize];
    player_flags(player, &raw mut f as *mut bitflag);
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SLOW_DIGEST as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FEATHER as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_FEATH as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_REGEN as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_REG as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_TELEPATHY as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SEE_INVIS as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_FREE_ACT as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_HOLD_LIFE as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_IMPACT as ::core::ffi::c_int,
    ) {
        *borg
            .trait_0
            .offset(BI_W_IMPACT as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AGGRAVATE as ::core::ffi::c_int,
    ) {
        *borg
            .trait_0
            .offset(BI_CRSAGRV as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_AFRAID as ::core::ffi::c_int,
    ) {
        *borg
            .trait_0
            .offset(BI_CRSFEAR as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_DRAIN_EXP as ::core::ffi::c_int,
    ) {
        *borg
            .trait_0
            .offset(BI_CRSDRAIN_XP as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        *borg
            .trait_0
            .offset(BI_CRSFVULN as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        *borg
            .trait_0
            .offset(BI_CRSAVULN as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        *borg
            .trait_0
            .offset(BI_CRSCVULN as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == -(1 as ::core::ffi::c_int)
    {
        *borg
            .trait_0
            .offset(BI_CRSEVULN as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        == 3 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) = true_0;
    }
    if (*rb_ptr).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_FEAR as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_BLIND as ::core::ffi::c_int,
    ) {
        *borg
            .trait_0
            .offset(BI_RBLIND as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_PROT_CONF as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_STR as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_INT as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_WIS as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_DEX as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) = true_0;
    }
    if flag_has(
        &raw mut f as *mut bitflag,
        (RF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_SUST_CON as ::core::ffi::c_int,
    ) {
        *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_FAST as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0
        || *(*player)
            .timed
            .offset(TMD_SPRINT as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) += 10 as ::core::ffi::c_int;
    } else if *(*player)
        .timed
        .offset(TMD_TERROR as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) += 5 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        if (*borg_items.offset(i as isize)).activ_idx != 0 {
            *borg
                .activation
                .offset((*borg_items.offset(i as isize)).activ_idx as isize) +=
                1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if *borg.activation.offset(act_staff_magi as isize) != 0 {
        *borg
            .trait_0
            .offset(BI_ASTFMAGI as ::core::ffi::c_int as isize) += 10 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).uncursable {
                *borg
                    .trait_0
                    .offset(BI_WHERE_CURSED as ::core::ffi::c_int as isize) |=
                    BORG_EQUIP as ::core::ffi::c_int;
                if *borg
                    .trait_0
                    .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize)
                    == 0
                {
                    *borg
                        .trait_0
                        .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize) =
                        i + 1 as ::core::ffi::c_int;
                }
            }
            if !cursed_nonartifact(item) {
                *borg
                    .trait_0
                    .offset(BI_WEIGHT as ::core::ffi::c_int as isize) +=
                    borg_item_weight(item) as ::core::ffi::c_int;
                if borg_item_note_needs_id(item) {
                    *borg
                        .trait_0
                        .offset(BI_ALL_NEED_ID as ::core::ffi::c_int as isize) +=
                        1 as ::core::ffi::c_int;
                    *borg
                        .trait_0
                        .offset(BI_WORN_NEED_ID as ::core::ffi::c_int as isize) +=
                        1 as ::core::ffi::c_int;
                }
                *borg.trait_0.offset(BI_ASTR as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_STR as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_STR as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_AINT as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_INT as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_INT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_AWIS as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_WIS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_ADEX as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_DEX as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_ACON as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_CON as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * (*(*player).obj_k).modifiers[OBJ_MOD_CON as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                *borg
                    .trait_0
                    .offset(BI_WS_ANIMAL as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_ANIMAL as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_EVIL as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_EVIL as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_UNDEAD as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_UNDEAD as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_DEMON as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_DEMON as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_ORC as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_ORC as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_TROLL as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_TROLL as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_GIANT as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_GIANT as ::core::ffi::c_int as usize];
                *borg
                    .trait_0
                    .offset(BI_WS_DRAGON as ::core::ffi::c_int as isize) =
                    (*item).slays[RF_DRAGON as ::core::ffi::c_int as usize];
                if (*item).brands[ELEM_ACID as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_WB_ACID as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).brands[ELEM_ELEC as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_WB_ELEC as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).brands[ELEM_FIRE as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_WB_FIRE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).brands[ELEM_COLD as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_WB_COLD as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).brands[ELEM_POIS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_WB_POIS as ::core::ffi::c_int as isize) = true_0;
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
                    OF_IMPACT as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_W_IMPACT as ::core::ffi::c_int as isize) = true_0;
                }
                *borg.trait_0.offset(BI_INFRA as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_INFRA as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_STEALTH as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_SRCH as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_SEARCH as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * 5 as ::core::ffi::c_int;
                let mut dig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if (*item).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int {
                    if flag_has(
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
                        OF_DIG_1 as ::core::ffi::c_int,
                    ) {
                        dig = 1 as ::core::ffi::c_int;
                    } else if flag_has(
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
                        OF_DIG_2 as ::core::ffi::c_int,
                    ) {
                        dig = 2 as ::core::ffi::c_int;
                    } else if flag_has(
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
                        OF_DIG_3 as ::core::ffi::c_int,
                    ) {
                        dig = 3 as ::core::ffi::c_int;
                    }
                }
                dig += (*item).modifiers[OBJ_MOD_TUNNEL as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) +=
                    dig * 20 as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_SPEED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                if i != (*z_info).pack_size as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_EXTRA_BLOWS as ::core::ffi::c_int as isize) += (*item).modifiers
                        [OBJ_MOD_BLOWS as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                }
                extra_shots += (*item).modifiers[OBJ_MOD_SHOTS as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                extra_might += (*item).modifiers[OBJ_MOD_MIGHT as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                if i != (*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                    || flag_has(
                        &raw mut (*borg_items.offset(i as isize)).flags as *mut bitflag,
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
                        OF_NO_FUEL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                    || (*item).timeout as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) += (*item)
                        .modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    if flag_has(
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
                        OF_LIGHT_2 as ::core::ffi::c_int,
                    ) {
                        *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) +=
                            2 as ::core::ffi::c_int;
                    } else if flag_has(
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
                        OF_LIGHT_3 as ::core::ffi::c_int,
                    ) {
                        *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) +=
                            3 as ::core::ffi::c_int;
                    }
                    if (*item).modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                            == CLASS_NECROMANCER
                    {
                        let ref mut fresh4 =
                            *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize);
                        *fresh4 -= 1;
                    }
                    *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) += (*item)
                        .modifiers[OBJ_MOD_LIGHT as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                }
                *borg
                    .trait_0
                    .offset(BI_MOD_MOVES as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_MOVES as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                *borg
                    .trait_0
                    .offset(BI_DAM_RED as ::core::ffi::c_int as isize) += (*item).modifiers
                    [OBJ_MOD_DAM_RED as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
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
                    *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) = true_0;
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
                    OF_AGGRAVATE as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_CRSAGRV as ::core::ffi::c_int as isize) = true_0;
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
                    OF_IMPAIR_HP as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_CRSHPIMP as ::core::ffi::c_int as isize) = true_0;
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
                    OF_IMPAIR_MANA as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_CRSMPIMP as ::core::ffi::c_int as isize) = true_0;
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
                    OF_AFRAID as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_CRSFEAR as ::core::ffi::c_int as isize) = true_0;
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
                    OF_DRAIN_EXP as ::core::ffi::c_int,
                ) {
                    *borg
                        .trait_0
                        .offset(BI_CRSDRAIN_XP as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_VULNERABILITY as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSVULN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_TELEPORTATION as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSTELE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_DULLNESS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSDULL as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_SICKLINESS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSICK as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_ENVELOPING as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSENVELOPING as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_IRRITATION as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSAGRV as ::core::ffi::c_int as isize) = true_0;
                    *borg
                        .trait_0
                        .offset(BI_CRSIRRITATION as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_WEAKNESS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSWEAK as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_CLUMSINESS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSCLUM as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_SLOWNESS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSLOW as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_ANNOYANCE as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSANNOY as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_POISON as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSPOIS as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_SIREN as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSIREN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_HALLUCINATION as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSHALU as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_PARALYSIS as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSPARA as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_DEMON_SUMMON as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSDEM as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_DRAGON_SUMMON as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSDRA as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_UNDEAD_SUMMON as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSUND as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_IMPAIR_MANA_RECOVERY as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSMPIMP as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses
                    [BORG_CURSE_IMPAIR_HITPOINT_RECOVERY as ::core::ffi::c_int as usize]
                {
                    *borg
                        .trait_0
                        .offset(BI_CRSHPIMP as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_COWARDICE as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSFEAR as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_STONE as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSTONE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_ANTI_TELEPORTATION as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSNOTEL as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_TREACHEROUS_WEAPON as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSTWEP as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_BURNING_UP as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSFVULN as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_CHILLED_TO_THE_BONE as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSCVULN as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_STEELSKIN as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSSTEELSKIN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_AIR_SWING as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSAIRSWING as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).curses[BORG_CURSE_UNKNOWN as ::core::ffi::c_int as usize] {
                    *borg
                        .trait_0
                        .offset(BI_CRSUNKNO as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    *borg
                        .trait_0
                        .offset(BI_CRSFVULN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    *borg
                        .trait_0
                        .offset(BI_CRSAVULN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    *borg
                        .trait_0
                        .offset(BI_CRSCVULN as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == -(1 as ::core::ffi::c_int)
                {
                    *borg
                        .trait_0
                        .offset(BI_CRSEVULN as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_REG as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_FEATH as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) = true_0;
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
                    *borg
                        .trait_0
                        .offset(BI_RBLIND as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) = true_0;
                    borg.temp.res_fire = true_0 != 0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) = true_0;
                    borg.temp.res_acid = true_0 != 0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) = true_0;
                    borg.temp.res_cold = true_0 != 0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    == 3 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) = true_0;
                    *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) = true_0;
                    borg.temp.res_elec = true_0 != 0;
                }
                if (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_POIS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_LIGHT as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_DARK as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) = true_0;
                }
                if (*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) = true_0;
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
                    *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) = true_0;
                }
                let mut bonuses: ::core::ffi::c_int = ((*item).el_info
                    [ELEM_POIS as ::core::ffi::c_int as usize]
                    .res_level
                    as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_SOUND as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_SHARD as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_NEXUS as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_NETHER as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_CHAOS as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_DISEN as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + ((*item).el_info[ELEM_FIRE as ::core::ffi::c_int as usize].res_level
                        as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_COLD as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_ELEC as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                        && (*item).el_info[ELEM_ACID as ::core::ffi::c_int as usize].res_level
                            as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + (flag_has(
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
                        OF_SUST_STR as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && flag_has(
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
                            OF_SUST_INT as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_WIS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_DEX as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                        && flag_has(
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
                            OF_SUST_CON as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
                if bonuses > 2 as ::core::ffi::c_int {
                    *borg
                        .trait_0
                        .offset(BI_MULTIPLE_BONUSES as ::core::ffi::c_int as isize) += bonuses;
                }
                if (*item).art_idx == 0
                    && (*item).ego_idx == 0
                    && (*item).ac as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                    && (*item).to_a as ::core::ffi::c_int + (*item).ac as ::core::ffi::c_int
                        <= 0 as ::core::ffi::c_int
                {
                    (*item).to_a = -(20 as ::core::ffi::c_int) as int16_t;
                }
                *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) +=
                    (*item).ac as ::core::ffi::c_int;
                *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) +=
                    (*item).to_a as ::core::ffi::c_int;
                if !(i == (*z_info).pack_size as ::core::ffi::c_int) {
                    if !(i == (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) {
                        *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) +=
                            (*item).to_h as ::core::ffi::c_int;
                        *borg.trait_0.offset(BI_TODAM as ::core::ffi::c_int as isize) +=
                            (*item).to_d as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) <= 0 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSVULN as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_CRSAGRV as ::core::ffi::c_int as isize) = true_0;
        *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) -= 50 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSDULL as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_CINT as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_CWIS as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSICK as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_CCON as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSWEAK as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize) -= 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSCLUM as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize) -= 10 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSLOW as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) -= 5 as ::core::ffi::c_int;
    }
    if *borg
        .trait_0
        .offset(BI_CRSANNOY as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) -= 10 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) -= 10 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_CRSAGRV as ::core::ffi::c_int as isize) = true_0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        stat_cur[i as usize] = (*player).stat_cur[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        let mut add: ::core::ffi::c_int = 0;
        let mut use_0: ::core::ffi::c_int = 0;
        let mut ind: ::core::ffi::c_int = 0;
        add = *borg
            .trait_0
            .offset((BI_ASTR as ::core::ffi::c_int + i) as isize);
        add += (*(*player).race).r_adj[i as usize] + (*(*player).class).c_adj[i as usize];
        use_0 = modify_stat_value(stat_cur[i as usize] as ::core::ffi::c_int, add)
            as ::core::ffi::c_int;
        if use_0 <= 18 as ::core::ffi::c_int {
            ind = use_0 - 3 as ::core::ffi::c_int;
        } else if use_0 <= 18 as ::core::ffi::c_int + 219 as ::core::ffi::c_int {
            ind = 15 as ::core::ffi::c_int
                + (use_0 - 18 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int;
        } else {
            ind = 37 as ::core::ffi::c_int;
        }
        if ind > 37 as ::core::ffi::c_int {
            *borg
                .trait_0
                .offset((BI_STR_INDEX as ::core::ffi::c_int + i) as isize) =
                37 as ::core::ffi::c_int;
        } else {
            *borg
                .trait_0
                .offset((BI_STR_INDEX as ::core::ffi::c_int + i) as isize) = ind;
        }
        *borg
            .trait_0
            .offset((BI_STR as ::core::ffi::c_int + i) as isize) = use_0;
        *borg
            .trait_0
            .offset((BI_CSTR as ::core::ffi::c_int + i) as isize) =
            stat_cur[i as usize] as ::core::ffi::c_int;
        i += 1;
    }
    *borg
        .trait_0
        .offset(BI_HP_ADJ as ::core::ffi::c_int as isize) = (*player).player_hp
        [((*player).lev as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int
        + borg_adj_con_mhp[*borg
            .trait_0
            .offset(BI_CON_INDEX as ::core::ffi::c_int as isize)
            as usize]
            * *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            / 100 as ::core::ffi::c_int;
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        *borg
            .trait_0
            .offset(BI_SP_ADJ as ::core::ffi::c_int as isize) = borg_adj_mag_mana[*borg
            .trait_0
            .offset((BI_STR_INDEX as ::core::ffi::c_int + spell_stat) as isize)
            as usize]
            * (*borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - (*(*player).class).magic.spell_first
                + 1 as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_FAIL1 as ::core::ffi::c_int as isize) = borg_adj_mag_stat[*borg
            .trait_0
            .offset((BI_STR_INDEX as ::core::ffi::c_int + spell_stat) as isize)
            as usize];
        *borg.trait_0.offset(BI_FAIL2 as ::core::ffi::c_int as isize) = borg_adj_mag_fail[*borg
            .trait_0
            .offset((BI_STR_INDEX as ::core::ffi::c_int + spell_stat) as isize)
            as usize];
    }
    if *borg
        .trait_0
        .offset(BI_ISGORGED as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) -= 10 as ::core::ffi::c_int;
    }
    *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) += borg_adj_dex_ta[*borg
        .trait_0
        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_TODAM as ::core::ffi::c_int as isize) += borg_adj_str_td[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) += borg_adj_dex_th[*borg
        .trait_0
        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) += borg_adj_str_th[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        as usize];
    hold = adj_str_hold[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize) as usize];
    *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) += borg_adj_str_dig[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        as usize];
    item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || cursed_nonartifact(item) as ::core::ffi::c_int != 0
    {
        (*item).ds = 0 as uint8_t;
        (*item).dd = 0 as uint8_t;
        (*item).to_d = 0 as int16_t;
        (*item).to_h = 0 as int16_t;
        (*item).weight = 0 as int16_t;
        (*item).ident = true_0 != 0;
        (*item).sval = 0 as uint8_t;
    }
    *borg
        .trait_0
        .offset(BI_BTOHIT as ::core::ffi::c_int as isize) = (*item).to_h as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_BTODAM as ::core::ffi::c_int as isize) = (*item).to_d as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_BID as ::core::ffi::c_int as isize) =
        (*item).ident as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SLING as ::core::ffi::c_int as isize) =
        ((*item).sval as ::core::ffi::c_int == sv_sling) as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_BART as ::core::ffi::c_int as isize) =
        (*item).art_idx as ::core::ffi::c_int;
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        *borg
            .trait_0
            .offset(BI_HEAVYBOW as ::core::ffi::c_int as isize) = true_0;
        *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) += 2 as ::core::ffi::c_int
            * (hold - (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int);
    }
    if (*item).iqty as ::core::ffi::c_int != 0
        && hold >= (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        if (*item).sval as ::core::ffi::c_int == sv_sling {
            *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) =
                TV_SHOT as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 3 as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) = 2 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_short_bow {
            *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) =
                TV_ARROW as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 4 as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) = 2 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_long_bow {
            *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) =
                TV_ARROW as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 4 as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) = 3 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_light_xbow {
            *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) =
                TV_BOLT as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 5 as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) = 3 as ::core::ffi::c_int;
        } else if (*item).sval as ::core::ffi::c_int == sv_heavy_xbow {
            *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) =
                TV_BOLT as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 5 as ::core::ffi::c_int;
            *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) = 4 as ::core::ffi::c_int;
        }
        *borg
            .trait_0
            .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize) += extra_might;
        if flag_has(
            &raw mut (*player).state.pflags as *mut bitflag,
            (PF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            PF_FAST_SHOT as ::core::ffi::c_int,
        ) {
            if *borg
                .trait_0
                .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
                == TV_ARROW as ::core::ffi::c_int
            {
                if *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    >= 20 as ::core::ffi::c_int
                {
                    my_num_fire += 1;
                }
            }
            if *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                >= 40 as ::core::ffi::c_int
            {
                my_num_fire += 1;
            }
            *borg
                .trait_0
                .offset(BI_FAST_SHOTS as ::core::ffi::c_int as isize) = true_0;
        }
        my_num_fire += extra_shots;
        if my_num_fire < 1 as ::core::ffi::c_int {
            my_num_fire = 1 as ::core::ffi::c_int;
        }
    }
    *borg.trait_0.offset(BI_SHOTS as ::core::ffi::c_int as isize) = my_num_fire;
    item = borg_items.offset((*z_info).pack_size as isize) as *mut borg_item;
    if (*item).iqty as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || cursed_nonartifact(item) as ::core::ffi::c_int != 0
    {
        (*item).ds = 0 as uint8_t;
        (*item).dd = 0 as uint8_t;
        (*item).to_d = 0 as int16_t;
        (*item).to_h = 0 as int16_t;
        (*item).weight = 0 as int16_t;
        (*item).ident = true_0 != 0;
    }
    *borg
        .trait_0
        .offset(BI_WTOHIT as ::core::ffi::c_int as isize) = (*item).to_h as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_WTODAM as ::core::ffi::c_int as isize) = (*item).to_d as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_WID as ::core::ffi::c_int as isize) =
        (*item).ident as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_WDD as ::core::ffi::c_int as isize) = (*item).dd as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_WDS as ::core::ffi::c_int as isize) = (*item).ds as ::core::ffi::c_int;
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        *borg
            .trait_0
            .offset(BI_HEAVYWEPON as ::core::ffi::c_int as isize) = true_0;
        *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) += 2 as ::core::ffi::c_int
            * (hold - (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int);
    }
    if (*item).iqty as ::core::ffi::c_int != 0
        && hold >= (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize) = borg_calc_blows(item);
        *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) +=
            (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_BRAVERY_30 as ::core::ffi::c_int,
    ) {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= 30 as ::core::ffi::c_int
        {
            *borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) = true_0;
        }
    }
    *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) += borg_adj_dex_dis[*borg
        .trait_0
        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) += borg_adj_int_dis[*borg
        .trait_0
        .offset(BI_INT_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) += borg_adj_int_dev[*borg
        .trait_0
        .offset(BI_INT_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) += borg_adj_wis_sav[*borg
        .trait_0
        .offset(BI_WIS_INDEX as ::core::ffi::c_int as isize)
        as usize];
    *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_DISARM_PHYS as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_DISARM_MAGIC as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_DEVICE as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_SAVE as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_STEALTH as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SRCH as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_SEARCH as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_THN as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_TO_HIT_MELEE as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_THB as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_TO_HIT_BOW as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_THT as ::core::ffi::c_int as isize) += (*cb_ptr).x_skills
        [SKILL_TO_HIT_THROW as ::core::ffi::c_int as usize]
        * *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        / 10 as ::core::ffi::c_int;
    if *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) > 30 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) = 30 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) < 0 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) < 1 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
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
        *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) -= 20 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) += 8 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) =
            *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) * 95 as ::core::ffi::c_int
                / 100 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*player).state.pflags as *mut bitflag,
        (PF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        PF_BLESS_WEAPON as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        && ((*item).tval as ::core::ffi::c_int == TV_HAFTED as ::core::ffi::c_int
            || flag_has(
                &raw mut (*item).flags as *mut bitflag,
                (OF_MAX as ::core::ffi::c_int as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    )
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(
                        (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                    ),
                OF_BLESSED as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0)
    {
        *borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize) += 2 as ::core::ffi::c_int;
        *borg.trait_0.offset(BI_TODAM as ::core::ffi::c_int as isize) += 2 as ::core::ffi::c_int;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !cursed_nonartifact(item) {
                    if !(i == (*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize)
                            < 3 as ::core::ffi::c_int
                        && ((*item).art_idx == 0 && (*item).ego_idx == 0))
                    {
                        if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            != 0
                            || *borg
                                .trait_0
                                .offset(BI_AENCH_SWEP as ::core::ffi::c_int as isize)
                                >= 1 as ::core::ffi::c_int
                        {
                            if ((*item).to_h as ::core::ffi::c_int)
                                < *borg_cfg
                                    .offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize) +=
                                    *borg_cfg
                                        .offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                                        - (*item).to_h as ::core::ffi::c_int;
                            }
                            if ((*item).to_d as ::core::ffi::c_int)
                                < *borg_cfg
                                    .offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize) +=
                                    *borg_cfg
                                        .offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                                        - (*item).to_d as ::core::ffi::c_int;
                            }
                        } else {
                            if ((*item).to_h as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                                *borg
                                    .trait_0
                                    .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize) +=
                                    8 as ::core::ffi::c_int - (*item).to_h as ::core::ffi::c_int;
                            }
                            if ((*item).to_d as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                                *borg
                                    .trait_0
                                    .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize) +=
                                    8 as ::core::ffi::c_int - (*item).to_d as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
    while i <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).ident {
                if !cursed_nonartifact(item) {
                    if borg_spell_legal_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                        || *borg
                            .trait_0
                            .offset(BI_AENCH_SARM as ::core::ffi::c_int as isize)
                            >= 1 as ::core::ffi::c_int
                    {
                        if ((*item).to_a as ::core::ffi::c_int)
                            < *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                        {
                            *borg
                                .trait_0
                                .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize) +=
                                *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
                                    - (*item).to_a as ::core::ffi::c_int;
                        }
                    } else if ((*item).to_a as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                        *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize) +=
                            8 as ::core::ffi::c_int - (*item).to_a as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    if borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) = true_0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && borg_spell_legal(SENSE_INVISIBLE) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) = true_0;
    }
    if *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) >= 100 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) = true_0;
    }
    if *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) >= 100 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0
    {
        *borg
            .trait_0
            .offset(BI_RBLIND as ::core::ffi::c_int as isize) = true_0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        borg_notice_ammo(i);
        i += 1;
    }
}
unsafe extern "C" fn borg_notice_inventory() {
    let mut i: ::core::ffi::c_int = 0;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        borg.need_statgain[i as usize] = false_0 != 0;
        borg.amt_statgain[i as usize] = 0 as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        borg.amt_book[i as usize] = 0 as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
    {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item).iqty == 0 {
            let ref mut fresh0 = *borg.trait_0.offset(BI_EMPTY as ::core::ffi::c_int as isize);
            *fresh0 += 1;
        } else if borg_is_ammo((*item).tval as ::core::ffi::c_int) {
            borg_notice_ammo(i);
        } else {
            *borg
                .trait_0
                .offset(BI_WEIGHT as ::core::ffi::c_int as isize) +=
                borg_item_weight(item) as ::core::ffi::c_int;
            if borg_item_note_needs_id(item) {
                *borg
                    .trait_0
                    .offset(BI_ALL_NEED_ID as ::core::ffi::c_int as isize) +=
                    1 as ::core::ffi::c_int;
            }
            if (*item).uncursable {
                *borg
                    .trait_0
                    .offset(BI_WHERE_CURSED as ::core::ffi::c_int as isize) |=
                    BORG_INVEN as ::core::ffi::c_int;
                if *borg
                    .trait_0
                    .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize)
                    == 0
                {
                    *borg
                        .trait_0
                        .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize) =
                        i + 1 as ::core::ffi::c_int;
                }
            }
            if (*item).aware {
                *borg.has.offset((*item).kind as isize) += (*item).iqty as ::core::ffi::c_int;
                if !cursed_nonartifact(item) {
                    let mut current_block_187: u64;
                    match (*item).tval as ::core::ffi::c_int {
                        30 | 31 | 32 | 33 | 34 => {
                            if !obj_kind_can_browse(
                                k_info.offset((*item).kind as isize) as *mut object_kind
                            ) {
                                current_block_187 = 2167674248514145403;
                            } else {
                                borg.amt_book[borg_get_book_num((*item).sval as ::core::ffi::c_int)
                                    as usize] = (borg.amt_book
                                    [borg_get_book_num((*item).sval as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_int
                                    + (*item).iqty as ::core::ffi::c_int)
                                    as int16_t;
                                current_block_187 = 2167674248514145403;
                            }
                        }
                        29 => {
                            if (*item).sval as ::core::ffi::c_int == sv_mush_purging
                                || (*item).sval as ::core::ffi::c_int == sv_mush_restoring
                                || (*item).sval as ::core::ffi::c_int == sv_mush_cure_mind
                            {
                                if *borg_cfg
                                    .offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize)
                                    != 0
                                    && *borg
                                        .trait_0
                                        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                        < *borg_cfg.offset(
                                            BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize,
                                        )
                                {
                                    current_block_187 = 2167674248514145403;
                                } else {
                                    current_block_187 = 8693738493027456495;
                                }
                            } else {
                                current_block_187 = 8693738493027456495;
                            }
                            match current_block_187 {
                                2167674248514145403 => {}
                                _ => {
                                    if (*item).sval as ::core::ffi::c_int == sv_mush_second_sight
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_emergency
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_terror
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_stoneskin
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_debility
                                        || (*item).sval as ::core::ffi::c_int == sv_mush_sprinting
                                    {
                                        if *borg_cfg.offset(
                                            BORG_MUNCHKIN_START as ::core::ffi::c_int as isize,
                                        ) != 0
                                            && *borg
                                                .trait_0
                                                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                                                >= *borg_cfg.offset(
                                                    BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int
                                                        as isize,
                                                )
                                        {
                                            *borg.trait_0.offset(
                                                BI_ASHROOM as ::core::ffi::c_int as isize,
                                            ) += (*item).iqty as ::core::ffi::c_int;
                                        }
                                    }
                                    current_block_187 = 18153031941552419006;
                                }
                            }
                        }
                        28 => {
                            current_block_187 = 18153031941552419006;
                        }
                        26 => {
                            if (*item).sval as ::core::ffi::c_int == sv_potion_healing {
                                *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_star_healing {
                                *borg
                                    .trait_0
                                    .offset(BI_AEZHEAL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_life {
                                *borg.trait_0.offset(BI_ALIFE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_critical
                            {
                                *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_serious {
                                *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_light {
                                *borg.trait_0.offset(BI_ACLW as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_cure_poison {
                                *borg
                                    .trait_0
                                    .offset(BI_ACUREPOIS as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_heat {
                                *borg
                                    .trait_0
                                    .offset(BI_ARESHEAT as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_cold {
                                *borg
                                    .trait_0
                                    .offset(BI_ARESCOLD as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_resist_pois {
                                *borg
                                    .trait_0
                                    .offset(BI_ARESPOIS as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_str {
                                borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_int {
                                borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_wis {
                                borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_dex {
                                borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_con {
                                borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_inc_all {
                                borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                                borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] =
                                    (borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        + (*item).iqty as ::core::ffi::c_int)
                                        as int16_t;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_restore_life {
                                *borg
                                    .trait_0
                                    .offset(BI_HASFIXEXP as ::core::ffi::c_int as isize) = true_0;
                            } else if (*item).sval as ::core::ffi::c_int == sv_potion_speed {
                                *borg
                                    .trait_0
                                    .offset(BI_ASPEED as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        25 => {
                            if (*item).sval as ::core::ffi::c_int == sv_scroll_identify {
                                *borg.trait_0.offset(BI_AID as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_recharging {
                                *borg
                                    .trait_0
                                    .offset(BI_ARECHARGE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_phase_door {
                                *borg
                                    .trait_0
                                    .offset(BI_APHASE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport {
                                *borg
                                    .trait_0
                                    .offset(BI_ATELEPORT as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_word_of_recall
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_RECALL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_armor
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_star_enchant_armor
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AENCH_SARM as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_enchant_weapon_to_hit
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_enchant_weapon_to_dam
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_star_enchant_weapon
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AENCH_SWEP as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_protection_from_evil
                            {
                                *borg.trait_0.offset(BI_APFE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_rune_of_protection
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AGLYPH as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_scroll_teleport_level
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                                *borg
                                    .trait_0
                                    .offset(BI_ATELEPORT as ::core::ffi::c_int as isize) +=
                                    1 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int
                                == sv_scroll_mass_banishment
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_AMASSBAN as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        24 => {
                            if (*item).sval as ::core::ffi::c_int == sv_rod_recall {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_RECALL as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 100 as ::core::ffi::c_int;
                                } else {
                                    *borg
                                        .trait_0
                                        .offset(BI_RECALL as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_detection {
                                *borg
                                    .trait_0
                                    .offset(BI_ADETTRAP as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                                *borg
                                    .trait_0
                                    .offset(BI_ADETDOOR as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                                *borg
                                    .trait_0
                                    .offset(BI_ADETEVIL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_illumination {
                                *borg.trait_0.offset(BI_ALITE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_speed {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_ASPEED as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 100 as ::core::ffi::c_int;
                                } else {
                                    *borg
                                        .trait_0
                                        .offset(BI_ASPEED as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_mapping {
                                *borg
                                    .trait_0
                                    .offset(BI_AMAGICMAP as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_healing {
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_AHEAL as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int
                                            * 3 as ::core::ffi::c_int;
                                } else {
                                    *borg
                                        .trait_0
                                        .offset(BI_AHEAL as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_light
                                || (*item).sval as ::core::ffi::c_int == sv_rod_fire_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_elec_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_cold_bolt
                                || (*item).sval as ::core::ffi::c_int == sv_rod_acid_bolt
                            {
                                *borg.trait_0.offset(BI_AROD1 as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_rod_drain_life
                                || (*item).sval as ::core::ffi::c_int == sv_rod_fire_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_elec_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_cold_ball
                                || (*item).sval as ::core::ffi::c_int == sv_rod_acid_ball
                            {
                                *borg.trait_0.offset(BI_AROD2 as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        23 => {
                            if (*item).sval as ::core::ffi::c_int == sv_wand_teleport_away {
                                *borg
                                    .trait_0
                                    .offset(BI_ATPORTOTHER as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_stinking_cloud
                                && *borg
                                    .trait_0
                                    .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                                    < 30 as ::core::ffi::c_int
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_GOOD_W_CHG as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_magic_missile
                                && *borg
                                    .trait_0
                                    .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                                    < 30 as ::core::ffi::c_int
                            {
                                *borg
                                    .trait_0
                                    .offset(BI_GOOD_W_CHG as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            if (*item).sval as ::core::ffi::c_int == sv_wand_annihilation {
                                *borg
                                    .trait_0
                                    .offset(BI_GOOD_W_CHG as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        22 => {
                            if (*item).sval as ::core::ffi::c_int == sv_staff_teleportation {
                                *borg
                                    .trait_0
                                    .offset(BI_AESCAPE as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                                if borg_activate_failure(
                                    (*item).tval as ::core::ffi::c_int,
                                    (*item).sval as ::core::ffi::c_int,
                                ) < 500 as ::core::ffi::c_int
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_AESCAPE as ::core::ffi::c_int as isize) +=
                                        (*item).pval as ::core::ffi::c_int;
                                }
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_speed {
                                *borg
                                    .trait_0
                                    .offset(BI_ASPEED as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_healing {
                                *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_the_magi {
                                *borg
                                    .trait_0
                                    .offset(BI_ASTFMAGI as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_destruction {
                                *borg
                                    .trait_0
                                    .offset(BI_ASTFDEST as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_power {
                                *borg
                                    .trait_0
                                    .offset(BI_GOOD_S_CHG as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            } else if (*item).sval as ::core::ffi::c_int == sv_staff_holiness {
                                *borg
                                    .trait_0
                                    .offset(BI_GOOD_S_CHG as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                                *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize) +=
                                    (*item).pval as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        27 => {
                            if (*borg_items.offset(
                                ((*z_info).pack_size as ::core::ffi::c_int
                                    + 5 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .sval as ::core::ffi::c_int
                                == sv_light_lantern
                            {
                                *borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        19 => {
                            if (*item).sval as ::core::ffi::c_int == sv_light_torch
                                && (*item).timeout as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                                && (*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 5 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .sval as ::core::ffi::c_int
                                    == sv_light_torch
                                && (*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 5 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .iqty as ::core::ffi::c_int
                                    != 0
                            {
                                *borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                            }
                            current_block_187 = 2167674248514145403;
                        }
                        6 => {
                            if (*item).value <= 0 as int32_t {
                                current_block_187 = 2167674248514145403;
                            } else if (*item).cursed {
                                current_block_187 = 2167674248514145403;
                            } else if *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize)
                                < BORG_DIG
                            {
                                current_block_187 = 2167674248514145403;
                            } else {
                                *borg
                                    .trait_0
                                    .offset(BI_ADIGGER as ::core::ffi::c_int as isize) +=
                                    (*item).iqty as ::core::ffi::c_int;
                                current_block_187 = 2167674248514145403;
                            }
                        }
                        7 | 8 | 9 | _ => {
                            current_block_187 = 2167674248514145403;
                        }
                    }
                    match current_block_187 {
                        18153031941552419006 => {
                            if !(borg_obj_has_effect(
                                (*item).kind,
                                EF_CRUNCH as ::core::ffi::c_int,
                                -(1 as ::core::ffi::c_int),
                            ) as ::core::ffi::c_int
                                != 0
                                || borg_obj_has_effect(
                                    (*item).kind,
                                    EF_TIMED_INC as ::core::ffi::c_int,
                                    TMD_CONFUSED as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int
                                    != 0)
                            {
                                if (*item).tval as ::core::ffi::c_int
                                    == TV_MUSHROOM as ::core::ffi::c_int
                                {
                                    if borg_obj_has_effect(
                                        (*item).kind,
                                        EF_NOURISH as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                    ) {
                                        *borg
                                            .trait_0
                                            .offset(BI_FOOD_LO as ::core::ffi::c_int as isize) +=
                                            (*item).iqty as ::core::ffi::c_int;
                                    }
                                } else if (*item).sval as ::core::ffi::c_int == sv_food_apple
                                    || (*item).sval as ::core::ffi::c_int == sv_food_handful
                                    || (*item).sval as ::core::ffi::c_int == sv_food_slime_mold
                                    || (*item).sval as ::core::ffi::c_int == sv_food_pint
                                    || (*item).sval as ::core::ffi::c_int == sv_food_sip
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_FOOD_LO as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int;
                                } else if (*item).sval as ::core::ffi::c_int == sv_food_ration
                                    || (*item).sval as ::core::ffi::c_int == sv_food_slice
                                    || (*item).sval as ::core::ffi::c_int == sv_food_honey_cake
                                    || (*item).sval as ::core::ffi::c_int == sv_food_waybread
                                    || (*item).sval as ::core::ffi::c_int == sv_food_draught
                                {
                                    *borg
                                        .trait_0
                                        .offset(BI_FOOD_HI as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_POISONED as ::core::ffi::c_int,
                                ) {
                                    *borg
                                        .trait_0
                                        .offset(BI_ACUREPOIS as ::core::ffi::c_int as isize) +=
                                        (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_CONFUSED as ::core::ffi::c_int,
                                ) {
                                    *borg.trait_0.offset(
                                        BI_FOOD_CURE_CONF as ::core::ffi::c_int as isize,
                                    ) += (*item).iqty as ::core::ffi::c_int;
                                }
                                if borg_obj_has_effect(
                                    (*item).kind,
                                    EF_CURE as ::core::ffi::c_int,
                                    TMD_BLIND as ::core::ffi::c_int,
                                ) {
                                    *borg.trait_0.offset(
                                        BI_FOOD_CURE_BLIND as ::core::ffi::c_int as isize,
                                    ) += (*item).iqty as ::core::ffi::c_int;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1;
    }
    if *borg.has.offset(kv_flask_oil as isize) != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
    {
        if *borg.has.offset(kv_flask_oil as isize) < 15 as ::core::ffi::c_int {
            *borg
                .trait_0
                .offset(BI_AMISSILES as ::core::ffi::c_int as isize) +=
                *borg.has.offset(kv_flask_oil as isize);
        } else {
            *borg
                .trait_0
                .offset(BI_AMISSILES as ::core::ffi::c_int as isize) += 15 as ::core::ffi::c_int;
        }
    }
    if borg_spell_legal_fail(REMOVE_HUNGER, 80 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(HERBAL_CURING, 80 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(IDENTIFY_RUNE) {
        *borg.trait_0.offset(BI_AID as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(FIND_TRAPS_DOORS_STAIRS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECTION) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ADETTRAP as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(REVEAL_MONSTERS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_LIFE) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_EVIL) as ::core::ffi::c_int != 0
        || borg_spell_legal(READ_MINDS) as ::core::ffi::c_int != 0
        || borg_spell_legal(DETECT_MONSTERS) as ::core::ffi::c_int != 0
        || borg_spell_legal(SEEK_BATTLE) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ADETEVIL as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(DETECTION) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enlightenment, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_clairvoyance, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ADETDOOR as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_ADETTRAP as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_ADETEVIL as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(SENSE_INVISIBLE) {
        *borg.trait_0.offset(BI_DINV as ::core::ffi::c_int as isize) = true_0;
    }
    if borg_spell_legal(SENSE_SURROUNDINGS) as ::core::ffi::c_int != 0
        || borg_equips_item(act_detect_all, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_mapping, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ADETDOOR as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_ADETTRAP as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_AMAGICMAP as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(LIGHT_ROOM) as ::core::ffi::c_int != 0
        || borg_equips_item(act_light, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_illumination, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(CALL_LIGHT) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_ALITE as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(PROTECTION_FROM_EVIL) as ::core::ffi::c_int != 0
        || borg_equips_item(act_protevil, false_0 != 0) as ::core::ffi::c_int != 0
        || *borg.has.offset(kv_staff_holiness as isize) != 0
        || borg_equips_item(act_staff_holy, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_APFE as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(GLYPH_OF_WARDING) as ::core::ffi::c_int != 0
        || borg_equips_item(act_glyph, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_AGLYPH as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(FIND_TRAPS_DOORS_STAIRS) {
        *borg
            .trait_0
            .offset(BI_ADETDOOR as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_ADETTRAP as ::core::ffi::c_int as isize) = 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(ENCHANT_WEAPON, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_weapon, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_AENCH_SWEP as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_enchant_tohit, false_0 != 0) {
        *borg
            .trait_0
            .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_enchant_todam, false_0 != 0) {
        *borg
            .trait_0
            .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_firebrand, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(BRAND_AMMUNITION, 65 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0
    {
        *borg
            .trait_0
            .offset(BI_ABRAND as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(ENCHANT_ARMOUR, 65 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_armor, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_enchant_armor2, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_AENCH_SARM as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TURN_STONE_TO_MUD, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_equips_item(act_stone_to_mud, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_ring(sv_ring_digging) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ADIGGER as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(WORD_OF_RECALL, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 100 as ::core::ffi::c_int
            && borg_spell_legal(WORD_OF_RECALL) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_RECALL as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_recall, false_0 != 0) {
        *borg
            .trait_0
            .offset(BI_RECALL as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_LEVEL, 20 as ::core::ffi::c_int) {
        *borg
            .trait_0
            .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(PHASE_DOOR, 3 as ::core::ffi::c_int) {
        *borg
            .trait_0
            .offset(BI_APHASE as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_tele_phase, false_0 != 0) {
        *borg
            .trait_0
            .offset(BI_APHASE as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_SELF, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(PORTAL, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(SHADOW_SHIFT, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        || borg_spell_legal_fail(DIMENSION_DOOR, 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ATELEPORT as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_tele_long, false_0 != 0) {
        *borg
            .trait_0
            .offset(BI_AESCAPE as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset(BI_ATELEPORT as ::core::ffi::c_int as isize) += 1 as ::core::ffi::c_int;
    }
    if borg_spell_legal_fail(TELEPORT_OTHER, 40 as ::core::ffi::c_int) {
        *borg
            .trait_0
            .offset(BI_ATPORTOTHER as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(HOLY_WORD) {
        *borg
            .trait_0
            .offset(BI_AHWORD as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_spell_legal(HASTE_SELF) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste1, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_haste2, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ASPEED as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_light, false_0 != 0) {
        *borg.trait_0.offset(BI_ACLW as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_serious, false_0 != 0) {
        *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_critical, false_0 != 0) {
        *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_full, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_full2, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal1, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal2, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_heal3, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(HEALING) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_exp, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_st_lev, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_life, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_HASFIXEXP as ::core::ffi::c_int as isize) = true_0;
    }
    if borg_spell_legal(REMEMBRANCE) as ::core::ffi::c_int != 0
        || borg_equips_item(act_cure_nonorlybig, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_exp, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_st_lev, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_equips_item(act_restore_life, false_0 != 0) as ::core::ffi::c_int != 0
    {
        *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) = true_0;
    }
    if borg_equips_item(act_recharge, false_0 != 0) as ::core::ffi::c_int != 0
        || borg_spell_legal(RECHARGING) as ::core::ffi::c_int != 0
    {
        *borg
            .trait_0
            .offset(BI_ARECHARGE as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if flag_has(
        &raw mut (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
        .flags as *mut bitflag,
        (OF_MAX as ::core::ffi::c_int as size_t)
            .wrapping_add((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t))
            .wrapping_sub(1 as size_t)
            .wrapping_div((::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t)),
        OF_NO_FUEL as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        != 0
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
    {
        *borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize) += 1000 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize)
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if *borg.trait_0.offset(BI_CINT as ::core::ffi::c_int as isize)
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if *borg.trait_0.offset(BI_CWIS as ::core::ffi::c_int as isize)
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if *borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize)
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if *borg.trait_0.offset(BI_CCON as ::core::ffi::c_int as isize)
        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize] = true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize)
        == 0
    {
        *borg
            .trait_0
            .offset(BI_HASFIXEXP as ::core::ffi::c_int as isize) = true_0;
    }
    *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) += *borg
        .trait_0
        .offset(BI_FOOD_HI as ::core::ffi::c_int as isize);
    *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) += *borg
        .trait_0
        .offset(BI_FOOD_LO as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
        && *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
            >= 1000 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) -= 1000 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice(mut notice_swap: bool) {
    memset(
        borg.has as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*z_info).k_max as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    memset(
        borg.trait_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (BI_MAX as ::core::ffi::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    memset(
        borg.activation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*z_info).act_max as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) = 110 as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
    *borg
        .trait_0
        .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize) = 4 as ::core::ffi::c_int;
    borg_notice_player();
    if borg_do_spell {
        borg_cheat_spells();
        borg_do_spell = false_0 != 0;
    }
    borg_notice_equipment();
    borg_notice_inventory();
    if notice_swap {
        borg_notice_weapon_swap();
        borg_notice_armour_swap();
    }
    *borg
        .trait_0
        .offset(BI_SRACID as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_acid as ::core::ffi::c_int != 0
            || weapon_swap_resist_acid as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRELEC as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_elec as ::core::ffi::c_int != 0
            || weapon_swap_resist_elec as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRFIRE as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_fire as ::core::ffi::c_int != 0
            || weapon_swap_resist_fire as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRCOLD as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_cold as ::core::ffi::c_int != 0
            || weapon_swap_resist_cold as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRPOIS as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_pois as ::core::ffi::c_int != 0
            || weapon_swap_resist_pois as ::core::ffi::c_int != 0
            || borg_spell_legal_fail(RESIST_POISON, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRFEAR as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_fear as ::core::ffi::c_int != 0
            || weapon_swap_resist_fear as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRLITE as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_light as ::core::ffi::c_int != 0
            || weapon_swap_resist_light as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRDARK as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_dark as ::core::ffi::c_int != 0
            || weapon_swap_resist_dark as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRBLIND as ::core::ffi::c_int as isize) = (*borg
        .trait_0
        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
        != 0
        || armour_swap_resist_blind as ::core::ffi::c_int != 0
        || weapon_swap_resist_blind as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRCONF as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_conf as ::core::ffi::c_int != 0
            || weapon_swap_resist_conf as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SRSND as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_sound as ::core::ffi::c_int != 0
            || weapon_swap_resist_sound as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRSHRD as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_shard as ::core::ffi::c_int != 0
            || weapon_swap_resist_shard as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRNXUS as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_nexus as ::core::ffi::c_int != 0
            || weapon_swap_resist_nexus as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRNTHR as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_neth as ::core::ffi::c_int != 0
            || weapon_swap_resist_neth as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SRKAOS as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_chaos as ::core::ffi::c_int != 0
            || weapon_swap_resist_chaos as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_SRDIS as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0
            || armour_swap_resist_disen as ::core::ffi::c_int != 0
            || weapon_swap_resist_disen as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SHLIFE as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0
            || armour_swap_hold_life as ::core::ffi::c_int != 0
            || weapon_swap_hold_life as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_SFRACT as ::core::ffi::c_int as isize) =
        (*borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0
            || armour_swap_free_act as ::core::ffi::c_int != 0
            || weapon_swap_free_act as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize) = borg_adj_str_wgt[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        as usize]
        * 100 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) -= (*borg
            .trait_0
            .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
            - *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                / 2 as ::core::ffi::c_int)
            / (*borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                / 10 as ::core::ffi::c_int);
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) > 199 as ::core::ffi::c_int {
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) = 199 as ::core::ffi::c_int;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) > 110 as ::core::ffi::c_int {
        borg_game_ratio = (100000 as ::core::ffi::c_int
            / ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                - 110 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int)) as int16_t;
    } else {
        borg_game_ratio = 1000 as int16_t;
    }
    *borg
        .trait_0
        .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize) = false_0;
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 99 as ::core::ffi::c_int
    {
        borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
        let mut total_big_heal: ::core::ffi::c_int = *borg.has.offset(kv_potion_healing as isize);
        total_big_heal += *borg
            .trait_0
            .offset(BI_AEZHEAL as ::core::ffi::c_int as isize);
        total_big_heal += *borg.trait_0.offset(BI_ALIFE as ::core::ffi::c_int as isize);
        total_big_heal += num_heal_true as ::core::ffi::c_int;
        total_big_heal += num_ezheal_true as ::core::ffi::c_int;
        total_big_heal += num_life_true as ::core::ffi::c_int;
        if total_big_heal < 30 as ::core::ffi::c_int
            || num_speed as ::core::ffi::c_int
                + *borg
                    .trait_0
                    .offset(BI_ASPEED as ::core::ffi::c_int as isize)
                < 15 as ::core::ffi::c_int
        {
            *borg
                .trait_0
                .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize) = true_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_player() {
    let mut i: ::core::ffi::c_int = 0;
    *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) =
        (*(*player).class).cidx as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_ISFIXLEV as ::core::ffi::c_int as isize) = false_0;
    if ((*player).lev as ::core::ffi::c_int) < (*player).max_lev as ::core::ffi::c_int {
        *borg
            .trait_0
            .offset(BI_ISFIXLEV as ::core::ffi::c_int as isize) = true_0;
    }
    *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize) = (*player).lev as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize) =
        (*player).max_lev as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) =
        (*player).total_winner as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize) = false_0;
    *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize) = (*player).depth as ::core::ffi::c_int;
    *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize) =
        (*player).max_depth as ::core::ffi::c_int;
    if (*player).exp < (*player).max_exp {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 50 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
        {
            *borg
                .trait_0
                .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize) = true_0;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 50 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 1 as ::core::ffi::c_int
        {
            *borg
                .trait_0
                .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize) = false_0;
        }
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            != 50 as ::core::ffi::c_int
        {
            *borg
                .trait_0
                .offset(BI_ISFIXEXP as ::core::ffi::c_int as isize) = true_0;
        }
    }
    *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) =
        (*player).au as ::core::ffi::c_int;
    if borg.goal.recalling == 0 && (*player).word_recall as ::core::ffi::c_int != 0 {
        borg.goal.recalling =
            (*player).word_recall as ::core::ffi::c_int * 1000 as ::core::ffi::c_int;
    }
    if borg.goal.recalling != 0 && (*player).word_recall == 0 {
        borg.goal.recalling = 0 as ::core::ffi::c_int;
    }
    if !borg.temp.prot_from_evil
        && *(*player)
            .timed
            .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        borg.temp.prot_from_evil = if *(*player)
            .timed
            .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != 0
        {
            true_0
        } else {
            false_0
        } != 0;
    }
    if !borg.temp.fast
        && (*(*player)
            .timed
            .offset(TMD_FAST as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
            || *(*player)
                .timed
                .offset(TMD_SPRINT as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
            || *(*player)
                .timed
                .offset(TMD_TERROR as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0)
    {
        borg.temp.fast = if *(*player)
            .timed
            .offset(TMD_FAST as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != 0
            || *(*player)
                .timed
                .offset(TMD_SPRINT as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
            || *(*player)
                .timed
                .offset(TMD_TERROR as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != 0
        {
            true_0
        } else {
            false_0
        } != 0;
    }
    borg.temp.res_acid = if *(*player)
        .timed
        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.res_elec = if *(*player)
        .timed
        .offset(TMD_OPP_ELEC as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.res_fire = if *(*player)
        .timed
        .offset(TMD_OPP_FIRE as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.res_cold = if *(*player)
        .timed
        .offset(TMD_OPP_COLD as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.res_pois = if *(*player)
        .timed
        .offset(TMD_OPP_POIS as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.bless = if *(*player)
        .timed
        .offset(TMD_BLESSED as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.shield = if *(*player)
        .timed
        .offset(TMD_SHIELD as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
        || *(*player)
            .timed
            .offset(TMD_STONESKIN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.fastcast = if *(*player)
        .timed
        .offset(TMD_FASTCAST as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.hero = if *(*player)
        .timed
        .offset(TMD_HERO as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.berserk = if *(*player)
        .timed
        .offset(TMD_SHERO as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.regen = if *(*player)
        .timed
        .offset(TMD_HEAL as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.venom = if *(*player)
        .timed
        .offset(TMD_ATT_POIS as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    borg.temp.smite_evil = if *(*player)
        .timed
        .offset(TMD_ATT_EVIL as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != 0
    {
        true_0
    } else {
        false_0
    } != 0;
    if borg.see_inv == 0
        && *(*player)
            .timed
            .offset(TMD_SINVIS as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        borg.see_inv = 1000 as int16_t;
    }
    *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) =
        (*player).chp as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) =
        (*player).mhp as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) =
        (*player).csp as ::core::ffi::c_int;
    *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) =
        (*player).msp as ::core::ffi::c_int;
    let ref mut fresh5 = *borg
        .trait_0
        .offset(BI_ISGORGED as ::core::ffi::c_int as isize);
    *fresh5 = false_0;
    let ref mut fresh6 = *borg
        .trait_0
        .offset(BI_ISFULL as ::core::ffi::c_int as isize);
    *fresh6 = *fresh5;
    let ref mut fresh7 = *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize);
    *fresh7 = *fresh6;
    *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize) = *fresh7;
    let ref mut fresh8 = *borg
        .trait_0
        .offset(BI_ISPOISONED as ::core::ffi::c_int as isize);
    *fresh8 = false_0;
    let ref mut fresh9 = *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize);
    *fresh9 = *fresh8;
    let ref mut fresh10 = *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize);
    *fresh10 = *fresh9;
    *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize) = *fresh10;
    let ref mut fresh11 = *borg
        .trait_0
        .offset(BI_ISSTUDY as ::core::ffi::c_int as isize);
    *fresh11 = false_0;
    let ref mut fresh12 = *borg
        .trait_0
        .offset(BI_ISIMAGE as ::core::ffi::c_int as isize);
    *fresh12 = *fresh11;
    let ref mut fresh13 = *borg
        .trait_0
        .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize);
    *fresh13 = *fresh12;
    let ref mut fresh14 = *borg
        .trait_0
        .offset(BI_ISSTUN as ::core::ffi::c_int as isize);
    *fresh14 = *fresh13;
    *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) = *fresh14;
    *borg
        .trait_0
        .offset(BI_ISPARALYZED as ::core::ffi::c_int as isize) = false_0;
    *borg
        .trait_0
        .offset(BI_ISFORGET as ::core::ffi::c_int as isize) = false_0;
    if (*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_WEAK
    {
        let ref mut fresh15 = *borg
            .trait_0
            .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize);
        *fresh15 = true_0;
        *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize) = *fresh15;
    } else if (*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_HUNGRY
    {
        *borg
            .trait_0
            .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize) = true_0;
    } else if !((*(*player)
        .timed
        .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        < PY_FOOD_FULL)
    {
        if (*(*player)
            .timed
            .offset(TMD_FOOD as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < PY_FOOD_MAX
        {
            *borg
                .trait_0
                .offset(BI_ISFULL as ::core::ffi::c_int as isize) = true_0;
        } else {
            let ref mut fresh16 = *borg
                .trait_0
                .offset(BI_ISFULL as ::core::ffi::c_int as isize);
            *fresh16 = true_0;
            *borg
                .trait_0
                .offset(BI_ISGORGED as ::core::ffi::c_int as isize) = *fresh16;
        }
    }
    if *(*player)
        .timed
        .offset(TMD_BLIND as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISBLIND as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_AFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_POISONED as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_CUT as ::core::ffi::c_int as isize)
        != 0
    {
        *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0
        && *(*player)
            .timed
            .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            <= 50 as ::core::ffi::c_int
    {
        *borg
            .trait_0
            .offset(BI_ISSTUN as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        > 50 as ::core::ffi::c_int
    {
        *borg
            .trait_0
            .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_PARALYZED as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        > 50 as ::core::ffi::c_int
    {
        *borg
            .trait_0
            .offset(BI_ISPARALYZED as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_IMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize) = true_0;
    }
    if *(*player)
        .timed
        .offset(TMD_AMNESIA as ::core::ffi::c_int as isize)
        != 0
    {
        *borg
            .trait_0
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize) = true_0;
    }
    if (*(*player).upkeep).new_spells != 0 {
        *borg
            .trait_0
            .offset(BI_ISSTUDY as ::core::ffi::c_int as isize) = true_0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        *borg
            .trait_0
            .offset((BI_ISFIXSTR as ::core::ffi::c_int + i) as isize) =
            (((*player).stat_cur[(STAT_STR as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int)
                < (*player).stat_max[(STAT_STR as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int) as ::core::ffi::c_int;
        *borg
            .trait_0
            .offset((BI_CSTR as ::core::ffi::c_int + i) as isize) =
            (*player).stat_cur[(STAT_STR as ::core::ffi::c_int + i) as usize] as ::core::ffi::c_int;
        i += 1;
    }
    *borg
        .trait_0
        .offset(BI_SAURON_DEAD as ::core::ffi::c_int as isize) =
        *borg_race_death.offset(borg.id.sauron as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_trait_init() {
    borg_trait_free();
    borg.has = mem_zalloc(
        ((*z_info).k_max as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    borg.trait_0 = mem_zalloc(
        (BI_MAX as ::core::ffi::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    borg.activation = mem_zalloc(
        ((*z_info).act_max as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_trait_free() {
    if !borg.has.is_null() {
        mem_free(borg.has as *mut ::core::ffi::c_void);
    }
    borg.has = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if !borg.trait_0.is_null() {
        mem_free(borg.trait_0 as *mut ::core::ffi::c_void);
    }
    borg.trait_0 = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if !borg.activation.is_null() {
        mem_free(borg.activation as *mut ::core::ffi::c_void);
    }
    borg.activation = ::core::ptr::null_mut::<::core::ffi::c_int>();
}
pub const BORG_DIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
