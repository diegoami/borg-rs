# Maintained launcher save/reload proof

The original driver in docs/validation/2026-11-07-launcher is a frozen M4
receipt and retains its historical timing race. Use this maintained driver
for new checks. It waits for process exit after each Ctrl-X and accepts
BadWindow only if the game actually exits0; file replacement, level50 and
both exact archived save hashes must still pass. A failed key send with a
live child or a nonzero game exit remains an error.

```sh
python3 harness/regress/launcher-save/test_race.py
git worktree add --detach /tmp/borg-launcher-check m4
python3 harness/regress/launcher-save/run.py NEW_OUTPUT --project-root /tmp/borg-launcher-check --runs 5
```

The project-root argument is required so callers select an isolated build.
The driver builds there before window timeouts start, starts its own local
Xvfb display with a readiness pipe, and uses disposable fixture profiles.
It does not connect to the owner's display or open the owner's save. The
fixed private display number permits Xvfb's abstract socket when the shared
filesystem socket directory is read-only; that directory is not modified.
Dependencies: the normal launcher build tools, Xvfb and xdotool.
