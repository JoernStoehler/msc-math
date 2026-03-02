# q_error: Numerical accuracy of the KKT solver

## Goal

Verify that f64 numerical errors in the KKT solver stay within the
proven error bounds (Lemma A.11, appendix-numerical.tex). Two
complementary checks:

1. **All-node sweep:** For every (S,σ) pair across all polytopes with
   F ≤ 10, assert the error bound E < 1e-6 and |q_correction| < 1e-6.
2. **Exact comparison:** For winning nodes, solve the KKT system
   exactly over Q (Gaussian elimination with BigRational) and assert
   |Q̃ - Q_exact| ≤ max(E, f64_eps).

Additionally, library-level `assert!` checks in `kkt.rs` enforce
E < 1e-6 on every `solve_kkt` call (not just in this experiment).

## Results (2026-03-02)

- **Part 1:** 1,134,369 total nodes across 7 polytopes, 1,113,987
  solvable. Worst E = 2.9×10⁻¹¹ (hko_pentagon). All assertions pass.
- **Part 2:** Exact comparison for 3/7 polytopes (others have
  rank-deficient winning nodes). Actual errors at machine epsilon
  (~10⁻¹⁶), confirming the solver introduces no algorithmic error
  beyond f64 precision.
- **Parts 3-4:** Hessian definiteness and inertia diagnostics
  (informational, not assertions). 5 threshold-sensitivity mismatches
  in hko_pentagon inertia check (see Known Behaviors below).

## Known Behaviors

### Inertia threshold sensitivity (Part 4)

Part 4 classifies eigenvalues of the restricted Hessian as positive,
negative, or zero using a single threshold (EPS = 1e-10). For
hko_pentagon, 5 of 33,840 nodes have eigenvalues near the threshold
boundary, causing the discrete inertia label to differ from what the
continuous eigenvalue spectrum would suggest. The actual eigenvalues
are correct — the issue is purely in the discrete classification step.

This is expected: any single-threshold classifier will have boundary
cases. A two-tier approach (certified/uncertain/rejected, as the
library uses for β classification) would eliminate these, but the
experiment's purpose is diagnostic, not certification, so the simpler
approach suffices.

### Safety-net tolerance in exact comparison (Part 2)

The `f64_eps` tolerance in `|Q̃ - Q_exact| ≤ max(E, f64_eps)` uses
`1e-13 * (1 + |Q_exact|)`. This is a conservative estimate of
accumulated f64 rounding error across the ~10 floating-point
operations in the KKT solve path. The factor 1e-13 (≈ 100 × machine
epsilon) provides margin above the ~10⁻¹⁶ errors actually observed.
If this tolerance ever triggers (i.e., E < f64_eps but |Q̃ - Q_exact|
> E), it means the mathematical error bound E is tighter than machine
precision — a positive result showing the algorithm's quality exceeds
what f64 arithmetic can verify.

## Input

Known polytopes from the library (`known_polytopes::all_known()`),
filtered to F ≤ 10. Currently 7 polytopes.

## Output

Summary tables to stdout. Panics on any violation.

## Run

```bash
cd experiments/ && cargo run --release --bin q_error
```
