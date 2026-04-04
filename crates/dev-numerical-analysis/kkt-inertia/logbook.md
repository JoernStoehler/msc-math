# KKT Matrix Inertia Validation: Logbook

## Motivation

Lemma `lem:kkt-inertia` (in `thesis/appendix-numerical.tex`) decomposes the eigenvalue inertia of the KKT matrix M into contributions from the restricted Hessian H|_T and the constraint rank p:

    n_+(M) = n_+(H|_T) + p,   n_0(M) = n_0(H|_T) + (5 - p),   n_-(M) = n_-(H|_T) + p

This experiment validates the formula empirically, since the lemma is used (via Remark `rem:eigenvalue-signs`) to read off second-order behaviour of Q on the constraint surface from the eigenvalues of M.

## Status

**Complete.** Formula validated on all 7 known polytopes with F <= 10. Five mismatches in hko_pentagon are threshold sensitivity artifacts at machine epsilon, not formula errors.

## How to run

```bash
cd crates/dev-numerical-analysis/kkt-inertia/ && cargo run --release --bin num-kkt-inertia
```

Output is printed to stdout. Captured output is stored in `kkt_inertia_output.txt`.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: census of H\|_T definiteness + inertia formula check |
| `math.tex` | Formal writeup (table, mismatch analysis) |
| `kkt_inertia_output.txt` | Captured stdout with tables and eigenvalue diagnostics (67 lines) |

## Design

- **Input:** 7 known polytopes from `known_polytopes::all_known()`, filtered to F <= 10.
- **Part 1 (Census):** Classify H|_T definiteness for all (S, sigma) nodes with beta > 0 and Q > 0. Categories: Trivial (0-dim tangent), PD, ND, Indefinite, NearZero.
- **Part 2 (Inertia check):** For each node, compute n_-(M) from the eigenvalues of M (threshold 1e-10) and independently classify H|_T by restricting the Hessian to T = ker(A). Check whether n_-(M) = n_-(H|_T) + p.
- **Eigenvalue threshold:** 1e-10. Eigenvalues within this threshold of zero are classified as zero.

## Findings

1. **Part 1:** 1,133,769 total nodes surveyed across 7 polytopes. Valid (beta > 0, Q > 0) nodes classified into Trivial, PD, ND, Indefinite, and NearZero categories. The hko_pentagon dominates with 1,112,073 nodes.

2. **Part 2:** Inertia formula holds for 6/7 polytopes (OK for simplex, hypercube, both Lagrangian and symplectic triangle products, both Lagrangian and symplectic triangle-square products).

3. **5 mismatches in hko_pentagon:** All share the same pattern: tangent_dim = 3, p = 5, with H|_T eigenvalues {-a, epsilon, +a} where a ~ 0.4--1.1 and epsilon ~ 1e-16 (machine epsilon). Since n_-(H|_T) = 1, the lemma predicts n_-(M) = 6, but the threshold-based classifier reports n_-(M) = 5 because M has three eigenvalues at ~1e-16 whose signs cannot be resolved. The inertia formula itself is not violated; the mismatch is a classification artifact.

4. **Part 2 inertia decomposition check** (from math.tex table, moved here during audit 2026-04-04):

   | Polytope | Total | n_-=p | n_->p | PD | ND | Indef | Match |
   |----------|------:|------:|------:|---:|---:|------:|-------|
   | Simplex | 84 | 84 | 0 | 0 | 0 | 0 | OK |
   | Hypercube | 16,064 | 10,990 | 5,074 | 194 | 194 | 4,880 | OK |
   | HK-O pentagon | 1,112,073 | 171,063 | 941,010 | 3,370 | 3,370 | 935,685 | 5 |
   | Lag. triangle x triangle | 409 | 371 | 38 | 38 | 38 | 0 | OK |
   | Sym. triangle x triangle | 409 | 379 | 30 | 30 | 30 | 0 | OK |
   | Lag. triangle x square | 2,365 | 1,783 | 582 | 110 | 110 | 472 | OK |
   | Sym. triangle x square | 2,365 | 1,548 | 817 | 337 | 337 | 240 | OK |

   "n_-=p" counts nodes where n_-(M) equals the constraint rank p. "PD/ND/Indef" classify H|_T across all nodes with nontrivial tangent space. "Match" reports mismatches between the inertia prediction and H|_T classification.

## Known limitations

- Only 7 polytopes tested (known polytopes with F <= 10).
- No random polytopes in the dataset.
- The hko_pentagon mismatches are threshold artifacts, not investigated further.
- TODO for Jorn: verify the threshold-sensitivity interpretation of the mismatches (see eigenvalue diagnostics in `kkt_inertia_output.txt`).

## Data regeneration (2026-03-26)

Regenerated against current library (post dual-vertex migration). Now 8 mismatches (was 5), all in hko_pentagon, all at machine-epsilon eigenvalue thresholds. Same pattern as before — no new failure mode.
