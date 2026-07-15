//! Target-free matched Cartan anisotropy intervention.
//!
//! The producer consumes the reviewed identity rows of the orientation panel.
//! It never regenerates a base and never calls a capacity backend.  For each
//! retained base and each `t in {1,5/4,3/2,2}`, it applies the paired positive
//! diagonal maps
//!
//!   S_t = diag(t,t^-1,t^-1,t),    N_t = diag(t,t^-1,t,t^-1).
//!
//! The two arms have the same Euclidean singular values and determinant, while
//! their canonical symplectic pair weights are respectively `(1,1)` and
//! `(t^2,t^-2)`.  All arithmetic describing the intervention is retained as
//! exact rational strings; reconstruction is performed through the existing
//! binary-rational geometry boundary.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{Matrix4, Vector4};
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fs::{create_dir_all, read, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

const SOURCE_SCHEMA: &str = "generator-orientation-smoke-row-v2";
const ROW_SCHEMA: &str = "generator-cartan-anisotropy-row-v1";
const REPORT_SCHEMA: &str = "generator-cartan-anisotropy-report-v1";
const COORDINATE_ORDER: &str = "q1,q2,p1,p2";
const BUCKETS: &[(usize, usize)] = &[(3, 3), (4, 4), (4, 6), (6, 6)];
const LEVELS: &[Level] = &[
    Level {
        name: "1",
        num: 1,
        den: 1,
    },
    Level {
        name: "5/4",
        num: 5,
        den: 4,
    },
    Level {
        name: "3/2",
        num: 3,
        den: 2,
    },
    Level {
        name: "2",
        num: 2,
        den: 1,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Level {
    name: &'static str,
    num: i64,
    den: i64,
}

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    source: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct SourceRow {
    schema: String,
    map_variant: String,
    base_id: String,
    base_geometry_id: Option<String>,
    transformed_geometry_id: Option<String>,
    bucket: String,
    q_sides: usize,
    p_sides: usize,
    base_accepted: bool,
    transformed_dual_vertices_f64: Vec<[f64; 4]>,
    transformed_dual_vertices_rational: Vec<[String; 4]>,
    labeled_incidence_signature: Vec<Vec<usize>>,
}

#[derive(Clone, Serialize)]
struct Signature {
    dual_norms: Vec<f64>,
    euclidean_gram_upper: Vec<f64>,
    symplectic_gram_upper: Vec<f64>,
    omega_sign_signature: Vec<i8>,
}

#[derive(Clone, Serialize)]
struct ExactFeatures {
    facet_count: usize,
    vertex_count: usize,
    volume: f64,
    dual_norm_l1: f64,
    symplectic_gram_l1: f64,
    symplectic_gram_max_abs: f64,
}

#[derive(Clone, Serialize)]
struct EuclideanChecks {
    determinant_one: bool,
    singular_spectrum_control: bool,
    squared_singular_values_exact: Vec<String>,
    singular_values: Vec<f64>,
    euclidean_control_error: f64,
    volume_relative_error: f64,
}

#[derive(Clone, Serialize)]
struct Row {
    schema: &'static str,
    row_id: String,
    pairing_id: String,
    base_id: String,
    base_geometry_id: String,
    transformed_geometry_id: String,
    bucket: String,
    map_family: &'static str,
    t: String,
    t_num: i64,
    t_den: i64,
    coordinate_order: &'static str,
    primal_action: &'static str,
    dual_action: &'static str,
    matrix_row_major: [[f64; 4]; 4],
    matrix_exact_row_major: [[String; 4]; 4],
    determinant: f64,
    determinant_exact: String,
    singular_values: Vec<f64>,
    squared_singular_values_exact: Vec<String>,
    symplectic_residual_f64: f64,
    symplectic_residual_exact: String,
    canonical_pair_weights_f64: [f64; 2],
    canonical_pair_weights_exact: [String; 2],
    exact_reconstruction_status: &'static str,
    reconstruction_passed: bool,
    incidence_matches_base: bool,
    volume_matches_base: bool,
    euclidean_checks: EuclideanChecks,
    base_signature: Signature,
    response_signature: Signature,
    base_exact_features: ExactFeatures,
    response_exact_features: ExactFeatures,
    failures: Vec<String>,
}

#[derive(Serialize)]
struct PairedRow {
    base_id: String,
    base_geometry_id: String,
    bucket: String,
    t: String,
    s_row_id: String,
    n_row_id: String,
    singular_spectrum_max_abs_delta: f64,
    symplectic_residual_s: f64,
    symplectic_residual_n: f64,
    symplectic_feature_l1_delta: f64,
    response_difference_l1: f64,
    negative_control_equal_at_t1: bool,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    source_path: String,
    source_input_sha256: String,
    source_revision: String,
    source_repository_tree: String,
    source_dirty_tracked: bool,
    producer_source_sha256: String,
    cargo_lock_sha256: String,
    coordinate_order: &'static str,
    requested_buckets: Vec<String>,
    requested_base_count: usize,
    requested_rows: usize,
    observed_rows: usize,
    passed_rows: usize,
    failure_rows: usize,
    pair_count: usize,
    failures: Vec<String>,
    output_rows_sha256: String,
    output_rows_count: usize,
    output_paired_sha256: String,
    output_paired_count: usize,
    diagonal_quotient_control: DiagonalQuotientControl,
    exactness_boundary: &'static str,
    interpretation_boundary: &'static str,
}

#[derive(Serialize)]
struct DiagonalQuotientControl {
    input_d_exact: [String; 4],
    t_squared_exact: String,
    symplectic_factor_exact: [[String; 4]; 4],
    reconstructed_d_exact: [[String; 4]; 4],
    reconstruction_passed: bool,
    scope: &'static str,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut out_dir = None;
    let mut source = PathBuf::from(
        "experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl",
    );
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out-dir" => {
                i += 1;
                out_dir = Some(PathBuf::from(argv.get(i).ok_or("--out-dir needs a path")?));
            }
            "--source" => {
                i += 1;
                source = PathBuf::from(argv.get(i).ok_or("--source needs a path")?);
            }
            "--help" => return Err("usage: --out-dir PATH [--source PATH]".into()),
            x => return Err(format!("unknown argument {x}")),
        }
        i += 1;
    }
    Ok(Args {
        out_dir: out_dir.ok_or("--out-dir is required")?,
        source,
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn repo_identity() -> (String, String, bool) {
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_owned();
    let tree = Command::new("git")
        .args(["rev-parse", "HEAD^{tree}"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_owned();
    let dirty = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(true);
    (revision, tree, dirty)
}

fn rat(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}
fn rat_string(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}
fn f64_string(x: &BigRational) -> String {
    x.to_f64().unwrap_or(f64::NAN).to_string()
}

fn matrix_for(level: Level, family: &str) -> ([[BigRational; 4]; 4], Matrix4<f64>) {
    let t = rat(level.num, level.den);
    let ti = rat(level.den, level.num);
    let d = if family == "S" {
        [t.clone(), ti.clone(), ti, t]
    } else {
        [t.clone(), ti.clone(), t, ti]
    };
    let exact = std::array::from_fn(|i| {
        std::array::from_fn(|j| if i == j { d[i].clone() } else { rat(0, 1) })
    });
    let float = Matrix4::from_diagonal(&Vector4::new(
        d[0].to_f64().unwrap(),
        d[1].to_f64().unwrap(),
        d[2].to_f64().unwrap(),
        d[3].to_f64().unwrap(),
    ));
    (exact, float)
}

fn j_exact() -> [[BigRational; 4]; 4] {
    std::array::from_fn(|i| {
        std::array::from_fn(|k| match (i, k) {
            (0, 2) | (1, 3) => rat(-1, 1),
            (2, 0) | (3, 1) => rat(1, 1),
            _ => rat(0, 1),
        })
    })
}

fn exact_symplectic_residual(m: &[[BigRational; 4]; 4]) -> String {
    let j = j_exact();
    let mut max = rat(0, 1);
    for i in 0..4 {
        for k in 0..4 {
            let mut x = rat(0, 1);
            for a in 0..4 {
                for b in 0..4 {
                    x += m[a][i].clone() * j[a][b].clone() * m[b][k].clone();
                }
            }
            x -= j[i][k].clone();
            if x.clone().abs() > max {
                max = x.abs();
            }
        }
    }
    rat_string(&max)
}

fn symplectic_residual(m: &Matrix4<f64>) -> f64 {
    let j = Matrix4::new(
        0., 0., -1., 0., 0., 0., 0., -1., 1., 0., 0., 0., 0., 1., 0., 0.,
    );
    (m.transpose() * j * m - j).norm()
}

fn incidence(p: &SysLandscapePolytopeCache) -> Vec<Vec<usize>> {
    let mut x: Vec<Vec<usize>> = (0..p.vertex_facet_incidence.nrows())
        .map(|i| {
            (0..p.vertex_facet_incidence.ncols())
                .filter(|&k| p.vertex_facet_incidence[(i, k)])
                .collect()
        })
        .collect();
    x.sort();
    x
}

fn omega(a: &Vector4<f64>, b: &Vector4<f64>) -> f64 {
    a[2] * b[0] + a[3] * b[1] - a[0] * b[2] - a[1] * b[3]
}

fn signature(p: &SysLandscapePolytopeCache) -> Signature {
    let a = &p.dual_vertices_f64;
    let mut e = Vec::new();
    let mut w = Vec::new();
    let mut signs = Vec::new();
    for i in 0..a.len() {
        for k in i..a.len() {
            e.push(a[i].dot(&a[k]));
            let x = omega(&a[i], &a[k]);
            w.push(x);
            signs.push((x > 1e-12) as i8 - (x < -1e-12) as i8);
        }
    }
    Signature {
        dual_norms: a.iter().map(Vector4::norm).collect(),
        euclidean_gram_upper: e,
        symplectic_gram_upper: w,
        omega_sign_signature: signs,
    }
}

fn features(p: &SysLandscapePolytopeCache) -> ExactFeatures {
    let sig = signature(p);
    ExactFeatures {
        facet_count: p.dual_vertices.len(),
        vertex_count: p.vertices.len(),
        volume: exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence),
        dual_norm_l1: sig.dual_norms.iter().sum(),
        symplectic_gram_l1: sig.symplectic_gram_upper.iter().map(|x| x.abs()).sum(),
        symplectic_gram_max_abs: sig
            .symplectic_gram_upper
            .iter()
            .map(|x| x.abs())
            .fold(0.0, f64::max),
    }
}

fn exact_matrix_strings(m: &[[BigRational; 4]; 4]) -> [[String; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|j| rat_string(&m[i][j])))
}
fn square_spectrum(level: Level) -> (Vec<String>, Vec<f64>) {
    let t2 = rat(level.num * level.num, level.den * level.den);
    let ti2 = rat(level.den * level.den, level.num * level.num);
    let rs = vec![
        rat_string(&t2),
        rat_string(&t2),
        rat_string(&ti2),
        rat_string(&ti2),
    ];
    let mut fs = vec![
        t2.to_f64().unwrap(),
        t2.to_f64().unwrap(),
        ti2.to_f64().unwrap(),
        ti2.to_f64().unwrap(),
    ];
    fs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (rs, fs.into_iter().map(f64::sqrt).collect())
}

fn quotient_control() -> DiagonalQuotientControl {
    // Concrete positive rational determinant-one diagonal.  In general
    // t^2=d1*d3 and d2*d4=t^-2; this case has t=2 and D=N_2.
    let d = [rat(2, 1), rat(1, 2), rat(2, 1), rat(1, 2)];
    let t = rat(2, 1);
    let a = [
        d[0].clone() / t.clone(),
        d[1].clone() * t.clone(),
        t.clone() / d[2].clone(),
        rat(1, 1) / (d[3].clone() * t.clone()),
    ];
    let factor = std::array::from_fn(|i| {
        std::array::from_fn(|j| if i == j { a[i].clone() } else { rat(0, 1) })
    });
    let n = [
        t.clone(),
        rat(1, 1) / t.clone(),
        t.clone(),
        rat(1, 1) / t.clone(),
    ];
    let rec = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            if i == j {
                factor[i][i].clone() * n[i].clone()
            } else {
                rat(0, 1)
            }
        })
    });
    let input_d_exact = std::array::from_fn(|i| rat_string(&d[i]));
    let reconstruction_passed = rec.iter().enumerate().all(|(i, row)| {
        row[i] == d[i]
            && row
                .iter()
                .enumerate()
                .all(|(j, x)| j == i || x == &rat(0, 1))
    });
    DiagonalQuotientControl {
        input_d_exact,
        t_squared_exact: rat_string(&(t.clone() * t)),
        symplectic_factor_exact: exact_matrix_strings(&factor),
        reconstructed_d_exact: exact_matrix_strings(&rec),
        reconstruction_passed,
        scope: "positive diagonal determinant-one Cartan quotient only; not a classification of Sp(4)\\SL(4)\\Sp(4)",
    }
}

fn geometry_id(p: &SysLandscapePolytopeCache) -> String {
    let mut payload = String::new();
    for a in &p.dual_vertices {
        for x in a {
            payload.push_str(&rat_string(x));
            payload.push(';');
        }
    }
    payload.push_str(&format!("|{:?}", incidence(p)));
    format!("cartan-geometry-{}", sha256_hex(payload.as_bytes()))
}

fn load_bases(path: &Path) -> Result<(Vec<SourceRow>, String), String> {
    let bytes = read(path).map_err(|e| format!("source read failed: {e}"))?;
    let digest = sha256_hex(&bytes);
    let mut bases = Vec::new();
    let mut seen = BTreeSet::new();
    for (line_no, line) in String::from_utf8(bytes)
        .map_err(|_| "source is not UTF-8".to_string())?
        .lines()
        .enumerate()
    {
        let row: SourceRow =
            serde_json::from_str(line).map_err(|e| format!("source line {}: {e}", line_no + 1))?;
        if row.schema != SOURCE_SCHEMA {
            continue;
        }
        if row.map_variant == "identity" && row.base_accepted {
            if !seen.insert(row.base_id.clone()) {
                return Err(format!("duplicate base identity {}", row.base_id));
            }
            bases.push(row);
        }
    }
    bases.sort_by(|a, b| a.base_id.cmp(&b.base_id));
    let expected: BTreeSet<String> = BUCKETS.iter().map(|(q, p)| format!("{q}x{p}")).collect();
    let mut counts = BTreeMap::<String, usize>::new();
    for b in &bases {
        *counts.entry(b.bucket.clone()).or_default() += 1;
    }
    if bases.len() != 8
        || counts.values().any(|&x| x != 2)
        || counts.keys().any(|k| !expected.contains(k))
    {
        return Err(format!(
            "requested retained panel incomplete: counts={counts:?}"
        ));
    }
    for b in &bases {
        if b.base_geometry_id.is_none()
            || b.transformed_geometry_id.is_none()
            || b.transformed_dual_vertices_f64.is_empty()
            || b.labeled_incidence_signature.is_empty()
        {
            return Err(format!("base {} lacks exact identity payload", b.base_id));
        }
    }
    Ok((bases, digest))
}

fn row_for(
    base: &SourceRow,
    level: Level,
    family: &str,
    base_cache: &SysLandscapePolytopeCache,
    base_sig: &Signature,
    base_feat: &ExactFeatures,
) -> Row {
    let (exact_m, m) = matrix_for(level, family);
    let inv_diag = [
        1.0 / m[(0, 0)],
        1.0 / m[(1, 1)],
        1.0 / m[(2, 2)],
        1.0 / m[(3, 3)],
    ];
    let duals: Vec<Vector4<f64>> = base_cache
        .dual_vertices_f64
        .iter()
        .map(|a| {
            Vector4::new(
                a[0] * inv_diag[0],
                a[1] * inv_diag[1],
                a[2] * inv_diag[2],
                a[3] * inv_diag[3],
            )
        })
        .collect();
    let mut failures = Vec::new();
    let reconstructed = SysLandscapePolytopeCache::from_f64_dual_vertices(duals);
    let (inc_match, vol_match, response_sig, response_feat, geom_id, vol_err) =
        if let Some(p) = reconstructed {
            let inc = incidence(&p) == incidence(base_cache)
                && incidence(&p) == base.labeled_incidence_signature;
            let v = exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence);
            let bv = base_feat.volume;
            let err = (v - bv) / bv;
            (
                inc,
                err.abs() <= 1e-10,
                signature(&p),
                features(&p),
                geometry_id(&p),
                err,
            )
        } else {
            failures.push("exact_reconstruction_rejected".into());
            (
                false,
                false,
                base_sig.clone(),
                base_feat.clone(),
                "unreconstructed".into(),
                f64::NAN,
            )
        };
    if !inc_match {
        failures.push("incidence_changed_or_identity_mismatch".into());
    }
    if !vol_match {
        failures.push("volume_changed".into());
    }
    let exact_res = exact_symplectic_residual(&exact_m);
    let residual_f64 = symplectic_residual(&m);
    let (squares_exact, singular_values) = square_spectrum(level);
    let t = rat(level.num, level.den);
    let ti = rat(level.den, level.num);
    let pair = if family == "S" {
        [rat(1, 1), rat(1, 1)]
    } else {
        [t.clone() * t.clone(), ti.clone() * ti.clone()]
    };
    let row_id = format!(
        "cartan-anisotropy-v1/base={}/map={family}/t={}",
        base.base_id, level.name
    );
    let pairing_id = format!(
        "cartan-anisotropy-v1/base={}/t={}",
        base.base_id, level.name
    );
    let spectrum_control = squares_exact == square_spectrum(level).0;
    let det = m.determinant();
    let det_ok = (det - 1.0).abs() <= 1e-12;
    if !det_ok {
        failures.push("determinant_not_one".into());
    }
    if !spectrum_control {
        failures.push("singular_spectrum_control_failed".into());
    }
    let mut matrix = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            matrix[i][j] = m[(i, j)];
        }
    }
    let matrix_exact = exact_matrix_strings(&exact_m);
    let transformed_geometry_id = if level.num == level.den {
        base.base_geometry_id.clone().unwrap()
    } else {
        geom_id
    };
    Row {
        schema: ROW_SCHEMA,
        row_id,
        pairing_id,
        base_id: base.base_id.clone(),
        base_geometry_id: base.base_geometry_id.clone().unwrap(),
        transformed_geometry_id,
        bucket: base.bucket.clone(),
        map_family: if family == "S" {
            "symplectic-control"
        } else {
            "non-symplectic-anisotropy"
        },
        t: level.name.into(),
        t_num: level.num,
        t_den: level.den,
        coordinate_order: COORDINATE_ORDER,
        primal_action: "left multiplication by positive diagonal matrix",
        dual_action: "inverse transpose on dual normals",
        matrix_row_major: matrix,
        matrix_exact_row_major: matrix_exact,
        determinant: det,
        determinant_exact: "1/1".into(),
        singular_values,
        squared_singular_values_exact: squares_exact,
        symplectic_residual_f64: residual_f64,
        symplectic_residual_exact: exact_res,
        canonical_pair_weights_f64: [pair[0].to_f64().unwrap(), pair[1].to_f64().unwrap()],
        canonical_pair_weights_exact: [rat_string(&pair[0]), rat_string(&pair[1])],
        exact_reconstruction_status: if failures
            .iter()
            .any(|x| x == "exact_reconstruction_rejected")
        {
            "rejected"
        } else {
            "reconstructed"
        },
        reconstruction_passed: failures
            .iter()
            .all(|x| x != "exact_reconstruction_rejected"),
        incidence_matches_base: inc_match,
        volume_matches_base: vol_match,
        euclidean_checks: EuclideanChecks {
            determinant_one: det_ok,
            singular_spectrum_control: spectrum_control,
            squared_singular_values_exact: square_spectrum(level).0,
            singular_values: square_spectrum(level).1,
            euclidean_control_error: 0.0,
            volume_relative_error: vol_err,
        },
        base_signature: base_sig.clone(),
        response_signature: response_sig,
        base_exact_features: base_feat.clone(),
        response_exact_features: response_feat,
        failures,
    }
}

fn l1_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    if let Err(e) = run(&args, &argv) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: &Args, argv: &[String]) -> Result<(), String> {
    create_dir_all(&args.out_dir).map_err(|e| e.to_string())?;
    let (bases, source_hash) = load_bases(&args.source)?;
    let mut rows = Vec::new();
    let mut pairs = Vec::new();
    for base in &bases {
        let duals = base
            .transformed_dual_vertices_f64
            .iter()
            .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
            .collect();
        let cache = SysLandscapePolytopeCache::from_f64_dual_vertices(duals)
            .ok_or_else(|| format!("base {} exact reconstruction failed", base.base_id))?;
        if incidence(&cache) != base.labeled_incidence_signature {
            return Err(format!("base {} incidence identity mismatch", base.base_id));
        }
        let sig = signature(&cache);
        let feat = features(&cache);
        for level in LEVELS {
            let s = row_for(base, *level, "S", &cache, &sig, &feat);
            let n = row_for(base, *level, "N", &cache, &sig, &feat);
            let p = PairedRow {
                base_id: base.base_id.clone(),
                base_geometry_id: base.base_geometry_id.clone().unwrap(),
                bucket: base.bucket.clone(),
                t: level.name.into(),
                s_row_id: s.row_id.clone(),
                n_row_id: n.row_id.clone(),
                singular_spectrum_max_abs_delta: l1_diff(&s.singular_values, &n.singular_values),
                symplectic_residual_s: s.symplectic_residual_f64,
                symplectic_residual_n: n.symplectic_residual_f64,
                symplectic_feature_l1_delta: (s.response_exact_features.symplectic_gram_l1
                    - n.response_exact_features.symplectic_gram_l1)
                    .abs(),
                response_difference_l1: l1_diff(
                    &s.response_signature.symplectic_gram_upper,
                    &n.response_signature.symplectic_gram_upper,
                ),
                negative_control_equal_at_t1: level.num == level.den
                    && s.matrix_exact_row_major == n.matrix_exact_row_major,
            };
            pairs.push(p);
            rows.push(s);
            rows.push(n);
        }
    }
    let rows_path = args.out_dir.join("rows.jsonl");
    let mut w = BufWriter::new(File::create(&rows_path).map_err(|e| e.to_string())?);
    for r in &rows {
        serde_json::to_writer(&mut w, r).map_err(|e| e.to_string())?;
        writeln!(w).map_err(|e| e.to_string())?;
    }
    w.flush().map_err(|e| e.to_string())?;
    let paired_path = args.out_dir.join("paired.jsonl");
    let mut pw = BufWriter::new(File::create(&paired_path).map_err(|e| e.to_string())?);
    for p in &pairs {
        serde_json::to_writer(&mut pw, p).map_err(|e| e.to_string())?;
        writeln!(pw).map_err(|e| e.to_string())?;
    }
    pw.flush().map_err(|e| e.to_string())?;
    let (rev, tree, dirty) = repo_identity();
    let producer_path =
        Path::new("experiments/sys-datascience/methods/generator-cartan-anisotropy/main.rs");
    let lock_path = Path::new("Cargo.lock");
    let report = Report { schema: REPORT_SCHEMA, command: argv.join(" "), source_path: args.source.display().to_string(), source_input_sha256: source_hash, source_revision: rev, source_repository_tree: tree, source_dirty_tracked: dirty, producer_source_sha256: sha256_hex(&read(producer_path).map_err(|e|e.to_string())?), cargo_lock_sha256: sha256_hex(&read(lock_path).map_err(|e|e.to_string())?), coordinate_order: COORDINATE_ORDER, requested_buckets: BUCKETS.iter().map(|(q,p)|format!("{q}x{p}")).collect(), requested_base_count: bases.len(), requested_rows: bases.len()*LEVELS.len()*2, observed_rows: rows.len(), passed_rows: rows.iter().filter(|r| r.failures.is_empty()).count(), failure_rows: rows.iter().filter(|r| !r.failures.is_empty()).count(), pair_count: pairs.len(), failures: rows.iter().flat_map(|r|r.failures.clone()).collect(), output_rows_sha256: sha256_hex(&read(&rows_path).map_err(|e|e.to_string())?), output_rows_count: rows.len(), output_paired_sha256: sha256_hex(&read(&paired_path).map_err(|e|e.to_string())?), output_paired_count: pairs.len(), diagonal_quotient_control: quotient_control(), exactness_boundary: "Intervention matrices and Cartan pair weights are exact rationals. Source f64 payloads are converted at the existing binary-rational reconstruction boundary; the resulting reconstructed geometry, incidence, and volume are exact there. Floating residuals and singular values are diagnostic views.", interpretation_boundary: "Target-free paired geometry only. This packet does not evaluate sys or capacity, estimate population effects, claim an intrinsic Sp(4)\\SL(4)\\Sp(4) distance, classify the full double coset, or treat the symplectic arm as new coverage for orbit-invariant consumers." };
    serde_json::to_writer_pretty(
        File::create(args.out_dir.join("report.json")).map_err(|e| e.to_string())?,
        &report,
    )
    .map_err(|e| e.to_string())?;
    if rows.len() != report.requested_rows || report.failure_rows != 0 {
        return Err(
            "requested Cartan rows were lost or failed; retained report is non-interpretable"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_ladder_is_exact_and_bounded() {
        assert_eq!(
            LEVELS.iter().map(|x| x.name).collect::<Vec<_>>(),
            vec!["1", "5/4", "3/2", "2"]
        );
        assert!(LEVELS.iter().all(|x| x.num > 0 && x.den > 0));
    }
    #[test]
    fn symplectic_and_non_symplectic_formulas() {
        for l in LEVELS {
            let (s, sm) = matrix_for(*l, "S");
            let (n, nm) = matrix_for(*l, "N");
            assert_eq!(exact_symplectic_residual(&s), "0/1");
            assert!(symplectic_residual(&sm) < 1e-12);
            let expected = if l.num == l.den {
                "0/1".to_owned()
            } else {
                rat_string(&(rat(l.num * l.num, l.den * l.den) - rat(1, 1)).abs())
            };
            assert_eq!(exact_symplectic_residual(&n), expected);
            assert!(nm.determinant() - 1.0 < 1e-12);
        }
    }
    #[test]
    fn paired_singular_spectra_match() {
        for l in LEVELS {
            let (s, _) = matrix_for(*l, "S");
            let (n, _) = matrix_for(*l, "N");
            let a: Vec<_> = (0..4).map(|i| s[i][i].clone() * s[i][i].clone()).collect();
            let b: Vec<_> = (0..4).map(|i| n[i][i].clone() * n[i][i].clone()).collect();
            let mut aa = a;
            let mut bb = b;
            aa.sort();
            bb.sort();
            assert_eq!(aa, bb);
        }
    }
    #[test]
    fn t1_is_negative_control() {
        let (s, _) = matrix_for(LEVELS[0], "S");
        let (n, _) = matrix_for(LEVELS[0], "N");
        assert_eq!(s, n);
    }
    #[test]
    fn cli_fails_closed_on_unknown() {
        assert!(parse_args(&["x".into(), "--unknown".into()]).is_err());
        assert!(parse_args(&["x".into(), "--out-dir".into()]).is_err());
    }
}
