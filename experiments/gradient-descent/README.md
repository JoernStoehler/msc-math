# gradient-descent

Gradient ascent on `sys = c_EHZ^2 / (2 vol)` for 1001 random F=10 polytopes (500 general, 501 Lagrangian products). Scales up the sys-optimization experiment.

## Running

```bash
cd experiments/
cargo run --release --bin gradient_descent
python3 gradient-descent/gradient_descent.py
```

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
- **Step-bound barrier**: the algorithm terminates because t_max shrinks (combinatorial type boundary), not because gradients vanish. Residual gradients are O(1) at termination, and positively correlated with final sys (r ≈ 0.80).

## Files

| File | Purpose |
|------|---------|
| `gradient_descent.rs` | Rust binary: polytope generation + gradient ascent |
| `kkt_instrumented.rs` | Shared instrumented KKT solver and orbit enumeration |
| `gradient_descent.py` | Python: scatter and convergence figures + summary stats |
| `gradient-descent.jsonl` | Per-iteration trajectory data (7631 rows, 995 polytopes) |
| `gradient-descent.tex` | Thesis writeup |
