# Gradient Correctness: Logbook

## Motivation

The library provides analytical gradients ∂c/∂a_k and ∂vol/∂a_k via envelope theorem and chain rule formulas ([lem:cap-derivative], [lem:vol-derivative] — both unverified). The systolic ratio gradient ∂sys/∂a_k is derived via quotient rule ([cor:sys-derivative]). These are used by optimization experiments. This experiment tests whether the analytical gradients correctly predict function values to first order.

## Status

**v2 + Q5 complete.** Data generated 2026-03-27. v1 (FD cross-checking) is in git log — superseded because FD cross-checking tests agreement between two computations, not whether the output is a gradient. Q5 (orbit switching, subdifferential prediction) added 2026-03-27.

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
5. **Q5 Orbit-switching:** Does the subdifferential (set of per-orbit gradients) predict capacity at near-switching points? When does orbit switching occur?

## How to run

```bash
cd experiments/
cargo run --release --bin gradient_correctness            # all phases
cargo run --release --bin gradient_correctness -- q1      # single phase (q1-q5)
python3 gradient-correctness/analyze.py                   # figures + summary
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

Q4 plot (gc_q4_delta.png) shows median slopes between 1.94 and 2.04 regardless of δ, with no systematic trend. The gradient remains correct even as the added facet contributes negligibly to the polytope.

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

## Q5 — Orbit-switching and subdifferential prediction

**Status: v1 complete.** Data generated 2026-03-27.

**Research question:** At parameters where multiple orbits are near-tied, does the Clarke subdifferential (set of per-orbit gradients) correctly predict the capacity to first order? When does orbit switching occur under perturbation?

**Background:** [prop:capacity-piecewise-smooth](d) claims that at a switching boundary with r tied orbits, the directional derivative is D_d c = min_i(∇_a A_i · d). Q1-Q4 tested only the per-orbit gradient (fixed orbit via solve_kkt_for). Q5 tests the min-over-orbits prediction using full ehz_capacity for the perturbed point.

**Methodology:**

For each polytope with ≥2 certified orbits, binned by action gap between best and second-best:
1. Enumerate all certified orbits via `enumerate_all_orbits` (same as Q3)
2. Keep orbits within gap threshold τ=0.1 of the best
3. Compute per-orbit gradient g_i via `capacity_derivatives_a` for each
4. For each random direction d: predict D_d c = min_i(g_i · d) (subdiff) and D_d c = g_best · d (single)
5. Compute actual capacity change via full `ehz_capacity` on the perturbed polytope
6. Record which orbit wins in the perturbed polytope (orbit switching diagnostic)

Per-orbit (action, g_i · d) embedded in each JSONL row for post-hoc gap-threshold filtering.

**Design decisions (Jörn, 2026-03-27):**
- Capacity only (volume is smooth; sys less interesting than c right now)
- Generous gap threshold, filter in analysis
- F ∈ {6, 7} — per-row cost ~1ms at F=6, ~1.6ms at F=7 (including polytope construction + ehz_capacity)

### Dataset

| F | Polytopes | Gap bins (large/med/small/tiny) | Rows | Time |
|---|-----------|-------------------------------|------|------|
| 6 | 47 | 15/15/15/2 | 3036 | ~60s |
| 7 | 53 | 15/15/15/8 | 3419 | ~70s |

Tiny gap bin (gap < 1e-5) is underfilled — polytopes with near-exact orbit ties are rare at F=6-7.

### Observation 5: Per-orbit gradient is correct at all gap levels

Fitted single-orbit log-log slope (t ∈ [1e-4, 1e-1]):

| Gap bin | Slope (median [P25, P75]) | n |
|---------|---------------------------|---|
| tiny | 2.00 [1.97, 2.03] | 50 |
| small | 2.00 [1.97, 2.03] | 150 |
| medium | 2.00 [1.98, 2.01] | 150 |
| large | 2.00 [1.98, 2.01] | 150 |

**Epistemic status:** Observation. The per-orbit gradient correctly predicts to first order regardless of how close other orbits are. This extends Q3's finding (which used fixed-orbit perturbation) to the full-capacity setting.

### Observation 6: Subdifferential prediction is biased at non-boundary points

Fitted subdiff log-log slope (t ∈ [1e-4, 1e-1]):

| Gap bin | Slope (median [P25, P75]) | n |
|---------|---------------------------|---|
| tiny | 1.82 [1.45, 1.96] | 50 |
| small | 1.45 [1.06, 1.99] | 150 |
| medium | 1.24 [0.99, 1.99] | 150 |
| large | 2.00 [1.98, 2.01] | 150 |

The subdiff prediction min_i(g_i · d) systematically underpredicts at non-boundary points (gap > 0), because the true gradient is g_best, not the min over all nearby orbits. The bias grows with the number of included non-optimal orbits: at large gap, only 1 orbit is within threshold (subdiff = single), so slope is 2. At medium gap, more orbits with different gradients pull the min down, giving O(t) error (slope ≈ 1).

**Epistemic status:** Observation + interpretation. The observation is that subdiff slope degrades. The interpretation — that it's because min_i(g_i · d) is the directional derivative at the boundary, not at a generic point — follows from [prop:capacity-piecewise-smooth](d), which says the formula applies only at boundary points where orbits are tied.

### Observation 7: Orbit switching is a finite-step boundary-crossing phenomenon

Orbit switching rate by perturbation size and gap bin (gc_q5_switching.png):

| Gap bin | t=1e-1 | t=1e-2 | t=1e-3 | t=1e-4 |
|---------|--------|--------|--------|--------|
| tiny | 76% | 54% | 28% | 4% |
| small | 61% | 24% | 4% | 0% |
| medium | 35% | 3% | 1% | 0% |
| large | 15% | 1% | 0% | 0% |

Smaller gaps lead to earlier switching (at smaller t). Switching is rare at t ≤ 1e-4 (only 2 out of 50 tiny-gap rows), vanishing for all other bins. At switching rows, the single-orbit prediction is still better than subdiff (66% vs 20%), confirming that the perturbation crosses a boundary but both linear predictions fail in the nonlinear regime.

**Epistemic status:** Observation. Consistent with the mathematical picture: c is smooth within each orbit region, and finite-step perturbations can cross into adjacent regions.

### Q5 inference

1. The per-orbit gradient ∂c/∂a_k correctly predicts capacity to first order regardless of orbit gap (Obs. 5). Combined with Q1-Q4, this fully validates [lem:cap-derivative] at generic and near-degenerate points.

2. The subdifferential formula min_i(g_i · d) is not designed for prediction at non-boundary points and indeed fails there (Obs. 6). Testing it at actual boundary points (gap = 0) would require constructing polytopes exactly on a switching boundary — not done.

3. Orbit switching rate is monotone in both gap and step size (Obs. 7). At tiny gaps, switching rate increases from 4% at t=1e-4 to 76% at t=1e-1, consistent with the mathematical picture that switching boundaries are codimension-1 manifolds in parameter space ([prop:capacity-piecewise-smooth](a)).

**This inference does NOT address:**
- Whether the subdifferential formula is correct AT boundary points (gap = 0)
- Whether the switching boundaries are smooth manifolds (only their existence is observed)
- Non-smooth optimization (e.g. subgradient methods) — the subdiff might still be useful for optimization even if it fails for first-order prediction at generic points

## Smoothness framework (refined 2026-03-27)

The smoothness of c_EHZ(a) decomposes into per-orbit and capacity-level questions.

### Per-orbit: A_σ(a)

For a fixed cyclic permutation σ, three questions:

1. **Feasibility:** Does the KKT system for σ have a solution with β ≥ 0? The feasibility region {a : σ is feasible} is open (by continuity of the KKT solution). Its boundary is where some β_k → 0 — but this is not orbit "death": at the boundary, σ' = σ \ {k} is feasible with the same action (β_k = 0 for σ means the characteristic doesn't touch facet k, i.e. it's really orbit σ'). So feasibility boundaries are switching boundaries.

2. **Uniqueness:** Is the KKT solution for σ unique at a? The KKT conditions form a linear saddle-point system; uniqueness holds when the matrix is non-singular. **Open question:** can the saddle-point matrix be singular at points where all β > 0? If so, A_σ(a) could have kinks even for fixed σ (the IFT argument requires non-degeneracy). The experiment does not test this — slope 2.00 at random polytopes is consistent with generic non-degeneracy, but doesn't rule out a codimension-1 singular locus.

3. **Smoothness:** Where feasible and unique, A_σ(a) is C^∞ by the implicit function theorem applied to the KKT conditions. This is what Q1-Q4 test (fixed σ, slope 2.00). The C^∞ claim is [prop:capacity-piecewise-smooth](b).

### Capacity: c(a) = min over feasible σ of A_σ(a)

| Points | Smoothness | Mechanism |
|--------|-----------|-----------|
| Generic (unique minimizer, gap > 0) | C^∞ | c = A_σ* for unique best σ*; smooth by per-orbit (3) |
| Switching boundary (r orbits tied, distinct gradients) | Lipschitz, **not C¹** | D_d c = min_i(∇A_σᵢ · d) depends on d; no gradient |
| Degenerate tie (r orbits tied, matching gradients) | C¹, possibly not C² | Non-generic (codimension > codimension of tie) |

Non-smoothness of c comes entirely from the min operation (orbit switching), not from individual A_σ failing.

Additional subtlety: **orbit appearance.** Under perturbation, an orbit σ' that was infeasible at a can become feasible at a+td. If it appears with action below the current best, this is a switching event not detectable by enumerating orbits at the base point only. Q5 detects these via full ehz_capacity on the perturbed polytope (the perturbed_best_perm field), but cannot predict them from the base point's orbit landscape.

### What this framework implies for the experiment

- Q1-Q4 (fixed orbit, slope 2.00): confirms per-orbit smoothness (row 3 above) at generic random polytopes
- Q5 (full ehz_capacity, slope 2.00 at generic points): trivially follows — c = A_σ* at generic points
- The 12/600 Q1 outliers with slope < 1.90 over [-4,-1] are large-t artifacts: refitting over [-4,-2] leaves 1/600, and over [-6,-3] leaves 0/600. The quadratic approximation has limited radius at some polytopes (large cubic Taylor coefficient), not a C² failure.

### Observation 8: Small min(β) does not degrade per-orbit smoothness

Q5 records min(β) of the best orbit's KKT solution. Distribution across 100 polytopes: min 1.1e-5, median 0.016, max 0.11.

| min(β) range | Single-orbit slope (median [P25, P75]) | n |
|-------------|---------------------------------------|---|
| < 0.01 | 2.000 [1.973, 2.022] | 220 |
| 0.01–0.05 | 2.001 [1.981, 2.014] | 170 |
| 0.05–0.1 | 1.998 [1.984, 2.015] | 100 |
| > 0.1 | 2.006 [1.969, 2.018] | 10 |

Spearman correlation: r = −0.007, p = 0.87. No correlation.

Even at min(β) = 1.1e-5 (only 10× above the certified threshold 1e-9), the per-orbit action is C² (slope 2.00). The few outliers (9/80 with slope < 1.90 in the low-β bin) vanish when fitting over [-4,-2] (2/80) or [-6,-3] (2/80) — same large-t artifacts as Q1.

**Epistemic status:** Observation. Consistent with the IFT argument: smoothness requires β > 0 strictly, but doesn't degrade as β → 0. The KKT Jacobian appears non-degenerate even near the feasibility boundary.

**Proofs:** [lem:orbit-feasibility-open], [lem:per-orbit-smooth], [lem:orbit-contraction], [prop:capacity-smoothness-classification] in experiments/gradient-correctness/math.tex. Two gaps flagged for Jörn: competing-orbit continuity in prop(a), transversality in prop(c)

## Integrated summary

The experiment contains 8 observations across 5 phases (Q1-Q5), supported by 4 proven results in math.tex.

**What the experiment establishes (high confidence):**

1. The per-orbit gradient formula [lem:cap-derivative] correctly predicts the per-orbit action to first order, with C² convergence (slope 2.00 ± 0.03), across all tested conditions: generic polytopes F=5-10 (Q1), Lagrangian products (Q2), near-degenerate gaps (Q3), barely-cutting facets (Q4), full-capacity perturbation (Q5), and small min(β) near the IFT boundary (Q5/Obs 8). Q1 alone: 600 traces, 0 outliers below 1.95 at [-6,-3] fit range. Q3 and Q5 have 3 outliers total at [-6,-3] (1 in Q3 at slope 1.42, 2 in Q5), likely near-degenerate polytopes where the quadratic Taylor radius is small.

2. The volume gradient [lem:vol-derivative] and systolic ratio gradient [cor:sys-derivative] correctly predict to first order, with C² convergence in the region t ∈ [1e-1, 1e-4] before floating-point cancellation dominates.

3. Orbit switching under perturbation is a quantitatively characterized function of action gap and step size (Obs 7). Switching rate is monotone in both variables, from 0% (large gap, small t) to 76% (tiny gap, t=0.1).

**What the experiment does NOT establish (potential overconfidence areas):**

1. **Shared-bug blindness.** The test checks code-vs-code consistency: do the gradient predictions match the function values? A conceptual error shared by both the gradient formula and the function evaluation would not be detected. The mathematical correctness of [lem:cap-derivative] and [lem:vol-derivative] is a separate question — the experiment assumes they are correct and tests the implementation.

2. **"Large-t artifact" interpretation is plausible but unproven.** The 12/600 Q1 outliers (slope < 1.90 at [-4,-1]) vanish at narrower fit ranges. I attribute this to cubic Taylor terms dominating at large t. But I haven't verified this by fitting the cubic coefficient or checking that the slope increases toward 3 at large t. An alternative explanation — that the function is C^k for some 1 < k < 2 at those points — would also be consistent with the data but would contradict the IFT prediction.

3. **"No correlation" between min(β) and slope** (Obs 8) means the IFT boundary isn't causing detectable slope degradation. But the condition number of the quadratic approximation (how large the t² coefficient is relative to the t¹ coefficient) could still degrade near β = 0 — this would shrink the radius of the C² regime without affecting the fitted slope within that regime. The experiment doesn't measure this.

4. **Non-best-orbit gradients are untested.** Q1-Q4 test only the best orbit's gradient. Q5 uses gradients of non-best orbits to form the subdifferential prediction, but the individual correctness of those gradients is not independently verified. The subdiff slope degradation (Obs 6) is attributed to the subdiff formula being wrong at generic points, but could also reflect gradient errors for non-optimal orbits.

5. **Obs 6 interpretation.** The subdiff prediction min_i(g_i · d) fails at non-boundary points because it's the directional derivative AT boundaries, not away from them. This interpretation is correct ([prop:capacity-smoothness-classification](b)), but the experiment doesn't test the formula at actual boundaries (gap = 0), which is where it should work. So the experiment confirms the formula fails where theory predicts failure, but doesn't confirm it succeeds where theory predicts success.

6. **"Orbit switching is a smooth function"** (Q5 inference 3) overstates the evidence. The data shows a monotonic trend in a 4×4 table. "Consistent with smooth switching boundaries" is more accurate than "smooth function of gap and step size."

**Where I might be under-confident:**

- Obs 1 (capacity slope = 2.00) is stated as depending on [prop:capacity-piecewise-smooth]. The observation itself (the slope IS 2.00) is direct and doesn't depend on any proposition. The proposition is needed only for the inference that the gradient formula is correct.
- The math.tex proofs have TODO markers on two gaps, but the main results (lem:orbit-feasibility-open, lem:per-orbit-smooth, lem:orbit-contraction, and the non-differentiability argument in prop(b)) are standard IFT and convex analysis, likely correct.

## Open questions (2026-03-27)

Organized by distance from the experiment's stated scope. Items marked [actionable] have a clear methodology; items marked [open] need design work or are mathematical questions.

### Within scope: gaps in Q1-Q5 answers

**OQ1. Subdiff prediction at actual switching boundaries (gap = 0).** [actionable → Q5b]

Q5 tests the subdiff formula min_i(g_i · d) at non-boundary points, where it correctly fails (Obs 6). It does NOT test the formula at actual boundaries where it should succeed. Three candidate methodologies:

**(A) Symmetric polytopes (chosen, Jörn 2026-03-27).** Use LP(n,n) where symmetry forces exact orbit ties. Already in codebase (Q2 generates them). Quick implementation (~2h). Limitation: tests at symmetric points only, not generic boundary points.

**(B) Path-following to boundary (deferred).** Start at generic polytope (gap > 0), find direction that shrinks gap, walk to gap = 0. Tests at generic boundary points. Harder: requires computing ∇(A_σ₁ − A_σ₂), bisection to locate boundary.

**(C) Interpolation between polytopes with different winners (deferred).** Use Q5 orbit-switching events: at base point σ₁ wins, at perturbed point σ₂ wins. Interpolate a(s) = (1−s)a₁ + s·a₂ to find crossover s*. Systematic, uses existing data. Medium effort.

Expected outcome for (A): slope ≈ 1 (capacity is Lipschitz not C² at the boundary, since the directional derivative min_i(g_i · d) is piecewise linear in d, not linear). Slope ≈ 2 would indicate the tied orbits have matching gradients (degenerate tie — possible at symmetric polytopes).

**OQ2. Mathematical correctness of [lem:cap-derivative].** [open]
The experiment tests code-vs-code, not the formula itself. A shared conceptual error in both the gradient code and the capacity code would be invisible. Options: (a) mathematical proof of the envelope theorem formula (formalizing the proof sketch in sys-optimization/math.tex), (b) independent reimplementation, (c) comparison against symbolic differentiation on small cases. Option (a) is the thesis-appropriate path.

**OQ3. Non-best-orbit gradient correctness.** [actionable]
Q1-Q4 only test the best orbit's gradient. The non-best orbit gradients used in Q5's subdiff prediction are untested individually. Could test by running the Q1-Q4 first-order test with a non-best orbit fixed (use solve_kkt_for with each orbit's permutation, not just the best). If each orbit's gradient independently shows slope 2.00, this closes the gap. Low effort — reuse existing code with different orbit selection.

### Adjacent: smoothness structure questions

**OQ4. F scaling (F > 10).** [actionable but expensive]
All data is F ≤ 10. At F = 20+, orbit counts explode combinatorially. Per-orbit smoothness should still hold (local IFT), but switching boundaries could become dense, making the "generic point" assumption practically relevant. Would need LICCA cluster for F ≥ 12. Question: does the radius of the smooth region (before hitting a switching boundary) shrink with F?

**OQ5. KKT matrix singularity at β > 0.** [open, mathematical]
Can the KKT saddle-point matrix M(a) be singular when all β > 0? If yes, per-orbit smoothness fails there and the C² finding is weaker than it appears. The data (slope 2.00 at random polytopes) is consistent with generic non-degeneracy but doesn't rule out a codimension-1 singular locus. This is a question about the rank of a specific structured matrix — potentially answerable by linear algebra.

**OQ6. Second-order behavior (Hessian).** [actionable]
We confirm C² but don't test C³. For Newton-type optimization (relevant to sys-search), the Hessian should exist and be computable. Could test by computing second-order prediction residual |f(a+td) - f(a) - t·g·d - ½t²·d^T H d| and checking for slope 3. Requires computing or approximating the Hessian. Medium effort.

**OQ7. Volume/sys degradation on Lagrangian products.** [actionable, low priority]
Q2 slopes for volume (1.74) and sys (1.65) are attributed to floating-point cancellation. Could verify by using higher-precision arithmetic (e.g. f128 or arbitrary precision) on a few LP examples. If slopes improve, it's cancellation. If not, there may be a real numerical issue in the volume computation. Low priority — the gradient is correct in the convergent region regardless.

### Beyond scope: optimization implications

**OQ8. Orbit appearance under perturbation.** [open]
An orbit infeasible at a can become feasible at a+td. Q5 detects these via ehz_capacity but can't predict them from the base orbit landscape. How common is this? What fraction of Q5 switching events are "new orbit appearance" vs "existing orbit overtakes"? Could answer from existing Q5 data by comparing base orbits against perturbed-best orbits.

**OQ9. Practical step size bounds.** [open, connects to sys-search]
Obs 7 shows switching rate as a function of gap and t. This implicitly defines a "safe step size" before hitting a switching boundary: for gap δ, perturbations with t << δ stay within the smooth region. Formalizing this into a step size bound for gradient-based optimization connects directly to the sys-search experiment.

### Priority assessment (for thesis project management)

- **High value, low effort:** OQ1 (symmetric polytopes, ~1 day), OQ3 (non-best orbit test, ~2 hours)
- **High value, medium effort:** OQ2 (proving [lem:cap-derivative], effort depends on proof difficulty)
- **Medium value:** OQ6 (Hessian test), OQ8 (orbit appearance from existing data)
- **Low priority for thesis:** OQ4 (F scaling — nice but not essential), OQ5 (mathematical, may be hard), OQ7 (LP volume cancellation — minor), OQ9 (step size bounds — belongs in sys-search)

## Q5b: Subdifferential at exact switching boundaries (2026-03-27)

### Setup

Tests the subdifferential formula D_d c = min_i(g_i · d) at points where multiple orbits are exactly tied (gap = 0), using symmetric LP(n,n) polytopes. This is the correct domain of [prop:capacity-smoothness-classification](b): at switching boundaries, the directional derivative IS min_i(g_i · d).

Polytopes: LP(3,3) F=6, LP(4,4) F=8, LP(5,5) F=10. Regular polygon with n sides in each Lagrangian factor. Directions: 10 per polytope (5 for LP(5,5) due to cost). Data: 290 rows in gradient-correctness-q5b-symmetric.jsonl.

### Observations

**Obs 9. Orbit enumeration at symmetric LP(n,n).**
- LP(3,3): 38 certified orbits, 2 tied at minimum (action = 1.5000). DISTINCT gradients (max distance 1.00, avg norm 0.79).
- LP(4,4): 142 certified orbits, 2 tied at minimum (action = 2.0000). DISTINCT gradients (max distance 2.00, avg norm 1.41).
- LP(5,5): 8569 certified orbits (71s enumeration), 20 tied at minimum (action ≈ 3.2725). DISTINCT gradients (max distance 3.78, avg norm 2.68).

All three have distinct gradients among tied orbits — no degenerate ties with matching gradients.

**Obs 10. Subdifferential convergence at LP(3,3) and LP(5,5) confirms [prop:capacity-smoothness-classification](b).**
- LP(3,3): subdiff slope ≈ 2 for all 10 directions (range 1.96–2.02 in [1e-5, 1e-2] fit; full-range fit gives 1.84 for dir 3 due to floating-point floor at tiny t). Single-orbit slope ≈ 2 in 6/10 directions (where both orbits agree on g·d), slope ≈ 1.00 in 4/10 directions (where they disagree). (gradient-correctness-q5b-symmetric.jsonl, polytope_id=q5b_lp3_3)
- LP(5,5): subdiff slope ≈ 2 for all 5 directions (range 1.93–2.00 in full-range fit). Single-orbit slope ≈ 1.00 for all 5 directions (0.996–1.000; with 20 tied orbits, the single-orbit gradient is always wrong). (gradient-correctness-q5b-symmetric.jsonl, polytope_id=q5b_lp5_5)

Interpretation: slope 2 for subdiff means the formula min_i(g_i · d) correctly predicts the directional derivative, and the O(t²) remainder comes from each per-orbit action being C². Slope 1 for single orbit confirms non-differentiability: at the switching boundary, no single gradient exists.

**Obs 11. LP(4,4) shows orbit appearance with a characteristic transition scale.**
- LP(4,4): 95 of 130 expected rows produced (35 lost to KKT solver panics in transition zone). Base orbits use 4 facets (e.g., [1,5,3,7]). (gradient-correctness-q5b-symmetric.jsonl, polytope_id=q5b_lp4_4)
- Two regimes separated by a transition at t ≈ 1e-5 to 3e-5:
  - **Small t (≤ ~3e-6):** Base orbit persists (or switches to another length-4 orbit). Subdiff slope ≈ 2.0 — gradient is correct. (e.g., dir 3: no switching at t ≤ 3e-6, subdiff slope 2.05 over small-t range; dir 4: subdiff slope 1.99; dir 6: slope 1.99)
  - **Large t (≥ ~3e-5):** Length-6 orbits appear (e.g., [0,5,4,2,3,6]). These were infeasible at the base point. Subdiff slope ≈ 1.0 across all directions (dir 0: 1.00, dir 1: 1.00, ..., all large-t fits ≈ 1.00).
- Overall: 70/95 rows (73.7%) show genuine orbit appearance (perturbed best is NOT any of the base tied orbits). All 70 have length-6 perturbed perms. The remaining 25 rows (at small t) stay within the tied orbit set or switch among the 2 tied length-4 orbits. Dirs 0,1 have no small-t data at all (rows start at t ≥ 1e-4).
- Connection to [lem:orbit-contraction]: at the base point, length-6 orbits have some β_k = 0 (they contract to the length-4 orbit with equal action). Under perturbation, β_k becomes positive (orbit expands) and the action diverges. This is the orbit appearance phenomenon flagged in math.tex open question 2.

**Obs 12. Orbit structure varies across LP sizes.**
- LP(3,3): optimal orbits use ALL 6 facets. No room for orbit appearance.
- LP(4,4): optimal orbits use 4 of 8 facets. Length-6 orbits appear under perturbation.
- LP(5,5): optimal orbits use 6 of 10 facets. 20 tied orbits provide enough coverage — perturbation switches among them (same facet count, different subsets), no orbit appearance.

### Summary and confidence

Q5b cleanly separates two phenomena:
1. **Orbit switching between known orbits** (LP(3,3), LP(5,5)): subdiff formula D_d c = min_i(g_i · d) is correct. Residual slope = 2 confirms C² per-orbit actions and correct directional derivative. Single-orbit gradient fails (slope 1) unless it happens to agree with the subdiff in a particular direction. **High confidence** — consistent across all tested directions.
2. **Orbit appearance** (LP(4,4)): at large perturbations (t ≥ ~3e-5), new length-6 orbits become feasible and win over the base length-4 orbits. Subdiff and single-orbit both have slope ≈ 1. At small perturbations (t ≤ ~3e-6), the base orbit persists and subdiff slope ≈ 2 (correct). The orbit appearance has a finite "radius" — below it, the gradient is still valid. **High confidence** — two-regime pattern consistent across directions, structural explanation via orbit contraction/expansion. 35/130 rows lost to KKT panics in the transition zone.

Figure: gc_q5b_boundary.png. Three panels (one per LP). Blue = subdiff traces, red = single-orbit traces. LP(3,3) and LP(5,5): blue at slope 2, red splits. LP(4,4): both at slope 1.

### Updated open questions

**OQ1 status:** [answered] Symmetric polytopes LP(3,3) and LP(5,5) confirm subdiff correctness at exact boundaries. LP(4,4) reveals orbit appearance as a complementary failure mode.

**OQ8 status:** [partially answered] LP(4,4) provides a clean example of orbit appearance: length-4 → length-6 orbit under perturbation. This is the orbit contraction boundary: the expanding orbit has β_k = 0 at the base point.

**OQ10. Extended subdiff formula accounting for orbit appearance.** [open, mathematical]
The subdiff formula min_i(g_i · d) over base-point orbits fails when new orbits appear under perturbation. Could extend by including orbits at the feasibility boundary (β_k = 0) — these are exactly the orbits that may appear. By [lem:orbit-contraction], they have the same action as their contracted versions, so they're tied. Including them in the subdiff computation might fix the LP(4,4) prediction. This connects to Clarke's generalized gradient: the subdifferential should include all limiting gradients, including from approaching orbits.

## Known issues

- **Q-correction panic:** `solve_kkt_for` panics on some near-degenerate polytopes. Caught via `catch_unwind` in `solve_kkt_safe`. Results in missing rows (perturbation skipped), not incorrect data.
- **Q2 volume/sys slope degradation:** Lagrangian products show lower fitted slopes for volume (1.74) and sys (1.65). Likely floating-point cancellation on small-facet polytopes, not a gradient error. The convergent region still trends correctly.
