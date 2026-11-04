pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_MISC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const GOAL_XTRA: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GOAL_VAULT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GOAL_RECOVER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_cave_light_h::{borg_glow_n, borg_glow_x, borg_glow_y};
use crate::headers::borg_cave_util_h::{
    borg_cave_floor_bold, borg_cave_floor_grid, borg_panel_hgt, borg_panel_wid,
};

use crate::headers::borg_danger_h::{
    borg_danger, borg_danger_one_kill, borg_fear_monsters, borg_fear_region,
};
use crate::headers::borg_escape_h::{borg_allow_teleport, borg_caution_phase, borg_shadow_shift};
pub use crate::headers::borg_flow_h::{
    borg_can_dig, borg_data, borg_data_cost, borg_flow_clear, borg_flow_commit,
    borg_flow_enqueue_grid, borg_flow_old, borg_flow_spread, borg_free_track, borg_init_track,
    borg_temp_n, borg_temp_x, borg_temp_y, borg_track, track_step,
};
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_kills_nxt};
use crate::headers::borg_flow_stairs_h::{borg_flow_cost_stair, track_less};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, w_x, w_y, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{act_tele_long, act_tele_phase};
use crate::headers::borg_item_use_h::{borg_activate_item, borg_read_scroll};
use crate::headers::borg_item_val_h::sv_scroll_phase_door;
pub use crate::headers::borg_magic_h::{
    borg_primarily_caster, borg_spell_fail, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_projection_h::{borg_distance, borg_los};
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
use crate::headers::borg_update_h::borg_detect_door;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx, ddx_ddd, ddy, ddy_ddd, distance, f_info, feature, heatmap,
    monster_group, square, square_in_bounds, square_in_bounds_fully, trap, C2RustUnnamedH_18,
    FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_mon_race_flags_h::{
    RF_ANIMAL, RF_ATTR_CLEAR, RF_ATTR_FLICKER, RF_ATTR_MULTI, RF_ATTR_RAND, RF_BASH_DOOR,
    RF_CHAR_CLEAR, RF_CLEAR_WEB, RF_COLD_BLOOD, RF_DEMON, RF_DRAGON, RF_DROP_1, RF_DROP_2,
    RF_DROP_20, RF_DROP_3, RF_DROP_4, RF_DROP_40, RF_DROP_60, RF_DROP_GOOD, RF_DROP_GREAT,
    RF_EMPTY_MIND, RF_EVIL, RF_FEMALE, RF_FORCE_DEPTH, RF_FORCE_EXTRA, RF_FORCE_SLEEP,
    RF_FRIGHTENED, RF_GIANT, RF_GROUP_AI, RF_HURT_COLD, RF_HURT_FIRE, RF_HURT_LIGHT, RF_HURT_ROCK,
    RF_IM_ACID, RF_IM_COLD, RF_IM_DISEN, RF_IM_ELEC, RF_IM_FIRE, RF_IM_NETHER, RF_IM_NEXUS,
    RF_IM_PLASMA, RF_IM_POIS, RF_IM_WATER, RF_INVISIBLE, RF_KILL_BODY, RF_KILL_ITEM, RF_KILL_WALL,
    RF_MALE, RF_METAL, RF_MIMIC_INV, RF_MOVE_BODY, RF_MULTIPLY, RF_NAME_COMMA, RF_NEVER_BLOW,
    RF_NEVER_MOVE, RF_NONE, RF_NONLIVING, RF_NO_CONF, RF_NO_FEAR, RF_NO_HOLD, RF_NO_SLEEP,
    RF_NO_SLOW, RF_NO_STUN, RF_ONLY_GOLD, RF_ONLY_ITEM, RF_OPEN_DOOR, RF_ORC, RF_PASS_WALL,
    RF_PASS_WEB, RF_POWERFUL, RF_QUESTOR, RF_RAND_25, RF_RAND_50, RF_REGENERATE, RF_SEASONAL,
    RF_SMART, RF_SMASH_WALL, RF_SPIRIT, RF_STUPID, RF_TAKE_ITEM, RF_TROLL, RF_UNAWARE, RF_UNDEAD,
    RF_UNIQUE, RF_WEIRD_MIND,
};
pub use crate::headers::list_player_flags_h::{
    PF_BEAM, PF_BLESS_WEAPON, PF_BRAVERY_30, PF_CHARM, PF_CHOOSE_SPELLS, PF_COMBAT_REGEN, PF_EVIL,
    PF_FAST_SHOT, PF_KNOW_MUSHROOM, PF_KNOW_ZAPPER, PF_NONE, PF_NO_MANA, PF_ROCK, PF_SEE_ORE,
    PF_SHIELD_BASH, PF_STEAL, PF_UNLIGHT, PF_ZERO_FAIL,
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
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape, r_info,
    C2RustUnnamedH_29, MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD,
    MON_GROUP_LEADER, MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON, RF_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_34, PF_MAX,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::store_h::{object_buy, owner, store, stores};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width, Term,
};
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
use crate::headers::z_virt_h::{mem_free, mem_zalloc};
#[no_mangle]
pub static mut track_shop_x: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut track_shop_y: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut track_vein: borg_track = borg_track {
    num: 0,
    size: 0,
    x: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    y: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
};
#[no_mangle]
pub unsafe extern "C" fn borg_flow_reverse(
    mut depth: ::core::ffi::c_int,
    mut optimize: bool,
    mut avoid: bool,
    mut tunneling: bool,
    mut stair_idx: ::core::ffi::c_int,
    mut sneak: bool,
) {
    borg_flow_clear();
    borg_flow_enqueue_grid(borg.c.y, borg.c.x);
    borg_flow_spread(depth, optimize, avoid, tunneling, stair_idx, sneak);
}
#[no_mangle]
pub unsafe extern "C" fn borg_get_leash(mut pick_up: bool) -> ::core::ffi::c_int {
    let mut leash: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
    if pick_up as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        leash = *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 3 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int;
    }
    if !pick_up
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 5 as ::core::ffi::c_int
    {
        leash = *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            * 3 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int;
    }
    if borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
        leash += borg.times_twitch as ::core::ffi::c_int;
    }
    return leash;
}
#[no_mangle]
pub unsafe extern "C" fn borg_happy_grid_bold(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
        .offset(x as isize) as *mut borg_grid;
    if y >= DUNGEON_HGT - 2 as ::core::ffi::c_int
        || y <= 2 as ::core::ffi::c_int
        || x >= DUNGEON_WID - 2 as ::core::ffi::c_int
        || x <= 2 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if (*ag).glyph {
        return true_0 != 0;
    }
    if (*ag).feat as ::core::ffi::c_int == FEAT_LAVA as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
        || *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.level >= 2000 as borg_time {
        return false_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        && !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y - 2 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x)
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 2 as ::core::ffi::c_int, x) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y, x + 1 as ::core::ffi::c_int)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y, x + 2 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !borg_cave_floor_bold(y, x - 1 as ::core::ffi::c_int)
        && borg_cave_floor_bold(y - 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y + 1 as ::core::ffi::c_int, x - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
        && borg_cave_floor_bold(y, x - 2 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_step.num as ::core::ffi::c_int {
        if *track_step.y.offset(i as isize) == y && *track_step.x.offset(i as isize) == x {
            if i < 25 as ::core::ffi::c_int {
                return true_0 != 0;
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_recover(mut dist: ::core::ffi::c_int) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if borg.antibounce_count as ::core::ffi::c_int > 500 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        <= 5 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_primarily_caster() {
        if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
            && (*borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
                > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                    / 4 as ::core::ffi::c_int
                || *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
            && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
            && *borg
                .trait_0
                .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
                == 0
            && *borg
                .trait_0
                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                == 0
        {
            return false_0 != 0;
        }
    } else if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        > *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) / 3 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) == 0
        && *borg
            .trait_0
            .offset(BI_ISSTUN as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISHEAVYSTUN as ::core::ffi::c_int as isize)
            == 0
        && *borg
            .trait_0
            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
            == 0
    {
        return false_0 != 0;
    }
    if borg.goal.fleeing {
        return false_0 != 0;
    }
    if borg.lunal_mode as ::core::ffi::c_int != 0 || borg.munchkin_mode as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    y = borg.c.y - 25 as ::core::ffi::c_int;
    while y < borg.c.y + 25 as ::core::ffi::c_int {
        x = borg.c.x - 25 as ::core::ffi::c_int;
        while x < borg.c.x + 25 as ::core::ffi::c_int {
            if square_in_bounds(cave, loc(x, y)) {
                if !(y == borg.c.y && x == borg.c.x) {
                    if !(distance(borg.c, loc(x, y)) < 7 as ::core::ffi::c_int) {
                        if borg_happy_grid_bold(y, x) {
                            if !((*borg_grids[y as usize].offset(x as isize)).feat
                                as ::core::ffi::c_int
                                >= FEAT_SECRET as ::core::ffi::c_int
                                && (*borg_grids[y as usize].offset(x as isize)).feat
                                    as ::core::ffi::c_int
                                    != FEAT_PASS_RUBBLE as ::core::ffi::c_int)
                            {
                                if borg_check_rest(y, x) {
                                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                    borg_temp_n += 1;
                                }
                            }
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        dist,
        false_0 != 0,
        true_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"Recover Grid\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_RECOVER,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_RECOVER) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_vein(
    mut viewable: bool,
    mut nearness: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut leash: ::core::ffi::c_int = borg_get_leash(true_0 != 0);
    let mut min_feat: uint8_t = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if track_vein.num == 0 {
        return false_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) >= 100000 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    min_feat = FEAT_QUARTZ_K as ::core::ffi::c_int as uint8_t;
    if borg.times_twitch as ::core::ffi::c_int > 21 as ::core::ffi::c_int {
        min_feat = FEAT_MAGMA_K as ::core::ffi::c_int as uint8_t;
    }
    if !borg_can_dig(true_0 != 0, min_feat) {
        return false_0 != 0;
    }
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    let mut current_block_25: u64;
    i = 0 as ::core::ffi::c_int;
    while i < track_vein.num as ::core::ffi::c_int {
        x = *track_vein.x.offset(i as isize);
        y = *track_vein.y.offset(i as isize);
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize)).offset(x as isize)
            as *mut borg_grid;
        if !(viewable as ::core::ffi::c_int != 0
            && (*ag).info as ::core::ffi::c_int & BORG_VIEW == 0)
        {
            borg_flow_clear();
            if nearness > 5 as ::core::ffi::c_int
                && *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    < 20 as ::core::ffi::c_int
            {
                cost = borg_flow_cost_stair(y, x, b_stair);
                if cost > leash {
                    current_block_25 = 5143058163439228106;
                } else {
                    current_block_25 = 10043043949733653460;
                }
            } else {
                current_block_25 = 10043043949733653460;
            }
            match current_block_25 {
                5143058163439228106 => {}
                _ => {
                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                    borg_temp_n += 1;
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        nearness,
        true_0 != 0,
        !viewable,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"vein\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_TAKE,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_TAKE) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
static mut spastic_x: uint8_t = 0;
static mut spastic_y: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn borg_flow_spastic(mut bored: bool) -> bool {
    let mut cost: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut b_x: ::core::ffi::c_int = borg.c.x;
    let mut b_y: ::core::ffi::c_int = borg.c.y;
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_stair: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.level > 3000 as borg_time
        && borg.avoidance as ::core::ffi::c_int
            <= *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        return false_0 != 0;
    }
    if !bored {
        let mut p: ::core::ffi::c_int = borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        );
        if p > borg.avoidance as ::core::ffi::c_int / 4 as ::core::ffi::c_int {
            return false_0 != 0;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < track_less.num as ::core::ffi::c_int {
        x = *track_less.x.offset(i as isize);
        y = *track_less.y.offset(i as isize);
        j = distance(borg.c, loc(x, y));
        if !(b_j >= j) {
            b_j = j;
            b_stair = i;
        }
        i += 1;
    }
    if spastic_x as ::core::ffi::c_int == borg.c.x && spastic_y as ::core::ffi::c_int == borg.c.y {
        spastic_x = 0 as uint8_t;
        spastic_y = 0 as uint8_t;
        ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.c.y as isize))
            .offset(borg.c.x as isize) as *mut borg_grid;
        borg_note(format(
            b"# Spastic Searching at (%d,%d)...value:%d\0" as *const u8
                as *const ::core::ffi::c_char,
            borg.c.x,
            borg.c.y,
            (*ag).xtra as ::core::ffi::c_int,
        ));
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            let mut xx: ::core::ffi::c_int = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            let mut yy: ::core::ffi::c_int = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(yy as isize))
                .offset(xx as isize) as *mut borg_grid;
            if ((*ag).xtra as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
                (*ag).xtra =
                    ((*ag).xtra as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t;
            }
            i += 1;
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
    y = 1 as ::core::ffi::c_int;
    while y < AUTO_MAX_Y - 1 as ::core::ffi::c_int {
        let mut current_block_69: u64;
        x = 1 as ::core::ffi::c_int;
        while x < AUTO_MAX_X - 1 as ::core::ffi::c_int {
            let mut ag_ptr: [*mut borg_grid; 8] = [::core::ptr::null_mut::<borg_grid>(); 8];
            let mut wall: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut supp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut diag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                .offset(x as isize) as *mut borg_grid;
            if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                if !(*ag).trap {
                    if borg_cave_floor_grid(ag) {
                        cost = (*borg_data_cost).data[y as usize][x as usize] as ::core::ffi::c_int;
                        if !(cost >= 250 as ::core::ffi::c_int) {
                            if !(cost >= 25 as ::core::ffi::c_int
                                && *borg
                                    .trait_0
                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                                    < 30 as ::core::ffi::c_int)
                            {
                                if !(cost >= 50 as ::core::ffi::c_int) {
                                    if !((*ag).xtra as ::core::ffi::c_int
                                        >= 50 as ::core::ffi::c_int)
                                    {
                                        if !((*ag).xtra as ::core::ffi::c_int
                                            >= *borg
                                                .trait_0
                                                .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                                        {
                                            if !(!bored
                                                && (*ag).xtra as ::core::ffi::c_int
                                                    > 5 as ::core::ffi::c_int)
                                            {
                                                if !*(*borg_detect_door
                                                    .offset((y / borg_panel_hgt()) as isize))
                                                .offset((x / borg_panel_wid()) as isize)
                                                {
                                                    if b_stair != -(1 as ::core::ffi::c_int)
                                                        && *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) < 15 as ::core::ffi::c_int
                                                        && borg.avoidance as ::core::ffi::c_int
                                                            <= *borg.trait_0.offset(
                                                                BI_CURHP as ::core::ffi::c_int
                                                                    as isize,
                                                            )
                                                    {
                                                        j = borg_distance(
                                                            *track_less.y.offset(b_stair as isize),
                                                            *track_less.x.offset(b_stair as isize),
                                                            y,
                                                            x,
                                                        );
                                                        b_j = borg_distance(
                                                            borg.c.y,
                                                            borg.c.x,
                                                            *track_less.y.offset(b_stair as isize),
                                                            *track_less.x.offset(b_stair as isize),
                                                        );
                                                        if b_j
                                                            <= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) * 3 as ::core::ffi::c_int
                                                                + 9 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) * 3 as ::core::ffi::c_int
                                                                + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 3 as ::core::ffi::c_int
                                                            && b_j
                                                                <= *borg.trait_0.offset(
                                                                    BI_CLEVEL as ::core::ffi::c_int
                                                                        as isize,
                                                                ) + 9 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 3 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 5 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else if *borg.trait_0.offset(
                                                            BI_CLEVEL as ::core::ffi::c_int
                                                                as isize,
                                                        ) <= 10 as ::core::ffi::c_int
                                                            && j >= *borg.trait_0.offset(
                                                                BI_CLEVEL as ::core::ffi::c_int
                                                                    as isize,
                                                            ) + 9 as ::core::ffi::c_int
                                                        {
                                                            current_block_69 = 2569451025026770673;
                                                        } else {
                                                            current_block_69 = 12556861819962772176;
                                                        }
                                                    } else {
                                                        current_block_69 = 12556861819962772176;
                                                    }
                                                    match current_block_69 {
                                                        2569451025026770673 => {}
                                                        _ => {
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < 8 as ::core::ffi::c_int {
                                                                let mut xx_0: ::core::ffi::c_int = x
                                                                    + ddx_ddd[i as usize]
                                                                        as ::core::ffi::c_int;
                                                                let mut yy_0: ::core::ffi::c_int = y
                                                                    + ddy_ddd[i as usize]
                                                                        as ::core::ffi::c_int;
                                                                ag_ptr[i as usize] =
                                                                    (*(&raw mut borg_grids
                                                                        as *mut *mut borg_grid)
                                                                        .offset(yy_0 as isize))
                                                                    .offset(xx_0 as isize)
                                                                        as *mut borg_grid;
                                                                i += 1;
                                                            }
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < 4 as ::core::ffi::c_int {
                                                                ag = ag_ptr[i as usize];
                                                                if (*ag).feat as ::core::ffi::c_int
                                                                    >= FEAT_GRANITE
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    wall += 1;
                                                                }
                                                                i += 1;
                                                            }
                                                            if !(wall < 1 as ::core::ffi::c_int) {
                                                                i = 0 as ::core::ffi::c_int;
                                                                while i < 4 as ::core::ffi::c_int {
                                                                    ag = ag_ptr[i as usize];
                                                                    if !((*ag).feat
                                                                        as ::core::ffi::c_int
                                                                        == FEAT_RUBBLE
                                                                            as ::core::ffi::c_int)
                                                                    {
                                                                        if (*ag).feat as ::core::ffi::c_int
                                                                            >= FEAT_SECRET as ::core::ffi::c_int
                                                                            && (*ag).feat as ::core::ffi::c_int
                                                                                <= FEAT_GRANITE as ::core::ffi::c_int
                                                                            || ((*ag).feat as ::core::ffi::c_int
                                                                                == FEAT_OPEN as ::core::ffi::c_int
                                                                                || (*ag).feat as ::core::ffi::c_int
                                                                                    == FEAT_BROKEN as ::core::ffi::c_int)
                                                                            || (*ag).feat as ::core::ffi::c_int
                                                                                == FEAT_CLOSED as ::core::ffi::c_int
                                                                        {
                                                                            supp += 1;
                                                                        }
                                                                    }
                                                                    i += 1;
                                                                }
                                                                i = 4 as ::core::ffi::c_int;
                                                                while i < 8 as ::core::ffi::c_int {
                                                                    ag = ag_ptr[i as usize];
                                                                    if !((*ag).feat
                                                                        as ::core::ffi::c_int
                                                                        == FEAT_RUBBLE
                                                                            as ::core::ffi::c_int)
                                                                    {
                                                                        if (*ag).feat as ::core::ffi::c_int
                                                                            >= FEAT_SECRET as ::core::ffi::c_int
                                                                        {
                                                                            diag += 1;
                                                                        }
                                                                    }
                                                                    i += 1;
                                                                }
                                                                if !(diag < 2 as ::core::ffi::c_int)
                                                                {
                                                                    i = 0 as ::core::ffi::c_int;
                                                                    while i < 8
                                                                        as ::core::ffi::c_int
                                                                    {
                                                                        ag = ag_ptr[i as usize];
                                                                        if (*ag).kill != 0 {
                                                                            monsters += 1;
                                                                        }
                                                                        i += 1;
                                                                    }
                                                                    if !(monsters
                                                                        >= 1 as ::core::ffi::c_int)
                                                                    {
                                                                        v = (supp as borg_time
                                                                            * 500 as borg_time
                                                                            + diag as borg_time
                                                                                * 100 as borg_time
                                                                            - (*ag).xtra
                                                                                as borg_time
                                                                                * 40 as borg_time
                                                                            - cost as borg_time
                                                                                * 2 as borg_time
                                                                            - (borg.time.now
                                                                                - borg.time.level))
                                                                            as ::core::ffi::c_int;
                                                                        v
                                                                            -= (50 as ::core::ffi::c_int
                                                                                - *borg
                                                                                    .trait_0
                                                                                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize))
                                                                                * 5 as ::core::ffi::c_int;
                                                                        if !(v <= 0
                                                                            as ::core::ffi::c_int)
                                                                        {
                                                                            if !(!bored && v < 1500 as ::core::ffi::c_int) {
                                                                                if !(b_v >= 0 as ::core::ffi::c_int && v < b_v) {
                                                                                    b_v = v;
                                                                                    b_x = x;
                                                                                    b_y = y;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    borg_flow_clear();
    if b_v < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(b_y as isize)).offset(b_x as isize)
        as *mut borg_grid;
    spastic_x = b_x as uint8_t;
    spastic_y = b_y as uint8_t;
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
        b"spastic\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_XTRA,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_XTRA) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_shop_entry(mut i: ::core::ffi::c_int) -> bool {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut name: *const ::core::ffi::c_char =
        (*f_info.offset((*stores.offset(i as isize)).feat as isize)).name;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    x = *track_shop_x.offset(i as isize);
    y = *track_shop_y.offset(i as isize);
    if x == 0 || y == 0 {
        return false_0 != 0;
    }
    if x == borg.c.x && y == borg.c.y {
        borg_note(b"# Re-entering a shop\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('5' as i32 as keycode_t);
        return true_0 != 0;
    }
    borg_flow_clear();
    borg_flow_enqueue_grid(y, x);
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(name, GOAL_MISC) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_MISC) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_light(mut why: ::core::ffi::c_int) -> bool {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    borg_glow_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
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
            let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
                .offset(y as isize))
            .offset(x as isize) as *mut borg_grid;
            if !((*ag).info as ::core::ffi::c_int & BORG_GLOW == 0) {
                borg_glow_y[borg_glow_n as usize] = y as uint8_t;
                borg_glow_x[borg_glow_n as usize] = x as uint8_t;
                borg_glow_n += 1;
            }
            x += 1;
        }
        y += 1;
    }
    if borg_glow_n == 0 {
        return false_0 != 0;
    }
    borg_flow_clear();
    i = 0 as ::core::ffi::c_int;
    while i < borg_glow_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_glow_y[i as usize] as ::core::ffi::c_int,
            borg_glow_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"a lighted area\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn borg_flow_vault(mut nearness: ::core::ffi::c_int) -> bool {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut b_x: ::core::ffi::c_int = 0;
    let mut can_dig_hard: bool = false;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    if !borg.status.vault {
        return false_0 != 0;
    }
    if !borg_can_dig(false_0 != 0, FEAT_QUARTZ as ::core::ffi::c_int as uint8_t) {
        return false_0 != 0;
    }
    can_dig_hard = borg_can_dig(false_0 != 0, FEAT_GRANITE as ::core::ffi::c_int as uint8_t);
    y = w_y;
    while y < w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        let mut current_block_16: u64;
        x = w_x;
        while x < w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            if !(distance(borg.c, loc(x, y)) > nearness) {
                let mut feat: uint8_t = (*borg_grids[y as usize].offset(x as isize)).feat;
                if feat as ::core::ffi::c_int != FEAT_RUBBLE as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_QUARTZ as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_MAGMA as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_QUARTZ_K as ::core::ffi::c_int
                    && feat as ::core::ffi::c_int != FEAT_MAGMA_K as ::core::ffi::c_int
                {
                    if !can_dig_hard
                        || feat as ::core::ffi::c_int != FEAT_GRANITE as ::core::ffi::c_int
                    {
                        current_block_16 = 8515828400728868193;
                    } else {
                        current_block_16 = 12800627514080957624;
                    }
                } else {
                    current_block_16 = 12800627514080957624;
                }
                match current_block_16 {
                    8515828400728868193 => {}
                    _ => {
                        i = 0 as ::core::ffi::c_int;
                        while i < 8 as ::core::ffi::c_int {
                            b_x = x + ddx_ddd[i as usize] as ::core::ffi::c_int;
                            b_y = y + ddy_ddd[i as usize] as ::core::ffi::c_int;
                            if square_in_bounds_fully(cave, loc(b_x, b_y)) {
                                ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                                    .offset(b_y as isize))
                                .offset(b_x as isize)
                                    as *mut borg_grid;
                                if !((*ag).feat as ::core::ffi::c_int
                                    != FEAT_PERM as ::core::ffi::c_int)
                                {
                                    borg_temp_y[borg_temp_n as usize] = y as uint8_t;
                                    borg_temp_x[borg_temp_n as usize] = x as uint8_t;
                                    borg_temp_n += 1;
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
    if borg_temp_n == 0 {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < borg_temp_n as ::core::ffi::c_int {
        borg_flow_enqueue_grid(
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    borg_flow_spread(
        250 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        -(1 as ::core::ffi::c_int),
        false_0 != 0,
    );
    if !borg_flow_commit(
        b"vault excavation\0" as *const u8 as *const ::core::ffi::c_char,
        GOAL_VAULT,
    ) {
        return false_0 != 0;
    }
    if !borg_flow_old(GOAL_VAULT) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_twitchy() -> bool {
    let mut dir: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0;
    let mut all_walls: bool = true_0 != 0;
    let mut grid: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut l: loc = loc { x: 0, y: 0 };
    borg_note(b"# Twitchy!\0" as *const u8 as *const ::core::ffi::c_char);
    if borg_allow_teleport() {
        if borg_caution_phase(15 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
            && (borg_spell_fail(PHASE_DOOR, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, 40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_shadow_shift(40 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_phase) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            return true_0 != 0;
        }
    }
    count = 20 as ::core::ffi::c_int;
    loop {
        dir = Rand_div(10 as uint32_t) as int32_t as ::core::ffi::c_int;
        if dir == 5 as ::core::ffi::c_int || dir == 0 as ::core::ffi::c_int {
            continue;
        }
        if count == 0 {
            break;
        }
        count -= 1;
        borg.goal.g.x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
        borg.goal.g.y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
        if !square_in_bounds_fully(cave, borg.goal.g) {
            continue;
        }
        grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(borg.goal.g.y as isize))
            .offset(borg.goal.g.x as isize) as *mut borg_grid;
        if (*grid).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
            && (*grid).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
        {
            continue;
        }
        if !((*grid).kill as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                != 0)
        {
            break;
        }
    }
    if count == 0 {
        let mut current_block_17: u64;
        dir = 1 as ::core::ffi::c_int;
        while dir < 10 as ::core::ffi::c_int {
            if !(dir == 5 as ::core::ffi::c_int) {
                l.x = borg.c.x + ddx[dir as usize] as ::core::ffi::c_int;
                l.y = borg.c.y + ddy[dir as usize] as ::core::ffi::c_int;
                if square_in_bounds_fully(cave, l) {
                    grid = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(l.y as isize))
                        .offset(l.x as isize) as *mut borg_grid;
                    if (*grid).feat as ::core::ffi::c_int >= FEAT_SECRET as ::core::ffi::c_int
                        && (*grid).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int
                    {
                        if *borg
                            .trait_0
                            .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                            == 0
                            || (*grid).feat as ::core::ffi::c_int == FEAT_PERM as ::core::ffi::c_int
                        {
                            current_block_17 = 6009453772311597924;
                        } else {
                            current_block_17 = 9828876828309294594;
                        }
                    } else {
                        current_block_17 = 9828876828309294594;
                    }
                    match current_block_17 {
                        6009453772311597924 => {}
                        _ => {
                            if !((*grid).kill as ::core::ffi::c_int != 0
                                && *borg
                                    .trait_0
                                    .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
                                    != 0)
                            {
                                all_walls = false_0 != 0;
                                break;
                            }
                        }
                    }
                }
            }
            dir += 1;
        }
        if all_walls {
            borg_keypress('R' as i32 as keycode_t);
            borg_keypress('1' as i32 as keycode_t);
            borg_keypress('0' as i32 as keycode_t);
            borg_keypress('0' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_ISAFRAID as ::core::ffi::c_int as isize)
        != 0
    {
        borg_keypress('+' as i32 as keycode_t);
    }
    borg_keypress((dir + '0' as i32) as keycode_t);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_extract_dir(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if y1 == y2 && x1 == x2 {
        return 5 as ::core::ffi::c_int;
    }
    if x1 == x2 {
        return if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
    }
    if y1 == y2 {
        return if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
    }
    if y1 < y2 {
        return if x1 < x2 {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
    if y1 > y2 {
        return if x1 < x2 {
            9 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
    }
    return 5 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_goto_dir(
    mut y1: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    let mut ax: ::core::ffi::c_int = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    e = borg_extract_dir(y1, x1, y2, x2);
    if ax <= 1 as ::core::ffi::c_int && ay <= 1 as ::core::ffi::c_int {
        return e;
    }
    if ay > ax {
        d = if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay < ax {
        d = if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    d = borg_extract_dir(y1, x1, y2, x2);
    if borg_cave_floor_bold(
        y1 + ddy[d as usize] as ::core::ffi::c_int,
        x1 + ddx[d as usize] as ::core::ffi::c_int,
    ) {
        return d;
    }
    if ay <= ax {
        d = if y1 < y2 {
            2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay >= ax {
        d = if x1 < x2 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ay == 0 {
        d = if x1 < x2 {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
        d = if x1 < x2 {
            9 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    if ax == 0 {
        d = if y1 < y2 {
            3 as ::core::ffi::c_int
        } else {
            9 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
        d = if y1 < y2 {
            1 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
        if borg_cave_floor_bold(
            y1 + ddy[d as usize] as ::core::ffi::c_int,
            x1 + ddx[d as usize] as ::core::ffi::c_int,
        ) {
            return d;
        }
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn borg_check_rest(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut borg_in_vault: bool = false_0 != 0;
    if *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
        == *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
        && flag_has(
            &raw mut (*player).state.pflags as *mut bitflag,
            (PF_MAX as ::core::ffi::c_int as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                )
                .wrapping_sub(1 as size_t)
                .wrapping_div(
                    (::core::mem::size_of::<bitflag>() as size_t).wrapping_mul(8 as size_t),
                ),
            PF_COMBAT_REGEN as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            != 0
    {
        return false_0 != 0;
    }
    if borg.time.now - borg.time.last_kill_mult < 4 as borg_time
        && borg.time.now - borg.time.last_kill_mult > 0 as borg_time
    {
        return false_0 != 0;
    }
    if (borg.temp.bless as ::core::ffi::c_int != 0
        || borg.temp.hero as ::core::ffi::c_int != 0
        || borg.temp.berserk as ::core::ffi::c_int != 0
        || borg.temp.fastcast as ::core::ffi::c_int != 0
        || borg.temp.regen as ::core::ffi::c_int != 0
        || borg.temp.smite_evil as ::core::ffi::c_int != 0)
        && !borg.munchkin_mode
        && *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            >= *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                * 8 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            >= *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                * 7 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    borg.time.last_kill_mult = 0 as ::core::ffi::c_int as borg_time;
    if borg.status.vault {
        i = -(1 as ::core::ffi::c_int);
        while i < 1 as ::core::ffi::c_int {
            ii = -(1 as ::core::ffi::c_int);
            while ii < 1 as ::core::ffi::c_int {
                if square_in_bounds_fully(cave, loc(borg.c.x + ii, borg.c.y + i)) {
                    if (*borg_grids[(borg.c.y + i) as usize].offset((borg.c.x + ii) as isize)).feat
                        as ::core::ffi::c_int
                        == FEAT_PERM as ::core::ffi::c_int
                    {
                        borg_in_vault = true_0 != 0;
                    }
                }
                ii += 1;
            }
            i += 1;
        }
    }
    if borg.no_rest_prep as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
        && !borg.munchkin_mode
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize)
            > *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize)
                / 4 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < 85 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*borg_grids[y as usize].offset(x as isize)).feat as ::core::ffi::c_int
        == FEAT_LAVA as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_IFIRE as ::core::ffi::c_int as isize) == 0
    {
        return false_0 != 0;
    }
    if !borg_in_vault {
        if borg_fear_region[(y / 11 as ::core::ffi::c_int) as usize]
            [(x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 20 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 100 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if borg_fear_monsters[y as usize][x as usize] as ::core::ffi::c_int
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 10 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                != 100 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if borg_danger(y, x, 1 as ::core::ffi::c_int, true_0 != 0, false_0 != 0)
            > *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
                / 40 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 85 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if (*borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
            == 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
            || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize)
                < 2 as ::core::ffi::c_int)
            && !borg.munchkin_mode
        {
            return false_0 != 0;
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = borg_kills.offset(i as isize) as *mut borg_kill;
        let mut r_ptr: *mut monster_race =
            r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        let mut x9: ::core::ffi::c_int = (*kill).pos.x;
        let mut y9: ::core::ffi::c_int = (*kill).pos.y;
        let mut ax: ::core::ffi::c_int = 0;
        let mut ay: ::core::ffi::c_int = 0;
        let mut d: ::core::ffi::c_int = 0;
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !((*kill).r_idx == 0) {
            ax = if x9 > x { x9 - x } else { x - x9 };
            ay = if y9 > y { y9 - y } else { y - y9 };
            d = if ax < ay { ay } else { ax };
            if !(d > (*z_info).max_range as ::core::ffi::c_int) {
                if d < 2 as ::core::ffi::c_int
                    && !flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_NEVER_MOVE as ::core::ffi::c_int,
                    )
                {
                    return false_0 != 0;
                }
                if d == 1 as ::core::ffi::c_int {
                    return false_0 != 0;
                }
                if d < 10 as ::core::ffi::c_int
                    && flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_MULTIPLY as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    return false_0 != 0;
                }
                if !(!(*kill).awake && d > 8 as ::core::ffi::c_int && !borg.munchkin_mode) {
                    p = borg_danger_one_kill(
                        y9,
                        x9,
                        1 as ::core::ffi::c_int,
                        i,
                        true_0 != 0,
                        true_0 != 0,
                    );
                    if !borg_in_vault {
                        if d < 5 as ::core::ffi::c_int
                            && p > borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                            && !borg.munchkin_mode
                        {
                            return false_0 != 0;
                        }
                    }
                    if borg_los(y9, x9, y, x) as ::core::ffi::c_int != 0
                        && (*kill).ranged_attack as ::core::ffi::c_int != 0
                    {
                        return false_0 != 0;
                    }
                    if borg.munchkin_mode as ::core::ffi::c_int != 0
                        && borg_los(y9, x9, y, x) as ::core::ffi::c_int != 0
                        && ((*kill).awake as ::core::ffi::c_int != 0
                            && !flag_has(
                                &raw mut (*r_ptr).flags as *mut bitflag,
                                (RF_MAX as ::core::ffi::c_int as size_t)
                                    .wrapping_add(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    )
                                    .wrapping_sub(1 as size_t)
                                    .wrapping_div(
                                        (::core::mem::size_of::<bitflag>() as size_t)
                                            .wrapping_mul(8 as size_t),
                                    ),
                                RF_NEVER_MOVE as ::core::ffi::c_int,
                            ))
                    {
                        return false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_PASS_WALL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && !borg_in_vault
                    {
                        return false_0 != 0;
                    }
                    if flag_has(
                        &raw mut (*r_ptr).flags as *mut bitflag,
                        (RF_MAX as ::core::ffi::c_int as size_t)
                            .wrapping_add(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            )
                            .wrapping_sub(1 as size_t)
                            .wrapping_div(
                                (::core::mem::size_of::<bitflag>() as size_t)
                                    .wrapping_mul(8 as size_t),
                            ),
                        RF_KILL_WALL as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int
                        != 0
                        && !borg_in_vault
                    {
                        return false_0 != 0;
                    }
                }
            }
        }
        i += 1;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_far_from_stairs(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut b_stair: ::core::ffi::c_int,
) -> bool {
    return borg_flow_far_from_stairs_dist(x, y, b_stair, borg_get_leash(false_0 != 0));
}
#[no_mangle]
pub unsafe extern "C" fn borg_flow_far_from_stairs_dist(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut b_stair: ::core::ffi::c_int,
    mut distance_0: ::core::ffi::c_int,
) -> bool {
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - 5 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 20 as ::core::ffi::c_int
    {
        let mut cost: ::core::ffi::c_int = borg_flow_cost_stair(y, x, b_stair);
        if cost > distance_0 {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_flow_misc() {
    track_shop_x = mem_zalloc(
        (9 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    track_shop_y = mem_zalloc(
        (9 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    borg_init_track(&raw mut track_vein, 100 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_flow_misc() {
    borg_free_track(&raw mut track_vein);
    if !track_shop_y.is_null() {
        mem_free(track_shop_y as *mut ::core::ffi::c_void);
        track_shop_y = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !track_shop_x.is_null() {
        mem_free(track_shop_x as *mut ::core::ffi::c_void);
        track_shop_x = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
