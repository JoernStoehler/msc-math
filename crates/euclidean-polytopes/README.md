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

## Robust Numeric Split

The crate should expose two kinds of geometry APIs when a computation has both
approximate and exact callers:

- `f64 -> f64 plus error bounds or indeterminate`: fast approximate routines
  return ordinary floating values with explicit error bounds whenever that is
  the natural shape. A consumer that needs a sign decision checks, for example,
  `x - x_error > 0`, `x + x_error < 0`, and otherwise treats the result as
  indeterminate. The API must not guess when singularity, duplicate-candidate,
  or halfspace-membership decisions are near a tolerance boundary.
- `exact -> exact`: exact routines return exact results. They may use the `f64`
  routine internally as a cheap filter, but every indeterminate case must be
  resolved by a slow exact calculation before returning.

Avoid a generic result wrapper until repeated call sites prove it helps. A tiny
global `True`, `False`, `Indeterminate` enum may be useful for bare predicates.
The default should still be operation-specific diagnostic results with semantic
field names: `candidate_sets_that_may_contain_zero`, `vertices`,
`coordinate_error`, `indeterminate_tuples`.

For example, vertex enumeration from dual vertices can test most 4-tuples of
hyperplanes cheaply with `f64`. Near-singular tuples, uncertain duplicate
intersections, and uncertain membership in the other halfspaces should become
indeterminate in the `f64` API. The exact API may use the same fast path, then
resolve those tuples exactly: the intersection is empty/non-unique and not a
vertex, or it is one point whose halfspace inequalities and duplicate status
are decided exactly.

## Target API Shape

The initial public API should be close to these mathematical operations:

```rust,ignore
use algebraic_numbers::ExactScalar;
use nalgebra::Vector4;

pub fn origin_in_interior_of_conv_exact<T: ExactScalar>(points: &[Vector4<T>]) -> bool;

pub fn origin_in_interior_of_conv_f64(
    points: &[Vector4<f64>],
) -> Result<OriginInteriorF64, ConvexHullError>;

pub fn all_points_are_extreme_exact<T: ExactScalar>(points: &[Vector4<T>]) -> bool;

pub fn all_points_are_extreme_f64(
    points: &[Vector4<f64>],
) -> Result<ExtremalityF64, ConvexHullError>;

pub enum OriginInteriorF64 {
    True,
    False,
    Indeterminate {
        candidate_sets_that_may_contain_zero: Vec<[usize; 5]>,
    },
}

pub struct ExtremalityF64 {
    pub non_extreme_points: Vec<usize>,
    pub indeterminate_points: Vec<usize>,
}

pub fn polar_vertices_exact<T: ExactScalar>(
    vertices: &[Vector4<T>],
) -> Result<PolarVertexData<T>, PolarError>;

pub struct PolarVertexData<T> {
    pub vertices: Vec<Vector4<T>>,
    pub incidence: nalgebra::DMatrix<bool>,
}

pub struct PolarVerticesF64 {
    pub vertices: Vec<Vector4<f64>>,
    pub coordinate_error: f64,
    pub incidence: Vec<IncidenceF64>,
    pub indeterminate_tuples: Vec<[usize; 4]>,
}

pub struct IncidenceF64 {
    pub vertex_index: usize,
    pub facet_index: usize,
    pub signed_gap: f64,
    pub signed_gap_error: f64,
}

pub fn polar_vertices_f64(
    vertices: &[Vector4<f64>],
) -> Result<PolarVerticesF64, PolarError>;

pub fn full_dimensional_volume_from_polar_pair_exact<T: ExactScalar>(
    dual_vertices: &[Vector4<T>],
    vertices: &[Vector4<T>],
) -> Result<T, VolumeError>;

pub fn full_dimensional_volume_from_polar_pair_f64(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
) -> Result<VolumeF64, VolumeError>;

pub struct VolumeF64 {
    pub volume: f64,
    pub volume_error: f64,
    pub indeterminate_incidence: Vec<(usize, usize)>,
}

pub fn polygon_area_in_affine_plane(vertices: &[Vector4<f64>]) -> Result<f64, VolumeError>;
```

`polar_vertices_exact(vertices)` assumes the input vertices define a
full-dimensional convex polytope with `0` in its interior. The same function
computes vertices from dual vertices, because polarity is involutive under that
contract.

The full-dimensional volume target should use `dual_vertices` only to recover
facet incidence (`<a_i, v> = 1`) and use `vertices` for Euclidean geometry. This
keeps the call close to the math and avoids constructing a symplectic
`Polytope4D` only to ask an ordinary volume question.

Exact incidence can use `DMatrix<bool>` or another plain boolean matrix shape.
Approximate incidence should not be forced into a boolean matrix when each
relation can be true, false, or indeterminate with diagnostics. Prefer a flat
`Vec<IncidenceF64>`-style relation list if it needs values such as
`signed_gap`, `signed_gap_error`, or candidate indices.

Lower-dimensional volume is a design target because the full-dimensional volume
implementation naturally decomposes into facet and ridge measures. The expected
need includes 3-faces of 4-polytopes and polygons in affine 2-planes of `R^4`,
especially planes cut out by equations such as `<x, a> = 1`. The migration
should still add the smallest function that the volume decomposition needs, not
a general polytope-measure framework.

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

Use `Result` for recoverable errors. Use `Option` only when `None` versus
`Some(_)` exactly matches the mathematical distinction, such as an empty
solution set versus an affine solution space with a marked solution. Use tuples
when each position is obvious at the call site. Define a local flat `struct`
when output variables need names, especially for multi-output computations like
`vertices`, `coordinate_error`, and `indeterminate_tuples`.

Exact combinatorial predicates should use `T: ExactScalar` and return `bool`.
They may call the corresponding `f64` diagnostic function first and, when it
returns an indeterminate case, check the listed candidates exactly until the
answer is decided. Approximate `f64` helpers should stay separate and state
their tolerance, error, and indeterminate semantics in their rustdoc.

## Non-Goals

- no symplectic form or capacity API;
- no public `Polytope4D` replacement before call sites prove it removes real
  complexity;
- no dimension-generic API before there is a caller outside ambient `R^4`;
- no general computational-geometry framework;
- no automatic orientation, units, or provenance layer around plain vectors.
