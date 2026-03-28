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

## Status

Not started.
