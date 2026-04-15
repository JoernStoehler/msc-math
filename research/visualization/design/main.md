# Visualization of Reeb Dynamics: Logbook

## Motivation

To build geometric intuition about Reeb flows on polytope boundaries, this experiment visualizes 4D polytopes and their Reeb orbits via stereographic projection to 3D. The interactive visualization serves both thesis figures and exploratory understanding.

## Status

**Complete.** Eight polytopes exported and visualized. Thesis figures generated via Playwright screenshot automation. Interactive viewer in `viz/` (run `viz/serve.sh` to start local server).

## How to run

```bash
cargo run -p visualization --release --bin visualization

# Export all polytopes
for name in simplex hypercube hko_pentagon lagrangian_triangle_product symplectic_triangle_product lagrangian_tri_sq symplectic_tri_sq crosspolytope; do
  cargo run -p visualization --release --bin visualization -- "$name" "viz/data/$name.json"
done

# Embed into data.js
cd experiments/visualization/main/viz && bash embed-data.sh > data.js

# Generate thesis figures (requires Playwright)
npm install playwright
npx serve -l 8080 &
node screenshot-figures.mjs
```

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: exports polytope JSON with Reeb orbits |
| `formal/visualization/main.tex` | Formal writeup (input by `formal/main.tex`) |
| `viz/` | Interactive Three.js viewer (index.html, viz.js, data files, screenshot automation) |
| `viz/data/*.json` | Per-polytope JSON data (intermediate, regenerated) |
| `viz/data.js` | Embedded data for viewer (auto-generated) |
| `viz/screenshot-figures.mjs` | Playwright screenshot automation |
| `viz-hypercube-edges.png` | Thesis figure: hypercube edge skeleton |
| `viz-hypercube-ridges.png` | Thesis figure: hypercube with ridge surfaces |
| `viz-hypercube-traj.png` | Thesis figure: Reeb trajectory on hypercube |
| `viz-simplex-traj.png` | Thesis figure: Reeb trajectory on simplex |
| `viz-hko-pentagon-edges.png` | Thesis figure: HK-O pentagon edge skeleton |
| `viz-hko-pentagon-traj.png` | Thesis figure: Reeb trajectory on HK-O pentagon |
| `viz-lagrangian-tri-product-traj.png` | Thesis figure: Reeb trajectory on triangle product |

## Design

### Pipeline

```
main.rs (Rust) -> data.js -> viz/index.html (Three.js) -> screenshot-figures.mjs (Playwright) -> figures
```

### Projection

- **Radial projection** maps polytope boundary to S^3 (well-defined since K contains origin in interior)
- **Stereographic projection** maps S^3 to R^3 for rendering (conformal, maps great circles to circles)
- Configurable north pole placement avoids sending edges to infinity
- Pole singularity handled by clipping: points near north pole with inner product >= (R^2 - 1)/(R^2 + 1) are omitted

### Trajectory types

1. **Minimum-action Reeb orbits:** Recovered from HK2017 dual solution (S, sigma, beta) via base point recovery. Multiple orbits may share the same minimum action.
2. **Higher-action Reeb orbits:** Other valid closed orbits from exhaustive enumeration. Some fail recovery (high max_violation) and are skipped. Total capped at 20 per polytope.
3. **Displaced orbits:** Perturbations of min-action orbit base point by epsilon = 0.02 in ridge tangent directions. Shows how nearby trajectories twist. Generally do not close.

### Special cases

- **Crosspolytope (F = 16):** Orbit computation skipped due to exponential cost. Placeholder forward-simulated trajectory used instead.
- **Max facets for orbit computation:** 12

## Findings

From the interactive viewer and the README orbit table (not independently verified against JSON data files, which are regenerated intermediate artifacts):

| Polytope | F | Total orbits | Min-action | c_EHZ | Segments |
|----------|---|-------------|------------|-------|----------|
| Simplex | 5 | 6 | 6 | 0.250 | 5 |
| Hypercube | 8 | 142 | 2 | 4.000 | 4 |
| HK-O pentagon | 10 | 718 | 20+ | 3.441 | 6-7 |
| Lag. triangle-product | 6 | 8 | 2 | 1.500 | 6 |
| Sym. triangle-product | 6 | 31 | 2 | 1.299 | 3 |
| Lag. tri x square | 7 | 4 | 2 | 1.500 | 5 |
| Sym. tri x square | 7 | 62 | 1 | 1.000 | 4 |
| Crosspolytope | 16 | -- | -- | -- | -- |

Qualitative observations:

1. On the hypercube, the two min-action orbits each visit 4 of 8 facets, reflecting the decomposition into two symplectic planes.
2. On the simplex, all 6 min-action orbits visit all 5 facets and achieve the same action (0.25), reflecting the full symmetry group.
3. The HK-O pentagon has at least 20 distinct min-action orbits, all achieving action ~3.441.
4. On Lagrangian products, higher-action orbits frequently fail base point recovery, while min-action orbits recover reliably.
5. Displaced trajectories visibly twist around the min-action orbit. On the hypercube they close (stable orbit family); on less symmetric polytopes they diverge.
6. Ridges curve under stereographic projection (great-circle arcs on S^3 map to circles in R^3).

## Changes (2026-03-28)

- `serve.sh` data path changed from `docs/viz/data` to local `viz/data` (after `docs/` deletion). **Untested** — visualization binary doesn't compile (KktOutcome API change).
- `docs/viz/` deleted (was stale copy of `viz/`, formerly used as GitHub Pages deployment target — no longer used).
- GitHub Pages URL removed from the deleted experiment logbook and formal writeup.

## Known limitations

- Crosspolytope uses placeholder trajectory (orbit computation skipped at F = 16).
- Some higher-action orbits fail base point recovery and are skipped.
- Screenshot automation requires Playwright and a local HTTP server.
- Stereographic projection distorts distances; north pole choice affects visual quality.
