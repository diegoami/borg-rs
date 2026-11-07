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
write('glyph-distance-fault.rs','borg-rs/src/borg_flow_glyph.rs',
      'engine::distance(st.borg.c, center) >= 50',
      'engine::distance(st.borg.c, center) > 50')
text=(root/'borg-rs/src/borg_flow_glyph.rs').read_text()
assert text.count('v < b_v')==2
(out/'glyph-tie-fault.rs').write_text(text.replace('v < b_v','v <= b_v'))
before,after=text.rsplit('v < b_v',1)
(out/'glyph-old-tie-fault.rs').write_text(before+'v <= b_v'+after)

text=(root/'borg-rs/src/borg_think_dungeon.rs').read_text();a=text.index('fn borg_think_dungeon_lunal(');b=text.index('\n}',a)
body=text[a:b];old='st.borg.goal.fleeing_lunal = false;';assert old in body
(out/'lunal-fault.rs').write_text(text[:a]+body.replace(old,'st.borg.goal.fleeing_lunal = true;',1)+text[b:])
