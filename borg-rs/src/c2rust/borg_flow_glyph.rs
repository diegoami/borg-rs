pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GOAL_MISC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
pub use crate::headers::borg_flow_h::{
    borg_data, borg_data_cost, borg_ddx_ddd, borg_ddy_ddd, borg_flow_clear, borg_flow_commit,
    borg_flow_enqueue_grid, borg_flow_old, borg_flow_spread, borg_free_track, borg_init_track,
    borg_track,
};
use crate::headers::borg_flow_misc_h::borg_flow_reverse;
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::borg_note;
use crate::headers::borg_item_activation_h::act_glyph;
use crate::headers::borg_item_use_h::{borg_activate_item, borg_read_scroll};
use crate::headers::borg_item_val_h::sv_scroll_rune_of_protection;
pub use crate::headers::borg_magic_h::{
    borg_spell_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS,
    BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION,
    CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS,
    CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE, DETECTION,
    DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR,
    DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL, DISPEL_LIFE, DISPEL_UNDEAD,
    DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC, ENCHANT_ARMOUR, ENCHANT_WEAPON,
    EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW, FOX_FORM, FRIGHTEN, FROST_BOLT,
    FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW, HASTE_SELF, HEALING,
    HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED, ICE_STORM, IDENTIFY_RUNE,
    LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM, MAGIC_MISSILE, MAIM_FOE,
    MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP, METEOR_SWARM, MINOR_HEALING,
    NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR, PORTAL, POWER_SACRIFICE,
    PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS, RECHARGING, RELENTLESS_TAUNTING,
    REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE, RESIST_POISON, RESTORATION,
    REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE, SENSE_INVISIBLE,
    SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT, SLEEP_EVIL,
    SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD, STORM_OF_DARKNESS,
    TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER, TELEPORT_SELF, THRUST_AWAY,
    TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD, UNHOLY_REPRIEVE, UNLEASH_CHAOS,
    VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION, WARG_FORM, WEREWOLF_FORM,
    WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL, ZONE_OF_UNMAGIC,
};

pub use crate::headers::cave_h::{
    chunk, connector, distance, heatmap, monster_group, square, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::list_terrain_h::{
    FEAT_BROKEN, FEAT_CLOSED, FEAT_FLOOR, FEAT_GRANITE, FEAT_HOME, FEAT_LAVA, FEAT_LESS,
    FEAT_MAGMA, FEAT_MAGMA_K, FEAT_MORE, FEAT_NONE, FEAT_OPEN, FEAT_PASS_RUBBLE, FEAT_PERM,
    FEAT_QUARTZ, FEAT_QUARTZ_K, FEAT_RUBBLE, FEAT_SECRET, FEAT_STORE_ALCHEMY, FEAT_STORE_ARMOR,
    FEAT_STORE_BLACK, FEAT_STORE_BOOK, FEAT_STORE_GENERAL, FEAT_STORE_MAGIC, FEAT_STORE_WEAPON,
};
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
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
static mut glyph_x: uint8_t = 0;
static mut glyph_y: uint8_t = 0;
static mut glyph_y_center: uint8_t = 0 as uint8_t;
static mut glyph_x_center: uint8_t = 0 as uint8_t;
#[no_mangle]
pub static mut track_glyph: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut borg_needs_new_sea: bool = false;
#[no_mangle]
pub unsafe extern "C" fn borg_flow_glyph() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut cost: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_x: ::core::ffi::c_int = borg.c.x;
    let mut b_y: ::core::ffi::c_int = borg.c.y;
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut goal_glyph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut glyph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if glyph_y_center as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && glyph_x_center as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || distance(
            borg.c,
            loc(
                glyph_x_center as ::core::ffi::c_int,
                glyph_y_center as ::core::ffi::c_int,
            ),
        ) >= 50 as ::core::ffi::c_int
    {
        borg_needs_new_sea = true_0 != 0;
    }
    if glyph_x as ::core::ffi::c_int == borg.c.x && glyph_y as ::core::ffi::c_int == borg.c.y {
        glyph_x = 0 as uint8_t;
        glyph_y = 0 as uint8_t;
        if borg_needs_new_sea {
            glyph_y_center = borg.c.y as uint8_t;
            glyph_x_center = borg.c.x as uint8_t;
        }
        borg_needs_new_sea = false_0 != 0;
        borg_note(format(
            b"# Glyph Creating at (%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
            borg.c.x,
            borg.c.y,
        ));
        if borg_spell_fail(GLYPH_OF_WARDING, 30 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_rune_of_protection) as ::core::ffi::c_int != 0
            || borg_activate_item(act_glyph) as ::core::ffi::c_int != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < track_glyph.num as ::core::ffi::c_int {
                if *track_glyph.x.offset(i as isize) == borg.c.x
                    && *track_glyph.y.offset(i as isize) == borg.c.y
                {
                    return false_0 != 0;
                }
                i += 1;
            }
            if (track_glyph.num as ::core::ffi::c_int) < track_glyph.size as ::core::ffi::c_int {
                borg_note(
                    b"# Noting the creation of a glyph.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                *track_glyph.x.offset(track_glyph.num as isize) = borg.c.x;
                *track_glyph.y.offset(track_glyph.num as isize) = borg.c.y;
                track_glyph.num += 1;
            }
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    borg_flow_reverse(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    y = 15 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 15 as ::core::ffi::c_int {
        x = 50 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 50 as ::core::ffi::c_int {
            let mut ag_ptr: [*mut borg_grid; 24] = [::core::ptr::null_mut::<borg_grid>(); 24];
            let mut floor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp_glyph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !((*ag).feat as ::core::ffi::c_int != FEAT_FLOOR as ::core::ffi::c_int
                && (*ag).glyph as ::core::ffi::c_int != 0)
            {
                cost = (*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int;
                if !(cost >= 75 as ::core::ffi::c_int) {
                    i = 0 as ::core::ffi::c_int;
                    while i < 24 as ::core::ffi::c_int {
                        let mut xx: ::core::ffi::c_int =
                            x + borg_ddx_ddd[i as usize] as ::core::ffi::c_int;
                        let mut yy: ::core::ffi::c_int =
                            y + borg_ddy_ddd[i as usize] as ::core::ffi::c_int;
                        ag_ptr[i as usize] =
                            (*(&raw mut borg_grids as *mut *mut borg_grid).offset(yy as isize))
                                .offset(xx as isize) as *mut borg_grid;
                        i += 1;
                    }
                    if borg_needs_new_sea {
                        goal_glyph = 24 as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i < 24 as ::core::ffi::c_int {
                            ag = ag_ptr[i as usize];
                            if (*ag).feat as ::core::ffi::c_int == FEAT_FLOOR as ::core::ffi::c_int
                                || (*ag).glyph as ::core::ffi::c_int != 0
                            {
                                floor += 1;
                            }
                            i += 1;
                        }
                        if !(floor != 24 as ::core::ffi::c_int) {
                            i = 0 as ::core::ffi::c_int;
                            while i < 24 as ::core::ffi::c_int {
                                ag = ag_ptr[i as usize];
                                if (*ag).glyph {
                                    tmp_glyph += 1;
                                }
                                i += 1;
                            }
                            v = 100 as ::core::ffi::c_int + tmp_glyph * 500 as ::core::ffi::c_int
                                - cost * 1 as ::core::ffi::c_int;
                            if (*borg_grids[y as usize].offset(x as isize)).feat
                                as ::core::ffi::c_int
                                == FEAT_FLOOR as ::core::ffi::c_int
                            {
                                v += 3000 as ::core::ffi::c_int;
                            }
                            if tmp_glyph == goal_glyph {
                                v += 5000 as ::core::ffi::c_int;
                            }
                            if tmp_glyph != goal_glyph
                                && (*borg_grids[y as usize].offset(x as isize)).glyph
                                    as ::core::ffi::c_int
                                    != 0
                            {
                                v = -(1 as ::core::ffi::c_int);
                            }
                            if !(v <= 0 as ::core::ffi::c_int) {
                                if !(b_v >= 0 as ::core::ffi::c_int && v < b_v) {
                                    b_v = v;
                                    b_x = x;
                                    b_y = y;
                                }
                            }
                        }
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 24 as ::core::ffi::c_int {
                            if !(glyph_x_center as ::core::ffi::c_int
                                + borg_ddx_ddd[i as usize] as ::core::ffi::c_int
                                != x)
                            {
                                if !(glyph_y_center as ::core::ffi::c_int
                                    + borg_ddy_ddd[i as usize] as ::core::ffi::c_int
                                    != y)
                                {
                                    if !(*borg_grids[y as usize].offset(x as isize)).glyph {
                                        v = 500 as ::core::ffi::c_int
                                            + tmp_glyph * 500 as ::core::ffi::c_int
                                            - cost * 1 as ::core::ffi::c_int;
                                        if !(v <= 0 as ::core::ffi::c_int) {
                                            if !(b_v >= 0 as ::core::ffi::c_int && v < b_v) {
                                                b_v = v;
                                                b_x = x;
                                                b_y = y;
                                            }
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if glyph_y_center as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && glyph_x_center as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < 24 as ::core::ffi::c_int {
            let mut xx_0: ::core::ffi::c_int = glyph_x_center as ::core::ffi::c_int
                + borg_ddx_ddd[i as usize] as ::core::ffi::c_int;
            let mut yy_0: ::core::ffi::c_int = glyph_y_center as ::core::ffi::c_int
                + borg_ddy_ddd[i as usize] as ::core::ffi::c_int;
            let mut ag_ptr_0: [*mut borg_grid; 24] = [::core::ptr::null_mut::<borg_grid>(); 24];
            ag_ptr_0[i as usize] = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(yy_0 as isize))
            .offset(xx_0 as isize) as *mut borg_grid;
            ag = ag_ptr_0[i as usize];
            if (*ag).glyph {
                glyph += 1;
            }
            if glyph == 24 as ::core::ffi::c_int {
                b_v = 5000 as ::core::ffi::c_int;
                b_x = glyph_x_center as ::core::ffi::c_int;
                b_y = glyph_y_center as ::core::ffi::c_int;
            }
            i += 1;
        }
    }
    borg_flow_clear();
    if b_v < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(b_y as isize)).offset(b_x as isize)
        as *mut borg_grid;
    glyph_x = b_x as uint8_t;
    glyph_y = b_y as uint8_t;
    borg_flow_enqueue_grid(b_y, b_x);
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"Glyph\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_MISC,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_MISC) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_glyph() {
    borg_init_track(&raw mut track_glyph, 200 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_glyph() {
    borg_free_track(&raw mut track_glyph);
}
