# Gradient Descent

Gradient ascent on `sys = c_EHZ^2 / (2 vol)` for 1001 random F=10 polytopes (500 general, 501 Lagrangian products). Scales up the sys-optimization experiment.

## Status
Complete

## Design

- 1001 random F=10 polytopes: 500 general, 501 Lagrangian products (split: 167 each of 3x7, 4x6, 5x5)
- Gradient ascent on sys with analytical gradients (d(sys)/dh)
- Step size controlled by t_max (maximum height step before combinatorial type changes)
- Seed 42 for reproducibility

## Key findings

**No polytope achieved sys > 1.** Best: sys = 0.905 (5x5 Lagrangian product).

| Type | N | Mean sys | Max sys | Mean improvement |
|------|---|----------|---------|------------------|
| General | 499 | 0.453 | 0.870 | +0.100 |
| Lagrangian 3x7 | 166 | 0.493 | 0.862 | +0.182 |
| Lagrangian 4x6 | 166 | 0.566 | 0.881 | +0.222 |
| Lagrangian 5x5 | 164 | 0.628 | 0.905 | +0.217 |

- Lagrangian products reach higher sys than general polytopes
- Balanced splits (5x5) outperform asymmetric splits (3x7)
- **Step-bound barrier**: the algorithm terminates because t_max shrinks (combinatorial type boundary), not because gradients vanish. Residual gradients are O(1) at termination, and positively correlated with final sys (r = 0.80).

## Files

| File | Purpose |
|------|---------|
| `gradient_descent.rs` | Rust binary: polytope generation + gradient ascent |
| `kkt_instrumented.rs` | Shared instrumented KKT solver and orbit enumeration |
| `gradient_descent.py` | Python: scatter and convergence figures + summary stats |
| `gradient-descent.jsonl` | Per-iteration trajectory data (7631 rows, 995 polytopes) |
| `gradient-descent.tex` | Thesis writeup |
| `gradient_descent_scatter.png` | Figure: starting sys vs final sys |
| `gradient_descent_convergence.png` | Figure: three-panel convergence diagnostics |
| `gradient_descent_gradient.png` | Figure: residual gradient vs final sys |
| `gradient_descent_stepsize.png` | Figure: step size decay |
| `gradient_descent_survival.png` | Figure: iteration survival curve |

## Run

```bash
cd experiments/
cargo run --release --bin gradient_descent
python3 gradient-descent/gradient_descent.py
```

## Known limitations

- Only F=10 polytopes tested; other facet counts may behave differently
- Step-bound barrier prevents convergence to true local optima
- 6 polytopes failed during optimization (995 of 1001 completed)
- Gradient ascent finds local optima only; global structure not explored
