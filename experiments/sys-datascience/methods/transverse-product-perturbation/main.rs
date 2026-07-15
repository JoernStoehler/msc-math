//! Target-free transverse perturbations of product polytopes.
//!
//! The perturbation coordinates are normalized dual inequalities `a_i.x <= 1`
//! in `(q1,q2,p1,p2)` order.  A numerical Euclidean complement of the actual
//! scale/translation/`sp(4)` tangent span is used only as a declared local
//! section; it is not a canonical quotient metric.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{DMatrix, DVector, Matrix4, Vector2, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command,
};
use symplectic::{geom::polygon::random_polygon_2d, omega0};

const BUCKETS: &[(usize, usize)] = &[(3, 3), (4, 4), (4, 6), (6, 6)];
const FRACTIONS: &[f64] = &[0.0, 0.125, 0.25, 0.5, 0.75];
const DIRECTIONS_PER_FIXTURE: usize = 2;
const COORDINATE_ORDER: &str = "q1,q2,p1,p2";

#[derive(Clone)]
struct Factor {
    n: Vec<Vector2<f64>>,
    h: Vec<f64>,
}

#[derive(Clone)]
struct Fixture {
    bucket: (usize, usize),
    base_attempt: usize,
    base: SysLandscapePolytopeCache,
    tangent: TangentSpan,
}

#[derive(Clone)]
struct TangentSpan {
    basis: DMatrix<f64>,
    rank: usize,
    singular_values: Vec<f64>,
}

#[derive(Clone)]
struct Check {
    cache: Option<SysLandscapePolytopeCache>,
    same_incidence: bool,
    full_dimensional: bool,
    bounded: bool,
    irredundant: bool,
    volume: Option<f64>,
    failures: Vec<String>,
}

#[derive(Serialize)]
struct Row {
    schema: &'static str,
    id: String,
    seed: u64,
    bucket: String,
    direction: usize,
    fraction: f64,
    epsilon: f64,
    boundary_epsilon: f64,
    boundary_censored: bool,
    coordinate_order: &'static str,
    raw_direction: Vec<f64>,
    projected_direction: Vec<f64>,
    raw_direct_distance: f64,
    sys_orbit_section_distance: f64,
    tangent_residual: f64,
    pure_orbit_projection_residual: f64,
    synthetic_transverse_survival: f64,
    finite_difference_scale_residual: f64,
    finite_difference_translation_residual: f64,
    finite_difference_sp4_residual: f64,
    exact_reconstruction_status: &'static str,
    same_face_lattice: bool,
    same_source_incidence: bool,
    full_dimensional: bool,
    bounded: bool,
    irredundant: bool,
    base_volume: f64,
    response_volume: Option<f64>,
    volume_ratio: Option<f64>,
    volume_normalized_reconstruction: bool,
    normalized_volume_ratio: Option<f64>,
    euclidean_feature_distance: Option<f64>,
    symplectic_feature_distance: Option<f64>,
    product_block_residual: Option<f64>,
    combinatorial_product_preserved: bool,
    failures: Vec<String>,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    seed: u64,
    fixtures: usize,
    directions_per_fixture: usize,
    fractions: Vec<f64>,
    rows: usize,
    passed: usize,
    failed: usize,
    retained_fixture_counts: BTreeMap<String, usize>,
    tangent_ranks: BTreeMap<String, usize>,
    tangent_min_singular_values: BTreeMap<String, f64>,
    boundary_censored_rows: usize,
    source_revision: String,
    source_repository_tree: String,
    source_dirty: bool,
    producer_source_sha256: String,
    cargo_lock_sha256: String,
    build_source_closure: &'static str,
    interpretation_boundary: &'static str,
    controls: Controls,
}

#[derive(Serialize)]
struct Controls {
    projected_tangent_max_residual: f64,
    pure_orbit_max_projection_residual: f64,
    synthetic_transverse_min_survival: f64,
    identity_max_distance: f64,
    finite_difference_max_scale_residual: f64,
    finite_difference_max_translation_residual: f64,
    finite_difference_max_sp4_residual: f64,
    incidence_failure_closed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
}

fn hash_seed(master: u64, label: &str, bucket: (usize, usize), index: usize) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&master.to_le_bytes());
    data.extend_from_slice(label.as_bytes());
    data.extend_from_slice(&(bucket.0 as u64).to_le_bytes());
    data.extend_from_slice(&(bucket.1 as u64).to_le_bytes());
    data.extend_from_slice(&(index as u64).to_le_bytes());
    *blake3::hash(&data).as_bytes()
}

fn active(f: &Factor) -> bool {
    for i in 0..f.n.len() {
        let j = (i + 1) % f.n.len();
        let a = f.n[i];
        let b = f.n[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.h[i] * b[1] - f.h[j] * a[1]) / det;
        let y = (a[0] * f.h[j] - b[0] * f.h[i]) / det;
        if f.n
            .iter()
            .zip(&f.h)
            .any(|(n, h)| n[0] * x + n[1] * y > *h + 1e-9)
        {
            return false;
        }
    }
    true
}

fn make_fixture(master: u64, bucket: (usize, usize)) -> Option<(SysLandscapePolytopeCache, usize)> {
    for attempt in 0..128 {
        let mut rng = ChaCha8Rng::from_seed(hash_seed(master, "base", bucket, attempt));
        let (qn, qh) = random_polygon_2d(bucket.0, 0.8, 1.2, &mut rng);
        let (pn, ph) = random_polygon_2d(bucket.1, 0.8, 1.2, &mut rng);
        let q = Factor { n: qn, h: qh };
        let p = Factor { n: pn, h: ph };
        if active(&q) && active(&p) {
            if let Some(cache) =
                SysLandscapePolytopeCache::from_lagrangian_product(&q.n, &q.h, &p.n, &p.h)
            {
                return Some((cache, attempt));
            }
        }
    }
    None
}

fn j() -> Matrix4<f64> {
    Matrix4::new(
        0., 0., 1., 0., 0., 0., 0., 1., -1., 0., 0., 0., 0., -1., 0., 0.,
    )
}

fn sp4_basis() -> Vec<Matrix4<f64>> {
    let mut result = Vec::with_capacity(10);
    for i in 0..2 {
        for k in 0..2 {
            let mut x = Matrix4::zeros();
            x[(i, k)] = 1.;
            x[(2 + k, 2 + i)] = -1.;
            result.push(x);
        }
    }
    for i in 0..2 {
        for k in i..2 {
            let mut x = Matrix4::zeros();
            x[(i, 2 + k)] = 1.;
            x[(k, 2 + i)] = 1.;
            result.push(x);
        }
    }
    for i in 0..2 {
        for k in i..2 {
            let mut x = Matrix4::zeros();
            x[(2 + i, k)] = 1.;
            x[(2 + k, i)] = 1.;
            result.push(x);
        }
    }
    result
}

fn flatten(vectors: &[Vector4<f64>]) -> DVector<f64> {
    DVector::from_iterator(
        vectors.len() * 4,
        vectors.iter().flat_map(|v| v.iter().copied()),
    )
}

fn unflatten(vector: &DVector<f64>, facets: usize) -> Vec<Vector4<f64>> {
    (0..facets)
        .map(|i| {
            Vector4::new(
                vector[4 * i],
                vector[4 * i + 1],
                vector[4 * i + 2],
                vector[4 * i + 3],
            )
        })
        .collect()
}

fn tangent_columns(duals: &[Vector4<f64>]) -> DMatrix<f64> {
    let facets = duals.len();
    let columns = 15;
    let mut data = vec![0.0; facets * 4 * columns];
    let mut put = |column: usize, i: usize, value: f64| data[i * columns + column] = value;
    for (i, a) in duals.iter().enumerate() {
        let base = 4 * i;
        for c in 0..4 {
            put(0, base + c, -a[c]);
        }
        for t in 0..4 {
            for c in 0..4 {
                put(1 + t, base + c, -a[t] * a[c]);
            }
        }
        for (offset, x) in sp4_basis().iter().enumerate() {
            let da = -x.transpose() * a;
            for c in 0..4 {
                put(5 + offset, base + c, da[c]);
            }
        }
    }
    DMatrix::from_row_slice(facets * 4, columns, &data)
}

fn tangent_span(duals: &[Vector4<f64>]) -> TangentSpan {
    let columns = tangent_columns(duals);
    let svd = columns.clone().svd(true, false);
    let singular_values = svd.singular_values.as_slice().to_vec();
    let max_sv = singular_values.iter().copied().fold(0.0, f64::max);
    let rank = singular_values
        .iter()
        .take(columns.ncols())
        .filter(|&&x| x > max_sv * 1e-10)
        .count();
    let basis = svd
        .u
        .expect("requested SVD left singular vectors")
        .columns(0, rank)
        .into_owned();
    TangentSpan {
        basis,
        rank,
        singular_values,
    }
}

fn project(span: &TangentSpan, value: &DVector<f64>) -> DVector<f64> {
    value - &span.basis * (&span.basis.transpose() * value)
}

fn tangent_component(span: &TangentSpan, value: &DVector<f64>) -> DVector<f64> {
    &span.basis * (&span.basis.transpose() * value)
}

fn incidence(cache: &SysLandscapePolytopeCache) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = (0..cache.vertex_facet_incidence.nrows())
        .map(|v| {
            (0..cache.vertex_facet_incidence.ncols())
                .filter(|&f| cache.vertex_facet_incidence[(v, f)])
                .collect()
        })
        .collect();
    rows.sort();
    rows
}

fn affine_rank4(cache: &SysLandscapePolytopeCache) -> bool {
    if cache.vertices_f64.len() < 5 {
        return false;
    }
    let origin = cache.vertices_f64[0];
    let matrix = DMatrix::from_fn(cache.vertices_f64.len() - 1, 4, |r, c| {
        cache.vertices_f64[r + 1][c] - origin[c]
    });
    matrix
        .svd(false, false)
        .singular_values
        .iter()
        .filter(|&&s| s > 1e-10)
        .count()
        == 4
}

fn check(base: &SysLandscapePolytopeCache, duals: Vec<Vector4<f64>>) -> Check {
    let Some(cache) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
        return Check {
            cache: None,
            same_incidence: false,
            full_dimensional: false,
            bounded: false,
            irredundant: false,
            volume: None,
            failures: vec!["exact_reconstruction_or_basic_validation_failed".into()],
        };
    };
    let same = incidence(&cache) == incidence(base);
    let full = affine_rank4(&cache);
    let bounded = cache.facet_intersection_is_nonempty.nrows() == cache.facet_count()
        && (0..cache.facet_count()).all(|i| {
            (0..cache.facet_count()).any(|j| cache.facet_intersection_is_nonempty[(i, j)])
        });
    let irredundant = cache.dual_vertices.iter().enumerate().all(|(facet, _)| {
        (0..cache.vertex_facet_incidence.nrows()).any(|v| cache.vertex_facet_incidence[(v, facet)])
    });
    let volume = Some(exact_volume_from_incidence_as_f64(
        &cache.vertices,
        &cache.vertex_facet_incidence,
    ));
    let mut failures = Vec::new();
    if !same {
        failures.push("source_incidence_changed".into());
    }
    if !full {
        failures.push("not_full_dimensional".into());
    }
    if !bounded {
        failures.push("boundedness_check_failed".into());
    }
    if !irredundant {
        failures.push("irredundancy_check_failed".into());
    }
    Check {
        cache: Some(cache),
        same_incidence: same,
        full_dimensional: full,
        bounded,
        irredundant,
        volume,
        failures,
    }
}

/// Cheap f64 incidence oracle used only to bracket the exact boundary search.
/// Every retained row still crosses `check`, which rationalizes and rebuilds
/// the exact polar. This avoids exact-polar work on interior bisection points.
fn f64_incidence(duals: &[Vector4<f64>]) -> Option<Vec<Vec<usize>>> {
    if duals.len() < 5 || duals.iter().any(|a| !a.iter().all(|x| x.is_finite())) {
        return None;
    }
    let mut rows = Vec::new();
    let rhs = Vector4::repeat(1.0);
    for i in 0..duals.len() {
        for k in i + 1..duals.len() {
            for l in k + 1..duals.len() {
                for m in l + 1..duals.len() {
                    let matrix = Matrix4::from_rows(&[
                        duals[i].transpose(),
                        duals[k].transpose(),
                        duals[l].transpose(),
                        duals[m].transpose(),
                    ]);
                    let Some(vertex) = matrix.try_inverse().map(|inverse| inverse * rhs) else {
                        continue;
                    };
                    if duals.iter().any(|a| a.dot(&vertex) > 1.0 + 1e-8) {
                        continue;
                    }
                    let row: Vec<usize> = duals
                        .iter()
                        .enumerate()
                        .filter(|(_, a)| (a.dot(&vertex) - 1.0).abs() < 1e-7)
                        .map(|(index, _)| index)
                        .collect();
                    if row.len() >= 4 && !rows.contains(&row) {
                        rows.push(row);
                    }
                }
            }
        }
    }
    rows.sort();
    (!rows.is_empty()).then_some(rows)
}

fn f64_same_incidence(base: &SysLandscapePolytopeCache, duals: &[Vector4<f64>]) -> bool {
    f64_incidence(duals).is_some_and(|rows| rows == incidence(base))
}

fn feature_distance(base: &[Vector4<f64>], other: &[Vector4<f64>], symplectic: bool) -> f64 {
    let mut sum = 0.;
    for i in 0..base.len() {
        for k in i..base.len() {
            let x = if symplectic {
                omega0(&base[i], &base[k])
            } else {
                base[i].dot(&base[k])
            };
            let y = if symplectic {
                omega0(&other[i], &other[k])
            } else {
                other[i].dot(&other[k])
            };
            sum += (x - y).powi(2);
        }
    }
    sum.sqrt()
}

fn product_block_residual(duals: &[Vector4<f64>], q_facets: usize) -> f64 {
    let mut mixed = 0.;
    let mut total = 0.;
    for (i, a) in duals.iter().enumerate() {
        let mixed_part = if i < q_facets {
            Vector2::new(a[2], a[3]).norm()
        } else {
            Vector2::new(a[0], a[1]).norm()
        };
        mixed += mixed_part.powi(2);
        total += a.norm_squared();
    }
    if total == 0. {
        0.
    } else {
        mixed.sqrt() / total.sqrt()
    }
}

fn finite_difference_residuals(duals: &[Vector4<f64>]) -> (f64, f64, f64) {
    let h = 1e-6;
    let scale_plus: Vec<_> = duals.iter().map(|a| a / (1. + h)).collect();
    let scale_minus: Vec<_> = duals.iter().map(|a| a / (1. - h)).collect();
    let scale_fd: Vec<_> = scale_plus
        .iter()
        .zip(&scale_minus)
        .map(|(p, m)| (p - m) / (2. * h))
        .collect();
    let scale_err = scale_fd
        .iter()
        .zip(duals)
        .map(|(x, a)| (x + a).norm())
        .fold(0., f64::max);

    let t = Vector4::new(0.31, -0.17, 0.23, 0.11);
    let translate = |eps: f64| {
        duals
            .iter()
            .map(|a| a / (1. + eps * a.dot(&t)))
            .collect::<Vec<_>>()
    };
    let tp = translate(h);
    let tm = translate(-h);
    let tfd: Vec<_> = tp
        .iter()
        .zip(&tm)
        .map(|(p, m)| (p - m) / (2. * h))
        .collect();
    let terr = tfd
        .iter()
        .zip(duals)
        .map(|(x, a)| (x + a.dot(&t) * a).norm())
        .fold(0., f64::max);

    let mut serr: f64 = 0.;
    for x in sp4_basis() {
        let plus = (x * h).exp();
        let minus = (x * -h).exp();
        let pmap = plus.try_inverse().expect("exp invertible").transpose();
        let mmap = minus.try_inverse().expect("exp invertible").transpose();
        for a in duals {
            let fd = (pmap * a - mmap * a) / (2. * h);
            serr = serr.max((fd + x.transpose() * a).norm());
        }
    }
    (scale_err, terr, serr)
}

fn duals_at(
    base: &SysLandscapePolytopeCache,
    direction: &DVector<f64>,
    epsilon: f64,
    scale: f64,
) -> Vec<Vector4<f64>> {
    let original = flatten(&base.dual_vertices_f64);
    unflatten(
        &(original + direction * (epsilon * scale)),
        base.facet_count(),
    )
}

fn find_boundary(
    base: &SysLandscapePolytopeCache,
    direction: &DVector<f64>,
    scale: f64,
) -> (f64, bool) {
    let mut low = 0.;
    let mut high = 1e-6;
    while high <= 4096. {
        if !f64_same_incidence(base, &duals_at(base, direction, high, scale)) {
            let mut exact_high = high;
            let mut exact_failure = false;
            for _ in 0..4 {
                let c = check(base, duals_at(base, direction, exact_high, scale));
                if c.cache.is_none()
                    || !c.same_incidence
                    || !c.full_dimensional
                    || !c.bounded
                    || !c.irredundant
                {
                    exact_failure = true;
                    break;
                }
                low = exact_high;
                exact_high *= 2.;
            }
            if !exact_failure {
                return (low, true);
            }
            high = exact_high;
            for _ in 0..12 {
                let mid = 0.5 * (low + high);
                let cm = check(base, duals_at(base, direction, mid, scale));
                if cm.cache.is_some()
                    && cm.same_incidence
                    && cm.full_dimensional
                    && cm.bounded
                    && cm.irredundant
                {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            return (high, false);
        }
        low = high;
        high *= 2.;
    }
    (low, true)
}

fn direction(
    seed: [u8; 32],
    span: &TangentSpan,
    dimension: usize,
) -> (DVector<f64>, DVector<f64>, f64, f64) {
    let mut rng = ChaCha8Rng::from_seed(seed);
    let raw = DVector::<f64>::from_iterator(
        dimension,
        (0..dimension).map(|_| StandardNormal.sample(&mut rng)),
    );
    let raw = &raw / raw.norm();
    let projected = project(span, &raw);
    let projected = &projected / projected.norm();
    let pure_coeff = DVector::from_iterator(
        span.rank,
        (0..span.rank).map(|_| StandardNormal.sample(&mut rng)),
    );
    let pure = &span.basis * pure_coeff;
    let pure_residual = project(span, &pure).norm();
    let synthetic_survival = project(span, &raw).norm();
    (raw, projected, pure_residual, synthetic_survival)
}

fn sha256(path: &str) -> String {
    Command::new("sha256sum")
        .arg(path)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_else(|| "unavailable".into())
}

fn provenance() -> (String, String, bool, String, String) {
    let output = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
            .unwrap_or_else(|| "unknown".into())
    };
    let status = output(&["status", "--porcelain", "--untracked-files=no"]);
    (
        output(&["rev-parse", "HEAD"]),
        output(&["rev-parse", "HEAD^{tree}"]),
        !status.is_empty(),
        sha256("experiments/sys-datascience/methods/transverse-product-perturbation/main.rs"),
        sha256("Cargo.lock"),
    )
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/transverse-product-perturbation/artifacts/smoke",
        ),
        seed: 20260715,
    };
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out-dir" => {
                let value = argv.get(i + 1).ok_or("--out-dir requires a value")?;
                args.out_dir = PathBuf::from(value);
                i += 2;
            }
            "--seed" => {
                let value = argv.get(i + 1).ok_or("--seed requires a value")?;
                args.seed = value.parse().map_err(|_| "--seed must be a u64")?;
                i += 2;
            }
            "--help" | "-h" => return Err("usage: --out-dir DIR --seed U64".into()),
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(args)
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let args = parse_args(&argv).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2)
    });
    let (revision, tree, dirty, source_sha256, lock_sha256) = provenance();
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("rows.jsonl");
    let report_path = args.out_dir.join("report.json");
    let mut rows_out = BufWriter::new(File::create(&rows_path).expect("create rows"));
    let mut rows = Vec::new();
    let mut fixture_counts = BTreeMap::new();
    let mut tangent_ranks = BTreeMap::new();
    let mut tangent_min_sv = BTreeMap::new();
    let mut control_tangent: f64 = 0.;
    let mut control_pure: f64 = 0.;
    let mut control_synthetic = f64::INFINITY;
    let mut control_identity: f64 = 0.;
    let mut fd_scale: f64 = 0.;
    let mut fd_translate: f64 = 0.;
    let mut fd_sp: f64 = 0.;
    let mut censored = 0;
    for &bucket in BUCKETS {
        let Some((base, base_attempt)) = make_fixture(args.seed, bucket) else {
            continue;
        };
        let tangent = tangent_span(&base.dual_vertices_f64);
        let fixture = Fixture {
            bucket,
            base_attempt,
            base,
            tangent,
        };
        let name = format!("{}x{}", bucket.0, bucket.1);
        fixture_counts.insert(name.clone(), 1);
        tangent_ranks.insert(name.clone(), fixture.tangent.rank);
        let min_sv = fixture
            .tangent
            .singular_values
            .iter()
            .copied()
            .filter(|x| *x > 0.)
            .fold(f64::INFINITY, f64::min);
        tangent_min_sv.insert(name, min_sv);
        let scale = flatten(&fixture.base.dual_vertices_f64).norm();
        let (fd1, fd2, fd3) = finite_difference_residuals(&fixture.base.dual_vertices_f64);
        fd_scale = fd_scale.max(fd1);
        fd_translate = fd_translate.max(fd2);
        fd_sp = fd_sp.max(fd3);
        for direction_index in 0..DIRECTIONS_PER_FIXTURE {
            let (raw, projected, pure_residual, synthetic) = direction(
                hash_seed(args.seed, "direction", bucket, direction_index),
                &fixture.tangent,
                fixture.base.facet_count() * 4,
            );
            control_tangent =
                control_tangent.max(tangent_component(&fixture.tangent, &projected).norm());
            control_pure = control_pure.max(pure_residual);
            control_synthetic = control_synthetic.min(synthetic);
            let (boundary, is_censored) = find_boundary(&fixture.base, &projected, scale);
            if is_censored {
                censored += FRACTIONS.len();
            }
            for &fraction in FRACTIONS {
                let epsilon = boundary * fraction;
                let response_duals = duals_at(&fixture.base, &projected, epsilon, scale);
                let response = check(&fixture.base, response_duals.clone());
                let direct_distance = (epsilon * scale * &projected).norm();
                let orbit_distance = direct_distance;
                let mut failures = response.failures.clone();
                let base_volume = exact_volume_from_incidence_as_f64(
                    &fixture.base.vertices,
                    &fixture.base.vertex_facet_incidence,
                );
                let (
                    response_volume,
                    volume_ratio,
                    euclidean_feature,
                    symplectic_feature,
                    block_residual,
                ) = if let Some(cache) = &response.cache {
                    let v = response.volume;
                    let ratio = v.map(|x| x / base_volume);
                    (
                        v,
                        ratio,
                        Some(feature_distance(
                            &fixture.base.dual_vertices_f64,
                            &cache.dual_vertices_f64,
                            false,
                        )),
                        Some(feature_distance(
                            &fixture.base.dual_vertices_f64,
                            &cache.dual_vertices_f64,
                            true,
                        )),
                        Some(product_block_residual(&cache.dual_vertices_f64, bucket.0)),
                    )
                } else {
                    (None, None, None, None, None)
                };
                let mut normalized_reconstruction = false;
                let mut normalized_ratio = None;
                if let (Some(v), Some(cache)) = (response_volume, response.cache.as_ref()) {
                    if v > 0. && base_volume > 0. {
                        let factor = (v / base_volume).powf(0.25);
                        let normalized =
                            cache.dual_vertices_f64.iter().map(|a| a * factor).collect();
                        let nc = check(&fixture.base, normalized);
                        normalized_reconstruction = nc.cache.is_some()
                            && nc.same_incidence
                            && nc.full_dimensional
                            && nc.bounded
                            && nc.irredundant;
                        normalized_ratio = nc.volume.map(|x| x / base_volume);
                        if !normalized_reconstruction {
                            failures.push("volume_normalization_reconstruction_failed".into());
                        }
                    }
                }
                if fraction == 0. {
                    control_identity = control_identity.max(direct_distance);
                }
                let exact_status = if response.cache.is_some() && failures.is_empty() {
                    "reconstructed_same_incidence"
                } else if response.cache.is_some() {
                    "reconstructed_contract_failure"
                } else {
                    "rejected"
                };
                let row = Row {
                    schema: "transverse-product-perturbation-row-v1",
                    id: format!("transverse-product-v1/seed={}/bucket={}x{}/direction={}/fraction={fraction}", args.seed, bucket.0, bucket.1, direction_index),
                    seed: args.seed,
                    bucket: format!("{}x{}", bucket.0, bucket.1),
                    direction: direction_index,
                    fraction,
                    epsilon,
                    boundary_epsilon: boundary,
                    boundary_censored: is_censored,
                    coordinate_order: COORDINATE_ORDER,
                    raw_direction: raw.iter().copied().collect(),
                    projected_direction: projected.iter().copied().collect(),
                    raw_direct_distance: direct_distance,
                    sys_orbit_section_distance: orbit_distance,
                    tangent_residual: tangent_component(&fixture.tangent, &projected).norm(),
                    pure_orbit_projection_residual: pure_residual,
                    synthetic_transverse_survival: synthetic,
                    finite_difference_scale_residual: fd1,
                    finite_difference_translation_residual: fd2,
                    finite_difference_sp4_residual: fd3,
                    exact_reconstruction_status: exact_status,
                    same_face_lattice: response.same_incidence,
                    same_source_incidence: response.same_incidence,
                    full_dimensional: response.full_dimensional,
                    bounded: response.bounded,
                    irredundant: response.irredundant,
                    base_volume,
                    response_volume,
                    volume_ratio,
                    volume_normalized_reconstruction: normalized_reconstruction,
                    normalized_volume_ratio: normalized_ratio,
                    euclidean_feature_distance: euclidean_feature,
                    symplectic_feature_distance: symplectic_feature,
                    product_block_residual: block_residual,
                    combinatorial_product_preserved: response.same_incidence,
                    failures,
                };
                serde_json::to_writer(&mut rows_out, &row).expect("serialize row");
                writeln!(&mut rows_out).expect("write row");
                rows.push(row);
            }
        }
    }
    rows_out.flush().expect("flush rows");
    let report = Report {
        schema: "transverse-product-perturbation-report-v1",
        command: argv.join(" "),
        seed: args.seed,
        fixtures: fixture_counts.len(),
        directions_per_fixture: DIRECTIONS_PER_FIXTURE,
        fractions: FRACTIONS.to_vec(),
        rows: rows.len(),
        passed: rows.iter().filter(|r| r.failures.is_empty()).count(),
        failed: rows.iter().filter(|r| !r.failures.is_empty()).count(),
        retained_fixture_counts: fixture_counts,
        tangent_ranks,
        tangent_min_singular_values: tangent_min_sv,
        boundary_censored_rows: censored,
        source_revision: revision,
        source_repository_tree: tree,
        source_dirty: dirty,
        producer_source_sha256: source_sha256,
        cargo_lock_sha256: lock_sha256,
        build_source_closure: "Pinned full-repository revision/tree and tracked-clean predicate bind transitive path dependencies; file hashes are convenience checks.",
        interpretation_boundary: "Target-free local geometry only. A same-incidence row is not a capacity/sys evaluation, population statement, canonical quotient metric, or proof of a product law.",
        controls: Controls {
            projected_tangent_max_residual: control_tangent,
            pure_orbit_max_projection_residual: control_pure,
            synthetic_transverse_min_survival: control_synthetic,
            identity_max_distance: control_identity,
            finite_difference_max_scale_residual: fd_scale,
            finite_difference_max_translation_residual: fd_translate,
            finite_difference_max_sp4_residual: fd_sp,
            incidence_failure_closed: rows.iter().any(|r| r.boundary_epsilon.is_finite() && !r.boundary_censored),
        },
    };
    let mut report_out = BufWriter::new(File::create(report_path).expect("create report"));
    serde_json::to_writer_pretty(&mut report_out, &report).expect("serialize report");
    writeln!(&mut report_out).expect("write report");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tangent_basis_has_expected_rank_on_small_product() {
        let (base, _) = make_fixture(20260715, (3, 3)).expect("fixture");
        assert_eq!(tangent_span(&base.dual_vertices_f64).rank, 15);
    }

    #[test]
    fn projected_direction_collapses_tangent_component() {
        let (base, _) = make_fixture(20260715, (3, 3)).expect("fixture");
        let span = tangent_span(&base.dual_vertices_f64);
        let v = DVector::from_element(base.facet_count() * 4, 1.0);
        assert!(tangent_component(&span, &project(&span, &v)).norm() < 1e-10);
    }

    #[test]
    fn incidence_failure_is_fail_closed() {
        let (base, _) = make_fixture(20260715, (3, 3)).expect("fixture");
        let mut duals = base.dual_vertices_f64.clone();
        duals[0] *= 1e-12;
        let result = check(&base, duals);
        assert!(result.cache.is_none() || !result.same_incidence || !result.full_dimensional);
    }
}
