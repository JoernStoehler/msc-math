# Rust crates

Reusable Rust implementation lives in three Cargo crates. Each crate README is
the entry point; source and tests establish behavior.

| Crate | Owns | Does not own |
| --- | --- | --- |
| `algebraic-numbers/` | exact real-algebraic scalars and exact dense linear algebra | general computer algebra or symplectic geometry |
| `euclidean-polytopes/` | ordinary convex-polytope geometry, incidence, polar, face, and volume helpers | EHZ capacity, Reeb dynamics, or symplectic signs |
| `symplectic/` | symplectic geometry, capacity algorithms, KKT/QP machinery, orbit search, derivatives, fixtures, and record helpers | thesis-specific empirical conclusions |

## Symplectic algorithm owners

| Concern | Source owner | Evidence beyond unit tests |
| --- | --- | --- |
| HK2017/QP capacity | `symplectic/src/algorithms/hk2017/`, `orbit_search.rs`, `kkt/` | `experiments/dev-quadratic-program/`, `experiments/verification/` |
| Billiard and Lagrangian-product enumeration | `symplectic/src/algorithms/billiard.rs` and shared QP/KKT layers | `experiments/regular-products/`, HKO packet |
| Flow graph | `symplectic/src/algorithms/flow_graph/` | `experiments/dev-flow-graph/`, `experiments/verification/` |
| Exact single-orbit calculations | `symplectic/src/exact/` | theorem-local Sage or formal packets when proof strength is required |
| Derivatives and local diagnostics | `symplectic/src/derivatives.rs` | first-order formal notes and development experiments |
| Random polytopes and datasets | `symplectic/src/random.rs`, `database.rs`, `dataset.rs` | consuming experiment owner |

Code existence does not imply a settled public API, exact proof, acceptable
numerical behavior, or thesis use. Confirm those separately at the relevant
owner.

## Baseline checks

```bash
cargo test -p algebraic-numbers --release
cargo test -p euclidean-polytopes
cargo test -p symplectic --release --lib
cargo test -p symplectic --release --test public_capacity_api
```

Use crate-local `DEVELOPMENT.md` files for maintainer notes and expensive or
non-obvious checks.
