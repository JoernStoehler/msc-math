# Visualization Decisions

The previous experiment log mixed operational notes with status history. The durable decisions for this topic are:

- Keep the three-file local-note convention and remove the old README as the top-level topic note.
- Keep `visualization` as an experiment binary (`[[bin]]`) with strictly local assets in `main/viz/`, not as a reusable crate API.
- Keep built-in known polytopes explicit (no external input artifacts), so command entry points are reproducible by name.
- Keep payload size bounded (`MAX_ORBITS`, `MAX_FACETS_FOR_ORBIT`) to keep the browser data object manageable.
- Keep `MAX_FACETS_FOR_ORBIT = 12` and use the existing placeholder approach for crosspolytope orbits instead of forcing full computation at F=16.
- Keep projection in 4D via radial projection → stereographic projection, because it preserves enough geometric structure for faces/trajectories while being fast enough for interactive use.
- Keep configurable north pole and pole-clipping behavior; changing `MAX_RADIUS` and projection pole is required tuning to avoid near-pole blow-up.
- Keep screenshot automation in `main/viz/screenshot-figures.mjs` as the canonical path for thesis figures because hand-capturing introduces drift.
- Keep formal/experimental boundaries: reusable correctness claims live in formal/, while this topic remains exploratory + presentation oriented.

Rejected routes and constraints worth remembering:
- Do not restore the removed `docs/viz` deployment path; local `main/viz` plus `python3 -m http.server` is the active layout.
- Do not rely on GitHub Pages URL references in this topic anymore.
- Do not assume a closed-form orbit recovery for all higher-action candidates; recovery can fail and is intentionally filtered out.

Known numeric thresholds that are not accidental defaults:
- recovery discard thresholds are explicit in logging (`closure_error`, `max_violation` checks),
- displacement visualization is driven by small ridge-normal perturbations (`DISPLACEMENT_EPS` in `main.rs`).
