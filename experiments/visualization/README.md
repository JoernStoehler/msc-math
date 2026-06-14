# Visualization

This package owns the local visualization producer and checked-in viewer assets
for the 4D Reeb dynamics side result. Use
`thesis/visualization-3d-content.md` for thesis-facing framing and figure
selection notes.

## Rust Command Contract

- `visualization` exports one named known polytope to the JSON path passed on
  the command line.
- `main/viz/serve.sh` is the full local refresh path. It builds the binary,
  regenerates the eight JSON inputs in `main/viz/data/`, embeds them into
  `main/viz/data.js`, and starts a local HTTP server.
- `main/viz/embed-data.sh` only rebuilds `main/viz/data.js` from existing JSON
  files. Use it after regenerating a subset of `main/viz/data/*.json`.
- `main/viz/screenshot-figures.mjs` captures the static PNG figures from a
  running viewer. It currently writes seven tracked PNG files under `main/`.

Tracked JSON and PNG files in this package are generated artifacts. Regenerate
them from the commands above instead of editing them directly.

## Current Artifact Set

- JSON viewer inputs: `main/viz/data/*.json`, currently eight files.
- Embedded viewer data: `main/viz/data.js`, generated from the JSON inputs.
- Static thesis-candidate figures: `main/viz-*.png`, currently seven files.

Crosspolytope orbit trajectories use the placeholder trajectory path because
the visualization producer skips full orbit enumeration above
`MAX_FACETS_FOR_ORBIT = 12`. Its capacity/source metadata still comes from
`known_polytopes::crosspolytope()`.

## Interpretation Boundary

The visualization work is exploratory. It supports figures and manual
inspection, but it has not produced a reliable visual candidate rule or proof
input. Thesis-facing wording belongs in `thesis/visualization-3d-content.md`
and `thesis/visualization-3d.tex`.
