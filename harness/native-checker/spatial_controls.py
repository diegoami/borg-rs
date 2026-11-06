"""Write deliberate source faults; never edit production inputs."""
from pathlib import Path
import sys
root,out=Path(sys.argv[1]),Path(sys.argv[2])
def write(name,path,old,new):
 text=(root/path).read_text();assert text.count(old)==1,(name,text.count(old))
 (out/name).write_text(text.replace(old,new))
write('flow-fault.rs','borg-rs/src/borg_flow.rs','''    for row in d.iter_mut() {
        row.fill(0);
    }
''','''    for row in d.iter_mut() {
        row.fill(0);
    }
    d[0][0] = 1;
''')
write('projection-fault.rs','borg-rs/src/borg_projection.rs','''    if ax < 2 && ay < 2 {
        return true;
    }''','''    if ax < 2 && ay < 2 {
        return false;
    }''')
write('combat-fault.rs','borg-rs/src/borg_fight_attack.rs','if borg_distance(st.borg.c.y, st.borg.c.x, y, x) != 1 {','if borg_distance(st.borg.c.y, st.borg.c.x, y, x) > 1 {')
text=(root/'borg-rs/src/borg_fight_attack.rs').read_text();a=text.index('fn borg_thrust_damage_one(');b=text.index('\n}',a)
body=text[a:b];assert body.count('dam *= mult;')==1
(out/'damage-fault.rs').write_text(text[:a]+body.replace('dam *= mult;','dam *= mult + 1;')+text[b:])

write('glyph-fault.rs','borg-rs/src/borg_flow_glyph.rs','st.glyph.glyph_x = 0;','st.glyph.glyph_x = 1;')
