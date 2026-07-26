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
`algorithms::capacity_4d::capacity_from_dual_vertices`. It checks the cheap
facet-count and dual-norm limits, constructs exact binary64-rational geometry,
checks primal norms, and dispatches exact q/p products to the KKT-free
six-facet closure-vertex route; other inputs use the certified general QP
route. Product results contain an exact dyadic-rational capacity and sparse
exact witnesses. General results contain outward binary64 bounds. Neither
route promises every minimizing or near-minimizing orbit branch.

Callers that need intermediate geometry can instead use the named stages
`check_facet_count`, `check_finite_dual_vertices`,
`check_dual_vertex_norm_bounds`, `exact_binary64_polytope_geometry`,
`check_primal_vertex_norm_bounds`, and `capacity`. `PolytopeGeometry4d` is a
plain exact-geometry data object, not an opaque prepared search.
`capacity_transition_graph` and
`classify_lagrangian_product` expose the two derived combinatorial facts.

The production route allows at most 16 facets and requires every primal and
dual vertex infinity norm to lie in `[1e-3, 1e3]`. General enumeration is
soft-rejected above 100,000 candidate cycles. That limit is checked by the
general route, so exact geometry and automatic product dispatch do not depend
on materializing the general candidate set.

The copy-editable experiment counterparts and the exact/numerical
correspondence producers live under
`experiments/dev-quadratic-program/src/selected_route/` and
`experiments/dev-quadratic-program/tools/`.
