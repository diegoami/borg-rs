/* Isolate the real thrust target-selection body; stub its scoring/engine boundary.
 * Natural seed2000 also reproduces the same +5 abort in the complete engine. */
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define BI_ISAFRAID 0
#define BI_CRSFEAR 1
#define BI_CDEPTH 2
#define BI_MAXCLEVEL 3
#define INVEN_WIELD 0
#define I2D(n) ('0' + (n))
typedef struct { int x, y; } point;
typedef struct { int kill; } borg_grid;
typedef struct { int r_idx, power; bool awake; } borg_kill;
static struct { int trait[4], avoidance; bool munchkin_mode; point c; struct { point g; } goal; } borg;
static borg_grid borg_grids[32][32];
static borg_kill borg_kills[2];
static struct { char desc[32]; } borg_items[1];
static struct { int r_max; } info = {100}, *z_info = &info;
static int borg_temp_n, borg_temp_x[1], borg_temp_y[1];
static bool borg_simulate;
static char keys[8];
static size_t key_count;
static int borg_distance(int y1, int x1, int y2, int x2) { int y=abs(y2-y1), x=abs(x2-x1); return y>x?y:x; }
static int borg_thrust_damage_one(int k) { assert(k==1); return 10; }
static int borg_danger_one_kill(int y,int x,int n,int k,bool a,bool b) { (void)y;(void)x;(void)n;(void)k;(void)a;(void)b;return 0; }
static const char *borg_race_name(int k) { (void)k;return "fixture"; }
static const char *format(const char *fmt, ...) { (void)fmt;return "fixture"; }
static void borg_note(const char *text) { (void)text; }
static int borg_extract_dir(int y1,int x1,int y2,int x2) { const int d[3][3]={{7,8,9},{4,5,6},{1,2,3}};return d[y2-y1+1][x2-x1+1]; }
static void borg_keypress(int key) { assert(key_count<sizeof(keys)-1);keys[key_count++]=(char)key;keys[key_count]=0; }
#include "thrust.inc"
static void check(int dy, int dx, bool simulate)
{
    memset(borg_grids,0,sizeof(borg_grids));
    borg.c=(point){10,10};borg.trait[BI_CDEPTH]=1;borg.avoidance=100;
    borg_temp_n=1;borg_temp_y[0]=10+dy;borg_temp_x[0]=10+dx;
    borg_grids[10+dy][10+dx].kill=1;
    borg_kills[1]=(borg_kill){1,20,true};borg_simulate=simulate;
    key_count=0;keys[0]=0;
    int result=borg_attack_aux_thrust();
    bool adjacent=(abs(dy)<=1 && abs(dx)<=1 && (dy || dx));
    if (!adjacent && (result || key_count)) fprintf(stderr,"non-adjacent (%d,%d): score=%d keys='%s'\n",dy,dx,result,keys);
    assert(result==(adjacent?10:0));
    assert(key_count==(adjacent && !simulate?2:0));
    if (key_count) { assert(keys[0]=='+');assert(keys[1]!='5'); }
}
int main(void)
{
    for (int mode=0;mode<2;++mode) {
        check(0,0,mode);check(0,2,mode);
        for (int y=-1;y<=1;++y) for (int x=-1;x<=1;++x) if (y||x) check(y,x,mode);
    }
    puts("actual C thrust body: center/distant rejected; all eight adjacent grids retained in apply/simulate modes");
}
