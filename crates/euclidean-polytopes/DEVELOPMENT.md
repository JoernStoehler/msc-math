# euclidean-polytopes development

This file is for maintainers. The README is for consumers.

## Instrumental Objective

Create a small Euclidean convex-geometry crate that removes ordinary polytope
geometry from `symplectic` without creating a second abstract geometry system.

The thesis-facing objective is agent velocity and validation trust:

- ordinary convex-geometry operations should have one reusable home;
- symplectic code should stop owning non-symplectic facts;
- exact combinatorial decisions should be reviewable independently from
  floating-point numerical helpers;
- API contracts should map directly to mathematical statements used by
  experiments and thesis prose.

## Current State

The implemented packets are in place:

- `sample_random_dual_vertices_f64(facet_count, h_min, h_max, rng) -> Vec<Vector4<f64>>`;
- `origin_in_interior_of_conv_exact(points) -> bool`;
- `all_points_are_extreme_exact(points) -> bool`;
- `polar_vertices_exact(vertices) -> PolarVerticesExact<T>`;
- `vertex_facets_from_vertex_facet_incidence(vertex_facet_incidence) -> Vec<Vec<usize>>`;
- `facet_vertices_from_vertex_facet_incidence(vertex_facet_incidence) -> Vec<Vec<usize>>`;
- `edges_from_vertex_facet_incidence(vertex_facet_incidence) -> Vec<[usize; 2]>`;
- `two_faces_from_vertex_facet_incidence(vertex_facet_incidence) -> Vec<TwoFace>`;
- `facet_intersection_is_nonempty_from_vertex_facet_incidence(vertex_facet_incidence) -> DMatrix<bool>`;
- `orient4_sign_f64(rows) -> CertifiedSign`;
- `origin_in_interior_of_conv_f64(points) -> OriginInteriorF64`;
- `volume_from_incidence_f64(vertices, incidence) -> Result<f64, F64GeometryError>`;
- `volume_from_incidence_exact(vertices, incidence) -> T`.

The exact path is the accepted first reusable slice. It checks the
origin-interior contract, enumerates 4-tuples of active polar inequalities,
tests halfspace feasibility exactly, and deduplicates vertices by exact
equality. It deliberately does not require input non-redundancy.

The exact extreme-point predicate checks V-representation non-redundancy
separately from polar enumeration. It uses ambient-`R^4` Caratheodory witnesses:
for each point, it enumerates subsets of at most five other points and solves
the exact affine barycentric system. A coordinate-bound reduction avoids
unnecessary exact solves for obvious coordinate-extreme cases. Exact duplicate
points return `false`; lower-dimensional point sets are valid inputs.

Public `f64` diagnostics are limited to theorem-backed sign/interior filters
with explicit indeterminate outcomes. They certify statements about the exact
real values represented by f64 inputs, not separate exact rationals rounded to
f64. The retained `f64` volume helpers require already-known boolean
incidence, validate finite coordinates, and do not decide incidence from
approximate signed gaps.

Do not add broad public API only because it is mathematically natural. Add a
function when a current migration caller needs it or when it removes duplicated
existing code.

## Implemented Slice: Random Dual-Vertex Candidate Sampling

Ordinary Euclidean random sampling of candidate normalized dual vertices lives
in this crate:

```rust,ignore
pub fn sample_random_dual_vertices_f64<R: rand::Rng + ?Sized>(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut R,
) -> Vec<Vector4<f64>>;
```

The helper samples independent unit normals on `S^3` and heights in
`[h_min, h_max)`, returning `a_i = n_i / h_i` for normalized halfspaces
`<a_i, x> <= 1`. It asserts the programmer contract `facet_count >= 5` and
finite `0 < h_min < h_max`. It deliberately does not validate boundedness,
validate non-redundancy, or own rejection sampling. `symplectic::random`
keeps the rejection loop and master-seed/attempt derivation, and validates
candidate dual vertices through the flat rational vertex-enumeration pipeline.

## Implemented Slice: Extreme-Point Predicate

The point-set non-redundancy check is implemented: every input point is an
extremum of the convex hull of the full input set.

This slice stayed separate from volume because it is a basic point-set contract
and the volume API has a larger decomposition contract.

Exact API:

```rust,ignore
pub fn all_points_are_extreme_exact<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
) -> bool;
```

Definition: return `true` iff for every index `i`, `points[i]` is not contained
in `conv(points without i)`. Duplicate exact points make the answer `false`.
The predicate should work for lower-dimensional point sets as well as
full-dimensional point sets in ambient `R^4`; for example, the vertices of a
polygon in an affine plane of `R^4` can all be extreme.

Use Caratheodory in ambient `R^4`: if `points[i]` lies in the convex hull of
the other points, there is a witness subset of at most five other points. The
exact implementation can enumerate subsets of size `1..=5`, solve the affine
barycentric system exactly, and accept a witness when all barycentric
coordinates are nonnegative. Keep helper functions local and concrete unless
they are clearly reused by another public operation.

The matching `f64` diagnostic API remains future work. It should prove `false`
as soon as it has a stable convex-combination witness for one non-extreme
point. It should prove `true` only after ruling out every relevant witness
subset for every point. Otherwise it should return the candidate subsets whose
classification is indeterminate, grouped by the tested point. Use names such as
`barycentric_abs_error_bound` instead of `_error`.

Do not introduce a public `ExtremePointSet` wrapper or generic
true/false/indeterminate abstraction for this slice. A flat output record for
the `f64` diagnostics is acceptable if a tuple would hide the meaning of the
payload.

## Implemented Slice: Known-Incidence f64 Volume

The ordinary full-dimensional `R^4` volume computation has a flat f64 entry
point for callers that already have reliable vertex-facet incidence:

```rust,ignore
pub fn volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> Result<f64, F64GeometryError>;
```

The implementation uses origin-star triangulation over flat slices. It uses
`vertices` for Euclidean determinants and uses `incidence[(v, f)]` only for
facet and 2-face combinatorics. It does not introduce a shared polytope
wrapper or a qhull dependency.

Fixture tests cover:

- simplex volume `1/24`;
- hypercube `[-1,1]^4` volume `16`;
- crosspolytope with vertices `+-2 e_i` volume `32/3`;
- non-finite input returns `F64GeometryError::NonFiniteCoordinate`;
- incidence row mismatch panics.

## Implemented Slice: Known-Incidence Volume and Symplectic Integration

This migration slice first let symplectic ordinary volume computation delegate
to this crate without throwing away exact incidence already known by the
fixture/database geometry. A later cleanup deleted the public
`symplectic::geom::volume` module. Current callers use
`euclidean_polytopes::volume_from_incidence_exact` directly, usually through
small package-local helpers when several experiment binaries need the same
exact-to-f64 projection.

`symplectic` is intentionally not routed through approximate incidence
recovery when exact boolean vertex-facet incidence is already available.
Recomputing that incidence in f64 would be less reliable than the stored
fixture/database path.

Implemented helper:

```rust,ignore
pub fn volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> Result<f64, F64GeometryError>;
```

Contract: `incidence[(v, f)]` tells whether `vertices[v]` lies on facet `f`
for a normalized full-dimensional `R^4` polytope containing the origin. The
helper validates finite vertices and asserts that incidence row count matches
the vertex count. Facet count, vertex count, and every facet having enough
vertices for the full-dimensional origin-star decomposition are caller
contracts and panic as programmer errors.

This slice first made the old symplectic volume wrapper call the Euclidean f64
helper with `polytope.vertices_f64()` and `polytope.incidence()`. A later exact
API slice replaced that f64 path as the source of truth. The final cleanup
removed the public wrapper and qhull benchmark/API surface; qhull remains only
as a private test helper under `symplectic`.

Verification witnesses for this slice:

- `euclidean-polytopes` tests cover simplex, hypercube, crosspolytope,
  non-finite input, and incidence row mismatch panic;
- `symplectic` tests and experiment helpers call the Euclidean exact
  known-incidence helper directly for exact-to-f64 volume projections;
- required commands for this slice are tracked in the task file.

## Implemented Slice: Known-Incidence Facet 3-Volume and Centroid

Ordinary facet 3-volume and centroid computation now lives in
`euclidean-polytopes` for callers that already have exact vertex-facet
incidence.

Do not recover facet/2-face incidence by checking `|a . v - 1| < EPS` when a
caller has a `DMatrix<bool>` incidence matrix. Derivative code should use
stored exact incidence from fixtures or experiment-local geometry caches.

Implemented helpers:

```rust,ignore
pub fn facet_volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    facet_index: usize,
) -> Result<f64, F64GeometryError>;

pub fn facet_volume_and_centroid_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    facet_index: usize,
) -> Result<(f64, Vector4<f64>), F64GeometryError>;
```

The helpers are flat and operation-specific. Contract: `incidence[(v, f)]`
tells whether `vertices[v]` lies on facet `f` for a normalized
full-dimensional `R^4` polytope containing the origin. The helpers validate
finite vertices and assert incidence row shape and facet-index range. They use
the target facet's incident vertices, triangulate each 2-face
`facet_index ∩ neighbor_index`, and compute 3-dimensional tetrahedron volume in
the facet's affine hyperplane using the ordinary 4D cross product norm.

`symplectic::geom::facet_volume` now reexports the flat Euclidean helpers.
`volume_derivatives_a` takes `dual_vertices`, `vertices`, and
`vertex_facet_incidence` explicitly and therefore benefits from exact
incidence supplied by fixtures or experiment-local geometry caches.

Implemented criteria for this slice:

- `euclidean-polytopes` exposes known-incidence facet volume and
  volume-plus-centroid helpers with docs that explain when they are preferable
  to raw f64 dual/vertex incidence recovery;
- `symplectic::geom::facet_volume` exposes the flat Euclidean helpers through
  the symplectic namespace;
- `volume_derivatives_a` no longer recomputes facet incidence from f64 raw
  dual/vertex arrays when exact incidence is available;
- tests cover hypercube facet volume, centroid-on-facet for known fixtures,
  divergence-theorem volume reconstruction, non-finite input, shape mismatch,
  out-of-range facet indices, and a symplectic API regression;
- verification command results are recorded in the branch handoff.

## Implemented Slice: Incidence-Only Face Combinatorics

Ordinary incidence-only face combinatorics now has a public flat surface:

```rust,ignore
pub struct TwoFace {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

pub fn vertex_facets_from_vertex_facet_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> Vec<Vec<usize>>;

pub fn facet_vertices_from_vertex_facet_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> Vec<Vec<usize>>;

pub fn edges_from_vertex_facet_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> Vec<[usize; 2]>;

pub fn two_faces_from_vertex_facet_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> Vec<TwoFace>;

pub fn facet_intersection_is_nonempty_from_vertex_facet_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> DMatrix<bool>;
```

The helpers use only `DMatrix<bool>` incidence. They do not inspect coordinates,
do not recover incidence from f64 signed gaps, and do not order 2-face vertices
cyclically. Empty matrices return empty results. Edge detection follows the
current 4D convention: two vertices form an edge when they share at least three
incident facets. A 2-face is reported when a sorted facet pair has at least
three common vertices; the returned vertex list is sorted by vertex index.
`facet_vertices_from_vertex_facet_incidence` transposes the public
vertex-facet incidence semantics into sorted facet-vertex lists.
`facet_intersection_is_nonempty_from_vertex_facet_incidence` returns the
false-diagonal facet-pair matrix where entries mean the two facets share at
least one vertex, not necessarily a 2-face.

The old shared skeleton wrapper was deleted during the flat migration. Current
callers use the incidence helpers directly or keep experiment-local feature
names for persisted datasets.

Implemented criteria for this slice:

- public helpers are exported from `euclidean-polytopes` and covered by pure
  incidence tests for the 4-simplex and 4-cube;
- tests check deterministic ordering of edge pairs, 2-face facet pairs, and
  2-face vertex lists;
- tests cover an incidence matrix with no valid 2-face candidates;
- symplectic and experiment callers use flat incidence helpers or local
  dataset/feature names instead of a shared skeleton wrapper.

## Implemented Slice: Incidence-Only 2-Face Ordering

This migration slice added an incidence-only helper for ordering the
vertices of the 2-face `facet_i ∩ facet_j`. This helper is the combinatorial
piece needed before exact full-dimensional volume can sum exact determinants
without using f64 polygon ordering.

Use `2-face` in public docs, task text, and new helper names. Do not introduce
new public APIs using the term `ridge`; old internal code may keep that name
until touched.

Target helper shape:

```rust,ignore
fn order_2face_vertices_from_incidence(
    incidence: &DMatrix<bool>,
    facet_i: usize,
    facet_j: usize,
) -> Vec<usize>;
```

The helper is private because no external caller needs it yet. It uses only
incidence and does not inspect coordinates or dual vertices.

Edge behavior:

- if `facet_i == facet_j`, panic as caller error;
- if either facet index is out of range, panic as caller error;
- if `facet_i ∩ facet_j` contains 0, 1, or 2 vertices, return those vertex
  indices in deterministic sorted order; callers treat them as not being a
  polygonal 2-face and skip them;
- if the intersection contains at least 3 vertices, build the induced graph
  where two vertices are adjacent when they share some third facet different
  from `facet_i` and `facet_j`;
- assert that this induced graph is one cycle, then return that cycle order;
- do not detect coordinate degeneracy. If incidence gives a valid cycle,
  determinant summation can handle zero or tiny geometric contribution.

Implemented criteria for this slice:

- helper tests cover empty, one-point, two-point, triangular, quadrilateral,
  and higher-vertex 2-face cases from constructed incidence matrices;
- known fixtures such as simplex, hypercube, crosspolytope, and available
  non-simple fixtures have their 2-face intersections classified/orderable;
- known-incidence f64 volume and facet-volume code use this helper instead of
  f64 coordinate polygon ordering when the helper returns at least 3 vertices;
- existing f64 volume/facet-volume tests still pass, including qhull and
  derivative checks;
- docs/tasks use `2-face` terminology for new work.

## Implemented Slice: Exact Full-Dimensional Volume From Known Incidence

Exact full-dimensional `R^4` volume from vertices and known incidence is
implemented:

```rust,ignore
pub fn volume_from_incidence_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    incidence: &DMatrix<bool>,
) -> T;
```

The helper does not take dual vertices. The determinant triangulation needs
only vertices and incidence. The contract is that `incidence[(v, f)]` matches
the vertices, the columns are the boundary facets of a full-dimensional
4-polytope, and `0` lies in the interior so coning facets to `0` decomposes the
polytope.

The implementation reuses incidence-only 2-face ordering, triangulates each
facet from its exact vertex mean, cones each triangle to `0`, and sums exact
4-simplex volumes as `abs(det) / 24`. The result stays in `T`; it does not
compute lower-dimensional Euclidean areas or distances that introduce
square-root intermediates.

Validated/asserted here:

- incidence row count equals `vertices.len()`;
- at least five vertices and five facets for full-dimensional 4-volume;
- every facet used by the volume decomposition has at least four incident
  vertices;
- every polygonal 2-face has a valid incidence cycle through the existing
  helper.

Not validated here:

- global geometric consistency between coordinates and incidence;
- existence or ordering of dual vertices;
- `0 in int(K)`.

Implemented criteria for this slice:

- public exact helper is exported and documented in `README.md`;
- exact tests cover simplex, hypercube, crosspolytope, shape panic, and a
  generated/scaled rational box property where the exact result is known;
- f64 known-incidence tests compare against the exact result on rational
  fixtures;
- code review confirms the exact computation uses no f64 arithmetic;
- required verification commands are listed in `tasks/euclidean-polytopes.md`.

## Implemented Slice: Symplectic Volume Uses Exact Known Incidence

The symplectic migration slice made the now-deleted
`symplectic::geom::volume` module use `volume_from_incidence_exact` as the
source of truth. Do not recompute incidence, do not pass dual vertices, and do
not use f64 vertices in exact volume paths.

That historical f64 volume API was an explicit f64
projection for callers that expected f64. The follow-up cleanup removed it from
`symplectic`; callers now import `euclidean-polytopes` directly and keep any
exact-to-f64 projection helper local to the package or test module.

Implemented criteria for this slice:

- the old `symplectic::geom::volume::volume_exact` and `volume_f64` APIs were
  deleted after call sites migrated;
- remaining symplectic and experiment callers delegate to
  `volume_from_incidence_exact`;
- exact fixture tests cover simplex, hypercube, and crosspolytope values;
- the wiring regression compares the symplectic API with exact
  Euclidean known-incidence volume, not the f64 Euclidean helper;
- docs/tasks record that full-dimensional Euclidean volume no longer belongs
  to the public symplectic API;
- the task file records the current verification commands.

## Test Code and Proposition Comments

Tests should make the mathematical proposition visible separately from the
sampling or fixture strategy.

Use this shape for property tests:

```rust,ignore
/// Proposition: for all point sets P in Q^4, if phi(P), then psi(P).
///
/// Operationalization: generate rational point sets with coordinates in
/// [-3, 3], discard samples that do not satisfy phi(P), and check psi(P).
/// Cases: 32 generated examples plus named fixtures in nearby tests.
#[test]
fn descriptive_property_name() { ... }
```

The proposition line should be the strongest theorem-shaped claim the test is
trying to exercise. The operationalization line should state the actual sample
space, discard rule, case count, tolerance, and any fixture names. Do not hide
a theorem inside the implementation details of the generator.

For `proptest`, use `prop_assume!(phi(sample))` only when `phi` is common
enough that rejects stay low. If the precondition is rare, write a generator
that constructs valid samples directly. For example, use paired points
`+-v_i` to force `0 in int conv(P)`, then add random redundant points.

Good test code in this crate:

- uses small local fixture constructors with mathematical names, such as
  `hypercube(scale)`, `crosspolytope_radius_2()`, or
  `positive_spanning_points()`;
- checks public APIs first and private helpers only for subtle regression
  reasons that cannot be observed cleanly through the public API;
- compares exact outputs as sets when the mathematical result is unordered;
- makes tolerance choices explicit and scale-aware for f64 metric tests;
- tests public f64 helper validation and shape contracts where approximate
  helpers remain public;
- states when a test is a fixture smoke check rather than evidence for a broad
  theorem;
- avoids random tests whose generator mostly proves input validation instead of
  the intended geometry.

Avoid comments that merely restate Rust code. Useful comments record the
mathematical object, the contract being exercised, why a sample is an edge
case, or why a tolerance is justified.

## Verification Property Backlog

These are the next high-value tests to add before broad caller migration.
Each item separates the theorem-shaped claim from one practical
operationalization.

1. Exact polar soundness:
   Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
   every `y` returned by `polar_vertices_exact(P)` satisfies
   `<p, y> <= 1` for all `p in P`, and the returned incidence matrix is exactly
   `(<p_j, y_i> == 1)`.
   Operationalization: generate rational point sets in `[-3, 3]^4`; either use
   `prop_assume!(origin_in_interior_of_conv_exact(&points))` or construct
   positive-spanning sets from `+-v_i`; assert feasibility and incidence for
   32 cases plus simplex, cube, and crosspolytope fixtures.

2. Exact polarity roundtrip:
   Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
   double polar of `P`, computed by binding
   `PolarVerticesExact { vertices, vertex_facet_incidence } =
   polar_vertices_exact(P)` and then `polar_vertices_exact(&vertices)`, is the
   set of extreme points of `conv(P)`.
   Operationalization: start with constructed positive-spanning exact point
   sets, optionally append exact convex-combination redundant points, compute
   the double polar, and compare with an explicit filtering of `P` by
   `all_points_are_extreme_exact`/single-point removal logic. Keep case counts
   modest until performance is measured.

3. Exact non-redundancy positive cases:
   Proposition: for every affinely independent simplex vertex set and every
   centrally symmetric box vertex set, every listed point is extreme.
   Operationalization: generate small random unimodular or axis-aligned
   rational transforms where exact coordinates stay small; assert
   `all_points_are_extreme_exact`.

4. Exact non-redundancy negative cases:
   Proposition: for every finite `P subset Q^4` and every
   `x in conv(P)`, not all points in `P union {x}` are extreme.
   Operationalization: the existing generated convex-combination test covers
   one 5-point witness shape. Extend it with lower-dimensional witnesses and
   duplicate/edge/face interior witnesses.

5. Known-incidence f64 volume invariants:
   Proposition: for full-dimensional polytopes with reliable boolean incidence,
   `volume_from_incidence_f64` returns the Euclidean volume and scales by `s^4`
   under `vertices -> s vertices`.
   Operationalization: the current hypercube scaling property covers powers of
   two and rational scale factors with scale-aware tolerance. A future
   permutation-invariance test should permute incidence rows/columns along with
   vertices/facets.

6. Symplectic migration regression:
   Proposition: for known symplectic fixtures, flat known-incidence volume
   preserves known ordinary volume values.
   Operationalization: current tests check simplex, hypercube, and
   crosspolytope values and compare symplectic fixture data with
   `volume_from_incidence_exact`.

## Proposed First Migration Slices

1. `polar_vertices_exact(vertices)` plus the validation and helper operations it
   needs. This includes exact affine/rank primitives over `Vector4<T>`, exact
   `origin_in_interior_of_conv`, and a separate exact non-redundancy/
   extreme-point check for callers that need that stronger input-list contract.
2. Known-incidence full-dimensional `R^4` volume using the existing origin-star
   triangulation idea. The retained f64 and exact variants both take vertices
   plus boolean incidence and do not recover incidence from approximate signed
   gaps.
4. A minimal affine-plane polygon helper for ordered or orderable
   `Vec<Vector4<f64>>` vertices, driven by the internal needs of the volume
   decomposition rather than only by external consumers asking for area.

The first implementation slice is intentionally not just a tiny predicate. It
should deliver polar vertex enumeration and whatever validation it needs so the
API is reviewed in a real caller-shaped context. Each later slice should still
be separately reviewable and keep `symplectic` compiling. Prefer moving tests
with the code, then adding one cross-crate regression in `symplectic` to prove
the old behavior still reaches the new helper.

## Module Structure

Use single-concern files and modules. Prefer a module per mathematical
operation family, with exact and f64 variants close enough to compare when they
implement the same algorithm. For example, `volume.rs` should own Euclidean
volume over both exact and f64 pathways unless implementation size proves that
`volume_exact.rs` and `volume_f64.rs` are easier to read and maintain.

The initial likely files are:

- `predicates.rs`: origin-interior and extremality predicates;
- `polar.rs`: polar vertex enumeration and incidence data;
- `volume.rs`: full-dimensional and affine-subspace Euclidean volume;
- `f64_geometry.rs`: shared f64 input validation and small diagnostic structs,
  only if repeated code makes a shared file simpler than local structs.

## API Review Notes

### Accepted Direction: Flat Inputs

Use `&[Vector4<T>]`, `Vec<Vector4<T>>`, tuples when positional meaning is
obvious, and local flat result records when outputs need names. Do not create
aliases such as `DualVertices4<T>`.

Reason: current callers already speak in lists of dual vertices, facets, and
vertices. A wrapper would mostly hide context-specific contracts that vary by
operation.

### Accepted Direction: Exact Naming

Use descriptive `_exact` and `_f64` suffixes when both pathways exist.

Reason: fresh agents do not reliably infer whether an unqualified geometry
function is exact or approximate. The suffixes make call sites easier to review.

### Accepted Direction: Output Records

`polar_vertices_exact` returns `PolarVerticesExact { vertices,
vertex_facet_incidence }` because the two exact outputs are central and their
index roles must remain visible at call sites.

Why it matters: volume and facet adjacency need incidence. Returning only
vertices would force recomputation. The record must not become a public
invariant wrapper unless future callers repeatedly pass the same certified
bundle through many operations.

Exact incidence can be a `DMatrix<bool>` or a `Vec<Vec<bool>>`, whichever makes
the implementation and callers simpler. Approximate incidence should use a flat
relation shape if each vertex/facet relation can carry true/false/indeterminate
diagnostics, such as a signed gap and error bound.

### Close Call: Full `Polytope` Type

No public polytope type in the first migration.

Reopen only if repeated call sites carry the same `(dual_vertices, vertices,
incidence)` bundle through several operations and the bundle itself becomes the
simple expression of the math. Until then, use explicit function contracts.

### Accepted Direction: `f64` Indeterminate, Exact Fallback

For combinatorial geometry, provide both approximate and exact pathways only
when callers need both.

The approximate pathway returns ordinary `f64` data with semantic names and
error bounds when that is the natural shape. For sign-like consumers, the usual
pattern is `x - x_abs_error_bound > 0`, `x + x_abs_error_bound < 0`, else
indeterminate. A tiny `True`/`False`/`Indeterminate` helper may be useful for
bare predicates. Prefer operation-specific diagnostic results when the
indeterminate case has useful payload, for example candidate sets of five
vertices that may contain zero. Do not introduce a generic wrapper for all
approximate results before repeated call sites prove it helps.

Approximate code must not silently decide cases where the result depends on a
near-singular solve, a near-duplicate candidate, or a halfspace membership test
near the tolerance boundary.

The exact pathway returns exact data. It may use the approximate pathway as a
fast filter, but every indeterminate branch must be resolved exactly before the
function returns. In vertex enumeration from dual vertices, a 4-tuple of
hyperplanes is exactly one of:

- singular or higher-dimensional, hence not a vertex candidate;
- uniquely intersecting in one point, then membership in all other halfspaces
  and duplicate equality are exact decisions.

Why it matters: this keeps the hot/common path fast without letting tolerance
choices become mathematical facts. If the exact fallback becomes hot, profile
before changing the contract.

For exact predicates specifically, return `bool`. The exact implementation may
call an `f64` diagnostic predicate first only when the diagnostic has a proved
error bound for the exact f64 inputs and every indeterminate branch is resolved
exactly before the exact function returns.

The checked polar APIs currently validate the origin-interior contract exactly.
The BigRational path uses integer-scaled determinant/gap arithmetic for the hot
case; it does not use a 5-set f64 precheck or an unproved f64 rejection filter.

For `origin_in_interior_of_conv_f64`, keep the diagnostic as the triple-normal
separation filter proved in `formal/f64-orientation-sign-filters.tex`.
`True` means every triple was certified nonseparating, `False` means one
triple was certified separating, and `Indeterminate` carries no claim. Do not
replace it with static margins or a 5-set barycentric heuristic.

### Accepted Direction: Result, Option, Panic, Tuple, Struct

Use `Result` for recoverable errors. Use `Option` only when the mathematical
distinction is exactly `None` versus `Some`, for example no solution versus an
affine solution space with a marked solution. Panic for irrecoverable caller
bugs such as wrong shapes, violated input contracts, or a detected mismatch
between the code and its mathematical invariants.

In the current API sketch, the only named recoverable error is non-finite f64
input. Do not introduce placeholder error types such as `ConvexHullError`,
`PolarError`, or `VolumeError` until their recoverable cases are known.

Use tuples when each position is obvious at the call site. Use a locally defined
flat data container when output variables need names. Avoid input wrappers
unless a pipeline naturally reuses the output type of an earlier function.

Name error bounds as `_abs_error_bound` for absolute error bounds and
`_rel_error_bound` for relative error bounds. Avoid `_error`, which is
ambiguous between a numeric bound and a failure/diagnostic.

### Accepted Direction: Polar Preconditions

`polar_vertices_exact(vertices)` needs `0 in int conv(vertices)` for the
normalized polar to be bounded and full-dimensional. Check/assert this in the
first implementation; optimize later only if profiling shows it matters.

The input list does not need to be non-redundant to compute the polar vertices:
a non-extreme input point produces a redundant polar inequality. Keep
`all_points_are_extreme_exact` as a separate check for callers that need the
input list itself to be exactly the extrema or need each input to correspond to
a non-redundant polar facet.

### Accepted Direction: Exact Versus `f64` Metric Outputs

Implement f64 volume first because existing thesis computations and stored data
use floating volume. Exact volume is still a real target, not a YAGNI violation:
the crate needs both f64 and exact geometry pathways eventually.

Why it matters: mixing exact predicates and approximate metrics in one function
would make contracts harder to audit. Keep exact validation and approximate
measurement separated.

### Close Call: Ambient Dimension

Keep ambient `R^4` as the first public target. Lower-dimensional polytopes are
subsets of `R^4`, not generic `R^d` values.

Reopen if a current thesis caller needs the same operation in another ambient
dimension. Const-generic dimensions would complicate determinants, rank helpers,
examples, and migration without solving a current problem.

### Accepted Direction: Affine-Subspace Volume Pressure

Affine-subspace volume is primarily determined by the internal decomposition of
`volume()`, not only by external consumers asking for a volume function. The
known-incidence 3-face need is covered by the facet-volume helpers. The
remaining likely need is polygons in affine 2-planes of `R^4`, with faces cut
out by equations like `<x, a> = 1`.

Decision that can still wait: whether the implementation should expose one
generic `affine_volume(vertices)` or a few concrete helpers such as
`polygon_area_in_affine_plane`. This matters because a generic function needs
a convex-hull/triangulation policy in each affine dimension, while the polygon
case can stay much simpler and easier to verify.

### Rejected For Now: LP Solver Dependency

Avoid adding a general LP dependency for origin-interior and extremality checks
until exact linear-algebra and small-dimensional Farkas-style predicates are
shown insufficient.

Reason: the current geometry is small-dimensional, exact, and already has
subset-enumeration/rank machinery. A solver dependency would add another
trust boundary and tolerance story.

## Migration Sources

Likely source files in `symplectic`:

- `src/geom/vertex_enumeration/`: boundedness, vertex enumeration,
  irredundancy, exact 4-vector linear algebra;
- `src/geom/volume.rs`: origin-star triangulation for full-dimensional
  `R^4` volume;
- `src/geom/polygon.rs` and `src/geom/polygon_order.rs`: polygon constructors,
  ordering, and area helpers;
- `src/geom/cross_product_4d.rs`: ordinary Euclidean 4D cross product;
- `src/geom/qhull.rs`: optional validation/backend helper, if still useful.

Keep in `symplectic`:

- `symplectic_form.rs`;
- capacity and orbit algorithms;
- KKT assembly from symplectic orbit data;
- omega-sign matrices and Reeb-direction adjacency;
- Lagrangian-product interpretation, though it may call Euclidean polygon
  helpers for the underlying geometry.

## Verification Target

For the scaffold:

```bash
cargo check -p euclidean-polytopes
```

For the first implementation packet, add:

```bash
cargo test -p euclidean-polytopes
cargo clippy -p euclidean-polytopes --all-targets -- -D warnings
cargo test -p symplectic --lib geom::
cargo check --workspace
```

Implementation workflow for the first packet:

- write exact fixture tests before implementing exact helpers;
- write f64 indeterminate/tolerance-boundary tests before f64 filtering;
- start with fixture tests, not property tests, until the public API is stable;
- use the workspace property-testing crates after fixture contracts are in
  place, especially for invariants such as polarity roundtrips, exact/f64
  agreement on well-conditioned generated inputs, and volume scaling;
- include tests for simplex/cube/crosspolytope polar duality, redundant input
  points, bad `0 in int conv` contract panics, exact deduplication, non-finite
  f64 input, and a near-boundary f64 case that returns indeterminate.
