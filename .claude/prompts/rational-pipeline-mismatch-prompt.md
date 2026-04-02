# Session: Fix rational arithmetic pipeline mismatch in appendix A.1

## Problem

`thesis/appendix-numerical.tex` Section A.1 ("Input Representation", lines 57-165) describes a rational arithmetic pipeline that the Rust library does not implement:

| Thesis claims | Code reality |
|--------------|--------------|
| Input polytopes have rational halfspace data (n_i ∈ Q^4, h_i ∈ Q>0) | `Polytope4D` stores `Vec<Vector4<f64>>` and `Vec<f64>` — pure f64 |
| Vertices computed via Cramer's rule over Q | Vertex enumeration via qhull subprocess on f64 data |
| Vertex-facet incidence exactly decidable over Q | Incidence checked via f64 dot product with tolerance `EPS_FACET_INCIDENCE = 1e-8` |
| Symplectic signs ω₀(n_i, n_k) exact since n_i ∈ Q^4 | `omega0(&n[j], &n[i]) >= 0.0` — floating-point comparison |
| Generic perturbation of rational normals to break ω₀=0 degeneracies | No perturbation step anywhere in library or experiments |
| fp64 conversion only for numerics (unit-length normals) | Everything is f64 from the start |

No rational arithmetic library exists in `Cargo.toml` dependencies.

## Severity

HIGH — thesis-visible. The appendix reads as if this pipeline is implemented. The `% [TODO: JÖRN -` markers (lines 102-105, 126-129, 139-142) are on secondary questions; the core rational-data claim (lines 61-97) has no TODO and reads as fact.

## Key files

- `thesis/appendix-numerical.tex:57-165` — the claims
- `crates/library/src/geom/polytope.rs` — `Polytope4D` struct (f64 storage)
- `crates/library/src/geom/vertices.rs`, `crates/library/src/geom/qhull.rs` — vertex enumeration (qhull, not Cramer)
- `crates/library/src/algorithms/hk2017/mod.rs:242` — symplectic sign check (f64 comparison)
- `crates/library/src/geom/known_polytopes.rs` — polytope constructors (exact-looking constants, but stored as f64)

## Jörn's direction

Jörn said: "we don't actually use [the rational pipeline]." The thesis appendix needs to describe what the code actually does. Jörn rejected the Q interval bounds approach as "not useful" — the code uses best-guess Q̃ + computed error bound E, which is already correctly described (after the label rename in commit 72cf05c).

## Scope

Rewrite appendix A.1 to describe the actual f64 pipeline. Key changes:
1. Replace "rational halfspace data" with f64 halfspace data (noting that known polytopes use exact-looking rational constants stored as f64)
2. Replace Cramer's rule with qhull vertex enumeration
3. Replace exact incidence with tolerance-based incidence
4. Replace exact symplectic signs with f64 comparison + tolerance handling (three-valued in hk2017)
5. Remove or reframe the perturbation paragraph (code doesn't do it)
6. Keep the f64 conversion/normalization paragraph (still accurate)

The rest of appendix-numerical.tex (KKT solver, Q error bound, accumulator) is already accurate — only A.1 "Input Representation" needs rewriting.
