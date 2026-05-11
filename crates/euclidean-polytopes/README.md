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

## Implemented API

The implemented public API currently covers random candidate dual-vertex
sampling, exact point-set predicates, polar vertex enumeration,
incidence-only face combinatorics, full-dimensional volume, and known-incidence
facet 3-volume in ambient `R^4`:

```rust,ignore
use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Vector4};

pub fn origin_in_interior_of_conv_exact<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
) -> bool;

pub fn all_points_are_extreme_exact<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
) -> bool;

pub struct PolarVerticesExact<T: ExactScalar + 'static> {
    pub vertices: Vec<Vector4<T>>,
    pub vertex_facet_incidence: DMatrix<bool>,
}

pub fn polar_vertices_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
) -> PolarVerticesExact<T>;

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

pub fn volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> Result<f64, F64GeometryError>;

pub fn volume_from_incidence_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    incidence: &DMatrix<bool>,
) -> T;

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

pub enum F64GeometryError {
    NonFiniteCoordinate {
        vector_role: &'static str,
        vector_index: usize,
        coordinate_index: usize,
        value: f64,
    },
}

pub fn sample_random_dual_vertices_f64<R: rand::Rng + ?Sized>(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut R,
) -> Vec<Vector4<f64>>;
```

`sample_random_dual_vertices_f64(facet_count, h_min, h_max, rng)` samples
candidate normalized dual vertices `a_i = n_i / h_i`, with unit normals `n_i`
uniform on `S^3` and independent heights `h_i` uniform in `[h_min, h_max)`.
It asserts `facet_count >= 5` and finite `0 < h_min < h_max`. It does not
construct a polytope, validate boundedness, or test non-redundancy.

`polar_vertices_exact(vertices)` computes vertices of the normalized polar
`{ y in R^4 : <v_i, y> <= 1 }`. It validates the required contract
`0 in int conv(vertices)` with f64-decided true/false cases and exact fallback
for f64-indeterminate cases, then panics when validation rejects the input. The
input does not have to be non-redundant: redundant points only add redundant
inequalities. Returned vertices are deduplicated by exact equality. It returns
`PolarVerticesExact { vertices, vertex_facet_incidence }`; rows of the
incidence matrix are returned polar vertices and columns are input facets.

`all_points_are_extreme_exact(points)` checks the stronger non-redundancy
contract for a V-representation: every listed point must be an extreme point of
`conv(points)`. It handles lower-dimensional point sets in ambient `R^4`; exact
duplicate points return `false`.

The incidence-only face helpers accept a plain `DMatrix<bool>` with rows as
vertices and columns as facets. `vertex_facets_from_vertex_facet_incidence`
returns sorted incident-facet lists. `facet_vertices_from_vertex_facet_incidence`
returns sorted incident-vertex lists per facet. `edges_from_vertex_facet_incidence`
returns vertex pairs that share at least three incident facets.
`two_faces_from_vertex_facet_incidence` returns facet pairs with at least three
shared vertices; `TwoFace::vertices` is sorted by vertex index and is not
polygon-ordered. The known-incidence volume helpers perform the polygon
ordering they need internally. `facet_intersection_is_nonempty_from_vertex_facet_incidence`
returns the `F x F` facet-pair matrix with false diagonal where entry `(i, k)`
means facets `i` and `k` share at least one vertex.

`volume_from_incidence_f64(vertices, incidence)` computes the same ordinary
full-dimensional `R^4` volume when the caller already has reliable
vertex-facet incidence. `incidence[(v, f)]` must mean that `vertices[v]` lies
on facet `f` of a normalized full-dimensional polytope containing the origin.
The helper validates finite vertex coordinates. Incidence row/column shape and
full-dimensional decomposition assumptions are caller contracts and panic on
violation. It does not recompute combinatorics from f64 signed gaps.

`volume_from_incidence_exact(vertices, incidence)` computes the same
full-dimensional `R^4` volume over `T: ExactScalar`. It uses only the vertices
and known incidence: facet centroids are exact arithmetic means, polygonal
2-face vertices are ordered from incidence, and each origin-coned 4-simplex
contributes `abs(det) / 24`. The function does not take dual vertices and does
not use f64 arithmetic.

`facet_volume_from_incidence_f64(vertices, incidence, facet_index)` and
`facet_volume_and_centroid_from_incidence_f64(vertices, incidence, facet_index)`
compute ordinary 3D facet volume, and optionally the volume-weighted facet
centroid, from the same known vertex-facet incidence convention. They validate
finite vertex coordinates, assert incidence row shape and facet-index range,
and do not recover 2-face membership from f64 signed gaps. These helpers are the
preferred path for callers that already have exact vertex-facet incidence, such
as symplectic known fixtures or experiment-local geometry caches.

## Robust Numeric Split

The crate should expose two kinds of geometry APIs when a computation has both
approximate and exact callers:

- `f64 -> f64 plus error bounds or indeterminate`: fast approximate routines
  return ordinary floating values with explicit error bounds whenever that is
  the natural shape. A consumer that needs a sign decision checks, for example,
  `x - x_abs_error_bound > 0`, `x + x_abs_error_bound < 0`, and otherwise
  treats the result as indeterminate. The API must not guess when singularity,
  duplicate-candidate, or halfspace-membership decisions are near a tolerance
  boundary.
- `exact -> exact`: exact routines return exact results. They may use the `f64`
  routine internally as a cheap filter, but every indeterminate case must be
  resolved by a slow exact calculation before returning.

Avoid a generic result wrapper until repeated call sites prove it helps. A tiny
global `True`, `False`, `Indeterminate` enum may be useful for bare predicates.
The default should still be operation-specific diagnostic results with semantic
field names when such a public diagnostic API has a real caller.

For example, vertex enumeration from dual vertices can test most 4-tuples of
hyperplanes cheaply with a private or experiment-owned `f64` filter. The exact
API may use such a fast path only if every uncertain tuple is resolved exactly:
the intersection is empty/non-unique and not a vertex, or it is one point whose
halfspace inequalities and duplicate status are decided exactly.

## Future API Targets

The following operations are planned but their signatures are intentionally not
fixed here:

- approximate origin-interior diagnostics with all candidate 5-sets that may
  contain zero;
- approximate extreme-point/non-redundancy diagnostics for callers that need
  stable non-extreme witnesses or indeterminate witness subsets rather than
  tolerance guesses;
- affine-subspace polygon and lower-dimensional volume helpers in ambient
  `R^4`.

`polar_vertices_exact(vertices)` assumes `0 in int conv(vertices)`. This
condition is needed for the normalized polar to be a bounded full-dimensional
polytope. The input list does not need to be non-redundant for the polar
vertices to be computed correctly: a non-extreme input point gives a redundant
polar inequality. Non-redundancy matters only when a caller needs the input list
itself to be exactly the extrema of its convex hull or needs every input point
to correspond to a non-redundant polar facet. The same function computes
vertices from dual vertices, because polarity is involutive under the
`0 in int conv` contract.

The full-dimensional f64 volume helper takes known incidence. Callers that
recover incidence from approximate signed gaps should own that diagnostic
policy locally; `volume_from_incidence_f64` uses only `vertices` for Euclidean
determinant geometry.

Exact known incidence uses a plain `DMatrix<bool>` in the current volume
helper. Approximate incidence should not be forced into a boolean matrix when
each relation can be true, false, or indeterminate with diagnostics. Prefer a
flat relation list if it needs values such as
`signed_gap`, `signed_gap_abs_error_bound`, or candidate indices.

Lower-dimensional volume is a design target because the full-dimensional volume
implementation naturally decomposes into facet and 2-face measures. The expected
need now has known-incidence 3-faces of 4-polytopes covered; remaining likely
needs include polygons in affine 2-planes of `R^4`, especially planes cut out by
equations such as `<x, a> = 1`. The migration should still add the smallest
function that the volume decomposition needs, not a general polytope-measure
framework.

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

Use `Result` for recoverable errors. In the current sketch, the only concrete
recoverable error is non-finite f64 input, represented by `F64GeometryError`.
Use `Option` only when `None` versus `Some(_)` exactly matches the mathematical
distinction, such as an empty solution set versus an affine solution space with
a marked solution. Use tuples when each position is obvious at the call site.
Define a local flat `struct` when output variables need names, especially for
multi-output computations like `vertices` and `vertex_facet_incidence`.

Exact combinatorial predicates should use `T: ExactScalar` and return `bool`.
They may call the corresponding `f64` diagnostic function first and, when it
returns an indeterminate case, check the listed candidates exactly until the
answer is decided. Approximate `f64` helpers should stay separate and state
their tolerance, error, and indeterminate semantics in their rustdoc.

## Non-Goals

- no symplectic form or capacity API;
- no shared polytope wrapper before call sites prove it removes real complexity;
- no dimension-generic API before there is a caller outside ambient `R^4`;
- no general computational-geometry framework;
- no automatic orientation, units, or provenance layer around plain vectors.
