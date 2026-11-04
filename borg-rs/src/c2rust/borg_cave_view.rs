pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BORG_XTRA: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_cave_util_h::borg_cave_floor_grid;

pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_projection_h::borg_los;
pub use crate::headers::cave_h::{
    cave, chunk, connector, heatmap, monster_group, square, square_in_bounds_fully, trap,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut borg_view_n: int16_t = 0 as int16_t;
#[no_mangle]
pub static mut borg_view_x: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub static mut borg_view_y: [uint8_t; 9000] = [0; 9000];
#[no_mangle]
pub unsafe extern "C" fn borg_forget_view() {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if borg_view_n == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_view_n as ::core::ffi::c_int {
        let mut y: ::core::ffi::c_int = borg_view_y[i as usize] as ::core::ffi::c_int;
        let mut x: ::core::ffi::c_int = borg_view_x[i as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_VIEW) as uint16_t;
        i += 1;
    }
    borg_view_n = 0 as int16_t;
}
unsafe extern "C" fn borg_update_view_aux(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> bool {
    let mut f1: bool = false;
    let mut f2: bool = false;
    let mut v1: bool = false;
    let mut v2: bool = false;
    let mut vis1: bool = false;
    let mut vis2: bool = false;
    let mut wall: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g1_ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut g2_ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    g1_ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y1 as isize)).offset(x1 as isize)
        as *mut borg_grid;
    g2_ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y2 as isize)).offset(x2 as isize)
        as *mut borg_grid;
    f1 = borg_cave_floor_grid(g1_ag);
    f2 = borg_cave_floor_grid(g2_ag);
    if !f1 && !f2 {
        return true_0 != 0;
    }
    v1 = f1 as ::core::ffi::c_int != 0 && (*g1_ag).info as ::core::ffi::c_int & BORG_VIEW != 0;
    v2 = f2 as ::core::ffi::c_int != 0 && (*g2_ag).info as ::core::ffi::c_int & BORG_VIEW != 0;
    if !v1 && !v2 {
        return true_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    wall = !borg_cave_floor_grid(ag);
    vis1 = v1 as ::core::ffi::c_int != 0 && (*g1_ag).info as ::core::ffi::c_int & BORG_XTRA != 0;
    vis2 = v2 as ::core::ffi::c_int != 0 && (*g2_ag).info as ::core::ffi::c_int & BORG_XTRA != 0;
    if vis1 as ::core::ffi::c_int != 0 && vis2 as ::core::ffi::c_int != 0 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if vis1 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if v1 as ::core::ffi::c_int != 0 && v2 as ::core::ffi::c_int != 0 {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if wall {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    if borg_los(borg.c.y, borg.c.x, y, x) {
        (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
        borg_view_y[borg_view_n as usize] = y as uint8_t;
        borg_view_x[borg_view_n as usize] = x as uint8_t;
        borg_view_n += 1;
        return wall;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_update_view() {
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut se: ::core::ffi::c_int = 0;
    let mut sw: ::core::ffi::c_int = 0;
    let mut ne: ::core::ffi::c_int = 0;
    let mut nw: ::core::ffi::c_int = 0;
    let mut es: ::core::ffi::c_int = 0;
    let mut en: ::core::ffi::c_int = 0;
    let mut ws: ::core::ffi::c_int = 0;
    let mut wn: ::core::ffi::c_int = 0;
    let mut full: ::core::ffi::c_int = 0;
    let mut over: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    full = (*z_info).max_sight as ::core::ffi::c_int;
    over = (*z_info).max_sight as ::core::ffi::c_int * 3 as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    while n < borg_view_n as ::core::ffi::c_int {
        y = borg_view_y[n as usize] as ::core::ffi::c_int;
        x = borg_view_x[n as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !(0x20 as ::core::ffi::c_int)) as uint16_t;
        n += 1;
    }
    borg_view_n = 0 as int16_t;
    y = borg.c.y;
    x = borg.c.x;
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
        as *mut borg_grid;
    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
    (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
    borg_view_y[borg_view_n as usize] = y as uint8_t;
    borg_view_x[borg_view_n as usize] = x as uint8_t;
    borg_view_n += 1;
    z = full * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int;
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x + d, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x - d, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x + d, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= z {
        if square_in_bounds_fully(cave, loc(x - d, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x, y + d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y + d) as isize))
                .offset(x as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y + d) as uint8_t;
            borg_view_x[borg_view_n as usize] = x as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    sw = d;
    se = sw;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x, y - d)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset((y - d) as isize))
                .offset(x as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = (y - d) as uint8_t;
            borg_view_x[borg_view_n as usize] = x as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    nw = d;
    ne = nw;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x + d, y)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset((x + d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = y as uint8_t;
            borg_view_x[borg_view_n as usize] = (x + d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    en = d;
    es = en;
    d = 1 as ::core::ffi::c_int;
    while d <= full {
        if square_in_bounds_fully(cave, loc(x - d, y)) {
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset((x - d) as isize) as *mut borg_grid;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_XTRA) as uint16_t;
            (*ag).info = ((*ag).info as ::core::ffi::c_int | BORG_VIEW) as uint16_t;
            borg_view_y[borg_view_n as usize] = y as uint8_t;
            borg_view_x[borg_view_n as usize] = (x - d) as uint8_t;
            borg_view_n += 1;
            if !borg_cave_floor_grid(ag) {
                break;
            }
        }
        d += 1;
    }
    wn = d;
    ws = wn;
    n = 1 as ::core::ffi::c_int;
    while n <= over / 2 as ::core::ffi::c_int {
        let mut ypn: ::core::ffi::c_int = 0;
        let mut ymn: ::core::ffi::c_int = 0;
        let mut xpn: ::core::ffi::c_int = 0;
        let mut xmn: ::core::ffi::c_int = 0;
        z = over - n - n;
        if z > full - n {
            z = full - n;
        }
        while z + n + (n >> 1 as ::core::ffi::c_int) > full {
            z -= 1;
        }
        ypn = y + n;
        ymn = y - n;
        xpn = x + n;
        xmn = x - n;
        if ypn < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
            m = if z > 66 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - ypn {
                66 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - ypn
            } else {
                z
            };
            if xpn <= AUTO_MAX_X - 1 as ::core::ffi::c_int && n < se {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn + d,
                        xpn,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xpn - 1 as ::core::ffi::c_int,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xpn,
                    ) {
                        if n + d >= se {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                se = k + 1 as ::core::ffi::c_int;
            }
            if xmn >= 0 as ::core::ffi::c_int && n < sw {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn + d,
                        xmn,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xmn + 1 as ::core::ffi::c_int,
                        ypn + d - 1 as ::core::ffi::c_int,
                        xmn,
                    ) {
                        if n + d >= sw {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                sw = k + 1 as ::core::ffi::c_int;
            }
        }
        if ymn > 0 as ::core::ffi::c_int {
            m = if z > ymn { ymn } else { z };
            if xpn <= AUTO_MAX_X - 1 as ::core::ffi::c_int && n < ne {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn - d,
                        xpn,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xpn - 1 as ::core::ffi::c_int,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xpn,
                    ) {
                        if n + d >= ne {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                ne = k + 1 as ::core::ffi::c_int;
            }
            if xmn >= 0 as ::core::ffi::c_int && n < nw {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn - d,
                        xmn,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xmn + 1 as ::core::ffi::c_int,
                        ymn - d + 1 as ::core::ffi::c_int,
                        xmn,
                    ) {
                        if n + d >= nw {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                nw = k + 1 as ::core::ffi::c_int;
            }
        }
        if xpn < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            m = if z > 198 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - xpn {
                198 as ::core::ffi::c_int - 1 as ::core::ffi::c_int - xpn
            } else {
                z
            };
            if ypn <= AUTO_MAX_Y - 1 as ::core::ffi::c_int && n < es {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn,
                        xpn + d,
                        ypn - 1 as ::core::ffi::c_int,
                        xpn + d - 1 as ::core::ffi::c_int,
                        ypn,
                        xpn + d - 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= es {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                es = k + 1 as ::core::ffi::c_int;
            }
            if ymn >= 0 as ::core::ffi::c_int && n < en {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn,
                        xpn + d,
                        ymn + 1 as ::core::ffi::c_int,
                        xpn + d - 1 as ::core::ffi::c_int,
                        ymn,
                        xpn + d - 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= en {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                en = k + 1 as ::core::ffi::c_int;
            }
        }
        if xmn > 0 as ::core::ffi::c_int {
            m = if z > xmn { xmn } else { z };
            if ypn <= AUTO_MAX_Y - 1 as ::core::ffi::c_int && n < ws {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ypn,
                        xmn - d,
                        ypn - 1 as ::core::ffi::c_int,
                        xmn - d + 1 as ::core::ffi::c_int,
                        ypn,
                        xmn - d + 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= ws {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                ws = k + 1 as ::core::ffi::c_int;
            }
            if ymn >= 0 as ::core::ffi::c_int && n < wn {
                k = n;
                d = 1 as ::core::ffi::c_int;
                while d <= m {
                    if borg_update_view_aux(
                        ymn,
                        xmn - d,
                        ymn + 1 as ::core::ffi::c_int,
                        xmn - d + 1 as ::core::ffi::c_int,
                        ymn,
                        xmn - d + 1 as ::core::ffi::c_int,
                    ) {
                        if n + d >= wn {
                            break;
                        }
                    } else {
                        k = n + d;
                    }
                    d += 1;
                }
                wn = k + 1 as ::core::ffi::c_int;
            }
        }
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < borg_view_n as ::core::ffi::c_int {
        y = borg_view_y[n as usize] as ::core::ffi::c_int;
        x = borg_view_x[n as usize] as ::core::ffi::c_int;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        (*ag).info = ((*ag).info as ::core::ffi::c_int & !BORG_XTRA) as uint16_t;
        n += 1;
    }
}
