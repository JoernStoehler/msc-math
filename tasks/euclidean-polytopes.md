<!--
Purpose: migration roadmap for a durable Euclidean convex-polytope crate.
Context: ordinary convex geometry currently lives inside symplectic-facing
modules, which makes non-symplectic helpers harder to reuse and review.
-->

# Euclidean Polytopes Roadmap

## Status

- State: active.
- Last updated: 2026-05-10.
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
| Crate scaffold and API target | `[active]` | mainline thesis | agents, Jorn for API taste close calls | Review `README.md` and `DEVELOPMENT.md`; decide whether the flat-input/no-public-wrapper direction is accepted for the first migration. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Robust floating/exact architecture | `[active]` | mainline thesis | agents | Define flat approximate return shapes with semantic names, `_abs_error_bound` fields, and operation-specific indeterminate diagnostics. Exact predicates return `bool` and may resolve f64 indeterminate diagnostic candidates exactly. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Polar vertex enumeration plus validation dependencies | `[implemented first slice]` | mainline thesis | agents | Review and integrate current `euclidean-polytopes` public API into callers when a migration packet needs it. Exact polar enumeration and exact origin-in-interior are implemented; the current `f64` path is diagnostic and reports indeterminate candidates instead of guessing. | `crates/euclidean-polytopes/src/polar.rs`, `crates/euclidean-polytopes/src/predicates.rs`, `crates/euclidean-polytopes/tests/polar_vertices.rs` |
| Extreme-point / non-redundant point-set predicate | `[implemented exact slice]` | mainline thesis | agents | Exact predicate is implemented and covered by fixture/property tests. Add the matching f64 diagnostic later only if its return shape stays flat and useful. | `crates/euclidean-polytopes/src/predicates.rs`, `crates/euclidean-polytopes/tests/extreme_points.rs`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Full-dimensional f64 volume | `[implemented f64 slice]` | mainline thesis | agents | Review and integrate `volume_f64(dual_vertices, vertices)` into callers when a migration packet needs it. The API uses dual vertices for incidence, primal vertices for determinant geometry, and an operation-specific indeterminate payload when f64 incidence is tolerance-sensitive. | `crates/euclidean-polytopes/src/volume.rs`, `crates/euclidean-polytopes/tests/volume.rs`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Verification property suite | `[implemented first property slice]` | mainline thesis | agents | Keep strengthening theorem-shaped property tests as APIs migrate. Current coverage includes exact polar soundness, exact polarity roundtrip, generated non-redundancy witnesses, f64 simplex/crosspolytope polar agreement, and f64 volume scaling/permutation invariants. The stronger no-indeterminate f64 polar proposition is not yet true for non-simple cube/crosspolytope tuple structures under the current diagnostic contract. | `crates/euclidean-polytopes/DEVELOPMENT.md`, `crates/euclidean-polytopes/tests/` |
| Full-dimensional exact volume | `[future slice]` | mainline thesis | agents | After the f64 volume shape is reviewed, add exact volume over `T` without field extensions. | `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Affine-subspace polygons and volume | `[active]` | mainline thesis | agents, Jorn only if generic-vs-specific API affects thesis callers | Let the internal volume decomposition determine the first affine-subspace helper shape; likely needs 3-face measures of 4-polytopes and polygon area in affine 2-planes of `R^4`. | `crates/symplectic/src/geom/volume.rs`, `crates/symplectic/src/geom/polygon.rs` |
| Symplectic integration cleanup | `[active]` | mainline thesis | agents | After each migrated slice, keep `symplectic` as the owner of symplectic form, capacity, KKT, omega signs, Reeb-direction adjacency, and experiment-facing wrappers only. | `crates/symplectic/src/geom/`, `crates/symplectic/src/algorithms/` |

## Done Criteria

The migration task is done when:

- `crates/euclidean-polytopes/` has consumer and maintainer docs matching the
  implemented public API;
- the first implementation packet is test-driven: exact fixture tests are
  written before exact implementation, and f64 indeterminate tests are written
  before f64 filtering;
- tests cover at least simplex/cube/crosspolytope polar duality, redundant input
  points, bad `0 in int conv` contract panics, exact deduplication, non-finite
  f64 input, and a near-boundary f64 case that returns indeterminate instead of
  guessing;
- the extreme-point predicate slice has fixture tests for simplex/cube vertices
  returning `true`, exact duplicates returning `false`, an interior point in a
  simplex returning `false`, and lower-dimensional polygon vertices in `R^4`
  returning `true`;
- the same slice has at least one property test that constructs a point as a
  convex combination of generated exact points and checks that adding it makes
  `all_points_are_extreme_exact` return `false`;
- the f64 volume slice has fixture tests for simplex, hypercube, and
  crosspolytope volume, a scaling test or property test, non-finite input, and
  an incidence-boundary case that returns indeterminate instead of guessing;
- property-test comments separate theorem-shaped propositions from the actual
  operationalization: generator, precondition/discard rule, case count,
  fixtures, and f64 tolerance;
- the verification suite covers exact polar soundness, polarity roundtrip,
  f64/exact agreement on well-conditioned fixtures, and volume scaling plus
  permutation invariance before broad caller migration;
- workspace commands pass:
  `cargo test -p euclidean-polytopes`,
  `cargo clippy -p euclidean-polytopes --all-targets -- -D warnings`, and
  `cargo check --workspace`;
- `symplectic` no longer owns reusable ordinary convex-geometry algorithms
  except thin wrappers needed for compatibility;
- existing symplectic behavior is preserved by tests or explicit migration
  notes;
- `f64` combinatorial APIs expose error bounds or indeterminate outcomes, and
  exact APIs resolve those outcomes before returning exact data;
- exact predicate APIs return `bool`, with diagnostics confined to f64 helper
  APIs or error types;
- API review has confirmed that no alias, wrapper, trait, or generic result
  abstraction was added without repeated current callers or a clear
  simplification;
- `crates/MAP.md`, this task, and affected crate docs are updated.

## Agent Cache

- [fresh 2026-05-10] `Polytope4D` currently mixes ordinary geometry with
  symplectic data: dual vertices, primal vertices, incidence, vertex adjacency,
  omega signs, and f64 copies. The Euclidean crate should take the ordinary
  pieces first; omega signs and capacity-facing adjacency remain symplectic.
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
- [implemented 2026-05-10] The f64 full-dimensional volume slice exposes
  `volume_f64(dual_vertices, vertices)`. It validates finite input, decides
  vertex-facet incidence from local signed-gap diagnostics, returns
  `VolumeF64::Indeterminate` for near-incidence relations, and computes decided
  volume by origin-star triangulation using primal vertices for determinants.
  The decided payload intentionally omits a volume error bound until a credible
  determinant-sum rounding analysis exists.
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
  and negative cases, f64 polar agreement on the currently decidable simplex
  and crosspolytope fixtures, and f64 volume rational-scaling/permutation
  invariants. The symplectic migration regression remains future work because
  adding it now would introduce a new `symplectic` -> `euclidean-polytopes`
  test dependency before the caller migration packet.

## Pruned / Stale

- None yet.
