# 4D Polytope Visualization

Interactive 3D visualization of 4D polytopes via stereographic projection.

## Live Demo

Once GitHub Pages is enabled for this repository, the webapp will be available at:
`https://joernstoehler.github.io/msc-math/viz/`

## Enabling GitHub Pages

1. Go to repository Settings → Pages
2. Set Source to "Deploy from a branch"
3. Select branch: `main`
4. Select folder: `/docs`
5. Click Save

The webapp will be published within a few minutes at the URL above.

## Local Development

To run locally:
```bash
cd docs/viz
python3 -m http.server 8080
```

Then navigate to http://localhost:8080

## Features

- **Interactive 3D rendering** via Three.js with OrbitControls
- **Stereographic projection** from S³ (4D sphere) to R³
- **Customizable north pole** for projection perspective
- **Toggle display** of vertices, edges, ridges, and Reeb trajectories
- **Optimized rendering** with batched geometries for smooth performance

## Performance

The webapp uses batched rendering to minimize draw calls:
- Vertices: instanced rendering (1 mesh per color)
- Edges/trajectories: merged LineSegments per color
- Ridges: merged triangle meshes per color

This reduces object count from 1000+ to ~10-20, enabling smooth interaction even on complex polytopes.

## Polytopes Included

- 4-Simplex (5 facets)
- Hypercube [-1,1]⁴ (8 facets)
- Cross-polytope (16 facets)
- HK-O Pentagon (10 facets) - from Haim-Kislev-Ostrover 2024
- Lagrangian Δ×Δ (6 facets)
- Symplectic Δ×Δ (6 facets)
- Lagrangian Δ×□ (7 facets)
- Symplectic Δ×□ (7 facets)
