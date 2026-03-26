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
cargo run --release --bin combinatorial_boundaries   # ~11 min, generates 3 JSONL files
python3 combinatorial-boundaries/analyze.py          # generates figures
```

Requires: `random-sweep/random-sweep.jsonl` and `random-product-sweep/random-product-sweep.jsonl`.

## Results (2026-03-26)

All directions and perturbations work directly in dual-vertex (a) space: a'_k(t) = a_k + t·d_k. The ω₀ sign flip detection uses sign(ω₀(a_i, a_j)) directly (not unit normals) — ω₀(a_i(t), a_j(t)) is quadratic in t by bilinearity, so flip times are exact roots.

### Dataset

- 140 polytopes (60 random F=5-10, 80 Lagrangian products F=6-10)
- 12 directions per polytope: 1 gradient, 1 neg-gradient, 5 dense random (uniform on S^{4F-1}), 5 sparse random (one random facet perturbed)
- 1680 anatomy rows, 1516 successful crossings, 1516 gradient rows

### RQ1: What causes combinatorial type changes?

| Event type | Count | Fraction |
|------------|-------|----------|
| Incidence flip | 1073 | 63.9% |
| ω₀ flip | 606 | 36.1% |

(anatomy JSONL, all 1680 rows, 1 unbounded excluded)

**Two kinds of boundaries.** Incidence flips (vertex gains a new facet) dominate. ω₀ flips (sign(ω₀(a_i, a_j)) changes for ridge-adjacent facets) are the second-most common.

### RQ2: sys is continuous; orbits switch at 7.7% of boundaries

sys is continuous at all 1516 tested boundaries: max |Δsys| = 3.86e-4 (crossing JSONL, boundary_sys_continuity.png).

**Orbit switch rate depends strongly on direction sparsity:**

| Direction type | Crossings | Orbit switches | Rate |
|----------------|-----------|----------------|------|
| Sparse random | 642 | 90 | 14.0% |
| Gradient | 140 | 5 | 3.6% |
| Neg gradient | 74 | 3 | 4.1% |
| Dense random | 660 | 18 | 2.7% |

(crossing JSONL, construction_ok_after=true rows)

**Sparse perturbations cause 5× more orbit switches than dense perturbations** (14% vs 2.7%). Perturbing a single facet is a targeted change that is much more likely to flip the optimal orbit than a smooth perturbation of all facets. This is a key structural finding: the orbit landscape has many boundaries in "single-facet" directions but few in "all-facet" directions.

### RQ3: Gradient is discontinuous at orbit-switching boundaries

| Metric | Median | Max |
|--------|--------|-----|
| Gradient angle change | 0.002° | 70.2° |

(gradient JSONL, all 1516 rows)

The gradient is effectively constant at non-switching boundaries (median 0.002°) but can rotate by up to 70° at orbit-switching boundaries. Phase 3 runs on all probes, confirming that gradient stability away from orbit switches is not an assumption but a validated observation.

### RQ4: Boundary density

**Boundary distance decreases with F** (boundary_tmax_vs_F.png). Gradient and sparse directions hit boundaries sooner than dense random (boundary_density_cdf.png).

### Crossing success rate

100% of boundary crossings succeed (1516/1516). No construction or capacity failures.

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (consistent with the min-of-continuous-functions structure of c_EHZ). The gradient can jump by up to 70° when the optimal orbit switches.

**Sparsity is the key predictor of orbit switches.** Sparse perturbations (one facet) cause 14% orbit switches; dense perturbations (all facets) cause 2.7%. This means: moving one facet at a time is much more likely to change which orbit is optimal. Gradient ascent (dense, structured) encounters orbit switches rarely (~3.6%), which is favorable for optimization — but boundary-crossing strategies that overshoot into sparse directions would encounter orbit switches frequently.

**ω₀ flips are significant.** They account for 36% of boundaries. Any step-bound computation that only tracks incidence flips would miss a third of the boundaries.

**Boundary density constrains gradient ascent.** The gradient direction hits boundaries faster than dense random directions.

## Open questions

1. **Orbit switch prediction:** Can we predict which boundaries will cause orbit switches from the boundary geometry (event type, which vertex/facet, proximity of second-best orbit's action to best orbit's action)?
2. **Continuity of sys:** The observation that sys is continuous at boundaries is consistent with the min-of-continuous-functions structure of c_EHZ, but a formal proof that new orbits enter continuously (not just that existing orbit actions are continuous) may be worth writing up for the thesis.
