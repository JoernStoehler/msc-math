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

All directions and perturbations work directly in dual-vertex (a) space: a'_k(t) = a_k + t·d_k.

### Dataset

- 140 polytopes (60 random F=5-10, 80 Lagrangian products F=6-10)
- ~44 directions per polytope: 1 gradient, 1 neg-gradient, 10 random, 4F coordinate (one per component per facet)
- 6160 anatomy rows, 5644 crossing attempts, 5531 successful crossings, 5531 gradient rows

### RQ1: What causes combinatorial type changes?

| Event type | Count | Fraction |
|------------|-------|----------|
| Incidence flip | 4221 | 68.5% |
| ω₀ flip | 1937 | 31.4% |
| Unbounded | 2 | 0.0% |

(anatomy JSONL, all 6160 rows)

**Two kinds of boundaries.** Incidence flips (vertex gains a new facet) are the most common. ω₀ flips (ω₀(n_i, n_j) changes sign for ridge-adjacent facets) are the second-most common and occur because a-space directions rotate normals. No dual-vertex degeneration events (directions are unit-normalized).

### RQ2: sys is continuous; orbits switch 10% of the time

sys is continuous at all 5531 tested boundaries: max |Δsys| = 3.34e-3 (crossing JSONL, boundary_sys_continuity.png).

**Orbit switches occur at 10% of boundaries** (550/5531). Broken down by event type:
- Incidence flips: 8.8% orbit switch rate (339/3845)
- ω₀ flips: 12.5% orbit switch rate (211/1686)

ω₀ flips are slightly more likely to trigger orbit switches than incidence flips.

### RQ3: Gradient is discontinuous at orbit-switching boundaries

| Metric | Median | p95 | Max |
|--------|--------|-----|-----|
| Gradient angle change | 0.002° | 6.7° | 75.2° |

(gradient JSONL, all 5531 rows)

8.1% of boundaries have gradient angle change > 1°, 3.9% have > 10°. The large gradient jumps correlate with orbit switches: when the optimal orbit changes, the gradient can rotate by up to 75°.

**Key implication for optimization:** gradient-based search must handle sudden direction changes at ~10% of boundaries. A step that crosses such a boundary may overshoot badly if it assumes the gradient is constant.

### RQ4: Boundary density

**Boundary distance decreases with F** (boundary_tmax_vs_F.png). Gradient directions hit boundaries soonest (boundary_density_cdf.png).

**Gradient directions** encounter boundaries earliest, followed by neg-gradient, then random, then coordinate. Coordinate directions have a long tail because single-component perturbations are less constrained.

### Crossing success rate

98% of boundary crossings succeed (5531/5644). Only 113 failures, all from near-degenerate capacity computation at the boundary. This is dramatically better than the initial (incorrect) h-only implementation which had 55% failure rate.

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (as expected from the min-of-continuous-functions structure of c_EHZ). But the gradient can jump by up to 75° when the optimal orbit switches.

**Orbit switches happen at ~10% of first boundaries.** This means that even the first gradient step can encounter an orbit switch. Optimization strategies need to detect and handle these transitions rather than assuming smooth behavior within a "large" combinatorial cell.

**ω₀ flips are significant.** They account for 31% of boundaries and have a higher orbit-switch rate (12.5%) than incidence flips (8.8%). Any boundary-crossing strategy that only handles incidence flips would miss a third of the boundaries.

**Boundary density constrains gradient ascent.** The gradient direction hits boundaries faster than random directions, confirming that gradient ascent inherently pushes toward combinatorial boundaries.

## Open questions

1. **Multi-boundary sweep:** What happens further along the path? How many boundaries must be crossed before an orbit switch? What fraction of orbit switches are reversible (orbit switches back after a later boundary)?
2. **Non-simple vertex handling:** The conservative step bound for non-simple vertices (Lagrangian products) underestimates t_max. How much data are we missing?
3. **Orbit switch prediction:** Can we predict which boundaries will cause orbit switches from the boundary geometry (event type, which vertex/facet, proximity to other boundaries)?
