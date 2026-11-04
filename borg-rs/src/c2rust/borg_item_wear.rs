pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_danger_h::borg_danger;
use crate::headers::borg_flow_misc_h::borg_check_rest;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_init_h::borg_init_failure;
use crate::headers::borg_inventory_h::{
    borg_first_empty_inventory_slot, borg_item_worth_id, borg_wield_slot,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
use crate::headers::borg_item_analyze_h::borg_ego_has_random_power;
pub use crate::headers::borg_item_h::{borg_item, borg_items, safe_items};
use crate::headers::borg_item_id_h::borg_item_note_needs_id;
use crate::headers::borg_item_use_h::borg_read_scroll;
use crate::headers::borg_item_val_h::{
    sv_light_lantern, sv_light_torch, sv_rod_recall, sv_scroll_identify,
};
pub use crate::headers::borg_magic_h::{
    borg_spell, borg_spell_legal, borg_spells, ACID_SPRAY, ANNIHILATE, BANISHMENT, BANISH_EVIL,
    BANISH_SPIRITS, BAT_FORM, BEAR_FORM, BECOME_PUKEL_MAN, BERSERK_STRENGTH, BLESS, BLOODLUST,
    BRAND_AMMUNITION, CALL_LIGHT, CLAIRVOYANCE, COMMAND, CONFUSE_MONSTER, COVER_TRACKS,
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
use crate::headers::borg_power_h::borg_power;
pub use crate::headers::borg_store_h::{
    borg_home_full, borg_inventory_full, borg_shop, borg_shops, SHOP_MENU_ITEMS,
};
use crate::headers::borg_think_h::{borg_oops, shop_num};
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
use crate::headers::borg_trait_swap_h::{armour_swap, weapon_swap};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_object_modifiers_h::{
    OBJ_MOD_BLOWS, OBJ_MOD_DAM_RED, OBJ_MOD_INFRA, OBJ_MOD_LIGHT, OBJ_MOD_MIGHT, OBJ_MOD_MOVES,
    OBJ_MOD_SEARCH, OBJ_MOD_SHOTS, OBJ_MOD_SPEED, OBJ_MOD_STEALTH, OBJ_MOD_TUNNEL,
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
pub use crate::headers::list_stats_h::{
    OBJ_MOD_CON, OBJ_MOD_DEX, OBJ_MOD_INT, OBJ_MOD_STR, OBJ_MOD_WIS,
};
pub use crate::headers::list_tvals_h::{
    TV_AMULET, TV_ARROW, TV_BOLT, TV_BOOTS, TV_BOW, TV_CHEST, TV_CLOAK, TV_CROWN, TV_DIGGING,
    TV_DRAG_ARMOR, TV_FLASK, TV_FOOD, TV_GLOVES, TV_GOLD, TV_HAFTED, TV_HARD_ARMOR, TV_HELM,
    TV_LIGHT, TV_MAGIC_BOOK, TV_MUSHROOM, TV_NATURE_BOOK, TV_NULL, TV_OTHER_BOOK, TV_POLEARM,
    TV_POTION, TV_PRAYER_BOOK, TV_RING, TV_ROD, TV_SCROLL, TV_SHADOW_BOOK, TV_SHIELD, TV_SHOT,
    TV_SOFT_ARMOR, TV_STAFF, TV_SWORD, TV_WAND,
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
pub use crate::headers::obj_properties_h::{
    C2RustUnnamedH_20, C2RustUnnamedH_39, OBJ_MOD_MAX, TV_MAX,
};
pub use crate::headers::object_h::{
    activation, artifact, curse_data, e_info, effect, ego_item, element_info, flavor, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::{player_options, C2RustUnnamedH_21, OPT_MAX};
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_h::UINT16_MAX;
pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memcpy;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
use crate::headers::ui_menu_h::all_letters_nohjkl;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::{my_rational, my_strcpy};
use crate::headers::z_virt_h::{mem_alloc, mem_free, mem_zalloc};
#[no_mangle]
pub static mut track_worn_num: int16_t = 0;
#[no_mangle]
pub static mut track_worn_size: int16_t = 0;
#[no_mangle]
pub static mut track_worn_name1: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
#[no_mangle]
pub static mut track_worn_time: int16_t = 0;
#[no_mangle]
pub unsafe extern "C" fn borg_test_stuff() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_v: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut inv_item_needs_id: bool = false_0 != 0;
    let mut free_id: bool = borg_spell_legal(IDENTIFY_RUNE);
    if crate::bridge::st().borg.trait_[(BI_CURSP as ::core::ffi::c_int) as usize] < 50 as ::core::ffi::c_int
        && borg_spell_legal(IDENTIFY_RUNE) as ::core::ffi::c_int != 0
        && !borg_check_rest(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x)
    {
        return false_0 != 0;
    }
    if borg_danger(
        crate::bridge::st().borg.c.y,
        crate::bridge::st().borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        false_0 != 0,
    ) > 1 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    i = (*z_info).pack_size as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).needs_ident {
                if (*item).art_idx != 0 {
                    v = ((*item).value as ::core::ffi::c_long + 150000 as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                }
                if borg_ego_has_random_power(
                    e_info.offset((*item).ego_idx as isize) as *mut ego_item
                ) {
                    v = ((*item).value as ::core::ffi::c_long + 100000 as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                }
                if borg_item_note_needs_id(item) {
                    v = ((*item).value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                }
                if !(v == 0) {
                    if !(v <= b_v) {
                        b_i = i;
                        b_v = v;
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i
        < (*z_info).pack_size as ::core::ffi::c_int
            + 12 as ::core::ffi::c_int
            + (*z_info).quiver_size as ::core::ffi::c_int
    {
        let mut v_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut item_0: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item_0).iqty == 0) {
            if (*item_0).needs_ident {
                if i < (*z_info).pack_size as ::core::ffi::c_int {
                    inv_item_needs_id = true_0 != 0;
                }
                if (*item_0).art_idx != 0 {
                    v_0 = ((*item_0).value as ::core::ffi::c_long + 150000 as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                }
                if borg_item_note_needs_id(item_0) {
                    v_0 = ((*item_0).value as ::core::ffi::c_long + 20000 as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                } else if free_id as ::core::ffi::c_int != 0
                    || borg_item_worth_id(item_0) as ::core::ffi::c_int != 0
                {
                    v_0 = (*item_0).value as ::core::ffi::c_int;
                }
                if !(*item_0).aware {
                    match (*item_0).tval as ::core::ffi::c_int {
                        21 | 20 => {
                            v_0 = (v_0 as ::core::ffi::c_long
                                + crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_long
                                    * 5000 as ::core::ffi::c_long)
                                as ::core::ffi::c_int;
                        }
                        24 => {
                            v_0 = (v_0 as ::core::ffi::c_long
                                + crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_long
                                    * 3000 as ::core::ffi::c_long)
                                as ::core::ffi::c_int;
                        }
                        23 | 22 => {
                            v_0 = (v_0 as ::core::ffi::c_long
                                + crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_long
                                    * 2000 as ::core::ffi::c_long)
                                as ::core::ffi::c_int;
                        }
                        26 | 25 => {
                            if !(crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                < 5 as ::core::ffi::c_int)
                            {
                                v_0 = (v_0 as ::core::ffi::c_long
                                    + crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                        as ::core::ffi::c_long
                                        * 500 as ::core::ffi::c_long)
                                    as ::core::ffi::c_int;
                            }
                        }
                        28 => {
                            v_0 = (v_0 as ::core::ffi::c_long
                                + crate::bridge::st().borg.trait_[(BI_MAXDEPTH as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_long
                                    * 10 as ::core::ffi::c_long)
                                as ::core::ffi::c_int;
                        }
                        _ => {}
                    }
                }
                if !(v_0 == 0) {
                    if !(v_0 <= b_v) {
                        b_i = i;
                        b_v = v_0;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i >= 0 as ::core::ffi::c_int {
        let mut item_1: *mut borg_item = borg_items.offset(b_i as isize) as *mut borg_item;
        if borg_spell(IDENTIFY_RUNE) as ::core::ffi::c_int != 0
            || borg_read_scroll(sv_scroll_identify) as ::core::ffi::c_int != 0
        {
            borg_note(format(
                b"# Identifying %s.\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*item_1).desc as *mut ::core::ffi::c_char,
            ));
            if b_i >= (*z_info).pack_size as ::core::ffi::c_int
                && b_i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            {
                if inv_item_needs_id {
                    borg_keypress('/' as i32 as keycode_t);
                }
                borg_keypress(
                    *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                        .offset((b_i - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                        as keycode_t,
                );
            } else if b_i >= (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
                borg_keypress('|' as i32 as keycode_t);
                borg_keypress(
                    (b_i - ((*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                        + '0' as i32) as keycode_t,
                );
            } else {
                borg_keypress(
                    *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                        .offset(b_i as isize) as keycode_t,
                );
            }
            borg_keypress(ESCAPE as keycode_t);
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_swap_rings() -> bool {
    let mut hole: ::core::ffi::c_int = borg_first_empty_inventory_slot();
    let mut v1: int32_t = 0;
    let mut v2: int32_t = 0;
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if hole + 1 as ::core::ffi::c_int
        >= (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 1000 as borg_time {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        != 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
    .iqty as ::core::ffi::c_int
        != 0
        && (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
        .iqty
            == 0
        && !(*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
        .one_ring
    {
        borg_note(b"# Taking off naked loose ring.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress('t' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - (*z_info).pack_size as ::core::ffi::c_int) as isize,
            ) as keycode_t,
        );
        return true_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
    .iqty
        == 0
    {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
    .one_ring
    {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
    .iqty
        == 0
    {
        memcpy(
            borg_items.offset(hole as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        (*borg_items.offset(
            ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        ))
        .iqty = 0 as uint8_t;
        borg_notice(true_0 != 0);
        v1 = borg_power();
        memcpy(
            borg_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(
                ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            ) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
    } else {
        v1 = crate::bridge::st().borg.power;
    }
    memcpy(
        borg_items.offset(hole as isize) as *mut borg_item as *mut ::core::ffi::c_void,
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
            as *mut borg_item as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_item>() as size_t,
    );
    (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
    .iqty = 0 as uint8_t;
    borg_notice(false_0 != 0);
    v2 = borg_power();
    memcpy(
        borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
            as *mut borg_item as *mut ::core::ffi::c_void,
        safe_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
            as *mut borg_item as *const ::core::ffi::c_void,
        ::core::mem::size_of::<borg_item>() as size_t,
    );
    (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
    if v2 > v1 {
        borg_note(
            b"# Taking off less valuable right ring.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
        .iqty
            != 0
        {
            borg_keypress('t' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                        - (*z_info).pack_size as ::core::ffi::c_int) as isize,
                ) as keycode_t,
            );
        }
        if (*borg_items
            .offset(((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
        .iqty
            != 0
        {
            borg_note(
                b"# Taking off more valuable left ring.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            borg_keypress('t' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(
                    ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                        - (*z_info).pack_size as ::core::ffi::c_int) as isize,
                ) as keycode_t,
            );
        }
        return true_0 != 0;
    }
    borg_notice(true_0 != 0);
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_wear_rings() -> bool {
    let mut hole: ::core::ffi::c_int = borg_first_empty_inventory_slot();
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut fix: bool = false_0 != 0;
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    if (*borg_items
        .offset(((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
    .iqty
        != 0
    {
        return false_0 != 0;
    }
    if hole + 1 as ::core::ffi::c_int
        >= (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
    {
        return false_0 != 0;
    }
    if (*borg_items.offset((hole + 1 as ::core::ffi::c_int) as isize)).iqty != 0 {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !((*item).value == 0) {
                    if !((*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                        && !(*item).ident
                        && (*item).art_idx as ::core::ffi::c_int != 0)
                    {
                        if !((*item).tval as ::core::ffi::c_int != TV_RING as ::core::ffi::c_int) {
                            memcpy(
                                borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut borg_item
                                    as *mut ::core::ffi::c_void,
                                item as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<borg_item>() as size_t,
                            );
                            (*borg_items.offset(
                                ((*z_info).pack_size as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .iqty = 1 as uint8_t;
                            (*item).iqty = (*item).iqty.wrapping_sub(1);
                            fix = true_0 != 0;
                            borg_notice(true_0 != 0);
                            p = borg_power();
                            if (*item).one_ring {
                                p = crate::bridge::st().borg.power * 2 as int32_t;
                            }
                            (*borg_items.offset(
                                ((*z_info).pack_size as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .iqty = 0 as uint8_t;
                            (*item).iqty = (*item).iqty.wrapping_add(1);
                            if !(b_i >= 0 as ::core::ffi::c_int && p < b_p) {
                                b_i = i;
                                b_p = p;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_i >= 0 as ::core::ffi::c_int && b_p > crate::bridge::st().borg.power {
        item = borg_items.offset(b_i as isize) as *mut borg_item;
        borg_note(b"# Putting on best tight ring.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_note(format(
            b"# Wearing %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('w' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        crate::bridge::st().borg.antibounce_count += 1;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_backup_swap(mut p: ::core::ffi::c_int) -> bool {
    let mut slot: ::core::ffi::c_int = 0;
    let mut swap: ::core::ffi::c_int = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut b_p1: int32_t = 0 as int32_t;
    let mut b_p2: int32_t = 0 as int32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut save_rconf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save_rblind: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save_fract: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut fix: bool = false_0 != 0;
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 300 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if armour_swap == 0 && weapon_swap == 0 {
        return false_0 != 0;
    }
    if armour_swap != 0 {
        save_rconf = crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize];
        save_rblind = crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize];
        save_fract = crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize];
        i = armour_swap - 1 as ::core::ffi::c_int;
        if i == -(1 as ::core::ffi::c_int) {
            i = 0 as ::core::ffi::c_int;
        }
        item = borg_items.offset(i as isize) as *mut borg_item;
        slot = borg_wield_slot(item);
        if slot < 0 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        memcpy(
            borg_items.offset(slot as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        (*borg_items.offset(slot as isize)).iqty = 1 as uint8_t;
        let ref mut fresh0 = (*borg_items.offset(i as isize)).iqty;
        *fresh0 = (*fresh0).wrapping_sub(1);
        fix = true_0 != 0;
        borg_notice(false_0 != 0);
        b_p1 = borg_danger(
            crate::bridge::st().borg.c.y,
            crate::bridge::st().borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        ) as int32_t;
        memcpy(
            borg_items.offset(slot as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(slot as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        memcpy(
            borg_items.offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        if save_rconf != 0
            && crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
        {
            b_p1 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if save_rblind != 0
            && (crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                == 0
                && crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    < 100 as ::core::ffi::c_int)
        {
            b_p1 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if save_fract != 0
            && (crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    < 100 as ::core::ffi::c_int)
        {
            b_p1 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if fix {
            borg_notice(true_0 != 0);
        }
        if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
            && (*item).art_idx as ::core::ffi::c_int != 0
            && !(*item).ident
        {
            b_p1 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if (*item).cursed as ::core::ffi::c_int != 0 && !(*item).uncursable {
            b_p1 = 9999 as ::core::ffi::c_int as int32_t;
        }
    }
    if weapon_swap != 0 {
        i = weapon_swap - 1 as ::core::ffi::c_int;
        if i == -(1 as ::core::ffi::c_int) {
            i = 0 as ::core::ffi::c_int;
        }
        item = borg_items.offset(i as isize) as *mut borg_item;
        slot = borg_wield_slot(item);
        if slot < 0 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        memcpy(
            borg_items.offset(slot as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        (*borg_items.offset(slot as isize)).iqty = 1 as uint8_t;
        let ref mut fresh1 = (*borg_items.offset(i as isize)).iqty;
        *fresh1 = (*fresh1).wrapping_sub(1);
        fix = true_0 != 0;
        borg_notice(false_0 != 0);
        b_p2 = borg_danger(
            crate::bridge::st().borg.c.y,
            crate::bridge::st().borg.c.x,
            1 as ::core::ffi::c_int,
            true_0 != 0,
            false_0 != 0,
        ) as int32_t;
        if save_rconf != 0
            && crate::bridge::st().borg.trait_[(BI_RCONF as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int
        {
            b_p2 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if save_rblind != 0
            && (crate::bridge::st().borg.trait_[(BI_RBLIND as ::core::ffi::c_int) as usize]
                == 0
                && crate::bridge::st().borg.trait_[(BI_RLITE as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_RDARK as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    < 100 as ::core::ffi::c_int)
        {
            b_p2 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if save_fract != 0
            && (crate::bridge::st().borg.trait_[(BI_FRACT as ::core::ffi::c_int) as usize] == 0
                && crate::bridge::st().borg.trait_[(BI_SAV as ::core::ffi::c_int) as usize]
                    < 100 as ::core::ffi::c_int)
        {
            b_p2 = 9999 as ::core::ffi::c_int as int32_t;
        }
        memcpy(
            borg_items.offset(slot as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(slot as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        memcpy(
            borg_items.offset(i as isize) as *mut borg_item as *mut ::core::ffi::c_void,
            safe_items.offset(i as isize) as *mut borg_item as *const ::core::ffi::c_void,
            ::core::mem::size_of::<borg_item>() as size_t,
        );
        if fix {
            borg_notice(true_0 != 0);
        }
        if (*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            != 0
            && (*item).art_idx as ::core::ffi::c_int != 0
            && !(*item).ident
        {
            b_p2 = 9999 as ::core::ffi::c_int as int32_t;
        }
        if (*item).cursed as ::core::ffi::c_int != 0 && !(*item).uncursable {
            b_p2 = 9999 as ::core::ffi::c_int as int32_t;
        }
    }
    if b_p1 <= b_p2 {
        b_p = b_p2;
        swap = weapon_swap - 1 as ::core::ffi::c_int;
    } else {
        b_p = b_p1;
        swap = armour_swap - 1 as ::core::ffi::c_int;
    }
    if p as int32_t > b_p
        && b_p
            <= (if crate::bridge::st().borg.near.unique as ::core::ffi::c_int != 0 {
                crate::bridge::st().borg.avoidance as int32_t * 2 as int32_t / 3 as int32_t
            } else {
                crate::bridge::st().borg.avoidance as int32_t / 2 as int32_t
            })
    {
        borg_note(format(
            b"# Swapping backup.  (%ld < %d).\0" as *const u8 as *const ::core::ffi::c_char,
            b_p as ::core::ffi::c_long,
            p,
        ));
        borg_keypress('w' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(swap as isize)
                as keycode_t,
        );
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_wear_stuff() -> bool {
    let mut hole: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slot: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut o: ::core::ffi::c_int = 0;
    let mut recently_worn: bool = false_0 != 0;
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ii: ::core::ffi::c_int = 0;
    let mut b_ii: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut danger: ::core::ffi::c_int = 0;
    let mut target_ring_desc: [::core::ffi::c_char; 80] = [0; 80];
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut fix: bool = false_0 != 0;
    b_p = crate::bridge::st().borg.power;
    if crate::bridge::st().borg.trait_[(BI_ISHUNGRY as ::core::ffi::c_int) as usize]
        != 0
        || crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
            != 0
    {
        return false_0 != 0;
    }
    hole = borg_first_empty_inventory_slot();
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 1300 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut current_block_59: u64;
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int {
        item = borg_items.offset(i as isize) as *mut borg_item;
        recently_worn = false_0 != 0;
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !((*item).value == 0) {
                    if !((*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                        && (*item).art_idx as ::core::ffi::c_int != 0
                        && !(*item).ident)
                    {
                        o = 0 as ::core::ffi::c_int;
                        while o < track_worn_num as ::core::ffi::c_int {
                            if track_worn_num as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                                && *track_worn_name1.offset(o as isize) as ::core::ffi::c_int
                                    == (*item).art_idx as ::core::ffi::c_int
                                && crate::bridge::st().borg.time.now - track_worn_time as borg_time > 10 as borg_time
                            {
                                recently_worn = true_0 != 0;
                            }
                            o += 1;
                        }
                        if recently_worn as ::core::ffi::c_int == true_0 {
                            borg_note(format(
                                b"# Not considering a %s; it was recently worn.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &raw mut (*item).desc as *mut ::core::ffi::c_char,
                            ));
                        } else {
                            slot = borg_wield_slot(item);
                            if !(slot < 0 as ::core::ffi::c_int) {
                                if crate::bridge::st().borg.trait_[(BI_ISENCUMB as ::core::ffi::c_int) as usize]
                                    != 0
                                {
                                    if (*borg_items.offset(slot as isize)).modifiers
                                        [OBJ_MOD_STR as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        > (*item).modifiers
                                            [OBJ_MOD_STR as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                    {
                                        current_block_59 = 5399440093318478209;
                                    } else {
                                        current_block_59 = 5689316957504528238;
                                    }
                                } else {
                                    current_block_59 = 5689316957504528238;
                                }
                                match current_block_59 {
                                    5399440093318478209 => {}
                                    _ => {
                                        danger = borg_danger(
                                            crate::bridge::st().borg.c.y,
                                            crate::bridge::st().borg.c.x,
                                            1 as ::core::ffi::c_int,
                                            true_0 != 0,
                                            false_0 != 0,
                                        );
                                        if slot
                                            != (*z_info).pack_size as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int
                                            || ((*borg_items.offset(
                                                ((*z_info).pack_size as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int)
                                                    as isize,
                                            ))
                                            .iqty
                                                == 0
                                                || (*borg_items.offset(
                                                    ((*z_info).pack_size as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                ))
                                                .iqty
                                                    == 0)
                                        {
                                            memcpy(
                                                borg_items.offset(hole as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                safe_items.offset(slot as isize) as *mut borg_item
                                                    as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                            memcpy(
                                                borg_items.offset(slot as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                safe_items.offset(i as isize) as *mut borg_item
                                                    as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                            (*borg_items.offset(slot as isize)).iqty = 1 as uint8_t;
                                            let ref mut fresh2 =
                                                (*borg_items.offset(i as isize)).iqty;
                                            *fresh2 = (*fresh2).wrapping_sub(1);
                                            fix = true_0 != 0;
                                            borg_notice(true_0 != 0);
                                            p = borg_power();
                                            d = borg_danger(
                                                crate::bridge::st().borg.c.y,
                                                crate::bridge::st().borg.c.x,
                                                1 as ::core::ffi::c_int,
                                                true_0 != 0,
                                                false_0 != 0,
                                            );
                                            memcpy(
                                                borg_items.offset(slot as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                safe_items.offset(slot as isize) as *mut borg_item
                                                    as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                            memcpy(
                                                borg_items.offset(i as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                safe_items.offset(i as isize) as *mut borg_item
                                                    as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                            (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
                                            if (*item).one_ring as ::core::ffi::c_int != 0
                                                && (*borg_items.offset(
                                                    ((*z_info).pack_size as ::core::ffi::c_int
                                                        + 3 as ::core::ffi::c_int)
                                                        as isize,
                                                ))
                                                .tval
                                                    == 0
                                            {
                                                p = -(99999 as ::core::ffi::c_int) as int32_t;
                                            }
                                            if danger < d {
                                                current_block_59 = 5399440093318478209;
                                            } else if p <= b_p + 50 as int32_t {
                                                current_block_59 = 5399440093318478209;
                                            } else {
                                                b_i = i;
                                                b_p = p;
                                                current_block_59 = 2543120759711851213;
                                            }
                                        } else {
                                            current_block_59 = 2543120759711851213;
                                        }
                                        match current_block_59 {
                                            5399440093318478209 => {}
                                            _ => {
                                                if Rand_div(100 as uint32_t) as int32_t
                                                    == 10 as int32_t
                                                    || (*item).one_ring as ::core::ffi::c_int != 0
                                                {
                                                    if slot
                                                        == (*z_info).pack_size as ::core::ffi::c_int
                                                            + 3 as ::core::ffi::c_int
                                                        && (*borg_items.offset(
                                                            ((*z_info).pack_size
                                                                as ::core::ffi::c_int
                                                                + 3 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                        .iqty
                                                            as ::core::ffi::c_int
                                                            != 0
                                                        && (*borg_items.offset(
                                                            ((*z_info).pack_size
                                                                as ::core::ffi::c_int
                                                                + 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                        .iqty
                                                            as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        ii = (*z_info).pack_size
                                                            as ::core::ffi::c_int
                                                            + 2 as ::core::ffi::c_int;
                                                        while ii
                                                            <= (*z_info).pack_size
                                                                as ::core::ffi::c_int
                                                                + 3 as ::core::ffi::c_int
                                                        {
                                                            slot = ii;
                                                            memcpy(
                                                                borg_items.offset(hole as isize)
                                                                    as *mut borg_item
                                                                    as *mut ::core::ffi::c_void,
                                                                safe_items.offset(slot as isize)
                                                                    as *mut borg_item
                                                                    as *const ::core::ffi::c_void,
                                                                ::core::mem::size_of::<borg_item>()
                                                                    as size_t,
                                                            );
                                                            memcpy(
                                                                borg_items.offset(slot as isize)
                                                                    as *mut borg_item
                                                                    as *mut ::core::ffi::c_void,
                                                                safe_items.offset(i as isize)
                                                                    as *mut borg_item
                                                                    as *const ::core::ffi::c_void,
                                                                ::core::mem::size_of::<borg_item>()
                                                                    as size_t,
                                                            );
                                                            (*borg_items.offset(slot as isize))
                                                                .iqty = 1 as uint8_t;
                                                            let ref mut fresh3 = (*borg_items
                                                                .offset(i as isize))
                                                            .iqty;
                                                            *fresh3 = (*fresh3).wrapping_sub(1);
                                                            fix = true_0 != 0;
                                                            borg_notice(true_0 != 0);
                                                            p = borg_power();
                                                            d = borg_danger(
                                                                crate::bridge::st().borg.c.y,
                                                                crate::bridge::st().borg.c.x,
                                                                1 as ::core::ffi::c_int,
                                                                true_0 != 0,
                                                                false_0 != 0,
                                                            );
                                                            memcpy(
                                                                borg_items.offset(slot as isize)
                                                                    as *mut borg_item
                                                                    as *mut ::core::ffi::c_void,
                                                                safe_items.offset(slot as isize)
                                                                    as *mut borg_item
                                                                    as *const ::core::ffi::c_void,
                                                                ::core::mem::size_of::<borg_item>()
                                                                    as size_t,
                                                            );
                                                            memcpy(
                                                                borg_items.offset(i as isize)
                                                                    as *mut borg_item
                                                                    as *mut ::core::ffi::c_void,
                                                                safe_items.offset(i as isize)
                                                                    as *mut borg_item
                                                                    as *const ::core::ffi::c_void,
                                                                ::core::mem::size_of::<borg_item>()
                                                                    as size_t,
                                                            );
                                                            (*borg_items.offset(hole as isize))
                                                                .iqty = 0 as uint8_t;
                                                            if ii
                                                                == (*z_info).pack_size
                                                                    as ::core::ffi::c_int
                                                                    + 3 as ::core::ffi::c_int
                                                                && (*item).one_ring
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                            {
                                                                p = -(99999 as ::core::ffi::c_int)
                                                                    as int32_t;
                                                            }
                                                            if !(p < b_p) {
                                                                if !(danger <= d
                                                                    && danger
                                                                        != 0 as ::core::ffi::c_int)
                                                                {
                                                                    b_i = i;
                                                                    b_p = p;
                                                                    b_ii = ii;
                                                                }
                                                            }
                                                            ii += 1;
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
        i += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_i >= 0 as ::core::ffi::c_int && b_p > crate::bridge::st().borg.power {
        item = borg_items.offset(b_i as isize) as *mut borg_item;
        my_strcpy(
            &raw mut target_ring_desc as *mut ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
        );
        if b_ii >= (*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            && (*item).tval as ::core::ffi::c_int == TV_RING as ::core::ffi::c_int
        {
            borg_note(format(
                b"# Removing %s to make room for %s.\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*borg_items.offset(b_ii as isize)).desc as *mut ::core::ffi::c_char,
                &raw mut (*item).desc as *mut ::core::ffi::c_char,
            ));
            borg_keypress('t' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((b_ii - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
            crate::bridge::st().borg.antibounce_count += 1;
            return true_0 != 0;
        }
        borg_note(format(
            b"# Wearing %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('w' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        crate::bridge::st().borg.antibounce_count += 1;
        if (*item).art_idx as ::core::ffi::c_int != 0
            && (track_worn_num as ::core::ffi::c_int) < track_worn_size as ::core::ffi::c_int
        {
            borg_note(
                b"# Noting the wearing of artifact.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            *track_worn_name1.offset(track_worn_num as isize) = (*item).art_idx;
            track_worn_time = crate::bridge::st().borg.time.now as int16_t;
            track_worn_num += 1;
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_best_stuff_order(mut n: ::core::ffi::c_int) -> uint16_t {
    match n {
        0 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint16_t;
        }
        1 => return (*z_info).pack_size,
        2 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
                as uint16_t;
        }
        3 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                as uint16_t;
        }
        4 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                as uint16_t;
        }
        5 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 9 as ::core::ffi::c_int)
                as uint16_t;
        }
        6 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                as uint16_t;
        }
        7 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
                as uint16_t;
        }
        8 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                as uint16_t;
        }
        9 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as uint16_t;
        }
        10 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                as uint16_t;
        }
        11 => {
            return ((*z_info).pack_size as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as uint16_t;
        }
        _ => return UINT16_MAX as uint16_t,
    };
}
unsafe extern "C" fn borg_one_step_wearing_best(mut skip: ::core::ffi::c_int) -> bool {
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    let mut item_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slot: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.goal.best_item.is_null() {
        return false_0 != 0;
    }
    loop {
        let mut skipping: ::core::ffi::c_int = skip;
        if (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).tval as ::core::ffi::c_int != 0
            || (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).pval as ::core::ffi::c_int != 0
            || (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).sval as ::core::ffi::c_int != 0
        {
            if !(skipping != 0) {
                break;
            }
            item_num += 1;
            skipping -= 1;
            if item_num >= (*z_info).equip_slots_max as ::core::ffi::c_int {
                return false_0 != 0;
            }
        } else {
            item_num += 1;
            if item_num >= (*z_info).equip_slots_max as ::core::ffi::c_int {
                borg_clear_best();
                if skip == 0 {
                    crate::bridge::st().borg.goal.do_best = false_0 != 0;
                }
                return false_0 != 0;
            }
        }
    }
    slot = borg_best_stuff_order(item_num) as ::core::ffi::c_int;
    if (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).home {
        if (*borg_items.offset(slot as isize)).iqty != 0 {
            if borg_home_full() {
                return borg_one_step_wearing_best(skip + 1 as ::core::ffi::c_int);
            }
            borg_keypress('d' as i32 as keycode_t);
            borg_keypress('/' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((slot - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
            return true_0 != 0;
        }
        if borg_inventory_full() {
            return borg_one_step_wearing_best(skip + 1 as ::core::ffi::c_int);
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).store_inven_max as ::core::ffi::c_int {
            item = (*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(i as isize) as *mut borg_item;
            if (*item).pval as ::core::ffi::c_int
                != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).pval as ::core::ffi::c_int
                || (*item).tval as ::core::ffi::c_int
                    != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).tval as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int
                    != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).sval as ::core::ffi::c_int
            {
                i += 1;
            } else {
                borg_keypress(*SHOP_MENU_ITEMS.offset(i as isize) as keycode_t);
                borg_keypress('p' as i32 as keycode_t);
                if (*item).iqty as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    borg_keypress(KC_ENTER as keycode_t);
                }
                (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).home = false_0 != 0;
                return true_0 != 0;
            }
        }
        borg_oops(b"unable to find item in home\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        if (*borg_items.offset(slot as isize)).iqty != 0 {
            if borg_inventory_full() {
                return borg_one_step_wearing_best(skip + 1 as ::core::ffi::c_int);
            }
            borg_keypress('t' as i32 as keycode_t);
            borg_keypress(
                *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                    .offset((slot - (*z_info).pack_size as ::core::ffi::c_int) as isize)
                    as keycode_t,
            );
            return true_0 != 0;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
            item = borg_items.offset(i as isize) as *mut borg_item;
            if (*item).pval as ::core::ffi::c_int
                != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).pval as ::core::ffi::c_int
                || (*item).tval as ::core::ffi::c_int
                    != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).tval as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int
                    != (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).sval as ::core::ffi::c_int
            {
                i += 1;
            } else {
                borg_keypress('w' as i32 as keycode_t);
                borg_keypress(
                    *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char)
                        .offset(i as isize) as keycode_t,
                );
                (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).home = false_0 != 0;
                (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).tval = 0 as uint8_t;
                (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).pval = 0 as int16_t;
                (*crate::bridge::st().borg.goal.best_item.offset(item_num as isize)).sval = 0 as uint8_t;
                return true_0 != 0;
            }
        }
        borg_oops(b"unable to find item in inventory\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return false_0 != 0;
}
unsafe extern "C" fn borg_best_stuff_aux(
    mut n: ::core::ffi::c_int,
    mut test: *mut uint16_t,
    mut best: *mut uint16_t,
    mut vp: *mut int32_t,
) {
    let mut i: uint16_t = 0;
    let mut slot: ::core::ffi::c_int = 0;
    slot = borg_best_stuff_order(n) as ::core::ffi::c_int;
    if slot == UINT16_MAX {
        let mut p: int32_t = 0;
        borg_notice(true_0 != 0);
        p = borg_power();
        if p > *vp {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (*z_info).equip_slots_max as ::core::ffi::c_int {
                *best.offset(i as isize) = *test.offset(i as isize);
                i = i.wrapping_add(1);
            }
            *vp = p;
        }
        return;
    }
    *test.offset(n as isize) = slot as uint16_t;
    borg_best_stuff_aux(n + 1 as ::core::ffi::c_int, test, best, vp);
    let mut current_block_23: u64;
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_int)
        < (*z_info).pack_size as ::core::ffi::c_int
            + (*z_info).store_inven_max as ::core::ffi::c_int
    {
        let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
        if (i as ::core::ffi::c_int) < (*z_info).pack_size as ::core::ffi::c_int {
            item = borg_items.offset(i as isize) as *mut borg_item;
        } else {
            item = (*borg_shops.offset(BORG_HOME as isize)).ware.offset(
                (i as ::core::ffi::c_int - (*z_info).pack_size as ::core::ffi::c_int) as isize,
            ) as *mut borg_item;
        }
        if !((*item).iqty == 0) {
            if (*item).aware {
                if !((*item).value == 0) {
                    if !((*item).cursed as ::core::ffi::c_int != 0 && !(*item).uncursable) {
                        if !((*player).opts.opt[OPT_birth_randarts as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            != 0
                            && (*item).art_idx as ::core::ffi::c_int != 0
                            && !(*item).ident)
                        {
                            if slot == borg_wield_slot(item)
                                || slot
                                    == (*z_info).pack_size as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                    && borg_wield_slot(item)
                                        == (*z_info).pack_size as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int
                            {
                                if slot
                                    == (*z_info).pack_size as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                {
                                    if (i as ::core::ffi::c_int)
                                        < (*z_info).pack_size as ::core::ffi::c_int
                                    {
                                        if *test.offset((n - 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int
                                            == i as ::core::ffi::c_int
                                        {
                                            current_block_23 = 7651349459974463963;
                                        } else {
                                            current_block_23 = 2891135413264362348;
                                        }
                                    } else if *test.offset((n - 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int
                                        == i as ::core::ffi::c_int
                                            - (*z_info).pack_size as ::core::ffi::c_int
                                            + 100 as ::core::ffi::c_int
                                    {
                                        current_block_23 = 7651349459974463963;
                                    } else {
                                        current_block_23 = 2891135413264362348;
                                    }
                                } else if slot
                                    == (*z_info).pack_size as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                {
                                    if (i as ::core::ffi::c_int)
                                        < (*z_info).pack_size as ::core::ffi::c_int
                                    {
                                        if *test.offset((n + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int
                                            == i as ::core::ffi::c_int
                                        {
                                            current_block_23 = 7651349459974463963;
                                        } else {
                                            current_block_23 = 2891135413264362348;
                                        }
                                    } else if *test.offset((n + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int
                                        == i as ::core::ffi::c_int
                                            - (*z_info).pack_size as ::core::ffi::c_int
                                            + 100 as ::core::ffi::c_int
                                    {
                                        current_block_23 = 7651349459974463963;
                                    } else {
                                        current_block_23 = 2891135413264362348;
                                    }
                                } else {
                                    current_block_23 = 2891135413264362348;
                                }
                                match current_block_23 {
                                    7651349459974463963 => {}
                                    _ => {
                                        if !(*borg_items.offset(slot as isize)).one_ring {
                                            memcpy(
                                                borg_items.offset(slot as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                item as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                            if (i as ::core::ffi::c_int)
                                                < (*z_info).pack_size as ::core::ffi::c_int
                                            {
                                                *test.offset(n as isize) = i;
                                            } else {
                                                *test.offset(n as isize) = (i as ::core::ffi::c_int
                                                    - (*z_info).pack_size as ::core::ffi::c_int
                                                    + 100 as ::core::ffi::c_int)
                                                    as uint16_t;
                                            }
                                            borg_best_stuff_aux(
                                                n + 1 as ::core::ffi::c_int,
                                                test,
                                                best,
                                                vp,
                                            );
                                            memcpy(
                                                borg_items.offset(slot as isize) as *mut borg_item
                                                    as *mut ::core::ffi::c_void,
                                                safe_items.offset(slot as isize) as *mut borg_item
                                                    as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<borg_item>() as size_t,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_best_stuff() -> bool {
    let mut k: ::core::ffi::c_int = 0;
    let mut value: int32_t = 0;
    let mut i: uint16_t = 0;
    let mut test: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut best: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    if shop_num as ::core::ffi::c_int != BORG_HOME {
        return false_0 != 0;
    }
    if !crate::bridge::st().borg.goal.do_best {
        return borg_one_step_wearing_best(0 as ::core::ffi::c_int);
    }
    if borg_home_full() as ::core::ffi::c_int != 0
        || borg_inventory_full() as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    if !crate::bridge::st().borg.goal.do_best {
        return false_0 != 0;
    }
    best = mem_alloc(
        (::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_mul((*z_info).equip_slots_max as size_t),
    ) as *mut uint16_t;
    test = mem_alloc(
        (::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_mul((*z_info).equip_slots_max as size_t),
    ) as *mut uint16_t;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).equip_slots_max as ::core::ffi::c_int {
        let ref mut fresh4 = *test.offset(k as isize);
        *fresh4 = UINT16_MAX as uint16_t;
        *best.offset(k as isize) = *fresh4;
        k += 1;
    }
    value = crate::bridge::st().borg.power;
    borg_best_stuff_aux(0 as ::core::ffi::c_int, test, best, &raw mut value);
    borg_notice(true_0 != 0);
    let mut item: *mut borg_item = ::core::ptr::null_mut::<borg_item>();
    crate::bridge::st().borg.goal.best_item = mem_zalloc(
        (::core::mem::size_of::<borg_best>() as size_t)
            .wrapping_mul((*z_info).equip_slots_max as size_t),
    ) as *mut borg_best;
    let mut found: bool = false_0 != 0;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).equip_slots_max as ::core::ffi::c_int {
        i = *best.offset(k as isize);
        if !(i as ::core::ffi::c_int == borg_best_stuff_order(k) as ::core::ffi::c_int
            || UINT16_MAX == i as ::core::ffi::c_int)
        {
            found = true_0 != 0;
            if (i as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
                item = borg_items.offset(i as isize) as *mut borg_item;
            } else {
                item = (*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset((i as ::core::ffi::c_int - 100 as ::core::ffi::c_int) as isize)
                    as *mut borg_item;
                (*crate::bridge::st().borg.goal.best_item.offset(k as isize)).home = true_0 != 0;
            }
            (*crate::bridge::st().borg.goal.best_item.offset(k as isize)).tval = (*item).tval;
            (*crate::bridge::st().borg.goal.best_item.offset(k as isize)).pval = (*item).pval;
            (*crate::bridge::st().borg.goal.best_item.offset(k as isize)).sval = (*item).sval;
        }
        k += 1;
    }
    if !found {
        borg_clear_best();
    }
    crate::bridge::st().borg.goal.do_best = false_0 != 0;
    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
        if found {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (*z_info).equip_slots_max as ::core::ffi::c_int {
                if *best.offset(i as isize) as ::core::ffi::c_int >= 100 as ::core::ffi::c_int {
                    borg_note(format(
                        b"new best: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut (*(*borg_shops.offset(BORG_HOME as isize)).ware.offset(
                            (*best.offset(i as isize) as ::core::ffi::c_int
                                - 100 as ::core::ffi::c_int) as isize,
                        ))
                        .desc as *mut ::core::ffi::c_char,
                    ));
                } else {
                    borg_note(format(
                        b"new best: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut (*borg_items.offset(*best.offset(i as isize) as isize)).desc
                            as *mut ::core::ffi::c_char,
                    ));
                }
                i = i.wrapping_add(1);
            }
        } else {
            borg_note(b"new best: no changes\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    mem_free(best as *mut ::core::ffi::c_void);
    best = ::core::ptr::null_mut::<uint16_t>();
    mem_free(test as *mut ::core::ffi::c_void);
    test = ::core::ptr::null_mut::<uint16_t>();
    return borg_one_step_wearing_best(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_clear_best() {
    if !crate::bridge::st().borg.goal.best_item.is_null() {
        mem_free(crate::bridge::st().borg.goal.best_item as *mut ::core::ffi::c_void);
        crate::bridge::st().borg.goal.best_item = ::core::ptr::null_mut::<borg_best>();
    }
    crate::bridge::st().borg.goal.do_best = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_wear_recharge() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut b_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut slot: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut b_slot: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !borg_check_rest(crate::bridge::st().borg.c.y, crate::bridge::st().borg.c.x) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISWEAK as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*z_info).pack_size as ::core::ffi::c_int + 12 as ::core::ffi::c_int {
        let mut item: *mut borg_item = borg_items.offset(i as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            if !((*item).timeout == 0) {
                slot = borg_wield_slot(item);
                if !((*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int
                    && (*item).sval as ::core::ffi::c_int != sv_rod_recall
                    && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
                        == 0 as ::core::ffi::c_int)
                {
                    if !((*item).tval as ::core::ffi::c_int == TV_LIGHT as ::core::ffi::c_int
                        && ((*item).sval as ::core::ffi::c_int == sv_light_torch
                            || (*item).sval as ::core::ffi::c_int == sv_light_lantern))
                    {
                        b_i = i;
                        b_slot = slot;
                    }
                }
            }
        }
        i += 1;
    }
    if b_i == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if b_i >= (*z_info).pack_size as ::core::ffi::c_int
        || (*borg_items.offset(b_i as isize)).tval as ::core::ffi::c_int
            == TV_ROD as ::core::ffi::c_int
    {
        borg_note(format(
            b"# Waiting for '%s' to Recharge.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*borg_items.offset(b_i as isize)).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('7' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        return true_0 != 0;
    }
    if b_slot >= (*z_info).pack_size as ::core::ffi::c_int {
        borg_note(b"# Swapping Item for Recharge.\0" as *const u8 as *const ::core::ffi::c_char);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress('w' as i32 as keycode_t);
        borg_keypress(
            *(&raw const all_letters_nohjkl as *const ::core::ffi::c_char).offset(b_i as isize)
                as keycode_t,
        );
        borg_keypress('R' as i32 as keycode_t);
        borg_keypress('7' as i32 as keycode_t);
        borg_keypress('5' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_is_ammo(mut tval: ::core::ffi::c_int) -> bool {
    match tval {
        2 | 3 | 4 => return true_0 != 0,
        _ => return false_0 != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_item_wear() {
    borg_free_item_wear();
    track_worn_num = 0 as int16_t;
    track_worn_size = 10 as int16_t;
    track_worn_time = 0 as int16_t;
    track_worn_name1 = mem_zalloc(
        (track_worn_size as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if (*z_info).equip_slots_max as ::core::ffi::c_int + (*z_info).pack_size as ::core::ffi::c_int
        >= 100 as ::core::ffi::c_int
    {
        msg(
            b"pack size plus equipment slots exceeds 100.  Need a rewrite of best_item code\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        borg_init_failure = true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn borg_free_item_wear() {
    if !track_worn_name1.is_null() {
        mem_free(track_worn_name1 as *mut ::core::ffi::c_void);
        track_worn_name1 = ::core::ptr::null_mut::<uint8_t>();
    }
}
