# Capacity/Orbit API Surface Target v2

Date: 2026-05-04
Repo context: `/workspaces/msc-math`
Status: fresh discussion draft, not accepted.
Guardrail: `crates/symplectic/API_REFACTOR_GOAL.md`.

This is the ambitious final API target, not a migration plan.

## Implementation Guardrails

These are part of the target, not optional handoff context.

- Implement the new API directly. Do not add compatibility wrappers, aliases,
  deprecated old entrypoints, or old-signature forwarding functions.
- If an old consumer breaks, migrate it to the new API. If migration is too
  large for the current slice, stop and report the exact blockers; do not keep
  the old API to make progress look complete.
- `Polytope4D` is not merely absent from the signatures below. It is rejected
  as a final API anchor for capacity/orbit code.
- Old root names such as `ehz_capacity`, `ehz_capacity_pruned`,
  `ehz_capacity_unpruned`, and `ehz_capacity_billiard` are rejected final
  public API names, not names to preserve as compatibility shims.
- `orbit_window` and exact `action_gap` are absent from core exact capacity
  because grep found no real exact gap-window consumer. Do not restore them
  without a current experiment-support caller.
- Exact `Billiard` is absent because grep found no exact billiard consumer.
  Do not add it as symmetry with the f64 enum.
- `FacetTopology` is deleted because it had no real invariant/consumer.
  Expose direct matrices only where a real consumer needs matrix-valued data.
- `kkt_augmented_system_f64` is deliberately not public. The public
  experiment-support boundary is `beta_directional_sensitivity_f64`.
- `F64Orbit` carries KKT data directly because f64 gradients and geometric
  recovery need `beta`, `q`, `mu`, and `xi`.
- `recover_geometric_orbit_from_parts_f64` is retained because visualization
  stores `(sigma, beta, action)` rather than a full solved orbit payload.
- `billiard_sigmas_with_bounce_count_f64` is retained because product
  experiments filter and persist bounce counts.
- `hk2017_transition_matrix_f64` is retained because datascience feature code
  consumes transition-density, bidirectionality, and out-degree statistics.

## Evaluation Criteria

Use these criteria before adding or keeping a symbol:

1. Real-consumer fit: ordinary capacity callers, custom experiment collectors,
   benchmark/diagnostic collectors, gradient experiments, and geometric orbit
   experiments should each get the surface they actually need.
2. Low caller burden: ordinary callers name the mathematical/search choice;
   they do not assemble coupled internal search data.
3. Mathematical contract first: f64 intervals, exact certification,
   admissibility verdicts, and smart pruned enumeration should be visible where
   they are part of the contract.
4. No fake abstraction: no wrapper, alias, options bag, or policy enum unless
   it enforces a real invariant or removes real complexity for a real consumer.
5. Simple Rust surface: prefer slices, direct `DMatrix<...>` in diagnostic
   helpers, ordinary `Vec` outputs, and flat enum arguments.
6. Performance shape: pruned and billiard enumeration must prune during
   traversal, not enumerate all cyclic subset permutations and filter them.
7. Scalar split clarity: f64 and exact/algebraic paths are distinct where their
   guarantees differ; `BigRational` is a specialization of the exact path.
8. Final-surface purity: no migration wrappers, no compatibility layers, and no
   legacy private/internal beta names.

## Public Core API

These are the polished capacity/orbit entrypoints and result types.

```rust
pub enum F64CapacitySearch {
    Hk2017Unpruned,
    Hk2017Pruned,
    Billiard,
}

pub enum ExactCapacitySearch {
    Hk2017Unpruned,
    Hk2017Pruned,
}

pub enum PredicateVerdict {
    True,
    False,
    Indeterminate,
}

pub struct F64Interval {
    pub lower: f64,
    pub upper: f64,
}

pub struct F64Orbit {
    pub sigma: Vec<usize>,
    pub beta: Vec<f64>,
    pub q: f64,
    pub q_error_bound: f64,
    pub mu: [f64; 4],
    pub xi: f64,
    pub action: F64Interval,
    pub admissible: PredicateVerdict,
}

pub struct F64CapacityResult {
    pub min_action: F64Interval,
    pub orbits: Vec<F64Orbit>,
}

impl F64CapacityResult {
    pub fn capacity(&self) -> Option<f64>;
    pub fn best_orbit(&self) -> Option<&F64Orbit>;
    pub fn best_sigma(&self) -> Option<&[usize]>;
    pub fn best_beta(&self) -> Option<&[f64]>;
}

pub struct ExactOrbit<F: OrderedField> {
    pub sigma: Vec<usize>,
    pub beta: Vec<F>,
    pub q: F,
    pub mu: [F; 4],
    pub xi: F,
    pub action: F,
}

pub struct ExactCapacityResult<F: OrderedField> {
    pub capacity: F,
    pub minimizers: Vec<ExactOrbit<F>>,
}

pub fn capacity_f64(
    dual_vertices: &[[f64; 4]],
    search: F64CapacitySearch,
    action_gap: f64,
) -> Result<F64CapacityResult, CapacityError>;

pub fn capacity_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    search: ExactCapacitySearch,
) -> Result<ExactCapacityResult<F>, CapacityError>;

pub fn solve_orbit_f64(
    dual_vertices: &[[f64; 4]],
    sigma: &[usize],
) -> Result<F64Orbit, F64OrbitSolveError>;

pub fn solve_orbit_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
) -> Result<Option<ExactOrbit<F>>, ExactOrbitSolveError>;
```

Capacity contract:

```rust
min_action.lower = min lower over admissible True or Indeterminate orbits
min_action.upper = min upper over admissible True orbits
```

The search enums name algorithm families only. Auto-routing is not a search
family. If ordinary consumers still need old convenience routing, it should be
a separate wrapper or private routing choice with a precise contract.

`capacity_exact::<BigRational>` is the rational path. There is no separate
`capacity_rational` alias unless a real consumer later needs a different
contract.

Exact capacity returns the certified exact capacity and exact minimizers. It
does not expose a gap-window result because the current real consumers do not
need one; selected exact one-sigma solves and exact gradients are the supported
experiment path.

Exact capacity does not currently expose `Billiard`. Billiard enumeration is
f64/product-classification shaped in the current code and discussion. Add exact
billiard only after it has a real exact contract.

## Public Experiment-Support API

These symbols are public because current experiments or reusable experiment
workflows need them. They are not the ordinary capacity entrypoint.

```rust
pub struct F64KktResult {
    pub beta: Vec<f64>,
    pub q: f64,
    pub q_error_bound: f64,
    pub mu: [f64; 4],
    pub xi: f64,
}

pub enum F64KktOutcome {
    Feasible(F64KktResult),
    Infeasible,
    SingularMatrix,
    ConstraintViolation,
    TypeCViolation,
}

pub fn solve_kkt_f64(
    dual_vertices: &[[f64; 4]],
    sigma: &[usize],
) -> F64KktOutcome;

pub fn solve_kkt_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
) -> Result<Option<ExactOrbit<F>>, ExactOrbitSolveError>;

pub fn hk2017_unpruned_sigmas(facet_count: usize) -> impl Iterator<Item = Vec<usize>>;

pub fn hk2017_pruned_sigmas_f64(
    dual_vertices: &[[f64; 4]],
) -> Result<impl Iterator<Item = Vec<usize>>, GeometryError>;

pub fn hk2017_transition_matrix_f64(
    dual_vertices: &[[f64; 4]],
    primal_vertices: &[[f64; 4]],
) -> Result<DMatrix<bool>, GeometryError>;

pub fn capacity_gradient_f64(
    dual_vertices: &[[f64; 4]],
    orbit: &F64Orbit,
) -> Result<Vec<[f64; 4]>, DerivativeError>;

pub fn capacity_gradient_from_kkt_f64(
    dual_vertices: &[[f64; 4]],
    sigma: &[usize],
    kkt: &F64KktResult,
) -> Result<Vec<[f64; 4]>, DerivativeError>;

pub fn capacity_gradient_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    orbit: &ExactOrbit<F>,
) -> Vec<[F; 4]>;

pub fn volume_gradient_f64(
    dual_vertices: &[[f64; 4]],
    primal_vertices: &[[f64; 4]],
) -> Vec<[f64; 4]>;

pub fn sys_gradient_f64(
    capacity: f64,
    volume: f64,
    capacity_gradient: &[[f64; 4]],
    volume_gradient: &[[f64; 4]],
) -> Vec<[f64; 4]>;

pub fn capacity_subgradients_f64(
    dual_vertices: &[[f64; 4]],
    orbits: &[F64Orbit],
) -> Result<Vec<Vec<[f64; 4]>>, DerivativeError>;

pub fn sys_subgradients_f64(
    dual_vertices: &[[f64; 4]],
    primal_vertices: &[[f64; 4]],
    capacity_result: &F64CapacityResult,
    volume: f64,
) -> Result<Vec<Vec<[f64; 4]>>, DerivativeError>;

pub fn directional_derivative_f64(
    gradient: &[[f64; 4]],
    direction: &[[f64; 4]],
) -> f64;

pub fn clarke_directional_derivative_f64(
    subdiff: &[Vec<[f64; 4]>],
    direction: &[[f64; 4]],
) -> Result<f64, DerivativeError>;

pub struct LagrangianProductFacetSplit {
    pub q_facets: Vec<usize>,
    pub p_facets: Vec<usize>,
}

pub struct BilliardError;

pub fn classify_lagrangian_product_facets_f64(
    dual_vertices: &[[f64; 4]],
) -> Result<LagrangianProductFacetSplit, BilliardError>;

pub fn billiard_sigmas_f64(
    dual_vertices: &[[f64; 4]],
) -> Result<impl Iterator<Item = Vec<usize>>, BilliardError>;

pub fn billiard_sigmas_with_bounce_count_f64(
    dual_vertices: &[[f64; 4]],
) -> Result<impl Iterator<Item = (Vec<usize>, usize)>, BilliardError>;

pub fn billiard_bounce_count(
    split: &LagrangianProductFacetSplit,
    sigma: &[usize],
) -> Option<usize>;

pub fn project_lagrangian_product_direction_f64(
    split: &LagrangianProductFacetSplit,
    direction: &[[f64; 4]],
) -> Vec<[f64; 4]>;

pub struct GeometricOrbit {
    pub breakpoints: Vec<[f64; 4]>,
    pub dwell_times: Vec<f64>,
    pub max_violation: f64,
    pub action: f64,
    pub closure_error: f64,
    pub solution_dim: usize,
    pub facet_sequence: Vec<usize>,
}

pub fn recover_geometric_orbit_from_f64(
    dual_vertices: &[[f64; 4]],
    orbit: &F64Orbit,
) -> Result<GeometricOrbit, GeometricOrbitError>;

pub fn recover_geometric_orbit_from_parts_f64(
    dual_vertices: &[[f64; 4]],
    sigma: &[usize],
    beta: &[f64],
    action: f64,
) -> Result<GeometricOrbit, GeometricOrbitError>;

pub fn beta_directional_sensitivity_f64(
    dual_vertices: &[[f64; 4]],
    sigma: &[usize],
    kkt: &F64KktResult,
    direction: &[[f64; 4]],
) -> Result<Vec<f64>, DerivativeError>;
```

Consumer notes:

- `capacity_gradient_from_kkt_f64` is kept because gradient experiments consume
  KKT solves directly. They should not construct fake `F64Orbit` values or
  re-solve.
- `F64Orbit` carries the f64 KKT payload directly because gradients and
  geometric recovery are real consumers. There is no optional multiplier layer
  in the final f64 path.
- `hk2017_unpruned_sigmas` and `hk2017_pruned_sigmas_f64` are public
  experiment-support because custom collectors and benchmarks need sigma
  streams without running the polished capacity entrypoint.
- `hk2017_transition_matrix_f64` is public experiment-support because
  datascience feature code consumes transition-density, bidirectionality, and
  out-degree statistics. It returns a plain `DMatrix<bool>`, not a wrapper.
- `billiard_sigmas_with_bounce_count_f64` exists because product experiments
  filter and persist bounce counts. The plain sigma stream remains for generic
  custom collectors that only need candidate sequences.
- The two billiard sigma iterators should share the same smart traversal. The
  plain iterator is a projection of the metadata iterator, not a second
  traversal implementation.
- `project_lagrangian_product_direction_f64` is pure and returns a fresh
  direction. In-place masking is rejected.
- The classifier name does not mention signed zero. The final public contract
  should be tolerant unless a strict signed-zero variant is actually needed.
- The facets are q/p-aligned facets of a Lagrangian product. They are not
  Lagrangian facets.
- `recover_geometric_orbit_from_parts_f64` is experiment-support, not the
  ordinary path. It is retained because visualization artifacts store
  `(sigma, beta, action)` rather than a full solved orbit payload.
- `beta_directional_sensitivity_f64` is public experiment-support for
  boundary-orbit and subdifferential numerics. Its KKT matrix assembly is an
  implementation detail, not public API.

## Public Geometry Support

These are reusable geometry utilities outside the polished capacity API. They
may belong in separate geometry modules.

```rust
pub struct Ridge2D {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

pub struct FaceLattice4D {
    pub vertex_facets: Vec<Vec<usize>>,
    pub edges: Vec<[usize; 2]>,
    pub ridges: Vec<Ridge2D>,
}

pub fn primal_vertices_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
) -> Result<Vec<[F; 4]>, GeometryError>;

pub fn vertex_facet_incidence_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    primal_vertices: &[[F; 4]],
) -> DMatrix<bool>;

pub fn face_lattice_exact<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    primal_vertices: &[[F; 4]],
    vertex_facet_incidence: &DMatrix<bool>,
) -> Result<FaceLattice4D, GeometryError>;

pub fn project_vertices_f64<F: OrderedField>(vertices: &[[F; 4]]) -> Vec<[f64; 4]>;

pub fn omega0_f64(left: &[f64; 4], right: &[f64; 4]) -> f64;
pub fn omega0_exact<F: OrderedField>(left: &[F; 4], right: &[F; 4]) -> F;

pub fn volume_f64(
    dual_vertices: &[[f64; 4]],
    primal_vertices: &[[f64; 4]],
) -> Result<f64, GeometryError>;

pub fn facet_volume_f64(
    dual_vertices: &[[f64; 4]],
    primal_vertices: &[[f64; 4]],
    facet: usize,
) -> Result<f64, GeometryError>;

pub fn facet_centroid_f64(
    primal_vertices: &[[f64; 4]],
    vertex_facet_incidence: &DMatrix<bool>,
    facet: usize,
) -> Result<[f64; 4], GeometryError>;

pub struct ReebSegment {
    pub start: [f64; 4],
    pub end: [f64; 4],
    pub facet: usize,
}

pub fn reeb_direction_f64(dual_vertex: &[f64; 4]) -> [f64; 4];

pub fn simulate_reeb_trajectory_f64(
    dual_vertices: &[[f64; 4]],
    start: [f64; 4],
    start_facet: usize,
    max_steps: usize,
) -> Result<Vec<ReebSegment>, TrajectoryError>;

pub fn regular_polygon_2d(n: usize, circumradius: f64) -> (Vec<[f64; 2]>, Vec<f64>);

pub fn rotate_polygon_2d(
    normals: &[[f64; 2]],
    heights: &[f64],
    angle: f64,
) -> (Vec<[f64; 2]>, Vec<f64>);

pub fn random_polygon_2d<R: rand::Rng>(
    n: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut R,
) -> (Vec<[f64; 2]>, Vec<f64>);

pub fn polygon_area(normals: &[[f64; 2]], heights: &[f64]) -> Option<f64>;

pub fn lagrangian_product(
    q_normals: &[[f64; 2]],
    q_heights: &[f64],
    p_normals: &[[f64; 2]],
    p_heights: &[f64],
) -> Result<Vec<[f64; 4]>, GeometryError>;

pub fn systolic_ratio(capacity: f64, volume: f64) -> f64;
```

Geometry notes:

- `volume_f64` and `facet_volume_f64` should derive topology internally or use
  a validated geometry handle. They should not require callers to pass
  correlated incidence/adjacency matrices.
- `lagrangian_product` is fallible because input polygons/heights can be
  invalid.
- `rotate_polygon_2d` returns heights unchanged because current product
  experiments pass polygon H-representations as `(normals, heights)`.
- `random_polygon_2d` keeps the height range arguments because product sweeps
  use them to control generated polygon scale and nondegeneracy.
- `FaceLattice4D` is not a polytope container. It is the reusable face-lattice
  data consumed by visualization and datascience feature code: vertex-facet
  incidence lists, edges, and 2-faces/ridges.

## Private/Internal Building Blocks

These are necessary implementation pieces, but not part of the polished public
target unless a real consumer appears.

```rust
aggregate_f64_orbits(...)
certify_f64_orbits_exact(...)
aggregate_exact_orbits(...)

kkt_augmented_system_f64(...)

facet_adjacency_f64(...)
omega_nonnegative_f64(...)
hk2017_pruned_filter(...)
billiard_filter(...)
cyclic_permutations(...)
is_feasible_cycle(...)
facet_adjacency_from_incidence(...)

billiard block enumeration
exact fallback policy variants
pruned traversal state
```

Internal contracts:

- `hk2017_pruned_sigmas_f64` and `billiard_sigmas_f64` must prune during
  traversal. They must not generate every cyclic subset permutation and then
  filter.
- HK2017 transition graphs keep edges whose omega predicate verdict is `True`
  or `Indeterminate` and prune only `False`.
- Exact certification is minima-safe in the polished capacity wrappers. Cheaper
  or exhaustive variants are profiling/internal unless a real experiment needs
  them as named diagnostics.

## Compared Alternatives

Capacity entrypoints:

- Rejected: separate public function for every scalar/search combination, such
  as `capacity_hk2017_pruned_f64` and `capacity_billiard_exact`.
- Rejected: a single iterator-only capacity function as the polished entrypoint.
- Rejected: one shared `CapacitySearch` containing `Billiard` for both f64 and
  exact capacity.
- Chosen: `capacity_f64` with `F64CapacitySearch`, and `capacity_exact` with
  `ExactCapacitySearch`. Exact billiard is not exposed without an exact
  contract.

Search data exposure:

- Rejected: one-field wrappers or aliases for adjacency/transition matrices.
- Rejected: requiring ordinary callers to assemble facet adjacency,
  omega-nonnegative verdicts, and transition graphs.
- Chosen: public ordinary paths derive search data from `dual_vertices`.
  Experiment-support helpers expose direct `DMatrix<...>` only where real
  consumers need matrix-valued diagnostics.
- Chosen for datascience features: `hk2017_transition_matrix_f64` exposes the
  direct transition matrix. Lower-level filters such as `is_feasible_cycle`
  remain private because custom collectors can use `hk2017_pruned_sigmas_f64`
  instead.

Billiard enumeration:

- Rejected: `billiard_sigmas(split, facet_adjacency, transitions)` as the normal
  public enumerator. It burdens callers with coupled derived data.
- Rejected: generate all cyclic subset permutations and filter with
  `billiard_filter`.
- Chosen: `billiard_sigmas_f64(dual_vertices)` derives the q/p split, facet
  adjacency, and HK2017 transition graph internally, then does smart traversal.
- Chosen for product experiments: `billiard_sigmas_with_bounce_count_f64`
  returns `(sigma, bounce_count)` because those callers immediately filter or
  persist bounce counts. This avoids re-classifying the same product for every
  sigma while not exposing block enumeration.

Current-consumer support:

- Rejected: shrinking the target to only ordinary capacity callers. That would
  drop visualization, gradient, subdifferential, and datascience consumers.
- Rejected: exposing every current module path as final public API. That would
  preserve the current implementation layout and import old technical debt.
- Chosen: keep polished capacity entrypoints small, then expose experiment
  support for the real reusable operations: sigma streams, one-sigma KKT
  solves, KKT-derived gradients, beta directional sensitivity, geometric orbit
  recovery, billiard bounce metadata, face-lattice geometry, product helpers,
  and scalar/volume helpers.

Geometric orbit recovery:

- Rejected: only accepting a full f64 orbit payload. Visualization currently
  stores `(sigma, beta, action)` and should not have to fake missing KKT fields.
- Rejected: promoting raw parts to the ordinary capacity path.
- Chosen: `recover_geometric_orbit_from_f64` for solved-orbit callers and
  `recover_geometric_orbit_from_parts_f64` for visualization/export workflows.

Exact capacity result breadth:

- Rejected: core `ExactCapacityResult { minimizers, orbit_window, action_gap }`.
  Current exact consumers use selected one-sigma exact solves, exact gradients,
  and certified minimizer profiling; no real caller needs a public exact
  gap-window result.
- Chosen: core `capacity_exact` returns exact capacity plus exact minimizers.
  Gap-window certification stays private/internal until an experiment-support
  caller exists.

Exact billiard:

- Rejected for now: `ExactCapacitySearch::Billiard`. Grep found no exact
  consumer for billiard-specific enumeration or bounce metadata.
- Chosen: exact consumers use selected exact one-sigma solves, exact gradients,
  and exact HK2017-style capacity/minimizer certification.

F64 capacity result conveniences:

- Rejected: invented `upper_bound_orbit`, `upper_bound_sigma`, and
  `upper_bound_value` names. Real callers overwhelmingly use scalar capacity,
  best orbit, best sigma, and best beta accessors.
- Chosen: `capacity`, `best_orbit`, `best_sigma`, and `best_beta` as
  conveniences over the explicit `min_action` interval and orbit list.

F64 interval representation:

- Rejected: checked constructor for the target draft. `F64Interval` is library
  output, not an input contract for ordinary callers, and the implementation can
  keep aggregation construction inside the module.
- Chosen: public fields for result readability. Functions that produce
  intervals must maintain `lower <= upper`; functions that accept user-built
  orbit/result payloads validate only the invariants they actually consume.

## Rejected Shapes

```rust
capacity_from_monolithic_polytope(...)
capacity_f64_from_monolithic_polytope(...)
capacity_exact_from_monolithic_polytope(...)
ehz_capacity(...)
ehz_capacity_pruned(...)
ehz_capacity_unpruned(...)
ehz_capacity_billiard(...)
RationalCapacityOptions
ExactCapacityOptions
F64KktOptions
OrbitCertificationPolicy
F64CapacityValue
visitor-only sigma enumeration
billiard_sigmas(split, facet_adjacency, transitions)
billiard_blocks(...)
BilliardBlock
capacity_gradient_f64_raw(...)
mask_lagrangian_direction_in_place(...)
OrbitGradientF64 = Vec<[f64; 4]>
ClarkeSubdiffF64 = Vec<OrbitGradientF64>
ReebTrajectory { segments: Vec<ReebSegment> }
FacetAdjacency wrapper or alias
TransitionGraph wrapper or alias
OmegaNonnegative wrapper or alias
FacetTopology
```

## Acceptance Status

No known technical open checks remain in this draft. Remaining status is Joern
review/acceptance.

Resolved during this pass:

- Exact capacity uses `ExactCapacitySearch` without `Billiard`.
- HK sigma streams are public experiment support; matrix/filter/combinatorics
  helpers remain private/internal unless a real diagnostic consumer justifies
  direct exposure.
- `F64Orbit` now carries KKT data directly, so f64 gradients and geometric
  recovery are not impossible by signature.
- Raw geometric recovery from `(sigma, beta, action)` is retained as
  experiment-support because visualization has a real stored-parts consumer.
- Exact capacity no longer has `orbit_window` or `action_gap` in the core
  result. Grep found selected exact one-sigma and exact-gradient consumers, and
  certified minimizer profiling, but no real gap-window consumer.
- Exact billiard remains absent because grep found no exact billiard consumer.
- `FacetTopology` was deleted; direct matrix outputs remain only where a real
  support consumer exists.
- `kkt_augmented_system_f64` was moved out of public support. The public
  boundary is `beta_directional_sensitivity_f64`.
- `F64Interval` stays a plain result struct.
- F64 result convenience methods now match real scalar/best-orbit callers.
