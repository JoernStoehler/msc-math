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

## Learnings from past data

- Normal gradient dominates height gradient by 1-2 orders of magnitude
- But (h,n) step bound is 1-3 orders of magnitude smaller (ω₀ sign constraint)
- Net effect: (h,n) steps give ~59% more improvement per single step
- Iterative improvement is ~3× single-step improvement (0.149 vs 0.054 mean Δsys)
- Best sys achieved: 0.878 (from 0.351 start, random_5x5_1)
- No polytope reaches sys > 1

## Jörn verification required

- Lemma [lem:vol-derivative-normal]: ∂vol/∂n_k · δ = -S_k(x̄_k · δ)
- Lemma [lem:cap-derivative-normal]: envelope theorem for ∂Q*/∂n_k with H and N dependence
- Sign chain in capacity normal derivative proof (noted by pre-reviewer)
