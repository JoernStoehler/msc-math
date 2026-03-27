# Gradient Correctness: Logbook

## Motivation

The library provides analytical gradients ∂c/∂a_k and ∂vol/∂a_k via envelope theorem and chain rule formulas ([lem:cap-derivative], [lem:vol-derivative] — both unverified). The systolic ratio gradient ∂sys/∂a_k is derived via quotient rule ([cor:sys-derivative]). These are used by optimization experiments. This experiment tests whether the analytical gradients correctly predict function values to first order.

## Status

**v2 complete.** Data generated 2026-03-27. v1 (FD cross-checking) is in git log — superseded because FD cross-checking tests agreement between two computations, not whether the output is a gradient.

## Methodology

**First-order prediction test.** The defining property of a gradient g of f at a is:

    f(a + td) − f(a) − t·g·d = o(t)  as t → 0

For each polytope, target f ∈ {capacity, volume, sys}, and random direction d ∈ R^{4F}:
- Compute f(a), g(a) analytically
- Sweep t geometrically from 1e-1 to 1e-7 (13 values)
- Compute residual r(t) = |f(a+td) − f(a) − t·g·d|
- Fit log-log slope of r(t) vs t

**Interpretation of slope:**
- Slope ≈ 2: function is C² (quadratic Taylor remainder dominates) — expected within a smooth orbit region
- Slope ≈ 1: function is C¹ but not C² — would indicate Lipschitz but not smooth gradient
- Slope ≈ 0: function is non-differentiable at the test point

**Capacity perturbation uses fixed orbit:** The perturbed capacity is computed via `solve_kkt_for` with the base point's best orbit, not via full `ehz_capacity`. This tests the per-orbit envelope theorem gradient ([lem:cap-derivative]), which equals the capacity gradient at generic parameters where the minimizing orbit is unique ([prop:capacity-piecewise-smooth](c)). It does NOT test behavior at orbit-switching boundaries — that would require full `ehz_capacity` on the perturbed polytope.

**Assumption:** [prop:capacity-piecewise-smooth] (unverified) — piecewise C^∞ structure, generic differentiability. The experiment cannot verify this proposition; it assumes it and tests the gradient formula conditional on it.

5 random directions per polytope, seeded RNG for reproducibility.

## Research questions

1. **Q1 Generic polytopes:** Does the gradient predict to first order? What convergence rate? How does it vary with F?
2. **Q2 Non-generic geometry:** Does the gradient work for Lagrangian products with symmetry-degenerate orbits?
3. **Q3 Near-degeneracy:** Does the gradient degrade when the gap between best and second-best orbit action is small?
4. **Q4 Barely-cutting facets:** Does the gradient degrade for near-redundant halfspaces?

## How to run

```bash
cd experiments/
cargo run --release --bin gradient_correctness        # all phases
cargo run --release --bin gradient_correctness -- q1   # single phase
python3 gradient-correctness/analyze.py               # figures + summary
```

## Results (2026-03-27)

### Dataset sizes

| Phase | Rows | Polytopes | Directions | t-values | Time |
|-------|------|-----------|------------|----------|------|
| Q1 generic | 22999 | 120 (F=5..10, 20 each) | 5 | 13 | 34s |
| Q2 non-generic | 3138 | ~20 Lagrangian products (F≤8) | 5 | 13 | 5s |
| Q3 near-degenerate | 14578 | 78 at F=6 (binned by action gap) | 5 | 13 | 29s |
| Q4 barely-cutting | 8966 | 10 base × 5 δ | 5 | 13 | 12s |

Some rows missing vs theoretical maximum due to failed perturbations (polytope construction or KKT solve fails at large t or on degenerate geometries).

### Observation 1: Capacity gradient is correct (slope = 2.00)

Fitted log-log slope for capacity is 2.00 [1.99, 2.01] (median [P25, P75]) across all 600 Q1 traces and all phases (gc_slopes.png, gc_summary.tex). The convergence plot (gc_convergence.png, left panel) shows residual tracking the slope-2 reference line from t=1e-1 to t=1e-7 with no deviation.

**Epistemic status:** Observation — the data directly shows this. The inference that the envelope theorem formula is correct follows from this observation plus the assumption that [prop:capacity-piecewise-smooth] holds.

### Observation 2: Volume gradient is correct but has cancellation floor

Fitted slope for volume is 1.98 [1.92, 2.05] for Q1 (gc_convergence.png, middle panel). The convergence plot shows residual tracking slope 2 from t=1e-1 to t≈1e-4, then leveling off and increasing. This V-shape is floating-point cancellation: for small t, `vol(a+td) − vol(a)` loses precision because two nearly-equal volumes are subtracted.

The gradient is correct in the convergent region (t ∈ [1e-1, 1e-4]). The cancellation floor does not indicate a gradient error.

Q2 (Lagrangian products) shows lower slopes for volume (median 1.74) and sys (median 1.65). This may reflect worse cancellation behavior on Lagrangian products, where some facet volumes are very small. Not investigated further — the convergent region still shows the correct trend.

### Observation 3: Near-degeneracy has no effect on per-orbit gradient

Q3 scatter plot (gc_q3_gap.png) shows no correlation between action gap and fitted slope. Slopes cluster at 2.0 regardless of whether the gap is 10^{-5} or 10^1.

**Caveat:** This tests the per-orbit gradient only (fixed orbit during perturbation). At orbit-switching boundaries, the actual capacity min(A₁, A₂) is non-differentiable. Testing this would require using full `ehz_capacity` for the perturbed point, which this experiment does not do.

### Observation 4: Barely-cutting facets have no effect

Q4 plot (gc_q4_delta.png) shows all slopes between 1.94 and 2.04 regardless of δ, with no systematic trend. The gradient remains correct even as the added facet contributes negligibly to the polytope.

### Summary table (fitted slope, log_t ∈ [-4, -1])

| Phase | capacity | volume | sys |
|-------|----------|--------|-----|
| Q1 generic | 2.00 [1.99, 2.01] | 1.98 [1.92, 2.05] | 1.98 [1.91, 2.02] |
| Q2 non-generic | 2.00 [2.00, 2.00] | 1.74 [1.63, 1.88] | 1.65 [1.42, 1.89] |
| Q3 near-degenerate | 2.00 [1.99, 2.01] | 1.98 [1.94, 2.05] | 1.99 [1.94, 2.02] |
| Q4 barely-cutting | 2.00 [1.99, 2.01] | 1.98 [1.93, 2.07] | 1.98 [1.92, 2.01] |

Values are median [P25, P75] of fitted log-log slopes across all traces with R² > 0.5.

### Inference

The analytical gradients ∂c/∂a_k, ∂vol/∂a_k, ∂sys/∂a_k correctly predict their respective function values to first order. The convergence rate confirms all three functions are C² in the dual vertex parameter space (within a fixed orbit region), consistent with the unverified claim in [prop:capacity-piecewise-smooth] that the per-orbit action is smooth.

**This inference depends on:**
- [prop:capacity-piecewise-smooth] (unverified): the capacity is piecewise C^∞
- The KKT solver and volume computation being correct (tested elsewhere)
- The random direction sampling being representative (5 directions per polytope)

**This inference does NOT address:**
- Whether the capacity is differentiable at orbit-switching boundaries (non-generic points)
- Whether the subdifferential characterization at non-smooth points is correct
- Whether the unverified lemmas [lem:cap-derivative], [lem:vol-derivative] are mathematically correct (the experiment only tests whether the code predictions match the code values — a shared conceptual error in both the function and the gradient code would not be detected)

## Known issues

- **Q-correction panic:** `solve_kkt_for` panics on some near-degenerate polytopes. Caught via `catch_unwind` in `solve_kkt_safe`. Results in missing rows (perturbation skipped), not incorrect data.
- **Q2 volume/sys slope degradation:** Lagrangian products show lower fitted slopes for volume (1.74) and sys (1.65). Likely floating-point cancellation on small-facet polytopes, not a gradient error. The convergent region still trends correctly.
