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
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn wcstombs(__s: *mut ::core::ffi::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn text_mbstowcs(
        dest: *mut wchar_t,
        src: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> size_t;
    fn event_add_handler(
        type_0: game_event_type,
        fn_0: Option<game_event_handler>,
        user: *mut ::core::ffi::c_void,
    );
    fn event_remove_handler(
        type_0: game_event_type,
        fn_0: Option<game_event_handler>,
        user: *mut ::core::ffi::c_void,
    );
    fn event_signal_message(
        type_0: game_event_type,
        t: ::core::ffi::c_int,
        s: *const ::core::ffi::c_char,
    );
    fn message_add(str: *const ::core::ffi::c_char, type_0: uint16_t);
    fn msg(fmt: *const ::core::ffi::c_char, ...);
    static mut Term: *mut term;
    static mut angband_term: [*mut term; 8];
    static mut window_flag: [uint32_t; 8];
    fn Term_fresh() -> errr;
    fn Term_addstr(
        n: ::core::ffi::c_int,
        a: ::core::ffi::c_int,
        s: *const ::core::ffi::c_char,
    ) -> errr;
    fn Term_erase(x: ::core::ffi::c_int, y: ::core::ffi::c_int, n: ::core::ffi::c_int) -> errr;
    fn Term_get_size(w: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int) -> errr;
    fn Term_locate(x: *mut ::core::ffi::c_int, y: *mut ::core::ffi::c_int) -> errr;
    fn Term_activate(t: *mut term) -> errr;
    fn borg_oops(what: *const ::core::ffi::c_char);
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub message: C2RustUnnamed_7,
    pub birthstage: C2RustUnnamed_6,
    pub birthpoints: C2RustUnnamed_5,
    pub explosion: C2RustUnnamed_4,
    pub bolt: C2RustUnnamed_3,
    pub missile: C2RustUnnamed_2,
    pub size: C2RustUnnamed_1,
    pub tunnel: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub nstep: ::core::ffi::c_int,
    pub npierce: ::core::ffi::c_int,
    pub ndug: ::core::ffi::c_int,
    pub dstart: ::core::ffi::c_int,
    pub dend: ::core::ffi::c_int,
    pub early: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub h: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub points: *const ::core::ffi::c_int,
    pub inc_points: *const ::core::ffi::c_int,
    pub remaining: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub msg: *const ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
}
pub type game_event_handler =
    unsafe extern "C" fn(game_event_type, *mut game_event_data, *mut ::core::ffi::c_void) -> ();
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const SOUND_MAX: C2RustUnnamed_8 = 153;
pub const MSG_MAX: C2RustUnnamed_8 = 153;
pub const MSG_SCRAMBLE: C2RustUnnamed_8 = 152;
pub const MSG_MULTIPLY: C2RustUnnamed_8 = 151;
pub const MSG_DRAIN_STAT: C2RustUnnamed_8 = 150;
pub const MSG_KILL_KING: C2RustUnnamed_8 = 149;
pub const MSG_KILL_UNIQUE: C2RustUnnamed_8 = 148;
pub const MSG_PRAYER: C2RustUnnamed_8 = 147;
pub const MSG_SPELL: C2RustUnnamed_8 = 146;
pub const MSG_HIT_HI_SUPERB: C2RustUnnamed_8 = 145;
pub const MSG_HIT_HI_GREAT: C2RustUnnamed_8 = 144;
pub const MSG_HIT_SUPERB: C2RustUnnamed_8 = 143;
pub const MSG_HIT_GREAT: C2RustUnnamed_8 = 142;
pub const MSG_HIT_GOOD: C2RustUnnamed_8 = 141;
pub const MSG_CAST_FEAR: C2RustUnnamed_8 = 140;
pub const MSG_SHRIEK: C2RustUnnamed_8 = 139;
pub const MSG_CREATE_TRAP: C2RustUnnamed_8 = 138;
pub const MSG_AMBIENT_DNG5: C2RustUnnamed_8 = 137;
pub const MSG_AMBIENT_DNG4: C2RustUnnamed_8 = 136;
pub const MSG_AMBIENT_DNG3: C2RustUnnamed_8 = 135;
pub const MSG_AMBIENT_DNG2: C2RustUnnamed_8 = 134;
pub const MSG_AMBIENT_DNG1: C2RustUnnamed_8 = 133;
pub const MSG_AMBIENT_NITE: C2RustUnnamed_8 = 132;
pub const MSG_AMBIENT_DAY: C2RustUnnamed_8 = 131;
pub const MSG_NOTICE: C2RustUnnamed_8 = 130;
pub const MSG_HUNGRY: C2RustUnnamed_8 = 129;
pub const MSG_RUNE: C2RustUnnamed_8 = 128;
pub const MSG_CURSED: C2RustUnnamed_8 = 127;
pub const MSG_QUIVER: C2RustUnnamed_8 = 126;
pub const MSG_WIELD: C2RustUnnamed_8 = 125;
pub const MSG_SUM_UNIQUE: C2RustUnnamed_8 = 124;
pub const MSG_SUM_WRAITH: C2RustUnnamed_8 = 123;
pub const MSG_SUM_HI_DEMON: C2RustUnnamed_8 = 122;
pub const MSG_SUM_HI_DRAGON: C2RustUnnamed_8 = 121;
pub const MSG_SUM_HI_UNDEAD: C2RustUnnamed_8 = 120;
pub const MSG_SUM_DRAGON: C2RustUnnamed_8 = 119;
pub const MSG_SUM_DEMON: C2RustUnnamed_8 = 118;
pub const MSG_SUM_HYDRA: C2RustUnnamed_8 = 117;
pub const MSG_SUM_HOUND: C2RustUnnamed_8 = 116;
pub const MSG_SUM_SPIDER: C2RustUnnamed_8 = 115;
pub const MSG_SUM_ANIMAL: C2RustUnnamed_8 = 114;
pub const MSG_SUM_UNDEAD: C2RustUnnamed_8 = 113;
pub const MSG_SUM_AINU: C2RustUnnamed_8 = 112;
pub const MSG_SUM_MONSTER: C2RustUnnamed_8 = 111;
pub const MSG_BR_FORCE: C2RustUnnamed_8 = 110;
pub const MSG_BR_PLASMA: C2RustUnnamed_8 = 109;
pub const MSG_BR_GRAVITY: C2RustUnnamed_8 = 108;
pub const MSG_BR_INERTIA: C2RustUnnamed_8 = 107;
pub const MSG_BR_TIME: C2RustUnnamed_8 = 106;
pub const MSG_BR_NEXUS: C2RustUnnamed_8 = 105;
pub const MSG_BR_NETHER: C2RustUnnamed_8 = 104;
pub const MSG_BR_DARK: C2RustUnnamed_8 = 103;
pub const MSG_BR_LIGHT: C2RustUnnamed_8 = 102;
pub const MSG_BR_SOUND: C2RustUnnamed_8 = 101;
pub const MSG_BR_SHARDS: C2RustUnnamed_8 = 100;
pub const MSG_BR_CHAOS: C2RustUnnamed_8 = 99;
pub const MSG_BR_DISEN: C2RustUnnamed_8 = 98;
pub const MSG_BR_FIRE: C2RustUnnamed_8 = 97;
pub const MSG_BR_GAS: C2RustUnnamed_8 = 96;
pub const MSG_BR_ACID: C2RustUnnamed_8 = 95;
pub const MSG_BR_ELEC: C2RustUnnamed_8 = 94;
pub const MSG_BR_FROST: C2RustUnnamed_8 = 93;
pub const MSG_BR_ELEMENTS: C2RustUnnamed_8 = 92;
pub const MSG_IDENT_ART: C2RustUnnamed_8 = 91;
pub const MSG_IDENT_EGO: C2RustUnnamed_8 = 90;
pub const MSG_IDENT_BAD: C2RustUnnamed_8 = 89;
pub const MSG_DISARM: C2RustUnnamed_8 = 88;
pub const MSG_LOCKPICK: C2RustUnnamed_8 = 87;
pub const MSG_STORE5: C2RustUnnamed_8 = 86;
pub const MSG_SHOOT_HIT: C2RustUnnamed_8 = 85;
pub const MSG_MONEY3: C2RustUnnamed_8 = 84;
pub const MSG_MONEY2: C2RustUnnamed_8 = 83;
pub const MSG_MONEY1: C2RustUnnamed_8 = 82;
pub const MSG_STORE_HOME: C2RustUnnamed_8 = 81;
pub const MSG_STORE_LEAVE: C2RustUnnamed_8 = 80;
pub const MSG_STORE_ENTER: C2RustUnnamed_8 = 79;
pub const MSG_STAIRS_UP: C2RustUnnamed_8 = 78;
pub const MSG_CUT: C2RustUnnamed_8 = 77;
pub const MSG_STUN: C2RustUnnamed_8 = 76;
pub const MSG_RES_POIS: C2RustUnnamed_8 = 75;
pub const MSG_RES_COLD: C2RustUnnamed_8 = 74;
pub const MSG_RES_FIRE: C2RustUnnamed_8 = 73;
pub const MSG_RES_ELEC: C2RustUnnamed_8 = 72;
pub const MSG_RES_ACID: C2RustUnnamed_8 = 71;
pub const MSG_INFRARED: C2RustUnnamed_8 = 70;
pub const MSG_SEE_INVIS: C2RustUnnamed_8 = 69;
pub const MSG_INVULN: C2RustUnnamed_8 = 68;
pub const MSG_PROT_EVIL: C2RustUnnamed_8 = 67;
pub const MSG_BOLD: C2RustUnnamed_8 = 66;
pub const MSG_BERSERK: C2RustUnnamed_8 = 65;
pub const MSG_HERO: C2RustUnnamed_8 = 64;
pub const MSG_BLESSED: C2RustUnnamed_8 = 63;
pub const MSG_SHIELD: C2RustUnnamed_8 = 62;
pub const MSG_SLOW: C2RustUnnamed_8 = 61;
pub const MSG_SPEED: C2RustUnnamed_8 = 60;
pub const MSG_DRUGGED: C2RustUnnamed_8 = 59;
pub const MSG_PARALYZED: C2RustUnnamed_8 = 58;
pub const MSG_AFRAID: C2RustUnnamed_8 = 57;
pub const MSG_POISONED: C2RustUnnamed_8 = 56;
pub const MSG_CONFUSED: C2RustUnnamed_8 = 55;
pub const MSG_BLIND: C2RustUnnamed_8 = 54;
pub const MSG_RECOVER: C2RustUnnamed_8 = 53;
pub const MSG_MON_MOAN: C2RustUnnamed_8 = 52;
pub const MSG_MON_INSULT: C2RustUnnamed_8 = 51;
pub const MSG_MON_BEG: C2RustUnnamed_8 = 50;
pub const MSG_MON_SPORE: C2RustUnnamed_8 = 49;
pub const MSG_MON_WAIL: C2RustUnnamed_8 = 48;
pub const MSG_MON_GAZE: C2RustUnnamed_8 = 47;
pub const MSG_MON_SPIT: C2RustUnnamed_8 = 46;
pub const MSG_MON_DROOL: C2RustUnnamed_8 = 45;
pub const MSG_MON_CRAWL: C2RustUnnamed_8 = 44;
pub const MSG_MON_ENGULF: C2RustUnnamed_8 = 43;
pub const MSG_MON_CRUSH: C2RustUnnamed_8 = 42;
pub const MSG_MON_BUTT: C2RustUnnamed_8 = 41;
pub const MSG_MON_STING: C2RustUnnamed_8 = 40;
pub const MSG_MON_BITE: C2RustUnnamed_8 = 39;
pub const MSG_MON_CLAW: C2RustUnnamed_8 = 38;
pub const MSG_MON_KICK: C2RustUnnamed_8 = 37;
pub const MSG_MON_PUNCH: C2RustUnnamed_8 = 36;
pub const MSG_MON_TOUCH: C2RustUnnamed_8 = 35;
pub const MSG_MON_HIT: C2RustUnnamed_8 = 34;
pub const MSG_DESTROY: C2RustUnnamed_8 = 33;
pub const MSG_USE_STAFF: C2RustUnnamed_8 = 32;
pub const MSG_ACT_ARTIFACT: C2RustUnnamed_8 = 31;
pub const MSG_HITPOINT_WARN: C2RustUnnamed_8 = 30;
pub const MSG_STAIRS_DOWN: C2RustUnnamed_8 = 29;
pub const MSG_LOCKPICK_FAIL: C2RustUnnamed_8 = 28;
pub const MSG_NOTHING_TO_OPEN: C2RustUnnamed_8 = 27;
pub const MSG_BELL: C2RustUnnamed_8 = 26;
pub const MSG_TPLEVEL: C2RustUnnamed_8 = 25;
pub const MSG_SHUTDOOR: C2RustUnnamed_8 = 24;
pub const MSG_OPENDOOR: C2RustUnnamed_8 = 23;
pub const MSG_DIG: C2RustUnnamed_8 = 22;
pub const MSG_STORE4: C2RustUnnamed_8 = 21;
pub const MSG_STORE3: C2RustUnnamed_8 = 20;
pub const MSG_STORE2: C2RustUnnamed_8 = 19;
pub const MSG_STORE1: C2RustUnnamed_8 = 18;
pub const MSG_EAT: C2RustUnnamed_8 = 17;
pub const MSG_HITWALL: C2RustUnnamed_8 = 16;
pub const MSG_TPOTHER: C2RustUnnamed_8 = 15;
pub const MSG_WALK: C2RustUnnamed_8 = 14;
pub const MSG_ZAP_ROD: C2RustUnnamed_8 = 13;
pub const MSG_QUAFF: C2RustUnnamed_8 = 12;
pub const MSG_SHOOT: C2RustUnnamed_8 = 11;
pub const MSG_TELEPORT: C2RustUnnamed_8 = 10;
pub const MSG_STUDY: C2RustUnnamed_8 = 9;
pub const MSG_DEATH: C2RustUnnamed_8 = 8;
pub const MSG_LEVEL: C2RustUnnamed_8 = 7;
pub const MSG_KILL: C2RustUnnamed_8 = 6;
pub const MSG_DROP: C2RustUnnamed_8 = 5;
pub const MSG_FLEE: C2RustUnnamed_8 = 4;
pub const MSG_MISS: C2RustUnnamed_8 = 3;
pub const MSG_HIT: C2RustUnnamed_8 = 2;
pub const MSG_BIRTH: C2RustUnnamed_8 = 1;
pub const MSG_GENERIC: C2RustUnnamed_8 = 0;
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
pub type keycode_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypress {
    pub type_0: ui_event_type,
    pub code: keycode_t,
    pub mods: uint8_t,
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
pub union ui_event {
    pub type_0: ui_event_type,
    pub mouse: mouseclick,
    pub key: keypress,
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
pub struct borg_best {
    pub home: bool,
    pub tval: uint8_t,
    pub sval: uint8_t,
    pub pval: int16_t,
}
pub type borg_time = int32_t;
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
pub const BORG_VERBOSE: C2RustUnnamed_17 = 0;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_17 = 31;
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_17 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_17 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_17 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_17 = 32;
pub const BORG_SAVE_DEATH: C2RustUnnamed_17 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_17 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_17 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_17 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_17 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_17 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_17 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_17 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_17 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_17 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_17 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_17 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_17 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_17 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_17 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_17 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_17 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_17 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_17 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_17 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_17 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_17 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_17 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_17 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_17 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_17 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_17 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_17 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_17 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_17 = 1;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_confirm_target: bool = false;
#[no_mangle]
pub unsafe extern "C" fn borg_what_text(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut a: *mut uint8_t,
    mut s: *mut ::core::ffi::c_char,
) -> errr {
    let mut i: ::core::ffi::c_int = 0;
    let mut screen_str: [wchar_t; 1024] = [0; 1024];
    let mut t_a: ::core::ffi::c_int = 0;
    let mut t_c: wchar_t = 0;
    let mut aa: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut cc: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut d_a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m: ::core::ffi::c_int = if n < 0 as ::core::ffi::c_int { -n } else { n };
    Term_get_size(&raw mut w, &raw mut h);
    if x + m > w {
        m = w - x;
    }
    aa = (*(*(*Term).scr).a.offset(y as isize)).offset(x as isize) as *mut ::core::ffi::c_int;
    cc = (*(*(*Term).scr).c.offset(y as isize)).offset(x as isize) as *mut wchar_t;
    i = 0 as ::core::ffi::c_int;
    while i < m {
        let fresh0 = aa;
        aa = aa.offset(1);
        t_a = *fresh0;
        let fresh1 = cc;
        cc = cc.offset(1);
        t_c = *fresh1;
        if t_c == ' ' as i32 || t_a == 0 {
            screen_str[i as usize] = ' ' as i32 as wchar_t;
        } else {
            if d_a != 0 {
                if t_a != d_a {
                    break;
                }
            } else {
                d_a = t_a;
            }
            screen_str[i as usize] = t_c;
        }
        i += 1;
    }
    screen_str[i as usize] = '\0' as i32 as wchar_t;
    *a = d_a as uint8_t;
    wcstombs(
        s,
        &raw mut screen_str as *mut wchar_t,
        ((if n < 0 as ::core::ffi::c_int { -n } else { n }) + 1 as ::core::ffi::c_int) as size_t,
    );
    if n > 0 as ::core::ffi::c_int && i != n {
        return 1 as errr;
    }
    return 0 as errr;
}
unsafe extern "C" fn borg_info(mut what: *const ::core::ffi::c_char) {}
unsafe extern "C" fn borg_note_internal(mut warning: bool, mut what: *const ::core::ffi::c_char) {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut old: *mut term = Term;
    if warning {
        msg(b"%s\0" as *const u8 as *const ::core::ffi::c_char, what);
    } else {
        message_add(what, MSG_GENERIC as ::core::ffi::c_int as uint16_t);
        event_signal_message(
            EVENT_MESSAGE,
            MSG_GENERIC as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    borg_info(what);
    if !strstr(
        what,
        b"Best Combo\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            what,
            b"Taking off \0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        borg.antibounce_count =
            (borg.antibounce_count as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as int16_t;
        borg_note(format(
            b"# Anti-loop variable tick (%d).\0" as *const u8 as *const ::core::ffi::c_char,
            borg.antibounce_count as ::core::ffi::c_int,
        ));
    }
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        if !angband_term[j as usize].is_null() {
            if !(window_flag[j as usize] as ::core::ffi::c_long & PW_BORG_1 == 0) {
                Term_activate(angband_term[j as usize]);
                Term_get_size(&raw mut w, &raw mut h);
                Term_locate(&raw mut x, &raw mut y);
                Term_erase(0 as ::core::ffi::c_int, y, w);
                n = strlen(what) as ::core::ffi::c_int;
                if n > w - 2 as ::core::ffi::c_int {
                    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
                    while n > w - 2 as ::core::ffi::c_int {
                        k = w - 2 as ::core::ffi::c_int;
                        i = w / 2 as ::core::ffi::c_int;
                        while i < w - 2 as ::core::ffi::c_int {
                            if *(*__ctype_b_loc())
                                .offset(*what.offset(i as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    as isize) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0
                            {
                                k = i;
                            }
                            i += 1;
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < k {
                            buf[i as usize] = *what.offset(i as isize);
                            i += 1;
                        }
                        let fresh2 = i;
                        i = i + 1;
                        buf[fresh2 as usize] = '\\' as i32 as ::core::ffi::c_char;
                        buf[i as usize] = '\0' as i32 as ::core::ffi::c_char;
                        Term_addstr(
                            -(1 as ::core::ffi::c_int),
                            COLOUR_WHITE,
                            &raw mut buf as *mut ::core::ffi::c_char,
                        );
                        y += 1;
                        if y >= h {
                            y = 0 as ::core::ffi::c_int;
                        }
                        Term_erase(0 as ::core::ffi::c_int, y, w);
                        what = what.offset(k as isize);
                        n -= k;
                    }
                    Term_addstr(-(1 as ::core::ffi::c_int), COLOUR_WHITE, what);
                    y += 1;
                    if y >= h {
                        y = 0 as ::core::ffi::c_int;
                    }
                    Term_erase(0 as ::core::ffi::c_int, y, w);
                } else {
                    Term_addstr(-(1 as ::core::ffi::c_int), COLOUR_WHITE, what);
                    y += 1;
                    if y >= h {
                        y = 0 as ::core::ffi::c_int;
                    }
                    Term_erase(0 as ::core::ffi::c_int, y, w);
                }
                Term_fresh();
                Term_activate(old);
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_warning(mut what: *const ::core::ffi::c_char) {
    borg_note_internal(true_0 != 0, what);
}
#[no_mangle]
pub unsafe extern "C" fn borg_note(mut what: *const ::core::ffi::c_char) {
    borg_note_internal(false_0 != 0, what);
}
static mut borg_key_queue: *mut keycode_t = ::core::ptr::null::<keycode_t>() as *mut keycode_t;
static mut borg_key_head: int16_t = 0;
static mut borg_key_tail: int16_t = 0;
static mut borg_key_history: *mut keypress = ::core::ptr::null::<keypress>() as *mut keypress;
static mut borg_key_history_head: int16_t = 0;
static mut borg_queued_direction: keycode_t = 0 as keycode_t;
#[no_mangle]
pub unsafe extern "C" fn borg_keypress(mut k: keycode_t) -> errr {
    if k == 0 {
        borg_note(b" & Key * *BAD KEY * *\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as errr);
    }
    let fresh3 = borg_key_head;
    borg_key_head = borg_key_head + 1;
    *borg_key_queue.offset(fresh3 as isize) = k;
    if borg_key_head as ::core::ffi::c_int == KEY_SIZE {
        borg_key_head = 0 as int16_t;
    }
    if borg_key_head as ::core::ffi::c_int == borg_key_tail as ::core::ffi::c_int {
        borg_oops(b"overflow\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if borg_key_tail as ::core::ffi::c_int == KEY_SIZE {
        borg_key_tail = 0 as int16_t;
    }
    return 0 as errr;
}
#[no_mangle]
pub unsafe extern "C" fn borg_save_keypress(mut kp: keypress) -> keypress {
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        if kp.type_0 as ::core::ffi::c_uint == EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: keycode_t = kp.code;
            if k >= 32 as keycode_t && k <= 126 as keycode_t {
                borg_note(format(
                    b"& Key <%c> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_char as ::core::ffi::c_int,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == KC_ENTER as keycode_t {
                borg_note(format(
                    b"& Key <Enter> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == ESCAPE as keycode_t {
                borg_note(format(
                    b"& Key <Esc> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else {
                borg_note(format(
                    b"& Key <0x%02lX>\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            }
        } else {
            borg_note(format(
                b"& non-Keyboard <0x%02X>\0" as *const u8 as *const ::core::ffi::c_char,
                kp.type_0 as ::core::ffi::c_uint,
            ));
        }
    }
    (*borg_key_history.offset(borg_key_history_head as isize)).code = kp.code;
    let fresh4 = borg_key_history_head;
    borg_key_history_head = borg_key_history_head + 1;
    (*borg_key_history.offset(fresh4 as isize)).type_0 = kp.type_0;
    if borg_key_history_head as ::core::ffi::c_int == KEY_SIZE {
        memcpy(
            borg_key_history as *mut ::core::ffi::c_void,
            borg_key_history.offset((KEY_SIZE - 101 as ::core::ffi::c_int) as isize)
                as *mut keypress as *const ::core::ffi::c_void,
            (::core::mem::size_of::<keypress>() as size_t).wrapping_mul(100 as size_t),
        );
        borg_key_history_head = 100 as int16_t;
    }
    return kp;
}
#[no_mangle]
pub unsafe extern "C" fn borg_keypresses(mut str: *const ::core::ffi::c_char) -> errr {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = str;
    while *s != 0 {
        borg_keypress(*s as keycode_t);
        s = s.offset(1);
    }
    return 0 as errr;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inkey(mut take: bool) -> keycode_t {
    let mut i: ::core::ffi::c_int = 0;
    if borg_key_head as ::core::ffi::c_int == borg_key_tail as ::core::ffi::c_int {
        return 0 as keycode_t;
    }
    i = *borg_key_queue.offset(borg_key_tail as isize) as ::core::ffi::c_int;
    if !take {
        return i as keycode_t;
    }
    borg_key_tail += 1;
    if borg_key_tail as ::core::ffi::c_int == KEY_SIZE {
        borg_key_tail = 0 as int16_t;
    }
    return i as keycode_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flush() {
    borg_key_tail = borg_key_head;
    borg_queued_direction = 0 as keycode_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_queue_direction(mut k: keycode_t) {
    borg_queued_direction = k;
    borg_confirm_target = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_queued_direction() -> keycode_t {
    let mut k: keycode_t = borg_queued_direction;
    borg_queued_direction = 0 as keycode_t;
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn borg_massage_special_chars(
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut wide_name: [wchar_t; 1024] = [0; 1024];
    let mut memory: *mut ::core::ffi::c_char = mem_zalloc(
        strlen(name)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    text_mbstowcs(
        &raw mut wide_name as *mut wchar_t,
        name,
        strlen(name).wrapping_add(1 as size_t) as ::core::ffi::c_int,
    );
    wcstombs(
        memory,
        &raw mut wide_name as *mut wchar_t,
        strlen(name).wrapping_add(1 as size_t),
    );
    return memory;
}
#[no_mangle]
pub unsafe extern "C" fn borg_dump_recent_keys(mut num: ::core::ffi::c_int) {
    let mut end: ::core::ffi::c_int = borg_key_history_head as ::core::ffi::c_int;
    let mut start: ::core::ffi::c_int = if (borg_key_history_head as ::core::ffi::c_int) < num {
        0 as ::core::ffi::c_int
    } else {
        borg_key_history_head as ::core::ffi::c_int - num
    };
    while start < end {
        let mut kp: *mut keypress = borg_key_history.offset(start as isize) as *mut keypress;
        if (*kp).type_0 as ::core::ffi::c_uint
            == EVT_KBRD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: keycode_t = (*kp).code;
            if k >= 32 as keycode_t && k <= 126 as keycode_t {
                borg_note(format(
                    b"& Key history <%c> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_char as ::core::ffi::c_int,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == KC_ENTER as keycode_t {
                borg_note(format(
                    b"& Key history <Enter> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else if k == ESCAPE as keycode_t {
                borg_note(format(
                    b"& Key history <Esc> (0x%02lX)\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            } else {
                borg_note(format(
                    b"& Key history <0x%02lX>\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_ulong,
                ));
            }
        } else {
            borg_note(format(
                b"& non-Keyboard <0x%02X>\0" as *const u8 as *const ::core::ffi::c_char,
                (*kp).type_0 as ::core::ffi::c_uint,
            ));
        }
        start += 1;
    }
}
unsafe extern "C" fn borg_bell(
    mut unused: game_event_type,
    mut data: *mut game_event_data,
    mut user: *mut ::core::ffi::c_void,
) {
    borg_note(
        b"** BELL SOUNDED Dumping keypress history ***\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    borg_dump_recent_keys(20 as ::core::ffi::c_int);
    if *borg_cfg.offset(BORG_STOP_ON_BELL as ::core::ffi::c_int as isize) != 0 {
        borg_oops(b"** BELL SOUNDED***\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_io() {
    borg_free_io();
    borg_key_queue = mem_zalloc(
        (KEY_SIZE as size_t).wrapping_mul(::core::mem::size_of::<keycode_t>() as size_t),
    ) as *mut keycode_t;
    borg_key_history =
        mem_zalloc((KEY_SIZE as size_t).wrapping_mul(::core::mem::size_of::<keypress>() as size_t))
            as *mut keypress;
    event_add_handler(
        EVENT_BELL,
        Some(
            borg_bell
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_io() {
    event_remove_handler(
        EVENT_BELL,
        Some(
            borg_bell
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    if !borg_key_history.is_null() {
        mem_free(borg_key_history as *mut ::core::ffi::c_void);
    }
    borg_key_history = ::core::ptr::null_mut::<keypress>();
    if !borg_key_queue.is_null() {
        mem_free(borg_key_queue as *mut ::core::ffi::c_void);
    }
    borg_key_queue = ::core::ptr::null_mut::<keycode_t>();
}
pub const PW_BORG_1: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const KEY_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
