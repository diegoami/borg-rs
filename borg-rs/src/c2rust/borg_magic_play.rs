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
    static mut player: *mut player;
    fn effect_handler_RANDOM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DAMAGE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_HEAL_HP(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MON_HEAL_HP(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MON_HEAL_KIN(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_NOURISH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CRUNCH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CURE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TIMED_SET(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TIMED_INC(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TIMED_INC_NO_RES(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MON_TIMED_INC(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TIMED_DEC(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_GLYPH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_WEB(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RESTORE_STAT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DRAIN_STAT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_LOSE_RANDOM_STAT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_GAIN_STAT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RESTORE_EXP(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_GAIN_EXP(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DRAIN_LIGHT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DRAIN_MANA(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RESTORE_MANA(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_REMOVE_CURSE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RECALL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DEEP_DESCENT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_ALTER_REALITY(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MAP_AREA(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_READ_MINDS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_TRAPS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_DOORS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_STAIRS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_ORE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SENSE_GOLD(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_GOLD(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SENSE_OBJECTS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_OBJECTS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_LIVING_MONSTERS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_VISIBLE_MONSTERS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_INVISIBLE_MONSTERS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_FEARFUL_MONSTERS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_IDENTIFY(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_EVIL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DETECT_SOUL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CREATE_STAIRS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DISENCHANT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_ENCHANT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RECHARGE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_PROJECT_LOS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_PROJECT_LOS_AWARE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_ACQUIRE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_WAKE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SUMMON(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BANISH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MASS_BANISH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_PROBE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TELEPORT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TELEPORT_TO(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TELEPORT_LEVEL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_RUBBLE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_GRANITE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DESTRUCTION(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_EARTHQUAKE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_LIGHT_LEVEL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DARKEN_LEVEL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_LIGHT_AREA(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_DARKEN_AREA(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SPOT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SPHERE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BALL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BREATH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_ARC(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SHORT_BEAM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_LASH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SWARM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_STRIKE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_STAR(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_STAR_BALL(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BOLT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BEAM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BOLT_OR_BEAM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_LINE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_ALTER(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BOLT_STATUS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BOLT_STATUS_DAM(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BOLT_AWARE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TOUCH(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TOUCH_AWARE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CURSE_ARMOR(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CURSE_WEAPON(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BRAND_WEAPON(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BRAND_AMMO(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BRAND_BOLTS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CREATE_ARROWS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TAP_DEVICE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_TAP_UNLIFE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SHAPECHANGE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CURSE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_COMMAND(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_JUMP_AND_BITE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MOVE_ATTACK(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SINGLE_COMBAT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_MELEE_BLOWS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SWEEP(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_BIZARRE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_WONDER(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SELECT(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SET_VALUE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_CLEAR_VALUE(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_SCRAMBLE_STATS(context: *mut effect_handler_context_t) -> bool;
    fn effect_handler_UNSCRAMBLE_STATS(context: *mut effect_handler_context_t) -> bool;
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    static mut borg: borg_struct;
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    static mut borg_magics: *mut borg_magic;
    fn borg_spell(spell: borg_spells) -> bool;
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
pub struct chest_trap {
    pub next: *mut chest_trap,
    pub name: *mut ::core::ffi::c_char,
    pub code: *mut ::core::ffi::c_char,
    pub level: ::core::ffi::c_int,
    pub effect: *mut effect,
    pub pval: ::core::ffi::c_int,
    pub destroy: bool,
    pub magic: bool,
    pub msg: *mut ::core::ffi::c_char,
    pub msg_death: *mut ::core::ffi::c_char,
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
pub type keycode_t = uint32_t;
pub const EF_BANISH: effect_index = 55;
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
pub const EF_TELEPORT_TO: effect_index = 59;
pub const EF_RANDOM: effect_index = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct effect_kind {
    pub index: uint16_t,
    pub aim: bool,
    pub info: *const ::core::ffi::c_char,
    pub handler: effect_handler_f,
    pub desc: *const ::core::ffi::c_char,
    pub menu_name: *const ::core::ffi::c_char,
}
pub type effect_handler_f = Option<unsafe extern "C" fn(*mut effect_handler_context_t) -> bool>;
pub type effect_handler_context_t = effect_handler_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct effect_handler_context_s {
    pub effect: effect_index,
    pub origin: source,
    pub obj: *const object,
    pub aware: bool,
    pub dir: ::core::ffi::c_int,
    pub beam: ::core::ffi::c_int,
    pub boost: ::core::ffi::c_int,
    pub value: random_value,
    pub subtype: ::core::ffi::c_int,
    pub radius: ::core::ffi::c_int,
    pub other: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub msg: *const ::core::ffi::c_char,
    pub ident: bool,
    pub cmd: *mut command,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub context: cmd_context,
    pub code: cmd_code,
    pub nrepeats: ::core::ffi::c_int,
    pub background_command: ::core::ffi::c_int,
    pub arg: [cmd_arg; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_arg {
    pub type_0: cmd_arg_type,
    pub data: cmd_arg_data,
    pub name: [::core::ffi::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cmd_arg_data {
    pub string: *const ::core::ffi::c_char,
    pub choice: ::core::ffi::c_int,
    pub obj: *mut object,
    pub number: ::core::ffi::c_int,
    pub direction: ::core::ffi::c_int,
    pub point: loc,
}
pub type cmd_arg_type = ::core::ffi::c_uint;
pub const arg_POINT: cmd_arg_type = 7;
pub const arg_TARGET: cmd_arg_type = 6;
pub const arg_DIRECTION: cmd_arg_type = 5;
pub const arg_NUMBER: cmd_arg_type = 4;
pub const arg_ITEM: cmd_arg_type = 3;
pub const arg_CHOICE: cmd_arg_type = 2;
pub const arg_STRING: cmd_arg_type = 1;
pub const arg_NONE: cmd_arg_type = 0;
pub type cmd_code = ::core::ffi::c_uint;
pub const CMD_COMMAND_MONSTER: cmd_code = 114;
pub const CMD_REPEAT: cmd_code = 113;
pub const CMD_HELP: cmd_code = 112;
pub const CMD_RETIRE: cmd_code = 111;
pub const CMD_WIZ_WIZARD_LIGHT: cmd_code = 110;
pub const CMD_WIZ_WIPE_RECALL: cmd_code = 109;
pub const CMD_WIZ_TWEAK_ITEM: cmd_code = 108;
pub const CMD_WIZ_TELEPORT_TO: cmd_code = 107;
pub const CMD_WIZ_TELEPORT_RANDOM: cmd_code = 106;
pub const CMD_WIZ_SUMMON_RANDOM: cmd_code = 105;
pub const CMD_WIZ_SUMMON_NAMED: cmd_code = 104;
pub const CMD_WIZ_STAT_ITEM: cmd_code = 103;
pub const CMD_WIZ_REROLL_ITEM: cmd_code = 102;
pub const CMD_WIZ_RERATE: cmd_code = 101;
pub const CMD_WIZ_RECALL_MONSTER: cmd_code = 100;
pub const CMD_WIZ_QUIT_NO_SAVE: cmd_code = 99;
pub const CMD_WIZ_QUERY_SQUARE_FLAG: cmd_code = 98;
pub const CMD_WIZ_QUERY_FEATURE: cmd_code = 97;
pub const CMD_WIZ_PUSH_OBJECT: cmd_code = 96;
pub const CMD_WIZ_PLAY_ITEM: cmd_code = 95;
pub const CMD_WIZ_PERFORM_EFFECT: cmd_code = 94;
pub const CMD_WIZ_PEEK_NOISE_SCENT: cmd_code = 93;
pub const CMD_WIZ_MAGIC_MAP: cmd_code = 92;
pub const CMD_WIZ_LEARN_OBJECT_KINDS: cmd_code = 91;
pub const CMD_WIZ_JUMP_LEVEL: cmd_code = 90;
pub const CMD_WIZ_INCREASE_EXP: cmd_code = 89;
pub const CMD_WIZ_HIT_ALL_LOS: cmd_code = 88;
pub const CMD_WIZ_EDIT_PLAYER_STAT: cmd_code = 87;
pub const CMD_WIZ_EDIT_PLAYER_START: cmd_code = 86;
pub const CMD_WIZ_EDIT_PLAYER_GOLD: cmd_code = 85;
pub const CMD_WIZ_EDIT_PLAYER_EXP: cmd_code = 84;
pub const CMD_WIZ_DUMP_LEVEL_MAP: cmd_code = 83;
pub const CMD_WIZ_DETECT_ALL_MONSTERS: cmd_code = 82;
pub const CMD_WIZ_DETECT_ALL_LOCAL: cmd_code = 81;
pub const CMD_WIZ_CURSE_ITEM: cmd_code = 80;
pub const CMD_WIZ_CURE_ALL: cmd_code = 79;
pub const CMD_WIZ_CREATE_TRAP: cmd_code = 78;
pub const CMD_WIZ_CREATE_OBJ: cmd_code = 77;
pub const CMD_WIZ_CREATE_ARTIFACT: cmd_code = 76;
pub const CMD_WIZ_CREATE_ALL_OBJ_FROM_TVAL: cmd_code = 75;
pub const CMD_WIZ_CREATE_ALL_OBJ: cmd_code = 74;
pub const CMD_WIZ_CREATE_ALL_ARTIFACT_FROM_TVAL: cmd_code = 73;
pub const CMD_WIZ_CREATE_ALL_ARTIFACT: cmd_code = 72;
pub const CMD_WIZ_COLLECT_PIT_STATS: cmd_code = 71;
pub const CMD_WIZ_COLLECT_OBJ_MON_STATS: cmd_code = 70;
pub const CMD_WIZ_COLLECT_DISCONNECT_STATS: cmd_code = 69;
pub const CMD_WIZ_CHANGE_ITEM_QUANTITY: cmd_code = 68;
pub const CMD_WIZ_BANISH: cmd_code = 67;
pub const CMD_WIZ_ADVANCE: cmd_code = 66;
pub const CMD_WIZ_ACQUIRE: cmd_code = 65;
pub const CMD_SPOIL_OBJ: cmd_code = 64;
pub const CMD_SPOIL_MON_BRIEF: cmd_code = 63;
pub const CMD_SPOIL_MON: cmd_code = 62;
pub const CMD_SPOIL_ARTIFACT: cmd_code = 61;
pub const CMD_RETRIEVE: cmd_code = 60;
pub const CMD_STASH: cmd_code = 59;
pub const CMD_BUY: cmd_code = 58;
pub const CMD_SELL: cmd_code = 57;
pub const CMD_SLEEP: cmd_code = 56;
pub const CMD_STEAL: cmd_code = 55;
pub const CMD_ALTER: cmd_code = 54;
pub const CMD_HOLD: cmd_code = 53;
pub const CMD_NAVIGATE_DOWN: cmd_code = 52;
pub const CMD_NAVIGATE_UP: cmd_code = 51;
pub const CMD_EXPLORE: cmd_code = 50;
pub const CMD_RUN: cmd_code = 49;
pub const CMD_CLOSE: cmd_code = 48;
pub const CMD_OPEN: cmd_code = 47;
pub const CMD_TUNNEL: cmd_code = 46;
pub const CMD_REST: cmd_code = 45;
pub const CMD_DISARM: cmd_code = 44;
pub const CMD_IGNORE: cmd_code = 43;
pub const CMD_AUTOPICKUP: cmd_code = 42;
pub const CMD_PICKUP: cmd_code = 41;
pub const CMD_THROW: cmd_code = 40;
pub const CMD_FIRE: cmd_code = 39;
pub const CMD_USE: cmd_code = 38;
pub const CMD_REFILL: cmd_code = 37;
pub const CMD_READ_SCROLL: cmd_code = 36;
pub const CMD_QUAFF: cmd_code = 35;
pub const CMD_EAT: cmd_code = 34;
pub const CMD_ACTIVATE: cmd_code = 33;
pub const CMD_USE_ROD: cmd_code = 32;
pub const CMD_USE_WAND: cmd_code = 31;
pub const CMD_USE_STAFF: cmd_code = 30;
pub const CMD_CAST: cmd_code = 29;
pub const CMD_STUDY: cmd_code = 28;
pub const CMD_BROWSE_SPELL: cmd_code = 27;
pub const CMD_DROP: cmd_code = 26;
pub const CMD_WIELD: cmd_code = 25;
pub const CMD_TAKEOFF: cmd_code = 24;
pub const CMD_AUTOINSCRIBE: cmd_code = 23;
pub const CMD_UNINSCRIBE: cmd_code = 22;
pub const CMD_INSCRIBE: cmd_code = 21;
pub const CMD_PATHFIND: cmd_code = 20;
pub const CMD_JUMP: cmd_code = 19;
pub const CMD_WALK: cmd_code = 18;
pub const CMD_GO_DOWN: cmd_code = 17;
pub const CMD_GO_UP: cmd_code = 16;
pub const CMD_ACCEPT_CHARACTER: cmd_code = 15;
pub const CMD_HISTORY_CHOICE: cmd_code = 14;
pub const CMD_NAME_CHOICE: cmd_code = 13;
pub const CMD_PREV_STATS: cmd_code = 12;
pub const CMD_ROLL_STATS: cmd_code = 11;
pub const CMD_REFRESH_STATS: cmd_code = 10;
pub const CMD_RESET_STATS: cmd_code = 9;
pub const CMD_SELL_STAT: cmd_code = 8;
pub const CMD_BUY_STAT: cmd_code = 7;
pub const CMD_CHOOSE_CLASS: cmd_code = 6;
pub const CMD_CHOOSE_RACE: cmd_code = 5;
pub const CMD_BIRTH_RESET: cmd_code = 4;
pub const CMD_BIRTH_INIT: cmd_code = 3;
pub const CMD_NEWGAME: cmd_code = 2;
pub const CMD_LOADFILE: cmd_code = 1;
pub const CMD_NULL: cmd_code = 0;
pub type cmd_context = ::core::ffi::c_uint;
pub const CTX_DEATH: cmd_context = 4;
pub const CTX_STORE: cmd_context = 3;
pub const CTX_GAME: cmd_context = 2;
pub const CTX_BIRTH: cmd_context = 1;
pub const CTX_INIT: cmd_context = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct source {
    pub what: C2RustUnnamed_1,
    pub which: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub trap: *mut trap,
    pub monster: ::core::ffi::c_int,
    pub object: *mut object,
    pub chest_trap: *mut chest_trap,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const SRC_CHEST_TRAP: C2RustUnnamed_1 = 5;
pub const SRC_OBJECT: C2RustUnnamed_1 = 4;
pub const SRC_MONSTER: C2RustUnnamed_1 = 3;
pub const SRC_PLAYER: C2RustUnnamed_1 = 2;
pub const SRC_TRAP: C2RustUnnamed_1 = 1;
pub const SRC_NONE: C2RustUnnamed_1 = 0;
pub type effect_index = ::core::ffi::c_uint;
pub const EF_MAX: effect_index = 113;
pub const EF_UNSCRAMBLE_STATS: effect_index = 112;
pub const EF_SCRAMBLE_STATS: effect_index = 111;
pub const EF_CLEAR_VALUE: effect_index = 110;
pub const EF_SET_VALUE: effect_index = 109;
pub const EF_SELECT: effect_index = 108;
pub const EF_WONDER: effect_index = 107;
pub const EF_BIZARRE: effect_index = 106;
pub const EF_SWEEP: effect_index = 105;
pub const EF_MELEE_BLOWS: effect_index = 104;
pub const EF_SINGLE_COMBAT: effect_index = 103;
pub const EF_MOVE_ATTACK: effect_index = 102;
pub const EF_JUMP_AND_BITE: effect_index = 101;
pub const EF_COMMAND: effect_index = 100;
pub const EF_CURSE: effect_index = 99;
pub const EF_SHAPECHANGE: effect_index = 98;
pub const EF_TAP_UNLIFE: effect_index = 97;
pub const EF_TAP_DEVICE: effect_index = 96;
pub const EF_CREATE_ARROWS: effect_index = 95;
pub const EF_BRAND_BOLTS: effect_index = 94;
pub const EF_BRAND_AMMO: effect_index = 93;
pub const EF_BRAND_WEAPON: effect_index = 92;
pub const EF_CURSE_WEAPON: effect_index = 91;
pub const EF_CURSE_ARMOR: effect_index = 90;
pub const EF_TOUCH_AWARE: effect_index = 89;
pub const EF_TOUCH: effect_index = 88;
pub const EF_BOLT_AWARE: effect_index = 87;
pub const EF_BOLT_STATUS_DAM: effect_index = 86;
pub const EF_BOLT_STATUS: effect_index = 85;
pub const EF_ALTER: effect_index = 84;
pub const EF_LINE: effect_index = 83;
pub const EF_BOLT_OR_BEAM: effect_index = 82;
pub const EF_BEAM: effect_index = 81;
pub const EF_BOLT: effect_index = 80;
pub const EF_STAR_BALL: effect_index = 79;
pub const EF_STAR: effect_index = 78;
pub const EF_STRIKE: effect_index = 77;
pub const EF_SWARM: effect_index = 76;
pub const EF_LASH: effect_index = 75;
pub const EF_SHORT_BEAM: effect_index = 74;
pub const EF_ARC: effect_index = 73;
pub const EF_BREATH: effect_index = 72;
pub const EF_BALL: effect_index = 71;
pub const EF_SPHERE: effect_index = 70;
pub const EF_SPOT: effect_index = 69;
pub const EF_DARKEN_AREA: effect_index = 68;
pub const EF_LIGHT_AREA: effect_index = 67;
pub const EF_DARKEN_LEVEL: effect_index = 66;
pub const EF_LIGHT_LEVEL: effect_index = 65;
pub const EF_EARTHQUAKE: effect_index = 64;
pub const EF_DESTRUCTION: effect_index = 63;
pub const EF_GRANITE: effect_index = 62;
pub const EF_RUBBLE: effect_index = 61;
pub const EF_TELEPORT_LEVEL: effect_index = 60;
pub const EF_TELEPORT: effect_index = 58;
pub const EF_PROBE: effect_index = 57;
pub const EF_MASS_BANISH: effect_index = 56;
pub const EF_SUMMON: effect_index = 54;
pub const EF_WAKE: effect_index = 53;
pub const EF_ACQUIRE: effect_index = 52;
pub const EF_PROJECT_LOS_AWARE: effect_index = 51;
pub const EF_PROJECT_LOS: effect_index = 50;
pub const EF_RECHARGE: effect_index = 49;
pub const EF_ENCHANT: effect_index = 48;
pub const EF_DISENCHANT: effect_index = 47;
pub const EF_CREATE_STAIRS: effect_index = 46;
pub const EF_DETECT_SOUL: effect_index = 45;
pub const EF_DETECT_EVIL: effect_index = 44;
pub const EF_IDENTIFY: effect_index = 43;
pub const EF_DETECT_FEARFUL_MONSTERS: effect_index = 42;
pub const EF_DETECT_INVISIBLE_MONSTERS: effect_index = 41;
pub const EF_DETECT_VISIBLE_MONSTERS: effect_index = 40;
pub const EF_DETECT_LIVING_MONSTERS: effect_index = 39;
pub const EF_DETECT_OBJECTS: effect_index = 38;
pub const EF_SENSE_OBJECTS: effect_index = 37;
pub const EF_DETECT_GOLD: effect_index = 36;
pub const EF_SENSE_GOLD: effect_index = 35;
pub const EF_DETECT_ORE: effect_index = 34;
pub const EF_DETECT_STAIRS: effect_index = 33;
pub const EF_DETECT_DOORS: effect_index = 32;
pub const EF_DETECT_TRAPS: effect_index = 31;
pub const EF_READ_MINDS: effect_index = 30;
pub const EF_MAP_AREA: effect_index = 29;
pub const EF_ALTER_REALITY: effect_index = 28;
pub const EF_DEEP_DESCENT: effect_index = 27;
pub const EF_RECALL: effect_index = 26;
pub const EF_REMOVE_CURSE: effect_index = 25;
pub const EF_RESTORE_MANA: effect_index = 24;
pub const EF_DRAIN_MANA: effect_index = 23;
pub const EF_DRAIN_LIGHT: effect_index = 22;
pub const EF_GAIN_EXP: effect_index = 21;
pub const EF_RESTORE_EXP: effect_index = 20;
pub const EF_GAIN_STAT: effect_index = 19;
pub const EF_LOSE_RANDOM_STAT: effect_index = 18;
pub const EF_DRAIN_STAT: effect_index = 17;
pub const EF_RESTORE_STAT: effect_index = 16;
pub const EF_WEB: effect_index = 15;
pub const EF_GLYPH: effect_index = 14;
pub const EF_TIMED_DEC: effect_index = 13;
pub const EF_MON_TIMED_INC: effect_index = 12;
pub const EF_TIMED_INC_NO_RES: effect_index = 11;
pub const EF_TIMED_INC: effect_index = 10;
pub const EF_TIMED_SET: effect_index = 9;
pub const EF_CURE: effect_index = 8;
pub const EF_CRUNCH: effect_index = 7;
pub const EF_NOURISH: effect_index = 6;
pub const EF_MON_HEAL_KIN: effect_index = 5;
pub const EF_MON_HEAL_HP: effect_index = 4;
pub const EF_HEAL_HP: effect_index = 3;
pub const EF_DAMAGE: effect_index = 2;
pub const EF_NONE: effect_index = 0;
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
pub const BI_CDEPTH: C2RustUnnamed_10 = 105;
pub const BI_ISSTUDY: C2RustUnnamed_10 = 123;
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
pub const BI_LIGHT: C2RustUnnamed_10 = 26;
pub const BI_ISCONFUSED: C2RustUnnamed_10 = 114;
pub const BI_ISBLIND: C2RustUnnamed_10 = 112;
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
pub const BI_ISENCUMB: C2RustUnnamed_10 = 122;
pub const BI_ISFORGET: C2RustUnnamed_10 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_10 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_10 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_10 = 118;
pub const BI_ISSTUN: C2RustUnnamed_10 = 117;
pub const BI_ISCUT: C2RustUnnamed_10 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_10 = 115;
pub const BI_ISAFRAID: C2RustUnnamed_10 = 113;
pub const BI_ISGORGED: C2RustUnnamed_10 = 111;
pub const BI_ISFULL: C2RustUnnamed_10 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_10 = 109;
pub const BI_ISWEAK: C2RustUnnamed_10 = 108;
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
pub const BI_IFIRE: C2RustUnnamed_10 = 64;
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
pub const BI_GOLD: C2RustUnnamed_10 = 45;
pub const BI_SPEED: C2RustUnnamed_10 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_10 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_10 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_10 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_10 = 40;
pub const BI_FOOD: C2RustUnnamed_10 = 39;
pub const BI_RECALL: C2RustUnnamed_10 = 38;
pub const BI_ESP: C2RustUnnamed_10 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_10 = 36;
pub const BI_CLEVEL: C2RustUnnamed_10 = 35;
pub const BI_FAIL2: C2RustUnnamed_10 = 34;
pub const BI_FAIL1: C2RustUnnamed_10 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_10 = 32;
pub const BI_MAXSP: C2RustUnnamed_10 = 31;
pub const BI_CURSP: C2RustUnnamed_10 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_10 = 29;
pub const BI_MAXHP: C2RustUnnamed_10 = 28;
pub const BI_CURHP: C2RustUnnamed_10 = 27;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut effects: [effect_kind; 114] = unsafe {
    [
        effect_kind {
            index: EF_NONE as ::core::ffi::c_int as uint16_t,
            aim: false_0 != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: None,
            desc: ::core::ptr::null::<::core::ffi::c_char>(),
            menu_name: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        effect_kind {
            index: EF_RANDOM as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RANDOM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"randomly \0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DAMAGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"hurt\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_DAMAGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"does %s damage to the player\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_HEAL_HP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"heal\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_HEAL_HP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals %s hitpoints%s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"heal self\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_HEAL_HP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_HEAL_HP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals monster hitpoints\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_HEAL_KIN as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_HEAL_KIN
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"heals fellow monster hitpoints\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_NOURISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_NOURISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s for %s turns (%s percent)\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CRUNCH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CRUNCH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"crunches\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"cures %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"cure %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_SET as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TIMED_SET
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"administers %s for %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"administer %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_INC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dur\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TIMED_INC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"extends %s for %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"extend %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_INC_NO_RES as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dur\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TIMED_INC_NO_RES
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"extends %s for %s turns (unresistable)\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"extend %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MON_TIMED_INC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MON_TIMED_INC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"increases monster %s by %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TIMED_DEC as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TIMED_DEC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces length of %s by %s turns\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"reduce %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GLYPH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GLYPH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"inscribes a glyph beneath you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"inscribe a glyph\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WEB as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WEB
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a web\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"create a web\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"restore %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LOSE_RANDOM_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LOSE_RANDOM_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"reduces a stat other than %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GAIN_STAT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GAIN_STAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"increases your %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_EXP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_EXP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores your experience\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"restore experience\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GAIN_EXP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GAIN_EXP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"grants %d experience points\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_LIGHT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_LIGHT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains your light source\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DRAIN_MANA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DRAIN_MANA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains some mana\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RESTORE_MANA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RESTORE_MANA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"restores some mana\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"restore some mana\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_REMOVE_CURSE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_REMOVE_CURSE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"attempts power %s removal of a single curse on an object\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"remove curse\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RECALL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RECALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"returns you from the dungeon or takes you to the dungeon after a short delay\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"recall\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DEEP_DESCENT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DEEP_DESCENT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports you up to five dungeon levels lower than the lowest point you have reached so far\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"descend to the depths\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ALTER_REALITY as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ALTER_REALITY
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a new dungeon level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"alter reality\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MAP_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MAP_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"maps the area around you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"map surroundings\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_READ_MINDS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_READ_MINDS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"maps the area around recently detected monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"read minds\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_TRAPS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_TRAPS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects traps nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect traps\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_DOORS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_DOORS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects doors nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect doors\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_STAIRS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_STAIRS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects stairs nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect stairs\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_ORE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_ORE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects veins nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect veins\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SENSE_GOLD as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SENSE_GOLD
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"senses gold nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"sense gold\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_GOLD as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_GOLD
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects gold nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect gold\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SENSE_OBJECTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SENSE_OBJECTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"senses objects nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"sense objects\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_OBJECTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_OBJECTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects objects nearby\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect objects\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_LIVING_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_LIVING_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects living creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect living\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_VISIBLE_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_VISIBLE_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects visible creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect visible\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_INVISIBLE_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_INVISIBLE_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects invisible creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect invisible\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_FEARFUL_MONSTERS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_FEARFUL_MONSTERS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects creatures nearby which are susceptible to fear\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"detect fearful\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_IDENTIFY as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_IDENTIFY
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"identifies a single unknown rune on a selected item\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"identify\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_EVIL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_EVIL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects evil creatures nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect evil\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DETECT_SOUL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DETECT_SOUL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"detects creatures with a spirit nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"detect souls\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CREATE_STAIRS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CREATE_STAIRS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a staircase beneath your feet\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create stairs\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DISENCHANT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DISENCHANT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"disenchants one of your wielded items\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"disenchant item\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ENCHANT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ENCHANT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"attempts to magically enhance an item\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"enchant item\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RECHARGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_RECHARGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"tries to recharge a wand or staff, destroying the wand or staff on failure\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"recharge\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROJECT_LOS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_PROJECT_LOS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s which are in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROJECT_LOS_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"power\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_PROJECT_LOS_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s which are in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s in line of sight\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ACQUIRE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ACQUIRE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates good items nearby\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create good items\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WAKE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WAKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"awakens all nearby sleeping monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"awaken all\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SUMMON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SUMMON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"summons %s at the current dungeon level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"summon %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BANISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BANISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"removes all of a given creature type from the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"banish\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MASS_BANISH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_MASS_BANISH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"removes all nearby creatures\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"banish all\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_PROBE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_PROBE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"gives you information on the health and abilities of monsters you can see\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"probe\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"range\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TELEPORT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports %s randomly %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT_TO as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TELEPORT_TO
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports toward a target\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport to target\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TELEPORT_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TELEPORT_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"teleports you one level up or down\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"teleport level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_RUBBLE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_RUBBLE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes rubble to fall around you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_GRANITE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_GRANITE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes a granite wall to fall behind you\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DESTRUCTION as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DESTRUCTION
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"destroys an area around you in the shape of a circle radius %d, and blinds you for 1d10+10 turns\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"destroy area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_EARTHQUAKE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_EARTHQUAKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"causes an earthquake around you of radius %d\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cause earthquake\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LIGHT_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LIGHT_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"completely lights up and magically maps the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"light level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DARKEN_LEVEL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DARKEN_LEVEL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"completely darkens up and magically maps the level\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"darken level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LIGHT_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LIGHT_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"lights up the surrounding area\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"light area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_DARKEN_AREA as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_DARKEN_AREA
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"darkens the surrounding area\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"darken area\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SPOT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SPOT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, centred on and hitting the player, with full intensity to radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"engulf with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SPHERE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SPHERE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, centred on the player, with full intensity to radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"project %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BALL as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a ball of %s with radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a ball of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BREATH as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BREATH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"breathes a cone of %s with width %d degrees, dealing %s damage at the source\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"breathe a cone of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ARC as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_ARC
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"produces a cone of %s with width %d degrees, dealing %s damage at the source\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"produce a cone of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SHORT_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SHORT_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"produces a beam of %s with length %d, dealing %s damage\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"produce a beam of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LASH as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_LASH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a beam of %s length %d, dealing damage determined by blows\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"lash with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SWARM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SWARM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a series of %s balls of radius %d, dealing %s damage at the centre of each\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a swarm of %s balls\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STRIKE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STRIKE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a ball of %s with radius %d, dealing %s damage at the centre\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"strike with %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STAR as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STAR
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires a line of %s in all directions, each dealing %s damage\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire a line of %s in all directions\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_STAR_BALL as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_STAR_BALL
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"fires balls of %s with radius %d in all directions, dealing %s damage at the centre of each\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"fire balls of %s in all directions\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a beam of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a beam of %s\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_OR_BEAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT_OR_BEAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt or beam of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt or beam of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_LINE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_LINE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a line of %s dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a line of %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_ALTER as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_ALTER
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a line which %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a line which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_STATUS as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BOLT_STATUS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt which %s\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_STATUS_DAM as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_BOLT_STATUS_DAM
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"casts a bolt which %s, dealing %s damage\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"cast a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BOLT_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BOLT_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create a bolt which %s\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TOUCH as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TOUCH
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s on all adjacent squares\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s all adjacent\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TOUCH_AWARE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TOUCH_AWARE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"%s on all adjacent squares\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"%s all adjacent\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE_ARMOR as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURSE_ARMOR
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"curses your worn armor\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"curse armor\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE_WEAPON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CURSE_WEAPON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"curses your wielded melee weapon\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"curse weapon\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_WEAPON as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_WEAPON
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands your wielded melee weapon\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand weapon\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_AMMO as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_AMMO
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands a stack of ammunition\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand ammunition\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BRAND_BOLTS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BRAND_BOLTS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"brands bolts with fire, in an unbalanced fashion\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"brand bolts\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CREATE_ARROWS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CREATE_ARROWS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"uses a staff to create a stack of arrows\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"use staff for arrows\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TAP_DEVICE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_TAP_DEVICE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains magical energy from a staff or wand\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"tap device\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_TAP_UNLIFE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_TAP_UNLIFE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"drains %s mana from the closest undead monster, damaging it\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"tap unlife\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SHAPECHANGE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SHAPECHANGE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"changes the player's shape\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"change shape\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CURSE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"dam\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_CURSE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"damages a monster directly\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"curse\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_COMMAND as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_COMMAND
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"takes control of a monster\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"command\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_JUMP_AND_BITE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_JUMP_AND_BITE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"jumps the player to the closest living monster and bites it\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"jump and bite\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MOVE_ATTACK as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_MOVE_ATTACK
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"moves the player up to 4 spaces and executes up to %d melee blows\0"
                as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"move and attack\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SINGLE_COMBAT as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SINGLE_COMBAT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"engages a monster in single combat\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"engage in single combat\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MELEE_BLOWS as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_MELEE_BLOWS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"strikes %d blows against an adjacent monster\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"pummel\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SWEEP as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: b"blows\0" as *const u8 as *const ::core::ffi::c_char,
            handler: Some(
                effect_handler_SWEEP
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"strikes %d blows against all adjacent monsters\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"sweep\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_BIZARRE as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_BIZARRE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"does bizarre things\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"do bizarre things\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_WONDER as ::core::ffi::c_int as uint16_t,
            aim: 1 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_WONDER
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"creates random and unpredictable effects\0" as *const u8
                as *const ::core::ffi::c_char,
            menu_name: b"create random effects\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SELECT as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SELECT
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"selects one of \0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SET_VALUE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SET_VALUE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_CLEAR_VALUE as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_CLEAR_VALUE
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_SCRAMBLE_STATS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_SCRAMBLE_STATS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_UNSCRAMBLE_STATS as ::core::ffi::c_int as uint16_t,
            aim: 0 as ::core::ffi::c_int != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: Some(
                effect_handler_UNSCRAMBLE_STATS
                    as unsafe extern "C" fn(*mut effect_handler_context_t) -> bool,
            ),
            desc: b"\0" as *const u8 as *const ::core::ffi::c_char,
            menu_name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        effect_kind {
            index: EF_MAX as ::core::ffi::c_int as uint16_t,
            aim: false_0 != 0,
            info: ::core::ptr::null::<::core::ffi::c_char>(),
            handler: None,
            desc: ::core::ptr::null::<::core::ffi::c_char>(),
            menu_name: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ]
};
unsafe extern "C" fn borg_can_play_spell(mut as_0: *mut borg_magic) -> bool {
    match (*as_0).spell_enum as ::core::ffi::c_uint {
        40 | 89 | 17 | 68 | 47 | 81 | 101 | 116 | 78 | 110 | 95 | 109 | 108 | 131 | 124 => {
            return false_0 != 0
        }
        _ => {}
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_BRAND_BOLTS as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_CREATE_ARROWS as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_BRAND_AMMO as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_ENCHANT as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_IDENTIFY as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*as_0).effect_index as ::core::ffi::c_int == EF_RECHARGE as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_play_magic(mut bored: bool) -> bool {
    let mut r: ::core::ffi::c_int = 0;
    let mut b_r: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut spell_num: ::core::ffi::c_int = 0;
    let mut b_spell_num: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*(*player).class).magic.total_spells == 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
        != 0
        || *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            != 0
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 {
        return false_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).info as ::core::ffi::c_int
        == BORG_DARK
    {
        return false_0 != 0;
    }
    spell_num = (*(*player).class).magic.total_spells - 1 as ::core::ffi::c_int;
    while spell_num >= 0 as ::core::ffi::c_int {
        let mut as_0: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
        if !((borg.book_idx[(*as_0).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
        {
            if !((*as_0).status as ::core::ffi::c_int != BORG_MAGIC_OKAY) {
                r = (*as_0).rating as ::core::ffi::c_int;
                if !(!bored && r <= 50 as ::core::ffi::c_int) {
                    if !(r <= 0 as ::core::ffi::c_int) {
                        if !(r <= b_r) {
                            b_r = r;
                            b_spell_num = spell_num;
                        }
                    }
                }
            }
        }
        spell_num -= 1;
    }
    if *borg
        .trait_0
        .offset(BI_ISSTUDY as ::core::ffi::c_int as isize)
        != 0
        && b_r > 0 as ::core::ffi::c_int
        && b_spell_num >= 0 as ::core::ffi::c_int
    {
        let mut as_1: *mut borg_magic = borg_magics.offset(b_spell_num as isize) as *mut borg_magic;
        borg_note(format(
            b"# Studying spell/prayer %s.\0" as *const u8 as *const ::core::ffi::c_char,
            (*as_1).name,
        ));
        borg_keypress('G' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset(borg.book_idx[(*as_1).book as usize] as isize) as keycode_t,
        );
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
            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
        ) {
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((*as_1).book_offset as isize) as keycode_t,
            );
        }
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && !borg.munchkin_mode
    {
        return false_0 != 0;
    }
    if !bored {
        return false_0 != 0;
    }
    let mut current_block_37: u64;
    spell_num = (*(*player).class).magic.total_spells - 1 as ::core::ffi::c_int;
    while spell_num >= 0 as ::core::ffi::c_int {
        let mut as_2: *mut borg_magic = borg_magics.offset(spell_num as isize) as *mut borg_magic;
        if !((borg.book_idx[(*as_2).book as usize] as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
        {
            if !((*as_2).status as ::core::ffi::c_int != BORG_MAGIC_TEST) {
                if borg_can_play_spell(as_2) {
                    if borg.munchkin_mode {
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
                            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int
                            != 0
                            && (*as_2).spell_enum as ::core::ffi::c_uint
                                == MAGIC_MISSILE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            current_block_37 = 4068382217303356765;
                        } else if !flag_has(
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
                            PF_CHOOSE_SPELLS as ::core::ffi::c_int,
                        ) && (*as_2).spell_enum as ::core::ffi::c_uint
                            == BLESS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            current_block_37 = 4068382217303356765;
                        } else {
                            current_block_37 = 18153031941552419006;
                        }
                    } else {
                        current_block_37 = 18153031941552419006;
                    }
                    match current_block_37 {
                        4068382217303356765 => {}
                        _ => {
                            borg_note(
                                b"# Testing untried spell/prayer\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            if borg_spell((*as_2).spell_enum) {
                                if effects[(*as_2).effect_index as usize].aim as ::core::ffi::c_int
                                    != 0
                                    || (*as_2).effect_index as ::core::ffi::c_int
                                        == EF_RANDOM as ::core::ffi::c_int
                                    || (*as_2).effect_index as ::core::ffi::c_int
                                        == EF_TELEPORT_TO as ::core::ffi::c_int
                                {
                                    borg_keypress('*' as i32 as keycode_t);
                                    borg_keypress('t' as i32 as keycode_t);
                                }
                                if (*as_2).effect_index as ::core::ffi::c_int
                                    == EF_BANISH as ::core::ffi::c_int
                                {
                                    borg_keypress('h' as i32 as keycode_t);
                                }
                                return true_0 != 0;
                            }
                        }
                    }
                }
            }
        }
        spell_num -= 1;
    }
    return false_0 != 0;
}
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_MAGIC_OKAY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BORG_MAGIC_TEST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
