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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    static mut Rand_quick: bool;
    static mut Rand_value: uint32_t;
    static mut player: *mut player;
    fn borg_notice_player();
    static mut inkey_hack: Option<unsafe extern "C" fn(::core::ffi::c_int) -> keypress>;
    fn Term_fresh() -> errr;
    fn Term_locate(x: *mut ::core::ffi::c_int, y: *mut ::core::ffi::c_int) -> errr;
    fn Term_inkey(ch: *mut ui_event, wait: bool, take: bool) -> errr;
    fn cmd_get_nrepeats() -> ::core::ffi::c_int;
    fn flush(unused: game_event_type, data: *mut game_event_data, user: *mut ::core::ffi::c_void);
    fn borg_reset_settings();
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_save_keypress(kp: keypress) -> keypress;
    fn borg_inkey(take: bool) -> keycode_t;
    fn borg_flush();
    fn borg_log_death();
    fn borg_log_death_data();
    fn borg_display_status();
    fn borg_write_map(ask: bool);
    fn borg_parse(msg: *mut ::core::ffi::c_char);
    fn borg_get_messages(key: *mut keypress, cursor: loc) -> bool;
    fn borg_clear_reactions();
    fn borg_force_reincarnate();
    fn borg_reincarnate_end();
    fn borg_oops(what: *const ::core::ffi::c_char);
    fn borg_think() -> bool;
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
pub type game_event_type = ::core::ffi::c_uint;
pub const EVENT_END: game_event_type = 64;
pub const EVENT_GEN_TUNNEL_FINISHED: game_event_type = 63;
pub const EVENT_GEN_ROOM_END: game_event_type = 62;
pub const EVENT_GEN_ROOM_CHOOSE_SUBTYPE: game_event_type = 61;
pub const EVENT_GEN_ROOM_CHOOSE_SIZE: game_event_type = 60;
pub const EVENT_GEN_ROOM_START: game_event_type = 59;
pub const EVENT_GEN_LEVEL_END: game_event_type = 58;
pub const EVENT_GEN_LEVEL_START: game_event_type = 57;
pub const EVENT_LEAVE_DEATH: game_event_type = 56;
pub const EVENT_ENTER_DEATH: game_event_type = 55;
pub const EVENT_LEAVE_STORE: game_event_type = 54;
pub const EVENT_ENTER_STORE: game_event_type = 53;
pub const EVENT_LEAVE_WORLD: game_event_type = 52;
pub const EVENT_ENTER_WORLD: game_event_type = 51;
pub const EVENT_LEAVE_GAME: game_event_type = 50;
pub const EVENT_ENTER_GAME: game_event_type = 49;
pub const EVENT_LEAVE_BIRTH: game_event_type = 48;
pub const EVENT_ENTER_BIRTH: game_event_type = 47;
pub const EVENT_LEAVE_INIT: game_event_type = 46;
pub const EVENT_ENTER_INIT: game_event_type = 45;
pub const EVENT_BIRTHPOINTS: game_event_type = 44;
pub const EVENT_INITSTATUS: game_event_type = 43;
pub const EVENT_CHEAT_DEATH: game_event_type = 42;
pub const EVENT_ANIMATE: game_event_type = 41;
pub const EVENT_COMMAND_REPEAT: game_event_type = 40;
pub const EVENT_NEW_LEVEL_DISPLAY: game_event_type = 39;
pub const EVENT_REFRESH: game_event_type = 38;
pub const EVENT_CHECK_INTERRUPT: game_event_type = 37;
pub const EVENT_MESSAGE_FLUSH: game_event_type = 36;
pub const EVENT_INPUT_FLUSH: game_event_type = 35;
pub const EVENT_STORECHANGED: game_event_type = 34;
pub const EVENT_USE_STORE: game_event_type = 33;
pub const EVENT_BELL: game_event_type = 32;
pub const EVENT_SOUND: game_event_type = 31;
pub const EVENT_MESSAGE: game_event_type = 30;
pub const EVENT_OBJECTTARGET: game_event_type = 29;
pub const EVENT_MONSTERTARGET: game_event_type = 28;
pub const EVENT_MONSTERLIST: game_event_type = 27;
pub const EVENT_ITEMLIST: game_event_type = 26;
pub const EVENT_EQUIPMENT: game_event_type = 25;
pub const EVENT_INVENTORY: game_event_type = 24;
pub const EVENT_MISSILE: game_event_type = 23;
pub const EVENT_BOLT: game_event_type = 22;
pub const EVENT_EXPLOSION: game_event_type = 21;
pub const EVENT_SEEFLOOR: game_event_type = 20;
pub const EVENT_PLAYERMOVED: game_event_type = 19;
pub const EVENT_STATE: game_event_type = 18;
pub const EVENT_LIGHT: game_event_type = 17;
pub const EVENT_FEELING: game_event_type = 16;
pub const EVENT_DETECTIONSTATUS: game_event_type = 15;
pub const EVENT_STATUS: game_event_type = 14;
pub const EVENT_STUDYSTATUS: game_event_type = 13;
pub const EVENT_RACE_CLASS: game_event_type = 12;
pub const EVENT_PLAYERSPEED: game_event_type = 11;
pub const EVENT_DUNGEONLEVEL: game_event_type = 10;
pub const EVENT_MONSTERHEALTH: game_event_type = 9;
pub const EVENT_GOLD: game_event_type = 8;
pub const EVENT_PLAYERTITLE: game_event_type = 7;
pub const EVENT_PLAYERLEVEL: game_event_type = 6;
pub const EVENT_EXPERIENCE: game_event_type = 5;
pub const EVENT_AC: game_event_type = 4;
pub const EVENT_MANA: game_event_type = 3;
pub const EVENT_HP: game_event_type = 2;
pub const EVENT_STATS: game_event_type = 1;
pub const EVENT_MAP: game_event_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union game_event_data {
    pub point: loc,
    pub string: *const ::core::ffi::c_char,
    pub flag: bool,
    pub message: C2RustUnnamed_6,
    pub birthstage: C2RustUnnamed_5,
    pub birthpoints: C2RustUnnamed_4,
    pub explosion: C2RustUnnamed_3,
    pub bolt: C2RustUnnamed_2,
    pub missile: C2RustUnnamed_1,
    pub size: C2RustUnnamed_0,
    pub tunnel: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub nstep: ::core::ffi::c_int,
    pub npierce: ::core::ffi::c_int,
    pub ndug: ::core::ffi::c_int,
    pub dstart: ::core::ffi::c_int,
    pub dend: ::core::ffi::c_int,
    pub early: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub h: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub obj: *mut object,
    pub seen: bool,
    pub y: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
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
pub struct C2RustUnnamed_2 {
    pub proj_type: ::core::ffi::c_int,
    pub drawing: bool,
    pub seen: bool,
    pub beam: bool,
    pub oy: ::core::ffi::c_int,
    pub ox: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub proj_type: ::core::ffi::c_int,
    pub num_grids: ::core::ffi::c_int,
    pub distance_to_grid: *mut ::core::ffi::c_int,
    pub drawing: bool,
    pub player_sees_grid: *mut bool,
    pub blast_grid: *mut loc,
    pub centre: loc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub points: *const ::core::ffi::c_int,
    pub inc_points: *const ::core::ffi::c_int,
    pub remaining: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub reset: bool,
    pub hint: *const ::core::ffi::c_char,
    pub n_choices: ::core::ffi::c_int,
    pub initial_choice: ::core::ffi::c_int,
    pub choices: *mut *const ::core::ffi::c_char,
    pub helptexts: *mut *const ::core::ffi::c_char,
    pub xtra: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub msg: *const ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_7 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_7 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_7 = 263;
pub const BI_EMPTY: C2RustUnnamed_7 = 262;
pub const BI_CARRY: C2RustUnnamed_7 = 261;
pub const BI_WEIGHT: C2RustUnnamed_7 = 260;
pub const BI_DINV: C2RustUnnamed_7 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_7 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_7 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_7 = 256;
pub const BI_ADIGGER: C2RustUnnamed_7 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_7 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_7 = 253;
pub const BI_AROD2: C2RustUnnamed_7 = 252;
pub const BI_AROD1: C2RustUnnamed_7 = 251;
pub const BI_ASHROOM: C2RustUnnamed_7 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_7 = 249;
pub const BI_AHWORD: C2RustUnnamed_7 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_7 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_7 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_7 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_7 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_7 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_7 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_7 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_7 = 240;
pub const BI_ABRAND: C2RustUnnamed_7 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_7 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_7 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_7 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_7 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_7 = 234;
pub const BI_ACLW: C2RustUnnamed_7 = 233;
pub const BI_ACSW: C2RustUnnamed_7 = 232;
pub const BI_ACCW: C2RustUnnamed_7 = 231;
pub const BI_AGLYPH: C2RustUnnamed_7 = 230;
pub const BI_APFE: C2RustUnnamed_7 = 229;
pub const BI_ALITE: C2RustUnnamed_7 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_7 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_7 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_7 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_7 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_7 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_7 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_7 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_7 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_7 = 219;
pub const BI_ASPEED: C2RustUnnamed_7 = 218;
pub const BI_AID: C2RustUnnamed_7 = 217;
pub const BI_ALIFE: C2RustUnnamed_7 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_7 = 215;
pub const BI_AHEAL: C2RustUnnamed_7 = 214;
pub const BI_AFUEL: C2RustUnnamed_7 = 213;
pub const BI_AESCAPE: C2RustUnnamed_7 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_7 = 211;
pub const BI_APHASE: C2RustUnnamed_7 = 210;
pub const BI_WB_POIS: C2RustUnnamed_7 = 209;
pub const BI_WB_COLD: C2RustUnnamed_7 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_7 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_7 = 206;
pub const BI_WB_ACID: C2RustUnnamed_7 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_7 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_7 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_7 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_7 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_7 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_7 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_7 = 198;
pub const BI_WS_ORC: C2RustUnnamed_7 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_7 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_7 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_7 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_7 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_7 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_7 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_7 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_7 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_7 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_7 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_7 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_7 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_7 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_7 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_7 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_7 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_7 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_7 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_7 = 178;
pub const BI_CRSSICK: C2RustUnnamed_7 = 177;
pub const BI_CRSDULL: C2RustUnnamed_7 = 176;
pub const BI_CRSVULN: C2RustUnnamed_7 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_7 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_7 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_7 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_7 = 171;
pub const BI_CRSSUND: C2RustUnnamed_7 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_7 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_7 = 168;
pub const BI_CRSPARA: C2RustUnnamed_7 = 167;
pub const BI_CRSHALU: C2RustUnnamed_7 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_7 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_7 = 164;
pub const BI_CRSTELE: C2RustUnnamed_7 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_7 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_7 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_7 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_7 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_7 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_7 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_7 = 156;
pub const BI_AMISSILES: C2RustUnnamed_7 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_7 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_7 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_7 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_7 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_7 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_7 = 149;
pub const BI_SHOTS: C2RustUnnamed_7 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_7 = 147;
pub const BI_BLOWS: C2RustUnnamed_7 = 146;
pub const BI_BART: C2RustUnnamed_7 = 145;
pub const BI_SLING: C2RustUnnamed_7 = 144;
pub const BI_BTODAM: C2RustUnnamed_7 = 143;
pub const BI_BTOHIT: C2RustUnnamed_7 = 142;
pub const BI_BID: C2RustUnnamed_7 = 141;
pub const BI_WDS: C2RustUnnamed_7 = 140;
pub const BI_WDD: C2RustUnnamed_7 = 139;
pub const BI_WID: C2RustUnnamed_7 = 138;
pub const BI_WTODAM: C2RustUnnamed_7 = 137;
pub const BI_WTOHIT: C2RustUnnamed_7 = 136;
pub const BI_TODAM: C2RustUnnamed_7 = 135;
pub const BI_TOHIT: C2RustUnnamed_7 = 134;
pub const BI_ARMOR: C2RustUnnamed_7 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_7 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_7 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_7 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_7 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_7 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_7 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_7 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_7 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_7 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_7 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_7 = 122;
pub const BI_ISFORGET: C2RustUnnamed_7 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_7 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_7 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_7 = 118;
pub const BI_ISSTUN: C2RustUnnamed_7 = 117;
pub const BI_ISCUT: C2RustUnnamed_7 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_7 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_7 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_7 = 113;
pub const BI_ISBLIND: C2RustUnnamed_7 = 112;
pub const BI_ISGORGED: C2RustUnnamed_7 = 111;
pub const BI_ISFULL: C2RustUnnamed_7 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_7 = 109;
pub const BI_ISWEAK: C2RustUnnamed_7 = 108;
pub const BI_KING: C2RustUnnamed_7 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_7 = 106;
pub const BI_CDEPTH: C2RustUnnamed_7 = 105;
pub const BI_SFRACT: C2RustUnnamed_7 = 104;
pub const BI_SHLIFE: C2RustUnnamed_7 = 103;
pub const BI_SRDIS: C2RustUnnamed_7 = 102;
pub const BI_SRKAOS: C2RustUnnamed_7 = 101;
pub const BI_SRNTHR: C2RustUnnamed_7 = 100;
pub const BI_SRNXUS: C2RustUnnamed_7 = 99;
pub const BI_SRSHRD: C2RustUnnamed_7 = 98;
pub const BI_SRSND: C2RustUnnamed_7 = 97;
pub const BI_SRCONF: C2RustUnnamed_7 = 96;
pub const BI_SRBLIND: C2RustUnnamed_7 = 95;
pub const BI_SRDARK: C2RustUnnamed_7 = 94;
pub const BI_SRLITE: C2RustUnnamed_7 = 93;
pub const BI_SRFEAR: C2RustUnnamed_7 = 92;
pub const BI_SRPOIS: C2RustUnnamed_7 = 91;
pub const BI_SRACID: C2RustUnnamed_7 = 90;
pub const BI_SRELEC: C2RustUnnamed_7 = 89;
pub const BI_SRCOLD: C2RustUnnamed_7 = 88;
pub const BI_SRFIRE: C2RustUnnamed_7 = 87;
pub const BI_FRACT: C2RustUnnamed_7 = 86;
pub const BI_HLIFE: C2RustUnnamed_7 = 85;
pub const BI_RDIS: C2RustUnnamed_7 = 84;
pub const BI_RKAOS: C2RustUnnamed_7 = 83;
pub const BI_RNTHR: C2RustUnnamed_7 = 82;
pub const BI_RNXUS: C2RustUnnamed_7 = 81;
pub const BI_RSHRD: C2RustUnnamed_7 = 80;
pub const BI_RSND: C2RustUnnamed_7 = 79;
pub const BI_RCONF: C2RustUnnamed_7 = 78;
pub const BI_RBLIND: C2RustUnnamed_7 = 77;
pub const BI_RDARK: C2RustUnnamed_7 = 76;
pub const BI_RLITE: C2RustUnnamed_7 = 75;
pub const BI_RFEAR: C2RustUnnamed_7 = 74;
pub const BI_RPOIS: C2RustUnnamed_7 = 73;
pub const BI_RACID: C2RustUnnamed_7 = 72;
pub const BI_RELEC: C2RustUnnamed_7 = 71;
pub const BI_RCOLD: C2RustUnnamed_7 = 70;
pub const BI_RFIRE: C2RustUnnamed_7 = 69;
pub const BI_IPOIS: C2RustUnnamed_7 = 68;
pub const BI_IELEC: C2RustUnnamed_7 = 67;
pub const BI_ICOLD: C2RustUnnamed_7 = 66;
pub const BI_IACID: C2RustUnnamed_7 = 65;
pub const BI_IFIRE: C2RustUnnamed_7 = 64;
pub const BI_DIG: C2RustUnnamed_7 = 63;
pub const BI_THT: C2RustUnnamed_7 = 62;
pub const BI_THB: C2RustUnnamed_7 = 61;
pub const BI_THN: C2RustUnnamed_7 = 60;
pub const BI_SRCH: C2RustUnnamed_7 = 59;
pub const BI_STL: C2RustUnnamed_7 = 58;
pub const BI_SAV: C2RustUnnamed_7 = 57;
pub const BI_DEV: C2RustUnnamed_7 = 56;
pub const BI_DISM: C2RustUnnamed_7 = 55;
pub const BI_DISP: C2RustUnnamed_7 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_7 = 53;
pub const BI_INFRA: C2RustUnnamed_7 = 52;
pub const BI_SINV: C2RustUnnamed_7 = 51;
pub const BI_REG: C2RustUnnamed_7 = 50;
pub const BI_FEATH: C2RustUnnamed_7 = 49;
pub const BI_SDIG: C2RustUnnamed_7 = 48;
pub const BI_DAM_RED: C2RustUnnamed_7 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_7 = 46;
pub const BI_GOLD: C2RustUnnamed_7 = 45;
pub const BI_SPEED: C2RustUnnamed_7 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_7 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_7 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_7 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_7 = 40;
pub const BI_FOOD: C2RustUnnamed_7 = 39;
pub const BI_RECALL: C2RustUnnamed_7 = 38;
pub const BI_ESP: C2RustUnnamed_7 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_7 = 36;
pub const BI_CLEVEL: C2RustUnnamed_7 = 35;
pub const BI_FAIL2: C2RustUnnamed_7 = 34;
pub const BI_FAIL1: C2RustUnnamed_7 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_7 = 32;
pub const BI_MAXSP: C2RustUnnamed_7 = 31;
pub const BI_CURSP: C2RustUnnamed_7 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_7 = 29;
pub const BI_MAXHP: C2RustUnnamed_7 = 28;
pub const BI_CURHP: C2RustUnnamed_7 = 27;
pub const BI_LIGHT: C2RustUnnamed_7 = 26;
pub const BI_CLASS: C2RustUnnamed_7 = 25;
pub const BI_SCON: C2RustUnnamed_7 = 24;
pub const BI_SDEX: C2RustUnnamed_7 = 23;
pub const BI_SWIS: C2RustUnnamed_7 = 22;
pub const BI_SINT: C2RustUnnamed_7 = 21;
pub const BI_SSTR: C2RustUnnamed_7 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_7 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_7 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_7 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_7 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_7 = 15;
pub const BI_CCON: C2RustUnnamed_7 = 14;
pub const BI_CDEX: C2RustUnnamed_7 = 13;
pub const BI_CWIS: C2RustUnnamed_7 = 12;
pub const BI_CINT: C2RustUnnamed_7 = 11;
pub const BI_CSTR: C2RustUnnamed_7 = 10;
pub const BI_ACON: C2RustUnnamed_7 = 9;
pub const BI_ADEX: C2RustUnnamed_7 = 8;
pub const BI_AWIS: C2RustUnnamed_7 = 7;
pub const BI_AINT: C2RustUnnamed_7 = 6;
pub const BI_ASTR: C2RustUnnamed_7 = 5;
pub const BI_CON: C2RustUnnamed_7 = 4;
pub const BI_DEX: C2RustUnnamed_7 = 3;
pub const BI_WIS: C2RustUnnamed_7 = 2;
pub const BI_INT: C2RustUnnamed_7 = 1;
pub const BI_STR: C2RustUnnamed_7 = 0;
pub type borg_time = int32_t;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_8 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_8 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_8 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_8 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_8 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_8 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_8 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_8 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_8 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_8 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_8 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_8 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_8 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_8 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_8 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_8 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_8 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_8 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_8 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_8 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_8 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_8 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_8 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_8 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_8 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_8 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_8 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_8 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_8 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_8 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_8 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_8 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_8 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_8 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_8 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_8 = 0;
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
    pub temp: C2RustUnnamed_16,
    pub status: C2RustUnnamed_15,
    pub time: C2RustUnnamed_14,
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
    pub goal: C2RustUnnamed_13,
    pub init_save: C2RustUnnamed_12,
    pub mon: C2RustUnnamed_11,
    pub id: C2RustUnnamed_10,
    pub near: C2RustUnnamed_9,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub struct C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub struct C2RustUnnamed_16 {
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut borg: borg_struct = borg_struct {
    player: ::core::ptr::null::<player>() as *mut player,
    trait_0: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    has: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    activation: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    power: 0,
    c: loc { x: 0, y: 0 },
    panels: loc { x: 0, y: 0 },
    avoidance: 0,
    oldchp: 0,
    lunal_mode: false,
    munchkin_mode: false,
    stair_less: false,
    stair_more: false,
    in_shop: false,
    ready_morgoth: 0,
    temp: C2RustUnnamed_16 {
        need_see_invis: 0,
        see_inv: 0,
        res_fire: false,
        res_cold: false,
        res_acid: false,
        res_elec: false,
        res_pois: false,
        prot_from_evil: false,
        fast: false,
        bless: false,
        hero: false,
        berserk: false,
        fastcast: false,
        regen: false,
        smite_evil: false,
        venom: false,
        shield: false,
    },
    status: C2RustUnnamed_15 {
        active: false,
        cancel: false,
        save: false,
        old_depth: 0,
        respawning: 0,
        cheat_death: false,
        anti_summon: false,
        digging: false,
        need_alter: false,
        no_alter: false,
        redraw: false,
        vault: false,
        desperate: false,
    },
    time: C2RustUnnamed_14 {
        now: 0,
        level: 0,
        town: 0,
        morgoth: 0,
        antisummon: 0,
        call_light: 0,
        wizard_light: 0,
        detect_traps: 0,
        detect_doors: 0,
        detect_walls: 0,
        detect_evil: 0,
        detect_obj: 0,
        last_kill_mult: 0,
    },
    need_see_invis: 0,
    see_inv: 0,
    need_shift_panel: false,
    when_shift_panel: 0,
    antibounce_count: 0,
    no_retreat: 0,
    resistance: 0,
    no_rest_prep: 0,
    times_twitch: 0,
    escapes: 0,
    trying_unknown: false,
    dont_react: false,
    targeting: false,
    goal: C2RustUnnamed_13 {
        type_0: 0,
        g: loc { x: 0, y: 0 },
        rising: false,
        leaving: false,
        fleeing: false,
        fleeing_lunal: false,
        fleeing_munchkin: false,
        fleeing_to_town: false,
        ignoring: false,
        less: false,
        waiting: false,
        recalling: 0,
        descending: 0,
        shop: 0,
        ware: 0,
        item: 0,
        do_best: false,
        best_item: ::core::ptr::null::<borg_best>() as *mut borg_best,
    },
    init_save: C2RustUnnamed_12 {
        key_mode: 0,
        kinfo_ignore: ::core::ptr::null::<uint8_t>() as *mut uint8_t,
        ignore_level: [0; 27],
        ego_ignore_types: ::core::ptr::null::<*mut bool>() as *mut *mut bool,
    },
    mon: C2RustUnnamed_11 {
        breeders: 0,
        scary: false,
        morgoth_panel: loc { x: 0, y: 0 },
        unique: 0,
    },
    id: C2RustUnnamed_10 {
        morgoth: 0,
        sauron: 0,
        tarrasque: 0,
        maggot: 0,
        bullroarer: 0,
        beorn: 0,
        grip: 0,
        fang: 0,
        osse: 0,
        squint_eyed_rogue: 0,
        small_kobold: 0,
        soldier: 0,
        cutpurse: 0,
        acolyte: 0,
        apprentice: 0,
        kobold: 0,
        shrieker_mushroom_patch: 0,
        filthy_street_urchin: 0,
        battle_scarred_veteran: 0,
        mean_looking_mercenary: 0,
        giant_white_mouse: 0,
        white_worm_mass: 0,
        green_worm_mass: 0,
        cave_spider: 0,
        red_naga: 0,
        giant_red_frog: 0,
        radiation_eye: 0,
        yellow_worm_mass: 0,
        gravity_hound: 0,
        inertia_hound: 0,
        dread: 0,
        dracolich: 0,
        dracolisk: 0,
        light_hound: 0,
        dark_hound: 0,
        pukelman: 0,
        night_mare: 0,
        drolem: 0,
    },
    near: C2RustUnnamed_9 {
        morgoth: false,
        breeder: false,
        unique: 0,
        evil_unique: false,
        summoner: false,
        summoner_idx: 0,
    },
    morgoth_position: false,
    amt_book: [0; 9],
    book_idx: [0; 9],
    need_statgain: [false; 5],
    amt_statgain: [0; 5],
};
#[no_mangle]
pub static mut borg_cfg: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_rand_local: uint32_t = 0 as uint32_t;
#[no_mangle]
pub static mut borg_engine_date: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"Nov  4 2026\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut borg_step: uint16_t = 0 as uint16_t;
#[no_mangle]
pub static mut w_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut w_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_entry_point(mut flush_first: ::core::ffi::c_int) -> keypress {
    return borg_save_keypress(generate_keypress(flush_first));
}
#[no_mangle]
pub unsafe extern "C" fn borg_update_entrypoint(mut start: bool) {
    if start {
        inkey_hack = Some(borg_entry_point as unsafe extern "C" fn(::core::ffi::c_int) -> keypress)
            as Option<unsafe extern "C" fn(::core::ffi::c_int) -> keypress>;
    } else {
        inkey_hack = None;
    };
}
unsafe extern "C" fn check_for_deactivate_or_death(mut key: *mut keypress) -> bool {
    if !borg.status.active {
        borg_note(b"# Removing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
        inkey_hack = None;
        borg_flush();
        flush(
            EVENT_MAP,
            ::core::ptr::null_mut::<game_event_data>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        borg_reset_settings();
        (*key).code = ESCAPE as keycode_t;
        return true_0 != 0;
    }
    if (*player).is_dead as ::core::ffi::c_int != 0 && borg.status.respawning == 0 {
        if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            >= *borg_cfg.offset(BORG_DUMP_LEVEL as ::core::ffi::c_int as isize)
            || !strstr(
                &raw mut (*player).died_from as *mut ::core::ffi::c_char,
                b"starvation\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            borg_write_map(false_0 != 0);
        }
        borg_log_death();
        borg_log_death_data();
        borg_flush();
        borg_parse(::core::ptr::null_mut::<::core::ffi::c_char>());
        borg_clear_reactions();
        borg_oops(b"player died\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) != 0 {
        if *borg_cfg.offset(BORG_STOP_KING as ::core::ffi::c_int as isize) != 0 {
            borg_write_map(false_0 != 0);
            borg_oops(b"retire\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if *borg_cfg.offset(BORG_RESPAWN_WINNERS as ::core::ffi::c_int as isize) != 0 {
            borg_write_map(false_0 != 0);
            borg_log_death();
            borg_log_death_data();
            borg_flush();
            borg_force_reincarnate();
            return false_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == *borg_cfg.offset(BORG_STOP_DLEVEL as ::core::ffi::c_int as isize)
    {
        borg_oops(b"Auto-stop for user DLevel.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        == *borg_cfg.offset(BORG_STOP_CLEVEL as ::core::ffi::c_int as isize)
    {
        borg_oops(b"Auto-stop for user CLevel.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        > *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg_cfg.offset(BORG_SELF_SCUM as ::core::ffi::c_int as isize) == 0
    {
        borg_oops(b"Money Scum complete.\0" as *const u8 as *const ::core::ffi::c_char);
        (*key).code = ('C' as i32 & 0x1f as ::core::ffi::c_int) as keycode_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn user_abort() -> bool {
    let mut ch_evt: ui_event = ui_event { type_0: EVT_NONE };
    Term_inkey(&raw mut ch_evt, false_0 != 0, true_0 != 0);
    if !borg.in_shop
        && (ch_evt.type_0 as ::core::ffi::c_uint
            & EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && ch_evt.key.code > 0 as keycode_t
            && ch_evt.key.code != 10 as keycode_t
            || ch_evt.type_0 as ::core::ffi::c_uint
                == EVT_DISCONNECT as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if ch_evt.type_0 as ::core::ffi::c_uint
            == EVT_DISCONNECT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            borg_oops(b"terminal disconnect abort\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            if ch_evt.key.code >= 32 as keycode_t && ch_evt.key.code <= 126 as keycode_t {
                borg_note(format(
                    b"# User key press <%lu><%c>\0" as *const u8 as *const ::core::ffi::c_char,
                    ch_evt.key.code as ::core::ffi::c_ulong,
                    ch_evt.key.code as ::core::ffi::c_char as ::core::ffi::c_int,
                ));
            } else {
                borg_note(format(
                    b"# User key press <%lu>\0" as *const u8 as *const ::core::ffi::c_char,
                    ch_evt.key.code as ::core::ffi::c_ulong,
                ));
            }
            borg_note(format(
                b"# Key type was <%d><%c>\0" as *const u8 as *const ::core::ffi::c_char,
                ch_evt.type_0 as ::core::ffi::c_uint,
                ch_evt.type_0 as ::core::ffi::c_uint,
            ));
            borg_oops(b"user abort\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn generate_keypress(mut flush_first: ::core::ffi::c_int) -> keypress {
    let mut borg_ch: keycode_t = 0;
    let mut key: keypress = keypress {
        type_0: EVT_KBRD,
        code: 0 as keycode_t,
        mods: 0 as uint8_t,
    };
    let mut cursor: loc = loc { x: 0, y: 0 };
    let mut rand_quick: bool = false;
    let mut rand_value: uint32_t = 0;
    Term_locate(&raw mut cursor.x, &raw mut cursor.y);
    Term_fresh();
    borg_notice_player();
    if check_for_deactivate_or_death(&raw mut key) {
        return key;
    }
    if flush_first != 0 {
        if borg_inkey(false_0 != 0) != 0 as keycode_t {
            borg_note(b"# Flushing keypress buffer\0" as *const u8 as *const ::core::ffi::c_char);
            borg_flush();
        }
    }
    if borg_get_messages(&raw mut key, cursor) {
        return key;
    }
    borg_parse(::core::ptr::null_mut::<::core::ffi::c_char>());
    borg.dont_react = false_0 != 0;
    borg_ch = borg_inkey(true_0 != 0);
    if borg_ch != 0 {
        key.code = borg_ch;
        return key;
    }
    if user_abort() {
        key.code = ESCAPE as keycode_t;
        return key;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 1 as ::core::ffi::c_int
    {
        borg.in_shop = false_0 != 0;
    }
    if (*(*player).upkeep).resting != 0 || cmd_get_nrepeats() > 0 as ::core::ffi::c_int {
        key.type_0 = EVT_NONE;
        return key;
    }
    if borg.status.respawning != 0 {
        if !(*player).is_dead {
            borg_reincarnate_end();
            borg.status.respawning = false_0 as int16_t;
        } else {
            key.code = ESCAPE as keycode_t;
            return key;
        }
    }
    borg.targeting = false_0 != 0;
    rand_quick = Rand_quick;
    rand_value = Rand_value;
    Rand_quick = true_0 != 0;
    Rand_value = borg_rand_local;
    while !borg_think() {}
    borg_display_status();
    borg_rand_local = Rand_value;
    Rand_quick = rand_quick;
    Rand_value = rand_value;
    if borg_step as ::core::ffi::c_int != 0 && {
        borg_step = borg_step.wrapping_sub(1);
        borg_step == 0
    } {
        borg.status.cancel = true_0 != 0;
    }
    borg_ch = borg_inkey(true_0 != 0);
    if borg_ch != 0 {
        key.code = borg_ch;
        return key;
    }
    borg_oops(b"normal abort\0" as *const u8 as *const ::core::ffi::c_char);
    key.code = ESCAPE as keycode_t;
    return key;
}
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
