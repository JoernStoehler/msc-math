//! Emit exact flow-graph tube visualization data as JSON.
//!
//! The selected input is the deterministic F6 attempt used by the thesis.
//! The random fixture is converted losslessly to `BigRational` immediately;
//! exact arithmetic owns all incidence, sign, tube, fixed-point, and orbit
//! decisions.  `f64` appears only in the serialized plotting fields.

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{
    closed_tube_visualization_snapshot_exact, face_polygon_snapshot_exact,
    ExactAffineScalarSnapshot, ExactAffineSnapshot, ExactClosedOrbitSnapshot,
    ExactClosedTubeMetrics, ExactFacePolygonSnapshot, ExactFixedPointSnapshot, ExactFlatTubeInput,
    ExactHalfspaceSnapshot, ExactTubeFaceFixedPointSnapshot, ExactTubeVisualizationSnapshot,
};
use symplectic::exact::{
    exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega_signs_exact,
};
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices;

const DEFAULT_MASTER_SEED: u64 = 20260605;
const DEFAULT_FACET_COUNT: usize = 6;
const DEFAULT_ATTEMPT: u64 = 3;
const DEFAULT_SIGMA: &[usize] = &[1, 2, 4, 5, 3];
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

type R = BigRational;

#[derive(Debug)]
struct Args {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigma: Vec<usize>,
    cutoff: Option<R>,
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct VisualizationData {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigma: Vec<usize>,
    cutoff: Option<f64>,
    cutoff_exact: Option<String>,
    arithmetic: ExactProvenance,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_exact: Vec<[String; 4]>,
    vertices_f64: Vec<[f64; 4]>,
    vertices_exact: Vec<[String; 4]>,
    edges: Vec<[usize; 2]>,
    two_faces: Vec<TwoFaceSnapshot>,
    facet_intersection_is_nonempty: Vec<Vec<bool>>,
    omega_signs: Vec<Vec<i8>>,
    all_two_faces: Vec<FacePolygonSnapshot>,
    closed_tube: Option<TubeVisualizationSnapshot>,
    closed_orbit: Option<ClosedOrbitSnapshot>,
}

#[derive(Debug, Serialize)]
struct ExactProvenance {
    decision_arithmetic: &'static str,
    input_source: &'static str,
    serialization_boundary: &'static str,
    coordinate_chart: &'static str,
    exact_tube_metrics: ExactMetricsSnapshot,
}

#[derive(Debug, Serialize)]
struct ExactMetricsSnapshot {
    polygon_polygon_intersections: u64,
    polygon_halfspace_intersections: u64,
    action_cutoff_intersections: u64,
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
struct ClosedOrbitSnapshot {
    facets: Vec<usize>,
    action: f64,
    action_exact: String,
    breakpoints: Vec<[f64; 4]>,
    breakpoints_exact: Vec<[String; 4]>,
    segment_times: Vec<f64>,
    segment_times_exact: Vec<String>,
}

#[derive(Debug, Serialize)]
struct TwoFaceSnapshot {
    facets: [usize; 2],
    vertices: Vec<usize>,
}

#[derive(Debug, Serialize)]
struct FacePolygonSnapshot {
    pair: [usize; 2],
    frame_base_exact: [String; 4],
    frame_u_exact: [String; 4],
    frame_v_exact: [String; 4],
    vertices: Vec<[f64; 2]>,
    vertices_exact: Vec<[String; 2]>,
    inequalities: Vec<HalfspaceSnapshot>,
    inequalities_exact: Vec<ExactHalfspaceSnapshotJson>,
}

#[derive(Debug, Serialize)]
struct HalfspaceSnapshot {
    normal: [f64; 2],
    rhs: f64,
}

#[derive(Debug, Serialize)]
struct ExactHalfspaceSnapshotJson {
    normal: [String; 2],
    rhs: String,
}

#[derive(Debug, Serialize)]
struct AffineSnapshot {
    matrix: [[f64; 2]; 2],
    offset: [f64; 2],
    matrix_exact: [[String; 2]; 2],
    offset_exact: [String; 2],
}

#[derive(Debug, Serialize)]
struct AffineScalarSnapshot {
    coeff: [f64; 2],
    constant: f64,
    coeff_exact: [String; 2],
    constant_exact: String,
}

#[derive(Debug, Serialize)]
struct TubeFaceSnapshot {
    pair: [usize; 2],
    polygon: FacePolygonSnapshot,
    role: String,
}

#[derive(Debug, Serialize)]
struct TubeFaceFixedPointSnapshot {
    pair: [usize; 2],
    role: String,
    point: Option<[f64; 2]>,
    point_exact: Option<[String; 2]>,
}

#[derive(Debug, Serialize)]
struct FixedPointSnapshot {
    status: String,
    point: Option<[f64; 2]>,
    point_exact: Option<[String; 2]>,
    line_point: Option<[f64; 2]>,
    line_direction: Option<[f64; 2]>,
    action: Option<f64>,
    action_exact: Option<String>,
    singular_status: Option<String>,
    min_action_exact: Option<String>,
    max_action_exact: Option<String>,
}

#[derive(Debug, Serialize)]
struct TubeVisualizationSnapshot {
    sequence: Vec<usize>,
    start_pair: [usize; 2],
    end_pair: [usize; 2],
    start_polygon: FacePolygonSnapshot,
    end_polygon: FacePolygonSnapshot,
    intermediate_polygons: Vec<TubeFaceSnapshot>,
    fixed_points_on_faces: Vec<TubeFaceFixedPointSnapshot>,
    start_to_end: AffineSnapshot,
    end_to_start: AffineSnapshot,
    action_on_start: AffineScalarSnapshot,
    action_on_end: AffineScalarSnapshot,
    fixed_point: FixedPointSnapshot,
    cutoff: Option<f64>,
    cutoff_exact: Option<String>,
    exact_metrics: ExactMetricsSnapshot,
}

fn main() -> Result<(), String> {
    let args = parse_args();
    let sampled_duals = generate_dual_vertices(
        args.facet_count,
        H_MIN,
        H_MAX,
        args.master_seed,
        args.attempt,
    )
    .map_err(|error| format!("generate dual vertices: {error:?}"))?;
    let dual_vertices: Vec<[R; 4]> = sampled_duals
        .iter()
        .map(|dual| std::array::from_fn(|index| f64_to_rational(dual[index])))
        .collect();
    let dual_vectors: Vec<Vector4<R>> = dual_vertices
        .iter()
        .map(|dual| {
            Vector4::new(
                dual[0].clone(),
                dual[1].clone(),
                dual[2].clone(),
                dual[3].clone(),
            )
        })
        .collect();
    let exact_polytope = exact_vertices_with_incidence(&dual_vectors)
        .map_err(|error| format!("enumerate exact polytope vertices: {error:?}"))?;
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_exact(&exact_polytope.vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&dual_vectors);
    let input = ExactFlatTubeInput {
        dual_vertices: &dual_vertices,
        facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
        omega_signs: &omega_signs,
    };
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersection_is_nonempty,
        &omega_signs,
    );

    let mut all_two_faces = Vec::new();
    for first in 0..args.facet_count {
        for second in 0..args.facet_count {
            if transition_is_allowed[(first, second)] {
                let face = face_polygon_snapshot_exact(&input, first, second).map_err(|error| {
                    format!("snapshot exact face [{first},{second}]: {error:?}")
                })?;
                all_two_faces.push(face_snapshot(&face)?);
            }
        }
    }

    let exact_tube =
        closed_tube_visualization_snapshot_exact(&input, &args.sigma, args.cutoff.as_ref())
            .map_err(|error| format!("snapshot exact closed tube {:?}: {error:?}", args.sigma))?
            .ok_or_else(|| format!("closed tube {:?} is empty", args.sigma))?;
    let exact_orbit = exact_tube
        .closed_orbit
        .clone()
        .ok_or_else(|| format!("closed tube {:?} has no positive exact orbit", args.sigma))?;

    let data = VisualizationData {
        facet_count: args.facet_count,
        master_seed: args.master_seed,
        attempt: args.attempt,
        sigma: args.sigma,
        cutoff: args.cutoff.as_ref().map(to_f64).transpose()?,
        cutoff_exact: args.cutoff.as_ref().map(ToString::to_string),
        arithmetic: ExactProvenance {
            decision_arithmetic: "BigRational throughout input geometry and tube decisions",
            input_source: "deterministic f64 sample converted losslessly to BigRational before exact decisions",
            serialization_boundary: "f64 only in JSON plotting fields and Python radial/stereographic projection",
            coordinate_chart: "plotted face vertices/fixed points use an orthonormalized f64 frame; exact_* fields retain the rational construction chart",
            exact_tube_metrics: metrics_snapshot(&exact_tube.metrics),
        },
        dual_vertices_f64: dual_vertices.iter().map(array4_to_f64).collect::<Result<_, _>>()?,
        dual_vertices_exact: dual_vertices.iter().map(array4_to_string).collect(),
        vertices_f64: exact_polytope
            .vertices
            .iter()
            .map(|vertex| vector4_to_f64(vertex))
            .collect::<Result<_, _>>()?,
        vertices_exact: exact_polytope
            .vertices
            .iter()
            .map(|vertex| vector4_to_string(vertex))
            .collect(),
        edges: polytope_edges(&exact_polytope.vertex_facet_incidence),
        two_faces: polytope_two_faces(&exact_polytope.vertex_facet_incidence),
        facet_intersection_is_nonempty: matrix_rows(&facet_intersection_is_nonempty),
        omega_signs: matrix_rows(&omega_signs),
        all_two_faces,
        closed_tube: Some(tube_snapshot(&exact_tube)?),
        closed_orbit: Some(orbit_snapshot(&exact_orbit)?),
    };

    write_json(&data, args.output.as_ref())
}

fn tube_snapshot(
    snapshot: &ExactTubeVisualizationSnapshot,
) -> Result<TubeVisualizationSnapshot, String> {
    Ok(TubeVisualizationSnapshot {
        sequence: snapshot.sequence.clone(),
        start_pair: snapshot.start_pair,
        end_pair: snapshot.end_pair,
        start_polygon: face_snapshot(&snapshot.start_polygon)?,
        end_polygon: face_snapshot(&snapshot.end_polygon)?,
        intermediate_polygons: snapshot
            .intermediate_polygons
            .iter()
            .map(|face| {
                Ok(TubeFaceSnapshot {
                    pair: face.pair,
                    polygon: face_snapshot(&face.polygon)?,
                    role: face.role.clone(),
                })
            })
            .collect::<Result<_, String>>()?,
        fixed_points_on_faces: snapshot
            .fixed_points_on_faces
            .iter()
            .zip(snapshot.intermediate_polygons.iter())
            .map(|(point, face)| fixed_point_on_face_snapshot(point, &face.polygon))
            .collect::<Result<_, _>>()?,
        start_to_end: affine_snapshot(&snapshot.start_to_end)?,
        end_to_start: affine_snapshot(&snapshot.end_to_start)?,
        action_on_start: affine_scalar_snapshot(&snapshot.action_on_start)?,
        action_on_end: affine_scalar_snapshot(&snapshot.action_on_end)?,
        fixed_point: fixed_point_snapshot(&snapshot.fixed_point, &snapshot.start_polygon)?,
        cutoff: snapshot.cutoff.as_ref().map(to_f64).transpose()?,
        cutoff_exact: snapshot.cutoff.as_ref().map(ToString::to_string),
        exact_metrics: metrics_snapshot(&snapshot.metrics),
    })
}

fn face_snapshot(snapshot: &ExactFacePolygonSnapshot) -> Result<FacePolygonSnapshot, String> {
    Ok(FacePolygonSnapshot {
        pair: snapshot.pair,
        frame_base_exact: array4_to_string(&snapshot.base),
        frame_u_exact: array4_to_string(&snapshot.u),
        frame_v_exact: array4_to_string(&snapshot.v),
        vertices: snapshot
            .vertices
            .iter()
            .map(|point| project_exact_point(snapshot, point))
            .collect::<Result<_, _>>()?,
        vertices_exact: snapshot.vertices.iter().map(array2_to_string).collect(),
        inequalities: snapshot
            .inequalities
            .iter()
            .map(|halfspace| {
                Ok(HalfspaceSnapshot {
                    normal: array2_to_f64(&halfspace.normal)?,
                    rhs: to_f64(&halfspace.rhs)?,
                })
            })
            .collect::<Result<_, String>>()?,
        inequalities_exact: snapshot
            .inequalities
            .iter()
            .map(halfspace_to_string)
            .collect(),
    })
}

fn halfspace_to_string(halfspace: &ExactHalfspaceSnapshot) -> ExactHalfspaceSnapshotJson {
    ExactHalfspaceSnapshotJson {
        normal: array2_to_string(&halfspace.normal),
        rhs: halfspace.rhs.to_string(),
    }
}

fn affine_snapshot(snapshot: &ExactAffineSnapshot) -> Result<AffineSnapshot, String> {
    Ok(AffineSnapshot {
        matrix: matrix2_to_f64(&snapshot.matrix)?,
        offset: array2_to_f64(&snapshot.offset)?,
        matrix_exact: matrix2_to_string(&snapshot.matrix),
        offset_exact: array2_to_string(&snapshot.offset),
    })
}

fn affine_scalar_snapshot(
    snapshot: &ExactAffineScalarSnapshot,
) -> Result<AffineScalarSnapshot, String> {
    Ok(AffineScalarSnapshot {
        coeff: array2_to_f64(&snapshot.coeff)?,
        constant: to_f64(&snapshot.constant)?,
        coeff_exact: array2_to_string(&snapshot.coeff),
        constant_exact: snapshot.constant.to_string(),
    })
}

fn fixed_point_on_face_snapshot(
    snapshot: &ExactTubeFaceFixedPointSnapshot,
    face: &ExactFacePolygonSnapshot,
) -> Result<TubeFaceFixedPointSnapshot, String> {
    Ok(TubeFaceFixedPointSnapshot {
        pair: snapshot.pair,
        role: snapshot.role.clone(),
        point: snapshot
            .point
            .as_ref()
            .map(|point| project_exact_point(face, point))
            .transpose()?,
        point_exact: snapshot.point.as_ref().map(array2_to_string),
    })
}

fn fixed_point_snapshot(
    snapshot: &ExactFixedPointSnapshot,
    face: &ExactFacePolygonSnapshot,
) -> Result<FixedPointSnapshot, String> {
    Ok(FixedPointSnapshot {
        status: snapshot.status.clone(),
        point: snapshot
            .point
            .as_ref()
            .map(|point| project_exact_point(face, point))
            .transpose()?,
        point_exact: snapshot.point.as_ref().map(array2_to_string),
        line_point: None,
        line_direction: None,
        action: snapshot.action.as_ref().map(to_f64).transpose()?,
        action_exact: snapshot.action.as_ref().map(ToString::to_string),
        singular_status: snapshot.singular_status.clone(),
        min_action_exact: snapshot.min_action.as_ref().map(ToString::to_string),
        max_action_exact: snapshot.max_action.as_ref().map(ToString::to_string),
    })
}

fn orbit_snapshot(snapshot: &ExactClosedOrbitSnapshot) -> Result<ClosedOrbitSnapshot, String> {
    Ok(ClosedOrbitSnapshot {
        facets: snapshot.facets.clone(),
        action: to_f64(&snapshot.action)?,
        action_exact: snapshot.action.to_string(),
        breakpoints: snapshot
            .breakpoints
            .iter()
            .map(array4_to_f64)
            .collect::<Result<_, _>>()?,
        breakpoints_exact: snapshot.breakpoints.iter().map(array4_to_string).collect(),
        segment_times: snapshot
            .segment_times
            .iter()
            .map(to_f64)
            .collect::<Result<_, _>>()?,
        segment_times_exact: snapshot
            .segment_times
            .iter()
            .map(ToString::to_string)
            .collect(),
    })
}

fn to_f64(value: &R) -> Result<f64, String> {
    value
        .to_f64()
        .ok_or_else(|| format!("exact rational is not representable as f64: {value}"))
}

fn metrics_snapshot(metrics: &ExactClosedTubeMetrics) -> ExactMetricsSnapshot {
    ExactMetricsSnapshot {
        polygon_polygon_intersections: metrics.polygon_polygon_intersections,
        polygon_halfspace_intersections: metrics.polygon_halfspace_intersections,
        action_cutoff_intersections: metrics.action_cutoff_intersections,
        pullbacks: metrics.pullbacks,
        images: metrics.images,
        emptiness_checks: metrics.emptiness_checks,
        line_pair_checks: metrics.line_pair_checks,
        containment_checks: metrics.containment_checks,
        containment_halfspace_checks: metrics.containment_halfspace_checks,
        max_numer_bits: metrics.max_numer_bits,
        max_denom_bits: metrics.max_denom_bits,
    }
}

struct VisualizationFrame {
    base: [f64; 4],
    u: [f64; 4],
    v: [f64; 4],
}

fn project_exact_point(
    face: &ExactFacePolygonSnapshot,
    point: &[R; 2],
) -> Result<[f64; 2], String> {
    let frame = visualization_frame(face)?;
    let base = array4_to_f64(&face.base)?;
    let u = array4_to_f64(&face.u)?;
    let v = array4_to_f64(&face.v)?;
    let coordinates = [to_f64(&point[0])?, to_f64(&point[1])?];
    let ambient = add4(
        base,
        add4(scale4(coordinates[0], u), scale4(coordinates[1], v)),
    );
    let displacement = sub4(ambient, frame.base);
    Ok([dot4(frame.u, displacement), dot4(frame.v, displacement)])
}

fn visualization_frame(face: &ExactFacePolygonSnapshot) -> Result<VisualizationFrame, String> {
    let base = array4_to_f64(&face.base)?;
    let raw_u = array4_to_f64(&face.u)?;
    let raw_v = array4_to_f64(&face.v)?;
    let u_norm = norm4(raw_u);
    if u_norm <= 1e-12 {
        return Err(format!(
            "face {:?} has a singular visualization frame",
            face.pair
        ));
    }
    let u = scale4(1.0 / u_norm, raw_u);
    let v_raw = sub4(raw_v, scale4(dot4(u, raw_v), u));
    let v_norm = norm4(v_raw);
    if v_norm <= 1e-12 {
        return Err(format!(
            "face {:?} has a singular visualization frame",
            face.pair
        ));
    }
    Ok(VisualizationFrame {
        base,
        u,
        v: scale4(1.0 / v_norm, v_raw),
    })
}

fn add4(left: [f64; 4], right: [f64; 4]) -> [f64; 4] {
    [
        left[0] + right[0],
        left[1] + right[1],
        left[2] + right[2],
        left[3] + right[3],
    ]
}

fn sub4(left: [f64; 4], right: [f64; 4]) -> [f64; 4] {
    [
        left[0] - right[0],
        left[1] - right[1],
        left[2] - right[2],
        left[3] - right[3],
    ]
}

fn scale4(scale: f64, vector: [f64; 4]) -> [f64; 4] {
    [
        scale * vector[0],
        scale * vector[1],
        scale * vector[2],
        scale * vector[3],
    ]
}

fn dot4(left: [f64; 4], right: [f64; 4]) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2] + left[3] * right[3]
}

fn norm4(vector: [f64; 4]) -> f64 {
    dot4(vector, vector).sqrt()
}

fn array2_to_f64(array: &[R; 2]) -> Result<[f64; 2], String> {
    Ok([to_f64(&array[0])?, to_f64(&array[1])?])
}

fn array4_to_f64(array: &[R; 4]) -> Result<[f64; 4], String> {
    Ok([
        to_f64(&array[0])?,
        to_f64(&array[1])?,
        to_f64(&array[2])?,
        to_f64(&array[3])?,
    ])
}

fn vector4_to_f64(vector: &Vector4<R>) -> Result<[f64; 4], String> {
    array4_to_f64(&[
        vector[0].clone(),
        vector[1].clone(),
        vector[2].clone(),
        vector[3].clone(),
    ])
}

fn vector4_to_string(vector: &Vector4<R>) -> [String; 4] {
    [
        vector[0].to_string(),
        vector[1].to_string(),
        vector[2].to_string(),
        vector[3].to_string(),
    ]
}

fn array2_to_string(array: &[R; 2]) -> [String; 2] {
    [array[0].to_string(), array[1].to_string()]
}

fn array4_to_string(array: &[R; 4]) -> [String; 4] {
    [
        array[0].to_string(),
        array[1].to_string(),
        array[2].to_string(),
        array[3].to_string(),
    ]
}

fn matrix2_to_f64(matrix: &[[R; 2]; 2]) -> Result<[[f64; 2]; 2], String> {
    Ok([array2_to_f64(&matrix[0])?, array2_to_f64(&matrix[1])?])
}

fn matrix2_to_string(matrix: &[[R; 2]; 2]) -> [[String; 2]; 2] {
    [array2_to_string(&matrix[0]), array2_to_string(&matrix[1])]
}

fn polytope_edges(incidence: &DMatrix<bool>) -> Vec<[usize; 2]> {
    let mut edges = Vec::new();
    for first in 0..incidence.nrows() {
        for second in first + 1..incidence.nrows() {
            let shared_facets = (0..incidence.ncols())
                .filter(|&facet| incidence[(first, facet)] && incidence[(second, facet)])
                .count();
            if shared_facets >= 3 {
                edges.push([first, second]);
            }
        }
    }
    edges
}

fn polytope_two_faces(incidence: &DMatrix<bool>) -> Vec<TwoFaceSnapshot> {
    let mut two_faces = Vec::new();
    for first in 0..incidence.ncols() {
        for second in first + 1..incidence.ncols() {
            let vertices: Vec<usize> = (0..incidence.nrows())
                .filter(|&vertex| incidence[(vertex, first)] && incidence[(vertex, second)])
                .collect();
            if vertices.len() >= 3 {
                two_faces.push(TwoFaceSnapshot {
                    facets: [first, second],
                    vertices,
                });
            }
        }
    }
    two_faces
}

fn matrix_rows<T: Copy>(matrix: &DMatrix<T>) -> Vec<Vec<T>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn write_json(data: &VisualizationData, output: Option<&PathBuf>) -> Result<(), String> {
    match output {
        Some(path) => {
            let file = File::create(path)
                .map_err(|error| format!("create output {}: {error}", path.display()))?;
            serde_json::to_writer_pretty(BufWriter::new(file), data)
                .map_err(|error| format!("serialize output: {error}"))?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer_pretty(&mut handle, data)
                .map_err(|error| format!("serialize output: {error}"))?;
            writeln!(handle).map_err(|error| format!("write newline: {error}"))?;
        }
    }
    Ok(())
}

fn parse_args() -> Args {
    let mut args = Args {
        facet_count: DEFAULT_FACET_COUNT,
        master_seed: DEFAULT_MASTER_SEED,
        attempt: DEFAULT_ATTEMPT,
        sigma: DEFAULT_SIGMA.to_vec(),
        cutoff: None,
        output: None,
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--facet-count" => {
                args.facet_count = iter
                    .next()
                    .expect("--facet-count needs a value")
                    .parse()
                    .expect("--facet-count must be usize");
            }
            "--master-seed" => {
                args.master_seed = iter
                    .next()
                    .expect("--master-seed needs a value")
                    .parse()
                    .expect("--master-seed must be u64");
            }
            "--attempt" => {
                args.attempt = iter
                    .next()
                    .expect("--attempt needs a value")
                    .parse()
                    .expect("--attempt must be u64");
            }
            "--sigma" => {
                args.sigma = iter
                    .next()
                    .expect("--sigma needs a comma-separated value")
                    .split(',')
                    .map(|part| part.parse().expect("sigma entries must be usize"))
                    .collect();
            }
            "--cutoff" => {
                let value = iter.next().expect("--cutoff needs a value");
                args.cutoff = if value == "inf" || value == "infinity" {
                    None
                } else {
                    Some(f64_to_rational(
                        value.parse().expect("cutoff must be f64 or inf"),
                    ))
                };
            }
            "--output" => {
                args.output = Some(PathBuf::from(iter.next().expect("--output needs a path")));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-visualize-tube-data [--facet-count N] [--master-seed N] [--attempt N] [--sigma 0,4,2,6] [--cutoff inf] [--output PATH]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    args
}
