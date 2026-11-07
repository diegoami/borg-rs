/* Original native C modules; fixture callbacks provide only engine/environment data. */
#include <stdarg.h>
#include <string.h>
#include "cave.h"
#include "borg/borg.h"
#include "borg/borg-cave.h"
#include "borg/borg-danger.h"
#include "borg/borg-flow-kill.h"
#define borg_los xc_borg_los
#define borg_projectable xc_borg_projectable
#define borg_offset_projectable xc_borg_offset_projectable
#define borg_projectable_pure xc_borg_projectable_pure
#define borg_projectable_dark xc_borg_projectable_dark
#define borg_inc_motion xc_borg_inc_motion
#define borg_distance xc_borg_distance
#define borg_target xc_borg_target
#define borg_target_unknown_wall xc_borg_target_unknown_wall
#define borg_flow_clear xc_borg_flow_clear
#define borg_flow_spread xc_borg_flow_spread
#define borg_flow_enqueue_grid xc_borg_flow_enqueue_grid
#define borg_flow_commit xc_borg_flow_commit
#include "borg/borg-projection.c"
#define borg_flow_old unused_original_borg_flow_old
#include "borg/borg-flow.c"
#undef borg_flow_old
static bool glyph_mode;static int glyph_flags;static int glyph_trace[16][4];static size_t glyph_calls;
static int glyph_xs[200],glyph_ys[200];
int sv_scroll_rune_of_protection=77;int act_glyph=88;
static bool glyph_call(int kind,int a,int b){if(glyph_calls==16)abort();bool result=(glyph_flags&(1<<(kind-1)))!=0;int*t=glyph_trace[glyph_calls++];t[0]=kind;t[1]=a;t[2]=b;t[3]=result;return result;}
bool borg_spell_fail(enum borg_spells spell,int fail){return glyph_call(1,spell,fail);}
bool borg_read_scroll(int sval){return glyph_call(2,sval,0);}
bool borg_activate_item(int act){return glyph_call(3,act,0);}
static bool fixture_glyph_flow_old(int why){return glyph_call(4,why,0);}
/* Candidate-cost adapter; #345 otherwise blocks outlying ties/cutoff cases. */
#include "borg/borg-flow-misc.h"
static int glyph_scenario;
static void fixture_glyph_reverse(int depth,bool optimize,bool avoid,bool tunneling,int stair_idx,bool sneak){
    borg_flow_reverse(depth,optimize,avoid,tunneling,stair_idx,sneak);
    if(glyph_scenario>=19 && glyph_scenario<=25){
        memset(borg_data_cost->data,255,sizeof(borg_data_cost->data));
        borg_data_cost->data[33][100]=0;
    }
    if(glyph_scenario>=19 && glyph_scenario<=22){
        borg_data_cost->data[33][99]=1;
        borg_data_cost->data[33][101]=glyph_scenario>=21?2:1;
    }
    if(glyph_scenario>=23 && glyph_scenario<=25)
        borg_data_cost->data[33][101]=74+glyph_scenario-23;
}
#define borg_flow_reverse fixture_glyph_reverse
#define borg_flow_old fixture_glyph_flow_old
#include "borg/borg-flow-glyph.c"
#undef borg_flow_old
#undef borg_flow_reverse

struct borg_struct borg;
borg_grid *borg_grids[AUTO_MAX_Y];
borg_kill *borg_kills;
uint16_t borg_fear_region[(AUTO_MAX_Y/11)+1][(AUTO_MAX_X/11)+1];
bool borg_danger_wipe;
static struct chunk arena;
struct chunk *cave=&arena;
static struct angband_constants settings;
struct angband_constants *z_info=&settings;
static struct monster_race fixture_races[10];
struct monster_race *r_info=fixture_races;
static borg_grid grids[AUTO_MAX_Y][AUTO_MAX_X];
static borg_kill kills[10];
borg_item *borg_items;static borg_item fixture_items[32];bool borg_simulate;
static uint16_t dangers[AUTO_MAX_Y][AUTO_MAX_X];
static bool physical_mode;
#include "physical_damage.inc"
static int damage_scores[10];static int combat_trace[256][7];static size_t combat_calls;
static int borg_thrust_damage_one(int k){int *t=combat_trace[combat_calls++];memset(t,0,7*sizeof(int));t[0]=0;t[1]=k;t[2]=physical_mode?xc_physical_damage_one(k):damage_scores[k];t[3]=physical_mode;return t[2];}
int borg_danger_one_kill(int y,int x,int n,int k,bool a,bool b){int *t=combat_trace[combat_calls++];t[0]=1;t[1]=y;t[2]=x;t[3]=n;t[4]=k;t[5]=a;t[6]=b;return dangers[y][x]+k*17+n*3;}
#include "thrust.inc"
static int traits[BI_MAX];
static borg_data flow_maps[5];

static int danger_trace[16384][5];static size_t danger_calls;
static int stair_x,stair_y;
struct borg_track track_less;
static uint32_t keys[1024];static size_t key_count;
static char notes[16384];static size_t note_count;
static const uint8_t *input;static size_t input_size,input_at;
static uint8_t *output;static size_t output_size,output_at;
static uint32_t read32(void){if(input_at+4>input_size)abort();uint32_t n=0;for(int i=0;i<4;i++)n|=(uint32_t)input[input_at++]<<(8*i);return n;}
static uint8_t read8(void){if(input_at==input_size)abort();return input[input_at++];}
static void put8(uint8_t n){if(output_at==output_size)abort();output[output_at++]=n;}
static void put32(int32_t n){for(int i=0;i<4;i++)put8((uint32_t)n>>(8*i));}
static void put16(uint16_t n){put8(n);put8(n>>8);}
bool square_in_bounds_fully(struct chunk *c,struct loc g){return g.x>0&&g.x<c->width-1&&g.y>0&&g.y<c->height-1;}
bool feat_is_shop(int f){return f>=FEAT_STORE_GENERAL&&f<=FEAT_HOME;}
int borg_danger(int y,int x,int n,bool a,bool b){if(danger_calls==16384)abort();int *t=danger_trace[danger_calls++];t[0]=y;t[1]=x;t[2]=n;t[3]=a;t[4]=b;return dangers[y][x];}
const char *borg_race_name(int index){(void)index;return "fixture";}
errr borg_keypress(keycode_t key){if(key_count==1024)abort();keys[key_count++]=key;return 0;}
void borg_note(const char *text){size_t n=strlen(text);if(note_count+n+1>=sizeof(notes))abort();memcpy(notes+note_count,text,n);note_count+=n;notes[note_count++]='\n';}
static bool fixture_lunal_mode;
static int lunal_params[19],lunal_trace[128][5];static size_t lunal_calls;
static int cfg[BORG_MAX_SETTINGS];int *borg_cfg=cfg;
static int less_xs[8],less_ys[8],more_xs[8],more_ys[8];struct borg_track track_more;
static struct square observed_square;
static int lcall(int kind,int a,int b,int c,int result){if(lunal_calls==128)abort();int*t=lunal_trace[lunal_calls++];t[0]=kind;t[1]=a;t[2]=b;t[3]=c;t[4]=result;return result;}
static int lbool(int kind,int a,int b,int c){return lcall(kind,a,b,c,(lunal_params[18]&(1<<kind))!=0);}
static struct square* lsquare(struct chunk*c,struct loc g){(void)c;(void)g;return &observed_square;}
static void lnotice(bool swap){lcall(0,swap,0,0,0);}
static const char*lprepared(int d){return lcall(1,d,0,0,lunal_params[10])?NULL:"not ready";}
static enum borg_need llight(void){return lcall(2,0,0,0,lunal_params[11]);}
static bool lrest(int y,int x){return lcall(3,y,x,0,lunal_params[12]);}
static bool lrecover(void){return lbool(4,0,0,0);}
static bool luse(void){return lbool(5,0,0,0);}
static bool ldrop(void){return lbool(6,0,0,0);}
static bool lold(int why){return lcall(7,why,0,0,(lunal_params[18]&(1<<(why==2?7:12)))!=0);}
static bool ltake(bool view,int distance){return lbool(8,view,distance,0);}
static bool lless(int why,bool sneak){return lbool(9,why,sneak,0);}
static bool lmore(int why,bool sneak,bool brave){return lbool(10,why,sneak,brave);}
static bool lboth(int why,bool sneak){return lbool(11,why,sneak,0);}
#define square lsquare
#define borg_notice lnotice
#define borg_prepared lprepared
#define borg_maintain_light llight
#define borg_check_rest lrest
#define borg_recover lrecover
#define borg_use_things luse
#define borg_drop_junk ldrop
#define borg_flow_old lold
#define borg_flow_take_lunal ltake
#define borg_flow_stair_less lless
#define borg_flow_stair_more lmore
#define borg_flow_stair_both lboth
#undef OPT
#define OPT(p,o) (lunal_params[17]!=0)
#include "lunal.inc"
#undef square
#undef borg_notice
#undef borg_prepared
#undef borg_maintain_light
#undef borg_check_rest
#undef borg_recover
#undef borg_use_things
#undef borg_drop_junk
#undef borg_flow_old
#undef borg_flow_take_lunal
#undef borg_flow_stair_less
#undef borg_flow_stair_more
#undef borg_flow_stair_both

size_t spatial_snapshot(const uint8_t *packet,size_t packet_size,uint8_t *result,size_t result_size)
{
    input=packet;input_size=packet_size;input_at=0;output=result;output_size=result_size;output_at=0;
    int f[32];for(int i=0;i<32;i++)f[i]=(int32_t)read32();
    memset(&borg,0,sizeof(borg));memset(traits,0,sizeof(traits));memset(kills,0,sizeof(kills));memset(fixture_races,0,sizeof(fixture_races));
    arena.width=(f[28]==6&&f[30]!=2)?66:AUTO_MAX_X;arena.height=(f[28]==6&&f[30]!=2)?22:AUTO_MAX_Y;settings.max_range=f[18];settings.r_max=10;settings.pack_size=0;borg_items=fixture_items;memset(fixture_items,0,sizeof(fixture_items));strcpy(fixture_items[0].desc,"fixture weapon");borg.trait=traits;borg_kills=kills;
    borg.c=loc(f[0],f[1]);traits[BI_CURHP]=f[2];traits[BI_MAXHP]=f[3];traits[BI_MAXCLEVEL]=f[4];traits[BI_CLEVEL]=f[5];traits[BI_FOOD]=f[6];traits[BI_DISP]=f[7];traits[BI_DISM]=f[8];traits[BI_IFIRE]=f[9];traits[BI_ISAFRAID]=f[10];borg.avoidance=f[11];traits[BI_CRSFEAR]=f[29]&1;traits[BI_CDEPTH]=f[28]==6?0:50;
    borg.status.desperate=f[12]&1;borg.status.vault=f[12]&2;borg.status.digging=f[12]&4;borg.mon.scary=f[12]&8;borg.goal.ignoring=f[12]&16;borg.lunal_mode=f[12]&32;borg.munchkin_mode=f[12]&64;borg.morgoth_position=f[12]&128;borg_danger_wipe=f[12]&256;
    borg.mon.unique=f[13];borg.time.now=f[14];borg.time.level=f[15];borg.goal.shop=f[16];
    for(int y=0;y<(AUTO_MAX_Y/11)+1;y++)for(int x=0;x<(AUTO_MAX_X/11)+1;x++)borg_fear_region[y][x]=f[17];
    int d[11];for(int i=0;i<11;i++)d[i]=(int32_t)read32();
    fixture_items[0].dd=d[0];fixture_items[0].ds=d[1];fixture_items[0].to_d=d[2];fixture_items[0].to_h=d[3];
    traits[BI_TODAM]=d[4];traits[BI_BLOWS]=d[5];traits[BI_THN]=d[6];traits[BI_TOHIT]=d[7];traits[BI_CLASS]=d[8];traits[BI_CURSP]=d[9];
    int slay_traits[]={BI_WS_ANIMAL,BI_WS_EVIL,BI_WS_UNDEAD,BI_WS_DEMON,BI_WS_ORC,BI_WS_TROLL,BI_WS_GIANT,BI_WS_DRAGON,BI_WB_ACID,BI_WB_FIRE,BI_WB_COLD,BI_WB_POIS,BI_WB_ELEC,BI_WK_UNDEAD,BI_WK_DEMON,BI_WK_DRAGON};
    for(int i=0;i<16;i++)traits[slay_traits[i]]=(d[10]&(1<<i))!=0;
    int race_flags[]={RF_ANIMAL,RF_EVIL,RF_UNDEAD,RF_DEMON,RF_ORC,RF_TROLL,RF_GIANT,RF_DRAGON,RF_IM_ACID,RF_IM_FIRE,RF_IM_COLD,RF_IM_POIS,RF_IM_ELEC,RF_UNIQUE,RF_MULTIPLY,RF_QUESTOR};
    int spell_flags[]={RSF_S_KIN,RSF_S_HI_DEMON,RSF_S_MONSTER,RSF_S_MONSTERS,RSF_S_ANIMAL,RSF_S_SPIDER,RSF_S_HOUND,RSF_S_HYDRA,RSF_S_AINU,RSF_S_DEMON,RSF_S_UNDEAD,RSF_S_DRAGON,RSF_S_HI_UNDEAD,RSF_S_WRAITH,RSF_S_UNIQUE,RSF_S_HI_DRAGON};
    for(int i=0;i<16;i++)put32(race_flags[i]);put32(RF_PASS_WALL);for(int i=0;i<16;i++)put32(spell_flags[i]);
    for(int k=0;k<10;k++){fixture_races[k].ac=(int32_t)read32();uint32_t flags=read32(),spells=read32();for(int i=0;i<16;i++){if(flags&(1u<<i))rf_on(fixture_races[k].flags,race_flags[i]);if(spells&(1u<<i))rsf_on(fixture_races[k].spell_flags,spell_flags[i]);}}
    borg_data_flow=&flow_maps[0];borg_data_cost=&flow_maps[1];borg_data_hard=&flow_maps[2];borg_data_know=&flow_maps[3];borg_data_icky=&flow_maps[4];
    for(int y=0;y<AUTO_MAX_Y;y++){borg_grids[y]=grids[y];for(int x=0;x<AUTO_MAX_X;x++){
        borg_grid *g=&grids[y][x];g->feat=read8();g->info=read8();g->info|=(uint16_t)read8()<<8;g->trap=read8()!=0;g->glyph=read8()!=0;g->web=read8()!=0;g->store=read8();g->take=read8();g->kill=read8();g->xtra=read8();
        for(int m=0;m<5;m++)flow_maps[m].data[y][x]=read8();
        dangers[y][x]=read8();dangers[y][x]|=(uint16_t)read8()<<8;
    }}
    for(int i=0;i<AUTO_FLOW_MAX;i++){borg_flow_x[i]=read8();borg_flow_y[i]=read8();}
    for(int i=0;i<10;i++){kills[i].r_idx=i;kills[i].power=i?100+i*10:0;kills[i].awake=(f[29]&(1<<(8+i)))!=0;damage_scores[i]=i==0?0:i==1?1000:((i+f[7])%5==0?0:10+f[7]*3+i*7);if(i&&(f[31]&(1<<i)))rf_on(fixture_races[i].flags,RF_PASS_WALL);}
    stair_x=f[24];stair_y=f[25];track_less.num=track_less.size=1;track_less.x=&stair_x;track_less.y=&stair_y;
    key_count=note_count=danger_calls=combat_calls=0;borg_target_loc=loc(0,0);
    int px=f[0],py=f[1],tx=f[19],ty=f[20];
    glyph_x=glyph_y=glyph_x_center=glyph_y_center=0;borg_needs_new_sea=false;track_glyph.num=0;track_glyph.size=200;track_glyph.x=glyph_xs;track_glyph.y=glyph_ys;memset(glyph_xs,0,sizeof(glyph_xs));memset(glyph_ys,0,sizeof(glyph_ys));glyph_calls=0;glyph_flags=f[29]&15;
    lunal_calls=0;track_more.num=0;track_more.size=8;track_more.x=more_xs;track_more.y=more_ys;memset(more_xs,0,sizeof(more_xs));memset(more_ys,0,sizeof(more_ys));
    if(fixture_lunal_mode){
        uint32_t n=read32();for(uint32_t i=0;i<n;i++){read8();read8();}
        for(int i=0;i<19;i++)lunal_params[i]=(int32_t)read32();
        settings.pack_size=20;borg.lunal_mode=true;borg.goal.fleeing=borg.goal.fleeing_lunal=true;
        traits[BI_CDEPTH]=lunal_params[0];traits[BI_MAXDEPTH]=lunal_params[1];traits[BI_ISWEAK]=lunal_params[2];traits[BI_ISHUNGRY]=lunal_params[3];traits[BI_LIGHT]=lunal_params[4];fixture_items[25].tval=lunal_params[5];fixture_items[18].iqty=lunal_params[6];
        observed_square.feat=lunal_params[7];grids[py][px].feat=lunal_params[8];memset(cfg,0,sizeof(cfg));cfg[BORG_SELF_LUNAL]=lunal_params[9];
        track_less.num=lunal_params[13];track_less.size=8;track_less.x=less_xs;track_less.y=less_ys;
        track_more.num=lunal_params[14];memset(less_xs,0,sizeof(less_xs));memset(less_ys,0,sizeof(less_ys));
        for(int i=0;i<8;i++){less_xs[i]=px+lunal_params[15]+i;less_ys[i]=py;more_xs[i]=px+lunal_params[16]+i;more_ys[i]=py;}
        put32(borg_think_dungeon_lunal());goto serialize_state;
    }
    if(glyph_mode){
        uint32_t n=read32();for(uint32_t i=0;i<n;i++){read8();read8();}
        int scenario=f[28];glyph_scenario=scenario;borg_needs_new_sea=scenario==0||scenario==2||scenario==5||scenario==6||scenario==7||scenario==11||scenario==12||scenario==13||scenario==15;
        if((scenario<=1||scenario==6||scenario==7||scenario==13||(scenario>=16&&scenario<=18))){glyph_x=px;glyph_y=py;}
        if(scenario==1||scenario==3||scenario==4||scenario==9||scenario==10||scenario==14){glyph_x_center=px+(scenario==9||scenario==10?-1:scenario==14?10:0);glyph_y_center=py;glyph_xs[0]=glyph_x_center;glyph_ys[0]=py;track_glyph.num=1;}
        if(scenario==4||scenario==14){for(int i=0;i<24;i++){glyph_xs[i+1]=glyph_x_center+borg_ddx_ddd[i];glyph_ys[i+1]=py+borg_ddy_ddd[i];}track_glyph.num=25;}
        if(scenario==6||scenario==7){track_glyph.num=scenario==6?200:199;for(int i=0;i<track_glyph.num;i++){glyph_xs[i]=1+i%196;glyph_ys[i]=1+i/196;}}
        if(scenario==8){glyph_x_center=10;glyph_y_center=10;borg_needs_new_sea=false;}
        if(scenario>=16&&scenario<=18){glyph_x_center=px-(49+scenario-16);glyph_y_center=py;glyph_xs[0]=glyph_x_center;glyph_ys[0]=py;track_glyph.num=1;}
        if(scenario==20||scenario==22||(scenario>=23&&scenario<=25)){glyph_x_center=px;glyph_y_center=py;glyph_xs[0]=px;glyph_ys[0]=py;track_glyph.num=1;}
        put32(borg_flow_glyph());goto serialize_state;
    }
    put32(xc_borg_los(0,0,py,px));put32(xc_borg_los(py,px,ty,tx));put32(xc_borg_projectable(py,px,ty,tx));put32(xc_borg_offset_projectable(py,px,ty,tx));put32(xc_borg_projectable_pure(py,px,ty,tx));put32(xc_borg_projectable_dark(py,px,ty,tx));
    int mx=px,my=py;for(int i=0;i<12;i++){xc_borg_inc_motion(&my,&mx,py,px,ty,tx);put32(mx);put32(my);}
    put32(xc_borg_target(loc(tx,ty),f[30]!=0));
    if(px!=tx||py!=ty)put32(xc_borg_target_unknown_wall(ty,tx));else put32(-1);
    put32(xc_borg_extract_dir(py,px,ty,tx));put32(xc_borg_goto_dir(py,px,ty,tx));
    borg_temp_n=0;if(grids[py][px].kill){borg_temp_x[borg_temp_n]=px;borg_temp_y[borg_temp_n++]=py;}
    for(int dy=-1;dy<=1;dy++)for(int dx=-1;dx<=1;dx++)if((dx||dy)&&grids[py+dy][px+dx].kill){borg_temp_x[borg_temp_n]=px+dx;borg_temp_y[borg_temp_n++]=py+dy;}
    if(grids[py][px+2].kill){borg_temp_x[borg_temp_n]=px+2;borg_temp_y[borg_temp_n++]=py;}
    for(int i=1;i<10;i++)put32(xc_physical_damage_one(i));
    physical_mode=false;
    borg_simulate=true;put32(borg_attack_aux_thrust());put32(borg.goal.g.x);put32(borg.goal.g.y);
    borg_simulate=false;put32(borg_attack_aux_thrust());put32(borg.goal.g.x);put32(borg.goal.g.y);
    physical_mode=true;
    borg_simulate=true;put32(borg_attack_aux_thrust());put32(borg.goal.g.x);put32(borg.goal.g.y);
    borg_simulate=false;put32(borg_attack_aux_thrust());put32(borg.goal.g.x);put32(borg.goal.g.y);
    flow_head=f[27];flow_tail=(f[27]+1)%AUTO_FLOW_MAX;
    xc_borg_flow_clear();put32(flow_head);put32(flow_tail);put32(borg_danger_wipe);
    for(int y=0;y<AUTO_MAX_Y;y++)for(int x=0;x<AUTO_MAX_X;x++)for(int m=1;m<5;m++)put8(flow_maps[m].data[y][x]);
    flow_head=flow_tail=f[27];
    uint32_t origins=read32();for(uint32_t i=0;i<origins;i++){int x=read8(),y=read8();xc_borg_flow_enqueue_grid(y,x);}
    put32(flow_head);put32(flow_tail);
    for(int y=0;y<AUTO_MAX_Y;y++)for(int x=0;x<AUTO_MAX_X;x++)for(int m=1;m<5;m++)put8(flow_maps[m].data[y][x]);
    for(int i=0;i<AUTO_FLOW_MAX;i++){put8(borg_flow_x[i]);put8(borg_flow_y[i]);}
    xc_borg_flow_spread(f[21],f[22]&1,f[22]&2,f[22]&4,f[23],f[22]&8);
    put32(xc_borg_flow_commit("fixture",f[26]));
serialize_state:
    put32(borg.lunal_mode);put32(borg.goal.fleeing);put32(borg.goal.fleeing_lunal);put32(borg.goal.less);
    put32(track_less.num);put32(track_less.size);for(int i=0;i<track_less.size;i++){put32(track_less.x[i]);put32(track_less.y[i]);}
    put32(track_more.num);put32(track_more.size);for(int i=0;i<track_more.size;i++){put32(track_more.x[i]);put32(track_more.y[i]);}
    put32(lunal_calls);for(size_t i=0;i<lunal_calls;i++)for(int j=0;j<5;j++)put32(lunal_trace[i][j]);
    if(input_at!=input_size)abort();
    put32(borg_needs_new_sea);put32(glyph_x);put32(glyph_y);put32(glyph_x_center);put32(glyph_y_center);put32(track_glyph.num);put32(track_glyph.size);for(int i=0;i<200;i++){put32(glyph_xs[i]);put32(glyph_ys[i]);}put32(glyph_calls);for(size_t i=0;i<glyph_calls;i++)for(int j=0;j<4;j++)put32(glyph_trace[i][j]);
    put32(flow_head);put32(flow_tail);put32(borg.goal.type);put32(borg_danger_wipe);put32(borg_target_loc.x);put32(borg_target_loc.y);
    for(int y=0;y<AUTO_MAX_Y;y++)for(int x=0;x<AUTO_MAX_X;x++){
        borg_grid *g=&grids[y][x];put8(g->feat);put16(g->info);put8(g->trap);put8(g->glyph);put8(g->web);put8(g->store);put8(g->take);put8(g->kill);put8(g->xtra);
        for(int m=0;m<5;m++)put8(flow_maps[m].data[y][x]);
    }
    for(int i=0;i<AUTO_FLOW_MAX;i++){put8(borg_flow_x[i]);put8(borg_flow_y[i]);}
    put32(combat_calls);for(size_t i=0;i<combat_calls;i++)for(int j=0;j<7;j++)put32(combat_trace[i][j]);
    put32(danger_calls);for(size_t i=0;i<danger_calls;i++)for(int j=0;j<5;j++)put32(danger_trace[i][j]);
    put32(key_count);for(size_t i=0;i<key_count;i++)put32(keys[i]);put32(note_count);for(size_t i=0;i<note_count;i++)put8(notes[i]);
    return output_at;
}

size_t glyph_snapshot(const uint8_t *packet,size_t n,uint8_t *output,size_t capacity){glyph_mode=true;size_t result=spatial_snapshot(packet,n,output,capacity);glyph_mode=false;return result;}

size_t lunal_snapshot(const uint8_t*p,size_t n,uint8_t*out,size_t cap){fixture_lunal_mode=true;size_t result=spatial_snapshot(p,n,out,cap);fixture_lunal_mode=false;return result;}
