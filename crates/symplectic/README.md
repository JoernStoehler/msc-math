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
- `src/algorithms/capacity_4d/`: validated scalar EHZ-capacity API; exact
  product dispatch, certified general bounds, and exact product witnesses
- `src/algorithms/`: HK2017, billiard, flow-graph, and orbit-sensitive
  algorithm surfaces
- `src/exact/`: exact single-orbit kernels over ordered fields
- `src/database.rs`, `src/dataset.rs`, `src/derivatives.rs`, `src/random.rs`:
  persistence, row schemas, derivatives, and sampling support

Local tests are smoke/unit/regression checks only. Larger validation or
performance suites belong in `experiments/verification/` or the relevant
experiment's benchmark directory.

Developer-facing math for reusable crate algorithms lives in `formal/`.

## Scalar four-dimensional capacity

Start with
`algorithms::capacity_4d::CapacityInput4d::try_from_dual_vertices`. Validation
interprets each binary64 coordinate as its exact dyadic rational, checks exact
four-dimensional polytope geometry, allows at most 16 facets, and requires
every primal and dual vertex infinity norm to lie in the inclusive interval
`[1e-3, 1e3]`.

Calling `capacity()` then dispatches exact q/p products to the KKT-free
six-facet closure-vertex route; other inputs use the certified general QP
route. Product results contain an exact dyadic-rational capacity and sparse
exact witnesses. General results contain outward binary64 bounds. Neither
route promises every minimizing or near-minimizing orbit branch.

The copy-editable experiment counterparts and the exact/numerical
correspondence producers live under
`experiments/dev-quadratic-program/src/selected_route/` and
`experiments/dev-quadratic-program/tools/`.
