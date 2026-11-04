pub const NOSCORE_BORG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const KC_ENTER: ::core::ffi::c_int = 0x9c as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RACE_HUMAN: ::core::ffi::c_int = 0;
pub const RACE_HALF_ELF: ::core::ffi::c_int = 1;
pub const RACE_ELF: ::core::ffi::c_int = 2;
pub const RACE_HOBBIT: ::core::ffi::c_int = 3;
pub const RACE_GNOME: ::core::ffi::c_int = 4;
pub const RACE_DWARF: ::core::ffi::c_int = 5;
pub const RACE_HALF_ORC: ::core::ffi::c_int = 6;
pub const RACE_HALF_TROLL: ::core::ffi::c_int = 7;
pub const RACE_DUNADAN: ::core::ffi::c_int = 8;
pub const RACE_HIGH_ELF: ::core::ffi::c_int = 9;
pub const RACE_KOBOLD: ::core::ffi::c_int = 10;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::__stddef_wchar_t_h::wchar_t;
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
use crate::headers::borg_init_h::{borg_prepare_race_class_info, borg_reinit_options};
use crate::headers::borg_io_h::{borg_keypress, borg_keypresses, borg_note};
use crate::headers::borg_messages_react_h::borg_clear_reactions;
pub use crate::headers::borg_trait_h::borg_notice_player;
pub use crate::headers::cave_h::{chunk, connector, heatmap, monster_group, square, trap};
pub use crate::headers::h_basic_h::errr;
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
pub use crate::headers::option_h::{option_set, player_options};
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
use crate::headers::ui_menu_h::all_letters;
pub use crate::headers::z_bitflag_h::bitflag;
pub use crate::headers::z_dice_h::{dice_s, dice_t};
pub use crate::headers::z_quark_h::quark_t;
pub use crate::headers::z_rand_h::{random, random_value, Rand_div};
pub use crate::headers::z_type_h::loc;
use crate::headers::z_util_h::{my_strcat, my_strcpy};
static mut dwarf_syllable1: [*const ::core::ffi::c_char; 15] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Th\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut dwarf_syllable2: [*const ::core::ffi::c_char; 6] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut dwarf_syllable3: [*const ::core::ffi::c_char; 16] = [
    b"bur\0" as *const u8 as *const ::core::ffi::c_char,
    b"fur\0" as *const u8 as *const ::core::ffi::c_char,
    b"gan\0" as *const u8 as *const ::core::ffi::c_char,
    b"gnus\0" as *const u8 as *const ::core::ffi::c_char,
    b"gnar\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"lin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lir\0" as *const u8 as *const ::core::ffi::c_char,
    b"mli\0" as *const u8 as *const ::core::ffi::c_char,
    b"nar\0" as *const u8 as *const ::core::ffi::c_char,
    b"nus\0" as *const u8 as *const ::core::ffi::c_char,
    b"rin\0" as *const u8 as *const ::core::ffi::c_char,
    b"ran\0" as *const u8 as *const ::core::ffi::c_char,
    b"sin\0" as *const u8 as *const ::core::ffi::c_char,
    b"sil\0" as *const u8 as *const ::core::ffi::c_char,
    b"sur\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable1: [*const ::core::ffi::c_char; 34] = [
    b"Al\0" as *const u8 as *const ::core::ffi::c_char,
    b"An\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cel\0" as *const u8 as *const ::core::ffi::c_char,
    b"El\0" as *const u8 as *const ::core::ffi::c_char,
    b"Elr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Elv\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ear\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fin\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"Is\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lan\0" as *const u8 as *const ::core::ffi::c_char,
    b"Leg\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lom\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nel\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sel\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tin\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable2: [*const ::core::ffi::c_char; 17] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"adrie\0" as *const u8 as *const ::core::ffi::c_char,
    b"ara\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ebri\0" as *const u8 as *const ::core::ffi::c_char,
    b"ele\0" as *const u8 as *const ::core::ffi::c_char,
    b"ere\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"io\0" as *const u8 as *const ::core::ffi::c_char,
    b"ithra\0" as *const u8 as *const ::core::ffi::c_char,
    b"ilma\0" as *const u8 as *const ::core::ffi::c_char,
    b"il-Ga\0" as *const u8 as *const ::core::ffi::c_char,
    b"ili\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"orfi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut elf_syllable3: [*const ::core::ffi::c_char; 26] = [
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"las\0" as *const u8 as *const ::core::ffi::c_char,
    b"lad\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldor\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldur\0" as *const u8 as *const ::core::ffi::c_char,
    b"linde\0" as *const u8 as *const ::core::ffi::c_char,
    b"lith\0" as *const u8 as *const ::core::ffi::c_char,
    b"mir\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"nd\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndel\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndil\0" as *const u8 as *const ::core::ffi::c_char,
    b"ndir\0" as *const u8 as *const ::core::ffi::c_char,
    b"nduil\0" as *const u8 as *const ::core::ffi::c_char,
    b"ng\0" as *const u8 as *const ::core::ffi::c_char,
    b"mbor\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"rith\0" as *const u8 as *const ::core::ffi::c_char,
    b"ril\0" as *const u8 as *const ::core::ffi::c_char,
    b"riand\0" as *const u8 as *const ::core::ffi::c_char,
    b"rion\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"thien\0" as *const u8 as *const ::core::ffi::c_char,
    b"viel\0" as *const u8 as *const ::core::ffi::c_char,
    b"wen\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyn\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable1: [*const ::core::ffi::c_char; 33] = [
    b"Aar\0" as *const u8 as *const ::core::ffi::c_char,
    b"An\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ar\0" as *const u8 as *const ::core::ffi::c_char,
    b"As\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"Han\0" as *const u8 as *const ::core::ffi::c_char,
    b"Har\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hel\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iir\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jan\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jar\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mar\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nik\0" as *const u8 as *const ::core::ffi::c_char,
    b"Os\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sam\0" as *const u8 as *const ::core::ffi::c_char,
    b"San\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ter\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tom\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ul\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable2: [*const ::core::ffi::c_char; 10] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"aa\0" as *const u8 as *const ::core::ffi::c_char,
    b"ai\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ei\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"uo\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"uu\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut gnome_syllable3: [*const ::core::ffi::c_char; 32] = [
    b"ron\0" as *const u8 as *const ::core::ffi::c_char,
    b"re\0" as *const u8 as *const ::core::ffi::c_char,
    b"la\0" as *const u8 as *const ::core::ffi::c_char,
    b"ki\0" as *const u8 as *const ::core::ffi::c_char,
    b"kseli\0" as *const u8 as *const ::core::ffi::c_char,
    b"ksi\0" as *const u8 as *const ::core::ffi::c_char,
    b"ku\0" as *const u8 as *const ::core::ffi::c_char,
    b"ja\0" as *const u8 as *const ::core::ffi::c_char,
    b"ta\0" as *const u8 as *const ::core::ffi::c_char,
    b"na\0" as *const u8 as *const ::core::ffi::c_char,
    b"namari\0" as *const u8 as *const ::core::ffi::c_char,
    b"neli\0" as *const u8 as *const ::core::ffi::c_char,
    b"nika\0" as *const u8 as *const ::core::ffi::c_char,
    b"nikki\0" as *const u8 as *const ::core::ffi::c_char,
    b"nu\0" as *const u8 as *const ::core::ffi::c_char,
    b"nukka\0" as *const u8 as *const ::core::ffi::c_char,
    b"ka\0" as *const u8 as *const ::core::ffi::c_char,
    b"ko\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"kki\0" as *const u8 as *const ::core::ffi::c_char,
    b"rik\0" as *const u8 as *const ::core::ffi::c_char,
    b"po\0" as *const u8 as *const ::core::ffi::c_char,
    b"to\0" as *const u8 as *const ::core::ffi::c_char,
    b"pekka\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjaana\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjatta\0" as *const u8 as *const ::core::ffi::c_char,
    b"rjukka\0" as *const u8 as *const ::core::ffi::c_char,
    b"la\0" as *const u8 as *const ::core::ffi::c_char,
    b"lla\0" as *const u8 as *const ::core::ffi::c_char,
    b"lli\0" as *const u8 as *const ::core::ffi::c_char,
    b"mo\0" as *const u8 as *const ::core::ffi::c_char,
    b"nni\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable1: [*const ::core::ffi::c_char; 22] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ber\0" as *const u8 as *const ::core::ffi::c_char,
    b"Br\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"Der\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fr\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ler\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mer\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Per\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable2: [*const ::core::ffi::c_char; 7] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"ia\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut hobbit_syllable3: [*const ::core::ffi::c_char; 25] = [
    b"bo\0" as *const u8 as *const ::core::ffi::c_char,
    b"ck\0" as *const u8 as *const ::core::ffi::c_char,
    b"decan\0" as *const u8 as *const ::core::ffi::c_char,
    b"degar\0" as *const u8 as *const ::core::ffi::c_char,
    b"do\0" as *const u8 as *const ::core::ffi::c_char,
    b"doc\0" as *const u8 as *const ::core::ffi::c_char,
    b"go\0" as *const u8 as *const ::core::ffi::c_char,
    b"grin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lba\0" as *const u8 as *const ::core::ffi::c_char,
    b"lbo\0" as *const u8 as *const ::core::ffi::c_char,
    b"lda\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldo\0" as *const u8 as *const ::core::ffi::c_char,
    b"lla\0" as *const u8 as *const ::core::ffi::c_char,
    b"ll\0" as *const u8 as *const ::core::ffi::c_char,
    b"lo\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"mwise\0" as *const u8 as *const ::core::ffi::c_char,
    b"nac\0" as *const u8 as *const ::core::ffi::c_char,
    b"noc\0" as *const u8 as *const ::core::ffi::c_char,
    b"nwise\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"ppin\0" as *const u8 as *const ::core::ffi::c_char,
    b"pper\0" as *const u8 as *const ::core::ffi::c_char,
    b"tho\0" as *const u8 as *const ::core::ffi::c_char,
    b"to\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable1: [*const ::core::ffi::c_char; 65] = [
    b"Ab\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ac\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ad\0" as *const u8 as *const ::core::ffi::c_char,
    b"Af\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ast\0" as *const u8 as *const ::core::ffi::c_char,
    b"As\0" as *const u8 as *const ::core::ffi::c_char,
    b"Al\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ar\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Br\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ch\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cad\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ed\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Et\0" as *const u8 as *const ::core::ffi::c_char,
    b"Er\0" as *const u8 as *const ::core::ffi::c_char,
    b"El\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eow\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fr\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gw\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gl\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ha\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ib\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jer\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ka\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ked\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"Loth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Leg\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mir\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nyd\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oc\0" as *const u8 as *const ::core::ffi::c_char,
    b"On\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rh\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sev\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tr\0" as *const u8 as *const ::core::ffi::c_char,
    b"Th\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"Wic\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable2: [*const ::core::ffi::c_char; 34] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"au\0" as *const u8 as *const ::core::ffi::c_char,
    b"ao\0" as *const u8 as *const ::core::ffi::c_char,
    b"are\0" as *const u8 as *const ::core::ffi::c_char,
    b"ale\0" as *const u8 as *const ::core::ffi::c_char,
    b"ali\0" as *const u8 as *const ::core::ffi::c_char,
    b"ay\0" as *const u8 as *const ::core::ffi::c_char,
    b"ardo\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"ei\0" as *const u8 as *const ::core::ffi::c_char,
    b"ea\0" as *const u8 as *const ::core::ffi::c_char,
    b"eri\0" as *const u8 as *const ::core::ffi::c_char,
    b"era\0" as *const u8 as *const ::core::ffi::c_char,
    b"ela\0" as *const u8 as *const ::core::ffi::c_char,
    b"eli\0" as *const u8 as *const ::core::ffi::c_char,
    b"enda\0" as *const u8 as *const ::core::ffi::c_char,
    b"erra\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"ia\0" as *const u8 as *const ::core::ffi::c_char,
    b"ie\0" as *const u8 as *const ::core::ffi::c_char,
    b"ire\0" as *const u8 as *const ::core::ffi::c_char,
    b"ira\0" as *const u8 as *const ::core::ffi::c_char,
    b"ila\0" as *const u8 as *const ::core::ffi::c_char,
    b"ili\0" as *const u8 as *const ::core::ffi::c_char,
    b"ira\0" as *const u8 as *const ::core::ffi::c_char,
    b"igo\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oa\0" as *const u8 as *const ::core::ffi::c_char,
    b"oi\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"ore\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut human_syllable3: [*const ::core::ffi::c_char; 70] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"and\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"bwyn\0" as *const u8 as *const ::core::ffi::c_char,
    b"baen\0" as *const u8 as *const ::core::ffi::c_char,
    b"bard\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"ctred\0" as *const u8 as *const ::core::ffi::c_char,
    b"cred\0" as *const u8 as *const ::core::ffi::c_char,
    b"ch\0" as *const u8 as *const ::core::ffi::c_char,
    b"can\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"dan\0" as *const u8 as *const ::core::ffi::c_char,
    b"don\0" as *const u8 as *const ::core::ffi::c_char,
    b"der\0" as *const u8 as *const ::core::ffi::c_char,
    b"dric\0" as *const u8 as *const ::core::ffi::c_char,
    b"dfrid\0" as *const u8 as *const ::core::ffi::c_char,
    b"dus\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"gord\0" as *const u8 as *const ::core::ffi::c_char,
    b"gan\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"li\0" as *const u8 as *const ::core::ffi::c_char,
    b"lgrin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lin\0" as *const u8 as *const ::core::ffi::c_char,
    b"lith\0" as *const u8 as *const ::core::ffi::c_char,
    b"lath\0" as *const u8 as *const ::core::ffi::c_char,
    b"loth\0" as *const u8 as *const ::core::ffi::c_char,
    b"ld\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldric\0" as *const u8 as *const ::core::ffi::c_char,
    b"ldan\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"mas\0" as *const u8 as *const ::core::ffi::c_char,
    b"mos\0" as *const u8 as *const ::core::ffi::c_char,
    b"mar\0" as *const u8 as *const ::core::ffi::c_char,
    b"mond\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"nydd\0" as *const u8 as *const ::core::ffi::c_char,
    b"nidd\0" as *const u8 as *const ::core::ffi::c_char,
    b"nnon\0" as *const u8 as *const ::core::ffi::c_char,
    b"nwan\0" as *const u8 as *const ::core::ffi::c_char,
    b"nyth\0" as *const u8 as *const ::core::ffi::c_char,
    b"nad\0" as *const u8 as *const ::core::ffi::c_char,
    b"nn\0" as *const u8 as *const ::core::ffi::c_char,
    b"nnor\0" as *const u8 as *const ::core::ffi::c_char,
    b"nd\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"ron\0" as *const u8 as *const ::core::ffi::c_char,
    b"rd\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"sh\0" as *const u8 as *const ::core::ffi::c_char,
    b"seth\0" as *const u8 as *const ::core::ffi::c_char,
    b"sean\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"th\0" as *const u8 as *const ::core::ffi::c_char,
    b"tha\0" as *const u8 as *const ::core::ffi::c_char,
    b"tlan\0" as *const u8 as *const ::core::ffi::c_char,
    b"trem\0" as *const u8 as *const ::core::ffi::c_char,
    b"tram\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"vudd\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"wan\0" as *const u8 as *const ::core::ffi::c_char,
    b"win\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyn\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyr\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyr\0" as *const u8 as *const ::core::ffi::c_char,
    b"wyth\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable1: [*const ::core::ffi::c_char; 14] = [
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Er\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gr\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pr\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vr\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tr\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dr\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable2: [*const ::core::ffi::c_char; 6] = [
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"oo\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"ui\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut orc_syllable3: [*const ::core::ffi::c_char; 24] = [
    b"dash\0" as *const u8 as *const ::core::ffi::c_char,
    b"dish\0" as *const u8 as *const ::core::ffi::c_char,
    b"dush\0" as *const u8 as *const ::core::ffi::c_char,
    b"gar\0" as *const u8 as *const ::core::ffi::c_char,
    b"gor\0" as *const u8 as *const ::core::ffi::c_char,
    b"gdush\0" as *const u8 as *const ::core::ffi::c_char,
    b"lo\0" as *const u8 as *const ::core::ffi::c_char,
    b"gdish\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"lg\0" as *const u8 as *const ::core::ffi::c_char,
    b"nak\0" as *const u8 as *const ::core::ffi::c_char,
    b"rag\0" as *const u8 as *const ::core::ffi::c_char,
    b"rbag\0" as *const u8 as *const ::core::ffi::c_char,
    b"rg\0" as *const u8 as *const ::core::ffi::c_char,
    b"rk\0" as *const u8 as *const ::core::ffi::c_char,
    b"ng\0" as *const u8 as *const ::core::ffi::c_char,
    b"nk\0" as *const u8 as *const ::core::ffi::c_char,
    b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    b"ol\0" as *const u8 as *const ::core::ffi::c_char,
    b"urk\0" as *const u8 as *const ::core::ffi::c_char,
    b"shnak\0" as *const u8 as *const ::core::ffi::c_char,
    b"mog\0" as *const u8 as *const ::core::ffi::c_char,
    b"mak\0" as *const u8 as *const ::core::ffi::c_char,
    b"rak\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn create_random_name(
    mut race: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
    mut name_len: size_t,
) {
    if name.is_null() {
        return;
    }
    match race {
        RACE_DWARF => {
            my_strcpy(
                name,
                dwarf_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 15]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                dwarf_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                dwarf_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_ELF | RACE_HALF_ELF | RACE_HIGH_ELF => {
            my_strcpy(
                name,
                elf_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 34]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                elf_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 17]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                elf_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 26]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_GNOME => {
            my_strcpy(
                name,
                gnome_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 33]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                gnome_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                gnome_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 32]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HOBBIT => {
            my_strcpy(
                name,
                hobbit_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 22]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                hobbit_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                hobbit_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 25]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HUMAN | RACE_DUNADAN => {
            my_strcpy(
                name,
                human_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 65]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                human_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 34]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                human_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 70]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        RACE_HALF_ORC | RACE_HALF_TROLL | RACE_KOBOLD => {
            my_strcpy(
                name,
                orc_syllable1[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 14]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                orc_syllable2[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
            my_strcat(
                name,
                orc_syllable3[Rand_div(
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 24]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as uint32_t,
                ) as int32_t as usize],
                name_len,
            );
        }
        _ => {
            *name.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn borg_force_reincarnate() {
    borg_keypress('Q' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('@' as i32 as keycode_t);
    borg_reincarnate_start();
}
#[no_mangle]
pub unsafe extern "C" fn borg_reincarnate_end() {
    borg.player = player;
    borg_prepare_race_class_info();
    borg_notice_player();
    borg_clear_reactions();
    borg.status.old_depth = 128 as int16_t;
    if (*player).noscore as ::core::ffi::c_int & NOSCORE_BORG == 0 {
        (*player).noscore = ((*player).noscore as ::core::ffi::c_int | NOSCORE_BORG) as uint16_t;
    }
    borg_reinit_options();
    option_set(
        b"cheat_live\0" as *const u8 as *const ::core::ffi::c_char,
        true_0,
    );
    borg_note(b"# Done respawning\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn borg_reincarnate_start() {
    let mut full_name: [::core::ffi::c_char; 32] = [0; 32];
    let mut race: ::core::ffi::c_uint = 0;
    let mut class: ::core::ffi::c_uint = 0;
    borg.status.respawning = true_0 as int16_t;
    borg_clear_reactions();
    borg_note(b"# Respawning\0" as *const u8 as *const ::core::ffi::c_char);
    borg_keypress('n' as i32 as keycode_t);
    borg_keypress('y' as i32 as keycode_t);
    borg_keypress('n' as i32 as keycode_t);
    if *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
        != -(1 as ::core::ffi::c_int)
    {
        race = *borg_cfg.offset(BORG_RESPAWN_RACE as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uint;
    } else {
        race = Rand_div(11 as uint32_t) as int32_t as ::core::ffi::c_uint;
    }
    if *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
        != -(1 as ::core::ffi::c_int)
    {
        class = *borg_cfg.offset(BORG_RESPAWN_CLASS as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uint;
    } else {
        class = Rand_div(9 as uint32_t) as int32_t as ::core::ffi::c_uint;
    }
    if race != (*(*player).race).ridx {
        borg_keypress(
            *(&raw const all_letters as *const ::core::ffi::c_char).offset(race as isize)
                as keycode_t,
        );
    }
    borg_keypress(KC_ENTER as keycode_t);
    if class != (*(*player).class).cidx {
        borg_keypress(
            *(&raw const all_letters as *const ::core::ffi::c_char).offset(class as isize)
                as keycode_t,
        );
    }
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    create_random_name(
        race as ::core::ffi::c_int,
        &raw mut full_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    borg_keypresses(&raw mut full_name as *mut ::core::ffi::c_char);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
    borg_keypress(KC_ENTER as keycode_t);
}
