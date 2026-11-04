pub const CLASS_MAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLASS_NECROMANCER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BORG_OKAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BORG_VIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
pub use crate::headers::borg_cave_h::{borg_grid, borg_grids};

use crate::headers::borg_danger_h::borg_danger;
pub use crate::headers::borg_fight_attack_h::{
    borg_calculate_attack_effectiveness, C2RustUnnamedH_19, BF_ACT_ACID_BALL, BF_ACT_ACID_BOLT,
    BF_ACT_ACID_BOLT2, BF_ACT_ACID_BOLT3, BF_ACT_ARROW, BF_ACT_COLD_BALL100, BF_ACT_COLD_BALL160,
    BF_ACT_COLD_BALL2, BF_ACT_COLD_BALL50, BF_ACT_COLD_BOLT, BF_ACT_COLD_BOLT2, BF_ACT_CONFUSE2,
    BF_ACT_DISPEL_ALL, BF_ACT_DISPEL_EVIL, BF_ACT_DISPEL_EVIL60, BF_ACT_DISPEL_UNDEAD,
    BF_ACT_DRAGON_BALANCE, BF_ACT_DRAGON_BLUE, BF_ACT_DRAGON_CHAOS, BF_ACT_DRAGON_GOLD,
    BF_ACT_DRAGON_GREEN, BF_ACT_DRAGON_LAW, BF_ACT_DRAGON_MULTIHUED, BF_ACT_DRAGON_POWER,
    BF_ACT_DRAGON_RED, BF_ACT_DRAGON_SHINING, BF_ACT_DRAIN_LIFE1, BF_ACT_DRAIN_LIFE2,
    BF_ACT_DRAIN_LIFE3, BF_ACT_DRAIN_LIFE4, BF_ACT_ELEC_BALL, BF_ACT_ELEC_BALL2, BF_ACT_ELEC_BOLT,
    BF_ACT_FEAR_MONSTER, BF_ACT_FIRE_BALL, BF_ACT_FIRE_BALL2, BF_ACT_FIRE_BALL200,
    BF_ACT_FIRE_BALL72, BF_ACT_FIRE_BOLT, BF_ACT_FIRE_BOLT2, BF_ACT_FIRE_BOLT3, BF_ACT_LIGHT_BEAM,
    BF_ACT_LOSCONF, BF_ACT_LOSSLEEP, BF_ACT_LOSSLOW, BF_ACT_MANA_BOLT, BF_ACT_MISSILE,
    BF_ACT_MON_CONFUSE, BF_ACT_MON_SLOW, BF_ACT_RING_ACID, BF_ACT_RING_FIRE, BF_ACT_RING_ICE,
    BF_ACT_RING_LIGHTNING, BF_ACT_SLEEPII, BF_ACT_SLEEP_ALL, BF_ACT_STAFF_HOLY, BF_ACT_STARLIGHT,
    BF_ACT_STARLIGHT2, BF_ACT_STAR_BALL, BF_ACT_STINKING_CLOUD, BF_ACT_WONDER, BF_DRAGON_BALANCE,
    BF_DRAGON_BLACK, BF_DRAGON_BLUE, BF_DRAGON_CHAOS, BF_DRAGON_GOLD, BF_DRAGON_GREEN,
    BF_DRAGON_LAW, BF_DRAGON_MULTIHUED, BF_DRAGON_POWER, BF_DRAGON_RED, BF_DRAGON_SHINING,
    BF_DRAGON_WHITE, BF_LAUNCH, BF_MAX, BF_OBJECT, BF_PRAYER_DISPEL_LIFE, BF_PRAYER_DISP_EVIL,
    BF_PRAYER_DISP_SPIRITS, BF_PRAYER_DISP_UNDEAD, BF_PRAYER_HOLY_ORB_BALL, BF_PRAYER_HOLY_WORD,
    BF_REST, BF_RING_ACID, BF_RING_FIRE, BF_RING_ICE, BF_RING_LIGHTNING, BF_ROD_ACID_BALL,
    BF_ROD_ACID_BOLT, BF_ROD_COLD_BALL, BF_ROD_COLD_BOLT, BF_ROD_DRAIN_LIFE, BF_ROD_ELEC_BALL,
    BF_ROD_ELEC_BOLT, BF_ROD_FIRE_BALL, BF_ROD_FIRE_BOLT, BF_ROD_LIGHT_BEAM, BF_ROD_SLEEP_MONSTER,
    BF_ROD_SLOW_MONSTER, BF_ROD_UNKNOWN, BF_SPELL_ACID_SPRAY, BF_SPELL_ANNIHILATE,
    BF_SPELL_BLIND_CREATURE, BF_SPELL_COLD_BOLT, BF_SPELL_COLD_STORM, BF_SPELL_CONFUSE_MONSTER,
    BF_SPELL_CRUSH, BF_SPELL_CURSE, BF_SPELL_DARK_SPEAR, BF_SPELL_DISENCHANT,
    BF_SPELL_EARTH_RISING, BF_SPELL_ELECTRIC_ARC, BF_SPELL_EXPLOSION, BF_SPELL_FIRE_BALL,
    BF_SPELL_FRIGHTEN, BF_SPELL_HOWL_OF_THE_DAMNED, BF_SPELL_LEAP_INTO_BATTLE,
    BF_SPELL_LIGHTNING_STRIKE, BF_SPELL_LIGHT_BEAM, BF_SPELL_LIGHT_OF_VARDA,
    BF_SPELL_MAGIC_MISSILE, BF_SPELL_MAGIC_MISSILE_RESERVE, BF_SPELL_MAIM_FOE, BF_SPELL_MANA_BOLT,
    BF_SPELL_MANA_STORM, BF_SPELL_METEOR_SWARM, BF_SPELL_NETHER_BOLT, BF_SPELL_RIFT,
    BF_SPELL_RIVER_OF_LIGHTNING, BF_SPELL_SHOCK_WAVE, BF_SPELL_SLEEP_EVIL, BF_SPELL_SLEEP_III,
    BF_SPELL_SLOW_MONSTER, BF_SPELL_SPEAR_OF_OROME, BF_SPELL_STINK_CLOUD, BF_SPELL_STONE_TO_MUD,
    BF_SPELL_STORM_OF_DARKNESS, BF_SPELL_TAP_UNLIFE, BF_SPELL_THRUST_AWAY, BF_SPELL_TRANCE,
    BF_SPELL_UNLEASH_CHAOS, BF_SPELL_VAMPIRE_STRIKE, BF_SPELL_VOLCANIC_ERUPTION,
    BF_SPELL_WHIRLWIND_ATTACK, BF_STAFF_DISPEL_EVIL, BF_STAFF_HOLINESS, BF_STAFF_POWER,
    BF_STAFF_SLEEP_MONSTERS, BF_STAFF_SLOW_MONSTERS, BF_THRUST, BF_WAND_ACID_BALL,
    BF_WAND_ACID_BOLT, BF_WAND_ANNIHILATION, BF_WAND_COLD_BALL, BF_WAND_COLD_BOLT,
    BF_WAND_CONFUSE_MONSTER, BF_WAND_DRAGON_COLD, BF_WAND_DRAGON_FIRE, BF_WAND_DRAIN_LIFE,
    BF_WAND_ELEC_BALL, BF_WAND_ELEC_BOLT, BF_WAND_FEAR_MONSTER, BF_WAND_FIRE_BALL,
    BF_WAND_FIRE_BOLT, BF_WAND_HOLD_MONSTER, BF_WAND_LIGHT_BEAM, BF_WAND_MAGIC_MISSILE,
    BF_WAND_SLOW_MONSTER, BF_WAND_STINKING_CLOUD, BF_WAND_UNKNOWN, BF_WAND_WONDER,
};
use crate::headers::borg_flow_h::{borg_temp_n, borg_temp_x, borg_temp_y};
pub use crate::headers::borg_flow_kill_h::{borg_kill, borg_kills, borg_kills_cnt, borg_kills_nxt};
pub use crate::headers::borg_h::{
    borg, borg_best, borg_struct, borg_time, C2RustUnnamedH_10, C2RustUnnamedH_11,
    C2RustUnnamedH_12, C2RustUnnamedH_13, C2RustUnnamedH_14, C2RustUnnamedH_15, C2RustUnnamedH_16,
    C2RustUnnamedH_17,
};
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::borg_trait_h::{
    borg_attacking, borg_simulate, C2RustUnnamedH_8, BI_ABRAND, BI_ACCW, BI_ACLW, BI_ACON, BI_ACSW,
    BI_ACUREPOIS, BI_ADETDOOR, BI_ADETEVIL, BI_ADETTRAP, BI_ADEX, BI_ADIGGER, BI_AENCH_ARM,
    BI_AENCH_SARM, BI_AENCH_SWEP, BI_AENCH_TOD, BI_AENCH_TOH, BI_AESCAPE, BI_AEZHEAL, BI_AFUEL,
    BI_AGLYPH, BI_AHEAL, BI_AHWORD, BI_AID, BI_AINT, BI_ALIFE, BI_ALITE, BI_ALL_NEED_ID,
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
    chunk, connector, distance, heatmap, monster_group, square, trap, C2RustUnnamedH_18, FEAT_MAX,
};
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
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
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub unsafe extern "C" fn borg_munchkin_mage() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: loc = loc { x: 0, y: 0 };
    let mut c: loc = loc { x: 0, y: 0 };
    let mut b_dam: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut dam: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        != FEAT_MORE as ::core::ffi::c_int
        && (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            != FEAT_LESS as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    ) > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                a.x = (*kill).pos.x;
                a.y = (*kill).pos.y;
                if !(*borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
                {
                    if (*kill).speed as ::core::ffi::c_int
                        > *borg.trait_0.offset(BI_SPEED as ::core::ffi::c_int as isize)
                        && distance(borg.c, a) <= 2 as ::core::ffi::c_int
                        || distance(borg.c, a) <= 1 as ::core::ffi::c_int
                    {
                        return false_0 != 0;
                    }
                    if borg.mon.scary {
                        return false_0 != 0;
                    }
                    c.x = (*kill).pos.x;
                    c.y = (*kill).pos.y;
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid).offset(c.y as isize))
                        .offset(c.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                            if !(distance(borg.c, c) > (*z_info).max_range as ::core::ffi::c_int) {
                                borg_temp_x[borg_temp_n as usize] = c.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = c.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < BF_MAX as ::core::ffi::c_int {
        if !(i <= 1 as ::core::ffi::c_int) {
            dam = borg_calculate_attack_effectiveness(i);
            if dam >= b_dam && dam > 0 as ::core::ffi::c_int {
                b_dam = dam;
                b_n = i;
            }
        }
        i += 1;
    }
    if b_n < 0 as ::core::ffi::c_int || b_dam <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing munchkin attack with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        b_dam,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(b_n);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn borg_munchkin_melee() -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ag: *mut borg_grid = ::core::ptr::null_mut::<borg_grid>();
    if *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_MAGE
        || *borg.trait_0.offset(BI_CLASS as ::core::ffi::c_int as isize) == CLASS_NECROMANCER
    {
        return false_0 != 0;
    }
    if (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
        != FEAT_MORE as ::core::ffi::c_int
        && (*borg_grids[borg.c.y as usize].offset(borg.c.x as isize)).feat as ::core::ffi::c_int
            != FEAT_LESS as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if borg_kills_cnt == 0 {
        return false_0 != 0;
    }
    if borg_danger(
        borg.c.y,
        borg.c.x,
        1 as ::core::ffi::c_int,
        true_0 != 0,
        true_0 != 0,
    ) > borg.avoidance as ::core::ffi::c_int * 7 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
        || *borg.trait_0.offset(BI_CURHP as ::core::ffi::c_int as isize)
            < *borg.trait_0.offset(BI_MAXHP as ::core::ffi::c_int as isize)
                / 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if *borg
        .trait_0
        .offset(BI_ISCONFUSED as ::core::ffi::c_int as isize)
        != 0
    {
        return false_0 != 0;
    }
    borg_attacking = true_0 != 0;
    borg_temp_n = 0 as int16_t;
    i = 1 as ::core::ffi::c_int;
    while i < borg_kills_nxt as ::core::ffi::c_int {
        let mut kill: *mut borg_kill = ::core::ptr::null_mut::<borg_kill>();
        kill = borg_kills.offset(i as isize) as *mut borg_kill;
        if !((*kill).r_idx == 0) {
            if !(borg.time.now - (*kill).when > 2 as borg_time) {
                if !(*borg
                    .trait_0
                    .offset(BI_CDEPTH as ::core::ffi::c_int as isize)
                    == 0 as ::core::ffi::c_int)
                {
                    if borg.mon.scary {
                        return false_0 != 0;
                    }
                    ag = (*(&raw mut borg_grids as *mut *mut borg_grid)
                        .offset((*kill).pos.y as isize))
                    .offset((*kill).pos.x as isize) as *mut borg_grid;
                    if !((*ag).info as ::core::ffi::c_int & BORG_OKAY == 0) {
                        if !((*ag).info as ::core::ffi::c_int & BORG_VIEW == 0) {
                            if !(distance(borg.c, (*kill).pos) != 1 as ::core::ffi::c_int) {
                                borg_temp_x[borg_temp_n as usize] = (*kill).pos.x as uint8_t;
                                borg_temp_y[borg_temp_n as usize] = (*kill).pos.y as uint8_t;
                                borg_temp_n += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if borg_temp_n == 0 {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_simulate = true_0 != 0;
    n = borg_calculate_attack_effectiveness(BF_THRUST as ::core::ffi::c_int);
    if n <= 0 as ::core::ffi::c_int {
        borg_attacking = false_0 != 0;
        return false_0 != 0;
    }
    borg_note(format(
        b"# Performing munchkin attack with value %d.\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    ));
    borg_simulate = false_0 != 0;
    borg_calculate_attack_effectiveness(BF_THRUST as ::core::ffi::c_int);
    borg_attacking = false_0 != 0;
    return true_0 != 0;
}
