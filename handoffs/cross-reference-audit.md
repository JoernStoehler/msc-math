# Code-Math Cross-Reference Audit: crates/src/

**Date:** 2026-03-28
**Scope:** All `[lem:]`, `[thm:]`, `[prop:]`, `[def:]` references in .rs files under `crates/src/`.
**Math sources:** `crates/src/{geom,kkt,algorithms}/math.tex`, `experiments/sys-optimization/math.tex`.
**Method:** For each reference, read citing code and cited math entry, check implementation correctness and precondition satisfaction.

**Totals:** ~170 reference instances across 25 files, citing 45 distinct labels.

---

## Verdict Summary

| Status | Count | Description |
|--------|-------|-------------|
| OK | ~160 | Code implements what the math states; preconditions satisfied |
| MISMATCH | 3 | Code contradicts or fails to satisfy cited lemma |
| TODO | 2 | Cited label doesn't exist in math.tex (intentional gap, flagged) |
| HEADER-ONLY | 1 | Module header omits labels that appear in the file body |

---

## MISMATCHES

### M1. saddle_point_solver.rs:358 → [lem:q-error-bound] — CRITICAL

**Lemma states:** E = (9/2)||r||²/|λ_min| where |λ_min| = min_j |λ_j| > 0 over ALL eigenvalues of M, and M is invertible.

**Code does:** Computes pseudoinverse using only retained eigenvalues (above threshold τ = 10⁻³), but uses |λ_min| over ALL eigenvalues (including near-zero discarded ones) in the error bound formula. This makes E artificially large for near-singular M.

**Consequence:** The bound is too loose — solver panics on basic polytopes (simplex, hypercube) because the bound says Q is inaccurate when it actually isn't. Code comment at line 361 explicitly states "the bound is wrong."

**Status:** Known. TASKS.md documents this as requiring lemma replacement. The verify-numerics experiment is developing a tighter bound.

### M2. saddle_point_solver.rs:477 → [lem:well-defined] — HIGH

**Lemma states:** All solutions (β, μ, ξ) yield the same Q = β^T H β = −ξ.

**Code does:** Correctly computes Q from β₀ (pseudoinverse solution), satisfying the lemma. But the returned `result.beta` is overwritten with β_final (LP-shifted), which may NOT be in the true null space — it's an approximate null-space shift. The structural contract "result.beta and result.q correspond" is broken: Q was computed from β₀ but the returned β is β_final.

**Incomplete TODO at lines 479-481:** Comment trails off: "approximate null shifts cause O(α²|λ_j|) Q drift that is spurious..." — the sentence doesn't complete the mathematical claim.

### M3. saddle_point_solver.rs:534-544 → [lem:q-error-bound] — MEDIUM

**Code comment claims:** "4.5 = 9/2 comes from [lem:q-error-bound]: the KKT block structure identity δβ^T H δβ = δx^T M δx − 2r₂^T δμ − 2r₃δξ removes the ||H||/|λ_min|² term."

**Math.tex proof (Step 4):** The derivation is more nuanced. The comment's "removes the ||H||/|λ_min|² term" is an oversimplification — the actual proof bounds |δβ^T H δβ| ≤ 5||r||²/|λ_min| via a different argument (spectral decomposition + component bounds). The 9/2 constant IS correct, but the explanatory comment is misleading.

---

## TODOs (missing labels)

### T1. qp_assembly.rs:58 → [lem:dual-vertex-qp]

Label does not exist in any math.tex file. Code has explicit TODO:
```
// TODO [JÖRN]: Write [lem:dual-vertex-qp] in kkt/math.tex proving that this
// reparameterization correctly recovers the same optimal action as the
// normals/heights formulation.
```
Intentional gap, properly flagged.

### T2. tube/mod.rs:344 → [lem:rotation-increment-approx]

Label does not exist. Code has explicit TODO:
```
// Write [lem:rotation-increment-approx] proving (or disproving) that this
// heuristic via tr(transition_matrix) is correct.
```
The rotation increment computation uses an angle heuristic instead of the exact CH2021 transition matrix formula.

---

## OK — By Module

### geom/ (15 files, ~60 reference instances) — ALL OK

| File | Lines | Labels | Notes |
|------|-------|--------|-------|
| cross_product_4d.rs | 9, 31 | def:cross-product-4d | Cofactor formula matches definition |
| symplectic_form.rs | 10, 18, 28, 53 | def:symplectic-form, def:J0 | J₀ matrix and ω₀ bilinear form correct |
| polytope.rs | 9, 134-136, 192-194, 282, 408, 692 | def:polytope-dual, def:polar-body, lem:vertex-enumeration, lem:positive-span, lem:bounded-triples, lem:irredundancy, lem:rational-pipeline | Pipeline references all delegate to vertex_enumeration.rs correctly |
| vertex_enumeration.rs | 13, 128, 194, 238, 282, 337, 383, 431, 458, 574, 636, 766, 814, 911, 1174 | lem:vertex-enumeration, lem:positive-span, lem:irredundancy, prop:integer-cramer, lem:bounded-triples, prop:prefilter-bound | Most complex file. All formulas verified: Cramer numerators, det computation, feasibility gap, prefilter constant C=10⁴ |
| known_polytopes.rs | 12, 72, 104, 136, 170, 221, 259, 261, 286, 299, 337, 339, 365 | def:ehz-capacity, thm:hko-counterexample, def:lagrangian-product, prop:capacity-symplectic-product, def:symplectic-product | Capacity values and product formulas match |
| facet_volume.rs | 11, 209 | def:volume | Per-facet specialization correct |
| rational_arithmetic.rs | 13, 55, 148 | def:symplectic-form, lem:rational-pipeline | Exact-arithmetic design pattern enforced |
| reeb_trajectory.rs | 15, 76, 124, 241 | def:reeb-vector-field, lem:piecewise-linear-reeb | R_i = 2J₀a_i correct |
| volume.rs | 9, 20, 43, 60, 188 | def:volume | Simplex determinant formula correct |
| lagrangian_product.rs | 15, 28, 62 | def:lagrangian-product | Construction matches definition |
| qhull.rs | 8, 179 | def:volume | Volume via qhull correct |
| skeleton.rs | 12, 230 | def:face-lattice | Face lattice construction correct |
| validation.rs | 7, 34, 78 | lem:positive-span | Boundedness check correct |
| polygon.rs | 9, 37, 117 | def:polygon-h-rep, def:polygon-area | H-rep and shoelace area correct |

### kkt/ (4 files excl. saddle_point_solver, ~20 reference instances) — ALL OK

| File | Lines | Labels | Notes |
|------|-------|--------|-------|
| qp_assembly.rs | 11, 43, 111, 165 | lem:kkt | H matrix convention matches math.tex: H_{ij} = ω₀(a_{σ(i)}, a_{σ(j)}) for i<j, symmetrized. Augmented system [H,A,1; A^T,0,0; 1^T,0,0] correct |
| projection_solver.rs | 23, 52, 176 | lem:kkt | Q = (1/2)β^T Hβ correct. Constraint projection approach valid |
| rational_solver.rs | 16, 20, 32, 64, 98, 549 | lem:kkt, lem:well-defined | Exact Gaussian elimination. Null-space invariance correctly applied: Q computed from any solution in affine set |
| mod.rs | 9, 41, 130 | lem:kkt, lem:q-error-bound, lem:H-quadratic | q_value() uses 0.5 * β^T Hβ, matching lem:H-quadratic |

**Cross-cutting check — 1/2 factor:** All Q computations consistently use (1/2)β^T Hβ, matching lem:H-quadratic line 44.

**Cross-cutting check — H matrix indices:** qp_assembly.rs builds H[i,j] = ω₀(a_{σ(i)}, a_{σ(j)}) for i<j, symmetric. This matches math.tex lines 24-26 (antisymmetry of ω₀ makes the convention consistent).

### kkt/saddle_point_solver.rs (~20 reference instances) — 3 MISMATCH, rest OK

| Line | Label | Verdict |
|------|-------|---------|
| 232 | lem:kkt | OK — correctly applies KKT conditions |
| 283 | lem:kkt | OK — system assembly correct |
| 297 | lem:H-quadratic | OK — double sum formula correct |
| 350 | lem:q-error-bound | See M1 |
| **358** | **lem:q-error-bound** | **MISMATCH (M1)** |
| 393 | lem:well-defined | OK — correctly invokes invariance |
| 465 | lem:well-defined | OK — saves β₀ for Q |
| **477** | **lem:well-defined** | **MISMATCH (M2)** |
| **534-544** | **lem:q-error-bound** | **MISMATCH (M3)** |
| 584 | lem:kkt, lem:q-error-bound | OK — test header |

**Header issue:** Module doc (lines 11, 23) lists only [lem:kkt] and [lem:q-error-bound] but the file also references [lem:H-quadratic] and [lem:well-defined]. Header is incomplete. (Fixed during this session — [lem:H-quadratic] and [lem:well-defined] added to line 23.)

### algorithms/ (9 files, ~50 reference instances) — ALL OK

| File | Lines | Labels | Notes |
|------|-------|--------|-------|
| facet_adjacency.rs | 9, 28, 60 | lem:numerical-transition-feasibility, cor:adjacency-pruning | Checks necessary conditions (vertex adjacency + ω₀ ≥ 0). Condition (2) of lemma delegated to KKT solver — acceptable pruning design |
| capacity_accumulator.rs | 8 | alg:ehz, thm:billiard-characterization | Two-tier (certified/uncertain) tracking correct |
| billiard/mod.rs | 8-9, 20, 89, 107, 198 | thm:billiard-characterization, thm:bounce-bound, alg:billiard | k in {2,3} matches bounce bound of 3 |
| billiard/facet_classification.rs | 11, 27, 42 | lem:lagrangian-facets | Normal classification (q-type vs p-type) correct |
| billiard/block_enumeration.rs | 9, 11, 16, 67 | lem:sigma-structure, alg:billiard | ([Q|QQ][P|PP])^k pattern correct with cyclic symmetry removal |
| hk2017/mod.rs | 118, 238, 510, 948, 1246-47, 1376, 1544 | lem:numerical-transition-feasibility, def:ehz-capacity, thm:hko-counterexample, lem:kkt, lem:q-error-bound, thm:conformality, thm:sympl-invariance | All test annotations correct |
| hk2017/orbit_recovery.rs | 29, 42, 63, 85, 203, 313 | lem:base-point-recovery, rem:beta-to-tau, lem:shoelace | tau_k = T*beta_k correct. Base-point linear system A_S b = r correct. Shoelace action formula correct |
| hk2017/generate_capacity_fixtures.rs | 25, 239 | thm:sympl-invariance, thm:conformality | Fixture generation uses invariance properties correctly |
| tube/mod.rs | 19, 37, 119, 220, 308, 320, 340, 379, 386, 396, 403, 426, 455, 672, 734, 1140 | alg:tube, def:tube, def:tube-data, def:symplectic-polytope, def:rotation-increment, def:tube-extension, def:tube-close, lem:prune-empty, lem:prune-action, lem:prune-rotation, lem:prune-simple, lem:fixed-point | All pruning lemmas implemented correctly |

### Top-level files (derivatives.rs, dataset.rs) — ALL OK

| File | Lines | Labels | Notes |
|------|-------|--------|-------|
| derivatives.rs | 15, 193 | lem:cap-derivative, lem:vol-derivative (experiments/sys-optimization/math.tex) | Derivative formulas match exactly; both lemmas unverified in math.tex but code-formula correspondence is exact |
| dataset.rs | 9, 32 | def:systolic-ratio | sys = c^2/(2*vol) matches definition exactly |

---

## Structural Observations

1. **All mismatches are in one file** (saddle_point_solver.rs) and all relate to the error bound machinery. The core KKT solving, H matrix assembly, and Q computation are correct everywhere.

2. **Two missing lemmas** (lem:dual-vertex-qp, lem:rotation-increment-approx) are both properly flagged as TODOs in the code — no silent gaps.

3. **facet_adjacency.rs** implements only condition (1) of lem:numerical-transition-feasibility (omega_0 >= 0) as a pruning filter, not the full condition (2). This is by design — it's a necessary condition used for fast rejection, with the KKT solver providing the full feasibility check downstream.

4. **Header completeness:** saddle_point_solver.rs module doc omitted [lem:H-quadratic] and [lem:well-defined] from its reference list despite using both. Fixed during this session.

---

## Action Items

1. **M1 (critical):** The lem:q-error-bound mismatch is known and tracked in TASKS.md. The verify-numerics experiment should produce a replacement bound that accounts for pseudoinverse truncation. No code change needed until the new lemma is written.

2. **M2 (high):** Complete the TODO at saddle_point_solver.rs:479. Either prove that beta_final's Q drift is bounded and document it, or restructure the return value so beta and Q are always from the same solution.

3. **T1:** Write [lem:dual-vertex-qp] in kkt/math.tex, or remove the dual-vertex code path if equivalence cannot be proven.

4. **T2:** Write [lem:rotation-increment-approx] in algorithms/math.tex, or replace the heuristic with the exact CH2021 formula.
