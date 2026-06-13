//! Exact endpoint-set spike for the flow-graph algorithm.
//!
//! This is a measurement spike, not a supported implementation. It tests the
//! simplest exact endpoint-set representation: a list of rational halfspaces
//! with operation-level empty/nonempty status.

use exp_combinatorial_cells::flat_polytope::{rational_arrays_to_vectors, CellPolytopeCache};
use exp_combinatorial_cells::name_from_record;
use nalgebra::DMatrix;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{enumerate_transition_pruned_words, half_cache_depth};
use symplectic::database;

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
    live_tubes: usize,
    empty_tubes: usize,
    unsupported_tubes: usize,
    max_halfspaces: usize,
    sum_live_halfspaces: usize,
    max_numer_bits: u64,
    max_denom_bits: u64,
}

#[derive(Debug, Serialize)]
struct Row {
    polytope_name: String,
    facet_count: usize,
    half_cache_total: usize,
    elapsed_ms: f64,
    live_tubes: usize,
    empty_tubes: usize,
    unsupported_tubes: usize,
    polygon_polygon_intersections: u64,
    polygon_halfspace_intersections: u64,
    pullbacks: u64,
    images: u64,
    emptiness_checks: u64,
    line_pair_checks: u64,
    containment_checks: u64,
    containment_halfspace_checks: u64,
    max_halfspaces: usize,
    sum_live_halfspaces: usize,
    max_numer_bits: u64,
    max_denom_bits: u64,
}

#[derive(Debug)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    max_facets: Option<usize>,
    max_rows: Option<usize>,
}

fn parse_args() -> Args {
    let mut input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../combinatorial-cells/polytopes.jsonl");
    let mut output = None;
    let mut max_facets = None;
    let mut max_rows = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                input = args
                    .next()
                    .map(PathBuf::from)
                    .expect("--input needs a path")
            }
            "--output" => {
                output = Some(
                    args.next()
                        .map(PathBuf::from)
                        .expect("--output needs a path"),
                );
            }
            "--max-facets" => {
                max_facets = Some(
                    args.next()
                        .expect("--max-facets needs a value")
                        .parse()
                        .expect("--max-facets must be a usize"),
                );
            }
            "--max-rows" => {
                max_rows = Some(
                    args.next()
                        .expect("--max-rows needs a value")
                        .parse()
                        .expect("--max-rows must be a usize"),
                );
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-endpoint-spike [--input PATH] [--output PATH] [--max-facets N] [--max-rows N]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }

    Args {
        input,
        output,
        max_facets,
        max_rows,
    }
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

fn run_row(polytope_name: String, polytope: &CellPolytopeCache) -> Row {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let words = enumerate_transition_pruned_words(
        &transition_is_allowed,
        half_cache_depth(polytope.facet_count()),
    );
    let duals = rational_arrays_to_vectors(&polytope.dual_vertices);
    let duals: Vec<Vec4> = duals
        .into_iter()
        .map(|v| [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
        .collect();

    let started = Instant::now();
    let mut metrics = Metrics::default();
    for word in &words {
        match build_tube(
            &duals,
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
            &word.facets,
            &mut metrics,
        ) {
            Ok(PolygonOutcomeTube::Nonempty(tube)) => {
                metrics.live_tubes += 1;
                metrics.max_halfspaces = metrics
                    .max_halfspaces
                    .max(tube.start_polygon.halfspaces.len());
                metrics.sum_live_halfspaces += tube.start_polygon.halfspaces.len();
                let _ = tube
                    .start_polygon
                    .image_under(&tube.start_to_end, &mut metrics);
            }
            Ok(PolygonOutcomeTube::Empty) => metrics.empty_tubes += 1,
            Err(()) => metrics.unsupported_tubes += 1,
        }
    }
    Row {
        polytope_name,
        facet_count: polytope.facet_count(),
        half_cache_total: words.len(),
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
        live_tubes: metrics.live_tubes,
        empty_tubes: metrics.empty_tubes,
        unsupported_tubes: metrics.unsupported_tubes,
        polygon_polygon_intersections: metrics.polygon_polygon_intersections,
        polygon_halfspace_intersections: metrics.polygon_halfspace_intersections,
        pullbacks: metrics.pullbacks,
        images: metrics.images,
        emptiness_checks: metrics.emptiness_checks,
        line_pair_checks: metrics.line_pair_checks,
        containment_checks: metrics.containment_checks,
        containment_halfspace_checks: metrics.containment_halfspace_checks,
        max_halfspaces: metrics.max_halfspaces,
        sum_live_halfspaces: metrics.sum_live_halfspaces,
        max_numer_bits: metrics.max_numer_bits,
        max_denom_bits: metrics.max_denom_bits,
    }
}

fn main() {
    let args = parse_args();
    let db = database::load_many(&[args.input.as_path()]).expect("load polytope database");
    let writer: Box<dyn Write> = match &args.output {
        Some(path) => {
            Box::new(BufWriter::new(File::create(path).unwrap_or_else(|err| {
                panic!("create {}: {err}", path.display())
            })))
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };
    let mut writer = writer;
    let mut emitted = 0usize;

    for (idx, (_, record)) in db.iter().enumerate() {
        let facet_count = record.dual_vertices_rational.len();
        if args
            .max_facets
            .is_some_and(|max_facets| facet_count > max_facets)
        {
            continue;
        }
        let Some(polytope) = CellPolytopeCache::from_rational_parts(
            record.dual_vertices_rational.clone(),
            record.vertices_rational.clone(),
        ) else {
            eprintln!("skip row {idx}: could not reconstruct polytope");
            continue;
        };
        let row = run_row(name_from_record(record, idx), &polytope);
        serde_json::to_writer(&mut writer, &row).expect("write row");
        writeln!(&mut writer).expect("write newline");
        emitted += 1;
        if args.max_rows.is_some_and(|max_rows| emitted >= max_rows) {
            break;
        }
    }
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
