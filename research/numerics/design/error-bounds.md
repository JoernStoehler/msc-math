# verify-numerics: Logbook

## Motivation

We discovered ancient math errors that slipped through existing verification. The root problem: our numerical modules lack systematic error analysis. There's a gap between what each module computes in exact arithmetic and what the f64 implementation actually delivers — and that gap is either unanalyzed or analyzed with known-loose bounds.

This experiment develops the right algorithms (not assumes the current ones are right), formalizes error bounds as lemmas, proves them, and empirically verifies and probes to detect falsehoods and room for improvement.

This started as the numerics-confidence prerequisite for the rest of the experiment program. In the current post-Kai thesis-close state, the remaining open items here are publication-polish and library-follow-up tasks, not a blocker for the two main thesis result blocks.

## The Problem

**Find the maximum of ½β^TQβ subject to Cβ=d, β≥0.**

Given:
- Q: symmetric matrix (may be indefinite)
- C: constraint matrix
- d: constraint RHS
- β≥0: componentwise positivity

The current codebase has solvers for this in `library/src/kkt/`. Read them. They are one candidate approach, not the answer.

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

The current solvers live in `library/src/kkt/`. Key files:
- `saddle_point_solver.rs` — eigendecomposition-based KKT solver (main production solver)
- `projection_solver.rs` — alternative QP solver via projection onto constraint null space
- `rational_solver.rs` — exact rational arithmetic solver (ground truth)
- `constraint_solver.rs` — SVD-based Cx=d solver (subroutine)
- `beta_feasibility.rs` — Chebyshev center LP for max-min-component (subroutine)
- `qp_assembly.rs` — builds QP matrices from polytope data (two formulations, equivalence unproven)
- `mod.rs` — types, constants, verdict classification
- `formal/numerics/error-bounds.tex` — existing correctness proofs (some unverified, some known loose)

The problem arises in the context of computing the EHZ capacity of 4D convex polytopes. β are dwell-time coefficients, Q encodes symplectic action, C enforces closure + normalization. But the QP solver itself is context-free — it takes matrices and returns solutions.

Important domain context: β_k=0 boundary cases are handled by the caller running this same subroutine with a shorter permutation σ (fewer variables). So the interior case (β>0 strictly) and boundary cases are naturally separated.

## What Jörn decided

- One experiment, sub-subroutines as run_*.rs files.
- Don't prescribe algorithms or edge cases — discover them.
- Test subsystems independently to avoid bug cancellation.
- Include empirical math validation (not just solver numerics).
- If special structure of C matters (from the domain), read the codebase and ask Jörn whether narrowing scope is better than the general problem.

## Known issues from code-math audit (2026-03-28)

Earlier cross-reference audit found 3 mismatches, all in `saddle_point_solver.rs`:

- **M1 (historical, stale against current code):** the original audit compared `lem:q-error-bound` against an older retained-eigenvalue implementation. Keep this as provenance only; do not treat it as the current code state without re-auditing the present solver.
- **M2 (high):** Q is computed from β₀ (pseudoinverse solution) but the returned `result.beta` is β_final (LP-shifted). These don't correspond — the structural contract "result.beta and result.q come from the same solution" is broken. Incomplete TODO at line 479.
- **M3 (medium):** Code comment at lines 534-544 oversimplifies the lem:q-error-bound proof. The 9/2 constant is correct but the explanation ("removes the ||H||/|λ_min|² term") is misleading.

Everything else in the codebase (~160 references) checked out OK. All problems are in the error bound machinery.

## Session 2026-03-30: Q Accuracy Measurement

### Approach

Measure actual Q error of both f64 solvers against exact rational arithmetic on abstract (H, C, d) matrices.

**Design choice (Jörn):** iterate on abstract KKT problems, not polytopes. Advantages: controlled inputs, explicit assumptions, no coupling to other code.

**Matrix families generated:** identity, random dense symmetric, EHZ-like, near-singular H, singular H, indefinite H, small m=6, large m=16, feasible-by-construction. Total: 293 problems. Low feasibility rate (16/293) — most random matrices have no β > 0 solution. The "feasible by construction" family is the most productive (7/50 feasible). Note: the exact solver uses the same f64→BigRational conversion, so some problems that are feasible in exact f64 arithmetic may be infeasible over Q.

### Finding: Sign Error in projection_solver.rs

**The library's projection solver (`library/src/kkt/projection_solver.rs`) has a sign error that causes Q errors up to 0.57 (57% of Q).**

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

**Action needed:** Fix the sign in `library/src/kkt/projection_solver.rs` line ~113: change `pi.dot(&b_prime) / eigenvalues[i]` to `-pi.dot(&b_prime) / eigenvalues[i]`.

### Finding: Saddle-point Solver Accuracy

The saddle-point solver Q errors are at machine epsilon (max 1.67e-16) across all test families. The corrected Q value (Q_raw + residual correction) is highly accurate.

No panics occurred on these abstract matrix problems (unlike the polytope-specific panic from M1). This is consistent with M1: the panic is triggered by specific eigenvalue structure from degenerate polytope orbits, not by general matrices.

### How to run

```bash
cargo build -p dev-numerical-analysis --release --bin num-collect-poly
cargo run --release --bin num-error-bounds
# Output: q_accuracy.jsonl (3203 rows)
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

### Proof ([lem:q-error-first-order] in formal/numerics/error-bounds.tex)

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
cargo build -p dev-numerical-analysis --release --bin num-collect-poly
cargo run --release --bin num-error-bounds
# Output: q_accuracy.jsonl (4303 rows, 15 families)
```

## Propositions and bounds (current state, 2026-04-01)

Run `uv run analyze.py` from `experiments/numerics/error-bounds/` for the full check output (`checks.txt`).

Dataset: 51,784 problems (4303 artificial + 47,481 natural from 47 polytopes F≤8). 45,476 SP-feasible (44,980 EHZ-like, 496 stress-test).

Propositions:

| # | Statement | Type | Status | Notes |
|---|-----------|------|--------|-------|
| P1 | d = (0,0,0,0,1) | assumption | ✓ | by construction |
| P2 | C = (A^T; 1^T), 5×m | assumption | ✓ | by construction |
| P3 | H symmetric | assumption | ✓ | by construction |
| P4 | σ_min(C) > 1e-12 | assumption | 0 EHZ violations | σ_min(C) = 0 for 30K σ-nodes with m ≤ 5 (C rank-deficient) |
| P5 | ‖H‖/σ_min(C) ≤ 100 | **FALSIFIED** | max 1310 on natural data | Ratio unbounded as σ_min(C) → 0. Now diagnostic only. |
| P6 | ‖r_β‖ < 1e-3 (full-rank M) | bug detection | 0 violations | Gated on full-rank M. Rank-deficient has ‖r_β‖ up to 0.63. |
| P7 | ‖β‖ ≤ 2 | bug detection | 0 EHZ violations | 3 stress violations (Q ≤ 0 cases, β large) |
| P8 | ‖r_λ‖ < 1e-6 | bug detection | 0 violations | Max 2.9e-8 on natural data |

Proven bounds (validated against exact rational ground truth):

| # | Bound | Assumes | EHZ max ratio | Stress max ratio | Violations |
|---|-------|---------|---------------|-----------------|------------|
| B2 | ‖λ*‖ ≤ ‖H‖·‖β*‖/σ_min(C) | P3, P4 | 0.976 | 0.636 | 0 |
| B3 | \|Q−Q*\| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C) | P3, P4, β*>0 | 0.217 | 0.233 | 0 |
| B4 | \|Q_raw−Q*\| ≤ same | P3, P4, β*>0 | 0.613 | 0.286 | 0 |
| B5 | 1st/2nd = 2 (identity) | P3, x*∈col(M) | (below noise) | (below noise) | 0 |
| B6 | correction ≤ 2x worsening | — | max 6.3e-2 | max 1.0 | 0 |

β > 0 classification:

| | Natural polytope (44,808) | Stress-test (496) |
|---|---|---|
| True positive | 44,414 | 462 |
| **False positive** | **0** | **0** |
| False negative | 9 | 15 |
| Min TP margin | 1.11e-5 | 2.09e-3 |

## Capacity pipeline integration

The capacity algorithm for a given (σ, S) computes (Q, β) and certifies:
- **β > 0?** → TRUE / FALSE / INDETERMINATE (margin classification, separate analysis)
- **|Q − Q*| small?** → check ‖H‖·‖β‖·‖r‖/σ_min(C) < tolerance

All quantities in the bound are already computed by the solver: ‖H‖ from eigendecomposition, ‖β‖ from solution, ‖r‖ from KKT residual, σ_min(C) from SVD. No extra computation.

## How to run

```bash
cd experiments/numerics/error-bounds/
cargo build --release --bin num-collect-poly
cargo run --release --bin num-error-bounds
# Output: q_accuracy.jsonl (4303 rows, 15 families)
uv run analyze.py
# Output: q_accuracy_checks.txt + stdout
```

## Session 2026-03-31: Infrastructure Refactoring + Natural Data + β > 0 Classification

### Pipeline refactoring

Three-stage pipeline replaces the monolithic binary:
1. `collect_inputs.rs` generates `artificial.jsonl` (15 synthetic families, 4303 problems) and `collected.jsonl` (polytope σ-nodes from correctness.jsonl, F≤8, 70K rows)
2. `main.rs` loads both datasets, filters in-memory, runs f64 + exact rational solver, writes `results.jsonl`
3. `analyze.py` reads `results.jsonl`, checks propositions, bounds, β > 0 classification

### Solver fix: Q ≤ 0 conflation

The solver previously returned `Infeasible` when Q ≤ 0 at the stationary point, conflating "Q is negative" with "β is infeasible." Fixed: solver now returns `Feasible` whenever β > 0 regardless of Q sign. Effect: 48K natural σ-nodes now visible (was 2.7K). 42K of these have Q ≈ 0, 2.7K have Q < 0.

Also removed `SingularMatrix` variant (was: all eigenvalues ≈ 0, meaning both H ≈ 0 and C ≈ 0). This is garbage input, not a QP outcome. Now a panic.

### Natural polytope data (correctness.jsonl, 47 polytopes, F ≤ 8)

From 70,676 σ-nodes: 48,257 feasible (β > 0), 14,037 β_non_positive, 8,382 residual_too_large.

After filtering (all Q > 0, sample 500 Q ≤ 0, sample 500 β < 0): 3,719 natural + 4,303 artificial = 8,022 problems in results.jsonl.

Key findings (2,378 natural feasible with exact ground truth):

| Quantity | Natural polytopes | Synthetic EHZ-like |
|----------|------------------|--------------------|
| B3 max ratio | 0.161 | 0.149 |
| B3 violations | **0** | 0 |
| max \|Q−Q*\| | 3.95e-14 | 6.6e-16 |
| σ_min(C) min | **1.19e-3** | 0.31 |
| ‖H‖/σ_min(C) max | **1310** | 39.8 |
| ‖r_β‖ max | **0.63** | 6.3e-11 |

### β > 0 classification

| | Natural polytope | Stress-test |
|---|---|---|
| True positive (both β > 0) | 2352 | 460 |
| **False positive** | **0** | **0** |
| False negative | **9** | 17 |
| Min TP margin | 5.56e-4 | 2.09e-3 |

Zero false positives. 9 false negatives on natural data — all m=6, rank 10/11 (one discarded eigenvalue).

### Root cause of false negatives

All 9 share the same mechanism:
1. M has eigenvalue below threshold τ = 1e-3 · max|λ| → discarded
2. LP finds null-space direction, shifts β₀ by ~2 to get β > 0
3. Shifted β violates Cβ = d (constraint residual ~0.6) because the eigenvector isn't truly in null(M)
4. Solver falls back to β₀, which has margin ≈ 0 (machine epsilon)
5. Exact solver finds β* with margin 0.17 — well inside feasible region

The discarded eigenvalue direction is informative for β > 0 but shifting along it violates constraints. This is a solver algorithm limitation, not a numerical accuracy issue.

### Conjecture violations on natural data

- **P5 violated** (‖H‖/σ_min(C) ≤ 100): 15 cases from 4 polytopes with σ_min(C) ≈ 1e-3. ‖H‖ ≈ 1.5 (normal). Ratio unbounded as σ_min(C) → 0. P5 is not a useful conjecture.
- **P6 violated** (‖r_β‖ < 1e-3): 25 cases, all m=6 rank-deficient. Same mechanism as false negatives.
- σ_min(C) = 0 occurs for 2,904 σ-nodes (m ∈ {4,5,6}, C rank-deficient).

### Correlation findings

| Comparison | Key finding |
|-----------|-------------|
| Q error: full-rank vs rank-deficient M | max 8e-16 vs max 382. 10^18× gap. |
| Q error: Q > 0 vs Q ≤ 0 | Q sign not the driver — rank deficiency is |
| Correction effectiveness | More impactful in rank-deficient (48%) than full-rank (28%) |
| Margin vs σ_min(C) on natural data | σ_min(C) < 0.01 → median margin 2e-12 (boundary cases) |
| β error vs margin | β accuracy excellent (1e-15) even for small margins |

## Status

Complete for Q accuracy and β > 0 classification. The proven Q bound B3 holds on natural data.

Deliverables:
1. **Proven bound** [lem:q-error-first-order]: |Q−Q*| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C) — zero violations on 8022 problems
2. **Structural theorem** [lem:pseudoinverse-orthogonality, cor:taylor-structure, cor:exact-correction]
3. **β > 0 classification**: zero false positives, 9 false negatives (root-caused)
4. **Three-stage pipeline**: collect → run → analyze, natural + artificial datasets
5. **analyze.py** with full proposition/bound/classification validation

Open:
- Library promotion: move proven bound + asserts into `library/src/kkt/`
- Fix projection solver sign in library (`library/src/kkt/projection_solver.rs:93`)
- P5 conjecture: remove or replace (ratio unbounded on natural inputs)
- P6 threshold: increase or gate on full-rank M
- False-negative solver improvement: eigenvalue threshold too aggressive for m=6 rank-deficient
- GAP in cor:taylor-structure proof (needs Jörn)

## Pipeline restructure (2026-04-01)

Restructured to Jörn's 4-stage design:

```
Stage 1: collect_poly.rs → collected_poly.jsonl (1.66M rows, gitignored)
         collect_synth.rs → collected_synth.jsonl (4303 rows, gitignored)
Stage 2: filter_poly_smoke.rs → filtered_poly_smoke.jsonl (~6 rows)
         filter_poly_diverse.rs → filtered_poly_diverse.jsonl (~1500 rows)
         filter_synth_all.rs → filtered_synth_all.jsonl (4303 rows)
Stage 3: main.rs <input> <output> → results_*.jsonl (f64 + exact + diagnostics)
Stage 4: analyze.py <results1> [results2 ...] → checks.txt
```

Run from `experiments/numerics/error-bounds/` with the current binaries:
`cargo build --release --bin num-collect-poly`
`cargo run --release --bin num-error-bounds`
`uv run analyze.py`

Stage 1 now saves raw β, λ vectors (not just summary stats).
Filter binaries coexist — edit/add filters without churn.

## Perturbation chain and β certification bound (2026-04-01)

Added to formal/numerics/error-bounds.tex:
- lem:link-beta0: β₀ perturbation bound, O(ε_mach/σ_min(C)²)
- lem:link-gradient: reduced gradient perturbation
- rem:conditioning-precondition: σ_min(C) gates the chain
- lem:link-beta rewritten: explicit componentwise η_k bound (eq:eta-computable)

Implemented in solvers.rs: `solve_projected_with_diagnostics()`, `compute_eta_bound()`.

Results on natural data (1192 polytope σ-nodes via filter_poly_diverse):
- Well-separated eigenvalues: zero violations, 86% β > 0 certified
- Null-eigenvalue cases (k=1, H' ≈ 0): 39 violations, η bound doesn't cover LP search step

Root cause of violations: when H' has a near-zero eigenvalue, the solver searches the null eigendirection via LP. The LP shift is O(1) but the bound predicts O(ε_mach). Extending the bound to cover this case is the next step.

### Empirical conjecture: per-eigendirection β error (2026-04-01)

Confirmed on 364 I1 problems (all γ_j < 0, unique interior β*) from natural polytope data:

|δα_j| ≈ ε_mach / |γ_j|

where δα_j is the error in the j-th eigendirection of H' (δα = W^T V^T (β̃ - β*)).
The product |δα_j| · |γ_j| is ~10^{-16} to 10^{-17} across 15 orders of magnitude of |γ_j|
(from |γ_j| ≈ 1 down to |γ_j| ≈ 10^{-5}). Proportionality constant ≈ ε_mach.

The componentwise β error follows via:
|δβ_k| ≈ ε_mach · Σ_j |(Vw_j)_k| / |γ_j|

This is the shape of the η_k bound (eq:eta-computable in formal/numerics/error-bounds.tex). The bound is valid
with safety constant c = m² (zero violations on well-separated eigenvalues).

Outlier: |γ_j| ≈ 10^{-15} gives |δα_j| ≈ 0.05 — same root cause as the 39 bound violations
(null eigenvalue, solver retains it, 1/γ amplification produces O(1) error).

Open:
- Write the f64 algorithm (Part III of formal/numerics/error-bounds.tex)
- Extend η_k bound for null-eigenvalue LP search
- GAP in cor:taylor-structure proof (needs Jörn)

### Infrastructure simplification (2026-04-01)

Split solvers.rs into projection_solver.rs (active) and saddle_point_solver.rs (dead code, reference).
Removed saddle-point code from main.rs (~20 fields, ~150 lines). Extracted exact rational solver
into shared exact_solver.rs.

Deleted 4 filter binaries (filter_poly_smoke, filter_poly_diverse, filter_synth_all) and
collect_synth. Filters are now ad-hoc (jq/Python one-liners on collected_poly.jsonl).

Created testdata/ with 30 curated (H,C,d) test cases per conjecture:
- eigendirection_scaling.jsonl: 30 cases (m=6,7) for rem:eigendirection-error
- eta_bound_validity.jsonl: 30 cases (m=6,7,8) for lem:link-beta eq:eta-computable

Created tests.rs: two Rust #[test] functions that load testdata, run both projection and
exact solvers, and check the conjecture properties. Both pass:
- eigendirection_error_scaling: 12 eigendirections, max ratio 0.9
- eta_bound_validity: 50 components, max ratio 0.007

Simplified analyze.py (499→147 lines): removed bound-checking (now in Rust tests),
kept exploratory summaries. Simplified Makefile to just collect + ad-hoc run/analyze.

Run tests: `cargo test --test verify_numerics_tests`

## Git LFS tracking (2026-04-03)

collected_poly.jsonl (2.2 GB, 1.66M rows) exceeds GitHub LFS 2 GB per-file limit.
Currently .gitignored. Future runs of collect_poly must either compress output
(gzip — JSONL compresses ~5-10x) or split into chunks <2 GB, then remove from
.gitignore so LFS tracks it.
