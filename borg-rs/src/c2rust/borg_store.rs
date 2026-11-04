pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_init_h::borg_init_failure;
use crate::headers::borg_inventory_h::borg_first_empty_inventory_slot;
use crate::headers::borg_item_analyze_h::borg_item_analyze;
pub use crate::headers::borg_item_h::borg_item;
pub use crate::headers::borg_store_h::borg_shop;
use crate::headers::borg_store_sell_h::{borg_free_store_sell, borg_init_store_sell};
pub use crate::headers::cave_h::{
    chunk, connector, f_info, feature, heatmap, monster_group, square, trap, C2RustUnnamedH_18,
    FEAT_MAX,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_options_h::{
    OPT_animate_flicker, OPT_auto_more, OPT_autoexplore_commands, OPT_birth_ai_learn,
    OPT_birth_connect_stairs, OPT_birth_feelings, OPT_birth_force_descend, OPT_birth_know_flavors,
    OPT_birth_know_runes, OPT_birth_levels_persist, OPT_birth_lose_arts, OPT_birth_no_artifacts,
    OPT_birth_no_recall, OPT_birth_no_selling, OPT_birth_percent_damage, OPT_birth_randarts,
    OPT_birth_stacking, OPT_birth_start_kit, OPT_center_player, OPT_cheat_hear, OPT_cheat_live,
    OPT_cheat_room, OPT_cheat_xtra, OPT_disturb_near, OPT_effective_speed, OPT_highlight_player,
    OPT_hp_changes_color, OPT_hybrid_walls, OPT_mouse_movement, OPT_none, OPT_notify_recharge,
    OPT_pickup_always, OPT_pickup_inven, OPT_purple_uniques, OPT_rogue_like_commands,
    OPT_score_hear, OPT_score_live, OPT_score_room, OPT_score_xtra, OPT_show_damage,
    OPT_show_flavors, OPT_show_target, OPT_solid_walls, OPT_use_old_target, OPT_use_sound,
    OPT_view_yellow_light,
};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
use crate::headers::message_h::msg;
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_desc_h::{
    object_desc, C2RustUnnamedH_42, ODESC_ALTNUM, ODESC_BASE, ODESC_CAPITAL, ODESC_COMBAT,
    ODESC_EXTRA, ODESC_FULL, ODESC_NOEGO, ODESC_PLURAL, ODESC_PREFIX, ODESC_SINGULAR, ODESC_SPOIL,
    ODESC_STORE, ODESC_TERSE,
};
use crate::headers::obj_power_h::{object_value, object_value_real};
use crate::headers::obj_tval_h::tval_can_have_charges;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::store_h::{object_buy, owner, store, store_stock_list, stores};
use crate::headers::string_h::{memcpy, memset, strcmp};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut SHOP_MENU_ITEMS: *const ::core::ffi::c_char =
    b"acfhjmnoqruvyzABDFGHJKLMNOPQRSTUVWXYZ\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut borg_shops: *mut borg_shop = ::core::ptr::null::<borg_shop>() as *mut borg_shop;
#[no_mangle]
pub static mut borg_safe_shops: *mut borg_shop = ::core::ptr::null::<borg_shop>() as *mut borg_shop;
#[no_mangle]
pub unsafe extern "C" fn borg_home_full() -> bool {
    return (*(*borg_shops.offset(BORG_HOME as isize)).ware.offset(
        ((*z_info).store_inven_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_inventory_full() -> bool {
    return borg_first_empty_inventory_slot() == -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn borg_price_item(
    mut obj: *const object,
    mut store_buying: bool,
    mut qty: ::core::ffi::c_int,
    mut this_store: ::core::ffi::c_int,
) -> int32_t {
    let mut adjust: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    let mut price: ::core::ffi::c_int = 0;
    let mut proprietor: *mut owner = ::core::ptr::null_mut::<owner>();
    if this_store
        == (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        return 0 as int32_t;
    }
    proprietor = (*stores.offset(this_store as isize)).owner;
    if tval_can_have_charges(obj) {
        price = if object_value_real(obj, qty) > object_value(obj, qty) {
            object_value(obj, qty)
        } else {
            object_value_real(obj, qty)
        };
    } else {
        price = if object_value_real(obj, 1 as ::core::ffi::c_int)
            > object_value(obj, 1 as ::core::ffi::c_int)
        {
            object_value(obj, 1 as ::core::ffi::c_int)
        } else {
            object_value_real(obj, 1 as ::core::ffi::c_int)
        };
    }
    if price <= 0 as ::core::ffi::c_int {
        return 0 as int32_t;
    }
    if this_store
        == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        adjust = 150 as ::core::ffi::c_int;
    }
    if store_buying {
        adjust = 100 as ::core::ffi::c_int + (100 as ::core::ffi::c_int - adjust);
        if adjust > 100 as ::core::ffi::c_int {
            adjust = 100 as ::core::ffi::c_int;
        }
        price = price * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
        if this_store
            == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            price = price / 2 as ::core::ffi::c_int;
        }
        if (*player).opts.opt[OPT_birth_no_selling as ::core::ffi::c_int as usize] {
            return 0 as int32_t;
        }
    } else {
        if tval_can_have_charges(obj) {
            price = object_value_real(obj, qty);
        } else {
            price = object_value_real(obj, 1 as ::core::ffi::c_int);
        }
        if this_store
            == (*f_info.offset(FEAT_STORE_BLACK as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            price = price * 2 as ::core::ffi::c_int;
        }
    }
    price = (((price * adjust) as ::core::ffi::c_long + 50 as ::core::ffi::c_long)
        / 100 as ::core::ffi::c_long) as ::core::ffi::c_int;
    if !tval_can_have_charges(obj) {
        price *= qty;
    }
    if store_buying as ::core::ffi::c_int != 0
        && price as int32_t > (*proprietor).max_cost * qty as int32_t
    {
        price = ((*proprietor).max_cost * qty as int32_t) as ::core::ffi::c_int;
    }
    if price <= 0 as ::core::ffi::c_int {
        return qty as int32_t;
    }
    return price as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_notice_store() {
    let mut slot: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut store_num: ::core::ffi::c_int = 0;
    let mut o_ptr: *mut object = ::core::ptr::null_mut::<object>();
    let mut list: *mut *mut object = mem_zalloc(
        (::core::mem::size_of::<*mut object>() as size_t)
            .wrapping_mul((*z_info).store_inven_max as size_t),
    ) as *mut *mut object;
    store_num = 0 as ::core::ffi::c_int;
    while store_num < (*z_info).store_max as ::core::ffi::c_int {
        let mut st_ptr: *mut store = stores.offset(store_num as isize) as *mut store;
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).store_inven_max as ::core::ffi::c_int {
            memset(
                (*borg_shops.offset(store_num as isize))
                    .ware
                    .offset(i as isize) as *mut borg_item
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<borg_item>() as size_t,
            );
            i += 1;
        }
        store_stock_list(
            st_ptr,
            list,
            (*z_info).store_inven_max as ::core::ffi::c_int,
        );
        slot = 0 as ::core::ffi::c_int;
        while slot < (*z_info).store_inven_max as ::core::ffi::c_int
            && !(*list.offset(slot as isize)).is_null()
        {
            o_ptr = *list.offset(slot as isize);
            let mut b_item: *mut borg_item = (*borg_shops.offset(store_num as isize))
                .ware
                .offset(slot as isize)
                as *mut borg_item;
            let mut buf: [::core::ffi::c_char; 120] = [0; 120];
            object_desc(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 120]>() as size_t,
                o_ptr,
                (ODESC_FULL as ::core::ffi::c_int | ODESC_STORE as ::core::ffi::c_int) as uint32_t,
                player,
            );
            if strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"(nothing)\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            borg_item_analyze(
                b_item,
                o_ptr,
                &raw mut buf as *mut ::core::ffi::c_char,
                if store_num == BORG_HOME {
                    false_0
                } else {
                    true_0
                } != 0,
            );
            (*b_item).cost =
                borg_price_item(o_ptr, false_0 != 0, 1 as ::core::ffi::c_int, store_num);
            slot += 1;
        }
        memcpy(
            (*borg_safe_shops.offset(store_num as isize)).ware as *mut ::core::ffi::c_void,
            (*borg_shops.offset(store_num as isize)).ware as *const ::core::ffi::c_void,
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        );
        store_num += 1;
    }
    mem_free(list as *mut ::core::ffi::c_void);
    list = ::core::ptr::null_mut::<*mut object>();
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_store() {
    borg_free_store();
    if (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int
        != BORG_HOME
    {
        msg(format(
            b"borg thinks home is %d, game thinks home is %d.  aborting. \0" as *const u8
                as *const ::core::ffi::c_char,
            (*f_info.offset(FEAT_HOME as ::core::ffi::c_int as isize)).shopnum
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            BORG_HOME,
        ));
        borg_init_failure = true_0 != 0;
    }
    borg_shops = mem_zalloc(
        ((*z_info).store_max as size_t).wrapping_mul(::core::mem::size_of::<borg_shop>() as size_t),
    ) as *mut borg_shop;
    borg_safe_shops = mem_zalloc(
        ((*z_info).store_max as size_t).wrapping_mul(::core::mem::size_of::<borg_shop>() as size_t),
    ) as *mut borg_shop;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_max as ::core::ffi::c_int {
        let ref mut fresh0 = (*borg_shops.offset(i as isize)).ware;
        *fresh0 = mem_zalloc(
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        ) as *mut borg_item;
        let ref mut fresh1 = (*borg_safe_shops.offset(i as isize)).ware;
        *fresh1 = mem_zalloc(
            ((*z_info).store_inven_max as size_t)
                .wrapping_mul(::core::mem::size_of::<borg_item>() as size_t),
        ) as *mut borg_item;
        i += 1;
    }
    borg_init_store_sell();
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_store() {
    borg_free_store_sell();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*z_info).store_max as ::core::ffi::c_int {
        if !borg_shops.is_null() && !(*borg_shops.offset(i as isize)).ware.is_null() {
            mem_free((*borg_shops.offset(i as isize)).ware as *mut ::core::ffi::c_void);
            let ref mut fresh2 = (*borg_shops.offset(i as isize)).ware;
            *fresh2 = ::core::ptr::null_mut::<borg_item>();
        }
        if !borg_safe_shops.is_null() && !(*borg_safe_shops.offset(i as isize)).ware.is_null() {
            mem_free((*borg_safe_shops.offset(i as isize)).ware as *mut ::core::ffi::c_void);
            let ref mut fresh3 = (*borg_safe_shops.offset(i as isize)).ware;
            *fresh3 = ::core::ptr::null_mut::<borg_item>();
        }
        i += 1;
    }
    if !borg_safe_shops.is_null() {
        mem_free(borg_safe_shops as *mut ::core::ffi::c_void);
        borg_safe_shops = ::core::ptr::null_mut::<borg_shop>();
    }
    if !borg_shops.is_null() {
        mem_free(borg_shops as *mut ::core::ffi::c_void);
        borg_shops = ::core::ptr::null_mut::<borg_shop>();
    }
}
