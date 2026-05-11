<!--
Purpose: migration roadmap for a durable Euclidean convex-polytope crate.
Context: ordinary convex geometry currently lives inside symplectic-facing
modules, which makes non-symplectic helpers harder to reuse and review.
-->

# Euclidean Polytopes Roadmap

## Status

- State: merged architecture; active follow-up routing.
- Last updated: 2026-05-11.
- Source surfaces: `crates/euclidean-polytopes/`,
  `crates/symplectic/src/geom/`, `crates/MAP.md`, `tasks/rust-tech-debt.md`.
- Refresh when: a geometry helper moves between crates, a public API decision
  is accepted/rejected, or a retained experiment starts using the new crate.

## Steering Cache

- [Jorn request 2026-05-10] Create a nice crate for Euclidean, not symplectic,
  polytopes. Start with the crate scaffold, `README.md`, `DEVELOPMENT.md`, and
  a migration task with done criteria. Prefer flat function style over aliases:
  callers should pass ordinary `Vec<Vector4<T>>` / `&[Vector4<T>]` values and
  own context-specific contracts such as `0 in int conv(a)`.
  Why it matters: this keeps the API close to the math and avoids another
  wrapper-heavy crate while refactoring existing geometry users.
- [agent synthesis 2026-05-10] The reusable Euclidean operations are:
  origin-interior test for point sets, V-representation non-redundancy, polar
  vertex enumeration, full-dimensional volume from dual/primal vertices, and
  affine-subspace polygon/volume helpers in ambient `R^4`.
  Why it matters: these are the operations that repeatedly appear underneath
  symplectic algorithms, exact validation, and experiment geometry setup.
- [Jorn correction 2026-05-10] The crate needs both `f64 -> f64 or
  indeterminate` APIs and `exact -> exact` APIs. Exact implementations may use
  the `f64` pathway as a cheap filter, but every indeterminate case must be
  decided by a slow exact calculation before returning.
  Why it matters: vertex enumeration and incidence tests have cheap common
  floating paths, but near-singular 4-tuples, duplicate candidates, and
  halfspace-boundary cases must not be guessed by tolerance.
- [Jorn API guidance 2026-05-10] Avoid premature abstractions. Prefer flat
  standard types with semantic names, such as `x` and `x_abs_error_bound`, over
  wrappers that fresh agents must learn. Use `_exact` and `_f64` suffixes when
  both pathways exist. Use `Result` for recoverable errors, `Option` only for
  exact mathematical `None`/`Some` distinctions, panics for irrecoverable
  contract or invariant violations, tuples when positions are obvious, and
  local flat structs when output variables need names.
  Why it matters: the crate should map to the math at call sites without
  wrapping/unwrapping overhead or hidden positional input contracts.
- [Jorn decisions 2026-05-10] f64 volume should be implemented before exact
  volume, but exact volume is a real eventual need and not a YAGNI violation.
  Affine-subspace volume follows naturally from the internal volume
  implementation, especially for 3-faces of a 4-polytope, so it should be
  shaped by that decomposition rather than only by external consumers. Exact
  predicates return `bool`; f64 predicates return diagnostic true/false/
  indeterminate outputs, for example candidate sets of five vertices that may
  contain zero.
  Why it matters: future agents should not cut exact volume from the target,
  over-index on consumer-facing API shape for affine volume, or make exact
  predicates return diagnostic wrappers.
- [Jorn API nit 2026-05-10] Exact incidence can probably be stored as
  `DMatrix<bool>` or `Vec<Vec<bool>>`. Approximate incidence may need a flat
  relation list if each relation is true, false, or indeterminate with
  diagnostics.
  Why it matters: do not overfit exact and approximate incidence to the same
  storage shape when their semantics differ.
- [Jorn decisions 2026-05-10] Exact volume over field `T` should stay in `T`;
  no field extensions are expected for exact polytope volume. f64 origin
  interior diagnostics should list all candidate 5-sets that may contain zero,
  because proving `false` needs ruling them all out. Code should use
  single-concern files/modules such as `volume.rs`, with exact and f64 variants
  colocated when that makes comparison and maintenance easier.
  Why it matters: these choices prevent future agents from weakening exact
  volume, truncating f64 diagnostics prematurely, or splitting code by numeric
  representation before it improves readability.
- [agent synthesis 2026-05-10] `polar_vertices_exact(vertices)` needs
  `0 in int conv(vertices)` for the normalized polar to be bounded and
  full-dimensional. It does not need `vertices` to be non-redundant to compute
  the polar vertices; redundant input points become redundant polar
  inequalities. Non-redundancy remains a separate check for callers that need
  every input point to be an extremum or every input to define a non-redundant
  polar facet.
  Why it matters: the first implementation should check/assert the interior
  condition but should not overconstrain polar vertex enumeration with an
  unnecessary non-redundancy precondition.
- [implemented 2026-05-10] `polar_vertices_exact(vertices)` now returns
  `(vertices, vertex_facet_incidence)`. The earlier public f64 diagnostic polar
  sketch was removed after API review found no current non-test caller. The
  former `vertex_adjacency` name for the facet-pair matrix was misleading: the
  matrix is now `facet_intersection_is_nonempty`, meaning two facets share at
  least one vertex, not necessarily a 2-face.
  Why it matters: the public names now expose the actual incidence semantics,
  which reduces caller mistakes in capacity and combinatorics code.
- [accepted 2026-05-10] Use TDD for the first implementation slice. Write
  contract tests for exact fixtures before implementing exact helpers, and
  write f64 indeterminate/tolerance-boundary tests before implementing f64
  filtering.
  Why it matters: this crate exists to prevent tolerance guesses and hidden
  geometry-contract drift; tests should fail before those mistakes can become
  accepted implementation behavior.
- [Jorn reminder 2026-05-10] Workspace property-testing crates are available.
  Use them after fixture contract tests are in place for generated invariants
  such as polarity roundtrips, exact/f64 agreement on well-conditioned inputs,
  and volume scaling.
  Why it matters: property tests are useful here, but they should strengthen
  rather than replace the explicit fixture tests that define the first API
  contracts.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Crate scaffold and API target | `[merged]` | mainline thesis | agents | Keep `README.md` and `DEVELOPMENT.md` synchronized when the public API changes. The flat-input/no-public-wrapper direction is accepted for the merged crate. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Robust floating/exact architecture | `[implemented first architecture]` | mainline thesis | agents | Exact APIs return exact answers; f64 APIs expose diagnostics or known-incidence f64 computations. Add new f64 diagnostic return shapes only with a real caller and explicit indeterminate semantics. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Polar vertex enumeration plus validation dependencies | `[implemented exact slice]` | mainline thesis | agents | Exact polar enumeration and exact origin-in-interior are implemented. The earlier public f64 diagnostic polar sketch was removed because no current non-test caller needed it. | `crates/euclidean-polytopes/src/polar.rs`, `crates/euclidean-polytopes/src/predicates.rs`, `crates/euclidean-polytopes/tests/polar_vertices.rs` |
| Polar vertex enumeration performance | `[implemented first optimization]` | mainline thesis | agents, Jörn only for further API/contract changes | Branch `tracing-perf-integration` added opt-in tracing, optimized the checked origin-interior predicate with an exact-verified f64-guided simplex certificate, then promoted the winning polar spike: BigRational integer-scaled determinant/gap enumeration. Random F=8 pruned HK2017 profile improved from exact geometry avg `40.407ms` and total avg `42.968ms` to exact geometry avg `4.670ms` and total avg `7.141ms` over 20 deterministic checked samples, with deterministic outputs unchanged. Remaining follow-ups are only if larger-F or heterogeneous-denominator profiles show new bottlenecks. | `/tmp/msc-math-tracing-perf-2026-05-11/spike-comparison.md`, `/tmp/msc-math-tracing-perf-2026-05-11/origin-optimized-f8-s20-trace.jsonl`, `crates/symplectic/src/bin/profile_pruned_hk2017.rs`, `crates/euclidean-polytopes/src/polar.rs`, `crates/euclidean-polytopes/src/predicates.rs` |
| Extreme-point / non-redundant point-set predicate | `[implemented exact slice]` | mainline thesis | agents | Exact predicate is implemented and covered by fixture/property tests. Add the matching f64 diagnostic later only if its return shape stays flat and useful. | `crates/euclidean-polytopes/src/predicates.rs`, `crates/euclidean-polytopes/tests/extreme_points.rs`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Full-dimensional known-incidence f64 volume | `[implemented migration slice]` | mainline thesis | agents | `volume_from_incidence_f64(vertices, incidence)` is implemented for callers that already have reliable incidence. A dual-vertices-plus-f64-vertices incidence-recovery API is not currently public; add one only with a real caller and explicit indeterminate semantics. | `crates/euclidean-polytopes/src/volume.rs`, `crates/euclidean-polytopes/tests/volume.rs`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Verification property suite | `[implemented first property slice]` | mainline thesis | agents | Keep strengthening theorem-shaped property tests as APIs migrate. Current coverage includes exact polar soundness, exact polarity roundtrip, generated non-redundancy witnesses, exact/f64 known-incidence volume agreement, and f64 volume scaling/permutation invariants. Public f64 diagnostic polar properties are future work only if that API returns. | `crates/euclidean-polytopes/DEVELOPMENT.md`, `crates/euclidean-polytopes/tests/` |
| Known-incidence volume integration | `[implemented migration slice]` | mainline thesis | agents | `volume_from_incidence_f64(vertices, incidence)` is implemented. This was the first symplectic volume migration target before the exact API became the source of truth; keep using known-incidence helpers instead of recomputing incidence through f64 signed gaps. | `crates/euclidean-polytopes/src/volume.rs`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Known-incidence facet 3-volume and centroid | `[implemented migration slice]` | mainline thesis | agents | `facet_volume_from_incidence_f64` and `facet_volume_and_centroid_from_incidence_f64` are implemented in `euclidean-polytopes`. `symplectic::geom::facet_volume` reexports these flat helpers, and `volume_derivatives_a` now takes exact incidence explicitly instead of retesting f64 facet membership. | `crates/euclidean-polytopes/src/volume.rs`, `crates/symplectic/src/geom/facet_volume.rs`, `crates/symplectic/src/derivatives.rs` |
| Incidence-only face combinatorics | `[implemented migration slice]` | mainline thesis | agents | `vertex_facets_from_vertex_facet_incidence`, `facet_vertices_from_vertex_facet_incidence`, `edges_from_vertex_facet_incidence`, `two_faces_from_vertex_facet_incidence`, and `facet_intersection_is_nonempty_from_vertex_facet_incidence` are public Euclidean helpers over `DMatrix<bool>`. Shared symplectic skeleton wrappers were deleted; remaining dataset code uses local feature names. | `crates/euclidean-polytopes/src/faces.rs`, `crates/euclidean-polytopes/tests/faces.rs`, `experiments/sys-landscape/datascience/tables/features_skeleton.rs` |
| Incidence-only 2-face ordering | `[implemented migration slice]` | mainline thesis | agents | Private `order_2face_vertices_from_incidence` orders `facet_i ∩ facet_j` using only the incidence matrix, with empty/1/2-vertex intersections skipped by callers and `>=3` intersections asserted to be a single cycle. Known-incidence f64 volume and facet-volume paths use it before exact volume. | `crates/euclidean-polytopes/DEVELOPMENT.md`, `crates/euclidean-polytopes/src/volume.rs`, `crates/euclidean-polytopes/tests/volume.rs` |
| Full-dimensional exact volume | `[implemented migration slice]` | mainline thesis | agents | `volume_from_incidence_exact(vertices, incidence) -> T` uses incidence-only 2-face ordering and exact 4-simplex determinant sums. It does not take dual vertices or use f64 in the exact computation. | `crates/euclidean-polytopes/DEVELOPMENT.md`, `crates/euclidean-polytopes/src/volume.rs`, `crates/euclidean-polytopes/tests/volume.rs` |
| Random dual-vertex candidate sampling | `[implemented migration slice]` | mainline thesis | agents | `sample_random_dual_vertices_f64(facet_count, h_min, h_max, rng)` samples candidate normalized dual vertices in `euclidean-polytopes`. `symplectic::random` keeps rejection sampling and master-seed/attempt derivation, validating candidates through the flat rational vertex-enumeration pipeline. | `crates/euclidean-polytopes/src/random.rs`, `crates/euclidean-polytopes/tests/random.rs`, `crates/symplectic/src/random.rs` |
| Symplectic exact/f64 volume API | `[deleted after migration]` | mainline thesis | agents | The public `symplectic::geom::volume` module was removed after callers migrated to `euclidean_polytopes::volume_from_incidence_exact` plus local exact-to-f64 helpers. | `crates/euclidean-polytopes/DEVELOPMENT.md`, `crates/symplectic/src/derivatives.rs`, `experiments/*/src/lib.rs` |
| Affine-subspace polygons and volume | `[future]` | mainline thesis | agents, Jorn only if generic-vs-specific API affects thesis callers | Let the internal Euclidean volume decomposition determine the first affine-subspace helper shape when a thesis caller needs it. | `crates/euclidean-polytopes/src/volume.rs`, `crates/symplectic/src/geom/polygon.rs` |
| Symplectic integration cleanup | `[merged]` | mainline thesis | agents | `symplectic` now keeps symplectic form, capacity, KKT, omega signs, Reeb-direction transition pruning, derivatives, persistence, and fixture/workflow code. Reusable ordinary geometry lives in `euclidean-polytopes` except thin namespace reexports. | `crates/symplectic/src/geom/`, `crates/symplectic/src/algorithms/` |
| Flat capacity/orbit internals | `[implemented migration slice]` | mainline thesis | agents | HK2017 enumeration, saddle-point KKT solving, orbit solving, and orbit aggregation now have flat helper entry points over facet count, transition matrices, f64 dual vertices, and exact dual vertices. Root capacity wrappers assemble flat data once and delegate to those helpers. | `crates/symplectic/src/algorithms/hk2017/enumeration.rs`, `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/kkt/saddle_point_solver.rs`, `crates/symplectic/src/lib.rs` |
| Flat KKT assembly boundary | `[implemented migration slice]` | mainline thesis | agents | Old shared-wrapper `build_qp` and `build_augmented_system` compatibility entry points were deleted. KKT assembly callers now pass the ordered dual-vertex slice explicitly to `build_qp_from_dual_vertices` and `build_augmented_system_from_dual_vertices`. | `crates/symplectic/src/kkt/qp_assembly.rs`, `crates/symplectic/src/kkt/test_saddle_point_solver.rs`, `crates/symplectic/src/kkt/projection_solver.rs` |
| Flat transition and HK2017 enumeration boundary | `[implemented migration slice]` | mainline thesis | agents | `build_transition_matrix(polytope)`, `for_each_sigma_unpruned(polytope, ...)`, and `for_each_sigma_pruned(polytope, ...)` wrappers were deleted. Callers now build transition matrices from explicit facet-intersection and omega-sign matrices, or enumerate from a facet count. | `crates/symplectic/src/algorithms/facet_adjacency.rs`, `crates/symplectic/src/algorithms/hk2017/enumeration.rs`, `crates/symplectic/src/lib.rs` |
| Flat KKT solve boundary | `[implemented migration slice]` | mainline thesis | agents | `solve_kkt_for(polytope, perm)` was deleted. Callers now pass the ordered dual-vertex slice explicitly to `solve_kkt_for_dual_vertices`, including derivative finite-difference paths and experiment ascent loops. | `crates/symplectic/src/kkt/saddle_point_solver.rs`, `crates/symplectic/src/derivatives.rs`, `experiments/numerics/gradient/src/lib.rs` |
| Flat orbit/result boundary | `[implemented migration slice]` | mainline thesis | agents | Old shared-wrapper `solve_orbit_sigma`, `solve_sigma_stream`, `aggregate_orbits`, and `aggregate_certified_orbits` entry points were deleted. Callers now pass f64 dual vertices for orbit solving and exact dual vertices for exact fallback aggregation. | `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/algorithms/mod.rs`, `experiments/verification/all-minimum/main.rs` |

## Merge Evidence

The architecture branch merged into `main` at `b90e92b2` on 2026-05-11 after
these criteria were checked:

- `crates/euclidean-polytopes/` has consumer and maintainer docs matching the
  implemented public API;
- exact origin-interior, exact extreme-point, exact polar, known-incidence f64
  volume, known-incidence exact volume, facet-volume, incidence-derived face
  helpers, and random dual-vertex sampling are covered by fixture or property
  tests;
- property-test comments separate theorem-shaped propositions from the actual
  operationalization: generator, precondition/discard rule, case count,
  fixtures, and f64 tolerance;
- symplectic and experiment volume/facet-volume callers delegate to
  `euclidean-polytopes` through known-incidence helpers rather than recomputing
  incidence from f64 signed gaps;
- `symplectic` no longer owns reusable ordinary convex-geometry algorithms
  except thin namespace reexports or symplectic-specific fixture/workflow code;
- shared `Polytope4D` and old KKT/orbit/transition compatibility entry points
  are deleted from live source, with remaining mentions only in historical or
  rejection docs;
- existing symplectic behavior is preserved by tests or explicit migration
  notes;
- exact predicate APIs return `bool`, with diagnostics confined to f64 helper
  APIs or error types;
- API review has confirmed that no alias, wrapper, trait, or generic result
  abstraction was added without repeated current callers or a clear
  simplification;
- `crates/MAP.md`, this task, and affected crate docs are updated;
- final branch checks pass:
  `cargo test -p euclidean-polytopes`,
  `cargo clippy -p euclidean-polytopes --all-targets -- -D warnings`,
  `cargo test -p symplectic --release --lib`,
  `cargo clippy -p symplectic --all-targets -- -D warnings`,
  and `cargo check --workspace`.

Deferred follow-ups:

- affine-subspace polygon and volume helpers in ambient `R^4`;
- polar vertex enumeration performance optimization after the architecture
  branch merge;
- f64 diagnostic extreme-point or incidence-recovery APIs without a current
  caller;
- stronger cleanup of deep expert-public symplectic import paths;
- standardizing experiment-local cache contracts beyond the migrations needed
  to delete shared wrappers.

## Agent Cache

- [implemented 2026-05-10] First flat symplectic assembly slice added
  transition-matrix and KKT/QP helpers over ordinary matrices, dual vertices,
  and permutations.
  Why it matters: capacity-facing code can now migrate call sites away from
  shared geometry wrapper without changing the transition or KKT semantics in
  the same review packet.
- [implemented 2026-05-10] Flat capacity/orbit internals now include
  `for_each_sigma_unpruned_facet_count`,
  `for_each_sigma_pruned_by_transition`,
  `solve_kkt_for_dual_vertices`,
  `solve_orbit_sigma_saddle_point`,
  `solve_sigma_stream_with_dual_vertices`, and flat exact-dual aggregation
  helpers. Exact fallback helpers require the same ordered facet set as the
  f64 candidate generator.
  Why it matters: flat solver semantics were established before later slices
  migrated call sites and deleted compatibility wrappers.
- [implemented 2026-05-10] Flat KKT assembly migration deleted
  `build_qp(polytope, perm)` and `build_augmented_system(polytope, perm)`.
  The remaining assembly API takes dual vertices directly, so callers must make
  the ordered facet set explicit before constructing QP/KKT matrices.
  Why it matters: the KKT matrix boundary now matches the math-shaped data
  dependency and cannot silently re-enter a shared geometry wrapper.
- [implemented 2026-05-10] Flat transition/enumeration migration deleted
  `build_transition_matrix(polytope)`, `for_each_sigma_pruned(polytope, ...)`,
  and `for_each_sigma_unpruned(polytope, ...)`. The remaining transition API
  takes `(facet_intersection_is_nonempty, omega_signs)`, and HK2017 enumeration
  takes either a flat facet count or a prebuilt transition matrix.
  Why it matters: pruning now exposes the exact data dependency at the call
  site instead of hiding Euclidean incidence plus symplectic omega signs behind
  a shared geometry wrapper.
- [implemented 2026-05-10] Flat KKT solve migration deleted
  `solve_kkt_for(polytope, perm)`. Derivative tests, ascent experiments, and
  KKT regression tests now call `solve_kkt_for_dual_vertices` with the
  dual-vertex slice from the same polytope that produced the permutation.
  Why it matters: saddle-point solving is now a flat dual-vertex operation, so
  capacity orchestration is no longer a hidden KKT matrix dependency.
- [implemented 2026-05-10] Flat orbit/result migration deleted
  `solve_orbit_sigma`, `solve_sigma_stream`, `aggregate_orbits`, and
  `aggregate_certified_orbits` shared-wrapper entry points. Public expert reexports
  now expose the flat dual-vertex variants. Aggregation call sites bind exact
  dual vertices from the same ordered facet set as the f64 orbit candidates.
  Why it matters: exact fallback contracts are now visible at call sites, and
  the remaining root `ehz_capacity*` wrappers are intentional user-facing
  capacity frontends instead of hidden orbit/result compatibility APIs.
- [implemented 2026-05-11] The shared `Polytope4D` wrapper was deleted.
  Ordinary geometry is now passed as flat dual vertices, primal vertices,
  vertex-facet incidence, and facet-intersection matrices. Omega signs and
  capacity-facing transition pruning remain symplectic.
- [fresh 2026-05-10] Existing full-dimensional volume triangulates facets from
  their centroid and cones to the origin. That relies on normalized
  H-representation `a_i . x <= 1`, hence `0` is inside every valid full
  polytope.
- [fresh 2026-05-10] The lower-dimensional volume requirement is underspecified
  enough that a generic `affine_volume` API may still be premature. Do not
  treat it as merely consumer-driven: the existing volume implementation itself
  creates pressure for 3-face and affine-polygon measures.
- [fresh 2026-05-10] Existing `symplectic` vertex enumeration already uses an
  `f64` prefilter before exact rational checks. The new architecture should make
  that pattern explicit: approximate callers see error bounds or indeterminate
  candidates, exact callers get a final exact answer after fallback.
- [implemented 2026-05-10] The first `euclidean-polytopes` slice now exposes
  exact origin-interior and exact polar vertex enumeration over
  `Vector4<T: ExactScalar>`. The exact polar path checks `0 in int conv`, accepts
  redundant input inequalities, returns exact incidence, and deduplicates by
  exact equality. The initial `f64` polar path is diagnostic-only: finite input
  validation plus partial vertices and indeterminate 4-tuples.
- [implemented 2026-05-10] The exact extreme-point slice exposes
  `all_points_are_extreme_exact(points)`. It decides V-representation
  non-redundancy in ambient `R^4`, including lower-dimensional point sets, by
  exact Caratheodory/barycentric witness enumeration. Duplicate exact points
  return `false`. The f64 diagnostic variant is still future work.
- [replaced 2026-05-11] The earlier f64 full-dimensional volume sketch with
  dual-vertex incidence recovery was not kept as public API. The current public
  f64 volume surface is `volume_from_incidence_f64(vertices, incidence)` for
  callers that already have reliable incidence. Reintroduce f64 incidence
  recovery only with a real caller and explicit indeterminate semantics.
- [accepted 2026-05-10] Good property-test comments should separate the
  universal proposition from the operationalization. For example, state
  "for all `P`, if `0 in int conv(P)`, then polar output is feasible and
  incidence is exact" separately from "generate rational points in
  `[-3, 3]^4`, discard with `prop_assume!`, and run 32 cases." Use
  constructed generators when the precondition is too rare for discard-based
  testing.
- [implemented 2026-05-10] The first verification-property slice adds
  theorem-shaped integration tests for exact polar soundness, exact polarity
  roundtrip with redundant edge points, generated exact non-redundancy positive
  and negative cases, exact/f64 known-incidence volume agreement, and f64
  volume rational-scaling/permutation invariants. At that point, the symplectic
  migration regression was deferred until the caller migration packet
  introduced the `symplectic` -> `euclidean-polytopes` dependency.
- [agent synthesis 2026-05-10] The first symplectic volume migration should use
  known exact incidence, not f64 incidence recovery.
  Why it matters: f64 incidence recovery is designed for callers that only have
  dual and primal vertices in f64; recomputing stored exact incidence through
  f64 signed gaps would weaken the old path.
- [implemented 2026-05-10] Known-incidence volume integration exposes
  `volume_from_incidence_f64(vertices, incidence)` in `euclidean-polytopes`.
  The first symplectic migration routed f64 volume through it with
  flat fixture data; the later exact migration replaced that as the source of
  truth. The final cleanup removed public qhull volume API and symplectic qhull
  benchmarking.
  Why it matters: ordinary volume triangulation now has one reusable home, and
  exact incidence is no longer weakened through f64 recovery.
- [agent synthesis 2026-05-10] The next ordinary-geometry migration should move
  facet 3-volume and centroid computation behind known-incidence Euclidean
  helpers. The exact full-volume slice is real but has a separate design risk:
  summing exact determinants is easy, while exact 2-face polygon ordering should
  not be guessed through an unreviewed f64 ordering shortcut.
  Why it matters: facet 3-volume immediately improves derivative code and
  advances affine-subspace volume pressure without locking in the exact
  2-face ordering strategy.
- [implemented 2026-05-10] Known-incidence facet 3-volume and centroid
  integration exposes `facet_volume_from_incidence_f64(vertices, incidence,
  facet_index)` and `facet_volume_and_centroid_from_incidence_f64(vertices,
  incidence, facet_index)` in `euclidean-polytopes`. Symplectic facet-volume
  reexports and `volume_derivatives_a` now use exact incidence supplied by the
  caller.
  Why it matters: derivative code no longer weakens exact incidence by
  retesting vertex-facet membership through f64 signed gaps.
- [implemented 2026-05-10] Ordinary random candidate dual-vertex sampling moved
  to `euclidean-polytopes` as `sample_random_dual_vertices_f64`. The function
  returns only `Vec<Vector4<f64>>` candidates and asserts the sampling contract.
  `symplectic::random` still owns validation/rejection and deterministic seed
  derivation for generated attempts.
  Why it matters: Euclidean sampling is reusable without making the Euclidean
  crate own symplectic validation or experiment-facing rejection loops.
- [Jorn terminology 2026-05-10] Prefer `2-face` over `ridge` in new public
  docs, APIs, and task text. Old internal code may keep existing terminology
  until touched, but new Euclidean-facing helper names should use `2face` or
  `2-face` depending on Rust identifier versus prose context.
  Why it matters: future agents should not introduce avoidable terminology
  drift while building the exact-volume prerequisites.
- [implemented 2026-05-10] Public incidence-only face combinatorics moved to
  `euclidean-polytopes` as `vertex_facets_from_vertex_facet_incidence`,
  `facet_vertices_from_vertex_facet_incidence`,
  `edges_from_vertex_facet_incidence`, `two_faces_from_vertex_facet_incidence`,
  and `facet_intersection_is_nonempty_from_vertex_facet_incidence`. `TwoFace`
  records a sorted facet pair and sorted vertex-index list, but does not
  promise polygon order. The old shared symplectic skeleton wrapper was deleted;
  remaining dataset code keeps local skeleton/2-face feature names where needed
  for persisted outputs.
  Why it matters: ordinary face combinatorics now has one reusable Euclidean
  owner, and no shared compatibility wrapper is needed for current callers.
- [implemented 2026-05-10] Incidence-only 2-face ordering is implemented as a
  private Euclidean volume helper. Known-incidence full volume, recovered f64
  volume after incidence has been decided, and known-incidence facet 3-volume
  now order polygonal 2-faces from `DMatrix<bool>` incidence instead of f64
  coordinate angle sorting.
  Why it matters: the exact-volume slice can reuse the same combinatorial
  ordering boundary without adding coordinate degeneracy detection.
- [accepted 2026-05-10] Exact full-dimensional volume should use determinant
  triangulation, not lower-dimensional Euclidean area times distance. The API
  does not need dual vertices when incidence is known.
  Why it matters: exact volume stays in the scalar field `T` and avoids
  square-root intermediate quantities.
- [implemented 2026-05-10] Exact full-dimensional known-incidence volume
  exposes `volume_from_incidence_exact(vertices, incidence) -> T` in
  `euclidean-polytopes`. It reuses incidence-only 2-face ordering, computes
  exact facet centroids, and sums `abs(det) / 24` over origin-coned
  4-simplices.
  Why it matters: exact volume now has the same flat known-incidence surface as
  the f64 migration helper, without recomputing incidence or introducing f64
  arithmetic.
- [implemented 2026-05-10] The first symplectic volume cleanup made the old
  f64 volume path an exact-to-f64 projection of exact known-incidence volume.
  Both paths delegated to `euclidean_polytopes::volume_from_incidence_exact`
  through exact vertices and exact vertex-facet incidence.
  Why it matters: this created the exact known-incidence boundary needed before
  deleting ordinary volume from `symplectic`.
- [implemented 2026-05-10] Legacy ordinary-geometry surface was pruned after
  migration. Removed public ordinary-volume wrappers, `simplex_volume_5`, raw
  facet slice helpers, and facet-volume tolerance constants from `symplectic`.
  Refactored repo callers to flat known-incidence volume and facet-volume
  helpers.
  Why it matters: new agents now see one source of truth for reusable Euclidean
  geometry and explicit `_f64` naming at symplectic call sites.
- [implemented 2026-05-10] The public `symplectic::geom::volume` module and
  root `symplectic::volume_f64` reexport were deleted. Symplectic derivative
  code and experiment packages now call
  `euclidean_polytopes::volume_from_incidence_exact` through private or
  package-local exact-to-f64 helpers. The symplectic qhull wrapper is no longer
  public and is compiled only for tests.
  Why it matters: ordinary Euclidean volume now has one reusable crate owner,
  while `symplectic` keeps symplectic form, capacity, KKT, Reeb, derivative,
  and polytope-construction responsibilities.

## Pruned / Stale

- `symplectic::geom::volume::volume`, public
  `symplectic::geom::volume::simplex_volume_5`,
  `symplectic::geom::volume::{volume_exact, volume_f64, volume_qhull}`,
  root `symplectic::volume_f64`,
  `symplectic::geom::facet_volume::{facet_volume_3d_raw,
  facet_volume_and_centroid_3d_raw}`, and the facet-volume local f64 incidence
  recovery helpers were removed on 2026-05-10 after consumers were migrated.
