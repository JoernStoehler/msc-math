# symplectic

Durable Rust crate for symplectic geometry on 4D convex polytopes.

Main code areas:
- `src/geom/`: flat polytope fixtures, dual-vertex validation, symplectic
  form helpers, and Euclidean geometry reexports used by symplectic algorithms
- `src/kkt/`: context-free KKT/QP solve machinery
- `src/algorithms/`: HK2017, billiard, and flow-graph algorithm surfaces
- `src/exact/`: exact single-orbit kernels over ordered fields
- `src/database.rs`, `src/dataset.rs`, `src/derivatives.rs`, `src/random.rs`:
  persistence, row schemas, derivatives, and sampling support

Local tests are smoke/unit/regression checks only. Larger validation or
performance suites belong in `experiments/verification/` or experiment-owned
benchmark directories.

Developer-facing math for reusable crate algorithms lives in `formal/`.
