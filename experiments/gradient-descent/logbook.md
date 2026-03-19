# Large-Scale Gradient Ascent on sys: Logbook

## Motivation

Random sampling experiments (random-sweep, random-product-sweep) found no polytope with sys > 1. The sys-optimization experiment developed gradient ascent on sys and demonstrated it on 140 polytopes. This experiment scales the procedure to ~1000 starting polytopes, distinguishing general polytopes from Lagrangian products with different facet splits, to test whether gradient ascent can push sys past 1 from diverse starting points.

## Status

**Complete.** 995 of 1001 polytopes successfully optimized. No polytope achieved sys > 1.

## How to run

```bash
cd experiments/
cargo run --release --bin gradient_descent
python3 gradient-descent/analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: polytope generation + gradient ascent |
| `kkt_instrumented.rs` | Shared instrumented KKT solver and orbit enumeration (included via `#[path]`) |
| `analyze.py` | Python: scatter, gradient, step-size, and survival figures |
| `math.tex` | Formal writeup (input'd from `thesis/experiments.tex`) |
| `gradient-descent.jsonl` | Per-iteration trajectory data: 7631 rows, 995 polytopes |
| `gradient_descent_scatter.png` | Figure: starting vs final sys |
| `gradient_descent_gradient.png` | Figure: residual gradient vs final sys |
| `gradient_descent_stepsize.png` | Figure: step size decay |
| `gradient_descent_survival.png` | Figure: iteration survival curve |

## Design

- **Starting polytopes:** 1001 random F = 10 polytopes
  - 500 general: normals uniform on S^3, heights uniform in [0.8, 1.2]
  - 501 Lagrangian products: random polygon pairs K_q x_L K_p with splits (3x7, 4x6, 5x5), 167 each
- **Algorithm:** Gradient ascent on sys with analytical gradients (d(sys)/dh)
  - Step size controlled by t_max (maximum height step before combinatorial type changes)
  - Line search: step fractions {0.1, 0.25, 0.5, 0.75, 0.95} x t_max for both h-only and (h,n) directions
  - Best of 10 candidates per iteration
- **Termination:** improvement < 10^-6 or 20 iterations
- **Capacity computation:** HK2017 pruned for general; billiard with omega_0 pruning for Lagrangian products
- **Lagrangian constraint:** Normal perturbations projected to preserve product structure
- **Seed:** 42
- **Failures:** 6 polytopes failed due to degenerate initial data (995 of 1001 completed)

## Findings

All verified against `gradient-descent.jsonl` (7631 rows, 995 polytopes).

1. **No polytope achieved sys > 1.** Best: sys = 0.9049 (lagrangian_5x5_143).

2. **Results by polytope class (final sys after optimization):**

   | Type | N | Mean sys | Max sys | P90 sys | Mean improvement |
   |------|---|----------|---------|---------|-----------------|
   | General | 499 | 0.453 | 0.870 | 0.687 | +0.100 |
   | Lagrangian 3x7 | 166 | 0.493 | 0.862 | 0.781 | +0.182 |
   | Lagrangian 4x6 | 166 | 0.566 | 0.881 | 0.784 | +0.222 |
   | Lagrangian 5x5 | 164 | 0.628 | 0.905 | 0.806 | +0.217 |

3. **Lagrangian products reach higher sys than general polytopes** (mean 0.56 vs 0.45). Balanced splits (5x5) outperform asymmetric splits (3x7), consistent with HKO2024 being a 5x5 Lagrangian product.

4. **Step-bound barrier:** The algorithm terminates because t_max shrinks (combinatorial type boundary), not because gradients vanish. Residual gradients are O(1) at termination and positively correlated with final sys (Pearson r = 0.80). The line search selects the most aggressive fraction (0.95 x t_max) in 87% of iterations.

## Known limitations

- Only F = 10 polytopes tested; other facet counts may behave differently.
- Step-bound barrier prevents convergence to true local optima. The algorithm is confined to the current combinatorial type cell.
- 6 polytopes failed during optimization (995 of 1001 completed).
- Gradient ascent finds local optima only; global landscape structure not explored.
- If a basin of attraction for sys > 1 exists, it may have small measure among random starting points.

## Open questions

1. **Naming inconsistency:** The directory and Cargo.toml bin are named `gradient-descent`/`gradient_descent`, but the algorithm performs gradient *ascent* on sys. The logbook title uses "ascent." Cosmetic — needs a decision on which name to standardize.

## Related experiments

- **sys-optimization:** The method origin. Developed the gradient computation, step bound, and line search on 140 polytopes.
- **pentagon-perturb:** Complementary approach — random perturbations of the known counterexample rather than gradient ascent from random starting points.
