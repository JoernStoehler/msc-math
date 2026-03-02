# Visualization of Reeb Dynamics

## Purpose

Build geometric intuition about Reeb flows on polytope boundaries by visualizing 4D polytopes and their Reeb trajectories via stereographic projection to 3D.

## Pipeline

```
viz_export (Rust) → data.js → viz/index.html (Three.js) → screenshot-figures.mjs (Playwright) → figures
```

## Trajectory types

The viz export generates three types of trajectories per polytope:

1. **Minimum-action Reeb orbits** — recovered from the HK2017 algorithm's dual solution (S, σ, β) via `recover_base_point()`. These are the closed orbits achieving c_EHZ(K). Multiple orbits may share the same minimum action.

2. **Higher-action Reeb orbits** — other valid closed orbits found during exhaustive enumeration. Some higher-action orbits fail recovery (high max_violation from underdetermined base point systems) and are skipped. Total orbits per polytope capped at 20 (including min-action orbits).

3. **Displaced orbits** — perturbations of the min-action orbit's base point by ε in a direction tangent to the starting facet and perpendicular to the Reeb vector. Shows how nearby trajectories twist relative to the minimum-action orbit. These may not close (combinatorics can diverge for the displaced starting point).

For the crosspolytope (F=16), orbit computation is skipped due to exponential cost. A placeholder forward-simulated trajectory is used instead.

## Design

- Radial projection maps the polytope boundary to S^3
- Stereographic projection maps S^3 to R^3 for rendering
- Configurable north pole placement avoids sending edges to infinity
- Interactive controls: toggle edges, ridges, vertices, trajectories; select north pole preset; rotate camera
- Each trajectory has a label shown in the UI checkbox panel

## Screenshot automation

`viz/screenshot-figures.mjs` uses Playwright (headless Chrome) to generate deterministic screenshots with fixed camera positions and display settings. Requires:

```bash
npm install playwright
cd experiments/visualization/viz
npx serve -l 8080 &
node screenshot-figures.mjs
```

Generated figures:
- `viz-hypercube-edges.png` — hypercube edge skeleton
- `viz-hypercube-ridges.png` — hypercube with ridge surfaces
- `viz-hypercube-traj.png` — Reeb trajectory on hypercube
- `viz-simplex-traj.png` — Reeb trajectory on simplex
- `viz-hko-pentagon-edges.png` — HK-O pentagon edge skeleton
- `viz-hko-pentagon-traj.png` — Reeb trajectory on HK-O pentagon
- `viz-lagrangian-tri-product-traj.png` — Reeb trajectory on triangle product

## Regeneration

```bash
cd experiments/
# Export all polytopes (crosspolytope gets placeholder trajectory automatically)
for name in simplex hypercube hko_pentagon lagrangian_triangle_product symplectic_triangle_product lagrangian_tri_sq symplectic_tri_sq crosspolytope; do
  cargo run --release --bin viz_export -- "$name" "visualization/viz/data/$name.json"
done
# Embed into data.js
cd visualization/viz && bash embed-data.sh > data.js
```

## Files

| File | Description |
|------|-------------|
| viz_export.rs | Rust binary — exports polytope JSON with real Reeb orbits |
| viz/ | Interactive Three.js viewer (index.html, viz.js, etc.) |
| viz/data/*.json | Per-polytope JSON data (intermediate, regenerated) |
| viz/data.js | Embedded data for viewer (auto-generated from JSON files) |
| viz/screenshot-figures.mjs | Playwright screenshot automation |
| viz-*.png | Generated thesis figures |
| visualization.tex | Thesis writeup |

## Orbit statistics (as of last regeneration)

"Total orbits" = certified (S,σ,β) solutions found by exhaustive enumeration before capping.

| Polytope | Facets | Total orbits | Min-action orbits | Action value |
|----------|--------|-------------|-------------------|-------------|
| simplex | 5 | 6 | 6 | 0.2500 |
| hypercube | 8 | 142 | 2 | 4.0000 |
| hko_pentagon | 10 | 718 | 20+ | 3.4410 |
| lagrangian_triangle_product | 6 | 8 | 2 | 1.5000 |
| symplectic_triangle_product | 6 | 31 | 2 | 1.2990 |
| lagrangian_tri_sq | 7 | 4 | 2 | 1.5000 |
| symplectic_tri_sq | 7 | 62 | 1 | 1.0000 |
| crosspolytope | 16 | — | — | — (placeholder) |
