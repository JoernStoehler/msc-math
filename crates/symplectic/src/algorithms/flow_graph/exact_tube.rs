//! Exact closed-word resolver for the flow-graph algorithm.
//!
//! This module resolves one closed facet word from exact rational flow-graph
//! data. It is used both by exact exhaustive search and by the f64 wrapper when
//! a diagnostic f64 word needs exact closed-word resolution.

use nalgebra::DMatrix;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

type R = BigRational;
type Vec2 = [R; 2];
type Vec4 = [R; 4];
type Mat2 = [[R; 2]; 2];

#[derive(Clone, Copy, Debug)]
pub struct ExactFlatTubeInput<'a> {
    /// Exact rational facet normals in the flow-graph coordinate convention.
    pub dual_vertices: &'a [[BigRational; 4]],
    /// Caller-supplied facet-pair intersection data.  The exact resolver checks
    /// shape here, not boundedness, irredundancy, or semantic correctness of
    /// this matrix against `dual_vertices`.
    pub facet_intersection_is_nonempty: &'a DMatrix<bool>,
    /// Caller-supplied exact signs of `omega_0(a_i,a_j)`.  Shape is validated
    /// locally; semantic agreement with `dual_vertices` belongs to the trusted
    /// fixture/data-generation boundary recorded in the flow-graph README.
    pub omega_signs: &'a DMatrix<i8>,
}

impl ExactFlatTubeInput<'_> {
    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }
}

#[derive(Clone, Debug)]
struct Halfspace {
    normal: Vec2,
    rhs: R,
}

#[derive(Clone, Debug)]
struct ExactPolygon {
    halfspaces: Vec<Halfspace>,
}

#[derive(Clone, Debug)]
enum PolygonOutcome {
    Empty,
    Nonempty(ExactPolygon),
}

#[derive(Clone, Debug)]
struct Affine2 {
    matrix: Mat2,
    offset: Vec2,
}

#[derive(Clone, Debug)]
struct AffineScalar {
    coeff: Vec2,
    constant: R,
}

#[derive(Clone, Debug)]
struct FaceFrame {
    first: usize,
    second: usize,
    free: [usize; 2],
    base: Vec4,
    u: Vec4,
    v: Vec4,
}

#[derive(Clone, Debug)]
struct ExactTube {
    sequence: Vec<usize>,
    start_frame: FaceFrame,
    end_frame: FaceFrame,
    start_polygon: ExactPolygon,
    start_to_end: Affine2,
    action_on_start: AffineScalar,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExactClosedTubeMetrics {
    pub polygon_polygon_intersections: u64,
    pub polygon_halfspace_intersections: u64,
    pub action_cutoff_intersections: u64,
    pub pullbacks: u64,
    pub images: u64,
    pub emptiness_checks: u64,
    pub line_pair_checks: u64,
    pub containment_checks: u64,
    pub containment_halfspace_checks: u64,
    pub max_numer_bits: u64,
    pub max_denom_bits: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactClosedWordResult {
    pub word: Vec<usize>,
    pub outcome: ExactClosedWordOutcome,
    pub tube_halfspaces: Option<usize>,
    pub tube_vertices: Option<usize>,
}

/// Exact local polygon data for visualization consumers.
///
/// The coordinates are affine coordinates in the ordered face frame.  They
/// remain `BigRational` until an owner explicitly serializes them for a
/// renderer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactHalfspaceSnapshot {
    pub normal: [BigRational; 2],
    pub rhs: BigRational,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactFacePolygonSnapshot {
    pub pair: [usize; 2],
    pub base: [BigRational; 4],
    pub u: [BigRational; 4],
    pub v: [BigRational; 4],
    pub vertices: Vec<[BigRational; 2]>,
    pub inequalities: Vec<ExactHalfspaceSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactAffineSnapshot {
    pub matrix: [[BigRational; 2]; 2],
    pub offset: [BigRational; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactAffineScalarSnapshot {
    pub coeff: [BigRational; 2],
    pub constant: BigRational,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactTubeFaceSnapshot {
    pub pair: [usize; 2],
    pub polygon: ExactFacePolygonSnapshot,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactTubeFaceFixedPointSnapshot {
    pub pair: [usize; 2],
    pub role: String,
    pub point: Option<[BigRational; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactFixedPointSnapshot {
    pub status: String,
    pub point: Option<[BigRational; 2]>,
    pub action: Option<BigRational>,
    pub singular_status: Option<String>,
    pub min_action: Option<BigRational>,
    pub max_action: Option<BigRational>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactClosedOrbitSnapshot {
    pub facets: Vec<usize>,
    pub action: BigRational,
    pub breakpoints: Vec<[BigRational; 4]>,
    pub segment_times: Vec<BigRational>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactTubeVisualizationSnapshot {
    pub sequence: Vec<usize>,
    pub start_pair: [usize; 2],
    pub end_pair: [usize; 2],
    pub start_polygon: ExactFacePolygonSnapshot,
    pub end_polygon: ExactFacePolygonSnapshot,
    pub intermediate_polygons: Vec<ExactTubeFaceSnapshot>,
    pub fixed_points_on_faces: Vec<ExactTubeFaceFixedPointSnapshot>,
    pub start_to_end: ExactAffineSnapshot,
    pub end_to_start: ExactAffineSnapshot,
    pub action_on_start: ExactAffineScalarSnapshot,
    pub action_on_end: ExactAffineScalarSnapshot,
    pub fixed_point: ExactFixedPointSnapshot,
    pub cutoff: Option<BigRational>,
    pub closed_orbit: Option<ExactClosedOrbitSnapshot>,
    pub metrics: ExactClosedTubeMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExactClosedWordOutcome {
    EmptyTube,
    ZeroActionNoOrbit {
        action: Option<BigRational>,
        start_coords: Option<[BigRational; 2]>,
        singular_status: Option<&'static str>,
    },
    NonStrictNoOrbit {
        action: BigRational,
        start_coords: [BigRational; 2],
    },
    PositiveOrbit {
        action: BigRational,
        start_coords: [BigRational; 2],
    },
    UnsupportedPositiveSingular {
        singular_status: &'static str,
        min_action: Option<BigRational>,
        max_action: Option<BigRational>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExactClosedTubeError {
    InvalidInput,
    InvalidWord,
    UnsupportedSingularTransition,
    InternalInconsistentSingularSolve,
}

fn q(n: i64) -> R {
    BigRational::from_integer(n.into())
}

fn zero4() -> Vec4 {
    [R::zero(), R::zero(), R::zero(), R::zero()]
}

fn dot4(a: &Vec4, b: &Vec4) -> R {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

fn dot2(a: &Vec2, b: &Vec2) -> R {
    &a[0] * &b[0] + &a[1] * &b[1]
}

fn det2(a: &Vec2, b: &Vec2) -> R {
    &a[0] * &b[1] - &a[1] * &b[0]
}

fn add2(a: &Vec2, b: &Vec2) -> Vec2 {
    [&a[0] + &b[0], &a[1] + &b[1]]
}

fn mat_vec(m: &Mat2, x: &Vec2) -> Vec2 {
    [
        &m[0][0] * &x[0] + &m[0][1] * &x[1],
        &m[1][0] * &x[0] + &m[1][1] * &x[1],
    ]
}

fn mat_mul(a: &Mat2, b: &Mat2) -> Mat2 {
    [
        [
            &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0],
            &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1],
        ],
        [
            &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0],
            &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1],
        ],
    ]
}

fn mat_transpose_vec(m: &Mat2, x: &Vec2) -> Vec2 {
    [
        &m[0][0] * &x[0] + &m[1][0] * &x[1],
        &m[0][1] * &x[0] + &m[1][1] * &x[1],
    ]
}

fn inverse(m: &Mat2) -> Option<Mat2> {
    let det = &m[0][0] * &m[1][1] - &m[0][1] * &m[1][0];
    if det.is_zero() {
        return None;
    }
    Some([
        [&m[1][1] / &det, -&m[0][1] / &det],
        [-&m[1][0] / &det, &m[0][0] / &det],
    ])
}

fn j_times(a: &Vec4) -> Vec4 {
    [-a[2].clone(), -a[3].clone(), a[0].clone(), a[1].clone()]
}

fn scale4(c: &R, v: &Vec4) -> Vec4 {
    [&v[0] * c, &v[1] * c, &v[2] * c, &v[3] * c]
}

fn add4(a: &Vec4, b: &Vec4) -> Vec4 {
    [&a[0] + &b[0], &a[1] + &b[1], &a[2] + &b[2], &a[3] + &b[3]]
}

fn point_on_frame(frame: &FaceFrame, coords: &Vec2) -> Vec4 {
    add4(
        &frame.base,
        &add4(&scale4(&coords[0], &frame.u), &scale4(&coords[1], &frame.v)),
    )
}

impl ExactPolygon {
    fn new(halfspaces: Vec<Halfspace>, metrics: &mut ExactClosedTubeMetrics) -> PolygonOutcome {
        let polygon = Self { halfspaces };
        if polygon.is_empty(metrics) {
            PolygonOutcome::Empty
        } else {
            PolygonOutcome::Nonempty(polygon)
        }
    }

    fn contains(&self, point: &Vec2, metrics: &mut ExactClosedTubeMetrics) -> bool {
        metrics.containment_checks += 1;
        metrics.containment_halfspace_checks += self.halfspaces.len() as u64;
        self.halfspaces
            .iter()
            .all(|h| dot2(&h.normal, point) <= h.rhs)
    }

    fn is_empty(&self, metrics: &mut ExactClosedTubeMetrics) -> bool {
        metrics.emptiness_checks += 1;
        if self.halfspaces.is_empty() {
            return false;
        }
        // This is used only for bounded two-face/tube polygons. It is not a
        // general unbounded halfspace-feasibility solver.
        for i in 0..self.halfspaces.len() {
            for j in (i + 1)..self.halfspaces.len() {
                metrics.line_pair_checks += 1;
                let a = &self.halfspaces[i];
                let b = &self.halfspaces[j];
                let det = det2(&a.normal, &b.normal);
                if det.is_zero() {
                    continue;
                }
                let point = [
                    (&a.rhs * &b.normal[1] - &a.normal[1] * &b.rhs) / &det,
                    (&a.normal[0] * &b.rhs - &a.rhs * &b.normal[0]) / &det,
                ];
                if self.contains(&point, metrics) {
                    track_rational(metrics, &point[0]);
                    track_rational(metrics, &point[1]);
                    return false;
                }
            }
        }
        true
    }

    fn vertices(&self, metrics: &mut ExactClosedTubeMetrics) -> Vec<Vec2> {
        let mut vertices = Vec::new();
        for i in 0..self.halfspaces.len() {
            for j in (i + 1)..self.halfspaces.len() {
                metrics.line_pair_checks += 1;
                let a = &self.halfspaces[i];
                let b = &self.halfspaces[j];
                let det = det2(&a.normal, &b.normal);
                if det.is_zero() {
                    continue;
                }
                let point = [
                    (&a.rhs * &b.normal[1] - &a.normal[1] * &b.rhs) / &det,
                    (&a.normal[0] * &b.rhs - &a.rhs * &b.normal[0]) / &det,
                ];
                if self.contains(&point, metrics) && !vertices.contains(&point) {
                    vertices.push(point);
                }
            }
        }
        vertices
    }

    fn intersect_polygon(
        &self,
        other: &Self,
        metrics: &mut ExactClosedTubeMetrics,
    ) -> PolygonOutcome {
        metrics.polygon_polygon_intersections += 1;
        let mut halfspaces = self.halfspaces.clone();
        halfspaces.extend(other.halfspaces.iter().cloned());
        Self::new(halfspaces, metrics)
    }

    fn intersect_halfspace(
        &self,
        halfspace: Halfspace,
        metrics: &mut ExactClosedTubeMetrics,
    ) -> PolygonOutcome {
        metrics.polygon_halfspace_intersections += 1;
        let mut halfspaces = self.halfspaces.clone();
        halfspaces.push(halfspace);
        Self::new(halfspaces, metrics)
    }

    fn pullback(&self, affine: &Affine2, metrics: &mut ExactClosedTubeMetrics) -> Self {
        metrics.pullbacks += 1;
        Self {
            halfspaces: self
                .halfspaces
                .iter()
                .map(|h| Halfspace {
                    normal: mat_transpose_vec(&affine.matrix, &h.normal),
                    rhs: &h.rhs - dot2(&h.normal, &affine.offset),
                })
                .collect(),
        }
    }

    fn image_under(&self, affine: &Affine2, metrics: &mut ExactClosedTubeMetrics) -> Option<Self> {
        metrics.images += 1;
        let inverse_matrix = inverse(&affine.matrix)?;
        let inverse_offset = [
            -mat_vec(&inverse_matrix, &affine.offset)[0].clone(),
            -mat_vec(&inverse_matrix, &affine.offset)[1].clone(),
        ];
        Some(Self {
            halfspaces: self
                .halfspaces
                .iter()
                .map(|halfspace| Halfspace {
                    normal: mat_transpose_vec(&inverse_matrix, &halfspace.normal),
                    rhs: &halfspace.rhs - dot2(&halfspace.normal, &inverse_offset),
                })
                .collect(),
        })
    }
}

fn track_rational(metrics: &mut ExactClosedTubeMetrics, value: &R) {
    metrics.max_numer_bits = metrics.max_numer_bits.max(value.numer().bits());
    metrics.max_denom_bits = metrics.max_denom_bits.max(value.denom().bits());
}

impl Affine2 {
    fn then(&self, next: &Self) -> Self {
        Self {
            matrix: mat_mul(&next.matrix, &self.matrix),
            offset: add2(&mat_vec(&next.matrix, &self.offset), &next.offset),
        }
    }
}

impl FaceFrame {
    fn coords(&self, point: &Vec4) -> Vec2 {
        [point[self.free[0]].clone(), point[self.free[1]].clone()]
    }

    fn linear_coords(&self, vector: &Vec4) -> Vec2 {
        [vector[self.free[0]].clone(), vector[self.free[1]].clone()]
    }

    fn pair(&self) -> (usize, usize) {
        (self.first, self.second)
    }
}

fn face_frame(duals: &[Vec4], first: usize, second: usize) -> Option<FaceFrame> {
    let a = &duals[first];
    let b = &duals[second];
    let mut chosen = None;
    for r in 0..4 {
        for s in (r + 1)..4 {
            let det = &a[r] * &b[s] - &a[s] * &b[r];
            if !det.is_zero() {
                chosen = Some([r, s]);
                break;
            }
        }
        if chosen.is_some() {
            break;
        }
    }
    let solve = chosen?;
    let free_vec: Vec<usize> = (0..4)
        .filter(|idx| *idx != solve[0] && *idx != solve[1])
        .collect();
    let free = [free_vec[0], free_vec[1]];
    let base = solve_with_free(
        a,
        b,
        solve,
        free,
        [R::zero(), R::zero()],
        [R::one(), R::one()],
    )?;
    let u = solve_with_free(
        a,
        b,
        solve,
        free,
        [R::one(), R::zero()],
        [R::zero(), R::zero()],
    )?;
    let v = solve_with_free(
        a,
        b,
        solve,
        free,
        [R::zero(), R::one()],
        [R::zero(), R::zero()],
    )?;
    Some(FaceFrame {
        first,
        second,
        free,
        base,
        u,
        v,
    })
}

fn solve_with_free(
    a: &Vec4,
    b: &Vec4,
    solve: [usize; 2],
    free: [usize; 2],
    free_values: Vec2,
    rhs: Vec2,
) -> Option<Vec4> {
    let mut x = zero4();
    x[free[0]] = free_values[0].clone();
    x[free[1]] = free_values[1].clone();
    let rhs0 = &rhs[0] - &a[free[0]] * &free_values[0] - &a[free[1]] * &free_values[1];
    let rhs1 = &rhs[1] - &b[free[0]] * &free_values[0] - &b[free[1]] * &free_values[1];
    let det = &a[solve[0]] * &b[solve[1]] - &a[solve[1]] * &b[solve[0]];
    if det.is_zero() {
        return None;
    }
    x[solve[0]] = (&rhs0 * &b[solve[1]] - &a[solve[1]] * &rhs1) / &det;
    x[solve[1]] = (&a[solve[0]] * &rhs1 - &rhs0 * &b[solve[0]]) / &det;
    Some(x)
}

fn face_polygon(
    duals: &[Vec4],
    frame: &FaceFrame,
    metrics: &mut ExactClosedTubeMetrics,
) -> PolygonOutcome {
    ExactPolygon::new(
        duals
            .iter()
            .map(|a| Halfspace {
                normal: [dot4(a, &frame.u), dot4(a, &frame.v)],
                rhs: R::one() - dot4(a, &frame.base),
            })
            .collect(),
        metrics,
    )
}

fn primitive_tube(
    duals: &[Vec4],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
    facets: [usize; 3],
    metrics: &mut ExactClosedTubeMetrics,
) -> Result<PolygonOutcomeTube, ()> {
    let [previous, current, next] = facets;
    if !facet_intersection_is_nonempty[(previous, current)]
        || !facet_intersection_is_nonempty[(current, next)]
        || omega_signs[(previous, current)] < 0
        || omega_signs[(current, next)] < 0
    {
        return Ok(PolygonOutcomeTube::Empty);
    }
    if omega_signs[(previous, current)] == 0 || omega_signs[(current, next)] == 0 {
        return Err(());
    }
    let start_frame = face_frame(duals, previous, current).ok_or(())?;
    let end_frame = face_frame(duals, current, next).ok_or(())?;
    let PolygonOutcome::Nonempty(start_face) = face_polygon(duals, &start_frame, metrics) else {
        return Ok(PolygonOutcomeTube::Empty);
    };
    let PolygonOutcome::Nonempty(end_face) = face_polygon(duals, &end_frame, metrics) else {
        return Ok(PolygonOutcomeTube::Empty);
    };

    let reeb = scale4(&q(2), &j_times(&duals[current]));
    let denom = dot4(&duals[next], &reeb);
    if denom.is_zero() {
        return Err(());
    }
    let tau_coeff = [
        -dot4(&duals[next], &start_frame.u) / &denom,
        -dot4(&duals[next], &start_frame.v) / &denom,
    ];
    let tau_const = (R::one() - dot4(&duals[next], &start_frame.base)) / &denom;
    let start_to_end =
        primitive_affine_map(&start_frame, &end_frame, &reeb, &tau_coeff, &tau_const);
    let pulled_end = end_face.pullback(&start_to_end, metrics);
    let PolygonOutcome::Nonempty(start_polygon) =
        start_face.intersect_polygon(&pulled_end, metrics)
    else {
        return Ok(PolygonOutcomeTube::Empty);
    };
    let nonnegative_time = Halfspace {
        normal: [-tau_coeff[0].clone(), -tau_coeff[1].clone()],
        rhs: tau_const.clone(),
    };
    let PolygonOutcome::Nonempty(start_polygon) =
        start_polygon.intersect_halfspace(nonnegative_time, metrics)
    else {
        return Ok(PolygonOutcomeTube::Empty);
    };
    Ok(PolygonOutcomeTube::Nonempty(ExactTube {
        sequence: facets.to_vec(),
        start_frame,
        end_frame,
        start_polygon,
        start_to_end,
        action_on_start: AffineScalar {
            coeff: tau_coeff,
            constant: tau_const,
        },
    }))
}

fn primitive_affine_map(
    start: &FaceFrame,
    end: &FaceFrame,
    reeb: &Vec4,
    tau_coeff: &Vec2,
    tau_const: &R,
) -> Affine2 {
    let image_base = add4(&start.base, &scale4(tau_const, reeb));
    let image_u = add4(&start.u, &scale4(&tau_coeff[0], reeb));
    let image_v = add4(&start.v, &scale4(&tau_coeff[1], reeb));
    Affine2 {
        matrix: [
            [
                end.linear_coords(&image_u)[0].clone(),
                end.linear_coords(&image_v)[0].clone(),
            ],
            [
                end.linear_coords(&image_u)[1].clone(),
                end.linear_coords(&image_v)[1].clone(),
            ],
        ],
        offset: end.coords(&image_base),
    }
}

enum PolygonOutcomeTube {
    Empty,
    Nonempty(ExactTube),
}

fn intersect_tubes(
    first: &ExactTube,
    second: &ExactTube,
    metrics: &mut ExactClosedTubeMetrics,
) -> Result<PolygonOutcomeTube, ()> {
    if first.end_frame.pair() != second.start_frame.pair() {
        return Err(());
    }
    let pulled = second.start_polygon.pullback(&first.start_to_end, metrics);
    let PolygonOutcome::Nonempty(start_polygon) =
        first.start_polygon.intersect_polygon(&pulled, metrics)
    else {
        return Ok(PolygonOutcomeTube::Empty);
    };
    let start_to_end = first.start_to_end.then(&second.start_to_end);
    let mut sequence = first.sequence.clone();
    sequence.extend(second.sequence.iter().skip(2).copied());
    let action_on_start = AffineScalar {
        coeff: [
            &first.action_on_start.coeff[0]
                + &first.start_to_end.matrix[0][0] * &second.action_on_start.coeff[0]
                + &first.start_to_end.matrix[1][0] * &second.action_on_start.coeff[1],
            &first.action_on_start.coeff[1]
                + &first.start_to_end.matrix[0][1] * &second.action_on_start.coeff[0]
                + &first.start_to_end.matrix[1][1] * &second.action_on_start.coeff[1],
        ],
        constant: &first.action_on_start.constant
            + dot2(&second.action_on_start.coeff, &first.start_to_end.offset)
            + &second.action_on_start.constant,
    };
    Ok(PolygonOutcomeTube::Nonempty(ExactTube {
        sequence,
        start_frame: first.start_frame.clone(),
        end_frame: second.end_frame.clone(),
        start_polygon,
        start_to_end,
        action_on_start,
    }))
}

fn build_tube(
    duals: &[Vec4],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
    word: &[usize],
    metrics: &mut ExactClosedTubeMetrics,
) -> Result<PolygonOutcomeTube, ()> {
    match word.len() {
        0..=2 => Err(()),
        3 => primitive_tube(
            duals,
            facet_intersection_is_nonempty,
            omega_signs,
            [word[0], word[1], word[2]],
            metrics,
        ),
        _ => {
            let total_plus = word.len() - 2;
            let left_plus = total_plus / 2;
            if left_plus == 0 || left_plus == total_plus {
                return Err(());
            }
            let left_len = left_plus + 2;
            let right_start = left_len - 2;
            let left = match build_tube(
                duals,
                facet_intersection_is_nonempty,
                omega_signs,
                &word[..left_len],
                metrics,
            )? {
                PolygonOutcomeTube::Empty => return Ok(PolygonOutcomeTube::Empty),
                PolygonOutcomeTube::Nonempty(tube) => tube,
            };
            let right = match build_tube(
                duals,
                facet_intersection_is_nonempty,
                omega_signs,
                &word[right_start..],
                metrics,
            )? {
                PolygonOutcomeTube::Empty => return Ok(PolygonOutcomeTube::Empty),
                PolygonOutcomeTube::Nonempty(tube) => tube,
            };
            intersect_tubes(&left, &right, metrics)
        }
    }
}

fn restrict_tube_to_action_cutoff(
    tube: ExactTube,
    action_cutoff: Option<&BigRational>,
    metrics: &mut ExactClosedTubeMetrics,
) -> PolygonOutcomeTube {
    let Some(action_cutoff) = action_cutoff else {
        return PolygonOutcomeTube::Nonempty(tube);
    };
    metrics.action_cutoff_intersections += 1;
    let action_halfspace = Halfspace {
        normal: tube.action_on_start.coeff.clone(),
        rhs: action_cutoff - &tube.action_on_start.constant,
    };
    // Apply the exact retained-output cutoff before fixed-point solving.  Any
    // later singular classification is therefore about the remaining searched
    // domain, not necessarily the uncut closed tube.
    match tube
        .start_polygon
        .intersect_halfspace(action_halfspace, metrics)
    {
        PolygonOutcome::Empty => PolygonOutcomeTube::Empty,
        PolygonOutcome::Nonempty(start_polygon) => PolygonOutcomeTube::Nonempty(ExactTube {
            start_polygon,
            ..tube
        }),
    }
}

#[derive(Debug)]
enum ClosedClassification {
    EmptyTube,
    ZeroActionNoOrbit {
        action: Option<R>,
        point: Option<Vec2>,
        singular_status: Option<&'static str>,
    },
    NonStrictNoOrbit {
        action: R,
        point: Vec2,
    },
    PositiveOrbit {
        action: R,
        point: Vec2,
    },
    UnsupportedPositiveSingular {
        singular_status: &'static str,
        min_action: Option<R>,
        max_action: Option<R>,
    },
    ConstructionError,
}

pub fn resolve_closed_word_exact(
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
) -> Result<(ExactClosedWordResult, ExactClosedTubeMetrics), ExactClosedTubeError> {
    resolve_closed_word_exact_with_action_cutoff(input, sigma, None)
}

/// Return one exact ordered face section for a visualization producer.
pub fn face_polygon_snapshot_exact(
    input: &ExactFlatTubeInput<'_>,
    first: usize,
    second: usize,
) -> Result<ExactFacePolygonSnapshot, ExactClosedTubeError> {
    validate_exact_input(input)?;
    if first == second || first >= input.facet_count() || second >= input.facet_count() {
        return Err(ExactClosedTubeError::InvalidWord);
    }
    let frame = face_frame(input.dual_vertices, first, second)
        .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?;
    let mut metrics = ExactClosedTubeMetrics::default();
    let PolygonOutcome::Nonempty(polygon) = face_polygon(input.dual_vertices, &frame, &mut metrics)
    else {
        return Err(ExactClosedTubeError::InternalInconsistentSingularSolve);
    };
    Ok(exact_polygon_snapshot(
        [first, second],
        &frame,
        &polygon,
        &mut metrics,
    ))
}

/// Resolve a closed word and expose the exact state needed by the retained
/// tube figures.  This is the visualization boundary: callers receive exact
/// polygons, affine maps, fixed-point data, and orbit breakpoints and choose
/// where to perform the eventual `f64` serialization.
pub fn closed_tube_visualization_snapshot_exact(
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    action_cutoff: Option<&BigRational>,
) -> Result<Option<ExactTubeVisualizationSnapshot>, ExactClosedTubeError> {
    validate_exact_input(input)?;
    if sigma.len() < 2
        || sigma.iter().any(|&facet| facet >= input.facet_count())
        || !all_distinct(sigma)
    {
        return Err(ExactClosedTubeError::InvalidWord);
    }

    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    let mut metrics = ExactClosedTubeMetrics::default();
    let tube = match build_tube(
        input.dual_vertices,
        input.facet_intersection_is_nonempty,
        input.omega_signs,
        &word,
        &mut metrics,
    ) {
        Ok(PolygonOutcomeTube::Empty) => return Ok(None),
        Err(()) => return Err(ExactClosedTubeError::UnsupportedSingularTransition),
        Ok(PolygonOutcomeTube::Nonempty(tube)) => tube,
    };
    let tube = match restrict_tube_to_action_cutoff(tube, action_cutoff, &mut metrics) {
        PolygonOutcomeTube::Empty => return Ok(None),
        PolygonOutcomeTube::Nonempty(tube) => tube,
    };
    let classification = solve_closed_tube(input.dual_vertices, &tube, &mut metrics);
    let outcome = classification.into_public()?;
    let fixed_point = exact_fixed_point_snapshot(&outcome);
    let end_pair = [tube.end_frame.first, tube.end_frame.second];
    let end_polygon = tube
        .start_polygon
        .image_under(&tube.start_to_end, &mut metrics)
        .map(|polygon| {
            exact_polygon_snapshot(
                [tube.end_frame.first, tube.end_frame.second],
                &tube.end_frame,
                &polygon,
                &mut metrics,
            )
        })
        .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?;

    let start_pair = [tube.start_frame.first, tube.start_frame.second];
    let mut intermediate_polygons = Vec::with_capacity(word.len() - 1);
    let mut fixed_points_on_faces = Vec::with_capacity(word.len() - 1);
    let start_coords = match &outcome {
        ExactClosedWordOutcome::PositiveOrbit { start_coords, .. }
        | ExactClosedWordOutcome::NonStrictNoOrbit { start_coords, .. } => Some(start_coords),
        ExactClosedWordOutcome::ZeroActionNoOrbit {
            start_coords: Some(start_coords),
            ..
        } => Some(start_coords),
        _ => None,
    };

    for index in 0..word.len() - 1 {
        let pair = [word[index], word[index + 1]];
        let role = if index == 0 {
            "start"
        } else if index == word.len() - 2 {
            "end"
        } else {
            "intermediate"
        };
        let polygon = if index == 0 {
            exact_polygon_snapshot(
                start_pair,
                &tube.start_frame,
                &tube.start_polygon,
                &mut metrics,
            )
        } else if index == word.len() - 2 {
            end_polygon.clone()
        } else {
            let prefix = match build_tube(
                input.dual_vertices,
                input.facet_intersection_is_nonempty,
                input.omega_signs,
                &word[..index + 2],
                &mut metrics,
            ) {
                Ok(PolygonOutcomeTube::Nonempty(prefix)) => prefix,
                Ok(PolygonOutcomeTube::Empty) => return Ok(None),
                Err(()) => return Err(ExactClosedTubeError::UnsupportedSingularTransition),
            };
            let prefix = match restrict_tube_to_action_cutoff(prefix, action_cutoff, &mut metrics) {
                PolygonOutcomeTube::Nonempty(prefix) => prefix,
                PolygonOutcomeTube::Empty => return Ok(None),
            };
            let polygon = tube
                .start_polygon
                .image_under(&prefix.start_to_end, &mut metrics)
                .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?;
            exact_polygon_snapshot(pair, &prefix.end_frame, &polygon, &mut metrics)
        };
        intermediate_polygons.push(ExactTubeFaceSnapshot {
            pair,
            polygon,
            role: role.to_string(),
        });

        let point = start_coords.map(|start_coords| {
            if index == 0 {
                (*start_coords).clone()
            } else if index == word.len() - 2 {
                tube.start_to_end.apply(start_coords)
            } else {
                build_tube(
                    input.dual_vertices,
                    input.facet_intersection_is_nonempty,
                    input.omega_signs,
                    &word[..index + 2],
                    &mut metrics,
                )
                .ok()
                .and_then(|outcome| match outcome {
                    PolygonOutcomeTube::Nonempty(prefix) => {
                        match restrict_tube_to_action_cutoff(prefix, action_cutoff, &mut metrics) {
                            PolygonOutcomeTube::Nonempty(prefix) => {
                                Some(prefix.start_to_end.apply(start_coords))
                            }
                            PolygonOutcomeTube::Empty => None,
                        }
                    }
                    PolygonOutcomeTube::Empty => None,
                })
                .expect("exact intermediate tube exists when its polygon was built")
            }
        });
        fixed_points_on_faces.push(ExactTubeFaceFixedPointSnapshot {
            pair,
            role: role.to_string(),
            point,
        });
    }

    let inverse = inverse(&tube.start_to_end.matrix)
        .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?;
    let end_to_start = Affine2 {
        matrix: inverse.clone(),
        offset: [
            -mat_vec(&inverse, &tube.start_to_end.offset)[0].clone(),
            -mat_vec(&inverse, &tube.start_to_end.offset)[1].clone(),
        ],
    };
    let action_on_end = compose_scalar_with_affine(&tube.action_on_start, &end_to_start);
    let closed_orbit = match &outcome {
        ExactClosedWordOutcome::PositiveOrbit {
            action,
            start_coords,
        } => Some(exact_closed_orbit_snapshot(
            input.dual_vertices,
            &tube,
            start_coords,
            action.clone(),
        )?),
        _ => None,
    };

    Ok(Some(ExactTubeVisualizationSnapshot {
        sequence: word.clone(),
        start_pair,
        end_pair,
        start_polygon: exact_polygon_snapshot(
            start_pair,
            &tube.start_frame,
            &tube.start_polygon,
            &mut metrics,
        ),
        end_polygon,
        intermediate_polygons,
        fixed_points_on_faces,
        start_to_end: exact_affine_snapshot(&tube.start_to_end),
        end_to_start: exact_affine_snapshot(&end_to_start),
        action_on_start: exact_affine_scalar_snapshot(&tube.action_on_start),
        action_on_end: exact_affine_scalar_snapshot(&action_on_end),
        fixed_point,
        cutoff: action_cutoff.cloned(),
        closed_orbit,
        metrics,
    }))
}

pub(crate) fn resolve_closed_word_exact_with_action_cutoff(
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    action_cutoff: Option<&BigRational>,
) -> Result<(ExactClosedWordResult, ExactClosedTubeMetrics), ExactClosedTubeError> {
    validate_exact_input(input)?;
    if sigma.len() < 2
        || sigma.iter().any(|&facet| facet >= input.facet_count())
        || !all_distinct(sigma)
    {
        return Err(ExactClosedTubeError::InvalidWord);
    }

    let mut metrics = ExactClosedTubeMetrics::default();
    let result = classify_closed_tube(input, sigma, action_cutoff, &mut metrics)?;
    Ok((result, metrics))
}

pub(crate) fn validate_exact_input(
    input: &ExactFlatTubeInput<'_>,
) -> Result<(), ExactClosedTubeError> {
    // This is only structural validation for the exact tube resolver.  It does
    // not prove the formal theorem hypotheses, bounded irredundancy, or
    // agreement between caller-supplied intersection/sign matrices and the
    // facet normals.  Keep those assumptions visible in the README/support
    // ledger rather than hiding them behind this function name.
    let facet_count = input.facet_count();
    if facet_count < 2
        || input.facet_intersection_is_nonempty.shape() != (facet_count, facet_count)
        || input.omega_signs.shape() != (facet_count, facet_count)
    {
        return Err(ExactClosedTubeError::InvalidInput);
    }
    Ok(())
}

fn all_distinct(values: &[usize]) -> bool {
    let mut seen = std::collections::HashSet::new();
    values.iter().all(|value| seen.insert(*value))
}

fn classify_closed_tube(
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    action_cutoff: Option<&BigRational>,
    metrics: &mut ExactClosedTubeMetrics,
) -> Result<ExactClosedWordResult, ExactClosedTubeError> {
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    match build_tube(
        input.dual_vertices,
        input.facet_intersection_is_nonempty,
        input.omega_signs,
        &word,
        metrics,
    ) {
        Ok(PolygonOutcomeTube::Empty) => Ok(ExactClosedWordResult {
            word,
            outcome: ExactClosedWordOutcome::EmptyTube,
            tube_halfspaces: None,
            tube_vertices: None,
        }),
        Err(()) => Err(ExactClosedTubeError::UnsupportedSingularTransition),
        Ok(PolygonOutcomeTube::Nonempty(tube)) => {
            let tube = match restrict_tube_to_action_cutoff(tube, action_cutoff, metrics) {
                PolygonOutcomeTube::Empty => {
                    return Ok(ExactClosedWordResult {
                        word,
                        outcome: ExactClosedWordOutcome::EmptyTube,
                        tube_halfspaces: None,
                        tube_vertices: None,
                    });
                }
                PolygonOutcomeTube::Nonempty(tube) => tube,
            };
            let halfspaces = tube.start_polygon.halfspaces.len();
            let vertices = tube.start_polygon.vertices(metrics).len();
            let classification = solve_closed_tube(input.dual_vertices, &tube, metrics);
            Ok(ExactClosedWordResult {
                word,
                outcome: classification.into_public()?,
                tube_halfspaces: Some(halfspaces),
                tube_vertices: Some(vertices),
            })
        }
    }
}

impl ClosedClassification {
    fn into_public(self) -> Result<ExactClosedWordOutcome, ExactClosedTubeError> {
        match self {
            ClosedClassification::EmptyTube => Ok(ExactClosedWordOutcome::EmptyTube),
            ClosedClassification::ZeroActionNoOrbit {
                action,
                point,
                singular_status,
            } => Ok(ExactClosedWordOutcome::ZeroActionNoOrbit {
                action,
                start_coords: point,
                singular_status,
            }),
            ClosedClassification::NonStrictNoOrbit { action, point } => {
                Ok(ExactClosedWordOutcome::NonStrictNoOrbit {
                    action,
                    start_coords: point,
                })
            }
            ClosedClassification::PositiveOrbit { action, point } => {
                Ok(ExactClosedWordOutcome::PositiveOrbit {
                    action,
                    start_coords: point,
                })
            }
            ClosedClassification::UnsupportedPositiveSingular {
                singular_status,
                min_action,
                max_action,
            } => Ok(ExactClosedWordOutcome::UnsupportedPositiveSingular {
                singular_status,
                min_action,
                max_action,
            }),
            ClosedClassification::ConstructionError => {
                Err(ExactClosedTubeError::InternalInconsistentSingularSolve)
            }
        }
    }
}

fn solve_closed_tube(
    duals: &[Vec4],
    tube: &ExactTube,
    metrics: &mut ExactClosedTubeMetrics,
) -> ClosedClassification {
    if let Some(classification) = solve_length_three_closed_tube(duals, tube, metrics) {
        return classification;
    }

    let m = &tube.start_to_end.matrix;
    let lhs = [
        [&m[0][0] - R::one(), m[0][1].clone()],
        [m[1][0].clone(), &m[1][1] - R::one()],
    ];
    let rhs = [
        -tube.start_to_end.offset[0].clone(),
        -tube.start_to_end.offset[1].clone(),
    ];
    if let Some(inv) = inverse(&lhs) {
        let point = mat_vec(&inv, &rhs);
        if !tube.start_polygon.contains(&point, metrics) {
            return ClosedClassification::EmptyTube;
        }
        let action = tube.action_on_start.evaluate(&point);
        if action.is_positive() {
            match all_segment_times_are_positive(duals, tube, &point) {
                Some(true) => ClosedClassification::PositiveOrbit { action, point },
                Some(false) => ClosedClassification::NonStrictNoOrbit { action, point },
                None => ClosedClassification::ConstructionError,
            }
        } else {
            ClosedClassification::ZeroActionNoOrbit {
                action: Some(action),
                point: Some(point),
                singular_status: None,
            }
        }
    } else {
        solve_singular_fixed_tube(tube, &lhs, &rhs, metrics)
    }
}

fn solve_length_three_closed_tube(
    duals: &[Vec4],
    tube: &ExactTube,
    metrics: &mut ExactClosedTubeMetrics,
) -> Option<ClosedClassification> {
    if tube.sequence.len() != 5
        || tube.sequence[0] != tube.sequence[3]
        || tube.sequence[1] != tube.sequence[4]
    {
        return None;
    }
    let [i, j, k] = [tube.sequence[0], tube.sequence[1], tube.sequence[2]];
    let [a_i, a_j, a_k] = [duals.get(i)?, duals.get(j)?, duals.get(k)?];
    if !linear_independent3(a_i, a_j, a_k) {
        return None;
    }

    // For a primitive closed word (i,j,k,i,j), fixed points satisfy
    // tau_j a_j + tau_k a_k + tau_i a_i = 0.  Independence of the three
    // normals forces all segment times to be zero, so the fixed set is the
    // triple-facet intersection inside the start face and has zero action.
    let facet_k_on_start = Halfspace {
        normal: [
            dot4(a_k, &tube.start_frame.u),
            dot4(a_k, &tube.start_frame.v),
        ],
        rhs: R::one() - dot4(a_k, &tube.start_frame.base),
    };
    let fixed_polygon = match tube
        .start_polygon
        .intersect_halfspace(facet_k_on_start.clone(), metrics)
    {
        PolygonOutcome::Empty => return Some(ClosedClassification::EmptyTube),
        PolygonOutcome::Nonempty(polygon) => polygon,
    };
    let fixed_polygon = match fixed_polygon.intersect_halfspace(
        Halfspace {
            normal: [
                -facet_k_on_start.normal[0].clone(),
                -facet_k_on_start.normal[1].clone(),
            ],
            rhs: -facet_k_on_start.rhs,
        },
        metrics,
    ) {
        PolygonOutcome::Empty => return Some(ClosedClassification::EmptyTube),
        PolygonOutcome::Nonempty(polygon) => polygon,
    };
    if fixed_polygon.vertices(metrics).is_empty() {
        return Some(ClosedClassification::EmptyTube);
    }
    Some(ClosedClassification::ZeroActionNoOrbit {
        action: Some(R::zero()),
        point: None,
        singular_status: Some("length_three_zero_time"),
    })
}

fn linear_independent3(a: &Vec4, b: &Vec4, c: &Vec4) -> bool {
    for r in 0..4 {
        for s in (r + 1)..4 {
            for t in (s + 1)..4 {
                let det = &a[r] * (&b[s] * &c[t] - &b[t] * &c[s])
                    - &a[s] * (&b[r] * &c[t] - &b[t] * &c[r])
                    + &a[t] * (&b[r] * &c[s] - &b[s] * &c[r]);
                if !det.is_zero() {
                    return true;
                }
            }
        }
    }
    false
}

fn all_segment_times_are_positive(
    duals: &[Vec4],
    tube: &ExactTube,
    start_coords: &Vec2,
) -> Option<bool> {
    // The tube polygon is closed (`tau >= 0`), but returned orbits use the
    // strict displayed word and therefore require every segment time `tau > 0`.
    let segment_count = tube.sequence.len().checked_sub(2)?;
    let start_point = point_on_frame(&tube.start_frame, start_coords);
    let mut point = start_point.clone();

    for index in 0..segment_count {
        let current = tube.sequence[index + 1];
        let next = tube.sequence[index + 2];
        let current_dual = duals.get(current)?;
        let next_dual = duals.get(next)?;
        let reeb = scale4(&q(2), &j_times(current_dual));
        let denom = dot4(next_dual, &reeb);
        if denom.is_zero() {
            return None;
        }
        let tau = (R::one() - dot4(next_dual, &point)) / denom;
        if !tau.is_positive() {
            return Some(false);
        }
        point = add4(&point, &scale4(&tau, &reeb));
    }

    Some(point == start_point)
}

fn solve_singular_fixed_tube(
    tube: &ExactTube,
    lhs: &Mat2,
    rhs: &Vec2,
    metrics: &mut ExactClosedTubeMetrics,
) -> ClosedClassification {
    // Slow exact callers use this branch to understand singular fixed-point
    // equations, not to turn singular cases into certified capacity values.
    // Length-three structural zero-time cases are handled before this generic
    // singular branch.  The remaining branch is kept because exact resolution
    // of f64 error words and exact development checks need to distinguish:
    // - no fixed points in the searched domain;
    // - singular fixed sets whose action is everywhere nonpositive;
    // - singular fixed sets containing positive-action closed candidates.
    // Collapsing these cases into one rejection would make the slow exact path
    // less useful and would hide unsupported positive-action singular cases.
    let rows = [(&lhs[0], &rhs[0]), (&lhs[1], &rhs[1])];
    let nonzero: Vec<(&Vec2, &R)> = rows
        .into_iter()
        .filter(|(row, _)| !row[0].is_zero() || !row[1].is_zero())
        .collect();

    if nonzero.is_empty() {
        if !rhs[0].is_zero() || !rhs[1].is_zero() {
            return ClosedClassification::EmptyTube;
        }
        return singular_fixed_polygon_result(
            tube,
            tube.start_polygon.clone(),
            "singular_all_points",
            metrics,
        );
    }

    let (row, b) = nonzero[0];
    for (other_row, other_b) in nonzero.iter().skip(1) {
        if !det2(row, other_row).is_zero() {
            return ClosedClassification::ConstructionError;
        }
        if &row[0] * *other_b != &other_row[0] * b || &row[1] * *other_b != &other_row[1] * b {
            return ClosedClassification::EmptyTube;
        }
    }

    let fixed_polygon = match tube.start_polygon.intersect_halfspace(
        Halfspace {
            normal: row.clone(),
            rhs: b.clone(),
        },
        metrics,
    ) {
        PolygonOutcome::Empty => return ClosedClassification::EmptyTube,
        PolygonOutcome::Nonempty(polygon) => polygon,
    };
    let fixed_polygon = match fixed_polygon.intersect_halfspace(
        Halfspace {
            normal: [-row[0].clone(), -row[1].clone()],
            rhs: -b.clone(),
        },
        metrics,
    ) {
        PolygonOutcome::Empty => return ClosedClassification::EmptyTube,
        PolygonOutcome::Nonempty(polygon) => polygon,
    };
    singular_fixed_polygon_result(tube, fixed_polygon, "singular_fixed_line", metrics)
}

fn singular_fixed_polygon_result(
    tube: &ExactTube,
    fixed_polygon: ExactPolygon,
    singular_status: &'static str,
    metrics: &mut ExactClosedTubeMetrics,
) -> ClosedClassification {
    // The fixed set is convex and the action is affine on the start polygon.
    // Vertex signs therefore decide whether the searched fixed set contains a
    // positive-action candidate.  Positive singular candidates remain
    // unsupported; nonpositive singular fixed sets are reported as no-orbit
    // outcomes for the displayed strict word.
    let actions: Vec<R> = fixed_polygon
        .vertices(metrics)
        .iter()
        .map(|point| tube.action_on_start.evaluate(point))
        .collect();
    let min_action = actions.iter().min().cloned();
    let max_action = actions.iter().max().cloned();
    if actions.iter().any(R::is_positive) {
        ClosedClassification::UnsupportedPositiveSingular {
            singular_status,
            min_action,
            max_action,
        }
    } else {
        ClosedClassification::ZeroActionNoOrbit {
            action: max_action,
            point: None,
            singular_status: Some(singular_status),
        }
    }
}

impl AffineScalar {
    fn evaluate(&self, point: &Vec2) -> R {
        dot2(&self.coeff, point) + &self.constant
    }
}

impl Affine2 {
    fn apply(&self, point: &Vec2) -> Vec2 {
        add2(&mat_vec(&self.matrix, point), &self.offset)
    }
}

fn exact_polygon_snapshot(
    pair: [usize; 2],
    frame: &FaceFrame,
    polygon: &ExactPolygon,
    metrics: &mut ExactClosedTubeMetrics,
) -> ExactFacePolygonSnapshot {
    ExactFacePolygonSnapshot {
        pair,
        base: frame.base.clone(),
        u: frame.u.clone(),
        v: frame.v.clone(),
        vertices: polygon.vertices(metrics),
        inequalities: polygon
            .halfspaces
            .iter()
            .map(|halfspace| ExactHalfspaceSnapshot {
                normal: halfspace.normal.clone(),
                rhs: halfspace.rhs.clone(),
            })
            .collect(),
    }
}

fn exact_affine_snapshot(affine: &Affine2) -> ExactAffineSnapshot {
    ExactAffineSnapshot {
        matrix: affine.matrix.clone(),
        offset: affine.offset.clone(),
    }
}

fn exact_affine_scalar_snapshot(affine: &AffineScalar) -> ExactAffineScalarSnapshot {
    ExactAffineScalarSnapshot {
        coeff: affine.coeff.clone(),
        constant: affine.constant.clone(),
    }
}

fn compose_scalar_with_affine(scalar: &AffineScalar, affine: &Affine2) -> AffineScalar {
    let coeff = mat_transpose_vec(&affine.matrix, &scalar.coeff);
    AffineScalar {
        coeff,
        constant: dot2(&scalar.coeff, &affine.offset) + &scalar.constant,
    }
}

fn exact_fixed_point_snapshot(outcome: &ExactClosedWordOutcome) -> ExactFixedPointSnapshot {
    match outcome {
        ExactClosedWordOutcome::EmptyTube => ExactFixedPointSnapshot {
            status: "empty_tube".to_string(),
            point: None,
            action: None,
            singular_status: None,
            min_action: None,
            max_action: None,
        },
        ExactClosedWordOutcome::ZeroActionNoOrbit {
            action,
            start_coords,
            singular_status,
        } => ExactFixedPointSnapshot {
            status: singular_status
                .unwrap_or("point_inside_start_polygon")
                .to_string(),
            point: start_coords.clone(),
            action: action.clone(),
            singular_status: singular_status.map(str::to_string),
            min_action: None,
            max_action: None,
        },
        ExactClosedWordOutcome::NonStrictNoOrbit {
            action,
            start_coords,
        } => ExactFixedPointSnapshot {
            status: "point_inside_start_polygon_non_strict_time".to_string(),
            point: Some(start_coords.clone()),
            action: Some(action.clone()),
            singular_status: None,
            min_action: None,
            max_action: None,
        },
        ExactClosedWordOutcome::PositiveOrbit {
            action,
            start_coords,
        } => ExactFixedPointSnapshot {
            status: "point_inside_start_polygon".to_string(),
            point: Some(start_coords.clone()),
            action: Some(action.clone()),
            singular_status: None,
            min_action: None,
            max_action: None,
        },
        ExactClosedWordOutcome::UnsupportedPositiveSingular {
            singular_status,
            min_action,
            max_action,
        } => ExactFixedPointSnapshot {
            status: singular_status.to_string(),
            point: None,
            action: None,
            singular_status: Some(singular_status.to_string()),
            min_action: min_action.clone(),
            max_action: max_action.clone(),
        },
    }
}

fn exact_closed_orbit_snapshot(
    duals: &[Vec4],
    tube: &ExactTube,
    start_coords: &Vec2,
    action: R,
) -> Result<ExactClosedOrbitSnapshot, ExactClosedTubeError> {
    let cyclic_word = tube
        .sequence
        .get(
            ..tube
                .sequence
                .len()
                .checked_sub(2)
                .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?,
        )
        .ok_or(ExactClosedTubeError::InternalInconsistentSingularSolve)?;
    let start_point = point_on_frame(&tube.start_frame, start_coords);
    let mut point = start_point.clone();
    let mut breakpoints = vec![start_point];
    let mut segment_times = Vec::with_capacity(cyclic_word.len());
    for index in 0..cyclic_word.len() {
        let current = cyclic_word[(index + 1) % cyclic_word.len()];
        let next = cyclic_word[(index + 2) % cyclic_word.len()];
        let current_dual = duals
            .get(current)
            .ok_or(ExactClosedTubeError::InvalidWord)?;
        let next_dual = duals.get(next).ok_or(ExactClosedTubeError::InvalidWord)?;
        let reeb = scale4(&q(2), &j_times(current_dual));
        let denom = dot4(next_dual, &reeb);
        if denom.is_zero() {
            return Err(ExactClosedTubeError::UnsupportedSingularTransition);
        }
        let tau = (R::one() - dot4(next_dual, &point)) / denom;
        segment_times.push(tau.clone());
        point = add4(&point, &scale4(&tau, &reeb));
        if index + 1 < cyclic_word.len() {
            breakpoints.push(point.clone());
        }
    }
    if point != breakpoints[0] || segment_times.iter().any(|time| !time.is_positive()) {
        return Err(ExactClosedTubeError::InternalInconsistentSingularSolve);
    }
    Ok(ExactClosedOrbitSnapshot {
        facets: cyclic_word.to_vec(),
        action,
        breakpoints,
        segment_times,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
    use crate::algorithms::flow_graph::exact_search::{
        search_closed_orbits_exact, ExactActionCutoffPolicy, ExactFlowGraphOrbit,
    };
    use crate::algorithms::hk2017::for_each_sigma_pruned_by_transition;
    use crate::algorithms::hk2017::solve_pruned_hk2017_candidates;
    use crate::algorithms::{
        aggregate_certified_orbits_with_dual_vertices_exact, CertifiedOrbitSetMode,
    };
    use crate::exact::{
        exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega_signs_exact,
    };
    use crate::geom::rational_arithmetic::f64_to_rational;
    use crate::random::generate_dual_vertices;
    use nalgebra::Vector4;
    use num_traits::ToPrimitive;
    use std::collections::BTreeMap;

    fn r(n: i64) -> R {
        q(n)
    }

    fn frac(numer: i64, denom: i64) -> R {
        BigRational::new(numer.into(), denom.into())
    }

    fn halfspace(nx: i64, ny: i64, rhs: i64) -> Halfspace {
        Halfspace {
            normal: [r(nx), r(ny)],
            rhs: r(rhs),
        }
    }

    fn unit_square(metrics: &mut ExactClosedTubeMetrics) -> ExactPolygon {
        match ExactPolygon::new(
            vec![
                halfspace(1, 0, 1),
                halfspace(-1, 0, 0),
                halfspace(0, 1, 1),
                halfspace(0, -1, 0),
            ],
            metrics,
        ) {
            PolygonOutcome::Nonempty(polygon) => polygon,
            PolygonOutcome::Empty => panic!("unit square should be nonempty"),
        }
    }

    fn polygon(
        halfspaces: Vec<Halfspace>,
        metrics: &mut ExactClosedTubeMetrics,
    ) -> Option<ExactPolygon> {
        match ExactPolygon::new(halfspaces, metrics) {
            PolygonOutcome::Nonempty(polygon) => Some(polygon),
            PolygonOutcome::Empty => None,
        }
    }

    struct ExactCaseData {
        dual_vertices_f64: Vec<Vector4<f64>>,
        dual_vertices: Vec<[BigRational; 4]>,
        facet_intersection_is_nonempty: DMatrix<bool>,
        omega_signs: DMatrix<i8>,
    }

    impl ExactCaseData {
        fn input(&self) -> ExactFlatTubeInput<'_> {
            ExactFlatTubeInput {
                dual_vertices: &self.dual_vertices,
                facet_intersection_is_nonempty: &self.facet_intersection_is_nonempty,
                omega_signs: &self.omega_signs,
            }
        }
    }

    fn deterministic_random_exact_case(facet_count: usize, attempt: u64) -> ExactCaseData {
        let dual_vertices_f64 = generate_dual_vertices(facet_count, 0.5, 2.0, 20260605, attempt)
            .expect("deterministic random polytope");
        let dual_vertices: Vec<[BigRational; 4]> = dual_vertices_f64
            .iter()
            .map(|a| std::array::from_fn(|idx| f64_to_rational(a[idx])))
            .collect();
        let dual_vertex_vectors: Vec<Vector4<BigRational>> = dual_vertices
            .iter()
            .map(|a| Vector4::new(a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()))
            .collect();
        let exact = exact_vertices_with_incidence(&dual_vertex_vectors)
            .expect("deterministic random polytope exact vertices");
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_exact(&exact.vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vertex_vectors);
        ExactCaseData {
            dual_vertices_f64,
            dual_vertices,
            facet_intersection_is_nonempty,
            omega_signs,
        }
    }

    fn certified_qp_capacity(
        case: &ExactCaseData,
        action_gap_exact: BigRational,
    ) -> crate::algorithms::CertifiedOrbitSearchResult {
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &case.facet_intersection_is_nonempty,
            &case.omega_signs,
        );
        let (orbits, iterations) =
            solve_pruned_hk2017_candidates(&case.dual_vertices_f64, &transition_is_allowed)
                .expect("certified QP candidate solve");
        aggregate_certified_orbits_with_dual_vertices_exact(
            &case.dual_vertices,
            orbits,
            iterations,
            action_gap_exact,
            CertifiedOrbitSetMode::GapWindow,
        )
        .expect("certified QP aggregation")
    }

    fn canonical_cyclic_word(word: &[usize]) -> Vec<usize> {
        assert!(!word.is_empty(), "cyclic word must be nonempty");
        (0..word.len())
            .map(|offset| {
                word.iter()
                    .cycle()
                    .skip(offset)
                    .take(word.len())
                    .copied()
                    .collect::<Vec<_>>()
            })
            .min()
            .expect("nonempty rotations")
    }

    fn orbit_map(orbits: &[ExactFlowGraphOrbit]) -> BTreeMap<Vec<usize>, BigRational> {
        let mut map = BTreeMap::new();
        for orbit in orbits {
            let canonical = canonical_cyclic_word(&orbit.facets);
            match map.insert(canonical.clone(), orbit.action.clone()) {
                Some(previous) => assert_eq!(
                    previous, orbit.action,
                    "cyclic duplicate {canonical:?} had inconsistent actions"
                ),
                None => {}
            }
        }
        map
    }

    fn exact_retained_words_by_full_resolution(
        input: &ExactFlatTubeInput<'_>,
        action_cutoff: &BigRational,
    ) -> BTreeMap<Vec<usize>, BigRational> {
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            input.facet_intersection_is_nonempty,
            input.omega_signs,
        );
        let mut expected = BTreeMap::new();
        let mut failure = None;
        for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
            if failure.is_some() {
                return;
            }
            let result = match resolve_closed_word_exact(input, sigma) {
                Ok((result, _metrics)) => result,
                Err(error) => {
                    failure = Some(format!("exact resolver failed for {sigma:?}: {error:?}"));
                    return;
                }
            };
            match result.outcome {
                ExactClosedWordOutcome::EmptyTube
                | ExactClosedWordOutcome::ZeroActionNoOrbit { .. }
                | ExactClosedWordOutcome::NonStrictNoOrbit { .. } => {}
                ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                    if action <= *action_cutoff {
                        let canonical = canonical_cyclic_word(sigma);
                        match expected.insert(canonical.clone(), action.clone()) {
                            Some(previous) => assert_eq!(
                                previous, action,
                                "cyclic duplicate {canonical:?} had inconsistent actions"
                            ),
                            None => {}
                        }
                    }
                }
                ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => {
                    failure = Some(format!(
                        "accepted-polytopes suite hit positive-action singular word {sigma:?}"
                    ));
                }
            }
        });
        if let Some(failure) = failure {
            panic!("{failure}");
        }
        expected
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum ExactOutcomeBucket {
        EmptyTube,
        ZeroActionRegular,
        ZeroActionLengthThree,
        ZeroActionOtherSingular,
        NonStrictNoOrbit,
        PositiveOrbit,
        UnsupportedPositiveSingular,
    }

    fn exact_outcome_bucket(outcome: &ExactClosedWordOutcome) -> ExactOutcomeBucket {
        match outcome {
            ExactClosedWordOutcome::EmptyTube => ExactOutcomeBucket::EmptyTube,
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some("length_three_zero_time"),
                ..
            } => ExactOutcomeBucket::ZeroActionLengthThree,
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some(_),
                ..
            } => ExactOutcomeBucket::ZeroActionOtherSingular,
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: None,
                ..
            } => ExactOutcomeBucket::ZeroActionRegular,
            ExactClosedWordOutcome::NonStrictNoOrbit { .. } => ExactOutcomeBucket::NonStrictNoOrbit,
            ExactClosedWordOutcome::PositiveOrbit { .. } => ExactOutcomeBucket::PositiveOrbit,
            ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => {
                ExactOutcomeBucket::UnsupportedPositiveSingular
            }
        }
    }

    fn assert_bucket_count(
        counts: &BTreeMap<(usize, ExactOutcomeBucket), usize>,
        word_len: usize,
        bucket: ExactOutcomeBucket,
        expected: usize,
    ) {
        assert_eq!(
            counts.get(&(word_len, bucket)).copied().unwrap_or(0),
            expected,
            "unexpected count for length {word_len} bucket {bucket:?}"
        );
    }

    fn assert_exact_search_supports_p1_p2(
        case_name: &str,
        case: &ExactCaseData,
        action_threshold: BigRational,
    ) {
        let input = case.input();
        let flow = search_closed_orbits_exact(
            &input,
            action_threshold.clone(),
            ExactActionCutoffPolicy::Disabled,
        )
        .unwrap_or_else(|error| panic!("{case_name}: exact flow-graph search failed: {error:?}"));
        let certified_qp = certified_qp_capacity(case, action_threshold.clone());
        assert_eq!(
            flow.capacity_action, certified_qp.capacity_exact,
            "{case_name}: exact flow-graph capacity disagrees with certified QP"
        );
        let action_cutoff = &flow.capacity_action + &action_threshold;
        assert!(
            flow.orbits
                .iter()
                .all(|orbit| orbit.action <= action_cutoff),
            "{case_name}: exact search retained an orbit above capacity + threshold"
        );
        let actual = orbit_map(&flow.orbits);
        let expected = exact_retained_words_by_full_resolution(&input, &action_cutoff);
        assert_eq!(
            actual, expected,
            "{case_name}: retained cyclic flow-graph words do not match full exact resolution"
        );
    }

    #[test]
    fn polygon_and_halfspace_can_stay_nonempty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        let outcome = square.intersect_halfspace(halfspace(1, 1, 1), &mut metrics);
        let PolygonOutcome::Nonempty(cut_square) = outcome else {
            panic!("cut square should be nonempty");
        };
        assert!(cut_square.contains(&[r(0), r(0)], &mut metrics));
        assert!(cut_square.contains(&[frac(1, 2), frac(1, 2)], &mut metrics));
        assert!(!cut_square.contains(&[r(1), r(1)], &mut metrics));
    }

    #[test]
    fn polygon_and_halfspace_can_become_empty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        let outcome = square.intersect_halfspace(halfspace(-1, 0, -2), &mut metrics);
        assert!(matches!(outcome, PolygonOutcome::Empty));
    }

    #[test]
    fn polygon_and_polygon_can_stay_nonempty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        let triangle = polygon(
            vec![
                halfspace(1, 0, 1),
                halfspace(-1, 0, 0),
                halfspace(0, 1, 1),
                halfspace(0, -1, 0),
                halfspace(1, 1, 1),
            ],
            &mut metrics,
        )
        .expect("triangle should be nonempty");
        let outcome = square.intersect_polygon(&triangle, &mut metrics);
        let PolygonOutcome::Nonempty(intersection) = outcome else {
            panic!("square cap triangle should be nonempty");
        };
        assert!(intersection.contains(&[frac(1, 2), frac(1, 2)], &mut metrics));
        assert!(!intersection.contains(&[r(1), r(1)], &mut metrics));
    }

    #[test]
    fn polygon_and_polygon_can_become_empty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        let disjoint_square = polygon(
            vec![
                halfspace(1, 0, 3),
                halfspace(-1, 0, -2),
                halfspace(0, 1, 1),
                halfspace(0, -1, 0),
            ],
            &mut metrics,
        )
        .expect("disjoint square should be nonempty by itself");
        let outcome = square.intersect_polygon(&disjoint_square, &mut metrics);
        assert!(matches!(outcome, PolygonOutcome::Empty));
    }

    #[test]
    fn feasible_segment_counts_as_nonempty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let segment = polygon(
            vec![
                halfspace(1, 0, 1),
                halfspace(-1, 0, 0),
                halfspace(0, 1, 0),
                halfspace(0, -1, 0),
            ],
            &mut metrics,
        );
        assert!(segment.is_some());
    }

    #[test]
    fn feasible_point_counts_as_nonempty() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let point = polygon(
            vec![
                halfspace(1, 0, 0),
                halfspace(-1, 0, 0),
                halfspace(0, 1, 0),
                halfspace(0, -1, 0),
            ],
            &mut metrics,
        );
        assert!(point.is_some());
    }

    #[test]
    fn contains_uses_closed_membership() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        assert!(square.contains(&[r(0), r(0)], &mut metrics));
        assert!(square.contains(&[frac(1, 2), frac(1, 2)], &mut metrics));
        assert!(square.contains(&[r(1), r(1)], &mut metrics));
        assert!(!square.contains(&[r(2), r(1)], &mut metrics));
    }

    #[test]
    fn redundant_inequality_preserves_nonempty_status() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let square = unit_square(&mut metrics);
        let outcome = square.intersect_halfspace(halfspace(1, 0, 2), &mut metrics);
        assert!(matches!(outcome, PolygonOutcome::Nonempty(_)));
    }

    #[test]
    fn exact_closed_word_resolves_known_positive_f7_attempt31() {
        let case = deterministic_random_exact_case(7, 31);
        let (result, metrics) =
            resolve_closed_word_exact(&case.input(), &[0, 1, 5, 6, 4, 2]).unwrap();

        assert_eq!(result.word, vec![0, 1, 5, 6, 4, 2, 0, 1]);
        assert!(metrics.emptiness_checks > 0);
        match result.outcome {
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                let action_f64 = action.to_f64().expect("action to f64");
                assert!((action_f64 - 9.4722557649991).abs() < 1e-10);
            }
            other => panic!("expected orbit with action > 0, got {other:?}"),
        }
    }

    #[test]
    fn exact_visualization_snapshot_exposes_f6_tube_and_orbit() {
        let case = deterministic_random_exact_case(6, 3);
        let snapshot =
            closed_tube_visualization_snapshot_exact(&case.input(), &[1, 2, 4, 5, 3], None)
                .expect("exact visualization snapshot")
                .expect("selected exact tube should be nonempty");

        assert_eq!(snapshot.sequence, vec![1, 2, 4, 5, 3, 1, 2]);
        assert_eq!(snapshot.intermediate_polygons.len(), 6);
        assert_eq!(snapshot.fixed_points_on_faces.len(), 6);
        assert_eq!(snapshot.metrics.images, 5);
        match snapshot.closed_orbit {
            Some(ExactClosedOrbitSnapshot {
                action,
                breakpoints,
                segment_times,
                ..
            }) => {
                assert!(action.is_positive());
                assert_eq!(breakpoints.len(), 5);
                assert_eq!(segment_times.len(), 5);
                assert!(segment_times.iter().all(R::is_positive));
            }
            None => panic!("selected exact tube should expose a positive orbit"),
        }
    }

    #[test]
    fn exact_positive_closed_word_reconstructs_positive_time_reeb_orbit_conditions() {
        let case = deterministic_random_exact_case(6, 3);
        assert_positive_closed_word_reconstructs(
            "generated_F6_attempt3_capacity_word",
            &case,
            &[1, 2, 4, 5, 3],
            true,
        );

        let case = deterministic_random_exact_case(7, 31);
        assert_positive_closed_word_reconstructs(
            "generated_F7_attempt31_capacity_word",
            &case,
            &[0, 1, 5, 6, 4, 2],
            true,
        );
    }

    #[test]
    fn primitive_exact_tubes_have_direct_segment_semantics() {
        let case = deterministic_random_exact_case(6, 3);
        assert_primitive_tubes_have_segment_semantics(
            "generated_F6_attempt3_capacity_word",
            &case,
            &[1, 2, 4, 5, 3, 1, 2],
        );
    }

    #[test]
    fn balanced_exact_tube_matches_left_associative_concatenation() {
        let case = deterministic_random_exact_case(6, 3);
        assert_balanced_tube_matches_left_associative(
            "generated_F6_attempt3_capacity_word",
            &case,
            &[1, 2, 4, 5, 3, 1, 2],
        );
    }

    #[test]
    fn exact_closed_word_resolves_zero_action_f7_attempt31() {
        let case = deterministic_random_exact_case(7, 31);
        let (result, _) = resolve_closed_word_exact(&case.input(), &[0, 4, 2, 6]).unwrap();

        match result.outcome {
            ExactClosedWordOutcome::ZeroActionNoOrbit { action, .. } => {
                assert_eq!(action, Some(BigRational::zero()));
            }
            other => panic!("expected zero-action no-orbit, got {other:?}"),
        }
    }

    #[test]
    fn length_three_closed_word_is_zero_time_no_orbit() {
        let case = deterministic_random_exact_case(5, 60);
        let (result, _) = resolve_closed_word_exact(&case.input(), &[0, 2, 4]).unwrap();

        match result.outcome {
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                action,
                singular_status: Some("length_three_zero_time"),
                ..
            } => {
                assert_eq!(action, Some(BigRational::zero()));
            }
            other => panic!("expected length-three zero-time no-orbit, got {other:?}"),
        }
    }

    #[test]
    fn exact_closed_word_empty_when_same_sigma_qp_has_critical_point_f6_attempt3() {
        let case = deterministic_random_exact_case(6, 3);
        let (result, _) = resolve_closed_word_exact(&case.input(), &[1, 5, 2, 4, 3]).unwrap();

        assert!(matches!(result.outcome, ExactClosedWordOutcome::EmptyTube));
    }

    #[test]
    fn exact_closed_word_action_cutoff_can_empty_higher_action_word() {
        let case = deterministic_random_exact_case(7, 31);
        let (result, metrics) = resolve_closed_word_exact_with_action_cutoff(
            &case.input(),
            &[0, 4, 1, 3, 2, 6],
            Some(&q(10)),
        )
        .unwrap();

        assert!(metrics.action_cutoff_intersections > 0);
        assert!(matches!(result.outcome, ExactClosedWordOutcome::EmptyTube));
    }

    #[test]
    fn exact_segment_time_filter_rejects_zero_time_boundary_point() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let duals = vec![
            [r(0), r(0), r(0), r(0)],
            [r(0), r(0), r(1), r(0)],
            [r(1), r(0), r(0), r(0)],
        ];
        let start_frame = FaceFrame {
            first: 0,
            second: 1,
            base: [r(1), r(0), r(0), r(0)],
            u: [r(0), r(1), r(0), r(0)],
            v: [r(0), r(0), r(0), r(1)],
            free: [1, 3],
        };
        let tube = ExactTube {
            sequence: vec![0, 1, 2],
            start_frame: start_frame.clone(),
            end_frame: start_frame,
            start_polygon: polygon(
                vec![
                    halfspace(1, 0, 1),
                    halfspace(-1, 0, 1),
                    halfspace(0, 1, 1),
                    halfspace(0, -1, 1),
                ],
                &mut metrics,
            )
            .unwrap(),
            start_to_end: Affine2 {
                matrix: [[r(0), r(0)], [r(0), r(0)]],
                offset: [r(0), r(0)],
            },
            action_on_start: AffineScalar {
                coeff: [r(0), r(0)],
                constant: r(1),
            },
        };

        assert_eq!(
            all_segment_times_are_positive(&duals, &tube, &[r(0), r(0)]),
            Some(false)
        );
        assert!(matches!(
            solve_closed_tube(&duals, &tube, &mut metrics),
            ClosedClassification::NonStrictNoOrbit { .. }
        ));
    }

    #[test]
    fn exact_singular_positive_fixed_set_is_unsupported() {
        let mut metrics = ExactClosedTubeMetrics::default();
        let frame = FaceFrame {
            first: 0,
            second: 1,
            base: [r(0), r(0), r(0), r(0)],
            u: [r(1), r(0), r(0), r(0)],
            v: [r(0), r(1), r(0), r(0)],
            free: [0, 1],
        };
        let tube = ExactTube {
            sequence: vec![0, 1, 0],
            start_frame: frame.clone(),
            end_frame: frame,
            start_polygon: unit_square(&mut metrics),
            start_to_end: Affine2 {
                matrix: [[r(1), r(0)], [r(0), r(1)]],
                offset: [r(0), r(0)],
            },
            action_on_start: AffineScalar {
                coeff: [r(0), r(0)],
                constant: r(1),
            },
        };

        assert!(matches!(
            solve_closed_tube(&[], &tube, &mut metrics),
            ClosedClassification::UnsupportedPositiveSingular {
                singular_status: "singular_all_points",
                min_action,
                max_action,
            } if min_action == Some(r(1)) && max_action == Some(r(1))
        ));
    }

    #[test]
    fn generated_f5_outcome_taxonomy_documents_supported_boundary() {
        let case = deterministic_random_exact_case(5, 60);
        let input = case.input();
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            input.facet_intersection_is_nonempty,
            input.omega_signs,
        );
        let mut counts = BTreeMap::new();

        for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
            let (result, _) = resolve_closed_word_exact(&input, sigma)
                .unwrap_or_else(|error| panic!("exact resolver failed for {sigma:?}: {error:?}"));
            *counts
                .entry((sigma.len(), exact_outcome_bucket(&result.outcome)))
                .or_insert(0) += 1;
        });

        assert_bucket_count(&counts, 3, ExactOutcomeBucket::ZeroActionLengthThree, 4);
        assert_bucket_count(&counts, 4, ExactOutcomeBucket::ZeroActionRegular, 3);
        assert_bucket_count(&counts, 5, ExactOutcomeBucket::EmptyTube, 2);
        assert_bucket_count(&counts, 5, ExactOutcomeBucket::PositiveOrbit, 1);

        assert_bucket_count(&counts, 3, ExactOutcomeBucket::ZeroActionOtherSingular, 0);
        assert_bucket_count(&counts, 4, ExactOutcomeBucket::ZeroActionOtherSingular, 0);
        assert_bucket_count(&counts, 5, ExactOutcomeBucket::ZeroActionOtherSingular, 0);
        assert_bucket_count(
            &counts,
            3,
            ExactOutcomeBucket::UnsupportedPositiveSingular,
            0,
        );
        assert_bucket_count(
            &counts,
            4,
            ExactOutcomeBucket::UnsupportedPositiveSingular,
            0,
        );
        assert_bucket_count(
            &counts,
            5,
            ExactOutcomeBucket::UnsupportedPositiveSingular,
            0,
        );
        assert_bucket_count(&counts, 3, ExactOutcomeBucket::NonStrictNoOrbit, 0);
        assert_bucket_count(&counts, 4, ExactOutcomeBucket::NonStrictNoOrbit, 0);
        assert_bucket_count(&counts, 5, ExactOutcomeBucket::NonStrictNoOrbit, 0);

        let total_words: usize = counts.values().sum();
        assert_eq!(total_words, 10);
    }

    #[test]
    fn exact_accepted_f5_polytope_supports_p1_p2() {
        let case = deterministic_random_exact_case(5, 60);
        assert_exact_search_supports_p1_p2("generated_F5_attempt60_gap0", &case, q(0));
        assert_exact_search_supports_p1_p2("generated_F5_attempt60_gap1", &case, q(1));
    }

    #[test]
    fn exact_accepted_f6_polytope_supports_p1_p2() {
        let case = deterministic_random_exact_case(6, 3);
        assert_exact_search_supports_p1_p2("generated_F6_attempt3_gap0", &case, q(0));
        assert_exact_search_supports_p1_p2("generated_F6_attempt3_gap1", &case, q(1));
    }

    #[test]
    #[ignore = "exhaustive exact F7 baseline takes about 60s in release; run before changing exact search semantics"]
    fn exact_accepted_f7_polytope_supports_p1_p2_gap0() {
        let case = deterministic_random_exact_case(7, 31);
        assert_exact_search_supports_p1_p2("generated_F7_attempt31_gap0", &case, q(0));
    }

    #[test]
    #[ignore = "exhaustive exact F7 baseline takes about 60s in release; run before changing exact search semantics"]
    fn exact_search_finds_f7_attempt31_capacity_word() {
        let case = deterministic_random_exact_case(7, 31);
        let result = search_closed_orbits_exact(
            &case.input(),
            BigRational::zero(),
            ExactActionCutoffPolicy::Disabled,
        )
        .unwrap();

        assert!(result.checked_word_count > 0);
        assert!(result.empty_or_no_orbit_count > 0);
        assert_eq!(result.orbits[0].facets, vec![0, 1, 5, 6, 4, 2]);
        let capacity = result.capacity_action.to_f64().expect("capacity to f64");
        assert!((capacity - 9.4722557649991).abs() < 1e-10);
        assert!(
            result
                .orbits
                .iter()
                .all(|orbit| orbit.action == result.capacity_action),
            "zero threshold should retain only capacity-action orbits"
        );
    }

    #[test]
    #[ignore = "exhaustive exact F7 threshold baseline takes about 60s in release; run before changing exact search semantics"]
    fn exact_accepted_f7_polytope_supports_p1_p2_positive_gap() {
        let case = deterministic_random_exact_case(7, 31);
        assert_exact_search_supports_p1_p2("generated_F7_attempt31_gap4", &case, q(4));
    }

    #[test]
    #[ignore = "exhaustive exact F7 threshold baseline takes about 60s in release; run before changing exact search semantics"]
    fn exact_search_threshold_retains_higher_f7_attempt31_word() {
        let case = deterministic_random_exact_case(7, 31);
        let result =
            search_closed_orbits_exact(&case.input(), q(4), ExactActionCutoffPolicy::Disabled)
                .unwrap();

        assert!(result
            .orbits
            .iter()
            .any(|orbit| orbit.facets == vec![0, 4, 1, 3, 2, 6]));
        assert!(result.orbits.iter().all(|orbit| {
            orbit.action <= result.capacity_action.clone() + result.action_threshold.clone()
        }));
    }

    #[test]
    fn exact_search_finds_f6_attempt3_capacity_with_same_sigma_qp_tube_difference() {
        let case = deterministic_random_exact_case(6, 3);
        let result = search_closed_orbits_exact(
            &case.input(),
            BigRational::zero(),
            ExactActionCutoffPolicy::Disabled,
        )
        .unwrap();

        assert_eq!(result.orbits[0].facets, vec![1, 2, 4, 5, 3]);
        let capacity = result.capacity_action.to_f64().expect("capacity to f64");
        assert!((capacity - 39.35654212420111).abs() < 1e-10);
    }

    #[test]
    fn exact_search_cutoff_policy_enabled_matches_disabled_f6_attempt3() {
        let case = deterministic_random_exact_case(6, 3);
        let baseline =
            search_closed_orbits_exact(&case.input(), q(1), ExactActionCutoffPolicy::Disabled)
                .unwrap();
        let cutoff =
            search_closed_orbits_exact(&case.input(), q(1), ExactActionCutoffPolicy::Enabled)
                .unwrap();

        assert_eq!(cutoff.capacity_action, baseline.capacity_action);
        assert_eq!(cutoff.orbits, baseline.orbits);
        assert_eq!(cutoff.checked_word_count, baseline.checked_word_count);
        assert_eq!(baseline.action_cutoff_word_count, 0);
        assert_eq!(baseline.action_cutoff_intersection_count, 0);
        assert!(cutoff.action_cutoff_word_count > 0);
    }

    fn assert_positive_closed_word_reconstructs(
        case_name: &str,
        case: &ExactCaseData,
        sigma: &[usize],
        require_strictly_positive_segment_times: bool,
    ) {
        let duals = exact_case_dual_vectors(case);
        let mut word = sigma.to_vec();
        word.push(sigma[0]);
        word.push(sigma[1]);
        let mut metrics = ExactClosedTubeMetrics::default();
        let tube = match build_tube(
            &duals,
            &case.facet_intersection_is_nonempty,
            &case.omega_signs,
            &word,
            &mut metrics,
        )
        .expect("supported exact closed word")
        {
            PolygonOutcomeTube::Nonempty(tube) => tube,
            PolygonOutcomeTube::Empty => panic!("{case_name}: closed tube was empty"),
        };
        let ClosedClassification::PositiveOrbit {
            action,
            point: start_coords,
        } = solve_closed_tube(&duals, &tube, &mut metrics)
        else {
            panic!("{case_name}: closed tube did not solve to a positive orbit");
        };

        assert!(
            tube.start_polygon.contains(&start_coords, &mut metrics),
            "{case_name}: fixed point is outside the start polygon"
        );
        let start_point = point_from_frame_coords(&tube.start_frame, &start_coords);
        let mut point = start_point.clone();
        let mut breakpoints = vec![point.clone()];
        let mut action_sum = R::zero();
        for segment_index in 0..word.len() - 2 {
            let previous = word[segment_index];
            let current = word[segment_index + 1];
            let next = word[segment_index + 2];
            assert_eq!(
                dot4(&duals[previous], &point),
                R::one(),
                "{case_name}: breakpoint {segment_index} is not on previous facet"
            );
            assert_eq!(
                dot4(&duals[current], &point),
                R::one(),
                "{case_name}: breakpoint {segment_index} is not on current facet"
            );
            assert_inside_polytope(case_name, &duals, &point);

            let reeb = scale4(&q(2), &j_times(&duals[current]));
            let denom = dot4(&duals[next], &reeb);
            assert!(
                !denom.is_zero(),
                "{case_name}: segment {segment_index} has singular transition"
            );
            let tau = (R::one() - dot4(&duals[next], &point)) / denom;
            assert!(
                !tau.is_negative(),
                "{case_name}: segment {segment_index} has negative time {tau}"
            );
            if require_strictly_positive_segment_times {
                assert!(
                    tau.is_positive(),
                    "{case_name}: segment {segment_index} has zero time but this check certifies theorem-level positive-time evidence"
                );
            }
            point = add4(&point, &scale4(&tau, &reeb));
            action_sum += tau;
            breakpoints.push(point.clone());
        }
        assert_eq!(
            point, start_point,
            "{case_name}: reconstructed orbit does not close"
        );
        assert_eq!(
            action_sum, action,
            "{case_name}: reconstructed action differs from closed-tube action"
        );
        for (breakpoint_index, breakpoint) in breakpoints.iter().enumerate() {
            assert_inside_polytope(case_name, &duals, breakpoint);
            let first = word[breakpoint_index];
            let second = word[breakpoint_index + 1];
            assert_eq!(
                dot4(&duals[first], breakpoint),
                R::one(),
                "{case_name}: breakpoint {breakpoint_index} is not on first two-face facet"
            );
            assert_eq!(
                dot4(&duals[second], breakpoint),
                R::one(),
                "{case_name}: breakpoint {breakpoint_index} is not on second two-face facet"
            );
        }
    }

    fn assert_inside_polytope(case_name: &str, duals: &[Vec4], point: &Vec4) {
        for (facet, dual) in duals.iter().enumerate() {
            assert!(
                dot4(dual, point) <= R::one(),
                "{case_name}: point violates facet {facet}"
            );
        }
    }

    fn exact_case_dual_vectors(case: &ExactCaseData) -> Vec<Vec4> {
        case.dual_vertices
            .iter()
            .map(|a| [a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()])
            .collect()
    }

    fn point_from_frame_coords(frame: &FaceFrame, coords: &Vec2) -> Vec4 {
        add4(
            &frame.base,
            &add4(&scale4(&coords[0], &frame.u), &scale4(&coords[1], &frame.v)),
        )
    }

    fn assert_primitive_tubes_have_segment_semantics(
        case_name: &str,
        case: &ExactCaseData,
        word: &[usize],
    ) {
        let duals = exact_case_dual_vectors(case);
        for start in 0..word.len() - 2 {
            let facets = [word[start], word[start + 1], word[start + 2]];
            let mut metrics = ExactClosedTubeMetrics::default();
            let tube = match primitive_tube(
                &duals,
                &case.facet_intersection_is_nonempty,
                &case.omega_signs,
                facets,
                &mut metrics,
            )
            .expect("supported primitive tube")
            {
                PolygonOutcomeTube::Nonempty(tube) => tube,
                PolygonOutcomeTube::Empty => {
                    panic!("{case_name}: primitive tube {facets:?} was empty")
                }
            };

            for start_coords in tube.start_polygon.vertices(&mut metrics) {
                assert_primitive_segment_semantics(
                    case_name,
                    &duals,
                    facets,
                    &tube,
                    &start_coords,
                    &mut metrics,
                );
            }
        }
    }

    fn assert_primitive_segment_semantics(
        case_name: &str,
        duals: &[Vec4],
        facets: [usize; 3],
        tube: &ExactTube,
        start_coords: &Vec2,
        metrics: &mut ExactClosedTubeMetrics,
    ) {
        let [previous, current, next] = facets;
        let start_point = point_from_frame_coords(&tube.start_frame, start_coords);
        assert_eq!(
            dot4(&duals[previous], &start_point),
            R::one(),
            "{case_name}: primitive {facets:?} start point is not on previous facet"
        );
        assert_eq!(
            dot4(&duals[current], &start_point),
            R::one(),
            "{case_name}: primitive {facets:?} start point is not on current facet"
        );
        assert_inside_polytope(case_name, duals, &start_point);

        let end_coords = tube.start_to_end.apply(start_coords);
        let PolygonOutcome::Nonempty(end_face) = face_polygon(duals, &tube.end_frame, metrics)
        else {
            panic!("{case_name}: primitive {facets:?} end face was empty");
        };
        assert!(
            end_face.contains(&end_coords, metrics),
            "{case_name}: primitive {facets:?} affine image is outside end face polygon"
        );
        let end_point = point_from_frame_coords(&tube.end_frame, &end_coords);
        assert_eq!(
            dot4(&duals[current], &end_point),
            R::one(),
            "{case_name}: primitive {facets:?} end point is not on current facet"
        );
        assert_eq!(
            dot4(&duals[next], &end_point),
            R::one(),
            "{case_name}: primitive {facets:?} end point is not on next facet"
        );
        assert_inside_polytope(case_name, duals, &end_point);

        let action = tube.action_on_start.evaluate(start_coords);
        assert!(
            !action.is_negative(),
            "{case_name}: primitive {facets:?} has negative segment action {action}"
        );
        let reeb = scale4(&q(2), &j_times(&duals[current]));
        let direct_end = add4(&start_point, &scale4(&action, &reeb));
        assert_eq!(
            direct_end, end_point,
            "{case_name}: primitive {facets:?} affine map disagrees with direct Reeb segment"
        );
    }

    fn assert_balanced_tube_matches_left_associative(
        case_name: &str,
        case: &ExactCaseData,
        word: &[usize],
    ) {
        let duals = exact_case_dual_vectors(case);
        let mut balanced_metrics = ExactClosedTubeMetrics::default();
        let balanced = expect_nonempty_tube(
            case_name,
            "balanced",
            build_tube(
                &duals,
                &case.facet_intersection_is_nonempty,
                &case.omega_signs,
                word,
                &mut balanced_metrics,
            ),
        );
        let mut left_metrics = ExactClosedTubeMetrics::default();
        let left = build_tube_left_associative(&duals, case, word, &mut left_metrics)
            .unwrap_or_else(|error| panic!("{case_name}: left-associative tube failed: {error}"));

        assert_eq!(balanced.sequence, left.sequence);
        assert_eq!(balanced.start_frame.pair(), left.start_frame.pair());
        assert_eq!(balanced.end_frame.pair(), left.end_frame.pair());
        assert_eq!(balanced.start_to_end.matrix, left.start_to_end.matrix);
        assert_eq!(balanced.start_to_end.offset, left.start_to_end.offset);
        assert_eq!(
            balanced.action_on_start.coeff, left.action_on_start.coeff,
            "{case_name}: composed action coefficients differ"
        );
        assert_eq!(
            balanced.action_on_start.constant, left.action_on_start.constant,
            "{case_name}: composed action constants differ"
        );
        assert_eq!(
            polygon_vertices_sorted(&balanced.start_polygon, &mut balanced_metrics),
            polygon_vertices_sorted(&left.start_polygon, &mut left_metrics),
            "{case_name}: balanced and left-associative tube domains differ"
        );
    }

    fn build_tube_left_associative(
        duals: &[Vec4],
        case: &ExactCaseData,
        word: &[usize],
        metrics: &mut ExactClosedTubeMetrics,
    ) -> Result<ExactTube, &'static str> {
        if word.len() < 3 {
            return Err("word too short");
        }
        let mut tube = expect_nonempty_tube(
            "left_associative",
            "first primitive",
            primitive_tube(
                duals,
                &case.facet_intersection_is_nonempty,
                &case.omega_signs,
                [word[0], word[1], word[2]],
                metrics,
            ),
        );
        for start in 1..word.len() - 2 {
            let next = expect_nonempty_tube(
                "left_associative",
                "next primitive",
                primitive_tube(
                    duals,
                    &case.facet_intersection_is_nonempty,
                    &case.omega_signs,
                    [word[start], word[start + 1], word[start + 2]],
                    metrics,
                ),
            );
            tube = expect_nonempty_tube(
                "left_associative",
                "intersection",
                intersect_tubes(&tube, &next, metrics),
            );
        }
        Ok(tube)
    }

    fn expect_nonempty_tube(
        case_name: &str,
        stage: &str,
        result: Result<PolygonOutcomeTube, ()>,
    ) -> ExactTube {
        match result.expect("supported exact tube operation") {
            PolygonOutcomeTube::Nonempty(tube) => tube,
            PolygonOutcomeTube::Empty => panic!("{case_name}: {stage} tube was empty"),
        }
    }

    fn polygon_vertices_sorted(
        polygon: &ExactPolygon,
        metrics: &mut ExactClosedTubeMetrics,
    ) -> Vec<Vec2> {
        let mut vertices = polygon.vertices(metrics);
        vertices.sort();
        vertices
    }
}
