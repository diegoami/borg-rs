# Native player-known views, historical recovery and D0005 analysis

Owner requested the updated C death-analysis/replay workflow, particularly
missing available spells, then explicitly delegated D0005 analysis to a
separate agent. Rust hub#3 and focused investigation#4 are open. The original
D0005 dossier was published at9b1007c without changing its original identity.
Owner feedback6713459074 is answered by the delegated analysis in6713687354.

Tooling comparison pin: better-borg-c85b6ba8e3748e061db1ff1206163c27272d2de31.
Only observation/viewer/analysis scaffolding is adapted, with source paths,
hashes and licence in IMPORTED.json. No C game fix or milestone is adopted;
accepted m5 remains0203227. No Rust Borg algorithms or game rules are edited.

## Native views and limits

New headless/GUI recordings include player-known inventory/equipment/quiver,
labels, native descriptions and burden. ODESC_SPOIL is never used; kind/ego
seen flags changed by native description are restored. **i/e** show these
known views; **d** retains explicit actual/spoiler gear and **m** actual monster
diagnostics. No hidden diagnostic kind is substituted for a missing inventory.

**s** shows native learned/forgotten spells/prayers with current mana, native
cost/failure chance, known carried/floor book and Ready/Low mana/Confused/
Cannot see/No book/Forgotten/Dead status. Ready does not mean Borg eligibility,
selection, guaranteed success or a successful escape. Native overexertion can
still be attempted at Low mana. Native engine getters supply the observations;
no additional information is fed into the playing Borg.

**p** shows action notes/all Borg notes/game messages (Tab changes source).
Rust headless key counters and new GUI message frame boundaries avoid exposing
later events in the same world turn. Old C trace support uses conservative
completed-turn visibility; missing old GUI streams fall back to visible message
line changes. Notes can be intents/simulation; keys/native state discriminate
actual movement. Sidebar HP/SP can lag recorded actual state, and the footer
flags that without changing original screens. Panel entry pauses playback.

GUI captures new message events exactly; when declared in capture metadata,
removing the stream/hash fails validation. Each historical field that existed
continues to compare exactly. New known views are full/delta fields; no private
whole-Borg-memory shadow or universal correctness is claimed.

## D0005's original recording stays immutable

The old life did not capture these views. The adapter rebuilds the ACTUAL
historical Rust/engine source9881301, whose production input SHA matches the
original d247f691… identity. Only the uniquely located headless frame-observer
site and three declared observer headers are added. All other historical source
blobs remain unchanged; rebuild/engine/helper hashes and original source hashes
are retained. It never substitutes the newer C consumer or a fresh checkpoint.

Qualified supplemental output is separate under ignored harness/out/
death-workflow/observations. Every original key/message/player-state/base-frame
value, frame count/order, complete final save and scheduled save matches; the
native archive body matches with the existing first-line timestamp exception.
Only declared fields absent from the original may be added. Changed original
values, undeclared extra keys, changed existing observation fields or missing/
extra rows fail. A same-input observer-disabled rebuild also reproduces all
original keys/messages/state/raw frames/final save exactly. Recovered views
are labeled, not represented as the original capture or another life.

The normal interactive headless replay reuses/prepares this qualified extension.
--view-only never builds one and remains UNVERIFIED; it may use an already
qualified cache. --verify-only/dumps retain their original scope. Unreconstructible
legacy source falls back to the verified original movie without guessed data.
Historical GUI enrichment is not qualified by this adapter; new GUI captures
have views directly. Original images/manifests/dossiers are not replaced.

D0005 actually records learned Nether Bolt1MP and Sense Invisible4MP. Its
supplement contains Ready/Low mana/Confused/Dead states. The separate analysis
keeps native readiness distinct from missing exact private Borg gates. The
analysis agent's earlier qualified supplemental frames and the final cache's
frames are byte-identical, so the16 curated anchors bind to both results.

## Actual checks

- Eight controlled actual-engine states: ready, low mana, blind, confused,
  no book, forgotten, dead and noncaster. RNG state/index/quick state and
  kind/ego seen flags remain unchanged; hidden flavored item descriptions
  exclude actual-kind identity. These are diagnostic probes, not natural lives.
- Eleven view/log/strict-extension controls plus19 workflow and10 existing
  headless controls pass (40 total). Nineteen existing death/CLI controls pass.
- Twelve GUI controls pass, including required native-stream removal and a
  rehashed message mutation. GUI six-case root-script regression passes,
  preserving both exact accepted launcher save hashes. New caster capture
  and original-image replay include native inventory/spells directly.
- Actual D0005 terminal replay displays the recovered provenance badge and
  i/e/s/p panes, exits normally and does not change personal review marks.
- Normal C and Rust smoke3/3 match original key/message/state references;
  no-C-Borg check passes154 identical engine objects and actual link inputs.
- D0005 original record plus24 original artifact hashes remain unchanged.
  Analysis curation validates16 exact original/supplemental anchors,24 timeline
  boundaries,27 local links and unchanged5cases/5starts/13attempts. Original
  and supplemental observation roles remain explicit. The analysis agent
  changed no gameplay/build/tooling and made no rescue experiment.

The source/header probes intentionally control metadata prerequisites; they do
not establish natural reachability of every combination. Whole-private-memory,
per-frame whole-world/RNG byte equality, a win, survival benefit and new coverage
are not inferred. Earlier raw receipts remain immutable. No owner save/settings/
display/playable build was used; GUI validation used a private display/checkout.
A probe initially lacked init.h for z_info; that test include was corrected.

## Analysis delivery and next evidence

The full overall-assessment/timeline/brainstorm now addresses the owner: Phase
was a scroll; stair/walking attempts occurred; final play combined melees and
four bolt attempts, not continuous spell spam. One known Cure Light Wounds
remained; an immediate NW step was legal, but no complete safe route/rescue is
proved. Exact private recovery/flow/attack admission remains provisional.
Native spell confidence versus terminal Borg estimate is source-consistent
with existing canonical296, not a new C finding or an adopted fix.

Focused Rust#4 links existing C34/159/193 and296 provenance. The next ranked
work is a source-qualified private admission trace before isolated alternatives,
not immediate tactical edits. Personal owner review is separate; analysis does
not mark a replay reviewed or close unresolved mechanisms.

Full local outputs and build logs remain under harness/out/player-observations;
D5 original/final display pointers are in d5-display-final.json. This export is
a compact review receipt, not a complete executable replay bundle. The exact
historical source build/cache and original full runtime remain outside Git.
