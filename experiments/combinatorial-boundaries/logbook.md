# Combinatorial Boundaries: Logbook

## Motivation

When deforming a polytope by moving dual vertices a_i, the combinatorial type (vertex-facet incidence, ω₀ signs) can change. These changes affect which orbit minimizes the action, creating discontinuities in the gradient. Understanding this structure is needed to design step algorithms that cross boundaries effectively (see `sys-search` experiment).

## Status

**Complete (2026-03-27).** Per-facet cell profiling, convexity testing (with transition matrix), orbit gap, gradient-cell alignment, products-vs-random split, multi-boundary sweep. 140 polytopes, 6 JSONL files, 15 figures. Key findings: cells are convex for random polytopes but not for Lagrangian products; a gradient step crosses ~F boundaries; gradient-cell alignment is favorable.

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
cargo run --release --bin combinatorial_boundaries   # ~60s, generates 6 JSONL files
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
| ω₀ sign change (ridge-adjacent) | 52.5% |
| Transition matrix change | 57.5% |

(convexity JSONL, midpoint_construction_ok=true rows, cell_convexity.png)

**Cells are NOT convex.** 57.5% of midpoints have a different transition matrix (= different set of feasible Reeb orbits). This is the operationally relevant measure: the transition matrix determines which cycles are feasible in HK2017.

The transition matrix depends on vertex adjacency AND ω₀ signs for all vertex-adjacent pairs (not just ridge-adjacent). Ridge-adjacent ω₀ flips account for 52.5% of failures; the remaining 5% come from ω₀ flips in vertex-adjacent but non-ridge-adjacent pairs. Every ridge-adjacent ω₀ flip also changes the transition matrix (100% overlap).

Incidence is almost always preserved (99.2%), consistent with incidence boundaries being (approximately) hyperplanes — the vertex-facet slack is affine in the dual vertices.

**Failure rate increases sharply with F:** ~0% at F=5, ~50% at F=6, ~65% at F≥8 (cell_convexity.png). More facets means more ω₀ pairs that can flip.

**Same-facet vs cross-facet:** same-facet 50.5% transition failure, cross-facet 58.6%. Cross-facet midpoints are worse because they combine perturbations of two different dual vertices, creating more opportunity for the bilinear ω₀ to change sign.

**Implications for optimization:** Non-convexity means that line searches cannot assume the combinatorial type is constant along a straight-line interpolation between two interior points. The transition matrix changes alter which orbits are feasible, so this is not merely a cosmetic issue — it can change which orbit minimizes the action and thus affect the gradient.

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
| Crossing coverage | 873/980 (89.1%) |
| Orbit switch rate | 26/873 (3.0%) |
| Max |Δsys| | 2.91e-4 |
| Gradient angle: median | 0.002° |
| Gradient angle: max | 70.2° |

(crossing JSONL, gradient JSONL)

The 107 missing crossings (10.9%) are concentrated in neg_gradient directions (66/107) and at low F (F=5,6: 60/107). The neg_gradient direction moves away from the sys optimizer, creating polytopes where EHZ has numerical issues. Not a structural limitation.

### Products vs random polytopes

Splitting all metrics by source dataset reveals a fundamental structural difference:

| Metric | Random | Lagrangian product |
|--------|--------|-------------------|
| Transition failures (convexity) | **0/1156 (0%)** | **1565/1565 (100%)** |
| Orbit gap median | 0.163 | 0.008 |
| Orbit-facet cell width | 0.169 | 0.363 |
| Anisotropy | 10.6× | 7.0× |

(all existing JSONL files, grouped by source_dataset)

**Non-convexity is entirely a product phenomenon.** The 57.5% overall rate is a Simpson's paradox — random polytopes have 0% transition failures, products have 100%. This makes structural sense: Lagrangian products have special ω₀ relationships between cross-factor facet pairs (near-zero values that flip easily under perturbation). Random polytopes have generic ω₀ values far from zero.

**Product orbit gaps are 20× smaller** (median 0.008 vs 0.163), meaning products are generically near-degenerate — the second-best orbit is almost as good as the best.

### Multi-boundary sweeps

Walk along a direction for distance 1.0, iteratively stepping past each boundary. 560 sweeps (140 polytopes × 4 directions: gradient, neg-gradient, 2 dense random).

| F | Median boundaries | Mean | Max |
|---|-------------------|------|-----|
| 5 | 1 | 2.1 | 6 |
| 6 | 3 | 3.2 | 13 |
| 7 | 4 | 5.5 | 17 |
| 8 | 7 | 8.0 | 36 |
| 9 | 10 | 10.8 | 33 |
| 10 | 14 | 14.4 | 45 |

(sweep JSONL, budget=1.0, sweep directions: gradient + neg-gradient + 2 dense random)

**A typical gradient step crosses ~6 boundaries** (median across all F and directions). This scales roughly linearly with F: ~1 at F=5, ~14 at F=10. The inter-boundary spacing is median 0.026, meaning boundaries are closely packed.

**Event type distribution in sweeps:** incidence flips 75.6%, ω₀ flips 24.4%. Incidence flips dominate multi-boundary paths more than single-boundary probes (57/43 split). This makes sense: as the polytope deforms further from the starting point, vertex positions accumulate larger changes, crossing more incidence boundaries.

**36% of sweeps end by construction failure** before exhausting the distance budget. This limits the practical range of multi-step algorithms — after ~6-14 boundary crossings, accumulated perturbation creates near-degenerate polytopes.

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (consistent with the min-of-continuous-functions structure of c_EHZ). The gradient can jump by up to 70° when the optimal orbit switches.

**ω₀ flips are significant.** They account for 43% of global-probe boundaries. Any step-bound computation that only tracks incidence flips would miss almost half the boundaries.

**Non-convexity is a product phenomenon, not generic.** Random polytopes have empirically convex cells (0% midpoint transition failures at F≤10). Lagrangian products have 100% failure — every midpoint test changes the transition matrix. This is due to the special symplectic structure of products: cross-factor ω₀ values are near-zero and flip easily.

**Gradient ascent should plan for ~F boundaries per step.** A step of distance ~1 crosses a median of F boundaries (linearly in F). Most of these are benign (gradient angle change ~0.002°), but any one could trigger an orbit switch (3% per boundary). Over F boundaries, the probability of at least one orbit switch is roughly 1-(0.97)^F ≈ 15-35% for F=5-10.

**Orbit facets are wider, not narrower.** Non-orbit facets are the bottleneck (median 0.124 vs 0.258). Optimization is free to move orbit facets significantly without hitting a boundary.

**High anisotropy (8.3×) constrains step selection.** Within each facet's R⁴, some directions have much more room than others. Anisotropy-aware step selection could take larger steps by aligning with wide directions.

**Gradient-cell alignment is favorable (r = 0.52).** The gradient doesn't push toward the narrowest cell boundary.

## Open questions

1. **Continuity of sys:** The observation that sys is continuous at boundaries is consistent with the min-of-continuous-functions structure of c_EHZ, but a formal proof that new orbits enter continuously (not just that existing orbit actions are continuous) may be worth writing up for the thesis.
2. **Anisotropy structure:** What determines the anisotropy directions within each facet's R⁴? Is it related to the positions of adjacent facets, the symplectic structure, or both? Deferred unless sys-search needs anisotropic steps.
3. **Construction failure after multi-boundary crossing:** 36% of sweeps fail. Is the failure mode predictable? Can the sweep be made more robust (e.g., by normalizing dual vertices after each step)?
