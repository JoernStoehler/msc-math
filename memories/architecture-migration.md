# Architecture migration opportunities

Source inspection 2026-09-07, including three parallel investigations. This is
a proposed migration surface, not active assignments or evidence that tests
pass. No builds, experiments or timing measurements were performed.

Jörn asked for architecture analysis and actual migration of settled reusable
code, potentially with instrumented copies linked bidirectionally. The earlier
plan had incorrectly narrowed this to one geometry-enumeration overlap.

## Existing migration, not a blank slate

The existing homes are meaningful: `crates/algebraic-numbers` owns exact scalars
and linear algebra; `crates/euclidean-polytopes` owns ordinary convex geometry;
`crates/symplectic` owns symplectic, capacity and orbit machinery. Inspection
did not establish a need for another core crate. Public exports in symplectic
include production, historical and branch-sensitive interfaces.

The requested copy relationship already exists between
`crates/symplectic/src/algorithms/capacity_4d/general.rs` and
`experiments/dev-quadratic-program/src/selected_route/general.rs`: reciprocal
headers distinguish the fixed production route from experimental variants and
counters. The experiment's `selected_route/product.rs` points to production
`capacity_4d/product.rs`, but the latter lacks the reciprocal pointer.

`experiments/dev-quadratic-program/tests/selected_route_correspondence.rs`
compares general bounds, exact action windows, derivatives, and product
certificates/winners. These are finite checks, not universal equivalence.
`experiments/regular-products/src/capacity.rs` is a useful migrated consumer:
cached geometry feeds production minimizers; local code selects a deterministic
word for bounce count. Its test compares the cached and raw-input routes.

Relevant history: `588ef643` extracted certified capacity routes; `1fde8b6f`
migrated representative consumers; `68b3ea9c` added action windows; `75daa6f4`
migrated the data-science producer. A new session can continue this trajectory.

## 1. Ordinary geometry: migrate components and callers

Parallel candidates, with existing `euclidean-polytopes` as destination:

- `symplectic/src/geom/polygon.rs`: regular/random polygons, rotation and area;
  consumers include regular-products, polytope-datasets and dev-sys-prediction.
- `symplectic/src/derivatives.rs::volume_derivatives_a`: Euclidean kernel mixed
  with capacity derivatives; consumers include HKO, optimizer branch models,
  combinatorial cells and sys prediction. Capacity/systolic-gradient assembly
  remains symplectic.
- Old `symplectic/src/geom/vertex_enumeration/enumerate.rs` versus
  `euclidean-polytopes/src/polar.rs`: the old rational route uses f64 filters;
  redundant-input/error behavior differs. `capacity_4d/geometry.rs` uses the
  newer route; `symplectic/src/random.rs` and `geom/known_polytopes.rs` retain
  old calls. Experiment-local flat-polytope validation also differs.

First migrations can retain compatibility reexports and move selected callers.
Characterization should cover small values, errors, incidence and ordering;
existing volume-derivative finite-difference tests are a starting source, not
an already measured cheap tier. The random-polygon boundedness claim needs
checking before being carried over as a contract.

## 2. Exact support: extract a narrow theorem-independent kernel

Strong first extraction: private generic `build_kkt_matrix` in
`crates/symplectic/src/exact/orbit.rs` and its specialized copy in
`experiments/hko-local-maximum/theorem/active_branch_diagnostic.rs` build the
same omega, closure, normalization matrix and RHS. Expose the assembly primitive
under `symplectic::exact` and migrate the diagnostic, preserving sign,
multiplier and facet-order conventions. This promises assembly, not capacity
certification. Exact entrywise comparisons on simplex/HKO words, including a
singular word, provide a bounded migration check.

Independent fixture-ownership candidate:
`hko-local-maximum/src/exact_bank.rs` constructs algebraic HKO over
`Q[tan(pi/5)]` and a rational simplex; `symplectic/geom/known_polytopes.rs`
constructs HKO from f64 literals and rationalizes binary64. These are different
exact objects. The latter's “single source of truth” wording overstates the
arrangement. Reusable exact constructors could gain a library home without
moving selected sigma banks, row labels, symmetry reduction or report logic.
Changing scaling/order/representation is not merely relocation.

## 3. Production, controls and instrumentation: reuse mechanisms

One session can strengthen the existing copy correspondence and migrate a
repeated traversal while leaving scientific selection policies explicit.

`hko-local-maximum/src/instrumented_search.rs` and
`combinatorial-cells/src/instrumented_capacity.rs` repeat subset/cyclic-word
traversal and transition pruning before calling the library saddle solver.
Both retain strict positive-beta orbits; HKO also records uncertain minima,
while cell studies need counts and second-best gaps. Shared enumeration from
`symplectic/algorithms/hk2017` is a candidate, but the richer collector or scalar
capacity is not a drop-in replacement. Compare word order/set, selections,
actions, counts and gaps, including near-zero beta—not scalar values alone.

Similarly repeated `capacity_pruned_hk2017`, `capacity_billiard`, `capacity_auto`
adapters occur in HKO, combinatorial-cells and sys-landscape `src/lib.rs`.
They could share an explicitly retained orbit frontend; argument ordering and
historical observable contracts need preserving.

`exp-dev-quadratic-program` exports exact conversion/transition/control helpers
alongside generated fixtures, profiling and report schemas. Performance,
qp-error-bounds, gradient code and optimizer-runs depend on it. In particular,
`src/exact_route/mod.rs` and `src/geometry.rs` merit a reusable-support boundary
inspection. Independent exact controls need not become production code.
The historical optimizer evaluator consumes winning-orbit/gradient context;
switching it to current scalar capacity changes the experiment, not just imports.

## 4. Deformation and flow graph: establish the contract before migration

Two independent candidates:

- `combinatorial-cells/src/boundary_events.rs` and
  `sys-landscape/src/step_bound.rs` overlap, with consumers in cell studies,
  ascent and gradient diagnostics. Incidence crossings are extrapolated from
  slack/current vertex derivative, whereas omega crossings use a quadratic.
  The first is not thereby a certified safe boundary. Characterize analytic
  simple trajectories, singular cases and threshold/cap behavior before
  extracting a shared event-prediction primitive; experiment policies stay local.
- `symplectic/algorithms/flow_graph/exact_tube.rs` already supplies metrics and
  snapshots to `dev-flow-graph/visualize-tube/main.rs`. Instrumentation therefore
  does not always need a copy. Older `dev-flow-graph/closed-word-spike/main.rs`
  embeds related affine/polygon/fixed-point code. Determine whether it earns
  retention as an independent control/historical producer or can use the library.
  Compare per-word classification, fixed-point dimension, exact action and
  boundary behavior. Resolver shape checks do not establish supplied incidence
  or omega-sign correctness.

## First work and remaining limits

Recommended first parallel work: Euclidean component characterization, exact
KKT assembly extraction, and traversal/correspondence characterization. Each
migration depends on its own contract/checks, not a completed global inventory.
Package 4 is less ready for immediate extraction but independently investigable.

The widely imported `exp-sys-landscape` cache/schema/policy hub is another
opportunity, not yet grounds for a new dataset crate. Launcher/artifact-cache
operational repairs belong to the separate execution-reliability session.
The separately pinned nalgebra 0.35 factorization dependency in
`symplectic/Cargo.toml` is tied to numerical evidence; collapsing it with
workspace 0.33 is not routine cleanup.

The inspection supports narrow reusable boundaries, not scientific stability
of all public code. Proof-specific Sage, witness selection, historical controls
and optimizer policies can remain local. Changes to scientific observables or
the intended value of a control need Jörn's input after recoverable caller and
provenance facts have been inspected; none blocks these initial investigations.
