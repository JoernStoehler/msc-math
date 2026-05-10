# euclidean-polytopes

Euclidean convex-polytope helpers for thesis Rust code.

Use this crate for ordinary convex geometry that does not mention the
symplectic form: convex-hull predicates, polar/dual conversion, vertex-facet
incidence, affine-subspace polygons, and Euclidean volume/area helpers.

Do not use this crate for EHZ capacity, Reeb dynamics, symplectic signs, KKT
assembly, orbit search, or thesis experiment workflow code. Those stay in
`symplectic` or experiment packages.

## Representation

The first migration target is flat function style over nalgebra vectors:

```rust,ignore
use nalgebra::Vector4;

fn caller<T>(dual_vertices: Vec<Vector4<T>>) {
    // The caller owns the mathematical contract for this list.
    // For normalized H-representation:
    //
    //     K = { x in R^4 : <a_i, x> <= 1 for all a_i in dual_vertices }.
}
```

There is intentionally no public alias for `Vec<Vector4<T>>` and no public
polytope wrapper in the first target. Context-dependent facts such as
`0 in int conv(a)`, non-redundancy, ordering, and whether a list is primal or
dual are function contracts or checked preconditions.

The crate may return small result records when a computation naturally produces
several coupled outputs, such as vertices plus vertex-facet incidence. Those
records should be output payloads, not smart constructors that pretend to prove
all future context.

## Target API Shape

The initial public API should be close to these mathematical operations:

```rust,ignore
use algebraic_numbers::ExactScalar;
use nalgebra::Vector4;

pub fn origin_in_interior_of_conv<T: ExactScalar>(points: &[Vector4<T>]) -> bool;

pub fn all_points_are_extreme<T: ExactScalar>(points: &[Vector4<T>]) -> bool;

pub fn polar_vertices<T: ExactScalar>(
    vertices: &[Vector4<T>],
) -> Result<PolarVertexData<T>, PolarError>;

pub fn full_dimensional_volume_from_polar_pair(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
) -> Result<f64, VolumeError>;

pub fn polygon_area_in_affine_plane(vertices: &[Vector4<f64>]) -> Result<f64, VolumeError>;
```

`polar_vertices(vertices)` assumes the input vertices define a full-dimensional
convex polytope with `0` in its interior. The same function computes vertices
from dual vertices, because polarity is involutive under that contract.

The full-dimensional volume target should use `dual_vertices` only to recover
facet incidence (`<a_i, v> = 1`) and use `vertices` for Euclidean geometry. This
keeps the call close to the math and avoids constructing a symplectic
`Polytope4D` only to ask an ordinary volume question.

Lower-dimensional volume is still a design target, not an accepted signature.
The expected need is area/perimeter/length for polytopes in affine subspaces of
`R^4`, especially polygons in planes cut out by equations such as
`<x, a> = 1`. The migration should add the smallest function that current
callers need, not a general polytope-measure framework.

## Contracts

Functions should classify preconditions explicitly:

- checked here: invalid lengths, zero vectors, non-finite `f64` coordinates,
  rank failures, and exact predicate failures when cheap enough;
- assumed after a named validation boundary: hot-path callers may use
  `debug_assert!` for contracts already checked by an upstream exact routine;
- mathematical non-success: return an error/outcome enum when the input is a
  valid point set but does not satisfy the requested predicate;
- programmer bug: panic only for shape mismatches that cannot be recovered from
  sensibly by a thesis caller.

Exact combinatorial predicates should use `T: ExactScalar`. Approximate `f64`
helpers should stay separate and state their tolerance/error semantics in their
rustdoc.

## Non-Goals

- no symplectic form or capacity API;
- no public `Polytope4D` replacement before call sites prove it removes real
  complexity;
- no dimension-generic API before there is a caller outside ambient `R^4`;
- no general computational-geometry framework;
- no automatic orientation, units, or provenance layer around plain vectors.

