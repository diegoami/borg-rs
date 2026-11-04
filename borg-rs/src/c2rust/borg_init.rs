#[repr(C)]
pub struct dice_s {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ang_file {
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
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn mem_alloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn prefix_i(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn Rand_div(m: uint32_t) -> uint32_t;
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
    fn event_signal(_: game_event_type);
    fn msg(fmt: *const ::core::ffi::c_char, ...);
    fn path_build(
        buf: *mut ::core::ffi::c_char,
        len: size_t,
        base: *const ::core::ffi::c_char,
        leaf: *const ::core::ffi::c_char,
    ) -> size_t;
    fn file_open(
        buf: *const ::core::ffi::c_char,
        mode: file_mode,
        ftype: file_type,
    ) -> *mut ang_file;
    fn file_close(f: *mut ang_file) -> bool;
    fn file_getl(f: *mut ang_file, buf: *mut ::core::ffi::c_char, n: size_t) -> bool;
    fn file_putf(f: *mut ang_file, fmt: *const ::core::ffi::c_char, ...) -> bool;
    static mut k_info: *mut object_kind;
    static mut screensaver: bool;
    fn option_set(opt: *const ::core::ffi::c_char, val: ::core::ffi::c_int) -> bool;
    static mut player: *mut player;
    fn player_id2class(id: guid) -> *mut player_class;
    static mut tile_width: uint8_t;
    static mut tile_height: uint8_t;
    fn Term_fresh() -> errr;
    fn do_cmd_redraw();
    fn borg_init_cave();
    fn borg_free_cave();
    static mut z_info: *mut angband_constants;
    static mut ANGBAND_DIR_USER: *mut ::core::ffi::c_char;
    fn borg_init_item();
    fn borg_free_item();
    fn borg_trait_init();
    fn borg_trait_free();
    static mut borg_rand_local: uint32_t;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    fn borg_init_flow();
    fn borg_free_flow();
    fn borg_init_flow_kill();
    fn borg_free_flow_kill();
    fn borg_init_flow_take();
    fn borg_free_flow_take();
    fn borg_load_formulas(fp: *mut ang_file) -> bool;
    fn borg_free_formulas();
    fn borg_init_item_activation();
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_warning(what: *const ::core::ffi::c_char);
    fn borg_init_io();
    fn borg_free_io();
    fn borg_init_item_val();
    fn borg_init_item_wear();
    fn borg_free_item_wear();
    fn borg_prepare_book_info();
    fn borg_init_messages();
    fn borg_free_messages();
    fn borg_init_store();
    fn borg_free_store();
    fn borg_free_detection();
    fn borg_init_update();
    fn borg_free_update();
    static mut ego_ignore_types: *mut *mut bool;
    fn ignore_birth_init();
    static mut ignore_level: [uint8_t; 0];
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
pub type guid = ::core::ffi::c_uint;
pub type file_mode = ::core::ffi::c_uint;
pub const MODE_APPEND: file_mode = 2;
pub const MODE_READ: file_mode = 1;
pub const MODE_WRITE: file_mode = 0;
pub type file_type = ::core::ffi::c_uint;
pub const FTYPE_HTML: file_type = 4;
pub const FTYPE_RAW: file_type = 3;
pub const FTYPE_SAVE: file_type = 2;
pub const FTYPE_TEXT: file_type = 1;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const OPT_MAX: C2RustUnnamed_8 = 46;
pub const OPT_birth_percent_damage: C2RustUnnamed_8 = 45;
pub const OPT_birth_levels_persist: C2RustUnnamed_8 = 44;
pub const OPT_birth_know_flavors: C2RustUnnamed_8 = 43;
pub const OPT_birth_know_runes: C2RustUnnamed_8 = 42;
pub const OPT_birth_ai_learn: C2RustUnnamed_8 = 41;
pub const OPT_birth_start_kit: C2RustUnnamed_8 = 40;
pub const OPT_birth_no_selling: C2RustUnnamed_8 = 39;
pub const OPT_birth_feelings: C2RustUnnamed_8 = 38;
pub const OPT_birth_lose_arts: C2RustUnnamed_8 = 37;
pub const OPT_birth_stacking: C2RustUnnamed_8 = 36;
pub const OPT_birth_no_artifacts: C2RustUnnamed_8 = 35;
pub const OPT_birth_no_recall: C2RustUnnamed_8 = 34;
pub const OPT_birth_force_descend: C2RustUnnamed_8 = 33;
pub const OPT_birth_connect_stairs: C2RustUnnamed_8 = 32;
pub const OPT_birth_randarts: C2RustUnnamed_8 = 31;
pub const OPT_score_live: C2RustUnnamed_8 = 30;
pub const OPT_cheat_live: C2RustUnnamed_8 = 29;
pub const OPT_score_xtra: C2RustUnnamed_8 = 28;
pub const OPT_cheat_xtra: C2RustUnnamed_8 = 27;
pub const OPT_score_room: C2RustUnnamed_8 = 26;
pub const OPT_cheat_room: C2RustUnnamed_8 = 25;
pub const OPT_score_hear: C2RustUnnamed_8 = 24;
pub const OPT_cheat_hear: C2RustUnnamed_8 = 23;
pub const OPT_effective_speed: C2RustUnnamed_8 = 22;
pub const OPT_notify_recharge: C2RustUnnamed_8 = 21;
pub const OPT_mouse_movement: C2RustUnnamed_8 = 20;
pub const OPT_hp_changes_color: C2RustUnnamed_8 = 19;
pub const OPT_auto_more: C2RustUnnamed_8 = 18;
pub const OPT_purple_uniques: C2RustUnnamed_8 = 17;
pub const OPT_center_player: C2RustUnnamed_8 = 16;
pub const OPT_animate_flicker: C2RustUnnamed_8 = 15;
pub const OPT_view_yellow_light: C2RustUnnamed_8 = 14;
pub const OPT_hybrid_walls: C2RustUnnamed_8 = 13;
pub const OPT_solid_walls: C2RustUnnamed_8 = 12;
pub const OPT_disturb_near: C2RustUnnamed_8 = 11;
pub const OPT_highlight_player: C2RustUnnamed_8 = 10;
pub const OPT_show_target: C2RustUnnamed_8 = 9;
pub const OPT_show_flavors: C2RustUnnamed_8 = 8;
pub const OPT_pickup_inven: C2RustUnnamed_8 = 7;
pub const OPT_pickup_always: C2RustUnnamed_8 = 6;
pub const OPT_use_old_target: C2RustUnnamed_8 = 5;
pub const OPT_show_damage: C2RustUnnamed_8 = 4;
pub const OPT_use_sound: C2RustUnnamed_8 = 3;
pub const OPT_autoexplore_commands: C2RustUnnamed_8 = 2;
pub const OPT_rogue_like_commands: C2RustUnnamed_8 = 1;
pub const OPT_none: C2RustUnnamed_8 = 0;
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
pub struct borg_setting {
    pub setting_string: *const ::core::ffi::c_char,
    pub setting_type: ::core::ffi::c_char,
    pub default_value: ::core::ffi::c_int,
}
pub const BORG_STOP_KING: C2RustUnnamed_19 = 17;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_19 = 19;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_19 = 20;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_19 = 21;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_19 = 28;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_19 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_19 = 2;
pub const BORG_LUNAL_MODE: C2RustUnnamed_19 = 26;
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
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_19 = 13;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_19 = 35;
pub const ITYPE_MAX: C2RustUnnamed_17 = 27;
pub const ITYPE_NONE: C2RustUnnamed_17 = 0;
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
pub const KEYMAP_MODE_ORIG: C2RustUnnamed_18 = 0;
pub const KEYMAP_MODE_ROGUE: C2RustUnnamed_18 = 1;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_19 = 34;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_19 = 32;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_19 = 18;
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const ITYPE_LIGHT: C2RustUnnamed_17 = 26;
pub const ITYPE_AMULET: C2RustUnnamed_17 = 25;
pub const ITYPE_RING: C2RustUnnamed_17 = 24;
pub const ITYPE_DIGGER: C2RustUnnamed_17 = 23;
pub const ITYPE_FEET: C2RustUnnamed_17 = 22;
pub const ITYPE_HANDGEAR: C2RustUnnamed_17 = 21;
pub const ITYPE_HEADGEAR: C2RustUnnamed_17 = 20;
pub const ITYPE_SHIELD: C2RustUnnamed_17 = 19;
pub const ITYPE_ELVEN_CLOAK: C2RustUnnamed_17 = 18;
pub const ITYPE_CLOAK: C2RustUnnamed_17 = 17;
pub const ITYPE_POWER_DRAGON_ARMOR: C2RustUnnamed_17 = 16;
pub const ITYPE_BALANCE_DRAGON_ARMOR: C2RustUnnamed_17 = 15;
pub const ITYPE_HIGH_DRAGON_ARMOR: C2RustUnnamed_17 = 14;
pub const ITYPE_MULTI_DRAGON_ARMOR: C2RustUnnamed_17 = 13;
pub const ITYPE_BASIC_DRAGON_ARMOR: C2RustUnnamed_17 = 12;
pub const ITYPE_BODY_ARMOR: C2RustUnnamed_17 = 11;
pub const ITYPE_ROBE: C2RustUnnamed_17 = 10;
pub const ITYPE_BOLT: C2RustUnnamed_17 = 9;
pub const ITYPE_ARROW: C2RustUnnamed_17 = 8;
pub const ITYPE_SHOT: C2RustUnnamed_17 = 7;
pub const ITYPE_CROSSBOW: C2RustUnnamed_17 = 6;
pub const ITYPE_BOW: C2RustUnnamed_17 = 5;
pub const ITYPE_SLING: C2RustUnnamed_17 = 4;
pub const ITYPE_GREAT: C2RustUnnamed_17 = 3;
pub const ITYPE_BLUNT: C2RustUnnamed_17 = 2;
pub const ITYPE_SHARP: C2RustUnnamed_17 = 1;
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const KEYMAP_MODE_MAX: C2RustUnnamed_18 = 2;
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const BORG_AUTOSAVE: C2RustUnnamed_19 = 33;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_19 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_19 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_19 = 29;
pub const BORG_SELF_LUNAL: C2RustUnnamed_19 = 27;
pub const BORG_SELF_SCUM: C2RustUnnamed_19 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_19 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_19 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_19 = 22;
pub const BORG_NO_DEEPER: C2RustUnnamed_19 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_19 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_19 = 14;
pub const BORG_USES_SWAPS: C2RustUnnamed_19 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_19 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_19 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_19 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_19 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_19 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_19 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_19 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_19 = 4;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_19 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_19 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut borg_initialized: bool = false;
#[no_mangle]
pub static mut game_closed: bool = false;
#[no_mangle]
pub static mut borg_init_failure: bool = false_0 != 0;
#[no_mangle]
pub static mut borg_settings: [borg_setting; 36] = [
    borg_setting {
        setting_string: b"borg_verbose\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_munchkin_start\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_munchkin_level\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 12 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_munchkin_depth\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 16 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_worships_damage\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_speed\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_hp\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_mana\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_ac\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_worships_gold\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_plays_risky\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_kills_uniques\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_uses_swaps\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_uses_dynamic_calcs\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_stop_dlevel\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 128 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_clevel\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 51 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_no_deeper\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 127 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_king\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_cheat_death\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_respawn_winners\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_respawn_class\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: -(1 as ::core::ffi::c_int),
    },
    borg_setting {
        setting_string: b"borg_respawn_race\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: -(1 as ::core::ffi::c_int),
    },
    borg_setting {
        setting_string: b"borg_chest_fail_tolerance\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 7 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_delay_factor\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_money_scum_amount\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_self_scum\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: true_0,
    },
    borg_setting {
        setting_string: b"borg_lunal_mode\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_self_lunal\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_enchant_limit\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 12 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_dump_level\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 1 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_save_death\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'i' as i32 as ::core::ffi::c_char,
        default_value: 1 as ::core::ffi::c_int,
    },
    borg_setting {
        setting_string: b"borg_stop_on_bell\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_allow_strange_opts\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_autosave\0" as *const u8 as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: b"borg_restore_ignore_settings\0" as *const u8
            as *const ::core::ffi::c_char,
        setting_type: 'b' as i32 as ::core::ffi::c_char,
        default_value: false_0,
    },
    borg_setting {
        setting_string: ::core::ptr::null::<::core::ffi::c_char>(),
        setting_type: 0 as ::core::ffi::c_char,
        default_value: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn borg_proc_setting(
    mut setting: ::core::ffi::c_int,
    mut setting_string: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
) -> bool {
    let mut negative: bool = false_0 != 0;
    while *line as ::core::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    if !prefix_i(line, setting_string) {
        return false_0 != 0;
    }
    line = line.offset(strlen(setting_string) as isize);
    while *line != 0 {
        let mut ch: ::core::ffi::c_char = *line;
        if ch as ::core::ffi::c_int == ' ' as i32 || ch as ::core::ffi::c_int == '=' as i32 {
            line = line.offset(1);
        } else if type_0 as ::core::ffi::c_int == 'b' as i32 {
            if ch as ::core::ffi::c_int == 'T' as i32
                || ch as ::core::ffi::c_int == 't' as i32
                || ch as ::core::ffi::c_int == '1' as i32
                || ch as ::core::ffi::c_int == 'Y' as i32
                || ch as ::core::ffi::c_int == 'y' as i32
            {
                *borg_cfg.offset(setting as isize) = true_0;
            } else {
                *borg_cfg.offset(setting as isize) = false_0;
            }
            break;
        } else {
            if !(type_0 as ::core::ffi::c_int == 'i' as i32) {
                continue;
            }
            if ch as ::core::ffi::c_int == '-' as i32 {
                negative = true_0 != 0;
                line = line.offset(1);
            } else {
                if !(*(*__ctype_b_loc())
                    .offset(ch as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0)
                {
                    continue;
                }
                *borg_cfg.offset(setting as isize) = atoi(line);
                break;
            }
        }
    }
    if negative {
        *borg_cfg.offset(setting as isize) *= -(1 as ::core::ffi::c_int);
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_free_txt_file() {
    borg_trait_free();
    borg_free_formulas();
    if !borg_cfg.is_null() {
        mem_free(borg_cfg as *mut ::core::ffi::c_void);
    }
    borg_cfg = ::core::ptr::null_mut::<::core::ffi::c_int>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_txt_file() -> bool {
    let mut fp: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut formulas_found: bool = false_0 != 0;
    let mut old_formulas_found: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut warning: bool = false_0 != 0;
    borg_free_txt_file();
    borg_trait_init();
    if strcmp(
        borg_settings[BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as usize].setting_string,
        b"borg_munchkin_level\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        || strcmp(
            borg_settings[BORG_RESPAWN_RACE as ::core::ffi::c_int as usize].setting_string,
            b"borg_respawn_race\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        msg(
            b"borg settings structures not correct.  aborting. \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_USER,
        b"borg.txt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fp = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_READ,
        4294967295 as file_type,
    );
    if fp.is_null() {
        msg(
            b"*****WARNING***** You do not have a proper BORG.TXT file!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg(
            b"writing borg.txt to the \\user\\ subdirectory with default values\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg(
            b"Which is: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        fp = file_open(
            &raw mut buf as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
        if fp.is_null() {
            borg_warning(
                b"*****WARNING***** unable to write default BORG.TXT file!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        file_putf(
            fp,
            b"# BORG.txt default settings \n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# A more descriptive version of this file is delivered. \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# Check your original install for borg.txt and \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        file_putf(
            fp,
            b"# replace this one with the delivered version. \n\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
            if borg_settings[i as usize].setting_type as ::core::ffi::c_int == 'b' as i32 {
                file_putf(
                    fp,
                    b"%s = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_settings[i as usize].setting_string,
                    if borg_settings[i as usize].default_value != 0 {
                        b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
            }
            if borg_settings[i as usize].setting_type as ::core::ffi::c_int == 'i' as i32 {
                file_putf(
                    fp,
                    b"%s = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_settings[i as usize].setting_string,
                    borg_settings[i as usize].default_value,
                );
            }
            i += 1;
        }
        file_close(fp);
        fp = file_open(
            &raw mut buf as *mut ::core::ffi::c_char,
            MODE_READ,
            4294967295 as file_type,
        );
    }
    borg_cfg = mem_alloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(BORG_MAX_SETTINGS as ::core::ffi::c_int as size_t),
    ) as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
        *borg_cfg.offset(i as isize) = borg_settings[i as usize].default_value;
        i += 1;
    }
    while file_getl(
        fp,
        &raw mut buf as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t).wrapping_sub(1 as size_t),
    ) {
        if buf[0 as ::core::ffi::c_int as usize] == 0
            || buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        buf[(::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize)
            .wrapping_sub(1 as usize) as usize] = '\0' as i32 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < BORG_MAX_SETTINGS as ::core::ffi::c_int {
            if borg_proc_setting(
                i,
                borg_settings[i as usize].setting_string,
                borg_settings[i as usize].setting_type,
                &raw mut buf as *mut ::core::ffi::c_char,
            ) {
                break;
            }
            i += 1;
        }
        if i != BORG_MAX_SETTINGS as ::core::ffi::c_int {
            continue;
        }
        if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"REQ\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"FORMULA\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"CND\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"POWER\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            old_formulas_found = true_0 != 0;
        } else if prefix_i(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"[BEGIN FORMULA SECTION]\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            formulas_found = true_0 != 0;
            borg_load_formulas(fp);
        }
    }
    file_close(fp);
    if *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) != 0 {
        warning = true_0 != 0;
        if old_formulas_found {
            borg_warning(
                b"** Borg's dynamic calculations enabled but old formulas found in borg.txt.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            borg_warning(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
            *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
        } else if !formulas_found {
            borg_warning(
                b"** Borg's dynamic calculations enabled but no formulas found in borg.txt.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            borg_warning(b"** formulas disabled ** \0" as *const u8 as *const ::core::ffi::c_char);
            *borg_cfg.offset(BORG_USES_DYNAMIC_CALCS as ::core::ffi::c_int as isize) = false_0;
        } else {
            borg_warning(
                b"Borg's dynamic calculations enabled.  You may see some performance loss (~20 percent).\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        Term_fresh();
    }
    borg.lunal_mode = *borg_cfg.offset(BORG_LUNAL_MODE as ::core::ffi::c_int as isize) != 0;
    if *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize)
        <= 1 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize) =
            1 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize)
        >= 50 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_LEVEL as ::core::ffi::c_int as isize) =
            50 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        <= 1 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize) =
            8 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        >= 100 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize) =
            100 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
        <= 8 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize) =
            8 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize)
        >= 15 as ::core::ffi::c_int
    {
        *borg_cfg.offset(BORG_ENCHANT_LIMIT as ::core::ffi::c_int as isize) =
            15 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize) >= MAX_RACES
        || *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
            < -(1 as ::core::ffi::c_int)
    {
        *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize) >= MAX_CLASSES
        || *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
            < -(1 as ::core::ffi::c_int)
    {
        *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_int;
    }
    if *borg_cfg.offset(BORG_RESPAWN_WINNERS as ::core::ffi::c_int as isize) != 0 {
        *borg_cfg.offset(BORG_STOP_KING as ::core::ffi::c_int as isize) = false_0;
    }
    return warning;
}
#[no_mangle]
pub unsafe extern "C" fn borg_reset_ignore() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if *borg_cfg.offset(BORG_RESTORE_IGNORE_SETTINGS as ::core::ffi::c_int as isize) != 0 {
        if !borg.init_save.kinfo_ignore.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).k_max as ::core::ffi::c_int {
                (*k_info.offset(i as isize)).ignore =
                    *borg.init_save.kinfo_ignore.offset(i as isize);
                i += 1;
            }
        }
        i = ITYPE_NONE as ::core::ffi::c_int;
        while i < ITYPE_MAX as ::core::ffi::c_int {
            *(&raw mut ignore_level as *mut uint8_t).offset(i as isize) =
                borg.init_save.ignore_level[i as usize];
            i += 1;
        }
        if !borg.init_save.ego_ignore_types.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*z_info).e_max as ::core::ffi::c_int {
                j = ITYPE_NONE as ::core::ffi::c_int;
                while j < ITYPE_MAX as ::core::ffi::c_int {
                    *(*ego_ignore_types.offset(i as isize)).offset(j as isize) =
                        *(*borg.init_save.ego_ignore_types.offset(i as isize)).offset(j as isize);
                    j += 1;
                }
                i += 1;
            }
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        mem_free(*borg.init_save.ego_ignore_types.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    mem_free(borg.init_save.ego_ignore_types as *mut ::core::ffi::c_void);
    borg.init_save.ego_ignore_types = ::core::ptr::null_mut::<*mut bool>();
    mem_free(borg.init_save.kinfo_ignore as *mut ::core::ffi::c_void);
    borg.init_save.kinfo_ignore = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn borg_init_ignore() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    borg.init_save.kinfo_ignore = mem_alloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*z_info).k_max as size_t),
    ) as *mut uint8_t;
    borg.init_save.ego_ignore_types = mem_zalloc(
        ((*z_info).e_max as size_t).wrapping_mul(::core::mem::size_of::<*mut bool>() as size_t),
    ) as *mut *mut bool;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        let ref mut fresh0 = *borg.init_save.ego_ignore_types.offset(i as isize);
        *fresh0 = mem_zalloc(
            (ITYPE_MAX as ::core::ffi::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        ) as *mut bool;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).k_max as ::core::ffi::c_int {
        *borg.init_save.kinfo_ignore.offset(i as isize) = (*k_info.offset(i as isize)).ignore;
        i += 1;
    }
    i = ITYPE_NONE as ::core::ffi::c_int;
    while i < ITYPE_MAX as ::core::ffi::c_int {
        borg.init_save.ignore_level[i as usize] =
            *(&raw mut ignore_level as *mut uint8_t).offset(i as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).e_max as ::core::ffi::c_int {
        j = ITYPE_NONE as ::core::ffi::c_int;
        while j < ITYPE_MAX as ::core::ffi::c_int {
            *(*borg.init_save.ego_ignore_types.offset(i as isize)).offset(j as isize) =
                *(*ego_ignore_types.offset(i as isize)).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    ignore_birth_init();
}
#[no_mangle]
pub unsafe extern "C" fn borg_reinit_options() {
    borg.init_save.key_mode = if (*player).opts.opt
        [OPT_rogue_like_commands as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        != 0
    {
        KEYMAP_MODE_ROGUE as ::core::ffi::c_int
    } else {
        KEYMAP_MODE_ORIG as ::core::ffi::c_int
    };
    borg_init_ignore();
    option_set(
        b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"auto_more\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"pickup_always\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    option_set(
        b"pickup_detail\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"use_old_target\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"pickup_inven\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    option_set(
        b"show_piles\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"show_labels\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"show_flavors\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"easy_open\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    option_set(
        b"easy_alter\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    );
    (*player).opts.hitpoint_warn = 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_reset_settings() {
    if borg.init_save.key_mode == KEYMAP_MODE_ROGUE as ::core::ffi::c_int {
        option_set(
            b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
            true_0,
        );
    } else if borg.init_save.key_mode == KEYMAP_MODE_ORIG as ::core::ffi::c_int {
        option_set(
            b"rogue_like_commands\0" as *const u8 as *const ::core::ffi::c_char,
            false_0,
        );
    }
    borg_reset_ignore();
    borg_free_detection();
}
unsafe extern "C" fn borg_leave_game(
    mut ev_type: game_event_type,
    mut ev_data: *mut game_event_data,
    mut user: *mut ::core::ffi::c_void,
) {
    game_closed = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prepare_race_class_info() {
    borg_prepare_book_info();
    game_closed = false_0 != 0;
    event_add_handler(
        EVENT_LEAVE_GAME,
        Some(
            borg_leave_game
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
pub unsafe extern "C" fn borg_init() {
    let mut memory_test: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut warning_given: bool = false;
    do_cmd_redraw();
    borg_note(b"Initializing the Borg... (memory)\0" as *const u8 as *const ::core::ffi::c_char);
    borg_init_failure = false_0 != 0;
    Term_fresh();
    memory_test = mem_zalloc(
        ((400 as ::core::ffi::c_long * 1024 as ::core::ffi::c_long) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    mem_free(memory_test as *mut ::core::ffi::c_void);
    memory_test = ::core::ptr::null_mut::<uint8_t>();
    if borg_rand_local == 0 {
        borg_rand_local = (Rand_div(0x10000000 as uint32_t) as int32_t + 1 as int32_t) as uint32_t;
    }
    borg.player = player;
    borg.status.old_depth = 128 as int16_t;
    borg_note(b"Initializing the Borg... (borg.txt)\0" as *const u8 as *const ::core::ffi::c_char);
    warning_given = borg_init_txt_file();
    borg_note(b"Initializing the Borg... (options)\0" as *const u8 as *const ::core::ffi::c_char);
    Term_fresh();
    if screensaver as ::core::ffi::c_int != 0
        || *borg_cfg.offset(BORG_CHEAT_DEATH as ::core::ffi::c_int as isize) != 0
    {
        option_set(
            b"cheat_live\0" as *const u8 as *const ::core::ffi::c_char,
            true_0,
        );
    }
    (*(*player).upkeep).redraw = ((*(*player).upkeep).redraw as ::core::ffi::c_long
        | 0x10000 as ::core::ffi::c_long) as uint32_t;
    borg_note(b"Initializing the Borg... (various)\0" as *const u8 as *const ::core::ffi::c_char);
    Term_fresh();
    borg.status.cheat_death = false_0 != 0;
    if (*player).opts.opt[OPT_cheat_live as ::core::ffi::c_int as usize] {
        borg.status.cheat_death = true_0 != 0;
    }
    borg_init_messages();
    borg_init_io();
    borg_init_item_val();
    borg_init_item_activation();
    borg_init_cave();
    borg_init_flow();
    borg_init_item_wear();
    borg_init_flow_take();
    borg_init_flow_kill();
    borg_init_item();
    borg_init_store();
    borg_init_update();
    borg_prepare_race_class_info();
    borg_note(b"Initializing the Borg... done.\0" as *const u8 as *const ::core::ffi::c_char);
    borg.time.now = 10 as ::core::ffi::c_int as borg_time;
    if strcmp(
        (*player_id2class(0 as guid)).name,
        b"Warrior\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        || strcmp(
            (*player_id2class(1 as guid)).name,
            b"Mage\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(2 as guid)).name,
            b"Druid\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(3 as guid)).name,
            b"Priest\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(4 as guid)).name,
            b"Necromancer\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(5 as guid)).name,
            b"Paladin\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(6 as guid)).name,
            b"Rogue\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(7 as guid)).name,
            b"Ranger\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || strcmp(
            (*player_id2class(8 as guid)).name,
            b"Blackguard\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        borg_note(
            b"**STARTUP FAILURE** classes do not match\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
    if *borg_cfg.offset(BORG_ALLOW_STRANGE_OPTS as ::core::ffi::c_int as isize) == 0 {
        if tile_width as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            || tile_height as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        {
            borg_note(
                b"**STARTUP FAILURE** Scaled tiles not allowed\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must allow up stairs\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if !(*player).opts.opt[OPT_birth_connect_stairs as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must connect stairs\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_no_recall as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** must allow recall\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_percent_damage as ::core::ffi::c_int as usize] {
            borg_note(
                b"**STARTUP FAILURE** strange damage calculation\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_note(b"** birth option failure **\0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** can be bypassed with borg_allow_strange_opts **\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_init_failure = true_0 != 0;
        }
    }
    if !borg_init_failure && !warning_given {
        borg_note(b"# Ready...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    borg_initialized = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free() {
    event_remove_handler(
        EVENT_LEAVE_GAME,
        Some(
            borg_leave_game
                as unsafe extern "C" fn(
                    game_event_type,
                    *mut game_event_data,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    borg_free_update();
    borg_free_item();
    borg_free_store();
    borg_free_flow_kill();
    borg_free_flow_take();
    borg_free_item_wear();
    borg_free_flow();
    borg_free_cave();
    borg_free_io();
    borg_free_messages();
    borg_free_txt_file();
}
pub const PR_MAP: ::core::ffi::c_long = 0x10000 as ::core::ffi::c_long;
pub const MAX_RACES: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MAX_CLASSES: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
