# Q-Error: Numerical Accuracy of the KKT Solver

Verify that f64 numerical errors in the KKT solver stay within proven error bounds (Lemma A.11), both by sweeping all (S,sigma) nodes and by exact comparison with rational arithmetic.

## Status
Complete

## Design

- Input: known polytopes from `known_polytopes::all_known()`, filtered to F <= 10 (7 polytopes)
- Part 1 (All-node sweep): For every (S,sigma) pair, assert error bound E < 1e-6 and |q_correction| < 1e-6
- Part 2 (Exact comparison): For winning nodes, solve KKT exactly over Q (Gaussian elimination with BigRational) and assert |Q_tilde - Q_exact| <= max(E, f64_eps)

## Key findings

- **Part 1:** 1,133,769 total nodes, 1,109,987 solvable. Worst E = 2.9e-11 (hko_pentagon). All assertions pass
- **Part 2:** Exact comparison passes for 6/7 polytopes (symplectic triangle product has singular winning node). Actual errors at machine epsilon (~1e-16), confirming no algorithmic error beyond f64 precision
- Safety-net tolerance (1e-13 * (1+|Q_exact|)) never triggered, meaning mathematical bound E is always above f64 noise

## Files

| File | Purpose |
|------|---------|
| `q_error.rs` | Rust binary: all-node sweep + exact comparison |
| `q-error.tex` | Thesis writeup |
| `q_error_output.txt` | Captured stdout output with summary tables |

## Run

```bash
cd experiments/ && cargo run --release --bin q_error
```

## Known limitations

- Only 7 polytopes tested (known polytopes with F <= 10)
- No random polytopes in dataset; may miss edge cases in general position
- Symplectic triangle product excluded from exact comparison (singular winning node)
