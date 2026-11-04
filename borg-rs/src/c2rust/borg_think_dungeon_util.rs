pub const GOAL_TAKE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GOAL_BORE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GOAL_FLEE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const AUTO_TEMP_MAX: ::core::ffi::c_int = 9000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BORG_MARK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BORG_GLOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BORG_DARK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BORG_IGNORE_MAP: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid};

use crate::headers::borg_danger_h::{borg_fear_monsters, borg_fear_region};
use crate::headers::borg_escape_h::borg_recall;
pub use crate::headers::borg_flow_h::{
    borg_flow_old, borg_temp_n, borg_temp_x, borg_temp_y, borg_track,
};
use crate::headers::borg_flow_misc_h::{borg_flow_light, track_vein};
use crate::headers::borg_flow_stairs_h::{
    borg_flow_stair_both, borg_flow_stair_less, borg_flow_stair_more, track_less, track_more,
};
use crate::headers::borg_flow_take_h::borg_flow_take_scum;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_activation_h::{act_illumination, act_light, act_stone_to_mud};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
pub use crate::headers::borg_item_use_h::{
    borg_activate_item, borg_activate_ring, borg_need, borg_read_scroll, borg_use_staff,
    borg_use_things, borg_zap_rod, BORG_MET_NEED, BORG_NO_NEED, BORG_UNMET_NEED,
};
use crate::headers::borg_item_val_h::{
    sv_ring_digging, sv_rod_illumination, sv_rod_recall, sv_scroll_light, sv_staff_light,
};
use crate::headers::borg_item_wear_h::borg_wear_stuff;
use crate::headers::borg_light_h::{borg_light_beam, borg_maintain_light};
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_legal, borg_spell_okay, borg_spell_okay_fail, borg_spells, ACID_SPRAY,
    ANNIHILATE, BANISHMENT, BANISH_EVIL, BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN,
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
use crate::headers::borg_messages_react_h::borg_react;
use crate::headers::borg_prepared_h::{borg_must_return_to_town, borg_prepared, borg_restock};
use crate::headers::borg_projection_h::{borg_los, borg_target};
use crate::headers::borg_store_sell_h::borg_count_sell;
use crate::headers::borg_think_dungeon_h::borg_feeling_stuff;
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
use crate::headers::borg_update_h::{borg_do_update_lite, borg_do_update_view};
pub use crate::headers::cave_h::{
    cave, chunk, connector, ddd, ddx, ddx_ddd, ddy, ddy_ddd, distance, heatmap, monster_group,
    square, square_in_bounds_fully, square_isglow, trap, C2RustUnnamedH_18, FEAT_MAX,
};
use crate::headers::game_world_h::turn;
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
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::strstr;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::{
    keycode_t, keypress, mouseclick, ui_event, ui_event_type, EVT_BUTTON, EVT_DISCONNECT,
    EVT_ESCAPE, EVT_KBRD, EVT_MOUSE, EVT_MOVE, EVT_NONE, EVT_RESIZE, EVT_SELECT, EVT_SWITCH,
};
pub use crate::headers::ui_term_h::{
    col_map, row_bottom_map, row_top_map, term, term_win, tile_height, tile_width, Term,
};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
static mut borg_stuff_feeling: [::core::ffi::c_int; 12] = [
    50000 as ::core::ffi::c_int,
    8000 as ::core::ffi::c_int,
    8000 as ::core::ffi::c_int,
    8000 as ::core::ffi::c_int,
    8000 as ::core::ffi::c_int,
    5000 as ::core::ffi::c_int,
    5000 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn borg_money_scum() -> bool {
    let mut divisor: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    if crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    borg_note(format(
        b"# Waiting for towns people to breed.  I need %d...\0" as *const u8
            as *const ::core::ffi::c_char,
        crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
            - crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize],
    ));
    crate::bridge::st().borg.in_shop = false_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 35 as ::core::ffi::c_int
    {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress('0' as i32 as keycode_t);
        borg_keypress('0' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
    } else if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        >= 15 as ::core::ffi::c_int
    {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('7' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
    } else {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('2' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
    }
    if turn as ::core::ffi::c_long
        % (10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long)
        < 10 as ::core::ffi::c_long * (*z_info).day_length as ::core::ffi::c_long
            / 2 as ::core::ffi::c_long
    {
        divisor = 5 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.time.now % divisor as borg_time != 0 {
        let mut dirs: [::core::ffi::c_int; 8] = [0; 8];
        let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut next: loc = loc(
                crate::bridge::st().borg.c.x + ddx_ddd[i as usize] as ::core::ffi::c_int,
                crate::bridge::st().borg.c.y + ddy_ddd[i as usize] as ::core::ffi::c_int,
            );
            if square_in_bounds_fully(cave, next) as ::core::ffi::c_int != 0
                && (*crate::bridge::grid(next.x, next.y)).feat as ::core::ffi::c_int
                    == FEAT_FLOOR as ::core::ffi::c_int
            {
                let fresh0 = count;
                count = count + 1;
                dirs[fresh0 as usize] = ddd[i as usize] as ::core::ffi::c_int;
            }
            i += 1;
        }
        if count != 0 {
            let mut dir: ::core::ffi::c_int = dirs[Rand_div(count as uint32_t) as int32_t as usize];
            crate::bridge::st().borg.goal.g = loc(
                crate::bridge::st().borg.c.x + ddx[dir as usize] as ::core::ffi::c_int,
                crate::bridge::st().borg.c.y + ddy[dir as usize] as ::core::ffi::c_int,
            );
            borg_keypress(ESCAPE as keycode_t);
            borg_keypress((dir + '0' as i32) as keycode_t);
        }
    }
    crate::bridge::st().borg.antibounce_count = 1 as int16_t;
    crate::bridge::st().borg.time.level = 1 as ::core::ffi::c_int as borg_time;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_dungeon_light() -> bool {
    let mut ii: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut not_safe: bool = false_0 != 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        && borg_use_things() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0
        && (crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
            <= 0 as ::core::ffi::c_int
            || (*borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
            ))
            .timeout as ::core::ffi::c_int
                <= 3 as ::core::ffi::c_int)
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 1 as ::core::ffi::c_int
    {
        let mut need: borg_need = BORG_NO_NEED;
        if crate::bridge::st().borg.goal.recalling != 0 {
            borg_keypress('R' as i32 as keycode_t);
            borg_keypress('9' as i32 as keycode_t);
            borg_keypress(KC_ENTER as keycode_t);
            return true_0 != 0;
        }
        if borg_wear_stuff() {
            return true_0 != 0;
        }
        need = borg_maintain_light();
        if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return true_0 != 0;
        }
        if need as ::core::ffi::c_uint == BORG_NO_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.goal.recalling == 0 && borg_zap_rod(sv_rod_recall) as ::core::ffi::c_int != 0 {
            return true_0 != 0;
        }
        if crate::bridge::st().borg.goal.recalling == 0 {
            if borg_recall() {
                return true_0 != 0;
            }
        }
        if !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            if (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat as ::core::ffi::c_int
                == FEAT_LESS as ::core::ffi::c_int
            {
                borg_note(b"# Testing for stairs .\0" as *const u8 as *const ::core::ffi::c_char);
                borg_keypress('<' as i32 as keycode_t);
            }
        }
        if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize]
            < crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
            && crate::bridge::st().borg.trait_[(BI_MAXSP as ::core::ffi::c_int) as usize]
                > 0 as ::core::ffi::c_int
            && ((*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).info
                as ::core::ffi::c_int
                & BORG_GLOW
                != 0
                || crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                    == CLASS_NECROMANCER)
            && crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
                == 0
            && (borg_spell_legal(HERBAL_CURING) as ::core::ffi::c_int != 0
                || borg_spell_legal(REMOVE_HUNGER) as ::core::ffi::c_int != 0
                || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                    >= crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize])
            && (borg_spell_legal(CALL_LIGHT) as ::core::ffi::c_int != 0
                || borg_spell_legal(LIGHT_ROOM) as ::core::ffi::c_int != 0)
            && (!borg_spell_okay(CALL_LIGHT) && !borg_spell_okay(LIGHT_ROOM))
        {
            ii = 0 as ::core::ffi::c_int;
            while ii < 8 as ::core::ffi::c_int {
                x = crate::bridge::st().borg.c.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                y = crate::bridge::st().borg.c.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                if square_in_bounds_fully(cave, loc(x, y)) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if (*ag).kill != 0 {
                        not_safe = true_0 != 0;
                    }
                }
                ii += 1;
            }
            if borg_fear_region[(crate::bridge::st().borg.c.y / 11 as ::core::ffi::c_int) as usize]
                [(crate::bridge::st().borg.c.x / 11 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    / 10 as ::core::ffi::c_int
            {
                not_safe = true_0 != 0;
            }
            if borg_fear_monsters[crate::bridge::st().borg.c.y as usize][crate::bridge::st().borg.c.x as usize] as ::core::ffi::c_int
                > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
                    / 10 as ::core::ffi::c_int
            {
                not_safe = true_0 != 0;
            }
            if !not_safe {
                borg_note(
                    b"# Resting on this Glowing Grid to gain mana.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg_keypress('R' as i32 as keycode_t);
                borg_keypress('*' as i32 as keycode_t);
                borg_keypress(KC_ENTER as keycode_t);
                return true_0 != 0;
            }
        }
        if crate::bridge::st().borg.time.call_light == 0 as borg_time
            || crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.call_light > 7 as borg_time
        {
            ii = 0 as ::core::ffi::c_int;
            while ii < 8 as ::core::ffi::c_int {
                x = crate::bridge::st().borg.c.x + ddx_ddd[ii as usize] as ::core::ffi::c_int;
                y = crate::bridge::st().borg.c.y + ddy_ddd[ii as usize] as ::core::ffi::c_int;
                if square_in_bounds_fully(cave, loc(x, y)) {
                    ag = crate::bridge::grid(x, y) as *mut borg_grid;
                    if !((*ag).feat as ::core::ffi::c_int >= FEAT_RUBBLE as ::core::ffi::c_int
                        && (*ag).feat as ::core::ffi::c_int <= FEAT_PERM as ::core::ffi::c_int)
                    {
                        if ((*ag).feat as ::core::ffi::c_int == FEAT_OPEN as ::core::ffi::c_int
                            || (*ag).feat as ::core::ffi::c_int
                                == FEAT_BROKEN as ::core::ffi::c_int)
                            && (y == crate::bridge::st().borg.c.y && x == crate::bridge::st().borg.c.x)
                        {
                            if square_isglow(cave, crate::bridge::st().borg.c) {
                                (*ag).info =
                                    ((*ag).info as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                            }
                        } else if crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_NECROMANCER
                        {
                            if (*ag).info as ::core::ffi::c_int & BORG_DARK != 0
                                || (*ag).feat as ::core::ffi::c_int
                                    == FEAT_NONE as ::core::ffi::c_int
                                || (*ag).info as ::core::ffi::c_int & BORG_MARK == 0
                                || (*ag).info as ::core::ffi::c_int & BORG_GLOW == 0
                            {
                                if borg_activate_item(act_illumination) as ::core::ffi::c_int != 0
                                    || borg_activate_item(act_light) as ::core::ffi::c_int != 0
                                    || borg_zap_rod(sv_rod_illumination) as ::core::ffi::c_int != 0
                                    || borg_use_staff(sv_staff_light) as ::core::ffi::c_int != 0
                                    || borg_read_scroll(sv_scroll_light) as ::core::ffi::c_int != 0
                                    || borg_spell(CALL_LIGHT) as ::core::ffi::c_int != 0
                                    || borg_spell(LIGHT_ROOM) as ::core::ffi::c_int != 0
                                {
                                    borg_note(
                                        b"# Illuminating the region while dark.\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    borg_react(
                                        b"SELF:lite\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"SELF:lite\0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    crate::bridge::st().borg.time.call_light = crate::bridge::st().borg.time.now;
                                    return true_0 != 0;
                                }
                                if borg_light_beam(false_0 != 0) {
                                    return true_0 != 0;
                                }
                            }
                        } else if (*ag).info as ::core::ffi::c_int & BORG_DARK == 0
                            || (*ag).feat as ::core::ffi::c_int == FEAT_NONE as ::core::ffi::c_int
                            || (*ag).info as ::core::ffi::c_int & BORG_MARK == 0
                            || (*ag).info as ::core::ffi::c_int & BORG_GLOW != 0
                        {
                            if borg_spell(CREATE_DARKNESS) {
                                borg_note(
                                    b"# Darkening the region that is lit.\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                crate::bridge::st().borg.time.call_light = crate::bridge::st().borg.time.now;
                                return true_0 != 0;
                            }
                        }
                    }
                }
                ii += 1;
            }
        }
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) as ::core::ffi::c_int != 0
            && !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize]
        {
            if (*crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y)).feat as ::core::ffi::c_int
                == FEAT_LESS as ::core::ffi::c_int
            {
                borg_note(
                    b"# Taking up Stairs (low Light).\0" as *const u8 as *const ::core::ffi::c_char,
                );
                borg_keypress('<' as i32 as keycode_t);
                return true_0 != 0;
            }
            borg_note(
                b"# Flowing to stairs (low Light).\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
            != 0
            && borg_flow_light(GOAL_FLEE) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_stair_scum() -> bool {
    let mut j: ::core::ffi::c_int = 0;
    let mut b_j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
    let mut feat: uint8_t = (*square(cave, crate::bridge::st().borg.c)).feat;
    let mut need: borg_need = BORG_NO_NEED;
    borg_notice(true_0 != 0);
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        borg_note(
            b"# Leaving Scumming Mode. (Town or Weak)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.lunal_mode = false_0 != 0;
        return false_0 != 0;
    }
    if (*borg_items.offset(
        ((*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
            - 1 as ::core::ffi::c_int) as isize,
    ))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    if feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_MORE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            if *track_more.x.offset(i as isize) == crate::bridge::st().borg.c.x
                && *track_more.y.offset(i as isize) == crate::bridge::st().borg.c.y
            {
                break;
            }
            i += 1;
        }
        if i == track_more.num as ::core::ffi::c_int && i < track_more.size as ::core::ffi::c_int {
            *track_more.x.offset(i as isize) = crate::bridge::st().borg.c.x;
            *track_more.y.offset(i as isize) = crate::bridge::st().borg.c.y;
            track_more.num += 1;
        }
        (*ag).feat = FEAT_MORE as ::core::ffi::c_int as uint8_t;
    }
    if feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        && (*ag).feat as ::core::ffi::c_int != FEAT_LESS as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) == crate::bridge::st().borg.c.x
                && *track_less.y.offset(i as isize) == crate::bridge::st().borg.c.y;
            i += 1;
        }
        if i == track_less.num as ::core::ffi::c_int && i < track_less.size as ::core::ffi::c_int {
            *track_less.x.offset(i as isize) = crate::bridge::st().borg.c.x;
            *track_less.y.offset(i as isize) = crate::bridge::st().borg.c.y;
            track_less.num += 1;
        }
        (*ag).feat = FEAT_LESS as ::core::ffi::c_int as uint8_t;
    }
    need = borg_maintain_light();
    if need as ::core::ffi::c_uint == BORG_MET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint {
        return true_0 != 0;
    } else if need as ::core::ffi::c_uint
        == BORG_UNMET_NEED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borg_note(b"# Scum. (need fuel)\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if borg_flow_old(GOAL_TAKE) {
        return true_0 != 0;
    }
    if borg_flow_take_scum(true_0 != 0, 6 as ::core::ffi::c_int) {
        return true_0 != 0;
    }
    borg_note(b"# Fleeing level. Scumming Mode\0" as *const u8 as *const ::core::ffi::c_char);
    crate::bridge::st().borg.goal.fleeing = true_0 != 0;
    if track_more.num as ::core::ffi::c_int != 0
        && ((*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < 30 as ::core::ffi::c_int)
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        if track_more.num as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            borg_note(
                b"# Scumming Mode: I know of a down stair.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < track_more.num as ::core::ffi::c_int {
            x = *track_more.x.offset(i as isize);
            y = *track_more.y.offset(i as isize);
            j = distance(crate::bridge::st().borg.c, loc(x, y));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                < 30 as ::core::ffi::c_int
        {
            borg_note(
                b"# Scumming Mode.  Power Diving. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_more(GOAL_FLEE, false_0 != 0, false_0 != 0) {
                return true_0 != 0;
            }
            if (*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int {
                borg_keypress('>' as i32 as keycode_t);
                return true_0 != 0;
            }
        }
    }
    if track_less.num as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 1 as ::core::ffi::c_int
        && ((*ag).feat as ::core::ffi::c_int == FEAT_MORE as ::core::ffi::c_int
            || (*ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int)
    {
        let mut y_0: ::core::ffi::c_int = 0;
        let mut x_0: ::core::ffi::c_int = 0;
        let mut tmp_ag: *mut borg_grid = crate::bridge::grid(crate::bridge::st().borg.c.x, crate::bridge::st().borg.c.y) as *mut borg_grid;
        i = 0 as ::core::ffi::c_int;
        while i < track_less.num as ::core::ffi::c_int {
            x_0 = *track_less.x.offset(i as isize);
            y_0 = *track_less.y.offset(i as isize);
            j = distance(crate::bridge::st().borg.c, loc(x_0, y_0));
            if !(b_j <= j && b_j != -(1 as ::core::ffi::c_int)) {
                b_j = j;
            }
            i += 1;
        }
        if b_j < 8 as ::core::ffi::c_int
            || (*tmp_ag).feat as ::core::ffi::c_int == FEAT_LESS as ::core::ffi::c_int
        {
            borg_note(
                b"# Scumming Mode.  Power Climb. \0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.less = true_0 != 0;
            if borg_flow_old(GOAL_FLEE) {
                return true_0 != 0;
            }
            if borg_flow_stair_less(GOAL_FLEE, false_0 != 0) {
                borg_note(
                    b"# Looking for stairs. Scumming Mode.\0" as *const u8
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
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        >= 2 as ::core::ffi::c_int
    {
        if borg_flow_old(GOAL_FLEE) {
            return true_0 != 0;
        }
        borg_note(b"# Scumming Mode.  Any Stair. \0" as *const u8 as *const ::core::ffi::c_char);
        if borg_flow_stair_both(GOAL_FLEE, true_0 != 0) {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_time_to_stay_on_level(mut bored: bool) -> ::core::ffi::c_int {
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 5 as ::core::ffi::c_int
        || borg_feeling_stuff == 0
    {
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 10 as ::core::ffi::c_int
        {
            return crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                * 50 as ::core::ffi::c_int;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                < 3 as ::core::ffi::c_int
        {
            return if crate::bridge::st().borg.trait_[(BI_REG as ::core::ffi::c_int) as usize] != 0 {
                2000 as ::core::ffi::c_int
            } else {
                2500 as ::core::ffi::c_int
            };
        }
        if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 20 as ::core::ffi::c_int
            && borg_feeling_stuff == 0 as ::core::ffi::c_int
        {
            return (*z_info).feeling_need as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
        }
    }
    if bored {
        return borg_stuff_feeling[borg_feeling_stuff as usize] / 10 as ::core::ffi::c_int;
    }
    return borg_stuff_feeling[borg_feeling_stuff as usize];
}
#[no_mangle]
pub unsafe extern "C" fn borg_leave_level(mut bored: bool) -> bool {
    let mut sellable_item_count: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut try_not_to_descend: bool = false_0 != 0;
    let mut need_restock: bool = false_0 != 0;
    if crate::bridge::st().borg.goal.recalling != 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            != 1 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 100 as ::core::ffi::c_int
        && crate::bridge::st().borg.near.morgoth as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.morgoth < 5000 as borg_time
    {
        crate::bridge::st().borg.goal.leaving = false_0 != 0;
        crate::bridge::st().borg.goal.rising = false_0 != 0;
        bored = false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 0
    {
        crate::bridge::st().borg.goal.rising = false_0 != 0;
        if !bored {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            == 100 as ::core::ffi::c_int
            && crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] == 0
        {
            if !borg_restock(100 as ::core::ffi::c_int).is_null() {
                crate::bridge::st().borg.goal.fleeing = true_0 != 0;
                crate::bridge::st().borg.goal.leaving = true_0 != 0;
                crate::bridge::st().borg.stair_more = true_0 != 0;
                borg_note(
                    b"# Borg must crawl to deep dungeon- no recall to 100.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if borg_flow_stair_more(GOAL_BORE, false_0 != 0, false_0 != 0) {
                    return true_0 != 0;
                }
                return false_0 != 0;
            }
        }
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            >= (if crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize] != 0 {
                10 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            })
            && crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
                >= 3 as ::core::ffi::c_int
            && ((borg_prepared(
                crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                    * 6 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char)
                .is_null()
                || crate::bridge::st().core.cfg[(BORG_PLAYS_RISKY as ::core::ffi::c_int) as usize] != 0)
            && borg_recall() as ::core::ffi::c_int != 0
        {
            borg_note(b"# Recalling into dungeon.\0" as *const u8 as *const ::core::ffi::c_char);
            return true_0 != 0;
        } else {
            if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                < (if crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize] != 0 {
                    10 as ::core::ffi::c_int
                } else {
                    8 as ::core::ffi::c_int
                })
            {
                borg_note(
                    b"# Not deep enough to recall\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
                <= 2 as ::core::ffi::c_int
            {
                borg_note(
                    b"# Not enough recalls to recall\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if !(borg_prepared(
                crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                    * 6 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char)
                .is_null()
            {
                borg_note(format(
                    b"# Way too scary to recall down there!   %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    borg_prepared(
                        crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize],
                    ),
                ));
            } else {
                borg_note(
                    b"# failed to recall when I wanted to\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            crate::bridge::st().borg.goal.fleeing = true_0 != 0;
            crate::bridge::st().borg.goal.leaving = true_0 != 0;
        }
        crate::bridge::st().borg.stair_more = true_0 != 0;
        if borg_flow_stair_more(GOAL_BORE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    let mut prep_cur_depth: *const ::core::ffi::c_char = borg_prepared(
        crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize],
    );
    let mut prep_next_depth: *const ::core::ffi::c_char = borg_prepared(
        crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            + 1 as ::core::ffi::c_int,
    );
    if !prep_cur_depth.is_null() {
        g = -(1 as ::core::ffi::c_int);
        borg_note(format(
            b"# heading up, not prep for current level: %s)\0" as *const u8
                as *const ::core::ffi::c_char,
            prep_cur_depth,
        ));
    }
    sellable_item_count = borg_count_sell();
    if sellable_item_count >= 12 as ::core::ffi::c_int {
        try_not_to_descend = true_0 != 0;
    }
    if g != 0
        && crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize]
            != 0
    {
        try_not_to_descend = true_0 != 0;
    }
    if bored as ::core::ffi::c_int != 0 && !prep_next_depth.is_null() {
        g = -(1 as ::core::ffi::c_int);
        borg_note(format(
            b"# heading up (bored and unable to dive: %s)\0" as *const u8
                as *const ::core::ffi::c_char,
            prep_next_depth,
        ));
    } else if bored as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.avoidance as ::core::ffi::c_int
            > crate::bridge::st().borg.trait_[(BI_CURHP as ::core::ffi::c_int) as usize]
    {
        if !prep_next_depth.is_null() {
            g = -(1 as ::core::ffi::c_int);
            borg_note(
                b"# heading up (bored and spastic).\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            g = 1 as ::core::ffi::c_int;
            borg_note(
                b"# heading down (bored and spastic).\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if !try_not_to_descend
        && borg_prepared(
            crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                + 5 as ::core::ffi::c_int,
        )
        .is_null()
        && sellable_item_count < 13 as ::core::ffi::c_int
    {
        g = 1 as ::core::ffi::c_int;
        borg_note(
            b"# power dive, playing too shallow.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if !try_not_to_descend
        && prep_next_depth.is_null()
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            >= 75 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            < 100 as ::core::ffi::c_int
    {
        g = 1 as ::core::ffi::c_int;
        borg_note(b"# power dive, head deep.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !prep_cur_depth.is_null() {
        if crate::bridge::st().borg.mon.unique == 0 {
            if g == 0
                && !borg_prepared(
                    crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                        * 5 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int,
                )
                .is_null()
                && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                    > 65 as ::core::ffi::c_int
            {
                borg_note(format(
                    b"# Returning to town (too deep: %s)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    prep_cur_depth,
                ));
                crate::bridge::st().borg.goal.rising = true_0 != 0;
            } else {
                borg_note(format(
                    b"# Climbing (too deep: %s)\0" as *const u8 as *const ::core::ffi::c_char,
                    prep_cur_depth,
                ));
                g = -(1 as ::core::ffi::c_int);
            }
        }
        if !borg_must_return_to_town().is_null() {
            borg_note(format(
                b"# returning to town to restock(too deep: %s)\0" as *const u8
                    as *const ::core::ffi::c_char,
                borg_must_return_to_town(),
            ));
            crate::bridge::st().borg.goal.rising = true_0 != 0;
            need_restock = true_0 != 0;
        }
        if !strstr(
            prep_cur_depth,
            b"Collect from house\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            borg_note(
                b"# Returning to town to Collect stock.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.rising = true_0 != 0;
            need_restock = true_0 != 0;
        }
    }
    if borg_prepared(
        crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            + 20 as ::core::ffi::c_int,
    )
    .is_null()
        && borg_prepared(
            crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                * 6 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int,
        )
        .is_null()
        && crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            > crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                + 20 as ::core::ffi::c_int
        && (crate::bridge::st().borg.trait_[(BI_RECALL as ::core::ffi::c_int) as usize]
            >= 3 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                > 2000 as ::core::ffi::c_int)
    {
        borg_note(
            b"# returning to town to recall back down (too shallow)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if (crate::bridge::st().core.cfg[(BORG_WORSHIPS_GOLD as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int)
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            <= 25 as ::core::ffi::c_int
        && sellable_item_count >= 12 as ::core::ffi::c_int
    {
        borg_note(
            b"# Going to town (Sell Stuff, Worshipping Gold).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if bored as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            >= 26 as ::core::ffi::c_int
        && sellable_item_count >= 12 as ::core::ffi::c_int
    {
        borg_note(b"# Going to town (Sell Stuff).\0" as *const u8 as *const ::core::ffi::c_char);
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISFIXLEV as ::core::ffi::c_int) as usize]
        != 0
    {
        borg_note(b"# Going to town (Fix Level).\0" as *const u8 as *const ::core::ffi::c_char);
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if bored as ::core::ffi::c_int != 0
        && crate::bridge::st().borg.trait_[(BI_ISFIXEXP as ::core::ffi::c_int) as usize]
            != 0
        && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            != 50 as ::core::ffi::c_int
    {
        borg_note(
            b"# Going to town (Fix Experience).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if !crate::bridge::st().borg.goal.rising
        && bored as ::core::ffi::c_int != 0
        && !crate::bridge::st().borg.status.vault
        && crate::bridge::st().borg.near.unique == 0
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town > 8000 as borg_time
    {
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 99 as ::core::ffi::c_int
            || crate::bridge::st().borg.mon.unique == 0
        {
            borg_note(
                b"# Going to town (I miss my home).\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.goal.rising = true_0 != 0;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
        >= crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            + 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            <= 12 as ::core::ffi::c_int
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town > 3500 as borg_time
    {
        borg_note(
            b"# Going to town (scumming check).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if !crate::bridge::st().borg.status.vault
        && crate::bridge::st().borg.trait_[(BI_PREP_BIG_FIGHT as ::core::ffi::c_int) as usize]
            != 0
        && (crate::bridge::st().borg.trait_[(BI_AEZHEAL as ::core::ffi::c_int) as usize]
            >= 3 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_ALIFE as ::core::ffi::c_int) as usize]
                >= 1 as ::core::ffi::c_int)
    {
        borg_note(
            b"# Going to town (Dropping off Potions).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        crate::bridge::st().borg.goal.rising = true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        == 99 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_SAURON_DEAD as ::core::ffi::c_int) as usize]
            != 0
        && crate::bridge::st().borg.ready_morgoth != 1 as ::core::ffi::c_int
    {
        borg_note(
            b"# Returning to level 98 to scum for items.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        g = -(1 as ::core::ffi::c_int);
    }
    if crate::bridge::st().borg.trait_[(BI_KING as ::core::ffi::c_int) as usize] != 0 {
        g = 1 as ::core::ffi::c_int;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        < 100 as ::core::ffi::c_int
        && borg_prepared(99 as ::core::ffi::c_int).is_null()
    {
        g = 1 as ::core::ffi::c_int;
    }
    if g == 0
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            > crate::bridge::st().core.cfg[(BORG_NO_DEEPER as ::core::ffi::c_int) as usize]
    {
        borg_note(format(
            b"# Going up a bit (No Deeper than %d).\0" as *const u8 as *const ::core::ffi::c_char,
            crate::bridge::st().core.cfg[(BORG_NO_DEEPER as ::core::ffi::c_int) as usize],
        ));
        g = -(1 as ::core::ffi::c_int);
    }
    if g == 0 && crate::bridge::st().borg.goal.rising as ::core::ffi::c_int != 0 {
        g = -(1 as ::core::ffi::c_int);
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        > 10 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level < 200 as borg_time
        && g < 0 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] > 1 as ::core::ffi::c_int
    {
        borg_note(
            b"# Staying on level 1 to rotate shops.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g = 0 as ::core::ffi::c_int;
    }
    if g == 0 && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > borg_time_to_stay_on_level(bored) as borg_time {
        if crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
            < 99 as ::core::ffi::c_int
            || crate::bridge::st().borg.mon.unique == 0
        {
            borg_note(format(
                b"# Spent too long (%ld) on level, leaving.\0" as *const u8
                    as *const ::core::ffi::c_char,
                (crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level) as ::core::ffi::c_long,
            ));
            if try_not_to_descend {
                g = -(1 as ::core::ffi::c_int);
            } else {
                g = if (Rand_div(100 as uint32_t) as int32_t) < 50 as int32_t {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                };
            }
        }
    }
    if g < 0 as ::core::ffi::c_int {
        if !(*player).opts.opt[OPT_birth_force_descend as ::core::ffi::c_int as usize] {
            borg_note(
                b"# Looking for up stairs.  Going up.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::bridge::st().borg.stair_less = true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            < 100 as ::core::ffi::c_int
            || borg_prepared(99 as ::core::ffi::c_int).is_null()
        {
            if crate::bridge::st().borg.goal.rising as ::core::ffi::c_int != 0
                && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.town > 200 as borg_time
                && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                    >= 5 as ::core::ffi::c_int
            {
                if borg_recall() {
                    borg_note(
                        b"# Recalling to town (goal rising)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return true_0 != 0;
                }
            }
            if need_restock as ::core::ffi::c_int != 0
                && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                    >= 5 as ::core::ffi::c_int
            {
                if borg_recall() {
                    borg_note(
                        b"# Recalling to town (need to restock)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return true_0 != 0;
                }
            }
        }
        if borg_flow_stair_less(GOAL_BORE, false_0 != 0) {
            borg_note(
                b"# Going to stairs up. I'm bored.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return true_0 != 0;
        } else {
            borg_note(
                b"# Bored but unable to flow to up stairs.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if crate::bridge::st().borg.goal.rising as ::core::ffi::c_int != 0
            && bored as ::core::ffi::c_int != 0
            && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level >= 1000 as borg_time
        {
            if borg_recall() {
                borg_note(
                    b"# Recalling to town (no stairs)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        if track_less.num as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            borg_note(
                b"# no up stairs found, going down\0" as *const u8 as *const ::core::ffi::c_char,
            );
            g = 1 as ::core::ffi::c_int;
        }
    }
    if g > 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.stair_more = true_0 != 0;
        if borg_flow_stair_more(GOAL_BORE, false_0 != 0, false_0 != 0) {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_excavate_vault(mut range: ::core::ffi::c_int) -> bool {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut b_x: ::core::ffi::c_int = 0;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    borg_temp_n = 0 as int16_t;
    i = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    if !crate::bridge::st().borg.status.vault {
        return false_0 != 0;
    }
    if !borg_spell_okay_fail(TURN_STONE_TO_MUD, 30 as ::core::ffi::c_int)
        && !borg_spell_okay_fail(SHATTER_STONE, 30 as ::core::ffi::c_int)
    {
        return false_0 != 0;
    }
    y = crate::bridge::st().core.w_y;
    while y < crate::bridge::st().core.w_y
        + ((*Term).hgt
            - row_top_map[(*Term).sidebar_mode as usize]
            - row_bottom_map[(*Term).sidebar_mode as usize])
            / tile_height as ::core::ffi::c_int
    {
        x = crate::bridge::st().core.w_x;
        while x < crate::bridge::st().core.w_x
            + ((*Term).wid - col_map[(*Term).sidebar_mode as usize] - 1 as ::core::ffi::c_int)
                / tile_width as ::core::ffi::c_int
        {
            if !(distance(crate::bridge::st().borg.c, loc(x, y)) > range) {
                if !((*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                    != FEAT_GRANITE as ::core::ffi::c_int
                    && (*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                        != FEAT_RUBBLE as ::core::ffi::c_int
                    && (*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                        != FEAT_QUARTZ as ::core::ffi::c_int
                    && (*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                        != FEAT_MAGMA as ::core::ffi::c_int
                    && (*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                        != FEAT_QUARTZ_K as ::core::ffi::c_int
                    && (*crate::bridge::grid(x, y)).feat as ::core::ffi::c_int
                        != FEAT_MAGMA_K as ::core::ffi::c_int)
                {
                    i = 0 as ::core::ffi::c_int;
                    while i < 8 as ::core::ffi::c_int {
                        b_x = x + ddx_ddd[i as usize] as ::core::ffi::c_int;
                        b_y = y + ddy_ddd[i as usize] as ::core::ffi::c_int;
                        if square_in_bounds_fully(cave, loc(b_x, b_y)) {
                            ag = crate::bridge::grid(b_x, b_y)
                                as *mut borg_grid;
                            if !((*ag).feat as ::core::ffi::c_int
                                != FEAT_PERM as ::core::ffi::c_int)
                            {
                                ii = 0 as ::core::ffi::c_int;
                                while ii < borg_temp_n as ::core::ffi::c_int {
                                    if borg_temp_y[ii as usize] as ::core::ffi::c_int == y
                                        && borg_temp_x[ii as usize] as ::core::ffi::c_int == x
                                    {
                                        break;
                                    }
                                    ii += 1;
                                }
                                if ii == borg_temp_n as ::core::ffi::c_int && ii < AUTO_TEMP_MAX {
                                    borg_temp_x[ii as usize] = x as uint8_t;
                                    borg_temp_y[ii as usize] = y as uint8_t;
                                    borg_temp_n += 1;
                                }
                            }
                        }
                        i += 1;
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
        if borg_los(
            crate::bridge::st().borg.c.y,
            crate::bridge::st().borg.c.x,
            borg_temp_y[i as usize] as ::core::ffi::c_int,
            borg_temp_x[i as usize] as ::core::ffi::c_int,
        ) {
            borg_target(
                loc(
                    borg_temp_x[i as usize] as ::core::ffi::c_int,
                    borg_temp_y[i as usize] as ::core::ffi::c_int,
                ),
                false_0 != 0,
            );
            if borg_spell(TURN_STONE_TO_MUD) as ::core::ffi::c_int != 0
                || borg_activate_ring(sv_ring_digging) as ::core::ffi::c_int != 0
                || borg_activate_item(act_stone_to_mud) as ::core::ffi::c_int != 0
                || distance(
                    crate::bridge::st().borg.c,
                    loc(
                        borg_temp_x[i as usize] as ::core::ffi::c_int,
                        borg_temp_y[i as usize] as ::core::ffi::c_int,
                    ),
                ) == 1 as ::core::ffi::c_int
                    && borg_spell(SHATTER_STONE) as ::core::ffi::c_int != 0
            {
                borg_note(b"# Excavation of vault\0" as *const u8 as *const ::core::ffi::c_char);
                borg_keypress('5' as i32 as keycode_t);
                borg_do_update_view = true_0 != 0;
                borg_do_update_lite = true_0 != 0;
                let ref mut fresh1 = (*crate::bridge::grid(borg_temp_x[i as usize], borg_temp_y[i as usize]))
                .info;
                *fresh1 = (*fresh1 as ::core::ffi::c_int & !BORG_GLOW) as uint16_t;
                let ref mut fresh2 = (*crate::bridge::grid(borg_temp_x[i as usize], borg_temp_y[i as usize]))
                .info;
                *fresh2 = (*fresh2 as ::core::ffi::c_int | BORG_GLOW) as uint16_t;
                (*crate::bridge::grid(borg_temp_x[i as usize], borg_temp_y[i as usize]))
                .feat = FEAT_FLOOR as ::core::ffi::c_int as uint8_t;
                let ref mut fresh3 = (*crate::bridge::grid(borg_temp_x[i as usize], borg_temp_y[i as usize]))
                .info;
                *fresh3 = (*fresh3 as ::core::ffi::c_int | BORG_IGNORE_MAP) as uint16_t;
                track_vein.num = 0 as int16_t;
                return true_0 != 0;
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
