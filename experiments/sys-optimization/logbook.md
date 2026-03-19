# Sensitivity Analysis and Gradient Optimization of sys: Logbook

## Motivation

Random sampling (random-sweep, random-product-sweep) has not found sys > 1. Instead of hoping to get lucky, this experiment computes the analytical gradient of sys = c_EHZ^2 / (2 vol) with respect to facet heights h_k and normals n_k, then takes directed gradient steps. The H-representation P = {x : n_k . x <= h_k} gives us knobs: heights and normals.

## Status

**Complete.** All four phases finished. Best sys achieved: 0.8778 (starting from 0.351). No polytope reaches sys > 1.

## How to run

```bash
cd experiments/
cargo run --release --bin sys_optimization
python3 sys-optimization/analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: all four phases (sensitivity, steps, iteration, validity) |
| `analyze.py` | Python: gradient histograms, comparison plots, convergence, validity, stats table |
| `math.tex` | Thesis writeup with formal lemmas and proofs (input'd from `thesis/experiments.tex`) |
| `sys-optimization-sensitivity.jsonl` | Phase 1: per-polytope gradients (140 rows) |
| `sys-optimization-steps.jsonl` | Phase 2: single gradient step evaluations (1400 rows) |
| `sys-optimization-iterations.jsonl` | Phase 3: iterative gradient ascent trajectories (868 rows) |
| `sys-optimization-validity.jsonl` | Phase 4: gradient prediction accuracy (4990 rows) |
| `sys_optimization_stats.tex` | Auto-generated LaTeX stats table |
| `sys_optimization_gradient_hist.png` | Figure: distribution of height sensitivities |
| `sys_optimization_gradient_comparison.png` | Figure: h-only vs (h,n) step comparison |
| `sys_optimization_improvement.png` | Figure: single-step improvement scatter |
| `sys_optimization_convergence.png` | Figure: iterative convergence trajectories |
| `sys_optimization_iteration_summary.png` | Figure: iteration count distribution, start vs final sys, step type usage |
| `sys_optimization_validity.png` | Figure: gradient validity analysis |

## Design

### Architecture: four phases in one binary

1. **Phase 1 (sensitivity):** Compute analytical d(sys)/dh and d(sys)/dn for 140 polytopes (from random-sweep and random-product-sweep with F <= 10). Uses envelope theorem for capacity derivatives, swept-volume argument for volume derivatives. All derivatives FD-cross-checked in debug builds.

2. **Phase 2 (single steps):** For each polytope, try 5 step fractions x 2 step types (h-only, (h,n)). Step bound preserves combinatorial type (height positivity, vertex-facet incidence, omega_0 signs).

3. **Phase 3 (iteration):** Iteratively recompute HK2017 + gradient + step. Pick best of 10 candidates per iteration. Converges in ~6 iterations on average.

4. **Phase 4 (validity testing):** Test gradient prediction accuracy along gradient and random directions, at step sizes from 0.01 x t_max to 10 x t_max. Tests whether the gradient is trustworthy and how conservative the step bounds are.

### Key design decisions

- **Self-contained binary:** Copies needed library internals (facet_volume_3d, KKT solve) rather than depending on unstable library APIs.
- **ValidOrbit struct:** Extracts all KKT data (beta*, Q*, nu, lambda) needed for analytical derivatives.
- **Step bound for (h,n):** In addition to height-positivity and vertex-crossing checks, enforces that omega_0(n_i, n_j) doesn't change sign for ridge-adjacent pairs. This is the binding constraint in practice.
- **Best-of-10 step selection in Phase 3:** Try all 5 fractions x 2 types at each iteration. Naturally adapts (early iterations prefer (h,n), later shift to h-only).

### Starting polytopes

The 140 polytopes from random-sweep and random-product-sweep with F <= 10.

## Findings

All verified against the four JSONL files.

1. **All 140 starting polytopes have nonzero gradient.** None is a critical point of sys. Every one admits a direction of improvement.

2. **Phase 1 — gradient structure:**
   - Grad_h sys and grad_n sys have different units (h has length scale, n dimensionless) — norms not directly comparable.
   - Predicted max delta_sys: mean 0.34 (h-only), 0.15 ((h,n)).
   - (h,n) step bound is 1-3 orders of magnitude smaller (omega_0 sign constraint).

3. **Phase 2 — single gradient steps:**
   - (h,n) step outperforms h-only in 95/140 polytopes (68%).
   - Mean delta_sys: 0.054 ((h,n)) vs 0.034 (h-only).
   - (h,n) gives ~59% more improvement despite much smaller step bound — landscape more curved in normal directions.
   - Best sys from single step: 0.805.

4. **Phase 3 — iterative gradient ascent (138 polytopes, 2 admit no improving step):**
   - Mean cumulative delta_sys: 0.149 (~3x single-step).
   - Best sys after iteration: 0.8778 (random_5x5_1, starting from sys = 0.351).
   - Mean iterations: 6.3. (h,n) steps selected 58% of the time (506/868 steps).
   - All 138 terminate with improvement < 10^-6 (step bound exhaustion, not gradient vanishing).

5. **Phase 4 — gradient validity:**
   - Height gradient: excellent predictor (<5% error at 0.25 x t_max, O(t) growth).
   - (h,n) gradient: bimodal — good for ~125/140 polytopes, ~15 outliers near orbit boundaries. Median pred/actual ratio 1.55 (systematic overprediction from normal renormalization).
   - Random directions: ~90% relative error at all scales (expected: random is nearly orthogonal to gradient in high dim).
   - Step bounds are conservative: 85% construction success at 2 x t_max, 60% at 10 x t_max. Type preservation drops fast: 35% at 2 x t_max.

6. **Wrong-sign gradient predictions (investigated 2026-03-02):** Of 4990 validity records, 1977 (39.6%) have negative actual_delta_sys. Two mechanisms: (a) combinatorial type change (32 records at t >= 1.0 x t_max), (b) Reeb orbit switching (remaining cases). The iterative optimizer avoids both: line search stays at t_fraction <= 0.95 and recomputes HK2017 after each step.

## Known limitations

- Only F <= 10 polytopes (HK2017 cost is exponential in F).
- Greedy gradient ascent preserves combinatorial type at each step — cannot cross type boundaries.
- Two polytopes (random_3x5_0, random_4x5_6) admit no improving step from start — mechanism unknown.
- Normal gradient has outliers (15/140 wrong-sign predictions) near orbit boundaries.

## Dead ends / deferred directions

- **Large steps beyond gradient regime:** Accept combinatorial type change, recompute sys from scratch. Gradient prediction invalid but actual sys still computable. Could find polytopes the conservative step bound misses.
- **Targeted single-facet moves:** For k not in (S, sigma), d(c_EHZ)/dh_k = 0, so height changes only affect vol. Large decrease in h_k could trigger orbit switch + volume reduction.
- **Adding facets (F = 11+):** Introduce new half-spaces clipping vertices. Changes combinatorial type and is exponentially more expensive. Deferred.
- **Convergence characterization:** Is converged sys a function of combinatorial type? Of facet count? Relationship between initial sys and improvement magnitude?

## Open questions

- Does the Phase 1 data show all gradient components point "inward" at HKO2024? If so, that's evidence for local maximality in the F = 10 ambient space.
- HKO2024 local maximality across ambient spaces: LP(5,5), general F = 10, degenerate F = 11+, convex bodies as F grows.

## Jorn verification status

All three items previously flagged are resolved:
- Lemma vol-derivative-normal: math approved (59ddc2c)
- Lemma cap-derivative-normal: math approved (59ddc2c), covers H-term sign chain
- Proposition capacity-piecewise-smooth: math approved (b01bf24)
- Corollary sys-derivative: math approved (b01bf24)
- Remark simplicity-generic: math approved (b01bf24)

## Related experiments

- **gradient-descent:** Scales up Phase 3 to ~1000 polytopes. Uses kkt_instrumented.rs copied from this experiment.
- **pentagon-perturb:** Complementary approach — random perturbations of the known counterexample.
- **random-sweep, random-product-sweep:** Source of the 140 starting polytopes.
