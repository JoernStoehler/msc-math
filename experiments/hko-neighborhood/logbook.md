# HKO-Neighborhood: Logbook

## Motivation

HKO2024 is the only known counterexample to Viterbo's conjecture: a 10-facet Lagrangian product polytope (pentagon x pentagon at theta=-90 degrees) with systolic ratio sys ~= 1.047 > 1 (hko-neighborhood-sensitivity.jsonl, `sys` field). The central question of this experiment is whether HKO2024 is a **local maximum** of the systolic ratio.

Why this matters: if HKO2024 is a local maximum, that constrains how a proof of local maximality might work and informs the structure of the counterexample region. If it is NOT a local maximum, there exist nearby polytopes with even higher sys, which would be new, potentially stronger counterexamples. Either answer is valuable for thesis conclusions.

**Key insight (Jörn, 2026-03-13):** HKO2024 lives in multiple ambient spaces simultaneously, and "local maximum" means different things depending on the embedding:

| Ambient space | DOF | What's perturbed | Status |
|---|---|---|---|
| LP(Fq=5, Fp=5) | ~20 | Pentagon vertex positions | Covered by pentagon-perturb |
| General polytopes F=10 | 40 | All normals + heights | Covered by Phase A (this experiment) + sys-optimization |
| LP(Fq=6, Fp=5) | ~30 | Degenerate: one q-facet collapsed | Covered by Phase B (this experiment) |
| General polytopes F=11+ | 44+ | Facet-splitting: add a new halfspace | Covered by Phase B (this experiment) |
| General polytopes F=13 | 52 | 3 facets collapsed | Not tested |
| Convex bodies | infinite | Smooth boundary perturbation, F->inf | Not tested |

Each embedding gives different perturbation directions. A local max in one space does not imply local max in a larger space.

## Status

**Active.** Phase A and Phase B data generated, writeup drafted (hko-neighborhood.tex), figures produced. Handoff file exists at `/workspaces/msc-math/handoffs/hko-neighborhood.md` with open review items.

## How to run

```bash
# Generate all data (Phase A + Phase B)
cd experiments/ && cargo run --bin hko_neighborhood --release

# Generate figures
python3 experiments/hko-neighborhood/analyze.py

# Phase C: LP test for local maximality (requires scipy)
python3 experiments/hko-neighborhood/phase_c_lp_test.py
```

### Files

| File | Role |
|---|---|
| `run.rs` | Rust binary (~2100 LOC): sensitivity analysis, gradient ascent, facet splitting |
| `analyze.py` | Python figures + analysis |
| `math.tex` | Formal math: symmetry analysis, orbit structure, subdifferential, consequences |
| `hko-neighborhood-sensitivity.jsonl` | Phase A: gradients at HKO2024 (1 row, large — all 44 orbit gradients inline) |
| `hko-neighborhood-ascent.jsonl` | Phase A: gradient ascent trajectory (1 row — converged in one step) |
| `hko-neighborhood-splitting.jsonl` | Phase B: facet-splitting data (211 rows) |
| `hko-neighborhood-gradient.png` | Figure: bar chart of d_sys/d_h_k |
| `hko-neighborhood-orbits.png` | Figure: orbit structure visualization |
| `hko-neighborhood-splitting.png` | Figure: splitting results |
| `phase_c_lp_test.py` | Phase C: LP test for 0 ∈ conv(per-orbit gradients) in (n,h)-space |

## What's been done

### Phase A: Sensitivity analysis in F=10 space (normals + heights)

**What:** Compute analytical derivatives d_sys/d_h_k and d_sys/d_n_k at the exact HKO2024 polytope. Run gradient ascent in joint (h, n) space. Track all near-optimal Reeb orbits.

**Findings:**

1. **Per-orbit height derivatives have mixed sign.** For the best orbit (S={0,2,4,6,8,9}), the d_sys/d_h values are: +0.198 (visited, small beta), -0.518 (unvisited), +0.640 (visited, large beta) (hko-neighborhood-sensitivity.jsonl, `d_sys_h` field). Positive at visited facets (increasing height grows both capacity and volume, but capacity faster), negative at unvisited facets (only volume grows). The 10 distinct per-orbit gradients form one orbit under the symplectic symmetry group; their average is zero by symmetry.

   **NOTE:** A now-deleted README.md (written before the sign bug fix in commit 6907406) claimed "All 10 height derivatives are negative (range: -0.52 to -1.68)." The post-fix data shows mixed signs. The .tex writeup has the correct description.

2. **Normal gradient is nonzero:** |grad_sys_n| ~= 1.53 (hko-neighborhood-sensitivity.jsonl, `gradient_norm_n` field). HKO2024 is NOT a critical point in the full (n, h) parameter space.

3. **Gradient ascent converges immediately:** One step, delta_sys = 0.0 (hko-neighborhood-ascent.jsonl, `delta_sys` field). The maximum step size t_max ~= 0.78 (hko-neighborhood-ascent.jsonl, `t_max` field). The converged sys equals the starting sys to machine precision. This suggests HKO2024 sits at or near a boundary of the feasible region where no smooth deformation improves sys.

4. **44 near-optimal orbits** out of 364 total valid orbits, all at essentially the same action (relative gaps < 5e-14) (hko-neighborhood-sensitivity.jsonl, `n_near_optimal`, `n_valid_orbits`, and per-orbit `relative_gap` fields). The orbit structure is highly degenerate.

5. **Orbit structure:** Each near-optimal orbit visits exactly 3 q-facets and 3 p-facets (6 of 10 total). The 44 orbits group into 10 distinct facet-set groups under the symplectic symmetry group of order 10 (generated by the diagonal rotation Delta_72 and the q-p exchange phi).

6. **Subgradient structure:** The per-orbit gradients have **mixed sign** — positive at visited facets, negative at unvisited ones. The 10 distinct per-orbit gradient vectors form a single orbit under the symplectic symmetry group. Their average is zero by symmetry (numerical residual ~7e-10). Three distinct derivative values correspond to: unvisited facets (d_cap/d_h = 0, d_sys/d_h ~= -0.518), visited with beta ~= 0.171 (d_sys/d_h ~= +0.198), visited with beta ~= 0.276 (d_sys/d_h ~= +0.640) (hko-neighborhood-sensitivity.jsonl, `d_sys_h` and per-orbit `beta` fields). The ratio of the two nonzero beta values is ~1.618 (golden ratio), characteristic of regular pentagon geometry.

7. **Symmetry analysis (in .tex):**
   - Full polytope symmetry group: order 50, isomorphic to (C5 x C5) semidirect Z2. All 10 facets equivalent.
   - Symplectic subgroup: order 10, generated by Delta_72 and phi. Only this subgroup maps Reeb orbits to Reeb orbits.
   - C5^q and C5^p (independent rotations in each Lagrangian plane) are NOT symplectomorphisms — they preserve K as a set but not the symplectic form.

### Phase B: Facet-splitting into F=11

**What:** Split one facet of HKO2024 into two by introducing a new halfspace (cutting plane close to an existing facet), creating an 11-facet polytope K' that is a sub-polytope of K (K' subset K). Test whether any such cut increases sys.

**Findings:**

1. **All tested cuts decrease sys.** 211 successful cuts in the current data (hko-neighborhood-splitting.jsonl, 211 rows):
   - Facet 0 (q-space representative): 200 rows (100 directions x 2 epsilon values)
   - Facet 5 (p-space representative): 11 rows

2. **Delta sys range:** best (closest to zero) = -5.09e-9, worst = -3.18e-4 (hko-neighborhood-splitting.jsonl, `delta_sys` field). Larger epsilon (deeper cuts) cause larger sys decrease.

3. **Epsilon values tested:** 1e-3 and 1e-4 (hko-neighborhood-splitting.jsonl, `epsilon` field). At 1e-4 the cuts are shallow enough that delta_sys approaches machine precision (~1e-9).

**Data discrepancy (?):** The Rust binary constants suggest N_SPLITTING_SAMPLES_PER_FACET=100, N_SPLITTING_MIXED=50, N_SPLITTING_CONTROL=20, implying ~536 planned cuts (2 facets × 100 directions + 48 mixed + 20 control, each at 2 epsilon values). The actual JSONL has 211 rows: 200 for facet 0 and 11 for facet 5. Facet 5 appears incomplete — the data may have been regenerated with early termination.

**Caveat (from README):** Phase B only tests sub-polytopes K' that are strict subsets of K. Joint perturbations (relax an existing halfspace while adding a cut) are not tested. Phase A's gradient ascent convergence suggests these also cannot help, but this is not proven.

## Interpretation

**In h-space (normals fixed):** No first-order improving direction exists (proved by symmetry + Euler homogeneity: uniform weights λ_i = 1/10 give 0 ∈ conv). Gradient rank 5 in R^10, giving 5 flat directions. This explains the gradient ascent convergence at machine precision — the subdifferential contains the origin. Local maximality itself is not yet established (flat directions need second-order analysis).

**In full (n, h) space for F=10:** No first-order improving direction exists (Phase C, 2026-03-23). The LP test confirms 0 ∈ conv(all 44 per-orbit gradients) in the 40D effective parameter space (residual ~7e-9). Individual orbits have nonzero normal gradients (||∇_n sys|| ≈ 1.5-1.8; hko-neighborhood-sensitivity.jsonl, `gradient_norm_n`), but different permutations of the same facet set give different n-gradients, and the full set of 44 provides enough directions for the convex hull to contain 0. (The 10 subset-unique representatives are insufficient.) Twenty orbits carry equal weight 1/20 in the LP solution. Gradient rank 24 in the 40D space gives 16 flat directions needing second-order analysis.

Note: the nonzero per-orbit normal gradient (|grad_n| ~= 1.53) does NOT mean HKO2024 fails to be a local max. The Danskin condition requires 0 ∈ conv(all gradients), not that individual gradients vanish. The "gradient points outward" interpretation from Phase A was partially correct — the feasibility boundary (omega_0 sign constraints) IS binding — but the deeper reason is the subdifferential structure.

**In the F=11 ambient space:** All tested facet-splitting directions decrease sys. This is consistent with local maximality but remains sampling-based (211 cuts tested). The F=11 space is not covered by the LP test (which is specific to F=10 continuous perturbations, not discrete facet-splitting).

**Overall:** The first-order necessary condition for local maximality (0 ∈ conv of subdifferential) is satisfied in the F=10 (n,h) parameter space — no first-order improving direction exists. Local maximality itself is not yet established: the 16 flat directions need second-order analysis. The F=11 and convex body directions remain untested analytically (sampling evidence only).

## Dead ends / deferred directions

### Joint perturbations in Phase B
Phase B only tests sub-polytopes (adding a constraint). Joint perturbations that simultaneously relax an existing constraint and add a new one could potentially increase sys. Deferred because Phase A's gradient ascent converges at machine precision, suggesting no smooth deformation (including joint ones) improves sys. Not formally ruled out.

### F=13 ambient space
HKO2024 can be viewed as a degenerate member of the F=13 space (3 facets collapsed). Not tested. Resume condition: if F=11 results were ambiguous (they weren't).

### Convex body limit (F -> infinity)
Using increasing F as a discretization of smooth boundary perturbation. Not tested. This is conceptually important (the original Viterbo conjecture is for all convex bodies, not just polytopes) but requires infrastructure for generating high-F approximations of HKO2024.

### Dense 2D slice boundary mapping
100 random samples give a histogram, not a boundary map. Dense sampling (1000-10000 points) in a 2D slice (e.g., two PCA directions) could map the sys=1 level set, showing the shape of the counterexample region. Mentioned in `IDEAS.md` (root) and pentagon-perturb logbook. Not attempted.

## Open questions

1. ~~**Is the gradient ascent convergence at machine precision real or numerical artifact?**~~ **RESOLVED (Phase C).** The subdifferential DOES contain the origin — verified via LP (residual ~7e-9) and by symmetry+homogeneity for h-space. The one-step convergence is real: every perturbation direction has at least one orbit whose sys gradient opposes it.

2. **Phase B facet 5 data incomplete.** Only 11 of expected ~200+ rows. Unknown whether intentional or regeneration bug. Needs investigation or regeneration.

3. **Cross-validation with sys-optimization.** Probably not done. Note: sys-optimization is a different experiment with limitations — it didn't use proper gradients and didn't look at cuts (introducing redundant halfspaces + checking subdifferential).

4. **Is there a theoretical argument for local maximality?** Partially resolved. Phase C verifies the first-order necessary condition (0 ∈ conv of subdifferential) via LP. The h-space case has a pure symmetry+homogeneity argument. The full (n,h) case relies on numerical LP (residual ~7e-9). Two gaps remain: (a) the 16 flat directions need second-order analysis, and (b) a structural explanation of *why* the pentagon geometry forces 0 ∈ conv (e.g., relating to the golden ratio β-structure or the order-10 symplectic symmetry) would strengthen the thesis.

5. **Saddle point vs local max?** First-order necessary condition satisfied (0 ∈ conv). The open question: do the 16 flat tangent directions have negative second-order change (local max), zero (plateau), or positive (saddle)? The gradient ascent convergence at machine precision and pentagon-perturb sampling both suggest local max, but no second-order analysis exists. This is the critical remaining gap.

6. **Facet-splitting + subdifferential.** Phase B tests cuts (adding a halfspace) and checks whether sys decreases. But it doesn't compute the subdifferential of the F=11 polytope at the cut. Computing the subdifferential after cutting would reveal whether the cut polytope is itself a local max in F=11 space, or whether there's a further ascent direction. This is the "introduce redundant halfspace, then look at subdifferential again" approach — not yet implemented.

7. **Neighborhood capacity landscape.** Beyond gradients at HKO2024: sample polytopes in a neighborhood (various distances, various directions), compute their sys, and map the landscape. This would show whether HKO2024 is an isolated peak, a ridge, or part of a plateau. Partially covered by pentagon-perturb (100 random perturbations in LP(5,5) space, all lower) but not done systematically in the full F=10 or F=11 space.

## Theoretical framework for local maximality (2026-03-23)

### The Danskin argument

sys is non-smooth at HKO2024 because c_EHZ = min over orbit actions, and 44 orbits achieve the minimum simultaneously. By Danskin's theorem composed with the smooth chain rule for sys = c²/(2·vol):

    D_d⁺ sys = min_{i ∈ active orbits} (∇sys_i · d)

where ∇sys_i is the per-orbit sys gradient and D_d⁺ is the one-sided directional derivative.

**Key consequence:** 0 ∈ conv({∇sys_1, ..., ∇sys_k}) ⟺ D_d⁺ sys ≤ 0 for all d (no first-order improving direction).

Proof: If 0 = Σ λ_i g_i (λ_i ≥ 0, Σ λ_i = 1), then for any d: min_i(g_i·d) ≤ Σ λ_i(g_i·d) = 0. Conversely, if 0 ∉ conv, separating hyperplane gives a direction d with all g_i·d > 0.

This is a **necessary condition** for local maximality, not sufficient. Along flat directions where D_d⁺ sys = 0, second-order terms can still make sys increase (e.g., min(x + y², -x + y²) has 0 ∈ conv at the origin but is not a local max due to the y² term). The 16 flat directions identified in Phase D are exactly this gap.

**Preconditions for Danskin:** The argument requires (a) each orbit's action A_i is smooth in the polytope parameters in a neighborhood of HKO2024, and (b) no currently-inactive orbit becomes the new minimizer in that neighborhood. Condition (b) is supported by a macroscopic gap: the 44 active orbits have relative gap < 5e-14, while the 45th best has gap > 0.01 (the run.rs near_optimal_threshold code comment states "any value in [1e-6, 0.1] gives the same 44 orbits").

### h-space local maximality: provable by symmetry

The 10 distinct per-orbit h-gradients form one orbit under the order-10 symplectic symmetry group ⟨Δ₇₂, φ⟩. This group acts transitively on the 10 gradient vectors, so the average (1/10)Σ g_i has all components equal (it is G-invariant). Furthermore, sys is degree-0 homogeneous in heights (scaling all h_k by α doesn't change sys = c²/(2·vol) since c ~ α and vol ~ α⁴ in R⁴... actually c ~ α² and vol ~ α⁴, so sys ~ α⁴/α⁴ = 1). By Euler's theorem, each per-orbit gradient satisfies Σ_k h_k · ∂sys_i/∂h_k = 0. Since all h_k = cos(π/5) are equal, Σ_k ∂sys_i/∂h_k = 0 for each orbit i. Combined with all-components-equal from symmetry, the average is zero. The uniform weights λ_i = 1/10 then give 0 = Σ λ_i g_i ∈ conv(g_1,...,g_10).

Note: the LP finds 0 ∈ conv but with gradient rank only 5 in R^10 (not full rank), so 0 is on the boundary, not interior. The 5 flat h-directions correspond to perturbations that don't change sys to first order. This is consistent with the symmetry structure: the 10 h-gradients have only 3 distinct values (unvisited, small-β visited, large-β visited), highly constrained by the (C5 × C5) ⋊ Z2 polytope symmetry.

This resolves open question 1 — the gradient ascent convergence is real, explained by the subdifferential containing the origin.

### (n,h)-space: open, testable via LP

The 10 per-orbit gradient vectors in R^{40} (10 heights + 30 normal DOF) are available in the sensitivity JSONL. Testing 0 ∈ conv(these vectors) is a **finite LP**: find λ_i ≥ 0, Σ λ_i = 1, Σ λ_i g_i = 0. The symmetry argument gives zero h-components automatically, but the n-components need the LP.

- Feasible → proves first-order local max in F=10.
- Infeasible → the LP dual gives an explicit improving direction → path to a better counterexample.

### Convex bodies: support function parameterization

Nearby convex bodies are parameterized by their support function h_{K'}(u) = h_K(u) + εf(u) for f: S³ → R. The Danskin argument extends: D_f sys = min_{i active} D_f sys_i. For local max among convex bodies: need 0 ∈ conv(per-orbit sys gradients) in the space of support function perturbations — an infinite-dimensional condition.

Two finite-dimensional approximations:
1. **Minkowski smoothing** K_ε = K + εB⁴: support function h_K + ε (constant perturbation). Computable by approximating K_ε with a high-F polytope.
2. **F-refinement**: approximate K with F=20,50,100 polytopes by adding tangent hyperplanes.

Both require c_EHZ computation at F >> 10, which is blocked by HK2017's exponential cost. The billiard algorithm handles Lagrangian products but not general high-F polytopes.

### Normal fan structure of the landscape

Near HKO2024, the sys landscape is **piecewise-smooth**: parameter space decomposes into cones based on which orbit determines c_EHZ. Within each cone, sys is smooth (single orbit active). At cone boundaries, orbits tie → kink. HKO2024 sits at the intersection of all 10 cone boundaries (maximum degeneracy due to symmetry).

This structure is determined by finitely many orbits and is much simpler than generic non-smooth optimization.

## Proposed experiments (2026-03-23)

Ranked by evidence value — ability to sharply update beliefs in either direction (conservation of expected evidence).

### Phase C: LP test in (n,h)-space [COMPLETED 2026-03-23]

**Script:** `phase_c_lp_test.py`

**Method:** Reconstruct KKT multipliers (μ, ξ) for each of the 44 near-optimal orbits from the stored (β, Q, permutation) data using the augmented saddle-point system. Compute per-orbit ∂sys/∂n_k via envelope theorem + tangent projection. Form full (h, n) gradient vectors and solve LP: find λ_i ≥ 0, Σ λ_i = 1, Σ λ_i g_i = 0.

**Critical finding:** using only 10 subset-unique orbits (as originally planned) gives LP INFEASIBLE. The n-gradient depends on the **permutation order** (via partial sums P_{i₀} = Σ_{j<i₀} β_j n_{σ(j)}), not just the facet subset. Two orbits with the same facet set but different cyclic orderings have the same h-gradient but different n-gradients. The 10 subset-unique orbits are insufficient; the full 44-orbit set is sufficient (whether a smaller subset also suffices was not tested).

**Results:**

| Test | Space | LP result | Gradient rank | Flat directions |
|---|---|---|---|---|
| h-space only | R^10, 10 DOF | 0 ∈ conv (10 unique-subset orbits) | 5/10 | 5D |
| Full (h,n) | R^50 ambient, 40 effective DOF | 0 ∈ conv (all 44 orbits) | 24/50 (24/40 effective) | 16D real + 10D gauge |

(Source: phase_c_lp_test.py stdout — no committed output file; regenerate with `python3 experiments/hko-neighborhood/phase_c_lp_test.py`)

**Interpretation:**
- **No first-order improving direction exists in the F=10 (n,h) parameter space** (necessary condition for local maximality). By Danskin's theorem, for every direction d in the 40D tangent parameter space, min_i(g_i · d) ≤ 0. Local maximality itself is not yet established (16 flat directions need second-order analysis). Result is computational (LP residual ~7e-9), not a mathematical proof.
- 0 is on the **boundary** (not interior) of conv(gradients): 20 of 44 orbits carry weight 1/20 each, 24 have zero weight. The 20 active orbits span all 10 distinct facet-subsets.
- **16 real flat directions** exist where D_d⁺ sys = 0. Second-order analysis is needed to determine if these are strict decreasing, constant, or saddle-like.
- Cross-check: all 44 per-orbit h-gradients match stored JSONL data to machine precision.

**Key subtlety:** The gradient ∂sys/∂n_k lives in T_{n_k}S³ (3D tangent space), not R^4. The (n,h) parameter space has 10 + 10×3 = 40 effective DOF, with 10 radial gauge directions. Jörn's suggestion to reparameterize using dual vertices a_i = h_i n_i ∈ R^4 (unconstrained) avoids this gauge issue entirely: ∇_{a_i} sys ∈ R^{40} with no projection.

### Phase D: Flat direction analysis [COMPLETED 2026-03-23]

Incorporated into Phase C script. The 16 real flat directions are all mixed (h+n), not pure-h or pure-n. The gradient matrix has singular value spectrum: top 10 values are 7.5, 7.4, 4.6, 4.1, 3.3, 2.9, 2.8, 2.7, 2.5, 2.4 — no sharp rank gap, suggesting the 24 "active" dimensions and 16 "flat" dimensions are numerically robust but not separated by orders of magnitude. (Source: phase_c_lp_test.py stdout — no committed output file.)

### Phase E: Minkowski smoothing sys(K + εB⁴) [tests smooth-body direction]

Approximate K + εB⁴ as a high-F polytope (sample many normals on S³, compute tangent hyperplanes). Compute sys for ε = 0.01, 0.001, 0.0001. This is the only proposed experiment that probes the convex-body direction — no existing experiment covers it.

Cost: high (needs high-F capacity computation). Could start with billiard algorithm if the rounded polytope retains enough Lagrangian structure, or with small F-increments.

### Phase F: F-refinement convergence

Generate polytopes K_F ⊃ K (or K_F ⊂ K?) with F = 12, 15, 20 that approximate HKO2024 at different resolutions. Track sys(K_F) as F increases. Tests whether polytope approximation converges from above or below.

### Bayesian experiment design notes

What observations would **decrease** confidence in local max:
- ~~LP infeasible in (n,h)-space → improving direction exists~~ **RESOLVED: LP feasible**
- sys(K + εB⁴) > sys(K) → not local max among convex bodies
- Some facet split increases sys → not local max in F=11+

What observations would **increase** confidence:
- ~~LP feasible → no first-order improving direction for F=10~~ **DONE ✓** (necessary condition; local max needs 2nd-order analysis of 16 flat directions)
- sys(K + εB⁴) < sys(K) with rate O(ε²) → smoothing strictly unfavorable
- All F=11 splits decrease sys (complete sampling)

**Remaining untested directions:** Phases E (Minkowski smoothing) and F (F-refinement) probe the convex-body neighborhood. Phase B completion (facet 5 data) covers F=11. Second-order analysis of the 16 flat directions determines strict vs non-strict max.

## Literature: BBLM2023 (added 2026-03-23)

**Paper:** Baracco-Bernardi-Lange-Mazzucchelli, "On the local maximizers of higher capacity ratios" (arXiv:2303.13348). Local copy: `papers/bblm2023/`, bib: `BBLM2023`.

**Main result (Theorem A):** Smooth star-shaped local maximizers of c_k-hat in dim 4 are precisely domains symplectomorphic to rational ellipsoids. For k=1 (= c_EHZ, our case), only the round ball.

**Why it doesn't apply to us:** Techniques require smooth strict convexity (Clarke functional, Legendre dual, C³ topology). Polytopes are excluded. The paper acknowledges non-smooth domains can beat smooth maximizers (Example 1.2: polydisk P(1,1) beats all ellipsoids for k≥3).

**Implication:** The question "is HKO2024 a local max of sys among polytopes?" is genuinely open. No existing theory addresses it. HKO2024 paper (line 642) calls for "further study of non-smooth domains with Besse-type dynamics." The full paper was read; nothing beyond the above is actionable for our work.

## Related experiments

- **pentagon-perturb** (`experiments/pentagon-perturb/`): Perturbation analysis of HKO pentagon in the LP(Fq=5, Fp=5) ambient space. 100 random perturbations (epsilon=0.01 per component) (source: pentagon-perturb logbook). All perturbed polytopes have lower sys. Includes PCA analysis.

- **sys-optimization** (`experiments/sys-optimization/`): Gradient-based optimization of sys for 140 polytopes including HKO2024. Provides the analytical gradient framework (envelope theorem for capacity derivatives, swept-volume for volume derivatives) that Phase A uses. Phase 3 iterative ascent starting from HKO2024 shows no improvement. Best sys across all polytopes: 0.878 (source: sys-optimization data — verify against that experiment's JSONL). No polytope reaches sys > 1.

- **gradient-descent** (`experiments/gradient-descent/`) — gradient ascent on F=10 polytopes. Related infrastructure for optimizing sys via gradient steps.
