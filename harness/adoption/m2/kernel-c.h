/* Fixture environment for selected production bodies. GPL-2.0-or-later /
 * Angband licence. These are controlled callbacks, not game simulations. */
#include "h-basic.h"
#include "z-util.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#define BI_ACCW 0
#define BI_DEV 1
#define BI_ISCONFUSED 2
#define BI_ISBLIND 3
#define BI_ISFORGET 4
#define BI_MAXHP 5
#define BI_CURSP 6
#define BI_ICOLD 7
#define BI_RCOLD 8
#define TV_RING 21
#define INVEN_WIELD 0
#define INVEN_RIGHT 2
#define INVEN_LEFT 3
#define RAPID_REGENERATION 0
#define MONBLOW_COLD 1
typedef int borg_time;
struct {int trait[9];struct{bool regen,res_acid,res_cold;}temp;
    struct{int unique;}mon,near;struct{int now;}time;int no_rest_prep,avoidance;bool dont_react;} borg;
typedef struct {int iqty,tval,sval,timeout,level;bool aware,ident;char *desc;} borg_item;
borg_item borg_items[4];
bool borg_simulate, borg_dont_react, borg_attacking;
int armour_swap,weapon_swap,sv_potion_cure_critical=7;
static const char all_letters_nohjkl[]="abcdefgimnopqrstuvwxyz";
static bool available,spell_ok,spell_result,pending;
static int cost,random_value,draws,quaffs,casts,notes,nkeys,keys[8];
#define borg_timer(t) (borg.time.now-(t))
static int randint1(int n) {draws++;return random_value;}
static bool borg_quaff_potion(int sval) {quaffs++;return available;}
static bool borg_spell_okay_fail(int spell,int fail) {return spell_ok;}
static int borg_get_spell_power(int spell) {return cost;}
static bool borg_spell(int spell) {casts++;return spell_result;}
static void borg_note(const char *s) {notes++;}
static char *format(const char *s,...) {return "controlled note";}
static void borg_keypress(int key) {keys[nkeys++]=key;}
static void borg_parse(const char *s) {if(pending)borg_dont_react=true;}
static int arg(char **argv,int n) {return atoi(argv[n+2]);}
static void ring(int slot,int bits,int level) {borg_item *p=borg_items+slot;
    p->aware=bits&1;p->iqty=(bits>>1)&1;p->tval=(bits&4)?TV_RING:0;
    p->sval=(bits&8)?1:2;p->timeout=(bits&16)?0:1;p->ident=bits&32;p->level=level;p->desc="Ring";}
static void emit(int value) {printf("%d %d %d %d %d %d %d",value,draws,quaffs,casts,notes,borg.no_rest_prep,nkeys);
    for(int i=0;i<nkeys;i++)printf(" %d",keys[i]);putchar('\n');}
