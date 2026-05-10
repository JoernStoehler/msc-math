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

1. Exact affine/rank primitives over `Vector4<T>` where `T: ExactScalar`.
2. `origin_in_interior_of_conv(points)` for full-dimensional point sets in
   ambient `R^4`.
3. `all_points_are_extreme(points)` for V-representation non-redundancy.
4. `polar_vertices(vertices)` by enumerating 4-subsets of supporting
   constraints `<v_i, y> <= 1`.
5. Full-dimensional `R^4` volume from `(dual_vertices, vertices)` using
   incidence and the existing origin-star triangulation idea.
6. A minimal affine-plane polygon helper for ordered or orderable
   `Vec<Vector4<f64>>` vertices, after checking the first concrete caller.

Each slice should be separately reviewable and keep `symplectic` compiling.
Prefer moving tests with the code, then adding one cross-crate regression in
`symplectic` to prove the old behavior still reaches the new helper.

## API Review Notes

### Accepted Direction: Flat Inputs

Use `&[Vector4<T>]`, `Vec<Vector4<T>>`, and plain result records. Do not create
aliases such as `DualVertices4<T>`.

Reason: current callers already speak in lists of dual vertices, facets, and
vertices. A wrapper would mostly hide context-specific contracts that vary by
operation.

### Close Call: Output Records

A small `PolarVertexData<T>`-style output record is acceptable if it contains
several inseparable outputs, for example vertices and incidence descriptors.

Why it matters: volume and facet adjacency need incidence. Returning only
vertices would force recomputation. Returning a tuple would be less readable at
call sites. The record must not become a public invariant wrapper unless future
callers repeatedly pass the same certified bundle through many operations.

### Close Call: Full `Polytope` Type

No public polytope type in the first migration.

Reopen only if repeated call sites carry the same `(dual_vertices, vertices,
incidence)` bundle through several operations and the bundle itself becomes the
simple expression of the math. Until then, use explicit function contracts.

### Close Call: Exact Versus `f64`

Combinatorial decisions should be exact by default. Volume/area can start as
`f64` because the existing thesis computations and stored data use floating
volume, and exact Euclidean volume in affine subspaces has no current caller.

Why it matters: mixing exact predicates and approximate metrics in one function
would make contracts harder to audit. Keep exact validation and approximate
measurement separated.

### Close Call: Ambient Dimension

Keep ambient `R^4` as the first public target. Lower-dimensional polytopes are
subsets of `R^4`, not generic `R^d` values.

Reopen if a current thesis caller needs the same operation in another ambient
dimension. Const-generic dimensions would complicate determinants, rank helpers,
examples, and migration without solving a current problem.

### High-Uncertainty Area: Affine-Subspace Volume

The concrete need is not fully pinned down. We likely need polygons in affine
2-planes of `R^4`, and possibly 1-, 2-, 3-, and 4-dimensional measures for
faces cut out by equations like `<x, a> = 1`.

Decision that should wait: whether to expose one generic
`affine_volume(vertices)` or a few concrete helpers such as
`polygon_area_in_affine_plane`. This matters because a generic function needs a
convex-hull/triangulation policy in each affine dimension, while the polygon
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

