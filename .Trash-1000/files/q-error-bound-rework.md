# Task: Derive a tighter Q error bound for near-singular KKT matrices

## Context

The KKT solver's Q error bound E = (9/2)||r||²/|λ_min| ([lem:q-error-bound]) blows up when M has near-zero eigenvalues. The code works around this by using |λ_min| of retained eigenvalues instead of all eigenvalues, but this doesn't match the lemma — the code applies an unproven tighter bound. This was discovered during the gradient-correctness session (2026-03-27/28) when auditing code-math correspondence.

## Scope

1. **Design and run a numerics experiment** that studies the KKT eigendecomposition error on near-singular M matrices. Use generic KKT matrices (not specific to capacity calculations). Vary |λ_min| from 1e-3 to 1e-15. Measure: actual Q error, predicted E from lemma, eigenvalue perturbation from Weyl's theorem, eigenvector mixing from Davis-Kahan.

2. **Prove a new lemma** giving a Q error bound that doesn't blow up for near-singular M. Promising direction: the β-block of near-null eigenvectors is O(|λ_j|) (Type C impossibility), so ||δβ|| = O(||r||) without the 1/|λ_min| factor. The quadratic term δβ^T H δβ = O(||r||²). The cross terms involving δμ, δξ are the hard part — they still have 1/|λ_min|. Using Q = -ξ/2 (lem:well-defined) and backward error bounds on eigendecomposition (Weyl, Davis-Kahan) may give a bound on |δξ| that depends on ε_mach·||M|| instead of 1/|λ_min|.

3. **Update the code** to use the new bound. Remove the KNOWN MISMATCH comment in `saddle_point_solver.rs`. Update the heuristic thresholds to match the new math.

## Out of scope

- Changing the two-tier eigenvalue detection strategy (it works fine)
- Changing the null-space LP search (it works fine)
- Fixing the gap invariant in capacity_accumulator.rs (separate issue, tracked in TASKS.md)
- Rewriting the KktOutcome enum (already done this session)

## Key files

- `crates/src/kkt/saddle_point_solver.rs` — the solver, lines 354-567 (Q error bound computation and quality gates)
- `crates/src/kkt/math.tex` — lem:q-error-bound (lines 251-393), lem:well-defined (lines 101-155)
- `experiments/q-error/` — existing Q error calibration experiment
- `TASKS.md` — q-error-threshold and code-math-correspondence-audit tasks

## Prior findings

**Dead ends tried this session:**
- Using all eigenvalues for |λ_min| (matches lemma but E blows up, 3 tests fail on LP(4,4) triangle product)
- Converting Q error assert to panic → return None → NumericalFailure (correct as KktOutcome but doesn't fix the bound)
- catch_unwind in experiments (violates error handling convention)

**Key structural property (not yet formalized):**
For near-null eigenvector v_j of M with |λ_j| ≤ τ: the β-block ||(v_j)_β|| = O(|λ_j|) (proved in kkt/math.tex as Type C impossibility, verified by Jörn 2026-03-22). This means δβ from discarded eigenvectors is O(||r||), not O(||r||/|λ_min|). The Q-quadratic term doesn't blow up. The cross terms (r₂^T δμ, r₃ δξ) still do because μ, ξ components of null eigenvectors are O(1).

**Standard perturbation theory to investigate:**
- Weyl's theorem: |λ̃_j - λ_j| ≤ ||δM||
- Davis-Kahan sin θ theorem: eigenvector mixing bounded by ||δM||/gap_j
- Backward error: computed eigendecomposition is exact for M + δM, ||δM|| = O(ε_mach ||M||)
- These may bound |δξ| independently of 1/|λ_min|

## Branch state

Branch `gradient-correctness`, worktree at `.claude/worktrees/gradient-correctness`. ~65 commits ahead of main. The KktOutcome refactor, error handling convention, and code-math audit fixes are committed. The heuristic quality gates (Q error bound, Q correction → NumericalFailure) work but are not mathematically justified.

## Success criteria

1. A new lemma in `crates/src/kkt/math.tex` that proves an error bound matching what the code computes (retained eigenvalue |λ_min|), or a better bound
2. The KNOWN MISMATCH comment in saddle_point_solver.rs is removed
3. The heuristic thresholds are either derived from the new lemma or documented with explicit justification
4. cargo test --release --lib passes (325 tests)
5. The numerics experiment validates the new bound empirically
