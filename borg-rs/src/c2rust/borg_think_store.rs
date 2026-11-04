pub const BORG_HOME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ESCAPE: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_h::{borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11, C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16, C2RustUnnamedH_17, C2RustUnnamedH_9, BORG_ALLOW_STRANGE_OPTS, BORG_AUTOSAVE, BORG_CHEAT_DEATH, BORG_CHEST_FAIL_TOLERANCE, BORG_DELAY_FACTOR, BORG_DUMP_LEVEL, BORG_ENCHANT_LIMIT, BORG_KILLS_UNIQUES, BORG_LUNAL_MODE, BORG_MAX_SETTINGS, BORG_MONEY_SCUM_AMOUNT, BORG_MUNCHKIN_DEPTH, BORG_MUNCHKIN_LEVEL, BORG_MUNCHKIN_START, BORG_NO_DEEPER, BORG_PLAYS_RISKY, BORG_RESPAWN_CLASS, BORG_RESPAWN_RACE, BORG_RESPAWN_WINNERS, BORG_RESTORE_IGNORE_SETTINGS, BORG_SAVE_DEATH, BORG_SELF_LUNAL, BORG_SELF_SCUM, BORG_STOP_CLEVEL, BORG_STOP_DLEVEL, BORG_STOP_KING, BORG_STOP_ON_BELL, BORG_USES_DYNAMIC_CALCS, BORG_USES_SWAPS, BORG_VERBOSE, BORG_WORSHIPS_AC, BORG_WORSHIPS_DAMAGE, BORG_WORSHIPS_GOLD, BORG_WORSHIPS_HP, BORG_WORSHIPS_MANA, BORG_WORSHIPS_SPEED};
use crate::headers::borg_io_h::{borg_keypress, borg_note};
pub use crate::headers::borg_item_h::{borg_item, borg_items};
use crate::headers::borg_item_wear_h::{borg_best_stuff, borg_wear_stuff};
use crate::headers::borg_store_buy_h::{
    borg_think_home_buy_swap_armour, borg_think_home_buy_swap_weapon, borg_think_home_buy_useful,
    borg_think_home_grab_useless, borg_think_shop_buy, borg_think_shop_buy_useful,
    borg_think_shop_grab_interesting,
};
pub use crate::headers::borg_store_h::{
    borg_home_full, borg_inventory_full, borg_shop, borg_shops, SHOP_MENU_ITEMS,
};
use crate::headers::borg_store_sell_h::{
    borg_think_home_sell_useful, borg_think_shop_sell, borg_think_shop_sell_useless,
};
use crate::headers::borg_think_h::{borg_do_spell, shop_num};
pub use crate::headers::borg_trait_h::{
    borg_uses_swaps, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW, BI_ACUREPOIS,
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
    cave, chunk, connector, f_info, feature, heatmap, monster_group, square, square_shopnum, trap,
};
pub use crate::headers::h_basic_h::errr;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
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
pub use crate::headers::option_h::player_options;
pub use crate::headers::player_h::{
    class_book, class_magic, class_spell, equip_slot, history_chart, history_entry, history_info,
    magic_realm, player, player_blow, player_body, player_class, player_history, player_race,
    player_shape, player_state, player_upkeep, quest, start_item,
};

pub use crate::headers::stdint_intn_h::{int16_t, int32_t};
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::store_h::{object_buy, owner, store, stores};
pub use crate::headers::target_h::target;
pub use crate::headers::types_h::{__int16_t, __int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::ui_event_h::keycode_t;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_type_h::loc;
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_choose_shop() -> bool {
    let mut best_home_power: int32_t = 0;
    if crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
        != 0
    {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level > 2000 as borg_time {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.antibounce_count as ::core::ffi::c_int > 1350 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if crate::bridge::st().borg.goal.shop as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
        && crate::bridge::st().borg.goal.ware as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0
    {
        crate::bridge::st().borg.goal.shop = 3 as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                >= 2 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.goal.shop = 0 as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 1 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize] >= 100 as ::core::ffi::c_int
    {
        crate::bridge::st().borg.goal.shop = 0 as int16_t;
    }
    if crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_FOOD as ::core::ffi::c_int) as usize] == 0 as ::core::ffi::c_int
        || crate::bridge::st().borg.trait_[(BI_ISCUT as ::core::ffi::c_int) as usize] != 0
        || crate::bridge::st().borg.trait_[(BI_ISPOISONED as ::core::ffi::c_int) as usize]
            != 0
        || crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] == 1 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
                >= 100 as ::core::ffi::c_int
            && crate::bridge::st().borg.trait_[(BI_CLEVEL as ::core::ffi::c_int) as usize]
                < 10 as ::core::ffi::c_int
    {
        if borg_think_shop_buy_useful() {
            borg_note(format(
                b"# Buying '%s' at '%s' immediately\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                    .ware
                    .offset(crate::bridge::st().borg.goal.ware as isize))
                .desc as *mut ::core::ffi::c_char,
                (*f_info.offset((*stores.offset(crate::bridge::st().borg.goal.shop as isize)).feat as isize)).name,
            ));
            return true_0 != 0;
        }
        if borg_think_home_buy_useful() {
            borg_note(format(
                b"# Buying '%s' from the home immediately.\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                    .ware
                    .offset(crate::bridge::st().borg.goal.ware as isize))
                .desc as *mut ::core::ffi::c_char,
            ));
            return true_0 != 0;
        }
    }
    if crate::bridge::st().borg.goal.shop as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
        && crate::bridge::st().borg.goal.ware as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
    {
        return true_0 != 0;
    }
    crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
    crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
    if crate::bridge::st().borg.goal.do_best as ::core::ffi::c_int != 0 && !borg_home_full() && !borg_inventory_full() {
        crate::bridge::st().borg.goal.shop = BORG_HOME as int16_t;
        return true_0 != 0;
    }
    if crate::bridge::st().borg.trait_[(BI_GOLD as ::core::ffi::c_int) as usize]
        < crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
        && crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize]
            != 0 as ::core::ffi::c_int
        && crate::bridge::st().borg.trait_[(BI_CDEPTH as ::core::ffi::c_int) as usize]
            == 0
        && crate::bridge::st().borg.trait_[(BI_LIGHT as ::core::ffi::c_int) as usize] != 0
        && crate::bridge::st().core.cfg[(BORG_SELF_SCUM as ::core::ffi::c_int) as usize] == 0
    {
        if borg_think_shop_buy_useful() {
            borg_note(format(
                b"# Buying '%s' at '%s' (money scumming)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                    .ware
                    .offset(crate::bridge::st().borg.goal.ware as isize))
                .desc as *mut ::core::ffi::c_char,
                (*f_info.offset((*stores.offset(crate::bridge::st().borg.goal.shop as isize)).feat as isize)).name,
            ));
            return true_0 != 0;
        } else {
            return false_0 != 0;
        }
    }
    if borg_think_home_sell_useful(&raw mut best_home_power) {
        if crate::bridge::st().borg.goal.item as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
            borg_note(format(
                b"# Selling '%s' to the home\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*borg_items.offset(crate::bridge::st().borg.goal.item as isize)).desc
                    as *mut ::core::ffi::c_char,
            ));
        } else {
            borg_note(format(
                b"# Buying '%s' from the home (step 1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                    .ware
                    .offset(crate::bridge::st().borg.goal.ware as isize))
                .desc as *mut ::core::ffi::c_char,
            ));
        }
        return true_0 != 0;
    }
    if borg_think_shop_sell_useless() {
        borg_note(format(
            b"# Selling '%s' at '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*borg_items.offset(crate::bridge::st().borg.goal.item as isize)).desc as *mut ::core::ffi::c_char,
            (*f_info.offset((*stores.offset(crate::bridge::st().borg.goal.shop as isize)).feat as isize)).name,
        ));
        return true_0 != 0;
    }
    if borg_think_shop_buy_useful() {
        borg_note(format(
            b"# Buying '%s'(%c) at '%s' (for player 'b')\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
            *SHOP_MENU_ITEMS.offset(crate::bridge::st().borg.goal.ware as isize) as ::core::ffi::c_int,
            (*f_info.offset((*stores.offset(crate::bridge::st().borg.goal.shop as isize)).feat as isize)).name,
        ));
        return true_0 != 0;
    }
    if borg_think_home_buy_useful() {
        borg_note(format(
            b"# Buying '%s' from the home (step 4)\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
        ));
        return true_0 != 0;
    }
    if borg_think_home_grab_useless() {
        borg_note(format(
            b"# Grabbing (to sell) '%s' from the home\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
        ));
        return true_0 != 0;
    }
    if crate::bridge::st().core.cfg[(BORG_MONEY_SCUM_AMOUNT as ::core::ffi::c_int) as usize] != 0 {
        return false_0 != 0;
    }
    if borg_think_shop_grab_interesting() {
        borg_note(format(
            b"# Grabbing (for home) '%s' at '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
            (*f_info.offset((*stores.offset(crate::bridge::st().borg.goal.shop as isize)).feat as isize)).name,
        ));
        return true_0 != 0;
    }
    if borg_uses_swaps() as ::core::ffi::c_int != 0
        && borg_think_home_buy_swap_weapon() as ::core::ffi::c_int != 0
    {
        borg_note(format(
            b"# Buying '%s' from the home as a backup\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
        ));
        return true_0 != 0;
    }
    if borg_uses_swaps() as ::core::ffi::c_int != 0
        && borg_think_home_buy_swap_armour() as ::core::ffi::c_int != 0
    {
        borg_note(format(
            b"# Buying '%s' from the home as a backup\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*borg_shops.offset(crate::bridge::st().borg.goal.shop as isize))
                .ware
                .offset(crate::bridge::st().borg.goal.ware as isize))
            .desc as *mut ::core::ffi::c_char,
        ));
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_think_store() -> bool {
    shop_num = square_shopnum(cave, (*player).grid) as int16_t;
    crate::bridge::st().borg.goal.type_0 = 0 as int16_t;
    borg_do_spell = true_0 != 0;
    if crate::bridge::st().borg.status.cancel {
        return true_0 != 0;
    }
    crate::bridge::st().borg.in_shop = true_0 != 0;
    if crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level >= 20000 as borg_time
        && crate::bridge::st().borg.time.now - crate::bridge::st().borg.time.level <= 20010 as borg_time
    {
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        borg_keypress(ESCAPE as keycode_t);
        return true_0 != 0;
    }
    if borg_best_stuff() {
        return true_0 != 0;
    }
    if (*borg_items.offset((*z_info).pack_size as isize)).tval as ::core::ffi::c_int
        == TV_DIGGING as ::core::ffi::c_int
        && (*borg_items.offset((*z_info).pack_size as isize)).art_idx == 0
        && borg_wear_stuff() as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if borg_choose_shop() {
        if shop_num as ::core::ffi::c_int != crate::bridge::st().borg.goal.shop as ::core::ffi::c_int {
            borg_note(format(
                b"# Currently in store '%d' would prefer '%d'.\0" as *const u8
                    as *const ::core::ffi::c_char,
                shop_num as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                crate::bridge::st().borg.goal.shop as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            ));
        } else {
            borg_note(format(
                b"# Currently in preferred store '%d'.\0" as *const u8
                    as *const ::core::ffi::c_char,
                crate::bridge::st().borg.goal.shop as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            ));
        }
        if borg_think_shop_sell() {
            return true_0 != 0;
        }
        if borg_think_shop_buy() {
            return true_0 != 0;
        }
    }
    crate::bridge::st().borg.goal.item = -(1 as ::core::ffi::c_int) as int16_t;
    crate::bridge::st().borg.goal.ware = crate::bridge::st().borg.goal.item;
    crate::bridge::st().borg.goal.shop = crate::bridge::st().borg.goal.ware;
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    borg_keypress(ESCAPE as keycode_t);
    return true_0 != 0;
}
