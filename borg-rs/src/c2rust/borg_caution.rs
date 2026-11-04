pub const GOAL_FLEE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

use crate::headers::borg_cave_view_h::{borg_view_n, borg_view_x, borg_view_y};
use crate::headers::borg_danger_h::{borg_danger, borg_fear_region};
use crate::headers::borg_escape_h::{
    borg_caution_phase, borg_escape, borg_freedom, borg_recall, borg_surrounded,
};
use crate::headers::borg_fight_defend_h::borg_defend;
use crate::headers::borg_flow_glyph_h::track_glyph;
pub use crate::headers::borg_flow_h::{borg_track, track_step};
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_kills_nxt};
use crate::headers::borg_flow_misc_h::{borg_check_rest, borg_goto_dir, borg_happy_grid_bold};
use crate::headers::borg_flow_stairs_h::{
    borg_flow_stair_less, borg_flow_stair_more, borg_prep_leave_level_spells, track_less,
    track_more,
};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_inventory_h::borg_slot;
use crate::headers::borg_io_h::{borg_keypress, borg_note};

pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_item_use_h::{
    borg_activate_failure, borg_activate_item, borg_eat, borg_eat_food_any, borg_eat_unknown,
    borg_equips_staff_fail, borg_need, borg_quaff_crit, borg_quaff_potion, borg_quaff_unknown,
    borg_read_scroll, borg_read_unknown, borg_use_staff, borg_use_staff_fail, borg_use_unknown,
    borg_zap_rod, BORG_MET_NEED, BORG_NO_NEED, BORG_UNMET_NEED,
};
use crate::headers::borg_item_val_h::{
    borg_lookup_kind, sv_mush_cure_mind, sv_mush_fast_recovery, sv_mush_purging, sv_mush_restoring,
    sv_potion_berserk, sv_potion_boldness, sv_potion_cure_critical, sv_potion_cure_light,
    sv_potion_cure_poison, sv_potion_cure_serious, sv_potion_healing, sv_potion_heroism,
    sv_potion_life, sv_potion_restore_mana, sv_potion_star_healing, sv_rod_healing,
    sv_scroll_phase_door, sv_scroll_teleport_level, sv_staff_curing, sv_staff_healing,
    sv_staff_teleportation, sv_staff_the_magi,
};
use crate::headers::borg_item_wear_h::borg_backup_swap;
use crate::headers::borg_light_h::borg_maintain_light;
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_fail, borg_spell_legal, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT,
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
use crate::headers::borg_messages_react_h::borg_dont_react;
use crate::headers::borg_prepared_h::{borg_prepared, borg_restock};
use crate::headers::borg_store_sell_h::borg_count_sell;
pub use crate::headers::borg_trait_h::{
    borg_game_ratio, borg_uses_swaps, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON,
    BI_ACSW, BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER,
    BI_AENCH_ARM, BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL,
    BI_AFUEL, BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID,
    BI_AMAGICMAP, BI_AMASSBAN, BI_AMISSILES, BI_AMISSILES_CURSED, BI_AMISSILES_SPECIAL,
    BI_AMMO_COUNT, BI_AMMO_POWER, BI_AMMO_SIDES, BI_AMMO_TVAL, BI_APFE, BI_APHASE, BI_ARECHARGE,
    BI_ARESCOLD, BI_ARESHEAT, BI_ARESPOIS, BI_ARMOR, BI_AROD1, BI_AROD2, BI_ASHROOM, BI_ASPEED,
    BI_ASTFDEST, BI_ASTFMAGI, BI_ASTR, BI_ATELEPORT, BI_ATELEPORTLVL, BI_ATPORTOTHER, BI_AWIS,
    BI_BART, BI_BID, BI_BLOWS, BI_BTODAM, BI_BTOHIT, BI_CARRY, BI_CCON, BI_CDEPTH, BI_CDEX,
    BI_CINT, BI_CLASS, BI_CLEVEL, BI_CON, BI_CON_INDEX, BI_CRSAGRV, BI_CRSAIRSWING, BI_CRSANNOY,
    BI_CRSAVULN, BI_CRSCLUM, BI_CRSCVULN, BI_CRSDRAIN_XP, BI_CRSDULL, BI_CRSENVELOPING,
    BI_CRSEVULN, BI_CRSFEAR, BI_CRSFVULN, BI_CRSHALU, BI_CRSHPIMP, BI_CRSIRRITATION, BI_CRSMPIMP,
    BI_CRSNOTEL, BI_CRSPARA, BI_CRSPOIS, BI_CRSSDEM, BI_CRSSDRA, BI_CRSSICK, BI_CRSSIREN,
    BI_CRSSLOW, BI_CRSSTEELSKIN, BI_CRSSTONE, BI_CRSSUND, BI_CRSTELE, BI_CRSTWEP, BI_CRSUNKNO,
    BI_CRSVULN, BI_CRSWEAK, BI_CSTR, BI_CURHP, BI_CURSP, BI_CWIS, BI_DAM_RED, BI_DEV, BI_DEX,
    BI_DEX_INDEX, BI_DIG, BI_DINV, BI_DISM, BI_DISP, BI_EMPTY, BI_ESP, BI_EXTRA_BLOWS, BI_FAIL1,
    BI_FAIL2, BI_FAST_SHOTS, BI_FEATH, BI_FIRST_CURSED, BI_FOOD, BI_FOOD_CURE_BLIND,
    BI_FOOD_CURE_CONF, BI_FOOD_HI, BI_FOOD_LO, BI_FRACT, BI_GOLD, BI_GOOD_S_CHG, BI_GOOD_W_CHG,
    BI_HASFIXEXP, BI_HEAVYBOW, BI_HEAVYWEPON, BI_HLIFE, BI_HP_ADJ, BI_IACID, BI_ICOLD, BI_IELEC,
    BI_IFIRE, BI_INFRA, BI_INT, BI_INT_INDEX, BI_IPOIS, BI_ISAFRAID, BI_ISBLIND, BI_ISCONFUSED,
    BI_ISCUT, BI_ISENCUMB, BI_ISFIXALL, BI_ISFIXCON, BI_ISFIXDEX, BI_ISFIXEXP, BI_ISFIXINT,
    BI_ISFIXLEV, BI_ISFIXSTR, BI_ISFIXWIS, BI_ISFORGET, BI_ISFULL, BI_ISGORGED, BI_ISHEAVYSTUN,
    BI_ISHUNGRY, BI_ISIMAGE, BI_ISPARALYZED, BI_ISPOISONED, BI_ISSTUDY, BI_ISSTUN, BI_ISWEAK,
    BI_KING, BI_LIGHT, BI_MAX, BI_MAXCLEVEL, BI_MAXDEPTH, BI_MAXHP, BI_MAXSP, BI_MOD_MOVES,
    BI_MULTIPLE_BONUSES, BI_NEED_BRAND_WEAPON, BI_NEED_ENCHANT_TO_A, BI_NEED_ENCHANT_TO_D,
    BI_NEED_ENCHANT_TO_H, BI_PREP_BIG_FIGHT, BI_QUIVER_SLOTS, BI_RACID, BI_RBLIND, BI_RCOLD,
    BI_RCONF, BI_RDARK, BI_RDIS, BI_RECALL, BI_REG, BI_RELEC, BI_RFEAR, BI_RFIRE, BI_RKAOS,
    BI_RLITE, BI_RNTHR, BI_RNXUS, BI_RPOIS, BI_RSHRD, BI_RSND, BI_SAURON_DEAD, BI_SAV, BI_SCON,
    BI_SDEX, BI_SDIG, BI_SFRACT, BI_SHLIFE, BI_SHOTS, BI_SINT, BI_SINV, BI_SLING, BI_SPEED,
    BI_SP_ADJ, BI_SRACID, BI_SRBLIND, BI_SRCH, BI_SRCOLD, BI_SRCONF, BI_SRDARK, BI_SRDIS,
    BI_SRELEC, BI_SRFEAR, BI_SRFIRE, BI_SRKAOS, BI_SRLITE, BI_SRNTHR, BI_SRNXUS, BI_SRPOIS,
    BI_SRSHRD, BI_SRSND, BI_SSTR, BI_STL, BI_STR, BI_STR_INDEX, BI_SWIS, BI_THB, BI_THN, BI_THT,
    BI_TODAM, BI_TOHIT, BI_WB_ACID, BI_WB_COLD, BI_WB_ELEC, BI_WB_FIRE, BI_WB_POIS, BI_WDD, BI_WDS,
    BI_WEIGHT, BI_WHERE_CURSED, BI_WID, BI_WIS, BI_WIS_INDEX, BI_WK_DEMON, BI_WK_DRAGON,
    BI_WK_UNDEAD, BI_WORN_NEED_ID, BI_WS_ANIMAL, BI_WS_DEMON, BI_WS_DRAGON, BI_WS_EVIL,
    BI_WS_GIANT, BI_WS_ORC, BI_WS_TROLL, BI_WS_UNDEAD, BI_WTODAM, BI_WTOHIT, BI_W_IMPACT,
};
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddd, ddx, ddx_ddd, ddy, ddy_ddd, distance, feat_is_shop, heatmap,
    monster_group, square, square_in_bounds, square_isvault, trap, C2RustUnnamedH_18, FEAT_MAX,
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
pub use crate::headers::list_player_timed_h::{
    TMD_AFRAID, TMD_AMNESIA, TMD_ATT_ACID, TMD_ATT_COLD, TMD_ATT_CONF, TMD_ATT_DEMON, TMD_ATT_ELEC,
    TMD_ATT_EVIL, TMD_ATT_FIRE, TMD_ATT_POIS, TMD_ATT_RUN, TMD_ATT_VAMP, TMD_BLACKBREATH,
    TMD_BLESSED, TMD_BLIND, TMD_BLOODLUST, TMD_BOLD, TMD_COMMAND, TMD_CONFUSED, TMD_COVERTRACKS,
    TMD_CUT, TMD_FAST, TMD_FASTCAST, TMD_FOOD, TMD_FREE_ACT, TMD_HEAL, TMD_HERO, TMD_IMAGE,
    TMD_INVULN, TMD_OPP_ACID, TMD_OPP_COLD, TMD_OPP_CONF, TMD_OPP_ELEC, TMD_OPP_FIRE, TMD_OPP_POIS,
    TMD_PARALYZED, TMD_POISONED, TMD_POWERSHOT, TMD_PROTEVIL, TMD_SCRAMBLE, TMD_SHERO, TMD_SHIELD,
    TMD_SINFRA, TMD_SINVIS, TMD_SLOW, TMD_SPRINT, TMD_STEALTH, TMD_STONESKIN, TMD_STUN, TMD_TAUNT,
    TMD_TELEPATHY, TMD_TERROR, TMD_TRAPSAFE,
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
    player_shape, player_state, player_upkeep, quest, start_item,
};
pub use crate::headers::player_timed_h::{C2RustUnnamedH_22, TMD_MAX};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
unsafe extern "C" fn borg_heal(mut danger: ::core::ffi::c_int) -> bool {
    let mut hp_down: ::core::ffi::c_int = 0;
    let mut pct_down: ::core::ffi::c_int = 0;
    let mut allow_fail: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut chance: ::core::ffi::c_int = 0;
    let mut clw_heal: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    let mut csw_heal: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut ccw_heal: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut cmw_heal: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut heal_heal: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
    let mut stats_needing_fix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rod_good: bool = false_0 != 0;
    hp_down = crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize];
    pct_down = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 100 as ::core::ffi::c_int
        / crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize];
    clw_heal = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 15 as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int;
    csw_heal = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 20 as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int;
    ccw_heal = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 25 as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int;
    cmw_heal = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 30 as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int;
    heal_heal = (crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize])
        * 35 as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int;
    if clw_heal < 15 as ::core::ffi::c_int {
        clw_heal = 15 as ::core::ffi::c_int;
    }
    if csw_heal < 25 as ::core::ffi::c_int {
        csw_heal = 25 as ::core::ffi::c_int;
    }
    if ccw_heal < 30 as ::core::ffi::c_int {
        ccw_heal = 30 as ::core::ffi::c_int;
    }
    if cmw_heal < 50 as ::core::ffi::c_int {
        cmw_heal = 50 as ::core::ffi::c_int;
    }
    if heal_heal < 300 as ::core::ffi::c_int {
        heal_heal = 300 as ::core::ffi::c_int;
    }
    if borg_slot(TV_ROD as ::core::ffi::c_int, sv_rod_healing) != -(1 as ::core::ffi::c_int) {
        if borg_activate_failure(TV_ROD as ::core::ffi::c_int, sv_rod_healing)
            < 500 as ::core::ffi::c_int
        {
            rod_good = true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXSTR as ::core::ffi::c_int) as usize]
        != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXINT as ::core::ffi::c_int) as usize]
        != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXWIS as ::core::ffi::c_int) as usize]
        != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXDEX as ::core::ffi::c_int) as usize]
        != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXCON as ::core::ffi::c_int) as usize]
        != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        && crate::bridge::st().borg.trait_[(BI_ISFIXINT as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
        && crate::bridge::st().borg.trait_[(BI_ISFIXWIS as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_DRUID
        && crate::bridge::st().borg.trait_[(BI_ISFIXWIS as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER
        && crate::bridge::st().borg.trait_[(BI_ISFIXINT as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
        && crate::bridge::st().borg.trait_[(BI_ISFIXCON as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] <= 850 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ISFIXCON as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 1;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] <= 700 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ISFIXCON as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 3 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
        && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] < 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ISFIXWIS as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] < 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ISFIXINT as ::core::ffi::c_int) as usize]
            != 0
    {
        stats_needing_fix += 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
        != 0
    {
        if pct_down >= 80 as ::core::ffi::c_int
            && danger - heal_heal < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            && borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Fixing Confusion. Level 1\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if pct_down >= 85 as ::core::ffi::c_int
            && danger
                >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    * 2 as ::core::ffi::c_int
            && (borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Fixing Confusion. Level 1.a\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + csw_heal
            && (borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind)
                as ::core::ffi::c_int
                != 0
                || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
                || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.cure_confusion) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Fixing Confusion. Level 2\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if borg_equips_staff_fail(sv_staff_teleportation) {
            if borg_activate_failure(TV_STAFF as ::core::ffi::c_int, sv_staff_teleportation)
                > 650 as ::core::ffi::c_int
                && danger
                    < (crate::bridge::st().borg.avoidance as ::core::ffi::c_int + ccw_heal) * 15 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int
                && (borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
                    || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0)
            {
                borg_note(
                    b"# Fixing Confusion. Level 3\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            } else if danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                borg_note(
                    b"# Too scary to fix Confusion. Level 4\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return false_0 != 0;
            }
        } else if borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Fixing Confusion. Level 5\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        && (Rand_div(100 as uint32_t) as int32_t) < 85 as int32_t
    {
        if danger
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 25 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
        {
            if borg_equips_staff_fail(sv_staff_teleportation) {
                return 0 as ::core::ffi::c_int != 0;
            }
        }
        if hp_down >= 300 as ::core::ffi::c_int
            && borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if !(crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                > crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 4 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_ESP as ::core::ffi::c_int) as usize] != 0)
        {
            if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery)
                as ::core::ffi::c_int
                != 0
                || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
                || borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(sv_staff_curing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            {
                borg_note(b"# Fixing Blindness.\0" as *const u8 as *const ::core::ffi::c_char);
                return true_0 != 0;
            }
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            != 0)
        && (hp_down >= 400 as ::core::ffi::c_int
            || danger
                > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    * 5 as ::core::ffi::c_int
                && hp_down > 100 as ::core::ffi::c_int)
        && borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
    {
        borg_note(b"# Fixing Confusion/Blind.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            <= 700 as ::core::ffi::c_int
            && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                > 250 as ::core::ffi::c_int
                && borg_spell_fail(HOLY_WORD, 14 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || stats_needing_fix >= 5 as ::core::ffi::c_int
                    && borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0
                || hp_down > 500 as ::core::ffi::c_int
                    && crate::bridge::st().borg.has[(borg_lookup_kind( TV_POTION as ::core::ffi::c_int, sv_potion_star_healing, )) as usize]
                        <= 0 as ::core::ffi::c_int
                    && borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.heal1) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.heal2) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.heal3) as ::core::ffi::c_int != 0
                || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < 250 as ::core::ffi::c_int
                    && borg_spell_fail(HOLY_WORD, 5 as ::core::ffi::c_int) as ::core::ffi::c_int
                        != 0
                || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    > 550 as ::core::ffi::c_int
                    && borg_spell_fail(HOLY_WORD, 15 as ::core::ffi::c_int) as ::core::ffi::c_int
                        != 0
                || borg_spell_fail(HEALING, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0
                || borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Healing in Questor Combat.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 5 as ::core::ffi::c_int
        && (Rand_div(100 as uint32_t) as int32_t) < 50 as int32_t
    {
        if borg_use_staff_fail(sv_staff_the_magi) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.staff_magi) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Use Magi Staff\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] < 70 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                > 200 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
            || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                && danger < crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
                + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
                && danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
        {
            if borg_use_staff_fail(sv_staff_the_magi) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.staff_magi) as ::core::ffi::c_int != 0
            {
                borg_note(b"# Restored My Mana\0" as *const u8 as *const ::core::ffi::c_char);
                return true_0 != 0;
            }
        }
    }
    if hp_down == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if danger == 0 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            == 0
        && crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] == 0
    {
        return false_0 != 0;
    }
    if stats_needing_fix >= 5 as ::core::ffi::c_int
        && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] > 650 as ::core::ffi::c_int
        && (borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_restoring) as ::core::ffi::c_int
            != 0
            || borg_activate_item(crate::bridge::st().act.restore_all) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Trying to fix stats in combat.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    chance = Rand_div(100 as uint32_t) as int32_t as ::core::ffi::c_int;
    if crate::bridge::st().borg.near.unique != 0 {
        chance -= 10 as ::core::ffi::c_int;
    }
    if danger >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        && danger < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
    {
        chance -= 75 as ::core::ffi::c_int;
    } else if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_PRIEST
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_PALADIN
    {
        chance -= 25 as ::core::ffi::c_int;
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] != 0 {
        chance += 5 as ::core::ffi::c_int;
    }
    if (pct_down <= 15 as ::core::ffi::c_int && chance < 98 as ::core::ffi::c_int
        || pct_down >= 16 as ::core::ffi::c_int
            && pct_down <= 25 as ::core::ffi::c_int
            && chance < 95 as ::core::ffi::c_int
        || pct_down >= 26 as ::core::ffi::c_int
            && pct_down <= 50 as ::core::ffi::c_int
            && chance < 80 as ::core::ffi::c_int
        || pct_down >= 51 as ::core::ffi::c_int
            && pct_down <= 65 as ::core::ffi::c_int
            && chance < 50 as ::core::ffi::c_int
        || pct_down >= 66 as ::core::ffi::c_int
            && pct_down <= 74 as ::core::ffi::c_int
            && chance < 25 as ::core::ffi::c_int
        || pct_down >= 75 as ::core::ffi::c_int && chance < 1 as ::core::ffi::c_int)
        && (crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
            == 0
            && crate::bridge::st().borg.trait_[(BI_ISSTUN as ::core::ffi::c_int) as usize]
                == 0
            && crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
                == 0
            && crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] == 0)
    {
        return false_0 != 0;
    }
    if pct_down >= 30 as ::core::ffi::c_int
        && (pct_down <= 40 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int)
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + clw_heal
        && clw_heal > danger / 3 as ::core::ffi::c_int
        && (borg_spell_fail(MINOR_HEALING, allow_fail) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_light) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 1.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down >= 40 as ::core::ffi::c_int
        && (pct_down <= 50 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 20 as ::core::ffi::c_int)
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + csw_heal
        && csw_heal > danger / 3 as ::core::ffi::c_int
        && (borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_serious) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 2.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down >= 50 as ::core::ffi::c_int
        && pct_down <= 55 as ::core::ffi::c_int
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + ccw_heal
        && ccw_heal > danger / 3 as ::core::ffi::c_int
        && (borg_activate_item(crate::bridge::st().act.cure_critical) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 3.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if danger >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        && danger < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 50 as ::core::ffi::c_int
        && danger < ccw_heal
        && borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
    {
        borg_note(b"# Healing Level 5.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        >= 80 as ::core::ffi::c_int
        && danger < 50 as ::core::ffi::c_int
        && pct_down >= 20 as ::core::ffi::c_int
        && borg_quaff_potion(sv_potion_cure_critical) as ::core::ffi::c_int != 0
    {
        borg_note(b"# Healing Level 5B.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down >= 55 as ::core::ffi::c_int
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + heal_heal
        && ((crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
            == 0
            && crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                == 0
            || rod_good as ::core::ffi::c_int != 0)
            && borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_nonorlybig) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal3) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
            || borg_spell_fail(HEALING, allow_fail) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 6.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= 98 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
        && crate::bridge::st().borg.near.unique == 0
        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] != CLASS_PRIEST
    {
        return false_0 != 0;
    }
    if pct_down > 50 as ::core::ffi::c_int
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + heal_heal
        && (borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
            || crate::bridge::st().borg.near.evil_unique as ::core::ffi::c_int != 0
                && borg_spell_fail(HOLY_WORD, allow_fail) as ::core::ffi::c_int != 0
            || borg_spell_fail(HEALING, allow_fail) as ::core::ffi::c_int != 0
            || (crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
                == 0
                && crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                    == 0
                || rod_good as ::core::ffi::c_int != 0)
                && borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 7.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down > 60 as ::core::ffi::c_int
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + heal_heal
        && (crate::bridge::st().borg.near.evil_unique as ::core::ffi::c_int != 0
            && borg_spell_fail(HOLY_WORD, allow_fail) as ::core::ffi::c_int != 0
            || (crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
                == 0
                && crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                    == 0
                || rod_good as ::core::ffi::c_int != 0)
                && borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_spell_fail(HEALING, allow_fail) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_nonorlybig) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal3) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 8.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down > 65 as ::core::ffi::c_int
        && danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] + heal_heal
        && (crate::bridge::st().borg.near.evil_unique as ::core::ffi::c_int != 0
            && borg_spell_fail(HOLY_WORD, allow_fail) as ::core::ffi::c_int != 0
            || borg_spell_fail(HEALING, allow_fail) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
            || (crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
                == 0
                && crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                    == 0
                || rod_good as ::core::ffi::c_int != 0)
                && borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_nonorlybig) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal1) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.heal3) as ::core::ffi::c_int != 0
            || crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                && (borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                    || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                    || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0))
    {
        borg_note(b"# Healing Level 9.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if pct_down > 75 as ::core::ffi::c_int
        && danger > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        && crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
            + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
            <= 0 as ::core::ffi::c_int
        && (borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Healing Level 10.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if danger
        > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
    {
        if borg_spell_fail(CURE_POISON, 60 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(HERBAL_CURING, 60 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_poison) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_body) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_critical) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_temp) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.rem_fear_pois) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_full2) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_nonorlybig) as ::core::ffi::c_int != 0
            || borg_use_staff(sv_staff_curing) as ::core::ffi::c_int != 0
            || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_fast_recovery)
                as ::core::ffi::c_int
                != 0
            || borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_purging) as ::core::ffi::c_int
                != 0
            || borg_activate_item(crate::bridge::st().act.shroom_purging) as ::core::ffi::c_int != 0
            || borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
            || borg_spell_fail(HEALING, 60 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_spell_fail(HOLY_WORD, 60 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Curing.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if (borg_spell_legal(CURE_POISON) as ::core::ffi::c_int != 0
            || borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0)
            && (borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Curing next round.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
        != 0
        && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 20 as ::core::ffi::c_int)
    {
        let mut sv_mana: ::core::ffi::c_int =
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize];
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
            crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize];
        if borg_spell(CURE_POISON) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
            || borg_spell(HOLY_WORD) as ::core::ffi::c_int != 0
            || borg_spell(HEALING) as ::core::ffi::c_int != 0
        {
            borg_note(
                b"# Emergency Cure Poison! Gasp!!!....\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = sv_mana;
        if borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 20 as ::core::ffi::c_int
            && (borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Healing. Curing section.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if borg_quaff_unknown() {
            return true_0 != 0;
        }
        if borg_eat_unknown() {
            return true_0 != 0;
        }
        if borg_use_unknown() {
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 3 as ::core::ffi::c_int
            || (Rand_div(100 as uint32_t) as int32_t) < 20 as int32_t)
    {
        if borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_quaff_crit(
                crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0
            || borg_spell(MINOR_HEALING) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_critical) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 2 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 20 as ::core::ffi::c_int)
    {
        let mut sv_mana_0: ::core::ffi::c_int =
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize];
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
            crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize];
        if borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 20 as ::core::ffi::c_int
            && (borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Healing.  Bleeding.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if borg_spell(MINOR_HEALING) {
            borg_note(
                b"# Emergency Wound Patch! Gasp!!!....\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = sv_mana_0;
        if borg_quaff_unknown() {
            return true_0 != 0;
        }
        if borg_eat_unknown() {
            return true_0 != 0;
        }
        if borg_use_unknown() {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_caution() -> bool {
    let mut j: ::core::ffi::c_int = 0;
    let mut pos_danger: ::core::ffi::c_int = 0;
    let mut borg_surround: bool = false_0 != 0;
    let mut nasty: bool = false_0 != 0;
    let mut on_dnstair: bool = false_0 != 0;
    let mut on_upstair: bool = false_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
        && ((*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize))
        .timeout as ::core::ffi::c_int)
            < 250 as ::core::ffi::c_int
    {
        nasty = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
    {
        nasty = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
        != 0
    {
        nasty = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
        != 0
    {
        nasty = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISIMAGE as ::core::ffi::c_int) as usize]
        != 0
    {
        nasty = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level < 10 as borg_time
        && !crate::bridge::st().borg.morgoth_position
    {
        if crate::bridge::st().borg.ready_morgoth <= 0 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
        {
            if borg_spell(TELEPORT_LEVEL) as ::core::ffi::c_int != 0
                || borg_read_scroll(sv_scroll_teleport_level) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_level) as ::core::ffi::c_int != 0
            {
                borg_note(
                    b"# Rising one dlevel (Not ready for Morgoth)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(
                    b"# Leaving (Not ready for Morgoth now)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
                if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                    return true_0 != 0;
                }
            }
        }
    }
    borg_surround = borg_surrounded();
    if crate::bridge::st().borg.escapes as ::core::ffi::c_int > 3 as ::core::ffi::c_int
        && crate::bridge::st().borg.mon.unique == 0
        && crate::bridge::st().borg.ready_morgoth == 0
        || crate::bridge::st().borg.escapes as ::core::ffi::c_int > 55 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            <= 98 as ::core::ffi::c_int
        {
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(
                    b"# Leaving (Too many escapes)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
            }
            if !crate::bridge::st().borg.goal.fleeing && crate::bridge::st().borg.escapes as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
                borg_note(
                    b"# Fleeing (Too many escapes)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.fleeing = true_0 != 0;
            }
        }
    }
    if crate::bridge::st().borg.mon.scary {
        borg_note(b"# Scary guy on level.\0" as *const u8 as *const ::core::ffi::c_char);
        if !crate::bridge::st().borg.goal.leaving {
            borg_note(
                b"# Leaving (Scary guy on level)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.fleeing {
            borg_note(
                b"# Fleeing (Scary guy on level)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.goal.fleeing_to_town = true_0 != 0;
        }
    }
    if crate::bridge::st().borg.goal.ignoring {
        borg_note(b"# Ignoring combat with monsters.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if borg_dont_react {
        borg_note(b"# Borg ignoring messges.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pos_danger = borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    );
    if (pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || pos_danger
            > borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
                [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
        || crate::bridge::st().borg.morgoth_position as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 100 as ::core::ffi::c_int)
        && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
    {
        borg_note(format(
            b"# Loc:%d,%d Dep:%d Lev:%d HP:%d/%d SP:%d/%d Danger:p=%d\0" as *const u8
                as *const ::core::ffi::c_char,
            crate::bridge::st().borg.c.y,
            crate::bridge::st().borg.c.x,
            crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
            crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize],
            crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize],
            crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize],
            crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize],
            crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize],
            pos_danger,
        ));
        if crate::bridge::st().borg.resistance != 0 {
            borg_note(format(
                b"# Protected by Resistance (borg turns:%d; game turns:%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                crate::bridge::st().borg.resistance as ::core::ffi::c_int / borg_game_ratio as ::core::ffi::c_int,
                *(*player)
                    .timed
                    .offset(TMD_OPP_ACID as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int,
            ));
        }
        if crate::bridge::st().borg.temp.shield {
            borg_note(b"# Protected by Mystic Shield\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if crate::bridge::st().borg.temp.prot_from_evil {
            borg_note(b"# Protected by PFE\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if crate::bridge::st().borg.morgoth_position {
            borg_note(b"# Protected by Sea of Runes.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
            borg_note(b"# Questor Combat.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if crate::bridge::st().borg.status.anti_summon {
            borg_note(
                b"# Protected by anti-summon corridor.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if track_glyph.num != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < track_glyph.num as ::core::ffi::c_int {
            if *track_glyph.y.offset(i as isize) == crate::bridge::st().borg.c.y
                && *track_glyph.x.offset(i as isize) == crate::bridge::st().borg.c.x
            {
                borg_note(format(
                    b"# Standing on Glyph\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            i += 1;
        }
    }
    if track_less.num != 0 {
        let mut i_0: ::core::ffi::c_int = 0;
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < track_less.num as ::core::ffi::c_int {
            if *track_less.y.offset(i_0 as isize) == crate::bridge::st().borg.c.y
                && *track_less.x.offset(i_0 as isize) == crate::bridge::st().borg.c.x
            {
                borg_note(format(
                    b"# Standing on up-stairs\0" as *const u8 as *const ::core::ffi::c_char,
                ));
                on_upstair = false_0 != 0;
            }
            i_0 += 1;
        }
    }
    if track_more.num != 0 {
        let mut i_1: ::core::ffi::c_int = 0;
        i_1 = 0 as ::core::ffi::c_int;
        while i_1 < track_more.num as ::core::ffi::c_int {
            if *track_more.y.offset(i_1 as isize) == crate::bridge::st().borg.c.y
                && *track_more.x.offset(i_1 as isize) == crate::bridge::st().borg.c.x
            {
                borg_note(format(
                    b"# Standing on dn-stairs\0" as *const u8 as *const ::core::ffi::c_char,
                ));
                on_dnstair = false_0 != 0;
            }
            i_1 += 1;
        }
    }
    if !crate::bridge::st().borg.goal.fleeing {
        if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
            && !crate::bridge::st().borg.morgoth_position
            && !crate::bridge::st().borg.status.anti_summon
            && crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
                == 0
            && crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] == 0
            && crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
                == 0
            && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
                == 0
        {
            if borg_defend(pos_danger) {
                return true_0 != 0;
            }
            if borg_heal(pos_danger) {
                return true_0 != 0;
            }
        } else {
            if borg_heal(pos_danger) {
                return true_0 != 0;
            }
            if borg_defend(pos_danger) {
                return true_0 != 0;
            }
        }
    }
    if borg_uses_swaps() {
        if pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int {
            if borg_backup_swap(pos_danger) {
                return true_0 != 0;
            }
        }
    }
    if crate::bridge::st().borg.goal.recalling != 0
        && borg_check_rest(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x) as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
        && crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
            == 0
    {
        borg_note(b"# Resting for Recall.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress('0' as i32 as keycode_t);
        borg_keypress('0' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        crate::bridge::st().borg.in_shop = false_0 != 0;
        return true_0 != 0;
    }
    if crate::bridge::st().borg.goal.recalling != 0
        && crate::bridge::st().borg.goal.recalling <= borg_game_ratio as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
    {
        if borg_prep_leave_level_spells() {
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.near.unique == 0
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town > 200 as borg_time
        && !borg_restock(
            crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
        )
        .is_null()
    {
        if !crate::bridge::st().borg.goal.leaving {
            borg_note(format(
                b"# Leaving (restock) %s\0" as *const u8 as *const ::core::ffi::c_char,
                borg_restock(
                    crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
                ),
            ));
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.fleeing
            && crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize]
                < 2 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                > 3 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_AFUEL as ::core::ffi::c_int) as usize]
                > 2 as ::core::ffi::c_int
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 400 as borg_time
        {
            borg_note(format(
                b"# Fleeing (restock) %s\0" as *const u8 as *const ::core::ffi::c_char,
                borg_restock(
                    crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
                ),
            ));
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
    } else if pos_danger
        > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] * 2 as ::core::ffi::c_int
    {
        if !crate::bridge::st().borg.goal.fleeing
            && crate::bridge::st().borg.near.unique == 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 50 as ::core::ffi::c_int
            && !crate::bridge::st().borg.status.vault
            && (crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < 100 as ::core::ffi::c_int
                && crate::bridge::st().borg.ready_morgoth == 1 as ::core::ffi::c_int)
        {
            borg_note(b"# Fleeing (excessive danger)\0" as *const u8 as *const ::core::ffi::c_char);
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
    } else if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
        && pos_danger > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 50 as ::core::ffi::c_int
    {
        if !crate::bridge::st().borg.goal.leaving {
            borg_note(b"# Leaving (potential danger)\0" as *const u8 as *const ::core::ffi::c_char);
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
    }
    if crate::bridge::st().borg.goal.leaving as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.goal.fleeing as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.goal.fleeing_lunal as ::core::ffi::c_int != 0
        || crate::bridge::st().borg.goal.fleeing_munchkin as ::core::ffi::c_int != 0
        || (pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 5 as ::core::ffi::c_int
                && pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
            && on_upstair as ::core::ffi::c_int != 0
    {
        if crate::bridge::st().borg.ready_morgoth == 0 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            crate::bridge::st().borg.stair_less = true_0 != 0;
            if crate::bridge::st().borg.mon.scary {
                borg_note(
                    b"# Fleeing and leaving the level. (scaryguy)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if crate::bridge::st().borg.goal.fleeing_lunal {
                borg_note(
                    b"# Fleeing and leaving the level. (fleeing_lunal)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if crate::bridge::st().borg.goal.fleeing_munchkin {
                borg_note(
                    b"# Fleeing and leaving the level. (fleeing munchkin)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    <= 49 as ::core::ffi::c_int
                && (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat
                    as ::core::ffi::c_int
                    == FEAT_LESS as ::core::ffi::c_int
            {
                borg_note(
                    b"# Leaving level,  Some danger but I'm on a stair.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            crate::bridge::st().borg.stair_less = true_0 != 0;
        }
        if crate::bridge::st().borg.goal.fleeing as ::core::ffi::c_int == true_0
            || crate::bridge::st().borg.goal.fleeing_lunal as ::core::ffi::c_int == true_0
            || crate::bridge::st().borg.goal.fleeing_munchkin as ::core::ffi::c_int != 0
        {
            crate::bridge::st().borg.stair_more = true_0 != 0;
        }
        if (borg_prepared(
            crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                + 1 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char)
            .is_null()
        {
            crate::bridge::st().borg.stair_more = true_0 != 0;
        }
        if track_less.num as ::core::ffi::c_int != 0
            && (crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
                || crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
                    != 0
                || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
                    != 0
                || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                    < 2 as ::core::ffi::c_int)
        {
            crate::bridge::st().borg.stair_more = false_0 != 0;
        }
        if track_less.num as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                != 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 25 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                < 25000 as ::core::ffi::c_int
            && borg_count_sell() >= 13 as ::core::ffi::c_int
        {
            crate::bridge::st().borg.stair_more = false_0 != 0;
        }
        if crate::bridge::st().borg.mon.scary {
            crate::bridge::st().borg.stair_more = true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
        {
            crate::bridge::st().borg.stair_more = true_0 != 0;
        }
    }
    if crate::bridge::st().borg.stair_less as ::core::ffi::c_int != 0
        && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
    {
        let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
        if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || on_upstair as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Leaving via up stairs.\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            borg_keypress('<' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.stair_more as ::core::ffi::c_int != 0 && crate::bridge::st().borg.goal.recalling == 0 {
        let mut ag_0: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
        if (*ag_0).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || on_dnstair as ::core::ffi::c_int != 0
        {
            if !crate::bridge::st().borg.goal.fleeing_lunal && !crate::bridge::st().borg.goal.fleeing_munchkin {
                if borg_prep_leave_level_spells() {
                    return true_0 != 0;
                }
            }
            borg_keypress('>' as i32 as keycode_t);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 {
        let mut need: borg_need = borg_maintain_light();
        if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return true_0 != 0;
        } else if need as ::core::ffi::c_uint
            == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                != 0
        {
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(b"# Leaving (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
            }
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
    {
        if borg_eat_food_any() as ::core::ffi::c_int != 0
            || borg_spell(REMOVE_HUNGER) as ::core::ffi::c_int != 0
            || borg_spell(HERBAL_CURING) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
        {
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(b"# Leaving (need food)\0" as *const u8 as *const ::core::ffi::c_char);
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
            }
            if !crate::bridge::st().borg.goal.fleeing {
                borg_note(b"# Fleeing (need food)\0" as *const u8 as *const ::core::ffi::c_char);
                crate::bridge::st().borg.goal.fleeing = true_0 != 0;
            }
        }
    }
    if crate::bridge::st().borg.near.breeder as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
    {
        if !crate::bridge::st().borg.goal.fleeing {
            borg_note(b"# Fleeing (breeder level)\0" as *const u8 as *const ::core::ffi::c_char);
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
    }
    if (track_less.num as ::core::ffi::c_int != 0 || track_more.num as ::core::ffi::c_int != 0)
        && (crate::bridge::st().borg.goal.fleeing as ::core::ffi::c_int != 0
            || crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
            || pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 35 as ::core::ffi::c_int)
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut i_2: ::core::ffi::c_int = 0;
        let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut m: ::core::ffi::c_int = 0;
        let mut b_m: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut safe: bool = true_0 != 0;
        let mut ag_1: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
        i_2 = 0 as ::core::ffi::c_int;
        while i_2 < track_less.num as ::core::ffi::c_int {
            x = *track_less.x.offset(i_2 as isize);
            y = *track_less.y.offset(i_2 as isize);
            ag_1 = crate::bridge::grid(x, y) as *mut borg_grid;
            j = distance(crate::bridge::st().borg.c, loc(x, y));
            if !((*ag_1).kill != 0) {
                if !(b_j >= j) {
                    b_j = j;
                }
            }
            i_2 += 1;
        }
        i_2 = 0 as ::core::ffi::c_int;
        while i_2 < track_more.num as ::core::ffi::c_int {
            x = *track_more.x.offset(i_2 as isize);
            y = *track_more.y.offset(i_2 as isize);
            ag_1 = crate::bridge::grid(x, y) as *mut borg_grid;
            m = distance(crate::bridge::st().borg.c, loc(x, y));
            if !((*ag_1).kill != 0) {
                if !(b_m >= m) {
                    b_m = m;
                }
            }
            i_2 += 1;
        }
        if b_j < 3 as ::core::ffi::c_int
            && b_j != -(1 as ::core::ffi::c_int)
            && pos_danger < crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        {
            crate::bridge::st().borg.status.desperate = true_0 != 0;
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Desperate for Stairs (one)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.status.desperate = false_0 != 0;
                return true_0 != 0;
            }
            crate::bridge::st().borg.status.desperate = false_0 != 0;
        }
        if b_j <= 2 as ::core::ffi::c_int && b_j != -(1 as ::core::ffi::c_int) {
            crate::bridge::st().borg.status.desperate = true_0 != 0;
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Desperate for Stairs (two)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.status.desperate = false_0 != 0;
                return true_0 != 0;
            }
            crate::bridge::st().borg.status.desperate = false_0 != 0;
        }
        if b_j < 20 as ::core::ffi::c_int
            && b_j != -(1 as ::core::ffi::c_int)
            && crate::bridge::st().borg.mon.scary as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 20 as ::core::ffi::c_int
        {
            i_2 = 0 as ::core::ffi::c_int;
            while i_2 < 8 as ::core::ffi::c_int {
                x = crate::bridge::st().borg.c.x + ddx_ddd[i_2 as usize] as ::core::ffi::c_int;
                y = crate::bridge::st().borg.c.y + ddy_ddd[i_2 as usize] as ::core::ffi::c_int;
                if square_in_bounds(cave, loc(x, y)) {
                    if !((*crate::bridge::grid(x, y)).kill == 0) {
                        if (*borg_kills
                            .offset((*crate::bridge::grid(x, y)).kill as isize))
                        .speed as ::core::ffi::c_int
                            > crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                        {
                            safe = false_0 != 0;
                        }
                    }
                }
                i_2 += 1;
            }
            if !(crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                <= 5 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                    != 0 as ::core::ffi::c_int
                && crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
                || !safe)
            {
                crate::bridge::st().borg.status.desperate = true_0 != 0;
                if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                    borg_note(
                        b"# Desperate for Stairs (three)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    crate::bridge::st().borg.status.desperate = false_0 != 0;
                    return true_0 != 0;
                }
                crate::bridge::st().borg.status.desperate = false_0 != 0;
            }
        }
        if b_m <= 2 as ::core::ffi::c_int && b_m != -(1 as ::core::ffi::c_int) {
            crate::bridge::st().borg.status.desperate = true_0 != 0;
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
                borg_note(
                    b"# Desperate for Stairs (four)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.status.desperate = false_0 != 0;
                return true_0 != 0;
            }
            crate::bridge::st().borg.status.desperate = false_0 != 0;
        }
    }
    if (pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int / 3 as ::core::ffi::c_int
        && !nasty
        && crate::bridge::st().borg.no_retreat == 0
        || borg_surround as ::core::ffi::c_int != 0 && pos_danger != 0 as ::core::ffi::c_int)
        && !crate::bridge::st().borg.morgoth_position
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon >= 50 as borg_time
        && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
        && !square_isvault(cave, crate::bridge::st().borg.c)
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 500 as ::core::ffi::c_int
    {
        let mut d: ::core::ffi::c_int = 0;
        let mut b_d: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut r: ::core::ffi::c_int = 0;
        let mut b_r: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut b_p: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut p1: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut b_x: ::core::ffi::c_int = crate::bridge::st().borg.c.x;
        let mut b_y: ::core::ffi::c_int = crate::bridge::st().borg.c.y;
        let mut ii: ::core::ffi::c_int = 0;
        let mut current_block_313: u64;
        j = 1 as ::core::ffi::c_int;
        while j < borg_view_n as ::core::ffi::c_int {
            let mut x1: ::core::ffi::c_int = crate::bridge::st().borg.c.x;
            let mut y1: ::core::ffi::c_int = crate::bridge::st().borg.c.y;
            let mut x2: ::core::ffi::c_int = borg_view_x[j as usize] as ::core::ffi::c_int;
            let mut y2: ::core::ffi::c_int = borg_view_y[j as usize] as ::core::ffi::c_int;
            if !(crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
                != 0)
            {
                if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y2, x2) {
                    if !(crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                        >= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                            * 7 as ::core::ffi::c_int
                            / 10 as ::core::ffi::c_int
                        && (track_step.num as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                            && (*track_step.y.offset(
                                (track_step.num as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                                    as isize,
                            ) == y2
                                && *track_step.x.offset(
                                    (track_step.num as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                                        as isize,
                                ) == x2
                                && *track_step.y.offset(
                                    (track_step.num as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                                        as isize,
                                ) == crate::bridge::st().borg.c.y
                                && *track_step.x.offset(
                                    (track_step.num as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                                        as isize,
                                ) == crate::bridge::st().borg.c.x))
                        || crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int >= 300 as ::core::ffi::c_int)
                    {
                        if borg_happy_grid_bold(y2, x2) {
                            if b_r >= 0 as ::core::ffi::c_int {
                                let mut ay: ::core::ffi::c_int =
                                    if y2 > y1 { y2 - y1 } else { y1 - y2 };
                                let mut ax: ::core::ffi::c_int =
                                    if x2 > x1 { x2 - x1 } else { x1 - x2 };
                                if ax > b_r || ay > b_r {
                                    current_block_313 = 6070092595894568400;
                                } else {
                                    current_block_313 = 5790857258831596080;
                                }
                            } else {
                                current_block_313 = 5790857258831596080;
                            }
                            match current_block_313 {
                                6070092595894568400 => {}
                                _ => {
                                    r = 0 as ::core::ffi::c_int;
                                    loop {
                                        let mut ag_2: *mut borg_grid =
                                            ::core::ptr::null_mut::<borg_grid>();
                                        d = borg_goto_dir(y1, x1, y2, x2);
                                        if d == 0 as ::core::ffi::c_int
                                            || d == 5 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                        r += 1;
                                        y1 += ddy[d as usize] as ::core::ffi::c_int;
                                        x1 += ddx[d as usize] as ::core::ffi::c_int;
                                        ag_2 = crate::bridge::grid(x1, y1)
                                            as *mut borg_grid;
                                        if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                                            >= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                                                * 7 as ::core::ffi::c_int
                                                / 10 as ::core::ffi::c_int
                                            && (track_step.num as ::core::ffi::c_int
                                                > 2 as ::core::ffi::c_int
                                                && (*track_step.y.offset(
                                                    (track_step.num as ::core::ffi::c_int
                                                        - 2 as ::core::ffi::c_int)
                                                        as isize,
                                                ) == y1
                                                    && *track_step.x.offset(
                                                        (track_step.num as ::core::ffi::c_int
                                                            - 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    ) == x1
                                                    && *track_step.y.offset(
                                                        (track_step.num as ::core::ffi::c_int
                                                            - 3 as ::core::ffi::c_int)
                                                            as isize,
                                                    ) == crate::bridge::st().borg.c.y
                                                    && *track_step.x.offset(
                                                        (track_step.num as ::core::ffi::c_int
                                                            - 3 as ::core::ffi::c_int)
                                                            as isize,
                                                    ) == crate::bridge::st().borg.c.x))
                                            || crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int
                                                >= 300 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                        if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag_2))
                                            || (*ag_2).feat as ::core::ffi::c_int
                                                == FEAT_LAVA as ::core::ffi::c_int
                                                && crate::bridge::st().borg.trait_[(BI_IFIRE as ::core::ffi::c_int) as usize]
                                                    == 0
                                        {
                                            break;
                                        }
                                        if r >= 10 as ::core::ffi::c_int {
                                            break;
                                        }
                                        p1 = borg_danger(
                                            y1,
                                            x1,
                                            1 as ::core::ffi::c_int,
                                            true_0 != 0,
                                            false_0 != 0,
                                        );
                                        if p1 >= pos_danger {
                                            break;
                                        }
                                        if r == 1 as ::core::ffi::c_int {
                                            if !borg_surround
                                                || borg_surround as ::core::ffi::c_int != 0
                                                    && crate::bridge::st().borg.goal.ignoring as ::core::ffi::c_int != 0
                                            {
                                                if p1
                                                    >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] * 4 as ::core::ffi::c_int
                                                        / 10 as ::core::ffi::c_int
                                                {
                                                    break;
                                                }
                                                if p1
                                                    > pos_danger * 5 as ::core::ffi::c_int
                                                        / 10 as ::core::ffi::c_int
                                                {
                                                    break;
                                                }
                                            } else if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                                                >= 20 as ::core::ffi::c_int
                                            {
                                                if p1
                                                    >= (if b_r <= 5 as ::core::ffi::c_int {
                                                        crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] * 15 as ::core::ffi::c_int
                                                            / 10 as ::core::ffi::c_int
                                                    } else {
                                                        crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                                                    })
                                                {
                                                    break;
                                                }
                                            } else if p1
                                                >= crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                                                    * 4 as ::core::ffi::c_int
                                            {
                                                break;
                                            }
                                            ii = 1 as ::core::ffi::c_int;
                                            while ii < borg_kills_nxt as ::core::ffi::c_int {
                                                let mut kill: *mut borg_kill =
                                                    ::core::ptr::null_mut::<borg_kill>();
                                                kill = borg_kills.offset(ii as isize)
                                                    as *mut borg_kill;
                                                if !((*kill).r_idx == 0) {
                                                    if !(crate::bridge::st().borg.time.now - (*kill).when
                                                        > 2 as borg_time)
                                                    {
                                                        if distance((*kill).pos, loc(x1, y1))
                                                            <= 1 as ::core::ffi::c_int
                                                            && (*kill).speed as ::core::ffi::c_int
                                                                > crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                                                            && !borg_surround
                                                        {
                                                            break;
                                                        }
                                                    }
                                                }
                                                ii += 1;
                                            }
                                        }
                                        if (*ag_2).kill != 0 {
                                            break;
                                        }
                                        if (*ag_2).trap as ::core::ffi::c_int != 0 && !(*ag_2).glyph
                                        {
                                            break;
                                        }
                                        if !(x1 == x2 && y1 == y2) {
                                            continue;
                                        }
                                        b_r = r;
                                        b_p = p1;
                                        b_x = x2;
                                        b_y = y2;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            j += 1;
        }
        if b_r >= 0 as ::core::ffi::c_int {
            b_d = borg_goto_dir(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, b_y, b_x);
            crate::bridge::st().borg.goal.g.x = crate::bridge::st().borg.c.x + ddx[b_d as usize] as ::core::ffi::c_int;
            crate::bridge::st().borg.goal.g.y = crate::bridge::st().borg.c.y + ddy[b_d as usize] as ::core::ffi::c_int;
            borg_note(format(
                b"# Retreating to %d,%d (distance %d) via %d,%d (%d > %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b_y,
                b_x,
                b_r,
                crate::bridge::st().borg.goal.g.y,
                crate::bridge::st().borg.goal.g.x,
                pos_danger,
                b_p,
            ));
            borg_keypress((b_d + '0' as i32) as keycode_t);
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
            return true_0 != 0;
        }
    }
    if borg_escape(pos_danger) {
        crate::bridge::st().borg.escapes += 1;
        crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
        return true_0 != 0;
    }
    if (pos_danger
        > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 4 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        && !nasty
        && crate::bridge::st().borg.no_retreat == 0
        || borg_surround as ::core::ffi::c_int != 0 && pos_danger != 0 as ::core::ffi::c_int)
        && !crate::bridge::st().borg.morgoth_position
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon >= 50 as borg_time
        && crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
        && !square_isvault(cave, crate::bridge::st().borg.c)
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 500 as ::core::ffi::c_int
    {
        let mut i_3: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut f: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut b_f: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut g_k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii_0: ::core::ffi::c_int = 0;
        let mut adjacent_monster: bool = false_0 != 0;
        b_k = pos_danger;
        if borg_surround {
            b_k = b_k * 12 as ::core::ffi::c_int / 10 as ::core::ffi::c_int;
        }
        b_f = borg_freedom(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x);
        let mut current_block_349: u64;
        i_3 = 0 as ::core::ffi::c_int;
        while i_3 < 8 as ::core::ffi::c_int {
            let mut x_0: ::core::ffi::c_int =
                crate::bridge::st().borg.c.x + ddx_ddd[i_3 as usize] as ::core::ffi::c_int;
            let mut y_0: ::core::ffi::c_int =
                crate::bridge::st().borg.c.y + ddy_ddd[i_3 as usize] as ::core::ffi::c_int;
            let mut ag_3: *mut borg_grid = crate::bridge::grid(x_0, y_0) as *mut borg_grid;
            if !(crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
                != 0)
            {
                if crate::borg_cave_util::borg_cave_floor_grid(&*(ag_3)) {
                    if !((*ag_3).kill != 0) {
                        if !feat_is_shop((*ag_3).feat as ::core::ffi::c_int) {
                            if (*ag_3).trap as ::core::ffi::c_int != 0 && !(*ag_3).glyph {
                                break;
                            }
                            if !(crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                                >= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                                    * 7 as ::core::ffi::c_int
                                    / 10 as ::core::ffi::c_int
                                && (track_step.num as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                                    && (*track_step.y.offset(
                                        (track_step.num as ::core::ffi::c_int
                                            - 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) == y_0
                                        && *track_step.x.offset(
                                            (track_step.num as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) == x_0
                                        && *track_step.y.offset(
                                            (track_step.num as ::core::ffi::c_int
                                                - 3 as ::core::ffi::c_int)
                                                as isize,
                                        ) == crate::bridge::st().borg.c.y
                                        && *track_step.x.offset(
                                            (track_step.num as ::core::ffi::c_int
                                                - 3 as ::core::ffi::c_int)
                                                as isize,
                                        ) == crate::bridge::st().borg.c.x))
                                || crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int
                                    >= 300 as ::core::ffi::c_int)
                            {
                                ii_0 = 1 as ::core::ffi::c_int;
                                while ii_0 < borg_kills_nxt as ::core::ffi::c_int {
                                    let mut kill_0: *mut borg_kill =
                                        ::core::ptr::null_mut::<borg_kill>();
                                    kill_0 = borg_kills.offset(ii_0 as isize) as *mut borg_kill;
                                    if !((*kill_0).r_idx == 0) {
                                        if !(crate::bridge::st().borg.time.now - (*kill_0).when > 2 as borg_time) {
                                            if distance((*kill_0).pos, loc(x_0, y_0))
                                                <= 1 as ::core::ffi::c_int
                                                && !borg_surround
                                            {
                                                adjacent_monster = true_0 != 0;
                                            }
                                            if distance((*kill_0).pos, loc(x_0, y_0))
                                                <= 2 as ::core::ffi::c_int
                                                && (*kill_0).speed as ::core::ffi::c_int
                                                    > crate::bridge::st().borg.trait_[(BI_SPEED as ::core::ffi::c_int) as usize]
                                                && !borg_surround
                                            {
                                                adjacent_monster = true_0 != 0;
                                            }
                                        }
                                    }
                                    ii_0 += 1;
                                }
                                if !(adjacent_monster as ::core::ffi::c_int == true_0) {
                                    k = borg_danger(
                                        y_0,
                                        x_0,
                                        1 as ::core::ffi::c_int,
                                        true_0 != 0,
                                        false_0 != 0,
                                    );
                                    if !(k > crate::bridge::st().borg.avoidance as ::core::ffi::c_int) {
                                        if !(crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                                            >= 35 as ::core::ffi::c_int
                                            && k > b_k * 6 as ::core::ffi::c_int
                                                / 10 as ::core::ffi::c_int)
                                        {
                                            if !(crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] < 35 as ::core::ffi::c_int
                                                && adjacent_monster as ::core::ffi::c_int
                                                    == false_0
                                                && k > b_k * 8 as ::core::ffi::c_int
                                                    / 10 as ::core::ffi::c_int)
                                            {
                                                if !(k > b_k) {
                                                    g_k = k;
                                                    f = borg_freedom(y_0, x_0);
                                                    if b_k == k {
                                                        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize] <= 10 as ::core::ffi::c_int
                                                            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] != 0
                                                            && (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] || crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize])
                                                        {
                                                            current_block_349 = 8940662058537996670;
                                                        } else if b_f > f
                                                            || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize] >= 85 as ::core::ffi::c_int
                                                        {
                                                            current_block_349 =
                                                                11231108819225936288;
                                                        } else {
                                                            current_block_349 = 8940662058537996670;
                                                        }
                                                    } else {
                                                        current_block_349 = 8940662058537996670;
                                                    }
                                                    match current_block_349 {
                                                        11231108819225936288 => {}
                                                        _ => {
                                                            b_i = i_3;
                                                            b_k = k;
                                                            b_f = f;
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
            i_3 += 1;
        }
        if b_i >= 0 as ::core::ffi::c_int {
            crate::bridge::st().borg.goal.g.x = crate::bridge::st().borg.c.x + ddx_ddd[b_i as usize] as ::core::ffi::c_int;
            crate::bridge::st().borg.goal.g.y = crate::bridge::st().borg.c.y + ddy_ddd[b_i as usize] as ::core::ffi::c_int;
            borg_note(format(
                b"# Backing up to %d,%d (%d > %d)\0" as *const u8 as *const ::core::ffi::c_char,
                crate::bridge::st().borg.goal.g.x,
                crate::bridge::st().borg.goal.g.y,
                pos_danger,
                g_k,
            ));
            borg_keypress((ddd[b_i as usize] as ::core::ffi::c_int + '0' as i32) as keycode_t);
            crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
        != 0
    {
        if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            >= 300 as ::core::ffi::c_int
            && (borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Healing.  Confusion.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
            || borg_quaff_crit(false_0 != 0) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_confusion) as ::core::ffi::c_int != 0
            || borg_use_staff_fail(sv_staff_healing) as ::core::ffi::c_int != 0
        {
            borg_note(b"# Healing.  Confusion.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_CRSFEAR as ::core::ffi::c_int) as usize]
            == 0
        && ((Rand_div(100 as uint32_t) as int32_t) < 70 as int32_t
            || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR
                && crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                    <= 0 as ::core::ffi::c_int)
    {
        if borg_eat(TV_MUSHROOM as ::core::ffi::c_int, sv_mush_cure_mind) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_boldness) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_heroism) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_berserk) as ::core::ffi::c_int != 0
            || borg_spell_fail(BERSERK_STRENGTH, 25 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
            || borg_spell_fail(HEROISM, 25 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_paranoia) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.hero) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.shero) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.cure_mind) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.rage_bless_resist) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.rem_fear_pois) as ::core::ffi::c_int != 0
            || borg_spell_fail(HOLY_WORD, 25 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] / 3 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    * 3 as ::core::ffi::c_int)
        && crate::bridge::st().borg.trait_[(BI_ACCW as ::core::ffi::c_int) as usize] < 3 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_AHEAL as ::core::ffi::c_int) as usize] < 1 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
            && (Rand_div(100 as uint32_t) as int32_t) < 25 as int32_t
        {
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(
                    b"# Leaving (low hit-points)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
            }
            if !crate::bridge::st().borg.goal.fleeing {
                borg_note(
                    b"# Fleeing (low hit-points)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.fleeing = true_0 != 0;
            }
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0)
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                / 2 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
            && (Rand_div(100 as uint32_t) as int32_t) < 25 as int32_t
        {
            if !crate::bridge::st().borg.goal.leaving {
                borg_note(
                    b"# Leaving (bleeding/poison)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
            }
            if !crate::bridge::st().borg.goal.fleeing {
                borg_note(
                    b"# Fleeing (bleeding/poison)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                crate::bridge::st().borg.goal.fleeing = true_0 != 0;
            }
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
        < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize] / 10 as ::core::ffi::c_int
        || pos_danger > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            && (crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
                + crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
                <= 2 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        / 4 as ::core::ffi::c_int)
        || crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            > 5 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 4 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            >= 600 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                >= 85 as ::core::ffi::c_int)
        && (borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
            || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0)
    {
        borg_note(b"# Using reserve EZ_Heal.\0" as *const u8 as *const ::core::ffi::c_char);
        return true_0 != 0;
    }
    if crate::bridge::st().borg.goal.fleeing as ::core::ffi::c_int != 0
        && !crate::bridge::st().borg.goal.fleeing_munchkin
        && !crate::bridge::st().borg.goal.fleeing_to_town
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
    {
        if borg_recall() {
            borg_note(b"# Fleeing the level (recall)\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.goal.recalling != 0
        && pos_danger > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.trait_[(BI_ISCONFUSED as ::core::ffi::c_int) as usize]
            == 0
            && crate::bridge::st().borg.trait_[(BI_ISBLIND as ::core::ffi::c_int) as usize]
                == 0
            && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                > 60 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                    / 4 as ::core::ffi::c_int
            && (borg_quaff_potion(sv_potion_restore_mana) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.restore_mana) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Buying time waiting for Recall.(1)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if borg_caution_phase(50 as ::core::ffi::c_int, 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
            && (borg_read_scroll(sv_scroll_phase_door) as ::core::ffi::c_int != 0
                || borg_spell_fail(PHASE_DOOR, 30 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, 30 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Buying time waiting for Recall.(2)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            < 100 as ::core::ffi::c_int
            && (borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Buying time waiting for Recall.(3)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            > 150 as ::core::ffi::c_int
            && (borg_zap_rod(sv_rod_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_star_healing) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_life) as ::core::ffi::c_int != 0
                || borg_quaff_crit(true_0 != 0) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_cure_serious) as ::core::ffi::c_int != 0
                || borg_quaff_potion(sv_potion_cure_light) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Buying time waiting for Recall.(4)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if pos_danger
        > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] * 4 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize] == 0
    {
        if borg_use_unknown() as ::core::ffi::c_int != 0
            || borg_read_unknown() as ::core::ffi::c_int != 0
            || borg_quaff_unknown() as ::core::ffi::c_int != 0
            || borg_eat_unknown() as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
