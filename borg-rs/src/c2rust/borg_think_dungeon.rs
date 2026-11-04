pub const BORG_TIME_MAX: ::core::ffi::c_int = INT32_MAX;
pub const GOAL_KILL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_MISC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const GOAL_DARK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const GOAL_XTRA: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GOAL_BORE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GOAL_FLEE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const GOAL_VAULT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GOAL_RECOVER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const GOAL_DIGGING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const TERM_XTRA_DELAY: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_attack_munchkin_h::{borg_munchkin_mage, borg_munchkin_melee};
use crate::headers::borg_caution_h::borg_caution;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};
use crate::headers::borg_danger_h::{borg_danger, borg_danger_wipe};
use crate::headers::borg_escape_h::{
    borg_allow_teleport, borg_dimension_door, borg_recall, borg_shadow_shift,
};
use crate::headers::borg_fight_attack_h::borg_attack;
use crate::headers::borg_fight_defend_h::borg_defend;
use crate::headers::borg_fight_perm_h::borg_perma_spell;
use crate::headers::borg_flow_dark_h::borg_flow_dark;
use crate::headers::borg_flow_glyph_h::{borg_flow_glyph, track_glyph};
pub use crate::headers::borg_flow_h::{
    borg_flow_old, borg_track, track_closed, track_door, track_step,
};
use crate::headers::borg_flow_kill_h::{
    borg_delete_kill, borg_flow_kill, borg_flow_kill_aim, borg_flow_kill_corridor,
    borg_flow_kill_direct, borg_kills_cnt, borg_kills_nxt, borg_near_monster_type,
};
use crate::headers::borg_flow_misc_h::{
    borg_check_rest, borg_flow_recover, borg_flow_shop_entry, borg_flow_spastic, borg_flow_vault,
    borg_flow_vein, borg_twitchy, track_vein,
};
use crate::headers::borg_flow_stairs_h::{
    borg_flow_stair_both, borg_flow_stair_less, borg_flow_stair_more, track_less, track_more,
};
use crate::headers::borg_flow_take_h::{
    borg_delete_take, borg_flow_take, borg_flow_take_lunal, borg_takes_cnt, borg_takes_nxt,
};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_cfg, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH,
    BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT,
    BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT,
    BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER,
    BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS,
    BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM,
    BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS,
    BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD,
    BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{act_tele_level, act_tele_phase};
use crate::headers::borg_item_enchant_h::borg_enchanting;
pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_need, borg_read_scroll, borg_recharging, borg_use_staff,
    borg_use_things, BORG_MET_NEED, BORG_NO_NEED, BORG_UNMET_NEED,
};
use crate::headers::borg_item_val_h::{
    sv_scroll_phase_door, sv_scroll_teleport, sv_scroll_teleport_level, sv_staff_teleportation,
};
use crate::headers::borg_item_wear_h::{
    borg_swap_rings, borg_test_stuff, borg_wear_recharge, borg_wear_rings, borg_wear_stuff,
};
use crate::headers::borg_junk_h::{
    borg_destroy_floor, borg_drop_hole, borg_drop_junk, borg_drop_slow, borg_dump_quiver,
    borg_remove_stuff,
};
use crate::headers::borg_light_h::{borg_check_light, borg_light_beam, borg_maintain_light};
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS,
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
use crate::headers::borg_magic_play_h::borg_play_magic;
use crate::headers::borg_prepared_h::{borg_must_return_to_town, borg_prepared};
use crate::headers::borg_projection_h::borg_projectable;
use crate::headers::borg_recover_h::borg_recover;
use crate::headers::borg_store_sell_h::borg_count_sell;
use crate::headers::borg_think_dungeon_util_h::{
    borg_excavate_vault, borg_leave_level, borg_money_scum, borg_think_dungeon_light,
    borg_think_stair_scum,
};
use crate::headers::borg_think_h::borg_oops;
use crate::headers::borg_think_store_h::borg_choose_shop;
pub use crate::headers::borg_trait_h::{
    borg_notice, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
    BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM, BI_AENCH_SARM,
    BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL, BI_AGLYPH,
    BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID, BI_AMAGICMAP,
    BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL, BI_AMMO_COUNT,
    BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE, BI_ARESCOLD,
    BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED, BI_ASTFDEST,
    BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS, BI_BART, BI_BID,
    BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX, BI_CINT, BI_CLASS,
    BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY, BI_CRSAVULN,
    BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING, BI_CRSEVULN, BI_CRSFEAR,
    BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP, BI_CRSNOTEL, BI_CRSPARA,
    BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN, BI_CRSSLOW, BI_CRSSTEELSKIN,
    BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO, BI_CRSVULN, BI_CRSWEAK, BI_CSTR,
    BI_CURHP, BI_CURSP, BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX, BI_DEX_INDEX, BI_DIG, BI_DINV,
    BI_DISM, BI_DISP, BI_EMPTY, BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1, BI_FAIL2, BI_FAST_SHOTS,
    BI_FEATH, BI_FIRST_CURSED, BI_FOOD, BI_FOOD_CURE_BLIND, BI_FOOD_CURE_CONF, BI_FOOD_HI,
    BI_FOOD_LO, BI_FRACT, BI_GOLD, BI_GOOD_S_CHG, BI_GOOD_W_CHG, BI_HASFIXEXP, BI_HEAVYBOW,
    BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ, BI_IACID, BI_ICOLD, BI_IELEC, BI_IFIRE, BI_INFRA, BI_INT,
    BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID, BI_ISBLIND, BI_ISCONFUSED, BI_ISCUT, BI_ISENCUMB,
    BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX, BI_ISFIXEXP, BI_ISFIXINT, BI_ISFIXLEV, BI_ISFIXSTR,
    BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL, BI_ISGORGED, BI_ISHEAVYSTUN, BI_ISHUNGRY, BI_ISIMAGE,
    BI_ISPARALYZED, BI_ISPOISONED, BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK, BI_KING, BI_LIGHT, BI_MAX,
    BI_MAXCLEVEL, BI_MAXDEPTH, BI_MAXHP, BI_MAXSP, BI_MOD_MOVES, BI_MULTIPLE_BONUSES,
    BI_NEED_BRAND_WEAPON, BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D, BI_NEED_ENCHANT_TO_H,
    BI_PREP_BIG_FIGHT, BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD, BI_RCONF, BI_RDARK, BI_RDIS,
    BI_RECALL, BI_REG, BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS, BI_RLITE, BI_RNTHR, BI_RNXUS,
    BI_RPOIS, BI_RSHRD, BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON, BI_SDEX, BI_SDIG, BI_SFRACT,
    BI_SHLIFE, BI_SHOTS, BI_SINT, BI_SINV, BI_SLING, BI_SPEED, BI_SP_ADJ, BI_SRACID, BI_SRBLIND,
    BI_SRCH, BI_SRCOLD, BI_SRCONF, BI_SRDARK, BI_SRDIS, BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS,
    BI_SRLITE, BI_SRNTHR, BI_SRNXUS, BI_SRPOIS, BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR,
    BI_STR_INDEX, BI_SWIS, BI_THB, BI_THN, BI_THT, BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD,
    BI_WB_ELEC, BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS, BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS,
    BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON, BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL,
    BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL, BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD,
    BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, distance, heatmap, monster_group, square,
    square_in_bounds, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::h_basic_h::errr;
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
pub use crate::headers::list_tvals_h::{
    TV_AMULET, TV_ARROW, TV_BOLT, TV_BOOTS, TV_BOW, TV_CHEST, TV_CLOAK, TV_CROWN, TV_DIGGING,
    TV_DRAG_ARMOR, TV_FLASK, TV_FOOD, TV_GLOVES, TV_GOLD, TV_HAFTED, TV_HARD_ARMOR, TV_HELM,
    TV_LIGHT, TV_MAGIC_BOOK, TV_MUSHROOM, TV_NATURE_BOOK, TV_NULL, TV_OTHER_BOOK, TV_POLEARM,
    TV_POTION, TV_PRAYER_BOOK, TV_RING, TV_ROD, TV_SCROLL, TV_SHADOW_BOOK, TV_SHIELD, TV_SHOT,
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND,
};
pub use crate::headers::mon_blows_h::{blow_effect, blow_message, blow_method};
pub use crate::headers::monster_h::{
    monster, monster_altmsg, monster_altmsg_type, monster_base, monster_blow, monster_drop,
    monster_friends, monster_friends_base, monster_group_info, monster_group_role,
    monster_learned_player, monster_mimic, monster_pain, monster_race, monster_shape,
    MON_ALTMSG_MISS, MON_ALTMSG_SEEN, MON_ALTMSG_UNSEEN, MON_GROUP_BODYGUARD, MON_GROUP_LEADER,
    MON_GROUP_MEMBER, MON_GROUP_SERVANT, MON_GROUP_SUMMON,
};
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, TV_MAX};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, object, object_base,
    object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item, C2RustUnnamedH_34, PF_MAX,
};

pub use crate::headers::stdint_h::INT32_MAX;
pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
use crate::headers::ui_command_h::do_cmd_redraw;
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::ui_term_h::Term_xtra;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut borg_feeling_danger: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut borg_feeling_stuff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_think_dungeon_lunal() -> bool {
    let mut safe_place: bool = false_0 != 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut feat: uint8_t = (*square(cave, borg.c)).feat;
    let mut need: borg_need = BORG_NO_NEED;
    borg_notice(true_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        borg_note(
            b"# Leaving Lunal Mode. (Town or Weak)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_MORE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            if *track_more.x.offset(i as isize) == borg.c.x
                && *track_more.y.offset(i as isize) == borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_more.num as ::core::ffi::c_int && i < track_more.size as ::core::ffi::c_int {
            *track_more.x.offset(i as isize) = borg.c.x;
            *track_more.y.offset(i as isize) = borg.c.y;
            track_more.num += 1;
        }
        (*ag).feat = FEAT_MORE as ::core::ffi::c_int as uint8_t;
    }
    if feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_LESS as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) == borg.c.x
                && *track_less.y.offset(i as isize) == borg.c.y;
            i += 1;
        }
        if i == track_less.num as ::core::ffi::c_int && i < track_less.size as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) = borg.c.x;
            *track_less.y.offset(i as isize) = borg.c.y;
            track_less.num += 1;
        }
        (*ag).feat = FEAT_LESS as ::core::ffi::c_int as uint8_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && track_less.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg_note(
            b"# Leaving Lunal Mode. (No Stairs seen)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if *borg_cfg.offset(BORG_SELF_LUNAL as ::core::ffi::c_int as isize) != 0 {
        if *borg
            .trait_0
            .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
            <= *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 15 as ::core::ffi::c_int
            || !(borg_prepared(
                *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    - 5 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char)
                .is_null()
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 50 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
                != 0
        {
            borg.lunal_mode = false_0 != 0;
            borg.goal.fleeing = false_0 != 0;
            borg.goal.fleeing_lunal = false_0 != 0;
            borg_note(
                b"# Self Lunal mode disengaged normally.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(b"# Lunal. (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
        .tval as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        borg_note(b"# No Light at all.\0" as *const u8 as *const ::core::ffi::c_char);
        return false_0 != 0;
    }
    safe_place = borg_check_rest(borg.c.y, borg.c.x);
    if safe_place as ::core::ffi::c_int != 0 && borg_recover() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_use_things() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        && borg_use_things() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_drop_junk() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_flow_old(GOAL_TAKE) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_flow_take_lunal(true_0 != 0, 4 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    borg_note(b"# Fleeing level. Lunal Mode\0" as *const u8 as *const ::core::ffi::c_char);
    borg.goal.fleeing_lunal = true_0 != 0;
    borg.goal.fleeing = true_0 != 0;
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 2 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut closeness: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
        let mut tmp_ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        {
            if track_less.num != 0 {
                closeness = 20 as ::core::ffi::c_int;
            }
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Lunal Mode.  Power Climb (needing to sell). \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Lunal Mode (needing to sell).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_more.num as ::core::ffi::c_int != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int)
    {
        let mut y_0: ::core::ffi::c_int = 0;
        let mut x_0: ::core::ffi::c_int = 0;
        if track_more.num as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            borg_note(
                b"# Lunal Mode: I know of a down stair.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            x_0 = *track_more.x.offset(i as isize);
            y_0 = *track_more.y.offset(i as isize);
            j = distance(borg.c, loc(x_0, y_0));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int && safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                < 30 as ::core::ffi::c_int
        {
            borg_note(
                b"# Lunal Mode.  Power Diving. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_more(GOAL_FLEE, true_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
            if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
                borg_keypress('>' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 1 as ::core::ffi::c_int
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut y_1: ::core::ffi::c_int = 0;
        let mut x_1: ::core::ffi::c_int = 0;
        let mut tmp_ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x_1 = *track_less.x.offset(i as isize);
            y_1 = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x_1, y_1));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(b"# Lunal Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Lunal Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
    {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        borg_note(b"# Lunal Mode.  Any Stair. \0" as *const u8 as *const ::core::ffi::c_char);
        if borg_flow_stair_both(GOAL_FLEE, true_0 != 0) {
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 99 as ::core::ffi::c_int
    {
        borg_note(b"# Lunal Mode ended at depth.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        > 1 as ::core::ffi::c_int
    {
        borg_note(b"# Lunal Mode ended incorrectly.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    borg_note(
        b"Leaving Lunal Mode. (End of Lunal Mode)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    borg.lunal_mode = false_0 != 0;
    borg.goal.fleeing_lunal = false_0 != 0;
    borg.goal.fleeing = borg.goal.fleeing_lunal;
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_dungeon_munchkin() -> bool {
    let mut safe_place: bool = false_0 != 0;
    let mut bb_j: ::core::ffi::c_int = (*z_info).max_range as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut closeness: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
        .offset(borg.c.y as isize))
    .offset(borg.c.x as isize) as *mut borg_grid;
    let mut feat: uint8_t = (*square(cave, borg.c)).feat;
    let mut need: borg_need = BORG_NO_NEED;
    borg_notice(true_0 != 0);
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        || *borg
            .trait_0
            .offset(BI_ISWEAK as ::core::ffi::c_int as isize)
            != 0
    {
        borg_note(
            b"# Leaving munchkin Mode. (Town or Weak)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_MORE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            if *track_more.x.offset(i as isize) == borg.c.x
                && *track_more.y.offset(i as isize) == borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_more.num as ::core::ffi::c_int && i < track_more.size as ::core::ffi::c_int {
            *track_more.x.offset(i as isize) = borg.c.x;
            *track_more.y.offset(i as isize) = borg.c.y;
            track_more.num += 1;
        }
        (*ag).feat = FEAT_MORE as ::core::ffi::c_int as uint8_t;
    }
    if feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_LESS as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) == borg.c.x
                && *track_less.y.offset(i as isize) == borg.c.y;
            i += 1;
        }
        if i == track_less.num as ::core::ffi::c_int && i < track_less.size as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) = borg.c.x;
            *track_less.y.offset(i as isize) = borg.c.y;
            track_less.num += 1;
        }
        (*ag).feat = FEAT_LESS as ::core::ffi::c_int as uint8_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    if track_more.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && track_less.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        borg_note(
            b"# Leaving Munchkin Mode. (No Stairs seen)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        borg.munchkin_mode = false_0 != 0;
        return false_0 != 0;
    }
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(b"# Munchkin. (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
        borg_note(b"# No Light at all.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    safe_place = borg_check_rest(borg.c.y, borg.c.x);
    if borg_munchkin_mage() {
        return true_0 != 0;
    }
    if borg_munchkin_melee() {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_use_things() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISHUNGRY as ::core::ffi::c_int as isize)
        != 0
        && borg_use_things() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_wear_stuff() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_remove_stuff() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_drop_junk() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_play_magic(true_0 != 0) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0
        && borg_flow_take_lunal(true_0 != 0, 5 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if safe_place as ::core::ffi::c_int != 0 && borg_recover() as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    borg_note(b"# Fleeing level. Munchkin Mode\0" as *const u8 as *const ::core::ffi::c_char);
    borg.goal.fleeing_munchkin = true_0 != 0;
    borg.goal.fleeing = true_0 != 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        <= 10 as ::core::ffi::c_int
    {
        closeness += *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            - 10 as ::core::ffi::c_int
            + (10 as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize));
    }
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 2 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize)
                == 0 as ::core::ffi::c_int)
    {
        let mut tmp_ag: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int
        {
            if track_less.num != 0 {
                closeness = 20 as ::core::ffi::c_int;
            }
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb (needing to sell). \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode (needing to sell).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            > *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut tmp_ag_0: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        b_j = -(1 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                    if b_j < bb_j {
                        bb_j = b_j;
                    }
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_0).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_more.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            < *borg_cfg.offset(BORG_MUNCHKIN_DEPTH as ::core::ffi::c_int as isize)
        && (safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int)
    {
        b_j = -(1 as ::core::ffi::c_int);
        if track_more.num as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            borg_note(
                b"# Munchkin Mode: I know of a down stair.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            x = *track_more.x.offset(i as isize);
            y = *track_more.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 1 as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Diving. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_more(GOAL_FLEE, true_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
            if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
                borg_keypress('>' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 1 as ::core::ffi::c_int
        && safe_place as ::core::ffi::c_int != 0
        || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
    {
        let mut tmp_ag_1: *mut borg_grid = (*(&raw mut borg_grids as *mut *mut borg_grid)
            .offset(borg.c.y as isize))
        .offset(borg.c.x as isize) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if borg_projectable(y, x, borg.c.y, borg.c.x) {
                if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                    b_j = j;
                }
            }
            i += 1;
        }
        if b_j < closeness && safe_place as ::core::ffi::c_int != 0
            || (*tmp_ag_1).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Munchkin Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Munchkin Mode.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if (*tmp_ag_1).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
                && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
            {
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        >= 2 as ::core::ffi::c_int
        || !safe_place
    {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        borg_note(b"# Munchkin Mode.  Any Stair. \0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            y = borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
            x = borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
            if square_in_bounds(cave, loc(x, y)) {
                ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(y as isize))
                    .offset(x as isize) as *mut borg_grid;
                if (*ag).kill != 0 {
                    ii = 0 as ::core::ffi::c_int;
                    while ii < track_less.num as ::core::ffi::c_int {
                        x = *track_less.x.offset(ii as isize);
                        y = *track_less.y.offset(ii as isize);
                        j = distance(borg.c, loc(x, y));
                        if borg_projectable(y, x, borg.c.y, borg.c.x) {
                            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                                b_j = j;
                            }
                        }
                        ii += 1;
                    }
                    ii = 0 as ::core::ffi::c_int;
                    while ii < track_more.num as ::core::ffi::c_int {
                        x = *track_more.x.offset(ii as isize);
                        y = *track_more.y.offset(ii as isize);
                        j = distance(borg.c, loc(x, y));
                        if borg_projectable(y, x, borg.c.y, borg.c.x) {
                            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                                b_j = j;
                            }
                        }
                        ii += 1;
                    }
                    if b_j <= 3 as ::core::ffi::c_int {
                        if borg_flow_stair_both(GOAL_FLEE, false_0 != 0) {
                            return true_0 != 0;
                        }
                    } else if borg_attack(false_0 != 0) {
                        return true_0 != 0;
                    }
                }
            }
            i += 1;
        }
        if borg_flow_stair_both(GOAL_FLEE, false_0 != 0) {
            return true_0 != 0;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
            borg_keypress('>' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    borg_note(b"Leaving Munchkin Mode. (End of Mode)\0" as *const u8 as *const ::core::ffi::c_char);
    borg.munchkin_mode = false_0 != 0;
    borg.goal.fleeing_munchkin = false_0 != 0;
    borg.goal.fleeing = borg.goal.fleeing_munchkin;
    return false_0 != 0;
}
unsafe extern "C" fn borg_think_dungeon_brave() -> bool {
    let mut p1: ::core::ffi::c_int = borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 100 as ::core::ffi::c_int
        && borg_defend(p1) as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_attack(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_light_beam(false_0 != 0) {
        return true_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        == FEAT_MORE as ::core::ffi::c_int
    {
        borg_note(b"# Fleeing via stairs.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('>' as i32 as keycode_t);
        return true_0 != 0;
    }
    if borg.goal.less {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                == 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Goal_less, brave.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if borg.goal.fleeing as ::core::ffi::c_int != 0
        || borg.goal.leaving as ::core::ffi::c_int != 0
        || borg.mon.scary as ::core::ffi::c_int != 0
    {
        borg.stair_less = borg.goal.fleeing;
        if borg.ready_morgoth == 0 as ::core::ffi::c_int {
            borg.stair_less = true_0 != 0;
        }
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg.stair_less = false_0 != 0;
        }
        if borg.stair_less as ::core::ffi::c_int == true_0 {
            borg_note(
                b"# Fleeing and leaving the level. Brave Thinking.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        borg.stair_more = borg.goal.fleeing;
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        {
            borg.stair_more = true_0 != 0;
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.stair_less {
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Flee, brave.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        if borg.stair_more {
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, true_0 != 0) {
                return true_0 != 0;
            }
        }
    }
    if borg.status.vault {
        if borg_flow_old(GOAL_KILL) {
            return true_0 != 0;
        }
        if borg_flow_kill(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_old(GOAL_TAKE) {
            return true_0 != 0;
        }
        if borg_flow_take(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_vein(true_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_old(GOAL_VAULT) {
            return true_0 != 0;
        }
        if borg_flow_vault(35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_kill(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_MISC) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_DARK) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_XTRA) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_BORE) {
        return true_0 != 0;
    }
    if borg_flow_dark(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_dark(false_0 != 0) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_take(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_kill(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_leave_level(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_spastic(true_0 != 0) {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_check_bounciness() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && (borg.antibounce_count as ::core::ffi::c_int >= 300 as ::core::ffi::c_int
            && borg.antibounce_count as ::core::ffi::c_int <= 303 as ::core::ffi::c_int)
    {
        borg.goal.type_0 = 0 as int16_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && (borg.antibounce_count as ::core::ffi::c_int >= 500 as ::core::ffi::c_int
            && borg.antibounce_count as ::core::ffi::c_int <= 503 as ::core::ffi::c_int)
    {
        i = 1 as ::core::ffi::c_int;
        while i < borg_takes_nxt as ::core::ffi::c_int {
            borg_delete_take(i);
            i += 1;
        }
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        i = 1 as ::core::ffi::c_int;
        while i < borg_kills_nxt as ::core::ffi::c_int {
            borg_delete_kill(i);
            i += 1;
        }
        borg_kills_cnt = 0 as int16_t;
        borg_kills_nxt = 1 as int16_t;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
        && borg.antibounce_count as ::core::ffi::c_int >= 700 as ::core::ffi::c_int
    {
        if !borg.goal.leaving {
            borg_note(b"# Leaving (bouncing-borg)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (bouncing-borg)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_check_breeders() -> bool {
    if borg.mon.breeders as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
        borg.near.breeder = true_0 != 0;
    }
    if borg.mon.breeders as ::core::ffi::c_int
        >= (if *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int
            > 5 as ::core::ffi::c_int
        {
            5 as ::core::ffi::c_int
        } else {
            *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                + 2 as ::core::ffi::c_int
        })
        && (*borg
            .trait_0
            .offset(BI_RECALL as ::core::ffi::c_int as isize)
            <= 0 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 35 as ::core::ffi::c_int)
    {
        if !borg.goal.ignoring && borg.time.now - borg.time.level >= 2500 as borg_time {
            borg_note(
                b"# Ignoring breeders (no recall)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            borg.goal.ignoring = true_0 != 0;
        }
        if !borg.goal.leaving {
            borg_note(b"# Leaving (no recall)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (no recall)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_dungeon() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut msec: ::core::ffi::c_int = 0;
    if borg.time.now >= 12000 as borg_time && borg.time.now <= 12025 as borg_time
        || borg.time.now >= 25000 as borg_time && borg.time.now <= 25025 as borg_time
    {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg.status.old_depth = 126 as int16_t;
        return true_0 != 0;
    }
    if borg.time.now >= BORG_TIME_MAX as borg_time - 3000 as borg_time {
        borg_oops(b"clock overflow\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    borg.trying_unknown = false_0 != 0;
    if borg_destroy_floor() {
        return true_0 != 0;
    }
    msec = (*player).opts.delay_factor as ::core::ffi::c_int
        * (*player).opts.delay_factor as ::core::ffi::c_int
        + *borg_cfg.offset(BORG_DELAY_FACTOR as ::core::ffi::c_int as isize)
            * *borg_cfg.offset(BORG_DELAY_FACTOR as ::core::ffi::c_int as isize);
    Term_xtra(TERM_XTRA_DELAY, msec);
    if borg.status.redraw {
        borg_note(format(
            b"#  Redrawing screen.\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        do_cmd_redraw();
        borg.status.redraw = false_0 != 0;
    }
    if borg.time.now - borg.time.level >= 10000 as borg_time {
        if !borg.goal.leaving {
            borg_note(b"# Leaving (boredom)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (boredom)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    borg_near_monster_type(
        if *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 15 as ::core::ffi::c_int
        {
            (*z_info).max_sight as ::core::ffi::c_int
        } else {
            12 as ::core::ffi::c_int
        },
    );
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 1 as ::core::ffi::c_int
        && borg.goal.fleeing_to_town as ::core::ffi::c_int != 0
    {
        if borg_think_stair_scum() {
            return true_0 != 0;
        }
        if !borg.goal.leaving {
            borg_note(b"# Leaving (finish shopping)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.leaving = true_0 != 0;
        }
        if !borg.goal.fleeing {
            borg_note(b"# Fleeing (finish shopping)\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.fleeing = true_0 != 0;
        }
    }
    if borg_check_bounciness() {
        return true_0 != 0;
    }
    if borg_check_breeders() {
        return true_0 != 0;
    }
    if borg.avoidance as ::core::ffi::c_int
        != *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
    {
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
    }
    if track_less.num as ::core::ffi::c_int != 0
        && (*borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
            < 30 as ::core::ffi::c_int
            || *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                < 15 as ::core::ffi::c_int)
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= *borg
                .trait_0
                .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                - 5 as ::core::ffi::c_int
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i as isize);
            y = *track_less.y.offset(i as isize);
            j = distance(borg.c, loc(x, y));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if !borg.goal.less
            && b_j
                > *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    * 3 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
        {
            if !borg.goal.less {
                borg_note(format(
                    b"# Return to Stair (wandered too far.  Leash: %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *borg
                        .trait_0
                        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                        * 3 as ::core::ffi::c_int
                        + 14 as ::core::ffi::c_int,
                ));
                borg.goal.less = true_0 != 0;
            }
        } else if borg.goal.less as ::core::ffi::c_int != 0 && b_j < 3 as ::core::ffi::c_int {
            borg_note(b"# Close enough to Stair.\0" as *const u8 as *const ::core::ffi::c_char);
            borg.goal.less = false_0 != 0;
            borg.goal.type_0 = 0 as int16_t;
        }
    }
    borg_notice(true_0 != 0);
    if *borg_cfg.offset(BORG_SELF_LUNAL as ::core::ffi::c_int as isize) != 0
        && *borg_cfg.offset(BORG_PLAYS_RISKY as ::core::ffi::c_int as isize) == 0
    {
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 15 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
            && *borg
                .trait_0
                .offset(BI_MAXDEPTH as ::core::ffi::c_int as isize)
                >= *borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    + 15 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                >= 1 as ::core::ffi::c_int
            && *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                > *borg
                    .trait_0
                    .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
                    / 3 as ::core::ffi::c_int
        {
            borg.lunal_mode = true_0 != 0;
            if borg.lunal_mode as ::core::ffi::c_int != 0
                && borg_think_dungeon_lunal() as ::core::ffi::c_int != 0
            {
                return true_0 != 0;
            }
        }
    }
    if *borg_cfg.offset(BORG_MUNCHKIN_START as ::core::ffi::c_int as isize) != 0
        && *borg
            .trait_0
            .offset(BI_MAXCLEVEL as ::core::ffi::c_int as isize)
            < 12 as ::core::ffi::c_int
    {
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            >= 1 as ::core::ffi::c_int
        {
            borg.munchkin_mode = true_0 != 0;
            if borg_think_dungeon_munchkin() {
                return true_0 != 0;
            }
        }
        borg.munchkin_mode = false_0 != 0;
    }
    if track_less.num as ::core::ffi::c_int != 0
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 10 as ::core::ffi::c_int
        && !borg.goal.less
        && !(borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize),
        ) as *mut ::core::ffi::c_char)
            .is_null()
    {
        borg_note(
            b"# Needing to get back on correct depth\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.goal.less = true_0 != 0;
        if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            == FEAT_LESS as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    if borg_think_dungeon_light() {
        return true_0 != 0;
    }
    if borg.no_retreat as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        borg.no_retreat -= 1;
    }
    if borg.times_twitch as ::core::ffi::c_int > 20 as ::core::ffi::c_int {
        borg.goal.fleeing = true_0 != 0;
    }
    if borg_flow_old(GOAL_DIGGING) {
        return true_0 != 0;
    }
    if borg_caution() {
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_ISCUT as ::core::ffi::c_int as isize) != 0
        || *borg
            .trait_0
            .offset(BI_ISPOISONED as ::core::ffi::c_int as isize)
            != 0
        || *borg.trait_0.offset(BI_FOOD as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
    {
        if *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            if borg_maintain_light() as ::core::ffi::c_uint
                == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return true_0 != 0;
            }
            if borg_wear_stuff() {
                return true_0 != 0;
            }
        }
        if borg_recover() {
            return true_0 != 0;
        }
        if (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int
                - *borg
                    .trait_0
                    .offset(BI_QUIVER_SLOTS as ::core::ffi::c_int as isize)
                - 1 as ::core::ffi::c_int) as isize,
        ))
        .iqty as ::core::ffi::c_int
            != 0
            && borg_drop_hole(false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_choose_shop() {
            if borg_flow_shop_entry(borg.goal.shop as ::core::ffi::c_int) {
                return true_0 != 0;
            }
        }
    }
    if !(borg_must_return_to_town() as *mut ::core::ffi::c_char).is_null() {
        if borg_leave_level(false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_play_magic(false_0 != 0) {
        return true_0 != 0;
    }
    if (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
        == TV_DIGGING as ::core::ffi::c_int
        && borg_wear_stuff() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if (*borg_items.offset((*z_info).pack_size as isize)).tval == 0
        && borg_wear_stuff() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_flow_kill_corridor() {
        return true_0 != 0;
    }
    if borg_attack(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_wear_stuff() {
        return true_0 != 0;
    }
    if borg_swap_rings() {
        return true_0 != 0;
    }
    if borg_wear_rings() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_wear_rings() {
        return true_0 != 0;
    }
    if borg_remove_stuff() {
        return true_0 != 0;
    }
    if borg_dump_quiver() {
        return true_0 != 0;
    }
    if borg_check_light() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_RECOVER) {
        return true_0 != 0;
    }
    if borg_recover() {
        return true_0 != 0;
    }
    if borg_flow_recover(50 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_perma_spell() {
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
        < 10 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_MAXSP as ::core::ffi::c_int as isize) != 0
        && *borg.trait_0.offset(BI_CURSP as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        && borg.no_rest_prep as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && !borg.temp.bless
        && !borg.temp.hero
        && !borg.temp.berserk
        && !borg.temp.fastcast
        && !flag_has(
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
        )
    {
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            != 0
        {
            let mut tmp_i: ::core::ffi::c_int = 0;
            let mut y_0: ::core::ffi::c_int = 0;
            let mut x_0: ::core::ffi::c_int = 0;
            tmp_i = 0 as ::core::ffi::c_int;
            while tmp_i < track_less.num as ::core::ffi::c_int {
                x_0 = *track_less.x.offset(tmp_i as isize);
                y_0 = *track_less.y.offset(tmp_i as isize);
                if !(borg.c.y != y_0 || borg.c.x != x_0) {
                    borg.goal.less = false_0 != 0;
                    if borg_danger(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ) == 0 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Resting on stair to gain Mana.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress(',' as i32 as keycode_t);
                        return true_0 != 0;
                    }
                }
                tmp_i += 1;
            }
        } else {
            let mut tmp_i_0: ::core::ffi::c_int = 0;
            let mut y_1: ::core::ffi::c_int = 0;
            let mut x_1: ::core::ffi::c_int = 0;
            tmp_i_0 = 0 as ::core::ffi::c_int;
            while tmp_i_0 < track_more.num as ::core::ffi::c_int {
                x_1 = *track_more.x.offset(tmp_i_0 as isize);
                y_1 = *track_more.y.offset(tmp_i_0 as isize);
                if !(borg.c.y != y_1 || borg.c.x != x_1) {
                    if borg_danger(
                        borg.c.y,
                        borg.c.x,
                        1 as ::core::ffi::c_int,
                        true_0 != 0,
                        false_0 != 0,
                    ) == 0 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Resting on town stair to gain Mana.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress(',' as i32 as keycode_t);
                        return true_0 != 0;
                    }
                }
                tmp_i_0 += 1;
            }
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, true_0 != 0) {
            borg_note(
                b"# Looking for stairs. Stair hugging.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CLEVEL as ::core::ffi::c_int as isize)
            < 6 as ::core::ffi::c_int
        && *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize) < 10 as ::core::ffi::c_int
        && borg_count_sell() < 5 as ::core::ffi::c_int
    {
        borg_note(
            b"# Nothing to sell in town (leaving).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        borg.goal.leaving = true_0 != 0;
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg.goal.less {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Goal_less, Fleeing.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if borg.goal.fleeing as ::core::ffi::c_int != 0 && borg.goal.recalling == 0 {
        borg.stair_more = true_0 != 0;
        borg.stair_less = borg.stair_more;
        if (*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg.stair_less = false_0 != 0;
        }
        borg_note(
            b"# Fleeing and leaving the level. (Looking for any stair)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.mon.scary as ::core::ffi::c_int != 0
            && borg_flow_stair_both(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
            borg_note(
                b"# Looking for stairs. Fleeing.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
    }
    if *borg.trait_0.offset(BI_KING as ::core::ffi::c_int as isize) == 0
        && borg.near.morgoth as ::core::ffi::c_int != 0
        && !borg.morgoth_position
        && (*borg
            .trait_0
            .offset(BI_AGLYPH as ::core::ffi::c_int as isize)
            >= 10 as ::core::ffi::c_int
            && (*borg
                .trait_0
                .offset(BI_ISBLIND as ::core::ffi::c_int as isize)
                == 0
                && *borg
                    .trait_0
                    .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
                    == 0))
    {
        if borg_flow_old(GOAL_MISC) {
            return true_0 != 0;
        }
        if borg_flow_glyph() {
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 5 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_kill(true_0 != 0, 20 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 10 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 10 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_KILL) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_VAULT) {
        return true_0 != 0;
    }
    if borg_flow_kill_aim(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_use_things() {
        return true_0 != 0;
    }
    if borg_test_stuff() {
        return true_0 != 0;
    }
    if borg_enchanting() {
        return true_0 != 0;
    }
    if borg_recharging() {
        return true_0 != 0;
    }
    if borg_drop_junk() {
        return true_0 != 0;
    }
    if borg_drop_hole(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_drop_slow() {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(true_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg.goal.leaving as ::core::ffi::c_int != 0
        && borg.goal.recalling == 0
        && borg.mon.unique == 0
        || *borg
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
    {
        if borg.ready_morgoth == 0 as ::core::ffi::c_int
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            borg_note(
                b"# Fleeing and leaving the level (Looking for Up Stair).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg.stair_less = true_0 != 0;
        }
        if (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 1 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        {
            borg.stair_more = true_0 != 0;
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg.stair_less {
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Goal_Leaving.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
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
        {
            borg.stair_more = false_0 != 0;
        }
        if borg.stair_more {
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0 as ::core::ffi::c_int
        && (borg_prepared(
            *borg
                .trait_0
                .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                + 5 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        && !borg.stair_less
    {
        borg.stair_more = true_0 != 0;
        if borg_flow_old(GOAL_BORE) {
            return true_0 != 0;
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
        {
            borg.stair_more = false_0 != 0;
        }
        if borg.stair_more as ::core::ffi::c_int != 0
            && borg_flow_stair_more(GOAL_BORE, true_0 != 0, false_0 != 0) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Powerdiving.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if borg_flow_old(GOAL_MISC) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_DARK) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_XTRA) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_BORE) {
        return true_0 != 0;
    }
    if borg_flow_old(GOAL_VAULT) {
        return true_0 != 0;
    }
    if borg.status.vault {
        if borg_flow_kill(
            false_0 != 0,
            (*z_info).max_range as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        ) {
            return true_0 != 0;
        }
        if borg_flow_take(false_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_vein(false_0 != 0, 35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_excavate_vault((*z_info).max_range as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        {
            return true_0 != 0;
        }
        if borg_flow_vault(35 as ::core::ffi::c_int) {
            return true_0 != 0;
        }
        if borg_flow_dark(true_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_flow_kill(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_take(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_vein(false_0 != 0, 250 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    if borg_flow_dark(true_0 != 0) {
        return true_0 != 0;
    }
    if !(*borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        < *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0)
    {
        if borg_leave_level(false_0 != 0) {
            return true_0 != 0;
        }
    }
    if borg_flow_dark(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_choose_shop() {
        if borg_flow_shop_entry(borg.goal.shop as ::core::ffi::c_int) {
            return true_0 != 0;
        }
    }
    if !borg.goal.fleeing && borg_play_magic(true_0 != 0) as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_spastic(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_kill_direct(false_0 != 0) {
        return true_0 != 0;
    }
    if borg_wear_recharge() {
        return true_0 != 0;
    }
    if *borg.trait_0.offset(BI_GOLD as ::core::ffi::c_int as isize)
        < *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
        && *borg_cfg.offset(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int as isize)
            != 0 as ::core::ffi::c_int
        && *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        && *borg.trait_0.offset(BI_LIGHT as ::core::ffi::c_int as isize) != 0
        && *borg_cfg.offset(BORG_PLAYS_RISKY as ::core::ffi::c_int as isize) == 0
    {
        if borg_money_scum() {
            return true_0 != 0;
        }
    } else if borg_leave_level(true_0 != 0) {
        return true_0 != 0;
    }
    if borg.time.detect_doors == 0 || borg.time.now - borg.time.detect_doors >= 500 as borg_time {
        if borg_check_light() {
            return true_0 != 0;
        }
    }
    if borg_flow_spastic(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_flow_kill_direct(false_0 != 0) {
        return true_0 != 0;
    }
    if borg.goal.recalling != 0
        && borg_danger(
            borg.c.y,
            borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        ) <= 0 as ::core::ffi::c_int
    {
        borg_note(b"# Waiting for Recall...\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('9' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        return true_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        == 0 as ::core::ffi::c_int
        && borg.time.now - borg.time.level > 800 as borg_time
        || borg.time.now - borg.time.level > 28000 as borg_time
    {
        borg.status.old_depth = 126 as int16_t;
    }
    borg.no_retreat = 5 as int16_t;
    if (borg.avoidance as ::core::ffi::c_int)
        < *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) * 2 as ::core::ffi::c_int
    {
        let mut done: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (1) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) * 2 as ::core::ffi::c_int,
        ));
        borg.avoidance = (*borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if borg_think_dungeon_brave() {
            done = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done {
            return true_0 != 0;
        }
    }
    borg.times_twitch += 1;
    if (borg.times_twitch as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        borg_note(b"# Considering Phase (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_spell(PHASE_DOOR) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_phase_door) as ::core::ffi::c_int != 0
                || borg_dimension_door(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(TELEPORT_SELF) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0
                || borg_shadow_shift(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0)
        {
            return true_0 != 0;
        }
    }
    borg.no_retreat = 10 as int16_t;
    if (borg.avoidance as ::core::ffi::c_int)
        < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 4 as ::core::ffi::c_int
    {
        let mut done_0: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (2) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize) * 4 as ::core::ffi::c_int,
        ));
        borg.avoidance = (*borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
            * 4 as ::core::ffi::c_int) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if borg_think_dungeon_brave() {
            done_0 = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done_0 {
            return true_0 != 0;
        }
    }
    if (borg.avoidance as ::core::ffi::c_int) < 30000 as ::core::ffi::c_int {
        let mut done_1: bool = false_0 != 0;
        borg_note(format(
            b"# Boosting bravery (3) from %d to %d!\0" as *const u8 as *const ::core::ffi::c_char,
            borg.avoidance as ::core::ffi::c_int,
            30000 as ::core::ffi::c_int,
        ));
        borg.avoidance = 30000 as int16_t;
        borg_danger_wipe = true_0 != 0;
        borg.mon.unique = 0 as ::core::ffi::c_uint;
        borg.mon.scary = false_0 != 0;
        borg.near.breeder = false_0 != 0;
        borg.goal.type_0 = 0 as int16_t;
        if *borg
            .trait_0
            .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
            == 0
        {
            borg.goal.rising = false_0 != 0;
        }
        borg.goal.ignoring = false_0 != 0;
        track_less.num = 0 as int16_t;
        track_more.num = 0 as int16_t;
        track_glyph.num = 0 as int16_t;
        track_step.num = 0 as int16_t;
        track_door.num = 0 as int16_t;
        track_closed.num = 0 as int16_t;
        track_vein.num = 0 as int16_t;
        borg_takes_cnt = 0 as int16_t;
        borg_takes_nxt = 1 as int16_t;
        if borg_think_dungeon_brave() {
            done_1 = true_0 != 0;
        }
        borg.avoidance = *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize) as int16_t;
        borg_danger_wipe = true_0 != 0;
        if done_1 {
            return true_0 != 0;
        }
    }
    borg.times_twitch += 1;
    if (borg.times_twitch as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(TELEPORT_SELF) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0
                || borg_shadow_shift(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_use_staff(sv_staff_teleportation) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_teleport_level) as ::core::ffi::c_int != 0
                || borg_activate_item(act_tele_level) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Teleport (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if *borg
        .trait_0
        .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
        != 0
    {
        if borg_recall() {
            borg_note(b"# Recalling (twitchy)\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    borg.mon.unique = 0 as ::core::ffi::c_uint;
    borg.mon.scary = false_0 != 0;
    borg.near.breeder = false_0 != 0;
    borg_takes_cnt = 0 as int16_t;
    borg_takes_nxt = 1 as int16_t;
    borg_kills_cnt = 0 as int16_t;
    borg_kills_nxt = 1 as int16_t;
    if borg_flow_kill_direct(true_0 != 0) {
        return true_0 != 0;
    }
    if borg_twitchy() {
        return true_0 != 0;
    }
    return false_0 != 0;
}
