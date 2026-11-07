/* Test-only engine fixture constructor. The original native C executable is
 * relinked with --wrap=savefile_save; no Borg algorithm or production object
 * is changed. Fixtures use the engine's object/level/terrain APIs.
 * SPDX-License-Identifier: GPL-2.0-only */
#include "angband.h"
#include "generate.h"
#include "game-world.h"
#include "init.h"
#include "obj-knowledge.h"
#include "obj-gear.h"
#include "obj-make.h"
#include "obj-pile.h"
#include "obj-util.h"
#include "mon-util.h"
#include "mon-make.h"
#include "object.h"
#include "player.h"
#include "player-calcs.h"
#include "player-util.h"
#include "savefile.h"
#include "trap.h"
#include "borg/borg.h"
#include "borg/borg-cave.h"
#include "borg/borg-flow-glyph.h"

extern bool __real_savefile_save(const char *path);
extern bool __real_savefile_load(const char *path, bool cheat_death);

static void describe(const char *phase)
{
    int x,y,glyphs=0,disarmable=0,runes=0,visible=0,known=-1,traps=-1;
    struct object *obj;
    for(y=0;y<cave->height;y++)for(x=0;x<cave->width;x++) {
        struct trap *t=square_trap(cave,loc(x,y));
        for(;t;t=t->next)if(trf_has(t->kind->flags,TRF_GLYPH)) {
            glyphs++;
            if(square_isseen(cave,loc(x,y)))visible++;
            if(square_isdisarmabletrap(cave,loc(x,y)))disarmable++;
        }
    }
    for(obj=player->gear;obj;obj=obj->next)
        if(obj->tval==TV_SCROLL && obj->sval==lookup_sval(TV_SCROLL,"Rune of Protection"))runes+=obj->number;
    if(getenv("BORG_SYNTHETIC_OBSERVE") && borg_grids) {
        known=traps=0;
        for(y=0;y<AUTO_MAX_Y;y++)for(x=0;x<AUTO_MAX_X;x++) {
            if(borg_grids[y][x].glyph)known++;
            if(borg_grids[y][x].trap)traps++;
        }
    }
    fprintf(stderr,"SYNTH_STATE {\"phase\":\"%s\",\"turn\":%ld,\"depth\":%d,\"max_depth\":%d,\"level\":%d,\"engine_glyphs\":%d,\"visible_engine_glyphs\":%d,\"disarmable_glyphs\":%d,\"rune_scrolls\":%d,\"borg_map_glyphs\":%d,\"borg_tracked_glyphs\":%d,\"borg_morgoth_position\":%d}\n",phase,(long)turn,player->depth,player->max_depth,player->lev,glyphs,visible,disarmable,runes,known,getenv("BORG_SYNTHETIC_OBSERVE")?track_glyph.num:-1,getenv("BORG_SYNTHETIC_OBSERVE")?borg.morgoth_position:-1);
    fflush(stderr);
    if(getenv("BORG_SYNTHETIC_OBSERVE"))
        fprintf(stderr,"SYNTH_TRAPS {\"borg_map_traps\":%d}\n",traps);
}

bool __wrap_savefile_load(const char *path, bool cheat_death)
{
    bool ok=__real_savefile_load(path,cheat_death);
    if(getenv("BORG_SYNTHETIC_INSPECT")) {
        if(!ok)_Exit(2);
        describe("loaded");
        _Exit(0); /* Inspector exits before any Borg or world action. */
    }
    return ok;
}

static void carry(int tval, const char *name, int number)
{
    struct object_kind *kind=lookup_kind(tval,lookup_sval(tval,name));
    struct object *obj=object_new();
    if(!kind)quit_fmt("synthetic: missing kind %s",name);
    object_prep(obj,kind,player->depth,RANDOMISE);
    obj->number=number;
    /* Same native knowledge/carry setup as main-borgh.c:fixture_carry(). */
    obj->origin=ORIGIN_CHEAT;
    obj->known=object_new();
    object_set_base_known(player,obj);
    object_flavor_aware(player,obj);
    obj->known->notice|=OBJ_NOTICE_ASSESSED;
    player_know_object(player,obj);
    obj->known->effect=obj->effect;
    inven_carry(player,obj,false,false);
}

bool __wrap_savefile_save(const char *path)
{
    const char *mode=getenv("BORG_SYNTHETIC_FIXTURE");
    static bool constructed;
    if(getenv("BORG_SYNTHETIC_OBSERVE") && strstr(path,"/final.save"))describe("replay-stop");
    if(!mode || constructed)return __real_savefile_save(path);
    constructed=true;
    if(!strncmp(mode,"lunal-",6)) {
        dungeon_change_level(player,20);
        prepare_next_level(player);
        player->upkeep->generate_level=false;
        on_new_level();
        player->max_depth=!strcmp(mode,"lunal-shallow-history")?34:99;
        player->recall_depth=player->max_depth;
    } else if(!strncmp(mode,"glyph-",6)) {
        struct monster *boss=NULL;
        struct loc center=loc(100,33),far=loc(165,55);
        int i,x,y;
        if(player->depth!=100)quit("synthetic glyph fixture requires depth100");
        for(i=cave_monster_max(cave)-1;i>=1;i--) {
            struct monster *m=cave_monster(cave,i);
            if(!m->race)continue;
            if(streq(m->race->name,"Morgoth, Lord of Darkness"))boss=m;
            else delete_monster_idx(cave,i);
        }
        if(!boss)quit("synthetic: Morgoth absent");
        for(i=1;i<z_info->r_max;i++) {
            if(rf_has(r_info[i].flags,RF_UNIQUE) && &r_info[i]!=boss->race)
                r_info[i].max_num=0;
        }
        square_set_feat(cave,far,FEAT_FLOOR);
        monster_swap(boss->grid,far);
        for(y=20;y<=46;y++)for(x=85;x<=115;x++)
            square_set_feat(cave,loc(x,y),FEAT_FLOOR);
        monster_swap(player->grid,center);
        carry(TV_SCROLL,"Rune of Protection",
            (!strcmp(mode,"glyph-ready") || !strcmp(mode,"glyph-complete"))?20:9);
        carry(TV_POTION,"*Healing*",20);
        if(!strcmp(mode,"glyph-complete")) {
            for(y=-2;y<=2;y++)for(x=-2;x<=2;x++)
                if(x || y)square_add_glyph(cave,loc(center.x+x,center.y+y),GLYPH_WARDING);
        } else if(!strcmp(mode,"glyph-visible-rune")) {
            square_add_glyph(cave,loc(102,33),GLYPH_WARDING);
        } else if(!strcmp(mode,"glyph-decoy")) {
            square_add_glyph(cave,loc(102,33),GLYPH_DECOY);
        } else if(!strcmp(mode,"glyph-visible-trap")) {
            place_trap(cave,loc(102,33),lookup_trap("pit")->tidx,100);
            square_reveal_trap(cave,loc(102,33),true,false);
        } else if(!strcmp(mode,"glyph-hidden-rune")) {
            for(y=20;y<=46;y++)square_set_feat(cave,loc(105,y),FEAT_GRANITE);
            square_add_glyph(cave,loc(110,33),GLYPH_WARDING);
        }
    } else quit_fmt("unknown synthetic fixture: %s",mode);
    player->upkeep->update|=PU_BONUS|PU_HP|PU_MANA|PU_INVEN|PU_UPDATE_VIEW|PU_MONSTERS;
    handle_stuff(player);
    player->chp=player->mhp;
    player->csp=player->msp;
    fprintf(stderr,"SYNTHETIC mode=%s depth=%d max_depth=%d level=%d grid=%d,%d\n",mode,player->depth,player->max_depth,player->lev,player->grid.x,player->grid.y);
    describe(mode);
    return __real_savefile_save(path);
}
