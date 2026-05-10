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

The crate is scaffolded but intentionally has no public functions yet. The next
implementation packet should migrate one small slice from existing code, add
tests, and then update this file with the accepted API.

Do not add broad public API only because it is mathematically natural. Add a
function when a current migration caller needs it or when it removes duplicated
existing code.

## Proposed First Migration Slices

1. `polar_vertices_exact(vertices)` plus the validation and helper operations it
   needs. This includes exact affine/rank primitives over `Vector4<T>`, exact
   `origin_in_interior_of_conv`, and exact non-redundancy/extreme-point checks
   if the polar-vertices contract needs them.
2. `polar_vertices_f64(vertices)` with flat approximate outputs and explicit
   indeterminate tuple/candidate reporting instead of tolerance guesses.
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
pattern is `x - x_error > 0`, `x + x_error < 0`, else indeterminate. A tiny
`True`/`False`/`Indeterminate` helper may be useful for bare predicates. Prefer
operation-specific diagnostic results when the indeterminate case has useful
payload, for example candidate sets of five vertices that may contain zero.
Do not introduce a generic wrapper for all approximate results before repeated
call sites prove it helps.

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
