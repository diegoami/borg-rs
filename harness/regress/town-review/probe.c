/* Disposable native PR83 fixture; relinked test binary only. */
#include "angband.h"
#include "init.h"
#include "object.h"
#include "obj-desc.h"
#include "obj-gear.h"
#include "obj-knowledge.h"
#include "obj-make.h"
#include "obj-pile.h"
#include "obj-util.h"
#include "option.h"
#include "player.h"
#include "savefile.h"
#include "store.h"
extern bool __real_savefile_load(const char *, bool);
extern uint64_t borg_rs_test_learning_identity(const struct object *);
#define CHECK(x) do { if (!(x)) { fprintf(stderr,"town review check failed: %s:%d: %s\n",__FILE__,__LINE__,#x); _Exit(9); } } while (0)
static struct object *make(int tval, const char *name, int count)
{
    struct object *o = object_new();
    struct object_kind *kind = lookup_kind(tval, lookup_sval(tval,name));
    CHECK(kind);
    object_prep(o,kind,0,MINIMISE);
    o->number=count; o->known=object_new();
    object_set_base_known(player,o);
    o->known->notice |= OBJ_NOTICE_ASSESSED;
    return o;
}
bool __wrap_savefile_load(const char *path, bool cheat)
{
    struct object *arrow, *offer, *potion, *wand, *original;
    struct object *pack, **inven, **quiver;
    struct player p;
    struct player_upkeep upkeep;
    struct store normal, black;
    struct owner owner;
    struct object split;
    char before[120], after[120];
    uint64_t key;
    int i, quote, partial, normal_quote, black_quote;
    CHECK(__real_savefile_load(path,cheat));
    arrow=make(TV_ARROW,"Arrow",2); arrow->to_h=3; arrow->to_d=5;
    of_on(arrow->flags,OF_FREE_ACT); of_wipe(player->obj_k->flags);
    player->obj_k->to_h=player->obj_k->to_d=0;
    original=player->gear; player->gear=arrow;
    update_player_object_knowledge(player);
    object_desc(before,sizeof(before),arrow,ODESC_COMBAT,player);
    key=borg_rs_test_learning_identity(arrow);
    missile_learn_on_ranged_attack(player,arrow);
    object_desc(after,sizeof(after),arrow,ODESC_COMBAT,player);
    if (!strcmp(before,after)) fprintf(stderr,"partial rune: before=%s after=%s known=%d,%d player=%d,%d\n",before,after,arrow->known->to_h,arrow->known->to_d,player->obj_k->to_h,player->obj_k->to_d);
    CHECK(strcmp(before,after));
    CHECK(!object_runes_known(arrow));
    CHECK(key==borg_rs_test_learning_identity(arrow));
    player->gear=original;
    printf("{\"case\":\"partial-rune\",\"description_changed\":true,\"still_unknown\":true,\"same_learning_key\":true}\n");

    /* A full pack with 2 loaded arrows still absorbs 38 matching arrows;
       adding the 41st quiver unit needs a real pack slot. */
    p=*player; upkeep=*player->upkeep; p.upkeep=&upkeep; p.body.count=0;
    inven=mem_zalloc((z_info->pack_size+1)*sizeof(*inven));
    quiver=mem_zalloc(z_info->quiver_size*sizeof(*quiver));
    pack=mem_zalloc(z_info->pack_size*sizeof(*pack));
    potion=make(TV_POTION,"Cure Light Wounds",1);
    for(i=0;i<z_info->pack_size-1;i++) {
        pack[i]=*potion; pack[i].number=potion->kind->base->max_stack;
        pack[i].next=&pack[i+1]; inven[i]=&pack[i];
    }
    pack[z_info->pack_size-2].next=arrow;
    arrow->next=NULL; arrow->to_h=arrow->to_d=0; of_wipe(arrow->flags);
    arrow->number=2; quiver[0]=arrow;
    p.gear=pack; upkeep.inven=inven; upkeep.quiver=quiver;
    offer=make(TV_ARROW,"Arrow",39);
    CHECK(pack_slots_used(&p)==z_info->pack_size);
    CHECK(inven_quiver_stack_room(&p,offer)==38);
    arrow->number=39; CHECK(inven_quiver_stack_room(&p,offer)==1); arrow->number=2;
    CHECK(inven_carry_num(&p,offer)==38); CHECK(inven_merge_num(&p,offer)==38);
    arrow->number=40; CHECK(inven_carry_num(&p,offer)==0);
    CHECK(inven_carry_num(&p,potion)==0);
    pack[0].number--; CHECK(inven_carry_num(&p,potion)==1); CHECK(inven_merge_num(&p,potion)==1);
    printf("{\"case\":\"native-capacity\",\"quiver_join\":38,\"expansion_blocked\":true,\"full_pack_home_join\":1}\n");

    /* Quotes match do_cmd_sell's object_copy_amt path, including a device
       with non-even charges, Black Market adjustment and owner cap. */
    memset(&normal,0,sizeof(normal)); memset(&black,0,sizeof(black)); memset(&owner,0,sizeof(owner));
    owner.max_cost=30000; normal.owner=black.owner=&owner;
    normal.feat=FEAT_STORE_MAGIC; black.feat=FEAT_STORE_BLACK;
    option_set("birth_no_selling",false);
    wand=make(TV_WAND,"Magic Missile",3); wand->pval=8; wand->kind->aware=true;
    object_copy_amt(&split,wand,1); quote=price_item(&normal,&split,true,1);
    CHECK(store_sale_quote(&normal,wand,1)==quote); object_wipe(&split);
    normal_quote=store_sale_quote(&normal,wand,3); black_quote=store_sale_quote(&black,wand,3);
    CHECK(normal_quote>black_quote); CHECK(wand->number==3 && wand->pval==8);
    owner.max_cost=1; CHECK(store_sale_quote(&normal,wand,3)==3);
    option_set("birth_no_selling",true); CHECK(store_sale_quote(&normal,wand,3)==0);
    printf("{\"case\":\"native-sale\",\"partial_device_quote\":%d,\"normal\":%d,\"black_market\":%d,\"owner_cap\":3,\"no_selling\":0}\n",quote,normal_quote,black_quote);

    /* Native rod charging controls for the model's short-horizon tests. */
    wand=make(TV_ROD,"Light",2); wand->time.base=40; wand->time.dice=wand->time.sides=wand->time.m_bonus=0;
    wand->timeout=40; CHECK(number_charging(wand)==1);
    partial=wand->timeout; recharge_timeout(wand); CHECK(wand->timeout==partial-1);
    wand->timeout=80; CHECK(number_charging(wand)==2); recharge_timeout(wand); CHECK(wand->timeout==78);
    printf("{\"case\":\"native-rods\",\"one_charging\":1,\"two_charging\":2}\n");
    fflush(stdout); _Exit(0);
}
