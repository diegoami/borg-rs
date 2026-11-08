#!/usr/bin/env python3
"""Differential production-body/seam checks for the seven M2 corrections.

The full native49 engine scenarios are separate. Here the same controlled
callbacks feed actual selected C/Rust code: three complete item-use functions,
regeneration plus helpers, swap selection tail, cold-blow arm, message batch
reset seam and unseen-name guard. No claim of full private engine shadowing.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--fault', choices=['swap','reset','regen','cold','ring-check','ring-activate','potion','potion-sentinel','name'])
    parser.add_argument('--ubsan', action='store_true')
    args = parser.parse_args()
    out = args.output.resolve();out.mkdir(parents=True, exist_ok=False)
    ledger = []
    def select(path, start, end=None):
        source=ROOT/path;text=source.read_text();assert text.count(start)==1,(path,start)
        first=text.index(start)
        if end is None:
            last=re.search(r'^}',text[first:],re.M).end()+first
        else:
            last=text.index(end,first)
        selected=text[first:last]
        ledger.append({'source':path,'source_sha256':hashlib.sha256(text.encode()).hexdigest(),
                       'start':start,'end':end,'selected_sha256':hashlib.sha256(selected.encode()).hexdigest()})
        return selected
    cuse='angband/src/borg/borg-item-use.c';ruse='borg-rs/src/borg_item_use.rs'
    c=(HERE/'kernel-c.h').read_text()+'\n'
    rust=(HERE/'kernel-rust.rs').read_text()+'\n'
    for name in ('borg_quaff_crit','borg_equips_ring','borg_activate_ring'):
        c+=select(cuse,'bool '+name+'(')+'\n'
        body=select(ruse,'pub fn '+name+'(')
        if args.fault=='potion' and name=='borg_quaff_crit':
            body=body.replace('last != 0 && elapsed <= 4 && elapsed > 0','elapsed > 4')
        if args.fault=='potion-sentinel' and name=='borg_quaff_crit':
            body=body.replace('last != 0 && elapsed <= 4 && elapsed > 0','elapsed <= 4 && elapsed > 0')
        if (args.fault=='ring-check' and name=='borg_equips_ring') or (args.fault=='ring-activate' and name=='borg_activate_ring'):
            body=body.replace('..=inven_left()', '..inven_left()')
        rust+=body+'\n'
    c+=select('angband/src/borg/borg-fight-perm.c','static int borg_perma_aux_regen(')+'\n'
    body=select('borg-rs/src/borg_fight_perm.rs','fn borg_perma_aux_regen(')
    if args.fault=='regen':body=body.replace('if st.traits.simulate {','if false {')
    rust+=body+'\n'
    for name in ('fail_allowed','too_costly','blind_or_confused'):
        rust+=select('borg-rs/src/borg_fight_perm.rs','fn '+name+'(')+'\n'
    for name in ('device_skill','device_fail'):
        rust+=select(ruse,'fn '+name+'(')+'\n'
    ctail=select('angband/src/borg/borg-item-wear.c','    /* prefer the lower danger among available swaps. */','\n}\n')
    rtail=select('borg-rs/src/borg_item_wear.rs','    /* Prefer the lower danger among available swaps. */','\n}\n')
    if args.fault=='swap':
        rtail=rtail.replace('st.trait_swap.weapon_swap != 0\n        && (st.trait_swap.armour_swap == 0 || b_p2 <= b_p1)','b_p1 <= b_p2')
    c+='static bool swap(int b_p1,int b_p2,int p){int b_p,swap;\n'+ctail+'\n}\n'
    rust+='fn swap(st:&mut BorgState,b_p1:i32,b_p2:i32,p:i32)->bool{\n'+rtail+'\n}\n'
    ccold=select('angband/src/borg/borg-danger.c','        case MONBLOW_COLD:','        case MONBLOW_BLIND:')
    rcold=select('borg-rs/src/borg_danger.rs',"            MONBLOW_COLD => 'b: {",'            MONBLOW_BLIND')
    if args.fault=='cold':rcold=rcold.replace('st.borg.temp.res_cold','st.borg.temp.res_acid')
    c+='static int cold(int d_dice,int d_side,bool pfe){int z=0,power=0;switch(1){\n'+ccold+'}\nreturn z*100+power;}\n'
    rust+='fn cold(st:&BorgState,d_dice:i32,d_side:i32,half:bool)->i32{let mut z=0;let mut power=0;match 1{\n'+rcold+'_=>{}}\nz*100+power}\n'
    creset=select('angband/src/borg/borg.c','    /* Flush messages */\n    borg_parse(NULL);','    /* Check for key on the queue */')
    rreset=select('borg-rs/src/borg.rs','    crate::borg_messages::borg_parse(st, None);\n    st.msgs.dont_react = false;','    let borg_ch = borg_inkey(st, true);')
    if args.fault=='reset':rreset=rreset.replace('st.msgs.dont_react = false;','st.borg.dont_react = false;')
    c+='static void reset(void){\n'+creset+'}\n'
    rust+='fn reset(st:&mut BorgState){\n'+rreset+'}\n'
    cname=select('angband/src/borg/borg-flow-kill.c','    if (my_strnicmp(who, "Something", 9) == 0',' {\n')
    rname=select('borg-rs/src/borg_flow_kill.rs','    if strnieq(who, b"Something", 9)',' {\n')
    if args.fault=='name':rname='    if strnieq(who, b"Something", 9) || strnieq(who, b"It", 2)'
    c+='static bool unseen(char *who){return '+cname.strip()[3:]+';}\n'
    rust+='fn unseen(who:&[u8])->bool{'+rname.strip()[3:]+'}\n'+select('borg-rs/src/borg_flow_kill.rs','fn strnieq(')+'\nfn c_toupper(c:u8)->u8{c.to_ascii_uppercase()}\n'
    c+=r'''
int main(int argc,char **argv){int mode=atoi(argv[1]),v=0;switch(mode){
case 0:available=true;if(arg(argv,1)){borg.time.now=arg(argv,1);borg_quaff_crit(true);}quaffs=0;
    borg.time.now=arg(argv,0);borg.trait[BI_ACCW]=arg(argv,3);available=arg(argv,4);random_value=arg(argv,5);
    v=borg_quaff_crit(arg(argv,2));break;
case 1:ring(2,arg(argv,0),arg(argv,1));ring(3,arg(argv,2),arg(argv,3));borg.trait[BI_DEV]=arg(argv,4);borg.trait[BI_ISCONFUSED]=arg(argv,5);
    v=10*borg_equips_ring(1);v+=borg_activate_ring(1);break;
case 2:borg_simulate=arg(argv,0);borg.temp.regen=arg(argv,1);borg.trait[BI_ISBLIND]=arg(argv,2);borg.trait[BI_ISCONFUSED]=arg(argv,3);
    borg.trait[BI_ISFORGET]=arg(argv,4);borg.trait[BI_MAXHP]=arg(argv,5);spell_ok=arg(argv,6);cost=arg(argv,7);borg.trait[BI_CURSP]=arg(argv,8);
    borg.mon.unique=arg(argv,9);borg.near.unique=arg(argv,10);spell_result=arg(argv,11);v=borg_perma_aux_regen();break;
case 3:armour_swap=arg(argv,2);weapon_swap=arg(argv,3);borg.avoidance=arg(argv,5);borg.near.unique=arg(argv,6);
    v=swap(arg(argv,0),arg(argv,1),arg(argv,4));break;
case 4:borg.trait[BI_ICOLD]=arg(argv,2);borg.trait[BI_RCOLD]=arg(argv,3);borg.temp.res_acid=arg(argv,4);borg.temp.res_cold=arg(argv,5);
    borg_attacking=arg(argv,7);v=cold(arg(argv,0),arg(argv,1),arg(argv,6));break;
case 5:borg_dont_react=arg(argv,0);borg.dont_react=arg(argv,1);pending=arg(argv,2);reset();v=borg_dont_react+2*borg.dont_react;break;
case 6:{char who[256]={0};unsigned int byte;for(int i=0;argv[2][i]&&i<500;i+=2){sscanf(argv[2]+i,"%2x",&byte);who[i/2]=byte;}
    v=unseen(who);break;}}
emit(v);return 0;}
'''
    rust+=r'''
fn main(){let args:Vec<String>=std::env::args().collect();let mode:i32=args[1].parse().unwrap();let a=|n:usize|->i32{args[n+2].parse().unwrap()};let mut st=BorgState::default();
let value=match mode {
0=>{st.item_use.when_last_quaff=a(1);st.borg.time.now=a(0);st.borg.values[BI_ACCW]=a(3);st.available=a(4)!=0;engine::RNG.with(|r|r.set((a(5),0)));borg_quaff_crit(&mut st,a(2)!=0) as i32},
1=>{st.items.items=(0..4).map(|_|Item::default()).collect();ring(&mut st,2,a(0),a(1));ring(&mut st,3,a(2),a(3));st.borg.values[BI_DEV]=a(4);st.borg.values[BI_ISCONFUSED]=a(5);10*borg_equips_ring(&mut st,1) as i32+borg_activate_ring(&mut st,1) as i32},
2=>{st.traits.simulate=a(0)!=0;st.borg.temp.regen=a(1)!=0;st.borg.values[BI_ISBLIND]=a(2);st.borg.values[BI_ISCONFUSED]=a(3);st.borg.values[BI_ISFORGET]=a(4);st.borg.values[BI_MAXHP]=a(5);st.spell_ok=a(6)!=0;st.cost=a(7);st.borg.values[BI_CURSP]=a(8);st.borg.mon.unique=a(9);st.borg.near.unique=a(10);st.spell_result=a(11)!=0;borg_perma_aux_regen(&mut st)},
3=>{st.trait_swap.armour_swap=a(2);st.trait_swap.weapon_swap=a(3);st.borg.avoidance=a(5) as i16;st.borg.near.unique=a(6);swap(&mut st,a(0),a(1),a(4)) as i32},
4=>{st.borg.values[BI_ICOLD]=a(2);st.borg.values[BI_RCOLD]=a(3);st.borg.temp.res_acid=a(4)!=0;st.borg.temp.res_cold=a(5)!=0;cold(&st,a(0),a(1),a(6)!=0&&a(7)==0)},
5=>{st.msgs.dont_react=a(0)!=0;st.borg.dont_react=a(1)!=0;st.pending=a(2)!=0;reset(&mut st);i32::from(st.msgs.dont_react)+2*i32::from(st.borg.dont_react)},
6=>{let who:Vec<u8>=args[2].as_bytes().chunks(2).map(|s|u8::from_str_radix(std::str::from_utf8(s).unwrap(),16).unwrap()).collect();unseen(&who) as i32},_=>panic!("mode")};emit(&st,value);}
'''
    (out/'selected.c').write_text(c);(out/'selected.rs').write_text(rust)
    (out/'source-ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
    flags=['-fsanitize=undefined','-fno-sanitize-recover=undefined'] if args.ubsan else []
    commands=[['cc','-O2','-DNDEBUG','-ffunction-sections','-fdata-sections','-I'+str(ROOT/'angband/src'),*flags,
               str(out/'selected.c'),str(ROOT/'angband/src/z-util.c'),'-Wl,--gc-sections','-o',str(out/'c')],
              ['rustc','--edition=2021','-O',str(out/'selected.rs'),'-o',str(out/'rust')]]
    for name,command in zip(('c','rust'),commands):
        with (out/(name+'-build.log')).open('w') as log:subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,check=True)
    cases=[]
    for now in (1,4,100):
        for last in (0,1,96,99,100,101):
            for bypass in (0,1):
                for count in (0,1,2,7):
                    for available in (0,1):
                        for draw in (1,74,75,100):cases.append((0,[now,last,bypass,count,available,draw]))
    for right in (0,31,47,55,59,61,62,63):
        for left in (0,31,47,55,59,61,62,63):
            for skill in (20,80):
                for confused in (0,1):cases.append((1,[right,30,left,40,skill,confused]))
    for simulate in (0,1):
        for guard in range(10):
            row=[simulate,0,0,0,0,100,1,20,401,0,0,1]
            if guard<5:row[guard+1]=[1,1,1,1,99][guard]
            if guard==5:row[6]=0
            if guard==6:row[7]=40
            if guard==7:row[9]=1;row[7]=50
            if guard==8:row[10]=1;row[11]=0
            cases.append((2,row))
    for armour,weapon in ((1,2),(1,0),(0,2)):
        for ap,wp in ((0,0),(29,79),(79,29),(40,40),(9999,30),(30,9999),(50,51),(51,50)):
            for p in (30,50,100):
                for unique in (0,1):cases.append((3,[ap,wp,armour,weapon,p,100,unique]))
    for bits in range(64):
        for dice,sides in ((1,1),(3,7),(10,10)):cases.append((4,[dice,sides,*[(bits>>k)&1 for k in range(6)]]))
    for bits in range(8):cases.append((5,[(bits>>k)&1 for k in range(3)]))
    for who in (b'It',b'it',b'IT',b'Itangast the Fire Drake',b'Itangast',b'I',b'Something',b'something else',b'Ogre',b'It\0',b'It\0unused',b'It ',b'Item'):
        cases.append((6,[who.hex()]))
    counts={};rows=[]
    for number,(mode,values) in enumerate(cases):
        args_wire=[str(mode),*map(str,values)]
        before=subprocess.run([str(out/'c'),*args_wire],capture_output=True,check=True).stdout
        after=subprocess.run([str(out/'rust'),*args_wire],capture_output=True,check=True).stdout
        if before!=after:
            report={'passed':False,'fault':args.fault,'case':number,'mode':mode,'input':values,'c':before.decode(),'rust':after.decode()}
            (out/'result.json').write_text(json.dumps(report,indent=2)+'\n');print(report)
            if args.fault:return
            raise AssertionError('Production C/Rust kernel mismatch')
        counts[str(mode)]=counts.get(str(mode),0)+1
        rows.append({'mode':mode,'input':values,'output':before.decode().strip()})
    assert not args.fault,'Fault survived the corpus'
    (out/'observations.json').write_text(json.dumps(rows)+'\n')
    (out/'result.json').write_text(json.dumps({'passed':True,'cases':len(cases),'by_kernel':counts,'ubsan':args.ubsan},indent=2)+'\n')
    print('PASS',len(cases),'production-body/seam comparisons',counts)


if __name__=='__main__':main()
