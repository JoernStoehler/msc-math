# Visualization Experiment

`experiments/visualization` owns the Reeb-flow rendering pipeline for thesis figures.

Subdirs and role:
- `main/` — Rust export pipeline to generate polytope/orbit JSON payloads.
- `main/viz/` — viewer assets (`index.html`, Three.js renderer, `data.js`, screenshot script).
- `main/viz/data/` — generated per-polytope payloads and figures.

Usage is local by design: export geometry, render via `viz/`, and generate static figures with Playwright.
