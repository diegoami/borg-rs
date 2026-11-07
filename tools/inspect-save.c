/* Engine-only metadata reader linked with the verified Rust executable's objects.
 * GPL-2.0-or-later or Angband licence; see angband/docs/copying.rst. */
#include "angband.h"
#include "player.h"
#include "savefile.h"
#include "game-world.h"
extern bool __real_savefile_load(const char *, bool);
bool __wrap_savefile_load(const char *path, bool cheat)
{
    if(!__real_savefile_load(path,cheat)) _Exit(2);
    printf("{\"race\":%u,\"class\":%u,\"turn\":%ld,\"level\":%d,\"dead\":%s}\n",
        player->race->ridx,player->class->cidx,(long)turn,player->lev,player->is_dead?"true":"false");
    fflush(stdout);
    _Exit(0);
}
