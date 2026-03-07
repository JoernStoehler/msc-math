# Sign Conventions — Single Source of Truth

## Jörn's 5 anchor conventions (his exact words)

1. Curves along positive Reeb direction
2. Index order [1 < 2] = time order = positive Reeb flow
3. Q > 0
4. A = 1/(2Q) per HK2017
5. Minimize A

## Additional anchor conventions (chosen, not derived)

6. Lagrangian L = Q + μᵀg → stationarity Hβ + Nμ + ηξ = 0 → M symmetric for eigensolver

## Derived conventions (with derivation and confidence)

### ω₀(n_i, n_j) ≥ 0 for transition F_i → F_j
- **Derives from:** conventions 1, 2
- **Derivation:** R_k = (2/h_k) J₀ n_k. Transition F_i → F_j requires trajectory to move toward F_j, so ⟨R_i, n_j⟩ = (2/h_i) ω₀(n_i, n_j) > 0. Since h_i > 0, this gives ω₀(n_i, n_j) ≥ 0.
- **Confidence:** 95% — derivation is straightforward but UNVERIFIED BY JÖRN
- **Empirical check:** code passes all tests with this convention

### Q = (1/2) β^T H β (positive sign)
- **Derives from:** definition of H and q_from_beta
- **Derivation:** H_{ij} = ω₀(n_{σ(i)}, n_{σ(j)}) for i < j (symmetrized). q_from_beta computes Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) = Σ_{i>j} β_i β_j H_{ij} = (1/2) β^T H β.
- **Confidence:** 100% — verified empirically (simplex: Q = 2.0, (1/2) β^T H β = 2.0, diff = 4.44e-16)
- **Matches Jörn-approved Lemma H-quadratic** in thesis

### Q̃ = Q(β̂) + (r₂ᵀμ̂ + r₃ξ̂) (plus sign in correction)
- **Derives from:** Q = +(1/2) β^T H β in error bound proof
- **Confidence:** 95% — verified empirically on symplectic_triangle_product (correction = 1.86e-10, plus gives err 5.55e-17 vs minus gives err 3.72e-10)
- **Code:** `q_corrected = q_raw + q_correction` in kkt.rs

### Lagrange multipliers μ, ξ: Hβ + Nμ + ηξ = 0
- **Anchor convention 6** (not derived from 1-5)
- L = Q + μᵀg gives stationarity ∇Q + Nμ + ηξ = Hβ + Nμ + ηξ = 0
- This makes M symmetric, enabling eigendecomposition
- **Confidence:** 100% — forced by symmetry requirement

### H|_T positive definite → Q strictly convex → β₀ is local minimum (for fixed S, σ)
- **Derives from:** Q = +(1/2) β^T H β, Hessian = +H
- **Note:** algorithm globally MAXIMIZES Q across (S, σ); this is the local classification within one (S, σ)
- **Confidence:** 99% — standard second-order analysis

## Old conventions (REMOVED)

- Q = -(1/2) β^T H β — WRONG, contradicts Lemma H-quadratic and empirical test
- "negated multipliers" μ = -λ — unnecessary concept, eliminated
- "physical order" — term not used by Jörn, replaced with "positive Reeb direction" / "natural order"
- perm.reverse() / beta.reverse() before/after KKT — all removed
