# Step Calibration: Logbook

## Motivation

boundary-crossing-search uses STEP_FRACTIONS [0.1, 0.3, 0.5, 0.7, 0.9] of t_max (distance to cell boundary). These fractions are arbitrary. cell-widths (exp-combinatorial-cells) measured per-facet cell widths of 0.12-0.26 for random polytopes. This instrument calibrates step sizes using actual cell geometry data, and compares fixed vs adaptive strategies.

## Status

**Scaffolded.** Not yet implemented.

## How to run

```bash
cargo run -p dev-gradient-ascent --release --bin dev_step_calibration
```

## Research questions

1. What fraction of t_max gives fastest convergence to cell-local maximum?
2. Does optimal fraction depend on F, polytope type (random vs Lagrangian product), or sys value?
3. Does adaptive step sizing (e.g. backtracking line search) outperform fixed fractions?

## Related experiments

- `exp-combinatorial-cells/cell-widths/` — cell width measurements
- `exp-sys-landscape/boundary-crossing-search/` — current step fraction choices
