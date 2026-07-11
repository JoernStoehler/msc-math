# Visualization

This package owns the interactive viewer and generated assets for the thesis's
small 4D-polytope visualization side result. The viewer is useful for
qualitative explanation and manual exploration; it is not proof or search
evidence.

## Reproduction

From the repository root, start the full data refresh and viewer:

```bash
cd experiments/visualization
./main/viz/serve.sh 8080
```

The script builds `visualization`, regenerates the eight JSON files in
`main/viz/data/`, embeds them in `main/viz/data.js`, and serves the viewer at
`http://localhost:8080`. In another shell, install the pinned screenshot tool
and regenerate the two selected figures:

```bash
cd experiments/visualization/main/viz
npm install --no-save --no-package-lock playwright@1.61.1
npx playwright@1.61.1 install --with-deps chromium  # one-time browser setup
node screenshot-figures.mjs
```

For a single JSON export, run:

```bash
cargo run --release --manifest-path experiments/visualization/Cargo.toml \
  --bin visualization -- <known-polytope-name> <output.json>
```

`main/viz/embed-data.sh` rebuilds only `data.js` from existing JSON files.
JSON, `data.js`, and PNG files are generated artifacts; regenerate rather than
editing them.

## Selected Figures

- `main/viz-hypercube-ridges.png` is thesis-ready explanatory material. It
  shows the projected edges and interpolated spherical two-face meshes of the
  4-cube. Its purpose is to help a reader form a qualitative picture of a 4D
  polytope boundary; the meshes are not metric or symplectic data.
- `main/viz-hko-pentagon-min-orbit.png` is thesis-ready explanatory and
  empirical material. It shows the projected one-skeleton of the HKO
  Lagrangian product of two regular pentagons together with trajectory index
  zero from `hko_pentagon.json`, a six-segment recovered minimum-action orbit.
  The structure is muted and the orbit purple so their roles remain readable
  at thesis width.

Both screenshots use the north pole `e4`, clipping radius `6`, and an
`800 x 600` viewport. The 4-cube camera is `(2.6,1.95,3.25)` and the HKO
camera is `(3.2,2.4,4.0)`. The thesis copies them deliberately to
`thesis/figures/visualization/`; that directory is self-contained and is not a
build-time link to this experiment.

The earlier orbit-only screenshots and alternative projections were rejected:
without the projected polytope they did not explain where a trajectory lies,
and the diagonal-pole HKO view made projection distortion dominate the object.

## Data And Checks

The eight viewer inputs contain known-polytope vertices, edges, two-faces,
facet Reeb directions, and computed trajectories. Recovered closed orbits are
exported only when recovery succeeds, closure error is at most `1e-6`, and
maximum half-space violation is at most `1e-4`. Simulated displaced and
placeholder trajectories are checked at every segment endpoint against their
declared facet and all polytope half-spaces with tolerance `1e-6`.

The crosspolytope has 16 facets, above `MAX_FACETS_FOR_ORBIT = 12`, so its JSON
contains a clearly labelled forward-simulated placeholder rather than a
recovered closed orbit. No selected thesis figure uses crosspolytope trajectory
content.

The viewer maps a boundary point radially to `S^3` and then stereographically
to `R^3`. Edges and trajectory segments are sampled along the resulting
great-circle arcs. Two-face interiors are qualitative spherical interpolation
meshes. Clipping near the stereographic pole and the projection itself distort
metric geometry.

## Interpretation Boundary

Manual exploration of these projections did not yield a reliable geometric
hypothesis, candidate rule, or proof input. The selected figures serve only to
help readers imagine the objects and the piecewise-linear Reeb dynamics.
Thesis-facing wording and copy provenance live in
`thesis/10-visualization-3d.tex` and `thesis/visualization-3d-content.md`.
