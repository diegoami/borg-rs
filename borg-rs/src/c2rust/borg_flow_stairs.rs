pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
pub use crate::headers::borg_flow_h::{
    borg_data, borg_data_cost, borg_flow_clear, borg_flow_commit, borg_flow_enqueue_grid,
    borg_flow_old, borg_flow_spread, borg_free_track, borg_init_track, borg_track,
};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::borg_magic_h::{
    borg_heroism_level, borg_spell_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
    BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS,
    BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS,
    CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE, DARK_SPEAR, DECOY, DEMON_BANE,
    DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS, DETECT_STAIRS, DIMENSION_DOOR,
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
use crate::headers::borg_prepared_h::borg_prepared;
use crate::headers::borg_store_sell_h::borg_count_sell;
pub use crate::headers::borg_trait_h::{
    C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS, BI_ADETDOOR,
    BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM, BI_AENCH_SWEP,
    BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL, BI_AGLYPH, BI_AHEAL, BI_AHWORD,
    BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID, BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES,
    BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL, BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES,
    BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE, BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS,
    BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED, BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR,
    BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS, BI_BART, BI_BID, BI_BLOWS, BI_BTODAM,
    BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX, BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON,
    BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY, BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN,
    BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING, BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU,
    BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP, BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM,
    BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN, BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND,
    BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO, BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP,
    BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX, BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY,
    BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1, BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD,
    BI_FOOD_CURE_BLIND, BI_FOOD_CURE_CONF, BI_FOOD_HI, BI_FOOD_LO, BI_FRACT, BI_GOLD,
    BI_GOOD_S_CHG, BI_GOOD_W_CHG, BI_HASFIXEXP, BI_HEAVYBOW, BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ,
    BI_IACID, BI_ICOLD, BI_IELEC, BI_IFIRE, BI_INFRA, BI_INT, BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID,
    BI_ISBLIND, BI_ISCONFUSED, BI_ISCUT, BI_ISENCUMB, BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX,
    BI_ISFIXEXP, BI_ISFIXINT, BI_ISFIXLEV, BI_ISFIXSTR, BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL,
    BI_ISGORGED, BI_ISHEAVYSTUN, BI_ISHUNGRY, BI_ISIMAGE, BI_ISPARALYZED, BI_ISPOISONED,
    BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK, BI_KING, BI_LIGHT, BI_MAX, BI_MAXCLEVEL, BI_MAXDEPTH,
    BI_MAXHP, BI_MAXSP, BI_MOD_MOVES, BI_MULTIPLE_BONUSES, BI_NEED_BRAND_WEAPON,
    BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D, BI_NEED_ENCHANT_TO_H, BI_PREP_BIG_FIGHT,
    BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD, BI_RCONF, BI_RDARK, BI_RDIS, BI_RECALL, BI_REG,
    BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS, BI_RLITE, BI_RNTHR, BI_RNXUS, BI_RPOIS, BI_RSHRD,
    BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON, BI_SDEX, BI_SDIG, BI_SFRACT, BI_SHLIFE, BI_SHOTS,
    BI_SINT, BI_SINV, BI_SLING, BI_SPEED, BI_SP_ADJ, BI_SRACID, BI_SRBLIND, BI_SRCH, BI_SRCOLD,
    BI_SRCONF, BI_SRDARK, BI_SRDIS, BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS, BI_SRLITE,
    BI_SRNTHR, BI_SRNXUS, BI_SRPOIS, BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR, BI_STR_INDEX,
    BI_SWIS, BI_THB, BI_THN, BI_THT, BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD, BI_WB_ELEC,
    BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS, BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS,
    BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON, BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL,
    BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL, BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD,
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
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
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
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
#[no_mangle]
pub static mut track_less: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub static mut track_more: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub unsafe extern "C" fn borg_flow_cost_stair(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut b_stair: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cost: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    borg_flow_clear();
    if b_stair == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    borg_flow_enqueue_grid(
        *track_less.y.offset(b_stair as isize),
        *track_less.x.offset(b_stair as isize),
    );
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
        false_0 != 0,
        b_stair,
        false_0 != 0,
    );
    cost = (*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int;
    if cost == 255 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return cost;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_stair_both(
    mut why: ::core::ffi::c_int,
    mut sneak: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if track_less.num == 0 && track_more.num == 0 {
        return false_0 != 0;
    }
    if !borg.goal.fleeing
        && !borg.mon.scary
        && track_less.num == 0
        && borg.avoidance as ::core::ffi::c_int
            <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                * 15 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        && (*borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
            || *borg
                .trait_0
                .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
                != 0
            || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                < 2 as ::core::ffi::c_int)
    {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && borg.munchkin_mode as ::core::ffi::c_int == false_0
    {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        if !((*borg_grids[*track_less.y.offset(i as isize) as usize]
            .offset(*track_less.x.offset(i as isize) as isize))
        .kill
            != 0)
        {
            borg_flow_enqueue_grid(
                *track_less.y.offset(i as isize),
                *track_less.x.offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_more.num as ::core::ffi::c_int {
        if !((*borg_grids[*track_more.y.offset(i as isize) as usize]
            .offset(*track_more.x.offset(i as isize) as isize))
        .kill
            != 0)
        {
            borg_flow_enqueue_grid(
                *track_more.y.offset(i as isize),
                *track_more.x.offset(i as isize),
            );
        }
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        false_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        sneak,
    );
    if !borg_flow_commit(b"stairs\0" as *const u8 as *const ::core::ffi::c_char, why) {
        return false_0 != 0;
    }
    if !borg_flow_old(why) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_stair_less(
    mut why: ::core::ffi::c_int,
    mut sneak: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
        return false_0 != 0;
    }
    if track_less.num == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        if !((*borg_grids[*track_less.y.offset(i as isize) as usize]
            .offset(*track_less.x.offset(i as isize) as isize))
        .kill
            != 0)
        {
            borg_flow_enqueue_grid(
                *track_less.y.offset(i as isize),
                *track_less.x.offset(i as isize),
            );
        }
        i += 1;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        > 35 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        borg_flow_spread(
            250 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            sneak,
        );
    } else {
        borg_flow_spread(
            250 as ::core::ffi::c_int,
            false_0 != 0,
            !borg.status.desperate,
            false_0 != 0,
            -(1 as ::core::ffi::c_int),
            sneak,
        );
    }
    if !borg_flow_commit(
        b"up-stairs\0" as *const u8 as *const ::core::ffi::c_char,
        why,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(why) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_stair_more(
    mut why: ::core::ffi::c_int,
    mut sneak: bool,
    mut brave: bool,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if track_more.num == 0 {
        return false_0 != 0;
    }
    if track_less.num != 0 {
        if !borg.lunal_mode
            && !borg.munchkin_mode
            && !brave
            && !(borg_prepared(
                *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    + 1 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char)
                .is_null()
        {
            return false_0 != 0;
        }
        if !brave
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 0
            && !borg.mon.scary
            && (*borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
                || *borg
                    .trait_0
                    .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
                    != 0
                || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                    < 2 as ::core::ffi::c_int)
        {
            return false_0 != 0;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
            && *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 25 as ::core::ffi::c_int
            && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
                < 25000 as ::core::ffi::c_int
            && borg_count_sell() >= 13 as ::core::ffi::c_int
            && !borg.munchkin_mode
        {
            return false_0 != 0;
        }
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && borg.munchkin_mode as ::core::ffi::c_int == false_0
        {
            return false_0 != 0;
        }
    }
    if borg.goal.recalling != 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < track_more.num as ::core::ffi::c_int {
        if !((*borg_grids[*track_more.y.offset(i as isize) as usize]
            .offset(*track_more.x.offset(i as isize) as isize))
        .kill
            != 0)
        {
            borg_flow_enqueue_grid(
                *track_more.y.offset(i as isize),
                *track_more.x.offset(i as isize),
            );
        }
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        sneak,
    );
    if !borg_flow_commit(
        b"down-stairs\0" as *const u8 as *const ::core::ffi::c_char,
        why,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(why) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_prep_leave_level_spells() -> bool {
    if borg.goal.fleeing {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
        < *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) * 6 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if !borg.temp.fast
        && borg_spell_fail(HASTE_SELF, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting speed spell before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 5000 as int16_t;
        return true_0 != 0;
    }
    if (borg.temp.res_fire as ::core::ffi::c_int
        + borg.temp.res_acid as ::core::ffi::c_int
        + borg.temp.res_elec as ::core::ffi::c_int
        + borg.temp.res_cold as ::core::ffi::c_int)
        < 3 as ::core::ffi::c_int
        && borg_spell_fail(RESISTANCE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Resistance spell before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 21000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.fastcast
        && borg_spell_fail(MANA_CHANNEL, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Mana Channel spell before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 6000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.berserk
        && borg_spell_fail(BERSERK_STRENGTH, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Berserk Strength spell before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 10000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.hero
        && borg_spell_fail(HEROISM, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            > borg_heroism_level()
    {
        borg_note(
            b"# Casting Heroism spell before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 3000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.regen
        && borg_spell_fail(RAPID_REGENERATION, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Regen before leaving level.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 6000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.smite_evil
        && *borg
            .trait_0
            .offset(BI_WS_EVIL as ::core::ffi::c_int as isize)
            == 0
        && borg_spell_fail(SMITE_EVIL, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Smite Evil before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 21000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.venom
        && *borg
            .trait_0
            .offset(BI_WB_POIS as ::core::ffi::c_int as isize)
            == 0
        && borg_spell_fail(VENOM, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        borg_note(
            b"# Casting Venom before leaving level.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 18000 as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.prot_from_evil
        && borg_spell_fail(PROTECTION_FROM_EVIL, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0
    {
        borg_note(
            b"# Casting PFE before leaving level.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = (*borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 1000 as ::core::ffi::c_int) as int16_t;
        return true_0 != 0;
    }
    if !borg.temp.bless
        && (borg_spell_fail(BLESS, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(DEMON_BANE, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0)
    {
        borg_note(
            b"# Casting blessing before leaving level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.no_rest_prep = 11000 as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_stairs() {
    borg_init_track(&raw mut track_less, 16 as ::core::ffi::c_int);
    borg_init_track(&raw mut track_more, 16 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_stairs() {
    borg_free_track(&raw mut track_more);
    borg_free_track(&raw mut track_less);
}
