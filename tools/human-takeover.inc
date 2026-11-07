/* Observer/control overlay for a source-pinned historical harness.
 * GPL-2.0-or-later or Angband licence; see angband/docs/copying.rst.
 * Included after borgh_finish(), before borgh_checkpoint(). */
void borgh_human_dump(FILE *fp)
{
    int i;
    fprintf(fp, "{\"turn\":%ld,\"x\":%d,\"y\":%d,\"depth\":%d,"
        "\"level\":%d,\"hp\":%d,\"sp\":%d,\"max_hp\":%d,\"max_sp\":%d,"
        "\"energy\":%d,\"total_energy\":%lu,\"race\":%u,\"class\":%u,"
        "\"dead\":%s,\"borg_active\":%s,\"rng_index\":%u,\"rng\":[",
        (long)turn, player->grid.x, player->grid.y, player->depth,
        player->lev, player->chp, player->csp, player->mhp, player->msp,
        player->energy, (unsigned long)player->total_energy,
        player->race->ridx, player->class->cidx,
        player->is_dead ? "true" : "false", inkey_hack ? "true" : "false", state_i);
    for (i = 0; i < RAND_DEG; i++) fprintf(fp, "%s%u", i ? "," : "", STATE[i]);
    fputs("]}\n", fp);
}

static void human_ready(void)
{
    const char *ready = getenv("BORG_HUMAN_READY");
    const char *approve = getenv("BORG_HUMAN_APPROVE");
    FILE *fp;
    int waited = 0;
    /* Never inherit an automated command source in a human session. */
    if (inkey_hack || !player || player->is_dead || !character_dungeon ||
        player->upkeep->generate_level) quit("human takeover: unsafe loaded state");
    if (!ready || !approve) quit("human takeover: missing handshake paths");
    fp = fopen(ready, "w");
    if (!fp) quit("human takeover: cannot write ready state");
    borgh_human_dump(fp);
    if (ferror(fp) || fclose(fp)) quit("human takeover: ready write failed");
    /* The launcher validates before any human command can be accepted.
     * Frozen GUI replay is noninteractive and validates this output later. */
    while (!getenv("BORG_REPLAY_DIR") && !file_exists(approve)) {
        if (++waited > 300) quit("human takeover: approval handshake timed out");
        usleep(100000);
    }
    borgh_command_hook = NULL;
}

void borgh_human_mode_init(void)
{
    const char *seed = getenv("BORG_HUMAN_SEED");
    if (!seed) return;
    /* Same nonholiday calendar as the original seeded headless run.
     * Set before initialization; never reseed the loaded complex RNG. */
    borgh_seed = (uint32_t)strtoul(seed, NULL, 10);
    borgh_seed_set = true;
    inkey_hack = NULL;
    borgh_command_hook = human_ready;
}

static void human_capture(void)
{
    const char *requested = getenv("BORG_TAKEOVER_TURN");
    const char *directory = getenv("BORG_TAKEOVER_DIR");
    char path[2048];
    pid_t child, parent;
    int status;
    FILE *fp;
    if (!requested || !directory || !borg_started || !keys_logged || !player ||
        player->is_dead || !character_dungeon || player->upkeep->generate_level ||
        turn < strtol(requested, NULL, 10)) return;
    path_build(path, sizeof(path), directory, "captured-state.json");
    fp = fopen(path, "w");
    if (!fp) quit("human takeover: cannot write capture metadata");
    borgh_human_dump(fp);
    if (ferror(fp) || fclose(fp)) quit("human takeover: capture metadata write failed");
    path_build(path, sizeof(path), directory, "boundary.json");
    fp = fopen(path, "w");
    if (!fp) quit("human takeover: cannot write boundary");
    fprintf(fp, "{\"requested_turn\":%ld,\"actual_turn\":%ld,\"keys_before\":%lu,"
        "\"phase\":\"before_game_command\"}\n", strtol(requested, NULL, 10), (long)turn, keys_logged);
    if (ferror(fp) || fclose(fp)) quit("human takeover: boundary write failed");
    path_build(path, sizeof(path), directory, "snapshot.save");
    if (file_exists(path)) quit("human takeover: refusing to replace a snapshot");
    /* Native save compacts monster indices: isolate it from replay state. */
    parent = getpid();
    child = fork();
    if (child < 0) quit("human takeover: fork failed");
    if (!child) {
#ifdef __linux__
        if (prctl(PR_SET_PDEATHSIG, SIGKILL) || getppid() != parent) _exit(3);
#else
        (void)parent;
#endif
        if (keylog_fp) close(fileno(keylog_fp));
        if (msglog_fp) close(fileno(msglog_fp));
        if (statelog_fp) close(fileno(statelog_fp));
        if (checkpoint_fp) close(fileno(checkpoint_fp));
        if (frames_fp) close(fileno(frames_fp));
        keylog_fp = msglog_fp = statelog_fp = checkpoint_fp = frames_fp = NULL;
        _exit(savefile_save(path) ? 0 : 3);
    }
    while (waitpid(child, &status, 0) < 0) {
        if (errno != EINTR) quit("human takeover: wait failed");
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status)) quit("human takeover: snapshot failed");
    borgh_finish("human_takeover");
}
