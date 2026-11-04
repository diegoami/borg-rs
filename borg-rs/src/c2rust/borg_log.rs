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
    fn wcstombs(__s: *mut ::core::ffi::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn wcslen(__s: *const wchar_t) -> ::core::ffi::c_ulong;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn quit_fmt(fmt: *const ::core::ffi::c_char, ...);
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn prefix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    fn messages_num() -> uint16_t;
    fn message_str(age: uint16_t) -> *const ::core::ffi::c_char;
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
    fn file_put(f: *mut ang_file, buf: *const ::core::ffi::c_char) -> bool;
    fn file_putf(f: *mut ang_file, fmt: *const ::core::ffi::c_char, ...) -> bool;
    static mut k_info: *mut object_kind;
    static mut player: *mut player;
    fn get_string(
        prompt: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> bool;
    static mut turn: int32_t;
    static mut z_info: *mut angband_constants;
    static mut ANGBAND_DIR_ARCHIVE: *mut ::core::ffi::c_char;
    fn object_desc(
        buf: *mut ::core::ffi::c_char,
        max: size_t,
        obj: *const object,
        mode: uint32_t,
        p: *const player,
    ) -> size_t;
    fn object_flags(obj: *const object, flags: *mut bitflag);
    fn player_of_has(p: *const player, flag: ::core::ffi::c_int) -> bool;
    static mut stores: *mut store;
    fn store_stock_list(store: *mut store, list: *mut *mut object, n: ::core::ffi::c_int);
    static mut Term: *mut term;
    static mut angband_term: [*mut term; 8];
    static mut window_flag: [uint32_t; 8];
    fn Term_fresh() -> errr;
    fn Term_putch(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        a: ::core::ffi::c_int,
        c: wchar_t,
    ) -> errr;
    fn Term_putstr(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        a: ::core::ffi::c_int,
        s: *const ::core::ffi::c_char,
    ) -> errr;
    fn Term_clear() -> errr;
    fn Term_activate(t: *mut term) -> errr;
    fn prt(str: *const ::core::ffi::c_char, row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    static mut borg_grids: [*mut borg_grid; 66];
    static mut borg_needs_new_sea: bool;
    static mut f_info: *mut feature;
    static mut cave: *mut chunk;
    fn square_monster(c: *mut chunk, grid: loc) -> *mut monster;
    static mut r_info: *mut monster_race;
    static mut borg_items: *mut borg_item;
    fn borg_get_note(item: *const borg_item) -> *const ::core::ffi::c_char;
    static mut weapon_swap: ::core::ffi::c_int;
    static mut armour_swap: ::core::ffi::c_int;
    static mut prefix_pref: [*const ::core::ffi::c_char; 0];
    static mut borg_engine_date: [::core::ffi::c_char; 0];
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    fn borg_race_name(i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static mut borg_takes: *mut borg_take;
    static mut num_mana: int16_t;
    static mut num_heal: int16_t;
    static mut num_ezheal: int16_t;
    static mut num_life: int16_t;
    static mut borg_magics: *mut borg_magic;
    fn borg_can_cast() -> bool;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    fn borg_spell_fail_rate(spell: borg_spells) -> ::core::ffi::c_int;
    fn borg_prepared(depth: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
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
pub type time_t = __time_t;
pub type size_t = usize;
pub type wchar_t = ::libc::wchar_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
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
pub type file_mode = ::core::ffi::c_uint;
pub const MODE_APPEND: file_mode = 2;
pub const MODE_READ: file_mode = 1;
pub const MODE_WRITE: file_mode = 0;
pub type file_type = ::core::ffi::c_uint;
pub const FTYPE_HTML: file_type = 4;
pub const FTYPE_RAW: file_type = 3;
pub const FTYPE_SAVE: file_type = 2;
pub const FTYPE_TEXT: file_type = 1;
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
pub const BORG_SAVE_DEATH: C2RustUnnamed_12 = 30;
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
    pub temp: C2RustUnnamed_8,
    pub status: C2RustUnnamed_7,
    pub time: C2RustUnnamed_6,
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
    pub init_save: C2RustUnnamed_4,
    pub mon: C2RustUnnamed_3,
    pub id: C2RustUnnamed_2,
    pub near: C2RustUnnamed_1,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
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
pub type borg_time = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
pub const BI_MAXDEPTH: C2RustUnnamed_11 = 106;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_12 = 24;
pub const BI_SCON: C2RustUnnamed_11 = 24;
pub const BI_SDEX: C2RustUnnamed_11 = 23;
pub const BI_SWIS: C2RustUnnamed_11 = 22;
pub const BI_SINT: C2RustUnnamed_11 = 21;
pub const BI_SSTR: C2RustUnnamed_11 = 20;
pub const BI_ISCONFUSED: C2RustUnnamed_11 = 114;
pub const BI_ISSTUN: C2RustUnnamed_11 = 117;
pub const BI_ISCUT: C2RustUnnamed_11 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_11 = 115;
pub const BI_ISWEAK: C2RustUnnamed_11 = 108;
pub const BI_FIRST_CURSED: C2RustUnnamed_11 = 159;
pub const BI_WB_FIRE: C2RustUnnamed_11 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_11 = 206;
pub const BI_WB_COLD: C2RustUnnamed_11 = 208;
pub const BI_WB_ACID: C2RustUnnamed_11 = 205;
pub const BI_WK_DRAGON: C2RustUnnamed_11 = 203;
pub const BI_WS_DRAGON: C2RustUnnamed_11 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_11 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_11 = 198;
pub const BI_WS_ORC: C2RustUnnamed_11 = 197;
pub const BI_WK_DEMON: C2RustUnnamed_11 = 202;
pub const BI_WS_DEMON: C2RustUnnamed_11 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_11 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_11 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_11 = 193;
pub const BI_HLIFE: C2RustUnnamed_11 = 85;
pub const BI_FRACT: C2RustUnnamed_11 = 86;
pub const BI_SINV: C2RustUnnamed_11 = 51;
pub const BI_ESP: C2RustUnnamed_11 = 37;
pub const BI_REG: C2RustUnnamed_11 = 50;
pub const BI_LIGHT: C2RustUnnamed_11 = 26;
pub const BI_FEATH: C2RustUnnamed_11 = 49;
pub const BI_SDIG: C2RustUnnamed_11 = 48;
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
pub const BI_ICOLD: C2RustUnnamed_11 = 66;
pub const BI_RCOLD: C2RustUnnamed_11 = 70;
pub const BI_IFIRE: C2RustUnnamed_11 = 64;
pub const BI_RFIRE: C2RustUnnamed_11 = 69;
pub const BI_IELEC: C2RustUnnamed_11 = 67;
pub const BI_RELEC: C2RustUnnamed_11 = 71;
pub const BI_IACID: C2RustUnnamed_11 = 65;
pub const BI_RACID: C2RustUnnamed_11 = 72;
pub const BI_MAX: C2RustUnnamed_11 = 265;
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
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_11 = 264;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_12 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_12 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_12 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_12 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_12 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_12 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_12 = 4;
pub const BORG_USES_SWAPS: C2RustUnnamed_12 = 12;
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
pub const TMD_OPP_POIS: C2RustUnnamed_10 = 23;
pub const TMD_OPP_COLD: C2RustUnnamed_10 = 22;
pub const TMD_OPP_ELEC: C2RustUnnamed_10 = 20;
pub const TMD_OPP_ACID: C2RustUnnamed_10 = 19;
pub const TMD_OPP_FIRE: C2RustUnnamed_10 = 21;
pub const TMD_CONFUSED: C2RustUnnamed_10 = 4;
pub const TMD_INVULN: C2RustUnnamed_10 = 12;
pub const TMD_SHIELD: C2RustUnnamed_10 = 15;
pub const TMD_PROTEVIL: C2RustUnnamed_10 = 11;
pub const TMD_SHERO: C2RustUnnamed_10 = 14;
pub const TMD_HERO: C2RustUnnamed_10 = 13;
pub const TMD_BLESSED: C2RustUnnamed_10 = 16;
pub const TMD_IMAGE: C2RustUnnamed_10 = 6;
pub const TMD_POISONED: C2RustUnnamed_10 = 7;
pub const TMD_STUN: C2RustUnnamed_10 = 9;
pub const TMD_CUT: C2RustUnnamed_10 = 8;
pub const TMD_AFRAID: C2RustUnnamed_10 = 5;
pub const TMD_BLIND: C2RustUnnamed_10 = 2;
pub const BI_SPEED: C2RustUnnamed_11 = 44;
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
pub const ODESC_FULL: C2RustUnnamed_9 = 3;
pub const ODESC_PREFIX: C2RustUnnamed_9 = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store {
    pub owners: *mut owner,
    pub owner: *mut owner,
    pub feat: ::core::ffi::c_int,
    pub stock_num: uint8_t,
    pub stock_size: int16_t,
    pub stock: *mut object,
    pub stock_k: *mut object,
    pub always_size: size_t,
    pub always_num: size_t,
    pub always_table: *mut *mut object_kind,
    pub normal_size: size_t,
    pub normal_num: size_t,
    pub normal_table: *mut *mut object_kind,
    pub buy: *mut object_buy,
    pub turnover: ::core::ffi::c_int,
    pub normal_stock_min: ::core::ffi::c_int,
    pub normal_stock_max: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_buy {
    pub next: *mut object_buy,
    pub tval: size_t,
    pub flag: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owner {
    pub oidx: ::core::ffi::c_uint,
    pub next: *mut owner,
    pub name: *mut ::core::ffi::c_char,
    pub max_cost: int32_t,
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
pub struct feature {
    pub name: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
    pub fidx: ::core::ffi::c_int,
    pub mimic: *mut feature,
    pub priority: uint8_t,
    pub shopnum: uint8_t,
    pub dig: uint8_t,
    pub flags: [bitflag; 4],
    pub d_attr: uint8_t,
    pub d_char: wchar_t,
    pub walk_msg: *mut ::core::ffi::c_char,
    pub run_msg: *mut ::core::ffi::c_char,
    pub hurt_msg: *mut ::core::ffi::c_char,
    pub die_msg: *mut ::core::ffi::c_char,
    pub confused_msg: *mut ::core::ffi::c_char,
    pub look_prefix: *mut ::core::ffi::c_char,
    pub look_in_preposition: *mut ::core::ffi::c_char,
    pub resist_flag: ::core::ffi::c_int,
}
pub const BI_DEV: C2RustUnnamed_11 = 56;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const ODESC_ALTNUM: C2RustUnnamed_9 = 1024;
pub const ODESC_NOEGO: C2RustUnnamed_9 = 512;
pub const ODESC_TERSE: C2RustUnnamed_9 = 256;
pub const ODESC_CAPITAL: C2RustUnnamed_9 = 128;
pub const ODESC_SPOIL: C2RustUnnamed_9 = 32;
pub const ODESC_SINGULAR: C2RustUnnamed_9 = 16;
pub const ODESC_PLURAL: C2RustUnnamed_9 = 8;
pub const ODESC_STORE: C2RustUnnamed_9 = 4;
pub const ODESC_EXTRA: C2RustUnnamed_9 = 2;
pub const ODESC_COMBAT: C2RustUnnamed_9 = 1;
pub const ODESC_BASE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const TMD_MAX: C2RustUnnamed_10 = 53;
pub const TMD_FREE_ACT: C2RustUnnamed_10 = 52;
pub const TMD_STEALTH: C2RustUnnamed_10 = 51;
pub const TMD_BLACKBREATH: C2RustUnnamed_10 = 50;
pub const TMD_BLOODLUST: C2RustUnnamed_10 = 49;
pub const TMD_TAUNT: C2RustUnnamed_10 = 48;
pub const TMD_POWERSHOT: C2RustUnnamed_10 = 47;
pub const TMD_COVERTRACKS: C2RustUnnamed_10 = 46;
pub const TMD_ATT_RUN: C2RustUnnamed_10 = 45;
pub const TMD_COMMAND: C2RustUnnamed_10 = 44;
pub const TMD_HEAL: C2RustUnnamed_10 = 43;
pub const TMD_ATT_VAMP: C2RustUnnamed_10 = 42;
pub const TMD_ATT_DEMON: C2RustUnnamed_10 = 41;
pub const TMD_ATT_EVIL: C2RustUnnamed_10 = 40;
pub const TMD_ATT_CONF: C2RustUnnamed_10 = 39;
pub const TMD_ATT_POIS: C2RustUnnamed_10 = 38;
pub const TMD_ATT_COLD: C2RustUnnamed_10 = 37;
pub const TMD_ATT_FIRE: C2RustUnnamed_10 = 36;
pub const TMD_ATT_ELEC: C2RustUnnamed_10 = 35;
pub const TMD_ATT_ACID: C2RustUnnamed_10 = 34;
pub const TMD_FASTCAST: C2RustUnnamed_10 = 33;
pub const TMD_TRAPSAFE: C2RustUnnamed_10 = 32;
pub const TMD_SCRAMBLE: C2RustUnnamed_10 = 31;
pub const TMD_BOLD: C2RustUnnamed_10 = 30;
pub const TMD_SPRINT: C2RustUnnamed_10 = 29;
pub const TMD_TERROR: C2RustUnnamed_10 = 28;
pub const TMD_STONESKIN: C2RustUnnamed_10 = 27;
pub const TMD_TELEPATHY: C2RustUnnamed_10 = 26;
pub const TMD_AMNESIA: C2RustUnnamed_10 = 25;
pub const TMD_OPP_CONF: C2RustUnnamed_10 = 24;
pub const TMD_SINFRA: C2RustUnnamed_10 = 18;
pub const TMD_SINVIS: C2RustUnnamed_10 = 17;
pub const TMD_FOOD: C2RustUnnamed_10 = 10;
pub const TMD_PARALYZED: C2RustUnnamed_10 = 3;
pub const TMD_SLOW: C2RustUnnamed_10 = 1;
pub const TMD_FAST: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
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
pub const BI_AGLYPH: C2RustUnnamed_11 = 230;
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
pub const BI_W_IMPACT: C2RustUnnamed_11 = 204;
pub const BI_WK_UNDEAD: C2RustUnnamed_11 = 201;
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
pub const BI_QUIVER_SLOTS: C2RustUnnamed_11 = 158;
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
pub const BI_ISAFRAID: C2RustUnnamed_11 = 113;
pub const BI_ISBLIND: C2RustUnnamed_11 = 112;
pub const BI_ISGORGED: C2RustUnnamed_11 = 111;
pub const BI_ISFULL: C2RustUnnamed_11 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_11 = 109;
pub const BI_KING: C2RustUnnamed_11 = 107;
pub const BI_CDEPTH: C2RustUnnamed_11 = 105;
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
pub const BI_IPOIS: C2RustUnnamed_11 = 68;
pub const BI_DIG: C2RustUnnamed_11 = 63;
pub const BI_THT: C2RustUnnamed_11 = 62;
pub const BI_THB: C2RustUnnamed_11 = 61;
pub const BI_THN: C2RustUnnamed_11 = 60;
pub const BI_SRCH: C2RustUnnamed_11 = 59;
pub const BI_STL: C2RustUnnamed_11 = 58;
pub const BI_SAV: C2RustUnnamed_11 = 57;
pub const BI_DISM: C2RustUnnamed_11 = 55;
pub const BI_DISP: C2RustUnnamed_11 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_11 = 53;
pub const BI_INFRA: C2RustUnnamed_11 = 52;
pub const BI_DAM_RED: C2RustUnnamed_11 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_11 = 46;
pub const BI_GOLD: C2RustUnnamed_11 = 45;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_11 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_11 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_11 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_11 = 40;
pub const BI_FOOD: C2RustUnnamed_11 = 39;
pub const BI_RECALL: C2RustUnnamed_11 = 38;
pub const BI_MAXCLEVEL: C2RustUnnamed_11 = 36;
pub const BI_CLEVEL: C2RustUnnamed_11 = 35;
pub const BI_FAIL2: C2RustUnnamed_11 = 34;
pub const BI_FAIL1: C2RustUnnamed_11 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_11 = 32;
pub const BI_MAXSP: C2RustUnnamed_11 = 31;
pub const BI_CURSP: C2RustUnnamed_11 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_11 = 29;
pub const BI_MAXHP: C2RustUnnamed_11 = 28;
pub const BI_CURHP: C2RustUnnamed_11 = 27;
pub const BI_CLASS: C2RustUnnamed_11 = 25;
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
pub const BORG_DUMP_LEVEL: C2RustUnnamed_12 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_12 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_12 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_12 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_12 = 25;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_12 = 23;
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
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_12 = 11;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_12 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_12 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_12 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_12 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLOUR_SLATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COLOUR_GREEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLOUR_BLUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn borg_log_death() {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut borg_log_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut death_time: time_t = 0;
    if *borg_cfg.offset(BORG_SAVE_DEATH as ::core::ffi::c_int as isize) == 0 {
        return;
    }
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        b"borg-log.txt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg_log_file = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_APPEND,
        FTYPE_TEXT,
    );
    if borg_log_file.is_null() {
        return;
    }
    time(&raw mut death_time);
    strftime(
        &raw mut buf as *mut ::core::ffi::c_char,
        80 as size_t,
        b"%Y/%m/%d %H:%M\n\0" as *const u8 as *const ::core::ffi::c_char,
        localtime(&raw mut death_time),
    );
    file_put(borg_log_file, &raw mut buf as *mut ::core::ffi::c_char);
    file_putf(
        borg_log_file,
        b"%s the %s %s, Level %d/%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*player).full_name as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).max_lev as ::core::ffi::c_int,
    );
    file_putf(
        borg_log_file,
        b"Exp: %lu  Gold: %lu  Turn: %lu\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).max_exp as ::core::ffi::c_long
            + (100 as ::core::ffi::c_int * (*player).max_depth as ::core::ffi::c_int)
                as ::core::ffi::c_long,
        (*player).au as ::core::ffi::c_long,
        turn as ::core::ffi::c_long,
    );
    file_putf(
        borg_log_file,
        b"Killed on level: %d (max. %d) by %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).depth as ::core::ffi::c_int,
        (*player).max_depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_putf(
        borg_log_file,
        b"Borg Compile Date: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_engine_date as *mut ::core::ffi::c_char,
    );
    file_put(
        borg_log_file,
        b"----------\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_close(borg_log_file);
}
#[no_mangle]
pub unsafe extern "C" fn borg_log_death_data() {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut borg_log_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut death_time: time_t = 0;
    if *borg_cfg.offset(BORG_SAVE_DEATH as ::core::ffi::c_int as isize) == 0 {
        return;
    }
    path_build(
        &raw mut buf as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        b"borg.dat\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg_log_file = file_open(
        &raw mut buf as *mut ::core::ffi::c_char,
        MODE_APPEND,
        FTYPE_TEXT,
    );
    if borg_log_file.is_null() {
        return;
    }
    time(&raw mut death_time);
    file_putf(
        borg_log_file,
        b"%s, %s, %s, %d, %d, %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_engine_date as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_close(borg_log_file);
}
unsafe extern "C" fn borg_index_to_label(mut i: ::core::ffi::c_int) -> ::core::ffi::c_char {
    if i < (*z_info).pack_size as ::core::ffi::c_int {
        return *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize);
    }
    return *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
        .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn borg_write_map(mut ask: bool) {
    let mut buf2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    let mut borg_map_file: *mut ang_file = ::core::ptr::null_mut::<ang_file>();
    let mut line: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut ch_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut itemm: ::core::ffi::c_int = 0;
    let mut m_idx: int16_t = 0;
    let mut o_name: [::core::ffi::c_char; 80] = [0; 80];
    if *borg_cfg.offset(BORG_SAVE_DEATH as ::core::ffi::c_int as isize) == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while (*player).full_name[i as usize] != 0 {
        let mut c: ::core::ffi::c_char = (*player).full_name[i as usize];
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _IScntrl as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            quit_fmt(
                b"Illegal control char (0x%02X) in player name\0" as *const u8
                    as *const ::core::ffi::c_char,
                c as ::core::ffi::c_int,
            );
        }
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
            && *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
        {
            c = '_' as i32 as ::core::ffi::c_char;
        }
        buf[i as usize] = c;
        i += 1;
    }
    let fresh0 = i;
    i = i + 1;
    buf[fresh0 as usize] = '.' as i32 as ::core::ffi::c_char;
    let fresh1 = i;
    i = i + 1;
    buf[fresh1 as usize] = 'm' as i32 as ::core::ffi::c_char;
    let fresh2 = i;
    i = i + 1;
    buf[fresh2 as usize] = 'a' as i32 as ::core::ffi::c_char;
    let fresh3 = i;
    i = i + 1;
    buf[fresh3 as usize] = 'p' as i32 as ::core::ffi::c_char;
    let fresh4 = i;
    i = i + 1;
    buf[fresh4 as usize] = '\0' as i32 as ::core::ffi::c_char;
    path_build(
        &raw mut buf2 as *mut ::core::ffi::c_char,
        1024 as size_t,
        ANGBAND_DIR_ARCHIVE,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    if ask as ::core::ffi::c_int != 0
        && get_string(
            b"Borg map File: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf2 as *mut ::core::ffi::c_char,
            70 as size_t,
        ) as ::core::ffi::c_int
            != 0
    {
        borg_map_file = file_open(
            &raw mut buf2 as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
        if borg_map_file.is_null() {
            msg(b"Cannot open that file.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    } else if !ask {
        borg_map_file = file_open(
            &raw mut buf2 as *mut ::core::ffi::c_char,
            MODE_WRITE,
            FTYPE_TEXT,
        );
    }
    file_putf(
        borg_map_file,
        b"%s the %s %s, Level %d/%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*player).full_name as *mut ::core::ffi::c_char,
        (*(*player).race).name,
        (*(*player).class).name,
        (*player).lev as ::core::ffi::c_int,
        (*player).max_lev as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"Exp: %lu  Gold: %lu  Turn: %lu\n\0" as *const u8 as *const ::core::ffi::c_char,
        ((*player).max_exp + 100 as int32_t * (*player).max_depth as int32_t)
            as ::core::ffi::c_long,
        (*player).au as ::core::ffi::c_long,
        turn as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Killed on level: %d (max. %d) by %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*player).depth as ::core::ffi::c_int,
        (*player).max_depth as ::core::ffi::c_int,
        &raw mut (*player).died_from as *mut ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Borg Compile Date: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_engine_date as *mut ::core::ffi::c_char,
    );
    line = mem_zalloc(
        ((DUNGEON_WID + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<wchar_t>() as size_t),
    ) as *mut wchar_t;
    ch_line = mem_zalloc(
        ((DUNGEON_WID + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < DUNGEON_HGT {
        j = 0 as ::core::ffi::c_int;
        while j < DUNGEON_WID {
            let mut ch: wchar_t = 0;
            let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(i as isize))
            .offset(j as isize) as *mut borg_grid;
            if !square_monster(cave, loc(j, i)).is_null() {
                m_idx = (*square_monster(cave, loc(j, i))).midx as int16_t;
            } else {
                m_idx = 0 as int16_t;
            }
            ch = ' ' as i32 as wchar_t;
            if (*ag).feat != 0 {
                ch = (*f_info.offset((*ag).feat as isize)).d_char;
            }
            if (*ag).take != 0 {
                let mut take: *mut borg_take =
                    borg_takes.offset((*ag).take as isize) as *mut borg_take;
                if (*take).kidx != 0 {
                    let mut k_ptr: *mut object_kind =
                        k_info.offset((*take).kidx as isize) as *mut object_kind;
                    ch = (*k_ptr).d_char;
                }
            }
            if m_idx != 0 {
                ch = '&' as i32 as wchar_t;
            }
            if (*ag).kill != 0 {
                let mut kill: *mut borg_kill =
                    borg_kills.offset((*ag).kill as isize) as *mut borg_kill;
                let mut r_ptr: *mut monster_race =
                    r_info.offset((*kill).r_idx as isize) as *mut monster_race;
                ch = (*r_ptr).d_char;
            }
            if i == borg.c.y && j == borg.c.x {
                ch = '@' as i32 as wchar_t;
            }
            *line.offset(j as isize) = ch;
            j += 1;
        }
        let fresh5 = j;
        j = j + 1;
        *line.offset(fresh5 as isize) = '\0' as i32 as wchar_t;
        wcstombs(
            ch_line,
            line,
            (wcslen(line) as size_t).wrapping_add(1 as size_t),
        );
        file_putf(
            borg_map_file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            ch_line,
        );
        i += 1;
    }
    mem_free(line as *mut ::core::ffi::c_void);
    line = ::core::ptr::null_mut::<wchar_t>();
    mem_free(ch_line as *mut ::core::ffi::c_void);
    ch_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill_0: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill_0).r_idx == 0) {
            file_putf(
                borg_map_file,
                b"monster '%s' (%d) at (%d,%d) speed:%d \n\0" as *const u8
                    as *const ::core::ffi::c_char,
                borg_race_name((*kill_0).r_idx as ::core::ffi::c_int),
                (*kill_0).r_idx as ::core::ffi::c_int,
                (*kill_0).pos.y,
                (*kill_0).pos.x,
                (*kill_0).speed as ::core::ffi::c_int,
            );
        }
        i += 1;
    }
    i = messages_num() as ::core::ffi::c_int;
    if i > 250 as ::core::ffi::c_int {
        i = 250 as ::core::ffi::c_int;
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Last Messages]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    loop {
        let fresh6 = i;
        i = i - 1;
        if !(fresh6 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut msg_0: *const ::core::ffi::c_char = message_str(i as int16_t as uint16_t);
        if prefix(
            msg_0,
            b"# Matched\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || prefix(
                msg_0,
                b"# There is\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# Tracking\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# MISS_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(
                msg_0,
                b"# HIT_BY:\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || prefix(msg_0, b"> \0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            continue;
        }
        file_putf(
            borg_map_file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            msg_0,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Character Equipment]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*player).body.count as ::core::ffi::c_int {
        let mut obj: *mut object = (*(*player).body.slots.offset(i as isize)).obj;
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            obj,
            ODESC_FULL as ::core::ffi::c_int as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            borg_index_to_label(i) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n  [Character Quiver]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).quiver_size as ::core::ffi::c_int {
        let mut obj_0: *mut object = *(*(*player).upkeep).quiver.offset(i as isize);
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            obj_0,
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            borg_index_to_label(i) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Character Inventory]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if (*item).iqty != 0 {
            if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                file_putf(
                    borg_map_file,
                    b"%c) %d %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_index_to_label(i) as ::core::ffi::c_int,
                    (*item).iqty as ::core::ffi::c_int,
                    &raw mut (*item).desc as *mut ::core::ffi::c_char,
                );
            } else {
                file_putf(
                    borg_map_file,
                    b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_index_to_label(i) as ::core::ffi::c_int,
                    &raw mut (*item).desc as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Home Inventory (page 1)]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut list: *mut *mut object = mem_zalloc(
        (::core::mem::size_of::<*mut object>() as size_t)
            .wrapping_mul((*z_info).store_inven_max as size_t),
    ) as *mut *mut object;
    store_stock_list(
        stores.offset(BORG_HOME as isize) as *mut store,
        list,
        (*z_info).store_inven_max as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            *list.offset(i as isize),
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i % 12 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"  [Home Inventory (page 2)]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = (*z_info).store_inven_max as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
    while i < (*z_info).store_inven_max as ::core::ffi::c_int {
        object_desc(
            &raw mut o_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            *list.offset(i as isize),
            (ODESC_PREFIX as ::core::ffi::c_int | ODESC_FULL as ::core::ffi::c_int) as uint32_t,
            player,
        );
        file_putf(
            borg_map_file,
            b"%c) %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i % 12 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            &raw mut o_name as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    mem_free(list as *mut ::core::ffi::c_void);
    list = ::core::ptr::null_mut::<*mut object>();
    if *borg_cfg.offset(BORG_USES_SWAPS as ::core::ffi::c_int as isize) != 0 {
        file_putf(
            borg_map_file,
            b"  [Swap info]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if weapon_swap != 0 {
            item = borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize)
                as *mut borg_item;
            file_putf(
                borg_map_file,
                b"Swap Weapon:  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*item).desc as *mut ::core::ffi::c_char,
            );
        } else {
            file_put(
                borg_map_file,
                b"Swap Weapon:  NONE\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if armour_swap != 0 {
            item = borg_items.offset((armour_swap - 1 as ::core::ffi::c_int) as isize)
                as *mut borg_item;
            file_putf(
                borg_map_file,
                b"Swap Armour:  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*item).desc as *mut ::core::ffi::c_char,
            );
        } else {
            file_put(
                borg_map_file,
                b"Swap Armour:  NONE\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        file_putf(
            borg_map_file,
            b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"  [Player State at Death] \n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Current speed: %d. \n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize),
    );
    if *(*player)
        .timed
        .offset(TMD_BLIND as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You cannot see.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are confused.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_AFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are terrified.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CUT as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are bleeding.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_STUN as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are stunned.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_POISONED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are poisoned.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_IMAGE as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are hallucinating.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if player_of_has(player, OF_AGGRAVATE as ::core::ffi::c_int) {
        file_putf(
            borg_map_file,
            b"You aggravate monsters.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_BLESSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You feel righteous.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_HERO as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You feel heroic.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_SHERO as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are in a battle rage.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_PROTEVIL as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are protected from evil.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_SHIELD as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are protected by a mystic shield.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_INVULN as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You are temporarily invulnerable.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_CONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"Your hands are glowing dull red.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*player).word_recall != 0 {
        file_putf(
            borg_map_file,
            b"You will soon be recalled.  (%d turns)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*player).word_recall as ::core::ffi::c_int,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_FIRE as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist fire exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist acid exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_ELEC as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist elec exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_COLD as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist cold exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(*player)
        .timed
        .offset(TMD_OPP_POIS as ::core::ffi::c_int as isize)
        != 0
    {
        file_putf(
            borg_map_file,
            b"You resist poison exceptionally well.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"Anti-bounce counter: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        borg.antibounce_count as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"Time on this level: %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
        (borg.time.now - borg.time.level) as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Time since left town: %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
        (borg.time.now - borg.time.town) as ::core::ffi::c_long,
    );
    file_putf(
        borg_map_file,
        b"Borg_no_retreat: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        borg.no_retreat as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"borg.near.breeder: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        borg.near.breeder as ::core::ffi::c_int,
    );
    file_putf(
        borg_map_file,
        b"borg.mon.unique: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        borg.mon.unique,
    );
    if turn as ::core::ffi::c_long
        % (10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long)
        < 10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long
            / 2 as ::core::ffi::c_long
    {
        file_putf(
            borg_map_file,
            b"It is daytime in town.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        file_putf(
            borg_map_file,
            b"It is night-time in town.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    file_putf(
        borg_map_file,
        b"borg_uses_swaps: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_USES_SWAPS as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_damage: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_DAMAGE as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_speed: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_SPEED as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_hp: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_HP as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_mana: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_MANA as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_ac: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_AC as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_worships_gold: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_WORSHIPS_GOLD as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"borg_plays_risky: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg_cfg.offset(BORG_PLAYS_RISKY as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"prepping for big fight: %d\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        *borg
            .trait_0
            .offset(BI_PREP_BIG_FIGHT as ::core::ffi::c_int as isize),
    );
    file_putf(
        borg_map_file,
        b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if borg_can_cast() {
        file_putf(
            borg_map_file,
            b"\n\n   [ Spells ] \n\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        file_putf(
            borg_map_file,
            b"Name                           Legal Times cast\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*(*player).class).magic.total_spells {
            let mut as_0: *mut borg_magic = borg_magics.offset(i as isize) as *mut borg_magic;
            let mut failpercent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if ((*as_0).level as ::core::ffi::c_int) < 99 as ::core::ffi::c_int {
                let mut legal: *const ::core::ffi::c_char =
                    if borg_spell_legal((*as_0).spell_enum) as ::core::ffi::c_int != 0 {
                        b"Yes\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"No \0" as *const u8 as *const ::core::ffi::c_char
                    };
                failpercent = borg_spell_fail_rate((*as_0).spell_enum);
                file_putf(
                    borg_map_file,
                    b"%-30s   %s   %ld   fail:%d \n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*as_0).name,
                    legal,
                    (*as_0).times as ::core::ffi::c_long,
                    failpercent,
                );
            }
            i += 1;
        }
        file_putf(
            borg_map_file,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    itemm = 0 as ::core::ffi::c_int;
    to = BI_MAX as ::core::ffi::c_int;
    while itemm < to {
        file_putf(
            borg_map_file,
            b"skill %d (%s) value= %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
            itemm,
            *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char).offset(itemm as isize),
            *borg.trait_0.offset(itemm as isize),
        );
        itemm += 1;
    }
    file_close(borg_map_file);
}
unsafe extern "C" fn borg_prt_binary(
    mut flags: uint32_t,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut bitmask: uint32_t = 0;
    bitmask = 1 as uint32_t;
    i = bitmask as ::core::ffi::c_int;
    while i <= 32 as ::core::ffi::c_int {
        if flags & bitmask != 0 {
            let fresh7 = col;
            col = col + 1;
            Term_putch(fresh7, row, COLOUR_BLUE, '*' as i32);
        } else {
            let fresh8 = col;
            col = col + 1;
            Term_putch(fresh8, row, COLOUR_WHITE, '-' as i32);
        }
        i += 1;
        bitmask = bitmask.wrapping_mul(2 as uint32_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_display_item(mut item2: *mut object, mut n: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: [bitflag; 5] = [0; 5];
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    item = borg_items.offset(n as isize) as *mut borg_item;
    object_flags(item2, &raw mut f as *mut bitflag);
    Term_clear();
    prt(
        &raw mut (*item).desc as *mut ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"kind = %-5lu  level = %-4d  tval = %-5d  sval = %-5d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).kind as ::core::ffi::c_ulong,
            (*item).level as ::core::ffi::c_int,
            (*item).tval as ::core::ffi::c_int,
            (*item).sval as ::core::ffi::c_int,
        ),
        4 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"number = %-3d  wgt = %-6d  ac = %-5d    damage = %dd%d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).iqty as ::core::ffi::c_int,
            (*item).weight as ::core::ffi::c_int,
            (*item).ac as ::core::ffi::c_int,
            (*item).dd as ::core::ffi::c_int,
            (*item).ds as ::core::ffi::c_int,
        ),
        5 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"pval = %-5d  toac = %-5d  tohit = %-4d  todam = %-4d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*item).pval as ::core::ffi::c_int,
            (*item).to_a as ::core::ffi::c_int,
            (*item).to_h as ::core::ffi::c_int,
            (*item).to_d as ::core::ffi::c_int,
        ),
        6 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"name1 = %-4d  name2 = %-4d  value = %ld   cursed = %d   can uncurse = %d\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*item).art_idx as ::core::ffi::c_int,
            (*item).ego_idx as ::core::ffi::c_int,
            (*item).value as ::core::ffi::c_long,
            (*item).cursed as ::core::ffi::c_int,
            (*item).uncursable as ::core::ffi::c_int,
        ),
        7 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"ident = %d      timeout = %-d\0" as *const u8 as *const ::core::ffi::c_char,
            (*item).ident as ::core::ffi::c_int,
            (*item).timeout as ::core::ffi::c_int,
        ),
        8 as ::core::ffi::c_int,
        j,
    );
    prt(
        format(
            b"Inscription: %s, chance: %d\0" as *const u8 as *const ::core::ffi::c_char,
            borg_get_note(item),
            *borg.trait_0.offset(BI_DEV as ::core::ffi::c_int as isize)
                - (*item).level as ::core::ffi::c_int,
        ),
        9 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"+------------FLAGS1------------+\0" as *const u8 as *const ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"AFFECT..........SLAY.......BRAND\0" as *const u8 as *const ::core::ffi::c_char,
        11 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"                ae      xxxpaefc\0" as *const u8 as *const ::core::ffi::c_char,
        12 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"siwdcc  ssidsasmnvudotgddduoclio\0" as *const u8 as *const ::core::ffi::c_char,
        13 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"tnieoh  trnipthgiinmrrnrrmniierl\0" as *const u8 as *const ::core::ffi::c_char,
        14 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"rtsxna..lcfgdkttmldncltggndsdced\0" as *const u8 as *const ::core::ffi::c_char,
        15 as ::core::ffi::c_int,
        j,
    );
    if (*item).ident {
        borg_prt_binary(
            f[0 as ::core::ffi::c_int as usize] as uint32_t,
            16 as ::core::ffi::c_int,
            j,
        );
    }
    prt(
        b"+------------FLAGS2------------+\0" as *const u8 as *const ::core::ffi::c_char,
        17 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"SUST........IMM.RESIST.........\0" as *const u8 as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"            afecaefcpfldbc s n  \0" as *const u8 as *const ::core::ffi::c_char,
        19 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"siwdcc      cilocliooeialoshnecd\0" as *const u8 as *const ::core::ffi::c_char,
        20 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"tnieoh      irelierliatrnnnrethi\0" as *const u8 as *const ::core::ffi::c_char,
        21 as ::core::ffi::c_int,
        j,
    );
    prt(
        b"rtsxna......decddcedsrekdfddxhss\0" as *const u8 as *const ::core::ffi::c_char,
        22 as ::core::ffi::c_int,
        j,
    );
    if (*item).ident {
        borg_prt_binary(
            f[1 as ::core::ffi::c_int as usize] as uint32_t,
            23 as ::core::ffi::c_int,
            j,
        );
    }
    prt(
        b"+------------FLAGS3------------+\0" as *const u8 as *const ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"s   ts h     tadiiii   aiehs  hp\0" as *const u8 as *const ::core::ffi::c_char,
        11 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"lf  eefo     egrgggg  bcnaih  vr\0" as *const u8 as *const ::core::ffi::c_char,
        12 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"we  lerln   ilgannnn  ltssdo  ym\0" as *const u8 as *const ::core::ffi::c_char,
        13 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"da reiedo   merirrrr  eityew ccc\0" as *const u8 as *const ::core::ffi::c_char,
        14 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"itlepnelf   ppanaefc  svaktm uuu\0" as *const u8 as *const ::core::ffi::c_char,
        15 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"ghigavaiu   aoveclio  saanyo rrr\0" as *const u8 as *const ::core::ffi::c_char,
        16 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"seteticfe   craxierl  etropd sss\0" as *const u8 as *const ::core::ffi::c_char,
        17 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    prt(
        b"trenhstel   tttpdced  detwes eee\0" as *const u8 as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        j + 32 as ::core::ffi::c_int,
    );
    if (*item).ident {
        borg_prt_binary(
            f[2 as ::core::ffi::c_int as usize] as uint32_t,
            19 as ::core::ffi::c_int,
            j + 32 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_display_status() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        let mut old: *mut term = Term;
        if !angband_term[j as usize].is_null() {
            if window_flag[j as usize] as ::core::ffi::c_long & 0x8000 as ::core::ffi::c_long != 0 {
                let mut attr: uint8_t = 0;
                Term_activate(angband_term[j as usize]);
                Term_putstr(
                    5 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"RESISTS\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if *borg.trait_0.offset(BI_RACID as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if borg.temp.res_acid {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if *borg.trait_0.offset(BI_IACID as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Acid\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if *borg.trait_0.offset(BI_RELEC as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if borg.temp.res_elec {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if *borg.trait_0.offset(BI_IELEC as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Elec\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if *borg.trait_0.offset(BI_RFIRE as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if borg.temp.res_fire {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fire\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if *borg.trait_0.offset(BI_RCOLD as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if borg.temp.res_cold {
                    attr = COLOUR_GREEN as uint8_t;
                }
                if *borg.trait_0.offset(BI_ICOLD as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cold\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                if *borg.trait_0.offset(BI_RPOIS as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if borg.temp.res_pois {
                    attr = COLOUR_GREEN as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Pois\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RFEAR as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fear\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RLITE as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Lite\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RDARK as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Dark\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_RBLIND as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Blind\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RCONF as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Confu\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RSND as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Sound\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RSHRD as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Shard\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RNXUS as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Nexus\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RNTHR as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Nethr\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RKAOS as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Chaos\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_RDIS as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    6 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Disen\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SDIG as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"S.Dig\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_FEATH as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Feath\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"PLite\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_REG as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Regen\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_ESP as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Telep\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SINV as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Invis\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_FRACT as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"FrAct\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_HLIFE as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    12 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"HLife\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    5 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Weapon Slays:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_ANIMAL as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Animal\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Evil\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_UNDEAD as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Undead\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_DEMON as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if *borg
                    .trait_0
                    .offset(BI_WK_DEMON as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_GREEN as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Demon\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_ORC as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Orc\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_TROLL as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Troll\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_GIANT as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Giant\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WS_DRAGON as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                }
                if *borg
                    .trait_0
                    .offset(BI_WK_DRAGON as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_GREEN as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Dragon\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WB_ACID as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Acid\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WB_COLD as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    8 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cold\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WB_ELEC as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    15 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Elec\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_WB_FIRE as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    22 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fire\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Concerns:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_FIRST_CURSED as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cursed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Weak\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Poison\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Cut\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    36 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Stun\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg
                    .trait_0
                    .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                    != 0
                {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Confused\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.goal.fleeing {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    29 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Goal Fleeing\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.no_rest_prep as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    attr = COLOUR_BLUE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    43 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"No Resting\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    60 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Time:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"This Level         \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    11 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        (borg.time.now - borg.time.level) as ::core::ffi::c_long,
                    ),
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"Since Town         \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    12 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        (borg.time.now - borg.time.town) as ::core::ffi::c_long,
                    ),
                );
                Term_putstr(
                    54 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_SLATE,
                    b"Anti-bounce count  \0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    65 as ::core::ffi::c_int,
                    13 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                        borg.antibounce_count as ::core::ffi::c_int,
                    ),
                );
                Term_putstr(
                    19 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Sustains\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SSTR as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"STR\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SINT as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"INT\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SWIS as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"WIS\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SDEX as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"DEX\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if *borg.trait_0.offset(BI_SCON as ::core::ffi::c_int as isize) != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    21 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"CON\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Temp Effects\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.prot_from_evil {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Prot. Evil\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.fastcast {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fastcast\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.hero {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.berserk {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Berserk\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.shield {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Shielded\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.bless {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Blessed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.temp.fast {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Fast\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.see_inv as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    28 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"See Inv\0" as *const u8 as *const ::core::ffi::c_char,
                );
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    b"Level Information\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.status.vault {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Vault on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.mon.unique != 0 {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Unique on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.mon.unique != 0 {
                    Term_putstr(
                        58 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"(%s)\0" as *const u8 as *const ::core::ffi::c_char,
                            (*r_info.offset(borg.mon.unique as isize)).name,
                        ),
                    );
                } else {
                    Term_putstr(
                        58 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                                                \0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if borg.mon.scary {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    3 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Scary Guy on level\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.near.breeder {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Breeder level (closing doors)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if borg.near.summoner_idx as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
                    attr = COLOUR_WHITE as uint8_t;
                } else {
                    attr = COLOUR_SLATE as uint8_t;
                }
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    5 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Summoner very close (AS-Corridor)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Reason for not diving:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_WHITE as uint8_t;
                Term_putstr(
                    64 as ::core::ffi::c_int,
                    6 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    format(
                        b"%s                              \0" as *const u8
                            as *const ::core::ffi::c_char,
                        borg_prepared(
                            *borg
                                .trait_0
                                .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                                + 1 as ::core::ffi::c_int,
                        ),
                    ),
                );
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    7 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Scumming: not active                          \0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
                    != 0 as ::core::ffi::c_int
                {
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        42 as ::core::ffi::c_int,
                        7 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"Scumming: $%d                  \0" as *const u8
                                as *const ::core::ffi::c_char,
                            *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize),
                        ),
                    );
                }
                attr = COLOUR_SLATE as uint8_t;
                Term_putstr(
                    42 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    b"Maximal Depth:\0" as *const u8 as *const ::core::ffi::c_char,
                );
                attr = COLOUR_WHITE as uint8_t;
                Term_putstr(
                    56 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    -(1 as ::core::ffi::c_int),
                    attr as ::core::ffi::c_int,
                    format(
                        b"%d    \0" as *const u8 as *const ::core::ffi::c_char,
                        *borg
                            .trait_0
                            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize),
                    ),
                );
                if *borg
                    .trait_0
                    .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                    >= 50 as ::core::ffi::c_int
                {
                    Term_putstr(
                        5 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"Important Deep Events:\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home *Heal*:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        13 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            num_ezheal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home Heal:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            num_heal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Home Life:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            num_life as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Res_Mana:        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d   \0" as *const u8 as *const ::core::ffi::c_char,
                            num_mana as ::core::ffi::c_int,
                        ),
                    );
                    if borg.near.morgoth {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        20 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"Morgoth on Level.  Last seen:%ld       \0" as *const u8
                                as *const ::core::ffi::c_char,
                            (borg.time.now - borg.time.morgoth) as ::core::ffi::c_long,
                        ),
                    );
                    if borg.morgoth_position {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    if borg_needs_new_sea {
                        attr = COLOUR_WHITE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        21 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Sea of Runes.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if borg.ready_morgoth != 0 {
                        attr = COLOUR_BLUE as uint8_t;
                    } else {
                        attr = COLOUR_SLATE as uint8_t;
                    }
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        22 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"Ready for Morgoth.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    Term_putstr(
                        5 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"                        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        10 as ::core::ffi::c_int,
                        16 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            num_ezheal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        10 as ::core::ffi::c_int,
                        17 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            num_life as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            num_heal as ::core::ffi::c_int,
                        ),
                    );
                    attr = COLOUR_SLATE as uint8_t;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        b"                   \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    attr = COLOUR_WHITE as uint8_t;
                    Term_putstr(
                        11 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int,
                        -(1 as ::core::ffi::c_int),
                        attr as ::core::ffi::c_int,
                        format(
                            b"%d       \0" as *const u8 as *const ::core::ffi::c_char,
                            num_mana as ::core::ffi::c_int,
                        ),
                    );
                }
                Term_fresh();
                Term_activate(old);
            }
        }
        j += 1;
    }
}
pub const PW_BORG_2: ::core::ffi::c_long = 0x8000 as ::core::ffi::c_long;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
