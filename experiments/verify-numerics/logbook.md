# verify-numerics: Logbook

## Motivation

We discovered ancient math errors that slipped through existing verification. The root problem: our numerical modules lack systematic error analysis. There's a gap between what each module computes in exact arithmetic and what the f64 implementation actually delivers — and that gap is either unanalyzed or analyzed with known-loose bounds.

This experiment develops the right algorithms (not assumes the current ones are right), formalizes error bounds as lemmas, proves them, and empirically verifies and probes to detect falsehoods and room for improvement.

**This blocks other experiment development.** All other experiments rely on this machinery. Verify it first.

## The Problem

**Find the maximum of ½β^TQβ subject to Cβ=d, β≥0.**

Given:
- Q: symmetric matrix (may be indefinite)
- C: constraint matrix
- d: constraint RHS
- β≥0: componentwise positivity

The current codebase has solvers for this in `crates/src/kkt/`. Read them. They are one candidate approach, not the answer.

## Generic Numerics Framework

This applies to every numerical subroutine in this experiment:

- **Certify propositions** as TRUE / FALSE / INDETERMINATE.
- **Error bounds for values** — proven, not empirical thresholds.
- **INDETERMINATE falls back to rational arithmetic** — but lazily. Short-circuit: if a proposition "C = A ∧ B" needs a certificate and B is INDETERMINATE, but A is FALSE, then C is FALSE. Don't invoke the rational fallback until you actually need it.

## What to do

For the main problem and each sub-subroutine that emerges:

1. **Math spec** — What does this compute in exact arithmetic?
2. **Algorithm design** — What algorithms exist for this problem? What's the right one? Explore alternatives, don't just validate the existing code.
3. **Error analysis** — Formalize f64 error bounds as lemmas. Prove them.
4. **Edge cases** — Discover and catalog. Handle explicitly instead of praying they don't occur.
5. **Empirical verification** — Generate inputs, compare f64 against exact (rational) ground truth, validate that proven bounds hold. Find where they're tight vs vacuous.
6. **Findings** — What's correct, what's broken, what needs Jörn.

Sub-subroutines each get their own `run_*.rs` file and the same treatment.

## Context from the codebase

The current solvers live in `crates/src/kkt/`. Key files:
- `saddle_point_solver.rs` — eigendecomposition-based KKT solver (main production solver)
- `projection_solver.rs` — alternative QP solver via projection onto constraint null space
- `rational_solver.rs` — exact rational arithmetic solver (ground truth)
- `constraint_solver.rs` — SVD-based Cx=d solver (subroutine)
- `beta_feasibility.rs` — Chebyshev center LP for max-min-component (subroutine)
- `qp_assembly.rs` — builds QP matrices from polytope data (two formulations, equivalence unproven)
- `mod.rs` — types, constants, verdict classification
- `math.tex` — existing correctness proofs (some unverified, some known loose)

The problem arises in the context of computing the EHZ capacity of 4D convex polytopes. β are dwell-time coefficients, Q encodes symplectic action, C enforces closure + normalization. But the QP solver itself is context-free — it takes matrices and returns solutions.

Important domain context: β_k=0 boundary cases are handled by the caller running this same subroutine with a shorter permutation σ (fewer variables). So the interior case (β>0 strictly) and boundary cases are naturally separated.

## What Jörn decided

- One experiment, sub-subroutines as run_*.rs files.
- Don't prescribe algorithms or edge cases — discover them.
- Test subsystems independently to avoid bug cancellation.
- Include empirical math validation (not just solver numerics).
- If special structure of C matters (from the domain), read the codebase and ask Jörn whether narrowing scope is better than the general problem.

## Known issues from code-math audit (2026-03-28)

Cross-reference audit (`handoffs/cross-reference-audit.md`) found 3 mismatches, all in `saddle_point_solver.rs`:

- **M1 (critical):** `lem:q-error-bound` uses |λ_min| over ALL eigenvalues, code uses |λ_min| of RETAINED eigenvalues (after threshold τ=10⁻³). Bound is too loose — panics on basic polytopes. Already tracked in TASKS.md.
- **M2 (high):** Q is computed from β₀ (pseudoinverse solution) but the returned `result.beta` is β_final (LP-shifted). These don't correspond — the structural contract "result.beta and result.q come from the same solution" is broken. Incomplete TODO at line 479.
- **M3 (medium):** Code comment at lines 534-544 oversimplifies the lem:q-error-bound proof. The 9/2 constant is correct but the explanation ("removes the ||H||/|λ_min|² term") is misleading.

Everything else in the codebase (~160 references) checked out OK. All problems are in the error bound machinery.

## Session 2026-03-30: Q Accuracy Measurement

### Approach

Measure actual Q error of both f64 solvers against exact rational arithmetic on abstract (H, C, d) matrices.

**Design choice (Jörn):** iterate on abstract KKT problems, not polytopes. Advantages: controlled inputs, explicit assumptions, no coupling to other code.

**Matrix families generated:** identity, random dense symmetric, EHZ-like, near-singular H, singular H, indefinite H, small m=6, large m=16, feasible-by-construction. Total: 293 problems. Low feasibility rate (16/293) — most random matrices have no β > 0 solution. The "feasible by construction" family is the most productive (7/50 feasible). Note: the exact solver uses the same f64→BigRational conversion, so some problems that are feasible in exact f64 arithmetic may be infeasible over Q.

### Finding: Sign Error in projection_solver.rs

**The library's projection solver (`crates/src/kkt/projection_solver.rs`) has a sign error that causes Q errors up to 0.57 (57% of Q).**

The stationarity condition for the reduced QP is H'α + g = 0 where g = V^T H β₀ and H' = V^T H V. The solution is α₀ = -(H')⁺g. The code at lines 108–116 computes α₀ = +(H')⁺g (no negation). The comment says "Solve H' alpha = b'" when the correct equation is "Solve H' alpha = -b'."

Measured Q errors (16 exact-feasible problems across all families):

| Solver | Median Q error | Max Q error | n |
|--------|---------------|-------------|---|
| Saddle-point | 4.16e-17 | 1.67e-16 | 12 |
| Projection (library) | 4.50e-02 | 5.69e-01 | 11 |
| Projection (sign fix) | 1.67e-16 | 2.83e-15 | 16 |

The corrected projection solver matches machine epsilon. The library version is off by orders of magnitude.

**Why existing tests didn't catch this:** The projection solver tests use H = I or simple block structures where b_prime = V^T H β₀ = V^T β₀ = 0 (because β₀ is the min-norm SVD solution, orthogonal to ker(C) = range(V)). When b_prime = 0, the sign error is invisible.

**Impact on production code:** None currently. The production pipeline uses the saddle-point solver. The projection solver is an alternative that's never called in the capacity algorithms.

**Action needed:** Fix the sign in `crates/src/kkt/projection_solver.rs` line ~113: change `pi.dot(&b_prime) / eigenvalues[i]` to `-pi.dot(&b_prime) / eigenvalues[i]`.

### Finding: Saddle-point Solver Accuracy

The saddle-point solver Q errors are at machine epsilon (max 1.67e-16) across all test families. The corrected Q value (Q_raw + residual correction) is highly accurate.

No panics occurred on these abstract matrix problems (unlike the polytope-specific panic from M1). This is consistent with M1: the panic is triggered by specific eigenvalue structure from degenerate polytope orbits, not by general matrices.

### How to run

```bash
cd experiments/ && cargo build --release --bin verify_numerics_q_accuracy
cargo run --release --bin verify_numerics_q_accuracy
# Output: verify-numerics/q_accuracy.jsonl (3203 rows)
```

### Finding: κ(C) Predicts E2E Error, Not κ(H) or |λ_min(M)|

Added two new matrix families to stress tiny |λ_min(M)|:
- **tiny_lam_min**: H with controlled small eigenvalues (10⁻¹ to 10⁻¹⁴), feasible by construction. 48/500 feasible.
- **ill_cond_c**: C with near-dependent rows (κ(C) from 10¹ to 10¹²), well-conditioned H. 192/200 feasible.

Results (3203 problems, 364 exact-feasible):

| Family | n feasible | SP max error | Proj max error | SP panics |
|--------|-----------|-------------|---------------|-----------|
| tiny_lam_min | 48 | 4.16e-17 | 3.14e-15 | 0 |
| ill_cond_c | 192 | **1.03e-01** | **1.03e-01** | **70** |
| near_singular_h | 27 | 8.33e-17 | 2.36e-16 | 0 |
| feasible_constructed | 52 | 6.66e-16 | 1.33e-15 | 0 |

**Tiny eigenvalues of H cause zero extra error.** Both solvers handle them fine to machine epsilon.

**Ill-conditioned C causes large e2e errors and panics.** When κ(C) > 10⁸, Q errors reach 10⁻³ to 10⁻¹. The saddle-point solver panics on 70/192 of these cases (the existing bound E > 1e-6 threshold trips). Error grows roughly with κ(C):

| log₁₀(κ(C)) | n | max error |
|-------------|---|-----------|
| 1-3 | 39 | 3e-13 |
| 4-6 | 51 | 7e-7 |
| 7-9 | 46 | 3e-3 |
| 10-12 | 28 | **1e-1** |

The relationship is not simply ε_mach · κ(C) · |Q| — the ratios vary by 5 orders of magnitude, suggesting other factors matter (C structure, null-space angle, etc.).

**The current error bound E = 4.5||r||²/|λ_min(M)| focuses on the wrong quantity.** |λ_min(M)| can be tiny from H's small eigenvalues (no real error) or from C's ill-conditioning (real error). The bound doesn't distinguish these. A better bound would involve κ(C) or σ_min(C).

### Insight: error bound vs e2e error mismatch

The existing bound E ~ 1e-28 for well-conditioned problems is a bound on |Q(β₀) - Q̃| (solver internal error, measured via KKT residual). The measured e2e error |Q_f64 - Q_exact| ~ 1e-16 for those same problems includes matrix-entry rounding that the bound doesn't account for. Both are valid but measure different things. For the certification framework, we need the e2e bound.

### Open questions

1. The experiment imports library solvers. Should copy/own them for self-containment and iterability (Jörn).
2. The error scaling with κ(C) is noisy. What additional quantities predict the error? σ_min(C), the angle between ker(C) and H's eigenspaces?
3. Is κ(C) ever large in the actual EHZ pipeline? If C always has σ_min > some threshold, the ill-conditioned regime may be irrelevant.

## Session 2026-03-30: Error Bound Iteration

### Conjecture evolution

**Candidate A (simple):** `|Q_err| ≤ C · ||r|| · κ(C)`. Tested on 3203 problems across 11 families: zero violations, max ratio 0.063 (q_accuracy.jsonl row: ill_cond_c with κ(C)=1.12e12). Log-log correlation between err and ||r||·κ(C): 0.90.

**Stress test:** Added family 12 "large_h_ill_c" — large ||H|| (eigenvalues 10–100) combined with ill-conditioned C. Also removed panics from experiment solver to measure actual Q error for all cases.

**Candidate A breaks.** large_h_ill_c produces max ratio **0.921** (92% of the bound). Case: m=7, ||H||=98, ||β||=0.55, κ(C)=1.17e9, err=11.4. With ||H||=200 the bound would be violated.

**Candidate B (perturbation):** `|Q_err| ≤ C · ||H|| · ||β|| · ||r|| / σ_min(C)`. Max ratio **0.186**, zero violations across 269 SP-feasible cases from 12 families (3703 problems total). This is the correct functional form.

**Why Candidate A appeared to work:** the ill_cond_c family uses well-conditioned H (||H|| ~ 4), so the ||H||·||β||/σ_max(C) factor is ≈ 0.3, absorbed by the constant. When ||H|| increases to 98, the factor jumps to 7.7 and the bound nearly breaks.

### Proof ([lem:q-error-first-order] in math.tex)

**Theorem.** At a KKT point β* with multiplier λ*:

|Q(β̃) − Q(β*)| ≤ (||H||·||β*||/σ_min(C))·||r|| + ½||H||·||δβ||²

**Proof sketch:**
1. Taylor: Q_err = (Hβ*)ᵀδβ + ½ δβᵀHδβ
2. Stationarity: Hβ* = −Cᵀλ*, so (Hβ*)ᵀδβ = −λ*ᵀCδβ = −λ*ᵀr_λ
3. Bound: ||λ*|| ≤ ||H||·||β*||/σ_min(C) from ||Cᵀλ*|| = ||Hβ*||
4. And ||r_λ|| ≤ ||r|| (subvector of total residual)
5. Combine with triangle inequality. □

**Key insight:** at the critical point, dQ/dβ = Hβ* = −Cᵀλ* is in range(Cᵀ). So null-space perturbations of β don't affect Q at first order — only the constraint residual r_λ matters.

### Finding: Q correction is second-order ([lem:q-correction-second-order])

The solver already computes Q_corrected = Q_raw + λ̃ᵀr_λ. This exactly cancels the first-order error term:

Q_corrected − Q_exact = δλᵀr_λ + ½ δβᵀHδβ

Both terms are products of error quantities (second-order). This explains why the measured Q errors are so small for well-conditioned problems (the first-order correction works).

### Decomposition validation

Step-by-step empirical validation against 269 feasible cases:

| Bound | Max ratio | Violations |
|-------|-----------|------------|
| err ≤ ||H||·||β||·||δβ|| (Taylor) | 0.207 | 0 |
| err ≤ ||H||·||β||·||r||/σ_min (Theorem) | 0.186 | 0 |
| err ≤ ||r||·κ(C) (simple, no ||H||) | 0.921 | 0 (barely) |
| ||δβ|| ≤ ||r||/σ_min(C) (naive) | 4.631 | 109 |
| err ≤ ||H||·(||r||/σ_min)² (quadratic) | 8.3e13 | 153 |

**Rejected bounds:** ||δβ|| ≤ ||r||/σ_min fails because δβ can have a large null(C) component that doesn't contribute to r_λ. The quadratic bound fails for well-conditioned problems because the Q error floor is set by floating-point rounding in the Q_raw computation, not by β perturbation.

### Runtime error bound

At runtime, compute: E = ||H|| · ||β̃|| · ||r|| / σ_min(C)

All quantities from the solver: ||H|| from eigendecomposition, ||β̃|| from the solution, ||r|| from KKT residual, σ_min(C) from SVD.

### Open questions

1. Can the constant 0.186 be improved? The proof gives 1 (the Taylor first-order coefficient). The gap may be structural (always <1/5) or an artifact of our test distribution.
2. For the EHZ pipeline: what are typical values of ||H||, ||β||, σ_min(C)? If ||H|| and 1/σ_min are bounded, the bound simplifies.
3. ||δβ|| in the second-order term: can we bound it at runtime without knowing β*? The constraint residual gives ||δβ_range|| ≤ ||r_λ||/σ_min(C) but the null-space component is harder.
4. The solver's Q correction already cancels the first-order error. Can the runtime bound be sharpened using the corrected Q?

## Session 2026-03-30: Structural Theorem and Chain Validation

### Finding: Q correction is exact in theory

**Theorem.** For the pseudoinverse solution x̃ = M⁺b of the KKT system Mx = b:

If x* ∈ col(M), then δx = x̃ − x* ∈ col(M), and r = Mδx ⊥ col(M) implies δx^T r = 0, hence **δx^T M δx = 0**.

Block expansion: δβ^T H δβ + 2 r_λ^T δλ = 0, so ½ δβ^T H δβ = −r_λ^T δλ.

**Consequence for Q_raw_err:** The Taylor expansion Q(β̃) − Q(β*) = (Hβ*)^T δβ + ½ δβ^T Hδβ satisfies first_order = 2 × second_order exactly (opposite signs), so Q_raw_err = ½ δβ^T Hδβ.

**Consequence for Q_corr_err:** Q_corr_err = δλ^T r_λ + ½ δβ^T Hδβ = δλ^T r_λ − r_λ^T δλ = 0. The correction is *exact* in exact arithmetic.

**Verified empirically:** first_order/second_order = 2.0000 to machine precision in all 95 rank-deficient feasible cases. For full-rank M, both errors are at machine epsilon.

**When the theorem fails:** x* ∉ col(M) when M is singular and x* has a null-space component. All high-error cases (err > 1e-2) are rank-deficient. The f64 eigendecomposition thresholds eigenvalues, changing the effective null space. If the exact x* has components in directions that get thresholded, the pseudoinverse "sees" a different subspace.

### Chain bound table

Validated on 477 SP-feasible cases across 15 matrix families (4303 problems total).

| Bound | Status | Max ratio | Note |
|-------|--------|-----------|------|
| ‖r_λ‖ ≤ ‖r‖ | VALID | 1.00 | Sub-vector, always tight |
| ‖λ*‖ ≤ ‖H‖·‖β*‖/σ_min(C) | VALID | 0.89 | Tight, proven [lem:q-error-first-order] |
| \|Q_corr_err\| ≤ E₁ | VALID | 0.196 | Runtime bound, moderate tightness |
| \|Q_raw_err\| ≤ E₁ | VALID | 0.33 | Also covers uncorrected Q |
| \|λ*^T r_λ\| ≤ E₁ | VALID | 0.37 | First-order term |
| \|½δβ^T Hδβ\| ≤ ½‖H‖·‖δβ‖² | VALID | 1.00 | Spectral norm, tight |
| ‖δβ‖ ≤ ‖r‖/σ_min(C) | **INVALID** | 4.63 | δβ has null(C) component |
| 4.5‖r‖²/\|λ_min\| as e2e bound | **INVALID** | 4.7e12 | Bounds wrong quantity |

### Stress test results

| Family | n_feasible | Max err | What it tests |
|--------|-----------|---------|---------------|
| double_singular | 8 | 2.0e-1 | Both H and C near-singular |
| clustered_h_eig | 7 | 9.7e-17 | Near-degenerate H eigenspaces |
| clustered_m_eig | 200 | 1.0e-2 | Near-degenerate M eigenspaces |

**Clustered/degenerate H eigenspaces cause ZERO extra error.** Q = ½β^T Hβ depends on H as a matrix, not on its eigenvectors. The eigenvector instability is irrelevant.

**Double-singular adds no error beyond κ(C).** The H near-singularity is harmless; only C near-singularity matters.

### How to run

```bash
cd experiments/ && cargo build --release --bin verify_numerics_q_accuracy
cargo run --release --bin verify_numerics_q_accuracy
# Output: verify-numerics/q_accuracy.jsonl (4303 rows, 15 families)
```

## Status

In progress. Key findings:
1. **Projection solver sign error** (confirmed, one-line fix needed in library)
2. **Proven Q error bound** [lem:q-error-first-order]: |Q_err| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C) + ½‖H‖·‖δβ‖²
3. **Q correction is exact in theory** (δx^T M δx = 0 for pseudoinverse)
4. **E₁ runtime bound validated**: zero violations across 477 cases, max ratio 0.196
5. **Chain table**: 6 valid bounds, 2 invalid, all intermediate quantities measured

Next: close gap in math.tex (eigendecomposition backward stability → ‖r‖ bound), check EHZ pipeline inputs, formalize the structural theorem.
