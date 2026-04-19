# Visualization Reasoning

This topic is the local, exploratory visualization pipeline for Reeb dynamics on 4D polytopes. It is currently the source of thesis figures and manual exploration, not a durable library API surface.

`experiments/visualization/main/main.rs` is the generator. Running it with a polytope name and output path writes a JSON payload with:
- combinatorial skeleton (`vertices`, `edges`, `ridges`, `vertex_facets`),
- `dual_vertices`, `reeb_vectors`,
- recovered Reeb trajectories plus displaced variants,
- `volume` and `systolic_ratio`.

The downstream flow is fixed by local assets:
- generated JSON in `experiments/visualization/main/viz/data/*.json` is embedded by `main/viz/embed-data.sh`,
- the interactive viewer in `main/viz/index.html` consumes `window.POLYTOPE_DATA`,
- static figures are produced by `main/viz/screenshot-figures.mjs`.

The geometry transform for rendering is deliberately chosen: points are radially projected to S³ and then stereographically projected to R³. This gives a stable 3D embedding suitable for orbit and face structure, while explicitly accepting projection distortion and a controlled north-pole singularity. In practice, usability depends on a tuned pole and clipping/segment culling around it.

Current artifacts imply these working constraints:
- orbit payload is bounded for visualization by `MAX_ORBITS` in `main.rs`,
- orbit enumeration is expensive above 12 facets, so `MAX_FACETS_FOR_ORBIT` is enforced and crosspolytope is not computed the same way as others,
- some higher-action orbits are dropped during recovery because of failed quality checks (`closure_error` / `max_violation`) and are therefore absent from the viewer.

Evidence in the topic is intentionally lightweight:
- eight figure PNGs are tracked under `experiments/visualization/main/`,
- JSON data files are tracked as generated inputs to that figure pipeline,
- formal reference point is `formal/visualization/main.tex` (for write-up context).

A new agent should treat this topic as self-contained: regenerate data before edits to trajectory content, then regenerate `data.js` and figures from that regenerated data.
