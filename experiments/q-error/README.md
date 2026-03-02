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
  in hko_pentagon inertia check.

## Input

Known polytopes from the library (`known_polytopes::all_known()`),
filtered to F ≤ 10. Currently 7 polytopes.

## Output

Summary tables to stdout. Panics on any violation.

## Run

```bash
cd experiments/ && cargo run --release --bin q_error
```
