# Visualization Next Steps

Active thread: keep the visualization pipeline deterministic and documented end-to-end.

Current objective:
1. Keep `main.rs`/`viz` in sync by regenerating data in one command flow.
2. Keep figure generation reproducible using the same camera, pole, and trajectory-selection presets.

What to run now:
1. `bash experiments/visualization/main/viz/serve.sh`
   - builds `visualization`,
   - regenerates `main/viz/data/*.json`,
   - regenerates `main/viz/data.js`,
   - starts a local HTTP server.
2. In another terminal, from `experiments/visualization/main/viz`: `node screenshot-figures.mjs`.
3. If a polytope changed, rerun just:
   - `cargo run -p visualization --release --bin visualization -- <name> experiments/visualization/main/viz/data/<name>.json`
   - `cd experiments/visualization/main/viz && bash embed-data.sh > data.js`.

Blockers:
- Playwright (`npm install playwright`) must be present for figure capture.
- Crosspolytope high-facet orbit recovery remains intentionally partial and can only be visualized via the forward path used in current data.

Stop condition:
- Data regeneration and figure regeneration succeed from the current CLI flow without manual file edits, and any changed binary/JS assumptions are reflected in `REASONING.md`, `DECISIONS.md`, and command notes.

Exact files to touch when this changes:
- `experiments/visualization/main/main.rs` (generator behavior),
- `experiments/visualization/main/viz/*.js` (projection, controls, capture presets),
- `experiments/visualization/main/viz/data.js` (generated, do not hand-edit).
