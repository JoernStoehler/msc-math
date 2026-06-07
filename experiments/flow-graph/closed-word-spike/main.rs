//! Exact closed-word resolver spike for the flow-graph algorithm.
//!
//! This is an isolated experiment binary, not a supported implementation. It
//! reuses the endpoint-set spike representation and adds an exact fixed-point
//! classifier for specific generated-polytope closed words.

use exp_combinatorial_cells::flat_polytope::CellPolytopeCache;
use nalgebra::DMatrix;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::Serialize;
use symplectic::random::generate_dual_vertices;

const DEFAULT_MASTER_SEED: u64 = 20260605;
const DEFAULT_FACET_COUNT: usize = 7;
const DEFAULT_ATTEMPT: u64 = 31;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

type R = BigRational;
type Vec2 = [R; 2];
type Vec4 = [R; 4];
type Mat2 = [[R; 2]; 2];

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
    solve: [usize; 2],
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

#[derive(Clone, Debug, Default)]
struct Metrics {
    polygon_polygon_intersections: u64,
    polygon_halfspace_intersections: u64,
    pullbacks: u64,
    images: u64,
    emptiness_checks: u64,
    line_pair_checks: u64,
    containment_checks: u64,
    containment_halfspace_checks: u64,
    max_numer_bits: u64,
    max_denom_bits: u64,
}

#[derive(Debug, Serialize)]
struct Row {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigma: Vec<usize>,
    word: Vec<usize>,
    classification: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_f64: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_point: Option<[String; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singular_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singular_min_action_exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singular_max_action_exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    tube_halfspaces: Option<usize>,
    tube_vertices: Option<usize>,
    polygon_polygon_intersections: u64,
    polygon_halfspace_intersections: u64,
    pullbacks: u64,
    images: u64,
    emptiness_checks: u64,
    line_pair_checks: u64,
    containment_checks: u64,
    containment_halfspace_checks: u64,
    max_numer_bits: u64,
    max_denom_bits: u64,
}

#[derive(Debug)]
struct Args {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigmas: Vec<Vec<usize>>,
}

fn parse_args() -> Args {
    let mut parsed = Args {
        facet_count: DEFAULT_FACET_COUNT,
        master_seed: DEFAULT_MASTER_SEED,
        attempt: DEFAULT_ATTEMPT,
        sigmas: vec![
            vec![0, 4, 2, 6],
            vec![0, 1, 5, 6, 4, 2],
            vec![0, 4, 1, 3, 2, 6],
        ],
    };
    let mut cli = std::env::args().skip(1);

    while let Some(arg) = cli.next() {
        match arg.as_str() {
            "--facet-count" => {
                parsed.facet_count = cli
                    .next()
                    .expect("--facet-count needs a value")
                    .parse()
                    .expect("--facet-count must be a usize");
            }
            "--master-seed" => {
                parsed.master_seed = cli
                    .next()
                    .expect("--master-seed needs a value")
                    .parse()
                    .expect("--master-seed must be a u64");
            }
            "--attempt" => {
                parsed.attempt = cli
                    .next()
                    .expect("--attempt needs a value")
                    .parse()
                    .expect("--attempt must be a u64");
            }
            "--sigma" => {
                parsed.sigmas.push(parse_sigma(
                    &cli.next().expect("--sigma needs comma-separated facets"),
                ));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-closed-word-spike [--facet-count N] [--master-seed N] [--attempt N] [--sigma 0,4,2,6]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }

    parsed
}

fn parse_sigma(value: &str) -> Vec<usize> {
    value
        .split(',')
        .map(|part| part.parse().expect("sigma entries must be usize"))
        .collect()
}

fn q(n: i64) -> R {
    BigRational::from_integer(n.into())
}

fn zero2() -> Vec2 {
    [R::zero(), R::zero()]
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

impl ExactPolygon {
    fn new(halfspaces: Vec<Halfspace>, metrics: &mut Metrics) -> PolygonOutcome {
        let polygon = Self { halfspaces };
        if polygon.is_empty(metrics) {
            PolygonOutcome::Empty
        } else {
            PolygonOutcome::Nonempty(polygon)
        }
    }

    fn contains(&self, point: &Vec2, metrics: &mut Metrics) -> bool {
        metrics.containment_checks += 1;
        metrics.containment_halfspace_checks += self.halfspaces.len() as u64;
        self.halfspaces
            .iter()
            .all(|h| dot2(&h.normal, point) <= h.rhs)
    }

    fn is_empty(&self, metrics: &mut Metrics) -> bool {
        metrics.emptiness_checks += 1;
        if self.halfspaces.is_empty() {
            return false;
        }
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

    fn vertices(&self, metrics: &mut Metrics) -> Vec<Vec2> {
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

    fn intersect_polygon(&self, other: &Self, metrics: &mut Metrics) -> PolygonOutcome {
        metrics.polygon_polygon_intersections += 1;
        let mut halfspaces = self.halfspaces.clone();
        halfspaces.extend(other.halfspaces.iter().cloned());
        Self::new(halfspaces, metrics)
    }

    fn intersect_halfspace(&self, halfspace: Halfspace, metrics: &mut Metrics) -> PolygonOutcome {
        metrics.polygon_halfspace_intersections += 1;
        let mut halfspaces = self.halfspaces.clone();
        halfspaces.push(halfspace);
        Self::new(halfspaces, metrics)
    }

    fn pullback(&self, affine: &Affine2, metrics: &mut Metrics) -> Self {
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

    fn image_under(&self, affine: &Affine2, metrics: &mut Metrics) -> Option<Self> {
        metrics.images += 1;
        let inverse = inverse(&affine.matrix)?;
        Some(Self {
            halfspaces: self
                .halfspaces
                .iter()
                .map(|h| Halfspace {
                    normal: mat_transpose_vec(&inverse, &h.normal),
                    rhs: &h.rhs + dot2(&h.normal, &mat_vec(&inverse, &affine.offset)),
                })
                .collect(),
        })
    }
}

fn track_rational(metrics: &mut Metrics, value: &R) {
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
    fn point(&self, coords: &Vec2) -> Vec4 {
        add4(
            &self.base,
            &add4(&scale4(&coords[0], &self.u), &scale4(&coords[1], &self.v)),
        )
    }

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
        solve,
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

fn face_polygon(duals: &[Vec4], frame: &FaceFrame, metrics: &mut Metrics) -> PolygonOutcome {
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
    metrics: &mut Metrics,
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
    metrics: &mut Metrics,
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
    metrics: &mut Metrics,
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

#[derive(Debug)]
enum ClosedClassification {
    EmptyTube,
    ZeroActionNoOrbit {
        action: Option<R>,
        point: Option<Vec2>,
        singular_status: Option<&'static str>,
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
    ConstructionError(String),
}

fn classify_closed_tube(
    duals: &[Vec4],
    polytope: &CellPolytopeCache,
    sigma: &[usize],
    metrics: &mut Metrics,
) -> (
    Vec<usize>,
    ClosedClassification,
    Option<usize>,
    Option<usize>,
) {
    if sigma.len() < 2 {
        return (
            sigma.to_vec(),
            ClosedClassification::ConstructionError(
                "sigma must have at least two facets".to_string(),
            ),
            None,
            None,
        );
    }
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    match build_tube(
        duals,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
        &word,
        metrics,
    ) {
        Ok(PolygonOutcomeTube::Empty) => (word, ClosedClassification::EmptyTube, None, None),
        Err(()) => (
            word,
            ClosedClassification::ConstructionError(
                "tube construction hit an unsupported singular transition".to_string(),
            ),
            None,
            None,
        ),
        Ok(PolygonOutcomeTube::Nonempty(tube)) => {
            let halfspaces = tube.start_polygon.halfspaces.len();
            let vertices = tube.start_polygon.vertices(metrics).len();
            let classification = solve_closed_tube(&tube, metrics);
            (word, classification, Some(halfspaces), Some(vertices))
        }
    }
}

fn solve_closed_tube(tube: &ExactTube, metrics: &mut Metrics) -> ClosedClassification {
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
            ClosedClassification::PositiveOrbit { action, point }
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

fn solve_singular_fixed_tube(
    tube: &ExactTube,
    lhs: &Mat2,
    rhs: &Vec2,
    metrics: &mut Metrics,
) -> ClosedClassification {
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
            return ClosedClassification::ConstructionError(
                "singular solve found rank two rows after zero determinant".to_string(),
            );
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
    metrics: &mut Metrics,
) -> ClosedClassification {
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

fn row_from_classification(
    args: &Args,
    polytope: &CellPolytopeCache,
    sigma: Vec<usize>,
    word: Vec<usize>,
    classification: ClosedClassification,
    tube_halfspaces: Option<usize>,
    tube_vertices: Option<usize>,
    metrics: Metrics,
) -> Row {
    let mut row = Row {
        facet_count: polytope.facet_count(),
        master_seed: args.master_seed,
        attempt: args.attempt,
        sigma,
        word,
        classification: String::new(),
        action_exact: None,
        action_f64: None,
        fixed_point: None,
        singular_status: None,
        singular_min_action_exact: None,
        singular_max_action_exact: None,
        error: None,
        tube_halfspaces,
        tube_vertices,
        polygon_polygon_intersections: metrics.polygon_polygon_intersections,
        polygon_halfspace_intersections: metrics.polygon_halfspace_intersections,
        pullbacks: metrics.pullbacks,
        images: metrics.images,
        emptiness_checks: metrics.emptiness_checks,
        line_pair_checks: metrics.line_pair_checks,
        containment_checks: metrics.containment_checks,
        containment_halfspace_checks: metrics.containment_halfspace_checks,
        max_numer_bits: metrics.max_numer_bits,
        max_denom_bits: metrics.max_denom_bits,
    };

    match classification {
        ClosedClassification::EmptyTube => row.classification = "empty_tube".to_string(),
        ClosedClassification::ZeroActionNoOrbit {
            action,
            point,
            singular_status,
        } => {
            row.classification = "zero_action_no_orbit".to_string();
            if let Some(action) = action {
                row.action_f64 = action.to_f64();
                row.action_exact = Some(action.to_string());
            }
            row.fixed_point = point.map(string_point);
            row.singular_status = singular_status.map(str::to_string);
        }
        ClosedClassification::PositiveOrbit { action, point } => {
            row.classification = "positive_orbit".to_string();
            row.action_f64 = action.to_f64();
            row.action_exact = Some(action.to_string());
            row.fixed_point = Some(string_point(point));
        }
        ClosedClassification::UnsupportedPositiveSingular {
            singular_status,
            min_action,
            max_action,
        } => {
            row.classification = "unsupported_positive_singular".to_string();
            row.singular_status = Some(singular_status.to_string());
            row.singular_min_action_exact = min_action.map(|value| value.to_string());
            row.singular_max_action_exact = max_action.map(|value| value.to_string());
        }
        ClosedClassification::ConstructionError(error) => {
            row.classification = "construction_error".to_string();
            row.error = Some(error);
        }
    }
    row
}

fn string_point(point: Vec2) -> [String; 2] {
    [point[0].to_string(), point[1].to_string()]
}

fn main() -> Result<(), String> {
    let args = parse_args();
    let dual_vertices = generate_dual_vertices(
        args.facet_count,
        H_MIN,
        H_MAX,
        args.master_seed,
        args.attempt,
    )
    .map_err(|error| format!("generate_dual_vertices: {error:?}"))?;
    let polytope = CellPolytopeCache::from_f64(dual_vertices)
        .ok_or_else(|| "generated polytope was rejected by CellPolytopeCache".to_string())?;
    let duals = polytope.dual_vertices.clone();

    for sigma in &args.sigmas {
        let mut metrics = Metrics::default();
        let (word, classification, tube_halfspaces, tube_vertices) =
            classify_closed_tube(&duals, &polytope, sigma, &mut metrics);
        let row = row_from_classification(
            &args,
            &polytope,
            sigma.clone(),
            word,
            classification,
            tube_halfspaces,
            tube_vertices,
            metrics,
        );
        serde_json::to_writer(std::io::stdout(), &row)
            .map_err(|error| format!("write JSON row: {error}"))?;
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn unit_square(metrics: &mut Metrics) -> ExactPolygon {
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

    fn polygon(halfspaces: Vec<Halfspace>, metrics: &mut Metrics) -> Option<ExactPolygon> {
        match ExactPolygon::new(halfspaces, metrics) {
            PolygonOutcome::Nonempty(polygon) => Some(polygon),
            PolygonOutcome::Empty => None,
        }
    }

    #[test]
    fn polygon_and_halfspace_can_stay_nonempty() {
        let mut metrics = Metrics::default();
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
        let mut metrics = Metrics::default();
        let square = unit_square(&mut metrics);
        let outcome = square.intersect_halfspace(halfspace(-1, 0, -2), &mut metrics);
        assert!(matches!(outcome, PolygonOutcome::Empty));
    }

    #[test]
    fn polygon_and_polygon_can_stay_nonempty() {
        let mut metrics = Metrics::default();
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
        let mut metrics = Metrics::default();
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
        let mut metrics = Metrics::default();
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
        let mut metrics = Metrics::default();
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
        let mut metrics = Metrics::default();
        let square = unit_square(&mut metrics);
        assert!(square.contains(&[r(0), r(0)], &mut metrics));
        assert!(square.contains(&[frac(1, 2), frac(1, 2)], &mut metrics));
        assert!(square.contains(&[r(1), r(1)], &mut metrics));
        assert!(!square.contains(&[r(2), r(1)], &mut metrics));
    }

    #[test]
    fn redundant_inequality_preserves_nonempty_status() {
        let mut metrics = Metrics::default();
        let square = unit_square(&mut metrics);
        let outcome = square.intersect_halfspace(halfspace(1, 0, 2), &mut metrics);
        assert!(matches!(outcome, PolygonOutcome::Nonempty(_)));
    }
}
