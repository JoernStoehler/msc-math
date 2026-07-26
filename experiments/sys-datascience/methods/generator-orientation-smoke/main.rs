//! Target-free reconstruction and invariant smoke for U(2) and SO(4) actions.
//!
//! Matrices act on primal points in `(q1,q2,p1,p2)` order. Dual normals are
//! therefore transformed by an explicitly computed inverse transpose.

use exp_sys_landscape::{
    rational_vec4_to_strings, reference::exact_volume_as_f64, SysLandscapePolytopeCache,
};
use nalgebra::{Matrix4, Vector2, Vector4};
use num_rational::BigRational;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;
use symplectic::geom::polygon::{polygon_area, random_polygon_2d};
use symplectic::omega0;

const SCHEMA_ROW: &str = "generator-orientation-smoke-row-v2";
const SCHEMA_REPORT: &str = "generator-orientation-smoke-report-v2";
const DEFAULT_SEED: u64 = 20_260_714;
const DEFAULT_ATTEMPTS: usize = 128;
const DEFAULT_BUCKETS: &[(usize, usize)] = &[(3, 3), (4, 4), (4, 6), (6, 6)];
const MATRIX_TOL: f64 = 1e-12;
const DET_TOL: f64 = 1e-12;
const EUCLIDEAN_TOL: f64 = 1e-10;
const VOLUME_REL_TOL: f64 = 1e-10;
const FLOATING_OMEGA_TOL: f64 = 1e-10;
const SO4_NON_U2_MIN_RESIDUAL: f64 = 0.1;

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
    attempts: usize,
    rows_per_bucket: usize,
    buckets: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Variant {
    Identity,
    DeterministicU2,
    HaarU2,
    DeterministicSo4,
    HaarSo4,
}

impl Variant {
    const ALL: [Self; 5] = [
        Self::Identity,
        Self::DeterministicU2,
        Self::HaarU2,
        Self::DeterministicSo4,
        Self::HaarSo4,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::DeterministicU2 => "u2-deterministic",
            Self::HaarU2 => "u2-haar",
            Self::DeterministicSo4 => "so4-deterministic",
            Self::HaarSo4 => "so4-haar",
        }
    }

    fn family(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::DeterministicU2 | Self::HaarU2 => "u2",
            Self::DeterministicSo4 | Self::HaarSo4 => "so4",
        }
    }

    fn mode(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::DeterministicU2 | Self::DeterministicSo4 => "deterministic",
            Self::HaarU2 | Self::HaarSo4 => "haar",
        }
    }
}

#[derive(Clone)]
struct MapSpec {
    variant: Variant,
    map_seed: u64,
    matrix: Matrix4<f64>,
    exact_signed_matrix: Option<[[i8; 4]; 4]>,
}

#[derive(Clone, Serialize)]
struct SmokeRow {
    schema: &'static str,
    sample_id: String,
    base_id: String,
    transformed_id: String,
    base_geometry_id: Option<String>,
    transformed_geometry_id: Option<String>,
    seed: u64,
    base_seed: u64,
    row_index: usize,
    accepted_attempt: Option<usize>,
    bucket: String,
    q_sides: usize,
    p_sides: usize,
    base_accepted: bool,
    base_error: Option<String>,
    base_facet_count: Option<usize>,
    base_vertex_count: Option<usize>,
    facet_count: Option<usize>,
    vertex_count: Option<usize>,
    reconstruction_status: String,
    reconstruction_error: Option<String>,
    map_variant: String,
    map_family: &'static str,
    map_mode: &'static str,
    map_status: &'static str,
    map_error: Option<String>,
    map_seed: u64,
    matrix_available: bool,
    matrix_row_major: [[f64; 4]; 4],
    coordinate_order: &'static str,
    map_direction: &'static str,
    dual_action: &'static str,
    determinant: f64,
    orthogonality_residual_frobenius: f64,
    symplectic_u2_residual_frobenius: f64,
    dual_gram_max_abs_error: Option<f64>,
    floating_omega_max_abs_error: Option<f64>,
    transformed_dual_vertices_f64: Vec<[f64; 4]>,
    transformed_dual_vertices_rational: Vec<[String; 4]>,
    reconstructed_primal_vertices_rational: Vec<[String; 4]>,
    base_exact_volume_as_f64: Option<f64>,
    exact_volume_as_f64: Option<f64>,
    relative_volume_change: Option<f64>,
    labeled_incidence_signature: Vec<Vec<usize>>,
    labeled_incidence_matches_base: Option<bool>,
    omega_sign_signature: Vec<i8>,
    omega_exactly_matches_base: Option<bool>,
    omega_comparison_status: String,
    exact_signed_payload_matches: Option<bool>,
    semantic_invariants_passed: bool,
    invariant_failures: Vec<String>,
    generation_ms: f64,
    transform_ms: f64,
    reconstruction_ms: f64,
}

#[derive(Clone, Default, Serialize)]
struct BucketVariantSummary {
    bucket: String,
    map_variant: String,
    requested: usize,
    base_accepted: usize,
    reconstructed: usize,
    invariant_passed: usize,
    invariant_failed: usize,
    total_generation_ms: f64,
    total_transform_ms: f64,
    total_reconstruction_ms: f64,
}

#[derive(Serialize)]
struct Tolerances {
    matrix_frobenius: f64,
    determinant_abs: f64,
    dual_gram_scaled_abs: f64,
    volume_relative: f64,
    floating_omega_scaled_abs: f64,
    haar_so4_discriminating_min_symplectic_residual: f64,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    source_revision: String,
    source_dirty: bool,
    seed: u64,
    max_attempts_per_base: usize,
    rows_per_bucket: usize,
    requested_buckets: Vec<String>,
    requested_base_count: usize,
    accepted_base_count: usize,
    observed_row_count: usize,
    expected_variants_per_accepted_base: usize,
    status_counts: BTreeMap<String, usize>,
    by_map_variant_and_bucket: Vec<BucketVariantSummary>,
    invariant_failure_count: usize,
    invariant_failure_counts: BTreeMap<String, usize>,
    all_requested_rows_passed: bool,
    map_algorithms: BTreeMap<String, &'static str>,
    tolerances: Tolerances,
    coordinate_order: &'static str,
    map_direction: &'static str,
    dual_action: &'static str,
    interpretation_boundary: &'static str,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.re / rhs, self.im / rhs)
    }
}

fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/generator-orientation-smoke/artifacts",
        ),
        seed: DEFAULT_SEED,
        attempts: DEFAULT_ATTEMPTS,
        rows_per_bucket: 1,
        buckets: DEFAULT_BUCKETS.to_vec(),
    };
    let mut i = 1;
    while i < argv.len() {
        let value = |flag: &str| {
            argv.get(i + 1)
                .cloned()
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(value("--out-dir")?);
                i += 2;
            }
            "--seed" => {
                args.seed = value("--seed")?
                    .parse()
                    .map_err(|_| "--seed must be a u64".to_string())?;
                i += 2;
            }
            "--attempts" => {
                args.attempts = value("--attempts")?
                    .parse()
                    .map_err(|_| "--attempts must be a usize".to_string())?;
                i += 2;
            }
            "--rows-per-bucket" => {
                args.rows_per_bucket = value("--rows-per-bucket")?
                    .parse()
                    .map_err(|_| "--rows-per-bucket must be a usize".to_string())?;
                i += 2;
            }
            "--buckets" => {
                args.buckets = parse_buckets(&value("--buckets")?)?;
                i += 2;
            }
            "--help" | "-h" => {
                println!(
                    "--out-dir DIR [--seed N] [--attempts N] [--rows-per-bucket N] [--buckets 3x3,4x4,4x6,6x6]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    if args.attempts == 0 {
        return Err("--attempts must be positive".to_string());
    }
    if args.rows_per_bucket == 0 {
        return Err("--rows-per-bucket must be positive".to_string());
    }
    if args.buckets.is_empty() {
        return Err("--buckets must not be empty".to_string());
    }
    Ok(args)
}

fn parse_buckets(raw: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut buckets = Vec::new();
    for item in raw.split(',') {
        let (q, p) = item
            .split_once('x')
            .ok_or_else(|| format!("invalid bucket {item:?}; expected KxM"))?;
        let bucket = (
            q.parse::<usize>()
                .map_err(|_| format!("invalid bucket {item:?}"))?,
            p.parse::<usize>()
                .map_err(|_| format!("invalid bucket {item:?}"))?,
        );
        if bucket.0 < 3 || bucket.1 < 3 {
            return Err(format!(
                "bucket {item:?} needs at least three sides per factor"
            ));
        }
        if !DEFAULT_BUCKETS.contains(&bucket) {
            return Err(format!(
                "unsupported bucket {item:?}; supported buckets are 3x3,4x4,4x6,6x6"
            ));
        }
        if buckets.contains(&bucket) {
            return Err(format!("duplicate bucket {item:?}"));
        }
        buckets.push(bucket);
    }
    Ok(buckets)
}

fn derive_seed(
    master: u64,
    label: &str,
    bucket: (usize, usize),
    row: usize,
    attempt: usize,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&master.to_le_bytes());
    bytes.extend_from_slice(label.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&(bucket.0 as u64).to_le_bytes());
    bytes.extend_from_slice(&(bucket.1 as u64).to_le_bytes());
    bytes.extend_from_slice(&(row as u64).to_le_bytes());
    bytes.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&bytes).as_bytes()
}

fn seed_u64(seed: [u8; 32]) -> u64 {
    u64::from_le_bytes(seed[..8].try_into().expect("eight-byte prefix"))
}

fn all_facets_active(f: &Factor) -> bool {
    let n = f.normals.len();
    if n < 3 || f.heights.len() != n {
        return false;
    }
    for i in 0..n {
        let j = (i + 1) % n;
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / det;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / det;
        if f.normals
            .iter()
            .zip(&f.heights)
            .any(|(normal, height)| normal[0] * x + normal[1] * y > *height + 1e-9)
        {
            return false;
        }
    }
    true
}

fn area_normalize(mut factor: Factor) -> Option<Factor> {
    if !all_facets_active(&factor) {
        return None;
    }
    let area = polygon_area(&factor.normals, &factor.heights)?;
    if !area.is_finite() || area <= 0.0 {
        return None;
    }
    let scale = area.sqrt().recip();
    factor
        .heights
        .iter_mut()
        .for_each(|height| *height *= scale);
    let normalized_area = polygon_area(&factor.normals, &factor.heights)?;
    (normalized_area.is_finite() && (normalized_area - 1.0).abs() <= 1e-10).then_some(factor)
}

fn generate_factor(sides: usize, seed: [u8; 32]) -> Option<Factor> {
    let mut rng = ChaCha8Rng::from_seed(seed);
    let (normals, heights) = random_polygon_2d(sides, 0.8, 1.2, &mut rng);
    area_normalize(Factor { normals, heights })
}

fn generate_base(
    args: &Args,
    bucket: (usize, usize),
    row: usize,
) -> (Option<SysLandscapePolytopeCache>, Option<usize>, u64, f64) {
    let started = Instant::now();
    let base_seed = seed_u64(derive_seed(args.seed, "base", bucket, row, 0));
    for attempt in 0..args.attempts {
        let q = generate_factor(
            bucket.0,
            derive_seed(args.seed, "base-q", bucket, row, attempt),
        );
        let p = generate_factor(
            bucket.1,
            derive_seed(args.seed, "base-p", bucket, row, attempt),
        );
        let Some((q, p)) = q.zip(p) else {
            continue;
        };
        if let Some(poly) = SysLandscapePolytopeCache::from_lagrangian_product(
            &q.normals, &q.heights, &p.normals, &p.heights,
        ) {
            return (
                Some(poly),
                Some(attempt),
                base_seed,
                started.elapsed().as_secs_f64() * 1000.0,
            );
        }
    }
    (
        None,
        None,
        base_seed,
        started.elapsed().as_secs_f64() * 1000.0,
    )
}

fn identity_matrix_i8() -> [[i8; 4]; 4] {
    [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
}

fn deterministic_u2_i8() -> [[i8; 4]; 4] {
    // Real embedding of diag(i, 1): [[A,-B],[B,A]].
    [[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]]
}

fn deterministic_so4_i8() -> [[i8; 4]; 4] {
    // Orientation-preserving and orthogonal, but anti-symplectic rather than U(2).
    [[-1, 0, 0, 0], [0, -1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
}

fn matrix_from_i8(data: [[i8; 4]; 4]) -> Matrix4<f64> {
    Matrix4::from_fn(|row, col| f64::from(data[row][col]))
}

fn complex_inner(left: &[Complex64; 2], right: &[Complex64; 2]) -> Complex64 {
    left.iter()
        .zip(right)
        .fold(Complex64::default(), |sum, (&a, &b)| sum + a.conj() * b)
}

fn normalize_complex(column: [Complex64; 2]) -> Option<[Complex64; 2]> {
    let norm = column.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    (norm.is_finite() && norm > 1e-14).then(|| column.map(|z| z / norm))
}

fn haar_u2(seed: [u8; 32]) -> Option<Matrix4<f64>> {
    // Complex standard-Gaussian QR. Rotational invariance makes Q Haar on U(2).
    let mut rng = ChaCha8Rng::from_seed(seed);
    let draw = |rng: &mut ChaCha8Rng| {
        Complex64::new(StandardNormal.sample(rng), StandardNormal.sample(rng))
    };
    let c0 = [draw(&mut rng), draw(&mut rng)];
    let c1 = [draw(&mut rng), draw(&mut rng)];
    let q0 = normalize_complex(c0)?;
    let projection = complex_inner(&q0, &c1);
    let q1 = normalize_complex([c1[0] - q0[0] * projection, c1[1] - q0[1] * projection])?;
    let u = [[q0[0], q1[0]], [q0[1], q1[1]]];
    Some(Matrix4::from_fn(|row, col| match (row < 2, col < 2) {
        (true, true) => u[row][col].re,
        (true, false) => -u[row][col - 2].im,
        (false, true) => u[row - 2][col].im,
        (false, false) => u[row - 2][col - 2].re,
    }))
}

fn real_inner(left: &[f64; 4], right: &[f64; 4]) -> f64 {
    (0..4).map(|i| left[i] * right[i]).sum()
}

fn normalize_real(column: [f64; 4]) -> Option<[f64; 4]> {
    let norm = real_inner(&column, &column).sqrt();
    (norm.is_finite() && norm > 1e-14).then(|| column.map(|x| x / norm))
}

fn haar_so4_once(seed: [u8; 32]) -> Option<Matrix4<f64>> {
    // Real standard-Gaussian QR. The determinant correction maps Haar O(4) to SO(4).
    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut q = [[0.0; 4]; 4]; // columns
    for col in 0..4 {
        let mut v = std::array::from_fn(|_| StandardNormal.sample(&mut rng));
        // Reorthogonalize once to keep the residual tight for the smoke contract.
        for _ in 0..2 {
            for prior in q.iter().take(col) {
                let projection = real_inner(prior, &v);
                for i in 0..4 {
                    v[i] -= projection * prior[i];
                }
            }
        }
        q[col] = normalize_real(v)?;
    }
    let mut matrix = Matrix4::from_fn(|row, col| q[col][row]);
    if matrix.determinant() < 0.0 {
        for row in 0..4 {
            matrix[(row, 3)] = -matrix[(row, 3)];
        }
    }
    Some(matrix)
}

fn j4() -> Matrix4<f64> {
    Matrix4::new(
        0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    )
}

fn orthogonality_residual(matrix: &Matrix4<f64>) -> f64 {
    (matrix.transpose() * matrix - Matrix4::identity()).norm()
}

fn symplectic_residual(matrix: &Matrix4<f64>) -> f64 {
    (matrix.transpose() * j4() * matrix - j4()).norm()
}

fn map_spec(
    master_seed: u64,
    bucket: (usize, usize),
    row: usize,
    variant: Variant,
) -> Result<MapSpec, String> {
    let seed = derive_seed(master_seed, variant.name(), bucket, row, 0);
    let map_seed = seed_u64(seed);
    let (matrix, exact_signed_matrix) = match variant {
        Variant::Identity => (
            matrix_from_i8(identity_matrix_i8()),
            Some(identity_matrix_i8()),
        ),
        Variant::DeterministicU2 => (
            matrix_from_i8(deterministic_u2_i8()),
            Some(deterministic_u2_i8()),
        ),
        Variant::HaarU2 => (
            haar_u2(seed).ok_or_else(|| "degenerate complex Gaussian QR".to_string())?,
            None,
        ),
        Variant::DeterministicSo4 => (
            matrix_from_i8(deterministic_so4_i8()),
            Some(deterministic_so4_i8()),
        ),
        Variant::HaarSo4 => (
            haar_so4_once(seed).ok_or_else(|| "degenerate real Gaussian QR".to_string())?,
            None,
        ),
    };
    Ok(MapSpec {
        variant,
        map_seed,
        matrix,
        exact_signed_matrix,
    })
}

fn matrix_rows(matrix: &Matrix4<f64>) -> [[f64; 4]; 4] {
    std::array::from_fn(|row| std::array::from_fn(|col| matrix[(row, col)]))
}

fn vector_rows(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn incidence_signature(poly: &SysLandscapePolytopeCache) -> Vec<Vec<usize>> {
    let incidence = &poly.vertex_facet_incidence;
    let mut signature: Vec<Vec<usize>> = (0..incidence.nrows())
        .map(|vertex| {
            (0..incidence.ncols())
                .filter(|&facet| incidence[(vertex, facet)])
                .collect()
        })
        .collect();
    signature.sort();
    signature
}

fn omega_signature(poly: &SysLandscapePolytopeCache) -> Vec<i8> {
    (0..poly.omega_signs.nrows())
        .flat_map(|row| (0..poly.omega_signs.ncols()).map(move |col| poly.omega_signs[(row, col)]))
        .collect()
}

fn geometry_id(poly: &SysLandscapePolytopeCache) -> String {
    let mut hasher = blake3::Hasher::new();
    for row in &poly.dual_vertices {
        for value in row {
            hasher.update(value.numer().to_string().as_bytes());
            hasher.update(b"/");
            hasher.update(value.denom().to_string().as_bytes());
            hasher.update(b";");
        }
        hasher.update(b"|");
    }
    hasher.finalize().to_hex().to_string()
}

fn transform_duals(matrix: &Matrix4<f64>, duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let inverse_transpose = matrix.try_inverse()?.transpose();
    Some(duals.iter().map(|dual| inverse_transpose * dual).collect())
}

fn max_scaled_gram_error(base: &[Vector4<f64>], transformed: &[Vector4<f64>]) -> (f64, f64) {
    let mut error: f64 = 0.0;
    let mut scale: f64 = 1.0;
    for i in 0..base.len() {
        for j in 0..base.len() {
            let original = base[i].dot(&base[j]);
            let changed = transformed[i].dot(&transformed[j]);
            error = error.max((changed - original).abs());
            scale = scale.max(original.abs());
        }
    }
    (error, scale)
}

fn max_scaled_omega_error(base: &[Vector4<f64>], transformed: &[Vector4<f64>]) -> (f64, f64) {
    let mut error: f64 = 0.0;
    let mut scale: f64 = 1.0;
    for i in 0..base.len() {
        for j in 0..base.len() {
            let original = omega0(&base[i], &base[j]);
            let changed = omega0(&transformed[i], &transformed[j]);
            error = error.max((changed - original).abs());
            scale = scale.max(original.abs());
        }
    }
    (error, scale)
}

fn exact_signed_transform(
    matrix: [[i8; 4]; 4],
    vectors: &[[BigRational; 4]],
) -> Vec<[BigRational; 4]> {
    vectors
        .iter()
        .map(|vector| {
            std::array::from_fn(|row| {
                (0..4).fold(BigRational::from_integer(0.into()), |sum, col| {
                    let coefficient = i64::from(matrix[row][col]);
                    sum + BigRational::from_integer(coefficient.into()) * &vector[col]
                })
            })
        })
        .collect()
}

fn rational_multiset(mut data: Vec<[String; 4]>) -> Vec<[String; 4]> {
    data.sort();
    data
}

fn exact_signed_payload_matches(
    map: &MapSpec,
    base: &SysLandscapePolytopeCache,
    transformed: &SysLandscapePolytopeCache,
) -> Option<bool> {
    let matrix = map.exact_signed_matrix?;
    let expected_duals = exact_signed_transform(matrix, &base.dual_vertices);
    let expected_vertices = exact_signed_transform(matrix, &base.vertices);
    Some(
        expected_duals == transformed.dual_vertices
            && rational_multiset(rational_vec4_to_strings(&expected_vertices))
                == rational_multiset(rational_vec4_to_strings(&transformed.vertices)),
    )
}

fn empty_row(
    args: &Args,
    bucket: (usize, usize),
    row_index: usize,
    base_seed: u64,
    generation_ms: f64,
    map: &MapSpec,
    map_error: Option<String>,
) -> SmokeRow {
    let bucket_name = format!("{}x{}", bucket.0, bucket.1);
    let base_id = format!(
        "generator-orientation-v1/base/seed={}/bucket={bucket_name}/row={row_index}",
        args.seed
    );
    let transformed_id = format!("{base_id}/map={}", map.variant.name());
    SmokeRow {
        schema: SCHEMA_ROW,
        sample_id: transformed_id.clone(),
        base_id,
        transformed_id,
        base_geometry_id: None,
        transformed_geometry_id: None,
        seed: args.seed,
        base_seed,
        row_index,
        accepted_attempt: None,
        bucket: bucket_name,
        q_sides: bucket.0,
        p_sides: bucket.1,
        base_accepted: false,
        base_error: Some("base rejected after bounded attempts".to_string()),
        base_facet_count: None,
        base_vertex_count: None,
        facet_count: None,
        vertex_count: None,
        reconstruction_status: "base_rejected".to_string(),
        reconstruction_error: map_error,
        map_variant: map.variant.name().to_string(),
        map_family: map.variant.family(),
        map_mode: map.variant.mode(),
        map_status: "generated",
        map_error: None,
        map_seed: map.map_seed,
        matrix_available: true,
        matrix_row_major: matrix_rows(&map.matrix),
        coordinate_order: "q1,q2,p1,p2",
        map_direction: "primal",
        dual_action: "inverse_transpose",
        determinant: map.matrix.determinant(),
        orthogonality_residual_frobenius: orthogonality_residual(&map.matrix),
        symplectic_u2_residual_frobenius: symplectic_residual(&map.matrix),
        dual_gram_max_abs_error: None,
        floating_omega_max_abs_error: None,
        transformed_dual_vertices_f64: Vec::new(),
        transformed_dual_vertices_rational: Vec::new(),
        reconstructed_primal_vertices_rational: Vec::new(),
        base_exact_volume_as_f64: None,
        exact_volume_as_f64: None,
        relative_volume_change: None,
        labeled_incidence_signature: Vec::new(),
        labeled_incidence_matches_base: None,
        omega_sign_signature: Vec::new(),
        omega_exactly_matches_base: None,
        omega_comparison_status: "not_compared_base_rejected".to_string(),
        exact_signed_payload_matches: None,
        semantic_invariants_passed: false,
        invariant_failures: vec!["base_generation_exhausted".to_string()],
        generation_ms,
        transform_ms: 0.0,
        reconstruction_ms: 0.0,
    }
}

#[allow(clippy::too_many_arguments)]
fn map_failure_row(
    args: &Args,
    bucket: (usize, usize),
    row_index: usize,
    accepted_attempt: Option<usize>,
    base_seed: u64,
    generation_ms: f64,
    base: Option<&SysLandscapePolytopeCache>,
    variant: Variant,
    error: String,
) -> SmokeRow {
    let placeholder = MapSpec {
        variant,
        map_seed: seed_u64(derive_seed(args.seed, variant.name(), bucket, row_index, 0)),
        matrix: Matrix4::zeros(),
        exact_signed_matrix: None,
    };
    let mut row = empty_row(
        args,
        bucket,
        row_index,
        base_seed,
        generation_ms,
        &placeholder,
        Some(error.clone()),
    );
    row.map_status = "rejected";
    row.map_error = Some(error.clone());
    row.matrix_available = false;
    row.reconstruction_status = "map_rejected".to_string();
    row.reconstruction_error = Some(error);
    if let (Some(base), Some(attempt)) = (base, accepted_attempt) {
        row.invariant_failures = vec!["map_generation_rejected".to_string()];
        row.base_accepted = true;
        row.base_error = None;
        row.accepted_attempt = Some(attempt);
        row.base_facet_count = Some(base.facet_count());
        row.base_vertex_count = Some(base.vertices.len());
        row.base_geometry_id = Some(geometry_id(base));
        row.base_exact_volume_as_f64 = Some(exact_volume_as_f64(
            &base.vertices,
            &base.vertex_facet_incidence,
        ));
        row.base_id = format!(
            "generator-orientation-v1/base/seed={}/bucket={}/row={row_index}/attempt={attempt}",
            args.seed, row.bucket
        );
        row.transformed_id = format!("{}/map={}", row.base_id, variant.name());
        row.sample_id = row.transformed_id.clone();
    } else {
        row.invariant_failures
            .push("map_generation_rejected".to_string());
    }
    row
}

fn evaluate_map(
    args: &Args,
    bucket: (usize, usize),
    row_index: usize,
    accepted_attempt: usize,
    base_seed: u64,
    generation_ms: f64,
    base: &SysLandscapePolytopeCache,
    map: &MapSpec,
) -> SmokeRow {
    let bucket_name = format!("{}x{}", bucket.0, bucket.1);
    let base_id = format!(
        "generator-orientation-v1/base/seed={}/bucket={bucket_name}/row={row_index}/attempt={accepted_attempt}",
        args.seed
    );
    let transformed_id = format!("{base_id}/map={}", map.variant.name());
    let determinant = map.matrix.determinant();
    let orthogonal = orthogonality_residual(&map.matrix);
    let symplectic = symplectic_residual(&map.matrix);
    let transform_started = Instant::now();
    let transformed_duals = transform_duals(&map.matrix, &base.dual_vertices_f64);
    let transform_ms = transform_started.elapsed().as_secs_f64() * 1000.0;
    let base_volume = exact_volume_as_f64(&base.vertices, &base.vertex_facet_incidence);
    let mut failures = Vec::new();
    if !map.matrix.iter().all(|value| value.is_finite()) {
        failures.push("matrix_nonfinite".to_string());
    }
    if !determinant.is_finite() || determinant <= 0.0 || (determinant - 1.0).abs() > DET_TOL {
        failures.push("determinant_not_near_positive_one".to_string());
    }
    if !orthogonal.is_finite() || orthogonal > MATRIX_TOL {
        failures.push("orthogonality_residual".to_string());
    }
    if map.variant.family() == "u2" && (!symplectic.is_finite() || symplectic > MATRIX_TOL) {
        failures.push("u2_symplectic_residual".to_string());
    }
    if map.variant == Variant::HaarSo4 && symplectic <= SO4_NON_U2_MIN_RESIDUAL {
        failures.push("haar_so4_not_demonstrably_non_u2".to_string());
    }
    let Some(transformed_duals) = transformed_duals else {
        failures.push("inverse_transpose_failed".to_string());
        return SmokeRow {
            sample_id: transformed_id.clone(),
            base_id,
            transformed_id,
            base_geometry_id: Some(geometry_id(base)),
            base_accepted: true,
            base_error: None,
            base_facet_count: Some(base.facet_count()),
            base_vertex_count: Some(base.vertices.len()),
            base_exact_volume_as_f64: Some(base_volume),
            accepted_attempt: Some(accepted_attempt),
            reconstruction_status: "transform_failed".to_string(),
            invariant_failures: failures,
            ..empty_row(
                args,
                bucket,
                row_index,
                base_seed,
                generation_ms,
                map,
                Some("matrix inverse failed".to_string()),
            )
        };
    };
    let transformed_duals_f64 = vector_rows(&transformed_duals);
    let (gram_error, gram_scale) =
        max_scaled_gram_error(&base.dual_vertices_f64, &transformed_duals);
    if gram_error > EUCLIDEAN_TOL * gram_scale {
        failures.push("dual_gram_not_preserved".to_string());
    }
    let (omega_error, omega_scale) =
        max_scaled_omega_error(&base.dual_vertices_f64, &transformed_duals);
    if map.variant == Variant::HaarU2 && omega_error > FLOATING_OMEGA_TOL * omega_scale {
        failures.push("haar_u2_floating_omega_not_preserved".to_string());
    }
    let reconstruction_started = Instant::now();
    let reconstructed = SysLandscapePolytopeCache::from_f64_dual_vertices(transformed_duals);
    let reconstruction_ms = reconstruction_started.elapsed().as_secs_f64() * 1000.0;
    let Some(reconstructed) = reconstructed else {
        failures.push("exact_reconstruction_failed".to_string());
        return SmokeRow {
            schema: SCHEMA_ROW,
            sample_id: transformed_id.clone(),
            base_id,
            transformed_id,
            base_geometry_id: Some(geometry_id(base)),
            transformed_geometry_id: None,
            seed: args.seed,
            base_seed,
            row_index,
            accepted_attempt: Some(accepted_attempt),
            bucket: bucket_name,
            q_sides: bucket.0,
            p_sides: bucket.1,
            base_accepted: true,
            base_error: None,
            base_facet_count: Some(base.facet_count()),
            base_vertex_count: Some(base.vertices.len()),
            facet_count: None,
            vertex_count: None,
            reconstruction_status: "rejected".to_string(),
            reconstruction_error: Some(
                "from_f64_dual_vertices rejected transformed duals".to_string(),
            ),
            map_variant: map.variant.name().to_string(),
            map_family: map.variant.family(),
            map_mode: map.variant.mode(),
            map_status: "generated",
            map_error: None,
            map_seed: map.map_seed,
            matrix_available: true,
            matrix_row_major: matrix_rows(&map.matrix),
            coordinate_order: "q1,q2,p1,p2",
            map_direction: "primal",
            dual_action: "inverse_transpose",
            determinant,
            orthogonality_residual_frobenius: orthogonal,
            symplectic_u2_residual_frobenius: symplectic,
            dual_gram_max_abs_error: Some(gram_error),
            floating_omega_max_abs_error: Some(omega_error),
            transformed_dual_vertices_f64: transformed_duals_f64,
            transformed_dual_vertices_rational: Vec::new(),
            reconstructed_primal_vertices_rational: Vec::new(),
            base_exact_volume_as_f64: Some(base_volume),
            exact_volume_as_f64: None,
            relative_volume_change: None,
            labeled_incidence_signature: Vec::new(),
            labeled_incidence_matches_base: None,
            omega_sign_signature: Vec::new(),
            omega_exactly_matches_base: None,
            omega_comparison_status: "not_compared_reconstruction_failed".to_string(),
            exact_signed_payload_matches: None,
            semantic_invariants_passed: false,
            invariant_failures: failures,
            generation_ms,
            transform_ms,
            reconstruction_ms,
        };
    };
    let exact_volume = exact_volume_as_f64(
        &reconstructed.vertices,
        &reconstructed.vertex_facet_incidence,
    );
    let relative_volume = (exact_volume - base_volume) / base_volume;
    let incidence = incidence_signature(&reconstructed);
    let incidence_matches = incidence == incidence_signature(base);
    let omega = omega_signature(&reconstructed);
    let omega_exact_match = omega == omega_signature(base);
    let exact_payload = exact_signed_payload_matches(map, base, &reconstructed);
    if reconstructed.facet_count() != base.facet_count() {
        failures.push("facet_count_changed".to_string());
    }
    if reconstructed.vertices.len() != base.vertices.len() {
        failures.push("vertex_count_changed".to_string());
    }
    if !incidence_matches {
        failures.push("labeled_incidence_changed".to_string());
    }
    if !relative_volume.is_finite() || relative_volume.abs() > VOLUME_REL_TOL {
        failures.push("relative_volume_change".to_string());
    }
    if map.exact_signed_matrix.is_some() && exact_payload != Some(true) {
        failures.push("exact_signed_payload_mismatch".to_string());
    }
    match map.variant {
        Variant::Identity => {
            if reconstructed.dual_vertices != base.dual_vertices
                || reconstructed.dual_vertices_f64 != base.dual_vertices_f64
                || reconstructed.vertices != base.vertices
                || reconstructed.vertices_f64 != base.vertices_f64
                || reconstructed.vertex_facet_incidence != base.vertex_facet_incidence
                || reconstructed.omega_signs != base.omega_signs
                || geometry_id(&reconstructed) != geometry_id(base)
            {
                failures.push("identity_exact_payload_mismatch".to_string());
            }
        }
        Variant::DeterministicU2 if !omega_exact_match => {
            failures.push("deterministic_u2_omega_sign_changed".to_string());
        }
        Variant::HaarU2
        | Variant::DeterministicSo4
        | Variant::HaarSo4
        | Variant::DeterministicU2 => {}
    }
    let omega_status = match map.variant {
        Variant::Identity | Variant::DeterministicU2 => {
            if omega_exact_match {
                "exact_invariant"
            } else {
                "unexpected_exact_change"
            }
        }
        Variant::HaarU2 => {
            if omega_error <= FLOATING_OMEGA_TOL * omega_scale {
                if omega_exact_match {
                    "floating_invariant_exact_signs_equal"
                } else {
                    "floating_invariant_rationalized_sign_drift"
                }
            } else {
                "floating_invariant_failed"
            }
        }
        Variant::DeterministicSo4 | Variant::HaarSo4 => {
            if omega_exact_match {
                "measured_equal_not_required"
            } else {
                "measured_changed_expected"
            }
        }
    };
    SmokeRow {
        schema: SCHEMA_ROW,
        sample_id: transformed_id.clone(),
        base_id,
        transformed_id,
        base_geometry_id: Some(geometry_id(base)),
        transformed_geometry_id: Some(geometry_id(&reconstructed)),
        seed: args.seed,
        base_seed,
        row_index,
        accepted_attempt: Some(accepted_attempt),
        bucket: bucket_name,
        q_sides: bucket.0,
        p_sides: bucket.1,
        base_accepted: true,
        base_error: None,
        base_facet_count: Some(base.facet_count()),
        base_vertex_count: Some(base.vertices.len()),
        facet_count: Some(reconstructed.facet_count()),
        vertex_count: Some(reconstructed.vertices.len()),
        reconstruction_status: "reconstructed".to_string(),
        reconstruction_error: None,
        map_variant: map.variant.name().to_string(),
        map_family: map.variant.family(),
        map_mode: map.variant.mode(),
        map_status: "generated",
        map_error: None,
        map_seed: map.map_seed,
        matrix_available: true,
        matrix_row_major: matrix_rows(&map.matrix),
        coordinate_order: "q1,q2,p1,p2",
        map_direction: "primal",
        dual_action: "inverse_transpose",
        determinant,
        orthogonality_residual_frobenius: orthogonal,
        symplectic_u2_residual_frobenius: symplectic,
        dual_gram_max_abs_error: Some(gram_error),
        floating_omega_max_abs_error: Some(omega_error),
        transformed_dual_vertices_f64: transformed_duals_f64,
        transformed_dual_vertices_rational: rational_vec4_to_strings(&reconstructed.dual_vertices),
        reconstructed_primal_vertices_rational: rational_vec4_to_strings(&reconstructed.vertices),
        base_exact_volume_as_f64: Some(base_volume),
        exact_volume_as_f64: Some(exact_volume),
        relative_volume_change: Some(relative_volume),
        labeled_incidence_signature: incidence,
        labeled_incidence_matches_base: Some(incidence_matches),
        omega_sign_signature: omega,
        omega_exactly_matches_base: Some(omega_exact_match),
        omega_comparison_status: omega_status.to_string(),
        exact_signed_payload_matches: exact_payload,
        semantic_invariants_passed: failures.is_empty(),
        invariant_failures: failures,
        generation_ms,
        transform_ms,
        reconstruction_ms,
    }
}

fn source_provenance() -> (String, bool) {
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_or(true, |output| {
            !output.status.success() || !output.stdout.is_empty()
        });
    (revision, dirty)
}

fn map_algorithms() -> BTreeMap<String, &'static str> {
    BTreeMap::from([
        ("identity".to_string(), "exact integer identity"),
        ("u2-deterministic".to_string(), "exact signed-permutation real embedding of complex diag(i,1)"),
        ("u2-haar".to_string(), "two independent complex standard-Gaussian columns, modified Gram-Schmidt QR; rotational invariance yields Haar U(2)"),
        ("so4-deterministic".to_string(), "exact signed diagonal diag(-1,-1,1,1), orientation-preserving orthogonal and anti-symplectic"),
        ("so4-haar".to_string(), "one seeded draw of four real standard-Gaussian columns, twice-reorthogonalized modified Gram-Schmidt QR, and final-column determinant correction; no residual-based conditioning or resampling"),
    ])
}

fn packet_passes(rows: &[SmokeRow], expected_row_count: usize) -> bool {
    rows.len() == expected_row_count
        && rows.iter().all(|row| {
            row.reconstruction_status == "reconstructed" && row.semantic_invariants_passed
        })
}

fn run(args: Args) -> Result<bool, String> {
    // Capture repository state before creating or overwriting the packet's own
    // outputs, so generated evidence cannot mark its producing source dirty.
    let (source_revision, source_dirty) = source_provenance();
    create_dir_all(&args.out_dir).map_err(|error| format!("create out-dir: {error}"))?;
    let rows_path = args.out_dir.join("rows.jsonl");
    let report_path = args.out_dir.join("report.json");
    let mut writer = BufWriter::new(
        File::create(&rows_path)
            .map_err(|error| format!("create {}: {error}", rows_path.display()))?,
    );
    let mut rows = Vec::new();
    let mut accepted_base_count = 0;
    for &bucket in &args.buckets {
        for row_index in 0..args.rows_per_bucket {
            let (base, accepted_attempt, base_seed, generation_ms) =
                generate_base(&args, bucket, row_index);
            if base.is_some() {
                accepted_base_count += 1;
            }
            for variant in Variant::ALL {
                let row = match map_spec(args.seed, bucket, row_index, variant) {
                    Ok(map) => match (&base, accepted_attempt) {
                        (Some(base), Some(attempt)) => evaluate_map(
                            &args,
                            bucket,
                            row_index,
                            attempt,
                            base_seed,
                            generation_ms,
                            base,
                            &map,
                        ),
                        _ => empty_row(
                            &args,
                            bucket,
                            row_index,
                            base_seed,
                            generation_ms,
                            &map,
                            None,
                        ),
                    },
                    Err(error) => map_failure_row(
                        &args,
                        bucket,
                        row_index,
                        accepted_attempt,
                        base_seed,
                        generation_ms,
                        base.as_ref(),
                        variant,
                        error,
                    ),
                };
                serde_json::to_writer(&mut writer, &row)
                    .map_err(|error| format!("serialize row: {error}"))?;
                writeln!(writer).map_err(|error| format!("write row newline: {error}"))?;
                rows.push(row);
            }
        }
    }
    writer
        .flush()
        .map_err(|error| format!("flush rows: {error}"))?;
    let mut status_counts = BTreeMap::new();
    let mut invariant_failure_counts = BTreeMap::new();
    let mut summaries: BTreeMap<(String, String), BucketVariantSummary> = BTreeMap::new();
    for row in &rows {
        *status_counts
            .entry(row.reconstruction_status.clone())
            .or_insert(0) += 1;
        for failure in &row.invariant_failures {
            *invariant_failure_counts.entry(failure.clone()).or_insert(0) += 1;
        }
        let summary = summaries
            .entry((row.bucket.clone(), row.map_variant.clone()))
            .or_insert_with(|| BucketVariantSummary {
                bucket: row.bucket.clone(),
                map_variant: row.map_variant.clone(),
                ..BucketVariantSummary::default()
            });
        summary.requested += 1;
        summary.base_accepted += usize::from(row.base_accepted);
        summary.reconstructed += usize::from(row.reconstruction_status == "reconstructed");
        summary.invariant_passed += usize::from(row.semantic_invariants_passed);
        summary.invariant_failed += usize::from(!row.invariant_failures.is_empty());
        summary.total_generation_ms += row.generation_ms;
        summary.total_transform_ms += row.transform_ms;
        summary.total_reconstruction_ms += row.reconstruction_ms;
    }
    let invariant_failure_count = rows
        .iter()
        .filter(|row| !row.invariant_failures.is_empty())
        .count();
    let expected_row_count = args.buckets.len() * args.rows_per_bucket * Variant::ALL.len();
    let all_requested_rows_passed = packet_passes(&rows, expected_row_count);
    let report = Report {
        schema: SCHEMA_REPORT,
        command: std::env::args().collect::<Vec<_>>().join(" "),
        source_revision,
        source_dirty,
        seed: args.seed,
        max_attempts_per_base: args.attempts,
        rows_per_bucket: args.rows_per_bucket,
        requested_buckets: args
            .buckets
            .iter()
            .map(|(q, p)| format!("{q}x{p}"))
            .collect(),
        requested_base_count: args.buckets.len() * args.rows_per_bucket,
        accepted_base_count,
        observed_row_count: rows.len(),
        expected_variants_per_accepted_base: Variant::ALL.len(),
        status_counts,
        by_map_variant_and_bucket: summaries.into_values().collect(),
        invariant_failure_count,
        invariant_failure_counts,
        all_requested_rows_passed,
        map_algorithms: map_algorithms(),
        tolerances: Tolerances {
            matrix_frobenius: MATRIX_TOL,
            determinant_abs: DET_TOL,
            dual_gram_scaled_abs: EUCLIDEAN_TOL,
            volume_relative: VOLUME_REL_TOL,
            floating_omega_scaled_abs: FLOATING_OMEGA_TOL,
            haar_so4_discriminating_min_symplectic_residual: SO4_NON_U2_MIN_RESIDUAL,
        },
        coordinate_order: "q1,q2,p1,p2",
        map_direction: "primal",
        dual_action: "inverse_transpose",
        interpretation_boundary: "This is a deterministic target-free semantic smoke for reconstruction and invariant conventions. It is not population evidence, does not call capacity, and is not a sys result.",
    };
    serde_json::to_writer_pretty(
        File::create(&report_path)
            .map_err(|error| format!("create {}: {error}", report_path.display()))?,
        &report,
    )
    .map_err(|error| format!("serialize report: {error}"))?;
    println!(
        "wrote {} rows ({} accepted bases, {} semantic failures) to {}",
        rows.len(),
        accepted_base_count,
        invariant_failure_count,
        args.out_dir.display()
    );
    Ok(all_requested_rows_passed)
}

fn main() {
    let args = parse_args().unwrap_or_else(|error| {
        eprintln!("argument error: {error}");
        std::process::exit(2);
    });
    match run(args) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("semantic invariant failures were written to the report");
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("producer error: {error}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_args(seed: u64, attempts: usize) -> Args {
        Args {
            out_dir: PathBuf::from("/tmp/generator-orientation-smoke-test-unused"),
            seed,
            attempts,
            rows_per_bucket: 1,
            buckets: vec![(3, 3)],
        }
    }

    #[test]
    fn u2_embedding_uses_q_block_p_block_convention() {
        let matrix = matrix_from_i8(deterministic_u2_i8());
        let input = Vector4::new(2.0, 3.0, 5.0, 7.0);
        assert_eq!(matrix * input, Vector4::new(-5.0, 3.0, 2.0, 7.0));
        assert_eq!(matrix.determinant(), 1.0);
        assert_eq!(orthogonality_residual(&matrix), 0.0);
        assert_eq!(symplectic_residual(&matrix), 0.0);
    }

    #[test]
    fn haar_u2_is_deterministic_unitary_and_symplectic() {
        let seed = derive_seed(7, "test-u2", (4, 6), 0, 0);
        let first = haar_u2(seed).expect("nondegenerate QR");
        let second = haar_u2(seed).expect("nondegenerate QR");
        assert_eq!(first, second);
        assert!(orthogonality_residual(&first) < 1e-12);
        assert!(symplectic_residual(&first) < 1e-12);
        assert!((first.determinant() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn deterministic_so4_control_is_orientation_preserving_but_not_u2() {
        let matrix = matrix_from_i8(deterministic_so4_i8());
        assert_eq!(matrix.determinant(), 1.0);
        assert_eq!(orthogonality_residual(&matrix), 0.0);
        assert!(symplectic_residual(&matrix) > SO4_NON_U2_MIN_RESIDUAL);
    }

    #[test]
    fn haar_so4_is_one_unconditioned_deterministic_special_orthogonal_draw() {
        let first = map_spec(9, (3, 3), 0, Variant::HaarSo4).expect("Haar SO(4)");
        let second = map_spec(9, (3, 3), 0, Variant::HaarSo4).expect("Haar SO(4)");
        let direct = haar_so4_once(derive_seed(9, "so4-haar", (3, 3), 0, 0))
            .expect("same direct Gaussian QR draw");
        assert_eq!(first.matrix, second.matrix);
        assert_eq!(first.matrix, direct);
        assert!(orthogonality_residual(&first.matrix) < 1e-12);
        assert!((first.matrix.determinant() - 1.0).abs() < 1e-12);
        assert!(symplectic_residual(&first.matrix) > SO4_NON_U2_MIN_RESIDUAL);
    }

    #[test]
    fn seed_zero_one_attempt_base_rejection_is_explicit_and_fails_closed() {
        let args = test_args(0, 1);
        let (base, accepted_attempt, base_seed, generation_ms) = generate_base(&args, (3, 3), 0);
        assert!(
            base.is_none(),
            "reviewer witness must exhaust its one attempt"
        );
        assert_eq!(accepted_attempt, None);
        let map = map_spec(args.seed, (3, 3), 0, Variant::Identity).unwrap();
        let row = empty_row(&args, (3, 3), 0, base_seed, generation_ms, &map, None);
        assert_eq!(row.reconstruction_status, "base_rejected");
        assert_eq!(row.invariant_failures, ["base_generation_exhausted"]);
        assert!(!row.semantic_invariants_passed);
        assert!(!packet_passes(&[row], 1));
    }

    #[test]
    fn map_generation_rejection_is_explicit_and_fails_closed() {
        let args = test_args(7, 1);
        let (q_normals, q_heights) = symplectic::geom::polygon::regular_polygon_2d(3, 1.0);
        let (p_normals, p_heights) = symplectic::geom::polygon::regular_polygon_2d(3, 1.0);
        let base = SysLandscapePolytopeCache::from_lagrangian_product(
            &q_normals, &q_heights, &p_normals, &p_heights,
        )
        .expect("regular triangle product");
        let row = map_failure_row(
            &args,
            (3, 3),
            0,
            Some(0),
            17,
            0.0,
            Some(&base),
            Variant::HaarSo4,
            "forced numerical degeneracy".to_string(),
        );
        assert_eq!(row.reconstruction_status, "map_rejected");
        assert_eq!(row.invariant_failures, ["map_generation_rejected"]);
        assert!(!row.semantic_invariants_passed);
        assert!(!packet_passes(&[row], 1));
    }

    #[test]
    fn inverse_transpose_dual_action_preserves_inequality_pairing() {
        let matrix = map_spec(11, (4, 6), 0, Variant::HaarSo4)
            .expect("Haar SO(4)")
            .matrix;
        let normal = Vector4::new(1.0, -2.0, 3.0, 0.5);
        let point = Vector4::new(-0.25, 2.0, 1.5, -3.0);
        let transformed_normal = matrix.try_inverse().unwrap().transpose() * normal;
        let transformed_point = matrix * point;
        assert!((transformed_normal.dot(&transformed_point) - normal.dot(&point)).abs() < 1e-12);
    }

    #[test]
    fn incidence_signature_ignores_vertex_order_but_preserves_facet_labels() {
        let incidence_a =
            nalgebra::DMatrix::from_row_slice(2, 3, &[true, false, true, false, true, true]);
        let incidence_b =
            nalgebra::DMatrix::from_row_slice(2, 3, &[false, true, true, true, false, true]);
        let signature = |incidence: &nalgebra::DMatrix<bool>| {
            let mut rows: Vec<Vec<usize>> = (0..incidence.nrows())
                .map(|row| {
                    (0..incidence.ncols())
                        .filter(|&col| incidence[(row, col)])
                        .collect()
                })
                .collect();
            rows.sort();
            rows
        };
        assert_eq!(signature(&incidence_a), signature(&incidence_b));
        let relabeled =
            nalgebra::DMatrix::from_row_slice(2, 3, &[true, true, false, true, false, true]);
        assert_ne!(signature(&incidence_a), signature(&relabeled));
    }

    #[test]
    fn floating_omega_check_accepts_u2_and_rejects_non_u2_control() {
        let duals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let u2 = matrix_from_i8(deterministic_u2_i8());
        let so4 = matrix_from_i8(deterministic_so4_i8());
        let u2_duals = transform_duals(&u2, &duals).unwrap();
        let so4_duals = transform_duals(&so4, &duals).unwrap();
        assert_eq!(max_scaled_omega_error(&duals, &u2_duals).0, 0.0);
        assert!(max_scaled_omega_error(&duals, &so4_duals).0 > 1.0);
    }
}
