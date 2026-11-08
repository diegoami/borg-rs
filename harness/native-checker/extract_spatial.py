"""Select verbatim production items for the standalone native fixture.

Top-level items in these formatted sources end with a column-zero brace.
No algorithm is rewritten; generated items retain their original text.
"""
from pathlib import Path
import hashlib,json,re,sys,os
root,out=Path(sys.argv[1]),Path(sys.argv[2]); ledger=[]
def item(path,needle):
 p=Path(os.environ.get({'borg-rs/src/borg_flow.rs':'BORG_XCHECK_FLOW_SOURCE','borg-rs/src/borg_projection.rs':'BORG_XCHECK_PROJECTION_SOURCE','borg-rs/src/borg_fight_attack.rs':'BORG_XCHECK_COMBAT_SOURCE','borg-rs/src/borg_flow_glyph.rs':'BORG_XCHECK_GLYPH_SOURCE','borg-rs/src/borg_think_dungeon.rs':'BORG_XCHECK_LUNAL_SOURCE'}.get(path,''),str(root/path)));text=p.read_text();starts=[m.start() for m in re.finditer(re.escape(needle),text)]
 assert len(starts)==1,(path,needle)
 start=starts[0];m=re.search(r'^}',text[start:],re.M);assert m,(path,needle)
 body=text[start:start+m.end()]
 ledger.append({'source':str(p),'item':needle,'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'item_sha256':hashlib.sha256(body.encode()).hexdigest()})
 return body
def statement(path,name):
 p=Path(os.environ.get({'borg-rs/src/borg_fight_attack.rs':'BORG_XCHECK_COMBAT_SOURCE','borg-rs/src/borg_flow_glyph.rs':'BORG_XCHECK_GLYPH_SOURCE','borg-rs/src/borg_think_dungeon.rs':'BORG_XCHECK_LUNAL_SOURCE'}.get(path,''),str(root/path)));text=p.read_text()
 matches=re.findall(r'(?m)^(?:pub )?(?:const|type) '+re.escape(name)+r'\b[^\n]*;$',text)
 assert len(matches)==1,(path,name)
 body=matches[0];ledger.append({'source':str(p),'item':name,'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'item_sha256':hashlib.sha256(body.encode()).hexdigest()})
 return body
def array_constant(path,name):
 p=root/path;t=p.read_text();a=t.index('pub const '+name+':');b=t.index(';',t.index('=',a));body=t[a:b+1]
 ledger.append({'source':str(p),'item':name,'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'item_sha256':hashlib.sha256(body.encode()).hexdigest()})
 return body
cave='borg-rs/src/borg_cave.rs';flow='borg-rs/src/borg_flow.rs';proj='borg-rs/src/borg_projection.rs'
preamble=''
terrain=root/'borg-rs/src/headers/list_terrain_h.rs';traits=root/'borg-rs/src/borg_headers/borg_trait_h.rs'
text=preamble+'pub mod cave_h {pub type C2RustUnnamedH_18=u32;} pub mod monster_h {pub type C2RustUnnamedH_29=u32; pub type C2RustUnnamedH_30=u32;} pub mod borg_item_h {#[allow(non_camel_case_types)] pub struct borg_item {pub desc:Vec<i8>,pub dd:u8,pub ds:u8,pub to_d:i16,pub to_h:i16,pub tval:u8,pub iqty:u8}}\n'+f'#[path = "{terrain}"] pub mod terrain;\n#[path = "{traits}"] pub mod traits;\n'
text+=f'#[path = "{root / "borg-rs/src/headers/list_mon_race_flags_h.rs"}"] pub mod race_flags;\nuse race_flags::RF_PASS_WALL;\n'
text+=f'#[path = "{root / "borg-rs/src/headers/list_mon_spells_h.rs"}"] pub mod spell_flags;\n'
text+='pub mod cave { use super::*;\n'+'\n'.join(statement(cave,n) for n in ['DUNGEON_WID','DUNGEON_HGT','AUTO_MAX_X','AUTO_MAX_Y'])+'\n'
text+='#[repr(C)] #[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]\n'+item(cave,'pub struct BorgGrid {')+'\n'
text+='#[derive(Default)]\n'+item(cave,'pub struct CaveState {')+'\n'+item(cave,'impl CaveState {')+'\n'
text+='pub fn allocated()->CaveState { CaveState{grids:vec![BorgGrid::default();(AUTO_MAX_X*AUTO_MAX_Y) as usize]} }\n}\n'
text+='pub mod cave_util {use super::*; use cave::BorgGrid; use terrain::*;\n'
for name in ['feat_is','borg_cave_floor_bold','borg_cave_floor_grid','borg_feature_protected']:
 prefix='fn ' if name=='feat_is' else 'pub fn '
 text+=item('borg-rs/src/borg_cave_util.rs',prefix+name+'(')+'\n'
text+='}\nuse cave_util::{borg_cave_floor_bold,borg_cave_floor_grid};\n'
text+='pub mod projection { use super::*; use cave::BorgGrid; use terrain::*; use traits::*;\n'
text+='#[derive(Default)] pub struct ProjectionState {pub target_loc:loc}\n'
for name in ['feat_is','los_with','borg_los','max_range','fear_region_scary','borg_projectable','borg_offset_projectable','borg_projectable_pure','borg_projectable_dark','borg_inc_motion','borg_distance','borg_target','is_floor','borg_target_unknown_wall']:
 prefix='pub fn ' if name.startswith('borg_') else 'fn '
 text+=item(proj,prefix+name+'(')+'\n'
text+='}\n'
text+='pub mod flow { use super::*; pub fn borg_flow_old(st:&mut BorgState,why:i32)->bool{glyph_callback(st,4,why,0)} use cave::{BorgGrid,AUTO_MAX_X,AUTO_MAX_Y}; use terrain::*; use traits::*;\n'
text+='\n'.join(statement(flow,n) for n in ['AUTO_FLOW_MAX','AUTO_TEMP_MAX','BorgData'])+'\n'
text+=array_constant(flow,'BORG_DDX_DDD')+'\n'+array_constant(flow,'BORG_DDY_DDD')+'\n'+item(flow,'pub fn borg_ddxy_ddd(')+'\n'
text+='#[derive(Default)] pub struct StairTrack{pub num:i16,pub size:i16,pub x:Vec<i32>,pub y:Vec<i32>}\n'
text+=item(flow,'pub struct FlowState {')+'\n'+item(flow,'impl Default for FlowState {')+'\n'+item(flow,'impl FlowState {')+'\n'
for name in ['copy_data','zero_data','borg_flow_clear','flow_fear','borg_flow_ignores_danger','monster_adjacent','spread_blocked','borg_flow_spread','borg_flow_enqueue_grid','borg_flow_commit','flow_commit']:
 prefix='fn ' if name in ['flow_fear','borg_flow_ignores_danger','monster_adjacent','spread_blocked'] else 'pub fn '
 text+=item(flow,prefix+name+'(')+'\n'
text+='}\n'
text+='pub mod directions {use super::*; use flow::{borg_flow_clear,borg_flow_enqueue_grid,borg_flow_spread};\n'+item('borg-rs/src/borg_flow_misc.rs','pub fn borg_extract_dir(')+'\n'+item('borg-rs/src/borg_flow_misc.rs','pub fn borg_goto_dir(')+'\n'+item('borg-rs/src/borg_flow_misc.rs','pub fn borg_flow_reverse(')+'\n}\nuse directions::borg_extract_dir;\n'
text+='pub mod combat {use super::*; use cave::BorgGrid; use traits::*; use projection::borg_distance; use borg_item_h::borg_item; type keycode_t=u32; use race_flags::*; use spell_flags::*;\n'
text+=statement('borg-rs/src/borg_fight_attack.rs','CLASS_MAGE')+'\n'+statement('borg-rs/src/borg_fight_attack.rs','CLASS_NECROMANCER')+'\n'
for name in ['key','i2d','tr','avoid','simulating','grid','is_ghost','rf','rsf','is_summoner','desc','danger_one_here','power','melee_score','borg_attack_aux_thrust']:
 body=item('borg-rs/src/borg_fight_attack.rs','fn '+name+'(')
 if name=='borg_attack_aux_thrust':body='pub '+body
 text+=body+'\n'
body=item('borg-rs/src/borg_fight_attack.rs','fn borg_thrust_damage_one(').replace('fn borg_thrust_damage_one(','pub fn physical_damage_one(',1)
text+=body+'\n'
text+='}\n'
text+='pub mod glyph {use super::*;use flow::StairTrack;use cave::{AUTO_MAX_X,AUTO_MAX_Y};use crate::{flow as borg_flow,glyph_environment as borg_flow_misc};use terrain::FEAT_FLOOR;type borg_spells=u32;\n'
text+=statement('borg-rs/src/borg_flow_glyph.rs','GOAL_MISC')+'\n'+statement('borg-rs/src/borg_headers/borg_magic_h.rs','GLYPH_OF_WARDING')+'\n'
text+='#[derive(Default)]\n'+item('borg-rs/src/borg_flow_glyph.rs','pub struct GlyphState {')+'\n'
for name in ['ring','track_new_glyph','borg_flow_glyph']:
 text+=item('borg-rs/src/borg_flow_glyph.rs',('pub fn ' if name.startswith('borg_') else 'fn ')+name+'(')+'\n'
text+='}\n'
text+='pub mod lunal {use super::*;use traits::*;use terrain::*;use projection::borg_projectable;type C2RustUnnamedH_9=u32;type C2RustUnnamedH_21=u32;\n'
for path,name in [('borg-rs/src/borg_think_dungeon.rs','GOAL_TAKE'),('borg-rs/src/borg_think_dungeon.rs','GOAL_FLEE'),('borg-rs/src/borg_headers/borg_h.rs','BORG_SELF_LUNAL'),('borg-rs/src/headers/list_options_h.rs','OPT_birth_force_descend')]:text+=statement(path,name)+'\n'
text+='#[derive(Clone,Copy,Debug,PartialEq,Eq)] #[allow(clippy::enum_variant_names)]\n'+item('borg-rs/src/borg_light.rs','pub enum BorgNeed {')+'\n'
text+='#[derive(Clone,Copy)]\n'+item('borg-rs/src/borg_think_dungeon.rs','enum Stairs {')+'\n'
for name in ['inven_light','pack_slot_iqty','feat_at','force_descend','prepared_null','stairs','stairs_mut','track_stairs_under','track_stair','nearest_stair','borg_think_dungeon_lunal']:
 body=item('borg-rs/src/borg_think_dungeon.rs','fn '+name+'(')
 if name=='borg_think_dungeon_lunal':body='pub '+body
 text+=body+'\n'
text+='}\n'
(out/'distance.rs').write_text(item('borg-rs/src/engine.rs','pub fn distance(')+'\n')
(out/'spatial_items.rs').write_text(text)
cpath=root/'angband/src/borg/borg-fight-attack.c';ctext=cpath.read_text();a=ctext.index('static int borg_attack_aux_thrust(void)');b=ctext.index('/* adapted from player_attack.c',a)
a_damage=ctext.index('static int borg_thrust_damage_one(int i)');b_damage=ctext.index('/*\n * Simulate/Apply',a_damage)
(out/'physical_damage.inc').write_text(ctext[a_damage:b_damage].replace('borg_thrust_damage_one(', 'xc_physical_damage_one(',1))
(out/'thrust.inc').write_text(ctext[:ctext.index('#include')]+ctext[a:b])
ledger.append({'source':str(cpath),'item':'borg_thrust_damage_one','source_sha256':hashlib.sha256(cpath.read_bytes()).hexdigest(),'item_sha256':hashlib.sha256(ctext[a_damage:b_damage].encode()).hexdigest()})
ledger.append({'source':str(cpath),'item':'borg_attack_aux_thrust','source_sha256':hashlib.sha256(cpath.read_bytes()).hexdigest(),'item_sha256':hashlib.sha256(ctext[a:b].encode()).hexdigest()})
cpath=root/'angband/src/borg/borg-think-dungeon.c';ctext=cpath.read_text();a=ctext.index('static bool borg_think_dungeon_lunal(void)');m=re.search(r'^}',ctext[a:],re.M);assert m;b=a+m.end()
(out/'lunal.inc').write_text(ctext[:ctext.index('#include')]+ctext[a:b])
ledger.append({'source':str(cpath),'item':'borg_think_dungeon_lunal','source_sha256':hashlib.sha256(ctext.encode()).hexdigest(),'item_sha256':hashlib.sha256(ctext[a:b].encode()).hexdigest()})
(out/'spatial-source-ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
