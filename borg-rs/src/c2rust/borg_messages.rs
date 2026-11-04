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
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strnfmt(
        buf: *mut ::core::ffi::c_char,
        max: size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> size_t;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn mem_alloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn mem_realloc(p: *mut ::core::ffi::c_void, len: size_t) -> *mut ::core::ffi::c_void;
    fn string_make(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn string_free(str: *mut ::core::ffi::c_char);
    fn prefix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn suffix(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    static mut player: *mut player;
    static mut Term: *mut term;
    static mut tile_width: uint8_t;
    fn Term_xtra(n: ::core::ffi::c_int, v: ::core::ffi::c_int) -> errr;
    static mut inkey_flag: bool;
    static mut cave: *mut chunk;
    fn square_iswarded(c: *mut chunk, grid: loc) -> bool;
    static mut blow_methods: *mut blow_method;
    static mut pain_messages: *mut monster_pain;
    static mut monster_spells: *mut monster_spell;
    static mut z_info: *mut angband_constants;
    static mut borg_items: *mut borg_item;
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_grids: [*mut borg_grid; 66];
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut successful_target: ::core::ffi::c_int;
    static mut target_closest: ::core::ffi::c_int;
    static mut borg_attempting_refresh_resist: bool;
    static mut track_glyph: borg_track;
    static mut borg_needs_new_sea: bool;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    fn borg_delete_kill(i: ::core::ffi::c_int);
    static mut track_less: borg_track;
    static mut track_more: borg_track;
    static mut borg_confirm_target: bool;
    fn borg_what_text(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        a: *mut uint8_t,
        s: *mut ::core::ffi::c_char,
    ) -> errr;
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_keypress(k: keycode_t) -> errr;
    fn borg_flush();
    static mut borg_dont_react: bool;
    fn borg_react(msg: *const ::core::ffi::c_char, buf: *const ::core::ffi::c_char);
    fn borg_react_prompted(
        buf: *const ::core::ffi::c_char,
        key: *mut keypress,
        cursor: loc,
    ) -> bool;
    fn borg_oops(what: *const ::core::ffi::c_char);
    static mut borg_failure: bool;
    fn borg_trim(line: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub const SOUND_MAX: C2RustUnnamed = 153;
pub const MSG_MAX: C2RustUnnamed = 153;
pub const MSG_SCRAMBLE: C2RustUnnamed = 152;
pub const MSG_MULTIPLY: C2RustUnnamed = 151;
pub const MSG_DRAIN_STAT: C2RustUnnamed = 150;
pub const MSG_KILL_KING: C2RustUnnamed = 149;
pub const MSG_KILL_UNIQUE: C2RustUnnamed = 148;
pub const MSG_PRAYER: C2RustUnnamed = 147;
pub const MSG_SPELL: C2RustUnnamed = 146;
pub const MSG_HIT_HI_SUPERB: C2RustUnnamed = 145;
pub const MSG_HIT_HI_GREAT: C2RustUnnamed = 144;
pub const MSG_HIT_SUPERB: C2RustUnnamed = 143;
pub const MSG_HIT_GREAT: C2RustUnnamed = 142;
pub const MSG_HIT_GOOD: C2RustUnnamed = 141;
pub const MSG_CAST_FEAR: C2RustUnnamed = 140;
pub const MSG_SHRIEK: C2RustUnnamed = 139;
pub const MSG_CREATE_TRAP: C2RustUnnamed = 138;
pub const MSG_AMBIENT_DNG5: C2RustUnnamed = 137;
pub const MSG_AMBIENT_DNG4: C2RustUnnamed = 136;
pub const MSG_AMBIENT_DNG3: C2RustUnnamed = 135;
pub const MSG_AMBIENT_DNG2: C2RustUnnamed = 134;
pub const MSG_AMBIENT_DNG1: C2RustUnnamed = 133;
pub const MSG_AMBIENT_NITE: C2RustUnnamed = 132;
pub const MSG_AMBIENT_DAY: C2RustUnnamed = 131;
pub const MSG_NOTICE: C2RustUnnamed = 130;
pub const MSG_HUNGRY: C2RustUnnamed = 129;
pub const MSG_RUNE: C2RustUnnamed = 128;
pub const MSG_CURSED: C2RustUnnamed = 127;
pub const MSG_QUIVER: C2RustUnnamed = 126;
pub const MSG_WIELD: C2RustUnnamed = 125;
pub const MSG_SUM_UNIQUE: C2RustUnnamed = 124;
pub const MSG_SUM_WRAITH: C2RustUnnamed = 123;
pub const MSG_SUM_HI_DEMON: C2RustUnnamed = 122;
pub const MSG_SUM_HI_DRAGON: C2RustUnnamed = 121;
pub const MSG_SUM_HI_UNDEAD: C2RustUnnamed = 120;
pub const MSG_SUM_DRAGON: C2RustUnnamed = 119;
pub const MSG_SUM_DEMON: C2RustUnnamed = 118;
pub const MSG_SUM_HYDRA: C2RustUnnamed = 117;
pub const MSG_SUM_HOUND: C2RustUnnamed = 116;
pub const MSG_SUM_SPIDER: C2RustUnnamed = 115;
pub const MSG_SUM_ANIMAL: C2RustUnnamed = 114;
pub const MSG_SUM_UNDEAD: C2RustUnnamed = 113;
pub const MSG_SUM_AINU: C2RustUnnamed = 112;
pub const MSG_SUM_MONSTER: C2RustUnnamed = 111;
pub const MSG_BR_FORCE: C2RustUnnamed = 110;
pub const MSG_BR_PLASMA: C2RustUnnamed = 109;
pub const MSG_BR_GRAVITY: C2RustUnnamed = 108;
pub const MSG_BR_INERTIA: C2RustUnnamed = 107;
pub const MSG_BR_TIME: C2RustUnnamed = 106;
pub const MSG_BR_NEXUS: C2RustUnnamed = 105;
pub const MSG_BR_NETHER: C2RustUnnamed = 104;
pub const MSG_BR_DARK: C2RustUnnamed = 103;
pub const MSG_BR_LIGHT: C2RustUnnamed = 102;
pub const MSG_BR_SOUND: C2RustUnnamed = 101;
pub const MSG_BR_SHARDS: C2RustUnnamed = 100;
pub const MSG_BR_CHAOS: C2RustUnnamed = 99;
pub const MSG_BR_DISEN: C2RustUnnamed = 98;
pub const MSG_BR_FIRE: C2RustUnnamed = 97;
pub const MSG_BR_GAS: C2RustUnnamed = 96;
pub const MSG_BR_ACID: C2RustUnnamed = 95;
pub const MSG_BR_ELEC: C2RustUnnamed = 94;
pub const MSG_BR_FROST: C2RustUnnamed = 93;
pub const MSG_BR_ELEMENTS: C2RustUnnamed = 92;
pub const MSG_IDENT_ART: C2RustUnnamed = 91;
pub const MSG_IDENT_EGO: C2RustUnnamed = 90;
pub const MSG_IDENT_BAD: C2RustUnnamed = 89;
pub const MSG_DISARM: C2RustUnnamed = 88;
pub const MSG_LOCKPICK: C2RustUnnamed = 87;
pub const MSG_STORE5: C2RustUnnamed = 86;
pub const MSG_SHOOT_HIT: C2RustUnnamed = 85;
pub const MSG_MONEY3: C2RustUnnamed = 84;
pub const MSG_MONEY2: C2RustUnnamed = 83;
pub const MSG_MONEY1: C2RustUnnamed = 82;
pub const MSG_STORE_HOME: C2RustUnnamed = 81;
pub const MSG_STORE_LEAVE: C2RustUnnamed = 80;
pub const MSG_STORE_ENTER: C2RustUnnamed = 79;
pub const MSG_STAIRS_UP: C2RustUnnamed = 78;
pub const MSG_CUT: C2RustUnnamed = 77;
pub const MSG_STUN: C2RustUnnamed = 76;
pub const MSG_RES_POIS: C2RustUnnamed = 75;
pub const MSG_RES_COLD: C2RustUnnamed = 74;
pub const MSG_RES_FIRE: C2RustUnnamed = 73;
pub const MSG_RES_ELEC: C2RustUnnamed = 72;
pub const MSG_RES_ACID: C2RustUnnamed = 71;
pub const MSG_INFRARED: C2RustUnnamed = 70;
pub const MSG_SEE_INVIS: C2RustUnnamed = 69;
pub const MSG_INVULN: C2RustUnnamed = 68;
pub const MSG_PROT_EVIL: C2RustUnnamed = 67;
pub const MSG_BOLD: C2RustUnnamed = 66;
pub const MSG_BERSERK: C2RustUnnamed = 65;
pub const MSG_HERO: C2RustUnnamed = 64;
pub const MSG_BLESSED: C2RustUnnamed = 63;
pub const MSG_SHIELD: C2RustUnnamed = 62;
pub const MSG_SLOW: C2RustUnnamed = 61;
pub const MSG_SPEED: C2RustUnnamed = 60;
pub const MSG_DRUGGED: C2RustUnnamed = 59;
pub const MSG_PARALYZED: C2RustUnnamed = 58;
pub const MSG_AFRAID: C2RustUnnamed = 57;
pub const MSG_POISONED: C2RustUnnamed = 56;
pub const MSG_CONFUSED: C2RustUnnamed = 55;
pub const MSG_BLIND: C2RustUnnamed = 54;
pub const MSG_RECOVER: C2RustUnnamed = 53;
pub const MSG_MON_MOAN: C2RustUnnamed = 52;
pub const MSG_MON_INSULT: C2RustUnnamed = 51;
pub const MSG_MON_BEG: C2RustUnnamed = 50;
pub const MSG_MON_SPORE: C2RustUnnamed = 49;
pub const MSG_MON_WAIL: C2RustUnnamed = 48;
pub const MSG_MON_GAZE: C2RustUnnamed = 47;
pub const MSG_MON_SPIT: C2RustUnnamed = 46;
pub const MSG_MON_DROOL: C2RustUnnamed = 45;
pub const MSG_MON_CRAWL: C2RustUnnamed = 44;
pub const MSG_MON_ENGULF: C2RustUnnamed = 43;
pub const MSG_MON_CRUSH: C2RustUnnamed = 42;
pub const MSG_MON_BUTT: C2RustUnnamed = 41;
pub const MSG_MON_STING: C2RustUnnamed = 40;
pub const MSG_MON_BITE: C2RustUnnamed = 39;
pub const MSG_MON_CLAW: C2RustUnnamed = 38;
pub const MSG_MON_KICK: C2RustUnnamed = 37;
pub const MSG_MON_PUNCH: C2RustUnnamed = 36;
pub const MSG_MON_TOUCH: C2RustUnnamed = 35;
pub const MSG_MON_HIT: C2RustUnnamed = 34;
pub const MSG_DESTROY: C2RustUnnamed = 33;
pub const MSG_USE_STAFF: C2RustUnnamed = 32;
pub const MSG_ACT_ARTIFACT: C2RustUnnamed = 31;
pub const MSG_HITPOINT_WARN: C2RustUnnamed = 30;
pub const MSG_STAIRS_DOWN: C2RustUnnamed = 29;
pub const MSG_LOCKPICK_FAIL: C2RustUnnamed = 28;
pub const MSG_NOTHING_TO_OPEN: C2RustUnnamed = 27;
pub const MSG_BELL: C2RustUnnamed = 26;
pub const MSG_TPLEVEL: C2RustUnnamed = 25;
pub const MSG_SHUTDOOR: C2RustUnnamed = 24;
pub const MSG_OPENDOOR: C2RustUnnamed = 23;
pub const MSG_DIG: C2RustUnnamed = 22;
pub const MSG_STORE4: C2RustUnnamed = 21;
pub const MSG_STORE3: C2RustUnnamed = 20;
pub const MSG_STORE2: C2RustUnnamed = 19;
pub const MSG_STORE1: C2RustUnnamed = 18;
pub const MSG_EAT: C2RustUnnamed = 17;
pub const MSG_HITWALL: C2RustUnnamed = 16;
pub const MSG_TPOTHER: C2RustUnnamed = 15;
pub const MSG_WALK: C2RustUnnamed = 14;
pub const MSG_ZAP_ROD: C2RustUnnamed = 13;
pub const MSG_QUAFF: C2RustUnnamed = 12;
pub const MSG_SHOOT: C2RustUnnamed = 11;
pub const MSG_TELEPORT: C2RustUnnamed = 10;
pub const MSG_STUDY: C2RustUnnamed = 9;
pub const MSG_DEATH: C2RustUnnamed = 8;
pub const MSG_LEVEL: C2RustUnnamed = 7;
pub const MSG_KILL: C2RustUnnamed = 6;
pub const MSG_DROP: C2RustUnnamed = 5;
pub const MSG_FLEE: C2RustUnnamed = 4;
pub const MSG_MISS: C2RustUnnamed = 3;
pub const MSG_HIT: C2RustUnnamed = 2;
pub const MSG_BIRTH: C2RustUnnamed = 1;
pub const MSG_GENERIC: C2RustUnnamed = 0;
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
pub const BI_CLEVEL: C2RustUnnamed_11 = 35;
pub const BI_CDEPTH: C2RustUnnamed_11 = 105;
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
pub const FEAT_BROKEN: C2RustUnnamed_10 = 4;
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
pub const FEAT_RUBBLE: C2RustUnnamed_10 = 16;
pub const BI_LIGHT: C2RustUnnamed_11 = 26;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_track {
    pub num: int16_t,
    pub size: int16_t,
    pub x: *mut ::core::ffi::c_int,
    pub y: *mut ::core::ffi::c_int,
}
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
pub const BI_ISCONFUSED: C2RustUnnamed_11 = 114;
pub const BI_ISIMAGE: C2RustUnnamed_11 = 120;
pub const FEAT_MAGMA: C2RustUnnamed_10 = 17;
pub const FEAT_QUARTZ: C2RustUnnamed_10 = 18;
pub const FEAT_MAGMA_K: C2RustUnnamed_10 = 19;
pub const FEAT_QUARTZ_K: C2RustUnnamed_10 = 20;
pub const FEAT_GRANITE: C2RustUnnamed_10 = 21;
pub const FEAT_PERM: C2RustUnnamed_10 = 22;
pub const FEAT_OPEN: C2RustUnnamed_10 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_read_messages {
    pub count: ::core::ffi::c_int,
    pub allocated: ::core::ffi::c_int,
    pub messages: *mut borg_read_message,
    pub index: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_read_message {
    pub message_p1: *mut ::core::ffi::c_char,
    pub message_p2: *mut ::core::ffi::c_char,
    pub message_p3: *mut ::core::ffi::c_char,
}
pub const BI_ISFORGET: C2RustUnnamed_11 = 121;
pub const BORG_VERBOSE: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub msg: *const ::core::ffi::c_char,
    pub omit_subject: bool,
    pub type_0: ::core::ffi::c_int,
}
pub const MON_MSG_0: mon_messages = 61;
pub const MON_MSG_10: mon_messages = 60;
pub const MON_MSG_20: mon_messages = 59;
pub const MON_MSG_35: mon_messages = 58;
pub const MON_MSG_50: mon_messages = 57;
pub const MON_MSG_75: mon_messages = 56;
pub const MON_MSG_95: mon_messages = 55;
pub const MON_MSG_DISAPPEAR: mon_messages = 21;
pub const MON_MSG_MAX: mon_messages = 62;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_spell {
    pub next: *mut monster_spell,
    pub index: uint16_t,
    pub msgt: ::core::ffi::c_int,
    pub hit: ::core::ffi::c_int,
    pub effect: *mut effect,
    pub level: *mut monster_spell_level,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_spell_level {
    pub next: *mut monster_spell_level,
    pub power: ::core::ffi::c_int,
    pub lore_desc: *mut ::core::ffi::c_char,
    pub lore_attr: uint8_t,
    pub lore_attr_resist: uint8_t,
    pub lore_attr_immune: uint8_t,
    pub message: *mut ::core::ffi::c_char,
    pub blind_message: *mut ::core::ffi::c_char,
    pub miss_message: *mut ::core::ffi::c_char,
    pub save_message: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_10 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_10 = 24;
pub const FEAT_LAVA: C2RustUnnamed_10 = 23;
pub const FEAT_SECRET: C2RustUnnamed_10 = 15;
pub const FEAT_HOME: C2RustUnnamed_10 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_10 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_10 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_10 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_10 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_10 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_10 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_10 = 7;
pub const FEAT_MORE: C2RustUnnamed_10 = 6;
pub const FEAT_LESS: C2RustUnnamed_10 = 5;
pub const FEAT_CLOSED: C2RustUnnamed_10 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_10 = 1;
pub const FEAT_NONE: C2RustUnnamed_10 = 0;
pub type mon_messages = ::core::ffi::c_uint;
pub const MON_MSG_QUAKE_HURT: mon_messages = 54;
pub const MON_MSG_QUAKE_DEATH: mon_messages = 53;
pub const MON_MSG_HIT_AND_RUN: mon_messages = 52;
pub const MON_MSG_APPEAR: mon_messages = 51;
pub const MON_MSG_UNHARMED: mon_messages = 50;
pub const MON_MSG_MAINTAIN_SHAPE: mon_messages = 49;
pub const MON_MSG_BRIEF_PUZZLE: mon_messages = 48;
pub const MON_MSG_MANA_DRAIN: mon_messages = 47;
pub const MON_MSG_FREEZE_SHATTER: mon_messages = 46;
pub const MON_MSG_DISINTEGRATES: mon_messages = 45;
pub const MON_MSG_MORIA_DEATH: mon_messages = 44;
pub const MON_MSG_SHAPE_FAIL: mon_messages = 43;
pub const MON_MSG_NOT_COMMAND: mon_messages = 42;
pub const MON_MSG_COMMAND: mon_messages = 41;
pub const MON_MSG_NOT_DISEN: mon_messages = 40;
pub const MON_MSG_DISEN: mon_messages = 39;
pub const MON_MSG_NOT_HELD: mon_messages = 38;
pub const MON_MSG_HELD: mon_messages = 37;
pub const MON_MSG_NOT_AFRAID: mon_messages = 36;
pub const MON_MSG_FLEE_IN_TERROR: mon_messages = 35;
pub const MON_MSG_MORE_AFRAID: mon_messages = 34;
pub const MON_MSG_NOT_HASTED: mon_messages = 33;
pub const MON_MSG_HASTED: mon_messages = 32;
pub const MON_MSG_MORE_HASTED: mon_messages = 31;
pub const MON_MSG_NOT_SLOWED: mon_messages = 30;
pub const MON_MSG_SLOWED: mon_messages = 29;
pub const MON_MSG_MORE_SLOWED: mon_messages = 28;
pub const MON_MSG_NOT_CONFUSED: mon_messages = 27;
pub const MON_MSG_CONFUSED: mon_messages = 26;
pub const MON_MSG_MORE_CONFUSED: mon_messages = 25;
pub const MON_MSG_NOT_DAZED: mon_messages = 24;
pub const MON_MSG_DAZED: mon_messages = 23;
pub const MON_MSG_MORE_DAZED: mon_messages = 22;
pub const MON_MSG_CHANGE: mon_messages = 20;
pub const MON_MSG_SHUDDER: mon_messages = 19;
pub const MON_MSG_BADLY_FROZEN: mon_messages = 18;
pub const MON_MSG_CATCH_FIRE: mon_messages = 17;
pub const MON_MSG_DISSOLVE: mon_messages = 16;
pub const MON_MSG_LOSE_SKIN: mon_messages = 15;
pub const MON_MSG_SHRIVEL_LIGHT: mon_messages = 14;
pub const MON_MSG_CRINGE_LIGHT: mon_messages = 13;
pub const MON_MSG_WAKES_UP: mon_messages = 12;
pub const MON_MSG_FALL_ASLEEP: mon_messages = 11;
pub const MON_MSG_HEALTHIER: mon_messages = 10;
pub const MON_MSG_SPAWN: mon_messages = 9;
pub const MON_MSG_UNAFFECTED: mon_messages = 8;
pub const MON_MSG_RESIST_SOMEWHAT: mon_messages = 7;
pub const MON_MSG_IMMUNE: mon_messages = 6;
pub const MON_MSG_RESIST: mon_messages = 5;
pub const MON_MSG_HIT_HARD: mon_messages = 4;
pub const MON_MSG_RESIST_A_LOT: mon_messages = 3;
pub const MON_MSG_DESTROYED: mon_messages = 2;
pub const MON_MSG_DIE: mon_messages = 1;
pub const MON_MSG_NONE: mon_messages = 0;
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
pub const BI_ISPARALYZED: C2RustUnnamed_11 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_11 = 118;
pub const BI_ISSTUN: C2RustUnnamed_11 = 117;
pub const BI_ISCUT: C2RustUnnamed_11 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_11 = 115;
pub const BI_ISAFRAID: C2RustUnnamed_11 = 113;
pub const BI_ISBLIND: C2RustUnnamed_11 = 112;
pub const BI_ISGORGED: C2RustUnnamed_11 = 111;
pub const BI_ISFULL: C2RustUnnamed_11 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_11 = 109;
pub const BI_ISWEAK: C2RustUnnamed_11 = 108;
pub const BI_KING: C2RustUnnamed_11 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_11 = 106;
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
pub const BI_GOLD: C2RustUnnamed_11 = 45;
pub const BI_SPEED: C2RustUnnamed_11 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_11 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_11 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_11 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_11 = 40;
pub const BI_FOOD: C2RustUnnamed_11 = 39;
pub const BI_RECALL: C2RustUnnamed_11 = 38;
pub const BI_ESP: C2RustUnnamed_11 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_11 = 36;
pub const BI_FAIL2: C2RustUnnamed_11 = 34;
pub const BI_FAIL1: C2RustUnnamed_11 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_11 = 32;
pub const BI_MAXSP: C2RustUnnamed_11 = 31;
pub const BI_CURSP: C2RustUnnamed_11 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_11 = 29;
pub const BI_MAXHP: C2RustUnnamed_11 = 28;
pub const BI_CURHP: C2RustUnnamed_11 = 27;
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
pub const BORG_SELF_LUNAL: C2RustUnnamed_12 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_12 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_12 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_12 = 24;
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
pub const BORG_USES_SWAPS: C2RustUnnamed_12 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_12 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_12 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_12 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_12 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_12 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_12 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_12 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_12 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_12 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_12 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_12 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_DARK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const TERM_XTRA_NOISE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_msg_len: int16_t = 0;
#[no_mangle]
pub static mut borg_msg_siz: int16_t = 0;
#[no_mangle]
pub static mut borg_msg_buf: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_msg_num: int16_t = 0;
#[no_mangle]
pub static mut borg_msg_max: int16_t = 0;
#[no_mangle]
pub static mut borg_msg_pos: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
#[no_mangle]
pub static mut borg_msg_use: *mut int16_t = ::core::ptr::null::<int16_t>() as *mut int16_t;
static mut suffix_pain: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_match: [::core::ffi::c_char; 128] = unsafe {
    ::core::mem::transmute::<
        [u8; 128],
        [::core::ffi::c_char; 128],
    >(
        *b"plain gold ring\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut prefix_kill: [*const ::core::ffi::c_char; 4] = [
    b"You have killed \0" as *const u8 as *const ::core::ffi::c_char,
    b"You have slain \0" as *const u8 as *const ::core::ffi::c_char,
    b"You have destroyed \0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_died: [*const ::core::ffi::c_char; 18] = [
    b" die.\0" as *const u8 as *const ::core::ffi::c_char,
    b" dies.\0" as *const u8 as *const ::core::ffi::c_char,
    b" is destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
    b" are destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
    b" is destroyed!\0" as *const u8 as *const ::core::ffi::c_char,
    b" are destroyed!\0" as *const u8 as *const ::core::ffi::c_char,
    b" shrivel away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
    b" shrivels away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
    b" dissolve!\0" as *const u8 as *const ::core::ffi::c_char,
    b" dissolves!\0" as *const u8 as *const ::core::ffi::c_char,
    b" scream of agony!\0" as *const u8 as *const ::core::ffi::c_char,
    b" screams of agony!\0" as *const u8 as *const ::core::ffi::c_char,
    b" disintegrate!\0" as *const u8 as *const ::core::ffi::c_char,
    b" disintegrates!\0" as *const u8 as *const ::core::ffi::c_char,
    b" freeze and shatter!\0" as *const u8 as *const ::core::ffi::c_char,
    b" freezes and shatters!\0" as *const u8 as *const ::core::ffi::c_char,
    b" is drained dry!\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_blink: [*const ::core::ffi::c_char; 6] = [
    b" disappears!\0" as *const u8 as *const ::core::ffi::c_char,
    b" intones strange words.\0" as *const u8 as *const ::core::ffi::c_char,
    b" teleports away.\0" as *const u8 as *const ::core::ffi::c_char,
    b" blinks.\0" as *const u8 as *const ::core::ffi::c_char,
    b" makes a soft 'pop'.\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_hit_by: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
static mut spell_msgs: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
static mut spell_invis_msgs: borg_read_messages = borg_read_messages {
    count: 0,
    allocated: 0,
    messages: ::core::ptr::null::<borg_read_message>() as *mut borg_read_message,
    index: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
unsafe extern "C" fn borg_message_contains(
    mut value: *const ::core::ffi::c_char,
    mut message: *mut borg_read_message,
) -> bool {
    if !strstr(value, (*message).message_p1).is_null()
        && ((*message).message_p2.is_null() || !strstr(value, (*message).message_p2).is_null())
        && ((*message).message_p3.is_null() || !strstr(value, (*message).message_p3).is_null())
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
static mut prefix_feeling_danger: [*const ::core::ffi::c_char; 11] = [
    b"You are still uncertain about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omens of death haunt this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems murderous\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems terribly dangerous\0" as *const u8 as *const ::core::ffi::c_char,
    b"You feel anxious about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"You feel nervous about this place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place does not seem too risky\0" as *const u8 as *const ::core::ffi::c_char,
    b"This place seems reasonably safe\0" as *const u8 as *const ::core::ffi::c_char,
    b"This seems a tame, sheltered place\0" as *const u8 as *const ::core::ffi::c_char,
    b"This seems a quiet, peaceful place\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut suffix_feeling_stuff: [*const ::core::ffi::c_char; 12] = [
    b"Looks like any other level.\0" as *const u8 as *const ::core::ffi::c_char,
    b"you sense an item of wondrous power!\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are superb treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are excellent treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are very good treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are good treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there may be something worthwhile here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there may not be much interesting here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there aren't many treasures here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there are only scraps of junk here.\0" as *const u8 as *const ::core::ffi::c_char,
    b"there is naught but cobwebs here.\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn borg_get_messages(mut key: *mut keypress, mut cursor: loc) -> bool {
    let mut borg_prompt: bool = false_0 != 0;
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: *mut ::core::ffi::c_char = &raw mut buffer as *mut ::core::ffi::c_char;
    let mut t_a: uint8_t = 0;
    buf = &raw mut buffer as *mut ::core::ffi::c_char;
    borg_what_text(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        ((*Term).wid - 1 as ::core::ffi::c_int) / tile_width as ::core::ffi::c_int,
        &raw mut t_a,
        &raw mut buffer as *mut ::core::ffi::c_char,
    );
    buf = borg_trim(buf);
    if t_a as ::core::ffi::c_int != COLOUR_DARK
        && (*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
            || *buf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32)
    {
        borg_prompt = true_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0
        && prefix(buf, b"Type\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        borg_prompt = false_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0 && !inkey_flag && strlen(buf) != 0 {
        if borg_react_prompted(buf, key, cursor) {
            return true_0 != 0;
        }
    }
    buf = &raw mut buffer as *mut ::core::ffi::c_char;
    if borg_prompt as ::core::ffi::c_int != 0
        && !inkey_flag
        && cursor.y == 0 as ::core::ffi::c_int
        && cursor.x >= 7 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == borg_what_text(
                cursor.x - 7 as ::core::ffi::c_int,
                cursor.y,
                7 as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        && suffix(buf, b" -more-\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
            borg_note(b"# message with -more-\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if 0 as ::core::ffi::c_int
            == borg_what_text(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                cursor.x - 7 as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        {
            borg_parse(buf);
        }
        if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
            borg_note(b"clearing -more-\0" as *const u8 as *const ::core::ffi::c_char);
        }
        (*key).code = ' ' as i32 as keycode_t;
        return true_0 != 0;
    }
    if borg_prompt as ::core::ffi::c_int != 0 && inkey_flag as ::core::ffi::c_int != 0 {
        if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
            borg_note(b"# parse normal message\0" as *const u8 as *const ::core::ffi::c_char);
        }
        buf = &raw mut buffer as *mut ::core::ffi::c_char;
        if 0 as ::core::ffi::c_int
            == borg_what_text(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                ((*Term).wid - 1 as ::core::ffi::c_int) / tile_width as ::core::ffi::c_int,
                &raw mut t_a,
                &raw mut buffer as *mut ::core::ffi::c_char,
            )
        {
            let mut k: ::core::ffi::c_int = strlen(buf) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int
                && *buf.offset((k - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == ' ' as i32
            {
                k -= 1;
            }
            *buf.offset(k as isize) = '\0' as i32 as ::core::ffi::c_char;
            borg_parse(buf);
        }
        (*key).code = ' ' as i32 as keycode_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_parse_aux(
    mut msg: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut x9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut who: [::core::ffi::c_char; 256] = [0; 256];
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.goal.g.y as isize))
    .offset(borg.goal.g.x as isize) as *mut borg_grid;
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 {
        borg_note(format(
            b"# Parse Msg bite <%s>\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
    }
    if prefix(
        msg,
        b"You die.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if !((*player).wizard as ::core::ffi::c_int != 0
            || (*player).opts.opt[OPT_cheat_live as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                != 0
            || borg.status.cheat_death as ::core::ffi::c_int != 0)
        {
            borg_oops(b"death\0" as *const u8 as *const ::core::ffi::c_char);
            if borg.status.respawning == 0 {
                borg.status.active = false_0 != 0;
            }
            Term_xtra(TERM_XTRA_NOISE, 1 as ::core::ffi::c_int);
        }
        return;
    }
    if prefix(
        msg,
        b"You failed \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_note(b"# Normal failure.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_failure = true_0 != 0;
        borg_flush();
        successful_target = 0 as ::core::ffi::c_int;
        borg_confirm_target = false_0 != 0;
        if borg_attempting_refresh_resist {
            if borg.resistance as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                borg.resistance = (borg.resistance as ::core::ffi::c_int
                    - 25000 as ::core::ffi::c_int) as int16_t;
            }
            borg_attempting_refresh_resist = false_0 != 0;
        }
        return;
    }
    if borg_match[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && !strstr(msg, &raw mut borg_match as *mut ::core::ffi::c_char).is_null()
    {
        borg.status.cancel = true_0 != 0;
    }
    if prefix(
        msg,
        b"You hit a teleport\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"An arrow \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"A small dart \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"The cave \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_react(
            msg,
            b"QUAKE:Somebody\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg_needs_new_sea = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp = strlen(b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"AFRAID:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You feel your memories fade.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        *borg
            .trait_0
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize) = true_0;
    }
    if strcmp(
        msg,
        b"Your memories come flooding back.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        *borg
            .trait_0
            .offset(BI_ISFORGET as ::core::ffi::c_int as isize) = false_0;
    }
    if strcmp(
        msg,
        b"You have been knocked out.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(b"Ignoring Messages While KO'd\0" as *const u8 as *const ::core::ffi::c_char);
        borg_dont_react = true_0 != 0;
    }
    if strcmp(
        msg,
        b"You are paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"Ignoring Messages While Paralyzed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg_dont_react = true_0 != 0;
    }
    if strcmp(
        msg,
        b"You feel drugged!\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"# Hallucinating.  Special control of wanks.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize) = true_0;
    }
    if strcmp(
        msg,
        b"The draining fails.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_react(
            msg,
            b"MISS_BY:something\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if strcmp(
        msg,
        b"You can see clearly again.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg_note(
            b"# Hallucination ended.  Normal control of wanks.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        *borg
            .trait_0
            .offset(BI_ISIMAGE as ::core::ffi::c_int as isize) = false_0;
    }
    if prefix(
        msg,
        b"You hit \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You hit \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"HIT:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You bite \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You bite \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"HIT:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        target_closest = 1 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"You draw power from\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = 1 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"No Available Target.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"This spell must target a monster.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"Not enough room next to \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        target_closest = -(12 as ::core::ffi::c_int);
        return;
    }
    if prefix(
        msg,
        b"You miss \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp =
            strlen(b"You miss \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"MISS:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(
        msg,
        b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        tmp = strlen(b"You are too afraid to attack \0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        strnfmt(
            &raw mut who as *mut ::core::ffi::c_char,
            (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg.offset(tmp as isize),
        );
        strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            256 as size_t,
            b"MISS:%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut who as *mut ::core::ffi::c_char,
        );
        borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
        return;
    }
    if prefix(msg, b"Your \0" as *const u8 as *const ::core::ffi::c_char) {
        if suffix(
            msg,
            b" is unaffected!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            return;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_pain.offset(i as isize)).is_null() {
            if suffix(msg, *suffix_pain.offset(i as isize)) {
                tmp = strlen(*suffix_pain.offset(i as isize)) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"PAIN:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !prefix_kill[i as usize].is_null() {
            if prefix(msg, prefix_kill[i as usize]) {
                tmp = strlen(prefix_kill[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - (tmp + 1 as ::core::ffi::c_int)) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg.offset(tmp as isize),
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"KILL:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !suffix_died[i as usize].is_null() {
            if suffix(msg, suffix_died[i as usize]) {
                tmp = strlen(suffix_died[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"DIED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !suffix_blink[i as usize].is_null() {
            if suffix(msg, suffix_blink[i as usize]) {
                tmp = strlen(suffix_blink[i as usize]) as ::core::ffi::c_int;
                strnfmt(
                    &raw mut who as *mut ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int + len - tmp) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"BLINK:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut who as *mut ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        if suffix(
            msg,
            b" misses you.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" misses you.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"MISS_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" is repelled.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" is repelled.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"MISS_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_hit_by.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if borg_message_contains(
                msg,
                suffix_hit_by.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                let mut start: *mut ::core::ffi::c_char =
                    strstr(msg, (*suffix_hit_by.messages.offset(i as isize)).message_p1);
                if !start.is_null() {
                    strnfmt(
                        &raw mut who as *mut ::core::ffi::c_char,
                        start.offset_from(msg) as ::core::ffi::c_long as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        msg,
                    );
                    strnfmt(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        256 as size_t,
                        b"HIT_BY:%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut who as *mut ::core::ffi::c_char,
                    );
                    borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                    if track_glyph.num != 0 {
                        i = 0 as ::core::ffi::c_int;
                        while i < track_glyph.num as ::core::ffi::c_int {
                            *track_glyph.x.offset(i as isize) = 0 as ::core::ffi::c_int;
                            *track_glyph.y.offset(i as isize) = 0 as ::core::ffi::c_int;
                            i += 1;
                        }
                        track_glyph.num = 0 as int16_t;
                        if square_iswarded(cave, borg.c) {
                            *track_glyph.x.offset(track_glyph.num as isize) = borg.c.x;
                            *track_glyph.y.offset(track_glyph.num as isize) = borg.c.y;
                            track_glyph.num += 1;
                        }
                    }
                    return;
                }
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*spell_invis_msgs.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if !prefix(
                msg,
                b"Something \0" as *const u8 as *const ::core::ffi::c_char,
            ) && !prefix(msg, b"You \0" as *const u8 as *const ::core::ffi::c_char)
            {
                break;
            }
            if borg_message_contains(
                msg,
                spell_invis_msgs.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                strnfmt(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"SPELL_%03d:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    *spell_invis_msgs.index.offset(i as isize),
                    b"Something\0" as *const u8 as *const ::core::ffi::c_char,
                );
                borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                return;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*spell_msgs.messages.offset(i as isize))
            .message_p1
            .is_null()
        {
            if borg_message_contains(
                msg,
                spell_msgs.messages.offset(i as isize) as *mut borg_read_message,
            ) {
                let mut start_0: *mut ::core::ffi::c_char =
                    strstr(msg, (*spell_msgs.messages.offset(i as isize)).message_p1);
                if !start_0.is_null() {
                    strnfmt(
                        &raw mut who as *mut ::core::ffi::c_char,
                        start_0.offset_from(msg) as ::core::ffi::c_long as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        msg,
                    );
                    strnfmt(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        256 as size_t,
                        b"SPELL_%03d:%s\0" as *const u8 as *const ::core::ffi::c_char,
                        *spell_msgs.index.offset(i as isize),
                        &raw mut who as *mut ::core::ffi::c_char,
                    );
                    borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
                    return;
                }
            }
            i += 1;
        }
        if suffix(
            msg,
            b" falls asleep!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" falls asleep!\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_SLEEP:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" looks confused.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" looks confused.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_CONFUSED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" looks more confused.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" looks more confused.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_CONFUSED:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" wakes up.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" wakes up.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE_AWAKE:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" flees in terror!\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" flees in terror!\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__FEAR:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers his courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers his courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers her courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers her courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        if suffix(
            msg,
            b" recovers its courage.\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            tmp = strlen(b" recovers its courage.\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            strnfmt(
                &raw mut who as *mut ::core::ffi::c_char,
                (1 as ::core::ffi::c_int + len - tmp) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                msg,
            );
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"STATE__BOLD:%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut who as *mut ::core::ffi::c_char,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
    }
    if strcmp(
        msg,
        b"The door appears to be broken.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int {
            (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"This seems to be permanent rock.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int >= FEAT_GRANITE as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            (*ag).feat = FEAT_PERM as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the granite wall.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        borg.antibounce_count = 1 as int16_t;
        if (*ag).feat as ::core::ffi::c_int >= FEAT_GRANITE as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            (*ag).feat = FEAT_GRANITE as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the quartz vein.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA_K as ::core::ffi::c_int {
            (*ag).feat = FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        } else if (*ag).feat as ::core::ffi::c_int == FEAT_MAGMA as ::core::ffi::c_int {
            (*ag).feat = FEAT_QUARTZ as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if strcmp(
        msg,
        b"You tunnel into the magma vein.\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ_K as ::core::ffi::c_int {
            (*ag).feat = FEAT_MAGMA_K as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        } else if (*ag).feat as ::core::ffi::c_int == FEAT_QUARTZ as ::core::ffi::c_int {
            (*ag).feat = FEAT_MAGMA as ::core::ffi::c_int as uint8_t;
            borg.goal.type_0 = 0 as int16_t;
        }
        return;
    }
    if prefix(
        msg,
        b"You chip away futilely \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if borg.goal.type_0 as ::core::ffi::c_int == GOAL_KILL
            && (*ag).kill as ::core::ffi::c_int != 0
        {
            borg_delete_kill((*ag).kill as ::core::ffi::c_int);
        }
        return;
    }
    if prefix(
        msg,
        b"The air about you becomes \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.goal.recalling = 15000 as ::core::ffi::c_int + 5000 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The air around you starts \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.goal.descending = 3000 as ::core::ffi::c_int + 2000 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself yanked \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_flush();
        borg.goal.recalling = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The floor opens beneath you!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_flush();
        borg.goal.descending = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"A tension leaves \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.goal.recalling = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"The air around you stops \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.goal.descending = 0 as ::core::ffi::c_int;
        return;
    }
    if prefix(
        msg,
        b"Oops!  It feels deathly cold!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if prefix(
        msg,
        b"You feel safe from evil!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.prot_from_evil = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel safe from evil.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.prot_from_evil = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself moving faster!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.fast = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel yourself slow down.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.fast = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel righteous\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.bless = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"The prayer has expired.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.bless = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel your mind accelerate.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.fastcast = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel your mind slow again.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.fastcast = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel like a hero!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.hero = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel heroic.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.hero = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel like a killing machine!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.berserk = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You no longer feel berserk.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.berserk = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"Your eyes feel very sensitive!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.see_inv = 30000 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"Your eyes no longer feel so sensitive.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.see_inv = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a wall \0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
    {
        borg.status.redraw = true_0 != 0;
        borg.status.need_alter = true_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a closed door blocking your way.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && (*borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
            && *borg
                .trait_0
                .offset(BI_ISIMAGE as ::core::ffi::c_int as isize)
                == 0)
    {
        borg.status.redraw = true_0 != 0;
        borg.status.need_alter = true_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"You spin around.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && *borg
            .trait_0
            .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
            == 0
    {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            if !((*kill).r_idx == 0) {
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
                if d < 4 as ::core::ffi::c_int {
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        borg.status.no_alter = true_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"No known path to \0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"Something is here.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat =
            FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        return;
    }
    if prefix(
        msg,
        b"You see nothing there \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        borg.status.no_alter = true_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"Illegal \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.antibounce_count =
            (borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
        return;
    }
    if prefix(
        msg,
        b"You have nothing to identify\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.antibounce_count =
            (borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
        i = (*z_info).pack_size as ::core::ffi::c_int;
        while i <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
            let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
            if !((*item).iqty == 0) {
                (*item).ident = true_0 != 0;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i <= (*z_info).pack_size as ::core::ffi::c_int {
            let mut item_0: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
            if !((*item_0).iqty == 0) {
                (*item_0).ident = true_0 != 0;
            }
            i += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"Identifying The Phial\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        let mut item_1: *mut borg_item = borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut borg_item;
        (*item_1).ident = true_0 != 0;
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.antibounce_count =
            (borg.antibounce_count as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as int16_t;
    }
    if prefix(
        msg,
        b"You feel resistant to acid!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_acid = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to acid.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_acid = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to electricity!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_elec = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to electricity.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_elec = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to fire!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_fire = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to fire.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_fire = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to cold!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_cold = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to cold.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_cold = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You feel resistant to poison!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_pois = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You are no longer resistant to poison.\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.temp.res_pois = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"A mystic shield forms around your body!\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"Your skin turns to stone.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        borg.temp.shield = true_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"Your mystic shield crumbles away.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"A fleshy shade returns to your skin.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        borg.temp.shield = false_0 != 0;
        return;
    }
    if prefix(
        msg,
        b"You inscribe a mystic symbol on the ground!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 0 as ::core::ffi::c_int;
        while i < track_glyph.num as ::core::ffi::c_int {
            if *track_glyph.x.offset(i as isize) == borg.c.x
                && *track_glyph.y.offset(i as isize) == borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_glyph.num as ::core::ffi::c_int && i < track_glyph.size as ::core::ffi::c_int
        {
            borg_note(
                b"# Noting the creation of a glyph.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            *track_glyph.x.offset(i as isize) = borg.c.x;
            *track_glyph.y.offset(i as isize) = borg.c.y;
            track_glyph.num += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The rune of protection is broken!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 0 as ::core::ffi::c_int;
        while i < track_glyph.num as ::core::ffi::c_int {
            *track_glyph.x.offset(i as isize) = 0 as ::core::ffi::c_int;
            *track_glyph.y.offset(i as isize) = 0 as ::core::ffi::c_int;
            i += 1;
        }
        track_glyph.num = 0 as int16_t;
        if square_iswarded(cave, borg.c) {
            *track_glyph.x.offset(track_glyph.num as isize) = borg.c.x;
            *track_glyph.y.offset(track_glyph.num as isize) = borg.c.y;
            track_glyph.num += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The object resists the spell\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"There is no clear floor\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        *track_glyph.x.offset(track_glyph.num as isize) = 0 as ::core::ffi::c_int;
        *track_glyph.y.offset(track_glyph.num as isize) = 0 as ::core::ffi::c_int;
        track_glyph.num -= 1;
        borg_note(
            b"# Removing the Glyph under me, placing with broken door.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
        return;
    }
    if prefix(
        msg,
        b"You have removed the \0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        let mut x: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_int = 0;
        y = borg.c.y - 1 as ::core::ffi::c_int;
        while y < borg.c.y + 1 as ::core::ffi::c_int {
            x = borg.c.x - 1 as ::core::ffi::c_int;
            while x < borg.c.x + 1 as ::core::ffi::c_int {
                if !(*borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0) {
                    if (*ag).feat as ::core::ffi::c_int == FEAT_RUBBLE as ::core::ffi::c_int {
                        (*ag).feat = FEAT_BROKEN as ::core::ffi::c_int as uint8_t;
                    }
                }
                x += 1;
            }
            y += 1;
        }
        return;
    }
    if prefix(
        msg,
        b"The enchantment failed\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        borg.antibounce_count = 1 as int16_t;
        return;
    }
    if prefix(
        msg,
        b"There is a searing blast of light!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            let mut kill_0: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
            x9 = (*kill_0).pos.x;
            y9 = (*kill_0).pos.y;
            if !((*kill_0).r_idx == 0) {
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
                if !(d > 12 as ::core::ffi::c_int) {
                    borg_delete_kill(i);
                }
            }
            i += 1;
        }
        borg_fear_region[(borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(borg.c.x / 11 as ::core::ffi::c_int) as usize] = 0 as uint16_t;
        return;
    }
    if prefix(
        msg,
        b"You hear a door burst open!\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            <= 3 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                <= 5 as ::core::ffi::c_int
        {
            borg.mon.scary = true_0 != 0;
        }
    }
    if prefix(
        msg,
        b"commands you to return.\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || prefix(
            msg,
            b"teleports you away.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || prefix(
            msg,
            b"gestures at your feet.\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        borg.lunal_mode = false_0 != 0;
        borg_note(
            b"# Disconnecting Lunal Mode due to monster spell.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while !prefix_feeling_danger[i as usize].is_null() {
        if prefix(msg, prefix_feeling_danger[i as usize]) {
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"FEELING_DANGER:%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while !suffix_feeling_stuff[i as usize].is_null() {
        if suffix(msg, suffix_feeling_stuff[i as usize]) {
            strnfmt(
                &raw mut buf as *mut ::core::ffi::c_char,
                256 as size_t,
                b"FEELING_STUFF:%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            borg_react(msg, &raw mut buf as *mut ::core::ffi::c_char);
            return;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_parse(mut msg: *mut ::core::ffi::c_char) {
    static mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    if *borg_cfg.offset(BORG_VERBOSE as ::core::ffi::c_int as isize) != 0 && !msg.is_null() {
        borg_note(format(
            b"# Parsing msg <%s>\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ));
    }
    if len != 0
        && (msg.is_null()
            || *msg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        j = i;
        while i < len - 1 as ::core::ffi::c_int {
            if buf[i as usize] as ::core::ffi::c_int == '.' as i32
                || buf[i as usize] as ::core::ffi::c_int == '!' as i32
                || buf[i as usize] as ::core::ffi::c_int == '?' as i32
                || buf[i as usize] as ::core::ffi::c_int == '"' as i32
            {
                if buf[(i + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int == ' ' as i32 {
                    buf[(i + 1 as ::core::ffi::c_int) as usize] =
                        '\0' as i32 as ::core::ffi::c_char;
                    borg_parse_aux(
                        (&raw mut buf as *mut ::core::ffi::c_char).offset(j as isize),
                        i + 1 as ::core::ffi::c_int - j,
                    );
                    buf[(i + 1 as ::core::ffi::c_int) as usize] = ' ' as i32 as ::core::ffi::c_char;
                    j = i + 2 as ::core::ffi::c_int;
                    while buf[j as usize] as ::core::ffi::c_int == ' ' as i32 {
                        j += 1;
                    }
                }
            }
            i += 1;
        }
        borg_parse_aux(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(j as isize),
            len - j,
        );
        len = 0 as ::core::ffi::c_int;
    }
    if msg.is_null() {
        len = 0 as ::core::ffi::c_int;
    } else if *msg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        len = (len as size_t).wrapping_add(strnfmt(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
            (1024 as ::core::ffi::c_int - len) as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        )) as ::core::ffi::c_int as ::core::ffi::c_int;
    } else {
        len = strnfmt(
            &raw mut buf as *mut ::core::ffi::c_char,
            1024 as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        ) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn borg_read_message_equal(
    mut msg1: *mut borg_read_message,
    mut msg2: *mut borg_read_message,
) -> bool {
    if (!(*msg1).message_p1.is_null()
        && !(*msg2).message_p1.is_null()
        && strcmp((*msg1).message_p1, (*msg2).message_p1) == 0
        || (*msg1).message_p1.is_null() && (*msg2).message_p1.is_null())
        && (!(*msg1).message_p2.is_null()
            && !(*msg2).message_p2.is_null()
            && strcmp((*msg1).message_p2, (*msg2).message_p2) == 0
            || (*msg1).message_p2.is_null() && (*msg2).message_p2.is_null())
        && (!(*msg1).message_p3.is_null()
            && !(*msg2).message_p3.is_null()
            && strcmp((*msg1).message_p3, (*msg2).message_p3) == 0
            || (*msg1).message_p3.is_null() && (*msg2).message_p3.is_null())
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn insert_msg(
    mut msgs: *mut borg_read_messages,
    mut msg: *mut borg_read_message,
    mut spell_number: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut found_dup: bool = false_0 != 0;
    if (*msgs).messages.is_null() {
        (*msgs).allocated = 10 as ::core::ffi::c_int;
        (*msgs).messages = mem_alloc(
            (::core::mem::size_of::<borg_read_message>() as size_t)
                .wrapping_mul((*msgs).allocated as size_t),
        ) as *mut borg_read_message;
        (*msgs).index = mem_alloc(
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul((*msgs).allocated as size_t),
        ) as *mut ::core::ffi::c_int;
    }
    if msg.is_null() {
        let ref mut fresh0 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p1;
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh1 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p2;
        *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh2 = (*(*msgs).messages.offset((*msgs).count as isize)).message_p3;
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*msgs).count += 1;
        (*msgs).messages = mem_realloc(
            (*msgs).messages as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<borg_read_message>() as size_t)
                .wrapping_mul((*msgs).count as size_t),
        ) as *mut borg_read_message;
        (*msgs).index = mem_realloc(
            (*msgs).index as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul((*msgs).count as size_t),
        ) as *mut ::core::ffi::c_int;
        (*msgs).allocated = (*msgs).count;
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*msgs).count {
        if borg_read_message_equal(
            (*msgs).messages.offset(i as isize) as *mut borg_read_message,
            msg,
        ) {
            found_dup = true_0 != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found_dup {
        memcpy(
            (*msgs).messages.offset((*msgs).count as isize) as *mut borg_read_message
                as *mut ::core::ffi::c_void,
            msg as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_read_message>() as size_t,
        );
        *(*msgs).index.offset((*msgs).count as isize) = spell_number;
        (*msgs).count += 1;
        if (*msgs).count == (*msgs).allocated {
            (*msgs).allocated += 10 as ::core::ffi::c_int;
            (*msgs).messages = mem_realloc(
                (*msgs).messages as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<borg_read_message>() as size_t)
                    .wrapping_mul((*msgs).allocated as size_t),
            ) as *mut borg_read_message;
            (*msgs).index = mem_realloc(
                (*msgs).index as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                    .wrapping_mul((*msgs).allocated as size_t),
            ) as *mut ::core::ffi::c_int;
        }
    } else {
        string_free((*msg).message_p1);
        string_free((*msg).message_p2);
        string_free((*msg).message_p3);
    };
}
unsafe extern "C" fn clean_msgs(mut msgs: *mut borg_read_messages) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*msgs).count {
        string_free((*(*msgs).messages.offset(i as isize)).message_p1);
        string_free((*(*msgs).messages.offset(i as isize)).message_p2);
        string_free((*(*msgs).messages.offset(i as isize)).message_p3);
        i += 1;
    }
    if !(*msgs).messages.is_null() {
        mem_free((*msgs).messages as *mut ::core::ffi::c_void);
    }
    (*msgs).messages = ::core::ptr::null_mut::<borg_read_message>();
    if !(*msgs).index.is_null() {
        mem_free((*msgs).index as *mut ::core::ffi::c_void);
    }
    (*msgs).index = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*msgs).count = 0 as ::core::ffi::c_int;
    (*msgs).allocated = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn borg_trim_lead_space(
    mut orig: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if orig.is_null()
        || *orig.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
    {
        return orig;
    }
    return orig.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn borg_load_read_message(
    mut message: *mut ::core::ffi::c_char,
    mut read_message: *mut borg_read_message,
) {
    (*read_message).message_p1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*read_message).message_p2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*read_message).message_p3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix_0: *mut ::core::ffi::c_char = strchr(message, '}' as i32);
    if suffix_0.is_null() {
        (*read_message).message_p1 = string_make(borg_trim_lead_space(message));
        return;
    }
    if *message.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '{' as i32 {
        suffix_0 = suffix_0.offset(1);
    } else {
        suffix_0 = message;
    }
    let mut var: *mut ::core::ffi::c_char = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        (*read_message).message_p1 = string_make(borg_trim_lead_space(suffix_0));
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    let mut part_len: ::core::ffi::c_int =
        strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    (*read_message).message_p1 = string_make(format(
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        part_len,
        suffix_0,
    ));
    suffix_0 = suffix_0.offset(part_len as isize);
    suffix_0 = strchr(var, '}' as i32);
    if suffix_0.is_null() {
        return;
    }
    suffix_0 = suffix_0.offset(1);
    var = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            suffix_0 = suffix_0.offset(1);
        }
        if strlen(suffix_0) != 0
            && strcmp(suffix_0, b".\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            (*read_message).message_p2 = string_make(suffix_0);
        }
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    part_len = strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    if part_len != 0 {
        (*read_message).message_p2 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    }
    suffix_0 = suffix_0.offset(part_len as isize);
    suffix_0 = strchr(var, '}' as i32);
    if suffix_0.is_null() {
        return;
    }
    suffix_0 = suffix_0.offset(1);
    var = strchr(suffix_0, '{' as i32);
    if var.is_null() {
        while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            suffix_0 = suffix_0.offset(1);
        }
        if strlen(suffix_0) != 0
            && strcmp(suffix_0, b".\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            if !(*read_message).message_p2.is_null() {
                (*read_message).message_p3 = string_make(borg_trim_lead_space(suffix_0));
            } else {
                (*read_message).message_p2 = string_make(borg_trim_lead_space(suffix_0));
            }
        }
        return;
    }
    while *suffix_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
        suffix_0 = suffix_0.offset(1);
    }
    part_len = strlen(suffix_0).wrapping_sub(strlen(var)) as ::core::ffi::c_int;
    if !(*read_message).message_p2.is_null() {
        (*read_message).message_p3 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    } else {
        (*read_message).message_p2 = string_make(borg_trim_lead_space(format(
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            part_len,
            suffix_0,
        )));
    };
}
unsafe extern "C" fn borg_init_spell_messages() {
    let mut spell: *const monster_spell = monster_spells;
    let mut spell_level: *const monster_spell_level = ::core::ptr::null::<monster_spell_level>();
    let mut read_message: borg_read_message = borg_read_message {
        message_p1: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p2: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p3: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    while !spell.is_null() {
        spell_level = (*spell).level;
        while !spell_level.is_null() {
            if !(*spell_level).blind_message.is_null() {
                borg_load_read_message((*spell_level).blind_message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_invis_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            if !(*spell_level).message.is_null() {
                borg_load_read_message((*spell_level).message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            if !(*spell_level).miss_message.is_null() {
                borg_load_read_message((*spell_level).miss_message, &raw mut read_message);
                insert_msg(
                    &raw mut spell_msgs,
                    &raw mut read_message,
                    (*spell).index as ::core::ffi::c_int,
                );
            }
            spell_level = (*spell_level).next;
        }
        spell = (*spell).next;
    }
    insert_msg(
        &raw mut spell_invis_msgs,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
    insert_msg(
        &raw mut spell_msgs,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
}
pub const MSG_PARSE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSG_PARSE_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MSG_PARSE_PLURAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn borg_get_parsed_pain(
    mut pain: *const ::core::ffi::c_char,
    mut do_plural: bool,
) -> *mut ::core::ffi::c_char {
    let mut buflen: size_t = strlen(pain).wrapping_add(1 as size_t);
    let mut buf: *mut ::core::ffi::c_char = mem_zalloc(buflen) as *mut ::core::ffi::c_char;
    let mut state: ::core::ffi::c_int = MSG_PARSE_NORMAL;
    let mut maxlen: size_t = strlen(pain);
    let mut pos: size_t = 1 as size_t;
    *buf.offset(0 as ::core::ffi::c_int as isize) = ' ' as i32 as ::core::ffi::c_char;
    let mut i: size_t = 0 as size_t;
    while i < maxlen && pos < buflen.wrapping_sub(1 as size_t) {
        let mut cur: ::core::ffi::c_char = *pain.offset(i as isize);
        if state == MSG_PARSE_NORMAL && cur as ::core::ffi::c_int == '[' as i32 {
            state = MSG_PARSE_SINGLE;
        } else if state == MSG_PARSE_SINGLE && cur as ::core::ffi::c_int == '|' as i32 {
            state = MSG_PARSE_PLURAL;
        } else if state != MSG_PARSE_NORMAL && cur as ::core::ffi::c_int == ']' as i32 {
            state = MSG_PARSE_NORMAL;
        } else if state == MSG_PARSE_NORMAL
            || state == MSG_PARSE_SINGLE && do_plural as ::core::ffi::c_int == false_0
            || state == MSG_PARSE_PLURAL && do_plural as ::core::ffi::c_int == true_0
        {
            let fresh8 = pos;
            pos = pos.wrapping_add(1);
            *buf.offset(fresh8 as isize) = cur;
        }
        i = i.wrapping_add(1);
    }
    return buf;
}
unsafe extern "C" fn borg_insert_pain(
    mut pain: *const ::core::ffi::c_char,
    mut capacity: *mut ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) {
    let mut new_message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *capacity <= *count + 2 as ::core::ffi::c_int {
        *capacity += 14 as ::core::ffi::c_int;
        suffix_pain = mem_realloc(
            suffix_pain as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(*capacity as size_t),
        ) as *mut *mut ::core::ffi::c_char;
    }
    new_message = borg_get_parsed_pain(pain, false_0 != 0);
    let fresh4 = *count;
    *count = *count + 1;
    let ref mut fresh5 = *suffix_pain.offset(fresh4 as isize);
    *fresh5 = new_message;
    new_message = borg_get_parsed_pain(pain, true_0 != 0);
    let fresh6 = *count;
    *count = *count + 1;
    let ref mut fresh7 = *suffix_pain.offset(fresh6 as isize);
    *fresh7 = new_message;
}
static mut borg_msg_repository: [C2RustUnnamed_9; 63] = [
    C2RustUnnamed_9 {
        msg: b"[is|are] hurt.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"die[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] destroyed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s] a lot.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] hit hard.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] immune.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"resist[s] somewhat.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] unaffected!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"spawn[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] healthier.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"fall[s] asleep!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"wake[s] up.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"cringe[s] from the light!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shrivel[s] away in the light!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"lose[s] some skin!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"dissolve[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"catch[es] fire!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] badly frozen.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shudder[s].\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"change[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"disappear[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] even more stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer stunned.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer confused.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more slowed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] slowed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"speed[s] up.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] even faster!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"start[s] moving faster.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"slow[s] down.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] more terrified!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"flee[s] in terror!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_FLEE as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] no longer afraid.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] frozen to the spot!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"can move again.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"seem[s] less magical!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"seem[s] magical again.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"falls under your spell!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"is no longer under your control.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"shimmers for a moment.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"You hear [a|several] scream[|s] of agony!\0" as *const u8
            as *const ::core::ffi::c_char,
        omit_subject: 1 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"disintegrate[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"freeze[s] and shatter[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"lose[s] some mana!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"look[s] briefly puzzled.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"maintain[s] the same shape.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] unharmed.\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"appear[s]!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"There is a puff of smoke!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 1 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"[is|are] embedded in rock!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_KILL as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"wail[s] out in pain!\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
    C2RustUnnamed_9 {
        msg: b"\0" as *const u8 as *const ::core::ffi::c_char,
        omit_subject: 0 as ::core::ffi::c_int != 0,
        type_0: MSG_GENERIC as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn borg_init_pain_messages() {
    let mut capacity: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut pain: *mut monster_pain = ::core::ptr::null_mut::<monster_pain>();
    suffix_pain = mem_alloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul(capacity as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    idx = 0 as ::core::ffi::c_int;
    while idx < (*z_info).mp_max as ::core::ffi::c_int {
        pain = pain_messages.offset(idx as isize) as *mut monster_pain;
        i = 0 as ::core::ffi::c_int;
        while i < 7 as ::core::ffi::c_int {
            if pain.is_null() || (*pain).messages[i as usize].is_null() {
                break;
            }
            borg_insert_pain(
                (*pain).messages[i as usize],
                &raw mut capacity,
                &raw mut count,
            );
            i += 1;
        }
        idx += 1;
    }
    idx = 0 as ::core::ffi::c_int;
    while idx < MON_MSG_MAX as ::core::ffi::c_int {
        if !(borg_msg_repository[idx as usize].type_0 == MSG_KILL as ::core::ffi::c_int) {
            let mut std_pain: *const ::core::ffi::c_char = borg_msg_repository[idx as usize].msg;
            match idx {
                21 | 55 | 56 | 57 | 58 | 59 | 60 | 61 => {}
                _ => {
                    if !std_pain.is_null() {
                        borg_insert_pain(std_pain, &raw mut capacity, &raw mut count);
                    }
                }
            }
        }
        idx += 1;
    }
    if count + 1 as ::core::ffi::c_int != capacity {
        suffix_pain = mem_realloc(
            suffix_pain as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul((count + 1 as ::core::ffi::c_int) as size_t),
        ) as *mut *mut ::core::ffi::c_char;
    }
    let ref mut fresh3 = *suffix_pain.offset(count as isize);
    *fresh3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn borg_init_hit_by_messages() {
    let mut read_message: borg_read_message = borg_read_message {
        message_p1: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p2: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        message_p3: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).blow_methods_max as ::core::ffi::c_int {
        let mut messages: *mut blow_message = (*blow_methods.offset(i as isize)).messages;
        while !messages.is_null() {
            if !(*messages).act_msg.is_null() {
                borg_load_read_message((*messages).act_msg, &raw mut read_message);
                insert_msg(
                    &raw mut suffix_hit_by,
                    &raw mut read_message,
                    (*blow_methods.offset(i as isize)).msgt,
                );
            }
            messages = (*messages).next;
        }
        i += 1;
    }
    insert_msg(
        &raw mut suffix_hit_by,
        ::core::ptr::null_mut::<borg_read_message>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_messages() {
    borg_free_messages();
    borg_init_spell_messages();
    borg_init_pain_messages();
    borg_init_hit_by_messages();
    borg_msg_len = 0 as int16_t;
    borg_msg_siz = 4096 as int16_t;
    borg_msg_buf = mem_zalloc(
        (borg_msg_siz as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    borg_msg_num = 0 as int16_t;
    borg_msg_max = 256 as int16_t;
    borg_msg_pos = mem_zalloc(
        (borg_msg_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    borg_msg_use = mem_zalloc(
        (borg_msg_max as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_messages() {
    let mut i: ::core::ffi::c_int = 0;
    if !borg_msg_buf.is_null() {
        mem_free(borg_msg_buf as *mut ::core::ffi::c_void);
        borg_msg_buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !borg_msg_use.is_null() {
        mem_free(borg_msg_use as *mut ::core::ffi::c_void);
        borg_msg_use = ::core::ptr::null_mut::<int16_t>();
    }
    if !borg_msg_pos.is_null() {
        mem_free(borg_msg_pos as *mut ::core::ffi::c_void);
        borg_msg_pos = ::core::ptr::null_mut::<int16_t>();
    }
    borg_msg_num = 0 as int16_t;
    if !borg_msg_buf.is_null() {
        mem_free(borg_msg_buf as *mut ::core::ffi::c_void);
        borg_msg_buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    borg_msg_siz = 0 as int16_t;
    if !suffix_pain.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*suffix_pain.offset(i as isize)).is_null() {
            mem_free(*suffix_pain.offset(i as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh9 = *suffix_pain.offset(i as isize);
            *fresh9 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            i += 1;
        }
        mem_free(suffix_pain as *mut ::core::ffi::c_void);
        suffix_pain = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    clean_msgs(&raw mut suffix_hit_by);
    clean_msgs(&raw mut spell_invis_msgs);
    clean_msgs(&raw mut spell_msgs);
}
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
