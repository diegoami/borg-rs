/* Controlled native-engine observation probe; no Borg actions or save edits.
 * GPL-2.0-or-later or Angband licence; see angband/docs/copying.rst. */
#include "angband.h"
#include "player.h"
#include "game-world.h"
#include "object.h"
#include "player-timed.h"
#include "savefile.h"
#include "z-color.h"
#include "z-quark.h"
#include "cave.h"
#include "init.h"
#include "player-calcs.h"

static FILE *probe_stream;
static unsigned long long mix_number(unsigned long long hash, long long value)
{
    int i;
    for (i = 0; i < 8; i++) { hash = (hash ^ ((unsigned long long)value & 255)) * 1099511628211ULL; value >>= 8; }
    return hash;
}
static void json_string(const char *text)
{
    const unsigned char *p = (const unsigned char *)(text ? text : "");
    fputc('"', probe_stream);
    for (; *p; ++p) {
        if (*p == '"' || *p == '\\') fprintf(probe_stream, "\\%c", *p);
        else if (*p < 32) fprintf(probe_stream, "\\u%04x", *p);
        else fputc(*p, probe_stream);
    }
    fputc('"', probe_stream);
}
#define REPLAY_OBSERVATION_STREAM probe_stream
#define REPLAY_ADD_VITALS
#include "replay-observations.h"
#undef REPLAY_ADD_VITALS
#undef REPLAY_OBSERVATION_STREAM

static void capture(const char *name)
{
    uint32_t old_state[RAND_DEG], old_index = state_i, old_value = Rand_value;
    bool old_quick = Rand_quick;
    struct object *obj;
    int unknown_checked = 0;
    memcpy(old_state, STATE, sizeof(old_state));
    fprintf(probe_stream, "{\"case\":"); json_string(name);
    replay_extra_observations(true);
    for (obj = player->gear; obj; obj = obj->next) {
        if (obj->kind->flavor && !obj->kind->aware) {
            struct replay_item_row row;
            replay_item(&row, obj, -1);
            if (strstr(row.description, obj->kind->name)) _Exit(6);
            unknown_checked++;
        }
    }
    fprintf(probe_stream, ",\"unknown_descriptions_checked\":%d}\n", unknown_checked);
    if (memcmp(old_state, STATE, sizeof(old_state)) || old_index != state_i || old_value != Rand_value || old_quick != Rand_quick)
        _Exit(4);
    for (obj = player->gear; obj; obj = obj->next)
        if (obj->kind->everseen || (obj->ego && obj->ego->everseen)) _Exit(5);
}
extern bool __real_savefile_load(const char *, bool);
bool __wrap_savefile_load(const char *path, bool cheat)
{
    int i;
    struct object *obj, *gear;
    if (!__real_savefile_load(path, cheat)) _Exit(2);
    probe_stream = stdout;
    calc_inventory(player);
    for (obj = player->gear; obj; obj = obj->next) {
        obj->kind->everseen = false;
        if (obj->ego) obj->ego->everseen = false;
    }
    if (!player->class->magic.total_spells) {
        capture("noncaster"); fflush(stdout); _Exit(0);
    }
    for (i = 0; i < player->class->magic.total_spells; i++) player->spell_flags[i] = 0;
    player->spell_flags[0] = PY_SPELL_LEARNED;
    player->timed[TMD_BLIND] = player->timed[TMD_CONFUSED] = 0;
    sqinfo_on(square(cave, player->grid)->info, SQUARE_SEEN);
    player->csp = player->msp = 100;
    capture("ready");
    player->csp = 0; capture("low_mana"); player->csp = 100;
    player->timed[TMD_BLIND] = 1; capture("blind"); player->timed[TMD_BLIND] = 0;
    player->timed[TMD_CONFUSED] = 1; capture("confused"); player->timed[TMD_CONFUSED] = 0;
    gear = player->gear; player->gear = NULL; capture("no_book"); player->gear = gear;
    player->spell_flags[0] = PY_SPELL_FORGOTTEN; capture("forgotten");
    player->spell_flags[0] = PY_SPELL_LEARNED; player->is_dead = true; capture("dead");
    fflush(stdout); _Exit(0);
}
