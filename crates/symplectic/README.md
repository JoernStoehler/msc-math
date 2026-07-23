# symplectic

Durable Rust crate for symplectic geometry on 4D convex polytopes.

This crate is the physical home of reusable implementation and cheap regression
tests. It does not establish theorem acceptance, numerical certification of
every public path, or thesis-level empirical conclusions. Confirm those in
`formal/`, `experiments/verification/`, and the relevant topic entry point.

## Start here

Choose the implementation area from this list, then inspect its tests and any
linked experiment evidence:

- `src/geom/`: flat polytope fixtures, dual-vertex validation, symplectic
  form helpers, and Euclidean geometry reexports used by symplectic algorithms
- `src/kkt/`: context-free KKT/QP solve machinery
- `src/algorithms/`: HK2017, billiard, and flow-graph algorithm surfaces
- `src/exact/`: exact single-orbit kernels over ordered fields
- `src/database.rs`, `src/dataset.rs`, `src/derivatives.rs`, `src/random.rs`:
  persistence, row schemas, derivatives, and sampling support

Local tests are smoke/unit/regression checks only. Larger validation or
performance suites belong in `experiments/verification/` or the relevant
experiment's benchmark directory.

Developer-facing math for reusable crate algorithms lives in `formal/`.
