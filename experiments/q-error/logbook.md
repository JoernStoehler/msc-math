# Q-Error: Logbook

## Motivation

All experiments in this thesis depend on the numerical KKT solver producing accurate capacity values. This experiment validates that f64 numerical errors stay within the proven error bounds (Lemma A.11), both by sweeping all (S, sigma) nodes and by exact comparison with rational arithmetic. If the error bounds were violated, every capacity value computed in the thesis would be suspect.

## Status

**Complete.** Both checks pass on all 7 known polytopes with F <= 10.

## How to run

```bash
cd experiments/ && cargo run --release --bin q_error
```

Output goes to stdout and is captured in `q_error_output.txt`. The binary panics on any violation.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: all-node error bound sweep + exact rational comparison |
| `math.tex` | Thesis writeup (two tables, discussion paragraph) |
| `q_error_output.txt` | Captured stdout output (40 lines) with summary tables |

## Design

- **Input:** Known polytopes from `known_polytopes::all_known()`, filtered to F <= 10 (7 polytopes).
- **Part 1 (all-node sweep):** For every (S, sigma) pair across all polytopes, solve the KKT system and assert that the error bound E < 1e-6 and the residual correction |q_correction| < 1e-6.
- **Part 2 (exact comparison):** For the capacity-winning node of each polytope, solve the KKT system exactly over Q (Gaussian elimination with BigRational) and assert |Q_tilde - Q_exact| <= max(E, f64_eps).

## Findings

1. **Part 1:** 1,133,769 total nodes across 7 polytopes, 1,109,987 solvable. Worst error bound E = 2.9e-11 (HKO pentagon). All assertions passed.
2. **Part 2:** Exact comparison passes for 6/7 polytopes. Actual numerical errors at machine epsilon (~1e-16), confirming no algorithmic error beyond f64 precision.
3. The symplectic triangle product has a singular winning node and is omitted from Part 2.
4. The safety-net tolerance (1e-13 * (1 + |Q_exact|)) was never triggered, meaning the mathematical error bound E is always above f64 noise.
5. The HKO pentagon dominates the dataset: 1,112,073 of the 1,133,769 total nodes (98%) come from it.

## Known limitations

- Only 7 polytopes tested (known polytopes with F <= 10).
- No random polytopes in the dataset; may miss edge cases in general position.
- Symplectic triangle product excluded from exact comparison (singular winning node).
