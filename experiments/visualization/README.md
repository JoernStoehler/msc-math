# Visualization of Reeb Dynamics

## Purpose

Build geometric intuition about Reeb flows on polytope boundaries by visualizing 4D polytopes and their Reeb trajectories via stereographic projection to 3D.

## Pipeline

```
viz_export (Rust) → data.js → viz/index.html (Three.js) → screenshot-figures.mjs (Playwright) → figures
```

## Design

- Radial projection maps the polytope boundary to S^3
- Stereographic projection maps S^3 to R^3 for rendering
- Configurable north pole placement avoids sending edges to infinity
- Interactive controls: toggle edges, ridges, vertices, trajectories; select north pole preset; rotate camera

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
- `viz-hypercube-traj.png` — single Reeb trajectory on hypercube
- `viz-simplex-traj.png` — single Reeb trajectory on simplex
- `viz-hko-pentagon-edges.png` — HK-O pentagon edge skeleton
- `viz-hko-pentagon-traj.png` — Reeb trajectory on HK-O pentagon
- `viz-lagrangian-tri-product-traj.png` — Reeb trajectory on triangle product

## Files

| File | Description |
|------|-------------|
| viz_export.rs | Rust binary — exports polytope JSON for viewer |
| viz/ | Interactive Three.js viewer (index.html, viz.js, etc.) |
| viz/screenshot-figures.mjs | Playwright screenshot automation |
| viz-*.png | Generated thesis figures |
| visualization.tex | Thesis writeup |
