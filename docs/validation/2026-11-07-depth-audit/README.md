# Depth reporting audit at production source28b8170

The per-key engine traces show deepest logged current depths of52 for
Warrior2000,59 for Blackguard3100 and99 for Warrior2007. Earlier notes
quoted the final summary max_depth values44/51/99. That field can be reset
by the recall-depth choice in effect-handler-general.c (around1133), so
it is not an all-time peak. This corrects the depth description; production
code, seeds, limits, saved fixtures and C references are unchanged.

These are Borg-played cheat-death observations. They do not establish
unassisted survival or a win. Natural fresh replays remain84/41 at their
deepest logged current depth. Per-key observations are the claimed sampling
scope; private Borg memory is not inferred from them.

The inputs are the accepted source28 corrected C references, whose full
state logs already matched two C passes and Rust in the ordinary checkpoint.
Current coverage collection is separate and remains incomplete at audit time.
SOURCES.json pins the compressed state logs and final summaries. Reproduce
from the project root with `python3 docs/validation/2026-11-07-depth-audit/audit.py`;
the script writes its result in harness/out/local/depth-audit.json.

| scenario | deepest logged current depth | final summary max_depth |
|---|---:|---:|
| extended/ext-blackguard | 19 | 19 |
| extended/ext-druid | 19 | 19 |
| extended/ext-long-blackguard-3100 | 59 | 51 |
| extended/ext-long-warrior-2000 | 52 | 44 |
| extended/ext-long-warrior-2007 | 99 | 99 |
| extended/ext-mage | 23 | 23 |
| extended/ext-necro | 3 | 3 |
| extended/ext-paladin | 31 | 31 |
| extended/ext-priest | 14 | 4 |
| extended/ext-ranger | 36 | 36 |
| extended/ext-rogue | 27 | 6 |
| extended/ext-warrior | 34 | 34 |
| natural/natural-blackguard-40 | 41 | 41 |
| natural/natural-warrior-78 | 84 | 84 |
