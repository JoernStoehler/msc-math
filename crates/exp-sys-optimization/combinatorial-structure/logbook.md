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

**36% of sweeps end by construction failure** before exhausting the distance budget. Failure modes: 84% "unbounded" (dual vertices no longer positively span R⁴ — the perturbation moved facets so far that containment is lost), 16% "facet redundant" (a facet was shrunk to nothing). Both are geometric degeneracy from accumulated perturbation, not numerical bugs.

**sys increases along gradient sweeps despite boundary crossings.** 71% of gradient sweeps end with higher sys than they started (median improvement 65%). 0% of neg-gradient sweeps increase sys (sanity check). The path is not monotonic (only 28% non-decreasing) — sys oscillates across boundaries, sometimes dipping before recovering. But the overall trend in the gradient direction is strongly positive.

**Orbit switches in the gradient direction tend to increase sys.** At single-boundary crossings: 4/5 gradient-direction orbit switches had positive Δsys, vs 6/20 for random directions. The gradient direction biases orbit switches toward better orbits.

## Interpretation

**sys is continuous but not smooth.** The systolic ratio is continuous across all combinatorial boundaries (consistent with the min-of-continuous-functions structure of c_EHZ). The gradient can jump by up to 70° when the optimal orbit switches.

**ω₀ flips are significant.** They account for 43% of global-probe boundaries. Any step-bound computation that only tracks incidence flips would miss almost half the boundaries.

**Non-convexity is a product phenomenon, not generic.** Random polytopes have empirically convex cells (0% midpoint transition failures at F≤10). Lagrangian products have 100% failure — every midpoint test changes the transition matrix. This is due to the special symplectic structure of products: cross-factor ω₀ values are near-zero and flip easily.

**Gradient ascent works across boundaries.** A step of distance ~1 crosses a median of F boundaries (linearly in F). Despite this, 71% of gradient sweeps increase sys (median +65%). The gradient direction is robust to boundary crossings: orbit switches in the gradient direction tend to find better orbits (4/5 positive). Sys-search can use aggressive steps that cross multiple boundaries rather than stopping at the first one.

**Orbit facets are wider, not narrower.** Non-orbit facets are the bottleneck (median 0.124 vs 0.258). Optimization is free to move orbit facets significantly without hitting a boundary.

**High anisotropy (8.3×) constrains step selection.** Within each facet's R⁴, some directions have much more room than others. Anisotropy-aware step selection could take larger steps by aligning with wide directions.

**Gradient-cell alignment is favorable (r = 0.52).** The gradient doesn't push toward the narrowest cell boundary.

## Cross-experiment notes

**sys-search (dev run, 2026-03-27):** 42 seeds, best sys=0.933, wiggle dominates overshoot 41/1. No contradictions with our findings — strong alignment:
- Overshoot never wins → consistent with 36% sweep construction failures from accumulated perturbation.
- Wiggle wins → consistent with random-polytope cells being convex (random re-entry lands in a valid cell).
- Step-bound is the limiter, not gradient vanishing → consistent with ~F boundaries per step.

**Unused synergies for sys-search to pick up:**
- **Wiggle strength from cell width.** sys-search uses 5% (unjustified). Per-facet median cell width (0.12–0.26) could inform this. Wiggle should exceed cell width to escape, but not so large as to cause construction failure.
- **Per-facet targeted wiggle.** Orbit facets are 2× wider → wiggle them more aggressively. Non-orbit facets are narrower → wiggle them less.
- **Orbit gap as escape trigger.** Small orbit gap means orbit switches are imminent even without combinatorial change. Could trigger earlier escape attempts.
- **Product-specific strategy.** 100% non-convexity means line search is unreliable for products. Overshoot is expected to fail; only wiggle (random re-entry) makes sense.

**Code overlap with sys-search:** Both experiments implement `compute_step_bound` in a-space with incidence + ω₀ detection. The sys-search version (run.rs:162-235) tracks only incidence flips, not ω₀ flips — it's missing 43% of boundaries. Should be unified or sys-search should copy our enriched version.

**Potential experiment reorganization:**
- The step-bound code (incidence + ω₀ detection) is now in 3 places: sys-optimization, combinatorial-boundaries, sys-search. Candidate for library promotion if stable.
- The gradient sweep (multi-boundary + sys tracking) could live in sys-search instead of here, since it directly answers a sys-search question. We "stole" it because the data was available.
- The products-vs-random split is relevant to every gradient experiment. Could become a standard analysis step in figure_config.py or a shared utility.

## Open questions

1. **Continuity of sys:** math.tex has a proof sketch (Prop. prop:sys-continuous). The polytope-specific argument gives lower semicontinuity; full continuity requires citing general c_EHZ continuity on convex bodies. Jörn to review.
2. **Anisotropy structure:** What determines the anisotropy directions within each facet's R⁴? Deferred unless sys-search needs anisotropic steps.
3. **Construction failure after multi-boundary crossing:** 36% of sweeps fail — 84% "unbounded" (lost positive spanning), 16% "facet redundant". Sys-search should detect and backtrack.
4. **sys-search ω₀ gap:** sys-search step bound doesn't detect ω₀ flips (only incidence). Missing 43% of boundaries. Impact unclear — may just cause slightly conservative steps (stopping at incidence boundaries before reaching ω₀ boundaries). Or may cause overshoot past undetected ω₀ boundaries.
