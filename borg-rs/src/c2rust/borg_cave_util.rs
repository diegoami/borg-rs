pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
pub use crate::headers::cave_h::{
    cave, chunk, connector, heatmap, monster_group, square, square_in_bounds_fully, trap,
    C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, monster_race, object,
    object_base, object_kind, poss_item,
};
use crate::headers::player_h::monster;

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    angband_term, col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width,
    Term,
};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
#[no_mangle]
pub unsafe extern "C" fn borg_cave_floor_bold(
    mut y: ::core::ffi::c_int,
    mut X: ::core::ffi::c_int,
) -> bool {
    if square_in_bounds_fully(cave, loc(X, y)) {
        if (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
            == FEAT_FLOOR as ::core::ffi::c_int
            || (*borg_grids[y as usize].offset(X as isize)).trap as ::core::ffi::c_int != 0
            || (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
                == FEAT_LESS as ::core::ffi::c_int
            || (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
                == FEAT_MORE as ::core::ffi::c_int
            || (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
                == FEAT_BROKEN as ::core::ffi::c_int
            || (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
                == FEAT_PASS_RUBBLE as ::core::ffi::c_int
            || (*borg_grids[y as usize].offset(X as isize)).feat as ::core::ffi::c_int
                == FEAT_OPEN as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_cave_floor_grid(mut ag: *mut borg_grid) -> bool {
    if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_BROKEN as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_PASS_RUBBLE as ::core::ffi::c_int
        || (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_feature_protected(mut ag: *mut borg_grid) -> bool {
    if (*ag).glyph as ::core::ffi::c_int != 0
        || (*ag).kill as ::core::ffi::c_int != 0
        || (*ag).feat as ::core::ffi::c_int >= FEAT_CLOSED as ::core::ffi::c_int
            && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_panel_hgt() -> ::core::ffi::c_int {
    let mut panel_hgt: ::core::ffi::c_int = 0;
    if Term == angband_term[0 as ::core::ffi::c_int as usize] {
        panel_hgt = ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int;
    } else {
        panel_hgt = (*Term).hgt / tile_height as ::core::ffi::c_int;
    }
    return if panel_hgt < 1 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        panel_hgt
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_panel_wid() -> ::core::ffi::c_int {
    let mut panel_wid: ::core::ffi::c_int = 0;
    if Term == angband_term[0 as ::core::ffi::c_int as usize] {
        panel_wid =
            ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int;
    } else {
        panel_wid = (*Term).wid / tile_width as ::core::ffi::c_int;
    }
    return if panel_wid < 1 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        panel_wid
    };
}
