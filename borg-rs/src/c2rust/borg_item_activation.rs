#[repr(C)]
pub struct dice_s {
    _opaque: [u8; 0],
}

extern "C" {
    fn format(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn my_stricmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut activations: *mut activation;
    static mut borg_init_failure: bool;
    fn borg_note(what: *const ::core::ffi::c_char);
}
pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct activation {
    pub next: *mut activation,
    pub name: *mut ::core::ffi::c_char,
    pub index: ::core::ffi::c_int,
    pub aim: bool,
    pub level: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub effect: *mut effect,
    pub message: *mut ::core::ffi::c_char,
    pub desc: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct effect {
    pub next: *mut effect,
    pub index: uint16_t,
    pub dice: *mut dice_t,
    pub y: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub radius: ::core::ffi::c_int,
    pub other: ::core::ffi::c_int,
    pub msg: *mut ::core::ffi::c_char,
}
pub type dice_t = dice_s;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut act_dragon_power: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_shining: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_balance: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_law: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_chaos: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_gold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_multihued: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_red: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_green: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dragon_blue: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_ring_lightning: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_ring_ice: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_ring_flames: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_ring_acid: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_purging: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_sprinting: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_debility: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_stone: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_terror: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shroom_emergency: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_food_waybread: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_drink_breath: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_staff_holy: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_staff_magi: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_wand_breath: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_wonder: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_berserker: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_starlight2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_starlight: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_polymorph: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_door_dest: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_disable_traps: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_light_line: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_mon_scare: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_sleep_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_mon_confuse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_mon_slow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_confuse2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tele_other: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_stone_to_mud: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_stinking_cloud: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_arrow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_bizarre: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_mana_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_missile: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_drain_life4: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_drain_life3: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_drain_life2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_drain_life1: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_elec_ball2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_elec_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_elec_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_acid_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_acid_bolt3: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_acid_bolt2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_acid_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_ball160: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_ball100: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_ball50: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_ball2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_bolt2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cold_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_ball200: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_ball2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_ball72: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_bolt3: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_bolt2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_fire_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_firebrand: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_rem_fear_pois: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_life: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_rage_bless_resist: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_star_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_sleepii: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dispel_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dispel_undead: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dispel_evil60: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_dispel_evil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_haste2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_haste1: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_haste: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_probing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_clairvoyance: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_illumination: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_loskill: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_losconf: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_lossleep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_losslow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_destruction2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_earthquakes: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_deep_descent: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_recall: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_blessing3: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_blessing2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_blessing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_satisfy: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_protevil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_banishment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_recharge: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_destroy_doors: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_glyph: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_mapping: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_confusing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tele_level: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tele_long: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tele_phase: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_remove_curse2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enchant_armor2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enchant_armor: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enchant_weapon: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enchant_todam: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enchant_tohit: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_detect_objects: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_detect_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_detect_evil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_detect_invis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_detect_treasure: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_pois: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_cold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_fire: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_elec: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_resist_acid: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_shero: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_hero: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_enlightenment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tmd_esp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tmd_sinvis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tmd_infra: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_tmd_free_act: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_st_lev: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_con: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_dex: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_wis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_int: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_str: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_nimbleness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_toughness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_contemplation: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_intellect: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_brawn: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_mana: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_restore_exp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_heal3: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_heal2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_heal1: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_temp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_nonorlybig: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_full2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_full: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_critical: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_serious: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_body: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_mind: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_confusion: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut act_cure_paranoia: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn borg_findact(
    mut act_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut act: *mut activation =
        activations.offset(1 as ::core::ffi::c_int as isize) as *mut activation;
    while !act.is_null() {
        if my_stricmp((*act).name, act_name) == 0 {
            return (*act).index;
        }
        act = (*act).next;
    }
    borg_note(format(
        b"**STARTUP FAILURE** activation lookup failure - %s \0" as *const u8
            as *const ::core::ffi::c_char,
        act_name,
    ));
    borg_init_failure = true_0 != 0;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_item_activation() {
    act_dragon_power = borg_findact(b"DRAGON_POWER\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_shining =
        borg_findact(b"DRAGON_SHINING\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_balance =
        borg_findact(b"DRAGON_BALANCE\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_law = borg_findact(b"DRAGON_LAW\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_chaos = borg_findact(b"DRAGON_CHAOS\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_gold = borg_findact(b"DRAGON_GOLD\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_multihued =
        borg_findact(b"DRAGON_MULTIHUED\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_red = borg_findact(b"DRAGON_RED\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_green = borg_findact(b"DRAGON_GREEN\0" as *const u8 as *const ::core::ffi::c_char);
    act_dragon_blue = borg_findact(b"DRAGON_BLUE\0" as *const u8 as *const ::core::ffi::c_char);
    act_ring_lightning =
        borg_findact(b"RING_LIGHTNING\0" as *const u8 as *const ::core::ffi::c_char);
    act_ring_ice = borg_findact(b"RING_ICE\0" as *const u8 as *const ::core::ffi::c_char);
    act_ring_flames = borg_findact(b"RING_FLAMES\0" as *const u8 as *const ::core::ffi::c_char);
    act_ring_acid = borg_findact(b"RING_ACID\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_purging =
        borg_findact(b"SHROOM_PURGING\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_sprinting =
        borg_findact(b"SHROOM_SPRINTING\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_debility =
        borg_findact(b"SHROOM_DEBILITY\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_stone = borg_findact(b"SHROOM_STONE\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_terror = borg_findact(b"SHROOM_TERROR\0" as *const u8 as *const ::core::ffi::c_char);
    act_shroom_emergency =
        borg_findact(b"SHROOM_EMERGENCY\0" as *const u8 as *const ::core::ffi::c_char);
    act_food_waybread = borg_findact(b"FOOD_WAYBREAD\0" as *const u8 as *const ::core::ffi::c_char);
    act_drink_breath = borg_findact(b"DRINK_BREATH\0" as *const u8 as *const ::core::ffi::c_char);
    act_staff_holy = borg_findact(b"STAFF_HOLY\0" as *const u8 as *const ::core::ffi::c_char);
    act_staff_magi = borg_findact(b"STAFF_MAGI\0" as *const u8 as *const ::core::ffi::c_char);
    act_wand_breath = borg_findact(b"WAND_BREATH\0" as *const u8 as *const ::core::ffi::c_char);
    act_wonder = borg_findact(b"WONDER\0" as *const u8 as *const ::core::ffi::c_char);
    act_berserker = borg_findact(b"BERSERKER\0" as *const u8 as *const ::core::ffi::c_char);
    act_starlight2 = borg_findact(b"STARLIGHT2\0" as *const u8 as *const ::core::ffi::c_char);
    act_starlight = borg_findact(b"STARLIGHT\0" as *const u8 as *const ::core::ffi::c_char);
    act_polymorph = borg_findact(b"POLYMORPH\0" as *const u8 as *const ::core::ffi::c_char);
    act_door_dest = borg_findact(b"DOOR_DEST\0" as *const u8 as *const ::core::ffi::c_char);
    act_disable_traps = borg_findact(b"DISABLE_TRAPS\0" as *const u8 as *const ::core::ffi::c_char);
    act_light_line = borg_findact(b"LIGHT_LINE\0" as *const u8 as *const ::core::ffi::c_char);
    act_mon_scare = borg_findact(b"MON_SCARE\0" as *const u8 as *const ::core::ffi::c_char);
    act_sleep_all = borg_findact(b"SLEEP_ALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_mon_confuse = borg_findact(b"MON_CONFUSE\0" as *const u8 as *const ::core::ffi::c_char);
    act_mon_slow = borg_findact(b"MON_SLOW\0" as *const u8 as *const ::core::ffi::c_char);
    act_confuse2 = borg_findact(b"CONFUSE2\0" as *const u8 as *const ::core::ffi::c_char);
    act_tele_other = borg_findact(b"TELE_OTHER\0" as *const u8 as *const ::core::ffi::c_char);
    act_stone_to_mud = borg_findact(b"STONE_TO_MUD\0" as *const u8 as *const ::core::ffi::c_char);
    act_stinking_cloud =
        borg_findact(b"STINKING_CLOUD\0" as *const u8 as *const ::core::ffi::c_char);
    act_arrow = borg_findact(b"ARROW\0" as *const u8 as *const ::core::ffi::c_char);
    act_bizarre = borg_findact(b"BIZARRE\0" as *const u8 as *const ::core::ffi::c_char);
    act_mana_bolt = borg_findact(b"MANA_BOLT\0" as *const u8 as *const ::core::ffi::c_char);
    act_missile = borg_findact(b"MISSILE\0" as *const u8 as *const ::core::ffi::c_char);
    act_drain_life4 = borg_findact(b"DRAIN_LIFE4\0" as *const u8 as *const ::core::ffi::c_char);
    act_drain_life3 = borg_findact(b"DRAIN_LIFE3\0" as *const u8 as *const ::core::ffi::c_char);
    act_drain_life2 = borg_findact(b"DRAIN_LIFE2\0" as *const u8 as *const ::core::ffi::c_char);
    act_drain_life1 = borg_findact(b"DRAIN_LIFE1\0" as *const u8 as *const ::core::ffi::c_char);
    act_elec_ball2 = borg_findact(b"ELEC_BALL2\0" as *const u8 as *const ::core::ffi::c_char);
    act_elec_ball = borg_findact(b"ELEC_BALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_elec_bolt = borg_findact(b"ELEC_BOLT\0" as *const u8 as *const ::core::ffi::c_char);
    act_acid_ball = borg_findact(b"ACID_BALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_acid_bolt3 = borg_findact(b"ACID_BOLT3\0" as *const u8 as *const ::core::ffi::c_char);
    act_acid_bolt2 = borg_findact(b"ACID_BOLT2\0" as *const u8 as *const ::core::ffi::c_char);
    act_acid_bolt = borg_findact(b"ACID_BOLT\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_ball160 = borg_findact(b"COLD_BALL160\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_ball100 = borg_findact(b"COLD_BALL100\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_ball50 = borg_findact(b"COLD_BALL50\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_ball2 = borg_findact(b"COLD_BALL2\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_bolt2 = borg_findact(b"COLD_BOLT2\0" as *const u8 as *const ::core::ffi::c_char);
    act_cold_bolt = borg_findact(b"COLD_BOLT\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_ball200 = borg_findact(b"FIRE_BALL200\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_ball2 = borg_findact(b"FIRE_BALL2\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_ball = borg_findact(b"FIRE_BALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_ball72 = borg_findact(b"FIRE_BALL72\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_bolt3 = borg_findact(b"FIRE_BOLT3\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_bolt2 = borg_findact(b"FIRE_BOLT2\0" as *const u8 as *const ::core::ffi::c_char);
    act_fire_bolt = borg_findact(b"FIRE_BOLT\0" as *const u8 as *const ::core::ffi::c_char);
    act_firebrand = borg_findact(b"FIREBRAND\0" as *const u8 as *const ::core::ffi::c_char);
    act_rem_fear_pois = borg_findact(b"REM_FEAR_POIS\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_life = borg_findact(b"RESTORE_LIFE\0" as *const u8 as *const ::core::ffi::c_char);
    act_rage_bless_resist =
        borg_findact(b"RAGE_BLESS_RESIST\0" as *const u8 as *const ::core::ffi::c_char);
    act_star_ball = borg_findact(b"STAR_BALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_sleepii = borg_findact(b"SLEEPII\0" as *const u8 as *const ::core::ffi::c_char);
    act_dispel_all = borg_findact(b"DISPEL_ALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_dispel_undead = borg_findact(b"DISPEL_UNDEAD\0" as *const u8 as *const ::core::ffi::c_char);
    act_dispel_evil60 = borg_findact(b"DISPEL_EVIL60\0" as *const u8 as *const ::core::ffi::c_char);
    act_dispel_evil = borg_findact(b"DISPEL_EVIL\0" as *const u8 as *const ::core::ffi::c_char);
    act_haste2 = borg_findact(b"HASTE2\0" as *const u8 as *const ::core::ffi::c_char);
    act_haste1 = borg_findact(b"HASTE1\0" as *const u8 as *const ::core::ffi::c_char);
    act_haste = borg_findact(b"HASTE\0" as *const u8 as *const ::core::ffi::c_char);
    act_probing = borg_findact(b"PROBING\0" as *const u8 as *const ::core::ffi::c_char);
    act_clairvoyance = borg_findact(b"CLAIRVOYANCE\0" as *const u8 as *const ::core::ffi::c_char);
    act_illumination = borg_findact(b"ILLUMINATION\0" as *const u8 as *const ::core::ffi::c_char);
    act_loskill = borg_findact(b"LOSKILL\0" as *const u8 as *const ::core::ffi::c_char);
    act_losconf = borg_findact(b"LOSCONF\0" as *const u8 as *const ::core::ffi::c_char);
    act_lossleep = borg_findact(b"LOSSLEEP\0" as *const u8 as *const ::core::ffi::c_char);
    act_losslow = borg_findact(b"LOSSLOW\0" as *const u8 as *const ::core::ffi::c_char);
    act_destruction2 = borg_findact(b"DESTRUCTION2\0" as *const u8 as *const ::core::ffi::c_char);
    act_earthquakes = borg_findact(b"EARTHQUAKES\0" as *const u8 as *const ::core::ffi::c_char);
    act_deep_descent = borg_findact(b"DEEP_DESCENT\0" as *const u8 as *const ::core::ffi::c_char);
    act_recall = borg_findact(b"RECALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_blessing3 = borg_findact(b"BLESSING3\0" as *const u8 as *const ::core::ffi::c_char);
    act_blessing2 = borg_findact(b"BLESSING2\0" as *const u8 as *const ::core::ffi::c_char);
    act_blessing = borg_findact(b"BLESSING\0" as *const u8 as *const ::core::ffi::c_char);
    act_satisfy = borg_findact(b"SATISFY\0" as *const u8 as *const ::core::ffi::c_char);
    act_protevil = borg_findact(b"PROTEVIL\0" as *const u8 as *const ::core::ffi::c_char);
    act_banishment = borg_findact(b"BANISHMENT\0" as *const u8 as *const ::core::ffi::c_char);
    act_recharge = borg_findact(b"RECHARGE\0" as *const u8 as *const ::core::ffi::c_char);
    act_destroy_doors = borg_findact(b"DESTROY_DOORS\0" as *const u8 as *const ::core::ffi::c_char);
    act_glyph = borg_findact(b"GLYPH\0" as *const u8 as *const ::core::ffi::c_char);
    act_mapping = borg_findact(b"MAPPING\0" as *const u8 as *const ::core::ffi::c_char);
    act_confusing = borg_findact(b"CONFUSING\0" as *const u8 as *const ::core::ffi::c_char);
    act_tele_level = borg_findact(b"TELE_LEVEL\0" as *const u8 as *const ::core::ffi::c_char);
    act_tele_long = borg_findact(b"TELE_LONG\0" as *const u8 as *const ::core::ffi::c_char);
    act_tele_phase = borg_findact(b"TELE_PHASE\0" as *const u8 as *const ::core::ffi::c_char);
    act_light = borg_findact(b"LIGHT\0" as *const u8 as *const ::core::ffi::c_char);
    act_remove_curse2 = borg_findact(b"REMOVE_CURSE2\0" as *const u8 as *const ::core::ffi::c_char);
    act_remove_curse = borg_findact(b"REMOVE_CURSE\0" as *const u8 as *const ::core::ffi::c_char);
    act_enchant_armor2 =
        borg_findact(b"ENCHANT_ARMOR2\0" as *const u8 as *const ::core::ffi::c_char);
    act_enchant_armor = borg_findact(b"ENCHANT_ARMOR\0" as *const u8 as *const ::core::ffi::c_char);
    act_enchant_weapon =
        borg_findact(b"ENCHANT_WEAPON\0" as *const u8 as *const ::core::ffi::c_char);
    act_enchant_todam = borg_findact(b"ENCHANT_TODAM\0" as *const u8 as *const ::core::ffi::c_char);
    act_enchant_tohit = borg_findact(b"ENCHANT_TOHIT\0" as *const u8 as *const ::core::ffi::c_char);
    act_detect_objects =
        borg_findact(b"DETECT_OBJECTS\0" as *const u8 as *const ::core::ffi::c_char);
    act_detect_all = borg_findact(b"DETECT_ALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_detect_evil = borg_findact(b"DETECT_EVIL\0" as *const u8 as *const ::core::ffi::c_char);
    act_detect_invis = borg_findact(b"DETECT_INVIS\0" as *const u8 as *const ::core::ffi::c_char);
    act_detect_treasure =
        borg_findact(b"DETECT_TREASURE\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_all = borg_findact(b"RESIST_ALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_pois = borg_findact(b"RESIST_POIS\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_cold = borg_findact(b"RESIST_COLD\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_fire = borg_findact(b"RESIST_FIRE\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_elec = borg_findact(b"RESIST_ELEC\0" as *const u8 as *const ::core::ffi::c_char);
    act_resist_acid = borg_findact(b"RESIST_ACID\0" as *const u8 as *const ::core::ffi::c_char);
    act_shero = borg_findact(b"SHERO\0" as *const u8 as *const ::core::ffi::c_char);
    act_hero = borg_findact(b"HERO\0" as *const u8 as *const ::core::ffi::c_char);
    act_enlightenment = borg_findact(b"ENLIGHTENMENT\0" as *const u8 as *const ::core::ffi::c_char);
    act_tmd_esp = borg_findact(b"TMD_ESP\0" as *const u8 as *const ::core::ffi::c_char);
    act_tmd_sinvis = borg_findact(b"TMD_SINVIS\0" as *const u8 as *const ::core::ffi::c_char);
    act_tmd_infra = borg_findact(b"TMD_INFRA\0" as *const u8 as *const ::core::ffi::c_char);
    act_tmd_free_act = borg_findact(b"TMD_FREE_ACT\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_st_lev =
        borg_findact(b"RESTORE_ST_LEV\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_all = borg_findact(b"RESTORE_ALL\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_con = borg_findact(b"RESTORE_CON\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_dex = borg_findact(b"RESTORE_DEX\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_wis = borg_findact(b"RESTORE_WIS\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_int = borg_findact(b"RESTORE_INT\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_str = borg_findact(b"RESTORE_STR\0" as *const u8 as *const ::core::ffi::c_char);
    act_nimbleness = borg_findact(b"NIMBLENESS\0" as *const u8 as *const ::core::ffi::c_char);
    act_toughness = borg_findact(b"TOUGHNESS\0" as *const u8 as *const ::core::ffi::c_char);
    act_contemplation = borg_findact(b"CONTEMPLATION\0" as *const u8 as *const ::core::ffi::c_char);
    act_intellect = borg_findact(b"INTELLECT\0" as *const u8 as *const ::core::ffi::c_char);
    act_brawn = borg_findact(b"BRAWN\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_mana = borg_findact(b"RESTORE_MANA\0" as *const u8 as *const ::core::ffi::c_char);
    act_restore_exp = borg_findact(b"RESTORE_EXP\0" as *const u8 as *const ::core::ffi::c_char);
    act_heal3 = borg_findact(b"HEAL3\0" as *const u8 as *const ::core::ffi::c_char);
    act_heal2 = borg_findact(b"HEAL2\0" as *const u8 as *const ::core::ffi::c_char);
    act_heal1 = borg_findact(b"HEAL1\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_temp = borg_findact(b"CURE_TEMP\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_nonorlybig =
        borg_findact(b"CURE_NONORLYBIG\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_full2 = borg_findact(b"CURE_FULL2\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_full = borg_findact(b"CURE_FULL\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_critical = borg_findact(b"CURE_CRITICAL\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_serious = borg_findact(b"CURE_SERIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_light = borg_findact(b"CURE_LIGHT\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_body = borg_findact(b"CURE_BODY\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_mind = borg_findact(b"CURE_MIND\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_confusion =
        borg_findact(b"CURE_CONFUSION\0" as *const u8 as *const ::core::ffi::c_char);
    act_cure_paranoia = borg_findact(b"CURE_PARANOIA\0" as *const u8 as *const ::core::ffi::c_char);
}
