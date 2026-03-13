# kkt_inertia: KKT matrix inertia validation

## Goal

Validate Lemma `lem:kkt-inertia` (Inertia of the KKT matrix) from
`thesis/appendix-numerical.tex`. The lemma states:

    n_+(M) = n_+(H|_T) + p,  n_0(M) = n_0(H|_T) + (5-p),  n_-(M) = n_-(H|_T) + p

where M is the KKT matrix, H|_T is the restricted Hessian on the
tangent space T = ker(A), and p = rank(A).

Two parts:
1. **Census:** Classify H|_T definiteness for all (S,σ) nodes with
   β>0 and Q>0 across all known polytopes with F ≤ 10.
2. **Inertia check:** Verify the inertia decomposition formula. On
   mismatch, print eigenvalue diagnostics.

## Results (2026-03-13)

- **Part 1:** Census across 1,133,769 nodes. Among valid (β>0, Q>0)
  nodes: mostly Trivial (0-dim tangent space), with PD, ND, Indefinite,
  and NearZero categories for nodes with nontrivial tangent spaces.
- **Part 2:** Inertia formula holds for 6/7 polytopes. 5 mismatches in
  hko_pentagon, all with the same pattern: tangent_dim=3, H|_T has
  eigenvalues [-a, ~0, +a]. The near-zero H|_T eigenvalue (~10⁻¹⁶) is
  at machine epsilon, and the M eigenvalues near zero (~10⁻¹⁶ to
  ~10⁻¹⁸) are ambiguous in sign. These are threshold sensitivity
  artifacts: the inertia formula is correct but the discrete eigenvalue
  classification cannot resolve eigenvalues at machine epsilon.

## Input

Known polytopes from the library (`known_polytopes::all_known()`),
filtered to F ≤ 10. Currently 7 polytopes.

## Output

Summary tables to stdout plus eigenvalue diagnostics for any mismatches.
No hard assertions (diagnostic experiment).

## Run

```bash
cd experiments/ && cargo run --release --bin kkt_inertia
```
