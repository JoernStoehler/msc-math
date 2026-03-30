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
# Output: verify-numerics/q_accuracy.jsonl (293 rows)
```

### Open questions

1. The current test matrix families have low exact-feasibility rates. Better generators (e.g., construct β > 0 first, then derive compatible H) would give more data points.
2. The experiment currently imports library solvers as "code under test." A future iteration should copy the solver code into the experiment for self-containment (Jörn).
3. The M1/M2/M3 mismatches are about the saddle-point solver's error bound machinery, not Q accuracy. The Q values are accurate — the *bound on the error* is what's broken. This experiment measures the actual error, which is the input for conjecturing a better bound.
4. No EHZ-like or indefinite problems produced exact-feasible instances. These families need better generators.

## Status

In progress. First measurement done. Sign error in projection solver found and verified. Next: improve feasibility rates, start error bound analysis on saddle-point solver.
