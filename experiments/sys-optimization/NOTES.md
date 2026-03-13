# sys-optimization: Developer Notes

Gradient-based optimization of the systolic ratio sys = c_EHZ^2 / (2 vol).

## Motivation

Random polytope sampling (random-sweep, random-product-sweep) has not found sys > 1. Instead of hoping to get lucky, we compute ∇_{(h,n)} sys and take directed steps. The H-representation P = {x : n_k · x ≤ h_k} gives us knobs: heights h_k and normals n_k.

## Files

| File | Purpose |
|------|---------|
| `sys_optimization.rs` | Rust binary: sensitivity, single steps, iteration, validity testing |
| `sys_optimization.py` | Python: figures and stats table |
| `sys-optimization.tex` | LaTeX writeup for thesis |
| `sys-optimization-sensitivity.jsonl` | Phase 1 output: per-polytope gradients |
| `sys-optimization-steps.jsonl` | Phase 2 output: single gradient step evaluations |
| `sys-optimization-iterations.jsonl` | Phase 3 output: iterative gradient ascent trajectories |
| `sys-optimization-validity.jsonl` | Phase 4 output: gradient prediction accuracy |

## Architecture

Four phases, all in one binary:

1. **Phase 1 (sensitivity):** Compute analytical ∂sys/∂h and ∂sys/∂n for 140 polytopes.
   Uses envelope theorem for capacity derivatives, swept-volume argument for volume derivatives.
   All derivatives FD-cross-checked in debug builds.

2. **Phase 2 (single steps):** For each polytope, try 5 step fractions × 2 step types (h-only, (h,n)).
   Step bound preserves combinatorial type (height positivity, vertex-facet incidence, ω₀ signs).

3. **Phase 3 (iteration):** Iteratively recompute HK2017 + gradient + step. Pick best of 10 candidates per iteration. Converges in ~6 iterations on average.

4. **Phase 4 (validity testing):** Test gradient prediction accuracy along gradient and random directions, at step sizes from 0.01×t_max to 10×t_max. Answers: is our gradient trustworthy? How far? How conservative are the step bounds?

## Key design decisions

- **Self-contained binary**: copies needed library internals (facet_volume_3d, KKT solve) rather than depending on unstable library APIs. This is per repo convention (new variants live in experiment binaries).

- **ValidOrbit struct**: extracts all KKT data (β*, Q*, ν, λ) needed for analytical derivatives. λ (closing constraint multiplier) was added specifically for normal derivatives.

- **Step bound for (h,n)**: in addition to height-positivity and vertex-crossing checks, enforces that ω₀(n_i, n_j) doesn't change sign for ridge-adjacent pairs. This is the binding constraint in practice.

- **Best-of-10 step selection in Phase 3**: at each iteration, try all 5 fractions × 2 types. This avoids committing to one step type and naturally adapts (early iterations prefer (h,n), later iterations shift to h-only).

## Known issues / future work

### Large steps beyond gradient regime
- Accept combinatorial type change, recompute sys from scratch
- Gradient prediction invalid, but actual sys still computable
- Could find better polytopes that the conservative step bound misses

### Targeted single-facet moves
- For k ∉ (S,σ): ∂c_EHZ/∂h_k = 0, so height changes only affect vol
- Large decrease in h_k could trigger orbit switch + volume reduction

### Convergence characterization
- 2 polytopes (random_3x5_0, random_4x5_6) admit no improving step from start — why?
- Is the converged sys a function of the combinatorial type? Of facet count?
- What's the relationship between initial sys and improvement magnitude?

### Adding facets (from original ideation)
- Instead of changing existing h_k/n_k, introduce new half-spaces that clip vertices
- Changes combinatorial type (more facets = exponentially more expensive HK2017)
- Deferred until h/n changes alone show their limits

### HKO2024 local maximality across ambient spaces
HKO2024 can be embedded in multiple ambient spaces (discussion 2026-03-13):
- **LP(Fq=5,Fp=5)**: current sys-optimization already covers this (∂sys/∂h, ∂sys/∂n at HKO2024)
- **General F=10**: same gradient data applies — but are there perturbation directions
  outside the Lagrangian product submanifold? The normal-perturbation directions ∂sys/∂n_k
  already break LP structure, so Phase 1 sensitivity data may already answer this.
- **Degenerate F=11+**: HKO2024 as a polytope with collapsed facets. Adding a facet is a
  direction invisible to the current gradient. This is the "adding facets" idea above.
- **Convex bodies**: increasing F as a discretization of smooth boundary perturbation.
  Track whether converged sys (Phase 3) changes as F grows.
Key question: does the Phase 1 data already show that all gradient components point
"inward" (sys decreases in every direction) at HKO2024? If so, that's evidence for
local maximality in the F=10 ambient space. Check sensitivity JSONL for HKO2024 entry.

## Learnings from past data

- ∇_h sys and ∇_n sys have different units (h has length scale, n dimensionless) — norms not comparable
- Predicted max Δsys (t_max × ‖∇sys‖): similar for both types (mean 0.34 h-only, 0.15 (h,n))
- (h,n) step bound is 1-3 orders of magnitude smaller (ω₀ sign constraint)
- Yet actual (h,n) improvement is larger — landscape more curved in n-directions than linear prediction suggests
- Net effect: (h,n) steps give ~59% more improvement per single step
- Iterative improvement is ~3× single-step improvement (0.149 vs 0.054 mean Δsys)
- Best sys achieved: 0.878 (from 0.351 start, random_5x5_1)
- No polytope reaches sys > 1

### Phase 4 validity findings
- Height gradient: excellent predictor (<5% error at 0.25×t_max, O(t) growth)
- (h,n) gradient: bimodal — good for ~125/140 polytopes, ~15 outliers near orbit boundaries
  - Median pred/actual ratio 1.55 (systematic overprediction from normal renormalization)
- Random directions: ~90% relative error at all scales (expected: random ⊥ gradient in high dim)
- Step bounds are conservative: 85% construction success at 2×t_max, 60% at 10×t_max
- Type preservation drops fast: 35% at 2×t_max, 19% at 10×t_max
- Validity is strongly non-spherical (direction-dependent, not radius-dependent)

### Wrong-sign gradient predictions (investigated 2026-03-02)
Of 4990 validity records, 1977 (39.6%) have negative actual_delta_sys (gradient direction decreases sys). Two mechanisms identified:

1. **Combinatorial type change** (32 records, all at t ≥ 1.0×t_max): `vertex_count_changed=True` — the polytope's vertex structure changes, so the gradient computed at the original polytope is meaningless for the perturbed one.

2. **Reeb orbit switching** (remaining cases at moderate t, no vertex change): a different (S,σ) candidate becomes the capacity-achieving orbit after the step. The gradient was computed for the original orbit, which is no longer the minimizer.

The 39.6% aggregate is dominated by random-direction records (~90% error rate at all scales, which is expected since random directions are nearly orthogonal to the gradient in high dimension) and large-step records. For the (h,n) gradient at small steps, only 15/140 polytopes show wrong-sign predictions — consistent with the .tex writeup.

The iterative optimizer (Phase 3) avoids both failure modes: its line search stays in the linear regime (t_fraction ≤ 0.95) and recomputes HK2017 after each step, detecting orbit switches immediately. Phase 3 has zero negative-delta-sys steps.

## Jörn verification status

All three items previously flagged as requiring verification are resolved:

- Lemma [lem:vol-derivative-normal]: math approved (59ddc2c) — `% Jörn: math approved` marker in sys-optimization.tex
- Lemma [lem:cap-derivative-normal]: math approved (59ddc2c) — marker explicitly covers "H-term sign chain"
- Sign chain in capacity normal derivative proof: covered by the [lem:cap-derivative-normal] approval above
