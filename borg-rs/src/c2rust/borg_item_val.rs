pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
use crate::headers::borg_init_h::borg_init_failure;
use crate::headers::borg_io_h::borg_note;
pub use crate::headers::init_h::{angband_constants, critical_level, o_critical_level, z_info};
use crate::headers::message_h::msg;
use crate::headers::obj_tval_h::{tval_find_idx, tval_find_name};
use crate::headers::obj_util_h::lookup_sval;
pub use crate::headers::object_h::{
    activation, effect, element_info, flavor, k_info, object_base, object_kind,
};

pub use crate::headers::stdint_intn_h::int16_t;
pub use crate::headers::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use crate::headers::types_h::{__int16_t, __uint16_t, __uint32_t, __uint8_t};
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
use crate::headers::z_form_h::format;
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value};
pub use crate::headers::z_util_h::my_rational;
#[no_mangle]
pub static mut sv_food_apple: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_ration: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_slime_mold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_draught: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_pint: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_sip: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_waybread: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_honey_cake: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_slice: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_food_handful: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_second_sight: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_fast_recovery: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_restoring: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_mana: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_emergency: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_terror: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_stoneskin: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_mush_stoneskin: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_debility: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_sprinting: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_cure_mind: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_mush_purging: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_light_lantern: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_light_torch: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_flask_oil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_flask_oil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_cure_critical: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_cure_serious: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_cure_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_potion_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_star_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_life: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_restore_mana: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_potion_restore_mana: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_cure_poison: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_resist_heat: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_resist_cold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_resist_pois: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_str: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_int: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_wis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_dex: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_con: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_str2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_int2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_wis2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_dex2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_con2: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_all: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_restore_life: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_speed: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_berserk: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_sleep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_slowness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_poison: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_blindness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_confusion: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_heroism: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_boldness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_detect_invis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_enlightenment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_slime_mold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_infravision: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_potion_inc_exp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_identify: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_phase_door: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_teleport: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_word_of_recall: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_enchant_armor: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_enchant_weapon_to_hit: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_enchant_weapon_to_dam: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_star_enchant_weapon: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_star_enchant_armor: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_protection_from_evil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_rune_of_protection: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_teleport_level: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_deep_descent: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_recharging: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_banishment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_mass_banishment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_scroll_mass_banishment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_blessing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_holy_chant: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_holy_prayer: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_detect_invis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_satisfy_hunger: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_mapping: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_acquirement: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_star_acquirement: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_scroll_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_star_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_scroll_star_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_monster_confusion: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_trap_door_destruction: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_scroll_dispel_undead: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_flames: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_ice: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_acid: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_lightning: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_digging: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_speed: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_damage: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_ring_dog: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_amulet_teleportation: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_recall: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_rod_recall: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_detection: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_illumination: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_speed: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_mapping: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_rod_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_fire_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_elec_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_cold_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_acid_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_drain_life: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_fire_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_elec_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_cold_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_acid_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_teleport_other: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_slow_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_sleep_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_rod_curing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_teleportation: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_destruction: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_speed: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_healing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_the_magi: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_power: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_curing: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_holiness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_staff_holiness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_sleep_monsters: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_slow_monsters: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_detect_invis: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_detect_evil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_dispel_evil: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_banishment: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_mapping: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_staff_remove_curse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_light: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_teleport_away: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_stinking_cloud: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_wand_stinking_cloud: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_magic_missile: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_wand_magic_missile: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_annihilation: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kv_wand_annihilation: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_stone_to_mud: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_wonder: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_slow_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_hold_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_fear_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_confuse_monster: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_fire_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_cold_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_acid_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_elec_bolt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_fire_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_cold_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_acid_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_elec_ball: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_dragon_cold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_dragon_fire: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_wand_drain_life: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dagger: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_sling: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_short_bow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_long_bow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_light_xbow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_heavy_xbow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_arrow_seeker: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_arrow_mithril: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_bolt_seeker: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_bolt_mithril: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_set_of_leather_gloves: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_cloak: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_robe: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_iron_crown: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_blue: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_black: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_white: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_red: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_green: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_multihued: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_shining: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_law: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_gold: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_chaos: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_balance: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sv_dragon_power: ::core::ffi::c_int = 0;
unsafe extern "C" fn borg_lookup_sval_fail(
    mut tval: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sval: ::core::ffi::c_int = lookup_sval(tval, name);
    if sval == -(1 as ::core::ffi::c_int) {
        borg_note(format(
            b"**STARTUP FAILURE** sval lookup failure - %s \0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        ));
        borg_init_failure = true_0 != 0;
    }
    return sval;
}
#[no_mangle]
pub unsafe extern "C" fn borg_init_item_val() {
    let mut tval: ::core::ffi::c_int =
        tval_find_idx(b"food\0" as *const u8 as *const ::core::ffi::c_char);
    sv_food_apple =
        borg_lookup_sval_fail(tval, b"Apple\0" as *const u8 as *const ::core::ffi::c_char);
    sv_food_ration = borg_lookup_sval_fail(
        tval,
        b"Ration of Food\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_slime_mold = borg_lookup_sval_fail(
        tval,
        b"Slime Mold\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_draught = borg_lookup_sval_fail(
        tval,
        b"Draught of the Ents\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_pint = borg_lookup_sval_fail(
        tval,
        b"Pint of Fine Wine\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_sip = borg_lookup_sval_fail(
        tval,
        b"Sip of Miruvor\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_waybread = borg_lookup_sval_fail(
        tval,
        b"Piece of Elvish Waybread\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_honey_cake = borg_lookup_sval_fail(
        tval,
        b"Honey-cake\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_slice = borg_lookup_sval_fail(
        tval,
        b"Slice of Meat\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_food_handful = borg_lookup_sval_fail(
        tval,
        b"Handful of Dried Fruits\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"mushroom\0" as *const u8 as *const ::core::ffi::c_char);
    sv_mush_second_sight = borg_lookup_sval_fail(
        tval,
        b"Second Sight\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_fast_recovery = borg_lookup_sval_fail(
        tval,
        b"Fast Recovery\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_restoring =
        borg_lookup_sval_fail(tval, b"Vigor\0" as *const u8 as *const ::core::ffi::c_char);
    sv_mush_mana = borg_lookup_sval_fail(
        tval,
        b"Clear Mind\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_emergency = borg_lookup_sval_fail(
        tval,
        b"Emergency\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_terror =
        borg_lookup_sval_fail(tval, b"Terror\0" as *const u8 as *const ::core::ffi::c_char);
    sv_mush_stoneskin = borg_lookup_sval_fail(
        tval,
        b"Stoneskin\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_mush_stoneskin = borg_lookup_kind(tval, sv_mush_stoneskin);
    sv_mush_debility = borg_lookup_sval_fail(
        tval,
        b"Debility\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_sprinting = borg_lookup_sval_fail(
        tval,
        b"Sprinting\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_cure_mind = borg_lookup_sval_fail(
        tval,
        b"Clear Mind\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_mush_purging = borg_lookup_sval_fail(
        tval,
        b"Purging\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"light\0" as *const u8 as *const ::core::ffi::c_char);
    sv_light_lantern = borg_lookup_sval_fail(
        tval,
        b"Lantern\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_light_torch = borg_lookup_sval_fail(
        tval,
        b"Wooden Torch\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"flask\0" as *const u8 as *const ::core::ffi::c_char);
    sv_flask_oil = borg_lookup_sval_fail(
        tval,
        b"Flask of Oil\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_flask_oil = borg_lookup_kind(tval, sv_flask_oil);
    tval = tval_find_idx(b"potion\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_cure_critical = borg_lookup_sval_fail(
        tval,
        b"Cure Critical Wounds\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_cure_serious = borg_lookup_sval_fail(
        tval,
        b"Cure Serious Wounds\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_cure_light = borg_lookup_sval_fail(
        tval,
        b"Cure Light Wounds\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_healing = borg_lookup_sval_fail(
        tval,
        b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_potion_healing = borg_lookup_kind(tval, sv_potion_healing);
    sv_potion_star_healing = borg_lookup_sval_fail(
        tval,
        b"*Healing*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_life =
        borg_lookup_sval_fail(tval, b"Life\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_restore_mana = borg_lookup_sval_fail(
        tval,
        b"Restore Mana\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_potion_restore_mana = borg_lookup_kind(tval, sv_potion_restore_mana);
    sv_potion_cure_poison = borg_lookup_sval_fail(
        tval,
        b"Neutralize Poison\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_resist_heat = borg_lookup_sval_fail(
        tval,
        b"Resist Heat\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_resist_cold = borg_lookup_sval_fail(
        tval,
        b"Resist Cold\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_resist_pois = borg_lookup_sval_fail(
        tval,
        b"Resist Poison\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_str = borg_lookup_sval_fail(
        tval,
        b"Strength\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_int = borg_lookup_sval_fail(
        tval,
        b"Intelligence\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_wis =
        borg_lookup_sval_fail(tval, b"Wisdom\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_inc_dex = borg_lookup_sval_fail(
        tval,
        b"Dexterity\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_con = borg_lookup_sval_fail(
        tval,
        b"Constitution\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_all = borg_lookup_sval_fail(
        tval,
        b"Augmentation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_str2 =
        borg_lookup_sval_fail(tval, b"Brawn\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_inc_int2 = borg_lookup_sval_fail(
        tval,
        b"Intellect\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_wis2 = borg_lookup_sval_fail(
        tval,
        b"Contemplation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_dex2 = borg_lookup_sval_fail(
        tval,
        b"Nimbleness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_con2 = borg_lookup_sval_fail(
        tval,
        b"Toughness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_restore_life = borg_lookup_sval_fail(
        tval,
        b"Restore Life Levels\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_speed =
        borg_lookup_sval_fail(tval, b"Speed\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_berserk = borg_lookup_sval_fail(
        tval,
        b"Berserk Strength\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_sleep =
        borg_lookup_sval_fail(tval, b"Sleep\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_slowness = borg_lookup_sval_fail(
        tval,
        b"Slowness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_poison =
        borg_lookup_sval_fail(tval, b"Poison\0" as *const u8 as *const ::core::ffi::c_char);
    sv_potion_blindness = borg_lookup_sval_fail(
        tval,
        b"Blindness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_confusion = borg_lookup_sval_fail(
        tval,
        b"Confusion\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_heroism = borg_lookup_sval_fail(
        tval,
        b"Heroism\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_boldness = borg_lookup_sval_fail(
        tval,
        b"Boldness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_detect_invis = borg_lookup_sval_fail(
        tval,
        b"True Seeing\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_enlightenment = borg_lookup_sval_fail(
        tval,
        b"Enlightenment\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_slime_mold = borg_lookup_sval_fail(
        tval,
        b"Slime Mold Juice\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_berserk = borg_lookup_sval_fail(
        tval,
        b"Berserk Strength\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_infravision = borg_lookup_sval_fail(
        tval,
        b"Infravision\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_potion_inc_exp = borg_lookup_sval_fail(
        tval,
        b"Experience\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"scroll\0" as *const u8 as *const ::core::ffi::c_char);
    sv_scroll_identify = borg_lookup_sval_fail(
        tval,
        b"Identify Rune\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_phase_door = borg_lookup_sval_fail(
        tval,
        b"Phase Door\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_teleport = borg_lookup_sval_fail(
        tval,
        b"Teleportation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_word_of_recall = borg_lookup_sval_fail(
        tval,
        b"Word of Recall\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_enchant_armor = borg_lookup_sval_fail(
        tval,
        b"Enchant Armour\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_enchant_weapon_to_hit = borg_lookup_sval_fail(
        tval,
        b"Enchant Weapon To-Hit\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_enchant_weapon_to_dam = borg_lookup_sval_fail(
        tval,
        b"Enchant Weapon To-Dam\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_star_enchant_armor = borg_lookup_sval_fail(
        tval,
        b"*Enchant Armour*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_star_enchant_weapon = borg_lookup_sval_fail(
        tval,
        b"*Enchant Weapon*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_protection_from_evil = borg_lookup_sval_fail(
        tval,
        b"Protection from Evil\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_rune_of_protection = borg_lookup_sval_fail(
        tval,
        b"Rune of Protection\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_teleport_level = borg_lookup_sval_fail(
        tval,
        b"Teleport Level\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_deep_descent = borg_lookup_sval_fail(
        tval,
        b"Deep Descent\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_recharging = borg_lookup_sval_fail(
        tval,
        b"Recharging\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_banishment = borg_lookup_sval_fail(
        tval,
        b"Banishment\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_mass_banishment = borg_lookup_sval_fail(
        tval,
        b"Mass Banishment\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_scroll_mass_banishment = borg_lookup_kind(tval, sv_scroll_mass_banishment);
    sv_scroll_blessing = borg_lookup_sval_fail(
        tval,
        b"Blessing\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_holy_chant = borg_lookup_sval_fail(
        tval,
        b"Holy Chant\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_holy_prayer = borg_lookup_sval_fail(
        tval,
        b"Holy Prayer\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_detect_invis = borg_lookup_sval_fail(
        tval,
        b"Detect Invisible\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_satisfy_hunger = borg_lookup_sval_fail(
        tval,
        b"Remove Hunger\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_light =
        borg_lookup_sval_fail(tval, b"Light\0" as *const u8 as *const ::core::ffi::c_char);
    sv_scroll_mapping = borg_lookup_sval_fail(
        tval,
        b"Magic Mapping\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_acquirement = borg_lookup_sval_fail(
        tval,
        b"Acquirement\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_star_acquirement = borg_lookup_sval_fail(
        tval,
        b"*Acquirement*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_remove_curse = borg_lookup_sval_fail(
        tval,
        b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_scroll_remove_curse = borg_lookup_kind(tval, sv_scroll_remove_curse);
    sv_scroll_star_remove_curse = borg_lookup_sval_fail(
        tval,
        b"*Remove Curse*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_scroll_star_remove_curse = borg_lookup_kind(tval, sv_scroll_star_remove_curse);
    sv_scroll_monster_confusion = borg_lookup_sval_fail(
        tval,
        b"Monster Confusion\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_trap_door_destruction = borg_lookup_sval_fail(
        tval,
        b"Door Destruction\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_scroll_dispel_undead = borg_lookup_sval_fail(
        tval,
        b"Dispel Undead\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"ring\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_flames =
        borg_lookup_sval_fail(tval, b"Flames\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_ice = borg_lookup_sval_fail(tval, b"Ice\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_acid =
        borg_lookup_sval_fail(tval, b"Acid\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_lightning = borg_lookup_sval_fail(
        tval,
        b"Lightning\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_ring_digging = borg_lookup_sval_fail(
        tval,
        b"Digging\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_ring_speed =
        borg_lookup_sval_fail(tval, b"Speed\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_damage =
        borg_lookup_sval_fail(tval, b"Damage\0" as *const u8 as *const ::core::ffi::c_char);
    sv_ring_dog = borg_lookup_sval_fail(
        tval,
        b"the Dog\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"amulet\0" as *const u8 as *const ::core::ffi::c_char);
    sv_amulet_teleportation = borg_lookup_sval_fail(
        tval,
        b"Teleportation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"rod\0" as *const u8 as *const ::core::ffi::c_char);
    sv_rod_recall =
        borg_lookup_sval_fail(tval, b"Recall\0" as *const u8 as *const ::core::ffi::c_char);
    kv_rod_recall = borg_lookup_kind(tval, sv_rod_recall);
    sv_rod_detection = borg_lookup_sval_fail(
        tval,
        b"Detection\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_illumination = borg_lookup_sval_fail(
        tval,
        b"Illumination\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_speed =
        borg_lookup_sval_fail(tval, b"Speed\0" as *const u8 as *const ::core::ffi::c_char);
    sv_rod_mapping = borg_lookup_sval_fail(
        tval,
        b"Magic Mapping\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_healing = borg_lookup_sval_fail(
        tval,
        b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_rod_healing = borg_lookup_kind(tval, sv_rod_healing);
    sv_rod_light =
        borg_lookup_sval_fail(tval, b"Light\0" as *const u8 as *const ::core::ffi::c_char);
    sv_rod_fire_bolt = borg_lookup_sval_fail(
        tval,
        b"Fire Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_elec_bolt = borg_lookup_sval_fail(
        tval,
        b"Lightning Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_cold_bolt = borg_lookup_sval_fail(
        tval,
        b"Frost Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_acid_bolt = borg_lookup_sval_fail(
        tval,
        b"Acid Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_drain_life = borg_lookup_sval_fail(
        tval,
        b"Drain Life\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_fire_ball = borg_lookup_sval_fail(
        tval,
        b"Fire Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_elec_ball = borg_lookup_sval_fail(
        tval,
        b"Lightning Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_cold_ball = borg_lookup_sval_fail(
        tval,
        b"Cold Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_acid_ball = borg_lookup_sval_fail(
        tval,
        b"Acid Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_teleport_other = borg_lookup_sval_fail(
        tval,
        b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_slow_monster = borg_lookup_sval_fail(
        tval,
        b"Slow Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_sleep_monster = borg_lookup_sval_fail(
        tval,
        b"Hold Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_rod_curing =
        borg_lookup_sval_fail(tval, b"Curing\0" as *const u8 as *const ::core::ffi::c_char);
    tval = tval_find_idx(b"staff\0" as *const u8 as *const ::core::ffi::c_char);
    sv_staff_teleportation = borg_lookup_sval_fail(
        tval,
        b"Teleportation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_destruction = borg_lookup_sval_fail(
        tval,
        b"*Destruction*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_speed =
        borg_lookup_sval_fail(tval, b"Speed\0" as *const u8 as *const ::core::ffi::c_char);
    sv_staff_healing = borg_lookup_sval_fail(
        tval,
        b"Healing\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_the_magi = borg_lookup_sval_fail(
        tval,
        b"the Magi\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_power =
        borg_lookup_sval_fail(tval, b"Power\0" as *const u8 as *const ::core::ffi::c_char);
    sv_staff_holiness = borg_lookup_sval_fail(
        tval,
        b"Holiness\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_staff_holiness = borg_lookup_kind(tval, sv_staff_holiness);
    sv_staff_curing =
        borg_lookup_sval_fail(tval, b"Curing\0" as *const u8 as *const ::core::ffi::c_char);
    sv_staff_sleep_monsters = borg_lookup_sval_fail(
        tval,
        b"Sleep Monsters\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_slow_monsters = borg_lookup_sval_fail(
        tval,
        b"Slow Monsters\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_detect_invis = borg_lookup_sval_fail(
        tval,
        b"Detect Invisible\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_detect_evil = borg_lookup_sval_fail(
        tval,
        b"Detect Evil\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_dispel_evil = borg_lookup_sval_fail(
        tval,
        b"Dispel Evil\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_banishment = borg_lookup_sval_fail(
        tval,
        b"Banishment\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_light =
        borg_lookup_sval_fail(tval, b"Light\0" as *const u8 as *const ::core::ffi::c_char);
    sv_staff_mapping = borg_lookup_sval_fail(
        tval,
        b"Mapping\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_staff_remove_curse = borg_lookup_sval_fail(
        tval,
        b"Remove Curse\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"wand\0" as *const u8 as *const ::core::ffi::c_char);
    sv_wand_light =
        borg_lookup_sval_fail(tval, b"Light\0" as *const u8 as *const ::core::ffi::c_char);
    sv_wand_teleport_away = borg_lookup_sval_fail(
        tval,
        b"Teleport Other\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_stinking_cloud = borg_lookup_sval_fail(
        tval,
        b"Stinking Cloud\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_wand_stinking_cloud = borg_lookup_kind(tval, sv_wand_stinking_cloud);
    sv_wand_magic_missile = borg_lookup_sval_fail(
        tval,
        b"Magic Missile\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_wand_magic_missile = borg_lookup_kind(tval, sv_wand_magic_missile);
    sv_wand_annihilation = borg_lookup_sval_fail(
        tval,
        b"Annihilation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    kv_wand_annihilation = borg_lookup_kind(tval, sv_wand_annihilation);
    sv_wand_stone_to_mud = borg_lookup_sval_fail(
        tval,
        b"Stone to Mud\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_wonder =
        borg_lookup_sval_fail(tval, b"Wonder\0" as *const u8 as *const ::core::ffi::c_char);
    sv_wand_hold_monster = borg_lookup_sval_fail(
        tval,
        b"Hold Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_slow_monster = borg_lookup_sval_fail(
        tval,
        b"Slow Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_fear_monster = borg_lookup_sval_fail(
        tval,
        b"Scare Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_confuse_monster = borg_lookup_sval_fail(
        tval,
        b"Scare Monster\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_fire_bolt = borg_lookup_sval_fail(
        tval,
        b"Fire Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_cold_bolt = borg_lookup_sval_fail(
        tval,
        b"Frost Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_acid_bolt = borg_lookup_sval_fail(
        tval,
        b"Acid Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_elec_bolt = borg_lookup_sval_fail(
        tval,
        b"Lightning Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_fire_ball = borg_lookup_sval_fail(
        tval,
        b"Fire Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_cold_ball = borg_lookup_sval_fail(
        tval,
        b"Cold Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_acid_ball = borg_lookup_sval_fail(
        tval,
        b"Acid Balls\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_elec_ball = borg_lookup_sval_fail(
        tval,
        b"Lightning Bolts\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_dragon_cold = borg_lookup_sval_fail(
        tval,
        b"Dragon's Frost\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_dragon_fire = borg_lookup_sval_fail(
        tval,
        b"Dragon's Flame\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_wand_drain_life = borg_lookup_sval_fail(
        tval,
        b"Drain Life\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"sword\0" as *const u8 as *const ::core::ffi::c_char);
    sv_dagger = borg_lookup_sval_fail(tval, b"Dagger\0" as *const u8 as *const ::core::ffi::c_char);
    tval = tval_find_idx(b"bow\0" as *const u8 as *const ::core::ffi::c_char);
    sv_sling = borg_lookup_sval_fail(tval, b"Sling\0" as *const u8 as *const ::core::ffi::c_char);
    sv_short_bow = borg_lookup_sval_fail(
        tval,
        b"Short Bow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_long_bow = borg_lookup_sval_fail(
        tval,
        b"Long Bow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_light_xbow = borg_lookup_sval_fail(
        tval,
        b"Light Crossbow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_heavy_xbow = borg_lookup_sval_fail(
        tval,
        b"Heavy Crossbow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"arrow\0" as *const u8 as *const ::core::ffi::c_char);
    sv_arrow_seeker = borg_lookup_sval_fail(
        tval,
        b"Seeker Arrow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_arrow_mithril = borg_lookup_sval_fail(
        tval,
        b"Mithril Arrow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"bolt\0" as *const u8 as *const ::core::ffi::c_char);
    sv_bolt_seeker = borg_lookup_sval_fail(
        tval,
        b"Seeker Bolt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_bolt_mithril = borg_lookup_sval_fail(
        tval,
        b"Mithril Bolt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"gloves\0" as *const u8 as *const ::core::ffi::c_char);
    sv_set_of_leather_gloves = borg_lookup_sval_fail(
        tval,
        b"Set of Leather Gloves\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"cloak\0" as *const u8 as *const ::core::ffi::c_char);
    sv_cloak = borg_lookup_sval_fail(tval, b"Cloak\0" as *const u8 as *const ::core::ffi::c_char);
    tval = tval_find_idx(b"soft armor\0" as *const u8 as *const ::core::ffi::c_char);
    sv_robe = borg_lookup_sval_fail(tval, b"Robe\0" as *const u8 as *const ::core::ffi::c_char);
    tval = tval_find_idx(b"crown\0" as *const u8 as *const ::core::ffi::c_char);
    sv_iron_crown = borg_lookup_sval_fail(
        tval,
        b"Iron Crown\0" as *const u8 as *const ::core::ffi::c_char,
    );
    tval = tval_find_idx(b"dragon armor\0" as *const u8 as *const ::core::ffi::c_char);
    sv_dragon_blue = borg_lookup_sval_fail(
        tval,
        b"Blue Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_black = borg_lookup_sval_fail(
        tval,
        b"Black Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_white = borg_lookup_sval_fail(
        tval,
        b"White Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_red = borg_lookup_sval_fail(
        tval,
        b"Red Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_green = borg_lookup_sval_fail(
        tval,
        b"Green Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_multihued = borg_lookup_sval_fail(
        tval,
        b"Multi-Hued Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_shining = borg_lookup_sval_fail(
        tval,
        b"Shining Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_law = borg_lookup_sval_fail(
        tval,
        b"Law Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_gold = borg_lookup_sval_fail(
        tval,
        b"Gold Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_chaos = borg_lookup_sval_fail(
        tval,
        b"Chaos Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_balance = borg_lookup_sval_fail(
        tval,
        b"Balance Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sv_dragon_power = borg_lookup_sval_fail(
        tval,
        b"Power Dragon Scale Mail\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn borg_lookup_kind(
    mut tval: ::core::ffi::c_int,
    mut sval: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    k = 1 as ::core::ffi::c_int;
    while k < (*z_info).k_max as ::core::ffi::c_int {
        let mut k_ptr: *mut object_kind = k_info.offset(k as isize) as *mut object_kind;
        if (*k_ptr).tval == tval && (*k_ptr).sval == sval {
            return k;
        }
        k += 1;
    }
    msg(
        b"No object (%s,%d,%d)\0" as *const u8 as *const ::core::ffi::c_char,
        tval_find_name(tval),
        tval,
        sval,
    );
    return 0 as ::core::ffi::c_int;
}
