# Boundary Characterization: Logbook

Split from the original `combinatorial-structure/` experiment (Pass 2: boundary anatomy + crossing + gradient).

## Motivation

Full EHZ at boundaries + crossing analysis + gradient measurement. For each polytope, probes the gradient direction, negative gradient, and 5 dense random directions. At each boundary: records anatomy (event type, t_max, orbit gap), evaluates crossing (sys before/after, orbit switch), and measures gradient change.

The main.rs binary also produces gradient data used by `gradient-discontinuity/`.

## Status

**Complete (2026-03-27).** 140 polytopes, 980 anatomy rows, 882 crossing rows, 882 gradient rows, 8 figures. Split into standalone experiment.

## How to run

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-boundary-characterization
uv run analyze.py
```

## Results (from combinatorial-structure, 2026-03-26, updated 2026-03-27)

### RQ1: What causes combinatorial type changes?

| Event type | Count | Fraction |
|------------|-------|----------|
| Incidence flip | 578 | 59.0% |
| omega_0 flip | 402 | 41.0% |

(anatomy JSONL, 980 rows, 0 unbounded)

### RQ2: sys is continuous; orbits switch at 3% of boundaries

sys is continuous at all 882 tested boundaries: max |delta_sys| = 8.69e-5 (crossing JSONL, boundary_sys_continuity.png).

Orbit switch rate: 28/882 (3.2%).

### RQ4: Boundary density

Boundary distance decreases with F (boundary_tmax_vs_F.png). Gradient direction hits boundaries sooner than dense random (boundary_density_cdf.png).

### Orbit gap

132/140 polytopes have >= 2 valid orbits. Median gap 0.054, min ~0, max 13.76 (orbit_gap_distribution.png).

Orbit gap predicts orbit switches (orbit_gap_vs_switch.png): lowest gap quartile has higher switch rates.

### Products vs random polytopes

| Metric | Random | Lagrangian product |
|--------|--------|-------------------|
| Orbit gap median | 0.163 | 0.008 |
| Orbit-facet cell width | 0.169 | 0.363 |

## Related experiments

- **gradient-discontinuity** -- analyzes gradient angle change and gradient-cell alignment using data from this experiment's main.rs
- **cell-widths** -- per-facet cell width data used for gradient-cell alignment analysis (in gradient-discontinuity)
- **gradient-correctness** -- validates gradient formula; this experiment studies what happens when the gradient changes
- **gradient-ascent-general**, **gradient-ascent-products** (`experiments/sys-landscape/`) -- use boundary-crossing strategies; this experiment characterizes the boundaries

## Open questions

1. **Continuity of sys:** `formal/combinatorial-cells/boundary-characterization.tex` has a proof sketch (Prop. prop:sys-continuous). Full continuity may require citing general c_EHZ continuity on convex bodies.
2. **sys-search omega_0 gap:** sys-search step bound doesn't detect omega_0 flips. Missing 43% of boundaries.
