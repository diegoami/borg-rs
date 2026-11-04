pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const CLASS_WARRIOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DUNGEON_WID: ::core::ffi::c_int = 198 as ::core::ffi::c_int;
pub const DUNGEON_HGT: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const AUTO_MAX_X: ::core::ffi::c_int = DUNGEON_WID;
pub const AUTO_MAX_Y: ::core::ffi::c_int = DUNGEON_HGT;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

use crate::headers::borg_danger_h::{borg_danger, borg_fear_region};
pub use crate::headers::borg_flow_h::borg_track;
pub use crate::headers::borg_flow_kill_h::{
    borg_kill, borg_kills, borg_kills_nxt, borg_race_death, borg_shoot_scoot_safe,
};
use crate::headers::borg_flow_stairs_h::{track_less, track_more};
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_keypress, borg_note};

use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_read_scroll, borg_use_staff, borg_use_staff_fail, borg_zap_rod,
};

pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_fail, borg_spell_okay_fail, borg_spells, ACID_SPRAY, ANNIHILATE,
    BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN,
    BERSERK_STRENGTH, BLESS, BLOODLUST, BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND,
    CONFUSE_MONSTER, COVER_TRACKS, CREATE_ARROWS, CREATE_DARKNESS, CRUSH, CURE_POISON, CURSE,
    DARK_SPEAR, DECOY, DEMON_BANE, DETECTION, DETECT_EVIL, DETECT_LIFE, DETECT_MONSTERS,
    DETECT_STAIRS, DIMENSION_DOOR, DISABLE_TRAPS_DESTROY_DOORS, DISENCHANT, DISPEL_EVIL,
    DISPEL_LIFE, DISPEL_UNDEAD, DOOR_CREATION, EAGLES_FLIGHT, EARTH_RISING, ELECTRIC_ARC,
    ENCHANT_ARMOUR, ENCHANT_WEAPON, EXPLOSION, FIND_TRAPS_DOORS_STAIRS, FIRE_BALL, FORCEFUL_BLOW,
    FOX_FORM, FRIGHTEN, FROST_BOLT, FUME_OF_MORDOR, GLYPH_OF_WARDING, GRIM_PURPOSE, GRONDS_BLOW,
    HASTE_SELF, HEALING, HERBAL_CURING, HEROISM, HIT_AND_RUN, HOLY_WORD, HOWL_OF_THE_DAMNED,
    ICE_STORM, IDENTIFY_RUNE, LEAP_INTO_BATTLE, LIGHTNING_STRIKE, LIGHT_OF_VARDA, LIGHT_ROOM,
    MAGIC_MISSILE, MAIM_FOE, MANA_BOLT, MANA_CHANNEL, MANA_STORM, MASS_BANISHMENT, MASS_SLEEP,
    METEOR_SWARM, MINOR_HEALING, NETHER_BOLT, OBJECT_DETECTION, ORB_OF_DRAINING, PHASE_DOOR,
    PORTAL, POWER_SACRIFICE, PROTECTION_FROM_EVIL, QUAKE, RAPID_REGENERATION, READ_MINDS,
    RECHARGING, RELENTLESS_TAUNTING, REMEMBRANCE, REMOVE_CURSE, REMOVE_HUNGER, RESISTANCE,
    RESIST_POISON, RESTORATION, REVEAL_MONSTERS, REVITALIZE, RIFT, RIVER_OF_LIGHTNING, SEEK_BATTLE,
    SENSE_INVISIBLE, SENSE_SURROUNDINGS, SHADOW_SHIFT, SHATTER_STONE, SHOCK_WAVE, SINGLE_COMBAT,
    SLEEP_EVIL, SLOW_MONSTER, SMITE_EVIL, SPEAR_OF_LIGHT, SPEAR_OF_OROME, STINKING_CLOUD,
    STORM_OF_DARKNESS, TAP_MAGICAL_ENERGY, TAP_UNLIFE, TELEPORT_LEVEL, TELEPORT_OTHER,
    TELEPORT_SELF, THRUST_AWAY, TRANCE, TREASURE_DETECTION, TREMOR, TURN_STONE_TO_MUD,
    UNHOLY_REPRIEVE, UNLEASH_CHAOS, VAMPIRE_FORM, VAMPIRE_STRIKE, VENOM, VOLCANIC_ERUPTION,
    WARG_FORM, WEREWOLF_FORM, WHIRLWIND_ATTACK, WORD_OF_DESTRUCTION, WORD_OF_RECALL,
    ZONE_OF_UNMAGIC,
};
use crate::headers::borg_projection_h::{borg_distance, borg_los, borg_target};
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
use crate::headers::borg_update_h::borg_detect_wall;
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddx_ddd, ddy_ddd, distance, feat_is_shop, heatmap, monster_group,
    square, square_in_bounds_fully, square_isno_teleport, trap, C2RustUnnamedH_18, FEAT_MAX,
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
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_recall() -> bool {
    if crate::bridge::st().borg.goal.recalling == 0 {
        if borg_zap_rod(crate::bridge::st().item_val.sv_rod_recall) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.recall) as ::core::ffi::c_int != 0
            || borg_spell_fail(WORD_OF_RECALL, 60 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_word_of_recall) as ::core::ffi::c_int != 0
        {
            if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                    != 0 as ::core::ffi::c_int
            {
                let mut diff_max_from_desired_depth: ::core::ffi::c_int = crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                    - crate::bridge::st().prepared.depth_hunted_unique;
                let mut diff_cur_from_desired_depth: ::core::ffi::c_int = crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                    - crate::bridge::st().prepared.depth_hunted_unique;
                if diff_max_from_desired_depth > 5 as ::core::ffi::c_int
                    && diff_cur_from_desired_depth <= 5 as ::core::ffi::c_int
                    && diff_cur_from_desired_depth > -(5 as ::core::ffi::c_int)
                {
                    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        >= 80 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                            < 100 as ::core::ffi::c_int
                        && *borg_race_death.offset(crate::bridge::st().borg.id.sauron as isize) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Resetting recall depth.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress('y' as i32 as keycode_t);
                    } else if crate::bridge::st().borg.goal.fleeing_munchkin as ::core::ffi::c_int == true_0 {
                        borg_note(
                            b"# Resetting recall depth during munchkin mode.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress('y' as i32 as keycode_t);
                    } else if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        >= 100 as ::core::ffi::c_int
                        && crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] == 0
                    {
                        borg_note(
                            b"# Not Resetting recall depth.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress('n' as i32 as keycode_t);
                    } else if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                        == 99 as ::core::ffi::c_int
                        && crate::bridge::st().prepared.numb_live_unique < 5 as ::core::ffi::c_int
                    {
                        borg_note(
                            b"# Not Resetting recall depth.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress('n' as i32 as keycode_t);
                    } else {
                        borg_note(
                            b"# Resetting recall depth.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        borg_keypress('y' as i32 as keycode_t);
                    }
                } else {
                    borg_note(
                        b"# Not resetting recall depth.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    borg_keypress('n' as i32 as keycode_t);
                }
            }
            borg_keypress(ESCAPE as keycode_t);
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_surrounded() -> bool {
    let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
    let mut r_ptr: *mut monster_race = ::core::ptr::null_mut::<monster_race>();
    let mut safe_grids: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut non_safe_grids: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut adjacent_monsters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x9: ::core::ffi::c_int = 0;
    let mut y9: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        r_ptr = r_info.offset((*kill).r_idx as isize) as *mut monster_race;
        if !((*kill).r_idx == 0) {
            if !((*kill).r_idx as ::core::ffi::c_int
                >= (*z_info).r_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            {
                x9 = (*kill).pos.x;
                y9 = (*kill).pos.y;
                ax = if x9 > crate::bridge::st().borg.c.x {
                    x9 - crate::bridge::st().borg.c.x
                } else {
                    crate::bridge::st().borg.c.x - x9
                };
                ay = if y9 > crate::bridge::st().borg.c.y {
                    y9 - crate::bridge::st().borg.c.y
                } else {
                    crate::bridge::st().borg.c.y - y9
                };
                d = if ax < ay { ay } else { ax };
                if !(d > 3 as ::core::ffi::c_int) {
                    if borg_los(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x, y9, x9) {
                        if (*kill).awake {
                            if !flag_has(
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
                            ) {
                                if !flag_has(
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
                                ) {
                                    if d == 1 as ::core::ffi::c_int {
                                        adjacent_monsters += 1;
                                    }
                                    monsters += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int;
        let mut ag: *mut borg_grid = crate::bridge::grid(x, y) as *mut borg_grid;
        if square_in_bounds_fully(cave, loc(x, y)) {
            if !crate::borg_cave_util::borg_cave_floor_grid(&*(ag)) {
                non_safe_grids += 1;
            } else if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int {
                non_safe_grids += 1;
            } else if (*ag).kill != 0 {
                non_safe_grids += 1;
            } else if feat_is_shop((*ag).feat as ::core::ffi::c_int) {
                non_safe_grids += 1;
            }
            if (*ag).trap as ::core::ffi::c_int != 0 && !(*ag).glyph {
                non_safe_grids += 1;
            }
        }
        i += 1;
    }
    safe_grids = safe_grids - non_safe_grids;
    if safe_grids == 1 as ::core::ffi::c_int && adjacent_monsters == 1 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if monsters > safe_grids {
        borg_note(format(
            b"# Possibility of being surrounded (monsters/safegrids)(%d/%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            monsters,
            safe_grids,
        ));
        if crate::bridge::st().borg.goal.ignoring {
        } else {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_freedom(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
        && track_more.num as ::core::ffi::c_int != 0
    {
        d = borg_distance(
            y << 1 as ::core::ffi::c_int,
            x << 1 as ::core::ffi::c_int,
            *track_more.y.offset(0 as ::core::ffi::c_int as isize) << 1 as ::core::ffi::c_int,
            *track_more.x.offset(0 as ::core::ffi::c_int as isize) << 1 as ::core::ffi::c_int,
        );
        f += 1000 as ::core::ffi::c_int - d;
        if d < 4 as ::core::ffi::c_int {
            f += 2000 as ::core::ffi::c_int - d * 500 as ::core::ffi::c_int;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        != 0
        && track_less.num as ::core::ffi::c_int != 0
    {
        d = borg_distance(
            y << 1 as ::core::ffi::c_int,
            x << 1 as ::core::ffi::c_int,
            *track_less.y.offset(0 as ::core::ffi::c_int as isize) << 1 as ::core::ffi::c_int,
            *track_less.x.offset(0 as ::core::ffi::c_int as isize) << 1 as ::core::ffi::c_int,
        );
        f += 1000 as ::core::ffi::c_int - d;
        if d < 4 as ::core::ffi::c_int {
            f += 2000 as ::core::ffi::c_int - d * 500 as ::core::ffi::c_int;
        }
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn borg_caution_phase(
    mut emergency: ::core::ffi::c_int,
    mut turns: ::core::ffi::c_int,
) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut dis: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = dis / 2 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    if crate::bridge::st().borg.trait_[(BI_APHASE as ::core::ffi::c_int) as usize]
        == 0
    {
        return false_0 != 0;
    }
    k = 0 as ::core::ffi::c_int;
    n = k;
    while k < 100 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 100 as ::core::ffi::c_int {
            loop {
                y = (crate::bridge::st().borg.c.y as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                x = (crate::bridge::st().borg.c.x as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                d = distance(crate::bridge::st().borg.c, loc(x, y));
                if d >= min && d <= dis {
                    break;
                }
            }
            if !(y <= 0 as ::core::ffi::c_int || y >= AUTO_MAX_Y - 1 as ::core::ffi::c_int) {
                if !(x <= 0 as ::core::ffi::c_int || x >= AUTO_MAX_X - 1 as ::core::ffi::c_int) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int) {
                        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y, x) {
                            if !((*ag).kill != 0) {
                                if !(*ag).web {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                < 30 as ::core::ffi::c_int
        {
            n += 1;
        } else if i >= 100 as ::core::ffi::c_int {
            n += 1;
        } else {
            p = borg_danger(y, x, turns, true_0 != 0, false_0 != 0);
            if p > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] {
                n += 1;
            }
        }
        k += 1;
    }
    if n > emergency {
        borg_note(format(
            b"# No Phase. scary squares: %d\0" as *const u8 as *const ::core::ffi::c_char,
            n,
        ));
        return false_0 != 0;
    } else {
        borg_note(format(
            b"# Safe to Phase. scary squares: %d\0" as *const u8 as *const ::core::ffi::c_char,
            n,
        ));
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_caution_teleport(
    mut emergency: ::core::ffi::c_int,
    mut turns: ::core::ffi::c_int,
) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut dis: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = dis / 2 as ::core::ffi::c_int;
    let mut q_x: ::core::ffi::c_int = 0;
    let mut q_y: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    q_x = crate::bridge::st().core.w_x / crate::borg_cave_util::borg_panel_wid();
    q_y = crate::bridge::st().core.w_y / crate::borg_cave_util::borg_panel_hgt();
    if crate::bridge::st().borg.trait_[(BI_ATELEPORT as ::core::ffi::c_int) as usize]
        == 0
        || crate::bridge::st().borg.trait_[(BI_AESCAPE as ::core::ffi::c_int) as usize]
            == 0
    {
        return false_0 != 0;
    }
    k = 0 as ::core::ffi::c_int;
    n = k;
    while k < 100 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 100 as ::core::ffi::c_int {
            loop {
                y = (crate::bridge::st().borg.c.y as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                x = (crate::bridge::st().borg.c.x as int32_t
                    + Rand_div((1 as ::core::ffi::c_int + dis + dis) as uint32_t) as int32_t
                    - dis as int32_t) as ::core::ffi::c_int;
                d = distance(crate::bridge::st().borg.c, loc(x, y));
                if d >= min && d <= dis {
                    break;
                }
            }
            if !(y <= 0 as ::core::ffi::c_int || y >= AUTO_MAX_Y - 1 as ::core::ffi::c_int) {
                if !(x <= 0 as ::core::ffi::c_int || x >= AUTO_MAX_X - 1 as ::core::ffi::c_int) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                        && (*(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                            .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == true_0
                            && *(*borg_detect_wall.offset((q_y + 0 as ::core::ffi::c_int) as isize))
                                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == true_0
                            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                                .offset((q_x + 0 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == true_0
                            && *(*borg_detect_wall.offset((q_y + 1 as ::core::ffi::c_int) as isize))
                                .offset((q_x + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == true_0
                            || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time))
                    {
                        if crate::borg_cave_util::borg_cave_floor_bold(crate::bridge::st(), y, x) {
                            if !((*ag).kill != 0) {
                                if !(*ag).web {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                < 30 as ::core::ffi::c_int
        {
            n += 1;
        } else if i >= 100 as ::core::ffi::c_int {
            n += 1;
        } else {
            p = borg_danger(y, x, turns, true_0 != 0, false_0 != 0);
            if p > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] {
                n += 1;
            }
        }
        k += 1;
    }
    if n > emergency {
        borg_note(format(
            b"# No Teleport. scary squares: %d\0" as *const u8 as *const ::core::ffi::c_char,
            n,
        ));
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn borg_escape_stair() -> bool {
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    if (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
    {
        borg_note(b"# Escaping level via stairs.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('<' as i32 as keycode_t);
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_allow_teleport() -> bool {
    if (*(*player).upkeep).arena_level {
        return false_0 != 0;
    }
    if square_isno_teleport(cave, crate::bridge::st().borg.c) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CRSNOTEL as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_shadow_shift(mut allow_fail: ::core::ffi::c_int) -> bool {
    if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] < 12 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return borg_spell_fail(SHADOW_SHIFT, allow_fail);
}
#[no_mangle]
pub unsafe extern "C" fn borg_dimension_door(mut allow_fail: ::core::ffi::c_int) -> bool {
    let mut x_off: ::core::ffi::c_int = 0;
    let mut y_off: ::core::ffi::c_int = 0;
    let mut t_x: ::core::ffi::c_int = 0;
    let mut t_y: ::core::ffi::c_int = 0;
    let mut best: loc = loc { x: 0, y: 0 };
    let mut d: ::core::ffi::c_int = 0;
    let mut best_d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut target: loc = loc { x: 0, y: 0 };
    let mut range: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    if !borg_spell_okay_fail(DIMENSION_DOOR, allow_fail) {
        return 0 as ::core::ffi::c_int != 0;
    }
    best_d = borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
        [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    x_off = range * -(1 as ::core::ffi::c_int);
    while x_off < range {
        y_off = range * -(1 as ::core::ffi::c_int);
        while y_off < range {
            t_x = crate::bridge::st().borg.c.x + x_off;
            t_y = crate::bridge::st().borg.c.y + y_off;
            if !(t_x < 0 as ::core::ffi::c_int || t_y < 0 as ::core::ffi::c_int) {
                target = loc(crate::bridge::st().borg.c.x + x_off, crate::bridge::st().borg.c.y + y_off);
                if square_in_bounds_fully(cave, target) {
                    d = borg_danger(t_y, t_x, 2 as ::core::ffi::c_int, true_0 != 0, false_0 != 0);
                    if d < best_d {
                        best_d = d;
                        best.x = t_x;
                        best.y = t_y;
                    }
                }
            }
            y_off += 1;
        }
        x_off += 1;
    }
    if best_d
        < borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
            [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
    {
        borg_target(best, false_0 != 0);
        borg_spell(DIMENSION_DOOR);
        borg_keypress('5' as i32 as keycode_t);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_teleport_off_level() -> bool {
    if !(crate::bridge::st().borg.goal.recalling != 0 || crate::bridge::st().borg.goal.descending != 0) {
        if borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport_level) as ::core::ffi::c_int != 0
            || borg_activate_item(crate::bridge::st().act.tele_level) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        if borg_activate_item(crate::bridge::st().act.deep_descent) as ::core::ffi::c_int != 0
            || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_deep_descent) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_escape(mut b_q: ::core::ffi::c_int) -> bool {
    let mut risky_boost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut glyphs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    let mut allow_fail: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let mut sv_mana: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize] * 100 as ::core::ffi::c_int
        / crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
        > 70 as ::core::ffi::c_int
    {
        allow_fail = 10 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
        != 0
    {
        allow_fail = 35 as ::core::ffi::c_int;
    }
    sv_mana = crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize];
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
        && (crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0
            || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
                != 0
            || crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0)
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
            >= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                * 5 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
    {
        if crate::bridge::st().borg.morgoth_position {
            return false_0 != 0;
        }
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            let mut y: ::core::ffi::c_int = crate::bridge::st().borg.c.y + ddy_ddd[j as usize] as ::core::ffi::c_int;
            let mut x: ::core::ffi::c_int = crate::bridge::st().borg.c.x + ddx_ddd[j as usize] as ::core::ffi::c_int;
            ag = crate::bridge::grid(x, y) as *mut borg_grid;
            if (*ag).glyph {
                glyphs += 1;
            }
            j += 1;
        }
        if glyphs >= 3 as ::core::ffi::c_int {
            return false_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 1 as ::core::ffi::c_int
    {
        if borg_teleport_off_level() {
            borg_note(
                b"# Attempting to leave via teleport level/deep descent\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
    }
    if crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] != 0 {
        risky_boost = 3 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
        != 0
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (45 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (40 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 100 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < 600 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (30 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                == 99 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < 600 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (25 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                >= 95 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < 550 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (17 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < 95 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (15 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique == 0
    {
        let mut tmp_allow_fail: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
        if borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_allow_teleport() as ::core::ffi::c_int != 0
                && (borg_dimension_door(tmp_allow_fail - 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    != 0
                    || borg_spell_fail(TELEPORT_SELF, tmp_allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_spell_fail(PORTAL, tmp_allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_shadow_shift(tmp_allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                    || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0
                    || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                    || borg_teleport_off_level() as ::core::ffi::c_int != 0
                    || borg_dimension_door(tmp_allow_fail + 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_spell_fail(TELEPORT_SELF, tmp_allow_fail + 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_spell_fail(PORTAL, tmp_allow_fail + 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_shadow_shift(tmp_allow_fail + 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_use_staff(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0
                    || borg_spell_fail(TELEPORT_LEVEL, tmp_allow_fail + 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_caution_phase(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                        && (borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0
                            || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                            || borg_spell_fail(PHASE_DOOR, tmp_allow_fail) as ::core::ffi::c_int
                                != 0
                            || borg_spell_fail(PORTAL, tmp_allow_fail) as ::core::ffi::c_int != 0))
        {
            borg_note(b"# Danger Level 1.\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] =
            crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize];
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    * 1 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int
            && borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(90 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(TELEPORT_SELF) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Danger Level 1.1  Critical Attempt\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
            && ((crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    * 1 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int
                || b_q
                    > crate::bridge::st().borg.avoidance as ::core::ffi::c_int
                        * (45 as ::core::ffi::c_int + risky_boost)
                        / 10 as ::core::ffi::c_int)
                && (borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                    || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0))
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(
                b"# Danger Level 1.2  Critical Phase\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                < crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    * 1 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int
            && (borg_spell_fail(PHASE_DOOR, 15 as ::core::ffi::c_int) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0)
        {
            borg_note(
                b"# Danger Level 1.3  Critical Attempt\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] = sv_mana;
    }
    if b_q
        < crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (25 as ::core::ffi::c_int + risky_boost)
            / 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 3 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 97 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
        != 0
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (3 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
            && crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
                <= 20 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
                >= 45 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (13 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                != 99 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (11 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique == 0
    {
        if borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_allow_teleport() as ::core::ffi::c_int != 0
                && (borg_dimension_door(allow_fail - 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    != 0
                    || borg_spell_fail(TELEPORT_SELF, allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_spell_fail(PORTAL, allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_shadow_shift(allow_fail - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != 0
                    || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0
                    || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                    || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                    || borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                    || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                    || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                    || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                    || borg_use_staff(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 2.1\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_caution_phase(50 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell(PHASE_DOOR) as ::core::ffi::c_int != 0
                || borg_spell(PORTAL) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 2.2\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_ISHEAVYSTUN as ::core::ffi::c_int) as usize]
        != 0
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (13 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (10 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique == 0
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (10 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_ISAFRAID as ::core::ffi::c_int) as usize]
                != 0
            && (crate::bridge::st().borg.trait_[(BI_AMISSILES as ::core::ffi::c_int) as usize]
                <= 0 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_WARRIOR)
    {
        if (borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_caution_phase(25 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(b"# Danger Level 3.1\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 3.2\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_caution_phase(75 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            != 0
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(b"# Danger Level 3.3\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_teleport_off_level() {
            borg_note(b"# Danger Level 3.4\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.fleeing
            && (crate::bridge::st().borg.near.unique == 0
                || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 35 as ::core::ffi::c_int)
            && !crate::bridge::st().borg.status.vault
        {
            borg_note(
                b"# Fleeing (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.leaving
            && (crate::bridge::st().borg.near.unique == 0
                || crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    < 35 as ::core::ffi::c_int)
            && !crate::bridge::st().borg.status.vault
        {
            borg_note(
                b"# Leaving (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
    }
    if b_q
        > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (8 as ::core::ffi::c_int + risky_boost)
            / 10 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 35 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                <= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                    / 3 as ::core::ffi::c_int)
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (9 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int
            && (crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 35 as ::core::ffi::c_int
                || crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    <= crate::bridge::st().borg.trait_[(BI_MAXHP as ::core::ffi::c_int) as usize]
                        / 3 as ::core::ffi::c_int)
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (6 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 20 as ::core::ffi::c_int
            && crate::bridge::st().borg.near.unique == 0
        || b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (6 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 35 as ::core::ffi::c_int
    {
        if (borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_caution_phase(20 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(b"# Danger Level 4.1\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 4.2\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.fleeing
            && crate::bridge::st().borg.near.unique == 0
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 25 as ::core::ffi::c_int
            && !crate::bridge::st().borg.status.vault
        {
            borg_note(
                b"# Fleeing (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.leaving && crate::bridge::st().borg.near.unique == 0 && !crate::bridge::st().borg.status.vault {
            borg_note(
                b"# Leaving (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
        if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
            || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 35 as ::core::ffi::c_int
            && borg_caution_phase(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(b"# Danger Level 4.3\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 10 as ::core::ffi::c_int
        && (b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (5 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            || b_q
                > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (7 as ::core::ffi::c_int + risky_boost)
                    / 10 as ::core::ffi::c_int
                && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int)
    {
        if (borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_caution_phase(20 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 5.1\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_shadow_shift(allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 5.2\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.fleeing && crate::bridge::st().borg.near.unique == 0 {
            borg_note(
                b"# Fleeing (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
        }
        if !crate::bridge::st().borg.goal.leaving && crate::bridge::st().borg.near.unique == 0 {
            borg_note(
                b"# Leaving (failed to teleport)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
        if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
            || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                <= 8 as ::core::ffi::c_int
            && borg_caution_phase(65 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0)
        {
            crate::bridge::st().borg.escapes -= 1;
            borg_note(b"# Danger Level 5.3\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
    }
    if (crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_MAGE
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_PRIEST
        || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize] == CLASS_NECROMANCER)
        && (b_q
            > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (6 as ::core::ffi::c_int + risky_boost)
                / 10 as ::core::ffi::c_int
            || b_q
                > crate::bridge::st().borg.avoidance as ::core::ffi::c_int * (8 as ::core::ffi::c_int + risky_boost)
                    / 10 as ::core::ffi::c_int
                && crate::bridge::st().borg.near.unique as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                && crate::bridge::st().borg.near.unique as ::core::ffi::c_int <= 8 as ::core::ffi::c_int)
        && (crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
            <= crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                * 1 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                >= 100 as ::core::ffi::c_int)
    {
        if (borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_caution_phase(20 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 6.1\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 6.2\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
    }
    if (borg_spell_okay_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
        || borg_spell_okay_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0)
        && borg_shoot_scoot_safe(20 as ::core::ffi::c_int, 2 as ::core::ffi::c_int, b_q)
            as ::core::ffi::c_int
            != 0
    {
        if borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
            || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
        {
            borg_note(
                b"# Shoot N Scoot. (Danger Level 7.1)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.escapes -= 1;
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.times_twitch as ::core::ffi::c_int > 50 as ::core::ffi::c_int {
        if (borg_escape_stair() as ::core::ffi::c_int != 0
            || borg_caution_phase(20 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                != 0)
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon > 50 as borg_time
            && (borg_spell_fail(PHASE_DOOR, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_phase) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_phase_door) as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 8\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            crate::bridge::st().borg.times_twitch = 0 as int16_t;
            return true_0 != 0;
        }
        if borg_allow_teleport() as ::core::ffi::c_int != 0
            && (borg_dimension_door(allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(TELEPORT_SELF, allow_fail) as ::core::ffi::c_int != 0
                || borg_spell_fail(PORTAL, allow_fail) as ::core::ffi::c_int != 0
                || borg_activate_item(crate::bridge::st().act.tele_long) as ::core::ffi::c_int != 0
                || borg_read_scroll(crate::bridge::st().item_val.sv_scroll_teleport) as ::core::ffi::c_int != 0
                || borg_use_staff_fail(crate::bridge::st().item_val.sv_staff_teleportation) as ::core::ffi::c_int != 0
                || borg_teleport_off_level() as ::core::ffi::c_int != 0)
        {
            borg_note(b"# Danger Level 8\0" as *const u8 as *const ::core::ffi::c_char);
            if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.antisummon < 50 as borg_time {
                crate::bridge::st().borg.time.antisummon = 0 as ::core::ffi::c_int as borg_time;
            }
            crate::bridge::st().borg.times_twitch = 0 as int16_t;
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
