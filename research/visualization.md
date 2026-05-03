# Visualization Research Note

## Scope

This topic is the local, exploratory visualization pipeline for 4D Reeb dynamics on
polytopes. It is the source of thesis figures and manual exploration content, and it
is not a reusable library API surface.

## Current State

The binary in `experiments/visualization/main/main.rs` is the generator for this topic.
It writes a JSON payload containing combinatorial data (`vertices`, `edges`, `ridges`,
`vertex_facets`), geometric values (`dual_vertices`, `reeb_vectors`), trajectory
payloads (recovered plus displaced variants), and summary values (`volume`,
`systolic_ratio`).

The downstream rendering pipeline is fixed in `experiments/visualization/main/viz/`:
- `main/viz/embed-data.sh` embeds generated JSON from
  `main/viz/data/*.json`.
- `main/viz/index.html` consumes `window.POLYTOPE_DATA`.
- `main/viz/screenshot-figures.mjs` produces static figures.

Rendering uses radial projection from 4D to S3 followed by stereographic projection to
R3. This keeps face and orbit structure visually usable but accepts controlled
distortion and a north-pole singularity; practical use depends on pole tuning and
clipping around the singularity.

Current runtime constraints are enforced in `main.rs`:
- `MAX_ORBITS` bounds orbit payload size.
- `MAX_FACETS_FOR_ORBIT = 12` bounds expensive orbit enumeration; crosspolytope is
  handled with existing special handling instead of full enumeration.
- trajectory filters drop high-action orbit recoveries when `closure_error` or
  `max_violation` quality checks fail.

Tracked artifacts remain under `experiments/visualization/main/`, including eight PNG
figure outputs and generated JSON inputs used by the figure pipeline.

## Evidence And Interpretation

- The generator/output contract above is currently the durable contract for this topic.
- `research/visualization.md` references this experiment for write-up context and
  empirical grounding.
- `MAX_ORBITS` and `MAX_FACETS_FOR_ORBIT` are evidence of an intentional data-size
  and browser-feasibility tradeoff.
- The crosspolytope visual path is intentionally partial for high-facet orbits
  because recovery can fail and is filtered at the quality-check layer.

## Decisions

- Keep visualization as an experiment-local, non-crate topic with local assets in
  `main/viz/`; do not move it into reusable crate API form.
- Keep known polytopes explicit in command entry points for reproducibility by name.
- Keep payload bounds (`MAX_ORBITS`, `MAX_FACETS_FOR_ORBIT`) and the fallback handling
  for crosspolytope as the current default.
- Keep projection as radial projection + stereographic projection for interactive speed and
  acceptable geometric readability.
- Keep configurable projection pole and clipping behavior (`MAX_RADIUS` and pole tuning)
  as the control mechanism for near-pole artifacts.
- Keep screenshot automation in `main/viz/screenshot-figures.mjs` as the canonical figure
  path to avoid drift from manual captures.
- Keep formal/experimental boundaries explicit: reusable correctness claims live in
  `formal/`, while this topic remains exploratory and presentation-oriented.

Rejected routes and explicit constraints remembered:
- Do not restore the removed `docs/viz` deployment path.
- Do not rely on GitHub Pages links for the active workflow.
- Do not assume closed-form recovery for all high-action candidates; filtered outputs are
  expected by design.

## History

- Previous mixed operation notes are collapsed into this consolidated research note.
- This topic kept explicit local orchestration with `main/viz` rather than a docs deployment
  path.
- The topic remains constrained by intentional orbit-coverage filtering where recovery fails.

## Next Steps

- Keep the CLI and `viz` workflow synchronized by treating data regeneration and figure
  generation as one documented flow:
  1. `bash experiments/visualization/main/viz/serve.sh`
  2. `node screenshot-figures.mjs` from `experiments/visualization/main/viz`
- If geometry changes, rerun:
  - `cargo run -p visualization --release --bin visualization -- <name> experiments/visualization/main/viz/data/<name>.json`
  - `cd experiments/visualization/main/viz && bash embed-data.sh > data.js`
- Keep Playwright installed for figure capture and runbooks.
- Continue using the `MAX_ORBITS` and `MAX_FACETS_FOR_ORBIT` controls unless there is a
  specific need to change reproducible presets.
