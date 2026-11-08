// Fixture-only structures/callbacks. Selected production bodies are appended.
#![allow(dead_code, unused_imports, unused_assignments, non_snake_case)]
use std::cell::Cell;
const BI_ACCW: usize = 0;
const BI_DEV: usize = 1;
const BI_ISCONFUSED: usize = 2;
const BI_ISBLIND: usize = 3;
const BI_ISFORGET: usize = 4;
const BI_MAXHP: usize = 5;
const BI_CURSP: usize = 6;
const BI_ICOLD: usize = 7;
const BI_RCOLD: usize = 8;
const TV_RING: u32 = 21;
const RAPID_REGENERATION: i32 = 0;
const MONBLOW_COLD: i32 = 1;
type BorgTime = i32;
#[derive(Default)] struct Unique {unique:i32}
#[derive(Default)] struct Temp {regen:bool,res_acid:bool,res_cold:bool}
#[derive(Default)] struct Time {now:i32}
#[derive(Default)] struct Borg {values:[i32;9],temp:Temp,mon:Unique,near:Unique,time:Time,no_rest_prep:i16,avoidance:i16,dont_react:bool}
impl Borg {fn tr(&self,i:usize)->i32{self.values[i]}}
#[derive(Default)] struct ItemUse {when_last_quaff:i32}
struct ItemVal {sv_potion_cure_critical:i32}
#[derive(Default)] struct Traits {simulate:bool}
#[derive(Default)] struct Swap {armour_swap:i32,weapon_swap:i32}
#[derive(Default)] struct Messages {dont_react:bool}
#[derive(Default)] struct Item {iqty:u8,aware:bool,tval:u8,sval:u8,timeout:i16,ident:bool,level:u8}
#[derive(Default)] struct Items {items:Vec<Item>}
struct BorgState {borg:Borg,item_use:ItemUse,item_val:ItemVal,traits:Traits,trait_swap:Swap,msgs:Messages,items:Items,
    available:bool,spell_ok:bool,spell_result:bool,pending:bool,cost:i32,quaffs:i32,casts:i32,notes:i32,keys:Vec<i32>}
impl Default for BorgState {fn default()->Self {Self{borg:Borg::default(),item_use:ItemUse::default(),item_val:ItemVal{sv_potion_cure_critical:7},traits:Traits::default(),trait_swap:Swap::default(),msgs:Messages::default(),items:Items::default(),available:false,spell_ok:false,spell_result:false,pending:false,cost:0,quaffs:0,casts:0,notes:0,keys:Vec::new()}}}
mod engine {use super::*; thread_local!{pub static RNG:Cell<(i32,i32)>=const{Cell::new((0,0))};}
    pub fn randint0(_:i32)->i32{RNG.with(|r|{let (value,count)=r.get();r.set((value,count+1));value-1})}}
mod borg_messages {use super::*;pub fn borg_parse(st:&mut BorgState,_:Option<&[u8]>){if st.pending{st.msgs.dont_react=true;}}}
fn borg_quaff_potion(st:&mut BorgState,_:i32)->bool{st.quaffs+=1;st.available}
fn borg_spell_okay_fail(st:&mut BorgState,_:i32,_:i32)->bool{st.spell_ok}
fn borg_get_spell_power(st:&BorgState,_:i32)->i32{st.cost}
fn borg_spell(st:&mut BorgState,_:i32)->bool{st.casts+=1;st.spell_result}
fn inven_right()->i32{2} fn inven_left()->i32{3} fn inven_wield()->i32{0}
fn key(k:u8)->u32{k as u32}
fn letter(i:i32)->u32{b"abcdefgimnopqrstuvwxyz"[i as usize] as u32}
fn swap_letter(i:i32)->u32{letter(i)}
fn note_desc(st:&mut BorgState,_:&[u8],_:usize,_:&[u8]){st.notes+=1;}
fn borg_note(st:&mut BorgState,_:&str){st.notes+=1;}
fn borg_keypress(st:&mut BorgState,k:u32){st.keys.push(k as i32);}
mod keycode_t {pub fn from(c:u8)->u32{c as u32}}
fn ring(st:&mut BorgState,slot:usize,bits:i32,level:i32){st.items.items[slot]=Item{aware:bits&1!=0,iqty:((bits>>1)&1) as u8,tval:if bits&4!=0{21}else{0},sval:if bits&8!=0{1}else{2},timeout:if bits&16!=0{0}else{1},ident:bits&32!=0,level:level as u8};}
fn emit(st:&BorgState,value:i32){let draws=engine::RNG.with(|r|r.get().1);print!("{} {} {} {} {} {} {}",value,draws,st.quaffs,st.casts,st.notes,st.borg.no_rest_prep,st.keys.len());for k in &st.keys{print!(" {}",k);}println!();}
