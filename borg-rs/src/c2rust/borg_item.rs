extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    static mut z_info: *mut angband_constants;
    static all_letters_nohjkl: [::core::ffi::c_char; 0];
    fn borg_equipment();
    fn borg_inventory();
    fn borg_keypress(k: keycode_t) -> errr;
    static mut sv_food_slime_mold: ::core::ffi::c_int;
    fn borg_notice_store();
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
pub struct element_info {
    pub res_level: int16_t,
    pub flags: bitflag,
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
pub type keycode_t = uint32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut borg_items: *mut borg_item = ::core::ptr::null::<borg_item>() as *mut borg_item;
#[no_mangle]
pub static mut safe_items: *mut borg_item = ::core::ptr::null::<borg_item>() as *mut borg_item;
#[no_mangle]
pub unsafe extern "C" fn borg_get_note(mut item: *const borg_item) -> *const ::core::ffi::c_char {
    if !(*item).note.is_null() {
        return (*item).note;
    }
    return b"\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn borg_deinscribe(mut i: ::core::ffi::c_int) {
    if (*borg_items.offset(i as isize)).tval as ::core::ffi::c_int == TV_FOOD as ::core::ffi::c_int
        && (*borg_items.offset(i as isize)).sval as ::core::ffi::c_int == sv_food_slime_mold
    {
        return;
    }
    borg_keypress('}' as i32 as keycode_t);
    if i < (*z_info).pack_size as ::core::ffi::c_int {
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(i as isize)
                as keycode_t,
        );
    } else if i < (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*z_info).pack_size as ::core::ffi::c_int {
            if (*borg_items.offset(j as isize)).iqty as ::core::ffi::c_int != 0
                && *(*borg_items.offset(j as isize))
                    .note
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '{' as i32
            {
                borg_keypress('/' as i32 as keycode_t);
                break;
            } else {
                j += 1;
            }
        }
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                .offset((i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                as keycode_t,
        );
    } else {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 <= (*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int {
            if (*borg_items.offset(j_0 as isize)).iqty as ::core::ffi::c_int != 0
                && *(*borg_items.offset(j_0 as isize))
                    .note
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '{' as i32
            {
                borg_keypress('|' as i32 as keycode_t);
                break;
            } else {
                j_0 += 1;
            }
        }
        borg_keypress(
            ('0' as i32
                + (i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)))
                as keycode_t,
        );
    }
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
}
#[no_mangle]
pub unsafe extern "C" fn borg_item_weight(mut item: *mut borg_item) -> int16_t {
    return ((*item).iqty as ::core::ffi::c_int * (*item).weight as ::core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_find_all_items() {
    borg_equipment();
    borg_inventory();
    borg_notice_store();
    memcpy(
        safe_items as *mut ::core::ffi::c_void,
        borg_items as *const ::core::ffi::c_void,
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_item() {
    borg_items = mem_zalloc(
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    ) as *mut borg_item;
    safe_items = mem_zalloc(
        (((*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
    ) as *mut borg_item;
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_item() {
    if !safe_items.is_null() {
        mem_free(safe_items as *mut ::core::ffi::c_void);
        safe_items = ::core::ptr::null_mut::<borg_item>();
    }
    if !borg_items.is_null() {
        mem_free(borg_items as *mut ::core::ffi::c_void);
        borg_items = ::core::ptr::null_mut::<borg_item>();
    }
}
