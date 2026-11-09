/* staleglyph: the Borg's map update on a grid whose rune of protection the
 * player still remembers, although the rune itself is gone (broken while
 * out of view).  better-borg-c patch 0098 (runeglyph-295) takes the glyph
 * test from the player's memory but then reads the cave's own trap:
 * square(cave, l)->trap->t_idx.  With the rune gone that trap is NULL.
 *
 * Modes (SG_MODE): "stale" removes the rune out of view before the update;
 * "fresh" leaves it in place (control).  SG_PROFILE is the user directory.
 * Pass: the update returns and the Borg's map is consistent with what the
 * player knows; the scenario prints OUTCOME and its finished line. */
#include "unit-test.h"
#include "test-utils.h"
#include "cave.h"
#include "game-world.h"
#include "generate.h"
#include "init.h"
#include "obj-gear.h"
#include "obj-knowledge.h"
#include "mon-util.h"
#include "obj-make.h"
#include "obj-pile.h"
#include "obj-util.h"
#include "player.h"
#include "player-birth.h"
#include "player-calcs.h"
#include "player-util.h"
#include "trap.h"
#include "ui-term.h"
#include "borg/borg.h"
#include "borg/borg-cave.h"
#include "borg/borg-flow-glyph.h"
#include "borg/borg-init.h"
#include "borg/borg-inventory.h"
#include "borg/borg-trait.h"
#include "borg/borg-update.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static term screen;
static const char *mode;
static struct loc rune;

static struct object *known_item(int tval, const char *name, int number)
{
    struct object *obj = object_new();
    struct object_kind *kind = lookup_kind(tval, lookup_sval(tval, name));
    if (!kind) quit("fixture item kind missing");
    object_prep(obj, kind, 50, MAXIMISE);
    obj->number = number;
    obj->known = object_new();
    object_set_base_known(player, obj);
    object_flavor_aware(player, obj);
    inven_carry(player, obj, false, false);
    object_touch(player, obj);
    player_know_object(player, obj);
    return obj;
}

static int glyph_tidx(void)
{
    struct trap_kind *kind = lookup_trap("glyph of warding");
    if (!kind) quit("glyph trap kind missing");
    return kind->tidx;
}

int setup_tests(void **state)
{
    const char *profile = getenv("SG_PROFILE");
    setvbuf(stdout, NULL, _IONBF, 0);
    mode = getenv("SG_MODE");
    if (!profile || !mode) return 1;
    set_file_paths();
    string_free(ANGBAND_DIR_USER); ANGBAND_DIR_USER = string_make(profile);
    string_free(ANGBAND_DIR_SAVE); ANGBAND_DIR_SAVE = string_make(format("%s/save", profile));
    string_free(ANGBAND_DIR_PANIC); ANGBAND_DIR_PANIC = string_make(format("%s/panic", profile));
    string_free(ANGBAND_DIR_SCORES); ANGBAND_DIR_SCORES = string_make(format("%s/scores", profile));
    string_free(ANGBAND_DIR_ARCHIVE); ANGBAND_DIR_ARCHIVE = string_make(format("%s/archive", profile));
    if (!init_angband()) { printf("SETUPFAIL init\n"); return 1; }
    create_needed_dirs();
    Rand_quick = false; Rand_state_init(295);
    if (!player_make_simple("Human", "Priest", "Staleglyph295")) { printf("SETUPFAIL birth\n"); return 1; }
    term_init(&screen, 80, 24, 256); Term_activate(&screen); angband_term[0] = &screen;

    /* An unlit open level: only the lantern lights grids. */
    cave = t_build_arena(AUTO_MAX_Y, AUTO_MAX_X);
    player->cave = cave_new(AUTO_MAX_Y, AUTO_MAX_X);
    player->depth = player->max_depth = cave->depth = player->cave->depth = 10;
    character_dungeon = true;
    character_generated = true;
    player_place(cave, player, loc(30, 10));
    struct object *lantern = known_item(TV_LIGHT, "Lantern", 1);
    lantern->timeout = 2000;
    inven_wield(lantern, wield_slot(lantern));
    player->upkeep->update |= PU_BONUS | PU_HP | PU_TORCH | PU_INVEN;
    update_stuff(player);
    player->chp = player->mhp;

    /* A rune next to the Borg, seen (lit by the lantern) and remembered. */
    rune = loc(31, 10);
    square_add_glyph(cave, rune, GLYPH_WARDING);
    update_view(cave, player);
    square_memorize_traps(cave, rune);

    borg_init();
    if (borg_init_failure) { printf("SETUPFAIL borg\n"); return 1; }
    Term->offset_y = 0; Term->offset_x = 0;
    borg_equipment(); borg_inventory(); borg_notice(true);
    borg.c = player->grid;
    borg_update();
    printf("SETUP mode=%s rune=(%d,%d) seen=%d remembered=%d borg_glyph=%d tracked=%d\n",
        mode, rune.x, rune.y, square_isseen(cave, rune) ? 1 : 0,
        square_trap(player->cave, rune) ? 1 : 0, borg_grids[rune.y][rune.x].glyph ? 1 : 0,
        track_glyph.num);
    return 0;
}

int teardown_tests(void *state)
{
    cleanup_angband(); term_nuke(&screen); angband_term[0] = NULL; return 0;
}

static int scenario(void *state)
{
    /* The Borg steps away: the rune is no longer lit or seen. */
    monster_swap(player->grid, loc(40, 10));
    update_view(cave, player);
    borg.c = player->grid;
    printf("STATE moved borg=(%d,%d) rune_seen=%d\n", borg.c.x, borg.c.y,
        square_isseen(cave, rune) ? 1 : 0);
    require(!square_isseen(cave, rune));

    if (streq(mode, "stale")) {
        /* As a monster breaking the rune does (mon-move.c). */
        square_remove_all_traps_of_type(cave, rune, glyph_tidx());
    }
    printf("STATE cave_trap=%d remembered=%d\n", square(cave, rune)->trap ? 1 : 0,
        square_trap(player->cave, rune) ? 1 : 0);
    require(square_trap(player->cave, rune));

    /* The Borg's next map update covers the rune's grid. */
    borg_update();
    printf("OUTCOME survived=1 borg_glyph=%d borg_trap=%d tracked=%d\n",
        borg_grids[rune.y][rune.x].glyph ? 1 : 0, borg_grids[rune.y][rune.x].trap ? 1 : 0,
        track_glyph.num);
    ok;
}

const char *suite_name = "native-c/staleglyph-0098";
struct test tests[] = {
    {"map_update_with_remembered_rune", scenario},
    {NULL, NULL}};
