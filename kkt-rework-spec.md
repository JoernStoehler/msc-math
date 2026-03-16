# KKT Solver Rework — Spec / Requirements / Plan

Working document. Jörn and Claude iterate until the design is gap-free, then hand off to implementation sessions.

## Voice conventions

- `<j>` Reviewed by Jörn.
- `<q>` Question needing Jörn's attention.
- `<s>` Suggestion from Claude. Jörn accepts/rejects/modifies.
- `<f>` Fact verified from code/thesis (with source reference).

---

## 1. Current system

<f>
The current KKT solver (`crates/src/kkt.rs`) solves the symmetric saddle-point system:

```
[ H   N   η ] [ β ]   [ 0 ]
[ N^T 0   0 ] [ μ ] = [ 0 ]
[ η^T 0   0 ] [ ξ ]   [ 1 ]
```

where:
- H is m×m with H_{ij} = ω₀(n_{σ(i)}, n_{σ(j)}) (symmetric, from antisymmetry of ω₀ applied with different index order)
- N is m×4 with N_{id} = n_{σ(i),d} (facet normals)
- η is m×1 with η_i = h_{σ(i)} (facet heights)
- β ∈ R^m: dwell-time-like coefficients (τ_k = c_EHZ · h_{σ(k)} · β_k, from simple-minimizer-existence.tex:204)
- μ ∈ R^4: Lagrange multiplier for closure N^T β = 0
- ξ ∈ R: Lagrange multiplier for normalization η^T β = 1
- M is (m+5)×(m+5), symmetric

Source: kkt.rs:228-283, appendix-numerical.tex:330-336
</f>

<f>
The Reeb vector on facet F_i is R_i = (2/h_i) J₀ n_i. (basic-definitions.tex:478)

The constraints mean:
- N^T β = 0: closure condition (orbit closes up). Derived from Σ τ_k R_{σ(k)} = 0 → Σ (τ_k/h_{σ(k)}) n_{σ(k)} = 0 → Σ β_k n_{σ(k)} = 0.
  (general-case-algorithm-proof.tex:118-120)
- η^T β = 1: normalization (fixes the scale). (general-case-algorithm.tex:35)
- β > 0: each facet is visited with positive dwell time.

Source: general-case-algorithm.tex:31-37
</f>

<f>
Solution method: eigendecomposition of M = VΛV^T. Two-tier thresholding for near-null eigenvalues (permissive 1e-12 absolute floor, strict 1e-3 relative to |λ_max|). Pseudoinverse x̂ = Σ_i (v_i · b / λ_i) v_i for retained eigenvalues. Null-space search for β > 0 when rank-deficient.

Q value: Q(β) = (1/2) β^T H β, with c_EHZ = (max Q)^{-1}. Residual-corrected: Q̃ = Q(β̂) + (r₂^T μ̂ + r₃ ξ̂). Error bound E = (9/2) ‖r‖² / |λ_min|.

Source: kkt.rs:60-83, 383-428
</f>

---

## 2. Parameterization: (n, h) → dual vertices a_i

<j>
Switch from (n_i, h_i) ∈ S³ × R>0 to a_i = n_i/h_i ∈ R⁴\{0} (dual polytope vertices). Rationale: n_i being unit length is never used; a_i is unconstrained; Reeb vector becomes R_i = 2 J₀ a_i; gradients ∂/∂a_i need no tangent-space projection.
</j>

<f>
Code already stores dual vertices as rationals in the polytope representation (used for vertex enumeration via Cramer's rule: v_S = N_S^{-1} h_S, which is the primal vertex computation). The dual vertices a_i = n_i/h_i are not explicitly stored but trivially computed.

Source: appendix-numerical.tex:78-85
</f>

<q>
Impact on formulas when switching to a_i = n_i/h_i:

The constraint system currently is:
- N^T β = 0 → Σ β_k n_{σ(k)} = 0
- η^T β = 1 → Σ β_k h_{σ(k)} = 1

With a_i = n_i/h_i, we have n_i = h_i a_i. Substituting:
- Σ β_k h_{σ(k)} a_{σ(k)} = 0 (closure, 4 equations)
- Σ β_k h_{σ(k)} = 1 (normalization, 1 equation)

This is less clean than I expected — the h_{σ(k)} factors don't cancel. Unless we also reparameterize β.

If we define γ_k = β_k h_{σ(k)} (absorbing the height), then:
- Σ γ_k a_{σ(k)} = 0 (closure)
- Σ γ_k = 1 (normalization becomes a simplex constraint)
- γ_k > 0 ⟺ β_k > 0 (since h > 0)

And Q(β) = (1/2) β^T H β where H_{ij} = ω₀(n_{σ(i)}, n_{σ(j)}) = h_{σ(i)} h_{σ(j)} ω₀(a_{σ(i)}, a_{σ(j)}).

In terms of γ: Q = (1/2) Σ_{i,j} (γ_i/h_{σ(i)}) (γ_j/h_{σ(j)}) h_{σ(i)} h_{σ(j)} ω₀(a_{σ(i)}, a_{σ(j)}) = (1/2) Σ_{i,j} γ_i γ_j ω₀(a_{σ(i)}, a_{σ(j)}).

So with γ_k = β_k h_{σ(k)} and A_{ij} = ω₀(a_{σ(i)}, a_{σ(j)}):
- Constraints: Σ γ_k a_{σ(k)} = 0, Σ γ_k = 1, γ > 0
- Objective: Q = (1/2) γ^T A γ

This IS cleaner: the constraint matrix becomes [A_matrix; 1^T] with A_matrix being 4×m (rows of a_{σ(k)}^T) and the normalization is just Σ γ = 1.

Jörn: is this the right reparameterization? Does it interact cleanly with the dwell time formula? (Currently τ_k = c_EHZ · h_{σ(k)} · β_k = c_EHZ · γ_k, so dwell times are directly proportional to γ.)
</q>

---

## 3. Solver variant A: augmented system (current, to be maintained)

<f>
Current implementation in kkt.rs. Keep for ablation comparison. Known issues:

1. Near-null eigenvalues of M mix β-directions and multiplier-directions — geometric meaning unclear
2. β > 0 check: when eigenvalue threshold makes system look full-rank, unique β with some β_k < 0 causes node rejection (kkt.rs:455-477). This is wrong if a different threshold would reveal null-space directions allowing β > 0.
3. Two separate null-space search functions (find_positive_beta_1d, find_positive_beta_nd) — should be unified as LP feasibility.

Source: kkt.rs:451-477, 118-226
</f>

---

## 4. Solver variant B: constraint projection (new)

<s>
Proposed algorithm:

**Step 1: Solve constraints.**
Let C = [N^T; η^T] be the 5×m constraint matrix with RHS d = [0;0;0;0;1].

Compute rank(C):
- rank < 5: degenerate (parallel normals, etc.) — handle separately
- rank = 5, m < 5: overdetermined, generically no solution → no valid orbit for this (S, σ)
- rank = 5, m = 5: unique β, determined entirely by constraints. Check β > 0 and compute Q.
- rank = 5, m > 5: (m-5)-dim affine solution space. Proceed to Step 2.

For m > 5 with rank 5: find particular solution β₀ and null-space basis V (m × (m-5) matrix) such that every solution is β₀ + V α for α ∈ R^{m-5}.

**Step 2: Project H onto constraint space.**
The projected objective is Q(α) = (1/2)(β₀ + Vα)^T H (β₀ + Vα).
Expanding: Q(α) = (1/2)(α^T V^T H V α + 2 β₀^T H V α + β₀^T H β₀).
Stationarity: V^T H V α = -V^T H β₀.

Let H' = V^T H V (symmetric (m-5)×(m-5)) and b' = -V^T H β₀.
Solve H' α = b'.

**Step 3: Handle near-null eigenvalues of H'.**
Eigendecompose H' = U Λ' U^T. Near-null eigenvalues of H' have clear meaning: constant-action directions in the feasible set.

Conservative approach: include near-null eigenvalues in the null space. This is safe because:
- Constraints are already satisfied by construction
- Q is (approximately) constant along these directions
- Expanding the null space only makes the β > 0 LP easier

So: retain eigenvalues with |λ'_i| > threshold. The solution set is an affine subspace in α-space, hence also in β-space.

**Step 4: Check β > 0 as LP feasibility.**
The set of candidate solutions is {β₀ + V(α₀ + W γ) : γ ∈ R^k} where α₀ is the particular α-solution and W spans the null space of H'.

Check: does this affine subspace intersect {β > 0}?

This is an LP feasibility problem: find γ such that β₀ + V(α₀ + W γ) > 0 componentwise. Equivalently: (VW) γ > -(β₀ + V α₀).

If feasible: pick the Chebyshev center (maximize min_j β_j) for numerical robustness.

**Step 5: Recover Lagrange multipliers.**
Given β, solve Hβ + Nμ + ηξ = 0 for (μ, ξ). This is a 5×5 system (5 unknowns in 4+1 = 5 equations from the m stationarity conditions). It's overdetermined (m equations, 5 unknowns) — solve via least squares.
</s>

<q>
Questions for Jörn:

1. For the conservative near-null eigenvalue inclusion (Step 3): is there a downside beyond losing uniqueness? If Q truly has a flat direction, any point along it gives the same Q, so we just need any β > 0. But if the eigenvalue is small-but-nonzero, including it in the null space introduces an error in Q. Is this bounded by |λ'_i| · ‖γ‖² or similar?

2. For the LP in Step 4: should we use a proper LP solver, or is the problem small enough that the current approach (interval arithmetic for 1D, coordinate ascent for nD) suffices after fixing the bugs? The maximum dimension of the LP is m-5 (at most ~11 for F=16 polytopes).

3. For Step 5 (multiplier recovery): the overdetermined system Hβ + Nμ + ηξ = 0 should be consistent if β is correct. The residual of this system is a quality check. If the residual is large, something went wrong. Agree?

4. For m = 5 (unique β from constraints, no H involvement): what should the solver return? Just check β > 0 and compute Q(β)? Or is there a reason to run through the full machinery?

5. Non-generic cases: rank(C) < 5. What causes this geometrically? Parallel normals? Is this common enough to need handling, or can we just return None (no valid orbit)?
</q>

---

## 5. Error analysis

<q>
For solver variant B, what error bounds apply?

Step 1 (constraint solving): standard QR or SVD decomposition. Constraint residual bounded by ε_mach · cond(C) · ‖d‖. For well-conditioned C (generic polytopes), this is O(ε_mach).

Step 2 (projected system): H' = V^T H V introduces V's numerical error. V comes from the constraint solve (Step 1). Perturbation: δH' ≈ V^T H δV + δV^T H V ≈ O(ε_mach · cond(C) · ‖H‖).

Step 3 (eigendecomposition of H'): standard symmetric eigendecomposition. Eigenvalue error O(ε_mach · ‖H'‖). Eigenvector error depends on eigenvalue gaps.

Step 4 (LP): exact (no numerical issue if done in exact arithmetic; in f64, bounded by LP conditioning).

Step 5 (multiplier recovery): residual of overdetermined system. If β is accurate, residual should be O(ε_mach · ‖H‖ · ‖β‖).

Overall Q error: need to derive. The current augmented-system error bound E = (9/2) ‖r‖² / |λ_min| (from lem:q-error-bound) does not directly apply to the projected system. Need a new bound.

Jörn: should we derive the projected-system error bound formally (for the thesis), or is an empirical comparison with the augmented system sufficient?
</q>

---

## 6. Test strategy

<s>
**Unit tests** (per step):
1. Constraint solver: known polytopes (simplex, hypercube, HKO pentagon), verify β₀ satisfies C β₀ = d, verify V spans null(C)
2. Projection: verify H' = V^T H V is symmetric, correct dimension
3. Eigendecomposition: verify reconstruction H' ≈ UΛ'U^T
4. LP feasibility: known feasible/infeasible cases, edge cases (m=5, m=6, large m)
5. Multiplier recovery: verify Hβ + Nμ + ηξ ≈ 0

**Integration tests:**
- Both solver variants agree on Q for all test polytopes (simplex, hypercube, crosspolytope, HKO pentagon, random F=5..10)
- Both solver variants agree on β > 0 verdicts (certified / uncertain / infeasible)
- Rational exact solver agrees with both variants

**Ablation data:**
- Run both variants on a large dataset (existing random-sweep + random-product-sweep polytopes)
- Compare: Q values, constraint residuals, β accuracy, multiplier recovery residuals
- This becomes an experiment (like q-error)

**Regression tests for known tricky cases:**
- Degenerate (4,4) Lagrangian product at θ≈0° (near-singular, eigenvalues ~8.6e-4)
- Any polytope where the current solver returns UNKNOWN (uncertain β)
- m=5 nodes (unique β, no optimization)
</s>

---

## 7. Implementation order

<s>
Code first, thesis follows. Ordering:

1. **Dual-vertex parameterization** — change the library's solver to accept a_i (or equivalently γ, A) instead of (n, h, β). Keep (n, h) → a_i conversion at the call site for now.
2. **Projection-based solver (variant B)** — implement Steps 1-5 above as a new function `solve_kkt_projected()`.
3. **LP-based β > 0 check** — replace find_positive_beta_1d/nd with a single LP approach, used by both variants.
4. **Ablation experiment** — compare variants A and B on existing datasets.
5. **Thesis updates** — once code is stable and tested.

Step 1 can be skipped if it complicates things — the projection solver works fine with (n, h) directly. The dual-vertex switch is a notation improvement, not a correctness requirement.
</s>

---

## 8. Dependencies and interactions

- **Tube algorithm:** The tube algorithm uses the KKT solver indirectly (for closing orbits). The projection variant should work as a drop-in replacement. No blocking dependency — tube can use the current solver.
- **Experiment deduplication:** The 4 duplicated instrumented KKT solvers should be deduplicated BEFORE or AFTER this rework, not during. If before: dedup first, then rework the shared module. If after: rework kkt.rs first, then dedup experiments against the new API.
- **Thesis appendix-numerical:** Needs updating to describe both solver variants and the error analysis. After code is stable.
