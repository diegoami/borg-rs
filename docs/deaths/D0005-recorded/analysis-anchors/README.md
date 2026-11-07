# Supplemental D0005 analysis anchors

These 16 exact-frame anchors are derived from the separately qualified
historical-source native observation replay described in
[analysis-evidence.json](../analysis-evidence.json). They are supplemental
observations of the original life, not original inventory/spell captures,
new independent deaths, alternate-action branches or replacements for the
immutable `snapshots/`.

Filenames retain actual world turn and one-based frame index. Each JSON pairs
the exact original base screen/player state with supplemental player-known
inventory and native learned spell costs/failures/readiness. Text files show
the same reconstructed terminal screen. Hidden diagnostic monster positions
are excluded. Native spell readiness does not establish private Borg
eligibility. Terminal map geometry does not expose the full private map or
prove safe escape.

| Turn | Frame | Observation |
| --- | --- | --- |
| 36014 | [2110](t36014-f2110.json) | Arrival at the new D1 upstairs, after earlier same-turn town/descent frames. |
| 36024 | [2112](t36024-f2112.json) | Beginning item pursuit. |
| 36094 | [2119](t36094-f2119.json) | Fang first encounter, before first bolt completes. |
| 36104 | [2130](t36104-f2130.json) | After first bolt; 3MP. |
| 36114 | [2142](t36114-f2142.json) | Before last Phase scroll; 2MP. |
| 36124 | [2147](t36124-f2147.json) | Actual Phase landing. |
| 36144 | [2150](t36144-f2150.json) | Before successful NW step. |
| 36154 | [2151](t36154-f2151.json) | On NW destination, kobold blocks continuation. |
| 36174 | [2153](t36174-f2153.json) | Second logged stair-flow episode. |
| 36194 | [2155](t36194-f2155.json) | Fang reappears; before third bolt. |
| 36204 | [2168](t36204-f2168.json) | After failed third bolt; 1MP. |
| 36224 | [2171](t36224-f2171.json) | Before final retreat to(191,28). |
| 36234 | [2172](t36234-f2172.json) | First injured final fight; legal NW candidate, CLW, 1MP. |
| 36263 | [2186](t36263-f2186.json) | HP4/MP1; before last bolt. |
| 36273 | [2198](t36273-f2198.json) | HP2/MP0, CLW still carried. |
| 36282 | [2203](t36282-f2203.json) | HP0/MP0, living before last melee. |

The actual death frame is2210 at36289, phase`death`, HP-2. Its exact metadata
is retained in analysis-evidence.json; original terminal evidence remains in
the original dossier/raw recording.
