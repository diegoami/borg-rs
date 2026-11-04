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
    static mut e_info: *mut ego_item;
    static mut player: *mut player;
    static adj_str_hold: [::core::ffi::c_int; 38];
    fn spell_chance(spell_index: ::core::ffi::c_int) -> int16_t;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    fn borg_item_weight(item: *mut borg_item) -> int16_t;
    static mut weapon_swap_value: int32_t;
    static mut armour_swap_value: int32_t;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    fn borg_power_dynamic() -> int32_t;
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
    static mut act_shroom_purging: ::core::ffi::c_int;
    static mut act_shroom_sprinting: ::core::ffi::c_int;
    static mut act_shroom_debility: ::core::ffi::c_int;
    static mut act_shroom_stone: ::core::ffi::c_int;
    static mut act_shroom_terror: ::core::ffi::c_int;
    static mut act_shroom_emergency: ::core::ffi::c_int;
    static mut act_food_waybread: ::core::ffi::c_int;
    static mut act_drink_breath: ::core::ffi::c_int;
    static mut act_staff_holy: ::core::ffi::c_int;
    static mut act_staff_magi: ::core::ffi::c_int;
    static mut act_wand_breath: ::core::ffi::c_int;
    static mut act_wonder: ::core::ffi::c_int;
    static mut act_berserker: ::core::ffi::c_int;
    static mut act_starlight2: ::core::ffi::c_int;
    static mut act_starlight: ::core::ffi::c_int;
    static mut act_polymorph: ::core::ffi::c_int;
    static mut act_disable_traps: ::core::ffi::c_int;
    static mut act_light_line: ::core::ffi::c_int;
    static mut act_mon_scare: ::core::ffi::c_int;
    static mut act_sleep_all: ::core::ffi::c_int;
    static mut act_mon_confuse: ::core::ffi::c_int;
    static mut act_mon_slow: ::core::ffi::c_int;
    static mut act_confuse2: ::core::ffi::c_int;
    static mut act_tele_other: ::core::ffi::c_int;
    static mut act_stone_to_mud: ::core::ffi::c_int;
    static mut act_stinking_cloud: ::core::ffi::c_int;
    static mut act_arrow: ::core::ffi::c_int;
    static mut act_bizarre: ::core::ffi::c_int;
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
    static mut act_acid_bolt3: ::core::ffi::c_int;
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
    static mut act_firebrand: ::core::ffi::c_int;
    static mut act_rem_fear_pois: ::core::ffi::c_int;
    static mut act_restore_life: ::core::ffi::c_int;
    static mut act_rage_bless_resist: ::core::ffi::c_int;
    static mut act_star_ball: ::core::ffi::c_int;
    static mut act_sleepii: ::core::ffi::c_int;
    static mut act_dispel_all: ::core::ffi::c_int;
    static mut act_dispel_undead: ::core::ffi::c_int;
    static mut act_dispel_evil60: ::core::ffi::c_int;
    static mut act_dispel_evil: ::core::ffi::c_int;
    static mut act_haste2: ::core::ffi::c_int;
    static mut act_haste1: ::core::ffi::c_int;
    static mut act_haste: ::core::ffi::c_int;
    static mut act_probing: ::core::ffi::c_int;
    static mut act_clairvoyance: ::core::ffi::c_int;
    static mut act_illumination: ::core::ffi::c_int;
    static mut act_loskill: ::core::ffi::c_int;
    static mut act_losconf: ::core::ffi::c_int;
    static mut act_lossleep: ::core::ffi::c_int;
    static mut act_losslow: ::core::ffi::c_int;
    static mut act_destruction2: ::core::ffi::c_int;
    static mut act_earthquakes: ::core::ffi::c_int;
    static mut act_deep_descent: ::core::ffi::c_int;
    static mut act_recall: ::core::ffi::c_int;
    static mut act_blessing3: ::core::ffi::c_int;
    static mut act_blessing2: ::core::ffi::c_int;
    static mut act_blessing: ::core::ffi::c_int;
    static mut act_satisfy: ::core::ffi::c_int;
    static mut act_protevil: ::core::ffi::c_int;
    static mut act_banishment: ::core::ffi::c_int;
    static mut act_recharge: ::core::ffi::c_int;
    static mut act_destroy_doors: ::core::ffi::c_int;
    static mut act_glyph: ::core::ffi::c_int;
    static mut act_mapping: ::core::ffi::c_int;
    static mut act_confusing: ::core::ffi::c_int;
    static mut act_tele_level: ::core::ffi::c_int;
    static mut act_tele_long: ::core::ffi::c_int;
    static mut act_tele_phase: ::core::ffi::c_int;
    static mut act_light: ::core::ffi::c_int;
    static mut act_remove_curse2: ::core::ffi::c_int;
    static mut act_remove_curse: ::core::ffi::c_int;
    static mut act_enchant_armor2: ::core::ffi::c_int;
    static mut act_enchant_armor: ::core::ffi::c_int;
    static mut act_enchant_weapon: ::core::ffi::c_int;
    static mut act_enchant_todam: ::core::ffi::c_int;
    static mut act_enchant_tohit: ::core::ffi::c_int;
    static mut act_detect_objects: ::core::ffi::c_int;
    static mut act_detect_all: ::core::ffi::c_int;
    static mut act_detect_evil: ::core::ffi::c_int;
    static mut act_detect_invis: ::core::ffi::c_int;
    static mut act_detect_treasure: ::core::ffi::c_int;
    static mut act_resist_all: ::core::ffi::c_int;
    static mut act_resist_pois: ::core::ffi::c_int;
    static mut act_resist_cold: ::core::ffi::c_int;
    static mut act_resist_fire: ::core::ffi::c_int;
    static mut act_resist_elec: ::core::ffi::c_int;
    static mut act_resist_acid: ::core::ffi::c_int;
    static mut act_shero: ::core::ffi::c_int;
    static mut act_hero: ::core::ffi::c_int;
    static mut act_enlightenment: ::core::ffi::c_int;
    static mut act_tmd_esp: ::core::ffi::c_int;
    static mut act_tmd_sinvis: ::core::ffi::c_int;
    static mut act_tmd_infra: ::core::ffi::c_int;
    static mut act_tmd_free_act: ::core::ffi::c_int;
    static mut act_restore_st_lev: ::core::ffi::c_int;
    static mut act_restore_all: ::core::ffi::c_int;
    static mut act_restore_con: ::core::ffi::c_int;
    static mut act_restore_dex: ::core::ffi::c_int;
    static mut act_restore_wis: ::core::ffi::c_int;
    static mut act_restore_int: ::core::ffi::c_int;
    static mut act_restore_str: ::core::ffi::c_int;
    static mut act_nimbleness: ::core::ffi::c_int;
    static mut act_toughness: ::core::ffi::c_int;
    static mut act_contemplation: ::core::ffi::c_int;
    static mut act_intellect: ::core::ffi::c_int;
    static mut act_brawn: ::core::ffi::c_int;
    static mut act_restore_mana: ::core::ffi::c_int;
    static mut act_restore_exp: ::core::ffi::c_int;
    static mut act_heal3: ::core::ffi::c_int;
    static mut act_heal2: ::core::ffi::c_int;
    static mut act_heal1: ::core::ffi::c_int;
    static mut act_cure_temp: ::core::ffi::c_int;
    static mut act_cure_nonorlybig: ::core::ffi::c_int;
    static mut act_cure_full2: ::core::ffi::c_int;
    static mut act_cure_full: ::core::ffi::c_int;
    static mut act_cure_critical: ::core::ffi::c_int;
    static mut act_cure_serious: ::core::ffi::c_int;
    static mut act_cure_light: ::core::ffi::c_int;
    static mut act_cure_body: ::core::ffi::c_int;
    static mut act_cure_mind: ::core::ffi::c_int;
    static mut act_cure_confusion: ::core::ffi::c_int;
    static mut act_cure_paranoia: ::core::ffi::c_int;
    fn borg_ego_has_random_power(e_ptr: *mut ego_item) -> bool;
    static mut kv_mush_stoneskin: ::core::ffi::c_int;
    static mut kv_potion_healing: ::core::ffi::c_int;
    static mut kv_potion_restore_mana: ::core::ffi::c_int;
    static mut sv_potion_inc_str: ::core::ffi::c_int;
    static mut sv_potion_inc_int: ::core::ffi::c_int;
    static mut sv_potion_inc_wis: ::core::ffi::c_int;
    static mut sv_potion_inc_dex: ::core::ffi::c_int;
    static mut sv_potion_inc_con: ::core::ffi::c_int;
    static mut sv_potion_inc_all: ::core::ffi::c_int;
    static mut sv_scroll_enchant_armor: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_hit: ::core::ffi::c_int;
    static mut sv_scroll_enchant_weapon_to_dam: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_weapon: ::core::ffi::c_int;
    static mut sv_scroll_star_enchant_armor: ::core::ffi::c_int;
    static mut kv_scroll_mass_banishment: ::core::ffi::c_int;
    static mut kv_scroll_remove_curse: ::core::ffi::c_int;
    static mut kv_scroll_star_remove_curse: ::core::ffi::c_int;
    static mut kv_rod_recall: ::core::ffi::c_int;
    static mut kv_rod_healing: ::core::ffi::c_int;
    static mut kv_wand_stinking_cloud: ::core::ffi::c_int;
    static mut kv_wand_magic_missile: ::core::ffi::c_int;
    static mut kv_wand_annihilation: ::core::ffi::c_int;
    static mut sv_sling: ::core::ffi::c_int;
    fn borg_spell_stat() -> ::core::ffi::c_int;
    fn borg_get_spell_entry(book: ::core::ffi::c_int, what: ::core::ffi::c_int) -> *mut borg_magic;
    fn borg_prepared(depth: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_options {
    pub opt: [bool; 46],
    pub hitpoint_warn: uint8_t,
    pub lazymove_delay: uint8_t,
    pub delay_factor: uint8_t,
    pub name_suffix: uint8_t,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const STAT_MAX: C2RustUnnamed_0 = 5;
pub const STAT_CON: C2RustUnnamed_0 = 4;
pub const STAT_DEX: C2RustUnnamed_0 = 3;
pub const STAT_WIS: C2RustUnnamed_0 = 2;
pub const STAT_INT: C2RustUnnamed_0 = 1;
pub const STAT_STR: C2RustUnnamed_0 = 0;
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
pub const BI_MAXDEPTH: C2RustUnnamed_10 = 106;
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
pub const BI_CARRY: C2RustUnnamed_10 = 261;
pub const BI_WEIGHT: C2RustUnnamed_10 = 260;
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
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_10 = 242;
pub const BI_AENCH_TOD: C2RustUnnamed_10 = 235;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_10 = 241;
pub const BI_AENCH_TOH: C2RustUnnamed_10 = 234;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_10 = 240;
pub const BI_AENCH_ARM: C2RustUnnamed_10 = 237;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_10 = 158;
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
pub const BI_STR: C2RustUnnamed_10 = 0;
pub const BI_MAXCLEVEL: C2RustUnnamed_10 = 36;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_magic {
    pub name: *const ::core::ffi::c_char,
    pub status: uint8_t,
    pub effect_index: uint16_t,
    pub rating: uint8_t,
    pub level: uint8_t,
    pub power: uint8_t,
    pub sfail: uint8_t,
    pub book_offset: ::core::ffi::c_int,
    pub book: ::core::ffi::c_int,
    pub times: int32_t,
    pub spell_enum: borg_spells,
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
pub const BI_MAXSP: C2RustUnnamed_10 = 31;
pub const BI_ADIGGER: C2RustUnnamed_10 = 255;
pub const BI_GOLD: C2RustUnnamed_10 = 45;
pub const BI_EMPTY: C2RustUnnamed_10 = 262;
pub const BI_AENCH_SARM: C2RustUnnamed_10 = 238;
pub const BI_AENCH_SWEP: C2RustUnnamed_10 = 236;
pub const BI_HASFIXEXP: C2RustUnnamed_10 = 126;
pub const BI_FIRST_CURSED: C2RustUnnamed_10 = 159;
pub const BI_CCON: C2RustUnnamed_10 = 14;
pub const BI_CDEX: C2RustUnnamed_10 = 13;
pub const BI_CWIS: C2RustUnnamed_10 = 12;
pub const BI_CSTR: C2RustUnnamed_10 = 10;
pub const BI_CINT: C2RustUnnamed_10 = 11;
pub const BI_AROD2: C2RustUnnamed_10 = 252;
pub const BI_AROD1: C2RustUnnamed_10 = 251;
pub const BI_GOOD_S_CHG: C2RustUnnamed_10 = 256;
pub const BI_GOOD_W_CHG: C2RustUnnamed_10 = 257;
pub const BI_CLEVEL: C2RustUnnamed_10 = 35;
pub const BI_CLASS: C2RustUnnamed_10 = 25;
pub const BI_CDEPTH: C2RustUnnamed_10 = 105;
pub const BI_ATPORTOTHER: C2RustUnnamed_10 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_10 = 220;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_10 = 156;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_10 = 157;
pub const BI_AMISSILES: C2RustUnnamed_10 = 155;
pub const BI_ARECHARGE: C2RustUnnamed_10 = 227;
pub const BI_ASPEED: C2RustUnnamed_10 = 218;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_11 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_11 = 1;
pub const BI_ALITE: C2RustUnnamed_10 = 228;
pub const BI_AMAGICMAP: C2RustUnnamed_10 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_10 = 225;
pub const BI_ESP: C2RustUnnamed_10 = 37;
pub const BI_ADETDOOR: C2RustUnnamed_10 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_10 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_10 = 222;
pub const BI_RPOIS: C2RustUnnamed_10 = 73;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_10 = 43;
pub const BI_RBLIND: C2RustUnnamed_10 = 77;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_10 = 42;
pub const BI_RCONF: C2RustUnnamed_10 = 78;
pub const BI_ACLW: C2RustUnnamed_10 = 233;
pub const BI_ACSW: C2RustUnnamed_10 = 232;
pub const BI_ACCW: C2RustUnnamed_10 = 231;
pub const BI_ASTFMAGI: C2RustUnnamed_10 = 219;
pub const BI_AMASSBAN: C2RustUnnamed_10 = 249;
pub const BI_SAURON_DEAD: C2RustUnnamed_10 = 263;
pub const BI_ALIFE: C2RustUnnamed_10 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_10 = 215;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_10 = 264;
pub const BI_AHEAL: C2RustUnnamed_10 = 214;
pub const BI_ATELEPORTLVL: C2RustUnnamed_10 = 247;
pub const BI_ATELEPORT: C2RustUnnamed_10 = 211;
pub const BI_AESCAPE: C2RustUnnamed_10 = 212;
pub const BI_APHASE: C2RustUnnamed_10 = 210;
pub const BI_RECALL: C2RustUnnamed_10 = 38;
pub const BI_AGLYPH: C2RustUnnamed_10 = 230;
pub const BI_APFE: C2RustUnnamed_10 = 229;
pub const BI_AID: C2RustUnnamed_10 = 217;
pub const BI_ALL_NEED_ID: C2RustUnnamed_10 = 254;
pub const BI_ARESPOIS: C2RustUnnamed_10 = 246;
pub const BI_IPOIS: C2RustUnnamed_10 = 68;
pub const BI_ARESCOLD: C2RustUnnamed_10 = 245;
pub const BI_ICOLD: C2RustUnnamed_10 = 66;
pub const BI_ARESHEAT: C2RustUnnamed_10 = 244;
pub const BI_IFIRE: C2RustUnnamed_10 = 64;
pub const BI_ISPOISONED: C2RustUnnamed_10 = 115;
pub const BI_ISCUT: C2RustUnnamed_10 = 116;
pub const BI_FOOD_LO: C2RustUnnamed_10 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_10 = 40;
pub const BI_FOOD: C2RustUnnamed_10 = 39;
pub const BI_REG: C2RustUnnamed_10 = 50;
pub const BI_ISWEAK: C2RustUnnamed_10 = 108;
pub const BI_ISHUNGRY: C2RustUnnamed_10 = 109;
pub const BI_AFUEL: C2RustUnnamed_10 = 213;
pub const BI_SP_ADJ: C2RustUnnamed_10 = 32;
pub const BI_STR_INDEX: C2RustUnnamed_10 = 15;
pub const BI_WORN_NEED_ID: C2RustUnnamed_10 = 253;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_10 = 258;
pub const BI_CRSUNKNO: C2RustUnnamed_10 = 192;
pub const BI_CRSAIRSWING: C2RustUnnamed_10 = 185;
pub const BI_CRSTWEP: C2RustUnnamed_10 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_10 = 172;
pub const BI_SPEED: C2RustUnnamed_10 = 44;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_10 = 184;
pub const BI_CRSSTONE: C2RustUnnamed_10 = 171;
pub const BI_CRSSUND: C2RustUnnamed_10 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_10 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_10 = 168;
pub const BI_CRSPARA: C2RustUnnamed_10 = 167;
pub const BI_CRSHALU: C2RustUnnamed_10 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_10 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_10 = 164;
pub const BI_CRSIRRITATION: C2RustUnnamed_10 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_10 = 161;
pub const BI_CRSTELE: C2RustUnnamed_10 = 163;
pub const BI_CRSAVULN: C2RustUnnamed_10 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_10 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_10 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_10 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_10 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_10 = 186;
pub const BI_CRSMPIMP: C2RustUnnamed_10 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_10 = 182;
pub const BI_CRSAGRV: C2RustUnnamed_10 = 174;
pub const BI_ARMOR: C2RustUnnamed_10 = 133;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_11 = 8;
pub const BI_RDARK: C2RustUnnamed_10 = 76;
pub const BI_RNTHR: C2RustUnnamed_10 = 82;
pub const BI_RDIS: C2RustUnnamed_10 = 84;
pub const BI_RKAOS: C2RustUnnamed_10 = 83;
pub const BI_RSND: C2RustUnnamed_10 = 79;
pub const BI_HLIFE: C2RustUnnamed_10 = 85;
pub const BI_RCOLD: C2RustUnnamed_10 = 70;
pub const BI_RACID: C2RustUnnamed_10 = 72;
pub const BI_RELEC: C2RustUnnamed_10 = 71;
pub const BI_RFIRE: C2RustUnnamed_10 = 69;
pub const BI_FRACT: C2RustUnnamed_10 = 86;
pub const BI_SINV: C2RustUnnamed_10 = 51;
pub const BI_SCON: C2RustUnnamed_10 = 24;
pub const BI_SDEX: C2RustUnnamed_10 = 23;
pub const BI_SWIS: C2RustUnnamed_10 = 22;
pub const BI_SINT: C2RustUnnamed_10 = 21;
pub const BI_SSTR: C2RustUnnamed_10 = 20;
pub const BI_RFEAR: C2RustUnnamed_10 = 74;
pub const BI_RNXUS: C2RustUnnamed_10 = 81;
pub const BI_RSHRD: C2RustUnnamed_10 = 80;
pub const BI_RLITE: C2RustUnnamed_10 = 75;
pub const BI_IACID: C2RustUnnamed_10 = 65;
pub const BI_IELEC: C2RustUnnamed_10 = 67;
pub const BI_DINV: C2RustUnnamed_10 = 259;
pub const BI_LIGHT: C2RustUnnamed_10 = 26;
pub const BI_FEATH: C2RustUnnamed_10 = 49;
pub const BI_SDIG: C2RustUnnamed_10 = 48;
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
pub const BI_ASTR: C2RustUnnamed_10 = 5;
pub const BI_HP_ADJ: C2RustUnnamed_10 = 29;
pub const BI_CON_INDEX: C2RustUnnamed_10 = 19;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_11 = 6;
pub const BI_DEX_INDEX: C2RustUnnamed_10 = 18;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_11 = 7;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_11 = 5;
pub const BI_DAM_RED: C2RustUnnamed_10 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_10 = 46;
pub const BI_AMMO_TVAL: C2RustUnnamed_10 = 152;
pub const BI_FAST_SHOTS: C2RustUnnamed_10 = 53;
pub const BI_TOHIT: C2RustUnnamed_10 = 134;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_11 = 4;
pub const BI_SHOTS: C2RustUnnamed_10 = 148;
pub const BI_AMMO_POWER: C2RustUnnamed_10 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_10 = 153;
pub const BI_BLOWS: C2RustUnnamed_10 = 146;
pub const BI_HEAVYWEPON: C2RustUnnamed_10 = 149;
pub const BI_W_IMPACT: C2RustUnnamed_10 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_10 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_10 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_10 = 201;
pub const BI_WS_EVIL: C2RustUnnamed_10 = 194;
pub const BI_WS_TROLL: C2RustUnnamed_10 = 198;
pub const BI_WS_ORC: C2RustUnnamed_10 = 197;
pub const BI_WB_COLD: C2RustUnnamed_10 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_10 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_10 = 206;
pub const BI_WB_ACID: C2RustUnnamed_10 = 205;
pub const BI_WS_GIANT: C2RustUnnamed_10 = 199;
pub const BI_WS_DRAGON: C2RustUnnamed_10 = 200;
pub const BI_WS_DEMON: C2RustUnnamed_10 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_10 = 195;
pub const BI_WS_ANIMAL: C2RustUnnamed_10 = 193;
pub const BI_TODAM: C2RustUnnamed_10 = 135;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_11 = 13;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_10 = 265;
pub const BI_ASHROOM: C2RustUnnamed_10 = 250;
pub const BI_AHWORD: C2RustUnnamed_10 = 248;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_10 = 243;
pub const BI_ABRAND: C2RustUnnamed_10 = 239;
pub const BI_WB_POIS: C2RustUnnamed_10 = 209;
pub const BI_CRSANNOY: C2RustUnnamed_10 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_10 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_10 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_10 = 178;
pub const BI_CRSSICK: C2RustUnnamed_10 = 177;
pub const BI_CRSDULL: C2RustUnnamed_10 = 176;
pub const BI_CRSVULN: C2RustUnnamed_10 = 175;
pub const BI_WHERE_CURSED: C2RustUnnamed_10 = 160;
pub const BI_AMMO_COUNT: C2RustUnnamed_10 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_10 = 150;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_10 = 147;
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
pub const BI_ISFIXALL: C2RustUnnamed_10 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_10 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_10 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_10 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_10 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_10 = 127;
pub const BI_ISFIXEXP: C2RustUnnamed_10 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_10 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_10 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_10 = 122;
pub const BI_ISFORGET: C2RustUnnamed_10 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_10 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_10 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_10 = 118;
pub const BI_ISSTUN: C2RustUnnamed_10 = 117;
pub const BI_ISCONFUSED: C2RustUnnamed_10 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_10 = 113;
pub const BI_ISBLIND: C2RustUnnamed_10 = 112;
pub const BI_ISGORGED: C2RustUnnamed_10 = 111;
pub const BI_ISFULL: C2RustUnnamed_10 = 110;
pub const BI_KING: C2RustUnnamed_10 = 107;
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
pub const BI_INFRA: C2RustUnnamed_10 = 52;
pub const BI_FAIL2: C2RustUnnamed_10 = 34;
pub const BI_FAIL1: C2RustUnnamed_10 = 33;
pub const BI_CURSP: C2RustUnnamed_10 = 30;
pub const BI_MAXHP: C2RustUnnamed_10 = 28;
pub const BI_CURHP: C2RustUnnamed_10 = 27;
pub const BI_WIS_INDEX: C2RustUnnamed_10 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_10 = 16;
pub const BI_ACON: C2RustUnnamed_10 = 9;
pub const BI_ADEX: C2RustUnnamed_10 = 8;
pub const BI_AWIS: C2RustUnnamed_10 = 7;
pub const BI_AINT: C2RustUnnamed_10 = 6;
pub const BI_CON: C2RustUnnamed_10 = 4;
pub const BI_DEX: C2RustUnnamed_10 = 3;
pub const BI_WIS: C2RustUnnamed_10 = 2;
pub const BI_INT: C2RustUnnamed_10 = 1;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_11 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_11 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_11 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_11 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_11 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_11 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_11 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_11 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_11 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_11 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_11 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_11 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_11 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_11 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_11 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_11 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_11 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_11 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_11 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_11 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_11 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_11 = 14;
pub const BORG_USES_SWAPS: C2RustUnnamed_11 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_11 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_11 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_11 = 9;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_11 = 3;
pub const BORG_VERBOSE: C2RustUnnamed_11 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn borg_power_equipment() -> int32_t {
    let mut hold: ::core::ffi::c_int = 0;
    let mut damage: ::core::ffi::c_int = 0;
    let mut dam: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cur_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_wgt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: int32_t = 0 as int32_t;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    hold = adj_str_hold[*borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize) as usize];
    item = borg_items.offset((*z_info).pack_size as isize) as *mut borg_item;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
        && (*item).iqty as ::core::ffi::c_int != 0
        && (*item).ident as ::core::ffi::c_int != true_0
    {
        value = (value as ::core::ffi::c_int + 1000000 as ::core::ffi::c_int) as int32_t;
    }
    damage = (((*item).dd as ::core::ffi::c_int * (*item).ds as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * 20 as ::core::ffi::c_long) as ::core::ffi::c_int;
    value = (value as ::core::ffi::c_int
        + damage
            * (*borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int)) as int32_t;
    value = (value as ::core::ffi::c_long
        + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
            + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + (*borg.trait_0.offset(BI_TODAM as ::core::ffi::c_int as isize)
            + (*item).to_d as ::core::ffi::c_int) as ::core::ffi::c_long
            * 30 as ::core::ffi::c_long) as int32_t;
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 15 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 75 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 15 as ::core::ffi::c_long) as int32_t;
        value = (value as ::core::ffi::c_long
            + ((*item).dd as ::core::ffi::c_int * (*item).ds as ::core::ffi::c_int)
                as ::core::ffi::c_long
                * 20 as ::core::ffi::c_long
                * 2 as ::core::ffi::c_long
                * *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long) as int32_t;
    }
    dam = damage
        * 2 as ::core::ffi::c_int
        * *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_WS_ANIMAL as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 7 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    dam = damage
        * 3 as ::core::ffi::c_int
        * *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_WS_UNDEAD as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_WK_UNDEAD as ::core::ffi::c_int as isize)
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_DEMON as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_WK_DEMON as ::core::ffi::c_int as isize)
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_DRAGON as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_WK_DRAGON as ::core::ffi::c_int as isize)
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 6 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_GIANT as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 4 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WB_ACID as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 4 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WB_ELEC as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WB_FIRE as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WB_COLD as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_ORC as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_TROLL as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_ORC as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WS_TROLL as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            == 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 1 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    dam = damage
        * 5 as ::core::ffi::c_int
        * *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize);
    if *borg
        .trait_0
        .offset(BI_WK_UNDEAD as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WK_DEMON as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_WK_DRAGON as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int
            + dam * 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int32_t;
    }
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_int + dam) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_W_IMPACT as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_HEAVYWEPON as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 500000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 10 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + *borg.trait_0.offset(BI_BLOWS as ::core::ffi::c_int as isize) as ::core::ffi::c_long
                * 45000 as ::core::ffi::c_long) as int32_t;
    }
    item = borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *mut borg_item;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
        && (*item).iqty as ::core::ffi::c_int != 0
        && (*item).ident as ::core::ffi::c_int != true_0
    {
        value = (value as ::core::ffi::c_int + 6000000 as ::core::ffi::c_int) as int32_t;
    }
    if (*item).to_d as ::core::ffi::c_int > 8 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 25 as ::core::ffi::c_int
    {
        damage = (*borg
            .trait_0
            .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize)
            + (*item).to_d as ::core::ffi::c_int)
            * *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize);
    } else {
        damage = (*borg
            .trait_0
            .offset(BI_AMMO_SIDES as ::core::ffi::c_int as isize)
            + 8 as ::core::ffi::c_int)
            * *borg
                .trait_0
                .offset(BI_AMMO_POWER as ::core::ffi::c_int as isize);
    }
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_SHOTS as ::core::ffi::c_int as isize) * damage)
                as ::core::ffi::c_long
                * 11 as ::core::ffi::c_long) as int32_t;
    } else {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_SHOTS as ::core::ffi::c_int as isize) * damage)
                as ::core::ffi::c_long
                * 9 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_SHOTS as ::core::ffi::c_int as isize) * damage)
                as ::core::ffi::c_long
                * 200 as ::core::ffi::c_long) as int32_t;
    }
    if (*item).sval as ::core::ffi::c_int == sv_sling
        && (*item).art_idx == 0
        && *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) < 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 5000 as ::core::ffi::c_long) as int32_t;
    }
    if (*item).sval as ::core::ffi::c_int == sv_sling
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) >= 9 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 8000 as ::core::ffi::c_int) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
            + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    if *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long
            + (*borg.trait_0.offset(BI_TOHIT as ::core::ffi::c_int as isize)
                + (*item).to_h as ::core::ffi::c_int) as ::core::ffi::c_long
                * 25 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_FAST_SHOTS as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_AMMO_TVAL as ::core::ffi::c_int as isize)
            == TV_ARROW as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
    }
    if hold < (*item).weight as ::core::ffi::c_int / 10 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long - 500000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER {
        value = (value as ::core::ffi::c_long
            - (*borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 10000 as ::core::ffi::c_long) as int32_t;
    } else if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
        <= 3 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) as ::core::ffi::c_long
                * 10000 as ::core::ffi::c_long) as int32_t;
    } else if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
        > 3 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + (30000 as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
                    * 1000 as ::core::ffi::c_int) as ::core::ffi::c_long))
            as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + *borg
            .trait_0
            .offset(BI_MOD_MOVES as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 3000 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg
            .trait_0
            .offset(BI_DAM_RED as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 10000 as ::core::ffi::c_long) as int32_t;
    if *borg_cfg.offset(BORG_WORSHIPS_SPEED as ::core::ffi::c_int as isize) != 0 {
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 150 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 185000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 145 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 149 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 180000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 140 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 144 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 135 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 139 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 130 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 134 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 160000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 125 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 129 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 135000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 120 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 124 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 110000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 115 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 119 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 85000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 110 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 114 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 65000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) < 110 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
    } else {
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 140 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 175000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 135 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 139 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 165000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 130 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 134 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 120 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 150000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 125 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 129 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 125000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 120 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 124 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 100000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 115 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 119 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 75000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
            >= 110 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                <= 114 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + ((*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long
                    + 55000 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) < 110 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                    - 110 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
    }
    value = (value as ::core::ffi::c_long
        + *borg
            .trait_0
            .offset(BI_STR_INDEX as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 100 as ::core::ffi::c_long) as int32_t;
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_long
            + *borg
                .trait_0
                .offset((BI_STR_INDEX as ::core::ffi::c_int + spell_stat) as isize)
                as ::core::ffi::c_long
                * 500 as ::core::ffi::c_long) as int32_t;
        if *borg_cfg.offset(BORG_WORSHIPS_MANA as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long
                + (*borg
                    .trait_0
                    .offset(BI_SP_ADJ as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 255 as ::core::ffi::c_long) as int32_t;
        } else {
            value = (value as ::core::ffi::c_long
                + (*borg
                    .trait_0
                    .offset(BI_SP_ADJ as ::core::ffi::c_int as isize)
                    / 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 155 as ::core::ffi::c_long) as int32_t;
        }
        value = (value as ::core::ffi::c_int
            + (100 as ::core::ffi::c_int
                - spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int) as int32_t;
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
            PF_ZERO_FAIL as ::core::ffi::c_int,
        ) {
            if (spell_chance(0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                < 1 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long + 30000 as ::core::ffi::c_long) as int32_t;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
        <= 37 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long
            + *borg
                .trait_0
                .offset(BI_DEX_INDEX as ::core::ffi::c_int as isize)
                as ::core::ffi::c_long
                * 120 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CON_INDEX as ::core::ffi::c_int as isize)
        <= 37 as ::core::ffi::c_int
    {
        if *borg_cfg.offset(BORG_WORSHIPS_HP as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long
                + *borg
                    .trait_0
                    .offset(BI_CON_INDEX as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 250 as ::core::ffi::c_long) as int32_t;
            if *borg
                .trait_0
                .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                < 800 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long
                    + *borg
                        .trait_0
                        .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_long
                        * 450 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((*borg
                        .trait_0
                        .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                        - 800 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long
                        + 350 as ::core::ffi::c_long * 500 as ::core::ffi::c_long))
                    as int32_t;
            }
        } else {
            value = (value as ::core::ffi::c_long
                + *borg
                    .trait_0
                    .offset(BI_CON_INDEX as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 150 as ::core::ffi::c_long) as int32_t;
            if *borg
                .trait_0
                .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                < 500 as ::core::ffi::c_int
            {
                value = (value as ::core::ffi::c_long
                    + *borg
                        .trait_0
                        .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_long
                        * 350 as ::core::ffi::c_long) as int32_t;
            } else {
                value = (value as ::core::ffi::c_long
                    + ((*borg
                        .trait_0
                        .offset(BI_HP_ADJ as ::core::ffi::c_int as isize)
                        - 500 as ::core::ffi::c_int) as ::core::ffi::c_long
                        * 100 as ::core::ffi::c_long
                        + 350 as ::core::ffi::c_long * 500 as ::core::ffi::c_long))
                    as int32_t;
            }
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_int
            + *borg
                .trait_0
                .offset((BI_ASTR as ::core::ffi::c_int + i) as isize)) as int32_t;
        i += 1;
    }
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_DISP as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_DISM as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 25 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 25 as ::core::ffi::c_long) as int32_t;
    if *borg.trait_0.offset(BI_SAV as ::core::ffi::c_int as isize) > 99 as ::core::ffi::c_int {
        value = (value as ::core::ffi::c_int + 10000 as ::core::ffi::c_int) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_STL as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_SRCH as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 1 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_THN as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 5 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_THB as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 35 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_THT as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + *borg.trait_0.offset(BI_DIG as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 2 as ::core::ffi::c_long) as int32_t;
    if *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 750 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 7500 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 7500 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        < 20 as ::core::ffi::c_int
    {
        if *borg.trait_0.offset(BI_FEATH as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        }
    } else if *borg.trait_0.offset(BI_FEATH as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as int32_t;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0 {
        if *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg.trait_0.offset(BI_DINV as ::core::ffi::c_int as isize) == 0 {
        if *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
        < 50 as ::core::ffi::c_int
    {
        if *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0 {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        }
    } else if *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_REG as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 65000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 40000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 3000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 3500 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 800 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 800 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 80000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        && *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 5500 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
    }
    if (*borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0
        || *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0)
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 10 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 20 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 25 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 40 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) >= 115 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 46 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 55000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 50 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 55 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 55 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 55000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 104000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 90000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) >= 120 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 60 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) >= 130 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 15000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 25000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) >= 140 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 1 as ::core::ffi::c_int
            >= 80 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
    {
        value = (value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg_cfg.offset(BORG_WORSHIPS_AC as ::core::ffi::c_int as isize) != 0 {
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) < 15 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 2500 as ::core::ffi::c_long) as int32_t;
        }
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                < 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 2000 as ::core::ffi::c_long
                    + 28250 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) >= 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 1500 as ::core::ffi::c_long
                    + 73750 as ::core::ffi::c_long)) as int32_t;
        }
    } else {
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) < 15 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 2000 as ::core::ffi::c_long) as int32_t;
        }
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                < 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 500 as ::core::ffi::c_long
                    + 28350 as ::core::ffi::c_long)) as int32_t;
        }
        if *borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize) >= 75 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long
                + (*borg.trait_0.offset(BI_ARMOR as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_long
                    * 100 as ::core::ffi::c_long
                    + 73750 as ::core::ffi::c_long)) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CRSAGRV as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSHPIMP as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int - 35000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_WARRIOR
        && *borg
            .trait_0
            .offset(BI_CRSMPIMP as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_int - 15000 as ::core::ffi::c_int) as int32_t;
    }
    if (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PRIEST
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_DRUID
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER)
        && *borg
            .trait_0
            .offset(BI_CRSMPIMP as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_int - 15000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 400000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSFEAR as ::core::ffi::c_int as isize)
        != 0
        && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_MAGE
    {
        value = (value as ::core::ffi::c_long - 200000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSDRAIN_XP as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 400000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSFVULN as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSEVULN as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSCVULN as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSAVULN as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int - 30000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSTELE as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSENVELOPING as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 50000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSIRRITATION as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 20000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSPOIS as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSIREN as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSHALU as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSPARA as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 800000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSDEM as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSDRA as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSUND as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSTONE as ::core::ffi::c_int as isize)
        != 0
        && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) < 140 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSSTEELSKIN as ::core::ffi::c_int as isize)
        != 0
        && *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize) < 140 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSNOTEL as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 700000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSTWEP as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 100000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSAIRSWING as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_CRSUNKNO as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long - 9999999 as ::core::ffi::c_long) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + 3000 as ::core::ffi::c_long
            * *borg
                .trait_0
                .offset(BI_MULTIPLE_BONUSES as ::core::ffi::c_int as isize)
                as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + 10000 as ::core::ffi::c_long
            * *borg
                .trait_0
                .offset(BI_WORN_NEED_ID as ::core::ffi::c_int as isize)
                as ::core::ffi::c_long) as int32_t;
    let mut activation_bonus: ::core::ffi::c_int = 0;
    let mut act: ::core::ffi::c_int = 0;
    act = 1 as ::core::ffi::c_int;
    while act < (*z_info).act_max as ::core::ffi::c_int {
        if !(*borg.activation.offset(act as isize) == 0) {
            activation_bonus = 0 as ::core::ffi::c_int;
            if act_illumination == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_mapping == act {
                activation_bonus += 550 as ::core::ffi::c_int;
            } else if act_clairvoyance == act {
                activation_bonus += 600 as ::core::ffi::c_int;
            } else if act_fire_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 9 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_cold_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_elec_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_acid_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_mana_bolt == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_stinking_cloud == act {
                activation_bonus += 500 as ::core::ffi::c_int + 24 as ::core::ffi::c_int;
            } else if act_cold_ball50 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 96 as ::core::ffi::c_int;
            } else if act_cold_ball100 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if act_fire_ball72 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 72 as ::core::ffi::c_int;
            } else if act_cold_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_fire_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 144 as ::core::ffi::c_int;
            } else if act_dispel_evil == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + (10 as ::core::ffi::c_int
                        + *borg
                            .trait_0
                            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                            * 5 as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int);
            } else if act_confuse2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_haste == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_haste1 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_haste2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_detect_objects == act {
                activation_bonus += 10 as ::core::ffi::c_int;
            } else if act_probing == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_stone_to_mud == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_tele_other == act {
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE {
                    activation_bonus += 500 as ::core::ffi::c_int;
                } else {
                    activation_bonus += 500 as ::core::ffi::c_int + 500 as ::core::ffi::c_int;
                }
            } else if act_drain_life1 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 90 as ::core::ffi::c_int;
            } else if act_drain_life2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if act_berserker == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_cure_light == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_cure_serious == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_cure_critical == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_cure_full2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_loskill == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if act_recall == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_arrow == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
            } else if act_rem_fear_pois == act {
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
                    || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PRIEST
                    || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_DRUID
                {
                    activation_bonus += 500 as ::core::ffi::c_int;
                } else {
                    activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
                }
            } else if act_tele_phase == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_detect_all == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_cure_full == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_heal1 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_heal2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_heal3 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_cure_nonorlybig == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_protevil == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_destroy_doors == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_banishment == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if act_resist_all == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_MAGE {
                    activation_bonus += 25000 as ::core::ffi::c_int;
                }
            } else if act_sleepii == act {
                activation_bonus += 500 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_DRUID
                    && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize)
                        != CLASS_NECROMANCER
                {
                    activation_bonus += 200 as ::core::ffi::c_int;
                }
            } else if act_recharge == act {
                activation_bonus += 500 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_MAGE
                    && *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_ROGUE
                {
                    activation_bonus += 100 as ::core::ffi::c_int;
                }
            } else if act_tele_long == act {
                activation_bonus += 300 as ::core::ffi::c_int;
            } else if act_missile == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_cure_temp == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_starlight2 == act {
                activation_bonus += 100 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_bizarre == act {
                activation_bonus += 999999 as ::core::ffi::c_int;
            } else if act_star_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 300 as ::core::ffi::c_int;
            } else if act_rage_bless_resist == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
                if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_MAGE {
                    activation_bonus += 25000 as ::core::ffi::c_int;
                }
            } else if act_polymorph == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_starlight == act {
                activation_bonus += 100 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_light == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_firebrand == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_restore_life == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_restore_exp == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_restore_st_lev == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enlightenment == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_hero == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_shero == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_cure_paranoia == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_cure_mind == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if act_cure_body == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if act_mon_slow == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_mon_confuse == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_sleep_all == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_mon_scare == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_light_line == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_disable_traps == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_drain_life3 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 150 as ::core::ffi::c_int;
            } else if act_drain_life4 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 250 as ::core::ffi::c_int;
            } else if act_elec_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 64 as ::core::ffi::c_int;
            } else if act_elec_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 250 as ::core::ffi::c_int;
            } else if act_acid_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_acid_bolt3 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_acid_ball == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if act_cold_ball160 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 160 as ::core::ffi::c_int;
            } else if act_cold_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if act_fire_ball2 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 120 as ::core::ffi::c_int;
            } else if act_fire_ball200 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 200 as ::core::ffi::c_int;
            } else if act_fire_bolt2 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_fire_bolt3 == act {
                activation_bonus += 500 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            } else if act_dispel_evil60 == act {
                activation_bonus += 500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int;
            } else if act_dispel_undead == act {
                activation_bonus += 500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int;
            } else if act_dispel_all == act {
                activation_bonus +=
                    500 as ::core::ffi::c_int + 60 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            } else if act_deep_descent == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_earthquakes == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_destruction2 == act {
                activation_bonus += 500 as ::core::ffi::c_int;
            } else if act_losslow == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_lossleep == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_losconf == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_satisfy == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_blessing == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_blessing2 == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_blessing3 == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_glyph == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_tele_level == act {
                activation_bonus = (activation_bonus as ::core::ffi::c_long
                    + 5000 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
            } else if act_confusing == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enchant_tohit == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enchant_todam == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enchant_weapon == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enchant_armor == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_enchant_armor2 == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_remove_curse == act {
                activation_bonus += 9000 as ::core::ffi::c_int;
            } else if act_remove_curse2 == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if act_detect_treasure == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_detect_invis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_detect_evil == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_mana == act {
                activation_bonus += 5000 as ::core::ffi::c_int;
            } else if act_brawn == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_intellect == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_contemplation == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_toughness == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_nimbleness == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_restore_str == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_int == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_wis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_dex == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_con == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_restore_all == act {
                activation_bonus += 150 as ::core::ffi::c_int;
            } else if act_tmd_free_act == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_tmd_infra == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_tmd_sinvis == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_tmd_esp == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_resist_acid == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_resist_elec == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_resist_fire == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_resist_cold == act {
                activation_bonus += 100 as ::core::ffi::c_int;
            } else if act_resist_pois == act {
                activation_bonus += 150 as ::core::ffi::c_int;
            } else if act_cure_confusion == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_wonder == act {
                activation_bonus += 300 as ::core::ffi::c_int;
            } else if act_wand_breath == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_staff_magi == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_staff_holy == act {
                activation_bonus += 1000 as ::core::ffi::c_int;
            } else if act_drink_breath == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_food_waybread == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_shroom_emergency == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_shroom_terror == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_shroom_stone == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_shroom_debility == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_shroom_sprinting == act {
                activation_bonus += 0 as ::core::ffi::c_int;
            } else if act_shroom_purging == act {
                activation_bonus += 50 as ::core::ffi::c_int;
            } else if act_ring_acid == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if act_ring_flames == act {
                activation_bonus += 25000 as ::core::ffi::c_int;
            } else if act_ring_ice == act {
                activation_bonus += 15000 as ::core::ffi::c_int;
            } else if act_ring_lightning == act {
                activation_bonus += 10000 as ::core::ffi::c_int;
            } else if act_dragon_blue == act {
                activation_bonus += 1100 as ::core::ffi::c_int;
            } else if act_dragon_green == act {
                activation_bonus += 2750 as ::core::ffi::c_int;
            } else if act_dragon_red == act {
                activation_bonus += 1100 as ::core::ffi::c_int;
            } else if act_dragon_multihued == act {
                activation_bonus += 3250 as ::core::ffi::c_int;
            } else if act_dragon_gold == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if act_dragon_chaos == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if act_dragon_law == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if act_dragon_balance == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if act_dragon_shining == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            } else if act_dragon_power == act {
                activation_bonus += 5150 as ::core::ffi::c_int;
            }
            value = (value as ::core::ffi::c_int
                + activation_bonus * *borg.activation.offset(act as isize))
                as int32_t;
        }
        act += 1;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize))
    .iqty
        != 0
    {
        item = borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item;
        if borg_ego_has_random_power(e_info.offset((*item).ego_idx as isize) as *mut ego_item)
            as ::core::ffi::c_int
            != 0
            && !(*item).ident
        {
            value = (value as ::core::ffi::c_long + 999999 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_STR_INDEX as ::core::ffi::c_int as isize)
        < 15 as ::core::ffi::c_int
    {
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 200 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int
                - (borg_item_weight(borg_items.offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) as *mut borg_item) as ::core::ffi::c_int
                    - 200 as ::core::ffi::c_int)
                    * 15 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 30 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
        if borg_item_weight(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item,
        ) as ::core::ffi::c_int
            > 50 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int - 250 as ::core::ffi::c_int) as int32_t;
        }
    }
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    cur_wgt += borg_item_weight(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize)
            as *mut borg_item,
    ) as ::core::ffi::c_int;
    max_wgt = (*(*player).class).magic.spell_weight;
    if (*(*player).class).magic.total_spells != 0
        && (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut max_sp: ::core::ffi::c_int = *borg
            .trait_0
            .offset(BI_SP_ADJ as ::core::ffi::c_int as isize)
            / 100 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        max_sp -= (cur_wgt - max_wgt) / 10 as ::core::ffi::c_int;
        if max_sp >= 300 as ::core::ffi::c_int && max_sp <= 350 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 400 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 200 as ::core::ffi::c_int && max_sp <= 299 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 800 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp >= 100 as ::core::ffi::c_int && max_sp <= 199 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1600 as ::core::ffi::c_long) as int32_t;
        }
        if max_sp <= 99 as ::core::ffi::c_int {
            value = (value as ::core::ffi::c_long
                - ((cur_wgt - max_wgt) / 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 3200 as ::core::ffi::c_long) as int32_t;
        }
    }
    return value;
}
unsafe extern "C" fn borg_power_inventory() -> int32_t {
    let mut k: ::core::ffi::c_int = 0;
    let mut book: ::core::ffi::c_int = 0;
    let mut value: int32_t = 0 as int32_t;
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < *borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 60000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int {
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_AFUEL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long
                + (6000 as ::core::ffi::c_long
                    - (k * 100 as ::core::ffi::c_int) as ::core::ffi::c_long))
                as int32_t;
            k += 1;
        }
    }
    if (*borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0)
        && *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_int + 100000 as ::core::ffi::c_int) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 7 as ::core::ffi::c_int
        && k < *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int {
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg.trait_0.offset(BI_REG as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            <= 15 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 15 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 700 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 7 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_FOOD_HI as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 52 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 15 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_FOOD_LO as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if (*borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0)
        && *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_int + 100000 as ::core::ffi::c_int) as int32_t;
    }
    if (*borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0)
        && *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize) != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if (*borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0)
        && *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize) != 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 25000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_ACUREPOIS as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_int + 15000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg.trait_0.offset(BI_IPOIS as ::core::ffi::c_int as isize) == 0
        && *borg
            .trait_0
            .offset(BI_ACUREPOIS as ::core::ffi::c_int as isize)
            <= 20 as ::core::ffi::c_int
    {
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
        {
            while k < 4 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_ARESPOIS as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 300 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            > 20 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
        {
            if *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_ARESHEAT as ::core::ffi::c_int as isize)
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            k = 0 as ::core::ffi::c_int;
            if *borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_ARESCOLD as ::core::ffi::c_int as isize)
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            if *borg.trait_0.offset(BI_IPOIS as ::core::ffi::c_int as isize) == 0 {
                while k < 4 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_ARESPOIS as ::core::ffi::c_int as isize)
                {
                    value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 10 as ::core::ffi::c_int
    {
        while k < 5 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_AID as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < *borg.trait_0.offset(BI_AID as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 600 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_ALL_NEED_ID as ::core::ffi::c_int as isize)
        != 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < *borg
            .trait_0
            .offset(BI_ALL_NEED_ID as ::core::ffi::c_int as isize)
            && k < *borg.trait_0.offset(BI_AID as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 6000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
    {
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_APFE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 25 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_APFE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 10 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    while k < 25 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 75 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AMASSBAN as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        > 7 as ::core::ffi::c_int
    {
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
        {
            while k < 3 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_RECALL as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
            if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize)
                >= 15 as ::core::ffi::c_int
            {
                while k < 7 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_RECALL as ::core::ffi::c_int as isize)
                {
                    value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
            if *borg
                .trait_0
                .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                >= 50 as ::core::ffi::c_int
                && *borg.has.offset(kv_rod_recall as isize) != 0
            {
                value = (value as ::core::ffi::c_int + 12000 as ::core::ffi::c_int) as int32_t;
            }
        }
    }
    k = 1 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_APHASE as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
    {
        while k < 8 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_APHASE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) >= 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_APHASE as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
    {
        while k < 2 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            > 70 as ::core::ffi::c_int
        {
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_AESCAPE as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 3 as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
            != 0
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 7 as ::core::ffi::c_int
    {
        while k < 3 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 30 as ::core::ffi::c_int
    {
        while k < 10 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ATELEPORT as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 15 as ::core::ffi::c_int
    {
        while k < 5 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ATELEPORTLVL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_ROGUE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_BLACKGUARD
    {
        k = 0 as ::core::ffi::c_int;
        while k < 15 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 46 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize)
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 6 as ::core::ffi::c_int && k < *borg.has.offset(kv_rod_healing as isize) {
            value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_RANGER
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PALADIN
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_AHEAL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 46 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize)
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PALADIN {
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int && k < *borg.has.offset(kv_potion_healing as isize) {
                value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 4 as ::core::ffi::c_int && k < *borg.has.offset(kv_rod_healing as isize) {
            value = (value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_PRIEST
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_DRUID
    {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        {
            k = 0 as ::core::ffi::c_int;
            while k < 10 as ::core::ffi::c_int && k < *borg.has.offset(kv_potion_healing as isize) {
                value = (value as ::core::ffi::c_long - 2000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int && k < *borg.has.offset(kv_potion_healing as isize) {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 46 as ::core::ffi::c_int
        {
            if *borg
                .trait_0
                .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize)
                != 0
            {
                while k < 1 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            } else {
                while k < 2 as ::core::ffi::c_int
                    && k < *borg
                        .trait_0
                        .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
                {
                    value =
                        (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
                    k += 1;
                }
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 99 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize)
            == 0
    {
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int && k < *borg.has.offset(kv_potion_healing as isize) {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AEZHEAL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ASPEED as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 8000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 99 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ALIFE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_WARRIOR {
            while k < 99 as ::core::ffi::c_int
                && k < *borg.has.offset(kv_potion_restore_mana as isize)
            {
                value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 0 as ::core::ffi::c_int;
        while k < 40 as ::core::ffi::c_int && k < *borg.has.offset(kv_mush_stoneskin as isize) {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg
            .trait_0
            .offset(BI_SAURON_DEAD as ::core::ffi::c_int as isize)
            != 0
        {
            k = 0 as ::core::ffi::c_int;
            while k < 99 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_AMASSBAN as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) > 100 as ::core::ffi::c_int {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int && k < *borg.has.offset(kv_potion_restore_mana as isize)
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        k = 0 as ::core::ffi::c_int;
        while k < 100 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ASTFMAGI as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 35 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            > 10 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        >= 35 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) > 15 as ::core::ffi::c_int {
            while k < 15 as ::core::ffi::c_int
                && k < *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize) < 5 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            > 10 as ::core::ffi::c_int
        && (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 35 as ::core::ffi::c_int
            || *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) == 0)
    {
        k = 0 as ::core::ffi::c_int;
        while k < 7 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 50 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) > 15 as ::core::ffi::c_int {
            while k < 10 as ::core::ffi::c_int
                && k < *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 5 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg.trait_0.offset(BI_ACCW as ::core::ffi::c_int as isize)
        + *borg.trait_0.offset(BI_ACSW as ::core::ffi::c_int as isize)
        < 5 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 8 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ACLW as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 550 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) == 0 {
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < 10 as ::core::ffi::c_int)
        {
            k = 0 as ::core::ffi::c_int;
            while k < 10 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_FOOD_CURE_CONF as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 400 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_RBLIND as ::core::ffi::c_int as isize)
        == 0
    {
        k = 0 as ::core::ffi::c_int;
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
        {
            while k < 5 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_FOOD_CURE_BLIND as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 300 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) == 0 {
        k = 0 as ::core::ffi::c_int;
        if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
            && *borg
                .trait_0
                .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
        {
            while k < 5 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_ACUREPOIS as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 250 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_ADETTRAP as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_ADETDOOR as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) == 0 {
        k = 0 as ::core::ffi::c_int;
        while k < 1 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ADETEVIL as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    while k < 1 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_AMAGICMAP as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) != CLASS_NECROMANCER {
        k = 0 as ::core::ffi::c_int;
        while k < 1 as ::core::ffi::c_int
            && k < *borg.trait_0.offset(BI_ALITE as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 1000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        >= 100 as ::core::ffi::c_int
    {
        k = 0 as ::core::ffi::c_int;
        while k < 10 as ::core::ffi::c_int
            && k < *borg.has.offset(kv_scroll_mass_banishment as isize)
        {
            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        while k < 25 as ::core::ffi::c_int
            && k < *borg.has.offset(kv_scroll_mass_banishment as isize)
        {
            value = (value as ::core::ffi::c_long + 2000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    k = 0 as ::core::ffi::c_int;
    if !(*borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize))
    {
        while k < 20 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_ASPEED as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ARECHARGE as ::core::ffi::c_int as isize)
        != 0
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            < 99 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_RANGER
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
    {
        k = 0 as ::core::ffi::c_int;
        while k < 40 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) > 15 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize)
                <= 18 as ::core::ffi::c_int
        {
            while k < 80 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) > 18 as ::core::ffi::c_int {
            while k < 180 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 8 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        k = 4 as ::core::ffi::c_int;
        while k < *borg
            .trait_0
            .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    } else {
        k = 0 as ::core::ffi::c_int;
        while k < 20 as ::core::ffi::c_int
            && k < *borg
                .trait_0
                .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long + 100 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) > 15 as ::core::ffi::c_int {
            while k < 50 as ::core::ffi::c_int
                && k < *borg
                    .trait_0
                    .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
            {
                value = (value as ::core::ffi::c_long + 10 as ::core::ffi::c_long) as int32_t;
                k += 1;
            }
        }
        if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize) <= 15 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_AMISSILES as ::core::ffi::c_int as isize)
                > 20 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_long - 1000 as ::core::ffi::c_long) as int32_t;
        }
        k = 2 as ::core::ffi::c_int;
        while k < *borg
            .trait_0
            .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
        {
            value = (value as ::core::ffi::c_long - 10000 as ::core::ffi::c_long) as int32_t;
            k += 1;
        }
    }
    value = (value as ::core::ffi::c_long
        - 1000 as ::core::ffi::c_long
            * *borg
                .trait_0
                .offset(BI_AMISSILES_CURSED as ::core::ffi::c_int as isize)
                as ::core::ffi::c_long) as int32_t;
    value = (value as ::core::ffi::c_long
        + 100 as ::core::ffi::c_long
            * *borg
                .trait_0
                .offset(BI_AMISSILES_SPECIAL as ::core::ffi::c_int as isize)
                as ::core::ffi::c_long) as int32_t;
    if *borg
        .trait_0
        .offset(BI_ASTFDEST as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 9 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_ASTFDEST as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 200 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg
        .trait_0
        .offset(BI_ATPORTOTHER as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
        && *borg
            .trait_0
            .offset(BI_ATPORTOTHER as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 50000 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 15 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_ATPORTOTHER as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if (*borg.has.offset(kv_wand_magic_missile as isize) != 0
        || *borg.has.offset(kv_wand_stinking_cloud as isize) != 0)
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            < 30 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg.has.offset(kv_wand_annihilation as isize) != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 30 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if (*borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_WARRIOR
        || *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            <= 20 as ::core::ffi::c_int)
        && (*borg.has.offset(kv_wand_magic_missile as isize) != 0
            || *borg.has.offset(kv_wand_annihilation as isize) != 0
            || *borg.has.offset(kv_wand_stinking_cloud as isize) != 0)
    {
        value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long) as int32_t;
    }
    value = (value as ::core::ffi::c_long
        + *borg
            .trait_0
            .offset(BI_GOOD_W_CHG as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            * 50 as ::core::ffi::c_long) as int32_t;
    if *borg
        .trait_0
        .offset(BI_GOOD_S_CHG as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 2500 as ::core::ffi::c_long) as int32_t;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 3 as ::core::ffi::c_int
        && k < *borg
            .trait_0
            .offset(BI_GOOD_S_CHG as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < *borg.trait_0.offset(BI_AROD1 as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_int + 8000 as ::core::ffi::c_int) as int32_t;
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < *borg.trait_0.offset(BI_AROD2 as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_int + 12000 as ::core::ffi::c_int) as int32_t;
        k += 1;
    }
    if !borg.need_statgain[STAT_STR as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !borg.need_statgain[STAT_INT as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 20000 as ::core::ffi::c_int) as int32_t;
    }
    if !borg.need_statgain[STAT_WIS as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 20000 as ::core::ffi::c_int) as int32_t;
    }
    if !borg.need_statgain[STAT_DEX as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if !borg.need_statgain[STAT_CON as ::core::ffi::c_int as usize] {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    let mut spell_stat: ::core::ffi::c_int = borg_spell_stat();
    if spell_stat >= 0 as ::core::ffi::c_int {
        if !borg.need_statgain[spell_stat as usize] {
            value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if borg.amt_statgain[STAT_STR as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && *borg.trait_0.offset(BI_CSTR as ::core::ffi::c_int as isize)
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if borg.amt_statgain[STAT_INT as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && *borg.trait_0.offset(BI_CINT as ::core::ffi::c_int as isize)
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 520000 as ::core::ffi::c_int) as int32_t;
    }
    if spell_stat >= 0 as ::core::ffi::c_int {
        if borg.amt_statgain[spell_stat as usize] as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset((BI_CSTR as ::core::ffi::c_int + spell_stat) as isize)
                < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
        {
            value = (value as ::core::ffi::c_int + 575000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if borg.amt_statgain[STAT_WIS as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && *borg.trait_0.offset(BI_CWIS as ::core::ffi::c_int as isize)
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 520000 as ::core::ffi::c_int) as int32_t;
    }
    if borg.amt_statgain[STAT_DEX as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && *borg.trait_0.offset(BI_CDEX as ::core::ffi::c_int as isize)
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if borg.amt_statgain[STAT_CON as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && *borg.trait_0.offset(BI_CCON as ::core::ffi::c_int as isize)
            < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_int + 550000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize)
        != 0
    {
        if *borg.has.offset(kv_scroll_star_remove_curse as isize) != 0 {
            value = (value as ::core::ffi::c_int + 90000 as ::core::ffi::c_int) as int32_t;
        }
        if *borg.has.offset(kv_scroll_remove_curse as isize) != 0 {
            value = (value as ::core::ffi::c_int + 90000 as ::core::ffi::c_int) as int32_t;
        }
    }
    if *borg
        .trait_0
        .offset(BI_HASFIXEXP as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_int + 50000 as ::core::ffi::c_int) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize)
        < 1000 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 540 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize)
        < 1000 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 540 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize)
        < 1000 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize)
            != 0
    {
        value = (value as ::core::ffi::c_long + 500 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_AENCH_SWEP as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_AENCH_SARM as ::core::ffi::c_int as isize)
        != 0
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    k = 1 as ::core::ffi::c_int;
    while k < 6 as ::core::ffi::c_int
        && k < *borg.trait_0.offset(BI_EMPTY as ::core::ffi::c_int as isize)
    {
        value = (value as ::core::ffi::c_long + 40 as ::core::ffi::c_long) as int32_t;
        k += 1;
    }
    if *borg.trait_0.offset(BI_EMPTY as ::core::ffi::c_int as isize) != 0 {
        value = (value as ::core::ffi::c_long + 4000 as ::core::ffi::c_long) as int32_t;
    }
    if *borg
        .trait_0
        .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
        <= 40 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            >= 25 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
            < 100000 as ::core::ffi::c_int
        && (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
            != TV_DIGGING as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_ADIGGER as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
    {
        value = (value as ::core::ffi::c_long + 5000 as ::core::ffi::c_long) as int32_t;
    }
    book = 0 as ::core::ffi::c_int;
    while book < 9 as ::core::ffi::c_int {
        if !(borg.amt_book[book as usize] == 0) {
            if (*(*(*player).class).magic.books.offset(book as isize)).dungeon {
                let mut what: ::core::ffi::c_int = 0;
                what = 0 as ::core::ffi::c_int;
                while what < 9 as ::core::ffi::c_int {
                    let mut as_0: *mut borg_magic = borg_get_spell_entry(book, what);
                    if as_0.is_null() {
                        break;
                    }
                    if !((*as_0).level as ::core::ffi::c_int
                        > *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize))
                    {
                        if !((*as_0).power as ::core::ffi::c_int
                            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize))
                        {
                            value = (value as ::core::ffi::c_long + 15000 as ::core::ffi::c_long)
                                as int32_t;
                        }
                    }
                    what += 1;
                }
            } else {
                let mut what_0: ::core::ffi::c_int = 0;
                let mut when: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
                what_0 = 0 as ::core::ffi::c_int;
                while what_0 < 9 as ::core::ffi::c_int {
                    let mut as_1: *mut borg_magic = borg_get_spell_entry(book, what_0);
                    if as_1.is_null() {
                        break;
                    }
                    if ((*as_1).level as ::core::ffi::c_int) < when {
                        when = (*as_1).level as ::core::ffi::c_int;
                    }
                    what_0 += 1;
                }
                if !(when > 5 as ::core::ffi::c_int
                    && when
                        >= *borg
                            .trait_0
                            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
                            + 2 as ::core::ffi::c_int)
                {
                    k = 0 as ::core::ffi::c_int;
                    while k < 1 as ::core::ffi::c_int
                        && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                    {
                        value = (value as ::core::ffi::c_long + 500000 as ::core::ffi::c_long)
                            as int32_t;
                        k += 1;
                    }
                    if *borg.trait_0.offset(BI_STR as ::core::ffi::c_int as isize)
                        > 5 as ::core::ffi::c_int
                    {
                        while k < 2 as ::core::ffi::c_int
                            && k < borg.amt_book[book as usize] as ::core::ffi::c_int
                        {
                            value = (value as ::core::ffi::c_long + 10000 as ::core::ffi::c_long)
                                as int32_t;
                            k += 1;
                        }
                    }
                }
            }
        }
        book += 1;
    }
    if *borg
        .trait_0
        .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize) / 2 as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
        let mut i: ::core::ffi::c_int = (*z_info).pack_size as ::core::ffi::c_int
            - *borg
                .trait_0
                .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize);
        while i >= 0 as ::core::ffi::c_int {
            if (*borg_items.offset(i as isize)).iqty != 0 {
                item = borg_items.offset(i as isize) as *mut borg_item;
                break;
            } else {
                i -= 1;
            }
        }
        if !(!item.is_null()
            && (*item).iqty as ::core::ffi::c_int != 0
            && (*item).aware as ::core::ffi::c_int != 0
            && ((*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                && ((*item).sval as ::core::ffi::c_int == sv_scroll_enchant_armor
                    && *borg
                        .trait_0
                        .offset(BI_AENCH_ARM as ::core::ffi::c_int as isize)
                        < 1000 as ::core::ffi::c_int
                    && *borg
                        .trait_0
                        .offset(BI_NEED_ENCHANT_TO_A as ::core::ffi::c_int as isize)
                        != 0
                    || (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_hit
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOH as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_H as ::core::ffi::c_int as isize)
                            != 0
                    || (*item).sval as ::core::ffi::c_int == sv_scroll_enchant_weapon_to_dam
                        && *borg
                            .trait_0
                            .offset(BI_AENCH_TOD as ::core::ffi::c_int as isize)
                            < 1000 as ::core::ffi::c_int
                        && *borg
                            .trait_0
                            .offset(BI_NEED_ENCHANT_TO_D as ::core::ffi::c_int as isize)
                            != 0
                    || (*item).sval as ::core::ffi::c_int == sv_scroll_star_enchant_weapon
                    || (*item).sval as ::core::ffi::c_int == sv_scroll_star_enchant_armor)
                || (*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
                    && ((*item).sval as ::core::ffi::c_int == sv_potion_inc_str
                        || (*item).sval as ::core::ffi::c_int == sv_potion_inc_int
                        || (*item).sval as ::core::ffi::c_int == sv_potion_inc_wis
                        || (*item).sval as ::core::ffi::c_int == sv_potion_inc_dex
                        || (*item).sval as ::core::ffi::c_int == sv_potion_inc_con
                        || (*item).sval as ::core::ffi::c_int == sv_potion_inc_all)))
        {
            value = (value as ::core::ffi::c_long
                - ((*borg
                    .trait_0
                    .offset(BI_WEIGHT as ::core::ffi::c_int as isize)
                    - *borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 2 as ::core::ffi::c_int)
                    / (*borg.trait_0.offset(BI_CARRY as ::core::ffi::c_int as isize)
                        / 10 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long) as int32_t;
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn borg_power() -> int32_t {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut value: int32_t = 0 as int32_t;
    if *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) != 0 {
        value += borg_power_dynamic();
    } else {
        value += borg_power_equipment();
        value += borg_power_inventory();
    }
    i = 1 as ::core::ffi::c_int;
    while i
        <= *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            + 50 as ::core::ffi::c_int
    {
        if !(borg_prepared(i) as *mut ::core::ffi::c_char).is_null() {
            break;
        }
        i += 1;
    }
    value = (value as ::core::ffi::c_long
        + (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_long * 40000 as ::core::ffi::c_long)
        as int32_t;
    value += weapon_swap_value;
    value += armour_swap_value;
    return value;
}
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLASS_RANGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_BLACKGUARD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
