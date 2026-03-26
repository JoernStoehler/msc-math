# Combinatorial Boundaries: Logbook

## Motivation

When deforming a polytope by moving dual vertices a_i, the combinatorial type (vertex-facet incidence, ω₀ signs) can change. These changes affect which orbit minimizes the action, creating discontinuities in the gradient. Understanding this structure is needed to design step algorithms that cross boundaries effectively (see `sys-search` experiment).

## Status

**Phase 1-3 complete (2026-03-26).** First-boundary anatomy, crossing evaluation, and gradient measurement for 140 polytopes × ~20 directions. Math.tex deferred.

## Research questions

1. **What causes combinatorial type changes?** Along a step path a(t) = a + t·d, what events occur — vertex-facet incidence flips, ω₀ sign changes, vertices appearing/disappearing? In what order?

2. **How does sys behave across a boundary?** Is it continuous? Does it have a kink? What happens to the optimal orbit — does it change, and if so, to what?

3. **How does the gradient change?** When the optimal orbit switches, how does ∂sys/∂a_k jump?

4. **How dense are boundaries?** For a typical polytope, how far is the nearest boundary in a random direction? In the gradient direction? How does this depend on F?

## Design notes

- The step-bound computation in existing experiments (gradient-descent, sys-optimization) already detects *when* a boundary is reached (t_max). This experiment should go further: characterize *what happens* at that boundary.
- May want to compare the implicit step path (continuous deformation) with the discrete step (jump to new polytope). These could differ if the step crosses multiple boundaries at once.

## Predecessor experiments

Characterization aspects of this question appear scattered across:
- sys-optimization Phase 4 (validity testing, gradient prediction accuracy across step sizes)
- gradient-search (overshoot mechanism, which implicitly crosses boundaries)
- hko-neighborhood Phase B (facet-splitting, which adds a boundary)

## Related experiments

- **gradient-correctness** — validates the gradient formula; this experiment studies what happens when the gradient changes
- **sys-search** — uses boundary-crossing strategies; this experiment characterizes the boundaries those strategies must navigate

## How to run

```bash
cd experiments/
cargo run --release --bin combinatorial_boundaries   # ~4 min, generates 3 JSONL files
python3 combinatorial-boundaries/analyze.py          # generates figures
```

Requires: `random-sweep/random-sweep.jsonl` and `random-product-sweep/random-product-sweep.jsonl`.

## Phase 1-3 results (2026-03-26)

### Dataset

- 140 polytopes (60 random F=5-10, 80 Lagrangian products F=6-10)
- ~20 directions per polytope: 1 gradient, 1 neg-gradient, 10 random, F coordinate
- 2800 anatomy rows, 1923 crossing attempts, 869 successful crossings, 869 gradient rows

### RQ1: What causes combinatorial type changes?

For height-only perturbations (h_k → h_k + t·g_k, normals fixed):

| Event type | Count | Fraction |
|------------|-------|----------|
| Incidence flip | 1925 | 68.8% |
| Height zero | 445 | 15.9% |
| Unbounded | 430 | 15.4% |
| ω₀ flip | 0 | 0% |

(anatomy JSONL, all 2800 rows)

**Incidence flips dominate:** a vertex approaches a non-incident facet until the slack reaches zero. ω₀ flips are absent because h-only perturbations don't change facet normals (so ω₀(n_i, n_j) stays constant). Height-zero events mean a facet "goes to infinity" — these are degeneracies, not crossable boundaries.

### RQ2: sys is continuous at boundaries

All 869 successful crossing evaluations show |Δsys| < 5.47e-5 (crossing JSONL). The sys-before vs sys-after scatter lies exactly on the diagonal (boundary_sys_continuity.png).

**No orbit switches observed:** the optimal orbit permutation is identical before and after the first boundary in all 869 cases (crossing JSONL, orbit_changed field).

This is consistent with the mathematical expectation: c_EHZ is a minimum over a discrete set of orbits, each with an action that depends continuously on a_k. A vertex gaining a new facet is a local change to the face lattice that typically doesn't affect which orbit achieves the minimum action.

### RQ3: Gradient is nearly unchanged at first boundary

Gradient angle change: max 0.10° across all 869 measurements (gradient JSONL). The gradient norm jump is O(1e-7). The gradient is effectively constant across the first combinatorial boundary.

**Key implication:** The orbit doesn't switch at the first boundary, so the gradient has no reason to jump. Gradient discontinuities would occur at boundaries where the optimal orbit switches — these are apparently not the first boundaries along any direction. This motivates the **multi-boundary sweep extension** (see below).

### RQ4: Boundary density

**Boundary distance decreases with F** (boundary_tmax_vs_F.png):

| F | Median t_max | IQR |
|---|-------------|-----|
| 5 | 1.61 | 1.33-2.38 |
| 6 | 1.56 | 0.91-2.20 |
| 7 | 0.76 | 0.38-0.97 |
| 8 | 0.67 | 0.29-1.32 |
| 9 | 0.33 | 0.11-0.56 |
| 10 | 0.26 | 0.09-0.50 |

(anatomy JSONL, non-unbounded rows, grouped by facet_count)

**Gradient directions hit boundaries sooner** than random or coordinate directions (boundary_density_cdf.png, boundary_tmax_by_direction.png). Gradient median t_max ≈ 0.28, random median ≈ 0.68. This confirms that gradient ascent pushes the polytope toward combinatorial boundaries faster than a random walk would.

### Construction failure rate

55% of incidence-flip crossing attempts fail even with proportional fallback epsilons up to 10% of t_max (1054/1923 failures). The polytope right after an incidence flip is often near-degenerate: the new vertex-facet incidence makes the skeleton barely non-simple, triggering numerical failures in either polytope construction (qhull) or capacity computation (KKT solver panics on small eigenvalues). Height-zero events always fail crossing evaluation (as expected — the polytope becomes unbounded).

## Interpretation

**The first combinatorial boundary is benign.** sys is continuous, the orbit doesn't switch, and the gradient barely changes. This means gradient ascent within a combinatorial cell can safely step to the boundary (at t_max) without losing accuracy. The interesting dynamics — orbit switches, gradient jumps, potential kinks in the sys landscape — happen at subsequent boundaries or at boundaries where the second-best orbit's action crosses the best orbit's action.

**Boundary density constrains gradient ascent.** At F=10, the median step budget is only 0.26 (in height-perturbation units), and the gradient direction hits boundaries ~2.5× sooner than random. This confirms the gradient-descent finding that the algorithm terminates because t_max shrinks, not because gradients vanish. It also quantifies the constraint: at F=10, a gradient step can only move ~0.26 before hitting a boundary.

## Open questions

1. **Multi-boundary sweep:** What happens further along the path? How many boundaries must be crossed before the orbit switches? Where are the orbit-switching boundaries?
2. **ω₀ flip characterization:** Adding normal perturbations (h,n gradient) would enable ω₀ flips. How do those compare to incidence flips?
3. **Crossing failure diagnosis:** Is the 55% failure rate due to polytope construction or capacity computation? Can it be reduced with better numerical handling?
4. **Non-simple vertex handling:** The conservative step bound for non-simple vertices (Lagrangian products) underestimates t_max. How much data are we missing?
