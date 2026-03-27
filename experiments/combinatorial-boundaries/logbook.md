# Combinatorial Boundaries: Logbook

## Motivation

When deforming a polytope by moving dual vertices a_i, the combinatorial type (vertex-facet incidence, ω₀ signs) can change. These changes affect which orbit minimizes the action, creating discontinuities in the gradient. Understanding this structure is needed to design step algorithms that cross boundaries effectively (see `sys-search` experiment).

## Status

**Phases A-E complete (2026-03-27).** Per-facet cell profiling, convexity testing, orbit gap, gradient-cell alignment. 140 polytopes, 11200 per-facet probes, 980 global probes, 2800 convexity tests.

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

## Results: boundary anatomy (2026-03-26, updated 2026-03-27)

All directions and perturbations work directly in dual-vertex (a) space: a'_k(t) = a_k + t·d_k. The ω₀ sign flip detection uses sign(ω₀(a_i, a_j)) directly (not unit normals) — ω₀(a_i(t), a_j(t)) is quadratic in t by bilinearity, so flip times are exact roots.

### Dataset

- 140 polytopes (60 random F=5-10, 80 Lagrangian products F=6-10)
- Global probes: 7 directions per polytope (1 gradient, 1 neg-gradient, 5 dense random)
- 980 anatomy rows, 873 successful crossings, 873 gradient rows

### RQ1: What causes combinatorial type changes?

| Event type | Count | Fraction |
|------------|-------|----------|
| Incidence flip | 556 | 56.7% |
| ω₀ flip | 424 | 43.3% |

(anatomy JSONL, 980 rows, 0 unbounded)

**Two kinds of boundaries.** Incidence flips (vertex gains a new facet) are more common in global probes (57%). ω₀ flips (sign(ω₀(a_i, a_j)) changes for ridge-adjacent facets) account for 43%. In per-facet probes, incidence flips are even more dominant (69.5%) — see Phase A results below.

### RQ2: sys is continuous; orbits switch at 3% of boundaries

sys is continuous at all 873 tested boundaries: max |Δsys| = 2.91e-4 (crossing JSONL, boundary_sys_continuity.png).

Orbit switch rate across all global crossings: 26/873 (3.0%). Gradient ascent (dense, structured) encounters orbit switches rarely, which is favorable for optimization.

### RQ3: Gradient is discontinuous at orbit-switching boundaries

| Metric | Median | Max |
|--------|--------|-----|
| Gradient angle change | 0.002° | 70.2° |

(gradient JSONL, all 873 rows)

The gradient is effectively constant at non-switching boundaries (median 0.002°) but can rotate by up to 70° at orbit-switching boundaries. Gradient measurement runs on all probes, confirming that gradient stability away from orbit switches is a validated observation, not an assumption.

### RQ4: Boundary density

**Boundary distance decreases with F** (boundary_tmax_vs_F.png). Gradient direction hits boundaries sooner than dense random (boundary_density_cdf.png).

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (consistent with the min-of-continuous-functions structure of c_EHZ). The gradient can jump by up to 70° when the optimal orbit switches.

**Sparsity is the key predictor of orbit switches.** Sparse perturbations (one facet) cause 14% orbit switches; dense perturbations (all facets) cause 2.7%. This means: moving one facet at a time is much more likely to change which orbit is optimal. Gradient ascent (dense, structured) encounters orbit switches rarely (~3.6%), which is favorable for optimization — but boundary-crossing strategies that overshoot into sparse directions would encounter orbit switches frequently.

**ω₀ flips are significant.** They account for 36% of boundaries. Any step-bound computation that only tracks incidence flips would miss a third of the boundaries.

**Boundary density constrains gradient ascent.** The gradient direction hits boundaries faster than dense random directions.

## Results: cell geometry (2026-03-27)

Refactored to three passes: per-facet cell profiling (11200 probes, no EHZ), global probes with crossing + gradient (980 probes, with EHZ), convexity testing (2800 midpoint checks). Instrumented EHZ provides orbit gap per polytope. Runtime: 47s.

### Phase A: Per-facet cell profiling

10 random S³ directions per facet per polytope. 11200 profiling rows total.

| Metric | Orbit facets | Non-orbit facets |
|--------|-------------|-----------------|
| Probes | 8387 | 2810 |
| Median t_max | 0.258 | 0.124 |

(profiling JSONL, t_max < 100, cell_orbit_vs_nonorbit.png)

**Orbit facets have 2× wider cells than non-orbit facets.** This is the opposite of what one might expect ("orbit facets are bottlenecks"). Instead, the facets *not* in the optimal orbit have narrower cells — perturbing them quickly changes the polytope's combinatorial type. This makes sense: non-orbit facets are not constrained by the optimal Reeb orbit and can be closer to degeneracy.

**Cells are highly anisotropic:** median max/min t_max ratio within a facet's R⁴ is 8.3×, with extreme outliers up to 9051× (cell_anisotropy.png). Cell cross-sections are elongated ellipsoids, not spheres. Some directions within a facet's R⁴ have much more room than others.

**Event types in per-facet probes:** incidence flips dominate even more strongly in per-facet probes (69.5%) than in global probes (56.7%). ω₀ flips are 30.5% (profiling_event_types.png). Moving a single dual vertex directly changes vertex positions (which determines incidence) while ω₀ changes require coordinated movement of two dual vertices — per-facet directions move only one.

**Cell width decreases with F** for both orbit and non-orbit facets (cell_width_by_F.png). More facets = more constraints = smaller cells.

### Phase B: Convexity testing

2721 successful midpoint constructions out of 2800 tests.

| Check | Failure rate |
|-------|-------------|
| Incidence change | 0.8% |
| ω₀ sign change | 52.5% |

(convexity JSONL, midpoint_construction_ok=true rows, cell_convexity.png)

**Cells are NOT convex.** 52.5% of midpoint tests show ω₀ sign changes, confirming that ω₀ boundaries are quadric hypersurfaces (not hyperplanes). Two points inside the same combinatorial cell can have a midpoint in a different cell due to ω₀ sign flips.

Incidence is almost always preserved (99.2%), consistent with incidence boundaries being (approximately) hyperplanes — the vertex-facet slack is affine in the dual vertices.

**Same-facet vs cross-facet:** same-facet pairs have 38.7% ω₀ failure, cross-facet pairs have 54.6% (cell_convexity.png). Cross-facet midpoints are worse because they combine perturbations of two different dual vertices, creating more opportunity for the bilinear ω₀ to change sign.

**Implications for optimization:** Non-convexity means that line searches cannot assume the combinatorial type is constant along a straight-line interpolation between two interior points. However, since incidence is preserved 99.2% of the time, the non-convexity is primarily about ω₀ signs — and ω₀ sign boundaries may be less disruptive than incidence changes (they don't change the skeleton, only the symplectic structure).

### Phase C: Gradient-cell alignment

Correlation between gradient boundary distance (t_max in gradient direction) and the narrowest per-facet cell width: r = 0.518 (gradient_cell_alignment.png).

**Moderate positive correlation.** The gradient doesn't specifically target narrow cell directions — it correlates positively with cell width. Polytopes with wider cells tend to have more room in the gradient direction too. This is favorable for optimization: the gradient isn't systematically pushing toward the nearest boundary.

### Phase D: Orbit gap

132/140 polytopes have ≥2 valid orbits. 8 polytopes (all F=5 random) have a single valid orbit.

| Metric | Value |
|--------|-------|
| Median gap | 0.054 |
| Min gap | ~0 (near-degenerate) |
| Max gap | 13.76 |

(anatomy JSONL, orbit_gap field, orbit_gap_distribution.png)

The distribution is heavily right-skewed: most polytopes have small orbit gaps, meaning the second-best orbit has nearly the same action as the best. This means orbit switches are "nearby" in action-value space for most polytopes.

**Orbit gap predicts orbit switches** (orbit_gap_vs_switch.png): polytopes in the lowest gap quartile (≤0.007) have higher orbit switch rates at boundaries than those in the highest quartile. But the signal is noisy with only 26 total orbit switches across 873 crossings.

### Updated global probe results

With the sparse/dense directions replaced by gradient + neg-gradient + 5 dense random (7 directions per polytope, 980 total):

| Metric | Value |
|--------|-------|
| Crossing success rate | 873/873 (100%) |
| Orbit switch rate | 26/873 (3.0%) |
| Max |Δsys| | 2.91e-4 |
| Gradient angle: median | 0.002° |
| Gradient angle: max | 70.2° |

(crossing JSONL, gradient JSONL)

Results consistent with Phases 1-3: sys is continuous, gradient is stable except at orbit switches.

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (consistent with the min-of-continuous-functions structure of c_EHZ). The gradient can jump by up to 70° when the optimal orbit switches.

**ω₀ flips are significant.** They account for 43% of global-probe boundaries and dominate per-facet probes. Any step-bound computation that only tracks incidence flips would miss a third of the boundaries.

**Cells are non-convex but "incidence-convex".** ω₀ sign boundaries are quadric, causing 52.5% of midpoint tests to fail. But incidence is preserved 99.2% of the time. The practical impact for optimization depends on whether ω₀ sign changes affect the optimal orbit (they can, since the set of feasible cycles depends on the transition matrix, which depends on ω₀ signs).

**Orbit facets are wider, not narrower.** The intuition that "orbit facets are bottlenecks" is wrong — non-orbit facets are narrower. This suggests that optimization is free to move orbit facets significantly without hitting a boundary, while non-orbit facets are more constrained. A step-size strategy should account for this asymmetry.

**High anisotropy (8.3×) constrains step selection.** Within each facet's R⁴, some directions have much more room than others. An isotropic step (equal perturbation in all directions) will hit a boundary in the narrow direction long before exhausting the wide direction. Anisotropy-aware step selection could take larger steps by aligning with the wide directions of each facet's cell.

**Gradient-cell alignment is favorable (r = 0.518).** The gradient doesn't push toward the narrowest cell boundary. This is good news for gradient ascent: the step-size limit imposed by cell geometry is correlated with, not opposed to, the gradient direction.

**Boundary density constrains gradient ascent.** The gradient direction hits boundaries faster than dense random directions. But the gradient boundary distance correlates with overall cell width, so narrower cells constrain everything, not just the gradient.

## Open questions

1. **Continuity of sys:** The observation that sys is continuous at boundaries is consistent with the min-of-continuous-functions structure of c_EHZ, but a formal proof that new orbits enter continuously (not just that existing orbit actions are continuous) may be worth writing up for the thesis.
2. **ω₀ non-convexity impact:** Does ω₀ sign non-convexity actually affect optimization? If the optimal orbit's feasibility is unchanged by ω₀ flips in non-orbit facet pairs, the non-convexity may be harmless for gradient ascent.
3. **Anisotropy structure:** What determines the anisotropy directions within each facet's R⁴? Is it related to the positions of adjacent facets, the symplectic structure, or both?
4. **Step-size strategy:** Can per-facet cell widths be used to construct an anisotropic step-size bound that allows larger steps than the isotropic bound?
