pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_DRUID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLASS_PRIEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CLASS_PALADIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLASS_ROGUE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLASS_RANGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_home_notice_h::borg_notice_home;
use crate::headers::borg_home_power_h::borg_power_home;
use crate::headers::borg_inventory_h::{
    borg_first_empty_inventory_slot, borg_slot, borg_wield_slot,
};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
pub use crate::headers::borg_item_h::{borg_item, borg_items, safe_items};

use crate::headers::borg_magic_h::{
    borg_get_book_num, borg_is_dungeon_book, borg_primarily_caster,
};
use crate::headers::borg_power_h::borg_power;
pub use crate::headers::borg_store_h::{borg_safe_shops, borg_shop, borg_shops, SHOP_MENU_ITEMS};
use crate::headers::borg_store_sell_h::{
    borg_min_item_quantity, borg_think_home_sell_useful, sold_item_num, sold_item_store,
    sold_item_sval, sold_item_tval,
};
use crate::headers::borg_think_h::shop_num;
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
use crate::headers::borg_trait_swap_h::{
    armour_swap, armour_swap_value, weapon_swap, weapon_swap_value,
};
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
pub use crate::headers::list_object_flags_h::{
    OF_AFRAID, OF_AGGRAVATE, OF_BLESSED, OF_BURNS_OUT, OF_DIG_1, OF_DIG_2, OF_DIG_3, OF_DRAIN_EXP,
    OF_EXPLODE, OF_FEATHER, OF_FRAGILE, OF_FREE_ACT, OF_HOLD_LIFE, OF_IMPACT, OF_IMPAIR_HP,
    OF_IMPAIR_MANA, OF_LIGHT_2, OF_LIGHT_3, OF_MAX, OF_MULTIPLY_WEIGHT, OF_NO_FUEL, OF_NO_TELEPORT,
    OF_PROT_BLIND, OF_PROT_CONF, OF_PROT_FEAR, OF_PROT_STUN, OF_REGEN, OF_SEE_INVIS,
    OF_SLOW_DIGEST, OF_STICKY, OF_SUST_CON, OF_SUST_DEX, OF_SUST_INT, OF_SUST_STR, OF_SUST_WIS,
    OF_TAKES_FUEL, OF_TELEPATHY, OF_THROWING, OF_TRAP_IMMUNE,
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
pub use crate::headers::obj_properties_h::{C2RustUnnamedH_20, C2RustUnnamedH_31, OF_NONE, TV_MAX};
use crate::headers::obj_util_h::obj_kind_can_browse;
pub use crate::headers::object_h::{
    activation, artifact, curse_data, effect, ego_item, element_info, flavor, k_info, object,
    object_base, object_kind, poss_item,
};
pub use crate::headers::option_h::player_options;
use crate::headers::player_calcs_h::adj_dex_safe;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::headers::string_h::memcpy;
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::{bitflag, flag_has};
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
static mut borg_money_scum_who: ::core::ffi::c_int = 0;
static mut borg_money_scum_ware: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bought_item_tval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut bought_item_sval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut bought_item_pval: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut bought_item_store: [::core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut bought_item_num: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut bought_item_nxt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn borg_good_buy(
    mut item: *mut borg_item,
    mut who: ::core::ffi::c_int,
    mut ware: ::core::ffi::c_int,
) -> bool {
    match (*item).tval as ::core::ffi::c_int {
        2 | 3 | 4 => {
            if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 35 as ::core::ffi::c_int
            {
                if (*item).to_h != 0 {
                    return false_0 != 0;
                }
                if (*item).to_d != 0 {
                    return false_0 != 0;
                }
            }
        }
        31 | 30 | 32 | 33 | 34 => {
            if !obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind) {
                return false_0 != 0;
            }
        }
        _ => {}
    }
    if who == 6 as ::core::ffi::c_int {
        if (*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
            && (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_remove_curse
            && crate::bridge::st().borg.trait_[(BI_FIRST_CURSED as ::core::ffi::c_int) as usize]
                != 0
        {
            return true_0 != 0;
        }
        if (*item).tval as ::core::ffi::c_int == TV_POTION as ::core::ffi::c_int
            && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_star_healing
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_life
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_healing
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_str
                    && crate::bridge::st().borg.trait_[(BI_CSTR as ::core::ffi::c_int) as usize]
                        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_int
                    && crate::bridge::st().borg.trait_[(BI_CINT as ::core::ffi::c_int) as usize]
                        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_wis
                    && crate::bridge::st().borg.trait_[(BI_CWIS as ::core::ffi::c_int) as usize]
                        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_dex
                    && crate::bridge::st().borg.trait_[(BI_CDEX as ::core::ffi::c_int) as usize]
                        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
                || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_potion_inc_con
                    && crate::bridge::st().borg.trait_[(BI_CCON as ::core::ffi::c_int) as usize]
                        < 18 as ::core::ffi::c_int + 100 as ::core::ffi::c_int)
            || (*item).tval as ::core::ffi::c_int == TV_ROD as ::core::ffi::c_int
                && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_healing
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_recall
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_PRIEST
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_PALADIN
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_speed
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_DRUID
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_RANGER
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_teleport_other
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            != CLASS_MAGE
                        && crate::bridge::st().borg.trait_[(BI_CLASS as ::core::ffi::c_int) as usize]
                            == CLASS_ROGUE
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_rod_illumination
                        && crate::bridge::st().borg.trait_[(BI_ALITE as ::core::ffi::c_int) as usize] == 0)
            || obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
                as ::core::ffi::c_int
                != 0
                && crate::bridge::st().borg.amt_book[borg_get_book_num((*item).sval as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && borg_is_dungeon_book(
                    (*item).tval as ::core::ffi::c_int,
                    (*item).sval as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
                    != 0
            || (*item).tval as ::core::ffi::c_int == TV_SCROLL as ::core::ffi::c_int
                && ((*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_teleport_level
                    || (*item).sval as ::core::ffi::c_int == crate::bridge::st().item_val.sv_scroll_teleport)
        {
            if crate::bridge::st().core.cfg[(BORG_SELF_SCUM as ::core::ffi::c_int) as usize] != 0
                && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    >= 10 as ::core::ffi::c_int
                && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0
                && crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                    + crate::bridge::st().home.num_food as ::core::ffi::c_int
                    >= 100 as ::core::ffi::c_int
                && (*item).cost <= 85000 as int32_t
            {
                if adj_dex_safe[crate::bridge::st().borg.trait_[(BI_DEX_INDEX as ::core::ffi::c_int) as usize]
                    as usize]
                    + crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                    > 90 as ::core::ffi::c_int
                {
                    crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] =
                        (*item).cost as ::core::ffi::c_int;
                    borg_money_scum_who = who;
                    borg_money_scum_ware = ware;
                }
            }
            return true_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 15 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                < 20000 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
            < 35 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                < 15000 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
            < 10000 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
    }
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < sold_item_num {
        if sold_item_tval[p as usize] == (*item).tval as ::core::ffi::c_int
            && sold_item_sval[p as usize] == (*item).sval as ::core::ffi::c_int
            && sold_item_store[p as usize] == who
        {
            if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                borg_note(format(
                    b"# Choosing not to buy back %s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*item).desc as *mut ::core::ffi::c_char,
                ));
            }
            return false_0 != 0;
        }
        p += 1;
    }
    if (*item).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*z_info).pack_size as ::core::ffi::c_int {
            let mut item2: *mut borg_item = borg_items.offset(ii as isize) as *mut borg_item;
            if (*item2).tval as ::core::ffi::c_int == TV_DIGGING as ::core::ffi::c_int {
                return false_0 != 0;
            }
            ii += 1;
        }
    }
    if crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
        < 5 as ::core::ffi::c_int
    {
        if obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
            as ::core::ffi::c_int
            != 0
            && (*item).sval as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
    }
    if !borg_primarily_caster()
        && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize]
            <= 8 as ::core::ffi::c_int
    {
        if obj_kind_can_browse(k_info.offset((*item).kind as isize) as *mut object_kind)
            as ::core::ffi::c_int
            != 0
            && (*item).sval as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_buy_useful() -> bool {
    let mut hole: ::core::ffi::c_int = borg_first_empty_inventory_slot();
    let mut slot: ::core::ffi::c_int = 0;
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut c: int32_t = 0;
    let mut b_c: int32_t = 0 as int32_t;
    let mut fix: bool = false_0 != 0;
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.goal.ware as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    b_p = crate::bridge::st().borg.power;
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).store_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        if !((crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize]
            == 0 as ::core::ffi::c_int
            || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                == 0 as ::core::ffi::c_int)
            && k != 0 as ::core::ffi::c_int
            && k != BORG_HOME)
        {
            if !((crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
                || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
                    != 0)
                && k != 3 as ::core::ffi::c_int)
            {
                n = 0 as ::core::ffi::c_int;
                while n < (*z_info).store_inven_max as ::core::ffi::c_int {
                    let mut item: *mut borg_item =
                        (*borg_shops.offset(k as isize)).ware.offset(n as isize) as *mut borg_item;
                    if !((*item).iqty == 0) {
                        if borg_good_buy(item, k, n) {
                            if !(crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
                                != 0
                                && (k != borg_money_scum_who || n != borg_money_scum_ware))
                            {
                                if !((crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                                    as int32_t)
                                    < (*item).cost)
                                {
                                    if !(crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize]
                                        == 0 as ::core::ffi::c_int
                                        && ((*item).tval as ::core::ffi::c_int
                                            != TV_FOOD as ::core::ffi::c_int
                                            && ((*item).tval as ::core::ffi::c_int
                                                != TV_SCROLL as ::core::ffi::c_int
                                                && (*item).sval as ::core::ffi::c_int
                                                    != crate::bridge::st().item_val.sv_scroll_satisfy_hunger)))
                                    {
                                        if !((*item).tval as ::core::ffi::c_int
                                            == TV_WAND as ::core::ffi::c_int
                                            && ((*item).sval as ::core::ffi::c_int
                                                == crate::bridge::st().item_val.sv_wand_magic_missile
                                                || (*item).sval as ::core::ffi::c_int
                                                    == crate::bridge::st().item_val.sv_wand_stinking_cloud)
                                            && crate::bridge::st().borg.trait_[(BI_GOOD_W_CHG as ::core::ffi::c_int) as usize] > 40 as ::core::ffi::c_int)
                                        {
                                            if !((*item).tval as ::core::ffi::c_int
                                                == TV_WAND as ::core::ffi::c_int
                                                && ((*item).sval as ::core::ffi::c_int
                                                    == crate::bridge::st().item_val.sv_wand_magic_missile
                                                    || (*item).sval as ::core::ffi::c_int
                                                        == crate::bridge::st().item_val.sv_wand_stinking_cloud)
                                                && crate::bridge::st().borg.trait_[(BI_MAXCLEVEL as ::core::ffi::c_int) as usize] > 30 as ::core::ffi::c_int)
                                            {
                                                qty = borg_min_item_quantity(item);
                                                let ref mut fresh0 = (*(*borg_shops
                                                    .offset(k as isize))
                                                .ware
                                                .offset(n as isize))
                                                .iqty;
                                                *fresh0 = (*fresh0 as ::core::ffi::c_int - qty)
                                                    as uint8_t;
                                                slot = -(1 as ::core::ffi::c_int);
                                                if (hole + 1 as ::core::ffi::c_int)
                                                    < (*z_info).pack_size as ::core::ffi::c_int
                                                        - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                                                {
                                                    slot = borg_wield_slot(item);
                                                    if (*item).tval as ::core::ffi::c_int
                                                        == TV_LIGHT as ::core::ffi::c_int
                                                        && (*item).sval as ::core::ffi::c_int
                                                            == crate::bridge::st().item_val.sv_light_torch
                                                    {
                                                        if flag_has(
                                                            &raw mut (*borg_items
                                                                .offset(
                                                                    ((*z_info).pack_size as ::core::ffi::c_int
                                                                        + 5 as ::core::ffi::c_int) as isize,
                                                                ))
                                                                .flags as *mut bitflag,
                                                            (OF_MAX as ::core::ffi::c_int as size_t)
                                                                .wrapping_add(
                                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                                        .wrapping_mul(8 as size_t),
                                                                )
                                                                .wrapping_sub(1 as size_t)
                                                                .wrapping_div(
                                                                    (::core::mem::size_of::<bitflag>() as size_t)
                                                                        .wrapping_mul(8 as size_t),
                                                                ),
                                                            OF_BURNS_OUT as ::core::ffi::c_int,
                                                        ) {
                                                            slot = -(1 as ::core::ffi::c_int);
                                                        }
                                                        if (*borg_items.offset(
                                                            ((*z_info).pack_size
                                                                as ::core::ffi::c_int
                                                                + 5 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                        .sval
                                                            as ::core::ffi::c_int
                                                            == crate::bridge::st().item_val.sv_light_lantern
                                                            && flag_has(
                                                                &raw mut (*borg_items.offset(
                                                                    ((*z_info).pack_size
                                                                        as ::core::ffi::c_int
                                                                        + 5 as ::core::ffi::c_int)
                                                                        as isize,
                                                                ))
                                                                .flags
                                                                    as *mut bitflag,
                                                                (OF_MAX as ::core::ffi::c_int
                                                                    as size_t)
                                                                    .wrapping_add(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    )
                                                                    .wrapping_sub(1 as size_t)
                                                                    .wrapping_div(
                                                                        (::core::mem::size_of::<
                                                                            bitflag,
                                                                        >(
                                                                        )
                                                                            as size_t)
                                                                            .wrapping_mul(
                                                                                8 as size_t,
                                                                            ),
                                                                    ),
                                                                OF_TAKES_FUEL as ::core::ffi::c_int,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            slot = -(1 as ::core::ffi::c_int);
                                                        }
                                                    }
                                                    if (*item).tval as ::core::ffi::c_int
                                                        == TV_DIGGING as ::core::ffi::c_int
                                                    {
                                                        slot = -(1 as ::core::ffi::c_int);
                                                    }
                                                }
                                                if slot >= 0 as ::core::ffi::c_int {
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
                                                        (*borg_safe_shops.offset(k as isize))
                                                            .ware
                                                            .offset(n as isize)
                                                            as *mut borg_item
                                                            as *const ::core::ffi::c_void,
                                                        ::core::mem::size_of::<borg_item>()
                                                            as size_t,
                                                    );
                                                    (*borg_items.offset(slot as isize)).iqty =
                                                        qty as uint8_t;
                                                    fix = true_0 != 0;
                                                    borg_notice(true_0 != 0);
                                                    p = borg_power();
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
                                                    (*borg_items.offset(hole as isize)).iqty =
                                                        0 as uint8_t;
                                                } else {
                                                    memcpy(
                                                        borg_items.offset(hole as isize)
                                                            as *mut borg_item
                                                            as *mut ::core::ffi::c_void,
                                                        (*borg_safe_shops.offset(k as isize))
                                                            .ware
                                                            .offset(n as isize)
                                                            as *mut borg_item
                                                            as *const ::core::ffi::c_void,
                                                        ::core::mem::size_of::<borg_item>()
                                                            as size_t,
                                                    );
                                                    (*borg_items.offset(hole as isize)).iqty =
                                                        qty as uint8_t;
                                                    fix = true_0 != 0;
                                                    borg_notice(true_0 != 0);
                                                    p = borg_power();
                                                }
                                                (*borg_items.offset(hole as isize)).iqty =
                                                    0 as uint8_t;
                                                memcpy(
                                                    (*borg_shops.offset(k as isize))
                                                        .ware
                                                        .offset(n as isize)
                                                        as *mut borg_item
                                                        as *mut ::core::ffi::c_void,
                                                    (*borg_safe_shops.offset(k as isize))
                                                        .ware
                                                        .offset(n as isize)
                                                        as *mut borg_item
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<borg_item>() as size_t,
                                                );
                                                c = (*item).cost * qty as int32_t;
                                                if !(p <= b_p) {
                                                    if !(p == b_p && c >= b_c) {
                                                        b_k = k;
                                                        b_n = n;
                                                        b_p = p;
                                                        b_c = c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    n += 1;
                }
            }
        }
        k += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_k >= 0 as ::core::ffi::c_int && b_n >= 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.goal.shop = b_k as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_buy_useful() -> bool {
    let mut hole: ::core::ffi::c_int = 0;
    let mut slot: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut stack: ::core::ffi::c_int = 0;
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut p_left: int32_t = 0 as int32_t;
    let mut p_right: int32_t = 0 as int32_t;
    let mut fix: bool = false_0 != 0;
    let mut skip_it: bool = false_0 != 0;
    b_p = crate::bridge::st().borg.power;
    let mut current_block_71: u64;
    n = 0 as ::core::ffi::c_int;
    while n < (*z_info).store_inven_max as ::core::ffi::c_int {
        let mut item: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            skip_it = false_0 != 0;
            i = 0 as ::core::ffi::c_int;
            while i < sold_item_num {
                if sold_item_tval[i as usize] == (*item).tval as ::core::ffi::c_int
                    && sold_item_sval[i as usize] == (*item).sval as ::core::ffi::c_int
                {
                    if crate::bridge::st().core.cfg[(BORG_VERBOSE as ::core::ffi::c_int) as usize] != 0 {
                        borg_note(format(
                            b"# Choosing not to buy back '%s' from home.\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &raw mut (*item).desc as *mut ::core::ffi::c_char,
                        ));
                    }
                    skip_it = true_0 != 0;
                }
                i += 1;
            }
            if !(skip_it as ::core::ffi::c_int == true_0) {
                hole = borg_first_empty_inventory_slot();
                if !(hole == -(1 as ::core::ffi::c_int)) {
                    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                    {
                        charge_each =
                            (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
                    }
                    qty = borg_min_item_quantity(item);
                    let ref mut fresh1 = (*(*borg_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize))
                    .iqty;
                    *fresh1 = (*fresh1 as ::core::ffi::c_int - qty) as uint8_t;
                    if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                        || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                    {
                        let ref mut fresh2 = (*(*borg_shops.offset(BORG_HOME as isize))
                            .ware
                            .offset(n as isize))
                        .pval;
                        *fresh2 = (*fresh2 as ::core::ffi::c_int - qty * charge_each) as int16_t;
                    }
                    slot = borg_wield_slot(item);
                    stack = borg_slot(
                        (*item).tval as ::core::ffi::c_int,
                        (*item).sval as ::core::ffi::c_int,
                    );
                    p = 0 as ::core::ffi::c_int as int32_t;
                    if slot >= 0 as ::core::ffi::c_int {
                        if hole == -(1 as ::core::ffi::c_int) {
                            current_block_71 = 15427931788582360902;
                        } else if hole + 1 as ::core::ffi::c_int
                            >= (*z_info).pack_size as ::core::ffi::c_int
                                - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                        {
                            current_block_71 = 15427931788582360902;
                        } else {
                            if slot
                                == (*z_info).pack_size as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                            {
                                if !(*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .one_ring
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
                                        (*borg_safe_shops.offset(BORG_HOME as isize))
                                            .ware
                                            .offset(n as isize)
                                            as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    (*borg_items.offset(slot as isize)).iqty = 1 as uint8_t;
                                    fix = true_0 != 0;
                                    borg_notice(true_0 != 0);
                                    p_left = borg_power();
                                    memcpy(
                                        borg_items.offset(slot as isize) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        safe_items.offset(slot as isize) as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                }
                                if !(*borg_items.offset(
                                    ((*z_info).pack_size as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .one_ring
                                {
                                    memcpy(
                                        borg_items.offset(hole as isize) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        safe_items.offset(
                                            ((*z_info).pack_size as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    memcpy(
                                        borg_items.offset(
                                            ((*z_info).pack_size as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        (*borg_safe_shops.offset(BORG_HOME as isize))
                                            .ware
                                            .offset(n as isize)
                                            as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    (*borg_items.offset(
                                        ((*z_info).pack_size as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ))
                                    .iqty = qty as uint8_t;
                                    fix = true_0 != 0;
                                    borg_notice(true_0 != 0);
                                    p_right = borg_power();
                                    memcpy(
                                        borg_items.offset(
                                            ((*z_info).pack_size as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        safe_items.offset(
                                            ((*z_info).pack_size as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                }
                                p = if p_right < p_left { p_left } else { p_right };
                                memcpy(
                                    borg_items.offset(hole as isize) as *mut borg_item
                                        as *mut ::core::ffi::c_void,
                                    safe_items.offset(hole as isize) as *mut borg_item
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<borg_item>() as size_t,
                                );
                            } else {
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
                                    (*borg_safe_shops.offset(BORG_HOME as isize))
                                        .ware
                                        .offset(n as isize)
                                        as *mut borg_item
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<borg_item>() as size_t,
                                );
                                (*borg_items.offset(slot as isize)).iqty = qty as uint8_t;
                                fix = true_0 != 0;
                                borg_notice(true_0 != 0);
                                p = borg_power();
                                memcpy(
                                    borg_items.offset(slot as isize) as *mut borg_item
                                        as *mut ::core::ffi::c_void,
                                    safe_items.offset(slot as isize) as *mut borg_item
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<borg_item>() as size_t,
                                );
                            }
                            current_block_71 = 12497913735442871383;
                        }
                    } else {
                        current_block_71 = 12497913735442871383;
                    }
                    match current_block_71 {
                        15427931788582360902 => {}
                        _ => {
                            if p <= b_p {
                                if slot >= 0 as ::core::ffi::c_int {
                                    (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
                                }
                                if stack != -(1 as ::core::ffi::c_int) {
                                    hole = stack;
                                }
                                if stack == -(1 as ::core::ffi::c_int)
                                    && hole == -(1 as ::core::ffi::c_int)
                                {
                                    current_block_71 = 15427931788582360902;
                                } else if stack == -(1 as ::core::ffi::c_int)
                                    && hole + 1 as ::core::ffi::c_int
                                        >= (*z_info).pack_size as ::core::ffi::c_int
                                            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
                                {
                                    current_block_71 = 15427931788582360902;
                                } else {
                                    memcpy(
                                        borg_items.offset(hole as isize) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        (*borg_safe_shops.offset(BORG_HOME as isize))
                                            .ware
                                            .offset(n as isize)
                                            as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    if stack != -(1 as ::core::ffi::c_int) {
                                        (*borg_items.offset(hole as isize)).iqty =
                                            ((*safe_items.offset(hole as isize)).iqty
                                                as ::core::ffi::c_int
                                                + qty)
                                                as uint8_t;
                                        if (*item).tval as ::core::ffi::c_int
                                            == TV_STAFF as ::core::ffi::c_int
                                            || (*item).tval as ::core::ffi::c_int
                                                == TV_WAND as ::core::ffi::c_int
                                        {
                                            let ref mut fresh3 =
                                                (*borg_items.offset(hole as isize)).pval;
                                            *fresh3 = (*fresh3 as ::core::ffi::c_int
                                                + qty * charge_each)
                                                as int16_t;
                                        }
                                    } else {
                                        (*borg_items.offset(hole as isize)).iqty = qty as uint8_t;
                                        if (*item).tval as ::core::ffi::c_int
                                            == TV_STAFF as ::core::ffi::c_int
                                            || (*item).tval as ::core::ffi::c_int
                                                == TV_WAND as ::core::ffi::c_int
                                        {
                                            (*borg_items.offset(hole as isize)).pval =
                                                (qty * charge_each) as int16_t;
                                        }
                                    }
                                    fix = true_0 != 0;
                                    borg_notice(true_0 != 0);
                                    p = borg_power();
                                    current_block_71 = 13125627826496529465;
                                }
                            } else {
                                current_block_71 = 13125627826496529465;
                            }
                            match current_block_71 {
                                15427931788582360902 => {}
                                _ => {
                                    memcpy(
                                        borg_items.offset(hole as isize) as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        safe_items.offset(hole as isize) as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    memcpy(
                                        (*borg_shops.offset(BORG_HOME as isize))
                                            .ware
                                            .offset(n as isize)
                                            as *mut borg_item
                                            as *mut ::core::ffi::c_void,
                                        (*borg_safe_shops.offset(BORG_HOME as isize))
                                            .ware
                                            .offset(n as isize)
                                            as *mut borg_item
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<borg_item>() as size_t,
                                    );
                                    if !(p <= b_p) {
                                        b_n = n;
                                        b_p = p;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        n += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_n >= 0 as ::core::ffi::c_int && b_p > crate::bridge::st().borg.power {
        crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_grab_interesting() -> bool {
    let mut k: ::core::ffi::c_int = 0;
    let mut b_k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: int32_t = 0 as int32_t;
    let mut b_s: int32_t = 0 as int32_t;
    let mut c: int32_t = 0;
    let mut b_c: int32_t = 0 as int32_t;
    let mut borg_empty_home_power: int32_t = 0;
    let mut hole: ::core::ffi::c_int = 0;
    if crate::bridge::st().borg.trait_[(BI_SAURON_DEAD as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
        < 15 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), true_0 != 0);
    borg_empty_home_power = borg_power_home();
    hole = borg_first_empty_inventory_slot();
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if hole + 1 as ::core::ffi::c_int
        >= (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
    {
        return false_0 != 0;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    b_s = borg_power_home();
    k = 0 as ::core::ffi::c_int;
    while k < (*z_info).store_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
        while n < (*z_info).store_inven_max as ::core::ffi::c_int {
            let mut item: *mut borg_item =
                (*borg_shops.offset(k as isize)).ware.offset(n as isize) as *mut borg_item;
            if !((*item).iqty == 0) {
                if borg_good_buy(item, k, n) {
                    if !((crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_long)
                        < 1000 as ::core::ffi::c_long
                            + ((*item).cost * 5 as int32_t) as ::core::ffi::c_long)
                    {
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            charge_each = (*item).pval as ::core::ffi::c_int
                                / (*item).iqty as ::core::ffi::c_int;
                        }
                        memcpy(
                            borg_items.offset(hole as isize) as *mut borg_item
                                as *mut ::core::ffi::c_void,
                            (*borg_shops.offset(k as isize)).ware.offset(n as isize)
                                as *mut borg_item
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<borg_item>() as size_t,
                        );
                        qty = borg_min_item_quantity(item);
                        (*borg_items.offset(hole as isize)).iqty = qty as uint8_t;
                        if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                            || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                        {
                            (*borg_items.offset(hole as isize)).pval =
                                (charge_each * qty) as int16_t;
                        }
                        borg_notice_home(
                            (*borg_shops.offset(k as isize)).ware.offset(n as isize)
                                as *mut borg_item,
                            false_0 != 0,
                        );
                        if !(borg_empty_home_power >= borg_power_home()) {
                            if borg_think_home_sell_useful(&raw mut s) {
                                c = (*item).cost * qty as int32_t;
                                if c > crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                                    as int32_t
                                    / 10 as int32_t
                                {
                                    s -= c;
                                }
                                if !(s < b_s) {
                                    if !(s == b_s && c >= b_c) {
                                        b_k = k;
                                        b_n = n;
                                        b_s = s;
                                        b_c = c;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            n += 1;
        }
        k += 1;
    }
    (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    s = borg_power_home();
    crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
    crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
    if b_k >= 0 as ::core::ffi::c_int && b_n >= 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.goal.shop = b_k as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_grab_useless() -> bool {
    let mut p: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut s: int32_t = 0;
    let mut b_s: int32_t = 0 as int32_t;
    let mut qty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut charge_each: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skip_it: bool = false_0 != 0;
    let mut hole: ::core::ffi::c_int = borg_first_empty_inventory_slot();
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    if hole + 1 as ::core::ffi::c_int
        >= (*z_info).pack_size as ::core::ffi::c_int
            - crate::bridge::st().borg.trait_[(BI_QUIVER_SLOTS as ::core::ffi::c_int) as usize]
    {
        return false_0 != 0;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    b_s = borg_power_home();
    n = 0 as ::core::ffi::c_int;
    while n < (*z_info).store_inven_max as ::core::ffi::c_int {
        let mut item: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            p = 0 as ::core::ffi::c_int;
            while p < sold_item_num {
                if sold_item_tval[p as usize] == (*item).tval as ::core::ffi::c_int
                    && sold_item_sval[p as usize] == (*item).sval as ::core::ffi::c_int
                    && sold_item_store[p as usize] == BORG_HOME
                {
                    skip_it = true_0 != 0;
                }
                p += 1;
            }
            if !(skip_it as ::core::ffi::c_int == true_0) {
                if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                    || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                {
                    charge_each =
                        (*item).pval as ::core::ffi::c_int / (*item).iqty as ::core::ffi::c_int;
                }
                qty = borg_min_item_quantity(item);
                let ref mut fresh4 = (*(*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(n as isize))
                .iqty;
                *fresh4 = (*fresh4 as ::core::ffi::c_int - qty) as uint8_t;
                if (*item).tval as ::core::ffi::c_int == TV_STAFF as ::core::ffi::c_int
                    || (*item).tval as ::core::ffi::c_int == TV_WAND as ::core::ffi::c_int
                {
                    let ref mut fresh5 = (*(*borg_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize))
                    .pval;
                    *fresh5 = (*fresh5 as ::core::ffi::c_int - charge_each * qty) as int16_t;
                }
                borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
                s = borg_power_home();
                memcpy(
                    (*borg_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize) as *mut borg_item
                        as *mut ::core::ffi::c_void,
                    (*borg_safe_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize) as *mut borg_item
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<borg_item>() as size_t,
                );
                if !(s < b_s) {
                    b_n = n;
                    b_s = s;
                }
            }
        }
        n += 1;
    }
    borg_notice_home(::core::ptr::null_mut::<borg_item>(), false_0 != 0);
    s = borg_power_home();
    if b_n >= 0 as ::core::ffi::c_int {
        crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_buy_swap_weapon() -> bool {
    let mut hole: ::core::ffi::c_int = 0;
    let mut slot: ::core::ffi::c_int = 0;
    let mut old_weapon_swap: ::core::ffi::c_int = 0;
    let mut old_weapon_swap_value: int32_t = 0;
    let mut old_armour_swap: ::core::ffi::c_int = 0;
    let mut old_armour_swap_value: int32_t = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: int32_t = 0 as int32_t;
    let mut b_p: int32_t = 0 as int32_t;
    let mut fix: bool = false_0 != 0;
    old_weapon_swap = weapon_swap;
    old_weapon_swap_value = weapon_swap_value;
    old_armour_swap = armour_swap;
    old_armour_swap_value = armour_swap_value;
    if weapon_swap <= 0 as ::core::ffi::c_int || weapon_swap_value <= 0 as int32_t {
        hole = borg_first_empty_inventory_slot();
        weapon_swap_value = -(1 as ::core::ffi::c_long) as int32_t;
    } else {
        hole = weapon_swap - 1 as ::core::ffi::c_int;
    }
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    b_p = weapon_swap_value;
    n = 0 as ::core::ffi::c_int;
    while n < (*z_info).store_inven_max as ::core::ffi::c_int {
        let mut item: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            slot = borg_wield_slot(item);
            if !(slot != (*z_info).pack_size as ::core::ffi::c_int) {
                let ref mut fresh6 = (*(*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(n as isize))
                .iqty;
                *fresh6 = (*fresh6).wrapping_sub(1);
                if slot == (*z_info).pack_size as ::core::ffi::c_int {
                    memcpy(
                        borg_items.offset(hole as isize) as *mut borg_item
                            as *mut ::core::ffi::c_void,
                        (*borg_safe_shops.offset(BORG_HOME as isize))
                            .ware
                            .offset(n as isize) as *mut borg_item
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<borg_item>() as size_t,
                    );
                    (*borg_items.offset(hole as isize)).iqty = 1 as uint8_t;
                    fix = true_0 != 0;
                    borg_notice(true_0 != 0);
                    p = weapon_swap_value;
                }
                (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
                memcpy(
                    (*borg_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize) as *mut borg_item
                        as *mut ::core::ffi::c_void,
                    (*borg_safe_shops.offset(BORG_HOME as isize))
                        .ware
                        .offset(n as isize) as *mut borg_item
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<borg_item>() as size_t,
                );
                if !(p <= b_p) {
                    b_n = n;
                    b_p = p;
                }
            }
        }
        n += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_n >= 0 as ::core::ffi::c_int && b_p > weapon_swap_value {
        crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        weapon_swap = old_weapon_swap;
        weapon_swap_value = old_weapon_swap_value;
        armour_swap = old_armour_swap;
        armour_swap_value = old_armour_swap_value;
        return true_0 != 0;
    }
    weapon_swap = old_weapon_swap;
    weapon_swap_value = old_weapon_swap_value;
    armour_swap = old_armour_swap;
    armour_swap_value = old_armour_swap_value;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_home_buy_swap_armour() -> bool {
    let mut hole: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: int32_t = 0;
    let mut b_p: int32_t = 0 as int32_t;
    let mut fix: bool = false_0 != 0;
    let mut old_weapon_swap: ::core::ffi::c_int = 0;
    let mut old_weapon_swap_value: int32_t = 0;
    let mut old_armour_swap: ::core::ffi::c_int = 0;
    let mut old_armour_swap_value: int32_t = 0;
    old_weapon_swap = weapon_swap;
    old_weapon_swap_value = weapon_swap_value;
    old_armour_swap = armour_swap;
    old_armour_swap_value = armour_swap_value;
    if armour_swap <= 0 as ::core::ffi::c_int || armour_swap_value <= 0 as int32_t {
        hole = borg_first_empty_inventory_slot();
        armour_swap_value = -(1 as ::core::ffi::c_long) as int32_t;
    } else {
        hole = armour_swap - 1 as ::core::ffi::c_int;
    }
    if hole == -(1 as ::core::ffi::c_int) {
        return false_0 != 0;
    }
    b_p = armour_swap_value;
    n = 0 as ::core::ffi::c_int;
    while n < (*z_info).store_inven_max as ::core::ffi::c_int {
        let mut item: *mut borg_item = (*borg_shops.offset(BORG_HOME as isize))
            .ware
            .offset(n as isize) as *mut borg_item;
        if !((*item).iqty == 0) {
            let ref mut fresh7 = (*(*borg_shops.offset(BORG_HOME as isize))
                .ware
                .offset(n as isize))
            .iqty;
            *fresh7 = (*fresh7).wrapping_sub(1);
            memcpy(
                borg_items.offset(hole as isize) as *mut borg_item as *mut ::core::ffi::c_void,
                (*borg_safe_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(n as isize) as *mut borg_item
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<borg_item>() as size_t,
            );
            (*borg_items.offset(hole as isize)).iqty = 1 as uint8_t;
            fix = true_0 != 0;
            borg_notice(true_0 != 0);
            p = armour_swap_value;
            (*borg_items.offset(hole as isize)).iqty = 0 as uint8_t;
            memcpy(
                (*borg_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(n as isize) as *mut borg_item
                    as *mut ::core::ffi::c_void,
                (*borg_safe_shops.offset(BORG_HOME as isize))
                    .ware
                    .offset(n as isize) as *mut borg_item
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<borg_item>() as size_t,
            );
            if !(p <= b_p) {
                b_n = n;
                b_p = p;
            }
        }
        n += 1;
    }
    if fix {
        borg_notice(true_0 != 0);
    }
    if b_n >= 0 as ::core::ffi::c_int && b_p > armour_swap_value {
        crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
        crate::bridge::st().borg.goal.ware = b_n as int16_t;
        weapon_swap = old_weapon_swap;
        weapon_swap_value = old_weapon_swap_value;
        armour_swap = old_armour_swap;
        armour_swap_value = old_armour_swap_value;
        return true_0 != 0;
    }
    weapon_swap = old_weapon_swap;
    weapon_swap_value = old_weapon_swap_value;
    armour_swap = old_armour_swap;
    armour_swap_value = old_armour_swap_value;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_shop_buy() -> bool {
    let mut purchase_target: ::core::ffi::c_char = '0' as i32 as ::core::ffi::c_char;
    if crate::bridge::st().borg.goal.shop as ::core::ffi::c_int == shop_num as ::core::ffi::c_int
        && crate::bridge::st().borg.goal.ware as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
    {
        let mut shop: *mut borg_shop = borg_shops.offset(crate::bridge::st().borg.goal.shop as isize) as *mut borg_shop;
        let mut item: *mut borg_item =
            (*shop).ware.offset(crate::bridge::st().borg.goal.ware as isize) as *mut borg_item;
        purchase_target = *SHOP_MENU_ITEMS.offset(crate::bridge::st().borg.goal.ware as isize);
        if (*item).tval as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
            crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
            crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
            crate::bridge::st().borg.antibounce_count += 1;
            return false_0 != 0;
        }
        borg_note(format(
            b"# Buying %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*item).desc as *mut ::core::ffi::c_char,
        ));
        borg_keypress(purchase_target as keycode_t);
        borg_keypress('p' as i32 as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        borg_keypress(KC_ENTER as keycode_t);
        if crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] != 0
            && (*item).cost
                >= crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
                    as int32_t
                    * 9 as int32_t
                    / 10 as int32_t
        {
            crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] =
                0 as ::core::ffi::c_int;
            borg_note(format(
                b"# Setting Money Scum to %d.\0" as *const u8 as *const ::core::ffi::c_char,
                crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize],
            ));
        }
        if bought_item_nxt >= 9 as ::core::ffi::c_int {
            bought_item_nxt = 0 as ::core::ffi::c_int;
        }
        bought_item_pval[bought_item_nxt as usize] = (*item).pval as ::core::ffi::c_int;
        bought_item_tval[bought_item_nxt as usize] = (*item).tval as ::core::ffi::c_int;
        bought_item_sval[bought_item_nxt as usize] = (*item).sval as ::core::ffi::c_int;
        bought_item_store[bought_item_nxt as usize] = crate::bridge::st().borg.goal.shop as ::core::ffi::c_int;
        bought_item_num = bought_item_nxt;
        bought_item_nxt += 1;
        crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
        crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
        crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
        crate::bridge::st().borg.antibounce_count += 1;
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        crate::bridge::st().borg.in_shop = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
