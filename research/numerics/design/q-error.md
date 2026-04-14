# Q-Error: Logbook

## Motivation

All experiments in this thesis depend on the numerical KKT solver producing accurate capacity values. This experiment validates that f64 numerical errors stay within the proven error bounds (Lemma `lem:q-error-bound` in math.tex), both by sweeping all (S, sigma) nodes and by exact comparison with rational arithmetic. If the error bounds were violated, every capacity value computed in the thesis would be suspect.

## Status

**Complete.** Both checks pass on all 7 known polytopes with F <= 10.

## How to run

```bash
cargo run -p dev-numerical-analysis --release --bin num-q-error
```

Output goes to stdout and is captured in `q_error_output.txt`. The binary panics on any violation.

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: all-node error bound sweep + exact rational comparison |
| `math.tex` | Formal writeup (two tables, discussion paragraph) |
| `q_error_output.txt` | Captured stdout output (40 lines) with summary tables |

## Design

- **Input:** Known polytopes from `known_polytopes::all_known()`, filtered to F <= 10 (7 polytopes).
- **Part 1 (all-node sweep):** For every (S, sigma) pair across all polytopes, solve the KKT system and assert that the error bound E < 1e-6 and the residual correction |q_correction| < 1e-6.
- **Part 2 (exact comparison):** For the capacity-winning node of each polytope, solve the KKT system exactly over Q (Gaussian elimination with BigRational) and assert |Q_tilde - Q_exact| <= max(E, f64_eps).

## Findings

1. **Part 1:** 1,133,769 total nodes across 7 polytopes, 1,111,987 solvable. Worst error bound E = 2.9e-11 (HKO pentagon). All assertions passed.
2. **Part 2:** Exact comparison passes for all 6 non-singular polytopes. The symplectic triangle product is excluded (singular winning node). Actual numerical errors at machine epsilon (~1e-16), confirming no algorithmic error beyond f64 precision.
3. **Part 2 error bounds are far below f64 noise:** At the capacity-winning nodes (Part 2), E_math ranges from 1e-30 to 1e-28 — far below f64_eps (~1e-13). The assertion `|Q̃ - Q_exact| <= max(E, f64_eps)` is always dominated by the f64_eps term at winning nodes. Note: Part 1's worst E across ALL nodes is 2.9e-11 (HKO pentagon), which exceeds f64_eps — but Part 1 uses a different assertion (`E < 1e-6`), not the exact comparison.
5. The HKO pentagon dominates the dataset: 1,112,073 of the 1,133,769 total nodes (98%) come from it.

### Table: Error bound sweep across all (S, sigma) nodes

| Polytope | F | Nodes | Solvable | Worst E | Worst ‖r‖ | Worst |dQ| |
|---|---|---|---|---|---|---|
| Simplex | 5 | 84 | 24 | 7.3e-28 | 5.2e-15 | 1.2e-14 |
| Hypercube | 8 | 16,064 | 15,600 | 3.4e-19 | 8.9e-11 | 2.6e-11 |
| HK-O pentagon | 10 | 1,112,073 | 1,091,364 | 2.9e-11 | 7.1e-7 | 4.3e-8 |
| Lag. triangle x triangle | 6 | 409 | 304 | 1.7e-19 | 1.2e-10 | 3.9e-15 |
| Sym. triangle x triangle | 6 | 409 | 243 | 1.2e-17 | 8.0e-10 | 1.9e-10 |
| Lag. triangle x square | 7 | 2,365 | 2,226 | 9.9e-22 | 1.2e-11 | 3.9e-14 |
| Sym. triangle x square | 7 | 2,365 | 2,226 | 2.6e-20 | 5.2e-11 | 1.3e-11 |

"Nodes" = total combinatorial types; "Solvable" = non-singular KKT system.
‖r‖ = KKT residual norm; dQ = Q_tilde - Q(beta_hat) = residual correction.
All assertions passed: E < 1e-6 and |dQ| < 1e-6 for every solvable node.

### Table: Exact rational comparison on capacity-winning nodes

| Polytope | m | Q_tilde | Q_exact | |Q_tilde - Q_exact| | E | Valid |
|---|---|---|---|---|---|---|
| Simplex | 5 | 2.0000 | 2.0000 | 0 | 2.6e-28 | OK |
| Hypercube | 4 | 0.1250 | 0.1250 | 0 | 1.2e-29 | OK |
| HK-O pentagon | 7 | 0.1453 | 0.1453 | 8.3e-17 | 1.1e-29 | OK |
| Lag. triangle x triangle | 6 | 0.3333 | 0.3333 | 5.6e-17 | 5.4e-30 | OK |
| Lag. triangle x square | 5 | 0.3333 | 0.3333 | 5.6e-17 | 4.6e-30 | OK |
| Sym. triangle x square | 4 | 0.5000 | 0.5000 | 1.1e-16 | 5.2e-30 | OK |

m = orbit length. Sym. triangle x triangle omitted (singular KKT matrix at winning node).
Assertion |Q_tilde - Q_exact| <= max(E, eps_machine) passed for all 6 solvable polytopes.

## Known limitations

- Only 7 polytopes tested (known polytopes with F <= 10).
- No random polytopes in the dataset; may miss edge cases in general position.
- Symplectic triangle product excluded from exact comparison (singular winning node).
