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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strnfmt(
        buf: *mut ::core::ffi::c_char,
        max: size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> size_t;
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn flag_has(flags: *const bitflag, size: size_t, flag: ::core::ffi::c_int) -> bool;
    fn my_strcpy(
        buf: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        bufsize: size_t,
    ) -> size_t;
    fn Rand_div(m: uint32_t) -> uint32_t;
    fn loc(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> loc;
    fn event_signal(_: game_event_type);
    fn msg(fmt: *const ::core::ffi::c_char, ...);
    static mut k_info: *mut object_kind;
    static mut a_info: *mut artifact;
    static mut player: *mut player;
    static mut z_info: *mut angband_constants;
    static ddx_ddd: [int16_t; 9];
    static ddy_ddd: [int16_t; 9];
    static mut f_info: *mut feature;
    static mut cave: *mut chunk;
    fn square_in_bounds(c: *mut chunk, grid: loc) -> bool;
    fn square(c: *mut chunk, grid: loc) -> *const square;
    fn target_okay() -> bool;
    fn target_get(grid: *mut loc);
    static mut borg_items: *mut borg_item;
    fn borg_find_all_items();
    static mut weapon_swap: ::core::ffi::c_int;
    static mut armour_swap: ::core::ffi::c_int;
    static mut weapon_swap_value: int32_t;
    static mut armour_swap_value: int32_t;
    static mut prefix_pref: [*const ::core::ffi::c_char; 0];
    fn borg_notice(notice_swap: bool);
    static mut borg_engine_date: [::core::ffi::c_char; 0];
    static mut borg_cfg: *mut ::core::ffi::c_int;
    static mut borg: borg_struct;
    static mut borg_step: uint16_t;
    static mut w_x: ::core::ffi::c_int;
    static mut w_y: ::core::ffi::c_int;
    fn borg_update_entrypoint(start: bool);
    fn get_string(
        prompt: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> bool;
    fn get_quantity(
        prompt: *const ::core::ffi::c_char,
        max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn get_com(prompt: *const ::core::ffi::c_char, command: *mut ::core::ffi::c_char) -> bool;
    fn source_player() -> source;
    fn project(
        origin: source,
        rad: ::core::ffi::c_int,
        finish: loc,
        dam: ::core::ffi::c_int,
        typ: ::core::ffi::c_int,
        flg: ::core::ffi::c_int,
        degrees_of_arc: ::core::ffi::c_int,
        diameter_of_source: uint8_t,
        obj: *const object,
    ) -> bool;
    static mut row_top_map: [::core::ffi::c_int; 3];
    static mut row_bottom_map: [::core::ffi::c_int; 3];
    static mut col_map: [::core::ffi::c_int; 3];
    static mut Term: *mut term;
    static mut tile_width: uint8_t;
    static mut tile_height: uint8_t;
    fn Term_fresh() -> errr;
    fn Term_putstr(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        a: ::core::ffi::c_int,
        s: *const ::core::ffi::c_char,
    ) -> errr;
    fn Term_erase(x: ::core::ffi::c_int, y: ::core::ffi::c_int, n: ::core::ffi::c_int) -> errr;
    fn Term_clear() -> errr;
    fn Term_flush() -> errr;
    fn Term_save() -> errr;
    fn Term_load() -> errr;
    fn get_com_ex(prompt: *const ::core::ffi::c_char, command: *mut ui_event) -> bool;
    fn print_rel(c: wchar_t, a: uint8_t, y: ::core::ffi::c_int, x: ::core::ffi::c_int);
    fn prt_map();
    fn c_put_str(
        attr: uint8_t,
        str: *const ::core::ffi::c_char,
        row: ::core::ffi::c_int,
        col: ::core::ffi::c_int,
    );
    fn screen_save();
    fn screen_load();
    fn window_make(
        origin_x: ::core::ffi::c_int,
        origin_y: ::core::ffi::c_int,
        end_x: ::core::ffi::c_int,
        end_y: ::core::ffi::c_int,
    );
    fn menu_new(id: skin_id, iter: *const menu_iter) -> *mut menu;
    fn menu_setpriv(menu: *mut menu, count: ::core::ffi::c_int, data: *mut ::core::ffi::c_void);
    fn menu_layout(menu: *mut menu, loc_0: *const region) -> bool;
    fn menu_select(menu: *mut menu, notify: ::core::ffi::c_int, popup: bool) -> ui_event;
    fn target_set_interactive(
        mode: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        allow_pathfinding: bool,
    ) -> bool;
    static mut borg_grids: [*mut borg_grid; 66];
    fn borg_cave_floor_bold(y: ::core::ffi::c_int, X: ::core::ffi::c_int) -> bool;
    fn borg_panel_hgt() -> ::core::ffi::c_int;
    fn borg_panel_wid() -> ::core::ffi::c_int;
    static mut borg_fear_region: [[uint16_t; 19]; 7];
    static mut borg_fear_monsters: [[uint16_t; 199]; 67];
    fn borg_danger(
        y: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        average: bool,
        full_damage: bool,
    ) -> ::core::ffi::c_int;
    static mut borg_flow_y: [uint8_t; 1536];
    static mut borg_flow_x: [uint8_t; 1536];
    static mut borg_data_flow: *mut borg_data;
    static mut track_step: borg_track;
    static mut track_glyph: borg_track;
    static mut borg_kills_nxt: int16_t;
    static mut borg_kills: *mut borg_kill;
    static mut borg_nasties_num: uint8_t;
    static mut borg_nasties_count: [uint8_t; 7];
    static mut borg_nasties: [::core::ffi::c_char; 7];
    static mut borg_nasties_limit: [uint8_t; 7];
    static mut borg_takes_nxt: int16_t;
    static mut borg_takes: *mut borg_take;
    fn borg_notice_home(in_item: *mut borg_item, no_items: bool);
    fn borg_power_home() -> int32_t;
    static mut borg_init_failure: bool;
    static mut borg_initialized: bool;
    static mut game_closed: bool;
    static mut borg_settings: [borg_setting; 0];
    fn borg_init_txt_file() -> bool;
    fn borg_reinit_options();
    fn borg_init();
    fn borg_free();
    fn borg_inventory();
    fn borg_note(what: *const ::core::ffi::c_char);
    fn borg_object_fully_id() -> bool;
    fn borg_clear_best();
    fn borg_write_map(ask: bool);
    fn borg_display_item(item2: *mut object, n: ::core::ffi::c_int);
    static mut borg_magics: *mut borg_magic;
    fn borg_can_cast() -> bool;
    fn borg_spell_legal(spell: borg_spells) -> bool;
    fn borg_spell_fail_rate(spell: borg_spells) -> ::core::ffi::c_int;
    static mut borg_match: [::core::ffi::c_char; 128];
    fn borg_power() -> int32_t;
    fn borg_los(
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
    fn borg_prepared(depth: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn borg_force_reincarnate();
    fn borg_update();
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
pub const STAT_MAX: C2RustUnnamed = 5;
pub const STAT_CON: C2RustUnnamed = 4;
pub const STAT_DEX: C2RustUnnamed = 3;
pub const STAT_WIS: C2RustUnnamed = 2;
pub const STAT_INT: C2RustUnnamed = 1;
pub const STAT_STR: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SQUARE_MAX: C2RustUnnamed_0 = 22;
pub const SQUARE_CLOSE_PLAYER: C2RustUnnamed_0 = 21;
pub const SQUARE_NO_STAIRS: C2RustUnnamed_0 = 20;
pub const SQUARE_DTRAP: C2RustUnnamed_0 = 19;
pub const SQUARE_PROJECT: C2RustUnnamed_0 = 18;
pub const SQUARE_NO_ESP: C2RustUnnamed_0 = 17;
pub const SQUARE_NO_MAP: C2RustUnnamed_0 = 16;
pub const SQUARE_NO_TELEPORT: C2RustUnnamed_0 = 15;
pub const SQUARE_MON_RESTRICT: C2RustUnnamed_0 = 14;
pub const SQUARE_WALL_SOLID: C2RustUnnamed_0 = 13;
pub const SQUARE_WALL_OUTER: C2RustUnnamed_0 = 12;
pub const SQUARE_WALL_INNER: C2RustUnnamed_0 = 11;
pub const SQUARE_INVIS: C2RustUnnamed_0 = 10;
pub const SQUARE_TRAP: C2RustUnnamed_0 = 9;
pub const SQUARE_FEEL: C2RustUnnamed_0 = 8;
pub const SQUARE_WASSEEN: C2RustUnnamed_0 = 7;
pub const SQUARE_VIEW: C2RustUnnamed_0 = 6;
pub const SQUARE_SEEN: C2RustUnnamed_0 = 5;
pub const SQUARE_ROOM: C2RustUnnamed_0 = 4;
pub const SQUARE_VAULT: C2RustUnnamed_0 = 3;
pub const SQUARE_GLOW: C2RustUnnamed_0 = 2;
pub const SQUARE_MARK: C2RustUnnamed_0 = 1;
pub const SQUARE_NONE: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FEAT_MAX: C2RustUnnamed_1 = 25;
pub const FEAT_PASS_RUBBLE: C2RustUnnamed_1 = 24;
pub const FEAT_LAVA: C2RustUnnamed_1 = 23;
pub const FEAT_PERM: C2RustUnnamed_1 = 22;
pub const FEAT_GRANITE: C2RustUnnamed_1 = 21;
pub const FEAT_QUARTZ_K: C2RustUnnamed_1 = 20;
pub const FEAT_MAGMA_K: C2RustUnnamed_1 = 19;
pub const FEAT_QUARTZ: C2RustUnnamed_1 = 18;
pub const FEAT_MAGMA: C2RustUnnamed_1 = 17;
pub const FEAT_RUBBLE: C2RustUnnamed_1 = 16;
pub const FEAT_SECRET: C2RustUnnamed_1 = 15;
pub const FEAT_HOME: C2RustUnnamed_1 = 14;
pub const FEAT_STORE_BLACK: C2RustUnnamed_1 = 13;
pub const FEAT_STORE_MAGIC: C2RustUnnamed_1 = 12;
pub const FEAT_STORE_ALCHEMY: C2RustUnnamed_1 = 11;
pub const FEAT_STORE_BOOK: C2RustUnnamed_1 = 10;
pub const FEAT_STORE_WEAPON: C2RustUnnamed_1 = 9;
pub const FEAT_STORE_ARMOR: C2RustUnnamed_1 = 8;
pub const FEAT_STORE_GENERAL: C2RustUnnamed_1 = 7;
pub const FEAT_MORE: C2RustUnnamed_1 = 6;
pub const FEAT_LESS: C2RustUnnamed_1 = 5;
pub const FEAT_BROKEN: C2RustUnnamed_1 = 4;
pub const FEAT_OPEN: C2RustUnnamed_1 = 3;
pub const FEAT_CLOSED: C2RustUnnamed_1 = 2;
pub const FEAT_FLOOR: C2RustUnnamed_1 = 1;
pub const FEAT_NONE: C2RustUnnamed_1 = 0;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const BI_MAX: C2RustUnnamed_2 = 265;
pub const BI_PREP_BIG_FIGHT: C2RustUnnamed_2 = 264;
pub const BI_SAURON_DEAD: C2RustUnnamed_2 = 263;
pub const BI_EMPTY: C2RustUnnamed_2 = 262;
pub const BI_CARRY: C2RustUnnamed_2 = 261;
pub const BI_WEIGHT: C2RustUnnamed_2 = 260;
pub const BI_DINV: C2RustUnnamed_2 = 259;
pub const BI_MULTIPLE_BONUSES: C2RustUnnamed_2 = 258;
pub const BI_GOOD_W_CHG: C2RustUnnamed_2 = 257;
pub const BI_GOOD_S_CHG: C2RustUnnamed_2 = 256;
pub const BI_ADIGGER: C2RustUnnamed_2 = 255;
pub const BI_ALL_NEED_ID: C2RustUnnamed_2 = 254;
pub const BI_WORN_NEED_ID: C2RustUnnamed_2 = 253;
pub const BI_AROD2: C2RustUnnamed_2 = 252;
pub const BI_AROD1: C2RustUnnamed_2 = 251;
pub const BI_ASHROOM: C2RustUnnamed_2 = 250;
pub const BI_AMASSBAN: C2RustUnnamed_2 = 249;
pub const BI_AHWORD: C2RustUnnamed_2 = 248;
pub const BI_ATELEPORTLVL: C2RustUnnamed_2 = 247;
pub const BI_ARESPOIS: C2RustUnnamed_2 = 246;
pub const BI_ARESCOLD: C2RustUnnamed_2 = 245;
pub const BI_ARESHEAT: C2RustUnnamed_2 = 244;
pub const BI_NEED_BRAND_WEAPON: C2RustUnnamed_2 = 243;
pub const BI_NEED_ENCHANT_TO_D: C2RustUnnamed_2 = 242;
pub const BI_NEED_ENCHANT_TO_H: C2RustUnnamed_2 = 241;
pub const BI_NEED_ENCHANT_TO_A: C2RustUnnamed_2 = 240;
pub const BI_ABRAND: C2RustUnnamed_2 = 239;
pub const BI_AENCH_SARM: C2RustUnnamed_2 = 238;
pub const BI_AENCH_ARM: C2RustUnnamed_2 = 237;
pub const BI_AENCH_SWEP: C2RustUnnamed_2 = 236;
pub const BI_AENCH_TOD: C2RustUnnamed_2 = 235;
pub const BI_AENCH_TOH: C2RustUnnamed_2 = 234;
pub const BI_ACLW: C2RustUnnamed_2 = 233;
pub const BI_ACSW: C2RustUnnamed_2 = 232;
pub const BI_ACCW: C2RustUnnamed_2 = 231;
pub const BI_AGLYPH: C2RustUnnamed_2 = 230;
pub const BI_APFE: C2RustUnnamed_2 = 229;
pub const BI_ALITE: C2RustUnnamed_2 = 228;
pub const BI_ARECHARGE: C2RustUnnamed_2 = 227;
pub const BI_AMAGICMAP: C2RustUnnamed_2 = 226;
pub const BI_ADETEVIL: C2RustUnnamed_2 = 225;
pub const BI_ADETDOOR: C2RustUnnamed_2 = 224;
pub const BI_ADETTRAP: C2RustUnnamed_2 = 223;
pub const BI_ACUREPOIS: C2RustUnnamed_2 = 222;
pub const BI_ATPORTOTHER: C2RustUnnamed_2 = 221;
pub const BI_ASTFDEST: C2RustUnnamed_2 = 220;
pub const BI_ASTFMAGI: C2RustUnnamed_2 = 219;
pub const BI_ASPEED: C2RustUnnamed_2 = 218;
pub const BI_AID: C2RustUnnamed_2 = 217;
pub const BI_ALIFE: C2RustUnnamed_2 = 216;
pub const BI_AEZHEAL: C2RustUnnamed_2 = 215;
pub const BI_AHEAL: C2RustUnnamed_2 = 214;
pub const BI_AFUEL: C2RustUnnamed_2 = 213;
pub const BI_AESCAPE: C2RustUnnamed_2 = 212;
pub const BI_ATELEPORT: C2RustUnnamed_2 = 211;
pub const BI_APHASE: C2RustUnnamed_2 = 210;
pub const BI_WB_POIS: C2RustUnnamed_2 = 209;
pub const BI_WB_COLD: C2RustUnnamed_2 = 208;
pub const BI_WB_FIRE: C2RustUnnamed_2 = 207;
pub const BI_WB_ELEC: C2RustUnnamed_2 = 206;
pub const BI_WB_ACID: C2RustUnnamed_2 = 205;
pub const BI_W_IMPACT: C2RustUnnamed_2 = 204;
pub const BI_WK_DRAGON: C2RustUnnamed_2 = 203;
pub const BI_WK_DEMON: C2RustUnnamed_2 = 202;
pub const BI_WK_UNDEAD: C2RustUnnamed_2 = 201;
pub const BI_WS_DRAGON: C2RustUnnamed_2 = 200;
pub const BI_WS_GIANT: C2RustUnnamed_2 = 199;
pub const BI_WS_TROLL: C2RustUnnamed_2 = 198;
pub const BI_WS_ORC: C2RustUnnamed_2 = 197;
pub const BI_WS_DEMON: C2RustUnnamed_2 = 196;
pub const BI_WS_UNDEAD: C2RustUnnamed_2 = 195;
pub const BI_WS_EVIL: C2RustUnnamed_2 = 194;
pub const BI_WS_ANIMAL: C2RustUnnamed_2 = 193;
pub const BI_CRSUNKNO: C2RustUnnamed_2 = 192;
pub const BI_CRSAVULN: C2RustUnnamed_2 = 191;
pub const BI_CRSCVULN: C2RustUnnamed_2 = 190;
pub const BI_CRSEVULN: C2RustUnnamed_2 = 189;
pub const BI_CRSFVULN: C2RustUnnamed_2 = 188;
pub const BI_CRSDRAIN_XP: C2RustUnnamed_2 = 187;
pub const BI_CRSFEAR: C2RustUnnamed_2 = 186;
pub const BI_CRSAIRSWING: C2RustUnnamed_2 = 185;
pub const BI_CRSSTEELSKIN: C2RustUnnamed_2 = 184;
pub const BI_CRSMPIMP: C2RustUnnamed_2 = 183;
pub const BI_CRSHPIMP: C2RustUnnamed_2 = 182;
pub const BI_CRSANNOY: C2RustUnnamed_2 = 181;
pub const BI_CRSSLOW: C2RustUnnamed_2 = 180;
pub const BI_CRSCLUM: C2RustUnnamed_2 = 179;
pub const BI_CRSWEAK: C2RustUnnamed_2 = 178;
pub const BI_CRSSICK: C2RustUnnamed_2 = 177;
pub const BI_CRSDULL: C2RustUnnamed_2 = 176;
pub const BI_CRSVULN: C2RustUnnamed_2 = 175;
pub const BI_CRSAGRV: C2RustUnnamed_2 = 174;
pub const BI_CRSTWEP: C2RustUnnamed_2 = 173;
pub const BI_CRSNOTEL: C2RustUnnamed_2 = 172;
pub const BI_CRSSTONE: C2RustUnnamed_2 = 171;
pub const BI_CRSSUND: C2RustUnnamed_2 = 170;
pub const BI_CRSSDRA: C2RustUnnamed_2 = 169;
pub const BI_CRSSDEM: C2RustUnnamed_2 = 168;
pub const BI_CRSPARA: C2RustUnnamed_2 = 167;
pub const BI_CRSHALU: C2RustUnnamed_2 = 166;
pub const BI_CRSSIREN: C2RustUnnamed_2 = 165;
pub const BI_CRSPOIS: C2RustUnnamed_2 = 164;
pub const BI_CRSTELE: C2RustUnnamed_2 = 163;
pub const BI_CRSIRRITATION: C2RustUnnamed_2 = 162;
pub const BI_CRSENVELOPING: C2RustUnnamed_2 = 161;
pub const BI_WHERE_CURSED: C2RustUnnamed_2 = 160;
pub const BI_FIRST_CURSED: C2RustUnnamed_2 = 159;
pub const BI_QUIVER_SLOTS: C2RustUnnamed_2 = 158;
pub const BI_AMISSILES_CURSED: C2RustUnnamed_2 = 157;
pub const BI_AMISSILES_SPECIAL: C2RustUnnamed_2 = 156;
pub const BI_AMISSILES: C2RustUnnamed_2 = 155;
pub const BI_AMMO_POWER: C2RustUnnamed_2 = 154;
pub const BI_AMMO_SIDES: C2RustUnnamed_2 = 153;
pub const BI_AMMO_TVAL: C2RustUnnamed_2 = 152;
pub const BI_AMMO_COUNT: C2RustUnnamed_2 = 151;
pub const BI_HEAVYBOW: C2RustUnnamed_2 = 150;
pub const BI_HEAVYWEPON: C2RustUnnamed_2 = 149;
pub const BI_SHOTS: C2RustUnnamed_2 = 148;
pub const BI_EXTRA_BLOWS: C2RustUnnamed_2 = 147;
pub const BI_BLOWS: C2RustUnnamed_2 = 146;
pub const BI_BART: C2RustUnnamed_2 = 145;
pub const BI_SLING: C2RustUnnamed_2 = 144;
pub const BI_BTODAM: C2RustUnnamed_2 = 143;
pub const BI_BTOHIT: C2RustUnnamed_2 = 142;
pub const BI_BID: C2RustUnnamed_2 = 141;
pub const BI_WDS: C2RustUnnamed_2 = 140;
pub const BI_WDD: C2RustUnnamed_2 = 139;
pub const BI_WID: C2RustUnnamed_2 = 138;
pub const BI_WTODAM: C2RustUnnamed_2 = 137;
pub const BI_WTOHIT: C2RustUnnamed_2 = 136;
pub const BI_TODAM: C2RustUnnamed_2 = 135;
pub const BI_TOHIT: C2RustUnnamed_2 = 134;
pub const BI_ARMOR: C2RustUnnamed_2 = 133;
pub const BI_ISFIXALL: C2RustUnnamed_2 = 132;
pub const BI_ISFIXCON: C2RustUnnamed_2 = 131;
pub const BI_ISFIXDEX: C2RustUnnamed_2 = 130;
pub const BI_ISFIXWIS: C2RustUnnamed_2 = 129;
pub const BI_ISFIXINT: C2RustUnnamed_2 = 128;
pub const BI_ISFIXSTR: C2RustUnnamed_2 = 127;
pub const BI_HASFIXEXP: C2RustUnnamed_2 = 126;
pub const BI_ISFIXEXP: C2RustUnnamed_2 = 125;
pub const BI_ISFIXLEV: C2RustUnnamed_2 = 124;
pub const BI_ISSTUDY: C2RustUnnamed_2 = 123;
pub const BI_ISENCUMB: C2RustUnnamed_2 = 122;
pub const BI_ISFORGET: C2RustUnnamed_2 = 121;
pub const BI_ISIMAGE: C2RustUnnamed_2 = 120;
pub const BI_ISPARALYZED: C2RustUnnamed_2 = 119;
pub const BI_ISHEAVYSTUN: C2RustUnnamed_2 = 118;
pub const BI_ISSTUN: C2RustUnnamed_2 = 117;
pub const BI_ISCUT: C2RustUnnamed_2 = 116;
pub const BI_ISPOISONED: C2RustUnnamed_2 = 115;
pub const BI_ISCONFUSED: C2RustUnnamed_2 = 114;
pub const BI_ISAFRAID: C2RustUnnamed_2 = 113;
pub const BI_ISBLIND: C2RustUnnamed_2 = 112;
pub const BI_ISGORGED: C2RustUnnamed_2 = 111;
pub const BI_ISFULL: C2RustUnnamed_2 = 110;
pub const BI_ISHUNGRY: C2RustUnnamed_2 = 109;
pub const BI_ISWEAK: C2RustUnnamed_2 = 108;
pub const BI_KING: C2RustUnnamed_2 = 107;
pub const BI_MAXDEPTH: C2RustUnnamed_2 = 106;
pub const BI_CDEPTH: C2RustUnnamed_2 = 105;
pub const BI_SFRACT: C2RustUnnamed_2 = 104;
pub const BI_SHLIFE: C2RustUnnamed_2 = 103;
pub const BI_SRDIS: C2RustUnnamed_2 = 102;
pub const BI_SRKAOS: C2RustUnnamed_2 = 101;
pub const BI_SRNTHR: C2RustUnnamed_2 = 100;
pub const BI_SRNXUS: C2RustUnnamed_2 = 99;
pub const BI_SRSHRD: C2RustUnnamed_2 = 98;
pub const BI_SRSND: C2RustUnnamed_2 = 97;
pub const BI_SRCONF: C2RustUnnamed_2 = 96;
pub const BI_SRBLIND: C2RustUnnamed_2 = 95;
pub const BI_SRDARK: C2RustUnnamed_2 = 94;
pub const BI_SRLITE: C2RustUnnamed_2 = 93;
pub const BI_SRFEAR: C2RustUnnamed_2 = 92;
pub const BI_SRPOIS: C2RustUnnamed_2 = 91;
pub const BI_SRACID: C2RustUnnamed_2 = 90;
pub const BI_SRELEC: C2RustUnnamed_2 = 89;
pub const BI_SRCOLD: C2RustUnnamed_2 = 88;
pub const BI_SRFIRE: C2RustUnnamed_2 = 87;
pub const BI_FRACT: C2RustUnnamed_2 = 86;
pub const BI_HLIFE: C2RustUnnamed_2 = 85;
pub const BI_RDIS: C2RustUnnamed_2 = 84;
pub const BI_RKAOS: C2RustUnnamed_2 = 83;
pub const BI_RNTHR: C2RustUnnamed_2 = 82;
pub const BI_RNXUS: C2RustUnnamed_2 = 81;
pub const BI_RSHRD: C2RustUnnamed_2 = 80;
pub const BI_RSND: C2RustUnnamed_2 = 79;
pub const BI_RCONF: C2RustUnnamed_2 = 78;
pub const BI_RBLIND: C2RustUnnamed_2 = 77;
pub const BI_RDARK: C2RustUnnamed_2 = 76;
pub const BI_RLITE: C2RustUnnamed_2 = 75;
pub const BI_RFEAR: C2RustUnnamed_2 = 74;
pub const BI_RPOIS: C2RustUnnamed_2 = 73;
pub const BI_RACID: C2RustUnnamed_2 = 72;
pub const BI_RELEC: C2RustUnnamed_2 = 71;
pub const BI_RCOLD: C2RustUnnamed_2 = 70;
pub const BI_RFIRE: C2RustUnnamed_2 = 69;
pub const BI_IPOIS: C2RustUnnamed_2 = 68;
pub const BI_IELEC: C2RustUnnamed_2 = 67;
pub const BI_ICOLD: C2RustUnnamed_2 = 66;
pub const BI_IACID: C2RustUnnamed_2 = 65;
pub const BI_IFIRE: C2RustUnnamed_2 = 64;
pub const BI_DIG: C2RustUnnamed_2 = 63;
pub const BI_THT: C2RustUnnamed_2 = 62;
pub const BI_THB: C2RustUnnamed_2 = 61;
pub const BI_THN: C2RustUnnamed_2 = 60;
pub const BI_SRCH: C2RustUnnamed_2 = 59;
pub const BI_STL: C2RustUnnamed_2 = 58;
pub const BI_SAV: C2RustUnnamed_2 = 57;
pub const BI_DEV: C2RustUnnamed_2 = 56;
pub const BI_DISM: C2RustUnnamed_2 = 55;
pub const BI_DISP: C2RustUnnamed_2 = 54;
pub const BI_FAST_SHOTS: C2RustUnnamed_2 = 53;
pub const BI_INFRA: C2RustUnnamed_2 = 52;
pub const BI_SINV: C2RustUnnamed_2 = 51;
pub const BI_REG: C2RustUnnamed_2 = 50;
pub const BI_FEATH: C2RustUnnamed_2 = 49;
pub const BI_SDIG: C2RustUnnamed_2 = 48;
pub const BI_DAM_RED: C2RustUnnamed_2 = 47;
pub const BI_MOD_MOVES: C2RustUnnamed_2 = 46;
pub const BI_GOLD: C2RustUnnamed_2 = 45;
pub const BI_SPEED: C2RustUnnamed_2 = 44;
pub const BI_FOOD_CURE_BLIND: C2RustUnnamed_2 = 43;
pub const BI_FOOD_CURE_CONF: C2RustUnnamed_2 = 42;
pub const BI_FOOD_LO: C2RustUnnamed_2 = 41;
pub const BI_FOOD_HI: C2RustUnnamed_2 = 40;
pub const BI_FOOD: C2RustUnnamed_2 = 39;
pub const BI_RECALL: C2RustUnnamed_2 = 38;
pub const BI_ESP: C2RustUnnamed_2 = 37;
pub const BI_MAXCLEVEL: C2RustUnnamed_2 = 36;
pub const BI_CLEVEL: C2RustUnnamed_2 = 35;
pub const BI_FAIL2: C2RustUnnamed_2 = 34;
pub const BI_FAIL1: C2RustUnnamed_2 = 33;
pub const BI_SP_ADJ: C2RustUnnamed_2 = 32;
pub const BI_MAXSP: C2RustUnnamed_2 = 31;
pub const BI_CURSP: C2RustUnnamed_2 = 30;
pub const BI_HP_ADJ: C2RustUnnamed_2 = 29;
pub const BI_MAXHP: C2RustUnnamed_2 = 28;
pub const BI_CURHP: C2RustUnnamed_2 = 27;
pub const BI_LIGHT: C2RustUnnamed_2 = 26;
pub const BI_CLASS: C2RustUnnamed_2 = 25;
pub const BI_SCON: C2RustUnnamed_2 = 24;
pub const BI_SDEX: C2RustUnnamed_2 = 23;
pub const BI_SWIS: C2RustUnnamed_2 = 22;
pub const BI_SINT: C2RustUnnamed_2 = 21;
pub const BI_SSTR: C2RustUnnamed_2 = 20;
pub const BI_CON_INDEX: C2RustUnnamed_2 = 19;
pub const BI_DEX_INDEX: C2RustUnnamed_2 = 18;
pub const BI_WIS_INDEX: C2RustUnnamed_2 = 17;
pub const BI_INT_INDEX: C2RustUnnamed_2 = 16;
pub const BI_STR_INDEX: C2RustUnnamed_2 = 15;
pub const BI_CCON: C2RustUnnamed_2 = 14;
pub const BI_CDEX: C2RustUnnamed_2 = 13;
pub const BI_CWIS: C2RustUnnamed_2 = 12;
pub const BI_CINT: C2RustUnnamed_2 = 11;
pub const BI_CSTR: C2RustUnnamed_2 = 10;
pub const BI_ACON: C2RustUnnamed_2 = 9;
pub const BI_ADEX: C2RustUnnamed_2 = 8;
pub const BI_AWIS: C2RustUnnamed_2 = 7;
pub const BI_AINT: C2RustUnnamed_2 = 6;
pub const BI_ASTR: C2RustUnnamed_2 = 5;
pub const BI_CON: C2RustUnnamed_2 = 4;
pub const BI_DEX: C2RustUnnamed_2 = 3;
pub const BI_WIS: C2RustUnnamed_2 = 2;
pub const BI_INT: C2RustUnnamed_2 = 1;
pub const BI_STR: C2RustUnnamed_2 = 0;
pub type borg_time = int32_t;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const BORG_MAX_SETTINGS: C2RustUnnamed_3 = 35;
pub const BORG_RESTORE_IGNORE_SETTINGS: C2RustUnnamed_3 = 34;
pub const BORG_AUTOSAVE: C2RustUnnamed_3 = 33;
pub const BORG_ALLOW_STRANGE_OPTS: C2RustUnnamed_3 = 32;
pub const BORG_STOP_ON_BELL: C2RustUnnamed_3 = 31;
pub const BORG_SAVE_DEATH: C2RustUnnamed_3 = 30;
pub const BORG_DUMP_LEVEL: C2RustUnnamed_3 = 29;
pub const BORG_ENCHANT_LIMIT: C2RustUnnamed_3 = 28;
pub const BORG_SELF_LUNAL: C2RustUnnamed_3 = 27;
pub const BORG_LUNAL_MODE: C2RustUnnamed_3 = 26;
pub const BORG_SELF_SCUM: C2RustUnnamed_3 = 25;
pub const BORG_MONEY_SCUM_AMOUNT: C2RustUnnamed_3 = 24;
pub const BORG_DELAY_FACTOR: C2RustUnnamed_3 = 23;
pub const BORG_CHEST_FAIL_TOLERANCE: C2RustUnnamed_3 = 22;
pub const BORG_RESPAWN_RACE: C2RustUnnamed_3 = 21;
pub const BORG_RESPAWN_CLASS: C2RustUnnamed_3 = 20;
pub const BORG_RESPAWN_WINNERS: C2RustUnnamed_3 = 19;
pub const BORG_CHEAT_DEATH: C2RustUnnamed_3 = 18;
pub const BORG_STOP_KING: C2RustUnnamed_3 = 17;
pub const BORG_NO_DEEPER: C2RustUnnamed_3 = 16;
pub const BORG_STOP_CLEVEL: C2RustUnnamed_3 = 15;
pub const BORG_STOP_DLEVEL: C2RustUnnamed_3 = 14;
pub const BORG_USES_DYNAMIC_CALCS: C2RustUnnamed_3 = 13;
pub const BORG_USES_SWAPS: C2RustUnnamed_3 = 12;
pub const BORG_KILLS_UNIQUES: C2RustUnnamed_3 = 11;
pub const BORG_PLAYS_RISKY: C2RustUnnamed_3 = 10;
pub const BORG_WORSHIPS_GOLD: C2RustUnnamed_3 = 9;
pub const BORG_WORSHIPS_AC: C2RustUnnamed_3 = 8;
pub const BORG_WORSHIPS_MANA: C2RustUnnamed_3 = 7;
pub const BORG_WORSHIPS_HP: C2RustUnnamed_3 = 6;
pub const BORG_WORSHIPS_SPEED: C2RustUnnamed_3 = 5;
pub const BORG_WORSHIPS_DAMAGE: C2RustUnnamed_3 = 4;
pub const BORG_MUNCHKIN_DEPTH: C2RustUnnamed_3 = 3;
pub const BORG_MUNCHKIN_LEVEL: C2RustUnnamed_3 = 2;
pub const BORG_MUNCHKIN_START: C2RustUnnamed_3 = 1;
pub const BORG_VERBOSE: C2RustUnnamed_3 = 0;
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
    pub temp: C2RustUnnamed_11,
    pub status: C2RustUnnamed_10,
    pub time: C2RustUnnamed_9,
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
    pub goal: C2RustUnnamed_8,
    pub init_save: C2RustUnnamed_7,
    pub mon: C2RustUnnamed_6,
    pub id: C2RustUnnamed_5,
    pub near: C2RustUnnamed_4,
    pub morgoth_position: bool,
    pub amt_book: [int16_t; 9],
    pub book_idx: [int16_t; 9],
    pub need_statgain: [bool; 5],
    pub amt_statgain: [int16_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub morgoth: bool,
    pub breeder: bool,
    pub unique: int16_t,
    pub evil_unique: bool,
    pub summoner: bool,
    pub summoner_idx: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
    pub breeders: uint16_t,
    pub scary: bool,
    pub morgoth_panel: loc,
    pub unique: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub key_mode: ::core::ffi::c_int,
    pub kinfo_ignore: *mut uint8_t,
    pub ignore_level: [uint8_t; 27],
    pub ego_ignore_types: *mut *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct source {
    pub what: C2RustUnnamed_13,
    pub which: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub trap: *mut trap,
    pub monster: ::core::ffi::c_int,
    pub object: *mut object,
    pub chest_trap: *mut chest_trap,
}
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const SRC_CHEST_TRAP: C2RustUnnamed_13 = 5;
pub const SRC_OBJECT: C2RustUnnamed_13 = 4;
pub const SRC_MONSTER: C2RustUnnamed_13 = 3;
pub const SRC_PLAYER: C2RustUnnamed_13 = 2;
pub const SRC_TRAP: C2RustUnnamed_13 = 1;
pub const SRC_NONE: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const PROJ_MAX: C2RustUnnamed_14 = 56;
pub const PROJ_MON_CRUSH: C2RustUnnamed_14 = 55;
pub const PROJ_MON_DRAIN: C2RustUnnamed_14 = 54;
pub const PROJ_MON_STUN: C2RustUnnamed_14 = 53;
pub const PROJ_MON_HOLD: C2RustUnnamed_14 = 52;
pub const PROJ_MON_CONF: C2RustUnnamed_14 = 51;
pub const PROJ_MON_SLOW: C2RustUnnamed_14 = 50;
pub const PROJ_MON_SPEED: C2RustUnnamed_14 = 49;
pub const PROJ_MON_HEAL: C2RustUnnamed_14 = 48;
pub const PROJ_MON_POLY: C2RustUnnamed_14 = 47;
pub const PROJ_MON_CLONE: C2RustUnnamed_14 = 46;
pub const PROJ_SLEEP_ALL: C2RustUnnamed_14 = 45;
pub const PROJ_SLEEP_EVIL: C2RustUnnamed_14 = 44;
pub const PROJ_SLEEP_UNDEAD: C2RustUnnamed_14 = 43;
pub const PROJ_DISP_ALL: C2RustUnnamed_14 = 42;
pub const PROJ_DISP_EVIL: C2RustUnnamed_14 = 41;
pub const PROJ_DISP_UNDEAD: C2RustUnnamed_14 = 40;
pub const PROJ_TURN_ALL: C2RustUnnamed_14 = 39;
pub const PROJ_TURN_LIVING: C2RustUnnamed_14 = 38;
pub const PROJ_TURN_EVIL: C2RustUnnamed_14 = 37;
pub const PROJ_TURN_UNDEAD: C2RustUnnamed_14 = 36;
pub const PROJ_AWAY_ALL: C2RustUnnamed_14 = 35;
pub const PROJ_AWAY_EVIL: C2RustUnnamed_14 = 34;
pub const PROJ_AWAY_SPIRIT: C2RustUnnamed_14 = 33;
pub const PROJ_AWAY_UNDEAD: C2RustUnnamed_14 = 32;
pub const PROJ_MAKE_TRAP: C2RustUnnamed_14 = 31;
pub const PROJ_MAKE_DOOR: C2RustUnnamed_14 = 30;
pub const PROJ_KILL_TRAP: C2RustUnnamed_14 = 29;
pub const PROJ_KILL_DOOR: C2RustUnnamed_14 = 28;
pub const PROJ_KILL_WALL: C2RustUnnamed_14 = 27;
pub const PROJ_DARK_WEAK: C2RustUnnamed_14 = 26;
pub const PROJ_LIGHT_WEAK: C2RustUnnamed_14 = 25;
pub const PROJ_ARROW: C2RustUnnamed_14 = 24;
pub const PROJ_HOLY_ORB: C2RustUnnamed_14 = 23;
pub const PROJ_MANA: C2RustUnnamed_14 = 22;
pub const PROJ_MISSILE: C2RustUnnamed_14 = 21;
pub const PROJ_METEOR: C2RustUnnamed_14 = 20;
pub const PROJ_PLASMA: C2RustUnnamed_14 = 19;
pub const PROJ_TIME: C2RustUnnamed_14 = 18;
pub const PROJ_FORCE: C2RustUnnamed_14 = 17;
pub const PROJ_INERTIA: C2RustUnnamed_14 = 16;
pub const PROJ_GRAVITY: C2RustUnnamed_14 = 15;
pub const PROJ_ICE: C2RustUnnamed_14 = 14;
pub const PROJ_WATER: C2RustUnnamed_14 = 13;
pub const PROJ_DISEN: C2RustUnnamed_14 = 12;
pub const PROJ_CHAOS: C2RustUnnamed_14 = 11;
pub const PROJ_NETHER: C2RustUnnamed_14 = 10;
pub const PROJ_NEXUS: C2RustUnnamed_14 = 9;
pub const PROJ_SHARD: C2RustUnnamed_14 = 8;
pub const PROJ_SOUND: C2RustUnnamed_14 = 7;
pub const PROJ_DARK: C2RustUnnamed_14 = 6;
pub const PROJ_LIGHT: C2RustUnnamed_14 = 5;
pub const PROJ_POIS: C2RustUnnamed_14 = 4;
pub const PROJ_COLD: C2RustUnnamed_14 = 3;
pub const PROJ_FIRE: C2RustUnnamed_14 = 2;
pub const PROJ_ELEC: C2RustUnnamed_14 = 1;
pub const PROJ_ACID: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const PROJECT_ROCK: C2RustUnnamed_15 = 32768;
pub const PROJECT_SELF: C2RustUnnamed_15 = 16384;
pub const PROJECT_SHORT: C2RustUnnamed_15 = 8192;
pub const PROJECT_INFO: C2RustUnnamed_15 = 4096;
pub const PROJECT_PLAY: C2RustUnnamed_15 = 2048;
pub const PROJECT_ARC: C2RustUnnamed_15 = 1024;
pub const PROJECT_SAFE: C2RustUnnamed_15 = 512;
pub const PROJECT_AWARE: C2RustUnnamed_15 = 256;
pub const PROJECT_HIDE: C2RustUnnamed_15 = 128;
pub const PROJECT_KILL: C2RustUnnamed_15 = 64;
pub const PROJECT_ITEM: C2RustUnnamed_15 = 32;
pub const PROJECT_GRID: C2RustUnnamed_15 = 16;
pub const PROJECT_STOP: C2RustUnnamed_15 = 8;
pub const PROJECT_THRU: C2RustUnnamed_15 = 4;
pub const PROJECT_BEAM: C2RustUnnamed_15 = 2;
pub const PROJECT_JUMP: C2RustUnnamed_15 = 1;
pub const PROJECT_NONE: C2RustUnnamed_15 = 0;
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
pub struct cmd_info {
    pub desc: *const ::core::ffi::c_char,
    pub key: [keycode_t; 2],
    pub cmd: cmd_code,
    pub hook: Option<unsafe extern "C" fn() -> ()>,
    pub prereq: Option<unsafe extern "C" fn() -> bool>,
    pub nested_keymap: ::core::ffi::c_int,
    pub nested_prompt: *const ::core::ffi::c_char,
    pub nested_error: *const ::core::ffi::c_char,
    pub nested_name: *const ::core::ffi::c_char,
    pub nested_cached_idx: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct region {
    pub col: ::core::ffi::c_int,
    pub row: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub page_rows: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu {
    pub header: *const ::core::ffi::c_char,
    pub title: *const ::core::ffi::c_char,
    pub prompt: *const ::core::ffi::c_char,
    pub selections: *const ::core::ffi::c_char,
    pub inscriptions: *mut ::core::ffi::c_char,
    pub cmd_keys: *const ::core::ffi::c_char,
    pub switch_keys: *const ::core::ffi::c_char,
    pub browse_hook: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void, *const region) -> (),
    >,
    pub keys_hook:
        Option<unsafe extern "C" fn(*mut menu, *const ui_event, ::core::ffi::c_int) -> bool>,
    pub context_hook:
        Option<unsafe extern "C" fn(*mut menu, *const ui_event, *mut ui_event) -> bool>,
    pub flags: ::core::ffi::c_int,
    pub boundary: region,
    pub filter_count: ::core::ffi::c_int,
    pub filter_list: *const ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub menu_data: *mut ::core::ffi::c_void,
    pub skin: *const menu_skin,
    pub row_funcs: *const menu_iter,
    pub cursor: ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub active: region,
    pub cursor_x_offset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_iter {
    pub get_tag: Option<unsafe extern "C" fn(*mut menu, ::core::ffi::c_int) -> ::core::ffi::c_char>,
    pub valid_row:
        Option<unsafe extern "C" fn(*mut menu, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub display_row: Option<
        unsafe extern "C" fn(
            *mut menu,
            ::core::ffi::c_int,
            bool,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub row_handler:
        Option<unsafe extern "C" fn(*mut menu, *const ui_event, ::core::ffi::c_int) -> bool>,
    pub resize: Option<unsafe extern "C" fn(*mut menu) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_skin {
    pub get_cursor: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut region,
        ) -> ::core::ffi::c_int,
    >,
    pub display_list: Option<
        unsafe extern "C" fn(
            *mut menu,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut region,
        ) -> (),
    >,
    pub get_tag: Option<unsafe extern "C" fn(*mut menu, ::core::ffi::c_int) -> ::core::ffi::c_char>,
    pub process_dir: Option<unsafe extern "C" fn(*mut menu, ::core::ffi::c_int) -> ui_event>,
}
pub type skin_id = ::core::ffi::c_uint;
pub const MN_SKIN_COLUMNS: skin_id = 3;
pub const MN_SKIN_OBJECT: skin_id = 2;
pub const MN_SKIN_SCROLL: skin_id = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_data {
    pub data: [[uint8_t; 198]; 66],
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
pub struct borg_setting {
    pub setting_string: *const ::core::ffi::c_char,
    pub setting_type: ::core::ffi::c_char,
    pub default_value: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_commands {
    pub selection: ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub function: Option<unsafe extern "C" fn() -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COLOUR_WHITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLOUR_ORANGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const COLOUR_RED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COLOUR_GREEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLOUR_BLUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COLOUR_YELLOW: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const COLOUR_L_BLUE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const TARGET_LOOK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const BORG_MARK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_LIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_TEMP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const BORG_XTRA: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
static mut borg_commands: [borg_commands; 29] = unsafe {
    [
        borg_commands {
            selection: '?' as i32 as ::core::ffi::c_char,
            description: b"borg command help\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_help as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '$' as i32 as ::core::ffi::c_char,
            description: b"reload borg.txt file\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_init_txt_file as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'z' as i32 as ::core::ffi::c_char,
            description: b"activate borg\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_start as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'x' as i32 as ::core::ffi::c_char,
            description: b"step the borg\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_step as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'm' as i32 as ::core::ffi::c_char,
            description: b"show map information\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_map as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'G' as i32 as ::core::ffi::c_char,
            description: b"target grid features\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_feature as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'I' as i32 as ::core::ffi::c_char,
            description: b"target grid info\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_info as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'D' as i32 as ::core::ffi::c_char,
            description: b"target grid danger\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_target_danger as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '@' as i32 as ::core::ffi::c_char,
            description: b"line of sight map\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_grid_line_of_sight as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'F' as i32 as ::core::ffi::c_char,
            description: b"map of fear levels\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_fear as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'f' as i32 as ::core::ffi::c_char,
            description: b"update config flags\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_flags as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'u' as i32 as ::core::ffi::c_char,
            description: b"refresh borg vars\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_update as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'c' as i32 as ::core::ffi::c_char,
            description: b"toggle cheat death\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_cheat as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 's' as i32 as ::core::ffi::c_char,
            description: b"set search string\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_search_string as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '^' as i32 as ::core::ffi::c_char,
            description: b"show the borg path\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_flow as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'y' as i32 as ::core::ffi::c_char,
            description: b"step history\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_steps as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 't' as i32 as ::core::ffi::c_char,
            description: b"current targeting\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_targeting as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'h' as i32 as ::core::ffi::c_char,
            description: b"things the borg has\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_has as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'p' as i32 as ::core::ffi::c_char,
            description: b"borg power\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_power as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'P' as i32 as ::core::ffi::c_char,
            description: b"preparation depth\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_prepare as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '!' as i32 as ::core::ffi::c_char,
            description: b"time tracking flags\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_time as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'C' as i32 as ::core::ffi::c_char,
            description: b"list count of 'nasties'\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_nasties as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: '0' as i32 as ::core::ffi::c_char,
            description: b"borg stats (str/int etc)\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_stats as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'w' as i32 as ::core::ffi::c_char,
            description: b"swap weapons/armor\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_swaps as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'S' as i32 as ::core::ffi::c_char,
            description: b"spells\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_spells as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'l' as i32 as ::core::ffi::c_char,
            description: b"creates snapshot log file\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_dump as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'o' as i32 as ::core::ffi::c_char,
            description: b"object details\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_object_desc as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'R' as i32 as ::core::ffi::c_char,
            description: b"respawn\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_respawn as unsafe extern "C" fn() -> ()),
        },
        borg_commands {
            selection: 'v' as i32 as ::core::ffi::c_char,
            description: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            function: Some(borg_cmd_version as unsafe extern "C" fn() -> ()),
        },
    ]
};
unsafe extern "C" fn borg_cmd_target() -> bool {
    let mut cmd: ::core::ffi::c_char = 'n' as i32 as ::core::ffi::c_char;
    Term_load();
    Term_flush();
    if target_okay() {
        if !get_com(
            b"Use current target? (y/n) \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut cmd,
        ) {
            borg_note(b"command aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
    }
    if cmd as ::core::ffi::c_int != 'y' as i32 && cmd as ::core::ffi::c_int != 'Y' as i32 {
        let mut ke: ui_event = ui_event { type_0: EVT_NONE };
        if !get_com_ex(
            b"Select grid location (t to target): \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ke,
        ) {
            borg_note(b"targeting aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
        if !target_set_interactive(TARGET_LOOK, borg.c.x, borg.c.y, true_0 != 0) {
            borg_note(b"targeting aborted\0" as *const u8 as *const ::core::ffi::c_char);
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_cmd_help() {
    let mut i: ::core::ffi::c_int = 0;
    let mut line: ::core::ffi::c_int = 0;
    let command_count: ::core::ffi::c_int = (::core::mem::size_of::<[borg_commands; 29]>() as usize)
        .wrapping_div(::core::mem::size_of::<borg_commands>() as usize)
        as ::core::ffi::c_int;
    Term_save();
    Term_clear();
    i = 0 as ::core::ffi::c_int;
    line = 2 as ::core::ffi::c_int;
    while i < command_count {
        Term_putstr(
            2 as ::core::ffi::c_int,
            line,
            -(1 as ::core::ffi::c_int),
            COLOUR_WHITE,
            format(
                b"Command '%c' %s\0" as *const u8 as *const ::core::ffi::c_char,
                borg_commands[i as usize].selection as ::core::ffi::c_int,
                borg_commands[i as usize].description,
            ),
        );
        if (i + 1 as ::core::ffi::c_int) < command_count {
            Term_putstr(
                42 as ::core::ffi::c_int,
                line,
                -(1 as ::core::ffi::c_int),
                COLOUR_WHITE,
                format(
                    b"Command '%c' %s\0" as *const u8 as *const ::core::ffi::c_char,
                    borg_commands[(i + 1 as ::core::ffi::c_int) as usize].selection
                        as ::core::ffi::c_int,
                    borg_commands[(i + 1 as ::core::ffi::c_int) as usize].description,
                ),
            );
        }
        i += 2 as ::core::ffi::c_int;
        line += 1;
    }
    do_cmd_borg();
    Term_load();
}
unsafe extern "C" fn borg_cmd_start() {
    borg_reinit_options();
    borg_clear_best();
    borg.status.active = true_0 != 0;
    borg.status.cancel = false_0 != 0;
    borg_step = 0 as uint16_t;
    if (*player).opts.lazymove_delay as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        borg_note(
            b"# Turning off lazy movement controls\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*player).opts.lazymove_delay = 0 as uint8_t;
    }
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_init_txt_file() {
    borg_init_txt_file();
}
unsafe extern "C" fn borg_cmd_avoidances() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            p = borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
            if !(p < borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int) {
                if p < borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a = COLOUR_ORANGE as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
                    a = COLOUR_YELLOW as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 6 as ::core::ffi::c_int {
                    a = COLOUR_GREEN as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
                    a = COLOUR_BLUE as uint8_t;
                }
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Avoidance value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        borg.c.y,
        borg.c.x,
        (*Term).offset_y / borg_panel_hgt(),
        (*Term).offset_x / borg_panel_wid(),
        borg.avoidance as ::core::ffi::c_int,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
}
unsafe extern "C" fn borg_cmd_step() {
    borg_reinit_options();
    borg_clear_best();
    borg.status.active = true_0 != 0;
    borg.status.cancel = false_0 != 0;
    borg_step = get_quantity(
        b"Step how many times? \0" as *const u8 as *const ::core::ffi::c_char,
        1000 as ::core::ffi::c_int,
    ) as uint16_t;
    if (borg_step as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        borg_step = 1 as uint16_t;
    }
    borg_notice(true_0 != 0);
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    borg_note(format(
        b"# Stepping Borg %d times\0" as *const u8 as *const ::core::ffi::c_char,
        borg_step as ::core::ffi::c_int,
    ));
    borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_update() {
    borg_reinit_options();
    borg.status.active = true_0 != 0;
    borg.status.cancel = true_0 != 0;
    borg_step = 0 as uint16_t;
    borg_notice(true_0 != 0);
    borg_note(b"# Installing keypress hook\0" as *const u8 as *const ::core::ffi::c_char);
    borg_update_entrypoint(true_0 != 0);
}
unsafe extern "C" fn get_cfg_display(
    mut menu: *mut menu,
    mut oid: ::core::ffi::c_int,
    mut cursor: bool,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut value: [::core::ffi::c_char; 40] = [0; 40];
    let mut label_attr: ::core::ffi::c_int = if cursor as ::core::ffi::c_int != 0 {
        COLOUR_L_BLUE
    } else {
        COLOUR_WHITE
    };
    Term_erase(
        if (col - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            col - 1 as ::core::ffi::c_int
        },
        row,
        width,
    );
    c_put_str(
        label_attr as uint8_t,
        (*(&raw mut borg_settings as *mut borg_setting).offset(oid as isize)).setting_string,
        row,
        col,
    );
    if (*(&raw mut borg_settings as *mut borg_setting).offset(oid as isize)).setting_type
        as ::core::ffi::c_int
        == 'b' as i32
    {
        my_strcpy(
            &raw mut value as *mut ::core::ffi::c_char,
            if *borg_cfg.offset(oid as isize) != 0 {
                b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 40]>() as size_t,
        );
    } else {
        snprintf(
            &raw mut value as *mut ::core::ffi::c_char,
            30 as size_t,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            *borg_cfg.offset(oid as isize),
        );
    }
    c_put_str(
        label_attr as uint8_t,
        &raw mut value as *mut ::core::ffi::c_char,
        row,
        col + 30 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn borg_cfg_get_quantity(mut config: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut amt: ::core::ffi::c_int = *borg_cfg.offset(config as isize);
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    strnfmt(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        amt,
    );
    if !get_string(
        format(
            b"Enter new temp value for %s: \0" as *const u8 as *const ::core::ffi::c_char,
            (*(&raw mut borg_settings as *mut borg_setting).offset(config as isize)).setting_string,
        ),
        &raw mut buf as *mut ::core::ffi::c_char,
        7 as size_t,
    ) {
        return 0 as ::core::ffi::c_int;
    }
    amt = atoi(&raw mut buf as *mut ::core::ffi::c_char);
    if amt < -(1 as ::core::ffi::c_int) {
        amt = -(1 as ::core::ffi::c_int);
    }
    return amt;
}
unsafe extern "C" fn borg_cmd_flags() {
    static mut borg_cfg_menu_iter: menu_iter = unsafe {
        menu_iter {
            get_tag: None,
            valid_row: None,
            display_row: Some(
                get_cfg_display
                    as unsafe extern "C" fn(
                        *mut menu,
                        ::core::ffi::c_int,
                        bool,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            row_handler: None,
            resize: None,
        }
    };
    let mut area: region = region {
        col: 21 as ::core::ffi::c_int,
        row: 5 as ::core::ffi::c_int,
        width: 1 as ::core::ffi::c_int,
        page_rows: 10 as ::core::ffi::c_int,
    };
    let mut choice: *mut cmd_info = ::core::ptr::null_mut::<cmd_info>();
    let mut m: *mut menu = ::core::ptr::null_mut::<menu>();
    let mut result: ui_event = ui_event { type_0: EVT_NONE };
    msg(
        b"Pick a configuration setting to change (runtime only)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    m = menu_new(MN_SKIN_SCROLL, &raw mut borg_cfg_menu_iter);
    menu_setpriv(
        m,
        BORG_MAX_SETTINGS as ::core::ffi::c_int,
        &raw mut borg_cfg_menu_iter as *mut ::core::ffi::c_void,
    );
    menu_setpriv(
        m,
        BORG_MAX_SETTINGS as ::core::ffi::c_int,
        &raw mut choice as *mut ::core::ffi::c_void,
    );
    menu_layout(m, &raw mut area);
    screen_save();
    window_make(
        19 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    );
    result = menu_select(m, 0 as ::core::ffi::c_int, true_0 != 0);
    screen_load();
    if result.type_0 as ::core::ffi::c_uint
        == EVT_ESCAPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if result.type_0 as ::core::ffi::c_uint
        == EVT_SELECT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*m).cursor < 0 as ::core::ffi::c_int
            || (*m).cursor >= BORG_MAX_SETTINGS as ::core::ffi::c_int
        {
            borg_note(b"oops\0" as *const u8 as *const ::core::ffi::c_char);
            return;
        }
        if (*(&raw mut borg_settings as *mut borg_setting).offset((*m).cursor as isize))
            .setting_type as ::core::ffi::c_int
            == 'b' as i32
        {
            *borg_cfg.offset((*m).cursor as isize) =
                (*borg_cfg.offset((*m).cursor as isize) == 0) as ::core::ffi::c_int;
            msg(format(
                b"new temp config for %s is %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*(&raw mut borg_settings as *mut borg_setting).offset((*m).cursor as isize))
                    .setting_string,
                if *borg_cfg.offset((*m).cursor as isize) != 0 {
                    b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                },
            ));
        } else {
            let mut new_value: ::core::ffi::c_int = borg_cfg_get_quantity((*m).cursor);
            *borg_cfg.offset((*m).cursor as isize) = new_value;
            msg(format(
                b"new temp config for %s is %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*(&raw mut borg_settings as *mut borg_setting).offset((*m).cursor as isize))
                    .setting_string,
                *borg_cfg.offset((*m).cursor as isize),
            ));
        }
    }
}
unsafe extern "C" fn borg_cmd_cheat() {
    borg.status.cheat_death = !borg.status.cheat_death;
    msg(
        b"Borg -- borg_cheat_death is now %d.\0" as *const u8 as *const ::core::ffi::c_char,
        borg.status.cheat_death as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn borg_cmd_nasties() {
    let mut i: ::core::ffi::c_int = 0;
    borg_note(b"Borg Nasties Count\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while i < borg_nasties_num as ::core::ffi::c_int {
        borg_note(format(
            b"Nasty: [%c] Count: %d, limited: %d\0" as *const u8 as *const ::core::ffi::c_char,
            borg_nasties[i as usize] as ::core::ffi::c_int,
            borg_nasties_count[i as usize] as ::core::ffi::c_int,
            borg_nasties_limit[i as usize] as ::core::ffi::c_int,
        ));
        i += 1;
    }
    msg(b"Borg Nasty Dump Complete.  Examine Log.\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn borg_cmd_search_string() {
    if !get_string(
        b"Borg Match String: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_match as *mut ::core::ffi::c_char,
        70 as size_t,
    ) {
        my_strcpy(
            &raw mut borg_match as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        );
        msg(b"Borg Match String de-activated.\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn get_feature_display(
    mut menu: *mut menu,
    mut oid: ::core::ffi::c_int,
    mut cursor: bool,
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut label_attr: ::core::ffi::c_int = if cursor as ::core::ffi::c_int != 0 {
        COLOUR_L_BLUE
    } else {
        COLOUR_WHITE
    };
    Term_erase(
        if (col - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            col - 1 as ::core::ffi::c_int
        },
        row,
        width,
    );
    if oid < FEAT_MAX as ::core::ffi::c_int {
        c_put_str(
            label_attr as uint8_t,
            (*f_info.offset(oid as isize)).name,
            row,
            col,
        );
    } else {
        let mut i: ::core::ffi::c_int =
            oid - FEAT_MAX as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
        if i >= 0 as ::core::ffi::c_int {
            match i {
                0 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"MARK\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                1 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"GLOW\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                2 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"DARK\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                3 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"OKAY\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                4 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"LIGHT\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                5 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"VIEW\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                6 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"TEMP\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                7 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"XTRA\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                8 => {
                    c_put_str(
                        label_attr as uint8_t,
                        b"IGNORE_MAP\0" as *const u8 as *const ::core::ffi::c_char,
                        row,
                        col,
                    );
                }
                _ => {}
            }
        } else {
            if oid == FEAT_MAX as ::core::ffi::c_int {
                c_put_str(
                    label_attr as uint8_t,
                    b"Traps\0" as *const u8 as *const ::core::ffi::c_char,
                    row,
                    col,
                );
            }
            if oid == FEAT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                c_put_str(
                    label_attr as uint8_t,
                    b"Glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                    row,
                    col,
                );
            }
        }
    };
}
unsafe extern "C" fn borg_cmd_grid_feature() {
    static mut borg_feature_menu_iter: menu_iter = unsafe {
        menu_iter {
            get_tag: None,
            valid_row: None,
            display_row: Some(
                get_feature_display
                    as unsafe extern "C" fn(
                        *mut menu,
                        ::core::ffi::c_int,
                        bool,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            row_handler: None,
            resize: None,
        }
    };
    let mut area: region = region {
        col: 21 as ::core::ffi::c_int,
        row: 5 as ::core::ffi::c_int,
        width: 1 as ::core::ffi::c_int,
        page_rows: 10 as ::core::ffi::c_int,
    };
    let mut m: *mut menu = ::core::ptr::null_mut::<menu>();
    let mut result: ui_event = ui_event { type_0: EVT_NONE };
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    m = menu_new(MN_SKIN_SCROLL, &raw mut borg_feature_menu_iter);
    menu_setpriv(
        m,
        FEAT_MAX as ::core::ffi::c_int + 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
        &raw mut borg_feature_menu_iter as *mut ::core::ffi::c_void,
    );
    menu_layout(m, &raw mut area);
    screen_save();
    window_make(
        19 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
    );
    result = menu_select(m, 0 as ::core::ffi::c_int, true_0 != 0);
    screen_load();
    if result.type_0 as ::core::ffi::c_uint
        == EVT_ESCAPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        let mut current_block_13: u64;
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y as isize))
            .offset(x as isize) as *mut borg_grid;
            if (*m).cursor < FEAT_MAX as ::core::ffi::c_int {
                if !((*ag).feat as ::core::ffi::c_int == (*m).cursor) {
                    current_block_13 = 11812396948646013369;
                } else {
                    current_block_13 = 7659304154607701039;
                }
            } else {
                let mut i: ::core::ffi::c_int =
                    (*m).cursor - FEAT_MAX as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
                if i >= 0 as ::core::ffi::c_int {
                    match i {
                        0 => {
                            current_block_13 = 12465488344595199675;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block_13 = 10562881441515713487;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block_13 = 3833396878894470733;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        3 => {
                            current_block_13 = 41435768513642067;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        4 => {
                            current_block_13 = 5412339429498267972;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        5 => {
                            current_block_13 = 13065097511781892099;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        6 => {
                            current_block_13 = 18315407756364132905;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        7 => {
                            current_block_13 = 12493991067027566107;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        8 => {
                            current_block_13 = 4533473061595253016;
                            match current_block_13 {
                                4533473061595253016 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_IGNORE_MAP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                10562881441515713487 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                3833396878894470733 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                41435768513642067 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_OKAY == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                5412339429498267972 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_LIGHT == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                13065097511781892099 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                18315407756364132905 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_TEMP == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                12493991067027566107 => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_XTRA == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                                _ => {
                                    if (*ag).info as ::core::ffi::c_int & BORG_MARK == 0 {
                                        current_block_13 = 11812396948646013369;
                                    } else {
                                        current_block_13 = 7659304154607701039;
                                    }
                                }
                            }
                        }
                        _ => {
                            current_block_13 = 7659304154607701039;
                        }
                    }
                } else {
                    if (*m).cursor == FEAT_MAX as ::core::ffi::c_int {
                        if !(*ag).trap {
                            current_block_13 = 11812396948646013369;
                        } else {
                            current_block_13 = 7333393191927787629;
                        }
                    } else {
                        current_block_13 = 7333393191927787629;
                    }
                    match current_block_13 {
                        11812396948646013369 => {}
                        _ => {
                            if (*m).cursor
                                == FEAT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            {
                                if !(*ag).glyph {
                                    current_block_13 = 11812396948646013369;
                                } else {
                                    current_block_13 = 7659304154607701039;
                                }
                            } else {
                                current_block_13 = 7659304154607701039;
                            }
                        }
                    }
                }
            }
            match current_block_13 {
                7659304154607701039 => {
                    if borg_cave_floor_bold(y, x) {
                        a = COLOUR_YELLOW as uint8_t;
                    }
                    print_rel('*' as i32, a, y, x);
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Press any key.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_feature() {
    let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut mask: uint16_t = 0;
    if !borg_cmd_target() {
        return;
    }
    let mut l: loc = loc { x: 0, y: 0 };
    target_get(&raw mut l);
    y = l.y;
    x = l.x;
    mask = (*borg_grids[y as usize].offset(x as isize)).feat as uint16_t;
    let mut feat: uint8_t = (*square(cave, borg.c)).feat;
    borg_note(format(
        b"Borg's Feat for grid (%d, %d) is %d, game Feat is %d\0" as *const u8
            as *const ::core::ffi::c_char,
        y,
        x,
        mask as ::core::ffi::c_int,
        feat as ::core::ffi::c_int,
    ));
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_info() {
    let mut i: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut l: loc = loc { x: 0, y: 0 };
    if !borg_cmd_target() {
        return;
    }
    target_get(&raw mut l);
    y = l.y;
    x = l.x;
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_MARK != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is MARK\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_GLOW != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is GLOW\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_DARK != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is DARK\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_OKAY != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is OKAY\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_LIGHT != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is LITE\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_VIEW != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is VIEW\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_TEMP != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is TEMP\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    if (*borg_grids[y as usize].offset(x as isize)).info as ::core::ffi::c_int & BORG_XTRA != 0 {
        msg(
            b"Borg Info for grid (%d, %d) is XTRA\0" as *const u8 as *const ::core::ffi::c_char,
            y,
            x,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < SQUARE_MAX as ::core::ffi::c_int {
        if flag_has(
            (*square(cave, l)).info,
            (SQUARE_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            i,
        ) {
            msg(format(
                b"Sys Info for grid (%d, %d) is %d\0" as *const u8 as *const ::core::ffi::c_char,
                y,
                x,
                i,
            ));
        }
        i += 1;
    }
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_target_danger() {
    let mut n: ::core::ffi::c_int = 0;
    let mut l: loc = loc { x: 0, y: 0 };
    if !borg_cmd_target() {
        return;
    }
    target_get(&raw mut l);
    n = get_quantity(
        b"Time on square? \0" as *const u8 as *const ::core::ffi::c_char,
        1000 as ::core::ffi::c_int,
    );
    msg(
        b"Danger(%d,%d,%d) is %d\0" as *const u8 as *const ::core::ffi::c_char,
        l.x,
        l.y,
        n,
        borg_danger(l.y, l.x, n, true_0 != 0, false_0 != 0),
    );
    prt_map();
}
unsafe extern "C" fn borg_cmd_steps() {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: uint8_t = COLOUR_RED as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_step.num as ::core::ffi::c_int {
        print_rel(
            '*' as i32,
            a,
            *track_step
                .y
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
            *track_step
                .x
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
        );
        msg(
            b"(-%d) Steps noted %d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            *track_step
                .y
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
            *track_step
                .x
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        print_rel(
            '*' as i32,
            COLOUR_ORANGE as uint8_t,
            *track_step
                .y
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
            *track_step
                .x
                .offset((track_step.num as ::core::ffi::c_int - i) as isize),
        );
        i += 1;
    }
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_monsters() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if (*kill).r_idx != 0 {
            let mut x: ::core::ffi::c_int = (*kill).pos.x;
            let mut y: ::core::ffi::c_int = (*kill).pos.y;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            n += 1;
        }
        i += 1;
    }
    msg(
        b"There are %d known monsters.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_grid_objects() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < borg_takes_nxt as ::core::ffi::c_int {
        let mut take: *mut borg_take = borg_takes.offset(i as isize) as *mut borg_take;
        if (*take).kidx != 0 {
            let mut x: ::core::ffi::c_int = (*take).x as ::core::ffi::c_int;
            let mut y: ::core::ffi::c_int = (*take).y as ::core::ffi::c_int;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            n += 1;
        }
        i += 1;
    }
    msg(
        b"There are %d known objects.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_targeting() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut n_x: ::core::ffi::c_int = 0;
    let mut n_y: ::core::ffi::c_int = 0;
    let mut l: loc = loc { x: 0, y: 0 };
    let mut ch: ::core::ffi::c_char = 0;
    let mut svDelay: uint8_t = (*player).opts.delay_factor;
    (*player).opts.delay_factor = 200 as uint8_t;
    n_x = borg.c.x;
    n_y = borg.c.y;
    target_get(&raw mut l);
    x = l.x;
    y = l.y;
    loop {
        print_rel('*' as i32, COLOUR_RED as uint8_t, n_y, n_x);
        if n_x == x && n_y == y {
            break;
        }
        borg_inc_motion(&raw mut n_y, &raw mut n_x, borg.c.y, borg.c.x, y, x);
    }
    msg(b"Borg's Targeting Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    n_x = borg.c.x;
    n_y = borg.c.y;
    x = l.x;
    y = l.y;
    if !get_com(
        b"Borg command: Show Arc (Y/y): \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut ch,
    ) {
        return;
    }
    msg(b"Actual Targeting Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    if ch as ::core::ffi::c_int != 'Y' as i32 && ch as ::core::ffi::c_int != 'y' as i32 {
        project(
            source_player(),
            0 as ::core::ffi::c_int,
            loc(x, y),
            1 as ::core::ffi::c_int,
            PROJ_MISSILE as ::core::ffi::c_int,
            PROJECT_BEAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as uint8_t,
            ::core::ptr::null::<object>(),
        );
    } else {
        project(
            source_player(),
            10 as ::core::ffi::c_int,
            loc(x, y),
            50 as ::core::ffi::c_int,
            PROJ_MISSILE as ::core::ffi::c_int,
            PROJECT_ARC as ::core::ffi::c_int,
            60 as ::core::ffi::c_int,
            4 as uint8_t,
            ::core::ptr::null::<object>(),
        );
    }
    (*player).opts.delay_factor = svDelay;
    prt_map();
}
unsafe extern "C" fn borg_cmd_flow() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut o: ::core::ffi::c_int = 0;
    let mut false_y: ::core::ffi::c_int = 0;
    let mut false_x: ::core::ffi::c_int = 0;
    false_y = borg.c.y;
    false_x = borg.c.x;
    o = 0 as ::core::ffi::c_int;
    while o < 250 as ::core::ffi::c_int {
        let mut b_n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0;
        let mut b_c: ::core::ffi::c_int = 0;
        b_c = (*borg_data_flow).data[borg.c.y as usize][borg.c.x as usize] as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int;
        b_c = b_c - 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            x = false_x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            y = false_y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            c = (*borg_data_flow).data[y as usize][x as usize] as ::core::ffi::c_int
                * 10 as ::core::ffi::c_int;
            if !(c > b_c) {
                if c < b_c {
                    b_n = 0 as ::core::ffi::c_int;
                }
                b_n += 1;
                if !(b_n >= 2 as ::core::ffi::c_int
                    && Rand_div(b_n as uint32_t) as int32_t != 0 as int32_t)
                {
                    b_i = i;
                    b_c = c;
                }
            }
            i += 1;
        }
        if b_i >= 0 as ::core::ffi::c_int {
            x = false_x + ddx_ddd[b_i as usize] as ::core::ffi::c_int;
            y = false_y + ddy_ddd[b_i as usize] as ::core::ffi::c_int;
            print_rel('*' as i32, COLOUR_RED as uint8_t, y, x);
            false_y = y;
            false_x = x;
        }
        o += 1;
    }
    print_rel(
        '*' as i32,
        COLOUR_YELLOW as uint8_t,
        borg_flow_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        borg_flow_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
    );
    msg(b"Probable Flow Path\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_fear() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            p = borg_fear_region[(y / 11 as ::core::ffi::c_int) as usize]
                [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            if !(p < borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int) {
                if p < borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a = COLOUR_ORANGE as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
                    a = COLOUR_YELLOW as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 6 as ::core::ffi::c_int {
                    a = COLOUR_GREEN as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
                    a = COLOUR_BLUE as uint8_t;
                }
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Regional Fear.\0" as *const u8 as *const ::core::ffi::c_char,
        borg.c.y,
        borg.c.x,
        (*Term).offset_y / borg_panel_hgt(),
        (*Term).offset_x / borg_panel_wid(),
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
    y = 1 as ::core::ffi::c_int;
    while y <= AUTO_MAX_Y {
        x = 1 as ::core::ffi::c_int;
        while x <= AUTO_MAX_X {
            let mut a_0: uint8_t = COLOUR_BLUE as uint8_t;
            p = borg_fear_monsters[y as usize][x as usize] as ::core::ffi::c_int;
            if !(p <= 0 as ::core::ffi::c_int) {
                if p == 1 as ::core::ffi::c_int {
                    a_0 = COLOUR_L_BLUE as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
                    && p > 1 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_BLUE as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                    && p > borg.avoidance as ::core::ffi::c_int / 20 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_GREEN as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int
                    && p > borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_YELLOW as uint8_t;
                }
                if p < borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    && p > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int
                {
                    a_0 = COLOUR_ORANGE as uint8_t;
                }
                if p > borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
                    a_0 = COLOUR_RED as uint8_t;
                }
                print_rel('*' as i32, a_0, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(
        b"(%d,%d of %d,%d) Monster Fear.\0" as *const u8 as *const ::core::ffi::c_char,
        borg.c.y,
        borg.c.x,
        (*Term).offset_y / borg_panel_hgt(),
        (*Term).offset_x / borg_panel_wid(),
    );
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_time() {
    msg(
        b"time: (%ld) \0" as *const u8 as *const ::core::ffi::c_char,
        borg.time.now as ::core::ffi::c_long,
    );
    msg(
        b"; from town (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        (borg.time.now - borg.time.town) as ::core::ffi::c_long,
    );
    msg(
        b"; bounciness (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        borg.antibounce_count as ::core::ffi::c_long,
    );
    msg(
        b"; when inviso (%ld)\0" as *const u8 as *const ::core::ffi::c_char,
        borg.need_see_invis as ::core::ffi::c_long,
    );
}
unsafe extern "C" fn borg_cmd_grid_line_of_sight() {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a: uint8_t = COLOUR_RED as uint8_t;
            if borg_los(borg.c.y, borg.c.x, y, x) {
                print_rel('*' as i32, a, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has Projectable to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a_0: uint8_t = COLOUR_YELLOW as uint8_t;
            if square_in_bounds(cave, loc(x, y)) {
                if borg_projectable_dark(borg.c.y, borg.c.x, y, x) {
                    print_rel('*' as i32, a_0, y, x);
                }
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has Projectable Dark to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            let mut a_1: uint8_t = COLOUR_GREEN as uint8_t;
            if borg_los(borg.c.y, borg.c.x, y, x) {
                print_rel('*' as i32, a_1, y, x);
            }
            x += 1;
        }
        y += 1;
    }
    msg(b"Borg has LOS to these places.\0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    prt_map();
}
unsafe extern "C" fn borg_cmd_power() {
    let mut p: int32_t = 0;
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    p = borg_power();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    msg(
        b"Current Borg Power %ld\0" as *const u8 as *const ::core::ffi::c_char,
        p,
    );
    msg(
        b"Current Home Power %ld\0" as *const u8 as *const ::core::ffi::c_char,
        borg_power_home(),
    );
}
unsafe extern "C" fn borg_cmd_prepare() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    i = 1 as ::core::ffi::c_int;
    while i <= 101 as ::core::ffi::c_int {
        if !(borg_prepared(i) as *mut ::core::ffi::c_char).is_null() {
            break;
        }
        i += 1;
    }
    msg(
        b"Max Level: %d  Prep'd For: %d  Reason: %s\0" as *const u8 as *const ::core::ffi::c_char,
        *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize),
        i - 1 as ::core::ffi::c_int,
        borg_prepared(i),
    );
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        msg(b"Unable to check for big fight from town.\0" as *const u8
            as *const ::core::ffi::c_char);
    } else if borg.ready_morgoth == 1 as ::core::ffi::c_int {
        msg(b"You are ready for the big fight!!\0" as *const u8 as *const ::core::ffi::c_char);
    } else if borg.ready_morgoth == 0 as ::core::ffi::c_int {
        msg(b"You are NOT ready for the big fight!!\0" as *const u8 as *const ::core::ffi::c_char);
    } else if borg.ready_morgoth == -(1 as ::core::ffi::c_int) {
        msg(b"No readiness check done.\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn borg_cmd_stats() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < STAT_MAX as ::core::ffi::c_int {
        borg_note(format(
            b"stat # %s, is: cur %d, used %d\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char)
                .offset((BI_STR as ::core::ffi::c_int + i) as isize),
            *borg
                .trait_0
                .offset((BI_CSTR as ::core::ffi::c_int + i) as isize),
            *borg
                .trait_0
                .offset((BI_STR as ::core::ffi::c_int + i) as isize),
        ));
        i += 1;
    }
}
unsafe extern "C" fn borg_cmd_swaps() {
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    borg_power();
    borg_update();
    if weapon_swap != 0 {
        item =
            borg_items.offset((weapon_swap - 1 as ::core::ffi::c_int) as isize) as *mut borg_item;
        msg(
            b"Swap Weapon:  %s, value= %d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            weapon_swap_value,
        );
    } else {
        msg(b"Swap Weapon:  NONE\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if armour_swap != 0 {
        item =
            borg_items.offset((armour_swap - 1 as ::core::ffi::c_int) as isize) as *mut borg_item;
        msg(
            b"Swap Armour:  %s, value= %d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            armour_swap_value,
        );
    } else {
        msg(b"Swap Armour:  NONE\0" as *const u8 as *const ::core::ffi::c_char);
    };
}
unsafe extern "C" fn borg_cmd_spells() {
    let mut ii: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut cmd: ::core::ffi::c_char = 0;
    Term_save();
    if borg_can_cast() {
        let mut i: ::core::ffi::c_int = 0;
        let mut book: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < (*(*player).class).magic.total_spells {
            let mut as_0: *mut borg_magic = borg_magics.offset(i as isize) as *mut borg_magic;
            let mut failpercent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if ((*as_0).level as ::core::ffi::c_int) < 99 as ::core::ffi::c_int {
                if book != (*as_0).book {
                    if book != -(1 as ::core::ffi::c_int) {
                        get_com(
                            format(
                                b"Exam spell book %d.  Press any key for next book.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*as_0).book,
                            ),
                            &raw mut cmd,
                        );
                    }
                    ii = 2 as ::core::ffi::c_int;
                    Term_clear();
                    let fresh0 = ii;
                    ii = ii + 1;
                    Term_putstr(
                        1 as ::core::ffi::c_int,
                        fresh0,
                        -(1 as ::core::ffi::c_int),
                        COLOUR_WHITE,
                        b"[ Spells ].\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    book = (*as_0).book;
                }
                let mut legal: *const ::core::ffi::c_char =
                    if borg_spell_legal((*as_0).spell_enum) as ::core::ffi::c_int != 0 {
                        b"legal\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"Not Legal \0" as *const u8 as *const ::core::ffi::c_char
                    };
                failpercent = borg_spell_fail_rate((*as_0).spell_enum);
                let fresh1 = ii;
                ii = ii + 1;
                Term_putstr(
                    1 as ::core::ffi::c_int,
                    fresh1,
                    -(1 as ::core::ffi::c_int),
                    COLOUR_WHITE,
                    format(
                        b"%s, %s, attempted %ld times, fail rate:%d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*as_0).name,
                        legal,
                        (*as_0).times as ::core::ffi::c_long,
                        failpercent,
                    ),
                );
            }
            i += 1;
        }
        get_com(
            format(
                b"Exam spell book %d.  Press any key for next book.\0" as *const u8
                    as *const ::core::ffi::c_char,
                book,
            ),
            &raw mut cmd,
        );
    }
    Term_load();
}
unsafe extern "C" fn borg_cmd_has() {
    let mut item: ::core::ffi::c_int = 0;
    let mut to: ::core::ffi::c_int = 0;
    let mut cmd: ::core::ffi::c_char = 0;
    if !get_com(
        b"Dynamic Borg Has What: ((a)ny/(i)nv/(w)orn/a(r)tifact/(s)kill) \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut cmd,
    ) {
        return;
    }
    match cmd as ::core::ffi::c_int {
        97 | 65 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).k_max as ::core::ffi::c_int;
        }
        105 | 73 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int;
        }
        119 | 87 => {
            item = (*z_info).pack_size as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int;
        }
        114 | 82 => {
            item = 0 as ::core::ffi::c_int;
            to = (*z_info).pack_size as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int
                + (*z_info).quiver_size as ::core::ffi::c_int;
        }
        _ => {
            item = 0 as ::core::ffi::c_int;
            to = BI_MAX as ::core::ffi::c_int;
        }
    }
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_update();
    borg_object_fully_id();
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    while item < to {
        match cmd as ::core::ffi::c_int {
            97 | 65 => {
                if *borg.has.offset(item as isize) != 0 {
                    borg_note(format(
                        b"Item-Kind:%03d name=%s value= %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        (*k_info.offset(item as isize)).name,
                        *borg.has.offset(item as isize),
                    ));
                }
            }
            105 | 73 => {
                if (*borg_items.offset(item as isize)).iqty != 0 {
                    borg_note(format(
                        b"Item-Invn:%03d desc= %s qty %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        &raw mut (*borg_items.offset(item as isize)).desc
                            as *mut ::core::ffi::c_char,
                        (*borg_items.offset(item as isize)).iqty as ::core::ffi::c_int,
                    ));
                }
            }
            119 | 87 => {
                if (*borg_items.offset(item as isize)).iqty != 0 {
                    borg_note(format(
                        b"Item-Worn:%03d desc= %s qty %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        item,
                        &raw mut (*borg_items.offset(item as isize)).desc
                            as *mut ::core::ffi::c_char,
                        (*borg_items.offset(item as isize)).iqty as ::core::ffi::c_int,
                    ));
                }
            }
            114 | 82 => {
                if (*borg_items.offset(item as isize)).iqty as ::core::ffi::c_int != 0
                    && (*borg_items.offset(item as isize)).art_idx as ::core::ffi::c_int != 0
                {
                    borg_note(format(
                        b"Item-Arti:%03d name= %s.\0" as *const u8 as *const ::core::ffi::c_char,
                        item,
                        (*a_info.offset((*borg_items.offset(item as isize)).art_idx as isize)).name,
                    ));
                }
            }
            _ => {
                borg_note(format(
                    b"trait %d (%s) value= %d.\0" as *const u8 as *const ::core::ffi::c_char,
                    item,
                    *(&raw mut prefix_pref as *mut *const ::core::ffi::c_char)
                        .offset(item as isize),
                    *borg.trait_0.offset(item as isize),
                ));
            }
        }
        item += 1;
    }
    msg(b"Borg_has[] dump complete.  Examine Log. \0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn borg_cmd_dump() {
    borg_inventory();
    borg_write_map(true_0 != 0);
}
unsafe extern "C" fn borg_cmd_version() {
    msg(
        b"Borg Version: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut borg_engine_date as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn borg_cmd_object_desc() {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item2: *mut object = ::core::ptr::null_mut::<object>();
    n = get_quantity(
        b"Which item?\0" as *const u8 as *const ::core::ffi::c_char,
        (*z_info).pack_size as ::core::ffi::c_int,
    );
    borg_find_all_items();
    borg_notice(true_0 != 0);
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    borg_power();
    borg_update();
    Term_save();
    item2 = *(*(*player).upkeep).inven.offset(n as isize);
    borg_display_item(item2, n);
    msg(b"Borg believes: \0" as *const u8 as *const ::core::ffi::c_char);
    event_signal(EVENT_MESSAGE_FLUSH);
    Term_load();
}
unsafe extern "C" fn borg_cmd_respawn() {
    let mut cmd: ::core::ffi::c_char = 0;
    get_com(
        b"Are you sure you want to Respawn this borg? (y or n)? \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut cmd,
    );
    if cmd as ::core::ffi::c_int == 'y' as i32 || cmd as ::core::ffi::c_int == 'Y' as i32 {
        borg_force_reincarnate();
        borg_cmd_start();
    }
}
unsafe extern "C" fn borg_cmd_grid_glyphs() {
    let mut glyph_check: ::core::ffi::c_int = 0;
    let mut a: uint8_t = COLOUR_RED as uint8_t;
    glyph_check = 0 as ::core::ffi::c_int;
    while glyph_check < track_glyph.num as ::core::ffi::c_int {
        print_rel(
            '*' as i32,
            a,
            *track_glyph.y.offset(glyph_check as isize),
            *track_glyph.x.offset(glyph_check as isize),
        );
        msg(
            b"Borg has Glyph (%d)noted.\0" as *const u8 as *const ::core::ffi::c_char,
            glyph_check,
        );
        event_signal(EVENT_MESSAGE_FLUSH);
        glyph_check += 1;
    }
}
unsafe extern "C" fn borg_cmd_map() {
    let mut option: ::core::ffi::c_char = 0;
    if !get_com(
        b"What to show ((a)voidances, (f)eatures, (g)lyphs, (m)onsters, (o)bjects): \0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut option,
    ) {
        return;
    }
    match option as ::core::ffi::c_int {
        97 => {
            borg_cmd_avoidances();
            return;
        }
        102 => {
            borg_cmd_grid_feature();
            return;
        }
        103 => {
            borg_cmd_grid_glyphs();
            return;
        }
        109 => {
            borg_cmd_grid_monsters();
            return;
        }
        111 => {
            borg_cmd_grid_objects();
            return;
        }
        _ => {
            msg(b"That is not a legal option.\0" as *const u8 as *const ::core::ffi::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_borg() {
    let mut i: ::core::ffi::c_int = 0;
    let mut cmd: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let command_count: ::core::ffi::c_int = (::core::mem::size_of::<[borg_commands; 29]>() as usize)
        .wrapping_div(::core::mem::size_of::<borg_commands>() as usize)
        as ::core::ffi::c_int;
    borg.c = (*player).grid;
    if !get_com(
        b"Borg command (? for help): \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut cmd,
    ) {
        Term_load();
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < command_count {
        if borg_commands[i as usize].selection as ::core::ffi::c_int == cmd as ::core::ffi::c_int {
            if !borg_initialized || game_closed as ::core::ffi::c_int != 0 {
                if borg_initialized {
                    borg_free();
                }
                borg_init();
                if borg_init_failure {
                    borg_initialized = false_0 != 0;
                    borg_free();
                    borg_note(
                        b"** startup failure borg cannot run ** \0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    Term_fresh();
                    return;
                }
            }
            borg_commands[i as usize]
                .function
                .expect("non-null function pointer")();
            return;
        }
        i += 1;
    }
    msg(b"That is not a legal Borg command.\0" as *const u8 as *const ::core::ffi::c_char);
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
