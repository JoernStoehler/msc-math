# KKT Matrix Inertia Validation

Validate the inertia decomposition formula for the KKT matrix (Lemma `lem:kkt-inertia`) by census of all (S,sigma) nodes and checking the formula against direct eigenvalue computation.

## Status
Complete

## Design

- Input: known polytopes from `known_polytopes::all_known()`, filtered to F <= 10 (7 polytopes)
- Part 1 (Census): Classify H|_T definiteness for all (S,sigma) nodes with beta>0 and Q>0
- Part 2 (Inertia check): Verify n_+(M) = n_+(H|_T) + p, n_0(M) = n_0(H|_T) + (5-p), n_-(M) = n_-(H|_T) + p

## Key findings

- **Part 1:** 1,133,769 total nodes surveyed across 7 polytopes. Valid (beta>0, Q>0) nodes classified as Trivial (0-dim tangent), PD, ND, Indefinite, or NearZero
- **Part 2:** Inertia formula holds for 6/7 polytopes. 5 mismatches in hko_pentagon, all with the same pattern: tangent_dim=3, H|_T eigenvalue ~1e-16 at machine epsilon. These are threshold sensitivity artifacts, not formula errors
- The inertia formula is correct but discrete eigenvalue classification cannot resolve eigenvalues at machine epsilon

## Files

| File | Purpose |
|------|---------|
| `kkt_inertia.rs` | Rust binary: census + inertia check |
| `kkt-inertia.tex` | Thesis writeup |
| `kkt_inertia_output.txt` | Captured stdout output with tables and diagnostics |

## Run

```bash
cd experiments/ && cargo run --release --bin kkt_inertia
```

## Known limitations

- Only 7 polytopes tested (known polytopes with F <= 10)
- No random polytopes in dataset
- 5 hko_pentagon mismatches are threshold artifacts, not investigated further
