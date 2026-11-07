import json,lzma,re
from pathlib import Path
rows={};pattern=re.compile(rb'^(\d+) .*? d(\d+)/(\d+).*? lv(\d+) ')
for suite in ('extended','natural'):
    for d in sorted((Path('harness/reference')/suite).iterdir()):
        f=d/'state.log.xz'
        if not f.exists():continue
        summary=json.loads((d/'summary.json').read_text());peak_depth=peak_field=peak_level=0;first_peak=None
        with lzma.open(f,'rb') as stream:
            for line in stream:
                m=pattern.match(line)
                if not m:continue
                turn,depth,field,level=map(int,m.groups())
                if depth>peak_depth:peak_depth=depth;first_peak=turn
                peak_field=max(peak_field,field);peak_level=max(peak_level,level)
        rows[suite+'/'+d.name]={'deepest_logged_current_depth':peak_depth,'first_game_turn_at_deepest_logged_depth':first_peak,'greatest_logged_max_depth_field':peak_field,'highest_logged_clevel':peak_level,'final_summary_max_depth_field':summary['max_depth'],'final_summary_depth':summary['depth'],'final_summary_clevel':summary['clevel'],'reference_state_log':str(f)}
        print(suite,d.name,'deepest logged',peak_depth,'final max_depth field',summary['max_depth'],flush=True)
result_path=Path('harness/out/local/depth-audit.json')
result_path.parent.mkdir(parents=True,exist_ok=True)
result_path.write_text(json.dumps(rows,indent=2,sort_keys=True)+'\n')
