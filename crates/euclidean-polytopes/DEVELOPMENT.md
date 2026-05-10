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

The first implementation packet is in place:

- `origin_in_interior_of_conv_exact(points) -> bool`;
- `polar_vertices_exact(vertices) -> PolarVertexData<T>`;
- `polar_vertices_f64(vertices) -> Result<PolarVerticesF64, F64GeometryError>`.

The exact path is the accepted first reusable slice. It checks the
origin-interior contract, enumerates 4-tuples of active polar inequalities,
tests halfspace feasibility exactly, and deduplicates vertices by exact
equality. It deliberately does not require input non-redundancy.

The `f64` path is intentionally narrower than a full exact replacement. It
validates finite coordinates, returns well-conditioned accepted candidates, and
reports near-singular solves, near-boundary halfspace tests, and uncertain
duplicate decisions as `indeterminate_candidates`. Each indeterminate candidate
records the source 4-tuple and either `None` for singular/unsolved tuples or
`Some(vertex)` for an approximate vertex whose membership or duplicate status
was not decided in `f64`.

Do not add broad public API only because it is mathematically natural. Add a
function when a current migration caller needs it or when it removes duplicated
existing code.

## Proposed First Migration Slices

1. `polar_vertices_exact(vertices)` plus the validation and helper operations it
   needs. This includes exact affine/rank primitives over `Vector4<T>`, exact
   `origin_in_interior_of_conv`, and a separate exact non-redundancy/
   extreme-point check for callers that need that stronger input-list contract.
2. `polar_vertices_f64(vertices)` with flat approximate outputs and explicit
   indeterminate candidate reporting instead of tolerance guesses.
3. Full-dimensional `R^4` volume from `(dual_vertices, vertices)` using
   incidence and the existing origin-star triangulation idea. The `f64` variant
   can be indeterminate if incidence is tolerance-sensitive; the exact variant
   should decide incidence exactly and sum exact determinant volumes. Implement
   the `f64` path first, but keep exact volume as a real target rather than
   future/YAGNI speculation.
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

A small `PolarVertexData<T>`-style output record is acceptable if output
variables need names, for example vertices and incidence descriptors. Use a
tuple if each index is obvious. Use a flat local struct if a tuple would make
the call site harder to read.

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

For combinatorial geometry, provide both approximate and exact pathways when
callers need both.

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
call the `f64` diagnostic predicate first. If the f64 result is indeterminate,
it should check the diagnostic candidates exactly until the answer is known.
For `origin_in_interior_of_conv_exact`, one expected f64 diagnostic payload is
the candidate 5-point simplex sets that may contain zero.

For `origin_in_interior_of_conv_f64`, a false diagnostic needs enough evidence
to rule out all candidate 5-point simplex sets. Therefore the indeterminate
case should list all candidate sets that may contain zero, not just the first
one. Revisit only if memory or runtime measurements show this can blow up.

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
known need includes 3-faces of 4-polytopes and likely polygons in affine
2-planes of `R^4`, with faces cut out by equations like `<x, a> = 1`.

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
bash scripts/toc.sh crates/euclidean-polytopes/README.md crates/euclidean-polytopes/DEVELOPMENT.md tasks/euclidean-polytopes.md crates/MAP.md tasks/MAP.md
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
